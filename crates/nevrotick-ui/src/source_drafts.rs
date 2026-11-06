//! Checked, process-local source drafts; no Apply, network, persistence or policy.
//!
//! The baseline and effective preview each contain at most 16 logical identities
//! (architecture §6.2). At most one change per ID is retained. The additional
//! 32-change ceiling is a UI bookkeeping budget derived from replacing all 16
//! baseline entries with 16 additions, not a new backend policy limit. Session
//! IDs identify editor rows, not production logical-source or endpoint authority.
//!
//! Every raw handler constructs fields through the existing `SourcePreview`
//! owner. This layer additionally owns the protocol, normalized identity key,
//! source-count budget and stable-ID relationships. Rejection preserves the
//! entire accepted baseline and draft state. Raw rejected widget text stays in
//! the view; errors contain fixed messages only.
#![forbid(unsafe_code)]

use std::{fmt, net::IpAddr};

use crate::model::{MAX_HOST_BYTES, ModelError, PreviewModel, SourcePreview};

/// Selected architecture §6.2 logical-source limit, also bounding this preview.
pub const MAX_SOURCES: usize = 16;
/// UI-only retained delta bound; separate from any production policy format.
pub const MAX_SOURCE_CHANGES: usize = 32;

/// An explicit source protocol; plain NTP is never an automatic NTS fallback.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Protocol {
    /// Network Time Security, whose source editor port is the NTS-KE port.
    Nts,
    /// Explicit unauthenticated NTP, with its own transport port.
    PlainNtp,
}

impl Protocol {
    /// Admit the closed signed GUI choice: zero is NTS, one is plain NTP.
    ///
    /// # Errors
    /// Any other value rejects without choosing a fallback protocol.
    pub const fn from_view(choice: i32) -> Result<Self, SourceDraftError> {
        match choice {
            0 => Ok(Self::Nts),
            1 => Ok(Self::PlainNtp),
            _ => Err(SourceDraftError::InvalidProtocol),
        }
    }

    /// Return the stable signed GUI choice, without granting network authority.
    #[must_use]
    pub const fn view_choice(self) -> i32 {
        match self {
            Self::Nts => 0,
            Self::PlainNtp => 1,
        }
    }

    /// Return the documented source-editor default for this protocol.
    #[must_use]
    pub const fn default_port(self) -> u16 {
        match self {
            Self::Nts => 4460,
            Self::PlainNtp => 123,
        }
    }
}

/// Session-local positive signed ID, stable across edits, filtering and removal.
///
/// Only the workflow creates IDs; consumer handlers additionally check current
/// membership. IDs removed by Undo or Discard are never reused in this session.
/// Raw construction is unavailable:
/// ```compile_fail,E0423
/// use nevrotick_ui::source_drafts::SourceId;
/// let id = SourceId(0);
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SourceId(i32);

impl SourceId {
    /// Return the signed callback value; callers must not use row indices as IDs.
    #[must_use]
    pub const fn view_id(self) -> i32 {
        self.0
    }

    /// Admit only a positive representation; current membership is a later guard.
    ///
    /// # Errors
    /// Reject zero and negatives before any collection lookup.
    fn from_view(raw: i32) -> Result<Self, SourceDraftError> {
        if raw > 0 {
            Ok(Self(raw))
        } else {
            Err(SourceDraftError::UnknownSource)
        }
    }
}

/// Borrowed, inert GUI fields. Every receiving operation admits all four fields.
#[derive(Clone, Copy, Debug)]
pub struct RawSourceInput<'a> {
    /// Raw friendly label, with the existing bounded ASCII field contract.
    pub label: &'a str,
    /// Raw DNS/IP identity; one optional final DNS root dot is supported.
    pub host: &'a str,
    /// Closed signed protocol choice, never accepted from an unchecked enum cast.
    pub protocol_choice: i32,
    /// Raw unsigned decimal port text, admitted by the existing checked owner.
    pub port: &'a str,
}

