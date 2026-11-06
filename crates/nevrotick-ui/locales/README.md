# English catalog source

`en.json` is the complete **unapproved** English candidate for text contract 2,
catalog schema 2 and catalog revision 2. It has 509 active IDs. The protected
registry keeps 596 stable ordinals, including 87 retired or reserved meanings.
GUI build 2 is an informational code-owned fact; compatible build-only fixes do
not require a new translation file.

The source is strict UTF-8 JSON without a BOM. Runtime delivery uses independently
checked canonical CBOR. Both formats are untrusted input; source or binary format,
file origin, fingerprint and successful schema checks establish no approval.
`src/localization/approvals.rs` deliberately has an empty production review ledger.
The byte-exact positive authority in unit tests is `cfg(test)` only.

The running engineering UI uses maintained source-owned English until an approved
`CatalogStore` is available. A rejected external catalog cannot select that
engineering mode or replace the active catalog. See
[`DESIGN.md`](../src/localization/DESIGN.md) for the typed snapshot bridge,
[`contract-map.md`](../src/localization/contract-map.md) for evidence boundaries,
and [`ui-localization.md`](../../../docs/plans/ui-localization.md) for owner scope.
