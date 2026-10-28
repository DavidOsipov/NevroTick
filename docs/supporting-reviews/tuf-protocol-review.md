# TUF protocol, time and recovery assessment

**Assessment date:** 28 October 2026. **Reviewer:** /root/tuf_time_model, independent of the proposal and accepted architecture. **Scope:** the supplied issue-888 proposal and RWTC-ARCH-001 revision 1.4, particularly sections 6–14, 18, 19 and OPEN-8. This is a review record, not a revision to the accepted architecture or its threat model. No dependency is selected.

## Result

**The proposal has a sound reason to evaluate TUF, and already states the important trustworthy-clock precondition. It does not yet establish how this particular time-repair application would satisfy that precondition.** TUF can improve release authorization, repository consistency, persistent replay resistance and key administration. It cannot independently supply correct current UTC or repair the updater's own trust assumptions.

The next stage can be a bounded policy and source assessment. **Adoption or implementation is not ready on this evidence:** the time authority, state ownership and recovery route remain decisions. In particular, revision 1.4 still specifies complete signed-MSI updates and excludes a bespoke privileged auto-updater. This assessment does not change that decision. [A §§14.1, 19.1, 19.5; P]

Initial observations were preserved in findings.json at 14:20:42 UTC, before peer discussion. They remain in that record. All runtime controls and tests discussed below are proposed; none was executed.

## 1. Protocol facts that constrain the evaluation

The following is a compact statement of the inspected specification, **version 1.0.37, dated 8 October 2026**:

- Expiry uses one fixed start time.
- Roots advance by one, meeting predecessor and successor thresholds of distinct keys. Expired installed/intermediate roots permit rotation; each successor is persisted. The final root must expire strictly after the start.
- Timestamp/snapshot key rotation deletes both trusted timestamp and snapshot state. Ordinary errors do not grant that reset.
- Role versions constrain replay; required metadata persists. Equal timestamp versions stop normally; lower versions fail. Snapshots constrain referenced versions and names.
- Root-threshold compromise needs out-of-band recovery. Bogus-clock support remains unresolved; no trusted-time bootstrap or expiry bypass is specified.

These facts come from TUF §§2.1.1, 5.1–5.6, 6.1 and 7.1. [S1]

**Interpretation:** qualify expired-root recovery and preservation of accepted progress after later failure. The proposal correctly defers to protocol ordering; a blanket rejection of every expired intermediate or whole-workflow state rewind would misapply it. This is a qualification detail, not an observed bug. [P; S1]

**Freeze and rollback are separate failures.** Rollback concerns an older repository view than retained knowledge; freeze concerns withholding progress. A client can retain every known version and still be frozen at that same view. Expiration provides a time-dependent bound on this condition; it cannot force an attacker to deliver a release. An authentic, currently admissible view also does not prove that no newer publication exists elsewhere. The TUF security documentation explicitly separates these attack classes and delivery limits. [S2]

An updater should therefore distinguish “refresh found no change,” “freshness cannot be established,” “metadata is expired,” and “version rollback detected.” Treating a routine equal-version result as proof of indefinitely current metadata would be an integration error. The exact library's cached-target and refresh behavior needs inspection and a planned fixture; this is not a claim that its implementation has that error.

## 2. Time assurance is an application admission question

Revision 1.4 deliberately separates packet authentication, certificate-time basis, source agreement, current quality, age and bootstrap history. That separation should survive any updater evaluation. An authenticated packet is not a proof of correct UTC, and a status field supplied by the engine is not independently verified evidence. The accepted threat model includes a compromised engine. [A §§6.3, 7.4, 8.3, 14.2–14.4]

**Authority consequence:** accepting an engine-provided interval would make that engine an authority for update freshness. Reading OS time instead does not automatically remove this dependence, because the engine can influence OS time through the broker. The broker limits corrections; it does not certify true UTC. A compromised engine still cannot invent target signatures merely by lying about time, but it may undermine freshness or help keep an already authorized vulnerable release usable. The owner must explicitly decide whether the existing time trust model is sufficient, an independent authority is necessary, or only independently authorized maintenance is acceptable. This assessment selects none of those architectures and does not assume a second time verifier exists. [A §§6.3, 14.3–14.4]

RFC 8915 discusses the certificate-time circularity, retained time, consistency with certificate validity, multiple sources, and residual delay attacks. It provides mitigations, not a perfect initial-time solution. Reusing a valid cached association can avoid a new certificate-time handshake, but does not make every server's claimed time correct. [S3 §§8.5–8.7]

