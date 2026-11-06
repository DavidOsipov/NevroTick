//! Checked English catalogs and typed presentation; release review remains separate.
//! Raw JSON/CBOR never supplies types, routes, validation rules or approvals.
#![forbid(unsafe_code)]

/// Protected semantic review ledger; never supplied by catalog input.
mod approvals;
/// Independent closed canonical CBOR reader and writer.
mod cbor;
/// Protected source-versioned typed message inventory.
pub mod contract;
/// One finite retained typed-feedback owner.
pub(crate) mod feedback;
/// Closed retained presenter facts, independent of catalog revision.
pub mod presentation;
pub use presentation::UiText;
/// Strict raw UTF-8 and duplicate-aware Serde source admission.
mod json;
/// Frozen character data shared by source and runtime admission.
mod repertoire;

use contract::{
    CATALOG_SCHEMA_VERSION, COUNT, DOMAIN, FINGERPRINT, Kind, PLURAL, SPECS, TEXT_CONTRACT_VERSION,
};
pub use contract::{Group, MessageId};
use garde::Validate;
pub use repertoire::UNICODE_PROFILE;
use std::{fmt, num::NonZeroU64};

/// Maximum original retained UTF-8 JSON bytes before decoding.
pub const JSON_BYTES: usize = 131_072;
/// Maximum retained CBOR bytes and canonical encoder output.
pub const CBOR_BYTES: usize = 65_536;
/// Maximum decoded UTF-8 bytes in one text value.
pub const STRING_BYTES: usize = 1_024;
/// Maximum Unicode scalar count in one text value.
pub const STRING_SCALARS: usize = 512;
/// Maximum decoded UTF-8 bytes in a map key.
pub const KEY_BYTES: usize = 96;
/// Maximum aggregate decoded key/value UTF-8 bytes per candidate.
pub const DECODED_BYTES: usize = 65_536;
/// Maximum nested value depth; the profile has four object layers.
pub const MAX_DEPTH: usize = 4;
/// Maximum cumulative map fields before another field is stored.
pub const MAX_FIELDS: usize = 2_048;
/// Maximum cumulative temporary wire nodes before another node is built.
pub const MAX_NODES: usize = 4_096;
/// Maximum fields in one temporary wire map; explicit full-inventory amendment.
pub const MAX_MAP: usize = 512;
/// Maximum charged byte/key-comparison units; scalar predicates have fixed bounds.
pub const MAX_WORK: usize = 4_000_000;
/// Maximum formatted UTF-8 bytes after every append.
pub const OUTPUT_BYTES: usize = 4_096;
/// Maximum formatted UTF-16 code units after every append.
pub const OUTPUT_UTF16: usize = 2_048;

/// Fixed redacted errors; never retain a catalog value or Serde diagnostic.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// Encoded candidate exceeds its fixed limit.
    Size,
    /// Original or CBOR text bytes are not strict UTF-8.
    Utf8,
    /// Original JSON begins with the forbidden UTF-8 BOM.
    Bom,
    /// JSON grammar or selected primitive representation is invalid.
    Json,
    /// CBOR syntax, canonical encoding or selected type is invalid.
    Cbor,
    /// A decoded key repeats before insertion.
    Duplicate,
    /// Closed fields, IDs, metadata or types do not match protected definitions.
    Schema,
    /// An aggregate, count, depth or output bound would be exceeded.
    Bounds,
    /// Decoded text violates the frozen scalar repertoire.
    Character,
    /// An inert placeholder escape, name or fact multiplicity is invalid.
    Placeholder,
    /// Caller supplied the wrong typed arguments for this message.
    Arguments,
    /// Required English message coverage is incomplete.
    IncompleteEnglish,
    /// No independent protected review binds the exact candidate.
    Approval,
}
impl fmt::Display for Error {
    /// Emit a fixed generic catalog error without reflecting payload text.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("The interface catalog could not be admitted.")
    }
}
impl std::error::Error for Error {}

