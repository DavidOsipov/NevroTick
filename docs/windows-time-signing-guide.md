# Signing the Windows time client: a practical MSI guide

**Revision:** 1.0  
**Research date:** 28 October 2026  
**Status:** Implementation guide; provider approval, production signing and clean-machine verification are **not completed**.  
**Applies to:** The Chrony-derived Rust time client for Windows 11, delivered as one complete MSI containing its services, ordinary-user UI and administrative helper.

## 1. The decision and the next action

The recommended free public-signing route is to **apply to SignPath Foundation for the project**. Start at [the Foundation website](https://signpath.org/), read its [current conditions](https://signpath.org/terms.html), and use its [application page](https://signpath.org/apply.html). Approval is discretionary; being open source does not by itself secure signing access. [S1–S3]

The immediate action is an eligibility inquiry with an accurate description of the project and its Chrony provenance. We should resolve that question early, while developing a working, documented preview. The project is still at the planning stage, and the Foundation's published conditions include an existing release in the intended distribution form. Ask what evidence it accepts for a first signed release; do not publish an unsafe installer merely to satisfy a checkbox. [S2]

The architecture already covered the strategy in section 19.5, and left actual acceptance as **OPEN-11**. This document supplies the operational steps. It does not claim that we have obtained a certificate, submitted an application, installed a signing integration or closed any release gate.

The distribution remains **one MSI for the complete application**. Signing the executable payloads inside that MSI is part of producing the package; it does not introduce another installer for the user.

## 2. What “getting the certificate” means here

A public code-signing certificate identifies a publisher and contains a public key. Signing requires the corresponding private key. Downloading a public `.cer` file would not provide that capability.

SignPath Foundation's model gives an approved project access to a controlled signing service. Its signing keys are held in SignPath's hardware security infrastructure. The Foundation is the certificate's publisher; this is not a general-purpose personal certificate issued in the maintainer's name or a private `.pfx` file to put in GitHub Secrets. Confirm the exact displayed publisher and project metadata during onboarding. [S1, S2]

For this project, the desired result is:

1. An approved project and signing policy at the provider.
2. A trusted build workflow associated with the correct repository.
3. A narrowly scoped ability to submit the intended release artifacts.
4. A designated human who approves release signing.
5. Downloadable, timestamped, signed payloads and the complete MSI.

The signing provider's token is still sensitive even when it cannot export the private key. Keep it out of agent workspaces, pull-request jobs, ordinary test machines and build logs. The release process must prevent a token holder from substituting another repository, workflow, artifact or publisher identity.

## 3. Eligibility questions specific to this project

The published conditions are labelled **Draft**. They require eligible open-source, maintained, documented, released software and impose source-ownership and release-control requirements. Re-read them before applying. [S2]

Ask how the provider classifies our Chrony-derived Rust translation: its modified-upstream rules refer to upstream signed builds and a visible fork. Acceptance must be confirmed rather than inferred. [S2]

Provider acceptance and copyright/license compatibility are separate decisions. The architecture retains the **GPL-2.0-only compatibility gate** for selected Chrony-derived source. It has not admitted the held Rustls/AWS-LC/ring provider graphs, chosen a substitute merely for signing convenience, or granted permission to remove notices. Before a production application, prepare an accurate source and dependency inventory with the license review results for the actual distributable graph.

Use this preparation table. It is our proposed application dossier, not a claim about the exact fields currently displayed in the provider's form.

| Prepare | What the project should supply |
| --- | --- |
| Project identity | Final product name, public repository and maintainer contact; no invented legal entity |
| Functionality | Windows NTP/NTS client, privileged clock broker, separate network engine, ordinary-user UI; no kernel driver planned |
| Source provenance | Exact Chrony revision and files used, port ledger, modifications, notices and corresponding-source/build instructions |
| License position | Proposed project license and reviewed obligations of every shipped component; clearly identify unresolved items |
| Distribution | One complete MSI, architectures supported, payload inventory and installer authoring tool/version |
| Release evidence | Working documented preview and the release evidence the provider says it requires |
| Build origin | Repository, protected release workflow, toolchain/lockfile identities and artifact path |
| Ownership | Maintainers, reviewers and release approver; account protection and access-removal procedure |
| User disclosure | Installation privileges, clock-service ownership changes, privacy/network behavior and complete removal behavior |
| Signing policy | Intended payload coverage, approval procedure, publisher expectations and compromise response |

Our release policy requires MFA, protected changes and an explicit human signing decision. Agents can prepare a release candidate and evidence; they cannot approve their own release, acquire new signing permissions or bypass a provider check.

### A draft inquiry you can adapt

The following has **not** been sent:

> We are preparing an open-source Windows 11 time-synchronization client in Rust. It will support NTP and NTS and use a small privileged clock broker, a separate network engine and an ordinary-user desktop UI, distributed together in one MSI. No kernel driver is planned.
>
> The numerical implementation is expected to port selected Chrony client algorithms. We will preserve source provenance and applicable GPL-2.0-only obligations and review the complete dependency graph before distribution. The project is currently in planning and does not yet have a production release.
>
> Could you clarify whether this kind of maintained Rust port can qualify under your own-source/upstream rules, what evidence you require for its first signed MSI release, and how you would want us to handle bundled third-party code and binaries? We would also like to confirm the publisher identity users would see and the supported workflow for signing the owned executables and final MSI.
>
> We intend to use a protected GitHub-hosted release build, explicit human signing approval and a documented release policy. We can provide the repository, port ledger, license inventory and initial build artifacts when available.

Do not add provider attribution or claim that signing is approved until it actually is. Keep the provider's written response with the release-policy decision record.

## 4. The onboarding sequence

### Step 1 — Establish the repository and a working preview

Create the product repository, preserve the source provenance, document installation/removal and produce the bounded laboratory preview described by architecture task S0.6. Resolve the installer authoring tool's own licensing and eligibility; free application signing does not settle that separate question.

Prepare the dossier above. Ask the early eligibility questions before investing in a provider-specific pipeline. If a first-release requirement is unclear, record the provider's answer instead of treating an unpublished plan as an eligible released product.

### Step 2 — Submit the application yourself

Use [SignPath Foundation's application page](https://signpath.org/apply.html). The page currently embeds a JavaScript-hosted form. If you see only a heading, a blocked or failed form script is one possible explanation; check that page's script permissions or try a normal browser profile. There is no need to disable browser protection globally. The form's current fields were not inspected in this research. [S3, S4]

Submission represents the maintainer and may require agreeing to terms, granting repository integration access or supplying account information. This guide prepares that work; it does not submit or accept anything on your behalf.

### Step 3 — Agree on the exact signing scope

After acceptance, confirm the approved publisher, project identifier, production signing policy, artifact configuration, allowed build origin, timestamping policy and approvers.

Configure the project's product name and consistent per-build product version on every owned binary, enforced through SignPath's metadata restrictions. After acceptance, publish its required **Code signing policy** on project and release/download pages, with roles, provider attribution and accurate privacy information. [S2, S5]

For our Rust executables, add the necessary version resources during the build and inspect the emitted PE files. Qualify a correct metadata fixture and rejection of missing/mismatched metadata. Preserve upstream metadata; do not assume every other field, such as each file's original name, should be identical. Write truthful network disclosures covering configured time servers and any admitted certificate/revocation traffic. Do not copy a privacy promise the implementation cannot meet.

Inventory the UI, network engine, clock broker, administrative helper, installer custom-action binaries and any shipped DLLs. State which are owned builds, which retain an upstream signature and which would otherwise be unsigned. An MSI signature does not automatically establish an acceptable publisher signature for each nested executable. The provider supports Authenticode signing for MSI and PE artifacts, but the actual nested artifact configuration must match our package. [S5]

If a bundled component cannot meet the project's agreed signing policy, resolve it explicitly. Do not re-sign third-party files as our own merely to make an inventory checker green. Preserve permitted upstream signatures and test the resulting files under the supported Windows security policies.

### Step 4 — Configure the protected build and submission workflow

SignPath's GitHub integration verifies workflow origin and consumes an artifact first uploaded to GitHub Actions. Its Foundation OSS path currently requires all workflow jobs leading to the signing request to run on **GitHub-hosted agents**. The integration uses provider project/policy identifiers and a submission credential; it does not require exporting the certificate's private key. [S6]

Our proposed controls are:

- Pin reviewed actions to immutable commits and record their origin. Evaluate GitHub's workflow dependency lockfiles separately while that feature remains in preview.
- Build from a protected, reviewed commit with an exact dependency graph and admitted toolchain.
- Keep untrusted PR compilation and analysis separate from credential-bearing release operations.
- Give the submission identity only the signing policy it needs, with approval enforced at the provider and repository release boundary.
- Bind the artifact to its repository, commit, workflow run, build configuration, payload manifest and hashes.
- Preserve the provider request identifier and signed-artifact receipt.

The Foundation's hosted-runner rule needs coordination with our separate Windows laboratory. Do not run privileged clock experiments on a shared build host or conceal a self-hosted build as a hosted artifact. A viable design is a provider-accepted hosted build, separate validation of its identified artifacts in a disposable Windows lab, and a signing request for the approved artifact lineage. Confirm how the provider supports that sequence. A rebuilt artifact is not automatically identical to the tested one; equivalence or repeated validation needs evidence.

### Step 5 — Sign in the agreed order

Use one of two provider-approved sequences:

| Sequence | Required handling |
| --- | --- |
| Sign payloads, then build and sign MSI | Use the returned signed payload bytes when assembling the MSI. Sign the final MSI afterward. |
| Provider-managed nested signing | Qualify the artifact configuration that signs/repackages nested payloads and then signs the enclosing MSI. Verify the returned complete artifact. |

Apply SHA-256 digest policy and a trusted timestamp as supported by the approved service. Preserve the resulting timestamp evidence. A timestamp helps a verifier assess a signature against its signing time; it is not permission to ignore revocation, a broken chain, a different publisher or Windows verification failures. Microsoft's SignTool documentation describes digest and timestamp verification options. [S7]

Never publish the unsigned input by mistake. Never edit an executable, MSI table or embedded file after its relevant signature has been created. Any such transformation requires the appropriate rebuild/re-sign/re-verify sequence.

## 5. Verification before publishing

Use the admitted Windows SDK's SignTool on the actual returned files. Example commands below are **verification recipes, not executed results**; substitute the real file names and run from a trusted tool environment.

```powershell
signtool verify /pa /all /v /tw .\release\Product.msi
if ($LASTEXITCODE -ne 0) { throw "MSI signature verification failed or warned" }

signtool verify /pa /all /v /tw .\release\Product.ClockBroker.exe
if ($LASTEXITCODE -ne 0) { throw "Broker signature verification failed or warned" }
```

Repeat the payload check for every file required by the signed inventory. `/pa` selects Authenticode verification policy, `/all` verifies all signatures present and `/tw` warns about a missing timestamp. Treat warnings as a release failure requiring investigation. Verifying the MSI does **not** recursively prove every embedded executable's signature. [S7]

The release checker must also enforce our intended identity and package manifest: a successful generic trust check alone can accept a valid signature from an unintended publisher. Design publisher/certificate renewal handling explicitly rather than permanently pinning a single leaf certificate without a rotation plan.

The release evidence must establish:

1. **Identity:** the expected provider/project publisher, code-signing use, chain, timestamp and applicable revocation checks.
2. **Completeness:** all required payloads, helper/custom-action files and the final MSI are covered; embedded payload hashes match the signed inventory.
3. **Origin:** exact reviewed commit, dependency lockfile, native inputs, toolchain, build and signing request.
4. **Windows behavior:** install, ordinary-user launch, service operation, update, repair, rejection of unsupported downgrade and uninstall on a clean supported Windows 11 machine.
5. **Negative controls:** a tampered package, substituted payload, unexpected publisher and unapproved version are rejected for the intended reason; a valid neighboring candidate succeeds.
6. **Publication identity:** checksums and any release metadata describe the final signed bytes. Every published version attaches its prepared, validated SBOM assets alongside the complete MSI and final checksums. Reconcile Rust, native/generated/resources and actual MSI payload inventory against the exact version/source candidate and final signed hashes before publication; see [the repository SBOM tooling and handoff](../TOOLING.md#release-sbom-tooling). Attestations and an SBOM supplement that record; they do not replace Authenticode.

Clock-dependent verification is particularly relevant for this application. Qualify the architecture's extreme-date, certificate-time and revocation behavior using controlled fixtures and disposable machines. Do not weaken TLS or executable-signature validation to get through a test with a wrong system clock. TUF is still a separate, unadopted update proposal; this guide does not add an automatic updater or accept a new time-bootstrap exception.

## 6. What the other routes do and do not solve

| Route | Suitable role for this project | Limitation |
| --- | --- | --- |
| **SignPath Foundation** | Preferred free OSS public-signing application for the complete MSI and approved owned payloads | Project acceptance and the exact provenance/payload policy remain open |
| **Microsoft Store with a complete MSIX** | Possible future complete-product channel; accepted MSIX packages receive Store signing | Requires package/service/capability qualification and Store acceptance; not a reason to split service and UI installers |
| **Microsoft Store with MSI/EXE** | Possible listing/distribution route for an already signed complete installer | The developer must supply trusted signatures; it does not provide free signing for an unsigned MSI |
| **Azure Artifact Signing Public Trust** | Paid hosted-signing alternative when the applicant is eligible | Not a free OSS entitlement; identity/geography restrictions apply |
| **Commercial public code-signing provider** | Fallback if cost, identity verification and provider policies are acceptable | Obtain current eligibility and price directly; no free availability assumed |
| **Self-signed laboratory certificate** | Controlled installer/privilege/lifecycle experiments | Not public publisher trust and not a production distribution solution |

Microsoft's Store MSI/EXE rules require the installer and included PE files to be signed with certificates chaining to an accepted authority; Store MSIX signing is a different path. A provider's willingness to sign an MSI containing an unsigned upstream DLL would not itself satisfy that Store requirement. [S8]

Azure Artifact Signing currently requires a paid Azure subscription. Public Trust individual applicants must be located in the **United States or Canada**; its organization eligibility list is broader but does not include Georgia. Thus an individual applying from Georgia should not plan on this as an available route under the current rules. Choosing an Azure resource region does not change applicant eligibility, and Private Trust is not a substitute for public Windows trust. Recheck eligibility if circumstances or the service change. [S9, S10]

A valid signature also does **not guarantee the absence of SmartScreen prompts**. Reputation and other Windows protections remain separate considerations. Microsoft's current guidance says EV certificates no longer automatically bypass SmartScreen reputation checks. Paying for EV solely to promise warning-free installation would be an unsound plan. [S11]

## 7. Development can proceed before public signing is approved

The architecture already permits an isolated, test-signed S0.6 MSI to qualify installation, privilege separation and removal behavior. Trust that test certificate only inside the disposable laboratory VM; record the substituted trust store and certificate in the evidence. Restore or destroy the VM afterward.

That experiment does not satisfy the public publisher-trust portion of CHECK-18 or CHECK-23. Ordinary users must not be instructed to install our development root certificate. Do not disable Windows signature, reputation or application-control protections to make a public package install.

If no acceptable free provider approves the project, retain the public signed-release gate while continuing bounded development. The choices then are an eligible sponsor/provider arrangement, a funded public-signing service, or a separately qualified complete Store package. The need for a single understandable installation remains unchanged.

## 8. Completion checklist and ownership

| Item | Owner | Present state |
| --- | --- | --- |
| Chrony/source and exact dependency-license position | Maintainer and qualified license reviewer | Open; preserve architecture gates |
| Provider treatment of this Rust port and first release | Maintainer and provider | Not confirmed |
| Public signing application and acceptance | Maintainer | Not submitted by this work |
| Publisher identity and permitted payload coverage | Release owner and provider | Open |
| Enforced binary metadata and public signing policy | Build/release owner | Not implemented or published |
| Protected build, submission and human approval | Release/security owner | Planned; not implemented |
| Valid/tampered/wrong-publisher signing checks | Release verifier | Not executed |
| Complete signed MSI lifecycle on clean Windows | Installer/Windows verifier | Not executed |
| Renewal, revocation, access removal and recovery | Release/security owner | Must be written and rehearsed before public release |

**OPEN-11 closes only with an approved usable route and recorded identity/policy evidence.** Public release additionally requires the actual signed-artifact and lifecycle checks. A provider account, a screenshot of a certificate or this guide alone is insufficient.

## 9. Sources and related documents

Primary provider and Microsoft pages were checked on 28 October 2026. Eligibility and service rules can change; recheck before application and release. The project-specific pipeline controls and acceptance criteria above are proposed engineering policy, not statements that a provider has accepted our implementation.

- **S1:** [SignPath Foundation overview](https://signpath.org/).
- **S2:** [SignPath Foundation conditions and draft code of conduct](https://signpath.org/terms.html).
- **S3:** [SignPath Foundation application](https://signpath.org/apply.html).
- **S4:** [Source of the embedded application form](https://github.com/SignPath/fdn-website/blob/main/docs/apply.md).
- **S5:** [SignPath artifact-configuration reference](https://docs.signpath.io/artifact-configuration/reference).
- **S6:** [SignPath trusted GitHub build integration](https://docs.signpath.io/trusted-build-systems/github).
- **S7:** [Microsoft SignTool reference](https://learn.microsoft.com/en-us/windows/win32/seccrypto/signtool).
- **S8:** [Microsoft Store MSI/EXE package requirements](https://learn.microsoft.com/en-us/windows/apps/publish/publish-your-app/msi/app-package-requirements).
- **S9:** [Azure Artifact Signing setup and eligibility](https://learn.microsoft.com/en-us/azure/artifact-signing/quickstart).
- **S10:** [Azure Artifact Signing FAQ](https://learn.microsoft.com/en-us/azure/artifact-signing/faq).
- **S11:** [Microsoft guidance on SmartScreen reputation](https://learn.microsoft.com/en-us/windows/apps/package-and-deploy/smartscreen-reputation).

Companion documents: [architecture revision 1.4](windows-time-architecture.md), [development-environment hardening](windows-time-development-hardening.md), and [update-system assessment](windows-time-update-assessment.md). This guide is independently readable; the architecture remains authoritative for product scope, authority boundaries and unresolved design gates.
