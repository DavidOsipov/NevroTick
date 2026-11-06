# Independent browser acceptance matrix

Target: `/workspace/nevrotick-ui-audit/ownership-work/NevroTick-UI-v0.4`, baseline `351c1f2`; final SHA `c6e08010ee98e354ef6d01342449e057a06eba1a`; standalone SHA-256 `5970857e19b26f467c67e35a84f5b22dac91a1310939b445418afcd3e28328bb`. App navigation began only after the coordinator supplied the stable target. Final journal retest uses the final SHA/hash above. Read-only verification, screenshots and evidence files only. No real Windows service/clock/configuration operations.

| Requirement / risk | Observable browser behavior | Planned direct evidence | Result |
| --- | --- | --- | --- |
| Runtime / startup separation | Autostart Off leaves running services/current clock status/DEBUG active; On leaves stopped services stopped | Actual controls, before/pending/confirmed text and diagnostics | Pass (fresh browser evidence; see REPORT-RU.md) |
| Startup pending/unknown | One mutation pending; startup unknown blocks blind repeat, clock/DEBUG unaffected | Reviewer response + current facts/status/debug | Pass (fresh browser evidence; see REPORT-RU.md) |
| Administrator/cancellation | Confirmation needed, Escape/Cancel submit nothing; rejected pre-effect response retains facts | Keyboard actions and state screenshots | Pass (fresh browser evidence; see REPORT-RU.md) |
| Full disable | Both stopped + startup disabled + cleanup confirmed; no active synchronizer warning only without another confirmed synchronizer; existing Windows owner/status retained when disabling observer; settings/history/auth age/budgets retained | Settings, Status, Sources, Recovery, export | Pass (fresh browser evidence; see REPORT-RU.md) |
| Enable and start | Explicit confirmation; manual running; fresh acquisition; Windows ownership retained if present | Dialog + result/status/owner facts | Pass (fresh browser evidence; see REPORT-RU.md) |
| Windows handover ready | Plan mentions current ownership/readiness/policy; release before start; services disabled; Windows owner, sync unverified | Dialog, pending response, result, all views/export | Pass (fresh browser evidence; see REPORT-RU.md) |
| Preflight blockers | Effective policy conflict, unavailable prerequisite, ownership uncertainty block before mutation; GPO management alone permits allowed transfer; reviewer panel only | External fixtures + unchanged observed facts | Pass (fresh browser evidence; see REPORT-RU.md) |
| Partial handover | Cleanup unknown prevents W32Time start; W32Time unavailable after cleanup means no active sync; no false success | Reviewer partial outcomes | Pass (fresh browser evidence; see REPORT-RU.md) |
| Lost acknowledgment | Unknown outcome persists; no blind duplicate; current ownership unreconciled | Reviewer unknown + disabled mutation controls | Pass (fresh browser evidence; see REPORT-RU.md) |
| Reverse handover | Explicit confirmed reverse; stops and disables Windows Time while preserving its configuration; manual product startup; new acquisition | Dialog/result/current owner | Pass (fresh browser evidence; see REPORT-RU.md) |
| Windows observation | Start/autostart do not seize time from Windows; running W32Time is not called synchronized | Start under Windows owner + Status/export | Pass (fresh browser evidence; see REPORT-RU.md) |
| History isolation | NTS/Roughtime remain prior evidence after runtime/ownership, no age/budget reset or instant promotion | Sources/Stats/Recovery/Diag/export | Pass (fresh browser evidence; see REPORT-RU.md) |
| Presentation consistency | Same owner/startup/runtime facts in Status/Simple/Settings/Sources/Stats/Recovery/Diag/footer/export/dialogs | Cross-view readback | Pass: Diagnostics event-local history P2 resolved in c6e0801, both HTML 360/480/1280 retested |
| Focus/pending/reset | Pending survives dialog close/navigation; disabled control does not leave BODY focus; reset warns about loss | Keyboard/focus check + reset cancel/confirm | Pass (fresh browser evidence; see REPORT-RU.md) |
| Existing behaviors | Prior Roughtime/logging/source focus/reset behavior still works | Existing focused browser suites | Pass (fresh browser evidence; see REPORT-RU.md) |
| Layout/local-only | Both HTML at 360 and 1280, selected 480; no overflow/page errors/external requests | Fresh screenshots inspected + runtime capture | Pass (fresh browser evidence; see REPORT-RU.md) |