/// One decoder-owned aggregate budget, never reset by nested seeds.
#[derive(Default)]
struct Budget {
    /// Cumulative retained decoded string bytes.
    decoded: usize,
    /// Cumulative temporary wire nodes.
    nodes: usize,
    /// Cumulative map entries.
    fields: usize,
    /// Charged candidate work units.
    work: usize,
    /// First bounded decoder rejection, without raw data.
    failure: Option<Error>,
}
impl Budget {
    /// Retain the first fixed rejection reason without retaining rejected input.
    ///
    /// # Errors
    /// Preserves and returns the supplied rejection, retaining only its fixed error code.
    fn remember<T>(&mut self, result: Result<T, Error>) -> Result<T, Error> {
        if let Err(error) = &result
            && self.failure.is_none()
        {
            self.failure = Some(*error);
        }
        result
    }
    /// Checked addition to one finite decoder counter; never authorizes saturation.
    ///
    /// # Errors
    /// Rejects arithmetic overflow or a cumulative counter above its supplied finite limit.
    fn charge(counter: &mut usize, amount: usize, limit: usize) -> Result<(), Error> {
        *counter = counter
            .checked_add(amount)
            .filter(|n| *n <= limit)
            .ok_or(Error::Bounds)?;
        Ok(())
    }
    /// Charge bounded decoder/key-comparison work to the shared candidate budget.
    ///
    /// # Errors
    /// Rejects cumulative charged decoder work above the fixed candidate limit.
    fn work(&mut self, n: usize) -> Result<(), Error> {
        let result = Self::charge(&mut self.work, n, MAX_WORK);
        self.remember(result)
    }
    /// Charge one wire node and reject excessive depth before construction.
    ///
    /// # Errors
    /// Rejects excessive value depth or cumulative temporary wire nodes.
    fn node(&mut self, depth: usize) -> Result<(), Error> {
        if depth > MAX_DEPTH {
            return self.remember(Err(Error::Bounds));
        }
        let result = Self::charge(&mut self.nodes, 1, MAX_NODES);
        self.remember(result)
    }
    /// Charge one map field before storing a key/value pair.
    ///
    /// # Errors
    /// Rejects cumulative map-field exhaustion before another pair is retained.
    fn field(&mut self) -> Result<(), Error> {
        let result = Self::charge(&mut self.fields, 1, MAX_FIELDS);
        self.remember(result)
    }
    /// Consume a text wire variant or render typed argument-free content; reject mismatched contracts.
    ///
    /// # Errors
    /// Rejects the wrong wire variant, invalid text or bounded output failure for this receiving operation.
    fn text(&mut self, s: &str, key: bool) -> Result<(), Error> {
        let result = (|| {
            if s.len() > if key { KEY_BYTES } else { STRING_BYTES } {
                return Err(Error::Bounds);
            }
            Self::charge(&mut self.decoded, s.len(), DECODED_BYTES)?;
            self.work(s.len())?;
            checked_text(s)
        })();
        self.remember(result)
    }
}

/// The only temporary wire tree: closed primitive set, bounded before growth.
/// This is not `serde_json::Value` and has no array/null/float/extension variant.
#[derive(Debug)]
enum Wire {
    /// Bounded unsigned integer primitive.
    Uint(u32),
    /// Argument-free inert text or the sole text-message variant.
    Text(String),
    /// Bounded unique text-key map primitive.
    Map(Vec<(String, Wire)>),
}
impl Wire {
    /// Consume only the map wire variant; reject alternate representations.
    ///
    /// # Errors
    /// Propagates a fixed setup or admission failure while preparing the discriminating control.
    fn map(self) -> Result<Vec<(String, Wire)>, Error> {
        if let Self::Map(v) = self {
            Ok(v)
        } else {
            Err(Error::Schema)
        }
    }
    /// Consume a text wire variant or render typed argument-free content; reject mismatched contracts.
    ///
    /// # Errors
    /// Rejects the wrong wire variant, invalid text or bounded output failure for this receiving operation.
    fn text(self) -> Result<String, Error> {
        if let Self::Text(v) = self {
            Ok(v)
        } else {
            Err(Error::Schema)
        }
    }
    /// Consume only the bounded unsigned-integer wire variant.
    ///
    /// # Errors
    /// Rejects every wire representation other than the selected bounded unsigned integer.
    fn uint(self) -> Result<u32, Error> {
        if let Self::Uint(v) = self {
            Ok(v)
        } else {
            Err(Error::Schema)
        }
    }
}
/// Remove an exact contracted field, rejecting absence.
///
/// # Errors
/// Returns a fixed rejection when the retained span, required field or charged work is unavailable.
fn take(fields: &mut Vec<(String, Wire)>, name: &str) -> Result<Wire, Error> {
    let i = fields
        .iter()
        .position(|(key, _)| key == name)
        .ok_or(Error::Schema)?;
    Ok(fields.remove(i).1)
}

