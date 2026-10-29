---
name: "qa-golden-fixture-parity"
description: "Compare NevroTick captured outputs or pinned reference trajectories with independent semantic facts and a deliberately tested comparator."
---

# Verify stable output and semantic parity

Read [the testing contract](../CONTRACT.md).

Freeze fixture rights/provenance, input bytes, producer revision, configuration, expected semantic facts. Useful goldens: protocol vectors, selection/correction trajectories, status/error projections, package inventories. Pinned chrony output: differential evidence, never sole truth.

Decide contract-derived normalization before comparison. Retain raw output/diff. Units/signs/source identity/order/trust/freshness/unknown fields/error semantics/generation values may be contractual; do not erase for stable snapshots. Incidental variation needs stated basis.

Challenge comparator: changed semantic sentinel must fail; permitted harmless variation must pass. Compare actual generated output; same-production-formatter fixture is no oracle.

When changing the driver or comparator, use [harness qualification](../references/harness-qualification.md) to bind exact input facts to actual calls and challenge wrong, missing or unexpected observations. A matching rejection category can hide the wrong input.

Review baseline changes as semantic decisions; no tool-update-mode GREEN. Report exact agreeing facts/representations, untested platforms/native effects. See [numerical/protocol truth](../references/numerical-protocol.md).
