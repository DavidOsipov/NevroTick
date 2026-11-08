# Release policy and incident recovery

**Prepared:** 4 November 2026.  
**Status:** operational specification; support baseline proposed for first-stable
approval. No stable release, completed rehearsal or native qualification is
established by this document.  
**Release approver:** David Osipov, under [OD-05](nevrotick-owner-decisions.md#7-od-05--signing-and-release-ownership).

## 1. Scope and authority

This document owns supported-release records, source backports, release promotion,
bad-release containment and recovery of compromised publication authority.
[SECURITY.md](../SECURITY.md) remains the vulnerability-intake authority.
[Architecture §11](windows-time-architecture.md#11-w32time-ownership-and-installation-lifecycle),
[§12.1](windows-time-architecture.md#121-state-separation) and
[§19.4](windows-time-architecture.md#194-future-complete-msix-gate) own the
technical lifecycle, protected-state and complete-package requirements.

The selected target is one complete hardened Store MSIX, including services.
Source/design preparation, permitted local mechanics, David's controlled Store
submission and actual delivered-artifact qualification remain separate stages.
MSI delivery remains a later owner choice. [OD-05 §7.7](nevrotick-owner-decisions.md#77-sign-every-supported-shipped-artifact--8-november-2026)
requires the signed package and every supported individual file signature on
either route; the unsigned-MSI/OpenPGP exception is superseded. SignPath can be a
candidate for required inner signatures even on MSIX. This policy
selects no TUF protocol, custom updater, remote kill switch, secondary installer
or new privileged recovery service.

The [existing Windows task](../tasks/windows11-integration.md) owns execution
records under W11-W10, with W11-W05 and the existing release, security, storage
and clock owners. A document review does not close OPEN-3/11/12/14, supply a
supported provisioning/removal bridge or lift a hold. Current Core priority and
the [current execution dispositions](plans/development-plan.md#7-next-executable-task-for-a-development-agent)
continue; the [narrow owned-Sandbox scope](../tasks/windows11-integration.md#419-owner-ready-sandbox-scope--3-november-2026)
does not authorize unrestricted Windows, fuzz, storage or publication work.

| Responsibility | Owner and required output |
| --- | --- |
| Stable-support commitment, exact candidate, Store account/submission/visibility and broad release | David. Record the scope/date of each decision; preparation and review confer no publishing authority. |
| Incident coordination and containment decision | David, or a specifically recorded bounded delegate. Identify the incident owner and next action; no backup person or continuous coverage is presumed. |
| Source/build custody and corrective candidate | Existing build/release owner. Produce the bound source, package and compatibility evidence. |
| Clock, mandatory state and lifecycle recovery | Existing W11-W05/platform/security/storage owners. Supply a qualified recovery route for the affected installed state. |
| Independent candidate review | Existing CHECK-24 reviewer. Reconcile actual evidence and residuals; agent review is not another human release approval. |

A credible protected-effect, state-continuity or release-trust failure stops
further approval/expansion while assessed. External account, credential, Store
and advisory actions remain with their authorized human owner. This preparation
does not perform or authorize those actions.

## 2. Support status and proposed baseline

### 2.1 Current published commitment

| Population, as of 5 November 2026 | Support status |
| --- | --- |
| Production stable releases | None declared. No stable series, backport schedule, support lifetime or fixed response/remediation deadline is established. |
| Development commits, snapshots, CI artifacts and prototypes | Experimental scope only. Reports about current and older revisions remain welcome through SECURITY.md. |
| Intended initial platforms | [OD-14](nevrotick-owner-decisions.md#18-od-14--compact-windows-platform-scope) selects Windows 11 26H2 x64 and Windows Server 2025 x64 with Desktop Experience. Exact OS/CPU/protection profiles, supported delivery and package transitions require row-specific qualification; this direction approves no stable-support commitment. |

A source tag, a GitHub release, a private Store candidate or Store certification
does not establish production support. Availability of archived bytes does not
make them recommended for installation.

### 2.2 Proposal for the first stable release

**Pending David's approval:** maintain the latest explicitly designated,
qualified, non-withdrawn stable release for its declared platform matrix. Put
ordinary fixes in a qualified successor. Do not infer the supported target from
the greatest package number, a movable tag or an automatic `latest` label.
“Qualified” means the required evidence and owner approval exist; it does not
promise that no undiscovered defect remains.

During a partial rollout, retain the previous qualified stable for users still
served that version until the replacement is actually obtainable through their
declared supported route and the transition decision is recorded. State the
temporary overlap and its end condition. This is a migration obligation, not an
indefinite promise to maintain two branches. If the predecessor is also unsafe,
withdraw it as appropriate and state the resulting support gap.

For each ordinary supersession or planned end of support, record the affected
package/platform, effective date, reason, replacement and migration limitations.
Raising an OS/CPU minimum can strand users even when a new version exists;
resolve that population explicitly before ending its support. No numerical
notice period, multi-year LTS term, 24/7 coverage or fix/publication deadline is
adopted. An urgent safety withdrawal may precede advance notice.

Prioritize credible loss of clock authority, compromised release lineage,
mandatory-state corruption, exposed secrets and failed recovery according to
reachable impact and affected users. A severity label is not a delivery promise.
Older-version reports still inform investigation; accepting one does not promise
a patch for every historical version.

### 2.3 Backports and withdrawn versions

A **backport** applies a selected source fix to a release branch to produce a new
candidate. Record the originating fix, dependencies, target branch/package,
eligible users, regression case, state effects, owner and end condition. Any
parallel maintenance exception needs a concrete owner commitment and evidence;
the default proposal creates no general backport program.

A source reversion may be the best repair. The new package must retain required
current protections and readers for mandatory state already written. Reverting
code does not authorize restoring all pre-update data or downgrading an installed
package.

When the newest release is quarantined or withdrawn, assess the predecessor
independently: shared defect, provenance, distribution eligibility, platform
applicability and compatibility with affected state. It may be suitable only for
new installations. If no package meets the required conditions, state **no
currently approved stable release** and publish only qualified mitigation and
the known recovery gap. A prior version never becomes safe by subtraction.

### 2.4 Release-status record

Keep one public supported-version/status table linked to the existing release
evidence record. Populate real entries only after the applicable decision.

| Required field | Meaning |
| --- | --- |
| Release and package identities | Marketed version; source commit; exact submitted and delivered package identity/version; channel/flight; predecessor and correction links. |
| Status, scope and date | Candidate, controlled trial, approved stable, transition overlap, superseded, quarantined or withdrawn; reason and decision owner. |
| Supported matrix | Qualified Windows/CPU/policy rows and compatible upgrade/recovery predecessors; explicit unsupported/unverified combinations. |
| Evidence and approval | Immutable source/build/artifact references, applicable CHECK results, known residuals and exact-candidate approval. |
| End or incident disposition | Effective support end/transition condition, affected population, advisory and available recovery path. |

Quarantine suspends recommendation while impact is assessed. Withdrawal records
a decision that the release must no longer be recommended. Either can concern
authentic signed bytes; neither proves installed copies have stopped. Preserve
historical identity/status changes instead of rewriting old approvals.

## 3. Candidate custody and promotion

### 3.1 Required release record

Bind approval to an auditable candidate evidence record containing:

- Source commit/tree, release branch/tag, reviewed changes and immutable workflow
  inputs; toolchain/SDK, lockfile, features, build/native dependencies and licenses.
- Complete UI/engine/broker/helper and dependency inventory; hashes of the local
  candidate, submitted package and actual delivered artifacts kept separately.
  Reconcile manifests, catalog, installed payloads and SBOM after Store processing.
- Bundled public-data inputs/generator/revision/digests, including the selected
  leap calendar and any admitted catalogue generation. Data travels through the
  approved complete release; this record adds no separate update channel.
- Package/publisher/product identity, submitted/delivered version, capabilities,
  platform applicability, launch paths, effective mitigations and helper/IPC
  admission evidence. Signer class alone is not product-role authority.
- Each claimed [client/Server profile](../tasks/windows11-integration.md#432-two-family-windows-qualification-matrix--5-november-2026):
  edition/channel/install mode, build/UBR/KB, CPU/OS state, effective policy and
  actual acquisition/licensing/update route; bind its CHECK results, timing/power
  and resource evidence separately from fixture availability and suitability.
- Supported component/schema/generation transitions from §6, actual lifecycle
  results, wrong-clock/offline verifier cases and remaining blockers.
- [Artifact content-trust evidence](windows-time-architecture.md#196-authenticated-artifact-content-and-consumer-trust):
  actual signature source/profile, approved complete-file SHA-256/lengths,
  applicable strict-padding and consumer controls, with observed, not-run and
  reasoned non-applicable outcomes kept distinct. Use the
  [existing verification procedure](windows-time-signing-guide.md#52-pe-certificate-padding-and-exact-final-bytes)
  and W11-W10/S6.4 receipt; a catalog result cannot supply embedded-padding evidence.
- [Complete required signature inventory](windows-time-signing-guide.md#41-required-signatures-for-the-shipped-inventory):
  actual format/nesting, expected project or approved upstream signer, separate
  individual/package results and post-Store preservation. Reconcile every shipped
  supported signable file, including nonbinary formats; retain exact bytes and
  unresolved capability/coverage blockers. No catalog-only or OpenPGP exception.
- Audience, visibility, flight/rollout state, observation/stop criteria, recovery
  target, approval owner/date and subsequent publication/availability observations.

Changing a bound package, generated input or workflow invalidates affected
evidence. Rebuilds receive their own identities. Never move a release tag or
replace different bytes under the same approved artifact identity. Checksums and
attestations identify material/provenance; they do not prove benign source or
authoritative approval. [W07, W09]

Section 3.3 distinguishes this required custody record from optional enhanced
build provenance. The record does not require an independent bit-for-bit rebuild
or a GitHub/Sigstore attestation.

### 3.2 Promotion sequence

1. **Prepare:** establish the complete source/design profile and applicable
   architecture gates. Define compatibility and recovery before a new writer
   can modify mandatory state.
2. **Qualify local mechanics when admitted:** inspect the actual candidate and
   run the required component/native/lifecycle cases in the permitted environment.
   Record test trust roots and substitutions; they prove no final Store trust.
3. **Controlled submission:** David approves the exact submission candidate and
   intended restricted audience. Configure the appropriate publication hold and
   audience before submission; see §4. A Store signature is not required before
   preparing or submitting this first candidate.
4. **Qualify delivery:** obtain the actual Store-delivered candidate through the
   intended route; reconcile its identity/content and all affected Windows,
   privilege, mitigation, maintenance and recovery observations.
5. **Approve stable/broad availability:** complete independent CHECK-24 review;
   David decides the exact delivered candidate, support table and distribution
   scope. Expansion requires its recorded observation/stop criteria, not merely
   a successful build, certification or elapsed waiting period.

Keep required failures, unexecuted checks and unresolved technical blockers
visible. Emergency urgency can remove unrelated optional work; it cannot convert
missing mandatory release evidence into a pass. The policy is not a prerequisite
for unrelated portable Core work.

### 3.3 Optional enhanced provenance

[OD-05 §7.5](nevrotick-owner-decisions.md#75-optional-enhanced-build-provenance--4-november-2026)
records David's accepted decision: artifact attestations, independent
reproducible builds and a public source-to-Store evidence bundle are optional
release strengthening. The [research and possible experiment](plans/release-provenance.md)
add no mandatory row to §9 and no prerequisite to §3.2, P6/P7 or release approval.
Their absence alone is not a blocker; report an unperformed or unsuccessful
optional comparison without claiming the stronger property.

Section 3.1's source/build custody, SBOM/licence and exact payload records, and
the existing package trust, integrity, installed identity and native/lifecycle
checks retain their required scope. A detected violation of those requirements
is handled under its existing gate even if found during optional work. This
decision does not approve the proposed first-stable support terms in §2.

### 3.4 WinGet capability and publication — 7 November 2026

[OD-17](nevrotick-owner-decisions.md#21-od-17--winget-capability-before-v1)
requires [WinGet delivery capability](plans/winget-distribution.md) before V1
within P6/P7. Preserve the complete Store MSIX and qualify `msstore` first;
Community publication is conditional on an authorized public trusted complete
installer. W11-W10/W11-W05 own the existing implementation and evidence chain.

Record WGT-C01 pre-V1 capability independently from WGT-C02 externally observed
catalog availability. Bind source/identifier/version/scope to the approved exact
package and supported platform. A generated manifest, successful submission job,
PR merge or source tag is not a working public install or customer upgrade.
External acceptance/propagation carries no promised completion time.

For an activated Community route, prepare reviewed manifest bytes without a
publishing credential, then use the authorized release operation for the
specific version. Final artifact/signature hashes follow all transformations;
re-download and compare before submission. Never replace released bytes in place
or silently adopt changed hashes. Qualify actual release-event propagation,
supported least-privilege credential behavior, duplicate/conflict handling and
source readback. Current GitHub token behavior and transitive tool pinning are
covered in the [automation contract](plans/winget-distribution.md#6-release-automation-design).

David retains account, credential, submission and visibility actions under §1.
No WinGet invocation in the time service, automatic customer update promise or
second installer is added. Removing a catalog entry, withdrawing a Store listing
or changing GitHub availability does not repair/recall installed copies; include
every admitted channel in the existing incident and corrective-release record.

## 4. Distribution containment: choose the actual Store operation

Microsoft's documented controls affect different populations. Recheck the real
product/account and preserve observed results when used; this table is not
evidence of an existing NevroTick submission.

| Situation/control | Operational meaning and required decision |
| --- | --- |
| Submission held before publication | Keep the manual hold while investigating. Once Publishing starts, cancellation/date changes are unavailable; use the applicable later containment route. [MS-12, MS-13] |
| Initial private audience | Select before any public submission. Only named personal Microsoft accounts can acquire it; a public product cannot later become private. An unlisted public URL is not private access control. [MS-03] |
| Later package flight | Available after an initial submission is published. Record named membership, rank and package applicability; the flight can prevent its testers receiving the non-flighted correction. Certification still applies. [MS-04] |
| Gradual update rollout | Random percentage of existing installations on supported desktop versions, including Windows 11; also usable within a flight. It is not first-release audience control or a limit on every new acquisition. [MS-01] |
| Halt active rollout | Record the stopped submission and actual predecessor/fallback. Already updated devices retain their version. A new submission requires halt or finalization; do not finalize a bad rollout to unlock editing. [MS-01, MS-11] |
| Finalize or increase percentage | An intentional expansion decision. Setting 100% and finalization differ; neither proves adoption by offline/deferred devices. [MS-01] |
| Make unavailable / stop acquisition | Contain the applicable acquisition routes and verify the result. Installed copies remain usable. Prior-owner/promotional-code behavior needs the qualification below. [MS-02, MS-03] |
| Reoffer retained older package | Can serve new acquisitions, subject to its separate suitability. It does not downgrade customers already above it; their remedy normally needs a compatible higher-version correction. [MS-05] |
| Remove tester or delete flight | Not local recall. Private-membership removal can cut off later fixes; flight removal does not force a lower version. Preserve a proven corrective route for those users. [MS-03, MS-04] |
| Store mandatory-update metadata | Windows does not enforce installation from this flag. Any app-enforced restriction would need an implemented, separately qualified design; no such mechanism is selected here. [MS-09] |

**Fallback and cloned-submission traps:** evaluate the actual fallback for the
same defect before recommending it. If unsafe, keep that fact in the acquisition
containment decision. The halt API documents that a subsequent submission is
cloned from the halted one: explicitly remove/replace bad packages and reconcile
the complete inventory. Do not assume a fresh submission is clean. [MS-11]

**Acquisition documentation discrepancy:** the removal page broadly describes
unavailability as blocking new acquisition. The MSIX visibility page retains
prior-owner/promotional-code exceptions and internally inconsistent adjacent
wording. The supported common conclusion is that unavailability is not recall.
Verify new customer, former owner, direct-link, promotional-code where applicable,
installed-update and flight cases for the chosen option. Do not promise either
total acquisition blockage or guaranteed corrective-update reach while unavailable.
[MS-02, MS-03]

A GitHub warning, withdrawn release record or removed download pointer supplies
another distribution/status action. It cannot repair Store installations.
If immutable releases are enabled, published assets/tags have different edit
constraints from notes/status; use the actual setting without assuming it is
enabled or making its adoption part of this task. [W06]

## 5. Bad-release runbook

Use one incident record in the existing W11/release evidence chain. The following
steps specify operator decisions and required outputs; they are not a claim that
a working recovery command exists today.

### 5.1 Declare, preserve and stop expansion

Record incident identifier, coordinator, detection/report time and uncertainty,
exact suspected source/build/submitted/delivered versions, symptoms, affected
platforms, reachable protected effects and contrary evidence. Distinguish a
regression from suspected unauthorized publication; enter §7 as well when
publication or build trust is uncertain.

Freeze promotion and prepare the appropriate §4 action for David. Capture useful
artifact/submission/access evidence before removal where feasible. Record the
requested action, actual provider result, time basis and remaining audiences.
Contain credible clock-ownership, mandatory-state or release-trust failures before
claiming a complete impact count. Do not call a suspicion confirmed intrusion.

### 5.2 Classify affected installations

The incident must cover reachable states, not just a fresh install:

| Observed population/state | Required recovery disposition |
| --- | --- |
| Not installed, or downloaded but not yet installed | Stop recommending the suspect bytes; distinguish unavailable acquisition from already held copies. |
| Previous qualified stable still running | Assess shared exposure and retain its explicit support/transition status; no automatic downgrade or automatic safety verdict. |
| Bad version installed, before/after schema migration | Identify actual schema/generation and a correction that reads/migrates it; record unsupported states visibly. |
| Reserved operation, effect or outcome uncertain | Preserve operation identity and charge the uncertainty; follow §6 before further protected effects. |
| Failed activation, interrupted maintenance, stopped/crashed service | Establish actual actuator/ownership and surviving recovery access; Blocked startup alone does not restore the previous time owner. |
| Other-user registration, removal/Reset or queued restart | Apply W11-W05's owner/lifetime contract; payload still on disk is not proof of callable recovery. |
| Offline, policy-delayed or severely wrong clock | Use the qualified verifier/acquisition/recovery matrix; local knowledge may lag public withdrawal. |
| Suspected execution of malicious privileged code | Treat in-product self-report and protected state as potentially untrusted; use §7's independent assessment path. |

Unknown installed state is not a healthy state. Collect bounded read-only
evidence through an admitted path and preserve the uncertainty until resolved.

### 5.3 Select qualified local mitigation

Link the exact affected-version procedure and its evidence before advising a
user to act. A procedure must reach §6's owned maintenance/restoration outcome.
Do not substitute generic service termination, forced downgrade, Reset,
uninstall/reinstall, journal deletion or blanket W32Time enablement.

When the broker may have left a rate programmed, process death is not cleanup.
When a new version refuses startup while its predecessor's takeover remains,
safe refusal is not restored availability. Require a compatible retained
recovery path or an explicit qualified administrator remedy. If none is known,
state that gap, keep dependent release claims blocked and escalate to the
lifecycle/security owner. Do not hide the gap behind “reinstall.”

### 5.4 Prepare a deliverable correction

Choose a bounded fix or reviewed source reversion and bind its new candidate
under §3. Include the causal defect, affected-state fixtures and valid neighboring
operation. A trusted clean installation alone is insufficient.

For the normal Store path, make the correction greater than every affected
applicable installed version, including exposed flights and Store-modified
version components; verify actual applicability and installation from each
required predecessor. The Store owns the submitted version's fourth component
under its Windows 10/11 rules, and numeric fields are bounded. Qualify the actual
desktop package rules and reserve usable version headroom. [MS-05]

A compromised publisher might consume that headroom. If an admissible higher
version, recovered identity or required state transition is unavailable, keep
the incident unresolved and bring a concrete provider-assisted recovery choice
to David. Do not silently change package identity, add a sideload/MSI route,
force a downgrade or reset authority. App Installer's separate downgrade feature
does not establish ordinary Store recovery and is not selected. [MS-10]

### 5.5 Requalify, publish and observe

Run defect-specific and affected lifecycle/security cases plus all applicable
release gates; retain independent healthy controls and exact-candidate review.
Follow §3's local → controlled Store → delivered-artifact → broad-release stages.
A rebuild or changed delivery artifact requires affected evidence reconciliation.

David approves the identified correction and containment/resumption action.
Publish the §8 advisory through an authorized channel; record actual corrected
acquisition and installed-case outcomes separately. Do not infer fleet repair
from publication success, a slider percentage or the absence of reports.
Offline/blocked clients retain explicit follow-up status.

## 6. Mandatory-state compatibility and owned recovery

This section applies existing contracts to incident recovery. It does not create
a second journal or choose a new state format.

### 6.1 Freeze the compatibility matrix before new writes

For each required transition, record old/bad/new UI, engine, broker and helper
versions; stored schema and ownership/maintenance/policy/boot generations; reader,
writer and migration owner; permitted path; interruption phases and expected
refusal/recovery. Mark supported, deliberately unsupported or unverified with
evidence. Do not reclassify a failed required case as unsupported after testing.

Cover clean install, direct and skipped-version update, correction after the bad
writer, partial migration, rollback of a failed package transaction, mixed/old
processes, repair, Reset and multi-user/final removal. One MSIX does not prove
simultaneous process replacement. A compatible predecessor may be restored only
with compatible current authority state; a pre-update snapshot is not that proof.

A repair carrying older application logic must understand the states created by
the bad release, or use an explicitly qualified bridge. Unknown mandatory schemas,
invalid records or indeterminate persistence must not become permissive defaults.

Microsoft distinguishes **Repair**, which retains app data, from **Reset**, which
deletes app data and reinstalls the app. This describes the package's app-data
scope; it does not establish that every separately owned machine-state or
ProgramData record is erased, retained or valid. Qualify the actual mandatory
state locations, their lifecycle and the surviving authorized recovery path for
each operation. Neither option is a generic safe incident remedy. [MS-06]

| State owner/class | Continuity required through correction |
| --- | --- |
| Broker authority ledger | Preserve reserved/consumed allowances, unique operations, applied outcomes and unresolved effects. Missing acknowledgement cannot authorize a repeated relative step or refund. |
| Lifecycle journal | Preserve original W32Time observations and every changed setting, installation/current-owner generation, maintenance inhibition and restoration progress; respect successor and later administrator/GPO state. |
| Machine security/source policy | Preserve current authorized revision/provenance and validate explicit migration through the same domain/authorization rules. |
| Protected time floor | Preserve evidence and supersession history. An intact but wrong floor may use only the narrowly authorized re-anchor contract; it grants no clock write and resets no correction budget or ownership. |
| NTS association/private state | Revalidate source/policy binding, full-authentication/key ages and current generations; old authentic bytes do not establish freshness or permit weaker fallback. |
| Optional priors/cache and user preferences | Apply their documented discard/reacquisition scope; their recovery rule cannot erase mandatory authority. |
| Bundled calendar/catalogue data | Keep admitted public-data provenance tied to the complete release and compatible policy meaning; old code/data cannot revive spent authority. |

See [durable authority](windows-time-architecture.md#1021-durable-authority-reservation-and-uncertain-outcomes),
[restoration validity](windows-time-architecture.md#1211-restoration-validity),
[time-floor recovery](windows-time-architecture.md#recovery-decision-flow) and
the [data-transition contract](../.agents/skills/implementation/evolve-data-contracts/references/transitions.md).

### 6.2 Required owned transition

The implemented lifecycle path must demonstrate these observations in its actual
transaction order, including failures and a successful neighboring transition:

1. Establish authentic admitted recovery code, operation-specific caller
   authority, current installation/ownership generation and required recovery
   availability before mutation. An old journal alone grants no current authority.
2. Enter durable maintenance inhibition before teardown/replacement can allow
   queued SCM, boot/manual or other admitted restart paths to resume effects.
   Quiesce proposals and retain custody of outstanding native work until its
   actual completion or defined recovery; a timed-out waiter is not completion.
3. While current ownership is established, reconcile the actual adjustment mode,
   rate, transient contribution and durable ledger. Cancel only attributable
   product work and settle proven outcomes; keep unresolved reservations charged.
   A numeric rate match or moving wall-clock reread cannot prove ownership or
   whether a particular step occurred.
4. Perform validated migration/replacement through the protected writer and
   defined persistence stages. Reconcile write/flush/publication/acknowledgement
   failures before retry; retain prior useful evidence and explicit uncertainty.
5. For continuing product control, admit exactly the compatible current
   generation after checking rights, mitigations, policy and mandatory state.
   Old cleanup must neither reset the successor nor enable W32Time beneath it.
6. For authorized ownership release/removal, restore only product-owned changes
   according to the original observations and current administrator/GPO/successor
   guards. Read back the actual mode/configuration and record durable restoration
   or honest divergence. No NTS-outage fallback to another controller follows.
7. Retire the last recovery code/journal only after its required durable outcome.
   Interrupted removal, Reset, stopped/crashed service and final-user removal
   need their own supported path; file retention alone does not satisfy this.

Unknown ownership blocks protected writes without a blind global reset. This
can leave a real availability/recovery problem; record it rather than claiming
that refusal restored Windows. Unavailable required recovery blocks takeover
before mutation. The [independent-removal boundary](plans/windows-msix-store.md#concrete-removal-and-recovery-boundary)
remains unresolved and prevents qualification of that path.

### 6.3 Rollback and offline limits

Supported in-product update, repair and recovery must not manufacture unused
authority. Whole-machine/VM snapshot or offline rollback by a privileged actor
can restore the complete observation environment; ordinary ACLs, MACs, encryption
and local generation files do not prevent it. Preserve this
[THR-17 residual](windows-time-architecture.md#thr-17--valid-old-state-restores-obsolete-trust-or-repeats-consumed-authority).
No hardware counter or universal rollback guarantee is adopted.

Withdrawal is public operational status, not an implemented on-device denylist.
An offline installation cannot be assumed to learn it. Its existing ownership,
freshness, authentication and budget rules continue; support-end status itself
neither grants authority nor automatically releases it to W32Time.

On reconnection, use the qualified acquisition and compatibility path. Apply the
[wrong-clock/offline matrix](windows-time-signing-guide.md#51-wrong-clock-and-offline-activation-matrix)
to actual startup/helper/install/update/removal paths, with verifier, trust/time
inputs, allowed retrieval and native completion custody recorded. Neither a warm
cache success nor a missing-cache refusal establishes universal availability.
Required trust failure cannot admit an unverified helper.

## 7. Compromised publisher, repository or build authority

### 7.1 Trust boundary and residual

**A malicious same-product package published through compromised Store authority
may carry a valid Microsoft signature. No independent on-device admission
mechanism has been established that guarantees rejection of that package.**
It may replace the very binaries or local checks expected to enforce approval.
The existing rejection requirements for unrelated Store packages, tampering,
copied-out helpers and inadmissible older artifacts apply at a still-trusted
boundary; they do not demonstrate protection against replacement of that boundary.

The operational approval record is useful only while its provenance is trusted.
Package integrity, CIG signer-category admission, SBOMs and attestations do not
prove benign approved source. Account recovery also does not stop already
installed code. This is the existing
[THR-15 residual](windows-time-architecture.md#thr-15--build-or-signing-pipeline-publishes-trusted-malware),
with OPEN-11's release/installed-code qualification still open. The first-stable
decision must state the verified containment/recovery limits; this document
neither accepts the residual for David nor selects a new verifier. [MS-05, W07]

### 7.2 Readiness before a real stable release

Maintain a private minimal access/recovery inventory: actual MSA or Entra account
type, accountable owner, authorized users/groups/apps, publication permissions,
credential expiration/renewal owner, notification/recovery contacts, provider
routes, independently retained approved artifacts and where protected recovery
material is kept. Record no passwords, tokens, private keys or recovery codes
in the repository, releases, support exports or this policy.

Review it before stable promotion and after authority changes; rehearse recovery,
access removal and the affected renewal/rotation procedures with admitted test
arrangements. Keep recovery material apart from the sole everyday sign-in
device. No backup responder, independent warning channel or quorum is presumed;
name a real arrangement before relying on it. David's absence does not delegate
his approval. [W16]

For Store MSIX, Microsoft supplies outer-package signing during publication; there is no
assumed project-owned Store signing certificate to revoke or renew. The inventory
must address the actual identity and submission credentials, plus the production
signers required for inner artifacts by OD-05 §7.7. Their provider/approval/renewal
and revocation procedures apply even on MSIX. A later selected MSI requires its
own package-specific procedures as well. [MS-05, MS-13]

### 7.3 Contain authority and establish a trusted baseline

1. **Declare uncertainty:** bound the possible compromise interval and affected
   source, workflow, build, publication and communication principals. Preserve
   non-secret IDs, logs, submission history and chronology through §8.
2. **Use trustworthy access:** recover from an independently trusted device/
   session through the applicable route below. Secure associated email,
   authentication and recovery settings. Do not use the suspect account's
   uncorroborated “fixed” message as resolution evidence.
3. **Remove persistence:** review users/groups/apps, permissions, sessions,
   tokens/keys, automation and account changes. Revoke/rotate affected material
   through its owner and verify the actual result. Removing one password or
   grant does not prove every remaining session/application lost access.
4. **Contain publication separately:** inspect active/scheduled submissions,
   flights, availability and contact changes; apply §4 and ask Store support for
   incident-specific help where necessary. Record what the provider actually
   confirms. No publisher-controlled Microsoft-certificate revocation or
   universal remote uninstall is established.
5. **Re-establish release custody:** inventory commits, tags, releases, workflow
   revisions, dependencies, generated data, caches and artifacts in the uncertain
   interval. Reconcile against independently retained identities, remove the
   cause and rebuild/requalify through trusted inputs. A clean repository is
   not proof that a previously produced or executed binary was clean.
6. **Assess installed systems:** determine whether suspect privileged code ran.
   Its own logs/version/self-check cannot establish host integrity or journal
   truth. Use qualified independently trusted administrative/provider assessment
   and recovery; broader host remediation may be necessary. Preserve the
   distinction between host integrity, clock restoration and spent authority.
7. **Correct and communicate:** qualify the trusted recovery/replacement route,
   obtain David's exact approval and issue an authorized warning through a
   recovered/independently verified channel. Track unreachable populations and
   absent recovery access as residual exposure.

### 7.4 Provider-specific owner actions

| Authority/problem | Prepared recovery route and limitation |
| --- | --- |
| Personal Microsoft account | Use the [compromised-account guide](https://support.microsoft.com/en-us/account-billing/how-to-recover-a-hacked-or-compromised-microsoft-account-24ca907d-bcdf-a44b-4656-47f0cd89c245) and its Sign-in Helper. The [recovery-form guidance](https://support.microsoft.com/en-us/account-billing/help-with-the-microsoft-account-recovery-form-b19c02d1-a782-dee6-93c3-dc8113b20c42) describes limits when all two-step-verification methods are lost; support recovery is not guaranteed. [MS-17, MS-18] |
| Entra-backed identity | An appropriately authorized trusted administrator follows [revoke user access](https://learn.microsoft.com/entra/identity/users/users-revoke-access). Disable/block access and revoke sessions as applicable, then verify service-specific access. Existing access/application sessions may outlast identity changes; Entra cannot directly revoke every application cookie. [MS-19] |
| Partner Center human/application permissions | Review actual account users and [Entra applications](https://learn.microsoft.com/windows/apps/publish/partner-center/manage-azure-ad-applications-in-partner-center), keys and publishing permissions. Removing Partner Center app access does not delete the tenant application; inspect both scopes. [MS-16, W11] |
| Store submission/account help | Use [Windows developer support](https://aka.ms/windowsdevelopersupport), linked by [Raise a support ticket](https://learn.microsoft.com/windows/apps/publish/contact-us); the [developer support page](https://learn.microsoft.com/windows/apps/develop/support#partner-center-and-store-support) separates MSA/Entra entry points. [MS-20, MS-21] |
| Cannot sign into Partner Center | Follow [sign-in troubleshooting](https://learn.microsoft.com/partner-center/account-settings/unable-to-sign-in), including its no-sign-in support-ticket route. Availability of a route establishes no case outcome or recovery time. [MS-15] |
| GitHub account/repository | Use [unauthorized-access guidance](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/preventing-unauthorized-access); recover inaccessible attacker-changed sign-in through GitHub Support. Review email/factors, collaborators, apps, tokens, SSH/deploy keys, webhooks and source/release/workflow changes. Preserve the security log promptly; the documented personal-account window is 90 days. [W14, W15] |

These are preparation links, not evidence that a NevroTick account, alternate
identity or recovery contact has been configured or tested. Do not substitute
a CSP/Azure-subscription runbook for the actual Store developer account.

## 8. Evidence, communication and closure

Keep incident evidence bounded and private where necessary. Prefer exact package/
source IDs, OS/configuration row, event codes, component and policy/clock/owner
generations, operation correlations and journal outcome classes. Preserve
provenance, acquisition method and uncertainty. Use monotonic/independent ordering
when wall time is unreliable; distinguish reports from independently observed facts.

Follow [diagnostics §12.3](windows-time-architecture.md#123-logging-and-support)
and SECURITY.md's transfer guidance. Ordinary logs/exports must exclude secrets
and unnecessary personal/private endpoint/path data. Raw dumps, EVTX, ETL or
packet captures are not automatically redacted. Do not infer absence of effects
from incomplete or suspect logs; collecting evidence cannot grant clock authority.
No default telemetry or automatic upload is introduced. [W08]

An authorized incident update states: affected and corrected exact identities;
facts/uncertainty and reachable impact; the qualified local action or explicit
recovery gap; corrective acquisition path and prerequisites; offline/flight/
applicability limits; reporting contact; and the next update owner. A
[repository security advisory](https://docs.github.com/en/code-security/how-tos/report-and-fix-vulnerabilities/fix-reported-vulnerabilities/create-repository-advisory)
can serve a genuine vulnerability; ordinary regressions need not become CVEs.
Public messages and private investigation records are separate. [W10]

Record closure per outcome, with evidence:

| Outcome | Required conclusion |
| --- | --- |
| Distribution contained | Exact channel action/readback, assessed fallback and remaining acquisition/update audiences. |
| Publishing authority recovered | Applicable unauthorized access removed, trusted custody re-established and remaining account uncertainty recorded. |
| Correction qualified and available | Exact delivered artifact, required checks, approval and actual availability for the supported affected matrix. |
| Installed case recovered | Current owner, mandatory state, actual mode/configuration and restoration outcome independently observed within the case's scope. |
| Residual population and prevention | Offline/unreachable/unsupported/host-compromise cases, owner and next action; causal fix, missed gate and prevention work in the existing task. |

Closing a bounded response must not label unobserved installations remediated.
If the owner closes coordination with residual exposure, record the decision
and continuing case owners explicitly. A missing required recovery path remains
a release blocker regardless of administrative incident status. [W08, W09]

## 9. Qualification matrix and remaining decisions

All product/account/Store cases below are **planned, not executed by this
documentation work**. Preserve existing CHECK owners and candidate-specific
receipts; this is not a new acceptance catalog.

Apply the [two-family platform matrix](../tasks/windows11-integration.md#432-two-family-windows-qualification-matrix--5-november-2026)
to every claimed OS profile. Local package mechanics, product/CPU admission and
actual delivered-byte qualification remain distinct; Server delivery applicability
is unresolved. No client result silently qualifies Server, and OPEN-2 stays open.

| Distinguishing case and useful control | Required observation | Existing owners/environment |
| --- | --- | --- |
| Supported-release transition; latest quarantined | Previous qualified route persists while its replacement is unavailable, unless unsafe; an unassessed predecessor is never silently recommended. A no-qualified-stable state is expressible. | CHECK-24; David/release; document/tabletop, then actual channel evidence. |
| Private initial audience, later flight and percentage rollout | Membership/rank/applicability and update-percentage controls are distinguished; intended tester obtains the correction, outsider/private and wrong-flight cases match the declared scope. | W11-W10, CHECK-18/23/24; David-controlled Store candidate. |
| Halt/unsafe fallback/cloned submission | Record actual fallback and retained installed version; cloned package inventory contains only intended candidates. Healthy corrective acquisition succeeds. | CHECK-18/23/24; controlled Store evidence. |
| Corrective greater version, skipped releases, applicability/headroom | Affected public/flight predecessors reach the intended correction; stranded/unsupported cases are explicit, not forced downgrades. | OPEN-11/14, CHECK-18/23/26; package and delivered/native evidence. |
| Defective/new schema and mixed components | Required states migrate or safely refuse without losing authority; valid compatible update works. If refusal leaves prior takeover, recovery remains actually available. | CHECK-13/15/23/25; state/lifecycle owners, host contracts then disposable Windows. |
| Reservation/effect/outcome/reply interruptions | Reached failures around writes/flushes/effect/acknowledgement preserve uncertainty and consumed authority; reconciled healthy operation works. | CHECK-10/12/13/15; broker/storage, authorized native effect lane. |
| Successor, administrator/GPO change and queued SCM restart | No stale reset, W32Time handback beneath successor or maintenance bypass; authorized current-owner restoration succeeds. | W11-W05, CHECK-5/12/13/23/27; disposable Windows with external restoration owner. |
| Repair/Reset/independent removal before UI launch; stopped service/last user | Recovery remains invocable until its durable outcome; no orphan takeover, erased sole journal or invisible privileged component. Ordinary permitted removal succeeds. | W11-W05/W11-W10, OPEN-12/14, CHECK-5/12/13/15/23/27; supported bridge still missing. |
| Mandatory corruption, optional cache and wrong intact floor | Mandatory failure blocks unsafe effects; optional reacquisition remains useful; authorized narrow floor supersession preserves history/authority and unauthorized reset fails. | CHECK-9/12/15/25, OPEN-6; trust/storage owners, required host/native layers. |
| Whole-environment rollback | Report demonstrated within-model controls and the unqualified privileged-snapshot residual separately; no file-authentication claim becomes universal anti-rollback. | CHECK-11/12/15; security/platform, admitted VM scenario. |
| Wrong-date/offline/stale or absent trust cache | Actual declared verifier/retrieval/custody and recovery behavior, with intended allowed path and hostile helper refusal; no trust-check bypass. | CHECK-9/18/23/27; signing §5.1, actual delivered artifact/native Windows. |
| Wrong, tampered, unrelated Store, copied-out or unapproved old code | Still-trusted admission boundary denies product authority to the inadmissible artifact; intended package/helper works. | CHECK-1/2/16/18/23; release/platform/security, exact installed artifact. |
| Authentic malicious same-product publication | Tabletop/inert fixture distinguishes signature success from approval. Document the absent independent gate, trustworthy recovery route or unresolved exposure; do not record automatic rejection as passed. | THR-15, OPEN-11, CHECK-18/20/24; release/security, then separately admitted account/native evidence. |
| Account/application access recovery and renewal | Real roles/routes are available; authorized test access removal and remaining session limits are observed. Publishing and installed-device recovery stay separate. | W11-W10, CHECK-18/20/24; private inventory/tabletop, separately authorized account exercise. |
| Corrective source/build/delivered custody | Substitution/unexpected inputs fail the actual release gate; reviewed valid candidate, full inventory/generated data and licenses reconcile. | CHECK-18/19/20/24/26; build/release and final Store evidence. |
| Support export and incident guidance | Synthetic secret/error cases yield useful bounded redacted output; incomplete observations remain explicit; guidance names only a qualified local action. | CHECK-14/17/24/27; diagnostics/release, content review then actual export/native paths. |

Before the first stable commitment, David must resolve §2's proposed maintenance
model and any exception, exact supported profiles/transitions, actual monitoring/
response owner, chosen notification/EOL expectations and rollout observation
criteria. Complete the private recovery inventory and rehearsals. Use the
[existing decision-card procedure](nevrotick-owner-decisions.md#decision-card)
when a dependent choice is concrete; no numeric commitment follows from research.

W11-W10 retains P6/P7 release/account/delivered-artifact work; W11-W05 retains
P0/P4/P6 provisioning, maintenance and restoration evidence. OPEN-11/12/14 and
all affected CHECK gates remain open. Independent source preparation continues
within current priorities and holds; this policy does not claim a native pass.

## 10. Reasoning, alternatives and sources

### 10.1 Transferable lessons and rejected shortcuts

Pinned PowerToys release practice supports explicit hotfix source/artifact custody;
its updater topology and reduced hotfix testing do not transfer automatically.
Node.js demonstrates that branch/backport ownership and support phases must be
explicit, not that NevroTick can promise Node's resources or lifetime. Firefox
illustrates old-reader/new-state hazards; its major-version guard and overrides
are unsuitable as the privileged authority-ledger rule. The tj-actions maintainer
incident statement separates upstream cleanup from downstream exposure; it is
a mutable first-party account, not independent forensic proof. [GH-1–GH-7]

| Alternative | Benefit and disposition |
| --- | --- |
| Parallel LTS branches | Can help pinned users; requires a real capacity/platform commitment. Retain only as an owner-approved exception. |
| Highest number or valid signature determines recommendation | Simple, but admits withdrawn/compromised/unqualified material. Use an explicitly approved status record; do not imply an independent on-device enforcement gate exists. |
| Listing reversion or service stop completes recovery | Fast containment step; installed code, programmed rate and migrated state remain separate obligations. |
| Restore all old data / always Reset or uninstall | Superficially simple; can erase recovery or revive consumed authority. Use the qualified state/ownership transition. |
| New TUF/updater, mandatory-flag kill switch or MSI escape path | Could change capabilities, but would change unselected architecture and does not supply native recovery. Excluded from this scope. |
| Automatic two-human approval or fixed emergency SLA | Could reduce some risk with real staffing; no such delegation/capacity exists by assumption. Preserve David's authority and explicit evidence. |

### 10.2 Primary source register

Retrieved 4 November 2026. These source labels are local references, not new
CHECK/REQ IDs. Vendor documents establish platform behavior; upstream practices
and incident guidance inform the proposed procedure, not product qualification.
Mutable provider behavior must be rechecked before the corresponding real action.

| ID | Primary source and scope |
| --- | --- |
| MS-01 | [Gradual package rollout](https://learn.microsoft.com/windows/apps/publish/gradual-package-rollout): existing-user percentage, halt/finalize and installed-version limit. |
| MS-02 | [Remove an app](https://learn.microsoft.com/windows/apps/publish/publish-your-app/remove-an-app-and-add-on): availability operation and continued installed use. |
| MS-03 | [MSIX visibility](https://learn.microsoft.com/windows/apps/publish/publish-your-app/msix/visibility-options): initial audience, personal tester identity and acquisition exceptions. |
| MS-04 | [Package flights](https://learn.microsoft.com/windows/apps/publish/package-flights): membership/rank, certification and recovery reach. |
| MS-05 | [MSIX package requirements](https://learn.microsoft.com/windows/apps/publish/publish-your-app/msix/app-package-requirements): Store signing, version/applicability and corrective package. |
| MS-06 | [Reset or Repair MSIX apps](https://learn.microsoft.com/windows/msix/desktop/managing-your-msix-reset-and-repair): Repair retains app data; Reset deletes app data and reinstalls. Separate mandatory machine-state fate still needs product qualification. |
| MS-09 | [Store package updates](https://learn.microsoft.com/windows/apps/package-and-deploy/package-updates-from-store): mandatory metadata requires application enforcement. |
| MS-10 | [App Installer update settings](https://learn.microsoft.com/windows/msix/app-installer/update-settings): distinct optional downgrade route, not selected here. |
| MS-11 | [Halt rollout API](https://learn.microsoft.com/windows/uwp/monetize/halt-the-package-rollout-for-an-app-submission): in-progress precondition, fallback and cloned inventory. |
| MS-12 | [MSIX submission options](https://learn.microsoft.com/windows/apps/publish/publish-your-app/msix/manage-submission-options): publication hold. |
| MS-13 | [MSIX certification process](https://learn.microsoft.com/windows/apps/publish/publish-your-app/msix/app-certification-process): signing/publishing stages and cancellation limit. |
| MS-15 | [Partner Center sign-in problems](https://learn.microsoft.com/partner-center/account-settings/unable-to-sign-in): identity-specific and no-sign-in support routes. |
| MS-16 | [Partner Center Entra apps](https://learn.microsoft.com/windows/apps/publish/partner-center/manage-azure-ad-applications-in-partner-center): application permissions/key inventory and removal scope. |
| MS-17 | [Compromised Microsoft account](https://support.microsoft.com/en-us/account-billing/how-to-recover-a-hacked-or-compromised-microsoft-account-24ca907d-bcdf-a44b-4656-47f0cd89c245): personal-account recovery route. |
| MS-18 | [Microsoft account recovery form](https://support.microsoft.com/en-us/account-billing/help-with-the-microsoft-account-recovery-form-b19c02d1-a782-dee6-93c3-dc8113b20c42): recovery limits. |
| MS-19 | [Revoke Entra user access](https://learn.microsoft.com/entra/identity/users/users-revoke-access): authorized steps and residual token/session lifetime. |
| MS-20 | [Windows developer support](https://learn.microsoft.com/windows/apps/develop/support#partner-center-and-store-support): MSA/Entra support entry points. |
| MS-21 | [Raise a support ticket](https://learn.microsoft.com/windows/apps/publish/contact-us): Store developer account/submission support. |
| W06 | [GitHub immutable releases](https://docs.github.com/en/code-security/concepts/supply-chain-security/immutable-releases): conditional asset/tag versus status-edit behavior. |
| W07 | [GitHub artifact attestations](https://docs.github.com/en/actions/concepts/security/artifact-attestations): provenance, verification and absence of a software-safety guarantee. |
| W08 | [NIST SP 800-61r3](https://doi.org/10.6028/NIST.SP.800-61r3): triage, protected evidence, restoration integrity and closure; voluntary guidance, not an SLA/conformance claim. |
| W09 | [NIST SP 800-218 v1.1](https://csrc.nist.gov/pubs/sp/800/218/final): release/provenance archives and vulnerability-response responsibilities. |
| W10 | [GitHub repository advisory](https://docs.github.com/en/code-security/how-tos/report-and-fix-vulnerabilities/fix-reported-vulnerabilities/create-repository-advisory): affected/fixed versions and separate draft/publication. |
| W11 | [Windows app account roles](https://learn.microsoft.com/windows/apps/publish/partner-center/overview-of-roles-and-permissions-for-account-users): publishing authority beyond one password. |
| W14 | [GitHub unauthorized access](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/preventing-unauthorized-access): credential/persistence review and account recovery. |
| W15 | [GitHub security log](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/reviewing-your-security-log): documented personal log window. |
| W16 | [Microsoft account recovery code](https://support.microsoft.com/en-us/accounts-billing/manage/how-to-get-a-microsoft-account-recovery-code): separated protected recovery material. |
| GH-1 | [PowerToys release process, 0b1dd814878f0dfbb07787d9c3e52a2897b6f769](https://github.com/microsoft/PowerToys/blob/0b1dd814878f0dfbb07787d9c3e52a2897b6f769/doc/devdocs/processes/release-process.md): release custody and essential hotfixes. |
| GH-2 | [PowerToys update process, same pin](https://github.com/microsoft/PowerToys/blob/0b1dd814878f0dfbb07787d9c3e52a2897b6f769/doc/devdocs/processes/update-process.md): non-transferable updater topology. |
| GH-3 | [PowerToys v0.84.1](https://github.com/microsoft/PowerToys/releases/tag/v0.84.1): historical corrective release; mutable release record, not a binary audit. |
| GH-4 | [Node.js Release policy, 6065a837c0c151577f449c666fec019069ffba77](https://github.com/nodejs/Release/blob/6065a837c0c151577f449c666fec019069ffba77/README.md): explicit support/backport roles; no borrowed duration. |
| GH-5 | [Firefox profile design, f0d8507effaed215a7ff810b76c9bc5d3f11a6ab](https://github.com/mozilla-firefox/firefox/blob/f0d8507effaed215a7ff810b76c9bc5d3f11a6ab/toolkit/profile/docs/changes.md): schema compatibility hazards. |
| GH-6 | [Firefox startup, same pin](https://github.com/mozilla-firefox/firefox/blob/f0d8507effaed215a7ff810b76c9bc5d3f11a6ab/toolkit/xre/nsAppRunner.cpp): major-version downgrade guard and overrides, not a privileged-ledger design. |
| GH-7 | [tj-actions maintainer incident statement](https://github.com/tj-actions/changed-files/issues/2464#issuecomment-2727015352): March 2025 upstream response and distinct downstream actions; mutable primary statement. |

