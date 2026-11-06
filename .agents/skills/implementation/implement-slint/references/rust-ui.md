# Rust and Slint view boundary

Read only the sections needed by the selected S0.5/P5 child. The API facts below
were checked against Slint **1.18.1**, commit
`372cf0ee5577c3dfec309a45e7b778ba4e81b734`. Future profile changes need a fresh
source check; this reference introduces no runtime dependency.

## Build and generated API

Use a checked-in `.slint` entry and reusable components, with Rust as the checked
state owner. The ordinary Rust integration uses `slint-build` in the build graph,
`slint_build::compile` in `build.rs`, and `slint::include_modules!()` in Rust.
Handle compilation errors within the repository's build/error policy. Generated
output belongs to build output, not manually maintained application source.
Match compiler/runtime versions and selected features; inspect build scripts,
proc macros, embedded fonts/images, styles and resource paths before execution.

Export the root component, intended data structs and public callbacks/properties.
Use generated `get_*`, `set_*`, `on_*` and `invoke_*` APIs, with the actual typed
signatures. Slint dashes become Rust underscores. Use explicit property direction
(`in`, `out`, `in-out`), reactive property expressions and `<=>` two-way links deliberately;
shared reactive state is not an authorization boundary. Globals are accessed
through the correct component instance, not a new service/state singleton. A
global handle can also retain its component; use its supported weak handle when
capturing it across callbacks. An imperative assignment replaces a property's
binding: do not mix direct setters and a reactive owner accidentally.

