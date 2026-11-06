# Slint and iced: source, resource and project-health assessment

**Decision and research date:** 5 November 2026, Asia/Tbilisi. Activity windows
below use UTC.<br>
**Status:** retained supporting research; **David decided to continue with
Slint** after considering this comparison.<br>
**Canonical decision:** [OD-13 — Slint as the default UI toolkit](../nevrotick-owner-decisions.md#17-od-13--slint-as-the-default-ui-toolkit).<br>
**Execution owners:** [S0.5](../../tasks/s0.5-ui-comparison.md) and the existing
[UI-M0 migration handoff](../../tasks/ui-m0.md#slint-migration-handoff--5-november-2026).

## 1. Decision and practical conclusion

David's final instruction was to retain this research in the repository together
with “my decision to go on with Slint.” This **reaffirms OD-13**; it creates no
new competing toolkit decision. Slint remains the default for subsequent
NevroTick UI work. The current Win32 implementation remains the behavioral and
measurement reference until its Slint replacement is qualified.

The evidence supports a qualified choice, rather than a claim that one framework
wins every engineering category:

| Question | Retained conclusion | Evidence strength and limit |
| --- | --- | --- |
| Which measured candidate uses fewer resources? | Both recorded modest footprints; no clear runtime-resource winner. | Both pilots now have retained S0.5 provenance and recorded independent recalculation. Separate cohorts and small idle-memory differences do not establish general efficiency. |
| Which source design is easier to reason about in Rust? | **iced has the advantage in the inspected ownership, task and message architecture.** | Comparative engineering judgment from bounded source inspection, not a defect-rate measurement or whole-graph safety proof. |
| Which project shows broader verification and more immediately available accessibility machinery? | **Slint has the advantage in the inspected compiler/runtime test infrastructure and existing AccessKit integration.** | Configuration, selected successful CI runs and source integration were observed. None qualifies NevroTick's actual Windows UI. |
| Which has the larger public following? | **iced**, by the observed GitHub stars and forks. | Following is not a count of users, contributors, maintainers or supported products. |
| Which currently shows greater development throughput and a broader active contributing group? | **Slint**, in both the 90-day and 365-day windows. | Scope, contribution style and company structure confound raw count comparisons. More commits do not prove better code. |
| Does age explain the difference as modern iced versus legacy Slint? | **No.** Both are comparatively recent Rust GUI projects and both show substantial modernization. | Slint's project start is later; a stable API does not imply frozen internals. |
| What should NevroTick do? | **Continue with Slint under OD-13 and qualify the actual admitted candidate.** | Owner decision informed by the combined evidence. Licensing, source concerns and product gates remain open. |

The recommendation does not rest on Slint's 1.74 MiB private-memory advantage
in the reported comparison. The more consequential considerations are delivery
fit, accessibility implementation, verification breadth and continuity of
maintenance. iced remains a credible alternative with attractive Rust design;
the research does not justify calling it abandoned, inherently unsafe or poorly
engineered.

## 2. Scope, versions and evidence rules

The review inspected released code and development code separately. These are
**source-review identities**, not automatically the identities of the measured
executables:

| Framework | Released snapshot | Development snapshot inspected on 5 November |
| --- | --- | --- |
| iced | 0.14.0, [`3997291f318a8bc06fa522f5579836fb3feb94df`][iced-stable] | 0.15.0-dev, [`8ff48c42b0d5188223501ea1ccc506fd5beaf36a`][iced-dev] |
| Slint | 1.18.1, [`372cf0ee5577c3dfec309a45e7b778ba4e81b734`][slint-stable] | [`d7fbf11316eb10ad58302d66ce5de379b1867a93`][slint-dev] |

The inspected areas included state and widget ownership, asynchronous work,
event delivery, renderer/window lifecycle, unsafe boundaries, accessibility,
compiler/runtime tests, CI, releases, Git history and contributor workflow.
The review did not compile either upstream, run a new framework experiment,
execute the suspected failure paths, audit every dependency, or measure a
framework-wide defect rate. Existing CI runs were inspected separately from
workflow configuration. No successful independent second-review result was
obtained for the source findings; they must not be labeled independently
confirmed.

This document uses four distinct evidence categories:

- **Observed:** a named source contract, workflow, upstream state or computed
  repository statistic at the stated snapshot.
- **Reported execution evidence:** experiment information supplied by David
  and retained in S0.5, with its custody and scope limits stated below. This
  documentation review did not rerun the experiments or independently reopen
  their private raw artifacts.
- **Inference:** a reasoned comparison of maintainability, delivery risk or
  continuity; no statistical probability is claimed.
- **Unconfirmed finding:** a specific source concern requiring a reproducer or
  further lifecycle analysis and upstream disposition. It is not a confirmed
  NevroTick defect, vulnerability, exploit or CVE.

Qualitative confidence is stronger for source contracts and reproduced Git
counts than for predictions about future maintenance. No numerical confidence
percentage is retained: this research did not define a calibrated probability
model.

## 3. Resource experiment and what it establishes

David reported the completed iced experiment as follows:

> Iced measured 9.47 MiB of visible private memory and 23.07 MiB of working set.
> The retained Slint figures are 7.73 MiB and 23.63 MiB respectively. Both
> recorded zero CPU-time change during the sampled idle phases. Iced’s EXE is
> smaller; these measurements show both configurations have modest footprints,
> with no clear runtime-resource winner.

The rounded summary was subsequently reconciled with the full
[S0.5 Slint record](../../tasks/s0.5-ui-comparison.md#slint-software-renderer-live-host-result--5-november-2026)
and the newly published
[S0.5 iced record](../../tasks/s0.5-ui-comparison.md#iced-software-renderer-pilot--5-november-2026),
present by repository commit `efd246fc8447b8f181879f9e6ae7c6a73bb8dc80`.
Both retain candidate/build/observer identities, custody hashes and independent
raw-data recalculation results. The raw artifacts remain private; this report
does not claim to have rerun or separately recalculated them.

Memory values are medians of three trial medians; parentheses show their range.
The precise values support the rounded summary above.

| Retained measure | Slint 1.18.1 / software | iced 0.14.0 / tiny-skia |
| --- | ---: | ---: |
| Visible private memory, MiB | 7.730 (7.719–7.762) | 9.473 (9.453–9.531) |
| Visible working set, MiB | 23.629 (23.594–23.664) | 23.066 (23.066–23.168) |
| Minimized private memory, MiB | 7.730 (7.719–7.762) | 9.473 (9.453–9.531) |
| Minimized working set, MiB | 23.641 (23.605–23.672) | 23.109 (23.074–23.176) |
| Process CPU-time change, visible/minimized | 0 ms in every sampled phase | 0 ms in every sampled phase |
| Sampled threads, visible/minimized | 8 / 5–8 | 24 / 21–24 |
| Sampled process handles | 208–209 | 203–204 |
| Standalone release EXE, bytes | 12,766,720 | 4,097,536 |
| Handwritten UI adapter lines, shared model excluded | 228 | 249 |

The visible differences, rounded from the retained byte medians, are
**+1.742 MiB private memory** and
**−0.563 MiB working set** for iced. Its smaller EXE does not establish a smaller
complete MSIX or lower physical-memory footprint. Private bytes are commitment;
working set includes shared and file-backed resident pages. They are not
additive measurements.

Slint's configuration used explicit winit/software rendering and Fluent-style
widgets. iced used defaults disabled with only `tiny-skia` and `thread-pool`:
resolved components included `iced_tiny_skia 0.14.1`, `iced_widget 0.14.2` and
`iced_winit 0.14.1`. Its native compositor initialization needs an executor for
`runtime.block_on`; the original tiny-skia-only proposal was corrected before
building because the null backend leaves that operation unimplemented. The
default worker pool is included in the measured footprint, although the fixture
has no asynchronous tasks, subscriptions or application timers. Sixteen extra
iced threads are consistent with that pool, without identifying every thread's
owner. This graph is evidence for a private experiment, not admitted product
dependencies.

Both used the unchanged checked fixture model and the same Windows 11 Pro
10.0.26300 / Ryzen 7 7735HS host, with 16 logical processors and Balanced power
mode. The iced record reports a Rust 1.99.1 MSVC x64 release build with the
earlier pilots' optimization and mitigation settings. Its first compiler build
passed in 136.93 seconds with one unchanged-model unused-item warning; it was
not a Clippy or full-workspace qualification. The adapter retained
`forbid(unsafe_code)`. The 108 selected package/version labels, ten build-script
crates and seven proc-macro crates were inventoried; archive/source checks and
system/runtime DLL imports are retained in S0.5.

The iced cohort comprised three runs, each with a three-second warm-up,
30-second visible Status idle and 30-second minimized idle: **186 samples**.
All three closed normally with exit code zero and no forced cleanup; the final
inventory found no owned iced process. The capture established the visible
Status fixture, not navigation, editing or accessibility behavior. Slint was
not rerun: iced was compared with retained historical Slint data. Both had
1200 × 900 physical client rectangles at DPI 144, but **iced was unfocused and
Slint was focused**, layouts differed, and equal background host load was not
established. Slint's older comparison with Win32 had additional DPI differences.

For continuation, S0.5 retains the exact private artifact locations and full
custody manifests. The iced EXE SHA-256 is
`fe94ea4ca216a0f9e839b9ea70ead83e179d1d40c7f95851832722937260b210`;
its measurement summary is
`7d4edbfcb4edc73289da5f69353752a2160279e7a3e577936bda34f102857703`;
its 98-file measurement-evidence manifest is
`26046626169bf5da0b5d05ed4415c151ad49183c4e8c088a23ca5b042a54f082`.
Slint's measurement summary is
`1fd0a3af7d1547f24e4dfb0c354d40c2f282244191f67ae11db92ca8ac13835a`
and measurement-evidence manifest is
`9649184d983dbd0090f94f99de2c368ddcee331f3cd27c79c74e8e9d4d76cf7f`.
These identifiers aid custody; copying them is not new verification of the
private artifact bytes.

The defensible interpretation is narrow. No detected CPU-time increment over
short idle phases does not mean a process consumes literally no CPU, never
wakes, or remains idle under realistic service updates. Private memory and
working set are different measurements; neither alone measures total cost.
The results establish no winner for responsiveness, rendering load, sustained
updates, long-running retention, battery use or package size. Layout/DPI and
feature differences also limit equivalence. In particular, the Slint pilot
disabled accessibility and tray support, so the numbers do not qualify a fully
featured production configuration. Neither pilot establishes a one-hour
baseline, 72-hour soak, clean-machine runtime availability or release package.

The earlier Slint/Win32 comparison also recorded a 228-line Slint adapter and a
1,432-line Win32 implementation. That suggests reduced fixture UI coding, but
different scope and generated/runtime work prevent interpreting it as a
sixfold reduction in total complexity or maintenance cost. Historical
Win32/egui/GPUI/Reactor results remain in S0.5; they are not a controlled
same-configuration ranking of every framework.

## 4. Source design and engineering practices

### 4.1 iced: clear Rust boundaries, with limits to automatic guarantees

The inspected iced code separates core types, widgets, runtime actions,
asynchronous subscriptions, window integration and renderers. Its
state/message/update/view model gives application behavior an explicit route.
Persistent widget state uses a tree with `Any`/type identity, rather than a
general custom ownership system. Native futures and streams carry `Send`
requirements, with local alternatives scoped for WebAssembly. These properties
make important application contracts relatively easy to locate and audit.
See the [runtime][iced-runtime], [future abstractions][iced-futures] and
[widget tree][iced-tree].

Specific positive observations include:

- [Tasks][iced-task] expose cancellation handles and abort-on-drop behavior;
  [subscription tracking][iced-subscriptions] cancels recipes that disappear.
  Cancellation ownership is visible, although an application still needs to
  decide which work should survive a view change or shutdown.
- [Clipboard ownership][iced-clipboard] retains the window through an `Arc`,
  declares the clipboard/window field order deliberately, documents the unsafe
  invariant and represents connection failure as unavailable clipboard access.
- The [workspace lints][iced-cargo] deny unsafe code by default, require public
  documentation and check unused results. Exceptions remain possible; this is
  not a repository-wide `forbid(unsafe_code)` guarantee. The inspected lint CI
  treats warnings as errors.
- The [window integration][iced-winit] distinguishes lost/outdated rendering
  surfaces from out-of-memory failure. Reconfiguration is attempted for the
  former; the latter can panic. These are materially different failure classes.

Two qualifications matter for NevroTick. First, the [event proxy][iced-proxy]
has a bounded `Sink` path with `MAX_SIZE = 100`, but direct `send`/`send_action`
paths bypass that backpressure; broadcast delivery can fail and be dropped
with a warning. It is inaccurate to describe all iced application messaging as
automatically bounded and lossless. Second, ordinary widget-tree child diffing
is positional; applications need appropriate stable identities when reordering
stateful items.

**Inference:** iced has the cleaner and more directly auditable Rust ownership
story in this bounded comparison. Fewer handwritten unsafe mechanisms in the
inspected areas do not establish safety of its full renderer/window/dependency
graph, nor that an application built with it automatically satisfies
NevroTick's input, authority or shutdown contracts.

### 4.2 Slint: broader responsibilities and more custom invariants

Slint separates its compiler, runtime, platform backends, renderers and language
bindings. A compiled UI language, multiple host languages and constrained
targets require machinery that iced does not attempt to provide in the same
way. Counting that extra machinery as waste would confuse product scope with
implementation quality.

The trade-off is that an auditor must understand more custom unsafe contracts:
`VRc`/`VWeak`/`VBox`, vtables and field offsets, copy-on-write `SharedVector`,
and intrusive property-dependency tracking. The [vtable implementation][slint-vtable]
documents layout, pinning and destruction requirements; the
[shared-vector implementation][slint-vector] contains alignment and bounds
regressions. The [core crate][slint-core] denies unsafe code by default with
scoped allowances, but also allows Clippy's missing-safety-documentation lint
globally. That weakens automated documentation enforcement; it does not imply
that all its unsafe operations lack safety explanations.

For intended generated component handles, the [public API][slint-api] checks
the originating thread before upgrading a weak handle. Event-loop upgrade
helpers skip work when the component has gone away. Weak captures are useful
for preventing component/callback reference cycles. These are useful design
features, but the generic contracts and asynchronous destruction paths below
deserve explicit disposition before relying on them in an admitted graph.

### 4.3 Unconfirmed Slint source findings to retain

The labels **SR-1** and **SR-2** are local to this report; they create no new
canonical product requirement or vulnerability identifier.

| Local finding | Observed contract and concern | Current disposition |
| --- | --- | --- |
| **SR-1 — generic weak-handle thread safety** | Public, safe, unsealed `StrongHandle` constrains `WeakInner` to `Clone + Default`. `Weak<T>` contains that associated value and supplies blanket unsafe `Send`/`Sync` implementations. Its safety explanation assumes the intended atomic `VWeak`, but the public bounds do not enforce that representation. An external safe implementation using `Rc` is not excluded by the stated bounds. Origin-thread checks on upgrade do not protect cloning or dropping the stored weak value on another thread. | Strong static API-contract concern in both reviewed release and development snapshots. No compiled reproducer, maintainer confirmation, exploitability assessment or observed NevroTick failure. Generated Slint handles using the intended representation are not thereby shown to trigger the issue. |
| **SR-2 — destruction of a thread-affine future** | `FutureRunner` stores a pinned boxed future without a `Send` bound, has manual unsafe `Send`/`Sync`, and participates in an `Arc`-based join-handle design. Polling checks its thread; a clear owner-thread final-destruction mechanism was not identified. Last-reference release, pending-future cancellation and waker teardown require examination to determine where a non-`Send` future's captures can be destroyed. | Lifecycle audit candidate, less conclusive than SR-1. No confirmed failing schedule or reproducer. A polling-thread check alone does not settle destructor affinity; setting an abort flag alone does not prove completion. |

Sources are the released [API][slint-api], its [public Rust re-export][slint-reexport]
and [future implementation][slint-future], cross-checked against the development
[API][slint-dev-api] and [future implementation][slint-dev-future]. The normative
reason for scrutiny is Rust's [`Send` contract][rust-send] and requirement that
safe callers cannot cause [undefined behavior][rust-ub] through an unsound safe
abstraction. This is a contract analysis, not proof that NevroTick exercises
the questionable generic implementation or destruction schedule.

M0-SL-01/02 must disposition these findings against the **exact candidate being
admitted**, including relevant fixes or upstream explanations. A possible
SR-1 remediation direction is to seal or appropriately constrain the
abstraction; no upstream patch was made here. If a confirmed, reachable issue
prevents acceptable delivery, use the existing OD-04/OD-13 blocker process.
The owner selection does not suppress that process.

## 5. Verification infrastructure and supply-chain observations

### Slint

Slint's inspected [CI][slint-ci] includes Miri for `vtable`,
`const-field-offset`, `i-slint-common` and selected core library tests, including
the `ffi,rtti` feature configuration. The Miri job runs only for the full-input
workflow path. Its configuration uses Tree Borrows, symbolic alignment checks
and disabled isolation. An actual successful [Miri job, 127213916246][slint-miri-run],
was inspected in run `42018190339` at commit
`8b87c803e4f953fb9750542cebd70dc2cf23c3ab`. This is execution evidence for that
job and commit, not a Miri pass for every test or the later reviewed head.

The [compiler fuzz target][slint-fuzz] exercises parsing, middle-end processing
and supported lower-level code generation paths. Its existence does not prove
a continuously running campaign, accumulated coverage or absence of crashes.
The [reusable build/test workflow][slint-build-test] and [test driver][slint-tests]
cover multiple language and integration paths; screenshot workflows add visual
checks. Locked dependency test lanes and separate dependency-update lanes are
a useful distinction. Some special configurations still have explicit limits:
selected property debug checks differ under Miri, and a giant-allocation vector
test is ignored there.

Slint's [safety documentation][slint-safety] concerns a defined safety-oriented
scope and qualification work. It does **not** certify the general Slint runtime,
every backend or NevroTick; the [standards-compliance scope][slint-safety-standards]
must be read before making such a claim.

### iced

The inspected released and development [test workflows][iced-tests] exercise
Linux, Windows and macOS across stable, beta and MSRV toolchains. The release
snapshot uses MSRV 1.88; the development snapshot uses 1.93. The workflow runs
workspace/default, ignored and all-feature tests, with the software test
backend selected. **Ignored tests are explicitly run in CI**; their annotation
alone is not evidence of neglect. The inspected development
[run 41347059782][iced-test-run] passed all nine matrix jobs at reviewed commit
`8ff48c42b0d5188223501ea1ccc506fd5beaf36a`.

Development tests include [user-interface behavior][iced-ui-tests], scrollable
widgets and [incremental Markdown][iced-markdown-tests]. The latter compares
incremental results with one-shot parsing over a deterministic 600-document
corpus and three chunking patterns. That is meaningful differential regression
testing, but it is not a coverage-guided fuzzing campaign. Development tests
must not be credited as already shipped in iced 0.14.0.

### Shared limits and an iced-specific audit caveat

Both inspected workflow sets use mutable action references, and neither
inspected head contained `.github/workflows/actions.lock`. This leaves room to
improve dependency integrity through full action commit pins or GitHub's
[workflow dependency locking][github-actions-lock]. Repository inspection alone
cannot establish organization-level permissions, external controls or an actual
compromise.

iced has an [audit workflow][iced-audit] on scheduled and change-triggered paths,
but it runs `cargo update` before auditing. That checks newly resolved
dependencies, not necessarily the unchanged lockfile graph used to build a
release. Frozen-graph and update-compatibility checks answer different
questions; a product gate should preserve both. No inference that Slint has no
security process follows from not finding an equivalent audit lane in the
sampled workflow set.

**Inference:** Slint has broader inspected verification infrastructure for its
custom machinery. iced has useful platform coverage and increasingly concrete
behavioral tests. Neither test-suite size nor a green workflow establishes
that every unsafe contract, dependency or downstream release is correct.

## 6. Accessibility, integration and licensing

Slint contains an implementation of
[AccessKit in its winit backend][slint-accesskit]. The inspected stock iced
release and development snapshots do not provide an equivalent completed
integration. On 5 November, iced [accessibility issue #552][iced-a11y-issue]
remained open and [PR #3111][iced-a11y-pr] remained open, draft and unmerged;
its documented work covered a limited Button/Text example and macOS VoiceOver
experimentation. Those states are dated observations, not a prediction that
accessibility will never arrive.

System76/COSMIC work is relevant ecosystem evidence, but its iced fork is not
identical to the stock `iced-rs/iced` release. Do not transfer fork-specific
capabilities or Windows qualification to upstream iced. Likewise, Slint's
bridge source is not a pass for our labels, focus order, keyboard navigation,
Narrator/UIA exposure, dialogs or error announcements.

Both frameworks principally draw their own widgets. Slint's Fluent appearance
does not turn those widgets into Win32 child controls with inherited native
control automation. Both support software rendering; iced's tiny-skia path and
0.14 reactive-rendering changes contradict the claim that iced necessarily
requires a constantly rendering GPU UI. See iced's [renderer selection][iced-renderer]
and [0.14 release][iced-014-release].

For NevroTick, preserve the existing integration boundaries: the UI runs as an
ordinary user, reuses checked models, and does not acquire the engine/broker/
helper's authority. The first migration is still fixture-only. D1 display/theme,
D2 production freshness/reconnect/admin results and D3 keyboard/Narrator/UIA
work remain **Deferred** to their existing re-entry gates; this report does not
move them into the present UI-M0 checkpoint.

Licensing remains an explicit admission problem in [OD-13](../nevrotick-owner-decisions.md#17-od-13--slint-as-the-default-ui-toolkit):
Slint's GPLv3/royalty-free/commercial routes and the selected Apache-2.0-only
winit dependency have not been cleared against NevroTick's current GPL-2.0-only
obligations. The iced pilot also selected Apache-2.0-only `winit 0.30.13` and
`unicode-linebreak 0.1.5`; iced's MIT framework license does not clear that entire
dependency graph either. No commercial agreement, relicensing, combined
distribution clearance or process-split exemption is inferred here. The
existing OPEN-1/13 and CHECK-20/26 owners decide the exact graph and route.

The Slint pilot also exposed the generated-code boundary: generated
`allow(unsafe_code)` conflicts with crate-level `forbid(unsafe_code)`.
M0-SL-02 must explicitly review that boundary; selecting Slint is not permission
to silently weaken workspace policy. The existing
[implementation skill](../../.agents/skills/implementation/implement-slint/SKILL.md)
and its references support implementation; this report retains the comparative
research and decision rather than turning volatile popularity figures into
coding instructions.

## 7. Development activity and community size

### 7.1 Snapshot and census results

All figures below were collected or calculated on **5 November 2026**. The
90-day window is **[7 August 2026 00:00 UTC, 5 November 2026 00:00 UTC)**;
the 365-day window is **[5 November 2025 00:00 UTC, 5 November 2026 00:00 UTC)**.
This excludes the incomplete research day from period counts.

| Measure | iced | Slint | What it describes |
| --- | ---: | ---: | --- |
| GitHub stars | 31,924 | 24,400 | Public interest/bookmark proxy |
| GitHub forks | 1,726 | 1,059 | Repository copies, including inactive copies |
| Non-merge, identified-non-bot commits, 365 days | 796 | 5,284 | Reachable default-branch history in the window |
| Git author identities, same 365-day set | 83 | 150 | Mailmap-aware author email identities, not verified distinct people |
| Non-merge, identified-non-bot commits, 90 days | 293 | 2,241 | Recent mainline activity |
| Git author identities, same 90-day set | 13 | 76 | Recent authorship breadth |
| Top author's share of 90-day commits | 91.5% | 28.0% | Authorship concentration, not review or ownership share |
| Top three authors' share, 90 days | 96.6% | 62.2% | Same concentration measure |
| Top author's share, 365 days | 80.4% | 28.7% | Concentration over a longer interval |
| Merged PRs, 90 days, including bots | 40 | 1,399 | Merged PRs across the repository, not only default-branch commits |
| Merged PRs, 90 days, identified bots removed | 40 | 1,359 | Contribution flow under the stated bot filter |
| Non-bot authors of those merged PRs | 14 | 62 | Distinct GitHub accounts, a different denominator from Git authors |
| Median creation-to-merge time, non-bot merged PRs | 10.6 hours | 10.8 hours | Successful merged PR cohort only |
| Open PRs at observation | 153 | 156 | Snapshot includes drafts and bots |
| Those open PRs created before 7 August | 107 | 40 | Age signal; approximately 70% versus 26% of the snapshot |
| Stable top-level GitHub releases in 365-day window | 1 | 8 | Excludes prereleases and separate subcrate publishing |

Primary inputs: [iced repository metadata][iced-meta],
[Slint repository metadata][slint-meta], the pinned Git histories in §2,
[iced merged-PR search][iced-merged-prs], [Slint merged-PR search][slint-merged-prs],
the dated [iced][iced-open-prs] and [Slint][slint-open-prs] open-PR queries, and
the projects' [iced][iced-releases] / [Slint][slint-releases] release records.
Live queries can change after this snapshot; the values above are dated research
results, not continuously refreshed counters.

In iced, the leading author accounts for **268 of 293** recent commits and
**640 of 796** annual commits. Slint's leading recent author accounts for
**627 of 2,241**; its leading annual author accounts for **1,515 of 5,284**.
The main contributor differs between Slint's two windows. The conclusion is
not that outside contributions to iced disappear: its merged-PR authorship is
broader than its mainline Git authorship, and authorship does not identify who
reviewed, suggested or helped produce a change.

For release cadence, iced's observed stable top-level release was **0.14.0,
7 December 2025**. Slint published **1.15.0 (4 February), 1.15.1 (12 February),
1.16.0 (16 April), 1.16.1 (23 April), 1.17.0 (24 June), 1.17.1 (7 July),
1.18.0 (16 September) and 1.18.1 (21 September)**, all in 2026. That is four
feature and four patch releases in this census. Fewer releases can reflect
larger batches and more API development; it is not proof of inactivity.

### 7.2 Method and limitations

Git statistics use the pinned development heads, reachable non-merge commits,
**committer timestamps** and mailmap-aware `%aN`/`%aE` author fields. Author
emails are case-folded for grouping. The identified-bot filter matches
`[bot]`, `dependabot`, `renovate`, `github.actions`, `slint.bot` and
`slint-ui-bot` in author names/emails. Unrecognized automation and work credited
to a human remain possible. Raw non-merge totals before that filter were
293/796 for iced and 2,290/5,454 for Slint, for 90/365 days respectively.

iced's annual history was rechecked from a complete, non-shallow clone. The
initial shallow-history annual result was discarded. Slint's retrieved
reachable history extended before the annual window. Reproduction must use
history complete through the start of each interval; a shallow clone can
silently undercount. For example, collect `git log <pinned-head> --no-merges
--format='%ct%x09%aN%x09%aE'`, then apply the timestamp, bot and identity rules
above. Do not use an unspecified recent-commit API page as the denominator.

The merged-PR census used paginated GitHub issue searches with `is:pr is:merged`
and inclusive date ranges corresponding to the complete UTC days. To avoid the
1,000-result search cap, Slint was split into **7 August–30 September (649)**
and **1 October–4 November (750)**, then deduplicated by PR identity. All 1,399
results were paginated; iced had 40. Forty Slint PRs were classified as bots.
PR author counts use non-bot GitHub accounts; elapsed merge times use
`created_at` to `merged_at` for that same cohort. The reported medians are
0.4412 days for iced and 0.4513 for Slint before conversion and rounding.

Merged-only latency has survivorship bias: it says little about submissions
that remain unmerged, first-response time or support availability. Old open
PRs can be drafts, intentionally deferred designs or changes needing revision;
age is not a count of neglected bugs. No representative Discord/chat member,
monthly user, production installation or paid-customer census was obtained.

Slint's monorepo covers more languages, compiler/tooling and embedded targets.
Commit splitting, direct commits, squashing and release processes differ.
Therefore **2,241 versus 293 is not a productivity, quality or optimization
ratio**. A supplemental Rust implementation-path check still showed activity
in both projects, but its unequal path sets are not used as a normalized
framework-quality metric. The stronger conclusion is that Slint's observed
activity involves more author identities and less concentrated authorship,
consistently across both windows.

### 7.3 Governance and ecosystem interpretation

iced's [maintainer FAQ][iced-faq] describes a deliberately personal,
maintainer-led project in which its creator writes or directly reviews every
line, keeps design authority and avoids feature timelines. Its
[contribution guide][iced-contributing] asks contributors to coordinate larger
work and warns that integration can take time. That can preserve coherence,
but it also concentrates final review capacity. The observed activity is
consistent with that published model; it does not establish the maintainer's
finances, available hours or future commitment.

Slint is developed by [SixtyFPS GmbH and its community][slint-about]. Its
named founders have GUI/tooling experience, and the observed contribution
pattern spans multiple active developers. The [1.16][slint-116] and
[1.17][slint-117] announcements identify NLnet-supported desktop work through
Slintify/LibrePCB. This provides evidence of organizational and funded
development capacity, but not a guarantee of company longevity, a support SLA
or a zero-risk vendor dependency.

The [awesome-iced directory][awesome-iced] records community widgets and
applications, including iced_aw, iced_audio, Halloy and Sniffnet. Slint's
[showcase][slint-showcase] records applications and embedded products. These
are useful discovery sources, not independently audited adoption statistics;
list entries and badges do not prove current application dependency versions.
COSMIC's fork must be identified explicitly when citing it. Stale marketing
contributor counters and old sponsor badges were excluded from the numerical
comparison.

**Inference:** iced has the larger visible Rust-community following, while
Slint currently has the broader measured active development base and less
concentrated maintenance model. That distinction matters more for NevroTick's
delivery planning than a single stars ranking.

## 8. Youth, maturity and modernization

The premise that iced is the new clean design while Slint is a much older
legacy codebase does not fit the observed chronology. The current GitHub iced
repository was created on **15 July 2019**, with code history reaching May
2019. Slint's repository was created on **4 May 2020**, and its
[1.0 announcement][slint-100] describes the project beginning in May 2020 as a
new Rust implementation. Repository creation is a useful date, not a complete
history of every precursor or imported component.

Both are still changing substantially:

| Framework | Concrete modernization evidence | Implication |
| --- | --- | --- |
| iced | The [0.14 release][iced-014-release] adds reactive rendering and work on headless/end-to-end testing, IME and hot reload. [Text selection #3497][iced-selection] merged on 2 October 2026; [FrameClock #3527][iced-frameclock] merged on 31 October. | Lower stable-release frequency coexists with meaningful active development. Development-head capabilities must be separated from the stable release. |
| Slint | [1.16][slint-116] and [1.17][slint-117] include desktop interaction and integration work. [1.18][slint-118] describes leaner generated code, text-performance work, Flexbox, experimental Vello and accessibility/input improvements. [Code-generation PR #14736][slint-codegen-pr] merged on 4 November. | A stable public API can coexist with compiler, renderer and runtime optimization. Upstream performance claims are not NevroTick measurements. |

Slint's [contribution policy][slint-contributing] and 1.0 direction emphasize
compatibility within 1.x. That reduces some downstream migration pressure but
does not promise zero exceptions: its [changelog][slint-changelog] records a
1.16.1 `non_exhaustive` adjustment to key-event/modifier types with compatibility
implications. iced's pre-1.0 evolution gives maintainers more design freedom,
while applications must budget for API migration. Neither policy proves that
internals are optimal or burdened by unfixable legacy.

The engineering question is whether each project improves important internals
while preserving the contracts our application depends on. The inspected
evidence supports active improvement in both. It does not support calling
Slint stagnant or iced immature in every dimension merely from version numbers.

## 9. Consequences and retained follow-through

The decision is to **proceed with Slint**, using the current task chain rather
than starting another general toolkit competition. Implementation still follows
the [development plan](../plans/development-plan.md); its Core/proof-first
priority and existing execution holds remain authoritative.

| Existing owner or gate | Concrete follow-through from this report |
| --- | --- |
| **S0.5 evidence custody** | Retain both exact candidate/build/observer identities and private raw-receipt manifests. Read the full records alongside the rounded comparison; preserve separate cohorts, focus/layout differences and qualified scope. |
| **M0-SL-01; OPEN-1/13; CHECK-20/26** | Freeze and admit the actual Slint/backend/renderer/dependency/license graph. Treat source-reviewed versions as evidence inputs, not automatic approval to adopt them. |
| **M0-SL-02** | Review generated unsafe/lint policy, callback ownership, UI-thread affinity and teardown. Disposition SR-1/SR-2 against the admitted version and reachable application paths, preserving unconfirmed status until supported by stronger evidence. |
| **M0-SL-03/04** | Reuse the checked fixture model, validation and authority boundaries. Implement and verify only after existing predecessors and the UI slice's activation conditions are met. |
| **M0-SL-05; CHECK-21/22** | Qualify the exact Windows candidate and realistic lifecycle/resource workload at the existing gate. Old Win32 passes and private idle results do not transfer automatically. |
| **D1/D2/D3 re-entry owners** | Restore display/theme, production state and full keyboard/Narrator/UIA checks at their specified gates. Slint's AccessKit implementation is a starting capability, not completed product evidence. |
| **OD-04/OD-13** | Revisit the toolkit only on an owner instruction or concrete licensing, target, accessibility, security or resource evidence that prevents acceptable selected delivery. No automatic fallback or blanket exception follows. |

This record closes the comparative research as an input to the existing owner
decision. It does not close migration, dependency admission, UI-M0, full S0.5,
native memory qualification or release readiness. Future agents should consult
the canonical owners for current status and refresh dated upstream facts only
when they materially affect a new decision.

## 10. Provenance and maintenance

This report consolidates the 5 November conversation, retained S0.5 evidence,
local inspection of the pinned upstream Git snapshots, primary project
documentation, GitHub metadata/search/release records and selected CI job
results. The owner's decision and initial iced summary are attributed to
David; the complete experiment records are attributed to their S0.5 execution
and review chain. The comparative analysis is an assistant-authored engineering
assessment. This report incorporated the iced provenance update that arrived
on `main` during drafting rather than preserving the initial missing-evidence
assumption.

Source links below identify the inspected files or primary records. Pinned
source links are immutable; project websites, open issues, PR searches and
metadata endpoints are mutable. No separate frozen raw GitHub-search dataset
is committed with this one-document report. Its aggregate statistics, exact
windows, filters and pagination counts are retained for future interpretation
and reproduction, not presented as a signed or independently audited census.

Corrections should amend this document with their date and basis, update OD-13
only if the owner decision changes, and place new execution receipts in S0.5/
UI-M0. Do not overwrite historical experiment values with a later configuration
or silently replace an unconfirmed source finding with a claimed pass.

[iced-stable]: https://github.com/iced-rs/iced/tree/3997291f318a8bc06fa522f5579836fb3feb94df
[iced-dev]: https://github.com/iced-rs/iced/tree/8ff48c42b0d5188223501ea1ccc506fd5beaf36a
[slint-stable]: https://github.com/slint-ui/slint/tree/372cf0ee5577c3dfec309a45e7b778ba4e81b734
[slint-dev]: https://github.com/slint-ui/slint/tree/d7fbf11316eb10ad58302d66ce5de379b1867a93
[iced-runtime]: https://github.com/iced-rs/iced/blob/3997291f318a8bc06fa522f5579836fb3feb94df/runtime/src/lib.rs
[iced-futures]: https://github.com/iced-rs/iced/blob/3997291f318a8bc06fa522f5579836fb3feb94df/futures/src/lib.rs
[iced-tree]: https://github.com/iced-rs/iced/blob/3997291f318a8bc06fa522f5579836fb3feb94df/core/src/widget/tree.rs
[iced-task]: https://github.com/iced-rs/iced/blob/3997291f318a8bc06fa522f5579836fb3feb94df/runtime/src/task.rs
[iced-subscriptions]: https://github.com/iced-rs/iced/blob/3997291f318a8bc06fa522f5579836fb3feb94df/futures/src/subscription/tracker.rs
[iced-clipboard]: https://github.com/iced-rs/iced/blob/3997291f318a8bc06fa522f5579836fb3feb94df/winit/src/clipboard.rs
[iced-cargo]: https://github.com/iced-rs/iced/blob/3997291f318a8bc06fa522f5579836fb3feb94df/Cargo.toml
[iced-winit]: https://github.com/iced-rs/iced/blob/3997291f318a8bc06fa522f5579836fb3feb94df/winit/src/lib.rs
[iced-proxy]: https://github.com/iced-rs/iced/blob/3997291f318a8bc06fa522f5579836fb3feb94df/winit/src/proxy.rs
[slint-vtable]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/helper_crates/vtable/src/lib.rs
[slint-vector]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/core/sharedvector.rs
[slint-core]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/core/lib.rs
[slint-api]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/core/api.rs
[slint-reexport]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/slint/lib.rs
[slint-future]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/core/future.rs
[slint-dev-api]: https://github.com/slint-ui/slint/blob/d7fbf11316eb10ad58302d66ce5de379b1867a93/internal/core/api.rs
[slint-dev-future]: https://github.com/slint-ui/slint/blob/d7fbf11316eb10ad58302d66ce5de379b1867a93/internal/core/future.rs
[rust-send]: https://doc.rust-lang.org/core/marker/trait.Send.html
[rust-ub]: https://doc.rust-lang.org/reference/behavior-considered-undefined.html
[slint-ci]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/.github/workflows/ci.yaml
[slint-miri-run]: https://github.com/slint-ui/slint/actions/runs/42018190339/job/127213916246
[slint-fuzz]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/compiler/fuzz/fuzz_targets/compiler_fuzzing.rs
[slint-build-test]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/.github/workflows/build_and_test_reusable.yaml
[slint-tests]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/tests/run_tests.sh
[slint-safety]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/docs/safety/src/content/docs/index.mdx
[slint-safety-standards]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/docs/safety/src/content/docs/qualification-plan/standards-compliance.mdx
[iced-tests]: https://github.com/iced-rs/iced/blob/8ff48c42b0d5188223501ea1ccc506fd5beaf36a/.github/workflows/test.yml
[iced-test-run]: https://github.com/iced-rs/iced/actions/runs/41347059782
[iced-ui-tests]: https://github.com/iced-rs/iced/blob/8ff48c42b0d5188223501ea1ccc506fd5beaf36a/runtime/tests/user_interface.rs
[iced-markdown-tests]: https://github.com/iced-rs/iced/blob/8ff48c42b0d5188223501ea1ccc506fd5beaf36a/widget/tests/markdown.rs
[iced-audit]: https://github.com/iced-rs/iced/blob/8ff48c42b0d5188223501ea1ccc506fd5beaf36a/.github/workflows/audit.yml
[github-actions-lock]: https://docs.github.com/en/actions/concepts/security/workflow-dependency-lockfiles
[slint-accesskit]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/backends/winit/accesskit.rs
[iced-a11y-issue]: https://github.com/iced-rs/iced/issues/552
[iced-a11y-pr]: https://github.com/iced-rs/iced/pull/3111
[iced-renderer]: https://github.com/iced-rs/iced/blob/3997291f318a8bc06fa522f5579836fb3feb94df/renderer/src/lib.rs
[iced-014-release]: https://github.com/iced-rs/iced/releases/tag/0.14.0
[iced-meta]: https://api.github.com/repos/iced-rs/iced
[slint-meta]: https://api.github.com/repos/slint-ui/slint
[iced-merged-prs]: https://github.com/iced-rs/iced/pulls?q=is%3Apr+is%3Amerged+merged%3A2026-08-07..2026-11-04
[slint-merged-prs]: https://github.com/slint-ui/slint/pulls?q=is%3Apr+is%3Amerged+merged%3A2026-08-07..2026-11-04
[iced-open-prs]: https://github.com/iced-rs/iced/pulls?q=is%3Apr+is%3Aopen+created%3A%3C2026-08-07
[slint-open-prs]: https://github.com/slint-ui/slint/pulls?q=is%3Apr+is%3Aopen+created%3A%3C2026-08-07
[iced-releases]: https://github.com/iced-rs/iced/releases
[slint-releases]: https://github.com/slint-ui/slint/releases
[iced-faq]: https://book.iced.rs/faq.html
[iced-contributing]: https://github.com/iced-rs/iced/blob/8ff48c42b0d5188223501ea1ccc506fd5beaf36a/CONTRIBUTING.md
[slint-about]: https://slint.dev/about-us
[slint-116]: https://slint.dev/blog/slint-1.16-released
[slint-117]: https://slint.dev/blog/slint-1.17-released
[slint-118]: https://slint.dev/blog/slint-1.18-released
[awesome-iced]: https://github.com/iced-rs/awesome-iced
[slint-showcase]: https://slint.dev/showcase
[slint-100]: https://slint.dev/blog/announcing-slint-1.0
[iced-selection]: https://github.com/iced-rs/iced/pull/3497
[iced-frameclock]: https://github.com/iced-rs/iced/pull/3527
[slint-codegen-pr]: https://github.com/slint-ui/slint/pull/14736
[slint-contributing]: https://github.com/slint-ui/slint/blob/d7fbf11316eb10ad58302d66ce5de379b1867a93/CONTRIBUTING.md
[slint-changelog]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/CHANGELOG.md
