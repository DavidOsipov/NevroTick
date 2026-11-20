// SPDX-License-Identifier: GPL-2.0-only
//! Process-retained laboratory source/endpoint policy at actual UDP effects.
//!
//! Stable source handles never arise from sessions or endpoints. This private
//! owner supplies no production configuration, persistent continuity, NTS trust,
//! clock authority or native qualification. Its explicit laboratory limits and
//! single process installation survive authority/session drop.
#![forbid(unsafe_code)]

use std::{
    error::Error,
    fmt,
    future::pending,
    io,
    net::{Ipv4Addr, Ipv6Addr, SocketAddr},
    pin::Pin,
    sync::{Arc, Mutex, MutexGuard, OnceLock, TryLockError},
    time::{Duration, Instant},
};

use garde::Validate;
use ntp_wire::RawHeader;
use tokio::sync::{Notify, futures::OwnedNotified, watch};

use super::{
    AcquisitionSession, LiveSlot, PACKET_BYTES, Phase, SEND_READY_CAP, SendBracket, SeriesConfig,
    SoftwareClock, StagedReply, TerminalOutcome, complete_send, finish_valid_reply,
};

/// Fixed maximum catalogue, not an adopted product source count.
const MAX_SOURCES: usize = 32;
/// Fixed non-evicting endpoint capacity ceiling.
const MAX_ENDPOINTS: usize = 64;
/// Maximum notification/selection work in one bounded policy transaction.
const MAX_RECORDS: usize = MAX_SOURCES + MAX_ENDPOINTS;
/// RATE control reference-ID bytes, interpreted only after request correlation.
const RATE: u32 = u32::from_be_bytes(*b"RATE");
/// DENY control reference-ID bytes.
const DENY: u32 = u32::from_be_bytes(*b"DENY");
/// RSTR control reference-ID bytes.
const RSTR: u32 = u32::from_be_bytes(*b"RSTR");
/// Retains the one successful process installation even with no live facade.
static PROCESS_POLICY: OnceLock<Arc<PolicyCell>> = OnceLock::new();

pub(crate) use configuration::SourceContext;
pub use configuration::{SourceAddressSet, SourceConfigurationSnapshot};

/// Time scale explicitly declared by the source's configuration authority.
/// A declaration supplies historical provenance only. It cannot authenticate
/// a peer, establish a mapping or date, or qualify a measurement.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceScale {
    /// The authority declared that the source serves unsmeared UTC.
    UnsmearedUtc,
    /// The authority declared that the source smears leap events.
    Smeared,
    /// No scale is established by this configuration.
    Unknown,
}

/// Sealed configuration construction and issuance within the actual policy owner.
mod configuration {
    use std::{fmt, num::NonZeroU64, sync::Arc};

    use garde::Validate;

    use super::{
        EndpointKey, PolicyGuard, RecordKey, SocketAddr, SourceHandle, SourceIdentity,
        SourcePolicyAuthority, SourcePolicyError, SourceScale,
    };

    /// Checked, complete allowed endpoints for one numeric laboratory source.
    /// Operation order, deadlines and experimental declarations are separate.
    /// Construction validates at most three loopback addresses before retaining
    /// canonical keys; failure retains no caller data or partial accepted set.
    ///
    /// ```
    /// use nevrotick_engine::SourceAddressSet;
    /// let peer = "127.0.0.1:123".parse()?;
    /// let set = SourceAddressSet::new(&[peer])?;
    /// assert!(set.allows(peer));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    ///
    /// Raw construction cannot bypass the complete checked-set constructor.
    /// ```compile_fail,E0451
    /// use nevrotick_engine::SourceAddressSet;
    /// let _set = SourceAddressSet { keys: [None; 3] };
    /// ```
    ///
    /// No default silently supplies an accepted empty definition.
    /// ```compile_fail,E0277
    /// use nevrotick_engine::SourceAddressSet;
    /// let _set: SourceAddressSet = Default::default();
    /// ```
    #[derive(Clone)]
    #[must_use = "an address set must be installed or explicitly discarded"]
    pub struct SourceAddressSet {
        /// Only successful checked construction populates this bounded array.
        keys: [Option<EndpointKey>; 3],
    }

