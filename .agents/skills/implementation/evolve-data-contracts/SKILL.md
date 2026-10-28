---
name: "evolve-data-contracts"
description: "Plan, implement or review compatible changes to NevroTick IPC, persisted state, units, normalization, versions and independently consumed outputs."
---

# Evolve data contracts

Read [the scoped contract](../CONTRACT.md). Compare semantics and lifetimes, not
only Rust field shapes. Trace raw input, domain construction, state, serialization
and actual consumers, including persisted data surviving upgrades or rollback.
Reuse the current plan and authorized endpoint: code, rollout or retirement.

Identify owner, old/new acceptance, meaning, units/sign, presence/null/defaults,
normalization, provenance and failure behavior. Decoder attributes, enabled
features, dependency versions and adapters can alter acceptance with unchanged
DTO source. A private newtype change alone need not change a wire format.

Before dependent edits, decide a compatibility matrix for required old/new
producers, readers and stored generations. Strict old readers may reject an added
optional field; a permissive reader can silently lose meaning. Required unknown or
failing combinations remain unverified/failed, not retrospectively declared
unsupported to obtain a pass.

Choose an explicit supported transition: versioned records, compatible projection,
reader-before-writer sequence, bounded migration or coordinated update. Define
unsupported-version behavior, protected migration ownership, interrupted writes,
rollback and mixed-component safety. Use
[transition procedure](references/transitions.md) when persistence, independent
versions or privileged lifecycle work is involved. A coupled internal change may
omit migration when no old data/consumer survives; explain that boundary.

Establish raw old/new fixtures and independent expected results before changing
owners. Implement bounded slices through
[implement-rust-contracts](../implement-rust-contracts/SKILL.md); hand evidence back
to this same task without restarting planning. Preserve canonical source/provenance
where required; do not introduce raw shadow storage by default.

Exercise actual serialized bytes through required consumers in both directions,
exact integer and temporal fidelity, rejection, failure atomicity and recovery.
Use [contract QA](../../testing/qa-contract-tests/SKILL.md) and selected lifecycle
tests. Never relax old fixtures/assertions solely to hide incompatibility.

Finish with supported combinations, changed owners, migration/rollback outcomes,
executed checks and outstanding holds. Code-ready, installed, published and old
reader/data retirement are separate endpoints. Do not remove the old path merely
because new-path tests pass.

Adapted David Osipov skill material: [licence and notices](THIRD_PARTY_NOTICES.md).
