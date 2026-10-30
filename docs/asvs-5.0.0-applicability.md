# OWASP ASVS 5.0.0 applicability profile

**Product:** Rust Windows Time Client  
**Plan:** RWTC-ARCH-001, revision 1.4, 28 October 2026  
**Status:** planning profile; no implemented controls or executed application checks

This Markdown document contains all **345** requirements from the pinned release. Common provenance fields are factored below. Each requirement retains its original planned verification and all earlier revision additions. Revision 1.4 explicitly corrects **16 records**, including **seven draft classification changes**; every other original field remains unchanged. The correction table and per-record notes retain the changed fields, prior values and reasons. Short summaries are adapted explanations, not authoritative standard text; read the full pinned clause, including stronger L3 conditions inside L1/L2 entries.

The revised proposed dispositions are **109 directly applicable, 28 native adaptations, 181 proposed N/A and 27 needing a decision**. **Thirty-nine rows** have open questions. No control has been promoted to implemented, executed or conformant. Native adaptations are not claims of literal equivalence. All classifications remain draft and require owner review.

The [architecture and threat model](windows-time-architecture.md) owns product decisions, trust boundaries, THR/CHECK identifiers and the CTRL-label map in Appendix C. The [full independent review report](windows-time-review.md) records scope, findings and rechecks. The preserved revisions 1.1–1.3 verification additions affect **42 unique rows**; revision 1.3 contributes detail to **23 rows**. Revision 1.4 correction notes are a separate history and do not erase those additions.

## Common provenance fields

