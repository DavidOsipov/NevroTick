// SPDX-License-Identifier: GPL-2.0-only
//! Installed numeric-source configuration through the actual managed owners.
//! S2.1/W11-W01; CHECK-6/11/21/27 and affected CHECK-7/25.
//! Source-owned KE Preview controls: S2.2; CHECK-6/9/21/25/27.

use super::*;
use crate::{PilotError, SourceAddressSet, SourceConfigurationSnapshot, SourceScale};

/// Preserve the explicit operation inputs independently of installed peer membership.
/// # Errors
/// Preserve the original mandatory checked series-construction failure.
fn series_config(peer: SocketAddr, request_ms: u64) -> Result<SeriesConfig, PilotError> {
    SeriesConfig::new(
        peer,
        Duration::from_millis(request_ms),
        Duration::from_secs(2),
    )
}

/// Observe every public restriction field without re-admitting an obsolete binding.
/// # Errors
/// Preserve actual source/endpoint snapshot failure.
fn restrictions(source: &SourceHandle, peer: SocketAddr) -> Check<[VisibleRestriction; 2]> {
    let observed = source.policy_snapshot(peer)?;
    Ok([
        visible_restriction(observed.source()),
        visible_restriction(observed.endpoint()),
    ])
}

/// Require a real current snapshot instead of substituting an unrestricted default.
/// # Errors
/// Preserve actual custody failure or an unexpectedly absent configuration.
fn installed(source: &SourceHandle) -> Check<SourceConfigurationSnapshot> {
    source
        .configuration_snapshot()?
        .ok_or_else(|| "installed source configuration disappeared".into())
}

/// Admit bounded actual loopback sets and reject malformed neighboring inputs.
/// # Errors
/// Preserve the healthy checked-set constructor failure.
/// # Panics
/// Reject wrong membership, canonical-duplicate handling or a relaxed raw endpoint rule.
#[test]
fn source_configuration_address_set_bounds_and_canonical_membership() -> Check<()> {
    let peers = [endpoint(43201), endpoint(43202), endpoint(43203)];
    let single = SourceAddressSet::new(&peers[..1])?;
    let maximum = SourceAddressSet::new(&peers)?;
    assert_eq!((single.len(), maximum.len()), (1, 3));
    assert!(!single.is_empty() && !maximum.is_empty());
    for peer in peers {
        assert!(maximum.allows(peer));
    }
    assert!(!single.allows(peers[1]));
    assert!(!maximum.allows(endpoint(43204)));
    let mapped = SocketAddr::new(Ipv4Addr::LOCALHOST.to_ipv6_mapped().into(), 43201);
    assert!(
        single.allows(mapped),
        "membership uses the canonical remote key"
    );
    for selection in [
        &[][..],
        &[peers[0], peers[1], peers[2], endpoint(43204)][..],
        &[peers[0], peers[0]][..],
    ] {
        assert!(matches!(
            SourceAddressSet::new(selection),
            Err(SourcePolicyError::InvalidSelection)
        ));
    }
    for peer in [
        endpoint(0),
        SocketAddr::from(([192, 0, 2, 1], 43201)),
        mapped,
        SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::LOCALHOST, 43201, 1, 0)),
        SocketAddr::V6(SocketAddrV6::new(Ipv6Addr::LOCALHOST, 43201, 0, 1)),
    ] {
        assert!(matches!(
            SourceAddressSet::new(&[peer]),
            Err(SourcePolicyError::InvalidEndpoint)
        ));
    }
    let ipv6 = SocketAddr::from((Ipv6Addr::LOCALHOST, 43201));
    assert!(SourceAddressSet::new(&[ipv6])?.allows(ipv6));
    Ok(())
}