    /// Raw address slice admitted by the mandatory complete-set Garde rule.
    #[derive(Validate)]
    struct AddressCandidate<'a> {
        /// One shared rule owns raw count, adapter admission and canonical uniqueness.
        #[garde(custom(address_rule))]
        values: &'a [SocketAddr],
    }

    /// Classify bounded raw input without allocation, effects or partial publication.
    fn address_issue(peers: &[SocketAddr]) -> Option<SourcePolicyError> {
        if peers.is_empty() || peers.len() > 3 {
            return Some(SourcePolicyError::InvalidSelection);
        }
        let mut keys = [None; 3];
        for (position, peer) in peers.iter().copied().enumerate() {
            if !crate::lab_peer_is_valid(peer) {
                return Some(SourcePolicyError::InvalidEndpoint);
            }
            let Ok(key) = EndpointKey::new(peer) else {
                return Some(SourcePolicyError::InvalidEndpoint);
            };
            if keys.contains(&Some(key)) {
                return Some(SourcePolicyError::InvalidSelection);
            }
            keys[position] = Some(key);
        }
        None
    }

    /// Adapt the sole mandatory address rule to Garde without exposing raw input.
    /// # Errors
    /// Reject invalid count, endpoint or canonical duplicate with fixed text.
    #[expect(
        clippy::trivially_copy_pass_by_ref,
        reason = "Garde custom rules borrow raw fields and their unit context"
    )]
    fn address_rule(peers: &&[SocketAddr], (): &()) -> garde::Result {
        if address_issue(peers).is_none() {
            Ok(())
        } else {
            Err(garde::Error::new("invalid source configuration addresses"))
        }
    }

    impl SourceAddressSet {
        /// Validate the entire raw set before retaining any canonical endpoint.
        /// Invalid Garde reports may allocate temporarily; success retains only
        /// the fixed array and performs no network or policy operation.
        /// # Errors
        /// Preserve count/duplicate versus endpoint refusal; no accepted value
        /// or permanent endpoint record is published on rejection.
        pub fn new(peers: &[SocketAddr]) -> Result<Self, SourcePolicyError> {
            AddressCandidate { values: peers }
                .validate()
                .map_err(|_| address_issue(peers).unwrap_or(SourcePolicyError::InvalidSelection))?;
            let mut keys = [None; 3];
            for (slot, peer) in keys.iter_mut().zip(peers) {
                *slot = Some(EndpointKey::new(*peer)?);
            }
            Ok(Self { keys })
        }

        /// Return the checked set's finite endpoint count.
        #[must_use]
        pub fn len(&self) -> usize {
            self.keys.iter().flatten().count()
        }

        /// Observe emptiness; supported construction always retains at least one key.
        #[must_use]
        pub fn is_empty(&self) -> bool {
            self.keys.iter().all(Option::is_none)
        }

        /// Compare canonical membership only; this historical query supplies no
        /// adapter admission, current configuration or send permission.
        #[must_use]
        pub fn allows(&self, peer: SocketAddr) -> bool {
            EndpointKey::new(peer).is_ok_and(|key| self.contains(key))
        }

        /// Test an already canonical key without interpreting it as current authority.
        fn contains(&self, key: EndpointKey) -> bool {
            self.keys.contains(&Some(key))
        }
    }

    impl fmt::Debug for SourceAddressSet {
        /// Redact endpoints, cardinality and caller configuration.
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("SourceAddressSet { .. }")
        }
    }

    /// Historical source-bound installed configuration; only the issuer constructs it.
    /// Cloning retains immutable data and the separate identity allocation, never
    /// a mutable policy cell, channel, socket or current permission.
    ///
    /// A consumer may retain and inspect genuine issued history.
    /// ```
    /// use nevrotick_engine::{SourceConfigurationSnapshot, SourceScale};
    /// fn inspect(snapshot: &SourceConfigurationSnapshot) -> SourceScale {
    ///     assert!(!snapshot.addresses().is_empty());
    ///     assert!(snapshot.same_configuration(&snapshot.clone()));
    ///     snapshot.declared_scale()
    /// }
    /// ```
    ///
    /// A consumer cannot retag a snapshot with a fabricated revision.
    /// ```compile_fail,E0616
    /// use nevrotick_engine::SourceConfigurationSnapshot;
    /// fn retag(snapshot: &mut SourceConfigurationSnapshot) {
    ///     snapshot.revision = std::num::NonZeroU64::MIN;
    /// }
    /// ```
    ///
    /// A declaration can change only by issuing an actual new configuration.
    /// ```compile_fail,E0616
    /// use nevrotick_engine::{SourceConfigurationSnapshot, SourceScale};
    /// fn retag(snapshot: &mut SourceConfigurationSnapshot) {
    ///     snapshot.scale = SourceScale::UnsmearedUtc;
    /// }
    /// ```
    ///
    /// A checked address set alone is not an issuer-bound configuration.
    /// ```compile_fail,E0277
    /// use nevrotick_engine::{SourceAddressSet, SourceConfigurationSnapshot};
    /// fn forge(set: SourceAddressSet) -> SourceConfigurationSnapshot { set.into() }
    /// ```
    #[derive(Clone)]
    #[must_use = "configuration history must not be mistaken for current permission"]
    pub struct SourceConfigurationSnapshot {
        /// Genuine original catalogue source.
        source: SourceIdentity,
        /// Nonzero, checked, never-recycled configuration revision.
        revision: NonZeroU64,
        /// Immutable definition coupled to this source/revision by the sole issuer.
        addresses: SourceAddressSet,
        /// Original scale declaration, bound to this same immutable revision.
        scale: SourceScale,
        /// Optional immutable KE target, issued in the same revision as the UDP set.
        /// Numeric-only installation/replacement deliberately supplies None.
        #[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
        preview_target: Option<crate::KePreviewTarget>,
    }

    impl SourceConfigurationSnapshot {
        /// Borrow original source identity without reading current policy.
        #[must_use]
        pub const fn source_identity(&self) -> &SourceIdentity {
            &self.source
        }

        /// Borrow original accepted addresses, without current admission.
        #[must_use]
        pub const fn addresses(&self) -> &SourceAddressSet {
            &self.addresses
        }

        /// Return the original declaration without inferring scale from packets
        /// or reading a later configuration.
        #[must_use]
        pub const fn declared_scale(&self) -> SourceScale {
            self.scale
        }

        /// Compare actual source/revision identity, not endpoint-set value equality.
        #[must_use]
        pub fn same_configuration(&self, other: &Self) -> bool {
            // The issuer binds addresses, scale and any KE target immutably to
            // source/revision. Only the identity participates in this comparison.
            let Self {
                source,
                revision,
                addresses: _,
                scale: _,
                #[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
                    preview_target: _,
            } = self;
            let Self {
                source: other_source,
                revision: other_revision,
                addresses: _,
                scale: _,
                #[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
                    preview_target: _,
            } = other;
            source.same_source(other_source) && revision == other_revision
        }
    }

    impl fmt::Debug for SourceConfigurationSnapshot {
        /// Redact addresses and all correlatable source/revision values.
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("SourceConfigurationSnapshot { .. }")
        }
    }

    /// One sealed source plus optional actually installed configuration context.
    #[derive(Clone)]
    pub(crate) struct SourceContext {
        /// Actual stable catalogue source, including for explicitly unconfigured work.
        source: SourceIdentity,
        /// None denotes genuine historical lack of an installed configuration.
        configuration: Option<SourceConfigurationSnapshot>,
    }

    impl SourceContext {
        /// Borrow original source identity without observing later state.
        pub(crate) const fn identity(&self) -> &SourceIdentity {
            &self.source
        }

        /// Borrow only the original issuer-supplied configuration.
        pub(crate) const fn configuration(&self) -> Option<&SourceConfigurationSnapshot> {
            self.configuration.as_ref()
        }

        /// Compare the complete original managed/unmanaged receiving context.
        pub(crate) fn same_optional(left: Option<&Self>, right: Option<&Self>) -> bool {
            match (left, right) {
                (None, None) => true,
                (Some(left), Some(right)) => {
                    let Self {
                        source,
                        configuration,
                    } = left;
                    let Self {
                        source: other_source,
                        configuration: other_configuration,
                    } = right;
                    source.same_source(other_source)
                        && match (configuration, other_configuration) {
                            (None, None) => true,
                            (Some(a), Some(b)) => a.same_configuration(b),
                            (None, Some(_)) | (Some(_), None) => false,
                        }
                }
                (None, Some(_)) | (Some(_), None) => false,
            }
        }

        /// Derive context from the actual source and its same-cell mutex custody.
        /// # Errors
        /// Refuse foreign custody, exhausted configuration or any selected key
        /// outside the installed definition before endpoint publication.
        pub(super) fn capture(
            source: &SourceHandle,
            guard: &PolicyGuard<'_>,
            keys: &[Option<EndpointKey>; 3],
        ) -> Result<Self, SourcePolicyError> {
            if !std::ptr::eq(guard.cell, source.cell.as_ref()) {
                return Err(SourcePolicyError::Foreign);
            }
            let state = guard.state()?;
            state.record(RecordKey::Source(source.index))?;
            let configuration = state
                .configurations
                .get(source.index)
                .ok_or(SourcePolicyError::UnknownSource)?;
            let identity = source.identity();
            if !configuration.source.same_source(&identity) {
                return Err(SourcePolicyError::Foreign);
            }
            let snapshot = configuration.snapshot()?;
            if snapshot.as_ref().is_some_and(|value| {
                keys.iter()
                    .flatten()
                    .any(|key| !value.addresses.contains(*key))
            }) {
                return Err(SourcePolicyError::Foreign);
            }
            Ok(Self {
                source: identity,
                configuration: snapshot,
            })
        }
    }

    /// Actual per-source canonical state, private to the configuration issuer.
    pub(super) struct ConfigState {
        /// Genuine identity established with the real bounded catalogue.
        source: SourceIdentity,
        /// Current installed snapshot; absence never supplies a positive witness.
        current: Option<SourceConfigurationSnapshot>,
        /// Sticky checked-revision exhaustion, independent of every `KoD` field.
        exhausted: bool,
    }

    impl ConfigState {
        /// Initialize actual catalogue ownership without installing configuration.
        pub(super) const fn new(source: SourceIdentity) -> Self {
            Self {
                source,
                current: None,
                exhausted: false,
            }
        }

        /// Observe installed history without making exhausted state look unconfigured.
        /// # Errors
        /// Preserve sticky configuration unavailability.
        pub(super) fn snapshot(
            &self,
        ) -> Result<Option<SourceConfigurationSnapshot>, SourcePolicyError> {
            if self.exhausted {
                Err(SourcePolicyError::ConfigurationUnavailable)
            } else {
                Ok(self.current.clone())
            }
        }

        /// Recheck the complete original context against this actual current source.
        /// # Errors
        /// Refuse foreign source, exhausted/stale configuration or an outside peer.
        pub(super) fn check(
            &self,
            context: &SourceContext,
            key: EndpointKey,
        ) -> Result<(), SourcePolicyError> {
            if !self.source.same_source(&context.source) {
                return Err(SourcePolicyError::Foreign);
            }
            let current = self.snapshot()?;
            match (current.as_ref(), context.configuration.as_ref()) {
                (None, None) => Ok(()),
                (Some(actual), Some(original)) if actual.same_configuration(original) => {
                    if actual.addresses.contains(key) {
                        Ok(())
                    } else {
                        Err(SourcePolicyError::Foreign)
                    }
                }
                _ => Err(SourcePolicyError::ConfigurationChanged),
            }
        }
    }

    /// Complete staged definition; accepted input types own their checked invariants.
    /// The target already owns checked shared data; cloning allocates nothing.
    struct Definition {
        /// The complete original allowed UDP endpoints.
        addresses: SourceAddressSet,
        /// Declaration provided by the authority for this exact definition.
        scale: SourceScale,
        /// None withdraws KE without changing the numeric source definition.
        #[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
        preview_target: Option<crate::KePreviewTarget>,
    }

    /// The actual owner of installation or the genuine expected replacement revision.
    enum ConfigurationUpdate<'a> {
        /// Initial publication for an actual same-root catalogue source.
        Install(&'a SourceHandle),
        /// Replacement targets only the source named by this issuer-bound snapshot.
        Replace(&'a SourceConfigurationSnapshot),
    }

    /// Publish every numeric/KE definition through the same checked revision kernel.
    /// # Errors
    /// Foreign, unknown, installed or stale targets refuse without publication.
    /// Revision exhaustion withdraws issuance, retaining every `KoD` field and floor.
    fn publish(
        authority: &mut SourcePolicyAuthority,
        update: &ConfigurationUpdate<'_>,
        definition: Definition,
    ) -> Result<SourceConfigurationSnapshot, SourcePolicyError> {
        let (index, expected): (usize, Option<&SourceConfigurationSnapshot>) = match update {
            ConfigurationUpdate::Install(source) => {
                if !Arc::ptr_eq(&authority.cell, &source.cell) {
                    return Err(SourcePolicyError::Foreign);
                }
                (source.index, None)
            }
            ConfigurationUpdate::Replace(snapshot) => {
                if !Arc::ptr_eq(&authority.cell.identity_domain, &snapshot.source.domain) {
                    return Err(SourcePolicyError::Foreign);
                }
                (snapshot.source.index, Some(snapshot))
            }
        };
        let mut guard: PolicyGuard<'_> = authority.cell.lock()?;
        let state: &mut super::PolicyState = guard.state_mut()?;
        let config: &mut ConfigState = state
            .configurations
            .get_mut(index)
            .ok_or(SourcePolicyError::UnknownSource)?;
        let record: &mut super::Record = state
            .sources
            .get_mut(index)
            .ok_or(SourcePolicyError::UnknownSource)?;
        if config.exhausted {
            return Err(SourcePolicyError::ConfigurationUnavailable);
        }
        let revision: NonZeroU64 = match (expected, config.current.as_ref()) {
            (None, None) => NonZeroU64::MIN,
            (Some(expected), Some(current)) if current.same_configuration(expected) => {
                let successor: Option<NonZeroU64> = current
                    .revision
                    .get()
                    .checked_add(1_u64)
                    .and_then(NonZeroU64::new);
                let Some(revision): Option<NonZeroU64> = successor else {
                    config.current = None;
                    config.exhausted = true;
                    record.changed = true;
                    return Err(SourcePolicyError::ConfigurationUnavailable);
                };
                revision
            }
            _ => return Err(SourcePolicyError::ConfigurationChanged),
        };
        let snapshot: SourceConfigurationSnapshot = SourceConfigurationSnapshot {
            source: config.source.clone(),
            revision,
            addresses: definition.addresses,
            scale: definition.scale,
            #[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
            preview_target: definition.preview_target,
        };
        config.current = Some(snapshot.clone());
        record.changed = true;
        Ok(snapshot)
    }

    /// Install checked numeric endpoints while explicitly leaving KE unavailable.
    /// # Errors
    /// Preserve the shared publication kernel's unchanged refusal semantics.
    pub(super) fn install(
        authority: &mut SourcePolicyAuthority,
        source: &SourceHandle,
        addresses: SourceAddressSet,
    ) -> Result<SourceConfigurationSnapshot, SourcePolicyError> {
        let definition: Definition = Definition {
            addresses,
            scale: SourceScale::Unknown,
            #[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
            preview_target: None,
        };
        publish(authority, &ConfigurationUpdate::Install(source), definition)
    }

    /// Numeric replacement withdraws any prior KE target in the same new revision.
    /// # Errors
    /// Preserve current-revision refusal and sticky checked exhaustion.
    pub(super) fn replace(
        authority: &mut SourcePolicyAuthority,
        expected: &SourceConfigurationSnapshot,
        addresses: SourceAddressSet,
    ) -> Result<SourceConfigurationSnapshot, SourcePolicyError> {
        let definition: Definition = Definition {
            addresses,
            scale: SourceScale::Unknown,
            #[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
            preview_target: None,
        };
        publish(
            authority,
            &ConfigurationUpdate::Replace(expected),
            definition,
        )
    }

    /// Stage a scale-only replacement of the actual expected definition.
    /// # Errors
    /// Preserve foreign/stale refusal and sticky revision exhaustion in publication.
    pub(super) fn replace_scale(
        authority: &mut SourcePolicyAuthority,
        expected: &SourceConfigurationSnapshot,
        scale: SourceScale,
    ) -> Result<SourceConfigurationSnapshot, SourcePolicyError> {
        let definition: Definition = Definition {
            addresses: expected.addresses.clone(),
            scale,
            #[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
            preview_target: expected.preview_target.clone(),
        };
        publish(
            authority,
            &ConfigurationUpdate::Replace(expected),
            definition,
        )
    }

    /// Stage checked immutable TLS inputs before installing their actual source revision.
    /// # Errors
    /// The shared kernel refuses foreign, installed, unavailable or unknown sources.
    #[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
    pub(super) fn install_preview(
        authority: &mut SourcePolicyAuthority,
        source: &SourceHandle,
        addresses: SourceAddressSet,
        target: crate::KePreviewTarget,
    ) -> Result<SourceConfigurationSnapshot, SourcePolicyError> {
        let definition: Definition = Definition {
            addresses,
            scale: SourceScale::Unknown,
            preview_target: Some(target),
        };
        publish(authority, &ConfigurationUpdate::Install(source), definition)
    }

    /// Replace one actual current definition with checked TLS/UDP inputs atomically.
    /// # Errors
    /// Preserve the same stale, foreign and exhaustion rules as numeric replacement.
    #[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
    pub(super) fn replace_preview(
        authority: &mut SourcePolicyAuthority,
        expected: &SourceConfigurationSnapshot,
        addresses: SourceAddressSet,
        target: crate::KePreviewTarget,
    ) -> Result<SourceConfigurationSnapshot, SourcePolicyError> {
        let definition: Definition = Definition {
            addresses,
            scale: SourceScale::Unknown,
            preview_target: Some(target),
        };
        publish(
            authority,
            &ConfigurationUpdate::Replace(expected),
            definition,
        )
    }

    /// Capture only an actual installed KE definition, never independently tagged parts.
    /// # Errors
    /// Missing/numeric-only definitions refuse; current source restriction,
    /// exhausted configuration and unavailable custody remain explicit failures.
    #[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
    pub(super) fn capture_preview(
        source: &SourceHandle,
    ) -> Result<super::ConfiguredKePreview, SourcePolicyError> {
        let guard: PolicyGuard<'_> = source.cell.lock()?;
        let state: &super::PolicyState = guard.state()?;
        let config: &ConfigState = state
            .configurations
            .get(source.index)
            .ok_or(SourcePolicyError::UnknownSource)?;
        let snapshot: SourceConfigurationSnapshot = config
            .snapshot()?
            .ok_or(SourcePolicyError::ConfigurationChanged)?;
        let target: crate::KePreviewTarget = snapshot
            .preview_target
            .clone()
            .ok_or(SourcePolicyError::ConfigurationChanged)?;
        let captured: super::ConfiguredKePreview = super::ConfiguredKePreview {
            source: source.clone(),
            snapshot,
            target,
        };
        check_preview(&captured, &guard, None)?;
        Ok(captured)
    }

    /// Inspect actual custody and original definition without allocation or mutation.
    /// # Errors
    /// Refuse foreign/stale/unavailable issuance, a restricted source or selected
    /// endpoint, and an endpoint outside the original still-current allowed set.
    #[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
    pub(super) fn check_preview(
        original: &super::ConfiguredKePreview,
        guard: &PolicyGuard<'_>,
        peer: Option<SocketAddr>,
    ) -> Result<(), SourcePolicyError> {
        if !std::ptr::eq(guard.cell, original.source.cell.as_ref())
            || !Arc::ptr_eq(
                &original.source.cell.identity_domain,
                &original.snapshot.source.domain,
            )
            || original.source.index != original.snapshot.source.index
        {
            return Err(SourcePolicyError::Foreign);
        }
        let state: &super::PolicyState = guard.state()?;
        let source_record: &super::Record =
            state.record(RecordKey::Source(original.source.index))?;
        let config: &ConfigState = state
            .configurations
            .get(original.source.index)
            .ok_or(SourcePolicyError::UnknownSource)?;
        if !config.source.same_source(&original.snapshot.source) {
            return Err(SourcePolicyError::Foreign);
        }
        if config.exhausted {
            return Err(SourcePolicyError::ConfigurationUnavailable);
        }
        let current: &SourceConfigurationSnapshot = config
            .current
            .as_ref()
            .ok_or(SourcePolicyError::ConfigurationChanged)?;
        if !current.same_configuration(&original.snapshot) {
            return Err(SourcePolicyError::ConfigurationChanged);
        }
        // Actual private issuance binds the immutable target to this exact
        // revision. Numeric replacement advances revision and withdraws it.
        if current.preview_target.is_none() {
            return Err(SourcePolicyError::ConfigurationChanged);
        }
        source_record.available()?;
        if let Some(peer) = peer {
            if !crate::lab_peer_is_valid(peer) {
                return Err(SourcePolicyError::InvalidEndpoint);
            }
            let key: EndpointKey = EndpointKey::new(peer)?;
            if !original.snapshot.addresses.contains(key) || !current.addresses.contains(key) {
                return Err(SourcePolicyError::Foreign);
            }
            if let Some(endpoint) = state.endpoints.iter().find(|entry| entry.key == Some(key)) {
                endpoint.record.available()?;
            }
        }
        Ok(())
    }

    /// Force the genuine installed revision to its last valid value for a boundary test.
    /// # Errors
    /// Refuse unknown, absent or already exhausted configuration.
    #[cfg(test)]
    pub(super) fn force_max(
        source: &SourceHandle,
    ) -> Result<SourceConfigurationSnapshot, SourcePolicyError> {
        let mut guard = source.cell.lock()?;
        let state = guard.state_mut()?;
        let config = state
            .configurations
            .get_mut(source.index)
            .ok_or(SourcePolicyError::UnknownSource)?;
        let record = state
            .sources
            .get_mut(source.index)
            .ok_or(SourcePolicyError::UnknownSource)?;
        if config.exhausted {
            return Err(SourcePolicyError::ConfigurationUnavailable);
        }
        let current = config
            .current
            .as_mut()
            .ok_or(SourcePolicyError::ConfigurationChanged)?;
        current.revision = NonZeroU64::MAX;
        record.changed = true;
        Ok(current.clone())
    }
}

/// Test only the actual installed owner's final representable revision.
/// # Errors
/// Preserve actual custody and absent/exhausted configuration failures.
#[cfg(test)]
fn force_configuration_revision_max_for_test(
    source: &SourceHandle,
) -> Result<SourceConfigurationSnapshot, SourcePolicyError> {
    configuration::force_max(source)
}

/// Sealed read capabilities preserve the real guarded observation positions.
pub(super) mod observations {
    use super::{Instant, SoftwareClock};

    /// A single deferred owner-clock read; construction samples nothing.
    pub(super) struct InstantRead {
        /// Only this leaf can select hardware or a test-only fixed observation.
        source: InstantSource,
    }

    /// Closed observation alternatives; fixed coordinates never enter production.
    enum InstantSource {
        /// Read freshly only when the guarded owner consumes the capability.
        Hardware,
        /// Test-only fixed coordinate with an independent consumption observer.
        #[cfg(test)]
        Fixed(Instant, std::rc::Rc<std::cell::Cell<usize>>),
    }

    impl InstantRead {
        /// Construct an unsampled production capability.
        pub(super) fn hardware() -> Self {
            Self {
                source: InstantSource::Hardware,
            }
        }

        /// Consume exactly one observation at the original guarded read point.
        pub(super) fn read(self) -> Instant {
            match self.source {
                InstantSource::Hardware => Instant::now(),
                #[cfg(test)]
                InstantSource::Fixed(value, count) => {
                    count.set(1);
                    value
                }
            }
        }

        /// Issue one modeled coordinate and its read-only consumption observer.
        #[cfg(test)]
        pub(super) fn fixed(value: Instant) -> (Self, ReadCount) {
            let count = std::rc::Rc::new(std::cell::Cell::new(0));
            (
                Self {
                    source: InstantSource::Fixed(value, std::rc::Rc::clone(&count)),
                },
                ReadCount { count },
            )
        }
    }

    /// Two deferred captures surrounding the one actual synchronous send.
    pub(in super::super) struct SendReads {
        /// Private hardware/test selection, never an arbitrary callback.
        source: SendSource,
    }

    /// Closed capture alternatives; the test script has exactly two entries.
    enum SendSource {
        /// Capture actual software brackets at each original effect position.
        Hardware,
        /// Fixed test captures and their bounded next-read index.
        #[cfg(test)]
        Fixed {
            /// Original before/after captures, copied only when consumed.
            values: [SoftwareClock; 2],
            /// Next index, bounded by the fixed two-entry script.
            next: usize,
            /// Shared solely with an immutable test observation handle.
            count: std::rc::Rc<std::cell::Cell<usize>>,
        },
    }

    impl SendReads {
        /// Construct an unsampled production capture capability.
        pub(in super::super) fn hardware() -> Self {
            Self {
                source: SendSource::Hardware,
            }
        }

        /// Capture at the current original guard/effect position.
        /// # Panics
        /// A test fixture reading beyond its two declared captures fails clearly.
        pub(in super::super) fn capture(&mut self) -> SoftwareClock {
            match &mut self.source {
                SendSource::Hardware => SoftwareClock::capture(),
                #[cfg(test)]
                SendSource::Fixed {
                    values,
                    next,
                    count,
                } => {
                    assert!(
                        *next < values.len(),
                        "fixed sender capture script exhausted"
                    );
                    let value = values[*next];
                    *next += 1;
                    count.set(*next);
                    value
                }
            }
        }

        /// Issue exactly two coherent monotonic captures and a read observer.
        /// Wall-clock readings retain their original independent values and may step.
        /// # Panics
        /// Reject an inverted individual bracket or cross-capture monotonic order.
        #[cfg(test)]
        pub(in super::super) fn fixed(
            before: SoftwareClock,
            after: SoftwareClock,
        ) -> (Self, ReadCount) {
            assert!(
                before.mono <= before.mono_after
                    && before.mono_after <= after.mono
                    && after.mono <= after.mono_after,
                "fixed sender captures must have coherent monotonic brackets"
            );
            Self::fixed_raw(before, after)
        }

