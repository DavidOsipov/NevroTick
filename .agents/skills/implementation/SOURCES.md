# Implementation adaptation sources

Reviewed 28 October 2026. Maintenance/provenance record; load task-relevant linked
conditional reference, not whole file. Repository architecture outranks external
examples. Documentation adaptation performed no Cargo graph, Rust build, parser
experiment or Windows UI execution.

## Slint implementation guidance — 5 November 2026

[implement-slint](implement-slint/SKILL.md) is an original local workflow alongside
the native-UI adaptation. Its six conditional references contain the reusable
API, source, Windows, testing, asset and example evidence; start at
[examples and source research](implement-slint/references/examples-and-sources.md).
The inspected Slint release is 1.18.1 at
[`372cf0ee5577c3dfec309a45e7b778ba4e81b734`](https://github.com/slint-ui/slint/tree/372cf0ee5577c3dfec309a45e7b778ba4e81b734).
Runtime/build manifests and immutable implementation qualify rolling-doc examples.
Microsoft Learn governs the referenced Windows contracts. Public application
repositories provide use cases, not inherited performance or security results.

The references distinguish property-binding replacement, checked model mutation,
UI-thread/lifetime ownership, precise time projection, feature/licence admission,
AVIF encoder-versus-decoder support, developer control-server features and
no-pixel versus rendered/native evidence. The Windows tray broadcast discrepancy
is a source-level recovery concern for later native qualification, not a
reproduced defect or a new UI-M0 blocker. No upstream implementation, dependency,
font or asset is vendored; source notices remain with their owners. The
[existing UI task](../../../tasks/ui-m0.md) retains admission and execution gates.

## Preserving established invariants — 4 November 2026

The [Parse, don't validate explanation](implement-rust-contracts/references/contract-enforcement.md#parse-dont-validate-preserve-established-invariants)
clarifies the existing private-construction and checked-consumer contract. It adds
a method for choosing the representation from the consumer's required facts and
uses existing `EraWindow`, `RawDatagram` and `RequestTiming` APIs as local examples.
It adds no skill, dependency, product behavior or runtime qualification claim.

| Source inspected | Finding used and limit |
| --- | --- |
| Alexis King, [Parse, don't validate](https://lexi-lambda.github.io/blog/2019/11/05/parse-don-t-validate/) (5 November 2019) | Preserve established information in a more precise value; refine the relevant branch and choose useful representations. The principle is not a ban on internal predicates, multiple parsing passes or necessary runtime checks. |
| [Rust API Guidelines: C-VALIDATE](https://rust-lang.github.io/api-guidelines/dependability.html#c-validate) | Prefer argument types that exclude invalid inputs; retain dynamic enforcement for properties that types cannot practically express. The page's optional debug-only/unchecked alternatives do not relax NevroTick's production rejection requirements. |
| [Rust HashMap: FromIterator](https://doc.rust-lang.org/std/collections/struct.HashMap.html#impl-FromIterator%3C(K,+V)%3E-for-HashMap%3CK,+V,+S%3E) | Equal keys cause values to be dropped during collection; map construction alone cannot prove duplicate-free input. Preserve the existing raw-input rejection contract. |

The explanation and project examples are original local writing; no upstream
implementation or article text is imported. Existing Serde and type-safety
sources below remain applicable. Local source inspection illustrates scoped
guarantees, not passing tests or a measured resource improvement.

## Domain equivalence and readability — 4 November 2026

The user supplied a Markdown export of the
[Rust codebase discussion](https://www.reddit.com/r/rust/comments/1xlwg4h/cleanest_best_written_rust_codebases/).
This records the supplied excerpt, not a live retrieval or all collapsed replies.
Its domain-fit, tightness and readability comments motivated the existing
[contract reference](implement-rust-contracts/references/contract-enforcement.md#domain-equivalence-and-trait-contracts),
Rust implementation guidance and targeted contract-test cases. The original
Parse explanation and pure-core/effect ownership already cover the other relevant
mechanisms; project-name recommendations alone establish no reusable contract.

| Source inspected | Finding used and limit |
| --- | --- |
| Supplied Reddit comments on [domain fit](https://www.reddit.com/r/rust/comments/1xlwg4h/comment/19s1bb406orpf/), [tightness](https://www.reddit.com/r/rust/comments/1xlwg4h/comment/19s1f0hfbw1tt/) and [readability](https://www.reddit.com/r/rust/comments/1xlwg4h/comment/19s19mi8pxmbm/) | Separate invalid states from equivalent representations; name coherent domain operations. These are review prompts, not a 100% tightness target, fixed nesting quota or independently verified codebase ranking. |
| Rust [`Hash` and `Eq`](https://doc.rust-lang.org/std/hash/trait.Hash.html#hash-and-eq), [`Ord`](https://doc.rust-lang.org/std/cmp/trait.Ord.html), [`HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html) and [`BTreeMap`](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) | Equal values require equal hashes, ordering traits must agree, and key behavior must remain stable while stored. Hash bytes are not generally portable/stable; lawful derives still need the correct domain meaning. |
| Microsoft [Pragmatic Rust Guidelines: applicability](https://microsoft.github.io/rust-guidelines/guidelines/index.html#applicability) and [universal guidance](https://microsoft.github.io/rust-guidelines/guidelines/universal/index.html) | Motivation and project context govern adoption. No blanket crate splitting, lint/tool adoption, naming quota or Drop-only native-completion policy follows from this review. Existing NevroTick ownership and execution contracts control. |

The additions are original local instructions, with no copied upstream code or
new skill/dependency. The discussion's Unicode example is not adopted: Rust
[`char`](https://doc.rust-lang.org/std/primitive.char.html#validity-and-layout)
models scalar values up to `0x10FFFF` excluding surrogate code points; unassigned
values are not generally invalid. Domain-specific text rules require their own
contract. Source review establishes no product implementation or measured gain.

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
| implement-semantic-html | implement-native-ui | Preserve semantic controls, relationships, keyboard/focus and real-consumer evidence; use Slint/Windows UIA/Narrator/DPI and English-first catalog under current OD-13 instead of HTML/ARIA/browser assertions or mandatory website locales |

Copied no browser/ECMAScript corpus, website examples, npm scripts or website-only
enforcement helper into Rust project.

## Project authority

- [Documentation guide](../../../docs/README.md): architecture controls; retained
  reviews and update research do not establish implementation.
- [Architecture](../../../docs/windows-time-architecture.md), sections 5–6:
  bounded exact representation and separate service authorization; section 16:
  Slint default under [OD-13](../../../docs/nevrotick-owner-decisions.md#17-od-13--slint-as-the-default-ui-toolkit), accessible behavior and English-first locale; sections
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

## CPU baseline and runtime dispatch — 30 October 2026

The original local [CPU dispatch skill](implement-cpu-dispatch/SKILL.md) implements
[OD-07](../../../docs/nevrotick-owner-decisions.md#11-od-07--x64-baseline-and-runtime-optimization)
and [hardening §6.7](../../../docs/windows-time-development-hardening.md#67-v3-baseline-and-runtime-cpu-dispatch).
The [maintained research](../../../docs/supporting-reviews/x86-runtime-optimization.md)
records primary Rust Reference, std detection implementation and psABI feature
sources. Function-level features require exact CPU/OS gates; v3 includes AVX2,
and AES is an independent provider capability. Feature support is no benchmark.
The skill is locally authored; no third-party skill implementation or example
code was copied. It admits no product dependency, unsafe exception or native pass.

The follow-up review checked Rust's [pinned atomic feature cache](https://github.com/rust-lang/rust/blob/58cf8f9c35b355199727627d07ae34a4e2846d67/library/std_detect/src/detect/cache.rs#L55-L90),
[x86 cold detection](https://doc.rust-lang.org/src/std_detect/detect/os/x86.rs.html)
and [`OnceLock`](https://doc.rust-lang.org/std/sync/struct.OnceLock.html).
Warm feature lookup is already cached; caching the selected typed kernel is a
separate measured design choice. Full-v4 membership does not require every
narrower AVX-512 function to enable unused extensions. These references correct
overbroad external advice without introducing a custom atomic dispatcher.

Three separate read-only research assignments then examined CPU/OS/VM admission,
compiler/ABI boundaries and cache/performance lifecycle. The coordinator also
checked numerical/memory semantics using Context7 against primary Rust docs and
the installed intrinsic source. Their reusable findings live in
[x86 caveats](implement-cpu-dispatch/references/x86-caveats.md), with source-specific
links to the pinned compiler, Microsoft XSTATE/DLL/calling-convention/Hyper-V
documentation, and Intel's generation-specific SIMD tuning and packet-processing
guides. Rust 1.99.0 Context7 examples were reconciled with the 1.99.1 source/docs;
portable SIMD examples grant no nightly/dependency admission. No third-party
skill code or application implementation was imported. Research and model checks
do not establish native compatibility, tail-load safety or a useful speedup.
