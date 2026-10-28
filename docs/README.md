# Rust Windows Time Client — latest documentation bundle

**Prepared for:** David Osipov  
**Snapshot date:** 28 October 2026  
**Current architecture:** RWTC-ARCH-001, revision 1.4

This checkout contains the five principal Markdown documents, three supporting independent TUF reviews, and two development/release guides. The architecture controls product decisions; navigation and supporting material do not establish implementation or compliance. This reading guide is maintained with the working checkout rather than treated as an unchanged frozen-bundle copy.

## Start here

| Reading order | Document | Scope and status |
| --- | --- | --- |
| 1 | [windows-time-architecture.md](windows-time-architecture.md) | Canonical architecture and creation specification, revision 1.4. Includes the threat model, Windows authority boundaries, NTP/NTS recovery, dependency policy, packaging, UX and verification plan. |
| 2 | [windows-time-diagrams.md](windows-time-diagrams.md) | Editable Mermaid views aligned with revision 1.4. The architecture document remains authoritative. |
| 3 | [asvs-5.0.0-applicability.md](asvs-5.0.0-applicability.md) | Revision 1.4 applicability and planned-verification profile covering all 345 ASVS 5.0.0 requirements. It is not evidence of implemented compliance. |
| 4 | [windows-time-review.md](windows-time-review.md) | Final independent review of revision 1.3, reconciliation into revision 1.4, and correction rechecks. Its older reviewed-baseline label is intentional. |
| 5 | [windows-time-update-assessment.md](windows-time-update-assessment.md) | Latest assessment of TUF-authenticated updates, including candidate topology, time semantics, dependency constraints and Windows handoff. No TUF implementation or updater architecture has been adopted. |

## Development and release guides

| Document | Read when | Status and limits |
| --- | --- | --- |
| [windows-time-development-hardening.md](windows-time-development-hardening.md) | Selecting or implementing toolchain, lint, dependency, CI, analyzer, test and native-artifact gates | Proposed enforcement specification and qualification plan; it does not establish installed tools, adopted settings or passing application checks. |
| [windows-time-signing-guide.md](windows-time-signing-guide.md) | Planning provider eligibility/onboarding, protected signing scope/order, publisher verification or a complete MSI release | Operational implementation guide; provider approval, production signing and clean-machine verification remain incomplete. |

## Supporting independent reviews

The following are retained research records. The consolidated update assessment above controls the current user-facing conclusions; these records are not competing normative architecture specifications.

- [tuf-protocol-review.md](supporting-reviews/tuf-protocol-review.md): protocol, time assurance and recovery.
- [tuf-dependency-review.md](supporting-reviews/tuf-dependency-review.md): Rust clients, crypto graph, persistence, publisher tooling and evidence limits.
- [tuf-windows-review.md](supporting-reviews/tuf-windows-review.md): component authority, protected staging, MSI handoff, lifecycle and applicability changes.

Some supporting reports preserve original workspace paths, independent-first-pass labels and research identifiers for provenance. Their inclusion does not imply runtime validation. No application build, Windows installation, clock manipulation or conformance testing was performed for these assessments.

## Format and integrity

All documents are UTF-8 Markdown. Mermaid diagrams remain editable fenced source and can be viewed in a Mermaid-capable Markdown renderer. Keep the extracted directory together so links between the principal documents work.

When supplied with a frozen bundle, `SHA256SUMS.txt` provides file-integrity comparison, not a digital signature or independent publisher authentication. A mutable checkout or updated reading guide needs its own current matching manifest before claiming byte-for-byte bundle verification.

The older PDF and CSV exports, frozen baselines, candidate working copies, helper scripts and intermediate review data are excluded from this latest-document bundle.
