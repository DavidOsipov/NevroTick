---
name: "qa-package-artifacts"
description: "Verify emitted NevroTick binaries, MSI contents, metadata, provenance and delivered-package consistency independently of source configuration."
---

# Inspect built Windows artifacts

Read [the testing contract](../CONTRACT.md).

Identify the frozen source/configuration/toolchain/dependency/feature inputs and actual final binary/package hashes. Inspect emitted PE/runtime protections, architecture, versions, manifests/resources, DLL loading dependencies and expected component/service inventory.

V1 delivers one complete signed per-machine MSI containing UI, engine, broker and administration helper. Compare actual package payloads and installed results, not only installer source. Check publisher/signature/tamper controls, clean-machine operation, one product version/lifecycle and corresponding source/SBOM/provenance obligations in scope.

For update/repair/removal and fault/restoration claims use [Windows evidence](../references/windows-evidence.md). Test-signed experiments qualify only their declared mechanics; production trust is a separate gate. Signing can change final bytes, so unsigned reproducibility and signed identity are distinct.

Reject a mismatched/tampered artifact with a legitimate neighboring package control. A future MSIX/TUF design is research until adopted; do not add it to baseline package acceptance. Report exact inspected outputs and unavailable Windows/security-profile evidence.
