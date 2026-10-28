---
name: "qa-unit-tests"
description: "Write or review focused Rust unit tests for NevroTick numerical, domain and state logic using independent expectations and controlled effects."
---

# Test pure Rust behavior

Read [the testing contract](../CONTRACT.md).

Locate the narrow production unit and its real invariant. Use existing Cargo/libtest conventions and fixtures; inspect actual package/features and test discovery. Avoid a new mocking/property framework when a small deterministic table suffices.

Construct expected results independently. Cover a meaningful valid case, the suspected failure, adjacent boundaries and forbidden state/effects as applicable. For time arithmetic, use explicit UTC/monotonic/active-awake clocks, exact units and analytically chosen values; do not invoke the production algorithm to calculate its own oracle.

Use legitimate constructors. A test-only bypass may challenge an internal invariant deliberately, but must not make a real consumer bypass possible. Assertions should observe returned values, stable error semantics and state after failure. Restore owned state and inspect leaked tasks/resources before fixture teardown.

For repairs, demonstrate the intended product RED before the smallest authorized change, then GREEN with unchanged expectations. Report exact named execution, not only listing/compilation. See [Rust techniques](../references/rust-techniques.md) for runner limits and [numerical truth](../references/numerical-protocol.md) for core cases; unit evidence does not qualify Windows FFI.
