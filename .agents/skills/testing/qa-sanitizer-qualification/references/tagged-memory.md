# Assess HWASan and memory tagging

Load for a named address/lifetime question on a supported analysis target, or
when an accepted product target makes memory tagging relevant. Use the
[coordinator](../SKILL.md) for checkpoint selection, admission, controls, bounded
execution and dispositions. This reference creates no platform port, execution
profile, hardware prerequisite or additional universal V1 campaign.

## Separate the mechanisms and targets

| Mechanism | Required distinction |
| --- | --- |
| HWASan | Compiler-inserted checks compare address and memory tags. AArch64 top-byte-ignore accommodates pointer tags; HWASan does not require MTE. |
| MemTag/MTE | Arm hardware supplies tag storage/checking. Actual hardware, OS, mappings, allocator, checking mode and selected compiler instrumentation determine coverage. |

Resolve the compiler, target and runtime together. The inspected Rust 1.100.0
[Windows x64 target][rust-windows] declares only ADDRESS sanitizer support.
Its [GNU x64 target][rust-linux-x64] includes neither HWADDRESS nor MEMTAG;
the [GNU AArch64 target][rust-linux-arm64] includes both. These fields establish
code generation eligibility, not installed runtimes or qualified application
instrumentation. Recheck the selected compiler when the profile changes.

LLVM has HWASan implementations beyond ARM, including experimental Linux x86
routes and a limited page-aliasing implementation. That upstream capability
does not establish a Rust x86 or Windows route. Do not infer support from an
architecture label alone, including a future Windows ARM64 target.

For the current native Windows x64 product, record the evidenced unsupported
mechanism scope while retaining the underlying address/lifetime obligations and
their alternative evidence. If an accepted requirement explicitly names the
unsupported mechanism, retain its owner-resolution requirement. Missing admission
for an otherwise required supported profile remains incomplete.

## Select a useful diagnostic question

A supported Linux AArch64 harness may investigate shared portable code before
an ARM product exists. Select it only for an identified additional benefit, such
as an address/lifetime question or campaign resource limit inadequately covered
by the existing profile. Preserve correspondence to the shipping source,
features and relevant dependency graph; portable evidence does not execute
Windows-only allocators, callbacks or FFI.

For HWASan, account for compiler instrumentation, runtime, allocator families,
assembly and uninstrumented modules, and which stack/heap/global accesses the
exact target covers. Record effective options and exclusions. Tag collisions
and granularity constrain detection; a clean bounded run is no memory-safety
proof. Published overhead figures are not NevroTick measurements.

Use the coordinator's independent input/oracle/mechanism controls and require
the intended HWASan diagnosis/site for a detector result. An assertion, generic
signal or a toolchain support bit alone supplies no such evidence. Reuse valid
controls only within their exact instrumentation/runtime assumptions.

## Qualify MTE coverage and deployment separately

Establish the effective hardware/OS configuration, tagged mappings, allocator
and per-thread checking mode. Record covered memory regions and excluded native
components. A successful build or enabled feature flag is insufficient:

- Android allocator-backed heap MTE can work without recompiling all application
  code; compiler stack tagging is a separate choice. Verify both claims against
  the actual platform, rather than imposing one compilation rule on all MTE use.
- Custom allocators must participate in tagging. A standard allocator's support
  does not qualify memory obtained through another allocator or mapping path.
- Synchronous faults provide different localization from delayed asynchronous
  reporting. Observe the actual selected mode and retain its diagnostic limits;
  ignored faults or untagged memory cannot count as covered detection.
- Mechanism controls need the relevant covered memory path and expected MTE
  fault classification, with a legitimate healthy neighbor and failure capture.

If MTE is proposed for production mitigation, route compatibility, final artifact,
startup policy, native lifecycle and measured resource costs through
[build/deployment security](../../../security/security-build-deployment/SKILL.md).
Diagnostic results alone cannot qualify the delivered protection. HWASan's
diagnostic runtime is not a production-hardening selection.

## Source discipline

LLVM's MemTag design page retains explicitly dated October 2019 status text.
Do not reuse those historical hardware/heap-implementation statements as current
support evidence. Use the selected compiler/runtime source together with current
[Linux MTE](https://docs.kernel.org/arch/arm64/memory-tagging-extension.html) and
[Android MTE](https://developer.android.com/ndk/guides/arm-mte) platform guidance.

Primary mechanisms: [LLVM HWASan](https://clang.llvm.org/docs/HardwareAssistedAddressSanitizer.html),
[HWASan design](https://clang.llvm.org/docs/HardwareAssistedAddressSanitizerDesign.html)
and pinned [HWASan architecture handling][llvm-hwasan]. Checked revisions and
source disagreements: [28 November source register](../../SOURCES.md#additional-sanitizers-and-hardening--28-november-2026).

[rust-windows]: https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/compiler/rustc_target/src/spec/targets/x86_64_pc_windows_msvc.rs
[rust-linux-x64]: https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/compiler/rustc_target/src/spec/targets/x86_64_unknown_linux_gnu.rs
[rust-linux-arm64]: https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/compiler/rustc_target/src/spec/targets/aarch64_unknown_linux_gnu.rs
[llvm-hwasan]: https://github.com/llvm/llvm-project/blob/a6321f2d019cc9563b2f552e486b7550d3ec51ce/compiler-rt/lib/hwasan/hwasan.h
