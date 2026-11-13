# NevroTick — owner decisions and remaining policy choices

**Document:** NV-DEC-001, revision 1.25<br>
**Decision dates:** 29 October 2026; OD-07 added 30 October; OD-08 added 2 November; OD-05 delivery direction amended, OD-01 future replacement and OD-02 Roughtime/coordination directions added and bootstrap priority refined 3 November; OD-09 leap research/design integration added 4 November; OD-10 mandatory mathematical proofs, OD-11 SOCKS5 scope/deferral, OD-12 GeoIP experiment placement, OD-13 default Slint UI and OD-14 compact Windows platform scope added 5 November; OD-10 first-release deferral and proof-purpose/completion amendment, OD-13 complete GUI migration/licence clarification and OD-15 localization integration added 6 November 2026, Asia/Tbilisi<br>
**Owner:** David Osipov  
**Security clarification:** every JSON/CBOR input remains potentially malicious, including contributor/translator catalogs and packaged/generated/English data.
**6 November CPU/hardening correction:** OD-07's v3 minimum is mandatory and now configured; OD-05 requires every applicable final-product protection at its strongest supported compatible setting. Missing implementation or qualification is open work, not an optional policy.
**Research baseline:** [NevroTick at db53dda222739b9b076b1b47a81ace7761b1596b](https://github.com/DavidOsipov/NevroTick/tree/db53dda222739b9b076b1b47a81ace7761b1596b)  
**Repository integration base:** [77ad8c416db32d5edfdc9e6a538d76637eaf44f0](https://github.com/DavidOsipov/NevroTick/tree/77ad8c416db32d5edfdc9e6a538d76637eaf44f0); later tooling, SBOM and dependency-refresh changes are preserved.  
**Status:** Records owner instructions, engineering recommendations and unresolved research separately. This is documentation, not implementation or release qualification.

**Current post-V1 scheduling:** the [13 November amendment to OD-16](#201-mathematics-experiments-first--13-november-2026) places mathematical estimation experiments first after actual V1 release, followed by multithreading, Khronos/Ananke, GeoIP and Android. It supersedes only MT-0's first position from the 7 November decision.

**Later 13 November qualification direction:** [OD-18](#22-od-18--literature-derived-qualification-before-v1) places small literature-derived tests/cases before V1 and records longer implementations in the roadmap and detailed documentation. Existing phase owners and the post-V1 order remain authoritative.

**Later 7 November distribution decision:** [OD-17](#21-od-17--winget-capability-before-v1) requires WinGet capability before V1, preserving OD-05's complete Store-MSIX target; the current post-V1 order is governed by amended OD-16.

**8 November signing decision:** [OD-05 §7.7](#77-sign-every-supported-shipped-artifact--8-november-2026)
requires signatures on the installer/package and every shipped file with a
supported format-appropriate signing mechanism, including nonbinary files.
This supersedes the unsigned-MSI/OpenPGP permission and catalog-only exemptions
for individually signable payloads; implementation and qualification remain open.

**Later 8 November signing-resource clarification:** [OD-05 §7.8](#78-signpath-and-interim-self-signed-msi--8-november-2026)
permits an interim self-signed MSI profile while David lacks a personal publicly
trusted code-signing certificate. All-file signing remains required; public
Windows trust and signature correctness are separate results.

**4 November release-operations addition:** section 7.4 records authorized
research/documentation and a proposed support baseline, not a new accepted
maintenance commitment or release approval.

**Later 4 November provenance decision:** section 7.5 records David's accepted
instruction to retain enhanced source-to-Store evidence as optional work, with
no new release blocker. This decision is separate from the proposed support
terms in section 7.4.

## 1. Authority and purpose

This record captures David's replies about reuse, correction policy, default servers, case-by-case decisions, release signing and future research. It also incorporates his supplied [NTS server directory](https://github.com/jauderho/nts-servers) as a discovery input.

Read this alongside the [architecture](windows-time-architecture.md), [feature plan](windows-time-features.md), [signing guide](windows-time-signing-guide.md) and [research handoff](nevrotick-research-handoff.md). The accepted changes are integrated into those canonical documents. Use this record to distinguish owner decisions from pending recommendations when reading or changing them. Existing requirements remain in force except where an accepted change below explicitly amends them. Recommendations do not become owner decisions merely because an agent wrote them down.

OD-13 owns the complete Slint replacement; OD-15 owns hostile-catalog localization. The current UI-M0 restriction remains intact: fixture-only, unelevated UI, with D1/D2/D3 deferred at their existing gates. These decisions enable source migration, not live service/clock authority or native qualification, and do not block unrelated permitted product work.

## 2. Decision summary

| ID | Owner instruction | Recorded status | What still needs evidence or a decision |
| --- | --- | --- | --- |
| OD-01 | Take selected upstream code, stress-test it, and alter it when bugs or justified optimizations are found; independently replace Chrony-derived implementations after Roughtime | **Accepted current reuse and future replacement direction** | Exact source/dependency licence inventory and notices; permitted Rust/native graph; justified behavioral deviations; future replacement qualification, rights review and public licence selection |
| OD-02 | Support very large clock recovery with several NTS sources, potentially a 100-year error; be more conservative with plain NTP; add a future Roughtime bootstrap stage for a badly wrong initial date | **Accepted behavior goal and future bootstrap direction** | Supported true-date interval, era anchor, exceptional authenticated quorum, direction/startup/runtime rules, correction budgets and real Windows evidence; RGT-0 protocol/provider/trust qualification. Twelve hours is a tentative plain-NTP candidate, not an approved constant |
| OD-03 | Prefer `pool.ntp.org` as the default source; use the supplied NTS directory to find secure servers | **Accepted provider preference; default-profile interaction unresolved** | Pool vendor-zone allocation; which role the Pool fills alongside NTS; qualified NTS operators and compatible time scales |
| OD-04 | Bring concrete policy choices to David when agents encounter them and record the outcomes | **Accepted decision process** | Individual exception decisions stay open until a bounded proposal and evidence exist |
| OD-05 | Prepare one complete hardened MSIX for David's Store attempt with services; sign the installer/package and every supported shipped artifact, including nonbinary files. Preserve approved upstream signatures and David's release authority | **Accepted delivery/hardening direction; signing scope amended 8 November 2026** | Production per-format signing capability, complete per-file and package evidence, Store preservation of inner signatures, least-privilege provisioning/recovery, native mitigations and actual Store acceptance; the unsigned-MSI/OpenPGP exception is superseded |
| OD-06 | Leave remaining research questions documented for another AI to investigate | **Deferred research** | The companion brief names questions, owners, evidence and stop conditions; deferral grants no implementation waiver |
| OD-07 | Require x86-64-v3 as the Windows product minimum and configure it now; retain runtime-selected higher-feature optimizations | **Accepted 30 October; implementation correction explicitly required 6 November 2026** | The target config and exact admission controls select v3. Qualify CPU/OS/VM startup admission, final artifacts, exact-feature dispatch, numerical equivalence and measured benefit; historical generic receipts do not qualify this configuration |
| OD-08 | Build the future Windows DNS feature for the whole computer and its applications; include DNSSEC with hardware acceleration and multiple CPU cores; document the adaptive-cache proposal | **Accepted product and optimization direction, 2 November 2026; detailed cache design remains an evaluated proposal** | Windows integration and observed application coverage, DoQ/bootstrap/pin policy, DNSSEC/crypto admission, CPU dispatch and worker bounds, statistics privacy, cache parameters and measured benefit |
| OD-09 | Use several research agents, including GitHub MCP and Windows Learn MCP, to select a concrete leap policy that handles silent servers and integrate it into repository documents | **Authorized research and documentation, 4 November 2026; engineering design selected in §13** | Implementation, native mapping/event behavior, finite guard/recovery limits and exact event-time evidence remain S1.4/OPEN-7/S0.4/P4; no new clock-authority or execution waiver |
| OD-10 | Defer formal proof work until after the first released software version (V1); require useful product guarantees when it resumes | **Amended by explicit owner instruction, 6 November 2026** | [First-release deferral](#141-first-release-deferral--6-november-2026) supersedes proof-first sequencing and first-release proof prerequisites; [proof purpose and completion](#142-proof-purpose-and-completion--6-november-2026) governs scope, stopping and honest accounting; preserve historical scoped evidence, post-V1 obligations and every nonformal gate |
| OD-11 | Design a concrete SOCKS5-only proxy profile and its checks; defer it to the roadmap if implementation/support cost is excessive | **Authorized conditional deferral, 5 November 2026; engineering assessment selects deferral** | First Windows delivery remains Direct; SOCKS-0 retains a limited future loopback/no-auth/local-DNS profile and SOCKS-C01. Protocol, dependency, Windows, resolver/verifier, timing and support qualification remain unexecuted |
| OD-12 | Put the geography-assisted time-source selection experiment in the roadmap after Khronos | **Accepted future experiment placement, 5 November 2026** | GEO-0/GEO-C01 compare optional geographic ordering with ordinary acquisition/history reuse. Evaluation/disposition precedes Android; adoption, data/provider choice and any production implementation remain separate decisions supported by evidence |
| OD-13 | Use Slint as the default UI toolkit and complete Win32 replacement; license original GUI code GPL-3.0-only while preserving GPL-2.0-only Core/engine | **Accepted toolkit direction, 5 November; GUI composition clarified 6 November 2026** | Qualify the exact Slint/backend/feature/licence graph, generated-code safety boundary and migrated Windows UI; preserve UI-M0 limits, D1–D3 re-entry, resource and complete-package gates |
| OD-14 | Limit initial intended OS scope to Windows 11 26H2 and a corresponding Windows Server; delegate the other parameters | **Accepted scope direction, 5 November 2026; researched counterpart and engineering fixtures recorded in §18** | Windows Server 2025 Desktop Experience is the selected current counterpart; exact OS/CPU/protection profiles, suitable available environments, complete MSIX delivery and native results remain open. This is not first-stable support approval |
| OD-15 | Make the UI easy to translate through separate string catalogs; prefer binary delivery such as CBOR | **Accepted localization requirement and binary-format preference, 5 November 2026** | Readable authoring → validated CBOR resource design; typed Rust/Slint/tray/notification bridge, codec/formatter and package profile, actual catalog/runtime/native evidence. English remains the adopted locale; independently imported user language packs remain a separate scope choice |
| OD-16 | Run mathematical estimation experiments first after actual V1 release, then the existing multithreading plan before Khronos | **7 November scheduling direction amended by explicit owner instruction, 13 November 2026** | MATH-0/MATH-C01 comparison and disposition precede MT-0/MT-C01; no production estimator or performance gain is selected. Existing V1 correctness and native lifecycle duties are not deferred |
| OD-17 | Add WinGet distribution capability before V1, after comprehensive verification of the supplied AI proposal, and record the plan/roadmap | **Accepted capability and scheduling requirement, 7 November 2026** | WGT-0 within P6/P7: real source/identity, metadata/release preparation and native qualification; WGT-C01 readiness remains distinct from WGT-C02 external catalog availability. Preserve Store-first complete MSIX; no automatic MSI selection, publication or update guarantee |
| OD-18 | Add small literature-derived tests/cases before V1; document longer implementation in ROADMAP.md and detailed docs | **Accepted qualification and planning direction, 13 November 2026** | Actual-consumer tests and reached V1 cases need candidate-specific results; absent future runtime cases remain prepared, not executed. LIT-0/NV-NCR-001 map longer work to existing owners without adopting TPKI, NTxMon or new estimator constants |

No item here establishes a completed security assessment, provider acceptance, safe numerical bound, verified release, or native Windows result.

## 3. OD-01 — source-derived port with independent verification

David intends to reuse actual code and permit evidence-driven repairs and optimization. Treat that as a source-derived port. There is no fixed percentage of changed code that agents can use to declare it independent.

The selected Chrony baseline remains **4.9, commit `8df4f1263e206585e6c4e61352d042c2dd12916a`**. For example, the inspected selection-code header grants GNU GPL version 2 without an “or later” clause. The GNU FAQ treats translation into another programming language as modification covered by the original grant. Fixing or optimizing a Rust translation therefore does not establish permission to relabel the derived code GPL-3.0-or-later. This record confirms the reuse direction; it does not invent an additional licence grant or declare the whole future dependency graph cleared. [S1, S2]

Agents must maintain a port ledger containing upstream project/revision/file/function, selected licence, retained notice, Rust owner, independent test expectations, upstream tests reused, and intentional deviations. Preserve any reproducible upstream counterexample. A differential disagreement requires investigation: either the port, the reference, the fixture or the assumption may be wrong.

Apply the existing architecture §20.1 and development-hardening §9.7 policy: independent numerical expectations, property/fuzz tests, adversarial simulation, bounded stress, actual Windows integration checks where required, and performance measurements with protections enabled. Compare optimization against correctness and resource evidence; neither faster output nor agreement with Chrony alone is sufficient. Keep bug fixes and performance changes separately reviewable where practical.

Before shipping, resolve the root application licence and actual source/native/build/procedural-macro graph under OPEN-1/CHECK-20/26. Keep proprietary branding and application-source licensing explicit and separate. Do not remove upstream attribution, fabricate clean-room provenance or treat process boundaries as automatic licensing exemptions.

**Future direction — 3 November 2026 (direct owner instruction):** David placed
[CHR-0, independent replacement of Chrony-derived implementations](../ROADMAP.md#chr-0--independently-replace-chrony-derived-implementations),
immediately after the Roughtime stage. The goal is a qualified replacement that
does not rely on Chrony's licence. The future public distribution licence remains
undecided; David will grant Microsoft permission to use and incorporate the
resulting code into Windows, including proprietary components, within the rights
established for it. The roadmap owns the replacement scope and exit evidence.
Current source-derived development, GPL-2.0-only obligations and provenance remain
in force until the replacement and release decisions are qualified; this future
direction is not an immediate licence change or an upstream permission grant.

**Owner escalation:** a proposed reuse/dependency change requires incompatible rights, a deliberate algorithm deviation changes security or observable behavior, or a measured performance tradeoff changes the product requirements. Routine compatible fixes within the approved contracts need no repeated policy approval.

## 4. OD-02 — distinguish authenticated recovery from ordinary correction

### 4.1 What is accepted

The product should aim to recover a badly wrong local date without an arbitrary small limit merely because the offset is large, provided the authenticated evidence and correction policy justify it. A local clock wrong by approximately 100 years is an explicit research/acceptance target.

That is different from accepting an unrestricted target date or allowing any large step during normal operation. Until the wider path is qualified, existing conservative broker limits apply. Preserve source agreement, fresh evidence, current machine-policy authorization, sole clock ownership, cumulative accounting, and acknowledgment of the actual applied effect.

**Future bootstrap direction — 3 November 2026 (direct owner clarification):**
David identified Roughtime servers as the intended initial time source for a
severely wrong local clock. [RGT-0](../ROADMAP.md#rgt-0--roughtime-bootstrap-for-a-badly-wrong-clock)
records the future stage: establish an authenticated approximate-time interval,
qualify its handoff to NTS certificate validation and date/era interpretation,
then complete the existing authenticated recovery and broker gate. Protocol,
provider/key and Windows-verifier qualification remain outstanding. This extends
the recovery plan; it does not make Roughtime an implemented source, permit a
direct bootstrap clock write or replace the current recovery policy before its
extension is qualified.

**Coordination request — 3 November 2026 (direct owner instruction):** record
the Roughtime research and design its cooperation with Khronos/Ananke.
[NV-TA-001](plans/time-assurance-integration.md) and its
[shared future task](../tasks/time-assurance-integration.md) own that proposal.
The design separates the initial rough interval, primary NTS, candidate watchdog
and Ananke-inspired pool admission, preserving one clock broker and explicit
dependencies between evidence. This instruction selects the documentation/design
work; it does not adopt Khronos, a provider/key list, new quorum, direct Roughtime
clock correction or watchdog inhibition policy. Those decisions remain subject
to their concrete evidence and existing owner-decision process.

**Later bootstrap-priority refinement — 3 November 2026 (direct owner instruction):**
David selected Roughtime as the preferred initial date source before NTS, with
Khronos/Ananke using an NTP population only when Roughtime is unavailable or
cannot provide a usable date for NTS. [NV-TA-001 revision 1.1](plans/time-assurance-integration.md)
records the source census and the bounded provisional-date reserve interpretation.
This future automatic-reserve role supersedes the earlier periodic-watchdog
proposal; continuous monitoring is a deferred alternative. It does not establish
that every NTS failure is a date error or that a large offset from a broken local
clock makes signed evidence wrong.

Both proposed routes complete the applicable NTS identity/trust/revocation,
fresh-observation, configured agreement, candidate-time revalidation and broker
gates. This instruction does not select a preliminary global clock step, weaker
NTS quorum, exact reserve limits or a production source list. Retain existing
cached/strict NTS when its trusted basis is adequate; the priority concerns cold
date bootstrap. Until this scoped extension is specified and qualified, the
current strict-default/explicit-helper baseline remains. OD-03 still owns Pool
vendor deployment and default-source admission. The earlier request and review
remain historical evidence for their original scope.

### 4.2 Roles of the sources

- **NTS sources:** supply authenticated observations eligible for agreement and selection under the configured policy.
- **Plain NTP helper:** supplies a bounded provisional date to resolve certificate-time circularity, under the current explicit-enable baseline or the separately qualified future automatic-reserve profile above. It has no vote in the NTS source population and no direct clock-setting authority.
- **Broker:** makes the current authorization/effect decision after all recovery checks pass. A successful handshake or an engine proposal alone grants no correction authority.

Thus “several NTS servers plus one NTP server” is supported as different roles in one recovery flow. The NTP helper does not turn into an authenticated supporting source. If ordinary NTS acquisition or cached associations work, avoid unnecessary helper queries. If authenticated agreement fails, the helper cannot become a fallback clock controller.

### 4.3 What Chrony and NTPsec actually establish

| Project evidence | Relevant behavior | Consequence for NevroTick |
| --- | --- | --- |
| Chrony 4.9 manual and pinned configuration source | Normal correction slews. `makestep` enables threshold/count-controlled steps. `maxchange` separately limits accepted offsets; compiled defaults do not enable that limit | Keep step eligibility separate from maximum correction authority; do not assume a shipped Linux configuration is the library default |
| Chrony FAQ example | `makestep 1 3` is an example configuration | It is not a mandatory security threshold or a restart-safe Windows Service budget |
| NTPsec manual and pinned loop filter | Step threshold is 0.128 s; panic threshold is 1,000 s. `-g` permits an exceptional large initial adjustment; panic can be disabled explicitly | Twelve hours is much looser than the ordinary panic threshold; inspect the exact startup state machine before adapting its behavior |

These are distinct implementations/configurations, not universal NTP protocol requirements. Neither establishes a qualified “100 years with NTS, 12 hours with NTP” rule for NevroTick. [S3–S6]

### 4.4 Why the era and trust checks matter

NTP's wire seconds repeat after roughly 136 years. Usual relative interpretation has an approximately 68-year ambiguity boundary; authenticated packets do not add an era number. Recovery from a 100-year-wrong local clock is feasible only with additional date/era information and appropriate arithmetic. It is not categorically impossible. The existing Chrony build-controlled era mapping is a useful source to study. [S7, S8]

Define a **supported true-date interval** separately from the amount by which Windows can be wrong. An approved build/persisted authenticated anchor may help choose an era, but neither an old build date nor a lower bound alone proves current UTC. An ambiguous or out-of-policy candidate remains blocked.

NTS authenticates the peer and data under its trust basis; a faulty source, correlated sources, delayed packets or a manipulated certificate-time bootstrap can still produce bad evidence. Keep architecture §8's isolated provisional verification, protected observations, candidate-time revalidation and retained bootstrap provenance. Do not disable certificate verification or set the global clock from the plain hint to make TLS work. [S9]

### 4.5 Recommendations to investigate, not adopted constants

1. Start with **four independently operated NTS sources** as the normal preset target. For exceptional very large recovery, test a stronger requirement such as **three agreeing independent NTS identities**, while preserving the existing majority rule. This is a research candidate, not a new quorum default.
2. Prefer automatic exceptional recovery during a clearly defined acquisition/recovery window. Evaluate large backward and forward changes after normal tracking separately; obtain fresh evidence before either. A Windows service starting does not prove other applications are not running.
3. For plain NTP, compare a **1,000-second outer guard and smaller limits** against realistic drift/outage cases. Retain David's “12 hours or smaller” as the question to answer, not a value to hardcode. A small per-update bound alone does not prevent accumulated manipulation.
4. Keep normal corrections gradual and bounded. Require explicit policy or a current one-use approval outside the qualified automatic envelope. Service restarts, retries, reconnects and sign changes must not replenish consumed authority.
5. For helper policy, retain strict-default disabled pending resolution of OD-03. If enabled, prefer two helpers when operationally practical; allow one only as the visibly weaker policy already requested. Two-helper failure never silently becomes one-helper mode.

OPEN-5/6/7/8 remain open for the exact parameters and enforcement evidence. See research R-02/R-03/R-05.

## 5. OD-03 — the Pool preference and an NTS preset

### 5.1 Two constraints on the requested default

The public NTP Pool currently does not provide authentication/NTS. Its distribution policy also requires an assigned vendor zone for applications shipping Pool defaults, including open-source projects. Generic `pool.ntp.org` names must not simply be embedded as NevroTick's fleet default. No zone has been requested or allocated by this work. [S10, S11]

Record the preference as **NTP Pool is the desired provider for the product's plain-NTP use**. This cannot by itself populate an NTS-only preset. Do not claim that a made-up NevroTick Pool hostname exists. User-entered ordinary Pool configuration and a redistributed built-in default are different cases.

### 5.2 Recommended resolution

Engineering recommendation, pending an owner decision:

| Purpose | Proposed source policy |
| --- | --- |
| Normal default synchronization | Qualified independent NTS providers |
| Optional certificate-date recovery | One or two explicitly configured plain helpers; use the allocated Pool service where suitable |
| Explicit plain-NTP profile | The NTP Pool, through the assigned vendor names, with an unauthenticated status label and conservative policy |

This preserves the requested Pool integration and the earlier NTS-first objective. It is a **recommendation**, not an assertion that David has approved this exact role split. If David instead wants unauthenticated Pool synchronization as the primary shipped profile, record that as an explicit change to DEC-5 and its security claims. Until resolved, agents may build profile support and controlled fixtures, but may not publish a supposedly settled zero-configuration default.

### 5.3 How to use the supplied server directory

Use [jauderho/nts-servers](https://github.com/jauderho/nts-servers) to discover candidates, then check each operator's own documentation. Its listing is not provider permission, proof of current uptime, an independent-operator guarantee, or qualification of the Windows implementation. Importing its entire sample configuration would exceed our intended small preset.

Research should choose one logical source per selected operator, compatible leap/smear behavior, allowed fleet use, viable reachability and bounded polling. Several addresses belonging to one operator must not acquire multiple independent votes. The proposed shortlist and exact directory pin belong in the research brief; no runtime download of executable scripts or automatic ingestion of an unreviewed GitHub list is adopted.

## 6. OD-04 — decisions when evidence makes them concrete

Agents should proceed with ordinary implementation and bounded experiments under existing authority. Ask David when the next action actually depends on a material unresolved product or risk choice, after doing the authorized work needed to present a concrete recommendation.

Use the architecture OPEN register for issue ownership, this document for owner decisions, and the ASVS worksheet for affected clauses. Do not create a separate hidden decision list in each agent's notes.

### Decision card

```text
Decision ID / related OPEN, DEC, THR, CHECK or ASVS clause:
Current status: Researching | Ready for owner | Accepted | Rejected | Superseded
Specific next action that depends on this decision:
Current rule and exact proposed change:
Evidence: source revision, test environment, observations, limitations:
Options: usually 2–3, with concrete behavior, risk, resource/cost effects:
Recommended option and reason:
Residual risk and compensating controls:
Owner and date / exact scope of recorded approval:
Validation needed after the decision:
Revisit trigger and unaffected work that can continue:
```

Bring a card for a wider clock authority envelope, weaker trust/authentication/revocation rule, ASVS exception, incompatible licence, material security/performance tradeoff, paid commitment or new release channel. An external provider's eligibility decision is evidence to obtain, not a policy agents can approve on David's behalf.

David decides exceptions **individually**. This is not blanket acceptance of ASVS deviations, TLS soft failure, weaker source counts, or a compromised engine's residual clock authority. Keep those items open and stop only the dependent feature/claim. Once accepted, update the canonical rule, threat/risk record, relevant applicability rows and verification expectations together. Do not ask again without new evidence or a meaningful scope change.

### 6.1 S2.2 cookie reissuance

**Card:** S2.2-COOKIE-01; REQ-8/BND-2, AC-04, CHECK-6/8/9/17/21/25/26/27.
**Status:** Accepted by David, 11 November 2026 — option A. Fresh cookies on the
wire; explicit return only before actual adapter invocation, under the same live
association and original expiry. No post-invocation reuse is approved.
The dependent action is the association owner's explicit cookie-return/reuse
transition in the [existing S2.2 request contract](../tasks/engine-transport-pilot.md#next-source-contract-one-nts-association-and-one-outstanding-request).

No production-qualified association exists. EP3's default-off source now moves
one genuine KE result into a private original-context association. Its default
preview remains unsendable; explicit `--protected-datagram` selects one bounded
protected UDP source path. Real KE/TLS and protected-UDP execution and production
adoption remain unqualified; the linked Windows scripted component cohort has
passed separately. Its count summary is not an association or send receipt.
The former request draft recorded emission facts without granting reissuance.
The accepted transition below supersedes that pending policy status. The
decision establishes policy; the implemented EP2/EP3 transitions and their
source-specific evidence are recorded below, separately from native/product
qualification.
[RFC 8915 §5.7](https://www.rfc-editor.org/rfc/rfc8915.html#section-5.7)
prefers fresh cookies while allowing a user/application resilience choice;
[§9.1](https://www.rfc-editor.org/rfc/rfc8915.html#section-9.1)
explains the network-linkability concern. Neither establishes a local default.

| Option | Concrete behavior and cost |
| --- | --- |
| A — fresh on the wire; return only before invocation (**selected**) | Never reissue a cookie after its send adapter was invoked, including error, short, cancelled or ambiguous results. An explicit owner operation may return an unchanged cookie only after proving that adapter was never invoked, retiring its old UID and rechecking the same live association and original expiry. When no fresh cookie remains, use bounded KE recovery. KE outages may interrupt acquisition. |
| B — one resilience reuse | Additionally permit one reuse of a possibly sent cookie after KE is unavailable, only within the same still-current association/path/trust context and original expiry, with a fresh UID and a retained reuse count. This improves outage resilience but exposes a repeatable wire identifier. No network/path-change reuse is allowed. |
| C — retire every reserved cookie | Never return even a proven-uninvoked reservation. This simplifies disposition but can consume cookies during local failures without any privacy benefit from avoiding an actual transmission. |

Recommendation A avoids wire reuse while recovering resources on a genuinely
uninvoked path. A transport error, dropped future or numeric byte count is not
that proof. An unqualified native no-send result remains ambiguous. Cookie age,
association expiry and request credit never reset on return; drop alone does not
return anything. No option permits plain fallback, weaker authentication or
unbounded retries. Persistent secret storage remains under its separate hold.

**Owner/date:** David, 11 November 2026. Exact reply: “Fresh cookies; pre-send
return only (Recommended)”. This accepts option A only; B and C remain unselected.
The selected source transitions exist within EP2/EP3's original-context scope;
continuous recovery and native/product qualification remain open. Follow the
[current source exit and recovery receiving boundary](../tasks/engine-transport-pilot.md#recovery-receiving-boundary-reconciliation--11-november-2026).
Validate actual
pre-invocation return, stale/expired return refusal, fresh-UID issuance and every
post-invocation/ambiguous refusal with genuine owner and adapter controls.
Revisit only for contrary interoperability/privacy evidence or a changed scope.
Association transfer, one-slot reservation and emission accounting can proceed
independently once their own real producer/graph/lifetime prerequisites are met.

### 6.2 S2.2 zero replacement cookies

**Card:** S2.2-COOKIE-02; same S2.2 requirements and receiving owner as above.
**Status:** Accepted by David, 11 November 2026 — option A. Reject zero-cookie
responses without publishing an observation or new cookies, preserving a healthy
pending request under its original deadline and receive-work budget. No
interoperability tolerance is selected; this decision supplies no native or
production receiver qualification.
The dependent action is protected-response acceptance/completion when the
authenticated plaintext contains zero replacement-cookie fields.

[RFC 8915 §5.7](https://www.rfc-editor.org/rfc/rfc8915.html#section-5.7)
requires the server's protected response to carry at least one encrypted cookie;
its client checks bind UID and S2C authentication to the outstanding request.
It does not prescribe the client's zero-cookie acceptance/completion policy.
David selected the strict local disposition below. Structural acceptance of an
empty plaintext is not a decision to admit that response or its time sample.

| Option | Concrete behavior and cost |
| --- | --- |
| A — reject the nonconforming response (**selected**) | Publish no observation or replacement cookies and leave the original healthy pending request usable until its unchanged deadline, within its existing receive-work budget. A later conforming reply may complete it once. Expiry retires it normally; a fresh request needs another issuable cookie or bounded KE recovery. No immediate retry loop or renewed deadline follows. |
| B — tolerate authenticated zero-cookie responses | After all other current UID, AEAD, endpoint, NTP and admission checks pass, complete the request once and expose its otherwise admissible observation, recording zero new cookies. Next issuance still needs another fresh cookie or bounded KE recovery. This is a local interoperability tolerance for a server that violates its sending requirement; RFC 8915 does not select the client's disposition. |

Recommendation A keeps the first product profile strict and preserves the existing
rule that rejected packets cannot consume a healthy pending request. Its cost is
loss of observations from an otherwise authentic but nonconforming peer. Option B
does not authorize reuse, infer physical time quality or relax any other guard.
One cookie with a zero-length body is a separate representability question; it is
not the zero-field case addressed here.

**Owner/date:** David, 11 November 2026. Exact reply: “Reject; preserve pending
request (Recommended)”. This accepts option A only; B remains unselected.
EP2/EP3 implement this rejection in their bounded original-context source and
retain linked Linux and Windows scripted-component evidence. Real TLS/transport,
product qualification and continuous recovery remain open. Required controls are
zero versus one encrypted cookie, wrong UID/tag/endpoint, late and duplicate
responses, a valid neighboring reply after rejection, original deadline and
finite receive budgets, and no history/phase effects on refusal. No first-send,
provider/native or production-history hold is lifted by either choice.

### 6.3 Mandatory cookie-policy acceptance — 11 November 2026

David added the following requirements after accepting both A choices. These
clarify their implementation; they are not additional pending decisions.

- Charge each received packet's admitted processing work before the affected
  parsing/cryptographic stage. Rejection preserves the **remaining** budget after
  that charge, never the initial budget or an earlier snapshot. No refund or
  deadline renewal follows from rejection. Budget exhaustion uses the existing
  terminal path and cannot leave an indefinitely usable request.
- Cookie exhaustion uses finite, rate-limited KE recovery with backoff or another
  suitable configured trusted source. Bound the aggregate recovery work as well
  as each invocation. No tight reconnect loop, implicit cookie reuse, duplicate
  source vote, reduced agreement requirement or silent plain-NTP fallback follows.
- Published state distinguishes an unsuccessful current attempt from the last
  successful synchronization. Rejected replies cannot update last-success time,
  renew freshness or claim a fresh synchronization. A still-valid older result
  keeps its own unchanged age and expiry; failed attempts do not manufacture a
  newer result or erase the distinction.
- Required paired cases cover cancellation before and after actual send
  invocation, repeated authenticated zero-cookie replies through budget
  exhaustion, cookie-pool exhaustion/backoff, and return attempted after
  association replacement. Observe actual effects and genuine valid neighbors,
  not only returned error variants. Native and UI execution retain their own
  environments and holds; the state contract applies before those bindings.

## 7. OD-05 — signing and release ownership

### 7.1 Accepted choices

**3 November 2026 amendment:** David asked to maximize MSIX protection, then try
publishing the complete application in Microsoft Store with its services. He will
reconsider delivery after an actual rejection or a concrete technical blocker.
This supersedes the earlier MSI-first direction and the requirement to obtain
Store approval before selecting MSIX for preparation. It does not pre-approve an
unsupported service topology or unsafe lifecycle.

- **Selected preparation target:** one complete hardened MSIX containing the UI,
  network engine, clock broker, narrow lifecycle helper and required payloads;
  Microsoft Store is the intended first publication attempt. No split
  MSI-service/MSIX-UI product or external service prerequisite.
- **Security objective:** use the strongest compatible, supported and verified
  controls for each process, including package-content integrity and the
  Store-compatible code-integrity profile in the
  [MSIX plan](plans/windows-msix-store.md#3-package-trust-and-the-privileged-helper).
  A required protection failure must prevent the affected authority/operation;
  no silent disabling, broader service privilege or weakened recovery to make
  packaging or certification succeed.
- **6 November enforcement clarification:** every applicable protection is a
  required final-product obligation, not an optional optimization or an item an
  agent may indefinitely leave as a research candidate. Select supported build
  protections now and implement/qualify role-specific runtime and package
  protections at their existing gates. Missing implementation or native evidence
  means **open work**, not inapplicability or accepted risk. A non-adoption record
  must name the exact role/toolchain/OS/hardware limitation, evidence and owner;
  a weaker release profile or security tradeoff needs David's explicit decision.
  Hardware-conditional enforcement does not silently raise the v3 CPU floor.
  This clarifies the existing objective without enabling incompatible switches,
  adopting an unpinned/nightly compiler or lifting unrelated execution holds.
- **Store outcome:** prepare the candidate and accurate service/capability
  justification without waiting for speculative preapproval. David makes the
  account/submission/visibility and release actions. An actual refusal is a
  decision input, not permission to conceal services or silently ship another
  channel. Store acceptance and application security qualification are distinct.
- **Signing-service candidate:** SignPath Foundation, if the project qualifies,
  may supply required inner-file signatures for the selected MSIX or signatures
  for a later owner-selected complete MSI. Eligibility and actual format support
  remain open; the Store's outer signature does not supply every inner signature.
- **Publisher display:** the Foundation's publisher identity is acceptable to David. Exact issued identity and renewal policy still require verification.
- **Human release/signing approver:** **David Osipov**.
- **MSI delivery alternative:** reconsider only after David's later decision.
  The 8 November requirement in §7.7 supersedes the earlier unsigned-MSI/OpenPGP
  exception; preserve legitimate upstream signatures under the current policy.

The [existing W11-W10/W11-W05 task](../tasks/windows11-integration.md#48-owner-selected-hardened-msix--3-november-2026)
owns the preparation and later qualification. The current portable Core priority,
Windows execution stop, separate fuzz no-retry hold, storage pause and UI-M0
deferrals remain. This amendment authorizes source/design preparation; it claims
no working MSIX, native mitigation enforcement, signing or Store acceptance.

Under §7.7, missing signing capability required by the declared profile blocks the
affected release, not source/design preparation; §7.8 permits interim self-signed
MSI without public CA trust. No delivery decision authorizes automatic
publication, removes unrelated gates, approves a paid certificate or turns absent
evidence into a pass. SignPath's acceptance remains external. [S12]

### 7.2 Separate release profiles

The selected Store MSIX profile uses the actual delivered package/catalog,
approved package identity and installed component lineage, with the exact
service, mitigation and recovery evidence in
[architecture §19.4](windows-time-architecture.md#194-future-complete-msix-gate)
and [signing §6.1](windows-time-signing-guide.md#61-store-msix-trust-and-publication-boundaries).
Store signing is not an arbitrary publisher allowlist or helper authorization.
The current profile additionally requires the per-file signatures in §7.7.
The following table is **historical: the two MSI profiles recorded on 29 October,
with the unsigned-MSI/OpenPGP release permission superseded on 8 November 2026**.
Its key-management and compatibility discussion records the earlier proposal;
it is not an admissible release exception or an instruction to set up that route.

> | Requirement | Preferred Authenticode profile | Initial OpenPGP profile |
> | --- | --- | --- |
> | Installable product | One complete MSI | The same complete-product MSI contract |
> | Artifact authenticity | Qualified payload/package signatures and expected publisher | Detached OpenPGP signature over exact final MSI bytes; trusted full release-key fingerprint |
> | Release manifest | Final hashes, commit/build identity, source and SBOM lineage | Same information, with an OpenPGP-signed manifest binding version and exact artifacts |
> | Windows publisher trust | Verify actual Windows signature/chain/timestamp behavior | Explicitly absent for unsigned owned files; an OpenPGP check does not supply it |
> | Installation compatibility | Test actual protections and policies | Test actual protections and report blocking restrictions honestly |
> | Human release approval | David approves the identified candidate | David approves the identified candidate |
>
> A valid OpenPGP signature proves a relationship between bytes and a key; the expected key/fingerprint must also be authenticated. Select a release key, publication channel, protected signing process, backup, expiry/revocation and rotation policy before release. This document selects no existing personal key and contains no secret material. Agents may prepare candidates and evidence; they do not acquire signing authority by generating an artifact. [S13]
>
> Windows UAC and application control do not treat detached OpenPGP as Authenticode. Unknown unsigned code can be blocked under Smart App Control or deployment policy. Do not promise warning-free installation or universal Windows 11 compatibility, and do not make disabling protections or installing a development root the normal user procedure. [S14, S15]
>
> OpenPGP sidecars and the release manifest are verification material, not additional application packages. The fallback does not introduce an embedded updater or require adding an OpenPGP library to the runtime. Ordinary MSIX deployment has separate signing requirements; Microsoft's unsigned-MSIX facility is for testing, so it is not the broad-distribution fallback. [S16]

### 7.3 Required follow-through before using the fallback

**Historical follow-through, superseded 8 November 2026:** the requirements below
were attached to the former unsigned-MSI/OpenPGP permission. They preserve its
original limits but cannot authorize a release now. Current work follows §7.7
and R-06's native signing qualification, without substituting detached signatures.

> Keep CHECK-18's Authenticode result **not qualified**. Add separately identified OpenPGP-profile evidence: expected key, valid and modified MSI controls, wrong signer, mismatched manifest/version, exact payload inventory, and complete lifecycle observations under the declared Windows protection configurations. Do not mark skipped Windows-signature tests as passed.
>
> Review every place that currently assumes a signed helper/payload. Establish how an approved package reaches protected installed paths, how ordinary users are prevented from substituting the helper or DLLs, how callers/endpoints are authenticated, and how maintenance validates its inputs and artifact lineage. A hash stored beside an attacker-writable executable is insufficient. Do not replace a signature check with unconditional acceptance. R-06 must produce the branch-specific integrity/identity design before this release route is qualified.
>
> All licence/source obligations, runtime hardening, privileged-boundary checks and relevant feature gates remain. Later migration to Authenticode must preserve one-product upgrade identity and reject unsupported rollback or silent reversion to a weaker release policy.

### 7.4 Release operations and proposed stable support — 4 November 2026

The owner requested research and integration of the missing stable-release
support and bad-release recovery procedures. The resulting
[release policy](release-policy.md) is the operational owner for support records,
backports, promotion, containment, corrective packages and publishing-authority
recovery. It preserves OD-05's selected complete Store MSIX and David's account,
submission, visibility and exact-candidate authority.

**Proposed, not yet approved:** the latest explicitly designated, qualified,
non-withdrawn stable release is the normal maintenance target, with a recorded
transition for users still served its predecessor. No stable release, LTS term,
parallel backport promise, fixed response/remediation deadline or backup release
approver is established. Use the existing decision-card process before the first
stable commitment to settle the actual support matrix, capacity, communication
and rollout criteria. Silence or completion of this research is not approval.

The [existing W11 task](../tasks/windows11-integration.md#421-release-operations-integration--4-november-2026)
routes the remaining OPEN-11/14 operational decisions and rehearsals to
W11-W10, with W11-W05 for ownership and mandatory-state recovery. The procedure
does not close OPEN-3/12/14's provisioning/removal gaps. Account recovery,
distribution containment and installed-device recovery require distinct
evidence; an authentic malicious package from compromised publication authority
has no established independent on-device rejection guarantee. Preserve the
existing THR-15 risk decision and current execution dispositions.

### 7.5 Optional enhanced build provenance — 4 November 2026

**Accepted owner instruction:** record the research on proving a release's
connection to the repository, but do not make that additional assurance a
mandatory release blocker. The [focused provenance record](plans/release-provenance.md)
retains the evidence, limitations and possible later experiment.

GitHub/Sigstore artifact attestations, an independently reproduced Windows
build, and a public source-to-Store verification bundle are **optional**.
Their absence alone does not block a release, P6/P7 closure, the Store attempt
or the current Core work. Do not import them as mandatory work through a task
index, CHECK mapping or the release-policy qualification matrix. A later
decision to require them must explicitly amend this instruction.

Existing source/build/artifact custody, corresponding-source and licence
obligations, required SBOM/payload reconciliation, package trust and tamper
checks, installed-product identity, lifecycle and native qualification remain
under their existing owners. Reproducible generation of admitted public data
is likewise distinct from an independent bit-for-bit rebuild of the complete
application. An optional experiment's absence or lack of reproducibility is
not itself a release blocker; a discovered failure of an existing mandatory
integrity or security requirement still follows that requirement.

No complete reproducible release or source-to-delivered-byte proof is claimed.
Neither signing, attestation nor matching rebuilds prove that approved source,
dependencies or build infrastructure contain no malicious behavior.

### 7.6 Extensive, correct product and package metadata — 5 November 2026

David requires the maximum useful, correct metadata whether the selected
delivery is MSIX or a later MSI route. The
[metadata contract](plans/windows-package-metadata.md) maps executable resources,
installer/package and installed-app information, Store fields, localization,
versions and source-of-value rules. Populate applicable supported fields and
record justified absence; do not invent identities, support contacts or shipped
capabilities, or enable package behavior merely to fill optional fields.

W11-W10/P6 own the shared record, generated outputs and actual artifact/readback
checks under existing CHECK-18/20/23/26/27. The field/generation design is source
preparation, not implemented metadata or a qualified package. The selected
complete-MSIX direction, possible later MSI reconsideration, Core priority and
execution holds remain; enhanced public build provenance stays optional under
§7.5.

### 7.7 Sign every supported shipped artifact — 8 November 2026

**Accepted owner instruction:** sign the installer itself and all binaries
inside, then extend that requirement to every shipped file that supports an
appropriate signature, including nonbinary files. This is a final-product
obligation for the selected complete Store MSIX and any later selected MSI;
it is not a statement that the current repository already produces signed files.

- Maintain one complete shipping inventory. Every supported signable artifact
  must have its own verified format-appropriate signature, in addition to its
  enclosing package's authentication. Include every PE by actual format, nested
  and third-party binaries, and admitted script or other signable formats if
  shipped. The [signing guide's inventory contract](windows-time-signing-guide.md#41-required-signatures-for-the-shipped-inventory)
  owns the format matrix, verification receipt and order of operations.
- Preserve acceptable signatures from approved upstream publishers. Do not
  silently replace them, attribute upstream code to NevroTick or sign unknown
  content merely to satisfy coverage. Resolve any unsigned third-party component
  through the approved provenance/licence/provider process before it can ship.
- Remaining data must be authenticated by the signed complete package and bound
  to its approved inventory. Do not invent a new JSON/XML envelope, script host
  or verifier merely because an arbitrary-byte signing tool exists. Assess the
  real format and its admitted consumer; provider/key unavailability is open
  signing work, not a reason to call a supported format inapplicable.
- Store package/catalog trust and individual signatures require separate
  evidence. Qualify the actual delivered artifact and preservation of required
  inner signatures after Store processing. Missing signatures, unsupported
  signing capability required by the declared profile or incompatible transformation block release
  pending resolution; there is no catalog-only exemption for signable files.
- The former unsigned-MSI plus OpenPGP release permission in §§7.1–7.3 is
  superseded. Ordinary users must not perform detached-signature verification as
  a substitute for native installer/payload trust. Optional enhanced provenance
  remains optional under §7.5; it is not expanded by this instruction.

[Architecture §19.7](windows-time-architecture.md#197-signatures-for-the-complete-shipped-inventory)
owns technical acceptance; W11-W10/S6.4 and P6 reuse CHECK-18/23/26, with CHECK-25/27
only for reached consumers/native verification. Finish metadata before signing,
sign and verify from inner artifacts outward, and preserve approved final bytes.
Signature presence does not establish automatic runtime enforcement or benign
code; package integrity, process/loader admission and the padding/content-trust
rules remain independent obligations.

The Store-first direction, Foundation publisher-display acceptance, David's
human signing/release authority, Core priority and existing execution holds
remain. This records no provider selection, paid certificate, account action,
production signing, Store submission or completed native result.

### 7.8 SignPath and interim self-signed MSI — 8 November 2026

**Direct owner clarification:** David reports that he currently has no personal
publicly trusted code-signing certificate. For MSI, the available approaches he
intends to use are SignPath Foundation, subject to acceptance, or his own generated
certificate; he prefers a self-signed artifact to an unsigned one. Record this as
an accepted interim signing option, not proof of an existing key or provider access.

For a later selected MSI release, an owner-controlled self-signed certificate
may sign the complete installer and the required owned payloads. This permission
is not confined to laboratory builds. Lack of a publicly trusted certificate
alone does not block that explicitly declared interim profile. It does not waive
§7.7's full supported signature coverage, approved upstream signatures, protected
release approval or the other applicable product/channel gates.

Self-signed Authenticode supplies checkable integrity and signer continuity only
relative to an independently approved key. It supplies no public CA validation
of David's identity, automatic Windows publisher trust or promise of warning-free
installation. Keep signature/key verification, the expected untrusted-chain
result on an unchanged clean machine and actual installation/runtime behavior
separate. Do not turn an arbitrary trust failure into a successful verification.
Use [signing guide §7.1](windows-time-signing-guide.md#71-interim-self-signed-msi-profile)
for the bounded evidence and transition procedure.

SignPath remains the intended public-trust option if the provider admits this
project and artifact graph; its Foundation certificate identifies the Foundation,
not a newly acquired personal certificate. Preserve its provenance and upstream
restrictions. The self-signed option does not satisfy Store MSI/EXE CA-chain
requirements or automatically provide trusted sideloaded MSIX/CIG admission.
It permits no customer certificate-store installation, disabled OS protection,
manual detached-verification requirement or unconditional helper/loader acceptance.

Complete Store MSIX remains the selected delivery direction; this clarification
does not itself switch to MSI or authorize a release. David retains exact-candidate
approval. No paid certificate, provider application, key generation/export,
signing operation, upload or native execution is performed or authorized by this
documentation update. W11-W10/S6.4 and R-06 retain the concrete qualification work.

## 8. OD-06 — future research and decision queue

The [research handoff](nevrotick-research-handoff.md) is the bounded assignment for another AI. It separates protocol/platform findings from choices that require David and from provider decisions nobody on the project can grant.

Priority order:

1. Confirm selected-source licence obligations and an admissible crypto/native graph before port/dependency commitments.
2. Resolve the Pool/NTS default interaction and provider permission before shipping defaults.
3. Prove era-aware authenticated recovery and propose conservative plain-NTP limits before clock actuation.
4. Prepare the owner-selected complete hardened MSIX and Store submission case
   with services through OD-05 and the existing W11 task. Qualify supported
   provisioning, recovery, installed trust and per-process protections. Resolve
   production signing for every supported shipped format under OD-05 §7.7,
   including SignPath eligibility if pursued for inner-file signatures. Missing
   capability blocks release, not independent MSIX source preparation.
5. Continue the existing native account/IPC/clock/lifecycle/resource investigations at their phase gates. Bring concrete exceptions rather than a speculative list of approvals.

The proprietary-artwork/free-signing question is **unresolved**. This work neither concludes SignPath rejects the project nor changes the brand licence. Research and any later authorized provider clarification must answer it.

The [30 October contract assessment](supporting-reviews/architecture-contract-review.md)
adds specificity to that queue, without recording new owner approvals. OPEN-5
needs the measured legitimate-drift versus residual compromised-engine authority
tradeoff (100 ppm integrates to 8.64 seconds/day), durable settlement and boot
semantics. OPEN-5/6 retain production quorum/freshness and floor-supersession
authority; OPEN-8 retains association/full-authentication lifetime and trust-change
policy; OPEN-11/14 retain signing key policy, wrong-clock recovery and maintenance
qualification. Prepare a decision card when evidence makes the dependent choice
ready. No TTL, additional reset power, wider correction budget or weaker trust
rule is approved by this review; unaffected implementation continues.

## 9. Integration and evidence

The maintained architecture integrates these instructions into requirements, decisions, large-error policy, packaging/release branches, relevant checks and the OPEN register. The feature/signing/hardening guides and diagram companion are aligned. Start from `AGENTS.md`, the root README or `docs/README.md`; the semantic context registry also lists this record and the research assignment. Existing numbering, earlier review history, UI-M0, independent upstream-verification policy and subsequent tooling/dependency updates are preserved.

Checks for this amendment concern document consistency, preserved concurrent changes, source/diagram/link identity and independent review of the documentation candidate. No NTP/NTS server was load-tested, application compiled, Windows clock changed, installer run, certificate obtained, release key generated, or software signed as evidence for these decisions.

## 10. Primary references

References were inspected on 29 October 2026. Mutable provider policies must be rechecked before release. User instructions are the authority for owner choices; these sources support technical/legal facts and identify research leads.

- **S1:** [Chrony selection source, selected 4.9 pin](https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/sources.c).
- **S2:** [GNU FAQ: translating code](https://www.gnu.org/licenses/gpl-faq.en.html#TranslateCode); [GNU licence compatibility](https://www.gnu.org/licenses/license-list.html#GPLv3).
- **S3:** [Chrony 4.9 configuration manual](https://chrony-project.org/doc/4.9/chrony.conf.html); [pinned configuration defaults](https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/conf.c).
- **S4:** [Chrony FAQ](https://chrony-project.org/faq.html).
- **S5:** [NTPsec ntpd manual](https://docs.ntpsec.org/latest/ntpd.html).
- **S6:** [NTPsec loop filter at 61eaa902b90e374698c1b4e380b0f27ed6632c65](https://github.com/ntpsec/ntpsec/blob/61eaa902b90e374698c1b4e380b0f27ed6632c65/ntpd/ntp_loopfilter.c).
- **S7:** [RFC 5905, date formats and timestamp arithmetic](https://www.rfc-editor.org/rfc/rfc5905.html#section-6).
- **S8:** [Chrony selected-era configuration](https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/configure); [timestamp conversion](https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/util.c).
- **S9:** [RFC 8915, especially security considerations](https://www.rfc-editor.org/rfc/rfc8915.html#section-8).
- **S10:** [NTP Pool authentication](https://news.ntppool.org/docs/authentication/).
- **S11:** [NTP Pool vendor policy](https://www.ntppool.org/vendors.html).
- **S12:** [SignPath Foundation terms, currently labelled Draft](https://signpath.org/terms.html).
- **S13:** [GnuPG signature/fingerprint verification](https://gnupg.org/download/integrity_check.html).
- **S14:** [Microsoft UAC behavior](https://learn.microsoft.com/en-us/windows/security/application-security/application-control/user-account-control/how-it-works).
- **S15:** [Microsoft Smart App Control](https://learn.microsoft.com/en-us/windows/apps/develop/smart-app-control/overview).
- **S16:** [Microsoft unsigned MSIX testing feature](https://learn.microsoft.com/en-us/windows/msix/package/unsigned-package).
- **S17:** [RFC 8633, source count and diversity](https://www.rfc-editor.org/rfc/rfc8633.html#section-3.2).
- **S18:** [User-supplied NTS directory](https://github.com/jauderho/nts-servers); see the research brief for its inspected revision and operator evidence.

## 11. OD-07 — x64 baseline and runtime optimization

**Owner/date:** David Osipov, 30 October 2026, Asia/Tbilisi. David requested
compilation for v2 with dynamic higher-level optimizations, then instructed agents
to document the research and adjust the current documents and plan because this
optimization is required. During documentation, David revised the selected
baseline to v3. This records the final instruction without another approval round. It establishes a required workstream, not an observed performance result.

**Decision and alternatives:** Require `x86-64-v3` as the minimum for the
Windows x64 product and retain ahead-of-time, runtime-selected specialized functions.
**6 November implementation correction:** David explicitly instructed agents to
install the already selected v3 minimum and resolve contradictory build/security
policy. `.cargo/config.toml`, the exact checker and independent controls now
select `target-cpu=x86-64-v3`; generic x86-64 is no longer the product build floor.
Missing native qualification cannot justify reverting that decision or deferring
the configuration indefinitely. Globally compiling for v4 would exclude
the supported v3 baseline machines. Do not use `target-cpu=native` for distributed
artifacts. Additional v4 capabilities authorize only functions whose full enabled feature
set is available; CPU capability alone does not establish a speed benefit.

**Evidence and limits:** The [research report](supporting-reviews/x86-runtime-optimization.md)
records primary Rust and processor-level sources, the pinned compiler's feature
queries and limitations. There are no new native Windows compatibility or
performance results. The decision does not admit an unreviewed dependency,
unsafe numerical shortcut, weaker mitigation, new floating-point environment or
crypto provider. AES acceleration remains a separate per-feature provider choice.

**Implementation/validation:** The build owner maintains v3 and the complete
hardening vector as one exact configuration contract; each measured kernel owner and an
independent reviewer qualify the dispatch, fallback and mathematical contract.
The package owner proves that a baseline-compatible prerequisite check rejects
unsupported CPU/OS/guest combinations before any v3 payload is started. Preserve one complete
package and existing process/privilege boundaries. Qualification must cover actual
supported CPU/OS/VM rows, masked/disabled vector support, numerical and rejection
parity, latency/resources/artifact size and existing CHECK-19/20/21/25/26 gates.

Use [hardening §6.7](windows-time-development-hardening.md#67-v3-baseline-and-runtime-cpu-dispatch)
and the [execution workstream](plans/development-plan.md#81-x64-baseline-and-runtime-optimization-workstream).
Independent pure work, including the current S1.3 fixture, can continue. Revisit
only for incompatible supported-machine evidence or a material measured tradeoff;
do not silently replace this decision with a wider CPU requirement.

## 12. OD-08 — system-wide Windows DNS and DNSSEC

**Owner/date:** David Osipov, 2 November 2026, Asia/Tbilisi. David explicitly
clarified that the planned DNS feature is a DNS answering service for the whole
computer and its programs, rather than a resolver used only by NevroTick's own
components. In the preceding feature discussion he required DNSSEC with hardware
acceleration and multiple CPU cores. He requested a detailed English record of
the adaptive-cache design in the repository with a roadmap link.

**Accepted direction:** deliver future Windows-wide DNS service with the existing
DoQ-only upstream and leaf/intermediate certificate-pinning direction. Qualify
hardware-accelerated validation and bounded parallel work on the actual supported
CPU/OS/provider graph. This does not select a backend, worker count, numerical
speedup or a broader CPU minimum than OD-07.

**Scope and evidence:** all applications are intended consumers. Actual routing
through Windows, VPN/enterprise namespaces and independent application DNS paths
requires qualification; the owner clarification is not proof of universal
interception. Local UDP/TCP loopback ingress is a proposed integration hop and
does not authorize plaintext upstream fallback. Android-wide DNS integration
remains separate from the accepted Windows scope and needs its own evidence.

**Detailed proposal:** [System-wide Windows DNS and adaptive cache](plans/system-dns-cache-design.md)
owns the proposed demand aggregates, decay, hot/warm/cold/frozen classes,
optional seasonal heatmap, useful-refresh accounting, encrypted statistics,
privacy controls, resource limits and staged acceptance. The request to record
these ideas does not approve unmeasured thresholds, retention periods, storage
identity, cryptographic provider or default-on autonomous refresh. Use OD-04 for
material dependent choices when evidence makes them concrete.

**Delivery:** the [roadmap DNS workstream](../ROADMAP.md#5-planned-dns-over-quic-and-certificate-pinning)
links this plan. The current Windows time-client execution plan, UI-M0 limits,
existing privilege boundaries retain their authority. The post-V1 order starts with
MATH-0, then MT-0 and KHR-0 under [amended OD-16](#201-mathematics-experiments-first--13-november-2026). No DNS implementation, host configuration change, benchmark, native
Windows result or new release qualification is established by this amendment.

## 13. OD-09 — leap policy and silent servers

**Direct owner instruction, 4 November 2026:** after requesting verification of
the Habr leap-file article, David requested several research agents, specifically
one using GitHub MCP and one using Windows Learn MCP. The required outcome is a
clear, checked decision for NevroTick that works with servers which do not
announce leap seconds, followed by integration into all affected repository
documents. This delegates selection of a concrete design within the existing
trust/correction boundaries; it does not record a verbatim owner choice of an API.

**Selected engineering resolution:** [architecture §10.5](windows-time-architecture.md#105-leap-seconds-and-time-scales)
and [NV-LP-001](plans/leap-second-policy.md) require reviewed release-bundled
history plus separately effective UTC rules; ordinary admission of silent LI=0
sources with established unsmeared-UTC provenance; no dynamic LI-vote event
authority; Windows as sole explicit-event executor; bounded protection of
ambiguous intervals followed by fresh-quorum measured-residual recovery.
Future-calendar expiry retains history and permits ordinary event-free tracking
with a visible qualifier. The continuous-UTC rule effective 20 May 2027 comes
from the cited CGPM resolution, not from a guessed text-file expiry.

The release uses its existing authenticated complete-package channel for immutable
calendar/rule data. No new downloader, trust root, global Windows leap setting,
undocumented event setter, synthetic ±1-second operation or correction-budget
exception is selected. Default process compatibility presentation is the v1
qualification target; presentation is distinct from global event ownership.

**Evidence and limits:** the focused plan pins primary sources and GitHub
implementation comparisons. Microsoft Learn MCP was used for the OS research.
Source inspection establishes documented mechanisms; it does not establish their
behavior in NevroTick on a supported Windows build. Exact UTC during a leap needs
native event-knowledge/behavior evidence; otherwise the selected contract is
visible uncertainty and qualified later convergence. OPEN-7 remains open for
that evidence and finite bounds. The current Core priority and Windows/fuzz/storage
holds, UI-M0 deferrals, source minima and ordinary broker authority remain.

The [S1.4 task](../tasks/s1.4-leap-policy.md) records implementation owners,
independent cases, review and publication verification. Revisit the policy if
the authoritative UTC rule changes, a supported Windows contract contradicts
the selected mapping/ownership design, or a proposed stronger event-time promise
requires new mechanisms or authority. Routine work under this selected contract
needs no repeated design permission; material exceptions still follow OD-04.

## 14. OD-10 — mandatory mathematical proofs

**Current disposition:** the [6 November first-release deferral](#141-first-release-deferral--6-november-2026) supersedes the proof-first execution and acceptance requirements below. The 5 November instruction and receipts are retained as historical policy.

**Direct owner instruction, 5 November 2026:** David stated that formal proofs
are now mandatory, supplied the source-bound rounding/era proof handoff and
instructed the main development agent to synchronize the repository and continue.
This supersedes earlier guidance that treated whole-domain mathematical proofs
as optional assurance. It does not select a new compiler or verifier, or weaken
the admitted execution controls.

**Required outcome:** product mathematical algorithms need machine-checked
proofs tied to their actual executable source and declared admitted domains.
The contract includes required success, rejection/error classification, exact
units, machine representability and the declared rounding/error bounds; include
state preservation, termination and bounded work where applicable. Inventory
assumptions, omitted constructors/callees and source correspondence explicitly.
Behavioral examples, a standalone tool control, a proof of a separate model or
an always-error-compatible postcondition cannot close that requirement.

The existing S1.1 domain, S1.4 leap and S1.2/S1.3 Core owners retain their proof
obligations under CHECK-7/8/25. The
[S1.1 continuation](../tasks/s1.1-time-domain.md#mandatory-proof-continuation-and-remote-integration--5-november-2026)
owns the initial rounding/era campaign and outstanding inventory. A completed
bounded theorem closes only its declared scope; other mathematical obligations
remain open before their owners' acceptance. Runtime validation, independent
tests, coverage/mutation, security, native Windows and final-package evidence
remain separately required. No protected host execution is authorized here.

The choice and admission of additional proof tools remains evidence-led. This
requirement is distinct from optional enhanced release/build provenance in
OD-05 §7.5, and grants no publication or release approval.

**Conditional tool direction, 5 November 2026:** David asked to use Verus if it
is better and to document justified work in the plan, backlog, tasks and relevant
docs. The [bounded Core addition pilot](../tasks/s1.3-source-selection-readiness.md#verus-addition-pilot--5-november-2026)
records the current evidence-led evaluation, finite gates and acceptance.
This instruction preserves the source-bound proof requirement and conditional
adoption: a justified pilot is not demonstrated superiority, general verifier
selection or a weaker mathematical acceptance rule.

### 14.1 First-release deferral — 6 November 2026

**Explicit owner instruction:** David directed the agent to finalize, commit and
push the current work, defer formal proofs until after the first released
software version (V1), document the decision, adjust the existing tasks and
continue product development. This amends OD-10's 5 November proof-first policy;
formal proof completion is no longer a V1 prerequisite, acceptance blocker or
release gate. The whole-product objective continues.

**Owner-reported rationale:** this proof work took more than one day without a
new demonstrated product mathematical defect. This is the owner's account of
this work, not a measured assurance claim or a claim that the product has no
software defects. Deliberately injected semantic mutants and defects in proof
tooling or harnesses remain distinct from demonstrated product defects.

**Current execution:** defer formal compiler/solver/proof jobs, proof authoring,
verifier/tool extensions, admission/metadata work and the pending metadata-consumer
repair until after V1. Do not restart them automatically because an old task,
skill, verifier capability or missing theorem says proof comes first. Continue
the ordinary product plan and its applicable nonformal acceptance. Ordinary
product compilation and testing are unaffected by this formal-work deferral.

Retain existing proofs, failed/incomplete receipts and unproved obligations in
their current task/evidence chain. Completed proofs retain only their exact
historical source/domain/configuration scope; neither this decision nor changed
source extends them. Track unfinished and newly affected obligations as post-V1
work, not permanently waived work or completed mathematical assurance. After V1,
resume through normal bounded planning and reconcile the source-bound contracts,
controls and candidate identity before claiming new proof acceptance.

Runtime validation, garde-backed private construction and invariants, checked
arithmetic, independent behavioral/property/boundary tests, security checks,
90% security-primitive executable-line coverage, 95% per-critical-file mutation,
native Windows, packaging and release requirements remain unchanged. Existing
Windows execution, fuzz, protected-storage and UI-M0 holds/deferrals remain.
Amended OD-05's complete hardened MSIX direction and David's release authority
remain canonical; this amendment grants no native execution, publication-account
action or release approval.

### 14.2 Proof purpose and completion — 6 November 2026

**Explicit owner instruction:** after the scoped review of the existing formal
proofs, David asked to make the agent skills prevent low-value proof activity and
to document the product guarantees that still need coverage. This amendment
changes planning, review and accounting; §14.1's post-V1 execution deferral stays
in force. It neither discards useful existing proofs nor asserts that their
author fabricated results or that the product contains a demonstrated new bug.

**Required behavior:** choose a meaningful product-operation contract before a
verifier, harness or convenient helper. Record its required success/errors,
admitted domain, actual constructor/callee/caller chain, independent expected
relation, plausible semantic defects and finite completion/stop budget. A local
lemma is useful when it closes a named requirement or a necessary dependency of
that contract. Its pass never closes omitted callers, validation, state effects
or a parent algorithm. A sound proof need not discover a previously unknown bug;
its value is the precise guarantee established and its demonstrated sensitivity
to relevant incorrect behavior.

When a pilot exhausts its budget or cannot establish the selected property,
record the incomplete obligation and blocker. Do not continue by silently
changing the target to easy helpers, restarting clocks, repeating an unchanged
failed approach or expanding proof infrastructure. Any later attempt must name
the diagnosed cause, material change, expected path to closure and bounded cost
in the same task. Infrastructure work must unblock that selected guarantee; it
is not product-proof progress by itself. Reconcile reports by distinct completed
product guarantees, remaining assumptions and unproved scope, separately from
attempted/qualified jobs, controls and tooling checks.

The [existing post-V1 backlog](../tasks/development-backlog.md#product-guarantees-still-open)
owns the consolidated debt and closure criteria; existing S1.1/S1.2/S1.3/S1.4
tasks retain detailed source/evidence ownership. The
[formal-proof workflow](../.agents/skills/testing/references/formal-verification.md)
owns operational guidance for planning, independent semantic review, real
verifier-output compatibility and reusable future invocation. Updating these
instructions and recording debt is authorized now; formal proof authoring,
execution and deferred consumer/toolchain repairs remain post-V1.

## 15. OD-11 — SOCKS5 scope and deferral

**Direct owner instruction, 5 November 2026:** David asked for a concrete working
proxy profile and its checks, proposed supporting only SOCKS5, and authorized
moving the functionality to `ROADMAP.md` if it would create disproportionate
implementation and maintenance work. He did not authorize a weaker routing,
authentication or storage guarantee.

**Assessment and disposition:** implementing only a TCP SOCKS handshake would
not support NTP/NTS. A usable profile also needs the UDP relay/control lifecycle,
cross-stage endpoint identity, strict route preservation, service-context recovery
and defensible timing evidence. The source-based assessment in the
[existing engine task](../tasks/engine-transport-pilot.md#socks5-feasibility-and-deferral--5-november-2026)
therefore selects deferral under the owner's conditional instruction. This is an
engineering judgement about scope, not a measured schedule estimate or a claim
that SOCKS5 is infeasible. Implementing it during current P2/P5 was considered;
its additional qualification and support duties are not justified for first
delivery. Deferral retains a concrete next stage instead of promising partial
TCP-only support.

**Scope now:** first Windows delivery uses Direct. Current direct-family/interface
policy, upstream-failure recovery and successful-path reuse stay with their
existing P2/P5 owners. Unsupported proxy configuration, including an imported
newer profile, must be rejected explicitly; it cannot be silently erased or
interpreted as Direct. Existing shared policy cutover, storage and security
requirements remain. This does not reorder the ordinary product plan or future KHR/Android and
RGT/CHR sequences; formal work follows the [first-release deferral](#141-first-release-deferral--6-november-2026).

**Proposed first future subset:** manual numeric loopback SOCKS5, method 0x00
only, explicitly local DNS, admitted numeric targets, NTS TCP CONNECT and a
separate monitored UDP ASSOCIATE owner. The
[architecture](windows-time-architecture.md#limited-socks5-profile--deferred)
owns exact semantics; [SOCKS-0](../ROADMAP.md#socks-0--optional-socks5-time-acquisition)
owns timing and SOCKS-C01. No password input or new secret purpose is needed for
this subset. Remote proxies, authentication and proxy DNS are later SOCKS5
extensions with their own qualification; other proxy protocol families and
automatic browser/system import are outside the selected direction.

**Revisit:** after first Windows delivery, when a real use case and maintenance
capacity justify the bounded stage. Keep independent review and exact-candidate
portable/native/interoperability evidence open until executed. No implementation,
dependency, protected-storage experiment, host reconfiguration or production
proxy support is approved or established by this documentation decision.

## 16. OD-12 — GeoIP experiment after Khronos

**Direct owner instruction, 5 November 2026:** David asked to place the discussed
experiment in `ROADMAP.md` after Khronos. The discussion proposed locating time
sources and optionally the client's public egress to test geographically favorable
server selection, with GeoLite2-City as a possible input.

**Roadmap integration:** [GEO-0](../ROADMAP.md#geo-0--evaluate-geography-assisted-source-ordering)
sits between KHR-C01 and Android. Its GEO-C01 comparison/disposition belongs to the
[existing catalogue task](../tasks/server-catalogue-and-statistics.md#deferred-geoip-experiment--after-khronos),
reusing catalogue and advisory-history owners. `ADOPT_LIMITED`, `DEFER` or `REJECT`
all close the experiment; production GeoIP implementation is separately scoped
and is not an Android prerequisite. An adopted Khronos implementation retains its
existing qualification requirement before Android. Current Windows P0–P7 and the
separate RGT-0 → CHR-0 order remain intact.

**Engineering experiment boundary:** geography may order ordinary acquisition
candidates but cannot grant trust, replace actual measurements, weaken independent
agreement or change a qualified Khronos/Ananke sampler. The roadmap defines
comparison controls, missing/incorrect-location cases and cost evidence. No
GeoIP database, mandatory client-IP lookup, additional persistent store or public
probe is selected or authorized by this planning instruction. Data/licence/update
review, exact experiment admission and independent review remain future work;
the instruction establishes no measured performance benefit.


## 17. OD-13 — Slint as the default UI toolkit

**Accepted owner decision, 5 November 2026, Asia/Tbilisi.** After the
[live-Windows Slint experiment](../tasks/s0.5-ui-comparison.md#slint-software-renderer-live-host-result--5-november-2026),
David instructed: “We'll use it as default! Put this decision in all docs.”
The unpublished local draft called this OD-11; integration with published
SOCKS5 OD-11 and GeoIP OD-12 renumbers the Slint decision to OD-13 without
changing its accepted scope.
Slint is the default for subsequent NevroTick UI work. This supersedes DEC-3's
Win32-common-controls toolkit selection and the 30 October instruction to retain
Win32 as the default. It does not erase the earlier candidates or their evidence.

**Reaffirmed after the iced comparison, 5 November 2026.** After the reported
iced experiment, upstream source review and development/community assessment,
David explicitly decided to continue with Slint. The consolidated
[Slint/iced assessment](supporting-reviews/slint-iced-assessment-2026-11-05.md)
retains the evidence, competing strengths and unresolved source concerns.
The reported measurements show no clear runtime-resource winner; this is a
reaffirmation of OD-13, not a new performance claim or qualification waiver.

| Decision card | Accepted direction and remaining qualification |
| --- | --- |
| Related owners | DEC-3, OPEN-10, S0.5, UI-M0, CHECK-21/22; OPEN-1/13 and CHECK-20/26 own dependency/licence admission |
| Starting configuration | Slint 1.18.1 with explicit winit/software rendering and Fluent-style widgets, as measured. Freeze the exact admitted version/features for the migration; a later renderer/feature change needs its own graph and affected evidence |
| Evidence and reason | Three live-host Slint trials measured 7.730 MiB visible private memory and 23.629 MiB working set, with no detected CPU-time increment in the selected visible/minimized idle phases. The 228-line UI adapter suggests less UI coding than the 1,432-line Win32 implementation. Different layouts/DPI, short sampling and incomplete interaction/accessibility limit those conclusions |
| Options and outcome | Retain Win32, select Slint, or continue other toolkit experiments. David selected Slint; agents must not reopen the general toolkit choice because qualification remains pending |
| Existing implementation | The earlier main snapshot retained Win32; this merged candidate contains the Slint fixture host, with the Win32 adapter unexported as historical source. Preserve checked-domain behavior and candidate-specific receipts. Implemented fixture source and portable results do not establish native cutover or live backend qualification |
| Product boundaries | Ordinary-user UI only. Preserve the engine/broker/helper authority split, fixture-only UI-M0 effects, D1/D2/D3 deferrals and their re-entry triggers, current Core priority and existing execution holds |
| Required follow-through | Reuse the existing UI-M0/S0.5 task chain for exact dependency/licence admission, generated-code/lint and lifetime ownership, checked validation reuse, native window/editor/lifecycle observations, actual accessibility at its gate, resources and complete MSIX delivery |
| Revisit trigger | Concrete licensing, target, accessibility, security or resource evidence prevents the selected delivery. Bring that bounded issue under OD-04; no automatic fallback or blanket exception follows |

**Selection is separate from admission.** The private pilot disabled accessibility
and tray features; that does not remove the product's keyboard/UIA/Narrator or
optional-tray requirements at their existing gates. Slint draws its Fluent-style
widgets rather than inheriting Win32 child-control semantics. Qualify actual
behavior and accessibility exposure on Windows; old Win32 passes cannot transfer.
The software renderer is the starting configuration, not a mandate to introduce
continuous redraw, a browser runtime or a GPU loop.

The original toolkit decision did not settle the GPLv3/Apache-2.0-only
composition issue. The accepted GUI-specific GPL-3.0-only amendment below now
resolves the original-code direction, preserving GPL-2.0-only Core/engine. Exact
source rights, dependency grants, supplemental notices and final distribution
remain separately reviewed; a process split alone supplies no licence clearance.
No paid agreement, Core relicensing or release approval is implied.

The private `slint!` prototype also exposed a generated-unsafe/lint boundary:
Slint-generated allowances conflict with crate-level `forbid(unsafe_code)`.
The product migration must define and review that boundary explicitly rather
than silently relaxing the workspace's unsafe policy. Retain bounded input
admission, weak callback ownership, UI-thread affinity and deterministic teardown.

The [architecture §16.1](windows-time-architecture.md#161-toolkit-and-interaction-model)
controls the toolkit contract; [S0.5](../tasks/s0.5-ui-comparison.md) and the
[GUI handoff](../tasks/slint-gui-handoff.md) own current continuation. UI-M0
retains its original fixture acceptance. This decision records direction, not completion
of S0.5, CHECK-21/22, P0/P5 or release qualification.

### Complete migration amendment — 6 November 2026

The earlier unpublished GUI decision was called OD-11; its accepted scope is
now retained under OD-13 without replacing published SOCKS5 OD-11.


**Direct owner instruction, 5 November 2026:** David selected Slint as the
complete replacement for the Win32 UI and explicitly named the
[S0.5 task](../tasks/s0.5-ui-comparison.md) and this decision record for the change.
He considers Slint suitable for the interface he needs. This is an accepted
product direction, not a request to choose between toolkits again.

**Selected outcome:** implement the Rust UI with Slint against the
[current v0.4 reference](nevrotick-design-v0.4/README.md) and canonical feature,
state and security contracts. Slint replaces Win32 common controls as the UI
toolkit; a second product UI or automatic Win32 fallback is not selected.
Windows platform APIs remain available through narrow reviewed adapters for
required OS integration. DEC-3 and architecture §16 are amended accordingly.

The existing Win32 source, native observations and closed Win32/egui comparison
remain historical baseline evidence. The earlier instruction to retain Win32
and UI-M0's prohibition on a replacement framework are superseded for this
migration. Those results prove nothing about Slint's memory, accessibility,
renderer, mitigation compatibility or native lifecycle. The current source candidate contains the Slint fixture implementation; actual
native qualification and production cutover remain open.

**Execution and ownership:** continue the existing S0.5 record as the bounded
Slint migration chain. Source and licence review, presentation/fixture contracts
and admitted fixture-only implementation can proceed in parallel with the
current ordinary Core/acquisition work; UI work does not require completion of
unrelated Core tasks. One owner serializes Cargo graph/lockfile changes and
shared presentation schemas; workers may implement disjoint views after that
boundary is frozen. Native desktop interaction and shared outputs remain
serialized. No further confirmation of the toolkit choice or repeat of the
closed egui comparison is required.

Before admitting Slint dependencies, pin the actual version, Windows
backend/renderer/features and build/runtime/native graph, and establish a
compatible licence basis under OPEN-1/CHECK-20/26. This instruction does not
relicense Chrony-derived code, grant commercial rights or accept licence terms.
A separate UI process is not automatic legal clearance. Bring any concrete
incompatible-rights conflict or required resource/security-policy exception to
David under OD-04; routine work within the selected direction continues.

The first implementation remains unelevated and fixture-only, with visible
sample-data context, transient checked editing and unavailable protected actions.
The richer HTML service, ownership, recovery and notification simulations are
design references, not authority to add live network/IPC, persistence, services,
elevation or clock effects. D1/D2/D3 keep their existing re-entry triggers;
production S5.1–S5.4 and actual Windows qualification remain separately gated.
Retain the proposed resource targets, native execution restrictions, separate
AES fuzz hold and storage pause. Adoption needs Slint-specific evidence; the
toolkit decision itself is not a passing build, release or Windows result.

### GUI licence/composition decision — 6 November 2026

**Accepted owner decision, 6 November 2026:** David approved licensing the
original GUI under **GPL-3.0-only**, preserving Chrony-derived Core/engine under
**GPL-2.0-only**. Do not ask for this choice again. Apply the GUI-specific grant
and provenance inventory before admitting the actual Slint runtime/build graph;
the licence direction does not replace package/feature/security review.

**Conflict that led to this decision:** the workspace declared GPL-2.0-only and
the UI inherited it. The inspected [Slint 1.18.1 grants](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/LICENSE.md)
offer GPL-3.0-only or separately conditioned royalty-free/commercial grants;
none inspected establishes a GPL-2.0-only-compatible combined distribution.
The proposed Windows Winit backend also selects [Apache-2.0-only Winit](https://github.com/rust-windowing/winit/blob/e9809ef54b18499bb4f2cac945719ecc2a61061b/Cargo.toml#L370-L374),
so a Slint-only exception would not clear all affected inputs. These are exact
source/grant findings, not an admitted NevroTick Slint dependency graph.

| Concrete direction | Required consequence and remaining evidence |
| --- | --- |
| **Selected:** separately license the original GUI as GPL-3.0-only, preserving Chrony-derived engine/Core as GPL-2.0-only | Confirm rights over the GUI/presenter/model and any shared contract code; use a package-specific UI licence rather than changing the whole workspace. Keep the GUI an independently composed program using the defined process protocol, without linking GPL-2.0-only Core/Chrony code. Review actual shared code, runtime/build/transitive graph and notices; process separation alone is insufficient evidence. |
| Retained alternative, not selected: GPL-2.0-only for the linked GUI | Would require compatible permissions covering Slint and required transitives, or a concrete compatible graph. The public royalty-free/commercial descriptions and a purchase alone supply no such evidence. No supplier contact or purchase is authorized here. |

**Implementation status:** S0.5 records the GUI-specific manifest/licence notices,
reviewed source inventory and admitted fixture runtime/build graph. Re-admit
affected graph changes under the selected GPLv3 direction before execution. Keep Core and its GPLv2 inputs independent; formal work remains deferred under
OD-10 §14.1. No framework reselection, Core relicensing,
blanket dependency-policy relaxation, automatic Win32 fallback or new release
permission follows. S0.5 retains candidate-specific graph/rights and remaining distribution checks.


## 18. OD-14 — compact Windows platform scope

**Direct owner direction, 5 November 2026:** David limited the initial intended
OS scope to **Windows 11 26H2 and a corresponding Windows Server**, delegating
the other parameters. This supersedes the generic Windows-11-only direction for
platform preparation; it establishes no supported stable release or native pass.

**Researched mapping:** Microsoft's dated release records identify Windows 11
26H2 as GAC/build family **26300** and Windows Server **2025** as current
LTSC/build family **26100**. The engineering counterpart is Server 2025, not a
Server release called “26H2” or a promise of identical builds/features/servicing.
[Client release information](https://learn.microsoft.com/en-us/windows/release-health/windows11-release-information)
and [Server release information](https://learn.microsoft.com/en-us/windows/release-health/windows-server-release-info)
were checked on 5 November 2026.

**Engineering fixtures:** start with Windows 11 26H2 **Pro x64** and Windows
Server 2025 **Standard x64 with Desktop Experience**, using the MSVC target and
OD-07's required v3 baseline. These edition/install choices keep the complete
desktop UI/services product concrete; they are not David's separate edition
instructions or all-edition support promises. Exact UBR/KB, hardware, effective
protections and availability remain inputs. Historical generic binaries do not
qualify the current v3 configuration. Server Core and further editions are outside these first fixtures.

The [existing W11 task §4.32](../tasks/windows11-integration.md#432-two-family-windows-qualification-matrix--5-november-2026)
owns two rows and shared controls under existing CHECK/P0/P4/P6/P7 owners.
OPEN-2 remains open. Qualify package applicability, product OS-family admission,
safe CPU preflight, Server acquisition/update applicability and actual delivered
bytes separately; one complete MSIX and David's Store attempt retain OD-05's
owners. Managed-domain observation and administrator-authorized takeover retain
architecture §1.2; DC/PDC time service remains outside the product.

First-stable terms remain proposed under the [release policy](release-policy.md).
Current Core/proof priority, native/S134 stop, separate fuzz hold, optional-storage
pause, scoped Sandbox permission, UI-M0 D1–D3 deferrals, the Direct network profile,
Slint and optional enhanced release provenance retain their existing scope.


## 19. OD-15 — externalized UI translations and binary catalogs

This GUI decision was called OD-12 in its unpublished branch. Integration
assigns OD-15 so published GeoIP OD-12 retains its existing identity and scope.


**Direct owner instruction, 5 November 2026:** David requested an easy translation
mechanism using separate files containing translated interface strings, naming
JSON/XML or other formats and expressing a preference for binary formats such
as CBOR. This selects the mechanism requirement and binary-delivery preference;
it does not require translators to edit binary bytes.

The [localization design](plans/ui-localization.md) defines the engineering route:
editable UTF-8 JSON sources, build-time validation and deterministic CBOR language
resources, with a typed application-owned Rust catalog projecting text into
stable Slint properties/models and the tray/notification/accessibility adapters.
Adding a translation changes catalog data rather than UI layout or business
logic. Source editing, binary generation and package delivery are separate steps;
CBOR is a serialization format, not compression or an automatic performance gain.

Slint's built-in PO bundling is a documented alternative, but is not a CBOR
loader. The inspected Windows Rust path does not establish ordinary gettext/MO
loading or a stable custom CBOR-to-`@tr` hook. Do not use private Slint internals
or claim such support to implement this direction. S0.5 now implements the
fixture codec, typed interfaces, finite bounds and admitted graph. Independent
critical-meaning approval, new locale adoption and native/package qualification
remain open in the existing S0.5/P5 chain.

Catalogs cover visible labels, errors, explanations/tooltips, dialogs, tray menu,
local notifications and accessible names under one language/fallback policy.
Keep stable error/action IDs and typed arguments; translation changes no wire,
policy, source identity, numeric-unit/sign or authorization semantics. Preserve
complete built-in English, check placeholder/plural contracts and display text
as inert content. A locale change is a per-user presentation operation, never a
service/clock or Windows-wide locale change.

English remains the only adopted product language. This request adopts no
complete Russian translation, automatic OS-language selection or arbitrary
third-party pack import. The initial design reads reviewed packaged resources;
independent post-install packs need their own explicit loading/trust contract.
The original design addition changed no implementation. The merged Slint fixture
now implements the catalog pipeline; S0.5 owns its exact evidence and remaining
semantic/native qualification.
Source preparation may parallel Core; native integration, locale qualification,
D1–D3 re-entry, package integrity, licensing and execution holds retain their
existing owners and gates.

### Security clarification — 6 November 2026

David explicitly declared all JSON and CBOR potentially unsafe because they
contain contributor/translator-controlled data, and requested malicious-payload
defenses. Treat every such input as hostile at each raw-byte admission boundary:
English source, other locales, schemas/metadata, generated output, previously
accepted or signed/package-delivered files are not exempt. Review/signing establish
different provenance/approval facts; they never replace parser/resource/sink checks.
Immutable privately constructed validated values may still be reused within
their unchanged context under the existing enforcement contract.

The [localization plan](plans/ui-localization.md) now specifies layered build,
decode, format, rendering and semantic controls. Message IDs/types/plural policy,
required facts, action/link/activation IDs and criticality come from a separately
controlled typed contract, never from catalog declarations. Translation bytes
cannot change those rules or the checker that admits them. Build conversion is
isolated from credentials/network/release authority; runtime independently admits
final bytes. Reject malformed/overbudget candidates atomically and retain the
last admitted state; invalid English data cannot become its own fallback.

Syntactically valid wording can still deceive. Critical warning/action/accessibility
groups need independently controlled per-locale semantic approval bound to exact
content and contract revision. Missing/unreviewed critical groups fall back as a
whole to admitted, approved English. No catalog field, signature, LLM verdict or
schema pass proves the truth of a translation. These requirements refine OD-15
and the existing security boundaries; no extra locale, import mode, dependency,
validator/runtime implementation, fuzz execution or native/release pass is granted.

David additionally requested strict UTF-8 for original JSON and a strict schema.
The engineering profile is UTF-8 without BOM, no encoding detection/transcoding
or lossy repair, and strict JSON escape decoding into Unicode scalar values.
Valid escaped surrogate pairs remain supported; lone surrogates do not. The
independently owned versioned schema is closed at every object layer, with exact
required/allowed fields, types and finite bounds, plus decoded-key duplicate and
typed semantic checks. No catalog selects or fetches its own schema. These rules
apply before CBOR generation and equally to English. The merged fixture now
enforces the complete closed schema and validator; current source/test evidence
and remaining critical-meaning/native gates belong to S0.5.

### Character repertoire clarification — 6 November 2026

David requires all emoji and unusual symbols in translation input to reject.
Implement that requirement as a code-owned closed repertoire for each adopted
locale/field, with hard emoji denial and exact versioned Unicode/UCD/emoji data.
Literal and escaped spellings are equivalent: inspect every decoded key/string
in original JSON before generation and independently in runtime CBOR admission.
No file header, translator exception or text-presentation selector waives it.

The [localization character profile](plans/ui-localization.md#emoji-free-closed-character-repertoires)
rejects emoji/pictographic forms, their flag/modifier/keycap/variation/tag variants,
private-use/unassigned/noncharacters, U+FFFD/U+FEFF, authored bidi controls and
unapproved controls/format/invisible/symbol characters. Plain ASCII digits,
`#` and `*` remain ordinary text; a blanket Unicode `Emoji` property ban would
incorrectly reject them. Ordinary letters, marks, punctuation, spaces and units
need explicit code-owned repertoire membership. Orthographic joining controls
require a reviewed locale/field/context rule and cannot admit emoji.

This supersedes any earlier example treating literal U+FFFD or an arbitrary
valid surrogate pair as fully admitted catalog content. Unicode decoding can
succeed while character admission rejects. English remains the only adopted
locale; all files reject atomically on a character violation, retaining admitted
state. The merged fixture implements pinned inventories, tables, budgets and
receiver enforcement; S0.5 binds its portable results. This clarification adopts
no extra locale or native result, and never supplies critical-text approval.

## 20. OD-16 — multithreading first after V1

**Direct owner instruction, 7 November 2026:** create the multithreading plan
under docs and place this work first in ROADMAP.md immediately after release of
the first software version (V1), before Khronos.

**Original accepted order, 7 November (amended below):** actual V1 release →
MT-0 / MT-C01 → KHR-0 / KHR-C01 → GEO-0 / GEO-C01 → Android. This superseded
older descriptions of Khronos as the first Windows follow-up. The
[13 November amendment](#201-mathematics-experiments-first--13-november-2026)
now inserts MATH-0 / MATH-C01 before MT-0. OD-12's relative Khronos → GeoIP → Android order and the
separate later RGT-0 → CHR-0 direction remain.

[NV-MT-001](plans/post-v1-multithreading.md) records the phased plan, resource/
ownership contracts, independent healthy/fault observations and MT-C01 exit.
The [roadmap entry](../ROADMAP.md#mt-0--post-v1-multithreading-and-concurrent-acquisition)
sets priority; the existing engine task owns implementation and evidence.

This schedules the enhancement; it does not select four workers, a multithread
Tokio runtime, a provider or new production constants. Re-inspect the actual
released implementation and qualify justified changes. An incomplete or blocked
attempt is not completion and cannot silently advance the queue.

Current V1 correctness, blocking-native-work isolation, permit/cookie custody,
policy checks, responsive control and release gates remain required. MT-0 adds no
pre-V1 enhancement prerequisite and lifts no Windows/fuzz/storage/UI hold.
OD-10 still defers formal work until after V1; that eligibility does not put the
whole formal backlog ahead of the explicitly scheduled post-V1 sequence. This decision
establishes no measured improvement or implementation/native qualification.

### 20.1 Mathematics experiments first — 13 November 2026

**Direct owner instruction, 13 November 2026 (Asia/Tbilisi):** put the reviewed
scientific papers and mathematical improvement experiments in ROADMAP.md as the
first TODO experiment after V1 is released.

**Current accepted order:** actual V1 release → MATH-0 / MATH-C01 →
MT-0 / MT-C01 → KHR-0 / KHR-C01 → GEO-0 / GEO-C01 → Android. This supersedes
MT-0's first position while preserving the relative order of the existing
follow-ups, OD-12, the pre-V1 WinGet requirement and the separate later
RGT-0 → CHR-0 sequence. Publishing source changes alone is not V1 release.

[MATH-0](../ROADMAP.md#math-0--post-v1-mathematical-estimation-experiments)
records the bounded comparison, primary references and MATH-C01 exit. Evaluate
lower-envelope frequency estimation, compensated/QR arithmetic, asymmetry and
uncertainty controls, Kalman/noise/holdover models and conditional native
clock/temperature extensions against equivalent inputs and independent truth.
The existing Core task owns the comparison evidence; native and GEO-0 extensions
retain their stated prerequisites and owners.

This is an experiment-priority decision. No paper's algorithm, numerical bound,
provider, dependency or production performance claim is selected by recording it.
An evidence-backed comparison and candidate dispositions admit MT-0; production
adoption is not required. Preserve current numerical/clock-authority contracts,
source provenance and the released baseline for later comparisons. Any proposed
adoption must define its bounded implementation and complete the applicable
qualification before affecting product decisions.

Current V1 implementation, correctness, acceptance and release work retain their
order and gates. This adds no V1 blocker, starts no experiment before release and
lifts no existing execution hold. OD-10's post-V1 eligibility and bounded
product-purpose rules remain; the complete proof backlog is not inserted ahead
of MATH-0.

## 21. OD-17 — WinGet capability before V1

**Owner instruction, 7 November 2026:** David requested WinGet distribution
capability before the first software release, and comprehensive verification of
the supplied AI proposal before creating the document and adjusting the roadmap.
This places the capability in existing P6/P7 work. The later
[13 November amendment](#201-mathematics-experiments-first--13-november-2026)
places MATH-0 before MT-0; WinGet remains a pre-V1 capability requirement.

The [WinGet plan](plans/winget-distribution.md) records the researched engineering
approach, primary sources, metadata/installer/automation contracts and native
acceptance cases. [WGT-0](../ROADMAP.md#wgt-0--winget-distribution-capability-before-v1)
and [W11-W10 with W11-W05](../tasks/windows11-integration.md#433-winget-distribution-before-v1--7-november-2026)
own scheduling and evidence.

Preserve OD-05's one complete hardened MSIX and actual Store attempt. Qualify
WinGet acquisition of the real Store product through `msstore` first. The
Community `winget` repository is a different source; preparing or activating
that path requires a compatible authorized public trusted artifact. This
instruction selects neither MSI, a new signing provider, a split installer nor
a service self-updater. Existing platform/CPU, privilege, lifecycle, recovery
and release gates remain.

**WGT-C01** records implemented and appropriately qualified pre-V1 preparation
and delivery capability. **WGT-C02** records actual public source availability
after external acceptance/propagation. Neither a document, generated YAML, PR
nor merge proves a functioning public installation. No fixed review deadline,
universal automatic customer update or public availability before a legitimate
release artifact exists is promised. Missing mandatory native evidence remains
open; concrete external blockers use the existing owner-decision process.

David retains actual account, credential, signing, submission, visibility and
release actions under OD-05. This research/documentation instruction creates
no external PR or release and lifts no current Core/native/fuzz/storage/UI hold.

## 22. OD-18 — literature-derived qualification before V1

**Direct owner instruction, 13 November 2026 (Asia/Tbilisi):** commit the reviewed
follow-through to main. Add further small tests/cases before V1; for work that
takes substantial implementation time, update ROADMAP.md and create detailed
documentation under docs.

**Accepted scheduling:** implement bounded missing tests against existing
production consumers now, with independent expected results and healthy/refused
controls. Prepare the remaining applicable cases before V1 and execute reached
V1 behavior before its existing phase/release exit. Existing correctness,
security, native, package and release requirements are not deferred merely
because their implementation is substantial.

Some reviewed cases concern future Roughtime/Khronos or an unselected TPKI
protocol. Specify their concrete input, expected result and consumer/stage early;
execute them when the actual mechanism reaches its selected implementation.
Preparation or a shared-component test does not qualify an absent runtime, and
agents must not build a dummy mechanism solely to report passing cases.

[LIT-0](../ROADMAP.md#lit-0--literature-derived-qualification-before-v1) and
[NV-NCR-001](supporting-reviews/chrony-ntpsec-noncore-research.md) record priorities,
primary evidence, independent cases, prerequisites, success/refutation criteria
and longer-work dispositions. Existing engine, W11-W01, catalogue, P4 recovery
and TA task owners retain implementation and candidate-specific evidence.
Reuse adequate existing cases rather than creating a parallel acceptance suite.

This instruction adopts qualification work and detailed planning. It does not
select a time-certificate authority, NTxMon service, live catalogue updater,
provider/dependency, timestamp correction, polling constant or new estimator.
Conditional mechanisms require the specified evidence and a concrete later
adoption decision. Paper percentages and server serving counts establish no
NevroTick performance or security result.

Preserve actual V1 → MATH-C01 → MT-C01 → KHR-C01 → GEO-C01 → Android and the
separate RGT-0 → CHR-0 direction. Ordinary regression tests do not run MATH-0
early or complete its exit. OD-10's post-V1 formal deferral and existing
native/fuzz/storage/UI execution holds retain their scopes; source preparation
and separately admitted portable tests continue within current authorization.