/// Fixed rejection reasons; no variant stores or formats rejected input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SourceDraftError {
    /// Existing shared label/host/port admission failed.
    Field(ModelError),
    /// The GUI supplied an unknown protocol choice.
    InvalidProtocol,
    /// The signed ID is invalid, missing, removed, or has no pending change.
    UnknownSource,
    /// Another effective entry has this normalized host and protocol.
    DuplicateIdentity,
    /// The selected logical-source or bounded retained-change budget is full.
    Capacity,
    /// This session consumed every positive signed view ID; none may be reused.
    IdExhausted,
}

impl fmt::Display for SourceDraftError {
    /// Emit fixed product feedback, never raw values or internal parser details.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Field(error) => error.fmt(formatter),
            Self::InvalidProtocol => formatter.write_str("Choose NTS or explicitly selected NTP."),
            Self::UnknownSource => {
                formatter.write_str("Choose a current source or preview change.")
            }
            Self::DuplicateIdentity => {
                formatter.write_str("This protocol and server identity are already in the preview.")
            }
            Self::Capacity => formatter.write_str("The source preview has reached its limit."),
            Self::IdExhausted => {
                formatter.write_str("Restart the preview before adding another source.")
            }
        }
    }
}

impl std::error::Error for SourceDraftError {}

impl From<ModelError> for SourceDraftError {
    /// Preserve the shared redacted field reason without retaining raw input.
    fn from(error: ModelError) -> Self {
        Self::Field(error)
    }
}

/// Canonical comparison only; no DNS, alias resolution or operator-vote claims.
#[derive(Clone, Debug, Eq, PartialEq)]
enum HostKey {
    /// DNS comparison is ASCII case-insensitive and excludes one final root dot.
    Dns(String),
    /// Numeric addresses compare parsed bytes within their IP address family.
    Ip(IpAddr),
}

/// Immutable checked source and protocol under a session-local editor identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DraftSource {
    /// Stable view identity, not an array position or backend endpoint ID.
    id: SourceId,
    /// Explicit checked protocol, independent of the displayed port.
    protocol: Protocol,
    /// Label/host/port carry the existing checked constructor's guarantees.
    fields: SourcePreview,
    /// Private normalized key, immutable for this accepted value.
    key: HostKey,
}

impl DraftSource {
    /// Return the stable session identity without exposing a mutation path.
    #[must_use]
    pub const fn id(&self) -> SourceId {
        self.id
    }

    /// Return the explicit protocol without inferring it from a port number.
    #[must_use]
    pub const fn protocol(&self) -> Protocol {
        self.protocol
    }

    /// Borrow checked field values; callers cannot mutate them through this view.
    #[must_use]
    pub const fn fields(&self) -> &SourcePreview {
        &self.fields
    }

    /// Construct a candidate before publishing any workflow mutation.
    ///
    /// # Errors
    /// Reject unknown protocols, shared field failures and malformed IPv4-like
    /// dotted quads. Raw host work is bounded before normalization/allocation.
    fn parse(id: SourceId, raw: RawSourceInput<'_>) -> Result<Self, SourceDraftError> {
        let protocol = Protocol::from_view(raw.protocol_choice)?;
        if raw.host.is_empty()
            || raw.host.len() > MAX_HOST_BYTES
            || !raw.host.bytes().all(|byte| (b'!'..=b'~').contains(&byte))
            || raw.host.ends_with("..")
        {
            return Err(ModelError::InvalidHost.into());
        }
        let host = if raw.host.contains(':') {
            raw.host
        } else {
            match raw.host.strip_suffix('.') {
                Some(without_root_dot) => without_root_dot,
                None => raw.host,
            }
        };
        let fields = SourcePreview::parse(raw.label, host, raw.port)?;
        let key = if let Ok(address) = fields.host().parse::<IpAddr>() {
            HostKey::Ip(address)
        } else {
            // The older field constructor permits numeric DNS labels. A
            // four-numeric-label spelling is instead an IPv4 literal in
            // this source UI, matching the documented HTML admission rule.
            let mut labels = fields.host().split('.');
            let numeric_quad = labels.clone().count() == 4
                && labels.all(|label| label.bytes().all(|byte| byte.is_ascii_digit()));
            if numeric_quad {
                return Err(ModelError::InvalidHost.into());
            }
            HostKey::Dns(fields.host().to_ascii_lowercase())
        };
        Ok(Self {
            id,
            protocol,
            fields,
            key,
        })
    }

