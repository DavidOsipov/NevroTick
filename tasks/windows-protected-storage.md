# Protected optional storage: agent task and evidence record

**Date:** 3 November 2026 (Asia/Tbilisi).  
**Status:** Documentation integration and the bounded ST-W01 documentary card are reviewed; the engineering construction freeze, implementation and native qualification remain open.
**Inspected project baseline:** [`6b28711f5510e3550039d5fcf8bbcf3ccba791a5`](https://github.com/DavidOsipov/NevroTick/tree/6b28711f5510e3550039d5fcf8bbcf3ccba791a5), tree `fbc8b31f827d55f2c2e85426268979b0d1f827eb`. This task was prepared from an API-derived partial snapshot; it establishes no local checkout or dirty/staged state.  
**Owner request:** integrate the completed deeper GitHub research into repository documentation, the plan and tasks for the development agent.

## Current delivery supersession — 4 November 2026

Amended [OD-05](../docs/nevrotick-owner-decisions.md#7-od-05--signing-and-release-ownership)
selects one complete hardened MSIX. Use the
[current package adoption](windows11-integration.md#417-primary-coordinator-adoption-of-the-selected-msix-profile--3-november-2026)
and W11-W05/W11-W10 for service provisioning, account/SID/ACL ownership,
servicing and removal. MSI-specific cases in the dated §6.5/§6.6 receipts are
retained source/history for a later owner-selected alternative; they are not
the current package contract or an automatic fallback. ST-W03 applies the same
directory/object-authority invariants to the selected MSIX lifecycle, with its
actual supported mechanisms still requiring qualification.

ST-W01–ST-W07 retain their recorded source-only/phase-gated dispositions. The
storage pause, separate mandatory-state owner and scoped Windows/fuzz holds
remain. The [task index](README.md) supplies navigation without reopening work.

**Primary coordinator checkpoint, 7 November 2026:** after the latest pull and
the [bounded reference-bridge exit](engine-transport-pilot.md#common-run-reference-bridge--final-qualification-and-next-action),
the coordinator rechecked §6.5–§6.7 and the imported ST-W queue. ST-W01 remains
ready source-only and unexecuted; ST-W02 waits for its frozen contracts and
authorized slice. ST-W03–ST-W07 retain their named filesystem, CNG, mitigation,
IPC-completion and privacy owners, phase triggers and next actions in §6.6,
subject to the MSIX supersession above. The next pure Core conditioning contract
introduces no storage adapter or experiment. No assignment or execution hold is
lifted by this checkpoint.

## 1. Authority, scope and reconciliation

Read [AGENTS.md](../AGENTS.md), the [documentation guide](../docs/README.md),
[architecture §12.1.2](../docs/windows-time-architecture.md#1212-protected-optional-storage-and-explicit-fallback),
the [protected-storage plan](../docs/plans/windows-protected-storage.md) and
[development §8.4](../docs/plans/development-plan.md#84-protected-optional-storage-design-and-qualification).
The plan's [ST-T01–ST-T18 mapping](../docs/plans/windows-protected-storage.md#7-focused-threat-scenarios-and-acceptance-mapping)
continues to own scenarios and their CHECK/DNS-C requirements. **ST-W labels below
are task-local assignments, not new requirements or additional release gates.**

The [first upstream inventory](../docs/plans/windows-protected-storage-upstream-lessons.md)
and [second inventory](../docs/plans/windows-protected-storage-upstream-lessons-2.md)
own pinned observations, primary sources, licence limits and adaptation caveats.
This record turns them into bounded work and owns subsequent execution receipts;
it does not duplicate their technical specification or authorize library adoption.

The [Microsoft Learn follow-up](../docs/plans/windows11-integration-follow-up.md)
adds concrete Windows API cases to these same assignments. Its later
[routing record](#65-microsoft-learn-follow-up-routing--3-november-2026)
preserves the original receipts and current execution holds; it creates no new
storage work-item or acceptance catalogue.

**Leap-policy integration, 4 November 2026:** [NV-LP-001](../docs/plans/leap-second-policy.md)
and [architecture §10.5](../docs/windows-time-architecture.md#105-leap-seconds-and-time-scales)
assign immutable compiled public leap data to the authenticated complete release
and pure domain owner. Apply the [plan's revised state separation](../docs/plans/windows-protected-storage.md#2-owners-data-classes-and-restoration)
when preparing ST-W01 or later recovery work. The [S1.4 task](s1.4-leap-policy.md)
owns history/coverage/effective-rule and guard implementation; no new ST-W/ST-T
item or storage-provider prerequisite is created. This is a documentation
integration, not execution of the source-only card, native tests or earlier
receipt refresh.

| Reconciliation with the predecessor | Disposition |
| --- | --- |
| Existing VBS/TPM/fresh optional state/no-cache direction, exact committed envelope, installation witness and finite slots | Retained; exact construction, default and provider/build/account/profile row remain open. No error-induced DPAPI/software/plaintext fallback. |
| Operation errors, file authority, CNG calls, durability and completion | Refined into the concrete assignments below, under existing ST-T owners. Much of the policy already exists; upstream examples supply implementation and test detail. |
| RedirectionGuard | Added as a research candidate in the existing mitigation matrix. Required/optional disposition, OS support, activation and compatibility are not selected. |
| Hardware epochs, attestation/PCR, chunking and database choice | Conditional reserves with explicit triggers; no current prerequisite or adoption. Authentic rollback, offline catalogue/witness replacement and full VM/vTPM rollback remain residuals. |
| Current product work and qualification | Current P1/S1.3 priority, held TLS/provider choices, P0–P7 order and UI-M0/D2 boundaries remain unchanged. No implementation, native, performance or conformance result follows from this document. |

The present request authorizes documentation, plan and task integration. The
owner's storage-experiment pause remains in effect. Future implementation uses
the applicable authorized slice and ready contracts; this task creates no new
blanket approval round and grants no native execution by implication. Native
storage work remains held until the pause is lifted and its existing gates are
satisfied. Do not acquire dependencies, create keys, change services/process
policy or scaffold product crates merely to close this documentation task.

Use [plan-code-change](../.agents/skills/planning/plan-code-change/SKILL.md) and
the [testing contract](../.agents/skills/testing/CONTRACT.md). Before implementing
a selected item, read its relevant contract/security/testing leaves through
[the skills index](../.agents/skills/README.md); use the existing configured
tooling rather than inventing test commands before source exists. In particular,
private contract construction, model-based testing, lifecycle/concurrency,
crypto-secret handling, Windows privileges and IPC have existing skill owners.
Only load the leaves needed by that item's changed guarantee.

## 2. Next source-only assignment

**5 November proxy-credential scope:** consume the
[conditional-purpose handoff](../docs/plans/windows-protected-storage.md#2-owners-data-classes-and-restoration)
when ST-W01 reaches authenticated proxy support.
[OD-11](../docs/nevrotick-owner-decisions.md#15-od-11--socks5-scope-and-deferral)
defers proxy support beyond first Windows delivery; only a later authenticated
SOCKS5 extension needs this purpose. The first future loopback/no-auth/local-DNS
subset does not, and this handoff is not a current ST-W01 or Direct prerequisite.
For that authenticated extension, reuse the existing engine storage manager;
freeze purpose/key/epoch isolation, bounded credential references,
authorized input/readers, replacement and missing-secret behavior before enabling
it. The NTS/DNS allowlists and continuation matrix do not already qualify this
purpose. Map malformed/old/wrong-purpose state, unavailable root, partial replace,
unattended restart, late credential readers and redacted exports to existing
ST-T03/05/07/10/14/17 and engine PATH-14/18, with healthy supported controls.
No new ST-W item, provider choice, secret file/database or native execution is
created; source-only preparation and the storage pause retain their scope.

### ST-W01 Prepare one construction and failure contract

**Owner:** storage lead, with bounded platform/security review.  
**Readiness:** the documentary assignment is complete at the [10 November card](#7-st-w01-documentary-card--10-november-2026); its engineering row is not execution-ready.
**Scope:** one proposed construction/provider/build/account/profile row, one operation-outcome table and one ownership/reconciliation map. Reuse the inventories; do not restart a general tool survey.

**10 November continuation:** the [bounded ST-W01 card](#7-st-w01-documentary-card--10-november-2026)
below supplies this assignment's proposed row, outcome/ownership map and precise
engineering blockers. Its review status controls this documentary child; the
construction freeze, implementation and storage experiments remain separate.

Read the plan's [construction](../docs/plans/windows-protected-storage.md#4-candidate-construction-commit-and-restart),
[platform admission](../docs/plans/windows-protected-storage.md#5-platform-admission-failure-classification-and-privacy)
and [staged gates](../docs/plans/windows-protected-storage.md#8-staged-execution-and-evidence-gates),
then second-inventory [directory authority](../docs/plans/windows-protected-storage-upstream-lessons-2.md#3-borrow-directory-authority-and-delayed-destruction-from-cap-std),
[operation outcomes](../docs/plans/windows-protected-storage-upstream-lessons-2.md#4-an-error-is-not-always-evidence-that-nothing-happened)
and [CNG admission](../docs/plans/windows-protected-storage-upstream-lessons-2.md#6-cng-admission-per-call-silence-real-lengths-and-explicit-parameters).

Prepare the following source-bound outputs in this record or the existing
canonical plan section that owns the decision:

First identify the separate data classes: compiled public leap history/rules,
optional secret snapshots and mandatory continuity. The public dataset's
revision/digest may be referenced by restored evidence, but its bytes do not
enter the optional-cache encryption/fallback construction. Distinguish a valid
bundle with unknown future coverage from a corrupt package or mandatory record;
neither key loss nor cache reset changes leap authority or refunds broker budgets.

1. Identify the candidate wrapping/signing operations, exact authenticated bytes,
   independent expected crypto result, actual-key evidence, per-call silence,
   property bounds and unsupported/refusal cases. Leave unproven native facts
   explicitly unresolved; source documentation does not qualify the row.
2. Separate an operation's definitely-not-applied, committed-with-receipt and
   reconciliation-required outcomes from the owner's ready, poisoned,
   recovery-required and closed states. Bind every retry to the same attributable
   operation and declared generation; identify the last decision before effects.
3. Map caller → admitted directory/key/record → preparation → durability barrier
   → catalogue publication → acknowledgement/retirement. Identify each enforcing
   constructor, mandatory guard or owner, and custody after cancellation/error.
   Suggested modules are ownership proposals until actual source exists.
4. Record a finite set of independent legal/refused traces: ordinary save, partial
   mutation, possible durable publication followed by an error, missing expected
   root, zero I/O progress, late completion and a second interruption during
   recovery. State allowed results and forbidden effects before writing tests.

**Exit:** a reviewer can trace the selected row and every listed failure to its
current owner, independent expectation and existing ST-T case. Routine exact
suite/provider/build evidence remains in the engineering freeze under plan §9;
it creates no new approval round. Material owner choices, defaults or changes to
the trust boundary use the existing NV-DEC process with concrete evidence/options.
Unresolved choices block only dependent work. Stop at that bounded card; do not
run a provider probe to make a source-only unknown disappear.
The relevant ST-T05/06/07/10/16/17/18 obligations remain unexecuted.

## 3. Later implementation and qualification work

These are planned assignments. Expand only the next authorized, ready slice into
exact files, source/feature identities, finite cases/time/resources, runner and
cleanup. Portable evidence precedes the native observations it cannot replace.
The work-item grouping does not prescribe a new crate/dependency graph.

### ST-W02 Enforce operation outcomes and model durability

**Owners/prerequisites:** storage and model-test owners; ST-W01's relevant outcome,
publication and recovery contracts frozen for the authorized implementation slice.
Use second-inventory [outcomes](../docs/plans/windows-protected-storage-upstream-lessons-2.md#4-an-error-is-not-always-evidence-that-nothing-happened)
and [durability/model monitor](../docs/plans/windows-protected-storage-upstream-lessons-2.md#5-model-durability-separately-then-monitor-the-operation-protocol).

**Work:** enforce unusable-owner and reconciliation transitions through private
APIs/current guards. Keep issued writes, visible bytes, durable bytes and product
acknowledgements separate in an independent scalar/event model. Distinguish
requested, successful and failed barriers. Derive exact digest/generation and
acknowledgement expectations independently of production serialization. Reuse
the existing sequence/seeded-defect workflow and declared device assumptions.

**Controls/exit:** valid commit/cleanup and authorized recovery succeed; premature
acknowledgement, wrong digest/generation, early retirement, lost charged mandatory
authority, forbidden repeat and post-forget publication fail at their actual
consumer. Exercise file growth, replacement, catalogue publication, cleanup,
recovery and a second recovery failure. Stop model execution at a crash cut or
exclude later synthetic returns from acknowledged obligations. Reconcile the
same operation after uncertain completion; do not infer rollback from an error.
An invalid event must be detected even when a sampled restart looks healthy.
Portable results must bind to actual enforcing source; native durability remains
open. **Mapping:** ST-T05/06/07/10/15/16/18 and their existing CHECK/DNS-C rows.

### ST-W03 Constrain directory authority before destructive effects

**Owners/prerequisites:** Windows filesystem and lifecycle owners; admitted store
root, finite relative namespace, owner/DACL and supported-filesystem contract.
Use [cap-std lessons](../docs/plans/windows-protected-storage-upstream-lessons-2.md#3-borrow-directory-authority-and-delayed-destruction-from-cap-std).

**Work:** expose an admitted directory capability; validate the actual opened
object before truncate/replace/delete. Make the allowed share modes and handle
release point explicit across stop, repair and update. Do not generalize a
partially path-reconstructing upstream wrapper into universal handle-relative
operation or relax sharing to hide an installer conflict.

**Controls/exit:** a legitimate bounded save succeeds; parent/leaf substitution,
unexpected reparse/hard-link/type/owner and prohibited absolute, UNC, device,
ADS or traversal names cannot change an unintended object. Verify an authorized
update after completed owner release and the declared result while it remains
active. Pure name checks are narrower than actual Windows path/token/replace
observations; skipped upstream tests are not passes. Native execution remains
held. **Mapping:** ST-T03/05/10/17.

**8 November failure-side refinement:** apply the same directory/object authority
before elevated startup errors, rollback/cleanup logs and crash records write
anything. The [OS-integration review](../docs/supporting-reviews/windows-os-integration-risk-research.md#3-cve-evidence-with-applicability-dispositions)
uses hMailServer CVE-2026-107578 as an external analogue, not a NevroTick finding.
Include a harmless principal-operation failure with a substituted service-writable
parent/leaf: an unrelated sentinel remains unchanged, while useful diagnostics
through an admitted sink and the legitimate operation still work. No logger
exception, path-only precheck or LocalSystem workaround satisfies the contract.
Reuse ST-T03/10/17 with W11-W05/W11-W08/W11-W10 and CHECK-13/15/17/23; optional
storage, mandatory state and collector-specific privacy retain separate owners.
The [canonical filesystem rule](../docs/windows-time-architecture.md#121-state-separation)
and [hardening §9.8](../docs/windows-time-development-hardening.md#98-diagnostics-and-shared-adapter-verification)
now carry this requirement and its paired check; the reusable skill routes are
recorded in [the integration map](../docs/supporting-reviews/windows-os-integration-risk-research.md#8-skill-and-document-integration--9-november-2026).

### ST-W04 Qualify each CNG call and its crypto result

**Owners/prerequisites:** CNG and crypto owners; the relevant ST-W01 suite,
key-policy, input/output sizes and final account/profile row. Use
[CNG lessons](../docs/plans/windows-protected-storage-upstream-lessons-2.md#6-cng-admission-per-call-silence-real-lengths-and-explicit-parameters).

**Work:** preserve native status and phase; use only documented operation-specific
silence flags. Bound both queried and actually returned property sizes, validate
version/flags and decode fields safely. Distinguish absent, unsupported and
failed queries. Freeze hash/KDF, padding, encoding, context and exact returned
lengths; pair roundtrips with independent known-answer/interoperability fixtures.

**Controls/exit:** healthy expected properties and expected crypto bytes succeed;
short/changed/oversized results, expected-None with access denial, actual-policy
mismatch, omitted mandatory parameter and changed context are distinguished or
rejected as declared. No-login private-key work and UI-required refusal without
weaker retry require native evidence on both actual roots. A shape-only test,
unavailable TPM or successful unrelated probe proves no roundtrip or actual-root
readiness. **Mapping:** ST-T01/02/05/12/13/16/18.

### ST-W05 Qualify mitigation startup and actual behavior

**Owners/prerequisites:** process-policy/platform and service/lifecycle owners;
the existing per-role mitigation manifest and authorized native stage. Read
[architecture §15.4](../docs/windows-time-architecture.md#154-per-process-mitigation-profiles-and-startup-admission),
[hardening §6.4](../docs/windows-time-development-hardening.md#64-per-process-mitigation-qualification)
and [behavioral lessons](../docs/plans/windows-protected-storage-upstream-lessons-2.md#7-mitigations-need-ordered-startup-and-behavioral-evidence).

**Work:** enforce startup transitions in release and keep requested policy,
successful query, actual denial and legitimate provider behavior separate.
Preserve status-bearing observations; unsupported/terminating/query failure is
not a trustworthy zero. Add [RedirectionGuard](../docs/plans/windows-protected-storage-upstream-lessons-2.md#2-redirectionguard-one-additional-windows-defense-worth-investigating)
as a candidate row with unresolved support/disposition, not an enabled default.

**Controls/exit:** distinguish ACG denial for executable/writable allocation,
existing code-page protection changes and anonymous/file-backed mappings, with
healthy controls. Qualify CNG/TLS/files/IPC together in cold startup and first
lazy use for the exact artifact, role and SCM token. If RedirectionGuard is
selected, distinguish audit/enforce, in-scope unprivileged junctions and
legacy/admin-created/out-of-scope controls; own file checks still apply. No
provider failure permits a weaker startup phase or required-policy exception.
Use owner-mediated ACL and handle-duplication challenges for the existing
second-service isolation requirement. **Mapping:** ST-T01/03/12/13/17.

### ST-W06 Keep IPC ownership and native completion explicit

**Owners/prerequisites:** IPC/platform and storage owners; frozen operation
lifetimes and existing identity/custody contracts. Read
[architecture §5.4](../docs/windows-time-architecture.md#54-native-ipc-implementation-and-qualification)
and [go-winio lessons](../docs/plans/windows-protected-storage-upstream-lessons-2.md#8-native-close-must-mean-what-its-callers-think-it-means).

**Work:** keep continuous endpoint ownership, stop-new-work, revoked publication
and actual-drained completion distinct. All consumers needing a completion
barrier await the actual shared completion. Preserve permits, key handles and
sensitive buffers through cancellation and recovery until real completion.
Do not infer caller identity or release-fatal, thread-bound impersonation
reversion from a generic library's defaults or RAII type.

**Controls/exit:** healthy connect/work/close succeeds; close racing connection,
concurrent closers, accept-after-close, delayed cancelled completion and
privacy-generation changes preserve declared publication and resource bounds.
Observe cleanup before harness teardown can hide unfinished work; distinguish
ordinary close acknowledgement from completion. Native token/pipe behavior
remains a separate held result. **Mapping:** ST-T03/04/10/15/16/17.

### ST-W07 Verify sensitive-memory lifetime and actual output privacy

**Owners/prerequisites:** secret-memory/diagnostics and privacy owners; existing
data-class/dump policy and the selected adapter's allocation/lifetime limits.
Use [memory and output lessons](../docs/plans/windows-protected-storage-upstream-lessons-2.md#10-smaller-and-conditional-lessons).

**Work:** preserve checked lock/protection outcomes, bounded allocations, wipe and
real native custody. Existing `secrecy`/`zeroize` policy remains the starting point.
Guarded arenas and WER exclusion are conditional implementation candidates;
neither authorizes new privilege, global paging changes or raw dump collection.
The [7 November service-error refinement](../docs/plans/windows11-integration-follow-up.md#effective-noninteractive-error-behavior)
adds no collector: coordinate with W11-W05/W11-W08 so effective unattended error
handling preserves useful diagnostics while WER invocation, UI, queue/upload and
LocalDumps/debugger confidentiality remain separate. No-UI proves no dump safety;
retain this task's reached-path prerequisites and native hold.
Use synthetic secrets and domain/cookie markers in actual permitted support,
log, temporary and backup outputs, with useful allowed fields beside exclusions.

**Controls/exit:** successful owned lifetime and allowed diagnostics work;
lock/protection failure, page-boundary faults where applicable, cancellation,
late completion and sentinel disclosure fail their declared claims. Distinguish
an executed fault witness from a skipped/sanitizer bypass and actual lock from
allocation success. Exercise forget barriers across retained readers and
inaccessible old blobs. Collector-specific exclusion proves no protection of all
copies or all dumps. **Mapping:** ST-T10/14/15/16; DNS-C11/13/14/15 as applicable.

## 4. Conditional reserves and re-entry triggers

| Reserve | Re-entry question and required disposition |
| --- | --- |
| Attestation, origin evidence or PCR binding | Revisit only for a named origin/custody or measured-boot property the admitted local evidence cannot establish. Keep exact public identity, trusted certification, fresh transcript binding, supported update/recovery and offline/wrong-date startup separate. [Source limits](../docs/plans/windows-protected-storage-upstream-lessons-2.md#91-preserve-the-exact-evidence-claim) do not select a remote service or close ST-T01/13. |
| TPM revocation epochs / PinWeaver recovery capacity | Revisit when an explicit stronger rollback boundary is required. Identify provisioning/reset principals, actual anchor, permitted epoch gap, disk/anchor order, attributable recovery capacity, finite retry/write budgets and VM restore boundary before any native action. [secboot/PinWeaver lessons](../docs/plans/windows-protected-storage-upstream-lessons-2.md#92-separate-snapshot-sequence-from-hardware-revocation-epoch) leave same-epoch replay, signed prepared future-policy admission and Windows/CNG integration unresolved; missing/unwritten/wrong/unavailable anchor is not fresh installation. Preserve ST-T06/07/10/17 and licence review. |
| Chunked snapshots or database backend | Revisit only after an evidenced size/workload need. [age/SQLCipher lessons](../docs/plans/windows-protected-storage-upstream-lessons-2.md#stream-completion-and-database-integration) require complete admitted-stream publication or qualified database integration; neither automatically supplies key custody, durability or rollback protection. Current bounded whole-record design remains. |

These reserves add no dependency to the next source-only task and close no
acknowledged residual risk. A counter proves ordering under its admitted model,
not UTC/NTS age or exact newest content; hardware and disk are not one atomic
transaction. No additional privileged service or selected database follows.

## 5. Integration into the existing phases

After the relevant owners are implemented and qualified, use the focused plan's
existing S2.3/S2.4, S3.3, future DNS, P5/D2 and package rows. NTS restoration must
still enforce current trust/source/age and complete association rules. Optional
cache loss cannot reset mandatory broker continuity or spent correction authority.
Ordinary DNS remains independent of optional learning persistence. D2's first
authenticated-connection notification and stale/Unknown presentation remain
deferred until their existing re-entry gate; no live storage or service UI enters
UI-M0. Final package identity, native account/profile and lifecycle results have
their own acceptance owners.

For release incidents, apply [the release policy](../docs/release-policy.md#6-mandatory-state-compatibility-and-owned-recovery)
to those existing package/state owners. The
[W11 operational handoff](windows11-integration.md#421-release-operations-integration--4-november-2026)
owns preparation and rehearsal routing; this storage task retains its paused
implementation/native work and its mandatory-versus-optional state boundary.

For restored time-model/evidence consumers, carry the admitted dataset/rule
revision, complete supported date/era hypotheses and compatible scale segment
through current mapping and guard checks. Unknown future coverage can remain a
qualifier on useful Tracking outside bounded guards; it is not an optional-store
failure or a reason to wait indefinitely for storage recovery. The ordinary
source/age/quorum and independent-bootstrap rules still apply. All-LI=0 unsmeared
sources remain useful; a demonstrably contradictory announcement is excluded
only through current observation admission, not by rewriting stored history or
globally disabling healthy peers. Preserve fresh re-entry and protected continuity
through updates/rollback. OS event execution cannot settle a broker outcome or
justify a synthetic extra second; those checks remain S1.4/OPEN-7 and the existing
CHECK-10/11/12/15/18/23 owners.

## 6. Evidence, review and continuation

| Evidence layer | Current state |
| --- | --- |
| First and second upstream source investigations | Recorded in the linked inventories at their pinned baselines. Code/test definitions and primary contracts were inspected; upstream tests were not executed. |
| Documentation and task integration | Nine-file integration completed against the baseline above; independent review and scoped documentation checks are recorded below. The containing Git commit records the published file identities; implementation and native claims remain separate. |
| ST-W01 construction/outcome/ownership card | Not yet executed; bounded source-only next action. |
| Portable implementation/model evidence | Not executed by this integration. No ST-W/ST-T/CHECK/DNS-C implementation pass. |
| Native Windows, installed-service and final-package evidence | Not executed; experiment pause and existing row-specific gates remain. |

For subsequent work, append the selected ST-W/ST-T scope, exact current source
including relevant dirty bytes, owner/writable files, admitted graph/runner and
environment, independent expected result, healthy/fault controls, limits, actual
outcomes and cleanup. Preserve failures, skips, unresolved choices and the next
permitted action; record no success from a zero-test run or unrelated probe.

The completed independent review below applies to its identified candidate; it
is not native qualification. Revalidate only the affected observations after
source, contract, provider, account, Windows build, mitigation, serialization or
package changes. Use this record for continuation rather than creating another
parallel storage plan.

### 6.1 Completed independent integration review

**Date:** 3 November 2026 (Asia/Tbilisi). **Reviewer:**
`storage_docs_integrated_review`, a separate non-authoring reviewer delegated
with `fork_turns: "none"`; no model override and no candidate edits. The reviewer
read the bounded packet and governing contracts, then independently checked the
frozen identities after both authors had stopped editing.

**Proposed change — reviewer comprehension.** This candidate brings the completed second source investigation into NevroTick's maintained documentation and gives a development agent one bounded route through it. The new supplement retains source observations and their limitations; architecture and hardening clarify the existing storage and adapter guarantees; the plan and task turn those guarantees into a source-only construction/outcome/ownership card followed by separately gated implementation and Windows qualification. The useful additions concern uncertain commit outcomes, directory authority, per-call CNG contracts, independent durability observations and actual native completion. RedirectionGuard and the larger attestation, counter, stream and database alternatives remain candidates or reserves.

**Current system — reviewer comprehension.** The supplied baseline describes a chrony-derived Windows time client with a network-facing engine, separate privileged broker, ordinary-user UI and narrow lifecycle helper. Existing fixture and bounded transport work does not implement the planned protected store or complete P1. Optional NTS and future DNS persistence prefer qualified VBS roots, may use separately qualified TPM roots for policy-authorized fresh optional state, and otherwise permit no persistent optional cache under the applicable policy. Mandatory policy, correction continuity and trusted floors have separate owners. The construction and native provider row remain unresolved; storage experiments are paused, and UI-M0 and D2 retain their present limits.


**Scoped verdict: ready** for this documentation/plan/task integration and for
ST-W01 as the next bounded source/documentation assignment. **Material findings:
none; corrective edits requested: none.** There are no unresolved review findings
or corrections. This verdict does not accept residual risk, lift the experiment
pause, authorize implementation or advance native/package/release gates.

The reviewer read both new documents in full, all seven existing-file diffs,
the relevant baseline architecture, hardening, plan, roadmap and agent guidance,
and the completed source report and its critical review. This was an integration
review using the completed research, not a new replay of every upstream source.

**Reviewed identities:** baseline commit/tree are recorded at the top of this
task. The frozen nine-file manifest had SHA-256
`af8197345d576b700d9ee8f4730e87e16c422398afe5d170273de91eff41ea1d`.
The following are the reviewed **pre-receipt** file identities:

| Candidate file | Reviewed SHA-256 |
| --- | --- |
| `AGENTS.md` | `0e8630afde2dd4a6ce1e376e9cc75149191237443336f266e478f5ec8caf25c5` |
| `ROADMAP.md` | `1e25db4303f8ea666a6a2b4c64bf18c4fe9ec368bc6b7c408bb46857d44bec7e` |
| `docs/README.md` | `48122355c1fb4a0837c4efce4760bdf44ef63b2c396256fef742c49e79f45edc` |
| `docs/plans/development-plan.md` | `89c16b3c4ee180675cdfec1e622f53cb9f913f896eb18a55e34d40515b899bb8` |
| `docs/plans/windows-protected-storage-upstream-lessons-2.md` | `6f712405a03b63867027ff400556cebbf4b2bfe0cff35cfc1fc19131fdcc7d63` |
| `docs/plans/windows-protected-storage.md` | `eeb77a06875933d6467bcfd5619ae96a6b2c9b4bc8074992d21d46275f2413b2` |
| `docs/windows-time-architecture.md` | `bca9ac958839464244babfb6e0cc32d1f094591f3e041b7fc4aaa5a4e7f78ead` |
| `docs/windows-time-development-hardening.md` | `37e7f3c66c4b30200b291a51419f6e6af5f77aab0c141f50906197d19764fd05` |
| `tasks/windows-protected-storage.md` | `3277a3d0515213544b9d64e0ad4171699505433febdfdc8954769b67423971b5` |

The reviewer independently matched all nine SHA-256 values, Git blob identities
and byte lengths and all eleven original source-manifest blob identities.
Focused preservation checks confirmed that the 18 ST-T scenario rows,
architecture DEC/BND/THR/CHECK definitions, both six-pair persistence-policy
tables, UI-M0/D1–D3 section and current development next-task section were
unchanged. ST-W01–ST-W07 each have one work-item heading and remain separate from
acceptance IDs. The substantive research conclusions were faithfully retained.

The reviewer independently checked 71 changed/new local links and anchors in
the supplied files with no errors. One locally absent skills-index target was
resolved by the coordinator against the complete pinned repository tree. An
exploratory ST-T count initially included an unchanged reconciliation row; the
corrected scenario-definition selector found 18 unchanged rows. This was a
check-selection correction, not a repaired document or behavioral failure.

This factual receipt and the task's status/evidence summary were added **after**
the review. The other eight reviewed files were retained byte-for-byte; the
review is not represented as examining these later receipt bytes. The containing
commit's tree identifies the final published versions.

### 6.2 Documentation validation receipt

The coordinator checked changed Markdown lines and both new documents in full.
The final checks resolved **72 distinct local links/anchors**, verified **48
distinct pinned GitHub source-file links** have immutable revision identities,
and resolved the second supplement's **65 reference definitions**. Local target
existence was checked against the complete 1,247-entry, non-truncated baseline
tree plus the two new documents; anchor checks used the retrieved document
bytes. No missing target/anchor, duplicate or unresolved reference, newly
unpinned GitHub source-file link, malformed trailing whitespace, replacement
character, missing final newline or unexpected changed file was found.

These are scoped documentation checks, not a whole-repository link audit or a
fresh network check of every external URL. There was no root
`docs/SHA256SUMS.txt` in the inspected tree, and no bundle-integrity claim is
made. The task was prepared in an API-derived partial snapshot; no local Git
index, staged tree or clean-worktree claim applies.

No product build, Rust test, solver, Windows experiment, installed-service test
or performance measurement was run for this documentation integration. No
library/provider was installed, and no key, service or mitigation was changed.
The next storage-development action remains ST-W01; current product-work
priority and the native-experiment pause remain in force.

### 6.3 Local checkout adoption — 3 November 2026

David requested the remote changes be pulled and adopted by the primary
development agent. Commit `a6c2401` is now the local `main` baseline. The pull
preserved 113 staged paths and the existing unstaged Core-task update. Two
overlapping guide/plan files were reconciled additively: the local Windows stop,
Core checkpoints and detailed storage gates remain beside the imported research
and ST-W assignments. Earlier API-snapshot review receipts above retain their
original candidates; they do not qualify this combined checkout.

The [startup guide](../AGENTS.md#start-each-task) now requires reconciliation after
a pull/documentation update, and the
[imported hardening work queue](../docs/plans/development-plan.md#imported-hardening-work-queue)
routes ST-W01–ST-W07 to their phase owners and prerequisites. At its next
checkpoint, the primary coordinator records applicable scope, assigned owner,
ready/held/deferred disposition and next action here. This is a discoverable
handoff; no acknowledgement from a separately running agent is claimed.

ST-W01 is ready for source-only preparation and remains unexecuted. ST-W02 waits
for the relevant frozen contracts and authorized implementation slice.
ST-W03–ST-W07 must inform their related Windows adapter work before effects are
implemented, including shared filesystem, CNG, mitigation, IPC-completion and
sensitive-memory/output paths. Current P1 work continues from its latest task
checkpoint; native Windows execution, storage experiments and the separately
flagged fuzz runner retain their existing holds. No default, provider, dependency,
hardware counter, attestation or extra privilege is adopted by this handoff.

New routing edits are kept unstaged; the pull's preserved staging is separate.
Documentation checks and the scoped combined-candidate review are recorded below.

### 6.4 Combined local-candidate review and checks

**3 November 2026:** `hardening_adoption_final_review`, delegated with
`fork_turns: "none"`, completed independent read-only review of the four routing
documents after the pull and merge. It inspected the incoming canonical storage/
hardening amendments, task prerequisites and current Core continuation. Its
verdict is **ready for this documentation-adoption stage**, with no material
findings. Reviewer comprehension, retained verbatim:

> The change makes the imported storage research discoverable through startup guidance, development-plan §7, the ST-W01–ST-W07 queue and storage-task §6.3. Each assignment has an owner and trigger; the text explicitly distinguishes routing from another agent’s acknowledgement or completed execution.

> The current system retains portable Core work, with S149 qualifying only its recorded two-owner Linux fixture candidate. The new projection pointer matches the actual next-work task. Windows execution, the flagged fuzz runner and storage experiments remain held. Construction/provider/default decisions, dependency admission, mandatory-state separation, UI-M0/D2 boundaries and conditional mechanisms remain intact.

The frozen four-file pre-receipt manifest SHA-256 is
`78c2fc46ec8414be62e0c122d22a3b658197736f58181d1c35cfb81e32fc97ab`.
It binds `AGENTS.md`, `docs/README.md`, `docs/plans/development-plan.md` and this
task before adding this subsection. All four file hashes matched the live
candidate during review; this receipt is a later factual addition.

Coordinator disposition: ST-W01 is ready and unexecuted; ST-W02 is planned after
its frozen contracts and authorized slice; ST-W03–ST-W07 are planned at their
mapped adapter boundaries, with actual Windows observations held. The primary
agent's acknowledgement remains pending its next checkpoint. No source-only
research or imported task is counted as implemented behavior.

Local checks resolve 724 link/anchor occurrences across the nine imported/current
documents, find no conflict markers, and pass whitespace checks for the new
routing edits. All 111 nonoverlapping staged entries retain their original
blob/mode/stage; the two overlapping guide/plan files preserve local intent in
the additive merge. The staged path set remains 113. Concurrent Core work was
preserved and may continue; no new product, analyzer or Windows test was run by
this pull/adoption task. The documentation checksum manifest remains absent.

### 6.5 Microsoft Learn follow-up routing — 3 November 2026

The owner requested the completed second Microsoft Learn investigation be added
to maintained documentation. This amendment is based on GitHub main
`45ff247d84e5e43bd13f54de1c84aa13ce2e6efd`, tree
`6fccf094d8eae9d8dbccfb406afed0e08e2b11f9`, in an API-derived partial snapshot;
it establishes no local index/dirty-state observation. The research originally
inspected `f163c193b0bb4167ec5c39e16b1db1b581509750`; earlier review and execution
receipts above retain their own candidates. This section records assignments,
not their execution or a separately running coordinator's acknowledgement.

| Existing assignment | Additional source-bound case at its existing trigger |
| --- | --- |
| ST-W01/02: outcome and recovery contract | For a selected `ReplaceFileW` adapter, carry errors 1175/1176/1177 and generic failure with/without a backup slot into the independent model. Reconcile actual objects/content/permissions before cleanup or retry; unsupported write-through supplies no durability. Preserve ST-T06/07/10/16/17. |
| ST-W03: directory and lifecycle ownership | Compare per-service directories/registry state only for a named optional/disposable class. A service-delete/recreate lifecycle can delete them; mandatory continuity stays independently owned. If declarative MSI ACLs are selected, qualify account/SID ordering, inherited parents and final runtime object owners/DACLs. Placement uses existing ST-T03/10/12/17 and CHECK-13/15/23; no new storage root is selected. |
| ST-W04 and the existing attestation reserve | VBS claim extensions are a conditional actual-key evidence lead, with prerelease/build, attestation-key/hash/padding, exact subject, anchoring, freshness and update constraints. No added key role, root, cloud dependency or stronger label follows from sample verification. Preserve ST-T01/05/13; this remains a reserve. |
| ST-W05 and the mitigation reserve | Side-channel-policy fields need a named role/threat, actual supported enforcement and healthy timing/resource measurements under CHECK-16/21. No field or required hardware profile is adopted. |
| ST-W07: actual output privacy | Add named WER/LocalDumps/debugger cases with synthetic sentinels, preserving ST-T14/CHECK-17. NOHEAP/exclusions and recovery callbacks do not prove all collectors safe. Coordinate shared ETW admission/session/retention cases with W11-W08 rather than duplicating its implementation owner. |

The [source supplement](../docs/plans/windows11-integration-follow-up.md#4-diagnostics-replacement-and-protected-storage)
and its [conditional section](../docs/plans/windows11-integration-follow-up.md#6-conditional-mechanisms)
own API facts, limitations and future healthy/refused cases. The integrated
documentation review/validation is recorded in the existing
[Windows integration task](windows11-integration.md), not credited from the older
research review. ST-W01 remains the bounded source-only next storage assignment;
other work retains its prerequisites. The current portable Core checkpoint, broad
Windows execution stop, separate fuzz no-retry hold, storage-experiment pause,
DoQ-only policy, one complete MSI, mandatory-state separation and UI-M0 deferrals
remain unchanged. No implementation, native test, provider or dependency admission
is claimed by these routing changes.

### 6.6 Primary development coordinator disposition — 3 November 2026

The primary P0–P7 coordinator has read the imported ST-W queue and §6.5 second-
pass refinements. The completed bounded S151 Linux checkpoint and next pure
fitted-tracking S152 journey use neither persistence nor a native storage/provider
adapter. Preserve that product priority; source-only readiness does not start a
storage experiment or create a new prerequisite for this Core slice.

| Assignment | Owner and disposition | Next dependent action |
| --- | --- | --- |
| ST-W01 | Storage lead with platform/security review; Ready source-only, unexecuted | Select one construction/provider/build/account/profile row, outcome table and ownership/reconciliation map; include ReplaceFileW 1175/1176/1177 and backup/no-backup outcomes before cleanup/retry. No provider probe is authorized. |
| ST-W02 | Storage/model QA; planned after relevant frozen ST-W01 contract and implementation admission | Separate owner usability from operation outcome; model issued/live/durable/acknowledged state and recovery failures. |
| ST-W03 | Windows filesystem/lifecycle; phase-gated, native held | Carry actual directory/object authority, service-managed lifetime and final MSI account/SID/ACL cases into P3/S3.3/P6 before destructive effects. |
| ST-W04 | CNG/crypto/platform; construction-gated, native held | Freeze each operation's status/parameters/lengths and no-UI/refusal behavior; VBS actual-key claims remain conditional and introduce no root or role. |
| ST-W05 | Process-policy/platform/service-lifecycle; phase-gated, native held | Preserve startup order/status and healthy/refused cases; side-channel fields and RedirectionGuard remain candidates until their named evidence/decision gate. |
| ST-W06 | IPC/platform/storage; completion-contract-gated, native held | Preserve stop-new-work, publication revocation, actual native completion and custody/reconciliation before related P3/storage integration. |
| ST-W07 | Secret-memory/diagnostics/privacy; reached-path-gated, native held | Add synthetic-sentinel WER/LocalDumps/debugger artifact cases and allocation/locking/wipe lifetime; coordinate ETW/session/retention with W11-W08. |

ST-W03–07 apply at their named shared boundaries even before optional caching.
Their existing ST-T/CHECK/DNS-C owners and acceptance identities are preserved.
The mandatory-state owner stays separate; DoQ-only and one complete MSI policies,
UI-M0 deferrals, Windows stop, separate fuzz no-retry hold and storage pause are
unchanged. No source card, provider/build/account/profile row or conditional
reserve has been executed or selected by this acknowledgement.

### 6.7 Windows binding coverage and reuse — 5 November 2026

**Scope/status:** source finding integrated against NevroTick
`e82f3f6b352fde88beceece4d3084862d8778338`; provider construction, dependency
admission and native experiments remain unexecuted/held. Read
[architecture §18.6.2/S135](../docs/windows-time-architecture.md#1862-windows-component-reuse-and-binding-coverage)
and [storage plan §5](../docs/plans/windows-protected-storage.md#5-platform-admission-failure-classification-and-privacy).
The inspected `windows` 0.62.2 projection lacks REQUIRE/PREFER VBS flags although
the reviewed new upstream ncrypt projection has them. A function's availability
or per-boot constant does not establish the missing flags or actual VBS support.

**ST-W01 next source-only action, storage/dependency/platform owners:** before
the affected construction row is frozen, enumerate required NCrypt functions,
flags, structures and status semantics; record selected published version,
checksum/source, features/target and SDK/metadata provenance. Resolve the gap
through a compatible binding or narrowly pinned generated surface. Record
coverage and unsupported cases without magic constants, mixed projection types
or a provider probe. Check package/source parity before executing build inputs.

**Independent exit:** the chosen source exposes every symbol/signature needed
by the selected row and preserves required flags/error distinctions; the old
projection and a missing-symbol/changed-generation case are discriminating
controls. Subsequent admitted target compilation proves only build compatibility.
Keep unknown provider/build/account/hardware behavior explicit. This is an
existing CHECK-26 engineering freeze, not another approval or global release gate.

**ST-W04 follow-on, CNG/crypto/native QA:** only after its construction and native
entry conditions, qualify create/open/use/teardown on actual roots with healthy
VBS and supported unavailable/denied/unsupported neighbors. REQUIRE must not
become PREFER/software fallback; document the prerelease/build qualification and
actual mode. Existing ST-T01/02/05/12/13/16/18 and CHECK-26/27 own the result.
Selecting generated bindings does not complete these native observations.

For a reached registry consumer, compare the focused wrapper only if it removes
concrete work while retaining rights/DACL, pre-allocation limits and recovery.
No authoritative-state relocation or cross-resource transaction is inferred.
Preserve the current complete-MSIX direction, separate mandatory-state owner,
VBS -> policy-authorised fresh TPM -> NoPersistentCache optional-state policy,
Core priority and Windows/fuzz/storage holds. This entry refines ST-W01/ST-W04;
it starts no new storage workstream or experiment.

## 7. ST-W01 documentary card — 10 November 2026

**Documentary card complete; engineering row not execution-ready.** This is the source/documentation child,
not a source-qualified executable construction or storage implementation. Root
owns integration; independent retained-context workers reviewed the CNG and
outcome sources separately. Local baseline is `a4c5c3a`, with existing staging
preserved. Current S2.2 native work is held for owner resume/headroom; completing
this already assigned card changes neither that priority nor the storage pause.
No dependencies, keys, probes, compiler jobs or Windows commands are introduced.
The existing plan §9 governs engineering details versus owner decisions.

### 7.1 One proposed row and its exact limits

`C1` is a task-local candidate label, not another acceptance identity. Purpose:
one optional NTS-association snapshot. Compiled public leap history/rules remain
release-authenticated domain data; an optional record may reference their
revision/digest but cannot replace them. Broker continuity, clock reservations,
ownership journals and privacy acknowledgments stay with their mandatory owners.
Cache/key loss changes neither leap authority nor charged correction allowances.
DNS learning and deferred authenticated proxy credentials are outside C1.

| Field | C1 proposed engineering row / dependent gate |
| --- | --- |
| Protection/provider | Two separate generated, nonexportable private roots in Microsoft Software KSP, each requiring VBS. No PREFER, per-boot key, overwrite, legacy-store or weaker-root fallback. This tests a VBS-only construction; it selects no shipped policy pair or default. |
| Wrapping | RSA-3072; OAEP with explicit SHA-256 and fixed label bytes `ASCII("NevroTick.NtsOptionalCache.DEK.C1")`, without NUL. Private usage is decryption-only; export-policy DWORD zero. Wrap exactly one fresh 32-byte DEK; admit exactly 384 ciphertext bytes. A decrypt sizing query may require up to384 bytes, but actual successful recovered DEK must be exactly32. |
| Signing | Separate ECDSA P-256 signing-only root, export-policy DWORD zero. Sign SHA-256 of the frozen domain-separated unsigned envelope; input hash32 bytes, signature64 bytes in fixed `r` followed by `s`, not DER. Signature randomness is permitted; do not require deterministic signature bytes. |
| Payload protection | One-shot AES-256-GCM, fresh DEK32 and nonce12 per independent preparation, tag16, explicit GCM mode and admitted AAD. One encryption use per DEK; interrupted/uncertain saves do not authorize regeneration or forbidden reuse. No plaintext is published before tag and semantic admission. |
| Target/build | Proposed Windows11 26H2 x64, selected v3/MSVC product profile. Exact OS/SDK/compiler/provider/build and required effective mitigations are **unfrozen**. Current Windows26100 laboratory receipts qualify none of this row. Server2025 is a separate required platform row, not silently covered. |
| Account/lifecycle | Intended own-process LocalService engine plus restricted service SID under architecture §4.1. Final SID, user/machine key scope, creation-time owner/DACL and MSIX provisioning are **unfrozen**. No broad LocalService grant, engine virtual-account substitution, broker identity or extra privilege is selected. |
| Actual-key evidence | REQUIRE requests, successful reopen and provider names are insufficient. Both actual roots need bound origin/public identity, export/use/access policy, cross-boot and unattended evidence under the final account/profile. Until then protection is Unknown and protected restore is not admitted. |

The primitive APIs are source-supported candidates, not a tested combination.
Use explicit [OAEP parameters](https://learn.microsoft.com/en-us/windows/win32/api/bcrypt/ns-bcrypt-bcrypt_oaep_padding_info)
and separate [encrypt](https://learn.microsoft.com/en-us/windows/win32/api/ncrypt/nf-ncrypt-ncryptencrypt)/
[decrypt](https://learn.microsoft.com/en-us/windows/win32/api/ncrypt/nf-ncrypt-ncryptdecrypt) byte counts.
[ECDSA's prehash contract](https://learn.microsoft.com/en-us/windows/win32/api/ncrypt/nf-ncrypt-ncryptsignhash)
and the [pinned .NET CNG consumer](https://github.com/dotnet/runtime/blob/e43d4b9672de5d11a31da7ef40d78564c1e9d166/src/libraries/Common/src/System/Security/Cryptography/ECDsaCng.SignVerify.cs)
support the proposed raw signature shape. Query GCM tag support and initialize
the [authenticated-mode structure](https://learn.microsoft.com/en-us/windows/win32/api/bcrypt/ns-bcrypt-bcrypt_authenticated_cipher_mode_info)
with explicit nonce/AAD/tag; no block padding, streaming release or implicit KDF.
Fresh material comes from the admitted OS RNG; RNG failure blocks preparation.

**Authenticated bytes — precise remaining construction blocker.** Let `H` be
the future canonical header containing domain/version/suite, installation/store/
purpose, both root identities, root/data/privacy/policy epochs, snapshot sequence,
wrapped DEK, nonce and bounded payload length. AEAD authenticates exact `H` as
AAD. The unsigned envelope contains exactly `H`, ciphertext and tag; its signature
also binds the sequence and complete ciphertext/tag, using a code-owned domain
separator. The committed-content digest is SHA-256 of the **entire final encoded
envelope including signature**, never a different decoded projection.
Verification identity comes from the independently admitted catalogue/provisioning
chain, not an envelope-supplied public key. Otherwise an attacker can choose a
DEK and fabricate a self-consistent public-key-wrapped record.

Canonical encoding/version/framing, signature-domain bytes, public-key identity
encoding and exact envelope/header/payload/queue limits are **not selected by
this card**. Storage/contract/security owners must freeze them before a codec,
crypto fixture or production consumer is implemented. Thus `H` and the complete
expected digest cannot yet be instantiated; this card is not that byte freeze.
Independent published AEAD vectors plus separately implemented wrapping/signature
fixtures must distinguish wrong hash, label, AAD, key, signature encoding and tag
beside valid neighbors. Roundtrips alone cannot close ST-T05/18. Persisted records
have no implemented old reader here; future version/rollback/migration admission
must be frozen before writing, rather than inferred from this proposal.

### 7.2 Binding evidence and per-call admission

The current lock contains `windows 0.62.2`, checksum
`527fadee13e0c05939a6a05d5bd6eec6cd2e3dbd648b9f8e447c6518133d8580`.
Source at release-associated commit `32c3144490c016fe496a0aed769bce60987a2e9d`
has Cryptography blob `76cecd33ec0463899d92eb31bfe319ecc17a95f5` and Foundation
blob `9a6014bf3852efe3939efbd57d7f712ae12faaac`.
Root independently fetched those exact blobs and the Microsoft header blob
`6844be98409e111e45eb28ee78c44b85373eb7f1` through the GitHub connector.

**Correction to the apparent missing-name blocker:** the
[existing projection](https://github.com/microsoft/windows-rs/blob/32c3144490c016fe496a0aed769bce60987a2e9d/crates/libs/windows/src/Windows/Win32/Security/Cryptography/mod.rs#L13153)
already exposes `NCRYPT_USE_VIRTUAL_ISOLATION_FLAG` and `NCRYPT_FLAGS(pub u32)`.
The [pinned header blob](https://api.github.com/repos/microsoft/win32metadata/git/blobs/6844be98409e111e45eb28ee78c44b85373eb7f1)
defines REQUIRE VBS as an alias of that existing constant. An explicit
same-projection typed alias is therefore a viable engineering option; missing
newer names alone do **not** require a dependency upgrade or copied numeric
constant. C1 does not use PREFER. SDK/package parity and target compilation still
need qualification. New upstream `Win32/ncrypt` has different module/signature
shapes and is not an admitted replacement.

The inspected Foundation projection lacks named VBS-unavailable status coverage.
Preserve raw HRESULTs and fail closed; exact unsupported/unavailable/denied/UI
classification remains a platform-owner gate, not permission to label unknown
errors as fallback. [Provider implementation flags](https://learn.microsoft.com/en-us/windows/win32/seccng/key-storage-property-identifiers#ncrypt_impl_type_property)
are expressly provider-wide. Header `Virtual Iso` or provider-root public blobs
do not establish a documented actual-key proof. The actual-key evidence method
needs its exact qualification protocol. The existing controlled-origin/public-
identity plus cross-boot, unattended and refused-neighbor route remains a valid
method to specify; no unidentified per-key property is made mandatory. Attestation
is still a conditional reserve, not a newly adopted online dependency or extra
root. Neither method has produced actual-key evidence for C1.

| Call family / future adapter owner | Mandatory admission and refusal contract |
| --- | --- |
| Open provider, create distinct roots | Fixed provider/algorithm/key namespace from admitted lifecycle state. New roots only for explicit first provisioning/rotation, never missing-key recovery by enumeration. REQUIRE VBS, no overwrite and supported descriptor/finalization ordering; validate actual key state before use. `NCryptCreatePersistedKey` does not document SILENT in its flag table: do not add unsupported flags or infer silence from that omission. |
| Set/get properties, finalize, reopen | Use documented per-call silence flags where supported; do not disable key validation at finalize. Reopen has no REQUIRE flag, so retain controlled origin/identity and actual-key evidence. DWORD properties require exact4 bytes. Variable data needs an engineering-frozen finite cap before allocation, valid version/layout and checked actual second-call length; absence/unsupported/denied remain distinct. A UI-requiring operation fails without retrying less strictly. |
| Encrypt/decrypt/sign/verify | Explicit OAEP hash/label and private-use SILENT; ECDSA uses NULL padding plus SILENT for signing. Check size-query and actual lengths independently; unexpected lengths, authentication failures and parameter refusal publish no usable state. Verification accepts only the independently anchored identity. |
| Retire/delete/free | Only attributable keys and files, current operation/generation guards, real completion and checked cleanup outcome. Retrying a possibly completed delete cannot claim a fresh root or release an occupied recovery slot. |

Independent controls must distinguish an omitted/changed flag or parameter beside
a valid supported neighbor. Removing REQUIRE or SILENT need not make an ordinary
healthy API call fail. Use an appropriate VBS-unavailable/UI-requiring environment
or an independent actual-boundary trace/contract oracle that detects omission;
do not require an incidental native refusal or credit an unchanged success.
Source spelling, provider-wide flags and successful compilation remain weaker
than actual-key, no-login, second-service-denial and effective-profile evidence
(ST-T01/05/12/13/16/18).

### 7.3 Outcomes, ownership and uncertain publication

The future single manager retains `(operation ID, expected catalogue generation,
root/data/privacy/policy identities, exact prepared-envelope digest)` through
completion and recovery. These are contract roles, not newly existing Rust types.
One active, one prepared and at most one attributable retired root set, with
equivalent snapshot slots, remain the inventory bound. Failed cleanup occupies
its slot. Current committed reads may remain usable under their guards; a new
transition must first reserve all capacity it could leave behind.

| Operation outcome | Owner/transaction consequence | Forbidden inference |
| --- | --- | --- |
| Definitely not applied | Established before effects or by validated rollback. Owner is Ready only after its invariant is restored; a poisoned transaction cannot commit. | Any returned error means rollback. |
| Committed with attributable receipt | Consume the transaction; only the exact current committed generation is eligible. Preserve cleanup debt separately and its occupied capacity. | Cleanup failure revives old state, refunds authority or permits another retired set. |
| Reconciliation required | Partial/possibly committed/unknown-completion work retains the same operation, objects, handles, buffers and charges; affected owner remains RecoveryRequired. | Retry a fresh operation, regenerate ciphertext/signature, reuse a generation or recreate missing roots. |

Ready, poisoned, RecoveryRequired and Closed belong to the **independent owner/
transaction-usability axis**, not the operation-outcome enumeration. A poisoned
transaction does not automatically poison every unaffected store operation.
Closed grants no further operational use; pending native completion/recovery
still has a custodian. Drop, reopen or logging alone supplies no Ready transition.

| Boundary / enforcing owner | Required input, effect and output guarantee |
| --- | --- |
| Caller → lifecycle/admission | Check current policy/witness, exact operation/generation, fixed namespace, finite budgets and allowed purpose before effects. Initialized missing/corrupt/conflicting state is RecoveryRequired, never inferred first installation. |
| Directory/key admission → preparation | Validate actual opened object/type/identity/owner/DACL/volume before destructive work. Prevent traversal/reparse/hard-link/substitution and shared-LocalService owner-mediated access. Hold exact directory/key authority through use; no arbitrary path/provider from an envelope. Prepare fresh immutable authenticated bytes without publishing eligibility. |
| Preparation → native worker | Retain operation, key/file handles, sensitive buffers and resource charge until actual completion. Cancellation/timeout revokes publication, not completion custody; never equate an aborted waiter with finished native work. |
| Native result → catalogue owner | Track issued writes, live bytes, successful/failed durability barriers, durable references and acknowledgments separately. After **durably preparing the exact content/digest**, check the current serialized policy/privacy/root/generation lease **immediately before issuing catalogue effects**. Retain that operation through actual completion, then validate post-effect file identity, owner/DACL and exact content/digest before any receipt/eligibility claim. Windows supplies no atomic key/file/catalogue transaction. |
| Commit → receipt/retirement | Only the declared durable publication yields an attributable receipt/active mode. Revoke old eligibility, retain bounded cleanup obligations and acknowledge the actual transaction. Recovery that discovers a committed operation returns that operation's result without executing it again. |
| Restore → NTS/privacy owner | Bounded framing/allowlist, independent envelope authentication, protected DEK opening, AEAD, mandatory typed semantic admission and current context precede publication. Stored ages or a valid signature do not create current NTS eligibility. Mandatory broker continuity and public leap authority are unchanged. |

The proposed file adapter uses one attributable backup slot when replacing a
catalogue, while immutable prepared snapshot bytes remain separately bound.
Exact namespace, share modes, owner descriptor and durability/recovery sequence
are still platform gates. No-backup behavior below is an adversarial neighbor,
not an alternative default. Hold directory authority without promising universal
handle-relative semantics; path-reconstructing calls and installer handle release
need their actual native tests.

| `ReplaceFileW` result | With admitted backup | Without backup |
| --- | --- | --- |
|1175| Both input names remain; reconcile backup/object state. | Both input names remain. |
|1176| Both input names remain. | Old target is gone; staged replacement retains its name. |
|1177| Staged file retains its name after inheriting old streams/attributes; old target uses the backup name. | Same staging consequence; old target survives under an unspecified different name. |
|Other error| Both input names remain, no backup exists; metadata inheritance may be incomplete. | Same. |

These [documented namespace effects](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-replacefilew)
are **not durable-commit receipts**. Reconcile exact bytes/digest, native file ID,
owner/DACL, attributes and streams before cleanup/retry; successful replacement
uses the staged file's ID and all three files need one volume. Do not select
unsupported WRITE_THROUGH or ignore-ACL/merge flags to pretend durability or
permission continuity. Never delete staging merely because the call failed.

### 7.4 Finite independent traces and engineering exit

| Planned trace | Independent expected result / forbidden effect | Existing mapping |
| --- | --- | --- |
| Ordinary save | Independently fixed `E/hash`, completed barriers and catalogue `g+1/hash` yield one receipt; restart accepts exactly `E`, rejects another validly signed envelope. | ST-T05/10/16/18 |
| Partial mutation | Poisoned transaction cannot acknowledge/commit. Preserve attributable objects; only validated rollback or same-operation reconciliation restores usability. | ST-T05/10/16/17/18 |
| Possible commit then error | Reconcile the same operation; discover either declared noncommit or exact durable `g+1/hash`. Return an already committed receipt without repeating publication/rotation. | ST-T06/07/10/16/17/18 |
| Expected root missing/wrong | RecoveryRequired; no fresh-install inference, enumeration-based replacement, root recreation or downgrade. Ordinary network/mandatory-state behavior follows its separate current policy. | ST-T05/06/07/17 |
| Zero I/O progress | Zero write becomes WriteZero; premature read end becomes UnexpectedEof. No spin, acknowledgment or implicit capacity refund; classify already issued effects separately. | ST-T10/16/18 |
| Cancellation/late completion | Before catalogue effects, a revoked lease refuses issuance. After an effect is issued, retain/reconcile the same possibly durable operation and occupied slot while suppressing stale eligibility. Wipe terminal working buffers without deleting required recovery evidence. Cancellation cannot prove noncommit or undo a historical committed receipt. | ST-T10/16/17/18 |
| Recovery failure | Keep RecoveryRequired, exact identities/evidence and occupied slot. Logging/destruction supplies no Ready transition or destructive reset. | ST-T05/06/07/10/16/17 |
| Second recovery interruption | Stop the independent model at the crash cut; later synthetic success adds no obligation. Next recovery uses the same operation and surviving evidence; no ID/generation reuse. | ST-T06/07/10/16/17/18 |

No traces, crypto fixtures, property decoders, actual-key checks or durability
models ran. Their future source-backed monitors must distinguish issued/live/
durable/acknowledged state and detect the first invalid event. Required line,
mutation, security, native and package gates are unchanged; formal work is post-V1.

| Precise open engineering field / owner | Evidence and dependent no-go |
| --- | --- |
| Canonical encoding/domain/identity/budgets — storage/contracts/security | Freeze exact `H`, signed bytes, complete-envelope digest and independent fixtures plus strict version/rollback behavior. No codec/restore consumer or crypto composition before this. |
| Binding/SDK/status parity — platform/dependency | Reconcile the existing typed REQUIRE alias with the selected SDK/package; freeze relevant features/status mapping and run admitted target controls later. No mixed-generation constants or unsupported downgrade classification. |
| Actual-key origin/mode/reopen — crypto/platform | Qualify both actual roots under final account/build/profile, including VBS-unavailable and no-login neighbors. Unsupported/unknown stays unqualified; source aliases cannot close it. |
| SID/key scope/owner/DACL/provisioning — W11-W05/W11-W10 | Supported MSIX creation-time isolation and second-service denial, including owner-mediated rights and finalization ordering. No product root provisioning before this. |
| Durable catalogue protocol/backup recovery — storage/platform/model QA | Freeze exact barriers/filesystem assumptions and all replacement/recovery cuts, then admit an ST-W02 implementation slice. No durable-receipt or reset claim from API Booleans. |

This card's documentary exit is review of the one proposed row, all eight traces
and their owners/blockers. The **engineering construction freeze remains open**;
ST-W02 is not automatically admitted and ST-W04/native storage remains paused.
Product defaults/strict policy, stronger in-use goals, unsupported alternatives
or trust-boundary changes use NV-DEC; this card grants none. Stop at this card.

**Independent review receipt:** a retained-context reviewer accepted the card,
architecture alias correction and storage-plan routing after four source-contract
repairs and restoration of explicit durable-content-before-catalogue ordering.
The second reviewer accepted the outcome/trace portion. Reviewed task SHA256,
before this status/receipt addendum, was
`1a8dee18521f9d7dfb7c4e38b2ecd407f776f808e01dab9ae1d7402cdeedb567`;
architecture/storage-plan SHA256 were
`eb462d0c33a975c762f2287c180eeace7439595fb129ebc8fce6263ba9c0d184` and
`3546ec21552397b5289ef6c4c0b07d6a9fa75c75cdaf496e3ae08ad01a6ad69f`.
These are documentary/source-consistency results only. No ST-T trace, primitive,
actual key, owner/DACL, persistence, durability or native gate ran. ST-W02 remains
behind its relevant frozen contracts and admitted implementation slice; the
storage experiment pause and every other execution hold remain unchanged.
