---
name: "implement-native-ui"
description: "Implement or review NevroTick native Windows controls, status presentation, keyboard and focus behavior, accessibility and localization."
---

# Implement native UI

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

Verify changed behavior on Windows. Check UIA names/roles/patterns separately from
actual keyboard, focus and Narrator behavior. Use
[qa-accessibility](../../testing/qa-accessibility/SKILL.md) plus selected
component/end-to-end owners. Mocked snapshots/screenshots alone prove no native
accessibility. Report unavailable Windows checks; UI prototypes/WSL builds do not
complete P0/P5 accessibility acceptance.

Adapted David Osipov skill material: [licence and notices](THIRD_PARTY_NOTICES.md).