#[derive(Validate)]
/// Raw text routed through the mandatory shared garde constructor.
struct TextInput<'a> {
    #[garde(custom(text_rule))]
    /// Untrusted borrowed text requiring shared construction.
    value: &'a str,
}
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde requires custom rules to borrow their context"
)]
/// Enforce the exact single-line repertoire and byte/scalar bounds through garde.
///
/// # Errors
/// Rejects text outside the exact shared byte, scalar and character profile.
fn text_rule(value: &str, (): &()) -> garde::Result {
    if !value.is_empty()
        && value.bytes().any(|byte| byte != b' ')
        && value.len() <= STRING_BYTES
        && value.chars().count() <= STRING_SCALARS
        && value.chars().all(repertoire::english_scalar)
    {
        Ok(())
    } else {
        Err(garde::Error::new("unsupported catalog text"))
    }
}
/// Invoke the shared garde constructor rule for decoded text.
///
/// # Errors
/// Rejects text that fails the mandatory shared garde-backed construction rule.
fn checked_text(value: &str) -> Result<(), Error> {
    TextInput { value }.validate().map_err(|_| Error::Character)
}

#[derive(Debug)]
/// Admitted inert text or exact English minute forms.
enum Message {
    /// Argument-free inert text or the sole text-message variant.
    Text(String),
    /// Two admitted plural forms sharing the checked minute argument.
    Minutes {
        /// Form used only for exactly one minute.
        one: String,
        /// Form used for every other admitted positive minute count.
        other: String,
    },
}
#[derive(Debug)]
/// Complete immutable English definitions after shared technical admission.
struct Catalog {
    /// Untrusted revision checked during construction and bound by approval.
    revision: u32,
    /// Complete immutable messages in protected inventory order.
    messages: Vec<Message>,
}
impl Catalog {
    /// Shared private construction for both independently bounded front ends.
    ///
    /// # Errors
    /// Rejects incompatible metadata, unknown or missing fields, wrong message contracts or incomplete English.
    fn checked(wire: Wire) -> Result<Self, Error> {
        let mut top = wire.map()?;
        if top.len() != 8 {
            return Err(Error::Schema);
        }
        if take(&mut top, "schema_version")?.uint()? != CATALOG_SCHEMA_VERSION {
            return Err(Error::Schema);
        }
        if take(&mut top, "text_contract_version")?.uint()? != TEXT_CONTRACT_VERSION {
            return Err(Error::Schema);
        }
        if take(&mut top, "domain")?.text()? != DOMAIN {
            return Err(Error::Schema);
        }
        let revision = take(&mut top, "catalog_revision")?.uint()?;
        if revision == 0 {
            return Err(Error::Schema);
        }
        if take(&mut top, "source_fingerprint")?.text()? != FINGERPRINT
            || take(&mut top, "locale")?.text()? != "en"
            || take(&mut top, "plural_profile")?.text()? != PLURAL
        {
            return Err(Error::Schema);
        }
        let entries = take(&mut top, "messages")?.map()?;
        if !top.is_empty() || entries.len() > COUNT {
            return Err(Error::Schema);
        }
        let mut slots: Vec<Option<Message>> = std::iter::repeat_with(|| None).take(COUNT).collect();
        for (key, entry) in entries {
            let id = MessageId::from_key(&key)?;
            if slots.get(id.index()).ok_or(Error::Schema)?.is_some() {
                return Err(Error::Duplicate);
            }
            let mut fields = entry.map()?;
            if fields.len() != 2 {
                return Err(Error::Schema);
            }
            let args = take(&mut fields, "args")?.map()?;
            let message = match id.spec()?.kind {
                Kind::Text => {
                    if !args.is_empty() {
                        return Err(Error::Schema);
                    }
                    let text = take(&mut fields, "text")?.text()?;
                    checked_text(&text)?;
                    placeholders(&text, false)?;
                    Message::Text(text)
                }
                Kind::Minutes => {
                    if args.len() != 1 || args.first().ok_or(Error::Schema)?.0 != "minutes" {
                        return Err(Error::Schema);
                    }
                    if args.into_iter().next().ok_or(Error::Schema)?.1.text()? != "minutes-u64" {
                        return Err(Error::Schema);
                    }
                    let mut forms = take(&mut fields, "forms")?.map()?;
                    if forms.len() != 2 {
                        return Err(Error::Schema);
                    }
                    let one = take(&mut forms, "one")?.text()?;
                    let other = take(&mut forms, "other")?.text()?;
                    if !forms.is_empty() {
                        return Err(Error::Schema);
                    }
                    checked_text(&one)?;
                    checked_text(&other)?;
                    placeholders(&one, true)?;
                    placeholders(&other, true)?;
                    Message::Minutes { one, other }
                }
            };
            if !fields.is_empty() {
                return Err(Error::Schema);
            }
            *slots.get_mut(id.index()).ok_or(Error::Schema)? = Some(message);
        }
        // English is always complete. No partial English can become its fallback.
        let messages = slots
            .into_iter()
            .map(|entry| entry.ok_or(Error::IncompleteEnglish))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { revision, messages })
    }
    /// Project admitted immutable values into a fresh bounded canonical-data record.
    ///
    /// # Errors
    /// Rejects any impossible missing checked message instead of publishing partial canonical data.
    fn wire(&self) -> Result<Wire, Error> {
        let entries = SPECS
            .iter()
            .map(|spec| {
                let fields = match self.messages.get(spec.id.index()).ok_or(Error::Schema)? {
                    Message::Text(text) => vec![
                        ("args".into(), Wire::Map(Vec::new())),
                        ("text".into(), Wire::Text(text.clone())),
                    ],
                    Message::Minutes { one, other } => vec![
                        (
                            "args".into(),
                            Wire::Map(vec![("minutes".into(), Wire::Text("minutes-u64".into()))]),
                        ),
                        (
                            "forms".into(),
                            Wire::Map(vec![
                                ("one".into(), Wire::Text(one.clone())),
                                ("other".into(), Wire::Text(other.clone())),
                            ]),
                        ),
                    ],
                };
                Ok((spec.key.into(), Wire::Map(fields)))
            })
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(Wire::Map(vec![
            ("schema_version".into(), Wire::Uint(CATALOG_SCHEMA_VERSION)),
            (
                "text_contract_version".into(),
                Wire::Uint(TEXT_CONTRACT_VERSION),
            ),
            ("domain".into(), Wire::Text(DOMAIN.into())),
            ("catalog_revision".into(), Wire::Uint(self.revision)),
            ("source_fingerprint".into(), Wire::Text(FINGERPRINT.into())),
            ("locale".into(), Wire::Text("en".into())),
            ("plural_profile".into(), Wire::Text(PLURAL.into())),
            ("messages".into(), Wire::Map(entries)),
        ]))
    }
}