        /// Issue exactly two unvalidated captures for a sender-boundary refusal test.
        /// Construction consumes neither value and does not grant send authority.
        #[cfg(test)]
        pub(in super::super) fn fixed_raw(
            before: SoftwareClock,
            after: SoftwareClock,
        ) -> (Self, ReadCount) {
            let count: std::rc::Rc<std::cell::Cell<usize>> =
                std::rc::Rc::new(std::cell::Cell::new(0));
            (
                Self {
                    source: SendSource::Fixed {
                        values: [before, after],
                        next: 0,
                        count: std::rc::Rc::clone(&count),
                    },
                },
                ReadCount { count },
            )
        }
    }

    /// Test-only inert observation of actual capability consumption.
    #[cfg(test)]
    pub(in super::super) struct ReadCount {
        /// Private counter; callers can observe but cannot manufacture a count.
        count: std::rc::Rc<std::cell::Cell<usize>>,
    }

    #[cfg(test)]
    impl ReadCount {
        /// Return the number of actual reads through the issued capability.
        pub(in super::super) fn reads(&self) -> usize {
            self.count.get()
        }
    }
}

/// Closed, payload-free policy refusals; none contains a peer or request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourcePolicyError {
    /// Raw laboratory limits failed their mandatory constructor.
    InvalidLimits,
    /// A successful process installation already exists.
    AlreadyInstalled,
    /// Bounded policy storage could not be reserved.
    Allocation,
    /// The requested catalogue source does not exist.
    UnknownSource,
    /// An endpoint has not been bound by this owner.
    UnknownEndpoint,
    /// A numeric endpoint lacks its required port or zone.
    InvalidEndpoint,
    /// All preallocated endpoint slots are already retained.
    Capacity,
    /// A short policy transaction currently owns the cell.
    Contended,
    /// A panicked policy transaction made the cell unavailable.
    Poisoned,
    /// A handle or token belongs to another actual cell.
    Foreign,
    /// A record revision, outstanding request or correlation epoch is stale.
    Stale,
    /// The original optional configuration is no longer current.
    ConfigurationChanged,
    /// Configuration identity exhausted and cannot be issued again.
    ConfigurationUnavailable,
    /// A re-enable selection contains the same record more than once.
    Duplicate,
    /// A bounded selection has invalid length or duplicate configuration addresses.
    InvalidSelection,
    /// An applicable record retains a server refusal.
    Disabled,
    /// An applicable record exhausted representable arithmetic or identity.
    Unavailable,
    /// An applicable earliest-send floor has not yet been reached.
    NotDue,
    /// A stratum-zero code has no selected control behavior.
    UnhandledControl,
}

impl fmt::Display for SourcePolicyError {
    /// Render only the closed category, never policy/input payloads.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidLimits => "invalid source policy limits",
            Self::AlreadyInstalled => "source policy already installed",
            Self::Allocation => "source policy allocation refused",
            Self::UnknownSource => "unknown source policy source",
            Self::UnknownEndpoint => "unknown source policy endpoint",
            Self::InvalidEndpoint => "invalid source policy endpoint",
            Self::Capacity => "source policy capacity exhausted",
            Self::Contended => "source policy contended",
            Self::Poisoned => "source policy poisoned",
            Self::Foreign => "foreign source policy owner",
            Self::Stale => "stale source policy identity",
            Self::ConfigurationChanged => "source configuration changed",
            Self::ConfigurationUnavailable => "source configuration unavailable",
            Self::Duplicate => "duplicate source policy target",
            Self::InvalidSelection => "invalid source policy selection",
            Self::Disabled => "source policy refused",
            Self::Unavailable => "source policy unavailable",
            Self::NotDue => "source policy not due",
            Self::UnhandledControl => "unhandled source control",
        })
    }
}

impl Error for SourcePolicyError {}

/// Raw constructor fields share one mandatory Garde admission rule.
#[derive(Validate)]
struct LimitsCandidate {
    /// Catalogue/capacity and both positive interval bounds are checked together.
    #[garde(custom(limits_rule))]
    values: (usize, usize, Duration, Duration),
}

/// Admit finite storage and intervals representable from actual construction time.
/// # Errors
/// Reject zero/oversized counts, zero/inverted intervals or deadline overflow.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "Garde custom rules borrow raw fields and their unit context"
)]
fn limits_rule(values: &(usize, usize, Duration, Duration), (): &()) -> garde::Result {
    let (sources, endpoints, initial, maximum) = *values;
    if !(1..=MAX_SOURCES).contains(&sources)
        || !(1..=MAX_ENDPOINTS).contains(&endpoints)
        || initial.is_zero()
        || initial > maximum
        || Instant::now().checked_add(maximum).is_none()
    {
        return Err(garde::Error::new("invalid laboratory source policy limits"));
    }
    Ok(())
}

/// Immutable checked laboratory capacities and intervals; no shipped default.
#[derive(Clone, Copy)]
pub struct SourcePolicyLimits {
    /// Fixed catalogue length, admitted before any source handle exists.
    source_count: usize,
    /// Preallocated endpoint count, never evicted or reused.
    endpoint_capacity: usize,
    /// Independent initial interval for every new record.
    initial_interval: Duration,
    /// Saturation bound, immutable throughout this root's lifetime.
    maximum_interval: Duration,
}

impl SourcePolicyLimits {
    /// Admit all raw limits through the sole mandatory Garde constructor.
    /// # Errors
    /// Refuse invalid capacities/intervals before installing state or handles.
    pub fn new(
        source_count: usize,
        endpoint_capacity: usize,
        initial_interval: Duration,
        maximum_interval: Duration,
    ) -> Result<Self, SourcePolicyError> {
        LimitsCandidate {
            values: (
                source_count,
                endpoint_capacity,
                initial_interval,
                maximum_interval,
            ),
        }
        .validate()
        .map_err(|_| SourcePolicyError::InvalidLimits)?;
        Ok(Self {
            source_count,
            endpoint_capacity,
            initial_interval,
            maximum_interval,
        })
    }
}

impl fmt::Debug for SourcePolicyLimits {
    /// Redact laboratory policy values.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SourcePolicyLimits { .. }")
    }
}

/// Immutable historical facts about one actual restriction record.
///
/// Copies retain the original observation; neither the values nor a zero wait
/// grant current send permission, re-enable authority or fresh synchronization.
#[must_use]
#[derive(Clone, Copy)]
pub struct SourceRestrictionSnapshot {
    /// This record's own retained interval, independent of the other record.
    interval: Duration,
    /// The same checked maximum that governed the observed record.
    maximum_interval: Duration,
    /// Original admitted earliest-send floor, including an already elapsed floor.
    floor: Option<Instant>,
    /// Remaining wait at the enclosing snapshot's one owned observation time.
    remaining_at_capture: Duration,
    /// Retained direct DENY/RSTR fact, including on an unavailable record.
    refused: bool,
    /// Sticky representational unavailability, never hidden as a clear record.
    unavailable: bool,
    /// Historical ordinary revision; this value is not a re-enable target.
    revision: u64,
}

impl SourceRestrictionSnapshot {
    /// Copy one guarded record and compute its wait only at the captured time.
    fn capture(record: &Record, maximum_interval: Duration, captured_at: Instant) -> Self {
        Self {
            interval: record.interval,
            maximum_interval,
            floor: record.floor,
            remaining_at_capture: record.floor.map_or(Duration::ZERO, |floor| {
                floor.saturating_duration_since(captured_at)
            }),
            refused: record.refused,
            unavailable: record.unavailable,
            revision: record.revision,
        }
    }

    /// Return this record's own interval at capture.
    #[must_use]
    pub fn interval(&self) -> Duration {
        self.interval
    }

    /// Return the checked maximum retained by this policy owner at capture.
    #[must_use]
    pub fn maximum_interval(&self) -> Duration {
        self.maximum_interval
    }

    /// Report whether this record's own interval reached that checked maximum.
    #[must_use]
    pub fn at_maximum(&self) -> bool {
        self.interval == self.maximum_interval
    }

    /// Return the retained earliest-send floor without interpreting it as authority.
    #[must_use]
    pub fn floor(&self) -> Option<Instant> {
        self.floor
    }

    /// Return the wait at capture; later calls do not resample or count it down.
    #[must_use]
    pub fn remaining_at_capture(&self) -> Duration {
        self.remaining_at_capture
    }

    /// Return the retained direct refusal, independently of availability or wait.
    #[must_use]
    pub fn refused(&self) -> bool {
        self.refused
    }

    /// Return sticky unavailability without erasing any other retained fact.
    #[must_use]
    pub fn unavailable(&self) -> bool {
        self.unavailable
    }

    /// Return the historical revision; it cannot authorize a policy operation.
    #[must_use]
    pub fn revision(&self) -> u64 {
        self.revision
    }
}

impl fmt::Debug for SourceRestrictionSnapshot {
    /// Redact restriction values and historical timing/revision facts.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SourceRestrictionSnapshot { .. }")
    }
}

/// One immutable source/endpoint observation under the actual policy mutex.
///
/// It contains no binding, request, re-enable target or current permission.
/// Copies remain historical even when the owner subsequently changes.
/// External callers cannot forge a replacement record:
/// ```compile_fail,E0451
/// use nevrotick_engine::SourcePolicySnapshot;
/// fn forge(snapshot: SourcePolicySnapshot) -> SourcePolicySnapshot {
///     SourcePolicySnapshot { ..snapshot }
/// }
/// ```
/// An absent observation cannot be manufactured as a clear default:
/// ```compile_fail,E0599
/// use nevrotick_engine::SourcePolicySnapshot;
/// let snapshot = SourcePolicySnapshot::default();
/// ```
/// Only the actual source owner can observe an already-bound endpoint:
/// ```
/// use std::net::SocketAddr;
/// use nevrotick_engine::{SourceHandle, SourcePolicyError, SourcePolicySnapshot};
/// fn observe(source: &SourceHandle, peer: SocketAddr)
///     -> Result<SourcePolicySnapshot, SourcePolicyError> {
///     source.policy_snapshot(peer)
/// }
/// ```
#[must_use]
#[derive(Clone, Copy)]
pub struct SourcePolicySnapshot {
    /// Actual logical-source facts from this capture.
    source: SourceRestrictionSnapshot,
    /// Actual canonical-endpoint facts from the same capture.
    endpoint: SourceRestrictionSnapshot,
    /// One owned clock observation sampled after obtaining mutex custody.
    captured_at: Instant,
}

impl SourcePolicySnapshot {
    /// Copy both actual records while the caller retains the same mutex guard.
    /// # Errors
    /// Refuse unknown source or unbound endpoint positions without mutation.
    fn capture(
        guard: &PolicyGuard<'_>,
        source: usize,
        endpoint: usize,
    ) -> Result<Self, SourcePolicyError> {
        let state = guard.state()?;
        let source = state.record(RecordKey::Source(source))?;
        let endpoint = state.record(RecordKey::Endpoint(endpoint))?;
        let captured_at = Instant::now();
        let maximum = state.limits.maximum_interval;
        Ok(Self {
            source: SourceRestrictionSnapshot::capture(source, maximum, captured_at),
            endpoint: SourceRestrictionSnapshot::capture(endpoint, maximum, captured_at),
            captured_at,
        })
    }

    /// Borrow historical logical-source facts, not current source permission.
    #[must_use]
    pub fn source(&self) -> &SourceRestrictionSnapshot {
        &self.source
    }

    /// Borrow historical endpoint facts, not current endpoint permission.
    #[must_use]
    pub fn endpoint(&self) -> &SourceRestrictionSnapshot {
        &self.endpoint
    }

    /// Return the one monotonic observation shared by both retained records.
    #[must_use]
    pub fn captured_at(&self) -> Instant {
        self.captured_at
    }

    /// Write exactly two fixed-scope diagnostic rows after policy custody ends.
    ///
    /// Each row is below 320 bytes even at maximum `Duration` values. No row
    /// includes addresses, source indices, revisions, packet or request values.
    /// The writer allocates no buffer and asserts no present permission or fresh
    /// synchronization. Repeated writes describe the same historical capture.
    /// # Errors
    /// Preserve the sink error and any partial output; neither failure nor a
    /// successful write changes this snapshot or the actual policy owner.
    pub fn write_status(&self, output: &mut impl io::Write) -> io::Result<()> {
        for (scope, record) in [("source", &self.source), ("endpoint", &self.endpoint)] {
            writeln!(
                output,
                concat!(
                    "event=source_policy scope={} interval_ns={} maximum_ns={} ",
                    "remaining_at_capture_ns={} refused={} unavailable={} ",
                    "at_maximum={} eligible=0 fresh_sync=0"
                ),
                scope,
                record.interval.as_nanos(),
                record.maximum_interval.as_nanos(),
                record.remaining_at_capture.as_nanos(),
                u8::from(record.refused),
                u8::from(record.unavailable),
                u8::from(record.at_maximum()),
            )?;
        }
        Ok(())
    }
}

impl fmt::Debug for SourcePolicySnapshot {
    /// Redact the captured clock coordinate and both restriction records.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SourcePolicySnapshot { .. }")
    }
}

/// Canonical remote identity; local socket and IPv6 flow information are absent.
#[derive(Clone, Copy, Eq, PartialEq)]
enum EndpointKey {
    /// IPv4, including normalized IPv4-mapped aliases.
    V4(Ipv4Addr, u16),
    /// IPv6 with its required zone, or zero for an unscoped address.
    V6(Ipv6Addr, u16, u32),
}

impl EndpointKey {
    /// Normalize endpoint identity without widening the actual socket adapter.
    /// # Errors
    /// Refuse zero ports or a missing zone for a scoped IPv6 destination.
    fn new(peer: SocketAddr) -> Result<Self, SourcePolicyError> {
        if peer.port() == 0 {
            return Err(SourcePolicyError::InvalidEndpoint);
        }
        match peer {
            SocketAddr::V4(address) => Ok(Self::V4(*address.ip(), address.port())),
            SocketAddr::V6(address) => {
                let ip = *address.ip();
                if let Some(mapped) = ip.to_ipv4_mapped() {
                    return Ok(Self::V4(mapped, address.port()));
                }
                let scoped = ip.is_unicast_link_local()
                    || (ip.is_multicast() && ip.segments()[0] & 0x000f != 0x000e);
                if scoped && address.scope_id() == 0 {
                    return Err(SourcePolicyError::InvalidEndpoint);
                }
                Ok(Self::V6(
                    ip,
                    address.port(),
                    if scoped { address.scope_id() } else { 0 },
                ))
            }
        }
    }
}

