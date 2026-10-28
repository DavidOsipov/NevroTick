# Evidence at Windows enforcement boundaries

Read only for a claim that depends on Windows enforcement. Architecture sections 14.9 and 20 define the relevant planned gates. Record exact Windows build/architecture, binaries, feature/configuration, accounts/tokens, initial ownership and exercised entrypoint; preserve evidence for that state.

| Claim | Required observation | Narrower evidence |
| --- | --- | --- |
| Engine cannot set time/inject broker | Actual engine token, denied prohibited API/object rights and successful legitimate broker control. | A mock token or service account string tests no OS enforcement. |
| Pipe identity/access | Genuine/hostile client and server processes, effective DACL/rights, identification and restart/squatter handling. | Handler tests establish only the substituted dispatch path. |
| Corrections obey ownership/budgets | Broker-side API/result trajectory, current ownership/generation and cumulative accounting under repeated requests and restarts. | A requested value or one successful return is not measured applied correction. |
| Stop/crash/suspend is safe | Controlled lifecycle, stale work rejection, actual cancellation/completion, bounded retained rate and independent observations. | Drop/timeout does not prove worker exit or reset of a global rate. |
| Files/policy resist replacement | Selected Windows path/handle/ACL behavior with owned reparse/open-handle cases and forbidden effects observed. | A Unix path test proves no Windows sharing/reparse behavior. |
| MSI rollback/restoration | Exact full package, original state/journal, reached failure stages, reboot/repair/upgrade/uninstall and restored ownership state. | A manifest or ordinary successful install does not qualify all maintenance paths. |

Clock-changing experiments run only in disposable Windows VMs under the canonical bounded plan: known initial controller/ownership, maximum raw API value/phase/rate, count/duration, independent clock observations and stop/recovery procedure. Do not use the user's real workstation as an exploratory time test bed. Unknown ownership means no global reset. Keep administrator/host races and precise-adjustment crash limits explicit.

A passing valid neighboring case is required alongside denied cases so reject-all implementations cannot pass. Prove the test reaches the intended native boundary before interpreting a failure. Report source hypotheses separately from actual causal observations. Record skipped/unavailable boundaries; do useful pure/source/pack work without promoting it to native qualification.
