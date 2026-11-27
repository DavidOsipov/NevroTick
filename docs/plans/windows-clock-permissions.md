# NV-CLK-001 — Windows clock-permission restriction plan

**Working feature name:** Restrict clock changes<br>
**Revision:** 1.0, 27 November 2026 (Asia/Tbilisi)<br>
**Status:** owner-requested future feature and roadmap position; detailed design proposal, subject to reassessment and native qualification<br>
**Owner decision:** [OD-21](../nevrotick-owner-decisions.md#25-od-21--restricted-local-clock-changing-permissions)<br>
**Roadmap:** [CLK-0](../../ROADMAP.md#clk-0--restrict-local-clock-changing-permissions), after MT-C01 and immediately before KHR-0<br>
**Disposition checkpoint:** CLK-C01, using existing Windows, lifecycle, UI and acceptance owners<br>
**Deferred execution intake:** [Windows task §4.43](../../tasks/windows11-integration.md#443-deferred-clock-permissions-feature--27-november-2026)<br>
**Research/source baseline:** [NevroTick `acbb85bd`](https://github.com/DavidOsipov/NevroTick/commit/acbb85bda8b467605f12e66819cd6b36147fd0fb); Microsoft Learn research checked on 27 November 2026<br>
**Next scheduled reassessment:** when MT-C01 is recorded and before CLK-0 implementation or Khronos/Ananke work starts

## 1. Purpose, authority and scheduled outcome

David requested a substantial, linked plan that can be reassessed and adjusted
later, and placed this feature immediately before Khronos. The accepted order is:

**V1 release → MATH-0 → TRUST-0 → MT-0 → CLK-0 → KHR-0 → GEO-0 → Android.**

This document preserves the investigation and specifies what the later team must
resolve, implement and verify. The scheduling decision is accepted. The proposed
Windows policy mutation, supported profiles and implementation details remain
conditional on evidence. Creating this plan does not implement the feature,
establish native enforcement or authorize changes to the development PC.

The intended outcome is an optional feature that explains the observed Windows
clock-changing permissions and, on qualified installations, lets an authorized
user apply a small, recoverable restriction. The first mutation candidate is
removing the direct `SeSystemtimePrivilege` assignment from
`BUILTIN\Administrators`, while preserving required system identities and other
assignments. It must never be described as giving NevroTick exclusive authority
over a compromised Windows machine.

The [architecture](../windows-time-architecture.md), accepted
[owner decisions](../nevrotick-owner-decisions.md),
[development plan](development-plan.md), and existing task/CHECK owners retain
their authority. This document owns the feature proposal and reassessment record;
it does not create a second privilege, recovery or acceptance framework. When a
later design changes a canonical contract, update that contract and its existing
task before dependent implementation.

### 1.1 Accepted decisions and open proposals

| Item | Disposition on 27 November 2026 |
| --- | --- |
| Preserve this research as a substantial, linked, maintainable plan | Requested by David. |
| Work on this feature immediately before Khronos | Accepted owner sequencing instruction: after MT-C01, before KHR-0. |
| Optional assessment and restriction experience | Proposed product design for that stage; detailed behavior must be reviewed at entry. |
| Remove only the direct Administrators time-right assignment | Initial qualification candidate; not a selected production recipe. |
| Change Local Service, arbitrary custom grants or third-party tokens | Outside this proposal. |
| Claim exclusion of malicious administrator/SYSTEM/kernel/firmware/VM host | Unsupported; outside the guarantee. |
| Make optional global grant removal a V1 requirement | Not selected. Existing V1 service isolation and recovery requirements remain mandatory. |
| Treat successful research, a model or a policy API return as native qualification | Not permitted. Evidence must match the claimed layer and candidate. |

### 1.2 CLK-C01 and the Khronos handoff

CLK-C01 records the feature's reached implementation, qualified profiles,
limitations, remaining work and explicit disposition. A successful shipping
outcome needs the admitted profile's actual assessment, consent, activation,
enforcement, recovery and installed-package evidence. An assessment-only result
must be labelled as such; it does not prove the mutation path.

If a necessary mechanism remains unsupported, finish the bounded investigation
with its precise blocker, alternatives and re-entry condition. Do not implement
an unsafe workaround, silently skip the feature, or call research a completed
protection. A material change to the requested delivery or sequencing uses the
existing [OD-04 decision procedure](../nevrotick-owner-decisions.md#6-od-04--decisions-when-evidence-makes-them-concrete).
The resulting recorded disposition supplies the handoff to KHR-0.

Current V1 development continues through
[development §7](development-plan.md#7-next-executable-task-for-a-development-agent).
This plan does not insert optional restriction implementation into that queue.
Existing Windows execution, fuzz, storage, UI and release gates retain their
scope; a future roadmap stage does not lift them.

## 2. Problem and defensible guarantee

A process able to change the system clock can disrupt authentication, logs,
scheduling and software that uses local wall time to decide when data expires.
The motivating concern is malicious forward or backward clock manipulation,
including a forward jump that causes a backup application to consider retained
copies old enough to delete. Microsoft identifies clock manipulation as a risk
of the system-time user right. [MS-01](#ms-01-change-the-system-time)

Windows exposes a privilege boundary for ordinary user-mode setters. NevroTick
can work within that boundary to reduce selected assignments and inspect its
own processes. It cannot turn its service into an authority above unrestricted
machine administration. Microsoft's security-servicing criteria treat
administrators as part of the Windows trusted computing base; administrative
access to the ordinary kernel is not a serviced security boundary. This does
not deny the existence of separate VBS protections; it bounds this feature's
ordinary Windows clock claim. [MS-10](#ms-10-windows-security-boundaries)

### 2.1 Threat and value boundary

| Actor or condition | Intended contribution | Limit that remains visible |
| --- | --- | --- |
| Process whose effective token lacks the time privilege | Windows should reject privileged clock-setting APIs; qualify the actual APIs on the selected profile. | An account-policy display alone does not establish the process token. |
| Newly created elevated process deriving the privilege only from Administrators | The proposed removal may eliminate that direct grant route. | Other groups, direct assignments or authority paths may still provide it. |
| Already-running privileged process | Show activation uncertainty and explain the existing-token boundary. | Changing account assignments does not retroactively rewrite its token. |
| Local Service and reported remaining principals | Preserve required assignments and disclose exceptions. | Their presence prevents a universal NevroTick-only claim. |
| Malicious unrestricted administrator or SYSTEM-level control | Reduce casual direct use where applicable; detect some configuration divergence. | The actor may alter policy, services, processes or the product itself. |
| Compromised broker | Existing broker isolation and authorization remain necessary. | Arbitrary execution in the clock authority is outside the restriction's guarantee. |
| Kernel, firmware or controlling VM host | Report relevant uncertainty where observable. | This application cannot enforce clock exclusivity over those authorities. |
| Backup cleanup driven by manipulated wall time | Reduce one possible local trigger within the qualified boundary. | No guarantee that a third-party backup operation is intercepted or reversible. |

The expected benefit is deliberately modest: a smaller direct permission surface,
fewer accidental elevated clock changes, and an honest assessment of the selected
configuration. Standard users normally lack this privilege already. A feature
that only removes the Administrators assignment must demonstrate useful value
without presenting itself as anti-ransomware or total host-compromise protection.
[MS-01](#ms-01-change-the-system-time)

NTS authentication concerns the time exchange. It does not revoke a local
process's Windows privileges. Likewise, detecting a discontinuity and correcting
the clock afterward cannot undo every operation another application performed
during the interval. The architecture's
[clock ownership and takeover rules](../windows-time-architecture.md#111-transactional-takeover)
remain separate from this proposed account-policy restriction.

## 3. Existing product baseline and dependencies

The following requirements already belong to NevroTick. They must not be made
dependent on opting into this feature:

- Separate own-process engine and broker, with qualified installed identities.
- Engine token containing the selected exact `SeChangeNotifyPrivilege` profile
  and lacking time, debug, impersonation and other unselected privileges.
- Broker token containing the selected exact `SeChangeNotifyPrivilege` and
  `SeSystemtimePrivilege` profile, with the time privilege disabled outside the
  bounded authorized effect.
- Effective-token admission, protected service/process/token/IPC access, and
  current authorization at the effect.
- Transactional W32Time handover, competing-controller handling, discontinuity
  processing and restoration under current ownership.
- Supported complete-MSIX provisioning before normal functional exposure,
  without mandatory first UI launch or manual rights-provisioning steps.
- Independently usable recovery for outstanding obligations during update,
  rollback and removal, including already-stopped or crashed product services.

These are canonical in
[architecture §4.1.1](../windows-time-architecture.md#411-service-privilege-profiles),
[§4.2.1](../windows-time-architecture.md#421-service-management-admission-and-administrative-recovery),
[§11](../windows-time-architecture.md#111-transactional-takeover),
[§12.1.4](../windows-time-architecture.md#1214-mandatory-state-lifetime-and-bounded-recovery)
and [§19.4](../windows-time-architecture.md#194-future-complete-msix-gate).

An optional setup screen cannot repair an engine that starts with excessive
authority. The complete package must establish its baseline safely on its own.
Do not convert this feature into manual installation instructions for removing
dangerous privileges from NevroTick after startup.

### 3.1 What the inspected source establishes

At `acbb85bd`, the repository includes portable privilege-observation and role
comparison groundwork in
[`privilege_observation.rs`](../../crates/windows-platform/src/privilege_observation.rs)
and native laboratory service/token work under the
[Windows service fixture](../../tools/windows-service-fixture/README.md).
Representation compatibility and fixture results do not establish current
production authority or complete installed-product qualification.

The current [MSIX plan](windows-msix-store.md#75-pre-start-provisioning-authority-and-admission--5-november-2026)
and [W11 task](../../tasks/windows11-integration.md)
retain unresolved provisioning and independent-recovery work. The
[helper-only privilege-assisted ownership candidate](../../tasks/windows11-integration.md#privilege-assisted-service-ownership-candidate--pending-owner-disposition)
has a separately authorized bounded feasibility experiment. Production adoption
remains unselected. That experiment's existence does not establish a usable
installer or removal route for this new feature.

Reinspect the actual source and evidence at CLK-0 entry. Do not extend this dated
inventory to a later revision without checking relevant changes.

## 4. Microsoft evidence and its design consequences

The references below were investigated using Microsoft Learn MCP search and
full-page retrieval. They support the stated Windows semantics; they do not
qualify NevroTick's future implementation. The reference catalogue in
[§17](#17-primary-source-catalogue) retains direct links for reassessment.

| Finding | Consequence for the design | Evidence and scope |
| --- | --- | --- |
| `SeSystemtimePrivilege` controls system-time changes; time-zone policy is separate. | Model UTC clock authority separately from display/time-zone configuration. | [MS-01](#ms-01-change-the-system-time), [MS-11](#ms-11-system-time-setters). |
| LSA enumeration reports direct assignments, excluding users that inherit the right through groups. | Keep direct grants, known inheritance and token observations as distinct evidence. | [MS-02](#ms-02-direct-right-enumeration), [MS-04](#ms-04-token-privileges-and-observation). |
| `STATUS_NO_MORE_ENTRIES` means no accounts have the specified direct right. | Preserve a complete empty result; do not collapse it into either failed observation or universal absence of clock writers. | [MS-02](#ms-02-direct-right-enumeration). |
| Assignment changes do not alter already-created user tokens. | A successful write/readback can still require activation. Existing tokens remain a separate question. | [MS-03](#ms-03-account-policy-and-existing-tokens). |
| Token inspection requires appropriate access; privileges can be present but disabled. | Query actual accessible product tokens. Denied/incomplete observation remains unknown; disabled is not absent. | [MS-04](#ms-04-token-privileges-and-observation). |
| SCM required-privilege filtering occurs at process start; shared-process services use a union. | Preserve own-process roles and next-start validation. A saved service setting does not establish the running profile. | [MS-05](#ms-05-service-required-privileges). |
| `SetSystemTime` can enable a privilege already present in a token; clock-rate adjustment also needs the time privilege. | Test step and rate setters. Do not equate disabled privilege with denial of future use. | [MS-11](#ms-11-system-time-setters). |
| LSA add/remove APIs can operate on a named SID/right; removing an absent right is ignored. | Use a narrow delta and `AllRights=FALSE`; distinguish no-op from a change NevroTick actually made. | [MS-06](#ms-06-narrow-policy-mutation). |
| The documented UserRights CSP requires Local Service for ChangeSystemTime and replaces the configured list. | Preserve Local Service; do not use wholesale CSP list replacement as the initial operation. | [MS-07](#ms-07-userrights-csp). This is the documented CSP constraint, not proof that every LSA path behaves identically. |
| Group Policy or management synchronization can supersede local state. | Managed or uncertain provenance is guidance-only in the initial design; never continually fight management policy. | [MS-01](#ms-01-change-the-system-time), [MS-08](#ms-08-management-provenance). |
| MDM registration is only one observation; the cited API lists no supported server. | A false registration result is insufficient to admit a standalone profile. Qualify a separate Server method or keep Apply unavailable there. | [MS-08](#ms-08-management-provenance). |
| The documented LSA notification categories do not include a general user-right-assignment category. | Use bounded readback at defined checkpoints. Do not promise immediate detection of every policy change. | [MS-09](#ms-09-policy-notifications). |
| Packaged-service manifest accounts and uninstall hooks have specific limits. | Resolve actual service identity and independently usable recovery before persistent mutation. | [MS-12](#ms-12-packaged-service-identities), [MS-13](#ms-13-uninstall-and-package-lifetime). |

**Inference from the documented API contracts:** the cited LSA calls provide no
compare-and-swap transaction spanning policy, SCM state, product journals and
process activation. Preflight, serialization and readback improve control and
observation; they do not make the sequence atomic against other administrators.
The design must retain ambiguous outcomes and concurrent-change handling.

## 5. Initial supported-profile candidate

The first qualification candidate is one explicitly admitted, unmanaged,
standalone Windows installation. Reuse the project's actual supported Windows
profile and package qualification; do not assume that a client result establishes
Windows Server behavior or that the stated target version is available and
qualified merely because it appears in a plan.

### 5.1 Admission conditions

Apply can become available only when all of the following are established for
the current proposal:

1. The installed product, service registrations, process roles and package
   instance match the qualified profile.
2. The broker retains its required authority independently of the grant being
   removed. A service name, service SID or account display name is insufficient.
3. Direct-grant enumeration is complete for its stated scope; known inheritance,
   remaining assignments and inspection gaps are represented honestly.
4. The selected evidence establishes the admitted standalone management profile.
   Domain/MDM/RSoP observations are not interchangeable and missing evidence is
   not evidence of unmanaged ownership.
5. Activation requirements and limits are known for that profile, including
   existing logon and service tokens.
6. Recovery is qualified for the actual installed lifecycle, including failure
   and independent removal with all product services already stopped.
7. The proposed delta remains current and an authorized user has approved its
   material contents through the existing maintenance path.

Unsupported, managed or unknown profiles may receive the read-only assessment
and precise administrative guidance. They receive no misleading Apply action.

### 5.2 Allowed initial mutation

The candidate operation removes only:

| Field | Proposed fixed value |
| --- | --- |
| Principal | `BUILTIN\Administrators`, identified by SID `S-1-5-32-544` |
| Right | `SeSystemtimePrivilege` |
| Operation | Remove this named right from this principal |
| API constraint | `LsaRemoveAccountRights`, `AllRights=FALSE` |
| Required preserved state | Local Service and every unrelated principal/right assignment |
| Restoration candidate | Re-add only this product-owned removed assignment when current ownership and recovery conditions permit |

Use SID identity internally; localized names are presentation only. If the
Administrators assignment was already absent, report no change needed for this
specific delta. Do not later add it as a supposed restoration. If the whole
direct-grant list is empty, preserve that complete empty result and separately
assess tokens, inheritance and support eligibility.

This candidate intentionally leaves custom grants visible. Removing arbitrary
additional principals requires a separate impact and compatibility assessment.
Local administrative tools and scripts may rely on the removed assignment;
explain the practical consequence and qualified restore path before consent.

### 5.3 Excluded mechanisms

Do not add a kernel driver, generic account-policy editor, arbitrary script
runner, second clock controller, extra installer, copied privileged helper or
resident privileged scanner. Do not strip third-party tokens, conceal Settings
as a security control, replace the entire policy list, remove Local Service via
an unqualified workaround, or weaken service isolation to keep Apply working.
Changing time-zone rights and forcing management-policy precedence are outside
this feature.

## 6. Assessment, proposal and receipt contracts

The implementation should reuse the existing strict boundary, private validated
domain construction and current-authority rules. The following are semantic
contracts, not a requirement to create particular new Rust types or crates.

### 6.1 Read-only assessment

| Evidence group | Required content | Prohibited inference |
| --- | --- | --- |
| Observation identity | Installed product instance, relevant source/profile version, observation generation and freshness basis. | A previously valid observation remains current indefinitely. |
| Direct assignments | Canonical SID set plus complete, complete-empty, denied, incomplete or failed outcome. Preserve meaningful Windows status. | Empty, absent and inaccessible are interchangeable. |
| Inheritance | Known contributing groups and explicit gaps, with the scope/method that established them. | Direct enumeration is a complete actor inventory. |
| Product tokens | Observed account, process identity, own-process role and exact present/enabled privilege evidence where accessible. | Configured service settings prove a running token. |
| Management | Qualified standalone, managed, unsupported or unknown, with supporting observations and their limits. | Not MDM-registered means unmanaged on every supported OS. |
| Remaining authority | Required system exceptions, other direct grants and known unresolved paths. | Only NevroTick can change time. |
| Activation | Which assignment/process changes are pending and what evidence can establish the selected activated profile. | Policy write success revokes existing tokens. |
| Recovery | Current qualified recovery method and readiness, or exact reason unavailable. | A journal or retained executable alone is sufficient. |
| Proposed action | Exact bounded delta or a refusal/no-change result. | An incomplete assessment can authorize Apply. |

Bound work, allocation and observation duration. A security assessment must not
add unlimited group expansion, process enumeration or domain lookups to a clock
service's critical path. Query-access failures should explain the missing scope;
they are not a reason to add debug or security privileges to the broker.

### 6.2 Immutable proposed change set

The trusted maintenance component constructs a proposal from current evidence.
It identifies the operation, fixed principal/right delta, relevant before-state,
installation and generation, retained exceptions, activation needs, recovery
method and user-visible consequences. A digest or identifier may bind review to
that immutable representation, but it is not itself authorization.

The UI displays the proposal; it does not supply arbitrary SIDs, paths, service
commands or scripts to a privileged helper. At application, the helper
reconstructs or revalidates the material facts and existing actor/request binding.
A changed grant, management state, installation, recovery route or other material
precondition invalidates the earlier consent and requires a new reviewable
proposal. A stale preview is never silently expanded into a different operation.

### 6.3 Result and recovery record

The protected result records the actual attempted operation and its outcome,
including indeterminate completion. It distinguishes before-state, intended
effect, reported API result, independent readback, activation observations and
remaining obligation. Reuse existing mandatory-state ownership and bounded
recovery; optional cache availability cannot control this record.

Persist only the information necessary for safety, diagnosis and reconciliation.
Account SIDs and policy snapshots can reveal local configuration; default exports
should minimize/redact identity details. Do not collect UPNs or a full user/group
directory merely because a discovery API can return them. The normal diagnostic
contract remains under W11-W08 and CHECK-17.

## 7. Component responsibilities and authority

| Component | Responsibility | Authority boundary |
| --- | --- | --- |
| Unelevated UI | Explain assessment, remaining grants, proposal, consent, pending activation and actual result. | No direct policy mutation; presentation state is not trusted authority. |
| Existing elevated lifecycle helper | Perform admitted policy observations/mutation and lifecycle reconciliation through the existing maintenance contract. | Fixed product-specific operations, current caller/instance binding and scoped authority. |
| Broker | Retain bounded runtime clock effects and existing ownership/discontinuity behavior. | Its exact privilege profile does not expand to support a machine-policy scanner. |
| Engine | Continue acquisition and production of admitted time information. | No machine-policy or clock-writing authority. |
| Existing protected transaction/state owner | Preserve current intent, actual outcomes and recovery obligations across supported lifecycle transitions. | No new competing journal or optional-cache dependency. |
| Packaging/recovery owner | Supply compatible trusted code, state, authority and invocation until obligations settle. | An MSIX payload or a service registration alone does not establish the whole recovery path. |

The source-to-effect chain is therefore: current assessment → trusted proposal →
authorized maintenance request → current preflight → recorded narrow mutation →
readback → activation evidence → truthful UI/result. Recovery follows actual
outstanding obligations, not the UI process lifetime. At implementation, trace
the real callers and consumers through this chain; an architectural diagram is
not evidence that the path exists or cannot be bypassed.

## 8. States, activation and honest status

The UI should show a narrow configuration assessment rather than a global green
protection shield. Keep service availability, clock ownership, time quality,
permission configuration, activation and observation freshness distinct.

| State | Meaning | Permitted next step |
| --- | --- | --- |
| Not assessed | No usable current assessment. | Run bounded assessment. |
| Assessment incomplete | Required evidence failed or is unavailable. | Explain the gap; no Apply. |
| Managed by your organization | Relevant management authority controls or may supersede this setting. | Report evidence and administrative guidance. |
| Change available | A current, supported bounded proposal exists. | Review and explicitly approve it. |
| No change needed for this grant | The selected Administrators assignment is absent in a complete direct observation. | Report remaining authorities and limits; no invented restoration duty. |
| Activation pending | Policy state was changed/read back, but required effective-state evidence is not complete. | Follow qualified activation guidance and reassess. |
| Selected configuration verified | Required observations establish the qualified profile within stated scope and freshness. | Show scope/time of observation; maintain defined readback checkpoints. |
| Configuration changed externally | A relevant later change or conflict invalidates the earlier result. | Stop automatic reapplication; reassess ownership and intent. |
| Recovery required | An actual or possible outstanding change needs reconciliation. | Use the supported recovery route and report unresolved state. |

Do not promise immediate revocation or assume restarting NevroTick refreshes
unrelated user tokens. Qualification must identify which logon/service processes
need replacement and how the selected profile is established afterward. If
reboot is part of a supported activation path, prove the resulting observation
and startup sequence; do not use a reboot suggestion as a substitute for evidence.
[MS-03](#ms-03-account-policy-and-existing-tokens),
[MS-05](#ms-05-service-required-privileges)

Observation freshness should follow existing monotonic/lifecycle conventions.
The wall clock being protected is not a trustworthy sole source for expiring a
consent or deciding that a prior assessment remains current after a discontinuity.
QPC is useful for local elapsed-time comparisons, but is not authenticated UTC
and is not a persistent trusted clock across reboot.
[MS-15](#ms-15-clock-observation)

## 9. Transaction, failure and recovery design

### 9.1 Apply sequence

1. Serialize NevroTick's own relevant maintenance operations and identify the
   current installed instance and recovery owner.
2. Revalidate the approved proposal, actual actor/request binding and all
   material preconditions immediately before the effect.
3. Write the protected intent and before-state needed to reconcile the bounded
   delta. Establish that recovery remains usable across the next transition.
4. Invoke the fixed narrow mutation. Treat failure and uncertain completion
   according to observed effects, not merely a top-level success/failure flag.
5. Read back the relevant policy state and preserve any divergence. Record the
   actual result or indeterminate outcome durably.
6. Report policy state separately from activation. Admit a verified selected
   configuration only after the profile's required effective observations.

The journal and readback do not eliminate races with outside administrators.
Equality of a current value with the intended value does not by itself prove
which actor wrote it or that no intervening change occurred. Preserve that
ownership uncertainty rather than treating state equality as a global lock.

### 9.2 Restoration rules

Restore only a delta NevroTick actually owns and is currently authorized to
reconcile. Do not overwrite a later known administrator, management-policy or
successor installation decision with a whole-machine historical snapshot.
Adding an assignment that was absent before this feature ran is not restoration.

After partial failure, helper death or ambiguous concurrent change, retain the
actual/possible obligation and require supported reconciliation. If ownership
cannot be determined safely, explain the ambiguity and use an explicit
authorized recovery decision. Neither blind re-addition nor silently discarding
the journal satisfies the contract.

Disabling the feature, update, rollback and uninstall must use the same ownership
rules. Repeated disable/removal attempts must not accumulate rights, repeat
unbounded changes or lose the first unsettled obligation. Reinstall and successor
versions must distinguish inherited obligations from a new default installation.

### 9.3 Independent recovery is an admission requirement

Recovery needs all four elements until the obligation is durably settled:

1. Trusted compatible executable code.
2. Protected necessary state.
3. Current authority for the bounded reconciliation.
4. A supported way to invoke it under the actual lifecycle transition.

Test the hard case: every NevroTick service is already stopped or crashed, an
obligation remains pending, and removal begins through Settings, Reset or the
supported administrative path. Normal service-stop cleanup cannot cover this
case by assumption. Package files retained through a lifetime reference prove
neither invocation nor current authority.

Microsoft's `desktop6:UninstallAction` documentation restricts that mechanism to
the stated MSIXVC desktop-game scenario. It is not a generic uninstall hook for
this utility. Package deregistration/destaging and virtualized cleanup likewise
do not prove restoration of external LSA or W32Time effects.
[MS-13](#ms-13-uninstall-and-package-lifetime)

Follow the existing
[independent-removal work](windows-msix-store.md#76-independent-removal-with-outstanding-recovery--5-november-2026).
Use its inert pending-obligation discriminator before real policy mutation. Do
not add an extra persistent watchdog or fallback installer to evade that gate.

## 10. Management, drift and controller conflicts

Management provenance is a supported-profile question, not a single Boolean
returned by one API. Domain state, MDM registration, RSoP and the actual source of
the relevant policy provide different information. A failed query, unavailable
Server API or absence of one management mechanism leaves the remaining question
open. The first implementation should prefer accurate limited guidance over an
unsupported claim of standalone ownership. [MS-08](#ms-08-management-provenance)

Read back at defined bounded points: before proposal/application, after reached
mutation boundaries, before reporting the selected verified state, during
relevant lifecycle transitions and on explicit reassessment. If later runtime
checks are useful, qualify their cost and exact detection scope within existing
owners. Do not invent a universal instantaneous user-right notification.
[MS-09](#ms-09-policy-notifications)

External divergence invalidates the affected assessment. It does not authorize
NevroTick to continually remove rights, stop unrelated services or reset the
clock against a current competing owner. W32Time ownership and configuration
restoration remain the existing transaction, not an implicit side effect of
viewing or accepting this permission proposal.

Optional audit information can improve diagnosis, but must not expand the
broker's privilege profile. Event 4616 and service time-change notifications are
evidence of observed changes, not preventive interlocks or complete evidence of
all possible tampering. [MS-15](#ms-15-clock-observation)

## 11. Backup retention and time-source interaction

This feature must not promise that manipulating the PC clock can no longer
expire backups. An application may observe a changed clock and delete data
before a corrective callback runs. NevroTick does not own that application's
retention transaction or know whether its records can be restored.

For NevroTick's own elapsed-time, retry and freshness decisions, keep monotonic
durations separate from UTC timestamps and invalidate affected state on the
existing discontinuity/lifecycle boundaries. Across reboot, persistent time and
authority require their actual evidence; a stored wall-clock value or QPC
sample is not a tamper-proof UTC oracle.

For third-party retention, the dependable boundary must be supplied by the
backup/storage system's own design and threat model. Remote or immutable
retention may provide a different enforcement boundary, but is not conferred
by installing NevroTick. If product documentation gives examples, independently
verify the specific mode and limits; an example such as
[S3 Object Lock](https://docs.aws.amazon.com/AmazonS3/latest/userguide/object-lock.html)
does not imply that every mode, credential or backup product has equivalent
protection.

Khronos/Ananke and Roughtime address different parts of time assurance. This
feature does not add a new source quorum or clock controller, replace the
Roughtime cold-date bootstrap, change NTS authentication, or create an extra
permission for preliminary clock writes. The later
[time-assurance plan](time-assurance-integration.md) retains those contracts.

## 12. Proposed user experience

The normal UI remains unelevated. The product should guide the user through a
small number of understandable steps, with elevation only for the admitted
maintenance operation:

1. **Assess:** show current scope, remaining authorities and any gaps.
2. **Review:** identify the exact Administrators grant change, affected behavior,
   activation requirements and qualified restoration path.
3. **Approve:** bind the user's decision to that current proposal through the
   existing trusted helper workflow.
4. **Apply and activate:** distinguish recorded policy from pending activation;
   show actual failures and recovery obligations.
5. **Reassess or restore:** provide the supported path and preserve external
   administrative changes.

Candidate explanation, subject to the existing UI text and accessibility review:

> Reduce selected direct clock-changing permissions. Existing privileged
> processes, required Windows services and administrative control may still be
> able to change the clock. NevroTick will show the changes, activation needs
> and recovery status for this installation.

Never label the action “exclusive time authority,” “trojan-proof clock” or
“backups protected.” A failed query cannot leave a stale protected badge.
Cancellation before the effect should leave policy unchanged; cancellation
after a possible effect cannot erase the need to observe and reconcile it.

At the scheduled UI stage, reuse
[S0.5](../../tasks/s0.5-ui-comparison.md), the
[Slint GUI handoff](../../tasks/slint-gui-handoff.md) and the
[localization contract](ui-localization.md). Validate keyboard/screen-reader
presentation, localized principal labels, long error states and the distinction
between available service, good time and verified permission configuration.
This plan does not implement or approve new production UI strings.

## 13. Acceptance and adversarial case matrix

These feature-local case labels refine existing CHECK owners. They are planned
observations, not new global acceptance identities or executed results. Every
negative case needs a legitimate neighboring success and an observer independent
of the implementation decision being tested.

| Case | Distinguishing observation and healthy control | Evidence layer / owner |
| --- | --- | --- |
| CLK-T01 Direct versus inherited | A direct-grant account and a group-inheriting account produce distinct evidence; direct enumeration is never labelled all writers. | Portable contract plus native LSA/token comparison; W11-W05, CHECK-1/2/27. |
| CLK-T02 Empty versus unknown | Documented empty enumeration, complete nonempty list without Administrators, denied query and incomplete result remain distinct. | Adapter/domain controls plus native status observation; W11-W05. |
| CLK-T03 Present versus enabled | A disabled-but-present time privilege cannot satisfy the absent-privilege claim; legitimate exact product profiles remain admissible. | Actual token inspection; CHECK-1/2/27. |
| CLK-T04 Old versus new tokens | After policy removal, an existing token is not described as revoked; newly created qualified tokens show the expected effect. | Disposable native profile with independent token observation. |
| CLK-T05 No-op ownership | Already-absent Administrators grant creates no product-owned removal and no later automatic re-addition. | Transaction model and native readback; CHECK-13/15/23/27. |
| CLK-T06 Narrow effect | Only the fixed SID/right changes; Local Service, other principals and unrelated rights retain their original state. | Before/after independent LSA observation. |
| CLK-T07 Broker continuity | The broker still has independently established required authority; engine/UI remain unprivileged for clock writes. | Qualified installed identities/tokens and actual effect controls. |
| CLK-T08 Role/process mismatch | Wrong account, extra privilege, shared-process union or wrong installation cannot pass admission; the legitimate own-process profile can. | Service/native package evidence; CHECK-1/2/27. |
| CLK-T09 Management eligibility | Managed, unknown and unsupported Server observations refuse Apply while a qualified standalone profile reaches review. | Contract controls plus profile-specific management evidence. |
| CLK-T10 Stale consent | Change a grant, management source, generation or recovery route between preview and effect; reject the stale proposal without substituting a new operation. | Actual helper/request boundary; CHECK-2/3/25. |
| CLK-T11 Write versus activation | Successful LSA write/readback alone produces activation-pending status when effective-state requirements remain open. | Helper/UI integration and native token observations. |
| CLK-T12 Step and rate effects | The engine/UI and fresh test tokens whose only time-right source was the removed Administrators grant are denied both relevant setter classes; preserved required system exceptions remain explicitly reported, and the authorized broker succeeds within bounded fixture effects. | Native `SetSystemTime` and adjustment API tests with independent clock/token observation; CHECK-5/12/27. |
| CLK-T13 Partial operation | Fail before effect, after possible effect, after API return and before durable result; retain the actual/uncertain obligation. | Model/fault controls followed by admitted native lifecycle evidence. |
| CLK-T14 Death and restart | Helper death, product restart and reboot preserve unresolved state and cannot turn unknown into success. | Recovery and installed startup; CHECK-13/15/23/27. |
| CLK-T15 Outside interference | Concurrent administration or policy refresh produces conflict/unknown handling, without automatic overwrite loops or an atomicity claim. | Controlled native interference with independent state observation. |
| CLK-T16 Restore ownership | Restore an actually owned delta when valid; preserve later known administrator/GPO/successor decisions and stop ambiguous automatic reconciliation. | Recovery model and native restore controls. |
| CLK-T17 Independent removal | All product services are already stopped/crashed and an obligation remains; supported removal still supplies code, state, authority and invocation until settlement. | Inert discriminator first, then exact installed lifecycle; W11-W10, CHECK-13/18/23. |
| CLK-T18 Update and rollback | Old/new versions, package transactions and interrupted rollback retain one current recovery owner and truthful obligation state. | Final supported package operations; CHECK-13/18/23/27. |
| CLK-T19 Stale UI and cancellation | Missing telemetry, elapsed freshness, sleep/discontinuity and cancellation cannot retain a misleading verified state or discard a possible effect. | Contract/UI and native integration; CHECK-14/22. |
| CLK-T20 Diagnostic minimization | Useful failure evidence omits unnecessary account-directory/private data and never logs authorization secrets. | Diagnostic boundary checks; W11-W08, CHECK-17. |
| CLK-T21 Controller independence | Permission assessment does not take over W32Time or authorize a stale broker correction; actual competing ownership remains respected. | Existing P4 ownership/discontinuity controls; CHECK-5/12/27. |
| CLK-T22 Removal/reinstall repetition | Repeated disable, uninstall and reinstall do not accumulate rights, erase pending obligations or restore a grant never removed by this installation. | Stateful lifecycle and native package controls. |
| CLK-T23 Baseline independence | Fresh install is safely provisioned before normal work even when this optional feature is never opened or enabled. | Complete-MSIX installation and role admission; CHECK-1/13/18/23. |

Pure tests establish contract behavior. Cross-builds establish only their
compilation scope. Native tests establish the exact tested Windows profile;
package tests establish the actual installed lifecycle. No layer substitutes
for the next. Existing coverage/mutation and security qualification requirements
apply when their implementation surfaces are reached.

Permission and lifecycle experiments do not need PPS/GNSS hardware or a calibrated
UTC reference merely to establish privilege denial and recoverability. Keep
accuracy qualification separate. Any real clock or service operation must use
the currently admitted, owned disposable Windows environment, bounded effects,
independent observation and restoration. This document grants no host-clock or
Windows execution authorization.

## 14. Staged implementation and existing owners

Start this sequence at the scheduled CLK-0 entry. Preserve useful source work and
evidence from the current baseline; do not repeat unchanged feasibility studies.

| Stage | Work and observable exit | Prerequisite / stop condition |
| --- | --- | --- |
| CLK-S0 Reassess | Pin current repository/Windows/source identities; reconcile this plan with MT-C01, accepted decisions and actual service/package evidence. Produce a bounded current design and open-input record. | MT-C01 and the current execution context. Unsupported prerequisites receive precise owners and re-entry evidence. |
| CLK-S1 Specify assessment | Define complete/empty/unknown observations, inheritance gaps, effective-token evidence, management classification, immutable proposal, consent invalidation and recovery-unavailable refusal. Obtain independent plan review. | Verified current owners and source facts. This stage needs no live policy mutation. |
| CLK-S2 Implement observation | Add bounded read-only adapters and actual consumers through existing Windows/maintenance owners. Exercise valid/invalid neighboring cases and truthful UI projection. | Profile-specific query access and existing process/IPC contracts. Unknown remains usable information, not an Apply admission. |
| CLK-S3 Qualify lifecycle dependencies | Establish installed identity, independent broker authority, pre-start baseline and usable recovery. Run the existing inert pending-obligation removal discriminator. | Existing W11-W05/W11-W10 admission. Stop persistent mutation if any required recovery element is missing. |
| CLK-S4 Qualify narrow mutation | Implement the fixed delta, protected intent/result, stale-consent rejection, activation and restoration on admitted disposable profiles. Exercise failure/concurrency boundaries and both setter classes. | S1–S3 evidence for the exact candidate; current native authorization. |
| CLK-S5 Integrate product experience | Connect the real unelevated UI, trusted helper, status, activation/recovery guidance, diagnostics and localized/accessibility presentation. | Implemented backend contracts and qualified profile semantics; no mock result advertised as protection. |
| CLK-S6 Record CLK-C01 | Independently reconcile implementation, native/package evidence, supported profiles, known limits and disposition; update this plan and roadmap, then hand off to KHR-0. | Reached guarantees demonstrated, or precise unresolved outcome and any required owner decision recorded. |

### 14.1 Ownership and canonical acceptance map

| Concern | Existing owner | Canonical checks |
| --- | --- | --- |
| Privileges, observation and admission | Architecture §4.1.1/§4.2.1; W11-W05; existing S0.2/S0.3/P3 owners. | CHECK-1/2/27. |
| Helper authorization and request binding | Existing maintenance/IPC owners under W11-W05. | CHECK-2/3/25. |
| Actual clock effects, competing controllers and discontinuities | Existing P4 broker/ownership work. | CHECK-5/12/27. |
| Protected intent, result and restoration | Architecture §11–§12; existing mandatory-state and W11-W05/W11-W10 owners. | CHECK-13/15/23/27. |
| Supported complete-MSIX identity and independent removal | Architecture §19.4; MSIX plan §7.5–§7.6; W11-W10/P6. | CHECK-1/13/18/23. |
| Consent, status and accessibility | Existing feature plan, Slint handoff and S0.5/P5 owners. | CHECK-14/22. |
| Diagnostic content | W11-W08 and existing diagnostic contract. | CHECK-17. |

CLK-S/T/C labels organize this feature only. Do not fork the existing task tree
or invent new global REQ/CHECK authorities. Import each reached assignment into
its current task and preserve one evidence chain.

### 14.2 First executable slice on resumption

After the entry reassessment, the first design slice is the **read-only
clock-permission assessment and immutable proposed change set** under
W11-W05/W11-W10. Its exit is a complete input/output/transition contract covering
the evidence groups in §6, unknown/empty outcomes, activation, freshness,
management provenance, exact consent and recovery-unavailable refusal.

The initial investigation budget is one bounded source/design pass and the
smallest discriminating admitted observation needed for each material open
mechanism. If a required input or method is unavailable, record it and stop that
dependent action. Retry only after evidence, platform support, authorization or
a justified method changes; do not restart the same research or hardware
inventory loop at every handoff.

## 15. Open decisions, risks and reconsideration criteria

| Question | Evidence needed before dependent implementation | Initial owner/disposition |
| --- | --- | --- |
| Is the one-grant restriction useful enough to ship? | Explicit user benefit and compatibility impact compared with accurate read-only assessment; no unsupported administrator-exclusion claim. | Product/security review at CLK-S0; exact mutation remains proposed. |
| Which standalone profiles can be established reliably? | Current management-provenance method, failure behavior and profile-specific evidence, including Server limitations. | W11-W05/W11-W10. Unknown is guidance-only. |
| Does the actual broker keep independent authority? | Installed account/token evidence before and after the candidate removal and next-start behavior. | Existing privilege/provisioning owner. No inferred service-SID shortcut. |
| What makes activation complete? | Old/new token observations, supported logon/service/reboot sequence and honest limits on unrelated processes. | W11-W05 plus UI status owner. |
| What survives independent package removal? | Compatible code, protected state, authority and supported invocation with services dead and obligation pending. | W11-W10/P6. Persistent mutation held until qualified. |
| How are concurrent administrative changes reconciled? | Narrow actual-owned-delta rules, ambiguous outcomes and explicit recovery decisions; no fictional multi-resource transaction. | Existing recovery owner. |
| How much drift observation is justified? | Bounded mechanism, cost and coverage; no undocumented notification guarantee or repeated policy fight. | W11-W05/W11-W08. |
| Should additional principals ever be changed? | Separate compatibility/security review and an explicit bounded proposal. | Deferred; not silently included in this profile. |
| What happens if mutation cannot qualify? | Exact blocker, alternatives, useful assessment-only outcome and requested-sequence implications. | CLK-C01; OD-04 only for the material unresolved choice. |

The principal risks are overstated protection, disrupted legitimate clock
administration, incomplete activation, fighting management policy, and losing
recoverability during removal. The mitigation is evidence-bound scope and
ownership, not a larger privileged component or more optimistic status text.

## 16. Reassessment and maintenance protocol

This is a living plan. Reassess it at CLK-S0, before enabling Apply on a new
profile, and whenever a material assumption changes. Relevant triggers include:

- Changed Windows privilege, LSA, service-account, management or package-lifetime
  documentation or observed behavior.
- Changed installed service identity, required privilege profile, helper
  admission, IPC, signing or complete-MSIX delivery mechanism.
- New evidence from the existing service-ownership or independent-removal work.
- Changes introduced by TRUST-0 or MT-0 that affect authority, worker lifecycle,
  freshness, cancellation, UI projection or recovery ownership.
- A newly observed bypass, unexpected policy race, restoration ambiguity or
  compatibility failure on an intended supported profile.
- A new owner decision about feature scope, supported platforms or roadmap order.

At each reassessment:

1. Record the date, exact repository/package/Windows identities, current plan
   revision and the reason for reopening the affected conclusions.
2. Re-read the relevant current primary sources and actual canonical contracts;
   preserve prior findings under their original identity and date.
3. Distinguish unchanged facts, changed facts, inference, rejected options,
   unknowns and actual test results. Update the affected sections and links.
4. Reconcile the existing task/CHECK owners and all active scheduling handoffs.
   Do not rewrite historical receipts as if the new result existed earlier.
5. Obtain independent review of consequential changed boundaries, preserving
   the reviewer's scope, comprehension, findings and actual completion.
6. State the next executable slice, its finite observation budget, blocking
   prerequisite if any, and a concrete re-entry condition.

Research currency is not a native support claim. A source may remain correct
while the selected implementation fails its contract. Conversely, a later source
change invalidates only the affected conclusions and dependent evidence; it
does not require repeating every unrelated check.

### 16.1 Initial research and review record

The predecessor is the Microsoft Learn investigation and independently reviewed
feature proposal from this conversation on 27 November 2026. It was reconciled
from repository `c27043a8` to `acbb85bd` before this document was requested.
The user then selected the future roadmap position and asked to preserve the
detailed plan here. That changes scheduling, not the truth of the remaining
provisioning and recovery gaps.

The prior fresh-context review admitted only the next read-only source/design
slice. Its useful refinement was to preserve documented empty direct-grant
results separately from failed observation. This document incorporates that
refinement in §4, §6 and CLK-T02. The review did not execute native APIs or
approve a production mutation path.

At initial publication, the evidence remains source research and documentation
review. No feature code, Windows policy, service, clock, package or native
qualification result is created by this plan.

### 16.2 Revision ledger

| Date | Revision | Change and evidence scope |
| --- | --- | --- |
| 27 November 2026 | 1.0 | Preserve the researched feature proposal, accepted placement after MT-C01/before KHR-0, initial candidate, limits, ownership, cases, source links and future reassessment protocol. Implementation/native evidence remains open. |

Future entries should identify the changed decision or evidence, affected
guarantee, owner and links to actual results. Do not fill the ledger with routine
wording edits or convert unexecuted test plans into passes.

## 17. Primary-source catalogue

These links support the material Windows claims. Re-fetch the relevant full
pages at reassessment; a documentation access date is not an operating-system
qualification date. The Windows policy article may redirect to an older-version
documentation path; verify applicability for each actual target profile.

### MS-01 Change the system time

- [Change the system time — policy, defaults, risks, activation and management](https://learn.microsoft.com/en-us/windows/security/threat-protection/security-policy-settings/change-the-system-time).

### MS-02 Direct right enumeration

- [`LsaEnumerateAccountsWithUserRight` — direct assignments, required policy access and empty result](https://learn.microsoft.com/en-us/windows/win32/api/ntsecapi/nf-ntsecapi-lsaenumerateaccountswithuserright).

### MS-03 Account policy and existing tokens

- [Assigning privileges to an account — changes and existing user tokens](https://learn.microsoft.com/en-us/windows/win32/secbp/assigning-privileges-to-an-account).

### MS-04 Token privileges and observation

- [Privileges — direct/group-derived token privileges](https://learn.microsoft.com/en-us/windows/win32/secauthz/privileges).
- [`GetTokenInformation` — token observation and access requirements](https://learn.microsoft.com/en-us/windows/win32/api/securitybaseapi/nf-securitybaseapi-gettokeninformation).
- [LocalSystem account — documented account authority](https://learn.microsoft.com/en-us/windows/win32/services/localsystem-account). An account's general capabilities do not establish every restricted service token.

### MS-05 Service required privileges

- [`SERVICE_REQUIRED_PRIVILEGES_INFOW` — process-start filtering, shared-process union and `SeChangeNotifyPrivilege`](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/ns-winsvc-service_required_privileges_infow).

### MS-06 Narrow policy mutation

- [`LsaAddAccountRights`](https://learn.microsoft.com/en-us/windows/win32/api/ntsecapi/nf-ntsecapi-lsaaddaccountrights).
- [`LsaRemoveAccountRights`](https://learn.microsoft.com/en-us/windows/win32/api/ntsecapi/nf-ntsecapi-lsaremoveaccountrights).
- [Managing account permissions](https://learn.microsoft.com/en-us/windows/win32/secmgmt/managing-account-permissions).

### MS-07 UserRights CSP

- [Policy CSP — UserRights / ChangeSystemTime](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-userrights#changesystemtime). Preserve its Local Service requirement and list-replacement scope when comparing mechanisms.

### MS-08 Management provenance

- [`IsDeviceRegisteredWithManagement`](https://learn.microsoft.com/en-us/windows/win32/api/mdmregistration/nf-mdmregistration-isdeviceregisteredwithmanagement).
- [`gpresult` — Resultant Set of Policy information](https://learn.microsoft.com/en-us/windows-server/administration/windows-commands/gpresult).
- [Policy CSP — ControlPolicyConflict](https://learn.microsoft.com/en-us/windows/client-management/mdm/policy-csp-controlpolicyconflict).

### MS-09 Policy notifications

- [`LsaRegisterPolicyChangeNotification`](https://learn.microsoft.com/en-us/windows/win32/api/ntsecapi/nf-ntsecapi-lsaregisterpolicychangenotification).
- [`POLICY_NOTIFICATION_INFORMATION_CLASS`](https://learn.microsoft.com/en-us/windows/win32/api/ntsecapi/ne-ntsecapi-policy_notification_information_class).

### MS-10 Windows security boundaries

- [Microsoft Windows security-servicing criteria](https://www.microsoft.com/en-us/msrc/windows-security-servicing-criteria).

### MS-11 System-time setters

- [`SetSystemTime`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-setsystemtime).
- [`SetSystemTimeAdjustment`](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-setsystemtimeadjustment).

### MS-12 Packaged-service identities

- [`desktop6:Service` — packaged-service schema and startup accounts](https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/element-desktop6-service).

### MS-13 Uninstall and package lifetime

- [`desktop6:UninstallAction` — limited supported scenario](https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/element-desktop6-uninstallaction).
- [MSIX package lifetime](https://learn.microsoft.com/en-us/windows/msix/package/package-lifetime).
- [Understanding how packaged desktop apps run — virtualization and cleanup](https://learn.microsoft.com/en-us/windows/msix/desktop/desktop-to-uwp-behind-the-scenes).

### MS-14 Service stop and failure behavior

- [`SetServiceStatus` — cleanup and stopped-state lifetime](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-setservicestatus).
- [`SERVICE_FAILURE_ACTIONSW` — scoped service recovery actions](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/ns-winsvc-service_failure_actionsw). Service failure actions do not by themselves establish independent package-removal recovery.

### MS-15 Clock observation

- [`HandlerEx` — service control and time-change notification](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nc-winsvc-lphandler_function_ex).
- [Event 4616 — system time changed](https://learn.microsoft.com/en-us/windows/security/threat-protection/auditing/event-4616).
- [Acquiring high-resolution time stamps — QPC semantics and limits](https://learn.microsoft.com/en-us/windows/win32/sysinfo/acquiring-high-resolution-time-stamps).
