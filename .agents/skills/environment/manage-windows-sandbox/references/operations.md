# Windows Sandbox operations and WSL control

**Source review:** 3 November 2026. Examples are prospective, source-reviewed
commands, not a Windows execution receipt. Read the current
[Windows task](../../../../../tasks/windows11-integration.md) and
[execution holds](../../../../../docs/README.md#current-baseline-and-amendments)
before using them. The skill does not lift the Windows execution stop, separate
fuzz hold or storage pause. Existing authorization remains valid within its scope;
do not request it again merely because a command appears in this reference.

## Contents

- [Prerequisites and version detection](#prerequisites-and-version-detection)
- [Host control and guest payload](#host-control-and-guest-payload)
- [An owned session lifecycle](#an-owned-session-lifecycle)
- [Native command reference](#native-command-reference)
- [Exit status and evidence collection](#exit-status-and-evidence-collection)
- [Older client and configuration-file fallback](#older-client-and-configuration-file-fallback)
- [Failure and interruption rules](#failure-and-interruption-rules)
- [Sources](#sources)

## Prerequisites and version detection

Windows Sandbox requires a supported Windows edition, firmware virtualization,
adequate memory/disk and its optional Windows feature. The native `wsb` management
interface is documented from Windows 11 24H2; the Store-serviced client and its
actual installed version matter independently of the host build. Microsoft still
describes this CLI as preview. A failed client update can leave the older client
working. Check the installed interface before selecting an automation recipe.
[O1][O2][O3]

For an authorized preflight, these are **HOST, read-only** observations:

```powershell
Get-AppxPackage -Name WindowsSandbox | Select-Object Version
wsb --version
wsb --help
wsb list --raw
```

Resolve the genuine installed Windows control executable first; do not run a
same-named binary from a repository, download directory or untrusted PATH entry.
Record host edition/build/architecture, Sandbox package/CLI version, command help
needed by the chosen recipe, feature readiness and current instances. A version
string alone proves neither a usable guest nor product compatibility. Missing
capabilities are an environment gap, not a reason to change the host automatically.
Microsoft's released winapp implementation illustrates resolving the registered
Sandbox execution alias from Windows' known Local Application Data folder rather
than trusting arbitrary PATH ordering. That check is not protection against an
attacker already controlling the same Windows account. [O2][O7]

Feature enablement, client/tool installation, host restart and configuration
changes need their applicable existing authorization. Defer any required restart
while the owner's long WSL run is active. Do not join an Insider channel, disable
security controls, restart WSL or modify host services to make a recipe work.

## Host control and guest payload

The control CLI runs on **HOST Windows**. Only the command carried by `wsb exec`
runs in the **GUEST**. Calling `powershell.exe`, `sc.exe`, `reg.exe` or an installer
directly from WSL normally affects the host. Put every intended guest mutation
inside an explicit Sandbox operation with the recorded ID. A text skill cannot
remove host access already available to the agent. [O1][O4]

WSL can launch Windows executables using their `.exe` names. Windows arguments
are passed through without automatic Linux-to-Windows path conversion. Separate
three namespaces: a WSL path, a Windows host staging path, and a guest path.
For example, `/mnt/c/...`, `C:\...` and a mapped guest `C:\LabInput` do not identify
the same object without a deliberate mapping. Quote spaces and preserve argument
boundaries. Use an already admitted Windows-side PowerShell controller when app
execution aliases do not launch directly from WSL; do not change shared interop
settings or search for a downloadable replacement. [O4][O8]

For complex commands, stage a reviewed guest script through a narrow read-only
input mapping and invoke its fixed guest path. Keep dynamic values in validated
arguments or a data file. Avoid a chain of Bash, PowerShell and `cmd /c` string
interpolation: a redirection or expansion evaluated by the outer shell affects
the host. Never import guest output as host commands, profiles or executable code.

## An owned session lifecycle

1. **Prepare:** identify the task, exact candidate, permitted effects and expected
   result. Select an explicit configuration, memory budget, bounded duration and
   retry count. Create a unique run record outside the guest, using the existing
   task/evidence location. Record only necessary host facts; exclude private
   account/device identifiers from public receipts.
2. **Inventory:** inspect existing instances. Windows supports one Sandbox at a
   time. An instance found by `list` is not automatically yours. Serialize control
   with the other agents; do not adopt, repair, mutate or stop an unknown instance.
3. **Create:** use a fresh GUID for a new authorized run and record it before
   starting. `--config` takes the XML **contents**, not a `.wsb` filename. Save the
   exact configuration, start result and observed instance identity.
4. **Establish:** reconcile the requested ID with the returned/current instance;
   record its generation and readiness. For `ExistingLogin`, establish the guest
   desktop session with `connect`. Validate the intended guest context using
   evidence produced inside that guest.
5. **Execute:** stage bounded inputs; copy build inputs into guest-private storage
   if they must be writable. Select the intended guest principal. Track the real
   job and independent result, not merely successful control-plane dispatch.
6. **Collect:** retrieve logs, exit/result records and required artifacts before
   ending the guest. Verify completeness and candidate/run identity. Classify
   partial, missing or cancelled work explicitly.
7. **Finish:** observe application cleanup before destroying the environment;
   stop only the owned ID when the agreed run lifecycle permits it. Confirm the
   stopped/absent state and release ownership. Preserve evidence on the host.

The GUID is an address and ownership record, not an authorization token or proof
of continuous boot identity. Never reuse it for a later run. A missing start
response can accompany an instance that was actually created; inspect that exact
requested ID before retrying. Do not assume failure means nothing changed. [O1][O2][O6]

## Native command reference

All invocations below are **HOST control commands**. `$sandboxId`, paths and
configuration are explicit values from the authorized run record. These examples
do not create a runnable experiment by themselves. [O1]

| Intent | Command | Boundary or prerequisite |
| --- | --- | --- |
| Inventory | `wsb list --raw` | Current user's Sandbox inventory; do not pick an arbitrary first result. |
| Start | `wsb start --id $sandboxId --config $configuration --raw` | `$configuration` is reviewed XML text, for example read from the chosen file with `Get-Content -Raw -LiteralPath`. |
| Open guest desktop | `wsb connect --id $sandboxId` | Establishes the session needed by `ExistingLogin`; may affect host window focus. |
| Read-only input share | `wsb share --id $sandboxId --host-path C:\Lab\Input --sandbox-path C:\LabInput` | Share only the prepared input directory. |
| Writable result share | `wsb share --id $sandboxId --host-path C:\Lab\Output --sandbox-path C:\LabOutput --allow-write` | Guest writes persist on the host; use a separate, bounded run directory. |
| Guest execution | `wsb exec --id $sandboxId --run-as ExistingLogin --command $guestCommand --raw` | `$guestCommand` is the reviewed **GUEST** command string; capture control output separately. |
| Guest working directory | Add `--working-directory C:\Lab\Work` to `exec` | The path is inside the guest and must exist. |
| Privileged guest setup | Use `--run-as System` for the specifically authorized `exec` | Guest SYSTEM; does not justify running the product or user-flow tests as SYSTEM. |
| Guest IP | `wsb ip --id $sandboxId --raw` | Needed only by a networking recipe; not an ownership check. |
| End run | `wsb stop --id $sandboxId --raw` | Destroys private guest state and releases its resources; export evidence first. |

`ExistingLogin` requires an active guest session and is not evidence of a standard
unelevated user's token. `System` uses a different principal/profile from the
interactive user. Observe the effective token needed by the test; distinguish
privileged setup from ordinary-user behavior and MSIX registration semantics.

Normalize documentation examples carefully: configuration values use `Disable`,
whereas the CLI page currently shows `Disabled` in one networking sample. Use
ASCII `--id`; the same page contains a typographic dash in an `exec` example.
Prefer documented long options and confirm installed help. [O1][O5]

## Exit status and evidence collection

Native `wsb exec` currently does **not** forward guest process stdout/stderr.
Arrange guest-side file logging before execution, with a deliberate mapped output
directory or another already admitted extraction path. Redirection after `wsb`
captures the host CLI's output, not the guest application's streams. [O1]

There are two status layers. The released Microsoft winapp adapter at commit
`e3cd0b94f3e7b776ac51ba0564f4908094e979b5` records that `wsb exec --raw` can exit
zero after successfully dispatching a guest process which itself exits nonzero.
The guest result is reported as JSON such as `{"ExitCode":42}`. Its adapter treats
a nonzero host CLI status or CLI diagnostic stderr as an infrastructure failure,
and refuses absent/malformed guest exit evidence. Do not report success from
`$LASTEXITCODE` alone. Preserve raw control output, the parsed guest result and the
independent job completion record. This is pinned implementation/test evidence,
not a promise that every future preview version has an identical schema. [O6]

Likewise, that adapter accepts a root `Id` or `WindowsSandboxEnvironments[].Id`
for `start`, and the latter wrapper for `list`; Learn documents JSON output
without defining these shapes as a stable contract. Record the installed
version, reject unrecognized output, and reconcile the requested ID explicitly.
Never interpret a missing collection/field as a confirmed empty inventory. [O1][O6]

`wsb exec` has no documented detach option. Microsoft's adapter backgrounds the
host `wsb exec` used for its persistent guest agent because it lasts for the
agent's lifetime. Background launch, a controller timeout or cancellation is not
proof that a guest job finished or stopped. Use bounded guest jobs, explicit
completion evidence and owned cleanup; retain an indeterminate outcome when the
observation is lost. [O6]

## Older client and configuration-file fallback

An older client can launch a reviewed `.wsb` file through Windows shell file
association. Its `LogonCommand` runs inside the guest after configured folders
are mapped. This can bootstrap a fixed guest script and file-based results, but
does not supply the modern CLI's inventory/exec interface. Use it only if the
authorized procedure can identify the owned guest, observe completion and safely
collect evidence. Otherwise record the missing automation capability. Never
substitute executing the same script on the host. [O5]

Guest restart support and viewer-close behavior depend on Windows/client/start
path. Do not use window closure as a universal stop or persistence contract.
Use explicit ID-based stopping for a CLI-managed run. Recreating a guest requires
new identities and bootstrap; a preserved writable host share is not a saved
guest checkpoint. [O2][O9]

## Failure and interruption rules

Keep retries bounded and conditional on a changed cause. A timed-out start, broken
connection or failed result transfer first needs inventory/evidence reconciliation.
Do not destroy the only remaining evidence to obtain a clean retry. If an unknown
instance blocks creation, return its ownership question to the coordinator rather
than stopping it. For a missing feature, pending restart or unusable client, retain
the setup gap and continue independent source work. Source/documentation review
does not complete any native CHECK gate.

## Sources

- [O1: Microsoft Learn — Windows Sandbox CLI](https://learn.microsoft.com/en-us/windows/security/application-security/application-isolation/windows-sandbox/windows-sandbox-cli).
- [O2: Microsoft Learn — Sandbox FAQ](https://learn.microsoft.com/en-us/windows/security/application-security/application-isolation/windows-sandbox/windows-sandbox-faq).
- [O3: Microsoft Learn — versions](https://learn.microsoft.com/en-us/windows/security/application-security/application-isolation/windows-sandbox/windows-sandbox-versions) and [installation requirements](https://learn.microsoft.com/en-us/windows/security/application-security/application-isolation/windows-sandbox/windows-sandbox-install).
- [O4: Microsoft Learn — running Windows tools from WSL](https://learn.microsoft.com/en-us/windows/wsl/filesystems#run-windows-tools-from-linux).
- [O5: Microsoft Learn — `.wsb` configuration](https://learn.microsoft.com/en-us/windows/security/application-security/application-isolation/windows-sandbox/windows-sandbox-configure-using-wsb-file).
- [O6: Microsoft winapp v0.7.1 — native CLI adapter, especially `StartAsync`, `RunExecAsync` and `LaunchAgentAsync`](https://github.com/microsoft/winappCli/blob/e3cd0b94f3e7b776ac51ba0564f4908094e979b5/src/winapp-CLI/WinApp.Cli/ExecutionTargets/WindowsSandbox/WindowsSandboxCli.cs); [exit classification tests](https://github.com/microsoft/winappCli/blob/e3cd0b94f3e7b776ac51ba0564f4908094e979b5/src/winapp-CLI/WinApp.Cli.Tests/WindowsSandboxCliClassificationTests.cs); [additional CLI tests](https://github.com/microsoft/winappCli/blob/e3cd0b94f3e7b776ac51ba0564f4908094e979b5/src/winapp-CLI/WinApp.Cli.Tests/WindowsSandboxCliTests.cs).
- [O7: Microsoft winapp v0.7.1 — trusted alias resolution and bounded version probe](https://github.com/microsoft/winappCli/blob/e3cd0b94f3e7b776ac51ba0564f4908094e979b5/src/winapp-CLI/WinApp.Cli/ExecutionTargets/WindowsSandbox/WindowsSandboxHostProbe.cs).
- [O8: Microsoft Learn — Windows Terminal's WSL alias caveat](https://learn.microsoft.com/en-us/windows/terminal/command-line-arguments#command-line-argument-examples); this illustrates an alias interoperability limitation, not a universal test result for `wsb.exe`.
- [O9: Version-specific viewer lifetime](winapp.md#lifecycle-and-version-drift); distinguish its cited released sources from current Learn and development-branch descriptions.
