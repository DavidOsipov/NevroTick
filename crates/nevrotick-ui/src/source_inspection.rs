//! Read-only source query/details over the existing checked source owner.
//!
//! This module never edits sources, IDs, selection, drafts or Statistics. Only a
//! separate visible-ID projection is filtered. Used/Other roles have no admitted
//! evidence feed in UI-M0; their controls/receivers remain explicitly unavailable.
#![forbid(unsafe_code)]

use std::fmt;

use garde::Validate;

use crate::source_drafts::{DraftSource, Protocol, SourceDraftError, SourceDrafts, SourceId};

/// M0-only raw search budget, in ASCII bytes; not a backend policy field.
pub const MAX_QUERY_BYTES: usize = 253;

/// Closed source-list filter identity; roles never derive from editor selection.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SourceFilter {
    /// Show every source matching the accepted query.
    #[default]
    All,
    /// Requires qualified Main/Supporting roles, absent from this fixture.
    Used,
    /// Requires qualified non-used roles, absent from this fixture.
    Other,
}

impl SourceFilter {
    /// Admit only the code-owned filter selector representation.
    ///
    /// # Errors
    /// Unknown signed choices reject; availability is a separate receiving guard.
    pub const fn from_view(raw: i32) -> Result<Self, InspectionError> {
        match raw {
            0 => Ok(Self::All),
            1 => Ok(Self::Used),
            2 => Ok(Self::Other),
            _ => Err(InspectionError::InvalidFilter),
        }
    }

    /// Explain the reached role-data dependency without inventing source roles.
    #[must_use]
    pub const fn unavailable_reason(self) -> Option<&'static str> {
        match self {
            Self::All => None,
            Self::Used | Self::Other => Some(
                "Selection roles are not available in this fixture. The selected editor row is not a Main or Supporting source.",
            ),
        }
    }
}

/// Fixed local rejection, containing no query/host payload or unchecked ID text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InspectionError {
    /// Query exceeds the byte bound or contains bytes outside printable ASCII.
    InvalidQuery,
    /// Filter choice is unknown to the closed selector.
    InvalidFilter,
    /// A known Used/Other filter has no qualified role evidence to consume.
    RolesUnavailable,
    /// Current source membership rejected a signed/stale identity.
    Source(SourceDraftError),
    /// A real source exists but does not match the accepted visible query.
    HiddenSource,
}

impl fmt::Display for InspectionError {
    /// Render fixed guidance, never rejected query or identity bytes.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidQuery => formatter.write_str("Use up to 253 printable ASCII search characters. An empty search shows all sources."),
            Self::InvalidFilter => formatter.write_str("Choose a listed source filter."),
            Self::RolesUnavailable => formatter.write_str("Used and Other filters need selection evidence, which is unavailable in this fixture."),
            Self::Source(error) => error.fmt(formatter),
            Self::HiddenSource => formatter.write_str("This source is hidden by the current search. Clear the search or choose a visible source."),
        }
    }
}

impl std::error::Error for InspectionError {
    /// Preserve the bounded typed membership cause without any offending input.
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Source(error) => Some(error),
            Self::InvalidQuery
            | Self::InvalidFilter
            | Self::RolesUnavailable
            | Self::HiddenSource => None,
        }
    }
}

impl From<SourceDraftError> for InspectionError {
    /// Preserve the existing fixed current-membership error without raw data.
    fn from(error: SourceDraftError) -> Self {
        Self::Source(error)
    }
}

/// Borrowed raw search input, with mandatory shared validation at construction.
#[derive(Validate)]
struct QueryInput<'a> {
    /// ASCII search text; byte-bound before validation report allocation.
    #[garde(custom(query_rule))]
    value: &'a str,
}

/// Check the UI-only query representation without trimming or copying it.
///
/// # Errors
/// Reject oversized/control/non-ASCII data; empty and ordinary spaces are valid.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde requires custom rules to borrow their context"
)]
fn query_rule(value: &str, (): &()) -> garde::Result {
    if value.len() <= MAX_QUERY_BYTES && value.bytes().all(|byte| (b' '..=b'~').contains(&byte)) {
        Ok(())
    } else {
        Err(garde::Error::new("invalid source query"))
    }
}

/// Checked ASCII search, preserving accepted spelling and bounded casefolded text.
///
/// Raw bypasses are unavailable:
/// ```compile_fail,E0451
/// use nevrotick_ui::source_inspection::SourceQuery;
/// let query = SourceQuery { raw: String::from("raw"), folded: String::from("raw") };
/// ```
/// Checked legal use remains available:
/// ```
/// use nevrotick_ui::source_inspection::SourceQuery;
/// let query = SourceQuery::parse("North")?;
/// assert_eq!(query.text(), "North");
/// # Ok::<(), nevrotick_ui::source_inspection::InspectionError>(())
/// ```
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SourceQuery {
    /// Exact accepted raw ASCII; no trimming or destructive truncation.
    raw: String,
    /// ASCII-only casefold preserves the already admitted byte-length bound.
    folded: String,
}

