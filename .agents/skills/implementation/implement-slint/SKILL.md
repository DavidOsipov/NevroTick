---
name: "implement-slint"
description: "Implement, review or debug NevroTick Slint .slint components and Rust adapters: build integration, properties and callbacks, models, UI-thread ownership, Windows lifecycle, resources, testing and dependency admission. Use for Slint-specific work alongside implement-native-ui."
---

# Implement Slint

Read [implement-native-ui](../implement-native-ui/SKILL.md) for product semantics,
then use this leaf for Slint mechanisms. Apply
[OD-13](../../../../docs/nevrotick-owner-decisions.md#17-od-13--slint-as-the-default-ui-toolkit),
[architecture §16.5](../../../../docs/windows-time-architecture.md#165-restricted-first-ui-milestone-ui-m0)
and the current [S0.5 task](../../../../tasks/s0.5-ui-comparison.md) and
[Slint GUI handoff](../../../../tasks/slint-gui-handoff.md) before implementing
or selecting checks. [UI-M0](../../../../tasks/ui-m0.md) retains the baseline
fixture boundary and deferred owners. Toolkit selection does not admit a
dependency graph, lift an execution hold or complete a native observation.

The researched starting point is **Slint 1.18.1, winit/software rendering and
Fluent style**. References pin upstream commit
`372cf0ee5577c3dfec309a45e7b778ba4e81b734`, checked on 5 November 2026.
Reconcile it with the actual task, manifests, lockfile and generated code; do not
copy newer rolling documentation into an older candidate. Source links establish
API evidence; current task receipts and actual source establish the admitted
NevroTick configuration and its narrower completed checks. The Windows entrypoint
now selects Slint; the retained Win32 adapter is historical source. David approved
GPL-3.0-only for the original GUI while Chrony-derived Core/engine remains
GPL-2.0-only. Keep exact graph/composition and native qualification separate.
Use [design v0.4](../../../../docs/nevrotick-design-v0.4/README.md), its current
audits and brand kit for composition within the admitted task scope.

## Keep the active milestone explicit

- In UI-M0, retain ordinary-user execution, fixture-only Status/Sources/Settings,
  persistent sample-data labels, checked in-memory preview edits and disabled
  protected actions. Add no live network/DNS, production IPC, persistence,
  service control, elevation or clock effects.
- Keep D1 display/theme/contrast/text/DPI, D2 production state/reconnect/policy/
  administrative-result handling and D3 full keyboard/Narrator/UIA **Deferred**
  until their existing task triggers. Preserve basic widget keyboard/editing
  behavior and meaningful labels now. Reference coverage of a future feature
  does not activate that feature.
- The M0-SL-01–05 chain records the original migration gates. Current S0.5
  addenda and the GUI handoff own the implemented Slint adapter, portable/native
  scope and remaining dependencies. Retain historical receipts at their exact
  source/configuration; no portable result closes Windows qualification.

## Load the reference matching the change

Read only the task-relevant canonical sections and reference files. Use heading
or symbol searches for long documents; do not recursively expand every link or
load the whole architecture and reference corpus for a bounded Slint question.

| Work | Read |
| --- | --- |
| `build.rs`, generated modules, callbacks, weak handles or async delivery | [Rust integration and ownership](references/rust-integration.md) |
| `.slint` interfaces, editable drafts, bindings, lists, sorting or layouts | [Language and models](references/language-and-models.md) |
| Native windows, HWND access, winit hooks, close/hide/tray or later UIA/DPI | [Windows integration](references/windows-integration.md) |
| Dependency/features/licence admission, fonts, AVIF assets or embedding | [Dependencies and assets](references/dependencies-and-assets.md) |
| Behavioral tests, visual checks, native evidence, developer MCP or performance | [Testing and performance](references/testing-and-performance.md) |
| Find a relevant example, real application or current primary source | [Examples and source research](references/examples-and-sources.md) |
| Detailed checked-editor, navigation or catalog boundary | [Rust and view boundary](references/rust-ui.md) |
| Explicit Windows feature/resource profile and lifecycle evidence | [Windows profile](references/windows-profile.md) and [pinned local source register](SOURCES.md) |
| Translation catalogs, language changes or binary resources | [OD-15 localization design](../../../../docs/plans/ui-localization.md) and the Rust/view reference; preserve hostile-catalog admission and exact-content approval |

## Procedure

1. **Freeze the changed boundary.** Inspect current source, dirty state, target,
   selected runtime/build/developer features and the existing acceptance owner.
   Trace fixture/domain value → presentation model → widget → callback → checked
   Rust operation → new projection. Name allowed effects and the rejection path.
   Reuse current task records; create no parallel qualification process.
2. **Resolve build and graph assumptions first.** Match `slint` and `slint-build`
   exact admitted versions. Inspect host build scripts, proc macros, target
   backends/renderers, fonts/codecs and native payloads separately. Keep licence
   compatibility and generated unsafe/lint review explicit. Never fix compilation
   by silently relaxing the workspace's safety policy, enabling all features or
   selecting another renderer/toolkit.
3. **Keep domain authority in Rust.** Use `.slint` for presentation and user
   intent. Its generated structs, `int`/`float` properties, models and globals
   are not checked domain types or trusted IPC contracts. Keep precise time
   values in typed Rust and project bounded display values with explicit units.
   Reuse [mandatory checked construction](../implement-rust-contracts/SKILL.md);
   test raw/programmatic calls as well as user interaction.
4. **Separate drafts from accepted state.** Bind an editor to a local draft,
   validate through the shared owner and publish only accepted projections.
   Preserve prior accepted state on error and discard unaccepted text on Cancel.
   Account for one-way bindings being replaced by assignments, deferred/coalesced
   change callbacks, and raw text allocation before an edit handler runs.
   Widget `enabled`/`read-only` settings are interaction aids, not validation.
5. **Keep models and components on the UI thread.** Retain the model owner,
   mutate through notifying APIs and address rows by stable identity after
   filtering/sorting. Check signed indexes before conversion. Bound retained
   rows, strings, images and pending work; visible-row virtualization is not a
   memory or queue limit. Preserve focus/selection when updating a projection.
6. **Make lifecycle observable.** Use weak component captures where callbacks
   could retain their owner; upgrade on the UI thread and handle disappearance
   and scheduling errors. A successfully queued update is not an applied update.
   Keep callbacks short and preserve the engine-only Tokio boundary. Specify
   close, hide, tray, quit, timer/task cancellation and actual resource release
   without creating an unowned second event loop.
7. **Verify the applicable layer.** Use the existing
   [testing contract](../../testing/CONTRACT.md). Check independent valid/invalid
   editor cases, Cancel/reset, correct row identity and lifecycle as relevant.
   Separate model tests, no-pixel behavioral tests, rendered visual checks and
   native Windows observations. Existing Win32 child-control automation does not
   automatically reach Slint widgets. Report exact executed scope and gaps.

## Stop common mistakes at review

- Do not pass authoritative NTP/UTC/monotonic values through Slint `float`:
  it maps to `f32`; `int` maps to `i32`, and `duration` uses milliseconds.
- Do not assume separately instantiated windows and tray components share a
  Slint global. Use an explicit Rust-owned state/projection boundary.
- Do not add Tokio or `async_compat` to the UI because an upstream example uses
  them. Slint's local future executor does not supply a Tokio runtime.
- Check the included tray API before proposing another tray dependency. Optional
  tray work still needs its later lifecycle and Explorer-recovery evidence.
- Do not assume `image-default-formats` provides an AVIF decoder. Read the exact
  codec/feature evidence before choosing brand-kit derivatives or a decoder.
- Keep `mcp`, `system-testing` and `live-preview` out of the product graph.
  Research through connected MCPs does not authorize an application control
  server. Any development fixture using it needs its own admitted scope.
- Do not claim Windows accessibility, native teardown, performance, package
  integrity or licence clearance from an example, screenshot or successful build.

At handoff, identify changed files/behavior, source and feature identity,
independent checks actually run, remaining M0/CHECK obligations and deferred
owners. Follow [dependency admission](../../environment/dependency-updates/SKILL.md)
for graph changes and [Windows memory diagnostics](../../testing/references/windows-memory.md)
for changed native ownership. This original local skill imports no upstream
implementation or licence grant; references retain their own owners.