The table gives **candidate admission conditions for evaluation**, not a selected policy. “AuthenticatedNts” is a descriptive category here, not a Rust type established in revision 1.4.

| Time basis | What the existing plan establishes | What a future updater would still need |
| --- | --- | --- |
| Fresh AuthenticatedNts from strict or previously qualified cached associations | Authenticated exchanges, source-policy gates, age and uncertainty are intended controls | A specified authority that can supply a current interval, its assumptions and age limit, and a decision about reliance on the potentially compromised engine |
| NTS reached through plain-bootstrap or operator-approximate certificate time | A deliberately weaker origin remains in provenance after the full recovery gate | An explicit decision whether that origin is sufficient for update freshness; a later successful handshake must not silently erase it |
| Holdover in a qualified continuity interval | Prior model survives only within age/error policy; uncertainty grows | An upper as well as lower UTC bound that includes elapsed time and drift; supported suspend behavior and an exhaustion rule |
| Saved authenticated floor after an unknown powered-off interval | A conservative lower-bound hint under stated assumptions | Another source of an upper bound; the floor alone cannot prove metadata is still current |
| Plain NTP helper, release date, HTTP date, or ordinary OS reading already judged unreliable | No authenticated current-time assurance | They cannot acquire update-freshness authority merely through plausibility or a signed-looking label |

TUF's timestamp role must not be treated as a time service. A publisher-chosen deadline or release date is not evidence of the time at which the client receives it. The same distinction applies to a code-signing timestamp: evidence about signing does not itself establish current UTC. This is a reasoning consequence of the information these values contain, not a new TUF mechanism.

### 2.1 A conservative bounded-time option

Suppose an independently justified model says that actual UTC at update start lies in **[L0, U0]**, and a metadata deadline is **E**. A conservative additional admission condition is:

**Accept freshness only when E > U0.**

If E is at or below L0, the object is expired throughout the interval. If L0 < E ≤ U0, freshness is uncertain and should not be asserted. If only a floor is known, U0 is unbounded and the condition cannot pass. Choosing the interval midpoint, clamping the OS clock to a saved floor, or taking the maximum of two historical lower bounds does not solve this problem.

This is elementary interval reasoning applied to the proposal, **not normative TUF behavior or a standardized trusted-time extension**. The ordinary protocol still needs its fixed start value and normal checks. An application may impose stricter admission on top; whether a library can expose the required seam without changing protocol behavior must be established from its actual source. Calling an interval “trusted” is insufficient: its error bound is conditional on the source, network and oscillator model. A statistical confidence interval must not be advertised as an unconditional security bound.

The application would also need a finite monotonic workflow budget. It should not retain a supposedly fixed start indefinitely across crashes, sleep or an abandoned download. If the admitted time model changes generation or proves inconsistent, a conservative policy can abort and begin a fresh workflow under a new admitted time basis. It should not silently replace the start time partway through the existing workflow. An additional installation-age condition, if wanted, is an application policy requiring its own evidence; it is not a retroactive reinterpretation of the protocol's start-time checks.

### 2.2 Consequence of the independently reported tough clock behavior

After the independent first pass, the coordinator reported another reviewer's source observation and subsequently confirmed an independent source recheck: tough 0.24.0's datastore time method reads the system clock, retains latest_known_time.json, rejects a sampled time below its prior value, and offers no caller-time option on the examined loader. **This reviewer has not independently inspected that source; the finding belongs to the dependency review and coordinator recheck.** No library choice or runtime conclusion follows from the source finding.

A minimal entry gate would block loader entry before its first time read or write whenever OS time lacks the chosen assurance. Repair time through an already qualified route, then evaluate the normal workflow. **Preflight alone is insufficient:** the product or another legitimate clock owner can change OS time during library calls. Qualification needs a caller-controlled time seam, or an explicitly evidenced transaction/time-epoch policy covering actual clock reads, state writes, discontinuity detection and final authorization. This is an unresolved requirement; it does not prescribe freezing the global Windows clock. A private NTS-derived interval cannot simply be injected into an API that does not have that capability.

A far-future clock sample could also make a legitimate later backward correction fail a persisted time-floor check. The exact write order matters: does the client retain that future value even when metadata verification later fails? This needs source evidence and a future fixture. It would be unsafe to recover by deleting the whole datastore or restoring the embedded root. Repair of one clock-history field, if permitted at all, needs a separately specified authority and must preserve legitimate root/metadata replay defenses. This is an unresolved recovery design, not a recommended edit to library state.

