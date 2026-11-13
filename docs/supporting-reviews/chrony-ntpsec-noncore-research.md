# Chrony and NTPsec: non-core research and qualification

**Document:** NV-NCR-001, revision 1.0

**Date:** 13 November 2026, Asia/Tbilisi

**Research/source baseline:** NevroTick 5dd75dfa0445ae56171d551d8d44bac09d6e8ccb

**Authority:** [OD-18](../nevrotick-owner-decisions.md#22-od-18--literature-derived-qualification-before-v1),
[LIT-0](../../ROADMAP.md#lit-0--literature-derived-qualification-before-v1)
and the existing phase/task owners below.

## 1. Decision and scope

Add small, meaningful regression tests against existing consumers before V1.
Prepare the other relevant cases early, with independent expected results,
healthy neighbors and forbidden effects. Execute reached V1 cases before their
existing phase/release exit. A case for an absent future mechanism remains a
specified case until that actual consumer exists at its selected stage.

This review supplies evidence and detailed handoff, not another protocol or
clock-policy specification. It adopts no paper's algorithm, production constant,
TCA service, runtime catalogue updater or accuracy claim. New implementations
with substantial prerequisites follow section 6 and their existing owners.

Preserve actual V1 → MATH-C01 → MT-C01 → KHR-C01 → GEO-C01 → Android and the
separate RGT-0 → CHR-0 sequence. Existing V1 correctness duties remain before
release; existing native/fuzz/storage/UI holds and the post-V1 formal deferral
retain their scopes. An ordinary regression test is not a new estimator
experiment or permission to exercise Windows clock effects.

## 2. Evidence and priority

Prioritize restart/authority, acquisition provenance, source identity/lifetime
and NTS resource behavior. Publisher monitoring/privacy evidence supports
curation. The ordering is engineering judgment; no matched NevroTick experiment
establishes which intervention would produce the largest benefit.

| Source | Actual scope and useful transfer | Evidence limit |
| --- | --- | --- |
| [Langer et al., 2020][L1] | Names NTPd, NTPsec, chrony and NTP-O; local latency/capture and interleaving controls | Compensation experiments use modified NTP-O; mixed hardware worsens total offset. Exact daemon configurations remain undisclosed. |
| [Mahlous, 2024][L2] | chrony 4.4/NTPsec 1.2.2 among tested clients; restart and initial-adjustment exceptions | Specific VM, single-source and spoofing conditions; authentication is not protection from all authenticated delay. |
| [SIDN client report, 2025][L3] | chrony 4.5/NTPsec 1.2.2 among eight clients; cold/hot start, false time, KoD, unavailability and era cases | Technical report with version inconsistencies; use discriminating scenarios, not a universal client ranking. |
| [AmpFuzz, 2022][L4] | chrony 4.0/NTPsec 1.2.0 server/control surfaces; behavioral amplification fuzzing | Low direct priority for a client-only product; reflection and amplification differ. |
| [EtherTime, 2025][L5] | chrony and PTP on embedded Linux under contention/faults | Workload design evidence, not a portable Windows tuning recipe or new estimator. |
| [Tychron chapter, 2026][L6] | Abstract claims Chrony/NTPsec comparison and combined discovery/scoring/fusion/reselection | Full methods not inspected. No component gain, reproduction or adoption is established. |
| [On Borrowed Time, 2026][L7] | Aliases, shared administration/upstreams, server lifetime and residual requests after removal | Operational independence taxonomy is not proof of independent adversarial control. |
| [IMC ecosystem study, 2025][L8] and [later IETF presentation][S1] | Upstream/source risks; later NTxMon hostname/identity and assignment proposals | Original full paper not inspected. Presentation, simulation and prototype/service claims remain separately attributed. |
| [Shark/Watchdog, 2023][L9] | Pool-monitor reference, anomaly and aggregation weaknesses | Historical system; current [monitoring v4][S2] must be considered before claiming the same weakness persists. |
| [Cleaning the NTP Pool, 2026 manuscript][L10] | Publisher-side evidence of source-associated IPv6 scanning | Inspected v3 describes integration discussion/work; completed deployment is not established by that text. |
| [BigTime, author-posted 2026 copy][L11] | Provider concentration and sources actually delivered to a client | No client-accuracy comparison; publication registration remained unconfirmed in this review. |
| [Leclerc/Bartos, 2022][L12] | Instrumented CFNTS startup/steady-state costs | Not a chrony/NTPsec comparison; no cookie replenishment in the tested program and synthetic plain-NTP subtraction limit its ratios. |
| [NTS covert channels, 2022][L13] | Fedora/chrony testbed and controlled/compromised endpoints | Proposed TLS-intercepting warden is not selected for NevroTick. |
| [TPKI, 2025][L14] | Two-page student-workshop paper; authors report chrony 4.2 client/server prototype | Conditional model and incomplete protocol; no NTPsec evaluation or verified public artifact. See section 4. |

The full Langer and TPKI papers were inspected, including the relevant figures
and tables. Full-text access does not resolve undisclosed methods. The IMC article
and the distinct TPKI paper must not be merged into one evidence record.

## 3. Langer: qualify acquisition without importing a correction

Langer reports about 62 to 1–2 microseconds of systematic offset on matched
endpoints with compensation/interleaving, but 19 to 61 microseconds on the mixed
NTS setup. Removing one latency component can expose another previously
canceling bias. These are reported experimental offsets, not NevroTick gains,
tail bounds or demonstrated absolute UTC accuracy. The described comparison is
direct Ethernet; its Internet discussion is not a documented WAN benchmark. [L1]

At the reviewed source baseline, the experimental
[association ledger](../../crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs)
prepares/seals the packet before the
[datagram adapter](../../crates/nevrotick-engine/src/nts_ke_experiment/datagram.rs)
takes its local send capture. Receive-return capture precedes
[response authentication](../../crates/nevrotick-engine/src/nts_ke_experiment/response.rs).
The wire transmit field is a correlation token, separate from the computational
timestamp. Subtracting full client crypto time from those later/earlier capture
boundaries would introduce an unsupported correction.

The [measurement consumer](../../crates/nevrotick-engine/src/measurement.rs)
must preserve its explicit clock-domain, elapsed-time and uncertainty contracts.
Test those actual boundaries using independent timestamp facts. Neither
artificially moving work outside the measured interval nor changing a correlation
token may manufacture a timing correction. Source ordering alone proves no
native timestamp attribution, physical delay bound or synchronization benefit.

The [W11-W01 task](../../tasks/windows11-integration.md#w11-w01-preserve-packet-timestamp-provenance)
already owns capture labels, socket/interface/generation binding, conversion,
bounded TX retrieval and qualified fallback. Its later comparison should include
matched and mixed endpoints, application/software/supported hardware timing,
asymmetric peer modes, idle/load transitions and missing/saturated/late metadata.
Measure signed bias, tails, uncertainty and cost separately. A more faithful
timestamp can coexist with worse total offset; retain that adverse control.

Use an independent client-server reference for relative error. An absolute UTC
claim additionally needs traceability and an uncertainty budget. Do not adopt
busy-poll, maximum-frequency settings, an empirical correction table or
interleaving merely from this paper; [RFC 9769][S3] and actual peer support govern
any later interleaved implementation.

## 4. TPKI: authenticated intervals and unresolved freshness

TPKI introduces a Time Certificate Authority that registers/assigns servers and
issues short-lived signed UTC intervals bound to server addresses/keys.
Servers relay a certificate and sign NTP replies; clients filter samples before
ordinary NTP processing. A separately trusted coarse interval could constrain
large malicious shifts while ordinary measurements provide precision. Relaying
credentials could distribute delivery load. These are useful architectural ideas,
not measured benefits for NevroTick. [L14, section 2]

The paper's example is theoretical: \(0.9^{60}\approx0.18\%\), with reciprocal
about 556.48. It assumes that one accurate queried server defeats the defined
attack. That premise must become one sufficiently fresh, authenticated, admitted
honest response before the deadline; the TCA must remain trustworthy and actual
sampling must match the assumed population. Under that premise an upper bound
on attack probability follows; equality requires further success assumptions.
GeoDNS assignment is not reconciled with uniform sampling, and attack magnitude,
certificate width and a lifetime security horizon are not specified. [L14, §3]

The newest received certificate can still be historical. The paper recognizes
expiry circularity but does not fully specify first boot, persisted bounds,
restart/rollback, offline aging or withheld fresh responses. A new signature over
a reply containing an old certificate does not itself renew the time assertion.
These are missing protocol details, not demonstrated implementation defects.

Table 1 reports two Pool servers serving 52M and 21M clients over 24 hours with
scores of 20. It does not establish that those clients validated certificates,
were unique devices or received the modeled security guarantee. The WAN timing
plot lacks enough ground-truth/configuration/statistical detail for a portable
accuracy claim. No public code/specification artifact was verified in the bounded
review; this does not establish that none exists. [L14, Fig.2 and Table 1]

The selected [time-assurance design](../plans/time-assurance-integration.md)
already distinguishes freshness, interval width, ancestry, source independence
and correction authority. [RFC 10049][S4], Experimental, October 2026, supplies
the current Roughtime specification; section 6 discusses NTP filtering and NTS
bootstrap. That makes the selected Roughtime→NTS route an appropriate comparator,
not a proven universal winner over possible TPKI designs.

## 5. Pre-V1 cases under existing owners

These rows refine existing acceptance. They are not records of executed tests.
Before adding a case, identify current coverage and the actual receiving
function; retain a useful healthy neighbor. Record selected tests and results in
the owning task. Do not treat two implementations of the same formula as
independent truth or add a fake future protocol merely to run a scenario.

| Case and phase owner | Independent expected result | Current execution boundary |
| --- | --- | --- |
| Capture/domain errors — [engine acquisition](../../tasks/engine-transport-pilot.md), W11-W01; CHECK-6/7/11/25/27 | Measurement follows the admitted capture/clock facts, with correct sign, range and uncertainty; mismatched frames/refused metadata cannot create a valid sample. | Add missing tests to current measurement consumers now; actual OS timestamp attribution remains the adapter/native owner's work. |
| Forged catalogue authority — [catalogue R/C task](../../tasks/server-catalogue-and-statistics.md); DATA-01/02/03/24 | Recomputed meaning rejects injected approval/independence/time-scale authority; a legitimate unapproved candidate remains usable as review evidence. | Current offline collect/diff consumers are testable. This does not test production source approval or cryptographic freshness. |
| Restart and consumed authority — P4 broker/recovery; CHECK-9/10/12/15 | Restart/state loss cannot silently replenish large-correction permission or erase mandatory limits; legitimate initialized/recovery controls remain possible. | Specify before V1 and execute against the actual V1 policy consumer before its exit; packet fixtures alone do not qualify a clock effect. |
| Polling, KoD and recovery — S0.1/P2/engine; CHECK-6/8/9/17/21/27 | Correlation/authentication and next-send guards enforce bounded backoff/re-entry; unavailable peers cause neither permanent abandonment nor burst retries. | Reuse the [ntp_usg handoff](../../tasks/s0.1-protocol-comparison.md#ntp_usg-comparison-handoff--11-november-2026); deduplicate its existing RATE/continuous-caller cases. |
| NTS inventory and costs — S2.2 association/receiver, AC-04; CHECK-6/8/9/17/21/25/26/27 | Repeated authenticated exchanges preserve cookie/permit/count/byte budgets and distinguish startup, rekey and steady state. | Preserve accepted cookie policies and current one-shot limits; continuous behavior must reach its eventual real caller. |
| Identity, retirement and observer failure — catalogue/curation; DATA-01/03/08/10/17/24 | Aliases do not create independent authority; a failed source snapshot is not mass withdrawal; endpoint/key changes require revalidation. Observer/path failure is not automatically source misconduct. | Current R input/diff checks first; production C admission, runtime replacement and publisher measurements retain their owners. |

### TPKI arrangements within the existing TA cases

Prepare these concrete input/expected-result refinements before V1. The
[TA task](../../tasks/time-assurance-integration.md#4-independent-acceptance-cases)
retains future runtime execution at KHR/RGT; early shared-component checks cannot
close those future gates.

| Existing case | Hostile/failure arrangement and healthy control | Required observation |
| --- | --- | --- |
| TA-C01 | Many endpoints relay one issuer/time premise; control has independently evidenced bases | Preserve separate measurements while preventing duplicated issuer authority. |
| TA-C06/10 | Correctly signed historical credentials after restart/restore, including a fresh reply carrying an old credential; control has a current independent trusted bound | Ordering and packet freshness alone cannot establish current certificate freshness or reset retained limits. |
| TA-C07 | Differently biased samples fit one coarse interval; control has appropriately narrow justified evidence | Interval admission does not manufacture fine precision or midpoint-only assurance. |
| TA-C08 | Honest server selected but its response withheld before the deadline; healthy complete response set | Credit actual admitted evidence, not sent queries; preserve source minima and explicit insufficiency. |
| TA-C04/10/13/14 | Changed issuer/key/endpoint/generation, missing renewal and signed contradiction; ordinary valid renewal | Revalidate current authority and distinguish stale, insufficient and contradictory evidence within common budgets. |

## 6. Longer work and reconsideration gates

### 6.1 Timestamp adapters and measured acquisition benefit

**Owner/stage:** W11-W01 and the existing engine S2.1/P2 card. Required V1
provenance remains at its current phase; optional adapter/hardware adoption is
conditional. Freeze exact API/binding, capture mode, clock mapping, interface,
conversion validity, retrieval bounds and fallback before implementation.

Compare the same qualified downstream estimator using controlled peers and
independent timing truth. Retain matched/mixed endpoints and both favorable and
adverse bias cases from section 3. Success requires correct attribution and
bounded lifecycle first; any claimed precision/cost gain needs repeatable matched
evidence. Unsupported mapping, excluded truth, worse relevant tails or excessive
cost must produce a scoped refusal, further bounded investigation or deferral.
Stop after the declared modes/cases and resource budget; no general API survey.

### 6.2 Continuous NTS, polling and resource qualification

**Owner/stage:** existing S0.1/S2.2/P2 engine and protocol tasks. Preserve selected
providers, zero-replacement/reissuance policy, aggregate limits and current
authorization. Prerequisites are actual continuous callers, stage-specific
instrumentation and controlled credential/peer fixtures.

Measure acquisition, crypto, scheduling and synchronization effects separately
under startup, steady state, rekey, loss, KoD, repeated errors and cancellation.
Compare equally qualified paths and hold accuracy, recovery and traffic budgets
together. Improvement in one average that worsens authority, tails or bounded
recovery fails the proposed optimization. Existing V1 requirements are not
deferred merely because these implementations are substantial.

### 6.3 Catalogue and publisher evidence

**Owner/stage:** existing DATA/R/C and source-admission work, with release/trust
owners. Complete operator/dependency evidence, use permission, withdrawal and
approval binding before claiming an approved roster. Separate source behavior
from observer/path faults and keep privacy-attribution evidence dated and scoped.

NTxMon-style assignment/weights or a live updater require a verified artifact,
usable authorized service, identity/endpoint rules, rollback and route/resource
contracts, then a matched benefit comparison. Retain release-bound reviewed data
unless a later decision changes it. Simulated-client availability ratios and
unverified deployment claims cannot select a runtime service or automatic
exclusion rule. Explicitly defer/reject when those prerequisites are absent.

### 6.4 Conditional TPKI protocol evaluation

**Status:** deferred research candidate, not selected implementation or a new
roadmap predecessor. Reopen with a complete wire/signature/freshness specification,
source/licence, usable trusted issuer/server infrastructure and a concrete
advantage to test against the selected Roughtime→NTS design.

Specify certificate width, reference instant, issuer error, delay/aging,
request binding, first-boot/restart/rollback state, key rotation/revocation,
authority loss and the final estimate's interval constraint. Preserve the
actual selected/responding/admitted population and common issuer dependencies.
The paper's 60 queries are not a production constant or permission for extra
active sources; any sequential survey needs its own finite cumulative budget.

Start offline with independent valid/stale/contradictory evidence and a declared
forbidden error magnitude. Accept a bounded adoption proposal only if protection,
availability and resource behavior improve under matched assumptions without
weakening existing trust or routing. Otherwise record DEFER or REJECT with the
specific missing evidence. Native/product adoption remains separately qualified.

### 6.5 Mathematical experiments

New estimator, noise/temperature model and uncertainty comparisons remain
[MATH-0](../../ROADMAP.md#math-0--post-v1-mathematical-estimation-experiments)
after actual V1 release. Existing-correctness regressions and early case
specifications neither run nor complete MATH-C01. Tychron remains a reproduction
lead until full methods and fair component comparisons are available.

## 7. Evidence and next-agent record

Use the [testing contract](../../.agents/skills/testing/CONTRACT.md) and existing
owner's configured route. Record source plus dirty-byte identity, case IDs,
independent oracle, actual consumer, tool/target/environment, selected/executed
counts, failures, healthy controls and remaining gaps. Use synthetic local peers
and original fixtures; this handoff authorizes no public-server probes.

Documentation/case preparation, host execution, native Windows, package and
release qualification remain separate. A blocked check stays incomplete.
Required V1 evidence cannot be moved to a later phase to obtain a pass; future
optional runtime cases do not become invented V1 blockers. Independent review
must check this distinction and preserve the existing canonical CHECK owners.

The coordinator records actual test results in the existing engine/catalogue/W11
tasks and reconciles them with the final candidate before publication. This
research document itself asserts no test pass, security percentage, native result
or measured NevroTick improvement.

## 8. Public primary references

The table's links identify the reviewed papers or primary author copies.
Related standards and presentation versions remain separate evidence.

- [L1] Langer et al., *Analysis and Compensation of Latencies in NTS-secured NTP Time Synchronization*, IFCS-ISAF 2020.
- [L2] Mahlous, *Quantitative Risk Analysis of Network Time Protocol (NTP) Spoofing Attacks*, IEEE Access 2024.
- [L3] Konjerla et al., *Are NTP Clients Always Right? Evaluating NTP Clients under Normal and Attack Scenarios*, SIDN technical report, October 2025.
- [L4] Krupp et al., *AmpFuzz: Fuzzing for Amplification DDoS Vulnerabilities*, USENIX Security 2022.
- [L5] Bode et al., *EtherTime: Cross-Vendor Evaluation of PTP/NTP on Ethernet-Based COTS Embedded Platforms*, RTCSA 2025.
- [L6] Liu et al., *Improving NTP Time Synchronization Accuracy for NTS Enhancement*, MobiQuitous proceedings, online 2026.
- [L7] Beverly and Rye, *On Borrowed Time: Measurement-Informed Understanding of the NTP Pool's Robustness to Monopoly Attacks*, NDSS 2026.
- [L8] Huang et al., *Measuring the Time Source Vulnerabilities in the NTP Ecosystem*, IMC 2025; later NTxMon material is [S1].
- [L9] Kwon et al., *Did the Shark Eat the Watchdog in the NTP Pool? Deceiving the NTP Pool's Monitoring System*, USENIX Security 2023.
- [L10] Rye and Beverly, *Cleaning the NTP Pool: Detecting and Mitigating NTP-Sourced IPv6 Scanning*, version 3 manuscript, September 2026.
- [L11] Huppert et al., *BigTime — Characterizing Large Time Service Providers*, author-posted CoNEXT 2026 copy.
- [L12] Leclerc and Bartos, *Experimental Analysis of the Performance and Scalability of Network Time Security for the Network Time Protocol*, ISPCS 2022.
- [L13] Lamshöft and Dittmann, *Covert Channels in Network Time Security*, IH&MMSec 2022.
- [L14] Huang et al., *Preventing Time-Shifting Attacks by Powerful Attackers with Time Public Key Infrastructure*, CoNEXT Student Workshop 2025, pp.1–2.

[L1]: https://doi.org/10.1109/IFCS-ISAF41089.2020.9234871
[L2]: https://doi.org/10.1109/ACCESS.2024.3493759
[L3]: https://www.sidnlabs.nl/downloads/7M1bz1otGu0D7hqr0hRT2c/3bc09536c3bc87f63b80d66944abc1d9/ntp-clients-tech_report-20251016.pdf
[L4]: https://www.usenix.org/system/files/sec22-krupp.pdf
[L5]: https://doi.org/10.1109/RTCSA66114.2025.00026
[L6]: https://link.springer.com/chapter/10.1007/978-3-032-22500-9_18
[L7]: https://www.ndss-symposium.org/wp-content/uploads/2026-f541-paper.pdf
[L8]: https://doi.org/10.1145/3730567.3764478
[L9]: https://www.usenix.org/system/files/usenixsecurity23-kwon.pdf
[L10]: https://arxiv.org/pdf/2607.21903v3
[L11]: https://www.sidnlabs.nl/downloads/400vgcNAZqVHWRRNGS02xL/79c0d3c436e6f850dd6b9e7e3336f621/ntp-big-time-conext26.pdf
[L12]: https://doi.org/10.1109/ISPCS55791.2022.9918378
[L13]: https://doi.org/10.1145/3531536.3532947
[L14]: https://doi.org/10.1145/3769700.3771698
[S1]: https://datatracker.ietf.org/meeting/125/materials/slides-125-maprg-measuring-the-time-source-vulnerabilities-in-the-ntp-ecosystem-05.pdf
[S2]: https://news.ntppool.org/2025/07/monitoring-v4/
[S3]: https://www.rfc-editor.org/rfc/rfc9769.html
[S4]: https://www.rfc-editor.org/rfc/rfc10049.html
