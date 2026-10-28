---
name: "qa-property-based-testing"
description: "Use bounded generated cases and shrinking to test a NevroTick invariant with an independent oracle and reproducible counterexamples."
---

# Check quantified Rust invariants

Read [the testing contract](../CONTRACT.md).

First state quantified property/admitted domain/preconditions/plausible wrong implementation. Use existing generator framework such as adopted proptest, or deterministic generated tables without dependency. No library merely for skill.

Generate related fields jointly/meaningful partitions by construction; avoid near-total filtering. Distinguish valid-domain invariants/malformed wire rejection. Useful properties: logical-source deduplication, monotonic budget consumption, generation isolation, finite bounded estimates, conversion constraints; include named analytic examples.

Keep independent expectations; encode/decode round trips alone may share bugs. Bound generated/rejected/replayed cases, sequence length, allocation, shrink work, total deadline. Each case/shrink/replay needs fresh state/resources; failed reset invalidates later reductions.

Retain concrete original/minimized input, seed, strategy/tool/configuration/source identity, actual reached failure. Replay concrete counterexample alone from clean state; keep named regression. Strategy edits can change seed meaning. Read [Rust techniques](../references/rust-techniques.md).
