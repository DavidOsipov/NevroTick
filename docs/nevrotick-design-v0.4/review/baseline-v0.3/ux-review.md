# NevroTick UI refresh — user journeys and information architecture review

**Review date:** 5 November 2026, Asia/Tbilisi  
**Source candidate:** `8a66af924c509e898a9bab9bb3282f11b24fba93`  
**Reviewer scope:** source-based UX review of the documentation and v0.2 HTML/React prototype. No repository files were changed.  
**Evidence limit:** the current browser could not load the local preview. These findings distinguish observable source contradictions from design recommendations; this is not a freshly rendered visual audit, native Windows test, or usability study.

## 1. Overall finding

The existing design has a sound foundation: it distinguishes source authentication from agreement, explains offset direction, separates machine changes from user preferences, retains recovery provenance, and treats service connection loss separately from confirmed service failure. Replacing that model with a generic security dashboard would lose useful work.

The important refresh is to make the newer architecture understandable through the existing small utility. V0.2 still presents MSI maintenance, only plain-NTP date helpers, and no leap-transition or scoped diagnostic-collection experience. Its correction scenario also contains inconsistent fixture values. The refreshed artifact should repair these contradictions, make evidence and recovery easier to inspect, and retain an unmistakable offline-preview boundary.

Keep **English application copy** and **Russian reviewer instructions/scenario labels**. This is explicit in the existing prototype instructions, UI specification, and feature plan; the user has not requested a product-language change. A Russian explanation file can guide David through the changes without silently adopting Russian localization.

## 2. Sources and authority

All paths below refer to the pinned candidate above. Main documents take precedence over archived copies in the design bundle.

| Ref | Source / relevant section | What it controls |
| --- | --- | --- |
| UX-S1 | `AGENTS.md`, Current UI milestone; Architectural constraints | Fixture-only scope; sample labels; deferred native D1–D3; current architecture precedence |
| UX-S2 | `tasks/ui-m0.md`, Acceptance and checks; Deferred work and re-entry | M0-01–08; local edits only; no real sync/admin success; native evidence limits |
| UX-S3 | `docs/windows-time-features.md`, F-01–F-10, §§3–6, AC-01–20 | Current product journeys, information hierarchy, permissions, status and scope |
| UX-S4 | `docs/nevrotick-design-v0.2/docs/02-ui-spec.md`, §§2–9 | Existing composition and interaction reference; retain useful distinctions, correct superseded MSI/recovery text |
| UX-S5 | `docs/nevrotick-design-v0.2/prototype/AGENTS.md` | English product copy, Russian reviewer labels, supplied mascot identity |
| UX-S6 | `docs/nevrotick-design-v0.2/prototype/src/model.js`, `presentation.js`, `NevroTick.jsx` | Actual old fixture values, scene coverage, labels and interaction wiring |
| UX-S7 | `docs/nevrotick-owner-decisions.md`, OD-02, OD-03, OD-05, OD-08–10 | Future bootstrap order, unresolved preset, complete MSIX direction, future DNS, evidence limits |
| UX-S8 | `docs/plans/time-assurance-integration.md`, §§1–3 | Roughtime-first cold bootstrap; conditional Khronos/Ananke reserve; NTS and broker gates; future status |
| UX-S9 | `docs/plans/leap-second-policy.md`; feature plan F-03/04/07/09 and §4 | Silent LI=0 is not a fault; coverage qualifier; guarded transition and fresh reacquisition |
| UX-S10 | `.agents/skills/security/security-native-ui/SKILL.md` and security/testing contracts | Separate presentation from authority; preserve exact-proposal outcomes; bounded redacted diagnostics |

This review used the repository's `security-native-ui` skill and its applicable security/testing contracts. No security or policy parameters were newly selected.

## 3. Prioritized recommendation set

### UX-01 — Make the specimen boundary survive every screen (P0)

**Finding:** the old workbench displays a global demonstration notice, but individual application screenshots and modal dialogs can be viewed outside that surrounding context. Several v0.2 actions simulate successful machine-policy activation. UI-M0 explicitly requires a persistent sample-data label and disables protected effects.

**Change:** place a discreet, permanent **Design preview · sample data** label inside the application shell and in detached dialogs. Product **Sync now**, **Apply**, installation, diagnostic service collection and exceptional-approval controls remain disabled with short visible explanations. Reviewers select future outcomes with a clearly external Russian scenario control. Source editing uses **Update preview / Cancel** and affects memory only. A selected example can show a successful historical outcome, but no protected button pretends it performed the operation.

