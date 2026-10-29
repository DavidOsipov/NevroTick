---
name: "qa-package-artifacts"
description: "Verify emitted NevroTick binaries, MSI contents, metadata, provenance and delivered-package consistency independently of source configuration."
---

# Inspect built Windows artifacts

Read [the testing contract](../CONTRACT.md).

Identify frozen source/configuration/toolchain/dependency/feature inputs, actual final binary/package hashes. Inspect emitted PE/runtime protections, architecture, versions, manifests/resources, DLL loading dependencies, expected component/service inventory.

V1: one complete signed per-machine MSI with UI/engine/broker/administration helper. Compare actual payloads/installed results; installer source alone insufficient. Check publisher/signature/tamper controls, clean-machine operation, one product version/lifecycle, corresponding in-scope source/SBOM/provenance obligations.

Use the [configured release-inventory workflow](../references/configured-tooling.md#coverage-mutation-and-release-inventory)
and existing `scripts/generate-sbom.py` wrapper for its current Windows x64
policy target, matching build features and fresh owned external outputs. Other
targets require explicit policy/wrapper admission. Require real
manifests, lockfile and cached dependencies; a fixture inventory is not a product
SBOM. Reconcile its admitted frozen Rust dependency inventory against the actual
native libraries, generated/build inputs and MSI payload. The wrapper does not
establish full-product completeness, final-byte/signature parity or publication
authority by itself.

For update/repair/removal and fault/restoration claims use [Windows evidence](../references/windows-evidence.md). Test-signed experiments qualify only their declared mechanics; production trust is a separate gate. Signing can change final bytes, so unsigned reproducibility and signed identity are distinct.

Reject mismatched/tampered artifact beside legitimate neighboring package control. Future MSIX/TUF design remains research until adopted; exclude from baseline acceptance. Report exact inspected outputs/unavailable Windows/security-profile evidence.
