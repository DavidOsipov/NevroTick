# Ownership, cloning and allocation anti-patterns

Apply when writing or reviewing Rust ownership, state mutation or hot-path work.
These rules apply equally to human and AI contributions. They forbid the defects
below, not the `Clone` trait, allocation or shared ownership in general. Keep the
[implementation contract](../../CONTRACT.md) and actual component owner authoritative.

## Forbidden patterns and required alternatives

| Forbidden pattern | Required alternative and evidence |
| --- | --- |
| Clone state only to silence a borrow error, mutate the detached copy, then report that the original owner changed. | Trace the authoritative producer, mutation target and consumer. Borrow the owner mutably, move ownership, or stage and explicitly commit a bounded candidate. Test the owner's observable state, not only the candidate or returned success. |
| Treat cloning as a uniform deep copy or a new independent authority. | Inspect the concrete `Clone` implementation and fields. A cloned `Arc<T>` shares the same allocation; a cloned struct can mix owned values and shared handles. Preserve identity, generations and authorization at the real effect owner. |
| Introduce `Arc<Mutex<_>>`, extra reference counting or clones merely to defeat borrow/lifetime errors. | Fix ownership and lifetime boundaries first. Admit shared ownership only for a real shared consumer, with synchronization, cancellation and release responsibilities. Locks must cover the invariant-bearing transaction. |
| Add avoidable per-packet/per-sample allocation, collection cloning or formatting in an established bounded hot path without reviewing its cost and limits. | Prefer existing borrows, moves, reused bounded buffers or small values where they preserve semantics. Account for cumulative bytes/work and validate before allocation. Measure a claimed performance improvement in the actual path; a different spelling is not a benchmark. |
| Remove cloning by extending a lock across blocking/async work, exposing mutable aliases, adding unsafe code, or sharing a snapshot that must remain independent. | Preserve the original state/concurrency/secret-lifetime contract. A justified bounded clone can be the correct implementation. Recheck authority at commit after intervening work. |

Intentional snapshotting, failure-atomic staging, independent outputs and task
ownership can justify clones. Name the purpose at the owning API or non-obvious
call site; ordinary idiomatic clones need no boilerplate annotation. Do not add a
blanket `Clone` derive to a handle, pending request or capability whose duplication
would violate its unique-use contract. Conversely, absence of `Clone` does not
prove unique authority through all other constructors and aliases.

`Arc::clone(&value)` makes reference-count sharing explicit; it does not copy `T`
and does not allocate a new `T` per clone. It still performs reference-count work
and prolongs the allocation's lifetime. `Arc::make_mut` can clone the inner value
when needed; reason about which owner sees the resulting mutation. Do not replace
thread-safe ownership with `Rc` where cross-thread use is required, or invent a
universal zero-allocation requirement for all UI, initialization or error paths.

## Enforcement and its limits

| Mechanism | Detects | Does not establish |
| --- | --- | --- |
| Existing denied Clippy `all`, including `clone_on_copy` | Unnecessary explicit cloning of recognized `Copy` values. | That a non-`Copy` clone is necessary, updates the correct state, or is cheap. |
| Existing denied `pedantic`, plus ownership/contract review | Additional suspicious API/ownership idioms within each lint's documented scope. | Absence of semantic ownership defects or bounded allocation. |
| Behavioral regression and deliberate clone-mutation control | The actual consumer observes the required original-owner transition; replacing it with detached mutation fails. | Other untested paths or lifetime/concurrency correctness. |
| Scoped allocation/work measurements and boundary tests | Costs and limits for the measured workload/candidate, including failure/cancellation. | A universal timing or memory guarantee from one successful run. |

Do not add duplicate individual lint settings when an existing denied group
already supplies the rule. The existing strict runner now contains a healthy
copy/intentional-clone neighbor and method/UFCS negative controls for
`clone_on_copy`. These are compiler controls, not a detector of every accidental
clone. Run them with the admitted pinned compiler before claiming qualification;
Python policy tests alone cannot establish Rust diagnostic behavior.

Do not enable `clippy::restriction` or `clippy::nursery` wholesale. In particular,
`redundant_clone` is a candidate for a separately qualified change, not a currently
added mandatory gate. `clone_on_ref_ptr` controls spelling, not whether sharing is
semantically valid. A custom regex looking for `.clone()` cannot resolve types,
aliases, macros or ownership intent and must not be sold as an equivalent lint.
New semantic analysis requires a demonstrated gap, false-positive controls and
integration with the existing runner, not another unqualified mandatory tool.

For a changed mutation path, retain both legal-use and detached-copy adversarial
cases at the actual owner. Challenge lost updates, stale staged results and
failure atomicity as applicable. For changed hot paths, measure actual bounded
work and retained resources; apply the existing QA/mutation gates. No arbitrary
allocation count, new percentage or product-wide benchmark campaign is selected
by this reference. See [performance optimization](../../optimize-rust-performance/SKILL.md)
and [role API restrictions](../../../testing/references/role-api-restrictions.md).

## Pending qualification of `redundant_clone`

Do not report this candidate as enabled or passing. In the existing isolated
strict-fixture lane, first verify the installed admitted Clippy recognizes the
lint. Temporarily pass `-D clippy::redundant_clone` only to this experiment; do not
change workspace policy until the controls and a bounded application assessment
pass. A diagnostic for `unknown_lints`, setup or a different lint is not success.

Use this deliberately unnecessary owned clone as a negative control:

```rust
/// Return the owned text without needing an independent snapshot.
#[must_use]
pub fn unnecessary_snapshot() -> String {
    let original = String::from("probe");
    original.clone()
}
```

The repaired neighbor returns `original` by move. A second legal neighbor must
retain both values: a function returning `(String, String)` whose final expression
is `(original.clone(), original)`. Require both neighbors to compile warning-free
and the negative to fail specifically with `clippy::redundant_clone`; preserve
any independent failing diagnostic for repair rather than treating it as a pass.
Also retain an `Arc::clone` shared-consumer case and a deliberate failure-atomic
snapshot with both original and candidate subsequently used. Inspect findings
on the actual workspace/targets for drop order and intentional ownership effects.
If an admitted compiler cannot support reliable controls, leave this experiment
pending; do not weaken another lint or add a repository-wide exemption.

## Primary sources and provenance

- [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html): explicit copying;
  smart pointers may still refer to the same underlying data.
- [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html), including cloning and
  `make_mut`: shared allocation, atomic reference counting and clone-on-write.
- [Clippy `clone_on_copy`](https://rust-lang.github.io/rust-clippy/stable/index.html#clone_on_copy),
  [`redundant_clone`](https://rust-lang.github.io/rust-clippy/stable/index.html#redundant_clone)
  and [`clone_on_ref_ptr`](https://rust-lang.github.io/rust-clippy/stable/index.html#clone_on_ref_ptr):
  separate rule scopes and lint groups; confirm behavior on the admitted compiler.
- [Cargo lint inheritance](https://doc.rust-lang.org/cargo/reference/workspaces.html#the-lints-table).

The user's [Reddit link](https://www.reddit.com/r/rust/s/FdCMMzmYYL) prompted this
review. Its complete discussion was unavailable during preparation; this is not
an exhaustive summary or endorsement of that thread. The rules above are original
project guidance grounded in Rust's documented semantics and existing contracts.
