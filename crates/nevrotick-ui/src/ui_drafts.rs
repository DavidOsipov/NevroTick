//! Bounded, process-local raw UI text and selector drafts, never domain authority.
//!
//! All variable text is admitted before allocation into one private checked type.
//! Invalid *format* within the presentation budget remains available for Submit
//! feedback; control/non-ASCII/oversized payloads never replace retained drafts.
//! Source protocol and manual-port provenance remain solely in `ProtocolPortDraft`.
//! The host owns reached page/phase/modal guards, widget resynchronization and
//! literal rendering. This module owns no toolkit, file, network or OS effects.
#![forbid(unsafe_code)]

use std::fmt;

use garde::Validate;

/// Maximum retained bytes for raw label, host, query and either helper field.
pub const MAX_RAW_TEXT_BYTES: usize = 1024;
/// Maximum retained bytes for raw source port or custom-minute text.
pub const MAX_SHORT_RAW_BYTES: usize = 64;
/// Maximum aggregate text bytes retained by this seven-field owner.
pub const MAX_RETAINED_RAW_BYTES: usize = 5248;

/// Closed stable presentation field identity, independent of translated labels.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextField {
    /// Source editor friendly-name raw input, not an accepted source field.
    SourceLabel,
    /// Source editor server identity raw input, without URI/DNS interpretation.
    SourceHost,
    /// Source editor port spelling; format validation belongs to Submit.
    SourcePort,
    /// Source-list search draft, separate from its accepted filtered projection.
    SourceQuery,
    /// Whole-minute input spelling, independent of an accepted custom interval.
    CustomMinutes,
    /// First explicit date-helper identity draft.
    RecoveryFirst,
    /// Second helper identity draft, retained even while currently unused.
    RecoverySecond,
}

impl TextField {
    /// Admit only the seven code-owned signed view field identities.
    ///
    /// # Errors
    /// Every unknown value rejects without selecting a default field.
    pub const fn from_view(raw: i32) -> Result<Self, DraftInputError> {
        match raw {
            0 => Ok(Self::SourceLabel),
            1 => Ok(Self::SourceHost),
            2 => Ok(Self::SourcePort),
            3 => Ok(Self::SourceQuery),
            4 => Ok(Self::CustomMinutes),
            5 => Ok(Self::RecoveryFirst),
            6 => Ok(Self::RecoverySecond),
            _ => Err(DraftInputError::InvalidField),
        }
    }

    /// Return the closed stable view identity; never a source or protocol ID.
    #[must_use]
    pub const fn view_id(self) -> i32 {
        match self {
            Self::SourceLabel => 0,
            Self::SourceHost => 1,
            Self::SourcePort => 2,
            Self::SourceQuery => 3,
            Self::CustomMinutes => 4,
            Self::RecoveryFirst => 5,
            Self::RecoverySecond => 6,
        }
    }

    /// Return the presentation byte budget, distinct from stricter domain caps.
    #[must_use]
    pub const fn byte_limit(self) -> usize {
        match self {
            Self::SourcePort | Self::CustomMinutes => MAX_SHORT_RAW_BYTES,
            Self::SourceLabel
            | Self::SourceHost
            | Self::SourceQuery
            | Self::RecoveryFirst
            | Self::RecoverySecond => MAX_RAW_TEXT_BYTES,
        }
    }

    /// Resolve a typed field to one in-bounds slot without accepting raw indices.
    const fn index(self) -> usize {
        match self {
            Self::SourceLabel => 0,
            Self::SourceHost => 1,
            Self::SourcePort => 2,
            Self::SourceQuery => 3,
            Self::CustomMinutes => 4,
            Self::RecoveryFirst => 5,
            Self::RecoverySecond => 6,
        }
    }
}

/// Fixed draft rejection, retaining neither raw payload nor derived user text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DraftInputError {
    /// The original UTF-8 byte length exceeds this field's presentation budget.
    TooLong,
    /// A byte is outside printable ASCII, including all controls and non-ASCII.
    Character,
    /// The signed view field is not in the independently code-owned field set.
    InvalidField,
    /// The signed interval/helper selector has no listed choice.
    InvalidChoice,
}

