# Windows notifications: research and HTML design contract

Research date: 5 November 2026. Read-only source and official-documentation research. Canonical repository `/workspace/NevroTick`, source `8a66af924c509e898a9bab9bb3282f11b24fba93`; prototype baseline `ed523ef` (source `c6e0801`). No repository/prototype edits, native calls, registration, service/clock/OS changes or real notifications. Microsoft bodies fetched successfully over the configured proxy with normal TLS verification; environment policy observed unrestricted/enforced, no credentials required. Some Microsoft reference pages carry the general prerelease disclaimer; this is source guidance, not selected-provider/native qualification.

## Main conclusion

Use **local Windows app notifications (toast/banner and Notification Center)** for local timekeeping problems. This meets the user's desired Windows notification experience without adding an external backend. **Windows Push Notification Services (WNS)** is the separate cloud-to-device push system: Microsoft describes an application's own cloud service, channel URI, authentication and network delivery. It is not required merely to show a local service-status warning. The product-facing label should be **Windows notifications**, not WNS. [M1–M3]

The HTML prototype should draw and exercise a labelled Windows-notification preview; it must not invoke browser `Notification`, request web-notification permission, register a service worker, send WNS/WebPush or call an OS protocol/notification API. The user has authorized a design change to the prototype, not those native effects.

## Canonical context and user override

`docs/windows-time-features.md` §7 (lines 367–373) explicitly says default closure leaves no UI process. Optional tray provides visibility only while that observer runs, and cannot warn if it also dies. It defers a compulsory watcher, independent watchdog and rich toast notifications (§9/line442). `docs/plans/windows11-integration.md` §8 (lines208–212) already identifies local app notifications as a future candidate: unelevated UI presentation, fresh observed reason, revalidated activation, no privileged authority from clicking.

The latest direct user instruction supersedes the previous omission **for this HTML design**. It does not select a new watcher, change native architecture or pass Windows execution gates. Preserve the distinction: enabling notifications is a per-user presentation preference; it does not start a service, enable the tray or guarantee continuous observation. Closing the last presenter means no newly detected immediate local notification can be promised. Retain in-window status and durable diagnostic events independently of delivery.

Canonical references:

