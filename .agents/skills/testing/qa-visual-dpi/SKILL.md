---
name: "qa-visual-dpi"
description: "Test NevroTick visual usability across Windows DPI, text scaling, contrast, window sizing and multi-monitor changes."
---

# Verify native layout and DPI changes

Read [the testing contract](../CONTRACT.md).

Use actual native candidate windows; name Windows/display settings. Select representative status/source/settings/error/elevation/tray states, meaningful long/non-ASCII text; screenshots alone prove neither behavior nor accessibility.

Test at least 200% display scaling, system text scaling, contrast themes, reduced-motion preferences, small supported windows, live moves between different-DPI monitors. Verify actual preference handling during state changes, dynamic layout, control/text visibility, focus indication, usable hit targets; no clipped critical action/misleading trust status.

Inspect emitted DPI awareness/actual change handling; manifest flag/static image insufficient. Compare before/after facts with justified incidental-rasterization tolerance; preserve contractual content/controls.

Coordinate [native accessibility](../qa-accessibility/SKILL.md) for keyboard/UIA/Narrator and [localization](../qa-i18n-localization/SKILL.md) for adopted locales. Read [Windows evidence](../references/windows-evidence.md). Report tested physical/VM/display limits and retain raw captures.
