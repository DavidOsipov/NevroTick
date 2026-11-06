---
name: "qa-i18n-localization"
description: "Test NevroTick message catalogs, units and native layout for adopted locales while preserving wire and security semantics."
---

# Verify adopted locale semantics

Read [the testing contract](../CONTRACT.md).

Inspect actual locale policy/typed message catalog. Initial scope English; Russian only when adopted. Missing unadopted locale is no product defect; no automatic website bilingual baseline.

For OD-15, read the [catalog design](../../../../docs/plans/ui-localization.md):
translator-editable JSON and generated CBOR are separate artifacts. Check the
actual selected loader/formatter and every reached Slint/tray/notification/
accessible-name consumer; native `@tr` support proves no CBOR or Windows MO path.
Reject duplicate/unknown keys, incompatible schema/interface and malformed
placeholder/plural data atomically, retaining a useful valid catalog and English
fallback. Locale choice changes no code/action IDs, IPC or service/clock authority.

Treat every raw JSON/CBOR input as potentially malicious, including English,
generated and signed files. Exercise decoded-key duplicates before insertion,
pre-allocation limits, exact CBOR types, formatting amplification, aggregate
overlap and stale publication through real receivers. Derive each negative from
a valid fixture with one named fault and retain useful accepted neighbors.
Check required fact placeholders in each plural arm and literal handling at each
actual sink. Critical wording needs independently held exact-content approval or
whole admitted English-group fallback, including buttons and accessible variants;
a syntactically valid lie is not caught by schema validation. Include legitimate
profile-admitted RTL/Indic/ZWJ/ZWNJ/combining text beside disallowed-control cases. Planned cases
and HTML observations establish no implemented parser or native protection.

For original JSON, enforce UTF-8 without BOM before structural admission: no
encoding detection/transcoding/lossy repair. Cover UTF-16/32, legacy invalid bytes,
overlong/truncated/out-of-range UTF-8, raw/escaped lone surrogates, BOM, decoded
duplicate aliases, JSON extensions and closed-schema violations at every object
layer. Retain encoding-stage controls for valid literal/escaped Unicode and
surrogate pairs; complete catalog admission additionally needs the code-owned
repertoire. Literal/escaped U+FFFD, U+FEFF and pairs decoding to emoji reject at
character admission. Retain reordered unique fields and permitted ordinary omissions. JSON
Schema alone cannot catch duplicates erased during parsing; encoding, strict
syntax/duplicates, closed structure and typed semantics need their own guards.

Apply the owner's [emoji-free repertoire](../../../../docs/plans/ui-localization.md#emoji-free-closed-character-repertoires)
equally through JSON generation and direct CBOR load. Challenge single/text-style/
flag/keycap/modifier/ZWJ/tag/variation emoji, unsupported symbols, private-use,
unassigned/noncharacters, hidden/bidi controls and any catalog exception claim.
Keep useful ordinary digits/`#`/`*`, allowed punctuation/time signs and explicitly
profiled non-emoji language controls. Do not blanket-reject `Emoji`/`Emoji_Component`
properties or admit arbitrary `Cf`/spaces/ZWJ. Freeze exact Unicode data identity;
use independent scalar/byte fixtures. Character-list success proves neither
truthful wording nor all confusable detection; classify the intended guard's
rejection and preserve admitted state. Planned fixtures are not execution results.

Adopted locales: verify stable error/message IDs/placeholders/plural behavior/accessible names/fallback semantics. Locale must not change policy/wire identifiers, numerical units, offset sign, source identity, authorization. Explicitly label local display/UTC diagnostics.

Test long messages/negative values/non-ASCII names/bidi/mixed-script text/settings/error states through actual consumers. Check relevant field errors/truncation/assistive-technology pronunciation; labels must not obscure security-sensitive canonical identities.

Coordinate [visual/DPI](../qa-visual-dpi/SKILL.md) and [accessibility](../qa-accessibility/SKILL.md) for real native observations. Report exact locales and any catalog-only versus live-UI limits.
