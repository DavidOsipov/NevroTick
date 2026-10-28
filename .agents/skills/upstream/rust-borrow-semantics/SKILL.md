---
name: "rust-borrow-semantics"
description: "Diagnose Rust temporary or guard drop timing, expression-dependent E0716, two-phase borrows, or edition-specific scope changes using exact-expression probes. Use rust-compiler-errors for general diagnostic triage."
license: "BSD-3-Clause"
---

# Rust Borrow Semantics

NevroTick language-mechanics reference alongside [implement-rust](../../implementation/implement-rust/SKILL.md); existing [QA contract](../../testing/CONTRACT.md) owns verification. Establish actual expression, existing Cargo project, edition, toolchain/MSRV and supported package/feature/target lane. Preserve approved state owner, invariant-bearing lock interval, resource lifetime and effect-sink authority. No authorization for new dependencies, runtimes, features, edition/MSRV changes or unsafe in pure core. Probe proves language question, not application safety.

Keep exact source expression until every borrow/drop point is known. Manual desugaring, added helper call, macro or edition change can move points without changing types.

## Start with four facts

Record before proposing fix:

1. Is each operand a place expression or a value expression?
2. Which expression creates each temporary?
3. Which borrow is explicit, and which borrow does autoref or reborrow insert?
4. Which edition and which minimum supported Rust version (MSRV) apply? Temporary scope and
   lifetime extension rules changed in several releases from 1.79 to 1.98.

