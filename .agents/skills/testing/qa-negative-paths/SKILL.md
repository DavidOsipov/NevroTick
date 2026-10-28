---
name: "qa-negative-paths"
description: "Test NevroTick invalid inputs, unavailable collaborators and denied operations for correct error, atomicity and forbidden-effect behavior."
---

# Verify expected rejections and failures

Read [the testing contract](../CONTRACT.md).

Derive failures from actual owner: transport admission, wire syntax, domain invariants, current authorization, native API, recovery policy. Test must reach owner; earlier parser rejection cannot prove later authorization guard.

Pair malformed/valid-shaped unauthorized/stale/replayed/overbudget cases with valid current neighbors. Assert stable bounded errors, unchanged prior valid state, no prohibited clock/policy/secret/network/installer effects. Required state must not silently become empty success.

Inspect retry/recovery: no plain-NTP downgrade, unlimited retries, cumulative-budget reset or automatic duplicate ambiguous effect. Preserve meaningful diagnostics; no secret/raw payload leakage.

Use [strict boundary guidance](../references/numerical-protocol.md). A caught panic, generic error or timeout before the intended action is not evidence of the selected fail-safe behavior.
