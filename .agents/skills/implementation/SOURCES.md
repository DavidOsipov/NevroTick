# Implementation adaptation sources

Reviewed 28 October 2026. Maintenance/provenance record; load task-relevant linked
conditional reference, not whole file. Repository architecture outranks external
examples. Documentation adaptation performed no Cargo graph, Rust build, parser
experiment or Windows UI execution.

## Modern Rust release review — 9 November 2026

The [modern stable Rust reference](implement-rust/references/modern-rust.md)
supports `implement-rust` with version-specific choices and caveats. It was
checked against NevroTick `61fd244a6bbd12cee887e6974da1b730a5aa1561`, whose
toolchain/MSRV already selected 1.99.1, edition 2024 and resolver 3. This update
changes instructions, not the toolchain, product source, dependency graph,
runtime topology, CPU floor or qualification status. Review/use evidence belongs
in the [existing skill-quality record](../../plans/2026-10-30-skill-quality.md#modern-rust-reference-review--9-november-2026).

The requested interval is **2024-11-09 through 2026-11-09**, inclusive. Checked
the [official release index](https://blog.rust-lang.org/releases/), each announcement
below and the corresponding language, compiler/platform, library/const API,
Cargo, rustdoc and compatibility sections present in the
[complete release notes](https://doc.rust-lang.org/releases.html), also available
in the [1.99.1 source snapshot](https://github.com/rust-lang/rust/blob/1.99.1/RELEASES.md).
The index ends at **1.99.1 (2026-10-15)**: **17 feature releases and 9 patches**
fall in the interval. Rust 1.82 (2024-10-17) is outside it; previously stable
prerequisites are not counted as new features. Do not infer future releases from
a schedule or treat beta/nightly material as a stable contract.

The table records the disposition of every release, with representative changes
that explain that disposition. The linked release/API inventories retain the full
upstream lists; this is not a copied changelog. Unrelated target support,
diagnostic/rustdoc presentation and minor convenience APIs do not each require a
normative instruction. **Prefer** means use for a matching touched contract;
**conditional** needs a concrete path and existing approval/evidence; neither
orders a mechanical code rewrite.

| Release / official source | Date | Guidance disposition |
| --- | --- | --- |
| [1.83.0](https://blog.rust-lang.org/2024/11/28/Rust-1.83.0/) | 2024-11-28 | Conditional const construction, `BufRead`/collection/`ControlFlow` conveniences; preserve explicit OS-error classification and artifact-path inspection. |
| [1.84.0](https://blog.rust-lang.org/2025/01/09/Rust-1.84.0/) | 2025-01-09 | Prefer strict-provenance tools inside necessary adapters and exact integer square root where applicable. Cargo MSRV-aware resolution reinforces existing resolver policy; IP address classification is not reachability. |
| [1.84.1](https://blog.rust-lang.org/2025/01/30/Rust-1.84.1/) | 2025-01-30 | Patch review: compiler/incremental/diagnostic/build corrections; no distinct coding pattern needed. |
| [1.85.0](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/) | 2025-02-20 | Document current 2024 safety/drop/capture semantics and async closures. Midpoint is conditional on rounding; `Waker::noop` cannot establish executor progress. |
| [1.85.1](https://blog.rust-lang.org/2025/03/18/Rust-1.85.1/) | 2025-03-18 | Patch review includes merged doctests and Windows rename regressions; already subsumed by the selected pin. |
| [1.86.0](https://blog.rust-lang.org/2025/04/03/Rust-1.86.0/) | 2025-04-03 | Prefer safe disjoint mutable access. Upcasting, adjacent floats, target-feature functions and `OnceLock::wait` are conditional on existing ownership/numerical/CPU/blocking contracts. |
| [1.87.0](https://blog.rust-lang.org/2025/05/15/Rust-1.87.0/) | 2025-05-15 | Document precise trait captures, signed midpoint and bit reinterpretation limits; conditional cursor/extraction helpers and pipes. Preserve explicit native links and CPU-feature eligibility for safer intrinsics. |
| [1.88.0](https://blog.rust-lang.org/2025/06/26/Rust-1.88.0/) | 2025-06-26 | Prefer checked array chunking and readable 2024 let chains. Conditional cfg/cache conveniences; `select_unpredictable` is no constant-time guarantee. No ordinary need for naked assembly. |
| [1.89.0](https://blog.rust-lang.org/2025/08/07/Rust-1.89.0/) | 2025-08-07 | Conditional const inference, lifetime syntax, `Result::flatten` and file locks; heed raw-pointer diagnostics and cross-target doctest execution. AVX-512 support changes no v3 minimum. |
| [1.90.0](https://blog.rust-lang.org/2025/09/18/Rust-1.90.0/) | 2025-09-18 | Prefer checked signed subtraction when its range matches. Linux linker defaults and multi-package publishing do not change Windows linking or publication authority. |
| [1.91.0](https://blog.rust-lang.org/2025/10/30/Rust-1.91.0/) | 2025-10-30 | Prefer checked signed differences; distinguish panic-on-overflow `strict_*`. Conditional duration/collection helpers and intermediate build directories. ARM64 tier changes do not expand the supported target. |
| [1.91.1](https://blog.rust-lang.org/2025/11/10/Rust-1.91.1/) | 2025-11-10 | Patch review: illumos locking and WASM linking; no Windows coding rule to add. |
| [1.92.0](https://blog.rust-lang.org/2025/12/11/Rust-1.92.0/) | 2025-12-11 | Preserve stronger never-type diagnostics. Conditional lock downgrade; zeroed allocation and raw union borrows retain initialization/aliasing obligations. |
| [1.93.0](https://blog.rust-lang.org/2026/01/22/Rust-1.93.0/) | 2026-01-22 | Prefer safe exact-size array views; bound `Duration::from_nanos_u128`. Raw-parts/`MaybeUninit` facilities grant no unsafe permission. Stable jump-table control is already configured. |
| [1.93.1](https://blog.rust-lang.org/2026/02/12/Rust-1.93.1/) | 2026-02-12 | Patch review: compiler/rustfmt crashes, Clippy false positives and WASI descriptors; no new product requirement. |
| [1.94.0](https://blog.rust-lang.org/2026/03/05/Rust-1.94.0/) | 2026-03-05 | Prefer fixed-size windows where useful; conditional lazy/iterator helpers. Review Cargo includes/TOML parser compatibility, closure captures and Windows time bounds when affected. |
| [1.94.1](https://blog.rust-lang.org/2026/03/26/Rust-1.94.1/) | 2026-03-26 | Patch review includes removal of incompatible Windows `OpenOptionsExt` additions and Cargo archive-security fixes; check exact patch versions. |
| [1.95.0](https://blog.rust-lang.org/2026/04/16/Rust-1.95.0/) | 2026-04-16 | Prefer useful `cfg_select!`, match guards and strict 0/1 conversion. Atomic updates need repeatable closures. Conditional insertion/range/layout/hint facilities; no custom target replacement. |
| [1.96.0](https://blog.rust-lang.org/2026/05/28/Rust-1.96.0/) | 2026-05-28 | Conditional diagnostic assertions and explicit new range types. Preserve pointer validity/trait contracts; Cargo security fixes and platform changes add no execution permission. |
| [1.96.1](https://blog.rust-lang.org/2026/06/30/Rust-1.96.1/) | 2026-06-30 | Patch review: MIR miscompilation, Cargo retries/timeouts and bundled libssh2 security fixes. |
| [1.97.0](https://blog.rust-lang.org/2026/07/09/Rust-1.97.0/) | 2026-07-09 | Conditional bit helpers, Cargo warning/lockfile-path controls; review v0 demangling, pin coercion, enum-layout assumptions and Windows socket errors when affected. |
| [1.97.1](https://blog.rust-lang.org/2026/07/16/Rust-1.97.1/) | 2026-07-16 | Patch review: LLVM miscompilation fix; compiler version is part of evidence identity. |
| [1.98.0](https://blog.rust-lang.org/2026/08/20/Rust-1.98.0/) | 2026-08-20 | Conditional integer buffers, UTF-16 decoders, bool-to-Result and subrange helpers. Reject relaxed algebraic floats for current timing/security contracts; review Windows TLS destruction and ownership compatibility. |
| [1.98.1](https://blog.rust-lang.org/2026/09/03/Rust-1.98.1/) | 2026-09-03 | Patch review: trait-object vtable miscompilation correction; no evidence NevroTick exercised the defect. |
| [1.99.0](https://blog.rust-lang.org/2026/10/01/Rust-1.99.0/) | 2026-10-01 | Document inherited default-feature override, raw-borrow diagnostics, tagged-union evaluation and ownership-transfer cautions. Defer unneeded variadics/layout machinery; preserve strict decoding and CI/artifact identity. |
| [1.99.1](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/) | 2026-10-15 | Current pin already includes x86-64 FFI `bool` codegen correction, `wcslen` fallback and macOS linker repair. No upgrade or affected-product claim. |

Exact method contracts are linked beside the reusable guidance. Three source
qualifications matter: the detailed 1.98 notes/rustdoc include `bool::ok_or*`
although the announcement's API list omits them; Cargo's 1.99 notes and current
dependency reference supersede the older Edition Guide's inherited-defaults
rejection; `set_var`'s official safety contract explicitly permits Windows
multithreaded calls despite its edition-2024 unsafe signature. Research is original
instructional synthesis; no upstream code, skill, dependency or licence grant was
imported. Existing adapted-skill notices and upstream inventories remain intact.

## Windows service workflow — 7 November 2026

The same-day metadata handoff extends [maintain-windows-metadata](maintain-windows-metadata/SKILL.md)
and service/package-QA routing. The [reviewed bounded catalogue](../../../docs/supporting-reviews/windows-service-metadata-catalogue.md)
retains Microsoft field/schema sources once, with the canonical metadata plan
owning applicability. Separate reviewers checked SCM, artifact/distribution and
diagnostic scopes; ambiguous required-privilege omission wording and a package
ledger's declaration/reference wording were corrected. Source/schema evidence
does not qualify generated, signed, installed or Store-delivered resources.
No donor code, dependency, identity or new metadata service was adopted.


The original local [implement-windows-service](implement-windows-service/SKILL.md)
routes the existing [SCM/native procedure](implement-rust/references/windows-service-adapters.md),
canonical service/error/resource contracts and current W11 owners. Reviewed the
owner-supplied handoff against `df6e28a4641f2f046373a9c5640aa60736716a22` and
verified consequential Microsoft and pinned upstream claims; the maintained
[research refinement](../../../docs/plans/windows11-integration-follow-up.md#additional-service-practices--7-november-2026)
retains direct sources, API discrepancies, inference and conditional applicability.
No upstream implementation was copied, dependency selected or native execution
performed. Register this leaf in `LOCAL.json`; it has no invented donor provenance.

## Defensive Rust patterns — 6 November 2026

Reviewed [Corrode's defensive-programming article](https://corrode.dev/blog/defensive-programming/)
as supporting material for the existing Rust/contract skills. The locally authored
[structural-change guidance](implement-rust-contracts/references/contract-enforcement.md#make-structural-changes-require-review)
adopts selective field/variant review and coupled collection access. Rust's
[privacy rules](https://doc.rust-lang.org/reference/visibility-and-privacy.html),
[`non_exhaustive` semantics](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute)
and [Clippy's actual field-pattern recommendation](https://clippy.rust-lang.org/master/index.html#unneeded_field_pattern)
qualify the article's constructor and lint claims. No article code was copied.
This guidance selects no new lint configuration, dependency or product proof;
existing admission and contract QA keep their owners. OD-10 formal obligations
remain post-V1 under its first-release deferral.

## Production hardening and illegal states follow-up — 6 November 2026

Reviewed [Corrode's production-hardening article](https://corrode.dev/blog/hardening-rust/)
against [Rust panic-hook semantics](https://doc.rust-lang.org/std/panic/fn.set_hook.html)
and [Cargo profile settings](https://doc.rust-lang.org/cargo/reference/profiles.html).
The existing error-handling and execution references now distinguish ordinary
panic hooks under abort/unwind, termination paths, and effective shipping settings
from the assertion-enabled `verify-release` profile. Hook cleanup and generic
debug/release claims supply no recovery or release qualification.
[Illegal-state guidance](https://corrode.dev/blog/illegal-state/) was also reviewed:
the existing checked representations, privacy-scope audit, contextual rechecks
and domain-equivalence rules already cover its useful guarantees. Its illustrative
date arithmetic and ambient clock reads were not adopted. No example code,
allocator, deployment framework, dependency or new execution lane was introduced.

## Windows metadata workflow — 5 November 2026

[maintain-windows-metadata](maintain-windows-metadata/SKILL.md) is an original
local workflow for the [existing field contract](../../../docs/plans/windows-package-metadata.md).
Its four conditional references cover harvest/provenance, authoritative records
and storage, generation/readback, and [primary sources](maintain-windows-metadata/references/sources.md).
Two explicitly incomplete TOML/JSON examples illustrate field dispositions and
observations without inventing a production identity, schema implementation or
collector. W11-W10/P6 and existing package QA retain acceptance ownership.

Microsoft Learn, GitHub, Context7, Firecrawl and web retrieval supplied research;
official contracts and pinned Cargo/resource-helper source govern factual claims.
The new evidence distinguishes Cargo resolution from build units/payload,
Windows MUI resolution from raw PE content, MSIX parsing from signature trust,
read-only MSI inspection from installed-state queries, and Win32_Product's
repair side effects. Reviewed TOML plus separate JSON observations and the
existing CycloneDX inventory is a project-fit recommendation. No crate/tool,
native experiment, runtime metadata service or Store publication was adopted.

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
[existing UI task](../../../tasks/ui-m0.md) retains its baseline gates;
[S0.5](../../../tasks/s0.5-ui-comparison.md) and the
[GUI handoff](../../../tasks/slint-gui-handoff.md) own current migration scope,
recorded evidence and remaining native/text-approval gaps.

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

## Additional local Slint source register — 5 November 2026

The canonical [implement-slint](implement-slint/SKILL.md) workflow retains a
companion source register for OD-13's selected replacement and S0.5 migration. Its
[source register](implement-slint/SOURCES.md) binds Slint 1.18.1 to immutable
GitHub source, distinguishes runtime/build features and licensing, and retains
the native AVIF decoder discrepancy. Rust callbacks, models and lifecycle
references are original instructional prose, not a copied upstream skill or
application implementation. Context7/GitHub MCP tools were unavailable in the
authoring session, including after explicit tags; the actual evidence came
from official HTTPS documentation and pinned GitHub source. No MCP use,
dependency adoption or Slint/Windows runtime qualification is claimed.
