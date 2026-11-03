# What a Sandbox result proves for NevroTick

Research checked 3 November 2026. This reference maps a possible environment to
existing requirements. It reports no executed experiment and selects no new
product topology, package capability or release route.

## Contents

- [Current authority and admission](#current-authority-and-admission)
- [Choose the smallest sufficient environment](#choose-the-smallest-sufficient-environment)
- [Services and security principals](#services-and-security-principals)
- [MSIX and Store boundaries](#msix-and-store-boundaries)
- [Clock and timing evidence](#clock-and-timing-evidence)
- [Cleanup and recovery](#cleanup-and-recovery)
- [Run receipt](#run-receipt)
- [Sources](#sources)

## Current authority and admission

Read the current [Windows task](../../../../../tasks/windows11-integration.md),
[Defender incident](../../../../../tasks/s0.1-crypto-feasibility.md#defender-alert--windows-execution-stopped)
and [development next task](../../../../../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent).
As of this reference's preparation, Windows execution is stopped, the separate
fuzz runner must not be retried, and storage experiments are paused. A proposal
to use Sandbox, or the presence of this skill, does not change those decisions.
Use a later explicit disposition when present; do not perpetuate a superseded
hold or ask again for work already authorized by the owner.

Keep the selected complete hardened MSIX and David's Store attempt under
[amended OD-05](../../../../../docs/nevrotick-owner-decisions.md#7-od-05--signing-and-release-ownership).
The [MSIX plan](../../../../../docs/plans/windows-msix-store.md) owns supported
provisioning, trust, confinement and lifecycle requirements. This lab guide
supplies a method, not an alternative architecture or new acceptance gate.
Independent portable Core work does not depend on preparing this laboratory.

Before a future authorized run, name the exact allowed effect. A service query,
an inert service installation and replacing W32Time are different scopes. Begin
with inert fixtures that discriminate the unresolved mechanism without actual
clock changes. Admit the exact tools, artifacts and staging path through the
existing project process. Guest execution does not protect a host-side compiler,
build script, installer, staging directory or scanner action from its own effects.
Never retry a held artifact under a new filename or invocation to evade the hold.

## Choose the smallest sufficient environment

This is an evidence-selection table, not a claim that the application already
implements these capabilities. Use actual installed guest capabilities and the
named CHECK owner to decide whether a proposed row is sufficient.

| Question | Potential Sandbox observation | Remaining limit or later environment |
| --- | --- | --- |
| Portable algorithm or parser contract | Usually unnecessary; use the existing admitted portable lane | Sandbox preparation must not block independent Core work |
| Win32/FFI API behavior | Bounded native execution on the identified guest build | Cross-build success alone remains insufficient; guest component availability must be observed |
| SCM configuration and process tokens | Inert service, configuration readback, fresh-process token and denied-operation controls | Does not establish production provisioning ownership or all Windows policies |
| Named-pipe ACL or caller identity | Real guest principals and allowed/denied clients | One administrative test cannot prove ordinary-user isolation |
| MSIX install/remove mechanics | The exact local package and admitted guest deployment route | Does not prove Store acquisition, approval, update delivery or all-user lifecycle |
| UI behavior | Explicit guest target, observed UI state and a supported interactive session | Full accessibility/display requirements follow their existing phase; unavailable UI infrastructure is not a pass |
| Sustained resource use | A bounded run with declared virtualization and host-load conditions | Not an uncontended hardware benchmark or proof of battery/Modern Standby behavior |
| Persistent baselines or multiple OS builds | Recreate and bootstrap only the current host-derived guest | Use a separately selected persistent VM or other suitable test machine for checkpoint/OS matrices |
| Physical clocks, device timing or hardware security | Limited guest API evidence | Qualify on appropriate hardware/environment for the exact property |

Microsoft documents the same host OS build and a single simultaneous Sandbox;
the guest lacks the Store app and support for installing optional Windows
features through the Features UI. Those limits make a full Windows environment
necessary for some release questions, not for every service experiment. [N1]

## Services and security principals

SCM operations have specific access requirements; creating a service requires
`SC_MANAGER_CREATE_SERVICE`. A successful setup under a highly privileged guest
token is evidence only for that setup path. The native CLI offers `System` and
`ExistingLogin`, but neither label alone establishes NevroTick's intended token,
integrity level, service SID or object permissions. Inspect the effective process
and use independent permitted and denied controls. [N2, N3]

Use guest `System` only when an already-authorized setup step needs it. Then
start the relevant component under its intended account and restriction policy.
Test ordinary-user behavior using a demonstrably ordinary-user token. Do not
run the network engine or all negative tests as System simply because that path
is convenient. System's profile and session differ from those of an interactive
user; package registration, environment variables and per-user files can differ.
[N3, N4]

Before naming a service result successful, retain its service identity, process
identity, effective account/privileges, start mode, relevant security descriptor,
configuration transitions, actual completion and independent application
readiness. A successful start request or configuration setter is not that
complete observation. Preserve the provisioning and recovery questions in
[MSIX §7.4](../../../../../docs/plans/windows-msix-store.md#74-service-provisioning-and-independent-removal-research).

Relevant ownership remains W11-W05 with CHECK-1/2/15/16 for provisioning and
authority, and CHECK-5/12/13/23/27 for lifecycle/recovery. Activation and mitigation
work also reaches W11-W10 and CHECK-16/26/27. These are routing reminders; the
canonical acceptance catalog defines each requirement.

## MSIX and Store boundaries

Service-bearing MSIX packages require administrative installation. Local
sideloading, developer registration and actual Store delivery are distinct
experiments. A successfully registered app inside a winapp-prepared guest does
not prove the final package's signature, immutable-content integrity, publisher
identity, servicing behavior or Store eligibility. [N5]

Keep an experiment's package bytes, dependency packages, architecture, certificate
provenance, deployment command, principal and observed package identity in its
receipt. If a local test certificate is needed, its import belongs only inside
the owned guest and the admitted test scope. Never import it into the developer
host's trust store as a convenience. Test signing is not Microsoft signing;
final Store-delivered artifact checks remain separate under the signing guide.

The current ordinary-utility custom-action restriction, unresolved independent
removal recovery and service-account/provisioning questions remain exactly those
recorded in the MSIX plan. Sandbox availability neither fixes nor waives them.
Keep CHECK-18/23/24 with their existing release/lifecycle owners. David retains
the actual Store submission and release decisions; no account login or upload
follows from this skill.

## Clock and timing evidence

Before interpreting any time result, identify the guest's active time providers,
relevant services and virtualization effects. Hyper-V documents host-to-guest
time synchronization, including startup/restore handling and samples used by
Windows Time. That is a confounder to investigate; it is not proof that Sandbox
exposes all configurable Hyper-V VM controls. [N6, N7]

Separate three questions: whether a clock API accepted a call, whether the
intended controller exclusively owned corrections, and whether measured time
accuracy meets the product requirement. A successful API call answers only the
first. Guest restart, host suspension, resource contention and VM clock behavior
can invalidate a measurement without demonstrating a NevroTick defect.

Do not alter the host clock, host W32Time, integration services or an undocumented
Sandbox utility VM to obtain a convenient result. Disabling a guest time-provider
setting is not evidence that every boot/resume correction has disappeared. If
the required controller isolation cannot be established through supported,
authorized operations, mark the timing row incomplete and use a suitable later
environment. Retain all clock-domain and ownership contracts from the architecture.

Use controlled peers for routine protocol observations; no public NTP traffic
is implied. Changing the offline profile to a network-enabled experiment needs
the existing task's actual network scope and a new recorded profile. Hardware
timestamp provenance, physical oscillator behavior, Modern Standby, TPM/VBS
properties and production security policy require their own matching evidence.

## Cleanup and recovery

Observe the product before destroying the guest. Service stop acknowledgement,
payload deletion, handle completion, journal lifetime and restored availability
have different owners and order. An external observer records transitions; it
must not perform the recovery that the product is supposed to demonstrate.

Guest termination is laboratory cleanup. It cannot establish successful product
uninstall, correct Reset behavior, W32Time restoration or absence of leaked
resources while the product was alive. A fresh guest starts from a clean state
and can conceal precisely the defect being tested. The running/stopped-service
removal pair and inert journal described in the MSIX plan remain the first
bounded lifecycle discriminator after admission; do not launch the full release
matrix simply because the environment is disposable.

## Run receipt

Append the smallest sufficient receipt to the existing task or its already owned
evidence location. Avoid a second task tree. Redact usernames, machine names,
private paths, credentials and unrelated desktop content before public commits.

| Field | Retain |
| --- | --- |
| Scope | Task/CHECK owner, explicit allowed effects, independent expected result, healthy and adverse cases |
| Candidate | Commit plus relevant dirty bytes, artifact/dependency/tool identities, profile and guest bootstrap hashes |
| Environment | Windows build, Sandbox/client/CLI versions, owned run and Sandbox IDs, architecture, effective principal/policy |
| Resources | Observed preflight, limits, deadline, concurrency, host-load caveats, consumed budget |
| Execution | Exact host-control and guest-payload commands, host dispatch status, guest exit, actual selected/executed case counts |
| Outcome | Independent observations, missing/skipped/failed cases, uncertainty, logs, completion marker and exported-file identities |
| Lifecycle | Product cleanup observed before teardown, guest stop result, retained output, unexpected surviving state |
| Disposition | What this proves, what remains incomplete, owner and next permitted action |

A run marker ties output to a case but does not authenticate it against a
malicious guest. Exported self-reports and artifact hashes are not substitutes
for an independent oracle. Preserve infrastructure failures separately from
application failures and keep unsuccessful attempts in the record.

## Sources

- **N1:** [Windows Sandbox FAQ](https://learn.microsoft.com/windows/security/application-security/application-isolation/windows-sandbox/windows-sandbox-faq).
- **N2:** [Service security and access rights](https://learn.microsoft.com/windows/win32/services/service-security-and-access-rights).
- **N3:** [Windows Sandbox CLI](https://learn.microsoft.com/windows/security/application-security/application-isolation/windows-sandbox/windows-sandbox-cli).
- **N4:** [LocalSystem account](https://learn.microsoft.com/windows/win32/services/localsystem-account).
- **N5:** [MSIX packages containing services](https://learn.microsoft.com/windows/msix/packaging-tool/convert-an-installer-with-services).
- **N6:** [Hyper-V integration services](https://learn.microsoft.com/virtualization/hyper-v-on-windows/reference/integration-services).
- **N7:** [Windows time improvements: Hyper-V](https://learn.microsoft.com/windows-server/networking/windows-time-service/windows-server-2016-improvements#hyper-v).

The linked project documents own NevroTick policy. Microsoft sources describe
platform behavior; their composition into the proposed lab workflow is project
guidance and still requires actual candidate-specific observations.