Write expected reservation, activation and drop points. Prove with probe (see [Verification](#verification)).

## Triage

| Symptom | Likely cause | First action |
| --- | --- | --- |
| Method call compiles, UFCS rewrite fails with E0502 | The rewrite removed an implicit two-phase borrow | Restore the method call, or compute the other argument first |
| E0716 after you add a helper call | A function call does not extend the temporary | Name the owner in a `let` before the call |
| `Some(&temp())` or `Wrapper(&temp())` compiles on stable, fails with E0716 on the MSRV lane | Constructor lifetime extension needs Rust 1.89 | Name the owner so the expression works on the selected MSRV; a toolchain change belongs to its project workflow |
| New E0716 at a block tail after a move to edition 2024 | The tail temporary now drops at the end of the block | Move the block value into a named `let` |
| After a move to edition 2024, a guard releases earlier: a deadlock disappears, a race appears, or `try_lock` succeeds in `else` | The `if let` scrutinee or the tail temporary scope narrowed | Bind the guard to a named local in an explicit block |
| A lock stays held through a whole `match`, or a second `lock()` later in the same statement deadlocks | A `match` scrutinee temporary lives through every arm on every edition. It drops at the end of the enclosing statement, or before the block's locals when the `match` is a block tail on edition 2024 | If the invariant allows release before matching, extract only the needed owned fact inside an explicit guard block, then match on it. Otherwise retain the guard and redesign the reentrant access. Probe a guarded pattern's exact syntax on the selected toolchain |
| E0502 or E0499 at a call to a function that returns `impl Trait`, only on edition 2024, with the note "may capture more lifetimes than intended" | The opaque return type captures the argument lifetime. This is a signature rule, not a temporary scope | Inspect the callee's real borrow requirement before changing the signature. The optional [rust-compiler-errors](../rust-compiler-errors/SKILL.md) reference covers this diagnostic |
| E0716 or a moved drop point in a `pin!`, `format_args!`, `write!`, `writeln!`, or `assert_eq!` call after a rewrite, a toolchain change, or on the MSRV lane | Macro temporaries have their own rules, changed in 1.89-1.98 | Probe the exact macro call on the MSRV and on stable |
| Adding braces changes whether code compiles | The block changed a temporary or borrow scope | Mark the exact creation and drop points |
| A clone makes the error disappear | Ownership was duplicated, not explained | See [Clone is not a scope operator](#clone-is-not-a-scope-operator) |

## Place expressions and value expressions

Place expression identifies storage; value expression produces value. Distinction decides move from existing storage versus temporary creation.

| Shape | Usually a place | Usually a value |
| --- | --- | --- |
| Local, static, dereference, index, field | Yes | No |
| Literal, arithmetic result, function call, constructor | No | Yes |
| Parentheses | Keep the inner classification | Keep the inner classification |
| Block tail | Depends on the tail expression | Depends on the tail expression |

Do not classify from type. `String` can be local place or call-returned value.

## Temporary scopes

Temporary drops at enclosing temporary scope's end: statement, condition, match arm or block tail.

Edition 2024 narrows two scopes: `if let` scrutinee temporaries drop before `else` block; block-tail temporaries drop before block's locals. Both release guards earlier, never later. `match` scrutinee temporary still lives through every arm.

Probe runs on edition 2024; first assertion fails on edition 2021:

```rust,run
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(None::<u32>);
    let free_in_else = if let Some(v) = *m.lock().unwrap() {
        v == 0
    } else {
        m.try_lock().is_ok()
    };
    assert!(free_in_else);
    let free_in_arm = match *m.lock().unwrap() {
        Some(v) => v == 0,
        None => m.try_lock().is_ok(),
    };
    assert!(!free_in_arm);
}
```

Narrowing can reject code that compiled on edition 2021:

```rust,compile_fail,E0716
fn main() {
    let len = { &String::from("1234") }.len();
    println!("{len}");
}
```

When resource guard must release before later work, bind it; make drop point explicit with braces. Same drop point on every edition:

```rust
use std::sync::Mutex;

let state = Mutex::new(vec![1]);
let first = {
    let guard = state.lock().expect("state mutex poisoned");
    guard[0]
};
assert_eq!(first, 1);
assert!(state.try_lock().is_ok());
```

## Temporary lifetime extension

Some `let` patterns (`ref` bindings) and initializers extend borrowed temporary to block's end. Syntactic rule, not general lifetime inference. Since Rust 1.89, extension passes through tuple struct/variant constructor arguments. On older MSRV, two constructor lines below fail with E0716:

```rust,run
fn make() -> String { String::from("alive") }

struct Wrapper<'a>(&'a String);

fn main() {
    let value = &make();
    let some = Some(&make());
    let wrapped = Wrapper(&make());
    assert_eq!(value, "alive");
    assert_eq!(some.map(String::as_str), Some("alive"));
    assert_eq!(wrapped.0, "alive");
}
```

Ordinary function call does not extend temporary:

```rust,compile_fail,E0716
fn make() -> String { String::from("short") }
fn keep(value: &String) -> &String { value }

fn main() {
    let value = keep(&make());
    println!("{value}");
}
```

Fix: name owner before borrow:

```rust
fn make() -> String { String::from("long") }
fn keep(value: &String) -> &String { value }

let owner = make();
let value = keep(&owner);
assert_eq!(value, "long");
```

## Two-phase borrows

Two-phase mutable borrow starts as reservation, becomes exclusive at activation. Shared access possible between points. Rust creates two-phase borrows only for these implicit borrows:

| Implicit shape | Typical example |
| --- | --- |
| Autoref of a mutable method receiver | `values.push(values.len())` |
| Mutable reborrow in a function argument | Passing an existing `&mut T` to another call |
| Mutable borrow for overloaded compound assignment | `place += rhs` |

Standard method-call shape compiles:

```rust,run
fn main() {
    let mut values = vec![10, 20];
    values.push(values.len());
    assert_eq!(values, [10, 20, 2]);
}
```

Explicit `&mut` borrow is not two-phase. Manual UFCS rewrite can fail while method call compiles:

```rust,compile_fail,E0502
fn main() {
    let mut values = vec![10, 20];
    let receiver = &mut values;
    Vec::push(receiver, values.len());
}
```

Do not explain first program by evaluation order alone. Record:

1. The receiver autoref reserves `&mut values`.
2. Rust evaluates the other arguments. Shared reads can occur during the reservation.
3. The call activates the mutable borrow.
4. The borrow ends after the call, unless the result keeps it alive.

Rust still rejects a conflicting write between reservation and activation.

## Clone is not a scope operator

Clone can hide borrow error by duplicating ownership; it does not explain borrow. Temporary scope: name owner or add block. Two-phase borrow: keep method-call shape or compute argument first. Clone only when independent copy matches domain, e.g. small value or shared read-only state. Do not clone resource handle or unique buffer to escape scope.

## Edition 2024 review

Run two migration lints while crate remains on edition 2021. Lints go quiet after edition changes: behavior already changed:

```bash
cargo clippy --locked -p <crate> --all-targets --target <supported-target> --no-default-features --features <supported-feature-set> -- -W if_let_rescope -W tail_expr_drop_order
```

Placeholders describe one explicitly selected lane, not literal command. Preserve actual default-feature selection; omit `--features` for lane with none, omit `--no-default-features` when defaults belong. Lints see only compiled code. Select relevant supported feature/target lanes through existing verification owner; do not combine incompatible features or expand small diagnostic into release qualification.

Both lints are allow-by-default `rust_2024_compatibility` members. Interpret results:

- `cargo fix --edition` rewrites an `if_let_rescope` site into a `match`. The `match` keeps the
  edition 2021 lock hold, and any deadlock that came with it. Keep that hold only when the
  `else` branch must run while the lock is still held. Then prefer a named guard in an explicit
  block over the generated `match`, because the block shows the hold. Otherwise restore the
  `if let` and accept the earlier drop.
- `cargo fix --edition` reports `tail_expr_drop_order` sites but changes nothing. Inspect each
  site, and bind the temporary to a named local when its drop order matters.
- Do not use an edition migration as a lock-lifetime refactor. Run behavior tests for code whose
  temporaries own locks, transactions, file descriptors, or other guards.

Use [implement-rust](../../implementation/implement-rust/SKILL.md) for actual feature/target lane and authorized edition migration. Language examples adopt no migration.

## Verification

Use smallest decisive proof:

1. Put the exact expression in a standalone file. Do not simplify it first.
2. Compile the probe on the workspace edition using an already available toolchain
   (`rustc --edition <edition> --emit=metadata probe.rs`). When the fix relies on a rule that is
   newer than the MSRV, compile the probe with `rustc +<msrv>` and the real fix with
   `cargo +<msrv> check --locked`. Examples: `if`/`match` lifetime extension (1.79+), `use<..>`
   (1.82+; in a trait, 1.87+), constructor lifetime extension (1.89+), and macro temporaries
   (changed in 1.89-1.98).
3. Use a `compile_fail` probe for a rejected borrow, and name the expected error code.
4. Use a runtime assertion, for example `try_lock().is_ok()`, when the question is a drop point
   or a destructor order.
5. Run the real workspace tests after the probe. The probe proves the language rule, not the
   application invariant.

Do not use Miri for type-check questions. Use after compilation when unsafe aliasing/provenance also in scope.

## Optional companion reference

[rust-compiler-errors](../rust-compiler-errors/SKILL.md) covers general ownership/lifetime diagnostics. Use when triage helps; otherwise inspect `rustc --explain` and primary language rules with [implement-rust](../../implementation/implement-rust/SKILL.md) as project owner. No absent specialist skill or new tool installation required.

Adapted from po4yka/rust-skills; see [LICENSE](LICENSE) and
[local changes and source pin](LOCAL-CHANGES.md).
