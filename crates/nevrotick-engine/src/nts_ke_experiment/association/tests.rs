// SPDX-License-Identifier: GPL-2.0-only
//! Actual association-owner component controls with public fixed material.
//!
//! These controls supply no certificate/exporter/native success. The production
//! candidate caller remains the actual Windows TLS connection, never this helper.

use std::cell::Cell;
use std::time::{Duration, Instant};
use zeroize::Zeroizing;

use super::{AssociationCandidate, AssociationOwner, Config, Control, DirectionalKeys, Failure};
use crate::nts_ke_experiment::process::{ProcessAdmission, ProviderScope};
use crate::nts_ke_experiment::profile::StrictProfile;
use crate::nts_ke_experiment::{CancelOutcome, IoPhase, Phase};

/// Admit the same explicit loopback inputs through the real checked constructor.
/// # Errors
/// Propagate unexpected rejection of the valid component configuration.
fn config() -> Result<Config, Failure> {
    Config::new("127.0.0.1:4460".into(), "Time.Example".into())
}

/// Supply only public fixed material to the real private receiving boundary.
fn component<'a>(
    config: &'a Config,
    control: &'a Control,
    provider: &'a ProviderScope<'a>,
    cookies: Vec<Zeroizing<Vec<u8>>>,
) -> AssociationCandidate<'a> {
    AssociationCandidate::new(
        config,
        control,
        provider,
        std::net::SocketAddr::new(config.peer.ip(), 123),
        StrictProfile::component(),
        DirectionalKeys::new(Zeroizing::new([0x11; 32]), Zeroizing::new([0x22; 32])),
        cookies,
    )
}

/// One exact body, retaining its opaque representation rather than rewriting it.
fn cookies() -> Vec<Zeroizing<Vec<u8>>> {
    vec![Zeroizing::new(vec![0xa5; 12])]
}

/// Keep a valid first preview cookie while varying only the aggregate's tail.
/// Each public component body fits the independently bounded `u16` KE length.
fn aggregate_cookies(tail_bytes: u16) -> Vec<Zeroizing<Vec<u8>>> {
    vec![
        Zeroizing::new(vec![0xa5; 12]),
        Zeroizing::new(vec![0x33; 65_530]),
        Zeroizing::new(vec![0x44; usize::from(tail_bytes)]),
    ]
}

/// Admit the healthy neighbor with original storage and publication authority.
/// # Errors
/// Preserve actual receiving or phase-lock refusal; no TLS result is simulated.
/// # Panics
/// Fail if payloads are copied, the original cap changes, or publication repeats.
fn assert_publication<'a>(
    config: &'a Config,
    control: &'a Control,
    provider: &'a ProviderScope<'a>,
    candidate: AssociationCandidate<'a>,
    expected: (usize, usize),
    original_expiry: Instant,
) -> Result<(), Failure> {
    assert_eq!(candidate.cookies.len(), expected.0);
    assert_eq!(
        candidate
            .cookies
            .iter()
            .map(|body| body.len())
            .sum::<usize>(),
        expected.1
    );
    let vector_address = candidate.cookies.as_ptr();
    let body_addresses: Vec<_> = candidate.cookies.iter().map(|body| body.as_ptr()).collect();
    let association = AssociationOwner::receive(config, control, provider, candidate)?;
    assert_eq!(association.cookie_population()?, expected);
    assert_eq!(
        association.receiving_population(control)?.counts(),
        expected
    );
    assert_eq!(association.material.cookies.as_ptr(), vector_address);
    assert_eq!(association.material.cookies.len(), expected.0);
    for (body, address) in association.material.cookies.iter().zip(body_addresses) {
        assert_eq!(body.as_ptr(), address);
    }
    assert_eq!(
        association
            .material
            .cookies
            .iter()
            .map(|body| body.len())
            .sum::<usize>(),
        expected.1
    );
    assert_eq!(association.material.expiry, original_expiry);
    assert_eq!(control.expiry, original_expiry);
    assert!(matches!(
        *control.phase.lock().map_err(|_| Failure::Control)?,
        Phase::Published
    ));
    let repeated = component(config, control, provider, cookies());
    assert!(matches!(
        AssociationOwner::receive(config, control, provider, repeated),
        Err(Failure::Control)
    ));
    assert!(matches!(
        *control.phase.lock().map_err(|_| Failure::Control)?,
        Phase::Published
    ));
    assert_eq!(association.material.cookies.as_ptr(), vector_address);
    assert_eq!(association.material.expiry, original_expiry);
    Ok(())
}

