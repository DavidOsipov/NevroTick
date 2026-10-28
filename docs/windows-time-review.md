# Full independent review of the Windows time-client design

**Review run:** RWTC-REVIEW-20261028-ALL-01  
**Reviewed baseline:** RWTC-ARCH-001 revision 1.3  
**Corrected documents:** revision 1.4, 28 October 2026  
**Method:** three fresh-context reviewers, followed by coordinator reconciliation and a bounded independent correction recheck  
**Stage assessed:** readiness for bounded P0 feasibility investigations

## Outcome

**The corrected design is ready for the bounded P0 investigations defined in the plan.** The review found six documentation findings across four themes, plus one source-derived dependency validation lead. The documentation findings were corrected and independently rechecked. The dependency lead remains an explicit future qualification case, not a reproduced application vulnerability.

The review does not qualify an implementation, an installed dependency graph, real Windows security controls, measured performance, accessibility, public signing or ASVS conformity. All 345 ASVS records remain **Not implemented / Not executed**, with draft classifications requiring owner review. No policy exception is accepted merely because reviewers agree that it needs a decision.

The complete maintained design is available in:

- [windows-time-architecture.md](windows-time-architecture.md) — architecture, threat model, dependency choices, lifecycle and implementation gates.
- [windows-time-diagrams.md](windows-time-diagrams.md) — the five editable Mermaid views.
- [asvs-5.0.0-applicability.md](asvs-5.0.0-applicability.md) — every canonical requirement, proposed scope, open questions, original planned verification and explicit correction history.

## 1. Review scope and independence

Each reviewer received a fresh context with the task, the three frozen files, exact hashes, canonical ASVS descriptions, an assigned chapter partition and relevant skills. They did not inherit the author's conversation history or receive a desired verdict. Each read the complete main specification, all five diagrams and worksheet front matter. The semantic ASVS work was divided across the team; **the claim is complete collective review of all 345 clauses, not that every reviewer individually reviewed all 345**.

The coordinator verified that the primary partitions contain exactly 345 unique canonical IDs, with no omitted IDs and no duplicated primary assignments. Every assigned row has an individual review reason. All reviewers recorded the complete 159-heading baseline main document. The revised main document has one additional subsection for the explicit applicability reconciliation.

| Reviewer | Principal perspective | Assigned ASVS chapters | Clauses |
| --- | --- | --- | ---: |
| Architecture | Numerical/source-selection design, authority, clock and lifecycle semantics, complete-package behavior, dependencies | V2, V5, V13, V14, V15, V16 | 98 |
| Security | Trust boundaries, threat paths, local identity, NTP/NTS/bootstrap, cryptography, Windows integration and release risks | V6, V7, V8, V9, V11, V12 | 122 |
| Traceability and QA | Requirements-to-evidence mapping, full clause scope, honest evidence states, UX/accessibility, diagrams and document integrity | V1, V3, V4, V10, V17 | 125 |
| **Total** | **All three maintained documents; every canonical clause has a primary reviewer** | **V1–V17** | **345** |

Initial observations were retained before cross-review reconciliation. The author-written current-state capsule exposes historical readiness assertions to anyone reading the complete plan. The architecture reviewer additionally disclosed an accidental early read of the embedded historical-review section. These assertions were treated as author claims, not independent evidence. No old independent-review report or peer report was used to establish a first-pass verdict. This is independence of context and examination within an agent review team, not an external organizational audit or independent laboratory certification.

After the first passes, the coordinator requested a precise server/client HTTP distinction. That follow-up found that V4.2.1–V4.2.3 also need conditional scope decisions. Their addition is recorded as post-first-pass reconciliation, not presented as three blind discoveries. The frozen baseline remained unchanged throughout the review; corrections were prepared as a separate candidate and rechecked before finalization.

## 2. Skills actually supplied and applied

The project-specific guidance came from the user's repository at commit `6530d3a8d44cbe4f829b1e09edd3a1c90cffc087`. All reviewers read `AGENTS.md`, the testing contract and `plan-independent-review`, plus their assigned skills and relevant references. The security contract and shared QA phase guidance supplied common evidence and phase rules. These were actual reads, not merely skill names attached to a task. [Pinned guidance repository](https://github.com/DavidOsipov/personal-website-templated/tree/6530d3a8d44cbe4f829b1e09edd3a1c90cffc087).

| Reviewer | Principal skills and references | How they affected the review |
| --- | --- | --- |
| Architecture | `plan-architecture`; data authority, contracts and module boundaries; `qa-state-concurrency-lifecycle`; `qa-fault-recovery`; `dependency-updates` in discovery-only mode | Traced who owns decisions and effects, late/cancelled work, actual correction acknowledgements, startup/stop/repair/rollback and exact dependency admission. Tested the prose against concrete counterexamples. |
| Security | `plan-threat-model` and ASVS applicability; `security-crypto-secrets`; `security-build-deployment`; `qa-security-boundaries`; available `security-audit` guidance in source-first design mode | Distinguished documented controls from executed evidence, checked full cryptographic requirements and source-backed Windows behavior, and preserved unknowns rather than inventing vulnerabilities. |
| Traceability and QA | ASVS applicability reference; `qa-coverage-gap-analysis`; `qa-test-planning`; `qa-accessibility`; `qa-final-qualification` | Compared complete canonical clauses, including stronger L3 conditions inside lower-level entries; mapped every requirement to an observable gate; checked valid/invalid controls, honest coverage and native accessibility evidence. |
| Coordinator | `plan-independent-review`, `qa-subagent-dispatch`, shared working-state/phase guidance and security design guidance | Froze inputs, assigned ownership, preserved independent first passes, reconciled overlapping findings, checked the full coverage union and sent corrections back for targeted review. |

Website-specific framework commands, browser mechanisms, SEO requirements and npm workflows were not converted into Windows product requirements. No supplied skill or repository policy was modified. The review did not create an implementation, install candidate crates or execute third-party build scripts.

## 3. Findings and dispositions

Priorities below concern the design documents. They are not assigned severities for a running product.

| Finding | Priority or evidence class | Finding and correction | Final disposition |
| --- | --- | --- | --- |
| DOC-1 | Low document priority | V1.2.3 still asked whether JSON was selected, despite DEC-18 and the strict Serde contract. The active rationale now records JSON and the answered question is closed. | Documentation correction rechecked; serializer implementation/tests remain future work. |
| DOC-2 | Medium document priority | V4.2.1/2/3/5 used generic no-HTTP or future-only retrieval rationales. The current held native verifier can introduce client-side HTTP responsibilities. Each now has its own OPEN-8 scope and evidence question. | Four rows changed from proposed N/A to Needs decision; no HTTP implementation or runtime defect inferred. |
| ARC-1 | Low document priority | V15.3.2 excluded external URL redirects even though native certificate retrieval is already a conditional candidate path. Redirect intent, hops and final destinations now have a named qualification gate. | Row changed to Needs decision; actual Windows redirect behavior remains unexecuted. |
| SEC-1 | Low document priority | V11.5.1's entropy requirement was narrowed by treating legacy NTP values as non-bearer fields. The full requirement concerns all intended non-guessable values. The plan now states the NTS UID minimum and an explicit legacy-NTP exception decision. | Row changed from direct applicability to Needs decision under OPEN-9; no exception or compliance pass granted. |
| SEC-2 | Low document priority | V12.2.1 treated certificate HTTP retrieval as future-only. Its current conditional transport scope and possible plain-HTTP PKI-material exception now belong to OPEN-8. | Row changed to Needs decision; cryptographic object signatures are not described as transport encryption. |
| SEC-3 | Low document priority | V6.1.3 named UI-to-engine status, while the architecture publishes engine snapshots through the broker. The row now names UI-to-broker status and engine-to-broker publication. | Endpoint wording corrected; no new direct UI-engine interface added. |
| SEC-4 | Source-grounded lead; runtime effect needs validation | The pinned Windows verifier has a date-conversion intermediate that exceeds `u64` for a sufficiently wrong future date. Exact source and independent arithmetic establish a bounded provider test case. | Added to existing OPEN-8/CHECK-9/27 qualification. Runtime behavior and exact artifact parity remain unverified. |

### 3.1 Conditional HTTP scope is a real design question

An application can have no web server and still initiate HTTP requests indirectly through a certificate verifier. Microsoft's chain-building documentation describes URL-based retrieval, distinct cache-only controls and some delegated `cryptsvc` behavior. This supports retaining a conditional retrieval surface; it does not establish that every fetch is owned by the engine or that all OS background traffic belongs to the product. [Microsoft CertGetCertificateChain documentation](https://learn.microsoft.com/en-us/windows/win32/api/wincrypt/nf-wincrypt-certgetcertificatechain).

The six revised HTTP-related clauses cover different properties: received-message framing, generated-message lengths, HTTP/2/3 connection-specific fields, receiver-compatible URI/header lengths, transport encryption and intended redirects. OPEN-8 must identify actual components, protocol versions, direct/delegated ownership and the assurance retained from Windows or a selected library. Generic NTP/IPC size bounds do not establish HTTP framing or receiver limits. A destination allowlist does not by itself establish that redirects are intended.

The correction does **not** require adding an HTTP server or reimplementing Windows HTTP. It requires an honest assessment boundary and evidence before a route is admitted. If the chosen route proves a condition absent, an appropriately scoped N/A may be justified. If a native platform supplies the property, its evidence and limits must be explicit. If a literal requirement is not met, the deviation needs an owner decision.

Six server/sensitive-action exclusions were narrowed without changing their classifications: V3.5.3, V4.1.1, V4.1.2, V4.1.4, V4.2.4 and V13.4.4. The V14.2.1 rationale now separately requires inspection and classification of actual URL contents, including private-deployment identifiers; it does not assume every certificate identifier is either secret or public.

### 3.2 Legacy NTP entropy needs an explicit exception

