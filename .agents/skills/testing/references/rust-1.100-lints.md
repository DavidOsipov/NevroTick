# Use Rust 1.100 diagnostics without weakening contracts

Use for lint-policy changes or fixes involving the released Rust/Cargo 1.100
diagnostics. Inspect the current manifests and configured commands first;
this reference supplies guidance, not adopted configuration or execution evidence.
Match the admitted toolchain to the pinned sources below; reverify on an upgrade.

## Contents

- [Enforcement namespaces](#keep-the-enforcement-namespaces-distinct)
- [Unused dependencies](#qualify-unused-dependencies-before-making-them-errors)
- [Clippy and intent](#use-existing-clippy-enforcement-and-preserve-intent)
- [Compiler and documentation controls](#select-compiler-and-documentation-controls-only-when-relevant)
- [Pinned sources](#pinned-primary-sources)

## Keep the enforcement namespaces distinct

Cargo manifest rules belong in `[workspace.lints.cargo]` or `[lints.cargo]`.
They are distinct from `[workspace.lints.rust]` and `clippy::cargo`.
Rust `warnings = "deny"` and Clippy's trailing `-D warnings` do not establish
Cargo-warning rejection. Verify the actual exit/output policy through the runner.
Cargo's public groups are `default`, `correctness`, `style`, `suspicious`,
`pedantic` and `restriction`; there is no public Cargo `all` group. Prefer named
rules to blanket group adoption; consult the [released catalog][cargo-lints].

Consider individual `deny` adoption for these three warn-by-default Cargo rules:

| Cargo lint | Mistake detected |
| --- | --- |
| `unknown_lints` | Unknown Cargo lint names, including misspellings; not unknown rustc/Clippy names. |
| `unused_workspace_dependencies` | A workspace dependency inherited by no member. |
| `unused_workspace_package_fields` | A shared package field inherited by no member. |

The last two do not prove that every member which needs a declaration inherits it.
Implement adoption as one coordinated change to root `Cargo.toml`, canonical
`policy/rust-workspace.toml`, `scripts/check-strict.py`'s admitted namespaces and
required rules, and `scripts/tests/test_strict_policy.py`. The reviewed checker
admits only rust/Clippy/rustdoc; adding a Cargo table alone is rejected. Inspect
current ownership before editing; this skills reference changes none of those files.
Retain root/canonical equality and exact member `[lints] workspace = true` checks
in `check-strict.py` and `verify.py`. Cargo's `missing_lints_inheritance` accepts
an empty `[lints]` as deliberate opt-out; a noninheriting member also lacks the
workspace lint level. It cannot replace the stronger repository adoption check.

Qualify a valid inherited policy, a misspelled Cargo lint and unused shared entries.
Reject missing/empty/local-only member lint tables through repository admission.
For each new required rule, reject its removal or lowered level; require the
intended diagnostic/nonzero propagation and a restored accepted neighbor.

## Qualify unused dependencies before making them errors

Treat `cargo::unused_dependencies` as evidence to investigate, not authorization
to delete dependencies. It checks selected packages and target-dependent normal/
build dependency coverage; `--all-targets` still does not cover dev-dependencies.
Selecting only a binary can skip normal-dependency assessment. A selected package
does not automatically qualify every unselected path dependency.

Preserve intentional feature carriers. The shipped implementation suppresses some
transitive dependencies as possible feature activators, while its docs still warn
of false positives. Do not predict every carrier will warn. In particular, retain
the documented non-Windows `i-slint-core` SVG/image feature carrier in
`crates/nevrotick-ui/Cargo.toml` unless its actual graph/consumer contract changes.
The upstream `cfg(false)` suggestion addresses version-pin-only dependencies;
do not use it to disable needed feature activation.

Before adopting `deny`, qualify used/unused normal and build dependency pairs,
a legitimate feature carrier, and relevant supported host/Windows feature rows.
Record uncovered dev-dependency/selection scope. Preserve lockfile, feature graph
and dependency-admission owners; no blanket deletion or broad suppression.

## Use existing Clippy enforcement and preserve intent

The released `option_zip_none` (suspicious) and `needless_nonzero_get` (complexity)
are covered by the existing `clippy::all = deny`; do not add redundant flags or
claim new policy adoption. Use the pinned implementation, not a rolling changelog
or a source version annotation alone, to establish shipped behavior.

- For `option_zip_none`, pair a meaningful two-option `zip` with a one-side-`None`
  defect and require that lint. Recover the intended result and evaluation effects;
  its suggested rewrite is not a semantic oracle or permission for blind auto-fix.
- For `needless_nonzero_get`, pair `nz.leading_zeros()` with
  `nz.get().leading_zeros()` on a supported `NonZero` type. Preserve the constrained
  value where useful; do not remove every `.get()` or claim a new nonzero proof.
- Preserve exhaustive named ignored fields where new fields must trigger review.
  Do not enable `clippy::unneeded_field_pattern`; its `..` suggestion defeats that
  [structural contract](../../implementation/implement-rust-contracts/references/contract-enforcement.md#make-structural-changes-require-review).

## Select compiler and documentation controls only when relevant

Rust 1.100 extends `dropping_references`, `dropping_copy_types` and
`undropped_manually_drops` to free-function and pointer-method `drop_in_place`.
Existing warning denial/default denial covers them. Unsafe deny/forbid already
blocks ordinary unsafe code; a relevant reviewed-adapter control must fail for the
drop diagnostic, not merely for unsafe syntax. Use compile-only controls and
retain narrow reasoned exceptions.

The new `repeated_reprs` warning is covered by warning denial;
`aligned_fields_in_packed` is deny-by-default. For changed layout enforcement,
pair intended rejections with legitimate representation declarations. The new
`rustdoc::invalid_markdown_table` is covered by existing `rustdoc::all`; escaped
table cells versus lossy unescaped pipes test documentation rendering, not runtime
invariants. Add only controls needed for the changed guarantee, not a blanket suite.

Use [configured tool execution](configured-tooling.md): admission, relevant strict
fixtures, then affected product rows. Missing tooling/isolation stays incomplete;
never bypass the runner with raw Cargo. Keep semantic consumer tests and the
existing post-V1 proof deferral; clean diagnostics prove no authorization or cleanup.

## Pinned primary sources

[cargo-lints]: https://github.com/rust-lang/cargo/blob/3acc79c0c841844675bc53a605922dbfc441e750/doc/book/src/reference/lints.md

- [Cargo inheritance](https://github.com/rust-lang/cargo/blob/3acc79c0c841844675bc53a605922dbfc441e750/src/diagnostics/rules/missing_lints_inheritance.rs) and [unused dependencies](https://github.com/rust-lang/cargo/blob/3acc79c0c841844675bc53a605922dbfc441e750/src/diagnostics/rules/unused_dependencies.rs).
- [Clippy Option diagnostic](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/src/tools/clippy/clippy_lints/src/methods/option_zip_none.rs), [NonZero diagnostic](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/src/tools/clippy/clippy_lints/src/needless_nonzero_get.rs) and [group registration](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/src/tools/clippy/declare_clippy_lint/src/lib.rs).
- [Drop diagnostics](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/compiler/rustc_lint/src/drop_forget_useless.rs), [repr lint definitions](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/compiler/rustc_lint_defs/src/builtin.rs) and [rustdoc rules](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/src/librustdoc/lint.rs).
