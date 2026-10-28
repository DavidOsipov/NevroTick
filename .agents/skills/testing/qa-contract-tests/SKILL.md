---
name: "qa-contract-tests"
description: "Verify strict NevroTick IPC, configuration, domain-construction and consumer compatibility contracts through their actual admitted paths."
---

# Test wire and domain contracts

Read [the testing contract](../CONTRACT.md).

Read the applicable implementation contract and [strict contract cases](../references/numerical-protocol.md). Trace raw bytes → bounded map-only DTO → private fallible domain construction → current guard → effect/acknowledgement. Identify each owner and the representation consumed at the next boundary.

Test unknown/duplicate/escaped keys, object-versus-array shape, missing/null/invalid values, complete consumption, versions, exact integers, finite computed values, limits and every constructor/migration bypass. Include valid neighboring and legitimate maximum-size cases so reject-all cannot pass.

For compatibility, freeze the supported producer/consumer version and feature matrix. Compare meaning, units, presence/defaults and failure behavior, not just field shape or round trips. Public Rust APIs need a minimal real external consumer where visibility/type/trait claims matter; wire version compatibility remains separate from Rust source compatibility.

Verify invalid input has no prohibited effect and valid-shaped unauthorized/stale input still fails the current guard. Cross-process Windows enforcement requires its native checks. For a disputed expectation, locate the authoritative contract before altering tests or production.
