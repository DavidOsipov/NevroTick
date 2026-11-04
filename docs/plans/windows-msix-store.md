# Complete MSIX and Microsoft Store: hardening and delivery plan

**Status:** owner-selected preparation target; implementation, native qualification and Store acceptance pending.<br>
**Recorded:** 3 November 2026 (UTC+04:00); sources checked 2 November 2026 UTC.  
**Repository basis:** `070927855ed99e1a0683d9f291d3462d33343118`.  
**Hardening amendment:** 3 November 2026, against `661281ad0e6504338d352b943154522b800974fa`; new Microsoft Learn sources checked on that date.<br>
**Source-profile continuation:** 3 November 2026, Asia/Tbilisi, against `01bcb679cdf09720a1ae54dfdcae25b0864f4d77`; sections 3.1.1 and 7.1 onward record the concrete profile and remaining supported-mechanism gaps.<br>
**Lifecycle research continuation:** 3 November 2026, against `474698543a07a69830d606388c5c2539e9a4e2ec`; section 7.4 records supported building blocks, unselected alternatives and distinguishing observations. David requested this research be retained in English; no alternative is adopted by that request.<br>
**Operational continuation:** 4 November 2026, against `dcda54448241d54c0554d141f636fab55364d1f4`; the [release policy](../release-policy.md) adds support proposals, Store containment and incident/recovery procedures, with actual rehearsals and product evidence still pending.<br>
**Method:** Microsoft Learn MCP searches and full-page retrieval, supplemented by
the official SignPath conditions. Search snippets were not the final evidence.

## 1. Decision, authority and evidence limits

**An accepted complete Store MSIX can remove the need to purchase and manage a
public package-signing certificate.** Store hosting, updates and package identity
also support the selected distribution direction. These benefits do not establish
that NevroTick's service isolation and recovery lifecycle fit that channel. [S01,
S02, S08]

