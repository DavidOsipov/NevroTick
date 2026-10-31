# Optional same-evaluation brands

Use for a concrete same-evaluation association obligation at a narrow synchronous
Rust boundary. Existing immutable borrows plus checked entry/context/membership
may already suffice; do not infer a missing protection from the absence of brands.
Trial one API only when it becomes easier to use correctly. This reference adopts
no dependency or repository-wide lifetime scheme.

## Freshness and invariance are separate obligations

An ordinary borrow lifetime establishes storage validity, not a unique evaluation.
An invariant `PhantomData` marker prevents some lifetime coercions but creates no
fresh identity by itself. Use a reviewed generative factory: for example, a scoped
higher-ranked callback whose output cannot carry the fresh brand, or an admitted
generativity guard. Generate a fresh brand per actual evaluation, rather than
reusing a broad scope's token for unrelated computations.

Keep brand introduction and checked-object construction at the private evaluation
owner. Accepting arbitrary data plus a trusted brand could merely relabel unrelated
objects. Review all constructors, variance, coercion and escape paths. Do not add
unchecked constructors, transmute or unsafe lifetime extension to satisfy callers.
Pure core's existing unsafe prohibition remains; any proposed crate needs the
normal exact dependency/features/license/source review.

`generativity::Id` is copyable/cloneable: association is not exclusive ownership,
consumption, a one-shot permission or distinct entry identity. A shared evaluation
brand alone proves neither selected-subset membership nor correct source/context,
authentication, numerical uncertainty or current external authority. Preserve
actual-entry/membership/generation and effect guards unless their redundancy is
separately established by the changed contract and discriminating evidence.

## Keep lifecycle and implementation limits explicit

Awaiting does not inherently invalidate a brand, but it proves no freshness or
authorization after the await. Scoped lifetimes can conflict with `'static`
spawned tasks. Persistence/restart/serialization require ordinary trusted identity,
generation checks and fresh admission; never erase and unsafely reconstruct a
brand to bridge those boundaries. Review applicable Send/Sync and ownership needs
rather than treating phantom markers as scheduling guarantees.

Inspect the selected generativity implementation and compiler, not just its API.
Upstream [issue 15](https://github.com/CAD97/generativity/issues/15) reports a
duplicate-brand problem involving diverging blocks; the
[1.2.1 release](https://github.com/CAD97/generativity/releases/tag/v1.2.1) records
its fix. Any trial needs exact-version admission and a corresponding divergent
scope control; this historical fix is no blanket soundness or adoption proof.

Retain a same-evaluation legal-use consumer and external compile-fail consumers
for mixing evaluations, escaping/rebranding and relevant variance/divergence
paths. Require the intended type/lifetime diagnostic, not an import/setup error.
Retain behavioral controls for association, membership and authority that the
brand does not encode. A sound scoped API prevents the selected safe-call mixup;
compiler success alone proves no numerical or native correctness.

Record the trial's simplification/cost and remaining guards in the existing task.
Use CHECK-25, CHECK-7 where numerical associations depend on it, and CHECK-20 for
any dependency proposal; no new acceptance threshold follows from this technique.

Primary semantics: [Rust variance](https://doc.rust-lang.org/reference/subtyping.html#variance),
[higher-ranked trait bounds](https://doc.rust-lang.org/reference/trait-bounds.html#higher-ranked-trait-bounds),
[generativity 1.2.1](https://docs.rs/generativity/1.2.1/generativity/),
[ID invariance, copying and construction](https://docs.rs/generativity/1.2.1/generativity/struct.Id.html).
