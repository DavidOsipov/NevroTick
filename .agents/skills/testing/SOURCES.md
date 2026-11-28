# Testing adaptation and source evidence

Research checked **28 October 2026**. Provenance/selection reasoning; no executed product verification. Canonical [architecture](../../../docs/windows-time-architecture.md), especially sections 6, 16, 18 and 20, controls requirements. CHECK-1–27 catalog planned; no retained passing run. Tools remain candidates unless adopted.

## Chrony testing-tool references — 30 October 2026

The [upstream tools reference](references/chrony-upstream-tools.md) records inspected
clknetsim `e9f11e60c72f2c0674a072c4ddcfe343f8634e58` and chrony-fuzz
`904a8789164ac294ab7e85523792ed906a7b5a57`, their primary sources, file-specific
rights and harness limits. Existing port/fuzz skills route scenario and seed
reuse without importing code/corpus, switching engines or admitting the C/build
graphs. Invasive protocol/crypto bypasses preclude treating patched outputs as
independent security truth. Source inspection runs no upstream campaign.

## Chrony and NTPsec mechanisms — 30 October 2026

The [source-task assessment](../../../tasks/s0.1-protocol-comparison.md#chrony-and-ntpsec-mechanism-assessment)
records Chrony 4.9 `8df4f1263e206585e6c4e61352d042c2dd12916a` and NTPsec
`097ea276d9e1b860aac548830a8bc03f1ceb6889`, exact functions and file-specific rights.
The port recipe, numerical/protocol truth and security leaves add consumer-level
aging/correction, scheduling, atomic KE publication, send-guard and restoration
checks. Shared lineage and implementation-derived tests are not independent truth.
This original local guidance imports no source/test bodies, tuning constants,
second controller or dependency graph; no upstream/native execution is claimed.

The repeated-proposal review also checks [RFC 9109 §§3.2–4](https://www.rfc-editor.org/rfc/rfc9109.html#section-4)
and [RFC 8915 §5.7](https://www.rfc-editor.org/rfc/rfc8915.html#section-5.7)
for port-lifecycle qualification and the control-only KoD/NAK test distinction.

## ntp_usg comparison refinements — 11 November 2026

The [source assessment](../../../docs/supporting-reviews/ntp-usg-reuse-assessment.md)
pins ntp_usg `bfab924b9a7f550d46eae53c53d9b77fb046e815`, actual continuous and
one-shot callers, protocol tests and benchmark/CI evidence. Existing
[numerical/protocol QA](references/numerical-protocol.md) gains independent
feedback-convergence, nonzero-uncertainty, repeated-cookie-inventory and actual
RATE-send cases. RFC 8915's correlated-control exception and pending S2.2
zero-replacement/reuse decisions remain intact. These original local procedures
import no source/test bodies, numerical constants, dependency graph or runtime
qualification; source reading is not a donor/native test run.

## Sanitizer selection and controls — 31 October 2026

The original local [sanitizer reference](references/sanitizers.md) extends the
existing Miri/fuzz/native workflows. It imports no tool, code sample or skill
implementation and changes no runtime pin or admission inventory. The selection
rules and mechanism-specific controls use these primary sources:

| Source | Consequence and limit |
| --- | --- |
| [Rust sanitizer guidance](https://doc.rust-lang.org/unstable-book/compiler-flags/sanitizer.html) and [Miri](https://github.com/rust-lang/miri#readme) | Select by defect, target and instrumentation; preserve Miri's unsupported FFI/execution limits. Rolling docs do not qualify the installed compiler or product harness. |
| [Clang sanitizer options](https://clang.llvm.org/docs/UsersManual.html#controlling-code-generation) and [LSan](https://clang.llvm.org/docs/LeakSanitizer.html) | Separate ASan/TSan/MSan configurations; supported ASan/LSan integration is a valid combination. Leak detection does not measure reachable growth or nonmemory resources. |
| [MSan external-code handling](https://clang.llvm.org/docs/MemorySanitizer.html#handling-external-code) and [TSan limitations](https://clang.llvm.org/docs/ThreadSanitizer.html#limitations) | Native libraries, standard-library instrumentation and synchronization visibility constrain results; missing coverage cannot become a clean-process claim. |
| [UBSan usage](https://clang.llvm.org/docs/UndefinedBehaviorSanitizer.html#usage) | Compile the actual native C/C++ source with selected checks; merely linking a runtime into Rust emits none of those missing checks. |
| [Rust Windows target snapshot](https://github.com/rust-lang/rust/blob/76456a688bc0ed897fa23a677d4ddb5b3fdb575a/compiler/rustc_target/src/spec/targets/x86_64_pc_windows_msvc.rs) and [Rust Fuzz Book Windows setup](https://rust-fuzz.github.io/book/cargo-fuzz/windows/setup.html) | That compiler snapshot admits ADDRESS although the Rust Unstable Book's ASan list omits Windows. Require exact pinned native qualification; do not infer support or impossibility from one document. |
| [Android HWASan](https://developer.android.com/ndk/guides/hwasan) and [Rust HWASan/memtag guidance](https://doc.rust-lang.org/unstable-book/compiler-flags/sanitizer.html#hwaddresssanitizer) | Future Android ARM64 work needs its own toolchain/runtime/device evidence. HWASan and hardware MTE are distinct; no Android lane is admitted here. |
| [Microsoft ASan limitations](https://learn.microsoft.com/en-us/cpp/sanitizers/asan-known-issues?view=msvc-170), [AppVerifier](https://learn.microsoft.com/en-us/windows-hardware/drivers/devtest/application-verifier) and [PageHeap](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/gflags-and-pageheap) | Track instrumented native modules, runtime/DLL lifetime and observed allocator/API paths; keep resource retention, ordinary-user execution and instrumentation restoration separate. |

Semantic assertion controls establish oracle sensitivity/failure propagation;
representative memory faults with the detector's own diagnostic establish a
different mechanism claim. This instruction amendment executes neither control
and retroactively upgrades no existing fixture or product evidence.

## Skill-quality procedures — 30 October 2026

The [workflow-observation reference](references/workflow-observation.md), added
2 November 2026, critically assesses pinned Microsoft TokenEconomizer source.
It borrows accounting and shadow-advisory ideas into original local procedures
and a standard-library offline helper. No upstream code, plugin, model service,
automatic memory or tool-pruning configuration is adopted. Its source links,
verified limitations and execution boundaries are retained in that reference;
the existing skill-quality task owns actual controls and remaining evaluation.

Original local procedures extend existing numerical, harness, mutation and
campaign skills; `qa-skill-evaluations` is registered separately in `LOCAL.json`.
No donor mapping or imported-file hash is changed. Corrected local findings in
[S1.1e](../../../tasks/s1.1-deterministic-harness.md),
[S1.1c mutation review](../../../tasks/s1.1-time-domain-equivalences.md) and
[S1.2 preparation](../../../tasks/s1.2-regression-readiness.md) motivated reusable
methods; their numeric limits and candidate results remain task-specific.

[Anthropic's agent-evaluation research](https://www.anthropic.com/research/demystifying-evals-for-ai-agents),
published 9 January 2026 and checked 30 October 2026, supports isolated tasks,
outcome-based grading and separately identified trials. The local exercise,
Rust fixture and standard-library grader are original examples, with no imported
implementation or framework dependency. Execution receipts and limits belong to
[the skill-quality task](../../plans/2026-10-30-skill-quality.md), not this source
registry. Neither packaging validation nor a single exercise proves general agent
reliability, application behavior or an improvement over untested old instructions.

## Donor skills and adaptations

Original user-owned source: `/home/david_osipov/personal-website-templated-1/.agents/skills/testing/`. All 36 entrypoints accounted for below. Retained central testing contract, independent expectations/controls, causal repair, evidence custody, finite stabilization, proportional qualification. Rust/native Windows evidence replaces website browser/DOM/Vitest/Cloudflare details. No imported per-leaf duplicate shared trees/website command wrappers/website fixture data. Shared guidance has one canonical owner under `references/`.

| Website skill | NevroTick skill |
| --- | --- |
| `qa-accessibility` | [qa-accessibility](qa-accessibility/SKILL.md) |
| `qa-adversarial-inputs` | [qa-adversarial-inputs](qa-adversarial-inputs/SKILL.md) |
| `qa-black-box-acceptance` | [qa-black-box-acceptance](qa-black-box-acceptance/SKILL.md) |
| `qa-boundary-edge-cases` | [qa-boundary-edge-cases](qa-boundary-edge-cases/SKILL.md) |
| `qa-build-content-metadata` | [qa-package-artifacts](qa-package-artifacts/SKILL.md) |
| `qa-component-tests` | [qa-component-tests](qa-component-tests/SKILL.md) |
| `qa-context-checkpoint` | [qa-context-checkpoint](qa-context-checkpoint/SKILL.md) |
| `qa-contract-tests` | [qa-contract-tests](qa-contract-tests/SKILL.md) |
| `qa-coverage-gap-analysis` | [qa-coverage-gap-analysis](qa-coverage-gap-analysis/SKILL.md) |
| `qa-e2e-tests` | [qa-e2e-tests](qa-e2e-tests/SKILL.md) |
| `qa-environment-preflight` | [qa-environment-preflight](qa-environment-preflight/SKILL.md) |
| `qa-failure-triage` | [qa-failure-triage](qa-failure-triage/SKILL.md) |
| `qa-fault-recovery` | [qa-fault-recovery](qa-fault-recovery/SKILL.md) |
| `qa-final-qualification` | [qa-final-qualification](qa-final-qualification/SKILL.md) |
| `qa-golden-fixture-parity` | [qa-golden-fixture-parity](qa-golden-fixture-parity/SKILL.md) |
| `qa-i18n-localization` | [qa-i18n-localization](qa-i18n-localization/SKILL.md) |
| `qa-independent-review` | [qa-independent-review](qa-independent-review/SKILL.md) |
| `qa-integration-tests` | [qa-integration-tests](qa-integration-tests/SKILL.md) |
| `qa-model-based-testing` | [qa-model-based-testing](qa-model-based-testing/SKILL.md) |
| `qa-mutation-testing` | [qa-mutation-testing](qa-mutation-testing/SKILL.md) |
| `qa-negative-paths` | [qa-negative-paths](qa-negative-paths/SKILL.md) |
| `qa-performance-resources` | [qa-performance-resources](qa-performance-resources/SKILL.md) |
| `qa-positive-paths` | [qa-positive-paths](qa-positive-paths/SKILL.md) |
| `qa-pre-commit` | [qa-pre-commit](qa-pre-commit/SKILL.md) |
| `qa-property-based-testing` | [qa-property-based-testing](qa-property-based-testing/SKILL.md) |
| `qa-regression-selection` | [qa-regression-selection](qa-regression-selection/SKILL.md) |
| `qa-security-boundaries` | [qa-security-boundaries](qa-security-boundaries/SKILL.md) |
| `qa-state-concurrency-lifecycle` | [qa-state-concurrency-lifecycle](qa-state-concurrency-lifecycle/SKILL.md) |
| `qa-static-checks` | [qa-static-checks](qa-static-checks/SKILL.md) |
| `qa-structured-fuzzing` | [qa-structured-fuzzing](qa-structured-fuzzing/SKILL.md) |
| `qa-subagent-dispatch` | [qa-subagent-dispatch](qa-subagent-dispatch/SKILL.md) |
| `qa-system-stabilization` | [qa-system-stabilization](qa-system-stabilization/SKILL.md) |
| `qa-test-orchestrator` | [qa-test-orchestrator](qa-test-orchestrator/SKILL.md) |
| `qa-test-planning` | [qa-test-planning](qa-test-planning/SKILL.md) |
| `qa-unit-tests` | [qa-unit-tests](qa-unit-tests/SKILL.md) |
| `qa-visual-responsive` | [qa-visual-dpi](qa-visual-dpi/SKILL.md) |

Website English/Russian baseline adapted to architecture English-first scope; Russian conditional. Artifact/content metadata: emitted PE/MSI/payload/provenance inspection. Responsive visual checks: native DPI/text scaling/multiple monitors. Preserve architecture 90% security-module executable-line/95% per-critical-file mutation implementation/release gates; no fabricated application checks for instruction files.

## Earlier user-owned Rust adaptation

The user supplied [PdfCraft PR 2912](https://github.com/storytold/pdfcraft/pull/2912). It is a closed, unmerged proposal; it does not establish an installed or qualified toolset here. Inspected the fork head `DavidOsipov/pdfcraft@c9a02b378271f92a73cb44be9fd9e4acc6921516`:

- [Rust verification workflow](https://github.com/DavidOsipov/pdfcraft/blob/c9a02b378271f92a73cb44be9fd9e4acc6921516/.agents/skills/QA/pdfcraft-qa-unit-tests/references/rust-workflow.md): explicit target/feature/compiler matrix, separate build/test concurrency, Cargo flag scope, doctests and runner isolation, actual external consumers. Adapted into `references/rust-techniques.md`, static/contract and environment leaves; no PdfCraft package/version/xtask assumptions retained.
- [Evidence identity](https://github.com/DavidOsipov/pdfcraft/blob/c9a02b378271f92a73cb44be9fd9e4acc6921516/.agents/skills/QA/pdfcraft-qa-unit-tests/references/evidence.md) and [task contract](https://github.com/DavidOsipov/pdfcraft/blob/c9a02b378271f92a73cb44be9fd9e4acc6921516/.agents/skills/QA/pdfcraft-qa-unit-tests/references/contract.md): selected versus executed semantic identities, first failures, known limitations and actual candidate identity. Adapted into this testing contract and qualification/coordination leaves; no nonexistent evidence command/schema introduced.
- [Staged-candidate preparation](https://github.com/DavidOsipov/pdfcraft/blob/c9a02b378271f92a73cb44be9fd9e4acc6921516/.agents/skills/QA/pdfcraft-qa-prepare-commit/references/commit-preparation.md): full-index candidate, transformation/input completeness, ownership and expiring readiness. Condensed into `references/staged-candidate.md` and qa-pre-commit; no automatic exporter, index replacement or staging grant.
- [Independent generation/replay](https://github.com/DavidOsipov/pdfcraft/blob/c9a02b378271f92a73cb44be9fd9e4acc6921516/.agents/skills/QA/pdfcraft-qa-structured-fuzzing/references/generative.md): per-case reset, exact input replay, minimization distinctions and modeled boundary limits. Adapted into Rust techniques and generated-test leaves.

David Osipov's 2026 authorship/MIT grant was inspected in [the pinned notice](https://github.com/DavidOsipov/pdfcraft/blob/c9a02b378271f92a73cb44be9fd9e4acc6921516/.agents/skills/QA/pdfcraft-rust-implementation/THIRD_PARTY_NOTICES.md). Pack-wide retained notices and donor identity are documented in [PROVENANCE.md](../PROVENANCE.md). External linked implementations and third-party skill bodies were not imported.

## Primary Rust and Windows research

| Evidence | Consequence for this pack | Limit |
| --- | --- | --- |
| [Cargo test](https://doc.rust-lang.org/cargo/commands/cargo-test.html), [Cargo targets](https://doc.rust-lang.org/cargo/reference/cargo-targets.html), [rustdoc tests](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html) | Check real selection/features/ignored tests and named execution; all-targets does not include doctests; compiler rejection must have the intended diagnostic | Rolling documentation is not proof of a chosen toolchain's APIs or of executed tests |
| [Clippy usage](https://doc.rust-lang.org/clippy/usage.html) | Use repository/pinned-toolchain policy; select restriction/nursery lints deliberately and validate enforcement with valid/invalid controls | New Cargo warning configuration described in rolling docs cannot be assumed on the documented host/toolchain |
| [Proptest failure persistence](https://proptest-rs.github.io/proptest/proptest/failure-persistence.html), [upstream implementation](https://github.com/proptest-rs/proptest/blob/main/proptest/src/test_runner/failure_persistence/mod.rs) | Preserve seeds and concrete minimized inputs; strategy changes can alter seed replay; keep named deterministic regressions | Optional framework; no dependency added, runner executed or version selected |
| [Loom upstream README](https://github.com/tokio-rs/loom/blob/master/README.md) | Explicit primitive instrumentation and bounded small models; acknowledge documented model incompleteness/false alarms | No automatic OS/FFI coverage or all-interleavings proof |
| [Tokio pause](https://docs.rs/tokio/latest/tokio/time/fn.pause.html), [Tokio advance](https://docs.rs/tokio/latest/tokio/time/fn.advance.html) | Paused Tokio time is separate from std/Windows time; require current-thread/time-test support and observable completion | Runtime helper cannot prove suspend/resume or native actuation |
| [cargo-fuzz README](https://github.com/rust-fuzz/cargo-fuzz/blob/main/README.md), [Rust Fuzz Book setup](https://rust-fuzz.github.io/book/cargo-fuzz/setup.html), [Windows setup](https://rust-fuzz.github.io/book/cargo-fuzz/windows/setup.html) | **Source conflict:** README says Unix-like only/not Windows; current book describes Windows MSVC AddressSanitizer. Require exact release/runner smoke qualification and retain the disagreement | Neither source alone establishes a usable host harness; no nightly/tool installation or fuzz result here |
| [cargo-mutants upstream](https://github.com/sourcefrog/cargo-mutants/blob/main/README.md), [tool guide](https://mutants.rs/) | Use stable baseline and bounded requirement-grounded mutations; reached code differs from assertion sensitivity | Tool is optional until adopted; full critical-file gate still requires actual per-file evidence |
| [Microsoft native accessibility tools](https://learn.microsoft.com/en-us/windows/win32/winauto/accessibility-testingtools), [Inspect](https://learn.microsoft.com/en-us/windows/win32/winauto/inspect-objects/) | UIA inspection, keyboard and Narrator observations remain distinct; prefer Accessibility Insights, with legacy Inspect fallback | Tool output alone is not complete native accessibility qualification |
| [Microsoft DPI behavior](https://learn.microsoft.com/en-us/windows/win32/hidpi/high-dpi-desktop-application-development-on-windows) | Exercise live DPI changes across monitors; manifest declaration alone is insufficient | Follow the current Slint default under OD-13 and qualify actual Windows scaling; the page's generic UWP recommendation does not replace project architecture |
| [MSI rollback custom actions](https://learn.microsoft.com/en-us/windows/win32/msi/rollback-custom-actions), [Windows Installer practices](https://learn.microsoft.com/en-us/windows/desktop/Msi/windows-installer-best-practices) | Fault-test actual compensation and rollback-disabled behavior; arbitrary service/system changes are not automatically restored | Baseline remains a complete per-machine MSI; generic per-user advice is not a product requirement |

Online sources were read through the web tool and public upstream GitHub connector. No application compilation, native Windows check, property/mutation/fuzz campaign or release qualification was executed as part of authoring this testing pack.

**Owner amendment, 5 November 2026:** [OD-13](../../../docs/nevrotick-owner-decisions.md#17-od-13--slint-as-the-default-ui-toolkit)
selects complete replacement of the Win32 UI with Slint. Retained Microsoft
sources explain Windows observation requirements, not Slint API/backend support
or a successful migration. Existing Win32 receipts remain pre-migration evidence;
D1–D3 triggers, execution holds and actual Windows qualification remain separate.

## EGM review coordination adaptation — 29 October 2026

The [review and handoff reference](../planning/references/review-and-handoff.md)
records exact source paths from Google's Elephant-Goldfish Model at
`f0ba151b252c9abf1318d9fe857a43265d06de89`, its Apache-2.0 license and modifications.
Dispatch, checkpoint and coverage skills use that one reference for actual
reviewer completion, stable findings and authority/stage-aware document comparison.
It preserves failed/missing evidence, current authorization and focused rechecks;
reviewer counts or budget exhaustion establish neither correctness nor accepted
risk. No EGM runtime, tester, plugin metadata or fixed swarm was installed.

## CodeQL data-flow workflow refinement — 29 October 2026

Improved the existing locally authored `qa-data-flow-analysis` leaf and its
shared references; no additional overlapping skill or imported skill body.
Context7 `/github/codeql`, official documentation, the installed CodeQL MCP and
pinned library source were compared. Live examples and retrieval summaries are
discovery aids: the admitted CLI 2.27.3 / Rust library 0.2.24 and its source SHA
`a1575b98a8cc19bcfa7bb6cb41be688cb3dae65c` control executable recipes.

| Primary evidence | Applied guidance |
| --- | --- |
| [Rust flow guide](https://codeql.github.com/docs/codeql-language-guides/analyzing-data-flow-in-rust/), [pinned Node API](https://github.com/github/codeql/blob/a1575b98a8cc19bcfa7bb6cb41be688cb3dae65c/rust/ql/lib/codeql/rust/dataflow/internal/Node.qll) | Local/global value flow versus taint; the installed `asExpr()` returns AST `Expr` directly, unlike the newer guide's CFG-node example. Compile against the admitted pack. |
| [Pinned call API](https://github.com/github/codeql/blob/a1575b98a8cc19bcfa7bb6cb41be688cb3dae65c/rust/ql/lib/codeql/rust/elements/internal/CallImpl.qll) | Identify actual callees, runtime targets, receiver and positional argument; fixture name matching is not a production identity model. |
| [Path queries](https://codeql.github.com/docs/writing-codeql-queries/creating-path-queries/), [BQRS interpretation](https://docs.github.com/en/code-security/reference/code-scanning/codeql/codeql-cli-manual/bqrs-interpret) | Flow-specific `PathGraph` and path-node columns; reconcile alert rows, path steps, pagination and limits separately. |
| [Partial-flow debugging](https://codeql.github.com/docs/writing-codeql-queries/debugging-data-flow-queries-using-partial-flow/), [pinned implementation](https://github.com/github/codeql/blob/a1575b98a8cc19bcfa7bb6cb41be688cb3dae65c/shared/dataflow/codeql/dataflow/internal/DataFlowImplStage1.qll) | Verify source/sink membership first, then bounded forward/reverse exploration. Partial reachability does not establish a complete flow or absence. |
| [Rust build options](https://docs.github.com/en/code-security/reference/code-scanning/codeql/build-options-for-compiled-languages), [pinned extractor](https://github.com/github/codeql/blob/a1575b98a8cc19bcfa7bb6cb41be688cb3dae65c/rust/codeql-extractor.yml), [cfg handling](https://github.com/github/codeql/blob/a1575b98a8cc19bcfa7bb6cb41be688cb3dae65c/rust/extractor/src/config.rs) | `none` can run build scripts/proc macros. Host target, all features, test cfg and signature-only dependencies require explicit coverage accounting. The current repository launcher exposes no matrix override. Contrary to a retrieved summary, `-test` can disable the initially enabled test cfg. |
| [Modeling guide](https://codeql.github.com/docs/codeql-language-guides/customizing-library-models-for-rust/), [pinned model predicates](https://github.com/github/codeql/blob/a1575b98a8cc19bcfa7bb6cb41be688cb3dae65c/rust/ql/lib/codeql/rust/dataflow/internal/ModelsAsData.qll), [pack manifest](https://github.com/github/codeql/blob/a1575b98a8cc19bcfa7bb6cb41be688cb3dae65c/rust/ql/lib/qlpack.yml) | Property-specific barriers, branch-sensitive guards and justified summaries; pinned YAML extension loading, rather than assuming newer JSON guidance applies. |
| [Installed MCP source](https://github.com/advanced-security/codeql-development-mcp-server/tree/b0fae2809de0a5ff4fb984677354b7015b6ad57f), [local policy](../../../policy/codeql-mcp.json) | Discover actual schemas; `query_compile.database` is incompatible with the CLI, imported queries need real-file compilation, outputs stay in the admitted workspace, and expected results must not be auto-accepted. |

The recipe and any isolated fixture controls qualify query-development mechanics
only. They supply no current product extraction, Windows behavior, vulnerability
absence or hosted-scanning result; candidate-specific receipts remain outside
the skills pack.

## Generated histories, schedules and admission readers — 15 November 2026

Extended existing lifecycle, model, property and independent-review skills with
original local references; no new skill leaf or imported upstream skill body.
Context7 resolved `/proptest-rs/proptest` and `/hypothesisworks/hypothesis`.
Its Shuttle search returned the unrelated cloud platform; official upstream
GitHub source supplied concurrency-tool evidence. Retrieval summaries were
checked against current API documentation and implementation before integration.

| Primary evidence | Applied guidance and limits |
| --- | --- |
| [Shuttle v0.9.10 source](https://github.com/awslabs/shuttle/tree/855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b), [release](https://github.com/awslabs/shuttle/releases/tag/v0.9.10), [configuration](https://github.com/awslabs/shuttle/blob/v0.9.10/shuttle-engine/src/config.rs), [Tokio time adapter](https://github.com/awslabs/shuttle/blob/v0.9.10/wrappers/tokio/impls/tokio/inner/src/time.rs) | [Concurrency reference](references/shuttle-concurrency.md): actual primitive/adapter mapping, sequentially consistent atomic model, explicit timeout triggers instead of elapsed-time modeling, bounded scheduler and external supervision, persistence directory and exact replay configuration. Changelog date differs from release publication; neither supplies every registry timestamp needed for Cargo admission. |
| [Proptest state-machine API](https://docs.rs/proptest-state-machine/latest/proptest_state_machine/test_runner/trait.StateMachineTest.html), [runner source](https://docs.rs/proptest-state-machine/latest/src/proptest_state_machine/test_runner.rs.html), [failure persistence](https://proptest-rs.github.io/proptest/proptest/failure-persistence.html) | [Sequential reference](references/proptest-state-machines.md): reviewed 0.9.0; explicit invariants because the default hook is a no-op, already-advanced reference state at application, panic-safe owned cleanup, deliberate invalid commands and concrete histories. Generated sequential transitions do not explore thread schedules. |
| [Hypothesis API/settings](https://hypothesis.readthedocs.io/en/latest/reference/api.html), [stateful tests](https://hypothesis.readthedocs.io/en/latest/stateful.html), [replay](https://hypothesis.readthedocs.io/en/latest/tutorial/replaying-failures.html) | [Admission reference](references/hypothesis-admission.md): reviewed 6.170.15; ordinary input properties before stateful machinery, real physical fixtures, integrity-valid semantic faults, independent exact rosters and refusal before effects. Record effective CI/settings/database behavior; concrete regressions outlive seeds and version-specific replay blobs. |

These are dated source observations, not installed versions, admitted graphs or
product qualification. Cargo OD-19 does not automatically establish Python
package policy. Current task permissions, remaining holds and the post-V1 proof
deferral remain authoritative. The historical EP1 omission supplies a defect
class for controls, not a verdict on another agent's unseen uncommitted repair.

### Extended implementation manuals — second review, 15 November 2026

The follow-up reread expanded the three references into operational manuals and
added a [shared selection guide](references/generated-testing-selection.md).
Three tool-specific authors and a separate source-based reviewer checked selection,
integration boundaries, examples, effective configuration, replay and diagnosis.
Context7 discovery was checked against official documentation and pinned source;
an unrelated Shuttle search result was not used as concurrency evidence.
The manuals' examples are original educational material, not imported upstream
tests or executed NevroTick harnesses. The Python example calls the current real
raw reader but does not implement complete semantic-admission qualification.

| Reviewed implementation | Corrections or details the manual now preserves |
| --- | --- |
| [Shuttle scheduler APIs](https://github.com/awslabs/shuttle/blob/855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b/shuttle-schedulers/src/check.rs), [PCT](https://github.com/awslabs/shuttle/blob/855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b/shuttle-schedulers/src/pct.rs), [runner](https://github.com/awslabs/shuttle/blob/855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b/shuttle-engine/src/runtime/runner.rs), [Tokio exports](https://github.com/awslabs/shuttle/blob/855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b/wrappers/tokio/impls/tokio/src/lib.rs) | Reviewed v0.9.10 and its wrapper cohort. DFS iteration count differs from depth; PCT depth differs from a global switch cap; random-seed environment overrides matter. Runner time limits act between executions. Some wrapper exports retain ordinary Tokio, so import aliases alone cannot establish modeled coverage. Full code examples distinguish healthy and intentionally faulty counter implementations and retain exact configured replay. |
| [Proptest 1.12.0 configuration](https://github.com/proptest-rs/proptest/blob/70cf37d46b7d83393b6ed94c611f8c52209f585d/proptest/src/test_runner/config.rs), [macro](https://github.com/proptest-rs/proptest/blob/70cf37d46b7d83393b6ed94c611f8c52209f585d/proptest/src/sugar.rs), [state-machine 0.9.0 strategy](https://github.com/proptest-rs/proptest/blob/70cf37d46b7d83393b6ed94c611f8c52209f585d/proptest-state-machine/src/strategy.rs), [runner](https://github.com/proptest-rs/proptest/blob/70cf37d46b7d83393b6ed94c611f8c52209f585d/proptest-state-machine/src/test_runner.rs) | Macro configuration is contextualized again, allowing ambient `PROPTEST_*` overrides even for explicit fields. State-machine precondition failure uses local rejection in the actual strategy, despite the trait documentation saying global. Direct concrete `test_sequential` calls bypass generated cases and the outer timeout runner. Feature unification, normal-only teardown, post-state application and observed receiver actions are explicit. |
| [Hypothesis 6.170.15 settings](https://github.com/HypothesisWorks/hypothesis/blob/9ab0be61aaaa27396fc1bbf4b34e7925cc68d1e2/hypothesis/src/hypothesis/_settings.py), [stateful adapter](https://github.com/HypothesisWorks/hypothesis/blob/9ab0be61aaaa27396fc1bbf4b34e7925cc68d1e2/hypothesis/src/hypothesis/stateful.py), [seed selection](https://github.com/HypothesisWorks/hypothesis/blob/9ab0be61aaaa27396fc1bbf4b34e7925cc68d1e2/hypothesis/src/hypothesis/core.py), [pytest option](https://github.com/HypothesisWorks/hypothesis/blob/9ab0be61aaaa27396fc1bbf4b34e7925cc68d1e2/hypothesis/src/_hypothesis_pytestplugin.py) | Generated state-machine `.TestCase` defaults disable deadlines and suppress all health checks; explicit settings restore the intended checks. Seed precedence is decorator, then derandomization, then CLI global seed. Class-level seeded replay must retain explicit database settings because the adapter forwards seed/reproduction attributes, not decorated settings. Database replay uses `Phase.reuse`; a tutorial sentence referring to `Phase.explain` does not control this API. |
| [Loom documented limitations](https://github.com/tokio-rs/loom#unsupported-features) and the existing local tool references | Selection distinguishes controlled schedules, small atomic models, sequential histories, generated Python inputs, deterministic anchors, fuzzing and static flow analysis. Loom's documented C11 gaps are retained; no exhaustive or native qualification is inferred. |

These are version-specific observations. Recheck the admitted release/source when
implementing; current docs cannot silently update dependency policy or replay
identities. Source/API review and Python syntax parsing establish neither runtime
success nor product correctness. The existing task record owns validation and
publication evidence; this registry owns source provenance.

### Missing-framework setup — 16 November 2026

The [setup reference](references/generated-testing-setup.md) closes the follow-up
gap between method selection and an available, admitted runner. A repository
auditor checked actual package declarations and setup/analysis entrypoints;
separate Rust and Python researchers checked primary installation sources through
MCPs. Context7 discovery was reconciled with pinned manifests rather than using
older retrieved Proptest examples as current package facts.

| Primary evidence | Applied setup guidance |
| --- | --- |
| [Cargo executable installation](https://doc.rust-lang.org/cargo/commands/cargo-install.html), [integration tests](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#integration-tests), [Shuttle manifest](https://github.com/awslabs/shuttle/blob/855ae5d92a02dd3c02ddd3ca99dfaeb3b5e31d5b/shuttle/Cargo.toml), [Proptest manifest](https://github.com/proptest-rs/proptest/blob/70cf37d46b7d83393b6ed94c611f8c52209f585d/proptest/Cargo.toml) | Testing libraries use an owning Cargo manifest and actual test-build topology. Optional normal dependencies may be needed for production-library instrumentation. Existing guarded resolution, acquisition and exact runner feature/dependency checks remain authoritative. |
| [Hypothesis package metadata](https://github.com/HypothesisWorks/hypothesis/blob/9ab0be61aaaa27396fc1bbf4b34e7925cc68d1e2/hypothesis/pyproject.toml), [Python venv](https://docs.python.org/3/library/venv.html), [ensurepip](https://docs.python.org/3/library/ensurepip.html) | Reviewed Hypothesis requires Python 3.11+, includes a native extension and uses Maturin for source builds. Distinguish interpreter, venv/pip, distribution and native/import failures; choose compatible wheels and the exact environment interpreter. The optional CLI and pytest integration are separate choices. |
| [pip installation/report](https://pip.pypa.io/en/stable/cli/pip_install/), [report schema](https://pip.pypa.io/en/stable/reference/installation-report/), [secure installs](https://pip.pypa.io/en/stable/topics/secure-installs/), [download](https://pip.pypa.io/en/stable/cli/pip_download/), [pytest plugins](https://docs.pytest.org/en/stable/how-to/plugins.html) | Wheel-only resolution proposal, reviewed exact transitive hash closure, bounded acquisition, offline local-wheel installation and compatibility/control checks. A report is not a lock; `--no-index` alone is not network isolation. Verify the selected pytest plugin and nodes separately from direct invocation. |

This guidance supplies concrete preparation recipes without installing anything,
choosing a Python package policy, changing product graphs or inventing configured
analysis selectors. Current task authorization governs necessary preparation;
skills create no extra approval round. Version observations and illustrative
paths must be reconciled with the actual target before execution.

### Follow-up skill audit — 16 November 2026

Independent Rust/API, Python/setup and workflow reviewers rechecked the recent
tool manuals and owning skill routes against candidate `f06d57b0`. The selected
Shuttle, Proptest and Hypothesis API/version guidance needed no correction.
The following local instruction changes retain the same tool/dependency choices:

| Evidence | Guidance correction |
| --- | --- |
| [Engine reader](../../../scripts/verify-engine-analysis.py), [shared tooling imports](../../../scripts/portable-analysis.py), [product verifier](../../../scripts/verify.py), [advisory module](../../../scripts/advisory_snapshot.py), [bounded reader](../../../scripts/check-analysis.py); Python [`resource`](https://docs.python.org/3/library/resource.html) and [open-flag availability](https://docs.python.org/3/library/os.html#os.O_CLOEXEC) | The concrete Hypothesis reader example requires the admitted Linux/WSL interpreter. Native Windows setup remains available for a separately selected Windows-compatible consumer; no I/O protection is removed to make this example run. |
| [Testing contract](CONTRACT.md#scope-expectations-and-repairs) and [review-only evaluation rule](qa-skill-evaluations/SKILL.md) | A complete source demonstration can establish a source-confirmed defect. Unresolved assumptions remain explicit; source reasoning supplies no executed host/native result or missing required gate. |
| [CodeQL launcher](../../../scripts/codeql-mcp.py) and [existing finalization controls](../../../TOOLING.md#local-codeql-development-mcp) | Mention the existing retained-database finalization option with the same source/ownership controls and execution holds. No new launcher capability is added. |

These findings come from source and instruction review. The existing skill-quality
task owns independent review, practice-case and candidate-validation evidence;
none establishes framework installation or product/native execution.

## Sanitizer qualification manuals — 28 November 2026

The original local [qualification coordinator](qa-sanitizer-qualification/SKILL.md)
adds conditional [ASan](qa-sanitizer-qualification/references/asan.md),
[LSan](qa-sanitizer-qualification/references/lsan.md),
[TSan](qa-sanitizer-qualification/references/tsan.md),
[MSan](qa-sanitizer-qualification/references/msan.md) and
[native UBSan](qa-sanitizer-qualification/references/ubsan.md) manuals. The
[shared reference](references/sanitizers.md) remains the common selection/control
owner. This amendment retains the 31 October research and historical detector
receipts; it neither redates them nor promotes them to product qualification.

Checked **28 November 2026** against primary Rust, LLVM, cargo-fuzz, Miri and
Microsoft documentation. Context7 discovery for `/rust-lang/rust`,
`/rust-fuzz/cargo-fuzz` and `/rust-lang/miri` was reconciled with the pinned
upstream sources below. Microsoft Learn pages and the published Rust Fuzz Book
are rolling guidance checked on that date. No upstream implementation or skill
body was imported. Research and authored instructions install no tool, admit no
application runner and execute no product, native or deliberately faulty program.

### Rust, LLVM and harness sources

| Immutable primary evidence | Manual consequence and limit |
| --- | --- |
| Rust **1.100.0**, commit `0ef1bc386059596dada8fb65b50425aab2ea637a`: [sanitizer guidance](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/src/doc/unstable-book/src/compiler-flags/sanitizer.md) and [target options](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/compiler/rustc_target/src/spec/mod.rs) | Explicit target keeps target instrumentation away from host build scripts/procedural macros. Recompiled `std`, native instrumentation and compatible runtimes constrain results; `-Zexternal-clangrt` requires its own integration qualification. TSan has fence/assembly synchronization gaps. The documented Rust modes contain no standard UBSan mode. `supported_sanitizers` and `default_sanitizers` describe code generation support even if supporting libraries are not distributed. |
| Same Rust commit: [ASan target](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/src/doc/rustc/src/platform-support/x86_64-unknown-linux-gnuasan.md), [MSan target](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/src/doc/rustc/src/platform-support/x86_64-unknown-linux-gnumsan.md), [TSan target](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/src/doc/rustc/src/platform-support/x86_64-unknown-linux-gnutsan.md) | These tier 2 targets document default instrumentation and instrumented standard libraries without nightly `build-std`. They are conditional preparation routes, not a claim that the existing installation includes them or that native dependencies are instrumented. Classic `-Zsanitizer`/`build-std` remains a separate nightly route. |
| LLVM commit `a6321f2d019cc9563b2f552e486b7550d3ec51ce`: [ASan](https://github.com/llvm/llvm-project/blob/a6321f2d019cc9563b2f552e486b7550d3ec51ce/clang/docs/AddressSanitizer.md) and [LSan](https://github.com/llvm/llvm-project/blob/a6321f2d019cc9563b2f552e486b7550d3ec51ce/clang/docs/LeakSanitizer.md) | Reached address/lifetime checking and unreachable-allocation detection have different controls. Supported ASan/LSan integration is valid; actual options, embedded defaults, suppressions and programmatic exclusions affect coverage. LSan-specific options still use `LSAN_OPTIONS` with ASan. These diagnostic runtimes are not intended for production executables. |
| Same LLVM commit: [TSan](https://github.com/llvm/llvm-project/blob/a6321f2d019cc9563b2f552e486b7550d3ec51ce/clang/docs/ThreadSanitizer.md) and [MSan](https://github.com/llvm/llvm-project/blob/a6321f2d019cc9563b2f552e486b7550d3ec51ce/clang/docs/MemorySanitizer.md) | Uninstrumented native code can hide races or create false reports. MSan's libc interceptors do not cover arbitrary external libraries or memory-writing assembly. Record actual graph/interceptor coverage, origin tracking and runtime options. Shadow-memory, static-linking and resource failures are setup evidence, not detected product defects; neither tool supplies exhaustive behavioral proof. |
| Same LLVM commit: [Users Manual](https://github.com/llvm/llvm-project/blob/a6321f2d019cc9563b2f552e486b7550d3ec51ce/clang/docs/UsersManual.md) and [UBSan](https://github.com/llvm/llvm-project/blob/a6321f2d019cc9563b2f552e486b7550d3ec51ce/clang/docs/UndefinedBehaviorSanitizer.md) | ASan/TSan/MSan require separate configurations. UBSan instruments the selected native translation units/checks; its `undefined` group is not every available check. Many checks recover by default, so exit zero cannot erase captured findings. Trap-only evidence needs attributable mechanism/site evidence. Linking a runtime adds no absent Rust instrumentation. |
| cargo-fuzz **0.13.2**, commit `bf2fc668dafda5295aa6fd01825ee67b885f0f2b`: [manifest](https://github.com/rust-fuzz/cargo-fuzz/blob/bf2fc668dafda5295aa6fd01825ee67b885f0f2b/Cargo.toml), [options](https://github.com/rust-fuzz/cargo-fuzz/blob/bf2fc668dafda5295aa6fd01825ee67b885f0f2b/src/options.rs), [invocation](https://github.com/rust-fuzz/cargo-fuzz/blob/bf2fc668dafda5295aa6fd01825ee67b885f0f2b/src/project.rs) and [compiler-version handling](https://github.com/rust-fuzz/cargo-fuzz/blob/bf2fc668dafda5295aa6fd01825ee67b885f0f2b/src/rustc_version.rs) | Selectors are address/leak/memory/thread/none. Memory, explicit `build_std` or careful mode requests `build-std`, except coverage mode. The option's default fixture is false despite a contradictory help comment; do not copy Context7's conflicting default claim. Future `-Csanitizer` selection uses an unreached version placeholder, leaving the actual route `-Zsanitizer`. Inherited `RUSTFLAGS` append after generated flags and can override them. Wrapper ASan/TSan options append `detect_odr_violation=0`/`report_signal_unsafe=0`; direct launches require their own effective-option evidence. |
| Miri commit `6287e97599b9cd3b7d6334e54dde783145297b01`: [README](https://github.com/rust-lang/miri/blob/6287e97599b9cd3b7d6334e54dde783145297b01/README.md) | Miri complements sanitizer checks on particular interpreted executions; weak-memory exploration and platform APIs/FFI are limited. Experimental native-library support loses initialization/provenance tracking and may miss UB, so it cannot establish full native-boundary coverage. Existing admitted Miri compiler/sysroot and nonformal evidence scope remain separate. |

### Native Windows sources and source conflicts

| Primary evidence | Applied Windows boundary |
| --- | --- |
| Rust 1.100.0 [Windows target](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/compiler/rustc_target/src/spec/targets/x86_64_pc_windows_msvc.rs), the pinned Rust sanitizer book above, cargo-fuzz [README](https://github.com/rust-fuzz/cargo-fuzz/blob/bf2fc668dafda5295aa6fd01825ee67b885f0f2b/README.md), and [Rust Fuzz Book Windows setup](https://rust-fuzz.github.io/book/cargo-fuzz/windows/setup.html) | Preserve the conflict: target source admits ADDRESS while the Rust book omits Windows; cargo-fuzz README says Unix-like only while its book documents MSVC setup. Code generation support or a setup page proves neither installed runtime availability nor a qualified Rust/native runner. Native controls, target, compiler and graph decide the admitted claim. |
| [Microsoft ASan overview](https://learn.microsoft.com/en-us/cpp/sanitizers/asan?view=msvc-170), [building](https://learn.microsoft.com/en-us/cpp/sanitizers/asan-building?view=msvc-170) and [runtime](https://learn.microsoft.com/en-us/cpp/sanitizers/asan-runtime?view=msvc-170) | Microsoft's stated target support applies to its C/C++ implementation, not automatic Rust qualification. Establish runtime-DLL availability/lifetime, compiler/runtime versions and actual allocator interception. Static CRT selection does not eliminate the modern ASan runtime DLL dependency. |
| [Microsoft known issues](https://learn.microsoft.com/en-us/cpp/sanitizers/asan-known-issues?view=msvc-170) and [runtime flags](https://learn.microsoft.com/en-us/cpp/sanitizers/asan-flags?view=msvc-170) | Partial instrumentation, DLL unload and allocator overrides constrain detection. Check the selected use-after-return instrumentation and runtime option together; record actual allocation/deallocation-mismatch settings. ASan shadow-memory first-chance exceptions and debugger handling must not redefine the existing PageHeap/CDB fault policy. These sources justify no blanket CFG/CET disablement. |
| Existing admitted Rust LLVM revision `f738f3b0ed65c01edcb44a504a77edc12a82c319`: [Windows allocation interceptors](https://github.com/rust-lang/llvm-project/blob/f738f3b0ed65c01edcb44a504a77edc12a82c319/compiler-rt/lib/asan/asan_malloc_win.cpp) and [LSan platform conditions](https://github.com/rust-lang/llvm-project/blob/f738f3b0ed65c01edcb44a504a77edc12a82c319/compiler-rt/lib/lsan/lsan_common.h) | Allocation interception overlaps native heap observation, while LSan's platform conditions exclude Windows. Keep ASan and the existing AppVerifier/PageHeap settings as separately qualified profiles unless their exact combination is admitted. This is a conservative repository integration rule, not a claim of universal tool incompatibility. |
| [GFlags/PageHeap](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/gflags-and-pageheap), [GFlags commands](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/gflags-commands), [Application Verifier testing](https://learn.microsoft.com/en-us/windows-hardware/drivers/devtest/application-verifier-testing-applications), [tests](https://learn.microsoft.com/en-us/windows-hardware/drivers/devtest/application-verifier-tests-within-application-verifier), [debugging stops](https://learn.microsoft.com/en-us/windows-hardware/drivers/devtest/application-verifier-debugging-application-verifier-stops), [FAQ](https://learn.microsoft.com/en-us/windows-hardware/drivers/devtest/application-verifier-faqs) and [`!avrf`](https://learn.microsoft.com/en-us/windows-hardware/drivers/debuggercmds/-avrf) | Preserve per-image ownership, persistent-setting readback/restoration, actual allocator/API coverage and intended token. GFlags `/leaks` changes PageHeap mode; AppVerifier leak-layer/unload observations and outstanding-resource listings are not a general absence-of-leaks result. The existing Windows-memory reference owns native execution and separate retention measurements. |

The initial inspected NevroTick baseline was
`172d03517123e6c4947fc94074b9b15a17dddc61`; implementation was reconciled against
`36849ebfc58ae0af1df5ee778b9d163b65acf492`, preserving its later tool-pin refresh.
The
[standalone detector profiles](../../../TOOLING.md#standalone-sanitizer-detector-controls)
remain fixed controls: Rust ASan/LSan use `no_std`; TSan/MSan use instrumented
`core`/`compiler_builtins`; native UBSan uses its separate Clang runtime. No
full-`std`, product/native graph, mixed-runtime or Windows product qualification
is inferred. Current checkpoint cadence and applicability come from the owner
decision and coordinator, not from upstream setup examples. Required unavailable
product evidence stays incomplete; tool success and source review do not close it.

## Additional sanitizers and hardening — 28 November 2026

The coordinator now routes conditional [data-flow](qa-sanitizer-qualification/references/dataflow.md),
[real-time](qa-sanitizer-qualification/references/realtime.md),
[memory-tagging](qa-sanitizer-qualification/references/tagged-memory.md) and
[control-flow/stack](qa-sanitizer-qualification/references/control-flow.md) guidance.
Three source specialists and a fresh judge assessed applicability against NevroTick
`249dcee18a8a1567bcc2c7b89e395cce220dc47a`; David then requested the guidance edits.
GitHub source retrieval, Microsoft Learn search/full-page retrieval and official
documentation supplied the evidence. This is locally authored instruction, not
imported implementation, runner admission, campaign adoption or product execution.

| Primary evidence checked on 28 November 2026 | Applied distinction |
| --- | --- |
| Rust 1.100.0 `0ef1bc386059596dada8fb65b50425aab2ea637a`: [Windows x64 target](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/compiler/rustc_target/src/spec/targets/x86_64_pc_windows_msvc.rs), [GNU x64 target](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/compiler/rustc_target/src/spec/targets/x86_64_unknown_linux_gnu.rs), [GNU AArch64 target](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/compiler/rustc_target/src/spec/targets/aarch64_unknown_linux_gnu.rs) | Windows x64 lists ADDRESS only. GNU x64 includes CFI, DATAFLOW, SAFESTACK and REALTIME but lacks HWADDRESS/MEMTAG; GNU AArch64 includes HWADDRESS/MEMTAG. These are compiler eligibility facts, not stable-channel switches, complete runtimes or admitted product profiles. Native exclusions and supported portable investigations retain separate scopes. |
| Same Rust revision: [sanitizer book](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/src/doc/unstable-book/src/compiler-flags/sanitizer.md), [RTSan async lint](https://github.com/rust-lang/rust/blob/0ef1bc386059596dada8fb65b50425aab2ea637a/compiler/rustc_lint_defs/src/builtin.rs) | Direct Rust DFSan and RTSan integration exists through unstable interfaces. Rust RTSan annotations do not cover all executor work at async suspension points. CFI needs compatible LTO/graph handling and may reject calls allowed by ordinary Rust ABI rules; a termination requires interpretation. |
| LLVM `a6321f2d019cc9563b2f552e486b7550d3ec51ce`: [runtime platform gates](https://github.com/llvm/llvm-project/blob/a6321f2d019cc9563b2f552e486b7550d3ec51ce/compiler-rt/cmake/config-ix.cmake) | The examined DFSan runtime is enabled for Linux; RTSan for Darwin/Linux. A separately supported analysis target cannot supply native Windows evidence or remove the admitted shipping profile. |
| LLVM [DFSan manual](https://clang.llvm.org/docs/DataFlowSanitizer.html), [design](https://clang.llvm.org/docs/DataFlowSanitizerDesign.html), pinned [interface](https://github.com/llvm/llvm-project/blob/a6321f2d019cc9563b2f552e486b7550d3ec51ce/compiler-rt/include/sanitizer/dfsan_interface.h) and [instrumentation](https://github.com/llvm/llvm-project/blob/a6321f2d019cc9563b2f552e486b7550d3ec51ce/llvm/lib/Transforms/Instrumentation/DataFlowSanitizer.cpp) | Labels need named sources, permitted transformations and sinks, instrumented propagation or an explicit boundary model, and a separate domain oracle. Pointer/select options, conditional callbacks, suppressions and zero-label behavior constrain interpretation. Validate propagation independently of sink assertions; DFSan supplies neither a built-in application-policy violation diagnostic nor complete implicit-flow/noninterference proof. |
| LLVM [RTSan manual](https://clang.llvm.org/docs/RealtimeSanitizer.html) and pinned [POSIX interceptors](https://github.com/llvm/llvm-project/blob/a6321f2d019cc9563b2f552e486b7550d3ec51ce/compiler-rt/lib/rtsan/rtsan_interceptors_posix.cpp) | Establish the explicit synchronous prohibited-operation contract and intercepted graph. Nonblocking socket configuration does not make an intercepted prohibited call acceptable; custom blocking and async/executor work need separate coverage. Keep legally allocating setup and rejection paths within their actual contracts. No WCET, deadline, jitter, UTC-accuracy or native Windows result follows. |
| LLVM [HWASan guide](https://clang.llvm.org/docs/HardwareAssistedAddressSanitizer.html), [design](https://clang.llvm.org/docs/HardwareAssistedAddressSanitizerDesign.html), pinned [runtime header](https://github.com/llvm/llvm-project/blob/a6321f2d019cc9563b2f552e486b7550d3ec51ce/compiler-rt/lib/hwasan/hwasan.h) and [Linux runtime](https://github.com/llvm/llvm-project/blob/a6321f2d019cc9563b2f552e486b7550d3ec51ce/compiler-rt/lib/hwasan/hwasan_linux.cpp) | HWASan does not require MTE. LLVM's experimental x86 mechanisms do not establish support in the pinned Rust x86 target. A supported AArch64 Linux diagnostic can be considered before an ARM product port, with its own graph/runtime controls and bounded claims. |
| Linux [MTE interface](https://docs.kernel.org/arch/arm64/memory-tagging-extension.html), Android [MTE guide](https://developer.android.com/ndk/guides/arm-mte), LLVM [MemTagSanitizer](https://llvm.org/docs/MemTagSanitizer.html) | Verify hardware, OS, mappings, allocator and effective checking mode. Android allocator-backed heap tagging may work without recompiling code; stack instrumentation is separate. LLVM's October 2019 status text is historical, not a current platform inventory. Shipping MTE remains a deployment decision with its own qualification. |
| LLVM [CFI](https://clang.llvm.org/docs/ControlFlowIntegrity.html), [SafeStack](https://clang.llvm.org/docs/SafeStack.html) and [ShadowCallStack](https://clang.llvm.org/docs/ShadowCallStack.html) | These mechanisms have distinct protected properties, graph/ABI/runtime dependencies and target limits. Software ShadowCallStack is distinct from hardware CET. Portable experiments do not replace existing Windows CFG/CET requirements or establish effective shipping enforcement. |
| Rust [CFG code generation](https://doc.rust-lang.org/rustc/codegen-options/index.html#control-flow-guard), Microsoft [CFG](https://learn.microsoft.com/en-us/cpp/build/reference/guard-enable-control-flow-guard?view=msvc-170), [CETCOMPAT](https://learn.microsoft.com/en-us/cpp/build/reference/cetcompat?view=msvc-170), [CFG process policy](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_control_flow_guard_policy) and [hardware shadow-stack process policy](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_user_shadow_stack_policy) | Compiler/linker settings and image declarations are distinct from effective process enforcement. Preserve required CFG, CETCOMPAT and CETCOMPATSTRICT with the security/build/native owners. Initial hardware shadow-stack activation cannot be supplied later by the ordinary setter when initially disabled; inspect strict/audit/module state separately. The x86-64-v3 CPU floor does not imply CET capability. |

The [canonical applicability matrix](../../../docs/windows-time-development-hardening.md#954-additional-sanitizer-and-hardening-applicability)
owns the project disposition. A conditional research candidate is not an adopted
required campaign. Existing five-family detector controls admit no additional
product profile, and this source review supplies no product or native qualification.
