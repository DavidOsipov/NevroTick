---
name: "qa-golden-fixture-parity"
description: "Compare NevroTick golden outputs or differential results from pinned references with independent semantic facts and a tested comparator."
---

# Verify stable output and semantic parity

Read [the testing contract](../CONTRACT.md).

Freeze fixture rights/provenance, input bytes, producer revision, configuration, expected semantic facts. Useful goldens: protocol vectors, selection/correction trajectories, status/error projections, package inventories. Pinned chrony output: differential evidence, never sole truth.

Differential comparison: declare shared input domain and intentional contract
differences before execution. Disagreements: use [failure triage](../qa-failure-triage/SKILL.md)
and [failure localization](../references/failure-localization.md): distinguish
product defect, reference defect, harness error, permitted difference and
unresolved cause before changing implementation or expectation. Retain raw
disagreement, bounded reproducible witness and healthy neighbor.

Decide contract-derived normalization before comparison. Retain raw output/diff. Units/signs/source identity/order/trust/freshness/unknown fields/error semantics/generation values may be contractual; do not erase for stable snapshots. Incidental variation needs stated basis.

Challenge comparator: changed semantic sentinel must fail; permitted harmless variation must pass. Compare actual generated output; same-production-formatter fixture is no oracle.

For complete sequences/bundles, compare cardinality and contractual order/metadata
as well as paired values. Challenge empty, matching-prefix and appended cases;
`zip` alone can hide missing or extra records. Use whole-byte equality where the
contract requires exact output; semantic normalization must retain required facts.

When changing the driver or comparator, use [harness qualification](../references/harness-qualification.md) to bind exact input facts to actual calls and challenge wrong, missing or unexpected observations. A matching rejection category can hide the wrong input.

Review baseline changes as semantic decisions; no tool-update-mode GREEN. Report exact agreeing facts/representations, untested platforms/native effects. See [numerical/protocol truth](../references/numerical-protocol.md).
