# Checked catalog and presentation contract

This describes the text-contract-2 native consumer candidate. It supersedes the
initial 184-ID staging notes. The complete inventory is `contract-source.json`:
509 active IDs, 596 stable ordinals, and 87 explicit retired/reserved meanings.
Existing ordinals are never reused. `text-contract-v1-baseline.json` remains the
unchanged version-1 semantic reference. English is the sole selected language;
the production semantic review ledger remains empty.

## Independent versions and approval

Catalogs have exactly eight root fields: `schema_version` (2),
`text_contract_version` (2), `domain` (`nevrotick.ui`), `catalog_revision`
(positive u32; current English candidate 2), `source_fingerprint`, `locale`
(`en`), `plural_profile` (`english-u64-cardinal-v1`), and `messages`.
Every object is closed. Required IDs must be complete; unknown, retired, reserved,
missing, empty and blank required values reject. Optional absent feedback is a
closed `Option<UiText>` state, never an empty translation.

File schema describes representation. Text contract version plus the exact
protected semantic fingerprint describes GUI-language compatibility. A change to
active meanings, placeholders, groups, reference wording, action context,
Unicode policy or bounds requires a new text contract version. The source gate
compares both directions against the retained baseline. The GUI build number is
a code-owned About fact and does not decide catalog compatibility; a build-only
bug fix can retain the same text contract.

A fingerprint is compatibility data, not authenticity or semantic approval.
The protected approval entry must independently bind the exact English bytes,
contract and revision. Neither signatures, translators, AI output, schemas nor a
catalog's own fields can grant it. `prepare_unapproved_json` returns only an
opaque offline review artifact; it cannot bootstrap a runtime `CatalogStore`.
Public pack/bootstrap paths use the empty release ledger even in tests. Positive
controls use separate byte-exact `cfg(test)` authority through the same decoder.

## Admission and resource bounds

The flow is retained bytes -> strict bounded JSON/CBOR front end -> private inert
`Wire` -> private checked `Catalog` -> independent approval -> `CatalogStore` ->
borrowed coherent `Snapshot` -> checked `PlainText`. Invariant-bearing types do
not derive `Deserialize`, expose mutable inners or offer unchecked constructors.

JSON length and strict UTF-8 of that same retained buffer precede decoding. A BOM
rejects; no detection, transcoding, lossy repair, normalization or stripping is
performed. Lexical span/depth/token guards precede Serde scratch allocation.
Serde owns grammar, strict escapes and exact end-of-document consumption. Seeds
compare decoded keys before decoding an already-duplicate value. No generic
`serde_json::Value`, positional-array record or trusted `size_hint` is used.
Decimal integer tokens alone are selected; fractions, exponents and coercions
reject. A valid escaped surrogate pair can still fail the separate repertoire.

CBOR admits only definite shortest-encoded u32, strict UTF-8 text and text-keyed
maps ordered by encoded key bytes. Duplicate/out-of-order keys, indefinite items,
arrays, byte strings, tags, negative numbers, floats, booleans/null and trailing
bytes reject. Both front ends use the same checked catalog and counters. No CBOR
dependency or runtime schema interpreter is required.

| Bound | Selected maximum |
| --- | ---: |
| Retained JSON / canonical CBOR | 131072 / 65536 bytes |
| Encoded JSON string span | 6144 bytes |
| Decoded text / key | 1024 / 96 UTF-8 bytes |
| Decoded text scalars / aggregate decoded bytes | 512 / 65536 |
| Object depth / fields per map / aggregate fields | 4 / 512 / 2048 |
| Temporary nodes / lexical tokens / charged work | 4096 / 8192 / 4000000 |
| One formatted output | 4096 UTF-8 bytes / 2048 UTF-16 units |
| Complete UI snapshot | 131072 UTF-8 bytes / 65536 UTF-16 units |

The native candidate fits the existing 512-map and four-million-work bounds; no
parser cap or algorithm expansion accompanies it. Counters are not observed RSS,
CPU instruction, toolkit layout or accessibility guarantees. Native fields have
additional sink-specific bounds and transformations before any OS submission.

