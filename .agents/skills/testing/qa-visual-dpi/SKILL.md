---
name: "qa-visual-dpi"
description: "Test NevroTick visual usability across Windows DPI, text scaling, contrast, window sizing and multi-monitor changes."
---

# Verify native layout and DPI changes

Read [the testing contract](../CONTRACT.md).

Use actual native candidate windows and name Windows/display settings. Select representative status/source/settings/error/elevation/tray states and meaningful long/non-ASCII text; screenshots alone do not prove behavior or accessibility.

Test at least 200% display scaling, system text scaling, contrast themes, reduced-motion preferences, small supported windows and moving between monitors with different DPI while running. Verify actual preference handling during state changes, dynamic layout, control/text visibility, focus indication, usable hit targets and no clipped critical action or misleading trust status.

Inspect emitted DPI awareness plus actual handling of changes; a manifest flag or one static image is insufficient. Compare before/after facts using justified tolerance for incidental rasterization while preserving contractual content and controls.

Coordinate [native accessibility](../qa-accessibility/SKILL.md) for keyboard/UIA/Narrator and [localization](../qa-i18n-localization/SKILL.md) for adopted locales. Read [Windows evidence](../references/windows-evidence.md). Report tested physical/VM/display limits and retain raw captures.
