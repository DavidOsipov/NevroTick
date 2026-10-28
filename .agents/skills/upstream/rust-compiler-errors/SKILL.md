---
name: "rust-compiler-errors"
description: "Diagnose an actual rustc or Cargo failure involving ownership, lifetimes, trait bounds, inference, dyn compatibility or imports. Use rust-borrow-semantics for expression-specific temporary and drop timing."
license: "BSD-3-Clause"
---

# Rust compiler errors

NevroTick diagnostic reference alongside [implement-rust](../../implementation/implement-rust/SKILL.md); existing [QA contract](../../testing/CONTRACT.md) owns verification. Start from actual failure, existing Cargo project, selected toolchain, package, features and target. Preserve approved state owner, resource lifetime and effect-sink authority. No authorization for new dependencies, features, runtimes, edition/MSRV change or unsafe in pure core. Keep current task scope.

## First moves

```bash
# The full explanation, with a worked example. Works offline.
rustc --explain E0499

# One line per diagnostic, for a wall of output.
cargo build --message-format=short

# Count the error codes in a large failure.
cargo build --message-format=json 2>/dev/null | grep -o '"code":{"code":"E[0-9]*"' | sort | uniq -c | sort -rn
```

Use command shapes with failing command's package, features and target; loading reference alone warrants no new build. Read full first causal diagnostic before filtered count: one move/type error can cascade.

## Prove the fix

- Iterate with `cargo check` on failing package, features and target: same type/borrow errors, faster.
- By default, `cargo check` selects library/binary targets. Select failing test, bench or example explicitly; use `--all-targets` when those targets belong to supported lane.
- Before claiming fix, rerun failed command. `cargo check` and `cargo clippy` stop before code generation/linking: can exit 0 while original build fails with "reached the recursion limit while instantiating" or linker error.
- Clean build proves types check, not correct behavior. Run test covering changed code.

## Fixes that compile and hide the bug

These reflexes can silence errors while retaining/introducing defects. First establish intended ownership and behavior.