/// Publication moves existing storage and leaves subsequent currentness live.
/// # Errors
/// Preserve checked configuration, admission and owner transition refusals.
/// # Panics
/// Fail if bytes are recopied, original expiry changes or completion repeats.
#[test]
fn genuine_owner_boundary_moves_cookies_and_consumes_completion_once() -> Result<(), Failure> {
    let config = config()?;
    let control = Control::new(Instant::now())?;
    let admission = ProcessAdmission::new();
    let claim = admission.claim()?;
    let provider = ProviderScope::component(claim);
    let population = cookies();
    let vector_address = population.as_ptr();
    let body_address = population[0].as_ptr();
    let candidate = component(&config, &control, &provider, population);
    let association = AssociationOwner::receive(&config, &control, &provider, candidate)?;
    assert_eq!(association.material.cookies.as_ptr(), vector_address);
    assert_eq!(association.material.cookies[0].as_ptr(), body_address);
    assert_eq!(association.material.expiry, control.expiry);
    assert!(matches!(
        *control.phase.lock().map_err(|_| Failure::Control)?,
        Phase::Published
    ));
    assert!(control.remaining(IoPhase::Exchange).is_ok());
    let duplicate = component(&config, &control, &provider, cookies());
    assert!(matches!(
        AssociationOwner::receive(&config, &control, &provider, duplicate),
        Err(Failure::Control)
    ));
    assert_eq!(association.material.cookies[0].as_slice(), &[0xa5; 12]);
    Ok(())
}

/// The receiving snapshot preserves the admitted zero-payload cookie domain.
/// # Errors
/// Preserve checked component admission and actual association publication.
/// # Panics
/// Fail if storing immutable counts invents a nonzero aggregate-payload requirement.
#[test]
fn receiving_population_preserves_empty_payload_without_currentness_authority()
-> Result<(), Failure> {
    let config = config()?;
    let control = Control::new(Instant::now())?;
    let admission = ProcessAdmission::new();
    let claim = admission.claim()?;
    let provider = ProviderScope::component(claim);
    let candidate = component(
        &config,
        &control,
        &provider,
        vec![Zeroizing::new(Vec::new())],
    );
    let owner = AssociationOwner::receive(&config, &control, &provider, candidate)?;
    assert_eq!(owner.receiving_population(&control)?.counts(), (1, 0));
    assert_eq!(owner.cookie_population()?, (1, 0));
    let foreign = Control::new(Instant::now())?;
    assert!(matches!(
        owner.receiving_population(&foreign),
        Err(Failure::Control)
    ));
    assert_eq!(control.cancel()?, CancelOutcome::Cancelled);
    assert_eq!(owner.receiving_population(&control)?.counts(), (1, 0));
    Ok(())
}

/// Equal labels cannot replace actual configuration/control/provider provenance.
/// Provider scopes now consume their claims, so distinct owners use independent checked
/// component admissions; reconstructing a second scope from one claim is outside the API.
/// # Errors
/// Propagate component setup refusal.
/// # Panics
/// Fail if a foreign completion changes either still-open receiving owner.
#[test]
fn foreign_completion_refuses_without_partial_publication() -> Result<(), Failure> {
    let original = config()?;
    let other = config()?;
    let control = Control::new(Instant::now())?;
    let foreign_control = Control::new(control.origin)?;
    let admission = ProcessAdmission::new();
    let claim = admission.claim()?;
    let provider = ProviderScope::component(claim);
    let foreign_admission = ProcessAdmission::new();
    let foreign_claim = foreign_admission.claim()?;
    let foreign_provider = ProviderScope::component(foreign_claim);
    for alternative in 0..3 {
        let candidate = component(&original, &control, &provider, cookies());
        let receiving_config = if alternative == 0 { &other } else { &original };
        let receiving_control = if alternative == 1 {
            &foreign_control
        } else {
            &control
        };
        let receiving_provider = if alternative == 2 {
            &foreign_provider
        } else {
            &provider
        };
        assert!(matches!(
            AssociationOwner::receive(
                receiving_config,
                receiving_control,
                receiving_provider,
                candidate
            ),
            Err(Failure::Control)
        ));
        assert!(matches!(
            *control.phase.lock().map_err(|_| Failure::Control)?,
            Phase::Open
        ));
        assert!(matches!(
            *foreign_control.phase.lock().map_err(|_| Failure::Control)?,
            Phase::Open
        ));
    }
    let accepted = component(&original, &control, &provider, cookies());
    assert!(AssociationOwner::receive(&original, &control, &provider, accepted).is_ok());
    Ok(())
}