**Copy:** `This preview does not connect to services or change Windows time.` For local editing: `Updated in this preview. The Windows configuration is unchanged.`

**Acceptance:** open every principal screen and dialog, make a valid local edit, reject a malformed hostname, cancel an edit, and reload the file. No network, persistence, system action, or real-success implication appears. **Mapping:** UX-S1/2/10; M0-02–06. This does not restore native D1–D3.

### UX-02 — Use one evidence fixture for the entire correction journey (P0)

**Finding:** `NevroTick.jsx`'s correction dialog proposes moving forward **6 min 12 s**, while `makeScenePresentation()` only changes `lastAppliedCorrectionAge` for the `correction` state. It therefore retains source offsets around **−0.7 ms** from `INITIAL_SOURCES`. Those values cannot support the same **+372 s** proposal. The current specification §7 already warns about this exact category of contradiction.

**Change:** define each correction scenario as one coherent data object: Windows time, proposed reference, signed delta, freshness, source intervals, proposal identity, current/expired state and actual outcome. Derive Status, Sources, Diagnostics, the review panel and export from that object. Keep illustrative quantities visibly illustrative; do not implement a second clock controller in the prototype.

**Acceptance:** status, source details, proposal and export show the same direction and compatible magnitude. Expired proposals have no usable approval control. A requested or unknown outcome never says **Clock updated**. **Mapping:** UX-S3 F-08, AC-05/11; UX-S4 §7; UX-S6.

### UX-03 — Replace the outdated MSI journey with the selected complete-MSIX journey (P0)

**Finding:** `model.js` and `NevroTick.jsx` still describe a complete signed MSI and Windows Installer handoff. OD-05 and F-01 now select a complete hardened MSIX preparation target and Store submission attempt.

**Change:** maintenance explains one installed product, background services, Store-managed package updates under the qualified lifecycle, and Windows-authorized repair/removal. Keep package/build qualification status in the reviewer notes; do not imply Store acceptance or a working packaged lifecycle. A future UI must name the actually qualified remedy, not promise that Reset or uninstall is always safe. The preview must not offer its own updater or a working installation button.

**Acceptance:** no current-flow text tells the user to obtain the MSI or says Windows Installer will carry out the selected route. Maintenance never claims that listing withdrawal repaired an installed machine. **Mapping:** UX-S3 F-01, AC-15; UX-S7 OD-05; UX-S4 §8 is superseded where it says MSI.

### UX-04 — Add a staged, clearly future date-recovery explanation (P1)

**Finding:** the existing recovery modal supports only one/two plain-NTP helpers. The current owner direction introduces Roughtime as preferred cold-date bootstrap, followed by NTS; Khronos/Ananke is a conditional reserve, not a continuous watchdog or a normal NTS vote.

**Change:** provide a recovery scenario with four comprehensible steps: **Obtain an approximate date → Verify NTS servers → Establish fresh source agreement → Review permitted clock correction**. Show where the process is blocked and whether Windows time has changed. In expanded evidence, show signed Roughtime interval provenance or the explicitly unauthenticated reserve basis. Place this scene under **Planned recovery flow**; retain the current strict/explicit-helper baseline as current and do not imply the future route is shipped.

**Copy:** `A rough date helps verify NTS certificates. It does not set the Windows clock.` For the reserve branch: `Using an unauthenticated date candidate for certificate checks. Windows time has not changed.`

**Acceptance:** recovery success alone cannot make the product claim clock correction; recovering from a hostname/certificate-identity error is not automatically routed to wrong-date recovery. NTS sources and recovery sources are separate lists. Retained recovery provenance remains discoverable after the headline returns to healthy. **Mapping:** UX-S7 OD-02; UX-S8; UX-S3 AC-09–11.

### UX-05 — Add leap knowledge without frightening users away from usable time (P1)

**Finding:** v0.2's specification has a later leap-policy paragraph, but the scene model has no coverage, guard, or post-transition scenario. The absence of a leap announcement is easy to misrepresent as a defective server.

**Change:** keep the normal headline when tracking is usable, with a compact qualifier such as **Future leap schedule unknown** where applicable. Provide a separate **Rechecking time around a UTC transition** example. Details show calendar coverage, compatible source scale, observed versus unknown Windows behavior, and remaining measured correction. A silent unsmeared LI=0 source remains eligible and does not get a warning merely for silence.

