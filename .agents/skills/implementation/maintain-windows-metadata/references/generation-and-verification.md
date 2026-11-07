# Generation and verification

Use this reference after the authoritative input records are known. The
[canonical metadata plan](../../../../../docs/plans/windows-package-metadata.md)
owns the full PE/MSIX/MSI/Store field map and numeric/version/length limits.
Reuse [package-artifact QA](../../../testing/qa-package-artifacts/SKILL.md) for
final custody, signing, native behavior and complete-product evidence.

## Contents

- [Choose the smallest authoring path](#choose-the-smallest-authoring-path)
- [Generate once from explicit inputs](#generate-once-from-explicit-inputs)
- [Maintain the portable pipeline](#maintain-the-portable-pipeline)
- [Read the source-delivery report](#read-the-source-delivery-report)
- [Check delivery and build invalidation](#check-delivery-and-build-invalidation)
- [Map each output deliberately](#map-each-output-deliberately)
- [Keep public build information deliberate](#keep-public-build-information-deliberate)
- [Read back independently](#read-back-independently)
- [Challenge meaningful failures](#challenge-meaningful-failures)
- [Hand off at the existing gate](#hand-off-at-the-existing-gate)

## Choose the smallest authoring path

Prefer reviewed declarative resource/package inputs and the admitted Windows
build toolchain. Generate `.rc` data, compile it to `.res` with the admitted
resource compiler, and link it into each intended PE. Generate the selected
MSIX manifest/localization inputs and use the existing package-authoring path.
If MSI is later selected, map the same shared semantic fields to that admitted
authoring tool's Product/Property/Summary/upgrade constructs. Do not hand-write
an installer database generator or put metadata collection in the clock broker.

Keep `windows-rs`/`windows-sys` as scoped API projections, particularly for
independent readback. They do not supply the authoritative product descriptions,
version policy, RC compilation or Store listing management. The existing
`windows-metadata` and `windows-version` crates address API metadata and OS
version respectively, as the canonical plan documents.

| Candidate | Useful role | Admission/review obligation |
| --- | --- | --- |
| Admitted SDK RC/linker plus explicit `.rc` inputs | Direct authoring of version/icon resources with minimal hidden defaults | Qualify Unicode, translations, component routing, resource IDs and host-to-target toolchain behavior |
| `embed-resource` | Build-script compilation/linking of an explicitly authored RC file | Admit exact source/license/features/tools; require intended resources, route each binary, handle errors and all consumed-file rerun rules |
| `winresource` | Convenience construction of Windows resources from Rust/Cargo fields | Override/verify every meaningful default; check string/fixed versions, file type, locales and per-component descriptions |

These are researched options, not newly selected dependencies. Prefer an existing
qualified path. At the inspected `winresource` commit, its documentation table
describes `FileDescription` as coming from the package description, but the
constructor actually initializes it from `CARGO_PKG_NAME`. This source discrepancy
is a reason to set values explicitly and inspect emitted bytes, not a reproduced
NevroTick defect. `embed-resource` examples also include administrator-elevation
manifests; copy only the compilation mechanism, never unrelated execution policy.
[Pinned helper sources](sources.md#resource-helper-source-review)

## Generate once from explicit inputs

Use the selected [metadata/ CLI](../../../../../metadata/README.md) and
[stage contract](../../../../../docs/plans/windows-package-metadata.md#13-metadata-pipeline-implementation--7-november-2026)
for the implemented source-tool slice. The current POSIX build-host adapter
requires actual no-follow directory-descriptor support; isolated Python imports
are not a dependency sandbox. Generate a fresh externally owned tree, then use
read-only `check` before any dependent consumer. Source mode admits current
prototype facts and reports pending owners; release mode remains closed until
its code-owned schema and real release prerequisites are resolved. Do not copy
pending values into executable package/service/event authoring or edit generated
files. Actual compile/link/installed/signature readback remains MD-P4–MD-P5.


1. Freeze the reviewed record, version policy, component/locale selection,
   generator/templates, asset source and build configuration. Retain input hashes
   and actual tool versions. Keep unsupported or pending required values as
   failures at their required gate; never synthesize production identities.
2. Validate the owned data model, then derive typed values with checked
   transformations. Keep semantic versions, Windows numeric tuples and display
   versions distinct. Check representable bounds before conversion; preserve
   prerelease/channel meaning through the reviewed mapping. A fourth MSI field
   is not an extra upgrade-order slot.
3. Generate target-specific outputs in owned output directories. Use appropriate
   XML/RC/CSV encoders, quoting and Unicode rules. Reject embedded NULs and
   invalid code units; apply length limits in the unit the destination defines,
   not blindly in UTF-8 bytes. Preserve long/non-ASCII text and deliberate
   line breaks where permitted. Do not concatenate untrusted strings into XML,
   shell commands or installer expressions.
4. Keep generated files generated. Put build-script output under `OUT_DIR`;
   avoid editing source files during a build. Track every consumed file and
   approved environment input with Cargo rerun rules. Do not scan the network,
   git history or developer identity opportunistically inside `build.rs`.
   Pass frozen release facts through the existing build orchestration.
5. Check the target, not merely the build host. Build scripts run for the host;
   use Cargo's target configuration for Windows resource work. Review any
   `cfg(windows)` or target-specific build-dependency pattern that could omit
   resources on an admitted Linux-to-Windows path. Handle every admitted
   component/binary, not only whichever Cargo target happened to build first.
6. Keep deterministic content separate from observation time. Stable ordering,
   encoding and declared release inputs avoid unnecessary diffs. Preserve the
   existing `SOURCE_DATE_EPOCH` policy when applicable; do not promise complete
   binary reproducibility from deterministic metadata alone.
7. Compile/link/package, then read back the actual result. Finalize metadata
   before signing; any content edit after signing creates a new candidate that
   needs the applicable signing/custody checks again. Bind final, submitted and
   Store-delivered hashes separately.

Cargo build scripts, environment variables and resource compilation contracts
are linked in [sources](sources.md). Do not broaden product features or introduce
another runtime validation framework to implement this build-time pipeline.

## Maintain the portable pipeline

Use the [source CLI and test scope](../../../../../metadata/README.md), not a
second generator. The current implementation owners are:

| Owner | Guarantee and affected regression |
| --- | --- |
| [contract.py](../../../../../metadata/pipeline/contract.py), `load_metadata()` / `validate_model()` | Raw admission and immutable model invariants, including strict ORCID/ISNI spelling and checksums; [identifier controls](../../../../../metadata/tests/test_identifiers.py) also challenge direct model/generation bypasses |
| [fields.py](../../../../../metadata/pipeline/fields.py) and [text_policy.py](../../../../../metadata/pipeline/text_policy.py) | Exact registry/alias ownership and pinned raw/decoded text policy; field-coverage and text-policy tests retain their separate scopes |
| [delivery.py](../../../../../metadata/pipeline/delivery.py), `delivery_contract()` / `verify_delivery()` | Describe source expectations and independently interpret the exact payload set; [delivery controls](../../../../../metadata/tests/test_delivery.py) include a faulty real CLI producer and healthy source propagation |
| [generate.py](../../../../../metadata/pipeline/generate.py), `build_outputs()` / `generate()` / `check()` | Enforce semantic verification before returning/publishing payloads and again on actual held-file bytes during `check`; retain bounded no-follow custody and complete fresh-tree publication |
| `generate.py`, `CODE_FILES` / receipt generation | Bind consumed input bytes, execution modules and payload hashes; [pipeline controls](../../../../../metadata/tests/test_pipeline.py) cover receipt completeness, stale code and filesystem failure paths |

`verify_delivery()` accepts the payload dictionary without `receipt.json`; it
rechecks the model, exact plain path/string/bytes types, file set and budgets.
Keep its semantic expectations independent of production encoders. When changing
a mapping or supported literal grammar, retain valid hostile-looking text beside
wrong-value, wrong-owner, duplicate and unclaimed-content refusals. These readers
understand only the generated JSON/XML/CSV/Rust/RC subset; widening them is not a
substitute for compilation or an independent native resource reader.

When adding an executed pipeline module, update `CODE_FILES`, the independently
specified receipt expectations and isolated copied-tool fixtures together.
Retain healthy generation before the stale-code rejection; an omitted import is
a fixture failure, not evidence that stale output was refused. Use the affected
existing tests through [the configured metadata test entry](../../../../../metadata/tests/README.md)
and the existing W11-W10 checkpoint. A routine value edit needs no new collector,
fresh catalogue survey or repetition of unchanged native/tool qualification.

## Read the source-delivery report

`field-coverage.json.delivery` describes required source expectations, not a
standalone successful observation. Bind any claimed source pass to the actual
CLI outcome, candidate hashes and inspected output tree.

| Member | Interpretation |
| --- | --- |
| `scope` / `status` | `generated-source` / `required-source-contract`; neither is a compiled/native pass |
| `binding_scope` / `obligations` | Selected fact bindings plus aggregate record rules; not every catalogue row or a count of unique shipped facts |
| `artifact_contracts` | Complete current payload observation modes, including exact typed internal records; the receipt is separate |
| `locator_convention` / `representation` | Semantic members keyed by admitted component ID/service role and explicit transformations; these locators are not RFC 6901 JSON Pointers |
| `public_artifacts` / `internal_only_artifacts` | Source-projection exposure boundaries; neither is an approved package payload manifest |
| Compiled/native/package readback members | Preserve their actual `not-run` state until their existing owners execute those stages |

Keep field dispositions, selected delivery bindings and complete artifact checks
as separate denominators. Derive current counts from the current report; do not
hard-code them as a skill gate or turn them into release-completion percentages.
`receipt.json` hashes payloads and consumed source/code; it excludes its own hash
and is neither a signature nor final binary/package evidence.

## Check delivery and build invalidation

Apply [the adopted controls](../../../../../docs/plans/windows-package-metadata.md#135-adopted-metadata-delivery-controls--7-november-2026)
within the existing MD-P1–MD-P5 stages. The source tool's
`field-coverage.json` carries a versioned `delivery` contract for the
reached generated-source bindings. Its required source-field/component/artifact/
locator tuples are separate from catalogue field dispositions. Compare parsed
JSON/XML/CSV and supported Rust/RC source literals against admitted model facts,
without calling the production encoders as the semantic oracle. Keep the exact
byte/tree/source-identity check alongside this comparison. Missing, extra,
duplicated or wrong-owner values must fail despite equal counts; raw literal
text stays data. This observer qualifies generated source, not compiled PE or
installed resources.

Before MD-P4 consumer adoption, select the actual required bindings for that
component and locale. Freeze a cold-build result, change one consumed metadata,
Cargo or generator input without an unrelated source edit, then compare warm
and clean rebuilds using independently decoded metadata. Include consumed-file
addition/removal where relevant. Verify Cargo rerun declarations and approved
environment/configuration inputs; input hashes alone cannot prove that the
build relinked the resource. A selected-tool/configuration change must also
invalidate previous producer admission and observations.

Bind downstream observations to the actual selected resource compiler, linker,
SDK, target, profile and configuration identities using the existing build
receipt owner. Never infer execution/custody from a version string, describe
ambient tools as selected, or stamp old artifacts with a new generator identity.
One build consumes one frozen generation; retain source, compiled, packaged,
signed and Store-delivered identities separately. Do not add a mutable cache,
database or public attestation requirement for this.

At package/export adoption, inspect the exact approved final payload, including
unexpected files. Internal field/coverage registers and `authoring/` records
are not automatically public or shippable. Pair a healthy payload with deliberate
whole-directory copying/private-sentinel leakage. A source public allowlist
cannot prove that a later packaging step excluded those files. Distinguish
intentional omissions from lost required fields using the destination contract.

The source receipt/checker is not an independent final artifact reader. At the
existing native/package gate challenge mutually consistent declarations that
disagree with emitted bytes, same-count wrong component/version/locale, missing
resources and stale generations. Missing or empty required output fails; a
warning, producer-generated golden update or same-helper parity alone cannot
establish the promised semantics. Keep native/Store gates pending until actually
executed under their existing holds.

## Map each output deliberately

| Output | Required mapping/reconciliation |
| --- | --- |
| Every first-party EXE/DLL | Product/publisher fields plus actual component filename, internal name, role description, fixed/string versions, file type/flags, translations and icon where applicable; headless components still need descriptive metadata |
| MSIX identity | Exact assigned Name/Publisher and reviewed numeric version/architecture/resource identity; distinguish friendly labels and derived family/full names |
| MSIX display/resources | Package and application fields, correct assets, resolvable `ms-resource:` keys, PRI/resource packs and actual supported languages/fallback |
| Platform applicability | Current OD-14 family/edition/installation scope, actual package minimum/tested values and separately enforced CPU prerequisites; neither x64 nor MaxVersionTested proves complete support or an upper support bound |
| Conditional MSI | Property table and Summary Information; correct manufacturer/product/upgrade/package identities, version/language, ARP fields, icons and actual install paths; preserve Installer-owned derived registration |
| About/diagnostics | Read the same build-generated public projection; distinguish product release version, component/file version and installed package identity; keep an unpackaged development state explicit |
| Store listings | Reviewed per-locale description/features/keywords/support/privacy/license information and actual screenshots; retain limits, CSV schema, release scope and separate UI-language claims |
| Software composition | Existing CycloneDX inventory linked to actual native/generated/resource payloads and final hashes; retain original third-party publisher/rights information |
| Installed services/shortcuts | Compare actual registration/display with intended per-role values under existing native owners; metadata work does not authorize service changes |
| SCM labels | Separate stable technical name, display name and purpose description; preserve raw MUI references, resolved locale and installed servicing behavior under W11-W05 |
| Event catalogue | Reconcile stable provider/event identities, schema versions, message/help resources and actual rendered payload under W11-W08; selected channels and privacy rules remain authoritative |
| Conditional signing descriptions | Map supported content description/URL on the admitted signer route; inspect final signed/delivered bytes separately from PE description or certificate identity |

Read [version/localization rules](../../../../../docs/plans/windows-package-metadata.md#8-version-and-localization-rules-that-prevent-silent-errors)
before implementing conversions. Do not apply a single permissive semantic-version
parser to all targets or guess an MSI GUID policy. Do not advertise planned
services, NTS, translations or future features as shipped capabilities.

Avoid populating behavior fields just to increase coverage. Capabilities,
extensions, execution level, virtualization, install scope, repair/removal flags
and platform requirements follow their existing architectural owners. A supported
schema field can still be inappropriate for this product.

For ordinary user-mode service EXEs use application file classification, not a
driver subtype. Keep generated resource-layout fields and truthful build flags
under the admitted toolchain; never embed `VS_FF_INFOINFERRED`. Native assembly
description is conditional: the published schema includes it while the modern
overview omits it. Validate selected SDK/output support rather than assuming a
shell presentation. MSIX namespace choice substitutions and inherited attributes
need the complete selected schema set; never emit abstract placeholder elements
or enable preview behavior to increase coverage. See the relevant artifact
appendix in the [bounded catalogue](../../../../../docs/supporting-reviews/windows-service-metadata-catalogue.md).

## Keep public build information deliberate

An optional project-defined public projection can reuse frozen component/role,
release/source revision, measured dirty-state, toolchain/profile, actual selected
features, schema versions and target/CPU-floor facts. It is not a native SCM
property or a new provenance requirement. `x64` alone does not express the
required x86-64-v3 floor. Unknown source state remains unknown, never clean.

Keep a reviewed minimal export allowlist, with verified public links and separate
source, measured build and publication timestamps. Exclude host paths, usernames,
credentials, private contacts and raw runtime policy. PID, current service state,
generation and time quality remain bounded runtime observations. The final
signed/delivered artifact hash belongs in an external receipt; embedding a file's
own final hash creates circularity. Self-reported values are no independent
authentication. Do not add a runtime inventory service or scrape the environment
to implement this optional projection.

## Read back independently

Build an expectation from the reviewed semantic record and destination rules;
compare with an independent reader of the emitted file. Do not make the oracle
call the same formatter or use a generator's own JSON summary as proof that a
resource was linked. Name actual artifact identity and comparison scope.

For PE, compare fixed tuples, strings and all intended language/code-page pairs,
resource IDs/type, actual filenames and MUI context. For MSIX, compare manifest,
payload, resource references and actual resolved values. For MSI, compare both
Property and Summary Information, with correct installer container semantics.
Use [harvesting](harvesting-and-provenance.md) for reader APIs and side effects.

At the later admitted native gate, observe actual Explorer Details, installed-app
information, Start/shortcut entries, service descriptions and About/diagnostics
for the exact candidate and selected locales. Shell views expose different
subsets; a hidden optional field is not necessarily missing. Preserve raw data,
resource resolution and presented state as separate evidence. Schema/readback
success does not establish installation, update/recovery, signature trust or
Store certification. Keep the existing test-signed → submitted → delivered
qualification stages and current execution holds.

## Challenge meaningful failures

Choose cases that discriminate the changed guarantee, with a valid neighbor.
Do not add a large new test campaign for a prose-only edit. When implementing a
generator/collector, include applicable cases such as:

| Fault or edge case | Required independent outcome |
| --- | --- |
| Missing required field; unresolved Store identity; placeholder public URL | Current required stage fails with owner/action; source-only preparation can remain explicitly pending |
| Wrong publisher or one component's role/name copied into another | Comparison reports the affected surface/component; does not mutate the source to match |
| Version overflow, prerelease collision or MSI fourth-part-only change | Checked mapping rejects the ambiguity before authoring; valid boundary values remain accepted |
| Quotes, ampersands, non-ASCII, embedded NUL and long text | Correct target round-trip for allowed text; explicit bounded rejection for invalid data |
| Missing translation, wrong PRI qualifier, missing main schema or MUI contribution | Distinguish missing input, unresolved reference, valid fallback and actual display mismatch |
| Correct `.rc` or manifest input omitted from the final artifact | Readback fails despite successful source validation/build-step status |
| Read denied, truncated data, malformed resource, collector failure | Retain native/error context; no empty-string substitution or false match |
| Stale source digest, file replaced during inspection, wrong release stage | Evidence remains incomplete/stale and cannot qualify the new candidate |
| Valid package structure with unverified signature or wrong installed project | Metadata parsing cannot be promoted to a trust/identity pass |

Test schema/formatter logic portably when possible, using fixtures that do not
need a live installer. Native resource/OS presentation and trust claims need their
actual platform. No native test or extra dependency is implied by creating this
skill. Use admitted instrumentation and scopes when implementation begins.

## Hand off at the existing gate

Report the source/record version and hashes; populated/derived/pending/justified
absent fields by component, locale and stage; generated outputs; actual readback
results; signature/installed/Store evidence not yet executed; and the owner of
each next action. Keep review corrections in authored inputs and regenerate.

Source preparation can be complete while Store-assigned identity, release
numbering or native execution is pending. An illustrative TOML/JSON template is
not an implemented schema, production record, collector, generator or passing
artifact check. Rich metadata does not make optional enhanced build provenance
mandatory, activate MSI delivery or alter David's release authority.
