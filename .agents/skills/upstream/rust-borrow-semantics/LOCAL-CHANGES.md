# Local adaptation

Source: [po4yka/rust-skills](https://github.com/po4yka/rust-skills), commit
`5d5f4706d6bf1d03366a1fb463aa1732de2dbbd2`, directory
[`skills/rust-borrow-semantics`](https://github.com/po4yka/rust-skills/tree/5d5f4706d6bf1d03366a1fb463aa1732de2dbbd2/skills/rust-borrow-semantics).
The upstream [BSD-3-Clause license and copyright notice](LICENSE) are retained
unchanged. The retained language examples are reference probes, not evidence of
NevroTick behavior or a chosen edition/MSRV migration.

Substantive local edits:

- Narrowed discovery to exact-expression temporary, drop and two-phase borrow
  questions; JSON-quoted the existing frontmatter scalar fields and added
  `agents/openai.yaml` with interface metadata.
- Established the existing implementation and QA owners, actual Cargo/target
  preflight, and approved state/effect authority. Guard scope must preserve the
  entire invariant-bearing transaction; the skill does not authorize new
  dependencies, runtimes, features, unsafe core or toolchain changes.
- Replaced the constructor-lifetime suggestion to raise MSRV with naming the
  owner at the selected MSRV. A migration remains with its existing project owner.
- Replaced unconditional guard cloning/release with extraction of only the needed
  owned fact when the invariant allows release. A reentrant access may require
  redesign while retaining the guard. Removed the absent pattern-skill route.
- Made callee-signature changes conditional on its actual borrow requirement and
  linked general compiler triage as an optional installed companion.
- Replaced unconditional `--all-features` with one explicitly selected supported
  target/feature lane, preserving default-feature selection. Verification scope
  remains with the existing project owner rather than becoming a release gate.
- Required an already available toolchain for probes. Replaced absent specialist
  routes with the available project owner; no new skill or tool is required.

Primary references:

- [Temporary scopes and lifetime extension](https://doc.rust-lang.org/reference/destructors.html#temporary-scopes)
- [Two-phase borrows](https://rustc-dev-guide.rust-lang.org/borrow_check/two_phase_borrows.html)
- [Edition 2024 `if let` temporary scope](https://doc.rust-lang.org/edition-guide/rust-2024/temporary-if-let-scope.html)
- [Edition 2024 tail-expression temporary scope](https://doc.rust-lang.org/edition-guide/rust-2024/temporary-tail-expr-scope.html)
- [Rust 1.89 constructor temporary lifetime extension](https://doc.rust-lang.org/releases.html#version-1890-2025-08-07)
- [Cargo feature selection](https://doc.rust-lang.org/cargo/reference/features.html#command-line-feature-options)

No upstream examples or downloaded executable code were run for this adaptation.

Later local change: compressed instructional prose; preserved code, metadata,
headings, links, technical conditions and licence texts. Upstream sources/pins unchanged.
