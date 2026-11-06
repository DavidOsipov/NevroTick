# NevroTick UI refresh — architecture and trust semantics review

**Review date:** 5 November 2026 (Asia/Tbilisi).  
**Repository candidate:** `8a66af924c509e898a9bab9bb3282f11b24fba93`.  
**Scope:** read-only comparison of current canonical documents with the v0.2 UI specification and React presentation source. This is a source-contract review, not a visual, native Windows, accessibility or security qualification.  
**Output owner:** architecture review agent. No repository files or host settings were changed.

## Finding and design direction

The existing v0.2 design already gets several important distinctions right: NTS authentication is separate from source agreement; a recent reply need not be an accepted measurement; historical snapshots are labelled; ordinary Sync now does not grant clock authority. Preserve those decisions. The main new work is to bring the design up to date with the selected MSIX delivery direction, the future Roughtime-first cold-bootstrap flow, the selected leap policy and later durable-outcome semantics, while making those meanings easier to understand.

Use a small everyday application with **Status, Sources and Settings**, plus an easily reachable **Diagnostics** utility. Put exceptional recovery behind a contextual action or a secondary Sources/Settings surface. Do not turn the core experience into a constantly rendering chart dashboard or make Roughtime/DNS separate everyday operating modes. This follows architecture §16 and features §3; it is a design recommendation, not a new toolkit decision.

A downloadable browser prototype is a review artifact. It does not supersede the native UI-M0 task, authorize live integrations or demonstrate Windows qualification.

## Priority recommendations

| ID | Priority | Recommendation | Basis and scope |
| --- | --- | --- | --- |
| UA-01 | P0 | Keep a persistent **Design preview · sample data** notice. Put scenario selection outside the simulated product window. All product operations with network, service, policy or clock effects remain unavailable; local source editing says **Update preview**. | Architecture §16.5; `tasks/ui-m0.md`, M0-03/04/05/06. The existing HTML includes broader simulations, but its rendered scenes are not behavioral authority. |
| UA-02 | P0 | Show **time evidence**, **clock action** and **service availability** as separate facts. A working connection or authenticated packet cannot make the main status healthy by itself. | Architecture §§6, 9, 16.2; F-04; AC-05; CHECK-14/25. |
| UA-03 | P0 | Preserve four distinct ages: last valid source reply, last qualified agreeing estimate, last acknowledged applied correction, and status snapshot age. A fresh UI render must not reset these. | Features §4; architecture §§6, 7.2.1; AC-02/05. Existing v0.2 has these semantics; retain them through the redesign. |
| UA-04 | P0 | Model NTS, plain-NTP and observe-only deliberately. Plain NTP stays visibly unauthenticated. Observe-only does not imply that Windows Time has been re-enabled or owns the clock. | F-02/06; architecture §§7.5, 9–11; AC-04/08/14. |
| UA-05 | P0 | Add clearly identified **future recovery design** scenes for the Roughtime-first cold-bootstrap direction, with an explicitly unauthenticated conditional NTP reserve. Do not present that flow as implemented/default-on. | OD-02 §4.1; architecture §8.5; NV-TA-001 §§1, 4, 6. Current explicit-helper baseline remains until extension qualification. |
| UA-06 | P0 | Add leap-knowledge qualifiers and a guarded transition scene without treating silent LI=0 peers as faulty. Do not offer a leap-second switch or an “apply one second” button. | OD-09; architecture §10.5; NV-LP-001 §§1, 4–6; AC-03/04/12; CHECK-14. |
| UA-07 | P0 | Replace MSI-specific maintenance/repair copy with the current complete-MSIX/qualified-package lifecycle. Do not add a custom updater or claim Microsoft Store approval. | Amended OD-05; F-01; architecture §§11, 19.4–19.5; AC-15. v0.2 `src/model.js` and `src/NevroTick.jsx` still explicitly say MSI/Windows Installer. |
| UA-08 | P1 | Distinguish correction requested, in progress, acknowledged, declined/expired and outcome unknown. An unknown result must not expose a “try the correction again” shortcut. | Architecture §§10.2–10.4; F-08; AC-11/13; CHECK-10/12/15. Future/static design scenes only; no fake UAC or completed transaction in UI-M0. |
| UA-09 | P1 | In Sources, separate current selected sources, the optional catalogue of candidates and recovery-only evidence. Use **Main / Supporting / Not used / Waiting**, stable rows and explicit reasons. | F-03/06; features §3.2; architecture §7.2; catalogue draft §§1.1–2. |
| UA-10 | P1 | Show operator independence and time-scale provenance in source details. A friendly name, fast ping, several IPs, good historical score or user-typed group does not create an independent vote. | Architecture §§6–7; OD-03; F-03; catalogue DATA inventory. |
| UA-11 | P1 | Put useful problem/remedy text before diagnostic internals. Offer a redacted preview/export and separate temporary UI logging from administrator-authorized service collection. | F-09; architecture §12.3; AC-16; CHECK-17. Live logging/session control is later implementation. |
| UA-12 | P1 | Keep future DNS, NTS pinning and experimental storage/scoring out of the active-product feature set. If shown for review, label them **Planned** and explain their limited role. | OD-08; features §10; catalogue draft §1.1. They are neither installed capabilities nor requirements for current native UI-M0. |
| UA-13 | P1 | Keep the primary action understandable: **Sync now** means a fresh check under the existing policy, not a force-time command. Show coalesced/limited/unavailable outcomes in design scenes. | F-05; architecture §§5.3, 16.2; AC-06. Native UI-M0 keeps the action disabled. |
| UA-14 | P2 | Preserve English product copy, explicit units and one offset sign convention. Russian can be used for review controls and explanatory documentation. No invented accuracy percentage, correction threshold or countdown SLA. | Architecture §§6, 16.4; features §§3.1, 5, 8. Russian product localization remains unadopted. |

