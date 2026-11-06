# GUI text development

The GUI keeps three separate identities:

| Identity | Purpose | Change trigger |
| --- | --- | --- |
| GUI build | Code-owned GUI code/build identity | A new GUI build |
| Catalog schema | JSON/CBOR representation | A file-format change |
| Text contract and fingerprint | Required IDs, types, meanings and action associations | A semantic text-contract change |

Catalogs carry the schema, text contract and semantic fingerprint. A build-only
change can keep existing translations compatible. Admission rejects an
incompatible or incomplete catalog before publication and preserves the previous
complete catalog. Required text cannot be empty or whitespace-only.

Add a new control and its protected ID together in
`crates/nevrotick-ui/src/localization/contract-source.json`. Keep retired ordinals
and the separately retained `text-contract-v1-baseline.json`. Increment the text
contract for a semantic change against that baseline. The source checker follows
actual AppWindow component instances, inherited components, text/accessibility
bindings and registered typed Rust presenters. It rejects missing resources,
unused active resources, duplicate destinations and invalid fixed choice models.
An element inside literal `if false` (including grouped false and its complete
container/inheritance/@children subtree) cannot approve a resource. All parsing
bounds still apply to that dead source. Other conditions remain bounded lexical
may-flow; this check does not prove general runtime branch feasibility.

Run the source checks from the checkout:

```sh
python3 -X dev -Werror -B scripts/check-ui-text.py --root .
python3 -X dev -Werror -B -m unittest discover -s scripts/tests -p test_ui_text_coverage.py -v
python3 -X dev -Werror -B -m unittest discover -s scripts/tests -p test_ui_text_codegen.py -v
python3 -X dev -Werror -B -m unittest discover -s scripts/tests -p test_ui_text_native.py -v
```

After reviewing the protected metadata, generate a private source candidate with
its reviewed SHA-256:

```sh
python3 -B scripts/generate-ui-text.py --expected-contract-sha256 REVIEWED_SHA256 --output-parent /tmp
```

The tool validates the fixed metadata and pinned template before creating a new
private directory. It emits bounded identifier definitions, the immutable text
model projection, alias recipes and an output-slot manifest. Reference English
remains inert fingerprint data. Output names are fixed, and the tool writes no
baseline, locale CBOR or approval ledger.

The nine generated `*-copy.slint` files are rewiring recipes. Integrate their
slot expressions while preserving the maintained conditional English fixture
fallbacks. Replacing those maintained files wholesale would remove the fallback
used while the semantic approval ledger is empty. Review generated changes, then
use the configured formatter and `scripts/verify.py` checks. The configured rows
freeze the generator, template, fixtures and controls before Cargo execution.

Translator JSON and delivered CBOR pass the separate Rust catalog admission
pipeline. UTF-8, closed fields, typed arguments, canonical CBOR, resource/work
limits and the selected emoji-free character repertoire are mandatory. Catalog
text is rendered as literal data; code owns actions, paths, URI targets and
native buffer identities. Critical wording requires the independent human review
defined in [the localization plan](plans/ui-localization.md#meaning-fallback-and-actual-presentation-sinks).

The original tooling grant is recorded in
[UI-TEXT-LICENSE.md](../scripts/UI-TEXT-LICENSE.md).
