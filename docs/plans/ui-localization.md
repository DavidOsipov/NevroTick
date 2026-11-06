# Externalized UI translations and binary catalogs

> Implementation checkpoint, 6 November 2026: the original design below is
> retained as design history. The integrated v1 pipeline has strict JSON/CBOR
> admission, typed immutable UI snapshot publication, reusable protected codegen,
> and independent forward/reverse GUI source coverage. The native consumer
> candidate advances to schema 2 / text contract 2 / English revision 2 / GUI
> build 2, with 509 active IDs and 596 stable ordinals. Its production semantic
> approval ledger remains empty; its new native source cohort requires separate
> configured/native evidence. See
> [`DESIGN.md`](../../crates/nevrotick-ui/src/localization/DESIGN.md) and
> [`contract-map.md`](../../crates/nevrotick-ui/src/localization/contract-map.md)
> for the current source contract. Earlier “no implementation”, schema-1 examples,
> and proposed paths below do not describe the current files.


**Recorded:** 5 November 2026. **Status:** design/source handoff; no implementation.
**Security amendment:** 6 November 2026 (Asia/Tbilisi); proposed controls and qualification only.
**Character refinement:** emoji-free, closed translation repertoires required by the owner; design only.
[OD-12](../nevrotick-owner-decisions.md#16-od-12--externalized-ui-translations-and-binary-catalogs)
requires separate translation files and prefers binary delivery for the Slint UI.
This refines [DEC-17/architecture §16.4](../windows-time-architecture.md#164-localization-and-presentation); [S0.5](../../tasks/s0.5-ui-comparison.md) owns preparation.
English remains initial; no extra locales, translator/provider services or user-pack imports are adopted.

## Direction and ownership

Use readable UTF-8 JSON for reviewed authoring, deterministic CBOR for delivered
catalogs, and an application-owned typed Rust catalog for lookup and formatting.
The same owner supplies Slint properties/models, Windows tray/notification text
and accessibility strings. Catalog edits require no UI composition/service changes.
The layout illustrates future files; none of this pipeline exists yet:

```text
locales/en.json                         reviewed, complete English source
locales/<adopted-locale>.json            later reviewed translation sources
generated/locales/locale.en.cbor         generated package resource
generated/locales/locale.<locale>.cbor   generated only for adopted locales
```

No packer/runtime exists; these are proposed paths, not added assets. No codec,
formatter or locale crate/version/features are selected or admitted. Keep one
catalog owner inside the UI presentation boundary, without another service or
general schema/event framework. Any interface/binding code generation uses only
an independently code-owned typed contract. Catalog JSON/CBOR, including English,
cannot define that contract. Never interpolate catalog strings or IDs into Rust,
`.slint`, shell or build instructions; they remain inert data.

This illustrative fragment shows data claims about IDs, arguments and plurals;
none is authority to change the independent contract. The complete catalog/schema remain future work:

```json
{
  "schema_version": 1,
  "domain": "nevrotick.ui",
  "catalog_revision": "example-only",
  "source_fingerprint": "example-only",
  "locale": "en",
  "plural_profile": "english-integer-cardinal-v1",
  "messages": {
    "ui.status.title": { "args": {}, "text": "Status" },
    "ui.sources.count": {
      "args": { "count": "u32" },
      "forms": { "one": "{count} source", "other": "{count} sources" }
    }
  }
}
```

## Catalog contract before implementation

Freeze version-1 field types/encoding, plural profiles, compatibility rules and
finite parser/output limits before authoring the packer/runtime loader. Example
version/profile labels freeze no details; this handoff adopts no numeric budget.

### Required JSON source encoding and closed schema

The owner's UTF-8/schema refinement fixes these representation rules before
codec selection. The original file must be **strict UTF-8 without a BOM**. Apply
the encoded-byte limit first, then validate the original retained bytes with no
encoding detection, UTF-16/UTF-32/legacy-code-page conversion, lossy replacement,
repair or sanitizing rewrite. Reject malformed/truncated UTF-8, overlong sequences,
encoded surrogate code points and values above U+10FFFF. Check the same bytes
that will be parsed and converted; an editor/extension or encoding label is no
evidence. This requirement covers English and every translation equally.

UTF-8 is the file encoding; JSON escapes remain ordinary JSON syntax. Decode
escapes strictly into Unicode scalar values: reject unpaired surrogate escapes,
accept a valid escaped surrogate pair and ordinary `\uXXXX` escapes, and compare
**decoded** object keys for duplicates before insertion. Escaped and literal
spellings must not acquire distinct identities. Valid literal U+FFFD is not itself
an encoding error; forbidden automatic replacement cannot be inferred from its
presence alone. An interior U+FEFF is not a file BOM or encoding error either;
both now reject at the separate character-admission boundary below. A valid
escaped pair proves only Unicode decoding; decoded emoji still reject.

Accept exactly one JSON object followed only by JSON whitespace. No comments,
trailing commas, JSON5 extensions, NaN/Infinity, concatenated documents or trailing
payload. Freeze a versioned **closed schema** owned with the protected typed
contract/checker, with equivalent `additionalProperties: false` behavior at every
object layer. Top-level metadata, message entries, argument maps and plural-form
maps each have exact allowed/required fields, types and finite bounds. Map keys
come from the code-owned message/argument/plural inventory; valid missing ordinary
translations remain explicit permitted omissions. Require exactly the contracted
`text` or `forms` variant; reject both, neither, unexpected nulls, missing required
fields, unknown fields/IDs/forms and implicit string/number/boolean coercions.
Integer fields require bounded decimal integer tokens without fraction/exponent
syntax. Schema version identifies the independently selected profile; it cannot
select a schema file, URL, remote `$ref`, plugin, import or new validator behavior.

Enforce encoding, JSON syntax/decoded-key uniqueness, closed structural schema and
typed semantic/placeholder rules as distinct bounded checks. A generic JSON Schema
pass alone cannot detect keys already discarded by a parser or approve meaning.
There is no unbounded generic-value parse before these guards and no CBOR output
until the complete source passes all admission. Runtime separately applies the
same semantic contract through the strict CBOR front end. The complete schema,
inventory and numeric budgets remain a source-freeze deliverable; these mandatory
encoding/closed-schema rules are selected now, not implemented validation.

Required independent source-profile cases remain planned:

| Rejected input, one fault per admitted fixture | Useful accepted neighbor |
| --- | --- |
| Invalid/overlong/truncated/surrogate/out-of-range UTF-8, or an initial BOM | Correct 1/2/3/4-byte UTF-8 and the same document without BOM |
| Lone/reversed surrogate or malformed `\u` escape | Valid escaped pair and the equivalent literal character |
| Duplicate decoded key at each object layer | One key in either literal or escaped spelling; reordered unique fields |
| Unknown/missing-required/wrong/null/coerced field or mixed variants | Exact valid record, permitted ordinary-message omission |
| Fraction/exponent token in an integer field | Admitted bounded decimal integer token |
| Second value, comments, trailing comma or trailing garbage | Same single object with permitted JSON whitespace |

These are encoding/syntax controls; passing them is not complete catalog
admission. Every decoded string must also satisfy the repertoire below.

### Emoji-free, closed character repertoires

The owner requires rejection of all emoji and unusual characters in contributed
catalogs. Make "unusual" deterministic through a **closed code-owned repertoire
per adopted locale and field**, not a font's appearance, AI judgment or a
translator's declaration. Freeze the Unicode release, exact UCD/emoji table
sources/hashes, classifier behavior and positive/negative vectors before
implementation; use the same profile during packing and runtime CBOR admission.
Updating Unicode/repertoires is a reviewed contract change, never a host-library
or catalog-selected automatic expansion. Only English is adopted; synthetic
multilingual controls admit no additional product language.

Check all decoded string keys/values, including English/metadata, after strict
JSON escape decoding and before checked construction or CBOR generation. The
runtime checks the same Unicode scalar sequences from CBOR text strings before
publication. Escaped/literal spellings, text-presentation requests and binary
delivery confer no exception. Reject the whole malformed candidate, preserving
the admitted catalog; do not strip, replace, normalize or silently sanitize text.

- **Hard emoji denial:** reject scalars classified as `Emoji_Presentation` or
  `Extended_Pictographic`, including pictographic symbols with text presentation;
  reject regional indicators, emoji modifiers, keycap U+20E3, variation selectors
  and tag characters. This covers single emoji, flags, keycaps, skin-tone,
  variation and ZWJ sequences; field/locale exceptions cannot admit them. Freeze
  precise component ranges with the table profile. Do not blanket-reject the
  `Emoji`/`Emoji_Component` properties: those include ordinary ASCII digits,
  `#` and `*`. Their plain forms may be admitted; their emoji sequences reject.
- **Forbidden non-text and hidden content:** reject private-use and unassigned
  scalars, Unicode noncharacters, U+FFFD and U+FEFF, author-supplied bidi controls,
  and control/format/default-ignorable characters except the narrow orthographic
  or multiline rules below. No raw/escaped NUL, terminal
  escape, zero-width padding or invisible control can bypass this owner. An LF
  is allowed only in a contract-designated multiline field; ordinary JSON file
  whitespace outside strings remains governed by JSON syntax.
- **Positive repertoire:** machine IDs/keys/argument names use their narrow exact
  ASCII grammar. Human text uses reviewed letters/marks from the locale's
  code-owned script repertoire plus an explicit shared punctuation/space/digit/
  unit-symbol list. Unknown symbols, decorative alphabets and characters outside
  that list reject; printable does not mean admitted. Scientific/time signs or
  punctuation such as minus/plus-minus must be explicitly listed or supplied as
  code-owned typed values. Space U+0020 is ordinary; NBSP/narrow NBSP require an
  explicit field/profile need, not a blanket space-separator allowance. No blanket
  non-ASCII or RTL-letter ban follows. The hard pictographic denial also excludes
  text-style copyright/registered/trademark signs; use admitted wording instead.
- **Orthographic exceptions remain narrow:** combining marks and ZWJ/ZWNJ may be
  admitted only by an explicitly reviewed locale/field/context rule needed for
  language spelling/shaping, with finite run/work limits. No unrestricted `Cf`
  allowance, translator exception list or arbitrary zero-width character is
  permitted. Application-owned bidi isolation for typed arguments remains
  separate from forbidden catalog-authored controls. These rules do not rewrite
  canonical source identities or bypass the existing admission of dynamic data.

Healthy controls include ordinary ASCII digits/`#`/`*`, profile-admitted accented
letters, punctuation and unit signs, and later synthetic non-emoji RTL/Indic/
supplementary-letter cases within explicit test profiles. Challenge single emoji,
flag/modifier/ZWJ/keycap/text-presentation variants in both literal and escaped
JSON, plus independently prepared equivalent CBOR, including signed malformed
resources. Challenge U+FFFD/U+FEFF, private-use/unassigned/noncharacters, forbidden
bidi/default-ignorable controls and unsupported symbols separately. An escaped
pair for an allowed supplementary letter can pass decoding/profile admission;
a pair for an emoji must reject. Required orthographic neighbors must still work.
Observe bounded rejection and preserved accepted state; these are planned cases,
not an implemented classifier or parser test. Repertoire checks do not establish
truthful wording or detect every visual confusable; semantic review remains.

### Typed message contract and admission

The UI catalog owner maintains an independently reviewed typed contract: stable ID,
semantic meaning/context, argument names/types, fixed plural profiles/forms,
required placeholder usage and multiplicity in each arm, criticality/coherent
groups, action/link bindings and approval requirements. English text is separate
untrusted content. IDs survive wording changes and are never reused for unrelated meanings.
Schema version identifies representation; catalog revision identifies reviewed
content; source fingerprint binds English definitions, semantic/warning meaning
and argument/plural contracts. Changed warning semantics must invalidate old
translations even if IDs remain. Freeze deterministic fingerprint input/digest
without circular self-hashing; it checks compatibility, not authenticity/approval.

Use the existing [strict DTO → checked domain boundary](../windows-time-architecture.md#61-validation-architecture-and-enforcement-order):
bounded source bytes → strict inert record → shared validation/private fallible
construction → immutable typed catalog → presentation consumers. No preliminary
unbounded generic JSON/CBOR value tree, runtime schema engine or unchecked setter
may bypass that owner. Apply the same definition checks during packing and load.
An immutable validated value can retain its guarantees within the unchanged
contract/context; changed bytes, representation or trusted context require admission again.

- Reject duplicate fields/message IDs before map insertion loses evidence;
  reject unknown fields/IDs, incompatible schema/domain/fingerprint, invalid
  UTF-8, trailing input and malformed or mixed message variants.
- Require exact argument names/types and valid placeholder references; translations
  may reorder arguments but cannot add/change their contract. Use a small inert
  placeholder grammar with defined escaping, not format-string code, scripts,
  templates with evaluation, URLs to fetch, or expressions supplied by a catalog.
- Plural selection uses a fixed admitted profile and typed count. Freeze English
  integer-cardinal behavior/count domain; later locales need reviewed profiles/tests.
  CBOR supplies no plural semantics; never evaluate catalog-supplied plural formulas.
- Freeze file bytes, nesting, fields/entries, UTF-8 string/ID/argument lengths,
  aggregate decoded bytes, formatted output, work and overlapping catalog memory.
  Check bounds before growing buffers or allocating from declared lengths/counts;
  checked arithmetic and bounded decoding remain necessary for packaged resources.
- Freeze deterministic CBOR ordering, definite lengths, shortest integer/length
  encodings and a closed type set; refuse unselected tags, floats, extension
  containers and alternate encodings. Generic decoding alone is insufficient.

English covers every admitted ID and needs the same technical admission and
content review. Valid ordinary omissions fall back per message; critical groups
fall back coherently as described below. Malformed/incompatible files reject
atomically, preserving the current admitted catalog. Missing arguments/invalid
entries never produce partial text. Without a valid required catalog and fallback,
use a minimal code-owned error/startup failure; never trust unchecked English JSON,
bypass MSIX integrity or search writable replacements.

## Presentation and language changes

All product text has this owner: headings, buttons, validation/status/errors,
tooltips/dialogs, tray menus, notification bodies/actions and accessible names/
descriptions. Keep machine error codes, wire/policy IDs, source identities,
OS activation/action IDs and authorization stable. Service/IPC retains typed codes
and arguments; user/server display text remains data. Localize at presentation.

The Rust catalog supplies stable Slint properties/models or exported callbacks.
Callbacks need explicit locale/revision dependencies or refresh; changing hidden
Rust state alone does not establish rebinding. UI and OS adapters share definitions
and locale policy. Do not mix `@tr`/PO lookup into a second catalog authority.

Validate a candidate catalog before publication. Commit language/catalog revision
and all affected presentation values as one UI-thread transition; reject stale
prepared results and retain the prior view on failure. Preserve editor drafts,
accepted values, focus and command availability. Update existing tray/menu adapter
text and use the chosen language for new notifications; already delivered OS
notifications are not promised to be rewritten. Keep worker-to-UI delivery and
component lifetimes under the existing Slint event-loop ownership contract.

Do not change global OS locale, service/clock state, synchronization results,
policy/input parsing, canonical numeric grammar, units, offset signs or authority.
Future date/number display needs a contract; diagnostic UTC/identifiers stay clear.
Long text, bidi and non-ASCII data need later layout/accessibility evidence.

## Delivery boundary and Slint source limits

Default catalogs are immutable resources of the complete approved MSIX,
including full English fallback, under the existing package-integrity/lifecycle
contract. Generating another locale does not require editing UI code; distributing
new resources still follows the complete-package update contract. Review,
generation, signing and package integrity establish no payload-safety exemption.
Independent language-pack installation remains a separate pending decision.
No third-party/user-pack import, download/watch-folder or integrity exception is
selected. Package-only trust applies even when UI code stays unchanged.
If imports are adopted, first define trust, compatibility, bounded admission,
replacement/recovery and failure handling for that separate boundary.

Measure complete package size, decoder code, startup/lookup cost, allocations and
peak/resident memory before claiming CBOR benefits or adding compression; CBOR
does not automatically compress strings. Readable source files need not ship.

Pinned Slint 1.18.1 research is reference evidence, not an adopted dependency pin:

- Its [translation guide](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/docs/astro/src/content/docs/guide/development/translations.mdx#L24-L105)
  supports literal `@tr`, context, positional placeholders and plurals; the
  [bundler](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/build/lib.rs#L197-L225)
  accepts PO source files and embeds tables, rather than loading CBOR packs.
- Bundled language selection is public and works [after component creation](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/core/translations.rs#L414-L462).
  PO bundling is an alternative to the application-owned CBOR boundary, not a
  simultaneous second string authority.
- The guide's runtime MO/gettext route is [Unix-gated in actual translation code](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/core/translations.rs#L223-L273)
  and [binding initialization](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/core/translations.rs#L315-L329);
  its [system gettext dependency is Unix-only](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/core/Cargo.toml#L156-L157).
  Do not claim this is a working Windows MO loader.
- Public Rust [generated-global properties/callbacks](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/slint/lib.rs#L175-L196)
  and [UI-thread delivery](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/slint/lib.rs#L163-L169)
  support the presentation boundary. No native CBOR catalog or stable public Rust
  CBOR-to-`@tr` registration hook was found in this pinned API; internal translator
  machinery is not an integration contract. Avoid [private generated-code APIs](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/slint/private_unstable_api.rs#L4-L8).

## Hostile catalogs: security amendment — 6 November 2026 (Asia/Tbilisi)

All JSON/CBOR is potentially malicious at ingress: contributor/translators' input,
English, reviewed/generated files and signed/MSIX resources alike. Preserve the
strict DTO/private-construction owner above; origin and technical validity do not
prove truthful wording. This refines existing THR-18 and CHECK-14/17/19/20/21/22/25/26,
without new acceptance IDs, crypto/signing framework or security daemon.

### One semantic contract, strict bounded front ends

JSON and CBOR front ends enforce the same typed semantic contract. File metadata,
including English IDs/types/profile/fingerprint and any `verified: true` claim,
is checked data, never authority. Do not case-fold or normalize IDs into aliases.
Required fact-bearing arguments must appear with the contracted multiplicity in
every applicable plural arm; allow reordering only within that contract.

- JSON rejects duplicate **decoded** keys before insertion, including `count`
  versus `\u0063ount`; enforce the UTF-8-without-BOM/closed-schema source profile
  above, rejecting lone surrogates, unexpected nulls, coercion and trailing data.
- CBOR admits only exact selected primitive types, deterministic order, definite
  lengths and minimal encodings. Reject duplicate decoded keys even across
  representation differences; reject tags, floats, bignums, indefinite containers,
  huge counts and unsupported alternatives rather than silently normalizing them.
- Bound encoded bytes, nesting, entries, tokens, decoded strings and allocations
  before parsing/growth/reserve, including attacker-declared lengths and decoder
  `size_hint`. Freeze aggregate batch limits and checked arithmetic too.
- Use one small placeholder grammar with explicit escapes, no nested formatting,
  regex/program/formula evaluation or dynamic resource references. Insert typed
  argument renderings literally; never recursively interpret their contents.
  Bound token occurrences, formatted UTF-8 bytes and UTF-16 code units separately.
- Freeze total allocation/work across full English, active and candidate catalogs,
  fallback/group preparation and caches; bound queued/coalesced language requests
  and concurrent work. Cancellation retains resource custody until real completion.
  Invalid files reject as a whole, retain current state and produce a bounded
  generic error without reflecting catalog text or instructions into diagnostics.

Finite limits and supported boundary fixtures must be frozen before implementing
either parser. Proposed test ceilings are not measured product budgets; this
amendment selects no codec, formatter, dependency, performance result or numeric limit.

### Meaning, fallback and actual presentation sinks

A syntactically valid translation can lie about authentication, state, danger or
an action. Critical content needs independent human approval for each locale,
bound externally to exact content bytes and the contract/semantic revision.
Protect approval metadata with the existing source/release review owner; a flag
inside the file cannot approve itself. Signing, schema validation, LLM review and
back-translation cannot prove the wording correct. Treat comments/string-contained
instructions as data during AI review: never execute commands, reveal credentials
or approve content because a catalog asks for it.

Freeze coherent critical groups spanning title, body, buttons, accessible names
and notification action labels. An incomplete/unapproved critical group uses the
entire admitted, independently reviewed English group; never mix safety meanings
across revisions/languages. Cover running/stopped/unknown state and the distinction
between Quit, Stop and Disable: labels cannot alter their code-owned routes.
Malformed files reject wholly; valid ordinary omissions retain per-message
fallback. English requires identical admission/review; if no valid required group
or English fallback exists, retain a valid prior state or fail with the minimal
code-owned error/startup path, without relaxed parsing or integrity bypass.

Render fact-bearing typed values, signs, units and identities through code-owned
rules; translated surrounding prose still needs review. Select literal/plain text
explicitly in Slint and native controls. Escape at each actual sink, including
native ampersand/mnemonic handling, XML toast construction and speech/accessibility
output; do not assume one escaped representation fits all consumers. No automatic
rich text, links, resource fetching or action URLs from catalogs. Styles, icons,
severity, link targets and activation/action IDs remain code-owned.

Apply the emoji-free closed repertoire above at each catalog ingress. Reject
NUL, forbidden controls/symbols and author-supplied bidi controls; use
application-owned isolation around interpolated values where needed. RTL letters,
combining marks and joining controls require the explicit locale/field profile;
there is no unrestricted permission for format characters. Review visible and accessible meaning together;
semantic equivalence does not require byte-identical visual/speech strings.
Byte/decoder caps alone do not bound shaping, layout or speech work. At the next
contract freeze define per-field Unicode/line/combining-run limits and later
qualify actual rendering, accessibility and notification resource budgets with
healthy profile-admitted RTL, Indic, ZWJ/ZWNJ and combining-text controls. No such field limit is
invented here, and byte caps establish no native availability result.
Native layout/Narrator evidence remains at the existing D1/D3 re-entry gates.

### Contributor bytes to package and runtime

Reuse [build/dependency/CI admission](../windows-time-development-hardening.md#3-development-threat-model-and-execution-boundaries)
and [complete-package custody](windows-msix-store.md#31-package-integrity-and-sealed-executable-content):

1. Independently protect the base validator, typed contract, adopted-locale/path
   inventory, tool pins and gate. A data-only PR supplies bytes to that protected
   checker; it cannot replace its own checker/contract. Changes to those tools
   require a separate untrusted-code lane and independent review. Existing
   [.github/workflows/security.yml](../../.github/workflows/security.yml) uses
   read-only-token PR jobs but executes candidate scripts; it does not establish
   this future data-only boundary or justify elevated candidate execution.
2. Select one bounded short-lived offline validator/converter process/job with
   read-only inputs, task-owned output and enforced CPU/RAM/process/disk/wall-time
   limits. No network, credentials, privileged token, host socket or writable shared
   executable cache. Invoke admitted absolute tool paths from a trusted working
   directory with allowlisted environment/configuration; import no PR modules,
   wrappers, plugins or configuration. Reuse existing isolation controls; a folder
   or `--offline` alone is insufficient. No new daemon or privilege follows.
3. Bind the exact candidate and allowlisted regular-file bytes, using exact Git
   blobs/modes or equivalently pinned bounded file handles. Reject symlinks,
   reparse points, hardlinks, special files and linked parent escapes. Pin/read
   bytes once and hash the retained immutable bytes; validate and generate from
   that same buffer, without reopening paths after inspection. A pathname or
   pre-read `stat` alone does not prevent replacement races.
4. Map adopted locale to fixed output/resource paths in code-owned inventory;
   filenames/locale headers cannot select paths. Reject traversal, separators,
   absolute/drive/UNC paths, ADS/reserved names and case/normalization collisions.
   Generate bindings only from the typed contract. If literal byte embedding is
   necessary, use a reviewed literal serializer; no catalog-derived source tokens,
   identifiers, shell/build instructions or hand-built source interpolation.
5. Use fresh exclusively owned output with exclusive regular temporary files;
   verify pinned inputs and complete results before atomic batch publication.
   Retain previous complete output on rejection/crash/cancellation; partial output
   is never publishable. Qualify real filesystem races/native enforcement later.
6. Treat PR artifacts/reports as bounded inert data, never executable input or
   imported configuration; safely inspect fixed schemas/paths and reject archive
   escapes. No checkout/run of PR scripts in elevated/secret jobs and no execution
   of PR outputs. Separate trust-domain caches and exact artifact identities.
7. Regenerate release resources afresh from reviewed pinned inputs with admitted
   tools; reconcile resource hashes, source/semantic revision and approved content
   with existing package/SBOM custody. Signing/submission never executes candidate
   scripts. Runtime independently admits actual approved installed CBOR resources;
   retain submitted versus Store-delivered identities and package integrity.
   Authentic signatures establish neither payload safety nor action authority.

OS-specific filesystem/isolation and actual packaged-open behavior require later
supported-Windows evidence. This source design grants no native changes or runs.

### Discriminating attacks and healthy controls

Each row requires rejection/no forbidden effect through the real receiving path
and the neighboring healthy control; planned cases are not executed evidence.

Derive each negative from an admitted fixture with one named fault, retaining
all unrelated fields/authority, so rejection tests the intended guard. Observe
bounded error classification/work and preserved state before and after a useful
accepted neighbor; rejection for an earlier unrelated fault proves less.

| Threat / attack | Required observation and healthy control | Existing checks |
| --- | --- | --- |
| English changes ID/type/profile/action or self-approves | Independent contract/approval rejects it; allowed wording and argument reordering succeed. | CHECK-19/25/26 |
| Decoded duplicate keys, malformed UTF-8/surrogates/null/trailing input | Whole-file bounded rejection; strict valid JSON converts deterministically. | CHECK-25 |
| CBOR alternate types/duplicate keys/indefinite or huge counts | Reject before allocation/effects; canonical binary admits through runtime loader. | CHECK-25/26 |
| Missing/repeated fact arguments, recursive/nested templates | Contract rejects each affected arm; literal argument text and valid plurals work. | CHECK-14/25 |
| Size/token/output/batch/overlap, shaping/speech pressure or request flood | Enforced bounds and actual completion cleanup; boundary-sized healthy work and profile-admitted RTL/Indic/ZWJ text succeed. | CHECK-17/21/22/25 |
| Emoji/unsupported symbols, hidden controls or escape/CBOR bypass | Whole-file bounded rejection at the shared repertoire guard; ordinary digits, allowed punctuation and non-emoji language controls succeed. | CHECK-14/22/25/26 |
| Syntactically valid lying warning or mixed critical group | Independent per-locale review/coherent English fallback; approved complete group works. | CHECK-14/22 |
| Markup, mnemonic, toast XML, bidi/control or action-URL injection | Actual sinks preserve literal truthful text and fixed routes; supported RTL/combining text works. | CHECK-14/17/22/25 |
| Traversal/collision/link or inspect-read replacement | No outside read/write or swapped-byte admission; ordinary pinned files publish correctly. | CHECK-19/25/26 |
| PR module/config/script, artifact or executable-cache takeover | Protected tool/isolation rejects execution; healthy data-only conversion passes. | CHECK-19/20/26 |
| Crash/cancel, tampered or signed malformed resource | Prior complete state retained and runtime rejects bad bytes; clean complete package/resource succeeds. | CHECK-14/19/25/26 |

Parser/rendering defects, semantic review mistakes and compromised authorized
reviewers remain residual risks. This amendment designs defenses and observations;
it demonstrates no implemented security, package safety or complete review result.

When the existing execution gates permit, use bounded property/fuzz cases at the
actual receiving boundary and its direct-call bypasses: chunk/escape partitions,
equivalent permitted JSON producing identical deterministic CBOR, independent
canonical byte vectors, required placeholders and stale-result publication.
Do not use only the production encoder as the expected-result oracle. A process
kill, timeout, panic or OOM is not successful ordinary payload rejection. Exercise
the actual literal sinks for no fetch/command/activation effect; real native
observations remain later work. This test design starts no held fuzz/native run
and adds no Core prerequisite. RFC 8259/RFC 8949 and Unicode guidance inform the
profile; citing a standard does not qualify a selected library or implementation.

## Existing workstream and observable exits

S0.5 source preparation records the frozen profile/bounds, message inventory and
UI/OS consumer map, package placement and exact dependency/grant proposal before
graph admission. After admission, its fixture port preserves M0-01–M0-08 and
proves meaningful catalog success/rejection/fallback through real receiving paths.
Future S5.1/S5.2 own live status/settings wording, S5.3 owns tray/notification text,
and S5.4 owns adopted-locale QA under existing CHECK-14/17/21/22 and P5 gates.

Require deterministic healthy source→binary controls; duplicate/unknown ID,
placeholder/type/plural/schema/fingerprint and resource-limit rejection preserving
valid neighboring state; complete English/missing-message fallback observations;
direct Rust/Slint consumer checks; atomic locale changes preserving drafts/focus
and updating new OS messages. Later native qualification observes long/expanded
text, bidi/non-ASCII IDs, keyboard/Narrator/UIA and actual packaged resources.
Record exact candidate/locales and skipped gaps.

The [current P1/proof-first priority](development-plan.md#7-next-executable-task-for-a-development-agent),
native execution holds, separate fuzz hold and storage pause remain. UI-M0 stays
fixture-only; [D1/D3](../windows-time-architecture.md#165-restricted-first-ui-milestone-ui-m0)
return before general-user preview/release or corresponding claims, D2 freshness
before live status, and D2 authorization/results before real settings/commands.
Do not implement/qualify D1–D3 now or make them UI-M0 completion prerequisites.
This handoff supplies no runtime, packer, extra locale, performance/native/accessibility result or release qualification.
