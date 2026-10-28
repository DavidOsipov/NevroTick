---
name: "implement-rust-contracts"
description: "Implement or review NevroTick serialized boundaries, strict Serde DTO decoding, private domain invariants and checked input-to-effect paths."
---

# Implement Rust contracts

Read [the scoped contract](../CONTRACT.md). Canonical architecture sections 5–6
and 18.5 select Serde DTOs plus fallible domain construction, not general-purpose
validation frameworks. New package versions/features still need admission.
Preserve review/planning/tests-only modes.

Trace actual raw bytes → decoding → construction → contextual guard → effect →
acknowledgement. Identify owners and all constructor/mutation bypasses. Internal
transformations need preconditions, postconditions, failure behavior and effects,
not invented serialization boundaries.

Define changed guarantees: representation/version, exact unit/sign/range,
presence versus null, identity/normalization, cross-field rules, provenance,
resource limits and current-state requirements. Equal shapes need not imply equal
authority. Read [strict boundary patterns](references/strict-boundaries.md) before
JSON/persistence adapter changes; binary NTP/NTS needs bounded protocol-specific
parsing/checks.

Implement at the real owner:

1. Authenticate/admit transport or protected file reads; bound work/bytes before
   expensive allocation. Decode one complete supported representation into inert
   wire records; no network/file/service/clock effects inside decoders.
2. Construct private domain values through checked `TryFrom` or named constructors.
   Consumers use checked result, never original DTO.
3. Receiving service uses OS-established identity, current policy,
   revision/generation, replay/freshness/ownership and cumulative limits. Serialize
   guard/effect; recheck after async work. Validated wrappers cannot preserve
   permission indefinitely.
4. Apply checked Windows conversions and reconcile actual acknowledgements.
   Invalid input has no protected effect; ambiguous outcomes do not justify an
   automatic repeated operation.

Before edits, identify detecting check and valid neighboring control. Verify actual
raw adapter/checked-output consumer, constructor non-bypassability, exact integer
round trips, post-transform bounds and failure atomicity. Use
[qa-contract-tests](../../testing/qa-contract-tests/SKILL.md) and selected QA owners;
source-text matching alone is insufficient. CHECK-25 plans whole-path qualification;
real pipe identity/clock effects need Windows lanes.

Use [evolution](../evolve-data-contracts/SKILL.md) before acceptance, normalization,
meaning, version, storage or consumer-compatibility changes. Share pure UI/service
validators; receiver independently decodes, constructs and authorizes. Report changed
owner/path, failures, affected consumers and executed/blocked checks. Schema/compiler
success proves no security.

Adapted David Osipov skill material: [licence and notices](THIRD_PARTY_NOTICES.md).