impl SourceQuery {
    /// Admit raw length/ASCII syntax before either bounded accepted allocation.
    ///
    /// # Errors
    /// Rejection contains no input and constructs no accepted query.
    pub fn parse(raw: &str) -> Result<Self, InspectionError> {
        if raw.len() > MAX_QUERY_BYTES {
            return Err(InspectionError::InvalidQuery);
        }
        QueryInput { value: raw }
            .validate()
            .map_err(|_| InspectionError::InvalidQuery)?;
        Ok(Self {
            raw: raw.to_owned(),
            folded: raw.to_ascii_lowercase(),
        })
    }

    /// Borrow the exact admitted search text without exposing mutation.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.raw
    }

    /// Match only checked friendly-label/server-identity fields, without DNS.
    fn matches(&self, source: &DraftSource) -> bool {
        self.folded.is_empty()
            || source
                .fields()
                .label()
                .to_ascii_lowercase()
                .contains(&self.folded)
            || source
                .fields()
                .host()
                .to_ascii_lowercase()
                .contains(&self.folded)
    }
}

/// Stable code-owned detail field, separate from a backend observation DTO.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DetailFact {
    /// Packet authentication is not established by selecting an NTS preview.
    PacketAuthentication,
    /// Most recent authenticated time measurement, distinct from KE/reachability.
    LastAuthenticatedMeasurement,
    /// Source membership in qualified selection, unrelated to editor selection.
    SelectionRole,
    /// Operator/independence grouping has not been supplied by these fixtures.
    OperatorGroup,
    /// No DNS resolution or actual exchange endpoint exists in the source model.
    ActualEndpoint,
    /// A session editor ID is not a production independent-vote identity.
    IndependentVoteIdentity,
    /// Current network restriction/control-response state is unavailable.
    RequestStatus,
    /// No measurement reply was acquired.
    LastReply,
    /// No accepted measurement was acquired.
    LastAcceptedMeasurement,
    /// No offset value or sign can be invented.
    Offset,
    /// No network delay value can be invented.
    Delay,
    /// No conditional-model uncertainty has been supplied.
    Uncertainty,
    /// NTS certificate-date provenance has not been supplied.
    CertificateTimeBasis,
    /// Protocol choice alone establishes no time scale.
    TimeScale,
    /// No packet leap announcement has been observed.
    LeapAnnouncement,
}

/// Honest absence/inherent-protocol meaning, never fabricated successful evidence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AbsentFact {
    /// Evidence has not been observed by a publication owner.
    NotObserved,
    /// No accepted measurement value exists.
    NotMeasured,
    /// Authentication/operator/vote identity has not been established.
    NotEstablished,
    /// Normal-source selection has not been evaluated.
    NotEvaluated,
    /// The fact has no meaning for an explicit plain-NTP source.
    NotApplicable,
    /// Plain NTP supplies no packet authentication, independently of freshness.
    UnauthenticatedProtocol,
}

/// Bounded current local detail snapshot, with checked fields and explicit scope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceDetails {
    /// Immutable checked display fields/protocol under a stable local view ID.
    source: DraftSource,
    /// The current query hides this retained detail target; UI must explain it.
    hidden_by_query: bool,
}

impl SourceDetails {
    /// Borrow checked local fields; this is not an endpoint/authentication sample.
    #[must_use]
    pub const fn source(&self) -> &DraftSource {
        &self.source
    }

    /// Explain retained details whose identity no longer matches the query.
    #[must_use]
    pub const fn hidden_by_query(&self) -> bool {
        self.hidden_by_query
    }

    /// Project only inherent protocol meaning and absent fixture evidence.
    #[must_use]
    pub fn fact(&self, fact: DetailFact) -> AbsentFact {
        match fact {
            DetailFact::PacketAuthentication => match self.source.protocol() {
                Protocol::Nts => AbsentFact::NotEstablished,
                Protocol::PlainNtp => AbsentFact::UnauthenticatedProtocol,
            },
            DetailFact::LastAuthenticatedMeasurement | DetailFact::CertificateTimeBasis => {
                match self.source.protocol() {
                    Protocol::Nts => AbsentFact::NotObserved,
                    Protocol::PlainNtp => AbsentFact::NotApplicable,
                }
            }
            DetailFact::SelectionRole => AbsentFact::NotEvaluated,
            DetailFact::OperatorGroup | DetailFact::IndependentVoteIdentity => {
                AbsentFact::NotEstablished
            }
            DetailFact::LastReply
            | DetailFact::LastAcceptedMeasurement
            | DetailFact::Offset
            | DetailFact::Delay
            | DetailFact::Uncertainty => AbsentFact::NotMeasured,
            DetailFact::ActualEndpoint
            | DetailFact::RequestStatus
            | DetailFact::TimeScale
            | DetailFact::LeapAnnouncement => AbsentFact::NotObserved,
        }
    }
}

