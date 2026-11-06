# NevroTick v0.4 — Simple / Power user candidate review

Review date: 2026-11-05. Scope: read-only review of `NevroTick-UI-v0.4/src/{model.js,views.js,app.js,styles.css}` plus focused Node checks. No implementation files changed.

## Final disposition

- **P0: none found within the reviewed scope.** No enabled service/clock/policy commit path was introduced by the mode or cadence controls.
- **P1: none found within the reviewed scope.** Simple retains the material trust, freshness, clock-ownership, recovery and unknown-result distinctions.
- **P2: two actionable findings, both corrected and independently rechecked below.** No remaining blocker identified for packaging this offline specimen.
- This is source/model/HTML-string evidence. No browser rendering, real clicks, keyboard walkthrough, screen-reader run, native UI qualification or final downloadable build verification was performed by this reviewer.

## Findings and confirmed fixes

| Priority | Finding | Evidence / fast fix | Final status |
|---|---|---|---|
| P2 | Policy-related actions initially opened the default Synchronization section after `settingsSection` changed to `schedule`. A person investigating blocked clock ownership landed on cadence controls. | `views.js` blocked status/help, Sources “View policy”, Simple “Open advanced settings”; explicit `policy-page` route in `model.js` and `app.js` now selects Power → Settings → Time policy. General mode switching still preserves the current section. | **Fixed.** All four CTA sources and reducer destination checked. The route closes the help modal and preserves source drafts, query, saved cadence and unsaved cadence. |
| P2 | Simple initially reused specialist next-step wording such as “bounded check”, package lifecycle and “authoritative state”. This weakened the intended reduction in reading effort. | `views.js` now uses `simpleNext()` for Simple status and help; examples include “Checking sources automatically”, “Open diagnostics for repair guidance” and “Check the result before trying again”. Retained estimate freshness now says “Last valid time check”. | **Fixed.** Targeted status/help output and holdover/offline freshness labels rechecked. Power keeps its detailed language. |

## Confirmed interaction and state properties

- Simple is the default; its navigation exposes only Status and Settings. Reviewer scenario controls start collapsed. The visible mode group uses native buttons with `aria-pressed`.
- Simple status is one section with state, plain explanation, trust qualifier, paired difference/uncertainty, freshness and a contextual action. The source, chronology, calendar and detailed evidence panels remain in Power.
- Ordinary mode changes preserve source drafts, search/filter values, diagnostic filter, Settings section, compact preference, saved cadence and the unsaved cadence selection. Returning from a Power-only page restores that page when appropriate; Settings remains Settings.
- While a modal is open, mode controls are disabled and `view-mode` returns the same state object. `app.js` skips rendering for that no-op, preserving raw source-editor input.
- Cadence supports Automatic and the 2/5/15-minute design choices in both modes. Update preview changes the in-memory preference only; active fixture data remains unchanged. Invalid values do not replace valid preferences. Cancel restores the saved selection.
- Scenario switching preserves mode and cadence while clearing source drafts; the reviewer controls disclose that behavior. Explicit reset restores Automatic and clears drafts while retaining the chosen interface mode.
- The final explicit policy route preserves these data properties. All four Settings sections survive a Simple/Power round trip.

## Truthful Simple representation across the 23 scenes

- NTP remains explicitly unauthenticated; Observe only says clock changes are disabled; blocked ownership is not presented as a measurement failure.
- Stale status states that current service health is unknown, distinct from confirmed service stoppage. Stale, expired, disagreement, resume, guarded UTC transition, expired proposal and unknown operation result do not display a numeric **current** clock difference.
- Holdover/offline retain a clearly labelled previous-model estimate with its age. Fresh metrics keep offset direction and uncertainty together; the large forward proposal reads “6 min 12 s behind” and remains not applied.
- Recovery scenes retain their future-design qualifier. Plain-NTP recovery ancestry stays visible after authenticated agreement resumes. Conflicting recovery cannot bypass verification.
- Correction review and unknown-result review keep their protective explanations and disabled privileged controls. Sync now remains disabled for this specimen. A preference update cannot create an applied-service result.

## Verification evidence and limits

- **12 focused grouped Node checks passed** for defaults, mode/data retention, modal protection, valid/invalid cadence, cancel, scenario/reset behavior, critical Simple qualifiers and metric coherence.
- **207 generated view documents checked:** 23 scenes × (2 Simple pages + 7 Power pages); no render exception, duplicate HTML IDs, or missing/duplicate page heading was found. This checks generated strings, not browser layout.
- A subsequent bounded pass confirmed the two P2 fixes, their four policy CTA sources, preserved state, all four Settings section round trips and the revised Simple copy.
- CSS source review supports the intended compact Simple surface: 680px maximum width, no imposed content minimum height, responsive header/actions, one-column settings, and removal of Power evidence grids from Simple. Actual visual fit and typography still require opening the artifact in a permitted browser.
- Download/build wiring was outside this pass; the parent owns final build, aggregate tests and artifact packaging. No browser-policy workaround was attempted.