impl fmt::Display for DraftInputError {
    /// Emit only fixed redacted guidance, with no offending content or field value.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::TooLong => "This text is too long for the preview field.",
            Self::Character => {
                "Use printable ASCII characters without control characters or emoji."
            }
            Self::InvalidField => "Choose a listed draft field.",
            Self::InvalidChoice => "Choose a listed draft option.",
        })
    }
}

impl std::error::Error for DraftInputError {}

/// Borrowed raw field, with mandatory shared validation inside private admission.
#[derive(Validate)]
struct RawInput<'a> {
    /// Original text, bounded before garde can allocate its one-field report.
    #[garde(custom(raw_text_rule))]
    value: &'a str,
}

/// Check a bounded raw text alphabet without normalization or copying.
///
/// # Errors
/// Oversized or non-printable/non-ASCII input rejects; empty text is intentional.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde requires custom rules to borrow their context"
)]
fn raw_text_rule(value: &str, (): &()) -> garde::Result {
    if value.len() <= MAX_RAW_TEXT_BYTES && value.bytes().all(|byte| (b' '..=b'~').contains(&byte))
    {
        Ok(())
    } else {
        Err(garde::Error::new("invalid raw UI draft"))
    }
}

/// Owned admitted raw text; no external caller can construct or mutate it.
///
/// String content is deliberately redacted from `Debug` so incidental owner
/// diagnostics cannot disclose user drafts. Only `UiDrafts::get()` exposes text.
#[derive(Clone, Eq, PartialEq)]
struct RawText(String);

impl RawText {
    /// Supply a code-owned empty constant without receiving any raw input.
    fn empty() -> Self {
        Self(String::new())
    }

    /// Supply the code-owned initial custom-minute spelling, with no raw input.
    fn one() -> Self {
        Self(String::from("1"))
    }

    /// Admit field-specific original length and shared alphabet before allocation.
    ///
    /// # Errors
    /// Reject first on length, then character policy; neither path retains input.
    fn parse(field: TextField, raw: &str) -> Result<Self, DraftInputError> {
        if raw.len() > field.byte_limit() {
            return Err(DraftInputError::TooLong);
        }
        RawInput { value: raw }
            .validate()
            .map_err(|_| DraftInputError::Character)?;
        Ok(Self(raw.to_owned()))
    }

    /// Borrow immutable admitted raw text without introducing a second owner.
    fn text(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for RawText {
    /// Redact user payload rather than letting whole-owner assertions/logs echo it.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("RawText(<redacted>)")
    }
}

/// Borrowed group reset input; public fields remain untrusted until `UiDrafts::reset_groups()`.
#[derive(Clone, Copy)]
pub struct SourceFields<'a> {
    /// Complete next source-label spelling, including empty or invalid format.
    pub label: &'a str,
    /// Complete next source-host spelling, never a URI or network request.
    pub host: &'a str,
    /// Complete next port spelling; no protocol/provenance field is present.
    pub port: &'a str,
}

/// Borrowed interval-group reset; this value is not an accepted interval.
#[derive(Clone, Copy)]
pub struct IntervalFields<'a> {
    /// Raw closed selector to admit as 0..4, independent of its numeric meaning.
    pub choice: i32,
    /// Retained raw custom spelling, including a currently unused invalid format.
    pub custom_minutes: &'a str,
}

/// Borrowed helper-group reset; this value is not an accepted recovery preview.
#[derive(Clone, Copy)]
pub struct RecoveryFields<'a> {
    /// Raw closed selector to admit as 0..2.
    pub count: i32,
    /// Complete first raw helper spelling, even when the selected count is0.
    pub first: &'a str,
    /// Complete second raw spelling, even when currently unused by the preview.
    pub second: &'a str,
}

