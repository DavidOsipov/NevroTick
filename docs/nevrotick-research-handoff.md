# NevroTick — research handoff for the next AI

**Document:** NV-RESEARCH-001, revision 1.0  
**Prepared:** 29 October 2026, Asia/Tbilisi  
**Owner:** David Osipov  
**Input:** [Owner decisions NV-DEC-001](nevrotick-owner-decisions.md), [architecture](windows-time-architecture.md), [feature plan](windows-time-features.md), [signing guide](windows-time-signing-guide.md)  
**Repository integration base:** `DavidOsipov/NevroTick@77ad8c416db32d5edfdc9e6a538d76637eaf44f0`. This maintained assignment accompanies the integrated owner-decision record; retrieve the current checkout and relevant updates before acting.

## 1. Assignment and limits

Investigate the unresolved questions below and produce concrete options with evidence. This assignment is research and planning. It does not authorize provider contact, accepting terms, submitting vendor/signing applications, spending money, generating release credentials, public releases or changes to the user's Windows clock/services/trust stores.

Retrieve the current project instructions and relevant source before starting; detect changes from the baseline. Apply the project's planning, relevant security, testing-contract and independent-review skills. Latest owner instructions take precedence over older proposed policy. Preserve UI-M0 deferrals and existing numerical/protocol/security boundaries.

Use primary operator, RFC, Microsoft, upstream-source and licence evidence. Pin upstream source revisions, distinguish compiled defaults from sample/distribution configuration, and retain contradictions. Use Context7 when it provides the relevant current API documentation, but qualify the actual selected version/features/target.

Return findings in Markdown. Each conclusion must say whether it is an observed fact, engineering inference, recommendation, owner decision required, provider approval required, or unexecuted test. Research on a source file is not a successful runtime experiment.

## 2. Priority and dependencies

| Task | Existing gates | Deliverable | Blocks only |
| --- | --- | --- | --- |
| R-01 — derivation and exact licence graph | OPEN-1/13; CHECK-20/26 | Source/port ledger, permitted dependency graph, unresolved grant questions | Dependent source/dependency adoption and distribution |
| R-02 — authenticated large-error recovery | OPEN-5/6/8; CHECK-7/8/9/10/12 | Era/trust model, exceptional correction policy options, independent test plan | Wider automatic clock authority |
| R-03 — plain-NTP correction limits | OPEN-5/9; CHECK-3/6/7/10/12 | Proposed numerical envelope with attack and usability evidence | Plain-profile automatic actuation policy |
| R-04 — source preset and provider use | DEC-5/7/8; OPEN-6/8 | Four-operator candidate assessment; Pool role and vendor requirements | Shipped default preset and associated claims |
| R-05 — Windows certificate/time behavior | OPEN-1/8; CHECK-9/26/27 | Exact provider/trust/revocation/egress qualification design and evidence | NTS implementation paths relying on those guarantees |
| R-06 — release profiles and signing eligibility | OPEN-11/14; CHECK-16/18/20/23 | Authenticode and initial OpenPGP release evidence paths | The selected public release profile |
| R-07 — concrete owner exceptions | OPEN-5/8/9 and affected clauses | Decision cards with compensating controls and bounded residual risk | Affected behavior or assurance claim |
| R-08 — remaining native feasibility | OPEN-2/3/4/7/10/12/13/14 | Measured/verified options at the relevant phase | Dependent native integrations and release claims |

These tasks do not reopen the settled native Win32 direction, two-service authority split, ordinary-user UI, time-only v1, Windows 11 x64 first, or single complete MSI. Future complete MSIX remains research, with no separate MSI service prerequisite.

## 3. R-01 — source reuse and licence compatibility

David intends to take selected upstream code, test it, and modify it where needed. Do not label this clean-room merely because the implementation language changes or many lines are rewritten.

Start with Chrony **4.9 / `8df4f1263e206585e6c4e61352d042c2dd12916a`**. Inspect every imported file's grant and dependencies, not just the repository's generic COPYING text. Preserve source provenance, attribution, corresponding-source/build obligations and intentional deviation records. Source headers already support the existing GPL-2.0-only constraint; no new GPL-3 permission has been granted by this conversation.

Produce a table of actual proposed Rust/native/build dependencies, exact versions and features, selected alternative licence grants, embedded third-party code and compatibility evidence. Revisit the existing held crypto-provider graphs rather than assuming a crate's top-level licence settles its contents. Changes after bugs or optimization retain their derivation record.

Escalate a concrete rights conflict or material behavioral deviation to David. Do not ask him to choose a preferred licence label independently of the source he intends to reuse. Do not remove notices to satisfy an installer or signing-provider requirement.

## 4. R-02 — century-scale authenticated recovery

### Required research result