/// Install only through the real authority and compare identities independently of values.
/// # Errors
/// Preserve actual checked roots, snapshots, bindings and replacement failures.
/// # Panics
/// Reject an implicit install, foreign replacement, stale CAS or changed neighbor state.
#[test]
fn source_configuration_install_and_expected_snapshot_replacement_are_atomic() -> Check<()> {
    let mut authority = isolated_authority(limits(2, 2)?)?;
    let a = authority.source(0)?;
    let b = authority.source(1)?;
    let e = endpoint(43211);
    let f = endpoint(43212);
    assert!(a.configuration_snapshot()?.is_none());
    assert!(b.configuration_snapshot()?.is_none());
    let first = authority.install_source_configuration(&a, SourceAddressSet::new(&[e])?)?;
    assert!(first.source_identity().same_source(&a.identity()));
    assert!(first.addresses().allows(e) && !first.addresses().allows(f));
    assert!(first.same_configuration(&installed(&a)?));
    assert!(b.configuration_snapshot()?.is_none());
    assert!(matches!(
        authority.endpoint_target(e),
        Err(SourcePolicyError::UnknownEndpoint)
    ));
    assert!(matches!(
        authority.install_source_configuration(&a, SourceAddressSet::new(&[f])?),
        Err(SourcePolicyError::ConfigurationChanged)
    ));
    let mut foreign = isolated_authority(limits(1, 1)?)?;
    let foreign_source = foreign.source(0)?;
    assert!(matches!(
        authority.install_source_configuration(&foreign_source, SourceAddressSet::new(&[e])?),
        Err(SourcePolicyError::Foreign)
    ));
    let foreign_snapshot =
        foreign.install_source_configuration(&foreign_source, SourceAddressSet::new(&[e])?)?;
    assert!(!first.same_configuration(&foreign_snapshot));
    assert!(matches!(
        authority.replace_source_configuration(&foreign_snapshot, SourceAddressSet::new(&[f])?),
        Err(SourcePolicyError::Foreign)
    ));
    assert!(first.same_configuration(&installed(&a)?));
    let equal_values =
        authority.replace_source_configuration(&first, SourceAddressSet::new(&[e])?)?;
    assert!(!first.same_configuration(&equal_values));
    assert!(first.addresses().allows(e) && equal_values.addresses().allows(e));
    assert!(matches!(
        authority.replace_source_configuration(&first, SourceAddressSet::new(&[f])?),
        Err(SourcePolicyError::ConfigurationChanged)
    ));
    assert!(equal_values.same_configuration(&installed(&a)?));
    let b_snapshot = authority.install_source_configuration(&b, SourceAddressSet::new(&[f])?)?;
    let replacement =
        authority.replace_source_configuration(&equal_values, SourceAddressSet::new(&[f])?)?;
    assert!(replacement.addresses().allows(f) && !replacement.addresses().allows(e));
    assert!(b_snapshot.same_configuration(&installed(&b)?));
    assert!(foreign_snapshot.same_configuration(&installed(&foreign_source)?));
    assert_eq!(format!("{first:?}"), "SourceConfigurationSnapshot { .. }");
    assert_eq!(
        format!("{:?}", first.addresses()),
        "SourceAddressSet { .. }"
    );
    Ok(())
}

/// Check original source/configuration identity in every actual committed result.
/// # Errors
/// Preserve missing real reports/facts or a missing installed configuration token.
/// # Panics
/// Reject invented completion, mismatched original configuration or bad receive accounting.
fn committed_configuration(
    report: &AcquisitionReport,
    expected: &SourceConfigurationSnapshot,
) -> Check<()> {
    committed_source_provenance(report, Some(expected.source_identity()))?;
    for attempt in report.series().attempts() {
        let result = attempt
            .report()
            .ok_or("actual configuration result absent")?;
        let facts = result.facts().ok_or("actual configuration facts absent")?;
        assert!(
            facts
                .source_configuration()
                .is_some_and(|actual| actual.same_configuration(expected)
                    && actual.declared_scale() == expected.declared_scale())
        );
    }
    Ok(())
}

/// Preserve legitimate concurrent endpoints and per-run inputs under one installed set.
/// # Errors
/// Preserve checked public setup, genuine loopback exchange and every scoped peer join.
/// # Panics
/// Reject accidental replacement, lost history or a wrong actual round candidate.
#[test]
fn source_configuration_parallel_public_sessions_and_reordered_round_share_revision() -> Check<()> {
    let mut authority = isolated_authority(SourcePolicyLimits::new(
        2,
        2,
        Duration::from_millis(1),
        Duration::from_millis(4),
    )?)?;
    let a = authority.source(0)?;
    let b = authority.source(1)?;
    let e_peer = std::net::UdpSocket::bind(endpoint(0))?;
    let f_peer = std::net::UdpSocket::bind(endpoint(0))?;
    let e = e_peer.local_addr()?;
    let f = f_peer.local_addr()?;
    let original_a: SourceConfigurationSnapshot =
        authority.install_source_configuration(&a, SourceAddressSet::new(&[e, f])?)?;
    assert_eq!(original_a.declared_scale(), SourceScale::Unknown);
    let a_configuration: SourceConfigurationSnapshot =
        authority.replace_source_scale(&original_a, SourceScale::UnsmearedUtc)?;
    assert_eq!(a_configuration.declared_scale(), SourceScale::UnsmearedUtc);
    let b_configuration: SourceConfigurationSnapshot =
        authority.install_source_configuration(&b, SourceAddressSet::new(&[e])?)?;
    assert_eq!(b_configuration.declared_scale(), SourceScale::Unknown);
    let mut ae = a.session(series_config(e, 400)?)?;
    let profile = crate::ExperimentalProfile::new(3_000_000_000, 4_500_000_000, 30_000_000_000)?;
    let mut af = a.session(series_config(f, 500)?.with_experimental_profile(profile))?;
    let mut be = b.session(series_config(e, 600)?)?;
    let initial_history = snapshot_public_session_exchange(&mut ae, &e_peer, None, 3)?;
    let alternate_result = snapshot_public_session_exchange(&mut af, &f_peer, None, 3)?;
    let neighbor_report = snapshot_public_session_exchange(&mut be, &e_peer, None, 3)?;
    committed_configuration(&initial_history, &a_configuration)?;
    committed_configuration(&alternate_result, &a_configuration)?;
    committed_configuration(&neighbor_report, &b_configuration)?;
    let (round, returned_peer) = managed_round_once(&a, &[f, e], f_peer, None)?;
    assert_eq!(returned_peer.local_addr()?, f);
    assert_eq!(round.attempts().len(), 1);
    let first = round.attempts()[0].report();
    assert_eq!(first.status(), Status::Header);
    assert_eq!((first.received(), first.rejected()), (1, 0));
    let facts = first
        .facts()
        .ok_or("healthy round configuration facts absent")?;
    assert_eq!(facts.peer_endpoint(), f);
    assert!(
        facts
            .source_configuration()
            .is_some_and(|actual| actual.same_configuration(&a_configuration)
                && actual.declared_scale() == SourceScale::UnsmearedUtc)
    );
    assert!(a_configuration.same_configuration(&installed(&a)?));
    assert!(b_configuration.same_configuration(&installed(&b)?));
    let replacement = authority
        .replace_source_configuration(&a_configuration, SourceAddressSet::new(&[f, e])?)?;
    assert!(!replacement.same_configuration(&a_configuration));
    assert_eq!(replacement.declared_scale(), SourceScale::Unknown);
    // Later replacement cannot rewrite an already returned successful acquisition.
    committed_configuration(&initial_history, &a_configuration)?;
    committed_configuration(&alternate_result, &a_configuration)?;
    committed_configuration(&neighbor_report, &b_configuration)?;
    let old_cell = Arc::downgrade(&a.cell);
    drop((ae, af, be, a, b, authority));
    assert!(
        old_cell.upgrade().is_none(),
        "historical configuration retains no mutable policy owner"
    );
    committed_configuration(&initial_history, &a_configuration)?;
    committed_configuration(&alternate_result, &a_configuration)?;
    Ok(())
}

