---
name: "qa-mutation-testing"
description: "Use a bounded NevroTick mutation campaign to assess behavioral assertions, preserving the architecture per-critical-file gate and honest outcome accounting."
---

# Assess critical test sensitivity

Read [the testing contract](../CONTRACT.md).

Read [Rust techniques](../references/rust-techniques.md). Freeze the source/candidate, selected critical-file manifest and unmodified passing baseline before mutation. Use the adopted tool or an isolated reversible bad variant; do not mutate user-owned source in place for convenience.

Choose faults tied to requirements: sign/unit mistakes, removed current guards, reduced source minima, stale-result publication, duplicate effects or broken failure atomicity. Require a meaningful assertion at the actual boundary to kill the mutant, not an unrelated build/fixture failure.

Record generated/viable/killed/survived/timed-out/unviable/excluded mutants per file, actual tests and source identity. Compile-failing mutants are unviable, not killed. Investigate timeout and survivors; equivalence/exclusion needs independent justification. Empty selections provide no passing score.

Preserve the **95% score per critical file** release obligation; a focused trial or favorable aggregate cannot satisfy an unrun full manifest. Bound workers/time and retain first failures. Unsupported Windows mutation paths remain a visible gate gap, not an excuse to remove them from critical scope.
