# Product metadata pipeline

Edit [product.toml](product.toml) to maintain product/component metadata in one
place. The pipeline derives current versions, repository and applicable licenses
from the referenced Cargo manifests. Generated files are projections, never
separately edited authorities.

The source now gives every field in the maintained catalogue its own explicit
disposition, alongside project facts and per-component PE/service values. It has
731 entries: the catalogue's 551 scoped rows (88 SCM, 267 artifact/distribution,
196 diagnostics) plus 180 project/component entries. These include compiler,
runtime and conditional fields; they authorize no Windows settings. Completeness
is bounded by the catalogue's declared structures, not every
Windows extension or every future SDK. The generated field coverage report keeps
that distinction visible.

The [canonical metadata contract](../docs/plans/windows-package-metadata.md)
and existing W11-W10/P6 task own adoption. W11-W05 retains service configuration
and W11-W08 diagnostics semantics. This source pipeline changes no product
runtime, service, account, privilege, package, signing or publication state.

## Layout

| Path | Purpose |
| --- | --- |
| [product.toml](product.toml) | Reviewed descriptive values, explicit references and pending owner actions |
| [pipeline/contract.py](pipeline/contract.py) | Bounded no-follow input reads, strict TOML admission and immutable typed model |
| [pipeline/fields.py](pipeline/fields.py) | Code-owned complete field identities, kinds, applicability, owners and derivations |
| [pipeline/text_policy.py](pipeline/text_policy.py) | Pinned strict UTF-8/Unicode admission, including raw comments and decoded escapes |
| [pipeline/generate.py](pipeline/generate.py) | Deterministic destination encoders, fresh-tree publication and read-only comparison |
| [pipeline/delivery.py](pipeline/delivery.py) | Source-derived delivery obligations and independent semantic interpretation of supported generated outputs |
| [pipeline/__main__.py](pipeline/__main__.py) | `validate`, `generate` and `check` command line |
| [tests](tests) | Independent valid/hostile source and actual CLI/output controls |

## Use

Run from the repository root with Python 3.11 or later on the qualified Linux/WSL
host. The custody implementation requires POSIX descriptor-relative operations
and `O_NOFOLLOW`; unsupported hosts fail closed. Windows Python execution has not
been qualified. No third-party Python package, Cargo resolution, dependency build
script or native resource compiler is executed by these commands.

```sh
python3 -I -B metadata/pipeline/__main__.py validate

metadata_output_parent="$(mktemp -d)"
python3 -I -B metadata/pipeline/__main__.py generate \
  --output "$metadata_output_parent/candidate"
python3 -I -B metadata/pipeline/__main__.py check \
  --output "$metadata_output_parent/candidate"

python3 -I -B -m unittest discover -s metadata/tests -v
```

The output is outside the source checkout, under an existing owned parent that
other users cannot write. `generate` requires a fresh destination. It renders to
a temporary sibling and publishes the complete tree by rename; an invalid input
or existing output never becomes an overwrite request. Keep prior generations
for comparison and generate a new destination after changing inputs. `check`
recomputes expectations and rejects stale, changed, missing, extra, linked or
unexpected files without modifying the inspected tree.

`--root PATH` selects an explicit source checkout; `--source PATH` selects a
different admitted source file within that checkout. Neither controls Python
imports, output names, templates, executable commands or network access.
Successful commands print a bounded JSON result. Rejections return nonzero and
omit raw values/private paths; inspect the reviewed pending entries or a valid
generated descriptor for owner actions.
The result includes `field_count` and `unresolved_field_count`; the latter counts
pending fields and derived observations with no value yet. It is not a count of
release blockers. The detailed coverage report separates these two kinds.

```sh
python3 -I -B metadata/pipeline/__main__.py validate --stage release
```

The checked-in source currently has unresolved release prerequisites. This
command must fail; removing pending descriptions is not permission to invent
Store identity, service technical names, provider GUIDs or release mappings.
Source validation/generation does not claim release readiness.

## Edit field values and dispositions