/// One independently retained restriction and its actual-change channel.
struct Record {
    /// This record's own interval, never an inherited effective maximum.
    interval: Duration,
    /// An admitted earliest send; None means no attempt/control floor exists yet.
    floor: Option<Instant>,
    /// A direct DENY/RSTR, cleared only by the root authority.
    refused: bool,
    /// Sticky representational failure; ordinary re-enable cannot clear it.
    unavailable: bool,
    /// Ordinary state revision, including actual allowance reservations.
    revision: u64,
    /// Changes only for refusal/re-enable, independently of ordinary floors.
    epoch: u64,
    /// Changed state awaiting notification after the lock is released.
    changed: bool,
    /// Coalesced actual changes; read-only lock release does not publish here.
    changes: watch::Sender<()>,
}

impl Record {
    /// Allocate the actual-change channel before publication or send custody.
    fn new(interval: Duration) -> Self {
        let (changes, receiver) = watch::channel(());
        drop(receiver);
        Self {
            interval,
            floor: None,
            refused: false,
            unavailable: false,
            revision: 1,
            epoch: 1,
            changed: false,
            changes,
        }
    }

    /// Snapshot inert transition fields without copying notification authority.
    fn update(&self) -> Update {
        Update {
            interval: self.interval,
            floor: self.floor,
            refused: self.refused,
            unavailable: self.unavailable,
            revision: self.revision,
            epoch: self.epoch,
        }
    }

    /// Publish one already staged transition; no allocation or failure follows.
    fn apply(&mut self, update: Update) {
        self.interval = update.interval;
        self.floor = update.floor;
        self.refused = update.refused;
        self.unavailable = update.unavailable;
        self.revision = update.revision;
        self.epoch = update.epoch;
        self.changed = true;
    }

    /// Retain existing facts while closing an exhausted representation permanently.
    fn exhaust(&mut self) {
        self.unavailable = true;
        self.changed = true;
    }

    /// Current receive availability intentionally ignores the next-send floor.
    /// # Errors
    /// Refuse a sticky unavailable record or its retained direct server refusal.
    fn available(&self) -> Result<(), SourcePolicyError> {
        if self.unavailable {
            Err(SourcePolicyError::Unavailable)
        } else if self.refused {
            Err(SourcePolicyError::Disabled)
        } else {
            Ok(())
        }
    }
}

/// Copyable inert staged mutation; it grants no ownership or current permission.
#[derive(Clone, Copy)]
struct Update {
    /// Revised own interval.
    interval: Duration,
    /// Preserved or later floor.
    floor: Option<Instant>,
    /// Revised direct refusal.
    refused: bool,
    /// Sticky unavailable state.
    unavailable: bool,
    /// Checked next revision.
    revision: u64,
    /// Checked correlation epoch.
    epoch: u64,
}

/// One preallocated endpoint slot; binding is permanent once selected.
struct EndpointRecord {
    /// None only before first binding; a live key is never replaced.
    key: Option<EndpointKey>,
    /// Independent endpoint restriction.
    record: Record,
}

/// Private canonical state; only short checked transactions may mutate it.
struct PolicyState {
    /// Immutable admitted bounds, retained across all sessions.
    limits: SourcePolicyLimits,
    /// Fixed source catalogue.
    sources: Vec<Record>,
    /// Separate checked configuration revisions, never `KoD` restriction state.
    configurations: Vec<configuration::ConfigState>,
    /// Fixed endpoint storage, preallocated before root publication.
    endpoints: Vec<EndpointRecord>,
}

/// Stable record selector, never accepted without actual root identity.
#[derive(Clone, Copy, Eq, PartialEq)]
enum RecordKey {
    /// Fixed source catalogue position.
    Source(usize),
    /// Permanently bound endpoint slot.
    Endpoint(usize),
}

impl PolicyState {
    /// Read one actual existing record; unused endpoint slots are not targets.
    /// # Errors
    /// Refuse an unknown source position or an unknown/unbound endpoint position.
    fn record(&self, key: RecordKey) -> Result<&Record, SourcePolicyError> {
        match key {
            RecordKey::Source(index) => self
                .sources
                .get(index)
                .ok_or(SourcePolicyError::UnknownSource),
            RecordKey::Endpoint(index) => self
                .endpoints
                .get(index)
                .filter(|value| value.key.is_some())
                .map(|value| &value.record)
                .ok_or(SourcePolicyError::UnknownEndpoint),
        }
    }

    /// Publish only to a previously validated permanent record position.
    /// # Errors
    /// Refuse an unknown source position or an unknown/unbound endpoint position.
    fn record_mut(&mut self, key: RecordKey) -> Result<&mut Record, SourcePolicyError> {
        match key {
            RecordKey::Source(index) => self
                .sources
                .get_mut(index)
                .ok_or(SourcePolicyError::UnknownSource),
            RecordKey::Endpoint(index) => self
                .endpoints
                .get_mut(index)
                .filter(|value| value.key.is_some())
                .map(|value| &mut value.record)
                .ok_or(SourcePolicyError::UnknownEndpoint),
        }
    }

    /// Validate both actual binding positions before any transition.
    /// # Errors
    /// Refuse an unknown source, changed configuration or a canonical endpoint
    /// that differs from its permanently retained slot.
    fn binding(&self, binding: &SourceBinding) -> Result<(), SourcePolicyError> {
        self.record(RecordKey::Source(binding.source.index))?;
        self.configurations
            .get(binding.source.index)
            .ok_or(SourcePolicyError::UnknownSource)?
            .check(&binding.context, binding.key)?;
        if self
            .endpoints
            .get(binding.endpoint)
            .and_then(|value| value.key)
            != Some(binding.key)
        {
            return Err(SourcePolicyError::Foreign);
        }
        Ok(())
    }

    /// Return the maximum independently retained floor after availability checks.
    /// # Errors
    /// Refuse an invalid binding or either record's retained refusal/unavailability.
    fn earliest(&self, binding: &SourceBinding) -> Result<Option<Instant>, SourcePolicyError> {
        self.binding(binding)?;
        let source = self.record(RecordKey::Source(binding.source.index))?;
        let endpoint = self.record(RecordKey::Endpoint(binding.endpoint))?;
        source.available()?;
        endpoint.available()?;
        Ok(source.floor.max(endpoint.floor))
    }

    /// Stage both own-interval debits before invocation; failure debits neither.
    /// # Errors
    /// Refuse invalid/unavailable bindings, an unreached floor or unrepresentable
    /// deadlines/revisions. Arithmetic exhaustion marks only affected records
    /// unavailable without debiting either record's send allowance.
    fn reserve(
        &mut self,
        binding: &SourceBinding,
        request: &Arc<()>,
        now: Instant,
    ) -> Result<(Correlation, Instant), SourcePolicyError> {
        if self.earliest(binding)?.is_some_and(|floor| now < floor) {
            return Err(SourcePolicyError::NotDue);
        }
        let keys = [
            RecordKey::Source(binding.source.index),
            RecordKey::Endpoint(binding.endpoint),
        ];
        let mut plans = [None, None];
        for (key, plan) in keys.into_iter().zip(plans.iter_mut()) {
            let record = self.record(key)?;
            *plan = now.checked_add(record.interval).and_then(|floor| {
                record.revision.checked_add(1).map(|revision| {
                    let mut update = record.update();
                    update.floor = Some(record.floor.map_or(floor, |old| old.max(floor)));
                    update.revision = revision;
                    update
                })
            });
        }
        if plans.iter().any(Option::is_none) {
            for (key, plan) in keys.into_iter().zip(plans) {
                if plan.is_none() {
                    self.record_mut(key)?.exhaust();
                }
            }
            return Err(SourcePolicyError::Unavailable);
        }
        let [Some(source_update), Some(endpoint_update)] = plans else {
            return Err(SourcePolicyError::Unavailable);
        };
        let floor = source_update
            .floor
            .zip(endpoint_update.floor)
            .map(|(source, endpoint)| source.max(endpoint))
            .ok_or(SourcePolicyError::Unavailable)?;
        let correlation = Correlation {
            cell: Arc::clone(&binding.source.cell),
            source: binding.source.index,
            endpoint: binding.endpoint,
            request: Arc::clone(request),
            sent_at: now,
            source_epoch: self.record(keys[0])?.epoch,
            endpoint_epoch: self.record(keys[1])?.epoch,
        };
        for (key, plan) in keys.into_iter().zip(plans) {
            self.record_mut(key)?
                .apply(plan.ok_or(SourcePolicyError::Unavailable)?);
        }
        Ok((correlation, floor))
    }

    /// Admit this actual outstanding request without applying send-due rules.
    /// # Errors
    /// Refuse invalid/foreign binding or request identity, unavailable/refused
    /// records, or correlation epochs invalidated since the actual send.
    fn receiving(
        &self,
        binding: &SourceBinding,
        stamp: &Correlation,
        request: &Arc<()>,
    ) -> Result<(), SourcePolicyError> {
        self.binding(binding)?;
        if !Arc::ptr_eq(&binding.source.cell, &stamp.cell)
            || stamp.source != binding.source.index
            || stamp.endpoint != binding.endpoint
            || !Arc::ptr_eq(request, &stamp.request)
        {
            return Err(SourcePolicyError::Foreign);
        }
        let source = self.record(RecordKey::Source(stamp.source))?;
        let endpoint = self.record(RecordKey::Endpoint(stamp.endpoint))?;
        source.available()?;
        endpoint.available()?;
        if source.epoch != stamp.source_epoch || endpoint.epoch != stamp.endpoint_epoch {
            return Err(SourcePolicyError::Stale);
        }
        Ok(())
    }

    /// Atomically publish the complete direct A/E control, including fail-closed plans.
    /// # Errors
    /// Refuse an invalid binding or inaccessible record position. Arithmetic
    /// exhaustion instead marks the affected record unavailable while publishing
    /// the other record's valid direct restriction.
    fn control(
        &mut self,
        binding: &SourceBinding,
        kind: ControlKind,
        now: Instant,
    ) -> Result<(), SourcePolicyError> {
        self.binding(binding)?;
        let keys = [
            RecordKey::Source(binding.source.index),
            RecordKey::Endpoint(binding.endpoint),
        ];
        let mut plans = [None, None];
        for (key, plan) in keys.into_iter().zip(plans.iter_mut()) {
            let record = self.record(key)?;
            let mut update = record.update();
            *plan = record.revision.checked_add(1).and_then(|revision| {
                update.revision = revision;
                match kind {
                    ControlKind::Rate => {
                        let remaining =
                            self.limits.maximum_interval.checked_sub(record.interval)?;
                        update.interval = if record.interval >= remaining {
                            self.limits.maximum_interval
                        } else {
                            record.interval.checked_add(record.interval)?
                        };
                        let floor = now.checked_add(update.interval)?;
                        update.floor = Some(record.floor.map_or(floor, |old| old.max(floor)));
                    }
                    ControlKind::Deny | ControlKind::Restrict => {
                        update.refused = true;
                        update.epoch = record.epoch.checked_add(1)?;
                    }
                }
                Some(update)
            });
        }
        // Resolve both destinations before the first mutation. Source and endpoint
        // storage are separate collections, so both checked borrows coexist.
        let source: &mut Record = self
            .sources
            .get_mut(binding.source.index)
            .ok_or(SourcePolicyError::UnknownSource)?;
        let endpoint: &mut Record = self
            .endpoints
            .get_mut(binding.endpoint)
            .filter(|record| record.key.is_some())
            .map(|record| &mut record.record)
            .ok_or(SourcePolicyError::UnknownEndpoint)?;
        for (record, plan) in [source, endpoint].into_iter().zip(plans) {
            if let Some(update) = plan {
                record.apply(update);
            } else {
                record.exhaust();
            }
        }
        Ok(())
    }
}

/// One retained cell and a separate release wake for bounded lock contention.
struct PolicyCell {
    /// Immutable catalogue identity, separately retained without mutable state.
    identity_domain: Arc<()>,
    /// All state/allowance commits serialize here; never block waiting for it.
    state: Mutex<PolicyState>,
    /// Read-only release is distinct from an actual policy change.
    released: Arc<Notify>,
    /// One fixed test pulse at the real setup-contention wait, never a callback.
    #[cfg(test)]
    setup_contention_probe: Mutex<Option<std::sync::mpsc::SyncSender<()>>>,
}

impl PolicyCell {
    /// Allocate every source/endpoint record and channel before publication.
    /// # Errors
    /// Refuse bounded source or endpoint vector reservation without publishing a root.
    fn new(limits: SourcePolicyLimits) -> Result<Arc<Self>, SourcePolicyError> {
        let identity_domain = Arc::new(());
        let mut sources = Vec::new();
        sources
            .try_reserve_exact(limits.source_count)
            .map_err(|_| SourcePolicyError::Allocation)?;
        let mut configurations = Vec::new();
        configurations
            .try_reserve_exact(limits.source_count)
            .map_err(|_| SourcePolicyError::Allocation)?;
        let mut endpoints = Vec::new();
        endpoints
            .try_reserve_exact(limits.endpoint_capacity)
            .map_err(|_| SourcePolicyError::Allocation)?;
        for index in 0..limits.source_count {
            sources.push(Record::new(limits.initial_interval));
            configurations.push(configuration::ConfigState::new(SourceIdentity {
                domain: Arc::clone(&identity_domain),
                index,
            }));
        }
        for _ in 0..limits.endpoint_capacity {
            endpoints.push(EndpointRecord {
                key: None,
                record: Record::new(limits.initial_interval),
            });
        }
        Ok(Arc::new(Self {
            identity_domain,
            state: Mutex::new(PolicyState {
                limits,
                sources,
                configurations,
                endpoints,
            }),
            released: Arc::new(Notify::new()),
            #[cfg(test)]
            setup_contention_probe: Mutex::new(None),
        }))
    }

    /// Acquire without waiting; no failed attempt grants permission or resets state.
    /// # Errors
    /// Refuse current contention or poisoning by a panicked policy transaction.
    fn lock(&self) -> Result<PolicyGuard<'_>, SourcePolicyError> {
        let guard = self.state.try_lock().map_err(|error| match error {
            TryLockError::WouldBlock => SourcePolicyError::Contended,
            TryLockError::Poisoned(_) => SourcePolicyError::Poisoned,
        })?;
        Ok(PolicyGuard {
            guard: Some(guard),
            cell: self,
        })
    }
}

/// Short transaction; release notifications occur only after actual mutex release.
struct PolicyGuard<'a> {
    /// Some throughout the transaction, taken exactly once during Drop.
    guard: Option<MutexGuard<'a, PolicyState>>,
    /// Actual cell supplying release and change notification owners.
    cell: &'a PolicyCell,
}

impl PolicyGuard<'_> {
    /// Borrow the actual guarded state, never a detached mutation snapshot.
    /// # Errors
    /// Refuse unavailable custody after this guard has relinquished its mutex guard.
    fn state(&self) -> Result<&PolicyState, SourcePolicyError> {
        self.guard.as_deref().ok_or(SourcePolicyError::Unavailable)
    }

    /// Mutably borrow only the actual guarded state.
    /// # Errors
    /// Refuse unavailable custody after this guard has relinquished its mutex guard.
    fn state_mut(&mut self) -> Result<&mut PolicyState, SourcePolicyError> {
        self.guard
            .as_deref_mut()
            .ok_or(SourcePolicyError::Unavailable)
    }
}