/// Fixed finite reset schema; absent groups retain their prior raw drafts.
///
/// Every supplied field, including unused custom/helper text, is admitted. The
/// schema has exactly seven possible text values, so no contributor collection,
/// duplicate key, dynamic field name or unbounded reset sequence is accepted.
#[derive(Clone, Copy, Default)]
pub struct ResetInput<'a> {
    /// Replace all three source raw fields together, without protocol state.
    pub source: Option<SourceFields<'a>>,
    /// Replace only the raw query spelling, not its accepted source projection.
    pub source_query: Option<&'a str>,
    /// Replace the interval selector and its retained raw custom spelling together.
    pub interval: Option<IntervalFields<'a>>,
    /// Replace the helper count and both retained raw fields together.
    pub recovery: Option<RecoveryFields<'a>>,
    /// Replace the raw policy selector, never an accepted profile or permission.
    pub policy_choice: Option<i32>,
}

/// Sole bounded raw UI owner; source protocol and accepted domain state are absent.
///
/// The private text type cannot be imported as an unchecked raw constructor:
/// ```compile_fail,E0603
/// use nevrotick_ui::ui_drafts::RawText;
/// let unchecked = RawText(String::from("unchecked"));
/// ```
/// Selector fields cannot bypass their fallible setters:
/// ```compile_fail,E0616
/// use nevrotick_ui::ui_drafts::UiDrafts;
/// let mut drafts = UiDrafts::default();
/// drafts.interval_choice = 99;
/// ```
/// Bounded format-invalid input remains available for separate Submit feedback:
/// ```
/// use nevrotick_ui::ui_drafts::{TextField, UiDrafts};
/// let mut drafts = UiDrafts::default();
/// drafts.replace(TextField::CustomMinutes, "1.5")?;
/// assert_eq!(drafts.get(TextField::CustomMinutes), "1.5");
/// # Ok::<(), nevrotick_ui::ui_drafts::DraftInputError>(())
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiDrafts {
    /// Exactly seven admitted raw strings, with immutable typed field indexing.
    text: [RawText; 7],
    /// Closed raw interval choice, unrelated to accepted scheduling authority.
    interval_choice: i32,
    /// Closed raw helper count, unrelated to an accepted recovery preview.
    recovery_count: i32,
    /// Closed raw policy choice, distinct from `PolicyDrafts` phase and preview.
    policy_choice: i32,
}

impl Default for UiDrafts {
    /// Introduce only known empty constants and custom "1", never external text.
    fn default() -> Self {
        let mut text = std::array::from_fn(|_| RawText::empty());
        text[TextField::CustomMinutes.index()] = RawText::one();
        Self {
            text,
            interval_choice: 0,
            recovery_count: 0,
            policy_choice: 0,
        }
    }
}

impl UiDrafts {
    /// Borrow an admitted raw field through a closed, in-bounds identity.
    #[must_use]
    pub fn get(&self, field: TextField) -> &str {
        self.text[field.index()].text()
    }

    /// Replace exactly one field after mandatory complete raw-text admission.
    ///
    /// # Errors
    /// Malformed character/oversized text leaves every field and choice unchanged.
    pub fn replace(&mut self, field: TextField, raw: &str) -> Result<(), DraftInputError> {
        let candidate = RawText::parse(field, raw)?;
        self.text[field.index()] = candidate;
        Ok(())
    }

    /// Return the already closed interval selector for raw view projection.
    #[must_use]
    pub const fn interval_choice(&self) -> i32 {
        self.interval_choice
    }

    /// Admit a raw interval selector without parsing its custom text as minutes.
    ///
    /// # Errors
    /// Values outside 0..4 reject without modifying text or either selector.
    pub fn set_interval_choice(&mut self, raw: i32) -> Result<(), DraftInputError> {
        check_interval(raw)?;
        self.interval_choice = raw;
        Ok(())
    }

    /// Return the already closed helper count for raw view projection.
    #[must_use]
    pub const fn recovery_count(&self) -> i32 {
        self.recovery_count
    }

