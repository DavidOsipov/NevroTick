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

Conditional or stateful properties: identify cases satisfying preconditions and
reaching each selected semantic partition. Sparse generation: retain concrete
witnesses and reached-case counts. Unreached antecedents or selected partitions
remain evidence gaps. Reuse analytic anchors and qualified fault controls; no new
framework, percentage target or full generator census required.

Keep independent expectations; encode/decode round trips alone may share bugs. Bound generated/rejected/replayed cases, sequence length, allocation, shrink work, total deadline. Each case/shrink/replay needs fresh state/resources; failed reset invalidates later reductions.

Retain concrete original/minimized input, seed, strategy/tool/configuration/source identity, actual reached failure. Replay concrete counterexample alone from clean state; keep named regression. Strategy edits can change seed meaning. Read [Rust techniques](../references/rust-techniques.md).

Product mathematics follows [OD-10’s first-release deferral](../../../../docs/nevrotick-owner-decisions.md#141-first-release-deferral--6-november-2026).
Continue independent sampled properties and boundary cases; they establish their
observed scope, not a full-domain proof. Formal jobs, verifier extensions and
pending proof admission/metadata repairs resume after V1. Missing proofs do not
block V1 acceptance; retain exact historical evidence and post-V1 obligations in
the existing tasks. Use the [proof workflow](../references/formal-verification.md)
on resumption.

For malformed byte coverage, route a separate production-decoder target through
[structured fuzzing](../qa-structured-fuzzing/SKILL.md). Neither tool changes the
quantified domain or replaces required native observations.
