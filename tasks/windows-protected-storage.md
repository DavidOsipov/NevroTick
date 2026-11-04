# Protected optional storage: agent task and evidence record

**Date:** 3 November 2026 (Asia/Tbilisi).  
**Status:** Documentation, plan and task integration reviewed and recorded; source-only follow-up ready, implementation and native qualification unexecuted.  
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

### ST-W01 Prepare one construction and failure contract

**Owner:** storage lead, with bounded platform/security review.  
**Readiness:** ready for the next source/documentation assignment; not executed by this integration.  
**Scope:** one proposed construction/provider/build/account/profile row, one operation-outcome table and one ownership/reconciliation map. Reuse the inventories; do not restart a general tool survey.

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
