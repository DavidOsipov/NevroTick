---
name: "maintain-windows-metadata"
description: "Harvest, reconcile, store, generate and independently verify NevroTick product, Cargo, SCM, PE VERSIONINFO, MSIX, event-catalogue, conditional MSI and Store metadata. Use for field inventories, missing Windows details, publisher/version/locale drift, packaging authoring, public build information and release metadata completeness."
---

# Maintain Windows metadata

Start with the [metadata contract](../../../../docs/plans/windows-package-metadata.md)
and [OD-05 §7.6](../../../../docs/nevrotick-owner-decisions.md#76-extensive-correct-product-and-package-metadata--5-november-2026).
Reuse [W11-W10](../../../../tasks/windows11-integration.md#w11-w10-re-enter-conditional-mechanisms-only-for-a-concrete-need)
and P6, with CHECK-18/20/23/26/27; W11-W05 owns service/lifecycle effects and
W11-W08 owns reached diagnostics, with ST-W07 for sensitive outputs.
Populate all useful, supported, applicable descriptive fields and record justified
absence. Do not infer new capabilities, identities, contacts or release readiness
from a request for more metadata.

Keep the selected complete MSIX route, current Core priority and execution holds.
MSI authoring is conditional on the existing owner decision. The owner-authorized
[top-level pipeline](../../../../metadata/README.md) now supplies the source-tool
implementation; use [the current staged plan](../../../../docs/plans/windows-package-metadata.md#13-metadata-pipeline-implementation--7-november-2026).
This workflow installs no collector, admits no dependency and lifts no native or
publication gate. Generated source is not installed or release qualification.
For platform fields, follow [OD-14's two-family matrix](../../../../tasks/windows11-integration.md#432-two-family-windows-qualification-matrix--5-november-2026).
Keep package applicability, OS/CPU admission and actual support/qualification
separate; a manifest version range is not the complete product support policy.

## Load only the needed reference

| Work | Read |
| --- | --- |
| Collect source, Cargo, file/package, installed or Store observations | [Harvesting and provenance](references/harvesting-and-provenance.md) |
| Choose authorities, handle conflicts, model fields or store evidence | [Record and storage](references/record-and-storage.md) |
| Change the portable pipeline or interpret its source-delivery report | [Implemented owners and report semantics](references/generation-and-verification.md#maintain-the-portable-pipeline), then the affected regression owner |
| Generate resources/manifests, map versions/locales or verify outputs | [Generation and verification](references/generation-and-verification.md) |
| Read PE/package metadata or consume artifact-derived application data | [Canonical content-trust rule](../../../../docs/windows-time-architecture.md#196-authenticated-artifact-content-and-consumer-trust) and [verification procedure](../../../../docs/windows-time-signing-guide.md#52-pe-certificate-padding-and-exact-final-bytes); signature coverage, inert inspection and approved final bytes apply independently of provider choice |
| Review signing-time metadata checks, file selectors or a conditional SignPath configuration | [Artifact signing restrictions](references/artifact-signing-restrictions.md); keep the selected delivery route and independent artifact QA |
| Inventory SCM, artifact or diagnostic schema members | Relevant appendix of the [bounded field catalogue](../../../../docs/supporting-reviews/windows-service-metadata-catalogue.md), through the canonical plan; read only the affected surface |
| Check an API claim, tool candidate or upstream revision | [Primary sources](references/sources.md) |
| Begin a field register or observation contract | [TOML field example](assets/field-register.example.toml), [JSON observation example](assets/observation.example.json); both are illustrative, incomplete inputs |

The canonical plan owns applicability and the detailed field map. Catalogue
completeness is limited to its named structures, schemas and dated sources;
custom keys and extension trees are open-ended. A field's presence is no reason
to enable its behavior. Keep research conditional; do not copy the catalogue
into every task or skill.

For an ordinary value edit, maintain the existing `product.toml` and use its
strict CLI with a fresh generated tree and read-only `check`. Reopen research
only for an uncertain changed rule. The example assets are design worksheets,
not accepted source files or successful observation records.

## Procedure

1. **Bound the request and stage.** Inspect current Git state, actual manifests,
   existing metadata sources, selected components, target/features, package
   route and task holds. Name whether the result is source inventory, reviewed
   authoring, generated output, emitted artifact, installed observation or
   Store-delivered evidence. A source-only task stops before build/native work.
2. **Build the field register.** For each applicable product/component/locale/
   surface, name its semantic field, authority, source locator, type, schema
   limits, value kind, applicability, transformation, owner, required stage,
   independent reader and disposition: populated,
   derived, pending with owner, or not applicable with reason. Identify display
   names, stable technical identities, behavioral settings, generated format data
   and runtime observations separately. Reuse existing
   license, release-version, brand and dependency owners.
3. **Harvest into observations.** Start with repository-owned sources; use
   scoped GitHub/documentation searches to establish contracts or provenance.
   Use admitted frozen Cargo metadata/SBOM tooling for dependency facts and
   read-only Windows/package interfaces for actual files. Bind every observation
   to exact input hashes, reader/version, stage, context and raw evidence.
   Never execute an inspected EXE, load its code, install/repair a package or
   query `Win32_Product` merely to read metadata.
4. **Reconcile before promotion.** Keep an imported value as a candidate until
   its field-specific authority and applicability are checked. Record conflicts;
   prefer neither the newest observation nor a majority of tools automatically.
   A dependency's author is not NevroTick's publisher. A known-good desired
   value must not be overwritten to hide a wrong emitted value. Preserve useful
   prior values and the reason for an approved correction.
5. **Maintain one authored record.** Edit the selected top-level
   [metadata/product.toml](../../../../metadata/product.toml), with component/locale
   extensions only through its code-owned schema and current owner.
   Reference Cargo/release/license authorities instead of creating competing
   versions. Keep machine observations in versioned JSON evidence beside the
   existing candidate receipt; keep the admitted CycloneDX inventory separate.
   Apply the owned schema and explicit public/export allowlist.
   The complete `[fields]` register must match the code-owned field set. Keep
   unanswered values as `value = ""` with their pending owner/source/reason;
   never substitute plausible identities or blank populated values. Use mandatory
   derivations for aliases. Runtime/compiler/reserved and unselected mechanisms
   have explicit non-authorable dispositions. Enforce the [UTF-8 text policy](../../../../metadata/text-policy.md)
   on raw source/comments and decoded keys/values, including TOML escapes.
6. **Generate supported projections.** Derive per-component RC resources,
   selected package authoring inputs, localization, About/diagnostics and Store
   listing exports from reviewed data and frozen build facts. Escape each target
   format correctly, enforce its own version/length/encoding rules and fail on
   missing required inputs. Finish resources before signing. Do not add package
   behaviors, broad Windows bindings or a runtime inspection service for this.
   Use the [adopted delivery controls](references/generation-and-verification.md#check-delivery-and-build-invalidation)
   to distinguish required source bindings from field dispositions and later
   compiled/native obligations. Check identifier semantics, not only spelling;
   an ORCID/ISNI checksum is not registration or identity authentication.
7. **Compare independent readback.** Use
   [package-artifact QA](../../testing/qa-package-artifacts/SKILL.md) and the
   [testing contract](../../testing/CONTRACT.md). Inspect actual PE and package
   bytes, resolve localized resources and reconcile complete payloads. Record
   mismatch, absence, access error and not-run distinctly. Native installed
   observations and final Store bytes remain their own existing gates; parser
   success and matching publisher strings do not prove signatures or trust.
   Check actual consumer invalidation, selected producer identity and final
   package/export exclusions at their existing MD-P4–MD-P5 gates.
   For a signing-configuration review, check the selected files and effective
   restrictions against independent release expectations; use the conditional
   reference above without treating provider acceptance as complete readback.
8. **Close the current scope honestly.** Report field coverage and missing
   required values by stage, changed sources/projections, exact checked inputs,
   observed differences, unexecuted checks and the next existing owner action.
   Update W11-W10/P6 rather than creating another package task. Do not submit,
   sign, publish or alter account data without the applicable existing authority.

## Review the common failure paths

- Keep `cargo metadata` discovery distinct from shipped build units and native
  payloads; `--filter-platform` does not turn every package entry into a shipment
  claim. Reuse the current SBOM wrapper and reconcile actual files.
- Inspect build-script target handling and resource-tool defaults. Host
  `cfg(windows)` can omit resources during a Linux-to-Windows build. Crate names
  and fixture descriptions are not automatically suitable product labels.
- Record PE MUI/locale context. `FILE_VER_GET_NEUTRAL` can still return strings
  from a MUI file; it does not establish a raw single-file extraction.
- Read MSIX manifest, PRI values and installed display separately.
  `CreatePackageReader` does not validate the package's digital signature.
- Read MSI Property and Summary Information separately, with read-only database
  access and zero requested summary updates. Preserve product/upgrade/package
  code semantics and actual installed-user/context distinctions.
- Never pad completeness with invented support endpoints, trademarks, Store
  IDs, translation claims, privileges or capabilities. Keep optional enhanced
  build attestations optional under OD-05 §7.5.

Use [dependency admission](../../environment/dependency-updates/SKILL.md) for
new collector/build crates or tools. This original local workflow imports no
upstream implementation and grants no upstream licensing clearance.
