# NevroTick — owner decisions and remaining policy choices

**Document:** NV-DEC-001, revision 1.12<br>
**Decision dates:** 29 October 2026; OD-07 added 30 October; OD-08 added 2 November; OD-05 delivery direction amended, OD-01 future replacement and OD-02 Roughtime/coordination directions added and bootstrap priority refined 3 November; OD-09 leap research/design integration added 4 November; OD-10 mandatory mathematical proofs, OD-11 SOCKS5 scope/deferral and OD-12 GeoIP experiment placement added 5 November 2026, Asia/Tbilisi<br>
**Owner:** David Osipov  
**Research baseline:** [NevroTick at db53dda222739b9b076b1b47a81ace7761b1596b](https://github.com/DavidOsipov/NevroTick/tree/db53dda222739b9b076b1b47a81ace7761b1596b)  
**Repository integration base:** [77ad8c416db32d5edfdc9e6a538d76637eaf44f0](https://github.com/DavidOsipov/NevroTick/tree/77ad8c416db32d5edfdc9e6a538d76637eaf44f0); later tooling, SBOM and dependency-refresh changes are preserved.  
**Status:** Records owner instructions, engineering recommendations and unresolved research separately. This is documentation, not implementation or release qualification.

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

The current UI-M0 restriction remains intact: fixture-only native UI, with D1/D2/D3 deferred at their existing gates. This decision record neither enables live UI integration nor blocks unrelated permitted core/platform work.

## 2. Decision summary

| ID | Owner instruction | Recorded status | What still needs evidence or a decision |
| --- | --- | --- | --- |
| OD-01 | Take selected upstream code, stress-test it, and alter it when bugs or justified optimizations are found; independently replace Chrony-derived implementations after Roughtime | **Accepted current reuse and future replacement direction** | Exact source/dependency licence inventory and notices; permitted Rust/native graph; justified behavioral deviations; future replacement qualification, rights review and public licence selection |
| OD-02 | Support very large clock recovery with several NTS sources, potentially a 100-year error; be more conservative with plain NTP; add a future Roughtime bootstrap stage for a badly wrong initial date | **Accepted behavior goal and future bootstrap direction** | Supported true-date interval, era anchor, exceptional authenticated quorum, direction/startup/runtime rules, correction budgets and real Windows evidence; RGT-0 protocol/provider/trust qualification. Twelve hours is a tentative plain-NTP candidate, not an approved constant |
| OD-03 | Prefer `pool.ntp.org` as the default source; use the supplied NTS directory to find secure servers | **Accepted provider preference; default-profile interaction unresolved** | Pool vendor-zone allocation; which role the Pool fills alongside NTS; qualified NTS operators and compatible time scales |
| OD-04 | Bring concrete policy choices to David when agents encounter them and record the outcomes | **Accepted decision process** | Individual exception decisions stay open until a bounded proposal and evidence exist |
| OD-05 | Prepare one complete MSIX with the strongest compatible verified protections; David will try Microsoft Store submission with services and reconsider delivery if rejected. Retain the previously accepted Foundation/OpenPGP MSI policies as alternatives, with David as release approver | **Accepted delivery and hardening direction, amended 3 November 2026** | Supported least-privilege service provisioning and recovery, actual package/catalog and per-process mitigation evidence, account/capability admission and the Store outcome; direction is selected, implementation and release are not qualified |
| OD-06 | Leave remaining research questions documented for another AI to investigate | **Deferred research** | The companion brief names questions, owners, evidence and stop conditions; deferral grants no implementation waiver |
| OD-07 | Select an x86-64-v3 baseline and runtime-selected higher-feature optimizations; document the research and make the work part of the plan | **Accepted compatibility and optimization direction, 30 October 2026** | Migrate the admitted build/checker configuration; qualify CPU/OS/VM admission, exact-feature dispatch, numerical equivalence and measured benefit. Current generic builds and historical receipts are not v3 or fast-path qualification |
| OD-08 | Build the future Windows DNS feature for the whole computer and its applications; include DNSSEC with hardware acceleration and multiple CPU cores; document the adaptive-cache proposal | **Accepted product and optimization direction, 2 November 2026; detailed cache design remains an evaluated proposal** | Windows integration and observed application coverage, DoQ/bootstrap/pin policy, DNSSEC/crypto admission, CPU dispatch and worker bounds, statistics privacy, cache parameters and measured benefit |
| OD-09 | Use several research agents, including GitHub MCP and Windows Learn MCP, to select a concrete leap policy that handles silent servers and integrate it into repository documents | **Authorized research and documentation, 4 November 2026; engineering design selected in §13** | Implementation, native mapping/event behavior, finite guard/recovery limits and exact event-time evidence remain S1.4/OPEN-7/S0.4/P4; no new clock-authority or execution waiver |
| OD-10 | Make formal proofs mandatory for product mathematical algorithms and continue the source-bound proof work | **Accepted mathematical assurance requirement, 5 November 2026** | Actual-source proof inventory and complete contracts, admitted verifier execution, meaningful defect controls and exact-candidate evidence under CHECK-7/8/25; two initial kernels do not establish whole-product correctness |
| OD-11 | Design a concrete SOCKS5-only proxy profile and its checks; defer it to the roadmap if implementation/support cost is excessive | **Authorized conditional deferral, 5 November 2026; engineering assessment selects deferral** | First Windows delivery remains Direct; SOCKS-0 retains a limited future loopback/no-auth/local-DNS profile and SOCKS-C01. Protocol, dependency, Windows, resolver/verifier, timing and support qualification remain unexecuted |
| OD-12 | Put the geography-assisted time-source selection experiment in the roadmap after Khronos | **Accepted future experiment placement, 5 November 2026** | GEO-0/GEO-C01 compare optional geographic ordering with ordinary acquisition/history reuse. Evaluation/disposition precedes Android; adoption, data/provider choice and any production implementation remain separate decisions supported by evidence |

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
- **Store outcome:** prepare the candidate and accurate service/capability
  justification without waiting for speculative preapproval. David makes the
  account/submission/visibility and release actions. An actual refusal is a
  decision input, not permission to conceal services or silently ship another
  channel. Store acceptance and application security qualification are distinct.
- **Retained free MSI alternative:** SignPath Foundation, if the project qualifies
  and David later returns to the complete MSI route.
- **Publisher display:** the Foundation's publisher identity is acceptable to David. Exact issued identity and renewal policy still require verification.
- **Human release/signing approver:** **David Osipov**.
- **Retained initial MSI fallback:** if that route is chosen and a usable free
  Authenticode service is unavailable, the earlier permission for a complete MSI
  with OpenPGP signatures remains. Preserve legitimate upstream signatures.

The [existing W11-W10/W11-W05 task](../tasks/windows11-integration.md#48-owner-selected-hardened-msix--3-november-2026)
owns the preparation and later qualification. The current portable Core priority,
Windows execution stop, separate fuzz no-retry hold, storage pause and UI-M0
deferrals remain. This amendment authorizes source/design preparation; it claims
no working MSIX, native mitigation enforcement, signing or Store acceptance.

The retained MSI fallback still replaces the earlier blanket rule that lack of
free public Authenticode must block every public release. Neither that permission
nor the new Store direction authorizes automatic publication, removes unrelated
product gates, approves a paid certificate or turns absent evidence into a pass.
SignPath's acceptance remains external. [S12]

### 7.2 Separate release profiles

The selected Store MSIX profile uses the actual delivered package/catalog,
approved package identity and installed component lineage, with the exact
service, mitigation and recovery evidence in
[architecture §19.4](windows-time-architecture.md#194-future-complete-msix-gate)
and [signing §6.1](windows-time-signing-guide.md#61-store-msix-trust-and-publication-boundaries).
Store signing is not an arbitrary publisher allowlist or helper authorization.
The following table retains the two **MSI alternatives**; neither is a parallel
v1 deliverable or a prerequisite to preparing MSIX.

| Requirement | Preferred Authenticode profile | Initial OpenPGP profile |
| --- | --- | --- |
| Installable product | One complete MSI | The same complete-product MSI contract |
| Artifact authenticity | Qualified payload/package signatures and expected publisher | Detached OpenPGP signature over exact final MSI bytes; trusted full release-key fingerprint |
| Release manifest | Final hashes, commit/build identity, source and SBOM lineage | Same information, with an OpenPGP-signed manifest binding version and exact artifacts |
| Windows publisher trust | Verify actual Windows signature/chain/timestamp behavior | Explicitly absent for unsigned owned files; an OpenPGP check does not supply it |
| Installation compatibility | Test actual protections and policies | Test actual protections and report blocking restrictions honestly |
| Human release approval | David approves the identified candidate | David approves the identified candidate |

A valid OpenPGP signature proves a relationship between bytes and a key; the expected key/fingerprint must also be authenticated. Select a release key, publication channel, protected signing process, backup, expiry/revocation and rotation policy before release. This document selects no existing personal key and contains no secret material. Agents may prepare candidates and evidence; they do not acquire signing authority by generating an artifact. [S13]

Windows UAC and application control do not treat detached OpenPGP as Authenticode. Unknown unsigned code can be blocked under Smart App Control or deployment policy. Do not promise warning-free installation or universal Windows 11 compatibility, and do not make disabling protections or installing a development root the normal user procedure. [S14, S15]

OpenPGP sidecars and the release manifest are verification material, not additional application packages. The fallback does not introduce an embedded updater or require adding an OpenPGP library to the runtime. Ordinary MSIX deployment has separate signing requirements; Microsoft's unsigned-MSIX facility is for testing, so it is not the broad-distribution fallback. [S16]

### 7.3 Required follow-through before using the fallback

Keep CHECK-18's Authenticode result **not qualified**. Add separately identified OpenPGP-profile evidence: expected key, valid and modified MSI controls, wrong signer, mismatched manifest/version, exact payload inventory, and complete lifecycle observations under the declared Windows protection configurations. Do not mark skipped Windows-signature tests as passed.

Review every place that currently assumes a signed helper/payload. Establish how an approved package reaches protected installed paths, how ordinary users are prevented from substituting the helper or DLLs, how callers/endpoints are authenticated, and how maintenance validates its inputs and artifact lineage. A hash stored beside an attacker-writable executable is insufficient. Do not replace a signature check with unconditional acceptance. R-06 must produce the branch-specific integrity/identity design before this release route is qualified.

All licence/source obligations, runtime hardening, privileged-boundary checks and relevant feature gates remain. Later migration to Authenticode must preserve one-product upgrade identity and reject unsupported rollback or silent reversion to a weaker release policy.

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

## 8. OD-06 — future research and decision queue

The [research handoff](nevrotick-research-handoff.md) is the bounded assignment for another AI. It separates protocol/platform findings from choices that require David and from provider decisions nobody on the project can grant.

Priority order:

1. Confirm selected-source licence obligations and an admissible crypto/native graph before port/dependency commitments.
2. Resolve the Pool/NTS default interaction and provider permission before shipping defaults.
3. Prove era-aware authenticated recovery and propose conservative plain-NTP limits before clock actuation.
4. Prepare the owner-selected complete hardened MSIX and Store submission case
   with services through OD-05 and the existing W11 task. Qualify supported
   provisioning, recovery, installed trust and per-process protections. Retain
   unsigned-MSI trust and SignPath eligibility research for a later owner-selected
   alternative; do not make those provider answers prerequisites to MSIX work.
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

**Decision and alternatives:** Select `x86-64-v3` as the intended minimum for the
Windows x64 product and ahead-of-time, runtime-selected specialized functions.
The previous generic `x86-64` floor remains the currently admitted implementation
until its controlled migration. Keeping that floor indefinitely would not deliver
the selected compatibility change; globally compiling for v4 would exclude
the intended v3 fallback machines. Do not use `target-cpu=native` for distributed
artifacts. Additional v4 capabilities authorize only functions whose full enabled feature
set is available; CPU capability alone does not establish a speed benefit.

**Evidence and limits:** The [research report](supporting-reviews/x86-runtime-optimization.md)
records primary Rust and processor-level sources, the pinned compiler's feature
queries and limitations. There are no new native Windows compatibility or
performance results. The decision does not admit an unreviewed dependency,
unsafe numerical shortcut, weaker mitigation, new floating-point environment or
crypto provider. AES acceleration remains a separate per-feature provider choice.

**Implementation/validation:** The build owner implements baseline admission and
the affected checker/configuration changes; each measured kernel owner and an
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
existing privilege boundaries and first KHR-0 follow-up retain their order and
authority. No DNS implementation, host configuration change, benchmark, native
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
requirements remain. This does not reorder the current proof-first work or future
KHR/Android and RGT/CHR sequences.

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
