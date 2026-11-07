# Post-V1 multithreading and concurrent NTS acquisition

**Document:** NV-MT-001, revision 1.0  
**Owner instruction:** David Osipov, 7 November 2026  
**Roadmap stage / exit:** MT-0 / MT-C01  
**Status:** planned; implementation, experiments and qualification have not run  
**Activation:** immediately after the first software version (V1) has actually been released; first post-V1 TODO, before Khronos/Ananke  
**Planning baseline:** [e5d9a0112e198ab41a50ee0b0057e06eab68ab84](https://github.com/DavidOsipov/NevroTick/commit/e5d9a0112e198ab41a50ee0b0057e06eab68ab84)  
**Implementation/evidence owner:** existing [engine transport task](../../tasks/engine-transport-pilot.md#post-v1-multithreading--mt-0-7-november-2026), with platform/protocol/performance owners and an independent reviewer

## 1. Decision, scope and authority

[OD-16](../nevrotick-owner-decisions.md#20-od-16--multithreading-first-after-v1)
records the accepted scheduling decision:

**V1 released → MT-0 / MT-C01 → KHR-0 / KHR-C01 → GEO-0 / GEO-C01 → Android.**

The [roadmap](../../ROADMAP.md#mt-0--post-v1-multithreading-and-concurrent-acquisition)
owns this order. Completing P0–P7 planning, compiling a candidate or creating a
release tag alone does not establish that V1 was released. At activation, identify
the actual released source/artifact and release record under the existing
[release policy](../release-policy.md).

The work evaluates and implements justified multithreading improvements, starting
with independent complete NTS acquisition pipelines and responsiveness during
blocking native work. Extend the released engine and its existing task/evidence
chain; do not build a competing engine or restart S2.2 from an old prototype.

**V1 boundary:** this is a post-V1 enhancement plan. Existing V1 correctness and
lifecycle duties remain required, including isolation of synchronous certificate
work, bounded native-job admission, actual-completion custody, current policy
checks and responsive control. They are already architecture requirements and
are not deferred by MT-0. The current [development continuation](development-plan.md#7-next-executable-task-for-a-development-agent)
retains its order before release.

Current Windows, fuzz, storage and UI execution holds remain until their actual
owners resolve them. V1 release is the scheduling trigger, not an automatic waiver
of an unresolved hold or dependency/native gate. OD-10's formal work remains
post-V1; eligibility after release does not put the whole proof backlog before
MT-0. Apply its product-guarantee and finite-budget rules to any relevant resumed
proof work.

### Retained and changed obligations

| Surface | Disposition |
| --- | --- |
| Earlier first-Khronos scheduling | Superseded by David's 7 November instruction: MT-0 is first after actual V1 release. |
| Khronos → GeoIP → Android | Relative order retained; MT-0 is inserted before Khronos. RGT-0 → CHR-0 retains its separate later scope. |
| Current-thread engine experiment | Retained as the inspected reference, not permanently selected for the post-V1 product or claimed superior. |
| Source admission, independence and clock authority | Unchanged. Concurrency cannot lower a source minimum, add votes or grant workers broker privileges. |
| Worker count, provider and runtime choice | Open. Neither four workers nor multithread Tokio is adopted by this plan. |
| Resource and release evidence | Required at the affected existing CHECK owners; research and a plan are not executed acceptance. |

Read architecture [§17.1](../windows-time-architecture.md#171-bounded-async-and-native-work),
[§18.6.1](../windows-time-architecture.md#1861-engine-runtime-decision-and-alternatives),
[§18.7.1](../windows-time-architecture.md#1871-license-gate-and-finite-provider-choice),
§7.3 policy cutover, and the existing
[S2.2 contract](../../tasks/engine-transport-pilot.md#ordinary-full-nts-composition--s22-preparation-5-november-2026).
MT-W/MT-T/MT-C labels below are local navigation IDs, not replacements for
REQ/BND/DEC/THR/CHECK requirements.

## 2. Research baseline and open hypothesis

At the planning pin, the engine's [manifest](../../crates/nevrotick-engine/Cargo.toml)
selects Tokio 1.54.1 with rt/net/time/sync/macros and no rt-multi-thread feature.
The [two-peer pilot](../../crates/nevrotick-engine/src/lib.rs) multiplexes sockets
on a current-thread runtime, configures max_blocking_threads(2), and explicitly
uses no blocking jobs. The [series fixture](../../crates/nevrotick-engine/src/series.rs)
is intentionally sequential. The [opt-in Windows tray](../../crates/nevrotick-ui/src/native_tray/windows.rs)
does start an OS thread. These are scoped source observations, not a production
thread census or complete NTS benchmark.

The 7 November research used separate Tokio, Windows and NTS reviewers and a
separate judge. The judge required current-policy checks at each new I/O issue
boundary, and clarified that one select! remains one task even on a multithread
scheduler. Both corrections are retained here. Its acceptance concerned the
research handoff, not this plan's execution or a runtime implementation.
Research handoff SHA-256:
5cb81768373897f35c6bc4f6a88efeb6689452a84cfccaa7db47663a89e4052c.
Relevant primary sources are listed in §10; this plan is usable without the
conversation artifact.

**Hypothesis:** independent source pipelines can reduce acquisition/recovery
waiting when some sources are slow, while bounded worker isolation preserves
control and existing UDP progress. Additional threads may increase peak CPU,
memory and contention. Lower energy use or better timing accuracy is unmeasured.

With two occupied native-job slots, two stalled calls can prevent healthy sources
C/D from starting. An explicitly admitted four-slot candidate could allow C/D
to progress if the worker and other limits also permit it and no shared resolver
or trust-provider bottleneck remains. Four stalled calls still saturate four
slots. C/D progress does not establish enough independent evidence for selection.

Re-inspect the actual released V1 design before using this example. Its source,
graph, runtime, admission limits and qualified resource budgets may differ from
the research pin.

## 3. Execution design to evaluate

| Candidate | Purpose | Qualification condition |
| --- | --- | --- |
| A: responsive current-thread coordinator plus bounded complete KE workers | Existing architectural reference; workers absorb synchronous native calls and own complete exchanges. | Show useful-source progress, bounded custody and control/UDP responsiveness under saturation. |
| B: explicitly bounded multithread Tokio plus isolated blocking work | Credible mainstream alternative for separately scheduled runnable tasks or a simpler compliant integration. | Same native limits and workload for comparison; explicit async worker count, Send/affinity and feature-graph review. |
| C: bounded synchronous workers with an explicit coordinator/readiness loop | Existing alternative if the chosen runtime has a demonstrated budget, lifecycle or complexity problem. | Same contracts and metrics; no second complete engine required merely to decide. |

First change one capacity parameter under equivalent conditions. Investigate
scheduler topology separately when a measured problem or concrete integration
need warrants it. Tokio recommends its multithread scheduler for most applications,
but that recommendation is not a NevroTick performance result. [P1]

A runtime change cannot make a sequential outer await loop concurrent, split one
select! across cores, or preempt that task's long poll. Distinguish delay caused by
other tasks from excessive synchronous work inside the state owner. [P1] [P2]

### Producer-to-consumer path

For each source, preserve ordered dependencies:

discovery/destination admission → TCP/TLS/ordinary validation → KE negotiation,
exporters and cookies → any delegated-endpoint resolution → complete association
publication → protected UDP → numerical eligibility → actual history/selection.

Different sources may progress independently. Do not add a global barrier that
waits for every KE before any valid source can poll. Normal protected UDP reuses
a valid association; fresh TLS on every poll is not the NTS design. [P7]

| Owner | Contract |
| --- | --- |
| Coordinator/state owner | Fair source admission, current revisions, atomic association publication, consume-once acceptance, history/selection input and terminal-result retirement. |
| KE/native worker | Owned admission, exact native inputs/handles, connection/exporter binding, private bounded candidate, cleanup and typed outcome. No direct live-history, policy or clock mutation. |
| UDP/request owner | Exclusive cookie/request custody, actual timestamp envelope, saved source/socket/endpoint identity and staged packet effects. |
| Core | Existing checked numerical transformations on admitted data; no runtime dependency by contagion. |
| Broker/lifecycle owner | Existing authorization, effects and recovery; no dependence on a saturated network-work queue. |

One logical state owner may run as a task on a multithread scheduler. Actual
thread-affine objects still require explicit ownership and an appropriate
adapter. Do not add unsafe Send/Sync to evade those requirements.

## 4. Mandatory ownership and resource contracts

### Admission and capacity

Freeze separate limits for logical sources, ready/deferred source records,
complete KE operations, submitted native jobs, blocking threads, async workers,
address alternatives, pending UDP requests, completion slots and bytes. Include
retained join results and allocations outside queues.

The inspected P0 contract starts with two submitted native jobs including queued
jobs, no separate backlog, and at most one acquisition per endpoint. Its KE
two-operation/20-second/128-KiB values are qualification candidates. On activation,
use the actual accepted V1 limits as the baseline; any experiment changing them
must be explicit. A thread cap does not supply application admission. [P3]

Acquire before submission and move custody into the actual job. Define canonical
source/endpoint keys, bounded ready state, fair admission and per-source retry
rules. Address racing and hidden DNS/file/provider work must fit the aggregate
budget. Do not hold the last pool slots while waiting synchronously for child
work requiring the same pool.

### Policy and publication

Captured generations are not continuing authorization to issue I/O. After a
wait, recheck current destination/path/policy at each new owned connection, send
or delegated-I/O issue boundary. The adapter must define how check and issue are
ordered against cutover.

Keep saved policy separate from effective acquisition policy. Stop old-profile
admission and publication eligibility; retain its actual native custody. If the
new boundary is not established, show blocked/indeterminate activation instead
of silently resuming the old route. Already-submitted traffic cannot be recalled.
Delegated resolver/PKI traffic needs its own enforceable boundary and honest limits.

Publish keys, cookies, algorithm, trust basis, admitted endpoint and generations
together. Failed or stale work cannot partially replace an association. Retain an
old association only while independently valid. Reserve cookies exclusively;
ambiguous sends cannot silently replenish the unused inventory. Reject unrelated,
malformed or unauthenticated packets without consuming a healthy pending request.

### Cancellation and completion

Distinguish logical expiry, cancellation requested, native completion, cleanup,
result delivery, owner acceptance and result retirement. Started blocking work
can continue after its waiter stops. A timeout is not a native termination
mechanism; revocation retrieval timeouts have narrower scope than the full
certificate/TLS pipeline. [P3] [P4] [P5]

Retain permits and native-accessible storage until actual completion. If results
outlive native slots, reserve a separate bound for their count and bytes; a
conservative whole-job lease until retirement is also possible. Preserve required
completion custody through closed/full-channel and panic paths. Cancellation of
send(value) can drop an unsent result; select a delivery mechanism deliberately. [P6]

Stop new admission first and continue draining necessary completion events. Do not
wait for a worker while blocking its only completion path. No replacement pools,
detached retry threads or new runtimes may evade saturation. No TerminateThread.
Use the existing diagnosed degradation and process-recovery policy, preserving
broker ownership and cumulative budgets. Actual native cancellation/completion
and buffer lifetime depend on the selected adapter, not the future's Drop.

### Timing and recovery

Freeze stage and total acquisition deadlines, admission waiting, final packet
acceptance, control latency and shutdown policy separately. Do not restart the
overall budget at every stage. Keep monotonic/UTC/active-awake meanings explicit.

Capture send/arrival timestamps at the qualified transport boundary and preserve
provenance through queues. Enqueue and crypto-completion times are not substitutes.
Recheck mapping/clock generations and the existing final processing deadline.

Reset retry history only after protected success using the corresponding KE
data. Delayed old-key success cannot reset a replacement attempt. Any optional
jitter preserves the selected floor/ceiling and is reproducible under tests.
Authentication, numerical eligibility and enough independent history remain
separate outcomes. Preserve the current no-downgrade, trust, cookie-age and
full-authentication-age policies.

## 5. Ordered work packages

All implementation and qualification items are open. Owners below are roles,
not an assertion that staffing or execution permission exists.

| ID / owner | Work | Exit and dependencies |
| --- | --- | --- |
| MT-W01 — coordinator/engine | After actual V1 release, inspect released source/artifact, current HEAD/dirty changes, graph, existing evidence, holds and concrete bottleneck. Map actual calls and consumers. | One bounded next-slice contract and baseline record in the existing engine task. No new runtime/provider selected. |
| MT-W02 — engine/platform/protocol | Freeze the work/result ownership, independent limits, policy-issue ordering, cancellation/stop semantics, actual blocking boundary, metrics and finite experiment budget. | Reviewed contract with independent healthy/fault expectations; unresolved native/provider decisions explicitly block only their dependent work. |
| MT-W03 — engine implementer | Extend the real released acquisition path with the smallest justified scheduling/isolation change. Preserve private candidate construction and serialized acceptance. | Focused meaningful RED→GREEN where repairing behavior, valid neighbors, bounded source/result state and unchanged time/trust authority. A model alone is not native integration. |
| MT-W04 — platform/QA | Qualify selected Windows adapters and full source-to-consumer behavior under the applicable native admission. | Scoped CHECK-6/8/9/11/17/25/26/27 evidence, real completion/cleanup and actual control/UDP progress. Missing native evidence remains incomplete. |
| MT-W05 — performance/engine | Compare admitted capacities with equivalent workloads; compare schedulers only for a named unresolved need. Measure first protected/eligible observations and selection readiness separately. | Raw matched results, rejected candidates, resource guards, uncertainty and justified production choice or evidence-supported baseline retention. |
| MT-W06 — independent reviewer/coordinator | Review exact implementation and receipts, reconcile changed contracts and relevant formal debt, and run applicable phase/release gates. | MT-C01 disposition; qualified improvement or justified retained baseline, explicit remaining debt and correct next roadmap state. |

### First post-V1 executable stage: MT-W01

**Inputs:** actual V1 release/source/artifact identity; current architecture,
development/task records and this plan; current feature graph; existing relevant
evidence and environment/hold status.

**Action:** bounded read-only inspection and profiling-plan preparation. Locate
real DNS/KE/verifier/UDP/consumer calls, every queue and native owner, the existing
source fairness rule, timestamp placement, stop path and unconsumed results.
Identify what V1 already satisfies before proposing changes.

**Independent expectation:** source paths and retained receipts support each
implemented-state claim. A configured thread ceiling is not observed utilization;
async tasks are not OS threads; mocks are not native acceptance. Name a useful
progress or responsiveness gap, or report that none is yet evidenced.

**Output:** one task entry containing candidate identity, owner/contract table,
finite next experiment, valid/fault controls, predeclared metrics/thresholds,
resource/cleanup limits and admission blockers. No new tool, provider, privileged
operation, public-server probing or runtime change is needed for this stage.

**Readiness:** deferred until actual V1 release. Documentation can be maintained
now. Afterwards MT-W01 can proceed read-only; later execution still follows its
own prerequisites. Do not use an unavailable benchmark environment as a reason
to claim a design winner.

## 6. Acceptance and comparison matrix

Use controlled endpoints and independent fixture truth. Preserve actual failed,
cancelled and timed-out runs; no same-code round-trip oracle or teardown-masked
cleanup claim. MT-T identifiers are local case labels mapped to existing CHECKs.

| Case | Healthy/fault control | Required observation / existing CHECK scope |
| --- | --- | --- |
| MT-T01 healthy overlap | Four independent healthy sources; cold/warm conditions separately. | Real overlapping stage progress, protected/eligible/readiness times and resources; 21/27 subsets. |
| MT-T02 occupied first slots | A/B held in native work, C/D healthy; reverse source order. | Honest two-slot behavior and any admitted four-slot improvement, no assumed quorum or bypassed bounds; 21/27. |
| MT-T03 full saturation | All native slots held; current valid UDP association and control active. | Responsive control/UDP, charged live jobs, bounded degraded state, no replacement growth; 14/21/27. |
| MT-T04 timeout/cancel races | Before/at/after expiry and normal completion racing cancellation. | Exactly-once terminal cleanup, no early capacity release or stale acceptance; 6/9/27. |
| MT-T05 policy cutover | Pause queued work immediately before I/O; change policy; also retain already-issued work. | No newly forbidden issue; permitted current neighbor succeeds; truthful saved/effective activation, real custody and no recall claim; 2/6/14/27. |
| MT-T06 stale generations | Source/path/trust/association/clock change while work runs. | No obsolete publication, cookie replenishment or eligible sample; cleanup still occurs; 8/11/25/27. |
| MT-T07 result backpressure | Fill status traffic, delay results, then stop. | Required completion survives, no worker/receiver wait cycle, bounded retained bytes and honest registry; 17/21/27. |
| MT-T08 packet/cookie custody | Wrong tag/UID/duplicate then valid reply; ambiguous send/exhaustion. | Healthy request preserved, one completion, correct cookie ownership and no plain fallback; 6/9/17/25. |
| MT-T09 retry outcomes | Failed KE, KE success with UDP failure, current and old-key success. | Same-transaction reset, correct rate/floor/ceiling and no storm; 6/9/27. |
| MT-T10 endpoint alternatives | Delegated hostname, delayed resolution, forbidden destination and first-address blackhole. | Complete provenance, current actual-use policy, bounded alternative progress and no extra vote; 6/8/9/27. |
| MT-T11 timestamps | Timely packet delayed in processing; mapping change while queued. | Unmodified arrival provenance and final deadline/generation rejection; 6/7/11/25/27 subsets. |
| MT-T12 aggregate/stop | One/four/sixteen configured sources; admitted maximum-size inputs; stop at each native stage. | Count/byte/thread/handle bounds, actual completion and truthful stop/recovery; 8/21/25/27 and reached service owners. |

Freeze run count, duration, repair/retry limits, sample sizes and stop criteria
before execution. Record actual Windows build/client-or-Server profile, hardware,
service account, power mode, normal protections, compiler/SDK/features, provider,
fixture versions, source order and cache state. Compare like with like and vary
one cause first.

Primary metric: time to sufficient current, independent, numerically eligible
evidence under the existing selection rules. Guard metrics include control and
receive-processing latency, CPU, private bytes, wakeups, threads/handles, native
jobs, queue depth and retained result/secret bytes. Include unsuccessful runs.
A finite maximum is not a proven worst-case bound; small samples do not support
confident tail-percentile claims.

Use the actual current resource contract from
[development §8](development-plan.md#8-performance-limits-and-qualification-environments).
The research's initial 32-MiB/four-source and idle-CPU targets were unmeasured,
not proof of V1 performance. Native memory diagnostics and ordinary resource
measurements answer different questions. Keep applicable CHECK-21 one-hour
baseline and 72-hour reconnect/fault-soak gates at their existing checkpoints;
a short comparison does not replace them.

## 7. Other operations and limits on scope

After the acquisition slice, use measured profiles to select any further work.
CPU-heavy bounded authentication/parsing may use admitted workers if it reduces
a demonstrated delay without losing timestamps or increasing unbounded results.
Independent numerical work may use immutable versioned snapshots; only the
serialized current owner can commit it.

Preserve Slint/UI affinity and the existing tray bridge. Optional diagnostics,
statistics and storage must not consume mandatory control/completion capacity.
The future [DNSSEC multicore design](system-dns-cache-design.md#33-hardware-acceleration-and-multiple-cores)
retains its separate owners and order; SIMD acceleration is not multicore
scheduling. MT-0 does not import DNS, Khronos, GeoIP, SOCKS, Roughtime or Android
implementation into its first slice.

No automatic worker-per-CPU or permanent thread-per-source rule, nested runtime
per request, new provider, custom allocator, CPU affinity, real-time priority or
global timer-resolution change is selected. Every new direct dependency or
feature graph must pass its existing admission.

## 8. Exit MT-C01 and onward scheduling

MT-0 is the first post-V1 job; Khronos does not begin merely because this plan
exists or an experiment was attempted.

Close MT-C01 with one of these evidence-backed dispositions in the existing
engine task:

- **IMPLEMENTED_QUALIFIED:** the selected improvement is implemented at the real
  consumer, required scoped checks and resource/native gates pass for the exact
  candidate, limitations and release follow-through are explicit.
- **BASELINE_RETAINED:** the bounded comparison supports retaining the released
  design; useful progress/correctness/resource expectations are met, rejected
  alternatives and their costs are recorded, and the independent reviewer verifies
  that retention is justified rather than absence of evidence.

An unavailable prerequisite, inconclusive experiment, failed guard or occupied
native slot is **BLOCKED/INCOMPLETE**, not BASELINE_RETAINED. Record cause and next
permitted action; do not silently advance to Khronos. A deliberate deferral or
reprioritization requires an explicit owner decision updating the same roadmap
chain. A qualified MT-C01 disposition admits KHR-0 subject to its own gates.

For permanent native hangs, finite capacity cannot promise unlimited new
admission. Qualify bounded degradation and the existing process-recovery contract;
increasing slots alone is not a solution to arbitrary nontermination. No claim of
speedup, energy saving, accuracy or completed release follows from source review.

## 9. Evidence and next-agent handoff

For implementation, retain one coordinator entry in the existing engine task:
released/candidate source and dirty identity, exact graph and native configuration,
owners and limits, selected cases/independent expectations, actual outcomes and
raw evidence, cleanup, consumed budget, open findings and next permitted action.
Use repository planning, Rust/native, concurrency/lifecycle and performance QA
skills as the affected boundary requires; preserve critical-file coverage/mutation
and applicable post-V1 proof obligations.

**Current work performed:** research was integrated into a deferred plan and
roadmap ordering. No engine, provider or runtime implementation, benchmark,
native test or release was performed. The document's plan review concerns
scheduling and contract completeness only.

**Next agent:** continue the current V1 development plan until the actual release.
Then start MT-W01 first, re-read the released implementation, freeze the smallest
useful comparison, preserve the contracts above, and report separate implemented,
host-tested, native-qualified and still-open results. Do not turn research
preferences into production constants.

### Documentation review record

**Review completed, 7 November 2026:** post_v1_plan_review returned **READY to
record the deferred plan and scheduling amendment**, with no material finding.
This was a distinct-agent review with inherited context; the initial
isolated-context launch failed with a context-capacity error and produced no
usable review. No blind-review or execution-readiness claim is made.

The reviewer inspected the complete eight-file diff, new plan, actual baseline
source and relevant authorities. Reviewed diff SHA-256:
79fb03485e6d20a23574a29565800e9834880b9e150a187129e37f76169d9768.
Reviewed new-plan SHA-256 before this closeout:
6934ed8db320fa5e9e734d5768bd3cfc8c4ac4600cd15e6048e15dd86da200c1.
Finalization adds this review record and normalizes the snapshot's final newline;
it selects no new implementation contract.

**Reviewer comprehension — proposed change (verbatim):**

> The candidate records David's 7 November instruction as OD-16 and inserts MT-0 as the first job after the first software version has actually been released. It adds a maintained plan under docs, with work packages and an MT-C01 exit, while keeping implementation evidence in the existing engine task. Khronos follows that exit, with the existing GeoIP, Android and separate Roughtime-to-Chrony-replacement relationships preserved. This changes future priority and prepares bounded acquisition/concurrency work; it does not start the work now or choose a runtime or worker count.

**Reviewer comprehension — current system (verbatim):**

> At the inspected source pin, NevroTick's observe-only engine pilot uses current-thread Tokio and configures a two-thread blocking ceiling without submitting blocking jobs. The Windows tray has an explicit OS thread. The architecture already requires responsive control, bounded native admission, real completion custody and isolation of the synchronous verifier before dependent integration. Those existing V1 requirements remain in force. The released V1 implementation may differ from this research snapshot, so MT-W01 must re-establish the actual released source, graph, limits and evidence before selecting a change.

**Coordinator checks:** new relative links/anchors and the first unchecked
roadmap job were checked. No product build, test, benchmark, native probe or
runtime change was performed. The next permitted action is documentation
integration; execution remains deferred under §1 and MT-W01.

## 10. Primary evidence and maintenance

Sources were checked during the 7 November research. Revalidate the exact released
dependency/native versions at MT-W01; API documentation and upstream code are
contract evidence, not performance measurements.

- **P1 — [Tokio runtime](https://docs.rs/tokio/latest/tokio/runtime/index.html):** scheduler alternatives and bounded-task/poll fairness; retrieved API identified 1.54.1.
- **P2 — [Tokio select!](https://docs.rs/tokio/latest/tokio/macro.select.html):** same-task concurrency, cancellation safety and biased polling.
- **P3 — [Tokio spawn_blocking](https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html):** separate pool, admission, started-job and shutdown limits.
- **P4 — [Rustls ServerCertVerifier](https://docs.rs/rustls/latest/rustls/client/danger/trait.ServerCertVerifier.html)** and **[Tokio timeout](https://docs.rs/tokio/latest/tokio/time/fn.timeout.html):** synchronous callback and non-preemptive deadline boundary; retrieved Rustls API identified 0.23.46.
- **P5 — [CertGetCertificateChain](https://learn.microsoft.com/en-us/windows/win32/api/wincrypt/nf-wincrypt-certgetcertificatechain)** and **[CancelIoEx](https://learn.microsoft.com/en-us/windows/win32/api/ioapiset/nf-ioapiset-cancelioex):** retrieval-timeout scope and actual native completion.
- **P6 — [Tokio bounded Sender](https://docs.rs/tokio/latest/tokio/sync/mpsc/struct.Sender.html):** completion capacity and cancellation/transfer semantics.
- **P7 — [RFC 8915](https://www.rfc-editor.org/rfc/rfc8915.html):** establishment versus protected polling and retry reset.
- **P8 — pinned chrony [client](https://github.com/mlichvar/chrony/blob/c97d8083cf9a9edc64d53236565916d326d8a109/nts_ke_client.c), [session](https://github.com/mlichvar/chrony/blob/c97d8083cf9a9edc64d53236565916d326d8a109/nts_ke_session.c), [GnuTLS adapter](https://github.com/mlichvar/chrony/blob/c97d8083cf9a9edc64d53236565916d326d8a109/tls_gnutls.c) and [server](https://github.com/mlichvar/chrony/blob/c97d8083cf9a9edc64d53236565916d326d8a109/nts_ke_server.c):** event-driven client overlap and separate incoming-server helpers. Neither supplies a Windows client worker count.

Reuse requires exact source/licence/provenance review; this research copies no
upstream implementation. Current canonical contracts and owner decisions govern
over historical source descriptions.

[P1]: https://docs.rs/tokio/latest/tokio/runtime/index.html
[P2]: https://docs.rs/tokio/latest/tokio/macro.select.html
[P3]: https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html
[P4]: https://docs.rs/rustls/latest/rustls/client/danger/trait.ServerCertVerifier.html
[P5]: https://learn.microsoft.com/en-us/windows/win32/api/wincrypt/nf-wincrypt-certgetcertificatechain
[P6]: https://docs.rs/tokio/latest/tokio/sync/mpsc/struct.Sender.html
[P7]: https://www.rfc-editor.org/rfc/rfc8915.html