/// One accepted query/filter and optional detail target; never another source owner.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SourceInspection {
    /// Accepted bounded query; rejected raw text stays only in the view.
    query: SourceQuery,
    /// Only All can be retained until qualified selection evidence exists.
    filter: SourceFilter,
    /// Stable detail identity; resolve membership afresh after any source change.
    detail: Option<SourceId>,
}

impl SourceInspection {
    /// Start with an empty all-source view and no details, without effects.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Borrow accepted query text for rendering or resetting a raw search draft.
    #[must_use]
    pub const fn query(&self) -> &SourceQuery {
        &self.query
    }

    /// Only All is admissible until a qualified role publication exists.
    #[must_use]
    pub const fn filter(&self) -> SourceFilter {
        self.filter
    }

    /// Admit a complete query before atomic replacement; source/draft state stays.
    ///
    /// # Errors
    /// Invalid raw bytes preserve the old query, detail target and visible set.
    pub fn set_query(&mut self, raw: &str) -> Result<(), InspectionError> {
        let candidate = SourceQuery::parse(raw)?;
        self.query = candidate;
        Ok(())
    }

    /// Guard a known typed filter as well as raw-selector entry points.
    ///
    /// # Errors
    /// Used/Other reject without inferring roles from UI row selection.
    pub const fn set_filter(&mut self, filter: SourceFilter) -> Result<(), InspectionError> {
        match filter {
            SourceFilter::All => {
                self.filter = filter;
                Ok(())
            }
            SourceFilter::Used | SourceFilter::Other => Err(InspectionError::RolesUnavailable),
        }
    }

    /// Admit the signed selector and then its current evidence availability.
    ///
    /// # Errors
    /// Unknown and known-unavailable filters reject without changing any state.
    pub fn set_filter_choice(&mut self, raw: i32) -> Result<(), InspectionError> {
        self.set_filter(SourceFilter::from_view(raw)?)
    }

    /// Clear only the accepted query; details/IDs/all-source owner are unchanged.
    pub fn clear_query(&mut self) {
        self.query = SourceQuery::default();
    }

    /// Project a bounded stable-ID subset, retaining the source owner's row order.
    #[must_use]
    pub fn visible_ids(&self, sources: &SourceDrafts) -> Vec<SourceId> {
        sources
            .preview()
            .into_iter()
            .filter(|source| self.query.matches(source))
            .map(DraftSource::id)
            .collect()
    }

    /// Resolve current membership independently of visible row positions.
    ///
    /// # Errors
    /// Signed-invalid/stale IDs reject instead of aliasing a filtered row index.
    pub fn source_visible(
        &self,
        sources: &SourceDrafts,
        view_id: i32,
    ) -> Result<bool, InspectionError> {
        Ok(self.query.matches(sources.source(view_id)?))
    }

    /// Open only a currently visible stable identity; retain old target on failure.
    ///
    /// # Errors
    /// Missing/stale/hidden IDs cannot silently open another displayed row.
    pub fn open_details(
        &mut self,
        sources: &SourceDrafts,
        view_id: i32,
    ) -> Result<(), InspectionError> {
        let source = sources.source(view_id)?;
        if !self.query.matches(source) {
            return Err(InspectionError::HiddenSource);
        }
        self.detail = Some(source.id());
        Ok(())
    }

    /// Resolve current checked fields again; retained hidden details are explicit.
    ///
    /// # Errors
    /// Removal/stale identity reports absence; no cached row becomes a new source.
    pub fn details(
        &self,
        sources: &SourceDrafts,
    ) -> Result<Option<SourceDetails>, InspectionError> {
        self.detail
            .map(|id| {
                let source = sources.source(id.view_id())?;
                Ok(SourceDetails {
                    source: source.clone(),
                    hidden_by_query: !self.query.matches(source),
                })
            })
            .transpose()
    }

    /// Close only the detail view, preserving query, sources and editor selection.
    pub fn close_details(&mut self) {
        self.detail = None;
    }
}
