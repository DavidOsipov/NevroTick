---
name: "implement-native-ui"
description: "Implement or review NevroTick native Windows controls, status presentation, keyboard and focus behavior, accessibility and localization."
---

# Implement native UI

Use canonical architecture section 16. The default candidate is Win32 common
controls through `windows`/`windows-sys` plus a small safe adapter. Slint and
egui/AccessKit are alternatives requiring a demonstrated need and license/target
qualification. Do not introduce a webview or change toolkits to translate the
website's HTML skill.

Trace the user task, control relationships, current status owner and effect path.
Choose native controls and UI Automation patterns that express the actual action,
value and selection semantics. Native controls are a foundation; their presence
does not prove keyboard or screen-reader behavior.

Preserve these product semantics:

- The normal-user UI projects coherent broker snapshots. Show Tracking/Acquiring/
  Holdover/Conflict/Blocked, authentication, agreement, observation age, uncertainty
  and action separately. Disconnection means unavailable/stale, never synchronized.
- “Sync now” requests measurements; it does not grant a clock step. Quit UI and
  stop synchronizing are distinct. Closing exits unless tray operation is enabled;
  service operation has its own lifecycle.
- Explain elevation requirements before UAC and show errors near the relevant
  control. Share pure field rules with the receiver, which independently decodes,
  constructs and authorizes every request. No UI validation grants permission.
- Keep focused source-row order stable and throttle meaningful state announcements;
  second-by-second offset announcements and color-only trust signals are unsuitable.

Read [native interaction and accessibility](references/interaction.md) for control,
dialog, status, DPI or locale changes. Preserve UI-thread and handle ownership in
[Rust adapters](../implement-rust/references/rust-windows.md). Route wire/state
changes to [contracts](../implement-rust-contracts/SKILL.md).

Verify affected behavior on Windows: UIA names/roles/patterns and actual keyboard,
focus and Narrator behavior are separate checks. Use
[qa-accessibility](../../testing/qa-accessibility/SKILL.md) plus the selected
component/end-to-end owners. Mocked snapshots or screenshots alone do not prove
native accessibility. Report unavailable Windows checks explicitly; do not claim a
UI prototype or WSL build completed P0/P5 accessibility acceptance.

Adapted David Osipov skill material: [licence and notices](THIRD_PARTY_NOTICES.md).