    /// Compare the logical identity key; a different port creates no extra vote.
    fn same_identity(&self, other: &Self) -> bool {
        self.protocol == other.protocol && self.key == other.key
    }
}

/// Kind of a single local difference from the immutable baseline.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChangeKind {
    /// An added preview entry, with no baseline source.
    Add,
    /// Replacement of one baseline source, preserving its view identity.
    Edit,
    /// Local removal of one baseline source, without changing active settings.
    Remove,
}

/// Immutable review row; no change represents backend acceptance or success.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceChange {
    /// Stable identity shared by the old/new projection where present.
    id: SourceId,
    /// Closed review operation; constructors preserve before/after consistency.
    kind: ChangeKind,
    /// Original baseline value for Edit/Remove; absent for Add.
    before: Option<DraftSource>,
    /// Checked local candidate for Add/Edit; absent for Remove.
    after: Option<DraftSource>,
}

impl SourceChange {
    /// Return the stable review-row identity for Undo.
    #[must_use]
    pub const fn id(&self) -> SourceId {
        self.id
    }

    /// Return the review operation without inferring it from optional fields.
    #[must_use]
    pub const fn kind(&self) -> ChangeKind {
        self.kind
    }

    /// Borrow the original baseline value if this change has one.
    #[must_use]
    pub const fn before(&self) -> Option<&DraftSource> {
        self.before.as_ref()
    }

    /// Borrow the checked local candidate if this change has one.
    #[must_use]
    pub const fn after(&self) -> Option<&DraftSource> {
        self.after.as_ref()
    }
}

/// Bounded baseline and local delta owner; no mutation can change the baseline.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceDrafts {
    /// Immutable fixture baseline, admitted and unique at construction.
    baseline: Vec<DraftSource>,
    /// One review delta per stable ID, bounded independently of effective count.
    changes: Vec<SourceChange>,
    /// Next positive signed view ID, or None after its finite domain is consumed.
    next_id: Option<i32>,
}

