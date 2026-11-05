# Windows Slint profile and evidence

Read this reference when proposing the Windows graph, changing renderer/assets,
or preparing candidate-specific accessibility, lifecycle or resource checks.
[OD-11](../../../../../docs/nevrotick-owner-decisions.md#15-od-11--replace-the-win32-ui-with-slint)
already selects complete Slint replacement; do not reopen toolkit selection.
The [existing S0.5 task](../../../../../tasks/s0.5-ui-comparison.md) controls
readiness, integration ownership and current execution holds.

## Prepare the profile before dependency execution

Record the proposed version, source/lock identity, `x86_64-pc-windows-msvc`,
toolchain, one backend/renderer, runtime/build features, font/image strategy and
generated/resource inputs in the existing task. [Pinned sources](../SOURCES.md)
describe Slint 1.18.1 at `372cf0ee5577c3dfec309a45e7b778ba4e81b734`, requiring
Rust 1.92. That research baseline admits no dependency or toolchain change.

For that version, a source-only software-profile proposal can disable runtime
defaults and select `std`, `backend-winit`, `renderer-software`, `accessibility`
and `compat-1-18`. This is a candidate to evaluate, not a required product graph
or a licence/resource result. State why the renderer fits the current slice;
do not add GPU/Qt alternatives or a fallback renderer without an actual need.

Declare `slint-build` separately at the matching exact version, including its
host graph and compatibility feature. Its defaults include software preprocessing
even when runtime defaults are disabled. For an explicit `EmbedFiles` proposal,
build defaults can be disabled with `compat-1-18`; preprocessing through
`EmbedForSoftwareRenderer` additionally needs the build `renderer-software`
feature. Resource mode and both feature sets must agree with the actual build
configuration. These feature names are version-specific; inspect a changed pin.

Exclude default `system-tray` from the fixture profile. Do not enable live-preview,
`mcp`, `system-testing`, unstable APIs or broad all-features builds as ordinary
integration defaults. Runtime environment selection can fall back among compiled
implementations; it does not prove a single renderer or prevent graph expansion.

One integration owner serializes Cargo manifests/lock, policy/verifier updates
and shared schemas. Apply [dependency admission](../../../environment/dependency-updates/SKILL.md)
to runtime, compiler/build scripts, proc macros, generated code, native inputs,
fonts/assets and transitives before their execution. Reuse existing inventories
and CHECK-20/26 evidence; a locked graph alone supplies no execution review.

## Resolve grants and assets explicitly

Record the concrete Slint grant and distribution obligations alongside the exact
runtime/build graph. Framework grants differ from MIT docs/examples and from
third-party inputs. Preserve Chrony's GPL-2.0-only provenance; neither selecting
Slint's royalty-free option nor placing a UI in a separate process establishes
compatibility. Supply source evidence and the bounded unresolved question to the
existing licence owner. Routine source preparation needs no toolkit reapproval.

Choose bounded packaged resource locations/embedding, font fallback and image
decoding deliberately. Absolute development paths are unsuitable package inputs.
The [source discrepancy](../SOURCES.md#imageavif-discrepancies) matters for the
[AVIF brand kit](../../../../../docs/NevroTick-AVIF-kit-v1/README.md): extra image
formats alone do not establish native AVIF decoding. Do not silently enable a
native decoder, install codecs or convert assets. Record any selected derivative
or decoder with its graph, rights, source metadata and original-asset custody;
browser rendering of the HTML reference supplies no native decode evidence.

## Qualify the admitted candidate at its current phase

Keep fixture-only M0-01–M0-08: persistent sample label, disabled protected actions,
accepted local edits, rejected replacement preserving accepted state, Cancel and
restart reset. Binding callbacks still enter shared checked construction;
Slint types/widget limits confer no validation or live-command authority.
The current native Windows stop, separate fuzz hold and storage pause remain.
Source review can proceed; no instruction here authorizes a Windows run.

When the existing task admits native work, freeze source/graph/binary/helper/
fixture identities and use a bounded ordinary-user candidate. Observe launch,
pages/editor, resize, minimize/restore and Close. Trace timers, callback captures,
component handles, renderer context/surfaces and task owners through normal/error
teardown. Hide/minimize need not release native resources. Observe repeated
creation/use/destruction and release before process exit using the reached
[Windows memory diagnostics](../../../testing/references/windows-memory.md),
retaining unsupported or missing coverage. Compare footprint/startup/idle and
resource observations with unchanged architecture §17 targets and CHECK-21.

Accessibility enables a Winit/AccessKit path on Windows; it proves no complete
UIA tree, useful action pattern, focus order or Narrator experience. At the
applicable re-entry trigger, inspect actual names/roles/states/patterns and
separately exercise keyboard/focus/Narrator with
[accessibility QA](../../../testing/qa-accessibility/SKILL.md). Follow the
[Windows evidence guide](../../../testing/references/windows-evidence.md).

Retain D1 display and D3 full accessibility deferrals until preview/release or
earlier corresponding claims. Restore D2 freshness/generation/authority work
before any live status/settings/effect. The fixture port cannot close P5 or full
CHECK-21/22. Old Win32/egui passes and HTML checks do not transfer to Slint.
At qualified cutover inventory/remove obsolete product UI entrypoints and UI-only
graph/harness paths; keep independently needed narrow Windows OS adapters.
