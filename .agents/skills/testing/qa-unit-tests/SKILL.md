---
name: "qa-unit-tests"
description: "Write or review focused Rust unit tests for NevroTick numerical, domain and state logic using independent expectations and controlled effects."
---

# Test pure Rust behavior

Read [the testing contract](../CONTRACT.md).

Locate narrow production unit/actual invariant. Use existing Cargo/libtest conventions/fixtures; inspect actual package/features/test discovery. No new mocking/property framework if small deterministic table suffices.

Derive expectations independently. Cover meaningful valid case, suspected failure, adjacent boundaries, applicable forbidden state/effects. Time arithmetic: explicit UTC/monotonic/active-awake clocks, exact units, analytic values; production algorithm must not calculate own oracle.

Use legitimate constructors. Deliberate test-only invariant bypass must enable no real consumer bypass. Assert returned values/stable error semantics/state after failure. Restore owned state; inspect leaked tasks/resources before fixture teardown.

Test the function's semantic postconditions and failure atomicity using valid
typed inputs, not only its entry validator. For garde-backed construction, call
the real constructor with invalid and valid neighboring records and then exercise
its real consumer. Route visibility/alternate-construction claims through
[contract QA](../qa-contract-tests/SKILL.md); a test that manually calls validation
before an otherwise unchecked operation cannot prove unavoidable validation.

Repair: intended product RED before smallest authorized change; GREEN with unchanged expectations. Report exact named execution; listing/compilation insufficient. See [Rust techniques](../references/rust-techniques.md) for runner limits, [numerical truth](../references/numerical-protocol.md) for core cases; unit evidence does not qualify Windows FFI.
