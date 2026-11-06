# Independent browser acceptance: statuses and Windows notifications

Target: `/workspace/nevrotick-ui-audit/notifications-work/NevroTick-UI-v0.4`.
Baseline package `ed523ef`; **final source `62aa9713548f8bfefc7ea087a01d788bc6f39c10`, standalone SHA-256 `4d8c3d211ec19ddea21a4b37d6b164f453a8ab8352cca6c554c8c8ac28370f86` accepted**. Navigation/capture began only after explicit coordinator freeze and identity verification. This is read-only browser verification of deterministic HTML samples, not Windows notification/SCM/clock qualification. No source/test edits.

Sources: `../SPEC-RU.md`, `../status-design-review.md`, `../status-source-audit.md`, `../windows-notifications-research.md`.

Both HTML files will be checked at 360/1280 px, with selected 480 px cases. Probe controls act through the rendered UI rather than internal state/reducer. Fresh accepted screenshots will be opened and inspected; old screenshots are contextual findings only.

| Requirement / risk | Observable expected behavior | Independent evidence | Result |
| --- | --- | --- | --- |
| Identity / source freeze | HEAD equals supplied immutable source; no src/test drift; exact bundled hash matches | Git readback and SHA-256 before navigation | Pass; UI scope below |
| Notification entry in both modes | Simple Settings has shared preference/clear details path; Power Settings Notifications reachable; mode switch preserves preference | UI navigation in both builds; screenshots | Pass; UI scope below |
| Preference vs Windows availability | Master On is separate from Allowed/blocked/policy/DND/Unknown/unsupported and active presenter | Settings facts + reviewer conditions; no automatic permission claim | Pass; UI scope below |
| Test notification | Clearly labelled test, separate identity; no health/delivery claim; obeys master Off and readiness | Test then real issue; problem remains eligible; actual preview/actions | Pass; UI scope below |
| Healthy / normal noise | Healthy NTS, one-source RATE with alternatives, deliberate NTP/observe, normal startup/resume/Roughtime/slew/history produce no issue toast | Representative scene matrix and published sample observations | Pass; UI scope below |
| Immediate real problem | Expired/conflict/active signed conflict/ownership conflict/current review/unknown operation/partial state produces one appropriately scoped issue preview | Actual external publish + preview text; affected status remains visible | Pass; UI scope below |
| Persistence gate | Holdover/offline/stale/refusal/all-backoff/unavailable or rejected date remains inline until explicit persistent/actionable observation | Publish before/after persistence; no fake elapsed time/poll count | Pass; UI scope below |
| Rejected signature vs verified contradiction | Single unverifiable Roughtime signature is not attack or immediate conflict; active signed contradiction is separate | Scene/readback comparison; lock-screen-safe content | Pass; UI scope below |
| Dedup / dismissal | Repeated same incident and navigation/filter/mode/export do not duplicate; dismiss closes preview but issue stays and repeat does not reappear | Publish repeatedly; dismiss; navigate then republish | Pass; UI scope below |
| Material/new incident | New relevant incident or escalation can surface; bounded retained identities; no duplicate from readiness changes | Existing UI-supported transition/publish sequence | Pass; UI scope below |
| Master Off / OS denied | No visible notification or delivery claim; app status, sync/auth/diagnostics unchanged; eligible problem not consumed | Toggle/readiness gates then restore and publish current issue | Pass; UI scope below |
| DND / no presenter | No promised immediate banner; current issue stays in app; no new watchdog/service implied | Simulated OS/presenter conditions + Settings explanation | Pass; UI scope below |
| Activation current | Safe Open/Review opens latest status or explicitly marked Power destination; no privileged action executes | Focus/page readback; no approve/apply/restart/clock mutation | Pass; UI scope below |
| Activation stale / invalid | Old/invalid ID cannot open expired proposal or replay command; actual current state governs | UI-supported stale transition and DOM event ID manipulation if needed, never internal model state | Pass; UI scope below |
| Intentional stop / disable / handover | No unexpected-outage toast for successful deliberate operation; Windows owner remains sync-unverified and no recovery success claim | Runtime/lifecycle controls + publish after confirmed effects | Pass; UI scope below |
| Cancel / rejected-before-effect | Only inline command outcome; no new command-result incident; pre-existing expired/conflict remains eligible | Cancel/UAC/rejected paths with existing issue | Pass; UI scope below |
| Startup-only unknown | Scope limited to future startup; current healthy sync/DEBUG preserved; no clock-failure copy | Startup request/lost ACK then current facts/notification | Pass; UI scope below |
| Outcome-unknown current owner | No unqualified confirmed NevroTick owner in Time policy/Diagnostics/export | Cross-view visible readback; no fresh authority inferred | Pass; UI scope below |
| Windows observer lost ACK | Windows independently confirmed owner/config remain; Nv runtime/startup uncertain only where affected; Windows sync unverified | Use Windows Time → Enable and start/Stop/Restart observer → unknown; both modes | Pass; UI scope below |
| Stopped / pending / unknown primary actions | Confirmed stopped leads to management; unknown leads to Diagnose/unavailable checks, never false stopped or enabled Sync now; startup-only healthy unaffected | Initial stopped plus runtime pending/unknown and adjacent healthy case | Pass; UI scope below |
| Observe-only authority and action | Observe is not confirmed clock writer; Check sources means measurements only; no implied Windows ownership | Status/Settings/export and source-check feedback | Pass; UI scope below |
| Scene / full reset | Scene is separate sample, not recovery; preference preserved if specified; full reset defaults guarded; pending data loss warning remains | Scenario/reset controls + cancellation/focus | Pass; UI scope below |
| Focus / keyboard / responsive | Toast/test/dismiss/open/settings reachable by keyboard; no BODY focus trap or horizontal overflow at 360/480/1280 | Focus/DOM measurements + inspected screenshots | Pass; UI scope below |
| Local-only / privacy | No page errors/unexpected remote requests or browser notification prompt; notification body has generic problem/action and no endpoint/offset/account/path/secrets | Browser runtime capture + visible content | Pass; UI scope below |

## Planned report boundaries

Use fresh screenshots and current candidate evidence only. Source/CSP/AVIF/checksum verification by other participants is not replaced by browser acceptance. OS registration, native Windows notification activation, delivery, DND detection, Session 0 IPC/presenter lifecycle, Narrator and real service/clock effects remain unexecuted.

## Executed scope and verdict

Pass within HTML browser acceptance. Full independent UI probe ran against frozen source `7f0a933c381f1092fde563fdd9c64c71f9f0d41e`, standalone `6c916c8b2ee7d0833b736641e27206b992727401018236485b3f5c1812c32cd0`: both HTML at360/1280/480, six matrix cases, 80 workflow groups including60 initial scene classification checks. Final production delta `62aa971` was accepted with a fresh bounded probe on exact final hash: six matrix cases,90 detailed cases (50 durable partial retry/readiness,10 startup-unknown with unrelated Stop cancellation,30 tray semantics/ARIA). Fresh final Settings/notification/suppression screenshots were inspected. Zero page errors/unexpected remote requests; earlier broad probe also trapped zero native Notification API calls.

The known source finding that a no-effect retry could hide a still-current partial handover incident is resolved. Cancel/UAC/rejected no longer suppress first eligible problem after OS readiness, nor duplicate an already shown incident. No open browser findings. UI-level new-incident behavior is exercised; internal storage-size/generation/CSP/AVIF/native qualification are other owners and are not inferred from screenshots. See REPORT-RU.md and verification-summary.json.
