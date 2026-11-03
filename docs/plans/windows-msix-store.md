# Complete MSIX and Microsoft Store: feasibility and signing research

**Status:** source-reviewed candidate; no packaging or release-channel adoption.  
**Recorded:** 3 November 2026 (UTC+04:00); sources checked 2 November 2026 UTC.  
**Repository basis:** `070927855ed99e1a0683d9f291d3462d33343118`.  
**Method:** Microsoft Learn MCP searches and full-page retrieval, supplemented by
the official SignPath conditions. Search snippets were not the final evidence.

## 1. Decision, authority and evidence limits

**An accepted complete Store MSIX can remove the need to purchase and manage a
public package-signing certificate.** Store hosting, updates and package identity
also make this a valuable distribution candidate. These benefits do not establish
that NevroTick's service isolation and recovery lifecycle fit that channel. [S01,
S02, S08]

Retain the selected **one complete per-machine MSI**, its
[OD-05 release profiles](../nevrotick-owner-decisions.md#7-od-05--signing-and-release-ownership),
and the [complete-MSIX gate](../windows-time-architecture.md#194-future-complete-msix-gate).
A future MSIX must contain the UI, engine, broker, helper and required payloads.
No MSI-service/MSIX-UI split, separate service prerequisite, broader LocalSystem
broker or hidden orphan component follows from free signing.

This report records documented facts, project-specific inferences and unresolved
questions separately. It establishes no Store permission, applicant eligibility,
package build, native execution, offline activation or release approval. Current
portable Core priority, Windows execution stop, separate fuzz no-retry hold,
storage-experiment pause and UI-M0/D1–D3 boundaries remain unchanged. The existing
[Windows task](../../tasks/windows11-integration.md#w11-w10-re-enter-conditional-mechanisms-only-for-a-concrete-need)
owns continuation under W11-W10, with W11-W05 for lifecycle and
[development §8.5](development-plan.md#85-windows-11-integration-research-and-qualification)
for phase placement. This is not another acceptance catalog or execution plan.

## 2. Distribution economics and publisher identity

| Route | Documented responsibility | Consequence |
| --- | --- | --- |
| Complete MSIX through Store | Microsoft re-signs after certification; no developer CA-trusted certificate, signing token or HSM purchase is required for this submission route. [S01, S02] | Genuine free signing for the accepted Store package. |
| Complete MSI/EXE listed in Store | Publisher supplies an immutable versioned HTTPS installer URL; installer and all included PE files need trusted Authenticode signatures. Microsoft does not re-sign them. [S03, S08] | Listing the current MSI does not solve signing. OpenPGP sidecars do not satisfy this Store requirement. |
| Developer-built MSIX distributed directly | Publisher establishes package signing and target-device trust. [S01, S02] | Store is not a general free certificate for arbitrary external builds or portable binaries. |
| Complete MSI through SignPath Foundation | Free managed signing for accepted OSS projects, subject to provider conditions. [S27] | Existing conditional alternative; compare its actual eligibility and implementation cost with complete MSIX. |

The verified MSIX submission fact is absence of a **CA-trusted developer
certificate requirement**. It does not qualify every unsigned upload/container
combination accepted by every packaging tool. Manifest name, publisher and other
required identity values must exactly match Partner Center, including case and
punctuation. Native/manual packaging remains a future build task. [S02]

### Account onboarding

Current dedicated Microsoft pages waive the former **$19 Individual** and
**$99 Company** registration fees in the new flow, starting at
<https://storedeveloper.microsoft.com>. The Individual FAQ warns that direct
Partner Center, Xbox or Visual Studio entry can show the legacy flow. Recheck
these dated terms when onboarding; no refund entitlement is established. [S04,
S05]

Free registration still requires verification. Individual onboarding uses
government ID/selfie and a personal Microsoft account; Company uses business and
employment verification. Account type follows the actual publisher's activity
and identity, not merely the app's free/OSS status. Store policy §10.14 can require
Company for professional/business publication or a business-like identity;
Individual-to-Company conversion is unsupported. [S04–S07]

Country-specific developer/ID eligibility was not established for every market.
The Individual page's broad market count, consumer Store availability and
supported UI languages do not prove a particular applicant's eligibility.
Resolve the actual publisher and country before reserving production identity;
record verification privately without placing identity documents in this repo.
Opening a Company account does not itself grant restricted capabilities. [S04–S07,
S14]

### Value beyond the certificate

MSIX can retain the native Rust/Win32 product; packaging does not require a UWP or
WinUI rewrite. Store supplies package hosting/updates and a stable application
identity. Full-trust desktop packaging is not an AppContainer boundary and does
not grant elevation or clock authority. [S08, S14, S30]

**Inference:** less custom delivery/update code could be a larger benefit than
certificate savings. Product-specific state migration, service authority and
recovery remain engineering responsibilities; no measured effort reduction is
claimed.

## 3. Package trust and the privileged helper

Microsoft documents a **common catalog signature covering the files of an MSIX
application**. App Control can derive a signer rule from the package or its
installed `AppxSignature.p7x`. This provides real content trust, beyond a signature
on an otherwise unrelated outer archive. [S12]

That evidence does not establish an independently usable embedded Authenticode
signature in every EXE/DLL, or retained trust after copying a member elsewhere.
An eventual MSIX trust profile should qualify package/catalog verification rather
than automatically requiring all MSI-specific embedded-signature assumptions.
Neither a missing embedded signature nor a successful generic trust call alone
settles that profile.

**Project inference:** the intended helper must belong to the approved installed
NevroTick package/release and permitted operation. An unrelated legitimately
Store-signed executable remains unauthorized, just as an unrelated
Foundation-signed executable does under the current guide. Preserve process/IPC
identity and protected installed paths; do not use a broad Microsoft/Store signer
class as project authorization. No particular package-aware verifier recipe was
qualified here.

Windows process signature policy exposes Store-signed-image modes. This is a
positive candidate for the future mitigation design, not proof that the actual
Rust/native DLL closure loads under the selected policy. Test intended imports
and allowed workflows beside tampered, copied-out and wrong-project artifacts.
Keep CIG, package identity, helper authorization and release approval distinct.
[S13]

App Control also has an enterprise compatibility caveat: if its base-policy scope
uses any Package Family Name rule, packaged apps in that scope must use PFN rules
rather than mixing PFN and signer rules. This informs compatibility testing; the
product must not rewrite enterprise policy to make itself run. Store's documented
SmartScreen benefit does not guarantee absence of UAC or other Windows/security
policy refusals. [S01, S12]

## 4. Services, capabilities and the unresolved provisioning bridge

### Store acceptance and platform support are separate

Windows supports packaged services, and installation requires administrative
privileges. The runtime feature guide gives Windows 10 version 2004 as its floor;
the older desktop6 schema's 1903 declaration is not a complete runtime-support
claim. [S15, S16]

| Capability or rule | Current primary-source position | NevroTick disposition |
| --- | --- | --- |
| `packagedServices` | Partner/enterprise-oriented; Store submission discouraged and most cases not approved. [S14] | Serious external gate, not a universal ban or presumed approval. |
| `localSystemServices` | LocalSystem requires this and `packagedServices`. [S14] | No justification to broaden broker authority. |
| `allowElevation` | Strict case-by-case Store approval; Microsoft requests advance justification. [S14] | Explain narrow lifecycle operations if contact is authorized. |
| Store §10.2.4 | Generally restricts non-Microsoft NT-service/driver dependencies and requires disclosure; its stated WHCP exception concerns drivers. [S07] | Obtain exact-product interpretation for this complete service utility. |
| Store §§10.2.7–10.2.8 | Clean removal, user consent and supported Windows-setting changes. [S07] | W32Time and later DNS ownership/restoration belong in certification notes. |

The older desktop preparation page says Store rejects applications needing any
elevation. The specific capability catalog documents conditional approval. Keep
this discrepancy visible; use the specific current policy for the conditional
conclusion, and obtain actual eligibility evidence before depending on it. [S14,
S29]

### Account schema and the strongest credible alternative

Both `desktop6:Service` and `desktop7:Service` expose only LocalSystem,
LocalService and NetworkService for `StartAccount`. Desktop7 adds `delayedStart`,
not virtual accounts. Neither inspected schema declares `RequiredPrivileges` or
`ServiceSidType`. The selected virtual-account broker therefore has no direct
manifest mapping; this does not prove all Windows provisioning methods
impossible. [S16, S17]

A candidate worth examining is **two separate LocalService processes**, with
distinct service SIDs, restricted privileges and explicit object security.
Windows client policy can grant LocalService `SeSystemtimePrivilege`; SCM's
required-privilege list removes unneeded rights at process startup. It does not
grant arbitrary missing rights, and policy/GPO can alter defaults. Shared-process
services receive the union of required privileges. [S18, S19]

Service SIDs provide access-control building blocks, not a separate logon
identity. The same-account candidate must meet
[architecture §4.1](../windows-time-architecture.md#41-selected-account-topology):
engine privilege removal, owner/`OWNER RIGHTS` and ACL isolation, process/token
protection, no injection/duplication/reconfiguration route, broker network denial
and legitimate IPC. [S20]

**Unknown:** a supported complete-package mechanism that establishes these
settings before exposed service execution and preserves them through repair and
update. Do not assume a service can authorize its own `ChangeServiceConfig2`,
require first-launch administrator setup, or insert an external MSI prerequisite.
The alternative needs equivalent isolation and lifecycle evidence before any
topology change is considered.

## 5. Recovery, data and update semantics

### Recovery before removal

The existing contract requires ownership-aware rate cleanup, reconciliation of
mandatory journal state and restoration of the prior W32Time configuration before
the last necessary recovery mechanism is removed. A package transaction, service
stop or clean file removal alone does not establish this application-specific
order. A stop can also mean update, restart or failure; treating every stop as
uninstall would conflate different ownership transitions.

The public `customInstallActions` policy describes restricted game/partner uses;
`UninstallActions` is documented for MSIXVC games. The capability table also names
the exceptional `Microsoft.classicAppInstaller_8wekyb3d8bbwe` route. Preserve that
exception rather than declaring universal OS impossibility, but no generally
available utility grant route was established. [S14, S21]

The current GDK path runs install actions **before first title launch**, not
initial package installation. It depends on retail Store execution; local
`Add-AppxPackage` does not exercise it. Uninstall actions depend on matching prior
install/repair actions. These facts do not qualify pre-first-UI provisioning or
reliable restoration for NevroTick. [S22]

### Distinguishing lifecycle cases

| Case | Documented behavior or existing requirement | Qualification consequence |
| --- | --- | --- |
| First installation | Packaged services require administrative installation. [S15] | Work before first **application UI launch**; this does not promise installation without normal administrative authorization. |
| In-use update | `uap17:UpdateWhileInUse=defer` is Windows 11 24H2+; specified force-update shutdown options are ignored, but required OS updates/reboot may force close. [S23] | Candidate scheduling aid; qualify service behavior, cleanup and bounded adoption of security fixes. |
| Bad Store release | Restoring an older package in Store protects new acquisitions; already upgraded installations need a higher-version corrective release. [S02] | Package-listing rollback does not repair migrated data or deployed clock state. |
| Repair versus Reset | Repair retains app data; Reset permanently removes it and reinstalls. [S25] | Map optional caches and mandatory authority/journal state separately; preserve a supported recovery path. |
| Multiple users | Staging, registration, deregistration and destaging differ; other registrations/provisioning/processes can retain payload. [S24] | Define the machine-service owner, last-user transition and ordinary/admin removal outcomes. |
| Update/interruption | Existing architecture requires quiescence, current generation and durable maintenance/recovery ownership. | Old-package cleanup must not restore W32Time underneath its successor or reset consumed correction authority. |

Continue the [wrong-clock/offline matrix](../windows-time-signing-guide.md#51-wrong-clock-and-offline-activation-matrix)
for initial installation, startup before login, helper activation, update/repair
and removal. Record actual verifier, package/OS identity, trust/time/cache inputs,
retrieval policy, finite work and safe refusal/recovery. Store signing proves
neither universal success nor universal failure with severely wrong dates or no
network. Do not weaken authentication or trust policy to manufacture a pass.

## 6. Delivery, controlled audiences and alternatives

The **Store Web Installer** supports website/GitHub-first acquisition: a Store
badge's Direct mode invokes a Microsoft-provided small EXE that obtains the
published application from Store infrastructure. Free packaged MSIX is supported.
It is an online bootstrapper, not a self-contained offline distribution or free
signing for an MSI listing. [S09]

Private audience must be selected initially; it can become public, while public
cannot become private again. Testers need personal Microsoft accounts; an
unlisted public app is not private access control. Package flights still pass
certification. These are useful controlled-release candidates, not capability
exceptions or guaranteed revocation of copies already acquired. [S10, S11]

There is an official download path: `winget download` can retrieve Store
packages, dependencies and offline licences. Its normal offline-license workflow
requires Entra authentication and specified administrator roles; `--skip-license`
omits the licence and removes that authorization requirement. This does not prove
universal offline activation, dependency completeness, eligibility for a specific
NevroTick product, or permission to rehost arbitrary Store packages. [S26]

SignPath Foundation remains the existing conditional free Authenticode route.
Its published conditions, still labelled Draft, include maintained/documented/
released OSS, restrictions on proprietary components and own-source/upstream
signing, verifiable builds, Foundation publisher identity and manual release
approval. Acceptance is discretionary. Treatment of the chrony-derived port and
brand assets remains unresolved; do not infer rejection or change their licences.
OD-05 already accepts Foundation display and David's approval role. [S27]

Azure Artifact Signing is a separate paid service: its FAQ excludes free, trial
and sponsored Azure subscriptions. Public-trust identity/geographic eligibility
also applies. Free Store onboarding supplies neither a free Artifact Signing
subscription nor eligibility for it. Use the current signing guide for that
fallback; no exact future price is promised. [S01, S28]

## 7. Bounded continuation and existing acceptance owners

The next packaging action is a **source-only W11-W10 decision card**, when that
owner is assigned, using this evidence rather than repeating a general survey.
It does not replace the current Core checkpoint. Prepare three successive exits:

1. **Source and eligibility:** identify publisher/account assumptions and the
   exact full-product capabilities; prepare questions about §10.2.4, service
   provisioning, elevation and supported removal. Record actual answers when
   external contact is authorized. Account creation, Microsoft contact and
   submission are not authorized by documentation integration.
2. **Concrete design:** name the supported provisioning mechanism, effective
   identities/rights, package-aware helper verifier, data/reset ownership and
   install/update/repair/removal state machine. Resolve the missing recovery
   bridge or record a bounded defer. Proposed alternatives remain unselected;
   native work retains the existing execution holds and environment gates.
3. **Actual complete artifact:** after relevant gates and authorization, qualify
   the intended Store-delivered candidate and supported builds. Observe normal
   administrative consent, operation before UI/login, effective tokens, valid
   dependencies, interrupted maintenance, multiple users, wrong-clock/offline
   paths and final restoration. Local schema acceptance is not retail-channel
   evidence; certification is not application qualification.

| Existing owners | Additional distinguishing evidence |
| --- | --- |
| CHECK-1/2/15; platform/security | Intended service/helper succeeds; second same-account service, wrong Store package, unauthorized copied-out helper and unapproved older release cannot obtain broker/data authority. |
| CHECK-5/12/13/23; lifecycle/packaging | Normal and interrupted update/removal preserve current ownership, journal and W32Time restoration; Reset and last-user removal cannot erase the only recovery path. |
| CHECK-18/23; signing/release, CHECK-9 for affected certificate-retrieval policy | Actual package/catalog identity and supported trust/time conditions; tamper/wrong-project rejection alongside valid installed startup and maintenance. |
| CHECK-16/20/26; build/security | Actual Store-produced binary/dependency closure supports required mitigations, provenance and licence obligations without broadening authority. |
| CHECK-24; independent review | Exact candidate, source/approval gaps and native observations remain separate; no planned case or documentation pass becomes a release claim. |

Record disposition and evidence in the existing Windows task. Select a complete
MSIX only after every architecture §19.4 gate passes; otherwise retain the
complete MSI and its qualified release profile. Revisit dated policy/fees before
onboarding, APIs against the selected SDK/build, and trust/lifecycle evidence when
the delivered package, publisher identity or state schema changes.

## 8. Primary source register

Sources are dated retrieval evidence, not frozen platform guarantees. The
documentation does not establish actual project eligibility or native behavior.

- **S01:** [Code-signing options](https://learn.microsoft.com/windows/apps/package-and-deploy/code-signing-options).
- **S02:** [Store MSIX package requirements, identity and version rollback](https://learn.microsoft.com/windows/apps/publish/publish-your-app/msix/app-package-requirements).
- **S03:** [Store MSI/EXE package requirements](https://learn.microsoft.com/windows/apps/publish/publish-your-app/msi/app-package-requirements).
- **S04:** [Free Individual onboarding](https://learn.microsoft.com/windows/apps/publish/whats-new-individual-developer).
- **S05:** [Free Company onboarding](https://learn.microsoft.com/windows/apps/publish/whats-new-company-developer).
- **S06:** [Opening a developer account](https://learn.microsoft.com/windows/apps/publish/partner-center/open-a-developer-account).
- **S07:** [Microsoft Store policies](https://learn.microsoft.com/windows/apps/publish/store-policies).
- **S08:** [Distribution paths](https://learn.microsoft.com/windows/apps/package-and-deploy/choose-distribution-path).
- **S09:** [Store Web Installer](https://learn.microsoft.com/windows/apps/distribute-through-store/how-to-use-store-web-installer-for-distribution).
- **S10:** [Visibility and private audience](https://learn.microsoft.com/windows/apps/publish/publish-your-app/msix/visibility-options).
- **S11:** [Package flights](https://learn.microsoft.com/windows/apps/publish/package-flights).
- **S12:** [MSIX catalog trust and App Control rules](https://learn.microsoft.com/windows/security/application-security/application-control/app-control-for-business/design/manage-packaged-apps-with-appcontrol).
- **S13:** [Process binary-signature mitigation policy](https://learn.microsoft.com/windows/win32/api/winnt/ns-winnt-process_mitigation_binary_signature_policy).
- **S14:** [Capabilities and restricted approval](https://learn.microsoft.com/windows/apps/package-and-deploy/app-capability-declarations).
- **S15:** [Packages containing services](https://learn.microsoft.com/windows/msix/packaging-tool/convert-an-installer-with-services).
- **S16:** [desktop6:Service schema](https://learn.microsoft.com/uwp/schemas/appxpackage/uapmanifestschema/element-desktop6-service).
- **S17:** [desktop7:Service schema](https://learn.microsoft.com/uwp/schemas/appxpackage/uapmanifestschema/element-desktop7-service).
- **S18:** [Change the system time right](https://learn.microsoft.com/windows/security/threat-protection/security-policy-settings/change-the-system-time).
- **S19:** [Required service privileges](https://learn.microsoft.com/windows/win32/api/winsvc/ns-winsvc-service_required_privileges_infow).
- **S20:** [Service SID configuration](https://learn.microsoft.com/windows/win32/api/winsvc/ns-winsvc-service_sid_info).
- **S21:** [UninstallActions](https://learn.microsoft.com/uwp/schemas/appxpackage/uapmanifestschema/element-desktop6-uninstallactions).
- **S22:** [GDK custom install actions](https://learn.microsoft.com/gaming/gdk/docs/features/common/packaging/packaging-custom-install-actions?view=gdk-2604).
- **S23:** [UpdateWhileInUse](https://learn.microsoft.com/uwp/schemas/appxpackage/uapmanifestschema/element-uap17-updatewhileinuse).
- **S24:** [Package registration and lifetime](https://learn.microsoft.com/windows/msix/package/package-lifetime).
- **S25:** [Reset and Repair](https://learn.microsoft.com/windows/msix/desktop/managing-your-msix-reset-and-repair).
- **S26:** [WinGet download](https://learn.microsoft.com/windows/package-manager/winget/download).
- **S27:** [SignPath Foundation conditions](https://signpath.org/terms.html).
- **S28:** [Artifact Signing FAQ](https://learn.microsoft.com/azure/artifact-signing/faq).
- **S29:** [Older desktop preparation guidance](https://learn.microsoft.com/windows/msix/desktop/desktop-to-uwp-prepare).
- **S30:** [Packaging overview](https://learn.microsoft.com/windows/apps/package-and-deploy/packaging/).
