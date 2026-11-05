# Signing the Windows time client: a practical package guide

**Revision:** 1.0 with owner-decision, trust-lifecycle, selected-MSIX and release-operations amendments, 4 November 2026

**Research date:** 28 October 2026  
**Status:** Preparation guide; Store acceptance, production package signing and native qualification are **not completed**.

**Applies to:** The Chrony-derived Rust time client for Windows 11, with one complete hardened MSIX containing its services, ordinary-user UI, administrative helper and all required payloads selected for a first Microsoft Store submission. Earlier MSI signing research is retained for a possible later owner decision.

## 1. The decision and the next action

**Selected direction, amended 3 November 2026:** prepare **one complete hardened MSIX for Microsoft Store**, including the services and every required application payload. David will try the Store submission himself and reconsider the distribution decision if an actual rejection or concrete blocker requires it. [OD-05 section 7.1](nevrotick-owner-decisions.md#71-accepted-choices) supersedes the earlier MSI-first selection. A Store rejection does not automatically select a weaker package or a separate service installer.

Preparation can proceed without speculative Store preapproval. The [MSIX plan](plans/windows-msix-store.md) defines the package-integrity, Store-compatible code-integrity and role-specific protection requirements, capability justification and qualification sequence. The current Core priority, Windows execution stop, fuzzing, storage and UI holds still apply. This documentation change does not authorize a held native experiment or claim a working package.

David retains account, submission, publication and exact-candidate release decisions. Store certification and signing are external outcomes. Initial controlled submission precedes verification of the actual Store-signed artifact; broad release follows the required post-Store artifact and native checks. See [section 6.1](#61-store-msix-trust-and-publication-boundaries) and the plan's publication sequence.

The [release policy](release-policy.md) owns the current support statement,
proposed first-stable baseline, exact release-status records and incident
procedures. Its written preparation supplies no support approval or completed
account, access-removal, renewal or recovery rehearsal.

**Retained alternatives:** sections 2–5 preserve the SignPath Foundation research and MSI Authenticode workflow; section 7 preserves the previously accepted initial OpenPGP MSI profile. Neither is a prerequisite for the selected Store MSIX, a simultaneous v1 distribution promise or an automatic fallback. If David later returns to MSI, the Foundation publisher display and David's human-approver role remain recorded, and that route still needs its own eligibility, trust and native qualification. The Foundation's conditions and application remain [S1–S3]; any later provider contact is a separate authorized action.

**OPEN-11** remains open for actual selected-route acceptance and qualification. This guide does not claim that an account, certificate, signing integration, submitted package or release gate has been completed.

## 2. What “getting the certificate” means here

This section and sections 3–5 retain the MSI Authenticode alternative for use only if the owner later selects it. Store package signing has the separate trust boundary described in section 6.1.

A public code-signing certificate identifies a publisher and contains a public key. Signing requires the corresponding private key. Downloading a public `.cer` file would not provide that capability.

SignPath Foundation's model gives an approved project access to a controlled signing service. Its signing keys are held in SignPath's hardware security infrastructure. The Foundation is the certificate's publisher; this is not a general-purpose personal certificate issued in the maintainer's name or a private `.pfx` file to put in GitHub Secrets. Confirm the exact displayed publisher and project metadata during onboarding. [S1, S2]

If the owner later selects this MSI alternative, the desired result is:

1. An approved project and signing policy at the provider.
2. A trusted build workflow associated with the correct repository.
3. A narrowly scoped ability to submit the intended release artifacts.
4. David Osipov as the human who approves release signing.
5. Downloadable, timestamped, signed payloads and the complete MSI.

The signing provider's token is still sensitive even when it cannot export the private key. Keep it out of agent workspaces, pull-request jobs, ordinary test machines and build logs. The release process must prevent a token holder from substituting another repository, workflow, artifact or publisher identity.

## 3. Eligibility questions specific to this project

The published conditions are labelled **Draft**. They require eligible open-source, maintained, documented, released software and impose source-ownership and release-control requirements. Re-read them before applying. [S2]

Research the provider treatment of proprietary NevroTick artwork as well as our Chrony-derived Rust translation. No approval/rejection or brand-licence change is inferred. If later authorized, ask how the provider classifies the translation: its modified-upstream rules refer to upstream signed builds and a visible fork. Acceptance must be confirmed rather than inferred. [S2]

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

The following historical preparation draft has **not** been sent. Refresh its project-state and distribution statements before any later authorized inquiry:

> We are preparing an open-source Windows 11 time-synchronization client in Rust. It will support NTP and NTS and use a small privileged clock broker, a separate network engine and an ordinary-user desktop UI, distributed together in one MSI. No kernel driver is planned.
>
> The numerical implementation is expected to port selected Chrony client algorithms. We will preserve source provenance and applicable GPL-2.0-only obligations and review the complete dependency graph before distribution. The project is currently in planning and does not yet have a production release.
>
> Could you clarify whether this kind of maintained Rust port can qualify under your own-source/upstream rules, what evidence you require for its first signed MSI release, and how you would want us to handle bundled third-party code and binaries? We would also like to confirm the publisher identity users would see and the supported workflow for signing the owned executables and final MSI.
>
> We intend to use a protected GitHub-hosted release build, explicit human signing approval and a documented release policy. We can provide the repository, port ledger, license inventory and initial build artifacts when available.

Do not add provider attribution or claim that signing is approved until it actually is. Keep the provider's written response with the release-policy decision record.

## 4. The onboarding sequence

These steps apply to a later owner-selected SignPath/MSI route. They are not required before preparing or submitting the selected complete Store MSIX, and do not lift the current Windows execution stop.

### Step 1 — Establish the repository and a working preview

Use the existing [NevroTick repository](https://github.com/DavidOsipov/NevroTick), preserve the source provenance, document installation/removal and produce the bounded laboratory preview described by architecture task S0.6. Resolve the installer authoring tool's own licensing and eligibility; free application signing does not settle that separate question.

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

This sequence is for the retained MSI Authenticode profile. Section 7 covers the retained initial OpenPGP alternative if the owner later returns to MSI.

Use one of two provider-approved sequences:

| Sequence | Required handling |
| --- | --- |
| Sign payloads, then build and sign MSI | Use the returned signed payload bytes when assembling the MSI. Sign the final MSI afterward. |
| Provider-managed nested signing | Qualify the artifact configuration that signs/repackages nested payloads and then signs the enclosing MSI. Verify the returned complete artifact. |

Apply SHA-256 digest policy and a trusted timestamp as supported by the approved service. Preserve the resulting timestamp evidence. A timestamp helps a verifier assess a signature against its signing time; it is not permission to ignore revocation, a broken chain, a different publisher or Windows verification failures. Microsoft's SignTool documentation describes digest and timestamp verification options. [S7]

For an Authenticode release, never publish the unsigned input by mistake. The separately declared OpenPGP release uses its own final-artifact verification and must not be mislabeled as Authenticode-signed. Never edit an executable, MSI table or embedded file after its relevant signature has been created. Any such transformation requires the appropriate rebuild/re-sign/re-verify sequence.

## 5. Authenticode verification before publishing

Apply these checks to the retained MSI Authenticode profile. They are not the Store MSIX verification recipe. If the owner later selects the initial OpenPGP profile, record Authenticode qualification separately; do not suppress failures and call that a pass.

Use the admitted Windows SDK's SignTool on the actual returned files. Example commands below are **verification recipes, not executed results**; substitute the real file names and run from a trusted tool environment.

```powershell
signtool verify /pa /all /v /tw .\release\Product.msi
if ($LASTEXITCODE -ne 0) { throw "MSI signature verification failed or warned" }

signtool verify /pa /all /v /tw .\release\Product.ClockBroker.exe
if ($LASTEXITCODE -ne 0) { throw "Broker signature verification failed or warned" }
```

Repeat the payload check for every file required by the signed inventory. `/pa` selects Authenticode verification policy, `/all` verifies all signatures present and `/tw` warns about a missing timestamp. Treat warnings as a release failure requiring investigation. Verifying the MSI does **not** recursively prove every embedded executable's signature. [S7]

The release checker must also enforce our intended identity and package manifest: a successful generic trust check alone can accept a valid signature from an unintended publisher. The accepted Foundation publisher display is not sufficient NevroTick project identity: qualify rejection of an unrelated, legitimately Foundation-signed artifact alongside acceptance of the approved NevroTick artifact. Bind admission to the approved project/artifact lineage and protected manifest, not display text or editable version resources alone. Design publisher/certificate renewal handling explicitly rather than permanently pinning a single leaf certificate without a rotation plan. [S2]

The release evidence must establish:

1. **Identity:** the expected provider/project publisher, code-signing use, chain, timestamp and applicable revocation checks.
2. **Completeness:** all required payloads, helper/custom-action files and the final MSI are covered; embedded payload hashes match the signed inventory.
3. **Origin:** exact reviewed commit, dependency lockfile, native inputs, toolchain, build and signing request.
4. **Windows behavior:** install, ordinary-user launch, service operation, update, repair, rejection of unsupported downgrade and uninstall on a clean installation of each claimed [client/Server profile](../tasks/windows11-integration.md#432-two-family-windows-qualification-matrix--5-november-2026). This retained MSI route needs its own applicability and native evidence if later selected.
5. **Negative controls:** a tampered package, substituted payload, unexpected publisher and unapproved version are rejected for the intended reason; a valid neighboring candidate succeeds.
6. **Publication identity:** checksums and any release metadata describe the final signed bytes. Every published version attaches its prepared, validated SBOM assets alongside the complete MSI and final checksums. Reconcile Rust, native/generated/resources and actual MSI payload inventory against the exact version/source candidate and final signed hashes before publication; see [the repository SBOM tooling and handoff](../TOOLING.md#release-sbom-tooling). Attestations and an SBOM supplement that record; they do not replace Authenticode.

Clock-dependent verification is particularly relevant for this application. Qualify the architecture's extreme-date, certificate-time and revocation behavior using controlled fixtures and disposable machines. Do not weaken TLS or executable-signature validation to get through a test with a wrong system clock. TUF is still a separate, unadopted update proposal; this guide does not add an automatic updater or accept a new time-bootstrap exception.

### 5.1 Wrong-clock and offline activation matrix

This matrix applies to the selected Store MSIX and to any MSI profile the owner
later selects; the profile-specific verifier and trust evidence stay distinct.

CHECK-9/18/23 must cover the code needed to reach recovery, including signature
checks before elevated-helper activation. This is an availability qualification
question, not evidence that Windows necessarily rejects every wrong-date launch.
Timestamping can preserve Authenticode verifiability after the signer certificate
expires; the selected chain, timestamp and revocation policy still governs. [S16]

| Operation | Required conditions and observations |
| --- | --- |
| Installed startup and helper activation | Grossly incorrect past/future date, offline state and absent/stale trust caches; identify whether approved code can start, refusal reason and supported recovery route |
| Initial complete-package installation | Selected MSIX, or MSI if later selected: date before signing or key creation and far-future date; observe package and payload admission, without relying on recovery code that is not yet installed |
| Update and repair | Unavailable trust infrastructure, stale caches and interrupted bounded verification; preserve installed-code identity and the prior usable product where authorized |
| Uninstall and W32Time restoration | Required recovery code remains callable or an explicit supported administrator remedy exists; no signature failure silently authorizes an unverified helper |

For each row and selected release profile, record the actual verifier/version,
calling process and any delegated Windows execution owner, trust inputs/time
basis, permitted retrieval, finite native-work budget, timeout/completion custody,
refusal behavior and recovery route. A deadline on the awaiting UI task does not
prove a native verification operation stopped. The TLS provider's execution and
egress qualification does not automatically cover the UI, helper or package verifier.

When using `WinVerifyTrust`, freeze the action and flags as part of that policy.
`WTD_REVOKE_NONE` alone does not prohibit network retrieval for code-signature
verification; Microsoft separately specifies `WTD_CACHE_ONLY_URL_RETRIEVAL`.
Qualify the required cache-only or admitted online mode, including stale/missing
cache behavior. Selecting cache-only operation does not prove fresh revocation
information, and this amendment does not approve disabling required checks. [S17]

## 6. What the other routes do and do not solve

| Route | Suitable role for this project | Limitation |
| --- | --- | --- |
| **SignPath Foundation** | Retained free OSS Authenticode candidate if the owner later selects the complete MSI route | Project acceptance and the exact provenance/payload policy remain open; not a Store-MSIX prerequisite |
| **Microsoft Store with a complete MSIX** | Selected first submission direction for the complete hardened product; accepted MSIX packages receive Store signing | Actual service/capability acceptance and package qualification remain open; no split service/UI installers or automatic fallback |
| **Microsoft Store with MSI/EXE** | Possible listing/distribution route for an already signed complete installer | The developer must supply trusted signatures; it does not provide free signing for an unsigned MSI |
| **Azure Artifact Signing Public Trust** | Paid hosted-signing alternative when the applicant is eligible | Not a free OSS entitlement; identity/geography restrictions apply |
| **Commercial public code-signing provider** | Possible later owner-selected alternative if cost, identity verification and provider policies are acceptable | Obtain current eligibility and price directly; no free availability or automatic fallback assumed |
| **Self-signed laboratory certificate** | Controlled installer/privilege/lifecycle experiments | Not public publisher trust and not a production distribution solution |

Microsoft's Store MSI/EXE rules require the installer and included PE files to be signed with certificates chaining to an accepted authority; Store MSIX signing is a different path. A provider's willingness to sign an MSI containing an unsigned upstream DLL would not itself satisfy that Store requirement. [S8]

Azure Artifact Signing currently requires a paid Azure subscription. Public Trust individual applicants must be located in the **United States or Canada**; its organization eligibility list is broader but does not include Georgia. Thus an individual applying from Georgia should not plan on this as an available route under the current rules. Choosing an Azure resource region does not change applicant eligibility, and Private Trust is not a substitute for public Windows trust. Recheck eligibility if circumstances or the service change. [S9, S10]

A valid signature also does **not guarantee the absence of SmartScreen prompts**. Reputation and other Windows protections remain separate considerations. Microsoft's current guidance says EV certificates no longer automatically bypass SmartScreen reputation checks. Paying for EV solely to promise warning-free installation would be an unsound plan. [S11]

### 6.1 Store MSIX trust and publication boundaries

The [MSIX/Store plan](plans/windows-msix-store.md), including the 3 November
owner amendment, defines the selected complete hardened MSIX direction. Source
and design preparation, later local candidate qualification, David's initial
controlled Store submission, and qualification of the actual Store-delivered
artifact are separate stages. Neither Store acceptance before preparation nor
a final Store signature before initial submission is required. No account,
accepted capability, signed artifact or executed Windows result is claimed.

[OD-05 §7.5](nevrotick-owner-decisions.md#75-optional-enhanced-build-provenance--4-november-2026)
makes enhanced build provenance optional: GitHub/Sigstore attestations,
independent reproducible builds and a public verification bundle are not release
blockers. [The provenance record](plans/release-provenance.md) explains their
possible use and the Store transformation. Required signing/trust, exact
artifact custody, SBOM/licence and installed-payload checks below remain distinct.

For an accepted Store MSIX, Microsoft supplies the package signature after
certification; the developer does not need a purchased public certificate or
signing token for that route. Current Individual and Company onboarding waive
their former registration fees through `storedeveloper.microsoft.com`. Select
the truthful account type and verify country/document eligibility; old portal
flows and broad market-availability statements are not sufficient evidence.
Store MSI/EXE submissions still require trusted installer and PE signatures;
the initial OpenPGP profile does not meet that requirement.

Microsoft documents a common catalog signature for the files in an MSIX app.
This supports a package-aware trust candidate and must not be dismissed as trust
only in an outer archive. It does not establish an independently usable embedded
Authenticode signature on every extracted EXE/DLL. The selected preparation
requires the following distinct controls; the plan owns their detailed contract.

- **Sealed executable content and runtime package integrity.** Declare
  `uap10:PackageIntegrity` with `uap10:Content Enforcement="on"` explicitly,
  using the supported namespace/schema and target OS. Keep all executable
  product payloads in the protected declared package/dependency graph. Do not
  enable executable mutable/external package content or load product code from
  writable data. An absent or empty
  integrity declaration is not enforcement evidence. Qualify actual service,
  UI and helper launch paths with tampered and intact packages and supported
  repair. See [plan section 3.1](plans/windows-msix-store.md#31-package-integrity-and-sealed-executable-content). [S19]
- **Store-compatible Code Integrity Guard (CIG).** Require the documented Store
  allowance where the role loads Store-signed code: the Exploit Protection
  names are `MicrosoftSignedOnly` and `AllowStoreSignedBinaries`. Strict
  Microsoft-only loading without that allowance is a different policy. Record
  the actual API/configuration representation, activation phase and readback
  for each real launch path, including SCM; a setter called after startup does
  not prove coverage of earlier module loads. Verify the complete actual
  import/DLL closure, including delayed and provider loads. See
  [plan section 3.2](plans/windows-msix-store.md#32-store-compatible-code-integrity-guard). [S20]
- **Role-specific protections.** Apply the strongest compatible supported and
  verified profile separately to the engine, broker, ordinary UI and elevated
  helper. Do not apply an unqualified blanket flag set or silently disable a
  required control to get a role to start. Record supported activation phase,
  readback and both refusal and valid-operation behavior. See
  [plan section 3.3](plans/windows-msix-store.md#33-strongest-compatible-process-protections)
  and [development hardening section 6.4](windows-time-development-hardening.md#64-per-process-mitigation-qualification).
- **Product authorization beyond signer class.** Bind the installed package's
  exact identity, approved release and payload to the helper/IPC contract.
  An unrelated valid Store package may satisfy a signer-class check; it must
  fail NevroTick role authorization beside the intended valid package. CIG
  and package signing do not grant NevroTick privileges.
- **Final distributed-artifact evidence.** Keep source/build, locally tested
  candidate and Store-delivered identities separately. After Store signing,
  reconcile the final package, catalog, manifest, payload hashes, SBOM and
  installed-file identity; read back the effective per-role protections and
  observed DLL closure on the actual delivered artifact. Use tampered,
  substituted, unsigned or unapproved DLL/package controls together with
  valid startup, cryptography, UI/helper, maintenance and recovery controls.
  Observe valid update/repair as well as refusal. The independent cases and
  CHECK-16/18/23/26 evidence belong to
  [plan section 3.4](plans/windows-msix-store.md#34-threats-and-independent-controls).
- **Compiled leap-data provenance.** The [selected calendar and policy](plans/leap-second-policy.md#3-knowledge-and-distribution-contract)
  ship as reviewed immutable generated data within that same authenticated
  complete release. Bind pinned inputs, generator identity, normalized revision
  and digest, and supported coverage to build and submitted/delivered payload
  records. `#h` or a standalone checksum is not release authentication. Do not
  add a separate downloader, runtime leap-file path, hot update or TUF authority.
  Package corruption is an integrity failure; valid unknown future coverage is
  a distinct policy state. Authorized update/rollback preserves protected floors,
  consumed correction authority and mandatory recovery.
- **Wrong-clock and offline availability.** Apply section 5.1 to first
  installation, pre-login startup, helper activation, maintenance and recovery.
  Signing and timestamping alone do not settle those paths.

Local test signing qualifies only the explicitly recorded laboratory trust
environment. It cannot substitute for final Store-artifact checks. Failure of
a required protection blocks the affected operation or authority; it does not
authorize weaker recovery code, expanded privilege or an automatic MSI route.

The Store Web Installer can offer website/GitHub-first acquisition of a published
free MSIX through the Store backend. It is an online delivery entry point, not a
standalone offline full installer or a separately managed prerequisite product.
Official `winget download` supports package/dependency acquisition and an offline
license workflow, but availability, licensing, rehosting and actual offline
behavior still need route-specific evidence. It is not a free signature service
for arbitrary newly built GitHub artifacts.

Private audience and package flights can support a later controlled release;
they do not waive certification or capability restrictions. Choose private
audience before first public publication if needed. A listing rollback to an
older package does not downgrade already updated installations: a corrective
higher-version release and compatible state recovery remain necessary. Preserve
David's exact-candidate release authority, truthful publisher/package identity,
and all [architecture §19.4 gates](windows-time-architecture.md#194-future-complete-msix-gate).
The [W11-W10 owner-selected MSIX assignment](../tasks/windows11-integration.md#48-owner-selected-hardened-msix--3-november-2026)
owns the preparation and later qualification work. David performs Store
submission and controls visibility/release; broad release requires the final
Store-artifact checks above. SignPath and OpenPGP MSI remain retained alternatives
only if he later revisits the distribution decision.

Use [release-policy §4](release-policy.md#4-distribution-containment-choose-the-actual-store-operation)
to distinguish initial private audience, later flights, gradual updates, halt and
availability controls. Record publication containment separately from installed
recovery and any publishing-authority incident. A corrective package must qualify
the affected state and actual delivery route; a Store signature does not prove
that malicious same-product publication through a compromised account is
independently rejected on the device. [Section 7](release-policy.md#7-compromised-publisher-repository-or-build-authority)
preserves that residual and the distinct MSA/Entra/application recovery routes.

## 7. Development and the accepted initial OpenPGP release profile

This section preserves the earlier MSI decision and its verification requirements. It applies only if David later selects that route; it is neither a prerequisite for Store MSIX nor the automatic response to Store rejection.

The architecture described an isolated, test-signed S0.6 MSI to qualify installation, privilege separation and removal behavior. Any later authorized laboratory package experiment must retain the current Windows execution stop until its owner-controlled gate is released. Trust a test certificate only inside the disposable laboratory VM; record the substituted trust store and certificate in the evidence. Restore or destroy the VM afterward.

That experiment does not satisfy the public publisher-trust portion of CHECK-18 or CHECK-23. Ordinary users must not be instructed to install our development root certificate. Do not disable Windows signature, reputation or application-control protections to make a public package install.

**Retained 29 October OD-05 policy:** if the owner returns to MSI and no usable free Authenticode route is available initially, David permits an MSI without project Authenticode signatures, accompanied by OpenPGP signatures. This earlier removal of a blanket Authenticode blocker does not override the 3 November MSIX selection, approve a paid service or authorize an automatic release. The package remains one complete product, and David approves the exact candidate.

Before this profile can ship, establish a trusted full release-key fingerprint and key-custody/expiry/revocation/rotation process. Produce a detached signature over the exact final MSI and an OpenPGP-signed manifest binding version, source/build identity, package hash, corresponding-source and SBOM artifacts. Authenticate the expected key, not just a matching name or short key ID. No key has been selected or generated by this guide. [S12]

Select the exact verifier and its key-policy enforcement separately from
cryptographic signature validity. The `gpgv` manual says its keyring is trusted
and expired/revoked keys are not checked. Therefore a successful `gpgv` exit plus
the expected fingerprint alone cannot establish this profile's expiry/revocation
requirements. Define protected trusted-key distribution, freshness evidence,
rotation/revocation authorization and offline/unknown-age behavior; qualify
revoked, expired, superseded and stale-keyring cases with a valid control. This
does not adopt `gpgv`, a runtime dependency or a time-check bypass. [S18]

Qualify valid, tampered, wrong-signer and mismatched-version/manifest cases. Preserve upstream signatures where present. Review the complete path from verified download and approved package to protected installed code, helper/IPC identity, DLL loading, repair and update lineage. Missing Authenticode must not become unconditional acceptance; a hash beside attacker-writable code is not an independent trust anchor. Research R-06 must resolve the signed-helper assumptions before qualifying this route.

OpenPGP verification supplies no Windows verified-publisher identity. Unknown unsigned code can be blocked by Smart App Control or organizational policy; test the installer and each payload under the supported security configurations, disclose restrictions, and do not recommend disabling protections or installing a development root. [S13, S14]

Keep CHECK-18 results separate: MSI Authenticode qualification remains incomplete if unavailable; the retained initial OpenPGP path requires its own exact-artifact, installed-file and native lifecycle evidence. All unrelated licence, privilege, mitigation, feature and applicable ASVS gates remain. Sidecar signatures/manifests are verification material, not another application installer. No embedded updater or runtime OpenPGP dependency is adopted. Microsoft's unsigned MSIX facility is a testing path, not the broad-distribution alternative. [S15]

See [NV-DEC-001 section 7](nevrotick-owner-decisions.md#7-od-05--signing-and-release-ownership) and [research R-06](nevrotick-research-handoff.md#8-r-06--signing-eligibility-and-initial-openpgp-releases). A later Authenticode transition must preserve product upgrade identity and reject unsupported downgrade of trust.

## 8. Completion checklist and ownership

| Item | Owner | Present state |
| --- | --- | --- |
| Chrony/source and exact dependency-license position | Maintainer and qualified license reviewer | Open; preserve architecture gates |
| Complete MSIX, service/capability justification and sealed executable inventory | Package/Windows owner | Selected preparation direction; implementation and native qualification remain open |
| Explicit package-content enforcement and role-specific Store-compatible CIG/mitigations | Windows/security verifier | Plan sections 3.1–3.4 and CHECK-16/18/23/26; exact-path readbacks and behavioral controls not executed |
| Store account, initial controlled submission and capability/certification acceptance | David Osipov / Microsoft Store | David performs submission; external acceptance is not established by preparation |
| Protected source/build candidate and final Store-delivered package/catalog/payload identity | Build/release verifier | Keep both identities and validated SBOM assets; actual Store artifact not available or qualified |
| Immutable normalized leap calendar/policy and reproducible provenance | Domain/build/release verifier | NV-LP-001 selects the same authenticated complete-release channel; generated data, revision/digest binding and final-artifact evidence remain to be implemented and qualified |
| Valid/tampered/unsigned/substituted/unrelated-Store-package controls | Release/security verifier | Package trust, CIG and NevroTick role authorization require separate evidence; not executed |
| Wrong-clock/offline activation and recovery | Windows/release/security verifier | Section 5.1 cases required on the selected artifact; not executed |
| Complete MSIX lifecycle on supported Windows, including reset/removal and W32Time restoration | Package/Windows verifier | Native execution remains held; final Store-artifact evidence required before broad release |
| Exact-candidate visibility and broad release after post-Store qualification | David Osipov | Owner decision retained; no publication by this work |
| SignPath eligibility or OpenPGP key/profile qualification if MSI is later selected | Maintainer / provider / verifier | Retained alternatives only; neither a selected-MSIX prerequisite nor an automatic fallback |
| Stable-support and transition commitment | David Osipov / release owner | [Release-policy §2](release-policy.md#2-support-status-and-proposed-baseline) is proposed; no stable series, support lifetime, backport schedule or response deadline approved |
| Renewal, revocation, access removal and recovery | Release/security owner | [Operational procedures are written](release-policy.md#7-compromised-publisher-repository-or-build-authority); actual private inventory and applicable renewal/access/recovery rehearsals remain pending and required before public release |

**OPEN-11 tracks acceptance and qualification of the selected Store MSIX route, while preserving the earlier MSI alternatives.** Preparation does not wait for speculative Store preapproval. Initial controlled submission does not claim completed post-Store checks. Broad release requires the actual delivered artifact's authentication, installed-file trust, protection and lifecycle evidence, with David's exact-candidate decision. A provider account, a screenshot of a certificate or this guide alone is insufficient.

## 9. Sources and related documents

Primary Microsoft sources checked for the 3 November selected-MSIX amendment:

- **S19:** [MSIX PackageIntegrity](https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/element-uap10-packageintegrity) and [Content enforcement](https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/element-uap10-content).
- **S20:** [Code Integrity Guard and its Store allowance](https://learn.microsoft.com/en-us/defender-endpoint/exploit-protection-reference#code-integrity-guard), [Exploit Protection PowerShell names](https://learn.microsoft.com/en-us/defender-endpoint/customize-exploit-protection#powershell-reference), [binary-signature policy fields](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_binary_signature_policy) and [process-creation mitigation attributes](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute).

These source findings support preparation requirements; they are not executed
MSIX, Store, CIG or Windows qualification. The [MSIX plan](plans/windows-msix-store.md)
owns the full source assessment and its dated submission/lifecycle findings.

Additional primary sources for the 29 October amendment:

- **S12:** [GnuPG verification and fingerprints](https://gnupg.org/download/integrity_check.html).
- **S13:** [Microsoft UAC behavior](https://learn.microsoft.com/en-us/windows/security/application-security/application-control/user-account-control/how-it-works).
- **S14:** [Microsoft Smart App Control](https://learn.microsoft.com/en-us/windows/apps/develop/smart-app-control/overview).
- **S15:** [Microsoft unsigned MSIX testing feature](https://learn.microsoft.com/en-us/windows/msix/package/unsigned-package).
- **S16:** [Microsoft Authenticode timestamp semantics](https://learn.microsoft.com/en-us/windows/win32/seccrypto/time-stamping-authenticode-signatures).
- **S17:** [Microsoft WINTRUST_DATA action, retrieval and revocation controls](https://learn.microsoft.com/en-us/windows/win32/api/wintrust/ns-wintrust-wintrust_data).
- **S18:** [GnuPG gpgv trust and key-policy limitations](https://www.gnupg.org/documentation/manuals/gnupg/gpgv.html).

S16–S18 and the project-identity implication of S2 were checked on 30 October
2026. These are documentation findings; no Windows/signing experiment was run.


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