/// Reject an outside peer through both actual public producers before slot publication.
/// # Errors
/// Preserve checked roots, operation plans, retained facts and healthy preparation.
/// # Panics
/// Reject a hidden endpoint allocation, altered configuration or widened allowed set.
#[test]
fn source_configuration_outside_peers_cannot_publish_endpoint_capacity() -> Check<()> {
    let mut authority = isolated_authority(limits(1, 3)?)?;
    let source = authority.source(0)?;
    let e = endpoint(43221);
    let f = endpoint(43222);
    let g = endpoint(43223);
    let configuration =
        authority.install_source_configuration(&source, SourceAddressSet::new(&[e, f])?)?;
    let existing = source.bind(e)?;
    let before = facts(&existing)?;
    assert!(matches!(
        source.session(series_config(g, 100)?),
        Err(SourcePolicyError::Foreign)
    ));
    assert!(matches!(
        source.prepare_address_round(managed_round_config(&[f, g])?),
        Err(PilotError::SourcePolicy(SourcePolicyError::Foreign))
    ));
    for unpublished in [f, g] {
        assert!(matches!(
            authority.endpoint_target(unpublished),
            Err(SourcePolicyError::UnknownEndpoint)
        ));
    }
    assert_eq!(facts(&existing)?, before);
    assert!(configuration.same_configuration(&installed(&source)?));
    let _session = source.session(series_config(f, 100)?)?;
    let (prepared, control) = source.prepare_address_round(managed_round_config(&[f, e])?)?;
    drop((prepared, control));
    assert!(authority.endpoint_target(f).is_ok());
    assert_eq!(facts(&existing)?, before);
    assert!(configuration.same_configuration(&installed(&source)?));
    Ok(())
}

/// Existing prepared plans must challenge the genuine configuration again at setup.
/// # Errors
/// Preserve checked setup, public execution and actual no-packet observation failure.
/// # Panics
/// Reject a false invocation-Stale/internal failure, native send or configuration bypass.
#[test]
fn source_configuration_first_install_and_equal_replace_retire_old_prepared_plans() -> Check<()> {
    for initially_installed in [false, true] {
        let mut authority = isolated_authority(limits(1, 1)?)?;
        let source = authority.source(0)?;
        let peer = std::net::UdpSocket::bind(endpoint(0))?;
        peer.set_nonblocking(true)?;
        let endpoint = peer.local_addr()?;
        if initially_installed {
            let _configuration = authority
                .install_source_configuration(&source, SourceAddressSet::new(&[endpoint])?)?;
        }
        let mut session = source.session(series_config(endpoint, 100)?)?;
        let (prepared, control) = session.prepare()?;
        let before = restrictions(&source, endpoint)?;
        let current = match source.configuration_snapshot()? {
            Some(previous) => authority.replace_source_scale(&previous, SourceScale::Smeared)?,
            None => authority
                .install_source_configuration(&source, SourceAddressSet::new(&[endpoint])?)?,
        };
        assert_eq!(
            current.declared_scale(),
            if initially_installed {
                SourceScale::Smeared
            } else {
                SourceScale::Unknown
            }
        );
        assert!(current.addresses().allows(endpoint));
        let report = prepared.execute(&crate::ControlSchedule::new(None, None)?)?;
        assert_eq!(report.series().end(), crate::SeriesEnd::Failure);
        assert_eq!(
            report
                .series()
                .primary_failure()
                .map(|failure| failure.code),
            Some("source-policy")
        );
        assert!(
            report
                .series()
                .attempts()
                .iter()
                .all(|attempt| attempt.report().is_none())
        );
        assert_eq!(restrictions(&source, endpoint)?, before);
        assert!(current.same_configuration(&installed(&source)?));
        assert!(!control.sender.borrow().stop && control.sender.borrow().epoch == 0);
        let mut bytes = [0; RECEIVE_BYTES];
        assert!(
            matches!(peer.recv_from(&mut bytes), Err(error) if error.kind() == io::ErrorKind::WouldBlock)
        );
        assert!(matches!(
            session.prepare(),
            Err(crate::AcquisitionError::Retired)
        ));
        let mut obsolete = source.session(series_config(endpoint, 100)?)?;
        let _replacement = authority
            .replace_source_configuration(&current, SourceAddressSet::new(&[endpoint])?)?;
        assert!(matches!(
            obsolete.prepare(),
            Err(crate::AcquisitionError::SourcePolicy(
                SourcePolicyError::ConfigurationChanged
            ))
        ));
    }
    Ok(())
}

