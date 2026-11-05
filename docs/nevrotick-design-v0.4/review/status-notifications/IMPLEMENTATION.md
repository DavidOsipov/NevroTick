# Status and Windows notification implementation

Local worktree: `/workspace/nevrotick-ui-audit/notifications-work/NevroTick-UI-v0.4`, branch `status-notifications-ui`. Binding inputs: `SPEC-RU.md`, `windows-notifications-research.md`, `status-source-audit.md`, and coordinator-confirmed browser/source findings. Scope remains the HTML prototype only.

## Candidate identity

- Status fixes: `cd90ed0`.
- Notification implementation: `7f0a933c381f1092fde563fdd9c64c71f9f0d41e`.
- Final production source and unit/render regression delta: **`62aa9713548f8bfefc7ea087a01d788bc6f39c10`**.
- Dedicated browser test only: **`bcbd4fc30becad3c0dd5109b7df93c3bc21c0099`**. Production `src/` is identical to `62aa971`; no source drift after the final production freeze.
- Final standalone SHA-256: **`4d8c3d211ec19ddea21a4b37d6b164f453a8ab8352cca6c554c8c8ac28370f86`**, 822,403 bytes.
- Both HTML files built with the existing dependency-free `build.cjs`; no new runtime dependency, module loader, framework, service or backend.

## Status corrections

- The initial unknown clock outcome displays **Unconfirmed · last known NevroTick** instead of unqualified ownership. Observe-only explicitly says no clock-writing authority and leaves the current time manager unconfirmed. Presentation qualifies the label without changing the raw source scenario or widening existing control guards.
- Lost replies from Start/Stop/Restart of a Windows-owned NevroTick observer leave independently confirmed Windows ownership intact. Only NevroTick runtime and, where applicable, enabling-start configuration become unknown. Simple, service feedback, Settings, Status and export keep Windows synchronization unverified.
- Stopped Status actions lead to Time management; pending/unknown runtime cannot present enabled Sync now. Diagnose remains available. Configuration-only pending/unknown preserves healthy checks. Observe/blocked/unknown-clock-outcome use **Check sources** and explain that measurements do not apply a clock change or repeat an uncertain command.
- The tray uses the same runtime-aware action without a dangling Status-only ARIA reference. Initial stopped Power next-step text points to Background service/Start; repair stays conditional on failure in Diagnostics.

## Notification behavior

- **Settings → Notifications** in Power and a shared switch/summary in Simple. **Notify me about problems** is On in the default sample. OS availability and app/tray availability are independent sample observations. The test button and Windows settings navigation example are reachable without administration.
- A labelled Windows notification preview appears outside the application window. It contains only a static, public-safe problem summary and a read-only opening action. No browser or Windows notification API is called.
- All 30 scenes have explicit policy entries. Current unusable estimates, source/ownership conflicts, current actionable correction, partial or unknown operation results and unexpected stops are distinguished from harmless progression and retained history. Soft problems require the reviewer-published persistence fact; no timeout or poll-count rule is invented. Routine tracking, ordinary acquisition/resume/recovery progress, historical recovery, explicit NTP/observe, useful RATE alternatives and planned Stop/Disable remain quiet. There is no recovery-notification option or fake synchronized success.
- The bounded sample ledger keeps separate monotonic local notification IDs, incident episodes and issue scopes. Repeated observations, navigation and dismissal do not produce another notification. Off, blocked/unknown Windows availability, DND suppression and a missing app/tray observer do not consume the first unshown eligible alert. Readiness changes do not reannounce an incident already shown.
- Tests use separate IDs and cannot acknowledge a real problem or verify time. A maximum of eight retained entries bounds the sample. Scene changes clear independent sample episodes while retaining the preference and monotonic ID counter; a full reset restores defaults but does not reuse an old ID.
- Activation validates the ID and rechecks current issue/proposal/generation. Stale activation opens current Status, not an expired proposal or an operation retry. Fresh activation opens current evidence/service settings/correction review only. Dirty recovery editing uses the existing leave guard; activation never grants clock authority, UAC approval or a service command.

## Review-driven issue lifetime refinement

The final delta separates current unresolved issues from the latest command feedback: `timeManagement.currentIssue`, `timeManagement.startupIssue` and service `currentIssue` carry the originating outcome scope. Cancel/Close/UAC cancellation/rejection of a later attempt cannot hide an unresolved partial handover or replace the identity of an earlier unknown autostart result. Notifications after a blocked first attempt remain possible when readiness returns; already announced incidents remain deduplicated.

