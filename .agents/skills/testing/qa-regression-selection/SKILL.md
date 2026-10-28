---
name: "qa-regression-selection"
description: "Choose the smallest justified NevroTick regression set while accounting for indirect consumers, targets, features and recovery paths."
---

# Select affected regression checks

Read [the testing contract](../CONTRACT.md).

Inspect actual patch/producer → state → consumer paths: serializers, constructors, OS adapters, generated code, fixtures, features/dependencies, installer/UI consumers. Test names alone are no impact map.

Map each changed guarantee/plausible regression to existing check/specific new case. Retain original failure/valid earlier controls; include failure/recovery seams/cross-worker interactions. Separate host/core checks/required Windows configurations.

After root-cause confirmation, use [the bounded variant-search method](../references/defect-variants.md) for potentially relevant related occurrences. Calibrate on known defect; inspect each candidate; record unsearched areas. Pattern matches are not findings.

Justify unaffected excluded checks from current source/configuration; history/convenience insufficient. Unknown impact needs broader applicable selection/explicit unresolved gap; no invented all-features/whole-suite mandate.

Verify discovery and actual execution for the selected tests, preserve doctests where in scope and invalidate prior evidence only for relevant changed state. Use [Rust techniques](../references/rust-techniques.md) for target/feature/runner selection.