impl SourceDrafts {
    /// Admit a bounded fixture baseline with unique normalized host+protocol keys.
    ///
    /// # Errors
    /// Oversized, duplicate or invalid baselines reject before publication. The
    /// source count is checked before retaining any caller-owned collection data.
    pub fn from_baseline(raw: &[RawSourceInput<'_>]) -> Result<Self, SourceDraftError> {
        if raw.len() > MAX_SOURCES {
            return Err(SourceDraftError::Capacity);
        }
        let mut owner = Self {
            baseline: Vec::with_capacity(raw.len()),
            changes: Vec::new(),
            next_id: Some(1),
        };
        for input in raw {
            let (id, next) = owner.available_id()?;
            let candidate = DraftSource::parse(id, *input)?;
            if owner
                .baseline
                .iter()
                .any(|source| source.same_identity(&candidate))
            {
                return Err(SourceDraftError::DuplicateIdentity);
            }
            owner.baseline.push(candidate);
            owner.next_id = next;
        }
        Ok(owner)
    }

    /// Reuse the existing checked three-source fixture without changing its owner.
    ///
    /// # Errors
    /// Returns a shared field error if the maintained fixture becomes invalid.
    pub fn fixture() -> Result<Self, SourceDraftError> {
        let fixture = PreviewModel::fixture()?;
        let ports: Vec<String> = fixture
            .sources()
            .iter()
            .map(|source| source.port().to_string())
            .collect();
        let raw: Vec<RawSourceInput<'_>> = fixture
            .sources()
            .iter()
            .zip(&ports)
            .map(|(source, port)| RawSourceInput {
                label: source.label(),
                host: source.host(),
                protocol_choice: Protocol::Nts.view_choice(),
                port,
            })
            .collect();
        Self::from_baseline(&raw)
    }

    /// Borrow the immutable fixture baseline, which draft operations never alter.
    #[must_use]
    pub fn baseline(&self) -> &[DraftSource] {
        &self.baseline
    }

    /// Borrow local review rows; none claims an applied or active policy change.
    #[must_use]
    pub fn review(&self) -> &[SourceChange] {
        &self.changes
    }

    /// Project at most 16 effective entries with stable IDs and baseline order.
    #[must_use]
    pub fn preview(&self) -> Vec<&DraftSource> {
        let mut preview = Vec::with_capacity(MAX_SOURCES);
        for source in &self.baseline {
            match self.changes.iter().find(|change| change.id == source.id) {
                Some(change) => {
                    if let Some(after) = &change.after {
                        preview.push(after);
                    }
                }
                None => preview.push(source),
            }
        }
        preview.extend(
            self.changes
                .iter()
                .filter(|change| change.kind == ChangeKind::Add)
                .filter_map(|change| change.after.as_ref()),
        );
        preview
    }

    /// Resolve a signed stable ID against the current effective preview.
    ///
    /// # Errors
    /// Invalid, unknown or currently removed IDs reject rather than becoming rows.
    pub fn source(&self, view_id: i32) -> Result<&DraftSource, SourceDraftError> {
        let id = SourceId::from_view(view_id)?;
        self.preview()
            .into_iter()
            .find(|source| source.id == id)
            .ok_or(SourceDraftError::UnknownSource)
    }

    /// Add one checked local candidate; baseline and old drafts remain unchanged.
    ///
    /// # Errors
    /// Capacity, unknown protocol, invalid fields, duplicate identity or ID-domain
    /// exhaustion reject atomically, including preserving the next-ID cursor.
    pub fn add(&mut self, raw: RawSourceInput<'_>) -> Result<SourceId, SourceDraftError> {
        if self.preview().len() >= MAX_SOURCES || self.changes.len() >= MAX_SOURCE_CHANGES {
            return Err(SourceDraftError::Capacity);
        }
        let (id, next) = self.available_id()?;
        let candidate = DraftSource::parse(id, raw)?;
        self.check_unique(&candidate)?;
        self.changes.push(SourceChange {
            id,
            kind: ChangeKind::Add,
            before: None,
            after: Some(candidate),
        });
        self.next_id = next;
        Ok(id)
    }

    /// Replace one effective entry's local candidate, preserving its stable ID.
    ///
    /// # Errors
    /// Signed-ID/current membership, field, protocol, uniqueness and retained
    /// capacity checks happen before any accepted baseline or draft mutation.
    pub fn edit(&mut self, view_id: i32, raw: RawSourceInput<'_>) -> Result<(), SourceDraftError> {
        let id = self.source(view_id)?.id;
        let candidate = DraftSource::parse(id, raw)?;
        self.check_unique(&candidate)?;
        let before = self.baseline.iter().find(|source| source.id == id).cloned();
        if before.as_ref() == Some(&candidate) {
            self.changes.retain(|change| change.id != id);
            return Ok(());
        }
        let kind = if before.is_some() {
            ChangeKind::Edit
        } else {
            ChangeKind::Add
        };
        self.replace_change(SourceChange {
            id,
            kind,
            before,
            after: Some(candidate),
        })
    }

    /// Preview removal; removing an added candidate discards that addition only.
    ///
    /// # Errors
    /// Unknown/removed/signed-invalid IDs or retained capacity reject atomically.
    pub fn remove(&mut self, view_id: i32) -> Result<(), SourceDraftError> {
        let id = self.source(view_id)?.id;
        let before = self.baseline.iter().find(|source| source.id == id).cloned();
        if let Some(source) = before {
            self.replace_change(SourceChange {
                id,
                kind: ChangeKind::Remove,
                before: Some(source),
                after: None,
            })
        } else {
            self.changes.retain(|change| change.id != id);
            Ok(())
        }
    }

    /// Undo the whole per-ID delta, restoring its original baseline value.
    ///
    /// # Errors
    /// Invalid/no-delta IDs reject. Restoring an original after another candidate
    /// reused its identity can collide or exceed 16 effective entries; reject and
    /// retain every draft rather than silently removing another source.
    pub fn undo(&mut self, view_id: i32) -> Result<(), SourceDraftError> {
        let id = SourceId::from_view(view_id)?;
        let change = self
            .changes
            .iter()
            .find(|change| change.id == id)
            .ok_or(SourceDraftError::UnknownSource)?;
        if let Some(before) = &change.before {
            self.check_unique(before)?;
            if change.kind == ChangeKind::Remove && self.preview().len() >= MAX_SOURCES {
                return Err(SourceDraftError::Capacity);
            }
        }
        self.changes.retain(|change| change.id != id);
        Ok(())
    }

    /// Discard all local changes, keeping the baseline and consumed IDs unchanged.
    pub fn discard(&mut self) {
        self.changes.clear();
    }

    /// Obtain a fresh ID and successor without consuming either on rejection.
    ///
    /// # Errors
    /// Return a fixed exhaustion error after the positive signed domain is used.
    fn available_id(&self) -> Result<(SourceId, Option<i32>), SourceDraftError> {
        let value = self.next_id.ok_or(SourceDraftError::IdExhausted)?;
        Ok((SourceId(value), value.checked_add(1)))
    }

    /// Check current effective identity uniqueness, ignoring the candidate's ID.
    ///
    /// # Errors
    /// Duplicate host+protocol rejects regardless of the peer's port or label.
    fn check_unique(&self, candidate: &DraftSource) -> Result<(), SourceDraftError> {
        if self
            .preview()
            .iter()
            .any(|source| source.id != candidate.id && source.same_identity(candidate))
        {
            Err(SourceDraftError::DuplicateIdentity)
        } else {
            Ok(())
        }
    }

    /// Replace the sole per-ID change, admitting capacity before any mutation.
    ///
    /// # Errors
    /// A new review row cannot exceed the UI retained-change budget.
    fn replace_change(&mut self, change: SourceChange) -> Result<(), SourceDraftError> {
        if let Some(existing) = self
            .changes
            .iter_mut()
            .find(|existing| existing.id == change.id)
        {
            *existing = change;
            return Ok(());
        }
        if self.changes.len() >= MAX_SOURCE_CHANGES {
            return Err(SourceDraftError::Capacity);
        }
        self.changes.push(change);
        Ok(())
    }
}

/// Required projection after an accepted protocol choice.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PortUpdate {
    /// The field was untouched; replace its text with this protocol default.
    Default(u16),
    /// Keep the view's raw manually edited text, including an invalid draft.
    PreserveDraft,
}

