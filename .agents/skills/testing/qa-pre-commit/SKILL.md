---
name: "qa-pre-commit"
description: "Review or prepare staged content, update relevant documentation and tasks, and verify the complete intended Git candidate while preserving user staging."
---

# Review a commit candidate faithfully

Read [the testing contract](../CONTRACT.md).

Resolve task authority/repository/index/HEAD state/intended changes. Separately inspect staged/unstaged/untracked work; preserve existing user hunks/receipts. Stage only with existing task authorization using literal intended paths/reviewed hunks.

Before final candidate freeze, **update relevant documentation and tasks in the
same change**. Use the [documentation guide](../../../../docs/README.md) and
[task map](../../../../tasks/README.md) to find existing canonical owners. Reconcile
affected behavior, contracts, configuration and usage documentation with the
actual change. Update the owning task's implementation and verification status,
observed evidence, remaining gates/blockers and next action; reconcile affected
development-plan, roadmap and backlog routing. Distinguish implemented,
development-checked and qualified work. Preserve dated historical receipts,
source bindings, acceptance thresholds and execution holds; a scoped pass does
not close its parent phase. Include these updates in the reviewed candidate.
If no documentation/task update applies, state the reason in the closeout;
avoid unrelated edits or duplicate plans/reports.

Read [staged-candidate verification](../references/staged-candidate.md) for index-dependent readiness. Dirty-working-tree pass does not establish complete staged tree; changed-file copy may omit unchanged dependencies/staged deletions. Materialization must preserve required bytes/modes/attributes/inputs/Git-dependent checks.

Review changes for reachable defects and required tests, configuration/lockfile/fixtures, generated output and accidental secrets. Meaningful suspected repair needs independent causal evidence and valid controls within existing scope. `git diff --cached --check` is only a structural/whitespace check.

For changed Rust contracts, inspect the staged API's pre/postconditions and actual
mandatory construction/checked-output path. Reject added bypasses, unchecked
public representations or `allow_unvalidated`; require reviewed ownership for
each `skip`. Reconcile [contract QA](../qa-contract-tests/SKILL.md) evidence with
the materialized candidate, including direct callers and legitimate controls.

Select gates from [configured tool execution](../references/configured-tooling.md) by the staged changes: workflow edits use workflow/security checks; Rust edits use the affected format/compiler/Clippy and regression lanes. Include candidate secret review where required. Repository wrappers bind their script root: execute the candidate's copy, not the original checkout's script with a different working directory. Record identity/results; compare custody before closeout. Formatting/hooks/index changes invalidate affected evidence. Skill implies no commit/push/hook bypass/unrelated cleanup.

Before commit readiness, confirm the relevant docs/tasks reflect the final diff
and actual verification outcomes. Later reconciliation edits require an updated
candidate identity and reassessment of affected evidence under the testing
contract; do not claim readiness for an earlier tree. Report the updated owners
or the reason no update was needed.