Source: [Rust integration and generated interface](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/slint/lib.rs#L55-L172),
[build compiler](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/build/lib.rs#L461-L516).

## Thread and lifetime ownership

- Create components and run the event loop on its owning thread, ordinarily the
  main thread. UI handles, `Rc<VecModel<_>>` and their property/model mutations
  remain there. A mutex or unsafe `Send` implementation is not a repair for a
  wrong-thread UI call.
- A component handle is a strong reference. Use `as_weak()` for captures in its
  own callbacks; `clone_strong()` deliberately extends lifetime and can form a
  component → callback → component cycle. Retain an explicit session owner for
  the root, models, callback state and any timer/worker resources.
- `Weak::upgrade()` returns `None` after destruction **or on another thread**.
  Handle that outcome without panic or recreating the window. A successful weak
  upgrade is no evidence of current service data or command authority.
- If a reached, authorized operation needs a worker, send bounded plain `Send`
  data through `invoke_from_event_loop` or `Weak::upgrade_in_event_loop`. Both
  closures require `Send + 'static`; capturing the UI-owned `Rc` inside one
  still violates the boundary. Obtain/update the bound model on the UI thread.
  Handle enqueue errors; a dropped component can cause queued weak work to be
  skipped. Queueing is not acknowledgement that a visible change occurred.
- Bound/coalesce pending updates and keep one lifecycle owner through shutdown.
  Cancel/join owned workers at the real completion point. Use an owned UI timer
  only for a concrete fixture/presentation need; do not add one-second polling or
  permanent animation merely to refresh static sample data.

Source: [strong-handle/callback and threading rules](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/slint/lib.rs#L154-L172),
[Weak and event-loop dispatch](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/core/api.rs#L1123-L1288).

## Models and atomic preview edits

Use Slint presentation rows as projections of accepted domain values. Keep the
bound `ModelRc`/`Rc<VecModel<_>>` alive under the UI owner. Import `slint::Model`
when calling its trait methods. `row_data()` returns a cloned row: editing that
clone alone does not notify the view. `VecModel::set_row_data`, `push`, `remove`
and `set_vec` perform their own notifications. A custom `Model` needs a correct
`model_tracker` and notifications matching its actual mutation; mutating a backing
vector without notifying Slint can leave a stale view.

Resolve a row to the current stable source identity before applying an edit.
Filtering/reordering invalidates captured row indices. `set_row_data` silently
does nothing for an out-of-range index in this researched version, so it cannot
establish that an update succeeded. Check selection/domain update outcomes and
project the resulting accepted state, retaining useful keyboard/focus order.

The edit path is raw UI strings → existing checked Rust parse/update → accepted
domain state → presentation row/model. Keep raw text and field errors separate
from the accepted preview. An invalid edit preserves accepted rows, selections
and draft text; Cancel discards unaccepted edits and restart reloads fixtures.
Do not replace Rust validation with a `LineEdit` input hint or JavaScript copy.
Disabled/read-only widgets also remain insufficient admission guards: Slint's
programmatic edit methods can act regardless of those interaction flags.
Current prototype ASCII/host/port rules belong to its scoped model; extending
protocol/profile fields requires their enforcing Rust owner.

Custom intervals accept **whole minutes ≥1**. Parse decimal integer input with
bounded length, explicit receiving-type limits and checked unit conversion.
Exercise zero, negatives, fractions, exponent/nonfinite strings and overflow.
Float parsing followed by `as i32` can truncate/saturate input and is unsuitable.
Set a representation/work bound through the applicable contract; invent no
product maximum merely to match a widget type. UI callbacks and direct model
calls must reject the same forbidden neighbors without changing accepted state.

Source: [Model contract](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/core/model.rs#L118-L220),
[VecModel mutation/notifications](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/core/model.rs#L511-L654).
Repository authority: [HTML design instructions](../../../../../docs/nevrotick-design-v0.4/AGENTS.md)
and [Rust contracts](../../implement-rust-contracts/SKILL.md).

## Composition and navigation

Use layout containers such as `VerticalLayout`, `HorizontalLayout` and
`GridLayout`; provide scrolling/minimum sizing for reached content. Fixed desktop
coordinates copied from the HTML do not adapt to source names, field errors or
window resizing. Prefer semantic standard widgets where suitable; custom cards
and tooltips still need meaningful labels and keyboard/focus behavior at the
applicable stage. Model lists separately from view composition; large repeated
rows need a reviewed bounded/virtualized view rather than unconstrained instances.
In 1.18.1, `ScrollView` instantiates all children; `ListView` instantiates visible
rows. Prefer current `content-*` extent properties over deprecated `viewport-*`
aliases. `ScrollView.enabled` affects appearance rather than disabling scrolling;
do not use it as a modal-interaction or authorization guard.

Source: [bindings](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/docs/astro/src/content/docs/reference/language/bindings.mdx#L11-L31),
[ScrollView](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/docs/astro/src/content/docs/reference/std-widgets/views/scrollview.mdx#L32-L103),
[ListView](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/docs/astro/src/content/docs/reference/std-widgets/views/listview.mdx#L9-L15),
[LineEdit programmatic behavior](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/docs/astro/src/content/docs/reference/std-widgets/views/lineedit.mdx#L87-L159).

The v0.4 design starts in **Simple**: Status, Settings and About. **Power user**
adds Sources, Statistics and Diagnostics. Settings/About remain reachable in both.
When switching from a Power-only page to Simple, show Status and remember the
Power page for return; preserve shared pages and transient state. Guard mode
changes while a dialog owns an in-progress edit. Port the pages authorized by the
current slice: the design's complete navigation is not a mandate to implement
every production workflow in UI-M0.

Retain persistent sample-data context in reached pages/dialogs. “Sync now” and
protected service/ownership/settings commands remain unavailable in the native
fixture slice. Keep the HTML's external scenario harness separate from product
navigation; rich mock outcomes are not real UAC, service or notification results.
Use the original AVIF assets and rights as reference; consult the
[asset/profile caveat](windows-profile.md#resolve-grants-and-assets-explicitly) before choosing how
the native UI consumes them.

## Translation catalogs

[OD-15's design](../../../../../docs/plans/ui-localization.md) selects readable
JSON sources → validated CBOR resources → one application-owned typed Rust
catalog feeding Slint properties/models and OS tray/notification adapters.
Code generation uses an independently code-owned typed contract, never catalog
text/IDs as Rust or `.slint` source. Every JSON/CBOR input, including English and
generated/signed files, requires bounded admission. Metadata cannot choose its
own IDs, argument types, plural logic, actions, criticality or approval. Keep
labels, errors, help and accessible names under the same catalog; preserve stable
control-flow/error/action IDs, required fact-bearing arguments and typed values.
Select plain text and escape each actual sink; catalog text supplies no markup,
resource fetch or URI/action target. Critical meaning requires independent
exact-content review or complete admitted English-group fallback. Syntax,
signing and an LLM score cannot establish truthful wording.

Enforce the owner's closed emoji-free locale/field repertoire on decoded
JSON/CBOR strings before checked construction; valid UTF-8/escape syntax alone
does not admit a character. Use pinned Unicode tables and hard emoji denial,
preserve ordinary digits/`#`/`*` and allow joining controls only through explicit
orthographic profiles. A text-presentation selector, escaped surrogate pair or
catalog exception claim cannot bypass the character guard. See the shared plan;
do not reproduce a conflicting widget-only regex or strip forbidden content.

Slint's `@tr`/PO bundling is a different built-in path; it does not load CBOR.
In the inspected 1.18.1 source, actual system gettext/MO lookup is Unix-gated,
and no public stable Rust custom-CBOR translator registration is established.
Do not borrow the C++ translator hook or private Rust APIs. A language change
needs an explicit validated UI-thread presentation refresh, retaining drafts,
focus and service state. Already delivered Windows notifications need not change
retroactively. No additional product locale is adopted by defining this mechanism.

See [source evidence](../SOURCES.md#translation-catalogs) before changing the
framework route, version or formatter/codec profile. The catalog mechanism does
not introduce new native work or activate deferred locale/accessibility campaigns.

## Verify the reached path

Check healthy edit → invalid edit preserving accepted state, direct binding/model
entrypoints, Cancel, reset/restart, stable selection and owned teardown. Use
fixtures with nonempty rows and accepted values so reject-everything/no-op code
cannot pass. A renderer-less or testing backend can establish its documented
scope after separate test-graph admission; it cannot prove real Windows rendering,
UIA/Narrator, theme/DPI, resources or package behavior. Do not add internal Slint
testing crates to the product graph merely because upstream tests use them.

Restore D2 freshness/generation/policy/result contracts before live UI state or
commands, and D1/D3 at their existing triggers. Preserve the distinction between
authentication, agreement, evidence ages, requested checks and applied correction.
Report actual test/backend/profile identities; an uncompiled illustrative example
or a successful HTML test is not Rust/Slint execution evidence.
