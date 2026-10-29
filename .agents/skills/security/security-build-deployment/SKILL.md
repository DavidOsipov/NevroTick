---
name: "security-build-deployment"
description: "Implement or review NevroTick Cargo and CI trust, final Windows binary protections, signing authority, complete MSI packaging and reversible install/update/repair/uninstall lifecycle."
---

# Preserve trust through the complete Windows package

Read [the security contract](../CONTRACT.md), architecture sections 11, 15, 18–20 and applicable release instructions. Route dependencies to [dependency-updates](../../environment/dependency-updates/SKILL.md). Current distribution: one complete per-machine MSI authenticated under the explicitly selected [architecture §19.5 release profile](../../../../docs/windows-time-architecture.md#195-free-signing-plan) and [OD-05](../../../../docs/nevrotick-owner-decisions.md#7-od-05--signing-and-release-ownership). Future MSIX/TUF research remain separate unqualified gates.

Prefer qualified Authenticode; OD-05 permits an initial OpenPGP profile if a usable free route is unavailable. Verify that profile's exact-artifact signatures/manifest and installed-code trust gate. OpenPGP supplies neither Windows publisher trust nor automatic helper/IPC identity; absent Authenticode is not a passed check. Preserve all applicable native lifecycle/security evidence and David's exact-candidate release approval.

For build policy read proposed [development-hardening guide](../../../../docs/windows-time-development-hardening.md); for provider onboarding, signing scope/order and publisher verification read [signing guide](../../../../docs/windows-time-signing-guide.md). Research/proposed steps prove no adopted enforcement, provider approval, production signing or clean-machine verification. Submission, signing and publication retain task authorization/protected approval owner.

For profile, CPU, LTO or PGO changes, read the
[Windows build guidance](../../implementation/optimize-rust-performance/references/measurement.md#build-optimization-and-windows-x64).
Preserve the admitted CPU floor, arithmetic/fault policy, symbols and complete
mitigation flags. Explicit setting preparation establishes no measured gain.
Admit PGO training/profile custody separately; Rust LTO admits no cross-language
LTO or BOLT PE lane. Rebuilds invalidate affected artifact/signature evidence:
reinspect the final PE and bind package/signing records to its exact bytes.

For packaged event-provider resources/channels or product Firewall/WFP policy, read [Windows host integration](../references/windows-host-integration.md): immutable payload/installed-resource receipts, actual service access/traffic observations, ownership-aware rollback/repair/removal.

1. Trace source/config/lockfile/features/toolchain → host build scripts/proc macros/native tools → linked Windows binaries → complete MSI → profile-authenticated final artifact → installed services/UI/helper. Keep exact identities/qualification for delivered candidate. `Cargo.lock` stays versionable; `target/` excluded. Lockfile reproducibility proves no origin trust or sandboxing.
2. Inspect build-time execution and target/host distinction. Isolate untrusted builds; signing credentials/privileged runners must never execute untrusted PR source or poisoned artifacts. Constrain CI token permissions; inspect third-party action origins/immutable revisions and cache/artifact trust. Skill discussion alone grants no workflow additions or dependency pins.
3. Verify selected final PE mitigations, DLL/import/load paths, protected installation/data ACLs and actual service tokens. Compiler flags/manifests are inputs, not emitted/effective proof. Match project target, native runtime and installed package; Linux builds cannot qualify these boundaries.
4. Keep one product/version/Installed apps entry and one UI/engine/broker/helper lifecycle. No service prerequisite, UI-only package, user provisioning script or hidden orphan services. Downloaders yielding separately managed products violate architecture's single-product contract.
5. Bound installer transactions: preflight identities/ACLs/IPC/backend, protected durable journal, preserve original W32Time state, quiesce/cancel attributable transient work under current ownership, hand over once, restore only product-owned changes. Required rollback disabled blocks takeover before mutation. Custom actions need correct install/upgrade/repair/uninstall conditions and paired rollback; validate properties/transforms and exact successor ownership. Ordinary-user repair remains privileged input.
6. Inject failures at meaningful reached stages/reboot interruptions. Separate package publication, installed state, journal progress and acknowledgement. Upgrade must not briefly restore W32Time through old-product removal; uninstall respects later administrator/GPO changes. Neither NTS outage nor MSI convenience permits automatic weak-authentication clock-controller fallback.
7. Signing/release trust needs the selected profile's verified provider/publisher or trusted full OpenPGP release-key fingerprint, protected authorization, verification and incident/rotation policy. Checksums alone provide no independent release authentication. Proposed free-signing routes/green unsigned builds prove no signing acceptance. Implement no bespoke privileged auto-updater from retained TUF assessment.

For changed workflow/scanner/gate policy, select the existing helper tests,
workflow checker and pinned secret scanner through the
[configured-tool workflows](../../testing/references/configured-tooling.md).
Hosted CodeQL scanning is explicitly deferred: its prepared jobs perform no
extraction. Keep local MCP analysis separate from hosted workflow activation.
The final workflow gate accepts only explicit deferral with skipped CodeQL jobs
and successful mandatory workflow/secret checks; this supplies no SAST result.
For local database/query work, use
[CodeQL data-flow analysis](../../testing/references/codeql-data-flow.md) and
the build/security owner's current installation, restricted-tool and CLI-terms
admission in [TOOLING.md](../../../../TOOLING.md#repository-security-automation).
Inspect actual exposed capabilities and build-script/proc-macro execution before
extraction; local MCP availability supplies no product scan or Windows evidence.
Treat retained hosted Rust preparation as a separate runner lane, not local setup.

For a changed analysis or dependency-review guarantee, select Miri/fuzz/Loom/
Kani/Lockbud/cargo-vet through the guarded analysis entry point in the configured
guide. Preserve exact separate analysis-compiler pins and shipping Rust 1.99.1;
fixture controls do not qualify a product graph. Cargo-vet missing audits block:
never fabricate reviews/imports/exemptions or widen features to obtain success.

At artifact inventory time, use that guide's existing `scripts/generate-sbom.py`
wrapper for its current `x86_64-pc-windows-msvc` policy target, matching build
features and a new external output directory. Other targets require explicit
policy/wrapper admission before use.
Reconcile its frozen Rust inventory with native/build/generated components and
the final MSI; generation alone does not qualify complete-package provenance.

Coordinate [qa-package-artifacts](../../testing/qa-package-artifacts/SKILL.md), [qa-fault-recovery](../../testing/qa-fault-recovery/SKILL.md) and [qa-final-qualification](../../testing/qa-final-qualification/SKILL.md) for changed claims. Report source/artifact identity, executed checks, lifecycle/restoration observations, signing status and release blockers separately. Readiness grants no publication/install permission.
