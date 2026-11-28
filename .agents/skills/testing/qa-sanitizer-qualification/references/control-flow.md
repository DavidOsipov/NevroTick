# Assess control-flow and stack protections

Load for an actual indirect-call/ABI or stack-protection question, a changed
target/dependency graph, or a deployment-hardening assessment. The
[coordinator](../SKILL.md) owns diagnostic admission and checkpoint evidence;
[build/deployment security](../../../security/security-build-deployment/SKILL.md)
owns shipped protections. Keep the existing task and canonical
[hardening §§6.2–6.4](../../../../../docs/windows-time-development-hardening.md#62-target-specific-windows-compilerlinker-settings)
as the implementation and native-qualification authority.

## Identify the exact mechanism

| Mechanism | Relevant guarantee and current scope |
| --- | --- |
| Windows CFG | Constrains indirect-call destinations. Already selected by NevroTick's Rust codegen/linker configuration; full relevant code coverage and effective runtime qualification remain required. |
| Windows CET/HSP | Hardware shadow-stack protection for returns. Selected compatibility/strict-compatibility image declarations require supported CPU/OS/VM and effective process-policy evidence. |
| LLVM CFI | Type-based indirect-call checks in the relevant Rust scheme, with LTO/ABI/dependency constraints. Not the Windows CFG mechanism. |
| SafeStack | Separates compiler-classified stack objects to protect certain control data. Its safe/unsafe regions are not Rust language safety categories or a general memory-error detector. |
| LLVM ShadowCallStack | Separate return-address storage with target/runtime/register-ABI requirements. Not Windows hardware shadow stacks. |

At the inspected Rust 1.100.0 pin, [Windows x64][rust-windows] declares only
ADDRESS sanitizer support; [GNU x64][rust-linux] includes CFI and SAFESTACK.
Thus these extra modes are unsupported for the pinned Rust Windows target, while
supported portable investigations remain possible. This is not a universal
exclusion of every Windows C/C++ compiler configuration; assess an actual native
component's compiler and graph when its protection is in scope. Code generation
support alone admits neither a runtime nor a product profile, and unstable
switches do not authorize changing the stable shipping compiler.

Use the coordinator's evidenced mechanism disposition and retain the underlying
requirement. Required supported profiles lacking admission remain incomplete;
an explicitly required unsupported mechanism still needs its owner's resolution.
Conditional research guidance does not adopt a new all-modules execution gate.

## Preserve current Windows obligations

CFG and CET are existing requirements, not deferred optional sanitizer ideas.
Follow [architecture §15.4](../../../../../docs/windows-time-architecture.md#154-per-process-mitigation-profiles-and-startup-admission)
and the existing build/package/Windows owners. Keep three evidence layers:

1. **Inputs and coverage:** exact compiler/linker settings, `std`, native objects,
   callbacks and the actual loaded-module closure. Linking uninstrumented code
   into a CFG-marked image does not retrofit checks into its call sites.
2. **Artifacts:** actual final image metadata and applicable emitted code. The
   existing [PE checker](../../../../../scripts/windows_pe.py) supplies static
   declaration evidence, not complete instrumentation or runtime enforcement.
3. **Runtime:** per-role launch path, effective policy, enforcement/audit state,
   supported hardware/OS/VM, legitimate callback/unwinding/lifecycle behavior
   and the existing owned, bounded native qualification cases.

Microsoft documents that HSP initially disabled cannot be enabled later through
`SetProcessMitigationPolicy`; upgrading enabled compatibility mode to strict
mode is a separate transition. Resolve startup integration before dependent
work. Module-finalization campaign timing does not postpone required launch-path
design until V1. Image markers do not establish effective HSP, and the required
x86-64-v3 floor does not imply CET hardware or permit a stronger universal floor.
Preserve the canonical EHCONT generation and module-admission conditions.

## Select and interpret an LLVM CFI investigation

Name the actual dispatch/callback/function-pointer boundary and the additional
question the run will answer. Admit the exact analysis compiler, target, LTO,
instrumented `std`/dependencies, FFI type representation, linkage and runtime.
Existing ThinLTO alone does not enable CFI. Mixed Rust/C/C++ protection requires
coordinated type normalization and cross-language LTO; shared-library boundaries
and experimental cross-DSO ABI support need explicit coverage decisions.

Keep the original language/ABI contract when interpreting results. Rust documents
CFI type-grouping restrictions stricter than some otherwise valid Rust ABI calls.
A trap is therefore not automatically a Rust-UB finding. Bind the diagnostic to
the intended check/site, examine type/ABI compatibility, and retain healthy
legitimate calls alongside the selected detector controls. Generic crashes,
missing symbols and incompatible graph setup establish no CFI detection.

A portable CFI run does not execute Windows-only callbacks or qualify CFG/HSP.
Deployment adoption additionally needs final-artifact, compatibility, resource
and native evidence through the existing security/package owners.

## Reconsider SafeStack and ShadowCallStack only for a useful scope

SafeStack's runtime, instrumented-library support and low-level stack operations
constrain adoption. A Linux diagnostic with a different stack layout does not
qualify Windows stack protection. Name the additional evidence or accepted
deployment requirement before preparing a profile.

Current LLVM ShadowCallStack includes AArch64, RISC-V and Hexagon implementations;
the earlier x86_64 implementation was removed. Resolve the selected Rust target,
runtime and reserved-register ABI across all code sharing the thread. A supported
architecture or an introductory documentation label alone is insufficient.
Preserve the distinction from the current Windows CET/HSP obligation.

Primary sources: [Rust CFI/stack guidance][rust-sanitizers],
[LLVM CFI](https://clang.llvm.org/docs/ControlFlowIntegrity.html),
[SafeStack](https://clang.llvm.org/docs/SafeStack.html),
[ShadowCallStack](https://clang.llvm.org/docs/ShadowCallStack.html),
[Microsoft CFG](https://learn.microsoft.com/en-us/cpp/build/reference/guard-enable-control-flow-guard?view=msvc-170)
and [HSP process policy](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_user_shadow_stack_policy).
Checked revisions and source disagreements: [28 November source register](../../SOURCES.md#additional-sanitizers-and-hardening--28-november-2026).

[rust-windows]: https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/compiler/rustc_target/src/spec/targets/x86_64_pc_windows_msvc.rs
[rust-linux]: https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/compiler/rustc_target/src/spec/targets/x86_64_unknown_linux_gnu.rs
[rust-sanitizers]: https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/src/doc/unstable-book/src/compiler-flags/sanitizer.md
