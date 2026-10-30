---
name: "implement-cpu-dispatch"
description: "Implement or review NevroTick Rust CPU baseline changes and runtime-dispatched x86 kernels with exact feature gates, safe fallbacks and preserved numerical contracts."
---

# Implement CPU-specific Rust paths

Use for CPU-baseline migration, function multiversioning and dispatch safety.
For locating a hotspot or claiming a performance improvement, also use
[performance optimization](../optimize-rust-performance/SKILL.md). Ordinary Rust
changes need no feature-dispatch framework.

Read [the implementation contract](../CONTRACT.md),
[testing contract](../../testing/CONTRACT.md),
[OD-07](../../../../docs/nevrotick-owner-decisions.md#11-od-07--x64-baseline-and-runtime-optimization)
and [hardening §6.7](../../../../docs/windows-time-development-hardening.md#67-v3-baseline-and-runtime-cpu-dispatch).
Use the [existing workstream](../../../../docs/plans/development-plan.md#81-x64-baseline-and-runtime-optimization-workstream)
and [research](../../../../docs/supporting-reviews/x86-runtime-optimization.md)
rather than creating another plan or copying a feature catalogue here.

OD-07 selects an intended **x86-64-v3** minimum with optional runtime-selected
higher-feature functions. Inspect actual configuration: the admitted build may
still use the earlier generic floor until coordinated migration completes.
AVX2 is already part of v3; detecting it does not identify a new higher tier.
The owner decision proves neither artifact compatibility nor measured speed.

For a changed implementation boundary, read the relevant sections of
[x86 caveats and controls](references/x86-caveats.md): checker/startup isolation,
cfg/helper compilation, callable/ABI safety, OS state, cache initialization,
length/cadence decisions and vector memory/arithmetic. Resolve those obligations
at their real owner; do not turn every research topic into an unrelated campaign.

## Admit the baseline and feature boundary

- Trace target configuration and native/build/proc-macro inputs through the
  guarded commands to the final executable/package. Change the target-specific
  CPU value, checker expectations and affected input bindings together. Preserve
  the complete mitigation vector, ABI, overflow checks, panic policy and symbols;
  ambient `RUSTFLAGS` can replace configured protections. Do not use build-host
  `native` or global v4 flags as a shortcut. A Rust flag does not rebuild `std`
  or establish a prebuilt native object's requirements.
- A v3 product cannot execute its own v3-compiled prerequisite check safely on
  every rejected machine. The package/startup owner needs a check compiled for
  a compatible lower baseline before starting any v3 payload. Verify the full
  required CPU **and OS** capability, including masked VM features and usable
  vector state. Keep the complete MSI and existing privilege boundaries.
- For each specialized function, record its explicit enabled features and the
  detector predicates establishing them. Compile variants ahead of time with
  supported `#[target_feature]` attributes; runtime selection does not recompile
  the application. Use Rust's detector, which checks usable OS vector state,
  rather than a CPU-model name or a hand-written raw-CPUID approximation.
  Keep the optional function compiled in the baseline build: whole-crate
  `cfg(target_feature)` is not the runtime gate, and ordinary helpers do not
  inherit the caller's attributes. Inspect the final optimized boundary.
- Check every feature the selected function requires. `avx512f` alone is valid
  for a function enabling only that additional subset; it does not establish
  full v4, which also adds BW/CD/DQ/VL atop v3. AES/PCLMUL/VAES are independent;
  retain the admitted crypto provider's dispatch and trust/licence gates.
  Declaring a narrower feature set is not inherently invalid code: match the
  attributes, intrinsics/callees and runtime guard rather than requiring all five
  AVX-512 flags on every kernel.

## Keep selection separate from execution authority

Choose the smallest auditable dispatch seam and keep repeated detection out of a
measured hot loop where appropriate. Rust already caches runtime feature discovery
using atomic feature bits; warm checks are not per-packet syscalls, and compile-
time-enabled features may fold to true. Measure a direct check before adding a
second cache. If repeated kernel selection matters, resolve an immutable typed
choice/function pointer at the owning boundary and reuse it; use `OnceLock<Kernel>`
when shared lazy initialization is actually needed. Measure warm lookup/indirect-
call cost too: an extra cache is not automatically faster. Prefer these typed
owners over a custom `AtomicPtr`/transmute scheme. Any genuinely needed mutable
atomic publication needs its own ordering, lifetime and feature-safety proof.
Challenge cold/concurrent initialization and warm selection when introducing a
shared cache. A cached choice retains its feature precondition; no public mutable
override may manufacture that precondition.
Keep initialization nonreentrant and bounded; do not freeze a first request's
length-specific profitability decision in a process-wide capability cache.
Unsupported or disabled optional features select the v3 baseline path.
If the baseline's AVX state is unavailable, reject v3 startup instead. Only
unavailable additional AVX-512 state with usable v3 permits the v3 fallback.

A synthetic feature inventory may test a **pure selector**. It cannot authorize
executing unsupported machine instructions. Before any forced optimized test or
benchmark, establish the actual required features through the real detector.
A test override may always disable an optimization. Test selection and native
instruction execution are separate observations.

Keep `#![forbid(unsafe_code)]` in pure owners. Any necessary unsafe call/intrinsic
belongs to a narrow reviewed owner outside that prohibition, behind a checked safe
interface. Establish CPU features, bounds, alignment, aliasing and lifetimes at
the actual call; do not expose a safe raw optimized function that bypasses them.
Reuse immutable validated inputs without duplicating their unchanged validation.
An extra adapter grants no clock, network, persistence or administrative effects.

## Preserve the algorithm and qualify useful work

For each variant, retain the same input/output, rejection and lifecycle contract.
Exact integer timestamp arithmetic stays exact. Preserve prescribed chronological
floating-point operations, intermediate checks, signed-zero/subnormal behavior
and failure atomicity. A changed reduction order or deliberate `mul_add` needs a
separate numerical decision and independent evidence; neither v3 nor AVX-512
authorizes fast-math, reassociation, approximate arithmetic or FTZ/DAZ changes.
Do not claim that enabling an ISA automatically changes or preserves every result.

Use [configured checks](../../testing/references/configured-tooling.md) for the
actual target/features and [numerical procedures](../implement-rust/references/numerical-kernels.md)
when arithmetic changes. Compare against independently specified facts and the
reference path, including valid neighbors, invalid lengths, boundary values,
rejection effects and tails/alignment relevant to the kernel. An optimized path
must not compute its own expected results.

The selected matrix includes v3 baseline, each admitted AVX-512 subset, optional
feature absent/masked, unavailable OS vector state and forced baseline selection.
Observe actual supported Windows execution separately from selection mocks and
cross-builds. Apply the existing critical-file coverage/mutation gates to their
declared scope; empty or unreachable optimized selections provide no qualification.

Predeclare representative workload, artifact/tool identities, metric, guard
metrics, acceptance rule and finite attempts. Measure useful-work latency/CPU,
startup, wakeups/power where observable, retained resources and binary size with
normal protections. AVX-512 availability alone does not make it the preferred
path. Keep a specialization only when numerical checks and the declared native
comparison support it; otherwise retain baseline selection and record the result.
Do not add a new dependency, unsafe allowance or generic framework merely to
produce an optimization-shaped change.

At handoff record actual baseline, feature sets, selected/executed paths, artifacts,
results and missing native/package rows in the existing task. Separate configured,
compiled, selection-tested, instruction-executed and beneficial outcomes. Neither
a probe nor this skill establishes Windows compatibility, CHECK-21 soak or release
qualification.
