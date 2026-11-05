# Slint language, drafts and presentation models

**Source baseline:** Slint 1.18.1, commit
`372cf0ee5577c3dfec309a45e7b778ba4e81b734`, inspected 5 November 2026.
Reconcile examples with the selected compiler and current canonical contracts.

Use this reference for source preparation in M0-SL-01/02 and, only when its gate
opens, the fixture migration. The [UI-M0 task](../../../../../tasks/ui-m0.md#slint-migration-handoff--5-november-2026)
owns implementation/native permission. Preserve sample labels, disabled protected
actions and D1/D2/D3 deferrals. No guidance here activates production effects.

## Contents

- [Define the presentation boundary](#define-the-presentation-boundary)
- [Keep numerical meaning in Rust](#keep-numerical-meaning-in-rust)
- [Preserve binding and draft semantics](#preserve-binding-and-draft-semantics)
- [Bound input before expensive work](#bound-input-before-expensive-work)
- [Maintain models and stable identity](#maintain-models-and-stable-identity)
- [Choose layout and checks deliberately](#choose-layout-and-checks-deliberately)
- [Keep translation separate from domain parsing](#keep-translation-separate-from-domain-parsing)

## Define the presentation boundary

Keep domain decisions in the existing checked Rust model. Export reusable Slint
components with small property/callback interfaces. Use `in` for Rust-projected
inputs, `out` for component-computed outputs and `in-out` only when both sides
intentionally write. Unqualified properties are private. Export/re-export a global
from the root file when Rust must access it; globals do not replace a controller.
Sources: [property visibility](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/docs/astro/src/content/docs/reference/language/properties.mdx)
and [generated Rust interfaces](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/slint/docs.rs).

Use callbacks to communicate intent to the Rust owner. Register required handlers
before exposing controls; a callback without a handler does nothing or returns
its type's default, which is not evidence of a successful action. Keep pure
bindings/pure callbacks free of mutation, I/O and time-dependent effects. Do not
hide validation or side effects in a formatting callback that may be evaluated
lazily. Sources: [callbacks](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/docs/astro/src/content/docs/reference/language/callbacks.mdx)
and [evaluation/purity](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/docs/astro/src/content/docs/reference/language/evaluation-and-purity.mdx).

## Keep numerical meaning in Rust

| Slint property | Verified Rust mapping | NevroTick projection rule |
| --- | --- | --- |
| `int` | `i32` | Use bounded counts/indexes only; check conversions and negative sentinels. |
| `float` | `f32` | Use for admitted visual quantities; never round-trip authoritative timestamps or high-precision offsets through it. |
| `duration` | `i64`, milliseconds | Do not treat it as nanoseconds, an NTP timestamp or a distinct clock-domain type. |
| `string` | `SharedString` | Format exact values in Rust with explicit units/sign and bounded output. |
| `[T]` | `ModelRc<T>` | Project bounded rows; this is not a validated domain container or wire format. |

Source: [pinned type mappings](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/slint/type-mappings.md).
Preserve UTC/monotonic/active-awake ownership and the positive-offset convention
in the Rust types. Distinguish unknown, stale and zero; formatted text must not
create authentication, agreement or applied-correction claims. Reuse existing
checked conversions and formatters. A change to numerical semantics still belongs
to its mathematical proof/contract owner, not a convenient UI cast.

Construct a complete bounded projection before publishing it. Generated Slint
structs have public fields; adding `@rust-attr(derive(serde...))` does not establish
private construction or permission. Retain [shared checked construction](../../implement-rust-contracts/SKILL.md)
and do not deserialize untrusted input directly into an authoritative UI model.

## Preserve binding and draft semantics

An ordinary `value: expression` binding is reactive until an imperative write
replaces it. Rust setters and user edits can therefore break a one-way text
binding. Use an intentional `<=>` link for a local editable draft, or a deliberate
controller/edit callback flow. Two-way binding propagates writes; it does not
validate them. Sources: [binding replacement](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/docs/astro/src/content/docs/reference/language/evaluation-and-purity.mdx)
and [two-way bindings](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/docs/astro/src/content/docs/reference/language/two-way-bindings.mdx).

Separate bounded raw draft text from the accepted preview value. Apply the shared
fallible constructor on Update preview; publish its checked result only on success.
Leave the accepted model unchanged on rejection, show a bounded field error and
allow correction within the draft budget. Cancel discards unaccepted edits. Fresh
fixture construction must restore fixture state; do not add persistence. Identify
the editor instance/source row so a delayed result cannot update a replaced draft.

Treat `LineEdit.edited` as a notification of user editing and `accepted` as Enter,
not proof that programmatic setters or direct callback invocations were validated.
Route all mutations through the same Rust acceptance path. `enabled`/`read-only`
restrict interaction, not authority: documented functions such as paste/cut can
still modify text programmatically. Preserve widget key processing by returning
`reject` for unhandled key events. Source: [LineEdit contract](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/docs/astro/src/content/docs/reference/std-widgets/views/lineedit.mdx).

Use declarative bindings for derived presentation. Property `changed` callbacks
run later and coalesce changes; a value changed and reverted before dispatch may
produce no callback. Never treat them as an action queue, audit history or actual
acknowledgement, and avoid feedback loops that rewrite their own dependencies.
Source: [change-callback semantics](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/docs/astro/src/content/docs/reference/language/callbacks.mdx).

## Bound input before expensive work

Reuse the current field and aggregate limits; do not invent alternate UI constants.
Specify bytes versus Unicode characters and bound draft storage, rows, errors,
clones, queued updates and transient work. Check raw input before normalization,
parsing or extra copies, then invoke the shared mandatory validation/construction.
Keep rejected payloads out of logs/errors and preserve the prior accepted state.

A guard in `edited`, a constructor or an Apply callback runs after some toolkit
work. Trace typing, paste, IME/preedit, programmatic setters and direct handlers
through text buffers, undo history, conversion and rendering allocations. Prove
where the first effective cap occurs for the selected widget/API. A post-allocation
length check establishes no pre-allocation memory bound. Do not invent a Slint
`max-length` property or claim that truncation after an oversized paste proves one.

Record any toolkit-owned allocation gap in M0-SL-02 with its owner and required
admission/observation. Choose a supported bounded integration only within that
contract; do not replace it with a custom text editor or a process-wide mitigation
without review. Keep local draft bounds distinct from D2's deferred production
Apply/IPC/policy work. Follow [implementation boundary rules](../../CONTRACT.md).

## Maintain models and stable identity

Keep a typed `Rc<VecModel<Row>>` owner on the UI thread and expose a `ModelRc<Row>`.
`VecModel::row_data` returns a cloned value; modifying it alone does not update the
model. Publish changes through the model's mutation API. For a custom `Model`,
provide its `ModelNotify` and emit correct row-changed/added/removed or reset
notifications after consistent mutation; a constant tracker is wrong for mutable
data. Source: [Model and VecModel implementations](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/core/model.rs).

| Operation | Required handling |
| --- | --- |
| Update one row | Resolve its current identity/index, mutate through the retained source model and emit/use the matching notification. |
| Replace/clear all rows | `set_vec`/`clear` issue reset; deliberately preserve or clear selection/focus according to the current UI contract. |
| Act on table selection | `current-row = -1` means none. Reject invalid/stale indexes before converting to `usize` or indexing. |
| Filter/sort | Map visual indexes through the current adapter chain or carry stable identity; never index the source vector with an unchecked visual position. |
| Change filter/comparator state | Call the adapter's `reset()` when external mutable state changes its result. |
| Edit mapped values | `MapModel` has no inverse-writeback contract. Update the retained source through its checked owner. |

Use stable source IDs for actions, not display strings or remembered positions.
Recheck editor/model revision after asynchronous work and resolve identity again
before mutation; `unfiltered_row`/`unsorted_row` also require a valid current index.
Keep a focused row's order stable under the applicable contract. Sources:
[model adapters](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/core/model/adapters.rs)
and [StandardTableView selection/sort callbacks](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/docs/astro/src/content/docs/reference/std-widgets/views/standardtableview.mdx).

## Choose layout and checks deliberately

Use layouts and explicit minimum/preferred/maximum sizes for relationships between
controls. Fixed width/height fixes that item within a layout; do not translate
browser coordinates into a rigid native layout. Preserve toolkit-provided basic
editing/labels now; full DPI/theme/accessibility qualification remains at D1/D3.
Source: [layout constraints](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/docs/astro/src/content/docs/guide/language/coding/positioning-and-layouts.mdx).

Use `ListView` with its repeated delegate for a list requiring visible-row
instantiation. Virtualization bounds neither the backing data nor sorting,
formatting, image decoding or queued work; retain explicit resource limits.
`StandardTableView` sort callbacks request sorting; the Rust model owner must
perform the admitted ordering. Source: [ListView](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/docs/astro/src/content/docs/reference/std-widgets/views/listview.mdx).

At the applicable task gate, exercise a valid neighboring edit, rejection with
accepted state unchanged, direct-call bypass, Cancel, programmatic updates after
typing, sorted/filtered row actions, invalid selection and stale delivery. Inspect
actual widget interaction separately from model calls. See [Rust integration](rust-integration.md)
for dispatch/lifecycle ownership and retain the existing M0/CHECK evidence owners.

## Keep translation separate from domain parsing

Preserve the existing English-first message/catalogue policy; this reference adds
no locale. When the applicable localization work is selected, use Slint's `@tr`
with placeholders/plurals instead of concatenated translated fragments. Match
extractor/compiler context settings. Choose bundled `.po` translations through
`with_bundled_translations` or a separately admitted runtime gettext path; neither
an upstream example nor translated markup admits a native library automatically.
See [the pinned translation guide](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/docs/astro/src/content/docs/guide/development/translations.mdx).

Translate display messages, not wire keys, stable IDs or units. Slint's
float/string conversions can use a locale-specific decimal separator; keep
configuration acceptance and exact time formatting in the existing checked Rust
owners. Test placeholders, long text and required glyphs at the relevant gate,
without silently expanding UI-M0 or claiming D1/D3 qualification.
