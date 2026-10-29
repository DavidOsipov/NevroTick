---
name: "qa-property-based-testing"
description: "Use bounded generated cases and shrinking to test a NevroTick invariant with an independent oracle and reproducible counterexamples."
---

# Check quantified Rust invariants

Read [the testing contract](../CONTRACT.md).

First state quantified property/admitted domain/preconditions/plausible wrong implementation. Use existing generator framework such as adopted proptest, or deterministic generated tables without dependency. No library merely for skill.

Generate related fields jointly/meaningful partitions by construction; avoid near-total filtering. Distinguish valid-domain invariants/malformed wire rejection. Useful properties: logical-source deduplication, monotonic budget consumption, generation isolation, finite bounded estimates, conversion constraints; include named analytic examples.

Generate valid typed inputs to test function/module postconditions across
transformations, plus invalid raw values for real constructor rejection. Do not
use the production garde validator as the expected-result oracle or filter away
the invalid cases the boundary must reject. Model mutation/context invalidation
separately from immutable value invariants.

Keep independent expectations; encode/decode round trips alone may share bugs. Bound generated/rejected/replayed cases, sequence length, allocation, shrink work, total deadline. Each case/shrink/replay needs fresh state/resources; failed reset invalidates later reductions.

Retain concrete original/minimized input, seed, strategy/tool/configuration/source identity, actual reached failure. Replay concrete counterexample alone from clean state; keep named regression. Strategy edits can change seed meaning. Read [Rust techniques](../references/rust-techniques.md).

When a small finite invariant needs bounded proof beyond sampled cases, select
admitted Kani through [configured analysis](../references/configured-tooling.md#additional-rust-analysis).
Keep independent expectations, explicit assumptions/unwind bounds and reachable
valid-state controls; report all incomplete obligations and excluded inputs.
For malformed byte coverage, route a separate production-decoder target through
[structured fuzzing](../qa-structured-fuzzing/SKILL.md). Neither tool changes the
quantified domain or replaces required native observations.
