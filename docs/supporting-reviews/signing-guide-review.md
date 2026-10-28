# Independent review of the MSI signing guide

## Review identity and scope

- Reviewed: `deliverables/windows-time-signing-guide.md`, revision 1.0, first-pass SHA-256 `471e9a60b6663c6d65c29095d969c8f20a17f3777259d42ea7151ea22c5aa7e3`.
- Canonical architecture: revision 1.4, SHA-256 `a9bd1432ecb2d04e55b77e1a270eb1a135027d9f4a7988d58d9299367a6ca075`; sections 19.5, 20/CHECK-18 and CHECK-23, 21/S0.1 and S0.6, and OPEN-11 checked.
- Reviewer: independent delegated reviewer, not the guide's author. First-pass record retained here; no deliverable or architecture edits made by this reviewer.
- Stage assessed: completion of a practical standalone onboarding/signing guide. Early eligibility inquiry is a separately permitted next step; production signing and release are not assessed as ready.
- Guidance read: pinned website-guidance `AGENTS.md`, `planning/plan-independent-review/SKILL.md`, security `CONTRACT.md`, security amendment applicability, and testing `CONTRACT.md`. Website-specific implementation tools and gates were not imported into this documentation review.
- Working-tree identity: the task directory is not a Git repository (`git status --short` returned that error); file hashes identify the reviewed material. Both deliverable hashes remained unchanged at the final read before this review was written.
- Budget: four primary-source lookup batches; no Context7 use, browser/login, installation, target execution, application submission, signing, or network mutation.

## Verdict

**Revise for completion of the practical onboarding guide.** One bounded provider-setup omission should be made explicit. It does not block preparing the early eligibility inquiry or the architecture's isolated S0.6 work. No critical/high finding, unsafe command, architecture conflict, false completion claim, or basis to reject SignPath as a candidate was found in the inspected scope.

## First-pass finding

### SIGN-01 — Medium: name the mandatory metadata and public-policy setup tasks

**Requirement / stage / location:** Provider onboarding and artifact admission; guide sections 3 and 4, especially lines 46–55, 71, and 87–106 in the reviewed hash.

**Problem:** The guide refers generally to project metadata, a signing policy, and artifact configuration, but its actionable sequence omits two concrete conditions on the provider's own conditions page:

1. Signed binaries must have the project's product-name attributes and a consistent product version per build, enforced through artifact-configuration metadata restrictions.
2. The project must publish a code-signing policy on its home page and download/release pages, including the required attribution, named roles/members, and accurate privacy information.

The current instruction to withhold attribution until approval is correct, but there is no corresponding instruction to publish the required policy once the arrangement is accepted. A maintainer could follow the sequence, prepare the named inventory and approval workflow, and still omit required PE metadata or public information. This is a documentation completeness issue, not evidence of a defective implementation or an unfulfilled obligation by an existing approved project.

