# Scoped Cargo operations and evidence

Inspect actual pinned versions/help/workspace before commands. Use the current
[configured-tool workflows](../../../testing/references/configured-tooling.md);
examples do not establish an application graph or executed checks. Use an
authorized owned candidate; retain
baseline manifest/lockfile identities. Never delete Cargo.lock to force resolution.

## Discover and resolve

Separate acquisition covered by the task from frozen graph inspection. Missing
cached sources or admitted metadata are preparation gaps, not permission for an
analysis command to fetch implicitly.

Before adding a newly proposed third-party dependency, match its canonical
package name, registry where applicable, and source to the intended upstream's
primary release material or reviewed source revision. Package existence or a
pinned checksum does not establish that match; keep an unverified generated name
as a research candidate.

`cargo metadata --frozen --format-version 1` describes resolved packages/targets;
retain needed fields only; print no credentials. `cargo tree --frozen -e features`
exposes feature activation; inspect `cargo tree --frozen -d` for multiple versions
and reverse graphs for security-sensitive dependencies. Select actual target/feature
flags per lane; trees approximate, not byte-level proof of shipped code.

Compare declared requirements, locked/proposed exact versions using primary release
metadata. Scanner/outdated helpers may aid discovery; installation/latest lists
are neither required nor compatibility proof. Check resolved transitive MSRV and
selected Windows SDK/linker/architecture lanes, beyond direct crates.

For approved compatible updates, prefer scoped operations such as
`cargo update -p PACKAGE --precise VERSION` over blanket updates. Inspect required
transitive changes. This intentionally changes resolution and is separate from
the frozen analysis lane; `--locked` would reject a required lockfile update.
Supporting toolchains offer `--dry-run` resolution previews.
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
Preserve and verify the actual virtual workspace's explicit resolver setting:
Rust 2024 implies resolver 3 for packages, but a virtual workspace does not infer
it from a package edition. The reviewed NevroTick workspace already selects 3;
MSRV-aware resolution still does not prove compilation with the selected compiler.
Choose edition/MSRV only under authorized workspace decision, never incidentally.

### Version-specific Cargo choices

With **Cargo 1.99 and edition 2024 or later**, a member can override an inherited
dependency's `default-features`, including disabling defaults enabled by the
workspace declaration. Earlier 2024-edition Cargo rejected that conflicting
`false`; older editions ignore it with a warning. Use this only for an intended
graph change and inspect all enabling paths: feature unification still applies.
See [inherited dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#inheriting-a-dependency-from-a-workspace),
[feature unification](https://doc.rust-lang.org/cargo/reference/features.html#feature-unification)
and the [1.99 release notes](https://doc.rust-lang.org/releases.html#version-1990-2026-10-01).
Reconcile versioned notes with rolling docs; an older Edition Guide description
of the 1.85–1.98 rejection is not the 1.99 rule.

Treat newer build conveniences as conditional: Cargo cache cleanup (1.88) does
not run under `--offline`/`--frozen`; `build.build-dir` (1.91) changes intermediate
artifact location; config includes/TOML 1.1 (1.94) require review of every included
input and repository parser; `build.warnings` and `resolver.lockfile-path` (1.97)
do not replace lint/linker policy or authorize an untracked lockfile. Keep source,
config, lockfile and outputs bound to the existing execution lane. See the
[release survey](../../../implementation/SOURCES.md#modern-rust-release-review--9-november-2026)
and [Cargo configuration](https://doc.rust-lang.org/cargo/reference/config.html).
Do not change global cache/configuration, publish packages or adopt a new graph
merely because a stable Cargo version supports it.

### Future publish-age watchlist — 9 November 2026

The [beta Cargo reference](https://doc.rust-lang.org/beta/cargo/reference/config.html#registryglobal-min-publish-age)
documents `registry.global-min-publish-age`, per-registry `min-publish-age` and
`resolver.incompatible-publish-age` as respected from **1.100+**. They are unavailable
in the pinned **1.99.1**; retain this as a future watchlist, not current configuration
or a reason to upgrade. Select no delay from a documentation example.

At a separately admitted later-toolchain/update-policy evaluation, recheck released
documentation and actual registry publication-time support. Use a disposable
resolution fixture for eligible older versions, too-new-only refusal, lockfile
exemptions, reviewed overrides and missing registry timestamps. This tests resolver
policy, not compromised-package detection; infer no coverage for path/git sources,
existing locked versions or `cargo install`. A delay can postpone a verified
security fix. Preserve exact pins, lockfiles and source/graph admission; change no
product/global configuration during that comparison. See [sources](../../SOURCES.md#publish-age-watchlist--9-november-2026).

### Future intermediate-layout compatibility — 9 November 2026

At the next admitted toolchain upgrade, recheck Cargo's intermediate artifact
layout against the actual verifier consumers. The [beta 1.100 changelog](https://doc.rust-lang.org/beta/cargo/CHANGELOG.html#cargo-1100-2026-11-12)
announces a layout change for the scheduled **12 November 2026** release;
the current Rust **1.99.1** pin is unchanged. Recheck released behavior before
adoption. This is distinct from relocating intermediates with `build.build-dir`.

Use emitted Cargo JSON for discovery while preserving package/target/profile,
host-versus-target role, allowed-root, complete inventory and byte-custody checks.
JSON consumption alone does not remove assumptions in path validation or copying:
the current allocation and Core/engine/time-domain contract collectors still
match legacy `build/{package}-{hash}` paths or enumerate `target/debug/deps`.
Keep their fail-closed rejection; never accept arbitrary emitted paths to make
an upgrade pass. Final EXE/PDB exports are a separate interface. At the existing
[build checkpoint](../../../../../tasks/build-optimization.md#cargo-intermediate-layout-upgrade-check--9-november-2026),
pair actual candidate-layout healthy artifacts with wrong-identity, redirected,
substituted-byte and incomplete-inventory controls. This watchlist authorizes
neither an upgrade nor native/formal execution. See [sources](../../SOURCES.md#cargo-intermediate-layout-watch--9-november-2026).

## Qualify the final graph

Use real repository CI/TOOLING commands from the shared configured-tool workflow
for selected Windows/portable lanes. After intended resolution and acquisition,
representative final checks with a real workspace are:

```sh
cargo fmt --all -- --check
cargo clippy --frozen --workspace --all-targets -- -D warnings
cargo test --frozen --workspace
cargo build --frozen --workspace
```

Apply intended features/target/profile and required checks. All-targets cannot cover
every OS/architecture; avoid all-features with conflicting backends. Order checks
without concurrent Cargo source/lockfile or shared-evidence mutation. Inspect
claim-relevant generated code/outputs. Resolution/download proves no compilation;
compilation proves no runtime behavior; host builds prove no Windows MSI/service/
clock qualification.

Use configured cargo-deny with fatal warnings for selected graph rows and any
selected complementary cargo-audit for lockfile advisory evidence, in the
frozen-snapshot lane described by the shared workflow. Record
policy/database/index identities and refresh receipts no older than 24 hours;
retain existing findings and assess changed graph. Missing/stale metadata is not
a pass. Chrony-derived
code's GPLv2-only constraints and selected transitive license grants need concrete
review. A permissive UI example license does not license its framework. Avoid
unsupported universal claims about Rust TLS providers or Windows trust parity.

Final receipt identifies source/lock/toolchain/features/target, graph delta,
advisory/licensing results, commands/statuses and actual artifacts. Unsigned
reproducible builds and timestamped signing have different byte identities.
Claim no release qualification before required native evidence.
[SOURCES](../../SOURCES.md) records primary workflow research.