/// Configuration replacement preserves every restriction field and existing target meaning.
/// # Errors
/// Preserve actual transition, complete observation, replacement and targeted re-enable errors.
/// # Panics
/// Reject a reset floor/rate/refusal, unrelated record change or stale-target invention.
#[test]
fn source_configuration_replacement_preserves_kod_records_floors_and_targets() -> Check<()> {
    let mut authority = isolated_authority(limits(2, 2)?)?;
    let a = authority.source(0)?;
    let b = authority.source(1)?;
    let e = endpoint(43231);
    let f = endpoint(43232);
    let configuration =
        authority.install_source_configuration(&a, SourceAddressSet::new(&[e, f])?)?;
    let ae = a.bind(e)?;
    let bf = b.bind(f)?;
    let start = Instant::now();
    let mut rate = fixture_send(&ae, &Arc::new(()), start)?;
    fixture_control(&ae, &mut rate, RATE, start)?;
    let due = facts(&ae)?[0].1.ok_or("charged RATE floor absent")?;
    let mut denied = fixture_send(&ae, &Arc::new(()), due)?;
    fixture_control(&ae, &mut denied, RSTR, due)?;
    let retained = restrictions(&a, e)?;
    let independent = facts(&bf)?;
    let source_target = authority.source_target(&a)?;
    let endpoint_target = authority.endpoint_target(e)?;
    let mut source_changes = ae.source_changes.subscribe();
    let endpoint_changes = ae.endpoint_changes.subscribe();
    let neighbor_changes = bf.source_changes.subscribe();
    let replacement =
        authority.replace_source_configuration(&configuration, SourceAddressSet::new(&[f, e])?)?;
    assert!(source_changes.has_changed()?);
    drop(source_changes.borrow_and_update());
    assert!(!endpoint_changes.has_changed()? && !neighbor_changes.has_changed()?);
    assert_eq!(restrictions(&a, e)?, retained);
    assert_eq!(facts(&bf)?, independent);
    assert!(replacement.same_configuration(&installed(&a)?));
    let current = a.bind(e)?;
    assert!(matches!(
        fixture_send(&current, &Arc::new(()), due),
        Err(SourcePolicyError::Disabled)
    ));
    authority.reenable(&[source_target, endpoint_target])?;
    let cleared = restrictions(&a, e)?;
    for (before, after) in retained.into_iter().zip(cleared) {
        assert!(before.3 && !after.3);
        assert_eq!(
            (after.0, after.1, after.2, after.4, after.6),
            (before.0, before.1, before.2, before.4, before.6)
        );
    }
    let before_probe = facts(&current)?;
    assert!(matches!(
        fixture_send(&current, &Arc::new(()), due),
        Err(SourcePolicyError::NotDue)
    ));
    assert_eq!(facts(&current)?, before_probe);
    assert_eq!(facts(&bf)?, independent);
    Ok(())
}

/// Exhaust the genuine checked counter without touching restriction counter domains.
/// # Errors
/// Preserve actual configuration, bound records, forcing seam and public authority errors.
/// # Panics
/// Reject wrapping/reused identity, reopened issuance or independent-source damage.
#[test]
fn source_configuration_revision_exhaustion_is_sticky_and_source_local() -> Check<()> {
    let mut authority = isolated_authority(limits(2, 2)?)?;
    let a = authority.source(0)?;
    let b = authority.source(1)?;
    let e = endpoint(43241);
    let f = endpoint(43242);
    let original = authority.install_source_configuration(&a, SourceAddressSet::new(&[e])?)?;
    let neighbor = authority.install_source_configuration(&b, SourceAddressSet::new(&[f])?)?;
    let ae = a.bind(e)?;
    let bf = b.bind(f)?;
    let original_facts = restrictions(&a, e)?;
    let neighbor_facts = facts(&bf)?;
    let maximum = force_configuration_revision_max_for_test(&a)?;
    assert!(!maximum.same_configuration(&original));
    assert!(maximum.same_configuration(&installed(&a)?));
    assert!(matches!(
        authority.replace_source_configuration(&original, SourceAddressSet::new(&[e])?),
        Err(SourcePolicyError::ConfigurationChanged)
    ));
    assert!(maximum.same_configuration(&installed(&a)?));
    assert!(matches!(
        authority.replace_source_configuration(&maximum, SourceAddressSet::new(&[e])?),
        Err(SourcePolicyError::ConfigurationUnavailable)
    ));
    for _ in 0..2 {
        assert!(matches!(
            a.configuration_snapshot(),
            Err(SourcePolicyError::ConfigurationUnavailable)
        ));
        assert!(matches!(
            a.session(series_config(e, 100)?),
            Err(SourcePolicyError::ConfigurationUnavailable)
        ));
        assert!(matches!(
            authority.replace_source_configuration(&maximum, SourceAddressSet::new(&[e])?),
            Err(SourcePolicyError::ConfigurationUnavailable)
        ));
    }
    assert_eq!(restrictions(&a, e)?, original_facts);
    assert_eq!(facts(&bf)?, neighbor_facts);
    assert!(neighbor.same_configuration(&installed(&b)?));
    assert!(maximum.addresses().allows(e) && original.addresses().allows(e));
    authority.reenable(&[authority.source_target(&a)?])?;
    assert!(matches!(
        a.configuration_snapshot(),
        Err(SourcePolicyError::ConfigurationUnavailable)
    ));
    assert!(SourceMode::Managed(bf).admit().is_ok());
    drop(ae);
    Ok(())
}

