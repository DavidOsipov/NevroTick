# Source-pipeline test scope

Run from the repository root:

```sh
python3 -I -X dev -Werror -B -m unittest discover -s metadata/tests -v
```

These tests use the public Python CLI with disposable, inert source repositories.
They execute no Cargo, native Windows, package, signing, service, network or proof
jobs. The API tests challenge typed-model bypasses at the generation boundary.

The independent expected facts are fixed before inspecting generated output:

| Input or operation | Expected fact | Forbidden result |
| --- | --- | --- |
| Actual source record | UI and observe prototypes only; component versions and licences follow their Cargo owners; Windows target and v3 CPU floor stay explicit | Planned engine, broker or helper masquerades as an actual binary |
| Author identifiers | Exact admitted ORCID/ISNI spelling and valid check character, with numeric/`X` healthy examples | Format-only admission accepts a mistyped check character or claims identity ownership |
| Source versus release stage | Source admits explicit pending fields; release rejects mandatory unresolved fields | Source success claims release readiness |
| Literal text | Quotes, backslashes, ampersands and Unicode remain one literal value in each reached output representation | Injected extra fields, elements or executable text |
| Identical inputs | Exact output paths and bytes agree in two fresh output trees | Wall-clock, root-path or output-path dependence |
| Changed source or generator input | Previously generated output fails `check` | Stale receipt accepted |
| Missing, changed or unexpected output | `check` rejects without changing any output bytes | Silent correction or ignored extra file |
| Source-delivery obligation | Independently decoded component/field/artifact value and multiplicity match the admitted record | Consistently wrong generator output, wrong owner with the same count, duplicate binding or unexpected public field passes |
| Invalid input, unsafe source or unsafe output | Bounded rejection, no raw rejected value or foreign file leak, prior output remains intact | Partial generation or source mutation |

Observer controls deliberately perturb a promised value, omit one required file,
and add one unexpected file. A real CLI healthy case remains separate. JSON,
XML and CSV observations use independent standard-library parsers. The Rust/RC
checks inspect the generated source literals; they establish source projection
facts and do not claim resource compilation or native readback.

The versioned `field-coverage.json` delivery result describes reached source
bindings, separately from the complete field-disposition register. Semantic
observers do not call production encoders to derive their interpretation. The
exact-byte check remains necessary for stale/corrupt candidate custody.
Actual Cargo/resource warm-build invalidation, independent linked-resource
readback and final-package privacy tests belong to the existing MD-P4–MD-P5
integration owners; these source tests do not execute or qualify those gates.

The owners remain W11-W10/P6 with CHECK-18/20/23/26/27. W11-W05 and W11-W08 own
later service and event integration. Emitted binaries, complete MSIX, installed
state and Store-delivered bytes retain their existing independent gates.
