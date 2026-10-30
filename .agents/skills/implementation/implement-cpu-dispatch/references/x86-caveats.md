# x86 dispatch caveats and discriminating controls

Research checked 30 October 2026 against Rust 1.99.1 and primary platform sources.
Use the sections relevant to the changed boundary. These findings refine the
[skill](../SKILL.md); they do not adopt a dependency, nightly compiler, new loader,
CPU affinity rule or measured speedup. OD-07's intended v3 floor and existing
numerical, privilege and package contracts remain authoritative.

## 1. A separate checker can still inherit the unsupported baseline

Rust's [detector contract](https://doc.rust-lang.org/std/macro.is_x86_feature_detected.html)
returns true for globally enabled features. A v3-compiled prerequisite checker
therefore cannot establish its own baseline support. A second binary in the same
workspace can inherit the same [Cargo flag inputs](https://doc.rust-lang.org/cargo/reference/config.html#buildrustflags)
and target dependencies. Inspect its effective flags and every startup/native
object reached before the decision. A lower-feature attribute does not subtract
global features.

Imported DLL entry points can run before the application's admission code;
see [Microsoft DLL initialization](https://learn.microsoft.com/en-us/windows/win32/dlls/dynamic-link-library-entry-point-function).
Keep v3 payload loading/launch after the compatible prerequisite decision.
Enumerate installer, SCM, UI and direct-launch entry paths: an installer-only
check is not proof about all later starts. Resolve uncovered paths with the
package/platform owner rather than claiming universal graceful refusal.

**Control:** inspect actual compiler/link inputs and relevant emitted startup
code, then observe below-v3 refusal before payload launch on an owned compatible
fixture, with a full-v3 accepted neighbor. A host model name or constant-true
detector cannot supply this observation.

## 2. Conditional compilation and function features are different mechanisms

`cfg(target_feature)` reflects the whole compilation configuration; a function's
`#[target_feature]` does not change it. Gating an optional AVX-512 function on
that cfg can remove it from the v3 binary entirely. Use architecture cfgs where
needed and keep the runtime specialization compiled in the admitted baseline.
Ordinary helpers do not automatically inherit the caller's feature set. Closures
normally inherit it, so an escaping closure also needs its execution precondition.
See the [Reference](https://doc.rust-lang.org/reference/attributes/codegen.html#attributes.codegen.target_feature.remark-cfg)
and [pinned closure handling](https://github.com/rust-lang/rust/blob/58cf8f9c35b355199727627d07ae34a4e2846d67/compiler/rustc_codegen_ssa/src/codegen_attrs.rs#L343-L362).
That compiler has special handling for inline-always closures; do not infer a new
compiler bug or a blanket workaround from it.

Rust prevents feature functions from being inlined into incompatible contexts;
`target_feature` plus `inline(always)` on a function is prohibited. Global and
function feature inputs are [combined by the pinned backend](https://github.com/rust-lang/rust/blob/58cf8f9c35b355199727627d07ae34a4e2846d67/compiler/rustc_codegen_llvm/src/attributes.rs#L422-L433).
Keep the existing ThinLTO policy; inspect the final optimized startup, dispatcher,
baseline and kernel bodies. Optional instructions legitimately occur in kernels:
finding one anywhere in the image is not itself baseline leakage.

**Control:** build without optional global features, establish that the intended
kernel remains present and reached through the real seam on supported hardware,
and compare its final instructions with the effective flags. Pure selector tests
cannot expose a specialization deleted by cfg or a generic helper left unoptimized.

## 3. Callable safety and SIMD call ABI must survive the dispatch boundary

Keep a feature-dependent pointer private in the checked owner, using an
`unsafe fn` pointer where its call needs that contract. A safe callable wrapper,
closure or successful coercion is not evidence of CPU support. Rust's
[call, pointer and Fn restrictions](https://doc.rust-lang.org/reference/attributes/codegen.html#attributes.codegen.target_feature.safety-restrictions)
still apply; OnceLock publication proves initialization, not ISA eligibility.

Prefer slices/scalars and existing domain results at the seam, keeping intrinsic
vector values within qualified bodies. Equal lane bytes do not prove alignment
or call ABI. The pinned compiler explains that the ordinary Rust ABI generally
[passes vectors by reference](https://github.com/rust-lang/rust/blob/58cf8f9c35b355199727627d07ae34a4e2846d67/compiler/rustc_target/src/target_features.rs#L27-L39),
so vector crossing is not inherently undefined behavior. Foreign/indirect calls
need their exact signature, caller/callee features and convention reviewed:
[MSVC x64](https://learn.microsoft.com/en-us/cpp/build/x64-calling-convention)
and [vectorcall](https://learn.microsoft.com/en-us/cpp/cpp/vectorcall) differ.

**Control:** external bypass compile failures with a legal caller, plus actual
native caller/callee interoperability for an admitted SIMD-bearing ABI. Neither
matching layouts nor a successful cross-link establishes that native result.

## 4. Usable OS state is required even for narrower AVX-512 instructions

The [pinned x86 detector](https://github.com/rust-lang/rust/blob/58cf8f9c35b355199727627d07ae34a4e2846d67/library/std_detect/src/detect/os/x86.rs#L151-L183)
checks XSAVE/OSXSAVE before XGETBV, AVX state bits 1 and 2, and additional
AVX-512 state bits 5, 6 and 7. Retain that gate for AVX512VL code operating on
128/256-bit vectors too. Do not replace usable-feature detection with raw CPUID
bits or execute XGETBV without its preconditions. Missing baseline AVX state
rejects v3; missing only optional AVX-512 state can select usable v3.

Windows [GetEnabledXStateFeatures](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-getenabledxstatefeatures)
reports enabled extended state, not every ISA subset.
[SetXStateFeaturesMask](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-setxstatefeaturesmask)
changes a CONTEXT mask and can ignore unavailable fields; success does not enable
processor instructions. Such observations can corroborate native qualification,
not replace the selected function's predicates.

**Control:** distinguish baseline and optional-state failures in pure selection
cases; use actual supported/masked-state fixtures for execution claims. Synthetic
inventories must never authorize a real unsupported call.

## 5. A cached choice has initialization and platform-lifetime contracts

[OnceLock](https://doc.rust-lang.org/std/sync/struct.OnceLock.html#get_or_init)
documents erroneous reentrant initialization, currently a deadlock. An unwinding
initializer panic leaves it uninitialized; later calls may retry. `get()` is
nonblocking but returns None while initialization is underway too. An aborting
artifact has a different failure path. Do not introduce initialization into a
latency-sensitive current-thread engine merely to avoid already-cached probes.

Prefer a bounded, nonreentrant, infallible feature-only initializer, resolved
before the event loop when practical. Trace logging/callback/configuration paths
that could reenter it. Test concurrent cold initialization with harmless kernels
and bounded completion; any deliberate reentrancy/deadlock probe belongs in an
externally timed child. Test panic/retry only under the actual unwind policy.

Probe inside the guest and retain cached choices only within the admitted
CPU/OS/guest contract; do not persist a function choice across machines. Rust's
global feature cache is not a topology/migration monitor. Hyper-V describes
[feature preservation or migration rejection](https://learn.microsoft.com/en-us/windows-server/virtualization/hyper-v/processor-compatibility-mode).
Do not invent per-core probes or affinity requirements from the word hybrid:
the cited [Alder Lake platform](https://edc.intel.com/content/www/us/en/design/ipla/software-development-platforms/client/platforms/alder-lake-desktop/12th-generation-intel-core-processors-datasheet-volume-1-of-2/007/intel-performance-hybrid-architecture/)
documents a shared P/E instruction set. Qualify named migration/topology claims;
repeated macro calls do not independently re-probe a changed guest feature set.

## 6. Cache capabilities without freezing the first request's cost decision

An immutable capability choice can retain a per-call length policy. Do not cache
the first short/long request's profitability decision as a process-wide rule.
Measure direct v3, direct specialized and full dispatch over actual admitted
lengths, tails and maxima; challenge any threshold at T-1/T/T+1 and both
short-then-long and long-then-short order. Thresholds are measured choices, not
constants inferred from an ISA name. Intel's
[SIMD tuning guidance](https://www.intel.com/content/www/us/en/developer/articles/technical/tuning-simd-vectorization-when-targeting-intel-xeon-processor-scalable-family.html)
identifies short-trip loops as a case needing particular care.

Use the actual sparse NTP/NTS cadence as well as bursts, and include subsequent
scalar work. Intel's [packet-processing guide, pp.14-15](https://builders.intel.com/docs/networkbuilders/intel-avx-512-instruction-set-for-packet-processing-technology-guide-1645717553.pdf)
reports generation-dependent frequency/transition behavior; polling-loop gains
do not establish gains for intermittent work. Inspect emitted call/return and
legacy-SSE boundaries before proposing instruction changes. Do not add blanket
VZEROUPPER/VZEROALL or claim a universal AVX-512 penalty. Measure end-to-end
latency, not just an isolated arithmetic loop.

## 7. SIMD memory and arithmetic retain their own exact contracts

An unaligned vector load still reads its full documented width. A mask applied
after an out-of-bounds load does not legalize it. Establish allocation bounds,
initialization, aliasing and the chosen intrinsic's alignment requirements before
access; handle tails with a proven-safe recipe. Check empty/minimum and vector-
width-minus-one/exact/plus-one cases, final allocation boundaries and offsets.
Use an admitted guard-page/sanitizer fixture where needed; an ordinary successful
mapped read is insufficient proof. [Load definition](https://doc.rust-lang.org/core/arch/x86/fn._mm256_loadu_si256.html)
and [pointer offset contract](https://doc.rust-lang.org/std/primitive.pointer.html#method.add).

Check each chosen operation's wrapping/saturating, conversion and FP semantics;
`overflow-checks=true` is not a proof that an explicit SIMD operation implements
the scalar checked contract. Portable SIMD's documented integer reductions wrap,
and its APIs are [nightly-only on the researched compiler](https://doc.rust-lang.org/std/simd/struct.Simd.html).
That documentation adopts no nightly API or new dependency. NaN handling and
signed-zero identity also need the actual operation's contract:
[scalar f64 min](https://doc.rust-lang.org/std/primitive.f64.html#method.min)
does not promise a deterministic zero sign for equal operands. Preserve the
core's stronger explicit promises where present, not an assumed primitive rule.

**Control:** independently test overflow neighbors, signed zeros, subnormal and
rounding boundaries, multiple invalid lanes and prescribed first-error index/
field/operation precedence. Preserve early no-read rejection and failure
atomicity, not just the final numeric value. Synthetic numeric models and tool
availability do not establish actual vector instruction or Windows behavior.