/// Inspect the real captured definition while retaining its actual policy lock.
/// # Errors
/// Preserve current source/configuration, endpoint and custody refusal exactly.
#[cfg(feature = "ke-association-prototype")]
fn ke_preview_check(
    preview: &ConfiguredKePreview,
    peer: Option<SocketAddr>,
) -> Result<(), SourcePolicyError> {
    let guard: KePreviewPolicyGuard<'_> = preview.lock()?;
    guard.check(peer)
}

/// Replace just the declaration and verify the actual KE admission on both revisions.
/// # Errors
/// Preserve any unexpected issuer, capture or refreshed guard failure.
/// # Panics
/// Reject a changed KE target, lost original declaration or stale guard bypass.
#[cfg(feature = "ke-association-prototype")]
fn replace_ke_scale(
    authority: &mut SourcePolicyAuthority,
    source: &SourceHandle,
    original: &ConfiguredKePreview,
    peer: SocketAddr,
) -> Check<SourceConfigurationSnapshot> {
    let first: &SourceConfigurationSnapshot = original.snapshot();
    let declared: SourceConfigurationSnapshot =
        authority.replace_source_scale(first, SourceScale::UnsmearedUtc)?;
    assert_eq!(first.declared_scale(), SourceScale::Unknown);
    assert_eq!(declared.declared_scale(), SourceScale::UnsmearedUtc);
    let refreshed: ConfiguredKePreview = source.capture_ke_preview()?;
    assert!(refreshed.snapshot().same_configuration(&declared));
    assert_eq!(
        (refreshed.target().peer(), refreshed.target().identity()),
        (endpoint(4460), "localhost")
    );
    assert_eq!(
        ke_preview_check(original, Some(peer)),
        Err(SourcePolicyError::ConfigurationChanged)
    );
    assert!(matches!(
        authority.replace_source_scale(first, SourceScale::Smeared),
        Err(SourcePolicyError::ConfigurationChanged)
    ));
    ke_preview_check(&refreshed, Some(peer))?;
    Ok(declared)
}

/// Issue KE only from the actual catalogue, then withdraw or exhaust its revision.
/// # Errors
/// Preserve real target/set construction, authority operations and healthy custody.
/// # Panics
/// Reject missing/foreign issuance, surviving numeric withdrawal or recycled revisions.
#[cfg(feature = "ke-association-prototype")]
#[test]
fn source_configuration_ke_preview_issuance_withdrawal_and_exhaustion() -> Check<()> {
    let mut authority: SourcePolicyAuthority = isolated_authority(limits(1, 1)?)?;
    let source: SourceHandle = authority.source(0)?;
    let peer: SocketAddr = endpoint(43251);
    let target: crate::KePreviewTarget = crate::KePreviewTarget::new(endpoint(4460), "localhost")?;
    assert!(matches!(
        source.capture_ke_preview(),
        Err(SourcePolicyError::ConfigurationChanged)
    ));
    let mut foreign: SourcePolicyAuthority = isolated_authority(limits(1, 1)?)?;
    let foreign_source: SourceHandle = foreign.source(0)?;
    let foreign_snapshot: SourceConfigurationSnapshot =
        foreign.install_source_configuration(&foreign_source, SourceAddressSet::new(&[peer])?)?;
    assert!(matches!(
        authority.replace_source_scale(&foreign_snapshot, SourceScale::Smeared),
        Err(SourcePolicyError::Foreign)
    ));
    assert!(matches!(
        authority.install_ke_preview_configuration(
            &foreign_source,
            SourceAddressSet::new(&[peer])?,
            target.clone(),
        ),
        Err(SourcePolicyError::Foreign)
    ));
    let first: SourceConfigurationSnapshot = authority.install_ke_preview_configuration(
        &source,
        SourceAddressSet::new(&[peer])?,
        target.clone(),
    )?;
    assert!(matches!(
        authority.replace_ke_preview_configuration(
            &foreign_snapshot,
            SourceAddressSet::new(&[peer])?,
            target.clone(),
        ),
        Err(SourcePolicyError::Foreign)
    ));
    let original: ConfiguredKePreview = source.capture_ke_preview()?;
    assert!(original.snapshot().same_configuration(&first));
    assert_eq!(
        (original.target().peer(), original.target().identity()),
        (endpoint(4460), "localhost")
    );
    ke_preview_check(&original, Some(peer))?;
    let declared: SourceConfigurationSnapshot =
        replace_ke_scale(&mut authority, &source, &original, peer)?;
    let numeric: SourceConfigurationSnapshot =
        authority.replace_source_configuration(&declared, SourceAddressSet::new(&[peer])?)?;
    assert!(matches!(
        source.capture_ke_preview(),
        Err(SourcePolicyError::ConfigurationChanged)
    ));
    assert_eq!(
        ke_preview_check(&original, None),
        Err(SourcePolicyError::ConfigurationChanged)
    );
    let restored: SourceConfigurationSnapshot = authority.replace_ke_preview_configuration(
        &numeric,
        SourceAddressSet::new(&[peer])?,
        target.clone(),
    )?;
    assert!(!restored.same_configuration(&first));
    assert_eq!(restored.declared_scale(), SourceScale::Unknown);
    let maximum: SourceConfigurationSnapshot = force_configuration_revision_max_for_test(&source)?;
    let at_maximum: ConfiguredKePreview = source.capture_ke_preview()?;
    assert!(at_maximum.snapshot().same_configuration(&maximum));
    ke_preview_check(&at_maximum, Some(peer))?;
    assert!(matches!(
        authority.replace_ke_preview_configuration(
            &maximum,
            SourceAddressSet::new(&[peer])?,
            target,
        ),
        Err(SourcePolicyError::ConfigurationUnavailable)
    ));
    assert_eq!(
        ke_preview_check(&at_maximum, None),
        Err(SourcePolicyError::ConfigurationUnavailable)
    );
    assert!(matches!(
        source.capture_ke_preview(),
        Err(SourcePolicyError::ConfigurationUnavailable)
    ));
    assert!(foreign_snapshot.same_configuration(&installed(&foreign_source)?));
    assert!(matches!(
        authority.endpoint_target(peer),
        Err(SourcePolicyError::UnknownEndpoint)
    ));
    Ok(())
}

