# Windows Time handover — source research for the HTML prototype

Research date: 5 November 2026. Scope: read-only documentation research and UI contract; no Windows host was inspected and no service, registry, clock, task or OS setting was changed. Microsoft pages below were fetched successfully with TLS verification through the configured environment proxy. Runtime network policy was unrestricted/enforced; no credentials were required.

## Recommendation

Provide an explicit **Use Windows Time** transaction alongside control of the two NevroTick services and their startup configuration. It must describe a cooperative handover, with distinct results for NevroTick quiescence, clock cleanup/release, Windows Time configuration, Windows Time service status and Windows synchronization evidence. A stopped NevroTick process is not proof of cleanup; a running W32Time service is not proof of synchronization.

Keep all states and operations simulated in this HTML prototype. The real helper/SCM/clock implementation and its Windows qualification are outside the authorized scope.

## What Microsoft documents

| Question | Documented evidence and implication |
| --- | --- |
| Must W32Time always be Automatic and Running? | No. Microsoft documents Manual/trigger-start and Stopped as the workgroup configuration, domain-join start and domain-leave stop triggers, and scheduled synchronization that can start the service. Changing Manual to Automatic alone can still leave it stopped. Its workarounds alter triggers/tasks and are environment-specific, not a universal restoration recipe. Preserve/query these settings; do not present a trigger-start service as broken solely because it is stopped. [M1] |
| Can a disabled W32Time simply be started? | `SERVICE_DISABLED` means attempts to start it fail with `ERROR_SERVICE_DISABLED`. A handover may therefore need a permitted start-configuration change before a start request. This does not select Automatic as the universal replacement. [M3, M5] |
| How are actual service prerequisites discovered? | `QueryServiceConfigW` returns configured service information; `QUERY_SERVICE_CONFIGW.lpDependencies` contains service or load-order-group names that must start first. An empty value means no configured dependencies. `Get-Service -Name W32Time -RequiredServices` is a documented read-only convenience for service prerequisites. `-DependentServices` asks the reverse question. No fixed W32Time dependency list has been established by this research. [M3, M4, M8] |
| Must all prerequisite services be switched to Automatic? | No such requirement follows. Microsoft explicitly says an auto-start service can cause its demand-start prerequisite to start automatically. A dependency declaration establishes ordering/availability, not permission to change that service's configuration. A disabled/missing/failing prerequisite needs a specific diagnosed result; W32Time start can report dependency-deleted/failure errors. [M5, M9] |
| Does a successful start request prove Running? | No. `StartService` returns after creation of the service's `ServiceMain` thread; its default status is `SERVICE_START_PENDING`. Track subsequent service status. Even confirmed Running supplies no time-source or last-sync evidence. [M5] |
| Why preserve domain and peer settings? | Most domain members use NT5DS/domain hierarchy. `w32tm /query /configuration` reports runtime configuration and the source of settings; `/source`, `/peers`, and `/status` report separate information. Microsoft says Group Policy values overwrite existing W32Time registry values. Preserve existing effective policy rather than selecting a public peer or applying `domhier` blindly. [M2] |
| Is unregister/register a restoration shortcut? | No. Microsoft documents `/unregister` as removing all configuration and `/register` as adding defaults. This would violate preservation of the user's settings and the NevroTick architecture. [M2; local §11.1] |
| Does stopping NevroTick prevent a delayed recovery restart? | No. Microsoft says queued SCM recovery restarts cannot be canceled, and the service must be explicitly disabled to prevent the queued restart. Local §11 additionally requires durable maintenance/startup inhibition and current-ownership admission. Apply this only to the product's own services. [M6] |
| What does releasing the clock actuator mean? | `SetSystemTimeAdjustmentPrecise(..., TRUE)` selects Windows internal synchronization and ignores the numeric adjustment; `FALSE` leaves application-programmed control. The getter exposes mode and value; it does not identify a current owner. This is privileged clock work, not a UI effect, and release is not established by stopping a service or matching a historical numeric rate. [M7; local §10.1] |

## What must be queried on the real host

Future native work must read actual W32Time installation/state/start type, delayed start/triggers, declared prerequisites and their state/configuration; domain membership, effective time policy/provider/peer settings and their policy source; the protected NevroTick ownership generation/journal, both product services, queued-restart protection, and current clock adjustment mode/value. Configuration stored for a future service run and the configuration currently effective in a running service can differ. [M3]

Example **read-only** observations for future Windows qualification (not executed here):

```powershell
Get-Service -Name W32Time
Get-Service -Name W32Time -RequiredServices
sc.exe qtriggerinfo w32time
w32tm /query /configuration /verbose
w32tm /query /source
w32tm /query /peers
w32tm /query /status /verbose
```

`QueryServiceConfigW` plus `QueryServiceConfig2W` are the supported native paths for configuration and optional delayed-start/trigger data. Resolve transitive service prerequisites when applicable; distinguish load-order groups, for which Microsoft documents that at least one member must run after attempting the group, from individual required services. PowerShell prerequisites do not prove all runtime needs: DNS/network availability, domain authentication, selected time providers and reachability remain separate checks. [M2–M4, M10, M11]

Do not invent a dependency result for RPC, RPC Endpoint Mapper, DCOM, Netlogon, networking or other Windows services. Some facilities can be relevant to a selected provider/domain environment without being a direct SCM prerequisite of W32Time. If a prerequisite is disabled, present its actual discovered identity and the proposed bounded change, or a blocked/admin-managed result. Do not offer “enable all Windows services.” This research has not established a safe universal configuration repair.

## UI transaction and result boundaries