/// One non-evaluating grammar: {{ and }} are literals; {minutes} is the sole slot.
///
/// # Errors
/// Rejects malformed braces, unknown placeholders, excessive scanning or wrong required multiplicity.
fn placeholders(text: &str, needs_minutes: bool) -> Result<(), Error> {
    let mut rest = text;
    let mut occurrences = 0usize;
    let mut tokens = 0usize;
    while !rest.is_empty() {
        tokens = tokens.checked_add(1).ok_or(Error::Bounds)?;
        if tokens > STRING_SCALARS {
            return Err(Error::Bounds);
        }
        if rest.starts_with("{{") || rest.starts_with("}}") {
            rest = rest.get(2..).ok_or(Error::Placeholder)?;
        } else if rest.starts_with("{minutes}") {
            occurrences += 1;
            rest = rest.get(9..).ok_or(Error::Placeholder)?;
        } else if rest.starts_with('{') || rest.starts_with('}') {
            return Err(Error::Placeholder);
        } else {
            let ch = rest.chars().next().ok_or(Error::Placeholder)?;
            rest = rest.get(ch.len_utf8()..).ok_or(Error::Placeholder)?;
        }
    }
    if occurrences == usize::from(needs_minutes) {
        Ok(())
    } else {
        Err(Error::Placeholder)
    }
}

