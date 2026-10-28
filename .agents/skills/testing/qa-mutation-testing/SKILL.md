---
name: "qa-mutation-testing"
description: "Use a bounded NevroTick mutation campaign to assess behavioral assertions, preserving the architecture per-critical-file gate and honest outcome accounting."
---

# Assess critical test sensitivity

Read [the testing contract](../CONTRACT.md).

Read [Rust techniques](../references/rust-techniques.md). Before mutation freeze source/candidate, selected critical-file manifest, unmodified passing baseline. Use adopted tool/isolated reversible bad variant; no convenient in-place mutation of user-owned source.

Choose requirement-linked faults: sign/unit mistakes, removed current guards, reduced source minima, stale-result publication, duplicate effects, broken failure atomicity. Killing requires meaningful actual-boundary assertion; unrelated build/fixture failure insufficient.

Record per-file generated/viable/killed/survived/timed-out/unviable/excluded mutants, actual tests, source identity. Compile failures unviable, not killed. Investigate timeout/survivors; independently justify equivalence/exclusion. Empty selections give no passing score.

Preserve **95% score per critical file** release obligation; focused trial/favorable aggregate cannot satisfy unrun full manifest. Bound workers/time; retain first failures. Unsupported Windows mutation paths remain visible gate gap; retain critical scope.
