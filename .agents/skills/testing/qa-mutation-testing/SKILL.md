---
name: "qa-mutation-testing"
description: "Use a bounded NevroTick mutation campaign to assess behavioral assertions, preserving the architecture per-critical-file gate and honest outcome accounting."
---

# Assess critical test sensitivity

Read [the testing contract](../CONTRACT.md).

Read [Rust techniques](../references/rust-techniques.md). Before mutation freeze source/candidate, selected critical-file manifest, unmodified passing baseline. Use adopted tool/isolated reversible bad variant; no convenient in-place mutation of user-owned source.

Use [configured tool execution](../references/configured-tooling.md) and the admitted cargo-mutants configuration. Select actual files/target/features and explicit baseline/jobs/test/build budgets; keep ordinary lint capping disabled. Installed/enumerated tools and an empty inventory do not establish a campaign or score.

Choose requirement-linked faults: sign/unit mistakes, removed current guards, reduced source minima, stale-result publication, duplicate effects, broken failure atomicity. Killing requires meaningful actual-boundary assertion; unrelated build/fixture failure insufficient.

Record per-file generated/viable/killed/survived/timed-out/unviable/excluded mutants, actual tests, source identity. Compile failures unviable, not killed. Investigate timeout/survivors; independently justify equivalence/exclusion. Empty selections give no passing score.

Before scoring, reconcile each critical file's requirement-bearing functions and guards against the actual generated inventory. Record source location, target/features, generated mutant identifiers or the reason none exist, and the independent evidence for every uncovered behavior. Include automatically skipped `unsafe fn`, explicit filters/skip attributes, macro-generated validation/serialization and build-generated code. A mixed file can pass its generated-mutant score while a critical function has no mutants. Automatically skipped or invisible code is outside generation; never classify it as a killed mutant or an equivalent missed mutant.

For uncovered behavior select discriminating actual-handler positive/negative cases, an isolated reviewed fault injection, or reached native memory/lifecycle checks as appropriate. Derive expected facts independently of the generated implementation. Retain missing alternatives as qualification gaps; a percentage cannot discharge them. Do not remove required `unsafe` declarations or weaken ABI/contracts merely to obtain mutants. A safe function containing an unsafe block is not the same exclusion as `unsafe fn`; inspect the pinned tool's enumeration rather than extrapolating.

For a proposed equivalence, use [boundary-domain proof](../references/mutation-equivalence.md). Distinguish the mutated helper's admitted inputs from values reachable through today's checked callers; retain a concrete witness or a source-bound argument over outputs, errors and effects.

Preserve **95% score per critical file** release obligation; focused trial/favorable aggregate cannot satisfy unrun full manifest. Bound workers/time; retain first failures. Unsupported Windows mutation paths remain visible gate gap; retain critical scope.

Tool limits: cargo-mutants 27.1.0 [excluded functions](https://github.com/sourcefrog/cargo-mutants/blob/v27.1.0/book/src/mutants.md#functions-that-are-excluded-from-mutation), [macro behavior](https://github.com/sourcefrog/cargo-mutants/blob/v27.1.0/book/src/macros.md) and [generation limitations](https://github.com/sourcefrog/cargo-mutants/blob/v27.1.0/book/src/limitations.md). Recheck these against the admitted version when changing the tool.