    /// Admit a raw helper selector independently of unused/format-invalid text.
    ///
    /// # Errors
    /// Values outside 0..2 reject without modifying text or either selector.
    pub fn set_recovery_count(&mut self, raw: i32) -> Result<(), DraftInputError> {
        check_recovery(raw)?;
        self.recovery_count = raw;
        Ok(())
    }

    /// Return the already closed raw policy selector without an activation claim.
    #[must_use]
    pub const fn policy_choice(&self) -> i32 {
        self.policy_choice
    }

    /// Admit only a listed raw profile choice, leaving actual policy state separate.
    ///
    /// # Errors
    /// Values outside 0..2 reject without modifying any text or other selector.
    pub fn set_policy_choice(&mut self, raw: i32) -> Result<(), DraftInputError> {
        check_policy(raw)?;
        self.policy_choice = raw;
        Ok(())
    }

    /// Prepare every supplied group before atomically publishing the whole reset.
    ///
    /// Temporary candidates allocate at most the seven fixed field budgets;
    /// rejection drops them and retains every old field/selector. No supplied
    /// unused raw field is omitted from presentation admission. Absent groups
    /// retain their current text and selectors. No domain/OS effect occurs.
    ///
    /// # Errors
    /// Any character/byte/choice failure rejects the complete reset with no change.
    pub fn reset_groups(&mut self, input: ResetInput<'_>) -> Result<(), DraftInputError> {
        if let Some(interval) = input.interval {
            check_interval(interval.choice)?;
        }
        if let Some(recovery) = input.recovery {
            check_recovery(recovery.count)?;
        }
        if let Some(policy) = input.policy_choice {
            check_policy(policy)?;
        }
        let raw = [
            input.source.map(|value| value.label),
            input.source.map(|value| value.host),
            input.source.map(|value| value.port),
            input.source_query,
            input.interval.map(|value| value.custom_minutes),
            input.recovery.map(|value| value.first),
            input.recovery.map(|value| value.second),
        ];
        let mut candidate: [Option<RawText>; 7] = std::array::from_fn(|_| None);
        for ((field, value), prepared) in FIELDS.into_iter().zip(raw).zip(&mut candidate) {
            *prepared = value.map(|raw| RawText::parse(field, raw)).transpose()?;
        }
        // All fallible admission has ended. Each following write is in-bounds.
        for (retained, prepared) in self.text.iter_mut().zip(candidate) {
            if let Some(value) = prepared {
                *retained = value;
            }
        }
        if let Some(interval) = input.interval {
            self.interval_choice = interval.choice;
        }
        if let Some(recovery) = input.recovery {
            self.recovery_count = recovery.count;
        }
        if let Some(policy) = input.policy_choice {
            self.policy_choice = policy;
        }
        Ok(())
    }
}

/// Complete code-owned field order, independent of any callback/catalog selector.
const FIELDS: [TextField; 7] = [
    TextField::SourceLabel,
    TextField::SourceHost,
    TextField::SourcePort,
    TextField::SourceQuery,
    TextField::CustomMinutes,
    TextField::RecoveryFirst,
    TextField::RecoverySecond,
];

/// Check the closed raw interval choice without converting through a widget type.
///
/// # Errors
/// Unknown signed values reject, including negative/extreme choices.
const fn check_interval(raw: i32) -> Result<(), DraftInputError> {
    match raw {
        0..=4 => Ok(()),
        _ => Err(DraftInputError::InvalidChoice),
    }
}

/// Check the closed raw helper choice without interpreting helper strings.
///
/// # Errors
/// Unknown signed values reject without selecting Off as a fallback.
const fn check_recovery(raw: i32) -> Result<(), DraftInputError> {
    match raw {
        0..=2 => Ok(()),
        _ => Err(DraftInputError::InvalidChoice),
    }
}

/// Check the closed raw profile choice without changing an accepted policy.
///
/// # Errors
/// Unknown signed values reject without selecting a fallback profile.
const fn check_policy(raw: i32) -> Result<(), DraftInputError> {
    match raw {
        0..=2 => Ok(()),
        _ => Err(DraftInputError::InvalidChoice),
    }
}
