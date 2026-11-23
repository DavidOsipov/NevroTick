# Use Rust 1.100 to make contracts explicit

Use when introducing or reviewing Rust 1.100 APIs, cleanup/ownership changes or
agent-written modernization. Apply [implementation contract](../../CONTRACT.md)
and [modern Rust baseline checks](modern-rust.md#check-the-actual-baseline).
This reference guides authorized work; it does not schedule repository-wide
rewrites, change Cargo policy, lift execution holds or establish product qualification.

Reviewed 13 November 2026 against NevroTick
`8251880a7223e5aa591a317c51196c13316fb7f6` and Rust
`0ef1bc386059596dada8fb65b50425aab2ea637a` (1.100.0). Reconciled
23 November 2026 against NevroTick `a0b461de628a690d9c8b32ebb79677ed5276ceff`
and the same Rust pin. Reconcile current source before applying examples.
See the [source register](../../SOURCES.md#rust-1100-stabilized-api-reconciliation--23-november-2026)
and [complete 47-entry disposition](../../../../../docs/supporting-reviews/rust-1.100-upgrade-review.md#stabilized-api-reconciliation--23-november-2026).

## Contents

- [Choose the actual guarantee](#choose-the-actual-guarantee)
- [Bind cleanup to successful acquisition](#bind-cleanup-to-successful-acquisition)
- [Express only genuinely impossible outcomes](#express-only-genuinely-impossible-outcomes)
- [Prefer precise bounded operations](#prefer-precise-bounded-operations)
- [Keep value equality and initialization ownership distinct](#keep-value-equality-and-initialization-ownership-distinct)
- [Preserve allocation and identity contracts](#preserve-allocation-and-identity-contracts)
- [Keep allocator custody explicit](#keep-allocator-custody-explicit)
- [Keep unsafe and Windows APIs within their owners](#keep-unsafe-and-windows-apis-within-their-owners)
- [Verify the changed contract](#verify-the-changed-contract)

## Choose the actual guarantee

For each substitution, name prevented mistake, enforcing mechanism,
legitimate behavior that must remain accepted and affected owner.
Distinguish compiler-excluded states, enforced diagnostics, automatic cleanup
and clearer expressions. Shorter expressions can reduce editing mistakes without
new type invariants. Prefer them when the actual operation becomes clearer;
do not require performance claims or invent defects to justify improvement.

Preserve private checked constructors, typed refusals, exhaustive field/state
handling, live authorization and prepare-before-commit transitions: existing
contracts, not new 1.100 features. Do not weaken them for a lint pass or new API.
Reuse existing owner tests rather than testing standard library implementation.

## Bind cleanup to successful acquisition

Use `std::mem::DropGuard` for simple local cleanup obligations currently exposed
to forgotten early-return/`?` cleanup. Bind a named guard immediately **after**
successful acquisition. An invocation that did not acquire the resource/flag
must not release another invocation's ownership. Preserve atomic orderings,
lock coverage, release order and explicit early `drop` before dependent work.
Never use an ignored temporary as intended lifetime owner.

In the reviewed [tray implementation](../../../../../crates/nevrotick-ui/src/native_tray/windows.rs),
`popup` can guard `menu_open` after successful swap and explicitly drop the
guard before `control(bridge)`. Current ordinary returns already reset the flag;
benefit: protection against later exit-path edits and unwind cleanup.
Challenge nested acquisition, success, error and unwind; observe flag before
dependent control operation. Flag release proves no native-menu destruction.

Use `DropGuard::dismiss(guard)` only at an actual acknowledged transfer/commit
point; it consumes the guard without running its callback. In `post_control`,
successful `PostMessageW` must retain `control_queued` until receiver consumption.
Failure rollback currently resets it under the endpoint lock. A rollback guard
declared before the mutex guard drops after it: preserve reset-before-unlock
or justify and qualify a deliberate contract change. Do not mechanically convert
rollback here.

Retain named guards enforcing admitted entry, owner association, state
transitions or lock order: `ProjectionGuard`, `PreparedAcquisition`, `ScopeGuard`
and `ControlGuard` carry those contracts. `PublishGuard` already covers scope
exit and unwind; generic replacement adds no cleanup coverage.
Do not expose arbitrary dismissal where a domain guard intentionally lacks it.

The service's [WorkPermit](../../../../../crates/windows-platform/src/service.rs)
intentionally does not complete on drop: do not wrap `complete` in `DropGuard`.
Its [drop/cancellation control](../../../../../crates/windows-platform/tests/service.rs)
requires outstanding work to remain charged. This is not a ban on permit
destructors: [DnsQueryPermit](../../../../../crates/windows-platform-native/src/dns_address.rs)
releases on drop because its `NativeQuery` owner retains it through actual
completion, independently of receiver abandonment. Preserve that custody chain.

For fallible native teardown, retain a named owner with explicit fallible
`finish`/`close` and nonpanicking drop fallback. Preserve cleanup-failure
reporting, ordering and single-attempt ownership; do not discard `DestroyMenu`
or `DeleteObject` errors in a closure. Synchronous drop cannot await native/task
completion: preserve explicit joins, completion custody and acknowledgements.
Abort/exit/leaks need separate cleanup contract; destructors are not guaranteed.

Source: [shipped DropGuard implementation](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/core/src/mem/drop_guard.rs).

## Express only genuinely impossible outcomes

Use `!` in an associated error type or `Result<T, !>` when ordinary typed failure
is impossible and the interface needs a `Result`; return `T` directly if no
such interface requires it. Use `Result<!, E>` only when normal successful return
is impossible. Neither type proves absence of panic, allocation failure or
nontermination. Return-position `fn ... -> !` predates this release.

`Infallible` now aliases `!`: spelling changes add no guarantee.
Apply the [outcome-diagnostic limits](../../implement-rust-contracts/references/contract-enforcement.md#make-structural-changes-require-review):
`Result<(), !>` alone does not trigger the type-based unused-result lint.
Do not erase real input/state/native failures or use panic/defaults to satisfy an
infallible signature. Real Windows status reporter remains fallible;
test reporter may use `!`. The [native message loop](../../../../../crates/nevrotick-ui/src/native.rs)
returns success on `WM_QUIT` then cleans up; retain that outcome.

`AsRef<T>`/`AsMut<T>` for `!` complete generic trait compatibility; they provide
no constructible value or reference and cannot turn a fallible path into an
infallible one. Their ordinary implementations are stable, not const-call support.

Source: [shipped Infallible definition](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/core/src/convert/mod.rs).

## Prefer precise bounded operations

| API | Prefer when | Preserve / reject |
| --- | --- | --- |
| `Vec::from_fn(length, f)` | A bounded finite indexed construction is clearer than an iterator with a separate limiter. The reviewed localization slots can use `Vec::from_fn(COUNT, |_| None)`. | Establish the count bound first; the closure receives ascending indices and is not called for zero length. Preserve missing/duplicate rejection. Length is not a type-level invariant; this constructor supplies no allocation-error return. |
| `str`/slice `trim_prefix` or `trim_suffix`; `Path::trim_prefix` | Remove one optional matching edge; absence returns the original borrowed input. Path matching uses components, not textual prefixes. | Keep `strip_*` plus typed absence rejection for required delimiters. These methods neither repeatedly trim nor validate input. Path trimming resolves no symlinks and establishes no filesystem confinement. Stable slice methods do not stabilize implementing `SlicePattern`. |
| `bool::toggle()` | The operation intentionally inverts that same Boolean. The engine's `reverse` fairness variable is such a case. | Preserve update timing relative to awaits/branches. Keep sticky cancellation/observed flags and values derived from another field explicit; repeated Stop must not clear cancellation. |

In [source draft parsing](../../../../../crates/nevrotick-ui/src/source_drafts.rs),
`raw.host.trim_suffix('.')` can replace the optional one-dot match only in
existing branch for hosts without `:`. Keep raw byte/ASCII/repeated-dot rejection before
normalization and checked constructor afterward. Do not substitute repeated
trimming. Existing accepted root-dot and invalid raw-input controls own this change.
The [CLI transcript checker](../../../../../crates/nevrotick-engine/tests/runtime_boundary.rs)
requires its final newline: retain `strip_suffix('\n')` and absence rejection.

The [DNS name owner](../../../../../crates/windows-platform-native/src/dns_address.rs)
provides a paired example: `DnsQueryName::new` must retain
`strip_suffix(b".").ok_or(DnsNameError::MissingRoot)`; its OS-reported-name
matcher may use `reported.trim_suffix(&[root])` for one optional UTF-16 root.
Keep ASCII folding, exact checked-name matching and the existing accepted,
multiple-root and rootless-query controls. Optional normalization cannot relax
the constructor's absolute-name requirement.

Sources: [Vec construction](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/alloc/src/vec/mod.rs),
[string trimming](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/core/src/str/mod.rs),
[slice trimming](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/core/src/slice/mod.rs),
[path trimming](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/std/src/path.rs),
[Boolean inversion](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/core/src/bool.rs).

## Keep value equality and initialization ownership distinct

The new `PartialEq<Cow<'_, [U]>>` implementations for `Vec<T, A>`, `&[T]`
and `&mut [T]` compare contents without forcing `Cow` ownership. Use them only
when element equality is the intended relation; they establish neither domain
equivalence, allocation identity nor constant-time secret comparison. No direct
current consumer was identified. Preserve `DraftSource::same_identity` separately
from derived whole-value equality and retain session `Arc::ptr_eq` checks.

`Once::default()` creates a fresh `Once::new()`; it does not reuse process-wide
initialization state. Preserve the same instance and its poisoning behavior.
Ordinary `Default` is stable; its const implementation still needs
`const_default`, while `Once::new` is already const. Do not replace the
[source policy's](../../../../../crates/nevrotick-engine/src/source_policy.rs)
static `OnceLock<Arc<PolicyCell>>` or its successful single-installation contract.

Sources: [cross-type equality](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/alloc/src/vec/partial_eq.rs)
and [Once default](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/std/src/sync/once.rs).

## Preserve allocation and identity contracts

`Box::map` consumes `T` through `FnOnce(T) -> U`; `Rc::map`/`Arc::map` call
`FnOnce(&T) -> U`. The pinned implementation reuses allocation only for equal
size and alignment; `Rc`/`Arc` additionally need one strong owner and no weak
owners. This is conditional reuse, not a performance or identity guarantee.
Mapping does not retarget existing aliases to the returned `U`: other strong
owners retain the old `T`; weak-only aliases can expire when the old `T` drops.
The closure can still change shared state through permitted interior mutability.
Keep `Arc::clone` for consumers requiring the same allocation, including
`Arc::ptr_eq` checks in [session execution](../../../../../crates/nevrotick-engine/src/session.rs)
and reply admission. Equal-looking replacement data is not the original owner.

Use `Box::take` only for a justified allocation-reuse protocol; it returns
`(T, Box<MaybeUninit<T>, A>)`. The returned value has separate ownership;
the retained allocation is neither initialized as `T`, zeroized nor freed.
Prefer safe `Box::write` when reinitializing it.
`Box`/`Rc`/`Arc::try_map` remain unstable (`smart_pointer_try_map`) in 1.100.
Do not invent stable fallible signatures or add unwrap/panic to fit `map`.

Preserve the [NTS response reader](../../../../../crates/nevrotick-engine/src/nts_ke_experiment/mod.rs)
sequence `try_reserve_exact` -> `Failure::Allocation` -> initialization. Replacing
it with `Vec::from_fn` removes explicit allocation refusal. Retain byte/count
bounds, zeroization and complete candidate preparation before mutation.

Sources: shipped [Box](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/alloc/src/boxed.rs),
[Rc](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/alloc/src/rc.rs)
and [Arc](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/alloc/src/sync.rs).

## Keep allocator custody explicit

`Allocator`, `Global` and the listed allocator-aware `Box`/`Vec` facilities are
stable in 1.100; `System` itself predates this release (1.28), while its
`Allocator` implementation is new. Stable availability does not imply const
callability. A custom allocator needs a concrete allocation requirement and a
reviewed unsafe boundary; preserve the engine's unsafe prohibition. An allocator
parameter alone enforces no budget, zeroization or performance improvement.

Forwarding through `&A`/`&mut A` preserves equivalence to the referent and its
lifetime; it proves no unrelated allocator or native-memory ownership. The
`Allocator` implementations for `Box<T, A>`, `Rc<T, A>` and `Arc<T, A>` delegate to the inner
allocator `T`, not the allocator `A` storing that smart pointer. Ordinary
`Arc::new(allocator)` can supply `Vec::new_in` when shared allocator ownership
is justified; `Rc::new_in`, `Arc::new_in` and their explicit allocator type
parameters remain unstable (`allocator_ext`).

`Box::new_in`/`Vec::with_capacity_in` return no allocation-error `Result`.
`Box::try_new_in`/`Vec::try_with_capacity_in` remain unstable. `Vec::new_in`
then `try_reserve`/`try_reserve_exact` preserves reported reservation failures;
the `Allocator` contract still permits an underlying allocator to abort.
Preserve the NTS reader's zeroizing owner, `Failure::Allocation` mapping and
initialization order, and stage all replacement cookies before publication.
Keep allocation, deallocation and allocator destruction nonunwinding.
For an actual changed allocation path or custom allocator, qualify exhaustion,
layout/lifetime, preservation of the old allocation and its contents after
failed growth, and required zeroization through the existing QA owners.

For raw allocator-aware handoffs, apply the existing
[raw-parts and initialization rules](modern-rust.md#use-modern-pointer-tools-inside-existing-unsafe-boundaries).
Retain the returned allocator with the pointer, initialized length and actual
capacity; the same custom allocator type or ordinary `Clone` alone does not
prove interchangeable allocation ownership. Use that allocator or a documented
equivalent. `NonNull` excludes null only; layout, validity, initialization,
unique ownership and lifetime obligations remain.

`Global` and `System` are not interchangeable. Per-collection `System` can
bypass the [global allocation observer](../../../../../policy/time-domain-windows-allocation-observer.rs);
keep its `GlobalAlloc` instrumentation and observation scope. Windows-provided
memory retains its matching release family: the
[WinDNS RecordList](../../../../../crates/windows-platform-native/src/windows_dns.rs)
requires `DnsFree`, never adoption into `Box`/`Vec` with `System`; the same rule
applies to `LocalFree`-owned buffers.

Sources: [Allocator contract](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/core/src/alloc/mod.rs),
[Global](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/alloc/src/alloc.rs),
[System](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/std/src/alloc.rs)
and the Box/Vec/Rc/Arc sources linked above.

## Keep unsafe and Windows APIs within their owners

- `UnsafeCell::replace`, `as_ref_unchecked` and `as_mut_unchecked` do not enforce
  borrowing or synchronization. `replace` requires no other live reference to
  the wrapped value and no race; `as_ref_unchecked` permits neither an active
  mutable reference nor mutation while the returned shared reference lives;
  `as_mut_unchecked` permits no other references or outside mutation during its
  exclusive borrow. Prefer safe `get_mut` where available and bind exposed
  references to the enforcing guard lifetime. Do not replace checked `RefCell`
  or `Mutex` ownership to silence a borrow error. Keep active WinDNS/OVERLAPPED
  storage raw while native code owns writes; cancellation is not completion.
- `conjure_zst` requires an inhabited zero-sized type and valid semantic use.
  Zero size does not authorize bypassing initialization. The private
  [Apartment token](../../../../../crates/nevrotick-ui/src/native_notifications_windows.rs)
  records successful `RoInitialize` and owns matching thread-bound teardown;
  retain its checked constructor. Never conjure an uninhabited type.
- Windows `ChildExt::main_thread_handle` borrows from `Child`; it transfers no
  handle ownership or completion guarantee. Do not close it or put the borrowed
  raw handle in an owning wrapper. `CommandExt::show_window(u16)` sets
  `STARTF_USESHOWWINDOW` and `STARTUPINFOW.wShowWindow`, not creation flags.
  Use documented startup values; `STARTUPINFOW` excludes `SW_SHOWDEFAULT`.
  It does not replace `CREATE_NO_WINDOW`, runtime UI `ShowWindow`,
  job/mitigation policy or waiting for actual exit. Adopt only for a real consumer.

Sources: [allocator contract](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/core/src/alloc/mod.rs),
[UnsafeCell](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/core/src/cell.rs),
[zero-sized construction](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/core/src/mem/mod.rs)
and [Windows process APIs](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/std/src/os/windows/process.rs),
[their implementation](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/library/std/src/sys/process/windows.rs)
and Microsoft's [STARTUPINFOW contract](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-startupinfow).

## Verify the changed contract

Use [1.100 lint enforcement](../../../testing/references/rust-1.100-lints.md) for
Cargo policy and new diagnostics; Rust warning denial is not Cargo lint policy.
Actual code changes: use [configured tooling](../../../testing/references/configured-tooling.md);
preserve target/feature/native boundaries. Reuse existing
behavioral controls for equivalent expressions; add focused evidence only for a
changed cleanup, error, ownership or enforcement guarantee. A compile-failure
control must fail for the intended diagnostic and retain a valid companion.

Route changed admission/equality rules to [contract tests](../../../testing/qa-contract-tests/SKILL.md),
allocation/aliasing and native memory to [memory diagnostics](../../../testing/references/windows-memory.md),
refusal/partial failure to [negative-path checks](../../../testing/qa-negative-paths/SKILL.md),
completion/scheduling to [lifecycle checks](../../../testing/qa-state-concurrency-lifecycle/SKILL.md)
and reached Windows behavior to [Windows evidence](../../../testing/references/windows-evidence.md).
Use only the owners and admitted checks needed for the changed guarantee.

Report actual change and evidence: clearer expression, automatic cleanup,
typed impossibility or executed lint rejection. Skill/source review alone proves
no compiler pass, native completion, performance gain or measured reduction in
AI defects. Documentation-only changes need package/link checks and proportionate
instruction-use review; they do not initiate product or formal execution.
