---
name: "qa-contract-tests"
description: "Verify strict NevroTick IPC, configuration, domain-construction and consumer compatibility contracts through their actual admitted paths."
---

# Test wire and domain contracts

Read [the testing contract](../CONTRACT.md).

Read applicable implementation contract/[strict contract cases](../references/numerical-protocol.md). Trace raw bytes → bounded map-only DTO → private fallible domain construction → current guard → effect/acknowledgement. Identify each owner/next-boundary consumed representation.

Test unknown/duplicate/escaped keys, object-versus-array shape, missing/null/invalid values, complete consumption, versions, exact integers, finite computed values, limits, every constructor/migration bypass. Include valid neighboring/legitimate maximum-size controls; reject-all must fail.

Compatibility: freeze supported producer/consumer version/feature matrix. Compare meaning, units, presence/defaults, failure behavior; shape/round trips insufficient. Public Rust visibility/type/trait claims need minimal real external consumer; separate wire-version/Rust-source compatibility.

Verify invalid input causes no prohibited effect; valid-shaped unauthorized/stale input fails current guard. Cross-process Windows enforcement requires native checks. Disputed expectation: locate authoritative contract before changing tests/production.
