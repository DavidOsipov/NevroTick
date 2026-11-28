---
name: "qa-final-qualification"
description: "Assess final verification evidence for the exact candidate and declared scope; plans, skips and host checks are not release passes."
---

# Qualify the observed candidate

Read [the testing contract](../CONTRACT.md).

Freeze final relevant source/configuration/lockfile/fixtures/built/installed artifact identities. Reconcile selected requirements/actual named execution/all outcomes/first failures/retries/resources/current unresolved findings. Earlier-candidate evidence needs explicit unaffected-state argument.

Confirm selected required gates completed at adequate evidence layer. Compiler/host simulation, actual Windows tokens/IPC/clock/SCM, native UI/AT, package trust/lifecycle, hardware soak establish distinct claims. Preserve 90% security-module executable-line coverage/95% per-critical-file mutation gates when declared implementation/release scope requires.

Within the declared Rust scope, reconcile function/module contracts with their
enforcing types/constructors/guards and executed bypass/postcondition controls.
Installed garde, deriving `Validate`, green annotation checks or a documented
contract do not establish unavoidable consumer validation. Apply
[contract QA](../qa-contract-tests/SKILL.md); missing required direct-call,
transformation or rejection-effect evidence remains incomplete.

For product mathematics, apply [OD-10’s first-release deferral](../../../../docs/nevrotick-owner-decisions.md#141-first-release-deferral--6-november-2026):
formal work and pending proof admission/metadata repairs resume after V1.
Missing or invalidated proofs do not block V1 acceptance or release. Record
remaining obligations under the existing source/CHECK owners as post-V1 work;
retain completed evidence only within its exact historical scope. Continue
independent behavioral/property/boundary checks and all nonformal gates.
Use the [proof workflow](../references/formal-verification.md) on post-V1 resumption.

For required native memory/resource claims, reconcile
[Windows diagnostics](../references/windows-memory.md): exact image/tools,
effective checks, intended execution token, reached cases and first stops,
allocator coverage, retention observations and verified settings restoration.
Tool controls, PageHeap success and process exit cannot substitute for required
product cleanup or sustained-resource evidence; instrumentation changes the
performance measurement conditions.

At module completion and before V1 release, reconcile
[sanitizer qualification](../qa-sanitizer-qualification/SKILL.md): applicable
families, actual instrumented product graph and execution, detector-specific
controls/diagnostics, semantic results, raw/accepted/reached fuzz counts and native
gaps. V1 needs the complete applicable matrix for the integrated candidate and
shipped configurations. Reuse earlier evidence only with an explicit identity and
validity argument; execute missing/invalidated required rows and preserve fresh
checkpoint duties. Fixed detector fixtures or assertion-only controls cannot
establish product ASan coverage or general leak freedom. Unsupported, held or
unqualified required execution remains incomplete, not N/A or a release pass.

For whole-system stabilization or explicit acceptance require direct [black-box inspection](../qa-black-box-acceptance/SKILL.md) of representative actual inputs/outputs on the frozen integrated candidate. Review source-derived claims and independent findings separately from runtime evidence.

Return qualified within declared scope, not ready for confirmed blockers, or incomplete for missing required evidence; give exact remaining action. Required failures/skips/cancellations/timeouts remain incomplete. Skill/documentation-only changes need pack verification, no unimplemented product gates. No staged readiness/signing/release approval/standard certification claim unless separately established.