Pending observation gaps are not treated as resolution. Running alone does not retire an incident. A newly published qualified tracking fixture can retire the prior episode; a new authority/proposal/outcome scope can establish a distinct issue. The partial-handover message describes the current absence of a confirmed synchronizer, rather than repeating stale stopped-service assertions after later observer activity.

## Fresh implementation checks

All listed commands completed with exit 0 for the stated candidate:

| Check | Evidence / result |
| --- | --- |
| `node --test tests/*.test.cjs` | Final source: **197/197 pass**, no skips. All 175 baseline tests remain; 22 new regressions/policy tests. The existing Settings test now covers five actual sections, including Notifications, instead of expecting four. |
| `node tests/render-fixtures.cjs ...` + `python3 tests/check_structure.py ...` | Final source: **1,247 documents**, zero structural errors. Final standalone CSP matches, no external standalone dependencies, original AVIF bytes preserved. Includes notifications, OS gates, both modes, settings modal, actual tray-from-preferences and retained partial-result variants. |
| `node tests/status-notifications.browser.cjs` | Final source: both HTML at **360/1280**, four matrix cases, **120 scene-policy checks**, settings/test/persistence/dedupe/dismiss/OS/presenter/current+stale+invalid activation/known+unknown lifecycle/focus/reset/overflow. **Zero page errors, unexpected remote requests and browser notification API calls.** Expected scene policies are explicit independent sets. |
| `node tests/startup-ownership.browser.cjs` | `7f0a933` before the isolated current-issue metadata delta: both HTML 360/1280, four cases pass; no page errors/remote requests. |
| `node tests/service-controls.browser.cjs` | Same pre-delta source: both HTML 360/1280, four cases pass; no page errors/remote requests. |
| `node tests/service-recovery-history.browser.cjs` | Same pre-delta source: **48 stage cases / 96 surfaces** pass. |
| `NAV_TEST_FILES=index.html,NevroTick-UI-v0.4.html node tests/navigation-reset.browser.cjs` | Same pre-delta source: both HTML 360/980, four cases pass. |
| `git diff --check` and final standalone hash | Pass. Source remained frozen after `62aa971`. |

The final issue-lifetime delta changes notification classification metadata and adds focused cases; it does not change the existing service/ownership operations, history rendering or navigation handlers. The final dedicated browser run exercises its no-effect retry/blocked-first-delivery/deduplication effects. Prior broad baseline passes are identified above rather than presented as new post-delta runs.

Logs: `/tmp/status-notifications-delta-unit.log`, `/tmp/status-notifications-browser-final.log`, `/tmp/status-notifications-{startup,service,history,navigation}-baseline.log`. Initial status regression failures are recorded in `/tmp/status-notifications-red.log`; the partial-lifetime regression failed on the earlier implementation in `/tmp/notification-partial-red.log` before its fix.

An initial browser harness attempt timed out on an incorrect helper input ID in an unnecessary final block. That block attempted an HTML interaction unavailable while its native dialog is modal; it was removed. Receiving-side dirty-editor activation is tested through the reducer. The corrected full browser script has no swallowed action failures and passed all four cases. Earlier harness output is preserved in `/tmp/status-notifications-browser-harness-error.log`.

Chromium version: **155.0.8059.79** on Debian. Twenty screenshots are under `notifications-work/screenshots`. Visually inspected final standalone notification-conflict 360 and Notifications Settings 360: both correctly show the intended surface, readable copy, correctly wrapped controls, distinct OS/app states and no horizontal overflow. Independent source and final browser acceptance are coordinated separately; this report does not substitute for their verdicts.

## Handoff boundaries

The source/test commits exclude generated HTML, coordinator README/review/checksum/package work and the canonical repository. Generated `index.html` and standalone remain modified locally for coordinator packaging. No canonical repository file, real service, registry, SCM/UAC, clock or notification platform was changed; no publication occurred.

Sample observations are published explicitly by the external reviewer panel. They model future service-to-user-app presentation without a live subscriber or automatic native notifications. The selected native notification binding/provider, registration/activation, protected IPC, Windows availability observation, DND behavior, lock-screen rendering and delivery need future Windows qualification. An app/tray observer must be active to detect/present new local alerts; no new always-on watcher is implied. HTML reload persistence of native notification identities is outside this prototype.