The main product/component sections own their existing facts. The `[fields]`
table is a flat register with stable, quoted identities; each inline record has
`value`, `status`, `source` and `reason`. Owner, type and applicability are fixed
by the separately maintained contract. All declared entries are required; deleting
one is an error, even if its current value is empty.

- `populated`: a reviewed literal value, with its source and reason.
- `derived`: leave the authored value empty. The pipeline derives known facts
  from the current product/component values; unobserved build/runtime facts stay
  empty instead of acquiring a fabricated observation.
- `pending`: leave the value empty and retain the unresolved question/source.
- `not-applicable`: leave the value empty and retain the concrete reason.

Unknown values use an actual empty string, never guessed identifiers or a word
such as `unknown`. The full code-owned Windows catalogue is a coverage/disposition
surface. Metadata work does not select a service account, privilege, startup
action, recovery policy, provider, counter set or optional integration. Actual
authoring of those operational fields still needs the existing owner and a
specific admitted contract.

Use semantic entries such as `project.urls.support` and
`component.ui.pe.LegalCopyright` for admitted editable facts. The two service
roles have separate technical-name/resource entries. A public author contact is
not automatically a support or regulatory contact. Exact copyright text and
release/platform measurements remain blank until established. Existing Cargo,
asset and locale sources retain their own authority; do not create parallel
versions in the record.
The current UI adopted-locale and fallback facts are both `en`; metadata cannot
claim an unimplemented UI translation. Ordinary non-English metadata text and
separately scoped future Store/listing locale candidates do not change that fact.
ORCID and ISNI values retain their exact admitted spelling and must have valid
check characters. This catches transcription errors; it does not establish
registry assignment or identity ownership. Empty pending identifiers stay empty.

The text policy applies to the complete raw product file, including comments,
and to decoded keys/values so TOML escapes cannot hide forbidden characters.
Use strict UTF-8 without BOM. Emoji/pictographs, hidden formatting controls,
noncharacters and the other explicitly excluded code points are rejected rather
than stripped. Ordinary international letters, punctuation and non-BMP letters
remain supported. See [the exact text policy](text-policy.md) for its pinned
Unicode version, ranges, exclusions and source provenance; UTF-8 alone is an
encoding rule, not a promise that every Unicode character is accepted.

## Generated contract

| Output | Consumer contract |
| --- | --- |
| `metadata.json` | Versioned resolved product/components, pending owner actions and exact consumed-input hashes |
| `field-register.json` | Complete internal field register with each disposition, value, source, owner, type, applicability and derivation |
| `field-coverage.json` | Declared-field/surface/status counts, unresolved owner actions and a versioned `delivery` contract/result for reached source bindings; no native or release qualification claim |
| `rust/product.rs` | Fixed typed constants for shared product/target facts |
| `rust/ui.rs`, `rust/observe.rs` | Fixed typed constants for the current fixture UI and observe-only CLI |
| `rc/ui.rc`, `rc/observe.rc` | UTF-8 VERSIONINFO source candidates for ordinary user-mode EXEs, with application type/subtype zero and explicit prerelease flags |
| `authoring/display.xml` | Escaped custom XML authoring projection, namespace `urn:nevrotick:metadata:authoring:1` |
| `authoring/msix-inputs.json` | Explicitly partial MSIX authoring data; no guessed package identity or runnable manifest |
| `authoring/service-inputs.json` | Planned engine/broker display/purpose data; no chosen SCM name, account or service behavior |
| `authoring/event-inputs.json` | Explicitly pending event-provider/resource inputs; no generated provider or event catalogue |
| `authoring/store.csv` | Custom review-field export with `field,value,status`; not a Partner Center upload schema |
| `authoring/project-inputs.json` | Internal project/rights/assets/locales/platform/build authoring data for explicit downstream consumers |
| `receipt.json` | Source/code identities and complete payload sizes/hashes; excludes its own hash to avoid circularity |

The fixed tree currently has 15 files. `field-register.json`,
`field-coverage.json` and all `authoring/` projections are internal authoring
evidence and may include contact, regulatory, operational or provenance details. They are not
automatically package payloads or public support exports. Public `metadata.json`
and shared Rust constants use an explicit descriptive-field allowlist; personal
email, regulatory contacts, raw policy and the complete field register do not
enter them. Final package/export owners must continue selecting exact payloads.

