# Windows Sandbox resources and WSL coordination

Read before allocating a guest alongside an ongoing WSL development run or
dispatching Windows control operations from Linux. Research checked
**3 November 2026**. No resource measurement or Windows execution is established
by this reference.

## Contents

- [Protect the ongoing workload](#protect-the-ongoing-workload)
- [Choose an adaptive guest budget](#choose-an-adaptive-guest-budget)
- [Interpret memory and CPU observations](#interpret-memory-and-cpu-observations)
- [Keep WSL configuration changes separate](#keep-wsl-configuration-changes-separate)
- [Bridge from WSL without losing the target](#bridge-from-wsl-without-losing-the-target)
- [Choose the environment for the evidence gap](#choose-the-environment-for-the-evidence-gap)
- [Sources](#sources)

## Protect the ongoing workload

On 3 November 2026, the owner reported possible WSL usage of **22 GB** and
substantial CPU during a four-day agent run. This is workload context, not a
current measurement, configured limit, leak diagnosis or permission to interrupt
the job. Recheck the situation before an authorized Sandbox run.

Do not create room by terminating distributions, restarting WSL, killing agent
processes, forcing cache drops, rebooting Windows, or changing host security.
If available resources are insufficient, defer the Sandbox run and continue
independent source work. Existing execution holds remain governed by the
[Windows task](../../../../../tasks/windows11-integration.md) and
[incident record](../../../../../tasks/s0.1-crypto-feasibility.md#defender-alert--windows-execution-stopped).

Enabling Sandbox requires administrator authority and may need a host reboot.
Prepare missing prerequisites for an appropriate owner checkpoint; do not change
the host, update clients or join Insider builds as incidental diagnostics. [R3]

## Choose an adaptive guest budget

Consider the proposed **3072 MB** offline profile in
[configuration and boundaries](configuration-and-boundaries.md), or 4096 MB
if the workload needs it and measured headroom permits it. Neither is a proven
budget for the laptop's current workload.

Microsoft documents a default Sandbox maximum capacity of 4 GB and automatic
raising of an insufficient `MemoryInMB` value to the required 2048 MB minimum.
That setting describes guest memory, not a guarantee about aggregate Windows,
WSL, graphics, paging or controller overhead. [R1]

Before launch, through existing authorized read-only tools:

1. Record the observed Windows available physical memory and commitment, plus
   the current CPU/disk pressure and the measurement time.
2. Identify the ongoing WSL work, its ownership and its demand trend.
3. Estimate guest demand: a prebuilt check differs from SDK installation,
   compilation and parallel tests.
4. Leave measured headroom for Windows and variation. There is no universal
   free-RAM threshold or fixed incremental Sandbox footprint here.
5. Admit one bounded guest workload. Set duration, concurrency and output limits
   before dispatch; reassess if the expected workload changes materially.
6. If the host becomes constrained, stop or postpone only the owned Sandbox
   work according to its cancellation/evidence plan. Do not sacrifice the
   unrelated WSL run to preserve a test campaign.

Sandbox shares physical memory pages for immutable Windows binaries and can
return memory under host pressure. This supports economical short sessions but
does not guarantee responsive operation under heavy simultaneous workloads.
The documented compressed 30 MB / installed approximately 500 MB dynamic base
image sizes describe **disk storage**, not guest RAM. [R2]

No CPU-count or CPU-quota control is documented in the current `.wsb` interface.
Reduce the actual guest workload and its tool-level concurrency where supported;
do not invent a Sandbox processor field. A vGPU-disabled profile uses WARP
software rendering, which can increase CPU load. A graphical performance
comparison must record that difference. [R1]

## Interpret memory and CPU observations

Keep measurements in their own units and scope:

| Observation | What it can show | What it does not establish alone |
| --- | --- | --- |
| Installed/usable RAM | Machine capacity | Currently free memory. |
| Windows available physical memory | Present physical headroom | Future headroom during a workload spike. |
| Windows commitment | Committed allocations against the commit limit | The resident working set of one process. |
| WSL VM memory shown by a Windows observer | That observer's current VM/process accounting | A Linux memory leak or the size of one agent. |
| Linux process resident memory | Resident pages attributed by that tool | All VM overhead or a sum safe from shared-page double counting. |
| Linux available memory and cache | Guest allocation/cache context | Memory that can instantly be reclaimed without affecting active work. |
| Nonzero swap use | Some pages have been swapped | Current continuous paging or CPU saturation. |

Compare a small time series when pressure is uncertain. Use installed read-only
tools such as Task Manager and Linux memory/process summaries; no new monitoring
stack is needed. Preserve timestamps, units, observer identity and missing
counters in the receipt.

For already bounded, decoded English-profile **HOST** `systeminfo.exe` stdout,
reuse [the pure memory-field parser](../../../../../tools/windows-sandbox/host_memory.py)
and its [focused controls](../../../../../scripts/tests/test_windows_sandbox_memory.py).
It handles consistent comma/space/NBSP/narrow-NBSP grouping and rejects duplicate,
malformed or oversized observations. Bind that module into the caller's input
freeze; the caller still owns the genuine host command, actual exit, bounded
capture/decoding, measurement time and selected admission floor. Parser success
is an observed count, not permission to run, and Linux memory is no substitute.
Keep missing/out-of-profile data unavailable rather than supplying a default.

The [10 November S2.2 source/resource checkpoint](../../../../../tasks/s0.1-crypto-feasibility.md#source-preparation-and-resource-stop--10-november-2026)
retains its particular 2048-MB **available host memory** floor and resource hold.
That is separate from Sandbox's documented minimum guest allocation above.
Do not automatically change WSL configuration, dispose of the prepared guest or
restart held work when a parser/test passes. A generated formatting regression
does not reconstruct a discarded original observation.

Cache reclaim, dropping a guest and aborting a test are different operations.
Aborting a controller wait does not prove that the guest process stopped.
Confirm termination or retain the operation as incomplete before admitting a
replacement workload.

## Keep WSL configuration changes separate

Current Microsoft documentation lists these `.wslconfig` defaults: memory at
50% of Windows RAM, all logical processors, and disk-backed swap at 25% of
Windows memory rounded up. The actual installation may differ; inspect its
configured state before reasoning from defaults. Do not infer David's settings
from the reported 22 GB. [R4]

The current experimental `autoMemoryReclaim` setting documents `dropCache` as
the default, with `disabled`, `gradual` and `dropCache` choices. Reclaim concerns
cached memory and does not guarantee that active applications can surrender
their allocations. Keep version-specific historical behavior separate from the
installed version's observed behavior. [R4]

WSL configuration takes effect after the subsystem stops and restarts.
Microsoft's eight-second explanation applies after distribution activity stops;
it is not a safe restart window while agents work. `wsl --shutdown` stops the
running distributions and WSL 2 VM. Distribution termination also interrupts
work. Prepare later tuning separately, preserving the running jobs. [R4]

## Bridge from WSL without losing the target

Launching a Windows executable from WSL runs it on the **Windows host**, under
the Windows user's authority. It does not automatically execute inside Sandbox.
The host controller must deliberately select the guest for each payload. WSL
file/executable access is not made safe merely by Linux root being unable to
exceed the Windows user's permissions. [R5, R6]

Use these dispatch rules:

- Discover installed commands and supported subcommands; record version and
  exit status. OS version alone does not establish alias availability.
- Resolve a Windows path for host-control files and a separate guest path for
  payload files. `/mnt/c/...`, `C:\...` and a path inside the guest are different
  namespaces even when they refer to similar content.
- Prefer an inspectable host-side command/script file or structured argument
  array over a long Bash-to-PowerShell-to-guest string. Keep host control and
  guest payload scripts separate and label their intended environment.
- Pass task text, filenames and guest output as data, preserving quoting across
  parsers. Do not compensate for command-construction errors with automatic host
  `ExecutionPolicy Bypass`, elevation or startup-profile changes.
- Require the owned Sandbox ID for guest execution and destructive lifecycle
  actions. Missing target information is a failed preflight, not an invitation
  to run the same command on the host.
- Keep bounded guest result files when the chosen transport does not stream
  process output. Distinguish controller failure, guest exit, timeout and
  evidence-transfer failure; do not collapse them into a generic success.

The optional `winapp` Sandbox backend has its own guest TCP/bootstrap requirements
and needs a separately admitted network profile. An offline `.wsb` example does
not make that backend offline-capable. Never enable the host firewall broadly
or modify WSL networking as an automatic control-channel repair. [R8]

## Choose the environment for the evidence gap

Use on-demand Sandbox for short Windows observations with a small bootstrap and
bounded artifact. Reinstalling a full development environment each time can
defeat its resource advantage. Decide where compilation is allowed: host builds
execute host tooling, and Defender-related staging holds still apply.

Sandbox uses the host's OS build, currently supports one instance at a time,
and has no general persistent checkpoint workflow. Guest reboot persistence
does not create a durable multi-day laboratory. The Store application and
optional Windows features are not generally available as in a full installation.
Use a later owned VM or remote Windows environment when a concrete gap requires
another build, durable state, a fuller installation or checkpoint-controlled
experiments. Do not make that future environment a prerequisite for independent
portable Core work. [R3, R7]

Store delivery, user/token matrices and physical clock, power or timestamp
properties need their appropriate environments. Carry each gap to its existing
CHECK owner; Sandbox success does not establish release qualification.

## Sources

- **R1:** Microsoft Learn, [Use and configure Windows Sandbox](https://learn.microsoft.com/windows/security/application-security/application-isolation/windows-sandbox/windows-sandbox-configure-using-wsb-file).
- **R2:** Microsoft Learn, [Windows Sandbox architecture](https://learn.microsoft.com/windows/security/application-security/application-isolation/windows-sandbox/windows-sandbox-architecture).
- **R3:** Microsoft Learn, [Install Windows Sandbox](https://learn.microsoft.com/windows/security/application-security/application-isolation/windows-sandbox/windows-sandbox-install).
- **R4:** Microsoft Learn, [Advanced settings configuration in WSL](https://learn.microsoft.com/windows/wsl/wsl-config).
- **R5:** Microsoft Learn, [WSL in an enterprise environment](https://learn.microsoft.com/windows/wsl/enterprise).
- **R6:** Microsoft Learn, [Working across Windows and Linux file systems](https://learn.microsoft.com/windows/wsl/filesystems).
- **R7:** Microsoft Learn, [Common questions about Windows Sandbox](https://learn.microsoft.com/windows/security/application-security/application-isolation/windows-sandbox/windows-sandbox-faq).
- **R8:** Microsoft `winappCli` v0.7.1 implementation, [WindowsSandboxBackend.cs at e3cd0b9](https://github.com/microsoft/winappCli/blob/e3cd0b94f3e7b776ac51ba0564f4908094e979b5/src/winapp-CLI/WinApp.Cli/ExecutionTargets/WindowsSandbox/WindowsSandboxBackend.cs), guest TCP transport and IP lookup.

Budgeting, dispatch and measurement recommendations are operating guidance.
They establish no measured performance, installed configuration or native pass.
