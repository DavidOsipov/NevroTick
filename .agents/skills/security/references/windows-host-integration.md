# Windows event and network-policy integration

Read relevant sections for Windows Event Log, optional ETW or installer-owned
Firewall/WFP changes. Architecture sections 12, 15.3 and 18.6 own decisions;
CHECK-1/13/15/17/18/21/23/26/27 select evidence. Candidate integrations/procedures
prove no implemented provider, channel, filter or application.

Installation, service operations, privileged policy changes/network probes require
existing task authority; this reference grants none or new approval rounds. Never
request `SeAuditPrivilege`, write arbitrary Security-log events, add kernel drivers,
disable host protections or add antivirus exclusions to pass checks.

## Establish the owner and candidate

Before API selection identify actual component, service SID/account/token,
installer transaction and protected effect. Record exact Windows build,
architecture, package/payload hashes, configuration, active host policy and initial
product-owned state. Real enforcement claims follow [native proof](native-proof.md);
source review/simulated failures remain narrower.

Separate installer registration, runtime provider registration, channel policy,
consumer/export authority and network enforcement owners. Neither `windows`
bindings nor diagnostic crates supply policies. Add no dependency/general
firewall/logging engine merely for this reference; narrow adapter/explicit features
need separate admission under architecture section 18.6.

## Event Log

1. Put the fixed provider GUID/name, event descriptors/schema and protected
   manifest/message/resource paths in the complete package inventory. The
   installer owns installation, upgrade, repair and removal of that registration;
   ordinary UI/service callers cannot substitute manifests, resource paths or
   channel names. Distinguish manifest installation from the process's runtime
   `EventRegister` lifetime. Query installed publisher metadata and render a
   known event against the exact installed resources; a successful registration
   call does not prove message-resource parity. Microsoft documents separate
   manifest, publisher, query and archive operations in
   [wevtutil](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/wevtutil).
2. Choose a protected product Operational/Admin channel or explicitly justify
   the shared Application channel. Record an identity/operation matrix for
   legitimate service publication, unauthorized publication/provider
   registration, channel reading, clearing, configuration and export. Include
   the ordinary user, service identity, intended reader and administrator roles;
   inspect effective permissions and exercise the required allow/deny cases.
   Do not infer writer isolation from an SDDL string: for Application/System
   isolation the channel `access` descriptor controls read access and ignores
   write permissions. Custom isolation permits channel-write control but uses
   a separate ETW session, so assess that resource cost and the actual writer
   boundary. A deliberately shared channel retains its documented limitations.
   See [ChannelType](https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-channeltype-complextype).
