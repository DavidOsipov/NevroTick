---
name: "implement-native-ui"
description: "Implement or review NevroTick native Windows controls, status presentation, keyboard and focus behavior, accessibility and localization."
---

# Implement native UI

Apply canonical architecture section 16.5 before the general guidance below.
During UI-M0 use fixture-only, unelevated native controls, sample-data labels and
disabled protected actions. Keep D1 display, D2 production state/policy and D3
full keyboard/Narrator/UIA work Deferred with owners/re-entry triggers; do not
implement or qualify them as UI-M0 prerequisites. Retain inherited standard
control behavior, accessible labels and the milestone's native observations.

Use canonical architecture section 16. Default candidate: Win32 common controls
through `windows`/`windows-sys` plus small safe adapter. Slint and egui/AccessKit
alternatives need demonstrated need and license/target qualification. Translating
website HTML skill does not justify webviews or toolkit changes.

Trace user task, control relationships, current status owner and effect path.
Choose native controls/UI Automation patterns expressing actual action, value and
selection semantics. Native controls alone prove no keyboard/screen-reader behavior.

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
