---
name: "rust-send-sync"
description: "Diagnose Rust Send/Sync obligations at an actual thread or task boundary, including lock payload bounds, PhantomData markers and public auto-trait guarantees."
license: "BSD-3-Clause"
---

# Rust Send and Sync

## NevroTick scope and precedence

Repository instructions and [canonical architecture](../../../../docs/windows-time-architecture.md) control ownership, privileges, runtime and dependencies. Use [implement-rust](../../implementation/implement-rust/SKILL.md) for changed owner/callers; this skill supplies auto-trait mechanics. Identify API requiring `Send` or `Sync` before changing type. Current-thread engine experiment permits local `!Send` futures where actual APIs allow; `tokio::spawn` still requires `Send` there. Do not add runtime, move work between owners, replace `Arc` with `Rc`, add locking or write `unsafe impl` merely to remove diagnostic. Windows thread/apartment and handle contracts remain separate obligations.

Upstream examples report rustc 1.98.1, edition 2024, aarch64-apple-darwin: historical illustrations, not local probes or qualified Windows/compiler support. Check selected compiler, target, features and actual consumer before relying on diagnostic/bound.

Use [QA contract](../../testing/CONTRACT.md) and [contract tests](../../testing/qa-contract-tests/SKILL.md) for positive/compile-fail probes: include valid control; confirm rejection comes from intended auto-trait obligation, not import, fixture or unrelated error. Select lints/optional tools through [static checks](../../testing/qa-static-checks/SKILL.md); this skill installs no tools, changes no dependencies, globally enables no `future_not_send`.

Related upstream skills optional. If absent, use `implement-rust`, QA references above and relevant Rust/Tokio documentation; installation not prerequisite. Local adaptations/sources: [LOCAL-CHANGES.md](LOCAL-CHANGES.md).

## Route the symptom to a section

