# Signing the Windows time client: a practical package guide

**Revision:** 1.0 with owner-decision, trust-lifecycle and release-operations amendments; direct-MSIX/no-Store decision 21 November, TUF updater/signing-boundary amendment 25 November and artifact/signing hardening reconciliation 28 November 2026

**Research date:** 28 October 2026  
**Status:** Preparation guide; Microsoft Store publication rejected. Signing is deferred and unresolved; production signing/trust and native qualification are **not completed**.

**Applies to:** The Chrony-derived Rust time client with one complete hardened MSIX for direct distribution, containing its services, ordinary-user UI, administrative helper and all required payloads. Earlier Store/MSI research is retained as history or a possible later owner decision, not an active publication route.

## 1. The decision and the next action

**Selected direction, amended 21 November 2026:** prepare **one complete MSIX
for direct distribution**. David will not publish NevroTick to Microsoft Store
because he rejects its broad publisher duty to defend and indemnify Microsoft,
including legal costs. [OD-05 §7.9](nevrotick-owner-decisions.md#79-direct-msix-distribution-microsoft-store-rejected--21-november-2026)
supersedes the former Store-attempt/reconsider-after-refusal direction. A Store
account, submission, certification or rejection is not a prerequisite.

**Signing is deferred to later.** Continue source, metadata and package
preparation without selecting a provider, buying a certificate or initiating
onboarding. This decision does not establish public publisher trust or approve
an unsigned release. Final required signatures and target-device trust remain
unresolved distribution work. [OD-05 §7.10](nevrotick-owner-decisions.md#710-tuf-authenticated-direct-msix-updater--25-november-2026)
now selects the TUF-authenticated, user-initiated complete-MSIX updater design;
[section 10](#10-updater-signing-and-key-operations) distinguishes its metadata
keys from Windows package signing. Actual client/provider/keys and hosting remain
unqualified, and SDK/runtime licences remain separate from the rejected Store terms.

The [MSIX plan](plans/windows-msix-store.md) retains sealed package content,
per-role strongest-compatible protections and native qualification. Reconcile
its former Store-signer CIG assumptions with the actual direct-distribution
signer/module graph; do not treat an ordinary trusted publisher certificate as
Store-signer admission. Current Core priority and execution holds remain.

David retains provider, publication and exact-candidate release decisions.
Local/test-signed mechanics, final signing and directly delivered-artifact
qualification are distinct stages. [Section 6.1](#61-direct-msix-trust-and-publication-boundaries)
owns those trust boundaries; no ready package or signing capability is claimed.

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
available MSI signing profile, not automatic trust admission for direct MSIX.

**OPEN-11** remains open for direct-route signing/trust and qualification. Its unresolved signing portion does not block independent preparation. This guide establishes no provider acceptance, certificate, signing integration, distributed package or release pass.

## 2. What “getting the certificate” means here

Sections 2–4 retain SignPath research for later signing work; they are not current onboarding instructions while signing is deferred. Exact MSIX/package and inner-file support, eligibility and terms need review if that provider is later selected. MSI-specific steps apply only to a later selected MSI. Sections 4.1 and 5.1–5.2 apply across routes; section 6.1 owns direct-package trust.

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
| Distribution | Selected complete directly distributed MSIX, architectures supported, package/inner-file signing inventory and packaging tool/version; signing is deferred, and MSI remains unselected |
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

These provider-specific steps apply only if SignPath is pursued for the required signatures. They do not block independent source/design preparation or lift the current Windows execution stop. Production package and per-file signing capability must be resolved before public distribution; section 4.1 defines the required scope independently of provider choice. No Store outer signature is expected, and no provider onboarding is required now.

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
| Final MSIX/AppX, supported packages/bundles, or a later selected MSI; shipped MSP/MST/MSM/CAB/CAT where supported | Sign each signable shipped artifact in its supported format and verify the actual returned/delivered object. Qualify nested/container signing and MSI cabinet/patch relationships when present. The later selected direct-distribution signer must cover the package; outer signing does not sign every inner PE/script for us. [S5, S8, S29, S30] |
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
Join each executable or loaded-image record to its actual role, image kind,
selected features/native dependency graph, activation route and supported
OS/hardware/VM profile in architecture §15.4. A shared source crate, signer or
package cannot transfer one artifact's mitigation evidence to another image.

Name the actual trust profile as well as the signing format. For interim
self-signed MSI, record approved-key signature verification separately from
**public trust: not established / untrusted chain** on an unchanged clean system.
A positive result in a deliberately configured verification VM is not a default
consumer-Windows trust pass. Section 7.1 defines this narrow owner exception;
it does not establish direct-MSIX target-device trust or relax its mitigation requirements.

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
   package, with no missing/extra executable content. Keep prepared, returned
   signed and directly downloaded artifacts as distinct observations.
   Reconcile and re-verify the actual delivered inner files after all transformations.
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

#### Resource-only DLL signing handoff — 28 November 2026

The optional event-resource DLL is an existing independently built artifact,
not an EXE covered by the shared executable gate. The
[resource builder](../metadata/build_event_resource.py),
[unsigned reader](../metadata/event_resource_pe.py) and
[metadata contract](../metadata/README.md) bind the selected SDK/compiler inputs,
generated provider identity, exact resources and unsigned DLL bytes. The reader
requires no code or entry point, no imports/exports, one read-only `.rsrc` section,
the fixed NX/ASLR/high-entropy profile and no unrelated directories or overlays.
Certificate-directory data and an individually signed representation are rejected
by this deliberately unsigned format. The provider is currently unset; the
optional event-resource archive path and signed-base fixture path are separate.
This establishes no final signed or installed resource qualification. [S37, S38]

If admitted into the shipping inventory, this PE still requires its individual
signature as well as the complete package signature. W11-W10/build/packaging and
W11-W05 retain the CHECK-18/20/23/26 handoff: preserve independently admitted
unsigned input/receipt identities, then bind the exact returned signed image to
the same validated resources and no-code invariant under an appropriate signed
artifact profile. Record permitted signing transformations, actual signature
source/expected signer, and separate final hashes/lengths. Do not relax the
unsigned reader to accept arbitrary signed shapes or inherit a pass from an
EXE-only gate. Reconcile those exact signed bytes through package/archive and
direct-download readback, then qualify the installed provider/resource path
without adding executable content. Final signature, installed resolution and
applicable padding results remain separate observations.

## 5. Authenticode verification before publishing

Apply the per-file Authenticode checks to required supported artifacts on every route. The MSI command applies only if MSI is selected; section 6.1 supplies the separate direct-package/catalog checks. Unsupported commands do not waive required signatures: choose the admitted format verifier and retain distinct results. For section 7.1's accepted interim self-signed MSI, use its separately labelled verification environment and clean-machine trust observation; the recipe below is not permission to suppress an untrusted-chain error or claim public trust.

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

Clock-dependent verification is particularly relevant for this application. Qualify the architecture's extreme-date, certificate-time and revocation behavior using controlled fixtures and disposable machines. Do not weaken TLS or executable-signature validation to get through a test with a wrong system clock. [Architecture §19.8](windows-time-architecture.md#198-tuf-authenticated-direct-msix-updater) now adopts TUF for the in-app updater, but its qualified freshness basis and Windows PKI are separate gates. A TUF verification time does not cause Windows to use that time, and no new time-bootstrap exception follows.

### 5.1 Wrong-clock and offline activation matrix

This matrix applies to the selected directly distributed MSIX and any MSI profile the owner
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
mandatory for every shipped PE, including direct-MSIX payloads. Apply these PE cases
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
   packaged and directly delivered observations distinct; an authorized later
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
test. The complete-package contract remains; the separately adopted §10 updater
acquires an approved whole MSIX, not independent runtime code or a new scanner.

The strict lab setting is not evidence about provider hosts or customer systems,
and this qualification does not authorize an installer to change customers'
global Wintrust policy. W11-W10/S6.4 owns the
[current assignment and outstanding results](../tasks/windows11-integration.md#pe-content-trust-refinement--8-november-2026).

## 6. What the other routes do and do not solve

| Route | Suitable role for this project | Limitation |
| --- | --- | --- |
| **SignPath Foundation** | Possible free OSS signer for required inner files on MSIX or a later selected MSI | Provider selection, eligibility and exact format/provenance policy remain open. This particular service is not mandatory; supported production signing for required artifacts is mandatory before release |
| **Directly distributed complete MSIX** | Selected delivery format and channel direction under OD-05 §7.9 | Signing/provider and target-device trust are deferred and unresolved; local preparation continues |
| **Microsoft Store with a complete MSIX** | Rejected by David on 21 November 2026 because of publisher indemnity obligations | Historical comparison only; no Store account, submission or certification work |
| **Microsoft Store with MSI/EXE** | Rejected Store publication route | Retained comparison: trusted publisher signatures would still be required; not a NevroTick task |
| **Azure Artifact Signing Public Trust** | Paid hosted-signing alternative when the applicant is eligible | Not a free OSS entitlement; identity/geography restrictions apply |
| **Commercial public code-signing provider** | Possible later owner-selected alternative if cost, identity verification and provider policies are acceptable | Obtain current eligibility and price directly; no free availability or automatic fallback assumed |
| **Owner-controlled interim self-signed certificate** | Explicitly accepted interim MSI release profile in section 7.1 | No public CA/Windows publisher trust or warning-free installation promise; channel, installed-code and native compatibility still need qualification |
| **Separate self-signed laboratory certificate** | Controlled installer/privilege/lifecycle experiments | Lab trust substitutions qualify only the recorded experiment, not normal consumer trust or the interim release profile's key custody |

Microsoft's Store MSI/EXE rules require the installer and included PE files to be signed with certificates chaining to an accepted authority; Store MSIX signing is a different path. A provider's willingness to sign an MSI containing an unsigned upstream DLL would not itself satisfy that Store requirement. [S8]

Azure Artifact Signing currently requires a paid Azure subscription. Public Trust individual applicants must be located in the **United States or Canada**; its organization eligibility list is broader but does not include Georgia. Thus an individual applying from Georgia should not plan on this as an available route under the current rules. Choosing an Azure resource region does not change applicant eligibility, and Private Trust is not a substitute for public Windows trust. Recheck eligibility if circumstances or the service change. [S9, S10]

A valid signature also does **not guarantee the absence of SmartScreen prompts**. Reputation and other Windows protections remain separate considerations. Microsoft's current guidance says EV certificates no longer automatically bypass SmartScreen reputation checks. Paying for EV solely to promise warning-free installation would be an unsound plan. [S11]

<a id="61-store-msix-trust-and-publication-boundaries"></a>

### 6.1 Direct MSIX trust and publication boundaries

The [MSIX plan](plans/windows-msix-store.md) and
[OD-05 §7.9](nevrotick-owner-decisions.md#79-direct-msix-distribution-microsoft-store-rejected--21-november-2026)
select direct distribution without Microsoft Store publication. Source/package
preparation continues while signing is deferred. Later local/test-signed
qualification, final signing and direct-download verification remain separate;
no Store-produced artifact is required or expected.

[OD-05 §7.5](nevrotick-owner-decisions.md#75-optional-enhanced-build-provenance--4-november-2026)
keeps enhanced provenance optional. The [provenance record](plans/release-provenance.md)
retains historical Store transformation research; current mandatory custody
covers the actual build, signing and direct publication. No independent rebuild,
GitHub/Sigstore attestation or public proof bundle becomes a new release gate.

Final public MSIX installation needs target-device trust in the package signer.
A self-signed package is not automatically trusted on unchanged consumer Windows;
local trust substitutions establish only their recorded laboratory result.
The ordinary direct-distribution route can use a trusted code-signing certificate
or suitable signing service, but no provider, eligibility or acceptance is
established here. Do not request that customers import development roots or
weaken Windows protection. Resolve this later signing work under the owner's
deferred decision; do not block unrelated development on it. [S35]

Microsoft's package catalog authenticates package content; it does not establish
an independent embedded Authenticode signature on each extracted PE. Preserve
both package and supported individual signatures under section 4.1, with these
separate qualification owners:

- **Sealed code and package integrity:** explicit `uap10:PackageIntegrity` /
  `uap10:Content Enforcement="on"`, protected declared executable/dependency
  content and actual intact/tampered launch observations. No mutable or external
  executable content is admitted. [Plan §3.1](plans/windows-msix-store.md#31-package-integrity-and-sealed-executable-content)
  owns the detailed contract. Its §3.1.1 distinguishes the base ScmFixture's
  Content-on/no-Trusted-Launch profile from ArtworkLab's already authored and
  archive-checked Content-on/Trusted-Launch-true profile. Exact final signed
  catalog and installed enforcement remain open. [S19, S39]
- **Direct-route CIG and other mitigations:** an ordinary publisher signature
  does not satisfy Microsoft/Store/WHQL signer-class requirements. The runtime
  `MitigationOptIn` field explicitly admits those three classes; it is not an
  arbitrary approved-publisher allowlist. Reconcile the exact per-role
  DLL closure, initial/late-load phase, supported policy and effective evidence
  under [plan §3.2](plans/windows-msix-store.md#32-store-compatible-code-integrity-guard)
  and architecture §15.4. Retain the strongest supported compatible setting;
  record concrete incompatibilities and owner dispositions rather than silently
  weakening a role or inheriting an impossible Store-signature assumption. [S20]
- **Forced image integrity and signer compatibility:** `/INTEGRITYCHECK` sets
  `IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY` for load-time signature checking.
  Microsoft's current guidance requires Azure Artifact Signing for files linked
  with it after cross-signing deprecation. This is a separate compatibility
  assessment for the exact image, signer, target Windows profile and load path;
  neither ordinary Authenticode nor MSIX catalog trust proves it. Signing through
  that service also does not itself confer a Microsoft/Store CIG signer class.
  W11-W10/build/security retain applicability, provider/prerequisite and native
  admission evidence under CHECK-16/18/20/23/26. SignPath remains a candidate,
  not selected; this documentation chooses no provider, adds no switch to
  unsigned fixtures and waives no required release protection. Independent
  source/resource work continues while the signing decision remains deferred. [S36]
- **Independent product authorization:** bind package identity, approved release,
  actual installed payload and permitted operation. A valid unrelated signed
  package, including one signed by Microsoft Store, grants no NevroTick
  helper/IPC authority.
- **Final distributed bytes:** preserve prepared, signed and downloaded hashes;
  verify catalog/manifest, every required individual signature, actual imports,
  SBOM/licence inventory and installed identities. Qualify normal and denied
  activation, cryptography, UI, maintenance and recovery using the final direct
  artifact. Include generated/native/prebuilt contributions and actual startup,
  lazy-load, recovery and shutdown module closure under the precise feature and
  activation profile. Resource-only DLLs retain their separate §4.1 handoff;
  C#/CLR, PowerShell and other diagnostic hosts retain separate admission and do
  not inherit Rust child flags. CHECK-16/18/23/26 keep their existing owners.
- **Bundled data:** pinned leap/calendar/catalogue input, generator, normalized
  revision/digest and supported coverage travel in the same complete release.
  A checksum alone is not authentication. Section 10's TUF authority applies to
  the whole complete package, with no data-only downloader or hot update;
  correction preserves protected floors and spent authority.
- **Wrong-clock/offline behavior:** section 5.1 still applies to installation,
  pre-login startup, helper activation, update, repair and removal. Signing and
  timestamping alone do not settle availability or safe refusal.

Use [release-policy §4](release-policy.md#4-distribution-containment-choose-the-actual-store-operation)
for the chosen direct channel's controlled availability, withdrawal and correction.
Actual hosting and access control remain unqualified. Section 10's in-app TUF
route is selected; a competing automatic `.appinstaller` association is not.
External routes retain their own declared update authority and must not become
an automatic fallback after TUF denial. None can be assumed to recall installed copies. Store flights,
Partner Center visibility and Store rollback are historical research only.
David retains exact-candidate release authority; the existing W11-W10/W11-W05
chain owns native and final-artifact evidence. A genuine protection failure is
not waived by direct distribution. MSI remains a later owner choice.

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
subject to its actual qualification and exact-candidate approval; direct MSIX
remains selected under OD-05 §7.9. No key generation, signing or publication has occurred here.

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
| Explicit package-content enforcement, Trusted Launch and role/artifact/feature/activation-specific direct-route mitigations | W11-W10/W11-W05, build and GUI/security verifiers | Plan sections 3.1–3.4 and CHECK-16/18/20/23/26: base ScmFixture has Content on without Trusted Launch; ArtworkLab already authors and archive-checks both. Exact signed/catalog/installed enforcement and full eligible module closure remain open |
| `/INTEGRITYCHECK` signer and loader compatibility | W11-W10/build/security and later signing owner | Section 6.1 records current Microsoft guidance requiring Azure Artifact Signing for this option; exact applicability/prerequisites and actual signer/load results remain open. No provider selection, weaker release profile or source-work block follows |
| Microsoft Store publication | David Osipov | Rejected under OD-05 §7.9; no account, submission, certification or refusal gate |
| Protected source/build candidate and final directly delivered package/catalog/payload identity | Build/release verifier | Bind prepared, signed and downloaded identities and SBOM assets; final signing and direct-artifact qualification remain open |
| Immutable normalized leap calendar/policy and reproducible provenance | Domain/build/release verifier | NV-LP-001 selects the same authenticated complete-release channel; generated data, revision/digest binding and final-artifact evidence remain to be implemented and qualified |
| Valid/tampered/unsigned/substituted/unrelated-Store-package controls | Release/security verifier | Package trust, CIG and NevroTick role authorization require separate evidence; not executed |
| Wrong-clock/offline activation and recovery | Windows/release/security verifier | Section 5.1 cases required on the selected artifact; not executed |
| Every shipped signable artifact, including nonbinary files, plus outer package | Build/package/security verifier | Section 4.1 defines inventory, signing order and expected identities; production provider capability, complete coverage and final delivered-signature preservation are open |
| Optional event-resource DLL through individual signing, packaging and installed resolution | W11-W10/build/package with W11-W05 | Existing separate unsigned resource-only gate and optional archive route do not admit individually signed bytes. Section 4.1 retains the validated resources/no-code invariant and requires a distinct exact final signed handoff under CHECK-18/20/23/26; not yet qualified |
| Embedded PE padding checks and complete final-byte binding | Release/security verifier | Every shipped PE requires its embedded result; section 5.2's strict-policy readback and behavioral cases are not executed |
| Complete MSIX lifecycle on supported Windows, including reset/removal and W32Time restoration | Package/Windows verifier | Native execution remains held; final direct-artifact evidence required before broad release |
| Exact-candidate direct availability and broad release | David Osipov | Direct MSIX and in-app TUF design selected; actual signer/trust/hosting and qualification unresolved; no publication or released artifact established |
| Per-format native signing capability and SignPath eligibility if pursued | Maintainer / provider / verifier | Required inner-file capability also applies to MSIX; missing capability blocks release, not independent source preparation. MSI delivery remains conditional; the OpenPGP-only exception is superseded |
| Interim self-signed MSI, if MSI delivery is later selected | Owner / release / Windows verifier | OD-05 §7.8 accepts the profile; section 7.1 requires approved-key coverage, truthful public-trust failure, actual channel/installed-code qualification and later signer migration. No key or signed artifact is established |
| Stable-support and transition commitment | David Osipov / release owner | [Release-policy §2](release-policy.md#2-support-status-and-proposed-baseline) is proposed; no stable series, support lifetime, backport schedule or response deadline approved |
| Renewal, revocation, access removal and recovery | Release/security owner | [Operational procedures are written](release-policy.md#7-compromised-publisher-repository-or-build-authority); actual private inventory and applicable renewal/access/recovery rehearsals remain pending and required before public release |
| TUF role keys and exact final target authorization | David / release/security owner | Section 10 and architecture §19.8 adopt role-separated release authentication; actual keys, algorithms/provider, thresholds/custody implementation, renewal and compromise recovery remain unqualified. No OpenPGP substitute or key-generation action is established |

**OPEN-11 tracks the selected direct-MSIX route's unresolved signing/trust and qualification.** Signing selection is deferred; source and package preparation continue. MSI remains a later owner choice, and no unsigned/OpenPGP-only exception is introduced. Eventual public distribution requires actual delivered-artifact coverage, installed trust, protection and lifecycle evidence with David's exact-candidate decision. No Store submission or refusal is required. A provider account, certificate screenshot or this guide alone supplies no signing or release pass.

## 9. Sources and related documents

Primary Microsoft sources for the 28 November artifact/signing reconciliation:

- **S36:** [`/INTEGRITYCHECK` load-time checking and current Azure Artifact Signing requirement](https://learn.microsoft.com/en-us/cpp/build/reference/integritycheck-require-signature-check?view=msvc-170), rechecked through Microsoft Learn MCP. This supplies a distinct signing prerequisite, not provider selection or CIG admission.
- **S37:** [Resource-only DLL creation](https://learn.microsoft.com/en-us/cpp/build/creating-a-resource-only-dll?view=msvc-170) and [`/NOENTRY`](https://learn.microsoft.com/en-us/cpp/build/reference/noentry-no-entry-point?view=msvc-170).
- **S38:** [PE format, resource directories and certificate table](https://learn.microsoft.com/en-us/windows/win32/debug/pe-format). The linked repository reader owns the stricter current unsigned shape; the generic format does not impose that project profile.
- **S39:** [Trusted Launch manifest prerequisites and catalog boundary](https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/element-trustedlaunch-trustedlaunch). The linked MSIX plan records current fixture declarations separately from final installed enforcement.

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
implementation, preservation of final signatures or native runtime enforcement.

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

- **S35:** [Direct MSIX distribution and trusted certificates](https://learn.microsoft.com/en-us/windows/msix/app-installer/installing-windows10-apps-web). Public trust, laboratory certificate substitution and package creation are separate; no signing provider is selected by this reference.

## 10. Updater signing and key operations

**Adopted design, 25 November 2026:** [OD-05 §7.10](nevrotick-owner-decisions.md#710-tuf-authenticated-direct-msix-updater--25-november-2026)
and [architecture §19.8](windows-time-architecture.md#198-tuf-authenticated-direct-msix-updater)
select TUF-authenticated complete-MSIX updates. **Check for updates → a verified
eligible target → “Update now?” → Yes** automatically downloads, verifies and
installs that target. Windows UAC/unavoidable OS prompts remain. Signing is an
internal release/admission mechanism, not a separate consumer Download,
detached-signature or routine Install-now step.

David's suggestion that a “PGP key or similar” may be needed expresses an open
signing-design question. It authorizes no creation/export of private keys,
provider account, certificate purchase, signing or publication. The correct
design has two independent cryptographic trust purposes:

| Purpose | Required authority and limit |
| --- | --- |
| TUF metadata/release authorization | Bootstrap and retained TUF root authorize roles/keys; targets bind approved final MSIX bytes and release policy. The client's public root is not a signing secret. HTTPS/DNSSEC/hosting access cannot replace these signatures. |
| Windows package and supported-file signing | The complete MSIX and §4.1's supported shipped files retain their required native signatures and expected identities. Target devices must trust the package signer under the qualified profile. TUF or OpenPGP signatures alone do not satisfy this Windows requirement. |

Microsoft documents required MSIX code signing and device trust separately from
package creation; timestamping does not remove identity/revocation or wrong-clock
qualification. Use the [MSIX signing overview](https://learn.microsoft.com/en-us/windows/msix/package/signing-package-overview)
and §5.1's actual verifier matrix. No customer certificate import, protection
bypass, self-signed-MSI transfer to MSIX or manual detached verification is
introduced. The earlier unsigned-MSI/OpenPGP exception stays superseded.

### 10.1 Role separation and custody decision

The accepted design separates bootstrap/key administration, release approval and
freshness publication. The preceding concrete recommendation is separate
**2-of-3 root** and **2-of-3 targets/release** thresholds with independently
protected custody/failure domains; exact provisioned keys, algorithm/provider,
operational threshold configuration and absence/recovery arrangements still need
admission. David remains the human release approver. Multiple keys in one CI job
or on one compromised everyday device do not provide independent custody or a
second human approval.

| Role | Planned custody and permitted use |
| --- | --- |
| Root | Offline or hardware-protected independently recoverable custody; authorize role/key/threshold transitions. Ordinary build/publication jobs possess no root private key. |
| Targets/release | Separately protected release signing under David's exact-candidate authority; authorize only the complete final qualified package and its policy. Build CI must not hold the release threshold. |
| Snapshot | Restricted online role for metadata consistency; not independent permission to add approved package targets. |
| Timestamp | Separate restricted online freshness publication; bounded expiry/renewal and monitoring owner, without root/targets authority. |
| Windows signer | Separately admitted certificate/service and approval/custody lifecycle for §4.1's inventory; neither its credential nor TUF keys enter untrusted build/test jobs. |

Before creating keys, freeze the maintained publisher/client interoperability
profile, supported algorithms/key formats, exact metadata schema, protected
storage/hardware/provider, independent backups and access/rotation/recovery
operators. Keep private inventory and recovery material outside source, packages,
logs and support exports. Select expiry/renewal periods and the supported
offline/maintainer-absence window from an actual sustainable procedure; this
document chooses no production TTL, provider or personal existing key.
Record the concrete decisions and evidence in [§10.4](#104-publisher-readiness-record--26-november-2026);
an incomplete field remains an open prerequisite for its dependent operation.

### 10.2 Final-byte release and publication order

Use §4.1 and §5.2 inside-out signing and custody. Finish all metadata/resources;
sign/verify each supported inner file and preserve approved upstream signatures;
assemble, sign and timestamp the complete MSIX. Only **after the final authorized
byte transformation** compute target SHA-256 and exact byte length and bind them
to product/package/publisher identity, architecture, channel, version, signed
release sequence/security floor and compatibility policy. An earlier hash or a
fresh checksum read from the delivery host supplies no target authorization.

Approve that exact candidate and sign its TUF target metadata through the
qualified roles. Publish immutable artifacts first, then the required roots and
consistent targets/snapshot dependencies, and publish the current timestamp last
after checking that its referenced objects are obtainable. Retain all numbered
roots needed by long-offline clients; never overwrite a versioned approved
artifact with different bytes. Use the [TUF repository operations](https://theupdateframework.github.io/specification/latest/#repository-operations)
for protocol ordering and the [release policy](release-policy.md) for real
promotion/containment authority.

Versioned objects may use immutable delivery caching; mutable timestamp renewal
needs a compatible bounded cache policy. Host access controls, object retention
and mirror consistency are defense in depth, not release authentication. A
static custom-domain/R2 layout and exact-byte GitHub mirror remain proposals to
qualify. Hosting setup, DNSSEC configuration and immutability have not been
independently observed or performed by this amendment.

The privileged maintenance owner rechecks raw metadata, current protected trust,
time/intent/policy and complete bytes independently, then enforces Windows trust
and protected staging under architecture §19.8. A valid TUF target with the wrong
Windows publisher, or a valid Windows package without eligible TUF authority,
must fail the in-app route. Cross-route Windows/WinGet administration has its own
declared authority and is not a TUF rejection fallback.

### 10.3 Renewal, loss and compromise qualification

Reuse the [release incident owner](release-policy.md#7-compromised-publisher-repository-or-build-authority)
and existing W11-W10 evidence. Qualify normal rotation/renewal and the following
failure cases before relying on their recovery:

- Renew timestamp/snapshot and release metadata without requiring a new binary
  release, with actual scheduled operational ownership and bounded expiry alerts.
  A dormant release still needs metadata maintenance; no monitoring is claimed
  to exist merely because the procedure is written.
- Exercise sequential old/new-threshold root rotation, permitted expired
  intermediates, specified history resets and long-offline catch-up using the
  actual client/provider and retained root chain. Preserve protocol-accepted
  metadata progress even if later download/deployment fails.
- Distinguish lost/inaccessible key material from suspected key compromise.
  Prove backup recovery and replacement with the still-valid authorized
  threshold. Loss of enough root custody may make ordinary in-band rotation
  impossible; define explicit trusted out-of-band recovery rather than silently
  trusting a new downloaded root or reinstalling to reset history.
- For timestamp/snapshot compromise, revoke or rotate through the qualified
  parent/root procedure, bound the affected metadata and restore valid renewal.
  For targets/root compromise, stop promotion, assess unauthorized targets,
  recover authority independently and qualify client recovery against malicious
  version/expiry/root history. Do not promise that sufficient root compromise
  is always repairable in band.
- For Windows signing compromise/expiry, use the actual provider's revocation
  and renewal procedure; qualify publisher/identity transition, trust caches,
  timestamps and affected long-offline clients separately. A request to revoke
  a certificate is not instant universal refusal or recall of installed code.
- Preserve installed-state and host-compromise distinctions. Enough valid TUF
  and Windows authorization can still approve malicious code; keys do not prove
  benign builds. Existing private custody, required SBOM/licence/artifact/native
  gates remain, while optional enhanced provenance stays optional.

All cases above remain **planned/unexecuted** here. This design adopts no key,
library, signer or online service. Trust/time/client/state and surviving Windows
deployment/recovery must both qualify before live apply; signing/native/release
claims require their own exact-artifact evidence afterward.

### 10.4 Publisher readiness record — 26 November 2026

Release/security/W11-W10 own one public-safe readiness record within the existing
[W11 execution inventory](../tasks/windows11-integration.md#442-architectural-closure-execution-inventory--26-november-2026).
It is a concrete output of OPEN-11 under the
[architectural closure inventory](windows-time-architecture.md#221-architectural-closure-coverage--26-november-2026),
not another release authority, key inventory in Git or completed operational pass.
Keep secrets, detailed custody locations, recovery material and unnecessary
personal/access information in the authorized private inventory. The public-safe
record contains only the profile, scope, status and references needed to assess
readiness. Existing [release §7.5](release-policy.md#75-tuf-publisher-operations--25-november-2026)
owns operational response; [its rehearsal evidence](release-policy.md#publisher-rehearsal-evidence--26-november-2026)
records actual outcomes rather than duplicating this specification.

| Record field | Required decision and inspectable evidence |
| --- | --- |
| Client/publisher interoperability | Exact maintained revisions/artifacts, metadata schema and representation, algorithms/key encodings, roles/thresholds and supported client starting roots. Bind valid and invalid signature/root/role fixtures to the actual selected client and publisher. Reference the assessment's per-process graph/time/storage admission; producer compatibility alone does not qualify a consumer or Windows trust. |
| Bootstrap and custody | Approved initial public root and application identity; nonsecret role/key identifiers and threshold configuration; private evidence of independent custody/failure domains, authorized access and recoverable backups. Separate root, release, freshness and Windows credentials. Show how loss of one custody domain differs from loss of sufficient authority; keys sharing one compromised job/device are not independent. |
| Renewal and maintainer absence | Selected metadata/key/certificate expiry and renewal policy, actual renewal operator/automation owner, declared absence coverage, bounded failure alerts and escalation, and permitted actions when the release approver is unavailable. Demonstrate renewal without a binary release. Record feasible recurring work, access, retention and provider/hosting costs before promising coverage; no second approver or delegated release authority is inferred. |
| Offline clients and finite retention | Supported client/version/offline window, required numbered-root availability, oldest supported starting roots, bounded catch-up work, publisher/client storage allocation and retirement/compaction policy. Preserve protocol-required root availability and every authenticated transition needed by supported clients. A support-window change, arbitrary old-root deletion or skipped roots cannot waive protocol requirements to satisfy a resource cap; any unsupported population and recovery route must be explicit before relying on the policy. |
| Final bytes and publication | Exact signed package and supported-file inventory, target digest/length/policy, approval identity, immutable-object and timestamp-last publication order, and retrieval/readback through each admitted route. Record mutable metadata versus immutable target cache policy, mirror consistency, partial-publication handling, bounded retries and rollback/containment ownership without treating host access as release authentication. |
| Windows signer lifecycle | Actual provider/certificate and expected package/publisher identity policy; renewal, revocation, timestamp and successor-signer handling. Reference §5.1's process-specific wrong-clock/offline/cache/delegated-retrieval evidence and native completion custody. TUF key rotation or an updater UTC interval cannot substitute for these observations. |
| Loss, compromise and corrective delivery | Separate recoverable loss, freshness-role compromise, targets/root compromise, Windows signing compromise and host compromise. Record the remaining authenticated recovery authority, response limits, malicious version/expiry/root-history cases, and compatible greater-version delivery. If sufficient authority is lost or compromised, preserve an explicit trusted recovery decision rather than promising in-band repair or ordinary reinstall/bootstrap. |
| Evidence and continuing owner | Candidate/profile identity, date, existing CHECK/AC owner, healthy/failure controls, observed outcomes, private/public evidence references, gaps and next review trigger. Distinguish a written procedure, an inert fixture, an authorized real-role rehearsal, and final distributed/native qualification. Changes to keys, thresholds, algorithms, clients, providers, timing, support window or delivery configuration invalidate the affected evidence. |

Qualify the declared retention and renewal policy together. A dormant release
still consumes operational work; metadata availability must remain compatible
with the supported absence/offline scope and finite client/publisher allocations.
Exercise a missed renewal, unavailable signer, missing numbered root, stale cache
and interrupted publication beside a valid renewal and long-offline client.
Observe recovery using the still-authorized roles and preserved metadata history;
reject-all behavior or a manually reset client supplies no useful pass.

The record must also demonstrate a useful compatible correction for its declared
affected population. A recovered account or newly issued certificate alone does
not establish delivery to an old installed client, a usable Windows helper or
surviving machine recovery. Keep sufficient-authority compromise, offline clients
and already compromised hosts explicit. The state/floor and deployment joins
remain under architecture §19.8 and release §6.

All fields and rehearsals remain **open/planned** until their owning task records
actual decisions and evidence. This amendment selects no key, algorithm, provider,
TTL, operator delegation or service-level promise; creates no credential or
account; and lifts no source, native, signing or release prerequisite. David's
exact-candidate release authority and optional enhanced-provenance policy remain.