/// KE publication does not make the association immune to later cancellation.
/// # Errors
/// Preserve setup/publication/cancellation refusal.
/// # Panics
/// Fail if retired custody acquires entropy, seals, or renews its original cap.
#[test]
fn post_ke_cancellation_refuses_before_entropy() -> Result<(), Failure> {
    let config = config()?;
    let control = Control::new(Instant::now())?;
    let admission = ProcessAdmission::new();
    let claim = admission.claim()?;
    let provider = ProviderScope::component(claim);
    let candidate = component(&config, &control, &provider, cookies());
    let mut association = AssociationOwner::receive(&config, &control, &provider, candidate)?;
    let original_expiry = association.material.expiry;
    assert_eq!(control.cancel()?, CancelOutcome::Cancelled);
    let calls = Cell::new(0);
    assert!(matches!(
        association.prepare_preview(|_| {
            calls.set(calls.get() + 1);
            Ok(())
        }),
        Err(Failure::Cancelled)
    ));
    assert_eq!(calls.get(), 0);
    assert_eq!(association.material.expiry, original_expiry);
    Ok(())
}

/// A healthy complete transfer reaches the maintained never-send consumer once.
/// # Errors
/// Propagate setup, publication, sealing or final currentness refusal.
/// # Panics
/// Fail unless actual preview completion has its bounded counts and final phase.
#[test]
fn published_association_reaches_actual_sealer_and_finishes_once() -> Result<(), Failure> {
    let config = config()?;
    let control = Control::new(Instant::now())?;
    let admission = ProcessAdmission::new();
    let claim = admission.claim()?;
    let provider = ProviderScope::component(claim);
    let candidate = component(&config, &control, &provider, cookies());
    let mut association = AssociationOwner::receive(&config, &control, &provider, candidate)?;
    let preview = association.prepare_preview(|bytes| {
        bytes.fill(0x44);
        Ok(())
    })?;
    let summary = preview.finish()?;
    assert_eq!(
        (summary.cookies, summary.cookie_bytes, summary.packet_bytes),
        (1, 12, 140)
    );
    assert_eq!(association.material.cookies[0].as_slice(), &[0xa5; 12]);
    assert_eq!(control.cancel()?, CancelOutcome::AlreadyCommitted);
    assert!(
        association
            .prepare_preview(|_| Err(Failure::Entropy))
            .is_err()
    );
    Ok(())
}

/// Invalid populations publish nothing; a useful neighbor retains the same cap.
/// # Errors
/// Propagate checked component setup or the healthy publication refusal.
/// # Panics
/// Fail if invalid material changes the owner or prevents the valid neighbor.
#[test]
fn rejected_population_preserves_atomic_publication() -> Result<(), Failure> {
    let config = config()?;
    let origin = Instant::now();
    let admission = ProcessAdmission::new();
    let claim = admission.claim()?;
    let provider = ProviderScope::component(claim);
    for (population, neighbor, expected) in [
        (Vec::new(), cookies(), (1, 12)),
        (
            (0..9).map(|_| Zeroizing::new(vec![0; 12])).collect(),
            (0..8).map(|_| Zeroizing::new(vec![0; 12])).collect(),
            (8, 96),
        ),
        (
            aggregate_cookies(65_531),
            aggregate_cookies(65_530),
            (3, 131_072),
        ),
    ] {
        let control = Control::new(origin)?;
        let original_expiry = control.expiry;
        let candidate = component(&config, &control, &provider, population);
        assert!(matches!(
            AssociationOwner::receive(&config, &control, &provider, candidate),
            Err(Failure::Cookies)
        ));
        assert!(matches!(
            *control.phase.lock().map_err(|_| Failure::Control)?,
            Phase::Open
        ));
        assert_eq!(control.expiry, original_expiry);
        let candidate = component(&config, &control, &provider, neighbor);
        assert_publication(
            &config,
            &control,
            &provider,
            candidate,
            expected,
            original_expiry,
        )?;
    }
    Ok(())
}

