# Implementation adaptation sources

Reviewed 28 October 2026. Maintenance/provenance record; load task-relevant linked
conditional reference, not whole file. Repository architecture outranks external
examples. Documentation adaptation performed no Cargo graph, Rust build, parser
experiment or Windows UI execution.

## Numerical and state-update procedures — 30 October 2026

The [numerical procedure](implement-rust/references/numerical-kernels.md) extends
the existing Chrony recipe with independent exact and ordered floating-point
controls, conditioning and contract-specific error behavior. Primary references
remain [Rust f64 arithmetic and mul_add](https://doc.rust-lang.org/std/primitive.f64.html#method.mul_add)
and pinned Chrony
[regress.c](https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/regress.c) /
[sourcestats.c](https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/sourcestats.c).
Caller findings and corrected arithmetic proposals in
[S1.2](../../../tasks/s1.2-regression-readiness.md) motivate the method without
adopting its synthetic thresholds as product policy. No upstream code is copied.

The [state-update example](implement-rust-contracts/references/contract-enforcement.md#prepare-and-commit-owned-state)
operationalizes existing architecture §6 failure-atomicity requirements and the
current exchange owner's staged event. It distinguishes synchronous ownership,
context invalidation and irreversible external effects. These are original local
instruction changes; their checks are recorded in
[the skill-quality task](../../plans/2026-10-30-skill-quality.md), not product qualification.

## Website source mapping

Source root: `/home/david_osipov/personal-website-templated-1/.agents/implementation/`.
Inspected source entrypoints, group router/shared contract; no website edits.

| Website skill | NevroTick skill | Preserved and changed decisions |
| --- | --- | --- |
| implement-typescript | implement-rust | Preserve owner/caller/effect tracing and proportional verification; use Rust ownership, Result, exact newtypes, FFI/async obligations and actual target admission |
| implement-zod-contracts | implement-rust-contracts | Preserve checked-output consumption, invariant ownership, failure atomicity and valid/invalid controls; replace Zod with the architecture's bounded Serde DTO → fallible private domain construction → current effect guard |
| evolve-data-contracts | evolve-data-contracts | Preserve semantic compatibility matrices and separate rollout/retirement; cover versioned native IPC, protected policy/journals, mixed generations and MSI rollback |
| implement-semantic-html | implement-native-ui | Preserve semantic controls, relationships, keyboard/focus and real-consumer evidence; use native Win32/UIA/Narrator/DPI and English-first catalog instead of HTML/ARIA/browser assertions or mandatory website locales |

Copied no browser/ECMAScript corpus, website examples, npm scripts or website-only
enforcement helper into Rust project.

## Project authority

- [Documentation guide](../../../docs/README.md): architecture controls; retained
  reviews and update research do not establish implementation.
- [Architecture](../../../docs/windows-time-architecture.md), sections 5–6:
  bounded exact representation and separate service authorization; section 16:
  native Win32 candidate, accessible behavior and English-first locale; sections
  17–18: bounded work, dependency admission and toolchain policy.
- [Update assessment](../../../docs/windows-time-update-assessment.md): TUF is
  research, not adopted runtime/updater architecture.

## Primary internet and GitHub research

Checked against official documentation/project-owned source. Rolling docs/branches
are dated observations, not pinned artifacts or claims of latest NevroTick dependencies.

| Source inspected | Finding used in the adaptation |
| --- | --- |
| [Serde container attributes](https://serde.rs/container-attrs.html) and [field attributes](https://serde.rs/field-attrs.html) | Unknown fields are otherwise ignored for self-describing formats; deny_unknown_fields conflicts with flatten; fallible conversion is a mechanism, not full input/authority enforcement |
| [serde-rs/json de.rs on GitHub](https://github.com/serde-rs/json/blob/master/src/de.rs) | The fetched source's deserialize_struct dispatches arrays to visit_seq as well as objects to visit_map; Deserializer::end rejects trailing non-whitespace. This independently supports architecture map-only/full-consumption obligations; selected versions/features still need execution fixtures |
| [Rust API Guidelines: type safety](https://rust-lang.github.io/api-guidelines/type-safety.html) | Newtypes distinguish interpretation/units statically; application provenance and permission remain explicit runtime obligations |
| [Rust 2024 unsafe operations](https://doc.rust-lang.org/edition-guide/rust-2024/unsafe-op-in-unsafe-fn.html) | An unsafe function's caller contract and explicit unsafe blocks in its implementation serve separate purposes |
| [Tokio spawn_blocking](https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html) | A started blocking closure cannot be aborted; shutdown timeout stops waiting without cancelling it. Bound real outstanding work and retain admission until its completion |
| [Microsoft UI Automation](https://learn.microsoft.com/en-us/windows/win32/winauto/entry-uiauto-win32) | UIA exposes programmatic UI information for assistive technology and automation; programmatic inspection still differs from Narrator/keyboard user behavior |
| [Cargo test](https://doc.rust-lang.org/cargo/commands/cargo-test.html) | Filtering and runner arguments affect actual admitted tests; doctests have a distinct rustdoc execution path |
| [Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html) | Shared package metadata is inherited explicitly; root metadata does not alone establish each member's settings |
| [Cargo build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html) | Cargo compiles and executes build.rs before package compilation; locked/offline resolution does not make that local code inert |

Connected GitHub retrieved serde-rs/json source. windows-rs README file fetches
returned 404; conclusions rely on none. Microsoft docs/local architecture support
Win32/UIA guidance. Installed no fallback package/tool.

## Error handling and safe diagnostics

Reviewed 30 October 2026 for the original
[implementation procedure](implement-rust/references/error-handling.md). Local
architecture §§3.4, 6.5, 12.3 and 18.1 and hardening §9.8 control ownership,
privacy and qualification. External recommendations do not adopt libraries,
centralize domain policy, freeze proposed limits or qualify runtime behavior.
No upstream implementation or example code was copied into the reference.

| Primary evidence | Finding used |
| --- | --- |
| [Rust `Error`](https://doc.rust-lang.org/std/error/trait.Error.html) and [API Guidelines C-GOOD-ERR](https://rust-lang.github.io/api-guidelines/interoperability.html#c-good-err) | Specific errors retain actionable meaning; `source()` preserves a cause across abstractions, not permission to emit it; avoid duplicating the cause in `Display` and the chain |
| [thiserror documentation](https://docs.rs/thiserror/latest/thiserror/), [pinned expansion](https://github.com/dtolnay/thiserror/blob/3b29f0107e7900860c7a061c069958569eb458e7/impl/src/expand.rs#L36-L69) and [transparent-error tests](https://github.com/dtolnay/thiserror/blob/3b29f0107e7900860c7a061c069958569eb458e7/tests/test_transparent.rs#L33-L48) | Ordinary typed error implementations; `from` implies source; transparent wrappers forward behavior and are not privacy barriers. Context7 documentation checked against project source |
| [anyhow formatting](https://github.com/dtolnay/anyhow/blob/bb1b0f9af0f645d5fdf1ac63a1fcbf3975ee452c/src/fmt.rs#L7-L65) and [formatting contract](https://github.com/dtolnay/anyhow/blob/bb1b0f9af0f645d5fdf1ac63a1fcbf3975ee452c/src/lib.rs#L526-L540) | Alternate Display and Debug can expose causes/backtraces/inner Debug; context can preserve downcasting but does not establish safe output |
| [Microsoft GetLastError](https://learn.microsoft.com/en-us/windows/win32/api/errhandlingapi/nf-errhandlingapi-getlasterror), [windows-rs 0.62.2 error implementation](https://github.com/microsoft/windows-rs/blob/32c3144490c016fe496a0aed769bce60987a2e9d/crates/libs/result/src/error.rs#L117-L138), [HRESULT](https://github.com/microsoft/windows-rs/blob/32c3144490c016fe496a0aed769bce60987a2e9d/crates/libs/result/src/hresult.rs#L10-L39) and [BOOL](https://github.com/microsoft/windows-rs/blob/32c3144490c016fe496a0aed769bce60987a2e9d/crates/libs/result/src/bool.rs#L16-L23) | Capture only the documented error convention immediately; preserve meaningful success statuses. Native message formatting may use attached error information; generic wrappers cannot decide each API's semantics |
| [OWASP Logging Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Logging_Cheat_Sheet.html#event-collection) | Shared event handling, sensitive-field exclusion, injection-resistant encoding and logging-failure checks; NevroTick's producer-side typed policy remains authoritative |
| [AWS timeouts, retries and backoff](https://aws.amazon.com/builders-library/timeouts-retries-and-backoff-with-jitter/) and [Tokio spawn_blocking](https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html) | Retry at a bounded owner with effect/idempotency knowledge; timeout does not prove absence of effects, and started blocking work cannot be aborted by its handle |
| [Rust catch_unwind](https://doc.rust-lang.org/std/panic/fn.catch_unwind.html) and [File](https://doc.rust-lang.org/std/fs/struct.File.html) | Catching handles only unwinding after the panic hook; File Drop ignores close errors. Required error observation needs explicit fallible completion; neither mechanism proves recovery or crash durability |

Pinned revisions identify inspected upstream evidence, not a selected NevroTick
Cargo graph. Rolling documentation must be reconciled with the actual locked
versions/features before implementation. The adapted leaves retain their notices.

## Prior Rust port supplied by the user

Pinned overlay: [DavidOsipov/pdfcraft c9a02b378271f92a73cb44be9fd9e4acc6921516](https://github.com/DavidOsipov/pdfcraft/tree/c9a02b378271f92a73cb44be9fd9e4acc6921516),
submitted as [storytold/pdfcraft PR 2912](https://github.com/storytold/pdfcraft/pull/2912).
Supplied reference material proves no upstream adoption.

Pinned files read through GitHub connector:

- [Rust skill](https://github.com/DavidOsipov/pdfcraft/blob/c9a02b378271f92a73cb44be9fd9e4acc6921516/.agents/skills/QA/pdfcraft-rust-implementation/SKILL.md),
  [Rust workflow](https://github.com/DavidOsipov/pdfcraft/blob/c9a02b378271f92a73cb44be9fd9e4acc6921516/.agents/skills/QA/pdfcraft-rust-implementation/references/rust-workflow.md):
  retain test identity, compiler/runner/feature distinctions, explicit MSRV
  inheritance, tool setup limits and host/runtime evidence boundaries.
- [Bounded parser design](https://github.com/DavidOsipov/pdfcraft/blob/c9a02b378271f92a73cb44be9fd9e4acc6921516/.agents/skills/QA/pdfcraft-bounded-parser-design/SKILL.md):
  separate checked arithmetic from allocation/work limits and charge aggregate work
  at an owner; cancellation request differs from completed/released native work.
- [Safe filesystem persistence](https://github.com/DavidOsipov/pdfcraft/blob/c9a02b378271f92a73cb44be9fd9e4acc6921516/.agents/skills/QA/pdfcraft-safe-filesystem-persistence/SKILL.md)
  and [transactional editing](https://github.com/DavidOsipov/pdfcraft/blob/c9a02b378271f92a73cb44be9fd9e4acc6921516/.agents/skills/QA/pdfcraft-transactional-editing/SKILL.md):
  distinguish preparation, publication, acknowledgement and recovery; atomic
  replacement differs from crash durability. NevroTick owners replace all PDF APIs.
- [Vendored maintenance](https://github.com/DavidOsipov/pdfcraft/blob/c9a02b378271f92a73cb44be9fd9e4acc6921516/.agents/skills/QA/pdfcraft-vendored-dependency-maintenance/SKILL.md):
  the [environment adaptation](../environment/SOURCES.md) retains local patch
  provenance/regression accounting when actual forks exist.
- [Original notice](https://github.com/DavidOsipov/pdfcraft/blob/c9a02b378271f92a73cb44be9fd9e4acc6921516/.agents/skills/QA/pdfcraft-rust-implementation/THIRD_PARTY_NOTICES.md):
  David-owned original material is MIT; the full notice is retained in each adapted
  portable leaf. No target application license is changed.
