# Signing the Windows time client: a practical package guide

**Revision:** 1.0 with owner-decision, trust-lifecycle, selected-MSIX and release-operations amendments; padding/content-trust and all-signable shipping requirements refined 8 November 2026

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

**8 November owner amendment:** [OD-05 §7.7](nevrotick-owner-decisions.md#77-sign-every-supported-shipped-artifact--8-november-2026) requires signatures on the installer/package and every supported shipped artifact, including nonbinary files. [Section 4.1](#41-required-signatures-for-the-shipped-inventory) is the shared inventory/verification contract. SignPath is a candidate for required inner-file signatures even on the selected MSIX route; it is not selected or approved. A later MSI delivery decision is separate. Section 7 retains the superseded unsigned-MSI/OpenPGP profile as history, not an admissible release route. Foundation publisher-display acceptance and David's human-approver role remain; provider contact is a separate authorized action. [S1–S3]

Sections 5.1–5.2 also refine cross-route qualification where applicable: keep
embedded PE, package/catalog and exact final-artifact evidence separate.

**Later 8 November clarification:** [OD-05 §7.8](nevrotick-owner-decisions.md#78-signpath-and-interim-self-signed-msi--8-november-2026)
accepts an interim self-signed MSI profile while David has no personal publicly
trusted code-signing certificate. [Section 7.1](#71-interim-self-signed-msi-profile)
distinguishes approved-key integrity from public publisher trust. This is an
available signing profile, not a switch from the selected Store MSIX direction.

**OPEN-11** remains open for actual selected-route acceptance and qualification. This guide does not claim that an account, certificate, signing integration, submitted package or release gate has been completed.

## 2. What “getting the certificate” means here

Sections 2–4 retain SignPath research that can support required inner-file signatures for MSIX or a later selected MSI. MSI-specific steps apply only to that delivery format. Sections 4.1 and 5.1–5.2 apply across routes; Store outer-package signing has the separate trust boundary in section 6.1.

A public code-signing certificate identifies a publisher and contains a public key. Signing requires the corresponding private key. Downloading a public `.cer` file would not provide that capability.

SignPath Foundation's model gives an approved project access to a controlled signing service. Its signing keys are held in SignPath's hardware security infrastructure. The Foundation is the certificate's publisher; this is not a general-purpose personal certificate issued in the maintainer's name or a private `.pfx` file to put in GitHub Secrets. Confirm the exact displayed publisher and project metadata during onboarding. [S1, S2]

If this provider is selected and accepts the project, the desired result is:

1. An approved project and signing policy at the provider.
2. A trusted build workflow associated with the correct repository.
3. A narrowly scoped ability to submit the intended release artifacts.
4. David Osipov as the human who approves release signing.
5. Returned, timestamped, signed artifacts matching the complete required inventory; sign the MSI too only if that delivery route is selected.

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
| Distribution | Selected complete Store MSIX, architectures supported, inner-file signing inventory and packaging tool/version; describe MSI only if later selected |
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

These provider-specific steps apply only if SignPath is pursued for the required signatures. They do not block independent source/design preparation or lift the current Windows execution stop. Production per-file signing capability must be resolved before release even when Microsoft supplies the outer Store signature; section 4.1 defines the required scope independently of provider choice.

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

Reconcile every shipped artifact against section 4.1, including the UI, engine, broker, helper, native dependencies, nested custom actions and supported nonbinary files. Identify owned builds, approved upstream signers and unresolved unsigned inputs. Detect actual formats rather than relying only on filename globs. An outer MSI/MSIX signature does not establish each required individual signature; the provider configuration must match the actual inventory and nesting. [S5, S28]

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

The complete inside-out order is in section 4.1. If the MSI delivery alternative is later selected, use one of the following admitted implementations of that order; the former OpenPGP exception is superseded.

Use one of two provider-approved sequences:

| Sequence | Required handling |
| --- | --- |
| Sign payloads, then build and sign MSI | Use the returned signed payload bytes when assembling the MSI. Sign the final MSI afterward. |
| Provider-managed nested signing | Qualify the artifact configuration that signs/repackages nested payloads and then signs the enclosing MSI. Verify the returned complete artifact. |

Apply SHA-256 digest policy and a trusted timestamp as supported by the approved service. Preserve the resulting timestamp evidence. A timestamp helps a verifier assess a signature against its signing time; it is not permission to ignore revocation, a broken chain, a different publisher or Windows verification failures. Microsoft's SignTool documentation describes digest and timestamp verification options. [S7]

Never publish an unsigned input as the release. Never edit executable resources, script text, MSI tables or nested files after their relevant signature has been created. Any authorized transformation requires the appropriate rebuild/re-sign/re-verify sequence and a new stage's approved byte record.

### 4.1 Required signatures for the shipped inventory

**Owner requirement, 8 November 2026:** sign the final installer/package and
every actually shipped file with a supported format-appropriate signing
mechanism. The complete shipping inventory, not a list of extensions supplied to
a signing service, determines coverage. This is production acceptance under
[architecture §19.7](windows-time-architecture.md#197-signatures-for-the-complete-shipped-inventory),
not a claim of implemented signing or native enforcement.

| Actually shipped format | Required signature and qualification |
| --- | --- |
| PE images: owned EXE/DLL, helpers/services, native dependencies, resource images and nested custom-action binaries | Verified embedded Authenticode on every PE, including files with unusual/no extensions and PE content stored in installer streams. Verify the expected project or approved upstream signer. Package/catalog trust is additionally required, not a substitute. [S7, S22, S28] |
| PowerShell files, if already admitted and shipped, such as `.ps1`, `.psm1`, `.psd1` and supported PowerShell XML formats | Native Authenticode using the platform-supported file/SIP profile. Verify exact text/encoding after all generation and packaging. Do not assume ordinary XML has the PowerShell signature format. [S26, S27] |
| Other admitted script formats with native signature support, such as supported WSH files | Use the supported host/format signing profile and identify its verifier and actual invocation behavior. This requirement admits no new script engine or script-based installer action. [S5] |
| Final MSIX/AppX, supported packages/bundles, or a later selected MSI; shipped MSP/MST/MSM/CAB/CAT where supported | Sign each signable shipped artifact in its supported format and verify the actual returned/delivered object. Qualify nested/container signing and MSI cabinet/patch relationships when present. Microsoft supplies the accepted Store package signature; that stage does not sign every inner PE/script for us. [S5, S8, S29, S30] |
| XML/JSON or another data format with an existing admitted signature schema and consuming verifier | Use that compatible mechanism and its exact coverage/identity rules. SignPath lists XMLDSIG and JSON signing, but arbitrary XML/JSON is not thereby an Authenticode format, nor does adding a signature make an unaware consumer enforce it. Do not change strict schemas or add custom crypto solely to create signatures. [S5] |
| Other package data: ordinary configuration, text/licences, images, resources and generated tables without such a compatible individual mechanism | Authenticate through the signed complete package and bind exact content to its approved inventory. Record the format/consumer reason for no separate signature. An untrusted adjacent checksum is not authentication; packaged data remains hostile parser input. |

For a later selected MSI, Windows Installer does not itself verify signatures of
internal CABs; the signed MSI authenticates that internal content. External CAB
verification needs the correct `MsiDigitalSignature` / `MsiDigitalCertificate`
relationships. An additional CAB-signature check in release QA is distinct from
installer enforcement; qualify both their actual scopes. [S29]

This matrix applies to the production shipping set, including nested content.
Source files, discarded build intermediates/static libraries and deliberately
damaged laboratory fixtures are not shipped artifacts merely because they exist
in the repository. Separately admitted OS/runtime dependencies keep their own
trust and inventory owner; this rule does not instruct agents to modify OS files.
If the actual format is signable but signing capability for the declared profile
is unavailable, record **open signing work / release blocked**, not unsupported
or N/A. The owner-approved interim self-signed MSI profile in section 7.1 is a
signing capability; absence of a public CA certificate alone does not block it.

**Signature identity and custody.** For each inventory entry record path and
container/stream location, actual format, ownership/provenance, required signing
profile and expected signer role/identity, verifier and signature source,
algorithm/timestamp policy, chain/revocation result, final full-file SHA-256 and
length, release identity and observation stage. Distinguish embedded, catalog and
package evidence; distinguish observed pass/fail, unknown/offline, not run and
reasoned non-applicability. A generic `Valid` result, signer display name or
Microsoft/Store signer class alone does not identify an approved component.
Certificate renewal must remain possible under the admitted identity policy.

Name the actual trust profile as well as the signing format. For interim
self-signed MSI, record approved-key signature verification separately from
**public trust: not established / untrusted chain** on an unchanged clean system.
A positive result in a deliberately configured verification VM is not a default
consumer-Windows trust pass. Section 7.1 defines this narrow owner exception;
it does not relax the selected Store route's trust or mitigation requirements.

Preserve valid approved upstream signatures. They can satisfy the component's
individual-signature obligation; do not reattribute the component to NevroTick.
`Set-AuthenticodeSignature` removes an existing signature, and SignPath's
`authenticode-sign` defaults to replacement; its documented append support is
for PE/CAB/CAT, not every format in this inventory. Review
each operation instead of bulk re-signing everything. Resolve an unsigned
third-party component through provenance, licence and provider policy or block
its shipment. A missing required signature cannot be waived by calling the
component external while bundling it. [S5, S27]

**Order and acceptance:**

1. Freeze the approved shipping inventory; finish linking, product metadata,
   resources, script generation and any other content transformation.
2. Sign and verify each required inner artifact. For new project Authenticode
   signatures use the admitted strong digest policy (SHA-256 baseline) and
   trusted RFC 3161 timestamp with an admitted digest/service. Qualify the exact
   tool/provider profile. Preserve admitted upstream signatures/timestamps;
   certificate expiry alone does not invalidate an appropriately timestamped
   signature. Chain/revocation and wrong-clock rules still apply. [S7, S16]
3. Assemble using those verified bytes; sign nested signable containers from the
   inside outward, then the final installer/package. Provider-managed nested
   signing must demonstrate the same ordering and returned inventory. [S30]
4. Verify the actual packaged members, required individual signatures and outer
   package, with no missing/extra executable content. Keep initial Store
   submission and Microsoft's delivered-package signing as distinct stages.
   Reconcile and re-verify the actual delivered inner files after Store processing.
   If required signatures are removed or altered incompatibly, release remains
   blocked pending resolution/owner decision, without a catalog-only exemption.
5. Record each stage's final full-file hashes/lengths and protected approval;
   preserve those bytes through publication and use. Full-file custody and the
   PE padding/content-trust checks in section 5.2 remain necessary.

At the admitted CHECK-18/23/26 checkpoint, pair a fully signed valid candidate
with missing/removed/invalid inner signatures, a valid wrong signer, omitted
nested or extensionless PE, modified signed script and mismatched inner/outer
inventory. Inspect inert fixtures without executing altered code. Name the
boundary that rejects each case; sign-tool success or `/all` does not establish
all-file coverage. Reuse CHECK-25 for an existing data consumer and CHECK-27 for
native verification work; there is no new test program or acceptance hierarchy.

**Runtime meaning:** signatures are verification material. Ordinary Windows
loading does not promise to verify every signed file on every use; PowerShell
execution policy is explicitly not a security boundary. Keep the selected
PackageIntegrity, CIG/App Control compatibility, safe loading, protected installed
paths and helper/IPC authorization requirements, with actual launch/load evidence.
Do not add a custom runtime scanner or ask users to verify detached signatures
as a substitute for these controls. [S19, S20, S22, S31]

## 5. Authenticode verification before publishing

Apply the per-file Authenticode checks to required supported artifacts on every route. The MSI command applies only if MSI is selected; section 6.1 supplies the separate Store package/catalog checks. Unsupported commands do not waive required signatures: choose the admitted format verifier and retain distinct results. For section 7.1's accepted interim self-signed MSI, use its separately labelled verification environment and clean-machine trust observation; the recipe below is not permission to suppress an untrusted-chain error or claim public trust.

Use the admitted Windows SDK's SignTool on the actual returned files. Example commands below are **verification recipes, not executed results**; substitute the real file names and run from a trusted tool environment.

```powershell
signtool verify /pa /all /v /tw .\release\Product.msi
if ($LASTEXITCODE -ne 0) { throw "MSI signature verification failed or warned" }

signtool verify /pa /all /v /tw .\release\Product.ClockBroker.exe
if ($LASTEXITCODE -ne 0) { throw "Broker signature verification failed or warned" }
```

Repeat the payload check for every file required by the signed inventory. `/pa` selects Authenticode verification policy, `/all` verifies all signatures present and `/tw` warns about a missing timestamp. For the publicly trusted profile, failures/warnings block acceptance. For the interim self-signed profile, section 7.1 records expected clean-machine chain distrust separately; unexpected failures and missing/invalid signatures still block it. Verifying the MSI does **not** recursively prove every embedded executable's signature. [S7]

The release checker must also enforce our intended identity and package manifest: a successful generic trust check alone can accept a valid signature from an unintended publisher. The accepted Foundation publisher display is not sufficient NevroTick project identity: qualify rejection of an unrelated, legitimately Foundation-signed artifact alongside acceptance of the approved NevroTick artifact. Bind admission to the approved project/artifact lineage and protected manifest, not display text or editable version resources alone. Design publisher/certificate renewal handling explicitly rather than permanently pinning a single leaf certificate without a rotation plan. [S2]

The release evidence must establish:

1. **Identity:** the expected project, approved upstream or package signer for the artifact's role, code-signing use, chain, timestamp and applicable revocation checks.
2. **Completeness:** all required signable payloads, helper/custom-action files and the selected final package are covered; packaged member hashes match the approved inventory, with remaining data authenticated by the package.
3. **Origin:** exact reviewed commit, dependency lockfile, native inputs, toolchain, build and signing request.
4. **Windows behavior:** install, ordinary-user launch, service operation, update, repair, rejection of unsupported downgrade and uninstall on a clean installation of each claimed [client/Server profile](../tasks/windows11-integration.md#432-two-family-windows-qualification-matrix--5-november-2026). Qualify the actual selected route; a later MSI needs its own applicability and native evidence.
5. **Negative controls:** a tampered package, substituted payload, unexpected publisher and unapproved version are rejected for the intended reason; a valid neighboring candidate succeeds.
6. **Publication identity:** checksums and release metadata describe the actual final signed bytes at the identified stage. Bind each published version's prepared, validated SBOM assets to the selected complete release and final checksums. Reconcile Rust, native/generated/resources and actual delivered-package inventory against the exact version/source candidate and signed hashes before broad publication; see [the repository SBOM tooling and handoff](../TOOLING.md#release-sbom-tooling). An SBOM and optional attestations supplement that record; they do not replace required signatures.

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

### 5.2 PE certificate padding and exact final bytes

**Qualification refinement, 8 November 2026:** reuse W11-W10/P6 and
CHECK-18/23/26/27. The amended owner requirement makes embedded Authenticode
mandatory for every shipped PE, including Store payloads. Apply these PE cases
to that inventory; do not transfer a PE-specific CVE to MSI/MSIX containers or
script formats. Final-byte custody applies at every admitted release stage.
Current Windows execution holds remain; these cases have **not been run**.

CVE-2013-3900 concerns extra data after a PKCS#7 structure in `WIN_CERTIFICATE`.
PE Authenticode does not hash every file byte. The opt-in
`EnableCertPaddingCheck` control rejects nonzero padding there, but does not
authenticate all other excluded content. Added data is not automatically
executable; a consumer can create risk by trusting or interpreting it. This is
certificate-table padding, not a choice of RSA signature padding. [S21, S22, S25]

Refine the existing verification receipt and custody rules as follows:

1. **Identify the actual verifier.** Record Windows build, tool/SDK version and
   process architecture, signature source, trust action/flags, retrieval and
   revocation policy, exact input hash and observed result. For the admitted
   strict Windows qualification host, record `EnableCertPaddingCheck` in both
   applicable registry views on x64 and restart state. Current Microsoft
   guidance uses DWORD `1`; an existing string `"1"` is also supported. A registry
   readback is configuration evidence, not the functional padding result. [S21]
2. **Keep embedded and catalog results distinct.** Microsoft's catalog PE
   verification path does not check embedded-signature padding.
   `Get-AuthenticodeSignature` prefers the catalog when both forms exist. Its
   `Valid` result therefore cannot establish this embedded check. The PE
   SignTool recipe above uses embedded verification; do not add `/a` or catalog
   selection switches merely to obtain a pass. `/all` selects signatures, not
   every byte. Keep package/catalog, CIG and project-identity results with their
   existing owners. A catalog-only shipped PE fails the individual-signature
   requirement; non-PE files use their own format checks, not PE-padding N/A as
   a waiver of their required signature.
   [S7, S22, S24]
3. **Bind approval to complete final bytes.** Record full-file SHA-256 and byte
   length after the stage's authorized signing, timestamping and packaging.
   Bind these values to the approved candidate in the existing protected
   release record, then compare the exact files through handoff and publication.
   A new digest computed from an unapproved download, or a replaceable checksum
   sidecar alone, supplies no approval authority. Keep pre-signing, returned,
   submitted and Store-delivered observations distinct; an authorized later
   transformation requires a new stage's verification and approved record.
4. **Qualify provider and native behavior.** The SignPath reference's
   `authenticode-verify` description does not establish its effective padding
   policy or embedded/catalog selection. Obtain candidate-bound provider evidence
   identifying the returned files, effective policy and signature source, or
   perform independent strict Windows verification of those PE files.
   If an admitted checker calls `WinVerifyTrust`, only return value **zero** is
   success; the result is a `LONG`, not an `HRESULT` for `SUCCEEDED`. Reuse the
   admitted Windows verifier; this refinement introduces no custom PE parser or
   cryptographic verifier. [S5, S23]

At the existing native qualification stage, use an owned harmless PE and its
controlled modified counterpart. Inspect signatures without executing either PE.
Keep the signing identity, chain and other inputs valid so unrelated trust
failures do not masquerade as evidence for the changed condition.

| Case | Required observation |
| --- | --- |
| Healthy approved PE with compliant padding | Required embedded verification succeeds under the recorded strict policy; full-file hash matches its approved record |
| Otherwise valid PE with nonzero data after PKCS#7 inside `WIN_CERTIFICATE` | Strict embedded verification rejects the padding case; an unrelated broken chain or malformed signed content is insufficient evidence |
| Changed final bytes against the previously approved full-file hash | Custody/admission comparison rejects the mismatch even if a generic signature check still reports valid |
| Catalog-preferred verification of a PE that also has an embedded signature | Identify the selected signature source; that result cannot be recorded as an embedded-padding pass |
| Harmless application-data marker in an unauthenticated region supplied to an existing consumer | A valid carrier-signature result cannot make the marker select a trusted command, destination, configuration or executable payload; observe the real admission boundary with inert effect sinks beside an admitted healthy input |
| Substituted extracted component or otherwise permitted downstream input | Its own component/release authentication and exact-byte custody reject substitution before protected use; the carrier's valid signature cannot supply missing authority |

Generate product metadata before signing. Do not personalize signed files by
placing configuration, download URLs, commands or purportedly trusted metadata
in unauthenticated certificate padding. Apply the producer **and consumer** rule
in [architecture section 19.6](windows-time-architecture.md#196-authenticated-artifact-content-and-consumer-trust):
excluded application data, unvalidated attributes and additional signature
material gain no application authority from a valid containing PE. Any separately
admitted configuration or payload keeps its own authentication/admission contract;
preserve the verified object through actual use. Legitimate signature/chain/
timestamp inspection remains with the admitted verifier, not a blanket ban on
additional signatures or attributes. [S22, S25]

The consumer cases apply when such a consumer exists in the admitted scope;
record its real source-to-effect path or justified non-applicability. Use harmless
markers and inert effect sinks, not malware or execution of the altered PE.
Consumer evidence is separate from native signature-verifier evidence. Do not
create a downloader, custom verifier or redundant runtime file scanner for this
test; the current complete-package/no-runtime-downloaded-code contract remains.

The strict lab setting is not evidence about provider hosts or customer systems,
and this qualification does not authorize an installer to change customers'
global Wintrust policy. W11-W10/S6.4 owns the
[current assignment and outstanding results](../tasks/windows11-integration.md#pe-content-trust-refinement--8-november-2026).

## 6. What the other routes do and do not solve

| Route | Suitable role for this project | Limitation |
| --- | --- | --- |
| **SignPath Foundation** | Possible free OSS signer for required inner files on MSIX or a later selected MSI | Provider selection, eligibility and exact format/provenance policy remain open. This particular service is not mandatory; supported production signing for required artifacts is mandatory before release |
| **Microsoft Store with a complete MSIX** | Selected first submission direction for the complete hardened product; accepted MSIX packages receive Store signing | Actual service/capability acceptance and package qualification remain open; no split service/UI installers or automatic fallback |
| **Microsoft Store with MSI/EXE** | Possible listing/distribution route for an already signed complete installer | The developer must supply trusted signatures; it does not provide free signing for an unsigned MSI |
| **Azure Artifact Signing Public Trust** | Paid hosted-signing alternative when the applicant is eligible | Not a free OSS entitlement; identity/geography restrictions apply |
| **Commercial public code-signing provider** | Possible later owner-selected alternative if cost, identity verification and provider policies are acceptable | Obtain current eligibility and price directly; no free availability or automatic fallback assumed |
| **Owner-controlled interim self-signed certificate** | Explicitly accepted interim MSI release profile in section 7.1 | No public CA/Windows publisher trust or warning-free installation promise; channel, installed-code and native compatibility still need qualification |
| **Separate self-signed laboratory certificate** | Controlled installer/privilege/lifecycle experiments | Lab trust substitutions qualify only the recorded experiment, not normal consumer trust or the interim release profile's key custody |

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

For an accepted Store MSIX, Microsoft supplies the outer package signature after
certification; no purchased developer certificate/token is required for that
outer signature. This does not supply the required inner-file signatures:
their production signer, supported formats and custody remain open qualification
work under section 4.1. No paid service is selected. Current Individual and Company onboarding waive
their former registration fees through `storedeveloper.microsoft.com`. Select
the truthful account type and verify country/document eligibility; old portal
flows and broad market-availability statements are not sufficient evidence.
Store MSI/EXE submissions still require trusted installer and PE signatures;
the initial OpenPGP profile does not meet that requirement.

Microsoft documents a common catalog signature for the files in an MSIX app.
This supports a package-aware trust candidate and must not be dismissed as trust
only in an outer archive. It does not establish an independently usable embedded
Authenticode signature on every extracted EXE/DLL. The current owner requirement
therefore verifies both levels independently, including all other supported
shipped formats in section 4.1. The selected preparation requires the following
distinct controls; the plan owns their detailed contract.

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
  reconcile the final package, catalog, manifest, required per-file signatures,
  payload hashes, SBOM and
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
Store-artifact checks above. MSI delivery remains conditional on a later owner
decision; SignPath may also supply inner-file signatures for MSIX if qualified.
The unsigned-MSI/OpenPGP exception is superseded.

Use [release-policy §4](release-policy.md#4-distribution-containment-choose-the-actual-store-operation)
to distinguish initial private audience, later flights, gradual updates, halt and
availability controls. Record publication containment separately from installed
recovery and any publishing-authority incident. A corrective package must qualify
the affected state and actual delivery route; a Store signature does not prove
that malicious same-product publication through a compromised account is
independently rejected on the device. [Section 7](release-policy.md#7-compromised-publisher-repository-or-build-authority)
preserves that residual and the distinct MSA/Entra/application recovery routes.

<a id="7-development-and-the-accepted-initial-openpgp-release-profile"></a>
<a id="7-development-signing-and-the-superseded-openpgp-profile"></a>

## 7. Development signing, interim MSI and historical profiles

The later 8 November [OD-05 §7.8 clarification](nevrotick-owner-decisions.md#78-signpath-and-interim-self-signed-msi--8-november-2026)
accepts the interim self-signed MSI option in §7.1. This preserves actual native
signatures under section 4.1; it does not restore the historical unsigned-MSI/
OpenPGP permission retained in §7.2. Earlier anchors remain for existing links.

The architecture described an isolated, test-signed S0.6 MSI to qualify installation, privilege separation and removal behavior. Any later authorized laboratory package experiment must retain the current Windows execution stop until its owner-controlled gate is released. Trust a test certificate only inside the disposable laboratory VM; record the substituted trust store and certificate in the evidence. Restore or destroy the VM afterward.

That experiment does not satisfy the public publisher-trust portion of CHECK-18 or CHECK-23. Ordinary users must not be instructed to install our development root certificate. Do not disable Windows signature, reputation or application-control protections to make a public package install.

### 7.1 Interim self-signed MSI profile

**Scope:** David accepts signing an interim MSI and its required owned payloads
with an owner-controlled generated certificate while public-trust signing is
unavailable. This is not limited to a lab build and does not require buying a
certificate. It is an available profile for a later selected complete MSI,
subject to its actual qualification and exact-candidate approval; Store MSIX
remains selected. No key generation, signing or publication has occurred here.

The useful guarantee is signature correctness against an **independently
approved project key**, within the format's authenticated coverage. Record the
approved public certificate/key identity through protected release authority;
reading a certificate from the artifact being checked cannot authenticate its
owner. An attacker can supply a different self-signed key. Signature removal
does not universally prevent MSI/PE execution, and PE padding/full-byte custody
still follows section 5.2. [S22, S32]

Keep the three evidence layers explicit:

| Observation | Required result and limit |
| --- | --- |
| Release QA in an isolated verifier context with the approved certificate explicitly trusted | Expected-key signature/format checks succeed; changed covered content, removed signatures and a different key, including the same subject/display name, fail the actual release gate. Independently bind expected key and final hashes to the approved candidate. Record the trust substitution; this is not public Windows trust. |
| Untouched clean target, without installing that certificate | Record the actual untrusted-chain/publisher result and effective policy. Expected chain distrust is permitted only by this named interim profile; retain the failure/status, never relabel it `Valid` or ignore all nonzero verifier returns. No public-trust CHECK-18 pass is claimed. |
| Actual delivery/install/startup/helper/loading/update/repair/removal | Qualify every claimed configuration with protections active. Record warnings and policy refusals honestly; a blocked configuration is not a supported installation. Other required security/lifecycle behavior must still work beside adversarial controls. |

Windows Installer's ability to install a direct MSI depends on applicable policy;
there is no universal MSI requirement that every signature chain to a public CA.
Microsoft's current SmartScreen guidance gives self-signed certificates the same
initial-warning behavior as unsigned files. Smart App Control does not treat
self-signed certificates as trusted-provider certificates; its separate reputation
assessment prevents concluding that every self-signed file is always blocked.
Do not advertise improved default Windows trust from signature presence. [S32]

The accepted lower publisher-trust level is explicit; ordinary consumers gain
no independent automatic proof that this self-signed key belongs to NevroTick
merely by downloading the MSI. Publish truthful release-channel/profile status.
Do not add customer root/Trusted People/Trusted Publishers installation, ask
users to disable protection, or require manual detached verification. The
controlled verifier's certificate trust belongs only in its disposable lab.

For any existing helper, maintenance or loader path that relies on a publicly
trusted signer, R-06/W11-W10 must resolve that exact admission dependency before
claiming the interim profile works. Keep CIG/DLL protections and current authority
requirements; do not replace `WinVerifyTrust` failure with unconditional acceptance
or introduce a custom runtime verifier as a side effect. Missing public trust is
not by itself a blocker for this approved profile; a demonstrated incompatibility
with another required control remains a concrete blocker for that configuration.

Maintain separate signing authority from ordinary lab keys: document protected
private-key custody, authorized use, backup, expiry, rotation and compromise
response. Expose no private key/PFX or password in the repository, package or logs.
Use the same metadata-first, inner-before-outer, strong-digest/timestamp and final
inventory controls as section 4.1, preserving approved upstream signatures.
A trusted timestamp does not make a self-signed issuer publicly trusted. Recheck
the actual timestamp/verification behavior rather than claiming CA revocation
service for an unissued self-signed certificate. [S7, S16]

**Channel boundaries:** this option does not satisfy the Store MSI/EXE requirement
for trusted-CA signatures on the installer and all PE files. Direct MSIX delivery
requires device trust and is not made equivalent to direct MSI by this permission.
Do not transfer those Store rules into an invented blanket WinGet-MSI prohibition;
the [existing WinGet plan](plans/winget-distribution.md) owns actual channel and
manifest validation. No alternative delivery channel is selected here. [S8, S33]

If SignPath accepts the project, qualify its exact allowed artifact graph and
Foundation publisher identity. Transition from the interim key to the approved
public-trust signer with deliberate issuer-policy updates, new final-byte records
and upgrade/repair/rollback checks. The old interim profile must not become a
silent downgrade after the stronger profile is adopted. Provider eligibility and
the owner's release decision remain separate; self-signing grants no exemption
from Foundation restrictions. [S1, S2]

### 7.2 Historical unsigned-MSI and OpenPGP permission

**Superseded on 8 November 2026:** the quoted former profile below is not an
admissible current release route. Section 7.1 permits actual embedded/native
self-signatures, not unsigned installers supplemented only by detached signatures.

> **Retained 29 October OD-05 policy:** if the owner returns to MSI and no usable free Authenticode route is available initially, David permits an MSI without project Authenticode signatures, accompanied by OpenPGP signatures. This earlier removal of a blanket Authenticode blocker does not override the 3 November MSIX selection, approve a paid service or authorize an automatic release. The package remains one complete product, and David approves the exact candidate.
>
> Before this profile can ship, establish a trusted full release-key fingerprint and key-custody/expiry/revocation/rotation process. Produce a detached signature over the exact final MSI and an OpenPGP-signed manifest binding version, source/build identity, package hash, corresponding-source and SBOM artifacts. Authenticate the expected key, not just a matching name or short key ID. No key has been selected or generated by this guide. [S12]
>
> Select the exact verifier and its key-policy enforcement separately from
> cryptographic signature validity. The `gpgv` manual says its keyring is trusted
> and expired/revoked keys are not checked. Therefore a successful `gpgv` exit plus
> the expected fingerprint alone cannot establish this profile's expiry/revocation
> requirements. Define protected trusted-key distribution, freshness evidence,
> rotation/revocation authorization and offline/unknown-age behavior; qualify
> revoked, expired, superseded and stale-keyring cases with a valid control. This
> does not adopt `gpgv`, a runtime dependency or a time-check bypass. [S18]
>
> Qualify valid, tampered, wrong-signer and mismatched-version/manifest cases. Preserve upstream signatures where present. Review the complete path from verified download and approved package to protected installed code, helper/IPC identity, DLL loading, repair and update lineage. Missing Authenticode must not become unconditional acceptance; a hash beside attacker-writable code is not an independent trust anchor. Research R-06 must resolve the signed-helper assumptions before qualifying this route.
>
> OpenPGP verification supplies no Windows verified-publisher identity. Unknown unsigned code can be blocked by Smart App Control or organizational policy; test the installer and each payload under the supported security configurations, disclose restrictions, and do not recommend disabling protections or installing a development root. [S13, S14]
>
> Keep CHECK-18 results separate: MSI Authenticode qualification remains incomplete if unavailable; the retained initial OpenPGP path requires its own exact-artifact, installed-file and native lifecycle evidence. All unrelated licence, privilege, mitigation, feature and applicable ASVS gates remain. Sidecar signatures/manifests are verification material, not another application installer. No embedded updater or runtime OpenPGP dependency is adopted. Microsoft's unsigned MSIX facility is a testing path, not the broad-distribution alternative. [S15]
>
> See [NV-DEC-001 section 7](nevrotick-owner-decisions.md#7-od-05--signing-and-release-ownership) and [research R-06](nevrotick-research-handoff.md#8-r-06--signing-eligibility-and-initial-openpgp-releases). A later Authenticode transition must preserve product upgrade identity and reject unsupported downgrade of trust.

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
| Every shipped signable artifact, including nonbinary files, plus outer package | Build/package/security verifier | Section 4.1 defines inventory, signing order and expected identities; production provider capability, complete coverage and post-Store preservation are open |
| Embedded PE padding checks and complete final-byte binding | Release/security verifier | Every shipped PE requires its embedded result; section 5.2's strict-policy readback and behavioral cases are not executed |
| Complete MSIX lifecycle on supported Windows, including reset/removal and W32Time restoration | Package/Windows verifier | Native execution remains held; final Store-artifact evidence required before broad release |
| Exact-candidate visibility and broad release after post-Store qualification | David Osipov | Owner decision retained; no publication by this work |
| Per-format native signing capability and SignPath eligibility if pursued | Maintainer / provider / verifier | Required inner-file capability also applies to MSIX; missing capability blocks release, not independent source preparation. MSI delivery remains conditional; the OpenPGP-only exception is superseded |
| Interim self-signed MSI, if MSI delivery is later selected | Owner / release / Windows verifier | OD-05 §7.8 accepts the profile; section 7.1 requires approved-key coverage, truthful public-trust failure, actual channel/installed-code qualification and later signer migration. No key or signed artifact is established |
| Stable-support and transition commitment | David Osipov / release owner | [Release-policy §2](release-policy.md#2-support-status-and-proposed-baseline) is proposed; no stable series, support lifetime, backport schedule or response deadline approved |
| Renewal, revocation, access removal and recovery | Release/security owner | [Operational procedures are written](release-policy.md#7-compromised-publisher-repository-or-build-authority); actual private inventory and applicable renewal/access/recovery rehearsals remain pending and required before public release |

**OPEN-11 tracks acceptance and qualification of the selected Store MSIX route and its required per-file signatures.** MSI delivery remains a later owner choice, with no unsigned/OpenPGP-only exception. Preparation does not wait for speculative Store preapproval. Initial controlled submission does not claim completed post-Store checks. Broad release requires actual delivered-artifact coverage, installed-file trust, protection and lifecycle evidence, with David's exact-candidate decision. A provider account, a screenshot of a certificate or this guide alone is insufficient.

## 9. Sources and related documents

Primary sources checked for the later 8 November self-signed MSI clarification:

- **S32:** [SmartScreen reputation](https://learn.microsoft.com/en-us/windows/apps/package-and-deploy/smartscreen-reputation), [Smart App Control signing](https://learn.microsoft.com/en-us/windows/apps/develop/smart-app-control/code-signing-for-smart-app-control), [SAC overview](https://learn.microsoft.com/en-us/windows/apps/develop/smart-app-control/overview) and [Windows Installer/software restriction policy](https://learn.microsoft.com/en-us/windows/win32/msi/windows-installer-and-software-restriction-policy). These distinguish warnings, trusted-certificate admission, reputation and policy-dependent installation; they do not establish a universal self-signed-MSI ban or successful NevroTick installation.
- **S33:** [MSIX certificate creation/trust](https://learn.microsoft.com/en-us/windows/msix/package/create-certificate-package-signing) and [App Installer trusted-certificate requirement](https://learn.microsoft.com/en-us/windows/msix/app-installer/troubleshoot-appinstaller-issues#trusted-certificates). This MSIX trust requirement is not a generic MSI rule. S8 states the separate Store MSI/EXE CA-chain requirement.

David's explicit instruction authorizes the interim profile; these sources
establish its trust limits. No source research is a signing or native pass.

Primary sources checked for the 8 November all-signable shipping requirement:

- **S26:** [PowerShell about_Signing](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_signing), including supported script/data formats.
- **S27:** [Set-AuthenticodeSignature](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.security/set-authenticodesignature), including SIP support and removal of an existing signature.
- **S28:** [Windows Installer custom action type 1](https://learn.microsoft.com/en-us/windows/win32/msi/custom-action-type-1), including DLLs held in Binary table streams and exported `.ibd` files.
- **S29:** [Digital signatures and Windows Installer](https://learn.microsoft.com/en-us/windows/win32/msi/digital-signatures-and-windows-installer), [internal/external cabinet verification](https://learn.microsoft.com/en-us/windows/win32/msi/digital-signatures-and-external-cabinet-files) and [a fully verified signed installation](https://learn.microsoft.com/en-us/windows/win32/msi/authoring-a-fully-verified-signed-installation).
- **S30:** [SignPath nested signing examples](https://docs.signpath.io/artifact-configuration/examples#deep-signing-an-msi-installer-with-nested-executables); S5 supplies supported formats and signature replacement/append semantics.
- **S31:** [PowerShell execution policies](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_execution_policies), explicitly not a security boundary.

The per-file requirement is David's policy, stronger than what a package signature
alone establishes; these references do not prove provider acceptance, signing
implementation, Store preservation of signatures or native runtime enforcement.

Primary sources checked for the 8 November PE-padding refinement:

- **S21:** [MSRC CVE-2013-3900](https://msrc.microsoft.com/update-guide/vulnerability/CVE-2013-3900), current FAQ revision 2.4 of 23 December 2024, and [advisory 2915720](https://learn.microsoft.com/en-us/security-updates/securityadvisories/2014/2915720). The current FAQ accepts DWORD `1` and the earlier string `"1"`; the advisory describes opt-in activation and restart. This is source research, not a verifier-host readback.
- **S22:** [Microsoft: Understanding executable file signing](https://learn.microsoft.com/en-us/windows/win32/secbp/understanding-pe-signatures), including the catalog-verification exception.
- **S23:** [WinVerifyTrust](https://learn.microsoft.com/en-us/windows/win32/api/wintrust/nf-wintrust-winverifytrust), including strict-padding behavior and exact-zero success semantics.
- **S24:** [Get-AuthenticodeSignature](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.security/get-authenticodesignature), including catalog preference.
- **S25:** [MSRC: MS13-098 Authenticode security explanation](https://www.microsoft.com/en-us/msrc/blog/2013/12/ms13-098-update-to-enhance-the-security-of-authenticode/). Use its mechanism explanation; its historical planned default-enforcement date was superseded by the opt-in policy in S21.

S7 and S5 were also checked for verification-source selection and the limits of
the documented provider checks. Section 5.2's receipt and acceptance criteria
are project engineering requirements derived from those sources, not claims of
completed Windows, provider or Store qualification.

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
