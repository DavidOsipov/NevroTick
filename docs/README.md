# Rust Windows Time Client — latest documentation bundle

**Prepared for:** David Osipov  
**Snapshot date:** 28 October 2026  
**Canonical architecture:** RWTC-ARCH-001, revision 1.4

This bundle contains eight principal Markdown documents and seven supporting independent reviews. The new feature plan translates the architecture into basic user capabilities, screens, defaults, acceptance cases and delivery slices. All seven previously supplied principal documents remain byte-for-byte unchanged. Proposed behavior and planned checks are not implementation evidence.

## Reading guide

| Order | Document | Scope and status |
| --- | --- | --- |
| 1 | [windows-time-features.md](windows-time-features.md) | V1 product feature plan: ten capabilities, main journeys, authority, recovery, twenty acceptance scenarios and staged delivery. |
| 2 | [windows-time-architecture.md](windows-time-architecture.md) | Canonical creation specification, revision 1.4: technical contracts, threat model, NTP/NTS, Windows authority and release gates. |
| 3 | [windows-time-diagrams.md](windows-time-diagrams.md) | Editable Mermaid architecture views aligned with revision 1.4. |
| 4 | [windows-time-development-hardening.md](windows-time-development-hardening.md) | Strict Rust/Windows agentic development policy, CI, auditors, tool qualification and evidence requirements. Proposed, not applied. |
| 5 | [asvs-5.0.0-applicability.md](asvs-5.0.0-applicability.md) | All 345 ASVS 5.0.0 requirements mapped to proposed applicability/verification; not completed compliance. |
| 6 | [windows-time-signing-guide.md](windows-time-signing-guide.md) | Practical MSI signing application, artifact workflow and verification guide; provider acceptance remains open. |
| 7 | [windows-time-update-assessment.md](windows-time-update-assessment.md) | TUF/update research and constraints; no automatic updater or TUF architecture has been adopted. |
| 8 | [windows-time-review.md](windows-time-review.md) | Independent architecture review and revision 1.4 reconciliation. Its earlier reviewed-baseline label is intentional. |

The architecture controls technical/security semantics. The feature plan supplies proposed presentation and product scope; unresolved provider, numerical, Windows, licensing and signing decisions retain their gates. The complete application remains one MSI.

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

`SHA256SUMS.txt` covers every Markdown file, including this reading guide. It supports byte-integrity comparison, not publisher authentication or a digital signature.

Older PDF/CSV exports, intermediate candidates, helper scripts, credentials and executable tools are excluded. No application build, Windows installation, clock change, scanner execution, signing request or conformance test was performed to create the feature-plan addition.