## 3. Persistent state and recovery

The proposal correctly says that network or verification errors must not silently reset trust. For this product, updater state would be **security-critical durable state**, separate from the optional drift/statistical prior that the engine may discard. A first-install bootstrap needs an explicit lifecycle condition; a missing file on an established installation must not automatically recreate that condition. The schema, writer, ACLs, atomicity, ownership during upgrade and evidence of successful persistence are not yet specified. [P; A §§12.1, 19.3]

File authenticity and disk rollback are different. ACLs and authenticated storage can protect against unauthorized editing within their boundary. They cannot distinguish a restored complete valid old snapshot from history the application never saw. The architecture already acknowledges VM/offline/admin rollback of saved time. The same residual applies to TUF trust history unless an additional, explicitly chosen anti-rollback authority is introduced. None is selected here. [A §12.1, THR-17]

Root and release compromise also remain distinct. Independent root custodians can protect key administration while a fully compromised release-signing authority approves bad bytes. TUF cannot recognize the approved program as malicious. An offline client can miss a revocation while its prior view remains admissible; keeping root intermediates available and renewing metadata on schedule are operational duties with owners, not properties supplied by a client crate. The proposal already identifies these limits and should retain them. [P; S2]

### 3.1 When the update is needed to repair time acquisition

Requiring the installed engine to establish the sole time basis for every possible repair creates a circular dependency if that engine is the broken component. The current architecture provides a useful starting point: complete signed-installer maintenance remains outside a bespoke updater. It does not yet prove every emergency case is executable. [A §19]

Three **options to evaluate**, without choosing one, are:

1. **Repair time, then use normal verification.** Cached strict NTS or an independently authorized administrative time remedy restores the precondition. The remedy must retain its real assurance level. The application must not set Windows time directly from bootstrap hints just to make updates pass.
2. **Retain an independently usable complete-MSI recovery route.** A separately authenticated, authorized artifact identity and allowed version can support explicit maintenance when the installed engine is unusable. This is not an automatic fallback after a TUF error. Its trust bootstrap, version limits, existing-state preservation, and Windows publisher/revocation behavior under a wrong date need qualification. Merely presenting the same download host's checksum is insufficient.
3. **Remain blocked when neither route can satisfy the selected assurance.** Keep the installed application usable within its existing safe state and give a precise remedy. Do not disable expiry, weaken TLS validation, replace trust roots from the failed download, or silently select an older release.

Once TUF enforcement were adopted, the recovery contract would have to explain exactly how an explicit maintenance operation is authorized and authenticated. It cannot be a convenient way for the routine update path to bypass its own enforcement. A complete MSI still needs the accepted single-product ownership, repair and rollback rules. These options do not authorize deployment or installation.

## 4. Threat, precondition, control and residual table

These are assessment-local scenarios linked to existing architecture threats; they do not add canonical THR identifiers.

| Scenario and existing link | Attacker/precondition and harmful path | Candidate control and owner | Residual or unresolved evidence |
| --- | --- | --- | --- |
| PT-T1: false freshness; THR-2/4/18 | On-path withholding plus an old, forged or unjustifiably narrow time estimate → expiry decision → stale release remains admissible | Updater admits an explicitly qualified time basis; distinguishes authentication/provenance/age; never disables expiry. Security/protocol owner | NTS sources and time bounds have assumptions; signatures do not certify UTC |
| PT-T2: engine acquires release authority; THR-5 | Compromised engine supplies fabricated AuthenticatedNts status or interval → updater authorizes privileged code | A documented update-time trust boundary and enforcement owner; caller identity alone is not evidence. Architecture/security owner | Existing broker correction bounds cannot prove updater UTC; a new verifier is not silently assumed |
| PT-T3: clock-floor poisoning; THR-11/17 | Grossly future OS date reaches persistent client clock state → legitimate backward correction is rejected → repair unavailable | Entry gate plus qualified caller-time seam or transaction/time-epoch policy; recovery preserves metadata history. Updater/platform owner | Preflight alone cannot control later clock changes; exact write order and real recovery still require qualification; no automatic datastore reset |
| PT-T4: restart erases replay resistance; THR-9/17 | Missing, corrupt, partial or deleted established trust store → embedded-root fallback → old signed history becomes acceptable | Protected mandatory state, explicit installation identity, atomic writes and fail-closed update admission. Installer/updater owner | Whole valid snapshot rollback is not defeated by ordinary file protection |
| PT-T5: long-offline client cannot traverse keys; THR-15/17 | Withheld or removed intermediate root, bad threshold, or expired final view → false success or permanent avoidable failure | Follow ordered protocol; bounded retries and documented root retention/offline window. Release operations owner | Delivery denial remains possible; root-threshold compromise needs separate trust recovery |
| PT-T6: incomplete commit loses protection; THR-16/23 | Crash/disk-full between authenticated role progress and target installation → install occurs without durable necessary state, or rollback restores obsolete knowledge | Qualify per-stage persistence and restart behavior before installation; preserve valid prior progress. Updater/installer owner | OS/filesystem failure can still block availability; whole-workflow rollback is not automatically correct |
| PT-T7: update and time repair depend on each other; THR-16/18 | Engine bug or TLS-date failure → cannot establish updater time → cannot install the engine repair | Explicit independently usable maintenance or time-recovery procedure; bounded errors. Product/platform/security owner | No evidence yet that all wrong-date publisher/revocation cases can recover automatically |
| PT-T8: fixed time becomes a permanent lease; THR-8/11/26 | Paused workflow, resume, stalled download or reused cached authorization → very old start value remains effective | Monotonic duration/generation policy; restart verification after invalidation. Updater owner | Additional application deadline must be distinguished from TUF's protocol expiry checks |

