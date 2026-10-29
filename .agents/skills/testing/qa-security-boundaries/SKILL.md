---
name: "qa-security-boundaries"
description: "Test NevroTick trust boundaries using real caller identity, current guards and observable protected effects."
---

# Verify protected effects and authority

Read [the testing contract](../CONTRACT.md).

Read relevant security owner/[Windows evidence](../references/windows-evidence.md). Map untrusted input/caller through transport/strict DTO/domain construction/current trusted effect owner. Identify exact challenged authority.

When cross-function propagation or an apparent guard bypass is uncertain, use
[CodeQL data-flow analysis](../references/codeql-data-flow.md) with named
attacker/secret sources, transformations and protected/output sinks. Review the
actual path and guard semantics, including stale checks after asynchronous work;
turn credible findings into the bounded behavioral cases below. Missing models
or findings retain the gap rather than proving an effect unreachable.

Use malformed input plus valid-shaped wrong-identity/wrong-verb/stale-generation/revoked-grant/replayed/overbudget requests. Bypass UI. Assert no forbidden clock/policy/secret/network/installer effect; nearest legitimate current request succeeds with acknowledged result.

For tokens/pipe ACL/server identity/filesystem handles/reparse attacks/clock privileges/SCM control, execute isolated real Windows path. Pure mocks test admission logic only. Account for stale async results/cumulative budgets across reconnect/restart.

Observe bounded errors/work/logging/secret redaction. Keep host security profile/exact candidate identity. Report ASVS applicability/native analogue evidence accurately; boundary success does not establish whole-standard compliance.