3. Treat channel access, provider registration/enabling permissions and ETW
   session/file access separately. A provider GUID or payload's component field
   alone is not proof that the trusted service wrote it. Observe whether an
   unauthorized process can register/publish the selected identity and how
   consumers distinguish any remaining spoofing exposure.
   [EventAccessControl](https://learn.microsoft.com/en-us/windows/win32/api/evntcons/nf-evntcons-eventaccesscontrol)
   documents separate provider/session rights; do not grant broad group
   membership to avoid selecting the necessary rights.
4. Preserve stable event IDs and versioned field meanings for lifecycle,
   denied privileged operations, policy changes, trust failures, correction
   decisions and recovery. Include component, policy/ownership generation,
   UTC with its time-quality label, a monotonic sequence with a restart/boot
   discriminator, and operation/request correlation where useful. Explicitly
   describe sequence scope; independent process counters do not establish a
   total order. A repaired or wildly wrong wall clock must not manufacture
   trustworthy UTC or reorder causal decisions. Never reuse an ID for a
   different meaning or substitute free-form secret-bearing error text.
5. Bound record fields, event rate, recent-event memory and persisted channel
   retention separately. Section 12's proposed diagnostic budget is not proof
   of the OS channel's actual maximum size/retention. Test reached sink failures,
   disabled/missing channels, saturation and repeated denials with a useful
   successful event control. Lost-event behavior stays visible and bounded;
   denial and safe clock cleanup cannot depend on logging succeeding.
6. Verify export source access and destination/file ACLs independently of
   read/clear rights. Keep privileged services on fixed bounded snapshots;
   the UI owns user-selected redacted support exports and preview. A raw EVTX
   export copies stored events rather than applying NevroTick field redaction;
   authorize and label that artifact separately. For a selected native export,
   account for `EvtExportLog`'s absolute, initially nonexistent destination and
   rendering/resource requirements; do not accept arbitrary service IPC paths
   or treat export success as permission to clear a channel. See
   [EvtExportLog](https://learn.microsoft.com/en-us/windows/win32/api/winevt/nf-winevt-evtexportlog).

## Optional ETW

Keep optional profiling/diagnostic ETW disabled until measured need. ETW does not
replace persisted operational events. Fix provider identity, levels/keywords and
allowed fields; exclude packet dumps, NTS cookies/keys, TLS material and approval
tokens from payloads/formatting. Inspect synthetic secrets through enabled,
filtered/failure paths, retained traces and selected real-time consumers.

Measure disabled overhead at real call site; guard expensive allocation/formatting
with selected framework's enabled check. `EventEnabled` is conservative: true
proves no consumer delivery. See
[EventEnabled](https://learn.microsoft.com/en-us/windows/win32/api/evntprov/nf-evntprov-eventenabled).

Qualify registration failure, enable/disable and level/keyword filtering,
controller/consumer permissions, bounded loss/backpressure, service stop/restart
and callback/unregistration ownership. Unregister before component unload and
keep callback state valid through teardown; generic `CloseHandle` ownership is
not a provider-registration destructor. See
[EventUnregister](https://learn.microsoft.com/en-us/windows/win32/api/evntprov/nf-evntprov-eventunregister).
Check trace-file ACLs separately from real-time consume/control rights using
[EventAccessControl](https://learn.microsoft.com/en-us/windows/win32/api/evntcons/nf-evntcons-eventaccesscontrol).
Clean up only the task's owned session and artifacts; preserve pre-existing
traces and any evidence the task requires retaining.

## Firewall and WFP

1. Write a traffic/owner matrix before policy changes: service SID and installed
   executable, direction, protocol, local/remote addresses and ports, IPv4/IPv6,
   profile/interface, destination-policy revision and expected effect. Engine
   destination admission remains application policy; broker external networking
   is denied. A virtual account or service SID alone does not remove networking.
   Verify service rule scope against the installed service SID type and actual
   own-process service. Microsoft documents the service-SID requirement in
   [Firewall configuration](https://learn.microsoft.com/en-us/windows/security/operating-system-security/network-security/windows-firewall/configure).
2. Capture effective profile defaults, local/managed policy merge and conflicting
   rules. Default outbound allow plus a few allow rules is not an allowlist;
   explicit Firewall blocks take precedence over conflicting allows. Do not
   present blanket-block-plus-allow-exceptions as a working recipe. Select and
   qualify a product-scoped mechanism without changing machine-wide default
   policy or overriding administrator/GPO decisions. See
   [Firewall rule precedence](https://learn.microsoft.com/en-us/windows/security/operating-system-security/network-security/windows-firewall/rules).
   If WFP is selected, identify its layer/sublayer, conditions, actions and
   arbitration; Firewall rule intuition alone does not prove WFP behavior.
   Do not introduce hard permits to bypass host policy. See
   [WFP filter arbitration](https://learn.microsoft.com/en-us/windows/win32/fwp/filter-arbitration).
3. Inventory DNS resolution, NTS-KE, negotiated NTP endpoints/ports and every
   direct or delegated certificate/intermediate/root/revocation retrieval.
   Identify the actual socket owner: a rule on the engine cannot be assumed
   to constrain a delegated Windows service. Cover A/AAAA changes, IPv4/IPv6,
   reconnect/profile/VPN changes, negotiated destinations, permitted redirect
   hops and cache/offline paths. Resolve allowed resource/protocol policy and
   finite work/deadlines under architecture section 18.7.2; do not disable the
   system root updater or silently authorize plaintext NTP when retrieval fails.
4. Separate declared intent, configured effective rules and observed enforcement.
   In an authorized Windows fixture, use reachable controlled endpoints and
   actual final service identities to pair permitted engine traffic with denied
   foreign destinations/protocols/ports and broker external traffic on both IP
   families. Preserve permitted local IPC. Record the reached socket/filter
   boundary, process/service attribution, result and independent peer/native
   observations. An unreachable server, DNS failure, app rejection before a
   socket or absent packet alone is not proof of OS blocking. A reachable
   allowed neighbor prevents deny-all or broken routing from passing.
   If the production path cannot reach a selected native probe, retain that
   gap or qualify a separate test adapter; its results have that narrower scope.
5. Installer policy objects need exact product-owned rule/provider/filter keys,
   protected ownership records, prior values and generation. Select lifetimes
   deliberately: dynamic WFP objects disappear when their session ends, whereas
   persistent objects need explicit removal. A process crash must not silently
   remove a restriction the design requires during recovery. WFP operation
   failure does not itself abort an explicit transaction; handle failure and
   abort/commit deliberately. Verify startup, stop, crash, reboot and maintenance
   outcomes for the chosen mechanism. See
   [WFP object management](https://learn.microsoft.com/en-us/windows/win32/fwp/object-management).

## Evidence, faults and restoration

Retain immutable candidate receipts that bind package/payload hashes and signer
evidence to provider manifest/resource bytes, installed publisher/resource
paths, channel isolation/ACL/retention, stable event schema and observed rendered
records. For ETW add enabled/disabled filters, session owner/access, callback
lifetime and measured overhead/loss. For network policy add owned object keys,
service identity, effective profiles/rules, direct/delegated retrieval owners,
reachable allow/deny observations and the restored final state. Protect/redact
receipts without losing the identity needed to reproduce the claim.

Select reached-boundary faults: denied registration/publication, missing resources,
full/disabled logs, ETW registration/stop failure, policy API failure/partial
transaction, interrupted upgrade/repair and maintenance-time administrator policy
changes. Observe bounded failure/legitimate recovery; assert cleanup before fixture
teardown hides handle/session/filter leaks. Logging failure must preserve safe
broker cleanup.

Compare saved/current ownership/generation before restoration. Remove/restore only
attributable product-owned provider/channel/session/rule changes; preserve successor
versions, later administrator changes, host policy and retained logs per declared
retention. Never reset all profiles, delete broad display-name rule matches, clear
unrelated logs or reapply whole-machine policy snapshots. Report ambiguity/diagnose
blocked operation; never widen privileges/policy.

Report chosen owner/mechanism, exact candidate, observed allowed/forbidden effects,
faults/cleanup and unexecuted boundaries. Pack checks, rendered test events or
policy dumps alone cannot qualify installed service isolation, release readiness
or ASVS analysis/alerting coverage.
