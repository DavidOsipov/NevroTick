# NevroTick v0.4 — focused cadence and Statistics review

**Date:** 2026-11-05. **Scope:** read-only inspection of `src/model.js`, `src/views.js`, and `src/app.js` in the v0.4 candidate. No browser, native tooling, network experiment, code edit, or runtime qualification. Findings are source-derived. Root's in-progress authentication/profile row, reply column, marker shapes and explicit range changes are not duplicated as findings.

## Disposition

No P0 or runtime clock/service safety issue found in this bounded review. The cadence reducer updates only local preview state; it does not rewrite the active fixture, run a check, change authority, or announce service success. One effective-state semantic mismatch should be corrected; three small interaction/copy fixes follow.

## FS-01 — Effective schedule claims an active state that some fixtures cannot establish

**Priority:** P1 for unhealthy-state semantic fidelity; this is an offline representation issue, not an actual service operation.

**Location:** `views.js`, `frequencySettings()`.

The Power disclosure is titled **Effective schedule & service policy**, but always renders **Active sample schedule: Automatic**. In `stale`, current service health/scheduling is unknown; in `service-stopped`, synchronization is confirmed unavailable. There is no effective-schedule field in the model to support this blanket current-state claim. `scenario.next` is also a next action, sometimes a repair/diagnostic instruction, rather than an effective cadence.

**Small fix:** label Automatic as **Sample scheduling policy**, not active/effective scheduling. Keep **Preview preference** separately. Label the scenario text **Next action**; if an effective state is shown, derive only supported states (e.g. unknown for stale; unavailable for stopped), without inventing a countdown or acknowledged interval. A changed preference must continue to leave the healthy fixture's 40-second example unchanged.

## FS-02 — Cancel leaves the rejected manual interval in hidden state

**Priority:** small interaction fix.

**Location:** `model.js`, `frequency-draft` / `frequency-cancel`.

Source trace: save 2 minutes → select 15 minutes → Cancel. Visible draft returns to 2 minutes, but `frequencyCustom` remains `900`. Selecting Automatic and then Custom now revives the cancelled 15-minute value. The visible Cancel and the remembered manual draft therefore disagree.

**Small fix:** restore the remembered manual value when cancelling a custom selection. If the saved preference is Automatic, explicitly decide whether Cancel restores the last committed manual choice; retain that baseline separately if necessary. Keep Automatic/Custom toggling useful without silently resurrecting a cancelled edit.

## FS-03 — Cancelling a frequency edit removes keyboard focus

**Priority:** small accessibility fix.

**Location:** `app.js`, `dispatch()` and the `frequency-cancel` click branch.

The focused Cancel button is removed when `dirty` becomes false. Generic focus restoration searches for that removed button and has no fallback; unlike submit, cancel does not explicitly focus `frequency-mode`. The user can lose their place after keyboard cancellation.

**Small fix:** after `frequency-cancel`, focus the surviving `frequency-mode` control (or another deliberate surviving control). This is a source-level observation, not browser/Narrator verification.

## FS-04 — Stale notice scopes historical provenance too broadly

**Priority:** small wording fix.

**Location:** `views.js`, `statistics()` stale notice.

“Every value below belongs to the last received snapshot” also covers the current locally edited **Preview check preference** at the bottom. That preference is not part of the historical service snapshot.

**Small fix:** use “The measurements and source status shown here belong to the last received snapshot.” Keep the preview preference's existing statement that the sample schedule is unchanged.

## Correct behavior confirmed from source

- `sourceHistorical` is distinct from aggregate `historical`: conflict keeps current disputed per-source intervals; stale source ages retain **at last snapshot** wording.
- Plot bounds are calculated only from finite source offsets and uncertainties; unavailable values are not plotted as zero. Exact values remain in the table. Source measurement ages are explicitly allowed to differ.
- Holdover/offline use retained-model labels and zero current agreement, with no fabricated per-source intervals. Expired, resume, uncertain-effect and bootstrap scenarios do not acquire new source values from Statistics.
- Authentication is shown independently from agreement; explicit NTP retains its unauthenticated status. No confidence percentage, RTT, RMS, jitter, time series or uptime is manufactured.
- Requested cadence remains local; save/cancel/mode switching do not modify model evidence. Presets are described as illustrative, while recovery/backoff/server limits can change actual checks.
- Source review establishes no visual rendering, browser interaction, native accessibility, service integration, or production scheduling qualification.

## Focused resolution check — 2026-11-05

Re-read only the four changed source locations after root's repairs. The findings above describe the earlier candidate; the following disposition supersedes their open status.

| Finding | Resolution verified from source |
| --- | --- |
| FS-01 | **Resolved.** The disclosure now says **Sample timing & service policy**. Stale shows **Unknown — status disconnected**; stopped shows **Unavailable — service stopped**. Other scenarios show their existing next-action text. The preview preference remains separate and does not rewrite fixture timing. |
| FS-02 | **Resolved for the reported saved-custom cancellation.** `frequency-cancel` now restores `frequencyCustom` from a saved custom preference, so the 2 → 15 → Cancel → Automatic → Custom sequence returns to 2 minutes. When the saved preference is Automatic, the implementation deliberately retains the last manual selection; it does not maintain a separate committed-manual baseline. |
| FS-03 | **Resolved in the event handler.** After cancellation and rerender, `app.js` explicitly focuses the surviving `frequency-mode` control. Actual browser/assistive-technology behavior is not claimed. |
| FS-04 | **Resolved.** The historical notice now scopes its statement to **measurements and source state**, leaving the separately labelled local preview preference outside that claim. |

**Final bounded disposition:** the reported semantic mismatch and the three concrete fixes are addressed. No additional audit or tests were performed during this recheck; root's separate tests remain separate evidence.
