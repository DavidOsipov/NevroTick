---
name: "implement-windows-service"
description: "Create, harden, diagnose or change NevroTick Windows services: SCM startup/status, identities, management admission, shutdown/recovery, service logging and measured resource costs. Use for service lifecycle or installation work; ordinary Rust logic and unrelated Windows adapters retain their own workflows."
---

# Implement and manage Windows services

Read the [implementation contract](../CONTRACT.md), then the affected parts of
[architecture §§4, 11, 12, 15 and 17](../../../../docs/windows-time-architecture.md).
Reuse [W11-W05](../../../../tasks/windows11-integration.md#w11-w05-observe-effective-scm-maintenance-and-process-policy),
S0.2/S0.3, P3 or the existing lifecycle owner. W11-W10/P6 owns the complete
package; W11-W08 and ST-W07 own reached diagnostic/privacy cases. Bind the work
to the applicable CHECK IDs, exact service/actor/verb and independent expected
effect before implementation or execution.
Match the requested planning, read-only diagnosis, implementation or management
mode. A small known query needs only its relevant operation and unchanged evidence,
not a new lifecycle qualification campaign.

Preserve separate own-process engine and clock-broker owners, an unprivileged
network-facing engine, normal-user UI and narrowly elevated lifecycle helper.
OD-05 selects a complete hardened MSIX; its supported service-account/provisioning
mapping remains unqualified. Do not select LocalSystem, a wrapper, watchdog,
dependency or extra installer to fill that gap. OD-07 requires x86-64-v3 and the
strongest applicable compatible protections; configured flags are not native proof.
Formal execution and proof-tool repairs remain post-V1 under OD-10. Keep the
current Windows/native stop, separate fuzz hold, optional-storage pause and
UI-M0 deferrals. Source work can continue within its admitted scope.

## Select the affected operation

The [service adapter guide](../implement-rust/references/windows-service-adapters.md)
is the maintained detailed procedure. Load its relevant sections and only the
additional owner needed for the changed guarantee:

| Work | Read and apply |
| --- | --- |
| Create a service adapter or change wrapper/bindings | Guide: adapter choice, lifecycle owner and callback custody; [dependency admission](../../environment/dependency-updates/SKILL.md) for graph changes. |
| Install, provision, repair, update, remove or reset | Guide: service configuration/recovery and stop/recovery; [MSIX provisioning/removal](../../../../docs/plans/windows-msix-store.md), architecture §11 and W11-W10. Retained MSI mechanics apply only under a later delivery decision. |
| Query, start, stop, reconfigure or expose a product management command | Guide: object/operation access matrix; [Windows privilege security](../../security/security-windows-privileges/SKILL.md) and [local IPC](../../security/security-local-ipc/SKILL.md). |
| Author or inspect service labels, resources, version/help or event metadata | [Maintain Windows metadata](../maintain-windows-metadata/SKILL.md) and [canonical service field contract](../../../../docs/plans/windows-package-metadata.md#12-service-and-diagnostics-metadata-refinement--7-november-2026); W11-W10/P6 owns projections, W11-W05 configuration effects and W11-W08 diagnostic semantics. |
| Diagnose errors, change events, exports or crash collection | [Error handling](../implement-rust/references/error-handling.md), [privacy/logging](../../security/security-privacy-logging/SKILL.md), architecture §12.3 and guide: service configuration/status/callbacks. |
| Reduce startup, CPU, wakeups, memory, handles or stop latency | Architecture §17 and [performance optimization](../optimize-rust-performance/SKILL.md); guide: actual native jobs and stop deadlines. |
| Qualify a native lifecycle, account or recovery claim | Guide: disposable-VM card; [testing contract](../../testing/CONTRACT.md), then the affected security/lifecycle/fault-recovery or package QA owner. |

## Implement the owned lifecycle

1. **Freeze the actual boundary.** Inspect current source, manifests, selected
   target/features, wrapper revision, package profile, task holds and existing
   evidence. Distinguish source-only preparation, host model, Windows cross-build,
   installed native behavior and final package/Store bytes. An ordinary service
   request does not authorize host service replacement, clock changes or a native
   probe. Select the existing admitted disposable-machine scope before mutation.

2. **Specify creation and admission.** Record the deployment/provisioning actor,
   immutable image and arguments, actual account/token/service SID, required
   privileges, dependencies, object security, per-operation requested rights,
   startup mode and effective mitigation activation. Qualify creation-time
   protection separately from later DACL/token defaults; existing handles can
   retain rights after tightening. Keep process CLI, transient SCM start arguments
   and protected durable policy distinct. Bind callback decoding to the selected
   wrapper/FFI representation; admit or explicitly ignore/reject additional
   options. Neither argument channel may replace role, protected roots, endpoints,
   maintenance inhibition or clock authority. Recovery cannot depend on a vanished
   manual-start option. Inventory actual environment/profile/path inputs, including
   dependency overrides; a temp-path string proves neither safe storage nor access.
   Keep stable technical service identity separate from localized display/purpose
   text, configured policy and runtime status. Metadata completeness does not
   authorize extra privileges, triggers, accounts or recovery behavior. Inspect
   actual installed resource resolution and servicing through the metadata owner;
   a source string, schema field or successful setter is not installed readback.

   Use [architecture §4.1.1's exact service privilege profiles](../../../../docs/windows-time-architecture.md#411-service-privilege-profiles)
   for both runtime services. Configuration needs explicit nonempty names;
   admission needs full present-set observation, not only enabled privileges or
   a familiar forbidden subset. Route the current laboratory observer's missing
   full-profile behavior through
   [W11 §4.38](../../../../tasks/windows11-integration.md#438-service-privilege-profile-handoff--9-november-2026).
   Preserve startup activation and supported package provisioning; do not add
   token/SCM rights or retain a powerful duplicate merely to make a wrapper,
   child-launch, memory or diagnostic convenience succeed. UI/helper tokens
   retain their distinct role/operation contracts.

3. **Give transitions one owner.** Trace process entry → dispatcher → ServiceMain
   → callback notification → serialized lifecycle owner → checked status report,
   worker admission and cleanup. Connect promptly to SCM; keep slow initialization
   out of process entry. Freeze controls by state, truthful checkpoints/wait hints,
   exit codes and finite monotonic deadlines. Running means the declared service
   is available, not authenticated time or permission to actuate. Check status API
   failures; finish owned cleanup/durable failure reporting before terminal Stopped.
   Callbacks copy only bounded valid event data, notify the owner and return;
   preserve callback/context lifetime and prevent unwinding across FFI. Logical
   serialization does not establish Win32 mutex native-thread ownership.

4. **Separate management from effect authority.** Apply
   [architecture §4.2.1](../../../../docs/windows-time-architecture.md#421-service-management-admission-and-administrative-recovery):
   SCM authorizes principals; custom controls supply no authenticated original
   application identity. Use qualified product IPC and current actor/component,
   generation and effect guards for protected commands. Preserve explicit elevated
   recovery without a healthy UI/service. Record the existing coordination object's
   namespace, creator, owner/DACL, minimum rights, lifetime and recovery owner.
   Distinguish new, trusted existing, collision/denied and abandoned outcomes.
   A fresh mutex after a crash can lack an abandoned indication; reconcile journal,
   generation and current clock ownership before effects in every case. No foreign
   object adoption, unprotected alternate controller or renewed correction budget.

5. **Make failure bounded and diagnosable.** Capture native errors at the failing
   API; retain typed domain errors and project permitted fields through shared
   diagnostics before formatting or queuing. Log the useful outcome once at its
   decision owner; maintain independent required-event routing, finite queues,
   retention and bounded fallback if Event Log/files/ETW fail. Logging cannot gate
   safe cleanup. Freeze per-role effective process/thread error policy and its
   establishment phase; critical-error handling, WER invocation, UI, submission,
   dumps and collector privacy are separate controls. Avoid competing process-wide
   setters and preserve approved diagnostics/administrator policy. Keep engine and
   broker recovery records separate: finite action/reset behavior, terminal None,
   actual queued restart semantics and protected maintenance inhibition. Timeout or
   dropping a future does not complete native work or free its permit. Never use
   replacement pools, arbitrary recovery scripts or a second supervisor to evade
   limits. Broker death requires independent recovery, not assumed destructor or
   rate expiry; preserve successor ownership and uncertain outcomes.

6. **Measure and qualify the changed behavior.** For optimization, attribute the
   observed useful-work cost and compare bounded before/after runs on the same
   profile; retain hostile, saturated, cold-start and shutdown behavior when
   relevant. Keep admission, mitigation, logging and correction guarantees intact;
   do not raise global timer resolution or weaken protection as incidental tuning.
   If startup reaches policy/installer/provider waits, inspect their actual wait
   graph and challenge reachable cycles; do not add policy locking or report
   Running early merely to copy another service. For regressions use the real
   changed entry, independent invalid/fault case and valid neighbor. Select the
   configured compiler/lint/tests and required coverage/mutation gates; native
   rights, process completion, no-modal-wait and installed lifecycle claims need
   actual Windows observations under the existing authorized card.

## Close the existing task

Record the exact candidate and installed profile where applicable, operations and
controls exercised, actual outputs/effects, retained native jobs/handles, cleanup
and restoration outcome. Keep configured versus effective policy and injected
versus native failures distinct. Missing budgets, unqualified account/provisioning,
failed/omitted native cases or ambiguous ownership stay incomplete with their
current owner and next action. A source review, wrapper example, green host test
or successful StartService call is no service-readiness or release pass.
