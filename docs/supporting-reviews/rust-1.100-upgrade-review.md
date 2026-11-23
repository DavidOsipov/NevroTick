# Rust 1.100 upgrade research — 11 November 2026

**Status:** source research and upgrade preparation, not an adopted compiler,
executed compatibility check or release qualification. NevroTick baseline:
`bc0f452cb5075cd8b0e52f8fbc119f28d6037cd5`, Rust **1.99.1**, edition **2024**,
resolver **3**, Windows product target **x86_64-pc-windows-msvc**.

## Source identity and completeness

Rust 1.100 is scheduled for **12 November 2026**, after this review. The inspected
[release-notes PR #165522](https://github.com/rust-lang/rust/pull/165522) was still
open. Its reviewed [notes at 234c935d00a7761bf0bcb7ca727c5ff7ff873baa](https://github.com/rust-lang/rust/blob/234c935d00a7761bf0bcb7ca727c5ff7ff873baa/RELEASES.md#version-11000-2026-11-12)
are the candidate inventory, not a final announcement. The earlier
[draft issue #163811](https://github.com/rust-lang/rust/issues/163811) is discovery
history and can omit later edits. Reconcile the final release/tag before adoption.

Rolling `beta`, `nightly` and `stable` documentation can disagree with dated
release notes or already describe another release. During this review some beta
pages identified **1.101.0-beta.1**. Neither the word `beta` nor a successful
`cargo +beta test` proves testing 1.100. Record the actual `rustc -Vv`, `cargo -V`,
component versions, compiler commit, target, distribution manifest and executable
identities; select a reviewed candidate explicitly through existing admission.

The inventory below covers the candidate notes' language, compiler, platform,
library/API, Cargo, rustdoc, compatibility and test-harness sections, plus the
inspected Cargo/rustfmt material. It is not a review of every merged compiler
commit, a guarantee that release notes list every regression, or a claim that
all NevroTick dependencies compile. The linked upstream notes retain the complete
API name list; no API is adopted by appearing here.

## Compatibility and behavior ledger

| Change and primary evidence | NevroTick disposition and discriminating check |
| --- | --- |
| [`!` stabilization, `Infallible` alias and fallback](https://github.com/rust-lang/rust/pull/155499) | Edition 2024 already selected the new fallback, but that does not exempt transitive older-edition code, custom trait implementations or aliases. Inspect the selected runtime/build/proc-macro graph; exercise inference and overlapping-impl failures with legal neighbors. Do not downgrade edition or suppress warnings to hide a failure. |
| [`Box` allocator-generalized impl inference](https://github.com/rust-lang/rust/pull/161946), [closure capture in `if let`](https://github.com/rust-lang/rust/pull/154210), [dyn-compatibility checks](https://github.com/rust-lang/rust/pull/154992), [orphan checking solver](https://github.com/rust-lang/rust/pull/160668) | Compile actual affected generic/closure/trait consumers and vendored patches. Use local type annotations only where they preserve the intended contract; compiler acceptance alone does not establish unchanged capture/drop behavior. No concrete broken NevroTick dependency is established by this survey. |
| [`let-else` macro grouping](https://github.com/rust-lang/rust/pull/158515), [removed `box` pattern syntax](https://github.com/rust-lang/rust/pull/156749), [enum-layout query cycles](https://github.com/rust-lang/rust/pull/157562) | Include macro-expanded/generated code and compile-fail fixtures. Classify a failure by its intended rule rather than counting every compiler rejection as a successful negative test. |
| [Repeated `repr`](https://github.com/rust-lang/rust/pull/157036), [`drop_in_place` lint coverage](https://github.com/rust-lang/rust/pull/160229), [`expect(dead_code)` behavior](https://github.com/rust-lang/rust/pull/161005), [`aligned_fields_in_packed`](https://github.com/rust-lang/rust/pull/162160), [ambiguous glob trait imports](https://github.com/rust-lang/rust/pull/161805) | The workspace denies warnings and unfulfilled expectations. Qualify its exact lint policy and seeded controls; retain narrow justified expectations, never blanket-allow a new group. A warning-to-error under local policy is distinct from a language hard error. |
| [Rustdoc table lint](https://github.com/rust-lang/rust/pull/159583) | `rustdoc::all` makes this relevant even when product Rust compiles. Run the admitted documentation/doctest lane; repair malformed tables without loosening documentation lint policy. |
| [`str::escape_debug` grapheme extenders](https://github.com/rust-lang/rust/pull/158303) and candidate standard-library Unicode **18.0.0** | Inspect literal diagnostic/formatting and character-classification expectations, including generated/localization inputs. Do not silently replace independently pinned Unicode/catalog data or treat debug escaping as protocol serialization/redaction. Qualify changed output against its real consumer. |
| [`Iterator::min/max` ordering calls](https://github.com/rust-lang/rust/pull/160203) | Correct `Ord` implementations retain their contract. Check ordering consistency and ties where used; do not update an expected result merely because the candidate and an inconsistent implementation agree. No product speedup is measured. |
| [Rust ABI SSE register change](https://github.com/rust-lang/rust/pull/161583) | Relevant to qualifying x86/x64 Rust-call boundaries and specialized/native code. It is not a promise of stable Rust ABI or a statement that Windows `extern "system"`/C ABI changed. Rebuild coherent artifacts and re-run affected boundary controls; retain v3 and the complete mitigation vector. |
| [Sample-profile optimization flag](https://github.com/rust-lang/rust/pull/155942) | A new optional compiler facility, not automatic PGO or a reason to change current O3/ThinLTO/one-CGU settings. Profile provenance, representative workload and native before/after evidence stay with the existing optimization workflow. LLVM 23 was already part of 1.99, not a new 1.100 benefit. |
| [`extern "custom"`](https://github.com/rust-lang/rust/pull/158504) and [volatile non-abstract-machine accesses may trap](https://github.com/rust-lang/rust/pull/160564) | No automatic adoption in protocol/domain code or Win32 wrappers. New low-level facilities do not waive unsafe preconditions, ownership, panic/ABI or OS-access contracts. |
| [Test-harness repeated options](https://github.com/rust-lang/rust/pull/161312) | Later `--test-threads`/`--color` values override earlier ones. Inspect composed wrapper arguments so caller-supplied duplicates cannot change an admitted concurrency policy; record effective command and actual test discovery. |

## New library opportunities and scope exclusions

The [allocator stabilization](https://github.com/rust-lang/rust/pull/156882)
adds a subset of allocator-aware collection facilities; the remaining allocator
extensions are not all stable. That is not evidence that replacing
the current allocator or introducing arenas improves NevroTick. Retain bounded
allocation/retention measurements and explicit deallocation/allocator identity;
no incidental unsafe or global allocator change.

Other candidate API groups include allocation-preserving Box/Rc/Arc operations,
cleanup and cell helpers, trim operations, Vec construction, boolean/Once helpers
and additional comparison/never-type implementations. Consider them only at a
real affected call site after admission, with exact signature/MSRV and semantic
tests. In particular, new unchecked `UnsafeCell` operations and zero-sized-value
facilities do not bypass the existing unsafe review. New trim helpers are not
interchangeable with a protocol parser merely because names are similar.

Windows candidates include borrowed main-thread handle access and command window
configuration: [ChildExt](https://doc.rust-lang.org/beta/std/os/windows/process/trait.ChildExt.html)
and [CommandExt](https://doc.rust-lang.org/beta/std/os/windows/process/trait.CommandExt.html).
Their candidate presence is checked against the dated notes; the rolling pages
alone are not version proof. Any later replacement of manual Win32 code must
preserve borrowed-handle lifetime, ownership, required rights and child startup
semantics. No wrapper is replaced in this documentation update.

The official [32-bit Windows announcement](https://blog.rust-lang.org/2026/10/02/demoting-i686-windows-targets-to-std-only/)
removes host tools from i686 MSVC (still Tier 1) and i686 GNU (still Tier 2);
cross-built standard libraries remain. This is **not** demotion of
NevroTick's x64 MSVC target. The candidate notes' Windows-gnullvm static libunwind
change is likewise not an instruction to change MSVC panic/unwind policy.

Other recorded platform changes concern wasm32-wasip3, PowerPC PS3, Xtensa,
ARM hardfloat/ABI feature validation, x86 softfloat SSE/AVX rejection, Fuchsia
Cortex-A53 mitigation and WASI SDK 34. The library list also contains ENOTSUP
mapping and an SGX-specific waitqueue soundness fix. They establish no defect in
the selected Windows product lane. Revisit only if the actual host, target or
dependency configuration reaches them; do not expand the product platform matrix.

## Cargo and companion tools

| Candidate change | Existing owner and action |
| --- | --- |
| [Intermediate layout v2](https://github.com/rust-lang/cargo/pull/17354) | Extend R4-B-02 in the existing build task, not a second migration plan. Cargo JSON discovery must retain identity, containment and byte checks; existing path consumers are listed below. Final EXE/PDB exports are separate from intermediate layout. |
| [Cargo lint configuration and unused-dependency checking](https://github.com/rust-lang/cargo/pull/17298) | Qualify the new `[lints.cargo]` surface separately from rustc/Clippy policy. An unused-dependency report is not authority to remove feature-, target-, build-script- or macro-dependent functionality; test actual selected rows before any graph edit. |
| [Minimum publish age](https://github.com/rust-lang/cargo/pull/17335) | Existing Cargo watchlist owns evaluation. Locked versions are exempt; delay is not malware detection and may delay a security fix. No age value or registry policy is adopted here. |
| [`cargo install` packaged lockfile default](https://github.com/rust-lang/cargo/pull/17388) | Review tool-acquisition recipes and resulting exact graph. Its `--locked` becomes a no-op; this does not make `--locked` a no-op for normal workspace build/update commands. Retain verified executable/tool identity. |
| [Cargo 1.100 fixes](https://doc.rust-lang.org/nightly/cargo/CHANGELOG.html#cargo-1100-2026-11-12) | Inspect the named section for Git acquisition retry/429 behavior, repository-configuration handling, relative-symlink manifest handling and Windows `cargo run` diagnostics as applicable to the actual lane. Nightly-only removal of experimental `update --breaking` is not loss of a stable command. |
| [Rustfmt changelog](https://github.com/rust-lang/rustfmt/blob/main/CHANGELOG.md) | Candidate 1.11.0 notes dated 12 November include attribute retention, workspace/manifest-path behavior, closure/range formatting and comment/panic fixes. Confirm the exact rustfmt shipped with the admitted Rust candidate, then inspect formatting-only changes; no mass reformat now. |
| [Clippy changelog](https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md) | No trustworthy complete 1.100-specific section was established in this review. Do not rebrand all `Unreleased` entries as 1.100. Resolve the candidate component commit and qualify the current strict lints/fixtures before claiming coverage. |

Do not import `build.profile`/`install.profile` or the OUT_DIR-leakage fix into this
1.100 inventory merely from a rolling configuration page: the inspected Cargo
changelog assigns them to **1.101**. Reconcile version disagreement against the
admitted candidate rather than silently choosing whichever page is newer.

## Repository integration and remaining evidence

The [existing build task](../../tasks/build-optimization.md#rust-1100-candidate-admission--11-november-2026)
owns the actionable upgrade sequence and results. Current pins and admission
consumers include `rust-toolchain.toml`, `Cargo.toml`, `policy/tool-manifest.json`,
`.github/security/prepare_rust.py`, `scripts/check-environment.py` and hardcoded
toolchain paths in `.github/workflows/rust.yml`. Compiler commit/LLVM/distribution
and binary hashes are verified inputs, not strings to substitute automatically.

R4-B-02 already identifies allocation/Core/engine/time-domain intermediate-path
consumers. Some use JSON and then validate old path shapes; finding JSON parsing
does not establish new-layout compatibility. The isolated Kani consumer's own
new-layout handling and receipts do not qualify the product compiler. Formal
execution remains post-V1; Windows, resource, fuzz and storage holds remain.

This research executed no candidate compiler, formatter, lint, test, dependency
update or native program. It establishes no current NevroTick breakage, speedup,
security fix or release readiness. At adoption, preserve the 1.99.1 baseline and
compare candidate behavior; revalidate final release changes and explicitly close
or retain the unresolved companion-tool and native evidence.

## Final-release implementation follow-up — 13 November 2026

The owner now requests the upgrade. The implementation starts from clean commit
`a25696b8bf67a53ebbcf40c4c0ba43c613159ff1`, after the receiver source closeout,
under the existing R4-B-02 / CHECK-19, CHECK-20 and CHECK-26 owners. The preceding
11 November review and its unexecuted candidate rows remain historical.

The supplied final-release review identifies Rust source
`0ef1bc386059596dada8fb65b50425aab2ea637a` and its Cargo submodule
`3acc79c0c841844675bc53a605922dbfc441e750`. Acquisition independently retrieved
the official 12 November distribution manifest and checked its published SHA-256
`3051e859570f64a8c5acb9d09e87425db52c7d97287c67e8f6d57d7d34ef9258`.
Executable/component admission and actual compatibility results belong to the
[existing build task](../../tasks/build-optimization.md#rust-1100-implementation--13-november-2026).
Neither source identities nor the review's synthetic controls supply a build pass.

The bounded migration preserves the lockfile, dependency features, patches,
v3 floor, overflow checking, unwinding, symbols and mitigation configuration.
It aligns the declared minimum with the sole qualified shipping compiler,
1.100.0; ongoing minimum-version qualification on 1.99.1 is not selected.
The preserved 1.99.1 installation and clean starting commit provide rollback.

The six reached artifact boundaries are allocation-probe custom-build/library
selection; Core build-script output admission; Core, time-domain and Engine
ordinary contract-dependency collection; and the Windows Engine libtest selector.
Cargo messages must retain package, target, profile, host/target-role, containment,
inventory and byte custody. A new layout is not permission to accept arbitrary
paths or transfer old Engine role metrics to the changed receiver.

The final-review corrections constrain implementation: keep the intentional
`i-slint-core` feature carrier unless actual selected-row evidence requires a
change; no exchange-copy hash collision or new product vulnerability was shown.
Do not speculatively edit the numerical `NonZero` use or lifecycle boolean branch.
The released Clippy inventory adds `option_zip_none` and `needless_nonzero_get`
within the existing lint scope, subject to actual execution. Cargo warnings have
their own enforcement surface; Rust's `-D warnings` supplies no Cargo-lint policy.
Explicit Cargo lints, allocator/DropGuard/trim refactors and sample PGO remain
separate optional work. The later [OD-19 decision](../nevrotick-owner-decisions.md#23-od-19--14-day-registry-dependency-cooldown)
selects fourteen days for registry version uptake; its complete configuration,
changed-graph admission and isolated fixtures now belong to the existing build
task. The earlier no-delay research status is historical. Toast callback-payload disposal remains
with the existing R4-G-02 owner. No native, fuzz, storage, UI or post-V1 proof
hold changes. Actual receipts and remaining qualification stay in the build task.

## Stabilized API reconciliation — 23 November 2026

**Scope:** source-only disposition of all **47 supplied API entries** against
NevroTick `a0b461de628a690d9c8b32ebb79677ed5276ceff` and Rust
`0ef1bc386059596dada8fb65b50425aab2ea637a` (1.100.0). Three independent reviews
covered allocator, value/convenience and ownership/native groups; a distinct
editor integrated the reusable deltas. The preceding 11 November candidate
review and 13 November adoption follow-up remain historical. This reconciliation
does not repeat compiler adoption or transfer earlier execution receipts.

The counts below track the supplied inventory, including its separate trait
implementations. In particular, `System` has existed since 1.28; its new
`Allocator` implementation is a separate entry. Stable ordinary use does not
imply stable const use or stabilization of neighboring extension APIs.
**Prefer when touched** means a matching existing operation can be expressed more
directly; **conditional** requires a justified consumer and the existing owner
checks. Neither disposition schedules a product rewrite.

The [feature-contract reference](../../.agents/skills/implementation/implement-rust/references/rust-1.100-contracts.md)
owns operative rules; the [source register](../../.agents/skills/implementation/SOURCES.md#rust-1100-stabilized-api-reconciliation--23-november-2026)
links the exact allocator, Box/Vec/Rc/Arc, value-trait, trimming, cell, cleanup and
Windows implementations plus Microsoft's `STARTUPINFOW` contract.

### Allocator entries — 23

| Count | Exact entries | Disposition and current-source implication |
| --- | --- | --- |
| 3 | `Allocator`; `Global`; `System` | **Conditional.** Stable allocation interface does not supply a budget, zeroization or measured benefit. No new custom-allocator requirement was established. Keep the engine's unsafe prohibition. |
| 2 | `Allocator for Global`; `Allocator for System` | **Conditional.** Global and System allocation are not interchangeable; per-collection System can bypass the existing `GlobalAlloc` observer. Preserve its instrumentation and observation scope. |
| 2 | `Allocator for &A`; `Allocator for &mut A` | **Conditional.** The reference is equivalent to its allocator referent and preserves its lifetime; this proves no unrelated allocator or native-memory ownership. |
| 3 | `Allocator for Box<T, A>`; `Allocator for Rc<T, A>`; `Allocator for Arc<T, A>` | **Conditional.** Forward to wrapped allocator `T`, not smart-pointer storage allocator `A`. Ordinary `Arc::new(allocator)` can support `Vec::new_in`; `Rc::new_in`, `Arc::new_in` and their explicit allocator type parameters remain `allocator_ext`. No current need for shared custom allocation was established. |
| 1 | `Box::new_in` | **Conditional.** Explicit allocator choice, with no allocation-error `Result`; `Box::try_new_in` remains unstable. It cannot replace an existing typed refusal path. |
| 4 | `Box::from_raw_in`; `Box::from_non_null_in`; `Box::into_raw_with_allocator`; `Box::into_non_null_with_allocator` | **Conditional.** Preserve allocator identity, layout, initialization and single ownership across matching handoffs. `NonNull` alone proves no allocation validity. No WinDNS/`LocalFree` allocation may be adopted with `System`. |
| 2 | `Box::allocator`; `Vec::allocator` | **Conditional.** Borrow the collection's actual allocator; this supplies neither ownership transfer nor permission to deallocate unrelated storage. No direct modernization benefit was identified. |
| 2 | `Vec::new_in`; `Vec::with_capacity_in` | **Conditional.** Preserve current fallible reservation before initialization. `Vec::try_with_capacity_in` remains unstable; `new_in` plus `try_reserve[_exact]` preserves reported failures but cannot prevent an allocator's permitted abort. |
| 4 | `Vec::from_raw_parts_in`; `Vec::from_parts_in`; `Vec::into_raw_parts_with_allocator`; `Vec::into_parts_with_allocator` | **Conditional.** Keep pointer, initialized length, actual capacity and returned allocator together. Same allocator type or ordinary `Clone` is insufficient evidence of equivalence. Existing generic raw-parts guidance remains the owner. |
| **23** | **Allocator subtotal** | No automatic allocator replacement or collection migration. |

Current discriminating paths are the
[allocation observer](../../policy/time-domain-windows-allocation-observer.rs),
[NTS response/cookie preparation](../../crates/nevrotick-engine/src/nts_ke_experiment/mod.rs)
and [WinDNS RecordList](../../crates/windows-platform-native/src/windows_dns.rs).
Keep `Zeroizing` + `try_reserve_exact` → `Failure::Allocation` → initialization,
complete cookie preparation before publication, and matching `DnsFree` teardown.

### Value and convenience entries — 13

| Count | Exact entries | Disposition and current-source implication |
| --- | --- | --- |
| 3 | `PartialEq<Cow<'_, [U]>> for Vec<T, A>`; `PartialEq<Cow<'_, [U]>> for &[T]`; `PartialEq<Cow<'_, [U]>> for &mut [T]` | **Conditional.** Content comparison avoids forced Cow ownership; it supplies no domain identity, allocation identity or constant-time guarantee. No current direct Cow consumer was identified. Preserve source-draft identity and session `Arc::ptr_eq`. |
| 1 | `Default for Once` | **Conditional.** Creates fresh `Once::new()` state; retain same-instance and poisoning semantics. Ordinary Default is stable; const Default remains `const_default`, while `Once::new` is already const. No replacement for the static policy `OnceLock<Arc<PolicyCell>>`. |
| 1 | `bool::toggle` | **Prefer when touched.** The engine's `reverse` fairness variable intentionally inverts after the await; preserve that timing. Repeated sticky Stop assignment must never become inversion. |
| 2 | `AsRef<T> for !`; `AsMut<T> for !` | **Conditional generic compatibility.** No constructible value/reference, new infallibility proof or const-call support. Do not erase real errors to use these impls. |
| 2 | `str::trim_prefix`; `str::trim_suffix` | **Prefer for optional single-edge removal.** Source-draft root-dot handling can become clearer while raw validation remains first. Keep required CLI newline rejection; do not introduce repeated trimming. |
| 2 | `slice::trim_prefix`; `slice::trim_suffix` | **Prefer for optional single-edge removal.** DNS reported-name matching can use `reported.trim_suffix(&[root])`. Required query-root admission must still return `DnsNameError::MissingRoot`; implementing `SlicePattern` remains unstable. |
| 1 | `Path::trim_prefix` | **Conditional.** Component matching with unchanged input on no match; no filesystem confinement, canonicalization or symlink resolution. No direct current consumer requiring replacement was identified. |
| 1 | `Vec::from_fn` | **Prefer when touched.** Fixed localization slots (`COUNT = 509`) fit bounded indexed construction; ascending indices, no closure call for zero length. No allocation-error return, so preserve the NTS fallible path. |
| **13** | **Value/convenience subtotal** | Add missing trait/Once distinctions and precise trimming limits; retain existing stronger contracts. |

The [source-draft owner](../../crates/nevrotick-ui/src/source_drafts.rs) separates
`same_identity` from whole-value equality. The
[policy authority](../../crates/nevrotick-engine/src/source_policy.rs) keeps the
single successful process root. The
[DNS owner and controls](../../crates/windows-platform-native/src/dns_address.rs)
pair optional reported-root matching with mandatory absolute query names and
accepted/multiple-root/rootless controls. These are source observations, not
newly executed tests.

### Ownership and native entries — 11

| Count | Exact entries | Disposition and current-source implication |
| --- | --- | --- |
| 1 | `Box::take` | **Conditional allocation reuse.** Returns `(T, Box<MaybeUninit<T>, A>)`; the value has separate ownership and the retained storage is not initialized as T, zeroized or freed. Prefer safe `Box::write` for reuse. No current reuse protocol needs adoption. |
| 3 | `Box::map`; `Rc::map`; `Arc::map` | **Conditional transformation.** Box moves T; Rc/Arc borrow T. Pinned reuse requires equal size/alignment and, for Rc/Arc, one strong owner with no Weak owners. Existing aliases are not retargeted; closures can still mutate permitted interior state. Preserve clone/weak-callback/session identity contracts; no speedup claim. `try_map` remains unstable. |
| 1 | `core::mem::DropGuard` | **Prefer for simple acquired local cleanup.** Existing optional popup guidance is valid: acquire first, release before dependent control. Retain domain guards and fallible teardown. Never complete service `WorkPermit` on scope exit; native-query custody explains why `DnsQueryPermit` can safely release on its own drop. |
| 3 | `UnsafeCell::replace`; `UnsafeCell::as_ref_unchecked`; `UnsafeCell::as_mut_unchecked` | **Conditional reviewed unsafe abstraction only.** Preserve each method's distinct aliasing/mutation preconditions and guard lifetime; prefer safe `get_mut` where available. Active WinDNS/OVERLAPPED buffers remain raw while native code can write. Cancellation request supplies no completion proof. |
| 1 | `core::mem::conjure_zst` | **No current adoption benefit.** Inhabited zero size does not authorize bypassing semantic initialization. Preserve the checked, thread-bound Apartment token and matching teardown. |
| 1 | `ChildExt::main_thread_handle` | **Conditional; no current consumer identified.** Borrows from Child, with no transfer of ownership or process-completion guarantee. Preserve owning-handle and actual-exit contracts. |
| 1 | `CommandExt::show_window` | **Conditional; no current consumer identified.** Sets `STARTF_USESHOWWINDOW` and `wShowWindow`, not creation flags or runtime `ShowWindow`. Microsoft excludes `SW_SHOWDEFAULT` for this startup field; preserve process policy and documented startup values. |
| **11** | **Ownership/native subtotal** | Preserve current owners; new syntax supplies no native or performance qualification. |

The [service permit](../../crates/windows-platform/src/service.rs) and its
[drop/cancellation test](../../crates/windows-platform/tests/service.rs) make
physical completion explicit. In contrast, the
[DNS permit](../../crates/windows-platform-native/src/dns_address.rs) stays with
[NativeQuery](../../crates/windows-platform-native/src/windows_dns.rs) through
completion even if the receiver retires. Existing
[UI weak callbacks](../../crates/nevrotick-ui/src/slint_host/native_workflow.rs)
and [session identity](../../crates/nevrotick-engine/src/session.rs) remain
consumers of the original allocation, not a mapped replacement.

### Integration and evidence limits

**Accounting: 23 + 13 + 11 = 47 entries.** Changes are confined to the existing
feature reference, implementation source register and this supporting review,
with independent review/check evidence in the
[existing skill-quality record](../../.agents/plans/2026-10-30-skill-quality.md#rust-1100-stabilized-api-reconciliation--23-november-2026).
The reference routes future changed contracts, allocation/aliasing, failure,
scheduling and reached Windows behavior to their existing QA owners. This is no
new leaf, product defect finding, allocator choice, measured performance claim
or completed Rust/native qualification. Documentation checks do not initiate
builds, native jobs, proof work or changes to existing holds and gates.
