# Primary sources

Reviewed 5 November 2026 (UTC); service/artifact refinements checked 7 November
2026; signing-configuration and PE-padding semantics checked 8 November 2026. This is an
original local workflow: no upstream implementation or third-party skill is
vendored. Research used Microsoft Learn,
GitHub, Context7, Firecrawl developer search/page extraction and web search/open.
Indexed snippets and generated extraction answers were discovery aids; current
official contracts and pinned implementation resolve material claims. Tool
availability and source inspection are not dependency/native qualification.

## Cargo and build records

| Primary source | What it supports |
| --- | --- |
| [cargo metadata](https://doc.rust-lang.org/cargo/commands/cargo-metadata.html) | JSON format/compatibility, opaque IDs, no-deps, filter-platform and locked/offline flags |
| [Cargo metadata implementation at 5a8f8937](https://github.com/rust-lang/cargo/blob/5a8f8937cb8c6da49714b3ad826f0a9a85d3fa95/src/ops/cargo_metadata.rs) | Entire-workspace resolution, target-information construction, dev-unit inclusion and graph traversal; main-branch source, not the project's admitted Cargo version |
| [Cargo unit graph](https://doc.rust-lang.org/cargo/reference/unstable.html#unit-graph) | Why metadata cannot completely represent feature relationships between different build-unit/dependency kinds; unstable tooling is not automatically admitted |
| [Package metadata](https://doc.rust-lang.org/cargo/reference/manifest.html#the-metadata-table), [workspace metadata](https://doc.rust-lang.org/cargo/reference/workspaces.html#the-metadata-table) | Custom tables are tool-defined; external tools choose fallback semantics |
| [Build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html), [environment variables](https://doc.rust-lang.org/cargo/reference/environment-variables.html) | Host execution, target context, OUT_DIR, rerun declarations and Cargo package values |

The source pin is a research anchor. Before implementation, inspect the actually
admitted Cargo version/configuration. Do not copy a main-branch-only option into
the project's command line or treat metadata collection as arbitrary-code-safe.

## Windows readers and resource tools

| Primary source | What it supports |
| --- | --- |
| [GetFileVersionInfoExW](https://learn.microsoft.com/windows/win32/api/winver/nf-winver-getfileversioninfoexw) | MUI involvement for localized and neutral modes, matching size-query flags, full paths and truncation |
| [VERSIONINFO](https://learn.microsoft.com/windows/win32/menurc/versioninfo-resource) | Fixed numeric fields, string/resource structure; complete field mapping remains in the canonical plan |
| [StringFileInfo](https://learn.microsoft.com/en-us/windows/win32/menurc/stringfileinfo-block) | OriginalFilename retains original naming; apply NevroTick's approved shipped-basename mapping |
| [Resource files](https://learn.microsoft.com/windows/win32/menurc/about-resource-files), [.res linker inputs](https://learn.microsoft.com/cpp/build/reference/dot-res-files-as-linker-input?view=msvc-170) | RC script → resource compilation → link into the executable |
| [CreatePackageReader](https://learn.microsoft.com/windows/win32/api/appxpackaging/nf-appxpackaging-iappxfactory-createpackagereader) | Read-only package reader validates structure/footprint; does not validate the digital signature |
| [MakePri options](https://learn.microsoft.com/windows/uwp/app-resources/makepri-exe-command-options) | PRI dumps, detailed output and external schema for schema-free resource packs |
| [Manifest/UI localization](https://learn.microsoft.com/windows/uwp/app-resources/localize-strings-ui-manifest) | Resource references and locale-dependent values |
| [MsiOpenDatabaseW](https://learn.microsoft.com/windows/win32/api/msiquery/nf-msiquery-msiopendatabasew) | Read-only database mode and handle lifetime |
| [MsiGetSummaryInformationW](https://learn.microsoft.com/windows/win32/api/msiquery/nf-msiquery-msigetsummaryinformationw), [Database.SummaryInformation](https://learn.microsoft.com/windows/win32/msi/database-summaryinformation) | Separate Summary Information and zero requested updates for a read-only inspection |
| [MsiGetProductInfoExW](https://learn.microsoft.com/windows/win32/api/msi/nf-msi-msigetproductinfoexw) | Installed instance/user/context distinctions, access requirements and buffer-length rules |
| [Windows Installer consistency-check side effects](https://learn.microsoft.com/troubleshoot/windows-server/admin-development/windows-installer-reconfigured-all-applications) | Win32_Product queries can initiate validation/repair of installed applications |
| [MSI ProductVersion](https://learn.microsoft.com/windows/win32/msi/productversion), [summary meanings](https://learn.microsoft.com/windows/win32/msi/summary-property-descriptions) | Different numeric version and summary-property semantics; avoid generic field-count assumptions |
| [Store identity](https://learn.microsoft.com/windows/apps/publish/view-app-identity-details), [listing import/export](https://learn.microsoft.com/windows/apps/publish/publish-your-app/import-and-export-store-listings?pivots=store-installer-msix) | Externally assigned technical identity and locale-specific CSV authoring surfaces |

The [canonical windows-rs map](../../../../../docs/plans/windows-package-metadata.md#3-where-windows-rs-helps)
retains release-associated `windows` 0.62.2 source at
`32c3144490c016fe496a0aed769bce60987a2e9d`, while the actual UI graph uses
`windows-sys` 0.61.2 at the inspected baseline. Its pinned modules cover PE,
Appx/MSIX, MSI and WinRT package APIs. A readback tool still needs its own exact
feature/target/lifetime review; do not add the typed crate to every product role.

## Resource-helper source review

| Pinned source | Observed mechanism and limit |
| --- | --- |
| [winresource implementation](https://github.com/benjaminri/winresource/blob/7ff4f7fd9b7e655a6805b7e1441d71d3e79b60fe/lib.rs) | WindowsResource constructor uses Cargo package version/name defaults; its FileDescription initialization differs from the nearby documentation table. Explicit setters/readback are necessary for reviewed product values |
| [embed-resource README](https://github.com/nabijaczleweli/rust-embed-resource/blob/0d183985c33096a658c3fd8ba19d7eec6cee1dba/README.md) | Compile/link an owned RC file and route resources to selected binaries; require intended resources and review rerun/error behavior. The elevation example is not a product-policy recommendation |

These alternatives were inspected as source; neither is selected or added to
NevroTick. A helper name or README example does not qualify exact emitted fields,
cross-compilation, Unicode, license compatibility or a native build. No framework
performance claim follows from this research.

## Bounded service and artifact catalogue

The [canonical metadata map](../../../../../docs/plans/windows-package-metadata.md)
owns applicability and links the [retained catalogue](../../../../../docs/supporting-reviews/windows-service-metadata-catalogue.md).
Its SCM, artifact and diagnostics appendices enumerate only their declared dated
structures/schema nodes. Read the affected appendix, not every schema. Counts
overlap across representations and do not measure implemented product coverage.

| Primary source | Refinement and limit |
| --- | --- |
| [VS_FIXEDFILEINFO](https://learn.microsoft.com/en-us/windows/win32/api/verrsrc/ns-verrsrc-vs_fixedfileinfo) | User-mode application versus driver classification, subtype and truthful flags; INFOINFERRED is not embedded resource content |
| [Application manifests](https://learn.microsoft.com/en-us/windows/win32/sbscs/application-manifests), [native manifest schema](https://learn.microsoft.com/en-us/windows/win32/sbscs/manifest-file-schema) | Overview/schema differ on optional description and identity details; selected SDK/output qualification remains necessary |
| [SignTool](https://learn.microsoft.com/en-us/windows/win32/seccrypto/signtool) | Sign-time content description/URL and verify-time display; no signing-route adoption or trust pass |
| [MSIX Packaging Tool conversion](https://learn.microsoft.com/en-us/windows/msix/packaging-tool/create-app-package) | Services report exposes editable display name/description; actual chosen pipeline and installed propagation remain unqualified |
| [Store privacy/support](https://learn.microsoft.com/en-us/windows/apps/publish/publish-your-app/msix/support-info) | Policy URL/text and separate regulatory contact context; no permission to supply private data or submit |
| [MSI settings identifier](https://learn.microsoft.com/en-us/windows/win32/msi/msiarpsettingsidentifier) | Conditional literal registry-location list, non-null if present; MSI remains a later owner-selected route |

The catalogue's Microsoft SDK snapshot is a research anchor, not the selected
build SDK. Refresh complete schema imports, inherited groups and choice
substitutions against the actually admitted toolchain; preview/internal fields
and mere schema presence establish neither product applicability nor OS/Store
support. No native checks were run for these source refinements.

## Conditional signing-configuration sources

Use [artifact signing restrictions](artifact-signing-restrictions.md) for the
bounded application of these sources. The [signing guide](../../../../../docs/windows-time-signing-guide.md)
already cites the SignPath reference as S5; this refinement supplies configuration
semantics, not another metadata catalogue or a provider-selection decision.

| Primary source | Refinement and limit |
| --- | --- |
| [SignPath reference](https://docs.signpath.io/artifact-configuration/reference) | Supported metadata restrictions, overrides and format-specific signature operations |
| [SignPath syntax](https://docs.signpath.io/artifact-configuration/syntax) | Path matching, cardinality, parameter declarations and required values |
| [SignPath examples](https://docs.signpath.io/artifact-configuration/examples) | Concrete restriction composition and nested-signing order; illustrative names/versions are not project defaults |
| [Artifact configuration overview](https://docs.signpath.io/artifact-configuration/) | Generated configurations need review, particularly for third-party payloads |
| [Microsoft package certificates](https://learn.microsoft.com/en-us/windows/msix/package/create-certificate-package-signing), [MSIX signing](https://learn.microsoft.com/en-us/windows/msix/package/sign-app-package-using-signtool) | Certificate/manifest publisher and block-map digest correspondence; use the admitted current SDK and existing Store qualification stages |
| [MSIX persistent identity](https://learn.microsoft.com/en-us/windows/msix/package/persistent-identity) | A separately reviewed publisher-bridging migration, not authority to change the current package identity |

These are dated documentation observations. Provider locale/version extraction,
unmatched-content handling and actual selected-service behavior require the
named controls before adoption; documentation inspection is not a signing run.

## PE certificate-padding qualification

The [signing guide §5.2](../../../../../docs/windows-time-signing-guide.md#52-pe-certificate-padding-and-exact-final-bytes)
owns the applicable Windows verification, negative case and complete final-byte
custody requirements. Its S21–S25 sources were checked on 8 November 2026:

| Primary source | Refinement and limit |
| --- | --- |
| [MSRC CVE-2013-3900](https://msrc.microsoft.com/update-guide/vulnerability/CVE-2013-3900), [advisory 2915720](https://learn.microsoft.com/en-us/security-updates/securityadvisories/2014/2915720) | Opt-in policy and current accepted setting representations; documentation does not establish a host's effective configuration |
| [Understanding executable file signing](https://learn.microsoft.com/en-us/windows/win32/secbp/understanding-pe-signatures), [WinVerifyTrust](https://learn.microsoft.com/en-us/windows/win32/api/wintrust/nf-wintrust-winverifytrust) | Partial PE digest coverage, strict-padding scope, catalog exception and exact-zero native success |
| [Get-AuthenticodeSignature](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.security/get-authenticodesignature), [SignTool](https://learn.microsoft.com/en-us/windows/win32/seccrypto/signtool) | Catalog preference versus explicit embedded verification; a generic valid result is not padding or project-identity evidence |

This is a dated source refinement, not an observed Windows rejection or a claim
that NevroTick has a demonstrated exploit. Preserve profile applicability and
existing execution holds; no custom cryptographic verifier is adopted.

## Standards and local owners

| Source | Use and limit |
| --- | --- |
| [SOURCE_DATE_EPOCH specification](https://reproducible-builds.org/specs/source-date-epoch/) | Stable source timestamp interchange; separate from actual collection time and full reproducible-build proof |
| [CycloneDX 1.5 JSON reference](https://cyclonedx.org/docs/1.5/json/) | Existing admitted inventory format, component identity and relationships; product authoring and artifact observations remain separate |
| [SBOM policy](../../../../../policy/sbom.toml), [wrapper](../../../../../scripts/generate-sbom.py) | Current frozen crate-inventory owner, target/features/custody and known completeness limits |
| [Metadata field contract](../../../../../docs/plans/windows-package-metadata.md), [package QA](../../../testing/qa-package-artifacts/SKILL.md) | Full field map and independent final artifact/native validation |
| [Release provenance policy](../../../../../docs/plans/release-provenance.md) | Enhanced public attestations/reproducibility remain optional under OD-05 §7.5 |
| [ORCID identifier structure](https://support.orcid.org/hc/en-us/articles/360006897674-Structure-of-the-ORCID-Identifier) | MOD 11-2 check character, numeric/`X` examples and the separate registry assignment authority; checksum consistency is not ownership authentication |

## Implemented source-tool owners

Use these current local owners to resolve implementation questions; do not
reconstruct behavior from a historical receipt or the sibling implementation.

| Local source | Scope |
| --- | --- |
| [Pipeline usage](../../../../../metadata/README.md), [test entry](../../../../../metadata/tests/README.md) | Supported POSIX source CLI, immutable fresh outputs, current configured regressions and later-stage limits |
| [Admission](../../../../../metadata/pipeline/contract.py), [registry](../../../../../metadata/pipeline/fields.py), [text policy and data provenance](../../../../../metadata/text-policy.md) | Exact source grammar, semantic IDs/aliases, checksums and pinned raw/decoded Unicode admission |
| [Independent delivery reader](../../../../../metadata/pipeline/delivery.py), [generation/custody](../../../../../metadata/pipeline/generate.py) | Source expectations, declared literal readers, mandatory observation, code/input receipts and publication/check ownership |
| [Delivery tests](../../../../../metadata/tests/test_delivery.py), [identifier tests](../../../../../metadata/tests/test_identifiers.py), [pipeline tests](../../../../../metadata/tests/test_pipeline.py) | Healthy/faulty real entry points, bypass controls and source/receipt/custody invariants; no native readback claim |

The [operations and report guide](generation-and-verification.md#maintain-the-portable-pipeline)
maps these owners to maintenance work. The canonical plan and W11-W10 task
retain acceptance and candidate-specific results; source files are not a new
release policy or proof of every product consumer.

## Adopted sibling-pipeline techniques

The [7 November adopted controls](../../../../../docs/plans/windows-package-metadata.md#135-adopted-metadata-delivery-controls--7-november-2026)
follow a source review of the dirty `personal-website-templated-1` working tree
at `6530d3a8d44cbe4f829b1e09edd3a1c90cffc087`. The review evidence lives under
`/tmp/nevrotick-metadata-comparison-20261107-2p4rmzhn`; HEAD alone does not
identify those uncommitted source bytes. Useful navigation locators within that
checkout are `src/schemas/person-data.ts`,
`scripts/lib/citation-release-requirements.ts`,
`scripts/lib/citation-release-output.ts`, `scripts/lib/image-representation.ts`,
`src/integrations/image-service-cache-identity.ts` and
`src/tests/citation-release-build-cache.test.ts`.

These supply design examples for identifier semantics, source-derived delivery
obligations, actual-byte readback, producer identity and warm-build invalidation.
Their tests were inspected rather than rerun for the comparison. NevroTick's
original Python implementation and independent controls retain their own
source/evidence owners; no sibling source is vendored or required at build time.
Do not copy its broader schemas, permissive normalization, warning-only
missing-output path, producer-generated golden expectations or cache database.
Its person/job-title, licence and support facts are not NevroTick authorities.

Engineering recommendations in this skill include TOML for reviewed intent,
JSON for observations, field-specific authority, semantic-diff promotion and
separate deterministic generation/readback. These are project-fit conclusions,
not requirements imposed by Microsoft or Cargo. Reuse existing equivalent
owners; keep the recommended storage filenames distinct from implemented files.
