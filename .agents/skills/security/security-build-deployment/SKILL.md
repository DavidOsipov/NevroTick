---
name: "security-build-deployment"
description: "Implement or review NevroTick Cargo and CI trust, final Windows binary protections, Store MSIX delivery authority and reversible complete-package lifecycle."
---

# Preserve trust through the complete Windows package

Formal/Kani proof work and pending proof admission/metadata repairs are deferred
until after V1 under [OD-10 §14.1](../../../../docs/nevrotick-owner-decisions.md#141-first-release-deferral--6-november-2026).
The tool capabilities below create no pre-V1 proof execution or acceptance gate.

Read [the security contract](../CONTRACT.md), architecture sections 11, 15, 18–20 and applicable release instructions. Route dependencies to [dependency-updates](../../environment/dependency-updates/SKILL.md). The selected target is one complete hardened MSIX for David's Store submission attempt, including services, under [architecture §19.4](../../../../docs/windows-time-architecture.md#194-future-complete-msix-gate) and [OD-05](../../../../docs/nevrotick-owner-decisions.md#7-od-05--signing-and-release-ownership). Direction is selected; supported provisioning, implementation, Store acceptance and native qualification remain separate unresolved results.

Use [MSIX plan §3](../../../../docs/plans/windows-msix-store.md#3-package-trust-and-the-privileged-helper) for explicit package integrity, immutable code, exact Store-compatible CIG and per-role mitigations. Follow [§7's stages](../../../../docs/plans/windows-msix-store.md#7-bounded-continuation-and-existing-acceptance-owners): source/design preparation → locally built/test-signed mechanics when prerequisites permit → David's Store trial → actual delivered-byte qualification before broad release. Do not require prior Store approval or a Store signature to prepare the first submission. Local trust substitutions prove no Store signer-category admission; keep those final-artifact cases unexecuted until that artifact exists. Current Core priority, Windows execution stop, separate fuzz hold and storage pause remain.

Apply [architecture §19.7](../../../../docs/windows-time-architecture.md#197-signatures-for-the-complete-shipped-inventory) and [signing guide §4.1](../../../../docs/windows-time-signing-guide.md#41-required-signatures-for-the-shipped-inventory): the package and every supported signable shipped file need verified signatures, including nonbinary formats. Preserve approved upstream signatures, finish metadata first and sign/verify inside-out; independently reconcile actual delivered bytes and required inner-signature preservation. Missing signing capability under the declared admitted profile blocks release, not source preparation. SignPath can be a candidate for inner files on MSIX; it is not selected. MSI delivery needs a later owner decision, and the unsigned-MSI/OpenPGP exception is superseded. Preserve native lifecycle/security evidence and David's exact-candidate authority.

For build policy read proposed [development-hardening guide](../../../../docs/windows-time-development-hardening.md); for provider onboarding, signing scope/order and publisher verification read [signing guide](../../../../docs/windows-time-signing-guide.md). Research/proposed steps prove no adopted enforcement, provider approval, production signing or clean-machine verification. Submission, signing and publication retain task authorization/protected approval owner.

For the later owner-approved interim self-signed MSI option, use
[signing guide §7.1](../../../../docs/windows-time-signing-guide.md#71-interim-self-signed-msi-profile).
Signing capability is profile-specific: a public CA certificate is not required
for that named profile. All-file coverage remains; preserve approved-key checks,
clean-machine chain distrust and actual helper/loader/channel results separately.
Do not report public trust, suppress trust errors broadly, install customer roots
or weaken required mitigations. This is neither lab-only permission nor an
automatic change from Store MSIX to MSI.

For artifact admission, extraction or consumption, apply
[the canonical content-trust rule](../../../../docs/windows-time-architecture.md#196-authenticated-artifact-content-and-consumer-trust)
and [signing guide section 5.2](../../../../docs/windows-time-signing-guide.md#52-pe-certificate-padding-and-exact-final-bytes)
directly, independently of provider choice. Trace excluded application data and
substituted components to actual command/destination/load sinks. Record the
applicable signature source, strict-policy result, approved full-file custody and
harmless consumer controls in existing W11-W10/S6.4 evidence; no valid carrier
signature alone supplies authority for unauthenticated content or a later payload.

For profile, CPU, LTO or PGO changes, read the
[Windows build guidance](../../implementation/optimize-rust-performance/references/measurement.md#build-optimization-and-windows-x64).
Preserve the admitted CPU floor, arithmetic/fault policy, symbols and complete
mitigation flags. Explicit setting preparation establishes no measured gain.
Enforce OD-07's **x86-64-v3** Windows minimum and OD-05's maximum-compatible
hardening requirement. The target config and exact policy controls now include
strict CET markers, restricted static-import search and disabled Rust switch
jump tables in the pinned bundled-LLD lane; use hardening §6.2–6.3 for exact
flags and concrete exclusions. Every applicable final-product protection needs
implementation and qualification. Missing evidence is open work, not permission
to call it optional or remove a flag; a weaker supported release profile needs
the owner's decision. Hardware-conditional enforcement does not silently expand
the v3 floor. Preserve the unrelated native/fuzz/storage and post-V1 proof holds.
Admit PGO training/profile custody separately; Rust LTO admits no cross-language
LTO or BOLT PE lane. Rebuilds invalidate affected artifact/signature evidence:
reinspect the final PE and bind package/signing records to its exact bytes.

For packaged event-provider resources/channels or product Firewall/WFP policy, read [Windows host integration](../references/windows-host-integration.md): immutable payload/installed-resource receipts, actual service access/traffic observations, ownership-aware rollback/repair/removal.

Contributor translation files use the [hostile-catalog chain](../../../../docs/plans/ui-localization.md#contributor-bytes-to-package-and-runtime).
All JSON/CBOR remains potentially malicious, even English/generated/signed bytes.
A future data-only lane must use an independently protected validator/contract,
fixed admitted tools and pinned bounded inputs, enforced offline credential-free
isolation, fixed output paths and atomic complete publication. Candidate scripts,
configuration, artifacts and executable caches cannot replace that checker or
run in a signing/secret job. Re-admit final resources at runtime; provenance alone
proves no parser safety or truthful wording. This adds no workflow or execution.

1. Trace source/config/lockfile/features/toolchain → host build scripts/proc macros/native tools → linked Windows binaries → complete local/submitted MSIX → actual Store-delivered artifact → installed services/UI/helper. Keep source, submitted and delivered identities/qualification distinct; Store processing can change bytes. `Cargo.lock` stays versionable; `target/` excluded. Lockfile reproducibility proves no origin trust or sandboxing.
2. Inspect build-time execution and target/host distinction. Isolate untrusted builds; signing credentials/privileged runners must never execute untrusted PR source or poisoned artifacts. Constrain CI token permissions; inspect third-party action origins/immutable revisions and cache/artifact trust. Skill discussion alone grants no workflow additions or dependency pins.
3. Verify final package-integrity declarations, immutable executable paths, selected PE/process mitigations, actual DLL/import/load paths, protected data ACLs and effective service tokens. Bind package/catalog identity and approved release to helper authorization; an unrelated Store-signed image remains unauthorized even if CIG admits its signer class. Rust crates linked into a binary are not separate runtime DLLs. Compiler flags/manifests are inputs, not effective proof; match the actual package, launch path and native dependencies. Linux builds cannot qualify these boundaries.
4. Keep one product/version/Installed apps entry and one UI/engine/broker/helper lifecycle. No service prerequisite, UI-only package, user provisioning script or hidden orphan services. Downloaders yielding separately managed products violate architecture's single-product contract.
5. Establish supported pre-start provisioning and bounded lifecycle transactions: effective identities/rights/ACLs/IPC/backend, protected durable journal, original W32Time state, attributable quiescence and single ownership handover. No first-UI provisioning, broad LocalSystem substitution or self-granted service configuration rights. Unavailable required recovery blocks takeover before mutation. Repair/Reset and last-user removal cannot erase the only recovery mechanism. If the retained MSI route is later selected, its custom-action conditions, paired rollback, properties/transforms and disabled-rollback controls additionally apply.
6. Inject failures at meaningful reached stages/reboot interruptions. Separate package publication, installed state, journal progress and acknowledgement. Package replacement/service stop is not proof of recovery or uninstall. Old-version removal cannot restore W32Time under its successor; uninstall respects later administrator/GPO changes. Neither NTS outage nor packaging convenience permits automatic weak-authentication clock-controller fallback.
7. Store release trust needs exact package/publisher/release identity, catalog/content and actual loaded-image evidence, protected submission/release authority and incident/version policy. Checksums alone provide no independent release authentication. Reconcile local-to-Store changes and final lifecycle/mitigation evidence before broad release. Store certification is not an application security pass; acceptance cannot waive a technical blocker. Retained TUF research adopts no bespoke privileged updater.

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
the submitted and actual Store-delivered package inventories; generation alone
does not qualify complete-package provenance.

Coordinate [qa-package-artifacts](../../testing/qa-package-artifacts/SKILL.md), [qa-fault-recovery](../../testing/qa-fault-recovery/SKILL.md) and [qa-final-qualification](../../testing/qa-final-qualification/SKILL.md) for changed claims. Report source/artifact identity, executed checks, lifecycle/restoration observations, signing status and release blockers separately. Readiness grants no publication/install permission.