## Proposed screen contents and copy

### Status: an answer before technical detail

Read in this order: profile and headline → current action or obstacle → evidence → service availability → actions. Use **Time sync active**, not “Time is correct”, “100% secure” or a single green shield. An illustrative healthy layout can state:

- **NTS only · Time sync active**
- **Gradually correcting the clock** or **Waiting for the next scheduled check**
- **Estimated offset +0.7 ms — Windows is behind**
- **3 independently operated sources agree; configured minimum and majority met**
- **Last qualified estimate: 18 seconds ago**

All numbers and identities must be visibly demonstrative. Where operator independence has not been established, say **3 sources agree** and expose the missing independence qualification rather than upgrading the claim. A plus sign means reference time minus Windows time; negative means Windows is ahead. Preserve that convention in source rows, details, correction review and exported diagnostics.

Keep model uncertainty near the estimate, with an explanation that it is conditional on the admitted measurement/fault model. Do not label it a guaranteed maximum UTC error or attach a fabricated confidence percentage. If only a prior model is available, say **Model estimate**; do not leave an apparently fresh measured offset in place.

A compact evidence disclosure can contain the last applied correction, certificate-time basis and snapshot age. Source-level last-reply/last-accepted ages belong in Sources. This avoids both hiding the distinctions and putting every technical field in the main reading path.

### Required state distinctions

| Review scene | User-facing meaning | Must not imply |
| --- | --- | --- |
| Healthy tracking | Current policy is met; show any ongoing correction separately. | NTS guarantees perfect UTC or no correction remains. |
| Acquiring | Waiting for enough current admissible evidence. | A service starting or packet arriving establishes synchronization. |
| Holdover/offline | **No fresh time check — using the previous model.** Age/uncertainty increase. | Fresh agreement, indefinite accuracy, automatic NTP/Windows fallback. |
| Expired | **No usable current time estimate.** Bounded retries continue. | Zero offset or a healthy state derived from a live service. |
| Conflict | **Time sources disagree.** No new correction from disputed observations. | Any two of four sources suffice; a helper can arbitrate NTS disagreement. |
| Status unavailable | **Showing the last received snapshot; current service health unknown.** | The service definitely stopped. |
| Confirmed failure | Service stop/failed recovery established separately; provide qualified package-repair guidance. | A disconnected status pipe alone proves failure or cleanup success. |
| Observe-only | Reference time is measured; product clock changes are disabled. | Automatic handback to Windows Time. |
| Resume/reacquisition | Old-generation work no longer counts; fresh observations are needed. | Sleeping time equals delivered active clock adjustment. |
| Correction outcome unknown | An earlier operation lacks a durable final result; follow reconciliation. | A second click can safely repeat the relative step. |

Sources: architecture §§6, 9, 10.2.1–10.4; features §4 and AC-02/05/08/11–13.

### Recovery: separate evidence acquisition from clock permission

Current documents contain two levels of policy. The baseline uses retained/strict NTS first and explicitly enabled plain helpers for certificate-date circularity. The owner has selected a **future** extension with Roughtime preferred for cold bootstrap and an automatic conditional Khronos/Ananke-inspired NTP reserve. The latter has unresolved source/default, cryptographic, Windows-verifier and envelope qualification; a polished mock must not erase that distinction.

