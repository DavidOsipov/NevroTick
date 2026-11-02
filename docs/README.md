# NevroTick — documentation guide

**Prepared for:** David Osipov  
**Snapshot date:** 29 October 2026  
**Canonical architecture:** RWTC-ARCH-001, revision 1.4

The maintained set contains ten principal Markdown documents, seven supporting independent reviews, a UI toolkit comparison report, an x64 runtime-optimization research report, a critical architecture-contract assessment, a state-guided fuzzing experiment report and a Verus assessment. The owner-decision/research addition is a scoped amendment, not a repeated full-product qualification. The feature plan translates the architecture into basic user capabilities, screens, defaults, acceptance cases and delivery slices. The architecture and development-hardening guide include 29 October 2026 amendments making upstream-code verification and stress testing explicit, adopting garde-backed construction plus explicit function/module contracts, and integrating installed native memory diagnostics into agent workflows. Proposed behavior and planned checks are not implementation evidence.

## Plans, acceptance and execution evidence

Agents start with [AGENTS.md](../AGENTS.md), choose the existing task and relevant
rows below, then load the matching [repository skills](../.agents/skills/README.md).
Use the [testing contract](../.agents/skills/testing/CONTRACT.md) to select evidence.
The plan is distributed by responsibility; these links locate its owners without
creating another specification.

The [development execution plan](plans/development-plan.md) consolidates the
P0–P7 sequence, NTP/NTS priorities, upstream evaluation, explicit implementation
contracts, mandatory skill reading by phase and phase-end acceptance. Use it to
assign the next bounded development task; the canonical documents below continue
to own requirements, and task records own executed evidence.

