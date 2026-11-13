# NevroTick roadmap

**Updated:** 13 November 2026 (Asia/Tbilisi)

**Pre-V1 distribution requirement:** [WGT-0, WinGet capability](#wgt-0--winget-distribution-capability-before-v1), prepared and qualified within P6/P7 before V1; actual public catalog availability is tracked separately.

**Pre-V1 literature qualification:** [LIT-0](#lit-0--literature-derived-qualification-before-v1) adds small actual-consumer tests and prepares the reviewed cases before V1 under [OD-18](docs/nevrotick-owner-decisions.md#22-od-18--literature-derived-qualification-before-v1). Substantial implementation follows its existing phase and detailed evidence gates.

**First post-V1 TODO:** [MATH-0, mathematical estimation experiments](#math-0--post-v1-mathematical-estimation-experiments), immediately after V1 is actually released and before MT-0, under [OD-16 §20.1](docs/nevrotick-owner-decisions.md#201-mathematics-experiments-first--13-november-2026).

**Post-V1 order:** MATH-0 → [MT-0, multithreading](#mt-0--post-v1-multithreading-and-concurrent-acquisition) → [Khronos/Ananke](#khr-0--evaluate-independent-time-cross-checking) → [GEO-0, geography-assisted source ordering](#geo-0--evaluate-geography-assisted-source-ordering) → Android

**Later owner-selected sequence:** [Roughtime bootstrap for a badly wrong clock](#rgt-0--roughtime-bootstrap-for-a-badly-wrong-clock) → [independent replacement of Chrony-derived implementations](#chr-0--independently-replace-chrony-derived-implementations); future public licence undecided

**Android status:** planned; no Android implementation or handset qualification is established  
**Original Android research baseline:** [a54c04382cf5e5cb2ae641d7356ce67fe24bc1ab](https://github.com/DavidOsipov/NevroTick/commit/a54c04382cf5e5cb2ae641d7356ce67fe24bc1ab)

## 1. Direction and authority

Finish the current Windows development and acceptance plan, including
[WGT-0 WinGet capability](#wgt-0--winget-distribution-capability-before-v1) under
[OD-17](docs/nevrotick-owner-decisions.md#21-od-17--winget-capability-before-v1),
and [LIT-0's applicable qualification](#lit-0--literature-derived-qualification-before-v1),
then actually release V1. Under [OD-16 §20.1](docs/nevrotick-owner-decisions.md#201-mathematics-experiments-first--13-november-2026),
the first post-V1 job is [MATH-0](#math-0--post-v1-mathematical-estimation-experiments).
After its comparison and disposition, continue with
[MT-0](#mt-0--post-v1-multithreading-and-concurrent-acquisition),
then the [Khronos/Ananke evaluation](#khr-0--evaluate-independent-time-cross-checking),
then the [GEO-0 experiment and disposition](#geo-0--evaluate-geography-assisted-source-ordering)
requested on 5 November 2026. Then pursue the Android application
David requested: a local time server that redirects Android's existing NTP client
to it, without root or an always-active VPN interface.

Here, **redirecting the client means changing its configured NTP endpoint through
an explicitly authorized ADB setup operation**. It does not mean replacing Android
framework binaries, patching the system service, or transparently intercepting
arbitrary applications' packets. The Android system remains responsible for
applying time to its own clock.

| Workstream | Planning owner and status |
| --- | --- |
| Literature-derived qualification | [LIT-0](#lit-0--literature-derived-qualification-before-v1) and [NV-NCR-001](docs/supporting-reviews/chrony-ntpsec-noncore-research.md): implement small missing tests against existing consumers and prepare concrete cases before V1. Existing engine, W11, catalogue, broker and TA owners retain evidence; absent future protocols receive no simulated pass. Longer work remains explicitly staged or conditional. |
| Slint UI default | [OD-13](docs/nevrotick-owner-decisions.md#17-od-13--slint-as-the-default-ui-toolkit) selects Slint with Winit/software and Fluent-style widgets. The checked-in fixture now selects Slint; [S0.5](tasks/s0.5-ui-comparison.md) and the [current GUI handoff](tasks/slint-gui-handoff.md) own its source, graph/licence controls and candidate-specific evidence. The old Win32 reference is historical. Generated-code safety retains its exact graph/source scope; native Windows, production backend, process/package hardening and release qualification remain open. UI-M0 D1–D3 deferrals and the current development-plan order remain. |
| Windows NTP/NTS client | [Development execution plan](docs/plans/development-plan.md): current work, P0–P7, prerequisites and acceptance. [OD-14's matrix](tasks/windows11-integration.md#432-two-family-windows-qualification-matrix--5-november-2026) targets Windows 11 26H2 x64 and Server 2025 x64 Desktop Experience; qualification remains open. This roadmap neither duplicates nor reorders that plan. |
| Complete hardened MSIX and Microsoft Store | [Owner-selected plan](docs/plans/windows-msix-store.md), [architecture §19.4](docs/windows-time-architecture.md#194-future-complete-msix-gate) and [current W11-W10 assignment](tasks/windows11-integration.md#48-owner-selected-hardened-msix--3-november-2026): prepare one complete MSIX with explicit integrity enforcement, protected executable content, Store-compatible CIG and verified per-process protections. David will try Store submission with services and reconsider delivery if refused. Preparation needs no speculative preapproval; supported least-privilege provisioning, W32Time/recovery lifetime and actual Store-artifact qualification still gate release. Retain MSI as an alternative for a later decision. The [release policy](docs/release-policy.md) adds proposed support terms and operational withdrawal/recovery work at the same release gates; support approval and rehearsals remain open. Current Core priority, post-V1 order and execution holds remain. |
| WinGet distribution before V1 | [WGT-0](#wgt-0--winget-distribution-capability-before-v1), [NV-WGT-001](docs/plans/winget-distribution.md) and [W11-W10](tasks/windows11-integration.md#433-winget-distribution-before-v1--7-november-2026): implement and qualify the source-specific delivery capability within P6/P7. Preserve the selected Store MSIX and qualify `msstore` first; Community YAML requires an authorized trusted public installer. WGT-C01 readiness and WGT-C02 actual catalog availability are separate; no publication or native result is claimed. |
| Windows 11 native integration | [Microsoft source review](docs/plans/windows11-integration.md), [Learn MCP follow-up](docs/plans/windows11-integration-follow-up.md), [existing-phase workstream](docs/plans/development-plan.md#85-windows-11-integration-research-and-qualification) and [agent assignments](tasks/windows11-integration.md): timestamp/power/lifecycle refinements, DNS and service-network candidates, resource use, diagnostics and deferred UI integration. The follow-up adds concrete callback, COM, MSI permission, ETW and partial-replacement cases. Research is recorded; implementation/native evidence and conditional choices remain open at their existing phases. Current priorities and execution holds remain; this is no new predecessor to KHR-0. |
| Protected optional Windows storage | [Architecture §12.1.2](docs/windows-time-architecture.md#1212-protected-optional-storage-and-explicit-fallback), [storage plan](docs/plans/windows-protected-storage.md), completed [first](docs/plans/windows-protected-storage-upstream-lessons.md) and [second](docs/plans/windows-protected-storage-upstream-lessons-2.md) source reviews, and [agent task](tasks/windows-protected-storage.md): VBS/TPM/fresh-state fallback, truthful UI, bounded storage/adapter contracts and distinguishing checks. S2.3/P3/P5/P6 work; design freeze/native qualification remain open, experiments paused, current priority and Khronos ordering unchanged. |
| Mathematical estimation experiments | [MATH-0](#math-0--post-v1-mathematical-estimation-experiments): first TODO after actual V1 release, before MT-0. Compare paper-informed candidates against the released estimator and pinned Chrony processing with independent truth; MATH-C01 records evidence and disposition, not automatic production adoption. |
| Post-V1 multithreading | [MT-0](#mt-0--post-v1-multithreading-and-concurrent-acquisition) and [NV-MT-001](docs/plans/post-v1-multithreading.md): after MATH-C01 and before Khronos. Qualify independent acquisition, bounded native workers and any justified scheduler/capacity change; existing V1 correctness duties remain. |
| Khronos/Ananke reserve | [KHR-0](#khr-0--evaluate-independent-time-cross-checking): after V1 release and MT-C01, before GEO-0/Android; evaluate the owner-selected conditional initial-date reserve and coordinated-false-agreement risks. |
| Geography-assisted acquisition | [GEO-0](#geo-0--evaluate-geography-assisted-source-ordering): bounded comparison after KHR-C01; the [existing catalogue task](tasks/server-catalogue-and-statistics.md#deferred-geoip-experiment--after-khronos) owns GEO-C01. Evaluation and disposition precede Android; GeoIP adoption is optional. |
| Android local SNTP bridge | This document, stages AND-0–AND-5, after KHR-C01 and GEO-C01. Future work with device-specific feasibility gates. |
| Android-to-Windows time service | AND-6 below. Retains the original companion-to-NevroTick idea as a separately gated future server extension. |
| System-wide Windows DNS | [Planned DoQ feature](#5-planned-dns-over-quic-and-certificate-pinning) and [detailed DNS/cache design](docs/plans/system-dns-cache-design.md): DNS answers for the whole computer and its applications, DoQ-only upstream resolution with leaf/intermediate pinning, DNSSEC with hardware acceleration and multiple CPU cores, and an evaluated adaptive cache. Native integration, actual coverage and performance remain to be qualified. |
| Roughtime bootstrap | [RGT-0](#rgt-0--roughtime-bootstrap-for-a-badly-wrong-clock): obtain an authenticated approximate-time interval to bootstrap NTS when the local date is badly wrong; provider, trust-policy and Windows qualification remain future work. |
| Chrony-independent implementation | [CHR-0](#chr-0--independently-replace-chrony-derived-implementations): owner-selected future replacement stage immediately after Roughtime. Remove reliance on Chrony's licence for the replacement implementation; the future public licence remains open, and David will authorize Microsoft to use the resulting code in Windows within the rights established for it. |
| Optional SOCKS5 time acquisition | [SOCKS-0](#socks-0--optional-socks5-time-acquisition): deferred beyond the first Windows P0–P7 delivery under [OD-11](docs/nevrotick-owner-decisions.md#15-od-11--socks5-scope-and-deferral). Direct family/interface policy and successful-path reuse stay in the current plan. No new predecessor to KHR-0, Android or RGT-0 → CHR-0 is introduced. |

The [architecture](docs/windows-time-architecture.md),
[owner decisions](docs/nevrotick-owner-decisions.md) and
[documentation guide](docs/README.md) retain their existing authority. Windows v1
remains an NTP/NTS client; this roadmap does not add a Windows NTP/NTS listener to v1.
Current development follows the [next-task plan](docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent),
including its portable Core continuation, Windows execution stop and separate
fuzz no-retry hold. Roadmap research does not lift those restrictions.

At the reviewed baseline, portable Rust protocol/numerical fixtures are reuse
candidates, not a production Android or NTS engine. Preserve Chrony-derived
provenance and GPL-2.0-only obligations. Admit any Android/JNI, TLS, AEAD, runtime
and build dependencies separately. The Windows x64 CPU policy in OD-07 does not
apply to Android ABIs or impose x86 compiler flags on shared portable code.

### WGT-0 — WinGet distribution capability before V1

**Status:** required pre-V1 delivery capability by David's instruction of
7 November 2026, recorded in [OD-17](docs/nevrotick-owner-decisions.md#21-od-17--winget-capability-before-v1).
Research and planning are recorded; implementation, native qualification and
public availability remain open.

**Placement:** existing P6/P7 packaging and release work, before actual V1 release.
Current Core continuation and native execution holds retain their scope. After
V1, MATH-0 runs first, then MT-0 and Khronos/Ananke.

**Plan / owner:** [WinGet distribution plan](docs/plans/winget-distribution.md)
and existing [W11-W10/W11-W05](tasks/windows11-integration.md#433-winget-distribution-before-v1--7-november-2026),
with metadata, release/security and independent package QA. Keep one complete
Store-first MSIX. A separate Community listing is conditional on an authorized,
trusted publicly downloadable full-product installer; it does not select MSI.

- [ ] Establish the real source/identity/package contract and pending inputs in
  WGT-W01; distinguish `msstore` Store IDs from Community package identifiers.
- [ ] Implement the reached metadata/release preparation and qualify actual
  install, upgrade, service/user registration and removal through WGT-W02–WGT-W03,
  retaining exact OS/CPU admission, signature trust and W32Time recovery.
- [ ] Prepare and dry-run bounded release automation where applicable; review
  exact tools, artifact hashes, credentials, trigger and retry behavior in WGT-W04.
- [ ] Record **WGT-C01** through WGT-W05 before V1 approval. Plans or placeholder
  manifests cannot close implemented/native capability.
- [ ] At authorized publication, record **WGT-C02**: actual source-specific
  discovery and acquisition of the approved version. External review/propagation
  can remain pending; never claim availability from a PR, merge or tag alone.

The [plan's completion rules](docs/plans/winget-distribution.md#10-completion-maintenance-and-next-agent-record)
separate pre-V1 readiness from external public availability. Missing required
local/native evidence remains open; no merge-time guarantee, automatic customer
upgrade promise, extra service updater or weakened installer protection is added.

### LIT-0 — literature-derived qualification before V1

**Owner direction, 13 November 2026:** [OD-18](docs/nevrotick-owner-decisions.md#22-od-18--literature-derived-qualification-before-v1)
places small additional tests/cases before V1 and requires roadmap/detail for
longer implementation. The [non-core research and qualification review](docs/supporting-reviews/chrony-ntpsec-noncore-research.md)
owns the evidence synthesis, case mapping and reconsideration gates.

**Execution owners:** existing engine S0.1/S2.1/S2.2, W11-W01, catalogue DATA/R/C,
P4 broker/recovery and TA assignments. Reuse their canonical CHECK/case IDs,
configured checks and candidate records; no separate runtime or evidence chain.

- [ ] Add missing, bounded regression tests at actual existing consumers, with
  independent expected results, refusal behavior and healthy neighbors. Retain
  the current timestamp/correlation separation and reject forged catalogue
  authority. Record the exact executed cases and remaining gaps in their tasks.
- [ ] Before V1, refine the existing capture, restart, polling, NTS lifecycle,
  source-identity/withdrawal and TA cases using the
  [case matrix](docs/supporting-reviews/chrony-ntpsec-noncore-research.md#5-pre-v1-cases-under-existing-owners).
  Execute reached V1 cases before their existing phase/release exit.
- [ ] For absent future TA/RGT/TPKI consumers, retain precise inputs, independent
  expectations and the future owner/stage. Case preparation is the pre-V1
  deliverable; it does not implement or qualify that future runtime.
- [ ] Reconcile selected cases, actual results, existing coverage and longer
  work with independent review before the affected release checkpoint.

**Longer work and placement:**

| Work | Detailed gate and existing placement |
| --- | --- |
| Timestamp adapter and end-to-end timing comparison | [§6.1](docs/supporting-reviews/chrony-ntpsec-noncore-research.md#61-timestamp-adapters-and-measured-acquisition-benefit), W11-W01/S2.1/P2. Required V1 provenance stays in the V1 path; optional OS/hardware timing needs mapping, lifecycle and matched independent evidence. |
| Continuous NTS/polling and catalogue admission | [§§6.2–6.3](docs/supporting-reviews/chrony-ntpsec-noncore-research.md#62-continuous-nts-polling-and-resource-qualification), existing P2 and DATA/R/C/release owners. Substantial implementation does not defer an existing V1 correctness requirement. NTxMon/live assignment remains conditional, with no runtime updater selected. |
| TPKI issuer/certificate protocol | [§6.4](docs/supporting-reviews/chrony-ntpsec-noncore-research.md#64-conditional-tpki-protocol-evaluation), deferred research. Reopen with a complete artifact/specification, usable trust infrastructure and a matched benefit question; no mandatory post-V1 predecessor or protocol adoption. |
| New estimator/model experiments | [§6.5](docs/supporting-reviews/chrony-ntpsec-noncore-research.md#65-mathematical-experiments), unchanged MATH-0 after actual V1 release. Ordinary correctness regressions do not complete MATH-C01. |

**Exit discipline:** relevant existing task/phase records distinguish prepared,
executed, blocked and deferred cases. No document, model-only result or another
candidate's receipt passes an absent mechanism or native/release gate. Preserve
V1 → MATH-C01 → MT-C01 → KHR-C01 → GEO-C01 → Android, RGT-0 → CHR-0, existing
execution holds and formal deferral. This work grants no new clock authority.

### MATH-0 — post-V1 mathematical estimation experiments

**Status:** first TODO experiment after actual V1 release, selected by David on
13 November 2026 (Asia/Tbilisi), under
[OD-16 §20.1](docs/nevrotick-owner-decisions.md#201-mathematics-experiments-first--13-november-2026).
Research supports comparison candidates; no algorithm, dependency, production
constant, accuracy improvement or experiment result is adopted or qualified.

**Order:** actual V1 release → MATH-0 / MATH-C01 → MT-0 / MT-C01 →
KHR-0 / KHR-C01 → GEO-0 / GEO-C01 → Android. A source push, successful build,
release tag or completed planning alone does not establish actual V1 release.
This experiment adds no pre-V1 requirement and does not activate work now.

**Owner / evidence:** numerical/Core implementer with independent numerical and
protocol QA; use this roadmap entry as the experiment plan and the
[existing Core task](tasks/s1.3-source-selection-readiness.md) for the bounded
comparison record. Reuse the [regression task](tasks/s1.2-regression-readiness.md),
[engine task](tasks/engine-transport-pilot.md) and their existing acceptance owners
for affected fit, timestamp and applied-correction behavior. MATH-W/C identifiers
are local experiment labels, not replacements for canonical CHECK requirements.
Do not create a competing product engine or a second roadmap/evidence chain.

#### Experiment sequence and finite exits

- [ ] **MATH-W01 — freeze a fair comparison.** Identify the actual released
  NevroTick source/artifact, estimator path, configuration and applicable current
  contracts. Record the exact Chrony reference; the research pin is 4.9,
  `8df4f1263e206585e6c4e61352d042c2dd12916a`. Preserve its relevant delay weighting,
  asymmetry correction and window adaptation; a win over unweighted regression
  alone does not establish a win over Chrony. Replay the same frozen observation
  trace for estimator comparisons, with documented unit/sign conversions.
  Closed-loop comparisons share an independent plant/network scenario and seeds;
  observations can differ causally as control changes the clock. Keep polling/
  startup policies equivalent and independent clock/delay truth explicit.
  Separate arithmetic, estimator and closed-loop comparisons. Before
  execution, freeze cases, seeds, sample counts, resource/time budget, metrics,
  success/refutation criteria and stop conditions. Start offline without public
  server traffic or Windows clock writes.
- [ ] **MATH-W02 — bounded lower-envelope frequency estimation.** Evaluate the
  Moon/Skelly/Towsley linear-programming approach [M1], informed by Bletsas's
  traffic-dependent comparison [M2]. Use appropriate directional timestamp
  differences and an explicit rate basis; midpoint-offset noise is not one-sided.
  Test bursty queues, stable and changing delay floors, frequency ramps and
  erroneous low observations. State the approximately constant-frequency and
  stable-floor assumptions. A fitted intercept does not identify absolute path
  asymmetry or trustworthy UTC offset by itself.
- [ ] **MATH-W03 — independent numerical comparison.** Compare the released
  fitter with compensated accumulation [M3] and weighted Householder QR [M4],
  using an independent exact/high-precision reference for the same mathematical
  weighted objective. Preserve actual input admission, exact anchors, scales,
  weights and the admitted window bound; include cancellation, near-degeneracy,
  disparate weights and admitted subnormal inputs. The reviewed Core already
  conditions its fit and carries separate enclosures: compare against that code,
  not a presumed unconditioned normal-equation implementation. Its divisors are
  recomputed when window membership changes; a fixed-weight remove/add update
  would change the estimator. Check coefficient error, enclosure containment and
  width, refusals and cost separately. Better point arithmetic does not by itself
  tighten an enclosure. Reconcile underflow assumptions before importing a proof.
- [ ] **MATH-W04 — asymmetry and uncertainty controls.** Use [M7–M9] to construct
  identical-observation scenarios with different true offsets/directional delays,
  constant asymmetry, correlated paths and route changes. Include correctly
  authenticated exchanges with asymmetric delivery delay; authentication does
  not eliminate timing uncertainty. Keep numerical rounding enclosures,
  statistical covariance and physical UTC uncertainty distinct. Include server,
  capture and rate-mapping uncertainty in any claimed physical bound. Record
  coverage/containment under declared assumptions together with interval width;
  a narrower interval that excludes admissible truth is not an improvement.
- [ ] **MATH-W05 — Kalman, clock-noise and holdover comparison.** Evaluate an
  explicitly specified phase/frequency model against the same baseline and
  lower-envelope candidate [M2, M5, M6]. State process/measurement noise and
  mismatch handling; cover Gaussian noise, bursty traffic, irregular or missing
  observations, route changes, suspend/discontinuity and holdover. Use appropriate
  Allan/time-deviation analysis with explicit sampling assumptions. Account for
  achieved corrections so control effects are not misclassified as oscillator
  drift. A small covariance does not prove small physical error; declared growth
  parameters are not measured machine calibration.
- [ ] **MATH-W06 — conditional native/model extensions.** Assess Graham-style
  temperature/frequency prediction [M10], requiring a relevant sensor,
  independent timing reference, per-machine calibration, power-state coverage
  and safe behavior outside the calibrated range. Record a bounded later
  qualification task or an evidenced deferral if those inputs are unavailable;
  do not invent temperature coefficients. Use iHorology [M9] for synthetic bound
  sensitivity now; actual geography/landmark acquisition retains GEO-0's position
  after KHR-C01 and requires validated location/path assumptions. Specialized
  WLAN-beacon results [M11] are context, not an admitted Windows/NTS algorithm.
- [ ] **MATH-W07 — independent review and disposition.** Retain exact candidates,
  reference/source versions, reproducible recipes, independent expectations,
  actual outcomes, model limitations and resource costs. Report offset/frequency
  error, tail error, convergence/reacquisition, holdover, refusal behavior and
  appropriate uncertainty coverage/containment. Do not replace these with a
  favorable mean or an unsupported percentage improvement over Chrony. Keep
  machine/model-dependent results scoped to their tested conditions.

**Exit MATH-C01:** record the bounded comparison and an evidence-backed
`ADOPT_LIMITED`, `DEFER` or `REJECT` disposition for each candidate, with reasons
and a return trigger for deferred work. A declared budget/evidence limit can
justify deferral; an unexecuted or blocked check is never reported as a pass.
This evaluation/disposition admits MT-W01; adoption of a new estimator is not
required. Any adoption recommendation names its bounded implementation,
contract/enclosure changes, provenance/licence obligations and remaining
qualification under the existing owners before it can affect product decisions.
A paper, an experimental fit or a roadmap checkbox grants no clock authority.

The first experiments are lower-envelope estimation, numerical comparison and
uncertainty controls; noise-model and conditional native work follow within the
same bounded evaluation. Preserve the actual released baseline for later MT-0
comparisons and identify any intervening adopted changes separately. Existing
V1 correctness and release gates, applicable execution holds, source trust and
quorum rules remain. Formal eligibility after V1 does not put the entire proof
backlog ahead of this experiment; selected proof work retains OD-10's product
purpose and finite budget.

#### Primary reading and applicability

| ID | Paper / primary source | Use and evidence limit |
| --- | --- | --- |
| M1 | Moon, Skelly and Towsley, *Estimation and Removal of Clock Skew from Network Delay Measurements*, INFOCOM 1999; [DOI](https://doi.org/10.1109/INFCOM.1999.749287), [underlying 1998 author report](https://web.cs.umass.edu/publication/docs/1998/UM-CS-1998-043.pdf). | Lower-envelope/linear-programming frequency candidate; not a modern Chrony comparison or a solution to unknown fixed asymmetry. |
| M2 | Bletsas, *Evaluation of Kalman Filtering for Network Time Keeping*, IEEE TUFFC, 2005; [DOI](https://doi.org/10.1109/TUFFC.2005.1516016), [author paper](https://users.isc.tuc.gr/~abletsas/files/TUFFC05.pdf). | Kalman versus linear programming under different traffic assumptions; no universally superior estimator or Chrony 4.9 result. |
| M3 | Higham, *The Accuracy of Floating Point Summation*, SIAM J. Scientific Computing, 1993; [DOI](https://doi.org/10.1137/0914050), [author paper](https://nhigham.com/wp-content/uploads/2023/10/high93s.pdf). | Compensated summation and error analysis; no uniformly most accurate method, and the analysis assumes no underflow. |
| M4 | Golub, *Numerical Methods for Solving Linear Least Squares Problems*, Numerische Mathematik, 1965; [publisher record / DOI](https://doi.org/10.1007/BF01436075). | Householder QR comparator. The initial review verified the abstract/metadata only; obtain the needed algorithm details before implementation. QR output alone is not independent truth. |
| M5 | Levine, *Synchronizing Computer Clocks Using Kalman Filters*, 2011; [NIST record](https://www.nist.gov/publications/synchronizing-computer-clocks-using-kalman-filters), [paper](https://tf.nist.gov/general/pdf/2544.pdf). | Clock-noise-informed filtering in a telephone ACTS setting; transfer to Internet NTS needs separate evidence. |
| M6 | Levine, *The Statistics of Computer Clocks and the Design of Synchronization Algorithms*, Journal of Research of NIST, 2020; [paper / DOI](https://doi.org/10.6028/jres.125.008). | Noise timescales, Allan/time deviation, window/polling and holdover models; measured stability is distinct from accuracy. |
| M7 | Freris, Graham and Kumar, *Fundamental Limits on Synchronizing Clocks Over Networks*, IEEE TAC, 2011; [DOI](https://doi.org/10.1109/TAC.2010.2089210), [published paper](https://cs.nyu.edu/~apanda/classes/sp25/papers/time-sync-limits.pdf). | Identifiability and indistinguishable offset/delay scenarios under explicit model assumptions; validation reference, not a faster estimator. |
| M8 | Narula and Humphreys, *Requirements for Secure Clock Synchronization*, IEEE JSTSP, 2018; [DOI](https://doi.org/10.1109/JSTSP.2018.2835772), [author paper](https://rnl.ae.utexas.edu/images/stories/files/papers/narula2018secureSynchronization.pdf). | Authenticated-delay controls; published security conditions do not automatically hold for Internet NTS. |
| M9 | Veitch, Mani, Cao and Barford, *iHorology: Lowering the Barrier to Microsecond-Level Internet Time*, IEEE/ACM ToN, 2022; [DOI](https://doi.org/10.1109/TNET.2022.3174189), [2020 preprint](https://arxiv.org/pdf/2011.06713). | Asymmetry constraints from additional geography/landmark information. Detailed initial review used the preprint; learned/modelled bounds need validation and may exclude truth. No Chrony benchmark. |
| M10 | Najafi and Wei, *Graham: Synchronizing Clocks by Leveraging Local Clock Properties*, USENIX NSDI, 2022; [publication and paper](https://www.usenix.org/conference/nsdi22/presentation/najafi). | Learned local clock/temperature behavior; Chrony already has temperature compensation. Headline drift-bound gains are not the same multiplier in UTC accuracy over Chrony. |
| M11 | Joo, Lee, Kim and Kim, *Efficient One-Way Time Synchronization for VANET with MLE-Based Multi-Stage Update*, CMC, 2025; [full paper / DOI](https://www.techscience.com/cmc/v84n2/62935/html). | Specialized WLAN-beacon inter-client comparison; different timing information and metric prevent direct transfer to the Internet NTS client. |

### MT-0 — post-V1 multithreading and concurrent acquisition

**Status:** planned under David's 7 November instruction; the
[13 November amendment](docs/nevrotick-owner-decisions.md#201-mathematics-experiments-first--13-november-2026)
places MATH-0 first and this stage immediately after MATH-C01, before Khronos.
Implementation, measurements and native qualification remain open.

**Order:** actual V1 release → MATH-C01 → MT-C01 → KHR-C01 → GEO-C01 → AND-0.
Finishing a build or P0–P7 paperwork alone does not activate MT-0.

**Plan and evidence:** [NV-MT-001](docs/plans/post-v1-multithreading.md) owns the
bounded work packages, contracts, comparison and MT-C01 exit. Reuse the
[existing engine task](tasks/engine-transport-pilot.md#post-v1-multithreading--mt-0-7-november-2026)
for implementation/evidence; engine, platform, protocol and performance owners
work with an independent reviewer.

- [ ] **After MATH-C01:** run MT-W01 against the actual released source/artifact
  and account separately for any subsequently adopted mathematical change;
  identify existing concurrency, blocking boundaries, limits and a concrete useful
  progress/responsiveness question.
- [ ] Freeze and implement the smallest justified concurrency change through
  MT-W02–MT-W04: independent complete NTS pipelines, bounded native work, private
  candidates, serialized acceptance and responsive control.
- [ ] Compare explicit capacity and, when justified, scheduler alternatives under
  MT-W05; preserve trust/quorum/timestamps, actual native completion, policy-issue
  checks and aggregate resource bounds. No worker count/runtime is selected here.
- [ ] Complete independent review and the applicable qualification through MT-W06,
  recording MT-C01 before Khronos begins.

**Exit MT-C01:** qualified implementation or evidence-supported retention of the
released baseline, under the [plan's exit rule](docs/plans/post-v1-multithreading.md#8-exit-mt-c01-and-onward-scheduling).
Blocked/inconclusive work stays incomplete; a deliberate deferral or reordered
priority needs an explicit owner decision. Do not count planning as completion.

This schedules an enhancement after V1. Existing V1 blocking-work isolation,
native custody, current-authority and lifecycle requirements remain mandatory.
The current Windows/fuzz/storage/UI holds and pre-release development order
remain. OD-10's post-V1 formal eligibility does not place the whole proof backlog
ahead of the scheduled MATH-0 → MT-0 sequence.

### KHR-0 — evaluate independent time cross-checking

**Status:** planned research and bounded experiment; later on 3 November 2026
the owner selected a conditional bootstrap-reserve role. No implementation,
source roster or recovery envelope is qualified.

**Order:** after actual V1 release and [MT-C01](#mt-0--post-v1-multithreading-and-concurrent-acquisition), before GEO-0 and AND-0.

**Owners:** numerical/protocol implementer, with independent security/test review.

**Coordination and evidence:** the [Roughtime/NTS/Khronos/Ananke design](docs/plans/time-assurance-integration.md)
and [shared future task](tasks/time-assurance-integration.md) own the proposed
interaction, dependency accounting, common comparison frame and combined cases.
Ananke informs pool admission; it is not another protocol vote. Roughtime is a
separate later bootstrap/coarse-evidence path, not a new KHR-C01 prerequisite.

Evaluate a bounded initial-date reserve, adapting Khronos's sampling/filtering
idea from
[Informational RFC 9523](https://www.rfc-editor.org/rfc/rfc9523.html) and
[Ananke's cautious pool-formation proposal](https://www.ndss-symposium.org/wp-content/uploads/ndss2021_1A-2_24302_paper.pdf).
The future preferred route is Roughtime → NTS. Invoke this reserve only when the
preferred branch cannot supply usable date evidence, then complete full NTS
recovery before any broker clock change. Healthy NTS does not periodically run
Khronos under this selected profile; continuous watchdog use remains a deferred
alternative. Evaluate cold-start/era assumptions and forced fallback explicitly;
published steady-state watchdog bounds do not prove this adapted use. Keep the
existing estimator/correction policy and evidence-dependency accounting.

- [ ] Define the threat model, pool provenance, sampling distribution, round
  rules, disagreement threshold and finite experiment budgets before execution.
  Version the candidate catalog; retain operator/shared-upstream information,
  uncertainty, admission/removal reasons and DNS/configuration provenance.
  Multiple addresses, ASNs, countries or self-reported stratum 1 do not establish
  independence. Examine pool poisoning, Sybil entries and selective availability.
- [ ] Model four explicit outcomes: **agrees**, **contradicts**,
  **insufficient evidence** and **stale**. Bind the estimate and its supporting
  observations to the same round, pool/configuration generations, evaluation
  time and clock frame. Account for aging, applied corrections and suspend;
  insufficient or stale evidence cannot become agreement.
- [ ] Use cryptographically secure production selection and reproducible injected
  randomness in tests. Prove a requested resampling invokes a fresh selection;
  do not require consecutive selections to differ. Keep selected, responding,
  admitted and independently supported sets distinct. Timeouts or repeated
  resampling must not silently reduce agreement requirements or search until a
  convenient majority appears.
- [ ] Qualify the reserve's population, provisional-date contract and automatic
  activation envelope, including failed preferred acquisition and date-specific
  NTS failures. Generic Pool answers are not an audited Ananke population.
  Preserve
  [architecture §7.5](docs/windows-time-architecture.md#75-operating-profiles):
  unauthenticated observations never enter an NTS quorum or trigger a silent
  downgrade. A plain-NTP diagnostic must not gain an unlimited veto over valid
  NTS corrections. Any changed trust, inhibition or recovery authority needs its
  own reviewed decision; the reserve has no independent clock-write authority.
  Distinguish the selected future reserve from the current opt-in helper default;
  apply OD-03's vendor-zone requirement before redistributing Pool defaults.
- [ ] Separate a bounded candidate catalog from ordinary active associations.
  Preserve the existing **16-source limit** and aggregate CPU, memory, socket,
  timer and request budgets. Measure both ordinary and expanded checks; exhausted
  budgets yield insufficient evidence. Catalog size grants no permission to
  create hundreds of active histories or overload public servers.
- [ ] Start with a pure deterministic model, independent truth and the existing
  testing tools, without network traffic or Windows clock writes. Compare the
  unchanged baseline with the proposed check on coordinated false agreement,
  gradual drift, honest delay/asymmetry, selective loss, pool changes, replay,
  aging and correction/suspend between replies. Include conflicting small versus
  expanded rounds, cold roster availability, Roughtime suppression, wrong-date
  versus other certificate failures and the common-path/expired-key case.
  Predeclare recovery/error/false-acceptance, latency and resource/load criteria.
- [ ] Before reuse, review exact revisions and licences of
  [netars/chronos](https://github.com/netars/chronos) and
  [nwtime/khronos](https://github.com/nwtime/khronos). Their shared lineage is no
  independent oracle. Reproduce the proposal's reported cached-selection and
  old-estimate/new-round provenance issues as discriminating controls before
  treating them as confirmed upstream defects. Preserve source/port provenance;
  published numerical security bounds apply only under their stated assumptions,
  not automatically to a modified sampler, pool or retry policy.

**Exit KHR-C01:** retain the compared candidates, assumptions, independent
expected results, actual outcomes and costs in the [shared task](tasks/time-assurance-integration.md#5-exit-and-claim-discipline). Record
`ADOPT_LIMITED`, `DEFER` or `REJECT` with evidence and remaining gaps. An adoption
must define and qualify its bounded implementation before Android work begins;
deferral or rejection records the reason and return trigger. Completing this
evaluation and disposition admits GEO-0; both dispositions precede AND-0.
Neither requires adopting Khronos. Research alone proves no protection in the
delivered client.

### GEO-0 — evaluate geography-assisted source ordering

**Status:** future experiment requested by David on 5 November 2026 under
[OD-12](docs/nevrotick-owner-decisions.md#16-od-12--geoip-experiment-after-khronos).
No GeoIP database, dependency, provider or production selection rule is adopted.

**Order:** V1 released → MATH-C01 → MT-C01 → KHR-C01 → GEO-C01 → AND-0. A documented
`ADOPT_LIMITED`, `DEFER` or `REJECT` disposition closes this experiment; GeoIP
deployment is not an Android prerequisite. RGT-0 → CHR-0 remains separate.

**Owners and evidence:** catalogue/release owner, engine acquisition owner and
independent QA/security reviewer, in the
[existing catalogue task](tasks/server-catalogue-and-statistics.md#deferred-geoip-experiment--after-khronos).
Reuse DATA-03/04/08/09/10/17, publisher R, delivered catalogue C and the existing
optional Q/path-hint owners. Create no parallel database, cache or experiment plan.

**Question:** does approximate source location reduce cold-start time and failed
attempts before obtaining a sufficient, policy-eligible independent source set,
compared with successful-path reuse and selection by actual measurements alone?
Geographic distance is a hypothesis about acquisition cost, not a timing guarantee.

- [ ] Freeze a bounded comparison: the same catalogue, source admission,
  independence/quorum rules, network scenarios and total request/resource budget.
  Baseline uses existing history and ordinary acquisition without GeoIP; candidate
  adds geography only to the ordering of new candidates. Keep eligible successful
  paths first in their current network/policy context. Include a no-location
  control; unknown or distant sources must remain eligible under existing policy.
- [ ] Separate operator-declared site/region from an IP-derived estimate. Retain
  provenance, data revision, observation/review age, uncertainty or accuracy radius
  when supplied, and explicit unknown/anycast/multisite states. Associate address
  estimates with the actual resolved endpoint; keep NTS-KE and negotiated UDP
  destinations distinct. A publisher's DNS observation is not the client's current
  answer. Country, ASN and distance establish neither authenticity nor independent
  operators/clocks. Do not enumerate or poll the entire NTP Pool.
- [ ] Preserve any qualified Khronos/Ananke population and random sampling
  distribution. GEO-0 covers ordinary acquisition ordering; a geographic change
  to the reserve sampler would require separate threat analysis and proofs.
  Neither geography nor a low RTT adds a trust vote, weakens a quorum, grants
  clock authority or overrides actual protocol quality/uncertainty checks.
- [ ] Evaluate coarse catalogue metadata first. Treat client public-egress-IP
  discovery/geolocation as a separate optional comparison arm, not a startup
  requirement. Bind observations to the selected route/family/proxy context;
  a local private address or VPN/proxy egress does not identify the user's physical
  location. Missing lookup results, a wrong local date or a failed HTTPS lookup
  must not block time acquisition. Retain no client IP/location history by default.
- [ ] Cover cold/warm start, IPv4/IPv6, asymmetric delay and loss, NAT/VPN/proxy
  context changes, DNS/delegation churn, anycast, LAN/private/custom sources and
  missing/stale/incorrect geographic data. Use controlled traces/fixtures first;
  actual NTP/NTS execution requires its existing admission. Roughtime and deferred
  SOCKS5 scenarios use fixtures until their own runtime stages permit them; neither
  future implementation is required for GEO-C01. Keep the existing active-source,
  resource, service-permission and provider-rate limits across all experiment arms.
- [ ] Predeclare benefit and non-regression thresholds, matched inputs, independent
  expected outcomes and finite runs before execution. Compare distributions of time
  to a sufficient eligible set, requests, failures/timeouts, operator diversity,
  offset/uncertainty and false acceptance, plus CPU/RAM/network, delivered-data size
  and update/maintenance cost. Account for unsuccessful and timed-out runs; a single
  fastest response is not evidence of better synchronization. Freeze production
  constants only at the applicable later decision; OD-10 proof obligations remain.
- [ ] Assess [GeoLite2-City](https://github.com/wp-statistics/GeoLite2-City) only as
  a candidate alongside declared regions and no GeoIP. Before any reuse, review
  exact input provenance, [MaxMind's licence and update/removal obligations](https://www.maxmind.com/en/geolite/eula),
  [accuracy limits](https://support.maxmind.com/knowledge-base/articles/maxmind-geolocation-accuracy)
  and bounded trusted distribution. Shipping the full database is not presumed;
  publisher extraction does not automatically remove upstream obligations.
  Use [Chrony's measured-quality and source-selection behavior](https://chrony-project.org/doc/4.9/chrony.conf.html)
  as comparison evidence, without assuming kilometre distance predicts root distance.

**Exit GEO-C01:** retain exact candidates/data revisions, comparison recipe,
independent expectations, actual results and limitations in the existing task.
Record `ADOPT_LIMITED`, `DEFER` or `REJECT`; adoption requires demonstrated benefit
within unchanged trust, diversity, uncertainty and resource constraints. Define
its bounded proposed scope and remaining implementation/proof/qualification work
separately. Deferral or rejection records the reason and re-entry trigger, including
unavailable execution/evidence where applicable, without claiming measured benefit.
This roadmap entry itself supplies no benchmark result or permission to run
experiments.

### RGT-0 — Roughtime bootstrap for a badly wrong clock

**Status:** planned future stage; the owner clarified the initial-recovery purpose
on 3 November 2026. No Roughtime implementation, provider selection or Windows
qualification is established. This stage is distinct from KHR-0.

**Order:** RGT-0 → CHR-0. This later sequence preserves the current
V1 released → MATH-0 → MT-0 → KHR-0 → GEO-0 → Android prerequisites. Its scheduling against later Android
and DNS work remains open; it creates no new active Windows prerequisite.

**Owners:** protocol implementer, NTS/trust-policy owner, independent security
reviewer and Windows qualification owner.

**Coordination and evidence:** [NV-TA-001](docs/plans/time-assurance-integration.md)
records the research, proposed joint state/evidence rules and open choices;
[TA-W01–TA-W09](tasks/time-assurance-integration.md#3-work-assignments-and-finite-exits)
own the bounded work and evidence. RGT-C01 remains separate from KHR-C01.

**Outcome:** prefer Roughtime servers for an authenticated approximate-time
interval during cold date bootstrap. If that branch cannot supply usable evidence,
the selected future profile uses a bounded Khronos/Ananke-inspired provisional
date reserve. Feed the applicable candidate into isolated TLS verification and
date/era interpretation, then complete authenticated NTS acquisition before any
broker-authorized system-clock correction. Preserve the interval's uncertainty
and trust provenance; a signed server assertion is not proof of accurate UTC.

This extends the recovery goal already recorded in
[OD-02](docs/nevrotick-owner-decisions.md#4-od-02--distinguish-authenticated-recovery-from-ordinary-correction),
[architecture §8](docs/windows-time-architecture.md#8-recovery-when-the-system-date-breaks-tls)
and [R-02](docs/nevrotick-research-handoff.md#4-r-02--century-scale-authenticated-recovery).
Those contracts currently describe cached NTS, strict NTS-KE and optional plain
NTP hints; they did not select Roughtime. DEC-7 identifies an independent trusted
time anchor as a reason to revisit the helper design.
[RFC 10049](https://www.rfc-editor.org/rfc/rfc10049.html), published as
Experimental in October 2026, explicitly supports NTS certificate-time bootstrap
in §6. Its long-term server keys supply trust roots independently of X.509 time
validation. This is a protocol basis to qualify, not a qualified service list.

- [ ] Freeze the supported protocol version, implementation/dependency provenance
  and interoperability evidence. Review the server/key catalogue, independent
  operators, authenticated distribution, rotation/revocation and stale-list
  behavior. RFC 10049 §8 requires at least three operational independently run
  servers and chained repeated measurements; a one-server demonstration does not
  qualify this stage.
- [ ] Define bounded interval admission, monotonic elapsed-time/delay limits,
  supported true-date/era range, saved-floor conflicts, retries and resource use.
  Specify the faulty-server/path assumptions and refusal behavior. Qualify the
  handoff to the actual NTS verifier, including the unresolved Windows
  trust/revocation gate OPEN-8; supplying a better date does not itself close it.
- [ ] Keep valid cached/strict NTS acquisition ahead of unnecessary bootstrap.
  Roughtime must not directly set Windows time, vote as an NTS source, disable
  certificate checks or authorize ordinary plain-NTP discipline. The qualified
  reserve supplies only a labelled provisional TLS date; distinguish no/insufficient
  Roughtime evidence, signed conflict, date-specific NTS incompatibility and other
  NTS failures. Preserve protected
  NTS observations, candidate-time revalidation, source agreement and the broker's
  current correction authority. Until this scoped automatic-reserve extension is
  qualified, the existing strict-default/explicit-enable recovery policy remains.
- [ ] Retain deterministic cases for local-date errors in both directions around
  100 years, supported/unsupported true dates and era boundaries. Challenge
  malformed responses, bad signatures, request mismatch/replay, contradictory
  intervals, delay/uncertainty excess, source loss and changed/compromised keys.
  Demonstrate successful NTS acquisition with valid controls and rejection without
  clock effects on failure. Native experiments use an owned disposable Windows
  environment under the existing execution gates; public services are not stress
  targets.
- [ ] Qualify bootstrap provenance and the comparison-epoch handoff to any adopted
  watchdog. Do not count dependent mechanisms as independent votes or filter a
  watchdog pool until it agrees with the bootstrap estimate. Also test the
  disabled/deferred/rejected-reserve configuration; Roughtime-only qualification
  does not require adopting Khronos. Claiming the combined automatic-reserve
  profile requires its relevant KHR implementation/integration evidence.
  Continuous checks or a new correction inhibitor need a separate qualified policy.

**Exit RGT-C01:** retain a pinned implementation and trust configuration, reviewed
interval/admission and key-maintenance policy, controlled protocol evidence,
actual provider interoperability and Windows verifier/recovery evidence. Show
both successful bounded bootstrap into NTS and observable blocked outcomes without
unauthorized clock changes or weakened TLS checks. Record the qualified envelope
and remaining limitations; a research note or successful handshake alone does not
complete the stage. CHR-0 follows this completion.

### CHR-0 — independently replace Chrony-derived implementations

**Status:** owner-selected future implementation and provenance/rights review,
recorded 3 November 2026; no replacement or relicensing has been completed.

**Depends on:** RGT-C01, immediately after the Roughtime stage.

**Owners:** numerical/protocol implementer, independent correctness/security
reviewer and provenance/licensing reviewer; David owns the eventual licence and
permission decisions. See the [OD-01 future-direction addendum](docs/nevrotick-owner-decisions.md#3-od-01--source-derived-port-with-independent-verification).

**Outcome:** replace the Chrony-derived implementation with independently
authored, qualified code that can be distributed without relying on Chrony's
GPL grant. The future public distribution licence is **undecided**. David will
grant Microsoft permission to use and incorporate the resulting code into
Windows, including proprietary components, once the necessary rights are
established. This is the intended reuse outcome, not a present grant of upstream
rights or evidence of Microsoft interest or adoption.

- [ ] Freeze a candidate and inventory Chrony-derived implementation, tables,
  copied tests/fixtures and other included material against the existing
  [source ledger](tasks/s0.1-chrony-ledger.json) and crate provenance. Identify the
  actual replacement boundary and all consumers, generated outputs and shipped
  dependencies; keep original additions and mathematical/protocol facts distinct
  from adapted implementation.
- [ ] Specify the required behavior independently from standards, mathematical
  sources and reviewed product contracts, then implement the replacements with
  recorded authorship and source provenance. A language change, renamed symbols,
  AI assistance or a percentage of changed lines does not establish independence;
  do not describe another translation as a clean-room implementation.
- [ ] Preserve independently specified numerical/protocol correctness, security,
  bounded-resource and failure/recovery requirements. Qualify the replacement
  through the applicable regression, adversarial and platform checks; review
  intentional behavioral changes. Chrony differential results are supplemental,
  not the sole expected-result oracle.
- [ ] Review the complete replacement candidate and its rights/dependency graph.
  Establish that no retained Chrony-derived material still requires its GPL grant
  for that candidate, and identify any other terms that would prevent the intended
  Microsoft permission. Keep unresolved provenance or rights as explicit blockers
  to claiming licence independence, not as a reason to delete attribution.
- [ ] Preserve accurate historical releases, source mappings and required notices
  for retained material. After the rights review, bring the future public licence
  and the concrete Microsoft permission to David for selection and formulation;
  do not assume MIT, Apache-2.0, a later GPL version or a proprietary licence now.

**Exit CHR-C01:** retain the pinned replacement candidate, complete provenance and
dependency inventory, reviewed rights assessment and candidate-specific
correctness/security/resource evidence. Record any remaining restrictions. Public
licence selection and the actual Microsoft permission remain explicit release
decisions; this stage's plan is not a completed legal grant.

Until replacement qualification and the corresponding release decisions, the
current Chrony-derived code keeps its existing GPL-2.0-only obligations. This
future stage does not stop the present port or advance its implementation ahead
of the selected Roughtime predecessor.

### SOCKS-0 — optional SOCKS5 time acquisition

**Status, 5 November 2026:** deferred beyond the first Windows P0–P7 delivery.
David requested a concrete SOCKS5 profile and allowed deferral if implementation
and support would be disproportionate. The engineering assessment selects that
deferral under [OD-11](docs/nevrotick-owner-decisions.md#15-od-11--socks5-scope-and-deferral);
the profile below is a future design, not a working or qualified product feature.
First Windows delivery uses Direct. Its IPv4/IPv6 admission, upstream-failure
handling, successful-path reuse and interface selection retain their existing
P2/P5 owners and gates.

**Why later:** NTS needs both a TCP KE tunnel and actual UDP time exchanges.
SOCKS5 adds an independently owned TCP-controlled UDP relay, endpoint admission,
policy cutover, resolver/verifier routing, timestamp uncertainty and service versus
user-proxy lifetime. A successful TCP connection does not qualify those behaviors.
The [bounded source assessment and check plan](tasks/engine-transport-pilot.md#socks5-feasibility-and-deferral--5-november-2026)
also found that the examined libraries do not remove these integration duties.
This is a qualitative engineering judgement; no measured implementation estimate
or compatibility pass is claimed.

**Owners:** existing engine/protocol and Windows adapter owners, with resolver,
verifier, UI and independent security/QA review at their dependent boundaries.
Reuse the current engine task; do not start a parallel proxy project or database.
Schedule the stage when a real supported use case and maintenance capacity justify
it, after Windows delivery. Its place among other future work remains undecided;
the V1 released → MATH → MT → KHR → GEO → Android and RGT → CHR ordering remains intact.

**First future profile:** the canonical
[limited SOCKS5 profile](docs/windows-time-architecture.md#limited-socks5-profile--deferred)
uses a manually supplied numeric loopback endpoint, no proxy authentication,
explicit local DNS and numeric admitted destinations. NTS keeps its original TLS
server name. CONNECT carries KE; a separate monitored UDP ASSOCIATE session carries
time/helper datagrams. There is no automatic Direct fallback. The initial relay
must return the configured loopback IP and a nonzero port; other relay forms are
unsupported until separately qualified. The proxy owns its onward interface.
Local DNS is a disclosed routing/privacy exception, not proxy DNS or a claim that
all host traffic is tunnelled.

Remote proxy endpoints, authentication and proxy-side DNS are later, separately
admitted **SOCKS5 extensions**. None is required for this first subset. SOCKS4,
HTTP proxy modes, PAC/WPAD, browser/system import and bypass lists are outside the
selected proxy direction. In particular, the Firefox example's proxy-DNS checkbox
does not become a promised first-profile capability.

| Future step | Required result and evidence owner |
| --- | --- |
| Freeze the subset | Engine/protocol owner specifies bounded wire/state transitions, relay admission, complete I/O ownership and supported failure/status outcomes. Reuse DATA-06/08/09/10/17 and applicable PATH-03–20; admit any dependency on its exact source/graph. Independent review must resolve material contract findings before implementation readiness. |
| Portable controlled fixture | Exercise a real adapter against an independently specified local SOCKS fixture: valid CONNECT and UDP traffic alongside TCP-only proxy, malformed replies, fragmentation, oversize/truncated packets, wrong source, close/stall, stale revision and retry-budget cases. Check actual sends and retained resources, not only return values. |
| Windows service and interoperability | Run the same controls under the intended engine service token, including proxy starts late, restart/logout, suspend/resume and Apply with outstanding I/O. Qualify one pinned real proxy supporting the exact subset, DNS/verifier egress, NTS identity/default delegation, timestamps and uncertainty; disclose the tested compatibility envelope. |
| Product controls and support | Enable only the qualified settings. An observation-only connection check reports proxy negotiation, UDP relay, KE and valid packet results separately; it changes no clock or trust policy. Distinguish configured, saved, effective and failed state. Diagnostics are bounded/redacted and failed proxy acquisition cannot escape to Direct. |

**Exit SOCKS-C01:** retain the exact product/proxy/environment identities and
executed positive, rejection, recovery and resource evidence above, independent
review and the applicable existing CHECK/AC gates. Demonstrate useful NTS and
explicitly unauthenticated NTP operation through the relay without a direct time
packet, unapproved verifier escape or false success after a TCP-only test. Local
DNS must match its explicit policy. Close relevant mathematical obligations under
OD-10; do not repeat unchanged proofs or replace native checks with a model.
Documentation, library API availability and a browser proxy test do not close this
exit. No dependency, server installation, native experiment or release is enabled
by this entry.

## 2. Intended behavior and limits

The target behavior is:

1. Install an ordinary Android APK.
2. Start a local SNTP responder bound exclusively to an explicit loopback address
   and an unprivileged UDP port, initially `127.0.0.1:10123`.
3. Obtain and validate remote time through NTS, maintaining a local estimate with
   an explicit age, uncertainty and authentication status.
4. After a readiness check, use ADB to configure Android's system NTP service to
   query that local endpoint.
5. Let Android's existing time services accept the sample and decide whether to
   adjust the system clock.
6. Provide a verified ADB restoration procedure before planned disabling or
   uninstalling the companion.

No root, bootloader unlocking, VPN/TUN interface, packet-redirection rules,
Device Owner enrollment, direct `SET_TIME` permission or persistent Shizuku
bridge is part of this baseline. If the baseline is unsupported, report that
result instead of silently introducing a different privilege model.

### Proposed responsibilities

These are logical owners, not a decision to create a particular crate/module tree.

| Owner | Inputs and result | Boundary |
| --- | --- | --- |
| Android UI and lifecycle adapter | User intent and platform events; local service state and diagnostics | Ordinary app identity; no arbitrary shell or direct system-clock API. Select Kotlin/Java and any Rust bridge after the feasibility stage. |
| NTS acquisition and estimate | Validated source configuration and authenticated observations; bounded UTC estimate anchored to monotonic time | Owns source identity, keys/cookies, freshness and uncertainty; never accepts silent plain-NTP fallback. |
| Local SNTP responder | Bounded client requests; correctly correlated time replies only while the estimate is eligible | Explicit loopback bind; no LAN listener, public service or generic proxy. |
| ADB setup/restoration tool | Selected device, captured original configuration and fixed validated operations; readback and restoration receipt | Temporary user-authorized shell access; no permanent ADB connection is required for the intended Android 14+ runtime. |
| Android time services | Local SNTP response; network-time suggestion and policy-dependent clock effect | Source selection, thresholds and actual clock actuation remain Android/OEM behavior. |
| Optional Windows responder | Accepted Windows-engine estimate; independently authenticated time service for the companion | Future AND-6 component; incoming traffic and TLS never enter the privileged Windows clock broker. |

### Honest security and accuracy claims

- NTS authenticates the companion's remote exchange. The last loopback SNTP hop
  does not authenticate the APK to Android, and Android does not verify an NTS
  proof. Another local app may occupy the configured high port while the
  companion is absent. Exclusive binding without port reuse and checking
  ownership before activation reduce exposure; they do not create authenticated
  local IPC. Evaluate this residual risk before a supported release.
- A failure in the companion must prevent a healthy synchronized reply from
  that companion. It does not guarantee that the whole OS rejects other time:
  Android may retain an older network sample or use telephony/another origin.
  Inspect and describe the actual firmware policy. [A1, A4]
- AOSP derives its network uncertainty from the SNTP round-trip measurement.
  With a local bridge that measurement can be tiny even when upstream
  uncertainty or holdover drift is large. Track those errors independently;
  truthful root-delay/dispersion fields alone do not fix this AOSP limitation.
  Never advertise Android's reported uncertainty as end-to-end accuracy. [A2, A3]
- A live listener, an authenticated observation, an eligible estimate, Android's
  acceptance of a fresh sample and a system-clock change are different states.
  Diagnostics must distinguish them; an unchanged clock can be correct when the
  adjustment threshold has not been crossed.

## 3. Compatibility assumptions to prove

The first physical-device target is the **Redmi Note 12 Pro family**. Exact
4G/5G variant, Android version, regional MIUI/HyperOS build and current behavior
remain unconfirmed. Keep serials and other unique device identifiers out of
public test receipts.

| Platform evidence | Planning consequence |
| --- | --- |
| AOSP Android 14 supports a persistent `ntp_server` value with an `ntp://host:port` URI. [A2] | Start qualification with Android 14+; this is a candidate baseline, not a declared minimum supported version or proof of Xiaomi compatibility. |
| The inspected Android 13 implementation has a persistent hostname setting, but obtains a non-default port through an in-memory test override. [A5] | Allow a bounded Android 13 experiment; do not promise persistence or unattended post-reboot support through that override. |
| Shell service commands vary across releases. [A1, A6] | Read the device's actual `help`; choose a known version adapter and reject unsupported output/operations. |
| AOSP checks that an active network is connected even when the server is on loopback. [A2] | Test Wi-Fi, mobile, transitions and no-network state. Offline failure is not by itself evidence of a broken UDP server. |
| Android and Xiaomi impose background-start, foreground-service and power restrictions. [A7, A8] | Prove lifecycle behavior on the selected firmware; a successful foreground demonstration is not background qualification. |

Do not append public/plain NTP servers to the redirected setting as an invisible
fallback. The protected remote acquisition belongs to the companion. A separate
explicit plain profile, if ever requested, needs its own policy and visible trust
label; it must not be entered automatically on NTS failure.

## 4. Ordered Android stages

Begin these stages only after V1 release, MATH-C01, MT-C01, KHR-C01, any adopted cross-check implementation and
the GEO-C01 experiment disposition have been completed. GeoIP implementation is
separately scoped if selected; it is not an Android prerequisite. This ordering
leaves the current Windows plan intact.

All implementation and runtime checkboxes below are **open**. The current request
creates the plan; it does not establish a built APK, device test or release.

### AND-0 — prove the system-to-loopback path

**Owner:** Android platform implementer, with an independent observer.  
**Outcome:** determine whether the exact firmware supports the intended path
before committing to the full application.

- [ ] Record exact model/variant, Android/API level, build, target SDK of the
  test app, automatic-time state and relevant shell command capabilities.
- [ ] Inspect the existing persistent NTP setting and any active test override.
  Capture presence separately from value; do not treat printed `null` as a
  lossless backup format.
- [ ] Build the smallest ordinary-UID test APK with a loopback-only high-port
  SNTP listener. Do not use a shell-UID server as the only proof of app support.
- [ ] First test protocol behavior in an emulator/test device. On a daily-use
  phone, read-only discovery precedes any explicit configuration change; do not
  inject fictitious dates or large clock jumps.
- [ ] Apply a reversible temporary server override using the device's supported
  command, with an active network and a known-valid time source.
- [ ] Observe a real system-service request at the intended endpoint, a fresh
  accepted sample, and its submission to `time_detector`. Observe the selected
  source and either the intended clock effect or its valid no-adjustment reason.
- [ ] Challenge occupied port, absent listener, malformed/unsynchronized reply,
  no active network and an unsupported command. Confirm bounded failure and
  restoration without replacing someone else's configuration.

**Exit AND-C01:** retain exact APK/source/device identity, relevant redacted
service state, fresh sample evidence, negative controls and restoration readback.
Exit code zero from `force_refresh` alone is insufficient: the command can print
`false` while returning zero. A previous cached sample is not a fresh success.
If the ordinary-APK path fails, record the unsupported case; do not progress by
adding root, VPN or a permanent shell bridge.

### AND-1 — implement the bounded local SNTP responder

**Depends on:** AND-C01.  
**Outcome:** a protocol-correct local server backed by an injected estimate;
no claim of authenticated production time yet.

- [ ] Support the actual Android request format, including NTP version 3,
  client mode 3 and its 48-byte base packet. Decide any additional version or
  extension support explicitly; NTS upstream uses a separate association. [A3]
- [ ] Echo the original request transmit timestamp into the reply's originate
  field **bit for bit**. Do not lose Android's randomized submillisecond bits
  by decoding to milliseconds and reconstructing it.
- [ ] Produce mode-4 replies with valid, truthful reference, receive and transmit
  timestamps, leap/stratum/root fields and era interpretation.
  Reuse the pure history/UTC-rule and silent-source semantics of
  [NV-LP-001](docs/plans/leap-second-policy.md); qualify Android's served timestamp
  and local clock-owner behavior at AND-C01/AND-1. The selected Windows kernel
  ownership mechanism is not an Android implementation claim.
- [ ] Derive served time from a validated UTC estimate plus a monotonic clock
  that includes suspend. Avoid feeding the same adjustable Android wall clock
  back into its own source. Reject cross-boot monotonic anchors.
- [ ] Define maximum estimate age and uncertainty before using them as gates.
  Ineligible state yields no synchronized success response; verify the selected
  refusal behavior against Android.
- [ ] Bound datagram size, allocations, parser work, queue, response rate and
  aggregate local-client load. Reject invalid modes/lengths; provide no control,
  discovery or reflection/amplification service.
- [ ] Keep explicit IPv4 loopback binding initially; any IPv6 support needs its
  own bind and test. Never fall back to wildcard/LAN binding after an error.

**Exit AND-C02:** independent packet vectors and native app socket observations
cover healthy neighbors, exact correlation, hostile packets, startup conflicts,
aging, suspend and wall-clock changes. A mock/client-only test does not establish
acceptance by Android's system service.

### AND-2 — connect authenticated NTS acquisition

**Depends on:** AND-C02 and the admitted Android dependency graph.  
**Outcome:** eligible remote time reaches the local responder through one checked
time model.

- [ ] Evaluate reuse of the portable NevroTick protocol/numerical owners and the
  planned NTS client. Preserve source provenance, license obligations and
  function/module contracts; qualify Android ABIs and FFI boundaries separately.
- [ ] Select and verify Android TLS/AEAD, trust-store, key/cookie storage and
  execution owners. Existing Windows provider/runtime decisions do not select
  Android implementations.
- [ ] Implement TLS 1.3+ NTS-KE with verified server identity, protocol-compliant
  key/cookie handling and authenticated NTPv4 exchanges; bound retries and
  concurrent work. Admit complete validated results atomically. [A9]
- [ ] Define source policy consistently with the canonical NTS/source-selection
  rules. A deliberately single-source laboratory fixture is not the production
  quorum/default profile. Do not ship unresolved provider presets.
- [ ] Feed only currently eligible authenticated observations into the estimate;
  preserve units, source generation, boot identity, freshness and uncertainty.
  Do not initiate an uncontrolled upstream query for every local UDP request.
- [ ] On TLS/authentication/UDP failure, retain only still-eligible state and
  eventually refuse synchronized replies. Never downgrade to plain NTP.
- [ ] Qualify certificate-time bootstrap before automatic recovery claims. A
  bounded prototype may stop and request correction of an obviously wrong
  approximate date; it must not disable certificate verification or set the OS
  clock from an unauthenticated hint. Preserve the existing recovery research as
  a separately qualified path.

**Exit AND-C03:** controlled independent NTS-peer tests and real Android
integration cover valid responses, wrong identity, corrupt tags, replay, stale
cookies, clock/era ambiguity, cancellation, network replacement and upstream
outage. Public NTS providers are not stress-test targets.

### AND-3 — make ADB activation and restoration reliable

**Depends on:** AND-C03 for normal activation; AND-0 retains its bounded fixture
exception.  
**Outcome:** enable and undo the system endpoint change without hidden privileges.

- [ ] Select the target device explicitly and verify the APK package/signing
  identity. Use fixed command arguments and validated values, never a general
  user-supplied shell-command surface.
- [ ] Store the original setting's presence and exact restorable representation,
  the selected device association, and any test override. Filter captured state
  to required fields; do not publish a full settings dump or device serial.
- [ ] Refuse mutation if the original representation or an existing override
  cannot be safely understood/restored. A literal `null`, nullable stored value
  and absent key must not be conflated. [A10]
- [ ] Verify exclusive listener readiness and an eligible authenticated estimate
  before installing a valid persistent local-only URI.
- [ ] Read back both the saved setting and the service's **effective** server
  configuration. A test override can mask the persistent setting.
- [ ] Verify a fresh system-service sample after activation. Restore on setup
  failure while the authorized ADB host remains available.
- [ ] Make repeated setup/restoration idempotent and detect conflicting edits.
  Recheck expected values before mutation; the CLI does not provide an atomic
  compare-and-swap guarantee. Never overwrite a known later user/admin change.
- [ ] Before planned stop/uninstall, restore the saved value, or delete the key
  only if it was originally absent; remove only test overrides owned by this
  setup. Verify effective configuration and any fresh restored-source sample.
- [ ] Provide host-side recovery instructions that work even after the APK is
  gone. Explain that removing an override does not itself erase cached samples
  or undo an already-applied clock change.

**Exit AND-C04:** successful and interrupted setup, wrong/multiple devices,
pre-existing override, absent/custom/ambiguous original values, concurrent edits,
APK upgrade and planned uninstall all have verified outcomes. The normal APK
cannot guarantee automatic restoration after force-stop, crash or uninstall;
do not promise it. After successful persistent setup, disconnect ADB and prove
that runtime synchronization does not depend on that connection.

### AND-4 — qualify background service and power behavior

**Depends on:** AND-C04.  
**Outcome:** define a supported operating envelope rather than claiming an
invisible permanent daemon.

- [ ] Choose a legitimate foreground-service type, permissions, notification
  behavior and launch path for the actual target SDK. `dataSync` is not an
  assumed perpetual-listener solution: Android 15+ target rules impose time
  budgets; `specialUse` needs a justified use case and, for Google Play,
  review. Do not assume acceptance. [A7]
- [ ] Test Doze, App Standby and OEM restrictions independently. A foreground
  service or wake lock does not by itself establish unrestricted network/CPU
  access. Determine which supported user-granted battery/autostart settings are
  necessary and document their actual effect. [A7, A8]
- [ ] Test screen-off/unplugged operation, applicable service timeouts,
  notification denial, low-memory process death, user stop/force-stop, package
  upgrade, reboot and before/after first unlock.
- [ ] On restart, reconcile actual binding/configuration and invalidate old
  monotonic anchors. Handle persistent endpoint configuration while the APK is
  temporarily unavailable; do not report healthy service during that gap.
- [ ] Measure bounded memory, wakeups, CPU, battery and network use. Start with
  a bounded overnight test and extend only for a concrete lifecycle/resource
  question; no continuous wake lock as a substitute for a design.
- [ ] Document observed loss of service, cached Android time and fallback
  behavior. Do not promise fail-closed behavior of the entire Android OS.

**Exit AND-C05:** reproducible lifecycle evidence on each claimed firmware row,
including recovery and consumption measurements. If reliable background service
cannot be achieved under the agreed constraints, retain the foreground experiment
as such; do not label that device supported for unattended synchronization.

### AND-5 — deliver a usable Android preview

**Depends on:** AND-C01–AND-C05 for each supported row.  
**Outcome:** a signed, documented preview with truthful setup, state and recovery.

- [ ] Provide setup status, upstream identity/authentication, observation age,
  uncertainty, listener state, last known Android acceptance and actionable errors.
  Where privileged diagnostic access is unavailable, display Android acceptance
  as unknown/last verified rather than claiming live observation.
- [ ] Include a guided activation and restoration workflow; explain the need for
  ADB when restoring global settings and the consequences of forced removal.
- [ ] Qualify application signing/update identity, dependency inventory,
  license/source delivery, secret exclusion from backup/export and redacted
  support diagnostics. Android APK signing is separate from Windows package signing.
- [ ] Maintain a compatibility table with exact model/variant/build, Android and
  target SDK, required settings, observed gates and remaining limitations.
- [ ] Publish only claims supported by the identified candidate. Distribution
  channel, any store policy and release approval remain explicit release work.

**Exit AND-C06:** exact signed-APK installation, upgrade, rollback/restoration and
uninstall evidence; completed selected compatibility rows and independently
reviewed trust claims. A successfully built APK is not this exit.

### AND-6 — optionally serve time from NevroTick on Windows

**Depends on:** Android acquisition/consumer qualification and the relevant
qualified Windows engine, plus a separately adopted server contract.  
**Outcome:** preserve the originally discussed phone-to-PC use case without
turning it into a hidden v1 requirement.

- [ ] Define an optional server/responder outside the privileged clock broker,
  consuming the engine's accepted time estimate and synchronization state.
- [ ] For an NTS-protected phone-to-PC path, implement and qualify an actual
  NTS-KE/NTP server, server identity/certificate lifecycle, destination/port
  policy, firewall exposure, rate limits and failure semantics. An NTS client
  alone cannot provide that service.
- [ ] Verify pairing/trusted server identity, PC sleep/offline/reboot, stale
  estimate rejection and changes in reachability when the phone leaves the LAN.
  No automatic public-port exposure.
- [ ] Distinguish this from an ordinary LAN SNTP gateway: protected upstream
  acquisition at the PC does not authenticate a plain-NTP PC-to-phone hop.
- [ ] Keep direct Android-to-NTS operation available as an independent design
  path; do not require a permanently reachable Windows computer for every mode.

**Exit AND-C07:** separately reviewed and adopted server architecture plus actual
two-device authenticated interoperability and lifecycle evidence. Windows v1's
client-only contract is changed only by the explicit later scope decision.

## 5. Planned DNS-over-QUIC and certificate pinning

**Status:** requested future feature; no DoQ implementation or pinning support
is established by this roadmap.

**Owner clarification, 2 November 2026:** the Windows feature is a DNS answering
service for the whole computer and its applications, including programs unrelated
to NevroTick. The [OD-08 record](docs/nevrotick-owner-decisions.md#12-od-08--system-wide-windows-dns-and-dnssec)
also records the requirement for DNSSEC using hardware acceleration and multiple
CPU cores. This settles the intended Windows product scope; it does not establish
that every application's independent DNS path is already routed through the service.

The [detailed system-wide DNS and adaptive-cache design](docs/plans/system-dns-cache-design.md)
owns the proposed local listener, Windows enrollment/restoration, trust and time
dependencies, demand aggregates, hot/warm/cold/frozen classes, optional seasonal
heatmap, bounded proactive refresh, encrypted statistics, privacy controls,
resource accounting, comparison experiment and DNS-C01–DNS-C18 acceptance cases.
Its engineering parameters are proposals pending qualification. This future
workstream does not reorder the current Windows plan or the post-V1 MATH-0 → MT-0 → KHR-0 follow-ups.

The [source-attributed DNS upstream lessons](docs/plans/dns-upstream-lessons.md)
record mechanisms from Unbound, AdGuard Home/dnsproxy, Pi-hole FTL and
dnscrypt-proxy, including exact source revisions, NevroTick adaptations and
future checks. Their [DNSSEC/cache follow-up](docs/plans/dns-upstream-lessons.md#dns-r11-reuse-verification-results-bound-to-exact-content-and-proof-scope)
adds exact-content validation reuse, eligible chain/delegation evidence, cycle-safe
shared prerequisites, active-material custody, individual client responses,
cause-specific failure caching and time/trust transitions. These refine the
existing work, ownership, cache/routing and telemetry contracts. SIEVE/sharding,
additional proof reuse and signed catalogs remain conditional experiments;
source review establishes no implementation, dependency admission or measured
performance. Existing DNS-C01–DNS-C18 acceptance owners remain authoritative.

The additional [Security-kit cache follow-up](docs/plans/dns-upstream-lessons.md#security-kit-follow-up-cache-contracts-and-regression-cases)
records the owner's TS implementation as a source for demand/inspection capabilities,
immutable publication, retained-memory budgets, private events and concrete
capacity/async regression cases. Its source-derived counterexamples were not run;
the associated DNS-C checks remain future evidence. Baseline ownership/resource
contracts apply at the first DNS-1/2 owner; richer residency-policy experiments
remain conditional later work.

The [protected-storage plan](docs/plans/windows-protected-storage.md) supplies
the shared optional-state lifecycle for future DNS learning with distinct keys:
qualified VBS/TPM modes, explicit no-cache continuation or owner suspension,
current-mode notification, and forget barriers over inaccessible retired data.
It does not make ordinary DNS dependent on statistics persistence or change
DoQ/DNSSEC trust. Exact provider/native evidence remains open.

The [second protected-storage review](docs/plans/windows-protected-storage-upstream-lessons-2.md)
adds source-backed directory authority, uncertain-commit handling, durable-image
and operation-order checks, CNG/mitigation qualification and privacy-output cases.
The [agent task](tasks/windows-protected-storage.md) owns their future execution.
RedirectionGuard is a candidate; hardware rollback epochs, remote attestation,
chunking and a database remain conditional choices, not new DNS prerequisites.

The [Microsoft Windows 11 review](docs/plans/windows11-integration.md)
adds native DNS configuration/listener/event candidates and private-namespace,
encrypted-DNS-policy, WSL/container and ZTDNS coexistence questions. Its
[agent task](tasks/windows11-integration.md) also prepares service-scoped WSH
restrictions, power/cost/memory-aware optional refresh, measured CNG hashing and
coarse native diagnostics. Preserve DHCP inheritance and later configuration
owners during restoration; OS DNS enrollment alone proves no universal capture.
CNG SIMD batching and bounded multicore verification remain separate concerns.
These refinements retain the existing DNS-C owners, DoQ-only direction and
current development ordering; no API, provider, new analyzer profile or
machine-wide policy is enabled by this research.

The [Learn MCP follow-up](docs/plans/windows11-integration-follow-up.md#5-dns-integration-and-resource-hints)
adds native connectivity/cost hints as an alternative when interface-level
information suffices, the immediate/pending native DNS completion distinction,
NRPT precedence over a custom server list, and preservation of unrelated DNS
fields. It also makes the Windows DNS Client cache's separate privacy scope
explicit: forgetting NevroTick learning cannot promise deletion of all DNS
traces on the computer. These refinements retain the same DNS-C owners.

Add **DNS-over-QUIC (DoQ)** with configurable pinning of both **leaf/server
certificates** and **intermediate CA certificates**. **DoH, DoT and DNSCrypt are
out of scope**, including DoH carried over HTTP/3. Upstream connection or
authentication failure must not silently switch to those protocols or plaintext
DNS. Proposed loopback UDP/TCP DNS ingress for local applications is a separate
hop, not an upstream plaintext fallback.

Use the dedicated DoQ mapping in RFC 9250, with ALPN `doq` and normally UDP port
853. QUIC's TLS handshake remains part of DoQ; excluding DoT does not exclude
TLS authentication. [A12]

- [ ] Qualify Windows-wide DNS integration: local endpoint ownership, IPv4/IPv6,
  supported application paths, interfaces/VPN/NRPT, administrative activation and
  exact owned-state recovery/restoration. Identify independent application DNS
  paths explicitly; changing OS settings alone does not prove universal routing.
  Android system-wide DNS remains a separate feasibility question under its
  no-root/no-continuously-active-VPN constraints.
- [ ] Qualify DNSSEC validation, trust-anchor/time lifecycle, exact-feature
  hardware dispatch and bounded multicore work under OD-07/OD-08. Select the
  provider and limits from current evidence; a CPU label, DO/AD flag or worker
  count does not establish local validation or a measured speedup.
- [ ] Evaluate the bounded demand model and proactive-refresh proposal against
  ordinary caching and demand-triggered prefetch. Require useful lifetime
  extension, actual client benefit, protected statistics and finite resource
  costs; retain or defer the mode according to the recorded results.
- [ ] Define resolver identity, bootstrap addresses, query/cache/resource limits
  and network-change behavior. Resolve bootstrap DNS and certificate-time
  dependencies explicitly; do not hide a system-resolver fallback or disable
  certificate validation to get started.
- [ ] Admit the QUIC/TLS dependency graph and enforce pin checks in its actual
  certificate verifier. Do not assume Android's network-security configuration
  automatically governs a separately selected native QUIC/TLS implementation.
- [ ] Treat pins as an additional restriction on normal certificate-chain,
  validity-period and expected-server-name validation. Scope pins to the intended
  resolver identity. An intermediate pin must match an intermediate on the
  accepted certification path, not an unrelated certificate sent by the peer.
  DNS transport authentication does not replace NTS server authentication. [A13]
- [ ] Select the pin representation before freezing configuration: full
  certificate fingerprints versus public-key/SPKI hashes. Support leaf and
  intermediate pinsets; explicitly define leaf-only, intermediate-only and
  combined-policy behavior, including whether both levels must match.
  A single flat any-match pinset must not be presented as enforcing both levels.
- [ ] Plan active/backup pins, overlapping rotation, CA/intermediate changes and
  authenticated configuration updates. Pin expiry, mismatch or update failure
  must not automatically disable pinning. Scope connection reuse and resumption
  to the current trust/pin policy. [A14]
- [ ] Expose the selected resolver, DoQ connection state, pin-policy state and
  actionable failures without logging query names by default. Distinguish
  configured policy from a successfully verified connection.
- [ ] Qualify valid neighboring cases and rejection controls: wrong hostname,
  invalid/expired chain, leaf mismatch, intermediate mismatch, an unrelated
  matching certificate, alternate/cross-signed paths, rotation, stale resumed
  sessions after policy change, blocked UDP and attempted protocol downgrade.

**Exit DOQ-C01:** a reviewed resolver/pinning contract, an admitted implementation
and independent interoperability/negative-test evidence for the claimed platform
and lookup scope. Pin formats, combined-match semantics, resolver presets and
Windows integration details remain implementation decisions; the whole-computer
Windows product scope is accepted. DOQ-C01 does not by itself qualify system
enrollment, DNSSEC or the adaptive model: use the [DNS stage and acceptance map](docs/plans/system-dns-cache-design.md#12-staged-delivery-and-next-action)
for those dependencies. This feature does not reorder the current Windows
execution plan or make Android feasibility depend on DoQ.

## 6. Command reference for the feasibility stage

These are **version-scoped examples**, not an instruction to change a phone
before the listener, captured configuration and restoration path exist.

Read-only discovery (replace the serial placeholder locally; keep it private):

~~~text
adb -s DEVICE_SERIAL shell getprop ro.product.model
adb -s DEVICE_SERIAL shell getprop ro.build.version.release
adb -s DEVICE_SERIAL shell getprop ro.build.version.sdk
adb -s DEVICE_SERIAL shell settings get global ntp_server
adb -s DEVICE_SERIAL shell settings get global auto_time
adb -s DEVICE_SERIAL shell cmd network_time_update_service help
adb -s DEVICE_SERIAL shell dumpsys network_time_update_service
adb -s DEVICE_SERIAL shell dumpsys time_detector
~~~

Capture setting presence separately using a filtered listing/provider read; the
`get` result alone is not a complete restoration record. Dumpsys output may
contain sensitive context: retain only the fields needed for qualification.

Modern AOSP temporary override, **after** AND-0 preflight and listener startup:

~~~text
adb -s DEVICE_SERIAL shell cmd network_time_update_service set_server_config_for_tests --server ntp://127.0.0.1:10123 --timeout_millis 5000
adb -s DEVICE_SERIAL shell cmd network_time_update_service force_refresh
~~~

Remove the temporary override created by this experiment:

~~~text
adb -s DEVICE_SERIAL shell cmd network_time_update_service reset_server_config_for_tests
~~~

Verified Android 13 test syntax uses
`set_server_config --hostname 127.0.0.1 --port 10123 --timeout_millis 5000`;
its reset is `set_server_config` without options. It is not a persistent
high-port deployment method. [A5, A6]

The Android 14+ **persistent candidate**, only after the activation gate, is:

~~~text
adb -s DEVICE_SERIAL shell settings put global ntp_server ntp://127.0.0.1:10123
~~~

Restore the exact captured value through the version-aware tool; when the key
was confirmed originally absent, restoration uses
`adb -s DEVICE_SERIAL shell settings delete global ntp_server`.
Do not substitute a hardcoded public server, blindly write `null`, reset an
unrelated test override or interpret setting readback as proof of synchronization.

`force_refresh` can submit a clock-affecting suggestion when automatic time is
enabled. Check its printed boolean and fresh service state asynchronously;
do not infer success from its exit code. USB ADB `reverse` is not an automatic
UDP tunnel; this design uses local UDP and the companion's own remote transport.
[A1, A6, A11]

## 7. Decision points and evidence discipline

Current implementation follows the Windows development plan, including WGT-0
capability at P6/P7 before actual V1 release. WGT-C02 separately records externally
observed catalog availability. Its first post-V1 TODO is **MATH-0**, followed by
**MT-0**, **KHR-0** and **GEO-0**. MATH-C01, MT-C01, KHR-C01 and GEO-C01 precede
Android work.
The first Android stage is then **AND-0**, starting with device identification and
read-only capability discovery. Dates for later stages are intentionally not promised before those
results.

Resolve these choices when the dependent stage is ready, using the existing
[owner-decision procedure](docs/nevrotick-owner-decisions.md#6-od-04--decisions-when-evidence-makes-them-concrete):

- Supported Android versions/builds and the reliability claim after the probe.
- Android dependency/provider/runtime choice, FFI and ABI support.
- Numerical age/uncertainty/holdover and production source-policy parameters.
- Admissible FGS/lifecycle/distribution design and residual local-port
  impersonation risk.
- Any requirement that the whole OS prohibit alternate time origins.
- Whether to add the separately exposed Windows NTS server in AND-6.
- Any Khronos/Ananke adoption, including pool/sampling parameters, NTS/plain-NTP
  roles and authority on contradiction or unavailable evidence.
- Any GEO-0 adoption, including measured benefit, data provenance/maintenance,
  optional client-location handling and its separately scoped implementation.
- Windows DNS integration/coverage and lifecycle details, resolver bootstrap,
  pin representation, combined leaf/intermediate matching and rotation policy.
- DNSSEC provider/trust/time policy, qualified acceleration and multicore bounds;
  adaptive-cache parameters, statistics privacy/identity and measured adoption.
- Any later Android-wide DNS integration and its separately qualified scope.
- Roughtime's protocol/provider/key policy, admitted uncertainty and recovery
  envelope, Windows verifier qualification and scheduling against other later
  work; CHR-0 remains its immediate successor.
- Preferred/reserve admission, cold-start sampling, dependency/epoch accounting and any later correction-inhibition
  policy under [NV-TA-001](docs/plans/time-assurance-integration.md#9-windows-gate-and-explicit-decision-alternatives);
  no generic majority vote across mechanisms or automatic new clock authority.
- The post-CHR-0 public distribution licence and concrete Microsoft permission,
  based on the replacement candidate's reviewed rights rather than a licence label.
- SOCKS-0 scheduling after first Windows delivery, the exact maintained proxy
  compatibility envelope and any later SOCKS5 authentication/remote/DNS extension.

Do not let an unresolved later choice block read-only discovery or a controlled
earlier fixture. Do not silently resolve a security-policy exception while
implementing an earlier stage.

WGT-C01/WGT-C02, MT-C01, KHR-C01, GEO-C01, RGT-C01, CHR-C01, SOCKS-C01, AND-C01–AND-C07 and DOQ-C01 are local roadmap exit IDs, not replacements for existing
REQ/BND/DEC/THR/CHECK IDs. Apply the relevant canonical protocol, trust,
input-contract, provenance, diagnostics and resource obligations; do not claim
Windows-native checks qualify Android behavior.

DNS-C01–DNS-C18 are local future-feature acceptance IDs in the linked DNS plan;
their planned cases establish no executed DNS, native Windows or release result.

At implementation start, create/reuse one stage task in `tasks/` and link it
from this roadmap. Record candidate commit/APK hash, exact environment, inputs,
independent expected outcomes, actual results, restoration, open findings and
next action. Repository planning/security/testing skills guide the applicable
boundaries; add Android-specific guidance only when an actual gap requires it.
Documentation review and upstream source evidence are not device-test passes.

## 8. Primary evidence

Reviewed for this roadmap on 30 October 2026. Versioned AOSP sources support the
specific version claims; mutable current sources and Android/OEM policies must
be rechecked for the eventual target build. No cited source establishes tested
support for the user's particular handset.

- **A1 — Android network time:** [network time detection](https://source.android.com/docs/core/connect/time/network-time-detection).
- **A2 — endpoint parsing, connectivity and estimate uncertainty:** [AOSP Android 14 NtpTrustedTime](https://github.com/aosp-mirror/platform_frameworks_base/blob/android-14.0.0_r1/core/java/android/util/NtpTrustedTime.java) and [current source](https://android.googlesource.com/platform/frameworks/base/+/HEAD/core/java/android/util/NtpTrustedTime.java).
- **A3 — real SNTP wire behavior:** [AOSP SntpClient](https://android.googlesource.com/platform/frameworks/base/+/refs/heads/main/core/java/android/net/SntpClient.java).
- **A4 — system source priority:** [Android time source priority](https://source.android.com/docs/core/connect/time-source).
- **A5 — Android 13 compatibility:** [NtpTrustedTime](https://github.com/aosp-mirror/platform_frameworks_base/blob/android-13.0.0_r1/core/java/android/util/NtpTrustedTime.java) and [NetworkTimeUpdateServiceShellCommand](https://github.com/aosp-mirror/platform_frameworks_base/blob/android-13.0.0_r1/services/core/java/com/android/server/NetworkTimeUpdateServiceShellCommand.java).
- **A6 — current test/refresh commands:** [NetworkTimeUpdateServiceShellCommand](https://android.googlesource.com/platform/frameworks/base/+/refs/heads/main/services/core/java/com/android/server/timedetector/NetworkTimeUpdateServiceShellCommand.java).
- **A7 — Android background operation:** [FGS types](https://developer.android.com/develop/background-work/services/fgs/service-types), [timeouts](https://developer.android.com/develop/background-work/services/fgs/timeout), [background-start restrictions](https://developer.android.com/develop/background-work/services/fgs/restrictions-bg-start), and [Doze/App Standby](https://developer.android.com/training/monitoring-device-state/doze-standby).
- **A8 — Xiaomi controls, not model-specific qualification:** [HyperOS autostart policy](https://dev.mi.com/xiaomihyperos/documentation/detail?pId=1624) and [Xiaomi background-operation guidance](https://www.mi.com/global/support/faq/details/KA-700752/).
- **A9 — NTS protocol and security limits:** [RFC 8915](https://www.rfc-editor.org/rfc/rfc8915.html), especially certificate-time bootstrap, delay attacks and downgrade resistance.
- **A10 — settings output/representation:** [SettingsService](https://github.com/aosp-mirror/platform_frameworks_base/blob/main/packages/SettingsProvider/src/com/android/providers/settings/SettingsService.java) and [SettingsState](https://github.com/aosp-mirror/platform_frameworks_base/blob/main/packages/SettingsProvider/src/com/android/providers/settings/SettingsState.java).
- **A11 — ADB transports and commands:** [official ADB man page](https://android.googlesource.com/platform/packages/modules/adb/+/refs/heads/main/docs/user/adb.1.md).
- **A12 — DNS-over-QUIC transport:** [RFC 9250](https://www.rfc-editor.org/rfc/rfc9250.html), especially connection establishment, port selection and authentication profiles.
- **A13 — TLS service identity:** [RFC 9525](https://www.rfc-editor.org/rfc/rfc9525.html), especially certification-path validation and matching the intended service identity.
- **A14 — pinning semantics and rotation example:** [Android certificate pinning](https://developer.android.com/privacy-and-security/security-config#CertificatePinning). Its any-match SPKI pinset and pin-expiration behavior are reference behavior, not the selected DoQ policy or proof of enforcement by a native stack.