impl Drop for PolicyGuard<'_> {
    /// Publish bounded actual changes separately from every lock-release wake.
    fn drop(&mut self) {
        let mut notifications: [Option<watch::Sender<()>>; MAX_RECORDS] =
            std::array::from_fn(|_| None);
        if let Some(state) = self.guard.as_deref_mut() {
            for (index, record) in state
                .sources
                .iter_mut()
                .chain(
                    state
                        .endpoints
                        .iter_mut()
                        .map(|endpoint| &mut endpoint.record),
                )
                .enumerate()
            {
                if record.changed {
                    if let Some(destination) = notifications.get_mut(index) {
                        *destination = Some(record.changes.clone());
                    }
                    record.changed = false;
                }
            }
        }
        drop(self.guard.take());
        for channel in notifications.into_iter().flatten() {
            channel.send_replace(());
        }
        self.cell.released.notify_waiters();
    }
}

/// Non-clonable laboratory policy authority; it is not production administration.
/// The administrative facade cannot be duplicated through a convenience trait:
/// ```compile_fail,E0277
/// use nevrotick_engine::SourcePolicyAuthority;
/// fn requires_clone<T: Clone>() {}
/// requires_clone::<SourcePolicyAuthority>();
/// ```
pub struct SourcePolicyAuthority {
    /// Actual once-installed process cell, not a caller identity label.
    cell: Arc<PolicyCell>,
}

impl SourcePolicyAuthority {
    /// Install exactly one successful process root; dropping this facade cannot reset it.
    /// # Errors
    /// Preserve prior installation or allocation failure; publish no partial root.
    pub fn install(limits: SourcePolicyLimits) -> Result<Self, SourcePolicyError> {
        if PROCESS_POLICY.get().is_some() {
            return Err(SourcePolicyError::AlreadyInstalled);
        }
        let cell = PolicyCell::new(limits)?;
        PROCESS_POLICY
            .set(Arc::clone(&cell))
            .map_err(|_| SourcePolicyError::AlreadyInstalled)?;
        Ok(Self { cell })
    }

    /// Retrieve one existing opaque catalogue handle without minting a new source.
    /// # Errors
    /// Refuse an unknown catalogue position or unavailable policy custody.
    pub fn source(&self, index: usize) -> Result<SourceHandle, SourcePolicyError> {
        let guard = self.cell.lock()?;
        guard.state()?.record(RecordKey::Source(index))?;
        Ok(SourceHandle {
            cell: Arc::clone(&self.cell),
            index,
        })
    }

    /// Install the first explicit numeric definition for this actual source.
    /// This changes no RATE/refusal, floor, `KoD` revision, endpoint or request epoch.
    /// # Errors
    /// Reject a foreign handle, unavailable custody, an existing configuration
    /// or exhausted configuration identity without publishing a replacement.
    pub fn install_source_configuration(
        &mut self,
        source: &SourceHandle,
        addresses: SourceAddressSet,
    ) -> Result<SourceConfigurationSnapshot, SourcePolicyError> {
        configuration::install(self, source, addresses)
    }

    /// Atomically replace exactly the source identified by an actual current snapshot.
    /// A successful replacement issues a fresh configuration revision, including
    /// for equal address values. Existing `KoD` restrictions and targets remain intact.
    /// # Errors
    /// Reject foreign/stale input unchanged. Revision exhaustion withdraws
    /// configuration and wakes the source without resetting its restrictions.
    pub fn replace_source_configuration(
        &mut self,
        expected: &SourceConfigurationSnapshot,
        addresses: SourceAddressSet,
    ) -> Result<SourceConfigurationSnapshot, SourcePolicyError> {
        configuration::replace(self, expected, addresses)
    }

    /// Replace the declared time scale of the actual current source revision.
    /// The addresses, KE target and restrictions retain their existing values.
    /// A fresh revision fences pending work. Completed facts retain their original
    /// declaration. Installing or replacing endpoints or a KE target initializes
    /// the new definition to [`SourceScale::Unknown`].
    /// # Errors
    /// Preserve stale and foreign refusal, unavailable custody and sticky revision
    /// exhaustion through the same configuration publication path.
    pub fn replace_source_scale(
        &mut self,
        expected: &SourceConfigurationSnapshot,
        scale: SourceScale,
    ) -> Result<SourceConfigurationSnapshot, SourcePolicyError> {
        configuration::replace_scale(self, expected, scale)
    }

    /// Install checked KE and UDP inputs together for this actual catalogue source.
    /// The existing checked target supplies syntax, bounds and verification-name
    /// invariants; installation supplies source/revision identity, not TLS success.
    /// Existing restrictions, floors and endpoint records remain unchanged.
    /// # Errors
    /// Preserve foreign/unknown source, existing installation, sticky exhaustion
    /// and nonblocking policy-custody failures without partial publication.
    #[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
    pub fn install_ke_preview_configuration(
        &mut self,
        source: &SourceHandle,
        addresses: SourceAddressSet,
        target: crate::KePreviewTarget,
    ) -> Result<SourceConfigurationSnapshot, SourcePolicyError> {
        configuration::install_preview(self, source, addresses, target)
    }

    /// Replace exactly the actual current source definition with checked KE/UDP inputs.
    /// Equal input values still receive a fresh revision. This grants neither
    /// protected-datagram permission nor a reset of retained source/endpoint policy.
    /// # Errors
    /// Refuse foreign/stale input unchanged; checked revision exhaustion withdraws
    /// configuration while retaining every applicable restriction and floor.
    #[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
    pub fn replace_ke_preview_configuration(
        &mut self,
        expected: &SourceConfigurationSnapshot,
        addresses: SourceAddressSet,
        target: crate::KePreviewTarget,
    ) -> Result<SourceConfigurationSnapshot, SourcePolicyError> {
        configuration::replace_preview(self, expected, addresses, target)
    }

    /// Issue one explicit protected invocation for this authority's current revision.
    /// # Errors
    /// Refuse a different root or obsolete, missing or restricted KE definition.
    #[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
    pub(super) fn authorize_protected_ke(
        &self,
        expected: &SourceConfigurationSnapshot,
    ) -> Result<ProtectedKeSource, SourcePolicyError> {
        let identity: &SourceIdentity = expected.source_identity();
        if !Arc::ptr_eq(&self.cell.identity_domain, &identity.domain) {
            return Err(SourcePolicyError::Foreign);
        }
        let source: SourceHandle = SourceHandle {
            cell: Arc::clone(&self.cell),
            index: identity.index,
        };
        let definition: ConfiguredKePreview = configuration::capture_preview(&source)?;
        if !definition.snapshot().same_configuration(expected) {
            return Err(SourcePolicyError::ConfigurationChanged);
        }
        Ok(ProtectedKeSource { definition })
    }

    /// Issue a current source-revision target from this actual authority only.
    /// # Errors
    /// Refuse foreign source identity or unavailable custody.
    pub fn source_target(
        &self,
        source: &SourceHandle,
    ) -> Result<SourcePolicyTarget, SourcePolicyError> {
        if !Arc::ptr_eq(&self.cell, &source.cell) {
            return Err(SourcePolicyError::Foreign);
        }
        self.target(RecordKey::Source(source.index))
    }

    /// Issue a target for an already bound canonical endpoint without creating it.
    /// # Errors
    /// Refuse an invalid/unknown endpoint or unavailable custody.
    pub fn endpoint_target(
        &self,
        peer: SocketAddr,
    ) -> Result<SourcePolicyTarget, SourcePolicyError> {
        let key = EndpointKey::new(peer)?;
        let guard = self.cell.lock()?;
        let index = guard
            .state()?
            .endpoints
            .iter()
            .position(|endpoint| endpoint.key == Some(key))
            .ok_or(SourcePolicyError::UnknownEndpoint)?;
        let revision = guard.state()?.record(RecordKey::Endpoint(index))?.revision;
        Ok(SourcePolicyTarget {
            cell: Arc::clone(&self.cell),
            key: RecordKey::Endpoint(index),
            revision,
        })
    }

    /// Capture an existing record's revision under actual custody.
    /// # Errors
    /// Refuse policy contention/poisoning, unavailable guard custody or an unknown
    /// source/endpoint record; no target is issued on refusal.
    fn target(&self, key: RecordKey) -> Result<SourcePolicyTarget, SourcePolicyError> {
        let guard = self.cell.lock()?;
        let revision = guard.state()?.record(key)?.revision;
        Ok(SourcePolicyTarget {
            cell: Arc::clone(&self.cell),
            key,
            revision,
        })
    }

    /// Atomically clear exactly selected current refusals, preserving intervals/floors.
    /// # Errors
    /// Reject the complete selection before mutation on foreign, duplicate, stale,
    /// unavailable or unrepresentable targets. No earlier epoch becomes current.
    pub fn reenable(&mut self, targets: &[SourcePolicyTarget]) -> Result<(), SourcePolicyError> {
        if targets.is_empty() || targets.len() > MAX_RECORDS {
            return Err(SourcePolicyError::InvalidSelection);
        }
        let mut guard = self.cell.lock()?;
        let state = guard.state_mut()?;
        let mut plans = [None; MAX_RECORDS];
        for (index, target) in targets.iter().enumerate() {
            if !Arc::ptr_eq(&self.cell, &target.cell) {
                return Err(SourcePolicyError::Foreign);
            }
            if targets.iter().take(index).any(|old| old.key == target.key) {
                return Err(SourcePolicyError::Duplicate);
            }
            let record = state.record(target.key)?;
            if record.unavailable {
                return Err(SourcePolicyError::Unavailable);
            }
            if record.revision != target.revision {
                return Err(SourcePolicyError::Stale);
            }
            let mut update = record.update();
            update.revision = record
                .revision
                .checked_add(1)
                .ok_or(SourcePolicyError::Unavailable)?;
            update.epoch = record
                .epoch
                .checked_add(1)
                .ok_or(SourcePolicyError::Unavailable)?;
            update.refused = false;
            *plans
                .get_mut(index)
                .ok_or(SourcePolicyError::InvalidSelection)? = Some((target.key, update));
        }
        for (key, update) in plans.into_iter().flatten() {
            state.record_mut(key)?.apply(update);
        }
        Ok(())
    }
}

impl fmt::Debug for SourcePolicyAuthority {
    /// Redact root identity and policy contents.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SourcePolicyAuthority { .. }")
    }
}

/// Authority-issued exact current-revision selection; it grants no authority by itself.
#[derive(Clone)]
pub struct SourcePolicyTarget {
    /// Actual issuer cell, not an integer/Boolean authorization claim.
    cell: Arc<PolicyCell>,
    /// Exactly one existing source or endpoint record.
    key: RecordKey,
    /// Revision which must still be current at the atomic operation.
    revision: u64,
}

impl fmt::Debug for SourcePolicyTarget {
    /// Redact identity and revision.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SourcePolicyTarget { .. }")
    }
}

/// Cloneable use handle for one already issued stable logical source.
#[derive(Clone)]
/// A caller cannot manufacture a different source from an existing handle:
/// ```compile_fail,E0451
/// use nevrotick_engine::SourceHandle;
/// fn forge(existing: SourceHandle) -> SourceHandle { SourceHandle { ..existing } }
/// ```
/// Nor can it change the registered source position:
/// ```compile_fail,E0616
/// use nevrotick_engine::SourceHandle;
/// fn relabel(mut source: SourceHandle) { source.index = 1; }
/// ```
/// The legal path retains the issued source and checked session constructor:
/// ```
/// use nevrotick_engine::{AcquisitionSession, SeriesConfig, SourceHandle, SourcePolicyError};
/// fn session(source: &SourceHandle, config: SeriesConfig)
///     -> Result<AcquisitionSession, SourcePolicyError> {
///     source.session(config)
/// }
/// ```
pub struct SourceHandle {
    /// Retains exactly the same actual cell across every session replacement.
    cell: Arc<PolicyCell>,
    /// Existing immutable catalogue position, never a session-derived identity.
    index: usize,
}

impl SourceHandle {
    /// Identify this fixed catalogue source without reading mutable policy.
    ///
    /// The original authority admits the immutable catalogue position. It is
    /// never recycled within this domain, including after session replacement,
    /// endpoint changes or re-enable. The returned value retains no policy cell
    /// and proves neither an exchange occurred nor any current permission.
    pub fn identity(&self) -> SourceIdentity {
        SourceIdentity {
            domain: Arc::clone(&self.cell.identity_domain),
            index: self.index,
        }
    }

    /// Read actual installed configuration history without creating endpoint state.
    /// None means genuinely unconfigured; it is not a current capability.
    /// # Errors
    /// Preserve contention, poisoning and sticky configuration exhaustion rather
    /// than manufacturing an unconfigured fallback.
    pub fn configuration_snapshot(
        &self,
    ) -> Result<Option<SourceConfigurationSnapshot>, SourcePolicyError> {
        let guard = self.cell.lock()?;
        let context = SourceContext::capture(self, &guard, &[None; 3])?;
        Ok(context.configuration().cloned())
    }

    /// Capture the real installed Preview definition and its current source owner.
    /// The result retains original immutable inputs, not permission across a later
    /// replacement or restriction. Each receiving effect must recheck actual custody.
    /// # Errors
    /// Refuse missing/numeric-only, changed or exhausted configuration, a disabled
    /// source and unavailable policy custody without creating endpoint state.
    #[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
    pub(super) fn capture_ke_preview(&self) -> Result<ConfiguredKePreview, SourcePolicyError> {
        configuration::capture_preview(self)
    }

    /// Observe this source and one already-bound canonical endpoint atomically.
    ///
    /// Refusal and unavailability remain visible data. This bounded read creates
    /// no endpoint/channel, publishes no record change and changes no allowance.
    /// Existing contention-release notifications remain part of guard cleanup.
    /// It never grants current send permission or issues a re-enable target.
    /// # Errors
    /// Preserve invalid/unknown endpoint, unknown source, contention or poisoning
    /// as typed errors without binding an endpoint or returning a clear default.
    pub fn policy_snapshot(
        &self,
        peer: SocketAddr,
    ) -> Result<SourcePolicySnapshot, SourcePolicyError> {
        let key = EndpointKey::new(peer)?;
        let guard = self.cell.lock()?;
        let state = guard.state()?;
        let endpoint = state
            .endpoints
            .iter()
            .position(|entry| entry.key == Some(key))
            .ok_or(SourcePolicyError::UnknownEndpoint)?;
        SourcePolicySnapshot::capture(&guard, self.index, endpoint)
    }

