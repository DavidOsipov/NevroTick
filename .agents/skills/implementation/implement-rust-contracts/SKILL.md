---
name: "implement-rust-contracts"
description: "Implement or review NevroTick serialized boundaries, strict Serde DTO decoding, private domain invariants and checked input-to-effect paths."
---

# Implement Rust contracts

Read [the scoped contract](../CONTRACT.md). Use the canonical architecture sections
5–6 and 18.5; it selects Serde DTOs plus fallible domain construction, not a
general-purpose validation framework. New package versions/features still require
admission. Review/planning/tests-only requests retain those modes.

Trace the actual raw bytes through decoding, construction, contextual guard,
effect and acknowledgement. Identify each owner and all constructor/mutation
bypasses. For internal transformations, state preconditions, postconditions,
failure behavior and effects without inventing another serialization boundary.

Define the changed guarantees: representation/version, exact unit/sign/range,
presence versus null, identity/normalization, cross-field rules, provenance,
resource limits and current-state requirements. Equal shapes need not mean equal
authority. Use [strict boundary patterns](references/strict-boundaries.md) before
changing JSON or persistence adapters; binary NTP/NTS needs its own bounded parser
and protocol checks.

Implement at the real owner:

1. Authenticate/admit the transport or protected file read and bound work/bytes
   before expensive allocation. Decode one complete supported representation into
   inert wire records; no network/file/service/clock effects inside decoders.
2. Construct private domain values through checked `TryFrom` or named constructors.
   Pass that checked result to consumers; do not execute the original DTO.
3. At the receiving service, use OS-established identity and current policy,
   revision/generation, replay/freshness/ownership and cumulative limits. Serialize
   guard and effect; recheck after async work. A validated wrapper cannot preserve
   permission indefinitely.
4. Apply checked Windows conversions and reconcile actual acknowledgements.
   Invalid input has no protected effect; ambiguous outcomes do not justify an
   automatic repeated operation.

Before edits, identify the detecting check and valid neighboring control. Verify
the actual raw adapter and checked-output consumer, constructor non-bypassability,
exact integer round trips, post-transform bounds, and failure atomicity. Use
[qa-contract-tests](../../testing/qa-contract-tests/SKILL.md) and other selected QA
owners; source-text matching alone is insufficient. CHECK-25 is planned whole-path
qualification; real pipe identity/clock effects need their Windows lanes.

Use [evolution](../evolve-data-contracts/SKILL.md) before changing accepted values,
normalization, meaning, versions, storage or consumer compatibility. Share pure
UI/service validators but independently decode, construct and authorize at the
receiver. Report changed owner/path, failure behavior, affected consumers and
executed/blocked checks; do not claim security from schema or compiler success.

Adapted David Osipov skill material: [licence and notices](THIRD_PARTY_NOTICES.md).
