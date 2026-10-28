---
name: "qa-security-boundaries"
description: "Test NevroTick trust boundaries using real caller identity, current guards and observable protected effects."
---

# Verify protected effects and authority

Read [the testing contract](../CONTRACT.md).

Read the relevant security owner and [Windows evidence](../references/windows-evidence.md). Map untrusted input/caller through transport, strict DTO, domain construction and current trusted effect owner. Identify exactly which authority the test challenges.

Use malformed input plus valid-shaped wrong-identity, wrong-verb, stale-generation, revoked-grant, replayed and overbudget requests. Bypass UI. Assert no forbidden clock/policy/secret/network/installer effect and that the nearest legitimate current request succeeds with its acknowledged result.

For tokens, pipe ACL/server identity, filesystem handles/reparse attacks, clock privileges or SCM control, execute the real Windows path in isolation. Pure mocks test admission logic only. Account for stale asynchronous results and cumulative budgets across reconnect/restart.

Observe bounded errors/work/logging and secret redaction. Keep host security profile and exact candidate identity. Report ASVS applicability and native analogue evidence accurately; a successful boundary test does not establish whole-standard compliance.
