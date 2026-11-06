---
name: "qa-static-checks"
description: "Select and validate NevroTick Rust and verification-Python static enforcement, Clippy, source policy and CodeQL evidence for admitted inputs and targets."
---

# Verify static enforcement

Read [the testing contract](../CONTRACT.md).

Inspect current manifests/pinned toolchain/lint configuration/cfg/feature/target selection/actual CI entry points. Confirm relevant files/generated/native sources admitted. Configured rule/installed analyzer proves no executed enforcement.

For Python verification helpers, collectors or report interfaces, use
[Python verification typing](../references/python-verification-typing.md).
Select the affected language/owner and consumers; a Python-only edit does not
itself require unrelated Rust builds. Preserve configured mandatory gates.

Use [configured tool execution](../references/configured-tooling.md) to choose the actual entry points: environment/policy admission, isolated enforcement controls, then applicable format/compiler/Clippy/rustdoc checks on real source. Run relevant workflow/SAST/secret checks when those guarantees change. Add only required supported matrix entries: member/default/no-default/optional features, Windows target and promised compiler. Do not blindly enable all features or entire Clippy restriction/nursery groups.

Development-policy changes: distinguish the adopted configuration and observed controls in [TOOLING.md](../../../../TOOLING.md) from proposed/application qualification in the [development-hardening guide](../../../../docs/windows-time-development-hardening.md). Run affected strict-policy fixtures/admission tests; configuration alone proves no executed gate. Reading installs no tools, mandates no every-analyzer-per-change campaign.

For pure-library effect boundaries, use
[role-specific API restrictions](../references/role-api-restrictions.md).
Verify effective configuration and lint levels, intended rejection diagnostics
and legal adapter use through the admitted gate; selected bans prove no complete
absence of effects.

Challenge changed enforcement: isolated known violation/valid companion through actual command; check nonzero propagation. Type/visibility/trait contracts need intended compile diagnostic; unresolved import insufficient. Preserve narrow reviewed exceptions/stale-expectation detection on selected compiler.

For a claimed [structural change guarantee](../../implementation/implement-rust-contracts/references/contract-enforcement.md#make-structural-changes-require-review),
start with compiling legitimate code, then add one field or closed-enum variant
in an isolated copy. Keep the selected consumer unchanged while making unrelated
construction sites compile. Require the intended diagnostic at that consumer's
exhaustive field pattern, explicit initializer or enum match, according to the
claimed guarantee. An unrelated missing-field initializer must not qualify a
trait-consumer check. Update the selected consumer with an explicit semantic
decision and require compilation again. Select only the changed guarantee;
ordinary edits need no blanket compile-failure campaign. A compiler rejection
proves change detection, not correct equality, redaction or state policy; retain
their behavioral checks and applicable mathematical proof obligations.

For garde-backed types, verify omitted-rule rejection on the admitted derive
version; reject `allow_unvalidated` and independently review `skip` ownership.
Check actual rule/`dive` coverage separately: metadata-only annotations do not
establish validation, even if the derive compiles.
Compile real external consumer bypasses for private construction/mutation alongside
a valid consumer. These controls prove field/API restrictions only: use
[contract QA](../qa-contract-tests/SKILL.md) to establish actual validation calls,
checked transformations and release-active failure behavior.

For static lock defects, select the guarded experimental Lockbud lane only on
source/features compatible with its separate compiler. Product mathematics
follows [OD-10’s first-release deferral](../../../../docs/nevrotick-owner-decisions.md#141-first-release-deferral--6-november-2026): formal jobs, verifier
extensions and pending proof admission/metadata repairs resume after V1. Missing
proofs do not block V1 acceptance; retain historical exact-scope evidence and
post-V1 obligations in the existing tasks. Miri, Loom and cargo-fuzz retain their
separate execution/model roles through [Rust techniques](../references/rust-techniques.md).
The [proof workflow](../references/formal-verification.md) applies after V1.

For source-to-sink or taint questions, use
[CodeQL data-flow analysis](../references/codeql-data-flow.md). Admit the actual
Rust database and query/model coverage, challenge the requested path property
with controls, and inspect reported paths against current source. Query compile
success or no findings does not qualify an unextracted target or native effect.

Native heap/handle misuse and resource retention require
[Windows memory diagnostics](../references/windows-memory.md) and lifecycle
observations where applicable. Application Verifier/PageHeap are runtime checks,
not static-analysis passes; a clean lint or `mem::forget` ban proves no leak-free
execution.

Read [Rust techniques](../references/rust-techniques.md). Static success cannot prove received bytes, current authorization, runtime cleanup or Windows effects. Documentation/skill edits need their applicable pack checks, not a fabricated product build.
