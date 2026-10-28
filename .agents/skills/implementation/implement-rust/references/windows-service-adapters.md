# Qualify Windows service and native-work adapters

Use for engine/broker SCM adapters, callback lifetimes, service accounts or
stop/restart implementation/review. First read [canonical architecture](../../../../../docs/windows-time-architecture.md)
sections 4, 9–11, 17.1, 18.6, CHECK-27 and S0.2–S0.3. Procedure adopts no dependency
or production limits.

Documentation-only stage has no Cargo/native candidate. Source review may prove
API fit/blockers; Windows runtime remains incomplete. Do not create runtime
projects, install tools, register services or change clocks merely for checklist
completion. Execute cases
only within task-authorized machine/mutation scope.

Navigate to [adapter choice](#freeze-the-adapter-choice-before-coding),
[lifecycle ownership](#identify-one-lifecycle-owner-per-own-process-service),
[status/control cases](#exercise-status-and-control-behavior),
[callback custody](#prove-callback-and-resource-custody),
[account fixtures](#qualify-actual-accounts-and-alloweddenied-effects),
[stop/recovery](#couple-stop-deadlines-to-actual-native-jobs-and-clock-ownership) or
[experiment receipts](#small-disposable-vm-experiment-card-and-receipts).

## Freeze the adapter choice before coding

Record exact package/version, registry checksum, packaged revision, features,
transitive Windows projection generations and target/toolchain. Compare published
package/reviewed source: dispatcher, status conversion, error propagation,
callback registration/release and panic paths. Use [dependency updates](../../../environment/dependency-updates/SKILL.md)
for graph admission/changes. Current main branches or similarly named packages
cannot replace comparison.

| Candidate from architecture 18.6 | Inspect and decide |
| --- | --- |
| Mullvad **`windows-service` 0.8.1**, singular; source `aab40570b50c05b8e6f3c375171727e666ee42a0` | Preferred prototype, still subject to CHECK-27. Its `set_service_status` returns an error; its callback wrapper frees the boxed handler after Stop, Shutdown or Preshutdown. Trace all release paths and permitted later activity before selecting it. [Pinned handler/status source](https://github.com/mullvad/windows-service-rs/blob/aab40570b50c05b8e6f3c375171727e666ee42a0/src/service_control_handler.rs). |
| Microsoft **`windows-services` 0.100.0**, plural; source `a57cc321bfdbac6bb98c488ba49f399da8fcb98a` | Held alternative. In this inspected revision `set_state` discards the `SetServiceStatus` result, detailed checkpoint/wait-hint control is absent, and `service_main` automatically reports Running after the start callback. Prove an upstream or adapter change resolves the required behavior before reconsidering it. This is an API-fit hold, not a vulnerability finding. [Pinned implementation](https://github.com/microsoft/windows-rs/blob/a57cc321bfdbac6bb98c488ba49f399da8fcb98a/crates/libs/services/src/lib.rs). |
| Small direct SCM adapter through the selected Windows bindings | Bounded fallback if the wrappers cannot meet the contract. Account for every added unsafe obligation and run the same qualification cases. Do not combine two dispatchers or retain both wrappers as production alternatives by accident. |

Separate candidate/hold decisions from adopted manifest/lockfile. Record API
mismatch, experiment result and narrow alternative. Unexecuted prototypes prove
no adopted/qualified production graph.

## Identify one lifecycle owner per own-process service

Draw the actual path: process entry → dispatcher → ServiceMain → registered
callback → bounded owner notification → serialized lifecycle owner → status sink,
worker admission and cleanup. Name who owns each transition, handle and generation.
Only that owner decides Running, Stopping and completion; callbacks and workers
cannot independently overwrite its state. The engine and broker retain separate
owners and processes. SCM Running describes service availability, not clock trust.

Trace one dispatcher call per process and prompt connection to SCM, with slow
service initialization in ServiceMain. Console/debug fallback is a separately
selected mode and cannot count as native SCM evidence. See Microsoft's
[dispatcher contract](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-startservicectrldispatcherw).

Before execution, define transitions, per-state accepted-control masks, service
exit codes, monotonic startup/stop deadlines and escalation owner. Mark undecided
budgets incomplete; library sample timeouts are no production defaults.

## Exercise status and control behavior

First inject failures/hold startup and cleanup at deterministic adapter barriers.
Then exercise selected executable under real SCM. Separate injected observations
from Windows results; seam failures prove no real OS failure path.

| Input/action | Observable output and pass condition |
| --- | --- |
| Healthy start; hold each real initialization stage | Handler registration precedes status use. StartPending has a zero accepted-control mask; Running follows successful readiness checks. Status snapshots and worker state agree. A healthy neighboring start succeeds after removing a fault. |
| Dispatcher/registration failure; failure after registration; failed startup status or Running report | Capture the defined native error before cleanup can overwrite it. No false successful-start acknowledgement, automatic Running transition or orphan worker survives. Cleanup and appropriate failure exit are attributable to the owner; if reporting itself fails, retain external diagnostics and an explicit failed outcome. |
| Lengthy startup/stop with one stage advancing, then a deliberately stuck stage | Checkpoint advances only with actual progress; wait hints describe that pending operation. Stable states clear pending progress fields. A heartbeat cannot conceal a stuck initializer or extend the declared deadline indefinitely. |
| Interrogate; unsupported control; each control actually enabled by the mask | Interrogate returns success without unnecessary state change; unsupported controls receive the documented result. Enabled stop/power/time/session paths reach the intended owner promptly and cannot wait behind network work. Do not enable pause, shutdown or preshutdown without a defined handler and recovery need. |
| Stop while Running; owner stop signal repeated during cleanup; late old-generation worker completion | Exactly one shutdown transition/effect sequence. StopPending remains honest while owned cleanup runs; repeated notifications do not double-free, restart work or renew a correction allowance. Old results cannot publish or actuate. |
| Orderly cleanup completes or fails durably | Complete required cleanup or durable failure reporting before the single final Stopped report. No later status call or application work relies on the status context; no claim that a failed status call reached SCM. |

Microsoft's [SetServiceStatus contract](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-setservicestatus)
defines failure reporting, real progress and the single terminal report; its
[SERVICE_STATUS fields](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/ns-winsvc-service_status)
define exit codes and pending-state bookkeeping. Check every returned status
result; an in-memory state assignment is not an SCM acknowledgement.

Choose late/repeated-control cases against the supported Windows delivery rules.
Microsoft documents no further SCM controls after Stop or Shutdown on current
Windows, and shutdown/preshutdown notifications require actual system delivery.
Use a disposable guest shutdown when that case is in scope; a synthetic call is
only adapter evidence. Test duplicate internal stop requests and late worker events
through a still-owned safe facade. Do not invoke a raw wrapper callback with an
already freed context to manufacture a scenario or infer a vulnerability. For
preshutdown and every enabled extended event, separately trace what can still be
delivered and when the callback context can be released. See
[HandlerEx controls, return values and responsiveness](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nc-winsvc-lphandler_function_ex).

## Prove callback and resource custody

Per callback record allocation owner, registration success/failure, event-data
validity, notification owner, last allowed invocation and release point. Instrument
entry/exit/context destruction with bounded IDs/counters. Cover pre-registration
errors, post-registration failure, normal stop, enabled shutdown and process death.
Inspect before fixture teardown; forced teardown cannot supply claimed cleanup.

The callback performs bounded decoding/copying and signals the existing owner;
it does not retain borrowed event pointers, wait for native jobs or acquire a lock
whose owner waits for the callback. Decode extended event data only for the matching
control/event type. Inspect the wrapper's entire extern callback and conversion
path, plus application callbacks, allocation/error handling and destructors: no
unwinding may cross the ABI. A chosen panic-to-failure/abort policy must be explicit;
`catch_unwind` cannot promise recovery from abort, foreign exceptions or invalid
pointers. Abort/process death requires an external recovery owner.

| Resource identity | Custody check |
| --- | --- |
| `SERVICE_STATUS_HANDLE` from handler registration | Status token, not a generic owned kernel handle. Do not put it in `OwnedHandle`/call `CloseHandle`; the registration contract requires no explicit close, and terminal status has its own lifetime. [Registration contract](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-registerservicectrlhandlerexw). |
| `SC_HANDLE` from SCM/service open or creation | Release once through `CloseServiceHandle`; closing the handle does not delete the service registration. [CloseServiceHandle](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-closeservicehandle). |
| Process/thread/token/event handles and callback/context allocations | Identify the exact creation API, ownership transfer and matching release API for each. Pseudo/borrowed handles and callback pointers are not generic owned handles. Check early-return and partial-start paths as well as normal destruction; never cast across incompatible Windows projection types to bypass a mismatch. |

## Qualify actual accounts and allowed/denied effects

Use [Windows privilege review](../../../security/security-windows-privileges/SKILL.md)
for CHECK-1 and [local IPC](../../../security/security-local-ipc/SKILL.md) for
S0.3/CHECK-2. A service wrapper establishes neither account isolation nor endpoint
authorization. Capture service configuration and each running process's actual
account SID, service/restricted SIDs, privilege presence/enabled state, object ACLs,
handle inheritance and process identity. Join observations to the executable hash
and process lifetime; a recycled PID is insufficient.

Use the intended two own-process fixtures: broker virtual account and engine
LocalService with restricted service SID and required-privilege allowlists. Verify
the engine lacks the architecture's dangerous privileges, including *present but
disabled* clock privilege. Unexpected rights are a failed configuration gate.
Stop before a nominally denied raw write if token inspection already violates that
absence; the test must not turn a known configuration failure into an unbounded effect.
Do not silently broaden to LocalSystem; record a failed proof and evaluate only
the architecture's constrained account fallback.

Pair each denial with a valid-shaped request and a healthy permitted operation:

| Fixture | Evidence to retain |
| --- | --- |
| Engine or ordinary user requests a protected clock effect | Actual access/privilege denial with unchanged clock adjustment and no success acknowledgement; the shape passes ordinary admission. Start with read-only observations, then the specifically authorized negative case. |
| Engine tries to obtain broker injection, writable-memory or token-duplication authority | Actual object-access results for the exact requested rights and current broker identity; no inherited or alternative handle defeats the denial. A denied UI control is insufficient. |
| Intended broker performs an allowed query/handle operation | Successful Windows result and correct cleanup under the actual service token, establishing a positive account/control fixture. |
| Intended broker's smallest authorized positive clock operation | Only under the bounded card below, with independent observation and recovery. A successful query does not qualify the omitted write/privilege subcase. |

S0.3 adds real pipe status/protected methods, fake servers and PID-reuse/restart
cases at IPC owner. Mark unexecuted; SCM startup proves no bidirectional endpoint
identity.

## Couple stop deadlines to actual native jobs and clock ownership

Freeze section 17.1 experimental budget: initially at most two submitted native
jobs including worker waiters, at most one acquisition per endpoint, no extra
application backlog. Label unmeasured P0 starting budget. Count hidden DNS,
certificate/provider/delegated work; measure submitted/running jobs, bytes,
threads/handles against declared admission graph.

Hold a native job behind a barrier, cancel/expire its request, invalidate its
generation, and request service stop. Its permit stays with the actual job until
return; timeout or dropping a future does not create replacement capacity. Confirm
control handling remains responsive and saturated work yields bounded busy/degraded
behavior. Release it late and verify no model update, fresh status or broker effect.

For an indefinitely stuck call, measure the finite stop deadline and the chosen
SCM-coordinated engine process stop/restart/backoff path. Do not use `TerminateThread`
or escape limits through replacement pools/runtimes. A shutdown timeout proves only
that waiting ended. Record what happens to the actual job and process. On restart,
old generations and acknowledgements remain invalid and cumulative correction
budgets remain consumed. Include one healthy request once capacity/fault recovers.

Broker cleanup is independent of engine survival. Observe adjustment/journal state
before death and on recovery. Before any reset/write, re-establish current ownership;
a saved rate or journal alone is insufficient. A successor/ambiguous controller
requires read-only Blocked recovery, including during test restoration. Measure
recovery delay and residual effect; do not claim a finite lease or destructor cleanup
after broker death. Leave W32Time restoration to its existing protected transaction
owner; no generic test teardown may overwrite later administrator/GPO/successor state.

## Small disposable-VM experiment card and receipts

Prepare card before S0.2 clock-writing subcase or S0.4 writes; retain same
identity/budgets for associated service lifecycle cases:

```yaml
experiment: S0.2 / selected CHECK-1 and CHECK-27 cases
candidate: exact source, manifest/lock/features, executable hashes
guest: disposable VM identity, Windows build, checkpoint and host time integration
scope: read-only first; exact permitted service/config/clock mutations
ownership: current clock controller, product generation, W32Time/policy baseline
recovery: independent access/operator and recovery procedure after process death
limits: exact raw APIs, phase/rate bounds, native jobs, duration, attempt count
stop: finite deadline, triggers, escalation owner and restart/backoff limit
restoration: prior owned state, successor check, verification and unresolved outcome
receipts: case IDs, raw output paths and external observations
```

Unset write bounds mean no write. Begin with baseline/read-only observations and
denied unprivileged calls. Admit only the smallest independently checked positive
operation needed to answer the question. Temporary laboratory bounds do not become
product defaults; those require S0.7 decisions and measurements. Service registration,
account rights, shutdown or kill experiments also need their scoped disposable
machine/recovery conditions; this card does not expand task permission.

Store future raw receipts under `evidence/P0/`: actual command/arguments, exit code,
stdout/stderr, source/package/executable hashes, VM/token/service configuration,
SCM/event-log observations, monotonic timing, callback/context counters, native job
admission/completion and before/after clock/journal/ownership observations. Preserve
raw errors separately from interpretation, injection receipts separately from native
results, and pre-teardown observations separately from restoration. Record missing
outputs, substitutions and failed cleanup; a summary cannot replace lost raw proof.

[Environment preflight](../../../testing/qa-environment-preflight/SKILL.md) owns
machine/fixture readiness; [state/concurrency/lifecycle](../../../testing/qa-state-concurrency-lifecycle/SKILL.md)
owns timing, generations and custody; [fault recovery](../../../testing/qa-fault-recovery/SKILL.md)
owns death/restart and restoration cases. Use
[Windows evidence](../../../testing/references/windows-evidence.md) for native evidence
layers and [final qualification](../../../testing/qa-final-qualification/SKILL.md)
to assess the exact candidate against the selected CHECK subset.

Report **pass** only for declared cases with native observations, healthy controls
and bounded recovery. Missing Cargo/artifact/Windows proof, undecided budgets or
omitted allowed effects mean **incomplete**. **Stop the dependent choice** for
unexplained lifetime/release failure, false Running/Stopped success, unexpected
rights, unbounded stop or ambiguous recovery ownership. Preserve receipts; review
one bounded alternative. Unrelated experiments may continue; no broader privilege,
dependency or host-mutation authority follows.
