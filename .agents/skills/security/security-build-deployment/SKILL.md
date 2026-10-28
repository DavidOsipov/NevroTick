---
name: "security-build-deployment"
description: "Implement or review NevroTick Cargo and CI trust, final Windows binary protections, signing authority, complete MSI packaging and reversible install/update/repair/uninstall lifecycle."
---

# Preserve trust through the complete Windows package

Read [the security contract](../CONTRACT.md), architecture sections 11, 15, 18–20 and applicable release instructions. Route dependencies to [dependency-updates](../../environment/dependency-updates/SKILL.md). Current distribution: one complete signed per-machine MSI; future MSIX/TUF research remain separate unqualified gates.

For build policy read proposed [development-hardening guide](../../../../docs/windows-time-development-hardening.md); for provider onboarding, signing scope/order and publisher verification read [signing guide](../../../../docs/windows-time-signing-guide.md). Research/proposed steps prove no adopted enforcement, provider approval, production signing or clean-machine verification. Submission, signing and publication retain task authorization/protected approval owner.

For packaged event-provider resources/channels or product Firewall/WFP policy, read [Windows host integration](../references/windows-host-integration.md): immutable payload/installed-resource receipts, actual service access/traffic observations, ownership-aware rollback/repair/removal.

1. Trace source/config/lockfile/features/toolchain → host build scripts/proc macros/native tools → linked Windows binaries → complete MSI → signed artifact → installed services/UI/helper. Keep exact identities/qualification for delivered candidate. `Cargo.lock` stays versionable; `target/` excluded. Lockfile reproducibility proves no origin trust or sandboxing.
2. Inspect build-time execution and target/host distinction. Isolate untrusted builds; signing credentials/privileged runners must never execute untrusted PR source or poisoned artifacts. Constrain CI token permissions; inspect third-party action origins/immutable revisions and cache/artifact trust. Skill discussion alone grants no workflow additions or dependency pins.
3. Verify selected final PE mitigations, DLL/import/load paths, protected installation/data ACLs and actual service tokens. Compiler flags/manifests are inputs, not emitted/effective proof. Match project target, native runtime and installed package; Linux builds cannot qualify these boundaries.
4. Keep one product/version/Installed apps entry and one UI/engine/broker/helper lifecycle. No service prerequisite, UI-only package, user provisioning script or hidden orphan services. Downloaders yielding separately managed products violate architecture's single-product contract.
5. Bound installer transactions: preflight identities/ACLs/IPC/backend, protected durable journal, preserve original W32Time state, quiesce/cancel attributable transient work under current ownership, hand over once, restore only product-owned changes. Required rollback disabled blocks takeover before mutation. Custom actions need correct install/upgrade/repair/uninstall conditions and paired rollback; validate properties/transforms and exact successor ownership. Ordinary-user repair remains privileged input.
6. Inject failures at meaningful reached stages/reboot interruptions. Separate package publication, installed state, journal progress and acknowledgement. Upgrade must not briefly restore W32Time through old-product removal; uninstall respects later administrator/GPO changes. Neither NTS outage nor MSI convenience permits automatic weak-authentication clock-controller fallback.
7. Signing/release trust needs actual approved provider/identity, protected authorization, verification and incident/rotation policy. Checksums alone provide no independent release authentication. Proposed free-signing routes/green unsigned builds prove no signing acceptance. Implement no bespoke privileged auto-updater from retained TUF assessment.

Coordinate [qa-package-artifacts](../../testing/qa-package-artifacts/SKILL.md), [qa-fault-recovery](../../testing/qa-fault-recovery/SKILL.md) and [qa-final-qualification](../../testing/qa-final-qualification/SKILL.md) for changed claims. Report source/artifact identity, executed checks, lifecycle/restoration observations, signing status and release blockers separately. Readiness grants no publication/install permission.
