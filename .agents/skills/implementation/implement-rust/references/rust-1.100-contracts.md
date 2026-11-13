# Use Rust 1.100 to make contracts explicit

Use when introducing or reviewing Rust 1.100 APIs, cleanup/ownership changes or
agent-written modernization. Apply the [implementation contract](../../CONTRACT.md)
and [modern Rust baseline checks](modern-rust.md#check-the-actual-baseline).
This reference guides authorized work; it does not schedule repository-wide
rewrites, change Cargo policy, lift execution holds or establish product qualification.

Reviewed 13 November 2026 against NevroTick
`8251880a7223e5aa591a317c51196c13316fb7f6` and Rust
`0ef1bc386059596dada8fb65b50425aab2ea637a` (1.100.0). Reconcile current source
before applying the examples. See the [source register](../../SOURCES.md#rust-1100-contract-guidance--13-november-2026).

## Contents

- [Choose the actual guarantee](#choose-the-actual-guarantee)
- [Bind cleanup to successful acquisition](#bind-cleanup-to-successful-acquisition)
- [Express only genuinely impossible outcomes](#express-only-genuinely-impossible-outcomes)
- [Prefer precise bounded operations](#prefer-precise-bounded-operations)
- [Preserve allocation and identity contracts](#preserve-allocation-and-identity-contracts)
- [Keep unsafe and Windows APIs within their owners](#keep-unsafe-and-windows-apis-within-their-owners)
- [Verify the changed contract](#verify-the-changed-contract)

## Choose the actual guarantee

For each substitution, name the mistake it prevents, the enforcing mechanism,
the legitimate behavior that must remain accepted and the affected owner.
Distinguish compiler-excluded states, enforced diagnostics, automatic cleanup
and clearer expressions. A shorter expression can reduce editing mistakes without
creating a new type invariant. Prefer it when the actual operation becomes clearer;
do not require a performance claim or invent a defect to justify that improvement.

Preserve private checked constructors, typed refusals, exhaustive field/state
handling, live authorization and prepare-before-commit transitions. They are
existing contracts, not new 1.100 features. Do not weaken them to obtain a lint
pass or fit a new API. Reuse the existing owner tests rather than testing the
standard library's implementation.

## Bind cleanup to successful acquisition

Use `std::mem::DropGuard` for a simple local cleanup obligation currently exposed
to forgotten early-return/`?` cleanup. Bind a named guard immediately **after**
successful acquisition. An invocation that did not acquire the resource/flag
must not release another invocation's ownership. Preserve atomic orderings,
lock coverage, release order and explicit early `drop` before dependent work.
Never use an ignored temporary as the intended lifetime owner.

In the reviewed [tray implementation](../../../../../crates/nevrotick-ui/src/native_tray/windows.rs),
`popup` can guard `menu_open` after the successful swap and explicitly drop the
guard before `control(bridge)`. Current ordinary returns already reset the flag;
the benefit is protection against later exit-path edits and unwind cleanup.
Challenge nested acquisition, success, error and unwind; observe the flag before
the dependent control operation. Flag release proves no native-menu destruction.

Use `DropGuard::dismiss(guard)` only at an actual acknowledged transfer/commit
point; it consumes the guard without running its callback. In `post_control`,
successful `PostMessageW` must retain `control_queued` until receiver consumption.
Failure rollback currently resets it while holding the endpoint lock. A rollback
guard declared before the mutex guard drops after it: preserve reset-before-unlock
or justify and qualify a deliberate contract change. Do not apply a mechanical
rollback conversion to this path.

Retain named guards that enforce admitted entry, owner association, state
transitions or lock order: `ProjectionGuard`, `PreparedAcquisition`, `ScopeGuard`
and `ControlGuard` carry those contracts. `PublishGuard` already covers scope
exit and unwind; generic replacement has no additional cleanup coverage.
Do not expose arbitrary dismissal where a domain guard intentionally lacks it.

For fallible native teardown, retain a named owner with explicit fallible
`finish`/`close` and a nonpanicking drop fallback. Preserve cleanup-failure
reporting, ordering and single-attempt ownership; do not discard `DestroyMenu`
or `DeleteObject` errors in a closure. Synchronous drop cannot await native/task
completion: preserve explicit joins, completion custody and acknowledgements.
Abort/exit/leaks need their separate cleanup contract; destructors are not guaranteed.

Source: [shipped DropGuard implementation](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/core/src/mem/drop_guard.rs).

## Express only genuinely impossible outcomes

Use `!` in an associated error type or `Result<T, !>` when ordinary typed failure
is impossible and the interface needs a `Result`; return `T` directly when no
such interface requires it. Use `Result<!, E>` only when normal successful return
is impossible. Neither type proves absence of panic, allocation failure or
nontermination. Return-position `fn ... -> !` predates this release.

`Infallible` is now an alias for `!`: changing its spelling adds no guarantee.
Do not erase real input/state/native failures or use panic/defaults to satisfy an
infallible signature. The real Windows status reporter remains fallible; the
test reporter may use `!`. The [native message loop](../../../../../crates/nevrotick-ui/src/native.rs)
returns success on `WM_QUIT` and then performs cleanup; retain that outcome.

Source: [shipped Infallible definition](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/core/src/convert/mod.rs).

## Prefer precise bounded operations

| API | Prefer when | Preserve / reject |
| --- | --- | --- |
| `Vec::from_fn(length, f)` | A bounded finite indexed construction is clearer than an iterator with a separate limiter. The reviewed localization slots can use `Vec::from_fn(COUNT, |_| None)`. | Establish the count bound first; preserve missing/duplicate rejection. Length is not a type-level invariant and this constructor supplies no explicit allocation-error return. |
| `str`/slice `trim_prefix` or `trim_suffix`; `Path::trim_prefix` | Removal is optional and absence must leave the input unchanged. Respect the selected type's matching semantics. | Keep `strip_*` plus typed absence rejection when the delimiter/prefix is mandatory. These APIs do not validate input, establish path confinement or authorize access. |
| `bool::toggle()` | The operation intentionally inverts that same Boolean. The engine's `reverse` fairness variable is such a case. | Preserve update timing relative to awaits/branches. Keep sticky cancellation/observed flags and values derived from another field explicit; repeated Stop must not clear cancellation. |

In [source draft parsing](../../../../../crates/nevrotick-ui/src/source_drafts.rs),
`raw.host.trim_suffix('.')` can replace the optional one-dot match only in the
existing branch for hosts without `:`. Keep raw byte/ASCII/repeated-dot rejection before
normalization and the checked constructor afterward. Do not substitute repeated
trimming. Existing accepted root-dot and invalid raw-input controls own this change.
The [CLI transcript checker](../../../../../crates/nevrotick-engine/tests/runtime_boundary.rs)
requires its final newline: retain `strip_suffix('\n')` and absence rejection.

Sources: [Vec construction](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/alloc/src/vec/mod.rs),
[string trimming](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/core/src/str/mod.rs),
[Boolean inversion](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/core/src/bool.rs).

## Preserve allocation and identity contracts

`Box::map` consumes `T` through `FnOnce(T) -> U`; `Rc::map`/`Arc::map` call
`FnOnce(&T) -> U`. Allocation reuse is conditional. Mapping a shared pointer
does not update existing aliases or preserve identity-bearing shared custody.
Keep `Arc::clone` where consumers require the same allocation, including
`Arc::ptr_eq` checks in [session execution](../../../../../crates/nevrotick-engine/src/session.rs)
and reply admission. Equal-looking replacement data is not the original owner.

Use `Box::take` only for a justified allocation-reuse protocol; it returns the
value and `Box<MaybeUninit<T>>`, so preserve initialization and ownership.
`Box`/`Rc`/`Arc::try_map` remain unstable (`smart_pointer_try_map`) in 1.100.
Do not invent stable fallible signatures or add unwrap/panic to fit `map`.

Preserve the [NTS response reader](../../../../../crates/nevrotick-engine/src/nts_ke_experiment/mod.rs)
sequence `try_reserve_exact` -> `Failure::Allocation` -> initialization. Replacing
it with `Vec::from_fn` removes explicit allocation refusal. Retain byte/count
bounds, zeroization and complete candidate preparation before mutation.

Sources: shipped [Box](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/alloc/src/boxed.rs),
[Rc](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/alloc/src/rc.rs)
and [Arc](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/alloc/src/sync.rs).

## Keep unsafe and Windows APIs within their owners

- `Allocator`, allocator-parameterized `Vec`/`Box`, `Vec::new_in` and
  `Vec::with_capacity_in` are stable. A custom allocator needs a concrete budget
  or allocation requirement and a reviewed unsafe boundary; preserve the engine's
  unsafe prohibition. `Vec::try_with_capacity_in` remains unstable
  (`allocator_ext`); stable fallible composition uses `Vec::new_in` then
  `try_reserve`/`try_reserve_exact`. Keep allocation, deallocation and allocator
  destruction nonunwinding. Qualify exhaustion, layout, lifetime,
  failed-growth preservation and zeroization; no implicit memory/performance claim.
- `UnsafeCell::replace`, `as_ref_unchecked` and `as_mut_unchecked` do not enforce
  borrowing or synchronization. Use only inside an already justified unsafe
  abstraction with its alias/lifetime proof; do not replace checked `RefCell` or
  `Mutex` ownership to silence an agent's borrow error.
- `conjure_zst` requires an inhabited zero-sized type and valid semantic use.
  Zero size does not authorize bypassing initialization. In particular, the
  private [Apartment token](../../../../../crates/nevrotick-ui/src/native_notifications_windows.rs)
  records successful `RoInitialize` and owns matching thread-bound teardown;
  retain its checked constructor. Never conjure an uninhabited type.
- Windows `ChildExt::main_thread_handle` borrows from `Child`; it transfers no
  handle ownership or completion guarantee. Do not close it or create an owning
  wrapper from the borrowed raw handle. `CommandExt::show_window(u16)` sets child
  startup visibility; it does not replace `CREATE_NO_WINDOW`, UI `ShowWindow`,
  job/mitigation policy or waiting for actual exit. Adopt only for a real consumer.

Sources: [allocator contract](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/core/src/alloc/mod.rs),
[UnsafeCell](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/core/src/cell.rs),
[zero-sized construction](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/core/src/mem/mod.rs)
and [Windows process APIs](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/std/src/os/windows/process.rs).

## Verify the changed contract

Use [1.100 lint enforcement](../../../testing/references/rust-1.100-lints.md) for
Cargo policy and new diagnostics; Rust warning denial is not Cargo lint policy.
Use [configured tooling](../../../testing/references/configured-tooling.md) for
actual code changes and preserve target/feature/native boundaries. Reuse existing
behavioral controls for equivalent expressions; add focused evidence only for a
changed cleanup, error, ownership or enforcement guarantee. A compile-failure
control must fail for the intended diagnostic and retain a valid companion.

Report the actual change and evidence: clearer expression, automatic cleanup,
typed impossibility or executed lint rejection. Skill/source review alone proves
no compiler pass, native completion, performance gain or measured reduction in
AI defects. Documentation-only changes need package/link checks and proportionate
instruction-use review; they do not initiate product or formal execution.