For review, represent the future flow as these compact stages:

1. **Check existing secure time context.** Usable cached/strict NTS avoids unnecessary bootstrap work.
2. **Establish an approximate date.** Signed Roughtime intervals from sufficiently independent admitted operators; otherwise an eligible, bounded **unauthenticated reserve**.
3. **Verify and compare NTS time.** Full certificate identity/trust/revocation policy, fresh protected observations, configured agreement and candidate-time revalidation.
4. **Review any exceptional clock correction.** Separate broker/administrator authority; no preliminary global clock step.

Suggested active-scene copy: **Checking NTS servers using a signed approximate date. Windows time has not changed.** Reserve copy: **Checking NTS servers using an unauthenticated date estimate. Windows time has not changed.** Neither is ordinary tracking.

After recovery, retain a discoverable **Recovery basis** record with its source, assurance and age. A plain reserve does not become authenticated retroactively because NTS later succeeds. Roughtime-conditioned NTS observations are not an additional independent vote over their own ancestry. **Reserve not needed / not assessed** is a correct healthy state; it must not be shown as an independently agreeing or continuously protecting monitor.

Separate no replies/too few Roughtime operators, invalid signatures, signed disagreement and NTS date incompatibility from unrelated name/chain/revocation/network/agreement failures. Do not classify every TLS failure as a bad date. In a compact UI, show the immediate effect and next action; put this detailed evidence in Diagnostics.

### Leap policy: precision without needless alarm

Selected policy permits ordinary same-segment tracking when future leap coverage is unknown, with a visible qualifier. A small line such as **UTC metadata: future leap schedule unknown** can accompany otherwise healthy tracking; it need not turn the entire application red. Around an ambiguous boundary use **Rechecking time around a UTC transition** and state that uncertainty is increasing until fresh agreement returns.

Source details should distinguish **Time scale: unsmeared UTC, provenance admitted**, **Announcement: none received** and **Leap claim: contradictory/rejected**, when those facts exist. LI=0 is not a defect. Unknown time scale and missing leap announcement are different facts. Raw LI bits and native API names are diagnostics, not mandatory everyday vocabulary.

Windows owns explicit leap events. Do not promise that Windows performed an event unless actual native evidence establishes it. After the guard, report only the measured residual correction; there is no synthetic ±1-second action. The selected effective-dated continuous-UTC rule belongs in expandable metadata if useful; it is not a new time zone, epoch reset or 37-second correction.

### Sources and candidate catalogues

Retain stable source rows with protocol/authentication, role, freshness and a human reason; keep offset, delay, uncertainty, operator and scale in details. A source may be **NTS authenticated · Not used** because its measurement disagrees. In draft/configuration preview, use **Not checked** or **Waiting**, never infer authentication from choosing NTS in a selector.

The new catalogue design anticipates hundreds of records; it does not increase the ordinary active-source population or approve every candidate. If a catalogue browser is useful for review, make **Available candidates** and **Active configuration** unmistakable. Adding a candidate to an in-memory preview does not establish fleet-use permission, runtime admission, operator independence or a source's current health. Use reserved example identities for fixtures or explicitly identify actual names as non-queried sample data.

Roughtime keys/profile/endpoint provenance belong to the separate future bootstrap detail surface. The ordinary Add NTS source form should not become a general cryptography editor. NTS certificate pinning is still a proposal and should not appear as an accepted security toggle.

### Settings, maintenance and diagnostics

Group machine policy separately from per-user preferences. Keep raw step/rate knobs, arbitrary “force time”, ignore-certificate-errors, reset-security-budgets and generic service restart commands out of the basic UI. They are not authorized v1 surfaces.

The static preview may show disabled protected settings and explanatory review scenes, but no fake UAC prompt or successful service apply. Local editor validation can work and preserve prior values on error. D1/D2/D3 native work remains Deferred; browser usability improvements establish no native theme, Narrator, UIA or mixed-DPI pass.

Maintenance uses the complete selected package. Do not retain v0.2's **Continue with the complete signed MSI** instruction. **Open installed-app maintenance** is only a proposed action until its supported route is qualified; in this preview it should be explanatory or unavailable. A Store badge/signature cannot represent service authorization, application security, complete lifecycle qualification or Store acceptance.

