# Record and storage

Use this reference for the metadata model and maintenance rules. The selected
[metadata/ pipeline](../../../../../metadata/README.md) implements the bounded
source record and projections; later observation/build/native conventions still
need their own stage evidence. The [canonical plan](../../../../../docs/plans/windows-package-metadata.md#4-shared-maintained-record-and-provenance-of-values)
owns the product fields and current pending facts.

## Contents

- [Keep distinct records](#keep-distinct-records)
- [Choose authority per field](#choose-authority-per-field)
- [Define a field contract](#define-a-field-contract)
- [Keep observation identity and results](#keep-observation-identity-and-results)
- [Store and maintain the records](#store-and-maintain-the-records)

## Keep distinct records

| Record | Recommended representation | Owner and purpose |
| --- | --- | --- |
| Reviewed product/component intent | Selected [metadata/product.toml](../../../../../metadata/product.toml), with component/locale extensions only under its owned schema | Human-reviewed names, descriptions, rights references, public destinations and field dispositions |
| Frozen build/release inputs | Existing candidate receipt, extended with metadata-input and generator identities | Actual Cargo/release mapping, toolchain, target/features and input digests |
| Harvested observations | Versioned JSON records with raw evidence references | What a particular collector observed from a particular input/context |
| Generated projections | RC/XML/RESW/PRI/selected installer authoring, About data and listing exports | Deterministic products of reviewed intent and frozen facts; not separately edited authorities |
| Dependency/component inventory | Existing admitted CycloneDX 1.5 JSON and native/asset reconciliation | Software composition, license/provenance and exact payload relationships |
| Submission/delivery/installed evidence | Existing release/package evidence with separate stage identities | What was submitted, delivered, installed and presented |

A small reviewed source file fits this project's scale and Git workflow better
than a new database or metadata server. Split by component/locale only when it
improves ownership or avoids merge conflicts. Use standard TOML and JSON parsers;
do not invent another serialization or hide the primary record in a spreadsheet.
An existing well-owned equivalent source is preferable to duplicate authorities.

Cargo's `package.metadata` and `workspace.metadata` are tool-defined data; Cargo
does not validate their custom semantics or automatically implement fallback
between them. Either can host a deliberately specified source, but do not keep
the same product values both there and in a new TOML record. Reuse real Cargo
package fields where they already own the fact; define explicit lookup and
override rules for custom metadata. [Cargo sources](sources.md#cargo-and-build-records)

## Choose authority per field

| Value class | Authority | Conflict rule |
| --- | --- | --- |
| Product name, purpose, public publisher/developer text, support/privacy destinations | Accepted owner/project record and maintained public endpoint | Resolve against current accepted scope; a harvested string cannot silently amend it |
| Technical Store identity | Exact product's Partner Center assignment | Match required identity exactly; record unresolved account data as pending |
| Release/component versions | Existing Cargo/release policy and approved mapping | Detect mismatches; never strip prerelease text or truncate integers to force agreement |
| Copyright, license, attribution | Exact source/asset provenance and applicable reviewed rights records | Retain upstream rights; manifest claims are inputs to review, not legal clearance |
| Actual binary/package bytes, architecture and embedded fields | Frozen artifact and independent reader | Report discrepancies against intent; do not rewrite intent to match a defect |
| Installed presentation and registration | Actual candidate/context observation | Preserve difference from raw embedded data, locale fallback and user/package context |
| Dependency facts | Exact selected source/lock/build graph plus payload reconciliation | A repository's current default branch or a package name alone cannot identify shipped code |

Do not use a global "latest wins" merge. Different authorities answer different
questions. Require an explicit reviewable semantic diff before promoting imported
values; retain old value, new value, evidence, reason, owner and affected outputs.
An automatic collector can propose a patch; it cannot make owner, rights, release
or Store identity decisions by itself.

## Define a field contract

The selected [complete record](../../../../../metadata/product.toml) carries the
code-owned registry's exact key set. Its empty unanswered values are intentional:
retain pending source/reason and the registry owner; a blank is not populated.
The registry defines the contract/applicability, while the TOML remains the single
editable value source. Reject missing/unknown keys and conflicting copied aliases.
Use [the text policy](../../../../../metadata/text-policy.md) for strict UTF-8,
raw/comment and decoded-escape emoji/control rejection. It preserves ordinary
international text without silently stripping or normalizing it.


Edit the selected complete record for actual values. Use
[the TOML example](../assets/field-register.example.toml) only as a design worksheet
when defining a new field contract. Its illustrative `[[fields]]` array is not
the implemented input schema: real `product.toml` uses the code-owned `[fields]`
table and exact required key set. Do not replace that source with the worksheet,
copy its placeholder identities into it or introduce a competing value authority.

For each semantic field, record:

- Stable ID and scope: product or component, applicable package route, locale,
  destination field/resource key, and the existing acceptance/stage owner.
- Type and constraints: Unicode/encoding, target-defined length unit and limits,
  allowed formats/ranges, supported schema version and localization/fallback rule.
- Kind and applicability: authored description, stable technical identity,
  behavioral/security setting, generated format member or runtime observation;
  selected component/route/schema and the reason an optional entry is reached.
- Authority and provenance: source kind, repository path/section or external
  identity/export, source revision/digest, review owner/date and derivation ID.
- Disposition: `populated` with a reviewed value; `derived` with named inputs and
  transformation; `pending` with a responsible owner/next action; or
  `not-applicable` with a concrete reason. Do not collapse these into empty text.
- Required stage, independent reader and export policy: public artifact/listing,
  internal evidence or private source. Keep a dedicated allowlist for each output
  consumer; name how its actual representation will be observed.

Keep the semantic product value separate from surface-specific formatting.
For example, one product description may have a reviewed short variant for a
constrained field; do not truncate a long description blindly or confuse it with
each component's role description. Locale keys and component IDs form part of
identity. Define explicit fallback instead of mixing languages within one row.

Use exact namespace identities and field paths; XML prefixes are aliases. Keep
catalogue rows, aliases and alternative projections distinct from unique semantic
facts. Generated padding, lengths and reserved values need a format owner, not
invented friendly values. Preview/internal schema members remain conditional or
inapplicable; refresh selected SDK/compiler support before authoring them. A
reasoned disposition does not authorize a behavior, provider or integration.

Preserve stable service, package, application and provider identities over routine
rebuilds and translations. Generate a new identity only through its actual owner
and compatibility decision. Display names and descriptions never establish an
executable's identity, authorization or signature trust.

Validate identifier meaning at its admitted boundary: ORCID and ISNI need valid
check characters as well as the exact selected spelling. Preserve empty pending
fields and reject invalid values; do not silently trim, rewrite or truncate.
A checksum does not establish registry assignment or ownership. When technical
identities become populated, check references and collisions within each actual
SCM/package/application/provider/resource namespace using its comparison rules.
Record intentional sharing; a common display name cannot choose another role.
Do not require global uniqueness across unrelated namespaces or copy personal
website facts over NevroTick's accepted authorities.

Use a versioned build-time schema/validator for owned records. Reject unknown
owned keys, duplicate IDs, unresolved required values, invalid destinations,
out-of-range versions and unsupported enum values. Validate JSON/TOML before
generation, then each output's native schema. This is build/release tooling;
it does not add a second product runtime schema engine or replace the existing
Rust domain-construction contracts. An external input such as Cargo JSON needs
its documented forward-compatible adapter, not the same blanket unknown-key rule.

Evaluate completeness by named stage and surface. Every in-scope field needs a
disposition; every applicable field required at the current gate needs a valid
value or completed derivation. A reasoned optional omission is acceptable; a
required field marked not applicable incorrectly is still a failure. Report
pending counts and reasons, not an unqualified percentage that hides uninspected
components, locales or output formats. Source-preparation work may retain pending
Store data; it cannot claim package/submission completeness.

## Keep observation identity and results

Use [the JSON example](../assets/observation.example.json) as a starting contract,
then extend the existing candidate receipt rather than creating competing logs.
Its `example_only` marker and `not-run` outcome intentionally prevent it from
being mistaken for successful evidence. Real successful records require actual
input/output digests and an observed value, not placeholder hashes.
This proposed observation shape is also distinct from generated `metadata.json`,
`field-coverage.json` and `receipt.json`; the source CLI does not admit it as a
product input or verify native observations. Reuse canonical registry field IDs
such as `component.ui.pe.ProductName`; give a genuinely new observed field its
owned contract instead of silently changing spelling or creating an alias.

Bind the record to the exact artifact, component, stage and reader. Preserve
raw typed values or bounded raw-evidence references, separately recorded derived
display/normalized values, locale/qualifiers, expected-value source and outcome.
Distinguish match, mismatch, absent, unsupported, denied, malformed, truncated,
collector failure and not-run. Preserve native error codes and read lengths.
Do not turn access failure into an empty string or mark unobserved fields absent.

For localization, model raw key → resource candidate/qualifiers → resolved value
→ observed shell/UI value. For MSI, include database/package/transform context
and installed instance separately. For MSIX, include main/bundle/resource package
identity. For PE MUI, include relevant companion input hashes. Treat a file read
and its hash as one custody operation or inspect immutable copies and verify
stability; a hash taken earlier cannot exclude a later replacement.

## Store and maintain the records

Commit reviewed authored inputs, schemas, generator source, maintained templates,
locale resources and meaningful review decisions in the project repository.
Keep large raw extracts, SBOMs and built/signed packages in the existing owned
artifact/evidence locations, indexed by exact hashes and existing task/release
identity. Store only necessary evidence pointers and handoff status in the task.
Do not put one-off receipts or huge research dumps inside the skill pack.

Retain original raw observations; write a new record when recollecting or
changing normalization. Link predecessor/supersession instead of overwriting
failed or historic evidence. Define retention/access through existing release
custody policy. Keep exact public exports distinct from restricted raw evidence;
never expose home paths, usernames, certificate private material or unapproved
contacts just because a tool returned them.

Recompute derived outputs when their inputs change. Declare change detection
for every consumed file/environment input; compare frozen hashes before and
after collection/generation. Use deterministic ordering and an explicit encoding.
Keep the collection timestamp in the observation, separate from a stable release
date or `SOURCE_DATE_EPOCH`; do not inject the current wall-clock time into every
EXE. The latter timestamp helps deterministic tooling but does not prove a
reproducible build or satisfy optional enhanced-attestation work by itself.

Use CycloneDX for the admitted component inventory, with its own versioned schema,
identities, relationships and provenance. Preserve `bom-ref`/package relationships
when associating Windows files with it. Do not replace it with the product TOML,
silently upgrade its schema, or add guessed package identifiers to appear complete.
The existing SBOM wrapper's consumed-structure checks are narrower than full
schema validation and whole-product completeness. [Standards and local owners](sources.md#standards-and-local-owners)