| Question | Source of truth |
| --- | --- |
| What should a development agent do next, and which skills must it read and use? | [Development execution plan](plans/development-plan.md), especially its phase skill map, acceptance gates and next executable task. |
| Where are the personal project direction, future platforms and DNS plans recorded? | [Product roadmap](../ROADMAP.md): Android local SNTP bridge, ADB setup/restoration and feasibility gates; [system-wide Windows DNS/cache design](plans/system-dns-cache-design.md) for DoQ-only upstream resolution with leaf/intermediate pinning, DNSSEC acceleration/multicore work and bounded adaptive caching. The Windows execution plan continues to own current work, phase readiness and acceptance. |
| Where are the origins and qualification scenarios for borrowed DNS mechanisms? | [DNS upstream lessons and adaptation plan](plans/dns-upstream-lessons.md): pinned Unbound, AdGuard Home/dnsproxy, Pi-hole FTL and dnscrypt-proxy sources; eighteen lessons, including the [DNSSEC/cache follow-up](plans/dns-upstream-lessons.md#dns-r11-reuse-verification-results-bound-to-exact-content-and-proof-scope), with observed mechanisms, proposed adaptations and existing DNS-C owners; the [Security-kit follow-up](plans/dns-upstream-lessons.md#security-kit-follow-up-cache-contracts-and-regression-cases) adds TS cache contracts and source-derived regression cases under the same owners. |
| Where are protected key storage, VBS fallback and their evidence gates owned? | [Architecture §12.1.2](windows-time-architecture.md#1212-protected-optional-storage-and-explicit-fallback), [protected-storage plan](plans/windows-protected-storage.md), [development workstream](plans/development-plan.md#84-protected-optional-storage-design-and-qualification) and [provider qualification](windows-time-development-hardening.md#641-protected-storage-provider-qualification). |
| Where are source examples and adaptation limits for protected Windows storage? | [First upstream review](plans/windows-protected-storage-upstream-lessons.md) and [second upstream review](plans/windows-protected-storage-upstream-lessons-2.md): pinned implementation/test sources, adaptation limits and future checks. The second adds directory authority, indeterminate commits, crash/protocol models, per-call CNG contracts, behavioral mitigation checks, RedirectionGuard and conditional rollback research. |
| What should an agent do with the protected-storage research? | [Protected-storage agent task](../tasks/windows-protected-storage.md): bounded work items, prerequisites, owners, independent expectations, healthy/defect controls and candidate-specific evidence, mapped to the existing storage plan. The native experiment pause and current development priority remain unchanged. |
| Which Microsoft Windows 11 mechanisms should be evaluated, and where does their work belong? | [Windows 11 integration research](plans/windows11-integration.md), [Microsoft Learn follow-up](plans/windows11-integration-follow-up.md), [phase workstream](plans/development-plan.md#85-windows-11-integration-research-and-qualification) and [agent task](../tasks/windows11-integration.md): packet timestamps, sleep/timer behavior, DNS/WSH integration, service/MSI/COM lifetimes, resource signals, CNG, diagnostics and future UI. The follow-up adds exact completion, metadata, permission, tracing and partial-replacement cases plus source discrepancies. Existing CHECK/DNS-C/ST-T owners and conditional triggers apply; no native pass or automatic API/dependency adoption. |
| Which product capabilities and user journeys are intended? | [Feature specifications, F-01–F-10](windows-time-features.md#2-feature-specifications) and [essential journeys](windows-time-features.md#6-essential-user-journeys). |
| What owns the behavior, authority and state? | [Component architecture](windows-time-architecture.md#3-component-architecture-and-authority), the task-relevant technical sections and [accepted owner decisions](nevrotick-owner-decisions.md). |
| Which phase is ready, and what must it prove? | [Architecture §21](windows-time-architecture.md#21-delivery-phases-and-readiness-gates): P0–P7, including the bounded S0.1–S0.7 experiments. |
| What integrated slice should the task deliver? | [Feature plan §9](windows-time-features.md#9-delivery-slices-for-agentic-development): UI-M0 and S-0–S-6, with prerequisites, owners and exit evidence. |
| What must a user-visible product scenario demonstrate? | [Feature plan §8](windows-time-features.md#8-acceptance-scenarios-and-evidence): AC-01–AC-20, mapped to technical checks. |
| What technical/security result is required? | [Architecture §20.2](windows-time-architecture.md#202-acceptance-catalog): CHECK-1–CHECK-27, independent expected results, protected effects, valid controls and stage/owner. |
| What is in scope for the current UI prototype? | [Architecture §16.5](windows-time-architecture.md#165-restricted-first-ui-milestone-ui-m0): M0-01–M0-08 and D1–D3 deferrals/re-entry gates; [UI-M0 task](../tasks/ui-m0.md) for implementation ownership and evidence. |
| Which unresolved decisions or investigations gate dependent work? | [Open decisions](windows-time-architecture.md#22-open-decisions-and-release-blockers), [owner-decision procedure](nevrotick-owner-decisions.md) and [research assignments](nevrotick-research-handoff.md). |
| How are checks run, and where are results kept? | [TOOLING.md](../TOOLING.md#starting-implementation) and [configured tool execution](../.agents/skills/testing/references/configured-tooling.md); the existing task record owns candidate-specific results and gaps. |

Keep the identifiers distinct: S-0 is a delivery slice; S0.1–S0.7 are P0
experiments. UI-M0's M0 checks do not complete the AC/CHECK catalogs or P0/P5.
Select only the criteria applicable to the authorized task and retain deferred
requirements with their owners and restoration triggers.

The current [UI-M0 task](../tasks/ui-m0.md#remaining-evidence-and-next-work)
records an implemented, observed prototype with required full native-memory
instrumentation still incomplete. Read its later candidate addenda before reusing
earlier results, and verify relevant source/environment identity before relying on
them. A task receipt records execution; a catalog specifies expectations.

## Current baseline and amendments

**Windows execution stopped, 2 November 2026:** the owner reported a Defender
detection during attempted CNG fixture staging and is keeping it blocked. All
Windows execution and retries are suspended under the
[incident record](../tasks/s0.1-crypto-feasibility.md#defender-alert--windows-execution-stopped).
The [next-task plan](plans/development-plan.md#7-next-executable-task-for-a-development-agent)
routes independent work to the current portable Core slice. The standalone
AES-SIV checkpoint remains evidence for its recorded candidate; the separately
held fuzz runner must not be retried through another invocation or agent.
Neither a confirmed false positive nor host compromise is established.

**Microsoft Learn MCP follow-up, 3 November 2026:** the
[second source pass](plans/windows11-integration-follow-up.md) records callback
invalidation and ancillary-data boundaries, conditional native socket retirement,
COM initialization/teardown, service-managed state lifetime, declarative MSI ACLs,
SCM observers, ETW authorization/delivery, partial file replacement and DNS/cost
hints. It preserves contradictory Microsoft overview/ABI descriptions as explicit
SDK/binding evidence gaps. VBS claim extensions, side-channel fields, UDP batching
and VM-generation signals remain conditional reserves. The existing W11/ST-W
tasks carry the applicable cases into their phase owners; no parallel plan or
acceptance catalog is created. The integration uses the later `45ff247` baseline
and retains its portable Core continuation, broad Windows execution stop,
separate fuzz no-retry hold and earlier adoption/review receipts. Source research
and this documentation amendment establish no implementation or native result.

**Microsoft Windows 11 integration research, 3 November 2026:** the
[source review](plans/windows11-integration.md) records primary Microsoft evidence,
project-specific adaptations, compatibility limits and distinguishing checks.
The [agent assignments](../tasks/windows11-integration.md) and
[development workstream](plans/development-plan.md#85-windows-11-integration-research-and-qualification)
place them within existing phases. Canonical refinements cover actual packet/
interface provenance, Modern Standby execution limits, configured versus effective
service protection, Restart Manager admission, DNS ownership and native resource
signals. WSH, packet timestamp adapters, CNG batching, PerfLib and new fault
profiles remain candidates pending their stated evidence. LPAC, full enclaves,
package identity and new distribution channels remain conditional; PPL is not
an ordinary utility-service route. Current work, DoQ-only upstream, the complete
MSI, storage pause and UI-M0/D1–D3 deferrals remain unchanged. Documentation
integration establishes no implemented feature, measured speedup or Windows pass.

The coordinator adopts W11-W01–W11-W10 through
[development §8.5](plans/development-plan.md#85-windows-11-integration-research-and-qualification)
and records their current dispositions in the existing Windows 11 task. This
shares the checkpoint procedure used for the imported storage queue; it does not
change the current Core priority or activate held Windows/fuzz execution.

**Protected-storage second upstream review, 3 November 2026:** the
[new source supplement](plans/windows-protected-storage-upstream-lessons-2.md)
retains the completed second search and critical review. The
[agent task](../tasks/windows-protected-storage.md) turns the findings into bounded
source/contract preparation, future portable implementation and separately held
Windows qualification. Canonical clarifications distinguish operation outcome
from owner usability, validate opened objects before destructive effects, retain
native custody through recovery, and qualify individual CNG/mitigation operations.
RedirectionGuard remains a candidate; attestation, hardware revocation epochs,
chunked snapshots and database adoption remain conditional. The original ST-T,
CHECK and DNS-C owners, construction freeze, rollback residuals, experiment pause
and UI-M0/D2 boundaries remain. No implementation or native pass is claimed.

On continuation, use the [imported hardening work queue](plans/development-plan.md#imported-hardening-work-queue)
to adopt ST-W01–ST-W07 into the existing task chain. It distinguishes the ready
source-only assignment from later implementation and held native observations.

**Protected optional storage, 2 November 2026:** the [architecture amendment](windows-time-architecture.md#1212-protected-optional-storage-and-explicit-fallback)
records the reviewed CNG VBS/TPM/fresh-state/no-cache direction requested by the
owner. The [focused plan](plans/windows-protected-storage.md) preserves the
18-scenario threat model, independent A1–A5/HR-10 corrections, actual-key evidence,
signed exact snapshots, class-specific recovery, per-role mitigation and future
D2 notification gates. It supersedes the earlier DPAPI-first optional NTS/DNS
candidate, not Windows DPAPI itself. Exact algorithms/defaults/native results
remain open; storage experiments are paused. Current fixtures, UI-M0 and phase
priority remain unchanged. The subsequent [upstream implementation review](plans/windows-protected-storage-upstream-lessons.md)
records actual-key lookup/isolation candidates, ACL-preserving file replacement,
secret serialization barriers, cancellation/completion tests and conditional
enclave patterns. Source inventory is complete; exact design freeze and native
qualification remain open.

**System-wide DNS direction, 2 November 2026:** [OD-08](nevrotick-owner-decisions.md#12-od-08--system-wide-windows-dns-and-dnssec)
records the owner's intended Windows DNS service for the whole computer and its
applications, plus DNSSEC using hardware acceleration and multiple CPU cores.
The [detailed design](plans/system-dns-cache-design.md) distinguishes local DNS
ingress from pinned DoQ upstream transport and proposes protected demand
statistics, temperature classes, a bounded refresh scheduler and measurable
adoption criteria. Its DNS-C01–DNS-C18 cases are future evidence requirements;
no service, application coverage, benchmark or native qualification is claimed.
Current Windows work and the roadmap's KHR-0 ordering remain unchanged.
The [source-attributed supplement](plans/dns-upstream-lessons.md), revision 1.2,
records eighteen reuse lessons. Its DNSSEC/cache follow-up adds exact-content
validation reuse, chain/delegation evidence, shared prerequisites, active-material
custody, individual response construction, failure caching, optional SIEVE/sharding
and time/trust transitions. The main DNS plan, revision 1.4, distinguishes
chain-material expiry from already validated answers and qualifies recursive
key-prefetch applicability. This is future guidance with concrete ownership,
resource, routing and privacy scenarios, not imported code or executed acceptance.
The additional [Security-kit assessment](plans/dns-upstream-lessons.md#security-kit-follow-up-cache-contracts-and-regression-cases)
refines demand/inspection APIs, immutable publication, retained-memory/work budgets
and private events. It retains two source-derived capacity/async counterexamples
and concrete stronger test scenarios; no reproduction, benchmark or DNS-C pass
is claimed.

**Engine workspace integration, 1 November 2026:** the bounded loopback engine is
now the sixth main workspace member. Its
[current checkpoint](../tasks/engine-transport-pilot.md#main-workspace-checkpoint)
records actual GNU portable tests/doctests/optimized cases, both frozen dependency
rows, real controlled-loopback CLI execution and Windows cross-linking. The
[native checkpoint](../tasks/engine-transport-pilot.md#native-windows-cli-checkpoint)
adds eight actual ordinary-user Windows CLI cases while retaining an earlier
observer timing failure. Four additional
[public-API cases](../tasks/engine-transport-pilot.md#native-live-process-port-release-checkpoint)
observe port reuse while their Windows children remain alive. The
[S42 coverage](../tasks/engine-transport-pilot.md#s42-current-critical-file-coverage)
and [S43 mutation checkpoint](../tasks/engine-transport-pilot.md#s43-complete-current-engine-mutation-checkpoint)
pass for the recorded S40 candidate; those receipts do not qualify later
candidates. Native completion/heap/resource and production acquisition gates
remain open. This does not complete P1/P2 or introduce clock effects.

**Pure NTS-KE preparation, 2 November 2026:** bounded request encoding and
caller-buffer incremental framing now have
[recorded qualification checkpoints](../tasks/s0.1-protocol-comparison.md#incremental-framing-final-qualification):
host checks and tests, direct contracts, per-file line/mutation gates, bounded
instrumented fuzzing and Windows-target compilation. The
[inert NTS authenticator envelope checkpoint](../tasks/s0.1-protocol-comparison.md#inert-nts-authenticator-envelope--2-november-2026)
adds complete outer framing before original-input AAD/nonce/ciphertext views,
with per-file host gates and seven independent source-fault controls. These
inert owners provide no live TLS, authentication, endpoint authority or complete
NTS operation. The
development plan retains the native crypto/trust and complete-execution gates.
The [inner plaintext framing checkpoint](../tasks/s0.1-protocol-comparison.md#inert-nts-plaintext-framing)
adds complete bounded framing and immutable original-input views, with focused,
contract, line-coverage, mutation, fuzz and Windows-target compiler evidence.
It performs no decryption or authentication.

**Verus assessment, 1 November 2026:** the
[research report](supporting-reviews/verus-assessment.md) recommends deferring
integration until a bounded actual-code comparison demonstrates additional value.
A valuable contract and a justified benefit hypothesis can trigger that pilot;
prior Kani failure is not required. The report retains candidate properties, the
Kani alternative, proof/trust boundaries and failure triage. No application proof
was executed or new tool/CI gate adopted; the [S1.1c task](../tasks/s1.1-time-domain.md#verus-proposal-assessment--1-november-2026)
owns assessment history and review results.

**State-guided fuzzing experiment, 1 November 2026:** the
[completed experiment report](supporting-reviews/state-guided-fuzzing-experiment.md)
records four private defect variants, strict/effects oracles and a bounded
mutation comparison. Lifecycle mutation helped cancellation search but slowed
context-invalidation search; retain strict/baseline defaults and keep the new
operator optional. The experiment is closed; the existing product plan continues.
Detailed execution evidence remains in the [S1.1e task](../tasks/s1.1-deterministic-harness.md#bounded-fault-suite-and-mutation-follow-up-1-november-2026).

**Architecture contract assessment, 30 October 2026:** the [critical review](supporting-reviews/architecture-contract-review.md) distinguishes valid specification gaps from existing controls and stale external snapshots. [Canonical acceptance refinements](windows-time-architecture.md#2021-contract-review-acceptance-refinements), [hardening §9.10](windows-time-development-hardening.md#910-contract-review-qualification-matrix) and [the execution workstream](plans/development-plan.md#82-contract-review-workstream-and-interface-gates) cover ownership, durable correction, adjustment modes, trust lifetimes, fresh agreement and lifecycle/tooling evidence. Unresolved numerical and trust-policy decisions remain held; no native qualification is claimed.

**UI toolkit comparison, 30 October 2026:** the [Win32 versus egui report](supporting-reviews/ui-toolkit-comparison.md) summarizes measurements, tradeoffs and evidence limits; the [S0.5 execution record](../tasks/s0.5-ui-comparison.md#results) retains detailed inputs and receipts. David rejected the tested egui Glow/AccessKit configuration on memory consumption and closed the comparison; retain Win32 and the existing resource targets. This pilot establishes no full accessibility, sustained performance or product-graph admission.

**Protocol follow-up, 30 October 2026:** [NTP client behavior](windows-time-architecture.md#73-ntp-client-behavior) makes KoD support explicit through pinned Chrony/independent-codebase analysis and reuse. The [existing protocol task](../tasks/s0.1-protocol-comparison.md#kod-reuse-and-rfc-8915-alignment-follow-up) records RFC 8915 alignment and remaining implementation checks; [TLS policy](windows-time-architecture.md#1871-license-gate-and-finite-provider-choice) prioritizes AES-GCM while retaining provider/CPU qualification. These are P2 requirements, not implemented NTS/KoD support or measured AES speedup.

**Upstream code is not presumed bug-free or vulnerability-free.** Chrony and other reused projects, libraries and tools require independent verification of the behavior NevroTick relies on. See [architecture section 20.1](windows-time-architecture.md#201-independent-numerical-truth) and [the test and stress matrix](windows-time-development-hardening.md#97-verification-of-upstream-code-and-stress-testing).

**Upstream testing tools, 30 October 2026:** the [clknetsim/chrony-fuzz reference](../.agents/skills/testing/references/chrony-upstream-tools.md)
adds pinned scenario/seed reuse to the existing plan and skills. clknetsim supplies
reference trajectories through an explicit seam; chrony-fuzz supplies testcase
ideas from an AFL++ harness that deliberately simplifies protocol/crypto checks.
Neither is a native Windows harness or an independent authentication oracle.
The [protocol task follow-up](../tasks/s0.1-protocol-comparison.md#upstream-simulation-and-fuzz-tool-follow-up)
records source-only inspection and the next bounded qualification work.

**Chrony/NTPsec mechanism assessment, 30 October 2026:** the
[critical source assessment](../tasks/s0.1-protocol-comparison.md#chrony-and-ntpsec-mechanism-assessment)
supports targeted refinements for estimate lifecycle, bounded scheduling, route
relearning, atomic NTS publication/send enforcement and restoration validity.
The [existing workstream map](plans/development-plan.md#41-apply-the-chrony-and-ntpsec-mechanism-assessment)
keeps current S1.3 qualification before dependent controller integration. Chrony
remains the numerical reference; NTPsec supplements protocol comparison alongside
ntpd-rs. No upstream constants, additional controller or implementation pass follows
from this review; the relevant implementation/security/testing skills carry the checks.

**Development baseline, 30 October 2026:** The [Cargo workspace](../Cargo.toml) contains the fixture-only `nevrotick-ui`, dependency-free `ntp-wire`, checked numerical `time-domain`, single-slot `ntp-exchange` and pure fixture-regression `time-core` owners. [The tooling record](../TOOLING.md#starting-implementation) describes the verification entry points; [UI-M0](../tasks/ui-m0.md), [S1.1a fixed headers](../tasks/s1.1-header.md), [S1.1b framing](../tasks/s1.1-framing.md), [S1.1c arithmetic](../tasks/s1.1-time-domain.md), [S1.1d exchange association](../tasks/s1.1-exchange-association.md) and [S1.2 regression](../tasks/s1.2-regression-readiness.md) own candidate-specific results and gaps. The five-member graph retains the sixteen external package records from the UI-M0 cohort; it admits no production networking or TLS. The [planned runtime dependency refresh](windows-time-architecture.md#186-runtime-dependency-register) still leaves held AWS-LC/Schannel providers and the production graph unresolved.

**Native memory diagnostics, 29 October 2026:** Application Verifier, GFlags/PageHeap and CDB are installed and configured; healthy, heap-overrun and invalid-handle bootstrap controls passed. [Development hardening section 9.5](windows-time-development-hardening.md#95-fuzzing-and-native-memory-diagnostics) defines when agents must select corruption checks and separate retention measurements; [the shared skill workflow](../.agents/skills/testing/references/windows-memory.md) provides execution and cleanup rules. These are tool controls, not application leak/soak or supported-product qualification. Native runs remain owned manual checks until a separate CI lane is qualified.

**Performance guidance:** [Architecture section 17](windows-time-architecture.md#17-resource-and-performance-targets) owns the proposed budgets. Use [performance optimization](../.agents/skills/implementation/optimize-rust-performance/SKILL.md) to locate and improve a measured bottleneck, paired with [resource QA](../.agents/skills/testing/qa-performance-resources/SKILL.md) for comparable measurements and acceptance. This workflow preserves correctness/security and does not claim measured product performance before implementation.

**Application build optimization:** Release settings now explicitly select optimization level 3, ThinLTO and one codegen unit while retaining correctness checks, symbols and Windows protections. [Hardening section 6.5](windows-time-development-hardening.md#65-windows-x64-optimization-policy-and-evidence-limits) records the compiler-selfbuild Windows issue, CPU compatibility, PGO and BOLT limits; [the task record](../tasks/build-optimization.md) owns actual checks. These settings establish no measured speedup or new supported CPU floor.

The [LLVM settings matrix](windows-time-development-hardening.md#66-llvm-settings-for-this-workload) selects the bundled backend's optimized pipeline, ordinary floating-point behavior and rustc-managed Windows linker defaults. Its experiment controls preserve exact numerical outputs, latency, resource and power observations; raw LLVM option recipes are not adopted defaults.

**Required CPU optimization, 30 October 2026:** [OD-07](nevrotick-owner-decisions.md#11-od-07--x64-baseline-and-runtime-optimization) selects an intended `x86-64-v3` baseline and runtime-selected AVX-512 specializations, with exact feature/OS checks, numerical parity and measured benefit. The [research report](supporting-reviews/x86-runtime-optimization.md), [hardening §6.7](windows-time-development-hardening.md#67-v3-baseline-and-runtime-cpu-dispatch) and [required execution workstream](plans/development-plan.md#81-x64-baseline-and-runtime-optimization-workstream) distinguish the current generic build from the planned migration and native qualification. V3 already includes AVX2; AES remains a separate provider capability. This amendment changes no compiler flag or Rust implementation.

**Contract adoption, 29 October 2026:** [DEC-18 and section 6](windows-time-architecture.md#61-validation-architecture-and-enforcement-order) select garde for mandatory shared DTO/settings validation inside private fallible construction. [Every function/module contract](windows-time-architecture.md#631-contracts-for-every-function-and-module) must establish preconditions, checked outputs, failure/effect behavior and current-state obligations through types and the relevant checks. The [agent enforcement rules](windows-time-development-hardening.md#34-mandatory-contracts-for-agent-written-code) preserve one rule owner and prohibit construction bypasses. The UI-M0 model now implements checked preview construction using the admitted garde graph; its behavioral evidence remains distinct from the original design adoption and from future production contracts.

**Windows mitigation refinement, 29 October 2026:** [architecture section 15.4](windows-time-architecture.md#154-per-process-mitigation-profiles-and-startup-admission) defines process roles, activation limits, startup failure handling and the mitigation manifest; [hardening section 6.4](windows-time-development-hardening.md#64-per-process-mitigation-qualification) supplies the native qualification matrix. Existing CFG/ASLR/DEP configuration and bootstrap evidence are distinguished from pending application enforcement. Read these before adding process policies or treating every mitigation as a runtime switch.

**Diagnostics and shared Windows adapters, 29 October 2026:** [architecture section 3.4](windows-time-architecture.md#34-shared-windows-adapters-and-diagnostics) distinguishes reusable OS adapters, portable IPC contracts and per-process composition. [Section 12.3](windows-time-architecture.md#123-logging-and-support) owns logger structure, INFO defaults, authorized bounded DEBUG sessions, required events, redaction and resource/failure rules; [hardening section 9.8](windows-time-development-hardening.md#98-diagnostics-and-shared-adapter-verification) supplies the acceptance matrix. Those amendments remain component contracts; the fixture-only UI-M0 implementation does not implement the production diagnostics or shared service/IPC adapters.

**IPC external comparison, 29 October 2026:** [architecture section 5.4](windows-time-architecture.md#54-native-ipc-implementation-and-qualification) records refinements from Microsoft guidance and pinned Tokio/Mio, Kestrel and OpenVPN source: caller-token association/reversion, effective rights, continuous pipe ownership and cancellation/completion. [Hardening section 9.9](windows-time-development-hardening.md#99-ipc-identity-framing-and-native-lifetime-verification) maps them to existing native acceptance checks. The transport, runtime, dependency and UI-M0 decisions remain; external comparison is not implementation evidence.

**Current UI milestone:** [UI-M0, architecture §16.5](windows-time-architecture.md#165-restricted-first-ui-milestone-ui-m0), is a restricted native fixture-only prototype. D1 display/theme, D2 production service/policy-state and D3 full keyboard/Narrator/UIA work are explicitly **Deferred**, not prerequisites for UI-M0. Restore them at their specified integration/preview gates. Other workstreams and eventual product/release requirements remain unchanged; completing UI-M0 does not complete P0 toolkit qualification.

**Latest owner instructions:** [nevrotick-owner-decisions.md](nevrotick-owner-decisions.md) records accepted reuse/recovery/release choices and unresolved default-profile policy. [nevrotick-research-handoff.md](nevrotick-research-handoff.md) assigns the remaining investigations, including the supplied NTS directory and proposed operator candidates. Read these before interpreting older unconditional signing language in retained historical reviews.

**Engine runtime clarification, 29 October 2026:** [architecture section 18.6.1](windows-time-architecture.md#1861-engine-runtime-decision-and-alternatives) records the rationale, component boundaries and reconsideration criteria for the existing minimal current-thread Tokio prototype. Section 18.7.1 keeps a bounded synchronous NTS-KE worker available alongside the held async transport candidate. Runtime choice establishes no measured resource, security or timing result; existing dependency and Windows qualification gates apply.

**Deterministic fixture work, 30 October 2026:** [S1.1e](../tasks/s1.1-deterministic-harness.md) records its qualified private driver/oracle candidate; its reviewed test delta is in the shared checkout. [S1.2](../tasks/s1.2-regression-readiness.md) records the qualified private coefficient, residual, normalization and atomic-history candidates, their independent expectations and actual test, per-file analysis, fuzz and cross-build evidence. The reviewed core and current build-policy merge are now in the shared checkout. The exact integrated staging candidate passed 191 ordinary tests, 53 doctests and 164 optimized tests, Linux and Windows dependency checks, and a Windows cross-build. Native Windows behavior, production integration and full S1.2/P1 remain incomplete.

## Reading guide

| Order | Document | Scope and status |
| --- | --- | --- |
| A | [nevrotick-owner-decisions.md](nevrotick-owner-decisions.md) | Owner instructions of 29 October; accepted choices, recommendations and unresolved policy distinguished. |
| B | [nevrotick-research-handoff.md](nevrotick-research-handoff.md) | Bounded future research, NTS operator candidates, evidence requirements and owner decision cards. |
| 1 | [windows-time-features.md](windows-time-features.md) | V1 product feature plan: ten capabilities, main journeys, authority, recovery, twenty acceptance scenarios and staged delivery. |
| 2 | [windows-time-architecture.md](windows-time-architecture.md) | Canonical creation specification, revision 1.4: technical contracts, threat model, NTP/NTS, Windows authority and release gates. |
| 3 | [windows-time-diagrams.md](windows-time-diagrams.md) | Editable Mermaid architecture views aligned with revision 1.4. |
| 4 | [windows-time-development-hardening.md](windows-time-development-hardening.md) | Strict Rust/Windows agentic development policy, CI, auditors, tool qualification and evidence requirements. See the current tooling record for adopted bootstrap checks; application/native/release qualification remains pending. |
| 5 | [asvs-5.0.0-applicability.md](asvs-5.0.0-applicability.md) | All 345 ASVS 5.0.0 requirements mapped to proposed applicability/verification; not completed compliance. |
| 6 | [windows-time-signing-guide.md](windows-time-signing-guide.md) | Practical MSI signing application, artifact workflow and verification guide; provider acceptance remains open. |
| 7 | [windows-time-update-assessment.md](windows-time-update-assessment.md) | TUF/update research and constraints; no automatic updater or TUF architecture has been adopted. |
| 8 | [windows-time-review.md](windows-time-review.md) | Independent architecture review and revision 1.4 reconciliation. Its earlier reviewed-baseline label is intentional. |

Accepted owner decisions govern their stated scope; recommendations and open questions remain proposals. The architecture controls the corresponding technical/security contracts. The feature plan supplies proposed presentation and product scope; unresolved provider, numerical, Windows, licensing and signing decisions retain their gates. The complete application remains one MSI. The preferred Authenticode route and owner-approved initial OpenPGP alternative have separate evidence and Windows compatibility claims; neither is qualified by this documentation.

## UI design and brand references

David has designated these existing bundles as references for agents working on
the UI. Use design v0.2 for screen composition and interaction flows, and the AVIF
kit for brand artwork and variants.

| Reference | Start here |
| --- | --- |
| Design guidance | [Design v0.2 README](nevrotick-design-v0.2/README.md), then [UI specification](nevrotick-design-v0.2/docs/02-ui-spec.md). |
| Visual and interaction examples | [Interactive HTML preview](nevrotick-design-v0.2/NevroTick-UI-preview.html) and [screen/scene catalogue](nevrotick-design-v0.2/screens/README.md). |
| Brand assets | [AVIF kit README](NevroTick-AVIF-kit-v1/README.md) and [asset manifest](NevroTick-AVIF-kit-v1/manifest.json), including transparent, light/dark and size variants. |
| Asset rights and provenance | [Brand notice](NevroTick-AVIF-kit-v1/BRAND-LICENSE.txt) and the kit's per-image metadata; keep these separate from the application source-code licence. |

Apply the references within the current UI-M0 scope and D1–D3 deferrals. The React
viewer illustrates the design; its validators, dependencies and mock outcomes do
not define the Rust/Win32 implementation. Rendered HTML/screenshots may predate
specification corrections. The live canonical architecture and accepted owner
decisions retain precedence over archived copies in the design bundle.

## Supporting reviews and comparison

These preserve first-pass findings and correction checks, not competing specifications or claims of runtime qualification.

- [tuf-protocol-review.md](supporting-reviews/tuf-protocol-review.md): protocol, time assurance and recovery.
- [tuf-dependency-review.md](supporting-reviews/tuf-dependency-review.md): Rust clients, crypto graphs and evidence limits.
- [tuf-windows-review.md](supporting-reviews/tuf-windows-review.md): authority, staging, MSI handoff and lifecycle.
- [signing-guide-review.md](supporting-reviews/signing-guide-review.md): provider onboarding and signing-guide corrections.
- [development-compiler-review.md](supporting-reviews/development-compiler-review.md): compiler/lint/profile and Windows mitigation review.
- [development-evidence-review.md](supporting-reviews/development-evidence-review.md): CI trust, isolation, mutation contract and correction verification.
- [basic-features-review.md](supporting-reviews/basic-features-review.md): integrated product feature, UX, authority and acceptance review.
- [ui-toolkit-comparison.md](supporting-reviews/ui-toolkit-comparison.md): measured Win32/egui pilot, dependency concerns and the owner's decision to retain Win32; detailed execution stays in the S0.5 task.
- [x86-runtime-optimization.md](supporting-reviews/x86-runtime-optimization.md): v3 baseline and ahead-of-time runtime dispatch, precise feature/OS gates, numerical constraints, compatibility and benchmark qualification; implementation follows OD-07 and the existing build task.
- [architecture-contract-review.md](supporting-reviews/architecture-contract-review.md): critical disposition of the external assessment, primary-source basis, contract amendments and remaining decision/native-evidence gates.
- [state-guided-fuzzing-experiment.md](supporting-reviews/state-guided-fuzzing-experiment.md): completed four-defect mutation/oracle comparison, retained defaults, per-class results, reusable lessons and frozen evidence references.
- [verus-assessment.md](supporting-reviews/verus-assessment.md): source-based evaluation, deferred integration, candidate properties, comparison with Kani, proof boundaries and conditions for a future pilot.

Some records retain workspace paths, initial hashes and source identifiers for provenance. Read correction addenda alongside the first-pass findings. Main documents control current requirements; the tooling record identifies completed tool controls separately from pending application, native-product and signing qualification.

## Format and integrity

All documents are UTF-8 Markdown. Mermaid remains editable fenced source. Keep the extracted directory together for relative document links.

An exported bundle may supply `SHA256SUMS.txt` for byte-integrity comparison; this checkout currently has no root documentation manifest, so it establishes no bundle-integrity result. The design subdirectories maintain their own separate manifests. Checksums do not authenticate a publisher or replace a digital signature.

Older PDF/CSV exports, intermediate candidates, helper scripts, credentials and executable tools are excluded. No application build, Windows installation, clock change, scanner execution, signing request or conformance test was performed to create the feature-plan addition.

