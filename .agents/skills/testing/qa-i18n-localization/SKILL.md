---
name: "qa-i18n-localization"
description: "Test NevroTick message catalogs, units and native layout for adopted locales while preserving wire and security semantics."
---

# Verify adopted locale semantics

Read [the testing contract](../CONTRACT.md).

Inspect the actual locale policy and typed message catalog. English is the initial scope; Russian is added only when adopted. Do not make a missing unadopted locale a product defect or copy the website's bilingual baseline automatically.

For adopted locales verify stable error/message IDs, placeholders, plural behavior, accessible names and fallback semantics. Locale must not change policy/wire identifiers, numerical units, offset sign, source identity or authorization. Local display and UTC diagnostics need explicit labels.

Test long messages, negative values, non-ASCII names, bidi/mixed-script text and settings/error states through actual consumers. Check field errors, truncation and assistive-technology pronunciation where relevant; display labels must not obscure security-sensitive canonical identities.

Coordinate [visual/DPI](../qa-visual-dpi/SKILL.md) and [accessibility](../qa-accessibility/SKILL.md) for real native observations. Report exact locales and any catalog-only versus live-UI limits.
