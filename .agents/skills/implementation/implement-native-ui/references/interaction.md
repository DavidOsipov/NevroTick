# Native interaction and accessibility

Architecture section 16 controls acceptance. Transfer website semantic intent to
native roles, relationships and behavior; HTML tags, ARIA/browser audits prove no
native Windows behavior.

Choose native text, action, check/radio, list/dialog controls by task. Expose meaningful
UI Automation names, roles, values, states, relationships and supported patterns.
Associate labels/errors; decorative icons must not duplicate action names. Task-specific
dialog titles, predictable initial focus, correct Escape/cancel and returned focus
after dialogs/UAC are required.

Preserve tab order, access keys, visible focus and full operation without pointing
devices. Keep critical settings/recovery outside tray. Test Narrator speech
separately from UIA assertions; synthetic trees cannot replace spoken-output evidence.

Support Windows contrast themes, text scaling, at least 200% display scaling and
multi-monitor DPI changes without hidden controls/values. Status uses text/icon/shape
plus color. Long labels, negative numbers/larger fonts must hide no action/rejection
reason. Future justified charts need text/table equivalents; avoid permanent
animation/GPU work.

Consume versioned current-generation snapshots; label retained data stale.
Transport errors differ from valid empty source lists. Show units/public offset sign.
Security source identity differs from localized/display labels. Expose implementation
details in normal actions only when needed for decisions.

Start with English and typed message catalog/stable error IDs. Add Russian only
when explicitly adopted, not inherited website parity. Translate display strings,
never wire keys, stable IDs, numerical units or authority. For adopted locales check
placeholders/plurals, accessible names, long text and bidi/non-ASCII display.
Diagnostic UTC/local presentation labels time scale.

Evaluate applicable WCAG A/AA through WCAG2ICT. WCAG2ICT is guidance, not native
certification. Retain UIA, Accessibility Insights, keyboard and Narrator evidence
for actual application/Windows lane.

Primary sources/architectural status: [SOURCES](../../SOURCES.md).