/// Exact aggregate and nonzero-port neighbors reach the actual receiving owner.
/// # Errors
/// Preserve checked setup and unexpected receiving-boundary refusal.
/// # Panics
/// Fail if a legitimate boundary cannot publish its original storage once.
#[test]
fn receiving_boundary_accepts_aggregate_and_endpoint_neighbors() -> Result<(), Failure> {
    let origin = Instant::now();
    let admission = ProcessAdmission::new();
    let claim = admission.claim()?;
    let provider = ProviderScope::component(claim);
    let config = config()?;
    for (tail, total) in [(65_529, 131_071), (65_530, 131_072)] {
        let control = Control::new(origin)?;
        let candidate = component(&config, &control, &provider, aggregate_cookies(tail));
        assert_publication(
            &config,
            &control,
            &provider,
            candidate,
            (3, total),
            control.expiry,
        )?;
    }
    for peer in ["127.0.0.1:4460", "[::1]:4460"] {
        let config = Config::new(peer.into(), "Time.Example".into())?;
        for port in [1, 123, 65_535] {
            let control = Control::new(origin)?;
            let mut candidate = component(&config, &control, &provider, cookies());
            candidate.udp.set_port(port);
            assert_publication(
                &config,
                &control,
                &provider,
                candidate,
                (1, 12),
                control.expiry,
            )?;
        }
    }
    Ok(())
}

/// Wrong IP, wrong family and zero port refuse before any KE publication.
/// # Errors
/// Preserve checked component setup or valid-neighbor refusal.
/// # Panics
/// Fail if endpoint refusal changes admission, its cap, or the valid neighbor.
#[test]
fn rejected_endpoint_preserves_atomic_publication() -> Result<(), Failure> {
    let origin = Instant::now();
    let admission = ProcessAdmission::new();
    let claim = admission.claim()?;
    let provider = ProviderScope::component(claim);
    for peer in ["127.0.0.1:4460", "[::1]:4460"] {
        let config = Config::new(peer.into(), "Time.Example".into())?;
        let ipv4 = std::net::SocketAddr::from(([127, 0, 0, 2], 123));
        let ipv6 = std::net::SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 2], 123));
        for udp in [ipv4, ipv6, std::net::SocketAddr::new(config.peer.ip(), 0)] {
            let control = Control::new(origin)?;
            let original_expiry = control.expiry;
            let mut candidate = component(&config, &control, &provider, cookies());
            candidate.udp = udp;
            assert!(matches!(
                AssociationOwner::receive(&config, &control, &provider, candidate),
                Err(Failure::Endpoint)
            ));
            assert!(matches!(
                *control.phase.lock().map_err(|_| Failure::Control)?,
                Phase::Open
            ));
            assert_eq!(control.expiry, original_expiry);
            let candidate = component(&config, &control, &provider, cookies());
            assert_publication(
                &config,
                &control,
                &provider,
                candidate,
                (1, 12),
                original_expiry,
            )?;
        }
    }
    Ok(())
}

/// Even a one-nanosecond candidate cap mismatch cannot replace the original cap.
/// # Errors
/// Preserve setup, checked neighbor arithmetic or valid-publication refusal.
/// # Panics
/// Fail if either mismatched cap publishes or damages the exact-cap neighbor.
#[test]
fn rejected_expiry_binding_preserves_atomic_publication() -> Result<(), Failure> {
    let origin = Instant::now();
    let config = config()?;
    let admission = ProcessAdmission::new();
    let claim = admission.claim()?;
    let provider = ProviderScope::component(claim);
    for later in [false, true] {
        let control = Control::new(origin)?;
        let original_expiry = control.expiry;
        let mut candidate = component(&config, &control, &provider, cookies());
        candidate.expiry = if later {
            original_expiry.checked_add(Duration::from_nanos(1))
        } else {
            original_expiry.checked_sub(Duration::from_nanos(1))
        }
        .ok_or(Failure::Deadline)?;
        assert!(matches!(
            AssociationOwner::receive(&config, &control, &provider, candidate),
            Err(Failure::Control)
        ));
        assert!(matches!(
            *control.phase.lock().map_err(|_| Failure::Control)?,
            Phase::Open
        ));
        assert_eq!(control.expiry, original_expiry);
        let candidate = component(&config, &control, &provider, cookies());
        assert_publication(
            &config,
            &control,
            &provider,
            candidate,
            (1, 12),
            original_expiry,
        )?;
    }
    Ok(())
}