/// Checked editor port and provenance; raw invalid text remains in the view.
///
/// The default follows protocol changes until any manual text edit, including
/// invalid text or a standard default entered explicitly. No view-supplied
/// flag may pretend a manual port is untouched. Submission must still call the
/// full source workflow handler with the other current raw editor fields.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProtocolPortDraft {
    /// Current explicit protocol, admitted from a closed GUI choice.
    protocol: Protocol,
    /// Accepted nonzero port, validated through the existing field constructor.
    port: u16,
    /// Only constructor/default-switch paths set this true.
    follows_default: bool,
}

impl ProtocolPortDraft {
    /// Start an add editor with the selected protocol's untouched default.
    #[must_use]
    pub const fn new(protocol: Protocol) -> Self {
        Self {
            protocol,
            port: protocol.default_port(),
            follows_default: true,
        }
    }

    /// Open an accepted source editor; an existing standard port starts untouched.
    #[must_use]
    pub fn from_source(source: &DraftSource) -> Self {
        Self {
            protocol: source.protocol,
            port: source.fields.port(),
            follows_default: source.fields.port() == source.protocol.default_port(),
        }
    }

    /// Return the currently accepted editor protocol.
    #[must_use]
    pub const fn protocol(&self) -> Protocol {
        self.protocol
    }

