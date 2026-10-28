# Scoped Cargo operations and evidence

Inspect the actual pinned tool versions/help and workspace before using commands.
These are recipes, not a claim that tools/configuration or a Cargo project exist.
Operate in the authorized owned candidate; retain the baseline manifest/lockfile
identities and never delete Cargo.lock to force fresh resolution.

## Discover and resolve

`cargo metadata --locked --format-version 1` describes resolved packages/targets;
retain only needed fields and do not print environment credentials.
`cargo tree --locked -e features` exposes feature activation paths; inspect
`cargo tree --locked -d` for multiple versions and use reverse inspection for
security-sensitive dependencies. Select the actual target and feature flags for
each lane; tree output is an approximation, not byte-level proof of shipped code.

Use primary release metadata to compare currently declared requirements, locked
versions and proposed exact versions. An available scanner/outdated helper may
aid discovery, but its installation or latest-version list is not required or
compatibility evidence. Check the MSRV of the resolved transitive graph and the
selected Windows SDK/linker/architecture lanes, not only the direct crate.

For an approved compatible update, a scoped operation such as
`cargo update -p PACKAGE --precise VERSION` is preferable to blanket update. Cargo
can change transitives needed to resolve that package; inspect the actual delta.
`--dry-run` can preview a resolution on a toolchain supporting it. Change a
manifest requirement only when the intended cohort requires it, then resolve and
review the resulting graph. Use exact reviewed revisions for git sources under
the architecture policy; do not substitute a branch head for a release checksum.

If local forks/vendor patches exist, reconcile root patch routing, lock entries,
upstream revision, retained diff/hash, license notices and the reason for each
patch. Map every retained/changed/dropped patch to a detecting regression and its
upstream removal condition. Workspace tests may exclude vendored packages; run
their required named regressions explicitly. An audit of a published base crate
does not attest to a changed local path dependency.

Features are additive and unified within the applicable resolver rules. A direct
`default-features = false` does not prove another path did not enable a feature.
Inspect actual features, source, checksum, patches, build.rs, proc macros and native
libraries. In a future virtual workspace, deliberately select the resolver;
Rust 2024 implies resolver 3 for a package but a virtual workspace still needs its
own workspace resolver setting. Choose edition/MSRV only during their authorized
workspace decision, not as an incidental update.

## Qualify the final graph

Use the repository's real CI/TOOLING commands with the selected Windows and portable
lanes. Once the workspace exists, representative checks are:

```sh
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
cargo build --locked --workspace
```

Apply the intended features/target/profile and project-required checks; do not
assume all-targets covers every OS/architecture or use all-features when backends
conflict. Run ordered checks without concurrent mutation of Cargo sources/lockfile
or shared evidence. Inspect generated code/build outputs where those affect the
claim. Package download/resolution is not compilation, compilation is not runtime
behavior, and a host build is not Windows MSI/service/clock qualification.

Use the admitted advisory/license/origin checker with recorded policy and database
revision; retain existing findings and assess the changed graph. Chrony-derived
code's GPLv2-only constraints and selected transitive license grants need concrete
review. A permissive UI example license does not license its framework. Avoid
unsupported universal claims about Rust TLS providers or Windows trust parity.

The final receipt identifies source/lock/toolchain/features/target, graph delta,
advisory and licensing results, commands/statuses and actual artifacts. Unsigned
build reproducibility and timestamped signing have different byte identities.
Do not declare the cohort release-qualified until its required native evidence
exists. [SOURCES](../../SOURCES.md) records primary research used in this workflow.
