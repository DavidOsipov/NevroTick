# x64 baseline and runtime optimization research

Research checked on **30 October 2026**, Asia/Tbilisi. This report supports
[OD-07](../nevrotick-owner-decisions.md#11-od-07--x64-baseline-and-runtime-optimization),
[hardening §6.7](../windows-time-development-hardening.md#67-v3-baseline-and-runtime-cpu-dispatch)
and the [planned implementation workstream](../plans/development-plan.md#81-x64-baseline-and-runtime-optimization-workstream).
Those canonical records own the adopted policy and acceptance gates. The existing
[build-optimization task](../../tasks/build-optimization.md) remains the execution
record for subsequent configuration, artifact and measurement evidence.

## Decision and present implementation

David selected an **`x86-64-v3` minimum**, superseding the earlier v2 proposal,
with optional higher-feature optimization.
The accepted direction is a baseline executable containing separately compiled
functions selected through runtime feature detection. AVX2 is already available
in the selected baseline; an optional v3/AVX2 fallback layer is unnecessary.
Useful AVX-512 paths are to be implemented and qualified for measured workloads;
availability alone does not establish that a path improves NevroTick.

At this research snapshot, `.cargo/config.toml` and its admission checker still
select **generic `x86-64`**. The v3 migration, optional kernels, native compatibility
matrix and comparative performance results are pending. This documentation task
changes no compiler flags, Rust code, dependency graph or historical receipt.
The selected floor is a deliberate compatibility change; supporting a Windows
version alone does not qualify the effective CPU/OS/VM feature combination.

## What Rust supports

Rust can compile selected functions with `#[target_feature(enable = "...")]`
while the rest of the program retains its declared baseline. A dispatcher uses
`std::is_x86_feature_detected!` to choose a compatible precompiled implementation.
This is ahead-of-time function multiversioning: detection neither recompiles the
application nor changes its global instruction requirements. Rust documents both
the [dispatch pattern](https://doc.rust-lang.org/std/arch/index.html#dynamic-cpu-feature-detection)
and [per-function code-generation rules](https://doc.rust-lang.org/reference/attributes/codegen.html#the-target_feature-attribute).

The CPU names are cumulative feature bundles, not interchangeable performance
ratings. The [primary x86-64 psABI definitions](https://gitlab.com/x86-psABIs/x86-64-ABI/-/raw/master/x86-64-ABI/low-level-sys-info.tex)
give the following additional requirements above each preceding level:

| Level | Added architectural requirements |
| --- | --- |
| v2 | CMPXCHG16B, LAHF/SAHF, POPCNT, SSE3, SSSE3, SSE4.1 and SSE4.2 |
| v3 | AVX, AVX2, BMI1, BMI2, F16C, FMA, LZCNT, MOVBE and OSXSAVE |
| v4 | AVX512F, AVX512BW, AVX512CD, AVX512DQ and AVX512VL |

`is_x86_feature_detected!("avx512f")` establishes that particular feature, not
the entire v4 bundle. A kernel enabling only AVX512F can use its exact check;
one compiled with additional features must establish all its requirements.
Omitting an unused extension does not inherently generate invalid code. Use all
five when declaring a full-v4 implementation; a narrower kernel may enable only
its required subsets. Intrinsics and called functions remain part of that safety
contract, so checking just the name of the outer function is insufficient.
Rust's detection macro accepts individual supported feature names, with
[separate checks for multiple features](https://doc.rust-lang.org/std/macro.is_x86_feature_detected.html).

CPU feature bits alone do not establish usable AVX state. Rust's
[x86 detection implementation](https://doc.rust-lang.org/src/std_detect/detect/os/x86.rs.html#181-201)
checks OSXSAVE and XCR0 state before reporting usable AVX/AVX-512 features. Use
that supported detector; do not substitute raw CPUID tests or change OS extended
state. VM masking is assessed from the features available inside the guest.
Missing baseline AVX state rejects a v3 payload before launch. Missing only the
additional AVX-512 state, with usable v3, selects the v3 implementation.

Executing a target-feature function without its required support is undefined
behavior. The Rust Reference also prevents higher-feature functions from being
inlined into contexts lacking those features. Every unsafe dispatch crossing
needs a narrow reviewed safety owner; detection and a comment are not substitutes
for checking the actual callee's complete enabled feature set.

### Detection caching and dispatch ownership

Rust 1.99.1's [pinned feature cache](https://github.com/rust-lang/rust/blob/58cf8f9c35b355199727627d07ae34a4e2846d67/library/std_detect/src/detect/cache.rs#L55-L90)
already stores discovered features in atomics. The x86 cold probe uses CPUID and
XGETBV; warm feature checks consult cached bits rather than making a syscall for
each packet. Features enabled at compile time can resolve to true through the
[macro's documented behavior](https://doc.rust-lang.org/std/macro.is_x86_feature_detected.html).
This is feature caching, separate from caching the chosen application kernel.

For a measured repeated-selection cost, prefer selecting a typed implementation
at initialization and retaining it in its ordinary owner. A
[`OnceLock<Kernel>`](https://doc.rust-lang.org/std/sync/struct.OnceLock.html)
is a supported safe publication mechanism when shared lazy initialization is
needed; reading it or making an indirect call still has costs to compare.
Do not adopt raw `AtomicPtr` conversions as an automatic optimization. Mutable
publication requires an actual use case and ordering/lifetime/safety proof.
Check cold/concurrent initialization, warm lookup, fallback and immutable feature
preconditions. No cache makes a synthetic feature override safe for native
instruction execution. These are implementation choices, not measured speedups.

## Pinned compiler observations and build implications

The subsequent multi-reviewer research is consolidated in the skill's
[x86 caveats and discriminating controls](../../.agents/skills/implementation/implement-cpu-dispatch/references/x86-caveats.md).
It covers inherited flags and DLL startup before prerequisite checks; cfg/helper
compilation and post-LTO instructions; callable/vector ABI safety; AVX512VL OS
state and Windows CONTEXT API meaning; OnceLock initialization and platform cache
lifetime; input-size and sparse-cadence profitability; and vector tail, arithmetic
and rejection contracts. Findings use primary Rust, Microsoft and Intel sources;
the selected controls are proposals until executed on an admitted candidate.
No universal AVX-512 slowdown, required affinity pinning, new compiler defect or
permission to adopt nightly SIMD follows from those sources.

The research queried the hash-admitted **Rust 1.99.1** compiler with
`--print cfg --target x86_64-pc-windows-msvc -C target-cpu=<level>`. This generated
no application artifact. Relative to the preceding Windows CPU selection, the
reported cfg additions were:

| Query | Additional reported target features |
| --- | --- |
| `x86-64-v2` | `popcnt,sse4.1,sse4.2,ssse3` |
| `x86-64-v3` | `avx,avx2,bmi1,bmi2,f16c,fma,lzcnt,movbe,xsave` |
| `x86-64-v4` | `avx512bw,avx512cd,avx512dq,avx512f,avx512vl` |

These are compiler-visible additions over that target's existing defaults, not
replacement definitions of the architectural levels. A cfg query proves neither
whole-package compatibility nor execution on the supported Windows matrix.
The [rustc CPU and feature controls](https://doc.rust-lang.org/rustc/codegen-options/index.html#target-cpu)
govern code generated for the selected compilation; they do not rebuild supplied
standard libraries, external native objects or system DLLs automatically.

During migration, change the admitted target-specific CPU entry and its policy
controls together. Preserve the complete CFG/ASLR/DEP and `/WX` vector, checked
arithmetic, symbols and other selected profile settings. The quoted standalone
`RUSTFLAGS="-C target-cpu=..."` recipe can replace this repository's configured
target flags. It is unsuitable as the product migration command. The selected
future target CPU is `x86-64-v3`; keep `native` and global v4 features out of the
distributed baseline. A higher-feature function still needs exact runtime gates.

Below-v3 rejection must run in qualified lower-baseline-compatible prerequisite
code before any v3 payload executes. It must establish the full cumulative v3
CPU set and OS-enabled AVX state, including the required OSXSAVE/XCR0 conditions.
A check inside an already v3-compiled application does not prove safe admission
on an unsupported processor. Keep the existing
complete-MSI delivery model and lifecycle ownership; this requirement creates no
separate download, installer package or generic privileged execution surface.

## Fit to NevroTick and numerical constraints

The current core has small bounded histories and prescribed chronological
floating-point operations. A useful SIMD benefit is therefore a hypothesis to
measure, rather than a result implied by the feature level. Start with a named
hotspot and its real caller; avoid a generic dispatch framework or a new product
crate solely to manufacture an optimization target.

Preserve private checked construction, typed errors, resource limits, independent
oracles and all existing security gates. The pure core's `forbid(unsafe_code)`
remains. A required unsafe CPU-specific adapter must have an admitted owner and
safe checked interface without weakening that prohibition.

Parallel reductions, changed accumulation order, approximate operations and
deliberate FMA can change numerical results or which checked rejection is reached.
Rust's [`f64::mul_add`](https://doc.rust-lang.org/std/primitive.f64.html#method.mul_add)
has single-rounding semantics. Enabling an instruction feature does not authorize
new arithmetic semantics. Preserve specified results and errors; any proposed
numerical change needs its own reviewed contract and independent expectations.
Do not add fast-math, flush-to-zero/denormals-are-zero, changed rounding modes or
unchecked transformations as an incidental optimization.

AES, PCLMUL and VAES are separate capabilities, not guarantees of the v2/v3/v4
levels above. Prefer the admitted maintained crypto provider's per-feature
dispatch. The ISA decision changes no held TLS/provider, licence, trust or NTS
algorithm-admission gate, and calls for no new cryptographic primitive.

## Required qualification and selection rule

Freeze the selected component, feature sets, source/graph/tool identities, scalar
reference, useful workload, metrics and decision rule before an experiment.
Retain actual selected-path evidence with input/output facts; test-path labels
alone establish no executed instruction coverage.

| Environment/control | Required observation |
| --- | --- |
| Below v3, including a v2-only CPU or masked VM | Lower-baseline-compatible prerequisite rejects before v3 payload execution; no illegal-instruction-based admission scheme |
| Qualified v3 CPU and OS | The v3 baseline completes healthy and hostile cases, including its AVX2-capable code, without requiring AVX-512 |
| Qualified v4 host | Exact supported AVX-512 kernels execute and meet the declared correctness and measurement rule |
| AVX unavailable to OS or guest | Prerequisite rejects before v3 payload execution despite physical CPU capability |
| AVX-512 unavailable but full v3 usable | Dispatcher selects the v3 baseline despite underlying physical CPU capability |
| Forced fallback on capable hardware | Baseline executes through the real dispatcher and retains expected results and failures |
| Invalid/stale/boundary inputs on every usable path | Identical applicable admission, bounds, error/effect and lifecycle guarantees remain observable |

Tests may suppress optional higher-feature capabilities to exercise the v3
fallback. Missing required v3 support must instead fail prerequisite admission.
Tests must never force an unsupported feature true and then execute its kernel.
Pure selector tests and mocked feature masks establish routing logic only; native execution remains
necessary for each claimed instruction path. Inspect final emitted artifacts and
the complete native dependency graph for accidental higher-feature requirements.

Use bounded repeated comparisons under comparable Windows/protection conditions.
Record useful-work latency, CPU time, allocations/private bytes, wakeups/power
where applicable, startup, dispatch overhead and final binary size. Preserve
coverage/mutation and required native evidence for the changed scope. Retain a
fast path only when its declared benefit and guard metrics pass; choose the lower
implementation when evidence favors it. The fallback remains a supported path.
No speedup, native ISA qualification or complete-product gate is established by
this report. Candidate results belong to the existing build-optimization task.
