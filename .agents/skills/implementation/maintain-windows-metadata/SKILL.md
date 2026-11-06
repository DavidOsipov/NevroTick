---
name: "maintain-windows-metadata"
description: "Harvest, reconcile, store, generate and independently verify NevroTick product, Cargo, PE VERSIONINFO, MSIX, conditional MSI and Store metadata. Use for metadata inventories, missing Explorer details, publisher/version/locale drift, packaging authoring, About data and release metadata completeness."
---

# Maintain Windows metadata

Start with the [metadata contract](../../../../docs/plans/windows-package-metadata.md)
and [OD-05 §7.6](../../../../docs/nevrotick-owner-decisions.md#76-extensive-correct-product-and-package-metadata--5-november-2026).
Reuse [W11-W10](../../../../tasks/windows11-integration.md#w11-w10-re-enter-conditional-mechanisms-only-for-a-concrete-need)
and P6, with CHECK-18/20/23/26/27; W11-W05 owns service/lifecycle effects.
Populate all useful, supported, applicable descriptive fields and record justified
absence. Do not infer new capabilities, identities, contacts or release readiness
from a request for more metadata.

Keep the selected complete MSIX route, current Core priority and execution holds.
MSI authoring is conditional on the existing owner decision. This skill supplies
a method and templates; it installs no collector, admits no dependency, creates
no production product record and lifts no native or publication gate.
For platform fields, follow [OD-14's two-family matrix](../../../../tasks/windows11-integration.md#432-two-family-windows-qualification-matrix--5-november-2026).
Keep package applicability, OS/CPU admission and actual support/qualification
separate; a manifest version range is not the complete product support policy.

## Load only the needed reference

| Work | Read |
| --- | --- |
| Collect source, Cargo, file/package, installed or Store observations | [Harvesting and provenance](references/harvesting-and-provenance.md) |
| Choose authorities, handle conflicts, model fields or store evidence | [Record and storage](references/record-and-storage.md) |
| Generate resources/manifests, map versions/locales or verify outputs | [Generation and verification](references/generation-and-verification.md) |
| Check an API claim, tool candidate or upstream revision | [Primary sources](references/sources.md) |
| Begin a field register or observation contract | [TOML field example](assets/field-register.example.toml), [JSON observation example](assets/observation.example.json); both are illustrative, incomplete inputs |

The canonical plan owns the exhaustive Windows/Store field map. Keep detailed
research here conditional; do not copy that map into every task or skill.

## Procedure

1. **Bound the request and stage.** Inspect current Git state, actual manifests,
   existing metadata sources, selected components, target/features, package
   route and task holds. Name whether the result is source inventory, reviewed
   authoring, generated output, emitted artifact, installed observation or
   Store-delivered evidence. A source-only task stops before build/native work.
2. **Build the field register.** For each applicable product/component/locale/
   surface, name its semantic field, authority, source locator, type, schema
   limits, transformation, owner, required stage and disposition: populated,
   derived, pending with owner, or not applicable with reason. Identify display
   names, technical identities and trust claims separately. Reuse existing
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
5. **Maintain one authored record.** Prefer a small versioned TOML source such
   as the proposed `packaging/product.toml`, component records and locale files.
   Reference Cargo/release/license authorities instead of creating competing
   versions. Keep machine observations in versioned JSON evidence beside the
   existing candidate receipt; keep the admitted CycloneDX inventory separate.
   Apply the owned schema and explicit public/export allowlist.
6. **Generate supported projections.** Derive per-component RC resources,
   selected package authoring inputs, localization, About/diagnostics and Store
   listing exports from reviewed data and frozen build facts. Escape each target
   format correctly, enforce its own version/length/encoding rules and fail on
   missing required inputs. Finish resources before signing. Do not add package
   behaviors, broad Windows bindings or a runtime inspection service for this.
7. **Compare independent readback.** Use
   [package-artifact QA](../../testing/qa-package-artifacts/SKILL.md) and the
   [testing contract](../../testing/CONTRACT.md). Inspect actual PE and package
   bytes, resolve localized resources and reconcile complete payloads. Record
   mismatch, absence, access error and not-run distinctly. Native installed
   observations and final Store bytes remain their own existing gates; parser
   success and matching publisher strings do not prove signatures or trust.
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
