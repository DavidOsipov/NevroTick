---
name: "qa-package-artifacts"
description: "Verify emitted NevroTick binaries, complete MSIX contents, metadata, provenance and Store-delivered consistency independently of source configuration."
---

# Inspect built Windows artifacts

Read [the testing contract](../CONTRACT.md).

For field inventory, provenance, source storage and generation/readback details,
use [maintain-windows-metadata](../../implementation/maintain-windows-metadata/SKILL.md).
Keep this workflow as the final artifact/native qualification owner; authored
records and metadata parsing alone do not prove a complete or trusted package.

Identify frozen source/configuration/toolchain/dependency/feature inputs, actual final binary/package hashes. Inspect emitted PE/runtime protections, architecture, versions, manifests/resources, DLL loading dependencies, expected component/service inventory.

The selected target is one complete hardened MSIX with UI/engine/broker/administration helper for David's Store trial, under [architecture §19.4](../../../../docs/windows-time-architecture.md#194-future-complete-msix-gate) and [OD-05](../../../../docs/nevrotick-owner-decisions.md#7-od-05--signing-and-release-ownership). Compare actual payloads/installed results; source selection proves no package or qualified implementation. Check package/catalog and project-specific identity, tamper controls, clean-machine operation, one product lifecycle and source/SBOM/provenance obligations. Use [MSIX plan §3](../../../../docs/plans/windows-msix-store.md#3-package-trust-and-the-privileged-helper) for integrity, immutable code and the actual per-role DLL/mitigation profile.

Follow [the existing stages in §7](../../../../docs/plans/windows-msix-store.md#7-bounded-continuation-and-existing-acceptance-owners). First identify and inspect the complete local candidate; test-signed Windows mechanics run only when prerequisites and execution holds permit. Record which catalog/CIG/activation cases need genuine Store bytes. David then tries the prepared submission with services; prior Store approval or signing is not a prerequisite for its preparation. After Store processing, independently inspect the actual delivered package and rerun affected trust, load, runtime and servicing cases before broad release. Keep local, submitted and Store-delivered hashes distinct. Certification and local schema success are not application passes. Current Core priority, Windows stop, separate fuzz hold and storage pause remain.

Use the [configured release-inventory workflow](../references/configured-tooling.md#coverage-mutation-and-release-inventory)
and existing `scripts/generate-sbom.py` wrapper for its current Windows x64
policy target, matching build features and fresh owned external outputs. Other
targets require explicit policy/wrapper admission. Require real
manifests, lockfile and cached dependencies; a fixture inventory is not a product
SBOM. Reconcile its admitted frozen Rust dependency inventory against the actual
native libraries, generated/build inputs and complete package payload. The wrapper does not
establish full-product completeness, final-byte/signature parity or publication
authority by itself.

For the selected Store route, verify the exact installed package/publisher/release and content trust, not just a broad Microsoft/Store signer class. Challenge tampered, wrong-project, copied-out and unapproved old-release artifacts beside the intended healthy package. Common catalog trust does not establish independently embedded signatures on every extracted PE or helper authority. CIG admission must match the actual selected mode and runtime image closure, not the Cargo crate list.

MSI/SignPath/OpenPGP workflows are retained fallback/reference only if David revisits delivery after rejection or concrete technical evidence. They are not parallel deliverables or MSIX prerequisites. If later selected, Authenticode needs actual publisher/chain/timestamp evidence; OpenPGP needs the trusted full key fingerprint, exact final MSI signature, signed manifest and qualified installed-code trust. Absent Authenticode is not passed, and no profile waives native gates or David's release approval.

For update/repair/Reset/removal and fault/restoration claims use [Windows evidence](../references/windows-evidence.md). Test-signed experiments qualify only declared mechanics; they do not establish Store signer-category admission. Store processing/signing can change bytes; record reproducibility, submitted lineage and final authenticated identity separately.

Reject mismatched/tampered artifacts beside legitimate neighboring package controls. TUF/custom-updater research remains unadopted. Report exact inspected outputs, stage-specific gaps and unavailable Windows/security-profile evidence; a Store refusal returns the delivery decision to David rather than silently selecting MSI.
