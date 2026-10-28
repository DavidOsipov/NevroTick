# Rust Windows Time Client — latest documentation bundle

**Prepared for:** David Osipov  
**Snapshot date:** 28 October 2026  
**Current architecture:** RWTC-ARCH-001, revision 1.4

This bundle contains the latest five principal Markdown documents and the three supporting independent TUF reviews. The principal documents are byte-for-byte copies of their verified saved versions. Packaging does not introduce architectural decisions or change their implementation status.

## Start here

| Reading order | Document | Scope and status |
| --- | --- | --- |
| 1 | [windows-time-architecture.md](windows-time-architecture.md) | Canonical architecture and creation specification, revision 1.4. Includes the threat model, Windows authority boundaries, NTP/NTS recovery, dependency policy, packaging, UX and verification plan. |
| 2 | [windows-time-diagrams.md](windows-time-diagrams.md) | Editable Mermaid views aligned with revision 1.4. The architecture document remains authoritative. |
| 3 | [asvs-5.0.0-applicability.md](asvs-5.0.0-applicability.md) | Revision 1.4 applicability and planned-verification profile covering all 345 ASVS 5.0.0 requirements. It is not evidence of implemented compliance. |
| 4 | [windows-time-review.md](windows-time-review.md) | Final independent review of revision 1.3, reconciliation into revision 1.4, and correction rechecks. Its older reviewed-baseline label is intentional. |
| 5 | [windows-time-update-assessment.md](windows-time-update-assessment.md) | Latest assessment of TUF-authenticated updates, including candidate topology, time semantics, dependency constraints and Windows handoff. No TUF implementation or updater architecture has been adopted. |

## Supporting independent reviews

The following are retained research records. The consolidated update assessment above controls the current user-facing conclusions; these records are not competing normative architecture specifications.

- [tuf-protocol-review.md](supporting-reviews/tuf-protocol-review.md): protocol, time assurance and recovery.
- [tuf-dependency-review.md](supporting-reviews/tuf-dependency-review.md): Rust clients, crypto graph, persistence, publisher tooling and evidence limits.
- [tuf-windows-review.md](supporting-reviews/tuf-windows-review.md): component authority, protected staging, MSI handoff, lifecycle and applicability changes.

Some supporting reports preserve original workspace paths, independent-first-pass labels and research identifiers for provenance. Their inclusion does not imply runtime validation. No application build, Windows installation, clock manipulation or conformance testing was performed for these assessments.

## Format and integrity

All documents are UTF-8 Markdown. Mermaid diagrams remain editable fenced source and can be viewed in a Mermaid-capable Markdown renderer. Keep the extracted directory together so links between the principal documents work.

`SHA256SUMS.txt` records the SHA-256 digest of every Markdown file, including this reading guide. It provides integrity comparison for this bundle; it is not a digital signature or independent publisher authentication.

The older PDF and CSV exports, frozen baselines, candidate working copies, helper scripts and intermediate review data are excluded from this latest-document bundle.
