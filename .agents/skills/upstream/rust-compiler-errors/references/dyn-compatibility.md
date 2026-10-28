# E0038: the trait is not dyn compatible

Read for E0038 or `where Self: Sized` item failing through `dyn Trait`. Triage table: [SKILL.md](../SKILL.md).

`dyn Trait` needs vtable. One item lacking vtable slot removes whole trait from `dyn` use; error points at `dyn Trait` type, not broken call:

```text
error[E0038]: the trait `NoSelf` is not dyn compatible
note: for a trait to be dyn compatible it needs to allow building a vtable
   |     fn describe() -> String;
   |        ^^^^^^^^ ...because associated function `describe` has no `self` parameter
```

Read `...because` first. rustc prints one note per shape; eleven common shapes below.

| Shape in the trait | The `...because` note |
| --- | --- |
| `fn describe() -> String;` | ...because associated function `describe` has no `self` parameter |
| `fn go<T: Copy>(&self, t: T);` | ...because method `go` has generic type parameters |
| `fn ser(&self, out: impl Write);` | ...because method `ser` has generic type parameters |
| `fn dup(&self) -> Self;` | ...because method `dup` references the `Self` type in its return type |
| `fn eq_me(&self, other: &Self) -> bool;` | ...because method `eq_me` references the `Self` type in this parameter |
| `fn it(&self) -> impl Iterator<Item = u8>;` | ...because method `it` references an `impl Trait` type in its return type |
| `async fn m(&self) -> u32;` | ...because method `m` is `async` |
| `const N: usize;` | ...because it contains associated const `N` |
| `type Item<T>;` | ...because it contains generic associated type `Item` |
| `trait T: Clone` or `trait T: Sized` | ...because it requires `Self: Sized` |
| `trait T: PartialEq<Self>` | ...because it uses `Self` as a type parameter |

Rows two/three print same note despite unlike signatures. Argument-position `impl Trait` hides generic parameter: `fn ser(&self, out: impl Write)` is `fn ser<W: Write>(&self, out: W)`. Take `&mut dyn Write` when consumer may need `Box<dyn Trait>`.

Last four rows name no method, appearing different. `Clone` has `Sized` supertrait; `: Clone` alone removes vtable.

## Fix one item with `where Self: Sized`

First seven rows and generic associated type row affect one item; one clause there fixes it:

```rust
pub trait Codec {
    fn decode(&self, src: &[u8]) -> Vec<u8>;   // keeps its vtable slot
    fn name() -> &'static str
    where
        Self: Sized;                           // leaves the vtable
    type Frame<T>
    where
        Self: Sized;                           // a GAT leaves the check the same way
}
```

`Vec<Box<dyn Codec>>` now compiles; implementors still call `name()` through concrete type.

GAT clause needs Rust 1.72 or later. Older MSRV: GAT makes trait not dyn compatible even with clause.

Cost: item leaves trait-object API. Later `dyn` call fails at call site, not trait definition; message never mentions dyn compatibility:

```text
error[E0277]: the size for values of type `dyn Codec` cannot be known at compilation time
note: required by a bound in `Codec::name`
```

Add `where Self: Sized` only when no caller needs item through `dyn`. Otherwise move item to second trait, or take `&self` and return owned type instead of `Self`.

## The three rows the clause does not fix

`where Self: Sized` does not fix associated const or two supertrait rows. Supertrait bound is not item; associated const rejects clause: `error[E0658]: generic const items are experimental`. Move associated const to second trait. Drop `Clone` supertrait; put clone in vtable:

```rust
trait Shape {
    fn area(&self) -> f64;
    fn clone_box(&self) -> Box<dyn Shape>;
}

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
```

## Search terms

rustc renamed "object safety" to "dyn compatibility". Current build-log grep for "object safe" finds nothing; grep `E0038` or "dyn compatible".
