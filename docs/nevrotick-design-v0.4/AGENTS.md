# NevroTick HTML design prototype v0.4

This is the current visual/interaction reference, imported and extended at David's request on 5 November 2026. Start with README.md, README-RU.md, review/tray/REPORT-RU.md and the retained status/notification audit. v0.2 is historical; do not run or port its React tooling to edit this bundle.

[OD-11](../nevrotick-owner-decisions.md#15-od-11--replace-the-win32-ui-with-slint) selects complete replacement of the Win32 UI with Slint. Rust and required Windows OS adapters remain applicable. Apply canonical architecture and UI-M0/D1–D3 scope before native work; toolkit selection supplies no implementation or qualification and lifts no execution hold. Imported Rust/Win32 descriptions in About and historical reports describe the pre-migration baseline; OD-11 governs future UI. Record authorized design changes in the current manifest; preserve the original imported receipt as history. HTML samples grant no network, notification, service, registry, UAC, clock, installation or publishing authority. Keep user-facing copy English and reviewer controls outside the app in Russian. Simple starts by default; Settings and About stay reachable in both modes.

Edit src/model.js, src/views.js, src/app.js and src/styles.css, then run node build.cjs. Commit generated index.html and NevroTick-UI-v0.4.html together when a commit is authorized. Keep original AVIF bytes, provenance and asset rights. Preserve IMPORTED-SHA256SUMS.txt as the imported receipt; refresh current SHA256SUMS.txt for intentional bundle changes.

Separate runtime, startup configuration, clock owner and verified synchronization. Stop is temporary; disabling autostart does not stop current execution. Full disable needs confirmed stop/start inhibition and owned cleanup. Windows Time handover is explicit; Running does not prove synchronization. Preserve unknown/partial outcomes, evidence history, budgets and navigation guards. Custom intervals accept whole minutes of at least one.

Notifications are local Windows app/toast UX, not cloud WNS. Keep app preference separate from Windows availability and an active app/tray presenter. Coalesce repeated incidents; cancellation cannot resolve or renumber an existing issue. Notification activation opens current context and grants no privileged effect. Never invoke browser Notification, Push, ServiceWorker or native OS actions from this prototype.

Tray is off by default, with separate sign-in opt-in. Its menu derives from current
presented status, preserves unknown/partial outcomes and offers navigation only
for service management. Quit UI opens a confirmation explaining that running
services continue, with Settings → Background service → Stop / Disable NevroTick.
Cancel retains the observer; confirmation changes no service/clock state or saved
tray/sign-in preference. The last observer's exit stops new local alerts. Keep the
visual Windows area/reviewer controls outside the app; protect in-progress dialogs.

Use existing Node/unit, structural and relevant Chromium tests proportionally; report HTML checks separately from actual Windows qualification. Run a local preview yourself when browser inspection is needed. Do not publish or modify machine configuration merely to preview the design.