/// Existing actual DENY/RSTR records constrain Preview without propagation or debit.
/// # Errors
/// Preserve genuine catalogue, checked inputs, production control-kernel and guard errors.
/// # Panics
/// Reject source/endpoint bypass, transitive refusal, floor changes or endpoint creation.
#[cfg(feature = "ke-association-prototype")]
#[test]
fn source_configuration_ke_preview_preserves_source_and_endpoint_restrictions() -> Check<()> {
    let codes: [u32; 2] = [DENY, RSTR];
    for code in codes {
        let mut authority: SourcePolicyAuthority = isolated_authority(limits(2, 1)?)?;
        let source: SourceHandle = authority.source(0)?;
        let alias: SourceHandle = authority.source(1)?;
        let peers: [SocketAddr; 2] = [endpoint(43261), endpoint(43262)];
        let target: crate::KePreviewTarget =
            crate::KePreviewTarget::new(endpoint(4460), "localhost")?;
        let source_configuration: SourceConfigurationSnapshot = authority
            .install_ke_preview_configuration(
                &source,
                SourceAddressSet::new(&peers)?,
                target.clone(),
            )?;
        let alias_configuration: SourceConfigurationSnapshot = authority
            .install_ke_preview_configuration(&alias, SourceAddressSet::new(&peers)?, target)?;
        let original: ConfiguredKePreview = source.capture_ke_preview()?;
        let neighbor: ConfiguredKePreview = alias.capture_ke_preview()?;
        let binding: SourceBinding = source.bind(peers[0])?;
        let now: Instant = Instant::now();
        let request: Arc<()> = Arc::new(());
        let mut pending: FixtureRequest = fixture_send(&binding, &request, now)?;
        fixture_control(&binding, &mut pending, code, now)?;
        let retained: [RecordFacts; 2] = facts(&binding)?;
        let independent: [VisibleRestriction; 2] = restrictions(&alias, peers[0])?;
        assert!(retained[0].2 && retained[1].2);
        assert!(!independent[0].3 && independent[1].3);
        assert!(matches!(
            source.capture_ke_preview(),
            Err(SourcePolicyError::Disabled)
        ));
        assert_eq!(
            ke_preview_check(&original, None),
            Err(SourcePolicyError::Disabled)
        );
        ke_preview_check(&neighbor, None)?;
        assert_eq!(
            ke_preview_check(&neighbor, Some(peers[0])),
            Err(SourcePolicyError::Disabled)
        );
        ke_preview_check(&neighbor, Some(peers[1]))?;
        assert_eq!(facts(&binding)?, retained);
        assert_eq!(restrictions(&alias, peers[0])?, independent);
        assert!(matches!(
            authority.endpoint_target(peers[1]),
            Err(SourcePolicyError::UnknownEndpoint)
        ));
        let source_target: SourcePolicyTarget = authority.source_target(&source)?;
        authority.reenable(std::slice::from_ref(&source_target))?;
        let current: ConfiguredKePreview = source.capture_ke_preview()?;
        ke_preview_check(&current, Some(peers[1]))?;
        assert_eq!(
            ke_preview_check(&current, Some(peers[0])),
            Err(SourcePolicyError::Disabled)
        );
        let cleared: [RecordFacts; 2] = facts(&binding)?;
        assert!(!cleared[0].2);
        assert_eq!(cleared[1], retained[1]);
        assert_eq!(
            (cleared[0].0, cleared[0].1, cleared[0].5),
            (retained[0].0, retained[0].1, retained[0].5)
        );
        assert_eq!(restrictions(&alias, peers[0])?, independent);
        assert!(source_configuration.same_configuration(&installed(&source)?));
        assert!(alias_configuration.same_configuration(&installed(&alias)?));
        assert!(matches!(
            authority.endpoint_target(peers[1]),
            Err(SourcePolicyError::UnknownEndpoint)
        ));
    }
    Ok(())
}

