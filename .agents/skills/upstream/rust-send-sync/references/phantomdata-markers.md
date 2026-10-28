# `PhantomData` markers for `Send` and `Sync`

`PhantomData<X>` inherits `X`'s auto traits exactly; any std type with wanted implementation works as marker. Remove only intended traits:

| Marker field | `Send` | `Sync` | Variance in `T` if the marker names your own `T` |
| --- | --- | --- | --- |
| `PhantomData<fn() -> T>` | kept | kept | covariant |
| `PhantomData<Cell<T>>` | when `T: Send` | removed | invariant |
| `PhantomData<MutexGuard<'static, T>>` | removed | when `T: Sync` | invariant, and it forces `T: 'static` |
| `PhantomData<*const T>` | removed | removed | covariant |
| `PhantomData<*mut T>` | removed | removed | invariant |

Marker's own bounds only. Other fields can remove either auto trait; marker cannot make non-`Send` or non-`Sync` payload satisfy trait. Use `()` when only removing trait; verify complete type at actual caller boundary.

```rust
use std::cell::Cell;
use std::marker::PhantomData;
use std::sync::MutexGuard;

struct SendNotSync(PhantomData<Cell<()>>);                 // moves threads, does not share
struct SyncNotSend(PhantomData<MutexGuard<'static, ()>>);  // shares, stays on its thread
struct NeitherOne(PhantomData<*const ()>);                 // both removed
struct BothKept<T>(PhantomData<fn() -> T>);                // both kept for every T

fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}
fn main() {
    assert_send::<SendNotSync>();
    assert_sync::<SyncNotSend>();
    assert_send::<BothKept<std::rc::Rc<i32>>>();
    assert_sync::<BothKept<std::rc::Rc<i32>>>();
    let _ = NeitherOne(PhantomData);   // constructs, but neither Send nor Sync
}
```

Rules:

- Use `PhantomData<*mut ()>` only for "neither". Wrong reflex for only "not shareable": also blocks move to worker thread; diagnostic names `*mut ()`, invisible in struct.
- Write `()` when marker only removes auto trait. Naming own parameter also constrains variance: `PhantomData<Cell<T>>` makes struct invariant in `T`, rejecting apparently safe caller substitutions. `rust-variance` covers this.
- `PhantomData<MutexGuard<'static, T>>` goes further than the other four markers. `MutexGuard<'a,
  T>` is declared `T: ?Sized + 'a`, so a naked `T` in this marker forces `T: 'static` on the
  struct, and rustc rejects the definition with `error[E0310]`. Write `MutexGuard<'static, ()>`
  for the auto trait effect, and name `T` in a second marker.