1. **Review:** show the intended owner, both NevroTick services and startup changes, W32Time's observed configuration, preserved Windows time policy, and only actual prerequisite changes established by preflight. Require the real operation's administrator authorization; the prototype uses a labelled sample response.
2. **Quiesce NevroTick:** inhibit new clock work and automatic/recovery reactivation of both product services. Cancel attributable transient corrections only while current ownership is still confirmed. Persist and read back the release/cleanup result before activating W32Time. Disabling startup is not itself a stop or cleanup result.
3. **Settle NevroTick stop/start configuration:** confirm both services stopped and the requested startup configuration. A partial stop or uncertain cleanup remains incomplete. Preserve protected recovery ability until the transaction has a durable result.
4. **Restore/enable Windows Time:** respect a later administrator/GPO/successor decision. Apply the recorded permitted W32Time configuration or the explicitly reviewed replacement if the user's goal is to enable a service that was already disabled before NevroTick. Preserve peer/domain/provider settings and triggers unless a particular change was reviewed. Enable only actual necessary prerequisites for which this operation has bounded authority; otherwise report the blocker.
5. **Verify separately:** distinguish request accepted, W32Time starting/running/stopped-by-trigger/failed, release confirmed, and a newly observed Windows time-source/sync result. The first completion should say **Windows Time enabled; synchronization not yet verified** unless fresh evidence proves more. Do not carry over NevroTick NTS agreement or label every Windows Time source unauthenticated: domain time has different authentication semantics. [M2, M11]

On ambiguity or loss of acknowledgment, expose **Outcome unknown — reconcile current owner** and block another ownership-changing command. On dependency or policy failure after release, report **NevroTick disabled; Windows Time unavailable** with the actual reason. Do not silently reactivate NevroTick or declare synchronized. A deliberate rollback is another ownership-checked transaction.

Keep startup preference separate from current owner. “Start NevroTick with Windows” controls future start configuration, not current synchronization, cleanup, or Windows ownership. **Fully disable NevroTick** requires both services stopped, startup disabled/inhibited, and a resolved release result. Re-enabling product startup alone grants no clock ownership. Future **Use NevroTick** must quiesce Windows Time and establish one new product generation before clock writes.

## Local contract and pre-implementation prototype gap

At the pre-implementation baseline `351c1f2`, the audited prototype `src/model.js:293–352` modelled Start/Stop/Restart, says Windows Time is not enabled after Stop, and correctly refuses to equate stopped services with cleanup. It had no persistent-startup or explicit handover model. This iteration extends those distinctions rather than redefining Stop as proof of release; see REPORT-RU.md for the implemented HTML behavior.

Canonical [architecture §10.1/10.4](https://github.com/DavidOsipov/NevroTick/blob/8a66af924c509e898a9bab9bb3282f11b24fba93/docs/windows-time-architecture.md#101-read-and-correction-contracts) requires current ownership before every write/reset, adjustment readback, and honest crash-cleanup uncertainty. [§11](https://github.com/DavidOsipov/NevroTick/blob/8a66af924c509e898a9bab9bb3282f11b24fba93/docs/windows-time-architecture.md#11-w32time-ownership-and-installation-lifecycle) requires preserved W32Time state, no unregister/re-register, administrator/GPO guards, rollback stages, queued-restart quiescence, and an elevated explicit release transaction. [§19.2](https://github.com/DavidOsipov/NevroTick/blob/8a66af924c509e898a9bab9bb3282f11b24fba93/docs/windows-time-architecture.md#192-lifecycle-and-user-expectations) requires one product lifecycle and durable restoration before deleting the last recovery mechanism. CHECK-13/23 qualification remains unexecuted; source research and a UI fixture do not pass it.

Unresolved native facts: exact service identities and prerequisite graph on a supported host; the accepted W32Time configuration if its captured pre-install state was already Disabled; actual policy authority; trigger/task behavior on the tested Windows build; and measured cleanup, interruption/rollback and synchronization evidence. These are explicit future implementation/qualification inputs, not reasons to omit the requested UI preview.

## Official Microsoft sources

- **M1 — Windows Time service doesn't start automatically on a workgroup computer**, supported Windows clients/servers, KB 2385818: https://learn.microsoft.com/en-us/troubleshoot/windows-client/active-directory/w32time-not-start-on-workgroup
- **M2 — Windows Time service tools and settings**: https://learn.microsoft.com/en-us/windows-server/networking/windows-time-service/windows-time-service-tools-and-settings
- **M3 — QueryServiceConfigW**, including future-run/current-run caveat: https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-queryserviceconfigw
- **M4 — QUERY_SERVICE_CONFIGW**, start types and `lpDependencies`: https://learn.microsoft.com/en-us/windows/win32/api/winsvc/ns-winsvc-query_service_configw
- **M5 — StartServiceW**, completion and failure boundaries: https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-startservicew
- **M6 — ChangeServiceConfig2W**, queued SCM recovery restart warning: https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-changeserviceconfig2w
- **M7 — Precise system time adjustment setter/getter**: https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-setsystemtimeadjustmentprecise and https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemtimeadjustmentprecise
- **M8 — Get-Service**, `RequiredServices` versus `DependentServices`: https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.management/get-service?view=powershell-7.5
- **M9 — Automatically Starting Services**, demand-start prerequisites need not become auto-start: https://learn.microsoft.com/en-us/windows/win32/services/automatically-starting-services
- **M10 — QueryServiceConfig2W**, delayed start/trigger query selectors: https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-queryserviceconfig2w
- **M11 — How the Windows Time service works**, time providers and domain time authentication: https://learn.microsoft.com/en-us/windows-server/networking/windows-time-service/how-the-windows-time-service-works