    /// Return the currently accepted editor port, with its protocol-specific label.
    #[must_use]
    pub const fn port(&self) -> u16 {
        self.port
    }

    /// Return whether a protocol switch may still replace the untouched default.
    #[must_use]
    pub const fn follows_default(&self) -> bool {
        self.follows_default
    }

    /// Record any user text edit before validation, without retaining its bytes.
    ///
    /// This monotone presentation event grants no field admission. The host must
    /// call it only after raw character/size admission, before domain parsing.
    /// An admitted format-invalid manual draft survives a later default switch;
    /// a rejected overbudget or forbidden-character input does not mark a port.
    pub fn mark_port_edited(&mut self) {
        self.follows_default = false;
    }

    /// Change protocol; only an untouched default port follows the new default.
    ///
    /// `PreserveDraft` forbids overwriting the raw view text with `port()`: the
    /// latter remains the last accepted port while the user may be editing an
    /// invalid raw draft. Complete source submission always rechecks current raw
    /// fields, never treating this presentation helper as admission evidence.
    ///
    /// # Errors
    /// Unknown signed choices preserve the entire accepted editor draft.
    pub fn switch_protocol(&mut self, choice: i32) -> Result<PortUpdate, SourceDraftError> {
        let protocol = Protocol::from_view(choice)?;
        self.protocol = protocol;
        if self.follows_default {
            self.port = protocol.default_port();
            Ok(PortUpdate::Default(self.port))
        } else {
            Ok(PortUpdate::PreserveDraft)
        }
    }

    /// Accept an explicitly entered port through the existing field rule owner.
    ///
    /// # Errors
    /// Invalid/oversized raw ports preserve protocol, port and provenance. The
    /// code-owned placeholder fields exercise no DNS and are never retained.
    pub fn set_port(&mut self, raw: &str) -> Result<(), SourceDraftError> {
        let checked = SourcePreview::parse("Port preview", "port.example.invalid", raw)?;
        self.port = checked.port();
        self.follows_default = false;
        Ok(())
    }
}

#[cfg(test)]
mod cursor_boundary_tests {
    //! Finite cursor boundary, otherwise requiring billions of public additions.

    use super::{RawSourceInput, SourceDraftError, SourceDrafts};

    /// The final ID is usable once; rejection does not consume it or recycle it.
    ///
    /// # Errors
    /// Propagates unexpected empty fixture/add/remove rejection.
    ///
    /// # Panics
    /// Fails when arithmetic exhaustion wraps or an invalid request consumes ID.
    #[test]
    fn final_signed_id_is_atomic_non_wrapping_and_never_recycled() -> Result<(), SourceDraftError> {
        let mut drafts = SourceDrafts::from_baseline(&[])?;
        // Deliberately scoped test setup: this is the same private cursor state
        // reached after consuming all prior IDs, with no test-only public API.
        drafts.next_id = Some(i32::MAX);
        let before = drafts.clone();
        let invalid = RawSourceInput {
            label: "Valid",
            host: "last.example.invalid",
            protocol_choice: -1,
            port: "4460",
        };
        assert_eq!(drafts.add(invalid), Err(SourceDraftError::InvalidProtocol));
        assert_eq!(drafts, before);
        let valid = RawSourceInput {
            protocol_choice: 0,
            ..invalid
        };
        let last = drafts.add(valid)?;
        assert_eq!(last.view_id(), i32::MAX);
        let before = drafts.clone();
        assert_eq!(drafts.add(valid), Err(SourceDraftError::IdExhausted));
        assert_eq!(drafts, before);
        drafts.remove(last.view_id())?;
        let before = drafts.clone();
        assert_eq!(drafts.add(valid), Err(SourceDraftError::IdExhausted));
        assert_eq!(drafts, before);
        drafts.discard();
        assert_eq!(drafts.add(valid), Err(SourceDraftError::IdExhausted));
        Ok(())
    }
}
