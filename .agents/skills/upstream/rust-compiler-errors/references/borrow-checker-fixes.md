# Borrow checker fix catalogue

Worked fixes for [SKILL.md](../SKILL.md) errors, authored upstream for rustc 1.98.1. Check relevant probe on NevroTick's selected edition, toolchain and MSRV; upstream baseline is not local execution evidence. Each entry states ownership tradeoff. Read only matching section; preserve approved owner/cleanup order.

- [Disjoint access to one collection](#disjoint-access-to-one-collection): E0499 on two indices
- [A read borrow that blocks a write](#a-read-borrow-that-blocks-a-write): E0502
- [A `&self` helper that blocks a field write](#a-self-helper-that-blocks-a-field-write): E0506
- [E0507: an owned value out of a reference](#e0507-an-owned-value-out-of-a-reference)
- [E0716: a temporary ended before its borrow](#e0716-a-temporary-ended-before-its-borrow)
- [A borrow that must cross a thread](#a-borrow-that-must-cross-a-thread): E0373, E0521
- [Interior mutability, and its cost](#interior-mutability-and-its-cost)
- [E0106 and lifetime annotation shapes](#e0106-and-lifetime-annotation-shapes)
- [Drop and the borrow checker](#drop-and-the-borrow-checker): E0597, E0502, E0507, E0509,
  E0184, E0367
- [E0631: a function item is not a coercion site](#e0631-a-function-item-is-not-a-coercion-site)

## Disjoint access to one collection

Compiler tracks borrows per place, not per element. Cannot prove `v[0]` and `v[1]` disjoint: two `&mut` into one `Vec` rejected even with different indices.

```rust,compile_fail,E0499
fn bump(v: &mut Vec<i32>) {
    let a = &mut v[0];
    let b = &mut v[1];
    *a += *b;
}
```

```rust
// Split at an index. The two halves are disjoint by construction.
pub fn add_first_to_second(v: &mut [i32]) {
    let (left, right) = v.split_at_mut(1);
    left[0] += right[0];
}

// Several disjoint indices at once. Returns Err(GetDisjointMutError) on a
// duplicate or out-of-range index.
pub fn swap_three(v: &mut [i32]) -> Option<()> {
    let [a, b, c] = v.get_disjoint_mut([0, 2, 4]).ok()?;
    *a += *b + *c;
    Some(())
}

// Iterate and mutate every element. No index, no bounds check.
pub fn double_all(v: &mut [i32]) {
    for slot in v.iter_mut() {
        *slot *= 2;
    }
}

// Two named fields of one struct. Field borrows are already disjoint.
pub struct Pair {
    pub left: Vec<i32>,
    pub right: Vec<i32>,
}

pub fn drain_into(pair: &mut Pair) {
    // Borrowing two distinct fields of the same struct is allowed.
    pair.left.append(&mut pair.right);
}
```

`get_disjoint_mut` stable since Rust 1.86. Earlier: `split_at_mut` twice or index-and-copy pass.

## A read borrow that blocks a write

```rust,compile_fail,E0502
fn push_first(v: &mut Vec<i32>) {
    let first = &v[0];
    v.push(1);
    println!("{first}");
}
```

```rust
// Fix 1: copy the value out. The borrow ends at the semicolon.
pub fn copy_out(v: &mut Vec<i32>) {
    let first = v[0];
    v.push(first);
}

// Fix 2: scope the borrow, when the value is not Copy.
pub fn scoped(v: &mut Vec<String>) {
    let first = {
        let borrowed = &v[0];
        borrowed.len()
    };
    v.push(first.to_string());
}

// Fix 3: collect the decisions first, then apply them.
pub fn retain_matching(v: &mut Vec<String>, needle: &str) {
    let keep: Vec<bool> = v.iter().map(|s| s.contains(needle)).collect();
    let mut index = 0;
    v.retain(|_| {
        let decision = keep[index];
        index += 1;
        decision
    });
}
```

Fix 3: compute plan under shared borrow, execute under exclusive borrow. General answer when read informs write. `retain` alone shorter when predicate needs no outside state.

## A `&self` helper that blocks a field write

`&self` helper borrows all `self`. While result lives, any field write yields E0506:

```rust,compile_fail,E0506
pub struct Cache {
    entries: Vec<String>,
    hits: usize,
}

impl Cache {
    pub fn get(&mut self, key: &str) -> Option<&String> {
        let found = self.lookup(key)?;
        self.hits += 1; // cannot assign to `self.hits` because it is borrowed
        Some(found)
    }

    fn lookup(&self, key: &str) -> Option<&String> {
        self.entries.iter().find(|e| *e == key)
    }
}
```

Work on fields, not `self`; distinct field borrows are disjoint:

```rust
pub struct Cache {
    entries: Vec<String>,
    hits: usize,
}

impl Cache {
    pub fn get(&mut self, key: &str) -> Option<&String> {
        let found = self.entries.iter().find(|e| *e == key)?;
        self.hits += 1;
        Some(found)
    }
}
```

Private `&self` helper forces whole-struct borrow. Free functions taking fields, or direct field access, keep borrows apart. Large struct with many `&mut self` methods fights borrow checker: every method borrows everything.

## E0507: an owned value out of a reference

```rust,compile_fail,E0507
struct S { name: String }
fn f(s: &S) -> String { s.name }
```

Choose original's postcondition:

| Intent | Call |
| --- | --- |
| The original keeps its value | `s.name.clone()` |
| The original is left empty and is still valid | `std::mem::take(&mut s.name)` |
| The original is left holding something else | `std::mem::replace(&mut s.name, other)` |
| The field is optional and becomes `None` | `s.name.take()` on an `Option` |
| The caller is finished with the whole value | Change the signature to take `self` |

`mem::take` needs `&mut` and `Default`; constructs `T::default()`. Shown `String` replacement is empty, no allocation; arbitrary `Default` implementations may allocate, panic or do other work. Choose required original-value postcondition.

```rust
#[derive(Default)]
pub struct Job {
    pub name: String,
    pub payload: Option<Vec<u8>>,
}

// Leaves an empty String behind. No allocation, no clone.
pub fn take_name(job: &mut Job) -> String {
    std::mem::take(&mut job.name)
}

// Leaves a chosen value behind, and returns the old one.
pub fn rename(job: &mut Job, next: String) -> String {
    std::mem::replace(&mut job.name, next)
}

// The Option case. Leaves None behind.
pub fn take_payload(job: &mut Job) -> Option<Vec<u8>> {
    job.payload.take()
}

// Swap two places without a temporary owner.
pub fn swap(a: &mut Job, b: &mut Job) {
    std::mem::swap(a, b);
}
```

`mem::replace` needs no `Default`; works without sensible empty value.

## E0716: a temporary ended before its borrow

```rust,compile_fail,E0716
fn foo() -> Vec<u8> { vec![1, 2, 3] }
fn bar(v: &Vec<u8>) -> &u8 { &v[0] }

// `foo()` produced a temporary with no name. It dies at the end of the statement.
let p = bar(&foo());
let q = *p;
```

Name temporary: extends to enclosing block's end:

```rust
fn foo() -> Vec<u8> { vec![1, 2, 3] }
fn bar(v: &Vec<u8>) -> &u8 { &v[0] }

let tmp = foo();
let p = bar(&tmp);
let q = *p;
```

Not every temporary dies at semicolon. `let r = &make();` extends temporary; so does `let r = Some(&make());` since Rust 1.89 (older MSRV: constructor form E0716). Function argument, e.g. `bar(&foo())`, never extends. Optional [rust-borrow-semantics](../../rust-borrow-semantics/SKILL.md) covers expression rules, edition-specific scopes and two-phase borrows. Project owner remains [implement-rust](../../../implementation/implement-rust/SKILL.md).

## A borrow that must cross a thread

`thread::spawn` requires `'static`; local borrows cannot enter.

```rust
use std::sync::Arc;

// Rejected: the closure outlives the borrow.
// pub fn f(data: &Vec<i32>) {
//     std::thread::spawn(move || println!("{data:?}"));
// }

// Fix 1: share ownership. One allocation, cheap clones.
pub fn shared(data: Arc<Vec<i32>>) {
    let handle = Arc::clone(&data);
    std::thread::spawn(move || println!("{handle:?}"));
}

// Fix 2: a scoped thread. The borrow is allowed because the scope joins
// every thread before it returns.
pub fn scoped(data: &Vec<i32>) {
    std::thread::scope(|s| {
        s.spawn(|| println!("{data:?}"));
    });
}
```

`thread::scope` stable since Rust 1.63. Prefer for bounded work when caller can wait. Use `Arc` when thread must outlive calling frame.

Borrowed parameter reaching `spawn`: E0521. Local borrowed by closure without `move`: E0373, "closure may outlive the current function, but it borrows `v`". Add `move` when thread may own value. First clone `Arc` into closure when caller still needs it.

## Interior mutability, and its cost

Use only after splits above fail.

| Type | Check | Failure mode | Use when |
| --- | --- | --- | --- |
| `Cell<T>` | no dynamic borrow check; `get` needs `T: Copy`, other methods have their own bounds | operation-specific; `swap` can panic for partially overlapping cells | Single-threaded interior mutation with an operation suited to `T` |
| `RefCell<T>` | run time | panics on conflict | A graph shape the compiler cannot verify, single thread |
| `Mutex<T>` | run time | blocks, or deadlocks | Shared write access across threads |
| `RwLock<T>` | run time | blocking, deadlock or starvation depending on access and platform scheduling | Shared access when the measured/read-write pattern and invariant justify it |
| `AtomicUsize` and friends | no dynamic borrow check | logical races; an invalid ordering argument can leave shared data unsynchronized | A counter or flag whose ordering and complete invariant are understood |

```rust
use std::cell::RefCell;

pub struct Graph {
    nodes: Vec<RefCell<Node>>,
}

pub struct Node {
    pub visited: bool,
}

impl Graph {
    // The borrow lives only inside this call, so the panic window is small.
    pub fn mark(&self, index: usize) {
        self.nodes[index].borrow_mut().visited = true;
    }
}
```

Keep every `borrow_mut` short; never across possibly reentrant call. `RefCell` panic: `RefCell already borrowed` from `borrow_mut`, or `RefCell already mutably borrowed` from `borrow`. Panic location names failed call; `RUST_BACKTRACE=1` shows path. Other live borrow unrecorded: shorten every `borrow` and `borrow_mut` guard potentially live there.

For thread-safe state and guard crossing `.await`, trace whole transaction/resource lifetime with [implement-rust](../../../implementation/implement-rust/SKILL.md). Container change alone establishes no ordering, cancellation or authority.

## E0106 and lifetime annotation shapes

```rust,compile_fail,E0106
struct S { name: &str }
```

Two answers; usually second is right:

```rust
struct Borrowed<'a> { name: &'a str }   // the struct cannot outlive the source
struct Owned { name: String }           // the struct owns its data
```

Store owned data unless short-lived view built/consumed within one function. Struct lifetime parameter spreads to every containing type and whole call graph. Pay cost for parser view/zero-copy frame, not config/message.

Returned reference: state source with narrowest true lifetime:

```rust
// One input, one output. The lifetime is inferred; do not write it.
pub fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

// Two inputs, one output. The compiler cannot guess, so state the source.
pub fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

// The output borrows from one input only. Say which, and free the other.
pub fn prefix_of<'a>(text: &'a str, _sep: &str) -> &'a str {
    text.split('=').next().unwrap_or(text)
}

// A struct that borrows. It cannot outlive `source`.
pub struct View<'a> {
    pub source: &'a [u8],
}

impl<'a> View<'a> {
    pub fn head(&self) -> &'a [u8] {
        &self.source[..self.source.len().min(4)]
    }
}
```

Third shape: tying output to both inputs when only one supplies it needlessly keeps other alive. Write narrowest true lifetime.

## Drop and the borrow checker

Adding `impl Drop` changes borrow rules; several errors follow, none titled `Drop`.

| You add `Drop` to | The new error | Cause |
| --- | --- | --- |
| a type with a lifetime parameter | E0597 on the borrowed local | dropck extends the borrow to the drop point |
| a guard that holds `&mut T` | E0502 at the next read of `T` | the drop point is one more use, after the last visible use |
| any type | E0509 at each partial move out of it | drop glue needs the whole value |
| a type that derives `Copy` | E0184 at the derive | `Copy` and `Drop` are exclusive |

NLL ends borrow at last use; `Drop` impl adds last use at scope's end.

### E0597: dropck extends the borrow to the drop point

Without `Drop` impl, compiler knows destruction cannot read `'a`. With one, `drop(&mut self)` could read reference; borrow must last until value drops. Locals drop in reverse declaration order: guard declared before source now fails.

```rust,compile_fail,E0597
struct NoDrop<'a>(&'a i32);
struct WithDrop<'a>(&'a i32);
impl Drop for WithDrop<'_> {
    fn drop(&mut self) {}
}

fn ok() {
    let d;
    let x = 5;
    d = NoDrop(&x);      // compiles
    let _ = d.0;
}

fn bad() {
    let d;
    let x = 5;
    d = WithDrop(&x);    // E0597: `x` does not live long enough
    let _ = d.0;
}
```

Note: "borrow might be used here, when `d` is dropped and runs the `Drop` code for type `WithDrop`", then "values in a scope are dropped in the opposite order they are defined". Declare borrowed local before guard, or keep type `Drop`-free.

### E0502: the drop point is one more use

```rust,compile_fail,E0502
struct Guard<'a>(&'a mut u32);
impl Drop for Guard<'_> {
    fn drop(&mut self) {}
}

fn read_while_guarded() {
    let mut x = 0u32;
    let _g = Guard(&mut x);
    println!("{x}"); // cannot borrow `x` as immutable because it is also borrowed as mutable
}
```

Note: "mutable borrow might be used here, when `_g` is dropped and runs the `Drop` code for type `Guard`". Call `drop(_g)` before read, or scope guard in inner block.

### E0507: `drop` holds `&mut self`, so a field pattern moves

`if let Some(h) = self.0` inside `drop` fails with "cannot move out of `self` as enum variant
`Some` which is behind a mutable reference" for every payload that is not `Copy`. rustc suggests
`if let Some(ref h) = self.0`, which gives a `&Handle` when you usually want `&mut`. Match on
`self` instead: `self` is already a reference, so default binding modes make every binding a
reference.

```rust
pub struct Handle;
impl Handle {
    pub fn report(&self, _reason: &str) {}
}

pub struct Guard(Option<Handle>);

impl Drop for Guard {
    fn drop(&mut self) {
        // `if let Some(h) = self.0` is E0507. Match on `self`; `h` binds as `&mut Handle`.
        let Self(Some(h)) = self else { return };
        h.report("unused");
    }
}
```

Take ownership with `self.0.take()` or `std::mem::replace` when cleanup must consume payload.

### E0509: a `Drop` impl blocks every partial move

```rust,compile_fail,E0509
struct Inner(String);
struct Outer {
    inner: Inner,
}
impl Drop for Outer {
    fn drop(&mut self) {}
}

// E0509: cannot move out of type `Outer`, which implements the `Drop` trait
fn take(o: Outer) -> Inner {
    o.inner
}
```

Drop glue needs whole value, with no holes. If cleanup belongs to separately owned resource, may move `Drop` to own guard and keep aggregate `Drop`-free. Preserve required cleanup order/lifetime: partial move must not release resource still needed by moved field. Here guard release is independent of `inner`:

```rust
pub struct Inner(pub String);

pub struct Ticket;
impl Ticket {
    pub fn release(&self) {}
}

// The guard holds only what the cleanup needs.
pub struct ReleaseGuard(pub Ticket);
impl Drop for ReleaseGuard {
    fn drop(&mut self) {
        self.0.release();
    }
}

pub struct Outer {
    pub inner: Inner,
    pub release: ReleaseGuard,
}

// Compiles: `Outer` has no `Drop` impl. `o.release` still drops at the end of `take`.
pub fn take(o: Outer) -> Inner {
    o.inner
}
```

When cleanup consumes guard's field, `Option` with `.take()` or deliberate `mem::replace` can express remaining state. Adding `Option` everywhere merely to allow move spreads absent states to all callers; name only real states. Keep pure core free of unsafe. Size difference authorizes no `ManuallyDrop` or manual unsafe cleanup; native ownership changes belong to [implement-rust](../../../implementation/implement-rust/SKILL.md).

### E0184 and E0367: the two `Drop` impl rules

`#[derive(Copy)]` plus `impl Drop` reports "the trait `Copy` cannot be implemented for this type; the type has a destructor". Bitwise copy plus destructor constructs double free; resource handle never `Copy`. Remove derive.

`impl<T: Clone> Drop for Foo<T>` on `struct Foo<T>(T)` reports E0367, "`Drop` impl requires `T: Clone` but the struct it is implemented for does not", with "note: the implementor must specify the same requirement". Drop glue required for every instantiation; impl cannot apply to only some. Move bound onto struct:

```rust
pub struct Bar<T: Clone>(pub T);
impl<T: Clone> Drop for Bar<T> {
    fn drop(&mut self) {}
}
```

## E0631: a function item is not a coercion site

```text
error[E0631]: type mismatch in function arguments
    = note: expected function signature `fn(&String) -> _`
               found function signature `fn(&str) -> _`
help: consider wrapping the function in a closure
```

Deref coercion runs at an expression, not at a trait bound. With `v: Vec<String>` and `fn
count_words(s: &str)`, the call `count_words(&v[0])` compiles because `&String` coerces to `&str`
at that call. `v.iter().map(count_words)` fails, because `Iterator::map` demands
`F: FnMut(&String) -> B` and rustc matches the function item's signature against that bound
exactly.

```rust
fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

pub fn totals(v: &[String]) -> (usize, usize) {
    let closure: usize = v.iter().map(|s| count_words(s)).sum();
    let adapter: usize = v.iter().map(String::as_str).map(count_words).sum();
    (closure, adapter)
}
```

Inside closure body, call is expression again; coercion applies. Same rule for `Option::map`, `Result::map_err` and every higher-order call. E0599 on `.sum()` after failed `map` is cascade; disappears with fix.

## Related

- [SKILL.md](../SKILL.md): the triage table and the fixes that hide the bug.
- For a callback-bound cascade or a diagnostic outside this catalogue, read the actual
  `rustc --explain` and locked source with
  [implement-rust](../../../implementation/implement-rust/SKILL.md). No additional skill is required.