| Reflex | What it hides | When it is right, or what to do instead |
| --- | --- | --- |
| `.clone()` for E0382 or E0505 | Duplicated ownership or shared state with type-specific cost; a handle clone may create a second owner of a resource | Clone only when the type supports it and the intended independent or shared ownership requires it; see [The clone reflex](#the-clone-reflex) |
| `'static` on a field or a bound for E0106, E0597, or E0521 | `&'static T` requires a referent valid for the program remainder; `T: 'static` excludes shorter borrowed data but permits ordinary owned values that are dropped normally | State the actual borrow relationship, store owned data, or scope the borrow |
| `RefCell` or `Rc<RefCell<T>>` for E0499 or E0502 | A `RefCell already borrowed` panic at run time | A split failed and the graph shape needs it |
| `Box::leak` for E0521 or E0597 | Memory that grows once per call | Use `Arc` or `std::thread::scope` |
| `.unwrap()` or `.expect()` to get `T` out of `Option<T>` or `Result<T, E>` for E0308 or E0277 | A panic path. rustc's own `help:` for E0308 on an `Option` suggests `Option::expect` | Use `?` or handle `None` and `Err`. Use `.expect("<invariant>")` only for a documented invariant |
| An `unsafe` block, `unsafe impl Send`, or `transmute` | The check that found the bug; possible undefined behavior | Fix ownership or the type. Use [implement-rust](../../implementation/implement-rust/SKILL.md) for a real native boundary; pure core forbids unsafe |
| `#![feature(...)]` for E0658 | E0554 on stable, and a crate that builds only on nightly | Use an alternative supported by the selected toolchain. A toolchain/MSRV change belongs to its existing project owner |
| `cargo add <name>` from a `help:` line for E0432 or E0433 | rustc suggests `cargo add` for any unknown first path segment, a module of this crate included. A guessed name can be an unrelated or malicious package | Follow [Unresolved imports](#e0432-and-e0433-unresolved-imports) |
| A higher `#![recursion_limit]` for E0275 or "reached the recursion limit while instantiating" | An infinite type chain; the build fails later and slower | See the E0275 row and the "while instantiating" row of the triage table |

If repeated edits do not explain same error, stop syntax changes; answer before ownership change:

1. Which single component should own this data for its whole lifetime?
2. Does the borrow cross a boundary it should not cross: a thread, an `.await`, a callback, an
   FFI call?
3. Would the error disappear if the data were owned rather than borrowed, and what does that cost?

## Triage table

| Code | Message | What it means | First move |
| --- | --- | --- | --- |
| E0382 | borrow of moved value | The value was consumed, then used again | Decide the owner; borrow instead of moving |
| E0505 | cannot move out of `x` because it is borrowed | A live borrow outlives the move | Shorten the borrow, or move before borrowing |
| E0507 | cannot move out of `x` which is behind a shared reference | You need ownership but only hold `&` | Pick by what the original keeps: `mem::take` or `mem::replace` through `&mut`, `Option::take`, a `self` receiver, or a clone. See [references/borrow-checker-fixes.md](references/borrow-checker-fixes.md) |
| E0509 | cannot move out of type `T`, which implements the `Drop` trait | A `Drop` impl blocks moving a non-`Copy` field out of the value | If the resource invariant permits separate ownership, move cleanup into its own guard. Otherwise retain the owner and use a deliberate replacement or consuming API; preserve cleanup order |
| E0499 | cannot borrow `x` as mutable more than once at a time | Two live `&mut` to overlapping storage | Expose disjoint access with `split_at_mut`, supported `get_disjoint_mut`, or named field borrows; otherwise end one borrow first |
| E0502 | cannot borrow `x` as mutable because it is also borrowed as immutable | A read borrow is live across a write | Copy the value out, then mutate |
| E0506 | cannot assign to `x` because it is borrowed | A write to a place while a borrow of it is live | Finish the read first, or borrow the fields instead of `self` |
| E0502, E0499 | note: this call may capture more lifetimes than intended | Edition 2024: a returned `impl Trait` captures every lifetime in scope | Add `+ use<..>` to the callee's return type when the returned value does not borrow that argument (1.82+; 1.87+ in a trait). If it does borrow it, end the borrow at the caller. Do not clone at the caller |
| E0596 | cannot borrow as mutable, as it is behind a `&` reference | The parameter is `&T`, not `&mut T` | Change the signature; do not reach for interior mutability first |
| E0373 | closure (or async block) may outlive the current function, but it borrows `x` | A `'static` closure or future (`thread::spawn`, `tokio::spawn`) borrows a local | Add `move`; clone an `Arc` first if the caller still needs the data, or use `thread::scope` |
| E0597 | `x` does not live long enough | A named local is dropped while borrowed | Move the binding to the outer scope |
| E0716 | temporary value dropped while borrowed | A temporary ended before its borrow | Name the owner in a `let`; a function argument such as `bar(&foo())` is never extended. See [references/borrow-checker-fixes.md](references/borrow-checker-fixes.md) |
| E0515 | cannot return reference to local variable | The callee owns the data the caller wants | Return owned, or accept a buffer parameter |
| E0521 | borrowed data escapes outside of function | A borrowed parameter reaches a `'static` bound such as `thread::spawn` | Pass an `Arc` or owned data, or use `thread::scope` |
| E0106 | missing lifetime specifier | A struct or return type holds a reference with no stated source | Struct field: store owned data unless the type is a short-lived view, such as a parser view or a zero-copy frame; a struct lifetime spreads to every type that holds it. Return type: name the input it borrows from, with the narrowest lifetime that is true. See [references/borrow-checker-fixes.md](references/borrow-checker-fixes.md) |
| E0277 | the trait bound `T: X` is not satisfied | A required trait implementation or generic bound is unsatisfied | Read the exact type and obligation; choose a type that implements the trait, supply the intended implementation, or state the justified generic bound. A trait import alone does not satisfy `T: X` |
| E0277 | the trait bound `!: X` is not satisfied | Edition 2024 never-type fallback | Name the type, see [the fallback section](#e0282-e0283-e0284-and-the-never-type-need-a-type-anchor) |
| E0277 | `T` cannot be sent between threads safely | A non-`Send` value crosses a thread boundary | Inspect fields and ownership with [implement-rust](../../implementation/implement-rust/SKILL.md); optionally use the installed [rust-send-sync](../rust-send-sync/SKILL.md) reference |
| (none) | future cannot be sent between threads safely | A non-`Send` value lives across an `.await` in a future that must be `Send` | Read the `note:`; it names the value and the `.await`. End the value's lifetime before that point when behavior permits, or redesign ownership. `Arc<T>` does not make a non-`Send`/non-`Sync` inner `T` thread-safe. Check held guards with `clippy::await_holding_lock` |
| E0271 | expected `A` to be an iterator that yields `B` | An associated type does not match | Fix the item type, usually with `map` |
| E0308 | mismatched types | The two sides differ, often by one reference layer or one `Option` | Compare the two types the note prints, not the expressions |
| E0599 | no method named `m` found | Typo, the trait is not imported, or the method is from another version of the crate | `use` the trait from `help:`; otherwise read the locked source, see step 4 [below](#e0432-and-e0433-unresolved-imports) |
| E0631 | type mismatch in function arguments | A function item was passed to a higher-order call; no deref coercion applies at a trait bound | Wrap the call in a closure, or insert `.map(String::as_str)` |
| E0275 | overflow evaluating the requirement `T: X` | The trait solver reached the recursion limit during type checking | A repeating type in the note means an infinite chain: fix the impl. Raise the limit only for a finite, deep type |
| (none) | reached the recursion limit while instantiating | A type chain at monomorphization, sometimes `&mut &mut ... W` from a recursive call on a by-value `impl Write` | Reproduce with the original build command. For that repeating `Write` chain, a stable `&mut dyn Write` boundary can stop type growth; establish the chain before changing the API |
| E0038 | the trait `T` is not dyn compatible | One item of the trait gets no vtable slot | Read the `...because` note. Read [references/dyn-compatibility.md](references/dyn-compatibility.md) when you choose the fix; it maps each note to its fix |
| E0562 | `impl Trait` is not allowed in the return type of `Fn` trait bounds | `impl FnMut(&T) -> impl Ord` puts `impl Trait` in an associated-type binding | Name a generic parameter instead: `<K: Ord>` |
| E0184 | the trait `Copy` cannot be implemented for this type; the type has a destructor | One type asks for both `Copy` and `Drop` | Remove the `Copy` derive; a resource handle is not `Copy` |
| E0367 | `Drop` impl requires `T: Clone` but the struct it is implemented for does not | The `Drop` impl added a bound the type definition does not carry | Move the bound onto the struct definition |
| E0740 | field must implement `Copy` or be wrapped in `ManuallyDrop<...>` | A union field has a type with drop obligations | Reassess the representation with [implement-rust](../../implementation/implement-rust/SKILL.md). Do not add a manual unsafe cleanup path to pure core to silence this error |
| E0432 | unresolved import `x` | A feature is off, a path is wrong, or the crate is missing or does not exist | See [Unresolved imports](#e0432-and-e0433-unresolved-imports) |
| E0433 | cannot find type `T` in this scope; cannot find module or crate `x` in this scope. Before 1.95: failed to resolve: use of undeclared type `T`, or use of unresolved module or unlinked crate `x` | A missing `use`, often from `std`; a module path that needs `crate::` or `super::`; or a missing crate | See [Unresolved imports](#e0432-and-e0433-unresolved-imports) |
| E0425 | cannot find value, type, or function `x` in this scope (E0412 for a type on older toolchains) | Typo, a missing import, or an item behind an off feature | Read the `note:` and `help:`, see [Unresolved imports](#e0432-and-e0433-unresolved-imports) |
| E0603 | `x` is private | The path exists but is not exported | `pub use` it, or use the public path |
| E0658 | use of unstable library feature `f` | The API is unstable, or stable only in a newer Rust than this toolchain | Compare `rustc -V` and the crate's `rust-version` with the version on the API's docs page |
| E0072 | recursive type has infinite size | A type contains itself by value | `Box`, `Rc`, or `Arc` the recursive field; `Weak` for a back edge |
| E0793 | reference to field of packed struct is unaligned | A reference into `#[repr(packed)]` | Copy a `Copy` field by value without creating a reference, or use a safe byte representation. Native raw-pointer alternatives require the existing adapter safety review in [implement-rust](../../implementation/implement-rust/SKILL.md) |

## The clone reflex

Cloning before move can avoid E0382 only when value implements `Clone` and new ownership fits operation. Calling `clone` after move does not recover value. Determine who needs value and whether clone duplicates storage or shares state; `Arc` clone shares allocation. Size alone does not settle correctness.

| The value is | The answer |
| --- | --- |
| Small and `Copy`-like, and the copy is the point | Clone when that is the intended operation; derive `Copy` only when the type's fields and ownership semantics permit it |
| Read by several places, never written | Borrow `&T` within the owner's lifetime, including scoped threads; consider `Arc<T>` when shared ownership must outlive that scope and `T` has the required auto traits |
| Written by several places | Identify the authoritative mutation owner first. Use shared synchronization only when the invariant needs it; then check transaction scope, ordering and lock order |
| An identity: a connection, a file, a job | One owner. Pass `&mut` down, or pass a handle |
| Large and consumed once | Move it, and restructure the caller so it can be moved |

Loop clone may allocate, increment reference count, copy inline data or run custom code. Check concrete `Clone` implementation and ownership before estimating cost; measure only when performance belongs to task.

## Borrow conflicts are usually a split problem

E0499/E0502 rarely need interior mutability. Show compiler disjoint borrows: `split_at_mut` or `get_disjoint_mut` for indices, field borrows instead of `&self` helper, or value copy before write. Consider `RefCell` only after splits fail (see hazard table).

Read [references/borrow-checker-fixes.md](references/borrow-checker-fixes.md) when triage's first move does not fit E0499, E0502, E0506, E0507, E0716, E0373, E0521 or E0106, or to compare interior mutability costs.

## E0282, E0283, E0284, and the never type need a type anchor

Available constraints select no single type. Rust does not infer every method/operator input backward from final result type. Add smallest local anchor:

```rust
let parsed: u64 = "42".parse()?;
let bytes = Vec::<u8>::new();
let converted = u64::from(7_u8);
```

Prefer typed local, turbofish on constructor/method owning unknown type, or fully qualified call. Do not change public return type, add `'static` or broad trait bound merely to silence inference. Rebuild after one anchor; later diagnostics may cascade from first unknown type.

Unconstrained generic statement call, e.g. `f()?;`, has no anchor. Edition 2024 falls back to `!` instead of `()`; error names never type:

```rust,compile_fail,E0277
fn parse_or_default<T: Default>() -> Result<T, ()> { Ok(T::default()) }

fn load() -> Result<(), ()> {
    parse_or_default()?; // the trait bound `!: Default` is not satisfied
    Ok(())
}
```

Note: "this error might have been caused by changes to Rust's type-inference algorithm". On edition 2021, same code fails with deny-by-default lint `dependency_on_unit_never_type_fallback` (1.92+): "this function depends on never type fallback being `()`". Both fixes: name type:

```rust
fn parse_or_default<T: Default>() -> Result<T, ()> { Ok(T::default()) }

fn load() -> Result<(), ()> {
    parse_or_default::<()>()?;
    Ok(())
}
```

`let () = parse_or_default()?;` also works. Do not allow the lint; it is a hard error on 2024.

## E0432 and E0433: unresolved imports

Read `note:` and `help:` before adding dependency. `cargo add` line in `help:` is not fix (see hazard table). Stop at first matching step:

1. `note: found an item that was configured out` with "the item is gated behind the `f`
   feature". The feature is off. Read the file path under the note.
   - The item is in a dependency: establish whether `f` belongs to the approved feature lane.
     If it does, restore that selection. A new dependency feature is a dependency-policy change,
     not an automatic response to the diagnostic.
   - The item is in this crate: the caller lacks the item's gate. Put the same
     `#[cfg(feature = "f")]` on the caller, or build with `--features f`. Do not add `f` to
     `default` to silence the error; that hides the break in the build without the feature.
2. `help:` offers a `use` path ("consider importing") or says "a similar path exists". This
   covers `cannot find type` and `cannot find value` (E0425 or E0433) and E0432. Take that path:
   `std::...`, `crate::...`, or `super::...`. Check `std` and this crate before any new crate.
3. The first path segment is a module of this crate: `grep -rn 'mod <segment>' src` finds it.
   Write the path from `crate::` or `super::`.
4. The first path segment is a crate that `Cargo.toml` already lists. The item path is wrong, or
   the item exists only in another version. Read the source of the version in `Cargo.lock`, not
   the documentation of the latest release. Use the package name as `Cargo.toml` writes it: the
   path segment `tokio_util` normally corresponds to package `tokio-util`, but a dependency's
   explicit rename in `Cargo.toml` controls the path used by code.

   ```bash
   cargo metadata --format-version 1 --locked \
     | jq -r '.packages[] | select(.name == "<package>") | "\(.version) \(.manifest_path)"'
   ```

   If `jq` is not installed, `cargo tree -i <package> --locked --depth 0` prints the locked
   version.
5. The crate is not in `Cargo.toml`. Add a crate only when the code was written against that
   crate, or the task asks for a new dependency. Otherwise use `std` or an existing dependency.
   Run `cargo info <name>`: it prints the description, version, `rust-version`, repository, and
   features of that exact package, or "could not find". It proves that the name exists, not that
   it is the crate the code expects. Compare its `repository` and `description` with that crate.
   If they do not match, or the name came only from rustc's `help:` line, keep the missing
   dependency as an unresolved hypothesis. Route any actual dependency change through
   [dependency-updates](../../environment/dependency-updates/SKILL.md) within the authorized
   task; this reference does not authorize `cargo add`.

## A `Drop` impl changes the borrow checker

`impl Drop` is not local change. Can break previously compiling code with E0597, E0502, E0509 or E0184; no error title names `Drop`: drop point adds use of every held borrow. Note: "... when `_g` is dropped and runs the `Drop` code for type `Guard`". Drop guard before read, scope in inner block or keep type `Drop`-free. Read `Drop` section of [references/borrow-checker-fixes.md](references/borrow-checker-fixes.md) after new `impl Drop` errors, or E0507 inside `drop(&mut self)`.

## Optional companion references

- [rust-borrow-semantics](../rust-borrow-semantics/SKILL.md): temporary scopes, lifetime
  extension, two-phase borrows and edition-specific drop order.
- [rust-send-sync](../rust-send-sync/SKILL.md): field-level `Send`/`Sync` obligations.

Use companion only when mechanism matters. Otherwise inspect actual locked source and `rustc --explain`, with [implement-rust](../../implementation/implement-rust/SKILL.md) as project owner. Missing specialist does not require installation.

Adapted from po4yka/rust-skills; see [LICENSE](LICENSE) and
[local changes and source pin](LOCAL-CHANGES.md).
