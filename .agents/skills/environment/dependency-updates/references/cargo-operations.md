# Scoped Cargo operations and evidence

Inspect actual pinned versions/help/workspace before commands. Recipes prove no
existing tools/configuration/Cargo project. Use authorized owned candidate; retain
baseline manifest/lockfile identities. Never delete Cargo.lock to force resolution.

## Discover and resolve

`cargo metadata --locked --format-version 1` describes resolved packages/targets;
retain needed fields only; print no credentials. `cargo tree --locked -e features`
exposes feature activation; inspect `cargo tree --locked -d` for multiple versions
and reverse graphs for security-sensitive dependencies. Select actual target/feature
flags per lane; trees approximate, not byte-level proof of shipped code.

Compare declared requirements, locked/proposed exact versions using primary release
metadata. Scanner/outdated helpers may aid discovery; installation/latest lists
are neither required nor compatibility proof. Check resolved transitive MSRV and
selected Windows SDK/linker/architecture lanes, beyond direct crates.

For approved compatible updates, prefer scoped operations such as
`cargo update -p PACKAGE --precise VERSION` over blanket updates. Inspect required
transitive changes. Supporting toolchains offer `--dry-run` resolution previews.
Change manifest requirements only for intended cohort; resolve/review resulting
graph. Use exact reviewed git revisions under architecture policy; never substitute
branch heads for release checksums.

For local forks/vendor patches, reconcile root patch routing, lock entries,
upstream revision, diff/hash, license notices and patch reasons. Map each retained,
changed or dropped patch to detecting regression/upstream removal condition.
Workspace tests may exclude vendored packages; run required named regressions
explicitly. Published-base audits do not attest to changed local path dependencies.

Features are additive/unified under applicable resolver rules. Direct
`default-features = false` cannot prove no other enabling path. Inspect actual
features, source, checksum, patches, build.rs, proc macros and native libraries.
Deliberately select future virtual workspace resolver: Rust 2024 implies resolver 3
for packages, but virtual workspaces still need explicit workspace setting.
Choose edition/MSRV only under authorized workspace decision, never incidentally.

## Qualify the final graph

Use real repository CI/TOOLING commands for selected Windows/portable lanes.
Once workspace exists, representative checks:

```sh
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
cargo build --locked --workspace
```

Apply intended features/target/profile and required checks. All-targets cannot cover
every OS/architecture; avoid all-features with conflicting backends. Order checks
without concurrent Cargo source/lockfile or shared-evidence mutation. Inspect
claim-relevant generated code/outputs. Resolution/download proves no compilation;
compilation proves no runtime behavior; host builds prove no Windows MSI/service/
clock qualification.

Use admitted advisory/license/origin checker; record policy/database revision,
retain existing findings and assess changed graph. Chrony-derived
code's GPLv2-only constraints and selected transitive license grants need concrete
review. A permissive UI example license does not license its framework. Avoid
unsupported universal claims about Rust TLS providers or Windows trust parity.

Final receipt identifies source/lock/toolchain/features/target, graph delta,
advisory/licensing results, commands/statuses and actual artifacts. Unsigned
reproducible builds and timestamped signing have different byte identities.
Claim no release qualification before required native evidence.
[SOURCES](../../SOURCES.md) records primary workflow research.
