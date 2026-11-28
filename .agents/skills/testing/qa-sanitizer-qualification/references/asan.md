# AddressSanitizer qualification

Use for reached address/lifetime defects: out-of-bounds heap, stack or global
access, use-after-free, supported use-after-scope/return, invalid free and
allocator mismatches. Follow [the coordinator](../SKILL.md) for checkpoint timing
and disposition, [shared sanitizer rules](../../references/sanitizers.md) for
controls/evidence, and [configured tooling](../../../../../TOOLING.md#standalone-sanitizer-detector-controls)
for actual admission. This manual describes no new executable runner lane.

## Rust and Linux profile

- Select the exact analysis compiler, explicit target, features, lockfile,
  instrumentation flags, linker and runtime. Rust's `-Zsanitizer=address` is an
  unstable compiler facility; the shipping compiler/profile remains separate.
  An explicit target keeps target instrumentation flags off host build scripts
  and procedural macros. Use the admitted invocation rather than replacing its
  complete flag vector with ambient `RUSTFLAGS`.
- Map the selected product code, standard library, native dependencies and
  allocators to their instrumentation. ASan can work with partially instrumented
  programs, with reduced detection coverage. Record uninstrumented modules and
  custom pools; an instrumented caller does not instrument a foreign callee.
  Rebuild/instrument the standard library when required by the qualified scope;
  the existing standalone `no_std` controls qualify no application `std` graph.
- For mixed Rust/C/C++ code, admit compatible compiler runtimes and the actual
  final link. Rust's `-Zexternal-clangrt` delegates runtime linking; it supplies
  neither an external runtime nor compatibility by itself. Preserve protections,
  isolated outputs, read-only inputs and the configured resource limits.
- Keep ASan separate from TSan/MSan builds. Linux ASan commonly includes LSan,
  but verify the linked runtime and effective leak options through the actual
  invocation. A `cargo-fuzz run` default need not apply to direct binary execution.
  Use [LSan](lsan.md) for leak-specific scope and controls.
- Exercise actual entry points and ownership transitions relevant to the module:
  accepted/rejected inputs, allocation growth boundaries, partial construction,
  failure, cancellation, delayed completion and release. Keep independent semantic
  assertions: memory instrumentation establishes no protocol/authorization truth.
  Safe Rust bounds-check panics and numerical assertion failures are not ASan
  detector controls.

Before relying on a new or materially changed profile, require the admitted
mechanism-specific faulty control and repaired healthy neighbor from the shared
workflow. Retain the ASan diagnosis and intended source site, allocation/free
stacks where applicable, exact options and native outcome. A generic signal,
panic, timeout or nonzero exit is insufficient. Keep detector faults in owned
test-only fixtures; do not add deliberate unsafe code to production owners.

Admit matching symbols and symbolizer provenance. Missing symbols, interception
failure or startup failure constrain the result; retain raw output and module
identity instead of treating an empty report as a pass. Preserve the first
diagnostic and rerun the repaired case with its healthy neighbor. Suppressions,
disabled checks and recover/continue settings are explicit coverage changes,
never a way to satisfy a failing checkpoint.

## Native Windows qualification

**Establish the exact configuration.** The
[Rust 1.100.0 target source](https://github.com/rust-lang/rust/blob/1.100.0/compiler/rustc_target/src/spec/targets/x86_64_pc_windows_msvc.rs)
admits ADDRESS for `x86_64-pc-windows-msvc`, while that version's
[sanitizer guidance](https://github.com/rust-lang/rust/blob/1.100.0/src/doc/unstable-book/src/compiler-flags/sanitizer.md)
omits Windows from the ASan list. The
[Rust Fuzz Book](https://rust-fuzz.github.io/book/cargo-fuzz/windows/setup.html)
documents Visual Studio C++ tools and the ASan runtime component. These sources
justify checking a native configuration; none qualifies the installed compiler,
runtime or product. Microsoft's x86/x64 Windows support statement describes its
MSVC implementation, not every Rust target/link route.

Record native OS/build, target, compiler/LLVM, linker, CRT mode, required runtime
DLLs, loader paths, symbols, mitigations and actual process token. Modern MSVC
ASan needs its runtime DLL even with a statically linked CRT; inspect the actual
Rust/native link instead of assuming identical packaging. Keep diagnostic and
shipping artifacts separate. A diagnostic runtime/load failure proves no product
pass and authorizes no mitigation or package-image-policy relaxation.

**Identify native coverage.** Name the instrumented modules, uninstrumented DLLs,
allocator interfaces, FFI ownership/release APIs and module lifetimes. Custom
allocators need qualified integration; ASan does not automatically manage their
shadow state. Partially sanitized processes have coverage gaps, and unloading
all ASan-enabled DLLs can leave dangling interceptor references. Treat callback,
load/unload and cross-module ownership cases explicitly. MSVC runtime defaults
can differ from Clang's: verify selected checks/options rather than transplanting
a Linux configuration. An ASan pass supplies no Windows handle/GUI-resource
retention or actual WinDNS completion/cancellation evidence.

**Keep diagnostic profiles distinct.** Repository policy is to qualify native
ASan separately from the existing Application Verifier Basics/full PageHeap
profile. Their allocator interception and debugger behavior differ; combining
them requires separate compatibility and mechanism evidence. This is a
conservative selection rule, not an upstream claim of universal incompatibility.
Microsoft documents handled first-chance access violations while x64 ASan commits
shadow memory. Admit its debugger exception policy independently; do not copy an
ASan exception mask into the PageHeap lane that relies on access-violation stops.

Use [Windows memory diagnostics](../../references/windows-memory.md) for owned
per-image settings, elevated configuration, intended-token execution, CDB capture
and verified restoration. That reference also owns PageHeap overrun/underrun
scope, the `/leaks` versus `/full` distinction and AppVerifier resource limits.
Use separate lifecycle measurements before teardown for retained allocations,
handles, GUI objects and tasks; detector memory/timing overhead is not the
uninstrumented resource budget. Linux/WSL evidence cannot replace native Windows
observations. Existing execution holds and owned disposable-Windows requirements
remain in force.

Primary Windows behavior: [support](https://learn.microsoft.com/en-us/cpp/sanitizers/asan?view=msvc-170),
[limitations and debugger behavior](https://learn.microsoft.com/en-us/cpp/sanitizers/asan-known-issues?view=msvc-170),
[runtime/allocators](https://learn.microsoft.com/en-us/cpp/sanitizers/asan-runtime?view=msvc-170),
[linking](https://learn.microsoft.com/en-us/cpp/sanitizers/asan-building?view=msvc-170),
[runtime options](https://learn.microsoft.com/en-us/cpp/sanitizers/asan-flags?view=msvc-170).
The [source register](../../SOURCES.md) records the checked upstream material;
rolling documentation is not execution evidence.
