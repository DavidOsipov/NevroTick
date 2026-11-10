# Windows Sandbox configuration and boundaries

Use this reference when preparing a `.wsb` profile, admitting host/guest data
exchange, or deciding whether an operation stays inside the disposable guest.
Research checked **3 November 2026**. The examples below are proposed,
source-reviewed configuration, not a deployed or Windows-tested profile.

## Contents

- [Authority and isolation](#authority-and-isolation)
- [Explicit offline profile](#explicit-offline-profile)
- [Defaults and version differences](#defaults-and-version-differences)
- [Host folders and evidence](#host-folders-and-evidence)
- [Network and device exposure](#network-and-device-exposure)
- [Policy and effective configuration](#policy-and-effective-configuration)
- [Lifecycle and recovery boundaries](#lifecycle-and-recovery-boundaries)
- [Sources](#sources)

## Authority and isolation

First read the current [Windows integration task](../../../../../tasks/windows11-integration.md)
and [execution incident](../../../../../tasks/s0.1-crypto-feasibility.md#defender-alert--windows-execution-stopped).
This reference supplies a reusable operating procedure. It grants no new
permission to execute a held fixture, retry a blocked artifact, modify host
security, or begin paused storage experiments. Existing authorization continues
within its stated scope; do not add repetitive approval requests to an already
authorized run.

Windows Sandbox uses a separate kernel under Microsoft's hypervisor. Its guest
is disposable, but intentionally exposed host folders, networks and devices
remain communication paths. A guest administrator can change guest state;
that does not authorize changing the host. The agent's existing WSL shell and
Windows tools retain their own host authority. Microsoft documents that Windows
file/executable access from WSL uses the Windows user's permissions. Following
this skill does not technically confine an otherwise unrestricted agent. [C2, C5]

Record each action as **host control**, **guest execution**, or **host evidence
collection** before dispatch. Resolve the actual target and owned Sandbox ID;
labels, window titles and a Windows-looking path alone do not establish where
a command will execute. A future restricted controller could enforce an
allowlist, but this skill does not implement one.

## Explicit offline profile

This profile proposes an initial **3072 MB** guest budget for short local work.
Use it only when the current resource preflight permits that allocation. It
deliberately has no `LogonCommand`: the first run should establish the environment
and ownership before executing a payload. Read
[resources and WSL](resources-and-wsl.md) before selecting a budget. [C1]

Replace `REPLACE_WITH_RUN_ID` with the owned run's identifier and resolve both
host directories before launch. The illustrative paths are not directories
already installed on the laptop. The input directory contains only admitted
inputs; the separate, initially empty output directory is writable because the
run explicitly needs to export evidence. If no export is required, omit that
mapping. If no inputs are required, omit the input mapping too.

```xml
<Configuration>
  <vGPU>Disable</vGPU>
  <Networking>Disable</Networking>
  <AudioInput>Disable</AudioInput>
  <VideoInput>Disable</VideoInput>
  <ProtectedClient>Enable</ProtectedClient>
  <PrinterRedirection>Disable</PrinterRedirection>
  <ClipboardRedirection>Disable</ClipboardRedirection>
  <MemoryInMB>3072</MemoryInMB>
  <MappedFolders>
    <MappedFolder>
      <HostFolder>C:\SandboxLab\REPLACE_WITH_RUN_ID\input</HostFolder>
      <SandboxFolder>C:\SandboxInput</SandboxFolder>
      <ReadOnly>true</ReadOnly>
    </MappedFolder>
    <MappedFolder>
      <HostFolder>C:\SandboxLab\REPLACE_WITH_RUN_ID\output</HostFolder>
      <SandboxFolder>C:\SandboxOutput</SandboxFolder>
      <ReadOnly>false</ReadOnly>
    </MappedFolder>
  </MappedFolders>
</Configuration>
```

XML escaping applies to substituted paths: a literal ampersand must become
`&amp;`. Do not splice an untrusted path or command into raw XML. Validate the
generated XML and record its exact content/hash before using it. Syntax checking
establishes neither installed-client acceptance nor effective isolation.

Qualify startup, the intended guest identity, read-only input behavior, bounded
output transfer and termination in an authorized smoke run. Keep incompatibility
as an explicit failure; do not silently enable clipboard/networking or disable
Protected Client. The released `winapp` Sandbox backend uses guest TCP and needs
a separately admitted network-enabled profile. This offline profile is not a
promise of `winapp` compatibility. [C6]

## Defaults and version differences

Current Learn documentation describes the following defaults. Specify relevant
values explicitly so a run receipt states the intended behavior. [C1]

| Setting | Documented default | Recommended starting treatment |
| --- | --- | --- |
| `Networking` | Enabled through the Hyper-V default switch | `Disable` for offline work. |
| `vGPU` | Enabled on non-Arm64 devices | `Disable` unless graphics behavior needs an admitted alternative. |
| `AudioInput` | Enabled | `Disable`. |
| `VideoInput` | Disabled | Keep `Disable` explicit. |
| `ProtectedClient` | Disabled | Propose `Enable`; qualify client compatibility. |
| `PrinterRedirection` | Disabled | Keep `Disable` explicit. |
| `ClipboardRedirection` | Enabled | `Disable`; use bounded file transfer. |
| `MemoryInMB` | A default launch has maximum capacity 4 GB | Choose a measured-feasible budget; 3072 MB is a proposal. |
| `MappedFolder.ReadOnly` | `false` | Set `true` for inputs and permit writing only to an owned output directory. |

The feature switches above accept `Enable`, `Disable` and `Default`;
`ReadOnly` uses `true` or `false`. Do not copy `Disabled` from a conflicting
example. If a memory value is too small to boot, Sandbox raises it to the
documented required minimum of **2048 MB**. There is no documented CPU-count or
CPU-quota field in this configuration interface. [C1]

Protected Client adds AppContainer-based protection to the Sandbox client/RDP
session. Do not infer that each guest application thereby receives an
AppContainer token or that its package trust is qualified. Copy/paste may be
restricted by this mode. Disabling vGPU substitutes WARP software rendering,
which can increase CPU use and alter graphical behavior. [C1]

Use absolute host and guest folder paths. The configuration page explicitly
rejects relative paths despite an inconsistent sample placeholder. Environment
variables in mapped paths are documented from Windows 11 **23H2**; resolving
absolute paths first avoids that dependency. Map folders before an optional
logon command; that is the documented execution order. [C1]

## Host folders and evidence

Mappings deliberately cross the boundary. Read-only access protects against
that guest write route but still discloses the files. A writable mapping allows
guest changes to survive disposal. Clearing the guest does not undo those
changes. [C1]

Apply these operating rules to the selected run:

1. Stage the exact admitted inputs in a dedicated directory. Do not expose an
   entire checkout, drive, user profile, credential store, browser profile or
   general Downloads folder for convenience. Exclude secrets and unrelated
   owner data even from read-only shares.
2. Resolve and inspect the host paths and their parent components. Reject
   unexpected symbolic links, junctions, reparse points or ownership changes
   in the staging/output paths. These are recommended controller checks, not
   an undocumented guarantee supplied by `.wsb`.
3. Use an initially empty, uniquely named output directory with no valuable
   host files beneath it. Prevent concurrent runs from writing there. Do not
   let the guest choose another host destination through a request file.
4. Copy writable build/test inputs into guest-private storage. A read-only
   source mapping is unsuitable as a build output directory. Keep the mapped
   input's exact identity in the run receipt.
5. Bound output size, file count and collection duration. Collect the declared
   result files and logs, including failures, and reject unexpected path
   escapes or reparse points during collection. Treat the files as untrusted
   data; do not auto-execute guest-produced scripts or binaries on the host.
6. Attribute evidence to the run ID, candidate, guest instance and observed
   completion. A pre-existing `success.json`, exit zero without expected
   observations, or a partial log does not prove a passed case.

Prefer explicit file names and text or inert structured output. Do not use an
export directory as an executable command queue for a privileged host watcher.
Keep credentials out of commands, configurations, crash dumps and receipts.
Apply the existing project diagnostic/redaction contracts to captured data.

## Network and device exposure

Default Sandbox networking can expose applications to the internal network.
The Hyper-V default switch is not evidence of a deny-by-default egress policy.
An offline profile keeps guest network access disabled; a networked profile
requires a concrete purpose, destinations and bounded traffic. [C1, C2]

For a networked `winapp` control session or protocol experiment, identify the
control channel and test traffic separately. Record the actual guest network
configuration and any tool-created guest firewall rule. Keep the host firewall,
WSL network mode and production network services intact. Missing connectivity
does not authorize broad host firewall exceptions or a silent switch to host
execution. Use controlled peers for routine protocol cases under the existing
project contract.

Keep clipboard, microphone, camera and printer redirection disabled unless the
specific observation requires an authorized exception. The clipboard can carry
both secrets and files; it is not a necessary default export mechanism. Do not
ask the owner to paste credentials into the guest simply to simplify automation.

## Policy and effective configuration

Group Policy can prevent changes requested by a `.wsb` file. The Sandbox policy
surface covers networking, vGPU, audio/video, clipboard and printers. Starting
with Windows 11 **24H2**, the documented `AllowMappedFolders` and
`AllowWriteToMappedFolders` policies can also restrict sharing. [C3, C4]

Record policy interference and the effective result; do not edit policy or the
registry to force a requested profile. The separately authorized
[guest-only SAC lab workaround](troubleshooting.md#smart-app-control-inside-the-guest)
is a specific exception, with configured/effective/retest evidence and no host
change; it does not qualify the SAC-enabled product profile. The current CSP description of
`AllowWriteToMappedFolders` repeats an apparently inconsistent registry mapping
for `AllowMappedFolders`. Use that page for supported policy semantics, not as
an unverified registry recipe. Local policy state and observed behavior remain
necessary evidence.

When requested and effective settings differ, narrow the run or report the
blocker. Do not claim hardening from the presence of XML alone, and do not
replace a required protection with an unrecorded fallback.

## Lifecycle and recovery boundaries

Only one concurrent Sandbox instance is currently documented. Do not adopt,
reset or stop an existing instance without establishing its ownership and
authorization. A guest restart can retain state on Windows 11 22H2 and later;
disposal discards private state. Export evidence before explicitly terminating
the owned instance. Viewer-close behavior depends on client/tool version and
must not substitute for a verified lifecycle operation. [C2, C3]

A fresh Sandbox does not prove product uninstall, restoration of W32Time,
service recovery or package cleanup. Observe the product's own cleanup before
environment disposal can hide leftovers. Keep the existing CHECK owners and
[MSIX laboratory plan](../../../../../docs/plans/windows-msix-store.md)
as the acceptance chain; this reference creates no new product gate.

## Sources

- **C1:** Microsoft Learn, [Use and configure Windows Sandbox](https://learn.microsoft.com/windows/security/application-security/application-isolation/windows-sandbox/windows-sandbox-configure-using-wsb-file).
- **C2:** Microsoft Learn, [Windows Sandbox overview](https://learn.microsoft.com/windows/security/application-security/application-isolation/windows-sandbox/).
- **C3:** Microsoft Learn, [Common questions about Windows Sandbox](https://learn.microsoft.com/windows/security/application-security/application-isolation/windows-sandbox/windows-sandbox-faq).
- **C4:** Microsoft Learn, [Policy CSP — WindowsSandbox](https://learn.microsoft.com/windows/client-management/mdm/policy-csp-windowssandbox).
- **C5:** Microsoft Learn, [WSL in an enterprise environment](https://learn.microsoft.com/windows/wsl/enterprise).
- **C6:** Microsoft `winappCli` v0.7.1 implementation, [WindowsSandboxBackend.cs at e3cd0b9](https://github.com/microsoft/winappCli/blob/e3cd0b94f3e7b776ac51ba0564f4908094e979b5/src/winapp-CLI/WinApp.Cli/ExecutionTargets/WindowsSandbox/WindowsSandboxBackend.cs), including guest TCP transport and Sandbox IP lookup.

Recommendations about staging, bounds, path checks and ownership above are
project operating guidance derived from these interfaces and the current
repository contracts. They are not claims that Microsoft automatically enforces
every listed check.