/// No public unvalidated `String` constructor and no interpreted text/action API.
#[derive(Debug)]
pub struct PlainText(String);
impl PlainText {
    /// Borrow literal formatted text without interpreting markup or actions.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
/// Check UTF-8 and UTF-16 output bounds before growing formatted text.
///
/// # Errors
/// Rejects arithmetic overflow or UTF-8 and UTF-16 output exhaustion before appending.
fn append(out: &mut String, text: &str) -> Result<(), Error> {
    let len = out.len().checked_add(text.len()).ok_or(Error::Bounds)?;
    if len > OUTPUT_BYTES
        || out
            .encode_utf16()
            .count()
            .checked_add(text.encode_utf16().count())
            .ok_or(Error::Bounds)?
            > OUTPUT_UTF16
    {
        return Err(Error::Bounds);
    }
    out.push_str(text);
    Ok(())
}
/// Insert only typed minute values literally; never reparse argument output.
///
/// # Errors
/// Rejects mismatched typed arguments, invalid retained placeholders or bounded formatted output failure.
fn render(message: &Message, minutes: Option<NonZeroU64>) -> Result<PlainText, Error> {
    let (text, argument) = match (message, minutes) {
        (Message::Text(text), None) => (text.as_str(), None),
        (Message::Minutes { one, other }, Some(value)) => {
            // The argument carries its unit; prose cannot change signs/units.
            let unit = if value.get() == 1 {
                " minute"
            } else {
                " minutes"
            };
            let mut argument = value.to_string();
            argument.push_str(unit);
            (
                if value.get() == 1 {
                    one.as_str()
                } else {
                    other.as_str()
                },
                Some(argument),
            )
        }
        _ => return Err(Error::Arguments),
    };
    let mut out = String::new();
    let mut rest = text;
    while !rest.is_empty() {
        if rest.starts_with("{{") {
            append(&mut out, "{")?;
            rest = rest.get(2..).ok_or(Error::Placeholder)?;
        } else if rest.starts_with("}}") {
            append(&mut out, "}")?;
            rest = rest.get(2..).ok_or(Error::Placeholder)?;
        } else if rest.starts_with("{minutes}") {
            append(&mut out, argument.as_deref().ok_or(Error::Arguments)?)?;
            rest = rest.get(9..).ok_or(Error::Placeholder)?;
        } else {
            let ch = rest.chars().next().ok_or(Error::Placeholder)?;
            append(
                &mut out,
                rest.get(..ch.len_utf8()).ok_or(Error::Placeholder)?,
            )?;
            rest = rest.get(ch.len_utf8()..).ok_or(Error::Placeholder)?;
        }
    }
    Ok(PlainText(out))
}

/// Approved-only binary output; technical syntax alone cannot construct this.
pub struct PackedCatalog(Vec<u8>);
impl PackedCatalog {
    /// Borrow approved canonical catalog bytes; never select an output path.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.0
    }
}
/// Technically admitted review bytes with no semantic or publication authority.
pub struct UnapprovedCandidate(Vec<u8>);
impl UnapprovedCandidate {
    /// Borrow inert candidate CBOR for a bounded review artifact, never activation.
    #[must_use]
    pub fn bytes_for_review(&self) -> &[u8] {
        &self.0
    }
}
/// Prepare an offline candidate only after complete technical source admission.
///
/// # Errors
/// Rejects the whole source on syntax, schema, repertoire, placeholder or budget failure.
pub fn prepare_unapproved_json(bytes: &[u8]) -> Result<UnapprovedCandidate, Error> {
    let catalog = Catalog::checked(json::read(bytes)?)?;
    Ok(UnapprovedCandidate(cbor::encode(&catalog.wire()?)?))
}
/// Shared pack admission; only the public release path exists outside tests.
///
/// # Errors
/// Rejects technical source failures or absence of an exact protected semantic grant.
fn pack_with_authority(
    bytes: &[u8],
    authority: approvals::Authority,
) -> Result<PackedCatalog, Error> {
    let catalog = Catalog::checked(json::read(bytes)?)?;
    let encoded = cbor::encode(&catalog.wire()?)?;
    if !approvals::approved(&catalog, &encoded, authority) {
        return Err(Error::Approval);
    }
    Ok(PackedCatalog(encoded))
}
/// Admit retained source bytes and return CBOR only after protected semantic approval.
///
/// # Errors
/// Returns only a fixed catalog rejection; no partial text or catalog state is published.
pub fn pack_approved_json(bytes: &[u8]) -> Result<PackedCatalog, Error> {
    pack_with_authority(bytes, approvals::Authority::RELEASE)
}

