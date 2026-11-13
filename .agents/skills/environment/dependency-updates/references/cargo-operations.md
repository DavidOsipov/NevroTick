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
For convenience features such as NTS, trace nested TLS/provider/post-quantum
activation and the provider actually constructed; the name or resolved graph
alone does not prove shipped or executed code.
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

<a id="future-publish-age-watchlist--9-november-2026"></a>

### Registry dependency cooldown — 13 November 2026

[OD-19](../../../../../docs/nevrotick-owner-decisions.md#23-od-19--14-day-registry-dependency-cooldown)
selects **14 days**, with explicit `resolver.incompatible-publish-age = "deny"`,
using the admitted Cargo 1.100 cohort. The former no-delay watchlist is historical.
This is the owner's balance between early-release exposure and delayed fixes;
it is not a measured safe age or a compromised-package detector.

Before approving a changed registry graph, reproduce the intended precise update
or manifest/feature/target/patch resolution from the accepted baseline in an owned
candidate through the guarded acquisition owner. Compare the entire parsed lock,
including edges and newly selected direct/transitive, host/build and proc-macro
identities. An imported/manual/bot lock is a proposal, not fresh age approval.
Initial or independently owned workspace resolution needs its own manifest,
lock/config and selected target/features; root settings are not assumed to apply
from an unrelated working directory. Never delete an accepted lock to force uptake.

Review publication-time evidence and the recorded UTC resolution/cutoff for every
new registry version before vendoring exact archives or executing package code.
Missing timestamps leave changed-version age unverified and unadmitted unless an
exact concrete exception applies. Existing intended-upstream, checksum, licence,
patch, feature, advisory and build-time reviews remain. The unchanged accepted
baseline is retained without inventing retrospective age evidence.

Use OD-04 for a verified urgent fix or blocking compatibility issue, identifying
exact package/version, primary evidence, urgency, age, targets/features and the
**whole proposed delta**. Only its one owned precise-update child may receive
`CARGO_RESOLVER_INCOMPATIBLE_PUBLISH_AGE=allow`; it disables age filtering for the
whole invocation. Keep the exception attached to the exact resulting graph and
verify subsequent fresh normal selection still uses 14/deny. Never persist allow,
a shorter per-registry setting, package exemption or shell/CI override. The general
policy grants no particular urgent exception.

Cargo retains locked versions and permits missing `pubtime`; git/path and ordinary
directory/vendor sources have no equivalent age filter. Shipped Cargo 1.100 also
excludes `cargo install`; RFC 3923's earlier applicability text does not override
released behavior. Keep admitted archive/rustup tool acquisition. Ordinary checks
stay frozen; `prepare-product.py` acquires already-approved bytes, while
`refresh-product-advisories.py` refreshes only metadata under a byte-identical lock.
Neither operation retroactively proves age. Preserve source/config/environment
admission and reject unauthorized `--config` or inherited age overrides.

The guarded metadata-only entrypoint is `scripts/dependency_cooldown.py`, using
an external private candidate, explicit accepted baseline lock, admitted toolchain,
dedicated bounded quota and new output directory. Fixed operations are `precise`,
`workspace` and independently owned `initial`; `--proposed-lock` compares an import
against the reproduced complete lock. It publishes only a generated lock/decision.
See [TOOLING's exact invocation](../../../../../TOOLING.md#registry-dependency-cooldown--selected-13-november-2026).
Source/policy acceptance and the small code-owned current-baseline checkpoint are
reviewed together; a proposed or standalone result cannot approve itself.

The existing [build record](../../../../../tasks/build-optimization.md#accepted-14-day-dependency-cooldown--13-november-2026)
owns actual guarded commands, fixtures, exception controls and remaining limits.
The isolated sparse fixture uses explicit 15/13-day UTC neighbors without changing
the host clock, public-registry traffic or package execution. Source/TOML checks
and isolated fixtures are separate from product/native qualification. See
[sources](../../SOURCES.md#publish-age-watchlist--9-november-2026).

### Future intermediate-layout compatibility — 9 November 2026

At the next admitted toolchain upgrade, recheck Cargo's intermediate artifact
layout against the actual verifier consumers. The [beta 1.100 changelog](https://doc.rust-lang.org/beta/cargo/CHANGELOG.html#cargo-1100-2026-11-12)
announces a layout change for the scheduled **12 November 2026** release;
that 9 November baseline selected Rust **1.99.1**. The current 1.100 implementation
and actual collector results are recorded in the existing build task. This is distinct from relocating intermediates with `build.build-dir`.

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

### Rust 1.100 candidate reconciliation — 11 November 2026

For the broader compiler/library/lint/Windows changes, use the
[dated research inventory](../../../../../docs/supporting-reviews/rust-1.100-upgrade-review.md)
and [existing build checkpoint](../../../../../tasks/build-optimization.md#rust-1100-candidate-admission--11-november-2026).
Keep this reference's publish-age/layout controls; do not duplicate their tests.
Rolling beta/nightly docs and channel names can already refer to 1.101: record
actual compiler/Cargo/component identities and recheck final release metadata
before attributing a check or feature to 1.100. The research-time pin was 1.99.1;
the current admitted 1.100 cohort and scoped results belong to the build task.
The candidate's Cargo-lint and cargo-install lockfile behavior need their own
exact-version qualification; neither changes workspace frozen-build semantics,
permits automatic dependency deletion, or supplies tool admission. No upgrade,
new Cargo policy, graph change or execution hold is lifted by this research.

For the shipped 1.100 manifest-lint namespace, inheritance checks or dependency
diagnostics, use [Cargo and lint enforcement](../../../testing/references/rust-1.100-lints.md).
Retain legitimate feature activation and the exact member-adoption checks;
instructional recommendations do not mean a Cargo deny policy is already configured.

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

Reconcile package-level licence metadata with the notices and grants on the exact
files or test material selected for reuse. A permissive workspace declaration
does not resolve a conflicting file header: record the ambiguity and obtain
adequate file-specific grant/provenance evidence before copying or adapting it.
Keep independently specified ideas/tests distinct from copied material and retain
required notices; rewriting code alone establishes no clean-room or relicensing
claim.

Final receipt identifies source/lock/toolchain/features/target, graph delta,
advisory/licensing results, commands/statuses and actual artifacts. Unsigned
reproducible builds and timestamped signing have different byte identities.
Claim no release qualification before required native evidence.
[SOURCES](../../SOURCES.md) records primary workflow research.