Diagnostics should start with **What happened → What it affects → What to do next**. Copy/export is user initiated and has a redacted preview. Show unavailable data as unavailable rather than zero; export should identify the fixture/scenario and contain no real credentials or user identity. Temporary service DEBUG collection needs explicit administrator authorization, effective scope, remaining duration and honest partial results; a local UI log preference is a different operation. Do not add a live logging controller to UI-M0.

## Concrete review cases for the refreshed artifact

These checks are for the downloadable design artifact and do not close the native product AC/CHECK gates:

1. Every scene and standalone dialog retains sample-data context; switching scenarios cannot be mistaken for a real service result.
2. A healthy scene, holdover, expiry, disagreement, disconnected snapshot and confirmed service failure have distinguishable wording and evidence.
3. The NTP scene keeps an unauthenticated label; observe-only keeps clock ownership separate.
4. Recovery scenes keep Roughtime, plain reserve and NTS roles separate and never count the reserve as an NTS supporter.
5. An unknown leap schedule can coexist with healthy tracking; silent LI=0 is not a source fault; guarded recovery has no invented one-second adjustment.
6. Source editing validates locally, preserves accepted values on invalid input, supports Cancel and says **Update preview** rather than service success.
7. A correction scene shows direction/interval, evidence freshness and one-time scope, but no operative approval. An unknown outcome never invites blind replay.
8. Maintenance/about text no longer presents MSI as the selected route; it claims no qualified release or Store approval.
9. Sources/statistics/catalogue labels cannot turn a historic score, convenient country or repeated operator into extra independent votes.
10. All product actions remain inert with respect to services, networking and host time; no native, formal-proof, ASVS or accessibility certification is inferred from this artifact.

## Source map

All source references below are pinned to the reviewed candidate:

- [AGENTS.md](https://github.com/DavidOsipov/NevroTick/blob/8a66af924c509e898a9bab9bb3282f11b24fba93/AGENTS.md) — current UI milestone and authority order.
- [Current architecture](https://github.com/DavidOsipov/NevroTick/blob/8a66af924c509e898a9bab9bb3282f11b24fba93/docs/windows-time-architecture.md) — §§6–12, 16, 19–20.
- [Current feature plan](https://github.com/DavidOsipov/NevroTick/blob/8a66af924c509e898a9bab9bb3282f11b24fba93/docs/windows-time-features.md) — F-01–F-10, §§3–5 and AC-01–AC-20.
- [Owner decisions](https://github.com/DavidOsipov/NevroTick/blob/8a66af924c509e898a9bab9bb3282f11b24fba93/docs/nevrotick-owner-decisions.md) — OD-02, OD-03, amended OD-05, OD-08–OD-10.
- [Native UI-M0 task](https://github.com/DavidOsipov/NevroTick/blob/8a66af924c509e898a9bab9bb3282f11b24fba93/tasks/ui-m0.md) — M0-01–M0-08 and native deferrals.
- [Time-assurance integration](https://github.com/DavidOsipov/NevroTick/blob/8a66af924c509e898a9bab9bb3282f11b24fba93/docs/plans/time-assurance-integration.md) — future bootstrap roles, priority, evidence ancestry and failure decisions.
- [Leap policy](https://github.com/DavidOsipov/NevroTick/blob/8a66af924c509e898a9bab9bb3282f11b24fba93/docs/plans/leap-second-policy.md) — selected silent-source/guard/Windows contract.
- [Catalogue and statistics draft](https://github.com/DavidOsipov/NevroTick/blob/8a66af924c509e898a9bab9bb3282f11b24fba93/docs/plans/server-catalogue-and-statistics-draft.md) — candidate catalogue versus active/admitted sources, authority and status.
- [v0.2 UI specification](https://github.com/DavidOsipov/NevroTick/blob/8a66af924c509e898a9bab9bb3282f11b24fba93/docs/nevrotick-design-v0.2/docs/02-ui-spec.md) and [presentation source](https://github.com/DavidOsipov/NevroTick/blob/8a66af924c509e898a9bab9bb3282f11b24fba93/docs/nevrotick-design-v0.2/prototype/src/presentation.js) — existing distinctions to preserve; illustrative, subordinate to current contracts.

**Evidence limits:** no application execution, screenshots, live source queries, Windows APIs, Store operation, accessibility testing or statistical/proof verification was performed by this reviewer. Exact UI color, density and interaction quality must be assessed by the visual/interaction reviewers and on the integrated artifact. The recommendations do not resolve pending production provider, quorum, correction-bound, package-lifecycle or native implementation decisions.
