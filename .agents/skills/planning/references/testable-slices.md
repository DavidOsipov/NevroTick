# Testable slices and delegation

Expand only next executable stage: input/preconditions, one observable integrated outcome, files/owner, independent oracle, legitimate/relevant failure controls, environment, limits, cleanup, exit. Component checks support, never replace outcome. Explicitly defer later choices; no fabricated APIs/test commands before source exists.

## Development delegation

Proactively delegate useful independent work across the
[development lifecycle](../../../../docs/plans/development-plan.md#development-lifecycle-and-skill-ownership):
planning/research, architecture/threat analysis, implementation, diagnosis/
refactoring, measured optimization, integration, packaging and verification.
Use available capacity and clear ownership to advance the next outcome.
Keep small, tightly coupled or dependency-blocked work local/sequential. No fixed
swarm quota; agent count does not establish progress or independent evidence.
Inspect [current Codex capabilities](../../../../TOOLING.md#codex-subagent-operation)
before dispatch; record a sequential fallback when unavailable.

1. Map dependencies and settle the shared interface before parallel writes:
   producer/consumer, types/units, errors, effects, lifecycle and acceptance.
   Name one coordinator for shared contracts, dependency order and integration;
   that responsibility does not require personally implementing every critical
   path. Assign each independent slice and each cross-worker seam an owner.
2. Give each worker a bounded packet: lifecycle stage/product phase, task/canonical sections/acceptance IDs,
   exact source and relevant dirty/configuration identity, required skills/raw
   inputs, interface contract, exclusive writable paths/resources, allowed
   effects, independent expected observations, finite budget, deliverable and
   cleanup. Name the actual output: decision/contract, implementation/refactor
   patch, causal diagnosis, measured optimization, compatibility transition,
   package inputs or integrated candidate. Each worker applies the planning,
   implementation, security, environment, specialist and QA skills required by
   its stage, phase and changed guarantee. Delegation grants no new authority or
   gate exemption.
3. Follow [workstation isolation](../../../../docs/windows-time-development-hardening.md#33-agent-workstation-and-sandbox-rules):
   parallel writers use separate worktrees/output directories and disjoint write
   ownership; serialize overlapping edits and shared-contract changes. Assign
   `Cargo.lock`, generated outputs, build caches and native lab state explicitly.
   Worktrees do not isolate machine state. Bound aggregate CPU/memory/process/
   disk/time use, including nested compiler/test workers; preserve admitted lane
   limits and current execution holds.
4. While workers run, advance unassigned nonoverlapping work, resolve dependencies
   or prepare integration. Do not redo the delegated task or idle-wait while
   useful independent work remains. Notify the coordinator before changing an
   agreed interface or write scope; pause affected dependents, preserve others'
   work and reconcile the contract before continuation. No worker silently
   reverts another's changes.
5. Require a compact return: stage deliverable/decisions, input/output candidate
   identity, changed paths and patch/commit, requirement-to-observation results with exact commands/status,
   raw-evidence locations, first failures/gaps, consumed budget and remaining
   processes/cleanup. Keep verbose logs in their owned artifacts. A launch,
   timeout, cancellation or absent result is incomplete, not a passing check.
6. Inspect returned patches and raw evidence, reconcile conflicts, then verify
   the combined candidate and cross-worker seams before acceptance. Reuse
   qualified unchanged evidence; run affected checks and full gates at prescribed
   exits. Worker success or agreement cannot close an untested integrated path.
   Use [QA dispatch](../../testing/qa-subagent-dispatch/SKILL.md) for complementary
   verification and [fresh review](review-and-handoff.md#freeze-review-inputs-and-account-for-completion)
   at required exits: give requirements/frozen raw inputs, not the author's
   verdict or inherited discussion. Record actual reviewer completion.

Checkpoint in the existing task via
[context continuity](../../testing/qa-context-checkpoint/SKILL.md): owners,
dependencies, patch/integration status, evidence, live effects, consumed budgets,
unresolved decisions and next action. Preserve applicable incomplete obligations,
holds and accepted deferrals; continue independent authorized work.

## Evidence environments

Keep environment claims separate:

- Pure Rust behavior can be tested on a supported host with injected observations and deterministic oracles.
- Cross-compiling proves only the compiler/linker path actually run, not Windows token, service, IPC or clock behavior.
- Native Windows experiments require the relevant effective identities, API observations, bounded side effects and cleanup. Clock experiments use disposable VMs under architecture section 14.9.
- Packaging needs the exact selected complete package and applicable install/repair/upgrade/rollback/uninstall observations. The current direct MSIX route distinguishes built, signed, distributed and installed artifacts; MSI remains a later owner-selected alternative. Use the [package evidence contract](../../testing/qa-package-artifacts/SKILL.md). Signing and release readiness are separate evidence and authority.

Without Cargo workspace, Cargo checks unavailable for task. Tool smoke tests/throwaway Rust examples do not qualify NevroTick. Documentation integrity/skill-pack checks remain valid bounded work.