## 5. Finite next-step evidence

The immediate permitted action is **a source-backed decision card**, not a dependency installation or application prototype. It should contain exactly these four outputs:

1. A completed time-basis table covering the five categories in section 2, naming the allowed authority, maximum age/error assumptions, suspend/offline behavior, generation invalidation, and workflow duration. Unknowns remain explicit owner decisions.
2. A source trace for the candidate client's clock read, fixed-start capture, persistent clock history, role persistence and root reload paths. Identify every state write that occurs before final verification succeeds. Carry forward the independently checked tough source findings and bind them to exact published-artifact/source provenance. Resolve the clock-change window between preflight, internal reads and final effects; do not infer safety from an API summary or an entry check.
3. One state-lifecycle table covering genuine first install, ordinary refresh, crash, write failure, missing/corrupt state, upgrade, backward clock repair and whole-disk rollback. Identify which states preserve progress, which may be discarded by protocol rule, and which require explicit recovery.
4. One recovery procedure for “the installed engine cannot obtain acceptable time and needs an update,” including complete-package identity/version authorization, trust bootstrap, effective Windows verification policy, and preservation of replay state. If no procedure meets the chosen assurance, state the dependent adoption blocker.

After those choices, a separately authorized finite fixture campaign can assess **these twelve families**. Each needs a hostile case and the stated legitimate neighbor, independent expected outcomes, actual client entry points, and retained state observations. None was run here.

| ID | Hostile/boundary case | Legitimate neighboring case and required observation |
| --- | --- | --- |
| PT-E1 | Expired intermediate and final roots; missing predecessor; skipped version; duplicate signer entries | Ordered expired intermediates ending in a current root succeed; final expired root or insufficient distinct thresholds cannot authorize a target |
| PT-E2 | Metadata expiry exactly at fixed start, and just before it | Expiry strictly after start is accepted; a later ordinary wall-clock tick does not redefine the fixed start |
| PT-E3 | Lower timestamp version; equal but expired cached view | Equal current version is a normal no-change result; stale cache cannot acquire a fresh-installation claim solely from that result |
| PT-E4 | Restart with replayed timestamp/snapshot/targets versions or removed known metadata entries | Current consistent metadata succeeds after restart with retained history |
| PT-E5 | Fast-forwarded metadata followed by authorized key rotation; unrelated network failure | Only the specified authenticated recovery transition clears relevant state; ordinary failure does not |
| PT-E6 | Crash or disk-full at each necessary role-persistence boundary | Restart resumes from legitimate persisted progress; no installation occurs without required durable state |
| PT-E7 | Established store missing/corrupt; genuine first-install marker forged by an ordinary user | Genuine authorized first install bootstraps; routine deletion cannot silently re-enroll |
| PT-E8 | E ≤ U0; only a stored lower bound; grossly wrong OS date | Qualified interval entirely before expiry is admissible under the chosen additional policy; unsafe clock cannot reach a prohibited persistence path |
| PT-E9 | Future persisted client clock followed by legitimate backward time repair | Normal qualified time progresses; the documented recovery preserves authentic root/metadata history and does not invent a trusted interval |
| PT-E10 | Engine-forged trust label; plain-bootstrap history relabelled as strict; exhausted holdover | Admitted provenance and age survive the actual trust boundary; a genuine eligible basis remains usable |
| PT-E11 | Workflow exceeds duration, sleeps across unsupported continuity, or changes time generation | A bounded uninterrupted workflow succeeds; invalidated start/authorization is not reused |
| PT-E12 | Installed engine broken, metadata host unavailable, or wrong-date OS verification failure | The selected complete-MSI/time-recovery procedure works through its real independent authority; errors preserve safe existing use and do not trigger a weaker automatic path |