**30 October contract refinement:** [architecture §20.2.1](windows-time-architecture.md#2021-contract-review-acceptance-refinements)
and [hardening §9.10](windows-time-development-hardening.md#910-contract-review-qualification-matrix)
add evidence cases for native owner-rights isolation, durable correction accounting,
adjustment modes, authentication lifetime, semantic-floor recovery, fresh quorum,
SCM maintenance, mutation omissions and broken-clock signing. Apply these to
affected row plans through their existing CHECK/native mappings. All 345 IDs,
dispositions, prior notes and unexecuted statuses remain; this amendment accepts
no ASVS exception and establishes no additional conformity.

Every requirement record inherits these exact fields:

| Field | Value |
| --- | --- |
| `asvs_version` | 5.0.0 |
| `canonical_source_url` | https://github.com/OWASP/ASVS/blob/5cf9b032440be53ce345ab3c130fda46ba1ce7a2/5.0/docs_en/OWASP_Application_Security_Verification_Standard_5.0.0_en.json |
| `source_commit` | 5cf9b032440be53ce345ab3c130fda46ba1ce7a2 |
| `source_license` | CC BY-SA 4.0 |
| `adaptation_license` | CC BY-SA 4.0 |
| `attribution` | Adapted from OWASP ASVS 5.0.0; summaries and applicability analysis are modifications. |
| `prepared_date` | 2026-10-28 |

The worksheet is adapted from the OWASP ASVS project and contributors under [Creative Commons Attribution-ShareAlike 4.0 International](https://creativecommons.org/licenses/by-sa/4.0/). Its summaries, native applicability analysis, review corrections and revision-specific verification additions are modifications under the same documentation license. See the [pinned source license](https://github.com/OWASP/ASVS/blob/5cf9b032440be53ce345ab3c130fda46ba1ce7a2/LICENSE.md). This is not OWASP endorsement and does not select or change the future software's license. The common `prepared_date` is original provenance; the revision date above identifies this update.

## Revision 1.4 review corrections

These are explicit changes to proposed scope and wording, not implementation passes. OPEN-8 resolves the current held certificate-retrieval route and actual Windows/dependency responsibility; it does not introduce a web server. OPEN-9 owns the legacy-NTP entropy exception. Original field values are retained in each affected record's correction note. All verification below is planned, not executed.

| Requirement | Changed fields | Reason |
| --- | --- | --- |
| `v5.0.0-1.2.3` | Applicability rationale, Open question | DEC-18 already selects JSON; the representation decision is answered. |
| `v5.0.0-3.5.3` | Applicability rationale | Narrow the exclusion to the actual server/sensitive-action surface; avoid a blanket assertion that every HTTP workflow is absent. |
| `v5.0.0-4.1.1` | Applicability rationale | Narrow the exclusion to the actual server/sensitive-action surface; avoid a blanket assertion that every HTTP workflow is absent. |
| `v5.0.0-4.1.2` | Applicability rationale | Narrow the exclusion to the actual server/sensitive-action surface; avoid a blanket assertion that every HTTP workflow is absent. |
| `v5.0.0-4.1.4` | Applicability rationale | Narrow the exclusion to the actual server/sensitive-action surface; avoid a blanket assertion that every HTTP workflow is absent. |
| `v5.0.0-4.2.1` | Proposed applicability, Applicability rationale, Native control mapping, Decision / implementation owner, Open question | Reconcile a current held direct/delegated certificate-retrieval candidate with the exact clause; no runtime failure is asserted. |
| `v5.0.0-4.2.2` | Proposed applicability, Applicability rationale, Native control mapping, Decision / implementation owner, Open question | Reconcile a current held direct/delegated certificate-retrieval candidate with the exact clause; no runtime failure is asserted. |
| `v5.0.0-4.2.3` | Proposed applicability, Applicability rationale, Native control mapping, Decision / implementation owner, Open question | Reconcile a current held direct/delegated certificate-retrieval candidate with the exact clause; no runtime failure is asserted. |
| `v5.0.0-4.2.4` | Applicability rationale | Narrow the exclusion to the actual server/sensitive-action surface; avoid a blanket assertion that every HTTP workflow is absent. |
| `v5.0.0-4.2.5` | Proposed applicability, Applicability rationale, Native control mapping, Decision / implementation owner, Open question | Reconcile a current held direct/delegated certificate-retrieval candidate with the exact clause; no runtime failure is asserted. |
| `v5.0.0-6.1.3` | Applicability rationale | Align the authentication inventory with the existing broker-mediated status design. |
| `v5.0.0-11.5.1` | Proposed applicability, Applicability rationale, Native control mapping, Decision / implementation owner, Open question | The literal entropy requirement applies to all intended non-guessable values, not only bearer credentials; legacy plain NTP needs an explicit exception decision. |
| `v5.0.0-12.2.1` | Proposed applicability, Applicability rationale, Native control mapping, Decision / implementation owner, Open question | Reconcile a current held direct/delegated certificate-retrieval candidate with the exact clause; no runtime failure is asserted. |
| `v5.0.0-13.4.4` | Applicability rationale | Narrow the exclusion to the actual server/sensitive-action surface; avoid a blanket assertion that every HTTP workflow is absent. |
| `v5.0.0-14.2.1` | Applicability rationale | Keep the data-specific exclusion while explicitly covering the current conditional retrieval route. |
| `v5.0.0-15.3.2` | Proposed applicability, Applicability rationale, Native control mapping, Decision / implementation owner, Open question | Reconcile a current held direct/delegated certificate-retrieval candidate with the exact clause; no runtime failure is asserted. |

## Preserved revision-specific verification additions

Revision 1.1 covers cumulative correction authority, privileged installer maintenance, outbound endpoint admission, authorization lifetime, mixed update generations and numeric representation. Revision 1.2 adds strict runtime validation, constructor integrity, bounded decoding and safe failure for THR-25. Revision 1.3 adds exact dependency provenance/features, Windows trust and native-work lifetime/availability evidence for DEC-19 and THR-26, plus diagnostics qualification. These historical additions preserved their then-current classifications. Revision 1.4's explicit corrections above are separate; no browser/login stack is introduced and all additions remain unexecuted.

| Threat | Affected requirement IDs | Planned verification refinement |
| --- | --- | --- |
| Revision 1.1: THR-19 | `v5.0.0-2.1.3`, `v5.0.0-2.3.2`, `v5.0.0-2.2.3` | CHECK-3/10/12: repeated just-below-limit corrections, alternating signs, actual applied rate integration, relabelled stable compensation and restart must consume the same bounded aggregate authority; legitimate oscillator discipline has an independent acceptable trajectory |
| Revision 1.1: THR-20 | `v5.0.0-2.2.1`, `v5.0.0-2.2.2`, `v5.0.0-8.2.1`, `v5.0.0-8.3.1`, `v5.0.0-8.3.3`, `v5.0.0-13.2.2`, `v5.0.0-15.4.2` | CHECK-13/15/23: hostile public properties, supported/unsupported transforms, rollback records and user-controlled paths through actual install/repair/remove entrypoints cannot trigger arbitrary service/file/process effects; forward and rollback use effective caller/operation authority and stable validated handles; normal complete lifecycle succeeds |
| Revision 1.1: THR-21 | `v5.0.0-13.1.1`, `v5.0.0-13.2.4`, `v5.0.0-13.2.5` | CHECK-6/8/9/21: denied negotiated/resolved loopback/internal/out-of-scope address or port, DNS rebinding and reconnect produce no unauthorized network operation; explicitly approved LAN and legitimate permitted negotiated destinations succeed; retain existing native-adaptation treatment where applicable |
| Revision 1.1: THR-22 | `v5.0.0-8.3.2`, `v5.0.0-8.2.1`, `v5.0.0-8.3.1` | CHECK-2/3/4: revoke delegation during a live real IPC session and before queued sensitive action; test product-policy revision separately from existing Windows token group membership; prohibit effect after effective revocation and record any unresolved literal immediate-revocation gap without calling a later correction equivalent |
| Revision 1.1: THR-23 | `v5.0.0-2.3.1`, `v5.0.0-2.3.3`, `v5.0.0-2.3.4`, `v5.0.0-15.4.2` | CHECK-3/12/13/23: old delayed proposal and mixed-version helper/UI cannot mutate the new generation; fault/reboot at upgrade stages leaves one valid owner or explicit blocked recovery; rollback restores compatible state without replenishing consumed authority; successful permitted upgrade remains usable |
| Revision 1.1: THR-24 | `v5.0.0-2.2.1`, `v5.0.0-2.2.3`, `v5.0.0-15.3.5` | CHECK-6/7/9/10: independent era/overflow/NaN/unit/sign vectors and actual positive/negative Windows effects; validate post-conversion representability and actual acknowledgement; valid near-boundary values still work |
| Revision 1.2: THR-25 / Documented representations and limits | `v5.0.0-2.1.1`, `v5.0.0-2.1.3` | CHECK-25/P1: freeze and review frame/file/collection/depth/error limits, object shape, duplicate/unknown-key policy, exact units, presence/null/default semantics, supported versions and failure classes; valid boundary-sized records must remain admissible. These are documentation and future implementation obligations, not an executed control. |
| Revision 1.2: THR-25 / Canonical typed admission at the service | `v5.0.0-1.1.1`, `v5.0.0-2.2.1`, `v5.0.0-2.2.2`, `v5.0.0-2.2.3`, `v5.0.0-15.3.3`, `v5.0.0-15.3.5` | CHECK-25 with CHECK-3/15: exercise the actual receiver with duplicate/escaped/unknown keys, nested array-for-object input, absent/null/default ambiguity, trailing documents, generic protected-field patches, integer boundaries and invalid combinations. Inspect all constructor/Deserialize/From/Default/mutation paths. Invalid input has no effect; independently valid neighboring inputs, supported migrations and exact integer round trips succeed. |
| Revision 1.2: THR-25 / Resource and flow admission | `v5.0.0-2.3.1`, `v5.0.0-2.3.2`, `v5.0.0-2.4.1` | CHECK-25 with CHECK-3/21: test well-shaped out-of-order operations, oversized/growing reads, excessive collections/nesting, escaped strings and repeated invalid input. Measure bounded peak allocation/work/errors before teardown, rather than relying on post-parse length checks. Legitimate bounded sequences and maximum permitted inputs still work. |
| Revision 1.2: THR-25 / Current authority and destination use | `v5.0.0-8.2.1`, `v5.0.0-8.3.1`, `v5.0.0-13.2.4`, `v5.0.0-15.4.2` | CHECK-25 with CHECK-2/3/6/10: valid DTOs with serialized admin/validated claims, revoked or stale context, prohibited resolved/negotiated destinations and a changed resource before effect cannot gain authority. Check current state in the protected operation using the real enforcement path; a fresh authorized permitted operation succeeds. Pure validator tests do not qualify Windows authorization. |
| Revision 1.2: THR-25 / Safe observable failure | `v5.0.0-16.2.5`, `v5.0.0-16.3.3`, `v5.0.0-16.4.1`, `v5.0.0-16.5.1`, `v5.0.0-16.5.3` | CHECK-25 with CHECK-15/17/22: supply secret-bearing payloads and hostile field names; require allowlisted bounded error paths/codes, appropriate encoded/redacted bypass events and no raw rejected values. Rejection keeps the prior valid policy or blocks unsafe startup, never substitutes permissive critical defaults. Useful safe feedback and documented optional-cache recovery still work. |
| Revision 1.3: THR-15 / Dependency remediation | `v5.0.0-15.2.1` | CHECK-26: establish documented update/remediation deadlines; test a known overdue or affected component and an approved patched neighboring graph through the actual release gate. Record database freshness, exact artifact and justified exceptions; no absence-of-advisory clearance. |
| Revision 1.3: THR-15 / Required production functionality | `v5.0.0-15.2.3` | CHECK-26: inspect production feature closure and binary/native inventory for unwanted TLS providers, TLS 1.2, certificate debug logs, examples and dev-only code; a deliberately expanded feature fixture must be detected while the approved minimal graph succeeds. |
| Revision 1.3: THR-15 / Expected dependency sources | `v5.0.0-15.2.4` | CHECK-26: compare package checksums and associated source, inspect build/proc-macro/native components and approved origins, and reject a substituted/unexpected source. Reconcile lockfile, embedded dependency metadata, complete SBOM and final artifact inventory; an approved origin succeeds. |
| Revision 1.3: THR-5/14/26 / Native encapsulation and privilege | `v5.0.0-15.2.5`, `v5.0.0-13.2.2` | CHECK-1/2/16/26/27: inspect actual service tokens and dependency placement, deny engine clock/injection and unauthorized pipe effects, qualify narrow FFI ownership/mitigations and keep TLS/AEAD out of the broker. Positive permitted operations and normal provider loading must work. Preserve literal ASVS interpretation limits. |
| Revision 1.3: THR-8/26 / Availability and connection budgets | `v5.0.0-15.2.2`, `v5.0.0-2.4.1`, `v5.0.0-13.2.6`, `v5.0.0-15.4.4` | CHECK-27 with CHECK-8/9/21: saturate TLS/DNS/native job, byte, connection and retry budgets; keep permits with jobs until actual completion, reject/coalesce before submission and avoid replacement-pool growth. Show fair scheduling when capacity exists, independent lifecycle/broker responsiveness, diagnosed saturation and a valid request after bounded recovery. |
| Revision 1.3: THR-11/12/26 / Shared ownership and shutdown | `v5.0.0-15.4.1`, `v5.0.0-15.4.3` | CHECK-27: exercise failed/partial initialization, preshutdown and permitted late controls, cancellation during blocking work and repeated stop; prove correct destructor/context lifetime, no ABI unwind and bounded shutdown without TerminateThread. Use an instrumented concurrency model only for actual shared state; real Windows evidence remains necessary. |
| Revision 1.3: THR-11/22/26 / Current state at effects | `v5.0.0-15.4.2`, `v5.0.0-2.3.2` | CHECK-2/3/11/27: delay a native result across timeout, cancellation, policy revocation, resume and service restart, then ensure no stale model update, fresh-trust status or clock proposal reaches an effect. Retain applied correction budgets across recovery and admit the fresh authorized neighboring operation. |
| Revision 1.3: THR-2/3/26 / Windows certificate policy | `v5.0.0-12.3.2` | CHECK-8/9/26/27: test actual service-account roots, enterprise distrust, hostname/signature/algorithm validation, revoked/expired/not-yet-valid/unknown status, offline operation and gross past/future OS dates. Record leaf-versus-chain and soft/hard revocation policy, current-time CRL limitation and bootstrap refusal/exception. Do not treat unknown as confirmed non-revoked; legitimate required-policy validation succeeds. |
| Revision 1.3: THR-21/26 / Native retrieval destinations | `v5.0.0-13.2.4`, `v5.0.0-13.2.5` | CHECK-6/8/9/27: observe the real process/service performing each retrieval, enforce the documented allowed resource/protocol policy, block denied destinations and show permitted acquisition. Do not claim engine firewall rules cover delegated services; inability to express required policy blocks verifier admission. Preserve existing native-adaptation classifications. |
| Revision 1.3: THR-10/18/26 / Windows event inventory and correlation | `v5.0.0-16.2.1`, `v5.0.0-16.2.2`, `v5.0.0-16.2.3`, `v5.0.0-16.2.4` | CHECK-14/17/23/27: verify installer-owned Operational/Admin or explicitly chosen Application channel, provider resources, ACLs, retention and actual Event Viewer rendering; include UTC/quality/generation and monotonic correlation as specified. Exercise missing/full sinks and gross clock shifts; retain any literal synchronization gap rather than treating metadata as proof of correct UTC. |
| Revision 1.3: THR-10 / Secret-safe diagnostics | `v5.0.0-16.2.5` | CHECK-17/26/27: inject secret-bearing failures and hostile strings through real errors/events/exports; prove explicit bounded fields, no key/cookie/raw certificate or packet dumps, no automatic secret serialization and safe failed sinks. Qualify owned-buffer zeroing/copy paths without claiming protection from privileged dumps or forced termination. |
| Revision 1.3: THR-8/10/26 / Security and failure events | `v5.0.0-16.3.3`, `v5.0.0-16.3.4` | CHECK-17/27: verify stable events for bypass attempts, certificate-policy failure, saturation, stale-result rejection and stop/recovery, with rate limiting/counters and no sensitive payload. Demonstrate that logging failure cannot authorize a denied effect or block clock cleanup, and legitimate operation remains diagnosable. |

## Chapter index

- [V1 - Encoding and Sanitization](#v1---encoding-and-sanitization)
- [V2 - Validation and Business Logic](#v2---validation-and-business-logic)
- [V3 - Web Frontend Security](#v3---web-frontend-security)
- [V4 - API and Web Service](#v4---api-and-web-service)
- [V5 - File Handling](#v5---file-handling)
- [V6 - Authentication](#v6---authentication)
- [V7 - Session Management](#v7---session-management)
- [V8 - Authorization](#v8---authorization)
- [V9 - Self-contained Tokens](#v9---self-contained-tokens)
- [V10 - OAuth and OIDC](#v10---oauth-and-oidc)
- [V11 - Cryptography](#v11---cryptography)
- [V12 - Secure Communication](#v12---secure-communication)
- [V13 - Configuration](#v13---configuration)
- [V14 - Data Protection](#v14---data-protection)
- [V15 - Secure Coding and Architecture](#v15---secure-coding-and-architecture)
- [V16 - Security Logging and Error Handling](#v16---security-logging-and-error-handling)
- [V17 - WebRTC](#v17---webrtc)

## V1 - Encoding and Sanitization

### v5.0.0-1.1.1

| Field | Value |
| --- | --- |
| Source shortcode | V1.1.1 |
| Minimum assigned level | L2 |
| Chapter | Encoding and Sanitization |
| Section | Encoding and Sanitization Architecture |
| Adapted requirement summary | Decode each expected representation once before validation. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic input, parsing or representation control |
| Applicability rationale | Network packets, local IPC and persisted/configuration data cross trust or representation boundaries. |
| Native control mapping | CTRL-INPUT: bounded canonical decoding and validated domain types; CTRL-SERIAL: typed serializers. |
| Decision / implementation owner | Protocol/core; configuration; IPC |
| Original planned verification | Use malformed, doubly encoded and valid fixtures; verify canonicalization occurs once before admission and never changes an already admitted value. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.2, THR-25:** Only the decoded and fallibly constructed value may reach a trusted handler; UI validation and generic map normalization are not that enforcement. CHECK-25 with CHECK-3/15: exercise the actual receiver with duplicate/escaped/unknown keys, nested array-for-object input, absent/null/default ambiguity, trailing documents, generic protected-field patches, integer boundaries and invalid combinations. Inspect all constructor/Deserialize/From/Default/mutation paths. Invalid input has no effect; independently valid neighboring inputs, supported migrations and exact integer round trips succeed.

### v5.0.0-1.1.2

| Field | Value |
| --- | --- |
| Source shortcode | V1.1.2 |
| Minimum assigned level | L2 |
| Chapter | Encoding and Sanitization |
| Section | Encoding and Sanitization Architecture |
| Adapted requirement summary | Apply sink-specific encoding at the final output boundary. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic input, parsing or representation control |
| Applicability rationale | Network packets, local IPC and persisted/configuration data cross trust or representation boundaries. |
| Native control mapping | CTRL-INPUT: bounded canonical decoding and validated domain types; CTRL-SERIAL: typed serializers. |
| Decision / implementation owner | Protocol/core; configuration; IPC |
| Original planned verification | Inspect actual IPC/config/log serializers and native text sinks; use delimiter/control-character inputs and verify structural integrity. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-1.2.1

| Field | Value |
| --- | --- |
| Source shortcode | V1.2.1 |
| Minimum assigned level | L1 |
| Chapter | Encoding and Sanitization |
| Section | Injection Prevention |
| Adapted requirement summary | Encode HTTP, HTML and XML output for its exact context. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | No HTTP/HTML/XML response or document renderer is planned; native text and any JSON/binary serializers have separate controls. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-1.2.2

| Field | Value |
| --- | --- |
| Source shortcode | V1.2.2 |
| Minimum assigned level | L1 |
| Chapter | Encoding and Sanitization |
| Section | Injection Prevention |
| Adapted requirement summary | Encode dynamic URL components and restrict schemes. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | No dynamic URL construction is planned; NTP endpoints are parsed host/port data. Any external help/update URL feature reopens this row. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-1.2.3

| Field | Value |
| --- | --- |
| Source shortcode | V1.2.3 |
| Minimum assigned level | L1 |
| Chapter | Encoding and Sanitization |
| Section | Injection Prevention |
| Adapted requirement summary | Serialize dynamic JavaScript or JSON without structure injection. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic input, parsing or representation control |
| Applicability rationale | JSON is selected for the bounded native wire/configuration contract in DEC-18 and sections 5.2/6. Use typed JSON serialization; no JavaScript execution is planned. This is a current representation decision, not a conditional choice awaiting a binary format. |
| Native control mapping | CTRL-INPUT: bounded canonical decoding and validated domain types; CTRL-SERIAL: typed serializers. |
| Decision / implementation owner | Protocol/core; configuration; IPC |
| Original planned verification | Inject quotes, delimiters, control characters and nested data through the actual chosen serializer; verify independent parsed meaning. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision 1.4 review correction:** DEC-18 already selects JSON; the representation decision is answered.

**Previous revision 1.3 values retained for traceability:**

- **Previous Applicability rationale:** No JavaScript is used. This is directly applicable if JSON is chosen for config, IPC or diagnostics; a binary-only choice must be recorded and this conditional scope reassessed.
- **Previous Open question:** Confirm whether any product representation uses JSON; do not infer it from this worksheet CSV.

**Current planned verification — not executed:** CHECK-25: exercise the selected direct Serde JSON serializer/decoder with quotes, escaped keys, delimiters and control characters; independent parsed meaning must match the intended typed value. This remains planned.

### v5.0.0-1.2.4

| Field | Value |
| --- | --- |
| Source shortcode | V1.2.4 |
| Minimum assigned level | L1 |
| Chapter | Encoding and Sanitization |
| Section | Injection Prevention |
| Adapted requirement summary | Parameterize database queries and data selection expressions. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | No database query engine or ORM is planned; bounded typed configuration/state files do not execute query strings. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-1.2.5

| Field | Value |
| --- | --- |
| Source shortcode | V1.2.5 |
| Minimum assigned level | L1 |
| Chapter | Encoding and Sanitization |
| Section | Injection Prevention |
| Adapted requirement summary | Prevent OS command injection through safe invocation. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic operating-system or memory-safety control |
| Applicability rationale | The native implementation calls Windows APIs and handles adversarial protocol data. |
| Native control mapping | CTRL-FFI: small audited Windows adapter; safe Rust core; checked conversion and resource ownership. |
| Decision / implementation owner | Windows integration; protocol/core |
| Original planned verification | Verify service/install actions use structured Windows APIs; test malicious executable/path/argument strings if any process launch remains; no shell evaluation. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-1.2.6

| Field | Value |
| --- | --- |
| Source shortcode | V1.2.6 |
| Minimum assigned level | L2 |
| Chapter | Encoding and Sanitization |
| Section | Injection Prevention |
| Adapted requirement summary | Prevent LDAP query injection. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | No LDAP query generation is planned. Windows token/domain identity delegation is not an application LDAP search feature. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-1.2.7

| Field | Value |
| --- | --- |
| Source shortcode | V1.2.7 |
| Minimum assigned level | L2 |
| Chapter | Encoding and Sanitization |
| Section | Injection Prevention |
| Adapted requirement summary | Prevent XPath query injection. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | No XML/XPath query engine is planned. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-1.2.8

| Field | Value |
| --- | --- |
| Source shortcode | V1.2.8 |
| Minimum assigned level | L2 |
| Chapter | Encoding and Sanitization |
| Section | Injection Prevention |
| Adapted requirement summary | Constrain LaTeX execution and allowed commands. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | No LaTeX processing or user-authored document compilation is planned. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-1.2.9

| Field | Value |
| --- | --- |
| Source shortcode | V1.2.9 |
| Minimum assigned level | L2 |
| Chapter | Encoding and Sanitization |
| Section | Injection Prevention |
| Adapted requirement summary | Escape untrusted values embedded in regular expressions. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | No dynamically constructed regular expression is planned; fixed parsers are preferred. Reassess if input is ever interpolated into a pattern. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-1.2.10

| Field | Value |
| --- | --- |
| Source shortcode | V1.2.10 |
| Minimum assigned level | L3 |
| Chapter | Encoding and Sanitization |
| Section | Injection Prevention |
| Adapted requirement summary | Prevent spreadsheet formulas and structural injection in CSV exports. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | The product baseline has no CSV/spreadsheet export. This architecture worksheet is a documentation artifact, not an application export feature; adding one reopens the row. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-1.3.1

| Field | Value |
| --- | --- |
| Source shortcode | V1.3.1 |
| Minimum assigned level | L1 |
| Chapter | Encoding and Sanitization |
| Section | Sanitization |
| Adapted requirement summary | Sanitize untrusted rich HTML input with a maintained mechanism. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no WebView, browser frontend, DOM, browser-origin session or application HTTP server. NTS cookies are not browser cookies. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-1.3.2

| Field | Value |
| --- | --- |
| Source shortcode | V1.3.2 |
| Minimum assigned level | L1 |
| Chapter | Encoding and Sanitization |
| Section | Sanitization |
| Adapted requirement summary | Avoid evaluating untrusted executable expressions. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic input, parsing or representation control |
| Applicability rationale | The design excludes arbitrary scripts/plugins and treats configuration as data. This generic avoidance control applies directly. |
| Native control mapping | CTRL-INPUT: bounded canonical decoding and validated domain types; CTRL-SERIAL: typed serializers. |
| Decision / implementation owner | Protocol/core; configuration; IPC |
| Original planned verification | Inventory execution/loading APIs and dependencies; feed script-looking config/server strings and prove no evaluation or plugin loading occurs. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-1.3.3

| Field | Value |
| --- | --- |
| Source shortcode | V1.3.3 |
| Minimum assigned level | L2 |
| Chapter | Encoding and Sanitization |
| Section | Sanitization |
| Adapted requirement summary | Constrain data before use in a dangerous context. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic input, parsing or representation control |
| Applicability rationale | Network packets, local IPC and persisted/configuration data cross trust or representation boundaries. |
| Native control mapping | CTRL-INPUT: bounded canonical decoding and validated domain types; CTRL-SERIAL: typed serializers. |
| Decision / implementation owner | Protocol/core; configuration; IPC |
| Original planned verification | Challenge control characters, sizes and semantic ranges at clock/config/file sinks; prove valid near-neighbors work and rejected data causes no protected effect. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-1.3.4

| Field | Value |
| --- | --- |
| Source shortcode | V1.3.4 |
| Minimum assigned level | L2 |
| Chapter | Encoding and Sanitization |
| Section | Sanitization |
| Adapted requirement summary | Remove unsafe behavior from user-supplied SVG. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | No user-supplied SVG or image asset ingestion is planned; bundled UI assets are release inputs. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-1.3.5

| Field | Value |
| --- | --- |
| Source shortcode | V1.3.5 |
| Minimum assigned level | L2 |
| Chapter | Encoding and Sanitization |
| Section | Sanitization |
| Adapted requirement summary | Disable or sanitize user-controlled scriptable markup/templates. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | No user-supplied Markdown, CSS, stylesheets or expression-template content is accepted. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-1.3.6

| Field | Value |
| --- | --- |
| Source shortcode | V1.3.6 |
| Minimum assigned level | L2 |
| Chapter | Encoding and Sanitization |
| Section | Sanitization |
| Adapted requirement summary | Restrict outbound requests to approved destinations and protocols. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic service configuration requirement |
| Applicability rationale | Administrators configure NTP/NTS destinations and NTS-KE may negotiate an endpoint; compromised input must not turn the engine into an arbitrary network client. |
| Native control mapping | CTRL-EGRESS: protocol/port/address policy and validated negotiated endpoints; no generic fetch API. |
| Decision / implementation owner | Architecture; Windows integration; installer |
| Original planned verification | Test configured and negotiated host/port rules, resolution changes and disallowed destinations; preserve valid private-network NTP use only when explicitly allowed. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-1.3.7

| Field | Value |
| --- | --- |
| Source shortcode | V1.3.7 |
| Minimum assigned level | L2 |
| Chapter | Encoding and Sanitization |
| Section | Sanitization |
| Adapted requirement summary | Prevent untrusted construction of executable templates. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | The native UI uses compiled/bundled resources; no untrusted runtime templates are planned. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-1.3.8

| Field | Value |
| --- | --- |
| Source shortcode | V1.3.8 |
| Minimum assigned level | L2 |
| Chapter | Encoding and Sanitization |
| Section | Sanitization |
| Adapted requirement summary | Constrain JNDI queries and unsafe lookup behavior. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | No Java/JNDI runtime or query path is planned. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-1.3.9

| Field | Value |
| --- | --- |
| Source shortcode | V1.3.9 |
| Minimum assigned level | L2 |
| Chapter | Encoding and Sanitization |
| Section | Sanitization |
| Adapted requirement summary | Prevent protocol injection into memcache. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | No memcache service or protocol client is planned. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-1.3.10

| Field | Value |
| --- | --- |
| Source shortcode | V1.3.10 |
| Minimum assigned level | L2 |
| Chapter | Encoding and Sanitization |
| Section | Sanitization |
| Adapted requirement summary | Keep format strings controlled rather than interpreting attacker data. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic input, parsing or representation control |
| Applicability rationale | Network packets, local IPC and persisted/configuration data cross trust or representation boundaries. |
| Native control mapping | CTRL-INPUT: bounded canonical decoding and validated domain types; CTRL-SERIAL: typed serializers. |
| Decision / implementation owner | Protocol/core; configuration; IPC |
| Original planned verification | Inspect formatting macros and logging adapters; feed format-looking input and verify it remains an argument, never an executable format string. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-1.3.11

| Field | Value |
| --- | --- |
| Source shortcode | V1.3.11 |
| Minimum assigned level | L2 |
| Chapter | Encoding and Sanitization |
| Section | Sanitization |
| Adapted requirement summary | Prevent injection into SMTP or IMAP commands. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | No mail sending or mailbox integration is planned. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-1.3.12

| Field | Value |
| --- | --- |
| Source shortcode | V1.3.12 |
| Minimum assigned level | L3 |
| Chapter | Encoding and Sanitization |
| Section | Sanitization |
| Adapted requirement summary | Prevent excessive regular-expression backtracking. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | No regex engine is required by the baseline; dependency review must confirm this, or reassess every input-facing expression with work bounds. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-1.4.1

| Field | Value |
| --- | --- |
| Source shortcode | V1.4.1 |
| Minimum assigned level | L2 |
| Chapter | Encoding and Sanitization |
| Section | Memory, String, and Unmanaged Code |
| Adapted requirement summary | Use memory-safe string, buffer and pointer operations. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic operating-system or memory-safety control |
| Applicability rationale | The native implementation calls Windows APIs and handles adversarial protocol data. |
| Native control mapping | CTRL-FFI: small audited Windows adapter; safe Rust core; checked conversion and resource ownership. |
| Decision / implementation owner | Windows integration; protocol/core |
| Original planned verification | Run boundary and lifetime tests plus scoped unsafe review; execute relevant Windows paths in a disposable VM, including failure and cleanup cases. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-1.4.2

| Field | Value |
| --- | --- |
| Source shortcode | V1.4.2 |
| Minimum assigned level | L2 |
| Chapter | Encoding and Sanitization |
| Section | Memory, String, and Unmanaged Code |
| Adapted requirement summary | Check integer signs, ranges and overflow. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic operating-system or memory-safety control |
| Applicability rationale | The native implementation calls Windows APIs and handles adversarial protocol data. |
| Native control mapping | CTRL-FFI: small audited Windows adapter; safe Rust core; checked conversion and resource ownership. |
| Decision / implementation owner | Windows integration; protocol/core |
| Original planned verification | Exercise timestamp/fixed-point, length and Win32-unit conversions at min/max/negative boundaries; use independent arithmetic answers and checked conversions. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-1.4.3

| Field | Value |
| --- | --- |
| Source shortcode | V1.4.3 |
| Minimum assigned level | L2 |
| Chapter | Encoding and Sanitization |
| Section | Memory, String, and Unmanaged Code |
| Adapted requirement summary | Release allocated resources and prevent stale references. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic operating-system or memory-safety control |
| Applicability rationale | The native implementation calls Windows APIs and handles adversarial protocol data. |
| Native control mapping | CTRL-FFI: small audited Windows adapter; safe Rust core; checked conversion and resource ownership. |
| Decision / implementation owner | Windows integration; protocol/core |
| Original planned verification | Stress poll/rekey/IPC/UI lifecycle and injected failures; check handles, tasks and buffers before harness teardown; audit FFI lifetimes. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-1.5.1

| Field | Value |
| --- | --- |
| Source shortcode | V1.5.1 |
| Minimum assigned level | L1 |
| Chapter | Encoding and Sanitization |
| Section | Safe Deserialization |
| Adapted requirement summary | Disable unsafe XML parser capabilities. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | No runtime XML parsing is planned. MSI build XML is trusted build input, not an untrusted application XML boundary. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-1.5.2

| Field | Value |
| --- | --- |
| Source shortcode | V1.5.2 |
| Minimum assigned level | L2 |
| Chapter | Encoding and Sanitization |
| Section | Safe Deserialization |
| Adapted requirement summary | Restrict deserialization to safe declared data types. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic input, parsing or representation control |
| Applicability rationale | Network packets, local IPC and persisted/configuration data cross trust or representation boundaries. |
| Native control mapping | CTRL-INPUT: bounded canonical decoding and validated domain types; CTRL-SERIAL: typed serializers. |
| Decision / implementation owner | Protocol/core; configuration; IPC |
| Original planned verification | Fuzz exact raw wire/IPC/config bytes; verify bounded schemas, no polymorphic object instantiation and no effects before successful admission. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-1.5.3

| Field | Value |
| --- | --- |
| Source shortcode | V1.5.3 |
| Minimum assigned level | L3 |
| Chapter | Encoding and Sanitization |
| Section | Safe Deserialization |
| Adapted requirement summary | Make multiple parsers agree on representation and encoding. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic input, parsing or representation control |
| Applicability rationale | Network packets, local IPC and persisted/configuration data cross trust or representation boundaries. |
| Native control mapping | CTRL-INPUT: bounded canonical decoding and validated domain types; CTRL-SERIAL: typed serializers. |
| Decision / implementation owner | Protocol/core; configuration; IPC |
| Original planned verification | Use parser-differential fixtures for any shared config/IPC/path format; check normalization, duplicate fields and encoding without erasing raw anomalies. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |


## V2 - Validation and Business Logic

### v5.0.0-2.1.1

| Field | Value |
| --- | --- |
| Source shortcode | V2.1.1 |
| Minimum assigned level | L1 |
| Chapter | Validation and Business Logic |
| Section | Validation and Business Logic Documentation |
| Adapted requirement summary | Document structural validation rules for every input. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic architecture/documentation requirement |
| Applicability rationale | The product has security-sensitive inputs, effects and dependent services that require explicit policy. |
| Native control mapping | CTRL-DOC: versioned architecture, trust-boundary and policy inventories. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Review the documented inventory against every actual entry point, dependency and configuration path; reject missing or contradictory policy. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.2, THR-25:** Each input boundary needs an explicit representation and resource contract before implementation. CHECK-25/P1: freeze and review frame/file/collection/depth/error limits, object shape, duplicate/unknown-key policy, exact units, presence/null/default semantics, supported versions and failure classes; valid boundary-sized records must remain admissible. These are documentation and future implementation obligations, not an executed control.

### v5.0.0-2.1.2

| Field | Value |
| --- | --- |
| Source shortcode | V2.1.2 |
| Minimum assigned level | L2 |
| Chapter | Validation and Business Logic |
| Section | Validation and Business Logic Documentation |
| Adapted requirement summary | Document consistency rules across related values. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic architecture/documentation requirement |
| Applicability rationale | The product has security-sensitive inputs, effects and dependent services that require explicit policy. |
| Native control mapping | CTRL-DOC: versioned architecture, trust-boundary and policy inventories. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Review cross-field rules for server identity, authentication mode, samples, limits and configuration generations against consumers. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-2.1.3

| Field | Value |
| --- | --- |
| Source shortcode | V2.1.3 |
| Minimum assigned level | L2 |
| Chapter | Validation and Business Logic |
| Section | Validation and Business Logic Documentation |
| Adapted requirement summary | Document per-caller and global functional limits. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic architecture/documentation requirement |
| Applicability rationale | The product has security-sensitive inputs, effects and dependent services that require explicit policy. |
| Native control mapping | CTRL-DOC: versioned architecture, trust-boundary and policy inventories. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Verify explicit per-client and aggregate limits, owners, units and failure behavior for polling, IPC and configuration. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.1, THR-19:** Documented limits must cover the sequence and combined effect, not just individually valid fields/requests CHECK-3/10/12: repeated just-below-limit corrections, alternating signs, actual applied rate integration, relabelled stable compensation and restart must consume the same bounded aggregate authority; legitimate oscillator discipline has an independent acceptable trajectory
- **Revision 1.2, THR-25:** Each input boundary needs an explicit representation and resource contract before implementation. CHECK-25/P1: freeze and review frame/file/collection/depth/error limits, object shape, duplicate/unknown-key policy, exact units, presence/null/default semantics, supported versions and failure classes; valid boundary-sized records must remain admissible. These are documentation and future implementation obligations, not an executed control.

### v5.0.0-2.2.1

| Field | Value |
| --- | --- |
| Source shortcode | V2.2.1 |
| Minimum assigned level | L1 |
| Chapter | Validation and Business Logic |
| Section | Input Validation |
| Adapted requirement summary | Validate all inputs against declared structure and constraints. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic input, parsing or representation control |
| Applicability rationale | Network packets, local IPC and persisted/configuration data cross trust or representation boundaries. |
| Native control mapping | CTRL-INPUT: bounded canonical decoding and validated domain types; CTRL-SERIAL: typed serializers. |
| Decision / implementation owner | Protocol/core; configuration; IPC |
| Original planned verification | Exercise independent valid/invalid raw fixtures through the actual decoder and consumer; assert required meaning and absence of protected effects on rejection. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.1, THR-20:** MSI maintenance/rollback data crosses into a more privileged intermediary and can race privileged filesystem effects CHECK-13/15/23: hostile public properties, supported/unsupported transforms, rollback records and user-controlled paths through actual install/repair/remove entrypoints cannot trigger arbitrary service/file/process effects; forward and rollback use effective caller/operation authority and stable validated handles; normal complete lifecycle succeeds
- **Revision 1.1, THR-24:** Correct types and related numeric bounds must survive wire/core/IPC/backend conversions CHECK-6/7/9/10: independent era/overflow/NaN/unit/sign vectors and actual positive/negative Windows effects; validate post-conversion representability and actual acknowledgement; valid near-boundary values still work
- **Revision 1.2, THR-25:** Only the decoded and fallibly constructed value may reach a trusted handler; UI validation and generic map normalization are not that enforcement. CHECK-25 with CHECK-3/15: exercise the actual receiver with duplicate/escaped/unknown keys, nested array-for-object input, absent/null/default ambiguity, trailing documents, generic protected-field patches, integer boundaries and invalid combinations. Inspect all constructor/Deserialize/From/Default/mutation paths. Invalid input has no effect; independently valid neighboring inputs, supported migrations and exact integer round trips succeed.

### v5.0.0-2.2.2

| Field | Value |
| --- | --- |
| Source shortcode | V2.2.2 |
| Minimum assigned level | L1 |
| Chapter | Validation and Business Logic |
| Section | Input Validation |
| Adapted requirement summary | Enforce validation at the trusted service boundary. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic authorization requirement |
| Applicability rationale | Windows identity and per-action permissions protect global clock and machine configuration changes. |
| Native control mapping | CTRL-AUTHZ: trusted service checks caller token, action and fields; no authority from UI claims. |
| Decision / implementation owner | Broker; privileged helper; IPC |
| Original planned verification | Bypass GUI validation and send malformed requests to actual engine/helper/broker; verify rejection before any privileged effect. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.1, THR-20:** MSI maintenance/rollback data crosses into a more privileged intermediary and can race privileged filesystem effects CHECK-13/15/23: hostile public properties, supported/unsupported transforms, rollback records and user-controlled paths through actual install/repair/remove entrypoints cannot trigger arbitrary service/file/process effects; forward and rollback use effective caller/operation authority and stable validated handles; normal complete lifecycle succeeds
- **Revision 1.2, THR-25:** Only the decoded and fallibly constructed value may reach a trusted handler; UI validation and generic map normalization are not that enforcement. CHECK-25 with CHECK-3/15: exercise the actual receiver with duplicate/escaped/unknown keys, nested array-for-object input, absent/null/default ambiguity, trailing documents, generic protected-field patches, integer boundaries and invalid combinations. Inspect all constructor/Deserialize/From/Default/mutation paths. Invalid input has no effect; independently valid neighboring inputs, supported migrations and exact integer round trips succeed.

### v5.0.0-2.2.3

| Field | Value |
| --- | --- |
| Source shortcode | V2.2.3 |
| Minimum assigned level | L2 |
| Chapter | Validation and Business Logic |
| Section | Input Validation |
| Adapted requirement summary | Enforce logical consistency between combined fields. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic input, parsing or representation control |
| Applicability rationale | Network packets, local IPC and persisted/configuration data cross trust or representation boundaries. |
| Native control mapping | CTRL-INPUT: bounded canonical decoding and validated domain types; CTRL-SERIAL: typed serializers. |
| Decision / implementation owner | Protocol/core; configuration; IPC |
| Original planned verification | Test independently valid fields in invalid combinations, including authentication mode and peer identity; assert whole-request rejection. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.1, THR-19:** Documented limits must cover the sequence and combined effect, not just individually valid fields/requests CHECK-3/10/12: repeated just-below-limit corrections, alternating signs, actual applied rate integration, relabelled stable compensation and restart must consume the same bounded aggregate authority; legitimate oscillator discipline has an independent acceptable trajectory
- **Revision 1.1, THR-24:** Correct types and related numeric bounds must survive wire/core/IPC/backend conversions CHECK-6/7/9/10: independent era/overflow/NaN/unit/sign vectors and actual positive/negative Windows effects; validate post-conversion representability and actual acknowledgement; valid near-boundary values still work
- **Revision 1.2, THR-25:** Only the decoded and fallibly constructed value may reach a trusted handler; UI validation and generic map normalization are not that enforcement. CHECK-25 with CHECK-3/15: exercise the actual receiver with duplicate/escaped/unknown keys, nested array-for-object input, absent/null/default ambiguity, trailing documents, generic protected-field patches, integer boundaries and invalid combinations. Inspect all constructor/Deserialize/From/Default/mutation paths. Invalid input has no effect; independently valid neighboring inputs, supported migrations and exact integer round trips succeed.

### v5.0.0-2.3.1

| Field | Value |
| --- | --- |
| Source shortcode | V2.3.1 |
| Minimum assigned level | L1 |
| Chapter | Validation and Business Logic |
| Section | Business Logic Security |
| Adapted requirement summary | Prevent skipped or reordered workflow steps. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic state, business-logic or resource invariant |
| Applicability rationale | Configuration, source state and clock corrections have ordered state/effect contracts. |
| Native control mapping | CTRL-STATE: explicit state machines and generations; CTRL-CLOCK: bounded authorized clock effects. |
| Decision / implementation owner | Core/controller; configuration; broker |
| Original planned verification | Model acquire/select/authorize/adjust and config-update transitions; reject reordered and stale-generation commands while valid flows succeed. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.1, THR-23:** Install/update ordering, coherent rollback and exclusive ownership constrain which generation may perform effects CHECK-3/12/13/23: old delayed proposal and mixed-version helper/UI cannot mutate the new generation; fault/reboot at upgrade stages leaves one valid owner or explicit blocked recovery; rollback restores compatible state without replenishing consumed authority; successful permitted upgrade remains usable
- **Revision 1.2, THR-25:** A valid shape cannot skip the state machine or authorize unlimited allocation, validation work or error output. CHECK-25 with CHECK-3/21: test well-shaped out-of-order operations, oversized/growing reads, excessive collections/nesting, escaped strings and repeated invalid input. Measure bounded peak allocation/work/errors before teardown, rather than relying on post-parse length checks. Legitimate bounded sequences and maximum permitted inputs still work.

### v5.0.0-2.3.2

| Field | Value |
| --- | --- |
| Source shortcode | V2.3.2 |
| Minimum assigned level | L2 |
| Chapter | Validation and Business Logic |
| Section | Business Logic Security |
| Adapted requirement summary | Enforce the documented business limits. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic state, business-logic or resource invariant |
| Applicability rationale | Configuration, source state and clock corrections have ordered state/effect contracts. |
| Native control mapping | CTRL-STATE: explicit state machines and generations; CTRL-CLOCK: bounded authorized clock effects. |
| Decision / implementation owner | Core/controller; configuration; broker |
| Original planned verification | Use independent models, valid controls and adverse event sequences; observe real resulting state and forbidden effects. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.1, THR-19:** Documented limits must cover the sequence and combined effect, not just individually valid fields/requests CHECK-3/10/12: repeated just-below-limit corrections, alternating signs, actual applied rate integration, relabelled stable compensation and restart must consume the same bounded aggregate authority; legitimate oscillator discipline has an independent acceptable trajectory
- **Revision 1.2, THR-25:** A valid shape cannot skip the state machine or authorize unlimited allocation, validation work or error output. CHECK-25 with CHECK-3/21: test well-shaped out-of-order operations, oversized/growing reads, excessive collections/nesting, escaped strings and repeated invalid input. Measure bounded peak allocation/work/errors before teardown, rather than relying on post-parse length checks. Legitimate bounded sequences and maximum permitted inputs still work.
- **Revision 1.3, THR-11/22/26:** A valid old dependency result cannot retain authority after policy, source or clock generation changes. CHECK-2/3/11/27: delay a native result across timeout, cancellation, policy revocation, resume and service restart, then ensure no stale model update, fresh-trust status or clock proposal reaches an effect. Retain applied correction budgets across recovery and admit the fresh authorized neighboring operation.

### v5.0.0-2.3.3

| Field | Value |
| --- | --- |
| Source shortcode | V2.3.3 |
| Minimum assigned level | L2 |
| Chapter | Validation and Business Logic |
| Section | Business Logic Security |
| Adapted requirement summary | Commit operations completely or preserve prior valid state. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic state, business-logic or resource invariant |
| Applicability rationale | Atomic configuration/state updates apply directly. Clock changes are externally visible effects; their commit/unknown-outcome semantics must be defined without pretending that a compensating backward step is a rollback. |
| Native control mapping | CTRL-STATE: explicit state machines and generations; CTRL-CLOCK: bounded authorized clock effects. |
| Decision / implementation owner | Core/controller; configuration; broker |
| Original planned verification | Crash before/during/after config commit and clock command acknowledgment; inspect actual durable state and effect receipts; prevent silent partial success. |
| Open question | Define the clock operation commit point and uncertain-acknowledgment behavior. Never promise exactly-once effects or rollback without evidence. |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.1, THR-23:** Install/update ordering, coherent rollback and exclusive ownership constrain which generation may perform effects CHECK-3/12/13/23: old delayed proposal and mixed-version helper/UI cannot mutate the new generation; fault/reboot at upgrade stages leaves one valid owner or explicit blocked recovery; rollback restores compatible state without replenishing consumed authority; successful permitted upgrade remains usable

### v5.0.0-2.3.4

| Field | Value |
| --- | --- |
| Source shortcode | V2.3.4 |
| Minimum assigned level | L2 |
| Chapter | Validation and Business Logic |
| Section | Business Logic Security |
| Adapted requirement summary | Prevent concurrent claims on exclusive limited resources. |
| Proposed applicability | Native adaptation (proposed) |
| Literal standard scope | Limited-resource business locking adapted to exclusive native clock and configuration effects |
| Applicability rationale | The native analogue is exclusive clock-writer/configuration-commit ownership, not ticket or inventory booking. |
| Native control mapping | CTRL-CONCURRENCY: clock-writer ownership and serialized commits. |
| Decision / implementation owner | Core/controller; configuration; broker |
| Original planned verification | Race competing engine generations/helper commits; verify one authoritative effect owner and no double application. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.1, THR-23:** Install/update ordering, coherent rollback and exclusive ownership constrain which generation may perform effects CHECK-3/12/13/23: old delayed proposal and mixed-version helper/UI cannot mutate the new generation; fault/reboot at upgrade stages leaves one valid owner or explicit blocked recovery; rollback restores compatible state without replenishing consumed authority; successful permitted upgrade remains usable

### v5.0.0-2.3.5

| Field | Value |
| --- | --- |
| Source shortcode | V2.3.5 |
| Minimum assigned level | L3 |
| Chapter | Validation and Business Logic |
| Section | Business Logic Security |
| Adapted requirement summary | Require multiple people to approve high-value operations. |
| Proposed applicability | Needs decision |
| Literal standard scope | Literal requirement or applicability needs an explicit security decision |
| Applicability rationale | Large clock overrides and insecure bootstrap enablement may be high-impact, but the planned desktop product has one administrator and no dual-approval workflow. |
| Native control mapping | CTRL-DECISION: record applicability, proposed exception/compensations and residual risk; no compliance claim. |
| Decision / implementation owner | Security owner; architecture |
| Original planned verification | Classify high-value operations with the security owner; either implement independently identified dual approval or explicitly document the scoped exception and compensations. |
| Open question | Does the L3 multi-person requirement apply to these overrides? Single UAC consent is not two-person approval. |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-2.4.1

| Field | Value |
| --- | --- |
| Source shortcode | V2.4.1 |
| Minimum assigned level | L2 |
| Chapter | Validation and Business Logic |
| Section | Anti-automation |
| Adapted requirement summary | Limit automated calls that can exhaust or abuse resources. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic availability and resource-control requirement |
| Applicability rationale | A network-facing engine and privileged local service can be exhausted by malformed input or excessive work. |
| Native control mapping | CTRL-RESOURCE: finite buffers, tasks, handles, connection counts, rates and retries. |
| Decision / implementation owner | Engine; broker; QA |
| Original planned verification | Measure bounded local workloads and injected failure with legitimate controls; verify limits, recovery and no persistent resource growth. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.2, THR-25:** A valid shape cannot skip the state machine or authorize unlimited allocation, validation work or error output. CHECK-25 with CHECK-3/21: test well-shaped out-of-order operations, oversized/growing reads, excessive collections/nesting, escaped strings and repeated invalid input. Measure bounded peak allocation/work/errors before teardown, rather than relying on post-parse length checks. Legitimate bounded sequences and maximum permitted inputs still work.
- **Revision 1.3, THR-8/26:** A current-thread runtime can submit blocking native work whose queue and lifetime outlive caller cancellation. CHECK-27 with CHECK-8/9/21: saturate TLS/DNS/native job, byte, connection and retry budgets; keep permits with jobs until actual completion, reject/coalesce before submission and avoid replacement-pool growth. Show fair scheduling when capacity exists, independent lifecycle/broker responsiveness, diagnosed saturation and a valid request after bounded recovery.

### v5.0.0-2.4.2

| Field | Value |
| --- | --- |
| Source shortcode | V2.4.2 |
| Minimum assigned level | L3 |
| Chapter | Validation and Business Logic |
| Section | Anti-automation |
| Adapted requirement summary | Prevent implausibly fast human transaction sequences. |
| Proposed applicability | Needs decision |
| Literal standard scope | Literal requirement or applicability needs an explicit security decision |
| Applicability rationale | Automatic synchronization is intentionally machine-paced. Administrative button/IPC actions need bounded abuse controls without imposing artificial delays on assistive technology. |
| Native control mapping | CTRL-DECISION: record applicability, proposed exception/compensations and residual risk; no compliance claim. |
| Decision / implementation owner | Security owner; architecture |
| Original planned verification | Distinguish unattended protocol work from human-sensitive commands; test approved rate limits and legitimate automation/accessibility use. |
| Open question | Decide whether this literal human-timing control is applicable to any selected administrative workflow. |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |


## V3 - Web Frontend Security

### v5.0.0-3.1.1

| Field | Value |
| --- | --- |
| Source shortcode | V3.1.1 |
| Minimum assigned level | L3 |
| Chapter | Web Frontend Security |
| Section | Web Frontend Security Documentation |
| Adapted requirement summary | Document required browser security capabilities and fallback. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no WebView, browser frontend, DOM, browser-origin session or application HTTP server. NTS cookies are not browser cookies. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-3.2.1

| Field | Value |
| --- | --- |
| Source shortcode | V3.2.1 |
| Minimum assigned level | L1 |
| Chapter | Web Frontend Security |
| Section | Unintended Content Interpretation |
| Adapted requirement summary | Prevent browser rendering in an unintended response context. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no WebView, browser frontend, DOM, browser-origin session or application HTTP server. NTS cookies are not browser cookies. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-3.2.2

| Field | Value |
| --- | --- |
| Source shortcode | V3.2.2 |
| Minimum assigned level | L1 |
| Chapter | Web Frontend Security |
| Section | Unintended Content Interpretation |
| Adapted requirement summary | Render text as inert content instead of executable markup. |
| Proposed applicability | Native adaptation (proposed) |
| Literal standard scope | Web rendering requirement adapted to native text controls |
| Applicability rationale | There is no DOM or WebView; untrusted server names/errors must still remain inert data. |
| Native control mapping | CTRL-UI: plain native text and semantic controls; no markup or command evaluation. |
| Decision / implementation owner | UI |
| Original planned verification | Render hostile-looking markup/control strings and valid localized text; verify inert display, semantic accessibility and no process/network execution. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-3.2.3

| Field | Value |
| --- | --- |
| Source shortcode | V3.2.3 |
| Minimum assigned level | L3 |
| Chapter | Web Frontend Security |
| Section | Unintended Content Interpretation |
| Adapted requirement summary | Avoid JavaScript DOM clobbering and ambient globals. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no WebView, browser frontend, DOM, browser-origin session or application HTTP server. NTS cookies are not browser cookies. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-3.3.1

| Field | Value |
| --- | --- |
| Source shortcode | V3.3.1 |
| Minimum assigned level | L1 |
| Chapter | Web Frontend Security |
| Section | Cookie Setup |
| Adapted requirement summary | Set secure cookie attributes and required name prefixes. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no WebView, browser frontend, DOM, browser-origin session or application HTTP server. NTS cookies are not browser cookies. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-3.3.2

| Field | Value |
| --- | --- |
| Source shortcode | V3.3.2 |
| Minimum assigned level | L2 |
| Chapter | Web Frontend Security |
| Section | Cookie Setup |
| Adapted requirement summary | Choose cookie SameSite policy for its purpose. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no WebView, browser frontend, DOM, browser-origin session or application HTTP server. NTS cookies are not browser cookies. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-3.3.3

| Field | Value |
| --- | --- |
| Source shortcode | V3.3.3 |
| Minimum assigned level | L2 |
| Chapter | Web Frontend Security |
| Section | Cookie Setup |
| Adapted requirement summary | Use host-bound cookie names unless sharing is intended. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no WebView, browser frontend, DOM, browser-origin session or application HTTP server. NTS cookies are not browser cookies. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-3.3.4

| Field | Value |
| --- | --- |
| Source shortcode | V3.3.4 |
| Minimum assigned level | L2 |
| Chapter | Web Frontend Security |
| Section | Cookie Setup |
| Adapted requirement summary | Keep script-inaccessible cookies out of script-visible channels. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no WebView, browser frontend, DOM, browser-origin session or application HTTP server. NTS cookies are not browser cookies. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-3.3.5

| Field | Value |
| --- | --- |
| Source shortcode | V3.3.5 |
| Minimum assigned level | L3 |
| Chapter | Web Frontend Security |
| Section | Cookie Setup |
| Adapted requirement summary | Keep browser cookie names and values within supported size. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no WebView, browser frontend, DOM, browser-origin session or application HTTP server. NTS cookies are not browser cookies. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-3.4.1

| Field | Value |
| --- | --- |
| Source shortcode | V3.4.1 |
| Minimum assigned level | L1 |
| Chapter | Web Frontend Security |
| Section | Browser Security Mechanism Headers |
| Adapted requirement summary | Enforce long-lived HTTPS policy through HSTS headers. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no WebView, browser frontend, DOM, browser-origin session or application HTTP server. NTS cookies are not browser cookies. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-3.4.2

| Field | Value |
| --- | --- |
| Source shortcode | V3.4.2 |
| Minimum assigned level | L1 |
| Chapter | Web Frontend Security |
| Section | Browser Security Mechanism Headers |
| Adapted requirement summary | Restrict cross-origin HTTP access to intended origins. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no WebView, browser frontend, DOM, browser-origin session or application HTTP server. NTS cookies are not browser cookies. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-3.4.3

| Field | Value |
| --- | --- |
| Source shortcode | V3.4.3 |
| Minimum assigned level | L2 |
| Chapter | Web Frontend Security |
| Section | Browser Security Mechanism Headers |
| Adapted requirement summary | Use a restrictive CSP with stronger per-response L3 controls. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no WebView, browser frontend, DOM, browser-origin session or application HTTP server. NTS cookies are not browser cookies. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-3.4.4

| Field | Value |
| --- | --- |
| Source shortcode | V3.4.4 |
| Minimum assigned level | L2 |
| Chapter | Web Frontend Security |
| Section | Browser Security Mechanism Headers |
| Adapted requirement summary | Disable MIME sniffing on HTTP responses. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no WebView, browser frontend, DOM, browser-origin session or application HTTP server. NTS cookies are not browser cookies. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-3.4.5

| Field | Value |
| --- | --- |
| Source shortcode | V3.4.5 |
| Minimum assigned level | L2 |
| Chapter | Web Frontend Security |
| Section | Browser Security Mechanism Headers |
| Adapted requirement summary | Prevent sensitive referrer information from leaving the application. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no WebView, browser frontend, DOM, browser-origin session or application HTTP server. NTS cookies are not browser cookies. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-3.4.6

| Field | Value |
| --- | --- |
| Source shortcode | V3.4.6 |
| Minimum assigned level | L2 |
| Chapter | Web Frontend Security |
| Section | Browser Security Mechanism Headers |
| Adapted requirement summary | Restrict browser framing through policy. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no WebView, browser frontend, DOM, browser-origin session or application HTTP server. NTS cookies are not browser cookies. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-3.4.7

| Field | Value |
| --- | --- |
| Source shortcode | V3.4.7 |
| Minimum assigned level | L3 |
| Chapter | Web Frontend Security |
| Section | Browser Security Mechanism Headers |
| Adapted requirement summary | Configure collection of CSP violations. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no WebView, browser frontend, DOM, browser-origin session or application HTTP server. NTS cookies are not browser cookies. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-3.4.8

| Field | Value |
| --- | --- |
| Source shortcode | V3.4.8 |
| Minimum assigned level | L3 |
| Chapter | Web Frontend Security |
| Section | Browser Security Mechanism Headers |
| Adapted requirement summary | Isolate top-level browser contexts with opener policy. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no WebView, browser frontend, DOM, browser-origin session or application HTTP server. NTS cookies are not browser cookies. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-3.5.1

| Field | Value |
| --- | --- |
| Source shortcode | V3.5.1 |
| Minimum assigned level | L1 |
| Chapter | Web Frontend Security |
| Section | Browser Origin Separation |
| Adapted requirement summary | Defend sensitive browser actions against request forgery. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no WebView, browser frontend, DOM, browser-origin session or application HTTP server. NTS cookies are not browser cookies. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-3.5.2

| Field | Value |
| --- | --- |
| Source shortcode | V3.5.2 |
| Minimum assigned level | L1 |
| Chapter | Web Frontend Security |
| Section | Browser Origin Separation |
| Adapted requirement summary | Ensure preflight-dependent controls cannot be bypassed. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no WebView, browser frontend, DOM, browser-origin session or application HTTP server. NTS cookies are not browser cookies. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-3.5.3

| Field | Value |
| --- | --- |
| Source shortcode | V3.5.3 |
| Minimum assigned level | L1 |
| Chapter | Web Frontend Security |
| Section | Browser Origin Separation |
| Adapted requirement summary | Use appropriate HTTP methods for sensitive operations. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | HTTP/web-service-specific requirement |
| Applicability rationale | No sensitive application functionality is invoked through an HTTP endpoint. Conditional certificate/status retrieval is read-only acquisition of PKI material; it does not introduce an HTTP route for privileged clock or configuration actions. Reassess if such a route is added. |
| Native control mapping | CTRL-SCOPE: no application HTTP endpoint; protocol framing and IPC remain covered separately. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect network-entry inventory and packaged dependencies; confirm absence of application HTTP routes; reassess if update/revocation HTTP integrations are introduced. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision 1.4 review correction:** Narrow the exclusion to the actual server/sensitive-action surface; avoid a blanket assertion that every HTTP workflow is absent.

**Previous revision 1.3 values retained for traceability:**

- **Previous Applicability rationale:** The baseline exposes no HTTP API or web service and has no application HTTP workflow; NTP and NTS-KE are different protocols. Revocation-fetch design may require reassessment.

**Current planned verification — not executed:** CHECK-4 and OPEN-8: confirm the role-specific absence stated in this rationale against the admitted component and retrieval inventory. This proposed N/A does not exempt any separate client HTTP, certificate, destination or confidentiality obligation.

### v5.0.0-3.5.4

| Field | Value |
| --- | --- |
| Source shortcode | V3.5.4 |
| Minimum assigned level | L2 |
| Chapter | Web Frontend Security |
| Section | Browser Origin Separation |
| Adapted requirement summary | Separate applications by browser origin. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no WebView, browser frontend, DOM, browser-origin session or application HTTP server. NTS cookies are not browser cookies. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-3.5.5

| Field | Value |
| --- | --- |
| Source shortcode | V3.5.5 |
| Minimum assigned level | L2 |
| Chapter | Web Frontend Security |
| Section | Browser Origin Separation |
| Adapted requirement summary | Validate postMessage sender origin and message syntax. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | No browser postMessage exists. Named-pipe peer identity and schema checks are a separate native control, not evidence of browser-origin enforcement. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-3.5.6

| Field | Value |
| --- | --- |
| Source shortcode | V3.5.6 |
| Minimum assigned level | L3 |
| Chapter | Web Frontend Security |
| Section | Browser Origin Separation |
| Adapted requirement summary | Disable JSONP script inclusion behavior. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no WebView, browser frontend, DOM, browser-origin session or application HTTP server. NTS cookies are not browser cookies. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-3.5.7

| Field | Value |
| --- | --- |
| Source shortcode | V3.5.7 |
| Minimum assigned level | L3 |
| Chapter | Web Frontend Security |
| Section | Browser Origin Separation |
| Adapted requirement summary | Keep protected data out of script-resource responses. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no WebView, browser frontend, DOM, browser-origin session or application HTTP server. NTS cookies are not browser cookies. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-3.5.8

| Field | Value |
| --- | --- |
| Source shortcode | V3.5.8 |
| Minimum assigned level | L3 |
| Chapter | Web Frontend Security |
| Section | Browser Origin Separation |
| Adapted requirement summary | Restrict unintended embedding of authenticated resources. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no WebView, browser frontend, DOM, browser-origin session or application HTTP server. NTS cookies are not browser cookies. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-3.6.1

| Field | Value |
| --- | --- |
| Source shortcode | V3.6.1 |
| Minimum assigned level | L3 |
| Chapter | Web Frontend Security |
| Section | External Resource Integrity |
| Adapted requirement summary | Version and integrity-check externally hosted browser assets. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | No runtime externally hosted UI assets are planned; bundled executable/resource authenticity is covered by release controls, not SRI. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-3.7.1

| Field | Value |
| --- | --- |
| Source shortcode | V3.7.1 |
| Minimum assigned level | L2 |
| Chapter | Web Frontend Security |
| Section | Other Browser Security Considerations |
| Adapted requirement summary | Use supported secure browser technologies. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no WebView, browser frontend, DOM, browser-origin session or application HTTP server. NTS cookies are not browser cookies. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-3.7.2

| Field | Value |
| --- | --- |
| Source shortcode | V3.7.2 |
| Minimum assigned level | L2 |
| Chapter | Web Frontend Security |
| Section | Other Browser Security Considerations |
| Adapted requirement summary | Allowlist automatic external-domain redirects. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no browser-managed redirect flow. Explicit external-link or updater features require separate URI/destination review if added. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-3.7.3

| Field | Value |
| --- | --- |
| Source shortcode | V3.7.3 |
| Minimum assigned level | L3 |
| Chapter | Web Frontend Security |
| Section | Other Browser Security Considerations |
| Adapted requirement summary | Warn and allow cancellation before external navigation. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | No automatic external navigation is planned. A future native help-link behavior needs its own accessible consent/destination contract. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-3.7.4

| Field | Value |
| --- | --- |
| Source shortcode | V3.7.4 |
| Minimum assigned level | L3 |
| Chapter | Web Frontend Security |
| Section | Other Browser Security Considerations |
| Adapted requirement summary | Preload the web domain for HSTS. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no WebView, browser frontend, DOM, browser-origin session or application HTTP server. NTS cookies are not browser cookies. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-3.7.5

| Field | Value |
| --- | --- |
| Source shortcode | V3.7.5 |
| Minimum assigned level | L3 |
| Chapter | Web Frontend Security |
| Section | Other Browser Security Considerations |
| Adapted requirement summary | Apply documented fallback when browser safeguards are missing. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no WebView, browser frontend, DOM, browser-origin session or application HTTP server. NTS cookies are not browser cookies. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |


## V4 - API and Web Service

### v5.0.0-4.1.1

| Field | Value |
| --- | --- |
| Source shortcode | V4.1.1 |
| Minimum assigned level | L1 |
| Chapter | API and Web Service |
| Section | Generic Web Service Security |
| Adapted requirement summary | Set HTTP response media type and safe charset correctly. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | HTTP/web-service-specific requirement |
| Applicability rationale | The product generates no HTTP server responses with message bodies. A native verifier receiving a certificate/status response is a client role, not an application HTTP response producer. Incoming data and native retrieval remain subject to their own parsing and OPEN-8 policy gates. |
| Native control mapping | CTRL-SCOPE: no application HTTP endpoint; protocol framing and IPC remain covered separately. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect network-entry inventory and packaged dependencies; confirm absence of application HTTP routes; reassess if update/revocation HTTP integrations are introduced. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision 1.4 review correction:** Narrow the exclusion to the actual server/sensitive-action surface; avoid a blanket assertion that every HTTP workflow is absent.

**Previous revision 1.3 values retained for traceability:**

- **Previous Applicability rationale:** The baseline exposes no HTTP API or web service and has no application HTTP workflow; NTP and NTS-KE are different protocols. Revocation-fetch design may require reassessment.

**Current planned verification — not executed:** CHECK-4 and OPEN-8: confirm the role-specific absence stated in this rationale against the admitted component and retrieval inventory. This proposed N/A does not exempt any separate client HTTP, certificate, destination or confidentiality obligation.

### v5.0.0-4.1.2

| Field | Value |
| --- | --- |
| Source shortcode | V4.1.2 |
| Minimum assigned level | L2 |
| Chapter | API and Web Service |
| Section | Generic Web Service Security |
| Adapted requirement summary | Avoid transparent insecure redirects for machine HTTP clients. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | HTTP/web-service-specific requirement |
| Applicability rationale | The product exposes no user-facing or service HTTP endpoint implementing HTTP-to-HTTPS redirects. A native retrieval client following an external redirect is assessed separately under V15.3.2 and OPEN-8; it does not create this server endpoint behavior. |
| Native control mapping | CTRL-SCOPE: no application HTTP endpoint; protocol framing and IPC remain covered separately. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect network-entry inventory and packaged dependencies; confirm absence of application HTTP routes; reassess if update/revocation HTTP integrations are introduced. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision 1.4 review correction:** Narrow the exclusion to the actual server/sensitive-action surface; avoid a blanket assertion that every HTTP workflow is absent.

**Previous revision 1.3 values retained for traceability:**

- **Previous Applicability rationale:** The baseline exposes no HTTP API or web service and has no application HTTP workflow; NTP and NTS-KE are different protocols. Revocation-fetch design may require reassessment.

**Current planned verification — not executed:** CHECK-4 and OPEN-8: confirm the role-specific absence stated in this rationale against the admitted component and retrieval inventory. This proposed N/A does not exempt any separate client HTTP, certificate, destination or confidentiality obligation.

### v5.0.0-4.1.3

| Field | Value |
| --- | --- |
| Source shortcode | V4.1.3 |
| Minimum assigned level | L2 |
| Chapter | API and Web Service |
| Section | Generic Web Service Security |
| Adapted requirement summary | Protect trusted intermediary HTTP headers from client override. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | HTTP/web-service-specific requirement |
| Applicability rationale | No proxy-provided HTTP identity headers exist; named-pipe caller tokens must be obtained from Windows rather than claimed message fields. |
| Native control mapping | CTRL-SCOPE: no application HTTP endpoint; protocol framing and IPC remain covered separately. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect network-entry inventory and packaged dependencies; confirm absence of application HTTP routes; reassess if update/revocation HTTP integrations are introduced. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-4.1.4

| Field | Value |
| --- | --- |
| Source shortcode | V4.1.4 |
| Minimum assigned level | L3 |
| Chapter | API and Web Service |
| Section | Generic Web Service Security |
| Adapted requirement summary | Allow only explicitly supported HTTP methods. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | HTTP/web-service-specific requirement |
| Applicability rationale | No application HTTP API accepts methods such as GET, POST or OPTIONS. Conditional native certificate retrieval is an outbound client operation. Reassess this server/API method policy if an inbound HTTP interface is introduced. |
| Native control mapping | CTRL-SCOPE: no application HTTP endpoint; protocol framing and IPC remain covered separately. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect network-entry inventory and packaged dependencies; confirm absence of application HTTP routes; reassess if update/revocation HTTP integrations are introduced. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision 1.4 review correction:** Narrow the exclusion to the actual server/sensitive-action surface; avoid a blanket assertion that every HTTP workflow is absent.

**Previous revision 1.3 values retained for traceability:**

- **Previous Applicability rationale:** The baseline exposes no HTTP API or web service and has no application HTTP workflow; NTP and NTS-KE are different protocols. Revocation-fetch design may require reassessment.

**Current planned verification — not executed:** CHECK-4 and OPEN-8: confirm the role-specific absence stated in this rationale against the admitted component and retrieval inventory. This proposed N/A does not exempt any separate client HTTP, certificate, destination or confidentiality obligation.

### v5.0.0-4.1.5

| Field | Value |
| --- | --- |
| Source shortcode | V4.1.5 |
| Minimum assigned level | L3 |
| Chapter | API and Web Service |
| Section | Generic Web Service Security |
| Adapted requirement summary | Sign especially sensitive messages beyond transport protection. |
| Proposed applicability | Needs decision |
| Literal standard scope | Literal requirement or applicability needs an explicit security decision |
| Applicability rationale | Clock commands are sensitive, but the design uses verified local pipe peers and bounded commands, not per-message digital signatures. NTS AEAD is not a digital signature. |
| Native control mapping | CTRL-DECISION: record applicability, proposed exception/compensations and residual risk; no compliance claim. |
| Decision / implementation owner | Security owner; architecture |
| Original planned verification | Threat-model message paths and replay; decide literal applicability/signature requirement or record a scoped native-channel exception with compensations. |
| Open question | Do sensitive local broker commands require independent message signatures under the chosen L3 target? |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-4.2.1

| Field | Value |
| --- | --- |
| Source shortcode | V4.2.1 |
| Minimum assigned level | L2 |
| Chapter | API and Web Service |
| Section | HTTP Message Structure Validation |
| Adapted requirement summary | Parse HTTP message boundaries consistently across components. |
| Proposed applicability | Needs decision |
| Literal standard scope | HTTP/web-service-specific requirement |
| Applicability rationale | The held Windows verifier may receive HTTP certificate/status messages directly or through Windows services. The literal clause covers incoming messages, including a possible client response path; it cannot be excluded solely because no application HTTP server exists. OPEN-8 must identify the in-scope HTTP parser, negotiated versions and delegated responsibility. |
| Native control mapping | CTRL-DECISION, CTRL-CERT, CTRL-EGRESS: explicit native/delegated retrieval scope and policy; CTRL-INPUT/CTRL-RESOURCE where this clause requires framing or length enforcement. No implemented control or platform guarantee is inferred. |
| Decision / implementation owner | Security; Windows platform; protocol; QA |
| Original planned verification | Inspect network-entry inventory and packaged dependencies; confirm absence of application HTTP routes; reassess if update/revocation HTTP integrations are introduced. |
| Open question | Which component receives and frames retrieval messages, what HTTP versions can it use, and what evidence establishes version-correct message boundaries or a documented scope exclusion? |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision 1.4 review correction:** Reconcile a current held direct/delegated certificate-retrieval candidate with the exact clause; no runtime failure is asserted.

**Previous revision 1.3 values retained for traceability:**

- **Previous Proposed applicability:** Proposed N/A
- **Previous Applicability rationale:** The baseline exposes no HTTP API or web service and has no application HTTP workflow; NTP and NTS-KE are different protocols. Revocation-fetch design may require reassessment.
- **Previous Native control mapping:** CTRL-SCOPE: no application HTTP endpoint; protocol framing and IPC remain covered separately.
- **Previous Decision / implementation owner:** Architecture; QA
- **Previous Open question:** None recorded

**Current planned verification — not executed:** OPEN-8/S0.7, CHECK-4/9/27: For the admitted route, qualify incoming message boundaries for each supported HTTP version, including conflicting length/framing cases and an accepted valid neighboring response. Record which Windows/provider component owns enforcement; do not substitute NTP packet bounds for HTTP evidence.

### v5.0.0-4.2.2

| Field | Value |
| --- | --- |
| Source shortcode | V4.2.2 |
| Minimum assigned level | L3 |
| Chapter | API and Web Service |
| Section | HTTP Message Structure Validation |
| Adapted requirement summary | Keep declared HTTP lengths consistent with framing. |
| Proposed applicability | Needs decision |
| Literal standard scope | HTTP/web-service-specific requirement |
| Applicability rationale | Conditional native certificate/status retrieval may generate HTTP requests. Whether the product, selected dependency or delegated platform component generates each message and controls its framing is unresolved under OPEN-8. The absence of an HTTP server does not establish absence of generated client messages. |
| Native control mapping | CTRL-DECISION, CTRL-CERT, CTRL-EGRESS: explicit native/delegated retrieval scope and policy; CTRL-INPUT/CTRL-RESOURCE where this clause requires framing or length enforcement. No implemented control or platform guarantee is inferred. |
| Decision / implementation owner | Security; Windows platform; protocol; QA |
| Original planned verification | Inspect network-entry inventory and packaged dependencies; confirm absence of application HTTP routes; reassess if update/revocation HTTP integrations are introduced. |
| Open question | Which component generates each retrieval request, and how are Content-Length and actual protocol framing kept consistent for the selected HTTP versions? |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision 1.4 review correction:** Reconcile a current held direct/delegated certificate-retrieval candidate with the exact clause; no runtime failure is asserted.

**Previous revision 1.3 values retained for traceability:**

- **Previous Proposed applicability:** Proposed N/A
- **Previous Applicability rationale:** The baseline exposes no HTTP API or web service and has no application HTTP workflow; NTP and NTS-KE are different protocols. Revocation-fetch design may require reassessment.
- **Previous Native control mapping:** CTRL-SCOPE: no application HTTP endpoint; protocol framing and IPC remain covered separately.
- **Previous Decision / implementation owner:** Architecture; QA
- **Previous Open question:** None recorded

**Current planned verification — not executed:** OPEN-8/S0.7, CHECK-4/9/27: Qualify the admitted request generator with correct-length and conflicting-length/framing cases, or retain explicit supported platform evidence and its limits. Do not claim the application constructs HTTP headers unless the route inventory demonstrates it.

### v5.0.0-4.2.3

| Field | Value |
| --- | --- |
| Source shortcode | V4.2.3 |
| Minimum assigned level | L3 |
| Chapter | API and Web Service |
| Section | HTTP Message Structure Validation |
| Adapted requirement summary | Exclude connection-specific fields from newer HTTP versions. |
| Proposed applicability | Needs decision |
| Literal standard scope | HTTP/web-service-specific requirement |
| Applicability rationale | The held retrieval route may send or accept HTTP/2 or HTTP/3 messages; actual version negotiation and platform ownership are unqualified. The literal send/accept obligation can concern a client stack. NTP and NTS-KE are separate protocols. |
| Native control mapping | CTRL-DECISION, CTRL-CERT, CTRL-EGRESS: explicit native/delegated retrieval scope and policy; CTRL-INPUT/CTRL-RESOURCE where this clause requires framing or length enforcement. No implemented control or platform guarantee is inferred. |
| Decision / implementation owner | Security; Windows platform; protocol; QA |
| Original planned verification | Inspect network-entry inventory and packaged dependencies; confirm absence of application HTTP routes; reassess if update/revocation HTTP integrations are introduced. |
| Open question | Does the admitted direct/delegated retrieval route support HTTP/2 or HTTP/3, and who prevents connection-specific fields on both sending and receiving paths? |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision 1.4 review correction:** Reconcile a current held direct/delegated certificate-retrieval candidate with the exact clause; no runtime failure is asserted.

**Previous revision 1.3 values retained for traceability:**

- **Previous Proposed applicability:** Proposed N/A
- **Previous Applicability rationale:** The baseline exposes no HTTP API or web service and has no application HTTP workflow; NTP and NTS-KE are different protocols. Revocation-fetch design may require reassessment.
- **Previous Native control mapping:** CTRL-SCOPE: no application HTTP endpoint; protocol framing and IPC remain covered separately.
- **Previous Decision / implementation owner:** Architecture; QA
- **Previous Open question:** None recorded

**Current planned verification — not executed:** OPEN-8/S0.7, CHECK-4/9/27: For supported HTTP/2/3 retrieval, establish rejection or prevention of forbidden connection-specific fields in both directions and successful valid traffic. If the qualified route cannot use either version, record that evidenced scope instead of inventing an HTTP implementation.

### v5.0.0-4.2.4

| Field | Value |
| --- | --- |
| Source shortcode | V4.2.4 |
| Minimum assigned level | L3 |
| Chapter | API and Web Service |
| Section | HTTP Message Structure Validation |
| Adapted requirement summary | Reject newline injection in newer HTTP header fields. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | HTTP/web-service-specific requirement |
| Applicability rationale | No application component accepts inbound HTTP/2 or HTTP/3 requests. Conditional native retrieval may receive responses, whose framing and header responsibilities require a separate OPEN-8 decision under V4.2.1/V4.2.3; it does not introduce the request-acceptance surface specified here. |
| Native control mapping | CTRL-SCOPE: no application HTTP endpoint; protocol framing and IPC remain covered separately. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect network-entry inventory and packaged dependencies; confirm absence of application HTTP routes; reassess if update/revocation HTTP integrations are introduced. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision 1.4 review correction:** Narrow the exclusion to the actual server/sensitive-action surface; avoid a blanket assertion that every HTTP workflow is absent.

**Previous revision 1.3 values retained for traceability:**

- **Previous Applicability rationale:** The baseline exposes no HTTP API or web service and has no application HTTP workflow; NTP and NTS-KE are different protocols. Revocation-fetch design may require reassessment.

**Current planned verification — not executed:** CHECK-4 and OPEN-8: confirm the role-specific absence stated in this rationale against the admitted component and retrieval inventory. This proposed N/A does not exempt any separate client HTTP, certificate, destination or confidentiality obligation.

### v5.0.0-4.2.5

| Field | Value |
| --- | --- |
| Source shortcode | V4.2.5 |
| Minimum assigned level | L3 |
| Chapter | API and Web Service |
| Section | HTTP Message Structure Validation |
| Adapted requirement summary | Bound outbound URI and header lengths to receiver limits. |
| Proposed applicability | Needs decision |
| Literal standard scope | HTTP/web-service-specific requirement |
| Applicability rationale | The held Windows certificate-verifier route may create outbound HTTP requests from certificate/status URLs, directly or through OS services. Actual request construction and URI/header length limits are not qualified. OPEN-8 must decide the assessment boundary and enforcement owner; absence of an application HTTP server is insufficient. |
| Native control mapping | CTRL-DECISION, CTRL-CERT, CTRL-EGRESS: explicit native/delegated retrieval scope and policy; CTRL-INPUT/CTRL-RESOURCE where this clause requires framing or length enforcement. No implemented control or platform guarantee is inferred. |
| Decision / implementation owner | Security; Windows platform; protocol; QA |
| Original planned verification | Inspect network-entry inventory and packaged dependencies; confirm absence of application HTTP routes; reassess if update/revocation HTTP integrations are introduced. |
| Open question | Which admitted component constructs retrieval URIs/header fields, what receiver-compatible limits apply, and how are overlong requests prevented or safely bounded without persistent retry failure? |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision 1.4 review correction:** Reconcile a current held direct/delegated certificate-retrieval candidate with the exact clause; no runtime failure is asserted.

**Previous revision 1.3 values retained for traceability:**

- **Previous Proposed applicability:** Proposed N/A
- **Previous Applicability rationale:** No application URI/header construction exists in the baseline. NTP/NTS and pipe framing have independent size limits; future revocation/update HTTP code reopens this row.
- **Previous Native control mapping:** CTRL-SCOPE: no application HTTP endpoint; protocol framing and IPC remain covered separately.
- **Previous Decision / implementation owner:** Architecture; QA
- **Previous Open question:** None recorded

**Current planned verification — not executed:** OPEN-8/S0.7, CHECK-4/9/27: Qualify the real retrieval route with supported and overlong URI/header inputs, bounded errors/retries and an accepted valid neighbor. Record receiver-limit assumptions and platform/provider evidence. Generic NTP/IPC frame bounds do not prove this clause.

### v5.0.0-4.3.1

| Field | Value |
| --- | --- |
| Source shortcode | V4.3.1 |
| Minimum assigned level | L2 |
| Chapter | API and Web Service |
| Section | GraphQL |
| Adapted requirement summary | Bound GraphQL query complexity and cost. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | No GraphQL/data-expression service is planned. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-4.3.2

| Field | Value |
| --- | --- |
| Source shortcode | V4.3.2 |
| Minimum assigned level | L2 |
| Chapter | API and Web Service |
| Section | GraphQL |
| Adapted requirement summary | Restrict GraphQL introspection in production. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | No GraphQL endpoint is planned. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-4.4.1

| Field | Value |
| --- | --- |
| Source shortcode | V4.4.1 |
| Minimum assigned level | L1 |
| Chapter | API and Web Service |
| Section | WebSocket |
| Adapted requirement summary | Protect WebSocket traffic with TLS. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | No WebSocket connection or endpoint is planned. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-4.4.2

| Field | Value |
| --- | --- |
| Source shortcode | V4.4.2 |
| Minimum assigned level | L2 |
| Chapter | API and Web Service |
| Section | WebSocket |
| Adapted requirement summary | Allowlist browser origins during WebSocket setup. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | No WebSocket handshake is planned. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-4.4.3

| Field | Value |
| --- | --- |
| Source shortcode | V4.4.3 |
| Minimum assigned level | L2 |
| Chapter | API and Web Service |
| Section | WebSocket |
| Adapted requirement summary | Apply session safeguards to dedicated WebSocket tokens. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | No WebSocket or WebSocket-specific session tokens are planned. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-4.4.4

| Field | Value |
| --- | --- |
| Source shortcode | V4.4.4 |
| Minimum assigned level | L2 |
| Chapter | API and Web Service |
| Section | WebSocket |
| Adapted requirement summary | Bind WebSocket authorization to an existing secure session. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | No HTTPS-to-WebSocket transition exists. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |


## V5 - File Handling

### v5.0.0-5.1.1

| Field | Value |
| --- | --- |
| Source shortcode | V5.1.1 |
| Minimum assigned level | L2 |
| Chapter | File Handling |
| Section | File Handling Documentation |
| Adapted requirement summary | Document accepted file formats, bounds and malicious-content handling. |
| Proposed applicability | Native adaptation (proposed) |
| Literal standard scope | File-upload/storage requirement adapted to local accepted files |
| Applicability rationale | There is no upload/download service. Apply the documentation purpose to local configuration/state and diagnostic generation; no archive or executable import is planned. |
| Native control mapping | CTRL-FILE: documented formats and bounds; CTRL-PATH; atomic state replacement. |
| Decision / implementation owner | Configuration/storage; installer |
| Original planned verification | Review each actual file entry/output path against format, extension, size, privilege and failure policy; confirm absent upload/distribution features. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-5.2.1

| Field | Value |
| --- | --- |
| Source shortcode | V5.2.1 |
| Minimum assigned level | L1 |
| Chapter | File Handling |
| Section | File Upload and Content |
| Adapted requirement summary | Accept files only within safely processable size limits. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic accepted-file validation requirement |
| Applicability rationale | Local configuration and state files are accepted data even without an upload service. |
| Native control mapping | CTRL-FILE: documented accepted formats, extensions and finite sizes; CTRL-PATH; atomic replacement. |
| Decision / implementation owner | Configuration/storage |
| Original planned verification | Exercise actual readers with independently constructed valid, malformed, mismatched-type and oversized files; verify resource bounds and preservation of last-good state. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-5.2.2

| Field | Value |
| --- | --- |
| Source shortcode | V5.2.2 |
| Minimum assigned level | L1 |
| Chapter | File Handling |
| Section | File Upload and Content |
| Adapted requirement summary | Check accepted file extensions and actual content format. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic accepted-file validation requirement |
| Applicability rationale | Config/state readers accept only documented file forms and validate content independently of filename. The L2/L3 scope includes every accepted file, not only security-decision files. |
| Native control mapping | CTRL-FILE: documented accepted formats, extensions and finite sizes; CTRL-PATH; atomic replacement. |
| Decision / implementation owner | Configuration/storage |
| Original planned verification | Submit valid files, correct-extension invalid contents, wrong extensions, truncation and oversized nesting to every accepted-file reader; verify policy for fixed filenames and no content sniffing into executable formats. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-5.2.3

| Field | Value |
| --- | --- |
| Source shortcode | V5.2.3 |
| Minimum assigned level | L2 |
| Chapter | File Handling |
| Section | File Upload and Content |
| Adapted requirement summary | Bound archive expansion size and file count before decompression. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | No compressed archive import or decompression of untrusted user files is planned. Installer packaging is a separate signed supply-chain boundary; adding archive ingestion reopens this row. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-5.2.4

| Field | Value |
| --- | --- |
| Source shortcode | V5.2.4 |
| Minimum assigned level | L3 |
| Chapter | File Handling |
| Section | File Upload and Content |
| Adapted requirement summary | Limit each user's accepted file count and storage consumption. |
| Proposed applicability | Native adaptation (proposed) |
| Literal standard scope | File-upload/storage requirement adapted to local accepted files |
| Applicability rationale | No arbitrary user file store exists. Native adaptation bounds each admitted configuration/diagnostic request and total app-owned state/log output; it does not impose quotas on all user-owned disk files. |
| Native control mapping | CTRL-FILE: documented formats and bounds; CTRL-PATH; atomic state replacement. |
| Decision / implementation owner | Configuration/storage; installer |
| Original planned verification | Flood file-producing requests by one permitted principal with legitimate concurrent controls; observe per-principal rate, total app storage bounds and cleanup, including full-disk behavior. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-5.2.5

| Field | Value |
| --- | --- |
| Source shortcode | V5.2.5 |
| Minimum assigned level | L3 |
| Chapter | File Handling |
| Section | File Upload and Content |
| Adapted requirement summary | Reject uploaded archive symlinks unless specifically allowlisted. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | No uploaded archive or archive extraction feature is planned. Windows reparse-point safety for app-owned paths is separately required by V5.3.2. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-5.2.6

| Field | Value |
| --- | --- |
| Source shortcode | V5.2.6 |
| Minimum assigned level | L3 |
| Chapter | File Handling |
| Section | File Upload and Content |
| Adapted requirement summary | Reject uploaded images exceeding allowed pixel dimensions. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | No image upload or untrusted image decoding is planned. Packaged static UI resources are reviewed release inputs. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-5.3.1

| Field | Value |
| --- | --- |
| Source shortcode | V5.3.1 |
| Minimum assigned level | L1 |
| Chapter | File Handling |
| Section | File Storage |
| Adapted requirement summary | Prevent public uploaded files from executing through HTTP. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | No public upload directory, HTTP server or server-side code execution from served files exists. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-5.3.2

| Field | Value |
| --- | --- |
| Source shortcode | V5.3.2 |
| Minimum assigned level | L1 |
| Chapter | File Handling |
| Section | File Storage |
| Adapted requirement summary | Build file paths from trusted data or strictly validate names. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic file-path safety requirement |
| Applicability rationale | Privileged components read or write configuration, state and logs. |
| Native control mapping | CTRL-PATH: fixed trusted roots; handle-based path validation; reject reparse/UNC/alternate-stream surprises where unsupported. |
| Decision / implementation owner | Windows integration; configuration/storage |
| Original planned verification | Exercise path traversal, reparse replacement, access denial and valid app-owned paths; observe exact file identities and no unauthorized writes. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-5.3.3

| Field | Value |
| --- | --- |
| Source shortcode | V5.3.3 |
| Minimum assigned level | L3 |
| Chapter | File Handling |
| Section | File Storage |
| Adapted requirement summary | Ignore untrusted archive paths during server-side processing. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | No untrusted archive extraction or file-processing server is planned; ordinary local file path safety remains applicable separately. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-5.4.1

| Field | Value |
| --- | --- |
| Source shortcode | V5.4.1 |
| Minimum assigned level | L2 |
| Chapter | File Handling |
| Section | File Download |
| Adapted requirement summary | Control download filenames and HTTP content disposition. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | HTTP/web-service-specific requirement |
| Applicability rationale | The app serves no HTTP downloads or Content-Disposition response. A local diagnostic save dialog does not create this HTTP feature; its native path rules remain applicable. |
| Native control mapping | CTRL-SCOPE: no application HTTP endpoint; protocol framing and IPC remain covered separately. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect network-entry inventory and packaged dependencies; confirm absence of application HTTP routes; reassess if update/revocation HTTP integrations are introduced. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-5.4.2

| Field | Value |
| --- | --- |
| Source shortcode | V5.4.2 |
| Minimum assigned level | L2 |
| Chapter | File Handling |
| Section | File Download |
| Adapted requirement summary | Encode filenames safely when serving them in messages. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | No HTTP file-serving or email-attachment feature is planned. Local display of filenames uses inert native text controls. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-5.4.3

| Field | Value |
| --- | --- |
| Source shortcode | V5.4.3 |
| Minimum assigned level | L2 |
| Chapter | File Handling |
| Section | File Download |
| Adapted requirement summary | Scan untrusted files before serving them to users. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | The product does not accept and redistribute untrusted files. Its signed release artifacts have a separate build/release assurance process; archive/file sharing would reopen this row. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |


## V6 - Authentication

### v5.0.0-6.1.1

| Field | Value |
| --- | --- |
| Source shortcode | V6.1.1 |
| Minimum assigned level | L1 |
| Chapter | Authentication |
| Section | Authentication Documentation |
| Adapted requirement summary | Document defenses against credential stuffing and password guessing. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application password/account credential requirement |
| Applicability rationale | The app has no password authentication or application account lockout. IPC abuse bounds remain required, but are not relabeled as password-guessing defenses. |
| Native control mapping | CTRL-SCOPE: no application credential UI/store/recovery; CTRL-IPC: Windows identity. |
| Decision / implementation owner | Architecture; Windows integration |
| Original planned verification | Inspect UI, IPC schemas, persistence and installer for password/account pathways; separately verify Windows peer authentication and authorization. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.1.2

| Field | Value |
| --- | --- |
| Source shortcode | V6.1.2 |
| Minimum assigned level | L2 |
| Chapter | Authentication |
| Section | Authentication Documentation |
| Adapted requirement summary | Document contextual words forbidden in passwords. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application password/account credential requirement |
| Applicability rationale | The app neither creates user accounts nor accepts, stores, changes or recovers passwords; Windows owns interactive identity. This does not waive generic identity/authorization rows. |
| Native control mapping | CTRL-SCOPE: no application credential UI/store/recovery; CTRL-IPC: Windows identity. |
| Decision / implementation owner | Architecture; Windows integration |
| Original planned verification | Inspect UI, IPC schemas, persistence and installer for password/account pathways; separately verify Windows peer authentication and authorization. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.1.3

| Field | Value |
| --- | --- |
| Source shortcode | V6.1.3 |
| Minimum assigned level | L2 |
| Chapter | Authentication |
| Section | Authentication Documentation |
| Adapted requirement summary | Document all authentication paths and their consistent assurance. |
| Proposed applicability | Native adaptation (proposed) |
| Literal standard scope | Authentication/session requirement adapted to local Windows identities |
| Applicability rationale | Document UI-to-broker status reads, engine-to-broker snapshot publication, helper/configuration elevation, engine-to-broker machine identity and TLS server identity separately. No path may gain authority merely because another path is stronger. This does not add a direct UI-to-engine interface. |
| Native control mapping | CTRL-IPC: secured local named pipes and verified peer token; CTRL-AUTHZ: per-action authorization. |
| Decision / implementation owner | Windows integration; security owner |
| Original planned verification | Review each real entry point and service identity against the authentication matrix; attempt alternate-path bypass and record the assurance actually available from Windows. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision 1.4 review correction:** Align the authentication inventory with the existing broker-mediated status design.

**Previous revision 1.3 values retained for traceability:**

- **Previous Applicability rationale:** Document UI-to-engine status, helper/configuration elevation, engine-to-broker machine identity and TLS server identity separately. No path may gain authority merely because another path is stronger.

**Current planned verification — not executed:** CHECK-2/4/14: trace ordinary UI-to-broker status reads, engine-to-broker snapshot publication, elevated helper/configuration actions and TLS server identity through their separate actual authority boundaries.

### v5.0.0-6.2.1

| Field | Value |
| --- | --- |
| Source shortcode | V6.2.1 |
| Minimum assigned level | L1 |
| Chapter | Authentication |
| Section | Password Security |
| Adapted requirement summary | Enforce the minimum password length and stronger recommendation. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application password/account credential requirement |
| Applicability rationale | The app neither creates user accounts nor accepts, stores, changes or recovers passwords; Windows owns interactive identity. This does not waive generic identity/authorization rows. |
| Native control mapping | CTRL-SCOPE: no application credential UI/store/recovery; CTRL-IPC: Windows identity. |
| Decision / implementation owner | Architecture; Windows integration |
| Original planned verification | Inspect UI, IPC schemas, persistence and installer for password/account pathways; separately verify Windows peer authentication and authorization. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.2.2

| Field | Value |
| --- | --- |
| Source shortcode | V6.2.2 |
| Minimum assigned level | L1 |
| Chapter | Authentication |
| Section | Password Security |
| Adapted requirement summary | Allow users to change passwords. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application password/account credential requirement |
| Applicability rationale | The app neither creates user accounts nor accepts, stores, changes or recovers passwords; Windows owns interactive identity. This does not waive generic identity/authorization rows. |
| Native control mapping | CTRL-SCOPE: no application credential UI/store/recovery; CTRL-IPC: Windows identity. |
| Decision / implementation owner | Architecture; Windows integration |
| Original planned verification | Inspect UI, IPC schemas, persistence and installer for password/account pathways; separately verify Windows peer authentication and authorization. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.2.3

| Field | Value |
| --- | --- |
| Source shortcode | V6.2.3 |
| Minimum assigned level | L1 |
| Chapter | Authentication |
| Section | Password Security |
| Adapted requirement summary | Require current and new passwords for password changes. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application password/account credential requirement |
| Applicability rationale | The app neither creates user accounts nor accepts, stores, changes or recovers passwords; Windows owns interactive identity. This does not waive generic identity/authorization rows. |
| Native control mapping | CTRL-SCOPE: no application credential UI/store/recovery; CTRL-IPC: Windows identity. |
| Decision / implementation owner | Architecture; Windows integration |
| Original planned verification | Inspect UI, IPC schemas, persistence and installer for password/account pathways; separately verify Windows peer authentication and authorization. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.2.4

| Field | Value |
| --- | --- |
| Source shortcode | V6.2.4 |
| Minimum assigned level | L1 |
| Chapter | Authentication |
| Section | Password Security |
| Adapted requirement summary | Reject commonly used passwords during setup or changes. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application password/account credential requirement |
| Applicability rationale | The app neither creates user accounts nor accepts, stores, changes or recovers passwords; Windows owns interactive identity. This does not waive generic identity/authorization rows. |
| Native control mapping | CTRL-SCOPE: no application credential UI/store/recovery; CTRL-IPC: Windows identity. |
| Decision / implementation owner | Architecture; Windows integration |
| Original planned verification | Inspect UI, IPC schemas, persistence and installer for password/account pathways; separately verify Windows peer authentication and authorization. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.2.5

| Field | Value |
| --- | --- |
| Source shortcode | V6.2.5 |
| Minimum assigned level | L1 |
| Chapter | Authentication |
| Section | Password Security |
| Adapted requirement summary | Allow password composition without arbitrary character rules. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application password/account credential requirement |
| Applicability rationale | The app neither creates user accounts nor accepts, stores, changes or recovers passwords; Windows owns interactive identity. This does not waive generic identity/authorization rows. |
| Native control mapping | CTRL-SCOPE: no application credential UI/store/recovery; CTRL-IPC: Windows identity. |
| Decision / implementation owner | Architecture; Windows integration |
| Original planned verification | Inspect UI, IPC schemas, persistence and installer for password/account pathways; separately verify Windows peer authentication and authorization. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.2.6

| Field | Value |
| --- | --- |
| Source shortcode | V6.2.6 |
| Minimum assigned level | L1 |
| Chapter | Authentication |
| Section | Password Security |
| Adapted requirement summary | Mask password entry with an optional reveal action. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application password/account credential requirement |
| Applicability rationale | The app neither creates user accounts nor accepts, stores, changes or recovers passwords; Windows owns interactive identity. This does not waive generic identity/authorization rows. |
| Native control mapping | CTRL-SCOPE: no application credential UI/store/recovery; CTRL-IPC: Windows identity. |
| Decision / implementation owner | Architecture; Windows integration |
| Original planned verification | Inspect UI, IPC schemas, persistence and installer for password/account pathways; separately verify Windows peer authentication and authorization. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.2.7

| Field | Value |
| --- | --- |
| Source shortcode | V6.2.7 |
| Minimum assigned level | L1 |
| Chapter | Authentication |
| Section | Password Security |
| Adapted requirement summary | Permit password paste and password-manager assistance. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application password/account credential requirement |
| Applicability rationale | The app neither creates user accounts nor accepts, stores, changes or recovers passwords; Windows owns interactive identity. This does not waive generic identity/authorization rows. |
| Native control mapping | CTRL-SCOPE: no application credential UI/store/recovery; CTRL-IPC: Windows identity. |
| Decision / implementation owner | Architecture; Windows integration |
| Original planned verification | Inspect UI, IPC schemas, persistence and installer for password/account pathways; separately verify Windows peer authentication and authorization. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.2.8

| Field | Value |
| --- | --- |
| Source shortcode | V6.2.8 |
| Minimum assigned level | L1 |
| Chapter | Authentication |
| Section | Password Security |
| Adapted requirement summary | Verify passwords exactly without truncation or transformation. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application password/account credential requirement |
| Applicability rationale | The app neither creates user accounts nor accepts, stores, changes or recovers passwords; Windows owns interactive identity. This does not waive generic identity/authorization rows. |
| Native control mapping | CTRL-SCOPE: no application credential UI/store/recovery; CTRL-IPC: Windows identity. |
| Decision / implementation owner | Architecture; Windows integration |
| Original planned verification | Inspect UI, IPC schemas, persistence and installer for password/account pathways; separately verify Windows peer authentication and authorization. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.2.9

| Field | Value |
| --- | --- |
| Source shortcode | V6.2.9 |
| Minimum assigned level | L2 |
| Chapter | Authentication |
| Section | Password Security |
| Adapted requirement summary | Allow passwords of at least sixty-four characters. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application password/account credential requirement |
| Applicability rationale | The app neither creates user accounts nor accepts, stores, changes or recovers passwords; Windows owns interactive identity. This does not waive generic identity/authorization rows. |
| Native control mapping | CTRL-SCOPE: no application credential UI/store/recovery; CTRL-IPC: Windows identity. |
| Decision / implementation owner | Architecture; Windows integration |
| Original planned verification | Inspect UI, IPC schemas, persistence and installer for password/account pathways; separately verify Windows peer authentication and authorization. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.2.10

| Field | Value |
| --- | --- |
| Source shortcode | V6.2.10 |
| Minimum assigned level | L2 |
| Chapter | Authentication |
| Section | Password Security |
| Adapted requirement summary | Avoid forced periodic password changes absent compromise. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application password/account credential requirement |
| Applicability rationale | The app neither creates user accounts nor accepts, stores, changes or recovers passwords; Windows owns interactive identity. This does not waive generic identity/authorization rows. |
| Native control mapping | CTRL-SCOPE: no application credential UI/store/recovery; CTRL-IPC: Windows identity. |
| Decision / implementation owner | Architecture; Windows integration |
| Original planned verification | Inspect UI, IPC schemas, persistence and installer for password/account pathways; separately verify Windows peer authentication and authorization. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.2.11

| Field | Value |
| --- | --- |
| Source shortcode | V6.2.11 |
| Minimum assigned level | L2 |
| Chapter | Authentication |
| Section | Password Security |
| Adapted requirement summary | Apply the documented context-specific password blocklist. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application password/account credential requirement |
| Applicability rationale | The app neither creates user accounts nor accepts, stores, changes or recovers passwords; Windows owns interactive identity. This does not waive generic identity/authorization rows. |
| Native control mapping | CTRL-SCOPE: no application credential UI/store/recovery; CTRL-IPC: Windows identity. |
| Decision / implementation owner | Architecture; Windows integration |
| Original planned verification | Inspect UI, IPC schemas, persistence and installer for password/account pathways; separately verify Windows peer authentication and authorization. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.2.12

| Field | Value |
| --- | --- |
| Source shortcode | V6.2.12 |
| Minimum assigned level | L2 |
| Chapter | Authentication |
| Section | Password Security |
| Adapted requirement summary | Reject passwords found in breach data. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application password/account credential requirement |
| Applicability rationale | The app neither creates user accounts nor accepts, stores, changes or recovers passwords; Windows owns interactive identity. This does not waive generic identity/authorization rows. |
| Native control mapping | CTRL-SCOPE: no application credential UI/store/recovery; CTRL-IPC: Windows identity. |
| Decision / implementation owner | Architecture; Windows integration |
| Original planned verification | Inspect UI, IPC schemas, persistence and installer for password/account pathways; separately verify Windows peer authentication and authorization. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.3.1

| Field | Value |
| --- | --- |
| Source shortcode | V6.3.1 |
| Minimum assigned level | L1 |
| Chapter | Authentication |
| Section | General Authentication Security |
| Adapted requirement summary | Implement the documented credential-guessing defenses. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application password/account credential requirement |
| Applicability rationale | The app neither creates user accounts nor accepts, stores, changes or recovers passwords; Windows owns interactive identity. This does not waive generic identity/authorization rows. |
| Native control mapping | CTRL-SCOPE: no application credential UI/store/recovery; CTRL-IPC: Windows identity. |
| Decision / implementation owner | Architecture; Windows integration |
| Original planned verification | Inspect UI, IPC schemas, persistence and installer for password/account pathways; separately verify Windows peer authentication and authorization. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.3.2

| Field | Value |
| --- | --- |
| Source shortcode | V6.3.2 |
| Minimum assigned level | L1 |
| Chapter | Authentication |
| Section | General Authentication Security |
| Adapted requirement summary | Remove or disable default application accounts. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic service configuration requirement |
| Applicability rationale | No application user account or default shared credential is created. Installed Windows service identities must be explicit, scoped and free of embedded reusable account passwords. |
| Native control mapping | CTRL-CONFIG: protected versioned configuration; CTRL-PRIV: minimum service privileges; CTRL-EGRESS: approved destinations. |
| Decision / implementation owner | Architecture; Windows integration; installer |
| Original planned verification | Inspect installer behavior, service identities, IPC ACLs and packaged configuration; verify no default application login or shared credential grants access. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.3.3

| Field | Value |
| --- | --- |
| Source shortcode | V6.3.3 |
| Minimum assigned level | L2 |
| Chapter | Authentication |
| Section | General Authentication Security |
| Adapted requirement summary | Require multiple factors and stronger hardware-backed intent at L3. |
| Proposed applicability | Needs decision |
| Literal standard scope | Literal requirement or applicability needs an explicit security decision |
| Applicability rationale | Windows identity or UAC elevation alone does not prove the stipulated factor combination, phishing resistance, hardware possession or explicit authentication intent. The clause permits relaxation only with documented rationale and comprehensive mitigations. |
| Native control mapping | CTRL-DECISION-AUTH: Windows assurance inventory; explicit L3 authentication gap; short-lived privileged actions and restrictive IPC as compensations, not factor equivalence. |
| Decision / implementation owner | Security owner; architecture |
| Original planned verification | Determine the supported Windows identity assurance and actual administrative flow; test it on representative machines. Record an explicit scoped relaxation if the literal factors cannot be guaranteed. |
| Open question | Will the product require a managed Windows hardware-MFA policy, or document the clause's permitted relaxation and residual risk for ordinary desktop use? |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.3.4

| Field | Value |
| --- | --- |
| Source shortcode | V6.3.4 |
| Minimum assigned level | L2 |
| Chapter | Authentication |
| Section | General Authentication Security |
| Adapted requirement summary | Enforce documented security across every authentication path. |
| Proposed applicability | Native adaptation (proposed) |
| Literal standard scope | Authentication/session requirement adapted to local Windows identities |
| Applicability rationale | The app has no password login; it delegates identity to Windows and authenticates local IPC peers. A kernel token is not a web session or proof of MFA. |
| Native control mapping | CTRL-IPC: secured local named pipes and verified peer token; CTRL-AUTHZ: per-action authorization. |
| Decision / implementation owner | Windows integration; security owner |
| Original planned verification | Test actual Windows tokens, service identity, session/integrity level and unauthorized neighboring clients; document delegated and unproven assurance. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.3.5

| Field | Value |
| --- | --- |
| Source shortcode | V6.3.5 |
| Minimum assigned level | L3 |
| Chapter | Authentication |
| Section | General Authentication Security |
| Adapted requirement summary | Notify users of suspicious authentication attempts. |
| Proposed applicability | Native adaptation (proposed) |
| Literal standard scope | Authentication/audit requirement adapted to local Windows and protocol events |
| Applicability rationale | Apply the purpose to suspicious owned IPC and NTS admission events, with bounded actionable notifications. The application does not monitor or certify all Windows logon attempts. |
| Native control mapping | CTRL-LOG: classified local and protocol security events; CTRL-IPC: verified identity and bounded notifications. |
| Decision / implementation owner | Observability; Windows integration; security owner |
| Original planned verification | Trigger repeated denied privileged IPC attempts and suspicious peer-authentication failures; verify an accessible, rate-bounded notice and retained event without exposing secrets or flooding the desktop. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.3.6

| Field | Value |
| --- | --- |
| Source shortcode | V6.3.6 |
| Minimum assigned level | L3 |
| Chapter | Authentication |
| Section | General Authentication Security |
| Adapted requirement summary | Exclude email as an authentication factor. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application-owned MFA/factor/OOB lifecycle requirement |
| Applicability rationale | No email or other application-owned user-authentication mechanism exists; Windows authentication is delegated and not asserted to satisfy hardware-MFA requirements. |
| Native control mapping | CTRL-SCOPE: no application factor service; V6.3.3 and delegated-auth assurance remain open. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Confirm absent factor-generation/enrollment/recovery paths; do not substitute NTS cookies or packet nonces for human MFA factors. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.3.7

| Field | Value |
| --- | --- |
| Source shortcode | V6.3.7 |
| Minimum assigned level | L3 |
| Chapter | Authentication |
| Section | General Authentication Security |
| Adapted requirement summary | Notify users when their authentication details change. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application password/account credential requirement |
| Applicability rationale | The application does not maintain or change user credentials, usernames or account email addresses. Configuration-change audit events are required separately and are not credential-update notices. |
| Native control mapping | CTRL-SCOPE: no application credential UI/store/recovery; CTRL-IPC: Windows identity. |
| Decision / implementation owner | Architecture; Windows integration |
| Original planned verification | Inspect UI, IPC schemas, persistence and installer for password/account pathways; separately verify Windows peer authentication and authorization. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.3.8

| Field | Value |
| --- | --- |
| Source shortcode | V6.3.8 |
| Minimum assigned level | L3 |
| Chapter | Authentication |
| Section | General Authentication Security |
| Adapted requirement summary | Prevent account enumeration through authentication failures. |
| Proposed applicability | Native adaptation (proposed) |
| Literal standard scope | Authentication/session requirement adapted to local Windows identities |
| Applicability rationale | The app accepts no username/password challenge or registration request. For native IPC, failures must not expose unnecessary account/group details; authorization must use verified SIDs rather than display-name lookup. |
| Native control mapping | CTRL-IPC: secured local named pipes and verified peer token; CTRL-AUTHZ: per-action authorization. |
| Decision / implementation owner | Windows integration; security owner |
| Original planned verification | Compare denial responses for unauthorized Windows principals and malformed claimed identities; inspect timing and diagnostic disclosure within the owned IPC boundary, without claiming OS-wide enumeration prevention. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.4.1

| Field | Value |
| --- | --- |
| Source shortcode | V6.4.1 |
| Minimum assigned level | L1 |
| Chapter | Authentication |
| Section | Authentication Factor Lifecycle and Recovery |
| Adapted requirement summary | Make initial passwords or activation codes random, short-lived and single-use. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application password/account credential requirement |
| Applicability rationale | The app neither creates user accounts nor accepts, stores, changes or recovers passwords; Windows owns interactive identity. This does not waive generic identity/authorization rows. |
| Native control mapping | CTRL-SCOPE: no application credential UI/store/recovery; CTRL-IPC: Windows identity. |
| Decision / implementation owner | Architecture; Windows integration |
| Original planned verification | Inspect UI, IPC schemas, persistence and installer for password/account pathways; separately verify Windows peer authentication and authorization. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.4.2

| Field | Value |
| --- | --- |
| Source shortcode | V6.4.2 |
| Minimum assigned level | L1 |
| Chapter | Authentication |
| Section | Authentication Factor Lifecycle and Recovery |
| Adapted requirement summary | Exclude password hints and knowledge-based security questions. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application password/account credential requirement |
| Applicability rationale | The app neither creates user accounts nor accepts, stores, changes or recovers passwords; Windows owns interactive identity. This does not waive generic identity/authorization rows. |
| Native control mapping | CTRL-SCOPE: no application credential UI/store/recovery; CTRL-IPC: Windows identity. |
| Decision / implementation owner | Architecture; Windows integration |
| Original planned verification | Inspect UI, IPC schemas, persistence and installer for password/account pathways; separately verify Windows peer authentication and authorization. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.4.3

| Field | Value |
| --- | --- |
| Source shortcode | V6.4.3 |
| Minimum assigned level | L2 |
| Chapter | Authentication |
| Section | Authentication Factor Lifecycle and Recovery |
| Adapted requirement summary | Reset forgotten passwords without bypassing enabled factors. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application password/account credential requirement |
| Applicability rationale | The app neither creates user accounts nor accepts, stores, changes or recovers passwords; Windows owns interactive identity. This does not waive generic identity/authorization rows. |
| Native control mapping | CTRL-SCOPE: no application credential UI/store/recovery; CTRL-IPC: Windows identity. |
| Decision / implementation owner | Architecture; Windows integration |
| Original planned verification | Inspect UI, IPC schemas, persistence and installer for password/account pathways; separately verify Windows peer authentication and authorization. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.4.4

| Field | Value |
| --- | --- |
| Source shortcode | V6.4.4 |
| Minimum assigned level | L2 |
| Chapter | Authentication |
| Section | Authentication Factor Lifecycle and Recovery |
| Adapted requirement summary | Re-prove identity when recovering a lost authentication factor. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application-owned MFA/factor/OOB lifecycle requirement |
| Applicability rationale | No application-owned OTP, lookup secret, SMS, push, biometric enrollment or factor recovery is implemented; Windows credential assurance is a separate unresolved policy boundary. |
| Native control mapping | CTRL-SCOPE: no application factor service; V6.3.3 and delegated-auth assurance remain open. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Confirm absent factor-generation/enrollment/recovery paths; do not substitute NTS cookies or packet nonces for human MFA factors. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.4.5

| Field | Value |
| --- | --- |
| Source shortcode | V6.4.5 |
| Minimum assigned level | L3 |
| Chapter | Authentication |
| Section | Authentication Factor Lifecycle and Recovery |
| Adapted requirement summary | Warn early enough to renew expiring human authentication methods. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application-owned MFA/factor/OOB lifecycle requirement |
| Applicability rationale | There is no application-owned human authentication factor or enrollment lifecycle. NTS key renewal and expiring trust/provider state are separate machine-protocol requirements. |
| Native control mapping | CTRL-SCOPE: no application factor service; V6.3.3 and delegated-auth assurance remain open. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Confirm absent factor-generation/enrollment/recovery paths; do not substitute NTS cookies or packet nonces for human MFA factors. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.4.6

| Field | Value |
| --- | --- |
| Source shortcode | V6.4.6 |
| Minimum assigned level | L3 |
| Chapter | Authentication |
| Section | Authentication Factor Lifecycle and Recovery |
| Adapted requirement summary | Let admins initiate resets without choosing another user's password. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application password/account credential requirement |
| Applicability rationale | The app neither creates user accounts nor accepts, stores, changes or recovers passwords; Windows owns interactive identity. This does not waive generic identity/authorization rows. |
| Native control mapping | CTRL-SCOPE: no application credential UI/store/recovery; CTRL-IPC: Windows identity. |
| Decision / implementation owner | Architecture; Windows integration |
| Original planned verification | Inspect UI, IPC schemas, persistence and installer for password/account pathways; separately verify Windows peer authentication and authorization. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.5.1

| Field | Value |
| --- | --- |
| Source shortcode | V6.5.1 |
| Minimum assigned level | L2 |
| Chapter | Authentication |
| Section | General Multi-factor authentication requirements |
| Adapted requirement summary | Make lookup secrets, out-of-band codes and TOTPs single-use. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application-owned MFA/factor/OOB lifecycle requirement |
| Applicability rationale | No application-owned OTP, lookup secret, SMS, push, biometric enrollment or factor recovery is implemented; Windows credential assurance is a separate unresolved policy boundary. |
| Native control mapping | CTRL-SCOPE: no application factor service; V6.3.3 and delegated-auth assurance remain open. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Confirm absent factor-generation/enrollment/recovery paths; do not substitute NTS cookies or packet nonces for human MFA factors. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.5.2

| Field | Value |
| --- | --- |
| Source shortcode | V6.5.2 |
| Minimum assigned level | L2 |
| Chapter | Authentication |
| Section | General Multi-factor authentication requirements |
| Adapted requirement summary | Store low-entropy lookup secrets with a salted approved password hash. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application-owned MFA/factor/OOB lifecycle requirement |
| Applicability rationale | No application-owned OTP, lookup secret, SMS, push, biometric enrollment or factor recovery is implemented; Windows credential assurance is a separate unresolved policy boundary. |
| Native control mapping | CTRL-SCOPE: no application factor service; V6.3.3 and delegated-auth assurance remain open. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Confirm absent factor-generation/enrollment/recovery paths; do not substitute NTS cookies or packet nonces for human MFA factors. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.5.3

| Field | Value |
| --- | --- |
| Source shortcode | V6.5.3 |
| Minimum assigned level | L2 |
| Chapter | Authentication |
| Section | General Multi-factor authentication requirements |
| Adapted requirement summary | Generate factor seeds and authentication codes using a CSPRNG. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application-owned MFA/factor/OOB lifecycle requirement |
| Applicability rationale | No application-owned OTP, lookup secret, SMS, push, biometric enrollment or factor recovery is implemented; Windows credential assurance is a separate unresolved policy boundary. |
| Native control mapping | CTRL-SCOPE: no application factor service; V6.3.3 and delegated-auth assurance remain open. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Confirm absent factor-generation/enrollment/recovery paths; do not substitute NTS cookies or packet nonces for human MFA factors. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.5.4

| Field | Value |
| --- | --- |
| Source shortcode | V6.5.4 |
| Minimum assigned level | L2 |
| Chapter | Authentication |
| Section | General Multi-factor authentication requirements |
| Adapted requirement summary | Give lookup and out-of-band codes sufficient entropy. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application-owned MFA/factor/OOB lifecycle requirement |
| Applicability rationale | No application-owned OTP, lookup secret, SMS, push, biometric enrollment or factor recovery is implemented; Windows credential assurance is a separate unresolved policy boundary. |
| Native control mapping | CTRL-SCOPE: no application factor service; V6.3.3 and delegated-auth assurance remain open. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Confirm absent factor-generation/enrollment/recovery paths; do not substitute NTS cookies or packet nonces for human MFA factors. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.5.5

| Field | Value |
| --- | --- |
| Source shortcode | V6.5.5 |
| Minimum assigned level | L2 |
| Chapter | Authentication |
| Section | General Multi-factor authentication requirements |
| Adapted requirement summary | Bound lifetimes of out-of-band and time-based codes. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application-owned MFA/factor/OOB lifecycle requirement |
| Applicability rationale | No application-owned OTP, lookup secret, SMS, push, biometric enrollment or factor recovery is implemented; Windows credential assurance is a separate unresolved policy boundary. |
| Native control mapping | CTRL-SCOPE: no application factor service; V6.3.3 and delegated-auth assurance remain open. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Confirm absent factor-generation/enrollment/recovery paths; do not substitute NTS cookies or packet nonces for human MFA factors. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.5.6

| Field | Value |
| --- | --- |
| Source shortcode | V6.5.6 |
| Minimum assigned level | L3 |
| Chapter | Authentication |
| Section | General Multi-factor authentication requirements |
| Adapted requirement summary | Support revocation of lost or stolen authentication factors. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application-owned MFA/factor/OOB lifecycle requirement |
| Applicability rationale | No application-owned OTP, lookup secret, SMS, push, biometric enrollment or factor recovery is implemented; Windows credential assurance is a separate unresolved policy boundary. |
| Native control mapping | CTRL-SCOPE: no application factor service; V6.3.3 and delegated-auth assurance remain open. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Confirm absent factor-generation/enrollment/recovery paths; do not substitute NTS cookies or packet nonces for human MFA factors. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.5.7

| Field | Value |
| --- | --- |
| Source shortcode | V6.5.7 |
| Minimum assigned level | L3 |
| Chapter | Authentication |
| Section | General Multi-factor authentication requirements |
| Adapted requirement summary | Use biometrics only alongside another factor type. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application-owned MFA/factor/OOB lifecycle requirement |
| Applicability rationale | No application-owned OTP, lookup secret, SMS, push, biometric enrollment or factor recovery is implemented; Windows credential assurance is a separate unresolved policy boundary. |
| Native control mapping | CTRL-SCOPE: no application factor service; V6.3.3 and delegated-auth assurance remain open. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Confirm absent factor-generation/enrollment/recovery paths; do not substitute NTS cookies or packet nonces for human MFA factors. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.5.8

| Field | Value |
| --- | --- |
| Source shortcode | V6.5.8 |
| Minimum assigned level | L3 |
| Chapter | Authentication |
| Section | General Multi-factor authentication requirements |
| Adapted requirement summary | Validate TOTPs against trusted service time. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application-owned MFA/factor/OOB lifecycle requirement |
| Applicability rationale | The product provides time synchronization but does not issue or validate TOTPs. Its general clock-trust design must not be mislabeled as verification of a nonexistent TOTP login. |
| Native control mapping | CTRL-SCOPE: no application factor service; V6.3.3 and delegated-auth assurance remain open. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Confirm absent factor-generation/enrollment/recovery paths; do not substitute NTS cookies or packet nonces for human MFA factors. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.6.1

| Field | Value |
| --- | --- |
| Source shortcode | V6.6.1 |
| Minimum assigned level | L2 |
| Chapter | Authentication |
| Section | Out-of-Band authentication mechanisms |
| Adapted requirement summary | Restrict telephone or SMS factors and exclude them at L3. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application-owned MFA/factor/OOB lifecycle requirement |
| Applicability rationale | No application-owned OTP, lookup secret, SMS, push, biometric enrollment or factor recovery is implemented; Windows credential assurance is a separate unresolved policy boundary. |
| Native control mapping | CTRL-SCOPE: no application factor service; V6.3.3 and delegated-auth assurance remain open. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Confirm absent factor-generation/enrollment/recovery paths; do not substitute NTS cookies or packet nonces for human MFA factors. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.6.2

| Field | Value |
| --- | --- |
| Source shortcode | V6.6.2 |
| Minimum assigned level | L2 |
| Chapter | Authentication |
| Section | Out-of-Band authentication mechanisms |
| Adapted requirement summary | Bind each out-of-band response to its original request. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application-owned MFA/factor/OOB lifecycle requirement |
| Applicability rationale | No application-owned OTP, lookup secret, SMS, push, biometric enrollment or factor recovery is implemented; Windows credential assurance is a separate unresolved policy boundary. |
| Native control mapping | CTRL-SCOPE: no application factor service; V6.3.3 and delegated-auth assurance remain open. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Confirm absent factor-generation/enrollment/recovery paths; do not substitute NTS cookies or packet nonces for human MFA factors. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.6.3

| Field | Value |
| --- | --- |
| Source shortcode | V6.6.3 |
| Minimum assigned level | L2 |
| Chapter | Authentication |
| Section | Out-of-Band authentication mechanisms |
| Adapted requirement summary | Rate-limit guessing of out-of-band authentication codes. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application-owned MFA/factor/OOB lifecycle requirement |
| Applicability rationale | No application-owned OTP, lookup secret, SMS, push, biometric enrollment or factor recovery is implemented; Windows credential assurance is a separate unresolved policy boundary. |
| Native control mapping | CTRL-SCOPE: no application factor service; V6.3.3 and delegated-auth assurance remain open. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Confirm absent factor-generation/enrollment/recovery paths; do not substitute NTS cookies or packet nonces for human MFA factors. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.6.4

| Field | Value |
| --- | --- |
| Source shortcode | V6.6.4 |
| Minimum assigned level | L3 |
| Chapter | Authentication |
| Section | Out-of-Band authentication mechanisms |
| Adapted requirement summary | Limit repeated push-based MFA prompts. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application-owned MFA/factor/OOB lifecycle requirement |
| Applicability rationale | No application-owned OTP, lookup secret, SMS, push, biometric enrollment or factor recovery is implemented; Windows credential assurance is a separate unresolved policy boundary. |
| Native control mapping | CTRL-SCOPE: no application factor service; V6.3.3 and delegated-auth assurance remain open. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Confirm absent factor-generation/enrollment/recovery paths; do not substitute NTS cookies or packet nonces for human MFA factors. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.7.1

| Field | Value |
| --- | --- |
| Source shortcode | V6.7.1 |
| Minimum assigned level | L3 |
| Chapter | Authentication |
| Section | Cryptographic authentication mechanism |
| Adapted requirement summary | Protect certificates used to verify cryptographic assertions from alteration. |
| Proposed applicability | Native adaptation (proposed) |
| Literal standard scope | Cryptographic user-authentication requirement adapted to protocol peer authentication |
| Applicability rationale | Map certificate-protection purpose to NTS-KE trust anchors and any protected pin configuration. These are server-authentication controls, not proof of an application-owned human cryptographic authenticator. |
| Native control mapping | CTRL-CERT: protected trust decisions; CTRL-NTS: protocol-required unique identifiers and authentication bindings. |
| Decision / implementation owner | Crypto/TLS; protocol/core |
| Original planned verification | Attempt unauthorized trust-store/pin changes at the actual chosen trust boundary; verify rejected modification and correct chain/hostname behavior with controlled peers. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.7.2

| Field | Value |
| --- | --- |
| Source shortcode | V6.7.2 |
| Minimum assigned level | L3 |
| Chapter | Authentication |
| Section | Cryptographic authentication mechanism |
| Adapted requirement summary | Use sufficiently long, nonrepeating cryptographic authentication challenges. |
| Proposed applicability | Native adaptation (proposed) |
| Literal standard scope | Cryptographic user-authentication requirement adapted to protocol peer authentication |
| Applicability rationale | TLS/NTS freshness and unique identifiers must follow their protocol and cryptographic requirements. They are not a replacement for a human authenticator challenge; the protocol-specific required sizes prevail over this minimum. |
| Native control mapping | CTRL-CERT: protected trust decisions; CTRL-NTS: protocol-required unique identifiers and authentication bindings. |
| Decision / implementation owner | Crypto/TLS; protocol/core |
| Original planned verification | Check actual random-provider use and identifier lengths; use replay, duplicate-request and cross-association controls against the real protocol adapter. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.8.1

| Field | Value |
| --- | --- |
| Source shortcode | V6.8.1 |
| Minimum assigned level | L2 |
| Chapter | Authentication |
| Section | Authentication with an Identity Provider |
| Adapted requirement summary | Namespace identities so one provider cannot impersonate another. |
| Proposed applicability | Native adaptation (proposed) |
| Literal standard scope | Authentication/session requirement adapted to local Windows identities |
| Applicability rationale | Native adaptation uses complete Windows SIDs and verified tokens, not potentially colliding local/domain account display names. This does not implement an app-level federation protocol. |
| Native control mapping | CTRL-IPC: secured local named pipes and verified peer token; CTRL-AUTHZ: per-action authorization. |
| Decision / implementation owner | Windows integration; security owner |
| Original planned verification | Test local and domain principals with colliding display names and changed textual names; verify authorization remains bound to the intended actual SID and authority. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.8.2

| Field | Value |
| --- | --- |
| Source shortcode | V6.8.2 |
| Minimum assigned level | L2 |
| Chapter | Authentication |
| Section | Authentication with an Identity Provider |
| Adapted requirement summary | Verify signatures on external authentication assertions. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Federated login/IdP-specific requirement |
| Applicability rationale | No application JWT/SAML or analogous signed user assertion is accepted. Windows kernel-issued peer tokens and TLS certificates have distinct verification mechanisms covered elsewhere. |
| Native control mapping | CTRL-SCOPE: no application federation; stable Windows SIDs and peer tokens are assessed separately. |
| Decision / implementation owner | Architecture; Windows integration |
| Original planned verification | Review login/IPC identity flows and dependencies for absent federation; reopen if app-level IdP/SSO is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.8.3

| Field | Value |
| --- | --- |
| Source shortcode | V6.8.3 |
| Minimum assigned level | L2 |
| Chapter | Authentication |
| Section | Authentication with an Identity Provider |
| Adapted requirement summary | Consume each SAML assertion at most once within its valid lifetime. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Federated login/IdP-specific requirement |
| Applicability rationale | The app does not consume SAML/OIDC authentication assertions or manage federated application sessions. Windows domain identity is not implemented as an app SSO protocol. |
| Native control mapping | CTRL-SCOPE: no application federation; stable Windows SIDs and peer tokens are assessed separately. |
| Decision / implementation owner | Architecture; Windows integration |
| Original planned verification | Review login/IPC identity flows and dependencies for absent federation; reopen if app-level IdP/SSO is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-6.8.4

| Field | Value |
| --- | --- |
| Source shortcode | V6.8.4 |
| Minimum assigned level | L2 |
| Chapter | Authentication |
| Section | Authentication with an Identity Provider |
| Adapted requirement summary | Verify delegated authentication strength and recency or assume the minimum. |
| Proposed applicability | Native adaptation (proposed) |
| Literal standard scope | Authentication/session requirement adapted to local Windows identities |
| Applicability rationale | Windows is the delegated identity authority. The application must document which assurance/recency facts it can actually obtain; a token or UAC prompt alone must not be represented as hardware MFA or recent strong authentication. |
| Native control mapping | CTRL-AUTH-ASSURANCE: documented Windows delegation and minimum-assurance fallback; CTRL-DECISION-AUTH. |
| Decision / implementation owner | Windows integration; security owner |
| Original planned verification | Inspect actual Windows metadata used by each sensitive action; test missing/insufficient assurance metadata and documented conservative fallback. Resolve this together with V6.3.3. |
| Open question | Which strength and recency facts can supported Windows configurations expose reliably, and what conservative fallback will be enforced? |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |


## V7 - Session Management

### v5.0.0-7.1.1

| Field | Value |
| --- | --- |
| Source shortcode | V7.1.1 |
| Minimum assigned level | L2 |
| Chapter | Session Management |
| Section | Session Management Documentation |
| Adapted requirement summary | Document idle and absolute session limits and reauthentication deviations. |
| Proposed applicability | Native adaptation (proposed) |
| Literal standard scope | User-session requirement adapted to privileged IPC lifecycle |
| Applicability rationale | Document human command/IPC authority lifetime separately from unattended machine service lifetime; assess any claimed NIST reauthentication correspondence explicitly, not by analogy alone. |
| Native control mapping | CTRL-SESSION: bounded privileged command lifetime, reconnect authentication and disposal; CTRL-IPC. |
| Decision / implementation owner | IPC; helper; security owner |
| Original planned verification | Review all human authority paths and connection lifetimes; exercise expiry using a monotonic test clock and real Windows reconnects, and review documented deviations. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-7.1.2

| Field | Value |
| --- | --- |
| Source shortcode | V7.1.2 |
| Minimum assigned level | L2 |
| Chapter | Session Management |
| Section | Session Management Documentation |
| Adapted requirement summary | Document concurrent-session caps and overflow handling. |
| Proposed applicability | Native adaptation (proposed) |
| Literal standard scope | User-session requirement adapted to privileged IPC lifecycle |
| Applicability rationale | Map to bounded concurrent IPC connections and per-principal administrative work. Unattended source polling is not a user session. |
| Native control mapping | CTRL-SESSION: bounded privileged command lifetime, reconnect authentication and disposal; CTRL-IPC. |
| Decision / implementation owner | IPC; helper; security owner |
| Original planned verification | Review per-principal and global IPC limits; attempt concurrent allowed and denied clients and verify defined overflow and recovery behavior. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-7.1.3

| Field | Value |
| --- | --- |
| Source shortcode | V7.1.3 |
| Minimum assigned level | L2 |
| Chapter | Session Management |
| Section | Session Management Documentation |
| Adapted requirement summary | Document coordination of federated session managers. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Federated login/IdP-specific requirement |
| Applicability rationale | The app does not consume SAML/OIDC authentication assertions or manage federated application sessions. Windows domain identity is not implemented as an app SSO protocol. |
| Native control mapping | CTRL-SCOPE: no application federation; stable Windows SIDs and peer tokens are assessed separately. |
| Decision / implementation owner | Architecture; Windows integration |
| Original planned verification | Review login/IPC identity flows and dependencies for absent federation; reopen if app-level IdP/SSO is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-7.2.1

| Field | Value |
| --- | --- |
| Source shortcode | V7.2.1 |
| Minimum assigned level | L1 |
| Chapter | Session Management |
| Section | Fundamental Session Management Security |
| Adapted requirement summary | Verify session credentials at the trusted service boundary. |
| Proposed applicability | Native adaptation (proposed) |
| Literal standard scope | Authentication/session requirement adapted to local Windows identities |
| Applicability rationale | No web session token exists. Each protected native service authenticates the actual Windows peer and enforces its own authorization; UI assertions are not trusted credentials. |
| Native control mapping | CTRL-IPC: secured local named pipes and verified peer token; CTRL-AUTHZ: per-action authorization. |
| Decision / implementation owner | Windows integration; security owner |
| Original planned verification | Test actual Windows tokens, service identity, session/integrity level and unauthorized neighboring clients; document delegated and unproven assurance. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-7.2.2

| Field | Value |
| --- | --- |
| Source shortcode | V7.2.2 |
| Minimum assigned level | L1 |
| Chapter | Session Management |
| Section | Fundamental Session Management Security |
| Adapted requirement summary | Use dynamic session credentials instead of static API secrets. |
| Proposed applicability | Native adaptation (proposed) |
| Literal standard scope | User-session requirement adapted to privileged IPC lifecycle |
| Applicability rationale | No application bearer session credential is issued; native peer identity comes from Windows. Exclude hard-coded shared pipe passwords and static API keys; do not claim a Windows token is an app-issued web token. |
| Native control mapping | CTRL-SESSION: bounded privileged command lifetime, reconnect authentication and disposal; CTRL-IPC. |
| Decision / implementation owner | IPC; helper; security owner |
| Original planned verification | Exercise disconnect, timeout, reconnect and stale commands with independent clock/control state; distinguish human authority from machine service lifetime. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-7.2.3

| Field | Value |
| --- | --- |
| Source shortcode | V7.2.3 |
| Minimum assigned level | L1 |
| Chapter | Session Management |
| Section | Fundamental Session Management Security |
| Adapted requirement summary | Generate opaque session tokens uniquely with at least 128 bits of entropy. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Opaque reference-token user-session requirement |
| Applicability rationale | The native app has no opaque reference-token user session store either. IPC request identifiers are not bearer credentials; if such tokens are later added this row becomes applicable. |
| Native control mapping | CTRL-SCOPE: no opaque application reference-token session store; request identifiers must never become bearer authority. |
| Decision / implementation owner | Architecture; protocol/core |
| Original planned verification | Inspect IPC schemas and authority lifecycle for absence of opaque bearer-session credentials; reopen if such credentials are introduced. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-7.2.4

| Field | Value |
| --- | --- |
| Source shortcode | V7.2.4 |
| Minimum assigned level | L1 |
| Chapter | Session Management |
| Section | Fundamental Session Management Security |
| Adapted requirement summary | Replace and invalidate session credentials after authentication. |
| Proposed applicability | Native adaptation (proposed) |
| Literal standard scope | User-session requirement adapted to privileged IPC lifecycle |
| Applicability rationale | Native adaptation reconnects/rechecks the actual peer and discards stale privileged bindings or request generations; it does not claim to rotate Windows logon tokens itself. |
| Native control mapping | CTRL-SESSION: bounded privileged command lifetime, reconnect authentication and disposal; CTRL-IPC. |
| Decision / implementation owner | IPC; helper; security owner |
| Original planned verification | Reconnect under changed principals and authority states; attempt to reuse stale requests and connection bindings; verify no preserved privilege from an earlier accepted session. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-7.3.1

| Field | Value |
| --- | --- |
| Source shortcode | V7.3.1 |
| Minimum assigned level | L2 |
| Chapter | Session Management |
| Section | Session Timeout |
| Adapted requirement summary | Enforce the documented idle limit before reauthentication. |
| Proposed applicability | Native adaptation (proposed) |
| Literal standard scope | User-session requirement adapted to privileged IPC lifecycle |
| Applicability rationale | Interactive administrative command authority is short-lived and distinct from unattended machine synchronization. No application bearer-session token is issued. |
| Native control mapping | CTRL-SESSION: bounded privileged command lifetime, reconnect authentication and disposal; CTRL-IPC. |
| Decision / implementation owner | IPC; helper; security owner |
| Original planned verification | Exercise disconnect, timeout, reconnect and stale commands with independent clock/control state; distinguish human authority from machine service lifetime. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-7.3.2

| Field | Value |
| --- | --- |
| Source shortcode | V7.3.2 |
| Minimum assigned level | L2 |
| Chapter | Session Management |
| Section | Session Timeout |
| Adapted requirement summary | Enforce the documented absolute session lifetime. |
| Proposed applicability | Native adaptation (proposed) |
| Literal standard scope | User-session requirement adapted to privileged IPC lifecycle |
| Applicability rationale | Interactive administrative command authority is short-lived and distinct from unattended machine synchronization. No application bearer-session token is issued. |
| Native control mapping | CTRL-SESSION: bounded privileged command lifetime, reconnect authentication and disposal; CTRL-IPC. |
| Decision / implementation owner | IPC; helper; security owner |
| Original planned verification | Exercise disconnect, timeout, reconnect and stale commands with independent clock/control state; distinguish human authority from machine service lifetime. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-7.4.1

| Field | Value |
| --- | --- |
| Source shortcode | V7.4.1 |
| Minimum assigned level | L1 |
| Chapter | Session Management |
| Section | Session Termination |
| Adapted requirement summary | Reject any further use of terminated sessions. |
| Proposed applicability | Native adaptation (proposed) |
| Literal standard scope | User-session requirement adapted to privileged IPC lifecycle |
| Applicability rationale | Closing an IPC connection or expiring a privileged command binding must invalidate subsequent requests and cancel undispatched authority. Already committed irreversible clock effects cannot be rolled back as a session operation. |
| Native control mapping | CTRL-SESSION: bounded privileged command lifetime, reconnect authentication and disposal; CTRL-IPC. |
| Decision / implementation owner | IPC; helper; security owner |
| Original planned verification | Exercise disconnect/expiry immediately before and after the broker commit boundary; verify defined cancellation, rejection of later requests, and accurate reporting of any committed or unknown effect. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-7.4.2

| Field | Value |
| --- | --- |
| Source shortcode | V7.4.2 |
| Minimum assigned level | L1 |
| Chapter | Session Management |
| Section | Session Termination |
| Adapted requirement summary | Terminate active sessions when an account is disabled or deleted. |
| Proposed applicability | Needs decision |
| Literal standard scope | Literal requirement or applicability needs an explicit security decision |
| Applicability rationale | Existing Windows tokens/connections may outlive a directory/account change. Rechecking the same cached token is not evidence that account disablement instantly invalidates every session. |
| Native control mapping | CTRL-DECISION-REVOCATION: account-change detection and session invalidation; no unsupported immediate-revocation claim. |
| Decision / implementation owner | Security owner; architecture |
| Original planned verification | Determine observable account-change signals and policy for local/domain/offline conditions; use actual disable/delete cases and existing connections to measure termination behavior. |
| Open question | How will account disablement reach existing IPC bindings, and what bounded or disconnected-domain limitation requires a documented exception? |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-7.4.3

| Field | Value |
| --- | --- |
| Source shortcode | V7.4.3 |
| Minimum assigned level | L2 |
| Chapter | Session Management |
| Section | Session Termination |
| Adapted requirement summary | Offer termination of other sessions after factor changes. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application-owned MFA/factor/OOB lifecycle requirement |
| Applicability rationale | No application-owned factor-change, password-reset or recovery path exists. Windows account-change effects on IPC remain an explicit open issue under V7.4.2. |
| Native control mapping | CTRL-SCOPE: no application factor service; V6.3.3 and delegated-auth assurance remain open. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Confirm absent factor-generation/enrollment/recovery paths; do not substitute NTS cookies or packet nonces for human MFA factors. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-7.4.4

| Field | Value |
| --- | --- |
| Source shortcode | V7.4.4 |
| Minimum assigned level | L2 |
| Chapter | Session Management |
| Section | Session Termination |
| Adapted requirement summary | Provide a visible logout action on protected pages. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | There are no authenticated web pages or app login/logout UI. Native command completion/disconnect disposes its authority; closing the UI is not Windows logout or termination of unattended synchronization. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-7.4.5

| Field | Value |
| --- | --- |
| Source shortcode | V7.4.5 |
| Minimum assigned level | L2 |
| Chapter | Session Management |
| Section | Session Termination |
| Adapted requirement summary | Allow administrators to terminate one user's sessions or all sessions. |
| Proposed applicability | Native adaptation (proposed) |
| Literal standard scope | User-session requirement adapted to privileged IPC lifecycle |
| Applicability rationale | Native adaptation requires a documented way to revoke active interactive IPC bindings for a principal or all clients. Stopping the machine service is not automatically equivalent to a per-user termination operation. |
| Native control mapping | CTRL-SESSION: bounded privileged command lifetime, reconnect authentication and disposal; CTRL-IPC. |
| Decision / implementation owner | IPC; helper; security owner |
| Original planned verification | Exercise the implemented administrative invalidation path for one principal and all principals; verify later requests fail without affecting unrelated identities beyond the requested scope. |
| Open question | Choose a minimal administrative IPC-disconnect mechanism or document that no reusable privileged user session exists; do not assume an unimplemented session dashboard. |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-7.5.1

| Field | Value |
| --- | --- |
| Source shortcode | V7.5.1 |
| Minimum assigned level | L2 |
| Chapter | Session Management |
| Section | Defenses Against Session Abuse |
| Adapted requirement summary | Require full reauthentication before sensitive account-profile changes. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application password/account credential requirement |
| Applicability rationale | The product has no account email, phone, factor settings or recovery profile. Administrative machine configuration is a different sensitive-action boundary covered by V7.5.3 and V8. |
| Native control mapping | CTRL-SCOPE: no application credential UI/store/recovery; CTRL-IPC: Windows identity. |
| Decision / implementation owner | Architecture; Windows integration |
| Original planned verification | Inspect UI, IPC schemas, persistence and installer for password/account pathways; separately verify Windows peer authentication and authorization. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-7.5.2

| Field | Value |
| --- | --- |
| Source shortcode | V7.5.2 |
| Minimum assigned level | L2 |
| Chapter | Session Management |
| Section | Defenses Against Session Abuse |
| Adapted requirement summary | Let users inspect and reauthenticate to terminate their active sessions. |
| Proposed applicability | Needs decision |
| Literal standard scope | Literal requirement or applicability needs an explicit security decision |
| Applicability rationale | No application login-session dashboard is in the minimal UI. The design may eliminate reusable privileged user sessions, but persistent IPC bindings still require a clearly scoped decision rather than silent N/A. |
| Native control mapping | CTRL-DECISION-SESSION: explicit user-session applicability; minimal authority lifetime; no invented UI feature or conformance claim. |
| Decision / implementation owner | Security owner; architecture |
| Original planned verification | Inventory reusable authority and user-visible connections; decide whether user inspection/termination is required or whether the literal user-session feature is absent with documented rationale. |
| Open question | Does the final IPC model create persistent user authority requiring a session-management view, or are all privileged actions one-shot and nonreusable? |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-7.5.3

| Field | Value |
| --- | --- |
| Source shortcode | V7.5.3 |
| Minimum assigned level | L3 |
| Chapter | Session Management |
| Section | Defenses Against Session Abuse |
| Adapted requirement summary | Require fresh authentication or secondary verification for sensitive actions. |
| Proposed applicability | Native adaptation (proposed) |
| Literal standard scope | Authentication/session requirement adapted to local Windows identities |
| Applicability rationale | Changing clock policy, trust settings or bootstrap mode requires a deliberately invoked privileged helper and policy-defined confirmation. UAC behavior is assessed as secondary verification only where actually justified; it is not automatically a factor or hardware MFA. |
| Native control mapping | CTRL-IPC: secured local named pipes and verified peer token; CTRL-AUTHZ: per-action authorization. |
| Decision / implementation owner | Windows integration; security owner |
| Original planned verification | Exercise low-privilege, already elevated, cancelled, stale and hostile invocation paths; prove the protected action enforces the approved fresh-authority/secondary-verification rule. |
| Open question | Define which actions require what fresh verification, including behavior when the caller is already elevated. |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-7.6.1

| Field | Value |
| --- | --- |
| Source shortcode | V7.6.1 |
| Minimum assigned level | L2 |
| Chapter | Session Management |
| Section | Federated Re-authentication |
| Adapted requirement summary | Coordinate federated session lifetime and termination as documented. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Federated login/IdP-specific requirement |
| Applicability rationale | The app does not consume SAML/OIDC authentication assertions or manage federated application sessions. Windows domain identity is not implemented as an app SSO protocol. |
| Native control mapping | CTRL-SCOPE: no application federation; stable Windows SIDs and peer tokens are assessed separately. |
| Decision / implementation owner | Architecture; Windows integration |
| Original planned verification | Review login/IPC identity flows and dependencies for absent federation; reopen if app-level IdP/SSO is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-7.6.2

| Field | Value |
| --- | --- |
| Source shortcode | V7.6.2 |
| Minimum assigned level | L2 |
| Chapter | Session Management |
| Section | Federated Re-authentication |
| Adapted requirement summary | Require consent or explicit action before creating a user session. |
| Proposed applicability | Native adaptation (proposed) |
| Literal standard scope | User-session requirement adapted to privileged IPC lifecycle |
| Applicability rationale | Interactive privileged work starts from a deliberate user action. Machine synchronization starts under explicit installation/configuration authorization and is not relabeled as a human login session. |
| Native control mapping | CTRL-SESSION: bounded privileged command lifetime, reconnect authentication and disposal; CTRL-IPC. |
| Decision / implementation owner | IPC; helper; security owner |
| Original planned verification | Attempt unsolicited privileged IPC setup and UI activation; verify no authority or sensitive action is granted merely from a notification, status read or service startup. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |


## V8 - Authorization

### v5.0.0-8.1.1

| Field | Value |
| --- | --- |
| Source shortcode | V8.1.1 |
| Minimum assigned level | L1 |
| Chapter | Authorization |
| Section | Authorization Documentation |
| Adapted requirement summary | Document authorization for functions and individual data objects. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic architecture/documentation requirement |
| Applicability rationale | Document the action/resource matrix for ordinary status reads, per-user UI preferences, machine policy, trust data, installer changes and broker corrections. |
| Native control mapping | CTRL-DOC: versioned architecture, trust-boundary and policy inventories. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Review every IPC operation and persisted resource against allowed Windows identities, required privilege and deny behavior. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-8.1.2

| Field | Value |
| --- | --- |
| Source shortcode | V8.1.2 |
| Minimum assigned level | L2 |
| Chapter | Authorization |
| Section | Authorization Documentation |
| Adapted requirement summary | Document read and write permissions for individual fields. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic architecture/documentation requirement |
| Applicability rationale | Status projections and administrative configuration have field-specific sensitivity; admission of a request must not authorize all fields automatically. |
| Native control mapping | CTRL-DOC: versioned architecture, trust-boundary and policy inventories. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Trace each schema field to its policy, including source names, paths, diagnostics, bootstrap flags and clock limits; review unauthorized-field rejection. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-8.1.3

| Field | Value |
| --- | --- |
| Source shortcode | V8.1.3 |
| Minimum assigned level | L3 |
| Chapter | Authorization |
| Section | Authorization Documentation |
| Adapted requirement summary | Document environmental and contextual authorization attributes. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic architecture/documentation requirement |
| Applicability rationale | Document actual Windows token SID, integrity/session attributes, machine policy and operation state used for decisions. Do not claim network location or clock time alone proves trust. |
| Native control mapping | CTRL-DOC: versioned architecture, trust-boundary and policy inventories. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Compare policy attributes with real Windows/IPC data sources; explicitly identify unavailable or intentionally unused device-posture/risk information. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-8.1.4

| Field | Value |
| --- | --- |
| Source shortcode | V8.1.4 |
| Minimum assigned level | L3 |
| Chapter | Authorization |
| Section | Authorization Documentation |
| Adapted requirement summary | Document how context changes authorization or authentication decisions. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic architecture/documentation requirement |
| Applicability rationale | Describe allow, deny and elevation/secondary-verification decisions for the native model, including stale authority and machine-state changes; unresolved adaptive or posture assurance remains explicit. |
| Native control mapping | CTRL-DOC: versioned architecture, trust-boundary and policy inventories. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Review context thresholds and transitions against V8.2.4 and V8.4.2 decisions; require coverage for each documented branch. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-8.2.1

| Field | Value |
| --- | --- |
| Source shortcode | V8.2.1 |
| Minimum assigned level | L1 |
| Chapter | Authorization |
| Section | General Authorization Design |
| Adapted requirement summary | Enforce explicit function-level permission. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic authorization requirement |
| Applicability rationale | Windows identity and per-action permissions protect global clock and machine configuration changes. |
| Native control mapping | CTRL-AUTHZ: trusted service checks caller token, action and fields; no authority from UI claims. |
| Decision / implementation owner | Broker; privileged helper; IPC |
| Original planned verification | Test real permitted/denied Windows principals and field/action combinations at the protected service; verify rejected requests cannot cause effects. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.1, THR-20:** MSI maintenance/rollback data crosses into a more privileged intermediary and can race privileged filesystem effects CHECK-13/15/23: hostile public properties, supported/unsupported transforms, rollback records and user-controlled paths through actual install/repair/remove entrypoints cannot trigger arbitrary service/file/process effects; forward and rollback use effective caller/operation authority and stable validated handles; normal complete lifecycle succeeds
- **Revision 1.1, THR-22:** A valid old session/token can preserve permissions after policy or role removal CHECK-2/3/4: revoke delegation during a live real IPC session and before queued sensitive action; test product-policy revision separately from existing Windows token group membership; prohibit effect after effective revocation and record any unresolved literal immediate-revocation gap without calling a later correction equivalent
- **Revision 1.2, THR-25:** Scalar/domain validity does not establish caller permission, trusted provenance, current ownership or an allowed resolved destination. CHECK-25 with CHECK-2/3/6/10: valid DTOs with serialized admin/validated claims, revoked or stale context, prohibited resolved/negotiated destinations and a changed resource before effect cannot gain authority. Check current state in the protected operation using the real enforcement path; a fresh authorized permitted operation succeeds. Pure validator tests do not qualify Windows authorization.

### v5.0.0-8.2.2

| Field | Value |
| --- | --- |
| Source shortcode | V8.2.2 |
| Minimum assigned level | L1 |
| Chapter | Authorization |
| Section | General Authorization Design |
| Adapted requirement summary | Enforce explicit permission for each requested data object. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic authorization requirement |
| Applicability rationale | Windows identity and per-action permissions protect global clock and machine configuration changes. |
| Native control mapping | CTRL-AUTHZ: trusted service checks caller token, action and fields; no authority from UI claims. |
| Decision / implementation owner | Broker; privileged helper; IPC |
| Original planned verification | Test real permitted/denied Windows principals and field/action combinations at the protected service; verify rejected requests cannot cause effects. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-8.2.3

| Field | Value |
| --- | --- |
| Source shortcode | V8.2.3 |
| Minimum assigned level | L2 |
| Chapter | Authorization |
| Section | General Authorization Design |
| Adapted requirement summary | Enforce explicit read/write permission for individual fields. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic authorization requirement |
| Applicability rationale | Windows identity and per-action permissions protect global clock and machine configuration changes. |
| Native control mapping | CTRL-AUTHZ: trusted service checks caller token, action and fields; no authority from UI claims. |
| Decision / implementation owner | Broker; privileged helper; IPC |
| Original planned verification | Test real permitted/denied Windows principals and field/action combinations at the protected service; verify rejected requests cannot cause effects. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-8.2.4

| Field | Value |
| --- | --- |
| Source shortcode | V8.2.4 |
| Minimum assigned level | L3 |
| Chapter | Authorization |
| Section | General Authorization Design |
| Adapted requirement summary | Apply documented contextual controls at session start and during use. |
| Proposed applicability | Native adaptation (proposed) |
| Literal standard scope | Adaptive authentication/authorization requirement adapted to supported Windows context |
| Applicability rationale | Native adaptation evaluates supported identity/integrity, session and current machine-policy conditions at each protected request, not just pipe connection. Broader device-posture or adaptive-risk assurance is not established. |
| Native control mapping | CTRL-IPC: secured local named pipes and verified peer token; CTRL-AUTHZ: per-action authorization. |
| Decision / implementation owner | Windows integration; security owner |
| Original planned verification | Change allowed machine context and request state during an existing connection; verify the next protected action observes current policy. Resolve any claimed posture/risk controls with V8.4.2. |
| Open question | Which contextual attributes are both trustworthy and practical on unmanaged Windows machines, and how are they refreshed? |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-8.3.1

| Field | Value |
| --- | --- |
| Source shortcode | V8.3.1 |
| Minimum assigned level | L1 |
| Chapter | Authorization |
| Section | Operation Level Authorization |
| Adapted requirement summary | Enforce authorization in a trusted service layer. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic authorization requirement |
| Applicability rationale | Windows identity and per-action permissions protect global clock and machine configuration changes. |
| Native control mapping | CTRL-AUTHZ: trusted service checks caller token, action and fields; no authority from UI claims. |
| Decision / implementation owner | Broker; privileged helper; IPC |
| Original planned verification | Test real permitted/denied Windows principals and field/action combinations at the protected service; verify rejected requests cannot cause effects. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.1, THR-20:** MSI maintenance/rollback data crosses into a more privileged intermediary and can race privileged filesystem effects CHECK-13/15/23: hostile public properties, supported/unsupported transforms, rollback records and user-controlled paths through actual install/repair/remove entrypoints cannot trigger arbitrary service/file/process effects; forward and rollback use effective caller/operation authority and stable validated handles; normal complete lifecycle succeeds
- **Revision 1.1, THR-22:** A valid old session/token can preserve permissions after policy or role removal CHECK-2/3/4: revoke delegation during a live real IPC session and before queued sensitive action; test product-policy revision separately from existing Windows token group membership; prohibit effect after effective revocation and record any unresolved literal immediate-revocation gap without calling a later correction equivalent
- **Revision 1.2, THR-25:** Scalar/domain validity does not establish caller permission, trusted provenance, current ownership or an allowed resolved destination. CHECK-25 with CHECK-2/3/6/10: valid DTOs with serialized admin/validated claims, revoked or stale context, prohibited resolved/negotiated destinations and a changed resource before effect cannot gain authority. Check current state in the protected operation using the real enforcement path; a fresh authorized permitted operation succeeds. Pure validator tests do not qualify Windows authorization.

### v5.0.0-8.3.2

| Field | Value |
| --- | --- |
| Source shortcode | V8.3.2 |
| Minimum assigned level | L3 |
| Chapter | Authorization |
| Section | Operation Level Authorization |
| Adapted requirement summary | Apply permission changes immediately or meet the specified mitigation. |
| Proposed applicability | Needs decision |
| Literal standard scope | Literal requirement or applicability needs an explicit security decision |
| Applicability rationale | Windows access tokens may retain old group/privilege facts. Current policy checks are necessary but do not by themselves refresh account membership. Alert-and-revert is insufficient for irreversible clock effects or leaked data. |
| Native control mapping | CTRL-DECISION-REVOCATION: fresh authorization policy, token lifecycle and no unsupported rollback equivalence. |
| Decision / implementation owner | Security owner; architecture |
| Original planned verification | Test group/permission changes against existing handles and tokens; specify revocation source and exact commit semantics. Do not accept rollback claims for irreversible disclosure or time changes. |
| Open question | Can immediate effective revocation be guaranteed for each authority source; if not, what explicit scoped deviation remains? |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.1, THR-22:** A valid old session/token can preserve permissions after policy or role removal CHECK-2/3/4: revoke delegation during a live real IPC session and before queued sensitive action; test product-policy revision separately from existing Windows token group membership; prohibit effect after effective revocation and record any unresolved literal immediate-revocation gap without calling a later correction equivalent

### v5.0.0-8.3.3

| Field | Value |
| --- | --- |
| Source shortcode | V8.3.3 |
| Minimum assigned level | L3 |
| Chapter | Authorization |
| Section | Operation Level Authorization |
| Adapted requirement summary | Preserve the originating subject's authority through intermediaries. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic authorization requirement |
| Applicability rationale | Human-initiated privileged operations must be authorized for the actual originator before a helper/service acts. Autonomous engine corrections use separately approved machine policy, not fabricated human authority. |
| Native control mapping | CTRL-AUTHZ: trusted service checks caller token, action and fields; no authority from UI claims. |
| Decision / implementation owner | Broker; privileged helper; IPC |
| Original planned verification | Attempt confused-deputy requests through UI, helper and engine; verify an unprivileged origin cannot borrow service authority, and approved machine work is bounded by its separate policy. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.1, THR-20:** MSI maintenance/rollback data crosses into a more privileged intermediary and can race privileged filesystem effects CHECK-13/15/23: hostile public properties, supported/unsupported transforms, rollback records and user-controlled paths through actual install/repair/remove entrypoints cannot trigger arbitrary service/file/process effects; forward and rollback use effective caller/operation authority and stable validated handles; normal complete lifecycle succeeds

### v5.0.0-8.4.1

| Field | Value |
| --- | --- |
| Source shortcode | V8.4.1 |
| Minimum assigned level | L2 |
| Chapter | Authorization |
| Section | Other Authorization Considerations |
| Adapted requirement summary | Isolate operations between unauthorized tenants. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | This is a single-machine time service, not a multi-tenant application; the host clock is intentionally shared. Windows multi-user authorization remains mandatory and is not represented as tenant-isolated time. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-8.4.2

| Field | Value |
| --- | --- |
| Source shortcode | V8.4.2 |
| Minimum assigned level | L3 |
| Chapter | Authorization |
| Section | Other Authorization Considerations |
| Adapted requirement summary | Layer admin access with continuing identity, posture and context checks. |
| Proposed applicability | Needs decision |
| Literal standard scope | Literal requirement or applicability needs an explicit security decision |
| Applicability rationale | Secured local pipes, SIDs and UAC provide useful layers but do not establish continuous device-posture assessment and contextual risk analysis demanded by the literal L3 clause. |
| Native control mapping | CTRL-DECISION-ADMIN: identity/privilege isolation plus explicit device-posture and contextual-risk applicability decision. |
| Decision / implementation owner | Security owner; architecture |
| Original planned verification | Assess actual supported posture signals and ongoing identity checks in the target Windows deployment; document absent guarantees and approved compensations. |
| Open question | Require an enterprise posture integration, define a narrower native adaptation, or record an explicit deviation for unmanaged desktops? |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |


## V9 - Self-contained Tokens

### v5.0.0-9.1.1

| Field | Value |
| --- | --- |
| Source shortcode | V9.1.1 |
| Minimum assigned level | L1 |
| Chapter | Self-contained Tokens |
| Section | Token source and integrity |
| Adapted requirement summary | Verify self-contained token integrity before accepting contents. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Self-contained application token issuer/consumer requirement |
| Applicability rationale | The app issues or interprets no JWT/JWS or similar self-contained authorization tokens. NTS cookies are opaque server-issued protocol data; this client is not their claims decoder or issuer. |
| Native control mapping | CTRL-SCOPE: no application bearer-token authorization; CTRL-NTS: cookie binding/lifecycle under protocol controls. |
| Decision / implementation owner | Architecture; protocol/core |
| Original planned verification | Review token/IPC schemas and NTS cookie handling; confirm opaque cookies never become user or broker authorization claims. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-9.1.2

| Field | Value |
| --- | --- |
| Source shortcode | V9.1.2 |
| Minimum assigned level | L1 |
| Chapter | Self-contained Tokens |
| Section | Token source and integrity |
| Adapted requirement summary | Allowlist token algorithms and prevent algorithm/key confusion. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Self-contained application token issuer/consumer requirement |
| Applicability rationale | The app issues or interprets no JWT/JWS or similar self-contained authorization tokens. NTS cookies are opaque server-issued protocol data; this client is not their claims decoder or issuer. |
| Native control mapping | CTRL-SCOPE: no application bearer-token authorization; CTRL-NTS: cookie binding/lifecycle under protocol controls. |
| Decision / implementation owner | Architecture; protocol/core |
| Original planned verification | Review token/IPC schemas and NTS cookie handling; confirm opaque cookies never become user or broker authorization claims. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-9.1.3

| Field | Value |
| --- | --- |
| Source shortcode | V9.1.3 |
| Minimum assigned level | L1 |
| Chapter | Self-contained Tokens |
| Section | Token source and integrity |
| Adapted requirement summary | Use trusted configured token-verification keys. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Self-contained application token issuer/consumer requirement |
| Applicability rationale | The app issues or interprets no JWT/JWS or similar self-contained authorization tokens. NTS cookies are opaque server-issued protocol data; this client is not their claims decoder or issuer. |
| Native control mapping | CTRL-SCOPE: no application bearer-token authorization; CTRL-NTS: cookie binding/lifecycle under protocol controls. |
| Decision / implementation owner | Architecture; protocol/core |
| Original planned verification | Review token/IPC schemas and NTS cookie handling; confirm opaque cookies never become user or broker authorization claims. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-9.2.1

| Field | Value |
| --- | --- |
| Source shortcode | V9.2.1 |
| Minimum assigned level | L1 |
| Chapter | Self-contained Tokens |
| Section | Token content |
| Adapted requirement summary | Enforce token validity intervals. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Self-contained application token issuer/consumer requirement |
| Applicability rationale | The app issues or interprets no JWT/JWS or similar self-contained authorization tokens. NTS cookies are opaque server-issued protocol data; this client is not their claims decoder or issuer. |
| Native control mapping | CTRL-SCOPE: no application bearer-token authorization; CTRL-NTS: cookie binding/lifecycle under protocol controls. |
| Decision / implementation owner | Architecture; protocol/core |
| Original planned verification | Review token/IPC schemas and NTS cookie handling; confirm opaque cookies never become user or broker authorization claims. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-9.2.2

| Field | Value |
| --- | --- |
| Source shortcode | V9.2.2 |
| Minimum assigned level | L2 |
| Chapter | Self-contained Tokens |
| Section | Token content |
| Adapted requirement summary | Accept only the intended token type and purpose. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Self-contained application token issuer/consumer requirement |
| Applicability rationale | The app issues or interprets no JWT/JWS or similar self-contained authorization tokens. NTS cookies are opaque server-issued protocol data; this client is not their claims decoder or issuer. |
| Native control mapping | CTRL-SCOPE: no application bearer-token authorization; CTRL-NTS: cookie binding/lifecycle under protocol controls. |
| Decision / implementation owner | Architecture; protocol/core |
| Original planned verification | Review token/IPC schemas and NTS cookie handling; confirm opaque cookies never become user or broker authorization claims. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-9.2.3

| Field | Value |
| --- | --- |
| Source shortcode | V9.2.3 |
| Minimum assigned level | L2 |
| Chapter | Self-contained Tokens |
| Section | Token content |
| Adapted requirement summary | Restrict accepted tokens to the intended audience. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Self-contained application token issuer/consumer requirement |
| Applicability rationale | The app issues or interprets no JWT/JWS or similar self-contained authorization tokens. NTS cookies are opaque server-issued protocol data; this client is not their claims decoder or issuer. |
| Native control mapping | CTRL-SCOPE: no application bearer-token authorization; CTRL-NTS: cookie binding/lifecycle under protocol controls. |
| Decision / implementation owner | Architecture; protocol/core |
| Original planned verification | Review token/IPC schemas and NTS cookie handling; confirm opaque cookies never become user or broker authorization claims. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-9.2.4

| Field | Value |
| --- | --- |
| Source shortcode | V9.2.4 |
| Minimum assigned level | L2 |
| Chapter | Self-contained Tokens |
| Section | Token content |
| Adapted requirement summary | Bind tokens from shared signing keys to unique intended audiences. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Self-contained application token issuer/consumer requirement |
| Applicability rationale | The app issues or interprets no JWT/JWS or similar self-contained authorization tokens. NTS cookies are opaque server-issued protocol data; this client is not their claims decoder or issuer. |
| Native control mapping | CTRL-SCOPE: no application bearer-token authorization; CTRL-NTS: cookie binding/lifecycle under protocol controls. |
| Decision / implementation owner | Architecture; protocol/core |
| Original planned verification | Review token/IPC schemas and NTS cookie handling; confirm opaque cookies never become user or broker authorization claims. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |


## V10 - OAuth and OIDC

### v5.0.0-10.1.1

| Field | Value |
| --- | --- |
| Source shortcode | V10.1.1 |
| Minimum assigned level | L2 |
| Chapter | OAuth and OIDC |
| Section | Generic OAuth and OIDC Security |
| Adapted requirement summary | Expose OAuth tokens only to components that need them. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.1.2

| Field | Value |
| --- | --- |
| Source shortcode | V10.1.2 |
| Minimum assigned level | L2 |
| Chapter | OAuth and OIDC |
| Section | Generic OAuth and OIDC Security |
| Adapted requirement summary | Bind OAuth flow responses and secrets to the initiating transaction. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.2.1

| Field | Value |
| --- | --- |
| Source shortcode | V10.2.1 |
| Minimum assigned level | L2 |
| Chapter | OAuth and OIDC |
| Section | OAuth Client |
| Adapted requirement summary | Protect OAuth code-flow token requests against browser forgery. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.2.2

| Field | Value |
| --- | --- |
| Source shortcode | V10.2.2 |
| Minimum assigned level | L2 |
| Chapter | OAuth and OIDC |
| Section | OAuth Client |
| Adapted requirement summary | Prevent authorization-server mix-up at OAuth clients. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.2.3

| Field | Value |
| --- | --- |
| Source shortcode | V10.2.3 |
| Minimum assigned level | L3 |
| Chapter | OAuth and OIDC |
| Section | OAuth Client |
| Adapted requirement summary | Request only necessary OAuth scopes and authorizations. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.3.1

| Field | Value |
| --- | --- |
| Source shortcode | V10.3.1 |
| Minimum assigned level | L2 |
| Chapter | OAuth and OIDC |
| Section | OAuth Resource Server |
| Adapted requirement summary | Validate access-token audience at the resource server. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.3.2

| Field | Value |
| --- | --- |
| Source shortcode | V10.3.2 |
| Minimum assigned level | L2 |
| Chapter | OAuth and OIDC |
| Section | OAuth Resource Server |
| Adapted requirement summary | Base delegated authorization on the relevant access-token claims. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.3.3

| Field | Value |
| --- | --- |
| Source shortcode | V10.3.3 |
| Minimum assigned level | L2 |
| Chapter | OAuth and OIDC |
| Section | OAuth Resource Server |
| Adapted requirement summary | Identify token users through stable, nonreassigned claims. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.3.4

| Field | Value |
| --- | --- |
| Source shortcode | V10.3.4 |
| Minimum assigned level | L2 |
| Chapter | OAuth and OIDC |
| Section | OAuth Resource Server |
| Adapted requirement summary | Check required authentication strength and recency in access tokens. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.3.5

| Field | Value |
| --- | --- |
| Source shortcode | V10.3.5 |
| Minimum assigned level | L3 |
| Chapter | OAuth and OIDC |
| Section | OAuth Resource Server |
| Adapted requirement summary | Require sender-bound tokens to resist theft and replay. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.4.1

| Field | Value |
| --- | --- |
| Source shortcode | V10.4.1 |
| Minimum assigned level | L1 |
| Chapter | OAuth and OIDC |
| Section | OAuth Authorization Server |
| Adapted requirement summary | Match redirect URIs exactly to each client's registered allowlist. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.4.2

| Field | Value |
| --- | --- |
| Source shortcode | V10.4.2 |
| Minimum assigned level | L1 |
| Chapter | OAuth and OIDC |
| Section | OAuth Authorization Server |
| Adapted requirement summary | Make authorization codes single-use and revoke on valid reuse. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.4.3

| Field | Value |
| --- | --- |
| Source shortcode | V10.4.3 |
| Minimum assigned level | L1 |
| Chapter | OAuth and OIDC |
| Section | OAuth Authorization Server |
| Adapted requirement summary | Give authorization codes short bounded lifetimes. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.4.4

| Field | Value |
| --- | --- |
| Source shortcode | V10.4.4 |
| Minimum assigned level | L1 |
| Chapter | OAuth and OIDC |
| Section | OAuth Authorization Server |
| Adapted requirement summary | Permit only required OAuth grants and exclude obsolete unsafe grants. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.4.5

| Field | Value |
| --- | --- |
| Source shortcode | V10.4.5 |
| Minimum assigned level | L1 |
| Chapter | OAuth and OIDC |
| Section | OAuth Authorization Server |
| Adapted requirement summary | Protect refresh tokens from replay with the required binding or rotation. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.4.6

| Field | Value |
| --- | --- |
| Source shortcode | V10.4.6 |
| Minimum assigned level | L2 |
| Chapter | OAuth and OIDC |
| Section | OAuth Authorization Server |
| Adapted requirement summary | Require strong PKCE validation for authorization-code grants. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.4.7

| Field | Value |
| --- | --- |
| Source shortcode | V10.4.7 |
| Minimum assigned level | L2 |
| Chapter | OAuth and OIDC |
| Section | OAuth Authorization Server |
| Adapted requirement summary | Constrain untrusted dynamic-client registration and disclose its risk. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.4.8

| Field | Value |
| --- | --- |
| Source shortcode | V10.4.8 |
| Minimum assigned level | L2 |
| Chapter | OAuth and OIDC |
| Section | OAuth Authorization Server |
| Adapted requirement summary | Give refresh tokens an absolute expiration. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.4.9

| Field | Value |
| --- | --- |
| Source shortcode | V10.4.9 |
| Minimum assigned level | L2 |
| Chapter | OAuth and OIDC |
| Section | OAuth Authorization Server |
| Adapted requirement summary | Let authorized users revoke refresh and opaque access tokens. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.4.10

| Field | Value |
| --- | --- |
| Source shortcode | V10.4.10 |
| Minimum assigned level | L2 |
| Chapter | OAuth and OIDC |
| Section | OAuth Authorization Server |
| Adapted requirement summary | Authenticate confidential clients on back-channel requests. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.4.11

| Field | Value |
| --- | --- |
| Source shortcode | V10.4.11 |
| Minimum assigned level | L2 |
| Chapter | OAuth and OIDC |
| Section | OAuth Authorization Server |
| Adapted requirement summary | Assign only necessary scopes to each OAuth client. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.4.12

| Field | Value |
| --- | --- |
| Source shortcode | V10.4.12 |
| Minimum assigned level | L3 |
| Chapter | OAuth and OIDC |
| Section | OAuth Authorization Server |
| Adapted requirement summary | Allow only each client's necessary response modes. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.4.13

| Field | Value |
| --- | --- |
| Source shortcode | V10.4.13 |
| Minimum assigned level | L3 |
| Chapter | OAuth and OIDC |
| Section | OAuth Authorization Server |
| Adapted requirement summary | Combine code grants with pushed authorization requests at L3. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.4.14

| Field | Value |
| --- | --- |
| Source shortcode | V10.4.14 |
| Minimum assigned level | L3 |
| Chapter | OAuth and OIDC |
| Section | OAuth Authorization Server |
| Adapted requirement summary | Issue sender-constrained access tokens at L3. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.4.15

| Field | Value |
| --- | --- |
| Source shortcode | V10.4.15 |
| Minimum assigned level | L3 |
| Chapter | OAuth and OIDC |
| Section | OAuth Authorization Server |
| Adapted requirement summary | Protect server-client authorization details from user tampering. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.4.16

| Field | Value |
| --- | --- |
| Source shortcode | V10.4.16 |
| Minimum assigned level | L3 |
| Chapter | OAuth and OIDC |
| Section | OAuth Authorization Server |
| Adapted requirement summary | Require confidential clients with strong replay-resistant asymmetric authentication. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.5.1

| Field | Value |
| --- | --- |
| Source shortcode | V10.5.1 |
| Minimum assigned level | L2 |
| Chapter | OAuth and OIDC |
| Section | OIDC Client |
| Adapted requirement summary | Prevent replay of OIDC identity tokens. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.5.2

| Field | Value |
| --- | --- |
| Source shortcode | V10.5.2 |
| Minimum assigned level | L2 |
| Chapter | OAuth and OIDC |
| Section | OIDC Client |
| Adapted requirement summary | Identify users by stable nonreassigned identity-token claims. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.5.3

| Field | Value |
| --- | --- |
| Source shortcode | V10.5.3 |
| Minimum assigned level | L2 |
| Chapter | OAuth and OIDC |
| Section | OIDC Client |
| Adapted requirement summary | Match discovery metadata to the configured issuer exactly. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.5.4

| Field | Value |
| --- | --- |
| Source shortcode | V10.5.4 |
| Minimum assigned level | L2 |
| Chapter | OAuth and OIDC |
| Section | OIDC Client |
| Adapted requirement summary | Validate that identity tokens target the receiving client. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.5.5

| Field | Value |
| --- | --- |
| Source shortcode | V10.5.5 |
| Minimum assigned level | L2 |
| Chapter | OAuth and OIDC |
| Section | OIDC Client |
| Adapted requirement summary | Validate back-channel logout token purpose and structure. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.6.1

| Field | Value |
| --- | --- |
| Source shortcode | V10.6.1 |
| Minimum assigned level | L2 |
| Chapter | OAuth and OIDC |
| Section | OpenID Provider |
| Adapted requirement summary | Restrict OpenID responses to supported secure flows. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.6.2

| Field | Value |
| --- | --- |
| Source shortcode | V10.6.2 |
| Minimum assigned level | L2 |
| Chapter | OAuth and OIDC |
| Section | OpenID Provider |
| Adapted requirement summary | Prevent forced logout attacks at the OpenID provider. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.7.1

| Field | Value |
| --- | --- |
| Source shortcode | V10.7.1 |
| Minimum assigned level | L2 |
| Chapter | OAuth and OIDC |
| Section | Consent Management |
| Adapted requirement summary | Obtain appropriate user consent for authorization requests. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.7.2

| Field | Value |
| --- | --- |
| Source shortcode | V10.7.2 |
| Minimum assigned level | L2 |
| Chapter | OAuth and OIDC |
| Section | Consent Management |
| Adapted requirement summary | Explain requested authority, client identity and duration during consent. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-10.7.3

| Field | Value |
| --- | --- |
| Source shortcode | V10.7.3 |
| Minimum assigned level | L2 |
| Chapter | OAuth and OIDC |
| Section | Consent Management |
| Adapted requirement summary | Let users review, change and revoke authorization consents. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | OAuth/OIDC role-specific requirement |
| Applicability rationale | No OAuth client, resource server, authorization server, OpenID provider or consent protocol exists in the declared product. |
| Native control mapping | CTRL-SCOPE: exclude OAuth/OIDC components; Windows IPC authorization remains in V6-V8. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review authentication flows, IPC schemas and dependencies for absence of OAuth/OIDC roles; reopen on scope change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |


## V11 - Cryptography

### v5.0.0-11.1.1

| Field | Value |
| --- | --- |
| Source shortcode | V11.1.1 |
| Minimum assigned level | L2 |
| Chapter | Cryptography |
| Section | Cryptographic Inventory and Documentation |
| Adapted requirement summary | Document standardized key management, lifecycle and holder limits. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic key/secrets management requirement |
| Applicability rationale | Inventory TLS/NTS session keys, cookies, trust anchors and release-signing material with creation, holders, use, rotation and destruction policy. Identify protocol-required sharing and any deviation from the chosen key-management standard. |
| Native control mapping | CTRL-KEYS: key inventory, access separation, rekey/disposal and no plaintext secret outputs; any persisted association requires explicit protected-storage policy. |
| Decision / implementation owner | Crypto/TLS; release; security owner |
| Original planned verification | Review each secret/key lifecycle and actual process holder against the documented standard and threat model; ensure UI, logs and broker never receive unneeded NTS secrets. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-11.1.2

| Field | Value |
| --- | --- |
| Source shortcode | V11.1.2 |
| Minimum assigned level | L2 |
| Chapter | Cryptography |
| Section | Cryptographic Inventory and Documentation |
| Adapted requirement summary | Maintain the inventory of cryptographic keys, algorithms and permitted uses. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic architecture/documentation requirement |
| Applicability rationale | The inventory includes crypto in dependencies, platform providers, installer/signing tooling and trust stores, with allowed uses and protected data classes. |
| Native control mapping | CTRL-DOC: versioned architecture, trust-boundary and policy inventories. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Compare dependency/provider/code discovery with inventory and actual release configuration; require ownership and allowed/prohibited use for each item. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-11.1.3

| Field | Value |
| --- | --- |
| Source shortcode | V11.1.3 |
| Minimum assigned level | L3 |
| Chapter | Cryptography |
| Section | Cryptographic Inventory and Documentation |
| Adapted requirement summary | Discover all cryptographic operations throughout the system. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic cryptographic requirement |
| Applicability rationale | Perform explicit source, dependency and platform-provider discovery, not merely a list of hand-written crypto calls. |
| Native control mapping | CTRL-CRYPTO: reviewed protocol-approved primitives/providers; explicit algorithm and key policy. |
| Decision / implementation owner | Crypto/TLS; security owner |
| Original planned verification | Inspect resolved transitive dependency features and Windows API use; map encryption, hashing, derivation, random generation and signature operations, including release tooling. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-11.1.4

| Field | Value |
| --- | --- |
| Source shortcode | V11.1.4 |
| Minimum assigned level | L3 |
| Chapter | Cryptography |
| Section | Cryptographic Inventory and Documentation |
| Adapted requirement summary | Plan migration of the maintained crypto inventory to newer standards. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic architecture/documentation requirement |
| Applicability rationale | Document protocol/library/provider migration and interoperability constraints, including future approved post-quantum choices. Do not invent a custom NTS algorithm merely to claim agility. |
| Native control mapping | CTRL-DOC: versioned architecture, trust-boundary and policy inventories. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Review upgrade seams, version negotiation and stored-state migration with security and protocol owners; record unsupported transitions and a review trigger when standards change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-11.2.1

| Field | Value |
| --- | --- |
| Source shortcode | V11.2.1 |
| Minimum assigned level | L2 |
| Chapter | Cryptography |
| Section | Secure Cryptography Implementation |
| Adapted requirement summary | Use reviewed, industry-validated cryptographic implementations. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic cryptographic requirement |
| Applicability rationale | Choose established TLS/NTS cryptographic implementations and record evidence of review, supported targets and actual provider configuration. Rust memory safety alone is not cryptographic validation. |
| Native control mapping | CTRL-CRYPTO: reviewed protocol-approved primitives/providers; explicit algorithm and key policy. |
| Decision / implementation owner | Crypto/TLS; security owner |
| Original planned verification | Review exact provider/library versions, security history and architecture support; run independent known-answer and tamper fixtures without describing ordinary unit tests as formal certification. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-11.2.2

| Field | Value |
| --- | --- |
| Source shortcode | V11.2.2 |
| Minimum assigned level | L2 |
| Chapter | Cryptography |
| Section | Secure Cryptography Implementation |
| Adapted requirement summary | Permit safe algorithm, key and protected-data migration. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic cryptographic requirement |
| Applicability rationale | Separate approved provider/policy choices from protocol logic and support key replacement. Agility does not mean arbitrary user-selected ciphers or fallback below NTS/TLS security requirements. |
| Native control mapping | CTRL-CRYPTO: reviewed protocol-approved primitives/providers; explicit algorithm and key policy. |
| Decision / implementation owner | Crypto/TLS; security owner |
| Original planned verification | Exercise a supported key/provider upgrade and state migration; prove unknown/disallowed algorithms fail closed and old secret material is not retained unintentionally. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-11.2.3

| Field | Value |
| --- | --- |
| Source shortcode | V11.2.3 |
| Minimum assigned level | L2 |
| Chapter | Cryptography |
| Section | Secure Cryptography Implementation |
| Adapted requirement summary | Use cryptographic settings providing at least 128-bit security. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic cryptographic requirement |
| Applicability rationale | Audit actual TLS ciphers, key exchanges, certificates, NTS AEAD and release signatures against the required strength. A public trust chain alone does not establish that every key meets this bar. |
| Native control mapping | CTRL-CRYPTO: reviewed protocol-approved primitives/providers; explicit algorithm and key policy. |
| Decision / implementation owner | Crypto/TLS; security owner |
| Original planned verification | Inspect negotiated and verified parameters with allowed and below-policy controlled peers; document root/intermediate/server key policy and protocol interoperability impact. |
| Open question | Decide exact certificate/key-size policy supported by the chosen validator, including any public-PKI compatibility exception. |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-11.2.4

| Field | Value |
| --- | --- |
| Source shortcode | V11.2.4 |
| Minimum assigned level | L3 |
| Chapter | Cryptography |
| Section | Secure Cryptography Implementation |
| Adapted requirement summary | Avoid timing-dependent cryptographic operations and comparisons. |
| Proposed applicability | Needs decision |
| Literal standard scope | Literal requirement or applicability needs an explicit security decision |
| Applicability rationale | Use provider guarantees and secret-independent comparisons, but blanket constant-time behavior across every chosen provider, algorithm, architecture and verification path is not established. Statistical timing tests alone cannot prove the literal requirement. |
| Native control mapping | CTRL-DECISION-CT: provider and secret-dependent control-flow review; no unqualified all-operations constant-time claim. |
| Decision / implementation owner | Security owner; architecture |
| Original planned verification | Review provider design, code paths and supported Windows CPUs; use targeted analysis and negative controls. Record the scope and limits of any constant-time assurance. |
| Open question | What evidence supports the exact provider/algorithm/platform set, and which operations require a documented interpretation or deviation? |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-11.2.5

| Field | Value |
| --- | --- |
| Source shortcode | V11.2.5 |
| Minimum assigned level | L3 |
| Chapter | Cryptography |
| Section | Secure Cryptography Implementation |
| Adapted requirement summary | Make cryptographic failures secure and non-oracular. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic cryptographic requirement |
| Applicability rationale | NTS, TLS, random security identifiers and release authenticity use cryptography. |
| Native control mapping | CTRL-CRYPTO: reviewed protocol-approved primitives/providers; explicit algorithm and key policy. |
| Decision / implementation owner | Crypto/TLS; security owner |
| Original planned verification | Use invalid tags, malformed records, invalid certificates and provider/RNG failures; compare externally observable failure classes and prove no unauthenticated data reaches source selection or clock control. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-11.3.1

| Field | Value |
| --- | --- |
| Source shortcode | V11.3.1 |
| Minimum assigned level | L1 |
| Chapter | Cryptography |
| Section | Encryption Algorithms |
| Adapted requirement summary | Exclude insecure cipher modes and weak padding schemes. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic cryptographic requirement |
| Applicability rationale | NTS, TLS, random security identifiers and release authenticity use cryptography. |
| Native control mapping | CTRL-CRYPTO: reviewed protocol-approved primitives/providers; explicit algorithm and key policy. |
| Decision / implementation owner | Crypto/TLS; security owner |
| Original planned verification | Review resolved library/provider configuration and independent known-answer/tamper controls; test fail-closed behavior and record assurance limits. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-11.3.2

| Field | Value |
| --- | --- |
| Source shortcode | V11.3.2 |
| Minimum assigned level | L1 |
| Chapter | Cryptography |
| Section | Encryption Algorithms |
| Adapted requirement summary | Use approved encryption algorithms and modes. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic cryptographic requirement |
| Applicability rationale | Use protocol-approved TLS and NTS AEAD algorithms through reviewed implementations. Do not substitute a familiar cipher that is not negotiated and supported by the protocol. |
| Native control mapping | CTRL-CRYPTO: reviewed protocol-approved primitives/providers; explicit algorithm and key policy. |
| Decision / implementation owner | Crypto/TLS; security owner |
| Original planned verification | Inspect allowed/negotiated AEAD and cipher policy; run independent accepted/rejected algorithm fixtures and provider known-answer vectors. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-11.3.3

| Field | Value |
| --- | --- |
| Source shortcode | V11.3.3 |
| Minimum assigned level | L2 |
| Chapter | Cryptography |
| Section | Encryption Algorithms |
| Adapted requirement summary | Protect encrypted data against unauthorized modification. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic cryptographic requirement |
| Applicability rationale | TLS records and NTS encrypted/authenticated extension fields require valid integrity protection. NTS does not thereby encrypt every timestamp or header field. |
| Native control mapping | CTRL-CRYPTO: reviewed protocol-approved primitives/providers; explicit algorithm and key policy. |
| Decision / implementation owner | Crypto/TLS; security owner |
| Original planned verification | Tamper with authenticated plaintext, ciphertext, tag, associated data and record framing; verify rejection before sample publication and preserve positive valid-message controls. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-11.3.4

| Field | Value |
| --- | --- |
| Source shortcode | V11.3.4 |
| Minimum assigned level | L3 |
| Chapter | Cryptography |
| Section | Encryption Algorithms |
| Adapted requirement summary | Generate nonces and IVs according to each algorithm's uniqueness rules. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic cryptographic requirement |
| Applicability rationale | Follow the exact TLS/NTS AEAD nonce requirements across key lifetime, restart and rekey. Misuse resistance is not permission to disregard the protocol's required nonce generation. |
| Native control mapping | CTRL-CRYPTO: reviewed protocol-approved primitives/providers; explicit algorithm and key policy. |
| Decision / implementation owner | Crypto/TLS; security owner |
| Original planned verification | Review actual provider nonce ownership and wrap/restart behavior; use forced duplicate/exhaustion and rekey fixtures without substituting ordinary sample timestamps for security nonces. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-11.3.5

| Field | Value |
| --- | --- |
| Source shortcode | V11.3.5 |
| Minimum assigned level | L3 |
| Chapter | Cryptography |
| Section | Encryption Algorithms |
| Adapted requirement summary | Use encrypt-then-MAC when composing separate encryption and MAC algorithms. |
| Proposed applicability | Needs decision |
| Literal standard scope | Literal requirement or applicability needs an explicit security decision |
| Applicability rationale | The design should avoid ad hoc encryption-plus-MAC composition and use protocol-approved AEAD. The literal clause's treatment of a standardized AEAD construction such as AES-SIV needs an explicit interpretation; it is not automatically an implementation defect or automatic compliance. |
| Native control mapping | CTRL-DECISION-AEAD: approved AEAD only; no custom composition; record literal applicability interpretation. |
| Decision / implementation owner | Security owner; architecture |
| Original planned verification | Review the chosen NTS AEAD standard and implementation against this clause; document whether the clause addresses application-level composition or requires a specific exception. |
| Open question | How is this clause scoped to standardized AEAD internals versus independently composed encryption and MAC in this product? |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-11.4.1

| Field | Value |
| --- | --- |
| Source shortcode | V11.4.1 |
| Minimum assigned level | L1 |
| Chapter | Cryptography |
| Section | Hashing and Hash-based Functions |
| Adapted requirement summary | Use approved hashes for all cryptographic purposes. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic cryptographic requirement |
| Applicability rationale | NTS, TLS, random security identifiers and release authenticity use cryptography. |
| Native control mapping | CTRL-CRYPTO: reviewed protocol-approved primitives/providers; explicit algorithm and key policy. |
| Decision / implementation owner | Crypto/TLS; security owner |
| Original planned verification | Review resolved library/provider configuration and independent known-answer/tamper controls; test fail-closed behavior and record assurance limits. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-11.4.2

| Field | Value |
| --- | --- |
| Source shortcode | V11.4.2 |
| Minimum assigned level | L2 |
| Chapter | Cryptography |
| Section | Hashing and Hash-based Functions |
| Adapted requirement summary | Store passwords with an approved, suitably costly password KDF. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application password/account credential requirement |
| Applicability rationale | The app neither creates user accounts nor accepts, stores, changes or recovers passwords; Windows owns interactive identity. This does not waive generic identity/authorization rows. |
| Native control mapping | CTRL-SCOPE: no application credential UI/store/recovery; CTRL-IPC: Windows identity. |
| Decision / implementation owner | Architecture; Windows integration |
| Original planned verification | Inspect UI, IPC schemas, persistence and installer for password/account pathways; separately verify Windows peer authentication and authorization. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-11.4.3

| Field | Value |
| --- | --- |
| Source shortcode | V11.4.3 |
| Minimum assigned level | L2 |
| Chapter | Cryptography |
| Section | Hashing and Hash-based Functions |
| Adapted requirement summary | Use adequately strong hashes for integrity and signatures. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic cryptographic requirement |
| Applicability rationale | Inventory hashes used by TLS, certificates, NTS constructions, release signatures and integrity metadata; distinguish cryptographic authentication from nonsecurity checksums. |
| Native control mapping | CTRL-CRYPTO: reviewed protocol-approved primitives/providers; explicit algorithm and key policy. |
| Decision / implementation owner | Crypto/TLS; security owner |
| Original planned verification | Inspect actual output lengths and algorithm policy; reject prohibited hashes where the application controls validation and record platform trust-chain constraints. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-11.4.4

| Field | Value |
| --- | --- |
| Source shortcode | V11.4.4 |
| Minimum assigned level | L2 |
| Chapter | Cryptography |
| Section | Hashing and Hash-based Functions |
| Adapted requirement summary | Derive password-based secret keys with an approved stretching KDF. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Application password/account credential requirement |
| Applicability rationale | The app neither accepts passwords nor derives application keys from them. OS-owned identity/secret protection is a delegated platform mechanism, not an app password-KDF feature. |
| Native control mapping | CTRL-SCOPE: no application credential UI/store/recovery; CTRL-IPC: Windows identity. |
| Decision / implementation owner | Architecture; Windows integration |
| Original planned verification | Inspect UI, IPC schemas, persistence and installer for password/account pathways; separately verify Windows peer authentication and authorization. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-11.5.1

| Field | Value |
| --- | --- |
| Source shortcode | V11.5.1 |
| Minimum assigned level | L2 |
| Chapter | Cryptography |
| Section | Random Values |
| Adapted requirement summary | Generate security-unpredictable values with sufficient CSPRNG entropy. |
| Proposed applicability | Needs decision |
| Literal standard scope | Generic cryptographic requirement |
| Applicability rationale | Use the approved OS/provider CSPRNG and at least 128 bits of entropy for values intended to be non-guessable where the protocol permits it; the NTS UID has the stronger RFC 8915 minimum of 32 octets. Legacy plain-NTP origin/correlation fields and randomized ports do not individually meet the literal entropy floor. Their non-bearer purpose does not exclude them from this clause. Retaining requested plain-NTP compatibility therefore requires a documented scoped deviation and risk decision under OPEN-9. |
| Native control mapping | CTRL-CRYPTO: CSPRNG and per-value entropy inventory; CTRL-DECISION: explicit legacy plain-NTP entropy deviation, residual off-path risk and owner acceptance. No literal conformity claim until the scoped decision and evidence exist. |
| Decision / implementation owner | Crypto/TLS; protocol; security-policy owner |
| Original planned verification | Trace each security-random value to its generator and requested entropy; test failure propagation and prohibit fallback to time, UUID-only or noncryptographic randomness. |
| Open question | Which legacy plain-NTP unpredictability limits are accepted under OPEN-9/S0.7, and does any optional standalone UID profile have proven interoperability? Preserve NTP support without calling sub-128-bit values compliant or silently requiring an extension every legacy server supports. |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision 1.4 review correction:** The literal entropy requirement applies to all intended non-guessable values, not only bearer credentials; legacy plain NTP needs an explicit exception decision.

**Previous revision 1.3 values retained for traceability:**

- **Previous Proposed applicability:** Directly applicable (proposed)
- **Previous Applicability rationale:** Use the approved OS/provider CSPRNG for NTS/TLS security identifiers and other non-guessable values. Plain-NTP finite timestamp/correlation fields are not bearer authentication secrets and do not gain 128-bit assurance by relabeling.
- **Previous Native control mapping:** CTRL-CRYPTO: reviewed protocol-approved primitives/providers; explicit algorithm and key policy.
- **Previous Decision / implementation owner:** Crypto/TLS; security owner
- **Previous Open question:** None recorded

**Current planned verification — not executed:** OPEN-9/S0.7 and CHECK-4/6/9: inventory each security-random value, its intended unpredictability, generator and actual entropy. Require the RFC 8915 NTS UID to contain at least 32 CSPRNG octets. Qualify entropy-source failure with no weak fallback. Separately document and approve or reject any protocol-constrained plain-NTP deviation; do not add timestamp and port widths to claim per-value 128-bit compliance.

### v5.0.0-11.5.2

| Field | Value |
| --- | --- |
| Source shortcode | V11.5.2 |
| Minimum assigned level | L3 |
| Chapter | Cryptography |
| Section | Random Values |
| Adapted requirement summary | Keep randomness secure under high demand. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic cryptographic requirement |
| Applicability rationale | NTS, TLS, random security identifiers and release authenticity use cryptography. |
| Native control mapping | CTRL-CRYPTO: reviewed protocol-approved primitives/providers; explicit algorithm and key policy. |
| Decision / implementation owner | Crypto/TLS; security owner |
| Original planned verification | Exercise bounded concurrent cryptographic demand and injected RNG/provider failure; verify no weak fallback, unsafe seed reuse or unbounded retry loop. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-11.6.1

| Field | Value |
| --- | --- |
| Source shortcode | V11.6.1 |
| Minimum assigned level | L2 |
| Chapter | Cryptography |
| Section | Public Key Cryptography |
| Adapted requirement summary | Use approved key and signature generation with secure parameters. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic cryptographic requirement |
| Applicability rationale | NTS, TLS, random security identifiers and release authenticity use cryptography. |
| Native control mapping | CTRL-CRYPTO: reviewed protocol-approved primitives/providers; explicit algorithm and key policy. |
| Decision / implementation owner | Crypto/TLS; security owner |
| Original planned verification | Review resolved library/provider configuration and independent known-answer/tamper controls; test fail-closed behavior and record assurance limits. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-11.6.2

| Field | Value |
| --- | --- |
| Source shortcode | V11.6.2 |
| Minimum assigned level | L3 |
| Chapter | Cryptography |
| Section | Public Key Cryptography |
| Adapted requirement summary | Use secure approved key-exchange algorithms and parameters. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic cryptographic requirement |
| Applicability rationale | NTS, TLS, random security identifiers and release authenticity use cryptography. |
| Native control mapping | CTRL-CRYPTO: reviewed protocol-approved primitives/providers; explicit algorithm and key policy. |
| Decision / implementation owner | Crypto/TLS; security owner |
| Original planned verification | Inspect actual TLS 1.3 key-exchange policy and negotiation; use incompatible and below-policy controlled peers, and verify no authentication-bypassing fallback. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-11.7.1

| Field | Value |
| --- | --- |
| Source shortcode | V11.7.1 |
| Minimum assigned level | L3 |
| Chapter | Cryptography |
| Section | In-Use Data Cryptography |
| Adapted requirement summary | Protect sensitive data in use with full memory encryption. |
| Proposed applicability | Needs decision |
| Literal standard scope | Literal requirement or applicability needs an explicit security decision |
| Applicability rationale | Safe Rust, process isolation, DPAPI protection at rest, page locking and zeroization do not demonstrate full memory encryption while data is in use. A practical Windows baseline has not established the literal L3 guarantee. |
| Native control mapping | CTRL-DECISION-MEMORY: explicit in-use confidentiality gap; minimal key holders, bounded lifetime and process isolation do not substitute for full memory encryption. |
| Decision / implementation owner | Security owner; architecture |
| Original planned verification | Determine the actual supported hardware/OS protection and threat boundary; require platform evidence before claiming encrypted-in-use coverage and document a gap otherwise. |
| Open question | Is full memory encryption a deployment prerequisite, or an explicit unmet L3 requirement with data-minimization and isolation compensations? |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-11.7.2

| Field | Value |
| --- | --- |
| Source shortcode | V11.7.2 |
| Minimum assigned level | L3 |
| Chapter | Cryptography |
| Section | In-Use Data Cryptography |
| Adapted requirement summary | Minimize exposed processing data and promptly protect it after use. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic data protection/minimization requirement |
| Applicability rationale | Keep NTS keys/cookies in the minimum owning process and scope; zeroize owned buffers where supported. If saved associations are required, use explicitly protected storage with bounded lifetime and integrity/rollback policy. Library copies, crash dumps and shared pages need review; disposal is not proof of all-memory encryption. |
| Native control mapping | CTRL-DATA: classification and minimum projections; CTRL-KEYS; CTRL-LOG. |
| Decision / implementation owner | Security owner; storage; observability |
| Original planned verification | Trace secret lifetimes and copies through real library boundaries; inspect actual persisted-association, dump/log and deletion policies with sentinel-secret controls; document unobservable copies and disposal limits. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |


## V12 - Secure Communication

### v5.0.0-12.1.1

| Field | Value |
| --- | --- |
| Source shortcode | V12.1.1 |
| Minimum assigned level | L1 |
| Chapter | Secure Communication |
| Section | General TLS Security Guidance |
| Adapted requirement summary | Use only recommended TLS versions and prefer the newest supported one. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic TLS client requirement |
| Applicability rationale | NTS-KE uses the protocol-required TLS 1.3 policy. There is no automatic downgrade to earlier TLS or unauthenticated trusted-time operation. |
| Native control mapping | CTRL-TLS: TLS 1.3 policy and verified server identity; CTRL-CERT: explicit trust and validation. |
| Decision / implementation owner | Crypto/TLS |
| Original planned verification | Attempt TLS 1.2-only and TLS 1.3 controlled peers; inspect the actual negotiated version and prove version failure cannot silently downgrade trust. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-12.1.2

| Field | Value |
| --- | --- |
| Source shortcode | V12.1.2 |
| Minimum assigned level | L2 |
| Chapter | Secure Communication |
| Section | General TLS Security Guidance |
| Adapted requirement summary | Allow only strong TLS suites with forward secrecy at L3. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic TLS client requirement |
| Applicability rationale | The NTS-KE client uses TLS and must enforce its actual configured security policy. |
| Native control mapping | CTRL-TLS: TLS 1.3 policy and verified server identity; CTRL-CERT: explicit trust and validation. |
| Decision / implementation owner | Crypto/TLS |
| Original planned verification | Inspect the actual TLS 1.3 cipher/key-exchange and resumption policy, including PSK-only modes if exposed; verify every supported mode meets the chosen forward-secrecy requirement. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-12.1.3

| Field | Value |
| --- | --- |
| Source shortcode | V12.1.3 |
| Minimum assigned level | L2 |
| Chapter | Secure Communication |
| Section | General TLS Security Guidance |
| Adapted requirement summary | Validate mTLS client certificates before using their identity. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | The product is an NTS-KE TLS client and does not accept mTLS client certificates or provide a client-certificate-authenticated server. Adding such a role reopens this row. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-12.1.4

| Field | Value |
| --- | --- |
| Source shortcode | V12.1.4 |
| Minimum assigned level | L3 |
| Chapter | Secure Communication |
| Section | General TLS Security Guidance |
| Adapted requirement summary | Configure effective certificate-revocation checking. |
| Proposed applicability | Needs decision |
| Literal standard scope | Literal requirement or applicability needs an explicit security decision |
| Applicability rationale | The selected TLS validator/provider, NTS-KE peers and revocation data sources are not yet fixed. Support for TLS or chain/hostname validation does not by itself establish OCSP/CRL or stapled-revocation checking. |
| Native control mapping | CTRL-DECISION-REVOCATION-TLS: provider/peer capability evidence and explicit revocation policy; no assumption that TLS-library defaults satisfy it. |
| Decision / implementation owner | Security owner; architecture |
| Original planned verification | Test actual revoked, valid, unknown-status, stale-response and unreachable-responder cases, including uncertain time. Record revocation fetch, freshness and failure policy with the chosen provider. |
| Open question | Which revocation mechanism and hard/soft-failure policy work with supported NTS peers, Windows trust integration and bootstrap uncertainty? |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-12.1.5

| Field | Value |
| --- | --- |
| Source shortcode | V12.1.5 |
| Minimum assigned level | L3 |
| Chapter | Secure Communication |
| Section | General TLS Security Guidance |
| Adapted requirement summary | Enable encrypted ClientHello to hide TLS handshake metadata. |
| Proposed applicability | Needs decision |
| Literal standard scope | Literal requirement or applicability needs an explicit security decision |
| Applicability rationale | ECH support and configuration for the chosen TLS library/provider, DNS discovery and NTS-KE servers are unverified. Non-HTTP TLS is not a reason to silently mark this generic TLS clause N/A. |
| Native control mapping | CTRL-DECISION-ECH: capability and interoperability assessment; no unproven ECH-enabled claim. |
| Decision / implementation owner | Security owner; architecture |
| Original planned verification | Verify actual provider and peer ECH capabilities and inspect a controlled handshake capture; test required/unsupported ECH policy and document remaining SNI exposure. |
| Open question | Is interoperable ECH available for the supported NTS-KE deployment, or must metadata exposure remain an explicit L3 gap? |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-12.2.1

| Field | Value |
| --- | --- |
| Source shortcode | V12.2.1 |
| Minimum assigned level | L1 |
| Chapter | Secure Communication |
| Section | HTTPS Communication with External Facing Services |
| Adapted requirement summary | Encrypt all external HTTP communication without insecure fallback. |
| Proposed applicability | Needs decision |
| Literal standard scope | HTTP/web-service-specific requirement |
| Applicability rationale | The currently held Windows verifier may retrieve external AIA/root/CRL/OCSP material directly or through Windows services. HTTP-specific applicability depends on the admitted route and its actual schemes, ownership and policy. This is a present OPEN-8 qualification question, not only a future feature. |
| Native control mapping | CTRL-DECISION, CTRL-CERT, CTRL-EGRESS: explicit native/delegated retrieval scope and policy; CTRL-INPUT/CTRL-RESOURCE where this clause requires framing or length enforcement. No implemented control or platform guarantee is inferred. |
| Decision / implementation owner | Security; Windows platform; protocol; QA |
| Original planned verification | Inspect network-entry inventory and packaged dependencies; confirm absence of application HTTP routes; reassess if update/revocation HTTP integrations are introduced. |
| Open question | Which external retrievals use TLS, can any use or fall back to plain HTTP, and what explicit scoped exception and residual risk is required if signed PKI material intentionally travels over HTTP? |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision 1.4 review correction:** Reconcile a current held direct/delegated certificate-retrieval candidate with the exact clause; no runtime failure is asserted.

**Previous revision 1.3 values retained for traceability:**

- **Previous Proposed applicability:** Proposed N/A
- **Previous Applicability rationale:** There is no application HTTP service or workflow in the baseline. Certificate-revocation retrieval or future update/help integrations may create HTTP-specific applicability and must reopen this row.
- **Previous Native control mapping:** CTRL-SCOPE: no application HTTP endpoint; protocol framing and IPC remain covered separately.
- **Previous Decision / implementation owner:** Architecture; QA
- **Previous Open question:** None recorded

**Current planned verification — not executed:** OPEN-8/S0.7, CHECK-4/9/27: Inventory direct and delegated retrieval schemes and identities; test downgrade, offline/cache behavior and allowed/denied retrieval. Signed PKI objects do not themselves satisfy a literal encrypted-transport requirement. A route with no in-scope external HTTP requires evidence before N/A is retained.

### v5.0.0-12.2.2

| Field | Value |
| --- | --- |
| Source shortcode | V12.2.2 |
| Minimum assigned level | L1 |
| Chapter | Secure Communication |
| Section | HTTPS Communication with External Facing Services |
| Adapted requirement summary | Use publicly trusted certificates for external-facing services. |
| Proposed applicability | Native adaptation (proposed) |
| Literal standard scope | External-facing service certificate requirement adapted to an NTS-KE client |
| Applicability rationale | Native adaptation applies public-trust expectations to external NTS-KE server authentication; this client does not expose an external HTTP service. Any administrator-selected private CA is a separately documented trust mode. |
| Native control mapping | CTRL-CERT: default public trust and protected explicit private trust; native client adaptation of an external-facing service requirement. |
| Decision / implementation owner | Crypto/TLS |
| Original planned verification | Test public-chain, untrusted, name-mismatch and explicitly configured private-root peers; ensure private trust cannot be added by an ordinary status client. |
| Open question | Will private/internal NTS-KE servers be supported through explicit protected trust configuration? |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-12.3.1

| Field | Value |
| --- | --- |
| Source shortcode | V12.3.1 |
| Minimum assigned level | L2 |
| Chapter | Secure Communication |
| Section | General Service to Service Communication Security |
| Adapted requirement summary | Encrypt every application connection without insecure fallback. |
| Proposed applicability | Needs decision |
| Literal standard scope | Literal requirement or applicability needs an explicit security decision |
| Applicability rationale | TLS protects NTS-KE, but NTS authenticates rather than encrypts the whole NTP packet: base headers and timestamps remain visible. Local named pipes are access-controlled, not thereby transport-encrypted. Optional plain NTP bootstrap hints create an additional explicit exception. |
| Native control mapping | CTRL-DECISION-CHANNELS: per-channel confidentiality/authenticity matrix; no assertion that NTS AEAD encrypts the entire payload or that pipe ACLs are TLS. |
| Decision / implementation owner | Security owner; architecture |
| Original planned verification | Inventory and capture every actual channel; distinguish confidential, authenticated-only and unauthenticated fields. Verify bootstrap opt-in and that hints alone never authorize clock changes or secure-time status. |
| Open question | What scoped exceptions are accepted for NTP/NTS wire semantics, local pipes and optional bootstrap hints; what residual risks remain? |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-12.3.2

| Field | Value |
| --- | --- |
| Source shortcode | V12.3.2 |
| Minimum assigned level | L2 |
| Chapter | Secure Communication |
| Section | General Service to Service Communication Security |
| Adapted requirement summary | Validate server certificates before trusting TLS application communication. |
| Proposed applicability | Needs decision |
| Literal standard scope | Literal requirement or applicability needs an explicit security decision |
| Applicability rationale | Strict mode requires normal chain, name, signature and validity checks. Optional bootstrap hints may influence a candidate validation time without bypassing these checks; whether that establishes sufficient temporal assurance is a separate unresolved design question, not automatic proof of invalid certificate checking. |
| Native control mapping | CTRL-DECISION-BOOTSTRAP: strict default certificate validation; explicit optional candidate-time procedure and residual uncertainty; no chain/hostname bypass and no full-L3 conformity claim. |
| Decision / implementation owner | Security owner; architecture |
| Original planned verification | Test expired/not-yet-valid, wrong-name, untrusted, malformed and valid chains under correct, grossly wrong and attacker-suggested candidate times. Verify no early trusted sample, host-clock action or secure-status transition from unauthenticated hints alone. |
| Open question | What bounded bootstrap time-validation policy is approved, how is it represented in assurance/status, and can strict mode remain usable without weakening certificate checks? |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.3, THR-2/3/26:** A supplied TLS date does not shift every Windows trust/revocation check, and the selected verifier has explicit soft-failure behavior. CHECK-8/9/26/27: test actual service-account roots, enterprise distrust, hostname/signature/algorithm validation, revoked/expired/not-yet-valid/unknown status, offline operation and gross past/future OS dates. Record leaf-versus-chain and soft/hard revocation policy, current-time CRL limitation and bootstrap refusal/exception. Do not treat unknown as confirmed non-revoked; legitimate required-policy validation succeeds.

### v5.0.0-12.3.3

| Field | Value |
| --- | --- |
| Source shortcode | V12.3.3 |
| Minimum assigned level | L2 |
| Chapter | Secure Communication |
| Section | General Service to Service Communication Security |
| Adapted requirement summary | Encrypt internal HTTP links without insecure fallback. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | HTTP/web-service-specific requirement |
| Applicability rationale | Local components use secured named pipes, not internal HTTP services; TLS and NTP wire assurances are assessed in their own rows. |
| Native control mapping | CTRL-SCOPE: no application HTTP endpoint; protocol framing and IPC remain covered separately. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect network-entry inventory and packaged dependencies; confirm absence of application HTTP routes; reassess if update/revocation HTTP integrations are introduced. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-12.3.4

| Field | Value |
| --- | --- |
| Source shortcode | V12.3.4 |
| Minimum assigned level | L2 |
| Chapter | Secure Communication |
| Section | General Service to Service Communication Security |
| Adapted requirement summary | Trust only intended certificates on internal TLS connections. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | No TLS connection is planned between local engine, broker, helper and UI; Windows named-pipe identity is a separate mechanism. Adding internal TLS reopens certificate trust requirements. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-12.3.5

| Field | Value |
| --- | --- |
| Source shortcode | V12.3.5 |
| Minimum assigned level | L3 |
| Chapter | Secure Communication |
| Section | General Service to Service Communication Security |
| Adapted requirement summary | Strongly authenticate both endpoints of internal service communication. |
| Proposed applicability | Native adaptation (proposed) |
| Literal standard scope | Strong internal-service endpoint authentication with explicit PKI wording; local Windows mechanism is a native adaptation |
| Applicability rationale | Native adaptation requires Windows service/peer identity, protected local pipe creation and reciprocal endpoint verification. It uses kernel-enforced local identity rather than the literal PKI-based mechanism; this is not full literal fulfillment. Local-only access is insufficient by itself. |
| Native control mapping | CTRL-IPC: local-only secured named pipes, verified service identity and bounded authenticated request lifecycle; explicit native adaptation, not a PKI-equivalence claim. |
| Decision / implementation owner | Windows integration; security owner |
| Original planned verification | Attempt pipe-name squatting, remote connection, wrong-service process, unexpected SID, hostile impersonation and stale/replayed requests; verify both endpoints reject unauthorized peers. |
| Open question | Confirm reciprocal peer-verification mechanisms and formally decide the local-kernel-identity adaptation versus the literal PKI wording before any conformance claim. |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |


## V13 - Configuration

### v5.0.0-13.1.1

| Field | Value |
| --- | --- |
| Source shortcode | V13.1.1 |
| Minimum assigned level | L2 |
| Chapter | Configuration |
| Section | Configuration Documentation |
| Adapted requirement summary | Document all communication needs and user-controlled destinations. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic architecture/documentation requirement |
| Applicability rationale | Inventory DNS, NTP, NTS-KE, configured/negotiated server redirects, local IPC and any certificate-status retrieval; identify which fields an administrator or remote peer can influence. |
| Native control mapping | CTRL-DOC: versioned architecture, trust-boundary and policy inventories. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Compare code/configuration/dependency network use and Windows endpoints to the documented communication matrix; reject unlisted communication paths. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.1, THR-21:** DNS and negotiated NTP destinations are outbound-resource choices even when received through authenticated TLS CHECK-6/8/9/21: denied negotiated/resolved loopback/internal/out-of-scope address or port, DNS rebinding and reconnect produce no unauthorized network operation; explicitly approved LAN and legitimate permitted negotiated destinations succeed; retain existing native-adaptation treatment where applicable

### v5.0.0-13.1.2

| Field | Value |
| --- | --- |
| Source shortcode | V13.1.2 |
| Minimum assigned level | L3 |
| Chapter | Configuration |
| Section | Configuration Documentation |
| Adapted requirement summary | Document connection caps and behavior at capacity for every service. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic architecture/documentation requirement |
| Applicability rationale | The product has security-sensitive inputs, effects and dependent services that require explicit policy. |
| Native control mapping | CTRL-DOC: versioned architecture, trust-boundary and policy inventories. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Review per-source, global TLS, UDP, DNS and IPC concurrency limits plus overflow/cancellation behavior; require a finite bound for every independently scheduled external operation. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-13.1.3

| Field | Value |
| --- | --- |
| Source shortcode | V13.1.3 |
| Minimum assigned level | L3 |
| Chapter | Configuration |
| Section | Configuration Documentation |
| Adapted requirement summary | Document resource release, deadlines and bounded retry strategies. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic architecture/documentation requirement |
| Applicability rationale | The product has security-sensitive inputs, effects and dependent services that require explicit policy. |
| Native control mapping | CTRL-DOC: versioned architecture, trust-boundary and policy inventories. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Review ownership and cleanup for sockets, pipes, handles, TLS tasks, timers and files; specify monotonic deadlines, bounded backoff and shutdown behavior for each failure path. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-13.1.4

| Field | Value |
| --- | --- |
| Source shortcode | V13.1.4 |
| Minimum assigned level | L3 |
| Chapter | Configuration |
| Section | Configuration Documentation |
| Adapted requirement summary | Document critical secrets and threat-based rotation schedules. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic key/secrets management requirement |
| Applicability rationale | NTS session keys/cookies, trust anchors and release-signing material need scoped ownership and lifetimes. |
| Native control mapping | CTRL-KEYS: key inventory, access separation, rekey/disposal and no plaintext secret outputs; any persisted association requires explicit protected-storage policy. |
| Decision / implementation owner | Crypto/TLS; release; security owner |
| Original planned verification | Inspect key lifecycle and actual holders; inject expiration/rekey/provider failure; use sentinel-secret disclosure tests and owned cleanup checks. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-13.2.1

| Field | Value |
| --- | --- |
| Source shortcode | V13.2.1 |
| Minimum assigned level | L2 |
| Chapter | Configuration |
| Section | Backend Communication Configuration |
| Adapted requirement summary | Authenticate backend peers with distinct nonstatic service identities. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic service configuration requirement |
| Applicability rationale | Windows service SIDs/identities and verified named-pipe peers replace shared API passwords. The engine, broker and privileged helper must not inherit a common unnecessary privileged identity. |
| Native control mapping | CTRL-CONFIG: protected versioned configuration; CTRL-PRIV: minimum service privileges; CTRL-EGRESS: approved destinations. |
| Decision / implementation owner | Architecture; Windows integration; installer |
| Original planned verification | Inspect installed accounts, service SIDs, restricted tokens and pipe ACLs; attempt unauthorized peer substitution and verify absence of embedded reusable shared secrets. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-13.2.2

| Field | Value |
| --- | --- |
| Source shortcode | V13.2.2 |
| Minimum assigned level | L2 |
| Chapter | Configuration |
| Section | Backend Communication Configuration |
| Adapted requirement summary | Use the least privilege needed for service and OS communication. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic service configuration requirement |
| Applicability rationale | The network engine must lose dangerous privileges; the clock broker alone retains narrowly controlled time-setting authority; elevation belongs to the installer/config helper and ordinary UI remains unelevated. |
| Native control mapping | CTRL-CONFIG: protected versioned configuration; CTRL-PRIV: minimum service privileges; CTRL-EGRESS: approved destinations. |
| Decision / implementation owner | Architecture; Windows integration; installer |
| Original planned verification | Inspect actual installed process tokens, enabled/removed privileges and OS access rights in Windows; prove denied access to clock/config effects from the engine and ordinary UI except through the approved broker path. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.1, THR-20:** MSI maintenance/rollback data crosses into a more privileged intermediary and can race privileged filesystem effects CHECK-13/15/23: hostile public properties, supported/unsupported transforms, rollback records and user-controlled paths through actual install/repair/remove entrypoints cannot trigger arbitrary service/file/process effects; forward and rollback use effective caller/operation authority and stable validated handles; normal complete lifecycle succeeds
- **Revision 1.3, THR-5/14/26:** Windows bindings and a service wrapper do not configure least privilege or contain unsafe native operations automatically. CHECK-1/2/16/26/27: inspect actual service tokens and dependency placement, deny engine clock/injection and unauthorized pipe effects, qualify narrow FFI ownership/mitigations and keep TLS/AEAD out of the broker. Positive permitted operations and normal provider loading must work. Preserve literal ASVS interpretation limits.

### v5.0.0-13.2.3

| Field | Value |
| --- | --- |
| Source shortcode | V13.2.3 |
| Minimum assigned level | L2 |
| Chapter | Configuration |
| Section | Backend Communication Configuration |
| Adapted requirement summary | Exclude default credentials from service authentication. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic service configuration requirement |
| Applicability rationale | No default shared password/API key is planned. Use explicitly scoped Windows service identities and authenticated local peer checks; packaging must not introduce sample credentials. |
| Native control mapping | CTRL-CONFIG: protected versioned configuration; CTRL-PRIV: minimum service privileges; CTRL-EGRESS: approved destinations. |
| Decision / implementation owner | Architecture; Windows integration; installer |
| Original planned verification | Inspect final installer/configuration and actual service setup for default/shared credentials; attempt known sample credentials only if an accidental credential path is discovered. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-13.2.4

| Field | Value |
| --- | --- |
| Source shortcode | V13.2.4 |
| Minimum assigned level | L2 |
| Chapter | Configuration |
| Section | Backend Communication Configuration |
| Adapted requirement summary | Allowlist permitted external resources and systems. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic service configuration requirement |
| Applicability rationale | Administratively approved sources and bounded NTS-negotiated destinations form an explicit egress policy. Validation must preserve legitimate approved internal NTP servers rather than blindly rejecting every private address. |
| Native control mapping | CTRL-CONFIG: protected versioned configuration; CTRL-PRIV: minimum service privileges; CTRL-EGRESS: approved destinations. |
| Decision / implementation owner | Architecture; Windows integration; installer |
| Original planned verification | Test approved and disallowed host/port/address transitions, DNS changes and negotiated endpoints against the final policy; verify the actual connected endpoint is authorized. |
| Open question | Define pinning versus re-resolution and allowed NTS endpoint negotiation, including approved private-network deployments. |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.1, THR-21:** DNS and negotiated NTP destinations are outbound-resource choices even when received through authenticated TLS CHECK-6/8/9/21: denied negotiated/resolved loopback/internal/out-of-scope address or port, DNS rebinding and reconnect produce no unauthorized network operation; explicitly approved LAN and legitimate permitted negotiated destinations succeed; retain existing native-adaptation treatment where applicable
- **Revision 1.2, THR-25:** Scalar/domain validity does not establish caller permission, trusted provenance, current ownership or an allowed resolved destination. CHECK-25 with CHECK-2/3/6/10: valid DTOs with serialized admin/validated claims, revoked or stale context, prohibited resolved/negotiated destinations and a changed resource before effect cannot gain authority. Check current state in the protected operation using the real enforcement path; a fresh authorized permitted operation succeeds. Pure validator tests do not qualify Windows authorization.
- **Revision 1.3, THR-21/26:** Certificate chain building can introduce AIA/root/revocation destinations and delegated Windows network activity. CHECK-6/8/9/27: observe the real process/service performing each retrieval, enforce the documented allowed resource/protocol policy, block denied destinations and show permitted acquisition. Do not claim engine firewall rules cover delegated services; inability to express required policy blocks verifier admission. Preserve existing native-adaptation classifications.

### v5.0.0-13.2.5

| Field | Value |
| --- | --- |
| Source shortcode | V13.2.5 |
| Minimum assigned level | L2 |
| Chapter | Configuration |
| Section | Backend Communication Configuration |
| Adapted requirement summary | Constrain application-server outbound resources with an allowlist. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic service configuration requirement |
| Applicability rationale | Treat the engine/service as the relevant application component even without HTTP. Bound network peers, trusted file roots and named-pipe targets according to the same approved resource policy. |
| Native control mapping | CTRL-CONFIG: protected versioned configuration; CTRL-PRIV: minimum service privileges; CTRL-EGRESS: approved destinations. |
| Decision / implementation owner | Architecture; Windows integration; installer |
| Original planned verification | Observe actual connections and file/IPC opens under controlled configuration and hostile redirects; verify no unintended privileged endpoint or path is reached. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.1, THR-21:** DNS and negotiated NTP destinations are outbound-resource choices even when received through authenticated TLS CHECK-6/8/9/21: denied negotiated/resolved loopback/internal/out-of-scope address or port, DNS rebinding and reconnect produce no unauthorized network operation; explicitly approved LAN and legitimate permitted negotiated destinations succeed; retain existing native-adaptation treatment where applicable
- **Revision 1.3, THR-21/26:** Certificate chain building can introduce AIA/root/revocation destinations and delegated Windows network activity. CHECK-6/8/9/27: observe the real process/service performing each retrieval, enforce the documented allowed resource/protocol policy, block denied destinations and show permitted acquisition. Do not claim engine firewall rules cover delegated services; inability to express required policy blocks verifier admission. Preserve existing native-adaptation classifications.

### v5.0.0-13.2.6

| Field | Value |
| --- | --- |
| Source shortcode | V13.2.6 |
| Minimum assigned level | L3 |
| Chapter | Configuration |
| Section | Backend Communication Configuration |
| Adapted requirement summary | Enforce documented connection, timeout and retry limits. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic availability and resource-control requirement |
| Applicability rationale | A network-facing engine and privileged local service can be exhausted by malformed input or excessive work. |
| Native control mapping | CTRL-RESOURCE: finite buffers, tasks, handles, connection counts, rates and retries. |
| Decision / implementation owner | Engine; broker; QA |
| Original planned verification | Measure bounded local workloads and injected failure with legitimate controls; verify limits, recovery and no persistent resource growth. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.3, THR-8/26:** A current-thread runtime can submit blocking native work whose queue and lifetime outlive caller cancellation. CHECK-27 with CHECK-8/9/21: saturate TLS/DNS/native job, byte, connection and retry budgets; keep permits with jobs until actual completion, reject/coalesce before submission and avoid replacement-pool growth. Show fair scheduling when capacity exists, independent lifecycle/broker responsiveness, diagnosed saturation and a valid request after bounded recovery.

### v5.0.0-13.3.1

| Field | Value |
| --- | --- |
| Source shortcode | V13.3.1 |
| Minimum assigned level | L2 |
| Chapter | Configuration |
| Section | Secret Management |
| Adapted requirement summary | Use managed secrets with hardware backing at L3 and no embedded secrets. |
| Proposed applicability | Needs decision |
| Literal standard scope | Literal requirement or applicability needs an explicit security decision |
| Applicability rationale | Protected NTS key ownership and OS storage protection are useful, but do not establish a hardware-backed secrets manager. Saved associations, if enabled, need an explicit protected persistence lifecycle. The exact Windows/provider mechanism and any hardware constraint remain unresolved. |
| Native control mapping | CTRL-DECISION-VAULT: secrets lifecycle and hardware-provider feasibility; DPAPI or process memory alone is not an HSM guarantee. |
| Decision / implementation owner | Security owner; architecture |
| Original planned verification | Inventory each backend secret and actual creation/storage/access/destruction provider; verify absence from source/build outputs and test any proposed hardware-backed path on supported Windows hardware. |
| Open question | Can the required NTS/TLS secrets use a supported hardware-backed lifecycle, or is this an explicit L3 gap with minimized persistence and access-isolation compensations? |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-13.3.2

| Field | Value |
| --- | --- |
| Source shortcode | V13.3.2 |
| Minimum assigned level | L2 |
| Chapter | Configuration |
| Section | Secret Management |
| Adapted requirement summary | Restrict every secret asset to the minimum authorized holders. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic key/secrets management requirement |
| Applicability rationale | NTS session keys/cookies, trust anchors and release-signing material need scoped ownership and lifetimes. |
| Native control mapping | CTRL-KEYS: key inventory, access separation, rekey/disposal and no plaintext secret outputs; any persisted association requires explicit protected-storage policy. |
| Decision / implementation owner | Crypto/TLS; release; security owner |
| Original planned verification | Inspect key lifecycle and actual holders; inject expiration/rekey/provider failure; use sentinel-secret disclosure tests and owned cleanup checks. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-13.3.3

| Field | Value |
| --- | --- |
| Source shortcode | V13.3.3 |
| Minimum assigned level | L3 |
| Chapter | Configuration |
| Section | Secret Management |
| Adapted requirement summary | Perform all cryptographic operations inside an isolated security module. |
| Proposed applicability | Needs decision |
| Literal standard scope | Literal requirement or applicability needs an explicit security decision |
| Applicability rationale | A software TLS/NTS library in the engine does not by itself satisfy isolation of every cryptographic operation/key inside a vault or HSM. Algorithm/provider support and performance must be established explicitly. |
| Native control mapping | CTRL-DECISION-MODULE: per-operation provider boundary; no equivalence claim from Rust, process separation or a TPM used only for a subset of keys. |
| Decision / implementation owner | Security owner; architecture |
| Original planned verification | Map every crypto operation to its actual provider and key boundary; demonstrate module isolation where claimed and record operations that remain in ordinary process memory. |
| Open question | Is an all-operations isolated module practical for TLS/NTS on the target Windows baseline, or must the literal requirement remain unmet? |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-13.3.4

| Field | Value |
| --- | --- |
| Source shortcode | V13.3.4 |
| Minimum assigned level | L3 |
| Chapter | Configuration |
| Section | Secret Management |
| Adapted requirement summary | Expire and rotate secrets according to the documented schedule. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic key/secrets management requirement |
| Applicability rationale | NTS session keys/cookies, trust anchors and release-signing material need scoped ownership and lifetimes. |
| Native control mapping | CTRL-KEYS: key inventory, access separation, rekey/disposal and no plaintext secret outputs; any persisted association requires explicit protected-storage policy. |
| Decision / implementation owner | Crypto/TLS; release; security owner |
| Original planned verification | Exercise NTS key/cookie exhaustion, rekey, provider errors, restart and trust/signing-key transition; verify old state cannot continue past the approved policy or leak into diagnostics. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-13.4.1

| Field | Value |
| --- | --- |
| Source shortcode | V13.4.1 |
| Minimum assigned level | L1 |
| Chapter | Configuration |
| Section | Unintended Information Leakage |
| Adapted requirement summary | Exclude source-control metadata from the deployed application. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic dependency/build/release requirement |
| Applicability rationale | Rust dependencies, generated code, native bindings and installer artifacts affect the trusted computing base. |
| Native control mapping | CTRL-SUPPLY: locked approved sources, inventory/SBOM and vulnerability policy; CTRL-RELEASE: immutable signed artifacts. |
| Decision / implementation owner | Release; QA/CI; security owner |
| Original planned verification | Verify actual resolved transitive graph and final installer contents/provenance; use invalid-source/artifact controls and confirm gate failure propagation. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-13.4.2

| Field | Value |
| --- | --- |
| Source shortcode | V13.4.2 |
| Minimum assigned level | L2 |
| Chapter | Configuration |
| Section | Unintended Information Leakage |
| Adapted requirement summary | Disable production debug modes and exposed debugging functions. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic dependency/build/release requirement |
| Applicability rationale | Release artifacts must not enable test clocks, fault injection, unauthenticated debug endpoints, developer consoles or secret-bearing dumps. Sanitized production diagnostics are an explicit supported feature, not an accidental debug mode. |
| Native control mapping | CTRL-SUPPLY: locked approved sources, inventory/SBOM and vulnerability policy; CTRL-RELEASE: immutable signed artifacts. |
| Decision / implementation owner | Release; QA/CI; security owner |
| Original planned verification | Inspect final compiled features, installer contents and running service configuration; attempt known test/debug entry points and inspect crash/reporting policy. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-13.4.3

| Field | Value |
| --- | --- |
| Source shortcode | V13.4.3 |
| Minimum assigned level | L2 |
| Chapter | Configuration |
| Section | Unintended Information Leakage |
| Adapted requirement summary | Disable unintended web-directory listings. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no WebView, browser frontend, DOM, browser-origin session or application HTTP server. NTS cookies are not browser cookies. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-13.4.4

| Field | Value |
| --- | --- |
| Source shortcode | V13.4.4 |
| Minimum assigned level | L2 |
| Chapter | Configuration |
| Section | Unintended Information Leakage |
| Adapted requirement summary | Disable HTTP TRACE in production. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | HTTP/web-service-specific requirement |
| Applicability rationale | No application HTTP server or production TRACE handler exists. Outbound native certificate/status retrieval does not introduce a TRACE endpoint. Verify the selected retrieval route and reassess if an HTTP server is introduced. |
| Native control mapping | CTRL-SCOPE: no application HTTP endpoint; protocol framing and IPC remain covered separately. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect network-entry inventory and packaged dependencies; confirm absence of application HTTP routes; reassess if update/revocation HTTP integrations are introduced. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision 1.4 review correction:** Narrow the exclusion to the actual server/sensitive-action surface; avoid a blanket assertion that every HTTP workflow is absent.

**Previous revision 1.3 values retained for traceability:**

- **Previous Applicability rationale:** The baseline exposes no HTTP API or web service and has no application HTTP workflow; NTP and NTS-KE are different protocols. Revocation-fetch design may require reassessment.

**Current planned verification — not executed:** CHECK-4 and OPEN-8: confirm the role-specific absence stated in this rationale against the admitted component and retrieval inventory. This proposed N/A does not exempt any separate client HTTP, certificate, destination or confidentiality obligation.

### v5.0.0-13.4.5

| Field | Value |
| --- | --- |
| Source shortcode | V13.4.5 |
| Minimum assigned level | L2 |
| Chapter | Configuration |
| Section | Unintended Information Leakage |
| Adapted requirement summary | Expose documentation or monitoring interfaces only by deliberate policy. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic service configuration requirement |
| Applicability rationale | Local status/diagnostic IPC remains an interface even without a web server. Document intended consumers and keep internal control surfaces, secrets and privileged monitoring details out of ordinary-user projections. |
| Native control mapping | CTRL-CONFIG: protected versioned configuration; CTRL-PRIV: minimum service privileges; CTRL-EGRESS: approved destinations. |
| Decision / implementation owner | Architecture; Windows integration; installer |
| Original planned verification | Enumerate packaged listeners/IPC operations and UI projections; test permitted and denied readers and verify internal/debug documentation is not exposed as an unintended service. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-13.4.6

| Field | Value |
| --- | --- |
| Source shortcode | V13.4.6 |
| Minimum assigned level | L3 |
| Chapter | Configuration |
| Section | Unintended Information Leakage |
| Adapted requirement summary | Avoid exposing detailed backend component versions. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic data protection/minimization requirement |
| Applicability rationale | Ordinary status should not reveal dependency/provider internals. A user-visible product version and explicitly authorized support bundle may be justified separately; the literal blanket scope needs a documented interpretation. |
| Native control mapping | CTRL-DATA: classification and minimum projections; CTRL-KEYS; CTRL-LOG. |
| Decision / implementation owner | Security owner; storage; observability |
| Original planned verification | Inspect normal status, errors and unauthenticated denials for dependency versions; review restricted diagnostic fields and explain any intentional support disclosure. |
| Open question | Define permitted product/build identification and restricted dependency/SBOM disclosure without claiming no version information exists. |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-13.4.7

| Field | Value |
| --- | --- |
| Source shortcode | V13.4.7 |
| Minimum assigned level | L3 |
| Chapter | Configuration |
| Section | Unintended Information Leakage |
| Adapted requirement summary | Restrict served file extensions at the web tier. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no WebView, browser frontend, DOM, browser-origin session or application HTTP server. NTS cookies are not browser cookies. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |


## V14 - Data Protection

### v5.0.0-14.1.1

| Field | Value |
| --- | --- |
| Source shortcode | V14.1.1 |
| Minimum assigned level | L2 |
| Chapter | Data Protection |
| Section | Data Protection Documentation |
| Adapted requirement summary | Identify sensitive data and assign protection levels. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic data protection/minimization requirement |
| Applicability rationale | Classify NTS keys/cookies, trust configuration, source addresses, host/user identifiers, logs and support exports. Encoding does not remove sensitivity; applicability of legal/organizational rules needs the actual deployment context. |
| Native control mapping | CTRL-DATA: classification and minimum projections; CTRL-KEYS; CTRL-LOG. |
| Decision / implementation owner | Security owner; storage; observability |
| Original planned verification | Review data-flow inventory from collection through deletion and every output projection; identify unknown protection obligations without inventing a legal compliance claim. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-14.1.2

| Field | Value |
| --- | --- |
| Source shortcode | V14.1.2 |
| Minimum assigned level | L2 |
| Chapter | Data Protection |
| Section | Data Protection Documentation |
| Adapted requirement summary | Document protection requirements for each sensitive-data class. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic architecture/documentation requirement |
| Applicability rationale | Define permitted holders, confidentiality/integrity, retention, diagnostics and access rules for every data class, including no-persistence choices and crash-dump exposure. |
| Native control mapping | CTRL-DOC: versioned architecture, trust-boundary and policy inventories. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Check policy completeness and contradictions against storage, UI, IPC, crypto and logging designs; explicitly record unsupported guarantees. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-14.2.1

| Field | Value |
| --- | --- |
| Source shortcode | V14.2.1 |
| Minimum assigned level | L1 |
| Chapter | Data Protection |
| Section | General Data Protection |
| Adapted requirement summary | Keep sensitive HTTP data out of URLs and query strings. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | HTTP/web-service-specific requirement |
| Applicability rationale | Application keys, NTS cookies and authentication tokens are not planned as HTTP URL/query data. A public certificate/status identifier is not automatically confidential. OPEN-8 must classify actual URL contents, including private-deployment identifiers, and verify absence of sensitive URL data before admission; any such data reopens this proposed N/A. Native endpoint names, paths and diagnostics retain separate secret-handling controls. |
| Native control mapping | CTRL-SCOPE: no application HTTP endpoint; protocol framing and IPC remain covered separately. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect network-entry inventory and packaged dependencies; confirm absence of application HTTP routes; reassess if update/revocation HTTP integrations are introduced. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision 1.4 review correction:** Keep the data-specific exclusion while explicitly covering the current conditional retrieval route.

**Previous revision 1.3 values retained for traceability:**

- **Previous Applicability rationale:** No application HTTP API/query workflow exists. Any certificate-status retrieval or later HTTP feature must be assessed; avoiding secrets in native endpoint names, paths and diagnostics is required separately.

**Current planned verification — not executed:** OPEN-8/CHECK-4/17: inspect the admitted direct/delegated retrieval URL and data inventory. Confirm that application secrets or sensitive identifiers do not enter URLs/query strings; reclassify if data classification or a later HTTP workflow changes this premise.

### v5.0.0-14.2.2

| Field | Value |
| --- | --- |
| Source shortcode | V14.2.2 |
| Minimum assigned level | L2 |
| Chapter | Data Protection |
| Section | General Data Protection |
| Adapted requirement summary | Avoid sensitive caching or securely purge caches after use. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic data protection/minimization requirement |
| Applicability rationale | The engine and TLS library may cache session material even without load balancers. Scope and bound actual caches; dispose of stale association data and never project secrets to the UI. |
| Native control mapping | CTRL-DATA: classification and minimum projections; CTRL-KEYS; CTRL-LOG. |
| Decision / implementation owner | Security owner; storage; observability |
| Original planned verification | Inspect provider/session caches, key/cookie queues, status snapshots and restart behavior; use sentinel secrets and forced rekey/expiry/disconnect to test exposure and disposal limits. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-14.2.3

| Field | Value |
| --- | --- |
| Source shortcode | V14.2.3 |
| Minimum assigned level | L2 |
| Chapter | Data Protection |
| Section | General Data Protection |
| Adapted requirement summary | Do not disclose protected data to untrusted recipients. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic data protection/minimization requirement |
| Applicability rationale | No telemetry or tracker is part of the baseline. Network peers receive only protocol-required data; user-triggered support export must have an explicit minimal projection. |
| Native control mapping | CTRL-DATA: classification and minimum projections; CTRL-KEYS; CTRL-LOG. |
| Decision / implementation owner | Security owner; storage; observability |
| Original planned verification | Inspect actual outbound destinations and payload schemas; test sentinel-sensitive fields against every log/UI/export/IPC projection and verify no automatic external diagnostic upload. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-14.2.4

| Field | Value |
| --- | --- |
| Source shortcode | V14.2.4 |
| Minimum assigned level | L2 |
| Chapter | Data Protection |
| Section | General Data Protection |
| Adapted requirement summary | Implement the documented data-class protection rules. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic data protection/minimization requirement |
| Applicability rationale | Host configuration, addresses, user identifiers and NTS secrets have different disclosure and retention needs. |
| Native control mapping | CTRL-DATA: classification and minimum projections; CTRL-KEYS; CTRL-LOG. |
| Decision / implementation owner | Security owner; storage; observability |
| Original planned verification | Trace classified fields into disk, IPC, UI and diagnostic outputs; exercise access, retention and sentinel-secret controls. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-14.2.5

| Field | Value |
| --- | --- |
| Source shortcode | V14.2.5 |
| Minimum assigned level | L3 |
| Chapter | Data Protection |
| Section | General Data Protection |
| Adapted requirement summary | Prevent sensitive caching and web-cache deception. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no WebView, browser frontend, DOM, browser-origin session or application HTTP server. NTS cookies are not browser cookies. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-14.2.6

| Field | Value |
| --- | --- |
| Source shortcode | V14.2.6 |
| Minimum assigned level | L3 |
| Chapter | Data Protection |
| Section | General Data Protection |
| Adapted requirement summary | Return only needed sensitive fields and mask full values by default. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic data protection/minimization requirement |
| Applicability rationale | UI/status/diagnostic DTOs are explicit minimal views, not serialized internal state. NTS secrets must never be revealable UI fields; only legitimate support identifiers may have an intentional disclosure path. |
| Native control mapping | CTRL-DATA: classification and minimum projections; CTRL-KEYS; CTRL-LOG. |
| Decision / implementation owner | Security owner; storage; observability |
| Original planned verification | Inspect all returned fields against their consumer's actual need; prove excluded internal fields and sentinels cannot appear through errors, serialization or alternate status views. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-14.2.7

| Field | Value |
| --- | --- |
| Source shortcode | V14.2.7 |
| Minimum assigned level | L3 |
| Chapter | Data Protection |
| Section | General Data Protection |
| Adapted requirement summary | Delete sensitive data according to a defined retention policy. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic data protection/minimization requirement |
| Applicability rationale | Host configuration, addresses, user identifiers and NTS secrets have different disclosure and retention needs. |
| Native control mapping | CTRL-DATA: classification and minimum projections; CTRL-KEYS; CTRL-LOG. |
| Decision / implementation owner | Security owner; storage; observability |
| Original planned verification | Exercise log rotation, association disposal, superseded configuration, support-export staging and uninstall policy; document the boundary between app-managed deletion and storage-media forensic erasure. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-14.2.8

| Field | Value |
| --- | --- |
| Source shortcode | V14.2.8 |
| Minimum assigned level | L3 |
| Chapter | Data Protection |
| Section | General Data Protection |
| Adapted requirement summary | Remove or obtain consent for sensitive metadata in submitted files. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | The baseline has no user-file upload or image/document processing feature. Configuration values are validated data, not forwarded file metadata; a future file-ingestion/export feature must reopen this scope. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-14.3.1

| Field | Value |
| --- | --- |
| Source shortcode | V14.3.1 |
| Minimum assigned level | L1 |
| Chapter | Data Protection |
| Section | Client-side Data Protection |
| Adapted requirement summary | Clear authenticated client data when the client or session ends. |
| Proposed applicability | Native adaptation (proposed) |
| Literal standard scope | User-session requirement adapted to privileged IPC lifecycle |
| Applicability rationale | Native adaptation clears privileged snapshots and command-bound data when authority expires or the connection ends, including offline cleanup. There is no browser storage, and the UI must never hold NTS keys/cookies. |
| Native control mapping | CTRL-UI-DATA: minimal native client state and cleanup; no DOM or Clear-Site-Data equivalence claim. |
| Decision / implementation owner | IPC; helper; security owner |
| Original planned verification | Disconnect or terminate the service during a privileged view; verify stale sensitive data is cleared, stale controls cannot execute, and only explicitly permitted non-sensitive UI preferences persist. |
| Open question | Define which status fields remain visible after disconnection and how stale/untrusted state is clearly marked. |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-14.3.2

| Field | Value |
| --- | --- |
| Source shortcode | V14.3.2 |
| Minimum assigned level | L2 |
| Chapter | Data Protection |
| Section | Client-side Data Protection |
| Adapted requirement summary | Use HTTP anti-caching headers for sensitive browser responses. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no WebView, browser frontend, DOM, browser-origin session or application HTTP server. NTS cookies are not browser cookies. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-14.3.3

| Field | Value |
| --- | --- |
| Source shortcode | V14.3.3 |
| Minimum assigned level | L2 |
| Chapter | Data Protection |
| Section | Client-side Data Protection |
| Adapted requirement summary | Exclude sensitive data from ordinary browser storage. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The baseline has no WebView, browser frontend, DOM, browser-origin session or application HTTP server. NTS cookies are not browser cookies. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |


## V15 - Secure Coding and Architecture

### v5.0.0-15.1.1

| Field | Value |
| --- | --- |
| Source shortcode | V15.1.1 |
| Minimum assigned level | L1 |
| Chapter | Secure Coding and Architecture |
| Section | Secure Coding and Architecture Documentation |
| Adapted requirement summary | Document risk-based dependency update and vulnerability deadlines. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic architecture/documentation requirement |
| Applicability rationale | Set an owned remediation policy for Rust crates, native libraries/providers, build tools and installer dependencies; source availability alone is not timely maintenance. |
| Native control mapping | CTRL-DOC: versioned architecture, trust-boundary and policy inventories. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Review severity/exploitability and exposure decision rules, deadlines, exceptions and ownership; verify the process covers transitive and platform-dependent components. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-15.1.2

| Field | Value |
| --- | --- |
| Source shortcode | V15.1.2 |
| Minimum assigned level | L2 |
| Chapter | Secure Coding and Architecture |
| Section | Secure Coding and Architecture Documentation |
| Adapted requirement summary | Maintain a trusted-source inventory of all third-party components. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic dependency/build/release requirement |
| Applicability rationale | Rust dependencies, generated code, native bindings and installer artifacts affect the trusted computing base. |
| Native control mapping | CTRL-SUPPLY: locked approved sources, inventory/SBOM and vulnerability policy; CTRL-RELEASE: immutable signed artifacts. |
| Decision / implementation owner | Release; QA/CI; security owner |
| Original planned verification | Verify actual resolved transitive graph and final installer contents/provenance; use invalid-source/artifact controls and confirm gate failure propagation. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-15.1.3

| Field | Value |
| --- | --- |
| Source shortcode | V15.1.3 |
| Minimum assigned level | L2 |
| Chapter | Secure Coding and Architecture |
| Section | Secure Coding and Architecture Documentation |
| Adapted requirement summary | Document expensive operations and availability protections. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic architecture/documentation requirement |
| Applicability rationale | Include DNS, TLS/NTS-KE, validation, selection/controller work, support exports, UI refresh, IPC handling and install/config operations. Bound work and consumer deadlines using monotonic time. |
| Native control mapping | CTRL-DOC: versioned architecture, trust-boundary and policy inventories. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Compare per-operation cost/concurrency policy with actual call graph and scheduled tasks; require queue, cancellation and overload behavior for every expensive path. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-15.1.4

| Field | Value |
| --- | --- |
| Source shortcode | V15.1.4 |
| Minimum assigned level | L3 |
| Chapter | Secure Coding and Architecture |
| Section | Secure Coding and Architecture Documentation |
| Adapted requirement summary | Identify particularly risky third-party components. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic architecture/documentation requirement |
| Applicability rationale | Flag network parsers, TLS/AEAD providers, unsafe/native dependencies, GUI/accessibility integration and installer/signing tools with reasons and owners. |
| Native control mapping | CTRL-DOC: versioned architecture, trust-boundary and policy inventories. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Review each dependency's exposure, unsafe/native boundary and update evidence; avoid using a language label as a substitute for component risk analysis. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-15.1.5

| Field | Value |
| --- | --- |
| Source shortcode | V15.1.5 |
| Minimum assigned level | L3 |
| Chapter | Secure Coding and Architecture |
| Section | Secure Coding and Architecture Documentation |
| Adapted requirement summary | Identify dangerous application functionality. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic architecture/documentation requirement |
| Applicability rationale | Document clock adjustment, service installation/configuration, privilege changes, file/registry writes, process launch if any, and unsafe FFI as dangerous capabilities. |
| Native control mapping | CTRL-DOC: versioned architecture, trust-boundary and policy inventories. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Trace all effectful capabilities and their trusted entry points; require a narrow owner, permitted caller and invalid-input/no-effect contract for each. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-15.2.1

| Field | Value |
| --- | --- |
| Source shortcode | V15.2.1 |
| Minimum assigned level | L1 |
| Chapter | Secure Coding and Architecture |
| Section | Security Architecture and Dependencies |
| Adapted requirement summary | Keep deployed components within the approved remediation deadlines. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic dependency/build/release requirement |
| Applicability rationale | Rust dependencies, generated code, native bindings and installer artifacts affect the trusted computing base. |
| Native control mapping | CTRL-SUPPLY: locked approved sources, inventory/SBOM and vulnerability policy; CTRL-RELEASE: immutable signed artifacts. |
| Decision / implementation owner | Release; QA/CI; security owner |
| Original planned verification | Check the exact final dependency/build inventory against current advisories and policy dates; a stale or unavailable advisory feed cannot be recorded as a clean result. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.3, THR-15:** The observed advisory snapshot is not a product graph assessment, and package updates need an owned policy. CHECK-26: establish documented update/remediation deadlines; test a known overdue or affected component and an approved patched neighboring graph through the actual release gate. Record database freshness, exact artifact and justified exceptions; no absence-of-advisory clearance.

### v5.0.0-15.2.2

| Field | Value |
| --- | --- |
| Source shortcode | V15.2.2 |
| Minimum assigned level | L2 |
| Chapter | Secure Coding and Architecture |
| Section | Security Architecture and Dependencies |
| Adapted requirement summary | Enforce documented bounds for expensive operations. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic availability and resource-control requirement |
| Applicability rationale | A network-facing engine and privileged local service can be exhausted by malformed input or excessive work. |
| Native control mapping | CTRL-RESOURCE: finite buffers, tasks, handles, connection counts, rates and retries. |
| Decision / implementation owner | Engine; broker; QA |
| Original planned verification | Measure bounded local workloads and injected failure with legitimate controls; verify limits, recovery and no persistent resource growth. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.3, THR-8/26:** A current-thread runtime can submit blocking native work whose queue and lifetime outlive caller cancellation. CHECK-27 with CHECK-8/9/21: saturate TLS/DNS/native job, byte, connection and retry budgets; keep permits with jobs until actual completion, reject/coalesce before submission and avoid replacement-pool growth. Show fair scheduling when capacity exists, independent lifecycle/broker responsiveness, diagnosed saturation and a valid request after bounded recovery.

### v5.0.0-15.2.3

| Field | Value |
| --- | --- |
| Source shortcode | V15.2.3 |
| Minimum assigned level | L2 |
| Chapter | Secure Coding and Architecture |
| Section | Security Architecture and Dependencies |
| Adapted requirement summary | Ship only required production functionality. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic dependency/build/release requirement |
| Applicability rationale | Exclude test-time backdoors, simulated clocks, fault injection, sample services, developer consoles, arbitrary scripts/plugins and unnecessary network listeners from release artifacts. |
| Native control mapping | CTRL-SUPPLY: locked approved sources, inventory/SBOM and vulnerability policy; CTRL-RELEASE: immutable signed artifacts. |
| Decision / implementation owner | Release; QA/CI; security owner |
| Original planned verification | Inspect resolved feature flags, packaged files and actual installed entry points; try known test-only controls against the release package and require their absence. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.3, THR-15:** Provider defaults and additive features can reintroduce unnecessary production capabilities. CHECK-26: inspect production feature closure and binary/native inventory for unwanted TLS providers, TLS 1.2, certificate debug logs, examples and dev-only code; a deliberately expanded feature fixture must be detected while the approved minimal graph succeeds.

### v5.0.0-15.2.4

| Field | Value |
| --- | --- |
| Source shortcode | V15.2.4 |
| Minimum assigned level | L3 |
| Chapter | Secure Coding and Architecture |
| Section | Security Architecture and Dependencies |
| Adapted requirement summary | Resolve all dependencies from intended trusted sources. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic dependency/build/release requirement |
| Applicability rationale | Pin and review direct/transitive source origins, checksums and feature changes; protect internal package names and registry configuration against source substitution. Absolute absence of dependency-confusion risk is not inferred from a lockfile alone. |
| Native control mapping | CTRL-SUPPLY: locked approved sources, inventory/SBOM and vulnerability policy; CTRL-RELEASE: immutable signed artifacts. |
| Decision / implementation owner | Release; QA/CI; security owner |
| Original planned verification | Verify actual fetched graph and registry/source overrides in a clean build; use controlled mismatched-source/checksum cases and confirm a hard gate failure. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.3, THR-15:** Published packages, source commits, native archives and generated code require an exact provenance chain. CHECK-26: compare package checksums and associated source, inspect build/proc-macro/native components and approved origins, and reject a substituted/unexpected source. Reconcile lockfile, embedded dependency metadata, complete SBOM and final artifact inventory; an approved origin succeeds.

### v5.0.0-15.2.5

| Field | Value |
| --- | --- |
| Source shortcode | V15.2.5 |
| Minimum assigned level | L3 |
| Chapter | Secure Coding and Architecture |
| Section | Security Architecture and Dependencies |
| Adapted requirement summary | Isolate dangerous capabilities and risky components. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic service configuration requirement |
| Applicability rationale | Separate network-facing parsing/TLS work from the narrowly privileged clock broker; restrict service tokens, IPC authority, file ACLs and process launch. Isolation must be demonstrated on the installed Windows system. |
| Native control mapping | CTRL-CONFIG: protected versioned configuration; CTRL-PRIV: minimum service privileges; CTRL-EGRESS: approved destinations. |
| Decision / implementation owner | Architecture; Windows integration; installer |
| Original planned verification | Attempt privileged clock/config effects from a compromised-engine-equivalent test principal and hostile local clients; inspect actual token and ACL boundaries and document allowed broker capability residual risk. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.3, THR-5/14/26:** Windows bindings and a service wrapper do not configure least privilege or contain unsafe native operations automatically. CHECK-1/2/16/26/27: inspect actual service tokens and dependency placement, deny engine clock/injection and unauthorized pipe effects, qualify narrow FFI ownership/mitigations and keep TLS/AEAD out of the broker. Positive permitted operations and normal provider loading must work. Preserve literal ASVS interpretation limits.

### v5.0.0-15.3.1

| Field | Value |
| --- | --- |
| Source shortcode | V15.3.1 |
| Minimum assigned level | L1 |
| Chapter | Secure Coding and Architecture |
| Section | Defensive Coding |
| Adapted requirement summary | Return only explicitly permitted object fields. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic data protection/minimization requirement |
| Applicability rationale | Host configuration, addresses, user identifiers and NTS secrets have different disclosure and retention needs. |
| Native control mapping | CTRL-DATA: classification and minimum projections; CTRL-KEYS; CTRL-LOG. |
| Decision / implementation owner | Security owner; storage; observability |
| Original planned verification | Trace classified fields into disk, IPC, UI and diagnostic outputs; exercise access, retention and sentinel-secret controls. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-15.3.2

| Field | Value |
| --- | --- |
| Source shortcode | V15.3.2 |
| Minimum assigned level | L2 |
| Chapter | Secure Coding and Architecture |
| Section | Defensive Coding |
| Adapted requirement summary | Follow external URL redirects only when explicitly intended. |
| Proposed applicability | Needs decision |
| Literal standard scope | HTTP/web-service-specific requirement |
| Applicability rationale | The held Windows certificate-verifier route may retrieve AIA/root/CRL/OCSP URLs, directly or through OS services. Redirect behavior and the enforcement owner are not qualified. NTS-KE endpoint negotiation remains a different protocol feature and does not itself establish HTTP redirect applicability. |
| Native control mapping | CTRL-DECISION, CTRL-CERT, CTRL-EGRESS: explicit native/delegated retrieval scope and policy; CTRL-INPUT/CTRL-RESOURCE where this clause requires framing or length enforcement. No implemented control or platform guarantee is inferred. |
| Decision / implementation owner | Security; Windows platform; protocol; QA |
| Original planned verification | Inspect network-entry inventory and packaged dependencies; confirm absence of application HTTP routes; reassess if update/revocation HTTP integrations are introduced. |
| Open question | Which retrieval route is admitted, are redirects required and intended, and how are redirect hops and final destinations constrained? Resolve this under OPEN-8 before admitting the route. |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision 1.4 review correction:** Reconcile a current held direct/delegated certificate-retrieval candidate with the exact clause; no runtime failure is asserted.

**Previous revision 1.3 values retained for traceability:**

- **Previous Proposed applicability:** Proposed N/A
- **Previous Applicability rationale:** No application external-URL fetch/HTTP redirect workflow exists. NTS-KE negotiated endpoints are a different protocol feature and require their own explicit destination policy; revocation/update HTTP fetch would reopen this row.
- **Previous Native control mapping:** CTRL-SCOPE: no application HTTP endpoint; protocol framing and IPC remain covered separately.
- **Previous Decision / implementation owner:** Architecture; QA
- **Previous Open question:** None recorded

**Current planned verification — not executed:** OPEN-8/S0.7, CHECK-4/9/27: Qualify an allowed direct retrieval and refused redirects unless explicitly intended; every permitted hop and final resource must satisfy the selected policy. Verify bounded failures and no resulting clock proposal on denied retrieval. A generic destination allowlist alone does not decide redirect intent.

### v5.0.0-15.3.3

| Field | Value |
| --- | --- |
| Source shortcode | V15.3.3 |
| Minimum assigned level | L2 |
| Chapter | Secure Coding and Architecture |
| Section | Defensive Coding |
| Adapted requirement summary | Limit each operation to its intended mutable fields. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic authorization requirement |
| Applicability rationale | Use separate typed IPC/config commands with explicit allowed fields and reject unknown or forbidden fields. Do not deserialize a status or full internal-state object back into privileged mutable state. |
| Native control mapping | CTRL-AUTHZ: trusted service checks caller token, action and fields; no authority from UI claims. |
| Decision / implementation owner | Broker; privileged helper; IPC |
| Original planned verification | Attempt extra, duplicate, unknown, privileged and state-inconsistent fields for every action; verify denial or documented rejection without partial mutation. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.2, THR-25:** Only the decoded and fallibly constructed value may reach a trusted handler; UI validation and generic map normalization are not that enforcement. CHECK-25 with CHECK-3/15: exercise the actual receiver with duplicate/escaped/unknown keys, nested array-for-object input, absent/null/default ambiguity, trailing documents, generic protected-field patches, integer boundaries and invalid combinations. Inspect all constructor/Deserialize/From/Default/mutation paths. Invalid input has no effect; independently valid neighboring inputs, supported migrations and exact integer round trips succeed.

### v5.0.0-15.3.4

| Field | Value |
| --- | --- |
| Source shortcode | V15.3.4 |
| Minimum assigned level | L2 |
| Chapter | Secure Coding and Architecture |
| Section | Defensive Coding |
| Adapted requirement summary | Use trusted original-client addressing across proxies. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Feature-specific requirement outside the declared baseline |
| Applicability rationale | No HTTP proxy/middleware user-address chain exists. Named-pipe identity uses actual Windows peers, not user-supplied identity/IP fields; network peer/source binding is covered by protocol controls. |
| Native control mapping | CTRL-SCOPE: exclude named feature and reassess before introduction. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect feature/dependency/data-flow inventory and final artifact for the named excluded feature; absence is proposed, not verified. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-15.3.5

| Field | Value |
| --- | --- |
| Source shortcode | V15.3.5 |
| Minimum assigned level | L2 |
| Chapter | Secure Coding and Architecture |
| Section | Defensive Coding |
| Adapted requirement summary | Enforce intended data types and strict comparisons. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic input, parsing or representation control |
| Applicability rationale | Rust types help, but wire/config decoding, integer conversions, time units, signedness, floating-point exceptional values and FFI remain explicit validation boundaries. |
| Native control mapping | CTRL-INPUT: bounded canonical decoding and validated domain types; CTRL-SERIAL: typed serializers. |
| Decision / implementation owner | Protocol/core; configuration; IPC |
| Original planned verification | Use independent boundary fixtures for overflow, NaN/infinity where relevant, era/unit conversions, invalid enums and duplicate fields; verify no implicit coercion changes admitted meaning. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.1, THR-24:** Correct types and related numeric bounds must survive wire/core/IPC/backend conversions CHECK-6/7/9/10: independent era/overflow/NaN/unit/sign vectors and actual positive/negative Windows effects; validate post-conversion representability and actual acknowledgement; valid near-boundary values still work
- **Revision 1.2, THR-25:** Only the decoded and fallibly constructed value may reach a trusted handler; UI validation and generic map normalization are not that enforcement. CHECK-25 with CHECK-3/15: exercise the actual receiver with duplicate/escaped/unknown keys, nested array-for-object input, absent/null/default ambiguity, trailing documents, generic protected-field patches, integer boundaries and invalid combinations. Inspect all constructor/Deserialize/From/Default/mutation paths. Invalid input has no effect; independently valid neighboring inputs, supported migrations and exact integer round trips succeed.

### v5.0.0-15.3.6

| Field | Value |
| --- | --- |
| Source shortcode | V15.3.6 |
| Minimum assigned level | L2 |
| Chapter | Secure Coding and Architecture |
| Section | Defensive Coding |
| Adapted requirement summary | Prevent JavaScript prototype pollution. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | Browser/HTTP-specific requirement |
| Applicability rationale | The native product has no JavaScript runtime, object-prototype state or WebView. This does not waive typed native deserialization controls. |
| Native control mapping | CTRL-SCOPE: exclude this web surface; retain native IPC/UI controls under their own rows. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review feature/dependency/endpoint inventories and final binaries for the excluded surface; reopen this row if a webview or HTTP interface is added. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-15.3.7

| Field | Value |
| --- | --- |
| Source shortcode | V15.3.7 |
| Minimum assigned level | L2 |
| Chapter | Secure Coding and Architecture |
| Section | Defensive Coding |
| Adapted requirement summary | Prevent ambiguity from duplicate or mixed-source HTTP parameters. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | HTTP/web-service-specific requirement |
| Applicability rationale | No HTTP request-parameter merging exists; duplicate native configuration/IPC fields still require explicit parser policy under V1/V2/V15.3.5. |
| Native control mapping | CTRL-SCOPE: no application HTTP endpoint; protocol framing and IPC remain covered separately. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Inspect network-entry inventory and packaged dependencies; confirm absence of application HTTP routes; reassess if update/revocation HTTP integrations are introduced. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-15.4.1

| Field | Value |
| --- | --- |
| Source shortcode | V15.4.1 |
| Minimum assigned level | L3 |
| Chapter | Secure Coding and Architecture |
| Section | Safe Concurrency |
| Adapted requirement summary | Access shared concurrent state with safe synchronization. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic concurrency and resource-ownership requirement |
| Applicability rationale | Polling, configuration changes, IPC and shutdown may overlap. |
| Native control mapping | CTRL-CONCURRENCY: single owners, bounded channels, generations and atomic protected operations. |
| Decision / implementation owner | Core; engine; broker |
| Original planned verification | Use deterministic interleavings/model tests plus actual Windows races; observe stale publication, TOCTOU, deadlock/starvation and cleanup. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.3, THR-11/12/26:** SCM callbacks, cancellation and blocking-job completion share lifetimes even when the numerical core has one owner. CHECK-27: exercise failed/partial initialization, preshutdown and permitted late controls, cancellation during blocking work and repeated stop; prove correct destructor/context lifetime, no ABI unwind and bounded shutdown without TerminateThread. Use an instrumented concurrency model only for actual shared state; real Windows evidence remains necessary.

### v5.0.0-15.4.2

| Field | Value |
| --- | --- |
| Source shortcode | V15.4.2 |
| Minimum assigned level | L3 |
| Chapter | Secure Coding and Architecture |
| Section | Safe Concurrency |
| Adapted requirement summary | Make resource checks and protected use atomic against races. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic concurrency and resource-ownership requirement |
| Applicability rationale | Use handle-based OS access decisions and broker-owned policy/command generations. A prior path existence check or old user token must not be presented as atomic current authorization. |
| Native control mapping | CTRL-CONCURRENCY: single owners, bounded channels, generations and atomic protected operations. |
| Decision / implementation owner | Core; engine; broker |
| Original planned verification | Race path replacement, policy update, disconnect, suspend/resume and stale correction requests at actual owned boundaries; verify the declared commit point and no check/use gap grants extra authority. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.1, THR-20:** MSI maintenance/rollback data crosses into a more privileged intermediary and can race privileged filesystem effects CHECK-13/15/23: hostile public properties, supported/unsupported transforms, rollback records and user-controlled paths through actual install/repair/remove entrypoints cannot trigger arbitrary service/file/process effects; forward and rollback use effective caller/operation authority and stable validated handles; normal complete lifecycle succeeds
- **Revision 1.1, THR-23:** Install/update ordering, coherent rollback and exclusive ownership constrain which generation may perform effects CHECK-3/12/13/23: old delayed proposal and mixed-version helper/UI cannot mutate the new generation; fault/reboot at upgrade stages leaves one valid owner or explicit blocked recovery; rollback restores compatible state without replenishing consumed authority; successful permitted upgrade remains usable
- **Revision 1.2, THR-25:** Scalar/domain validity does not establish caller permission, trusted provenance, current ownership or an allowed resolved destination. CHECK-25 with CHECK-2/3/6/10: valid DTOs with serialized admin/validated claims, revoked or stale context, prohibited resolved/negotiated destinations and a changed resource before effect cannot gain authority. Check current state in the protected operation using the real enforcement path; a fresh authorized permitted operation succeeds. Pure validator tests do not qualify Windows authorization.
- **Revision 1.3, THR-11/22/26:** A valid old dependency result cannot retain authority after policy, source or clock generation changes. CHECK-2/3/11/27: delay a native result across timeout, cancellation, policy revocation, resume and service restart, then ensure no stale model update, fresh-trust status or clock proposal reaches an effect. Retain applied correction budgets across recovery and admit the fresh authorized neighboring operation.

### v5.0.0-15.4.3

| Field | Value |
| --- | --- |
| Source shortcode | V15.4.3 |
| Minimum assigned level | L3 |
| Chapter | Secure Coding and Architecture |
| Section | Safe Concurrency |
| Adapted requirement summary | Keep locking consistent, encapsulated and free of deadlock or livelock. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic concurrency and resource-ownership requirement |
| Applicability rationale | Polling, configuration changes, IPC and shutdown may overlap. |
| Native control mapping | CTRL-CONCURRENCY: single owners, bounded channels, generations and atomic protected operations. |
| Decision / implementation owner | Core; engine; broker |
| Original planned verification | Use deterministic interleavings/model tests plus actual Windows races; observe stale publication, TOCTOU, deadlock/starvation and cleanup. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.3, THR-11/12/26:** SCM callbacks, cancellation and blocking-job completion share lifetimes even when the numerical core has one owner. CHECK-27: exercise failed/partial initialization, preshutdown and permitted late controls, cancellation during blocking work and repeated stop; prove correct destructor/context lifetime, no ABI unwind and bounded shutdown without TerminateThread. Use an instrumented concurrency model only for actual shared state; real Windows evidence remains necessary.

### v5.0.0-15.4.4

| Field | Value |
| --- | --- |
| Source shortcode | V15.4.4 |
| Minimum assigned level | L3 |
| Chapter | Secure Coding and Architecture |
| Section | Safe Concurrency |
| Adapted requirement summary | Allocate resources fairly enough to prevent starvation. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic availability and resource-control requirement |
| Applicability rationale | Bound background work and reserve timely processing for shutdown, policy updates and broker control. UI/diagnostic or hostile-client traffic must not starve synchronization or its safety controls. |
| Native control mapping | CTRL-RESOURCE: finite buffers, tasks, handles, connection counts, rates and retries. |
| Decision / implementation owner | Engine; broker; QA |
| Original planned verification | Run bounded mixed legitimate/hostile workloads; measure control latency and fairness with explicit budgets, task/handle cleanup and recovery after overload. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.3, THR-8/26:** A current-thread runtime can submit blocking native work whose queue and lifetime outlive caller cancellation. CHECK-27 with CHECK-8/9/21: saturate TLS/DNS/native job, byte, connection and retry budgets; keep permits with jobs until actual completion, reject/coalesce before submission and avoid replacement-pool growth. Show fair scheduling when capacity exists, independent lifecycle/broker responsiveness, diagnosed saturation and a valid request after bounded recovery.


## V16 - Security Logging and Error Handling

### v5.0.0-16.1.1

| Field | Value |
| --- | --- |
| Source shortcode | V16.1.1 |
| Minimum assigned level | L2 |
| Chapter | Security Logging and Error Handling |
| Section | Security Logging Documentation |
| Adapted requirement summary | Inventory event types, formats, sinks, access and retention. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic architecture/documentation requirement |
| Applicability rationale | Include UI, engine, broker, installer/helper, provider failures and release support diagnostics; identify which OS-level events are delegated rather than app-generated. |
| Native control mapping | CTRL-DOC: versioned architecture, trust-boundary and policy inventories. |
| Decision / implementation owner | Architecture; security owner |
| Original planned verification | Compare actual event schemas and installed sinks/ACLs with the inventory; require an owner and retention rule for every sink and export. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-16.2.1

| Field | Value |
| --- | --- |
| Source shortcode | V16.2.1 |
| Minimum assigned level | L2 |
| Chapter | Security Logging and Error Handling |
| Section | General Logging |
| Adapted requirement summary | Record enough event context to reconstruct what happened. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic security logging requirement |
| Applicability rationale | Include event kind, component/versioned schema, actual authorized subject where relevant, operation/correlation identifier, UTC estimate, time-trust state, monotonic order and result. Avoid sensitive payloads. |
| Native control mapping | CTRL-LOG: typed event schemas, protected sinks, UTC/trust metadata and bounded retention. |
| Decision / implementation owner | Observability; security owner |
| Original planned verification | Execute independently known event sequences across clock steps/restarts; verify correlation and subject attribution without treating a payload-supplied identity as authoritative. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.3, THR-10/18/26:** Native event integration needs stable metadata, a bounded destination inventory and honest correlation while UTC is unreliable. CHECK-14/17/23/27: verify installer-owned Operational/Admin or explicitly chosen Application channel, provider resources, ACLs, retention and actual Event Viewer rendering; include UTC/quality/generation and monotonic correlation as specified. Exercise missing/full sinks and gross clock shifts; retain any literal synchronization gap rather than treating metadata as proof of correct UTC.

### v5.0.0-16.2.2

| Field | Value |
| --- | --- |
| Source shortcode | V16.2.2 |
| Minimum assigned level | L2 |
| Chapter | Security Logging and Error Handling |
| Section | General Logging |
| Adapted requirement summary | Synchronize logging clocks and include unambiguous UTC or offset timestamps. |
| Proposed applicability | Needs decision |
| Literal standard scope | Literal requirement or applicability needs an explicit security decision |
| Applicability rationale | This product must log while host time is wrong or not yet authenticated. All components can share OS time, but correct synchronized UTC cannot be assumed during bootstrap or holdover; monotonic order, clock epoch and trust-quality metadata are needed. |
| Native control mapping | CTRL-DECISION-LOG-TIME: UTC estimate plus trust quality, monotonic sequence and correction epochs; no claim that untrusted startup timestamps are already correct. |
| Decision / implementation owner | Security owner; architecture |
| Original planned verification | Test gross initial error, step/slew, suspend/resume and restart; verify explicit UTC/offset representation and reconstructable monotonic/epoch ordering while uncertainty is displayed honestly. |
| Open question | What literal synchronized-time guarantee is possible before trusted synchronization, and how will that unavoidable degraded phase be documented? |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.3, THR-10/18/26:** Native event integration needs stable metadata, a bounded destination inventory and honest correlation while UTC is unreliable. CHECK-14/17/23/27: verify installer-owned Operational/Admin or explicitly chosen Application channel, provider resources, ACLs, retention and actual Event Viewer rendering; include UTC/quality/generation and monotonic correlation as specified. Exercise missing/full sinks and gross clock shifts; retain any literal synchronization gap rather than treating metadata as proof of correct UTC.

### v5.0.0-16.2.3

| Field | Value |
| --- | --- |
| Source shortcode | V16.2.3 |
| Minimum assigned level | L2 |
| Chapter | Security Logging and Error Handling |
| Section | General Logging |
| Adapted requirement summary | Write or send logs only to documented sinks. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic security logging requirement |
| Applicability rationale | Time and privileged-action failures require useful bounded diagnostics without secret disclosure. |
| Native control mapping | CTRL-LOG: typed event schemas, protected sinks, UTC/trust metadata and bounded retention. |
| Decision / implementation owner | Observability; security owner |
| Original planned verification | Inspect actual event payloads and sink ACLs; force success/rejection/serialization/sink failures with sensitive sentinels and legitimate event controls. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.3, THR-10/18/26:** Native event integration needs stable metadata, a bounded destination inventory and honest correlation while UTC is unreliable. CHECK-14/17/23/27: verify installer-owned Operational/Admin or explicitly chosen Application channel, provider resources, ACLs, retention and actual Event Viewer rendering; include UTC/quality/generation and monotonic correlation as specified. Exercise missing/full sinks and gross clock shifts; retain any literal synchronization gap rather than treating metadata as proof of correct UTC.

### v5.0.0-16.2.4

| Field | Value |
| --- | --- |
| Source shortcode | V16.2.4 |
| Minimum assigned level | L2 |
| Chapter | Security Logging and Error Handling |
| Section | General Logging |
| Adapted requirement summary | Use log formats that the chosen processor can read and correlate. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic security logging requirement |
| Applicability rationale | Time and privileged-action failures require useful bounded diagnostics without secret disclosure. |
| Native control mapping | CTRL-LOG: typed event schemas, protected sinks, UTC/trust metadata and bounded retention. |
| Decision / implementation owner | Observability; security owner |
| Original planned verification | Feed actual structured events into the chosen local analysis/export path; verify schema/version handling, escaping, correlation and malformed-event rejection with independent expected fields. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.3, THR-10/18/26:** Native event integration needs stable metadata, a bounded destination inventory and honest correlation while UTC is unreliable. CHECK-14/17/23/27: verify installer-owned Operational/Admin or explicitly chosen Application channel, provider resources, ACLs, retention and actual Event Viewer rendering; include UTC/quality/generation and monotonic correlation as specified. Exercise missing/full sinks and gross clock shifts; retain any literal synchronization gap rather than treating metadata as proof of correct UTC.

### v5.0.0-16.2.5

| Field | Value |
| --- | --- |
| Source shortcode | V16.2.5 |
| Minimum assigned level | L2 |
| Chapter | Security Logging and Error Handling |
| Section | General Logging |
| Adapted requirement summary | Apply data-class rules to all logged sensitive information. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic security logging requirement |
| Applicability rationale | Never log NTS keys/cookies or raw credential material. Minimize endpoint/user/path information and explicitly review crash dumps/support exports; hashed secrets are not a default license to log them. |
| Native control mapping | CTRL-LOG: typed event schemas, protected sinks, UTC/trust metadata and bounded retention. |
| Decision / implementation owner | Observability; security owner |
| Original planned verification | Place unique sentinel values in each classified field and exercise success/failure/retry/crash-related paths; inspect all real sinks and exports for forbidden values and transformations. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.2, THR-25:** Validation errors must not expose secrets, inject logs, amplify resource use or permit partial/fail-open effects. CHECK-25 with CHECK-15/17/22: supply secret-bearing payloads and hostile field names; require allowlisted bounded error paths/codes, appropriate encoded/redacted bypass events and no raw rejected values. Rejection keeps the prior valid policy or blocks unsafe startup, never substitutes permissive critical defaults. Useful safe feedback and documented optional-cache recovery still work.
- **Revision 1.3, THR-10:** Error derives, automatic tracing and temporary secret copies can expose data despite secret wrapper types. CHECK-17/26/27: inject secret-bearing failures and hostile strings through real errors/events/exports; prove explicit bounded fields, no key/cookie/raw certificate or packet dumps, no automatic secret serialization and safe failed sinks. Qualify owned-buffer zeroing/copy paths without claiming protection from privileged dumps or forced termination.

### v5.0.0-16.3.1

| Field | Value |
| --- | --- |
| Source shortcode | V16.3.1 |
| Minimum assigned level | L2 |
| Chapter | Security Logging and Error Handling |
| Section | Security Events |
| Adapted requirement summary | Log successful and unsuccessful authentication operations. |
| Proposed applicability | Native adaptation (proposed) |
| Literal standard scope | Authentication/audit requirement adapted to local Windows and protocol events |
| Applicability rationale | Log owned Windows IPC admission and TLS/NTS peer-authentication outcomes with method and reason classes. Windows desktop logon/factor events remain OS-owned; do not claim to log every OS authentication operation. |
| Native control mapping | CTRL-LOG: classified local and protocol security events; CTRL-IPC: verified identity and bounded notifications. |
| Decision / implementation owner | Observability; Windows integration; security owner |
| Original planned verification | Trigger owned success and rejection events with actual Windows clients and TLS peers; inspect accessible notifications and protected logs, and document the delegated OS boundary. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-16.3.2

| Field | Value |
| --- | --- |
| Source shortcode | V16.3.2 |
| Minimum assigned level | L2 |
| Chapter | Security Logging and Error Handling |
| Section | Security Events |
| Adapted requirement summary | Log authorization denials and all decisions at L3. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic security logging requirement |
| Applicability rationale | Define audit events for every protected decision, including allowed sensitive reads and broker effects. Bounded logging/flood control must preserve the approved audit semantics or explicitly document a gap rather than silently drop required events. |
| Native control mapping | CTRL-LOG: typed event schemas, protected sinks, UTC/trust metadata and bounded retention. |
| Decision / implementation owner | Observability; security owner |
| Original planned verification | Enumerate authorization decision sites and trigger allow/deny cases; stress the sink/queue and prove the documented no-loss or loss-signaling behavior without permitting log exhaustion to authorize unsafe actions. |
| Open question | How will complete L3 decision auditing coexist with finite queues and disk limits; what explicit loss/fail-safe policy is approved? |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-16.3.3

| Field | Value |
| --- | --- |
| Source shortcode | V16.3.3 |
| Minimum assigned level | L2 |
| Chapter | Security Logging and Error Handling |
| Section | Security Events |
| Adapted requirement summary | Log required security events and attempted control bypasses. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic security logging requirement |
| Applicability rationale | Time and privileged-action failures require useful bounded diagnostics without secret disclosure. |
| Native control mapping | CTRL-LOG: typed event schemas, protected sinks, UTC/trust metadata and bounded retention. |
| Decision / implementation owner | Observability; security owner |
| Original planned verification | Inspect actual event payloads and sink ACLs; force success/rejection/serialization/sink failures with sensitive sentinels and legitimate event controls. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.2, THR-25:** Validation errors must not expose secrets, inject logs, amplify resource use or permit partial/fail-open effects. CHECK-25 with CHECK-15/17/22: supply secret-bearing payloads and hostile field names; require allowlisted bounded error paths/codes, appropriate encoded/redacted bypass events and no raw rejected values. Rejection keeps the prior valid policy or blocks unsafe startup, never substitutes permissive critical defaults. Useful safe feedback and documented optional-cache recovery still work.
- **Revision 1.3, THR-8/10/26:** Blocked library work, exhausted budgets and backend TLS/policy failures need observable bounded outcomes. CHECK-17/27: verify stable events for bypass attempts, certificate-policy failure, saturation, stale-result rejection and stop/recovery, with rate limiting/counters and no sensitive payload. Demonstrate that logging failure cannot authorize a denied effect or block clock cleanup, and legitimate operation remains diagnosable.

### v5.0.0-16.3.4

| Field | Value |
| --- | --- |
| Source shortcode | V16.3.4 |
| Minimum assigned level | L2 |
| Chapter | Security Logging and Error Handling |
| Section | Security Events |
| Adapted requirement summary | Log unexpected errors and security-control failures. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic security logging requirement |
| Applicability rationale | Time and privileged-action failures require useful bounded diagnostics without secret disclosure. |
| Native control mapping | CTRL-LOG: typed event schemas, protected sinks, UTC/trust metadata and bounded retention. |
| Decision / implementation owner | Observability; security owner |
| Original planned verification | Inspect actual event payloads and sink ACLs; force success/rejection/serialization/sink failures with sensitive sentinels and legitimate event controls. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.3, THR-8/10/26:** Blocked library work, exhausted budgets and backend TLS/policy failures need observable bounded outcomes. CHECK-17/27: verify stable events for bypass attempts, certificate-policy failure, saturation, stale-result rejection and stop/recovery, with rate limiting/counters and no sensitive payload. Demonstrate that logging failure cannot authorize a denied effect or block clock cleanup, and legitimate operation remains diagnosable.

### v5.0.0-16.4.1

| Field | Value |
| --- | --- |
| Source shortcode | V16.4.1 |
| Minimum assigned level | L2 |
| Chapter | Security Logging and Error Handling |
| Section | Log Protection |
| Adapted requirement summary | Encode event data to prevent forged or injected log entries. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic security logging requirement |
| Applicability rationale | Time and privileged-action failures require useful bounded diagnostics without secret disclosure. |
| Native control mapping | CTRL-LOG: typed event schemas, protected sinks, UTC/trust metadata and bounded retention. |
| Decision / implementation owner | Observability; security owner |
| Original planned verification | Inject line breaks, delimiters, control characters, oversized names and hostile-looking structured fragments; verify one event remains one typed event with correct parser-visible fields. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.2, THR-25:** Validation errors must not expose secrets, inject logs, amplify resource use or permit partial/fail-open effects. CHECK-25 with CHECK-15/17/22: supply secret-bearing payloads and hostile field names; require allowlisted bounded error paths/codes, appropriate encoded/redacted bypass events and no raw rejected values. Rejection keeps the prior valid policy or blocks unsafe startup, never substitutes permissive critical defaults. Useful safe feedback and documented optional-cache recovery still work.

### v5.0.0-16.4.2

| Field | Value |
| --- | --- |
| Source shortcode | V16.4.2 |
| Minimum assigned level | L2 |
| Chapter | Security Logging and Error Handling |
| Section | Log Protection |
| Adapted requirement summary | Protect logs from unauthorized reading and modification. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic security logging requirement |
| Applicability rationale | Use protected OS sinks/ACLs and a threat model for compromised app processes. Same-host administrator control and an app forging future events are not solved merely by an append-only file flag. |
| Native control mapping | CTRL-LOG: typed event schemas, protected sinks, UTC/trust metadata and bounded retention. |
| Decision / implementation owner | Observability; security owner |
| Original planned verification | Inspect actual sink ownership/ACLs; attempt read, rewrite, truncation and deletion from engine/UI/ordinary-user principals, and document what a privileged host adversary can still change. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-16.4.3

| Field | Value |
| --- | --- |
| Source shortcode | V16.4.3 |
| Minimum assigned level | L2 |
| Chapter | Security Logging and Error Handling |
| Section | Log Protection |
| Adapted requirement summary | Send logs securely to a separate analysis and alerting system. |
| Proposed applicability | Needs decision |
| Literal standard scope | Literal requirement or applicability needs an explicit security decision |
| Applicability rationale | Local files alone do not establish a logically separate protected analysis/detection system. A Windows event service may provide separation from the app process, but the exact protection/analysis/alerting guarantee needs evidence; external forwarding is not in the baseline. |
| Native control mapping | CTRL-DECISION-AUDIT-SINK: separate protected log-analysis boundary and explicit deployment choice; no external transmission is authorized by this worksheet. |
| Decision / implementation owner | Security owner; architecture |
| Original planned verification | Assess the actual chosen event service or approved enterprise collector against compromise of each app process; test transport, availability, ACLs, alerting and retention without silently enabling telemetry. |
| Open question | Does a protected local Windows event subsystem satisfy the documented threat boundary, or is optional managed forwarding/explicit deviation required? |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-16.5.1

| Field | Value |
| --- | --- |
| Source shortcode | V16.5.1 |
| Minimum assigned level | L2 |
| Chapter | Security Logging and Error Handling |
| Section | Error Handling |
| Adapted requirement summary | Return sanitized generic errors for sensitive unexpected failures. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic secure failure/error handling requirement |
| Applicability rationale | Give users an actionable safe error class and correlation ID; keep authorized diagnostics structured and secret-free. Do not expose stacks, provider internals or raw packets merely to explain a sync failure. |
| Native control mapping | CTRL-ERROR: explicit failure strata and bounded recovery; CTRL-CLOCK: no effect from invalid/failed state. |
| Decision / implementation owner | Engine; broker; observability |
| Original planned verification | Force representative parser, TLS, IPC, privilege and storage failures; inspect ordinary-user UI/status and restricted diagnostics for useful separation and forbidden disclosure. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.2, THR-25:** Validation errors must not expose secrets, inject logs, amplify resource use or permit partial/fail-open effects. CHECK-25 with CHECK-15/17/22: supply secret-bearing payloads and hostile field names; require allowlisted bounded error paths/codes, appropriate encoded/redacted bypass events and no raw rejected values. Rejection keeps the prior valid policy or blocks unsafe startup, never substitutes permissive critical defaults. Useful safe feedback and documented optional-cache recovery still work.

### v5.0.0-16.5.2

| Field | Value |
| --- | --- |
| Source shortcode | V16.5.2 |
| Minimum assigned level | L2 |
| Chapter | Security Logging and Error Handling |
| Section | Error Handling |
| Adapted requirement summary | Remain secure when an external dependency fails. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic secure failure/error handling requirement |
| Applicability rationale | DNS, network peers, TLS validation, disk, event sinks and OS calls may fail. Degrade to explicit unsynchronized/holdover state and bounded retry according to policy, without promoting hints or stale samples to authenticated success. |
| Native control mapping | CTRL-ERROR: explicit failure strata and bounded recovery; CTRL-CLOCK: no effect from invalid/failed state. |
| Decision / implementation owner | Engine; broker; observability |
| Original planned verification | Inject each failure after a valid operation starts; verify deadlines, cancellation, safe clock authority, truthful state and bounded recovery with legitimate positive controls. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-16.5.3

| Field | Value |
| --- | --- |
| Source shortcode | V16.5.3 |
| Minimum assigned level | L2 |
| Chapter | Security Logging and Error Handling |
| Section | Error Handling |
| Adapted requirement summary | Fail securely on validation and exception paths. |
| Proposed applicability | Directly applicable (proposed) |
| Literal standard scope | Generic secure failure/error handling requirement |
| Applicability rationale | Network, storage, crypto and Windows-call failures must not cause unsafe clock effects or false success. |
| Native control mapping | CTRL-ERROR: explicit failure strata and bounded recovery; CTRL-CLOCK: no effect from invalid/failed state. |
| Decision / implementation owner | Engine; broker; observability |
| Original planned verification | Inject faults at real owned seams after valid initiation; assert safe state, useful sanitized diagnostics and recovery without duplicated effects. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

**Revision-specific additions — planned, not executed:**

- **Revision 1.2, THR-25:** Validation errors must not expose secrets, inject logs, amplify resource use or permit partial/fail-open effects. CHECK-25 with CHECK-15/17/22: supply secret-bearing payloads and hostile field names; require allowlisted bounded error paths/codes, appropriate encoded/redacted bypass events and no raw rejected values. Rejection keeps the prior valid policy or blocks unsafe startup, never substitutes permissive critical defaults. Useful safe feedback and documented optional-cache recovery still work.

### v5.0.0-16.5.4

| Field | Value |
| --- | --- |
| Source shortcode | V16.5.4 |
| Minimum assigned level | L3 |
| Chapter | Security Logging and Error Handling |
| Section | Error Handling |
| Adapted requirement summary | Provide last-resort exception handling without losing availability or diagnostics. |
| Proposed applicability | Needs decision |
| Literal standard scope | Literal requirement or applicability needs an explicit security decision |
| Applicability rationale | A Rust panic may indicate a violated invariant. Continuing a corrupted or logically invalid privileged process merely to satisfy a catch-all clause may be unsafe; controlled exit and SCM restart do not literally guarantee that the process stays alive. |
| Native control mapping | CTRL-DECISION-PANIC: documented recoverable/fatal error policy; minimal crash diagnostics, no unsafe continuation and bounded service recovery. |
| Decision / implementation owner | Security owner; architecture |
| Original planned verification | Classify recoverable Result errors versus invariant panics/FFI faults; test sanitized fatal reporting, fail-safe cessation of clock effects and bounded restart without treating catch_unwind as a universal safety boundary. |
| Open question | Which faults can safely remain in-process, and where must an explicit L3 deviation favor safe shutdown plus supervised recovery? |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |


## V17 - WebRTC

### v5.0.0-17.1.1

| Field | Value |
| --- | --- |
| Source shortcode | V17.1.1 |
| Minimum assigned level | L2 |
| Chapter | WebRTC |
| Section | TURN Server |
| Adapted requirement summary | Restrict TURN destinations to allowed nonspecial addresses. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | WebRTC/TURN/media/signaling-specific requirement |
| Applicability rationale | The application has no WebRTC, TURN relay, DTLS-SRTP media or signaling server. NTP UDP traffic is not media traffic. |
| Native control mapping | CTRL-SCOPE: exclude media/relay features; UDP robustness remains under generic/protocol controls. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review protocol/listener/dependency inventory for absence of WebRTC/media components; reopen if introduced. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-17.1.2

| Field | Value |
| --- | --- |
| Source shortcode | V17.1.2 |
| Minimum assigned level | L3 |
| Chapter | WebRTC |
| Section | TURN Server |
| Adapted requirement summary | Prevent TURN port-allocation exhaustion. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | WebRTC/TURN/media/signaling-specific requirement |
| Applicability rationale | The application has no WebRTC, TURN relay, DTLS-SRTP media or signaling server. NTP UDP traffic is not media traffic. |
| Native control mapping | CTRL-SCOPE: exclude media/relay features; UDP robustness remains under generic/protocol controls. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review protocol/listener/dependency inventory for absence of WebRTC/media components; reopen if introduced. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-17.2.1

| Field | Value |
| --- | --- |
| Source shortcode | V17.2.1 |
| Minimum assigned level | L2 |
| Chapter | WebRTC |
| Section | Media |
| Adapted requirement summary | Manage DTLS certificate keys according to policy. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | WebRTC/TURN/media/signaling-specific requirement |
| Applicability rationale | The application has no WebRTC, TURN relay, DTLS-SRTP media or signaling server. NTP UDP traffic is not media traffic. |
| Native control mapping | CTRL-SCOPE: exclude media/relay features; UDP robustness remains under generic/protocol controls. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review protocol/listener/dependency inventory for absence of WebRTC/media components; reopen if introduced. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-17.2.2

| Field | Value |
| --- | --- |
| Source shortcode | V17.2.2 |
| Minimum assigned level | L2 |
| Chapter | WebRTC |
| Section | Media |
| Adapted requirement summary | Use approved DTLS suites and SRTP key-establishment profiles. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | WebRTC/TURN/media/signaling-specific requirement |
| Applicability rationale | The application has no WebRTC, TURN relay, DTLS-SRTP media or signaling server. NTP UDP traffic is not media traffic. |
| Native control mapping | CTRL-SCOPE: exclude media/relay features; UDP robustness remains under generic/protocol controls. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review protocol/listener/dependency inventory for absence of WebRTC/media components; reopen if introduced. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-17.2.3

| Field | Value |
| --- | --- |
| Source shortcode | V17.2.3 |
| Minimum assigned level | L2 |
| Chapter | WebRTC |
| Section | Media |
| Adapted requirement summary | Verify SRTP authentication before accepting media. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | WebRTC/TURN/media/signaling-specific requirement |
| Applicability rationale | The application has no WebRTC, TURN relay, DTLS-SRTP media or signaling server. NTP UDP traffic is not media traffic. |
| Native control mapping | CTRL-SCOPE: exclude media/relay features; UDP robustness remains under generic/protocol controls. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review protocol/listener/dependency inventory for absence of WebRTC/media components; reopen if introduced. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-17.2.4

| Field | Value |
| --- | --- |
| Source shortcode | V17.2.4 |
| Minimum assigned level | L2 |
| Chapter | WebRTC |
| Section | Media |
| Adapted requirement summary | Continue safe media processing despite malformed SRTP packets. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | WebRTC/TURN/media/signaling-specific requirement |
| Applicability rationale | The application has no WebRTC, TURN relay, DTLS-SRTP media or signaling server. NTP UDP traffic is not media traffic. |
| Native control mapping | CTRL-SCOPE: exclude media/relay features; UDP robustness remains under generic/protocol controls. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review protocol/listener/dependency inventory for absence of WebRTC/media components; reopen if introduced. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-17.2.5

| Field | Value |
| --- | --- |
| Source shortcode | V17.2.5 |
| Minimum assigned level | L3 |
| Chapter | WebRTC |
| Section | Media |
| Adapted requirement summary | Maintain media service under legitimate-user SRTP floods. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | WebRTC/TURN/media/signaling-specific requirement |
| Applicability rationale | The application has no WebRTC, TURN relay, DTLS-SRTP media or signaling server. NTP UDP traffic is not media traffic. |
| Native control mapping | CTRL-SCOPE: exclude media/relay features; UDP robustness remains under generic/protocol controls. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review protocol/listener/dependency inventory for absence of WebRTC/media components; reopen if introduced. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-17.2.6

| Field | Value |
| --- | --- |
| Source shortcode | V17.2.6 |
| Minimum assigned level | L3 |
| Chapter | WebRTC |
| Section | Media |
| Adapted requirement summary | Prevent the DTLS ClientHello race vulnerability. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | WebRTC/TURN/media/signaling-specific requirement |
| Applicability rationale | The application has no WebRTC, TURN relay, DTLS-SRTP media or signaling server. NTP UDP traffic is not media traffic. |
| Native control mapping | CTRL-SCOPE: exclude media/relay features; UDP robustness remains under generic/protocol controls. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review protocol/listener/dependency inventory for absence of WebRTC/media components; reopen if introduced. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-17.2.7

| Field | Value |
| --- | --- |
| Source shortcode | V17.2.7 |
| Minimum assigned level | L3 |
| Chapter | WebRTC |
| Section | Media |
| Adapted requirement summary | Keep recording service available during SRTP floods. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | WebRTC/TURN/media/signaling-specific requirement |
| Applicability rationale | The application has no WebRTC, TURN relay, DTLS-SRTP media or signaling server. NTP UDP traffic is not media traffic. |
| Native control mapping | CTRL-SCOPE: exclude media/relay features; UDP robustness remains under generic/protocol controls. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review protocol/listener/dependency inventory for absence of WebRTC/media components; reopen if introduced. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-17.2.8

| Field | Value |
| --- | --- |
| Source shortcode | V17.2.8 |
| Minimum assigned level | L3 |
| Chapter | WebRTC |
| Section | Media |
| Adapted requirement summary | Match DTLS certificates to the negotiated SDP fingerprint. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | WebRTC/TURN/media/signaling-specific requirement |
| Applicability rationale | The application has no WebRTC, TURN relay, DTLS-SRTP media or signaling server. NTP UDP traffic is not media traffic. |
| Native control mapping | CTRL-SCOPE: exclude media/relay features; UDP robustness remains under generic/protocol controls. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review protocol/listener/dependency inventory for absence of WebRTC/media components; reopen if introduced. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-17.3.1

| Field | Value |
| --- | --- |
| Source shortcode | V17.3.1 |
| Minimum assigned level | L2 |
| Chapter | WebRTC |
| Section | Signaling |
| Adapted requirement summary | Rate-limit signaling floods while serving legitimate messages. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | WebRTC/TURN/media/signaling-specific requirement |
| Applicability rationale | The application has no WebRTC, TURN relay, DTLS-SRTP media or signaling server. NTP UDP traffic is not media traffic. |
| Native control mapping | CTRL-SCOPE: exclude media/relay features; UDP robustness remains under generic/protocol controls. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review protocol/listener/dependency inventory for absence of WebRTC/media components; reopen if introduced. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

### v5.0.0-17.3.2

| Field | Value |
| --- | --- |
| Source shortcode | V17.3.2 |
| Minimum assigned level | L2 |
| Chapter | WebRTC |
| Section | Signaling |
| Adapted requirement summary | Handle malformed signaling without losing service availability. |
| Proposed applicability | Proposed N/A |
| Literal standard scope | WebRTC/TURN/media/signaling-specific requirement |
| Applicability rationale | The application has no WebRTC, TURN relay, DTLS-SRTP media or signaling server. NTP UDP traffic is not media traffic. |
| Native control mapping | CTRL-SCOPE: exclude media/relay features; UDP robustness remains under generic/protocol controls. |
| Decision / implementation owner | Architecture; QA |
| Original planned verification | Review protocol/listener/dependency inventory for absence of WebRTC/media components; reopen if introduced. |
| Open question | None recorded |
| Implementation status | Not implemented |
| Evidence status | Not executed |
| Classification status | Draft; requires owner review |

