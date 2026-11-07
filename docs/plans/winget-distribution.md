# WinGet distribution: pre-V1 capability and release plan

**Document:** NV-WGT-001, revision 1.0  
**Owner instruction / research date:** David Osipov, 7 November 2026  
**Roadmap stage:** WGT-0, within the existing pre-V1 P6/P7 delivery work  
**Status:** researched and planned; distribution implementation, native qualification and public availability remain open  
**Inspected project:** [8bc7c8ed0091278d4fb6e5d6ea3c6c69ab767772](https://github.com/DavidOsipov/NevroTick/commit/8bc7c8ed0091278d4fb6e5d6ea3c6c69ab767772)  
**Execution/evidence owner:** [W11-W10](../../tasks/windows11-integration.md#w11-w10-re-enter-conditional-mechanisms-only-for-a-concrete-need), with W11-W05, metadata, release, security and independent package QA

## 1. Decision, scope and reconciliation

[OD-17](../nevrotick-owner-decisions.md#21-od-17--winget-capability-before-v1)
places WinGet distribution capability **before the first software release**.
Implement and qualify its preparation, package integration and release operation
within P6/P7. The [roadmap](../../ROADMAP.md#wgt-0--winget-distribution-capability-before-v1)
then retains **V1 release → MT-0 multithreading → Khronos/Ananke → GeoIP → Android**.
WinGet does not become a new Core prerequisite or move post-V1 multithreading.

The existing [OD-05](../nevrotick-owner-decisions.md#7-od-05--signing-and-release-ownership)
selects one complete hardened MSIX containing UI, engine, broker and lifecycle
helper, with a Microsoft Store submission attempt. Preserve that direction.
Qualify WinGet access to the actual Store product through `msstore` first. The
Community `winget` source is a separate route for an authorized public installer;
it is conditional, not a reason to select MSI or create a split product. [S01–S03]

Two results must be recorded separately:

- **WGT-C01 — pre-V1 capability qualified:** real candidate-bound preparation,
  metadata/identity integration, appropriate WinGet installation/lifecycle tests,
  a reproducible release procedure and any applicable automation dry run. A
  document, placeholder YAML or passing syntax check alone cannot close it.
- **WGT-C02 — public source available:** the actual approved product/version is
  discoverable and usable from its specified source after external publication.
  Store visibility, community moderation and catalog propagation have external
  dependencies. A PR, merge or release tag alone cannot close this result.

This separation avoids requiring a community PR to be accepted before its
approved public installer exists. Prepare and qualify the capability before V1;
record public catalog availability at the authorized release operation. Publish
user installation claims only for an observed working source. If an essential
native/package result is unavailable, C01 remains open; an external-publication
wait is not a substitute for that result or an automatic implementation failure.

| Existing obligation | Treatment in this plan |
| --- | --- |
| Complete MSIX and actual Store attempt | Retained. No automatic MSI fallback, standalone service bootstrapper or second product lifecycle. |
| P6/P7, W11-W10/W11-W05 and CHECK owners | Retained; WGT labels organize the distribution slice rather than replacing architecture requirements. |
| Extensive accurate metadata | Extended with source-specific identities and conditional Community projections from the existing metadata authority. No new parallel product record. |
| Public release, account, visibility and credential authority | Remains with David under the release policy. This task records preparation; it performs no submission or publication. |
| CPU/OS floor, service isolation, W32Time recovery, package integrity | Retained at the actual effect, even when WinGet cannot express a prerequisite. |
| Current Core priority and Windows/fuzz/storage/UI holds | Retained. Read-only/source preparation does not authorize a held native experiment. |
| Optional enhanced provenance | Remains optional under OD-05 §7.5. Required artifact custody and ordinary release evidence remain mandatory. |

## 2. What the supplied proposal gets right and wrong

| Supplied claim | Verified correction and project consequence |
| --- | --- |
| WinGet uses YAML in a public GitHub repository. | Correct for the Community route. `msstore` is a distinct Store catalog and uses a Store ProductId; the selected Store product does not need a second Community listing merely to be acquired through WinGet. [S01, S02] |
| MSI, MSIX, EXE and ZIP/portable are supported. | Broadly correct as client capabilities. NevroTick still requires its complete-product service/lifecycle package; a portable EXE or installation script does not meet that requirement. Community script installers are expressly disallowed. [S03, S04] |
| MSI/MSIX make installation completely unattended. | Supported installer types have known installation behavior, but privilege authorization and source/package agreements are separate. Packaged services require administrative installation. Silent mode does not bypass UAC or consent. [S02, S05] |
| `/S` or `/VERYSILENT` can be copied into an EXE manifest. | Switches depend on the real installer technology. Use the specific supported installer type and test its actual switches; do not add generic EXE arguments to MSIX. [S06] |
| Any publicly reachable URL is sufficient. | Use the official publisher release location over HTTPS, reachable by validation and customers without login. A version-specific release-asset URL is preferable to mutable `latest` links, shorteners or expiring tokens. [S04, S07] |
| WinGetCreate is Microsoft's authoring tool. | Correct. It generates/updates manifests, but detected fields still require review against final bytes and the installed product. Tool/schema versions must be admitted together. [S08] |
| Put everything in `manifests/p/Publisher/App/Version/`. | `p` is the lower-case partition for the identifier's publisher initial, not a universal directory. Community submissions use the multi-file format; singleton manifests are prohibited there. One PR contains one package version. [S04, S09] |
| A PAT makes submission automatic and complete. | A supported credential can enable fork/PR operations. It does not approve the package, grant release authority or make the package visible to clients. [S08, S10] |
| Automated checks prove the installer is safe. | Checks cover manifest, origin, hashes, installation and security scans; moderation can intervene. They do not prove NevroTick's clock authority, least privilege or recovery invariants. [S07, S10] |
| PRs usually merge within a few hours. | No end-to-end merge guarantee was established. The repository's typical publishing delay is measured after merge, not from PR creation. Keep external review and propagation explicit. [S10, S11] |
| `vedantmgoyal2009/winget-releaser@v2` is official. | It is third-party; the old URL redirects to `vedantmgoyal9/winget-releaser`. Its update flow requires an existing accepted package, and its inspected execution chain contains floating dependencies. It is not adopted by this plan. [S12] |
| Trigger on every new tag so users always get the latest build. | Tags can precede assets/qualification and include previews. Publish only the approved stable artifact, then verify source visibility. Client upgrade execution, Store update settings, pins, policy and offline users remain separate. [S13–S15] |
| WinGet provides free Microsoft signing. | Community listing does not sign the installer. Store processing signs accepted MSIX/AppX; a direct MSIX needs its own valid target-device trust path. A file hash is not a publisher signature. [S16–S18] |

Confidence is high for these source-documented distinctions. Compatibility of
NevroTick's actual service-bearing package with a selected WinGet/client/OS route
has not been observed and must not be assigned a passing status.

## 3. Current repository baseline and channel choice

At the inspected project pin, the Git tree contains the Rust/security CI workflows
and the implemented [metadata source pipeline](../../metadata/README.md), but no
WinGet publication workflow or checked-in MSI/MSIX installer artifact. The GitHub
Releases API returned no releases during this review. The metadata pipeline
explicitly leaves release numbering, Store identity and full-product qualification
pending. Its current projections are not a working WinGet manifest generator.

The product's maintained records, not the supplied example, own publisher,
licenses and supported platforms. The current target is Windows 11 26H2 x64 and
Windows Server 2025 x64 Desktop Experience, with the x86-64-v3 floor. Core/engine,
original GUI and brand rights have separate authorities. Do not label the complete
product MIT, invent a single GPL grant, or fill pending support/privacy/license
URLs with plausible values.

| Route | Plan and required evidence | Identity |
| --- | --- | --- |
| Selected Store MSIX through `msstore` | First route to qualify. Observe actual Store-source availability and authorized installation of the complete package, services, user registration, updates and removal on each supported platform. Store acceptance alone is not this observation. | Actual Store ProductId plus MSIX identity/PackageFamilyName, publisher and exact delivered version. |
| Community `winget` source | Prepare the contract; activate only when David authorizes a compatible, public, trusted complete installer and its distribution. A self-hosted MSIX needs an established signing/trust route; a Store download does not automatically establish rehosting rights or a suitable permanent URL. | Reviewed Community PackageIdentifier, version, immutable installer URL/hash and actual MSIX or MSI installed identity. |
| MSI alternative | Remains conditional on the existing owner reconsideration after a concrete Store blocker/refusal. If selected later, qualify the complete MSI, silent/elevated lifecycle, signatures and installed identity. | Actual ProductCode/UpgradeCode/Apps & Features values and separate release mapping. |

Record `source + identifier + package identity + version + scope + platform` as a
single tested route. A display-name match is insufficient. Never present a proposed
`DavidOsipov.NevroTick` identifier or an illustrative Store ID as registered.
Cross-source or cross-format migration requires explicit detection, trust and
one-controller lifecycle evidence; it is not an alternate URL edit.

## 4. Metadata and artifact contract

Reuse [metadata/product.toml](../../metadata/product.toml), its code-owned schema,
and [the metadata plan](windows-package-metadata.md). Extend a supported projection
only in its admitted implementation slice. Keep derived artifact observations and
publication receipts distinct from authored product facts.

| Field family | Authority / check |
| --- | --- |
| PackageIdentifier, Store ProductId, MSIX identity/PackageFamilyName | Distinct namespaces. Confirm actual assigned/reviewed identity and source; preserve update continuity. |
| PackageVersion and Windows version | Approved release mapping plus real package and installed readback. Preserve monotonic upgrade behavior and handle differing display versions explicitly. |
| PackageName, Publisher, Author and descriptions | Reviewed public product facts; component authors and signer identity are not interchangeable. Describe implemented behavior. |
| PublisherUrl, PublisherSupportUrl, PrivacyUrl, PackageUrl and LicenseUrl | Real maintained public endpoints approved for those purposes. Pending remains pending, not a fabricated link. |
| License, Copyright and notices | Exact shipped component/asset grants and approved aggregate description. WinGet metadata grants no extra rights. |
| PackageLocale, additional locales, tags and release notes | Accurate listing translations, useful supported tags and actual release facts. Listing languages do not establish UI translation. |
| Icons and documentation links | Applicable public assets/URLs. The inspected schema permits PNG/JPEG/ICO icons, not AVIF; use an approved derived asset with the existing brand notice if needed. [S19] |
| InstallerType, Architecture, Scope and ElevationRequirement | Actual installer/installation behavior; do not infer machine scope from the existence of a service or suppress real elevation. |
| MinimumOSVersion and dependencies | Reconcile numeric package requirements and the actual client/source/OS dependency closure. Preserve the two-family support policy independently. |
| InstallerUrl and InstallerSha256 | Exact final distributable after signing/processing; re-download and compare with the approved receipt. Do not hash an earlier unsigned candidate. |
| SignatureSha256 for applicable MSIX manifests | Derive with the admitted MSIX hash tooling from the real artifact. It does not replace signature/trust verification. [S18] |
| AppsAndFeaturesEntries, ProductCode, UpgradeCode | Use when applicable and observed. Verify correlation, uninstall and N→N+1 detection; do not manufacture MSI codes for an MSIX product. |
| InstallModes, InstallerSwitches, return codes, reboot and upgrade behavior | Encode observed semantics. Zero exit, reboot-required and partial/in-use failure must not be conflated. Unsupported optional fields remain absent with a reason. |

For an activated Community route, generate a version manifest, installer manifest
and default-locale manifest, plus warranted locale files, beneath the correct
identifier/version directory. Review the complete set. Restrict public output to
an explicit allowlist; internal field registers, credentials and personal/operational
metadata are not publication inputs. [S06, S09]

Freeze an approved asset identity and do not replace its bytes under the same
version URL. A rebuild, signature change or Store transformation requires new
hashes and affected qualification. A published release name alone cannot select
the artifact: match source/tag/commit, version, asset name, hash, package identity,
architecture and existing release approval. Follow the [release policy](../release-policy.md).

## 5. Platform, privilege and lifecycle boundaries

### 5.1 Schema support is not complete product applicability

The inspected frozen installer schema lists broad architectures, including `x64`,
but no x86-64-v3 ISA tier. The accompanying documentation says `Platform` and
`Channel` do not currently provide the assumed selection behavior. WinGet can
also select emulated architectures. These are schema/source findings, not a
native execution result. [S20, S21]

Follow the [two-family qualification matrix](../../tasks/windows11-integration.md#432-two-family-windows-qualification-matrix--5-november-2026):

- Record real OS builds, editions and Desktop Experience; do not invent a 26H2
  build number. A single numeric lower bound cannot encode the complete Windows
  11 26H2/Server 2025 union. In particular, a newer desktop minimum could exclude
  the selected Server build. Align package applicability and independent product
  admission without silently broadening the support claim.
- The baseline-compatible admission path must precede every v3-dependent payload
  or startup dependency, including direct launch, SCM, helper and update paths.
  Check the complete v3 feature requirement and usable OS vector state. A v3
  program that traps before checking is not a safe prerequisite check.
- Qualify below-floor rejection and valid-v3/no-AVX512 success. No AVX512 requirement,
  ARM64/emulation support or weaker CPU floor is added by this plan.
- Do not invent a general MSIX pre-install hook. Where deployment can register a
  package on an unsupported target, observe the resulting inert/refused state and
  supported cleanup. Require rejection before incompatible execution, functional
  service exposure or W32Time/clock mutation; do not claim YAML prevented all
  registration.
- A Community validator running an older or ineligible environment is an explicit
  compatibility/review issue. Record exact failure and the supported matrix;
  neither lower the floor nor remove guards to obtain a green check.

Microsoft lists WinGet on Windows Server 2025, but that does not establish
`msstore` availability for this product, service-MSIX behavior, Server Core or
every enterprise policy profile. Each admitted source/platform row needs its
own observation. A Windows Sandbox may not have WinGet or Store preinstalled;
use only an admitted reproducible setup, and retain full-VM lifecycle evidence
where Sandbox cannot exercise the requirement. [S03]

### 5.2 Installation authority and recovery

Keep the selected [MSIX lifecycle contract](windows-msix-store.md#7-bounded-continuation-and-existing-acceptance-owners)
and architecture §19.4. Noninteractive installation still requires legitimate
administrative authorization for services. Qualify standard-user initiation,
approved elevation and denied elevation; the ordinary UI remains unelevated.
Required source/package agreements need legitimate acceptance in the actual
installation context. Do not use prompt suppression as evidence of consent.

Keep time services and distribution orchestration separate. Microsoft's WinGet
CLI is unsupported in LocalSystem context; a documented PowerShell-management
alternative does not adopt a new NevroTick updater. No polling `winget upgrade
--all` job, SYSTEM broker invocation, scheduled task or custom privileged updater
is introduced. [S22]

Preserve pre-exposure service identity/token/ACL/protection admission, one active
clock controller, no-login operation and W32Time ownership/restoration. Test
install, repair where supported, update, Reset, interrupted servicing and removal
through the actual package/source path. A process exit code alone cannot prove
that services, user registration and mandatory recovery state are correct.

Retain separate local test-signed, submitted, Store-delivered and installed
identities. Test roots can support admitted lab mechanics, not a public
installation claim. Never disable hash verification, certificate checks,
Defender, required mitigations or service controls to satisfy distribution tests.
Acquisition itself needs working network/TLS/trust conditions; an uninstalled
time client cannot promise to repair the clock that prevents its download.

## 6. Release automation design

For the selected Store route, reuse the existing Store release process and add
source-specific discovery/installation receipts. A Community manifest publisher
is needed only if that route is activated. Do not create an inert secret-bearing
workflow while the installer, route and identities are unresolved.

| Stage | Required behavior and output |
| --- | --- |
| Build/package owner | Produce the complete qualified artifact, notices and release receipt under existing P6 rules. Complete signing/processing before final distributable hashes. |
| Preparation job | Read approved release metadata and explicit public fields; validate URL/origin/identity/hash and generate or update the complete manifest set. No external-write credential is needed. |
| Verification job | Use the admitted client, generator and schema; reject missing/extra/changed metadata and exercise appropriate native lifecycle cases. Retain source/artifact/tool identities and redacted results. |
| Authorized publication job | Consume the reviewed frozen output, recheck the actual approved release and artifact hash, then submit only the intended version if the route is activated. Human release authority remains OD-05; automation does not add another approval to already-authorized routine preparation. |
| External outcome observer | Record PR URL/version, validation/moderator outcome, merge and source publication separately; test exact-source discovery after propagation. A failed submission cannot label the release WinGet-available. |
| Maintenance owner | Triage failed/duplicate submissions, expired credentials, wrong identity, stale catalogs, security removals and withdrawn artifacts through the existing release/incident record. |

Use an approved **published stable release with final assets**, or a dependent
release job carrying the same bound identity, rather than a raw tag push. Explicitly
reject draft, preview, wrong-repository, wrong-version and incomplete-asset events.
`Channel: beta` is not preview isolation. A future preview channel requires its
own admitted identity/update design. [S13, S21]

GitHub suppresses most workflow-triggering events produced with the ordinary
`GITHUB_TOKEN`. Therefore a release made by that token cannot be assumed to start
a separate release-triggered publisher. Use a qualified dependent/reusable job or
explicit supported dispatch; verify the actual event chain. Do not add a more
powerful credential solely to conceal a broken trigger design. [S14]

The repository token is scoped to its repository, not arbitrary writes to a
fork/upstream catalog. Current WinGetCreate documentation supports a classic PAT
with `public_repo` and recommends `WINGET_CREATE_GITHUB_TOKEN`; it does not support
fine-grained PATs. Classic scopes have broad public-repository reach. Bind any
necessary credential to the controlled publication context, use minimum supported
scope and lifetime, redact it and plan rotation/revocation. Do not add `workflow`
scope by default. A GitHub App or other narrower alternative needs demonstrated
compatibility with the chosen submission path before selection. [S08, S15]

Pin actions to reviewed full commit SHAs and downloaded tools to reviewed versions
and verified bytes, including transitive installation steps. Satisfy the existing
[workflow policy](../../.github/security/check_workflows.py) and security gate.
Avoid untrusted PR execution with publishing secrets, downloaded scripts executed
as setup, ambient cached tokens and credentials in command arguments/logs. Do
not execute an installer on a credential-bearing publisher merely to inspect it.

The suggested third-party action is not selected. Its inspected update path
requires prior accepted manifests and published assets; its wrapper uses a
floating action/tool installation chain. Pinning only its outer SHA does not
freeze those dependencies. The plan favors reviewed Microsoft WinGetCreate
operations with a qualified fixed tool chain; this preference is not evidence
that a particular future binary/schema combination has passed. [S08, S12]

Make retries idempotent by package/source/version and approved asset hash. Check
for an existing PR or published version first; update the intended work or report
a conflict, without duplicate PRs, force-overwriting another contributor or
replacing released bytes. Serialize publication for the same package. Bound
retries and retain the last failure; there is no indefinite polling process.

## 7. Tool and command qualification

Research observed WinGet stable `v1.29.380`, WinGetCreate `v1.12.13.0`, numbered
schema `1.28.0` and older `1.12.0` Learn examples. The Community template recommends
1.28.0, while the inspected WinGetCreate release model defaults to 1.12.0 and can
normalize updated manifests to its supported model. These observations do not
select a universally compatible tool tuple. [S06, S08, S09, S20, S23]

Before implementation, freeze the client/App Installer, WinGetCreate, accepted
schema, validation tooling, Windows/SDK and source-policy versions. Use the least
sufficient accepted schema and prove that generation/update preserves required
metadata. Do not merely change `ManifestVersion` to advertise unsupported fields.
Current upstream `master`, an example and an installed release are different
evidence states.

The following are **command forms for later qualification**, not executed
instructions or a populated release manifest. Replace placeholders with actual
reviewed values; verify options against the admitted tool version.

```powershell
# Discovery of the selected Store product (read-only)
winget show --id <actual-store-product-id> --exact --source msstore

# A separately authorized native test of the real route
winget install --id <actual-store-product-id> --exact --source msstore

# Conditional Community authoring; choose the admitted tool version first
winget install --id Microsoft.WingetCreate --exact --source winget --version <admitted-wingetcreate-version>
# Initial authoring: decline the wizard's final submission prompt for review
wingetcreate new <official-version-specific-installer-url>
# Later accepted Community versions: prepare files without --submit
wingetcreate update <verified-community-id> --version <version> --urls <official-version-specific-installer-url> --out <candidate-directory>
winget validate --manifest <directory-containing-the-complete-manifest-set>
winget hash <final-msix-path> --msix
```

Local manifest installation requires the documented LocalManifestFiles setting
in the owned test fixture. Restore the fixture setting afterward. Local success
does not prove Store discovery, community acceptance or final Store trust. Test
the default silent-with-progress behavior and `--silent` as appropriate to the actual client/type;
never infer suppression of UAC, agreements, reboot or lifecycle requirements.
Public instructions use the observed exact source/ID and qualified flags, not
these placeholders. [S02, S18]

## 8. Ordered implementation packages

Use one existing task/evidence chain. Expand only the next admitted package;
the complete product and unlifted native holds can block dependent execution
without blocking useful source preparation.

| Package / owner | Action and observable exit |
| --- | --- |
| WGT-W01 — release/packaging coordinator | Reinspect current source, metadata, OD-05/14/17, real package status and holds. Establish the Store-first route and exact pending identity/artifact questions. Record the smallest next source slice; do not invent a runnable manifest. |
| WGT-W02 — metadata/build owner | Implement the reached source identity/readback contract and, only for an activated Community route, deterministic multi-file projection/update. Preserve private/public boundaries and truthful pending states. Independent valid and malformed/changed controls exercise the real producer and consumer. |
| WGT-W03 — W11-W10/W11-W05 and QA | On admitted disposable targets, qualify the actual WinGet/package path and complete lifecycle using the matrix below and existing CHECK owners. Record unsupported rows and exact-client limits. A source-only result leaves this work open. |
| WGT-W04 — release/security owner | Prepare and dry-run the selected release operation. For Community submission, qualify artifact custody, supported credential design, trigger behavior, frozen tools, duplicate/conflict behavior and failure recovery without public writes. |
| WGT-W05 — independent reviewer/coordinator | Reconcile exact candidate/tool/platform receipts and remaining blockers; record WGT-C01 before V1 release approval. Scope local, Store and Community evidence separately. |
| WGT-W06 — David/release owner | At authorized publication, perform the selected source operation and observe WGT-C02. Later versions reuse the tested update path and triage policy; listing/merge alone is not an installed recovery result. |

**Next executable stage now:** WGT-W01 source/identity preparation under existing
P6 ownership. The current request records this plan and routing, not execution
of held native work, a new dependency, credential creation, signing or publication.
Before any later bounded experiment record its exact candidate, environment,
success/refusal expectations, elapsed-time/retry limit and cleanup. Stop for the
concrete failed prerequisite; do not repeatedly probe or redesign unrelated work.

## 9. Independent acceptance cases

These cases are planned, not executed. Map observations to existing
CHECK-5/12/13/15/16/18/20/23/24/26/27 and the affected metadata/release owners;
WGT-T labels are local navigation, not another general acceptance catalog.

| Case | Independent observation and useful control |
| --- | --- |
| WGT-T01 — final artifact custody | Known approved package downloads with exact identity/hash; wrong URL, replaced asset, signature, version or publisher fails. Hash the final signed/delivered form. |
| WGT-T02 — projection and schema | Valid complete metadata survives generation/update; missing/duplicate/unsupported fields, wrong version model, stale inputs and private-field leakage fail. Independently compare actual artifact and installed identity. |
| WGT-T03 — clean supported platforms | Complete package works on each admitted Windows 11/Server Desktop Experience row, recording client/source/scope, CPU, OS and dependency state. Sandbox setup is not a substitute for an untested row. |
| WGT-T04 — elevation and noninteractive behavior | Authorized install succeeds; denied elevation/agreements and unavailable dependencies leave a defined recoverable state. Ordinary UI stays unelevated; no hidden first-launch provisioning prerequisite. |
| WGT-T05 — CPU/OS admission | Baseline-safe guard rejects below-v3/unsupported OS and excluded emulation before incompatible execution or protected effects; valid-v3 without AVX512 succeeds. Observe any package registration and its cleanup honestly. |
| WGT-T06 — service and user registration | Correct service identities/ACLs/startup and multiuser UI registration; wrong package/peer/copied helper cannot acquire privileged authority. Observe no-login behavior and direct bypass paths. |
| WGT-T07 — upgrade and detection | N→N+1, skipped-version and already-installed cases preserve configuration, identity and one controller. Detect actual installed versions; pins/user policy remain respected. |
| WGT-T08 — busy/interrupted servicing | In-use files, stop delays, cancellation, reboot-required and interrupted update report correct outcomes and preserve mandatory state/current ownership. No automatic uncontrolled reboot. |
| WGT-T09 — removal and recovery | Supported uninstall/Reset/repair and last-user paths preserve recovery until W32Time/clock ownership is safely settled. Retained data and cleanup match the declared policy. |
| WGT-T10 — source/channel isolation | Explicit ID/source selects the intended product. A similarly named package, wrong source, preview event or unsupported cross-format route is refused by the release operation. |
| WGT-T11 — automation and credentials | Authorized frozen release produces a reviewable result; draft/missing assets, wrong hash, untrusted PR, token expiry, duplicate PR and conflicting version stop safely. Test actual trigger propagation. |
| WGT-T12 — external publication and withdrawal | Observe PR/Store outcome, catalog visibility and source-specific acquisition separately. A rejected/delayed/withdrawn release does not produce a false availability claim or pretend to recall installed copies. |

Reuse unchanged evidence only with an explicit source/artifact/platform identity
argument. Changes to signing, package family, dependency closure, service
configuration, source, scope, tool version or platform invalidate affected cases.
Required missing native evidence is incomplete, not not-applicable.

## 10. Completion, maintenance and next-agent record

Close **WGT-C01** only when the selected capability has a real implementation,
reviewed metadata/identity contract, reproducible preparation/release operation
and all applicable pre-publication native/package receipts. Record exactly which
Store/catalog observations await an externally available approved product;
that dependency does not turn local/test-signed results into a Store pass.
P6/P7 still require their actual final Store-delivered artifact and lifecycle
evidence before broad release. A source-only plan cannot satisfy either gate.

Close **WGT-C02** only with actual source-specific discovery and successful
acquisition/identity observations for the approved public version. If source
availability is pending at release, state that limitation and retain its owner;
never publish a non-working `winget install` promise. A material blocked route
returns to the existing owner-decision process with evidence, rather than
silently changing packaging, dropping a supported platform or weakening guards.

Use the [release policy](../release-policy.md) for bad-version withdrawal,
credential compromise, compatible higher-version correction and retained support
populations. Community removal, Store visibility changes and GitHub asset actions
do not uninstall or repair existing clients. If two channels are ever admitted,
track each actual published identity and recovery reach. No automatic downgrade,
forced update or continuous monitoring is introduced.

The task receipt must contain source/diff, package/manifest hashes, exact tools
and schema, route/ID/scope, OS/CPU/identity, observed outcomes, raw evidence links,
unexecuted rows, open external decisions and one next action. Keep credentials
and internal-only metadata out of public receipts.

### Research and review record

Two completed specialist research passes covered Microsoft Learn/package
contracts and GitHub tooling/automation; a separate judge reviews the integrated
candidate. Fresh-context research launches failed before usable output; the
completed researchers and judge use inherited context. They are distinct agents,
not a blind-review claim. Research includes primary source inspection only;
no Windows execution, installer creation, trust-store change, signing, credential
creation, Store action or Community PR occurred.

**Completed integrated review, 7 November 2026:** the distinct judge
`/root/winget_judge` returned **READY to record this research/plan and pre-V1
routing**, with no material finding. This is documentation readiness only;
implementation, native qualification and public availability remain open.
The review inspected the ten-file candidate, current authority/skills, both
specialist reports and decisive primary sources. It independently matched every
candidate file's size, SHA-256 and Git blob identity.

- Reviewed diff SHA-256: `36905c951810d48e0e1a74e8ddc330d9ee1165c15c06db673004ab3561ae1498`.
- Reviewed plan SHA-256: `b9dcbb66eee234ffe240513b2a8b0ffa6afe5b82f88a460356a8996db754b3a0`.
- Raw judge review SHA-256: `8f2c9c833232462c13bb4f4080f2f7a538b001a48869fda49e06f99c1d35ec71`.
- **WG-N01**, nonblocking example correction: the WinGetCreate acquisition
  command now includes `--version <admitted-wingetcreate-version>`. It retains
  the independent artifact/digest verification requirement. This is the only
  technical edit after the reviewed candidate; the remaining addition is this
  attributed closeout.

The judge's comprehension paragraphs are retained verbatim:

**Proposed change:** The candidate places WinGet capability in the existing P6/P7 delivery work before V1 and gives the Windows packaging task a concrete preparation, implementation and qualification sequence. It keeps the complete Store MSIX as the selected product and evaluates its real `msstore` route first; Community manifests require an authorized trusted public installer. It separates candidate-bound capability from public catalog availability, assigns metadata and release-operation responsibilities, and retains multithreading as the first job after V1, before Khronos. Recording this plan neither publishes the product nor declares the capability implemented.

**Current system:** At the inspected project revision, NevroTick has Rust/security workflows and a bounded metadata source pipeline, but the tree has no WinGet publisher workflow or MSI/MSIX installer artifact, and the Releases API returned no releases when checked. Metadata source outputs leave real package identity and release qualification pending. Existing authority selects one complete hardened Store MSIX with services, requires safe x86-64-v3 and two-family Windows admission, and preserves service isolation, package trust and W32Time recovery. Those obligations and the current execution holds remain in force; no new updater or MSI route follows from WinGet research.

The permitted next action is documentation integration under the existing
current-head lease; later WGT-W01–WGT-W06 execution retains the plan's prerequisites.

## 11. Primary sources and refresh rules

Sources checked 7 November 2026. Repository pins preserve this research; recheck
mutable submission policy, supported releases and exact tool behavior at the
implementation/publication stage. A source description is not product evidence.

- **S01:** [Microsoft Learn: WinGet sources](https://learn.microsoft.com/en-us/windows/package-manager/winget/source).
- **S02:** [Microsoft Learn: install](https://learn.microsoft.com/en-us/windows/package-manager/winget/install).
- **S03:** [Microsoft Learn: WinGet availability and formats](https://learn.microsoft.com/en-us/windows/package-manager/winget/).
- **S04:** [Community policies, pinned](https://github.com/microsoft/winget-pkgs/blob/6398a6bb5248b8e884a17390eb120c6c9b31be3f/doc/Policies.md).
- **S05:** [Microsoft Learn: packaged services](https://learn.microsoft.com/en-us/windows/msix/packaging-tool/convert-an-installer-with-services).
- **S06:** [Microsoft Learn: manifest authoring](https://learn.microsoft.com/en-us/windows/package-manager/package/manifest).
- **S07:** [Microsoft Learn: submission and validation](https://learn.microsoft.com/en-us/windows/package-manager/package/winget-validation).
- **S08:** Microsoft WinGetCreate [v1.12.13.0 release](https://github.com/microsoft/winget-create/releases/tag/v1.12.13.0), with pinned [token guidance](https://github.com/microsoft/winget-create/blob/1aa9b6637ca872911b84639d89526f5fe67a06c3/doc/token.md), [new](https://github.com/microsoft/winget-create/blob/1aa9b6637ca872911b84639d89526f5fe67a06c3/doc/new.md), [update](https://github.com/microsoft/winget-create/blob/1aa9b6637ca872911b84639d89526f5fe67a06c3/doc/update.md), [submit](https://github.com/microsoft/winget-create/blob/1aa9b6637ca872911b84639d89526f5fe67a06c3/doc/submit.md), [model defaults](https://github.com/microsoft/winget-create/blob/1aa9b6637ca872911b84639d89526f5fe67a06c3/src/WingetCreateCore/Models/VersionManifestModels.cs) and [update normalization](https://github.com/microsoft/winget-create/blob/1aa9b6637ca872911b84639d89526f5fe67a06c3/src/WingetCreateCLI/Commands/BaseCommand.cs).
- **S09:** [Community authoring](https://github.com/microsoft/winget-pkgs/blob/6398a6bb5248b8e884a17390eb120c6c9b31be3f/doc/Authoring.md), [contribution guidance](https://github.com/microsoft/winget-pkgs/blob/6398a6bb5248b8e884a17390eb120c6c9b31be3f/CONTRIBUTING.md) and [schema recommendation](https://github.com/microsoft/winget-pkgs/blob/6398a6bb5248b8e884a17390eb120c6c9b31be3f/.github/PULL_REQUEST_TEMPLATE.md).
- **S10:** [Community validation process, pinned](https://github.com/microsoft/winget-pkgs/blob/6398a6bb5248b8e884a17390eb120c6c9b31be3f/doc/Validation.md).
- **S11:** [Community FAQ, pinned](https://github.com/microsoft/winget-pkgs/blob/6398a6bb5248b8e884a17390eb120c6c9b31be3f/doc/FAQ.md).
- **S12:** WinGet Releaser [README](https://github.com/vedantmgoyal9/winget-releaser/blob/b7370d4d152efdce803a26cc6ca9eee58cb2ad54/README.md) and [action dependency chain](https://github.com/vedantmgoyal9/winget-releaser/blob/b7370d4d152efdce803a26cc6ca9eee58cb2ad54/action.yml), pinned at the inspected revision; third-party source, not Microsoft policy.
- **S13:** [Microsoft Learn: upgrade](https://learn.microsoft.com/en-us/windows/package-manager/winget/upgrade).
- **S14:** [GitHub: GITHUB_TOKEN scope and event behavior](https://docs.github.com/en/actions/concepts/security/github_token) and [release triggers](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows#release).
- **S15:** [GitHub: workflow security](https://docs.github.com/en/actions/reference/security/secure-use).
- **S16:** [Microsoft Learn: Store package requirements](https://learn.microsoft.com/en-us/windows/apps/publish/publish-your-app/msix/app-package-requirements).
- **S17:** [Microsoft Learn: MSIX signing](https://learn.microsoft.com/en-us/windows/msix/package/signing-package-overview).
- **S18:** [Microsoft Learn: hash command](https://learn.microsoft.com/en-us/windows/package-manager/winget/hash).
- **S19:** [Frozen default-locale schema 1.28.0](https://github.com/microsoft/winget-cli/blob/afc9a471496c4cd80ac363df269362f84ea5a457/schemas/JSON/manifests/v1.28.0/manifest.defaultLocale.1.28.0.json).
- **S20:** [Frozen installer schema 1.28.0](https://github.com/microsoft/winget-cli/blob/afc9a471496c4cd80ac363df269362f84ea5a457/schemas/JSON/manifests/v1.28.0/manifest.installer.1.28.0.json) and [numbered versus development schema guidance](https://github.com/microsoft/winget-cli/blob/afc9a471496c4cd80ac363df269362f84ea5a457/schemas/JSON/manifests/README.md).
- **S21:** [Installer-field behavior, pinned](https://github.com/microsoft/winget-pkgs/blob/6398a6bb5248b8e884a17390eb120c6c9b31be3f/doc/manifest/schema/1.28.0/installer.md).
- **S22:** [Microsoft Learn: troubleshooting and SYSTEM context](https://learn.microsoft.com/en-us/windows/package-manager/winget/troubleshooting).
- **S23:** [WinGet stable v1.29.380 release](https://github.com/microsoft/winget-cli/releases/tag/v1.29.380); observed release identity, not a NevroTick native qualification.
