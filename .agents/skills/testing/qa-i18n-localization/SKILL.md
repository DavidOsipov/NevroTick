---
name: "qa-i18n-localization"
description: "Test NevroTick message catalogs, units and native layout for adopted locales while preserving wire and security semantics."
---

# Verify adopted locale semantics

Read [the testing contract](../CONTRACT.md).

Inspect actual locale policy/typed message catalog. Initial scope English; Russian only when adopted. Missing unadopted locale is no product defect; no automatic website bilingual baseline.

Adopted locales: verify stable error/message IDs/placeholders/plural behavior/accessible names/fallback semantics. Locale must not change policy/wire identifiers, numerical units, offset sign, source identity, authorization. Explicitly label local display/UTC diagnostics.

Test long messages/negative values/non-ASCII names/bidi/mixed-script text/settings/error states through actual consumers. Check relevant field errors/truncation/assistive-technology pronunciation; labels must not obscure security-sensitive canonical identities.

Coordinate [visual/DPI](../qa-visual-dpi/SKILL.md) and [accessibility](../qa-accessibility/SKILL.md) for real native observations. Report exact locales and any catalog-only versus live-UI limits.
