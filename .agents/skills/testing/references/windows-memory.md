# Native Windows memory and resource diagnostics

Use for a concrete Windows heap, handle, FFI ownership or resource-lifecycle gap.
[TOOLING.md](../../../../TOOLING.md#native-windows-memory-diagnostics) owns the
installed versions, admitted commands and actual control results. Read the
[testing contract](../CONTRACT.md) and [Windows evidence](windows-evidence.md).
Installation, an enabled profile and a clean debugger exit establish different
facts; none alone qualifies an application.

## Select the observation

| Question | Evidence to select |
| --- | --- |
| Does the reached native code misuse a Windows heap, handle or supported synchronization API? | Application Verifier checks on the actual Windows executable and selected API paths, with debugger stop/log capture. |
| Does a reached allocation overrun its protected Windows heap boundary? | Full PageHeap on that image, with a matching faulty control and useful healthy neighbor. |
| Does a repeated lifecycle retain allocations, handles, threads, tasks or permits? | Bounded lifecycle/recovery measurements before teardown; allocator attribution where needed. A clean PageHeap run is no general leak result. |
| Does a UI path retain GDI/USER objects or Windows handles? | Separate per-process GUI-object and kernel-handle observations, tied to the resource's matching release API. Heap/working-set counts do not cover all these resources. |
| Is portable Rust code violating ownership, initialization or aliasing obligations? | The admitted Miri lane on supported reached tests; keep actual Win32/FFI coverage separate. |

Use the debugger to retain the first verifier stop/exception, stack and relevant
module identity. Unexpected stops, truncated output, missing symbols, startup
failure or a killed test remain explicit; do not continue through faults and
report only the later process exit. Record which allocators and native modules
the selected checks actually cover. Custom pools or allocations outside observed
Windows heap APIs need their own evidence.

The default full PageHeap guard follows the allocation. A named underrun risk
needs a separately admitted `/backwards` run with a healthy/faulty control;
default overrun coverage does not establish that result. GFlags `/leaks`
switches to standard heap verification and disables `/full`, including its
modifiers. Do not combine these as an allegedly stronger full-PageHeap profile.

## Configure only the owned image

Use [the guarded PowerShell entry point](../../../../scripts/windows-memory.ps1)
from the inspected checkout. `-Action Preflight` checks installed tools without
elevation; it does not validate a candidate executable. `-Action Enable` and
`-Action Disable` require `-ImagePath <absolute-exe-path>` and elevated setup. The selected image
basename must match `NevroTick-<12-64 lowercase hex digits>.exe`; this naming
restriction does not itself prove ownership or content identity. Use the
runner's recorded state and TOOLING's current commands; do not substitute raw
registry edits when it refuses a target or detects changed settings. The runner
does not launch the application: run the admitted target/debugger separately
with its intended token after successful configuration.
The workload owner supplies the debugger/target deadline, output limits and
process cleanup. The helper's tool-command timeout and post-collection output
acceptance check do not supervise the debuggee or bound live output memory.

Keep the admitted current-user debugger startup configuration at
`%LOCALAPPDATA%\DBG\config.xml`: extension galleries,
automatic NuGet/credential-provider acquisition, network symbol paths and
automatic settings saving are disabled. Use TOOLING's exercised CDB options and
a reviewed command file; a CDB help spelling is not execution evidence. Retain
missing-symbol limits. This is not a sandbox for arbitrary extension/debuggee
code. Run unattended elevated configuration hidden with redirected output;
console selection can otherwise pause output and delay cleanup.
Run Preflight from the ordinary-user account that will run CDB; elevated
Enable/Disable admits tools and configures the image, but never starts CDB and
does not establish that ordinary user's debugger readiness.

1. Admit exact tool paths, architecture, versions/integrity and target executable
   identity. Use an owned Windows fixture with an explicit configuration and
   restoration owner. Before mutation, inspect existing per-image settings and
   running processes; refuse to overwrite unowned configuration or operate on a
   shared image basename. Image matching is not an absolute-path sandbox.
2. Record the pre-state and the selected finite checks/workload/budgets. Use only
   the documented target-specific Application Verifier and full PageHeap profile;
   no system-wide GFlags settings, arbitrary registry cleanup or unrelated
   feature changes. Select additional checks only for a named supported risk;
   enabling every layer can produce unsuitable combinations or distort evidence.
3. Apply the image configuration using the required elevated setup process,
   verify the effective settings, then launch a **new** target/debugger process
   with its intended account and token. Setup elevation must not elevate an
   ordinary-user UI/engine test. Record the actual token/environment; profile
   installation or WSL launch alone proves neither.
4. Execute the admitted healthy/faulty tool controls before relying on a new
   configuration, then the scoped product cases. Controls must reach the API
   under test and discriminate the intended fault. Retain native exits,
   verifier logs/stops, debugger output, candidate hash and selected/executed
   case counts outside the skills pack.
5. Observe target cleanup before harness teardown, stop only owned processes,
   and restore the owned image configuration on success, failure and timeout.
   Verify the restored state explicitly. Settings survive process exit and
   reboot; closing the debugger does not remove them. If configuration has
   changed concurrently, preserve it and report restoration incomplete rather
   than deleting another owner's values. Interrupted setup needs the same
   recorded recovery procedure.

Do not leave an active product profile when no product executable exists.
Before Cargo source exists, only isolated tool controls may run; create no
application workspace merely to qualify the tools. Registration/configuration
does not authorize service, clock, trust-store or package effects. UI-M0 stays
fixture-only, unelevated and within its existing D1/D2/D3 deferrals.

## Distinguish corruption detection from retention

For each resource-producing path, identify its owner, maximum retained quantity,
release event and cancellation/shutdown behavior. Exercise creation/use/release,
repeated recovery, partial construction failures, timeout and late completion.
Native work retains its permits until it actually finishes; cancelling a future
is not proof of cleanup. Avoid reference-count ownership cycles; a lint against
`mem::forget` does not rule out `Arc`/`Rc` cycles, unbounded caches or stuck tasks.
Safe Rust and garde validation establish no automatic resource-retention bound.
Check matching release semantics for each resource kind; a generic `CloseHandle`
wrapper is not a valid release strategy for every GUI object or native allocation.

Application Verifier's Leak layer reports tracked resources left when the owning
DLL unloads; it is not an exhaustive long-running process leak detector.
Debugger `!avrf -leak` shows outstanding tracked resources, which may still be
legitimately owned. Attribute them to expected release events before calling
them leaks, and retain the module/allocator/resource coverage limits.

Use [resource QA](../qa-performance-resources/SKILL.md) for warmup, repeats,
steady-state/recovery windows and declared bounds. Measure the actual owners'
private allocation/commit evidence where available, working set, handles,
threads and applicable GUI objects, plus internal queue/task/permit counts.
Distinguish allocator caching and transient peaks from retained growth; memory
still reachable from a cache can be a resource defect. Do not infer cleanup from
OS reclamation at process exit or fixture teardown.

Full PageHeap and verifier instrumentation alter memory usage, timing and
scheduling. Keep diagnostics separate from uninstrumented hardware performance
budgets, and preserve both candidate identities. A short control run does not
complete the planned one-hour baseline or 72-hour soak. Nextest's `leak-timeout`
concerns test process/output lifetime; it is not a heap-leak detector.

Report tool readiness, reached native defect checks, measured retention and
remaining product/allocator/native coverage separately. Neither this lane nor
Miri, ASan or CodeQL establishes absence of every memory error.

Primary tool behavior:
[Application Verifier](https://learn.microsoft.com/en-us/windows-hardware/drivers/devtest/application-verifier),
[verification layers](https://learn.microsoft.com/en-us/windows-hardware/drivers/devtest/application-verifier-tests-within-application-verifier),
[debugger resource observations](https://learn.microsoft.com/en-us/windows-hardware/drivers/debuggercmds/-avrf),
[GFlags and PageHeap](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/gflags-and-pageheap),
[GFlags command options](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/gflags-commands),
[Debugging Tools for Windows](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/debugger-download-tools).
