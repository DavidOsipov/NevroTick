# Local adaptation

Source: [po4yka/rust-skills](https://github.com/po4yka/rust-skills), commit
`5d5f4706d6bf1d03366a1fb463aa1732de2dbbd2`, directory
[`skills/rust-compiler-errors`](https://github.com/po4yka/rust-skills/tree/5d5f4706d6bf1d03366a1fb463aa1732de2dbbd2/skills/rust-compiler-errors).
The upstream [BSD-3-Clause license and copyright notice](LICENSE) are retained
unchanged. This adaptation is reference guidance, not application implementation
or evidence that upstream examples ran in NevroTick.

Substantive local edits:

- Narrowed discovery to actual compiler diagnostics; JSON-quoted the existing
  frontmatter scalar fields and added `agents/openai.yaml` with interface metadata.
- Established the existing implementation and QA owners, actual failure lane and
  approved state/effect authority. Removed any implication that the reference
  authorizes new dependencies, features, runtimes, unsafe core or toolchain changes.
- Made diagnostic commands illustrative shapes for the existing failed lane.
  Distinguished default `cargo check` targets and possible codegen/link-only
  failures from the original command's outcome.
- Corrected clone availability and timing, independent versus shared ownership,
  loop allocation claims, and type-specific cost. Multiple readers/writers now
  require an ownership decision rather than a blanket `Arc`/`Arc<Mutex<_>>` answer.
- Distinguished `&'static T` from `T: 'static`. Corrected E0277's unsatisfied
  implementation/bound meaning; a trait import alone does not satisfy that bound.
- Qualified E0499's disjoint-access options and E0509's non-`Copy` move restriction.
  Kept cleanup order and resource liveness when considering separate drop guards.
- Qualified a future's `Send` requirement and stated that `Arc<T>` does not repair
  its inner type's auto traits. Removed mandatory absent specialist routes for
  packed fields, union cleanup and recursive generic instantiation.
- Replaced automatic dependency/feature additions with current-lane diagnosis and
  the existing dependency workflow. Included Cargo dependency rename semantics.
- In `references/borrow-checker-fixes.md`, qualified `mem::take`'s `Default` cost,
  `Cell::get` versus `Cell<T>` bounds, platform/access-dependent `RwLock` failure
  and atomics' logical/ordering risks. Removed a suggested unsafe `ManuallyDrop`
  size optimization and preserved cleanup ownership in the guard example.
- Replaced absent named-skill routes with available project owners. Installed
  companion references remain optional and do not trigger further installation.
  `references/dyn-compatibility.md` and `LICENSE` remained unchanged during initial adaptation.

Primary references for the corrections:

- [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html)
- [`'static` reference lifetime and trait bound](https://doc.rust-lang.org/rust-by-example/scope/lifetime/static_lifetime.html)
- [E0277](https://doc.rust-lang.org/error_codes/E0277.html)
- [`mem::take`](https://doc.rust-lang.org/std/mem/fn.take.html)
- [`Cell`](https://doc.rust-lang.org/std/cell/struct.Cell.html)
- [`RwLock` priority and deadlock caveats](https://doc.rust-lang.org/std/sync/struct.RwLock.html)
- [`Arc` thread safety](https://doc.rust-lang.org/std/sync/struct.Arc.html#thread-safety)
- [Atomic memory ordering](https://doc.rust-lang.org/std/sync/atomic/index.html#memory-model-for-atomic-accesses)
- [E0509](https://doc.rust-lang.org/error_codes/E0509.html)
- [`cargo check`](https://doc.rust-lang.org/cargo/commands/cargo-check.html)
- [Renaming Cargo dependencies](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#renaming-dependencies-in-cargotoml)

No upstream examples or downloaded executable code were run for this adaptation.

Later local change: compressed instructional prose; preserved code, metadata,
headings, links, technical conditions and licence texts. Upstream sources/pins unchanged.