A whole valid VM rollback is a separate limitation-demonstration case in PT-E7/PT-E9, not a requirement to claim prevention without an anti-rollback mechanism. A passing synthetic fixture cannot establish real Windows persistence, native verification or elevated installation behavior. Those remain separately authorized qualification work.

## 6. ASVS and guidance treatment

The pinned ASVS baseline is 5.0.0, release commit 5cf9b032440be53ce345ab3c130fda46ba1ce7a2. The local source was read for the exact relevant identifiers. Potential affected mappings include v5.0.0-2.2.1/2.2.2 (boundary rules), 8.3.1 (trusted authorization), 11.1.1/11.1.2 (key lifecycle and inventory), 13.1.3 (bounded external work), 15.1.5/15.2.5 (dangerous functions and containment), 15.4.2 (check-to-effect atomicity), and 16.3.4/16.5.2/16.5.3 (observable secure failure). The existing worksheet was not changed and no conformity is claimed. [G; A Appendix C]

The website-guidance skills were applied as review methods: preserve requirements, trace threats to effects, keep evidence independent, and distinguish planned tests from execution. Browser, JavaScript and site-specific commands were not imported into the Windows design. The security supplement is explicitly a candidate amendment; this review does not adopt it. The user's read-only evaluation scope takes precedence over the threat-model skill's generic instruction to update the main plan.

## 7. Sources and evidence identity

- **P — supplied proposal:** upload/Pasted text(20261028-141654).txt; all 125 lines reviewed. SHA-256: 5b2bf4eabb183dcec647232e0cf1e8533ccef2281db59532d0bd6dd35a0e0694. No independent claim was made about the linked launcher implementation or incident.
- **A — accepted plan:** deliverables/windows-time-architecture.md, RWTC-ARCH-001 revision 1.4. SHA-256 at review: a9bd1432ecb2d04e55b77e1a270eb1a135027d9f4a7988d58d9299367a6ca075. This review concentrated on the specified sections and relevant domain-contract/boundary text.
- **S1 — TUF specification:** [official specification](https://theupdateframework.github.io/specification/latest/), observed 1.0.37, 8 October 2026. Relevant sections: 2.1.1, 5.1–5.6, 6.1, 7.1. Version/date are observed; an immutable commit was not resolved. The official repository search corroborated the version header; guessed release/license URLs failed and are not evidence. Retrieval identifiers are retained in findings.json for coordinator tracing.
- **S2 — TUF security:** [official security documentation](https://theupdateframework.io/docs/security/), last-modified label 12 October 2024, site revision 2856e69 as displayed. Used for attack definitions and delivery/signing limits.
- **S3 — NTS:** [RFC 8915](https://www.rfc-editor.org/rfc/rfc8915.html), September 2020, particularly 8.5–8.7. Used only for the stated certificate-time, authentication and delay considerations.
- **G — guidance:** architecture-research/website-guidance at the accepted source pin 6530d3a8d44cbe4f829b1e09edd3a1c90cffc087: AGENTS.md; testing/CONTRACT.md; planning/plan-independent-review/SKILL.md; planning/plan-threat-model/SKILL.md and its threat-model-method/asvs-applicability references; security/CONTRACT.md, applicable Constitution sections and amendment status. ASVS local JSON SHA-256: dfcb76fbd04995f9687f35a78af0053af32773676476c2ac0e3b283c3411e2d2.

**Evidence limits:** read-only source/document review, plus writing this assigned review record. The workspace was not a Git repository; git status returned that error, so no clean-tree claim is made. No packages, builds, application tests, network probes, clock changes, publisher operations or external writes were performed. Documentation reads do not prove implementation conformance, freshness of a deployed repository, exact published-package parity, or working recovery on Windows.