/// One synchronous UI-owner store; no queue, cache, thread or filesystem access.
pub struct CatalogStore {
    /// Complete admitted and independently approved fallback catalog.
    english: Catalog,
    /// Optional approved English revision; None selects the English base.
    active: Option<Catalog>,
    /// Fixed receiving-path authority; no raw constructor or mutable getter exists.
    authority: approvals::Authority,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Outcome after a complete candidate is prepared without partial publication.
pub enum Update {
    /// One independently approved complete English revision is selected.
    ApprovedEnglish,
    /// The entire admitted English base is selected without mixing critical copy.
    EnglishFallback,
}
/// Borrowed coherent presentation generation; cannot outlive its immutable store borrow.
pub struct Snapshot<'a> {
    /// One immutable catalog borrowed for coherent projection.
    catalog: &'a Catalog,
}
impl CatalogStore {
    /// Admit and independently approve a complete English base; otherwise no store exists.
    ///
    /// # Errors
    /// Returns only a fixed catalog rejection; no partial text or catalog state is published.
    pub fn bootstrap_english(bytes: &[u8]) -> Result<Self, Error> {
        Self::bootstrap_with_authority(bytes, approvals::Authority::RELEASE)
    }
    /// Share the exact receiving mechanism with an explicitly test-only authority.
    ///
    /// # Errors
    /// Rejects malformed or unapproved complete English; no fallback store is constructed.
    fn bootstrap_with_authority(
        bytes: &[u8],
        authority: approvals::Authority,
    ) -> Result<Self, Error> {
        let english = Catalog::checked(cbor::read(bytes)?)?;
        let canonical = cbor::encode(&english.wire()?)?;
        if !approvals::approved(&english, &canonical, authority) {
            return Err(Error::Approval);
        }
        Ok(Self {
            english,
            active: None,
            authority,
        })
    }
    /// Stage a complete candidate, then atomically choose approved content or the English base.
    ///
    /// # Errors
    /// Returns only a fixed catalog rejection; no partial text or catalog state is published.
    pub fn replace(&mut self, bytes: &[u8]) -> Result<Update, Error> {
        // No state write precedes complete syntax/semantic/repertoire admission.
        let candidate = Catalog::checked(cbor::read(bytes)?)?;
        let canonical = cbor::encode(&candidate.wire()?)?;
        if approvals::approved(&candidate, &canonical, self.authority) {
            self.active = Some(candidate);
            Ok(Update::ApprovedEnglish)
        } else {
            // Initial English-only policy is deliberately stronger than group-only
            // approval: the entire view falls back, so no critical group can mix.
            self.active = None;
            Ok(Update::EnglishFallback)
        }
    }
    /// Borrow one coherent immutable view; replacement needs an exclusive store borrow.
    #[must_use]
    pub fn snapshot(&self) -> Snapshot<'_> {
        Snapshot {
            catalog: self.active.as_ref().unwrap_or(&self.english),
        }
    }
}
impl Snapshot<'_> {
    /// Return the revision of the single borrowed catalog snapshot.
    #[must_use]
    pub fn revision(&self) -> u32 {
        self.catalog.revision
    }
    /// Consume a text wire variant or render typed argument-free content; reject mismatched contracts.
    ///
    /// # Errors
    /// Returns only a fixed catalog rejection; no partial text or catalog state is published.
    pub fn text(&self, id: MessageId) -> Result<PlainText, Error> {
        render(
            self.catalog.messages.get(id.index()).ok_or(Error::Schema)?,
            None,
        )
    }
    /// Format the contracted minute message using a checked nonzero integer argument.
    ///
    /// # Errors
    /// Returns only a fixed catalog rejection; no partial text or catalog state is published.
    pub fn minutes(&self, id: MessageId, count: NonZeroU64) -> Result<PlainText, Error> {
        render(
            self.catalog.messages.get(id.index()).ok_or(Error::Schema)?,
            Some(count),
        )
    }
}

#[cfg(test)]
/// Unexecuted discriminating source controls and synthetic test authority.
mod tests;

/// Bound maintained fixture literals; this technical check grants no semantic authority.
///
/// # Errors
/// Rejects invalid nonempty fixture text through the same frozen repertoire owner.
pub(crate) fn check_fixture_literal(text: &str) -> Result<(), Error> {
    if text.is_empty() {
        Ok(())
    } else {
        checked_text(text)
    }
}

/// Exact test-only fixture admitted through the same production mechanism.
///
/// # Errors
/// Rejects a changed/malformed fixture instead of granting arbitrary test data.
#[cfg(test)]
pub(crate) fn test_only_projection_fixture() -> Result<CatalogStore, Error> {
    CatalogStore::bootstrap_with_authority(
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/fixtures/test-only-english.cbor"
        )),
        approvals::Authority::SYNTHETIC_FIXTURE,
    )
}
