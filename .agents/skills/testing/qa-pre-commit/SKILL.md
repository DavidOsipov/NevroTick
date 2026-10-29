---
name: "qa-pre-commit"
description: "Review or prepare NevroTick staged content with preserved user staging and verification tied to the complete intended Git candidate."
---

# Review a commit candidate faithfully

Read [the testing contract](../CONTRACT.md).

Resolve task authority/repository/index/HEAD state/intended changes. Separately inspect staged/unstaged/untracked work; preserve existing user hunks/receipts. Stage only with existing task authorization using literal intended paths/reviewed hunks.

Read [staged-candidate verification](../references/staged-candidate.md) for index-dependent readiness. Dirty-working-tree pass does not establish complete staged tree; changed-file copy may omit unchanged dependencies/staged deletions. Materialization must preserve required bytes/modes/attributes/inputs/Git-dependent checks.

Review changes for reachable defects and required tests, configuration/lockfile/fixtures, generated output and accidental secrets. Meaningful suspected repair needs independent causal evidence and valid controls within existing scope. `git diff --cached --check` is only a structural/whitespace check.

For changed Rust contracts, inspect the staged API's pre/postconditions and actual
mandatory construction/checked-output path. Reject added bypasses, unchecked
public representations or `allow_unvalidated`; require reviewed ownership for
each `skip`. Reconcile [contract QA](../qa-contract-tests/SKILL.md) evidence with
the materialized candidate, including direct callers and legitimate controls.

Select gates from [configured tool execution](../references/configured-tooling.md) by the staged changes: workflow edits use workflow/security checks; Rust edits use the affected format/compiler/Clippy and regression lanes. Include candidate secret review where required. Repository wrappers bind their script root: execute the candidate's copy, not the original checkout's script with a different working directory. Record identity/results; compare custody before closeout. Formatting/hooks/index changes invalidate affected evidence. Skill implies no commit/push/hook bypass/unrelated cleanup.
