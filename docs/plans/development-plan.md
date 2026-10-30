# NevroTick development and acceptance plan

**Date:** 29 October 2026  
**Status:** reviewed; ready to assign the next bounded task; implementation phases remain open  
**Inspected source baseline:** `f7ba1e7724a41cc846b1df6b9ddb20f095988b7a`, initially clean  
**Immediate priority:** secure, bounded and performant NTP/NTS acquisition, with independently verified algorithms  
**Current task:** [S1.3 source-selection fixture](../../tasks/s1.3-source-selection-readiness.md), following the separately qualified bounded [main S1.2 runs integration](../../tasks/s1.2-regression-readiness.md#main-fixture-qualification-result-30-october-2026). Checked constructor, batch and evaluator progress belongs to the live S1.3 record; old five-owner receipts do not qualify the new owner. The required [OD-07 CPU workstream](#81-x64-baseline-and-runtime-optimization-workstream) starts at the next controlled build-policy checkpoint and continues with measured runtime specialization. Independent S1.3 work can proceed; full S1.2/P1, native Windows and release gates remain open.

Agent navigation: [implementation contracts](#3-mandatory-explicit-contracts-during-implementation),
[phase skill map](#5-required-skill-reading-and-use-by-phase),
[phase acceptance](#6-phase-implementation-and-acceptance-gates),
[next task](#7-next-executable-task-for-a-development-agent),
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
| Cargo | [Workspace](../../Cargo.toml), lockfile, fixture [UI](../../crates/nevrotick-ui/Cargo.toml), inert [wire](../../crates/ntp-wire/Cargo.toml), checked [time-domain](../../crates/time-domain/Cargo.toml), fixture [exchange](../../crates/ntp-exchange/Cargo.toml) and pure [time-core](../../crates/time-core/Cargo.toml) owners exist. | Inspect current files and linked task evidence; fixture numerical source does not establish the complete core. |
| UI | `main` → native adapter → checked fixture model → standard Win32 controls. Preview edits are transient. | Keep UI-M0 isolated; no network, production IPC, persistence, service, elevation or clock effects. |
| UI evidence | [UI-M0 task](../../tasks/ui-m0.md), including later candidate addenda, records portable, cross-build and native observations. Full required native-memory instrumentation remains incomplete. | Historical results belong to their exact candidates. Short interaction/resource observations do not establish CHECK-21 soak or full UI qualification. |
| Production software | Header/framing, checked arithmetic, fixture exchange and pure regression/history exist; production sample admission, source selection, network engine, broker, SCM services, production IPC and MSI remain planned. | Ownership names below are proposed modules/crates; create only what an authorized slice needs. |
| Dependencies | Five local owners retain the sixteen-package UI-M0 external cohort; exchange and core use reviewed local edges and the existing minimal garde graph. Production TLS/provider/AEAD and service graphs remain subject to admission. | Integrated graph checks apply to their recorded source/features. Older “no Cargo workspace” statements in retained research are historical, not source inventory. |

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
S2.3/S2.4 exercise cache recovery, retry and resource failure paths. Keep graph,
Windows trust and interoperability gates distinct from this documentation review.

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
