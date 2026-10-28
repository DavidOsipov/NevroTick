---
name: "qa-golden-fixture-parity"
description: "Compare NevroTick captured outputs or pinned reference trajectories with independent semantic facts and a deliberately tested comparator."
---

# Verify stable output and semantic parity

Read [the testing contract](../CONTRACT.md).

Freeze fixture rights/provenance, input bytes, producer revision, configuration and expected semantic facts. Useful goldens include protocol vectors, selection/correction trajectories, status/error projections and package inventories; pinned chrony output is differential evidence, not sole truth.

Decide normalization from the contract before comparison. Retain raw output/diff. Units, signs, source identity/order, trust/freshness, unknown fields, error semantics and generation values may be contractual; do not erase them to stabilize snapshots. Allow incidental variation only with a stated basis.

Challenge the comparator with a changed semantic sentinel that must fail and a permitted harmless variation that must pass. Compare actual generated output, not a fixture produced by the same production formatter used as the oracle.

Review baseline changes as semantic decisions rather than accepting a tool's update mode for GREEN. Report exactly which facts and representations agree, plus untested platforms/native effects. See [numerical/protocol truth](../references/numerical-protocol.md).