/// Closed commands for a length-three, sequential issuer history.
#[cfg(feature = "ke-association-prototype")]
#[derive(Clone, Copy, Debug)]
enum PreviewHistoryAction {
    /// Read the actual installed snapshot without replacing anything.
    Observe,
    /// Replace the current definition with equal KE and UDP values.
    ReplaceCurrent,
    /// Replace the current definition with numeric UDP inputs only.
    Withdraw,
    /// Attempt KE replacement using the genuine initial expected snapshot.
    ReplaceOriginal,
}

/// Requirements-only state: an independent replacement ordinal and KE presence.
/// No production revision value, transition helper or currentness predicate is used.
#[cfg(feature = "ke-association-prototype")]
#[derive(Clone, Copy)]
struct PreviewHistoryModel {
    /// Zero denotes the genuine installation; every accepted replacement advances.
    revision: usize,
    /// Numeric replacement withdraws KE; a successful KE replacement restores it.
    preview: bool,
}

#[cfg(feature = "ke-association-prototype")]
impl PreviewHistoryModel {
    /// Specify CAS and withdrawal semantics independently of returned source state.
    /// # Errors
    /// Reject the stale initial snapshot or an unrepresentable model revision.
    fn after(self, action: PreviewHistoryAction) -> Result<Self, SourcePolicyError> {
        match action {
            PreviewHistoryAction::Observe => Ok(self),
            PreviewHistoryAction::ReplaceOriginal if self.revision != 0 => {
                Err(SourcePolicyError::ConfigurationChanged)
            }
            PreviewHistoryAction::ReplaceCurrent
            | PreviewHistoryAction::ReplaceOriginal
            | PreviewHistoryAction::Withdraw => Ok(Self {
                revision: self
                    .revision
                    .checked_add(1)
                    .ok_or(SourcePolicyError::ConfigurationUnavailable)?,
                preview: !matches!(action, PreviewHistoryAction::Withdraw),
            }),
        }
    }
}

/// Compare actual live custody and historical grants after each delivered action.
/// # Errors
/// Preserve real snapshot, captured-context and private policy-guard refusals.
/// # Panics
/// Reject stale-grant revival, equal-value identity reuse, KE withdrawal bypass,
/// target substitution, outside membership or an endpoint slot created by observation.
#[cfg(feature = "ke-association-prototype")]
fn assert_preview_history(
    source: &SourceHandle,
    authority: &SourcePolicyAuthority,
    current: &SourceConfigurationSnapshot,
    grants: &mut Vec<(usize, ConfiguredKePreview)>,
    model: PreviewHistoryModel,
    peer: SocketAddr,
    trace: ([PreviewHistoryAction; 3], usize),
) -> Check<()> {
    assert!(installed(source)?.same_configuration(current), "{trace:?}");
    assert!(
        current.source_identity().same_source(&source.identity()),
        "{trace:?}"
    );
    assert_eq!(current.addresses().len(), 1, "{trace:?}");
    assert!(current.addresses().allows(peer), "{trace:?}");
    for (revision, grant) in grants.iter() {
        let expected: Result<(), SourcePolicyError> =
            if *revision == model.revision && model.preview {
                Ok(())
            } else {
                Err(SourcePolicyError::ConfigurationChanged)
            };
        assert_eq!(
            ke_preview_check(grant, None),
            expected,
            "{trace:?} grant={revision}"
        );
        assert_eq!(
            ke_preview_check(grant, Some(peer)),
            expected,
            "{trace:?} grant={revision}"
        );
        assert_eq!(
            grant.snapshot().same_configuration(current),
            *revision == model.revision,
            "{trace:?}"
        );
    }
    let captured: Result<ConfiguredKePreview, SourcePolicyError> = source.capture_ke_preview();
    match captured {
        Ok(preview) => {
            assert!(
                model.preview,
                "numeric-only definition captured KE: {trace:?}"
            );
            assert!(preview.snapshot().same_configuration(current), "{trace:?}");
            assert_eq!(
                (preview.target().peer(), preview.target().identity()),
                (endpoint(4460), "localhost"),
                "{trace:?}"
            );
            assert_eq!(ke_preview_check(&preview, None), Ok(()), "{trace:?}");
            assert_eq!(ke_preview_check(&preview, Some(peer)), Ok(()), "{trace:?}");
            assert_eq!(
                ke_preview_check(&preview, Some(endpoint(43272))),
                Err(SourcePolicyError::Foreign),
                "{trace:?}"
            );
            grants.push((model.revision, preview));
        }
        Err(error) => {
            assert!(!model.preview, "legal KE capture refused: {trace:?}");
            assert_eq!(error, SourcePolicyError::ConfigurationChanged, "{trace:?}");
        }
    }
    for endpoint in [peer, endpoint(43272)] {
        assert!(
            matches!(
                authority.endpoint_target(endpoint),
                Err(SourcePolicyError::UnknownEndpoint)
            ),
            "{trace:?}"
        );
        assert!(
            matches!(
                source.policy_snapshot(endpoint),
                Err(SourcePolicyError::UnknownEndpoint)
            ),
            "{trace:?}"
        );
    }
    Ok(())
}

