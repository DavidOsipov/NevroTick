# Windows service metadata field catalogue

**Reviewed:** 7 November 2026, Asia/Tbilisi.<br>
**Status:** supporting source/schema research, adapted from the owner-supplied
Windows Service Metadata Handoff at baseline
`df6e28a4641f2f046373a9c5640aa60736716a22`.
The [canonical metadata contract](../plans/windows-package-metadata.md#12-service-and-diagnostics-metadata-refinement--7-november-2026)
owns applicability, values and the existing implementation handoff. This reference
retains detailed fields once; it is neither a second plan nor a production record.

Load only the applicable appendix:

- [A: SCM and packaged services](#appendix-a-scm-and-packaged-services).
- [B: PE, manifests, packages, signing and distribution](#appendix-b-pe-manifests-packages-signing-and-distribution).
- [C: Diagnostics and performance counters](#appendix-c-diagnostics-and-performance-counters).

Completeness is bounded by each named structure/schema ledger, not all Windows
metadata or every SDK version. Counts include internal, reserved, behavioral,
security and runtime members; do not sum them as useful public product facts.
Source fields are not final product dispositions. For each selected component,
surface and admitted locale, the metadata owner still records **populated**,
**derived**, **pending with owner**, or **not applicable with reason**. Unknown
technical service names, provider GUIDs, Store identities, versions and resource
paths stay pending; the proposed wording below is not approved shipping text.

Separate reviewers assessed SCM, artifact/distribution and diagnostics scopes
against current repository authority and primary Microsoft documentation. The
integration corrected ambiguous required-privileges update wording and the
MSIX Properties ledger's declaration/reference wording. Diagnostics structural
review matched 187 XML members against 37 published Microsoft schema pages;
SCM/artifact review rechecked selected consequential sources and pinned schemas.
These checks establish neither the full selected SDK grammar nor native behavior.
The review
qualifies research rather than emitted resources, native Windows behavior or
release readiness. Exact selected SDK/compiler, signatures, installed resources,
localization and delivered bytes still require their existing independent gates.
No package, account, privilege, channel, counter set, dependency or collector is
selected by catalogue presence. Preserve the complete MSIX direction, conditional
MSI references, current product queue, Windows/fuzz/storage/UI holds and post-V1
formal deferral. W11-W10/P6, W11-W05 and W11-W08 retain their existing owners.

## Appendix A: SCM and packaged services

Research snapshot: 7 November 2026, `df6e28a4641f2f046373a9c5640aa60736716a22`. Source inventory; no installed service behavior qualified.

### A.1. Scope and completeness

**Complete member enumeration against the named published contracts:** `CreateServiceW` semantic values, `QUERY_SERVICE_CONFIGW`, all ten info levels listed by current `ChangeServiceConfig2W`/`QueryServiceConfig2W`, their top-level and nested structures, and `SERVICE_STATUS`/`SERVICE_STATUS_PROCESS`. XML scope covers the complete published `desktop6:Service` descendant graph, including the documented `desktop10:PredefinedTriggerEvents` data branch, plus a direct-member cross-reference for `desktop7:Service`.

**Counts:** 10 base values + 27 Config2 structure members + 9 deduplicated runtime members = **46 native rows**. XML adds **42 attribute/child-slot rows**, for **88 canonical rows**. Three non-metadata `CreateServiceW` arguments are separately accounted for. Ten info-level selector rows, source records, and scope-ledger rows are not extra metadata fields. The 88 rows include behavior, security, runtime, container, count, and reserved members; they are not 88 descriptive properties to fill.

**Boundaries:** no claim to every Windows metadata surface, undocumented SCM levels, private registry schema, or every SDK enum constant. Service-object security descriptors/SDDL, package-wide identity/extension/application attributes, PE resources, signatures, event-publisher schemas, and process policy belong to separate inventories. The desktop7 cross-reference does not recursively enumerate desktop7 descendant types. Published XML documentation is sufficient for member inventory; exact XSD particle/lexical validation and installed behavior remain open qualification work.

### A.2. Recommended NevroTick profile

SCM's principal human-facing descriptive fields are **display name and description**. Its technical service name is identity. Extra operational fields are not harmless descriptive enrichment. Service display names surface in Services and name mapping; descriptions surface in the Services snap-in. [SCM-S06] [SCM-S22]

The scoped repository review found the engine/broker role distinction and metadata contract, but no finalized literal technical SCM names in the reviewed owner documents. Do not invent accepted identifiers such as a guessed engine/broker key name. The following strings are **proposals for owner review**, not assertions about installed or shipping behavior.

| Role / field | Proposed content or rule | Lifecycle owner / repository status |
| --- | --- | --- |
| Engine technical service name | Reuse the eventual authoritative stable service identifier; no new literal proposed. | Platform/install owner. Existing identity-preservation requirement. |
| Engine display name | NevroTick Synchronization Engine | Release metadata owner; installer projection. Proposed wording. |
| Engine description | Maintains time estimates from configured sources and submits correction requests to the NevroTick Clock Broker. | Role/architecture owner must confirm truth; release metadata owner maintains wording. Proposed wording. |
| Broker technical service name | Reuse the authoritative stable broker identifier; no version, locale, or marketing name in the identity. | Platform/install owner. Existing identity-preservation requirement. |
| Broker display name | NevroTick Clock Broker | Release metadata owner; installer projection. Proposed wording. |
| Broker description | Applies authorized system-clock corrections and enforces NevroTick clock-ownership policy. | Role/architecture owner must confirm scope; release metadata owner maintains wording. Proposed wording. |
| Localized variants | Maintain approved translations and resource mappings in the source record; literal English fallback is acceptable until localization is qualified. | Release/localization owner. New mechanism precision; not a new SCM field. |
| Behavioral/security configuration | Read and preserve the approved role-specific policy; record desired and observed values separately. | Platform/security owner and installer/elevated helper. Existing adapter contract. |
| Runtime status | Observe actual reported state and timestamps; exclude from static product-description claims. | Service lifecycle owner and independent readback. New inventory precision. |

Relevant existing surfaces: `docs/plans/windows-package-metadata.md`, `.agents/skills/implementation/maintain-windows-metadata/SKILL.md`, and `.agents/skills/implementation/implement-rust/references/windows-service-adapters.md`. The first already requires useful applicable metadata; the skill already requires source-of-truth projection and independent readback. This section adds field-level completeness, precise limits/update semantics, native-versus-package mapping, and explicit exclusions. It does not claim those projections are implemented or installed behavior has been proven.

Configuration remains owned by the installer/elevated helper. Runtime code does not gain configuration authority through this inventory. MSIX provisioning actor, rights, account mapping, and installed results remain unqualified. Existing Windows/native stop and other project validation gates remain unchanged.

### A.3. Reading the inventory

Kinds: **D** descriptive; **I** identity; **B** behavior; **S** security; **R** runtime; **T** transport, representation, count, or update/internal state. A combined kind exposes a mixed-purpose field.

Dispositions: **Recommended** means maintain truthful approved metadata; **Conditional** means use only when an existing reviewed requirement applies; **Observed-only** means read or derive it without inventing a product value; **Not applicable** means no authorable product field or no applicable mode. Conditional does not mean a required installation parameter can be omitted; it means this handoff does not choose its value.

A pointer-named member can carry a useful string, GUID, or array value. The tables inventory that value, never a memory address. Counts are derived. Schema paths are qualified **element-definition/member paths**, not fully expanded instance XPaths; referenced child definitions appear once. Corresponding status members are explicitly aliased, not described as C-language inheritance.

### A.4. Scope ledger

#### A.4.1. Native contracts

| Contract | Enumerated count / alias treatment | Source |
| --- | --- | --- |
| CreateServiceW | 13 parameters accounted: 9 query-field aliases + 1 additional identity + 3 exclusions | [SCM-S01] |
| `QUERY_SERVICE_CONFIGW` | 9 | [SCM-S02] |
| `SERVICE_FAILURE_ACTIONSW` | 5 | [SCM-S04] |
| `SC_ACTION` | 2 | [SCM-S05] |
| `SERVICE_DESCRIPTIONW` | 1 | [SCM-S06] |
| `SERVICE_DELAYED_AUTO_START_INFO` | 1 | [SCM-S07] |
| `SERVICE_FAILURE_ACTIONS_FLAG` | 1 | [SCM-S08] |
| `SERVICE_SID_INFO` | 1 | [SCM-S09] |
| `SERVICE_REQUIRED_PRIVILEGES_INFOW` | 1 | [SCM-S10] |
| `SERVICE_PRESHUTDOWN_INFO` | 1 | [SCM-S11] |
| `SERVICE_PREFERRED_NODE_INFO` | 2 | [SCM-S12] |
| `SERVICE_LAUNCH_PROTECTED_INFO` | 1 | [SCM-S13] |
| `SERVICE_TRIGGER_INFO` | 3 | [SCM-S14] |
| `SERVICE_TRIGGER` | 5 | [SCM-S15] |
| `SERVICE_TRIGGER_SPECIFIC_DATA_ITEM` | 3 | [SCM-S16] |
| `SERVICE_STATUS` | 7 (aliases of first seven runtime rows) | [SCM-S17] |
| `SERVICE_STATUS_PROCESS` | 9 (seven corresponding members + two additional) | [SCM-S18] |
| ChangeServiceConfig2W / QueryServiceConfig2W | 10 published info-level selectors; selectors excluded from field total | [SCM-S03] [SCM-S21] |

The nine `QUERY_SERVICE_CONFIGW` members map to nine creation values; `lpServiceName` adds the tenth. `lpdwTagId` is the creation output for the tag value, not an additional durable field. Config2 contributes 17 top-level members and 10 nested members: two in `SC_ACTION`, five in `SERVICE_TRIGGER`, three in `SERVICE_TRIGGER_SPECIFIC_DATA_ITEM`. Runtime aliases deduplicate seven shared status values.

#### A.4.2. XML contracts

| Qualified schema element | Enumerated members | Source |
| --- | --- | --- |
| `desktop6:Service` | 4 attributes + 2 child slots = 6 | [SCM-S26] |
| `desktop6:Dependencies` | 0 attributes + 1 child slot = 1 | [SCM-S27] |
| `desktop6:DependentService` | 1 attribute + 0 children = 1 | [SCM-S28] |
| `desktop6:TriggerEvents` | 0 attributes + 2 child slots = 2 | [SCM-S29] |
| `desktop6:TriggerCustom` | 2 attributes + 5 child slots = 7 | [SCM-S32] |
| `desktop10:PredefinedTriggerEvents` | 3 attributes + 5 child slots = 8 | [SCM-S30] |
| `desktop6:StringData` | 0 attributes + 1 child slot = 1 | [SCM-S34] |
| `desktop6:DataItem` | 1 attribute + 0 children = 1; reused by desktop10:StringData | [SCM-S35] |
| `desktop6:BinaryData` | 1 attribute + 0 children = 1 | [SCM-S36] |
| `desktop6:LevelData` | 1 attribute + 0 children = 1 | [SCM-S37] |
| `desktop6:KeywordAnyData` | 1 attribute + 0 children = 1 | [SCM-S38] |
| `desktop6:KeywordAllData` | 1 attribute + 0 children = 1 | [SCM-S39] |
| `desktop10:StringData` | 0 attributes + 1 child slot = 1 | [SCM-S30] |
| `desktop10:BinaryData` | 1 attribute + 0 children = 1 | [SCM-S30] |
| `desktop10:LevelData` | 1 attribute + 0 children = 1 | [SCM-S30] |
| `desktop10:KeywordAnyData` | 1 attribute + 0 children = 1 | [SCM-S30] |
| `desktop10:KeywordAllData` | 1 attribute + 0 children = 1 | [SCM-S30] |
| `desktop7:Service (direct contract only)` | 4 attributes + 2 child slots = 6 | [SCM-S33] |

Shared semantic meaning does not erase distinct namespace-qualified fields. Conversely, `desktop6:DataItem/@Value` is one definition reused under both string-data branches. Containers are counted as child slots, not repeated once per possible instance.

### A.5. Base configuration and identity — 10 rows

Readback uses `QueryServiceConfigW` for the nine query fields; retain the opened service's technical identity in the observation record. Creation aliases below identify the same semantic value. [SCM-S01] [SCM-S02] [SCM-S20]

| Canonical field / member | Kind | Disposition | Type, rule, source |
| --- | --- | --- | --- |
| `CreateServiceW.lpServiceName` | I | Recommended | UTF-16 name; maximum 256 characters; no slash or backslash. Case-insensitive identity. Retain the release-owned identifier; do not derive from localized labels. [SCM-S01] |
| `QUERY_SERVICE_CONFIGW.dwServiceType` | B/S | Conditional | DWORD flags. Creation alias: CreateServiceW.dwServiceType. Preserve approved process/service model; full domain in [SCM-S01] and [SCM-S02]. |
| `QUERY_SERVICE_CONFIGW.dwStartType` | B | Conditional | DWORD enum. Creation alias: CreateServiceW.dwStartType. Preserve approved startup policy; do not select auto/boot/driver settings to fill metadata. [SCM-S01] [SCM-S02] |
| `QUERY_SERVICE_CONFIGW.dwErrorControl` | B | Conditional | DWORD enum controlling startup-error handling; creation alias has the same member name. This is not application error severity. [SCM-S01] [SCM-S02] |
| `QUERY_SERVICE_CONFIGW.lpBinaryPathName` | B/S | Conditional | UTF-16 fully qualified executable path, possibly with arguments; quote paths containing spaces. Creation alias has the same member name. Record approved image and fixed arguments; never add user-controlled commands. [SCM-S01] [SCM-S02] |
| `QUERY_SERVICE_CONFIGW.lpLoadOrderGroup` | B | Conditional | UTF-16 group name; creation alias has the same member name. Absent group is valid. Load ordering is not a product category or descriptive tag. [SCM-S01] [SCM-S02] |
| `QUERY_SERVICE_CONFIGW.dwTagId` | B/T | Observed-only | DWORD ordering tag; creation output alias: CreateServiceW.lpdwTagId value. Zero means no assigned tag. Evaluated for boot/system-start drivers; no proposed NevroTick value. Never persist the output pointer. [SCM-S01] [SCM-S02] |
| `QUERY_SERVICE_CONFIGW.lpDependencies` | B | Conditional | MULTI_SZ of service/group names; creation alias has the same member name. Group entries use SC_GROUP_IDENTIFIER. Preserve actual approved dependency semantics; empty removes dependencies during update. [SCM-S01] [SCM-S02] [SCM-S19] |
| `QUERY_SERVICE_CONFIGW.lpServiceStartName` | S | Conditional | UTF-16 account name for user-mode service; driver-object name for driver type. Creation alias has the same member name. Preserve approved account mapping; metadata work does not authorize LocalSystem or account migration. [SCM-S01] [SCM-S02] |
| `QUERY_SERVICE_CONFIGW.lpDisplayName` | D | Recommended | UTF-16 label; maximum 256 characters. Creation alias: CreateServiceW.lpDisplayName. NULL on ChangeServiceConfigW means unchanged. Keep stable identity separate; support literal or qualified indirect-resource representation. [SCM-S01] [SCM-S19] |

### A.6. Optional configuration information levels — 10 selectors

These are the complete selectors listed by the two current API pages. Numeric gaps are not an invitation to invent additional supported levels. `dwInfoLevel` is an API selector, not product metadata. Full enum/value domains are linked through the associated member sources. [SCM-S03] [SCM-S21]

| Value | Selector | Structure |
| --- | --- | --- |
| 1 | `SERVICE_CONFIG_DESCRIPTION` | `SERVICE_DESCRIPTIONW` |
| 2 | `SERVICE_CONFIG_FAILURE_ACTIONS` | `SERVICE_FAILURE_ACTIONSW` |
| 3 | `SERVICE_CONFIG_DELAYED_AUTO_START_INFO` | `SERVICE_DELAYED_AUTO_START_INFO` |
| 4 | `SERVICE_CONFIG_FAILURE_ACTIONS_FLAG` | `SERVICE_FAILURE_ACTIONS_FLAG` |
| 5 | `SERVICE_CONFIG_SERVICE_SID_INFO` | `SERVICE_SID_INFO` |
| 6 | `SERVICE_CONFIG_REQUIRED_PRIVILEGES_INFO` | `SERVICE_REQUIRED_PRIVILEGES_INFOW` |
| 7 | `SERVICE_CONFIG_PRESHUTDOWN_INFO` | `SERVICE_PRESHUTDOWN_INFO` |
| 8 | `SERVICE_CONFIG_TRIGGER_INFO` | `SERVICE_TRIGGER_INFO` |
| 9 | `SERVICE_CONFIG_PREFERRED_NODE` | `SERVICE_PREFERRED_NODE_INFO` |
| 12 | `SERVICE_CONFIG_LAUNCH_PROTECTED` | `SERVICE_LAUNCH_PROTECTED_INFO` |

### A.7. Config2 structures and nested members — 27 rows

`SERVICE_FAILURE_ACTIONSW.lpsaActions[]` contains `SC_ACTION`. `SERVICE_TRIGGER_INFO.pTriggers[]` contains `SERVICE_TRIGGER`; its `pDataItems[]` contains `SERVICE_TRIGGER_SPECIFIC_DATA_ITEM`. Preserve this nesting and ordering in any future source/readback schema.

| Canonical field / member | Kind | Disposition | Type, rule, source |
| --- | --- | --- | --- |
| `SERVICE_DESCRIPTIONW.lpDescription` | D | Recommended | UTF-16 description or supported indirect-resource string; limited by REG_SZ capacity, not an invented 256/1024-character cap. NULL leaves unchanged; empty deletes. [SCM-S06] |
| `SERVICE_FAILURE_ACTIONSW.dwResetPeriod` | B | Conditional | DWORD seconds until failure count resets; INFINITE disables reset. Preserve reviewed recovery budget. [SCM-S04] |
| `SERVICE_FAILURE_ACTIONSW.lpRebootMsg` | D/B | Conditional | UTF-16 broadcast text, including supported resource indirection, used with a reboot action. NULL unchanged; empty deletes. No reboot action or message proposed for NevroTick. [SCM-S04] |
| `SERVICE_FAILURE_ACTIONSW.lpCommand` | B/S | Conditional | UTF-16 recovery command; runs under the service account for RUN_COMMAND. NULL unchanged; empty deletes. Do not introduce a recovery-command execution path for completeness. [SCM-S04] |
| `SERVICE_FAILURE_ACTIONSW.cActions` | T | Observed-only | DWORD count derived from the action array. Setter zero with non-NULL lpsaActions clears the reset period and action list; absence must not silently become a clear operation. [SCM-S04] |
| `SERVICE_FAILURE_ACTIONSW.lpsaActions` | B | Conditional | Ordered SC_ACTION array value, never a memory address. NULL makes cActions/dwResetPeriod ignored. Last action repeats after the array is exhausted; retain the reviewed terminal NONE/bounded-recovery policy. [SCM-S04] |
| `SC_ACTION.Type` | B | Conditional | SC_ACTION_TYPE enum per array element. Full domain in [SCM-S05]; select only an approved recovery action. |
| `SC_ACTION.Delay` | B | Conditional | DWORD milliseconds before that action. Coordinate with total recovery budget and stop/uninstall behavior. [SCM-S05] |
| `SERVICE_DELAYED_AUTO_START_INFO.fDelayedAutostart` | B | Conditional | BOOL; meaningful with automatic startup. A delayed-start request provides no precise startup-time guarantee. [SCM-S07] |
| `SERVICE_FAILURE_ACTIONS_FLAG.fFailureActionsOnNonCrashFailures` | B | Conditional | BOOL extends configured recovery to SERVICE_STOPPED reports with nonzero dwWin32ExitCode. Record desired versus effective configuration; docs specify next system start for effect. [SCM-S08] |
| `SERVICE_SID_INFO.dwServiceSidType` | S | Conditional | DWORD service-SID mode; full domain in [SCM-S09]. Token/ACL behavior, not descriptive identity or client authentication. Docs specify next system start for effect. |
| `SERVICE_REQUIRED_PRIVILEGES_INFOW.pmszRequiredPrivileges` | S | Conditional | UTF-16 MULTI_SZ privilege allowlist; constants linked from [SCM-S10]. When no required-privilege list is configured, SCM uses the process token defaults; omitting an update preserves existing configuration. Set only the verified minimum; changed list applies on next service start. |
| `SERVICE_PRESHUTDOWN_INFO.dwPreshutdownTimeout` | B | Conditional | DWORD milliseconds. Preserve an explicit lifecycle budget; do not copy a Windows default as a NevroTick guarantee. [SCM-S11] |
| `SERVICE_PREFERRED_NODE_INFO.usPreferredNode` | B | Conditional | USHORT NUMA node number. Omit unless an approved measured placement policy requires it. [SCM-S12] |
| `SERVICE_PREFERRED_NODE_INFO.fDelete` | T | Conditional | BOOLEAN update directive: TRUE removes the preferred-node setting. Not a durable descriptive field. [SCM-S12] |
| `SERVICE_LAUNCH_PROTECTED_INFO.dwLaunchProtected` | S | Observed-only | DWORD protection mode; complete domain in [SCM-S13]. Windows modes are reserved; antimalware mode has eligibility/signing requirements. Protected modes are not applicable to the current ordinary NevroTick service proposal. |
| `SERVICE_TRIGGER_INFO.cTriggers` | T | Observed-only | DWORD derived trigger count. Setter zero removes all triggers; docs report ERROR_INVALID_PARAMETER if none exist. An omitted trigger policy must not be encoded as a destructive clear by default. [SCM-S14] |
| `SERVICE_TRIGGER_INFO.pTriggers` | B | Conditional | Array value of SERVICE_TRIGGER records. Do not serialize a pointer or add triggers merely to populate metadata. [SCM-S14] |
| `SERVICE_TRIGGER_INFO.pReserved` | T | Not applicable | Reserved pointer; must be NULL. Never a product field. Accounted for solely to make the structure inventory complete. [SCM-S14] |
| `SERVICE_TRIGGER.dwTriggerType` | B | Conditional | DWORD trigger-type enum; full domain and type-specific requirements in [SCM-S15]. |
| `SERVICE_TRIGGER.dwAction` | B | Conditional | DWORD start/stop action. NETWORK_ENDPOINT allows start only; validate action against type. [SCM-S15] |
| `SERVICE_TRIGGER.pTriggerSubtype` | I/B | Conditional | GUID value whose meaning depends on trigger type; for a custom trigger it identifies an ETW provider. Never retain the pointer address. [SCM-S15] |
| `SERVICE_TRIGGER.cDataItems` | T | Observed-only | DWORD array count derived from pDataItems; data is valid only for applicable trigger types. Use the declared struct, not the documentation prose typo noted below. [SCM-S15] |
| `SERVICE_TRIGGER.pDataItems` | B | Conditional | Array value of SERVICE_TRIGGER_SPECIFIC_DATA_ITEM records; no raw pointers. Validate data against trigger/subtype. [SCM-S15] |
| `SERVICE_TRIGGER_SPECIFIC_DATA_ITEM.dwDataType` | B | Conditional | DWORD typed-payload discriminator; full domain in [SCM-S16]. XML choice and underlying trigger must agree. |
| `SERVICE_TRIGGER_SPECIFIC_DATA_ITEM.cbData` | T | Observed-only | DWORD payload byte count, derived from encoding; documented maximum 1024. XML string/schema limits do not override this native bound. [SCM-S16] |
| `SERVICE_TRIGGER_SPECIFIC_DATA_ITEM.pData` | B | Conditional | Typed payload value: bytes, Unicode string data, byte level, or 64-bit keyword mask as allowed by type. Never a pointer address; string payloads require correct Unicode representation. [SCM-S16] |

Source precision: the `SERVICE_TRIGGER.cDataItems` prose in [SCM-S15] names a nonexistent `dwDataType` member and uses an inconsistent device-arrival spelling. The declaration has `dwTriggerType`. Use the actual declaration plus the type-specific contract; do not copy that prose typo into generated field names.

### A.8. Runtime status — 9 unique rows

Readback uses `QueryServiceStatusEx(SC_STATUS_PROCESS_INFO)`. The first seven rows correspond to `SERVICE_STATUS`; the final two exist only in `SERVICE_STATUS_PROCESS`. Values are observed at a time, not written into static service metadata. A reported status can lag execution. [SCM-S17] [SCM-S18] [SCM-S40]

| Canonical field / member | Kind | Disposition | Type, rule, source |
| --- | --- | --- | --- |
| `SERVICE_STATUS_PROCESS.dwServiceType` | R | Observed-only | DWORD observed service-type flags; same reported field as SERVICE_STATUS.dwServiceType. Do not author a second configuration value. [SCM-S17] [SCM-S18] |
| `SERVICE_STATUS_PROCESS.dwCurrentState` | R | Observed-only | DWORD current state; alias SERVICE_STATUS.dwCurrentState. Observe the most recently reported state, which can lag actual execution. [SCM-S17] [SCM-S18] [SCM-S40] |
| `SERVICE_STATUS_PROCESS.dwControlsAccepted` | R | Observed-only | DWORD accepted-control flags; alias SERVICE_STATUS.dwControlsAccepted. Report only controls the current state and implementation support. [SCM-S17] [SCM-S18] |
| `SERVICE_STATUS_PROCESS.dwWin32ExitCode` | R | Observed-only | DWORD Win32 completion/error code; alias SERVICE_STATUS.dwWin32ExitCode. Keep failure/recovery semantics consistent. [SCM-S17] [SCM-S18] |
| `SERVICE_STATUS_PROCESS.dwServiceSpecificExitCode` | R | Observed-only | DWORD service error; alias SERVICE_STATUS.dwServiceSpecificExitCode. Meaningful when dwWin32ExitCode is ERROR_SERVICE_SPECIFIC_ERROR. [SCM-S17] [SCM-S18] |
| `SERVICE_STATUS_PROCESS.dwCheckPoint` | R | Observed-only | DWORD real pending-progress counter; alias SERVICE_STATUS.dwCheckPoint. Do not fake progress or treat it as a build/version counter. [SCM-S17] [SCM-S18] |
| `SERVICE_STATUS_PROCESS.dwWaitHint` | R | Observed-only | DWORD milliseconds estimating pending-operation time; alias SERVICE_STATUS.dwWaitHint. Not a renewable unlimited shutdown/startup allowance. [SCM-S17] [SCM-S18] |
| `SERVICE_STATUS_PROCESS.dwProcessId` | R | Observed-only | DWORD process identifier; valid in running/paused-related states specified by QueryServiceStatusEx, potentially invalid during start/stop pending, never valid when stopped. Not persistent identity. [SCM-S40] |
| `SERVICE_STATUS_PROCESS.dwServiceFlags` | R | Observed-only | DWORD system-process flag: zero or SERVICE_RUNS_IN_SYSTEM_PROCESS in the documented domain. Read status; never use it as a product security badge. [SCM-S18] |

### A.9. Packaged service projection — 42 rows

All rows are conditional on using and qualifying that packaging route. The `windows.service` extension belongs under the application's `Extensions`; service capability declaration does not by itself prove deployment authorization. Use the appropriate `packagedServices` or `localSystemServices` restricted-capability contract. Common enclosing application/extension attributes are outside this SCM section. [SCM-S26] [SCM-S30] [SCM-S33]

#### A.9.1. desktop6 service and dependency graph — 8 rows

| Canonical field / member | Kind | Disposition | Type, rule, source |
| --- | --- | --- | --- |
| `desktop6:Service/@Name` | I | Conditional | Required service identity; map to approved lpServiceName. Published page leaves lexical type incomplete; validate selected SDK schema. [SCM-S26] |
| `desktop6:Service/@StartupType` | B | Conditional | Required enum: auto/manual/disabled. Map approved policy to dwStartType; desktop6 does not advertise delayedStart. [SCM-S26] |
| `desktop6:Service/@StartAccount` | S | Conditional | Required enum: localSystem/localService/networkService. Packaging options do not qualify NevroTick's actual account/provisioning route. [SCM-S26] |
| `desktop6:Service/@Arguments` | B/S | Conditional | Optional string, 1–32767 characters, non-whitespace endpoints. Fixed reviewed arguments only; omission is valid. [SCM-S26] |
| `desktop6:Service/desktop6:Dependencies` | B | Conditional | Optional single container; use only approved dependencies. [SCM-S26] |
| `desktop6:Service/desktop6:TriggerEvents` | B | Conditional | Optional single container; no trigger needed merely for completeness. [SCM-S26] |
| `desktop6:Dependencies/desktop6:DependentService` | B | Conditional | One to 10000 children when container exists; schema maximum is not a recommended dependency count. [SCM-S27] |
| `desktop6:DependentService/@Name` | I/B | Conditional | Required dependency name. Preserve a real dependency; lexical type is incompletely described in the page. [SCM-S28] |

#### A.9.2. Trigger containers and event definitions — 17 rows

| Canonical field / member | Kind | Disposition | Type, rule, source |
| --- | --- | --- | --- |
| `desktop6:TriggerEvents/desktop6:TriggerCustom` | B | Conditional | Custom-trigger child slot. Shares a combined zero-to-20000 choice with the desktop10 branch. [SCM-S29] |
| `desktop6:TriggerEvents/desktop10:PredefinedTriggerEvents` | B | Conditional | Predefined-trigger child slot; same combined choice. Requires Windows 11 22H2/build 22621 or supported later target. [SCM-S29] [SCM-S30] |
| `desktop6:TriggerCustom/@Action` | B | Conditional | Required enum: ActionStart/ActionStop. Project only reviewed trigger behavior. [SCM-S32] |
| `desktop6:TriggerCustom/@Subtype` | I/B | Conditional | Required GUID; custom ETW-provider identity, subject to the native trigger contract. [SCM-S32] [SCM-S15] |
| `desktop6:TriggerCustom/desktop6:StringData` | B | Conditional | String payload child slot; data schema below. Exact choice/cardinality needs selected-SDK validation. [SCM-S32] |
| `desktop6:TriggerCustom/desktop6:BinaryData` | B | Conditional | Binary payload child slot; data schema below. Same particle-validation caveat. [SCM-S32] |
| `desktop6:TriggerCustom/desktop6:LevelData` | B | Conditional | Byte-level payload child slot; data schema below. Same particle-validation caveat. [SCM-S32] |
| `desktop6:TriggerCustom/desktop6:KeywordAnyData` | B | Conditional | Any-keyword-mask child slot; data schema below. Same particle-validation caveat. [SCM-S32] |
| `desktop6:TriggerCustom/desktop6:KeywordAllData` | B | Conditional | All-keyword-mask child slot; data schema below. Same particle-validation caveat. [SCM-S32] |
| `desktop10:PredefinedTriggerEvents/@Action` | B | Conditional | Required enum: ActionStart/ActionStop; also enforce type-specific native action restrictions. [SCM-S30] [SCM-S15] |
| `desktop10:PredefinedTriggerEvents/@Subtype` | I/B | Conditional | Required GUID, interpreted according to TriggerType; arbitrary GUIDs are not valid substitutes. [SCM-S30] |
| `desktop10:PredefinedTriggerEvents/@TriggerType` | B | Conditional | Required enum with six documented predefined types; use the complete value domain in [SCM-S30]. |
| `desktop10:PredefinedTriggerEvents/desktop10:StringData` | B | Conditional | String data slot; one of five child kinds in a combined zero-to-10000 choice. [SCM-S30] |
| `desktop10:PredefinedTriggerEvents/desktop10:BinaryData` | B | Conditional | Binary data slot; participates in the same combined choice. [SCM-S30] |
| `desktop10:PredefinedTriggerEvents/desktop10:LevelData` | B | Conditional | Byte-level data slot; participates in the same combined choice. [SCM-S30] |
| `desktop10:PredefinedTriggerEvents/desktop10:KeywordAnyData` | B | Conditional | Any-keyword-mask data slot; participates in the same combined choice. [SCM-S30] |
| `desktop10:PredefinedTriggerEvents/desktop10:KeywordAllData` | B | Conditional | All-keyword-mask data slot; participates in the same combined choice. [SCM-S30] |

#### A.9.3. Payload definitions — 11 rows

| Canonical field / member | Kind | Disposition | Type, rule, source |
| --- | --- | --- | --- |
| `desktop6:StringData/desktop6:DataItem` | B | Conditional | One to 10000 string items; native encoded payload limits still apply. [SCM-S34] |
| `desktop6:DataItem/@Value` | B | Conditional | Required string, 1–32767 characters, non-whitespace endpoints. Shared definition is counted once, including use under desktop10:StringData. [SCM-S35] [SCM-S30] |
| `desktop6:BinaryData/@File` | B/S | Conditional | Required filename, 1–256 characters; prohibited-character rules in [SCM-S36]. Resolve only the reviewed payload resource. |
| `desktop6:LevelData/@Value` | B | Conditional | Required unsigned byte. [SCM-S37] |
| `desktop6:KeywordAnyData/@Value` | B | Conditional | Required unsigned 64-bit integer mask. [SCM-S38] |
| `desktop6:KeywordAllData/@Value` | B | Conditional | Required unsigned 64-bit integer mask. [SCM-S39] |
| `desktop10:StringData/desktop6:DataItem` | B | Conditional | One to 10000 children. DataItem remains in the desktop6 namespace; its attribute definition is above. [SCM-S30] |
| `desktop10:BinaryData/@File` | B/S | Conditional | Required binary-data filename; corresponding desktop6 payload type supplies lexical constraints. [SCM-S30] [SCM-S36] |
| `desktop10:LevelData/@Value` | B | Conditional | Required integer from 0 to 255. [SCM-S30] |
| `desktop10:KeywordAnyData/@Value` | B | Conditional | Required unsigned 64-bit integer mask. [SCM-S30] |
| `desktop10:KeywordAllData/@Value` | B | Conditional | Required unsigned 64-bit integer mask. [SCM-S30] |

#### A.9.4. desktop7 direct-service cross-reference — 6 rows

These are a separate namespace-qualified contract, not aliases that may be freely renamed in a desktop6 manifest.

| Canonical field / member | Kind | Disposition | Type, rule, source |
| --- | --- | --- | --- |
| `desktop7:Service/@Name` | I | Conditional | Required identity; direct schema cross-reference, not a second service. Same published lexical-description gap as desktop6. [SCM-S33] |
| `desktop7:Service/@StartupType` | B | Conditional | Required enum: auto/manual/disabled/delayedStart. This namespace adds delayedStart; do not add it to desktop6 syntax. [SCM-S33] |
| `desktop7:Service/@StartAccount` | S | Conditional | Required enum: localSystem/localService/networkService. Retain the unqualified provisioning/account gate. [SCM-S33] |
| `desktop7:Service/@Arguments` | B/S | Conditional | Optional string, 1–32767 characters, non-whitespace endpoints. Same fixed-argument policy. [SCM-S33] |
| `desktop7:Service/desktop7:Dependencies` | B | Conditional | Optional single child slot. Only the direct desktop7 Service contract is enumerated here; do not infer descendant namespace rewrites. [SCM-S33] |
| `desktop7:Service/desktop7:TriggerEvents` | B | Conditional | Optional single child slot; source states the desktop6 trigger-child contract applies and desktop10 predefined events remain desktop10. [SCM-S33] [SCM-S30] |

#### A.9.5. XML precision and route limits

The published `desktop6:Service/@Name` and dependency-name pages do not provide a complete lexical type description. `desktop6:TriggerCustom` shows each payload kind with an occurrence range but does not adequately explain the choice particle. Do not conclude that all five payload kinds must be present. Validate the selected Windows SDK XSD before implementing a generator; no XSD validation or packaging was executed here. Some older descendant pages also show inconsistent duplicated hierarchy breadcrumbs. Use the explicit application-level placement in current Service/PredefinedTriggerEvents documentation. [SCM-S26] [SCM-S28] [SCM-S32] [SCM-S30]

A valid XML shape is only one gate. Trigger subtype/action/data compatibility and native byte-size limits still apply. XML maxima are not recommended product budgets. The desktop7 direct cross-reference adds `delayedStart`; desktop6's listed startup values do not. [SCM-S15] [SCM-S16] [SCM-S26] [SCM-S33]

Neither `desktop6:Service` nor `desktop7:Service` exposes `DisplayName` or `Description` attributes. However, the official MSIX Packaging Tool conversion report explicitly offers **Display name** and **Description** fields. Captured-installer metadata and manual manifest declaration therefore need separate qualification. Do not claim MSIX cannot retain those fields, and do not assume post-install setters are the only supported route. The conversion page also documents its outside-package-dependency limitation; do not silently generalize that statement to every packaging route. [SCM-S26] [SCM-S33] [SCM-S31]

### A.10. Localization and faithful readback

Supported SCM indirection uses a string resource reference, commonly `@<PE-path>,-<stringID>[;<comment>]`. Generic Windows string redirection refers to a PE resource; it does not inherently require adding a new DLL. Use an existing approved protected resource location when applicable. The optional comment belongs to the reference syntax, not a separate SCM property. Do not silently substitute package `ms-resource:` URIs for SCM indirection. [SCM-S06] [SCM-S19] [SCM-S23] [SCM-S24]

Keep **stored reference**, **resource identity**, and **resolved locale text** distinguishable in verification evidence. Locale mappings are authoring/build inputs, not extra SCM registry properties. Resource indirection must continue to resolve after installation, upgrade, repair, rollback, and supported locale selection. Those lifecycle outcomes require future qualified evidence, not inference from a valid reference string. For direct resource resolution, use the Unicode `RegLoadMUIStringW` contract; the advertised ANSI variant is not implemented. [SCM-S25]

`QueryServiceConfigW`/`QueryServiceConfig2W` require `SERVICE_QUERY_CONFIG`; `QueryServiceStatusEx` requires `SERVICE_QUERY_STATUS`. Query/configuration buffer limits are transport bounds, not substitute per-field character limits. Configuration setters require `SERVICE_CHANGE_CONFIG`; a metadata reader does not need that write right. A display-name mapping can use `GetServiceDisplayNameW`, with its documented character/buffer contract. No query or setter was executed against Windows in this research. [SCM-S20] [SCM-S21] [SCM-S40] [SCM-S19] [SCM-S03] [SCM-S22]

Absent, explicit empty, zero, NULL, and delete directives are not interchangeable. Future adapters must preserve the documented update semantics, particularly description clearing, dependency clearing, failure-action reset, trigger removal, and preferred-node deletion. Desired configuration, successful write, observed configuration, and effective runtime state are separate evidence claims. [SCM-S06] [SCM-S19] [SCM-S04] [SCM-S14] [SCM-S12]

### A.11. Explicit API exclusions

| Argument / category | Reason for exclusion | Handling |
| --- | --- | --- |
| `CreateServiceW.hSCManager` | SCM handle/capability, not service content. | Never persist a handle value in product metadata. [SCM-S01] |
| `CreateServiceW.dwDesiredAccess` | Requested access to the returned handle, not the installed service's descriptive/configuration field. | Keep in code/security design; do not confuse with the service object's DACL. [SCM-S01] |
| `CreateServiceW.lpPassword` | Secret account-credential input, not metadata and not returned by QUERY_SERVICE_CONFIGW. | Do not retain, export, log, or place it in a metadata source record. Account provisioning remains separately governed. [SCM-S01] [SCM-S02] |

Other API handles, status handles, info-level request selectors, buffer addresses, buffer capacities, return/error codes, and byte-count output parameters are API transport/operation state. They are excluded as product metadata. Member counts and `pReserved` are visible above solely because the requested structure inventory includes them. Registry paths identify SCM storage; arbitrary added values such as a fabricated SCM Publisher/SupportURL do not become standard Services properties.

### A.12. Review and implementation handoff

1. Confirm truthful role descriptions and stable technical identities with the authoritative architecture/install owner. Keep proposal text visibly provisional until accepted.
2. Extend the existing metadata source/projection contract with supported service labels and descriptions, locale/resource mappings where needed, and explicit per-field desired/observed status. Do not create competing identity records.
3. Preserve approved service account, ACL, privilege, process, startup, dependency, trigger, recovery, and shutdown policies. Apply optional settings only through their existing reviewed owners.
4. Select and qualify the actual package/provisioning path. Check the chosen SDK XML schema, emitted package/captured service metadata, permissions, resource paths, and update/repair behavior.
5. At the permitted native validation stage, read back names, descriptions, base configuration, applicable Config2 values, locale resolution, and actual runtime state. Verify meaningful absent/empty/delete cases and effective-state timing; do not install or reconfigure during read-only inspection.
6. Keep unresolved gates explicit: no native validation performed, no installed account mapping proven, no launch-protection eligibility established, no new service capabilities authorized.

### A.13. Source manifest

The supplied research lists these 40 primary sources. Integration rechecked selected consequential SCM claims, not every page or the selected SDK XSD. Source IDs are local to this section.

| ID | Primary source |
| --- | --- |
| [SCM-S01] | CreateServiceW |
| [SCM-S02] | QUERY_SERVICE_CONFIGW |
| [SCM-S03] | ChangeServiceConfig2W |
| [SCM-S04] | SERVICE_FAILURE_ACTIONSW |
| [SCM-S05] | SC_ACTION |
| [SCM-S06] | SERVICE_DESCRIPTIONW |
| [SCM-S07] | SERVICE_DELAYED_AUTO_START_INFO |
| [SCM-S08] | SERVICE_FAILURE_ACTIONS_FLAG |
| [SCM-S09] | SERVICE_SID_INFO |
| [SCM-S10] | SERVICE_REQUIRED_PRIVILEGES_INFOW |
| [SCM-S11] | SERVICE_PRESHUTDOWN_INFO |
| [SCM-S12] | SERVICE_PREFERRED_NODE_INFO |
| [SCM-S13] | SERVICE_LAUNCH_PROTECTED_INFO |
| [SCM-S14] | SERVICE_TRIGGER_INFO |
| [SCM-S15] | SERVICE_TRIGGER |
| [SCM-S16] | SERVICE_TRIGGER_SPECIFIC_DATA_ITEM |
| [SCM-S17] | SERVICE_STATUS |
| [SCM-S18] | SERVICE_STATUS_PROCESS |
| [SCM-S19] | ChangeServiceConfigW |
| [SCM-S20] | QueryServiceConfigW |
| [SCM-S21] | QueryServiceConfig2W |
| [SCM-S22] | GetServiceDisplayNameW |
| [SCM-S23] | Using Registry String Redirection |
| [SCM-S24] | Locating Redirected Strings |
| [SCM-S25] | RegLoadMUIStringW |
| [SCM-S26] | desktop6:Service |
| [SCM-S27] | desktop6:Dependencies |
| [SCM-S28] | desktop6:DependentService |
| [SCM-S29] | desktop6:TriggerEvents |
| [SCM-S30] | desktop10:PredefinedTriggerEvents and data children |
| [SCM-S31] | Convert an installer that includes services |
| [SCM-S32] | desktop6:TriggerCustom |
| [SCM-S33] | desktop7:Service |
| [SCM-S34] | desktop6:StringData |
| [SCM-S35] | desktop6:DataItem |
| [SCM-S36] | desktop6:BinaryData |
| [SCM-S37] | desktop6:LevelData |
| [SCM-S38] | desktop6:KeywordAnyData |
| [SCM-S39] | desktop6:KeywordAllData |
| [SCM-S40] | QueryServiceStatusEx |

[SCM-S01]: https://learn.microsoft.com/windows/win32/api/winsvc/nf-winsvc-createservicew
[SCM-S02]: https://learn.microsoft.com/windows/win32/api/winsvc/ns-winsvc-query_service_configw
[SCM-S03]: https://learn.microsoft.com/windows/win32/api/winsvc/nf-winsvc-changeserviceconfig2w
[SCM-S04]: https://learn.microsoft.com/windows/win32/api/winsvc/ns-winsvc-service_failure_actionsw
[SCM-S05]: https://learn.microsoft.com/windows/win32/api/winsvc/ns-winsvc-sc_action
[SCM-S06]: https://learn.microsoft.com/windows/win32/api/winsvc/ns-winsvc-service_descriptionw
[SCM-S07]: https://learn.microsoft.com/windows/win32/api/winsvc/ns-winsvc-service_delayed_auto_start_info
[SCM-S08]: https://learn.microsoft.com/windows/win32/api/winsvc/ns-winsvc-service_failure_actions_flag
[SCM-S09]: https://learn.microsoft.com/windows/win32/api/winsvc/ns-winsvc-service_sid_info
[SCM-S10]: https://learn.microsoft.com/windows/win32/api/winsvc/ns-winsvc-service_required_privileges_infow
[SCM-S11]: https://learn.microsoft.com/windows/win32/api/winsvc/ns-winsvc-service_preshutdown_info
[SCM-S12]: https://learn.microsoft.com/windows/win32/api/winsvc/ns-winsvc-service_preferred_node_info
[SCM-S13]: https://learn.microsoft.com/windows/win32/api/winsvc/ns-winsvc-service_launch_protected_info
[SCM-S14]: https://learn.microsoft.com/windows/win32/api/winsvc/ns-winsvc-service_trigger_info
[SCM-S15]: https://learn.microsoft.com/windows/win32/api/winsvc/ns-winsvc-service_trigger
[SCM-S16]: https://learn.microsoft.com/windows/win32/api/winsvc/ns-winsvc-service_trigger_specific_data_item
[SCM-S17]: https://learn.microsoft.com/windows/win32/api/winsvc/ns-winsvc-service_status
[SCM-S18]: https://learn.microsoft.com/windows/win32/api/winsvc/ns-winsvc-service_status_process
[SCM-S19]: https://learn.microsoft.com/windows/win32/api/winsvc/nf-winsvc-changeserviceconfigw
[SCM-S20]: https://learn.microsoft.com/windows/win32/api/winsvc/nf-winsvc-queryserviceconfigw
[SCM-S21]: https://learn.microsoft.com/windows/win32/api/winsvc/nf-winsvc-queryserviceconfig2w
[SCM-S22]: https://learn.microsoft.com/windows/win32/api/winsvc/nf-winsvc-getservicedisplaynamew
[SCM-S23]: https://learn.microsoft.com/windows/win32/intl/using-registry-string-redirection
[SCM-S24]: https://learn.microsoft.com/windows/win32/intl/locating-redirected-strings
[SCM-S25]: https://learn.microsoft.com/windows/win32/api/winreg/nf-winreg-regloadmuistringw
[SCM-S26]: https://learn.microsoft.com/uwp/schemas/appxpackage/uapmanifestschema/element-desktop6-service
[SCM-S27]: https://learn.microsoft.com/uwp/schemas/appxpackage/uapmanifestschema/element-desktop6-dependencies
[SCM-S28]: https://learn.microsoft.com/uwp/schemas/appxpackage/uapmanifestschema/element-desktop6-dependentservice
[SCM-S29]: https://learn.microsoft.com/uwp/schemas/appxpackage/uapmanifestschema/element-desktop6-triggerevents
[SCM-S30]: https://learn.microsoft.com/uwp/schemas/appxpackage/uapmanifestschema/element-desktop10-predefinedtriggerevents
[SCM-S31]: https://learn.microsoft.com/windows/msix/packaging-tool/convert-an-installer-with-services
[SCM-S32]: https://learn.microsoft.com/uwp/schemas/appxpackage/uapmanifestschema/element-desktop6-triggercustom
[SCM-S33]: https://learn.microsoft.com/uwp/schemas/appxpackage/uapmanifestschema/element-desktop7-service
[SCM-S34]: https://learn.microsoft.com/uwp/schemas/appxpackage/uapmanifestschema/element-desktop6-stringdata
[SCM-S35]: https://learn.microsoft.com/uwp/schemas/appxpackage/uapmanifestschema/element-desktop6-dataitem
[SCM-S36]: https://learn.microsoft.com/uwp/schemas/appxpackage/uapmanifestschema/element-desktop6-binarydata
[SCM-S37]: https://learn.microsoft.com/uwp/schemas/appxpackage/uapmanifestschema/element-desktop6-leveldata
[SCM-S38]: https://learn.microsoft.com/uwp/schemas/appxpackage/uapmanifestschema/element-desktop6-keywordanydata
[SCM-S39]: https://learn.microsoft.com/uwp/schemas/appxpackage/uapmanifestschema/element-desktop6-keywordalldata
[SCM-S40]: https://learn.microsoft.com/windows/win32/api/winsvc/nf-winsvc-queryservicestatusex

---

## Appendix B: PE, manifests, packages, signing and distribution

Research snapshot: 7 November 2026, `df6e28a4641f2f046373a9c5640aa60736716a22`. Artifact/schema inventory; no generated or signed candidate qualified.

### B.1. Scope and interpretation

“Complete” here means every member, attribute or direct child of the specifically named structures below, at the cited documentation/schema snapshot. It does not mean every possible PE resource, X.509 extension, COM registration, MSIX extension, Partner Center workflow or project-defined key. Arbitrary VERSIONINFO string names and project release-record keys form open sets. Nested behavioral extension trees are deliberately routed to their owning specification.

Disposition: **A** = useful authored description; **D** = generated/observed; **B** = behavior/identity contract, not decoration; **C** = conditional on an admitted delivery/component route; **N** = compiler-owned, reserved, inapplicable or intentionally not authored. Combinations are intentional. A catalogue entry is not a recommendation to enable it.

The current repository already owns most high-level rules in docs/plans/windows-package-metadata.md §§4–11, the maintain-windows-metadata skill and OD-05 §7.6. Preserve W11-W10/P6 and CHECK-18/20/23/26/27 ownership; service effects stay under W11-W05. Current native/build/storage/security holds remain. This section supplies precision and a field-by-field inventory, not a new workstream or release gate.

### B.2. Completeness ledger

Counts are local to each named structure. Do not add them as “independent metadata values”: fixed fields, RC directives, displayed versions, resource containers and installed projections overlap.

| Named scope | Count and boundary | Sources |
| --- | --- | --- |
| Conventional VERSIONINFO strings | 12 names; custom names remain open-ended | ART-S01, ART-S02, ART-S07 |
| VS_FIXEDFILEINFO | 13 members | ART-S03 |
| VS_VERSIONINFO | 8 layout members | ART-S04 |
| StringFileInfo | 6 layout members | ART-S05 |
| StringTable | 6 layout members | ART-S06 |
| String | 6 layout members | ART-S07 |
| VarFileInfo | 6 layout members | ART-S08 |
| Var | 6 layout members | ART-S09 |
| Translation value | 2 logical components per DWORD; already contained in Var.Value | ART-S09 |
| RC fixed-field authoring | 7 directives, plus resource ID; projections of fixed/layout fields | ART-S01 |
| Native assembly identity/presentation subset | assembly/@manifestVersion: 1; assemblyIdentity: 6 attributes; assembly/description: 1 optional element | ART-S13, ART-S14 |
| MSIX Identity | 5 attributes; no children | ART-S15, ART-S19 |
| MSIX Properties | 0 attributes; 24 concrete direct children in pinned schema composition: 22 ordinary declarations plus 2 concrete choice substitutions | ART-S16, ART-S19, ART-S21, ART-S22 |
| MSIX Application | 24 attributes in pinned schema composition: 22 on current Learn page plus 2 inherited preview attributes; 5 possible concrete direct-child names, with alternative VisualElements nodes | ART-S17, ART-S19, ART-S23–ART-S27 |
| uap:VisualElements | 6 attributes; 4 direct children | ART-S18, ART-S23 |
| uap3:VisualElements | Inherits the preceding 6 attributes and 4 children; adds 1 attribute | ART-S24 |
| MSIX Resource | 3 attributes; no children in this named node | ART-S19 |
| MSIX TargetDeviceFamily | 3 attributes; no children in this named node | ART-S19 |
| Authenticode descriptive authoring subset | 2 SignTool options; not an exhaustive signature or certificate inventory | ART-S28 |
| MSI product/ARP authoring subset | 22 property names: 6 product identity/display properties, 15 documented ARP properties, 1 migration-settings property | ART-S36–ART-S38, ART-S41 |
| MSI Uninstall registry documentation | 22 named values; projections/observations, not 22 extra authoring inputs | ART-S37 |
| MSI installation-database Summary Information | 17 properties; includes fields that should remain null | ART-S39, ART-S40 |
| Store descriptive inventory below | 40 named UI fields/fieldsets: 21 listing/additional and 19 product/support/regulatory; excludes recursive game/display/declaration/payment/account workflows | ART-S30–ART-S34 |

**Schema precision:** Microsoft’s checked-in Windows SDK files are pinned to microsoft/win32metadata commit 15fc870a5e96df7c560877ff30136c0fb5a21e9b. They corroborate field membership; their presence does not prove support on NevroTick’s minimum OS, Store admissibility or adoption by the selected packaging tool. Generated Learn pages omit some inherited/choice details and contain unfinished datatype placeholders. Use the selected SDK’s complete schema set and actual artifact readback when implementation is authorized.

### B.3. Recommended values and pending decisions

| Surface/component | Recommended treatment | Status |
| --- | --- | --- |
| ProductName; package display name | NevroTick, subject to actual Store reservation/localization rules | Existing canonical product name |
| CompanyName; publisher/developer display attribution | David Osipov; do not invent a corporation | Existing intended display identity |
| UI FileDescription | “NevroTick user interface” | Proposed wording; reflect shipped role |
| Engine-service FileDescription | “NevroTick time synchronization engine” | Proposed wording; only if that capability ships |
| Clock-broker FileDescription | “NevroTick clock control broker” | Proposed wording; actual privilege boundary must match |
| Administration/helper FileDescription | Describe that particular shipped helper | Pending actual component inventory |
| Product/file versions | Existing release authority mapped to readable strings and numeric formats | No new version number selected |
| InternalName; OriginalFilename | Actual stable module name; actual shipped basename | Derive from frozen outputs |
| Copyright/trademark/license | Exact approved rights and current license inventory | Never infer registration, exclusive rights or SPDX choice |
| Public URLs | Actual maintained project/support/docs/privacy/license destinations | Dedicated support/privacy/signature URLs remain pending verification |
| Package Name/Publisher/Application Id | Assigned/admitted identities, preserved over updates | Never derive cryptographic identity from a friendly author name |
| Architecture, OS, CPU requirements | Actual qualified targets, including x86-64-v3 policy where applicable | “x64” alone does not describe CPU prerequisites |
| Signature description | Component role or product/package purpose | Optional only on the admitted signer route |
| Native manifest description | Short component-purpose sentence | Optional; selected SDK/output qualification required |

A service EXE is a user-mode application: VS_FIXEDFILEINFO.dwFileType should reflect VFT_APP, with subtype zero. A DLL uses VFT_DLL. Calling something a Windows service does not make it VFT_DRV. Rights strings, service descriptions and publisher labels do not authenticate a client or authorize service control. [ART-S01, ART-S03]

### B.4. PE VERSIONINFO

#### B.4.1. All 12 conventional string fields

Path prefix: VS_VERSIONINFO/StringFileInfo/StringTable[language-codepage]/String. Names below are String.szKey values; each row describes its String.Value. Seven names marked “conventional required” follow Microsoft’s VERSIONINFO guidance, rather than claiming that every Windows loader rejects a file without them. [ART-S01, ART-S02, ART-S07]

| Canonical field | Disposition and intended value |
| --- | --- |
| String[Comments].Value | A/C: useful short component context; optional |
| String[CompanyName].Value | A: David Osipov; conventional required |
| String[FileDescription].Value | A: role-specific public description; conventional required |
| String[FileVersion].Value | D/A: readable actual component version; conventional required |
| String[InternalName].Value | D/A: actual stable module name; conventional required |
| String[LegalCopyright].Value | A/C: truthful applicable notice |
| String[LegalTrademarks].Value | A/C: truthful applicable trademark statement; otherwise omit |
| String[OriginalFilename].Value | D: shipped basename including extension, no directory; conventional required |
| String[PrivateBuild].Value | C: only with truthful VS_FF_PRIVATEBUILD |
| String[ProductName].Value | A: NevroTick; conventional required |
| String[ProductVersion].Value | D/A: readable actual product version; conventional required |
| String[SpecialBuild].Value | C: only with truthful VS_FF_SPECIALBUILD |

Custom String.szKey values are permitted Unicode names, not extra standardized Explorer columns. Treat custom provenance/support/license keys as optional projections; keep their authoritative record in the release inventory. No exhaustive finite list of custom names exists. [ART-S07]

#### B.4.2. All VS_FIXEDFILEINFO members

| Canonical field | Disposition and rule |
| --- | --- |
| VS_FIXEDFILEINFO.dwSignature | N: format marker 0xFEEF04BD |
| VS_FIXEDFILEINFO.dwStrucVersion | N: resource-structure version |
| VS_FIXEDFILEINFO.dwFileVersionMS | D: first two 16-bit file-version components |
| VS_FIXEDFILEINFO.dwFileVersionLS | D: last two file-version components |
| VS_FIXEDFILEINFO.dwProductVersionMS | D: first two product-version components |
| VS_FIXEDFILEINFO.dwProductVersionLS | D: last two product-version components |
| VS_FIXEDFILEINFO.dwFileFlagsMask | D: valid flag-bit mask |
| VS_FIXEDFILEINFO.dwFileFlags | D: truthful build flags |
| VS_FIXEDFILEINFO.dwFileOS | D: documented target-OS classification; no invented VOS_WINDOWS64 |
| VS_FIXEDFILEINFO.dwFileType | D: actual application/DLL/driver/etc. classification |
| VS_FIXEDFILEINFO.dwFileSubtype | D/N: zero unless the documented type requires a subtype |
| VS_FIXEDFILEINFO.dwFileDateMS | D/N: high part of optional binary creation-date value |
| VS_FIXEDFILEINFO.dwFileDateLS | D/N: low part of that value |

Source: ART-S03. Do not invent date epochs or a fresh wall-clock value merely to fill the two date words. Preserve reproducibility policy.

The six documented flag names are VS_FF_DEBUG, VS_FF_INFOINFERRED, VS_FF_PATCHED, VS_FF_PRERELEASE, VS_FF_PRIVATEBUILD and VS_FF_SPECIALBUILD. **VS_FF_INFOINFERRED must never be embedded in a file’s version resource.** Debug/prerelease/private/special labels must follow actual build semantics, not distribution assumptions. [ART-S03]

The seven RC directives are aliases/projections, not extra independent metadata:

| RC field | Target |
| --- | --- |
| VERSIONINFO.FILEVERSION | dwFileVersionMS/LS |
| VERSIONINFO.PRODUCTVERSION | dwProductVersionMS/LS |
| VERSIONINFO.FILEFLAGSMASK | dwFileFlagsMask |
| VERSIONINFO.FILEFLAGS | dwFileFlags |
| VERSIONINFO.FILEOS | dwFileOS |
| VERSIONINFO.FILETYPE | dwFileType |
| VERSIONINFO.FILESUBTYPE | dwFileSubtype |
| VERSIONINFO.resourceID | Resource identifier 1, as documented |

Source: ART-S01. Four numeric version components each fit 0–65535. Readable versions can preserve approved prerelease/channel notation; their relationship to numeric versions must be explicit.

#### B.4.3. All version-resource layout members

These Microsoft pseudo-structures describe variable-length resource layout. They are not a license to cast untrusted bytes to fixed C structures. Length units differ. All padding, lengths and literal container keys are generated structural data, not user-facing metadata to hand-fill.

| Canonical field | Meaning/disposition | Source |
| --- | --- | --- |
| VS_VERSIONINFO.wLength | N: total structure length in bytes | ART-S04 |
| VS_VERSIONINFO.wValueLength | N: Value length in bytes; zero if absent | ART-S04 |
| VS_VERSIONINFO.wType | N: text/binary indicator | ART-S04 |
| VS_VERSIONINFO.szKey | N: literal VS_VERSION_INFO | ART-S04 |
| VS_VERSIONINFO.Padding1 | N: alignment before Value | ART-S04 |
| VS_VERSIONINFO.Value | N/D: VS_FIXEDFILEINFO payload | ART-S04 |
| VS_VERSIONINFO.Padding2 | N: alignment before Children | ART-S04 |
| VS_VERSIONINFO.Children | N: StringFileInfo/VarFileInfo blocks | ART-S04 |
| StringFileInfo.wLength | N: block length in bytes | ART-S05 |
| StringFileInfo.wValueLength | N: zero | ART-S05 |
| StringFileInfo.wType | N: text/binary indicator | ART-S05 |
| StringFileInfo.szKey | N: literal StringFileInfo | ART-S05 |
| StringFileInfo.Padding | N: child alignment | ART-S05 |
| StringFileInfo.Children | D: language/codepage StringTable blocks | ART-S05 |
| StringTable.wLength | N: block length in bytes | ART-S06 |
| StringTable.wValueLength | N: zero | ART-S06 |
| StringTable.wType | N: text/binary indicator | ART-S06 |
| StringTable.szKey | D: eight hexadecimal digits; LANGID then codepage | ART-S06 |
| StringTable.Padding | N: child alignment | ART-S06 |
| StringTable.Children | A/D: String records | ART-S06 |
| String.wLength | N: record length in bytes | ART-S07 |
| String.wValueLength | N: Value length in WORDs | ART-S07 |
| String.wType | N: text/binary indicator | ART-S07 |
| String.szKey | A/D: Unicode metadata name | ART-S07 |
| String.Padding | N: value alignment | ART-S07 |
| String.Value | A/D: terminated Unicode value | ART-S07 |
| VarFileInfo.wLength | N: block length in bytes | ART-S08 |
| VarFileInfo.wValueLength | N: zero | ART-S08 |
| VarFileInfo.wType | N: text/binary indicator | ART-S08 |
| VarFileInfo.szKey | N: literal VarFileInfo | ART-S08 |
| VarFileInfo.Padding | N: child alignment | ART-S08 |
| VarFileInfo.Children | D: Var records | ART-S08 |
| Var.wLength | N: record length in bytes | ART-S09 |
| Var.wValueLength | N: Value length in bytes | ART-S09 |
| Var.wType | N: text/binary indicator | ART-S09 |
| Var.szKey | N: literal Translation for the documented language record | ART-S09 |
| Var.Padding | N: value alignment | ART-S09 |
| Var.Value | D: language/codepage DWORD array | ART-S09 |
| Var.Value[i].lowWord | D: LANGID component | ART-S09 |
| Var.Value[i].highWord | D: codepage component | ART-S09 |

Translation rows decompose Var.Value; they are not extra physical members. StringTable’s textual hexadecimal order and the DWORD’s low/high word order are different representations. Match emitted tables to actual translations; for example 040904B0 denotes English (US)/Unicode, only when such resources actually exist. Do not advertise every Windows language automatically. [ART-S06, ART-S09]

#### B.4.4. Icons, resource identification and debug-path privacy

| Named surface | Treatment | Sources |
| --- | --- | --- |
| PE resource type RT_VERSION | Version-information resource type 16; resource ID above remains distinct from type | ART-S10, ART-S01 |
| PE resource type RT_GROUP_ICON | Icon-group resource type 14 | ART-S10 |
| PE resource type RT_ICON | Individual icon resource type 3 | ART-S10 |
| PE icon group/name/ID | Real component icon identity; chosen ID is not a globally registered product ID | ART-S10, ART-S11 |
| PE resource type RT_MANIFEST | Native manifest resource type 24 | ART-S10 |
| RT_MANIFEST resource ID | EXE convention 1; DLL convention 2 for private dependencies under documented linker behavior | ART-S12 |
| Recorded PDB location | Inspect emitted debug data for workstation paths; linker-specific alternate-path support exists | ART-S29 |

Use approved ICO variants and actual resource-language mappings. Do not infer a “service icon” field in SCM from an EXE icon. Recording an alternate PDB reference does not move the PDB itself; validate actual toolchain behavior before choosing a flag. No full PE header/debug-directory/resource-format inventory is claimed here.

### B.5. Native application manifest identity and presentation

These paths use the native assembly namespace, not AppxManifest.xml and not ClickOnce’s deployment manifest.

| Canonical field | Treatment | Sources |
| --- | --- | --- |
| assembly/@manifestVersion | N: 1.0 | ART-S13 |
| assembly/assemblyIdentity/@type | B: win32 in current application-manifest guidance | ART-S13 |
| assembly/assemblyIdentity/@name | B: stable component assembly identity | ART-S13 |
| assembly/assemblyIdentity/@language | C: actual language; omit for a language-neutral application per current guidance | ART-S13 |
| assembly/assemblyIdentity/@processorArchitecture | B/D: actual target; native token amd64 differs from MSIX x64 | ART-S13 |
| assembly/assemblyIdentity/@version | B/D: four components, each 0–65535 | ART-S13 |
| assembly/assemblyIdentity/@publicKeyToken | C: actual shared side-by-side assembly identity; never invent from a certificate thumbprint | ART-S13 |
| assembly/description | A/C: optional short purpose text in the published native manifest schema | ART-S14 |

The older native schema explicitly contains description, while the modern application-manifest overview omits it. The schema also differs from current prose on whether type is mandatory. Treat the current application guidance as the authoring baseline, and validate optional description against the selected SDK/output path. No guarantee that Explorer or Services automatically displays this element. Do not import ClickOnce publisher/product/supportUrl attributes into this native element.

Behavioral boundary: noInherit; compatibility/application/supportedOS and maxversiontested; dependency/dependentAssembly; file/COM/activation declarations; trustInfo/requestedExecutionLevel/uiAccess; and application/windowsSettings declarations for codepage, elevation, theming, window filtering, DPI, scrolling, long paths, printer isolation, MSIX identity lookup, heap type, architectures and console allocation all require their existing owner’s behavioral review. Their nested schemas are not additional descriptive checkboxes. ART-S13 and ART-S14 are the reference entry points; this section does not enumerate the separate COM/ClickOnce ecosystem.

### B.6. MSIX package and application metadata

Namespaces are part of field identity. Prefix spelling is an alias, not the namespace identity. The ordinary unprefixed paths below use http://schemas.microsoft.com/appx/manifest/foundation/windows10. Namespace declarations themselves are not additional metadata attributes.

#### B.6.1. All Identity attributes

| Canonical field | Treatment | Sources |
| --- | --- | --- |
| Package/Identity/@Name | B: exact assigned/admitted name; schema 3–50 permitted identifier characters | ART-S15, ART-S19, ART-S20 |
| Package/Identity/@ProcessorArchitecture | B/D: actual x86, x64, arm, arm64, x86a64 or neutral classification; presence/support depends on package kind | ART-S15, ART-S20 |
| Package/Identity/@Publisher | B: exact admitted distinguished name; certificate/Store identity rules apply | ART-S15, ART-S20, ART-S34 |
| Package/Identity/@Version | B/D: four 0–65535 components, with release/channel restrictions | ART-S15, ART-S20, ART-S35 |
| Package/Identity/@ResourceId | C: actual resource identity; schema at most 30 permitted identifier characters | ART-S15, ART-S20 |

Publisher is not PublisherDisplayName. Package family name, package full name, publisher ID, package SID and app-user-model identity are derived/read-back identities, not invented additional Identity attributes. Store guidance for Windows 10/11 UWP package numbering reserves the fourth component and requires it to be zero in submissions, with a nonzero major component. Keep that documented channel constraint qualified; do not impose it blindly on every packaging route. [ART-S34, ART-S35]

#### B.6.2. All Properties direct children

Properties has no authored attributes in the pinned foundation schema. The following 24 rows cover its concrete direct children. Only the four ordinary descriptive fields merit population just for presentation.

| Canonical field | Disposition and rule |
| --- | --- |
| Package/Properties/Framework | B/C: framework-package classification |
| Package/Properties/DisplayName | A: package name/localizable resource; NevroTick |
| Package/Properties/PublisherDisplayName | A: admitted publisher display spelling |
| Package/Properties/Description | A: concise package purpose |
| Package/Properties/Logo | A/D: real qualified package image |
| Package/Properties/ResourcePackage | B/C: resource-only package classification |
| Package/Properties/uap:SupportedUsers | B/C: supported-user execution model |
| Package/Properties/uap6:AllowExecution | B/C: execution policy |
| Package/Properties/desktop6:RegistryWriteVirtualization | B/C: registry virtualization policy |
| Package/Properties/desktop6:FileSystemWriteVirtualization | B/C: filesystem virtualization policy |
| Package/Properties/virtualization:FileSystemWriteVirtualization | B/C: newer virtualization schema variant |
| Package/Properties/virtualization:RegistryWriteVirtualization | B/C: newer registry virtualization variant |
| Package/Properties/rescap6:ModificationPackage | B/C: modification-package classification |
| Package/Properties/uap10:AllowExternalContent | B/C: external-location behavior |
| Package/Properties/uap10:PackageIntegrity | B/C: package-integrity policy subtree |
| Package/Properties/uap13:AutoUpdate | B/C: update configuration subtree |
| Package/Properties/uap15:DependencyTarget | B/C: dependency-target policy |
| Package/Properties/uap16:UpdateWhileInUse | B/C: documented schema member; do not assume route/OS support |
| Package/Properties/uap17:UpdateWhileInUse | B/C: distinct namespace variant; qualify support |
| Package/Properties/heap:HeapPolicy | B/C: heap behavior |
| Package/Properties/deployment2:StageWhileInUse | B/C: staging behavior; schema presence alone insufficient |
| Package/Properties/trustedlaunch:TrustedLaunch | B/C: trust/launch policy subtree |
| Package/Properties/accesscontrol:AccessControl | B/C: concrete AccessControlChoice substitution; registration access-control subtree |
| Package/Properties/appdata:ApplicationData | B/C: concrete ApplicationDataChoice substitution; machine-data subtree |

Sources: ART-S16, ART-S19–ART-S22. Last two prefixes are aliases chosen here for the exact namespaces http://schemas.microsoft.com/appx/manifest/accesscontrol/windows10 and http://schemas.microsoft.com/appx/manifest/appdata/windows10. AccessControlChoice and ApplicationDataChoice are **abstract schema placeholders**, not XML elements to emit.

For boundary precision, AccessControl currently contains optional Register with required Sddl; ApplicationData contains optional MachineFolder with optional Sddl. These are access-control decisions, not metadata enrichment. No adoption proposed. [ART-S21, ART-S22]

Literal DisplayName/PublisherDisplayName use the schema’s display-name type, including resource-reference support; the literal branch permits at most 256 characters. Description uses the 1–2048 description type. Check the actual chosen language/resource resolution too. This precision comes from the SDK type definitions, rather than copying unrelated field limits. [ART-S20]

#### B.6.3. All Application attributes, including inherited attributes

Path prefix: Package/Applications/Application. Every row is identity, activation or runtime behavior; do not add attributes just for a larger field count.

| Canonical field | Treatment |
| --- | --- |
| Application/@Id | B: stable package-relative app identity; schema at most 64 characters |
| Application/@Executable | B/D: actual payload executable |
| Application/@EntryPoint | B: actual activation entry point |
| Application/@StartPage | B/C: web/JavaScript application route only when implemented |
| Application/@ResourceGroup | B/C: resource-management group |
| Application/@desktop4:Subsystem | B/C: console/windows behavior |
| Application/@iot2:Subsystem | B/C: IoT namespace variant |
| Application/@uap10:Subsystem | B/C: current namespace variant |
| Application/@desktop4:SupportsMultipleInstances | B/C: instance-lifetime behavior |
| Application/@iot2:SupportsMultipleInstances | B/C: IoT namespace variant |
| Application/@uap10:SupportsMultipleInstances | B/C: current namespace variant |
| Application/@uap11:CurrentDirectoryPath | B/C: initial working-directory contract |
| Application/@uap11:Parameters | B/C: launch parameters |
| Application/@previewsecurity:TrustLevel | B/C: preview security model; catalogue only |
| Application/@uap10:TrustLevel | B/C: appContainer/mediumIL under selected design |
| Application/@uap10:RuntimeBehavior | B/C: windowsApp/packagedClassicApp/win32App model |
| Application/@uap10:HostId | B/C: actual host-runtime association |
| Application/@uap10:Parameters | B/C: distinct namespace launch-parameter field |
| Application/@previewsecurity2:RuntimeBehavior | B/C: preview runtime model; catalogue only |
| Application/@uap16:BaseNamedObjectsIsolation | B/C: named-object isolation |
| Application/@uap17:BaseNamedObjectsIsolation | B/C: distinct namespace isolation variant |
| Application/@desktop11:AppLifecycleBehavior | B/C: system-managed/unmanaged lifecycle |
| Application/@previewappcompat:CompatMode | B/C: inherited preview field; classic/modern; catalogue only |
| Application/@previewappcompat:Scope | B/C: inherited preview field; machine/user; catalogue only |

Sources: ART-S17, ART-S19, ART-S20, ART-S25, ART-S26. The last two are absent from the current Learn attribute table but are inherited through uap10:TrustLevelGroup → previewappcompat:PreviewEntryPointAttributesGroup. Exact previewappcompat namespace: http://schemas.microsoft.com/appx/manifest/preview/windows10/msixappcompatsupport. Six fields in the expanded uap10 group are counted once, not both as group members and independent extra attributes.

The current Learn prose and pinned SDK differ on HostId limits/patterns: the SDK’s host-runtime type permits a dotted identifier up to 256 characters. Do not freeze copied limits from the generated page without selected-SDK validation. Activation/trust/entry-point combinations also have semantic constraints; schema membership is not proof that arbitrary combinations work.

#### B.6.4. All Application direct-child names

| Canonical field | Treatment | Sources |
| --- | --- | --- |
| Application/uap:VisualElements | A/B: ordinary visual declaration | ART-S19, ART-S23 |
| Application/uap3:VisualElements | A/B/C: alternative visual declaration, inherits ordinary fields | ART-S19, ART-S24 |
| Application/uap:ApplicationContentUriRules | B/C: web-content access/activation rules | ART-S19 |
| Application/Extensions | B/C: implemented integration declarations; open extension universe not expanded | ART-S19, ART-S43 |
| Application/uap7:Properties | B/C: app behavior properties | ART-S19, ART-S27 |

VisualElementsChoice is abstract; the first two are alternatives, not an instruction to insert both. uap7:Properties has no attributes and three direct children in the cited schema: ImportRedirectionTable, uap8:ActiveCodePage and uap10:LanguagePreference. They are behavior rather than marketing descriptions. [ART-S27]

#### B.6.5. All uap:VisualElements attributes and direct children

| Canonical field | Treatment | Sources |
| --- | --- | --- |
| uap:VisualElements/@DisplayName | A: actual app label; 1–256 literal characters | ART-S18, ART-S20 |
| uap:VisualElements/@Description | A: app-specific description; 1–2048 characters | ART-S18 |
| uap:VisualElements/@BackgroundColor | A/B: valid schema color consistent with assets | ART-S18 |
| uap:VisualElements/@Square150x150Logo | A/D: real image/resource path | ART-S18 |
| uap:VisualElements/@Square44x44Logo | A/D: real image/resource path | ART-S18 |
| uap:VisualElements/@AppListEntry | B/C: default/none shell visibility | ART-S18 |
| uap:VisualElements/uap:DefaultTile | A/B/C: actual tile assets/behavior | ART-S18, ART-S23 |
| uap:VisualElements/uap:LockScreen | A/B/C: only implemented lock-screen integration | ART-S18 |
| uap:VisualElements/uap:SplashScreen | A/B/C: actual splash presentation | ART-S18 |
| uap:VisualElements/uap:InitialRotationPreference | B/C: actual rotation behavior | ART-S18 |
| uap3:VisualElements/@VisualGroup | A/B/C: one additional grouping attribute; inherits all preceding base attributes/children | ART-S24 |

Logo paths must satisfy the schema and refer to PNG/JPG/JPEG assets, not AVIF masters. A headless service does not need a fabricated Start-menu identity or splash screen; preserve the selected package/UI/service model. Child subtrees, including tile updates and mixed-reality content, are not recursively expanded here.

#### B.6.6. Resource and target declarations already relevant to the plan

| Canonical field | Treatment | Sources |
| --- | --- | --- |
| Package/Resources/Resource/@Language | A/D: real supported resource language | ART-S19 |
| Package/Resources/Resource/@uap:Scale | D/C: actual resource-scale qualification | ART-S19, ART-S23 |
| Package/Resources/Resource/@uap:DXFeatureLevel | B/D/C: actual graphics resource qualification | ART-S19, ART-S23 |
| Package/Dependencies/TargetDeviceFamily/@Name | B: admitted family, not a claim of testing | ART-S19 |
| Package/Dependencies/TargetDeviceFamily/@MinVersion | B/D: real minimum version | ART-S19 |
| Package/Dependencies/TargetDeviceFamily/@MaxVersionTested | D: actual tested target declaration; not an upper support bound | ART-S19 |
| Package/@IgnorableNamespaces | B/C: namespace compatibility handling, not permission to invent fields | ART-S19 |

Conditional service, COM, protocol, file-association, startup, capability and dependency schemas remain linked through the official schema root, ART-S43. Do not recursively count that entire registry as service metadata. The desktop6 service schema’s lack of DisplayName/Description attributes does **not** mean packaged service descriptions are impossible: Microsoft’s Packaging Tool Services report exposes editable Description and Display name. The actual admitted package-generation route must prove how those values survive deployment. [ART-S42]

### B.7. Authenticode content-description fields

| Canonical field | Treatment | Source |
| --- | --- | --- |
| SignTool.sign./d | A/C: short description of the signed content | ART-S28 |
| SignTool.sign./du | A/C: real public information URL for that content | ART-S28 |

Source for both: ART-S28. SignTool verification’s /d option can display the description and description URL. These are distinct from PE FileDescription, certificate subject and Store listing text. Adopt only if the admitted signing route supports and retains them; final signed/Store-returned bytes require readback. Do not select a certificate, signer, timestamp service or new dependency here.

Certificate subject/issuer/serial/thumbprint, signature algorithms, signing/timestamp observations and verification results belong in measured release evidence. They are not author-filled claims of authenticity. This is not a complete X.509/PKCS#7 attribute catalogue. Package signing also does not automatically establish that every payload PE carries its own embedded signature.

### B.8. Conditional MSI inventory

MSI is not the selected NevroTick delivery route. Keep this section conditional; no MSI authoring proposal is made. The 22 property names below cover the plan’s product/installed-app descriptive identity plus the entire documented ARP property list and SettingsIdentifier mapping. They do not cover all Windows Installer properties/actions.

#### B.8.1. Product and ARP authoring fields

| Canonical field | Treatment | Sources |
| --- | --- | --- |
| MSI.Property.ProductName | A: NevroTick product display name | ART-S37 |
| MSI.Property.Manufacturer | A: actual publisher, David Osipov if still approved | ART-S37 |
| MSI.Property.ProductVersion | B/D: three numeric components; 255.255.65535 maxima | ART-S41 |
| MSI.Property.ProductLanguage | B/D: actual installer language | ART-S37 |
| MSI.Property.ProductCode | B: update-governed product GUID | ART-S45 |
| MSI.Property.UpgradeCode | B: admitted related-product family GUID | ART-S44 |
| MSI.Property.ARPAUTHORIZEDCDFPREFIX | B/C: actual update-channel URL | ART-S36 |
| MSI.Property.ARPCOMMENTS | A/C: useful installed-product description | ART-S36 |
| MSI.Property.ARPCONTACT | A/C: actual public support contact | ART-S36 |
| MSI.Property.ARPINSTALLLOCATION | D: resolved installed primary directory | ART-S36 |
| MSI.Property.ARPHELPLINK | A/C: maintained support URL | ART-S36 |
| MSI.Property.ARPHELPTELEPHONE | A/C: real public support telephone or omit | ART-S36 |
| MSI.Property.ARPNOMODIFY | B/C: maintenance-control visibility; not hardening | ART-S36 |
| MSI.Property.ARPNOREMOVE | B/C: removal-control visibility; not an uninstall security boundary | ART-S36 |
| MSI.Property.ARPNOREPAIR | B/C: repair-control visibility | ART-S36 |
| MSI.Property.ARPPRODUCTICON | A/D: actual Icon-table reference | ART-S36 |
| MSI.Property.ARPREADME | A/C: real readme destination | ART-S36 |
| MSI.Property.ARPSIZE | D/C: justified estimate in KB; prefer installer calculation where appropriate | ART-S36 |
| MSI.Property.ARPSYSTEMCOMPONENT | B/C: installed-app visibility; do not hide ordinary product for completeness | ART-S36 |
| MSI.Property.ARPURLINFOABOUT | A/C: maintained product URL | ART-S36 |
| MSI.Property.ARPURLUPDATEINFO | A/C: maintained update-information URL | ART-S36 |
| MSI.Property.MSIARPSETTINGSIDENTIFIER | B/C: actual settings-registry locations for migration; omit if inapplicable | ART-S38 |

All ARP rows without an explicit per-row source use ART-S36; the last row uses ART-S38. MSIARPSETTINGSIDENTIFIER is a semicolon-separated actual registry-location list, not a friendly settings description; if present it must not be null. No private contact data should be filled automatically.

#### B.8.2. All 22 named values in the documented MSI Uninstall mapping

Source: ART-S37. These are installer-generated mappings/observations, not a competing hand-maintained registry registration.

| Canonical field | Origin/treatment |
| --- | --- |
| MSI.Uninstall.DisplayName | ProductName |
| MSI.Uninstall.DisplayVersion | ProductVersion projection |
| MSI.Uninstall.Publisher | Manufacturer |
| MSI.Uninstall.VersionMinor | ProductVersion projection |
| MSI.Uninstall.VersionMajor | ProductVersion projection |
| MSI.Uninstall.Version | ProductVersion projection |
| MSI.Uninstall.HelpLink | ARPHELPLINK |
| MSI.Uninstall.HelpTelephone | ARPHELPTELEPHONE |
| MSI.Uninstall.InstallDate | Servicing-sensitive installer observation |
| MSI.Uninstall.InstallLocation | ARPINSTALLLOCATION |
| MSI.Uninstall.InstallSource | SourceDir observation |
| MSI.Uninstall.URLInfoAbout | ARPURLINFOABOUT |
| MSI.Uninstall.URLUpdateInfo | ARPURLUPDATEINFO |
| MSI.Uninstall.AuthorizedCDFPrefix | ARPAUTHORIZEDCDFPREFIX |
| MSI.Uninstall.Comments | ARPCOMMENTS |
| MSI.Uninstall.Contact | ARPCONTACT |
| MSI.Uninstall.EstimatedSize | Installer-derived |
| MSI.Uninstall.Language | ProductLanguage |
| MSI.Uninstall.ModifyPath | Installer-derived |
| MSI.Uninstall.Readme | ARPREADME |
| MSI.Uninstall.UninstallString | Installer-derived |
| MSI.Uninstall.SettingsIdentifier | MSIARPSETTINGSIDENTIFIER |

InstallDate can change after repair/patch servicing; it is not immutable original-install provenance. This is the cited Windows Installer mapping, not an exhaustive list of arbitrary Uninstall keys other installers may create.

#### B.8.3. All 17 installation-database Summary Information properties

The PID is part of the property identity. Semantics below are for an MSI installation database, not MST transforms or MSP patches. Sources: ART-S39, ART-S40.

| Canonical field | PID/type | Treatment |
| --- | --- | --- |
| MSI.Summary.Codepage | 1 / VT_I2 | D: authoring codepage, set before strings |
| MSI.Summary.Title | 2 / VT_LPSTR | A: meaningful installation-database title |
| MSI.Summary.Subject | 3 / VT_LPSTR | A: product name |
| MSI.Summary.Author | 4 / VT_LPSTR | A: actual manufacturer/publisher |
| MSI.Summary.Keywords | 5 / VT_LPSTR | A: relevant installer/product terms |
| MSI.Summary.Comments | 6 / VT_LPSTR | A: accurate installer purpose |
| MSI.Summary.Template | 7 / VT_LPSTR | B/D: actual platform/languages |
| MSI.Summary.LastSavedBy | 8 / VT_LPSTR | N: null in final shipping installation database |
| MSI.Summary.RevisionNumber | 9 / VT_LPSTR | B/D: actual package-code GUID, not source revision |
| MSI.Summary.LastPrinted | 11 / VT_FILETIME | C/N: administrative-image semantics; no invented date |
| MSI.Summary.CreateTimeDate | 12 / VT_FILETIME | D: actual authoring timestamp under reproducibility policy |
| MSI.Summary.LastSaveTimeDate | 13 / VT_FILETIME | D/C: actual save timestamp under policy |
| MSI.Summary.PageCount | 14 / VT_I4 | B/D: minimum installer-engine encoding |
| MSI.Summary.WordCount | 15 / VT_I4 | B/D: source-image/installer flags |
| MSI.Summary.CharacterCount | 16 / VT_I4 | N: null for installation package |
| MSI.Summary.CreatingApplication | 18 / VT_LPSTR | D: actual authoring software |
| MSI.Summary.Security | 19 / VT_I4 | B: recommended-read-only value 2; not cryptographic protection |

Numeric MSI ProductVersion ignores a fourth component; do not map SemVer prerelease text directly into it. MSI package code, product code, upgrade family, product version, creation date, signature timestamp and servicing date are separate concepts. [ART-S41, ART-S44–ART-S46; repository §7]

### B.9. Conditional Microsoft Store descriptive inventory

Paths in this subsection identify Partner Center UI fields, not a promised submission API JSON schema. Values remain pending actual release, account and route qualification. Do not create an independent Store identity or publish anything.

#### B.9.1. Listing and additional information

| Field path / documented UI concept | Treatment/checked bound | Sources |
| --- | --- | --- |
| StoreListing[locale].Language | Real listing locale; does not prove translated UI | ART-S30 |
| StoreListing[locale].Product name | Choose a reserved product name | ART-S30 |
| StoreListing[locale].Description | Actual capabilities; plain text, ≤10,000 characters | ART-S30 |
| StoreListing[locale].What's new in this version | Actual update changes; ≤1,500; blank for first submission | ART-S30 |
| StoreListing[locale].Product features | ≤20 entries, each ≤200 characters | ART-S30 |
| StoreListing[locale].Screenshots | Actual app/device presentation; at least one for submission | ART-S30 |
| StoreListing[locale].Store logos | Actual approved branding assets | ART-S30 |
| StoreListing[locale].Use only uploaded Store images | Conditional presentation preference; not a package-logo field | ART-S30 |
| StoreListing[locale].Trailers | Actual optional video assets | ART-S30 |
| StoreListing[locale].Additional assets | Actual optional promotional images | ART-S30 |
| StoreListing[locale].Short title | ≤50 characters; conditional presentation usefulness | ART-S30 |
| StoreListing[locale].Sort title | ≤255 characters | ART-S30 |
| StoreListing[locale].Voice title | ≤255 characters; only useful supported context | ART-S30 |
| StoreListing[locale].Short description | ≤1,000; Microsoft recommends under 270 | ART-S30 |
| StoreListing[locale].Additional system requirements.Minimum hardware | ≤11 actual requirements, each ≤200 characters | ART-S30 |
| StoreListing[locale].Additional system requirements.Recommended hardware | ≤11 actual recommendations, each ≤200 characters | ART-S30 |
| StoreAdditional.Keywords | ≤7 terms, ≤40 characters each, ≤21 total words | ART-S31 |
| StoreAdditional.Copyright and trademark info | Truthful rights text, ≤200 characters | ART-S31 |
| StoreAdditional.Additional license terms | Admitted license; URL or ≤10,000 text characters | ART-S31 |
| StoreAdditional.Developed by | Optional real attribution, ≤255 characters | ART-S31 |
| StoreListing.Published by | D: account publisher display name; distinct from Developed by | ART-S31 |

The asset rows enumerate descriptive fieldsets, not every per-device upload slot, dimension, trailer codec or screenshot caption. Those mutable asset specifications remain under the screenshots/images link reached from ART-S30; re-read for the actual device/channel. No game or Xbox support inferred.

#### B.9.2. Product properties, support and separate regulatory contact fields

| Field path / documented UI concept | Treatment | Sources |
| --- | --- | --- |
| StoreProperties.Category | Required truthful category | ART-S32 |
| StoreProperties.Subcategory/genre | Actual applicable subcategory | ART-S32 |
| StoreProperties.Secondary category | Optional truthful second category | ART-S32 |
| StoreProperties.Accesses, collects or transmits personal information | Actual behavior/capabilities declaration | ART-S33 |
| StoreProperties.Privacy policy URL | Actual maintained policy when chosen/required | ART-S33 |
| StoreProperties.Privacy policy text | Alternative supported by current guidance; real policy, not fabricated | ART-S33 |
| StoreSupport.Website | Actual product website; not the app’s Store listing URL | ART-S33 |
| StoreSupport.Support contact info | Maintained support page or public support email | ART-S33 |
| StoreRegulatory.Phone number | Conditional regulatory contact; do not infer from private data | ART-S33 |
| StoreRegulatory.Address | Conditional regulatory contact | ART-S33 |
| StoreRegulatory.Apartment / Suite | Conditional regulatory contact | ART-S33 |
| StoreRegulatory.City | Conditional regulatory contact | ART-S33 |
| StoreRegulatory.State / Province | Conditional regulatory contact | ART-S33 |
| StoreRegulatory.Country | Conditional regulatory contact | ART-S33 |
| StoreRegulatory.Postal code | Conditional regulatory contact | ART-S33 |
| StoreProperties.System requirements | Actual device/hardware requirement fieldset | ART-S32 |
| StoreProperties.Game settings | C/N: only if the product is actually a game | ART-S32 |
| StoreProperties.Display mode | B/C: only implemented capabilities; no invented mixed-reality/HDR/4K claims | ART-S32 |
| StoreProperties.Product declarations | B/C: actual submission facts and capabilities | ART-S32 |

Last three rows use ART-S32. These behavioral fieldsets are named for completeness at this boundary but not recursively expanded; neither are age-rating questionnaires, market distribution, prices, tax, payments, certification credentials or account identity. Separate regulatory contact is not ordinary public Store marketing text. Current guidance says it is generally not displayed to browsing customers; do not promise secrecy or auto-fill personal information. [ART-S33]

### B.10. Existing coverage, useful additions and qualification gaps

**Already canonical:** per-component VERSIONINFO; the 12 conventional strings; versions/flags/locales/icons; shared value provenance; meaningful package/visual descriptions; actual identity and public URLs; Store text; conditional MSI product/ARP/Summary fields; custom-key limitations; output readback before/after signing; privacy and reproducibility boundaries. Do not present this whole inventory as newly discovered missing work.

**Useful additions/refinements:** all structural fields with units; service EXE VFT_APP/subtype zero; INFOINFERRED exclusion; conditional Authenticode description/URL; native description with documented-source discrepancy; concrete MSIX choice substitutions; inherited preview attributes; exact per-node counts; MSI SettingsIdentifier; current Store privacy-text alternative and separate regulatory contacts. Richer coverage requires deliberate “not applicable” decisions as often as new populated values.

**Still pending:** frozen output filenames, release version mapping, actual Store identity/publisher/URLs, emitted metadata/localization assets, selected SDK schema qualification, signer-route support for descriptive attributes, signed/package/installed/delivered readback. No implementation or Windows result is claimed. The inventory deliberately excludes open custom keys, all extension descendants, complete certificate structures and unrelated packaging ecosystems.

### B.11. Primary source index

The supplied research records these Microsoft Learn sources. Integration rechecked selected consequential contracts and pinned schema membership, not every listed page. GitHub URLs identify the retained Microsoft schema snapshot.

| ID | Exact URL |
| --- | --- |
| ART-S01 | [ART-S01 source](https://learn.microsoft.com/en-us/windows/win32/menurc/versioninfo-resource) |
| ART-S02 | [ART-S02 source](https://learn.microsoft.com/en-us/windows/win32/menurc/stringfileinfo-block) |
| ART-S03 | [ART-S03 source](https://learn.microsoft.com/en-us/windows/win32/api/verrsrc/ns-verrsrc-vs_fixedfileinfo) |
| ART-S04 | [ART-S04 source](https://learn.microsoft.com/en-us/windows/win32/menurc/vs-versioninfo) |
| ART-S05 | [ART-S05 source](https://learn.microsoft.com/en-us/windows/win32/menurc/stringfileinfo) |
| ART-S06 | [ART-S06 source](https://learn.microsoft.com/en-us/windows/win32/menurc/stringtable) |
| ART-S07 | [ART-S07 source](https://learn.microsoft.com/en-us/windows/win32/menurc/string-str) |
| ART-S08 | [ART-S08 source](https://learn.microsoft.com/en-us/windows/win32/menurc/varfileinfo) |
| ART-S09 | [ART-S09 source](https://learn.microsoft.com/en-us/windows/win32/menurc/var-str) |
| ART-S10 | [ART-S10 source](https://learn.microsoft.com/en-us/windows/win32/menurc/resource-types) |
| ART-S11 | [ART-S11 source](https://learn.microsoft.com/en-us/windows/win32/menurc/about-icons) |
| ART-S12 | [ART-S12 source](https://learn.microsoft.com/en-us/cpp/build/reference/manifest-create-side-by-side-assembly-manifest?view=msvc-170) |
| ART-S13 | [ART-S13 source](https://learn.microsoft.com/en-us/windows/win32/sbscs/application-manifests) |
| ART-S14 | [ART-S14 source](https://learn.microsoft.com/en-us/windows/win32/sbscs/manifest-file-schema) |
| ART-S15 | [ART-S15 source](https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/element-f-identity) |
| ART-S16 | [ART-S16 source](https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/element-f-properties) |
| ART-S17 | [ART-S17 source](https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/element-application) |
| ART-S18 | [ART-S18 source](https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/element-uap-visualelements) |
| ART-S19 | [ART-S19 source](https://github.com/microsoft/win32metadata/blob/15fc870a5e96df7c560877ff30136c0fb5a21e9b/generation/WinSDK/RecompiledIdlHeaders/winrt/FoundationManifestSchema.xsd) |
| ART-S20 | [ART-S20 source](https://github.com/microsoft/win32metadata/blob/15fc870a5e96df7c560877ff30136c0fb5a21e9b/generation/WinSDK/RecompiledIdlHeaders/winrt/AppxManifestTypes.xsd) |
| ART-S21 | [ART-S21 source](https://github.com/microsoft/win32metadata/blob/15fc870a5e96df7c560877ff30136c0fb5a21e9b/generation/WinSDK/RecompiledIdlHeaders/winrt/AccessControlManifestSchema.xsd) |
| ART-S22 | [ART-S22 source](https://github.com/microsoft/win32metadata/blob/15fc870a5e96df7c560877ff30136c0fb5a21e9b/generation/WinSDK/RecompiledIdlHeaders/winrt/AppDataManifestSchema.xsd) |
| ART-S23 | [ART-S23 source](https://github.com/microsoft/win32metadata/blob/15fc870a5e96df7c560877ff30136c0fb5a21e9b/generation/WinSDK/RecompiledIdlHeaders/winrt/UapManifestSchema.xsd) |
| ART-S24 | [ART-S24 source](https://github.com/microsoft/win32metadata/blob/15fc870a5e96df7c560877ff30136c0fb5a21e9b/generation/WinSDK/RecompiledIdlHeaders/winrt/UapManifestSchema_v3.xsd) |
| ART-S25 | [ART-S25 source](https://github.com/microsoft/win32metadata/blob/15fc870a5e96df7c560877ff30136c0fb5a21e9b/generation/WinSDK/RecompiledIdlHeaders/winrt/UapManifestSchema_v10.xsd) |
| ART-S26 | [ART-S26 source](https://github.com/microsoft/win32metadata/blob/15fc870a5e96df7c560877ff30136c0fb5a21e9b/generation/WinSDK/RecompiledIdlHeaders/winrt/PreviewManifestSchema_MsixAppCompatSupport.xsd) |
| ART-S27 | [ART-S27 source](https://github.com/microsoft/win32metadata/blob/15fc870a5e96df7c560877ff30136c0fb5a21e9b/generation/WinSDK/RecompiledIdlHeaders/winrt/UapManifestSchema_v7.xsd) |
| ART-S28 | [ART-S28 source](https://learn.microsoft.com/en-us/windows/win32/seccrypto/signtool) |
| ART-S29 | [ART-S29 source](https://learn.microsoft.com/en-us/cpp/build/reference/pdbaltpath-use-alternate-pdb-path?view=msvc-170) |
| ART-S30 | [ART-S30 source](https://learn.microsoft.com/en-us/windows/apps/publish/publish-your-app/msix/add-and-edit-store-listing-info) |
| ART-S31 | [ART-S31 source](https://learn.microsoft.com/en-us/windows/apps/publish/publish-your-app/msix/add-additional-information) |
| ART-S32 | [ART-S32 source](https://learn.microsoft.com/en-us/windows/apps/publish/publish-your-app/msix/enter-app-properties) |
| ART-S33 | [ART-S33 source](https://learn.microsoft.com/en-us/windows/apps/publish/publish-your-app/msix/support-info) |
| ART-S34 | [ART-S34 source](https://learn.microsoft.com/en-us/windows/apps/publish/view-app-identity-details) |
| ART-S35 | [ART-S35 source](https://learn.microsoft.com/en-us/windows/apps/publish/publish-your-app/app-package-requirements?pivots=store-installer-msix) |
| ART-S36 | [ART-S36 source](https://learn.microsoft.com/en-us/windows/win32/msi/configuring-add-remove-programs-with-windows-installer) |
| ART-S37 | [ART-S37 source](https://learn.microsoft.com/en-us/windows/win32/msi/uninstall-registry-key) |
| ART-S38 | [ART-S38 source](https://learn.microsoft.com/en-us/windows/win32/msi/msiarpsettingsidentifier) |
| ART-S39 | [ART-S39 source](https://learn.microsoft.com/en-us/windows/win32/msi/summary-property-descriptions) |
| ART-S40 | [ART-S40 source](https://learn.microsoft.com/en-us/windows/win32/msi/summary-information-stream-property-set) |
| ART-S41 | [ART-S41 source](https://learn.microsoft.com/en-us/windows/win32/msi/productversion) |
| ART-S42 | [ART-S42 source](https://learn.microsoft.com/en-us/windows/msix/packaging-tool/create-app-package) |
| ART-S43 | [ART-S43 source](https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/schema-root) |
| ART-S44 | [ART-S44 source](https://learn.microsoft.com/en-us/windows/win32/msi/upgradecode) |
| ART-S45 | [ART-S45 source](https://learn.microsoft.com/en-us/windows/win32/msi/productcode) |
| ART-S46 | [ART-S46 source](https://learn.microsoft.com/en-us/windows/win32/msi/package-codes) |

---

## Appendix C: Diagnostics and performance counters

Research snapshot: 7 November 2026, `df6e28a4641f2f046373a9c5640aa60736716a22`. Schema inventory; no manifest compilation, registration, publication or PerfLib execution qualified.

### C.1. Scope and recommendation

This catalogue covers the **named attributes and direct child declarations of 37 Microsoft-documented XML complex types, three scoped anonymous types, all seven EVENT_DESCRIPTOR members, and a two-parameter runtime correlation slice**. It also records 18 XML extension points. It is complete within that declared boundary: **196 catalogue rows = 115 named XML attributes + 54 named XML children + 18 extension points + 7 descriptor members + 2 correlation parameters**. Repeated elements count once. This is not a catalogue of every Windows diagnostic API, XML vocabulary, runtime property, or SDK revision.

For NevroTick, useful additions are precise field ownership and readback requirements, not new telemetry infrastructure:

- Preserve fixed provider identity; explicitly include a maintained `helpLink`, localized provider/channel labels, and distinct metadata/message/parameter resource roles. `EvtPublisherMetadataHelpLink` exposes the help link as a string; localized message IDs are resolved through `EvtFormatMessage`. No claim that SCM or every Event Viewer screen displays the URL. [DIA-S01] [DIA-S02] [DIA-S40]
- Complete the existing event catalogue: stable IDs and versions, meaningful levels/keywords, useful task/opcode labels, typed ordered templates, and localized messages/maps. Populate applicable fields; do not invent levels, events or payloads to fill optional slots. [DIA-S05]–[DIA-S19] [DIA-S20]
- Treat activity correlation as per-operation runtime data. Static provider identity cannot substitute for `ActivityId` or a real parent relationship. [DIA-S21]
- If the already-optional PerfLib interface is implemented, provide truthful counter-set/counter names and descriptions, stable identifiers, actual calculation types, and correct dependent-counter references. The reviewed counter schema has **no generic `units` attribute**; explain units in the owned metric contract and descriptive text. Do not add counters solely to obtain more fields. [DIA-S22]–[DIA-S26]
- Retain installer-owned resources, privacy filtering, bounded operational sinks, and candidate-specific readback under the existing owners. This section does not select channel count, isolation, access, retention, callbacks, aggregation, or tracing technology.

Existing repository coverage: `docs/plans/windows-package-metadata.md` §§4/11 already separates intended facts and observed artefacts; `.agents/skills/implementation/maintain-windows-metadata/SKILL.md` owns the metadata workflow; `.agents/skills/security/references/windows-host-integration.md` already covers provider identity, installed resources, rendering, channel/access boundaries and optional ETW; architecture §12.3.4 makes PerfLib optional. W11-W08 owns diagnostics semantics, W11-W10/P6 own metadata/release evidence, W11-W05 owns service effects. These are research increments to those authorities, not replacement plans.

#### C.1.1. Interpretation

- **Recommended:** useful for an already-applicable surface; still derive the value from its owner. It does not mean every optional feature becomes mandatory.
- **Conditional:** use only when the owned schema, feature or operating policy needs it. All PerfLib entries remain conditional on that interface.
- **Runtime:** per-event value, not install-time product metadata.
- **Internal/unused:** documented reserved or unused; catalogue presence is not permission to populate.
- **Not-applicable:** legacy, unsupported or incompatible with the current user-mode service profile.
- `EM.Type/field` and `PERF.Type/field` are **type-qualified catalogue keys**, not literal document XPaths. `@` means XML attribute; a bare name is a direct child declaration. `*{##other}` and `@*{##other}` denote foreign-namespace extension slots, not named product fields.
- Named child declarations inside XSD sequences/choices count as the type's direct children. Key/keyref/unique constraints, occurrence limits, XML namespace declarations and XSD annotations are grammar, not extra product fields. Simple-content string bases and mixed-content allowances add no separately documented metadata semantics.
- Reused types are listed once: `DataDefinitionType` applies at template/data and struct/data; `OpcodeListType` applies at provider/opcodes and task/opcodes. Anonymous `binary`, `resources`, and `string` attributes are counted only at their scoped paths. No named inherited attributes were found in the reviewed types.

### C.2. Completeness ledger

A = named XML attributes; C = named direct child declarations; X = extension slots; M = native members or the explicitly bounded runtime parameters. Sum A+C+X+M equals catalogue rows. Source IDs resolve to exact primary URLs below.

| Exact type or scoped anonymous object | A | C | X | M | Rows | Source |
| --- | ---: | ---: | ---: | ---: | ---: | --- |
| `EM.ProviderType` | 10 | 10 | 2 | 0 | 22 | [DIA-S01] |
| `EM.ChannelListType` | 0 | 2 | 2 | 0 | 4 | [DIA-S27] |
| `EM.ImportChannelType` | 3 | 0 | 0 | 0 | 3 | [DIA-S28] |
| `EM.ChannelType` | 9 | 2 | 0 | 0 | 11 | [DIA-S02] |
| `EM.ChannelLoggingType` | 0 | 3 | 2 | 0 | 5 | [DIA-S03] |
| `EM.ChannelPublishingType` | 0 | 10 | 2 | 0 | 12 | [DIA-S04] |
| `EM.EventDefinitionType` | 11 | 0 | 0 | 0 | 11 | [DIA-S05] |
| `EM.LevelType` | 4 | 0 | 0 | 0 | 4 | [DIA-S06] |
| `EM.TaskType` | 5 | 1 | 1 | 0 | 7 | [DIA-S07] |
| `EM.OpcodeType` | 5 | 0 | 1 | 0 | 6 | [DIA-S08] |
| `EM.KeywordType` | 4 | 0 | 1 | 0 | 5 | [DIA-S09] |
| `EM.MapType` | 0 | 2 | 0 | 0 | 2 | [DIA-S10] |
| `EM.ValueMapType` | 2 | 1 | 0 | 0 | 3 | [DIA-S11] |
| `EM.ValueMapValueType` | 3 | 0 | 0 | 0 | 3 | [DIA-S12] |
| `EM.BitMapType` | 2 | 1 | 0 | 0 | 3 | [DIA-S13] |
| `EM.BitMapValueType` | 3 | 0 | 0 | 0 | 3 | [DIA-S14] |
| `EM.TemplateItemType` | 2 | 4 | 0 | 0 | 6 | [DIA-S15] |
| `EM.TemplateItemType/binary` | 1 | 0 | 0 | 0 | 1 | [DIA-S15] |
| `EM.DataDefinitionType` | 6 | 0 | 1 | 0 | 7 | [DIA-S16] |
| `EM.StructDefinitionType` | 3 | 1 | 1 | 0 | 5 | [DIA-S17] |
| `EM.LocalizationType` | 1 | 1 | 2 | 0 | 4 | [DIA-S18] |
| `EM.LocalizationType/resources` | 1 | 1 | 1 | 0 | 3 | [DIA-S18] |
| `EM.StringTableType` | 0 | 1 | 1 | 0 | 2 | [DIA-S19] |
| `EM.StringTableType/string` | 3 | 0 | 0 | 0 | 3 | [DIA-S19] |
| `EM.FilterListType` | 0 | 1 | 0 | 0 | 1 | [DIA-S29] |
| `EM.FilterType` | 6 | 0 | 0 | 0 | 6 | [DIA-S30] |
| `EM.XmlType` | 0 | 0 | 1 | 0 | 1 | [DIA-S31] |
| `EM.LevelListType` | 0 | 1 | 0 | 0 | 1 | [DIA-S32] |
| `EM.TaskListType` | 0 | 1 | 0 | 0 | 1 | [DIA-S33] |
| `EM.OpcodeListType` | 0 | 1 | 0 | 0 | 1 | [DIA-S34] |
| `EM.KeywordListType` | 0 | 1 | 0 | 0 | 1 | [DIA-S35] |
| `EM.TemplateListType` | 0 | 1 | 0 | 0 | 1 | [DIA-S36] |
| `EM.DefinitionType` | 0 | 2 | 0 | 0 | 2 | [DIA-S37] |
| `NATIVE.EVENT_DESCRIPTOR` | 0 | 0 | 0 | 7 | 7 | [DIA-S20] |
| `RUNTIME.EventWriteTransfer-correlation` | 0 | 0 | 0 | 2 | 2 | [DIA-S21] |
| `PERF.provider` | 7 | 1 | 0 | 0 | 8 | [DIA-S22] |
| `PERF.counterSet` | 6 | 2 | 0 | 0 | 8 | [DIA-S23] |
| `PERF.counter` | 15 | 1 | 0 | 0 | 16 | [DIA-S24] |
| `PERF.counterAttributes` | 0 | 1 | 0 | 0 | 1 | [DIA-S25] |
| `PERF.counterAttribute` | 1 | 0 | 0 | 0 | 1 | [DIA-S26] |
| `PERF.structs` | 0 | 1 | 0 | 0 | 1 | [DIA-S38] |
| `PERF.struct` | 2 | 0 | 0 | 0 | 2 | [DIA-S39] |
| **Total** | **115** | **54** | **18** | **9** | **196** | |

### C.3. Provider and channel definitions

| Canonical field path | Category | Disposition | Meaning / constraint | Source |
| --- | --- | --- | --- | --- |
| `EM.ProviderType/@name` | Identity | Recommended | Stable provider name; required. | [DIA-S01] |
| `EM.ProviderType/@guid` | Identity | Recommended | Fixed provider GUID; required. | [DIA-S01] |
| `EM.ProviderType/@resourceFileName` | Resource | Recommended | Installed metadata-resource file path. | [DIA-S01] |
| `EM.ProviderType/@messageFileName` | Resource | Recommended | Installed localized-message resource file path. | [DIA-S01] |
| `EM.ProviderType/@parameterFileName` | Resource | Conditional | Parameter-string files, only when messages use them. | [DIA-S01] |
| `EM.ProviderType/@helpLink` | Display | Recommended | Maintained public event-help URL. | [DIA-S01] |
| `EM.ProviderType/@symbol` | Authoring | Recommended | Generated provider GUID symbol; required. | [DIA-S01] |
| `EM.ProviderType/@message` | Display | Recommended | Localized provider display-name reference. | [DIA-S01] |
| `EM.ProviderType/@source` | Reserved | Internal/unused | Documented internal; do not author as product metadata. | [DIA-S01] |
| `EM.ProviderType/@warnOnApplicationCompatibilityError` | Reserved | Internal/unused | Documented internal; leave alone. | [DIA-S01] |
| `EM.ProviderType/channels` | Container | Conditional | ChannelListType. | [DIA-S01] |
| `EM.ProviderType/levels` | Container | Conditional | LevelListType; custom levels need a reason. | [DIA-S01] |
| `EM.ProviderType/tasks` | Container | Conditional | TaskListType. | [DIA-S01] |
| `EM.ProviderType/opcodes` | Container | Conditional | OpcodeListType; provider scope. | [DIA-S01] |
| `EM.ProviderType/keywords` | Container | Conditional | KeywordListType. | [DIA-S01] |
| `EM.ProviderType/maps` | Container | Conditional | MapType. | [DIA-S01] |
| `EM.ProviderType/namedQueries` | Reserved | Internal/unused | NamedQueryType; documented unused; subtree unexpanded. | [DIA-S01] |
| `EM.ProviderType/templates` | Container | Conditional | TemplateListType. | [DIA-S01] |
| `EM.ProviderType/events` | Container | Recommended | DefinitionType for the owned event catalogue. | [DIA-S01] |
| `EM.ProviderType/filters` | Behavior | Conditional | FilterListType; controller/provider filtering contract. | [DIA-S01] |
| `EM.ProviderType/*{##other}` | Extension | Conditional | Foreign-namespace elements; open extension point. | [DIA-S01] |
| `EM.ProviderType/@*{##other}` | Extension | Conditional | Foreign-namespace attributes; open extension point. | [DIA-S01] |
| `EM.ChannelListType/importChannel` | Container | Conditional | ImportChannelType; existing external definition. | [DIA-S27] |
| `EM.ChannelListType/channel` | Container | Conditional | ChannelType; owned definition. | [DIA-S27] |
| `EM.ChannelListType/*{##other}` | Extension | Conditional | Foreign-namespace elements. | [DIA-S27] |
| `EM.ChannelListType/@*{##other}` | Extension | Conditional | Foreign-namespace attributes. | [DIA-S27] |
| `EM.ImportChannelType/@chid` | Schema | Conditional | Provider-local channel reference token. | [DIA-S28] |
| `EM.ImportChannelType/@name` | Identity | Conditional | Existing channel name; required when imported. | [DIA-S28] |
| `EM.ImportChannelType/@symbol` | Authoring | Conditional | Generated channel symbol. | [DIA-S28] |
| `EM.ChannelType/@name` | Identity | Recommended | Stable channel name; required. | [DIA-S02] |
| `EM.ChannelType/@chid` | Schema | Recommended | Unambiguous provider-local reference token. | [DIA-S02] |
| `EM.ChannelType/@type` | Behavior | Conditional | Admin/Operational/Analytic/Debug; required owned choice. | [DIA-S02] |
| `EM.ChannelType/@symbol` | Authoring | Conditional | Generated channel symbol. | [DIA-S02] |
| `EM.ChannelType/@access` | Security | Conditional | SDDL; effects depend on channel isolation. | [DIA-S02] |
| `EM.ChannelType/@isolation` | Security | Conditional | Application/System/Custom; changes access/session behavior. | [DIA-S02] |
| `EM.ChannelType/@enabled` | Behavior | Conditional | Initial enablement; not a completeness switch. | [DIA-S02] |
| `EM.ChannelType/@value` | Schema | Conditional | UInt8 channel number; compiler can assign. | [DIA-S02] |
| `EM.ChannelType/@message` | Display | Recommended | Localized channel display-name reference. | [DIA-S02] |
| `EM.ChannelType/logging` | Behavior | Conditional | ChannelLoggingType. | [DIA-S02] |
| `EM.ChannelType/publishing` | Behavior | Conditional | ChannelPublishingType; limited applicability. | [DIA-S02] |
| `EM.ChannelLoggingType/autoBackup` | Behavior | Conditional | Backup behavior; Admin/Operational; requires retention. | [DIA-S03] |
| `EM.ChannelLoggingType/retention` | Behavior | Conditional | Retention/full-log behavior; owned storage policy. | [DIA-S03] |
| `EM.ChannelLoggingType/maxSize` | Behavior | Conditional | UInt64 log budget in bytes; actual allocation differs. | [DIA-S03] |
| `EM.ChannelLoggingType/*{##other}` | Extension | Conditional | Foreign-namespace elements. | [DIA-S03] |
| `EM.ChannelLoggingType/@*{##other}` | Extension | Conditional | Foreign-namespace attributes. | [DIA-S03] |
| `EM.ChannelPublishingType/level` | Behavior | Conditional | UInt8 collection threshold. | [DIA-S04] |
| `EM.ChannelPublishingType/keywords` | Behavior | Conditional | UInt64 collection mask. | [DIA-S04] |
| `EM.ChannelPublishingType/controlGuid` | Behavior | Conditional | WPP control GUID; Debug-channel constraints. | [DIA-S04] |
| `EM.ChannelPublishingType/bufferSize` | Behavior | Conditional | UInt32 buffer allocation in KB. | [DIA-S04] |
| `EM.ChannelPublishingType/minBuffers` | Behavior | Conditional | UInt32 minimum buffer count. | [DIA-S04] |
| `EM.ChannelPublishingType/fileMax` | Behavior | Conditional | UInt32 log-generation limit. | [DIA-S04] |
| `EM.ChannelPublishingType/maxBuffers` | Behavior | Conditional | UInt32 maximum buffer count. | [DIA-S04] |
| `EM.ChannelPublishingType/latency` | Behavior | Conditional | UInt32 flush latency in milliseconds. | [DIA-S04] |
| `EM.ChannelPublishingType/clockType` | Behavior | Conditional | SystemTime/QPC; timestamp-source choice. | [DIA-S04] |
| `EM.ChannelPublishingType/sidType` | Privacy | Conditional | None/Publishing; publisher SID inclusion. | [DIA-S04] |
| `EM.ChannelPublishingType/*{##other}` | Extension | Conditional | Foreign-namespace elements. | [DIA-S04] |
| `EM.ChannelPublishingType/@*{##other}` | Extension | Conditional | Foreign-namespace attributes. | [DIA-S04] |

### C.4. Event definitions, vocabularies and maps

| Canonical field path | Category | Disposition | Meaning / constraint | Source |
| --- | --- | --- | --- | --- |
| `EM.EventDefinitionType/@value` | Schema | Recommended | Event ID; required; UInt32 schema versus native UInt16 caveat. | [DIA-S05] |
| `EM.EventDefinitionType/@level` | Schema | Recommended | Meaningful severity reference; Admin constraints apply. | [DIA-S05] |
| `EM.EventDefinitionType/@template` | Schema | Conditional | Template ID for actual payload. | [DIA-S05] |
| `EM.EventDefinitionType/@channel` | Schema | Recommended | Defined/imported channel for persisted events. | [DIA-S05] |
| `EM.EventDefinitionType/@keywords` | Schema | Recommended | Meaningful category references; native descriptor mask. | [DIA-S05] |
| `EM.EventDefinitionType/@task` | Schema | Conditional | Task reference for the event's operation group. | [DIA-S05] |
| `EM.EventDefinitionType/@opcode` | Schema | Conditional | Operation reference; respect task-local scope. | [DIA-S05] |
| `EM.EventDefinitionType/@symbol` | Authoring | Recommended | Stable generated event symbol. | [DIA-S05] |
| `EM.EventDefinitionType/@version` | Schema | Recommended | UInt8 event-schema revision; not product version. | [DIA-S05] |
| `EM.EventDefinitionType/@message` | Display | Recommended | Localized actionable event text; required for Admin. | [DIA-S05] |
| `EM.EventDefinitionType/@notLogged` | Behavior | Conditional | Retired-event compatibility marker; contradictory prose needs compiler qualification. | [DIA-S05] |
| `EM.LevelType/@name` | Schema | Conditional | Custom level QName; required if defined. | [DIA-S06] |
| `EM.LevelType/@symbol` | Authoring | Conditional | Generated level symbol. | [DIA-S06] |
| `EM.LevelType/@value` | Schema | Conditional | UInt8 custom value; predefined levels usually suffice. | [DIA-S06] |
| `EM.LevelType/@message` | Display | Conditional | Localized custom-level label. | [DIA-S06] |
| `EM.TaskType/@name` | Schema | Conditional | Task QName; required if defined. | [DIA-S07] |
| `EM.TaskType/@symbol` | Authoring | Conditional | Generated task symbol. | [DIA-S07] |
| `EM.TaskType/@value` | Schema | Conditional | UInt16 task value; required; documentation range caveat. | [DIA-S07] |
| `EM.TaskType/@eventGUID` | Legacy | Not-applicable | Downlevel MOF task support, outside current profile. | [DIA-S07] |
| `EM.TaskType/@message` | Display | Conditional | Localized task label. | [DIA-S07] |
| `EM.TaskType/opcodes` | Container | Conditional | OpcodeListType at task-local scope. | [DIA-S07] |
| `EM.TaskType/@*{##other}` | Extension | Conditional | Foreign-namespace attributes. | [DIA-S07] |
| `EM.OpcodeType/@name` | Schema | Conditional | Opcode QName; required if defined. | [DIA-S08] |
| `EM.OpcodeType/@symbol` | Authoring | Conditional | Generated opcode symbol. | [DIA-S08] |
| `EM.OpcodeType/@value` | Schema | Conditional | UInt8 opcode; predefined/custom ranges apply. | [DIA-S08] |
| `EM.OpcodeType/@mofValue` | Reserved | Internal/unused | Reserved internal field. | [DIA-S08] |
| `EM.OpcodeType/@message` | Display | Conditional | Localized opcode label. | [DIA-S08] |
| `EM.OpcodeType/@*{##other}` | Extension | Conditional | Foreign-namespace attributes. | [DIA-S08] |
| `EM.KeywordType/@name` | Schema | Conditional | Category QName; required if defined. | [DIA-S09] |
| `EM.KeywordType/@symbol` | Authoring | Conditional | Generated keyword symbol. | [DIA-S09] |
| `EM.KeywordType/@mask` | Schema | Conditional | One application-owned bit among low 48; required. | [DIA-S09] |
| `EM.KeywordType/@message` | Display | Conditional | Localized category label. | [DIA-S09] |
| `EM.KeywordType/@*{##other}` | Extension | Conditional | Foreign-namespace attributes. | [DIA-S09] |
| `EM.MapType/valueMap` | Container | Conditional | ValueMapType; numeric value labels. | [DIA-S10] |
| `EM.MapType/bitMap` | Container | Conditional | BitMapType; flag labels. | [DIA-S10] |
| `EM.ValueMapType/@name` | Schema | Conditional | Map reference name; required. | [DIA-S11] |
| `EM.ValueMapType/@symbol` | Authoring | Conditional | Generated map symbol. | [DIA-S11] |
| `EM.ValueMapType/map` | Container | Conditional | ValueMapValueType entries. | [DIA-S11] |
| `EM.ValueMapValueType/@value` | Schema | Conditional | UInt32 mapped value; required. | [DIA-S12] |
| `EM.ValueMapValueType/@message` | Display | Conditional | Localized value label; required. | [DIA-S12] |
| `EM.ValueMapValueType/@symbol` | Authoring | Conditional | Generated value symbol. | [DIA-S12] |
| `EM.BitMapType/@name` | Schema | Conditional | Map reference name; XSD does not mark required. | [DIA-S13] |
| `EM.BitMapType/@symbol` | Authoring | Conditional | Generated map symbol. | [DIA-S13] |
| `EM.BitMapType/map` | Container | Conditional | BitMapValueType entries. | [DIA-S13] |
| `EM.BitMapValueType/@value` | Schema | Conditional | Single HexInt32 bit; required. | [DIA-S14] |
| `EM.BitMapValueType/@message` | Display | Conditional | Localized flag label; required. | [DIA-S14] |
| `EM.BitMapValueType/@symbol` | Authoring | Conditional | Generated flag symbol. | [DIA-S14] |

### C.5. Templates, field layouts and localization

| Canonical field path | Category | Disposition | Meaning / constraint | Source |
| --- | --- | --- | --- | --- |
| `EM.TemplateItemType/@tid` | Schema | Conditional | Unique provider-local template ID; required. | [DIA-S15] |
| `EM.TemplateItemType/@name` | Authoring | Conditional | Optional template name; distinct from binary/@name. | [DIA-S15] |
| `EM.TemplateItemType/data` | Container | Conditional | DataDefinitionType; ordered payload fields. | [DIA-S15] |
| `EM.TemplateItemType/struct` | Container | Conditional | StructDefinitionType; structured payload. | [DIA-S15] |
| `EM.TemplateItemType/binary` | Reserved | Internal/unused | Reserved anonymous subtree. | [DIA-S15] |
| `EM.TemplateItemType/UserData` | Rendering | Conditional | XmlType custom rendering; arbitrary XML not expanded. | [DIA-S15] |
| `EM.TemplateItemType/binary/@name` | Reserved | Internal/unused | Anonymous binary element's reserved name. | [DIA-S15] |
| `EM.DataDefinitionType/@name` | Schema | Conditional | Payload field name; required. | [DIA-S16] |
| `EM.DataDefinitionType/@inType` | Schema | Conditional | Wire/input QName; required. | [DIA-S16] |
| `EM.DataDefinitionType/@outType` | Rendering | Conditional | Compatible display/output QName. | [DIA-S16] |
| `EM.DataDefinitionType/@map` | Rendering | Conditional | Value/bit map reference for supported input types. | [DIA-S16] |
| `EM.DataDefinitionType/@length` | Schema | Conditional | Literal/reference length; type-specific bytes or characters. | [DIA-S16] |
| `EM.DataDefinitionType/@count` | Schema | Conditional | Literal/reference array element count. | [DIA-S16] |
| `EM.DataDefinitionType/@*{##other}` | Extension | Conditional | Foreign-namespace attributes. | [DIA-S16] |
| `EM.StructDefinitionType/@name` | Schema | Conditional | Payload structure name; required. | [DIA-S17] |
| `EM.StructDefinitionType/@length` | Legacy | Not-applicable | Still in published XSD; unavailable since Windows 7. | [DIA-S17] |
| `EM.StructDefinitionType/@count` | Schema | Conditional | Structure count; literal or eligible outside-field reference. | [DIA-S17] |
| `EM.StructDefinitionType/data` | Container | Conditional | DataDefinitionType; member layout. | [DIA-S17] |
| `EM.StructDefinitionType/@*{##other}` | Extension | Conditional | Foreign-namespace attributes. | [DIA-S17] |
| `EM.LocalizationType/@fallbackCulture` | Reserved | Internal/unused | Documented unused; no locale-fallback guarantee. | [DIA-S18] |
| `EM.LocalizationType/resources` | Container | Conditional | Anonymous per-culture resources object below. | [DIA-S18] |
| `EM.LocalizationType/*{##other}` | Extension | Conditional | Foreign-namespace elements. | [DIA-S18] |
| `EM.LocalizationType/@*{##other}` | Extension | Conditional | Foreign-namespace attributes. | [DIA-S18] |
| `EM.LocalizationType/resources/@culture` | Localization | Recommended | Actual supported locale; required. | [DIA-S18] |
| `EM.LocalizationType/resources/stringTable` | Container | Recommended | StringTableType for that locale. | [DIA-S18] |
| `EM.LocalizationType/resources/@*{##other}` | Extension | Conditional | Foreign-namespace attributes. | [DIA-S18] |
| `EM.StringTableType/string` | Container | Recommended | Anonymous localized-string entries below. | [DIA-S19] |
| `EM.StringTableType/@*{##other}` | Extension | Conditional | Foreign-namespace attributes. | [DIA-S19] |
| `EM.StringTableType/string/@id` | Localization | Recommended | Unique resource key; required. | [DIA-S19] |
| `EM.StringTableType/string/@value` | Localization | Recommended | Correct localized text; required. | [DIA-S19] |
| `EM.StringTableType/string/@stringType` | Reserved | Internal/unused | Documented unused. | [DIA-S19] |

### C.6. Filters, reusable containers and XML escape hatch

| Canonical field path | Category | Disposition | Meaning / constraint | Source |
| --- | --- | --- | --- | --- |
| `EM.FilterListType/filter` | Behavior | Conditional | FilterType; optional controller/provider contract. | [DIA-S29] |
| `EM.FilterType/@value` | Schema | Conditional | UInt8 filter ID; required. | [DIA-S30] |
| `EM.FilterType/@version` | Schema | Conditional | UInt8 filter-data schema revision. | [DIA-S30] |
| `EM.FilterType/@name` | Schema | Conditional | Filter QName; required. | [DIA-S30] |
| `EM.FilterType/@symbol` | Authoring | Conditional | Generated filter symbol. | [DIA-S30] |
| `EM.FilterType/@message` | Display | Conditional | Localized filter description. | [DIA-S30] |
| `EM.FilterType/@tid` | Schema | Conditional | Template describing enable-time filter data. | [DIA-S30] |
| `EM.XmlType/*{##other}` | Extension | Conditional | Arbitrary foreign-namespace XML; schema content skipped. | [DIA-S31] |
| `EM.LevelListType/level` | Container | Conditional | LevelType definitions. | [DIA-S32] |
| `EM.TaskListType/task` | Container | Conditional | TaskType definitions. | [DIA-S33] |
| `EM.OpcodeListType/opcode` | Container | Conditional | OpcodeType; provider or task scope. | [DIA-S34] |
| `EM.KeywordListType/keyword` | Container | Conditional | KeywordType definitions. | [DIA-S35] |
| `EM.TemplateListType/template` | Container | Conditional | TemplateItemType definitions. | [DIA-S36] |
| `EM.DefinitionType/event` | Container | Recommended | EventDefinitionType definitions. | [DIA-S37] |
| `EM.DefinitionType/task` | Legacy | Not-applicable | TaskEventDefinitionType; unsupported since Windows 7 message compiler; unexpanded. | [DIA-S37] |

### C.7. Native descriptor and event-instance correlation

| Canonical field path | Category | Disposition | Meaning / constraint | Source |
| --- | --- | --- | --- | --- |
| `NATIVE.EVENT_DESCRIPTOR/Id` | Native schema | Recommended | USHORT; 16-bit event ID. | [DIA-S20] |
| `NATIVE.EVENT_DESCRIPTOR/Version` | Native schema | Recommended | UCHAR; 8-bit schema revision. | [DIA-S20] |
| `NATIVE.EVENT_DESCRIPTOR/Channel` | Native schema | Conditional | UCHAR; routing/processing code; reserved values apply. | [DIA-S20] |
| `NATIVE.EVENT_DESCRIPTOR/Level` | Native schema | Recommended | UCHAR; meaningful nonzero severity. | [DIA-S20] |
| `NATIVE.EVENT_DESCRIPTOR/Opcode` | Native schema | Conditional | UCHAR; operation code. | [DIA-S20] |
| `NATIVE.EVENT_DESCRIPTOR/Task` | Native schema | Conditional | USHORT; 16-bit task code. | [DIA-S20] |
| `NATIVE.EVENT_DESCRIPTOR/Keyword` | Native schema | Recommended | ULONGLONG; 64-bit category mask. | [DIA-S20] |
| `RUNTIME.EventWriteTransfer-correlation/ActivityId` | Runtime | Runtime | Optional GUID pointer; NULL uses current thread activity. | [DIA-S21] |
| `RUNTIME.EventWriteTransfer-correlation/RelatedActivityId` | Runtime | Runtime | Optional parent-activity GUID; NULL means absent. | [DIA-S21] |

### C.8. Optional PerfLib V2 field set

| Canonical field path | Category | Disposition | Meaning / constraint | Source |
| --- | --- | --- | --- | --- |
| `PERF.provider/@symbol` | Authoring | Conditional | Generated provider-handle symbol. | [DIA-S22] |
| `PERF.provider/@callback` | Behavior | Conditional | Notification callback mode; not descriptive metadata. | [DIA-S22] |
| `PERF.provider/@providerGuid` | Identity | Conditional | Provider GUID; required if interface exists. | [DIA-S22] |
| `PERF.provider/@applicationIdentity` | Resource | Conditional | Resource EXE/DLL basename; required; not full path. | [DIA-S22] |
| `PERF.provider/@providerType` | Behavior | Conditional | User-mode profile; schema/prose kernel spelling mismatch. | [DIA-S22] |
| `PERF.provider/@providerName` | Identity | Conditional | WMI class-name contribution; not generic publisher text. | [DIA-S22] |
| `PERF.provider/@resourceBase` | Authoring | Conditional | UInt32 generated-resource starting ID. | [DIA-S22] |
| `PERF.provider/counterSet` | Container | Conditional | CounterSet definitions; keys/references constrain entries. | [DIA-S22] |
| `PERF.counterSet/@symbol` | Authoring | Conditional | Generated counter-set GUID symbol; required. | [DIA-S23] |
| `PERF.counterSet/@guid` | Identity | Conditional | Counter-set GUID; required. | [DIA-S23] |
| `PERF.counterSet/@uri` | Identity | Conditional | Unique counter-set URI; required; not a network endpoint grant. | [DIA-S23] |
| `PERF.counterSet/@name` | Display | Conditional | Counter-set display name; required. | [DIA-S23] |
| `PERF.counterSet/@description` | Display | Conditional | Truthful counter-set description; required. | [DIA-S23] |
| `PERF.counterSet/@instances` | Behavior | Conditional | Single/multiple/aggregation semantics. | [DIA-S23] |
| `PERF.counterSet/structs` | Other profile | Not-applicable | Structure-backed route unused in user-mode profile; expanded below. | [DIA-S23] |
| `PERF.counterSet/counter` | Container | Conditional | Counter definitions; at least one if set exists. | [DIA-S23] |
| `PERF.counter/@symbol` | Authoring | Conditional | Generated counter ID symbol. | [DIA-S24] |
| `PERF.counter/@id` | Identity | Conditional | UInt32 set-local ID; required. | [DIA-S24] |
| `PERF.counter/@uri` | Identity | Conditional | Unique counter URI; required. | [DIA-S24] |
| `PERF.counter/@name` | Display | Conditional | Display name; omit only when justified by noDisplay. | [DIA-S24] |
| `PERF.counter/@description` | Display | Conditional | Meaning and truthful units; omit only for justified noDisplay. | [DIA-S24] |
| `PERF.counter/@type` | Behavior | Conditional | Required counter-type enum; determines calculation. | [DIA-S24] |
| `PERF.counter/@baseID` | Behavior | Conditional | Same-set base-counter ID when type requires. | [DIA-S24] |
| `PERF.counter/@detailLevel` | Display | Conditional | Required intended-audience level. | [DIA-S24] |
| `PERF.counter/@defaultScale` | Behavior | Conditional | Display exponent; published XSD range -10 through 10. | [DIA-S24] |
| `PERF.counter/@aggregate` | Behavior | Conditional | Aggregation function for applicable instance mode. | [DIA-S24] |
| `PERF.counter/@perfTimeID` | Behavior | Conditional | Same-set timestamp counter reference when required. | [DIA-S24] |
| `PERF.counter/@perfFreqID` | Behavior | Conditional | Same-set frequency counter reference when required. | [DIA-S24] |
| `PERF.counter/@multiCounterID` | Behavior | Conditional | Same-set multiplier counter reference when required. | [DIA-S24] |
| `PERF.counter/@struct` | Other profile | Not-applicable | Explicitly forbidden for user-mode providers. | [DIA-S24] |
| `PERF.counter/@field` | Other profile | Not-applicable | Explicitly forbidden for user-mode providers. | [DIA-S24] |
| `PERF.counter/counterAttributes` | Behavior | Conditional | CounterAttributes display/storage modifiers. | [DIA-S24] |
| `PERF.counterAttributes/counterAttribute` | Behavior | Conditional | CounterAttribute entries; schema permits one to five. | [DIA-S25] |
| `PERF.counterAttribute/@name` | Behavior | Conditional | reference/noDisplay/noDigitGrouping/displayAsHex/displayAsReal. | [DIA-S26] |
| `PERF.structs/struct` | Other profile | Not-applicable | Structure-backed route; structure descriptors. | [DIA-S38] |
| `PERF.struct/@name` | Other profile | Not-applicable | Structure name; required in that route. | [DIA-S39] |
| `PERF.struct/@type` | Other profile | Not-applicable | Generated structure symbol; required in that route. | [DIA-S39] |

### C.9. Boundaries that prevent misleading “complete metadata”

**Identity is not producer authentication.** Provider GUIDs, component names, event text, file version strings and runtime self-reports describe identity. Trust still depends on the existing protected installation, access policy and authenticated artefact evidence. Preserve the repository's separation between asserted metadata, generated bytes and independent installed observations.

**Manifest and native types can disagree.** Published `EventDefinitionType/@value` is UInt32, while native `EVENT_DESCRIPTOR.Id` is 16-bit. Do not assume every schema-accepted integer survives compilation or native emission. `TaskType/@value` is UInt16; its prose range is narrower and even the example differs. Qualify actual admitted compiler/output/native interpretation. Event identity and payload layout must remain consistent for provider decode GUID + ID + version; a product-version change alone does not require changing event version. [DIA-S05] [DIA-S07] [DIA-S20]

**Some documentation has known ambiguities.** The `notLogged` prose conflicts with its retirement example/name; this handoff does not choose a Boolean. PerfLib provider XSD spells the driver alternative `kernelMode`, while prose uses `kernel`; NevroTick's user-mode profile needs neither. The counter scaling prose contains sign/range mistakes; the shown XSD allows -10 through 10. Record these mismatches for the existing compiler/readback gate instead of silently normalizing them into production settings. [DIA-S05] [DIA-S22] [DIA-S24]

**Behavioral fields require their existing owner.** Channel publishing settings apply only to the channel configurations documented in ChannelPublishingType; this is not a request to enable them. Default-looking values are not chosen product budgets. Auto-backup does not itself establish a finite backup policy. Counter types, dependent IDs, aggregation, instance mode and reference storage change measurement behavior. [DIA-S02]–[DIA-S04] [DIA-S22]–[DIA-S26]

**TraceLogging is a separate representation.** Its self-describing ETW events do not automatically install manifest resources or make a durable Event Log channel. Microsoft retains manifest-based ETW guidance for the System/Application Windows Logs. This catalogue neither switches libraries nor selects a new channel/provider topology. [DIA-S41]

**Explicit exclusions:** the unused `NamedQueryType` subtree; unsupported legacy `TaskEventDefinitionType` subtree; arbitrary foreign-namespace extensions and arbitrary `UserData` XML; root instrumentation/metadata/type-system schemas not named in the ledger; complete winmeta/input/output-type enums; full Event Log rendering XML, publisher-property enum and channel-configuration API; full `EVENT_HEADER`, `EVENT_DATA_DESCRIPTOR`, trace-session/controller structures and write-function arguments; complete runtime PerfLib APIs/values and other PerfLib manifest roots/localization schemas. Their existence does not imply an omission within a listed type's direct-field inventory. `EventWriteTransfer` is intentionally a **two-parameter slice**, not its full six-parameter signature.

#### C.9.1. Support identity projection: optional product-defined fields

The repository already proposes one versioned metadata authority and separate observations. If an existing support/report surface needs a compact build identity, project facts from that authority: component role and actual shipped basename; product/display and component versions; source commit and release channel; protocol/configuration/event-schema versions that actually exist; admitted target/profile/toolchain identifier; final artefact digest with algorithm and stage; existing SBOM identifier/path or digest and format/version; manifest/resource digest and locale. These are **proposed product fields**, not additional SCM, ETW provider or PerfLib attributes. This paragraph is not an exhaustive SBOM schema.

Bind each observation to the actual inspected artefact, collector version, observation time, stage and outcome. A runtime self-report cannot independently attest its own signature/hash. Keep private build paths, hostnames, usernames, endpoints, secrets and per-user identifiers out of static/public metadata. Health, readiness, time quality, last synchronization and current errors remain changing status; do not freeze them into descriptions or present them as release identity. Existing privacy/export and release-custody rules apply.

#### C.9.2. Verification handoff

Future authorized implementation: reconcile authored schema against the **admitted SDK/compiler**, inspect generated descriptors/resources, query actual installed publisher metadata, resolve localized messages, render known owned events, and compare observation receipts with exact resource/payload hashes. Exercise only the relevant existing gates; no proof of Windows registration, rendering, performance or access was produced by this research. Reject missing required identities, duplicate/reused meanings, unresolved message keys, incorrect locale scope and resource paths not matching the installed candidate. Preserve pending native and release qualifications.

### C.10. Primary source register

The supplied research records these Microsoft Learn type/API sources. Integration independently compared all 37 XML-type pages and selected native/semantic contracts. Counts follow displayed XSD/C declarations within the limits above; the documentation snapshot need not equal the selected installed SDK.

| ID | Primary source |
| --- | --- |
| [DIA-S01] | ProviderType Complex Type |
| [DIA-S02] | ChannelType Complex Type |
| [DIA-S03] | ChannelLoggingType Complex Type |
| [DIA-S04] | ChannelPublishingType Complex Type |
| [DIA-S05] | EventDefinitionType Complex Type |
| [DIA-S06] | LevelType Complex Type |
| [DIA-S07] | TaskType Complex Type |
| [DIA-S08] | OpcodeType Complex Type |
| [DIA-S09] | KeywordType Complex Type |
| [DIA-S10] | MapType Complex Type |
| [DIA-S11] | ValueMapType Complex Type |
| [DIA-S12] | ValueMapValueType Complex Type |
| [DIA-S13] | BitMapType Complex Type |
| [DIA-S14] | BitMapValueType Complex Type |
| [DIA-S15] | TemplateItemType Complex Type |
| [DIA-S16] | DataDefinitionType Complex Type |
| [DIA-S17] | StructDefinitionType Complex Type |
| [DIA-S18] | LocalizationType Complex Type |
| [DIA-S19] | StringTableType Complex Type |
| [DIA-S20] | EVENT_DESCRIPTOR structure (evntprov.h) |
| [DIA-S21] | EventWriteTransfer function (evntprov.h) |
| [DIA-S22] | provider Complex Type |
| [DIA-S23] | counterSet Complex Type |
| [DIA-S24] | counter Complex Type |
| [DIA-S25] | counterAttributes Complex Type |
| [DIA-S26] | counterAttribute Complex Type |
| [DIA-S27] | ChannelListType Complex Type |
| [DIA-S28] | ImportChannelType Complex Type |
| [DIA-S29] | FilterListType Complex Type |
| [DIA-S30] | FilterType Complex Type |
| [DIA-S31] | XmlType Complex Type |
| [DIA-S32] | LevelListType Complex Type |
| [DIA-S33] | TaskListType Complex Type |
| [DIA-S34] | OpcodeListType Complex Type |
| [DIA-S35] | KeywordListType Complex Type |
| [DIA-S36] | TemplateListType Complex Type |
| [DIA-S37] | DefinitionType Complex Type |
| [DIA-S38] | structs Complex Type |
| [DIA-S39] | struct Complex Type |
| [DIA-S40] | EVT_PUBLISHER_METADATA_PROPERTY_ID enumeration (winevt.h) |
| [DIA-S41] | About TraceLogging |

[DIA-S01]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-providertype-complextype
[DIA-S02]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-channeltype-complextype
[DIA-S03]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-channelloggingtype-complextype
[DIA-S04]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-channelpublishingtype-complextype
[DIA-S05]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-eventdefinitiontype-complextype
[DIA-S06]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-leveltype-complextype
[DIA-S07]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-tasktype-complextype
[DIA-S08]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-opcodetype-complextype
[DIA-S09]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-keywordtype-complextype
[DIA-S10]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-maptype-complextype
[DIA-S11]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-valuemaptype-complextype
[DIA-S12]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-valuemapvaluetype-complextype
[DIA-S13]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-bitmaptype-complextype
[DIA-S14]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-bitmapvaluetype-complextype
[DIA-S15]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-templateitemtype-complextype
[DIA-S16]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-datadefinitiontype-complextype
[DIA-S17]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-structdefinitiontype-complextype
[DIA-S18]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-localizationtype-complextype
[DIA-S19]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-stringtabletype-complextype
[DIA-S20]: https://learn.microsoft.com/en-us/windows/win32/api/evntprov/ns-evntprov-event_descriptor
[DIA-S21]: https://learn.microsoft.com/en-us/windows/win32/api/evntprov/nf-evntprov-eventwritetransfer
[DIA-S22]: https://learn.microsoft.com/en-us/windows/win32/perfctrs/performance-counters-provider-complex-type
[DIA-S23]: https://learn.microsoft.com/en-us/windows/win32/perfctrs/performance-counters-counterset-complex-type
[DIA-S24]: https://learn.microsoft.com/en-us/windows/win32/perfctrs/performance-counters-counter-complex-type
[DIA-S25]: https://learn.microsoft.com/en-us/windows/win32/perfctrs/performance-counters-counterattributes-complex-type
[DIA-S26]: https://learn.microsoft.com/en-us/windows/win32/perfctrs/performance-counters-counterattribute-complex-type
[DIA-S27]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-channellisttype-complextype
[DIA-S28]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-importchanneltype-complextype
[DIA-S29]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-filterlisttype-complextype
[DIA-S30]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-filtertype-complextype
[DIA-S31]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-xmltype-complextype
[DIA-S32]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-levellisttype-complextype
[DIA-S33]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-tasklisttype-complextype
[DIA-S34]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-opcodelisttype-complextype
[DIA-S35]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-keywordlisttype-complextype
[DIA-S36]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-templatelisttype-complextype
[DIA-S37]: https://learn.microsoft.com/en-us/windows/win32/wes/eventmanifestschema-definitiontype-complextype
[DIA-S38]: https://learn.microsoft.com/en-us/windows/win32/perfctrs/performance-counters-structs-complex-type
[DIA-S39]: https://learn.microsoft.com/en-us/windows/win32/perfctrs/performance-counters-struct-complex-type
[DIA-S40]: https://learn.microsoft.com/en-us/windows/win32/api/winevt/ne-winevt-evt_publisher_metadata_property_id
[DIA-S41]: https://learn.microsoft.com/en-us/windows/win32/tracelogging/trace-logging-about

---
