---
name: "qa-property-based-testing"
description: "Use bounded generated cases and shrinking to test a NevroTick invariant with an independent oracle and reproducible counterexamples."
---

# Check quantified Rust invariants

Read [the testing contract](../CONTRACT.md).

State the quantified property, admitted domain/preconditions and plausible wrong implementation first. Choose an existing generator framework such as adopted proptest, or deterministic generated tables if no dependency exists. Do not add a library merely to use this skill.

Generate related fields jointly and meaningful partitions by construction rather than filtering almost everything. Distinguish valid-domain invariants from malformed wire rejection. Useful properties include logical-source deduplication, monotonic budget consumption, generation isolation, finite bounded estimates and conversion constraints, with named analytic examples alongside them.

Keep independent expected results; encode/decode round trips alone may share bugs. Bound generated/rejected/replayed cases, sequence length, allocation, shrink work and total deadline. Fresh state/resources are required for each case/shrink/replay; failed reset invalidates later reductions.

Retain concrete original/minimized input, seed, strategy/tool/configuration/source identity and actual reached failure. Replay the concrete counterexample alone from clean state and keep it as a named regression; seeds can change meaning after strategy edits. Read [Rust techniques](../references/rust-techniques.md).