/// Deliver one concrete history to genuine issuers; never skip a stale command.
/// # Errors
/// Preserve actual checked setup and observation failures.
/// # Panics
/// Reject an outcome or identity effect that differs from the independent ordinal model.
#[cfg(feature = "ke-association-prototype")]
fn run_preview_history(
    actions: [PreviewHistoryAction; 3],
    partitions: &mut [usize; 3],
) -> Check<()> {
    let mut authority: SourcePolicyAuthority = isolated_authority(limits(1, 1)?)?;
    let source: SourceHandle = authority.source(0)?;
    let peer: SocketAddr = endpoint(43271);
    let target: crate::KePreviewTarget = crate::KePreviewTarget::new(endpoint(4460), "localhost")?;
    let addresses: SourceAddressSet = SourceAddressSet::new(&[peer])?;
    let original: SourceConfigurationSnapshot =
        authority.install_ke_preview_configuration(&source, addresses.clone(), target.clone())?;
    let mut current: SourceConfigurationSnapshot = original.clone();
    let mut model: PreviewHistoryModel = PreviewHistoryModel {
        revision: 0,
        preview: true,
    };
    let initial: ConfiguredKePreview = source.capture_ke_preview()?;
    assert!(initial.snapshot().same_configuration(&original));
    let mut grants: Vec<(usize, ConfiguredKePreview)> = Vec::with_capacity(4);
    grants.push((0, initial));
    for (step, action) in actions.into_iter().enumerate() {
        let expected: Result<PreviewHistoryModel, SourcePolicyError> = model.after(action);
        let actual: Result<SourceConfigurationSnapshot, SourcePolicyError> =
            match action {
                PreviewHistoryAction::Observe => source
                    .configuration_snapshot()?
                    .ok_or(SourcePolicyError::ConfigurationChanged),
                PreviewHistoryAction::ReplaceCurrent => authority.replace_ke_preview_configuration(
                    &current,
                    addresses.clone(),
                    target.clone(),
                ),
                PreviewHistoryAction::Withdraw => {
                    authority.replace_source_configuration(&current, addresses.clone())
                }
                PreviewHistoryAction::ReplaceOriginal => authority
                    .replace_ke_preview_configuration(&original, addresses.clone(), target.clone()),
            };
        let actual_outcome: Result<(), SourcePolicyError> = actual
            .as_ref()
            .map(|_: &SourceConfigurationSnapshot| ())
            .map_err(|error: &SourcePolicyError| *error);
        assert_eq!(
            actual_outcome,
            expected.map(|_: PreviewHistoryModel| ()),
            "{actions:?} step={step}"
        );
        if let (Ok(snapshot), Ok(next)) = (actual, expected) {
            let observation: bool = matches!(action, PreviewHistoryAction::Observe);
            assert_eq!(
                snapshot.same_configuration(&current),
                observation,
                "{actions:?} step={step}"
            );
            partitions[usize::from(!observation)] += 1;
            current = snapshot;
            model = next;
        } else {
            partitions[2] += 1;
        }
        assert_eq!(
            current.same_configuration(&original),
            model.revision == 0,
            "{actions:?} step={step}"
        );
        assert_preview_history(
            &source,
            &authority,
            &current,
            &mut grants,
            model,
            peer,
            (actions, step),
        )?;
        assert!(grants.len() <= 4);
    }
    Ok(())
}

/// S2.2 CHECK-6/9/21/25/27: exhaustive finite issuer histories, not schedule exploration.
/// Exactly 64 fresh histories contain all 4^3 commands (192 delivered actions).
/// The external oracle is the task's CAS contract: observation preserves revision;
/// every accepted equal-value replacement advances it; numeric input withdraws KE;
/// only the actual still-current expected snapshot may replace a definition.
/// Thus there are 48 observations, 117 accepted replacements and 27 stale refusals.
/// Each prefix checks all retained actual grants (at most four), current capture,
/// selected/outside endpoints and no endpoint allocation. No network, clock, shrink,
/// random seed, new dependency, forged context or production reducer supplies truth.
/// # Errors
/// Preserve genuine setup and observer failures for the concrete reported history.
/// # Panics
/// Reject wrong transitions, incomplete enumeration or changed partition cardinality.
#[cfg(feature = "ke-association-prototype")]
#[test]
fn source_configuration_ke_preview_generated_histories_match_independent_model() -> Check<()> {
    let alphabet: [PreviewHistoryAction; 4] = [
        PreviewHistoryAction::Observe,
        PreviewHistoryAction::ReplaceCurrent,
        PreviewHistoryAction::Withdraw,
        PreviewHistoryAction::ReplaceOriginal,
    ];
    let mut partitions: [usize; 3] = [0; 3];
    let mut histories: usize = 0;
    for ordinal in 0_usize..64 {
        let actions: [PreviewHistoryAction; 3] = [
            alphabet[ordinal / 16],
            alphabet[(ordinal / 4) % 4],
            alphabet[ordinal % 4],
        ];
        run_preview_history(actions, &mut partitions)?;
        histories += 1;
    }
    assert_eq!(histories, 64);
    assert_eq!(partitions, [48, 117, 27]);
    Ok(())
}