**Acceptance:** unknown future schedule does not become **Service failed**. No simulated **+1 second applied** or **Windows handled it** appears without the corresponding example evidence. A contradictory observation is explained at that source; healthy peers are not all marked failed. **Mapping:** UX-S9; UX-S3 F-03/04/07/09, AC-03/04/09/12.

### UX-06 — Preserve a compact status hierarchy (P1, design judgment)

**Change:** keep the user's first reading task to one state, one reason and one next action. Place offset and uncertainty together; place NTS authentication, independent-source agreement and last qualified estimate together. Move correction detail, calendar metadata, certificate-time basis and process details into expandable evidence or Diagnostics. The application is a time discipline utility; it does not need a decorative ticking clock, security score or historic dashboard chart.

Suggested reading order:

1. Profile and state: **NTS only · Time sync active**.
2. Activity/remedy: **Gradually correcting the clock** or the specific blocker.
3. Paired values: **Estimated offset −0.7 ms** / **Estimated uncertainty ±4.8 ms**, with **Windows is ahead**.
4. Evidence: authenticated source data, agreement and qualified-estimate age.
5. Background availability and expandable details.

A missing current status connection must move above the numbers. Retained values receive **Historical** labels and cannot look like fresh green success. Preserve all four freshness fields across the appropriate surfaces; do not collapse them to **Last synced**.

**Mapping:** UX-S3 F-04 and §§3.1/4; UX-S4 §§4/4.1. This is a presentation recommendation, not evidence of improved task-completion time.

### UX-07 — Make source inspection and editing different modes (P1)

**Change:** default Sources to readable source rows with canonical identity, protocol/authentication, Main/Supporting/Not used/Waiting role, age and a short reason. Open a selected row's details without reordering the list. Show operator grouping and scale in those details. Keep original active evidence distinct from an edited preview; adding a source cannot suddenly create a fresh vote. Retain manual NTS/NTP entry, no Auto protocol, and a clear profile-mismatch error.

For a future product draft, place a persistent review strip at the bottom with count and behavioral effect, and show old/new values in review. In this downloadable artifact, use **Update preview** and an external result selector instead of pretending the service applied policy. No **Test server** request is sent for an unaccepted arbitrary address.

**Acceptance:** adding a duplicate or URL produces a field-local remedy; invalid data remains editable and existing good state is preserved. Removing or disabling sources never lowers agreement policy. Sorting rows does not change trust. Roughtime and reserve helpers never appear as Supporting NTS sources. **Mapping:** UX-S3 F-03/06, AC-03/04/07; UX-S4 §5; M0-04–06.

### UX-08 — Separate two classes of Settings and keep Observe-only precise (P1)

**Change:** group **Time sources & trust**, **Clock correction / recovery**, and **This window**. Machine settings carry an administrative-authority explanation; ordinary window preferences do not share the machine-policy Apply state. Show **Active clock mode** separately from any proposed mode. Observe-only says **Clock changes by NevroTick are disabled** and never implies Windows Time was restarted.

Keep tray and tray-at-sign-in as distinct opt-ins. Explain that service startup is independent. Do not turn the HTML theme switch into evidence that native D1 is implemented; if retained for visual review, label its scope. No forced onboarding wizard is needed after a successful complete setup; show exact missing setup action in the empty state.

**Mapping:** UX-S3 F-01/06/10, §§5–7, AC-07/08/18; UX-S4 §§2/6; native D1/D2/D3 remain deferred.

### UX-09 — Make Diagnostics a clear remedy and evidence surface (P1)

**Change:** start with **Current issue → What it affects → Suggested next action**. Then show ordered events, evidence/freshness and optional technical details. With the expanded recovery/leap/collection content, a fourth discoverable main navigation destination is a reasonable design improvement; a large nested modal is no longer necessary. Keep About secondary.

Add a source-level reason for **Server requested slower polling** / **Access refused** where relevant; a control response is not a time measurement. Temporary DEBUG collection must show scope, remaining duration and complete/partial/failed state. Service collection is an administrative action; user UI diagnostics have their own scope. In this prototype, describe/illustrate these states and disable the service-control effect.

Keep **Copy summary** and **Export diagnostics** as local, user-initiated actions with a content preview. Redact private identities by default. Explicitly exclude keys/cookies and do not add upload or telemetry. Ordered events cannot rely solely on the possibly wrong Windows date.

