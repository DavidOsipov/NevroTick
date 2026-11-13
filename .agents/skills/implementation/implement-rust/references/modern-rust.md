# Choose modern stable Rust deliberately

Use this reference when a changed Rust path could benefit from a newer language
or standard-library facility, or when reviewing compatibility. Apply the
[implementation contract](../../CONTRACT.md); this reference selects mechanisms,
not new product requirements or permission to change the toolchain.

Reviewed **9 November 2026**, covering stable releases published from
**9 November 2024 through 9 November 2026**: Rust **1.83.0–1.99.1**. The
[source register](../../SOURCES.md#modern-rust-release-review--9-november-2026)
records every feature and patch release in that window. Versions below identify
stabilization, not a new NevroTick minimum. Earlier prerequisites such as raw
references, `#[expect]`, `LazyLock`, `split_at_checked` and non-trait `use<...>`
capture remain useful but are not discoveries from this window.

The **13 November 2026** follow-up covers the admitted **Rust 1.100.0** baseline.
Use [1.100 feature contracts](rust-1.100-contracts.md) for cleanup guards, never
types, precise helpers, allocation/identity and the linked Cargo-lint guidance.
The earlier release survey below retains its historical scope.

## Contents

- [Check the actual baseline](#check-the-actual-baseline)
- [Prefer safe representations for parsing and collections](#prefer-safe-representations-for-parsing-and-collections)
- [Keep arithmetic and time semantics exact](#keep-arithmetic-and-time-semantics-exact)
- [Review edition and borrowing behavior](#review-edition-and-borrowing-behavior)
- [Preserve concurrency and native-work ownership](#preserve-concurrency-and-native-work-ownership)
- [Use modern pointer tools inside existing unsafe boundaries](#use-modern-pointer-tools-inside-existing-unsafe-boundaries)
- [Select optional conveniences by a concrete need](#select-optional-conveniences-by-a-concrete-need)
- [Qualify compiler and build changes](#qualify-compiler-and-build-changes)
- [Finish the changed guarantee](#finish-the-changed-guarantee)

## Check the actual baseline

Read [rust-toolchain.toml](../../../../../rust-toolchain.toml), the
[workspace and member manifests](../../../../../Cargo.toml),
[target configuration](../../../../../.cargo/config.toml) and the affected lane.
At the 13 November follow-up (`8251880a7223e5aa591a317c51196c13316fb7f6`),
the workspace selected **1.100.0**, **edition 2024**, **resolver 3** and Windows
**`x86_64-pc-windows-msvc` / `x86-64-v3`**; the 9 November review used 1.99.1. Verify actual
member inheritance and separate tool workspaces; do not infer their settings from
the root. Re-read these authorities after any update.

Choose a feature only when it improves an identified owner/caller contract, removes
unnecessary unsafe work or a justified dependency, or simplifies a measured cost.
Preserve existing correct implementations when a rewrite adds only novelty.
Keep current stable, beta/nightly documentation and experimental features distinct;
check the exact API's stability rather than assuming all methods on a stable type
are stable. Route graph changes to [dependency updates](../../../environment/dependency-updates/SKILL.md)
and CPU changes to [CPU dispatch](../../implement-cpu-dispatch/SKILL.md).

## Prefer safe representations for parsing and collections

Use fallible safe APIs to couple a structural check to the value consumed by the
next operation. Retain protocol limits, endianness and private domain construction.
Safe library calls can still panic or consume excessive resources.

| Facility | Available since | Use and required constraint |
| --- | --- | --- |
| `slice::as_chunks`, `as_chunks_mut`, `as_rchunks` and mutable counterpart | 1.88 | Represent fixed-width records as arrays. Use a nonzero constant width. Reject a forbidden remainder, or identify the already-enforced exact-length invariant that makes it empty. Never silently discard trailing wire bytes. |
| `slice::as_array`, `as_mut_array` | 1.93 | Obtain an array reference only for an exact length; map `None` into the owner's typed rejection. This does not decode endianness or validate field meaning. |
| `slice::array_windows` | 1.94 | Express overlapping adjacent samples as fixed-size arrays; require nonzero width. A window iterator over too few samples is empty, so retain the separate minimum-sample admission. |
| `slice::get_disjoint_mut` | 1.86 | Mutate bounded, nonoverlapping elements/ranges without raw pointers; handle its error for overlap or invalid indices. Prefer simpler borrowing/splitting when it already expresses the transaction. |
| `HashMap::get_disjoint_mut` | 1.86 | Distinct from the slice API: duplicate keys **panic**, missing keys yield `None`, and the current duplicate check is quadratic. Use a small bounded key set with established distinctness; never expose the panic to hostile input. |
| Slice `split_off*` cursor helpers | 1.87 | Consider for advancing a borrowed input cursor when bounds and error position remain clearer than the existing parser. Preserve whole-message consumption and transactional failure. |
| `bool::try_from(integer)` | 1.95 | Accept exactly 0 or 1 when the wire/schema contract requires it. Do not apply this rule to Win32 `BOOL` or other native result types without their actual binding/API contract. |
| `bool::ok_or`, `ok_or_else` | 1.98 | Express a boolean requirement as `Result<(), E>` when it preserves a specific error. Use lazy error construction where needed; this is not a replacement for mandatory construction or current authorization. |

Primary contracts: [slices](https://doc.rust-lang.org/std/primitive.slice.html),
[HashMap multi-borrow](https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get_disjoint_mut)
and [boolean conversions/results](https://doc.rust-lang.org/std/primitive.bool.html).
Consult the linked method's panic/error section before changing a caller.

Existing examples are
[NTS word parsing](../../../../../crates/ntp-wire/src/nts_negotiation.rs) and
[privilege-record decoding](../../../../../crates/windows-platform/src/privilege_observation.rs):
both already use array chunks behind length checks. Preserve those owners rather
than adding duplicate validators. Adjacent-sample loops in
[regression runs](../../../../../crates/time-core/src/regression/runs.rs) are
possible `array_windows` users when that algorithm is otherwise being changed;
no rewrite or speed claim follows from this observation.

## Keep arithmetic and time semantics exact

Use [numerical kernels](numerical-kernels.md) and the actual unit/range/rounding
contract before substituting arithmetic. Keep errors explicit at raw admission
and preserve bounded transformed values.

| Facility | Available since | Decision rule |
| --- | --- | --- |
| Integer `isqrt` / signed `checked_isqrt` | 1.84 | Prefer an integer result when integer square root is the actual operation. A signed unchecked-domain `isqrt` can panic for negative input; establish the domain or use the checked form. |
| Unsigned/floating `midpoint`; signed `midpoint` | 1.85; 1.87 | Avoid intermediate integer addition overflow where midpoint is wanted. Integer midpoint rounds toward zero; this is not the nearest-ties-even or outward rounding a time/uncertainty calculation may require. |
| `f32`/`f64::next_up`, `next_down` | 1.86 | Express neighboring representable values or independently specified boundary cases. One neighboring step does not establish a complete conservative error bound. |
| `cast_signed`, `cast_unsigned` | 1.87 | Same-width bit reinterpretation, **not range validation**. Use only for a specified bit-level representation with explicit domain justification; never use it to evade checked `TryFrom` conversion. |
| Unsigned `is_multiple_of` | 1.87 | Useful for framing/divisibility; it does not establish a nonzero divisor (`0.is_multiple_of(0)` is true). Keep the width contract. |
| Unsigned `checked_sub_signed`; `checked_signed_diff` | 1.90; 1.91 | Prefer when their result type/range matches the intended signed adjustment or difference. Handle `None`; preserve signedness, units, era and independent mathematical expectations. |
| Integer `strict_*` | 1.91 | These **panic on overflow even without overflow checks**. Do not replace required fallible rejection with them. Preserve `checked_*` plus typed errors at untrusted/numerical boundaries. |
| `Duration::from_nanos_u128` | 1.93 | Avoid truncating nanoseconds to `u64`, but establish the representable duration bound first: the constructor can panic. Keep clock-domain conversion at its existing owner. |
| Floating `algebraic_*` | 1.98 | These allow reassociation and other relaxed optimizations with nondeterministic results. Do not introduce them into clock estimation, uncertainty, source selection or security decisions under the current numerical contracts. Stability and absence of UB are insufficient. |

Sources: [integer square root](https://blog.rust-lang.org/2025/01/09/Rust-1.84.0/),
[signed integer semantics](https://doc.rust-lang.org/std/primitive.i64.html),
[unsigned checked operations](https://doc.rust-lang.org/std/primitive.u64.html),
[duration bounds](https://doc.rust-lang.org/std/time/struct.Duration.html#method.from_nanos_u128)
and [algebraic floating-point semantics](https://doc.rust-lang.org/std/primitive.f64.html#method.algebraic_add).

The current [rounding owner](../../../../../crates/time-domain/src/rounding.rs)
requires nearest-ties-even with exact signed residuals. That is concrete evidence
against a mechanical midpoint substitution. New APIs supply no numerical oracle
or permission to resume the deferred formal-proof work.

## Review edition and borrowing behavior

Treat Rust 2024 (1.85) as the current edition, not a pending migration. Review
semantic changes when adapting older examples or simplifying expressions:

- `if let` scrutinee temporaries drop before `else`; tail-expression temporary
  scopes also changed. Give lock/privilege/handle guards explicit owners and
  scopes when the transaction relies on their lifetime. Shorter syntax alone does
  not prove the same lock coverage or cleanup order.
- Return-position `impl Trait` captures in-scope lifetimes by default in 2024.
  Use precise `use<...>` capture when the exposed API should not retain an
  unrelated borrow. Trait-method support arrived in 1.87; ordinary support
  predates this review window. Explicitly inferred const arguments (`_`, 1.89)
  are a readability convenience, not arbitrary generic const expressions.
- Let chains (1.88) require edition 2024. `if let` match guards (1.95) can clarify
  related conditions, but guarded arms do not establish exhaustive handling.
  Preserve explicit failure reasons and review effects/borrows in each condition.
- Async closures and `AsyncFn*` (1.85) can borrow captured state across their
  returned futures. Use them for an actual asynchronous borrowing contract;
  do not assume ordinary `Fn*` compatibility, `Send`, `'static`, safe spawning or
  reduced allocation. Keep the owning state alive through actual completion.
- Keep 2024's explicit unsafe extern blocks/unsafe attributes and deny-by-default
  `static_mut_refs` rule; they do not replace adapter safety review. Fix
  never-type inference instead of suppressing the strengthened 1.92 diagnostics.

Use the existing [borrow specialist](../../../upstream/rust-borrow-semantics/SKILL.md)
or [Send/Sync specialist](../../../upstream/rust-send-sync/SKILL.md) for a concrete
compiler or lifecycle problem; do not load both for routine syntax edits.
Primary sources: [Rust 2024](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/),
[`if let` scope](https://doc.rust-lang.org/edition-guide/rust-2024/temporary-if-let-scope.html),
[tail scope](https://doc.rust-lang.org/edition-guide/rust-2024/temporary-tail-expr-scope.html),
[opaque-type captures](https://doc.rust-lang.org/edition-guide/rust-2024/rpit-lifetime-capture.html)
and [1.95 match guards](https://blog.rust-lang.org/2026/04/16/Rust-1.95.0/).

## Preserve concurrency and native-work ownership

Apply [Rust/Windows ownership](rust-windows.md). The engine's current-thread Tokio
choice is not changed by a standard-library stabilization.

- Atomic `update`/`try_update` (1.95) may evaluate their closure repeatedly.
  Keep that closure a pure proposed value transformation; perform no logging,
  I/O, privilege change or irreversible effect there. Justify success/failure
  orderings, retained bounds and progress separately. An atomic counter does
  not make an entire state transition atomic. The existing
  [service work admission](../../../../../crates/windows-platform/src/service.rs)
  already uses `try_update` with a checked counter; preserve its lifecycle owner.
- `OnceLock::wait` (1.86), anonymous `std::io::pipe` operations (1.87) and
  blocking `File::lock*` (1.89) block threads. Do not put potentially blocking
  work on the current-thread executor, UI loop or SCM control callback. Use the
  already-admitted bounded worker/lifetime design if the operation is necessary;
  timeouts and dropped futures still do not establish native completion.
- Standard file locks are conditional storage-adapter tools. Review platform/open
  modes, handle clones, repeated locking and failure/cleanup semantics. They do
  not establish path identity, application authorization or crash durability.
  Anonymous pipes do not replace authenticated Windows named-pipe IPC.
- `RwLockWriteGuard::downgrade` (1.92) can preserve protected access while turning
  a writer into a reader; it does not justify adding synchronous locks to async
  paths. `LazyLock::get` (1.94) can inspect without forcing initialization; retain
  explicit not-ready/error behavior rather than inventing a default success.

Use stable atomic aliases such as `AtomicU32`; the generic `Atomic<T>` type shown
in current Rustdoc remains experimental. Sources:
[stable alias](https://doc.rust-lang.org/std/sync/atomic/type.AtomicU32.html),
[its update contract](https://doc.rust-lang.org/std/sync/atomic/struct.Atomic.html#method.try_update-7),
[`OnceLock`](https://doc.rust-lang.org/std/sync/struct.OnceLock.html#method.wait),
[pipes](https://doc.rust-lang.org/std/io/fn.pipe.html),
[file locking](https://doc.rust-lang.org/std/fs/struct.File.html#method.lock) and
[lock downgrade](https://doc.rust-lang.org/std/sync/struct.RwLockWriteGuard.html#method.downgrade).

## Use modern pointer tools inside existing unsafe boundaries

Pure core/wire code continues to forbid unsafe. In a necessary narrow Windows
adapter, use strict-provenance APIs (1.84) such as `addr`, `with_addr` and
`map_addr` for applicable allocation-pointer transformations instead of casual
integer round trips. They preserve information; they prove no validity, alignment,
initialization, aliasing or lifetime on their own.

Distinguish allocation pointers from opaque OS handles, pseudo-handles and numeric
IDs. `without_provenance*` can represent a documented address/ID without granting
dereference permission; `with_exposed_provenance*` is a different, weaker-provenance
operation requiring its own justification. Do not mechanically rewrite the
[tray's native handle conversions](../../../../../crates/nevrotick-ui/src/native_tray/windows.rs)
as though they were Rust allocation pointers. Retain exact native ownership and
matching release APIs. See [pointer provenance](https://doc.rust-lang.org/std/ptr/index.html#strict-provenance).

For new `MaybeUninit`, zeroed allocation, raw-parts, `NonNull`/`Box` conversion or
raw-layout facilities (1.92–1.99), first justify why a safe owning representation
cannot do the job. Zero bytes need not form a valid `T`; partially initialized
buffers require exact initialized lengths, partial-failure cleanup and no
uninitialized exposure. Keep allocator, layout and unique ownership intact.
For an actual ownership handoff, use the matching `into_raw`/`from_raw` or admitted
equivalent, rather than treating `Box::leak` as a temporary handoff and later
reclaiming an allocation while leaked references may still exist.

For a native tagged union, check the discriminant **before** accessing the chosen
field. A single pattern combining tag and union field does not guarantee tag-first
evaluation. This is a documented [1.99 safety correction](https://github.com/rust-lang/reference/pull/2303),
not a demonstrated defect in NevroTick.

Keep raw-pointer diagnostics actionable: `dangerous_implicit_autorefs` became
deny-by-default in 1.89; `dangling_pointers_from_locals` arrived in 1.91;
`raw_borrows_via_references` is allow-by-default in 1.99. Consider the last lint
for a scoped adapter review through the existing tooling owner; do not silently
change workspace policy. Prefer raw-reference construction where a Rust reference
would assert unestablished validity. Debug-only null checks do not validate a
release boundary. Sources: [1.89](https://blog.rust-lang.org/2025/08/07/Rust-1.89.0/),
[1.91](https://blog.rust-lang.org/2025/10/30/Rust-1.91.0/) and
[1.99 notes](https://doc.rust-lang.org/releases.html#version-1990-2026-10-01).

Safe `#[target_feature]` definitions (1.86), safer non-pointer intrinsics (1.87)
and more stable x86 intrinsics (1.89 onward) do not establish CPU/OS eligibility.
Use [CPU dispatch and x86 caveats](../../implement-cpu-dispatch/references/x86-caveats.md);
preserve v3 and exact optional dispatch gates. Do not introduce naked functions,
handwritten assembly or C-variadic definitions merely because they are stable.

Edition 2024 also makes `std::env::set_var`/`remove_var` unsafe to call. Prefer
explicit configuration or a child's `Command::env` when possible, but do not
claim Windows environment mutation is inherently UB: the
[official safety contract](https://doc.rust-lang.org/std/env/fn.set_var.html#safety)
explicitly permits it on Windows even with multiple threads. Portability and the
project's ambient-state rules remain separate concerns.

## Select optional conveniences by a concrete need

| Candidate | Appropriate boundary and limit |
| --- | --- |
| Expanded const evaluation (1.83 onward) | Build fixed immutable tables or validate fixed bounds at compile time when that clarifies the invariant and removes work. Const support is API-specific; it does not imply const traits or arbitrary generic const expressions. |
| Trait-object upcasting (1.86) | Remove a workaround in an existing trait hierarchy where useful. Do not introduce dynamic dispatch, new vtable assumptions or an abstraction solely to use it. |
| `Result::flatten` (1.89), collection extraction/insertion helpers | Use for an existing matching error/ownership contract; retain meaningful error layers, bounded traversal and partial-mutation/drop behavior. |
| `cfg_select!` (1.95) | Express first-matching compile-time branches clearly. Review arm order, fallback and admitted target/feature lanes; it is not runtime CPU detection. |
| `assert_matches!` (1.96) | Improve a test's pattern-failure diagnostic when appropriate. Match expected fields/values as well as variants. Neither it nor `debug_assert_matches!` is production input validation. |
| New `core::range` types (1.95–1.96) | Consider explicit range/iterator separation for a real API need. Ordinary range syntax still produces legacy types; do not assume a migration occurred. Never recover stable bounds from an exhausted legacy `RangeInclusive`. |
| Integer bit-position helpers (1.97) | Simplify actual bitfield operations while preserving width and zero-value handling; retain explicit byte-level protocol decoding. |
| `NumBuffer` / integer `format_into` (1.98) | Consider bounded allocation-free integer formatting for an attributed hot path. Preserve output/redaction and buffer lifetime; claim footprint/speed improvements only after measuring the relevant graph and path. |
| UTF-16 byte decoders (1.98), owned lossy UTF-8 conversion (1.99) | Use only when the input representation and failure policy match. Windows wide paths may contain unpaired surrogates; keep their `OsStr`/wide-string contracts. Lossy text is unsuitable for identifiers, protocol admission or authenticated bytes. |
| `select_unpredictable` (1.88), `cold_path` (1.95) | Treat as measured optimization candidates. They guarantee neither constant-time behavior nor a useful speedup; do not use them as cryptographic controls. |

Use each release's API list in the
[source register](../../SOURCES.md#modern-rust-release-review--9-november-2026)
and the selected method's documentation. For a measurement task, route to
[performance optimization](../../optimize-rust-performance/SKILL.md). Portable
SIMD, experimental float types, remaining allocator extensions, async destruction
and other unstable facilities require exact API/status checking; a stable
neighboring API does not admit them or a nightly toolchain. Rust 1.100 stabilizes
the allocator foundation for `Vec`/`Box`; consult the
[exact stable/unstable boundary](rust-1.100-contracts.md#keep-unsafe-and-windows-apis-within-their-owners)
before selecting a method.

## Qualify compiler and build changes

Retain [execution evidence](execution-evidence.md) and
[Cargo operations](../../../environment/dependency-updates/references/cargo-operations.md).
They own actual commands, inherited features, graph identity and selected lanes.
Review patch notes as well as feature releases. Rust 1.99.1 fixes x86-64 FFI
`bool` return-value code generation; the reviewed NevroTick pin already includes
it. This does not prove the project exercised the defect or make Rust `bool`
interchangeable with every Windows boolean type. Source:
[1.99.1 release](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/).

During an actual toolchain update, inspect compatibility changes affecting the
selected paths: 2024 merged doctests (1.85), cross-target doctests (1.89), v0
symbols/debugger demangling (1.97), Windows thread-local destruction via FLS (1.98),
and Cargo's CI incremental default (1.99). Keep actual executed tests and final
symbols separate from compilation. Retain explicit Windows link dependencies;
1.87 removed an implicit `advapi32` link. Updated OS error mappings, including
post-shutdown `BrokenPipe` in 1.97, require semantic error handling rather than
unqualified exact-message expectations.

For Windows cleanup, account for the
[1.86 removal change](https://doc.rust-lang.org/releases.html#version-1860-2025-04-03):
`std::fs::remove_file` can remove read-only files on recent Windows versions.
The read-only attribute is neither an authorization boundary nor a reliable
deletion-failure fixture. For a removal-denied case, qualify the actual
object/parent permissions and selected operation under the intended token, with
a removable healthy control. See the
[current API contract](https://doc.rust-lang.org/std/fs/fn.remove_file.html).

The already-selected `-C jump-tables=no` became stable in 1.93. Preserve the
complete mitigation flag vector and existing artifact/native qualification;
compiler defaults for another target or tables in prebuilt dependencies/assembly
are not covered by that flag. New path-remapping/build-warning options remain
conditional tooling changes, not permission to replace guarded commands or
suppress diagnostics. Sources:
[release compatibility notes](https://doc.rust-lang.org/releases.html) and
[jump-table scope](https://doc.rust-lang.org/rustc/codegen-options/index.html#jump-tables).

## Finish the changed guarantee

Select the existing owner's tests for the actual semantic change: malformed and
valid neighboring lengths, arithmetic limits/rounding, closure retries, retained
guards, exact feature lanes or real adapter lifetimes. A Markdown-only update
needs skill/link checks and source review, not application compilation. A future
code change needs the applicable compiler, behavioral and native evidence from
the existing testing contract. Preserve all current execution holds and the
post-V1 formal deferral. Report what was used and demonstrated; feature
availability or a release announcement establishes no product qualification.
