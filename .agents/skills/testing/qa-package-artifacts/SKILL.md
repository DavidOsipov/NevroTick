---
name: "qa-package-artifacts"
description: "Verify emitted NevroTick binaries, MSI contents, metadata, provenance and delivered-package consistency independently of source configuration."
---

# Inspect built Windows artifacts

Read [the testing contract](../CONTRACT.md).

Identify frozen source/configuration/toolchain/dependency/feature inputs, actual final binary/package hashes. Inspect emitted PE/runtime protections, architecture, versions, manifests/resources, DLL loading dependencies, expected component/service inventory.

V1: one complete per-machine MSI with UI/engine/broker/administration helper, authenticated under the explicitly selected [architecture §19.5 release profile](../../../../docs/windows-time-architecture.md#195-free-signing-plan) and [OD-05](../../../../docs/nevrotick-owner-decisions.md#7-od-05--signing-and-release-ownership). Compare actual payloads/installed results; installer source alone insufficient. Check profile-specific signatures/tamper controls, clean-machine operation, one product version/lifecycle, corresponding in-scope source/SBOM/provenance obligations.

Preferred Authenticode requires actual publisher/chain/timestamp evidence. The owner-approved initial OpenPGP alternative requires the trusted full release-key fingerprint, exact-final-MSI detached signature, signed artifact/version/source/SBOM manifest and qualified installed-code trust. It supplies no Windows publisher trust or automatic helper/IPC identity; record absent Authenticode separately, never as passed. Neither profile waives applicable native/package gates or David's release approval.

For update/repair/removal and fault/restoration claims use [Windows evidence](../references/windows-evidence.md). Test-signed experiments qualify only their declared mechanics; production trust is a separate gate. Authenticode changes artifact bytes; detached OpenPGP signatures bind the exact final bytes. Record reproducibility and final authenticated identity separately.

Reject mismatched/tampered artifact beside legitimate neighboring package control. Future MSIX/TUF design remains research until adopted; exclude from baseline acceptance. Report exact inspected outputs/unavailable Windows/security-profile evidence.