**Acceptance:** disconnection says the service's current health is unknown; a confirmed failure uses its distinct evidence and remedy. An unavailable sink does not mean synchronization failed, and a collection request is not called a successful completed capture. **Mapping:** UX-S3 F-02/09, AC-05/16; UX-S4 §8; UX-S10.

### UX-10 — Fix browser-specific window metaphors (P2)

**Finding:** the v0.2 **Minimize** button calls `setClosed(true)` and displays **Окно закрыто в макете**, contradicting the UI specification's distinction between minimize and exit.

**Change:** either omit nonfunctional window-chrome controls in the web specimen, or model minimized and closed states distinctly. Minimize retains the draft and restores the same page. Closing the specimen cannot claim to have stopped a service. Do not fake a Windows secure-desktop UAC prompt or collect an administrative password.

**Mapping:** UX-S4 §2; UX-S6 final shell render; M0-03 and AC-01/18 remain separate native observations.

### UX-11 — Make review easy without expanding product scope (P2)

**Change:** replace the long always-visible 43-scene rail with a compact external grouped scenario selector and **Сбросить пример**. Suggested groups: everyday use, degraded confidence, recovery, settings outcomes. Keep scenario/result controls outside the product shell and explain each in Russian. Choose a small representative walkthrough as the default, while keeping edge scenarios discoverable.

The downloadable primary artifact should open directly as a self-contained HTML file with embedded final AVIF brand assets, no CDN, no package installation and no live endpoints. Include readable source, recommendation summary and provenance notices in a ZIP. The browser-verification limitation must be stated in the handoff if it remains unresolved.

Do not add active DNS, Android, world-clock, graph, watchdog, updater, force-time, ignore-certificate or formal-proof-certification features to fill space. Their research or future-roadmap presence does not make them current user actions.

**Mapping:** UX-S1/3/5/7; UX-S3 §10; design judgment for reviewer ergonomics.

## 4. Minimum scenario walkthrough for the refreshed deliverable

| Step | Scenario | User question answered | Essential evidence |
| --- | --- | --- | --- |
| 1 | Normal tracking | Is it operating, and on what basis? | Authentication, agreement, estimate age, offset and uncertainty |
| 2 | Source detail | Why was this server excluded? | Authenticated-but-inconsistent is distinct from unauthenticated |
| 3 | Local source editor | Can I correct an input mistake safely? | Invalid host preserved; Update preview only; Cancel restores previous sample |
| 4 | Holdover and expiry | Does an outage make the old estimate useless immediately? | Valid bounded model versus expired estimate; no invented recovery deadline |
| 5 | Status unavailable | Did the service actually stop? | Historical snapshot; current service health unknown |
| 6 | Future date recovery | What does Roughtime do before NTS? | Coarse date, NTS checks, fresh agreement, separate broker permission |
| 7 | Large correction / expired proposal | What exact change would need approval? | One consistent target/delta/evidence; disabled expired action |
| 8 | UTC transition | Does missing leap information break all synchronization? | Coverage qualifier; guarded reacquisition; measured residual |
| 9 | Settings / sample outcome | What changes require an administrator? | Machine versus user preferences; outcome selected externally |
| 10 | Diagnostics / local export | What can I inspect or send to support? | Visible redacted content; no automatic transmission |

## 5. Handoff checks and limits

- Confirm every fixture's headline, source list, values, recovery record and export belong to the same scenario. The +372-second example above supplies a discriminating consistency check.
- Confirm all actual prototype mutations remain in-memory, except explicitly requested local export downloads. No effect path reaches NTP/NTS/DNS, services, UAC or clocks.
- Confirm English product copy/Russian reviewer copy, current complete-MSIX wording, final supplied mascot identity and preserved brand notices.
- Inspect representative layout widths and interactions when browser access becomes available. Without that access, label render/focus/overflow/browser compatibility as unverified rather than infer them from source.
- Do not claim native Narrator/UIA, mixed-DPI, Windows high contrast, service behavior, Store acceptance, WCAG conformance or mathematical proof coverage from this design artifact.

**Recommended implementation order:** coherent scenario data and fixture boundary; status/sources; updated recovery and leap states; settings/diagnostics/maintenance; package and review guidance. Keep the existing accepted trust semantics while simplifying how the user reaches the explanation.
