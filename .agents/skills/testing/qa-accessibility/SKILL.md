---
name: "qa-accessibility"
description: "Assess NevroTick native control semantics, keyboard use, Narrator behavior and accessibility error/status communication."
---

# Verify native Windows accessibility

Read [the testing contract](../CONTRACT.md).

Read [Windows evidence](../references/windows-evidence.md), architecture section 16/CHECK-22. Use actual status/source/settings/elevation/diagnostics/tray views. Native controls/source declarations do not establish application accessibility.

Inspect UIA names/roles/values/states/relationships/patterns/meaningful events. Exercise keyboard-only navigation, access keys, tab order, visible focus, Escape/cancel, errors, focus restoration after dialogs/UAC. Separately test actual Narrator speech/UIA assertions.

Check trust/freshness/uncertainty and stale/unavailable messages without color-only meaning. Preserve stable focused row order, accessible field errors and rate-limited state announcements. Critical recovery/settings actions must remain reachable without tray-only interaction.

Include contrast themes/text scaling/relevant DPI/long-text states; coordinate [visual/DPI checks](../qa-visual-dpi/SKILL.md). Record exact desktop/AT versions/observed gaps. Apply relevant WCAG criteria via WCAG2ICT guidance; no standalone certification claim.
