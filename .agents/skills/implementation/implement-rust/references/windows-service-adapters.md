# Qualify Windows service and native-work adapters

Use for engine/broker SCM adapters, callback lifetimes, service accounts or
stop/restart implementation/review. First read [canonical architecture](../../../../../docs/windows-time-architecture.md)
sections 4, 9–11, 17.1, 18.6, CHECK-27 and S0.2–S0.3. Procedure adopts no dependency
or production limits.

The dedicated [Windows service skill](../../implement-windows-service/SKILL.md)
selects creation, management, hardening, diagnostics and optimization work. This
reference remains the detailed SCM/native procedure; load only its relevant parts.

Source/design preparation does not establish a service/package candidate. Source
review may identify API fit/blockers; Windows runtime needs actual evidence. Do not create runtime
projects, install tools, register services or change clocks merely for checklist
completion. Execute cases
only within task-authorized machine/mutation scope.

The [current OD-05 direction](../../../../../docs/nevrotick-owner-decisions.md#7-od-05--signing-and-release-ownership)
is a complete hardened MSIX for David's Store trial with services. Read
[plan §3](../../../../../docs/plans/windows-msix-store.md#3-package-trust-and-the-privileged-helper)
for package/CIG/per-role controls and
[§7](../../../../../docs/plans/windows-msix-store.md#7-bounded-continuation-and-existing-acceptance-owners)
for local/test-signed mechanics, owner submission and subsequent actual Store-byte
qualification. Prior Store acceptance is not a prerequisite for source/design
work or candidate preparation. The supported MSIX service-account/provisioning mapping remains
open; retain own-process separation and effective least privilege. Earlier MSI
mechanics are fallback/reference only, not a prerequisite or automatic parallel
target. Current Core priority, Windows stop, separate fuzz hold and storage pause
remain.

Navigate to [adapter choice](#freeze-the-adapter-choice-before-coding),
[lifecycle ownership](#identify-one-lifecycle-owner-per-own-process-service),
[service configuration/recovery](#freeze-service-configuration-and-recovery-behavior),
[status/control cases](#exercise-status-and-control-behavior),
[callback custody](#prove-callback-and-resource-custody),
[object access](#freeze-the-object-and-operation-access-matrix),
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

Preserve raw `dwServiceType`, including unknown bits, beside the wrapper's
interpreted type. The pinned `windows-service` conversion uses
`from_bits_truncate` and its enum omits the packaged-service `0x200` bit; a
wrapper-only receipt can discard that distinction. Use the
[source basis in MSIX plan §7.5](../../../../../docs/plans/windows-msix-store.md#75-pre-start-provisioning-authority-and-admission--5-november-2026)
and the selected bindings to interpret raw evidence; this observation selects no
different wrapper or dispatcher and proves no actual installed type.

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

### Separate startup inputs and reachable waits

Record deployment process arguments, SCM per-start arguments and protected
durable policy separately under architecture §12.2. Inspect the selected
wrapper/FFI's actual callback bounds, encoding and service-name metadata; never
index an assumed nonempty vector. Extra options need an explicit unsupported/
ignored, rejected or bounded admitted grammar. Do not forward the callback vector
to a console/debug parser or let either channel override protected role, endpoint,
state root, maintenance inhibition or clock ownership. Pair an installed ordinary
start with conflicting/excessive/malformed options and a later ordinary recovery
without transient parameters. Preserve legitimate administrative start/recovery.

Inventory actual environment/profile/dependency inputs influencing configuration,
trust, storage or diagnostics, including missing/redirected path outcomes under
the real account before login. An OS temp-path result verifies no existence,
access, symlink safety or private live-spill custody. Reuse
[SEC-A02](../../../../../tasks/server-catalogue-and-statistics.md#sec-a02--file-backed-temporaries-need-concrete-location-and-peak-accounting-coverage),
not a new storage policy; do not assume profiles are absent, erase all environment
variables or change to SYSTEM for convenience. Identify an actual writable input
before asserting attacker control.

For reached policy locks, installer/SCM/native-provider or dependency waits,
inspect the complete wait graph and challenge reachable cycles beside a healthy
ordering. Preserve finite progress/failure and actual cancellation custody. The
[pinned Tailscale policy-lock example](../../../../../docs/plans/windows11-integration-follow-up.md#additional-service-practices--7-november-2026)
does not establish the same MSIX cycle here or justify reporting Running before
availability. A genuinely available read-only Blocked control plane still grants
no clock effect while required policy is unknown.

### Establish effective unattended error behavior

Record each role's effective process/thread error policy, establishment owner and
phase in the existing startup manifest. Account for inherited, runtime and
dependency behavior before adding setters. Keep critical-error dialog handling,
WER invocation, WER UI, queue/upload choices and collector privacy distinct.
The `SetErrorMode` page describes `SEM_NOGPFAULTERRORBOX` as suppressing WER
invocation; do not use it merely as a no-UI switch. The thread API page uses
different dialog wording; qualify the selected API rather than extending one
page's semantics to every process/thread path.
Do not disable approved reporting by default or copy unexplained numeric flags.
Use [reviewed SDK/API distinctions](../../../../../docs/plans/windows11-integration-follow-up.md#additional-service-practices--7-november-2026).

Avoid competing process-wide setters. Thread-local modes can suit a scoped native
operation, but need actual OS-thread coverage and restoration; an async future
may migrate and hidden native workers may use another mode. Qualify service
profiles separately from UI/helper or a shared installer host. Pair ordinary
startup/useful diagnostics with a selected critical-error fault during startup,
work and stop: no modal wait, truthful failure, bounded cleanup and useful safe
output. Query/setter success alone proves no such behavior. W11-W08/ST-W07 retain
WER/LocalDumps/debugger synthetic-secret and retention cases; no UI is neither no
dump nor no upload, and no global administrator policy change is authorized.

Keep event provider/channel resources with the selected package/lifecycle owner;
runtime logging does not gain installation privilege. If a classic Event Log
adapter is selected, [RegisterEventSourceW](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-registereventsourcew)
can succeed for an unregistered source by falling back to Application without
useful message descriptions. That return value is not provider installation or
delivered-event qualification. Inspect real channel/provider identity, event
rendering and unavailable/full-sink behavior under the selected API; the classic
fallback is not a modern manifest-provider contract or a selected sink.

## Freeze service configuration and recovery behavior

For descriptive/resource work, use [maintain-windows-metadata](../../maintain-windows-metadata/SKILL.md)
and the [canonical service field contract](../../../../../docs/plans/windows-package-metadata.md#12-service-and-diagnostics-metadata-refinement--7-november-2026).
Keep stable technical service name, localized display name, purpose description,
next-run configuration and current status distinct. SCM indirect MUI references
and package `ms-resource:` references need separate stored/resolved observations.
Description null means unchanged; empty deletes. Read configuration/status with
the appropriate query rights; a metadata request grants no configuration rights.
The package Service schemas lack direct display/description attributes while
the conversion report exposes those values: qualify the actual selected pipeline
and installed update/rollback resource resolution before claiming propagation.
The detailed catalogue is an applicability inventory, not a request to configure
all fields. When no required-privilege list is configured SCM uses token defaults;
omitting an update preserves the existing list. Existing W11-W05/W11-W10 owners
retain privilege/provisioning authority and actual native qualification.


The elevated installer/helper owns service registration/configuration and bounded
maintenance under architecture section 11. Runtime lifecycle owners report status
and handle admitted controls; they do not grant themselves service configuration
rights. Keep one reviewed configuration record **per service**, bound to the
product generation and executable identity. Populate this matrix before native
qualification; unresolved counts/durations/flag choices remain P0 decisions.

For MSIX, first establish actual per-operation rights on the deployment-created
objects under the proposed provisioning actor. Generic CreateService defaults,
elevation or LocalSystem identity prove no such grant. Follow
[plan §7.5's handle-only first probe](../../../../../docs/plans/windows-msix-store.md#75-pre-start-provisioning-authority-and-admission--5-november-2026)
before setters; request descriptor rights only when the operation needs them,
and make no start/stop call during that probe. Supported no-UI invocation and
servicing reentry are separate prerequisites. Later disabled/quiescent
configuration and activation precede controlled authenticated management start,
then fresh process admission before normal work. Creation-time protection already
applies, and existing recovery authority must survive a normal-admission refusal.

| Configuration or outcome | Engine record | Broker record |
| --- | --- | --- |
| Registration and startup | Fixed deployment-owned identity/image/arguments, own-process type, supported account and restricted service SID, required-privilege allowlist, start mode, dependencies and readiness conditions; prove removal of dangerous rights before exposure | Separate own-process identity and minimum required privileges through a supported MSIX provisioning method; exact account mapping remains unqualified. Readiness includes ownership assessment and recovery reconciliation. Unavailable/ambiguous ownership permits only defined read-only Blocked behavior, never a clock write; SCM Running still describes service availability |
| Recovery sequence | Exact ordered action/delay pairs, finite restart budget, terminal behavior and quiet-period reset policy | Separate action/delay pairs and budget justified by the measured retained-rate recovery envelope; do not copy engine values without proof |
| Failure classification | Explicit `fFailureActionsOnNonCrashFailures`, meaningful Win32/service-specific exit-code mapping, configuration versus effective activation evidence | Same fields; distinguish initialization failure, read-only Blocked availability, ordinary request rejection and fatal failed impersonation reversion. A rejected request is not a process fault; failed reversion follows section 5.4's fatal path, and no successful exit is fabricated to conceal failure |
| Stop and escalation | Accepted controls by state, monotonic deadlines, native-job custody, SCM-coordinated process recovery and backoff owner | Independent cleanup deadline, attributable clock-state handling, external failure diagnostics and recovery owner |
| Maintenance and exhaustion | Quiescence/disable/restore procedure; exhausted recovery is visible and does not create replacement runtimes | Same lifecycle ownership; exhausted recovery also reports unresolved adjustment cleanup, with no automatic takeover by another clock controller |

Microsoft's [failure-actions contract](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/ns-winsvc-service_failure_actionsw)
repeats the **last action** after the list is exhausted. End a finite restart
sequence with `SC_ACTION_NONE`; a short list ending in Restart is unbounded.
`dwResetPeriod` is seconds without a failure before the counter resets, not a
stop timeout; `INFINITE` prevents that reset. The
[action delay](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/ns-winsvc-sc_action)
uses milliseconds. Record reset/boot behavior as well as the action count: a reset
can reopen the sequence. Do not add automatic host reboot or arbitrary recovery
commands to the bounded restart design; those introduce separate authority and
system effects requiring a new decision.

With configured actions, the [non-crash failure flag](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/ns-winsvc-service_failure_actions_flag)
controls whether reporting Stopped with nonzero `dwWin32ExitCode` also triggers
recovery; termination without Stopped already qualifies. It is not a per-exit-code
recovery policy. Microsoft documents flag changes taking effect at the next system
start. Query configured values and separately qualify actual activation, including
reboot where required; successful setters/readback prove no current behavior.

An [already queued SCM restart cannot be canceled](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-changeserviceconfig2w).
A later orderly stop or changed recovery list does not establish quiescence.
For maintenance that must keep a service down, the authorized installer/helper
must capture the owned configuration, explicitly disable the affected service to
prevent the queued restart, then establish stopped/process-quiescent state within
the existing transaction. Keep that protection through replacement and qualify
re-enablement against delayed actions. Disabling prevents restart; it does not
prove queue deletion. Restore only the authorized intended configuration, with
product-generation and later administrator/GPO/successor checks; never blindly
restore Automatic or modify unrelated services.

Keep maintenance inhibition and captured configuration in the protected durable
lifecycle transaction. Startup admission must honor incomplete maintenance before
admitting workers or clock effects, including after interruption/reboot. This
admission guard supplements service disabling; it does not cancel a queued SCM
action or prove process quiescence. Pair completion/rollback with the authorized
configuration restoration and qualify a queued action firing at each boundary.

| Native scenario | Required observation |
| --- | --- |
| Unexpected engine or broker termination | Correct configured delayed action and new process generation; no old results or renewed correction allowances. Broker re-establishes ownership before any write/reset. |
| Controlled startup/runtime failure reporting Stopped with error | Actual behavior matches the effective flag and truthful exit-code policy; compare a healthy start and successful intentional stop. |
| Repeated failures through exhaustion and after the declared reset interval | Measured attempt count/delays, terminal None, reset behavior and external diagnosed outcome; no secondary restart mechanism defeating the bound. |
| Delayed restart queued before update/repair/rollback/uninstall | Maintenance disable/quiescence prevents unexpected restart; interrupted maintenance and re-enablement preserve ownership and authorized configuration. |
| Broker fails throughout the recovery envelope | Retained adjustment is measured/reported under section 10.4; terminal None is not proof of clock cleanup or a finite lease. |

Retain queried configuration, effective-state observations, boot/product/process
identities, monotonic event times and external SCM diagnostics with each result.
Never change the actual host merely to fill this matrix; use the admitted
disposable fixture and bounded recovery card below.

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

## Freeze the object and operation access matrix

Before selecting ACLs or wrapper defaults, record each real operation as:
**actor/effective token → exact object → API and minimum requested access →
allowed effect → denied neighboring effect → native evidence**. Include object
owner, DACL/inheritance, service/restricted/logon SIDs, handle lifetime and any
impersonation state. Distinguish access requested, access granted and a successful
effect; source constants or a copied SDDL string prove none of them.

For the existing product coordination primitive, apply
[architecture §11.1](../../../../../docs/windows-time-architecture.md#product-coordination-and-crash-recovery).
Freeze representation, namespace, creator, owner/DACL, minimum actual API rights,
native owner and recovery/lifetime. `CreateMutexW` opens an existing mutex with
`MUTEX_ALL_ACCESS`, ignoring the proposed new descriptor/initial ownership;
inspect an appropriate rights-selecting API rather than widening ACLs to fit it.
`Global\` supplies scope, not security; it does not by itself require the
privilege documented for global file-mapping/symbolic-link creation. A private
namespace is unselected and boundary matching alone would not secure it.

Pair trusted first/opened ownership with foreign precreation, wrong-type
collision, access denial and intended cross-session sharing. Refusal cannot by
itself prove reservation availability. Exercise abandoned ownership while a
legitimate handle keeps the object alive, and separately a crash that destroys
the last handle before recreation. Both paths must reconcile journal, generation,
consumed allowance and current controller before effects. Preserve owning-thread
release, recursive acquisition accounting and separate logical-task serialization;
test cross-thread release/reentry through the selected safe adapter contract.
Never treat a successful wait as clock authorization or a budget refund.

Apply [management admission and administrative recovery](../../../../../docs/windows-time-architecture.md#421-service-management-admission-and-administrative-recovery):
Record normal route, required installed component and deliberate elevated
exception per actor/verb. SCM checks principal rights; HandlerEx supplies
no authenticated original-caller application credential. Protected product
commands require qualified IPC plus current actor/component/effect guards.
Preserve public status/refresh policy and independently usable recovery.

Establish protection before exposing authority. DACL tightening does not prove
revocation of existing handles; Qualify pre-existing handles across repair/update.
Holding a process handle after lookup does not prove race-free acquisition or
prevent delegation. Inspect staged/moved/replaced files: final owner/DACL and parent authority; moving into a protected directory is insufficient. Use supported
SCM/package configuration APIs, not direct Services-registry edits. Local-only
pipes do not settle remote SCM policy. The [source review](../../../../../docs/plans/windows11-integration.md#service-management-hardening--7-november-2026)
owns API distinctions. Exact descriptors, peer binding and exceptions remain
native qualification outputs; no universal SDDL.

For runtime-created objects, distinguish a service SID in the token/DACL from the
token's **default owner**. A service SID does not automatically become object
owner, and an owner normally receives implicit `WRITE_DAC`. A narrow allow-ACE
list alone therefore does not qualify separation from another service sharing
LocalService. Inspect the actual primary/impersonation token and restricted SID
access semantics; do not claim every LocalService token can bypass every DACL.
[Owner/default-owner contract](https://learn.microsoft.com/en-us/windows/win32/secauthz/owner-of-a-new-object).

TokenOwner/defaults govern relevant later creation; they do not secure the token
object itself or retrospectively protect the initial process/thread and earlier
startup objects. Verify eligible SIDs and the actual primary/impersonation token
for each creation path. Keep creation-time isolation and later runtime/default-
owner policy as separate native obligations: late safe readback cannot exclude
an earlier dangerous handle. The [MSIX refinement](../../../../../docs/plans/windows-msix-store.md#75-pre-start-provisioning-authority-and-admission--5-november-2026)
owns that distinction and its evidence limits; no TokenOwner implementation or
new authority is selected here.

Before freezing storage/process security, qualify a concrete owner-isolation
mechanism: a valid per-service default owner, an appropriate `OWNER RIGHTS` ACE
policy, or a separately qualified distinct-account design supported by the
selected package. The retained virtual-account reference is not an admitted MSIX
mapping. The `OWNER RIGHTS` policy controls implicit owner permissions; its presence and inheritance must
be inspected on each relevant object rather than inferred from a parent.
[Microsoft OWNER RIGHTS definition](https://learn.microsoft.com/en-us/windows-server/identity/ad-ds/manage/understand-security-identifiers).
Cover initial creation, staging, log rotation and atomic replacement of secret,
journal and diagnostic files, plus relevant process/token/kernel objects. Avoid
an externally accessible creation-to-security-fixup interval. An ACL inherited
by one file or an installer-created original is no proof about a replacement.

For CHECK-1/15, use a second service under the shared account to attempt
owner-mediated `WRITE_DAC`/data access after runtime creation and replacement.
Retain actual owner/DACL/inheritance, effective tokens, requested/granted rights
and effects. Pair denial with the intended service's successful create/read/
replace path. DPAPI under a shared account supplies no assumed isolation from
that account. Exact security descriptors remain native proof outputs.

| Object boundary | Required per-operation entry and separation |
| --- | --- |
| SCM manager | Separate manager rights from service-object rights. Justify `SC_MANAGER_CONNECT` for opening a known service; creation/database mutation belongs to the elevated lifecycle owner, not UI/engine/broker runtime. No default `SC_MANAGER_ALL_ACCESS`. |
| Engine/broker service objects | Request only the query rights actually used for service/server identity, such as `SERVICE_QUERY_STATUS` or `SERVICE_QUERY_CONFIG`. Installer/helper start/stop/configure/delete/security changes get separate exact masks and fixed service targets; no general runtime `SERVICE_CHANGE_CONFIG`, `WRITE_DAC`, `WRITE_OWNER` or deletion authority. No default `SERVICE_ALL_ACCESS`. |
| Process and thread objects | Preserve the limited query/synchronization handles required by the actual stable-identity proof. Separately deny engine-to-broker injection, writable memory and unneeded termination/suspension/handle-duplication rights; no inherited handle or alternate opening path may bypass the boundary. |
| Token objects and privileges | Identify token query versus adjustment/duplication/impersonation access. Inspect actual primary and relevant thread tokens, required-privilege allowlists and present/enabled privileges; engine cannot acquire broker authority through a token handle. |
| Named pipes | Record each endpoint's creator, instance rights, caller token and verb authority. Preserve local-only admission, authenticated server identity, bounded messages and current per-method authorization under architecture section 5.4; do not broaden file/pipe generic masks for wrapper convenience. |
| Files, directories and registry keys | Separate immutable installed code, broker policy/journal, engine secrets, diagnostic sinks and per-user UI data by their existing writers. Record exact read/write/create/delete/security rights, parent inheritance and protected paths; standard users/engine cannot replace broker code, policy or trusted identity material. |

The [SCM/service access contract](https://learn.microsoft.com/en-us/windows/win32/services/service-security-and-access-rights)
defines distinct rights and warns about service mutation grants. Its default
service DACL includes user-defined-control access for local authenticated users;
do not assume defaults authorize only administrators. Accepted-control masks
describe supported delivery, not caller authorization. No custom control may
bypass the broker's protected-method policy. Ordinary UI status uses product IPC;
retain only the SCM/process query access actually required for server verification,
instead of either denying all queries or granting lifecycle control.

For restart recovery configuration, [ChangeServiceConfig2W](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-changeserviceconfig2w)
requires `SERVICE_CHANGE_CONFIG` and, when configuring Restart, `SERVICE_START`.
Keep those rights with the installer/helper. Exact ACLs/SDDL and wrapper access
requests remain S0.2/S0.3 proof outputs, not newly adopted universal masks.
Qualify the matrix under actual engine, broker, standard-user, filtered-admin and
elevated-helper tokens; pair denied access with legitimate query/control cases.
Unexpected dangerous grants fail startup/installation qualification, rather than
being repaired by adding privileges. Use
[Windows privilege review](../../../security/security-windows-privileges/SKILL.md)
and [native proof](../../../security/references/native-proof.md) for the evidence
and residual administrator/host limits.

## Qualify actual accounts and allowed/denied effects

Use [Windows privilege review](../../../security/security-windows-privileges/SKILL.md)
for CHECK-1 and [local IPC](../../../security/security-local-ipc/SKILL.md) for
S0.3/CHECK-2. A service wrapper establishes neither account isolation nor endpoint
authorization. Capture service configuration and each running process's actual
account SID, service/restricted SIDs, privilege presence/enabled state, object ACLs,
handle inheritance and process identity. Join observations to the executable hash
and process lifetime; a recycled PID is insufficient.

Use two own-process fixtures with the exact proposed package-supported identities,
service SIDs and required-privilege allowlists. The retained MSI design uses a
broker virtual account and LocalService engine; that account mapping and a
two-LocalService alternative both need supported pre-start provisioning and native
proof for MSIX. Do not invent a manifest account or assume the service may grant
itself configuration rights. Verify
the engine lacks the architecture's dangerous privileges, including *present but
disabled* clock privilege. Unexpected rights are a failed configuration gate.
Stop before a nominally denied raw write if token inspection already violates that
absence; the test must not turn a known configuration failure into an unbounded effect.
Do not silently broaden to LocalSystem; record a failed proof and evaluate only
an architecture-compatible account/provisioning design. Unsupported isolation or
recovery is a concrete blocker to return to the owner, not permission to add an
external MSI prerequisite or mandatory first-UI setup.

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

For independent removal/Reset after service death, apply
[architecture §11.2](../../../../../docs/windows-time-architecture.md#112-upgrade-rollback-and-uninstall)
and [MSIX plan §7.6](../../../../../docs/plans/windows-msix-store.md#76-independent-removal-with-outstanding-recovery--5-november-2026).
Prove usable code, protected data, current authority and independent invocation
together; task/SCM/clock and cross-user observation rights are distinct. Package
retention plus a task remains unselected. Start the inert removal pair with a
durable obligation still pending, then observe fresh activation and actual effect
beside clean-STOP/successor controls. Record observer retention/launch effects
without supplying recovery; preserve required CHECK-27/native-memory instrumentation
and its control. Keep completed restoration, a retained usable path and authorized
no-restoration distinct in the existing task receipt.

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