Design a bounded recovery path that can correct a very wrong local date when authenticated evidence is sufficient. Separate the supported **true date** from the initial **Windows date error**. Investigate both forward and backward 100-year errors, without assuming both directions or all runtime circumstances have already been approved for automatic steps.

Specify:

- Source and trust of the era anchor; supported interval; behavior with no saved state, old binaries, clock rollback and invalid protected state.
- Full-width internal time representation, exact units, interval conversion and checked arithmetic. No implicit narrowing to signed 32-bit Unix time or ambiguous modulo timestamp differences.
- NTS certificate-time bootstrap, cached-association path, fresh request/replay checks, candidate-time revalidation and retained provenance.
- Operator/endpoint deduplication, uncertainty/interval agreement, majority/minimum-source semantics and a proposed exceptional quorum. Evaluate three independent agreeing NTS identities as a candidate, not an accepted requirement.
- Separate acquisition-time and unexpected running-system recovery; approved target interval/directions, one-use authority, cumulative budgets, restart/reboot policy and ambiguous-acknowledgment reconciliation.
- Observable failure behavior and recovery remedies when certificates, era, quorum or broker authority cannot be established.

Study [RFC 5905](https://www.rfc-editor.org/rfc/rfc5905.html#section-6), [RFC 8915](https://www.rfc-editor.org/rfc/rfc8915.html#section-8), and the selected Chrony [configure](https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/configure) / [util.c](https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/util.c) era mapping. Do not derive the era solely from the broken OS clock. Do not claim a lower-bound date, NTS label, fresh handshake or multiple aliases alone proves UTC.

### Required controlled cases

| Case family | Expected property |
| --- | --- |
| Local date ±100 years; true date inside/outside supported interval; 2036 rollover; era boundaries | Correct independent interpretation when supported; explicit rejection when ambiguous or out of range |
| No trusted floor, stale build, corrupted state, restored old snapshot | No permissive default or unearned date trust |
| One malicious NTS source, duplicate operator aliases, contradictory intervals, loss of quorum | No duplicate votes, no silent minimum reduction, no unauthenticated substitute |
| One/two bad helpers, disagreement, timeout, replay, no NTS completion | Hints cannot change Windows time or enter ordinary selection |
| Expired/not-yet-valid or revoked certificates; compromised old key made valid at the hinted date | Normal identity/signature policy retained; residual bootstrap attack represented honestly |
| Repeated small shifts, frequency abuse, sleep/resume, service crashes, acknowledgment loss | Real effects consume persistent authority; no repeated step or cleanup that changes a successor's clock policy |
| Large runtime changes with representative time-sensitive applications | Measured consequences and a reasoned proposal for additional approval boundaries |

Begin with deterministic simulation and controlled protocol fixtures. Windows execution requires an owned disposable VM, bounded experiment, independent observer and tested restoration. Public time services are not stress-test targets.

## 5. R-03 — conservative plain-NTP policy

Compare NevroTick's intended behavior with the [Chrony 4.9 manual](https://chrony-project.org/doc/4.9/chrony.conf.html), [FAQ](https://chrony-project.org/faq.html), [NTPsec manual](https://docs.ntpsec.org/latest/ntpd.html) and [pinned NTPsec loop filter](https://github.com/ntpsec/ntpsec/blob/61eaa902b90e374698c1b4e380b0f27ed6632c65/ntpd/ntp_loopfilter.c). Do not translate a sample `makestep` directive into a maximum security bound.

Starting evidence: NTPsec's ordinary panic threshold is 1,000 seconds, separate from its 128 ms step threshold and startup exception. Chrony exposes different slew, step and maximum-change controls. Investigate smaller automatic bounds and a 1,000-second comparison case before considering the user's tentative 12-hour ceiling. The user's “or smaller” leaves the final value open.

Record an upstream discrepancy: rendered NTPsec configuration documentation reports 900 seconds for default stepout, while the inspected loop-filter snapshot defines `CLOCK_MINSTEP` as 300 seconds. Preserve the pin and reconcile the actual version/configuration; neither value is a NevroTick requirement. Also examine `-g` state-machine consumption rather than assuming it is a persistent Windows Service approval counter.

Deliver bounds for magnitude, rate, direction, age/uncertainty, cumulative absolute phase movement, exceptional count, boot/session recovery and holdover. Evaluate shared-path attackers controlling multiple plain replies. Show both malicious cases and a legitimate offline/RTC-drift case. A rate cap alone may permit cumulative corruption; a bound that rejects everything fails the legitimate-control requirement.

Ask David only after presenting measured options and their practical recovery differences. The resulting profile must remain visibly unauthenticated.

## 6. R-04 — default servers and the supplied NTS list

### Inspected discovery source

David supplied **[jauderho/nts-servers](https://github.com/jauderho/nts-servers)**. The inspected snapshot is **`90269c99d9c247a1a42659521c20dfa1e430202f`**, committed 26 October 2026:

- [README at the inspected revision](https://github.com/jauderho/nts-servers/blob/90269c99d9c247a1a42659521c20dfa1e430202f/README.md).
- [nts-sources.yml at the inspected revision](https://github.com/jauderho/nts-servers/blob/90269c99d9c247a1a42659521c20dfa1e430202f/nts-sources.yml).

The directory explicitly disclaims endorsement. We inspected its data; no bundled scripts were run and no connectivity or load test was performed. Do not make runtime configuration depend on automatic fetching of this mutable third-party repository. Do not copy its broad mixing statement into the protocol contract: NevroTick's isolated helper role and selection policy have their own specification.

### Proposed shortlist — qualification pending

| Operator | Candidate identity | Evidence already found | Remaining gate |
| --- | --- | --- | --- |
| Cloudflare | `time.cloudflare.com` | Operator documents NTS at this hostname and explicitly says its time service does not smear leap seconds | Review incorporated use terms for distribution/load; anycast association changes, negotiated targets, actual trust/AEAD compatibility and regional reachability |
| Netnod | `nts.netnod.se` | Operator offers free NTS worldwide at this identity | Qualify current leap policy, permitted default-client load/polling and anycast behavior |
| PTB | `ptbtime1.ptb.de` | PTB documents free NTS without registration on its public time servers | Qualify selected endpoint, leap behavior, traffic policy and distributed-default suitability |
| SIDN Labs / TimeNL | `ntppool1.time.nl` for research | Operator recommends this production family; main service documentation states no leap smearing | Vendor/default use needs prior arrangement and an assigned name where required; no invented NevroTick name may ship |

These represent four candidate operators. They do not establish independent reference chains, implementations, routes, certificate authorities or failure modes. Additional addresses from one provider are availability alternatives, not extra independent votes.

Primary operator sources:

- Cloudflare: [NTS](https://developers.cloudflare.com/time-services/nts/), [NTP/leap policy](https://developers.cloudflare.com/time-services/ntp/), [terms entry point](https://developers.cloudflare.com/time-services/tos/).
- Netnod: [NTS service](https://www.netnod.se/nts/network-time-security), [free time services](https://www.netnod.se/free-time-services), [source-diversity guidance](https://www.netnod.se/blog/best-practices-connecting-ntp-servers). The guidance article is historical advice, not a current endpoint qualification certificate.
- PTB: [public NTP/NTS service](https://www.ptb.de/cms/ptb/fachabteilungen/abt9/gruppe-95/ref-952/zeitsynchronisation-von-rechnern-mit-hilfe-des-network-time-protocol-ntp.html), [technical background](https://www.ptb.de/cms/ptb/fachabteilungen/abt9/gruppe-95/ref-952/zeitsynchronisation-technische-hinweise.html). Do not infer current software versions from historical text.
- TimeNL: [production and experimental NTS endpoints](https://nts.time.nl/), [service, vendor and leap policies](https://time.nl/index_en.html).

Do not select Ubuntu's distro-only endpoints for NevroTick merely because they appear in the list. TimeNL's `nts1.time.nl` is experimental, and its main FAQ describes `any.time.nl` as an experimental anycast testbed; prefer the provider's production family when researching a shipped preset.

### Pool and default-profile resolution

The NTP Pool preference is recorded, but Pool service is plain NTP and a shipped default needs an allocated vendor zone. See [authentication](https://news.ntppool.org/docs/authentication/) and [vendor policy](https://www.ntppool.org/vendors.html). The recommendation is NTS for normal operation and Pool for the deliberate plain profile/helper role. If David wants Pool-only as the primary default instead, present that explicit DEC-5 change and consequences. Do not infer it from the provider name or treat the supplied NTS list as approval of the four-host shortlist.

Produce one admission record per selected provider: official identity and ports, operator/reference dependencies, geographic suitability, permission/terms and date, leap/smear policy, DNS/delegated endpoint rules, normal/burst/KoD limits, TLS/AEAD/Windows-verifier behavior, observation environment and open conditions. Apply bounded backoff and provider restrictions. Do not freeze IP addresses or treat two rotating Pool results as assured independent operators.

## 7. R-05 — Windows trust with a wrong system date

Qualify the exact Rust TLS provider, verifier, native dependencies and selected Windows service context. A provisional time argument does not establish that every Windows trust/revocation operation uses that time.

Document trust-store ownership, enterprise policy and explicit distrust, chain/hostname/EKU/algorithm checks, known-revoked versus unknown status, offline semantics, validity at candidate time, Windows API date range, resumption isolation and cached-association policy. Trace direct and delegated DNS/AIA/CRL/OCSP/network work, redirects and native cancellation/resource ownership. Preserve the existing conditional ASVS retrieval analysis.

Test wrong name, expired/future certificates, revoked intermediates, unavailable responders, untrusted roots and extreme date conversions with valid neighboring controls. Reconcile OPEN-8's previously pinned source lead against the actual selected version. If the necessary policy cannot be enforced, report the blocked path and alternatives rather than disabling checks. A residual-risk change requires an OD-04 decision card.

## 8. R-06 — signing eligibility and initial OpenPGP releases

### Accepted owner policy to preserve

SignPath Foundation's publisher display is acceptable. **David Osipov** approves releases. If no usable free Authenticode route is available initially, a complete MSI with OpenPGP verification artifacts is permitted as a release-policy alternative. This is not permission to skip qualification or sign/publish without David's approval.

### Research questions

1. How do the provider's current terms treat a Rust port of Chrony, third-party/native payloads, the first-release condition and proprietary NevroTick brand assets? The current [SignPath conditions](https://signpath.org/terms.html) are labelled Draft. Give supported interpretations and questions for later authorized clarification; do not assert approval or rejection or change the artwork licence.
2. For the OpenPGP profile, what is the precise chain from approved source/build to final MSI, trusted key fingerprint, verified download, elevated installation, protected helper/service paths and future maintenance? Review existing signed-helper assumptions, not just the final MSI signature.
3. Which Windows 11 security configurations allow or block the unsigned installer and payloads? Test actual MSI, UI, helper and services; keep protections active for the claimed profile. A blocked configuration is an unsupported route/result to disclose, not a reason to advise disabling protection.
4. What minimum release-key custody, expiry/revocation, backup and rotation process can David operate? Also investigate verification when the user's date is badly wrong: signature/key lifetime checks and a trustworthy time basis must not be bypassed by trusting the unverified installer as an oracle. Prepare a process without generating or exposing private keys. Do not automatically reuse a personal Git commit key.
5. How does the later Authenticode transition preserve product identity, upgrade/repair/uninstall behavior and explicit anti-downgrade policy? Which trust checks remain channel-specific?

Deliver separate CHECK-18 evidence plans for the Authenticode and OpenPGP profiles. Include correct signer/bytes, tamper, wrong signer, stale/mismatched manifest, payload substitution, maintenance attacks and valid lifecycle controls. Preserve the Authenticode result as unqualified when only OpenPGP is available. Runtime DLL protections, ACLs, native mitigations and licence obligations remain mandatory for the supported release profile.

Primary references: [GnuPG verification and fingerprints](https://gnupg.org/download/integrity_check.html), [Microsoft UAC](https://learn.microsoft.com/en-us/windows/security/application-security/application-control/user-account-control/how-it-works), [Smart App Control](https://learn.microsoft.com/en-us/windows/apps/develop/smart-app-control/overview), [MSIX signing](https://learn.microsoft.com/en-us/windows/msix/package/signing-package-overview), [unsigned MSIX testing](https://learn.microsoft.com/en-us/windows/msix/package/unsigned-package).

Do not add a second application package, automatic updater, provider application, paid subscription or OS-protection bypass as a side effect of this research.

## 9. R-07 and R-08 — exception decisions and remaining platform work

For every relevant ASVS requirement, separate literal applicability, native adaptation, implementation, observation and unresolved exception. Bring David individual evidence-backed decisions about hardware-backed isolation, authentication/revocation semantics, in-use secrets, logging, delegated retrieval or plain-NTP assurance. No blanket L3 acceptance has been granted.

Continue the existing phase-specific investigations for account/token/DACL separation, race-safe IPC, real Windows rate/step/leap semantics, recovery after process death, exact supported builds, installer-tool terms, native resource budgets and complete MSIX feasibility. Propose a decision only when an actual tradeoff remains after authorized investigation. Keep full UI qualification deferred at the UI-M0 reentry triggers; completing a fixture prototype closes no later accessibility gate.

## 10. Required output and completion criteria

Return:

1. A finding table mapping each research question to evidence, limitations, recommendation, affected gate and owner.
2. Source pins and current provider policies; disclose contradictory documentation and any unavailable runtime environment.
3. Proposed exact changes to the canonical docs, preserving existing identities and amendment history.
4. Concrete OD-04 decision cards only for choices that need David; report provider approval separately.
5. A bounded next implementation/experiment stage with independent valid/hostile expectations, environment, limits, cleanup and observable exit.

An independent reviewer should assess the changed proposal. Review agreement accepts no residual risk and proves no native test. Leave unavailable tests incomplete. Continue unaffected work while a dependent decision remains open.

**Success:** the next implementer knows the owner of every changed rule, the exact unresolved fact, the evidence required to resolve it, and the effect that must remain disabled until that gate passes. Do not declare the whole product ready merely because this research is complete.
