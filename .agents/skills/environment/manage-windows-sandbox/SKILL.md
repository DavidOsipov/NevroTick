---
name: "manage-windows-sandbox"
description: "Prepare and manage owned Windows Sandbox sessions for NevroTick agents working from Windows or WSL: select a version-compatible control path, protect the host and active WSL jobs, stage minimal inputs, run bounded guest experiments, export evidence and dispose of the guest. Use for Sandbox configuration, CLI automation, troubleshooting and lab readiness; not ordinary portable Rust work or host cleanup."
---

# Manage Windows Sandbox

## Establish scope

Read [AGENTS.md](../../../../AGENTS.md), the current
[Windows task](../../../../tasks/windows11-integration.md) and
[testing contract](../../testing/CONTRACT.md). Reuse existing authorization;
check the current execution holds before any Windows command. This skill neither
lifts a hold nor requires renewed approval for already authorized work. Keep the
current Core priority, separate fuzz hold, storage pause and UI deferrals.

Record the existing task/CHECK owner, candidate, independent expected result,
allowed guest effects, host changes, duration and resource budget. For research
or skill maintenance, stop at documentation: do not probe or launch Windows.

## Operate one owned session

1. **Preflight.** Read [operations](references/operations.md) and
   [resources and WSL](references/resources-and-wsl.md). Establish actual Windows,
   Sandbox and control-tool versions; inspect existing sessions and current host
   headroom through admitted means. Preserve running WSL jobs. Missing features,
   insufficient capacity or an execution hold leave the run incomplete; do not
   install, reboot, terminate WSL or relax protection to unblock it.
2. **Choose a boundary.** Read [configuration and boundaries](references/configuration-and-boundaries.md).
   Prefer the proposed offline, on-demand profile, subject to compatibility and
   measured capacity. Share only exact run inputs read-only and a separate bounded
   output folder when needed. Keep secrets and the working checkout outside the
   shares. Requested XML is not proof of effective policy.
3. **Own the instance.** Serialize access. Record a unique run ID, Sandbox ID,
   configuration identity and owner before effects. Never adopt, replace or stop
   an unrelated instance. Label every command **HOST control** or **GUEST payload**;
   verify its target before mutation. The agent's existing WSL/host rights remain
   outside Sandbox isolation.
4. **Execute narrowly.** Use native `wsb` with an explicit ID and principal.
   `ExistingLogin` needs a guest session and does not prove a standard-user token;
   guest `System` is only for the bounded privileged step. Native `wsb` has no
   guest stdout/stderr stream. Check host dispatch, parsed guest `ExitCode` and
   the independent outcome separately; host exit 0 alone is insufficient.
   For optional `winapp`, first read [its separate profile](references/winapp.md):
   its guest transport needs networking, builds can execute on the host, and UI
   commands need explicit `--on sandbox`. Never silently fall back to the host.
5. **Observe and export.** Use [NevroTick evidence](references/nevrotick-evidence.md)
   for services, MSIX, clock and UI claims. Observe product cleanup before harness
   disposal masks it. Export bounded logs and actual outcomes before termination;
   treat returned files as untrusted data. Record failures, missing evidence and
   timeouts without promoting them to passes.
6. **Finish deliberately.** Follow [troubleshooting and closeout](references/troubleshooting.md).
   Stop only the owned guest after collection, verify termination and preserve
   the run receipt. Window closure, cancellation and a `winapp` snapshot are not
   reliable substitutes for explicit lifecycle evidence. Reuse conclusions only
   while their candidate, environment and prerequisites remain valid.

Report what actually ran, host/guest identities, configuration and privileges,
results, evidence paths, remaining native gaps and the next permitted action.
Keep one receipt in the existing task; do not create a parallel acceptance catalog.