- [Feature plan §7](https://github.com/DavidOsipov/NevroTick/blob/8a66af924c509e898a9bab9bb3282f11b24fba93/docs/windows-time-features.md#7-visibility-when-the-window-is-closed).
- [Windows integration §8](https://github.com/DavidOsipov/NevroTick/blob/8a66af924c509e898a9bab9bb3282f11b24fba93/docs/plans/windows11-integration.md#8-present-live-state-through-native-ui-facilities).

## Verified platform facts and implications

| Question | Official evidence | Prototype/implementation implication |
| --- | --- | --- |
| Local versus WNS | App-notifications overview explicitly covers local notifications created/displayed from the app; cloud delivery is separate push. [M1–M3] | No account, push endpoint, server URL, cloud credentials or network permission setting. |
| Who presents a service problem? | Windows App SDK app notifications are unsupported for elevated apps. Windows services cannot directly interact with a user as of Vista; services run in session 0. Microsoft recommends a separate interactive-user GUI/IPC. [M2, M4, M5] | An unelevated, per-user UI/tray presenter observes approved service state. Do not make the privileged service/helper draw interactive UI or imply services alone guarantee toast delivery. Do not add a watcher silently. |
| App closed when user clicks? | SDK quickstart describes running-app notification events and COM activation when not running. It requires registration before lifecycle activation query and handling activation arguments; desktop `activationType="background"` alone does not implement silent background handling. [M4] | A future installed, registered native application may be launched by an existing notification. That does not mean a dead app is detecting and sending new local warnings. In HTML, clicking preview routes to relevant current state. |
| Can user/organization block notifications? | `AppNotificationManager.Setting` reports whether enabled and why disabled. Enum distinguishes Enabled, DisabledForApplication, DisabledForUser, DisabledByGroupPolicy, DisabledByManifest and Unsupported. [M6, M7] | Separate app preference from current Windows availability. Include unverified/unavailable/blocked preview states; never derive Windows permission from app toggle. Group Policy block is an explanation, not an app bypass. |
| Do Not Disturb and lock screen? | Windows supports global/app notification settings, DND, banner and Notification Center choices, sound, priority and hiding lock-screen content. [M8] | Respect system suppression. Do not claim every important issue appears immediately or force Alarm/IncomingCall behavior to bypass DND. In-window problem remains regardless. Avoid app claims about exact DND state unless actually observed by a selected native contract. |
| Notification accepted versus seen? | `Show` is a submission operation after registration, while OS/user settings control presentation; current docs do not establish user-read acknowledgment. [M4, M6–M8, M11] | Use “preview”, “submitted” or “Windows may suppress the banner”, not “delivered/read”. A successful test submission does not prove user saw it. |
| Tidy Notification Center? | Microsoft recommends clear intent, avoiding distractions, silent Notification Center placement when appropriate, and removing obsolete notifications. `Tag`/`Group` and remove-by-tag/group APIs support identity-based cleanup; expiration/reboot removal are supported. [M9, M10] | One current incident notification, update/retire by stable incident identity. Do not accumulate every poll or remove unrelated applications' notifications. Runtime source state remains authoritative. |
| SDK/provider selected? | SDK APIs include deployment/Singleton dependencies; existing Rust/Win32 project is not automatically a WinUI/Windows App SDK app. [M4, M6] | This research selects local notification UX, not a new dependency/UI framework. Future native owner must qualify chosen bindings, package registration, activation and supported Windows build. |

## Recommended settings and wording

Make notification controls reachable in **Simple Settings** and **Power user Settings**; a compact shared Notifications section or clear entry is preferable to an isolated advanced-only menu. Notifications are a per-user preference, distinct from administrator-only service startup.

- **Notify me about problems** — On by default for this new design; user may turn it Off. Explain: “Important timekeeping problems and actions that need your attention. Repeated checks do not send repeated alerts.”
- **Notify me when a reported problem is resolved** — optional, default Off to avoid success noise. Enable only if the app actually reported that incident and fresh state confirms its resolution.
- **Windows notification availability** — separate sample observation: “Not checked”, “Allowed by Windows · banner presentation still depends on system settings”, “Blocked in Windows”, “Managed by your organization”, “Unavailable”. Do not label allowed as “delivery guaranteed”.
- **Preview notification** — shows local in-page toast sample. Keep OS readiness/outcome selectors in the external reviewer panel; user does not choose whether a production command succeeded.
- **Windows notification settings** — in HTML explain/show a launcher preview only; do not actually open an OS URI. Native implementation can later open the documented OS settings destination with user intent.

Explain availability at the relevant settings point: “New alerts require NevroTick or its tray observer to be running. Windows may suppress banners. Current problems remain visible in the app.” This is meaningful for a user's tray/close choice; avoid inserting COM/session/API names into the product flow.

A preview should have app identity/logo, short problem title, one consequence, one safe next action, and a clear **notification preview** label. Include optional dismiss action; dismissal is acknowledgment of that banner, not resolution of the service problem. Avoid extra charts, configuration toggles and diagnostic code inside a toast.

## Proposed trigger policy for the existing statuses

This table is design guidance, not a native timing/persistence policy. The UI must consume an explicit authoritative incident/persistence classification; it must not invent elapsed time from frozen fixture ages or install a timer. Native persistence limits, clocks, bounds and service policy need their owning implementation contract.

| State/event family | Notify? | Reason and safe destination |
| --- | --- | --- |
| Confirmed loss of all usable time estimates (`expired`), conflicting current time evidence (`conflict`), active signed bootstrap contradiction (`signed-conflict`), competing clock controller (`blocked`) | Important incident on new occurrence/material escalation | State consequence without declaring compromise. Open current Status/Diagnostics or precise Time policy context. |
| A live correction proposal requires administrator review (`correction`) | Attention notification when a current proposal becomes actionable | “A clock correction needs review”; opens review, never applies it. Expired proposal must not remain an actionable old toast. |
| Unknown protected correction result (`outcome-unknown`), unknown runtime/ownership after lost acknowledgment, cleanup-unconfirmed/partial ownership transition | Important incident; do not claim failure or success | “Result needs checking” / “Time management needs attention”; open Diagnose/reconciliation. No Retry/Apply/Start directly in a toast. |
| Confirmed unexpected service failure, including both services unavailable while expecting NevroTick control | Important incident, only with confirmed observation | Do not equate disconnected UI with stopped service. Open service diagnostics. Intentional user Stop/Disable is not an unexpected outage. |
| `stale` / status channel unavailable | Notify only after an explicit persistent connection-loss classification | “Current time status is unavailable”; do not say time synchronization stopped or service crashed. |
| Holdover/offline/all-rate-limited; rejected or unavailable Roughtime when no usable independent NTS path | Coalesced warning only when service declares prolonged/degraded/actionable | Explain last estimate versus expired estimate. Do not alert on every retry/RATE packet. Use relevant Diagnostics/Recovery. |
| Single-source DENY or RATE with sufficient fresh agreement remaining | No generic critical toast; an actionable persistent source configuration issue may be a separate normal warning | Healthy clock state must stay healthy. Never recommend more aggressive requests or authentication downgrade. |
| Deliberately chosen plain NTP/observe-only; expected startup/resume/reacquisition, normal slew, healthy NTS, Roughtime success/history, routine leap knowledge updates | No problem notification merely for being in this state | Persistent in-window explanation suffices. Explicit profile choice is not an unexpected failure. |
| Date recovery's historical rejected/contradictory evidence while independently usable current NTS exists | No new notification based only on old history | Never resurrect prior events after startup/owner changes or treat history as a fresh incident. |
| Windows owns time; NevroTick stopped/disabled or running observation-only | No NevroTick-outage notification | Handover/intentional disable is valid. Windows owner, W32Time runtime and verified Windows sync remain separate. Expected initial “sync not verified” is not itself proof of outage. |
| User-requested startup change pending, cancelled or rejected before effects | In-window result; no ambient problem toast solely for ordinary interaction | Configuration-only pending/unknown does not turn healthy current synchronization into unknown. A persistent unresolved config error can later warrant its own notification, scoped to startup. |
| Previously reported incident resolved by fresh independent authoritative state | Replace/retire stale alert; optional one resolution notification | Never call recovered from Running, accepted request, Roughtime-only success, source reply count or navigation. |

## Repetition, dismissal, privacy and activation

1. Key an incident to protected service/clock generation and meaningful problem identity. Do not key it to text, render count or every poll. Coalesce duplicates; notify once on first observed incident and once on material escalation. Keep memory/history bounded. A later new incident may alert again; merely reopening a page must not.
2. Dismiss/mute affects presentation only. Preserve in-window problem, diagnostic record, actual state and correction policy. Do not prevent a new materially worse or different incident from surfacing merely because an old banner was closed. Respect master Off and OS suppression even for important status. No silent bypass.
3. Notification body stays generic: problem, consequence, safe action. Put source hostnames/IPs, detailed system policy, accounts, secrets, full diagnostics and precise proposal data behind the app's appropriate review. Windows can show content on the lock screen; make the default safe without claiming to control every OS setting.
4. Actions open/activate the relevant context and reconcile current incident, generation and authority. A stale notification should say the old issue changed/resolved or show the latest state; it must not replay an old protected request. Clicking grants no administrator permission, profile change, service operation, clock write, relaxed NTS check or duplicate retry.
5. If entering Power user is needed, make that transition clear and preserve a sensible return path. A universal “Review clock status” action can safely open Simple Status first; a second explicit action can lead to Diagnostics/Recovery/Time policy.
6. Retire/replace an obsolete product notification using its own tag/group; do not use global Windows notification controls as a proxy for resolving the issue. Remove/expire old proposals and use generation-aware activation after reboot.
7. Model Windows-blocked, no-presenter, stale activation, dismissal, duplicate incident, escalation, resolution, app preference Off and no-current-issue cases in the prototype. Never simulate OS delivery as native proof.

## Suggested acceptance cases

- Same incident observed/re-rendered repeatedly produces one preview entry; real escalation/new generation can produce a new entry.
- Notification Off or simulated Windows block does not hide Status, alter sync/authentication or stop diagnostics/services. App On does not display Windows Enabled without a separate observation.
- Intentional Disable/Windows handover never emits “NevroTick stopped unexpectedly”; valid Windows management is retained.
- Old Roughtime/NTS/events after runtime operations do not create new current-issue notifications.
- Startup-only pending/unknown is reported as startup configuration, not clock failure; no spurious DEBUG interruption.
- Toast Review opens the current relevant context. A resolved, expired or superseded event cannot authorize a clock/service mutation; no native API or external protocol is invoked.
- Recovery notification requires a previously reported incident and fresh qualified restoration, and is suppressed by default when disabled.
- Dismissal changes the visible sample only; unresolved incident remains in app. Bounded duplicate suppression and unknown/blocked OS sample states are readable.
- Verify keyboard/focus and no horizontal overflow at 360/480/1280; preview is distinguishable from an actual browser/Windows notification. Native Narrator, notification activation, registration, OS settings, session routing and delivery remain unexecuted.

## Official sources

- **M1 — Windows notifications overview:** https://learn.microsoft.com/en-us/windows/apps/develop/notifications/
- **M2 — App notifications overview (local; elevated limitation):** https://learn.microsoft.com/en-us/windows/apps/develop/notifications/app-notifications/
- **M3 — WNS overview (cloud channels/backend):** https://learn.microsoft.com/en-us/windows/apps/develop/notifications/push-notifications/wns-overview
- **M4 — Local app notification quickstart and activation:** https://learn.microsoft.com/en-us/windows/apps/develop/notifications/app-notifications/app-notifications-quickstart
- **M5 — Interactive services and session 0:** https://learn.microsoft.com/en-us/windows/win32/services/interactive-services
- **M6 — AppNotificationManager.Setting:** https://learn.microsoft.com/en-us/windows/windows-app-sdk/api/winrt/microsoft.windows.appnotifications.appnotificationmanager.setting?view=windows-app-sdk-1.8
- **M7 — AppNotificationSetting enum:** https://learn.microsoft.com/en-us/windows/windows-app-sdk/api/winrt/microsoft.windows.appnotifications.appnotificationsetting?view=windows-app-sdk-1.8
- **M8 — Windows notifications, Do Not Disturb and user settings:** https://support.microsoft.com/en-us/windows/change-notification-settings-in-windows-8942c744-6198-fe56-4639-34320cf9444e
- **M9 — Notification UX guidance:** https://learn.microsoft.com/en-us/windows/apps/design/shell/tiles-and-notifications/toast-ux-guidance
- **M10 — Removing/expiring local app notifications; Tag/Group:** https://learn.microsoft.com/en-us/windows/apps/develop/notifications/app-notifications/manage-app-notifications
- **M11 — AppNotificationManager.Show:** https://learn.microsoft.com/en-us/windows/windows-app-sdk/api/winrt/microsoft.windows.appnotifications.appnotificationmanager.show?view=windows-app-sdk-1.8

The fetch for a guessed `.notificationsetting` property URL returned404; the actual documented property is `.setting` (M6). No conclusion relies on the failed URL. Research-page text snapshots are under `research-pages/` beside this report for coordinator traceability; they are not product assets.