    /// Bind a complete fixed round without partially publishing endpoint keys.
    /// # Errors
    /// Reject invalid count/keys, unavailable custody or insufficient permanent
    /// capacity before changing any record. Canonical aliases share one slot.
    pub(super) fn bind_round(
        &self,
        peers: &[SocketAddr],
    ) -> Result<RoundBindings, SourcePolicyError> {
        if peers.is_empty() || peers.len() > 3 {
            return Err(SourcePolicyError::InvalidSelection);
        }
        let mut keys = [None; 3];
        for (key, peer) in keys.iter_mut().zip(peers) {
            *key = Some(EndpointKey::new(*peer)?);
        }
        let mut guard = self.cell.lock()?;
        let context = SourceContext::capture(self, &guard, &keys)?;
        let state = guard.state_mut()?;
        let source_changes = state.record(RecordKey::Source(self.index))?.changes.clone();
        let mut positions = [None; 3];
        let mut publications = [None; 3];
        for (position, key) in keys.iter().copied().enumerate().take(peers.len()) {
            let key = key.ok_or(SourcePolicyError::InvalidEndpoint)?;
            let existing = state
                .endpoints
                .iter()
                .position(|entry| entry.key == Some(key));
            let planned = publications
                .iter()
                .flatten()
                .find_map(|(index, value)| (*value == key).then_some(*index));
            let index = if let Some(index) = existing.or(planned) {
                index
            } else {
                let index = state
                    .endpoints
                    .iter()
                    .enumerate()
                    .find_map(|(index, entry)| {
                        (entry.key.is_none()
                            && !publications
                                .iter()
                                .flatten()
                                .any(|(used, _)| *used == index))
                        .then_some(index)
                    })
                    .ok_or(SourcePolicyError::Capacity)?;
                publications[position] = Some((index, key));
                index
            };
            positions[position] = Some(index);
        }
        let mut bindings = std::array::from_fn(|_| None);
        for (position, binding) in bindings.iter_mut().enumerate().take(peers.len()) {
            let endpoint = positions[position].ok_or(SourcePolicyError::Capacity)?;
            *binding = Some(SourceBinding {
                source: self.clone(),
                context: context.clone(),
                endpoint,
                key: keys[position].ok_or(SourcePolicyError::InvalidEndpoint)?,
                source_changes: source_changes.clone(),
                endpoint_changes: state
                    .endpoints
                    .get(endpoint)
                    .ok_or(SourcePolicyError::Capacity)?
                    .record
                    .changes
                    .clone(),
            });
        }
        // All fallible work and complete result construction precede publication.
        for (index, endpoint) in state.endpoints.iter_mut().enumerate() {
            if let Some((_, key)) = publications
                .iter()
                .flatten()
                .find(|(slot, _)| *slot == index)
            {
                endpoint.key = Some(*key);
            }
        }
        Ok(RoundBindings { bindings })
    }

    /// Create a managed session from the original checked one-peer configuration.
    /// # Errors
    /// Preserve endpoint/capacity/custody refusal before preparation or socket work.
    pub fn session(&self, config: SeriesConfig) -> Result<AcquisitionSession, SourcePolicyError> {
        let binding = self.bind(config.peer())?;
        Ok(AcquisitionSession::new_managed(config, binding))
    }

    /// Bind one permanent endpoint slot, reusing every canonical alias's record.
    /// # Errors
    /// Refuse an invalid numeric endpoint, unavailable policy custody, unknown
    /// source or exhausted permanent endpoint capacity.
    pub(super) fn bind(&self, peer: SocketAddr) -> Result<SourceBinding, SourcePolicyError> {
        let key = EndpointKey::new(peer)?;
        let mut guard = self.cell.lock()?;
        let context = SourceContext::capture(self, &guard, &[Some(key), None, None])?;
        let state = guard.state_mut()?;
        let source_changes = state.record(RecordKey::Source(self.index))?.changes.clone();
        let endpoint = if let Some(index) = state
            .endpoints
            .iter()
            .position(|value| value.key == Some(key))
        {
            index
        } else {
            let index = state
                .endpoints
                .iter()
                .position(|value| value.key.is_none())
                .ok_or(SourcePolicyError::Capacity)?;
            state
                .endpoints
                .get_mut(index)
                .ok_or(SourcePolicyError::Capacity)?
                .key = Some(key);
            index
        };
        let endpoint_changes = state.record(RecordKey::Endpoint(endpoint))?.changes.clone();
        Ok(SourceBinding {
            source: self.clone(),
            context,
            endpoint,
            key,
            source_changes,
            endpoint_changes,
        })
    }
}

/// Complete bounded endpoint custody issued atomically by one actual source.
#[derive(Clone)]
pub(super) struct RoundBindings {
    /// Occupied entries retain their genuine source/root and canonical endpoint.
    bindings: [Option<SourceBinding>; 3],
}

impl RoundBindings {
    /// Derive the actual candidate's mode; absent/foreign entries never downgrade.
    /// # Errors
    /// Reject a destination outside this immutable issued batch.
    pub(super) fn mode(&self, peer: SocketAddr) -> Result<SourceMode, SourcePolicyError> {
        let key = EndpointKey::new(peer)?;
        self.bindings
            .iter()
            .flatten()
            .find(|binding| binding.key == key)
            .cloned()
            .map(SourceMode::Managed)
            .ok_or(SourcePolicyError::Foreign)
    }
}

impl fmt::Debug for SourceHandle {
    /// Redact stable source/root identity.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SourceHandle { .. }")
    }
}

/// Immutable identity of one actual managed catalogue source.
///
/// Same endpoints do not imply the same source. A source keeps this identity
/// across sessions and endpoints; separate installed catalogues never share it.
/// Only the real catalogue handle can issue a token. Clones retain a separate
/// identity allocation and immutable position, without mutable policy, channels
/// or native resources. Retained identity is historical provenance, not source
/// independence, authentication, configuration revision or current permission.
///
/// Raw allocation labels cannot manufacture an accepted identity:
/// ```compile_fail,E0451
/// use nevrotick_engine::SourceIdentity;
/// use std::sync::Arc;
/// let forged = SourceIdentity { domain: Arc::new(()), index: 0 };
/// ```
/// An issued identity cannot be retargeted:
/// ```compile_fail,E0616
/// use nevrotick_engine::SourceIdentity;
/// fn retarget(mut identity: SourceIdentity) { identity.index = 1; }
/// ```
/// No default source exists:
/// ```compile_fail,E0599
/// use nevrotick_engine::SourceIdentity;
/// let identity = SourceIdentity::default();
/// ```
/// An arbitrary catalogue number supplies no identity:
/// ```compile_fail,E0277
/// use nevrotick_engine::SourceIdentity;
/// let identity: SourceIdentity = 0_usize.into();
/// ```
/// A real consumer compares returned provenance with its actual source handle:
/// ```
/// use nevrotick_engine::{SourceHandle, TransportFacts};
/// fn belongs_to_source(source: &SourceHandle, facts: &TransportFacts) -> bool {
///     facts.source_identity().is_some_and(|identity|
///         identity.same_source(&source.identity()))
/// }
/// ```
#[derive(Clone)]
#[must_use = "retain and compare source provenance explicitly"]
pub struct SourceIdentity {
    /// Separately retained original catalogue allocation, never a numeric label.
    domain: Arc<()>,
    /// Original admitted position; fixed catalogues never remove/recycle sources.
    index: usize,
}

impl SourceIdentity {
    /// Compare actual logical-source identity, without reading policy or time.
    /// Equal identities establish neither independent votes nor current validity.
    #[must_use]
    pub fn same_source(&self, other: &Self) -> bool {
        let Self { domain, index } = self;
        let Self {
            domain: other_domain,
            index: other_index,
        } = other;
        Arc::ptr_eq(domain, other_domain) && index == other_index
    }
}

impl fmt::Debug for SourceIdentity {
    /// Retain no catalogue position, allocation address or correlatable label.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("SourceIdentity { .. }")
    }
}

/// Original KE inputs issued only from an actual installed catalogue configuration.
/// This live invocation context retains its source owner; the returned historical
/// snapshot alone retains no mutable policy, native resource or current permission.
#[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
#[must_use = "a captured Preview definition must be consumed through current owner checks"]
pub(super) struct ConfiguredKePreview {
    /// Actual owner whose current state every admission must inspect.
    source: SourceHandle,
    /// Original jointly installed source/revision/UDP definition.
    snapshot: SourceConfigurationSnapshot,
    /// Cheap immutable clone of the original snapshot's issued target.
    target: crate::KePreviewTarget,
}

#[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
impl ConfiguredKePreview {
    /// Borrow the actual originally installed checked target without reinterpreting it.
    #[must_use]
    pub(super) fn target(&self) -> &crate::KePreviewTarget {
        &self.target
    }

    /// Borrow original configuration history, without reading or granting live policy.
    #[must_use]
    pub(super) const fn snapshot(&self) -> &SourceConfigurationSnapshot {
        &self.snapshot
    }

    /// Acquire actual nonblocking policy custody before performing any checks.
    /// The receiving owner must retain this guard through its decision and release
    /// Control custody before this value drops and publishes policy-release wakes.
    /// # Errors
    /// Preserve contention/poisoning; no missing or stale definition becomes a
    /// successful snapshot check merely by obtaining the lock.
    pub(super) fn lock(&self) -> Result<KePreviewPolicyGuard<'_>, SourcePolicyError> {
        let guard: PolicyGuard<'_> = self.source.cell.lock()?;
        Ok(KePreviewPolicyGuard {
            guard,
            original: self,
        })
    }
}

/// Actual policy custody tied to the original issued KE definition.
/// Its Drop may notify waiters after releasing policy; combined owners must drop
/// their Control gates first on success, refusal and unwind alike.
#[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
#[must_use = "retain policy custody through the actual serialized Preview decision"]
pub(super) struct KePreviewPolicyGuard<'a> {
    /// The actual same-cell mutex owner, not a currentness Boolean.
    guard: PolicyGuard<'a>,
    /// The same originally captured source and complete installed definition.
    original: &'a ConfiguredKePreview,
}

#[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
impl KePreviewPolicyGuard<'_> {
    /// Check current definition and availability while retaining actual custody.
    /// None checks the source before a returned UDP endpoint is known; Some also
    /// checks that endpoint's original membership and any existing restriction.
    /// No endpoint slot, channel, polling-floor debit or permission is created.
    /// # Errors
    /// Preserve foreign/stale/unavailable definition, source/endpoint refusal,
    /// and invalid/outside selected endpoint without allocation or state change.
    pub(super) fn check(&self, peer: Option<SocketAddr>) -> Result<(), SourcePolicyError> {
        configuration::check_preview(self.original, &self.guard, peer)
    }
}

/// Explicit single-invocation authorization from the actual catalogue authority.
#[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
pub(super) struct ProtectedKeSource {
    /// Reuse the checked immutable inputs shared with the no-send branch.
    definition: ConfiguredKePreview,
}

#[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
impl ProtectedKeSource {
    /// Borrow the originally issued definition without granting an endpoint.
    pub(super) const fn definition(&self) -> &ConfiguredKePreview {
        &self.definition
    }

    /// Stage the exact negotiated destination under the already held installation guard.
    /// # Errors
    /// Refuse foreign/stale configuration, a restricted/disallowed peer or full endpoint custody.
    pub(super) fn prepare_endpoint<'a>(
        &self,
        policy: &'a mut KePreviewPolicyGuard<'_>,
        peer: SocketAddr,
    ) -> Result<PreparedProtectedEndpoint<'a>, SourcePolicyError> {
        if !std::ptr::eq(policy.original, &raw const self.definition) {
            return Err(SourcePolicyError::Foreign);
        }
        policy.check(Some(peer))?;
        let source: &SourceHandle = &self.definition.source;
        let key: EndpointKey = EndpointKey::new(peer)?;
        let context: SourceContext =
            SourceContext::capture(source, &policy.guard, &[Some(key), None, None])?;
        let state: &mut PolicyState = policy.guard.state_mut()?;
        let source_changes: watch::Sender<()> = state
            .record(RecordKey::Source(source.index))?
            .changes
            .clone();
        let endpoint: usize = state
            .endpoints
            .iter()
            .position(|entry: &EndpointRecord| entry.key == Some(key))
            .or_else(|| {
                state
                    .endpoints
                    .iter()
                    .position(|entry: &EndpointRecord| entry.key.is_none())
            })
            .ok_or(SourcePolicyError::Capacity)?;
        let record: &mut EndpointRecord = state
            .endpoints
            .get_mut(endpoint)
            .ok_or(SourcePolicyError::Capacity)?;
        let binding: SourceBinding = SourceBinding {
            source: source.clone(),
            context,
            endpoint,
            key,
            source_changes,
            endpoint_changes: record.record.changes.clone(),
        };
        Ok(PreparedProtectedEndpoint {
            slot: &mut record.key,
            endpoint: ProtectedEndpoint { binding, peer },
        })
    }
}

/// A still-unpublished exact endpoint, coupled to its guarded canonical slot.
#[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
pub(super) struct PreparedProtectedEndpoint<'a> {
    /// Its exact canonical slot remains borrowed from current policy custody.
    slot: &'a mut Option<EndpointKey>,
    /// Complete same-source outcome prepared without publishing any state.
    endpoint: ProtectedEndpoint,
}

#[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
impl PreparedProtectedEndpoint<'_> {
    /// Publish infallibly after the original KE Control admits the same association.
    pub(super) fn publish(self) -> ProtectedEndpoint {
        *self.slot = Some(self.endpoint.binding.key);
        self.endpoint
    }
}

/// Exact negotiated UDP destination and independently canonical allowance identity.
#[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
#[derive(Clone)]
pub(super) struct ProtectedEndpoint {
    /// Retained canonical allowance owner and original source revision.
    binding: SourceBinding,
    /// Exact negotiated socket address, also required by wire correlation.
    peer: SocketAddr,
}

#[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
impl ProtectedEndpoint {
    /// Preserve the original authority-owned definition in committed history.
    pub(super) const fn configuration(&self) -> Option<&SourceConfigurationSnapshot> {
        self.binding.context.configuration()
    }

    /// Observe pre-send eligibility through the same source and endpoint subscriptions.
    /// This pre-send observer owns no Pending stamp and grants no send authority.
    pub(super) fn observe_send_eligibility(&self) -> LivePolicy {
        LivePolicy::from_mode(SourceMode::Managed(self.binding.clone()))
    }

    /// Observe changes affecting the genuine pending send without applying its next-send floor.
    /// The observer can recheck correlation but cannot commit a reply or control.
    pub(super) fn observe_pending(&self, stamp: &ProtectedSendStamp) -> LivePolicy {
        LivePolicy::managed(self.binding.clone(), Some(stamp.correlation.clone()))
    }

    /// Acquire the actual endpoint's policy custody without registering a waiter.
    /// # Errors
    /// Preserve contention and poison without granting another route.
    pub(super) fn lock(&self) -> Result<ProtectedEndpointGuard<'_>, SourcePolicyError> {
        let guard: PolicyGuard<'_> = self.binding.source.cell.lock()?;
        Ok(ProtectedEndpointGuard {
            guard,
            original: self,
        })
    }
}

/// Policy custody for the actual protected transaction. Release Control before this guard.
#[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
pub(super) struct ProtectedEndpointGuard<'a> {
    /// Actual nonblocking policy custody used for every protected effect check.
    guard: PolicyGuard<'a>,
    /// The original exact negotiated endpoint and source revision.
    original: &'a ProtectedEndpoint,
}