**Evidence:** [SignPath Foundation conditions](https://signpath.org/terms.html), sections “Conditions for the website / repository” and “SignPath configuration requirements” (retrieved 28 October 2026; web reference `turn291view1`, source lines 82–103). [Artifact-configuration reference](https://docs.signpath.io/artifact-configuration/reference), “File metadata restrictions” (web reference `turn292view1`, lines 887–895), identifies `product-name` and `product-version` restrictions for PE files. The provider conditions are mandatory even though some checks may not yet be automatically enforced.

**Smallest correction / owner:** Guide author adds a short prerequisite or two bullets in Step 3, with a corresponding checklist entry: configure and inspect the product metadata on each owned executable/DLL and enforce it in the approved artifact configuration; after acceptance, publish the provider-required code-signing policy on the project and release/download surfaces with truthful roles, attribution and network/privacy disclosures. Preserve upstream metadata/signatures and the existing no-false-attribution instruction. Do not imply that every metadata field must be identical across different files or invent an already-configured XML policy.

**Verification of correction:** Read the revised guide for those named tasks and their order. The future implementation must independently demonstrate both a valid metadata case and rejection of missing/mismatched required metadata; this review did not execute either case.

## Checked claims with no finding

| Topic | Review conclusion and evidence |
| --- | --- |
| Architecture and scope | One complete MSI remains the user-facing installer. Payload signing is internal release work. The GPL-2.0-only graph gate, open provider decision, S0.6 test-signing exception, CHECK-18/CHECK-23 separation, and non-adoption of TUF are preserved. |
| Foundation identity and key custody | The overview describes HSM-held signing keys; the conditions identify the Foundation as publisher. The guide correctly avoids promising a personal certificate or exportable private PFX. [Overview](https://signpath.org/) (`turn291view2`); [conditions](https://signpath.org/terms.html) (`turn288view0`). |
| Eligibility, Chrony derivation and first release | The conditions require an existing release and impose modified-upstream constraints. Asking the provider how they apply to this planned Rust translation is a sound unresolved decision; the guide claims neither acceptance nor automatic exclusion. [Conditions](https://signpath.org/terms.html) (`turn288view0`). |
| Hosted build and approval | The GitHub integration requires prior workflow-artifact upload and GitHub-hosted jobs leading to OSS signing. The guide accurately preserves that boundary and asks how an external disposable Windows lab fits the approved lineage. Manual release approval is retained. [GitHub integration](https://docs.signpath.io/trusted-build-systems/github) (`turn288view1`); [conditions](https://signpath.org/terms.html) (`turn291view1`). |
| Nested payloads and MSI | The reference supports PE/MSI signing and composite deep signing. The guide correctly separates nested payload verification from the enclosing MSI and requires final-byte inventory binding. It does not promise permission to re-sign arbitrary upstream binaries. [Artifact reference](https://docs.signpath.io/artifact-configuration/reference) (`turn288view3`). |
| SignTool recipes | `/pa`, `/all`, `/v`, and `/tw` are documented options. Microsoft documents exit codes 0 for success, 1 for failure, and 2 for warnings, so the proposed nonzero-exit failure check is consistent with the stated policy. Generic trust remains explicitly separate from expected-publisher enforcement. Commands were inspected, not executed. [SignTool](https://learn.microsoft.com/en-us/windows/win32/seccrypto/signtool) (`turn291view0`). |
| Store MSI versus MSIX | Store MSI/EXE submissions require the installer and included PE files to have accepted CA-chain signatures. MSIX Store signing is a distinct route; the guide does not offer it as free signing for the present MSI or split the product. [Store requirements](https://learn.microsoft.com/en-us/windows/apps/publish/publish-your-app/msi/app-package-requirements) (`turn289view2`). |
| Azure eligibility | Current official sources require a paid subscription and restrict individual Public Trust applicants to the US/Canada; Georgia is absent from the organization list too. Resource-region selection does not establish applicant eligibility. [Quickstart](https://learn.microsoft.com/en-us/azure/artifact-signing/quickstart) (`turn289view0`); [FAQ](https://learn.microsoft.com/en-us/azure/artifact-signing/faq) (`turn289view1`). |
| SmartScreen | Microsoft's current source supports the guide's claims that a valid signature need not eliminate prompts and EV no longer gives an automatic reputation bypass. [SmartScreen guidance](https://learn.microsoft.com/en-us/windows/apps/package-and-deploy/smartscreen-reputation) (`turn289view3`). |
| Completion and authority | The guide repeatedly labels preparation, example commands and future evidence honestly. It does not claim a submitted application, approved provider, signed artifact, successful lifecycle test or closed release gate. |

## Limits and remaining work

The application page was retrieved and displayed a heading without the form. Fetching its cited GitHub source and the raw variant returned `DisabledError`; this reviewer therefore did not independently verify the JavaScript embedding implementation. That retrieval limit is not evidence that the guide's statement is wrong. The live form, current fields, account permissions and provider decision were not inspected.

No Windows runner, signing account or product artifact was used. This is documentary and primary-source review only; it cannot satisfy implementation, publisher-trust, clean-machine, revocation, Smart App Control, hostile-input or lifecycle gates. No additional style-only findings were raised.

**Next:** Root incorporates or records an evidence-backed disposition of SIGN-01, then rechecks the changed passage. The early eligibility inquiry can be prepared; sending it remains a separate authorized action. Production signing remains dependent on provider acceptance and the architecture's existing release gates.

## Correction recheck — 28 October 2026

Re-read the corrected Step 3 and completion checklist at signing-guide SHA-256 `b598d5ca10331bece2fc9051cfa21c71584bc06c9e82f78a8b9931a1ca1a6587`. The canonical architecture hash remains `a9bd1432ecb2d04e55b77e1a270eb1a135027d9f4a7988d58d9299367a6ca075`.

**SIGN-01: addressed.** Step 3 now names the missing tasks, explains inspection of emitted Rust PE version resources, plans valid and missing/mismatched metadata cases, preserves upstream metadata, and puts public-policy publication after provider acceptance. Network disclosures are expressly required to match actual behavior. The checklist records this work as not implemented or published.

**Rechecked verdict: ready for delivery of the standalone guide within the reviewed scope.** The original finding above is unchanged. No further finding arose from the correction. This is a documentation recheck only; the original provider/form and Windows-execution limits remain, and no production signing or release gate is closed. No additional web research or target execution was performed.