The selected repertoire is printable ASCII plus U+00B7, U+2013 and U+2014. All
unlisted scalars reject, including LF, controls, replacement/BOM, combining marks,
bidi/default-ignorable characters and private/unassigned/noncharacters. Pinned
Unicode 15.1 / Emoji 15.1 hard denial covers pictographic/presentation emoji and
sequence components. Plain ASCII digits, `#` and `*` remain valid. Future scripts
require their own explicit code-owned profile; no new locale is implied.

Only `{minutes}` is a typed placeholder, exactly once in each `one`/`other` form;
`{{` and `}}` are literal braces. Positive `NonZeroU64`, canonical digits and unit
facts come from code. There are no arbitrary string arguments, expressions,
recursive templates, URLs, resource references or catalog-defined formatting.

## One immutable UI publication

The private catalog binding owner retains closed `UiText` meanings for 29 feedback
slots. It renders all static labels, fixed typed presets and retained feedback
against one snapshot, checks all output, then replaces private state and publishes
one `LocalizationCopy.value`. No English string comparison identifies state.
Raw drafts, IDs, accepted preferences and command authority remain separate.

`UiCopy` holds readiness, revision and one shallow reference-counted immutable
string model: 397 static rows plus 29 feedback rows, 426 total. Private
`CopySlot` constants and append checks fix shape/order. Model setters cannot mutate
an old generation. Replacing feedback constructs and validates a new model before
publication; retained readers still see the prior immutable generation. This
avoids cloning hundreds of strings at every Slint property read.

The nine maintained copy globals expose readonly aliases. Before an approved store
exists they select explicitly source-owned engineering English. Once ready, every
alias and all feedback select the same published generation. Invalid external
bytes preserve the prior UI/store; they do not enable an engineering fallback.
An impossible private copy/raw invariant uses a fixed bootstrap error/exit flow
with separate exact human-review material. Its safe UI-only Close/Cancel/Stay controls remain
usable; it cannot authorize service or clock effects.

## Source coverage, generation and native consumers

`scripts/check-ui-text.py --root <frozen-source>` independently reads actual
AppWindow instances, bindings, tooltips, accessibility labels, required controls
and typed Rust receiving paths. The gate rejects both a GUI sink lacking owned
text and an active ID lacking a real consumer. Dormant components, aliases on
other instances and pure enum/catalog helpers cannot satisfy reverse coverage.
Explicit retired/reserved records keep identities without pretending to render.

Native witnesses separately follow the selected main/host/timer/callback chain to
active-snapshot preparation, actual menu rows and `AppendMenuW` text arguments, or
checked toast title/body/action fields, DOM text nodes and guarded `Show`.
Native-only keys enter coverage only after that closed source witness. This is
bounded lexical source evidence, not Rust name resolution or a native execution,
accessibility, banner-delivery or semantic-approval result.

The tray has six code-owned command identities. Labels never choose an action.
A test toast is explicitly an example: OS request acceptance proves neither a
banner nor reading nor clock/service health. Fixed activation targets are owned
by code, and DOM construction avoids catalog XML. Observer cleanup timeout or
uncertainty keeps the app open; elapsed time does not establish completion.

`scripts/generate-ui-text.py` reads only the protected typed metadata with an
explicit reviewed SHA and pinned template. It emits bounded files in a fresh
private output directory, never the checkout, baseline or approval ledger. Text
remains inert fingerprint data; translated bytes never enter Rust/Slint code.
Generated copy files are fixed-slot rewiring recipes: preserve maintained source
fallback literals when applying them. Codegen controls retain exact v1 fixtures
and an additional v2 shallow-carrier fixture.

Source/technical validity is independent of exact human review and packaged/native
qualification. The v1 portable checkpoint passed in Root's configured lane; the
new v2/native cohort needs its own configured evidence. UIA/Narrator, installed
resource custody/tamper/races, native lifecycle and notification behavior retain
their existing owners and holds. No source gate or catalog grants service,
clock, elevation, persistence, OS-locale or arbitrary language-pack authority.
