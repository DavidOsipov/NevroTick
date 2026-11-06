# Exact CBMC property identity source candidate

**Status: source candidate; not built; not adopted.** This separate driver patch
supports the existing [S1.1c proof task](../../../../tasks/s1.1-time-domain.md).
It does not qualify singleton refinement, the public constructor theorem or a
new verifier execution profile. The [parent source kit](../README.md) owns the
separate padded-union compiler repair and its historical evidence.

## Contract and scope

[The patch](preserve-raw-property-ids.patch) adds `PropertyId.raw_id: String`.
Both successful deserialization returns retain the exact decoded CBMC `id_str`
before parsing or demangling changes its presentation. Existing function, class,
counter and `Property::property_name()` behavior remain unchanged, including the
special recursion and classless fallback cases. The retained value is the decoded
string, not its original JSON escape spelling.

Each exported check gains `raw_property_id`. The existing numeric `id` remains
the enumeration index, and existing status, category, description and location
fields retain their meanings. For example, a classless guard can have raw ID
`_RNvCsjGD3dMRtdCH_8controls13selected_call.1` while its existing presentation
reports function `controls::selected_call` and category `missing_definition`.
The raw ID must not be reconstructed from that presentation. Likewise,
`assertion.01` and `assertion.1` retain distinct raw identities despite their
equal parsed counters.

The five-file patch changes the parser, JSON exporter and their tests. SARIF and
concrete-playback edits only supply the new field in existing test literals.
No manifest, lockfile, dependency, canonical qualifier or contract registry is
changed. The separate refinement owner changes other driver files.

## Compatibility

Kani's JSON metadata version remains `1.0`; this patch makes no silent version
change. The pinned JSON template and `scripts/validate_json_export.py` validate
declared fields and permit additive fields. Four pure Python fixture controls
confirmed that the legacy check shape and the additive shape are accepted, a
missing legacy `id` is rejected, and NevroTick's unchanged canonical qualifier
accepts an otherwise healthy fixture with the additive field. These are source
and consumer compatibility checks, not observations of the Rust exporter.

Strict external readers are unverified. Old exports lack `raw_property_id`.
Any later guard-accounting consumer must explicitly require a nonempty, unique
raw identity and match it to the actual final model; accepting an old export or
counting properties cannot establish guard preservation. This patch preserves
identities for checks that reach export. It does not establish that all required
guards survived the compiler, instrumentation, filtering and export pipeline.

## Source and remaining validation

[Provenance](provenance.json) binds the exact five before/after source files,
patch, immutable upstream revision, compatibility authorities and authored tests.
The original consumed source tree remains untouched. Apply this patch from the
root of a separately owned copy whose five files match the recorded before
hashes. This patch is independent of the separate driver-refinement patch.

New Rust tests cover the actual retained mangled fallback ID, distinct raw IDs
with equal parsed counters, and parser-to-export retention for fallback, normal
and recursion cases. Existing parser tests gain raw-identity assertions, and the
existing exporter test retains its numeric IDs alongside the new strings.

No Rustfmt, Cargo, compiler, Rust test, driver build, solver or installation
command has run for this candidate. Formatting, compilation, the authored Rust
tests, exact candidate admission and actual final-model-to-export controls remain
required before use. No product proof follows from this source preparation.

## Licence

The modified Kani source retains its `Apache-2.0 OR MIT` notices. The parent kit
contains [Apache-2.0](../LICENSE-APACHE) and [MIT](../LICENSE-MIT) licence copies.
