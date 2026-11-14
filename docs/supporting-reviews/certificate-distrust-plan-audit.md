# Certificate distrust plan: independent skills and practice audit

**Date:** 14 November 2026.

**Canonical plan:** [NV-TRUST-001](../plans/post-v1-certificate-distrust.md),
revision 1.1 reviewed; corrections in revision 1.2.

**Scope:** deferred TRUST-0 planning and its existing crypto/engine handoff.
This record preserves review evidence; it introduces no implementation plan,
policy authority, execution permission or release qualification.

## 1. Request, candidate and review method

David requested another audit against repository skills and relevant best
practices, using separate skill/contract, security, architecture and UI/UX roles.
The coordinator applied the repository's
[independent review skill](../../.agents/skills/planning/plan-independent-review/SKILL.md)
and [review/handoff procedure](../../.agents/skills/planning/references/review-and-handoff.md).
Canonical owner decisions and contracts control the comparison; external sources
inform uncertain technical facts. Reviewer agreement is not risk acceptance.

The four initial reviews inspected the frozen baseline
[`6acd1e504033c1b57992cb0823667f5e6a8bff46`](https://github.com/DavidOsipov/NevroTick/commit/6acd1e504033c1b57992cb0823667f5e6a8bff46).
The initial plan blob is `2be464cc47b66127ed0a29891ba21bf648d69821`.
The checkout was initially clean. The contract/QA reviewer continued using
`git show` against that baseline while the coordinator applied corrections.
The plan's historical product pin remains explicitly historical; it was not
silently replaced by the documentation head.

Every specialist was launched with `fork_turns: "none"`, actual paths and a
bounded read-only assignment. No parent conversation or private discussion was
supplied. Shared files and public repository instructions were available, so
fresh context is not claimed as statistical independence. The completion and
inspected-input claims below come from the actual reviewer returns; no unobserved
read or successful execution is inferred.

| Reviewer | Assigned comparison | Context and completion | Initial verdict |
| --- | --- | --- | --- |
| R1 — `/root/plan_architecture_audit` | `plan-architecture`, ownership/data-state guidance, local IPC, contract enforcement and concurrency/lifecycle QA; selected actual TLS/association/send owners | Fresh context; completed, read only | REVISE: ARC-01 |
| R2 — `/root/plan_security_audit` | Security contract, crypto/secrets, protocol inputs and threat modeling; selected ASVS applicability and current primary upstream evidence | Fresh context; completed, read only | READY for deferred planning; SEC-01 low clarification |
| R3 — `/root/plan_ux_audit` | Native UI/Slint, secure UI and privacy, accessibility and i18n; canonical presentation and notification contracts | Fresh context; completed, read only | REVISE: UX-01 |
| R4 — `/root/plan_contract_qa_audit` | Rust contracts, data-contract transitions, QA planning and contract tests; architecture/release compatibility and actual prototype boundaries | Fresh context; completed, read only | REVISE: TRUST-CQ-01 |
| J1 — `/root/plan_integrated_correction_audit` | Revised plan, affected handoffs, original findings and relevant canonical requirements; verify corrections and check regressions | Fresh context; completed, read only | READY for completion of the deferred documentation audit; all four corrections verified |

The attributed raw returns below retain each reviewer's two comprehension
paragraphs and initial verdict. They describe the reviewed baseline, not an
unreconciled verdict on revision 1.2. Reviewers saved their original prose;
chat-only citation tokens and a local file link were converted to equivalent
primary-source or immutable GitHub links for this repository record.

## 2. Findings and correction lifecycle

Initial reviews found three medium/P2 documentation gaps and one low terminology
clarification. None demonstrates an exploitable product implementation. No
finding calls for changing the accepted post-V1 order or starting native work.

| Stable finding | Requirement, consequence and smallest correction | Revised location | Disposition |
| --- | --- | --- | --- |
| **ARC-01 — Medium** | The event contract already reaches the broker's final effect issue, but the stage/task/closeout assignment stopped at association/send. Name existing Core/IPC/broker owners, freeze effective trust context and applied-transition acknowledgement ordering, and carry actual outcomes/settlement into qualification. A local engine revision or queued notification is not broker enforcement. | Plan §8 W02/W04/W05, §9.5, EV-04, §10–11; crypto and engine TRUST-0 addenda | Verified by J1 against the frozen normative candidate |
| **UX-01 — P2** | CERT-01/02/04 unconditionally said the source was unused even when LIFE-02 preserved an older valid association. Bind visible/spoken source usage to actual published eligibility in the same coherent snapshot; identify failed replacement separately. Add the paired EV-03 consumer observation. | Plan §9.1 target context, §9.2 shared wording rule and CERT-01/02/04, EV-03; engine handoff | Verified by J1 against the frozen normative candidate |
| **TRUST-CQ-01 — Medium** | Feed-version and anti-rollback tests did not explicitly cover old/new application readers, surviving processes and stored generations. Freeze compatibility/refusal before writes, protect checked migration, reconcile interrupted publication and prevent code rollback from restoring consumed authority. Require independent old/new consumer fixtures and healthy transitions. | New plan §5.1; §8 W02/W05; §9 application-compatibility case; §10–11 and both task addenda | Verified by J1 against the frozen normative candidate |
| **SEC-01 — Low** | "Existing strict native policy" could imply an already selected shipping revocation/offline profile. Route unknown native status to the policy approved under OPEN-8 and identify strict cache-only verification as laboratory-specific. This neither selects nor weakens production policy. | Plan §9.1 precedence rule 4 | Verified by J1 against the frozen normative candidate |

The implementation owners remain existing owners. ARC-01 assigns the future
cross-process contract without selecting a new transport or duplicating feed
verification in the broker. TRUST-CQ-01 requires explicit unsupported combinations
as well as supported ones; it does not invent old-state migration where no such
state survives, or select hardware anti-rollback infrastructure. UX-01 preserves
both sides of the distinction: an unrelated failed replacement need not evict a
qualified old association; a prohibition matching that old association still
retires its authority. SEC-01 supplies no new fallback or tolerance.

### Reconciliation with existing decisions

| Obligation | Disposition and authority |
| --- | --- |
| Actual V1 release → MATH-C01 → TRUST-C01 → MT-C01, with TRUST-0 ranked second | Retained from [OD-20](../nevrotick-owner-decisions.md#24-od-20--supplementary-certificate-distrust-after-v1) and [ROADMAP](../../ROADMAP.md#trust-0--post-v1-certificate-distrust-and-revocation); no queue change |
| Rejection-only initial overlays; native checks retain their independent authority | Retained; SEC-01 clarifies the unresolved shipping-profile owner |
| Actual accepted-path custody and current authority at final effects | Retained; ARC-01 repairs stage/owner/evidence traceability and explicitly covers cross-process gaps |
| Compatible update, mixed-process behavior and safe rollback | Existing architecture/release obligation made explicit for this feature by TRUST-CQ-01; no new schema or compatibility policy selected |
| Source eligibility, feed condition and aggregate time state remain separate | Retained; UX-01 corrects contradictory example copy and the corresponding observable case |
| Existing V1 requirements, Windows/fuzz/storage/UI holds, finite activation budget and explicit TRUST-C01 disposition | Retained; no obligation removed or current execution gate lifted |

Reopen a finding if the relevant event semantics, producer/consumer ownership,
canonical decisions, compatibility inventory or assurance profile changes. A
heading move alone does not invalidate the semantic result. At activation,
TRUST-W01 must inspect the actual released source and refresh time-sensitive
external evidence; this document supplies no inherited implementation pass.

## 3. Integrated verification and limits

**Integrated correction verdict: READY for completion of this deferred documentation
audit.** J1 verified ARC-01, UX-01, TRUST-CQ-01 and SEC-01 and raised no new
actionable finding in the changed boundaries. The full attributed return and
exact reviewed blobs are retained below. The three normative file blobs remain
unchanged from that review; this evidence record subsequently adds the completed
review, dispositions and commit-preparation observations.

**Staged documentation checks:** passed. Read the complete intended index via
`git show :path` and retained its `git write-tree` identity at commit preparation.
The scoped check resolved 31 local link/anchor references (plan/report and added
task links), checked 17 plan/report tables, preserved the same 27 event rows and
nine acceptance groups, and confirmed verbatim inclusion of all five reviewer
returns. `git diff --cached --check` passed. Only the four intended Markdown files
were staged, with no unstaged tracked changes or unexpected file modes. No root
`docs/SHA256SUMS.txt` exists, so no documentation-bundle integrity claim is made.
The checks were repeated after recording these observations; they establish
document consistency, not runtime behavior.

The security specialist inspected six named primary targets, including the
Remote Settings specification, Windows chain API, two RFCs and pinned Firefox
and `clubcard-crlite` source. The raw return records successful GitHub fallback
after web raw-file access failed, and the narrower NSS-caller scope. The UI
specialist checked Microsoft notification guidance and W3C WCAG2ICT. Other
roles relied on relevant repository contracts and selected source. This is a
bounded audit of affected practices, not a claim to have exhaustively audited
every skill, dependency, source file or external standard.

Runtime code, dependencies, Windows trust stores, services and package content
were not changed. No Cargo, fuzz, formal, native Windows, accessibility-device
or installed-lifecycle qualification was performed. The source comparisons do
not demonstrate that the future feed, IPC or UI behavior already works.

Still intentionally open for activation: required-evidence versus additional-veto
assurance mode, precise freshness and reaction bounds, root-policy input/scope,
full-authentication and historical-model predicates, supported version matrix,
resource limits and the finite execution budget. The plan names their owners
and preceding gates; a deferred decision is not automatically a current defect.

**Permitted next action:** publish this verified documentation correction record
under the current request. Product work continues
under the existing development plan; TRUST implementation remains deferred.

## R1. Architecture and lifecycle — original reviewer return

## Comprehension

The proposal adds authenticated OneCRL and CRLite evidence to NevroTick’s existing Windows certificate checks after actual V1 release and MATH-C01, with OneCRL implemented first. Its initial policy adds rejection: a matching admitted prohibition can veto Windows acceptance, while overlay absence or a good result supplies no override of native failure. The plan connects actual accepted certificate paths to immutable receipts, NTS associations and continuing use, then expands the effect of relevant distrust to dependent samples, models and broker proposals. Update authentication, freshness, historical evidence and full certificate authentication remain separate facts. Root-program policy and any later leaf-revocation substitution retain their own decisions.

The inspected system is still a bounded prototype. Windows-gated transport constructs a CNG-backed TLS 1.3 connection with the platform verifier, disabled resumption and disabled early data. The vendored verifier can inspect the native chain but currently returns a rustls success marker rather than a selected-path receipt. Private candidate and association owners preserve same-invocation exporter/cookie custody, and the actual datagram adapter guards the send syscall. These mechanisms do not supply production trust-feed generations or a production NTS-to-clock integration. The actual Windows facade passes `None` for the optional experimental numerical profile and publishes explicitly prototype-scoped results.

## Scope, baseline and verdict

**Verdict: REVISE — one narrow planning-handoff defect.** There is no architecture reason to change the accepted post-V1 ordering or activate implementation now. The defect is correctable in the existing plan and task handoff; it is not a new V1 gate or a demonstrated runtime vulnerability.

- **Candidate:** `/workspace/scratch/dc7d02421f60/NevroTick`, HEAD `6acd1e504033c1b57992cb0823667f5e6a8bff46`; clean at initial and final inspection.
- **Plan:** `docs/plans/post-v1-certificate-distrust.md`, NV-TRUST-001 revision 1.1.
- **Independence:** read-only reviewer; no edits, Cargo, Windows/native runs, installation or service mutation.
- **Guidance inspected:** repository `AGENTS.md`; relevant routing/authority portions of `docs/README.md`; `plan-independent-review` and its review-and-handoff reference; `plan-architecture` with data/state and ownership references; `implement-rust-contracts`, implementation contract and relevant contract-enforcement guidance; `security-local-ipc`; `qa-state-concurrency-lifecycle`; testing contract.
- **Canonical comparison:** relevant architecture §§5–10, 11.2, 12.1–12.3, 17–18; R-02 floor/supersession and R-05 authentication lifetime; OD-20; existing crypto and engine TRUST-0 addenda.
- **Bounded source inspection:** engine manifest/configuration gates, TLS connection/candidate publication, association receive and request ledger, actual guarded datagram send, retirement custody and facade; vendored Windows verifier. These inspected source owners have no diff from the plan’s recorded product `bad2db638bb112496d16bb1172835473440b421e` to the reviewed HEAD.
- **Evidence limit:** source/document comparison only. This review did not independently revalidate the external incident chronology, CRLite distribution facts or cryptographic formats.

## Finding ARC-01 — Carry the expanded downstream contract into stage ownership and closeout

**Priority:** Medium; correctable planning defect.

**Requirement and boundary.** `plan-architecture` requires the canonical writer, affected consumers, effect owner and lifecycle to remain traceable. Architecture §6.4 assigns current effect authority to the receiving service; §§5.2 and 12.2 require current generations/revisions across engine–broker IPC; §10.2.1 retains the final mutable guard immediately before the Windows call and separates durable physical settlement from model publication.

**Exact plan evidence.**

- Plan **§9.5, lines 451–466**, now requires serialized policy transition and current-authority checks through numerical sample admission, broker proposal acceptance and the broker’s final pre-effect/issue boundary.
- **EV-04, line 595**, includes the race after proposal acceptance and before final clock-effect issue.
- But **§8, TRUST-W04, line 279**, assigns only **crypto/engine** and describes selected-path, KE/exporter, association/publication and send integration. Its exit does not name the Core/model, IPC or broker consumers added by §9.5.
- **§11, lines 652–655**, asks closeout to record the “actual chain-to-association consumer path,” again stopping before the newly required downstream boundaries.
- The existing task handoffs retain the same narrower scope: `tasks/s0.1-crypto-feasibility.md:2055–2060` and `tasks/engine-transport-pilot.md:16000–16007` route accepted-chain → association → protected sender, without assigning the added broker/Core work.

**Affected consumer and consequence.** A plan implementer has a clear owner for admitting a new feed snapshot and retiring an association, but no equally explicit stage assignment for making that trust transition effective at a separate broker process and reconciling dependent Core/model state. In particular, an engine-local revision change is not by itself proof that the broker has withdrawn authority from an already accepted proposal. A delayed, lost or reordered IPC transition—or a restart leaving the processes at different effective contexts—must have a defined disposition before a queued Windows effect can be permitted.

This is a missing handoff/design-freeze assignment, **not a claim that the plan deliberately permits stale effects**. The normative prohibition is present; the execution map and named closeout path did not catch up with it.

**Smallest correction.**

1. Expand TRUST-W02’s future contract freeze to include the **effective trust context across engine and broker**: canonical writer, relevant per-source/association/model scope, identity/version/incarnation binding, effective-transition/acknowledgement ordering and failure behavior. State what the broker treats as current and what remains pending or unavailable.
2. Expand TRUST-W04 to name the existing **Core, IPC and broker owners**, covering dependent sample/model eligibility and final issue authority in addition to association/send.
3. Extend W04/W05’s required discriminating cases and §11’s closeout path through **sample/model → proposal → broker final issue → actual outcome/settlement**. Include delayed or missing invalidation delivery, duplicate/reordered delivery and single-process restart beside an unaffected-source control.
4. Reconcile the existing crypto/engine handoff addenda with this expanded assignment. Reuse their existing records and canonical IPC contracts.

This does **not** require choosing a transport algorithm, concrete timeout, freshness constant, new service or new authority now. It requires explicitly assigning the later contract and evidence before its dependent post-V1 implementation.

**Counterevidence limiting severity.** §9.5 already names the correct sinks, forbids stale effects and preserves native custody; §9.7 maps the clock boundaries to CHECK-2/3/10; §10 retains trust/Core/broker decision owners. Those provisions make this a narrow traceability/readiness correction, not a missing end-to-end security requirement or an implementation blocker today.

## Findings that held up

The following points are sufficiently represented for the deferred planning stage:

- **Actual selected-path evidence:** §§4–5 correctly reject reconstructing the Windows path from the peer’s certificate ordering or using an unrelated verification boolean. The present verifier’s marker-only return and the private candidate transfer support the stated integration need.
- **Current authority versus immutable history:** receipt identity, snapshot revision, enforcement generation, refresh observation and original full-authentication age are distinguished. An unchanged refresh need not cause a KE storm or renew authentication lifetime.
- **Cancellation and native custody:** §9.5 explicitly separates retirement from resource release. The native send adapter currently holds `Control::published_guard()` around the actual `try_send_to` call, so the plan is not relying solely on packet preparation. Trust-policy suppliers remain future work.
- **Earlier samples and models:** §9.5 calls for provenance-aware reassessment of regressions, predictions and drift priors; it does not automatically turn unqualified history into trusted holdover. It preserves the possibility that historical replies were valid.
- **Persistent floors and clock settlement:** the plan routes floor changes to existing supersession authority, preserves budgets, forbids resetting foreign adjustments and recognizes that already issued effects cannot be undone by changing a flag.
- **Update/storage lifecycle:** complete admitted snapshots, atomic replacement, missing anti-rollback state, corruption, authenticated unchanged refresh, signer rotation and recovery are all represented. Mandatory evidence does not inherit optional-cache fallback.
- **Finite scope:** TRUST-C01 requires qualified integration or an explicit owner defer/replan; incomplete attempts cannot silently advance the queue. The effort budget is intentionally frozen at activation, and current measurements are not replaced by historical Firefox resource figures.

## Open decisions that are not current defects

The plan properly leaves assurance mode, coverage requirements, accepted freshness evidence, age limits, root-policy scope, reaction bounds, full-reauthentication rules, historical-model predicates, concrete resource limits and the execution budget to the named owners before dependent stages. Their absence today does not block a deferred plan.

At activation, the storage owner should also apply the canonical §11.2/§12 compatibility and rollback limits to the new mandatory state: compatible code/state rollback is different from restoring all old data, and a protected file does not itself defeat offline disk or VM rollback. The plan’s existing lifecycle obligations can carry this work; no hardware rollback mechanism should be inferred or selected now.

## One permitted next action

**Make the narrow ARC-01 handoff correction in the existing plan/task chain and recheck only that changed ownership and cross-process ordering scope.** Preserve actual V1 → MATH-C01 → TRUST-C01 → MT-C01 ordering and all current execution holds.

## R2. Security — original reviewer return

## Comprehension

The proposed change adds supplementary certificate rejection to NevroTick’s own NTS authentication after actual V1 release and MATH-C01. OneCRL would reject matching certificates or subject/key pairs on the Windows-accepted path; CRLite would add revocation decisions for covered public TLS leaves. Neither mechanism could turn a native rejection into success. The plan also defines how admitted trust changes propagate through existing associations, outstanding requests, derived time models, broker authorization, and truthful user status. Supplementary root-policy constraints remain a separate investigation.

The current system does not have that production feed integration. Its relevant TLS and association paths are laboratory/prototype work, with shipping service-context trust, retrieval, age, and native qualifications still governed by OPEN-8/R-05. The selected strict cache-only verifier is laboratory-specific. The plan correctly requires the released product to be reinspected at activation, treats its proposed types and events as future contracts, preserves existing V1 obligations, and does not authorize implementation or native execution now.

## Verdict

**READY for the assigned deferred-planning scope.**

I found **no blocking security defect** in the reviewed plan. One low-severity authority clarification is worth making during the present documentation maintenance. It does not justify changing a trust policy, starting implementation, or reopening settled deferrals.

### Review identity and scope

- **Candidate:** repository HEAD `6acd1e504033c1b57992cb0823667f5e6a8bff46`.
- **Target:** `docs/plans/post-v1-certificate-distrust.md`, NV-TRUST-001 revision 1.1, all 711 lines.
- **Target Git blob:** `2be464cc47b66127ed0a29891ba21bf648d69821`.
- **State:** clean at initial and final inspection; no edits or runtime/native tests performed.
- **Reviewer:** separate security reviewer; read-only assignment. Parent should record the actual delegation context setting from its launch record.
- **Relevant authority inspected:** `AGENTS.md`; documentation routing; OD-20; architecture §18.7.2/OPEN-8; R-05 including retrieval and full-authentication lifetime; crypto task’s deferred TRUST-0 handoff and ordinary/strict laboratory trust cards.
- **Skills applied:** `plan-independent-review`, its `review-and-handoff` reference; security `CONTRACT.md`; `security-crypto-secrets`; `security-protocol-inputs`; `plan-threat-model` and its threat method; testing contract. Relevant ASVS applicability entries were inspected for validation, TLS revocation, service TLS, outbound destinations, and check/use races. This was not a full ASVS conformance review.

## Finding

### SEC-01 — Low, nonblocking: distinguish the strict laboratory policy from the unresolved shipping profile

**Location:** `docs/plans/post-v1-certificate-distrust.md`, lines **348–349**, under §9.1 decision precedence.

**Observation:** The statement that unknown native revocation “remains subject to the existing strict native policy” can be read as asserting an already selected shipping policy.

**Requirement and evidence:**

- Architecture `docs/windows-time-architecture.md`, lines **3926–3934**, leaves the ordinary profile with its owner and explicitly says that the preparation selects neither universal soft failure nor strict online policy.
- `tasks/s0.1-crypto-feasibility.md`, lines **2110–2112**, preserves that distinction.
- The same task, lines **2136–2147**, identifies the stricter configuration as a laboratory-only experiment and leaves shipping/service trust policy under OPEN-8; a later soft-failure exception would still require David’s decision.

**Consequence:** A future implementer could mistake a laboratory source observation for a frozen production requirement. This is documentation authority drift, not evidence of a runtime vulnerability. The rest of the plan’s reinspection and decision gates substantially contains the ambiguity.

**Minimum correction:** Replace the final clause with wording such as:

> Unknown native revocation remains governed by the revocation/offline policy approved under OPEN-8; this matrix neither selects nor weakens that policy. The present strict cache-only verifier remains laboratory-specific.

**Owner:** plan coordinator/security owner. This is a narrow correction under existing authority, not a reason to choose a weaker profile or request a new policy decision now.

## Security properties checked and contrary hypotheses resolved

### 1. Exact matching and accepted-path coverage

The plan does not confuse company names, issuer/serial identity, and subject/key identity. Lines **81–87** specify the paired OneCRL forms and extend matching across the actual accepted path. Lines **118–130** explicitly reject reconstructing that path from peer-supplied intermediate order or an opaque verifier-success marker.

The pinned Firefox storage source independently confirms separate issuer/serial and subject/hash keys. It also confirms that CRLite uses the issuer-SPKI hash and serial input, and that a revocation found in any loaded applicable filter wins over a prior good result. The inspected `clubcard-crlite` source preserves `Good`, `NotCovered`, `NotEnrolled`, and `Revoked` rather than turning absence into validity.

**Hypotheses rejected:** “Only the leaf is checked”; “peer chain ordering can supply the issuer”; “one good filter masks a later revocation”; “missing coverage becomes good.” The plan contains direct prohibitions and discriminating cases for each.

**Limit:** I reread the storage/query implementations, not the complete NSS caller chain that supplies the DER public-key argument. The plan’s exact SPKI representation claim therefore received a narrower independent recheck than its paired-key and CRLite query semantics.

### 2. Remote Settings authentication, canonicalization, and attachments

The current Remote Settings specification supports the plan’s separation between transport and object authentication. It describes an independently fixed signing root, signing-certificate checks, canonical sorted collection data, the content-signature prefix, and fixed-format P-384/SHA-384 verification. Attachment size and SHA-256 are separate metadata-bound checks. The plan captures these requirements at lines **179–201**. [Remote Settings client specifications](https://remote-settings.readthedocs.io/en/stable/client-specifications.html#signature-verification)

The plan also requires expected collection/signer identities, correct changeset/tombstone processing, complete immutable installation, and no partial activation. Its strict typed decoding and preallocation/resource requirements prevent the signature from becoming a payload-safety exemption.

**Hypotheses rejected:** “HTTPS alone authenticates the feed”; “a hash supplied by an unverified response is sufficient”; “a signed payload bypasses hostile-input bounds”; “failed installation can silently drop known distrust.”

### 3. Currentness, replay, and rollback

Lines **159–171**, **192–208**, **304–305**, and DATA-04/08 distinguish content revision, authenticated refresh observation, coverage horizon, and elapsed age. They expressly prohibit renewing freshness from replay, disk timestamps, transport success, or a same-revision response alone.

This is an appropriately unresolved future protocol decision. Remote Settings documents cached polling/fetching and does not guarantee that collection cache TTL stays below a fixed number of hours. Its signature procedure does not itself establish the application’s desired maximum data age. The plan consequently does not claim that content authentication solves freeze detection or timely emergency delivery. [Remote Settings client specifications](https://remote-settings.readthedocs.io/en/stable/client-specifications.html#cache-busting)

**Not a finding:** exact age limits, authenticated unchanged-refresh evidence, first-install behavior, offline distribution, and anti-rollback recovery remain explicit activation-stage decisions. Inventing those values during this audit would exceed the assigned planning scope.

### 4. Signer trust, transport trust, and recovery

The plan distinguishes NTS-KE, feed transport, feed signing, and later DoQ purposes. DATA-09 does not convert an update-transport failure into revocation of unrelated time servers. DATA-10 requires independently trusted recovery after signer compromise/withdrawal and explicitly forbids a feed from choosing its own arbitrary successor.

It also recognizes shared Mozilla ecosystem dependencies rather than describing OneCRL and CRLite as independent votes. Valid signatures establish the selected publisher’s statements; the plan does not claim that they detect every unreported compromise.

**Hypotheses rejected:** “A failed update implies certificate compromise”; “the compromised signing authority can bootstrap its own replacement”; “two Mozilla mechanisms constitute independent consensus.”

### 5. Windows verification and wrong-clock behavior

Microsoft’s current `CertGetCertificateChain` contract confirms that the supplied verification time does not control trust-list, root-store, and all revocation checking; CRLs themselves must be valid at current time. It also distinguishes chain-building cache-only retrieval from revocation cache-only retrieval, and scopes the accumulated timeout to revocation URL retrieval. These support the plan’s continued OPEN-8/native-work gates. [Microsoft CertGetCertificateChain](https://learn.microsoft.com/en-us/windows/win32/api/wincrypt/nf-wincrypt-certgetcertificatechain)

**Hypotheses rejected:** “A provisional TLS time fixes every Windows check”; “cache-only means no relevant delegated/background retrieval”; “a retrieval timeout proves total native-call completion.” No such claim appears in the reviewed contract.

### 6. Fail-closed behavior and availability

The initial policy is rejection-only. Missing optional supplementary coverage and missing required native evidence remain separate. Mandatory supplementary evidence can withhold authority; an additional-veto mode can preserve independently qualified native acceptance only under its explicit approved policy. The document selects no grace period or silent downgrade.

This is not a failure to finish the plan: deciding whether an overlay must be mandatory for a named mode is explicitly assigned to the future owner decision. Known applicable rejection continues to win in either mode.

### 7. Association lifetime, race boundaries, and downstream effects

The plan correctly does not equate original TLS-certificate expiry with automatic termination of an otherwise valid NTS association. RFC 8915 §8.5 states that expiry alone normally need not trigger another KE; §8.2 and §8.7 separately address cookie-key compromise and stripping/fallback. The plan preserves independent full-authentication/key lifetime and current-trust requirements. [RFC 8915 §8](https://www.rfc-editor.org/rfc/rfc8915.html#section-8)

Sections **9.4–9.5** cover failed replacement versus existing authority, queued results, protected send, response/cookie admission, numerical admission, broker acceptance, and the final pre-effect boundary. Cancellation retains custody through actual completion. Affected historical evidence cannot simply be relabelled as trusted holdover; floors and consumed correction budgets keep their existing owners.

**Hypotheses rejected:** “A refresh failure kills every still-valid association”; “old authenticated UDP can override newly applicable distrust”; “the broker may actuate a proposal already accepted under stale policy”; “changing a trust flag undoes an already completed clock effect.”

### 8. Corrections and revocation semantics

The plan permits authority-specific authenticated supersession without making missing or expired-out data a reinstatement. It keeps revocation reasons, invalidity dates, publication/detection times, and scope distinct. RFC 5280 supports these distinctions: invalidity can predate processed revocation, and `removeFromCRL` can indicate expiry or release from hold rather than an unconditional new validity assertion. [RFC 5280 §5.3](https://www.rfc-editor.org/rfc/rfc5280.html#section-5.3)

## External evidence and limits

Six distinct upstream targets were checked:

1. Remote Settings client specifications.
2. Microsoft `CertGetCertificateChain`.
3. RFC 8915.
4. RFC 5280.
5. Firefox `cert_storage/src/lib.rs` at `c72617c016491f426f33f84b8822aad7fdd532c2`; returned blob `73af27769155c2fad34700915dedba1339d46c20`.
6. `clubcard-crlite/src/query.rs` at `054c1799ae71ff6f7d98a804fa98a4f4b4bedd11`; returned blob `02ff480477ed78f87bf90ab7468ef118d225d779`.

The web tool failed on the two raw GitHub URLs with `DisabledError`; bounded reads of those same pins through the connected GitHub tool succeeded. Re-reading portions of the successful sources added no broader survey.

I did not reproduce the incident chronology, audit all proposed dependencies, fetch live feed datasets, execute product code, test storage recovery, inspect an installed Windows service, or measure reaction/resource bounds. These remain outside the deferred documentation review. No numerical security confidence or compliance claim is warranted from this evidence.

## One permitted next action

**Apply SEC-01’s narrow policy-authority clarification during the current documentation maintenance, then retain TRUST-0 as deferred until actual V1 release and MATH-C01.** No implementation or native execution follows from this READY verdict.

## R3. UI/UX — original reviewer return

## Independent UI/UX review — completed

**Verdict: REVISE the deferred plan for one localized presentation contradiction.** I found no UI reason to block continued documentation work or add a V1 requirement. The correction concerns proposed wording, not the underlying certificate-enforcement rule.

**Reviewed candidate:** `/workspace/scratch/dc7d02421f60/NevroTick`, HEAD `6acd1e504033c1b57992cb0823667f5e6a8bff46`; working tree clean before and after review. Target: **NV-TRUST-001 revision 1.1**, especially §§9.1–9.7, with upstream state and lifecycle contracts inspected. No files changed, tests run, native UI opened, dependencies altered, or holds lifted.

### Comprehension

The proposal adds supplementary certificate rejection after V1 and MATH-C01. OneCRL, CRLite and any separately admitted root constraints can veto a path accepted by Windows; they cannot turn native failure into success. Its event model carries four separate facts to presentation: certificate-policy verdict, security-data condition, current association/source eligibility, and aggregate time state. The engine and other authoritative owners evaluate those facts; the UI explains their consequences and offers bounded navigation or measurement requests. Feed refresh, successful replacement authentication and incident dismissal each have deliberately narrower meanings than restored synchronization.

The current GUI is a Slint application with typed presentation, catalog admission and fixture notification boundaries. The GUI handoff explicitly leaves production status IPC, a real incident producer, native accessibility/delivery and independent critical-text approval open. Canonical architecture already requires coherent snapshots, historical labels on stale information, separate service/trust/clock authority, stable focused rows and controlled state announcements. The v0.4 HTML design supplies interaction references and simulated notification behavior. None of that constitutes native Windows qualification or permission to activate deferred TRUST implementation.

## Material finding

### UX-01 — P2: Certificate rejection copy can falsely say a still-usable source is excluded

**Location:** [post-v1-certificate-distrust.md](https://github.com/DavidOsipov/NevroTick/blob/6acd1e504033c1b57992cb0823667f5e6a8bff46/docs/plans/post-v1-certificate-distrust.md), §9.2 **CERT-01, line 390**; the same unconditional source-usage suffix appears in **CERT-02, line 391**, and **CERT-04, line 393**. Compare **LIFE-02, line 436**, and **EV-03, line 594**.

**Requirement/boundary:** CHECK-14 and the source-role presentation contract; CHECK-22 for corresponding accessible wording. The certificate verdict and current source eligibility must remain independent facts.

**Evidence and counterexample:** CERT-01 correctly says to refuse matching new authentication and retire associations **whose own receipt is affected**. LIFE-02 correctly permits an older association to remain usable while its own receipt and limits remain valid.

A valid counterexample is:

1. A source has a usable association established using certificate A.
2. A replacement KE presents certificate B.
3. B is revoked under an exact issuer/serial rule that does not match A.
4. The replacement is refused; A’s association remains independently eligible under LIFE-02.

The proposed CERT-01 message nevertheless says, “This source is not used.” That statement is false in this permitted branch. Similar ambiguity exists when a replacement chain uses a different revoked intermediate or explicitly distrusted root while the older association’s own receipt remains unaffected. The event table labels these messages **source-level** and does not explicitly require the preserved-association qualifier to replace the exclusion claim.

**Consequence:** A future implementation can faithfully implement the mechanism response while showing contradictory source status. The Sources view, accessible description or event details can claim exclusion while the same source still contributes eligible evidence. This can also prompt an unnecessary source change and obscure which authentication attempt failed.

**Minimum correction:** Add a shared rule immediately before the certificate-event table:

> Source-usage wording comes from current published eligibility, not the certificate event code alone. If only a replacement candidate fails and an older association remains qualified, identify the failed refresh and the independently usable existing association. Say “This source is not used” only after the source actually has no usable association under current policy.

Then make CERT-01/02/04’s suffix conditional, or point those rows explicitly to that rule and LIFE-02.

Extend **EV-03** with the paired presentation observation: an unrelated revoked replacement preserves the eligible older association and truthful visible/spoken source status; a prohibition that also matches the older receipt produces actual exclusion and the exclusion message. This is a future acceptance addition, not a request to execute UI or native tests now.

**Classification:** A contradiction in proposed presentation examples, not an observed runtime bug. The underlying scoped invalidation and association-preservation rules are already sound.

## Passed and refuted concerns

### Truthful hierarchy and aggregate state

The plan keeps source rejection separate from global synchronization failure. Section 9.6 permits Tracking only when remaining qualified evidence meets the existing agreement/readiness conditions; it does not reduce this to a source count. Acquiring, Holdover, Expired, Blocked and Conflict remain canonical states, and no new “Secure” badge asserts UTC accuracy. The qualified-prior-model restriction in §9.5 prevents affected evidence from being relabelled as trusted holdover.

The plan also preserves the distinct last reply, qualified estimate, applied correction and UI snapshot ages. Its additional authentication/feed times do not replace those facts.

### Stale or disconnected service information

The unavailable-snapshot row states that status is unknown and retained information is historical. It does not assert that the service or trust enforcement stopped. Monotonic age is allowed only with established continuity. These rules align with architecture §16.2 and feature F-04.

### Recovery and notification lifecycle

Successful downloads, dismissal, unsuccessful commands and reopened notifications cannot resolve incidents. Recovery requires the authoritative transition and never resurrects retired associations or rejected measurements. Notification activation revalidates current context and performs navigation without granting clock or trust authority.

The proposed CA-wide grouping initially raised a possible escalation concern: an already announced source issue must not hide a later loss of all usable estimates. I did **not** elevate that to a defect. Section 9.6 explicitly retains the important-event policy, and the linked v0.4 reference distinguishes important aggregate conditions; its prototype notification context includes severity. A future EV-09 fixture should exercise escalation within one CA incident, but the present text does not authorize suppressing the important transition.

Microsoft’s current notification guidance supports the plan’s controlled notification frequency and context-specific activation. It also advises retiring old Notification Center entries consistently. That is useful presenter implementation guidance; it does not require a recovery toast or deletion of the service’s incident history. [Microsoft: Notifications design basics](https://learn.microsoft.com/en-us/windows/apps/develop/notifications/app-notifications/app-notifications-ux-guidance)

### Accessibility and live announcements

The plan maps future presentation cases to CHECK-22 and retains reviewed visible/spoken wording. Canonical §16.3 and the selected UI skills already require meaningful UIA state exposure, keyboard access, focus stability, controlled announcements, non-color meaning and access to critical recovery without the tray.

I found no contradictory instruction to announce every feed refresh, move focus on source changes, or treat HTML checks as native accessibility evidence. Adding a blanket WCAG implementation requirement to this deferred plan would duplicate existing obligations and improperly expand the current milestone.

The current WCAG2ICT guidance explicitly recognizes the need to expose status messages through platform accessibility services without moving focus. It supports the inherited native UIA/Narrator approach; it supplies no standalone certification or implementation evidence. [W3C: WCAG2ICT — Status Messages](https://www.w3.org/TR/wcag2ict-22/#status-messages)

### Privacy, hostile content and localization authority

The plan limits notifications to a generic problem and safe review route, keeping private hostnames, certificate names, fingerprints and paths out of lock-screen messages. Detailed identities stay controlled and bounded. Keys, cookies, TLS exporters, raw certificates and arbitrary error chains remain excluded.

Service facts are typed projections, never policy parsed from logs. The linked localization contract already requires literal sinks, sink-specific escaping, fixed action/activation routes, code-owned severity and independently approved complete critical wording groups, including accessible variants. Section 9.6 does not weaken those rules. No additional locale is silently adopted.

The diagnostic export path remains user initiated and redacted. The plan does not grant a privileged service an arbitrary export path or authorize automatic uploads.

### Safe next actions and authority

Bounded retry/check, configured-source inspection, diagnostics and redacted export are consistent with existing authority. The explicit prohibitions on “Trust anyway,” disabling revocation, importing unsolicited roots, changing the date to bypass checks and automatic NTP fallback prevent misleading recovery actions.

## Inputs inspected

Repository procedure and skills:

- Root `AGENTS.md`; applicable v0.4 nested `AGENTS.md`.
- Documentation guide’s authority, plan-routing and UI-reference sections.
- `plan-independent-review` and `planning/references/review-and-handoff.md`.
- Testing and security contracts.
- `implement-native-ui`, its interaction reference, `implement-slint`.
- `security-native-ui`, `security-privacy-logging`.
- `qa-accessibility`, `qa-i18n-localization`.

Product and design authority:

- Target plan §§1–11, including all 27 event rows and EV-01–09.
- Owner decisions OD-13, OD-15 and OD-20.
- Architecture runtime states, diagnostics/privacy, UI/localization/accessibility and applicable CHECK entries.
- Feature F-04/F-05/F-09/F-10, Sources, state/freshness presentation and acceptance scenarios.
- Localization presentation, hostile-catalog meaning/fallback and sink contracts.
- Slint GUI handoff’s implemented fixture scope and unimplemented/unqualified scope.
- v0.4 overview/usage references, tray report, status-notification report, and relevant notification model functions.

Two current external primary pages were checked: Microsoft **Notifications design basics** and W3C **WCAG2ICT**, including status-message guidance. No broader external survey was needed.

## Limits and next action

This review assesses deferred document readiness. It establishes no native delivery, Narrator speech, UIA tree, layout, parser safety or service-enforcement result. Open freshness/profile decisions, finite implementation budgets and native/environment admissions remain under their existing owners.

**Permitted next action:** Correct the eligibility-dependent message rule and its EV-03 presentation pair in the existing plan, then recheck only those rows against LIFE-02. With that correction, this UI/UX scope is ready as a deferred post-V1 planning contract.

## R4. Contracts and QA — original reviewer return

The proposed change adds post-V1 OneCRL and CRLite rejection checks to NevroTick’s existing Windows certificate validation, then carries the resulting trust context through NTS association use, sample eligibility, broker authorization, and truthful status. The new event matrix keeps certificate verdicts, feed availability, source eligibility, and aggregate time state separate. It requires bounded invalidation and fresh recovery without treating feed success as time agreement or reviving retired authority.

The inspected system still contains a default-off NTS association prototype. Its Windows transport configures CNG, TLS 1.3, the platform verifier, and disabled resumption, then transfers same-connection exporters and cookies through `InternalCandidate` and the guarded `AssociationOwner::receive`. Those inspected transfers do not contain the proposed feed receipts. Current repository authority also keeps production acquisition, native/service/package qualification, and the supplemental-feed work separate; this plan does not establish those implementations or results.

## Verdict

**REVISE — the deferred plan’s implementation/QA handoff needs one compatibility obligation made explicit.**

This is a documentation-stage finding, not a demonstrated runtime defect, a new V1 blocker, or authorization to start TRUST implementation. I found no separate material defect in the plan’s source-to-effect race coverage, independent fixture requirement, healthy-neighbor requirement, evidence-layer separation, or finite-budget/exit rules.

## TRUST-CQ-01 — Add application and stored-state compatibility to the activation gates

**Severity:** Medium, planning/handoff.

**Location:** `docs/plans/post-v1-certificate-distrust.md`, §5; §6 items 4–5; TRUST-W02–W05 in §8; event envelope in §9.1; §9.7 acceptance additions.

**Requirement:** The inspected `evolve-data-contracts` skill and transition reference require the old/new producer, reader, and stored-generation matrix before dependent changes. Architecture §5.2 and §6.4 require explicit supported protocol/schema behavior; §19.2 specifically warns that one complete package cannot eliminate old in-memory processes. Release policy §6.1 requires compatibility to be frozen before new mandatory-state writes, including mixed processes, interrupted migration, and rollback.

**Evidence:** The plan introduces versioned event meanings and security snapshots that may become mandatory state. It explicitly preserves anti-rollback state across reinstall/upgrade, qualifies feed format transitions, and requires installed-lifecycle evidence. However, neither its W02 contract exit nor W03–W05 acceptance identifies the compatibility disposition of older UI/engine/broker/helper consumers and previously stored generations. Its named rollback cases concern older signed feed revisions and feed-install recovery. They do not explicitly cover application downgrade, an old process surviving package replacement, or an older reader consuming new trust-policy/status meaning.

**Consequence:** The future integration could meet the listed new-path and feed-update tests without determining whether supported older components safely reject, correctly project, or accidentally omit the new authority requirements. That matters especially if an activated assurance mode makes the supplemental snapshot mandatory. An intact anti-rollback record alone does not establish that older code understands or enforces its meaning.

**Minimum correction:** Add a short W02 activation requirement and corresponding W05 acceptance obligation:

- Inventory the affected persisted security records, event/status projections, and independently consumed interfaces; record the required old/new writer, reader, and stored-generation combinations as supported, deliberately unsupported, or unverified before dependent writes.
- Assign the transition owner and define supported update/downgrade behavior, mixed-process rejection or compatibility, protected migration through the same checked constructors, interrupted publication/reconciliation, and safe rollback without resetting anti-rollback history, freshness, or authority.
- Require independent raw old/new fixtures through the actual required consumers in both applicable directions, plus successful neighboring transitions and interruption/rollback cases in the eventual installed lane.
- Where no old state or independently surviving consumer exists, record that boundary explicitly instead of inventing migration work.

This requires **no concrete future version numbers, selected schema, dependency, implementation, or native execution now**. A concise reference to the existing transition/release-policy owners is sufficient.

**Counterevidence considered:** §6 already has unusually good feed-update continuity requirements; §7 explicitly treats old-format rejection as a compatibility result; §9.1 includes an event schema version; W05 retains installed qualification; and the broader repository already mandates safe version transitions. Those provisions narrow the gap, but they do not identify this feature’s application/state transition obligation in its own staged handoff.

## Other scoped conclusions

- **Contract enforcement:** §5 explicitly requires private fallible construction and existing enforcement conventions; §4 requires sealed same-verification custody; §9.5 names the actual downstream authority guards. Read together with architecture §6.3.1 and CHECK-25, these retain mandatory constructor, transformed-output, direct-caller/bypass, and release-active enforcement requirements. I do not treat the absence of finalized Rust type definitions as a present defect.
- **QA discrimination:** §9 requires independent expected results, actual production entries/consumers, and useful accepted neighbors. §9.7 strengthens this by forbidding reliance on a duplicate policy model or formatter. EV-04 reaches the broker’s final effect-issue boundary; EV-03, EV-05, EV-07, and EV-08 cover meaningful temporal and recovery distinctions.
- **Evidence limits:** §8 separates source-first work from controlled native and installed qualification, while §11 explicitly denies implementation, dependency, native-test, and release credit from the documentation review.
- **Budget and exit:** The effort budget being TBD is appropriate for this deferred stage because §10 requires a finite total preparation/review/implementation/test budget and attempt count at activation, retains consumed effort, and requires stopping at the bound. TRUST-C01 does not silently advance on an incomplete attempt.

## Inspected baseline and authorities

**Frozen baseline:** `6acd1e504033c1b57992cb0823667f5e6a8bff46`; target NV-TRUST-001 **revision 1.1**. The initial working-tree check was clean. Target and supporting reads used `git show` against that commit, so later coordinator edits are outside this verdict.

**Review completion:** Completed, read only; no edits, builds, tests, or product execution. I did not rely on the plan’s earlier reviewer approvals as independent evidence.

**Skills and shared contracts inspected:**

- `.agents/skills/planning/plan-independent-review/SKILL.md`
- `.agents/skills/planning/references/review-and-handoff.md`
- `.agents/skills/testing/CONTRACT.md`
- `.agents/skills/testing/qa-test-planning/SKILL.md`
- `.agents/skills/testing/qa-contract-tests/SKILL.md`
- `.agents/skills/implementation/CONTRACT.md`
- `.agents/skills/implementation/implement-rust-contracts/SKILL.md`
- `.agents/skills/implementation/implement-rust-contracts/references/contract-enforcement.md`
- `.agents/skills/implementation/evolve-data-contracts/SKILL.md`
- `.agents/skills/implementation/evolve-data-contracts/references/transitions.md`

**Repository authority and source inspected:**

- Applicable `AGENTS.md` instructions and `docs/README.md` current-state/routing sections.
- Entire target plan.
- `docs/nevrotick-owner-decisions.md`, OD-20.
- `docs/windows-time-architecture.md`, relevant portions of §§5.1–5.3, 6.1–6.5, 12.3.1, and 19.1–19.2.
- `docs/release-policy.md`, §6.
- TRUST-0 assignments in `tasks/s0.1-crypto-feasibility.md` and `tasks/engine-transport-pilot.md`.
- `crates/nevrotick-engine/Cargo.toml`.
- `nts_ke_experiment/transport.rs`: `InternalCandidate`, `publish`, and `connection`.
- `nts_ke_experiment/association.rs`: `AssociationCandidate` and `AssociationOwner::receive`.

**Excluded scope:** Re-verification of external Mozilla/Microsoft/CRLite factual claims; complete source audit; dependency admission; selection of future assurance modes, schemas, limits, or tools; execution of compiler controls or behavioral/native/package tests; review of later working-tree changes.

**One permitted next action:** The coordinator can add the bounded compatibility gate and future acceptance obligation to the existing plan under the current documentation-only scope.

## J1. Integrated correction check — original reviewer return

The proposed revision keeps OneCRL and CRLite as deferred, supplementary rejection mechanisms for NevroTick’s own certificate authentication. It repairs the planning handoff so that a trust change has an identified route from the accepted certificate path through association and sample/model eligibility to the broker’s final effect boundary and actual outcome settlement. It also assigns application/state compatibility before dependent writes, makes certificate-event wording depend on current source eligibility, and distinguishes the strict laboratory verifier from the shipping policy still owned by OPEN-8. These are documentation corrections; they do not select the remaining activation decisions or start implementation.

The inspected engine manifest still makes the KE association prototype optional, with Windows-only TLS dependencies. The connection source constructs the CNG-backed TLS 1.3 configuration and platform verifier, disables resumption and early data, and passes same-connection exporters and cookies through private candidate and association owners. The inspected transfer fields do not contain the proposed supplemental-feed receipt. Current architecture and task records keep production trust-policy selection, integration, native/service/package qualification and TRUST-0 activation open. An existing prototype guard or a written downstream contract is therefore no evidence that the future trust-to-clock path already works.

## Verdict

**READY — NV-TRUST-001 revision 1.2 is ready for completion of this deferred documentation audit.**

All four reported corrections are verified against the frozen candidate for their documentation guarantees. I found no new actionable defect in the changed ownership, compatibility, presentation or policy-authority boundaries. This verdict does not close TRUST-C01, approve implementation, qualify a release, or count reviewer agreement as behavioral evidence.

The permitted sequence remains **actual V1 release → MATH-C01 → TRUST-C01 → MT-C01**, followed by the existing later roadmap work. The plan and both handoffs retain the current Windows, fuzz, storage and UI holds and existing V1 requirements.

## Correction dispositions

### ARC-01 — Verified correction

**Affected requirement:** traceable producer/consumer ownership, broker-local current authorization, serialized final issue and separate actual-outcome settlement. The relevant authorities are `plan-architecture`, its data/state reference, `security-local-ipc`, architecture §§5.2, 6.4 and 10.2.1, and the lifecycle QA skill.

**Revised evidence:** `docs/plans/post-v1-certificate-distrust.md` §8 assigns existing Core/IPC/broker owners to W02/W04 and their native/installed consumers to W05. Section 9.5 now requires the canonical trust-context writer, source/association/model scope, process incarnation, supported schema, broker-accepted context and acknowledgements bound to the exact applied transition. It explicitly distinguishes local admission, pending delivery and effectiveness at the receiving consumer. EV-04 covers delayed, missing, duplicate and reordered notifications/acknowledgements and one-process restart beside an unaffected-source control. Section 11 carries closeout through accepted chain → association → sample/model → proposal → broker final issue → outcome/settlement.

The TRUST-0 addenda in `tasks/s0.1-crypto-feasibility.md` and `tasks/engine-transport-pilot.md` now carry that same assignment. The crypto task remains coordinator; the engine task coordinates reached downstream owners rather than taking over the broker’s authority.

**Opposing cases checked:** an engine-local generation or queued message cannot stand in for broker application; unresolved required context withholds affected authority; a prior process acknowledgement cannot authorize a restarted process; an unaffected source can continue when its dependencies are established. The unchanged retirement and outcome clauses still retain native custody, account for already delivered effects and prevent stale feedback from restoring future authority. The broker gains neither feed-fetch authority nor an obligation to duplicate certificate verification.

**Conclusion:** the missing stage/task/closeout assignment is repaired. A concrete cross-process protocol and reaction bound remain required at W02 activation; choosing them during this audit would exceed the requested stage.

### UX-01 — Verified correction

**Affected requirement:** truthful source-role presentation and corresponding accessible wording under CHECK-14/CHECK-22, architecture §16.2–16.4 and `implement-native-ui`.

**Revised evidence:** plan §9.1’s target context distinguishes replacement candidates from published associations. The shared rule in §9.2 binds visible/spoken usage, reason and aggregate state to current published eligibility in one coherent snapshot. CERT-01/02/04 no longer attach unconditional source-exclusion copy. LIFE-02 retains the independently valid older association only while its own receipt and limits remain valid. EV-03 now requires both the preserved-association presentation and actual-exclusion presentation; the engine handoff explicitly assigns this consumer distinction to the UI owner.

**Opposing cases checked:** certificate B can fail a replacement while the existing association established with unaffected certificate A remains usable. The revised wording reports the failed refresh and existing association rather than falsely excluding the source. If the prohibition also matches A, LIFE-02 and §9.5 still retire its authority and the message reports actual exclusion. Neither branch changes aggregate Tracking merely from a certificate event: the remaining agreement/readiness requirements still control it. Feed refresh, fresh TLS and incident dismissal still cannot manufacture synchronization.

**Conclusion:** the presentation contradiction is repaired without changing source eligibility or weakening rejection. The wording remains proposed English content, with the existing exact-content, localization and native accessibility owners retained.

### TRUST-CQ-01 — Verified correction

**Affected requirement:** explicit compatibility before dependent writes, protected checked migration, mixed-process safety and authority-preserving rollback. The relevant authorities are `evolve-data-contracts`, its transition reference, architecture §§5.2, 6.4 and 19.2, and release policy §6.

**Revised evidence:** new plan §5.1 separates feed encoding from application compatibility. It inventories writers, decoders, mandatory constructors, publication points and consumers for snapshots, anti-rollback state, receipts, status/IPC and surviving association/model references. Required old/new writer–reader–stored-generation combinations must be classified before writes. The scope includes UI, engine, broker and the maintenance owner, including old processes that survive package replacement.

Migration must validate original version/provenance and use the checked construction and current authorization of fresh state. The text separates prepublication rejection from a replacement whose acknowledgement was lost, requires disk/service reconciliation before retry, and prevents code rollback from restoring consumed authority, lowering anti-rollback history or fabricating freshness. It explicitly limits claims about offline disk/VM rollback.

W02, W05, §9’s application-compatibility case, §§10–11 and both task addenda now carry these obligations. W05 requires independent old/new raw fixtures through actual readers/consumers, incompatible refusal, supported healthy neighbors and interrupted migration/restart/rollback. The linked transition procedure retains both applicable directions and separate retirement of old writers, readers, data and migration paths.

**Opposing cases checked:** supported combinations must preserve meaning; unknown required semantics cannot silently default to acceptance or green status; deliberately unsupported combinations have a declared refusal; unverified combinations do not become passes. Where no old data or independently surviving reader exists, the plan requires an explicit boundary rather than unnecessary migration work. A complete package is not treated as proof that every old process has stopped.

**Conclusion:** the application/state-transition gap is repaired. No schema version, hardware rollback mechanism, compatibility window or new updater has been selected.

### SEC-01 — Verified correction

**Affected requirement:** preserve OPEN-8’s ownership of the shipping revocation/offline profile and distinguish that unresolved policy from laboratory source observations.

**Revised evidence:** plan §9.1 precedence rule 4 now routes unknown native revocation to the policy approved under OPEN-8 and explicitly identifies the current strict cache-only verifier as laboratory-specific. This matches architecture §18.7.2, the ordinary S2.2 trust-profile card, the strict laboratory child and the crypto TRUST-0 addendum.

**Opposing cases checked:** the correction neither adopts universal soft failure nor requires an unselected strict-online profile. Known applicable rejection still wins under §3/§9.1; DATA-02 still refuses dependent authority when required native checks cannot be completed; optional leaf-evidence substitution remains separately unapproved.

**Conclusion:** the policy-authority ambiguity is repaired with no new exception or weakened trust rule.

## Integrated consistency and review record

The revised plan keeps one canonical technical/evidence chain. Existing owners retain certificate verification, network acquisition, Core admissibility, broker effects, maintenance and UI presentation. The additions make inherited obligations explicit at the relevant post-V1 stages; they do not introduce a new V1 gate or move an existing gate merely to avoid unavailable evidence.

The supporting record’s R1–R4 bodies compare verbatim, after surrounding-whitespace normalization, with the four supplied reviewer files. Their original comprehension, findings, limitations and verdicts are preserved. Its finding table correctly marks the edits as applied with the integrated check pending; those are pre-completion bookkeeping fields, not a substantive defect. The coordinator can now record the dispositions above and preserve this return. I did not independently reconstruct the original four launch traces; their fresh-context settings remain coordinator-recorded metadata.

No actionable new finding is raised. The assurance mode, freshness/currentness proof, reaction bounds, historical-model treatment, root-policy scope, concrete compatibility matrix and finite execution/resource budgets remain intentionally unresolved for activation. Their named preceding gates make that deferral appropriate for this documentation stage.

## Candidate identity, completion and actual inputs

**Reviewer:** `/root/plan_integrated_correction_audit`. Fresh task context, with the original reviewer returns supplied as correction inputs and no inherited parent conversation. Review completed on 14 November 2026. The repository was read only throughout; the sole permitted write is this review’s archival file outside the repository.

**Baseline HEAD:** `6acd1e504033c1b57992cb0823667f5e6a8bff46`.

The frozen manifest and current Git object identities matched at initial and final inspection:

| Reviewed file | Git blob |
| --- | --- |
| `docs/plans/post-v1-certificate-distrust.md` | `61177dbf75f8c5c3853ef81f877e75deaea958eb` |
| `tasks/s0.1-crypto-feasibility.md` | `170403400854deeaff228736725d48fc70810ef6` |
| `tasks/engine-transport-pilot.md` | `03804a1d1ec02d5c89eca8b9ac244afe3b45791d` |
| `docs/supporting-reviews/certificate-distrust-plan-audit.md` | `4e40b2ddac2a3ddc269d8a7d5beb02d766f2a6b1` |

The report hash identifies its pending-integrated-review version. Recording this completed return will change that archival document’s identity; this verdict remains bound to the reviewed normative candidate.

**Inputs actually inspected:**

- Root `AGENTS.md`; relevant documentation-guide current-state, authority and TRUST-0 routing sections. No nested instruction file applies to the reviewed paths.
- `plan-independent-review` and `review-and-handoff.md`; testing, implementation and security contracts.
- `plan-architecture` and its data/state reference; `evolve-data-contracts` and its transition reference; `security-local-ipc`; `implement-native-ui`; `qa-state-concurrency-lifecycle`.
- The complete revision 1.2 plan, its tracked predecessor diff, both complete TRUST-0 addenda, relevant ordinary/strict laboratory trust cards, and the adjacent engine MT-0 order.
- The supporting audit record and all four original role returns.
- OD-20; architecture §§5.2–5.3, 6.4–6.5, 10.2.1, 16.2–16.4, the relevant §18.7.2 profile/deferral clauses and §19.2; release policy §6 compatibility/recovery clauses.
- Engine `Cargo.toml`; bounded source reads of `transport.rs::InternalCandidate`, its publication transfer and `connection`; `association.rs::AssociationCandidate` and `AssociationOwner::receive`.

**Performed document/source checks:** candidate hashes and Git state; normative diff inspection; raw-return preservation comparison; `git diff --check` on the three tracked changed documents, which returned no errors. The product paths, tools and Cargo files showed no diff from the baseline. These are document/source observations, not product test results.

## Limits and one permitted next action

This is a bounded correction review, not a fresh whole-product audit or a survey of every repository skill. Unchanged external technical research and broader source observations from the initial role reviews were not independently repeated. No Cargo, compiler-control, fuzz, formal, native Windows, UIA/Narrator, installed-lifecycle, feed, clock, service, trust-store or release operation was executed. No dependency, implementation or production policy was admitted.

Reopen the affected disposition if the normative candidate, trust semantics, owner boundaries, supported compatibility scope or canonical decisions change. Reinspect the actual released product and time-sensitive upstream evidence at TRUST-W01 as already required.

**One permitted next action:** finalize the existing audit record with this completed READY verdict and the four verified documentation corrections, retaining TRUST-0’s deferred status, accepted order and current execution holds.

**Archival copy:** `/workspace/scratch/dc7d02421f60/trust-audit-working/integrated-review.md`.
