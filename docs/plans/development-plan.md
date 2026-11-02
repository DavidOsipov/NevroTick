# NevroTick development and acceptance plan

**Date:** 29 October 2026  
**Status:** reviewed; ready to assign the next bounded task; implementation phases remain open  
**Inspected source baseline:** `f7ba1e7724a41cc846b1df6b9ddb20f095988b7a`, initially clean  
**Immediate priority:** secure, bounded and performant NTP/NTS acquisition, with independently verified algorithms  
**Current task:** [S1.3 source-selection fixture](../../tasks/s1.3-source-selection-readiness.md), following the separately qualified bounded [main S1.2 runs integration](../../tasks/s1.2-regression-readiness.md#main-fixture-qualification-result-30-october-2026). The [92-input projected-batch host-fixture result](../../tasks/s1.3-source-selection-readiness.md#projected-batch-host-fixture-result-31-october-2026) records the borrowed common-context consumer, 254 ordinary/56 doctests/227 optimized executions, all nine line gates, 717 native mutation variants at ten workers, actual CLI union, contracts and bounded history semantic fuzz. The newer [principal receiver checkpoint](../../tasks/s1.3-source-selection-readiness.md#principal-receiver-checkpoint-31-october-2026) has fourteen principal cases and passed the complete portable row at 268 ordinary/56 doctests/241 optimized executions, both selected line-coverage gates and scoped contracts. Its two exact 96-input mutation cohorts and actual CLI union passed all 327 generated variants at ten workers, with one reviewed equivalent disposition; earlier failed campaigns are retained. The [89-input enclosure result](../../tasks/s1.3-source-selection-readiness.md#enclosure-host-fixture-result-31-october-2026), [86-input projection result](../../tasks/s1.3-source-selection-readiness.md#projection-host-fixture-result-31-october-2026) and earlier [six-owner checkpoint](../../tasks/s1.3-source-selection-readiness.md#host-fixture-checkpoint-31-october-2026) remain historical. Preserve the qualified inputs before further edits. The [expanded 106-input combination-lifecycle checkpoint](../../tasks/s1.3-source-selection-readiness.md#expanded-lifecycle-controls-and-portable-checkpoint--1-november-2026) now has twelve public lifecycle trajectories and two private-boundary controls, with 298 ordinary/56 doctests/271 optimized portable executions. The [V5 lifecycle host-fixture checkpoint](../../tasks/s1.3-source-selection-readiness.md#v5-lifecycle-checkpoint--1-november-2026) now passes all six selected line gates, per-file mutation gates (747 variants with seven independently reviewed equivalences), scoped contracts and the actual CLI union. All 753 original invocations were reconciled by a separately qualified report receiver without rerunning Rust; the two original receiving failures remain retained. Historical coverage/results do not qualify the newer candidate; macro-validator controls and production eligibility remain open. The [inert NTS-KE framing owner](../../tasks/s0.1-protocol-comparison.md#bounded-raw-framing-host-checkpoint) is now implemented in `ntp-wire`; its frozen 108-input checkpoint passed portable, targeted release, external caller, all-three-owner line coverage, per-file mutation, bounded NTS-specific fuzz and Windows cross-build checks. The [pending NTPv4 negotiation owner](../../tasks/s0.1-protocol-comparison.md#pending-ntpv4-negotiation-implementation--1-november-2026) is implemented with its frozen 110-input host checkpoint, including the complete portable row, wire release tests, external callers, per-file line/mutation gates, bounded four-owner semantic fuzz and Windows cross-build passed. TLS/stream integration and native Windows execution remain open. Preserve observation ages and unresolved production policy. The required [OD-07 CPU workstream](#81-x64-baseline-and-runtime-optimization-workstream) remains owned by its controlled build-policy checkpoint and measured runtime specialization. Full S1.2/S1.3/P1, native Windows and release gates remain open.

The current [engine qualification work](../../tasks/engine-transport-pilot.md#s40-mandatory-completion-owners-and-current-native-baseline) overlaps P1 preparation for S2.1. Its 119-input candidate gives native admission/completion and argument budgets mandatory private owners. It passes 24 GNU unit/integration tests plus two doctests in ordinary and optimized builds, with compiler/Clippy, and 213 verification-harness controls. Independently reviewed enumeration binds 185 library and 31 CLI variants. [Current line coverage](../../tasks/engine-transport-pilot.md#s42-current-critical-file-coverage) passes at 95.929% library and 100% CLI. Both [complete per-file mutation gates](../../tasks/engine-transport-pilot.md#s43-complete-current-engine-mutation-checkpoint) pass with all 81 library and 20 CLI viable variants caught, zero survivors/timeouts/exclusions, and ten observed Cargo workers. The earlier [library campaign](../../tasks/engine-transport-pilot.md#s39-library-faults-and-next-contract-slice) exposed 22 misses and two fixture timeouts, which motivated this bounded slice. Historical coverage/mutation receipts retain their original candidates; full P1/P2 and native completion remain open.

The [frozen TLS source recovery](../../tasks/s0.1-crypto-feasibility.md#frozen-source-recovery--1-november-2026) restores all 61 external metadata-package archives and source maps with unchanged versions, features and byte-identical metadata. The [bounded helper inspection and frozen advisory/source row](../../tasks/s0.1-crypto-feasibility.md#source-helper-inspection-and-frozen-advisorysource-row--1-november-2026) are complete. The [concrete CNG fixture checkpoint](../../tasks/s0.1-crypto-feasibility.md#s48s50-concrete-cng-fixture-and-build-input-admission--1-november-2026) adds a separate GPL-2.0-only native fixture, freezes its 33-external-package selected graph without new versions, and passes its full dependency policy with exact file-hash controls. Its bounded Windows cross-build is next; native CNG/TLS, the broader provider graph and product adoption remain unqualified.

Agent navigation: [implementation contracts](#3-mandatory-explicit-contracts-during-implementation),
[phase skill map](#5-required-skill-reading-and-use-by-phase),
[phase acceptance](#6-phase-implementation-and-acceptance-gates),
[next task](#7-next-executable-task-for-a-development-agent),
[protected-storage task](../../tasks/windows-protected-storage.md),
[Windows 11 integration task](../../tasks/windows11-integration.md),
[evidence and stop rules](#9-verification-commands-phase-records-and-stop-rules).

## 1. Purpose, authority and current state

This is the execution roadmap requested by David for subsequent development
agents. It adds an ordered backlog, mandatory skill-reading map, explicit
implementation contracts and phase acceptance gates. It does not replace the
[architecture](../windows-time-architecture.md), [owner decisions](../nevrotick-owner-decisions.md),
[feature plan](../windows-time-features.md) or their requirement IDs. Read the
[documentation guide](../README.md) first. Preserve `REQ`, `BND`, `DEC`, `THR`,
`CHECK`, `AC`, `M0`, `OPEN` and `OD` meanings; P0–P7 below are the existing
architecture phases, and S-0–S-6 are the existing integrated delivery slices.

The original request authorized planning/documentation. David's subsequent
whole-plan implementation goal authorizes progress through P0–P7, recorded in
section 10 and the linked task chain. Routine work within accepted contracts needs
no repeated approval. Material unresolved policy choices use the decision-card
procedure in NV-DEC-001; block only dependent work. Public release retains its
exact-candidate approval gate.

| Area | Observed repository state | Consequence for the next agent |
| --- | --- | --- |
| Cargo | [Workspace](../../Cargo.toml), lockfile, fixture [UI](../../crates/nevrotick-ui/Cargo.toml), inert [wire](../../crates/ntp-wire/Cargo.toml), checked [time-domain](../../crates/time-domain/Cargo.toml), fixture [exchange](../../crates/ntp-exchange/Cargo.toml), pure [time-core](../../crates/time-core/Cargo.toml) and bounded [engine](../../crates/nevrotick-engine/Cargo.toml) owners exist. | Inspect current files and linked task evidence; fixture numerical source and loopback acquisition do not establish the complete core or production acquisition. |
| UI | `main` → native adapter → checked fixture model → standard Win32 controls. Preview edits are transient. | Keep UI-M0 isolated; no network, production IPC, persistence, service, elevation or clock effects. |
| UI evidence | [UI-M0 task](../../tasks/ui-m0.md), including later candidate addenda, records portable, cross-build and native observations. Full required native-memory instrumentation remains incomplete. | Historical results belong to their exact candidates. Short interaction/resource observations do not establish CHECK-21 soak or full UI qualification. |
| Production software | Header/framing, checked arithmetic, fixture exchange, pure regression/history/selection and a two-peer loopback engine exist. Production sample admission, acquisition, broker, SCM services, production IPC and MSI remain incomplete. | Follow exact-candidate task receipts; create only what an authorized slice needs. |
| Dependencies | The six-owner workspace retains the reviewed UI-M0/garde cohort and adds the admitted minimal Tokio/getrandom engine graph. Exchange and core keep reviewed local edges; production TLS/provider/AEAD and service graphs remain gated. | The [engine dependency checkpoint](../../tasks/engine-transport-pilot.md#current-acquisition-evidence--1-november-2026) owns its selected graph rows. Integrated checks apply to their recorded source/features. |
| Windows 11 integration research | [Completed Microsoft-source investigation](windows11-integration.md) and [bounded agent assignments](../../tasks/windows11-integration.md) refine existing platform, DNS, lifecycle and resource owners. | Source-only follow-up is ready when its slice is assigned; documentation establishes no implemented mechanism, native support or installed-MSI result. Current task ordering and the native-experiment pause remain. |

Keep one evidence chain: this plan records phase readiness and links to concrete
task evidence; existing UI work stays in `tasks/ui-m0.md`. New development tasks
may have bounded records under `tasks/`, linked here, without duplicating the
architecture. Raw generated logs/builds belong in owned external output locations
or the architecture's phase evidence location, not in the maintained design bundle.

## 2. Development order and ownership

Prioritize ordinary, correct-date NTS communication and explicitly selected
plain-NTP observation. Large-date recovery, real clock correction and finished UI
follow their own gates. A useful headless observe-only client is an early outcome;
it must not be labelled the complete S-2 slice, which also requires secured
service status and a real read-only UI.

1. **P0 foundations:** start S0.1 provenance/provider feasibility and freeze the
   next pure input/numerical contracts. Run independent Windows experiments in
   parallel when their environments and temporary bounds exist.
2. **P1 deterministic core:** implement admission/parsers first, then regression,
   selection/combination and abstract discipline in separately tested slices.
3. **P2 acquisition:** bounded UDP/DNS and NTP observation, then ordinary NTS-KE
   and authenticated packets, then qualified cache/recovery behavior. Fixture
   preparation can overlap P1; do not claim P2 integration before its prerequisites.
4. **P3 services and secured state:** integrate final service identities, IPC,
   protected configuration and diagnostic status using a simulated clock backend.
5. **S-2 observe-only integration:** P2 + P3 + early P5 status UI, with D2 restored.
6. **P4 clock integration:** real bounded correction, ownership, recovery and
   lifecycle in disposable Windows environments.
7. **P5 finished UI; P6 complete MSI:** finish user journeys, accessibility,
   deployment protections and profile-specific artifact trust. Early P0 UI and
   package feasibility experiments precede these full implementations.
8. **P7 exact-candidate qualification:** reconcile all required evidence and
   obtain David's approval before public release.

P1 entry requires the S0.1 decisions relevant to its selected source/graph and a
frozen numerical/input contract. It does not wait for every unrelated P0 experiment
or UI-M0 completion. A completed pure slice does not close unrelated S0.1 TLS,
licensing or publisher work. Unresolved crypto/trust gates stop dependent NTS
integration, not read-only source analysis or independent pure work.

| Owner | Producer → state → consumer/effect | Boundary |
| --- | --- | --- |
| Domain/protocol owner | Raw bounded bytes → inert records → checked values/observations | No sockets, ambient clock, Windows calls or privileged effects in the pure core. Binary NTP/NTS has protocol-specific parsers. |
| Numerical owner | Admitted observations → bounded history → regression/majority/combination → estimate/proposal | An estimate or proposal is not authority and is not an applied correction. |
| Engine owner | Admitted endpoints → DNS/UDP/TLS/native jobs → authenticated observations → core | Network-facing, no clock privilege; engine-only bounded runtime. |
| Broker owner | OS-authenticated request + current policy/ownership/budgets → serialized guard/effect → actual acknowledgement | Sole clock-effect owner; hostile authorized engine is part of the threat model. |
| Platform owner | Narrow role-specific Windows adapter → real token/handle/SCM/clock result | Shared `windows-platform` code grants no generic privilege or command surface. |
| UI owner | Coherent admitted snapshot → presentation; user intent → independently authorized receiver | Normal user, no direct clock/service-policy writes; UI checks convey no authority. |
| Lifecycle/release owner | Approved complete package + protected journal → install/maintenance/restoration | One per-machine MSI, both services, UI and narrow helper; no separate engine prerequisite. |

One worker owns each shared schema, `Cargo.lock`, mutable Windows fixture,
installer identity and evidence output. Delegations name canonical sections,
acceptance IDs, candidate, writable paths, expected observations and stop bounds.
The coordinator verifies the integrated candidate, not only worker summaries.

## 3. Mandatory explicit contracts during implementation

Every new or changed **module, function, private helper, method, trait
implementation and FFI callback** needs an explicit contract before or with its
implementation. Read [implement-rust], [implement-rust-contracts], the
[implementation contract](../../.agents/skills/implementation/CONTRACT.md) and
[contract-enforcement reference](../../.agents/skills/implementation/implement-rust-contracts/references/contract-enforcement.md).
Place contracts beside their source owners; small helpers can refer to shared
invariants and typed signatures rather than duplicate a large template.

| Contract item | Required content and enforcement |
| --- | --- |
| Inputs | Exact units/sign, range, representation, identity/provenance, ownership and allowed states. Distinguish UTC, monotonic, active-awake, NTP-era timestamps, phase and frequency. Positive offset means Windows is behind. |
| Preconditions | Which immutable private types establish stable facts; which current policy, generation, freshness, destination, ownership and cumulative-budget checks remain necessary. |
| Successful outputs | Checked units/ranges, finite numeric values, invariants, exact state transition and guarantees to downstream consumers. Check transformations before publishing results. |
| Failure | Typed useful error, bounded redaction, preserved prior valid state, no partial protected effect; ambiguous effects enter reconciliation rather than blind retry. |
| Effects | Enumerate permitted network/file/state/clock effects or explicitly state purity. Identify the sole effect owner and actual acknowledgement. |
| Resources/lifecycle | Allocation/queue/byte/job/handle bounds; native destructor; cancellation, partial construction, stop/restart and eventual completion ownership. |
| Evidence | Requirement/CHECK IDs, actual consumer, independent valid and invalid cases, bypass/transition tests and required host/native environment. |

For JSON/configuration/persisted records, use bounded strict Serde decoding into
inert DTOs and mandatory shared garde/domain validation **inside private fallible
construction**. The consumer uses the checked result. No normalization through
`serde_json::Value`, optional caller-only validation, unrestricted domain
deserialization, unchecked factory/mutation or public raw fields may bypass it.
`garde(allow_unvalidated)` is prohibited; each `skip` needs its actual invariant
owner and independent review. Binary packet admission is not a JSON-schema task.

Required rejection must work in release builds. Comments, annotation presence,
`debug_assert!`, unchecked casts, silent clamps, fallback success and ignored
errors cannot enforce the contract. Reuse unchanged immutable validated values;
recheck transformed/external results and facts invalidated by an await, policy
change, reconnect, suspend or generation change. Serialize current guards with
the protected effect.

Use [qa-contract-tests] for external-consumer compile-fail cases paired with
compiling legal use, direct-handler bypasses, invalid mutations/migrations,
overflow/non-finite outputs and stale-context transitions. Inspect defining-module
bypasses as well as external privacy. A valid constructor does not permanently
authorize a clock operation. Native adapters require narrow unsafe obligations,
correct cleanup and native lifetime evidence.

## 4. Evaluate upstream code before reuse or replacement

OD-01 selects a source-derived Chrony port with independent repairs and measured
optimization. Keep the canonical Chrony **4.9** reference commit
`8df4f1263e206585e6c4e61352d042c2dd12916a`; do not silently advance it. Read the
[Chrony port/simulator recipe](../../.agents/skills/implementation/implement-rust/references/chrony-port-simulator.md),
[architecture port map](../windows-time-architecture.md#71-port-map-and-retained-behavior)
and [upstream verification/stress policy](../windows-time-development-hardening.md#97-verification-of-upstream-code-and-stress-testing).

| Behavior to inspect | Primary Chrony files | Additional comparison purpose |
| --- | --- | --- |
| Packet validation and polling | `ntp_core.c`, `ntp.h`, `ntp_ext.c`, `ntp_auth.c` | Independently derive protocol vectors; compare another implementation's rejection and correlation behavior. |
| Kiss-o'-Death control and recovery | `ntp_core.c`, `ntp_sources.c`, `nts_ntp_client.c`, relevant scheduler/callers/tests | Inspect pinned Chrony and at least one of ntpd-rs/NTPsec; reuse/adapt qualified `RATE`, `DENY`/`RSTR`, unknown-code and NTS `NTSN` behavior. Reconcile RFC 5905 erratum 3007, RFC 8633 §5.4 and RFC 8915 §5.7 before freezing bounds and recovery policy. |
| NTS exchange and association | `nts_ke_client.c`, `nts_ke_session.c`, `nts_ntp_client.c`, `nts_ntp_auth.c` | Compare pinned ntpd-rs/NTPsec behavior and controlled interoperability; review maintained TLS/AEAD library code. |
| Regression/history | `sourcestats.c`, `regress.c` | Analytic trajectories, independent high-precision reference and bounded differential traces. |
| Source selection/combination | `sources.c` | Verify interval majority, minimum agreement, principal hysteresis and distinct offset/frequency weighting. |
| Discipline/accounting | `reference.c`, `local.c`, `sys_generic.c` | Compare requested versus acknowledged corrections; replace Unix integration with qualified Windows adapters. |
| Source lifecycle | `ntp_sources.c`, scheduler/resolver paths | Inspect endpoint changes, cancellation, retry, identity and generation ownership. |

Use the [pinned upstream testing-tool reference](../../.agents/skills/testing/references/chrony-upstream-tools.md)
alongside that process. P1/S1.4–S1.5 can select a small clknetsim/Chrony trajectory
cohort through the existing pure event seam; compare healthy/asymmetric-delay/
majority-loss cases with independent truth and a challenged comparator. P1/P2
can adapt relevant chrony-fuzz testcase ideas to existing Rust targets after
rights, framing, forwarding/reset and finite-resource qualification. Its patched
origin/authentication/SIV/TLS outcomes supply no acceptance oracle. Keep the
current Chrony pin, admitted libFuzzer lanes, 12-core tool budget and narrower
lane limits. CHECK-6/7/8/10/19/20/25/26/27 keep their existing stage meanings;
source-only review neither runs these tools nor closes an implementation gate.

For each bounded component, the implementer must:

1. Read the exact source bodies, relevant callers, tests, notices, known issues
   and fixes. Evaluate at least one credible alternative where it can change the
   reuse decision; ntpd-rs and NTPsec are comparison candidates, not admitted
   dependency graphs or replacement controllers. Record unavailable evidence.
2. Write the required behavior and independent oracle **before** translating or
   changing it. Do not use the production algorithm to generate its own expected
   answers. Chrony parity and upstream test suites are supplemental evidence.
3. Record a port ledger: project/version/immutable revision, URL/hash, file and
   function, grant/notices, destination owner, retained behavior, test provenance,
   suspected defects, deviations and last upstream review. Keep corresponding
   source/build obligations and brand-asset notices separate.
4. Choose and justify reuse/translation, adapted reuse, or a new implementation
   against correctness, attack surface, resource bounds, Windows fit and
   maintenance. “New Rust” or “widely used upstream” proves neither safety nor
   better performance. Preserve derivation history and the GPL-2.0-only decision;
   rewriting or moving code to another process is not an automatic licence escape.
5. Preserve a reproducer for a bug and investigate differential disagreement:
   port, reference, fixture or assumptions may be wrong. Make a minimal causal
   repair and establish independent RED/healthy-control/GREEN where practical.
6. For a claimed improvement, use [optimize-rust-performance] and
   [qa-performance-resources]: predeclare metric/guard metrics, compare frozen
   baseline/candidate on identical useful workloads, retain raw samples and
   reject unsupported gains. Never weaken validation, precision, uncertainty,
   resource caps or Windows protections to win a benchmark.

Reused and newly written code both need varied applicable unit, contract,
property, differential, fuzz/adversarial, lifecycle, fault/recovery, load and soak
evidence. No project is presumed bug-free or vulnerability-free. Material changes
to algorithm semantics, trust, requirements or incompatible reuse use OD-04;
routine compatible repairs within accepted contracts continue.

Primary reading for protocol facts: [RFC 5905](https://www.rfc-editor.org/rfc/rfc5905.html),
[RFC 8915](https://www.rfc-editor.org/rfc/rfc8915.html), their current errata, the
[Chrony project](https://chrony-project.org/) and
[ntpd-rs documentation](https://docs.ntpd-rs.pendulum-project.org/).
Pin actual implementation sources during each evaluation. This plan's reference
check is not a source audit or dependency admission.

### 4.1 Apply the Chrony and NTPsec mechanism assessment

The [source assessment](../../tasks/s0.1-protocol-comparison.md#chrony-and-ntpsec-mechanism-assessment)
retains Chrony for the coherent estimator/controller and adds NTPsec as a bounded
protocol/operational comparator alongside ntpd-rs. Agreement among these projects
is supplemental evidence, not independent numerical or RFC truth. Keep current
S1.3 qualification first; the external proposal's priority order does not move
discipline ahead of its selection prerequisites or reopen qualified pure kernels.

| Existing workstream / owner | Added integration obligation and independent observation |
| --- | --- |
| S1.2 consumers → S1.3/S1.4, numerical owner | Carry estimate reference time, observation identity/age and distinct uncertainty terms through common-time projection. Repeated selection creates no fresh evidence; uncertain updates and drift changes have declared trajectories. CHECK-7/8/14. |
| S1.3 selection, then S1.4 discipline | Finish eligibility/majority → principal hysteresis → compatible combination before correction. Compare stable and remapped physical traces for known/refused/unknown effects; distinguish isolated anomalies from fresh sustained agreement without adopting another controller. CHECK-7/8/10. |
| P1 model → S2.1 transport, numerical/engine owners | A permanent admissible route-delay increase plus spikes reaches bounded relearning/reacquisition; a route exceeding fixed limits stays diagnosed unavailable. Qualify association port isolation/lifecycle under §7.3. No threshold relaxation or copied quantile default. CHECK-6/7/11/27. |
| S2.1–S2.4, engine/protocol owners | One scheduling policy enforces per-operation earliest times, per-source/aggregate budgets and current generations. Flapping/refresh/KE success with failed protected UDP cannot create storms or reset retry success incorrectly. CHECK-6/9/21/27. |
| S2.2–S2.4, protocol/security owners | Fragment/coalesce KE records, preserve complete-message atomic publication, and inspect actual outgoing packets through cookie exhaustion, failed/cancelled KE and restart. A failed replacement preserves only still-valid old state; an NTS-required sender cannot emit plain fallback. CHECK-6/9/17/27. |
| S2.3/P3/P4, persistence/platform owners | Qualify [restoration validity](../windows-time-architecture.md#1211-restoration-validity) independently for drift, optional history and NTS state, including a healthy warm restart. Existing bounded status and diagnostic budgets remain required. CHECK-9/11/14/15/17/25. |

Before each dependent implementation, freeze scenario bounds, expected transitions
and source/function provenance in its existing task. This amendment selects no
new numerical constants, crypto graph, full-history persistence or Windows effect.

## 5. Required skill reading and use by phase

The repository's skills supply implementation and verification methods. Reading
them is mandatory where assigned below; successful execution and evidence, not
reading alone, establish quality. Prefer these repository skills over overlapping
plugins. Read full `SKILL.md` files and their triggered references, not only names
or summaries. Reuse already-read unchanged guidance within a continuing task.

### Common requirements for every phase

- The phase coordinator reads [plan-code-change], [qa-environment-preflight],
  [qa-test-planning], [qa-regression-selection] and the
  [testing contract](../../.agents/skills/testing/CONTRACT.md) before selecting work
  and checks. Use [qa-test-orchestrator] when coordinating the phase's multiple
  evidence layers. Each worker reads the skills for its own changed guarantee.
- Every Rust implementer reads the two implementation skills and contract
  reference in section 3, plus [qa-contract-tests] and [qa-static-checks]. Before tests, read [qa-positive-paths],
  [qa-negative-paths] and [qa-boundary-edge-cases] to establish useful accepted
  controls, rejection cases and exact boundaries. These inform case design;
  they do not require three separate test suites.
- After **every phase**, including P0 experiments and intermediate executable
  slices, use the applicable testing skills in the table, then
  [qa-final-qualification]. Every full phase exit also uses
  [qa-independent-review] against the integrated candidate. A substantial plan
  or revised gate uses [plan-independent-review] before dependent implementation.
- Record skills read, why each applies, commands/observations, selected and
  executed counts, first failures, actual results and gaps. Missing required
  environment or failed/skipped/cancelled checks leave the affected phase open.
  Continue independent work; do not claim a phase exit or silently waive a gate.
- Security work also reads the [security contract](../../.agents/skills/security/CONTRACT.md).
  Qualification of native enforcement, protected persistence/resources, or
  firewall/service/event integration loads respectively
  [native proof](../../.agents/skills/security/references/native-proof.md),
  [persistence and resources](../../.agents/skills/security/references/persistence-and-resources.md),
  or [Windows host integration](../../.agents/skills/security/references/windows-host-integration.md).

### Phase-specific mandatory reading

Each row adds to the common requirements. Security/implementation leaves apply
to the owner performing the named subtask; a protocol worker need not load an
unrelated UI leaf. Testing leaves are used at the corresponding acceptance seam
and again at closeout when relevant evidence has changed.

| Phase | Before implementation or investigation | Required testing skills for its exits |
| --- | --- | --- |
| **P0 — foundations** | [plan-architecture], [plan-threat-model]; S0.1: [dependency-updates], [security-build-deployment], [security-crypto-secrets]; S0.2/4: [security-windows-privileges] and service adapter reference; S0.3: [security-local-ipc]; S0.5: [implement-native-ui], [security-native-ui]; S0.6: [security-build-deployment]; S0.7: affected security owner. | [qa-static-checks] for graph/tool gates; [qa-security-boundaries], [qa-integration-tests], [qa-state-concurrency-lifecycle], [qa-fault-recovery] for native experiments; [qa-performance-resources] for baseline budgets; S0.5 [qa-accessibility]/[qa-visual-dpi]; S0.6 [qa-package-artifacts]. Review source-only S0.1 as source evidence, not a native pass. |
| **P1 — deterministic core** | [security-protocol-inputs], [security-build-deployment]; [dependency-updates] for new graph; Chrony port/simulator and numerical/protocol references. [plan-architecture] fixes domain/state owners; section 3 contracts apply throughout. | [qa-unit-tests], [qa-component-tests], [qa-contract-tests], [qa-property-based-testing], [qa-golden-fixture-parity], [qa-adversarial-inputs], [qa-structured-fuzzing], [qa-static-checks], [qa-coverage-gap-analysis], [qa-mutation-testing]; [qa-model-based-testing] for abstract discipline/holdover transitions. Apply numerical/resource measurements to actual core workloads. |
| **P2 — NTP/NTS acquisition** | [security-protocol-inputs], [security-crypto-secrets], [security-privacy-logging], [dependency-updates]; [plan-threat-model] for destinations/trust/recovery; [security-windows-privileges] for service/native retrieval owners; Rust/Windows boundary reference for native jobs. | [qa-component-tests], [qa-contract-tests], [qa-integration-tests], [qa-security-boundaries], [qa-adversarial-inputs], [qa-structured-fuzzing], [qa-golden-fixture-parity] for protocol/crypto vectors, [qa-state-concurrency-lifecycle], [qa-fault-recovery], [qa-performance-resources], [qa-static-checks]. Preserve critical-file coverage/mutation gates. |
| **P3 — services, IPC, persistence** | [security-windows-privileges], [security-local-ipc], [security-protocol-inputs], [security-privacy-logging], [evolve-data-contracts]; service adapter, strict boundary and Windows-memory references. | [qa-contract-tests], [qa-integration-tests], [qa-security-boundaries], [qa-state-concurrency-lifecycle], [qa-fault-recovery], [qa-adversarial-inputs], [qa-performance-resources], [qa-static-checks]; [qa-structured-fuzzing] for actual framing/decoding. |
| **P4 — real Windows discipline** | [security-windows-privileges], [security-local-ipc], [security-protocol-inputs], [security-privacy-logging], [security-build-deployment] for takeover/restoration; [security-crypto-secrets] for newly connected recovery/cache paths; service adapter, Chrony accounting, Windows evidence and Windows-memory references. | [qa-integration-tests], [qa-contract-tests], [qa-state-concurrency-lifecycle], [qa-fault-recovery], [qa-security-boundaries], [qa-model-based-testing], [qa-e2e-tests], [qa-performance-resources]. Use [qa-package-artifacts] for installed handover/lifecycle effects. |
| **P5 — production UI** | [implement-native-ui], [security-native-ui], [security-local-ipc], [security-privacy-logging]; [security-windows-privileges] for helper flows; [evolve-data-contracts] for live snapshot/settings compatibility; design/brand references and D1–D3 re-entry rules. | [qa-component-tests], [qa-contract-tests], [qa-integration-tests], [qa-e2e-tests], [qa-accessibility], [qa-visual-dpi], [qa-i18n-localization], [qa-performance-resources], [qa-security-boundaries]. Restore full requirements at their triggers; none are retroactive UI-M0 blockers. |
| **P6 — hardened complete package** | [security-build-deployment], [security-windows-privileges], [security-local-ipc], [security-crypto-secrets], [dependency-updates], [evolve-data-contracts] for version/migration/rollback; [security-privacy-logging] for packaged event resources; signing and mitigation authorities. | [qa-package-artifacts], [qa-security-boundaries], [qa-integration-tests], [qa-e2e-tests], [qa-fault-recovery], [qa-state-concurrency-lifecycle], [qa-static-checks], [qa-performance-resources], [qa-black-box-acceptance]. |
| **P7 — exact release candidate** | [security-build-deployment] and the affected security owners; [qa-test-orchestrator] reconciles the full matrix. Re-read changed contracts/decisions, not the whole pack indiscriminately. | [qa-coverage-gap-analysis], [qa-regression-selection], [qa-mutation-testing], [qa-static-checks], [qa-e2e-tests], [qa-package-artifacts], [qa-performance-resources], [qa-accessibility], [qa-visual-dpi], [qa-i18n-localization], [qa-black-box-acceptance], [qa-independent-review], [qa-final-qualification]. Reuse unchanged qualified evidence with an explicit identity argument. |

UI-M0 stays on its existing task: [implement-native-ui], section 3 Rust/contracts,
applicable [qa-component-tests]/[qa-contract-tests]/[qa-static-checks], native
memory guidance and M0 observations. Full P5 reading/testing does not activate
D1–D3 early merely because an agent is working on the fixture UI.

### Conditional skills and references

These are **mandatory when the trigger occurs**, not an invitation to run every
tool on every change.

| Trigger | Read and apply |
| --- | --- |
| Measured bottleneck or explicit optimization | [optimize-rust-performance] with [qa-performance-resources]; benchmark/profiler measurement reference. |
| Unknown producer-to-consumer/effect path | [qa-data-flow-analysis]; its CodeQL reference when the source path cannot otherwise be established. Tool extraction coverage limits remain explicit. |
| A failure, regression or disputed result | [qa-failure-triage]; [qa-system-stabilization] only for a bounded multi-defect convergence effort. Preserve first failure and healthy control. |
| Multiple delegated owners or interrupted work | [qa-subagent-dispatch], [qa-context-checkpoint]; one record and exclusive mutable ownership. |
| Wire/persistence/version/units/validation acceptance changes | [evolve-data-contracts] with [qa-contract-tests], including old/new consumer and interrupted-migration cases. |
| More state transitions/interleavings than ordinary examples cover | [qa-model-based-testing], [qa-state-concurrency-lifecycle]; choose Loom/Kani only for a named feasible model/proof obligation. |
| Concrete compiler diagnostic, borrow/drop issue, or Send/Sync problem | Respectively [rust-compiler-errors], [rust-borrow-semantics], [rust-send-sync]. Do not change architecture just to silence the compiler. |
| Available admitted native Windows UI automation | [winapp-ui-automation]; automation supplements actual keyboard/Narrator observations. Do not install a tool just to follow a preferred path. |
| SCM, native handles, callbacks, FFI or cancellation | [Service adapter guide](../../.agents/skills/implementation/implement-rust/references/windows-service-adapters.md), [Rust/Windows boundaries](../../.agents/skills/implementation/implement-rust/references/rust-windows.md), [Windows memory diagnostics](../../.agents/skills/testing/references/windows-memory.md). |
| Numerical/protocol oracle or native acceptance | [Numerical/protocol truth](../../.agents/skills/testing/references/numerical-protocol.md), [Windows evidence](../../.agents/skills/testing/references/windows-evidence.md), as applicable. |
| Authorized stage/commit preparation | [qa-pre-commit]. A phase pass itself grants no commit, push, signing or release authorization. |

## 6. Phase implementation and acceptance gates

All results in this section are **required future observations**, not completed
tests. Each implementation task first states its bounded inputs, source owner,
independent expectation, accepted neighbor, failure/forbidden effect, environment,
resource/time limits, cleanup and exact exit. Use the complete canonical
[CHECK catalog](../windows-time-architecture.md#202-acceptance-catalog) and
[AC catalog](../windows-time-features.md#8-acceptance-scenarios-and-evidence)
when expanding a row; these operational criteria do not remove catalog cases.

### P0 — admit sources and prove platform assumptions

Complete the existing S0.1–S0.7 cards in
[architecture §21](../windows-time-architecture.md#21-delivery-phases-and-readiness-gates):
exact source/graph licences; two service identities; mutual IPC; actual clock
semantics; accessible UI feasibility; a complete laboratory MSI; finite policy,
native-work and supported-environment decisions. Begin with the next task in
section 7, not a scaffold of the entire product.

Acceptance: rejected source/feature/provider controls fail admission and a
compatible neighboring graph passes (CHECK-20/26). Real final-account token and
SCM observations establish CHECK-1/27 subsets. One real status and one protected
IPC operation exercise allowed/denied callers and fake endpoints (CHECK-2).
Clock experiments establish measured limits (CHECK-10/11/12). UI feasibility
uses CHECK-22; laboratory MSI uses CHECK-5/13/23. Review CHECK-4 applicability
and current OPEN decisions without claiming compliance.

Before any laboratory clock write, record disposable guest/snapshot, independent
recovery access, current owner, exact operation, phase/rate/duration/attempt bounds,
stop condition and restoration. Start read-only and with denied unprivileged
calls. Unset experiment bounds mean no write. Test certificates, if needed for
S0.6, are confined to the owned guest and cannot establish public publisher trust.
Use the canonical two-to-five focused engineering-day experiment timebox and
one bounded alternative before reviewing a failed approach.

### P1 — pure protocols and independently checked algorithms

Implement S1.1 admission/types and deterministic harness; S1.2 regression/history;
S1.3 interval selection/principal/supporters; S1.4 abstract discipline/holdover/leap;
S1.5 actual application verification gates. Enforce `forbid(unsafe_code)` in pure
owners. Keep sockets/OS clocks outside them.

| Acceptance | Independent observation and valid/fault controls | Mapping |
| --- | --- | --- |
| Packet and numeric admission | Raw valid NTP vector is accepted once; malformed/truncated/oversized/duplicate/stale/wrong-correlation/invalid-extension vectors cannot mutate time/history. Checked era/unit/overflow and finite-result boundaries work in release builds. | CHECK-6/25 |
| Timestamp meaning | In one millisecond coordinate system, T1=0, T2=70, T3=80, T4=110 yields +20 ms offset and 100 ms round-trip delay. These are relative analytic coordinates, not literal zero-valued wire timestamps; packet fixtures need valid timestamp encodings. Derive independent asymmetric-delay, precision, rollover and ambiguous-era cases; do not replace upstream precision handling with a simplistic negative-delay rule. | CHECK-7/25 |
| Regression | Known offset/drift/noise trajectories meet predeclared error and uncertainty envelopes; sample aging/pruning remains bounded. Applied corrections use actual acknowledgements or a documented stable coordinate system. | CHECK-7/10, AC-19 |
| Selection | Independently specified survivor/principal/supporter sets and reasons match. Fast wrong peers, duplicate identities, lost peers and two-source disagreement cannot manufacture votes, lower minimum agreement or silently become a single-peer profile. | CHECK-7/8, AC-03 |
| Combination/discipline | Preserve different offset/frequency weighting, principal hysteresis and uncertainty. Step/slew/holdover proposals remain pure, bounded and invalidated by modeled generation changes; no feedback from merely requested effects. | CHECK-7/8/10 |
| Contracts/tooling | Real entry points reject construction/transform bypasses; healthy controls succeed. Nonzero tests, lint/SAST failure propagation, source provenance, reviewed security-module/critical-file scope and applicable coverage/mutation gates are established. | CHECK-19/20/25/26 |

Use exact/analytic and independently implemented reference calculations, adversarial
simulation and pinned upstream differential traces together. A matching translated
algorithm alone is not an oracle. Freeze tolerances before running the candidate;
there is no invented universal Internet-accuracy SLA. P1 exit is CHECK-6/7/8/19/20
and pure CHECK-25 with declared simulation envelopes; it proves no Windows effect.

### P2 — secure, bounded NTP/NTS communication

Deliver S2.1 NTP-only observation over real controlled UDP/DNS; S2.2 ordinary
NTS-KE/packets using admitted dependencies; S2.3 protected association reuse and
qualified opt-in recovery; S2.4 adversarial/resource/secret handling. A diagnostic
consumer must expose actual admitted results and failure reasons. Do not wire
production networking into UI-M0.

The existing [protocol comparison follow-up](../../tasks/s0.1-protocol-comparison.md#kod-reuse-and-rfc-8915-alignment-follow-up)
owns KoD source analysis and the RFC 8915 alignment checklist. Extend its pinned
source/port evidence before implementation; reuse upstream algorithms with
independent expected results. S1.1's opaque control completion is a seam, not a
polling/recovery implementation. S2.1 owns plain-NTP KoD integration; S2.2 adds
NTS control semantics and the [AES-preferred TLS policy](../windows-time-architecture.md#1871-license-gate-and-finite-provider-choice).
S2.3/S2.4 exercise cache recovery, retry and resource failure paths. The
[protected-storage workstream](#84-protected-optional-storage-design-and-qualification)
adds the reviewed optional-state contract and its paused native-evidence gates. Keep graph,
Windows trust and interoperability gates distinct from this documentation review.
The same task's [replay and delayed-response matrix](../../tasks/s0.1-protocol-comparison.md#replay-and-delayed-response-qualification)
binds S2.2–S2.4 to restoration, consume-once publication, monotonic processing-time
expiry and genuine first-delivery-delay cases. Existing fixture correlation is
not production NTS replay qualification; retain OPEN-5/6's unresolved bounds.

| Acceptance | Independent observation and valid/fault controls | Mapping |
| --- | --- | --- |
| NTP transport and KoD | Controlled IPv4/IPv6 peers exercise correlation, endpoints, eras and generations. Test reviewed RATE backoff, DENY/RSTR cessation, unknown codes, forged/stale/replayed controls and extreme poll claims. Observe sends and source state; KoD never enters time arithmetic/history or lowers agreement policy. A healthy neighboring source remains usable. | CHECK-6/8/27 |
| NTS-KE | TLS 1.3 baseline, `ntske/1`, verified identity, complete request flush, bounded framing/cardinality/end rules and offered protocol/AEAD negotiation. Wrong identity, unknown critical records, truncation and unoffered algorithms fail without a usable association. | CHECK-6/9/26 |
| RFC 8915 and cipher preference | Close the linked checklist with actual entry-point cases for KE errors/warnings/closure, exporter/nonce/AD, cookies, trailing fields, NTS KoD/NAK and retry reset. Observe AES-GCM offer order and negotiated suites; qualify provider acceleration/fallback without raising the CPU floor or admitting a new graph. | CHECK-6/9/17/26 |
| NTS packets | Independent exporter/AEAD known-answer and peer tests cover direction, UID, origin, replay, corrupt tag and wrong key. Fresh request UID has at least 32 CSPRNG octets; use the required AES-SIV-CMAC-256 profile through an admitted maintained library, not new cryptographic primitives. | CHECK-6/20/26 |
| Atomic association state | Commit cookie/key/time changes only after the complete protected response validates. Invalid responses leave healthy state intact. Correlated NAK permits bounded re-establishment only; uncorrelated storms create no downgrade or unbounded work. | CHECK-6/17/27 |
| Endpoint policy | Revalidate actual destination at initial connection, reconnect and DNS/delegation change. Authenticated delegation cannot reach unapproved local/private/other destinations; explicitly approved LAN sources still work. | THR-21, CHECK-6/8/9 |
| Trust and recovery | Cached NTS then strict KE precede any qualified provisional-time path. Test service-account roots/distrust, expired/future/revoked/unknown/offline certificates and extreme-date conversions. Plain hints never change global time or become authenticated votes; two-helper failure never silently becomes one-helper mode. | CHECK-8/9, AC-02/04/09/10 |
| Runtime ownership | Stalled DNS/certificate/KE work does not block lifecycle/control. Cancelling the awaiter retains native-job capacity until actual completion; old-generation results have no state effect. No replacement pools/retry threads evade saturation. | CHECK-27 |
| Useful interoperability | Real normal NTS acquisition succeeds against at least two independent implementations where practical, with a controlled bad peer beside healthy peers. An unavailable second implementation remains an explicit evidence gap. | P2 exit, CHECK-6/9 |

Normal NTS communication is a separately reportable milestone while exceptional
recovery remains open. Full P2 exit requires CHECK-6/8/9/17, NTS CHECK-26 and engine
CHECK-27. No global clock writes are needed. Keep plain NTP explicitly selected
and labelled unauthenticated; no mixed votes or silent fallback from NTS failure.

### P3 — Windows services, secure IPC and protected state

Implement two own-process services using qualified narrow SCM adapters, then
bounded versioned handlers, current authorization, protected persistence and
coherent status. Start proposal integration with a simulated clock backend.

| Acceptance | Required Windows observation | Mapping |
| --- | --- | --- |
| Service isolation | Actual engine/UI tokens cannot set time or inject/write into the broker; broker has only qualified rights and no external network role. No LocalSystem shortcut. Valid permitted broker operation remains possible in the owned fixture. | CHECK-1/2/27 |
| SCM lifecycle | Registration/status/callback lifetimes are correct; Running means available, not authenticated Tracking. Prompt callbacks, finite wait hints, actual readiness, single shutdown owner, correct destructors and no unwinding across ABI. Failure and delayed restart policies are separate per service and exercised. | CHECK-27, AC-01/13 |
| Mutual IPC identity | Real standard/filtered/elevated/delegated/revoked/remote/old-session/fake-server clients prove bidirectional identity and per-operation rights. Include endpoint squatting/restart/PID reuse, minimum pipe rights, thread-bound caller capture and fatal failed reversion. | CHECK-2/3 |
| Framing and native completion | Partial frames, slow readers/senders, trickle input, cancellation and storms cannot exceed byte/client/job bounds or duplicate effects. Native buffers/handles/OVERLAPPED/permits outlive actual I/O; cancellation request is not completion. Valid neighboring traffic remains usable. | CHECK-3/25/27 |
| State and persistence | Strict schema/garde construction precedes current guards. Invalid, stale or unauthorized requests have no protected effect. Corrupt critical policy blocks safely; bounded atomic replacement survives disk-full/interruption. Reparse/hardlink/path-swap cannot redirect privileged writes. | CHECK-15/25, AC-07 |
| Protected optional state | Apply [section 12.1.2](../windows-time-architecture.md#1212-protected-optional-storage-and-explicit-fallback): admitted root origin, exact committed snapshot, explicit no-root fallback, bounded crash/recovery, separate mandatory authority and first-connection protection notice. | CHECK-1/9/14/15/16/17/25/27; [focused plan](windows-protected-storage.md) |
| Truthful status/diagnostics | Real snapshots distinguish availability, authentication, agreement, freshness, uncertainty and acknowledged correction. Redacted required events survive optional filters; authorized DEBUG expires locally and sink failure/flood remains bounded. Logs are not authoritative state or the clock journal. | CHECK-14/17, AC-05/16 |

P3 exit requires CHECK-1/2/3/14/15/17, service CHECK-27 and real-handler CHECK-25.
OS-token revocation limits remain explicit. “Sync now” requests fresh measurement
under current configuration; it carries neither arbitrary destination/target time
nor permission to alter machine policy. Real clock-effect qualification is P4.

### P4 — Windows clock integration, ownership and recovery

Only after relevant S0.2–S0.4/S0.7 and P3 evidence, connect the real backend in an
owned disposable Windows environment. Qualify read/rate/step sign, units,
quantization, achieved correction, cancellation and acknowledgement loss using an
independent reference. Reconcile ambiguous effects; never repeat a step blindly.

Acceptance includes cumulative accounting across reconnect/sign change/crash;
no automatic budget renewal; sole W32Time/controller ownership; transactional
takeover/restoration; managed-policy refusal; suspend/S3/S4/Modern Standby, manual
step, VM restore and network transition invalidation. Sleeping time is not
delivered active slew. Engine/broker death during correction exercises measured
recovery, crash-loop limits and successor/ambiguous ownership without blind resets.
Observe-only startup performs no clock write; transition from active correction
completes only current-owner cleanup and does not implicitly hand back W32Time.

Freeze supported date/era, leap owner, automatic/exceptional correction envelopes
and one-use approval rules before enabling them. OD-02's approximately 100-year
authenticated-recovery target is not an unlimited step policy. “12 hours” is not
an approved plain-NTP constant. Test valid neighbors and forbidden effects for
CHECK-5/10/11/12/13, applicable CHECK-3/25/27 and AC-08/11/12/13/14/19. P4
performance/lifetime evidence includes the applicable CHECK-21 measurements.

### P5 — truthful, accessible production UI

Use the [design specification](../nevrotick-design-v0.2/docs/02-ui-spec.md),
[interactive preview](../nevrotick-design-v0.2/NevroTick-UI-preview.html),
[screen catalogue](../nevrotick-design-v0.2/screens/README.md) and
[brand kit](../NevroTick-AVIF-kit-v1/README.md). Keep asset notices separate from
code licensing. Reference scenes are not behavioral authority. Do not execute
the archived React tooling before resolving its recorded dependency findings.

- **UI-M0:** preserve M0-01–M0-08, sample labels, disabled protected actions,
  transient checked editing and native window behavior; keep its current
  instrumentation gap visible. It does not complete P0/S0.5 or P5.
- **D2 re-entry:** restore freshness/reconnect/generation semantics before any
  live read-only status. Restore policy revisions, receiver authorization and
  confirmed/failed/unknown results before real settings/protected commands.
- **D1/D3 re-entry:** restore theme/contrast/text/DPI and full keyboard/Narrator/UIA
  requirements before general-user preview/release or an earlier corresponding
  qualification claim. UI ownership and the P5 task track those triggers.

| Acceptance | Required observation | Mapping |
| --- | --- | --- |
| Honest status | Unavailable/stale/malformed/old-generation snapshots cannot look current or Tracking. Reply, accepted-estimate, applied-correction and snapshot ages remain distinct; principal/supporters and rejection reasons match real state. | CHECK-14/25, AC-02/03/05 |
| Real workflows | Valid refresh/settings/review/export work through the receiver. UAC cancellation, stale edit, denied command, storage failure and lost acknowledgement never become success. Exceptional approval applies only once to its exact current proposal. | CHECK-2/3/10/15/25, AC-06/07/11/16 |
| Independent lifecycle | Boot-before-login and synchronization continue when UI closes/crashes or user logs off. Default close leaves no UI process; optional tray/sign-in settings affect only the user observer. | CHECK-12/21/23, AC-01/18 |
| Accessible composition | Actual Windows keyboard, Narrator and UIA complete critical Status/Sources/Settings/Diagnostics, correction review, cancel and tray journeys; labels, focus/error association and announcements work. Qualify contrast, themes, text scaling, 200% scaling, mixed DPI and localized long/non-ASCII text. | CHECK-22, AC-17/18 |
| Resources | Native window/control/heap/GDI/USER/handle cleanup is observed before process exit; responsiveness and memory meet the declared profile. | CHECK-21/27, AC-20 |

P5 exit requires CHECK-14/17/21/22 on actual Windows. Screenshots, mock snapshots,
standard-control selection and cross-compilation cannot substitute for usability,
assistive technology or production state evidence.

Localization checks cover the adopted English UI, locale-sensitive presentation
and expansion fixtures; they do not add an unapproved translation/locale scope.

### P6 — complete installation, hardening and release artifacts

Deliver one complete per-machine MSI containing UI, engine, broker, helper and all
required payloads. Observe install, unelevated launch, upgrade, repair, downgrade
rejection and uninstall without UI, with other sessions and at every relevant
interruption/reboot point. Preserve one product/version/controller, exact prior
W32Time state and ownership-aware rollback; do not fight GPO. Disabled required
rollback must stop takeover before mutation. Pending SCM recovery cannot escape
maintenance quiescence. Hostile repair properties/transforms/paths cannot become
privileged commands; old-generation cleanup cannot damage successor ownership.

Per-process mitigation manifests distinguish build, creation-time and runtime
requirements, effective state and valid workflows. Check actual SCM child-policy
behavior, DLL search/tamper canaries, native provider compatibility and UI/MSI-host
requirements. Required startup-protection failure blocks normal protected work
while preserving authorized recovery; compiler flags alone prove no full profile.

Reconcile exact Cargo/runtime/build/proc-macro/native graph, source grants/notices,
generated code, SBOM and **complete package payload**, including non-Cargo files.
Qualify the selected Authenticode or OD-05 initial OpenPGP release profile with
tamper/wrong-signer/manifest/payload controls and installed-file trust. OpenPGP
does not establish Windows publisher trust. A valid supported candidate must work
without developer roots or disabled Windows protections. No automatic updater,
separate UI package or unqualified MSIX route is added.

P6 exit: CHECK-13/16/18/20/23, applicable CHECK-5/15/26/27 and AC-15/20, with
profile-specific signing/lifecycle evidence on supported clean Windows machines.

### P7 — final integrated acceptance

Freeze source, lockfile, tools/SDK, feature/target rows, policy, fixtures and exact
packaged/installed bytes. Reconcile all applicable CHECK-1–CHECK-27 and
AC-01–AC-20, phase receipts, changed contracts and open findings. Reuse earlier
evidence only while its relevant state is unchanged; rerun affected observations
after invalidating changes. Required missing evidence is incomplete, not N/A.

Require direct black-box input-to-output inspection, supported Windows matrix,
native authority/lifecycle/accessibility evidence, complete-package trust,
critical-file coverage/mutation gates and CHECK-21 baseline/soak. Independently
review ASVS dispositions and native adaptations; do not claim completed L3/WCAG
conformance from skill use or a narrow test suite. A fresh reviewer checks exact
artifacts, remaining risk, recovery and claims. Release needs complete required
evidence, resolved applicable blockers/explicit owner decisions, compatible
licensing and David's approval for the exact candidate.

## 7. Next executable task for a development agent

**Implementation:** continue the existing [S1.3 fixture task](../../tasks/s1.3-source-selection-readiness.md)
in the main checkout. S1.1 and the exchange-to-history fixture seam already exist;
reconcile the latest S1.2/S1.3 addenda before assigning work rather than repeating
the original S0.1/S1.1 preparation. Existing S0.1 provider/platform gates still
block only their dependent integration and claims.

**Acquisition pilot continuation:** the
[engine main-workspace checkpoint](../../tasks/engine-transport-pilot.md#main-workspace-checkpoint)
now records the admitted Tokio/getrandom graph, sixth main member, full host
portable checks, real loopback CLI cases and Windows cross-build. Continue native
Windows completion/heap/resource observations after the
[eight-case native CLI checkpoint](../../tasks/engine-transport-pilot.md#native-windows-cli-checkpoint)
and [four-case live-process port checkpoint](../../tasks/engine-transport-pilot.md#native-live-process-port-release-checkpoint),
then processing-time expiry and the remaining engine qualification
under P0/architecture §18.6.1. This follows §2's permitted
overlap and advances toward S2.1; it does not relabel fixture contexts or close
the remaining S1.3/P1, DNS, KoD, trust or native qualification requirements. The
[measured admission optimization](../../tasks/s0.1-protocol-comparison.md#full-tool-admission-metadata-optimization--1-november-2026)
preserves the complete tool inventory and custody checks.

Before extending the affected owners, apply the bounded
[WIP quality follow-up](#83-wip-code-quality-feedback-and-bounded-follow-up).
Finish the current engine processing-guard checkpoint first; the follow-up does
not turn unfinished product features into newly discovered defects or suspend
independent work.

1. Recheck root, HEAD/index/working bytes, active owners and the selected task's
   exact source/graph admission. Follow the common/P1 skills and section 3 contracts.
2. Use the reviewed interval/minimum, batch, continuous-axis majority/witness and
   rejection-priority contracts. Advance from the last reached owner: checked
   constructor evidence cannot count as batch/evaluator RED or GREEN. Preserve
   independently configured minimum agreement and no single-peer fallback.
3. Establish independent accepted/rejected neighbors and reached behavioral
   controls, then implement the smallest ready owner. Preserve exact source
   provenance, private construction, capacity/duplicate rules and failure atomicity.
4. Qualify the integrated candidate with selected contract, format/compiler/Clippy,
   ordinary/optimized and applicable coverage/mutation evidence. Use one artifact
   owner per worker and retain actual failures/incomplete observations. A passing
   historical five-owner campaign is no six-owner or full P1 result.
5. Update that task's short current status, exact evidence and next permitted
   action. Native, production eligibility, principal/supporter policy and full
   P1 remain separate obligations.

**Required parallel build-policy work:** execute the baseline-admission stage in
[section 8.1](#81-x64-baseline-and-runtime-optimization-workstream) at the next
owned configuration checkpoint, before claiming the new Windows x64 CPU profile.
Keep the one writer for shared Cargo/policy/checker inputs explicit. A task may
prepare isolated candidates while another owns the shared source; do not overwrite
its admission or rewrite historical receipts. This planning amendment runs no
product code, Windows mutation, signing or publication.

## 8. Performance, limits and qualification environments

Use [architecture §17](../windows-time-architecture.md#17-resource-and-performance-targets)
and [§17.1](../windows-time-architecture.md#171-bounded-async-and-native-work).
The following are **unmeasured initial targets**, to freeze or explicitly revise
with evidence, not achieved performance or authority to loosen limits.

| Measurement | Initial target and required context |
| --- | --- |
| Service memory | Engine + broker ≤32 MiB steady-state private bytes; p95 one-second samples over a one-hour, four-source run after warm-up; peaks separately. |
| UI memory/lifecycle | Visible UI ≤48 MiB; separate optional tray budget; default closed UI has no process. |
| Idle service CPU/wakeups | ≤0.1% of one logical CPU averaged ten minutes; ≤4 timer wakeups/second in the canonical idle tracking condition; retain burst data. |
| UI latency | Local-action p95 <100 ms; cold usable window <500 ms on named hardware; network/UAC separate. |
| Storage/package | Aggregate 10 MiB rotating diagnostic budget; initial ≤20 MiB compressed complete MSI, without hidden runtime downloads. |
| Long run | One-hour baseline and 72-hour reconnect/fault soak; no unexplained retained handle/thread/allocation growth before teardown. |

Retain the canonical hard configured cap of 16 logical sources, 64 history samples
per source and separate maximum-two helpers. The four-source target does not
settle OD-03's shipped preset. Freeze protocol/IPC/string/cookie/queue/file/error
limits from their canonical contracts before exposing inputs; test exact accepted
neighbors and one-over-limit rejection plus aggregate pressure.

P0 starts with at most two submitted native jobs, including waiting jobs, no
separate backlog and at most one acquisition job per endpoint. Each actual job
owns its permit until completion. The proposed KE profile is 128 KiB overall,
acceptance of at least 65,536 bytes, 20-second connection/handshake budget and
at most two concurrent exchanges, subject to protocol/interoperability and native
completion qualification. A timeout cannot preempt a synchronous Windows verifier
callback. Qualify the complete exchange off the responsive engine loop; engine-only
Tokio does not mandate `tokio-rustls` or runtime dependencies in other components.

Name hardware/VM, Windows build, token, power profile, Defender/protections,
toolchain/SDK, target/features, fixtures, seeds/load, warm-up, sampling, duration,
stop conditions and cleanup. Profile useful valid and hostile/recovery workloads.
Keep AppVerifier/PageHeap corruption checks separate from uninstrumented resource
measurements; neither alone proves leak freedom. Pure host tests, Windows
cross-builds, native execution, package lifecycle and final artifact trust are
separate evidence layers.

### 8.1 x64 baseline and runtime optimization workstream

**Required owner direction:** [OD-07](../nevrotick-owner-decisions.md#11-od-07--x64-baseline-and-runtime-optimization)
selects `x86-64-v3` as the intended Windows x64 minimum and runtime-selected
higher-feature optimization. Read [the research](../supporting-reviews/x86-runtime-optimization.md)
and [hardening §6.7](../windows-time-development-hardening.md#67-v3-baseline-and-runtime-cpu-dispatch).
The currently admitted flags are still generic `x86-64`; this document changes the
planned compatibility contract, not executed configuration or measured speed.
V3 already includes AVX2: do not create a redundant AVX2 tier just to populate a
three-level framework. Runtime AVX-512 paths use their exact additional features;
AES/PCLMUL/VAES remain independently qualified provider capabilities.

The existing [build-optimization task](../../tasks/build-optimization.md) owns
candidate identity, admission, observations and remaining work. Use
[CPU dispatch implementation](../../.agents/skills/implementation/implement-cpu-dispatch/SKILL.md),
[optimize-rust-performance] and [qa-performance-resources] when carrying out the
changed guarantee. Scope stages to the available implementation and fixtures;
no new product crate, runtime or generic dispatch framework is required.

| Stage / owner | Next action and observable exit | Existing acceptance mapping |
| --- | --- | --- |
| Baseline admission / build owner, next controlled configuration checkpoint | Migrate the Windows target-specific CPU entry to v3 with the entire mitigation vector. Update exact checker/control expectations, affected policy/input bindings and skills measurement guidance together; preserve staged/concurrent work and historical results. Check actual package/feature/target/native/prebuilt inputs, portable and optimized arithmetic, Windows compiler/Clippy/link artifacts and useful failure propagation. No unreviewed global v4, `native` or ambient flag replacement. | CHECK-19/20/25/26; P1 S1.5 and Windows artifact rows |
| Compatibility / platform and package owners | Declare CPU/OS/VM rows; implement a lower-baseline-compatible prerequisite check before starting any v3 payload. Qualify v3 CPU plus OS vector-state availability, below-v3 refusal and masked guest features. Keep one complete MSI and existing helper privilege/lifecycle constraints; coordinate its final delivery with P6. No unsupported-machine or v3 shipping claim before actual observations. | CHECK-19/20/26; S0.6 and P6 package compatibility |
| Hotspot and kernel / affected algorithm or crypto owner | Profile a representative admitted workload and predeclare useful-work metrics, finite bounds, guard metrics and a decision rule. Inspect emitted code before adding an optimized function. Prefer an admitted provider's existing crypto dispatch. Prepare meaningful AVX-512 candidates only where data justify them, while preserving a tested v3 implementation. This stage is required evaluation work; an unused probe is no delivered optimization. | CHECK-7/17/21/25/26 as applicable; P1 numerical work or P2 real NTS workloads |
| Dispatch safety and parity / implementation and independent QA owners | Declare each function's exact enabled features and safety preconditions. Challenge missing individual features and OS support, v3 fallback, forced baseline, and supported optimized paths. Synthetic inventories test selection only; executing a forced optimized path still requires real detected support. Preserve pure-core unsafe prohibitions, FP operation/intermediate-check order, exact integer results and failure atomicity. Reject changed rounding/underflow behavior without a separate algorithm decision. | CHECK-19/25/26 and affected contract/security scope |
| Native adoption / performance and release owners | Compare baseline and each supported optimized path on named native Windows hardware/VMs with normal protections. Observe useful latency/CPU, wakeups/power where measurable, startup, memory and binary size; retain raw results and discarded candidates. Ship a specialization only when its predeclared benefit and all correctness/resource guards pass. Reconcile final payload CPU requirements and existing baseline/soak/package gates. | CHECK-19/20/21/25/26; P2 resource evaluation and P6/P7 final artifacts |

Keep the v3 baseline prerequisite on the delivery path and execute meaningful
hotspot evaluation before closing the affected performance scope. Independent
pure S1.3 development may continue while build/native prerequisites are prepared.
Missing hardware, provider admission or representative workloads leave their
named stage incomplete; do not count unsupported execution, source-only feature
lists or repeated compile success as optimization evidence. A non-beneficial
specialization stays disabled with its evidence and revisit trigger; it does not
justify a performance claim or silently removing the required workstream.

### 8.2 Contract-review workstream and interface gates

The [30 October critical assessment](../supporting-reviews/architecture-contract-review.md)
and [canonical acceptance refinements](../windows-time-architecture.md#2021-contract-review-acceptance-refinements)
extend this existing plan. They do not restart P0–P7, qualify the product or block
independent current S1.3 fixture work. Close the relevant contracts before freezing
or enabling the dependent production interfaces:

| Existing stage/owner | Contract and exit |
| --- | --- |
| S0.2/P3, platform/security | Choose and demonstrate object-owner isolation through runtime creation/replacement, not DACL allow-list inspection alone; CHECK-1/15. |
| S0.4/P4, broker/storage | Freeze durable reservation/outcome protocol, explicit unknowns, mode/value transitions, slew settlement and boot/Fast Startup epochs. Fault each persistence/effect/acknowledgement seam; CHECK-10/11/12/15. |
| S0.7/P2, protocol/security | R-05 freezes association/key/full-authentication/resumption ages, trust-change detection and unknown-age behavior before S2.3 reuse; CHECK-8/9/15. |
| S0.7/P1/P2/P4, core/security | R-02 freezes uncertainty/floor provenance and narrow supersession; production vote/freshness/independence tables distinguish retained prediction from fresh routine or exceptional agreement; CHECK-7/8/9/14/15. Existing pure selection fixtures remain bounded evidence. |
| S0.6/P6, installer/release | Qualify durable maintenance startup inhibition against queued SCM recovery; wrong-clock/offline signing, key policy and project-specific admission through repair/removal/restoration; CHECK-9/13/18/23. |
| P1/P7, tooling/QA | Reconcile per-critical-file functions/guards with generated and automatically absent mutants and alternative evidence. Keep the 95% gate and scoped historical receipts; CHECK-19/25. |

OPEN-5/6/8/11/14 retain unresolved policy ownership. Prepare NV-DEC-001 cards when
evidence makes a dependent material choice concrete; this assessment accepts no
new TTL, source count, correction allowance, trust exception or reset privilege.
Freeze the relevant docs, configuration and source at one commit plus identified
dirty bytes for handoff. Archived design copies and older review snapshots retain
their historical status; the current maintained authorities control.

### 8.3 WIP code-quality feedback and bounded follow-up

**Assessment date:** 1 November 2026. Reviewed the supplied code-quality feedback
against `b14d4e628f7d2ea5b909bf736a7bf5f908ea4e4b` and the existing dirty S26
engine/test-task delta. That delta adds processing/control tests without changing
production function bodies; its task still records execution as pending.
Recheck current bytes and active ownership before implementation. This is a
planning amendment; no fix, product test or new qualification pass is claimed.

The feedback's three proposals are useful maintenance/testing improvements;
source review established no new reachable product defect. Its approximately
80/100 score is an uncalibrated reviewer opinion, not a project metric or
readiness result. The positive observations about staged lifecycle commit,
private borrowed wire types and owned timer cleanup match the inspected code;
they establish neither complete product correctness nor native AFD completion.
Missing production features remain WIP under their existing stages.

Keep these local finding IDs in the existing task evidence chain. They supplement
the canonical CHECK requirements and do not restart P0–P7.

#### QF-01 — couple terminal engine outcomes to their payloads

**Disposition:** valid maintainability risk; no inconsistent reachable transition
was found. **Owner/record:** engine implementer in the existing
[transport task](../../tasks/engine-transport-pilot.md). **Timing:** finish the
current S26 processing/deadline/control checkpoint, then perform this bounded
cleanup before adding production states/TLS, preferably before the final engine
coverage/mutation candidate is frozen. CHECK-6/21/25/27 apply within this scope.

[`LiveSlot` and `SlotReport`](../../crates/nevrotick-engine/src/lib.rs) separately
store terminal status and optional payloads. Replace that relationship with a
private terminal-outcome enum: an ordinary header carries its stratum/facts,
a control carries its code, and I/O distinguishes a captured error from an
anomalous send length. Do not assume every I/O outcome has an OS error code:
the existing short-send path deliberately has none. Retain genuinely independent
send/rejection history; `sent` can differ before/after a confirmed send, and a
previous rejection can legitimately precede a valid result. Preserve current
public getters, CLI projection, one-way transitions and terminal history. A broad
typestate or public API rewrite is not required.

**Exit:** matching payloads are enforced by construction; real consumers retain
healthy/control/I/O and pre/post-send outcomes, current processing/control
rejection, completed-neighbor preservation and timer/error cleanup behavior.
Reuse existing cases and add only missing discriminators. Ensure the selected
runner actually executes library units as well as runtime-boundary integration
cases; the current portable engine row selects `runtime_boundary` explicitly.
Refresh affected admission, run configured format/compiler/Clippy, ordinary/
optimized and consumer checks, then affected native regressions and the declared
critical-file gates. Earlier Windows observations keep their original identities;
host tests cannot establish native completion, retention or soak.

#### QF-02 — name accounting recipes without weakening completion

**Disposition:** valid mechanical maintenance cost, not faulty accounting.
**Owner/record:** verification maintainer in the existing
[S1.3 task](../../tasks/s1.3-source-selection-readiness.md). **Timing:** at the next
recipe/counter-family change; unrelated engine/NTS work need not wait.
CHECK-19/25 apply.

The [Python recipes](../../scripts/core-combination-accounting.py) contain long
positional vectors, but runtime transport already has named fields; Rust already
has a closed operation enum and named limits. First replace authored recipes and
limits with typed named records using explicit keyword fields and no defaults
that conceal omissions. Preserve current field names/order, literal counts,
bounds, emitted bytes and complete/partial/missing classifications. In particular,
373 versus 374 getter calls must still distinguish exact recipe completion,
even below the 6720 ceiling. Completion and assertion success remain separate.

Keep independently authored numerical expectations and literal transport rows in
tests; do not derive them from production recipes. Add field/order conformance
controls across Rust emission and Python reception. Consolidate mechanical field
metadata only where it removes actual duplication; introduce no cross-language
generator or format migration as a prerequisite. If the wire format eventually
changes, version it explicitly and retain historical readers/receipts.

**Exit:** focused accounting, lifecycle-transport, native-receipt and affected
cohort/core-verifier controls preserve legacy healthy inputs, the 373/374 case,
missing/extra/reordered/duplicate fields, invalid numeric types/ranges, overflow,
unknown cases, prefixes and over-ceiling rejection. Run the admitted Python
typing checks and actual Rust emitters when changed. Qualify changed receiver
bytes in a new receipt; preserve earlier receipts. Re-admit changed inputs and
reuse unaffected evidence explicitly instead of automatically repeating the
747-variant historical campaign.

#### QF-03 — vary the offer used by response-negotiation fuzzing

**Disposition:** valid relational-fuzz gap, not a demonstrated negotiation bug.
**Owner/record:** wire/protocol tester in the existing
[S0.1 negotiation task](../../tasks/s0.1-protocol-comparison.md#pending-ntpv4-negotiation-implementation--1-november-2026).
**Timing:** before broadening negotiation/TLS integration. CHECK-6/9/25 apply.

The [target](../../policy/ntp-wire-fuzz-target.rs) already fuzzes arbitrary request
bytes. Response negotiation, however, always uses the literal offer `[15,16]`;
the response oracle hardcodes those IDs too. Existing integration tests already
cover variable/full-width identifiers, ordering, duplicate offered IDs and
unoffered selection. Extend coverage of the relationship without changing that
accepted protocol policy or claiming crypto-provider support.

Add a bounded input adapter that supplies actual request/response slices, retaining
the existing raw-message checks. Independently traverse request bytes to determine
the offered identifiers; expected membership must not use the production offer
accessor or negotiator. Include the same response accepted for one offer and
rejected for another, non-first/full-width/zero selections, overlapping/disjoint
offers, reordered/duplicate IDs, malformed duplicate records, unknown critical
records, refusals and missing cookies. Preserve request immutability and pending
output checks. A deterministic membership/context fault needs an intended witness;
label harness-observation controls separately from actual product mutations.

**Exit:** literal pair seeds verify slicing/forwarding and independent healthy/
rejected neighbors. Update target hash, seed expectations and runner controls
together; run focused wire-runner controls and its configured fuzz lane. Preserve
the existing aggregate 131,073-byte input cap, 256-run/time/RSS/allocation limits;
retain per-message boundary coverage in the raw path. Fuzz still exercises all
four owners regardless of `--subject`. Bind source/corpus/binaries and retain
earlier results. Harness-only changes need no automatic replay of unaffected
product coverage/mutation campaigns; changed product owners require their gates.

**Next permitted action:** the current engine owner finishes and records the
existing S26 checkpoint, then takes QF-01 before expanding production state.
QF-02/QF-03 can proceed at their named checkpoints with separate writers for
shared policies/runners. Preserve ordinary WIP development, held trust/provider
choices, current resource bounds and all existing release requirements.

**Independent review:** `/root/quality_plan_review`, fresh context
(`fork_turns: none`), completed read-only with no material findings;
**ready for the S26 → QF-01 handoff**. It inspected the supplied feedback,
affected source/tests/runners/policies, task checkpoints and canonical guidance.
The reviewed amendment, before this receipt, had SHA-256
`db9679b6dce12b973c81eeb161388fcddc41e46634a6644f5180e3734ae31866`.
Its comprehension record is retained verbatim:

> The amendment converts three review suggestions into bounded follow-ups: couple terminal engine outcomes to their payloads, replace positional accounting recipes with named records, and fuzz response negotiation against changing requests. It schedules these within existing tasks and preserves the current S26 checkpoint.

> The current code contains a two-peer loopback engine with one-way slot transitions, borrowed pending NTS-KE types, atomic fixture-core updates, and strict accounting receipts. These provide narrower guarantees than production acquisition or native completion; the plan retains those limits.

This review ran no product tests and requalified no historical receipt. Plan
validation checked all 86 local links/anchors and whitespace; existing engine
work and the index were preserved.

### 8.4 Protected optional-storage design and qualification

**Owner request, 2 November 2026:** record the reviewed VBS/TPM/fallback design
in architecture and plan, then inspect upstream implementations. The
[canonical contract](../windows-time-architecture.md#1212-protected-optional-storage-and-explicit-fallback)
owns state/protection/authority; the [focused plan](windows-protected-storage.md)
owns staged evidence, ST-T01–ST-T18 and retained A/HR review dispositions.
The completed [upstream source inventory](windows-protected-storage-upstream-lessons.md)
records MPSS, Tailscale, Chromium, VbsEnclaveTooling, Smallstep and
go-attestation mechanisms, exact source revisions, adaptation limits and planned
checks. **Owner follow-up, 3 November 2026:** incorporate the deeper research in
documentation, this plan and executable agent assignments. The
[second source inventory](windows-protected-storage-upstream-lessons-2.md)
adds directory authority, indeterminate commit/recovery semantics, durability
models and event monitoring, per-call CNG contracts, behavioral mitigation
checks, completion ownership and memory/output privacy. It retains
RedirectionGuard as a candidate and attestation/hardware rollback as conditional
investigations. The [protected-storage task](../../tasks/windows-protected-storage.md)
owns ST-W01–ST-W07 assignments, prerequisites and future candidate evidence;
these work-item labels introduce no acceptance catalogue.

The construction/provider/build/account/profile freeze remains open. This is a
documentation and task-planning amendment. Storage experiments remain paused;
current S1.3, P0–P7 order, held TLS/provider gates and UI-M0 scope remain unchanged.

| Existing stage / owner | Bounded follow-up | Exit / retained boundary |
| --- | --- | --- |
| Source/design; security/platform | Use both attributed inventories and [ST-W01](../../tasks/windows-protected-storage.md#st-w01-prepare-one-construction-and-failure-contract); prepare one construction row, operation-outcome table and ownership/reconciliation map | Source research recorded; first source-only assignment ready; exact design freeze open; no sample-to-production or source-to-native-pass promotion |
| Portable contracts/model; storage/QA | After the relevant contract is ready and the slice is authorized, implement [ST-W02](../../tasks/windows-protected-storage.md#st-w02-enforce-operation-outcomes-and-model-durability): separate live/durable/acknowledged state, requested/successful/failed barriers, indeterminate outcomes and poisoned owners; challenge recovery and late publication | ST-T05/06/07/10/15/16/18 with their existing CHECK/DNS-C mapping; no execution claimed by this plan and no model-to-native promotion |
| S2.3/P3; protocol/storage | Specify root/data/snapshot identities, witness/catalogue commit, authenticated exact envelope and deliberate no-root NoPersistentCache | NTS trust/ages and mandatory broker authority remain independent; no native execution until pause is lifted |
| P3/P6; platform/lifecycle | Apply [ST-W03–ST-W07](../../tasks/windows-protected-storage.md#3-later-implementation-and-qualification-work) to directory handles, CNG calls, mitigation startup/behavior, pipe completion, sensitive memory and actual diagnostic artifacts; qualify final service/key/file owners, complete root creation/reopen and interruptions/retirement | Applicable CHECK-1/9/11/12/15/16/17/18/20/23/25/27; healthy operations beside denials; RedirectionGuard support/disposition unresolved; native work held |
| Future P5/D2; UI | Consume actual mode/readiness/freshness; notify on first connection after unattended transition | CHECK-14/17/22; acknowledgement changes no policy; no UI-M0 implementation |
| Future DNS-4; DNS/privacy | Separate keys, bounded snapshots and privacy barriers across retired/inaccessible data | DNS-C12–DNS-C15; ordinary DNS independent of learning persistence |

Recommended PreferVbsAllowTpm + ContinueWithoutRestore remains a proposed
product default. No automatic DPAPI/software/plaintext persistence is selected.
Recording the design accepts no residual risk and completes no memory-encryption,
ASVS, performance or native qualification.

Use the task's [conditional reserves](../../tasks/windows-protected-storage.md#4-conditional-reserves-and-re-entry-triggers)
only when their named question becomes relevant. A hardware revocation epoch
does not prove exact latest-snapshot freshness or close same-epoch/offline/VM
rollback. No NV-counter integration, remote attestation, stream format, database,
dependency, extra privileged service or mitigation exception is selected here.

### 8.5 Windows 11 integration research and qualification

**Owner request, 3 November 2026:** integrate the completed Microsoft Windows 11
investigation into maintained documentation, architecture, this plan and agent
tasks. The [detailed research](windows11-integration.md) retains primary sources,
API/build/capability limits, proposed controls and the completed source/fit review
against `a6c2401b9a8166515293653f14f637e5543c689e`, tree
`7d0558ab295a7859e0d7d9e5b1180bea0fe30443`. The
[agent task](../../tasks/windows11-integration.md) owns W11-W01–W11-W10, their
prerequisites and subsequent candidate-specific evidence. These are local work
labels, not new CHECK, DNS-C, ST-T or release gates.

**Status:** research recorded; integrated documentation review/validation is
tracked in the [task receipt](../../tasks/windows11-integration.md#41-integration-review-and-documentation-validation-receipt).
Implementation, native Windows and installed-MSI qualification are unexecuted by
this amendment. The next source-only action can prepare
[W11-W01's timestamp-provenance card](../../tasks/windows11-integration.md#2-next-source-only-assignment)
when the acquisition owner reaches it. It does not displace section 7's current
S1.3/engine work or require a broad API survey before that work continues.

| Existing stage / owner | Bounded assignment and observable exit | Existing acceptance ownership |
| --- | --- | --- |
| Source contract → S2.1/P2; engine/platform | W11-W01: distinguish application/QPC/NIC timing; bind actual packet/interface/socket/exchange identity and conversion uncertainty. Healthy timing and unsupported/stale/saturated cases have explicit outcomes; software comparison precedes optional hardware work. | CHECK-6/11/21/27; CHECK-7/25 only for affected downstream numerical/construction invariants at their existing scopes; no source-to-native promotion |
| S0.4/P4; platform/clock | W11-W02: qualify timer/deadline semantics, Modern Standby quiescence, actual delivered correction and residual cleanup. A system clock excluding sleep does not prove broker execution. Process leap presentation remains separate from leap ownership. | CHECK-10/11/12/21/25/27; DNS-C05/11 for affected consumers |
| DNS-0/1/3; DNS/platform/lifecycle | W11-W03: freeze native field/ownership contracts, exclusive loopback readiness, bounded reconciliation and namespace/application coverage. DHCP-origin restoration remains unproven by an effective server list; private-name failure cannot become public DoQ. | DNS-C01/02/03/11/17; applicable CHECK-13/25/27 |
| P0/P3/P6; platform/security/lifecycle | W11-W04/05: evaluate WSH before custom WFP, observe actual service-setting activation, compose child policy with process-handle restrictions and extend maintenance admission to Restart Manager. Healthy installed workflows accompany forbidden-network/rights/startup cases. | CHECK-1/9/12/13/15/16/23/27, with CHECK-9 limited to delegated certificate retrieval; applicable ST-T03/10/12/17 and DNS-C14/17 |
| DNS-4 shadow → DNS-5 refresh; DNS/resources | W11-W06: bounded power/cost/memory observations influence only eligible optional work. Persistent pressure signals cannot spin; required foreground/time work, retained-reader charges and fixed caps remain. EcoQoS needs actual suitable thread placement and measured benefit. | DNS-C05/08/11/12/18; applicable CHECK-21/27 |
| DNS-2; DNSSEC/crypto/performance | W11-W07: evaluate CNG SIMD hash batches only for an evidenced workload, with independent result parity and latency/cost comparisons. Provider handles and multicore scheduling retain separate ownership; no implicit graph selection or storage-root qualification. | DNS-C04/12/16; applicable CHECK-20/21/25/26/27 |
| Affected native phase; diagnostics/QA | W11-W08: correlate bounded redacted WPR/WPA events and record ETW loss; admit one exact low-resource profile with healthy/fault controls before use. PerfLib V2 remains optional coarse monitoring. | CHECK-17/21/27; applicable ST-T14/16 and DNS-C12/14 |
| P5 after existing D2/D3 triggers; UI/accessibility | W11-W09: current-state UIA/local notifications, actual Narrator behavior and current-generation action checks. Assess deployment costs before selecting Windows App SDK; ordinary ticking status cannot create announcement spam. | CHECK-14/17/22; applicable ST-T01/14; UI-M0 D1/D2/D3 remain Deferred |
| Conditional existing owner; no current prerequisite | W11-W10: re-enter Job Objects/triggers, LPAC/app isolation, stricter CET/EHCONT, full enclaves, identity/WinGet or W32Time-provider alternatives only for a named need and separately bounded comparison. PPL remains excluded as an ordinary utility-service option. | Selected reserve reuses its existing architecture/storage/package owners; no blanket gate or default adoption |

Preserve DoQ-only upstream and leaf/intermediate pinning, the selected service
topology, one complete MSI and the independent mandatory-state owner. Native DNS
policies are coexistence inputs, not permission to add DoH/DoT/DNSCrypt/plaintext
fallback or enable ZTDNS. Hardware timing, CNG batching, PerfLib, EcoQoS and
notification packages remain optional mechanisms; a required product property
still needs evidence even when its suggested mechanism is not selected.

The existing pause on native experiments remains. No dependency/tool install,
service/key/clock/policy mutation or release qualification follows from this
integration. Later execution uses the applicable existing authorization and
gates, with source/model, real Windows and final installed-path evidence kept
distinct. Preserve UI-M0's fixture-only scope and existing D1/D2/D3 restoration
triggers; source research does not activate deferred work.


## 9. Verification commands, phase records and stop rules

Use [TOOLING.md](../../TOOLING.md#starting-implementation) and
[configured verification](../../.agents/skills/testing/references/configured-tooling.md).
Current supported entry points for UI-M0 and the S1.1 wire/domain owners, after their
inputs have been admitted, are:

```sh
python3 -B scripts/verify.py --lane portable
python3 -B scripts/verify.py --lane dependencies --advisory-receipt /path/to/current-receipt.json
python3 -B scripts/verify.py --lane windows --output-dir /path/to/new-build-output
```

Paths above are placeholders for owned actual inputs/outputs, not literal ready
commands. The current verifier/policy explicitly admits UI-M0, the pure
`ntp-wire` header/framing and checked `time-domain` arithmetic owners, fixture
`ntp-exchange` and pure `time-core`.
Further workspace expansion requires reviewed
graph/policy/runner support and discriminating gate
controls before execution. Do not weaken or bypass the verifier to make new crates
pass. Inspect test discovery and keep portable format/compiler/Clippy/rustdoc,
library/integration and separate doctest evidence distinct. Native Windows tests
and future component-specific commands must be supplied by their implementation
cards; do not invent commands for nonexistent crates.

Preserve **90% executable-line coverage for security-primitive modules** and
**95% mutation score for each critical source file**, with reviewed scope and
denominators. Repository averages, empty selections, unviable mutants, caught
errors or exit zero with no relevant tests cannot pass. Use bounded tools for
named obligations, not every available tool on every file. Documentation-only
work needs link/canonical consistency and applicable manifest checks, not a
fabricated product test run.

Every phase/slice closeout records:

```text
Phase/slice and bounded claim:
Candidate: HEAD + relevant dirty bytes/config/lockfile/fixture hashes:
Owners, changed files and actual producer -> state -> consumer/effect:
Canonical requirements and independent expected results:
Function/module contracts, enforcement and invalidation/bypass coverage:
Skills read/applied, conditional choices and reasons:
Commands/versions/targets/features; native token/OS/fixture where required:
Selected/executed/passed/failed/skipped cases; first failure and rerun reason:
Actual outputs/forbidden-effect observations; raw evidence paths/hashes:
Resource/clock/lab cleanup and verified restoration:
Independent review, findings and resolution/invalidation evidence:
Verdict: qualified within stated scope / not ready / incomplete:
Open decisions, deferred owner/trigger and next permitted task:
```

| Unresolved gate | Dependent action to keep disabled | Useful work that continues |
| --- | --- | --- |
| OPEN-1, CHECK-20/26 | Integration/execution/distribution of unadmitted TLS/native/build graph | Source review, independent vectors and admitted pure slices. |
| OPEN-3/4, CHECK-1/2/27 | Production privileged service/IPC effects | Portable domain/state and controlled transport preparation. |
| OPEN-5/6/7/8 | Unqualified date/era/recovery/correction, helper/default-profile or trust policy | Ordinary controlled acquisition under admitted strict policy and bounded simulation. |
| OD-03/R-04 | Shipped Pool/NTS defaults without vendor/operator qualification | Explicit controlled profiles; no silent trust downgrade. |
| OPEN-2/8/13 | Unsupported Windows/native-stop/performance claim | Qualified supported rows and bounded experiments; keep unavailable rows incomplete. |
| OPEN-10 and D1–D3 | Premature production UI/accessibility/preview claim | Existing restricted UI-M0 and unrelated engine work. |
| OPEN-9/11/14 | Unsupported compliance, signing, installer or public-release claim | Independent component work and already permitted laboratory experiments. |

Use the canonical [open-decision register](../windows-time-architecture.md#22-open-decisions-and-release-blockers)
for the complete choices and owners. Do not make up a permissive default, change
an oracle, suppress a finding, relax native protections or rewrite a requirement
to obtain a pass. Bring David only a concrete unresolved decision with evidence,
options, recommendation and dependent action; silence is not approval.

## 10. Plan review and execution status

This plan reorganizes the existing P0–P7/feature-slice roadmap. It retains all
canonical technical, licence, privilege, UI deferral and release obligations.
David's additions make explicit-contract implementation and phase-specific skill
reading/testing mandatory. No architecture policy is superseded.

| Record | Status |
| --- | --- |
| Source and canonical-doc inspection | Completed against the baseline above; current UI source distinguished from planned production components. |
| Skill-pack reading and phase mapping | All 58 repository `SKILL.md` leaves read: coordinator covered 4 planning, 5 implementation and 1 environment; bounded reviewers covered 37 testing, 7 security and 4 upstream leaves. Section 5 separates mandatory phase reading from triggered use. |
| Independent plan review | Completed by fresh reviewer `independent_plan_review` with conversation inheritance disabled, including a delegated skill-map check. No material findings; ready only for section 7's bounded next task. |
| Document checks | Local links/Markdown anchors and all 58 skill references resolve; phase/canonical mapping reviewed; whitespace checks pass. No root `docs/SHA256SUMS.txt` exists, so no root bundle-integrity claim is made. |
| Product implementation/tests | Not executed by this planning task. P0–P7 remain open; UI-M0 retains its separate existing record and gaps. |
| Subsequent S0.1/R-01 investigation, 29 October 2026 | [Source readiness and S1.1 card](../../tasks/s0.1-source-readiness.md), with the Chrony ledger, independent protocol/crypto comparisons and literal vectors. Source/contract investigation; no product implementation, graph admission, native qualification or phase exit. |
| Active whole-plan implementation goal, 29 October 2026 | David authorized implementation of the full plan. [S1.1a](../../tasks/s1.1-header.md), [S1.1b](../../tasks/s1.1-framing.md) and [S1.1c](../../tasks/s1.1-time-domain.md) are qualified within their declared pure/header/framing/numerical scopes. The [fresh-reviewed S1.1d card](../../tasks/s1.1-exchange-association.md) now owns fixture association and its separate graph/runner admission. Existing owner-decision, dependency/native qualification and public-release approval gates remain. |
| Protected-storage research and assignments, 3 November 2026 | The [second source inventory](windows-protected-storage-upstream-lessons-2.md) and [task record](../../tasks/windows-protected-storage.md) connect reviewed mechanisms to ST-T01–ST-T18 and the existing phase owners. This records research and future work; construction/default/provider decisions, implementation and native evidence remain open. The task owns the integration review and later candidate-specific receipts. |

Append phase readiness and task/evidence links here as development proceeds;
preserve this plan as the roadmap and each existing task as its candidate record.

### Independent review record

The completed review inspected the substantive draft with SHA-256
`48c917813da4fa9e9f334a31fee79c361405ecf6eff9dbb955c2854eb8b83913`
and documentation guide SHA-256
`1f92e3885dd133536bd7d2cab76a9e1d466a2f7df9368715bea607e89d2f21f1`.
Final editorial changes add navigation, analytic-coordinate/locale clarification
and this completed evidence record; they do not change phase authority.

Attributed comprehension, preserved from `independent_plan_review`:

> **Independent reviewer comprehension — proposed change:** The plan organizes existing P0–P7 requirements into executable development stages, prioritizing bounded NTP/NTS acquisition and independent numerical verification. It requires pinned upstream evaluation, explicit enforced function/module contracts, phase-specific skill reading, and evidence-based acceptance after each phase.

> **Independent reviewer comprehension — actual system:** The checkout contains one fixture-only native UI crate with checked transient preview editing and retained portable/native observations. Production core, networking, services, IPC and packaging remain planned. UI-M0’s full native-memory instrumentation remains incomplete; D1–D3 remain deferred until their specified integration or preview triggers.

The reviewer inspected the full plan/guide, repository instructions, review skill
and handoff procedure, testing contract, owner decisions, relevant architecture
sections 6–8/16.5/17–22, feature sections 8–9, research R-01, current UI-M0
addenda, actual manifests/source excerpts and relevant implementation/testing
guidance. Its separate skill check inspected actual contract, dependency, crypto,
contract-testing and qualification bodies. Review completed read-only: no product
tests, dependency installation or native qualification. Verdict: **ready for
S0.1/R-01 and S1.1 preparation under the subsequent task instruction**; later
graph, policy, Windows, UI, package and release gates remain open. Revalidate the
affected review conclusions if their source, contract, skill or phase scope changes.

<!-- Full skill paths are centralized here to keep the phase map readable. -->

[dependency-updates]: ../../.agents/skills/environment/dependency-updates/SKILL.md
[evolve-data-contracts]: ../../.agents/skills/implementation/evolve-data-contracts/SKILL.md
[implement-native-ui]: ../../.agents/skills/implementation/implement-native-ui/SKILL.md
[implement-rust]: ../../.agents/skills/implementation/implement-rust/SKILL.md
[implement-rust-contracts]: ../../.agents/skills/implementation/implement-rust-contracts/SKILL.md
[optimize-rust-performance]: ../../.agents/skills/implementation/optimize-rust-performance/SKILL.md
[plan-architecture]: ../../.agents/skills/planning/plan-architecture/SKILL.md
[plan-code-change]: ../../.agents/skills/planning/plan-code-change/SKILL.md
[plan-independent-review]: ../../.agents/skills/planning/plan-independent-review/SKILL.md
[plan-threat-model]: ../../.agents/skills/planning/plan-threat-model/SKILL.md
[qa-accessibility]: ../../.agents/skills/testing/qa-accessibility/SKILL.md
[qa-adversarial-inputs]: ../../.agents/skills/testing/qa-adversarial-inputs/SKILL.md
[qa-black-box-acceptance]: ../../.agents/skills/testing/qa-black-box-acceptance/SKILL.md
[qa-boundary-edge-cases]: ../../.agents/skills/testing/qa-boundary-edge-cases/SKILL.md
[qa-component-tests]: ../../.agents/skills/testing/qa-component-tests/SKILL.md
[qa-context-checkpoint]: ../../.agents/skills/testing/qa-context-checkpoint/SKILL.md
[qa-contract-tests]: ../../.agents/skills/testing/qa-contract-tests/SKILL.md
[qa-coverage-gap-analysis]: ../../.agents/skills/testing/qa-coverage-gap-analysis/SKILL.md
[qa-data-flow-analysis]: ../../.agents/skills/testing/qa-data-flow-analysis/SKILL.md
[qa-e2e-tests]: ../../.agents/skills/testing/qa-e2e-tests/SKILL.md
[qa-environment-preflight]: ../../.agents/skills/testing/qa-environment-preflight/SKILL.md
[qa-failure-triage]: ../../.agents/skills/testing/qa-failure-triage/SKILL.md
[qa-fault-recovery]: ../../.agents/skills/testing/qa-fault-recovery/SKILL.md
[qa-final-qualification]: ../../.agents/skills/testing/qa-final-qualification/SKILL.md
[qa-golden-fixture-parity]: ../../.agents/skills/testing/qa-golden-fixture-parity/SKILL.md
[qa-i18n-localization]: ../../.agents/skills/testing/qa-i18n-localization/SKILL.md
[qa-independent-review]: ../../.agents/skills/testing/qa-independent-review/SKILL.md
[qa-integration-tests]: ../../.agents/skills/testing/qa-integration-tests/SKILL.md
[qa-model-based-testing]: ../../.agents/skills/testing/qa-model-based-testing/SKILL.md
[qa-mutation-testing]: ../../.agents/skills/testing/qa-mutation-testing/SKILL.md
[qa-negative-paths]: ../../.agents/skills/testing/qa-negative-paths/SKILL.md
[qa-package-artifacts]: ../../.agents/skills/testing/qa-package-artifacts/SKILL.md
[qa-performance-resources]: ../../.agents/skills/testing/qa-performance-resources/SKILL.md
[qa-positive-paths]: ../../.agents/skills/testing/qa-positive-paths/SKILL.md
[qa-pre-commit]: ../../.agents/skills/testing/qa-pre-commit/SKILL.md
[qa-property-based-testing]: ../../.agents/skills/testing/qa-property-based-testing/SKILL.md
[qa-regression-selection]: ../../.agents/skills/testing/qa-regression-selection/SKILL.md
[qa-security-boundaries]: ../../.agents/skills/testing/qa-security-boundaries/SKILL.md
[qa-state-concurrency-lifecycle]: ../../.agents/skills/testing/qa-state-concurrency-lifecycle/SKILL.md
[qa-static-checks]: ../../.agents/skills/testing/qa-static-checks/SKILL.md
[qa-structured-fuzzing]: ../../.agents/skills/testing/qa-structured-fuzzing/SKILL.md
[qa-subagent-dispatch]: ../../.agents/skills/testing/qa-subagent-dispatch/SKILL.md
[qa-system-stabilization]: ../../.agents/skills/testing/qa-system-stabilization/SKILL.md
[qa-test-orchestrator]: ../../.agents/skills/testing/qa-test-orchestrator/SKILL.md
[qa-test-planning]: ../../.agents/skills/testing/qa-test-planning/SKILL.md
[qa-unit-tests]: ../../.agents/skills/testing/qa-unit-tests/SKILL.md
[qa-visual-dpi]: ../../.agents/skills/testing/qa-visual-dpi/SKILL.md
[rust-borrow-semantics]: ../../.agents/skills/upstream/rust-borrow-semantics/SKILL.md
[rust-compiler-errors]: ../../.agents/skills/upstream/rust-compiler-errors/SKILL.md
[rust-send-sync]: ../../.agents/skills/upstream/rust-send-sync/SKILL.md
[security-build-deployment]: ../../.agents/skills/security/security-build-deployment/SKILL.md
[security-crypto-secrets]: ../../.agents/skills/security/security-crypto-secrets/SKILL.md
[security-local-ipc]: ../../.agents/skills/security/security-local-ipc/SKILL.md
[security-native-ui]: ../../.agents/skills/security/security-native-ui/SKILL.md
[security-privacy-logging]: ../../.agents/skills/security/security-privacy-logging/SKILL.md
[security-protocol-inputs]: ../../.agents/skills/security/security-protocol-inputs/SKILL.md
[security-windows-privileges]: ../../.agents/skills/security/security-windows-privileges/SKILL.md
[winapp-ui-automation]: ../../.agents/skills/upstream/winapp-ui-automation/SKILL.md

