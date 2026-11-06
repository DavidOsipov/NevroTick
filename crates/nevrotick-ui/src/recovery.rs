//! Process-local preview of explicit, unauthenticated certificate-date helpers.
//!
//! Roughtime is the automatic signed approximate-date path described by OD-02
//! and the v0.4 design. These optional plain-NTP helpers are separate, off by
//! default, and never ordinary synchronization votes. This module provides no
//! network request, signature evidence, date, NTS bypass or clock authority.
#![forbid(unsafe_code)]

use std::fmt;

use crate::model::SourcePreview;
use crate::source_drafts::{Protocol, RawSourceInput, SourceDraftError, SourceDrafts};

/// Closed, redacted preview rejection; rejected host text is never retained.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecoveryError {
    /// The signed count must select Off, one helper or two helpers.
    InvalidCount,
    /// A used helper failed shared identity or duplicate admission.
    Source(SourceDraftError),
}

impl fmt::Display for RecoveryError {
    /// Provide fixed user feedback without echoing raw hosts or parser details.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCount => formatter.write_str("Choose Off, one helper, or two helpers."),
            Self::Source(error) => write!(formatter, "Date helper: {error}"),
        }
    }
}

impl std::error::Error for RecoveryError {
    /// Preserve a redacted shared cause when identity admission failed.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidCount => None,
            Self::Source(error) => Some(error),
        }
    }
}

/// Immutable admitted helper preview, with at most two checked identities.
///
/// Construction does not apply settings. The empty default means the local
/// preview is Off, without making any observation about installed services.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RecoveryPreview {
    /// At most two distinct checked hosts; each has explicit NTP port 123.
    helpers: Vec<SourcePreview>,
}

impl RecoveryPreview {
    /// Admit a signed Off/one/two choice and only the corresponding used fields.
    ///
    /// Work is bounded before retaining raw host data; normalization, malformed
    /// IPv4 rejection and canonical duplicate detection have one shared owner.
    /// Unused widget drafts are neither retained nor interpreted as helpers.
    ///
    /// # Errors
    /// Reject unknown signed counts, invalid used identities or duplicates.
    pub fn parse(count: i32, first: &str, second: &str) -> Result<Self, RecoveryError> {
        let used = match count {
            0 => 0,
            1 => 1,
            2 => 2,
            _ => return Err(RecoveryError::InvalidCount),
        };
        let raw = [first, second].map(|host| RawSourceInput {
            label: "Date helper",
            host,
            protocol_choice: Protocol::PlainNtp.view_choice(),
            port: "123",
        });
        let admitted = SourceDrafts::from_baseline(&raw[..used]).map_err(RecoveryError::Source)?;
        Ok(Self {
            helpers: admitted
                .baseline()
                .iter()
                .map(|source| source.fields().clone())
                .collect(),
        })
    }

    /// Borrow the checked local helper preview; the caller cannot mutate it.
    #[must_use]
    pub fn helpers(&self) -> &[SourcePreview] {
        &self.helpers
    }
}

/// Sole owner of the submitted local preview, independent of raw widget drafts.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RecoveryDrafts {
    /// Off until a complete valid local preview replaces it.
    preview: RecoveryPreview,
}

impl RecoveryDrafts {
    /// Borrow the currently admitted local preview without an effect or copy.
    #[must_use]
    pub const fn preview(&self) -> &RecoveryPreview {
        &self.preview
    }

    /// Replace only the in-memory helper preview after complete shared admission.
    ///
    /// # Errors
    /// Every rejection preserves the entire prior preview and retains no raw data.
    pub fn update(&mut self, count: i32, first: &str, second: &str) -> Result<(), RecoveryError> {
        let candidate = RecoveryPreview::parse(count, first, second)?;
        self.preview = candidate;
        Ok(())
    }

    /// Discard the local preview, without changing any service or active policy.
    pub fn discard(&mut self) {
        self.preview = RecoveryPreview::default();
    }
}