**Owner decision, 3 November:** prepare one complete MSIX with the strongest
compatible, supported and verified protections; David will try publishing it in
Microsoft Store with its services and reconsider delivery if rejected. This
supersedes the earlier MSI-first/MSIX-conditional selection in this study. Store
preapproval is not a prerequisite to source/design work or preparing a submission
candidate. The [amended OD-05](../nevrotick-owner-decisions.md#7-od-05--signing-and-release-ownership)
owns that direction; [architecture §19.4](../windows-time-architecture.md#194-future-complete-msix-gate)
owns the technical qualification required before release.

The complete MSIX must contain the UI, engine, broker, helper and required payloads.
No MSI-service/MSIX-UI split, separate service prerequisite, broader LocalSystem
broker or hidden orphan component follows from free signing.
Keep the prior complete-MSI design and its SignPath/OpenPGP profiles as retained
alternatives, not an automatic fallback, parallel v1 deliverable or predecessor
to MSIX preparation. Bring an actual Store refusal or concrete unsupported
technical requirement back to David with its evidence and alternatives.

This report records documented facts, project-specific inferences and unresolved
questions separately. It establishes no Store permission, applicant eligibility,
package build, native execution, offline activation or release approval. Current
portable Core priority, Windows execution stop, separate fuzz no-retry hold,
storage-experiment pause and UI-M0/D1–D3 boundaries remain unchanged. The existing
[Windows task](../../tasks/windows11-integration.md#w11-w10-re-enter-conditional-mechanisms-only-for-a-concrete-need)
owns continuation under W11-W10, now selected for MSIX preparation, with W11-W05 for lifecycle and
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
The selected MSIX trust profile must qualify package/catalog verification rather
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

The intended production profile includes **CIG with the explicit Store-signed
image allowance**. Qualify the actual Rust/native DLL closure and activation
mechanism for each role; the selection is not evidence that the binaries already
support it. Test intended imports and allowed workflows beside tampered,
copied-out and wrong-project artifacts. Keep CIG, package identity, helper
authorization and release approval distinct. [S13, S31]

App Control also has an enterprise compatibility caveat: if its base-policy scope
uses any Package Family Name rule, packaged apps in that scope must use PFN rules
rather than mixing PFN and signer rules. This informs compatibility testing; the
product must not rewrite enterprise policy to make itself run. Store's documented
SmartScreen benefit does not guarantee absence of UAC or other Windows/security
policy refusals. [S01, S12]

### 3.1 Package integrity and sealed executable content

Require explicit content-integrity enforcement in the **final packaged manifest**:

```xml
<uap10:PackageIntegrity>
  <uap10:Content Enforcement="on" />
</uap10:PackageIntegrity>
```

This fragment belongs under `Package/Properties`, with
`xmlns:uap10="http://schemas.microsoft.com/appx/manifest/uap/windows10/10"` and the
prefix added to the existing `IgnorableNamespaces` list. It is a requirement for
future authoring, **not a deployable product manifest**. The documented minimum
is build 19041; NevroTick still needs its selected supported Windows 11 matrix.
Use explicit `on`: the documented `default` behavior is `off`. Store integrity
and repair behavior does not remove the obligation to inspect the delivered
manifest and observe tamper handling. [S34–S36]

The package/build owner must also establish:

- All executable product code is in protected installed package content or an
  admitted installed dependency. Forbid `windows.mutablePackageDirectories`,
  `modifiableApp` and enabled `uap10:AllowExternalContent` for this product. No
  plugin directory, downloaded EXE/DLL, executable cache, self-replacement or
  runtime code update outside the declared package lifecycle. These are project
  restrictions; the excluded Windows features are not inherently vulnerabilities.
- Separate immutable binaries/resources from writable state. Optional NTS/DNS
  caches remain disposable; protected policy and clock-recovery journals retain
  their distinct owners and ACL/lifetime requirements. Writable data is always
  untrusted input, even when produced by an earlier signed version.
- Resolve the approved installed identity and actual deployment/path type.
  `PackagePathType_Effective` may select mutable/external content, so it is not
  proof of protected installation. Do not hard-code a `WindowsApps` drive/path
  prefix as the trust boundary; supported package volumes can differ. [S37–S39]
- Minimize and inventory capabilities, extensions, package dependencies,
  COM/protocol/file activations and executable entry points. Declare only the
  justified service and narrowly needed elevation capabilities. Record each
  executable's `RuntimeBehavior`, `TrustLevel`, account, token and allowed work.
  Full-trust MSIX is not an AppContainer sandbox or a manifest-capability network
  firewall. AppContainer/LPAC remain separately qualified role candidates. [S14,
  S30, S40]
- Preserve safe dependency loading: no writable-directory search additions or
  user-selected native libraries. Qualify the packaged dependency graph and
  approved system DLL paths, including transitive and lazy loads. Packaged DLL
  search order differs from unpackaged search; `LoadPackagedLibrary` is available
  for appropriate packaged modules, not a universal system-DLL replacement.
  Full-path loading of the first DLL alone does not prove its dependencies safe.
  [S41, S42]

The frozen release inventory binds source/build inputs, actual PE files, native
dependencies, resources, manifest, package identity, SBOM and corresponding-source
obligations. Preserve both submitted and Store-delivered identities because Store
processing/signing may change bytes. A Cargo crate list is not an inventory of
runtime DLLs; inspect the emitted and actually loaded images.

The [selected leap dataset and policy](leap-second-policy.md#3-knowledge-and-distribution-contract)
are **reviewed immutable generated data compiled into the admitted consumers**,
delivered through this same authenticated complete release. Bind pinned source
identities, generator version, normalized data/policy revision and digest, and
supported coverage to the build and submitted/Store-delivered records. No build
silently fetches mutable upstream data; no new downloader, arbitrary runtime file
path, hot replacement or TUF trust root follows. A corrupt payload is a package
integrity failure; valid unknown future coverage is a policy state, not an
installation failure or a reason to erase usable history. Authorized update and
rollback retain protected floors, consumed correction authority and recovery.

#### 3.1.1 Trusted Launch for the supported package profile

Add **Trusted Launch** to the strongest-profile qualification target on supported
Windows builds. Microsoft documents `trustedLaunch:TrustedLaunch` under
`Package/Properties`, with the value `true`, together with the explicit
`uap10:PackageIntegrity` / `uap10:Content Enforcement="on"` requirement above:

```xml
<trustedLaunch:TrustedLaunch>true</trustedLaunch:TrustedLaunch>
```

Bind that prefix to
`http://schemas.microsoft.com/appx/manifest/trustedlaunch/windows10`. The page
lists minimum build **26100**; its wording says "Windows 10 version 26100", so
use the build requirement and qualify the selected Windows 11/SDK combination.
This does not silently change the product's still-unfrozen supported Windows 11
matrix. A profile claiming Trusted Launch must meet its platform prerequisites
and enforce it. An unsupported/ignored declaration is unavailable, not a passed
protection; a supported older-OS profile needs an explicit compatibility
disposition before release. [S50, S53]

The documented boundary is **which executables may run under this package's
identity**: they must be in its signed catalog. CIG separately restricts images
loaded into a process; helper/IPC authorization separately checks NevroTick's
approved identity, release and operation. None substitutes for the others.
Do not use `CodeIntegrityExternal.cat` to admit executable content outside the
sealed package design; external-location content remains excluded. [S50]

The current Microsoft example has a placeholder value comment and lowercase
`uap10:content`; the dedicated schema names the case-sensitive element
`uap10:Content`. Validate the final combined manifest against the selected SDK
schema rather than copying the defective example. XML well-formedness and an
ignorable namespace do not establish schema support or enforcement. [S34, S50,
S53]

CHECK-16/18/23/26 must observe a catalog-listed intended executable starting
under the correct identity, and a harmless unlisted executable being refused
when attempting to use that identity through the exercised launch mechanism.
Cover UI, actual SCM services and the installed elevated helper separately;
record whether each really has package identity. Recheck the catalog and launch
results on Store-delivered bytes. Local test signing, a generic startup result
or one UI observation does not qualify the other roles. No such native case has
run in this source continuation.

### 3.2 Store-compatible code integrity guard

Microsoft documents CIG for Store-distributed third-party applications with
**“Also allow loading of images signed by Microsoft Store”** enabled. The
Exploit Protection names are `MicrosoftSignedOnly` plus
`AllowStoreSignedBinaries`; strict Microsoft-only and the Store-allowing variant
are different configurations. Neither accepts an arbitrary publisher merely
because its Authenticode chain is trusted, and neither limits accepted Store
images to NevroTick. [S31, S32]

The implementation/qualification record must specify the actual configuration
layer. For process creation, Microsoft documents
`PROCESS_CREATION_MITIGATION_POLICY_BLOCK_NON_MICROSOFT_BINARIES_ALLOW_STORE`
through `PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY`. The runtime
`PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY` has separately documented fields
`MicrosoftSignedOnly`, `StoreSignedOnly` and `MitigationOptIn`; do not invent a
bit-for-bit mapping from PowerShell option names or enable every field blindly.
Use the selected SDK definitions and actual positive/negative observations.
[S13, S33]

SCM, packaged activation and an application-owned child launcher have different
process-creation owners. Document how each role obtains its policy before
exposure. An attribute usable by a child launcher is not automatically an option
on an SCM service manifest. If a role uses early self-opt-in, record the modules
loaded before that call and protect/qualify that path separately; a late setter
does not retroactively prove initial-load protection. No external bootstrapper,
system-wide Exploit Protection rewrite or unsupported SCM hook is selected by
this requirement. [S33, S43]

Qualify the exact installed **Store-signed artifact**, including lazy-loaded CRT,
TLS/crypto/storage providers and supported UI accessibility workflows. A local
test-signed package can qualify declared mechanics but is not proof of Store
signer-category admission. Audit settings can help measure compatibility;
audit-only evidence is not enforcement. When an existing supported mechanism
is used to export Exploit Protection XML, retain its actual exported settings;
do not invent XML element/attribute names. [S31, S32]

Failure to establish a required signature policy blocks the affected role's
network acquisition, sensitive IPC or clock authority. Diagnose and recover via
the separately trusted, bounded lifecycle path; do not retry by disabling CIG or
accepting arbitrary signed DLLs. A stricter Microsoft-only mode may replace the
Store-compatible mode only if the exact role and delivered dependency graph
actually pass; Store signing alone does not establish that result.

### 3.3 Strongest compatible process protections

Use the existing [per-process mitigation manifest](../windows-time-architecture.md#154-per-process-mitigation-profiles-and-startup-admission),
not a second configuration owner. The following are production qualification
targets; none are asserted active by this document. [S31, S43–S48]

| Control | Required target and compatibility boundary |
| --- | --- |
| DEP/NX, ASLR, high entropy, CFG | Preserve the build baseline; inspect final images and instrumented call sites/dependencies. Strict CFG is the target when every executable module supports it; do not manufacture compatibility by dropping instrumentation. CFG has no audit mode. |
| CIG | Store-compatible enforced profile in §3.2 for each role, with independently authorized helper/package identity. |
| ACG / dynamic code | Target `ProhibitDynamicCode` with no thread opt-out or remote downgrade. Inventory JIT/dynamic-code users; do not add a JIT or escape flag for convenience. Services are strongest candidates; UI compatibility needs its own observations. |
| CET / hardware stack protection | Qualify shadow stacks on supported hardware and OS; test strict mode and non-CET/EHCONT image restrictions against the actual toolchain/modules. `/CETCOMPAT` alone is no hardware or runtime proof. Unsupported hardware is a declared platform class, not a fictitious pass. |
| Image loading | Target `NoRemoteImages`, `NoLowMandatoryLabelImages`, `PreferSystem32Images` alongside sealed content and safe search. The last is a search preference; low-IL blocking does not reject every medium-IL writable path. |
| Extension points, handles and heap | Disable legacy extension points where compatible, use strict handle checks and heap corruption termination. Qualify supported UI/IME/accessibility paths; this does not block every injection technique. |
| Win32k | Target prohibition for headless engine/broker when their dependencies permit. Never apply to the native Win32 UI. |
| Child creation | Block children in leaf engine/broker processes. Inventory UI/helper launch needs and qualify their narrow launch path separately; no broad shell/command interpreter escape. |

For every row record role, OS/architecture/hardware support, required or optional
status, exact field/flag, compile/creation/early-runtime phase, effective readback,
positive work, denied action and failure/recovery behavior. CFG enablement and
initial shadow-stack/context-IP validation cannot all be added with a late
`SetProcessMitigationPolicy` call. Creation and compiler/linker support are
separate prerequisites. Treat unsupported, audit-only and enforced outcomes
separately. [S43, S46, S47]

Select the strictest demonstrated compatible setting. A necessary exception
must be narrow, justified by a reproduced legitimate case, linked to the owner
decision and visible in the release evidence; never silently weaken every role.
Required-protection failure denies new protected work while preserving the
separately designed recovery path. These are product-process policies, not
authorization to alter global Defender, WDAC, UAC or enterprise security settings.

### 3.4 Threats and independent controls

This supplements the existing THR/CHECK contracts without new acceptance IDs.
All execution below is future, bounded work on an authorized disposable Windows
fixture, after the current execution stop is lifted.

| Attacker input or failure | Required observation alongside a legitimate control | Existing owner |
| --- | --- | --- |
| Modified package, mutable/external executable path or user-writable DLL candidate | Tampered content/forbidden load is rejected; clean installed package and approved dependency load work. Exercise the actual loader with a harmless canary; source flags alone are insufficient. | CHECK-16/18/23/26 |
| Unrelated valid Store-signed binary, copied-out helper, unapproved older release | No NevroTick helper/IPC or protected-data authority; intended current installed helper performs only the authorized operation. A CIG-allowed image can still be an unauthorized caller. | CHECK-1/2/15/18 |
| Compromised network engine | Cannot set time, impersonate the broker, write broker memory, duplicate protected tokens, change SCM/ACLs or replace mandatory state; legitimate bounded engine-to-broker IPC still works. | CHECK-1/2/15/16 |
| Unsupported/failed mitigation, early loaded incompatible module | Affected role cannot expose sensitive operations under an undeclared weaker profile; supported protected startup and bounded recovery remain available. | CHECK-16/23/27 |
| Signed but malformed/stale IPC, data or update state | Strict bounded admission, current identity/generation and effect authority reject the request; valid neighboring input succeeds. Code signing does not validate data or authorize clock correction. | CHECK-2/3/12/13/15 |
| Forced update, crash, Reset or last-user removal | No stale successor overwrite, erased sole journal/recovery path or orphaned W32Time takeover; ordinary maintenance restores the correct owner/state. | CHECK-5/12/13/23/27 |

CIG reduces an image-loading attack surface; ACG/CFG/CET and privilege boundaries
address other mechanisms. None establishes absence of parser, logic, race,
resource-exhaustion, signed-dependency or clock-control bugs, and Store
certification is not a security proof.

## 4. Services, capabilities and the unresolved provisioning bridge

### Store acceptance and platform support are separate

Windows supports packaged services, and installation requires administrative
privileges. The runtime feature guide gives Windows 10 version 2004 as its floor;
the older desktop6 schema's 1903 declaration is not a complete runtime-support
claim. [S15, S16]

| Capability or rule | Current primary-source position | NevroTick disposition |
| --- | --- | --- |
| `packagedServices` | Partner/enterprise-oriented; Store submission discouraged and most cases not approved. [S14] | Disclose and justify the real services in David's planned submission; record the actual outcome without blocking preparation on speculative preapproval. |
| `localSystemServices` | LocalSystem requires this and `packagedServices`. [S14] | No justification to broaden broker authority. |
| `allowElevation` | Strict case-by-case Store approval; Microsoft requests advance justification. [S14] | Prepare the narrow lifecycle justification and required capability request for David; no hidden elevation or unreviewed entitlement. |
| Store §10.2.4 | Generally restricts non-Microsoft NT-service/driver dependencies and requires disclosure; its stated WHCP exception concerns drivers. [S07] | State the complete utility's service dependency honestly; retain actual certification feedback, not a presumed exception. |
| Store §§10.2.7–10.2.8 | Clean removal, user consent and supported Windows-setting changes. [S07] | W32Time and later DNS ownership/restoration belong in certification notes. |

The older desktop preparation page says Store rejects applications needing any
elevation. The specific capability catalog documents conditional approval. Keep
this discrepancy visible; use the specific current policy for the conditional
conclusion, and obtain actual eligibility evidence before claiming approval. [S14,
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

Freeze a per-object/per-operation matrix for service configuration, process and
thread handles, tokens, IPC, installed files, mandatory state and optional data.
The engine must lack, rather than merely disable, forbidden privileges. Provision
and verify before opening sensitive endpoints or taking clock ownership; repair
and update must re-establish the same boundary. No broad LocalService ACL or
shared-process union of privileges substitutes for isolation. Effective network
restrictions include delegated certificate/DNS retrieval under their real owner.
If this bridge cannot be supported by the complete package, report that precise
blocker and alternatives to David; do not silently select LocalSystem or ship an
external installer workaround.

### Concrete provisioning boundary

The pinned Microsoft reference XSDs confirm that the inspected `CT_Service`
types contain `Name`, `StartupType`, `StartAccount`, `Arguments`, Dependencies
and TriggerEvents. They have no `RequiredPrivileges`, `ServiceSidType`, security
descriptor or `ServiceType` attribute. The exact account spellings are
`localSystem`, `localService`, `networkService`; desktop7 adds the
`delayedStart` startup value. Query the installed service's actual
`dwServiceType` rather than treating a declaration as proof of own-process
hosting. These reference XSDs are pinned source evidence, not selected-SDK
schema validation or an observed deployment. [S16, S17, S63, S64]

The Win32 setters exist: `ChangeServiceConfig2W` accepts
`SERVICE_CONFIG_REQUIRED_PRIVILEGES_INFO` and `SERVICE_CONFIG_SERVICE_SID_INFO`,
and `QueryServiceConfig2W` reads the configured values. The configuration handle
requires `SERVICE_CHANGE_CONFIG`. The documented default service-object access
for LocalService/NetworkService does not grant that right. A service cannot be
assumed to bootstrap its own privilege restrictions, SID policy or sibling
isolation merely because it can report status to SCM. [S54, S55]

The missing link is an **authorized installation/maintenance owner and ordering**
that applies and verifies the settings before automatic or manual service
exposure, then re-establishes them after repair/update. A startup mode of
`manual` or `disabled` alone supplies no such owner. Granting the engine broad
configuration rights, starting it with excess rights until the UI fixes them,
or treating a registered service SID as a separate account would change the
required boundary. Keep the virtual-account broker mismatch and the unselected
two-LocalService alternative explicit.

Configured privilege lists are not evidence of an already running token.
Qualify the selected setting's documented activation point and actual new
process after servicing; readback of the SCM database cannot substitute for
token/owner/ACL/denied-operation observations. Required privileges take effect
on the next service start; the service-SID page documents next system start.
Keep separately observed first-install behavior distinct from that documented
contract. Shared-process privilege union and the retained
`SeChangeNotifyPrivilege` exception still apply. [S19, S20]

A supported custom-install route
would also need applicable entitlement and proof that its action runs before
the relevant service, including no first UI launch. The exceptional capability
name in S14 and first-launch GDK behavior in S22 do not establish that ordering.

## 5. Recovery, data and update semantics

### Recovery before removal

The existing contract requires ownership-aware rate cleanup, reconciliation of
mandatory journal state and restoration of the prior W32Time configuration before
the last necessary recovery mechanism is removed. A package transaction, service
stop or clean file removal alone does not establish this application-specific
order. A stop can also mean update, restart or failure; treating every stop as
uninstall would conflate different ownership transitions.

The public `customInstallActions` grant policy is stronger than unspecified
partner eligibility: it limits the capability to particular Microsoft/partner
desktop games and says other scenarios will not receive it. The separate
technical condition names `Microsoft.classicAppInstaller_8wekyb3d8bbwe` as a
custom capability, not a package dependency or a general utility entitlement.
Treat this route as blocked under the published policy for NevroTick's ordinary
scenario; only explicit applicable Microsoft exception evidence could change
that disposition. This does not prove universal OS impossibility. [S14, S21]

The current GDK path runs install actions **before first title launch**, not
initial package installation. It depends on retail Store execution; local
`Add-AppxPackage` does not exercise it. Uninstall actions depend on matching prior
install/repair actions. These facts do not qualify pre-first-UI provisioning or
reliable restoration for NevroTick. [S22]

### Concrete removal and recovery boundary

The following public mechanisms were checked for the missing lifecycle order:

| Mechanism | Documented behavior | What remains unproved for NevroTick |
| --- | --- | --- |
| `PackageCatalog.PackageUninstalling` / its event arguments | Reports activity, result, completion, package and progress; the arguments expose no documented deferral, cancellation or recovery acknowledgement. [S57] | Observation cannot require deployment to wait for restored clock state or guarantee that a stopped service is activated for cleanup. |
| `PackageManager.RemovePackageAsync(String)` | The request cannot be canceled; associated applications are shut down during removal. [S58] | A helper can restore before initiating its own removal request, but that does not intercept independent Settings/administrative removal or Reset. |
| `RemovalOptions` | The caller chooses removal options; `PreserveApplicationData` is limited to development-mode packages. [S59] | A caller-selected defer option is not a package-authored barrier for every removal path; development-mode preservation is not a Store journal strategy. |
| Service STOP / preshutdown | STOP is a service-lifecycle event; preshutdown allows a bounded wait until timeout or `SERVICE_STOPPED`. [S60] | Neither identifies final package removal or guarantees recovery after an already stopped/crashed service. Ordinary STOP cannot restore W32Time underneath a successor. |
| `GetServiceDirectory` / `GetServiceRegistryStateKey` | SCM deletes service state directories/registry keys when the service is uninstalled. [S61, S62] | Neither location can be the sole journal that must survive incomplete recovery through service removal. |
| Package retention and Reset | Registration/deregistration and staging/destaging differ; Reset removes app data and reinstalls. [S24, S25] | Remaining payload, another user's registration or a surviving data file alone does not establish an invocable recovery owner. |

**Bounded finding:** no reviewed generally applicable contract currently ensures
that independent Settings uninstall, Reset and last-user/all-users removal
complete NevroTick's recovery before the last necessary code/journal disappears,
or retain an actually invocable recovery path until it does. This is unresolved
support/ordering evidence, not a universal claim that MSIX cannot run uninstall
actions. Preserve the restricted/custom-capability exception above.

A protected journal, service cleanup and helper-driven restore-before-remove are
useful parts of the design. They do not close that independent-removal path.
The lifecycle owner must supply the supported mechanism and ordering evidence
before real takeover; a hidden off-package watchdog, mandatory first UI launch
or separate installer is not selected as a workaround. The initial distinguishing
experiment is bounded in section 7.3.

### Distinguishing lifecycle cases

| Case | Documented behavior or existing requirement | Qualification consequence |
| --- | --- | --- |
| First installation | Packaged services require administrative installation. [S15] | Work before first **application UI launch**; this does not promise installation without normal administrative authorization. |
| In-use update | `uap17:UpdateWhileInUse=defer` is Windows 11 24H2+; specified force-update shutdown options are ignored, but required OS updates/reboot may force close. [S23] | Candidate scheduling aid; qualify service behavior, cleanup and bounded adoption of security fixes. |
| Bad Store release | Restoring an older package in Store can serve new acquisitions after its suitability is assessed; already upgraded installations need a higher-version corrective release. [S02] | [Release-policy §§4–6](../release-policy.md#4-distribution-containment-choose-the-actual-store-operation) distinguish containment, deliverable correction and current mandatory-state recovery. An unsafe fallback is not automatically recommended. |
| Repair versus Reset | Repair retains app data; Reset permanently removes it and reinstalls. [S25] | Map optional caches and mandatory authority/journal state separately; preserve a supported recovery path. |
| Multiple users | Staging, registration, deregistration and destaging differ; other registrations/provisioning/processes can retain payload. [S24] | Define the machine-service owner, last-user transition and ordinary/admin removal outcomes. |
| Update/interruption | Existing architecture requires quiescence, current generation and durable maintenance/recovery ownership. | Old-package cleanup must not restore W32Time underneath its successor or reset consumed correction authority. |

Continue the [wrong-clock/offline matrix](../windows-time-signing-guide.md#51-wrong-clock-and-offline-activation-matrix)
for initial installation, startup before login, helper activation, update/repair
and removal. Record actual verifier, package/OS identity, trust/time/cache inputs,
retrieval policy, finite work and safe refusal/recovery. Store signing proves
neither universal success nor universal failure with severely wrong dates or no
network. Do not weaken authentication or trust policy to manufacture a pass.

The [release-policy compatibility matrix](../release-policy.md#6-mandatory-state-compatibility-and-owned-recovery)
requires recovery from the bad version's actual schemas, partial migrations,
consumed/uncertain effects and current ownership. Reusing older code in a new
package does not authorize an old authority snapshot. These procedures retain
the independent-removal gap above; they supply no new provisioning mechanism or
native result.

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

The [operational control table](../release-policy.md#4-distribution-containment-choose-the-actual-store-operation)
adds the distinct gradual-update rollout, halt/fallback, cloned-submission and
mandatory-metadata cases with current Microsoft sources. Initial audience,
flight membership and update percentage are different controls; an unavailable
listing is not recall. Qualify the actual former-owner, flight and corrective-
update reach for the selected visibility setting before promising containment
or recovery. David retains every Store action and expansion decision.

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

The [accepted optional-provenance decision](../nevrotick-owner-decisions.md#75-optional-enhanced-build-provenance--4-november-2026)
keeps attestations, independent reproducible builds and the public
[source-to-Store evidence bundle](release-provenance.md) outside mandatory
release prerequisites. The submitted/delivered identity, integrity, SBOM/licence,
installed trust and native/lifecycle checks below remain required; they do not
implicitly require that additional proof.

The selected work remains **W11-W10 with W11-W05 and the existing release/security
owners**. Source/design preparation is ready and does not require Store
preapproval. It does not replace the current Core checkpoint or lift an execution
hold. Record work and evidence in
[task §4.8](../../tasks/windows11-integration.md#48-owner-selected-hardened-msix--3-november-2026),
using these successive exits rather than another plan or approval campaign:

1. **Freeze the complete design and security profile.** Inventory components,
   required capabilities and entry points, immutable code/data layout, supported
   pre-exposure service provisioning, identity/IPC and mitigation activation per
   role. Design recovery before removal/Reset and maintenance across versions.
   Prepare concise certification notes: what each service does, why it must run
   before UI/login, exact accounts/rights, elevation and user consent, resources,
   removal/restoration and reproducible reviewer steps. Preserve unresolved
   questions with a concrete evidence owner; no general research rerun.
   Prepare the [release record and incident procedures](../release-policy.md)
   with explicit support-proposal status, compatibility matrix, account recovery
   owner and rehearsal plan. This adds no implementation prerequisite to the
   current Core slice.
2. **Build and qualify a complete local candidate when prerequisites permit.**
   Implement actual roles and packaging through the existing phase work. Inspect
   the emitted manifest, capability/dependency closure, service declarations,
   final PE protections, source/SBOM/licence lineage and package integrity.
   After the Windows stop is lifted, qualify test-signed mechanics in an owned
   disposable VM: effective tokens/ACLs/network boundaries, startup before login,
   legitimate and rejected DLL/IPC actions, wrong-clock/offline behavior, repair,
   Reset, interrupted maintenance, multiuser and final restoration. Record which
   CIG/catalog claims await a genuine Store artifact. No fake production manifest
   or unsigned fixture counts as a complete candidate.
3. **David attempts the Store submission with services.** Provide the concrete
   package and certification notes for his account/capability/visibility choices.
   A private initial audience is the controlled route to obtain and test actual
   Store-delivered bytes before broad rollout; decide visibility before the first
   submission because public-to-private conversion is unavailable. Flights follow
   an initial submission and still undergo certification. Do not require a Store
   signature before this first submission: Microsoft signing occurs during the
   publication pipeline after certification. Account work, upload and publication
   remain David's actions, not actions delegated to an agent by this plan.
   [S02, S10, S11, S49]
4. **Qualify the Store-delivered candidate before broad release.** Capture actual
   package identity, final manifest/catalog/images and dependency hashes; rerun
   the affected trust/CIG, activation, runtime and servicing cases on supported
   Windows builds with protections enforced. Reconcile submitted-to-delivered
   provenance and report changed-byte evidence. Certification supplies no
   application security pass. If refused, retain the actual reason and bring
   compatible alternatives to David; do not broaden authority or silently switch
   to MSI. If accepted but a technical gate fails, correct the candidate and
   requalify instead of treating acceptance as a waiver.
   Before broad release, complete the applicable operational rehearsals and
   obtain David's actual stable-support/transition decision. Preserve separate
   observations for contained distribution, available correction and recovered
   installed cases; publication alone is none of those native recovery results.

| Existing owners | Additional distinguishing evidence |
| --- | --- |
| CHECK-1/2/15; platform/security | Intended service/helper succeeds; second same-account service, wrong Store package, unauthorized copied-out helper and unapproved older release cannot obtain broker/data authority. |
| CHECK-5/12/13/23; lifecycle/packaging | Normal and interrupted update/removal preserve current ownership, journal and W32Time restoration; Reset and last-user removal cannot erase the only recovery path. |
| CHECK-18/23; signing/release, CHECK-9 for affected certificate-retrieval policy | Actual package/catalog identity and supported trust/time conditions; tamper/wrong-project rejection alongside valid installed startup and maintenance. |
| CHECK-16/20/26; build/security | Actual Store-produced binary/dependency closure supports explicit integrity enforcement, sealed executable content and the role-specific CIG/ACG/CFG/CET/image policies; source/licence/SBOM lineage and compatible positive controls accompany denied actions. |
| CHECK-24; independent review | Exact candidate, source/approval gaps and native observations remain separate; no planned case or documentation pass becomes a release claim. |
| W11-W10/W11-W05; existing release, state and lifecycle CHECK owners | [Release-policy §9](../release-policy.md#9-qualification-matrix-and-remaining-decisions) supplies support transition, halt/correction, publishing-authority and bad-state recovery cases. The support baseline is proposed; account/Store/native rehearsals remain unexecuted. |

Record disposition and evidence in the existing Windows task. The MSIX direction
is selected now; **release qualification** still requires every applicable
architecture §19.4 gate. Revisit dated policy/fees before
onboarding, APIs against the selected SDK/build, and trust/lifecycle evidence when
the delivered package, publisher identity or state schema changes.

### 7.1 Concrete source inventory and manifest profile

**Inspected baseline:** `01bcb679cdf09720a1ae54dfdcae25b0864f4d77`, complete
untruncated Git tree, root/member Cargo manifests, lockfile, UI entry point and
Windows target configuration. The main workspace has six members; the lockfile
contains 31 package records, including those six local members. These are source
inventory facts, not a Windows runtime dependency closure or release SBOM.

| Product role | Actual source at this checkpoint | Packaging/activation consequence |
| --- | --- | --- |
| Ordinary-user desktop UI | `crates/nevrotick-ui` has a fixture-only `main.rs` and Win32 adapter; direct dependencies are garde 0.23.0 and Windows-only windows-sys 0.61.2 | It is a usable reference for the future UI entry point, not a production package payload. Retain `asInvoker` behavior and UI-M0 deferrals. |
| Network-facing engine service | `crates/nevrotick-engine` contains a library and explicit `nevrotick-observe` CLI; direct external dependencies are getrandom 0.4.3 and Tokio 1.54.1, plus local `ntp-wire` | The observation CLI is not an SCM service. A production service entry point, host adapter, providers and native dependency closure remain absent/unqualified. |
| Clock broker | No `clock-broker` production crate or executable is present in the inspected tree | Its service identity, clock/IPC/recovery adapters and independently qualified payload must be implemented through P3/P4. Do not substitute the observer CLI. |
| Elevated lifecycle helper | No `admin-helper` production crate or executable is present | The narrow installed helper and supported activation/provisioning mechanism remain prerequisites. A declaration alone supplies neither executable nor authority. |
| Linked portable owners | `ntp-wire`, `time-domain`, `ntp-exchange`, `time-core` | Library code is linked into the relevant process; these are not four additional installed services or runtime DLLs. Their fixture qualifications do not establish production sample/clock eligibility. |

**4 November addition:** NV-LP-001 assigns the shared normalized leap calendar and
effective-dated policy to generated immutable data, with pure implementation
pending in S1.4. Inventory the eventual compiled data and its provenance alongside
the affected linked owners; it creates no extra executable, service, capability,
mutable package-data store or independently updated product. This addition does
not rewrite the inspected-source facts in the table above.

Use the actual Cargo feature graph before naming runtime providers: UI/domain/core
use garde with `derive`; the engine's current-thread Tokio feature set is `rt`,
`net`, `time`, `sync`, `macros`. No production TLS/NTS provider, storage provider,
Windows service wrapper, CRT layout or PSF fixup closure is admitted by this
inventory. The existing `.cargo/config.toml` supplies generic `x86-64`, CFG,
ASLR and DEP flags; OD-07's intended v3 migration still has its own build and
below-baseline startup-admission work. `ProcessorArchitecture="x64"` alone is
not a v3 CPU/OS-state prerequisite check.

The package author uses this field-level profile when real components exist:

| Manifest/input surface | Concrete intended value or rule | Unfilled input / independent check |
| --- | --- | --- |
| `Identity` | One product identity, `ProcessorArchitecture="x64"`, monotonic release version | Exact `Name`/`Publisher` come from David's Partner Center identity; do not invent them. Bind submitted and delivered versions separately. [S02, S51] |
| `Dependencies/TargetDeviceFamily` | `Name="Windows.Desktop"`; freeze actual `MinVersion` and `MaxVersionTested` | Existing Windows 11 target, required features and the build-26100 Trusted Launch row must agree. A test maximum is not proof of coverage. [S51] |
| Ordinary UI `Application` | Explicit actual `Executable`, stable unique `Id`, `uap10:RuntimeBehavior="packagedClassicApp"`, `uap10:TrustLevel="mediumIL"` | This expresses the selected ordinary full-trust Win32 UI; it supplies no AppContainer or administrative authority. Avoid contradictory legacy `EntryPoint` attributes. [S51, S52] |
| Package `Properties` | Explicit content integrity; Trusted Launch for the admitted supported profile; real display/asset metadata | Resolve namespaces and SDK schema, then inspect the emitted and Store-delivered manifest. Protect all executable content as in section 3.1. |
| Compiled calendar/policy provenance | Same authenticated complete release and supported coverage as [NV-LP-001](leap-second-policy.md#3-knowledge-and-distribution-contract) | Reproduce reviewed pinned-input generation and bind the exact normalized revision/digest and generator to the actual payload. No new manifest capability, runtime download or separate updater is required. |
| `runFullTrust` | Required for the intended medium-IL desktop application | Declare the restricted capability and verify the actual activation graph. It is not elevation. [S14, S51] |
| `packagedServices` | Required for declared `windows.service` extensions | Disclose the real engine and broker; record their own-process SCM service entries, executable paths, accounts and startup configuration. Account/provisioning mapping remains unresolved under section 4. [S14–S17] |
| `allowElevation` | Justified only for the intended bounded elevated lifecycle operations | Exact helper activation and Store capability admission remain unresolved; normal UI use stays unelevated. [S14] |
| `localSystemServices`, mutable/external executable content | Excluded from this selected profile | Do not insert broader authority or mutable code to get a package accepted or a mitigation test to pass. |
| `customInstallActions` and `Microsoft.classicAppInstaller_8wekyb3d8bbwe` | Blocked for this ordinary utility scenario under the published grant policy; retain only as an explicitly confirmed exception | The technical capability condition does not override the games-only grant policy. Any actual exception still needs activation/ordering and lifecycle evidence. [S14, S21, S22] |
| Other capabilities/extensions/dependencies | Only individually justified by an actual component | No speculative broad filesystem, COM, protocol, startup-task, shell or networking declaration. Full-trust network confinement still needs the actual service/WFP owner. |

The immutable layout contains the approved UI/service/helper images, admitted
native dependencies, resources and required source/licence metadata. Diagnostics,
optional caches and mandatory policy/recovery state keep their separate existing
owners and lifetimes. Neither per-user package data nor a guessed `WindowsApps`
path is a proven machine-wide recovery store. Final filenames, paths and ACLs
are bound to real build/installed artifacts; this source card is deliberately not
an installable manifest with invented services, publisher or assets.

### 7.2 Concrete per-role activation record

Use architecture §15.4's existing manifest fields; the following rows populate
the source/activation layer without inventing observed enforcement:

| Role / actual creation owner | Earliest supported application-owned work | Remaining creation/compatibility evidence |
| --- | --- | --- |
| Future own-process engine and broker / SCM | Main connects promptly with `StartServiceCtrlDispatcherW`; bounded service-specific initialization in `ServiceMain` establishes supported policies/readback and the existing token/configuration admission before providers, normal network/IPC or clock work. Preserve control/status registration and bounded cleanup. [S43, S56] | SCM owns creation. No reviewed service manifest field supplies a general process-mitigation vector. Loader/static-import/CRT and thread-local-storage callback execution precedes service code; startup inventory and final package trust must cover it. Child restrictions, initial CFG/HSP and any creation-only policy still need their real supported mechanism. |
| Ordinary desktop UI / Windows packaged activation | Its trusted entry point can set supported runtime policies before its application work; retain Win32k and separately qualify native controls/accessibility. [S43, S52] | The application does not thereby control Windows' parent-side creation attributes. Record the initial image/loader state, CIG/ACG compatibility and actual package identity. |
| Installed elevated helper / approved activation and UAC path | The helper's own trusted entry point can perform bounded admission before its permitted lifecycle operation. IPC payloads never choose a weaker role. [S43] | The exact supported activation/account/package-identity path remains unresolved. A UAC-launched helper is not automatically created with attributes set on some different process. Keep its narrow rights and recovery behavior separately qualified. |
| An actually application-created child, if a necessary operation later requires one | `PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY` can carry the documented creation values, including `PROCESS_CREATION_MITIGATION_POLICY_BLOCK_NON_MICROSOFT_BINARIES_ALLOW_STORE`. [S33] | This is a mechanism reference, not selection of a launcher or additional child process. It does not prove SCM/UI/UAC activation or bypass the leaf-service no-child contract. |

For the runtime signature-policy candidate, Microsoft documents
`SetProcessMitigationPolicy(ProcessSignaturePolicy, ...)` and the
`PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY` structure. Its `MitigationOptIn`
description admits the **Microsoft, Store and WHQL signer classes**. Record that
documented breadth explicitly; do not silently relabel it the exact equivalent
of a PowerShell option or infer an OR operation by setting both
`MicrosoftSignedOnly` and `StoreSignedOnly`. The selected profile's actual
configuration/allowed-image set must be reconciled and observed before adoption.
This source card adopts no untested native bit combination. [S13, S31–S33, S43]

The early-runtime layer can evaluate ACG, signature/image/extension policies,
strict handles and compatible headless restrictions through their documented
setters. It cannot retroactively protect the loader or enable every initial
CFG/CET state; section 3.3 and architecture §15.4 retain those distinctions.
Long scans before the service dispatcher are not a workaround: Microsoft directs
own-process services to connect immediately and do their initialization in
`ServiceMain`. Query support, successful setting, effective state, denied action
and legitimate operation remain five separately recorded observations. [S43,
S46, S47, S56]

### 7.3 Bounded blockers and the first later package experiment

The source card's exit is **prepared with named unresolved mechanisms**, rather
than a deployable or release-qualified package:

| Existing owner / checks | Exact remaining condition | Next useful evidence |
| --- | --- | --- |
| W11-W05 platform/security; CHECK-1/2/15/16 | Manifest-supported account topology plus a trusted, supported pre-exposure provisioning owner that survives repair/update | Resolve the section 4 account/ordering bridge against the selected SDK and applicable custom-capability contract. Do not promote two LocalService services to a chosen topology without the same-account isolation proof. |
| W11-W05 lifecycle/storage; CHECK-5/12/13/23/27 | Recovery owner remains invocable until durable restoration, including independent removal/Reset when the service is already stopped | Identify the removal bridge, then challenge running versus stopped-service removal before expanding the package matrix. A record without executing code cannot restore the system. |
| W11-W10 platform/build; CHECK-16/18/20/26/27 | Concrete launch path and module graph satisfy required CIG and other policies; Trusted Launch supported on the selected OS row | Inspect real emitted/loaded images and runtime admission once actual roles exist. Preserve the pre-entry loader gap and the documented signer-class distinction. |
| Release/packaging; CHECK-18/23/24 | Real identity, production payloads, qualified local mechanics and candid capability/reviewer notes | Build the complete submission candidate through P3/P4/P6 when those prerequisites permit. David's Store attempt precedes genuine Store-byte trust tests; prior certification remains no preparation prerequisite. |

**First native experiment, still held:** use one inert locally test-signed
package with an automatic test service, no first UI launch and no clock/W32Time
mutation. Record controls, protected lifecycle generation, cleanup start and a
durable simulated recovery-complete marker. An independent observer records
service/package events but never performs the claimed cleanup.

Start with two cases: ordinary Settings removal while the service runs, and
the same removal after it has already stopped/crashed. Use an ordinary service
STOP as the control trace. The independent expectation is that the proposed
supported recovery owner actually runs, and incomplete recovery cannot destroy
the last usable recovery path. Merely seeing STOP or a retained file is not a
pass. Bind code, package, Windows build, tokens, elapsed work, interruption and
cleanup to the receipt; one build's observation does not create a platform
guarantee. Only a credible mechanism surviving this pair justifies expanding
to Reset, update, reboot/interruption and multiuser cases. GDK-dependent action
execution needs its documented Store pipeline and cannot be qualified by local
registration. Existing Windows STOP and package/environment prerequisites remain.

The provisioning owner can separately test two inert initially disabled services
with an explicitly administrative laboratory harness: apply the exact SCM
settings, observe new effective tokens/objects and valid/denied operations,
then update to a second version and re-observe before exposure. Its scope is
the Windows building blocks and servicing behavior. Success with that external
harness does not supply a production installation owner; the complete-package
bootstrap must later demonstrate its own supported pre-exposure sequence.

Certification preparation can proceed from section 7.1's role/capability table.
The release owner fills it with actual executable/service names, effective
accounts and rights, user-consent/elevation flow, expected network/resource use,
startup before login, restoration/removal results and exact reviewer steps for
the frozen candidate. Missing rows stay unresolved; do not submit planned
protections or hypothetical removal results as implemented facts.

### 7.4 Service provisioning and independent-removal research

This continuation refines sections 4, 5 and 7.3 using Microsoft Learn full-page
reads. It separates documented APIs, architecture proposals and native evidence.
The selected complete-MSIX direction, current account/authority contracts and
recovery requirement remain authoritative. None of the candidates below is a
chosen topology, approved capability, implemented mechanism or Windows pass.
The source inventory remains that of section 7.1.

#### Provisioning candidates and decision boundaries

The configuration APIs exist; the unresolved question is which trusted package
role applies them in a safe order and retains the guarantee through servicing.
Do not give the network-facing engine `SERVICE_CHANGE_CONFIG`: that right also
allows changing the service executable. Configuring service DACL/owner requires
the corresponding `WRITE_DAC`/`WRITE_OWNER` authority. Readback through
`QueryServiceConfig2W` is necessary but does not prove the effective token or
object isolation of an already running process. [S54, S55, S69–S71]

| Unselected candidate | Useful property | Remaining cost or evidence |
| --- | --- | --- |
| Initially disabled services plus an explicitly elevated enable/configure operation | Supplies a narrow configuration owner without a permanent LocalSystem runtime role | Changes activation UX if it requires an initial user action; helper elevation/Store eligibility, exact package identity and servicing behavior remain unqualified. Do not silently add mandatory first UI launch. |
| Short-lived automatic LocalSystem bootstrap service, with functional services initially disabled | A manifest-supported account/start mode could provide configuration without depending on UI launch | Adds a privileged service registration and `localSystemServices`; process exit does not remove that registration. First-start ordering, protected re-entry, interruption, idempotence and repair/update/SID activation remain unqualified. This is not a broader LocalSystem clock broker or an adopted exception. |
| One SCM broker creating a restricted engine child | Can establish the engine's token restrictions and creation-time mitigations before engine entry | Changes topology and supervision. It does not itself supply all broker/service configuration authority, outbound-network confinement or independent-uninstall recovery. |

The first two candidates are inferred compositions of supported manifest/SCM
building blocks, not Microsoft-endorsed complete MSIX recipes. The manifest
permits disabled and automatic services and LocalSystem/LocalService accounts;
Store approval is a separate condition. Changing a packaged service to a virtual
account through the general SCM setter would likewise require a supported
servicing contract; API support for virtual accounts does not establish that
MSIX preserves a manifest/account mismatch. [S14, S16, S17, S55, S69]

For an explicit enable operation, keep service names, executable ownership,
permitted operations and expected package identities product-controlled. The
helper must not accept arbitrary executable paths, service names, command lines,
accounts or security descriptors from UI/IPC. Configure while functional services
remain disabled, verify each transition, then obtain admission evidence from
the actual fresh processes before allowing networking/IPC/clock ownership.
Interrupted provisioning must remain nonfunctional and recoverable. Reconcile
the selected lifecycle with the existing owner-decision procedure before its
dependent implementation.

#### SCM facts that change the implementation design

| Documented fact | Consequence for NevroTick |
| --- | --- |
| A manual service can have a start trigger; Microsoft's desktop10 example uses `StartupType="manual"` with a domain-join trigger. [S67] | Manual mode is not a pre-provisioning barrier. Use disabled state for the candidate and verify that premature starts fail, then separately qualify every permitted activation path. |
| Since Vista, `LockServiceDatabase` is compatibility-only and has no database-locking effect. [S65] | Do not implement a fictitious atomic multi-service transaction by taking that lock. Every interrupted intermediate state needs a safe disposition. |
| Win32 `StartServiceW` returns after SCM learns that the ServiceMain thread was created, without waiting for the first status update. [S66] | Successful launch is not token, mitigation, IPC or application-readiness evidence. Require a bounded admission result bound to the actual process and generation. |
| Required privileges apply at the next service start; shared-process services receive a union, and `SeChangeNotifyPrivilege` is retained. [S19] | Inspect own-process hosting and effective privileges. RequiredPrivileges removes authority; it does not manufacture missing privileges. |
| The service-SID reference documents next-system-start activation. [S20] | Distinguish configuration readback, pending reboot and separately observed first-install behavior. Do not assume a restart alone establishes every requested SID property. |
| UserRights CSP's ChangeSystemTime policy requires LocalService to remain assigned. [S68] | The account name cannot prove that a network engine lacks clock authority. Check actual machine policy and token, including absence rather than temporary disabling of forbidden privileges. |

The CSP statement is scoped to its documented policy/platform support; it is not
a claim that every token on every Windows configuration is identical. The
generic account description and a successful configuration call are insufficient
substitutes for native token and denied-operation evidence.

#### Restricted child: concrete benefit and restart tradeoff

`CreateRestrictedToken` can remove privileges, mark SIDs deny-only and add
restricting SIDs. The documented access check evaluates both ordinary and
restricting SID sets. Microsoft explicitly supports passing a restricted copy
of the caller's own primary token to `CreateProcessAsUserW` without
`SeAssignPrimaryTokenPrivilege`. The latter API still typically requires
`SeIncreaseQuotaPrivilege`; it can temporarily enable a required privilege
already present in the caller's token. [S72, S73]

This makes a broker-created engine a credible way to keep clock/debug/
impersonation authority out of the engine from process creation. It also gives
the broker ownership of supported creation attributes. It does not retroactively
restrict the SCM-launched broker's loader/CRT path, establish package identity
for the child, prove Trusted Launch/CIG behavior, or provide broker network
denial. An AppContainer/LPAC alternative would require its own supported token,
capability, networking, IPC and identity qualification; it is not silently
combined with this restricted-token proposal.

Permanent removal differs from disabling: a removed privilege cannot be restored
to the same token. If the broker removes authority needed by the selected child
creation path, its later engine-restart strategy must still work. Retaining a
necessary minimal privilege or obtaining a fresh SCM process are alternatives
to qualify. Keeping an unrestricted duplicate token merely to regain supposedly
removed authority would preserve that authority. [S74]

Account sharing remains a substantive boundary. Default process security derives
from token security, and dangerous process/handle access can undermine isolation.
Explicit process/thread/token/IPC/journal descriptors and controlled handle
inheritance must deny engine-to-broker escalation while permitting legitimate
IPC. An `OWNER RIGHTS` ACE can replace the owner's implicit READ_CONTROL and
WRITE_DAC permissions; its existence does not prove a proposed descriptor is
correct. Retain the architecture's per-object matrix and actual positive/negative
Windows controls. The broker also becomes responsible for child lifetime,
bounded restart and preventing an orphan engine after broker failure. [S73,
S75, S76]

#### Custom-action policy and lifecycle refinement

The current S14 grant policy blocks ordinary NevroTick use of custom actions;
the separate custom-capability formula does not override it. If Microsoft later
confirms an applicable exception, obtain the exact supported channel, process
identity, service-start ordering, failure/cancellation behavior and cleanup/data
lifetime contract before relying on it.

The GDK/MSIXVC documentation says installation actions run before first title
launch, continue toward launch after failed actions, and retry failed/incomplete
install actions on later launches. Uninstall actions depend on matching prior
install/repair action names; a successful action name may be skipped across an
update, so changing arguments alone is not a migration identity. Local
Add-AppxPackage/wdApp development installation does not execute this documented
Store pipeline. These are GDK-path facts, not a proven description of every
possible exceptional classicAppInstaller path. The launch-failure discussion does
not prove that every uninstall failure is ignored. No service ordering or
uninstall-veto guarantee is inferred. [S14, S21, S22]

#### Recovery lifetime, stop acknowledgement and delayed restart

Detailed MSIX virtualization/uninstallation documentation concerns packaged and
redirected state. It does not promise transactional rollback of arbitrary SCM
changes to the pre-existing W32Time service. This is why clean-uninstall claims
do not close the application-specific restoration requirement. Service-managed
state remains unsuitable as the sole surviving journal because SCM deletes it
when the service is removed. [S37, S61, S62]

`SERVICE_STOPPED` is the final acknowledgement: Microsoft requires necessary
cleanup before that report, one stopped report only, and no further work because
the process may be terminated at any time afterward. Do not acknowledge stopped
and then schedule restoration in a background task. A separately qualified
recovery handoff must actually retain an authorized, invocable owner, not only
a record asserting that somebody will restore later. [S77]

SCM failure actions cover process death without stopped status and, when the
configured non-crash flag applies, stopped status with a nonzero exit code.
A successful planned stop is not equivalent. A recovery command runs under the
service's account, not an automatically elevated one. An already queued restart
cannot be canceled; Microsoft documents explicit disabling to prevent delayed
unexpected restart. Keep the durable maintenance/generation admission barrier
even when the service was manually started and stopped after the failure.
The failure flag's documented next-system-start activation remains separate from
configuration readback. [S54, S78, S79]

Neither the ordinary package uninstall event arguments nor the newer package
extension uninstall arguments establish a deferral/acknowledgement contract.
`SERVICE_NOTIFY_DELETE_PENDING` is an observation after DeleteService is invoked;
it needs a live observer and does not turn service handles into a supported
package-removal veto. DeleteService defers SCM entry removal until relevant
handles are closed and the service stops. That does not prove necessary package
payload, registration and recovery invocation remain available. [S57, S80, S81,
S87]

**Supported design inference:** independent recovery requires all of retained
trusted data, executable code, authority and an invocation path. An in-package
watchdog can supervise another role while alive, but shared package lifetime
alone proves no survivor after final removal. This preserves the bounded open
question; it does not assert that every Windows mechanism is impossible.

#### Alternatives that reduce or move the recovery problem

| Unselected approach | Useful property | Unresolved boundary |
| --- | --- | --- |
| Restore before the helper's own removal request | Controls that request's order | Independent Settings/administrative removal and Reset can bypass it. |
| Preserve W32Time startup configuration and only stop it temporarily | Reduces persistent changes | W32Time can be trigger/task started. Polling and stopping it again leaves a competing-controller interval; exclusivity still needs proof. [S82] |
| Retain an OS-owned recovery task using inbox executable actions | Could preserve an invocable mechanism without a separate installed application binary | Adds privileged state outside MSIX; needs explicit acceptance, protected data/action binding, suitable rights, update/successor discrimination, correct clock handling and eventual self-removal. [S83–S86] |

A fixed `sc.exe` sequence does not implement ownership-generation checks,
clock-rate cleanup and journal reconciliation. A surviving recovery task must
not restore stale pre-update state underneath a valid successor. A calendar
StartBoundary is a date/time, not evidence of a monotonic lease that remains
correct after large forward/backward clock corrections. A post-removal task
could only satisfy the existing alternative of retaining an invocable recovery
mechanism until restoration; it cannot prove restoration occurred before payload
removal. The hidden off-package watchdog remains unselected. [S84–S86]

**Counterexample to design against, not an observed product bug:** version A has
taken ownership and disabled W32Time; version B starts after an update, detects
unsafe token/configuration/mitigation state and correctly refuses normal work;
W32Time is still disabled. Fail-closed admission protects actuation authority but
does not itself restore synchronization availability. Qualify recovery despite
normal-work refusal before allowing the original takeover. Never weaken the
admission checks to make this scenario appear successful.

#### Bounded next evidence under the existing owners

Keep section 7.3's running-versus-stopped removal pair as the first lifecycle
discriminator. An external observer records controls, cleanup, dummy journal
completion, service deletion and payload/data lifetime; it must not perform the
claimed recovery. Only a credible mechanism surviving that pair justifies the
larger Reset/update/reboot/multiuser matrix.

For provisioning, the next inert package experiment compares disabled initial
services, refused premature start, interruption between configuration steps,
fresh effective tokens and one update/repair. An external administrative harness
can establish API behavior but cannot supply the production provisioning owner.
If the restricted-child alternative is selected for investigation, separately
observe child identity/privilege absence, permitted IPC, denied broker access,
restart after privilege removal and child lifetime after broker failure.
The delayed-restart maintenance case is a distinct later observation. None of
these requires real clock/W32Time changes for its first distinguishing result.

W11-W05 retains provisioning/security CHECK-1/2/15/16 and lifecycle/storage
CHECK-5/12/13/23/27; W11-W10 retains activation/mitigation CHECK-16/26/27 and
release CHECK-18/23/24. The
[continuation task](../../tasks/windows11-integration.md#415-provisioning-and-independent-removal-research--3-november-2026)
records disposition. Current Core priority and all Windows/fuzz/storage/UI holds
remain. Researching a safer Windows laboratory is not permission to run these
experiments on the developer's host or to retry the Defender-triggering fixture.

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
- **S31:** [Exploit Protection reference: CIG, ACG, CFG and other process mitigations](https://learn.microsoft.com/defender-endpoint/exploit-protection-reference).
- **S32:** [Exploit Protection configuration and PowerShell option names](https://learn.microsoft.com/defender-endpoint/customize-exploit-protection).
- **S33:** [Process-creation mitigation attributes](https://learn.microsoft.com/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute).
- **S34:** [uap10:Content schema, namespace and OS requirements](https://learn.microsoft.com/uwp/schemas/appxpackage/uapmanifestschema/element-uap10-content).
- **S35:** [Package-integrity enforcement and default behavior](https://learn.microsoft.com/windows/msix/desktop/tamper-protection).
- **S36:** [Signing overview and package-integrity enforcement](https://learn.microsoft.com/windows/msix/package/signing-package-overview).
- **S37:** [Packaged desktop behavior, installed content and virtualization](https://learn.microsoft.com/windows/msix/desktop/desktop-to-uwp-behind-the-scenes).
- **S38:** [AllowExternalContent](https://learn.microsoft.com/uwp/schemas/appxpackage/uapmanifestschema/element-uap10-allowexternalcontent).
- **S39:** [PackagePathType and mutable/external path precedence](https://learn.microsoft.com/windows/win32/api/appmodel/ne-appmodel-packagepathtype).
- **S40:** [MSIX containerization and trust levels](https://learn.microsoft.com/windows/msix/msix-containerization-overview).
- **S41:** [Packaged DLL search order](https://learn.microsoft.com/windows/win32/dlls/dynamic-link-library-search-order).
- **S42:** [LoadPackagedLibrary](https://learn.microsoft.com/windows/win32/api/winbase/nf-winbase-loadpackagedlibrary).
- **S43:** [SetProcessMitigationPolicy and activation restrictions](https://learn.microsoft.com/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessmitigationpolicy).
- **S44:** [Dynamic-code policy and exception fields](https://learn.microsoft.com/windows/win32/api/winnt/ns-winnt-process_mitigation_dynamic_code_policy).
- **S45:** [Image-load policy](https://learn.microsoft.com/windows/win32/api/winnt/ns-winnt-process_mitigation_image_load_policy).
- **S46:** [CFG policy](https://learn.microsoft.com/windows/win32/api/winnt/ns-winnt-process_mitigation_control_flow_guard_policy).
- **S47:** [User shadow-stack policy](https://learn.microsoft.com/windows/win32/api/winnt/ns-winnt-process_mitigation_user_shadow_stack_policy).
- **S48:** [CET compatibility flag](https://learn.microsoft.com/cpp/build/reference/cetcompat) and [EH continuation metadata](https://learn.microsoft.com/cpp/build/reference/guard-enable-eh-continuation-metadata).
- **S49:** [Store MSIX certification and publishing stages](https://learn.microsoft.com/windows/apps/publish/publish-your-app/msix/app-certification-process).
- **S50:** [Trusted Launch package-identity restriction and build floor](https://learn.microsoft.com/uwp/schemas/appxpackage/uapmanifestschema/element-trustedlaunch-trustedlaunch).
- **S51:** [Manual MSIX component/identity/application authoring](https://learn.microsoft.com/windows/msix/desktop/desktop-to-uwp-manual-conversion). Use its ordinary-UI authoring guidance, not its generic interactive-user note as a claim about SCM services.
- **S52:** [Application activation attributes and inheritance](https://learn.microsoft.com/uwp/schemas/appxpackage/uapmanifestschema/element-application).
- **S53:** [Package root and ignorable-namespace semantics](https://learn.microsoft.com/uwp/schemas/appxpackage/uapmanifestschema/element-f-package).
- **S54:** [ChangeServiceConfig2W configuration rights and setting classes](https://learn.microsoft.com/windows/win32/api/winsvc/nf-winsvc-changeserviceconfig2w).
- **S55:** [Service security and default object access](https://learn.microsoft.com/windows/win32/services/service-security-and-access-rights).
- **S56:** [SCM dispatcher connection and own-process initialization](https://learn.microsoft.com/windows/win32/api/winsvc/nf-winsvc-startservicectrldispatcherw).
- **S57:** [PackageUninstalling event arguments](https://learn.microsoft.com/uwp/api/windows.applicationmodel.packageuninstallingeventargs).
- **S58:** [RemovePackageAsync, cancellation and shutdown semantics](https://learn.microsoft.com/uwp/api/windows.management.deployment.packagemanager.removepackageasync).
- **S59:** [RemovalOptions and development-only data preservation](https://learn.microsoft.com/uwp/api/windows.management.deployment.removaloptions).
- **S60:** [Finite service preshutdown wait](https://learn.microsoft.com/windows/win32/api/winsvc/ns-winsvc-service_preshutdown_info).
- **S61:** [GetServiceDirectory and uninstall deletion](https://learn.microsoft.com/windows/win32/api/winsvc/nf-winsvc-getservicedirectory).
- **S62:** [GetServiceRegistryStateKey and uninstall deletion](https://learn.microsoft.com/windows/win32/api/winsvc/nf-winsvc-getserviceregistrystatekey).
- **S63:** [Microsoft desktop6 reference XSD](https://github.com/microsoft/msix-packaging/blob/25a65f5c1690930813bcc10cdf1d59fa865f2bb1/resources/AppxPackaging/Manifest/Schema/2018/DesktopManifestSchema_v6.xsd), pinned `CT_Service` and account/startup types.
- **S64:** [Microsoft desktop7 reference XSD](https://github.com/microsoft/msix-packaging/blob/25a65f5c1690930813bcc10cdf1d59fa865f2bb1/resources/AppxPackaging/Manifest/Schema/2020/DesktopManifestSchema_v7.xsd), same pinned source revision.
- **S65:** [LockServiceDatabase compatibility-only behavior since Vista](https://learn.microsoft.com/windows/win32/api/winsvc/nf-winsvc-lockservicedatabase).
- **S66:** [StartServiceW completion and disabled-service error](https://learn.microsoft.com/windows/win32/api/winsvc/nf-winsvc-startservicew).
- **S67:** [Predefined service triggers and manual-start example](https://learn.microsoft.com/uwp/schemas/appxpackage/uapmanifestschema/element-desktop10-predefinedtriggerevents).
- **S68:** [UserRights CSP ChangeSystemTime and required LocalService assignment](https://learn.microsoft.com/windows/client-management/mdm/policy-csp-userrights#changesystemtime).
- **S69:** [ChangeServiceConfigW, startup mode and virtual-account support](https://learn.microsoft.com/windows/win32/api/winsvc/nf-winsvc-changeserviceconfigw).
- **S70:** [QueryServiceConfig2W configuration readback](https://learn.microsoft.com/windows/win32/api/winsvc/nf-winsvc-queryserviceconfig2w).
- **S71:** [SetServiceObjectSecurity and descriptor rights](https://learn.microsoft.com/windows/win32/api/winsvc/nf-winsvc-setserviceobjectsecurity).
- **S72:** [CreateRestrictedToken and restricted own-token process creation](https://learn.microsoft.com/windows/win32/api/securitybaseapi/nf-securitybaseapi-createrestrictedtoken).
- **S73:** [CreateProcessAsUserW rights, descriptors and handle inheritance](https://learn.microsoft.com/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessasuserw).
- **S74:** [Permanent removal versus disabling token privileges](https://learn.microsoft.com/windows/win32/secbp/changing-privileges-in-a-token).
- **S75:** [Process security and dangerous object/handle access](https://learn.microsoft.com/windows/win32/procthread/process-security-and-access-rights).
- **S76:** [OWNER RIGHTS and implicit owner permissions](https://learn.microsoft.com/windows-server/identity/ad-ds/manage/understand-special-identities-groups#owner-rights).
- **S77:** [SetServiceStatus, cleanup and final stopped acknowledgement](https://learn.microsoft.com/windows/win32/api/winsvc/nf-winsvc-setservicestatus).
- **S78:** [Service failure actions and recovery-command account](https://learn.microsoft.com/windows/win32/api/winsvc/ns-winsvc-service_failure_actionsw).
- **S79:** [Non-crash failure-action conditions and activation](https://learn.microsoft.com/windows/win32/api/winsvc/ns-winsvc-service_failure_actions_flag).
- **S80:** [Service deletion notification and observer lifetime](https://learn.microsoft.com/windows/win32/api/winsvc/nf-winsvc-notifyservicestatuschangew).
- **S81:** [DeleteService and SCM entry lifetime](https://learn.microsoft.com/windows/win32/api/winsvc/nf-winsvc-deleteservice).
- **S82:** [W32Time trigger/task startup on Windows clients](https://learn.microsoft.com/troubleshoot/windows-client/active-directory/w32time-not-start-on-workgroup). Do not generalize its historical exact schedule to every supported build.
- **S83:** [Task Scheduler registered tasks](https://learn.microsoft.com/windows/win32/taskschd/tasks).
- **S84:** [Task actions and ordered executable operations](https://learn.microsoft.com/windows/win32/taskschd/task-actions).
- **S85:** [Task registration, execution identity and security](https://learn.microsoft.com/windows/win32/taskschd/security-contexts-for-running-tasks).
- **S86:** [Calendar-based trigger StartBoundary](https://learn.microsoft.com/windows/win32/taskschd/trigger-startboundary).
- **S87:** [Package-extension uninstall event arguments](https://learn.microsoft.com/uwp/api/windows.applicationmodel.packageextensions.packageextensionpackageuninstallingeventargs).