The source-delivery checker compares independently parsed JSON/XML/CSV and the
supported Rust/RC source literals with model-derived component/field/artifact
obligations before accepting generation or read-only `check`. It rejects missing,
extra, duplicate and wrong-owner bindings even when counts agree. Its report is
separate from field-disposition coverage: not every catalogue row is a promised
public output. Keep the exact-byte/custody checks alongside this semantic check.
Neither comparison establishes resource compilation or actual package contents.

The [adopted integration controls](../docs/plans/windows-package-metadata.md#135-adopted-metadata-delivery-controls--7-november-2026)
give the existing MD-P4–MD-P5 owners concrete next exits: independent compiled
resource readback, cold/warm/clean rebuild controls, actual selected tool/SDK/
target/profile identity, namespace references and exact final-package/private
authoring exclusions. These remain pending until real consumers and authorized
environments execute them. This pipeline imports no website tooling or data.

In the custom CSV, `derived` labels a generated projection of an admitted source
value; it does not replace that value's authored/Cargo authority or constitute a
production field-register disposition. Pending rows remain explicitly pending.
The RC encoder uses doubled embedded quotes and explicit string terminators as
shown in Microsoft's [resource-string](https://learn.microsoft.com/en-us/windows/win32/menurc/stringtable-resource)
and [VERSIONINFO](https://learn.microsoft.com/en-us/windows/win32/menurc/versioninfo-resource)
guidance; only an admitted resource compilation/readback establishes the native
interpretation for the selected compiler.

The RC candidates use the current component prototype version for both fixed
file/product tuples. A final product release-version mapping remains an owned
prerequisite. `040904B0`/translation `0x0409,1200` is the explicit English-US
Unicode resource candidate, not evidence of installed locale resolution.
Copyrights, trademarks and unknown identities are not fabricated to fill fields.
Known per-component copyright, trademark and comments values seed their specific
RC/Rust projections. A nonempty PrivateBuild/SpecialBuild text cannot select the
native build-classification flags; the current prototype profile refuses that
unadmitted request and retains its empty not-applicable defaults.
No executable/resource projection is emitted for planned engine service, broker
or helper components. Their source catalogue entries remain visibly planned.

For later admitted build integration, run this pipeline once in the existing
controlled build orchestration, freeze its inputs/output receipt, and supply
the required generated Rust file to the component's fixed `include!`/build-output
path. Compile/link each RC through the selected resource tool, then read back
actual emitted resources independently. Do not reparse editable product metadata
at runtime, discover generated code via an arbitrary environment override or
run a new Python process from each dependency build script. Actual product
consumers, resource compilation, complete MSIX/event authoring, native installed
readback and final signed/Store-delivered comparison retain their existing gates.

## Security boundary

All source TOML and referenced manifests are untrusted until strict bounded
admission. The model rejects unknown fields and inconsistent identities/types;
critical values are rejected rather than silently normalized. Rust, RC, XML and
CSV have separate encoders. Only code-owned output filenames and symbols exist.
The CSV review export rejects formula-leading text instead of changing its value.

Read-only source custody, bounded parsing, output isolation and deterministic
hashes address different failure modes. Python `-I`, a private output directory
and a recorded hash do not sandbox a malicious build script or authenticate a
dependency. The build owner must enforce immutable reviewed inputs, a protected
generator/checker, restricted writable outputs and separate signing authority.
Publication/checking assumes that the owned parent and frozen input/code snapshot
have no competing writer with the same execution identity. Enforcing that
prerequisite belongs to the admitted build sandbox. No new sandbox guarantee is
established by this portable source tool.

Keep secrets, host paths, usernames, dynamic PID/state, clock quality and runtime
policy out of static public metadata. No ambient Git state, current timestamp,
environment-variable scrape or network lookup supplies release facts here.
Missing measurements stay missing; a source/code digest is not an independent
signature, final artifact hash or proof of executing-source provenance.
