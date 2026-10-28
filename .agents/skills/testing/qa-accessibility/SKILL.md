---
name: "qa-accessibility"
description: "Assess NevroTick native control semantics, keyboard use, Narrator behavior and accessibility error/status communication."
---

# Verify native Windows accessibility

Read [the testing contract](../CONTRACT.md).

Read [Windows evidence](../references/windows-evidence.md) and the architecture's section 16/CHECK-22. Use the actual status/source/settings/elevation/diagnostics/tray views. Native controls and source declarations do not establish application accessibility.

Inspect UIA names, roles, values, states, relationships, patterns and meaningful events. Exercise keyboard-only navigation, access keys, tab order, visible focus, Escape/cancel, errors and focus restoration after dialogs/UAC. Test actual Narrator spoken output separately from UIA assertions.

Check trust/freshness/uncertainty and stale/unavailable messages without color-only meaning. Preserve stable focused row order, accessible field errors and rate-limited state announcements. Critical recovery/settings actions must remain reachable without tray-only interaction.

Include contrast themes, text scaling and relevant DPI/long-text states, coordinating with [visual/DPI checks](../qa-visual-dpi/SKILL.md). Record exact desktop/AT versions and observed gaps. Apply relevant WCAG criteria via WCAG2ICT as guidance, without claiming a standalone certification.
