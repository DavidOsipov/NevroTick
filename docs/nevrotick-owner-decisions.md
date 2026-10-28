# NevroTick — owner decisions and remaining policy choices

**Document:** NV-DEC-001, revision 1.0  
**Decision date:** 29 October 2026, Asia/Tbilisi  
**Owner:** David Osipov  
**Research baseline:** [NevroTick at db53dda222739b9b076b1b47a81ace7761b1596b](https://github.com/DavidOsipov/NevroTick/tree/db53dda222739b9b076b1b47a81ace7761b1596b)  
**Repository integration base:** [77ad8c416db32d5edfdc9e6a538d76637eaf44f0](https://github.com/DavidOsipov/NevroTick/tree/77ad8c416db32d5edfdc9e6a538d76637eaf44f0); later tooling, SBOM and dependency-refresh changes are preserved.  
**Status:** Records owner instructions, engineering recommendations and unresolved research separately. This is documentation, not implementation or release qualification.

## 1. Authority and purpose

This record captures David's replies about reuse, correction policy, default servers, case-by-case decisions, release signing and future research. It also incorporates his supplied [NTS server directory](https://github.com/jauderho/nts-servers) as a discovery input.

Read this alongside the [architecture](windows-time-architecture.md), [feature plan](windows-time-features.md), [signing guide](windows-time-signing-guide.md) and [research handoff](nevrotick-research-handoff.md). The accepted changes are integrated into those canonical documents. Use this record to distinguish owner decisions from pending recommendations when reading or changing them. Existing requirements remain in force except where an accepted change below explicitly amends them. Recommendations do not become owner decisions merely because an agent wrote them down.

The current UI-M0 restriction remains intact: fixture-only native UI, with D1/D2/D3 deferred at their existing gates. This decision record neither enables live UI integration nor blocks unrelated permitted core/platform work.

## 2. Decision summary

| ID | Owner instruction | Recorded status | What still needs evidence or a decision |
| --- | --- | --- | --- |
| OD-01 | Take selected upstream code, stress-test it, and alter it when bugs or justified optimizations are found | **Accepted reuse direction** | Exact source/dependency licence inventory and notices; permitted Rust/native graph; justification for each intentional behavioral deviation |
| OD-02 | Support very large clock recovery with several NTS sources, potentially a 100-year error; be more conservative with plain NTP | **Accepted behavior goal** | Supported true-date interval, era anchor, exceptional authenticated quorum, direction/startup/runtime rules, correction budgets and real Windows evidence. Twelve hours is a tentative plain-NTP candidate, not an approved constant |
| OD-03 | Prefer `pool.ntp.org` as the default source; use the supplied NTS directory to find secure servers | **Accepted provider preference; default-profile interaction unresolved** | Pool vendor-zone allocation; which role the Pool fills alongside NTS; qualified NTS operators and compatible time scales |
| OD-04 | Bring concrete policy choices to David when agents encounter them and record the outcomes | **Accepted decision process** | Individual exception decisions stay open until a bounded proposal and evidence exist |
| OD-05 | Accept the Foundation publisher display; David Osipov is the human approver; use OpenPGP signatures initially if free Authenticode is unavailable | **Accepted release policy** | Provider eligibility or a qualified OpenPGP release process; exact artifacts, key identity, installed-file trust and Windows compatibility evidence |
| OD-06 | Leave remaining research questions documented for another AI to investigate | **Deferred research** | The companion brief names questions, owners, evidence and stop conditions; deferral grants no implementation waiver |

No item here establishes a completed security assessment, provider acceptance, safe numerical bound, verified release, or native Windows result.

## 3. OD-01 — source-derived port with independent verification

David intends to reuse actual code and permit evidence-driven repairs and optimization. Treat that as a source-derived port. There is no fixed percentage of changed code that agents can use to declare it independent.

The selected Chrony baseline remains **4.9, commit `8df4f1263e206585e6c4e61352d042c2dd12916a`**. For example, the inspected selection-code header grants GNU GPL version 2 without an “or later” clause. The GNU FAQ treats translation into another programming language as modification covered by the original grant. Fixing or optimizing a Rust translation therefore does not establish permission to relabel the derived code GPL-3.0-or-later. This record confirms the reuse direction; it does not invent an additional licence grant or declare the whole future dependency graph cleared. [S1, S2]

Agents must maintain a port ledger containing upstream project/revision/file/function, selected licence, retained notice, Rust owner, independent test expectations, upstream tests reused, and intentional deviations. Preserve any reproducible upstream counterexample. A differential disagreement requires investigation: either the port, the reference, the fixture or the assumption may be wrong.

Apply the existing architecture §20.1 and development-hardening §9.7 policy: independent numerical expectations, property/fuzz tests, adversarial simulation, bounded stress, actual Windows integration checks where required, and performance measurements with protections enabled. Compare optimization against correctness and resource evidence; neither faster output nor agreement with Chrony alone is sufficient. Keep bug fixes and performance changes separately reviewable where practical.

Before shipping, resolve the root application licence and actual source/native/build/procedural-macro graph under OPEN-1/CHECK-20/26. Keep proprietary branding and application-source licensing explicit and separate. Do not remove upstream attribution, fabricate clean-room provenance or treat process boundaries as automatic licensing exemptions.

**Owner escalation:** a proposed reuse/dependency change requires incompatible rights, a deliberate algorithm deviation changes security or observable behavior, or a measured performance tradeoff changes the product requirements. Routine compatible fixes within the approved contracts need no repeated policy approval.

## 4. OD-02 — distinguish authenticated recovery from ordinary correction

### 4.1 What is accepted

The product should aim to recover a badly wrong local date without an arbitrary small limit merely because the offset is large, provided the authenticated evidence and correction policy justify it. A local clock wrong by approximately 100 years is an explicit research/acceptance target.

That is different from accepting an unrestricted target date or allowing any large step during normal operation. Until the wider path is qualified, existing conservative broker limits apply. Preserve source agreement, fresh evidence, current machine-policy authorization, sole clock ownership, cumulative accounting, and acknowledgment of the actual applied effect.

### 4.2 Roles of the sources

- **NTS sources:** supply authenticated observations eligible for agreement and selection under the configured policy.
- **Plain NTP helper:** optionally supplies a bounded provisional date to resolve certificate-time circularity. It has no vote in the NTS source population and no direct clock-setting authority.
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

- **Preferred free route:** SignPath Foundation, if the project qualifies and the workflow is approved.
- **Publisher display:** the Foundation's publisher identity is acceptable to David. Exact issued identity and renewal policy still require verification.
- **Human release/signing approver:** **David Osipov**.
- **Initial fallback:** if a usable free Authenticode route is unavailable, permit an initial **complete MSI without project Authenticode signatures**, accompanied by **OpenPGP signatures**. Preserve legitimate upstream signatures where present.

This explicitly replaces the earlier blanket rule that lack of free public Authenticode must block every public release. It does not authorize an automatic release, remove unrelated product gates, approve a paid certificate, split the package, or turn absent Authenticode evidence into a pass. SignPath's acceptance remains external. [S12]

### 7.2 Separate release profiles

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

## 8. OD-06 — future research and decision queue

The [research handoff](nevrotick-research-handoff.md) is the bounded assignment for another AI. It separates protocol/platform findings from choices that require David and from provider decisions nobody on the project can grant.

Priority order:

1. Confirm selected-source licence obligations and an admissible crypto/native graph before port/dependency commitments.
2. Resolve the Pool/NTS default interaction and provider permission before shipping defaults.
3. Prove era-aware authenticated recovery and propose conservative plain-NTP limits before clock actuation.
4. Research the unsigned release trust path and SignPath eligibility, including proprietary brand assets and port treatment, before the first public release.
5. Continue the existing native account/IPC/clock/lifecycle/resource investigations at their phase gates. Bring concrete exceptions rather than a speculative list of approvals.

The proprietary-artwork/free-signing question is **unresolved**. This work neither concludes SignPath rejects the project nor changes the brand licence. Research and any later authorized provider clarification must answer it.

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