/// Format rejection spends the actual owner's allowance without renewing it.
/// # Errors
/// Propagate setup or complete-publication refusal.
/// # Panics
/// Fail if a second attempt acquires entropy or gets a new deadline/allowance.
#[test]
fn preview_refusal_does_not_reset_work_credit() -> Result<(), Failure> {
    let config = config()?;
    let origin = Instant::now();
    let admission = ProcessAdmission::new();
    let claim = admission.claim()?;
    let provider = ProviderScope::component(claim);
    for entropy_failure in [false, true] {
        let control = Control::new(origin)?;
        let original_expiry = control.expiry;
        let population = if entropy_failure {
            cookies()
        } else {
            vec![Zeroizing::new(Vec::new())]
        };
        let candidate = component(&config, &control, &provider, population);
        let mut association = AssociationOwner::receive(&config, &control, &provider, candidate)?;
        let first_calls = Cell::new(0);
        let first = association.prepare_preview(|out| {
            first_calls.set(first_calls.get() + 1);
            if entropy_failure {
                return Err(Failure::Entropy);
            }
            out.fill(0x44);
            Ok(())
        });
        assert!(matches!(
            (entropy_failure, first),
            (true, Err(Failure::Entropy)) | (false, Err(Failure::Packet(_)))
        ));
        let second_calls = Cell::new(0);
        assert!(matches!(
            association.prepare_preview(|_| {
                second_calls.set(second_calls.get() + 1);
                Ok(())
            }),
            Err(Failure::PreviewSpent)
        ));
        assert_eq!(first_calls.get(), 1);
        assert_eq!(second_calls.get(), 0);
        assert_eq!(association.material.expiry, original_expiry);
        assert_eq!(control.expiry, original_expiry);
        assert!(matches!(
            *control.phase.lock().map_err(|_| Failure::Control)?,
            Phase::Published
        ));
    }
    Ok(())
}

/// Dropping a successfully prepared preview retains the actual owner's charge.
/// # Errors
/// Preserve setup, publication and successful first-preview preparation refusal.
/// # Panics
/// Fail if drop commits, renews the cap, or permits a second entropy acquisition.
#[test]
fn abandoned_successful_preview_does_not_reset_owner_credit() -> Result<(), Failure> {
    let config = config()?;
    let control = Control::new(Instant::now())?;
    let original_expiry = control.expiry;
    let admission = ProcessAdmission::new();
    let claim = admission.claim()?;
    let provider = ProviderScope::component(claim);
    let candidate = component(&config, &control, &provider, cookies());
    let mut association = AssociationOwner::receive(&config, &control, &provider, candidate)?;
    let first_calls = Cell::new(0);
    let preview = association.prepare_preview(|out| {
        first_calls.set(first_calls.get() + 1);
        out.fill(0x44);
        Ok(())
    })?;
    drop(preview);
    assert_eq!(first_calls.get(), 1);
    assert!(matches!(
        *control.phase.lock().map_err(|_| Failure::Control)?,
        Phase::Published
    ));
    let second_calls = Cell::new(0);
    assert!(matches!(
        association.prepare_preview(|_| {
            second_calls.set(second_calls.get() + 1);
            Ok(())
        }),
        Err(Failure::PreviewSpent)
    ));
    assert_eq!(second_calls.get(), 0);
    assert_eq!(association.material.expiry, original_expiry);
    assert_eq!(control.expiry, original_expiry);
    assert_eq!(association.material.cookies[0].as_slice(), &[0xa5; 12]);
    Ok(())
}

/// A published owner remains subject to the original strict expiry checkpoint.
/// # Errors
/// Preserve setup/publication and checked short component deadline arithmetic.
/// # Panics
/// Fail if an expired association reaches entropy or changes its deadline.
#[test]
fn association_expiry_after_ke_prevents_preview() -> Result<(), Failure> {
    let config = config()?;
    let mut control = Control::new(Instant::now())?;
    control.expiry = Instant::now()
        .checked_add(Duration::from_millis(10))
        .ok_or(Failure::Deadline)?;
    let admission = ProcessAdmission::new();
    let claim = admission.claim()?;
    let provider = ProviderScope::component(claim);
    let candidate = component(&config, &control, &provider, cookies());
    let mut association = AssociationOwner::receive(&config, &control, &provider, candidate)?;
    std::thread::sleep(
        control.expiry.saturating_duration_since(Instant::now()) + Duration::from_millis(1),
    );
    let calls = Cell::new(0);
    assert!(matches!(
        association.prepare_preview(|_| {
            calls.set(calls.get() + 1);
            Ok(())
        }),
        Err(Failure::Deadline)
    ));
    assert_eq!(calls.get(), 0);
    assert_eq!(association.material.expiry, control.expiry);
    Ok(())
}
