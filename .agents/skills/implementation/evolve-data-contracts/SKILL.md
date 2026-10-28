---
name: "evolve-data-contracts"
description: "Plan, implement or review compatible changes to NevroTick IPC, persisted state, units, normalization, versions and independently consumed outputs."
---

# Evolve data contracts

Read [the scoped contract](../CONTRACT.md). Compare semantics/lifetimes beyond Rust
field shapes. Trace raw input → domain construction → state → serialization →
actual consumers, including data surviving upgrades/rollback. Reuse current plan
and authorized endpoint: code, rollout or retirement.

Identify owner, old/new acceptance, meaning, units/sign, presence/null/defaults,
normalization, provenance and failure behavior. Decoder attributes, enabled
features, dependency versions and adapters can change acceptance without DTO
source changes. Private newtype changes need not change wire format.

Before dependent edits, decide required old/new producer, reader and stored-generation
compatibility matrix. Strict old readers may reject added optional fields;
permissive readers can silently lose meaning. Required unknown/failing combinations
remain unverified/failed; never relabel unsupported retrospectively to obtain passes.

Choose explicit supported transition: versioned records, compatible projection,
reader-before-writer sequence, bounded migration or coordinated update. Define
unsupported-version behavior, protected migration owner, interrupted writes,
rollback and mixed-component safety. Use [transition procedure](references/transitions.md)
for persistence, independent versions or privileged lifecycle work. Coupled internal
changes may omit migration only when no old data/consumer survives; explain boundary.

Before owner changes, establish raw old/new fixtures and independent expectations.
Implement bounded slices through [implement-rust-contracts](../implement-rust-contracts/SKILL.md);
return evidence to same task without restarting planning. Preserve required
canonical source/provenance; no raw shadow storage by default.

Exercise actual serialized bytes through required consumers both directions;
check exact integer/temporal fidelity, rejection, failure atomicity and recovery.
Use [contract QA](../../testing/qa-contract-tests/SKILL.md) and selected lifecycle
tests. Never relax old fixtures/assertions solely to hide incompatibility.

Report supported combinations, changed owners, migration/rollback outcomes,
executed checks and holds. Code-ready, installed, published and old reader/data
retirement remain separate endpoints. Passing new-path tests alone cannot retire
old path.

Adapted David Osipov skill material: [licence and notices](THIRD_PARTY_NOTICES.md).
