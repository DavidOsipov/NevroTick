---
name: "dependency-updates"
description: "Research, update and qualify a bounded NevroTick Cargo dependency cohort, including features, MSRV, Windows support, licenses and supply-chain changes."
---

# Rust dependency updates

Use architecture section 18, especially 18.3 and 18.5–18.10. Its package/version
register is retained research, not an installed/qualified graph or permission to
enable defaults. If no Cargo project exists, produce the requested research or
admission proposal; do not create a workspace or install packages merely to update
the skill pack. Existing task authorization controls actual updates.

Inspect branch/HEAD and staged/unstaged/untracked ownership, manifests, Cargo.lock,
toolchain/MSRV/edition, target/features, patches, git sources, Cargo configuration
and build/tool policy. Preserve existing work, lockfile and receipts. Use an owned
isolated candidate when updates would conflict with another worker's graph/tests.
Do not reset/stash/overwrite or expose registry credentials.

Record the baseline graph identity, enabled features, runtime/build/proc-macro/
native components, advisory database revision and existing findings. Research a
bounded cohort against primary release notes, manifests/source, security policy,
Windows support and license grants. Report patch/minor/major/zero-major,
prerelease and git-source changes separately; SemVer does not establish behavioral
or license compatibility.

Before resolving, decide the cohort and visible holds. Re-evaluate Serde/parser
behavior, TLS provider/trust/licensing, NTS packet AEAD and Win32/service boundaries
when affected. Preserve held/optional candidates; do not replace the selected
validation model or admit TUF from research without a concrete design decision.

Use [Cargo operations and evidence](references/cargo-operations.md) for scoped
resolution, feature/source inspection and qualification. Review the entire resulting
lockfile delta, including unexpected transitives; preserve justified patches and
explain changes. New build scripts/proc macros/native compilation execute code and
need provenance review before a build, not only a runtime-crate audit afterward.

Run applicable pinned compiler/Clippy/tests/build and policy checks for the final
cohort in its target matrix, using locked resolution. Add causal parser/protocol/
Windows tests where the changed guarantee requires them; do not run every expensive
gate blindly. Advisory/license checks need the actual graph and fresh database;
two tools using the same database are not independent assurance.

Finish with admitted versions/checksums/origins/features/license grants, reasons and
held candidates, transitive delta, old/new findings, executed checks and missing
Windows/release evidence. Report inherited failures distinctly. Do not stage,
commit, sign, publish or release as an implied dependency-maintenance step.

Adapted David Osipov skill material: [licence and notices](THIRD_PARTY_NOTICES.md).
