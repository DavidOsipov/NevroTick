# Native interaction and accessibility

Architecture section 16 controls acceptance. The website skill's semantic intent
transfers to native roles, relationships and behavior; HTML tags, ARIA and browser
audits do not constitute native Windows evidence.

Choose native text, action, check/radio, list and dialog controls according to the
task. Expose meaningful UI Automation names, roles, values, states, relationships
and supported patterns. Associate labels and field errors; decorative icons must
not become duplicate action names. Dialog titles describe the task; focus starts
predictably, Escape/cancel behaves correctly and focus returns after dialogs/UAC.

Preserve keyboard tab order, access keys, visible focus and complete operation
without a pointing device. Keep critical settings/recovery reachable outside the
tray. Test Narrator speech separately from UIA property assertions; avoid replacing
real spoken-output evidence with a synthetic accessibility tree.

Support Windows contrast themes, text scaling, at least 200% display scaling and
multi-monitor DPI changes without hidden controls or values. Status distinctions
use text/icon/shape as well as color. Long source labels, negative numbers and
larger fonts must not hide the action or rejection reason. Provide text/table
equivalents if a chart is later justified; avoid permanent animation/GPU work.

The UI consumes versioned current-generation snapshots and labels retained data
stale. A transport error and an empty valid source list are different outcomes.
Show explicit units and the public offset sign. The source identity used for
security decisions stays distinct from its potentially localized/display label.
Keep implementation details out of normal user actions unless needed for a choice.

Start with English and a typed message catalog/stable error IDs. Russian is added
only when explicitly adopted; it is not mandatory inherited website parity.
Translate display strings, not wire keys, stable IDs, numerical units or authority.
For an adopted locale check placeholders/plurals, accessible names, long text and
bidi/non-ASCII display. Diagnostic UTC/local presentation labels its time scale.

Use applicable WCAG A/AA through WCAG2ICT as an evaluation framework. WCAG2ICT is
guidance, not a native certification. Retain UIA inspection, Accessibility Insights,
keyboard and Narrator evidence for the actual target application and Windows lane.

Primary sources and architectural status are recorded in [SOURCES](../../SOURCES.md).
