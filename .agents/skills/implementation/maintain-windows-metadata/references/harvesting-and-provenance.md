# Harvesting and provenance

Use this reference to collect evidence without silently changing its meaning or
the inspected system. Read [sources](sources.md) for primary contracts and
version pins. The [metadata plan](../../../../../docs/plans/windows-package-metadata.md)
owns the Windows field inventory.

## Contents

- [Choose the source by question](#choose-the-source-by-question)
- [Collect repository and Cargo facts](#collect-repository-and-cargo-facts)
- [Inspect files and packages](#inspect-files-and-packages)
- [Collect installed and Store facts](#collect-installed-and-store-facts)
- [Bind and admit observations](#bind-and-admit-observations)

## Choose the source by question

| Question | Preferred harvest | Limit to retain |
| --- | --- | --- |
| What should this release say? | Accepted owner/project records, exact Cargo/license/brand sources, approved descriptions and translations | Design intent does not prove generated or shipped content |
| What did a source generation consume and promise? | Actual generated source tree, its receipt and source-delivery contract, checked against current admitted inputs | Source hashes and required bindings establish neither compiled resource delivery nor native/signature observations |
| Which Rust packages and declared metadata are involved? | Frozen Cargo JSON and the existing SBOM wrapper | Resolver inventory is not a complete build-unit or packaged-file inventory |
| What is embedded in a particular file/package? | Exact hashed file, admitted PE/resource reader, read-only Appx/MSI interfaces | Metadata is untrusted data, not publisher authentication |
| What does Windows present for an installed candidate? | Package/Installer registration and scoped actual UI/resource observations | User, machine, OS, locale, resource packs and cache context affect presentation |
| Which service/event metadata is actually registered? | Scoped admitted SCM or publisher-metadata reader for the exact installed candidate | Stable identity, raw resource reference, resolved message and current runtime state are separate observations |
| What identity/listing did the Store assign or retain? | Existing authorized Partner Center identity record and listing export | Keep assigned identity, submission, published listing and delivered bytes distinct |
| What is a field/API allowed to mean? | Microsoft Learn, Cargo documentation and pinned upstream implementation | Search results locate sources; they do not approve product values |

Use connected Microsoft Learn for Windows contracts, GitHub for exact source,
Context7 for library discovery and web/Firecrawl for gaps and corroboration.
Confirm consequential indexed summaries in the underlying official source.
Do not send private source, raw machine metadata or account exports to a public
search service. Use unavailable-tool fallbacks; do not install a new MCP just
to repeat a completed lookup.

## Collect repository and Cargo facts

1. Resolve the actual checkout, inspect dirty state and read applicable rules.
   Inventory Cargo manifests/lockfile/toolchain, release policy, current component
   filenames, rights/notices, brand assets and localization. Use `rg` and direct
   file reads before creating an index or collector. Record relevant input hashes;
   a commit alone misses dirty/untracked inputs.
2. For purely static discovery, parse the owned TOML/JSON and read license/notice
   files. A Cargo command is a separate admitted tool operation: inspect Cargo
   configuration, toolchain overrides, wrappers and target behavior first.
3. When execution is admitted, obtain machine-readable Cargo data with a pinned
   format and frozen resolution. These are command shapes, not permission to
   bypass [configured tooling](../../../testing/references/configured-tooling.md):

   ```sh
   cargo metadata --format-version 1 --locked --offline --no-deps
   cargo metadata --format-version 1 --locked --offline --filter-platform x86_64-pc-windows-msvc
   ```

   Supply the actual build's default-feature choice and explicit features to the
   second command. Capture stdout and stderr separately into a fresh owned
   evidence location, with exit status, command, tool identity and input digest.
   `--frozen` combines locked/offline behavior. Missing cache or a required lockfile
   change leaves collection incomplete; do not silently fetch/update to get a pass.
4. Interpret the JSON, not formatted `cargo tree` text, for a new collector.
   Keep package IDs opaque, select roots through `workspace_members`/the actual
   component selection, retain dependency kinds and target predicates, and
   distinguish declared features from resolved nodes. Preserve external unknown
   fields where possible; Cargo format version 1 allows compatible additions.
   Apply strict validation to fields actually consumed by the adapter.
5. Retain the documented limits: `--no-deps` omits dependency resolution;
   `--filter-platform` filters the resolve view, while package definitions still
   carry other declarations. The graph is workspace-oriented, can include dev
   relationships and does not express separate host/target build-unit feature
   relationships fully. It does not inspect native DLLs, generated resources,
   dynamically loaded modules or installed payloads.
6. Reuse [generate-sbom.py](../../../../../scripts/generate-sbom.py) and
   [sbom.toml](../../../../../policy/sbom.toml) for admitted inventory work. Its
   current output is crate-level CycloneDX 1.5 JSON for Windows x64, with explicit
   features, frozen inputs, native/build caveats and an external output receipt.
   Do not replace its custody or claim it covers the whole product. Reconcile
   native, generated, asset and selected package payloads through package QA.

At inspected Cargo source, metadata resolution constructs target information and
can query the configured compiler; it is not synonymous with parsing TOML. It
does not run the project's build steps, but this is not a promise that arbitrary
Cargo configuration/toolchain wrappers are side-effect free. Do not opt into
nightly unit-graph/SBOM features merely to make inventory look more exact. The
Cargo unit-graph documentation explains the missing resolution detail; any such
tooling needs its own admitted purpose and exact toolchain. [Cargo sources](sources.md#cargo-and-build-records)

## Inspect files and packages

**PE resources.** Use absolute paths to immutable candidate copies. Collect file
hash/size, fixed numeric version/type/flags, every relevant string translation,
language/code-page pairs, icon/resource presence and application-manifest identity.
Use the checked bindings map in the canonical plan; do not write a bespoke PE
parser just to gather product metadata. Keep raw resource observation separate
from Windows' localized version view. `GetFileVersionInfoSizeExW` and
`GetFileVersionInfoExW` must use matching flags and an adequate bounded buffer;
an undersized buffer can truncate. Both `FILE_VER_GET_LOCALISED` and
`FILE_VER_GET_NEUTRAL` can involve a companion MUI file. Record flags, locale and
contributing file identities; do not label the result as raw single-file content.
Use an admitted resource-only reader for that separate question. [PE sources](sources.md#windows-readers-and-resource-tools)

**MSIX.** Use a read-only `IStream` and `IAppxFactory::CreatePackageReader`;
enumerate manifest identity/properties/applications/resources and actual payloads.
Retain the original manifest bytes and its namespace/schema context. Distinguish
main, bundle, dependency and language/resource packages. Bound archive entries,
decompressed bytes and output paths if extracting. A reader validates package
structure/footprint consistency, but does **not** validate its digital signature.
Keep signature/catalog/project-identity checks with package QA.

For localized values, retain `ms-resource:` references and inspect the associated
PRI/resource packs. The admitted SDK can dump a PRI with:

```powershell
MakePri.exe dump /if "C:\owned-candidate\resources.pri" /of "C:\owned-evidence\resources.pri.xml" /dt Detailed
```

Use real admitted absolute tool/input paths and a fresh output; do not execute
this illustrative command on an unapproved Windows host. Schema-free resource
packs can require the matching main PRI via `/es`; a missing external schema is
not proof of corrupt product data. A PRI dump inventories candidates/qualifiers;
actual language fallback and Windows display still require scoped native checks.

**MSI, if relevant.** Open the exact MSI with `MsiOpenDatabaseW` and
`MSIDBOPEN_READONLY`. Query the Property table and relevant Icon/File/Component/
Shortcut/ServiceInstall records separately from Summary Information. Set
`MsiGetSummaryInformationW`'s update count to zero and read typed properties with
`MsiSummaryInfoGetPropertyW`; close database/view/record/summary handles. Do not
commit, persist, run custom actions or install to inspect. Distinguish a base MSI
from transforms/patches and any composed view; retain every applied input's hash.
Do not interpret an MST/MSP summary as an MSI summary. [MSI sources](sources.md#windows-readers-and-resource-tools)

## Collect installed and Store facts

Run installed queries only at an admitted existing native gate and for the named
candidate. For MSIX, distinguish the querying process's `Package::Current` from
the package being inspected; handle an unpackaged reader. For MSI use actual
ProductCode, user SID and installation context with `MsiGetProductInfoExW`.
Machine context requires a null user SID; another user's context has access
restrictions. Do not elevate a collector merely to make a missing value disappear.
Retain access-denied, absent, unsupported and successful empty-value outcomes.

For service labels and event resources, preserve both stored references and
resolved values under each admitted locale. SCM MUI `@<PE-path>,-<stringID>` and
MSIX `ms-resource:` references use different resolution paths. The absence of
direct DisplayName/Description attributes on `desktop6:Service` does not prove
that packaged labels are impossible or setter-only: trace the selected authoring
pipeline through actual installed results. Registration readback alone cannot
prove event rendering, service purpose, management authority or servicing safety.
Keep service configuration and runtime status in distinct typed observations;
do not persist a transient PID/state as static product metadata. W11-W05 owns
reached service checks and W11-W08 owns diagnostics readback.

Avoid `Win32_Product`, including WMI/CIM/PowerShell wrappers: queries can cause
Windows Installer consistency checks and repairs. An apparent inventory command
can therefore change unrelated installations. If registry observation is needed,
record hive, architecture view and user scope; never mistake registry display
strings for a complete or authenticated inventory. [Installed-query sources](sources.md#windows-readers-and-resource-tools)

Use already authorized Store identity records and locale-specific listing CSV
exports. Import them as dated, bounded observations, then review differences
against owned source. Retain locale identifiers and CSV structure. Listing
languages do not establish UI translations. Do not infer missing Partner Center
IDs from publisher names, repurpose a certificate subject as a friendly label,
or automate login/submission because a harvest step needs evidence. Record a
pending owner input if no authoritative export is available.

## Bind and admit observations

Capture artifact SHA-256, logical component, package route, stage, reader/version,
timestamp in UTC, command/flags, platform/locale/user context as applicable,
field path, typed raw value, normalization and result/error. Bind external
documents to exact URL/revision and retrieval date; keep a retained bounded
excerpt or hash where the existing evidence policy calls for it. A hash identifies
bytes; it does not authenticate an untrusted source or prove field truth.

Keep byte/entry/string/depth/time limits at every collector boundary. Do not
follow arbitrary URLs, XML external entities, reparse points or package paths
outside owned inputs/outputs. Handle invalid encoding, duplicate/conflicting
fields and truncated reads explicitly. Preserve the original evidence rather
than replacing malformed text with a successful-looking normalized value.
Export only approved public fields; machine paths, account identifiers and raw
private logs remain in appropriately restricted evidence. Then use
[record and storage](record-and-storage.md) to decide whether any observation
may update an authoritative authored value.