The complete ASVS V11.5.1 clause requires a cryptographically secure generator and at least 128 bits of entropy for values intended to resist guessing. It has no bearer-credential limitation. A legacy NTP origin/correlation field or source port cannot independently satisfy that floor merely because its purpose differs from a password-reset token. Adding field widths is not proof of per-value compliance. [Pinned ASVS 5.0.0 source](https://github.com/OWASP/ASVS/blob/5cf9b032440be53ce345ab3c130fda46ba1ce7a2/5.0/docs_en/OWASP_Application_Security_Verification_Standard_5.0.0_en.json).

The NTS UID is a different case: RFC 8915 requires at least 32 CSPRNG-generated octets. The updated plan makes that minimum explicit. Retaining the requested legacy NTP support requires a transparent scoped entropy exception under OPEN-9/S0.7, with the actual generator, uncertainty, safeguards and residual off-path risk documented. An optional standalone UID extension requires interoperability evidence; it is not silently assumed to work with every legacy server. [RFC 8915 section 5.3](https://www.rfc-editor.org/rfc/rfc8915.html#section-5.3).

### 3.3 Extreme-date verifier case

The inspected `rustls-platform-verifier` Windows source at commit `252e25161a91af476cbea620e29d277592d27ec7` computes a FILETIME interval by multiplying seconds since 1601 by one billion and then dividing by 100. The multiplication occurs in `u64`. Its first overflowing whole-second input corresponds to **2185-07-21 23:34:34 UTC**, or Unix second **6,802,270,474**. The security reviewer, coordinator and targeted architecture reviewer independently checked the source/arithmetic. No provider code was executed. [Pinned conversion source](https://github.com/rustls/rustls-platform-verifier/blob/252e25161a91af476cbea620e29d277592d27ec7/rustls-platform-verifier/src/verification/windows.rs#L440-L456).

The threshold calculation is:

```text
floor((2^64 - 1) / 1_000_000_000) + 1 - 11_644_473_600
= 6_802_270_474 Unix seconds
```

This matters for the requested recovery behavior: a grossly wrong system date can reach normal TLS verification before a provisional helper date is used. Actual effects depend on the selected artifact, source parity, compiler/profile, earlier guards and callback handling. The review therefore records a qualification lead, not a tested vulnerability or a claim about every version of the library.

The existing provider gate now requires defined supported date ranges and checked conversions for both normal and provisional verification. It must reject an unsupported date into the explicit recovery/Blocked path, or qualify a corrected provider. It must not silently clamp or wrap the date into one that makes a certificate appear valid. A controlled injectable-time harness should cover a valid ordinary date, the prior second, the boundary and year 2200, separating certificate validity from conversion behavior and confirming subsequent valid work. Supplied-time injection can test the conversion without changing an ordinary host clock. This evidence remains unexecuted.

## 4. What changed in revision 1.4

Exactly **16 ASVS records** have explicit current-field corrections, comprising **45 changed fields**. Seven classifications changed. The other **329 records** preserve their original field values. The worksheet retains previous values for every corrected field, the reason for the change and current planned verification. All original planned-verification text and revision 1.1–1.3 additions remain present.

| Measure | Frozen revision 1.3 | Corrected revision 1.4 |
| --- | ---: | ---: |
| Canonical ASVS requirements | 345 | 345 |
| Directly applicable, proposed | 110 | 109 |
| Native adaptation, proposed | 28 | 28 |
| Proposed N/A | 187 | 181 |
| Needs decision | 20 | 27 |
| Rows with an open question | 33 | 39 |
| Implemented controls | 0 | 0 |
| Executed application-control checks | 0 | 0 |

The six newly conditional HTTP rows and the entropy row add seven explicit questions; closing the answered JSON question yields the net increase of six. This is a more accurate remaining-decision inventory, not a reduction in implemented security.

The main plan adds the corresponding scope/entropy explanations, the date-conversion admission case and references to existing owners and checks. Its requirements, decisions, boundaries, threats, acceptance checks and open-decision identifiers remain stable. No new process, privileged interface or distribution split was introduced. All five Mermaid source blocks are unchanged and match their companion exactly.

## 5. Architecture assessed and boundaries retained

The reviewers traced concrete failure paths through the documented design. Their conclusions concern internal coherence and adequate next-stage gates; they do not establish that the future implementation will satisfy them.

| Design question | Assessment and remaining proof |
| --- | --- |
| Can the normal-user UI securely control time synchronization? | The UI requests permitted operations through authenticated local IPC. The network engine lacks clock-setting privilege; the broker owns current authorization and clock effects. Actual token, process and pipe behavior remains S0.2/S0.3 evidence. |
| Can a compromised engine repeatedly exploit individually permitted corrections? | The plan accounts for actual applied steps and rate-integrated correction, generation freshness and retained cumulative budgets. This bounds permitted damage rate; it does not make a compromised engine a source of true UTC. |
| What happens after sleep, an external time step or lost acknowledgement? | The model separates time domains and generations, rejects stale work and reconciles actual broker state. Unknown clock effects are not treated as safely retryable commands. Windows effects remain controlled native tests. |
| What happens if the broker dies during a slew? | Current ownership is required before cleanup/reset writes; the plan retains the process-death residual and measured recovery gate. An old journal or matching numeric rate does not prove authority over a successor. |
| Can plain-NTP helpers silently weaken NTS? | Helpers affect only an isolated provisional certificate time. They do not directly set the system clock or become normal authenticated voters. Expired-key/bootstrap residuals and provenance remain explicit. |
| Does a timeout stop blocked Windows certificate work? | The design keeps admission with the actual job until return, limits queues/bytes/workers and rejects stale results. Handshake execution and stop/restart behavior require native qualification; a timeout is not assumed to preempt a callback. |
| Is installation one product? | The complete MSI contains the UI, services, helper and required payloads, with one installation/update/repair/removal lifecycle. A future MSIX must meet the same complete-product contract. No split UI/service package is reintroduced. |
| Are accessibility and lightweight operation measurable? | The plan requires real UIA, Narrator, keyboard/focus, contrast/scaling and representative interaction evidence, plus resource measurements with the supported security protections active. A toolkit capability declaration is not a pass. |
| Does using Rust or strict linting establish security? | The plan retains explicit FFI, parser, dependency, native mitigation, secret, lifecycle and independent numerical verification obligations. Tool success must be tied to real discovery and meaningful valid/invalid cases. |

The architecture reviewer also checked selected pinned chrony source-selection/combination behavior and the independent numerical example. This was a bounded source-consistency check, not an audit of the entire chrony tree or proof of an unwritten Rust port. The source register remains the provenance authority for the proposed implementation.

## 6. Next-stage gates that remain open

The review supports beginning the plan's bounded investigations. It does not convert unresolved design choices into permissive defaults or authorize unrestricted clock writes.

| Gate | Required evidence before dependent adoption |
| --- | --- |
| S0.1 / OPEN-1 / CHECK-26 | Exact source/artifact/feature/native graph and compatible license grants; candidate names or wrapper licenses are insufficient. |
| S0.2 / OPEN-3 | Actual service tokens, privilege removal, process access and broker/engine separation under selected accounts. |
| S0.3 / OPEN-4 | Race-resistant mutual local identity and per-operation authority, with the chosen named-pipe route or qualified fallback. |
| S0.4 / OPEN-5/7 | Real clock API units/sign/quantization, finite laboratory bounds, slew/crash behavior, competing ownership and one leap policy. |
| S0.7 / OPEN-8 / CHECK-9/27 | Service-account trust and revocation behavior, supported date conversions, normal/provisional certificate policy, actual retrieval ownership and the six conditional HTTP obligations. |
| S0.7 / OPEN-9 | Explicit policy decisions for literal ASVS gaps, including legacy NTP entropy; no blanket L3 claim. |
| S0.5 / OPEN-10 | Native UI accessibility and resource fit using representative real interactions. |
| S0.6 and P6 / OPEN-11/14 | Complete-package tooling/lifecycle qualification, hostile maintenance cases and actual signing-provider acceptance. Isolated test signing supplies only its permitted laboratory subset. |
| P7 | Independent evidence tied to the exact release candidate, with remaining blockers and limitations honestly retained. |

No host clock, Windows service, installer, dependency graph or live NTP endpoint was exercised in this review. These are the future gates already defined by the architecture, not new claimed test results.

## 7. Verification performed and limitations

Document checks verified canonical IDs/levels/chapters/sections, every current record field against its original or exact authorized correction, preserved prior verification text, stable identifier definitions/references, expected counts, complete semantic assignment coverage and frozen input hashes. A GFM Markdown parser checked tables and internal navigation. Local file links and Markdown fences were checked after the final report existed. All five Mermaid source blocks match exactly and were read for flow/authority semantics.

**Native Mermaid rendering was not executed.** No renderer was installed. Diagram source parity and semantic review do not establish rendered geometry or compatibility with every Mermaid version.

The team read the complete source register and used focused primary-source checks for disputed claims. It did not independently retrieve and audit all 93 external sources, resolve every crate graph, execute candidate libraries or obtain legal clearance. Where Windows or provider behavior depends on actual configuration, identity, artifact or runtime, it remains a named qualification question.

The appendices below retain exact review identities, recheck conclusions, requirements traceability and a per-clause coverage ledger. The coverage ledger records review reasoning; it is not an implementation assessment. The canonical current applicability and implementation/evidence status for every clause remain in the worksheet.

## Documentation attribution

The clause coverage and applicability analysis derive from OWASP ASVS 5.0.0 at commit `5cf9b032440be53ce345ab3c130fda46ba1ce7a2`, by the OWASP ASVS project and contributors. These adapted documentation portions retain [Creative Commons Attribution-ShareAlike 4.0 International](https://creativecommons.org/licenses/by-sa/4.0/). They are not OWASP endorsement and do not determine the future software license. The report's factual dependency lead is tied to its exact source commit, not an assertion about all releases.

## Appendix A. Exact reviewed identities and correction rechecks

The first pass reviewed these revision 1.3 bytes. The historical files were retained unchanged during review.

| Baseline document | SHA-256 |
| --- | --- |
| windows-time-architecture.md | `2ea4e288de2774030e5a524d9cb1358fe5ecadffe7f0a5fe567136f48ea8789f` |
| windows-time-diagrams.md | `91d04c60e0c0f826c4a03a983c58d908706b4812388d0306aaf301cee161415e` |
| asvs-5.0.0-applicability.md | `e6de1427953b64a45eb0c25822baeda77ff44fce62eadb6e2cebb1061ad26385` |

The targeted rechecks assessed the following revision 1.4 candidate. These are the exact final three design-document bytes; final integration adds this report and does not alter those reviewed files.

| Final design document | SHA-256 |
| --- | --- |
| windows-time-architecture.md | `a9bd1432ecb2d04e55b77e1a270eb1a135027d9f4a7988d58d9299367a6ca075` |
| windows-time-diagrams.md | `6a2c15dacf6dfd5c378804bf8280ca058a1b965a3604642daeb3aa37b0da4cff` |
| asvs-5.0.0-applicability.md | `efdd173840c68fd484e00ce5c89ced73c1c7dcd5e5422af979f0734be10e529a` |

| Recheck | Result | Evidence boundary |
| --- | --- | --- |
| Architecture | ARC-1 resolved; V13.4.4/V14.2.1 narrowed rationale accepted; no unresolved or new architecture correction | Exact candidate hashes, 16 overrides, 329 unchanged records and five unchanged diagram bodies checked. Pinned date arithmetic independently rechecked. Native behavior remains unexecuted. |
| Security | SEC-1/2/3 corrections accepted; SEC-4 accepted as an unresolved admission fixture | Both normal/provisional date routes, no silent clamp/wrap, actual entropy and transport exception ownership verified in prose. No provider execution or exception approval. |
| Traceability and QA | DOC-1/DOC-2 resolved as document findings; counts and all changed records checked | Exact canonical metadata, 109/28/181/27 dispositions, 39 questions, original/prior verification and diagram parity confirmed. Final report links were an expected integration dependency, checked after report creation. |

The main design retains 15 requirements, 19 decisions, 8 boundaries, 26 threats, 27 acceptance checks, 14 open-decision records and 93 source entries. Final GFM table/navigation, identifier, field, relative-link and frozen-input checks are document validation, not implementation evidence. No native Mermaid renderer or product tests were run.

## Appendix B. Requirement-to-evidence review map

This independent map covers every REQ-1–REQ-15 in the baseline. It remains consistent with revision 1.4's stable identifiers. It is a review of planned evidence, not a report that the checks ran.

| Requirement | Decisions | Boundaries | Threats | Checks | Stage | Owner | Failure and valid neighbor |
| --- | --- | --- | --- | --- | --- | --- | --- |
| REQ-1 | DEC-2/3/4/16 | BND-3/4/5/7 | THR-5/6/7/22 | CHECK-1/2/3/12 | S0.2/S0.3; P3/P4 | Platform/security | Engine/UI clock calls and unauthorized IPC fail; genuine permitted broker operations work; UI close leaves services running. |
| REQ-2 | DEC-14/16/18/19 | BND-1–8 | THR-1–26 | CHECK-4/16/17/24/25/26/27 | P0; P7 exact candidate | Security/independent reviewer | Unsupported N/A and native-equivalence claims are rejected; justified scope decisions and native controls remain traceable. DOC-1/2 correct current scope bookkeeping. |
| REQ-3 | DEC-9/11/12 | BND-5/7 | THR-12/13/16/23 | CHECK-5/12/13 | S0.4/S0.6; P4/P6 | Installer/platform | Conflict or ambiguous successor causes no new reset; legitimate handover/restoration preserves the actual prior W32Time state. |
| REQ-4 | DEC-9/13/16 | BND-6/8 | THR-9/14/15/20 | CHECK-16/18/23 | P0 feasibility; P6/P7 publisher qualification | Security/build/release | Wrong publisher, tampered artifacts and canary DLL loading fail; correct dependencies/package and accessibility still work. Laboratory test signing is not public signing. |
| REQ-5 | DEC-15/16/18/19 | BND-1/3/5/8 | THR-1/5/15/24/25/26 | CHECK-19/20/24/25/26 | P1; P7 | Tooling/security/release | Invalid lint/origin/feature fixtures fail the real CI entry point; valid code and the approved graph pass with nonempty discovery and honest coverage/mutation accounting. |
| REQ-6 | DEC-2/3/10/19 | BND-1/2/4 | THR-8/26 | CHECK-21/27 | P0 measurements; P4/P5/P7 | Performance/platform/engine | Flood/stuck calls cannot exceed admitted work/queue/bytes or starve cleanup; fresh legitimate work succeeds after bounded recovery; numerical budgets remain unmeasured. |
| REQ-7 | DEC-3/17 | BND-4 | THR-18 | CHECK-22 | S0.5; P5/P7 | UI/accessibility | Keyboard, focus return, real Narrator, contrast/scaling and meaningful stale/error status are required; cancellation preserves policy and critical actions remain reachable. |
| REQ-8 | DEC-1/5/6/7/8/11/12 | BND-1/2/3/7 | THR-1/2/3/4/21/24/26 | CHECK-6/7/8/9/10/11/27 | P1/P2; P4 actuation | Protocol/core/platform/security | Malformed/replayed/unauthenticated or invalid TLS paths cannot become NTS discipline; valid normal/cached/explicitly approved bootstrap paths work, with one/two-helper semantics and residual history preserved. |
| REQ-9 | DEC-2/9/13 | BND-5/6/7/8 | THR-16/20/23 | CHECK-13/18/23 | S0.6; P6 | Installer/packaging/release | Maintenance faults retain a recoverable one-owner state; valid complete install/update/repair/remove gives one product/version/entry with no separate service prerequisite. |
| REQ-10 | DEC-1/8 | BND-1/2/3 | THR-2/24 | CHECK-7/8 | P1/P2 | Numerical core/security | No majority or insufficient logical identities causes no new disputed phase correction; independent coherent scenarios choose expected principal/supporters without duplicate votes. |
| REQ-11 | DEC-6/11/12/19 | BND-1/3/6/7 | THR-11/12/13/16/17/19/23/26 | CHECK-10/11/12/13/27 | S0.4; P4 | Platform/core/security | Late/sleep-crossing work is rejected and unknown ownership prevents cleanup writes; fresh acquisition and owned recovery work without replenishing consumed authority. |
| REQ-12 | DEC-3/6/7/11/12 | BND-2/3/4 | THR-2/3/4/18/23 | CHECK-14/22 | P3/P5 | Core/UI/accessibility | Stale/mixed/failure snapshots never fabricate synchronization; coherent valid status separately communicates authentication, agreement, age and uncertainty. |
| REQ-13 | DEC-4/10/18/19 | BND-3/4/5/6/8 | THR-6/7/8/9/10/17/20/22/25/26 | CHECK-2/3/15/17/25/27 | P3/P4/P5 | IPC/platform/security | Invalid mandatory policy/journal blocks effects without permissive defaults; protected valid updates, optional-cache recovery and useful bounded redacted diagnostics remain usable. |
| REQ-14 | DEC-1/13/15/19 | BND-8 | THR-15 | CHECK-20/24/26 | S0.1; P1/P6/P7 | Product/license/release/security | Unresolved incompatible/native/source graphs are held; authoritatively admitted compatible grants, exact origins and port provenance form the valid route without silently relicensing chrony. |
| REQ-15 | DEC-11/14/15 | BND-1–8 | THR-1–26 | CHECK-24 and all specific checks | Each phase; P7 | Independent reviewer and each evidence owner | Stale/mock-only/empty/failed/substituted evidence cannot become a pass; exact-candidate reproducible evidence with independent expectations supports only its declared claim. |

## Appendix C. Complete clause-review ledger

Every canonical requirement has exactly one primary semantic reviewer. Each reviewer read the complete clause and its current worksheet record/additions; review notes below are condensed conclusions, not substitutes for the canonical text. Changed rows show the explicit integrated correction; other notes retain the primary review reason. Every row remains **Not implemented / Not executed** and draft for owner review. Native adaptations and proposed exclusions are not conformity passes.


### V1 — Encoding and Sanitization

| Canonical ID | Minimum level | Primary reviewer | Current draft applicability | Review reason |
| --- | --- | --- | --- | --- |
| `v5.0.0-1.1.1` | L2 | traceability | Directly applicable (proposed) | The once-only canonical decoding clause applies to JSON, IPC, packet and persisted representations. Section 6 preserves decoded values through construction and effect; CHECK-25 supplies both invalid and valid neighbors. |
| `v5.0.0-1.1.2` | L2 | traceability | Directly applicable (proposed) | Final-context escaping is a generic representation obligation. Actual serializers/native sinks and hostile delimiter/control strings are included; no DOM control is claimed. |
| `v5.0.0-1.2.1` | L1 | traceability | Proposed N/A | The declared product creates no HTTP responses or untrusted HTML/XML document rendering; native text/JSON have their own controls. Trusted installer/event manifests are build inputs, not an untrusted runtime document generator. |
| `v5.0.0-1.2.2` | L1 | traceability | Proposed N/A | No dynamic URL construction is selected. Host/port and certificate-supplied retrieval destinations still need endpoint/SSRF controls; accepting a URL is not automatically dynamic URL component construction. |
| `v5.0.0-1.2.3` | L1 | traceability | Directly applicable (proposed) | Corrected in revision 1.4: DEC-18 already selects JSON; the representation decision is answered. Independent targeted recheck accepted the document correction; underlying admission/policy evidence remains pending where applicable. |
| `v5.0.0-1.2.4` | L1 | traceability | Proposed N/A | No database query engine, ORM or stored-procedure path is in the baseline; typed files are not database expressions. |
| `v5.0.0-1.2.5` | L1 | traceability | Directly applicable (proposed) | Structured Windows APIs and tightly scoped helper/installer invocation address command injection; process/path arguments remain adversarial where invocation survives. |
| `v5.0.0-1.2.6` | L2 | traceability | Proposed N/A | No application LDAP queries are planned. Windows domain/token identity alone does not create an LDAP query-construction feature. |
| `v5.0.0-1.2.7` | L2 | traceability | Proposed N/A | No XPath parser/query engine or untrusted XPath expression is planned. |
| `v5.0.0-1.2.8` | L2 | traceability | Proposed N/A | No LaTeX processing is planned; document research does not create an installed LaTeX processor. |
| `v5.0.0-1.2.9` | L2 | traceability | Proposed N/A | No dynamically constructed regular expression is planned; new interpolation would reopen applicability. |
| `v5.0.0-1.2.10` | L3 | traceability | Proposed N/A | No product CSV/spreadsheet export is selected. The former research worksheet is not a product export feature; future diagnostic format selection must preserve this exclusion or reopen it. |
| `v5.0.0-1.3.1` | L1 | traceability | Proposed N/A | No WYSIWYG or untrusted rich HTML is accepted by the native UI. |
| `v5.0.0-1.3.2` | L1 | traceability | Directly applicable (proposed) | Avoidance of dynamic execution applies directly: no scripts/plugins/executable configuration; actual execution/loading inventory and inert hostile strings are planned. |
| `v5.0.0-1.3.3` | L2 | traceability | Directly applicable (proposed) | Dangerous native sinks require bounded domain admission and no effect on rejection; rejecting unsafe values is preferable to silently changing correction meaning. |
| `v5.0.0-1.3.4` | L2 | traceability | Proposed N/A | No user-supplied SVG ingestion exists in the declared scope; bundled release assets are handled as supply-chain inputs. |
| `v5.0.0-1.3.5` | L2 | traceability | Proposed N/A | No user-controlled scriptable markup or expression templates are selected. |
| `v5.0.0-1.3.6` | L2 | traceability | Directly applicable (proposed) | Configured and negotiated NTS destinations are outbound requests; explicit address/port/protocol admission and valid approved private-network controls fit the generic SSRF clause. OPEN-8 also owns conditional native retrieval. |
| `v5.0.0-1.3.7` | L2 | traceability | Proposed N/A | Compiled/bundled UI resources do not introduce runtime templates constructed from untrusted content. |
| `v5.0.0-1.3.8` | L2 | traceability | Proposed N/A | No Java/JNDI runtime or lookup path is planned. |
| `v5.0.0-1.3.9` | L2 | traceability | Proposed N/A | No memcache client or protocol is planned. |
| `v5.0.0-1.3.10` | L2 | traceability | Directly applicable (proposed) | Static format strings and explicit structured fields keep hostile values as data. Errors and dependency instrumentation are separately bounded/redacted. |
| `v5.0.0-1.3.11` | L2 | traceability | Proposed N/A | No SMTP/IMAP command path is planned. |
| `v5.0.0-1.3.12` | L3 | traceability | Proposed N/A | No regex engine is required by the selected baseline; dependency feature review must reopen this row if an input-facing pattern engine is admitted. This is a proposed absence, not a final graph assertion. |
| `v5.0.0-1.4.1` | L2 | traceability | Directly applicable (proposed) | Rust memory safety plus narrow audited FFI/string/buffer adapters addresses the generic memory-safety obligation; native lifetime/boundary qualification remains planned. |
| `v5.0.0-1.4.2` | L2 | traceability | Directly applicable (proposed) | Checked exact integer units, signs, era conversion and post-conversion bounds apply directly; independent numerical neighbors and real backend sign checks are specified. |
| `v5.0.0-1.4.3` | L2 | traceability | Directly applicable (proposed) | RAII/correct destructor families, callback lifetimes and cleanup before teardown cover the resource obligation. Non-preemptible jobs retain permits until return or controlled process recovery; cancellation is not claimed as release proof. |
| `v5.0.0-1.5.1` | L1 | traceability | Proposed N/A | No runtime untrusted XML parser is selected; trusted MSI build XML is not the excluded application XML boundary. |
| `v5.0.0-1.5.2` | L2 | traceability | Directly applicable (proposed) | Strict inert DTOs and known records avoid polymorphic deserialization; actual bounded raw-byte decoder and no-effect fixtures qualify the selected mechanism later. |
| `v5.0.0-1.5.3` | L3 | traceability | Directly applicable (proposed) | Shared encoding/field semantics and parser-differential cases cover representation consistency without erasing duplicate/ambiguous raw input. |

### V2 — Validation and Business Logic

| Canonical ID | Minimum level | Primary reviewer | Current draft applicability | Review reason |
| --- | --- | --- | --- | --- |
| `v5.0.0-2.1.1` | L1 | architecture | Directly applicable (proposed) | Structure/presence/version/size documentation maps directly; section6 and S1.1 freeze each input contract before effects. |
| `v5.0.0-2.1.2` | L2 | architecture | Directly applicable (proposed) | Related source/profile/poll/generation values have named cross-field rules and consumer review; not merely individual scalar validation. |
| `v5.0.0-2.1.3` | L2 | architecture | Directly applicable (proposed) | Per-caller and aggregate limits explicitly include actual accumulated corrections; restart/relabel/sign-change counterexamples retained. |
| `v5.0.0-2.2.1` | L1 | architecture | Directly applicable (proposed) | All-input scope at L2/L3 is preserved across network, IPC, files, constructors and installer inputs; not limited to L1 decision fields. |
| `v5.0.0-2.2.2` | L1 | architecture | Directly applicable (proposed) | Real receiving service decodes and constructs independently of UI; valid schema remains separate from operation rights. |
| `v5.0.0-2.2.3` | L2 | architecture | Directly applicable (proposed) | Invalid combinations of valid fields and unit/era/aggregate conversions have whole-request no-effect obligations. |
| `v5.0.0-2.3.1` | L1 | architecture | Directly applicable (proposed) | Ordered acquire/select/authorize/apply and versioned installer transitions reject skipped stages and stale generations. |
| `v5.0.0-2.3.2` | L2 | architecture | Directly applicable (proposed) | Broker limits cover sequence-wide applied authority, not request claims; stale native completions recheck current state. |
| `v5.0.0-2.3.3` | L2 | architecture | Directly applicable (proposed) | Atomic policy commits are direct; externally visible clock effects retain an explicit uncertain-acknowledgement/commit question rather than fictitious rollback. |
| `v5.0.0-2.3.4` | L2 | architecture | Native adaptation (proposed) | Exclusive native clock/configuration ownership is identified as an adaptation of limited-resource locking, not literal ticket-booking equivalence. |
| `v5.0.0-2.3.5` | L3 | architecture | Needs decision | Dual-person approval is explicitly unresolved for exceptional steps and bootstrap; one UAC consent is not misrepresented as two users. |
| `v5.0.0-2.4.1` | L2 | architecture | Directly applicable (proposed) | Per-caller/global work and native-job permits protect costly paths before submission; valid neighboring traffic and recovery remain required. |
| `v5.0.0-2.4.2` | L3 | architecture | Needs decision | Machine-paced synchronization and assistive-technology use make literal human-timing scope an explicit decision, not a blanket rate-limit pass. |

### V3 — Web Frontend Security

| Canonical ID | Minimum level | Primary reviewer | Current draft applicability | Review reason |
| --- | --- | --- | --- | --- |
| `v5.0.0-3.1.1` | L3 | traceability | Proposed N/A | Read the complete clause, including any embedded stronger L3 condition. Document required browser security capabilities and fallback. belongs to the absent browser/HTTP frontend surface under DEC-10 and section 1.2; native IPC, NTS cookies and endpoint negotiation are not that mechanism. Inventory confirmation and reopening on scope change remain required. |
| `v5.0.0-3.2.1` | L1 | traceability | Proposed N/A | Read the complete clause, including any embedded stronger L3 condition. Prevent browser rendering in an unintended response context. belongs to the absent browser/HTTP frontend surface under DEC-10 and section 1.2; native IPC, NTS cookies and endpoint negotiation are not that mechanism. Inventory confirmation and reopening on scope change remain required. |
| `v5.0.0-3.2.2` | L1 | traceability | Native adaptation (proposed) | The literal clause is about safe browser text rendering. A native text/control analogue is explicitly labelled an adaptation; hostile markup stays inert, with semantic accessibility and valid localized controls, without claiming DOM equivalence. |
| `v5.0.0-3.2.3` | L3 | traceability | Proposed N/A | Read the complete clause, including any embedded stronger L3 condition. Avoid JavaScript DOM clobbering and ambient globals. belongs to the absent browser/HTTP frontend surface under DEC-10 and section 1.2; native IPC, NTS cookies and endpoint negotiation are not that mechanism. Inventory confirmation and reopening on scope change remain required. |
| `v5.0.0-3.3.1` | L1 | traceability | Proposed N/A | Read the complete clause, including any embedded stronger L3 condition. Set secure cookie attributes and required name prefixes. belongs to the absent browser/HTTP frontend surface under DEC-10 and section 1.2; native IPC, NTS cookies and endpoint negotiation are not that mechanism. Inventory confirmation and reopening on scope change remain required. |
| `v5.0.0-3.3.2` | L2 | traceability | Proposed N/A | Read the complete clause, including any embedded stronger L3 condition. Choose cookie SameSite policy for its purpose. belongs to the absent browser/HTTP frontend surface under DEC-10 and section 1.2; native IPC, NTS cookies and endpoint negotiation are not that mechanism. Inventory confirmation and reopening on scope change remain required. |
| `v5.0.0-3.3.3` | L2 | traceability | Proposed N/A | Read the complete clause, including any embedded stronger L3 condition. Use host-bound cookie names unless sharing is intended. belongs to the absent browser/HTTP frontend surface under DEC-10 and section 1.2; native IPC, NTS cookies and endpoint negotiation are not that mechanism. Inventory confirmation and reopening on scope change remain required. |
| `v5.0.0-3.3.4` | L2 | traceability | Proposed N/A | Read the complete clause, including any embedded stronger L3 condition. Keep script-inaccessible cookies out of script-visible channels. belongs to the absent browser/HTTP frontend surface under DEC-10 and section 1.2; native IPC, NTS cookies and endpoint negotiation are not that mechanism. Inventory confirmation and reopening on scope change remain required. |
| `v5.0.0-3.3.5` | L3 | traceability | Proposed N/A | Read the complete clause, including any embedded stronger L3 condition. Keep browser cookie names and values within supported size. belongs to the absent browser/HTTP frontend surface under DEC-10 and section 1.2; native IPC, NTS cookies and endpoint negotiation are not that mechanism. Inventory confirmation and reopening on scope change remain required. |
| `v5.0.0-3.4.1` | L1 | traceability | Proposed N/A | Read the complete clause, including any embedded stronger L3 condition. Enforce long-lived HTTPS policy through HSTS headers. belongs to the absent browser/HTTP frontend surface under DEC-10 and section 1.2; native IPC, NTS cookies and endpoint negotiation are not that mechanism. Inventory confirmation and reopening on scope change remain required. |
| `v5.0.0-3.4.2` | L1 | traceability | Proposed N/A | Read the complete clause, including any embedded stronger L3 condition. Restrict cross-origin HTTP access to intended origins. belongs to the absent browser/HTTP frontend surface under DEC-10 and section 1.2; native IPC, NTS cookies and endpoint negotiation are not that mechanism. Inventory confirmation and reopening on scope change remain required. |
| `v5.0.0-3.4.3` | L2 | traceability | Proposed N/A | Read the complete clause, including any embedded stronger L3 condition. Use a restrictive CSP with stronger per-response L3 controls. belongs to the absent browser/HTTP frontend surface under DEC-10 and section 1.2; native IPC, NTS cookies and endpoint negotiation are not that mechanism. Inventory confirmation and reopening on scope change remain required. |
| `v5.0.0-3.4.4` | L2 | traceability | Proposed N/A | Read the complete clause, including any embedded stronger L3 condition. Disable MIME sniffing on HTTP responses. belongs to the absent browser/HTTP frontend surface under DEC-10 and section 1.2; native IPC, NTS cookies and endpoint negotiation are not that mechanism. Inventory confirmation and reopening on scope change remain required. |
| `v5.0.0-3.4.5` | L2 | traceability | Proposed N/A | Read the complete clause, including any embedded stronger L3 condition. Prevent sensitive referrer information from leaving the application. belongs to the absent browser/HTTP frontend surface under DEC-10 and section 1.2; native IPC, NTS cookies and endpoint negotiation are not that mechanism. Inventory confirmation and reopening on scope change remain required. |
| `v5.0.0-3.4.6` | L2 | traceability | Proposed N/A | Read the complete clause, including any embedded stronger L3 condition. Restrict browser framing through policy. belongs to the absent browser/HTTP frontend surface under DEC-10 and section 1.2; native IPC, NTS cookies and endpoint negotiation are not that mechanism. Inventory confirmation and reopening on scope change remain required. |
| `v5.0.0-3.4.7` | L3 | traceability | Proposed N/A | Read the complete clause, including any embedded stronger L3 condition. Configure collection of CSP violations. belongs to the absent browser/HTTP frontend surface under DEC-10 and section 1.2; native IPC, NTS cookies and endpoint negotiation are not that mechanism. Inventory confirmation and reopening on scope change remain required. |
| `v5.0.0-3.4.8` | L3 | traceability | Proposed N/A | Read the complete clause, including any embedded stronger L3 condition. Isolate top-level browser contexts with opener policy. belongs to the absent browser/HTTP frontend surface under DEC-10 and section 1.2; native IPC, NTS cookies and endpoint negotiation are not that mechanism. Inventory confirmation and reopening on scope change remain required. |
| `v5.0.0-3.5.1` | L1 | traceability | Proposed N/A | Read the complete clause, including any embedded stronger L3 condition. Defend sensitive browser actions against request forgery. belongs to the absent browser/HTTP frontend surface under DEC-10 and section 1.2; native IPC, NTS cookies and endpoint negotiation are not that mechanism. Inventory confirmation and reopening on scope change remain required. |
| `v5.0.0-3.5.2` | L1 | traceability | Proposed N/A | Read the complete clause, including any embedded stronger L3 condition. Ensure preflight-dependent controls cannot be bypassed. belongs to the absent browser/HTTP frontend surface under DEC-10 and section 1.2; native IPC, NTS cookies and endpoint negotiation are not that mechanism. Inventory confirmation and reopening on scope change remain required. |
| `v5.0.0-3.5.3` | L1 | traceability | Proposed N/A | Corrected in revision 1.4: Narrow the exclusion to the actual server/sensitive-action surface; avoid a blanket assertion that every HTTP workflow is absent. Independent targeted recheck accepted the document correction; underlying admission/policy evidence remains pending where applicable. |
| `v5.0.0-3.5.4` | L2 | traceability | Proposed N/A | Read the complete clause, including any embedded stronger L3 condition. Separate applications by browser origin. belongs to the absent browser/HTTP frontend surface under DEC-10 and section 1.2; native IPC, NTS cookies and endpoint negotiation are not that mechanism. Inventory confirmation and reopening on scope change remain required. |
| `v5.0.0-3.5.5` | L2 | traceability | Proposed N/A | Read the complete clause, including any embedded stronger L3 condition. Validate postMessage sender origin and message syntax. belongs to the absent browser/HTTP frontend surface under DEC-10 and section 1.2; native IPC, NTS cookies and endpoint negotiation are not that mechanism. Inventory confirmation and reopening on scope change remain required. |
| `v5.0.0-3.5.6` | L3 | traceability | Proposed N/A | Read the complete clause, including any embedded stronger L3 condition. Disable JSONP script inclusion behavior. belongs to the absent browser/HTTP frontend surface under DEC-10 and section 1.2; native IPC, NTS cookies and endpoint negotiation are not that mechanism. Inventory confirmation and reopening on scope change remain required. |
| `v5.0.0-3.5.7` | L3 | traceability | Proposed N/A | Read the complete clause, including any embedded stronger L3 condition. Keep protected data out of script-resource responses. belongs to the absent browser/HTTP frontend surface under DEC-10 and section 1.2; native IPC, NTS cookies and endpoint negotiation are not that mechanism. Inventory confirmation and reopening on scope change remain required. |
| `v5.0.0-3.5.8` | L3 | traceability | Proposed N/A | Read the complete clause, including any embedded stronger L3 condition. Restrict unintended embedding of authenticated resources. belongs to the absent browser/HTTP frontend surface under DEC-10 and section 1.2; native IPC, NTS cookies and endpoint negotiation are not that mechanism. Inventory confirmation and reopening on scope change remain required. |
| `v5.0.0-3.6.1` | L3 | traceability | Proposed N/A | Read the complete clause, including any embedded stronger L3 condition. Version and integrity-check externally hosted browser assets. belongs to the absent browser/HTTP frontend surface under DEC-10 and section 1.2; native IPC, NTS cookies and endpoint negotiation are not that mechanism. Inventory confirmation and reopening on scope change remain required. |
| `v5.0.0-3.7.1` | L2 | traceability | Proposed N/A | Read the complete clause, including any embedded stronger L3 condition. Use supported secure browser technologies. belongs to the absent browser/HTTP frontend surface under DEC-10 and section 1.2; native IPC, NTS cookies and endpoint negotiation are not that mechanism. Inventory confirmation and reopening on scope change remain required. |
| `v5.0.0-3.7.2` | L2 | traceability | Proposed N/A | Read the complete clause, including any embedded stronger L3 condition. Allowlist automatic external-domain redirects. belongs to the absent browser/HTTP frontend surface under DEC-10 and section 1.2; native IPC, NTS cookies and endpoint negotiation are not that mechanism. Inventory confirmation and reopening on scope change remain required. |
| `v5.0.0-3.7.3` | L3 | traceability | Proposed N/A | Read the complete clause, including any embedded stronger L3 condition. Warn and allow cancellation before external navigation. belongs to the absent browser/HTTP frontend surface under DEC-10 and section 1.2; native IPC, NTS cookies and endpoint negotiation are not that mechanism. Inventory confirmation and reopening on scope change remain required. |
| `v5.0.0-3.7.4` | L3 | traceability | Proposed N/A | Read the complete clause, including any embedded stronger L3 condition. Preload the web domain for HSTS. belongs to the absent browser/HTTP frontend surface under DEC-10 and section 1.2; native IPC, NTS cookies and endpoint negotiation are not that mechanism. Inventory confirmation and reopening on scope change remain required. |
| `v5.0.0-3.7.5` | L3 | traceability | Proposed N/A | Read the complete clause, including any embedded stronger L3 condition. Apply documented fallback when browser safeguards are missing. belongs to the absent browser/HTTP frontend surface under DEC-10 and section 1.2; native IPC, NTS cookies and endpoint negotiation are not that mechanism. Inventory confirmation and reopening on scope change remain required. |

### V4 — API and Web Service

| Canonical ID | Minimum level | Primary reviewer | Current draft applicability | Review reason |
| --- | --- | --- | --- | --- |
| `v5.0.0-4.1.1` | L1 | traceability | Proposed N/A | Corrected in revision 1.4: Narrow the exclusion to the actual server/sensitive-action surface; avoid a blanket assertion that every HTTP workflow is absent. Independent targeted recheck accepted the document correction; underlying admission/policy evidence remains pending where applicable. |
| `v5.0.0-4.1.2` | L2 | traceability | Proposed N/A | Corrected in revision 1.4: Narrow the exclusion to the actual server/sensitive-action surface; avoid a blanket assertion that every HTTP workflow is absent. Independent targeted recheck accepted the document correction; underlying admission/policy evidence remains pending where applicable. |
| `v5.0.0-4.1.3` | L2 | traceability | Proposed N/A | No trusted intermediary HTTP identity header feeds authorization. Windows caller tokens are verified separately and are not treated as header equivalence. |
| `v5.0.0-4.1.4` | L3 | traceability | Proposed N/A | Corrected in revision 1.4: Narrow the exclusion to the actual server/sensitive-action surface; avoid a blanket assertion that every HTTP workflow is absent. Independent targeted recheck accepted the document correction; underlying admission/policy evidence remains pending where applicable. |
| `v5.0.0-4.1.5` | L3 | traceability | Needs decision | The requirement is not limited to a browser: sensitive local clock/policy messages need an explicit signature-versus-native-channel decision. The row correctly retains Needs decision and does not equate AEAD/ACLs to digital signatures. |
| `v5.0.0-4.2.1` | L2 | traceability | Needs decision | Corrected in revision 1.4: Reconcile a current held direct/delegated certificate-retrieval candidate with the exact clause; no runtime failure is asserted. Independent targeted recheck accepted the document correction; underlying admission/policy evidence remains pending where applicable. |
| `v5.0.0-4.2.2` | L3 | traceability | Needs decision | Corrected in revision 1.4: Reconcile a current held direct/delegated certificate-retrieval candidate with the exact clause; no runtime failure is asserted. Independent targeted recheck accepted the document correction; underlying admission/policy evidence remains pending where applicable. |
| `v5.0.0-4.2.3` | L3 | traceability | Needs decision | Corrected in revision 1.4: Reconcile a current held direct/delegated certificate-retrieval candidate with the exact clause; no runtime failure is asserted. Independent targeted recheck accepted the document correction; underlying admission/policy evidence remains pending where applicable. |
| `v5.0.0-4.2.4` | L3 | traceability | Proposed N/A | Corrected in revision 1.4: Narrow the exclusion to the actual server/sensitive-action surface; avoid a blanket assertion that every HTTP workflow is absent. Independent targeted recheck accepted the document correction; underlying admission/policy evidence remains pending where applicable. |
| `v5.0.0-4.2.5` | L3 | traceability | Needs decision | Corrected in revision 1.4: Reconcile a current held direct/delegated certificate-retrieval candidate with the exact clause; no runtime failure is asserted. Independent targeted recheck accepted the document correction; underlying admission/policy evidence remains pending where applicable. |
| `v5.0.0-4.3.1` | L2 | traceability | Proposed N/A | No GraphQL or data-layer expression execution is selected; native bounded records are not a query language. |
| `v5.0.0-4.3.2` | L2 | traceability | Proposed N/A | No GraphQL endpoint or introspection exists in the declared design. |
| `v5.0.0-4.4.1` | L1 | traceability | Proposed N/A | No WebSocket connection exists; NTS-KE over TLS is a different protocol. |
| `v5.0.0-4.4.2` | L2 | traceability | Proposed N/A | No HTTP WebSocket handshake or browser Origin trust decision exists. |
| `v5.0.0-4.4.3` | L2 | traceability | Proposed N/A | The section concerns WebSocket session tokens. No such tokens exist; Windows IPC/session obligations are retained under V6–V8. |
| `v5.0.0-4.4.4` | L2 | traceability | Proposed N/A | No HTTPS-to-WebSocket authorization transition exists. |

### V5 — File Handling

| Canonical ID | Minimum level | Primary reviewer | Current draft applicability | Review reason |
| --- | --- | --- | --- | --- |
| `v5.0.0-5.1.1` | L2 | architecture | Native adaptation (proposed) | Local configuration/state formats and output bounds are honestly marked a native adaptation of upload/download documentation. |
| `v5.0.0-5.2.1` | L1 | architecture | Directly applicable (proposed) | Every accepted local file reader is bounded before allocation; growth/corruption preserves safe state. |
| `v5.0.0-5.2.2` | L1 | architecture | Directly applicable (proposed) | Stronger L2/L3 all-accepted-files condition is explicit; fixed names still need expected extension/content contract. |
| `v5.0.0-5.2.3` | L2 | architecture | Proposed N/A | No arbitrary archive ingestion in the baseline; signed fixed MSI payload extraction is scoped to installer provenance/lifecycle. Reopen if untrusted archives are accepted. |
| `v5.0.0-5.2.4` | L3 | architecture | Native adaptation (proposed) | Per-principal creation pressure and total application storage are adapted, with no invented quota over arbitrary user disk files. |
| `v5.0.0-5.2.5` | L3 | architecture | Proposed N/A | No uploaded archive or archive symlink feature; reparse/hardlink attacks on owned paths remain covered separately. |
| `v5.0.0-5.2.6` | L3 | architecture | Proposed N/A | No untrusted image decoder/upload; packaged static resources remain release inputs. |
| `v5.0.0-5.3.1` | L1 | architecture | Proposed N/A | No HTTP public-file execution surface; absence must be checked in final dependency/listener inventory. |
| `v5.0.0-5.3.2` | L1 | architecture | Directly applicable (proposed) | Fixed trusted roots, stable handles and reparse/UNC/stream constraints cover privileged paths; UI export uses user authority. |
| `v5.0.0-5.3.3` | L3 | architecture | Proposed N/A | No untrusted archive/file-processing server or propagated submitted archive path; ordinary local path control remains applicable under5.3.2. |
| `v5.0.0-5.4.1` | L2 | architecture | Proposed N/A | No served download Content-Disposition response; native Save dialog is not relabelled an HTTP endpoint. |
| `v5.0.0-5.4.2` | L2 | architecture | Proposed N/A | No filename-bearing served/email message workflow; inert local text display is separately bounded. |
| `v5.0.0-5.4.3` | L2 | architecture | Proposed N/A | No untrusted-file redistribution; build/release assurance is a separate trusted-input boundary, not an antivirus equivalence claim. |

### V6 — Authentication

| Canonical ID | Minimum level | Primary reviewer | Current draft applicability | Review reason |
| --- | --- | --- | --- | --- |
| `v5.0.0-6.1.1` | L1 | security | Proposed N/A | Retain scoped N/A: there is no application password challenge, credential stuffing target or account lockout. IPC resource abuse is separately covered and does not establish this password control. |
| `v5.0.0-6.1.2` | L2 | security | Proposed N/A | Retain scoped N/A: no application password policy or password creation accepts context-specific words. |
| `v5.0.0-6.1.3` | L2 | security | Native adaptation (proposed) | Corrected in revision 1.4: Align the authentication inventory with the existing broker-mediated status design. Independent targeted recheck accepted the document correction; underlying admission/policy evidence remains pending where applicable. |
| `v5.0.0-6.2.1` | L1 | security | Proposed N/A | Retain scoped N/A for minimum length: the product accepts, stores and changes no application password; Windows owns user credentials, while native identity and authorization remain in scope. |
| `v5.0.0-6.2.2` | L1 | security | Proposed N/A | Retain scoped N/A for password changes: the product accepts, stores and changes no application password; Windows owns user credentials, while native identity and authorization remain in scope. |
| `v5.0.0-6.2.3` | L1 | security | Proposed N/A | Retain scoped N/A for current/new password confirmation: the product accepts, stores and changes no application password; Windows owns user credentials, while native identity and authorization remain in scope. |
| `v5.0.0-6.2.4` | L1 | security | Proposed N/A | Retain scoped N/A for common-password rejection: the product accepts, stores and changes no application password; Windows owns user credentials, while native identity and authorization remain in scope. |
| `v5.0.0-6.2.5` | L1 | security | Proposed N/A | Retain scoped N/A for composition freedom: the product accepts, stores and changes no application password; Windows owns user credentials, while native identity and authorization remain in scope. |
| `v5.0.0-6.2.6` | L1 | security | Proposed N/A | Retain scoped N/A for masked password input: the product accepts, stores and changes no application password; Windows owns user credentials, while native identity and authorization remain in scope. |
| `v5.0.0-6.2.7` | L1 | security | Proposed N/A | Retain scoped N/A for paste/password-manager support: the product accepts, stores and changes no application password; Windows owns user credentials, while native identity and authorization remain in scope. |
| `v5.0.0-6.2.8` | L1 | security | Proposed N/A | Retain scoped N/A for exact password comparison: the product accepts, stores and changes no application password; Windows owns user credentials, while native identity and authorization remain in scope. |
| `v5.0.0-6.2.9` | L2 | security | Proposed N/A | Retain scoped N/A for long-password acceptance: the product accepts, stores and changes no application password; Windows owns user credentials, while native identity and authorization remain in scope. |
| `v5.0.0-6.2.10` | L2 | security | Proposed N/A | Retain scoped N/A for nonperiodic password rotation: the product accepts, stores and changes no application password; Windows owns user credentials, while native identity and authorization remain in scope. |
| `v5.0.0-6.2.11` | L2 | security | Proposed N/A | Retain scoped N/A for context-word rejection: the product accepts, stores and changes no application password; Windows owns user credentials, while native identity and authorization remain in scope. |
| `v5.0.0-6.2.12` | L2 | security | Proposed N/A | Retain scoped N/A for breached-password rejection: the product accepts, stores and changes no application password; Windows owns user credentials, while native identity and authorization remain in scope. |
| `v5.0.0-6.3.1` | L1 | security | Proposed N/A | Retain scoped N/A for password-guessing controls; bounded unauthenticated IPC and network work remain independently required. |
| `v5.0.0-6.3.2` | L1 | security | Directly applicable (proposed) | Retain direct applicability: default/shared application accounts must remain absent; explicitly scoped service identities are not permission to embed reusable credentials. |
| `v5.0.0-6.3.3` | L2 | security | Needs decision | Retain Needs decision: Windows token possession/UAC does not prove the L3 hardware, phishing resistance and user-intent conditions. The clause permits only a documented relaxation with comprehensive mitigations. |
| `v5.0.0-6.3.4` | L2 | security | Native adaptation (proposed) | Retain native adaptation: all UI/helper/engine/broker paths need consistent authority checks; a stronger path cannot elevate a weaker path. |
| `v5.0.0-6.3.5` | L3 | security | Native adaptation (proposed) | Retain explicit native adaptation of suspicious authentication notifications to owned IPC/NTS events; do not claim monitoring of all Windows logons. |
| `v5.0.0-6.3.6` | L3 | security | Proposed N/A | Retain scoped N/A: no email authentication factor is implemented; delegated Windows assurance is separately unresolved. |
| `v5.0.0-6.3.7` | L3 | security | Proposed N/A | Retain scoped N/A: the product changes neither user credentials nor account identifiers; policy-change audit events are distinct. |
| `v5.0.0-6.3.8` | L3 | security | Native adaptation (proposed) | Retain native adaptation: bounded generic IPC failures and SID-based authority avoid unnecessary identity disclosure; no app registration/recovery enumeration exists. |
| `v5.0.0-6.4.1` | L1 | security | Proposed N/A | Retain scoped N/A: no generated initial password or activation-code account lifecycle. |
| `v5.0.0-6.4.2` | L1 | security | Proposed N/A | Retain scoped N/A: no password hint or knowledge-based authentication. |
| `v5.0.0-6.4.3` | L2 | security | Proposed N/A | Retain scoped N/A: no product-owned forgotten-password recovery pathway to bypass factors. |
| `v5.0.0-6.4.4` | L2 | security | Proposed N/A | Retain scoped N/A: no product factor enrollment/recovery; Windows assurance cannot be inferred from that absence. |
| `v5.0.0-6.4.5` | L3 | security | Proposed N/A | Retain scoped N/A for human authentication renewal; NTS association/key renewal is a separately specified protocol lifecycle. |
| `v5.0.0-6.4.6` | L3 | security | Proposed N/A | Retain scoped N/A: product administrators do not initiate or choose user passwords. |
| `v5.0.0-6.5.1` | L2 | security | Proposed N/A | Retain scoped N/A: no lookup secret, out-of-band user code or TOTP validation; NTS replay handling is separate. |
| `v5.0.0-6.5.2` | L2 | security | Proposed N/A | Retain scoped N/A: no user lookup-secret store, so its entropy-dependent hashing rule has no owned object. |
| `v5.0.0-6.5.3` | L2 | security | Proposed N/A | Retain scoped N/A for user-factor seeds; generic NTS/IPC randomness remains within V11.5.1. |
| `v5.0.0-6.5.4` | L2 | security | Proposed N/A | Retain scoped N/A: no out-of-band user authentication codes; this 20-bit rule cannot weaken NTS UID requirements. |
| `v5.0.0-6.5.5` | L2 | security | Proposed N/A | Retain scoped N/A: no out-of-band or TOTP code validity window; protocol deadlines are separate controls. |
| `v5.0.0-6.5.6` | L3 | security | Proposed N/A | Retain scoped N/A for product-owned human factors; existing Windows account/grant revocation decisions remain open elsewhere. |
| `v5.0.0-6.5.7` | L3 | security | Proposed N/A | Retain scoped N/A: the product neither enrolls nor validates a biometric factor. |
| `v5.0.0-6.5.8` | L3 | security | Proposed N/A | Retain scoped N/A: a time client is not a TOTP verifier. General time authenticity cannot be relabeled as compliance with a nonexistent login flow. |
| `v5.0.0-6.6.1` | L2 | security | Proposed N/A | Retain scoped N/A: no PSTN/SMS factor. Preserve the L3 prohibition if a human login feature is ever introduced. |
| `v5.0.0-6.6.2` | L2 | security | Proposed N/A | Retain scoped N/A: no out-of-band user authentication transaction. NTS request binding is reviewed under its protocol clauses. |
| `v5.0.0-6.6.3` | L2 | security | Proposed N/A | Retain scoped N/A: no code-based out-of-band authentication brute-force surface. |
| `v5.0.0-6.6.4` | L3 | security | Proposed N/A | Retain scoped N/A: no MFA push notification or push-bombing surface. |
| `v5.0.0-6.7.1` | L3 | security | Native adaptation (proposed) | Retain disclosed native adaptation to protected NTS trust anchors/pins; this is machine/server authentication, not proof of a human cryptographic authenticator. |
| `v5.0.0-6.7.2` | L3 | security | Native adaptation (proposed) | Retain disclosed protocol adaptation; the clause minimum is not permission to reduce NTS UID size below RFC 8915 requirements. |
| `v5.0.0-6.8.1` | L2 | security | Native adaptation (proposed) | Retain native adaptation: full verified Windows SIDs avoid same-display-name collisions across local/domain identities; no app federation is asserted. |
| `v5.0.0-6.8.2` | L2 | security | Proposed N/A | Retain scoped N/A: no app JWT/SAML or analogous signed user assertion parser; kernel token and TLS certificate validation have separate mechanisms. |
| `v5.0.0-6.8.3` | L2 | security | Proposed N/A | Retain scoped N/A: no SAML assertion consumer or federated application session. |
| `v5.0.0-6.8.4` | L2 | security | Native adaptation (proposed) | Retain adaptation and existing question: available Windows strength/recency facts must be demonstrated and missing facts receive conservative assurance, never assumed hardware MFA. |

### V7 — Session Management

| Canonical ID | Minimum level | Primary reviewer | Current draft applicability | Review reason |
| --- | --- | --- | --- | --- |
| `v5.0.0-7.1.1` | L2 | security | Native adaptation (proposed) | Retain adaptation to IPC authority lifetime/inactivity policy; no claim that a pipe reconnect reauthenticates the Windows logon. Risk-based bounds still require owner approval. |
| `v5.0.0-7.1.2` | L2 | security | Native adaptation (proposed) | Retain adaptation to per-principal pipe/concurrency bounds and explicit saturation behavior; do not invent an application account session store. |
| `v5.0.0-7.1.3` | L2 | security | Proposed N/A | Retain scoped N/A: no app-managed federated SSO session ecosystem. |
| `v5.0.0-7.2.1` | L1 | security | Native adaptation (proposed) | Retain native adaptation: the trusted broker verifies the actual peer token rather than accepting client identity assertions. |
| `v5.0.0-7.2.2` | L1 | security | Native adaptation (proposed) | Retain native adaptation: live kernel-authenticated bindings replace static shared credentials; Windows tokens are not app bearer secrets. |
| `v5.0.0-7.2.3` | L1 | security | Proposed N/A | Retain scoped N/A: no application reference-token session identifier. Unpredictable protocol values remain covered under V11.5.1. |
| `v5.0.0-7.2.4` | L1 | security | Native adaptation (proposed) | Retain native adaptation: reconnect establishes a new verified binding and invalidates old process/generation authority; it does not refresh the OS logon automatically. |
| `v5.0.0-7.3.1` | L2 | security | Native adaptation (proposed) | Retain adaptation to idle IPC authority expiration and revalidation, with the Windows-session assurance limitation explicit. |
| `v5.0.0-7.3.2` | L2 | security | Native adaptation (proposed) | Retain adaptation to maximum IPC authority lifetime and bounded revalidation rather than assuming unlimited cached token authority. |
| `v5.0.0-7.4.1` | L1 | security | Native adaptation (proposed) | Retain native adaptation: connection/generation termination prevents stale binding reuse and queued privileged effects. |
| `v5.0.0-7.4.2` | L1 | security | Needs decision | Retain Needs decision: a cached Windows token may survive account disablement; detection latency/disconnected-domain limitations are not solved by repeating a token check. |
| `v5.0.0-7.4.3` | L2 | security | Proposed N/A | Retain scoped N/A: no product factor change/removal or password recovery workflow. |
| `v5.0.0-7.4.4` | L2 | security | Proposed N/A | Retain scoped N/A to authenticated web pages; native IPC termination controls remain separately required. |
| `v5.0.0-7.4.5` | L2 | security | Native adaptation (proposed) | Retain adaptation with existing choice: prove an administrative disconnect/revocation mechanism or show no reusable user privilege persists; an unimplemented session dashboard is not evidence. |
| `v5.0.0-7.5.1` | L2 | security | Proposed N/A | Retain scoped N/A: no app-owned authentication/account-recovery attributes. Sensitive product configuration still needs current authorization. |
| `v5.0.0-7.5.2` | L2 | security | Needs decision | Retain Needs decision: final persistence of user authority determines the session visibility/termination obligation; one-shot elevation has not been proven yet. |
| `v5.0.0-7.5.3` | L3 | security | Native adaptation (proposed) | Retain adaptation and open verification policy: a UAC prompt or already-elevated caller does not by itself prove an additional factor for sensitive actions. |
| `v5.0.0-7.6.1` | L2 | security | Proposed N/A | Retain scoped N/A: no relying-party/IdP session protocol is implemented by this application. |
| `v5.0.0-7.6.2` | L2 | security | Native adaptation (proposed) | Retain explicit native adaptation for intentional UI/helper authority creation; machine service activity is not a silent interactive user login. |

### V8 — Authorization

| Canonical ID | Minimum level | Primary reviewer | Current draft applicability | Review reason |
| --- | --- | --- | --- | --- |
| `v5.0.0-8.1.1` | L1 | security | Directly applicable (proposed) | Retain direct applicability: method/resource rights belong in the broker/engine authority matrix and are checked independently of valid input shape. |
| `v5.0.0-8.1.2` | L2 | security | Directly applicable (proposed) | Retain direct applicability: field read/write rights, including secret omission and protected policy values, are distinct from method access. |
| `v5.0.0-8.1.3` | L3 | security | Directly applicable (proposed) | Retain direct applicability: document actual identity, generation, ownership, policy and freshness inputs; do not rely on attacker-controlled wall time as authority. |
| `v5.0.0-8.1.4` | L3 | security | Directly applicable (proposed) | Retain direct applicability: contextual thresholds and deny/challenge actions need explicit policy; unused speculative attributes need not be invented. |
| `v5.0.0-8.2.1` | L1 | security | Directly applicable (proposed) | Retain direct applicability: trusted handlers enforce allowlisted action rights even when the UI is bypassed. |
| `v5.0.0-8.2.2` | L1 | security | Directly applicable (proposed) | Retain direct applicability: object-specific source/policy/state access cannot be inferred from possession of a syntactically valid identifier. |
| `v5.0.0-8.2.3` | L2 | security | Directly applicable (proposed) | Retain direct applicability: deny unauthorized field writes and withhold secret fields rather than relying on UI hiding. |
| `v5.0.0-8.2.4` | L3 | security | Native adaptation (proposed) | Retain native adaptation and existing contextual-refresh question; current grants/generations apply at connection and effect time, with trustworthy attributes still to qualify. |
| `v5.0.0-8.3.1` | L1 | security | Directly applicable (proposed) | Retain direct applicability: broker/engine service boundaries own authorization; an unelevated UI is not the security enforcement point. |
| `v5.0.0-8.3.2` | L3 | security | Needs decision | Retain Needs decision: application grant revisions can revoke promptly, while OS/domain changes may not. The L3 immediate-change/mitigation text must remain visible. |
| `v5.0.0-8.3.3` | L3 | security | Directly applicable (proposed) | Retain direct applicability: helper/service mediation must preserve original subject permission; service privilege must not become delegated caller authority. |
| `v5.0.0-8.4.1` | L2 | security | Proposed N/A | Retain scoped N/A to multi-tenant application isolation: this is a machine-global clock service. Its cross-user effects still require the explicit administrative authority model. |
| `v5.0.0-8.4.2` | L3 | security | Needs decision | Retain Needs decision: local-only access and Windows identity do not establish continuous posture/risk assessment on unmanaged desktops. |

### V9 — Self-contained Tokens

| Canonical ID | Minimum level | Primary reviewer | Current draft applicability | Review reason |
| --- | --- | --- | --- | --- |
| `v5.0.0-9.1.1` | L1 | security | Proposed N/A | Retain scoped N/A: the client does not validate application self-contained authorization claims. Opaque NTS cookies are returned to their server and do not grant local client authority. |
| `v5.0.0-9.1.2` | L1 | security | Proposed N/A | Retain scoped N/A: no self-contained claim-token algorithm selector; NTS AEAD negotiation is separately allowlisted and verified. |
| `v5.0.0-9.1.3` | L1 | security | Proposed N/A | Retain scoped N/A: no attacker-controlled JWT/JWS signing-key URL or issuer-key claim field is consumed. |
| `v5.0.0-9.2.1` | L1 | security | Proposed N/A | Retain scoped N/A: no app self-contained token validity claims are decoded; NTS association age and trust deadlines are separate. |
| `v5.0.0-9.2.2` | L2 | security | Proposed N/A | Retain scoped N/A: no access-token versus ID-token interpretation in the client. |
| `v5.0.0-9.2.3` | L2 | security | Proposed N/A | Retain scoped N/A: no app token audience claim is accepted; protocol association and endpoint binding remain required. |
| `v5.0.0-9.2.4` | L2 | security | Proposed N/A | Retain scoped N/A: the product is not an issuer signing authorization tokens for multiple audiences. |

### V10 — OAuth and OIDC

| Canonical ID | Minimum level | Primary reviewer | Current draft applicability | Review reason |
| --- | --- | --- | --- | --- |
| `v5.0.0-10.1.1` | L2 | traceability | Proposed N/A | Read the exact Generic OAuth and OIDC Security clause: Expose OAuth tokens only to components that need them. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.1.2` | L2 | traceability | Proposed N/A | Read the exact Generic OAuth and OIDC Security clause: Bind OAuth flow responses and secrets to the initiating transaction. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.2.1` | L2 | traceability | Proposed N/A | Read the exact OAuth Client clause: Protect OAuth code-flow token requests against browser forgery. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.2.2` | L2 | traceability | Proposed N/A | Read the exact OAuth Client clause: Prevent authorization-server mix-up at OAuth clients. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.2.3` | L3 | traceability | Proposed N/A | Read the exact OAuth Client clause: Request only necessary OAuth scopes and authorizations. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.3.1` | L2 | traceability | Proposed N/A | Read the exact OAuth Resource Server clause: Validate access-token audience at the resource server. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.3.2` | L2 | traceability | Proposed N/A | Read the exact OAuth Resource Server clause: Base delegated authorization on the relevant access-token claims. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.3.3` | L2 | traceability | Proposed N/A | Read the exact OAuth Resource Server clause: Identify token users through stable, nonreassigned claims. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.3.4` | L2 | traceability | Proposed N/A | Read the exact OAuth Resource Server clause: Check required authentication strength and recency in access tokens. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.3.5` | L3 | traceability | Proposed N/A | Read the exact OAuth Resource Server clause: Require sender-bound tokens to resist theft and replay. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.4.1` | L1 | traceability | Proposed N/A | Read the exact OAuth Authorization Server clause: Match redirect URIs exactly to each client's registered allowlist. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.4.2` | L1 | traceability | Proposed N/A | Read the exact OAuth Authorization Server clause: Make authorization codes single-use and revoke on valid reuse. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.4.3` | L1 | traceability | Proposed N/A | Read the exact OAuth Authorization Server clause: Give authorization codes short bounded lifetimes. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.4.4` | L1 | traceability | Proposed N/A | Read the exact OAuth Authorization Server clause: Permit only required OAuth grants and exclude obsolete unsafe grants. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.4.5` | L1 | traceability | Proposed N/A | Read the exact OAuth Authorization Server clause: Protect refresh tokens from replay with the required binding or rotation. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.4.6` | L2 | traceability | Proposed N/A | Read the exact OAuth Authorization Server clause: Require strong PKCE validation for authorization-code grants. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.4.7` | L2 | traceability | Proposed N/A | Read the exact OAuth Authorization Server clause: Constrain untrusted dynamic-client registration and disclose its risk. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.4.8` | L2 | traceability | Proposed N/A | Read the exact OAuth Authorization Server clause: Give refresh tokens an absolute expiration. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.4.9` | L2 | traceability | Proposed N/A | Read the exact OAuth Authorization Server clause: Let authorized users revoke refresh and opaque access tokens. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.4.10` | L2 | traceability | Proposed N/A | Read the exact OAuth Authorization Server clause: Authenticate confidential clients on back-channel requests. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.4.11` | L2 | traceability | Proposed N/A | Read the exact OAuth Authorization Server clause: Assign only necessary scopes to each OAuth client. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.4.12` | L3 | traceability | Proposed N/A | Read the exact OAuth Authorization Server clause: Allow only each client's necessary response modes. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.4.13` | L3 | traceability | Proposed N/A | Read the exact OAuth Authorization Server clause: Combine code grants with pushed authorization requests at L3. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.4.14` | L3 | traceability | Proposed N/A | Read the exact OAuth Authorization Server clause: Issue sender-constrained access tokens at L3. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.4.15` | L3 | traceability | Proposed N/A | Read the exact OAuth Authorization Server clause: Protect server-client authorization details from user tampering. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.4.16` | L3 | traceability | Proposed N/A | Read the exact OAuth Authorization Server clause: Require confidential clients with strong replay-resistant asymmetric authentication. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.5.1` | L2 | traceability | Proposed N/A | Read the exact OIDC Client clause: Prevent replay of OIDC identity tokens. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.5.2` | L2 | traceability | Proposed N/A | Read the exact OIDC Client clause: Identify users by stable nonreassigned identity-token claims. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.5.3` | L2 | traceability | Proposed N/A | Read the exact OIDC Client clause: Match discovery metadata to the configured issuer exactly. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.5.4` | L2 | traceability | Proposed N/A | Read the exact OIDC Client clause: Validate that identity tokens target the receiving client. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.5.5` | L2 | traceability | Proposed N/A | Read the exact OIDC Client clause: Validate back-channel logout token purpose and structure. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.6.1` | L2 | traceability | Proposed N/A | Read the exact OpenID Provider clause: Restrict OpenID responses to supported secure flows. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.6.2` | L2 | traceability | Proposed N/A | Read the exact OpenID Provider clause: Prevent forced logout attacks at the OpenID provider. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.7.1` | L2 | traceability | Proposed N/A | Read the exact Consent Management clause: Obtain appropriate user consent for authorization requests. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.7.2` | L2 | traceability | Proposed N/A | Read the exact Consent Management clause: Explain requested authority, client identity and duration during consent. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |
| `v5.0.0-10.7.3` | L2 | traceability | Proposed N/A | Read the exact Consent Management clause: Let users review, change and revoke authorization consents. The declared product has no OAuth client/resource server/authorization server, OpenID provider or OAuth consent flow. Windows access tokens, NTS cookies and UAC intent are not OAuth/OIDC protocol objects; the scoped exclusion is justified at design level. |

### V11 — Cryptography

| Canonical ID | Minimum level | Primary reviewer | Current draft applicability | Review reason |
| --- | --- | --- | --- | --- |
| `v5.0.0-11.1.1` | L2 | security | Directly applicable (proposed) | Retain direct applicability: NTS/TLS/cache/signing keys need explicit owners, sharing limits, lifetimes, replacement and deletion; no implemented lifecycle is claimed. |
| `v5.0.0-11.1.2` | L2 | security | Directly applicable (proposed) | Retain direct applicability: inventory must include algorithms, certificates, keys and allowed uses across Rust, native providers and release signing. |
| `v5.0.0-11.1.3` | L3 | security | Directly applicable (proposed) | Retain L3 direct applicability: resolved-graph/native/build inspection is needed to discover cryptography hidden below top-level crate names. |
| `v5.0.0-11.1.4` | L3 | security | Directly applicable (proposed) | Retain L3 direct applicability: crypto migration/PQC planning belongs in maintained inventory; the plan does not assert current post-quantum protection. |
| `v5.0.0-11.2.1` | L2 | security | Directly applicable (proposed) | Retain direct applicability as an admission requirement: named maintained providers are candidates, and unaudited/held AES-SIV or CNG choices are not yet qualified industry validation. |
| `v5.0.0-11.2.2` | L2 | security | Directly applicable (proposed) | Retain direct applicability: versioned provider/association state and migration policy support agility; authenticated policy cannot authorize arbitrary attacker-selected algorithms. |
| `v5.0.0-11.2.3` | L2 | security | Directly applicable (proposed) | Retain direct applicability with its recorded certificate/key-size exception question; ordinary public-PKI validation alone does not prove every primitive reaches 128-bit strength. |
| `v5.0.0-11.2.4` | L3 | security | Needs decision | Retain Needs decision: exact algorithm/provider/platform constant-time evidence is missing and the AES-SIV limitation is disclosed; library branding is insufficient. |
| `v5.0.0-11.2.5` | L3 | security | Directly applicable (proposed) | Retain direct applicability: RNG, exporter, provider and tag errors must stop the operation without authentication bypass or secret-bearing diagnostics. |
| `v5.0.0-11.3.1` | L1 | security | Directly applicable (proposed) | Retain direct applicability: admitted primitives and transitive provider features must exclude insecure modes/padding; source-only naming does not prove the artifact. |
| `v5.0.0-11.3.2` | L1 | security | Directly applicable (proposed) | Retain direct applicability: protocol-required AES-SIV-CMAC-256 must receive an explicit approved configuration and key mapping, not substitution with AES-GCM-SIV. |
| `v5.0.0-11.3.3` | L2 | security | Directly applicable (proposed) | Retain direct applicability: AEAD validation precedes cookie/cache/model mutation and persisted secret integrity must be protected. |
| `v5.0.0-11.3.4` | L3 | security | Directly applicable (proposed) | Retain direct applicability: algorithm-appropriate per-key nonce/UID lifecycle and failure tests are required; SIV misuse resistance is not a waiver of the documented policy. |
| `v5.0.0-11.3.5` | L3 | security | Needs decision | Retain Needs decision: literal encrypt-then-MAC wording needs explicit treatment for standardized SIV AEAD internals; do not redesign the NTS standard to claim compliance. |
| `v5.0.0-11.4.1` | L1 | security | Directly applicable (proposed) | Retain direct applicability: approved cryptographic hash policy must cover signatures, exporters/KDFs and providers; nonsecurity hashes cannot be relabeled as cryptographic assurance. |
| `v5.0.0-11.4.2` | L2 | security | Proposed N/A | Retain scoped N/A: no product-owned password hash store; Windows account handling is outside the application implementation. |
| `v5.0.0-11.4.3` | L2 | security | Directly applicable (proposed) | Retain direct applicability: verify collision/second-preimage output length according to actual usage rather than counting an arbitrary checksum as cryptographic integrity. |
| `v5.0.0-11.4.4` | L2 | security | Proposed N/A | Retain scoped N/A: no application derivation of secret keys from user passwords; TLS exporter and DPAPI mechanisms are different primitives. |
| `v5.0.0-11.5.1` | L2 | security | Needs decision | Corrected in revision 1.4: The literal entropy requirement applies to all intended non-guessable values, not only bearer credentials; legacy plain NTP needs an explicit exception decision. Independent targeted recheck accepted the document correction; underlying admission/policy evidence remains pending where applicable. |
| `v5.0.0-11.5.2` | L3 | security | Directly applicable (proposed) | Retain L3 direct applicability: approved OS/provider entropy under demand, failure propagation and bounded request work need measured qualification, not a time-seeded fallback. |
| `v5.0.0-11.6.1` | L2 | security | Directly applicable (proposed) | Retain direct applicability: key generation and signing/verification require approved actual provider parameters and exact graph evidence. |
| `v5.0.0-11.6.2` | L3 | security | Directly applicable (proposed) | Retain L3 direct applicability: TLS key exchange parameters and authenticated NTS exporter mapping must be qualified independently of a successful transport connection. |
| `v5.0.0-11.7.1` | L3 | security | Needs decision | Retain Needs decision: DPAPI, ACLs and zeroization do not establish full in-use memory encryption; deployment prerequisite or explicit gap remains necessary. |
| `v5.0.0-11.7.2` | L3 | security | Directly applicable (proposed) | Retain L3 direct applicability: minimize plaintext key/cookie copies, exclude debug/export paths and protect persisted material promptly; perfect zeroization of all provider copies is not claimed. |

### V12 — Secure Communication

| Canonical ID | Minimum level | Primary reviewer | Current draft applicability | Review reason |
| --- | --- | --- | --- | --- |
| `v5.0.0-12.1.1` | L1 | security | Directly applicable (proposed) | Retain direct applicability: NTS-KE policy requires TLS 1.3 or later; transitive defaults enabling TLS 1.2 must not affect negotiation. |
| `v5.0.0-12.1.2` | L2 | security | Directly applicable (proposed) | Retain direct applicability including L3 forward secrecy: qualify actual suites and resumption/key-exchange behavior, not just the TLS version label. |
| `v5.0.0-12.1.3` | L2 | security | Proposed N/A | Retain scoped N/A: the baseline is not an mTLS server accepting client-certificate identity. |
| `v5.0.0-12.1.4` | L3 | security | Needs decision | Retain Needs decision: leaf-only/unknown-status behavior, OS-current-time revocation checks and offline policy are explicitly held under OPEN-8. |
| `v5.0.0-12.1.5` | L3 | security | Needs decision | Retain Needs decision: ECH interoperability is unqualified and absence exposes metadata; no silent L3 pass. |
| `v5.0.0-12.2.1` | L1 | security | Needs decision | Corrected in revision 1.4: Reconcile a current held direct/delegated certificate-retrieval candidate with the exact clause; no runtime failure is asserted. Independent targeted recheck accepted the document correction; underlying admission/policy evidence remains pending where applicable. |
| `v5.0.0-12.2.2` | L1 | security | Native adaptation (proposed) | Retain disclosed client/native adaptation and private-CA question: external server trust is explicit, and a managed internal NTS server may require a scoped public-trust exception. |
| `v5.0.0-12.3.1` | L2 | security | Needs decision | Retain Needs decision: NTP/NTS UDP confidentiality, optional helpers and local IPC cannot be described as literal encryption of every connection. |
| `v5.0.0-12.3.2` | L2 | security | Needs decision | Retain Needs decision: provisional-time validation changes assurance and cannot erase expired-key or current-OS-time revocation limits; normal validation remains mandatory outside the approved branch. |
| `v5.0.0-12.3.3` | L2 | security | Proposed N/A | Retain scoped N/A: no HTTP service-to-service channel exists inside the app; external native PKI retrieval is a separate current conditional question. |
| `v5.0.0-12.3.4` | L2 | security | Proposed N/A | Retain scoped N/A: engine/broker communications do not use internal TLS certificates; the kernel-identity adaptation is explicitly handled in V12.3.5. |
| `v5.0.0-12.3.5` | L3 | security | Native adaptation (proposed) | Retain native adaptation with recorded formal decision: reciprocal Windows peer identity and replay/generation guards must be proven; they are not literally intra-service PKI. |

### V13 — Configuration

| Canonical ID | Minimum level | Primary reviewer | Current draft applicability | Review reason |
| --- | --- | --- | --- | --- |
| `v5.0.0-13.1.1` | L2 | architecture | Directly applicable (proposed) | Network inventory includes DNS, NTP, negotiated endpoints, IPC and conditional certificate retrieval; actor-controlled destinations are named. |
| `v5.0.0-13.1.2` | L3 | architecture | Directly applicable (proposed) | Every independently scheduled service operation must have finite connection and at-capacity behavior; hidden native jobs are included. |
| `v5.0.0-13.1.3` | L3 | architecture | Directly applicable (proposed) | Resource owners, cleanup, timeouts and bounded backoff are required for sockets/handles/tasks; cancellation is not assumed to stop blocking work. |
| `v5.0.0-13.1.4` | L3 | architecture | Directly applicable (proposed) | Critical secrets have lifecycle/rotation documentation ownership; precise schedules remain implementation policy work, not claimed complete. |
| `v5.0.0-13.2.1` | L2 | architecture | Directly applicable (proposed) | Distinct effective service identities/SIDs and authenticated peers replace static shared secrets; actual token separation remains a P0 gate. |
| `v5.0.0-13.2.2` | L2 | architecture | Directly applicable (proposed) | Least privilege applies to OS APIs and interservice calls; engine clock/debug/injection authority must actually be absent. |
| `v5.0.0-13.2.3` | L2 | architecture | Directly applicable (proposed) | No default password/API credential is introduced; final installer and peer setup must substantiate absence. |
| `v5.0.0-13.2.4` | L2 | architecture | Directly applicable (proposed) | External resource allowlist is direct; DNS/rebinding/NTS delegation/LAN and direct/delegated certificate fetch are held under OPEN-8. |
| `v5.0.0-13.2.5` | L2 | architecture | Directly applicable (proposed) | Non-HTTP application service still needs resource restrictions; actual file/IPC/network endpoints rather than strings are checked. |
| `v5.0.0-13.2.6` | L3 | architecture | Directly applicable (proposed) | Connection/capacity/retry strategy is checked against actual retained native work, including recovery and positive controls. |
| `v5.0.0-13.3.1` | L2 | architecture | Needs decision | L3 hardware-backed condition embedded in an L2 row is retained as an unresolved vault/provider gap; DPAPI is not treated as HSM. |
| `v5.0.0-13.3.2` | L2 | architecture | Directly applicable (proposed) | Key holders, secret storage, sentinel disclosure and disposal are scoped to actual service rights, not process naming alone. |
| `v5.0.0-13.3.3` | L3 | architecture | Needs decision | All-operations isolated crypto requirement remains unresolved; a TPM for a subset or in-process CNG is not blanket compliance. |
| `v5.0.0-13.3.4` | L3 | architecture | Directly applicable (proposed) | Expiration/rekey/restart transitions must follow documented schedules and discard old association state without leaking it. |
| `v5.0.0-13.4.1` | L1 | architecture | Directly applicable (proposed) | Source-control metadata exclusion is direct and belongs in final package-content checks; a dependency/SBOM inventory alone is not execution evidence. |
| `v5.0.0-13.4.2` | L2 | architecture | Directly applicable (proposed) | Debug/test clocks, fault injection, developer endpoints and secret dumps excluded from release; sanitized diagnostics are an explicit separate feature. |
| `v5.0.0-13.4.3` | L2 | architecture | Proposed N/A | No web server directory-listing surface; future new surface reopens scope. |
| `v5.0.0-13.4.4` | L2 | architecture | Proposed N/A | Corrected in revision 1.4: Narrow the exclusion to the actual server/sensitive-action surface; avoid a blanket assertion that every HTTP workflow is absent. Independent targeted recheck accepted the document correction; underlying admission/policy evidence remains pending where applicable. |
| `v5.0.0-13.4.5` | L2 | architecture | Directly applicable (proposed) | Native status/monitoring interfaces are intentionally scoped to permitted readers; no HTTP prerequisite to apply this clause. |
| `v5.0.0-13.4.6` | L3 | architecture | Directly applicable (proposed) | Normal status hides backend versions, while product/support/SBOM disclosure remains explicitly open rather than falsely universally absent. |
| `v5.0.0-13.4.7` | L3 | architecture | Proposed N/A | No web tier that serves extension-selected files; static release distribution is outside the daemon server surface. |

### V14 — Data Protection

| Canonical ID | Minimum level | Primary reviewer | Current draft applicability | Review reason |
| --- | --- | --- | --- | --- |
| `v5.0.0-14.1.1` | L2 | architecture | Directly applicable (proposed) | Keys/cookies, source addresses, host/user identities and support exports have classification ownership; encoded data remains sensitive. |
| `v5.0.0-14.1.2` | L2 | architecture | Directly applicable (proposed) | Per-class holders, integrity, retention, diagnostics and crash exposure must be documented; unsupported guarantees remain visible. |
| `v5.0.0-14.2.1` | L1 | architecture | Proposed N/A | Corrected in revision 1.4: Keep the data-specific exclusion while explicitly covering the current conditional retrieval route. Independent targeted recheck accepted the document correction; underlying admission/policy evidence remains pending where applicable. |
| `v5.0.0-14.2.2` | L2 | architecture | Directly applicable (proposed) | TLS/session/cookie caches are explicitly in scope; stale disposal is tested rather than no-browser used as a blanket exclusion. |
| `v5.0.0-14.2.3` | L2 | architecture | Directly applicable (proposed) | No automatic telemetry/tracker; actual peer payload and support projection must exclude unintended sensitive fields. |
| `v5.0.0-14.2.4` | L2 | architecture | Directly applicable (proposed) | Data-class guarantees are traced through disk/IPC/UI/diagnostics; per-class policy rather than generic encryption label owns acceptance. |
| `v5.0.0-14.2.5` | L3 | architecture | Proposed N/A | No public web cache/resource fallback surface; ordinary NTS or OS caches do not automatically instantiate Web Cache Deception. |
| `v5.0.0-14.2.6` | L3 | architecture | Directly applicable (proposed) | Minimal status DTOs and forbidden secret projections meet the intended scope; NTS keys are never revealable UI data. |
| `v5.0.0-14.2.7` | L3 | architecture | Directly applicable (proposed) | Retention covers logs, association state, obsolete policy and uninstall; logical deletion is not misrepresented as forensic erasure. |
| `v5.0.0-14.2.8` | L3 | architecture | Proposed N/A | No uploaded image/document metadata persistence; configuration values are parsed fields. Reopen for future file-ingestion metadata. |
| `v5.0.0-14.3.1` | L1 | architecture | Native adaptation (proposed) | Native privileged view/session cleanup is an adaptation; what non-sensitive stale status may persist is expressly open. |
| `v5.0.0-14.3.2` | L2 | architecture | Proposed N/A | No browser-sensitive response caching or HTTP header producer; native state cleanup is covered separately. |
| `v5.0.0-14.3.3` | L2 | architecture | Proposed N/A | No browser storage; NTS cookies are protocol credentials, not browser cookie storage. |

### V15 — Secure Coding and Architecture

| Canonical ID | Minimum level | Primary reviewer | Current draft applicability | Review reason |
| --- | --- | --- | --- | --- |
| `v5.0.0-15.1.1` | L1 | architecture | Directly applicable (proposed) | Risk-based update/remediation deadlines include transitive/native/build/installer dependencies; P0 must freeze the policy. |
| `v5.0.0-15.1.2` | L2 | architecture | Directly applicable (proposed) | Complete trusted-source SBOM and native/build closure are required; lockfile alone is not the inventory. |
| `v5.0.0-15.1.3` | L2 | architecture | Directly applicable (proposed) | Expensive DNS/TLS/export/IPC/lifecycle paths have owner and deadline requirements; synchronous callback blocking is explicitly addressed. |
| `v5.0.0-15.1.4` | L3 | architecture | Directly applicable (proposed) | Risky crypto/parser/unsafe/installer components are identified by exposure with held adoption gates, not cleared by Rust labels. |
| `v5.0.0-15.1.5` | L3 | architecture | Directly applicable (proposed) | Clock writes, service/account changes, unsafe FFI and privileged filesystem effects are named dangerous capabilities. |
| `v5.0.0-15.2.1` | L1 | architecture | Directly applicable (proposed) | Exact graph/advisory age/remediation deadline release checks have invalid and valid fixtures; absent/stale advisory data is not clearance. |
| `v5.0.0-15.2.2` | L2 | architecture | Directly applicable (proposed) | Native-work capacity stays with actual jobs; generation guards and independent broker control address partial cancellation. |
| `v5.0.0-15.2.3` | L2 | architecture | Directly applicable (proposed) | Test/debug/alternative provider features excluded from production; final feature/native inventory is required, not inferred from top-level defaults. |
| `v5.0.0-15.2.4` | L3 | architecture | Directly applicable (proposed) | Exact checksum/origin and transitive/source override review covers dependency confusion; approved and substituted graph fixtures are distinct. |
| `v5.0.0-15.2.5` | L3 | architecture | Directly applicable (proposed) | Process/token/file/IPC restrictions protect dangerous effects; compromised-engine residual is explicitly bounded rather than claimed eliminated. |
| `v5.0.0-15.3.1` | L1 | architecture | Directly applicable (proposed) | Only permitted projections cross UI/IPC/log boundaries; internal object serialization cannot expose secret fields. |
| `v5.0.0-15.3.2` | L2 | architecture | Needs decision | Corrected in revision 1.4: Reconcile a current held direct/delegated certificate-retrieval candidate with the exact clause; no runtime failure is asserted. Independent targeted recheck accepted the document correction; underlying admission/policy evidence remains pending where applicable. |
| `v5.0.0-15.3.3` | L2 | architecture | Directly applicable (proposed) | Action-specific strict DTOs/private constructors reject mass assignment, including unknown/duplicate/privileged fields and generic patches. |
| `v5.0.0-15.3.4` | L2 | architecture | Proposed N/A | No user-IP proxy/middleware forwarding chain; native peer identity is OS-derived and NTP endpoint binding is separate. |
| `v5.0.0-15.3.5` | L2 | architecture | Directly applicable (proposed) | Units/sign/era/integer/floating exceptional values and FFI boundaries are included despite Rust typing; independent vectors retained. |
| `v5.0.0-15.3.6` | L2 | architecture | Proposed N/A | No JavaScript prototype/object runtime; native serialization controls remain in scope. |
| `v5.0.0-15.3.7` | L2 | architecture | Proposed N/A | No HTTP request parameter merging; duplicate native JSON keys are explicitly rejected under their actual clauses. |
| `v5.0.0-15.4.1` | L3 | architecture | Directly applicable (proposed) | Single-owner models, typed messages, native callback lifetime and real-thread shared resource gates address concurrency. |
| `v5.0.0-15.4.2` | L3 | architecture | Directly applicable (proposed) | Resource check/use needs stable handles/current serialized authority; global clock has no ownership CAS and host/admin races are disclosed outside guarantee. |
| `v5.0.0-15.4.3` | L3 | architecture | Directly applicable (proposed) | Encapsulated resource owners and bounded stop/retry checks cover deadlock/livelock, rather than assuming async removes locks. |
| `v5.0.0-15.4.4` | L3 | architecture | Directly applicable (proposed) | Fair endpoint scheduling and independent lifecycle/control work preserve progress; nonterminating job recovery remains an explicit bounded gate. |

### V16 — Security Logging and Error Handling

| Canonical ID | Minimum level | Primary reviewer | Current draft applicability | Review reason |
| --- | --- | --- | --- | --- |
| `v5.0.0-16.1.1` | L2 | architecture | Directly applicable (proposed) | Full stack event/sink/ACL/retention inventory includes installer, provider and OS-delegated boundaries. |
| `v5.0.0-16.2.1` | L2 | architecture | Directly applicable (proposed) | Subject/correlation/generation and UTC quality plus monotonic ordering permit reconstruction without trusting sender-claimed identity. |
| `v5.0.0-16.2.2` | L2 | architecture | Needs decision | Wrong-clock logging creates an explicit synchronized-time decision; metadata is not represented as magically correct UTC. |
| `v5.0.0-16.2.3` | L2 | architecture | Directly applicable (proposed) | Documented sinks and failure behavior apply to actual Windows event resources and optional diagnostics; no telemetry implied. |
| `v5.0.0-16.2.4` | L2 | architecture | Directly applicable (proposed) | Actual Event Viewer/parser rendering and version/correlation tests are required, not merely JSON formatting success. |
| `v5.0.0-16.2.5` | L2 | architecture | Directly applicable (proposed) | Sentinel secrets exercise all real sinks/export/error paths; zeroing and redacted wrappers have candid memory/dump limitations. |
| `v5.0.0-16.3.1` | L2 | architecture | Native adaptation (proposed) | Owned IPC/TLS/NTS authentication outcomes are a native adaptation; unrelated Windows login events remain OS-owned. |
| `v5.0.0-16.3.2` | L2 | architecture | Directly applicable (proposed) | L3 all-authorization-decisions and sensitive-access condition embedded in L2 is preserved; finite log loss/fail-safe policy is explicit open work. |
| `v5.0.0-16.3.3` | L2 | architecture | Directly applicable (proposed) | Bypass events include actual malformed/overload/stale input and safe bounded outcomes; logging failure cannot grant authority. |
| `v5.0.0-16.3.4` | L2 | architecture | Directly applicable (proposed) | Unexpected TLS/provider/saturation/recovery failures require observable sanitized events with healthy-operation control. |
| `v5.0.0-16.4.1` | L2 | architecture | Directly applicable (proposed) | Typed/encoded events preserve single-event structure under hostile delimiters/control characters and error paths. |
| `v5.0.0-16.4.2` | L2 | architecture | Directly applicable (proposed) | Read/rewrite/truncate/delete attempts use actual sink ACLs and distinct principals; same-host malicious administrator limits are disclosed. |
| `v5.0.0-16.4.3` | L2 | architecture | Needs decision | Separate analysis/detection/alerting boundary remains unresolved; local rotation alone is not called sufficient, and no silent telemetry is added. |
| `v5.0.0-16.5.1` | L2 | architecture | Directly applicable (proposed) | Generic safe error classes/correlation IDs separate ordinary feedback from privileged diagnostics without raw secrets/stacks. |
| `v5.0.0-16.5.2` | L2 | architecture | Directly applicable (proposed) | Real dependency faults lead to honest bounded holdover/Blocked and recoverable operation, not hidden unauthenticated fallback. |
| `v5.0.0-16.5.3` | L2 | architecture | Directly applicable (proposed) | Invalid/exception/partial state cannot fail open; applied-effect uncertainty is reconciled rather than automatically duplicated. |
| `v5.0.0-16.5.4` | L3 | architecture | Needs decision | Catch-all process-availability clause remains an explicit native fail-stop/SCM interpretation gap; catch_unwind is not assumed safe for FFI/invariant corruption. |

### V17 — WebRTC

| Canonical ID | Minimum level | Primary reviewer | Current draft applicability | Review reason |
| --- | --- | --- | --- | --- |
| `v5.0.0-17.1.1` | L2 | traceability | Proposed N/A | Read the exact WebRTC clause: Restrict TURN destinations to allowed nonspecial addresses. No TURN, DTLS-SRTP media, recording or WebRTC signaling role is selected. NTP UDP traffic does not create that role; generic UDP resource/parse controls remain elsewhere. |
| `v5.0.0-17.1.2` | L3 | traceability | Proposed N/A | Read the exact WebRTC clause: Prevent TURN port-allocation exhaustion. No TURN, DTLS-SRTP media, recording or WebRTC signaling role is selected. NTP UDP traffic does not create that role; generic UDP resource/parse controls remain elsewhere. |
| `v5.0.0-17.2.1` | L2 | traceability | Proposed N/A | Read the exact WebRTC clause: Manage DTLS certificate keys according to policy. No TURN, DTLS-SRTP media, recording or WebRTC signaling role is selected. NTP UDP traffic does not create that role; generic UDP resource/parse controls remain elsewhere. |
| `v5.0.0-17.2.2` | L2 | traceability | Proposed N/A | Read the exact WebRTC clause: Use approved DTLS suites and SRTP key-establishment profiles. No TURN, DTLS-SRTP media, recording or WebRTC signaling role is selected. NTP UDP traffic does not create that role; generic UDP resource/parse controls remain elsewhere. |
| `v5.0.0-17.2.3` | L2 | traceability | Proposed N/A | Read the exact WebRTC clause: Verify SRTP authentication before accepting media. No TURN, DTLS-SRTP media, recording or WebRTC signaling role is selected. NTP UDP traffic does not create that role; generic UDP resource/parse controls remain elsewhere. |
| `v5.0.0-17.2.4` | L2 | traceability | Proposed N/A | Read the exact WebRTC clause: Continue safe media processing despite malformed SRTP packets. No TURN, DTLS-SRTP media, recording or WebRTC signaling role is selected. NTP UDP traffic does not create that role; generic UDP resource/parse controls remain elsewhere. |
| `v5.0.0-17.2.5` | L3 | traceability | Proposed N/A | Read the exact WebRTC clause: Maintain media service under legitimate-user SRTP floods. No TURN, DTLS-SRTP media, recording or WebRTC signaling role is selected. NTP UDP traffic does not create that role; generic UDP resource/parse controls remain elsewhere. |
| `v5.0.0-17.2.6` | L3 | traceability | Proposed N/A | Read the exact WebRTC clause: Prevent the DTLS ClientHello race vulnerability. No TURN, DTLS-SRTP media, recording or WebRTC signaling role is selected. NTP UDP traffic does not create that role; generic UDP resource/parse controls remain elsewhere. |
| `v5.0.0-17.2.7` | L3 | traceability | Proposed N/A | Read the exact WebRTC clause: Keep recording service available during SRTP floods. No TURN, DTLS-SRTP media, recording or WebRTC signaling role is selected. NTP UDP traffic does not create that role; generic UDP resource/parse controls remain elsewhere. |
| `v5.0.0-17.2.8` | L3 | traceability | Proposed N/A | Read the exact WebRTC clause: Match DTLS certificates to the negotiated SDP fingerprint. No TURN, DTLS-SRTP media, recording or WebRTC signaling role is selected. NTP UDP traffic does not create that role; generic UDP resource/parse controls remain elsewhere. |
| `v5.0.0-17.3.1` | L2 | traceability | Proposed N/A | Read the exact WebRTC clause: Rate-limit signaling floods while serving legitimate messages. No TURN, DTLS-SRTP media, recording or WebRTC signaling role is selected. NTP UDP traffic does not create that role; generic UDP resource/parse controls remain elsewhere. |
| `v5.0.0-17.3.2` | L2 | traceability | Proposed N/A | Read the exact WebRTC clause: Handle malformed signaling without losing service availability. No TURN, DTLS-SRTP media, recording or WebRTC signaling role is selected. NTP UDP traffic does not create that role; generic UDP resource/parse controls remain elsewhere. |
