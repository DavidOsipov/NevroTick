---
name: "qa-regression-selection"
description: "Choose the smallest justified NevroTick regression set while accounting for indirect consumers, targets, features and recovery paths."
---

# Select affected regression checks

Read [the testing contract](../CONTRACT.md).

Inspect the actual patch and producer → state → consumer paths, including serializers, constructors, OS adapters, generated code, fixtures, features/dependencies and installer/UI consumers. Existing test names alone are not an impact map.

Map each changed guarantee and plausible regression to an existing check or a specific new case. Retain the original failure and valid earlier controls; include failure/recovery seams and cross-worker interactions. Select host/core checks separately from required Windows configurations.

After confirming a root cause, use [the bounded variant-search method](../references/defect-variants.md) when related occurrences could affect this regression scope. Calibrate on the known defect, inspect each candidate and record unsearched areas rather than treating pattern matches as findings.

Record why excluded checks are unaffected, relying on current source/configuration rather than history or convenience. Unknown impact needs a broader applicable selection or an explicit unresolved gap; no invented all-features or whole-suite mandate.

Verify discovery and actual execution for the selected tests, preserve doctests where in scope and invalidate prior evidence only for relevant changed state. Use [Rust techniques](../references/rust-techniques.md) for target/feature/runner selection.
