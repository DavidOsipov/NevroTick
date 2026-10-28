---
name: "qa-negative-paths"
description: "Test NevroTick invalid inputs, unavailable collaborators and denied operations for correct error, atomicity and forbidden-effect behavior."
---

# Verify expected rejections and failures

Read [the testing contract](../CONTRACT.md).

Derive each expected failure from its owner: transport admission, wire syntax, domain invariants, current authorization, native API or recovery policy. Make the test reach that owner; earlier parser rejection cannot prove a later authorization guard.

Pair malformed and valid-shaped unauthorized/stale/replayed/overbudget cases with valid current neighbors. Assert stable bounded error semantics, unchanged prior valid state and absence of prohibited clock/policy/secret/network/installer effects. Required state must not silently become an empty successful result.

Inspect retry/recovery disposition: no plain-NTP downgrade, unlimited retry loop, cumulative-budget reset or automatic duplicate ambiguous effect. Preserve meaningful diagnostics without secret/raw payload leakage.

Use [strict boundary guidance](../references/numerical-protocol.md). A caught panic, generic error or timeout before the intended action is not evidence of the selected fail-safe behavior.
