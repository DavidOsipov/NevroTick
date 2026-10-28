# Implementation adaptation sources

Reviewed 28 October 2026. This is a maintenance/provenance record; read the linked
conditional reference during the actual task rather than loading this whole file.
Repository architecture takes precedence over external examples. No Cargo graph,
Rust build, parser experiment or Windows UI execution was performed for this
documentation adaptation.

## Website source mapping

Source root: `/home/david_osipov/personal-website-templated-1/.agents/implementation/`.
These source entrypoints, group router and shared contract were inspected; the
website files were not modified.

| Website skill | NevroTick skill | Preserved and changed decisions |
| --- | --- | --- |
| implement-typescript | implement-rust | Preserve owner/caller/effect tracing and proportional verification; use Rust ownership, Result, exact newtypes, FFI/async obligations and actual target admission |
| implement-zod-contracts | implement-rust-contracts | Preserve checked-output consumption, invariant ownership, failure atomicity and valid/invalid controls; replace Zod with the architecture's bounded Serde DTO → fallible private domain construction → current effect guard |
| evolve-data-contracts | evolve-data-contracts | Preserve semantic compatibility matrices and separate rollout/retirement; cover versioned native IPC, protected policy/journals, mixed generations and MSI rollback |
| implement-semantic-html | implement-native-ui | Preserve semantic controls, relationships, keyboard/focus and real-consumer evidence; use native Win32/UIA/Narrator/DPI and English-first catalog instead of HTML/ARIA/browser assertions or mandatory website locales |

No browser/ECMAScript reference corpus, website code examples, npm scripts or
website-only enforcement helper was copied into the Rust project.

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

These observations were checked against official documentation or project-owned
source. Rolling documentation/branch links are a dated observation, not a pinned
dependency artifact or a claim that NevroTick uses their latest release.

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

The connected GitHub fetch retrieved serde-rs/json source. Attempts to fetch the
windows-rs README through the file endpoint returned 404; no conclusion relies on
those failed reads. Microsoft documentation and the local architectural record
support the Win32/UIA guidance. No additional fallback package or tool was installed.

## Prior Rust port supplied by the user

Pinned overlay: [DavidOsipov/pdfcraft c9a02b378271f92a73cb44be9fd9e4acc6921516](https://github.com/DavidOsipov/pdfcraft/tree/c9a02b378271f92a73cb44be9fd9e4acc6921516),
submitted as [storytold/pdfcraft PR 2912](https://github.com/storytold/pdfcraft/pull/2912).
Treat it as supplied reference material, not proof the upstream adopted it.

The following pinned files were read via the GitHub connector:

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
