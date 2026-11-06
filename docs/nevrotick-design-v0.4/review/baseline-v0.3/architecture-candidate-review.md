# NevroTick v0.3 — independent semantic source review

Date: 5 November 2026 (Asia/Tbilisi). Reviewed `src/model.js` and `src/views.js` in `/workspace/scratch/34014f1556a3/NevroTick-UI-v0.3`.
Scope: read-only source review of 23 fixtures and their rendered text; no browser, native tooling or code changes. This review does not establish visual/browser/native qualification.
Disposition at first inspection: material fixture repairs required below. Root was notified promptly. Known NTP Delta wording, duplicate preview-boundary IDs and marketing-title repairs are not duplicated as new findings.

## Material repairs

| ID | Priority | Finding / source | Required repair |
| --- | --- | --- | --- |
| C-01 | High | `outcome-unknown` inherits a fresh −0.7 ms estimate, ±4.8 ms uncertainty and 3/4 current agreement while its latest clock effect may have occurred; `sourceTable` retains Main/Supporting roles. | Define the operation explicitly. If it may change clock coordinates, withhold current offset/uncertainty and current vote claims pending reconciliation; mark prior evidence historical and source rows waiting. A generic unknown effect must not leave current coordinates looking certain. |
| C-02 | High | `stale` changes top-level freshness, but `deriveSources` leaves source reply/accepted ages at 18/21/16/8 s ago. `sourceDetails` lacks the Sources page's historical banner. | Make each source age relative to the last snapshot or explicitly historical, including details. Qualify source roles/authentication as last-known there. A 2 min 14 s old snapshot cannot provide an 8-second-old current reply. |
| C-03 | Medium | `ntp` inherits `recovery = Strict NTS verification` and its NTS association note, which appear in Status, Diagnostics and exported summary. | Set the certificate-time basis to **Not applicable — plain NTP** and remove the NTS-history claim. The source detail renderer already does this correctly for individual NTP rows. |
| C-04 | Medium | `correction-expired` has source rows Waiting/Expired evidence but inherits 3/4 agreement, 3 operator groups and Minimum + majority met. | Set no current agreement/fresh agreement required, or explicitly identify every retained count as historical. Do not leave the evidence card in present-tense agreement while all eligible rows wait. |
| C-05 | Medium | `signed-conflict` says NTS not established but inherits `authNote = Packet authentication is verified`. | Explain signed bootstrap conflict separately from NTS packet authentication; NTS is not yet established. |
| C-06 | Medium | `resume` invalidates mapping/qualified age but retains recent source reply ages and the base 3-minute applied age. | Identify replies and correction as prior-generation/history with unknown current ages, unless a coherent independent elapsed-time basis is explicitly supplied. Do not turn reset fixtures into new freshness. |
| C-07 | Medium | `observe` sets Clock ownership to **Read-only observation**, which is a mode rather than the owner. | Give a concrete known fixture owner or Unknown. State that observe-only does not automatically re-enable Windows Time. Use **No correction by this profile** without conflating ownership with permission. |
| C-08 | Medium | `diagnostics()` always says **Normal logging is active** and Service scope INFO, including a stopped or status-disconnected service. | Label these configured/default profiles, or show effective collection unknown/unavailable for those states. A logging policy is not proof that its sink/process is operating. |

These repairs implement existing architecture §§6, 7.2.1, 9, 10.2.1–10.3, 12.3 and feature AC-02/05/08/11–13; they adopt no new product policy.

## Fixture coverage

| Fixture | Result of source inspection |
| --- | --- |
| healthy | Coherent sample identities, sign, interval, agreement and acknowledged correction. |
| acquiring | No estimate/no agreement; valid authenticated replies may exist before a qualified estimate. |
| holdover | Core semantics coherent; describe NTS authentication as last accepted evidence for best clarity. |
| expired | Estimate withheld and no current agreement; historical source information remains distinguishable. |
| offline | Correctly reports no replies without claiming a proven firewall/network cause. |
| conflict | 2–2 split, no majority, no new correction; no trust downgrade. |
| stale | C-02 and C-08. |
| service-stopped | Confirmed stop separated from disconnected status; C-08. |
| blocked | Useful time observations remain separate from clock ownership. |
| resume | C-06. |
| observe | C-07. |
| ntp | C-03 plus root's already-known Delta reason repair. |
| slewing | Offset +12.4 ms and interval +7.2…+17.6 ms agree; active-time acknowledgement is distinct; no invented ETA. |
| leap-unknown | Correctly allows healthy tracking with a visible qualifier. |
| leap-guard | Correctly withholds current interval, requires fresh post-event agreement and avoids a synthetic second. |
| roughtime | Future label, no NTS agreement or preliminary clock change; bootstrap role retained. |
| reserve | Future/unauthenticated labels, downstream NTS gates and signed-contradiction retention are present. |
| recovered | Retains unauthenticated ancestry after current NTS authentication. |
| signed-conflict | C-05; no reserve-based conflict erasure. |
| correction | +372000 ms = +372 s; ±8 ms and +371.992…+372.008 s interval match 10:06:11.992…10:06:12.008 UTC for 10:00:00 UTC baseline. All approval controls are inert. |
| correction-expired | C-04; historical proposal is explicitly non-approvable. |
| outcome-unknown | C-01; no blind retry control is present. |
| debug-partial | UI-only fixture collection and unauthorized/unconfirmed service collection remain distinct. |

## Important positive findings

- Protected sync/apply/correction/logging controls are disabled; no UAC imitation or fake administrative success appears in these files.
- Scene changes are outside the product window; every dialog has explicit sample-data context.
- Current explicit-helper baseline and future Roughtime/automatic reserve are distinguished in Recovery and Settings.
- NTP, recovery helpers and catalogue candidates cannot join the ordinary NTS vote set merely through preview edits.
- Leap-unknown and leap-guard are different states; LI=0 is not labelled a fault, and OS event execution is not claimed.
- Correction example arithmetic and sign match the canonical public convention. Expiry is not a failed/executed step.
- Export is explicitly fixture-only and retains current scenario/evidence provenance.
- Maintenance follows the complete-MSIX preparation direction and does not claim Store acceptance.

## Remaining limits

Fixes above should be re-inspected against the integrated files. No claim is made about CSS layout, browser event behavior, screen readers, actual Windows clock semantics or product AC completion. The model's scalar symmetric uncertainty is acceptable as a labelled illustrative fixture; it is not a production interval-model decision or guaranteed UTC bound.

## Focused recheck after C-01–C-08 repairs

The eight reported repairs are present in the source; scenario amendments run before source/event derivation. NTP basis, expired/unknown-effect agreement, prior-generation ages, observe-only ownership, bootstrap authentication note and logging availability are now explicit. Two narrow presentation follow-ups were sent to the coordinator:

- A historical **aggregate estimate** in the conflict fixture must not mark its fresh authenticated **source replies** as prior-only. Scope historical source labels independently so the 2–2 authenticated-but-rejected case stays visible.
- `recovery()` must not label every non-future scene **Healthy NTS stops here**. The reference recovery view opened from NTP/stale/expired must have no asserted current NTS stage.

No additional broad audit or execution was performed. Root-reported 20/20 model tests and 258 structural DOM variants are separate coordinator evidence, not observations of this reviewer. Final semantic disposition is conditional only on these two presentation corrections; browser/native qualification remains unperformed.
