---
name: "implement-native-ui"
description: "Implement or review NevroTick Slint UI, Windows adapters, status presentation, keyboard and focus behavior, accessibility and localization."
---

# Implement native UI

For Slint-specific code and build work, also use
[implement-slint](../implement-slint/SKILL.md). Its conditional references cover
Rust integration, language/models, Windows lifecycle, dependencies/assets,
testing and source examples; this skill retains product interaction semantics.

Apply canonical architecture section 16.5 before the general guidance below.
During UI-M0 use a fixture-only, unelevated UI, sample-data labels and disabled
protected actions. Keep D1 display, D2 production state/policy and D3 full
keyboard/Narrator/UIA work Deferred with owners/re-entry triggers; do not implement
or qualify them as UI-M0 prerequisites. Preserve labels and the milestone's native
observations. The retained Win32 adapter is historical source; its inherited
control behavior and evidence do not transfer to the current Slint adapter.
Read current [S0.5](../../../../tasks/s0.5-ui-comparison.md) addenda and the
[GUI handoff](../../../../tasks/slint-gui-handoff.md) for the admitted migration
scope, actual source and remaining native/text-approval dependencies.

[OD-13](../../../../docs/nevrotick-owner-decisions.md#17-od-13--slint-as-the-default-ui-toolkit)
and architecture section 16 select Slint as the default and complete Win32 UI
replacement. The Slint 1.18.1 winit/software-renderer, Fluent-style configuration
is the researched starting point; inspect the actual admitted manifest and graph.
The checked-in Windows entrypoint selects Slint; the old Win32 adapter is retained
historical source. Keep `windows`/`windows-sys` in narrow OS adapters where needed.
Slint widgets are drawn by Slint, not inherited Win32 child controls; qualify actual
accessibility integration at its task trigger. The pilot's omitted accessibility
feature is no permanent product policy. David approved GPL-3.0-only for the
original GUI while Chrony-derived Core/engine remains GPL-2.0-only. Do not reopen
that choice; apply the exact grant/provenance inventory and remaining composition,
feature, security, resource, native and package checks. Source preparation and
admitted fixture work may parallel Core; retain execution holds and one shared
graph/schema owner. Translating website HTML skills adds no webview or toolkit
selection requirement.

Review Slint-generated Rust and its unsafe/lint boundary with the selected compiler
and features. The pilot's `deny(unsafe_code)` accommodation for macro-owned
allowances grants no blanket product lint or unsafe-code waiver. Record generated
unsafe scopes, handwritten adapter contracts and any narrowly justified allowance
before product admission. Keep callbacks and views on their required UI thread;
use weak component handles where ownership cycles are possible, and observe
last-window close, event-loop shutdown and actual resource release.

Trace user task, control relationships, current status owner and effect path.
Choose Slint widgets and their Windows accessibility integration to express actual
action, value and selection semantics. Verify the resulting UI Automation patterns;
widget names or source declarations alone prove no keyboard/screen-reader behavior.

Preserve these product semantics:

- Normal-user UI projects coherent broker snapshots. Show Tracking/Acquiring/
  Holdover/Conflict/Blocked, authentication, agreement, observation age, uncertainty
  and action separately. Disconnection means unavailable/stale, never synchronized.
- “Sync now” requests measurements, grants no clock step. Quit UI and stop
  synchronizing differ. Closing exits unless tray operation enabled; service has
  separate lifecycle.
- Explain elevation before UAC; show errors near relevant control. Share pure field
  rules with receiver; it independently decodes, constructs and authorizes every
  request. UI validation grants no permission.
- Keep focused source-row order stable; throttle meaningful state announcements.
  No second-by-second offset announcements or color-only trust signals.

Read [native interaction and accessibility](references/interaction.md) for control,
dialog, status, DPI or locale changes. Preserve UI-thread and handle ownership in
[Rust adapters](../implement-rust/references/rust-windows.md). Route wire/state
changes to [contracts](../implement-rust-contracts/SKILL.md).

For changed window/control, native buffer, callback or handle lifetimes, select
[Windows memory diagnostics](../../testing/references/windows-memory.md) and
repeat creation/use/destruction with observations before process exit. Account
for GDI/USER objects separately from heap and kernel-handle counts; each resource
needs its matching release owner. These Rust/FFI checks also apply to UI-M0's
fixture paths and do not activate deferred D1/D2/D3 work.

Use the shared garde/domain construction path for settings and source-edit
validation; adapter callbacks consume checked values and preserve state on error.
Do not duplicate rules in control handlers or let direct programmatic calls bypass
them. Apply [function/module enforcement](../implement-rust-contracts/references/contract-enforcement.md)
to callback inputs, transformed snapshots and returned FFI values. During UI-M0
these obligations cover the local preview only; they do not activate deferred
production Apply, persistence, service policy or other D2 behavior.

Select Rust/FFI format, compiler/Clippy and focused checks through the
[configured-tool workflows](../../testing/references/configured-tooling.md),
using the UI crate's actual Windows target and binding features. Keep dependency
admission separate from fixture behavior. During UI-M0 retain those engineering
checks and M0 native observations; apply the full accessibility guidance below
only at its re-entry trigger.

Verify changed behavior on Windows. Check UIA names/roles/patterns separately from
actual keyboard, focus and Narrator behavior. Use
[qa-accessibility](../../testing/qa-accessibility/SKILL.md) plus selected
component/end-to-end owners. Mocked snapshots/screenshots alone prove no native
accessibility. Report unavailable Windows checks; UI prototypes/WSL builds do not
complete P0/P5 accessibility acceptance.

Adapted David Osipov skill material: [licence and notices](THIRD_PARTY_NOTICES.md).
