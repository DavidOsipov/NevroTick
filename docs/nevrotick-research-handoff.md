# NevroTick — research handoff for the next AI

**Document:** NV-RESEARCH-001, revision 1.4<br>
**Prepared:** 29 October 2026, Asia/Tbilisi  
**Owner:** David Osipov  
**Input:** [Owner decisions NV-DEC-001](nevrotick-owner-decisions.md), [architecture](windows-time-architecture.md), [feature plan](windows-time-features.md), [signing guide](windows-time-signing-guide.md)  
**Repository integration base:** `DavidOsipov/NevroTick@77ad8c416db32d5edfdc9e6a538d76637eaf44f0`. This maintained assignment accompanies the integrated owner-decision record; retrieve the current checkout and relevant updates before acting.

**30 October 2026 trust-lifecycle amendment:** R-02/R-04/R-05 now require explicit authentication ages, semantic time-floor recovery and vote/freshness rules before the dependent production interfaces are frozen. These refine existing open gates; they adopt no key lifetime, stronger quorum, floor-reset privilege or new source preset. Source review and controlled-case specifications below establish no implemented NTS or Windows behavior.

**3 November 2026 delivery amendment:** [OD-05](nevrotick-owner-decisions.md#7-od-05--signing-and-release-ownership)
selects complete hardened MSIX preparation and David's Store attempt with services.
The [focused plan](plans/windows-msix-store.md) and existing W11-W10/W11-W05 task
own that work; Store preapproval and MSI completion are not preparation gates.
Technical qualification and actual Store outcome remain open; existing execution
holds and Core priority remain.

**4 November 2026 leap-policy amendment:** [architecture §10.5](windows-time-architecture.md#105-leap-seconds-and-time-scales)
and [NV-LP-001](plans/leap-second-policy.md) select reviewed compiled leap history,
separate future coverage and the continuous-UTC rule effective **20 May 2027**,
distributed in the authenticated complete release. Correct unsmeared sources
remain usable even if all send LI=0. Unknown future coverage alone does not block
ordinary tracking outside bounded ambiguity guards. The [S1.4 task](../tasks/s1.4-leap-policy.md)
owns the pure contract; R-02/R-04/R-08 carry its date, provider and native evidence
requirements into their existing assignments. The kernel alone executes explicit
events; no source vote, global leap-setting change or synthetic ±1-second broker
request is adopted. Source/design selection is separate from implementation,
native qualification and the existing execution holds.

**5 November 2026 UI amendment:** [OD-13](nevrotick-owner-decisions.md#17-od-13--slint-as-the-default-ui-toolkit)
selects complete Slint replacement against v0.4, with winit/software and
Fluent-style widgets as the measured starting configuration. The current Windows
entry selects that toolkit; the old Win32 adapter remains unexported historical
source. The [current GUI handoff](../tasks/slint-gui-handoff.md) owns fixture source
and portable evidence. OD-13's 6 November amendment selects separately
GPL-3.0-only original GUI composition while preserving GPL-2.0-only Core/engine.
R-01 retains rights/shared-code/exact graph and distribution review; R-08/UI-M0
retain generated-code, Windows/accessibility, resource and package qualification.
No current execution hold or UI-M0 deferral is lifted.

**5 November 2026 clock-limit preparation:** [W11 §4.31](../tasks/windows11-integration.md#431-clock-correction-limits-preparation--5-november-2026)
consolidates pinned upstream, Microsoft and protocol/model evidence for
OPEN-5/7/13 under S0.4/S0.7/P4 and W11-W02/04/05. Its field matrix separates
policy tolerance, native capability, model assumptions and enforced authority.
Measurements, production values and decisions remain open; current Core proof
priority, native/S134 stop, separate fuzz hold, optional-storage pause and
limited Sandbox scope remain.

**5 November 2026 platform amendment:** [OD-14](nevrotick-owner-decisions.md#18-od-14--compact-windows-platform-scope)
supersedes the generic Windows-11-x64-first wording below with Windows 11 26H2
x64 and Server 2025 x64 Desktop Experience. R-08 uses the existing
[two-family matrix](../tasks/windows11-integration.md#432-two-family-windows-qualification-matrix--5-november-2026)
for exact profiles, admission/delivery, fixture availability and native evidence;
OPEN-2 and all current execution/acceptance gates remain open.

**Controlling proof amendment, 6 November 2026:**
[OD-10 §14.1](nevrotick-owner-decisions.md#141-first-release-deferral--6-november-2026)
defers formal proof work until after the first released version (V1). Earlier
proof-priority language below is retained as dated context; follow the
[current development queue](plans/development-plan.md#7-next-executable-task-for-a-development-agent).
Missing proofs do not block V1 implementation, acceptance or release. This
amendment resumes no proof compiler/solver job or verifier/metadata repair and
lifts no Windows, fuzz or storage hold; ordinary runtime, behavioral, coverage/
mutation, security, native, package and release gates remain.

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
| R-06 — release profiles and signing eligibility | OPEN-11/12/14; CHECK-16/18/20/23/26 | Production signing for every supported shipped format, actual Store package/catalog and inner-signature preservation, capability and installed-trust evidence; MSI delivery remains a later owner choice | Dependent release claims, not independent source/design preparation; the unsigned-MSI/OpenPGP exception is superseded |
| R-07 — concrete owner exceptions | OPEN-5/8/9 and affected clauses | Decision cards with compensating controls and bounded residual risk | Affected behavior or assurance claim |
| R-08 — remaining native feasibility | OPEN-2/3/4/7/10/12/13/14 | Measured/verified options at the relevant phase | Dependent native integrations and release claims |

These tasks do not reopen the
[OD-13 Slint replacement decision](nevrotick-owner-decisions.md#17-od-13--slint-as-the-default-ui-toolkit),
two-service authority split,
ordinary-user UI, time-only v1 or Windows 11 x64 first. The selected delivery target
is now one complete hardened MSIX; its supported service provisioning and recovery
remain to be qualified, with no separate MSI service prerequisite. Retained MSI
profiles are alternatives for a later owner decision. The old Win32 adapter and
observations remain historical baseline evidence; current Slint fixture source
and portable evidence are recorded in the GUI handoff. Exact version/features/
backend, rights/graph/distribution and native behavior retain their own gates. Toolkit
selection neither removes required Windows OS adapters nor lifts execution holds.

## 3. R-01 — source reuse and licence compatibility

David intends to take selected upstream code, test it, and modify it where needed. Do not label this clean-room merely because the implementation language changes or many lines are rewritten.

Start with Chrony **4.9 / `8df4f1263e206585e6c4e61352d042c2dd12916a`**. Inspect every imported file's grant and dependencies, not just the repository's generic COPYING text. Preserve source provenance, attribution, corresponding-source/build obligations and intentional deviation records. Source headers support the existing GPL-2.0-only constraint for Chrony-derived code; OD-13's accepted GPL-3.0-only grant applies to the original GUI and does not relicense Core/engine or upstream inputs. Use the [GUI source/grant inventory](../crates/nevrotick-ui/LICENSING.md) alongside the Core port ledger; actual shared-code rights and complete distribution remain review obligations.

Produce a table of actual proposed Rust/native/build dependencies, exact versions and features, selected alternative licence grants, embedded third-party code and compatibility evidence. Revisit the existing held crypto-provider graphs rather than assuming a crate's top-level licence settles its contents. Changes after bugs or optimization retain their derivation record.

Escalate a concrete rights conflict or material behavioral deviation to David. Do not repeat the accepted separate GUI/Core licence choice; escalate only a concrete unresolved rights/graph issue with the source and grants involved. Do not remove notices to satisfy an installer or signing-provider requirement.

## 4. R-02 — century-scale authenticated recovery

Use [the clock-limit handoff](../tasks/windows11-integration.md#431-clock-correction-limits-preparation--5-november-2026)
for supported target-date/era versus local-error fields, one-use step authority,
durable unknown outcomes and qualified actual feedback. This source preparation
selects no exceptional quorum, floor-supersession rule or production envelope.

**Future Roughtime coordination:** [NV-TA-001](plans/time-assurance-integration.md)
and the [shared task](../tasks/time-assurance-integration.md) now retain the
owner-selected Roughtime-first cold bootstrap and conditional Khronos/Ananke
provisional-date reserve. R-02 still owns era, interval, fresh
agreement and saved-floor supersession questions; TA-C01–TA-C22 add combined
dependency/frame/conflict cases without replacing these requirements. Neither
an auxiliary agreement nor a new source label proves independent corroboration
or grants correction authority. Include forced fallback, common-path Pool
manipulation plus expired credentials, and preservation of signed conflicts.
The dated provider census is source evidence, not live availability or an
admitted roster. Runtime cooperation does not reorder delivery.

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

### Leap-aware date admission

Carry the admitted release dataset/effective-rule
revision, complete true-date/era hypotheses, source-scale provenance and compatible
segment into recovery. A host date wrong by a century cannot select the regime
or establish calendar freshness. An interval straddling the rule's effective date
can proceed only when all admitted hypotheses yield the same event-free
interpretation and admissible target. A hypothesis crossing a possible event
withholds the dependent sample's clock authority, not independent bootstrap
acquisition or NTS transport. Do not make discipline eligibility a prerequisite
for obtaining the date evidence needed to decide that eligibility.

Qualify useful ordinary Tracking with an unknown-future qualifier outside the
calendar-derived guard. In a leap-capable regime, unknown coverage protects every
possible month-end, not just June and December; covered continuous UTC needs no
recurring legacy guard. Post-guard observations must be fresh, initiated after the
guard and wholly within one compatible segment, with unchanged quorum/broker
limits. Historical TAI−UTC knowledge, future scheduling and fresh NTP UTC evidence
are distinct; a missing current conversion does not by itself prohibit tracking.

### Floor construction and supersession

Identify the exact uncertainty quantity, fault/delay assumptions, model version and supporting source evidence used to derive a saved floor. A regression standard error, confidence interval, root distance and conservative bound are not interchangeable. An authenticated, intact record can still contain a floor ahead of true UTC after previously accepted bad evidence; strict decoding and corruption recovery do not address that case.

R-02 must propose a narrowly authorized supersession procedure, its independent evidence and refusal behavior. Retain the old record and provenance, identify dependent models/associations to invalidate, and preserve broker correction budgets, consumed step authority and installer ownership. Until the procedure is approved and qualified, contradictions block the dependent automatic recovery; deleting all security state is not an accepted remedy. Bring material authority choices through NV-DEC-001 rather than inferring approval from this requirement.

### Votes, prediction and fresh agreement

R-02/R-04 must freeze a decision table naming the majority denominator, configured minimum, operator/endpoint grouping, timeout and statistical-rejection effects, accepted-observation age and uncertainty growth, separately for routine selection and exceptional recovery. Include four fresh groups, two unavailable groups, duplicate operators, a 2–2 split and one fresh source plus usable historical predictors. State the assumed faulty-group/path model and availability tradeoff; two malicious agreeing groups with two honest groups blocked does not disprove a one-fault guarantee.

[Chrony's FAQ §3.7](https://chrony-project.org/faq.html#_an_unreachable_source_is_selected) explains that an unreachable predictor may remain selected for hours. Its usefulness for prediction is not fresh independent corroboration or exceptional-correction authority. KE completion, cookie replacement, NAK and reachability changes must not refresh the last accepted time-observation timestamp. Preserve useful history without claiming renewed agreement.

### Required controlled cases

| Case family | Expected property |
| --- | --- |
| Local date ±100 years; true date inside/outside supported interval; 2036 rollover; era boundaries | Correct independent interpretation when supported; explicit rejection when ambiguous or out of range |
| Full date interval spans a leap event or the 20 May 2027 rule boundary; valid event-free hypotheses as control | No convenient midpoint/host-date regime; ambiguous samples have no clock authority while independent acquisition continues. Identical admissible interpretations across the rule boundary proceed without an artificial clock impulse |
| Unknown future leap coverage, expired source metadata and all-LI=0 unsmeared sources; covered history and fresh same-segment observations | Historical knowledge survives metadata expiry; useful Tracking continues outside bounded possible-month-end guards with honest coverage status and fresh post-guard quorum |
| A demonstrably false LI=1/2 claim beside healthy peers; later consistent observation | Exclude the contradictory observation, preserve healthy votes and fresh re-entry; no dynamic leap schedule, indefinite guard renewal or lowered minimum |
| No trusted floor, stale build, corrupted state, restored old snapshot | No permissive default or unearned date trust |
| Intact future floor from previously accepted bad evidence; correct independent sources return | Execute only the approved supersession/refusal path; retain provenance and consumed broker authority |
| One malicious NTS source, duplicate operator aliases, contradictory intervals, loss of quorum | No duplicate votes, no silent minimum reduction, no unauthenticated substitute |
| Historical predictors remain usable while fresh corroboration is lost | Selection, reported agreement and exceptional correction follow their distinct frozen eligibility rules |
| One/two bad helpers, disagreement, timeout, replay, no NTS completion | Hints cannot change Windows time or enter ordinary selection |
| Expired/not-yet-valid or revoked certificates; compromised old key made valid at the hinted date | Normal identity/signature policy retained; residual bootstrap attack represented honestly |
| Repeated small shifts, frequency abuse, sleep/resume, service crashes, acknowledgment loss | Real effects consume persistent authority; no repeated step or cleanup that changes a successor's clock policy |
| Large runtime changes with representative time-sensitive applications | Measured consequences and a reasoned proposal for additional approval boundaries |

Begin with deterministic simulation and controlled protocol fixtures. Windows execution requires an owned disposable VM, bounded experiment, independent observer and tested restoration. Public time services are not stress-test targets.

## 5. R-03 — conservative plain-NTP policy

The [5 November preparation matrix](../tasks/windows11-integration.md#431-clock-correction-limits-preparation--5-november-2026)
adds actual combined-rate accounting, including stable compensation, holdover
assumptions and conditional recovery exposure. Compare useful drift/recovery with
malicious repeated movement; upstream constants remain evidence, not adopted limits.

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

Produce one admission record per selected provider: official identity and ports, operator/reference dependencies, geographic suitability, permission/terms and date, leap/smear policy and its operator/configuration provenance, separately recorded announcement behavior, DNS/delegated endpoint rules, normal/burst/KoD limits, TLS/AEAD/Windows-verifier behavior, observation environment and open conditions. V1 requires admitted unsmeared-UTC provenance; unknown or smeared scale is ineligible, but an unsmeared provider's absent/unknown announcement behavior is not a scale failure. NTS, LI=0 and agreement alone do not prove scale. Apply bounded backoff and provider restrictions. Do not freeze IP addresses or treat two rotating Pool results as assured independent operators.

## 7. R-05 — Windows trust with a wrong system date

The future [Roughtime handoff](plans/time-assurance-integration.md#9-windows-gate-and-explicit-decision-alternatives)
must qualify this same OPEN-8 boundary. TA-W07 owns its coordinated Windows
experiment when the artifacts/environment are admitted; successful Roughtime or
Khronos results cannot bypass the required verifier/revocation policy. Distinguish
certificate-time incompatibility from other failures; no date-search loop may
make an unrelated trust/name/revocation error disappear. Qualify isolated fresh
verification/session state for both bootstrap bases. A preliminary
global clock step remains a separate, unselected authority alternative.

Qualify the exact Rust TLS provider, verifier, native dependencies and selected Windows service context. A provisional time argument does not establish that every Windows trust/revocation operation uses that time.

Document trust-store ownership, enterprise policy and explicit distrust, chain/hostname/EKU/algorithm checks, known-revoked versus unknown status, offline semantics, validity at candidate time, Windows API date range, resumption isolation and cached-association policy. Trace direct and delegated DNS/AIA/CRL/OCSP/network work, redirects and native cancellation/resource ownership. Preserve the existing conditional ASVS retrieval analysis.

### Association and full-authentication lifetime

Track separately the latest accepted time observation, cookie receipt, C2S/S2C key establishment, original full certificate-authenticated handshake and applicable trust-policy revision. Packet/cookie renewal and resumed TLS cannot silently renew full-authentication age. Specify bounded association/key lifetimes and jittered full reauthentication, including when resumption is forbidden, age measurement across restart/suspend/wrong UTC, and refusal or explicit recovery when age is unknown. Define source/algorithm removal, known-compromise and detected trust-store/policy invalidation, the detection owner and maximum reaction delay. Detection is not instantaneous merely because the policy has a revision.

[RFC 8915 §8.2](https://www.rfc-editor.org/rfc/rfc8915.html#section-8.2) describes impersonation using a compromised cookie-encryption key, including an interception case not remedied solely by genuine-server rotation. [RFC 8446 §4.6.1](https://www.rfc-editor.org/rfc/rfc8446.html#section-4.6.1) recommends a cumulative lifetime bound for resumed key material. Conversely, [RFC 8915 §8.5](https://www.rfc-editor.org/rfc/rfc8915.html#section-8.5) says original certificate expiry alone normally need not trigger KE; do not equate certificate `notAfter` with association expiry. Chrony 4.9's [manual](https://chrony-project.org/doc/4.9/chrony.conf.html#ntsrefresh) and [pinned default](https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/conf.c#L264) give `ntsrefresh` as 2,419,200 seconds (four weeks). Record it in the port ledger as comparison evidence, not an adopted NevroTick constant.

CHECK-8/9/15 cases must keep protected UDP responses arriving while full reauthentication fails, policy changes or age becomes unknown. Verify the selected expiry/refusal behavior, genuine accepted-neighbor recovery, preserved observation timestamps and no plain-NTP downgrade. OPEN-8/R-05 owns the policy proposal; dependent cached-association support remains gated until its choice and actual provider behavior qualify.

Test wrong name, expired/future certificates, revoked intermediates, unavailable responders, untrusted roots and extreme date conversions with valid neighboring controls. Reconcile OPEN-8's previously pinned source lead against the actual selected version. If the necessary policy cannot be enforced, report the blocked path and alternatives rather than disabling checks. A residual-risk change requires an OD-04 decision card.

<a id="8-r-06--signing-eligibility-and-initial-openpgp-releases"></a>

## 8. R-06 — signing eligibility and complete artifact coverage

### Accepted owner policy to preserve

**Current direction:** prepare the complete hardened MSIX, including services, for
David's Store submission attempt. Follow [plan §7](plans/windows-msix-store.md#7-bounded-continuation-and-existing-acceptance-owners):
source/design and local test-signed mechanics precede owner-controlled submission;
actual Store-delivered trust/CIG and lifecycle evidence precede broad release.
Preparing the candidate requires no prior Store acceptance or signature. R-06
supplies package/publisher identity, approved-release/helper binding, exact
dependency/catalog inventory, wrong-clock/offline trust conditions and truthful
capability justification to the existing W11 task; do not create another survey.

The [release policy](release-policy.md) now supplies the source-based operational
draft for stable support, backports, withdrawal, compatible corrective releases
and compromised publication authority. Its support baseline remains proposed;
account readiness, real access/recovery rehearsals and installed-state/native
evidence remain open. Continue through
[W11 task §4.21](../tasks/windows11-integration.md#421-release-operations-integration--4-november-2026)
and OPEN-11/14 instead of repeating the release survey. This common operational
work applies to the selected Store route, including required inner-file signing
capability; it is not limited to a later MSI route.

**8 November owner amendment:** [OD-05 §7.7](nevrotick-owner-decisions.md#77-sign-every-supported-shipped-artifact--8-november-2026)
requires signatures on the package and every supported shipped artifact,
including nonbinary files. The unsigned-MSI/OpenPGP permission is superseded.
Use the [canonical format/inventory procedure](windows-time-signing-guide.md#41-required-signatures-for-the-shipped-inventory).
SignPath Foundation's publisher display remains acceptable; its eligibility may
matter for inner files on MSIX as well as a later selected MSI. Missing production
capability blocks release, not independent source preparation. **David Osipov**
approves releases; research supplies no submission/signing/publication authority.

The later [OD-05 §7.8](nevrotick-owner-decisions.md#78-signpath-and-interim-self-signed-msi--8-november-2026)
accepts interim self-signed MSI if MSI delivery is selected. Use
[signing guide §7.1](windows-time-signing-guide.md#71-interim-self-signed-msi-profile)
to resolve approved-key custody, controlled-verifier versus clean-machine trust,
the existing helper/loader admission dependencies and later signer migration.
Missing public CA trust alone is accepted for this named profile; missing
required signatures, invalid identity or another required-control incompatibility
is not waived. No consumer certificate import or new custom verifier is adopted.

### Research questions

1. How do the provider's current terms treat a Rust port of Chrony, third-party/native payloads, the first-release condition and proprietary NevroTick brand assets? The current [SignPath conditions](https://signpath.org/terms.html) are labelled Draft. Give supported interpretations and questions for later authorized clarification; do not assert approval or rejection or change the artwork licence.
2. Which actual shipped formats require individual signatures, which expected project/upstream issuer satisfies each row, and what supported production provider/tool profile can create and verify them? Include nested/stream PE, existing signable scripts/data and packages; keep unavailability distinct from true format non-applicability. Do not introduce new scripting or data envelopes to create signing work.
3. Does the actual Store-delivered artifact preserve all required inner signatures and admitted component identities? Qualify package/catalog trust and embedded/per-format signatures independently, with missing, wrong-signer, modified and healthy controls. A generic Store signature or catalogue count cannot answer this.
4. What minimum signing/submission custody, expiry/revocation, access-removal and rotation process can David operate for the actual signer roles? Preserve approved upstream signatures and qualify wrong-clock/offline verification. Prepare the process without generating/exposing keys or reusing a personal Git commit key.
5. How does inside-out signing bind approved source/build, final metadata, exact signed members, submitted/delivered package and protected installed paths to maintenance and anti-downgrade policy? If MSI is later selected, qualify its actual cabinet/stream/custom-action and lifecycle relationships under the same signature requirement.

Deliver the next bounded W11-W10/S6.4 CHECK-18/23/26 evidence card for the selected
route, with signature coverage, correct signer/bytes, tamper, missing/removed
signatures, stale/mismatched manifest, nested-payload substitution and valid
controls. Missing required signatures/provider capability remains release-blocking;
the historical OpenPGP-only profile cannot close it. Runtime DLL protections,
ACLs, native mitigations and licence obligations remain mandatory.

Primary references and their limits are maintained in the [signing guide](windows-time-signing-guide.md#9-sources-and-related-documents), including Microsoft per-format/Store guidance and SignPath's reference. Unsigned/test-signed laboratory artifacts do not qualify production signatures.

Do not add a second application package, bespoke privileged updater, provider
application, paid subscription or OS-protection bypass as a side effect of this
research. Store-managed updates need the selected complete lifecycle qualification.

## 9. R-07 and R-08 — exception decisions and remaining platform work

For OPEN-7/13, [W11 §4.31](../tasks/windows11-integration.md#431-clock-correction-limits-preparation--5-november-2026)
assigns precise-API conversion/quantization, active-delivery/power mapping and
actual stop/native-work/recovery evidence to existing owners. Retain censored
and failed recovery observations; measured maxima do not establish hard expiry.

For every relevant ASVS requirement, separate literal applicability, native adaptation, implementation, observation and unresolved exception. Bring David individual evidence-backed decisions about hardware-backed isolation, authentication/revocation semantics, in-use secrets, logging, delegated retrieval or plain-NTP assurance. No blanket L3 acceptance has been granted.

Continue the existing phase-specific investigations for account/token/DACL separation, race-safe IPC, real Windows rate/step/leap semantics, recovery after process death, exact supported builds, installer-tool terms, native resource budgets and complete MSIX feasibility. Propose a decision only when an actual tradeoff remains after authorized investigation. Keep full UI qualification deferred at the UI-M0 reentry triggers; completing a fixture prototype closes no later accessibility gate.

For OPEN-7, apply the selected [Windows leap contract](plans/leap-second-policy.md#6-windows-contract-and-limits)
through W11-W02/S0.4/P4. The open evidence now concerns supported-build process
presentation/conversions, actual kernel knowledge and event behavior, finite
guard/cancellation/recovery bounds and clock ownership. Do not reopen an extra
event scheduler or change global OS leap state to make a test pass. Observe
OS-handled, absent and unexpected events of both signs, including no extra
correction when the measured residual is zero, lost acknowledgements and fresh
reacquisition. Timer expiry or kernel event execution is not an acknowledged
broker effect. No reviewed API documentation or portable fixture closes this
native gate.

For native mitigations, follow [architecture section 15.4](windows-time-architecture.md#154-per-process-mitigation-profiles-and-startup-admission) and [hardening section 6.4](windows-time-development-hardening.md#64-per-process-mitigation-qualification). Resolve the actual SCM child-policy installation/readback/denial path, required startup failure and ownership-aware recovery, exact native-provider compatibility, and standalone-helper versus MSI DLL hosting. Strict CFG and CET strengthening remain separately qualified candidates. Do not replace the two-service topology, introduce a launcher or silently modify host policy to close an evidence gap.

## 10. Required output and completion criteria

Return:

1. A finding table mapping each research question to evidence, limitations, recommendation, affected gate and owner.
2. Source pins and current provider policies; disclose contradictory documentation and any unavailable runtime environment.
3. Proposed exact changes to the canonical docs, preserving existing identities and amendment history.
4. Concrete OD-04 decision cards only for choices that need David; report provider approval separately.
5. A bounded next implementation/experiment stage with independent valid/hostile expectations, environment, limits, cleanup and observable exit.

An independent reviewer should assess the changed proposal. Review agreement accepts no residual risk and proves no native test. Leave unavailable tests incomplete. Continue unaffected work while a dependent decision remains open.

**Success:** the next implementer knows the owner of every changed rule, the exact unresolved fact, the evidence required to resolve it, and the effect that must remain disabled until that gate passes. Do not declare the whole product ready merely because this research is complete.
