# Windows product and package metadata

**Owner requirement and research:** 5 November 2026, Asia/Tbilisi.<br>
**Status:** accepted objective; proposed implementation contract. No metadata
generator, release identity or package is implemented by this document.<br>
**Owners:** OD-05, P6, W11-W10 with build/release, branding/localization and the
existing CHECK-18/20/23/26/27 owners. Service descriptions also involve W11-W05.

## 1. Objective and scope

David requires correct, extensive metadata regardless of whether delivery uses
MSIX or a later selected MSI route. The objective is to populate every useful,
applicable, supported descriptive field and explain deliberate omissions. It is
not to maximize text length, invent release facts, or activate optional package
behavior merely because an XML element or installer property exists.

The selected delivery remains one complete hardened MSIX under
[OD-05](../nevrotick-owner-decisions.md#7-od-05--signing-and-release-ownership).
MSI mappings are retained for possible later owner reconsideration, not a second
deliverable or a reason to defer shared executable metadata. This contract
supplements the [MSIX field profile](windows-msix-store.md#71-concrete-source-inventory-and-manifest-profile),
the [P6 plan](development-plan.md#p6--complete-installation-hardening-and-release-artifacts)
and [W11-W10](../../tasks/windows11-integration.md#w11-w10-re-enter-conditional-mechanisms-only-for-a-concrete-need).
Existing execution holds, Core priority, actual licensing admission and optional
enhanced-provenance policy remain unchanged.

At the inspected NevroTick baseline
`21726b2db44f225da0aaed82f03255c2f2c81023`, Cargo records project license,
repository and crate descriptions; the UI crate is version `0.0.0` and explicitly
describes a fixture prototype. The searched product/build paths contain no
maintained `VERSIONINFO`, `.rc`, package manifest or MSI authoring source. This
is a source inventory observation, not inspection of every historical executable.
Cargo metadata does not by itself populate Windows Explorer or installer fields.
The existing [package-artifact skill](../../.agents/skills/testing/qa-package-artifacts/SKILL.md)
already requires inspecting emitted versions/resources/manifests; this plan adds
the missing comprehensive field map and consistency rules.

## 2. Separate the metadata surfaces

| Surface | Authoritative contents and consumers |
| --- | --- |
| Shared product/release record | Product name, publisher display name, public URLs, rights, localized descriptions, release/channel mapping and component inventory |
| Every first-party shipped PE | `VERSIONINFO`, icon resources, and a separate reviewed application manifest; Explorer Details, diagnostics and process identification |
| MSIX manifest/resources | Package identity, package/application display fields, resources and supported platform declarations; deployment, Start and Settings |
| MSI database | Product/upgrade/package identity, Property table, Summary Information stream, icon and localized installer resources; Windows Installer and installed-app information |
| Installed integration | Actual package or MSI registration, shortcut display/icon/AppUserModelID where applicable, service display names and descriptions |
| Store submission | Localized descriptions, features, screenshots, search terms, category, support/privacy/license information; a separate submission surface |
| Release and component evidence | Source revision, dependency/native payload inventory, hashes, license notices and existing provenance records; support, audit and release custody |

No single field supplies all of these. The display publisher, certificate subject,
Store-assigned identity, Cargo author and original upstream copyright holders
are distinct facts. Descriptive strings cannot authorize a helper or prove the
trustworthiness of a binary. [PE] [MSIX-PROPERTIES] [MSI-ARP] [STORE-IDENTITY]

## 3. Where windows-rs helps

The review checked release-associated `windows` **0.62.2** source at
`32c3144490c016fe496a0aed769bce60987a2e9d`; its Cargo manifest declares the
features below. The symbols were found in the named source modules. This proves
binding presence, not a new dependency admission, target build or native pass.
NevroTick currently uses `windows-sys` 0.61.2 for its UI; retain that graph until
the existing dependency owner admits any additional validation-tool bindings.

| Need | Verified `windows` module / feature and API examples |
| --- | --- |
| Read emitted EXE/DLL version information | `Win32::Storage::FileSystem` / `Win32_Storage_FileSystem`: `GetFileVersionInfoSizeW`, `GetFileVersionInfoW`, `VerQueryValueW` |
| Inspect MSIX package/manifest | `Win32::Storage::Packaging::Appx` / `Win32_Storage_Packaging_Appx`: `IAppxFactory::CreatePackageReader`, `IAppxManifestReader`, `GetPackageId`, `GetProperties`; enumerate actual applications/resources/payload through the appropriate interfaces |
| Inspect MSI without installing it | `Win32::System::ApplicationInstallationAndServicing` / corresponding feature: `MsiOpenDatabaseW`, `MsiDatabaseOpenViewW`, `MsiGetSummaryInformationW`, `MsiSummaryInfoGetPropertyW` |
| Compare installed MSI registration | Same MSI module: `MsiGetProductInfoExW`, using the actual product, user and installation context |
| Read current installed MSIX identity/display data | `ApplicationModel` / `ApplicationModel`: `Package::Current`, `Id`, `DisplayName`, `PublisherDisplayName`, `Description`; an unpackaged process must handle absence of package identity |

Sources: [BINDINGS-MANIFEST], [BINDINGS-PE], [BINDINGS-MSIX], [BINDINGS-MSI],
[BINDINGS-PACKAGE]. Use selected Windows bindings rather than handwritten FFI or
a new home-grown PE/MSI binary parser. COM initialization, allocations, handle
ownership, encoding, buffer lengths, error distinctions and target support still
belong to the adapter. This is an API-surface map, not a complete Cargo feature
stanza: COM stream/initialization and other method dependencies must also be
included in the admitted validation-tool graph. For an MSI inspection tool use read-only database access
and zero requested summary updates; querying metadata must not install a package
or run its custom actions. [MSI-SUMMARY-API]

Bindings also expose writer operations such as `CreatePackageWriter`; windows-rs
is not limited to reading. Nevertheless, the recommended split is **declarative
authoring with admitted build/package tools, independent readback with Windows
APIs**. Resource compilation/linking, MakeAppx, selected MSI authoring and actual
signing remain build/release responsibilities. A projection does not invent the
manifest, compile `.rc` files, maintain versions or operate Partner Center.

Two tempting crate names have different purposes. At inspected upstream master
`db9801679bac964b6367b1d1d1bf8472ab001065`, `windows-metadata` reads/writes
ECMA-335 API metadata such as `.winmd`; it is not a product-description editor.
`windows-version` reports operating-system version information; it is not a
writer for our executable's `ProductVersion`. [WINDOWS-METADATA] [WINDOWS-VERSION]

## 4. Shared maintained record and provenance of values

**Proposed implementation:** a versioned, machine-readable source such as
`packaging/product.toml`, with per-component records and localization resources.
The filename/serialization is a proposal; do not create a second independent
version authority. The existing Cargo/release versions remain inputs until a
reviewed release-version policy establishes their relationship.
The [metadata workflow skill](../../.agents/skills/implementation/maintain-windows-metadata/SKILL.md)
and [section 11](#11-harvesting-storage-and-use-workflow--5-november-2026)
provide the reusable collection, authority, storage and use procedure; this
document remains the canonical field map.

| Group | Information to maintain | Value origin |
| --- | --- | --- |
| Product | `NevroTick`, short/long descriptions, intended component roles | Owner-approved product definitions; shipping descriptions reflect implemented capabilities |
| Publisher | Public display name `David Osipov`; developer/manufacturer labels | Owner identity; Store display spelling must match Partner Center when submitted |
| Public destinations | Repository, homepage, documentation, support, update/release notes, privacy and license URLs; optional public support email/phone | Actual maintained endpoints. The repository URL is established; dedicated support/privacy pages and contact availability need validation |
| Rights | Current code license, copyright notices, third-party notices, brand/asset rights | Exact admitted source and license inventory; preserve upstream notices and pending license questions |
| Versioning | Logical product version, release channel, numeric Windows versions, MSI upgrade family/product/package identifiers if selected | Explicit mapping and release process, not guessed GUIDs or independent edits |
| Platform | Actual architectures, minimum OS, tested OS, installer engine floor if relevant, CPU prerequisites | Admitted target/build/runtime policy. `x64` alone does not express the required x86-64-v3 CPU/OS-state prerequisite |
| Components | Filename, internal name, role description, file type, numeric/file display version, icon, language resource mapping | Actual frozen build outputs; UI, engine, broker, helper and shipped diagnostics are distinct |
| Assets/locales | Source asset attribution, generated PNG/ICO variants, supported UI/installer languages, translated strings, fallback | Existing branding sources plus actual generated resources; Store listing languages tracked separately |
| Build/release evidence | Commit, dependency lock/inventory, toolchain/profile, release timestamp if defined, final hashes and stage identity | Measured/generated release records; source, submitted and delivered bytes remain distinguishable |

For every field the register records its output surface, schema/type/length,
localization rule, owner, source/derivation, applicability and status:
**populated**, **derived**, **not applicable with reason**, or **pending with
owner**. Validation aims for complete disposition and full population of the
approved applicable fields. A fake support number, invented Store ID or empty
placeholder does not count as completeness. Optional fields with no meaningful
value are deliberately absent.

Use field-specific descriptions rather than one huge paragraph everywhere. Do
not emit planned NTS/Roughtime/DNS functionality as an implemented release
capability. Do not publish build-machine paths, usernames or local contact data
as a side effect of populating author/build fields. Keep enhanced public
attestations/rebuild proofs optional under OD-05 §7.5; rich descriptive metadata
does not turn them into new release blockers.

## 5. Executable and DLL metadata, independent of installer format

Populate a version resource for every first-party executable/DLL that actually
ships, including headless services. Preserve third-party file identities rather
than replacing their authorship with NevroTick's. Resource compilation is a build
step before final signing and hashing. [PE]

| Resource field | NevroTick rule |
| --- | --- |
| `CompanyName` | Actual publisher/developer display name; `David Osipov` is the current intended value, not a fabricated company |
| `ProductName` | `NevroTick`, consistent across product-owned components |
| `FileDescription` | Role-specific: e.g. UI, synchronization engine, clock broker or administration helper; prototype builds remain labeled as prototypes |
| String `FileVersion` / `ProductVersion` | Readable component/product versions; may preserve channel/prerelease text under the version policy |
| Numeric `FILEVERSION` / `PRODUCTVERSION` | Four 16-bit components with defined mapping; validate against the corresponding readable strings and release inventory |
| `InternalName` | Stable component/module name; when no separate internal name exists, original filename without extension |
| `OriginalFilename` | Actual shipped basename including extension, never a build path |
| `LegalCopyright` | Accurate applicable notices, including retained third-party rights where required |
| `LegalTrademarks` | Only an accurate applicable trademark statement; no invented registration claim |
| `Comments` | Concise useful component/release context; not the only home for support, licensing or provenance |
| `PrivateBuild` / `SpecialBuild` | Only when their corresponding `VS_FF_PRIVATEBUILD` / `VS_FF_SPECIALBUILD` flags truthfully apply |
| `FILEFLAGSMASK`, `FILEFLAGS`, `FILEOS`, `FILETYPE`, `FILESUBTYPE` | Derive from actual binary/build classification; app versus DLL, release/debug/prerelease semantics and valid flags; no unsupported capability claims |
| `StringFileInfo` language/codepage blocks and `VarFileInfo/Translation` | Matching actual Unicode/localized resources; enumerate available translations when reading back |

Microsoft documents the standard string names and fixed fields in [PE] and
[PE-STRINGS]. Custom strings can carry additional project context, but they are
not guaranteed to appear in Explorer. Put detailed source/license/build
information in the release inventory and About/diagnostics surfaces rather than
relying on arbitrary resource keys. A `Cargo.toml` `authors`/`description` value
or an embedded application manifest is not a substitute for `VERSIONINFO`.

Give each shipped application its appropriate icon and reviewed application
manifest. Execution level, DPI/compatibility declarations and mitigations are
behavioral contracts under existing owners, not decorative metadata to enable
en masse. Services' display names and SCM descriptions should explain the
engine/broker distinction without changing their stable service identities,
accounts, privileges or startup policy merely to improve labels.

## 6. MSIX manifest, resources and installed identity

| Surface / fields | Rule |
| --- | --- |
| `Identity/Name`, `Publisher` | Exact assigned Store values for that route, or admitted certificate-matching identity for another authorized route; not user-facing marketing strings |
| `Identity/Version`, `ProcessorArchitecture`, conditional `ResourceId` | Version mapping, actual code architecture and actual resource-package semantics; no invented resource identity |
| `Properties/DisplayName`, `PublisherDisplayName`, `Description`, `Logo` | Fully populate applicable descriptive fields, including the optional useful package description; follow current schema and channel identity requirements |
| Application `Id`, `Executable` and existing activation declarations | Stable per-application identity and actual payload; preserve the selected ordinary-user UI and separate service/helper contract |
| `uap:VisualElements` | Actual display name, description, background and square 44/150 image resources; conditional short name/tile/splash assets only where used |
| `Resources/Resource` and PRI resources | Real supported resource languages and qualified assets; localized manifest strings use resolvable `ms-resource:` references |
| `Dependencies/TargetDeviceFamily` | Qualified device-family/schema and actual `MinVersion` and `MaxVersionTested`; resolve the candidate `Windows.Desktop` for both [OD-14 OS rows](../../tasks/windows11-integration.md#432-two-family-windows-qualification-matrix--5-november-2026) through the MSIX profile. Tested-version metadata is not an upper support bound or evidence of OS qualification |
| Other dependencies/extensions | Exact dependencies and implemented integration; no protocol/file association/COM/startup declarations solely to increase metadata count |
| Derived installed identifiers | Read and retain PFN, full name, actual installed version and application identity; do not maintain a hand-invented duplicate of derived identifiers |

`DisplayName`/`PublisherDisplayName` are display strings; `Identity/Publisher`
must satisfy the signing identity requirements. Store product identity supplies
`Name`, `Publisher` and publisher display information. The package family name
and package SID are derived values, not extra identity elements to add to the
manifest. [STORE-IDENTITY] [MSIX-IDENTITY]

Package properties and application visual properties are different surfaces.
The inspected VisualElements schema permits display names of 1–256 characters
and descriptions of 1–2,048; validate each actual field against its own schema.
Its logo paths accept PNG/JPG/JPEG, not the project's AVIF masters. Generate
appropriate package PNG assets and executable/installer ICO resources from the
approved branding sources, with asset-size and localization checks. [MSIX-VISUAL]

Maintain translated descriptions and resource references, build the PRI as
needed, and check fallback resolution. A translated Store listing is not proof
that the installed application supports that UI language. [MSIX-LOCALIZE]
[STORE-LISTING]

Many children under `Properties` change behavior: framework/resource-package
classification, execution, virtualization, external content, multi-user scope,
integrity and Trusted Launch. They stay under the selected
[MSIX protection profile](windows-msix-store.md#3-package-trust-and-the-privileged-helper).
Likewise, no extra capability is justified by the desire for more metadata.
Unknown attributes or elements must not be inserted as a custom data store.
Keep additional project information in a schema-defined bundled release record
or the proper Store field. [MSIX-PROPERTIES]

## 7. MSI metadata if that delivery route is selected later

### 7.1 Product and installed-app information

| Property or field | Rule |
| --- | --- |
| `ProductName`, `Manufacturer` | Product and actual publisher, consistent with summary and binary display fields |
| `ProductVersion`, `ProductLanguage` | MSI numeric version and actual installer language; product-language metadata is not a list of every UI language |
| `ProductCode` | Product identity governed by Windows Installer update rules; major upgrades require the appropriate new product identity |
| `UpgradeCode` | Maintained identity for the related product family under the selected upgrade design |
| Package code | Unique identity for the actual installer package, stored in Summary Information `Revision Number`; different MSI bytes must not reuse a package code |
| `ARPCOMMENTS`, `ARPCONTACT` | Useful description and real public support contact |
| `ARPHELPLINK`, `ARPURLINFOABOUT`, `ARPURLUPDATEINFO` | Maintained support, product and update-information URLs |
| `ARPHELPTELEPHONE` | Only an actual supported public contact number; absence is preferable to a private or fictitious number |
| `ARPREADME` | Real installed readme or documented destination |
| `ARPPRODUCTICON` | Valid reference to an actual Icon table entry |
| `ARPINSTALLLOCATION` | Resolved installed path, not a hard-coded developer machine path |
| `ARPSIZE` | Only if the authoring design needs a supported explicit estimate, derived from the actual payload; preserve Windows Installer's size calculation when appropriate |

These fields are mapped by Windows Installer to installed-app registration.
Author them through the installer rather than writing a competing Uninstall
registry entry manually. Windows Installer derives values such as
`DisplayVersion`, `UninstallString`, `ModifyPath` and `EstimatedSize`.
Its `InstallDate` may change after servicing and is not an immutable original
installation timestamp. [MSI-ARP] [MSI-UNINSTALL] [MSI-IDENTITY]

`ARPNOMODIFY`, `ARPNOREPAIR`, `ARPNOREMOVE`, `ARPSYSTEMCOMPONENT`, installation
scope and update-channel properties such as `ARPAUTHORIZEDCDFPREFIX` affect
behavior or discoverability. They
must follow the selected lifecycle, not be filled simply because optional.
Avoid hiding this ordinary product or suppressing maintenance controls as a
metadata-completeness exercise. [MSI-ARP]

### 7.2 The separate Summary Information stream

| Summary field | Correct treatment for a shipping MSI installation database |
| --- | --- |
| Title | Meaningful installation-database title |
| Subject / Author | Product name / manufacturer consistent with Property table |
| Keywords / Comments | Relevant installer/product search terms and an accurate package description |
| Template | Correct platform and installer language, e.g. `x64;1033` for an actually English x64 installer; `Intel64` means Itanium |
| Revision Number | Package-code GUID, not product version, source commit or changelog revision |
| Creating Application | Actual MSI authoring software |
| Codepage | Correct representation for strings, set according to the authoring API/tool's requirements before strings |
| Create Time/Date / Last Save Time/Date | Actual authored-package timestamps under the reproducibility policy; distinct from signing timestamp and installed servicing date |
| Page Count | Minimum Windows Installer engine version encoding, not document pages |
| Word Count | Source-image and relevant installer flags, not the description's word count |
| Security | Summary recommendation semantics; Microsoft's installation-database guidance says 2, recommended read-only; this is not cryptographic integrity |
| Last Saved By | Null in a final shipping installation database, as Microsoft specifies |
| Character Count | Null for an installation package |
| Last Printed | Administrative-image semantics; not a field to fill with a fabricated date |

The same summary names have different meanings for MSI installation databases,
MST transforms and MSP patches. Validate against the actual container type.
The Template schema has specific architecture/language restrictions; do not
claim x86 and x64 together just to advertise broad compatibility.
[MSI-SUMMARY] [MSI-SUMMARY-SET] [MSI-TEMPLATE]

## 8. Version and localization rules that prevent silent errors

| Surface | Representation and constraint |
| --- | --- |
| Logical/Cargo release version | May include semantic-version prerelease/build information; current `0.0.0` prototype versions are not a selected Store numbering policy |
| PE fixed version | Four unsigned 16-bit values; human-readable version strings can retain extra display context |
| MSIX identity version | Numeric quad. For the documented Windows 10/11 Store package route, leave the fourth component zero for Store use and use a nonzero major component |
| MSI `ProductVersion` | Three significant components: major ≤255, minor ≤255, build ≤65535. Windows Installer ignores a fourth component for product-version comparison |

A simple stable example is logical `1.2.3`, PE `1.2.3.0`, MSIX `1.2.3.0`, MSI
`1.2.3`; this is illustrative, not selection of NevroTick's next release.
Previews and multiple releases from one logical version need a defined monotonic
numeric mapping that cannot collide. Preserve the logical/channel identifier
separately rather than forcing a semantic version such as `0.1.0-alpha.2` into
an incompatible installer field. Check downgrade/update behavior using the
selected channel rules. Store-delivered revision changes need an allowed mapping
and distinct receipts, not a false byte-for-byte equality requirement.
[PE] [MSIX-VERSION] [MSI-VERSION]

Package identity, MSI upgrade identity and display branding have different
lifetimes. Routine renaming must not accidentally create another product family;
routine regeneration must not break upgrade detection. Keep authored
descriptive translations separate from invariant identifiers. PE language/code
page pairs, MSI LANGIDs and MSIX resource language tags require explicit mapping.

## 9. Store metadata and other distribution descriptions

Maintain Store input alongside the shared record, with channel-specific fields.
It is not embedded into MSIX merely by adding XML, and an MSI distributed through
the Store also has a separate listing/submission. Populate the meaningful
optional fields in addition to mandatory ones. [STORE-LISTING] [STORE-MSI]

- Reserved product name; primary/secondary category and applicable subcategory.
- Clear localized long and short descriptions, implemented feature list and
  release notes for updates; leave first-submission “What's new” appropriately
  empty rather than inventing an earlier release.
- Genuine screenshots of the shipped application; appropriate Store logos,
  supplementary artwork and a trailer only if one exists.
- Relevant keywords, short/sort/voice title where useful, and accurate minimum
  and recommended hardware requirements. Distinguish CPU prerequisites from
  memory observations on a private fixture.
- Public website, support and privacy URLs; actual developer attribution;
  copyright/trademark information and applicable license terms.
- Truthful age/content/privacy/capability declarations and distribution details
  required by the actual submission. These declarations are not marketing filler.

Checked MSIX listing limits include 10,000 characters for the description,
1,500 for update notes, up to 20 feature entries of at most 200 characters,
50 for short title, 255 for sort/voice title and 1,000 for short description
(Microsoft recommends less than 270 for common views). Additional information
allows seven search terms, each at most 40 characters and no more than 21 words
overall; copyright/trademark information is 200 characters and developer
attribution 255. Additional license terms accept a URL or up to 10,000 text
characters. Recheck these mutable channel limits at submission; they are not
targets to fill with repetitive text. [STORE-LISTING] [STORE-ADDITIONAL]

The Store's Published by label comes from the account's publisher display name;
the optional Developed by field is separate. Listing languages can exceed
package languages, but must not misrepresent actual UI translation. Privacy,
support, licenses and website belong in their designated fields. Cargo metadata,
a future WinGet manifest and an SBOM have their own schemas and consumers; reuse
appropriate values if those channels are admitted, without silently adding a
distribution channel or making every fact a PE custom string.
[STORE-ADDITIONAL] [STORE-PROPERTIES]

## 10. Generation, verification and existing task handoff

1. **Inventory and assign.** Under W11-W10/P6, complete the field register from
   actual product/component/rights records. Identify pending Partner Center
   identity, public support/privacy endpoints, release numbering, translations
   and authoring-tool inputs. Existing schemas determine valid absence.
2. **Generate deterministically.** From reviewed data and frozen release inputs,
   produce per-binary resources, selected package inputs, localization resources,
   About/diagnostic version data and the relevant Store listing export. XML/RC/
   MSI/CSV escaping and field limits are explicit generator responsibilities.
3. **Inspect emitted artifacts independently.** Compare expected values with
   actual PE resources, MSIX reader results or read-only MSI database and summary
   queries. Resolve resources, inventory all shipped first-party components and
   check genuine role-specific filenames/descriptions, icons and translations.
   A generator unit test alone cannot prove embedded resource presence.
4. **Bind final bytes.** Finish resources before signing. Freeze and inspect the
   signed/submitted artifacts and their hashes; post-signing metadata edits
   require a newly built/signed candidate. Keep Store-delivered transformation
   and signing evidence distinct under the existing release profile.
5. **Observe the installed surfaces at the existing native gate.** Check Explorer
   Details, Start, Settings/installed-app information, service descriptions and
   About/diagnostics for the admitted package and locales. Windows surfaces may
   expose different subsets; preserve optional useful metadata even if one view
   does not display it. Compare installed identity with the actual artifact.
6. **Prevent regressions.** At the affected build/release gate, challenge missing
   required strings, placeholder URLs/identities, unresolvable resources,
   inconsistent versions, invalid Unicode/lengths, wrong architecture, unsupported
   language claims and unintended identity churn. Pair rejection with a valid
   neighboring example. Source schema success is distinct from native behavior.

This is a proposed build/release workflow through existing owners, not a new
always-running application service. It requires no installer-inspection engine
inside the clock broker. Keep the validator's bindings/features scoped to its
actual readback needs; do not add a new general Windows manager or duplicate
the existing signing/SBOM/package validation infrastructure.

**Exit for this research:** the correct surfaces, fields, API coverage and traps
are documented. **Implementation exit remains open:** an admitted generator,
actual emitted artifacts and selected installed observations must substantiate
completeness. No installer, registry, service, certificate or Store account was
modified in this research; no package build or installation was run. Delegated
research attempts returned no usable review, so this is coordinator research,
not independently reviewed implementation evidence.

## 11. Harvesting, storage and use workflow — 5 November 2026

The follow-up research used Microsoft Learn, GitHub, Context7, Firecrawl developer
search/page extraction and web retrieval. The new
[maintain-windows-metadata skill](../../.agents/skills/implementation/maintain-windows-metadata/SKILL.md)
contains the operational procedure, four conditional references, primary-source
pins and two clearly marked example records. The recommended architecture is
**reviewed authored data plus immutable observations, deterministic projections
and independent artifact readback**. This is an engineering recommendation
within OD-05 §7.6; no collector or production metadata source is implemented here.

| Stage | Recommended method | Required distinction |
| --- | --- | --- |
| Harvest | Repository/Cargo/license/brand records first; admitted frozen Cargo/SBOM tooling; read-only PE/MSIX/MSI interfaces; existing authorized Store exports | Tool-observed or imported values remain candidates until reconciled with the correct field authority |
| Store intent | Versioned TOML, proposed `packaging/product.toml`, component and locale records; reference existing Cargo/release/license owners | One authority per fact; no duplicate manually edited release versions or implicit Cargo custom-metadata inheritance |
| Store observations | Versioned JSON in existing candidate evidence, bound to input hashes, collector, time, stage, locale/context, raw value and outcome | Intended, generated, embedded, Windows-resolved, installed, submitted and delivered values are different evidence |
| Use | Generate per-component RC resources, selected package inputs, localization, About/diagnostics and Store CSV projections | Explicit schemas, destination escaping and version/locale mapping; no invented identities or added capabilities |
| Verify | Compare independent readers against reviewed expectations, then observe native installed surfaces at their existing gate | Generated inputs, parser success, native display, signature trust and actual Store delivery prove different properties |

Use a small Git-reviewed source instead of introducing a database/service. Keep
large raw evidence and final artifacts in the existing owned artifact locations,
with hashes and task pointers. Retain the admitted CycloneDX 1.5 Rust inventory
and native/asset reconciliation; a product TOML is not a replacement SBOM. Every
field carries its destination, owner, source/derivation, type/limits, locale,
required stage and populated/derived/pending/justified-absence disposition.
Promote harvested changes through a semantic diff; never silently replace a
reviewed desired value with an erroneous binary's output. Preserve collection
time separately from stable release/build inputs.

The additional research establishes several concrete failure cases:

- Cargo metadata is a resolution/discovery surface. Its target filtering and
  package declarations do not prove a complete build-unit feature graph or
  shipped native/generated payload; its implementation also obtains compiler
  target information. Use frozen admitted tooling and the existing SBOM owner.
- Cargo custom metadata tables have tool-defined semantics. Set validation and
  fallback rules explicitly; arbitrary keys do not populate Windows resources.
- Windows version APIs can merge fixed data and MUI strings. Even the neutral
  flag can return MUI text; preserve raw-file and resolved-locale observations.
- An MSIX package reader validates structure/footprint data, not its digital
  signature. PRI dumps and actual localized presentation also remain distinct.
- MSI Property/Summary reads need read-only access and zero summary updates.
  Installed queries need correct user/context. Avoid Win32_Product: its queries
  can trigger consistency checks and repairs of installed applications.
- Resource helpers need explicit value review. At the pinned winresource source,
  FileDescription's constructor default differs from its documentation table.
  Explicit RC inputs or setters plus actual emitted-resource checks avoid relying
  on those defaults; no new helper dependency is selected by this observation.

The skill's [source register](../../.agents/skills/implementation/maintain-windows-metadata/references/sources.md)
links the official contracts and immutable Cargo/helper code supporting these
findings. Its [generation reference](../../.agents/skills/implementation/maintain-windows-metadata/references/generation-and-verification.md)
maps independent controls for version bounds, placeholders, wrong publishers,
Unicode, unresolved resources, omitted linked resources, denied/truncated reads
and stale candidates. Example records are illustrative contracts, not an adopted
schema implementation or passing release evidence. W11-W10/P6 and existing
CHECK-18/20/23/26/27 retain implementation and verification ownership; all current
execution holds and optional enhanced-provenance policy remain.

## Primary sources

Microsoft Learn schemas and Windows Installer references were retrieved using
the Microsoft Learn MCP and checked against primary web documentation on
5 November 2026. Binding locations were verified by reading pinned Git source;
no upstream code was executed.

[PE]: https://learn.microsoft.com/windows/win32/menurc/versioninfo-resource
[PE-STRINGS]: https://learn.microsoft.com/windows/win32/menurc/stringfileinfo-block
[MSIX-PROPERTIES]: https://learn.microsoft.com/uwp/schemas/appxpackage/uapmanifestschema/element-f-properties
[MSIX-IDENTITY]: https://learn.microsoft.com/uwp/schemas/appxpackage/uapmanifestschema/element-f-identity
[MSIX-VISUAL]: https://learn.microsoft.com/uwp/schemas/appxpackage/uapmanifestschema/element-uap-visualelements
[MSIX-LOCALIZE]: https://learn.microsoft.com/windows/uwp/app-resources/localize-strings-ui-manifest
[MSIX-VERSION]: https://learn.microsoft.com/windows/apps/publish/publish-your-app/app-package-requirements?pivots=store-installer-msix
[MSI-ARP]: https://learn.microsoft.com/windows/win32/msi/configuring-add-remove-programs-with-windows-installer
[MSI-UNINSTALL]: https://learn.microsoft.com/windows/win32/msi/uninstall-registry-key
[MSI-SUMMARY]: https://learn.microsoft.com/windows/win32/msi/summary-property-descriptions
[MSI-SUMMARY-SET]: https://learn.microsoft.com/windows/win32/msi/summary-information-stream-property-set
[MSI-SUMMARY-API]: https://learn.microsoft.com/windows/win32/api/msiquery/nf-msiquery-msigetsummaryinformationw
[MSI-TEMPLATE]: https://learn.microsoft.com/windows/win32/msi/template-summary
[MSI-VERSION]: https://learn.microsoft.com/windows/win32/msi/productversion
[MSI-IDENTITY]: https://learn.microsoft.com/windows/win32/msi/windows-installer-best-practices#keep-package-names-and-package-codes-consistent
[STORE-IDENTITY]: https://learn.microsoft.com/windows/apps/publish/view-app-identity-details
[STORE-LISTING]: https://learn.microsoft.com/windows/apps/publish/publish-your-app/msix/add-and-edit-store-listing-info
[STORE-ADDITIONAL]: https://learn.microsoft.com/windows/apps/publish/publish-your-app/msix/add-additional-information
[STORE-PROPERTIES]: https://learn.microsoft.com/windows/apps/publish/publish-your-app/msix/enter-app-properties
[STORE-MSI]: https://learn.microsoft.com/windows/apps/publish/publish-your-app/msi/enter-app-properties
[BINDINGS-MANIFEST]: https://github.com/microsoft/windows-rs/blob/32c3144490c016fe496a0aed769bce60987a2e9d/crates/libs/windows/Cargo.toml
[BINDINGS-PE]: https://github.com/microsoft/windows-rs/blob/32c3144490c016fe496a0aed769bce60987a2e9d/crates/libs/windows/src/Windows/Win32/Storage/FileSystem/mod.rs
[BINDINGS-MSIX]: https://github.com/microsoft/windows-rs/blob/32c3144490c016fe496a0aed769bce60987a2e9d/crates/libs/windows/src/Windows/Win32/Storage/Packaging/Appx/mod.rs
[BINDINGS-MSI]: https://github.com/microsoft/windows-rs/blob/32c3144490c016fe496a0aed769bce60987a2e9d/crates/libs/windows/src/Windows/Win32/System/ApplicationInstallationAndServicing/mod.rs
[BINDINGS-PACKAGE]: https://github.com/microsoft/windows-rs/blob/32c3144490c016fe496a0aed769bce60987a2e9d/crates/libs/windows/src/Windows/ApplicationModel/mod.rs
[WINDOWS-METADATA]: https://github.com/microsoft/windows-rs/blob/db9801679bac964b6367b1d1d1bf8472ab001065/crates/libs/metadata/readme.md
[WINDOWS-VERSION]: https://github.com/microsoft/windows-rs/blob/db9801679bac964b6367b1d1d1bf8472ab001065/crates/libs/version/src/lib.rs
