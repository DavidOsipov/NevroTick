# Evidence at Windows enforcement boundaries

Read only for Windows enforcement claims. Architecture sections 14.9 and 20 define planned gates. Record exact Windows build/architecture, binaries, feature/configuration, accounts/tokens, initial ownership and exercised entrypoint; preserve matching evidence.

| Claim | Required observation | Narrower evidence |
| --- | --- | --- |
| Engine cannot set time/inject broker | Actual engine token, denied prohibited API/object rights, successful legitimate broker control. | Mock tokens/account strings prove no OS enforcement. |
| Pipe identity/access | Genuine/hostile client/server processes, effective DACL/rights, identification, restart/squatter handling. | Handler tests prove substituted dispatch only. |
| Corrections obey ownership/budgets | Broker API/result trajectory, current ownership/generation, cumulative accounting across repeated requests/restarts. | Requested values/one success return do not measure applied correction. |
| Stop/crash/suspend is safe | Controlled lifecycle, stale-work rejection, actual cancellation/completion, bounded retained rate, independent observations. | Drop/timeout proves neither worker exit nor global-rate reset. |
| Files/policy resist replacement | Selected Windows path/handle/ACL behavior, owned reparse/open-handle cases, observed forbidden effects. | Unix paths prove no Windows sharing/reparse behavior. |
| MSI rollback/restoration | Exact full package, original state/journal, reached failure stages, reboot/repair/upgrade/uninstall, restored ownership. | Manifests/ordinary installs cannot qualify all maintenance paths. |

Clock experiments require disposable Windows VMs under canonical bounded plan: known initial controller/ownership, maximum raw API value/phase/rate, count/duration, independent clock observations and stop/recovery. Never explore on user's workstation. Unknown ownership forbids global reset. Preserve administrator/host races and precise-adjustment crash limits.

Pair denials with passing valid neighbors; reject-all must fail. Prove intended native boundary reached before interpreting failures. Separate source hypotheses from actual causal observations. Record skipped/unavailable boundaries; pure/source/pack work remains useful without becoming native qualification.
