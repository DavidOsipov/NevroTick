---
name: "qa-pre-commit"
description: "Review or prepare NevroTick staged content with preserved user staging and verification tied to the complete intended Git candidate."
---

# Review a commit candidate faithfully

Read [the testing contract](../CONTRACT.md).

Resolve task authority, repository/index/HEAD state and intended changes. Inspect staged, unstaged and untracked work separately; preserve existing user-owned hunks and receipts. Stage only when the task already authorizes it, using literal intended paths or reviewed hunks.

Read [staged-candidate verification](../references/staged-candidate.md) when readiness depends on index contents. A dirty-working-tree pass does not establish the complete staged tree; changed-file copying can omit unchanged dependencies or staged deletions. Materialization must preserve required bytes/modes, attributes, inputs and any Git-dependent checks.

Review changes for reachable defects and required tests, configuration/lockfile/fixtures, generated output and accidental secrets. Meaningful suspected repair needs independent causal evidence and valid controls within existing scope. `git diff --cached --check` is only a structural/whitespace check.

Run applicable selected gates on the actual candidate, record identity/results and compare custody before closeout. Formatting/hooks/index changes invalidate affected evidence. Do not commit, push, bypass hooks or clean unrelated work as an implicit consequence of this skill.
