# NevroTick — documentation guide

**Prepared for:** David Osipov  
**Snapshot date:** 29 October 2026  
**Canonical architecture:** RWTC-ARCH-001, revision 1.4

The maintained set contains ten principal Markdown documents and seven supporting independent reviews. The owner-decision/research addition is a scoped amendment, not a repeated full-product qualification. The feature plan translates the architecture into basic user capabilities, screens, defaults, acceptance cases and delivery slices. The architecture and development-hardening guide include a 29 October 2026 amendment making upstream-code verification and stress testing explicit. Proposed behavior and planned checks are not implementation evidence.

**Upstream code is not presumed bug-free or vulnerability-free.** Chrony and other reused projects, libraries and tools require independent verification of the behavior NevroTick relies on. See [architecture section 20.1](windows-time-architecture.md#201-independent-numerical-truth) and [the test and stress matrix](windows-time-development-hardening.md#97-verification-of-upstream-code-and-stress-testing).

**Development baseline, 29 October 2026:** [the tooling record](../TOOLING.md) describes adopted bootstrap checks and the new repository security workflow separately from pending application/native qualification. The [planned dependency refresh](windows-time-architecture.md#186-runtime-dependency-register) retains the selected Rust stack and updates held AWS-LC/Schannel references without admitting those providers or installing application crates.

**Windows mitigation refinement, 29 October 2026:** [architecture section 15.4](windows-time-architecture.md#154-per-process-mitigation-profiles-and-startup-admission) defines process roles, activation limits, startup failure handling and the mitigation manifest; [hardening section 6.4](windows-time-development-hardening.md#64-per-process-mitigation-qualification) supplies the native qualification matrix. Existing CFG/ASLR/DEP configuration and bootstrap evidence are distinguished from pending application enforcement. Read these before adding process policies or treating every mitigation as a runtime switch.

**Current UI milestone:** [UI-M0, architecture §16.5](windows-time-architecture.md#165-restricted-first-ui-milestone-ui-m0), is a restricted native fixture-only prototype. D1 display/theme, D2 production service/policy-state and D3 full keyboard/Narrator/UIA work are explicitly **Deferred**, not prerequisites for UI-M0. Restore them at their specified integration/preview gates. Other workstreams and eventual product/release requirements remain unchanged; completing UI-M0 does not complete P0 toolkit qualification.

**Latest owner instructions:** [nevrotick-owner-decisions.md](nevrotick-owner-decisions.md) records accepted reuse/recovery/release choices and unresolved default-profile policy. [nevrotick-research-handoff.md](nevrotick-research-handoff.md) assigns the remaining investigations, including the supplied NTS directory and proposed operator candidates. Read these before interpreting older unconditional signing language in retained historical reviews.

## Reading guide

| Order | Document | Scope and status |
| --- | --- | --- |
| A | [nevrotick-owner-decisions.md](nevrotick-owner-decisions.md) | Owner instructions of 29 October; accepted choices, recommendations and unresolved policy distinguished. |
| B | [nevrotick-research-handoff.md](nevrotick-research-handoff.md) | Bounded future research, NTS operator candidates, evidence requirements and owner decision cards. |
| 1 | [windows-time-features.md](windows-time-features.md) | V1 product feature plan: ten capabilities, main journeys, authority, recovery, twenty acceptance scenarios and staged delivery. |
| 2 | [windows-time-architecture.md](windows-time-architecture.md) | Canonical creation specification, revision 1.4: technical contracts, threat model, NTP/NTS, Windows authority and release gates. |
| 3 | [windows-time-diagrams.md](windows-time-diagrams.md) | Editable Mermaid architecture views aligned with revision 1.4. |
| 4 | [windows-time-development-hardening.md](windows-time-development-hardening.md) | Rust/Windows development policy, CI, auditors and evidence requirements. See the current tooling record for adopted bootstrap checks; application/native qualification remains pending. |
| 5 | [asvs-5.0.0-applicability.md](asvs-5.0.0-applicability.md) | All 345 ASVS 5.0.0 requirements mapped to proposed applicability/verification; not completed compliance. |
| 6 | [windows-time-signing-guide.md](windows-time-signing-guide.md) | Practical MSI signing application, artifact workflow and verification guide; provider acceptance remains open. |
| 7 | [windows-time-update-assessment.md](windows-time-update-assessment.md) | TUF/update research and constraints; no automatic updater or TUF architecture has been adopted. |
| 8 | [windows-time-review.md](windows-time-review.md) | Independent architecture review and revision 1.4 reconciliation. Its earlier reviewed-baseline label is intentional. |

Accepted owner decisions govern their stated scope; recommendations and open questions remain proposals. The architecture controls the corresponding technical/security contracts. The feature plan supplies proposed presentation and product scope; unresolved provider, numerical, Windows, licensing and signing decisions retain their gates. The complete application remains one MSI. The preferred Authenticode route and owner-approved initial OpenPGP alternative have separate evidence and Windows compatibility claims; neither is qualified by this documentation.

## Supporting independent reviews

These preserve first-pass findings and correction checks, not competing specifications or claims of runtime qualification.

- [tuf-protocol-review.md](supporting-reviews/tuf-protocol-review.md): protocol, time assurance and recovery.
- [tuf-dependency-review.md](supporting-reviews/tuf-dependency-review.md): Rust clients, crypto graphs and evidence limits.
- [tuf-windows-review.md](supporting-reviews/tuf-windows-review.md): authority, staging, MSI handoff and lifecycle.
- [signing-guide-review.md](supporting-reviews/signing-guide-review.md): provider onboarding and signing-guide corrections.
- [development-compiler-review.md](supporting-reviews/development-compiler-review.md): compiler/lint/profile and Windows mitigation review.
- [development-evidence-review.md](supporting-reviews/development-evidence-review.md): CI trust, isolation, mutation contract and correction verification.
- [basic-features-review.md](supporting-reviews/basic-features-review.md): integrated product feature, UX, authority and acceptance review.

Some records retain workspace paths, initial hashes and source identifiers for provenance. Read correction addenda alongside the first-pass findings. Main documents control current recommendations; actual implementation/tool/native/signing qualification remains pending.

## Format and integrity

All documents are UTF-8 Markdown. Mermaid remains editable fenced source. Keep the extracted directory together for relative document links.

An exported bundle may supply `SHA256SUMS.txt` for byte-integrity comparison; this checkout currently has no root documentation manifest, so it establishes no bundle-integrity result. The design subdirectories maintain their own separate manifests. Checksums do not authenticate a publisher or replace a digital signature.

Older PDF/CSV exports, intermediate candidates, helper scripts, credentials and executable tools are excluded. No application build, Windows installation, clock change, scanner execution, signing request or conformance test was performed to create the feature-plan addition.