#[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
impl ProtectedEndpointGuard<'_> {
    /// Check the actual destination, configuration and both records.
    /// # Errors
    /// Refuse a different wire destination, obsolete lease or restricted record.
    pub(super) fn check(&self, peer: SocketAddr) -> Result<(), SourcePolicyError> {
        if peer != self.original.peer {
            return Err(SourcePolicyError::Foreign);
        }
        self.guard.state()?.earliest(&self.original.binding)?;
        Ok(())
    }

    /// Charge both source and shared endpoint immediately before actual invocation.
    /// # Errors
    /// Refuse an invalid lease, early observation, or unavailable policy without a send.
    pub(super) fn reserve(
        &mut self,
        peer: SocketAddr,
        request: &Arc<()>,
        observed: Instant,
    ) -> Result<ProtectedSendStamp, SourcePolicyError> {
        if peer != self.original.peer {
            return Err(SourcePolicyError::Foreign);
        }
        let (correlation, _): (Correlation, Instant) =
            self.guard
                .state_mut()?
                .reserve(&self.original.binding, request, observed)?;
        Ok(ProtectedSendStamp { correlation })
    }

    /// Match the actual sent request without applying its already advanced send floors.
    /// # Errors
    /// Refuse a foreign request, changed configuration, refusal or changed restriction epoch.
    pub(super) fn receiving(
        &self,
        stamp: &ProtectedSendStamp,
        request: &Arc<()>,
    ) -> Result<(), SourcePolicyError> {
        self.guard
            .state()?
            .receiving(&self.original.binding, &stamp.correlation, request)
    }

    /// Apply the verified control to the exact managed source and endpoint.
    /// The retired receiver supplies a consume-once authorization under the live Control.
    /// # Errors
    /// Reject reused/foreign request identities, invalidated policy or missing
    /// state before publishing either direct restriction record.
    #[cfg(test)]
    pub(super) fn apply_control(
        &mut self,
        authorization: crate::nts_ke_experiment::VerifiedRetiredControl<'_>,
    ) -> Result<(), SourcePolicyError> {
        use crate::nts_ke_experiment::ProtectedControlKind;

        let now: Instant = authorization.decided_at();
        let kind: ControlKind = match authorization.kind() {
            ProtectedControlKind::Rate => ControlKind::Rate,
            ProtectedControlKind::Deny => ControlKind::Deny,
            ProtectedControlKind::Restrict => ControlKind::Restrict,
        };
        let [prior, pending]: [(&ProtectedSendStamp, &Arc<()>); 2] =
            authorization.into_submissions();
        if Arc::ptr_eq(prior.1, pending.1) {
            return Err(SourcePolicyError::Foreign);
        }
        let state: &mut PolicyState = self.guard.state_mut()?;
        state.receiving(&self.original.binding, &prior.0.correlation, prior.1)?;
        state.receiving(&self.original.binding, &pending.0.correlation, pending.1)?;
        state.control(&self.original.binding, kind, now)
    }
}

/// Provenance of one attempted protected send, usable only with the live original owner.
#[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
pub(super) struct ProtectedSendStamp {
    /// Same-cell source/endpoint epochs, actual request allocation and send observation.
    correlation: Correlation,
}

/// Mandatory immutable source/endpoint custody passed down the prepared chain.
#[derive(Clone)]
pub(super) struct SourceBinding {
    /// Stable source identity retained independently of this endpoint.
    source: SourceHandle,
    /// Actual optional installed configuration captured with the same policy guard.
    context: SourceContext,
    /// Permanently bound endpoint position in the same actual cell.
    endpoint: usize,
    /// Canonical key checked against each concrete native destination.
    key: EndpointKey,
    /// Existing source change channel, shared without copying restriction values.
    source_changes: watch::Sender<()>,
    /// Existing endpoint change channel.
    endpoint_changes: watch::Sender<()>,
}

impl SourceBinding {
    /// Observe only this genuine issued source/endpoint binding under one lock.
    /// # Errors
    /// Preserve contention, poisoning, unknown records or foreign binding failure;
    /// no error changes policy state or falls back to an unmanaged observation.
    pub(super) fn policy_snapshot(&self) -> Result<SourcePolicySnapshot, SourcePolicyError> {
        let guard = self.source.cell.lock()?;
        guard.state()?.binding(self)?;
        SourcePolicySnapshot::capture(&guard, self.source.index, self.endpoint)
    }
}

/// Every constructor explicitly selects historical fixture or managed custody.
#[derive(Clone)]
pub(super) enum SourceMode {
    /// Historical one-shot observation; confers no managed-policy claim.
    UnmanagedLab,
    /// Mandatory original same-root source/endpoint binding.
    Managed(SourceBinding),
}

impl SourceMode {
    /// Derive original source/configuration context from the prepared policy owner.
    /// Unmanaged laboratory work makes no managed-source claim.
    pub(super) fn source_context(&self) -> Option<SourceContext> {
        match self {
            Self::UnmanagedLab => None,
            Self::Managed(binding) => Some(binding.context.clone()),
        }
    }

    /// Classify both current records before this candidate starts native work.
    /// Source refusal takes precedence; floors remain the final sender's waits.
    /// # Errors
    /// Preserve non-refusal custody/binding/availability failures without guessing scope.
    pub(super) fn pre_socket_refusal(
        &self,
    ) -> Result<Option<crate::AddressPolicyRefusalScope>, SourcePolicyError> {
        let Self::Managed(binding) = self else {
            return Ok(None);
        };
        let guard = binding.source.cell.lock()?;
        let state = guard.state()?;
        state.binding(binding)?;
        for (key, scope) in [
            (
                RecordKey::Source(binding.source.index),
                crate::AddressPolicyRefusalScope::Source,
            ),
            (
                RecordKey::Endpoint(binding.endpoint),
                crate::AddressPolicyRefusalScope::Endpoint,
            ),
        ] {
            match state.record(key)?.available() {
                Ok(()) => {}
                Err(SourcePolicyError::Disabled) => return Ok(Some(scope)),
                Err(error) => return Err(error),
            }
        }
        Ok(None)
    }
    /// Observe at most one release after a fresh availability check. This is
    /// only a retry hint: the actual setup must re-admit the same binding.
    /// # Errors
    /// Refuse unmanaged use or any fresh binding/availability failure other than
    /// contention. A release wake grants no permission or completed setup.
    pub(super) async fn retry_after_contention(&self) -> Result<(), SourcePolicyError> {
        let Self::Managed(binding) = self else {
            return Err(SourcePolicyError::Foreign);
        };
        let released = binding.source.cell.released.notified();
        tokio::pin!(released);
        released.as_mut().enable();
        match self.admit() {
            Ok(()) => Ok(()),
            Err(SourcePolicyError::Contended) => {
                #[cfg(test)]
                {
                    if let Ok(mut probe) = binding.source.cell.setup_contention_probe.try_lock()
                        && let Some(sender) = probe.take()
                    {
                        let _delivery = sender.try_send(());
                    }
                }
                released.await;
                Ok(())
            }
            Err(error) => Err(error),
        }
    }
    /// Admit current availability before preparation without checking send floors.
    /// Final send/receive custody remains mandatory after this earlier observation.
    /// # Errors
    /// Refuse policy contention/poisoning, an invalid managed binding or either
    /// applicable record's retained refusal/unavailability.
    pub(super) fn admit(&self) -> Result<(), SourcePolicyError> {
        let Self::Managed(binding) = self else {
            return Ok(());
        };
        let guard = binding.source.cell.lock()?;
        let state = guard.state()?;
        state.binding(binding)?;
        state
            .record(RecordKey::Source(binding.source.index))?
            .available()?;
        state
            .record(RecordKey::Endpoint(binding.endpoint))?
            .available()
    }
}

/// One actual attempted-send correlation, never issued by a session constructor.
#[derive(Clone)]
struct Correlation {
    /// Actual root cell identity.
    cell: Arc<PolicyCell>,
    /// Original source record.
    source: usize,
    /// Original endpoint record.
    endpoint: usize,
    /// Actual prepared socket/request allocation identity.
    request: Arc<()>,
    /// Epochs sampled at the concrete invocation, not during preparation.
    source_epoch: u64,
    /// Exact endpoint epoch at that same invocation.
    endpoint_epoch: u64,
    /// Fresh owner attempt observation for ordering control admission.
    sent_at: Instant,
}

/// Code-specific control semantics; peer poll fields never enter this type.
#[derive(Clone, Copy)]
enum ControlKind {
    /// Increase each directly affected record's own retained interval.
    Rate,
    /// Refuse the directly affected source and endpoint until authorized action.
    Deny,
    /// Apply the same refusal lifetime for a restricted server.
    Restrict,
}

impl ControlKind {
    /// Interpret only the three selected correlated control codes.
    /// # Errors
    /// Refuse every code other than RATE, DENY or RSTR without changing state.
    fn from_code(code: u32) -> Result<Self, SourcePolicyError> {
        match code {
            RATE => Ok(Self::Rate),
            DENY => Ok(Self::Deny),
            RSTR => Ok(Self::Restrict),
            _ => Err(SourcePolicyError::UnhandledControl),
        }
    }
}

/// Sealed staged request/cell/epoch evidence, not current receive permission.
pub(super) struct StagedPolicy {
    /// Closed historical or actual-request evidence, privately constructed.
    kind: StagedKind,
}

/// Explicit historical compatibility versus one exact actual managed request.
enum StagedKind {
    /// No managed claim is introduced for the existing fixture API.
    UnmanagedLab,
    /// Actual request correlation plus optional recognized control.
    Managed {
        /// Original cell, request and actual invocation epochs.
        stamp: Correlation,
        /// A recognized control, or an ordinary time-header path.
        control: Option<ControlKind>,
    },
}

/// Cached wait cause; only a fresh guarded concrete sender grants an effect.
#[derive(Clone, Copy)]
enum WaitState {
    /// Writable readiness may be selected; the actual sender still rechecks.
    Ready,
    /// Observe the already enabled release waiter before trying custody again.
    Contended,
    /// Await this eligibility time or an actual policy/control change.
    Floor(Instant),
}

/// One managed slot's private notification and outstanding-request custody.
struct ManagedLive {
    /// Original immutable source/endpoint owner.
    binding: SourceBinding,
    /// Issued only after a confirmed full actual send.
    pending: Option<Correlation>,
    /// Distinguishes contention release from actual policy/floor changes.
    wait: WaitState,
    /// Source's original actual-change observation.
    source_changes: watch::Receiver<()>,
    /// Exact endpoint's actual-change observation.
    endpoint_changes: watch::Receiver<()>,
    /// One preallocated pinned intrusive waiter, reset without allocating at send.
    released: Pin<Box<OwnedNotified>>,
}

impl ManagedLive {
    /// Enable before `try_lock`; notification custody spans the failed acquisition.
    fn arm_release(&mut self) {
        self.released
            .as_mut()
            .set(Arc::clone(&self.binding.source.cell.released).notified_owned());
        self.released.as_mut().enable();
    }

    /// Cancel unused release registration after successful acquisition.
    fn disarm_release(&mut self) {
        self.released
            .as_mut()
            .set(Arc::clone(&self.binding.source.cell.released).notified_owned());
    }
}

/// Closed live owner; unmanaged construction cannot accidentally omit managed fields.
pub(super) struct LivePolicy {
    /// Mandatory closed lifetime custody, without an optional managed escape.
    mode: LiveMode,
}

/// Native lifetime policy alternatives remain closed inside the leaf.
enum LiveMode {
    /// Existing one-shot fixture with no managed-policy claim.
    UnmanagedLab,
    /// Original checked same-process binding and its wake/request state.
    Managed(Box<ManagedLive>),
}

impl LivePolicy {
    /// Derive provenance only from this actual slot's retained owner.
    ///
    /// This immutable observation changes no policy, reads no clock and grants
    /// no permission. The final guarded commit alone attaches it to facts.
    pub(super) fn source_context(&self) -> Option<SourceContext> {
        match &self.mode {
            LiveMode::UnmanagedLab => None,
            LiveMode::Managed(live) => Some(live.binding.context.clone()),
        }
    }

    /// Borrow the configuration originally bound to this slot for conditional arithmetic.
    pub(super) fn source_configuration(&self) -> Option<&SourceConfigurationSnapshot> {
        match &self.mode {
            LiveMode::UnmanagedLab => None,
            LiveMode::Managed(live) => live.binding.context.configuration(),
        }
    }

    /// Receiving check for a setup result before any request can be driven.
    /// Identity is actual root/source/endpoint custody, not a copied label.
    pub(super) fn belongs_to(&self, mode: &SourceMode) -> bool {
        match (&self.mode, mode) {
            (LiveMode::UnmanagedLab, SourceMode::UnmanagedLab) => true,
            (LiveMode::Managed(live), SourceMode::Managed(binding)) => {
                Arc::ptr_eq(&live.binding.source.cell, &binding.source.cell)
                    && live.binding.source.index == binding.source.index
                    && live.binding.endpoint == binding.endpoint
                    && live.binding.key == binding.key
                    && SourceContext::same_optional(
                        Some(&live.binding.context),
                        Some(&binding.context),
                    )
            }
            _ => false,
        }
    }
    /// Allocate the single pinned release waiter outside actual sender custody.
    pub(super) fn from_mode(mode: SourceMode) -> Self {
        match mode {
            SourceMode::UnmanagedLab => Self {
                mode: LiveMode::UnmanagedLab,
            },
            SourceMode::Managed(binding) => Self::managed(binding, None),
        }
    }

    /// Subscribe before the first currentness check of this exact policy binding.
    fn managed(binding: SourceBinding, pending: Option<Correlation>) -> Self {
        let source_changes: watch::Receiver<()> = binding.source_changes.subscribe();
        let endpoint_changes: watch::Receiver<()> = binding.endpoint_changes.subscribe();
        let released: Pin<Box<OwnedNotified>> =
            Box::pin(Arc::clone(&binding.source.cell.released).notified_owned());
        Self {
            mode: LiveMode::Managed(Box::new(ManagedLive {
                binding,
                pending,
                wait: WaitState::Ready,
                source_changes,
                endpoint_changes,
                released,
            })),
        }
    }

    /// Readiness hint only; the mandatory sender independently rechecks current state.
    pub(super) fn wants_writable(&self) -> bool {
        match &self.mode {
            LiveMode::UnmanagedLab => true,
            LiveMode::Managed(live) => matches!(live.wait, WaitState::Ready),
        }
    }

    /// Refresh the actual pending correlation before admitting a socket readiness wait.
    /// A contended policy waits for release. This hint grants no publication authority.
    /// # Errors
    /// Refuse missing pending provenance or any currentness failure.
    #[cfg(all(feature = "ke-association-prototype", any(windows, test)))]
    pub(super) fn refresh_pending(&mut self, request: &Arc<()>) -> Result<bool, SourcePolicyError> {
        self.refresh(request)?;
        match &self.mode {
            LiveMode::Managed(live) if live.pending.is_some() => {
                Ok(matches!(live.wait, WaitState::Ready))
            }
            LiveMode::Managed(_) | LiveMode::UnmanagedLab => Err(SourcePolicyError::Foreign),
        }
    }