| Symptom or task | Section |
| --- | --- |
| `error[E0277]: X cannot be sent between threads safely` with `help: the trait Sync is not implemented` | [The one rule](#the-one-rule-t-is-send-exactly-when-t-is-sync) |
| `error[E0277]: X cannot be shared between threads safely` | [The auto trait table](#the-auto-trait-table) |
| `error[E0321]: cross-crate traits with a default impl, like Send` | [E0321](#unsafe-impl-send-cannot-target-a-reference-e0321) |
| A `Mutex` was changed to an `RwLock` and the build broke | [Lock payload bounds](#lock-payload-bounds-mutex-and-rwlock-are-not-interchangeable) |
| `MutexGuard<'_, T> cannot be sent between threads safely` | [Guards](#guards-mutexguard-is-not-send-and-it-is-sync) |
| You want a type that moves between threads but is not shareable | [`PhantomData` surgery](#phantomdata-surgery) |
| An unrelated call site broke after you edited a function body | [Auto traits leak](#auto-traits-leak-out-of-impl-trait-and-async-fn) |
| `error: future cannot be sent between threads safely` | [Auto traits leak](#auto-traits-leak-out-of-impl-trait-and-async-fn) |
| ``warning: use of `async fn` in public traits is discouraged`` | the `rust-async-internals` skill, when it is installed |
| A public type must keep or lose `Send` or `Sync` on purpose | [Verify](#verify-the-auto-traits) |
| You are about to write `unsafe impl Send for MyType {}` | the `rust-unsafe` skill, when it is installed |

## The one rule: `&T` is `Send` exactly when `T` is `Sync`

`Sync` states that type-level fact. `Send` error whose `help:` names `Sync` asks for no `Send` impl: shared reference points to unshareable value.

Both auto traits derive field-wise. Fix bound errors in fields, not with impl. Two helpers expose readable compile errors:

```rust
fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

fn main() {
    assert_send::<&i32>();                  // i32 is Sync, so &i32 is Send
    assert_sync::<&i32>();                  // &T is Sync exactly when T is Sync
    assert_send::<std::cell::Cell<i32>>();  // Cell is Send: a single owner moves it
}
```

`Cell<i32>` is `Send`. A shared reference to it is not:

```rust,compile_fail,E0277
use std::cell::Cell;
fn assert_send<T: Send>() {}
fn main() { assert_send::<&Cell<i32>>(); }
```

```text
error[E0277]: `&Cell<i32>` cannot be sent between threads safely
3 | fn main() { assert_send::<&Cell<i32>>(); }
  |                           ^^^^^^^^^^ `&Cell<i32>` cannot be sent between threads safely
  |
  = help: the trait `Sync` is not implemented for `Cell<i32>`
  = note: required for `&Cell<i32>` to implement `Send`
```

Read the two lines together. The head trait is `Send`, because that is the bound that failed. The
unmet obligation is `Sync`, on the pointee. The failing impl is `impl<T: Sync + ?Sized> Send for
&T`, and you cannot write a different one (see [E0321](#unsafe-impl-send-cannot-target-a-reference-e0321)).

Repair in order: remove sharing; move value instead of reference. Or give pointee `Sync` interior mutability: atomic, `Mutex` or `RwLock`. Only then read `rust-unsafe`, only for owned type whose invariant makes impl sound.

Never add `unsafe impl Send` or `unsafe impl Sync` merely to remove diagnostic, including wrappers around `Rc`, `Cell` or guard. Compiler accepts impl whether sound or not. Fix field, lock or signature.

Messages differ: `cannot be sent` means failed `Send` bound; `cannot be shared` means failed `Sync` bound. Second appears under `Arc` because `Arc<T>: Send` requires `T: Sync`.

## Verify the auto traits

Compiler proves only field-derived auto traits. After manual `unsafe impl`, no check of raw pointer field's pointee; green build proves nothing about impl.

| Claim | Check |
| --- | --- |
| A public type stays `Send` or `Sync` | A `const` assertion next to the type (below). A field edit then fails at the definition, not at a distant caller. Across releases, `cargo semver-checks` reports `auto_trait_impl_removed` |
| A public `-> impl Trait` return stays `Send` | Write `+ Send` (and `+ Sync` when callers share it) in the signature. The body then fails at the definition. See [Auto traits leak](#auto-traits-leak-out-of-impl-trait-and-async-fn) |
| A type is deliberately not `Send` | A `compile_fail` doctest that defines `assert_send` and calls `assert_send::<my_crate::Type>()`. A doctest is a separate crate, so `crate::Type` fails for the wrong reason. The doctest passes on any error, so keep only that call in it. Stable rustdoc does not check an error code after `compile_fail` |
| An `Arc` payload is shareable | `clippy::arc_with_non_send_sync`, warn by default |
| No `std::sync` guard lives across an `.await` | `clippy::await_holding_lock`, warn by default |
| A future used at a `Send` boundary satisfies that bound | Assert the bound at the real consumer. Optional `clippy::future_not_send` can help diagnose selected code; it is not a project-wide requirement |

Select existing Clippy target/feature lane when bounds change. `cargo clippy --locked --all-targets -- -D warnings` is example once Cargo project/lockfile exist, not proof of Windows-only consumer checks. `future_not_send` is allow-by-default; may report intentionally local futures. Do not enable globally or add blanket expectations for one-boundary diagnosis. Generic limitations mean lint success does not prove every instantiation is `Send`. Use actual caller and focused positive/negative probes.

```rust
pub struct Session { pub id: u32, pub buf: Vec<u8> }

const _: () = {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Session>();
};
```

## The auto trait table

| Type | `Send` when | `Sync` when |
| --- | --- | --- |
| `T` (your struct) | every field is `Send` | every field is `Sync` |
| `&T` | `T: Sync` | `T: Sync` |
| `&mut T` | `T: Send` | `T: Sync` |
| `Box<T>` | `T: Send` | `T: Sync` |
| `Arc<T>` | `T: Send + Sync` | `T: Send + Sync` |
| `Rc<T>` | never | never |
| `*const T`, `*mut T` | never | never |
| `Cell<T>`, `RefCell<T>` | `T: Send` | never |
| `Mutex<T>`, `RwLock<T>` | `T: Send` | see [lock bounds](#lock-payload-bounds-mutex-and-rwlock-are-not-interchangeable) |
| `MutexGuard<'_, T>`, `RwLockReadGuard`, `RwLockWriteGuard` | never | `T: Sync` |

Two surprising rows. **`Arc<T>` needs both payload traits, not only `Send`.** `Arc` clone gives second owner shared access to one value: payload must be shareable and movable:

```rust,compile_fail,E0277
use std::cell::Cell;
use std::sync::Arc;
fn assert_send<T: Send>() {}
fn main() {
    assert_send::<Cell<i32>>();        // ok on its own
    assert_send::<Arc<Cell<i32>>>();   // E0277: Arc<T>: Send needs T: Send + Sync
}
```

```text
error[E0277]: `Cell<i32>` cannot be shared between threads safely
  = help: the trait `Sync` is not implemented for `Cell<i32>`
  = note: if you want to do aliasing and mutation between multiple threads, use
          `std::sync::RwLock` or `std::sync::atomic::AtomicI32` instead
  = note: required for `Arc<Cell<i32>>` to implement `Send`
```

Put the lock inside the `Arc`. `Arc<Mutex<Cell<i32>>>` is
both `Send` and `Sync`, because `Mutex<T>: Sync` needs only `T: Send`. The `note:` means
"replace the `Cell`": `Arc<AtomicI32>` and `Arc<RwLock<i32>>` both work. Do not wrap the `Cell`
in the suggested `RwLock`: `RwLock<Cell<i32>>` fails again (see
[lock bounds](#lock-payload-bounds-mutex-and-rwlock-are-not-interchangeable)).

`clippy::arc_with_non_send_sync` (warn by default) reports `Arc::new` of payload lacking `Send + Sync` at construction, before thread boundary. `Rc` permitted only by complete owner/caller contract, including task bounds; single-thread execution alone does not justify replacing existing `Arc`.

**`&mut T` is not the mirror of `&T`.** `&mut T: Send` requires `T: Send`, not `T: Sync`:

```rust,compile_fail,E0277
use std::cell::Cell;
use std::rc::Rc;
fn assert_send<T: Send>() {}
fn main() {
    assert_send::<&mut Cell<i32>>();  // ok: Cell<i32> is Send, although it is not Sync
    assert_send::<&mut Rc<i32>>();    // E0277: &mut T: Send needs T: Send
}
```

```text
error[E0277]: `Rc<i32>` cannot be sent between threads safely
  = help: within `&mut Rc<i32>`, the trait `Send` is not implemented for `Rc<i32>`
  = note: required because it appears within the type `&mut Rc<i32>`
```

`&mut T` is move channel, not view. `std::mem::replace(x, T::default())` removes value through reference; receiving thread owns/drops it. Hence `Send` obligation.

## `unsafe impl Send` cannot target a reference (E0321)

Cannot choose `Send` per reference type; impl forbidden:

```rust,compile_fail,E0321
pub struct Handle(*mut u8);
unsafe impl Send for &Handle {}
unsafe impl Send for &mut Handle {}
```

```text
error[E0321]: cross-crate traits with a default impl, like `Send`, can only be implemented
              for a struct/enum type, not `&Handle`
2 | unsafe impl Send for &Handle {}
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can't implement cross-crate trait with a default impl
  |                              for non-struct/enum type
```

`unsafe impl Send for Handle {}` compiles: `Handle` is local struct. Only `Sync` reaches `&Handle`: `unsafe impl Sync for Handle {}` makes `&Handle` become `Send` through blanket impl. Stronger promise; `rust-unsafe` supplies required audit.

## Lock payload bounds: `Mutex` and `RwLock` are not interchangeable

- `Mutex<T>: Sync` requires only `T: Send`.
- `RwLock<T>: Sync` requires `T: Send + Sync`.

`Mutex` gives one `&mut T` at a time: payload need not be shareable. `RwLock` gives many concurrent `&T`: payload itself must be `Sync`.

```rust
use std::cell::Cell;
use std::sync::Mutex;
fn assert_sync<T: Sync>() {}
fn main() { assert_sync::<Mutex<Cell<i32>>>(); }   // compiles
```

```rust,compile_fail,E0277
use std::cell::Cell;
use std::sync::RwLock;
fn assert_sync<T: Sync>() {}
fn main() { assert_sync::<RwLock<Cell<i32>>>(); }
```

```text
error[E0277]: `Cell<i32>` cannot be shared between threads safely
  = help: the trait `Sync` is not implemented for `Cell<i32>`
  = note: required for `std::sync::RwLock<Cell<i32>>` to implement `Sync`
```

Read-heavy `Mutex<T>` → `RwLock<T>` is **not** drop-in change. Payload holding `Cell` or `RefCell` stops compilation; error appears on distant `Arc<RwLock<...>>` caller. Check payload first. `Rc` payload is not swap symptom: `Mutex<T>: Sync` also requires `T: Send`, so `Mutex<Rc<T>>` was already neither `Send` nor `Sync`. `Mutex<T>: Send` and `RwLock<T>: Send` both need only `T: Send`; `Send` direction gives no warning.

## Guards: `MutexGuard` is not `Send`, and it is `Sync`

`std::sync::MutexGuard<'_, T>` is `!Send` for every `T`. POSIX requires locking/unlocking on same thread; guard's `Drop` must run on creation thread.

```rust,compile_fail,E0277
use std::sync::MutexGuard;
fn assert_send<T: Send>() {}
fn main() { assert_send::<MutexGuard<'static, i32>>(); }
```

```text
error[E0277]: `std::sync::MutexGuard<'static, i32>` cannot be sent between threads safely
  = help: the trait `Send` is not implemented for `std::sync::MutexGuard<'static, i32>`
```

This does not mean "nothing derived from guard leaves thread". Guard is `Sync` whenever `T: Sync`; `&MutexGuard<'_, T>` **is** `Send`. Reading through guard reads `&T`, safe to share. Runs and prints `total=6`:

```rust,run
use std::sync::Mutex;
use std::thread;

fn main() {
    let lock = Mutex::new(vec![1u32, 2, 3]);
    let guard = lock.lock().expect("lock poisoned");
    let total: u32 = thread::scope(|s| {
        // &MutexGuard is Send, because MutexGuard is Sync.
        s.spawn(|| guard.iter().sum::<u32>()).join().unwrap()
    });
    assert_eq!(total, 6);
    println!("total={total}");
    drop(guard);   // the unlock runs on the locking thread
}
```

Consequences:

- Give a worker `&guard` only when the payload is `Sync`. `&MutexGuard<'_, T>` is `Send` exactly
  when `T: Sync`, so `&MutexGuard<'_, Cell<T>>` and `&MutexGuard<'_, RefCell<T>>` do not cross.
  Clone the value out of the guard instead. Never give a worker the guard itself.
- Pass `&guard` through `thread::scope`. `thread::spawn` requires `'static`, so it rejects the
  borrow with ``error[E0597]: `lock` does not live long enough``, plus `error[E0373]` when the
  closure borrows `guard` directly. Do not apply the E0373 `move` suggestion: it moves the guard
  itself, and that fails with E0277.
- Do not hold a `std::sync` guard across an `.await`. The task can resume on another thread, so
  the future stops being `Send`. `clippy::await_holding_lock` (warn by default) catches it. The
  `rust-async-internals` skill covers the async lock choice.

## `PhantomData` surgery

`PhantomData<X>` inherits `X` auto traits. Use `PhantomData<Cell<()>>` to remove only `Sync`. Raw pointer marker, e.g. `PhantomData<*mut ()>`, removes both, blocking move to worker thread. Write `()` in marker, not own `T`: naming `T` also changes variance. Read [references/phantomdata-markers.md](references/phantomdata-markers.md) when designing marker: variance table and marker removing only `Send`.

## Auto traits leak out of `impl Trait` and `async fn`

Public `-> impl Trait` without explicit `+ Send` publishes **body**'s auto traits. Adding one `Rc` inside function then breaks callers; compiler reports each distant call site with `note:` pointing into private body:

```text
error[E0277]: `Rc<Vec<u32>>` cannot be sent between threads safely
10 |     std::thread::spawn(move || { let _: Vec<u32> = it.collect(); });
   |                        ^^^^^^^ within this `{closure@...}`
note: required because it appears within the type `impl Iterator<Item = u32>`
```

State bound in signature: freezes guarantee; body must comply:

```rust
pub fn ids() -> impl Iterator<Item = u32> + Send {
    (0..3).map(|i| i * 2)
}
fn main() { assert_eq!(ids().sum::<u32>(), 6); }
```

With `+ Send`, offending body fails at definition:

```rust,compile_fail,E0277
use std::rc::Rc;
pub fn ids() -> impl Iterator<Item = u32> + Send {
    let names: Rc<Vec<u32>> = Rc::new(vec![1, 2, 3]);
    (0..3).map(move |i| names[i])
}
```

```text
error[E0277]: `Rc<Vec<u32>>` cannot be sent between threads safely
3 | pub fn ids() -> impl Iterator<Item = u32> + Send {
  |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `Rc<Vec<u32>>` cannot be sent ...
5 |     (0..3).map(move |i| names[i])
  |     ----------------------------- return type was inferred to be `Map<...>` here
```

Write `+ Send` only when intended; add `+ Sync` when callers share returned value. Single-threaded-by-design API cannot carry bound: fails at definition. Record decision in doc comment.

Not async-specific. `async fn f() -> T` desugars to `fn f() -> impl Future<Output = T>`; same leak. Stable Rust 1.98.1 has no syntax for bound on `async fn` return type. Write desugared form:

```rust
use std::future::Future;

pub fn load(id: u32) -> impl Future<Output = u32> + Send {
    async move { id + 1 }
}

fn main() { let _ = load(1); }
```

Do not silence `async_fn_in_trait` warning when callers spawn future. For spawned trait `async fn` and return type notation, use `rust-async-internals` when installed.

Async diagnostic has **no error code**; E0277 search finds nothing:

```text
error: future cannot be sent between threads safely
  |                         ^^^^ future created by async block is not `Send`
note: future is not `Send` as this value is used across an await
5 |         let names = Rc::new(vec![1u32, 2, 3]);
  |             ----- has type `Rc<Vec<u32>>` which is not `Send`
6 |         std::future::ready(()).await;
  |                                ^^^^^ await occurs here, with `names` maybe used later
```

Read `note:`: names value and exact `.await` trapping it. `rust-compiler-errors` covers message shape when installed. Choose repair from real boundary/intended ownership contract; signature change is one option.

## Related skills

Use named skills when installed; otherwise NevroTick fallbacks above. Do not stop or install skills merely because table names them.

| Skill | Boundary |
| --- | --- |
| `rust-unsafe` | The proof obligation of a manual `unsafe impl Send` or `Sync`: the field audit, the compile-time field assertions, and the `SAFETY` comment |
| `rust-variance` | The other half of a `PhantomData` marker: which substitutions the marker still allows, and the invariance a `Cell` or `*mut` marker introduces |
| `rust-compiler-errors` | Reading E0277 in general, and the async block message that carries no error code |
| `rust-async-internals` | Holding a guard across an `.await`, the async lock choice, `Send` trait variants, and cancel safety |
| `memory-model` | Atomics, orderings, `loom`, and shared statics once the bounds are satisfied |
| `rust-crate-release` | Semver review of a public auto trait change |