    /// Refresh pending availability or send waits with a fresh owned clock reading.
    /// Contention arms a bounded wait, never terminal retirement or permission.
    /// # Errors
    /// Refuse poisoned/unavailable custody, invalid or foreign binding/request
    /// identity, retained refusal or a stale outstanding correlation epoch.
    pub(super) fn refresh(&mut self, request: &Arc<()>) -> Result<(), SourcePolicyError> {
        self.refresh_observing(request, observations::InstantRead::hardware())
    }

    /// Use one deferred observation only if this refresh reaches the floor guard.
    /// # Errors
    /// Preserve the same binding/custody/refusal/epoch failures as hardware refresh.
    fn refresh_observing(
        &mut self,
        request: &Arc<()>,
        observation: observations::InstantRead,
    ) -> Result<(), SourcePolicyError> {
        let LiveMode::Managed(live) = &mut self.mode else {
            return Ok(());
        };
        live.source_changes.mark_unchanged();
        live.endpoint_changes.mark_unchanged();
        live.arm_release();
        let cell = Arc::clone(&live.binding.source.cell);
        let guard = match cell.lock() {
            Ok(guard) => guard,
            Err(SourcePolicyError::Contended) => {
                live.wait = WaitState::Contended;
                return Ok(());
            }
            Err(error) => return Err(error),
        };
        live.disarm_release();
        let state = guard.state()?;
        if let Some(stamp) = &live.pending {
            state.receiving(&live.binding, stamp, request)?;
            live.wait = WaitState::Ready;
        } else {
            let floor = state.earliest(&live.binding)?;
            let now = observation.read();
            live.wait = floor
                .filter(|value| now < *value)
                .map_or(WaitState::Ready, WaitState::Floor);
        }
        Ok(())
    }

    /// Await actual changes for every phase; release wakes apply only to contention.
    pub(super) async fn wake(&mut self) {
        let LiveMode::Managed(live) = &mut self.mode else {
            pending::<()>().await;
            return;
        };
        let floor = if let WaitState::Floor(value) = live.wait {
            Some(value)
        } else {
            None
        };
        let timer = async {
            if let Some(value) = floor {
                tokio::time::sleep_until(value.into()).await;
            } else {
                pending::<()>().await;
            }
        };
        tokio::pin!(timer);
        tokio::select! { biased;
            _ = live.source_changes.changed() => {},
            _ = live.endpoint_changes.changed() => {},
            () = live.released.as_mut(), if matches!(live.wait, WaitState::Contended) => {},
            () = &mut timer => {},
        }
        live.wait = WaitState::Ready;
    }

    /// Seal actual pending correlation and classify controls exactly once.
    /// # Errors
    /// Refuse missing outstanding correlation, foreign request identity or an
    /// unhandled stratum-zero code; no pending request is consumed here.
    pub(super) fn stage(
        &self,
        request: &Arc<()>,
        header: &RawHeader,
    ) -> Result<StagedPolicy, SourcePolicyError> {
        let kind = match &self.mode {
            LiveMode::UnmanagedLab => StagedKind::UnmanagedLab,
            LiveMode::Managed(live) => {
                let stamp = live.pending.as_ref().ok_or(SourcePolicyError::Stale)?;
                if !Arc::ptr_eq(&stamp.request, request) {
                    return Err(SourcePolicyError::Foreign);
                }
                let control = if header.stratum() == 0 {
                    Some(ControlKind::from_code(header.reference_id())?)
                } else {
                    None
                };
                StagedKind::Managed {
                    stamp: stamp.clone(),
                    control,
                }
            }
        };
        Ok(StagedPolicy { kind })
    }
}

/// Concrete delegation result, never a reusable allowed/denied permission value.
#[must_use = "historical unmanaged delegation must be handled explicitly"]
pub(super) enum ManagedEffect {
    /// Parent may run the existing explicitly unmanaged implementation.
    UnmanagedLab,
    /// The managed owner already performed or refused the concrete effect.
    Handled,
}

/// Own current policy admission, both debits and the actual nonblocking send.
/// The parent retains invocation-control read custody before entering here.
pub(super) fn attempt_managed_send(slot: &mut LiveSlot, deadline: Instant) -> ManagedEffect {
    attempt_managed_send_observing(slot, deadline, observations::SendReads::hardware())
}

/// Preserve one concrete send with deferred captures at its original positions.
fn attempt_managed_send_observing(
    slot: &mut LiveSlot,
    deadline: Instant,
    mut observations: observations::SendReads,
) -> ManagedEffect {
    let LiveMode::Managed(live) = &mut slot.policy.mode else {
        return ManagedEffect::UnmanagedLab;
    };
    let binding = live.binding.clone();
    live.arm_release();
    let cell = Arc::clone(&binding.source.cell);
    let mut guard = match cell.lock() {
        Ok(guard) => guard,
        Err(SourcePolicyError::Contended) => {
            live.wait = WaitState::Contended;
            return ManagedEffect::Handled;
        }
        Err(error) => {
            slot.finish(TerminalOutcome::SourcePolicy(error));
            return ManagedEffect::Handled;
        }
    };
    live.disarm_release();
    if !slot.can_send() {
        return ManagedEffect::Handled;
    }
    if slot.send_attempts == SEND_READY_CAP {
        slot.finish(TerminalOutcome::Budget);
        return ManagedEffect::Handled;
    }
    if EndpointKey::new(slot.peer) != Ok(binding.key) {
        slot.finish(TerminalOutcome::SourcePolicy(SourcePolicyError::Foreign));
        return ManagedEffect::Handled;
    }
    let before: SoftwareClock = observations.capture();
    if !slot.admit_send_capture(&before, deadline) {
        return ManagedEffect::Handled;
    }
    let state = match guard.state_mut() {
        Ok(state) => state,
        Err(error) => {
            slot.finish(TerminalOutcome::SourcePolicy(error));
            return ManagedEffect::Handled;
        }
    };
    match state.earliest(&binding) {
        Ok(Some(floor)) if before.mono_after < floor => {
            if let LiveMode::Managed(live) = &mut slot.policy.mode {
                live.wait = WaitState::Floor(floor);
            }
            return ManagedEffect::Handled;
        }
        Ok(_) => {}
        Err(error) => {
            slot.finish(TerminalOutcome::SourcePolicy(error));
            return ManagedEffect::Handled;
        }
    }
    let (stamp, floor) = match state.reserve(&binding, &slot.identity, before.mono_after) {
        Ok(reservation) => reservation,
        Err(error) => {
            slot.finish(TerminalOutcome::SourcePolicy(error));
            return ManagedEffect::Handled;
        }
    };
    slot.send_attempts += 1;
    let result = slot.socket.try_send_to(&slot.packet, slot.peer);
    let after = observations.capture();
    if let LiveMode::Managed(live) = &mut slot.policy.mode {
        live.pending = if matches!(&result, Ok(PACKET_BYTES)) {
            Some(stamp)
        } else {
            None
        };
    }
    complete_send(slot, result, SendBracket { before, after });
    if let LiveMode::Managed(live) = &mut slot.policy.mode {
        live.wait = WaitState::Floor(floor);
    }
    ManagedEffect::Handled
}

/// Hold current policy custody through the parent's concrete terminal publication.
/// The third argument is the original request deadline, never an old observation.
pub(super) fn commit_managed_reply(
    slot: &mut LiveSlot,
    candidate: &StagedReply,
    deadline: Instant,
) -> ManagedEffect {
    commit_managed_reply_observing(
        slot,
        candidate,
        deadline,
        observations::InstantRead::hardware(),
    )
}

/// Consume one deferred final observation under the same actual policy custody.
fn commit_managed_reply_observing(
    slot: &mut LiveSlot,
    candidate: &StagedReply,
    deadline: Instant,
    observation: observations::InstantRead,
) -> ManagedEffect {
    let (live, stamp, control) = match (&mut slot.policy.mode, &candidate.policy.kind) {
        (LiveMode::UnmanagedLab, StagedKind::UnmanagedLab) => return ManagedEffect::UnmanagedLab,
        (LiveMode::Managed(live), StagedKind::Managed { stamp, control }) => (live, stamp, control),
        _ => {
            slot.rejected += 1;
            slot.last_rejection = Some(super::Rejection::SourcePolicy(SourcePolicyError::Foreign));
            return ManagedEffect::Handled;
        }
    };
    let binding = live.binding.clone();
    let pending_stamp = live.pending.clone();
    live.arm_release();
    let cell = Arc::clone(&binding.source.cell);
    let mut guard = match cell.lock() {
        Ok(guard) => guard,
        Err(SourcePolicyError::Contended) => {
            live.wait = WaitState::Contended;
            slot.rejected += 1;
            return ManagedEffect::Handled;
        }
        Err(error) => {
            slot.rejected += 1;
            slot.finish(TerminalOutcome::SourcePolicy(error));
            return ManagedEffect::Handled;
        }
    };
    live.disarm_release();
    let now = observation.read();
    if now >= deadline {
        slot.rejected += 1;
        slot.expire();
        return ManagedEffect::Handled;
    }
    if !matches!(slot.phase, Phase::Pending(_)) {
        return ManagedEffect::Handled;
    }
    if now < candidate.read.mono_after {
        slot.rejected += 1;
        return ManagedEffect::Handled;
    }
    let admission = (|| {
        let state = guard.state_mut()?;
        let actual = pending_stamp.as_ref().ok_or(SourcePolicyError::Stale)?;
        state.receiving(&binding, actual, &slot.identity)?;
        if !Arc::ptr_eq(&actual.cell, &stamp.cell)
            || !Arc::ptr_eq(&actual.request, &stamp.request)
            || actual.source != stamp.source
            || actual.endpoint != stamp.endpoint
            || actual.source_epoch != stamp.source_epoch
            || actual.endpoint_epoch != stamp.endpoint_epoch
            || actual.sent_at != stamp.sent_at
        {
            slot.rejected += 1;
            slot.last_rejection = Some(super::Rejection::SourcePolicy(SourcePolicyError::Foreign));
            return Ok(false);
        }
        if now < stamp.sent_at {
            return Err(SourcePolicyError::Stale);
        }
        if let Some(kind) = control {
            // Policy effects require the whole original receive bracket at or after send.
            if candidate.read.mono < stamp.sent_at
                || candidate.read.mono > candidate.read.mono_after
            {
                slot.rejected += 1;
                return Ok(false);
            }
            state.control(&binding, *kind, now)?;
        }
        Ok(true)
    })();
    match admission {
        Err(error) => {
            slot.rejected += 1;
            slot.finish(TerminalOutcome::SourcePolicy(error));
        }
        Ok(true) => finish_valid_reply(slot, candidate, now),
        Ok(false) => {}
    }
    ManagedEffect::Handled
}

/// Create an isolated laboratory fixture root using the same bounded allocator.
/// # Errors
/// Preserve bounded source/endpoint allocation refusal before fixture publication.
#[cfg(test)]
pub(super) fn isolated_authority(
    limits: SourcePolicyLimits,
) -> Result<SourcePolicyAuthority, SourcePolicyError> {
    Ok(SourcePolicyAuthority {
        cell: PolicyCell::new(limits)?,
    })
}

/// Invoke one bounded causal setup probe while genuine policy custody is held.
/// This fixture-only helper creates no alternative production transition.
/// # Errors
/// Preserve actual lock/custody failure before invoking the probe.
#[cfg(test)]
pub(super) fn fixture_while_locked<T>(
    binding: &SourceBinding,
    probe: impl FnOnce() -> T,
) -> Result<T, SourcePolicyError> {
    let held = binding.source.cell.lock()?;
    let result = probe();
    drop(held);
    Ok(result)
}

/// Inert test observation of the actual record, without mutation/authority access.
#[cfg(test)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct RecordSnapshot {
    /// Actual own interval.
    interval: Duration,
    /// Actual retained floor.
    floor: Option<Instant>,
    /// Actual direct refusal.
    refused: bool,
    /// Actual ordinary revision.
    revision: u64,
    /// Actual correlation epoch.
    epoch: u64,
    /// Actual sticky unavailable state.
    unavailable: bool,
}

/// Observe source then endpoint through actual guarded record lookup.
/// # Errors
/// Preserve custody, invalid-binding and unknown-record failures without mutation.
#[cfg(test)]
fn record_snapshots(binding: &SourceBinding) -> Result<[RecordSnapshot; 2], SourcePolicyError> {
    let guard = binding.source.cell.lock()?;
    let state = guard.state()?;
    state.binding(binding)?;
    let snapshot = |record: &Record| RecordSnapshot {
        interval: record.interval,
        floor: record.floor,
        refused: record.refused,
        revision: record.revision,
        epoch: record.epoch,
        unavailable: record.unavailable,
    };
    Ok([
        snapshot(state.record(RecordKey::Source(binding.source.index))?),
        snapshot(state.record(RecordKey::Endpoint(binding.endpoint))?),
    ])
}

/// One checked kernel fixture request; no native-send fact is asserted.
#[cfg(test)]
pub(super) struct FixtureRequest {
    /// Same actual reservation stamp used by the production sender.
    stamp: Correlation,
    /// Consume-once fixture completion, independent of native qualification.
    consumed: bool,
}

/// Exercise the production debit kernel with deterministic laboratory time only.
/// # Errors
/// Preserve custody, binding, availability, not-due and arithmetic failures from
/// the actual reservation kernel; no native send is performed.
#[cfg(test)]
pub(super) fn fixture_send(
    binding: &SourceBinding,
    request: &Arc<()>,
    now: Instant,
) -> Result<FixtureRequest, SourcePolicyError> {
    let mut guard = binding.source.cell.lock()?;
    let (stamp, _floor) = guard.state_mut()?.reserve(binding, request, now)?;
    Ok(FixtureRequest {
        stamp,
        consumed: false,
    })
}

/// Exercise the same current receive and direct-control kernels once.
/// # Errors
/// Refuse consumed/stale or foreign requests, unhandled codes, unavailable
/// custody/records and observations preceding the actual reservation time.
#[cfg(test)]
pub(super) fn fixture_control(
    binding: &SourceBinding,
    request: &mut FixtureRequest,
    code: u32,
    now: Instant,
) -> Result<(), SourcePolicyError> {
    if request.consumed {
        return Err(SourcePolicyError::Stale);
    }
    let kind = ControlKind::from_code(code)?;
    let mut guard = binding.source.cell.lock()?;
    let state = guard.state_mut()?;
    state.receiving(binding, &request.stamp, &request.stamp.request)?;
    if now < request.stamp.sent_at {
        return Err(SourcePolicyError::Stale);
    }
    state.control(binding, kind, now)?;
    request.consumed = true;
    Ok(())
}

/// Exercise successful receive availability without treating a send floor as expiry.
/// # Errors
/// Refuse consumed requests and preserve the production receiver's custody,
/// binding, availability, request-identity and correlation-epoch failures.
#[cfg(test)]
fn fixture_receive(
    binding: &SourceBinding,
    request: &mut FixtureRequest,
) -> Result<(), SourcePolicyError> {
    if request.consumed {
        return Err(SourcePolicyError::Stale);
    }
    let guard = binding.source.cell.lock()?;
    guard
        .state()?
        .receiving(binding, &request.stamp, &request.stamp.request)?;
    request.consumed = true;
    Ok(())
}

#[cfg(test)]
#[path = "../tests/private/source_policy.rs"]
mod tests;
