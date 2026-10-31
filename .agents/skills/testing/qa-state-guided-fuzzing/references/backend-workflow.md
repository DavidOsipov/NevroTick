# Select and qualify semantic feedback

Use the existing task's declared runner and fresh owned outputs. Inspect current
installation/admission and execution receipts before selecting a backend; research
or an example in this reference establishes no installed or qualified lane.

## Exercised fixture route

The current bounded lane uses the standalone
[state-guidance supervisor](../../../../../tools/state-guidance/run.py) and
[its policy](../../../../../tools/state-guidance/policy.json), with the existing
[S1.1e follow-up](../../../../../tasks/s1.1-deterministic-harness.md#semantic-feedback-follow-up-31-october-2026)
owning actual candidate results. It uses Rust 1.99.1 and a custom command mutation
recipe. The selected [engine manifest](../../../../../tools/state-guidance/engine/Cargo.toml)
disables default features on both 0.16.1 crates: `libafl` enables only `derive`
and `serdeany_autoreg`; `libafl_bolts` retains `std`, `derive` and
`serdeany_autoreg`. This is a comparison of two feedback modes of one binary,
not a cargo-fuzz/AFL++ engine benchmark.

The first graph enabled optional `libafl/std`, which pulled in `bincode 2.0.1`.
Its strict advisory audit failed on the unmaintained warning
`RUSTSEC-2025-0141`; keep that receipt as historical evidence. A feature-only
trim removed `libafl/std` without patching upstream code or ignoring warnings.
The current [lockfile](../../../../../tools/state-guidance/engine/Cargo.lock)
contains 130 packages; the selected GNU host graph contains 101. A fresh private
RustSec advisory-only audit of that exact lock reported zero vulnerabilities and
zero warnings with no ignored advisory IDs. Yank status was excluded from that
audit's scope. Preserve the exact-lock advisory receipt required by the supervisor;
this result is no registry/index, native Windows or complete security/release
qualification.

From the checkout root:

```sh
python3 -B tools/state-guidance/run.py
python3 -B tools/state-guidance/run.py --controls --output-dir /absolute/existing-parent/new-controls
python3 -B tools/state-guidance/run.py --compare --output-dir /absolute/existing-parent/new-comparison
```

The output directory must not exist and must be outside the checkout/install
inputs. The supervisor creates private work, clears the environment and uses the
existing offline namespace/resource runner. The private schema-1 pointer at
`~/.local/share/nevrotick/state-guidance-installation.json` locates the installed
payload; the checkout policy pins its manifest, exact source/recipe/runtime bytes
and completed control receipt. Missing/drifted installation is incomplete; do not
fall back to raw host execution. Fresh controls do not automatically change policy.
Intentional source/tool updates need a rebuilt frozen candidate and reviewed
re-admission, preserving earlier receipts.

Version-1 inputs contain 1–32 two-byte commands after the version byte, at most
65 bytes. The model checks actual public association state after each action;
pending identity, next identity and full context are observable, while hidden
pending token/timing fields are challenged through completion facts and later
actions. Do not claim immediate inspection of every internal field.
feedback intentionally coarsens it to outcome/pending/generation-parity features.
Only the three real pure SUT crates receive LLVM PC guards. A stable owned
`current-input.trace` is written before every campaign target call; inspect it
after process abort as well as typed oracle-failure witnesses. This is no
power-loss durability or sanitizer-detection claim.

The standard pilot uses three declared RNG seeds and 1,024 evaluations per healthy arm.
Its current input registry contains four lifecycle traces; RNG seeds and initial
input seeds are different dimensions.
Separate seeded-fault searches stop at detection or that same maximum budget.
Record actual calls to detection and censoring. The initial pilot increased
semantic-feature exposure but detected its single fault earlier for only one of
three seeds. Preserve that mixed result. Different trace lengths caused different
completed-action counts; no equal-work/CPU or general detection advantage follows.

`Arm` in an action trace means create a pending request. CPU architecture and
`target-cpu` are separate build properties. Keep ISA changes separate from a
corpus/feedback experiment so they do not confound its comparison.

When improving detection, change one factor at a time. A small requirement-derived
corpus can preserve useful lifecycle prefixes; admit identical bytes and lexical
ordering in both feedback modes, and count initial evaluations inside the budget.
Exclude known fault-triggering witnesses from that starting corpus. Check seeds
against healthy and private fault controls, record the actually admitted subset,
and separate supplied initial features from novelty found by later mutations.
Preserve paired RNG trials, misses censored at the budget and completed-action
counts. One injected fault establishes sensitivity only for its reached behavior.

An omitted early guard can expose a later rejection as well as a successful
event. Qualify diagnosis against the actual guard order, exact observed step and
independent oracle; replay the same bytes on both binaries. Report acceptance and
error-precedence failures separately. If a diagnosis filter was too narrow,
preserve its failed receipt and document the independently reviewed correction;
do not silently change the endpoint or count later acceptance that was not reached.

## Compare fault classes, oracle objectives and mutations

Keep strict contract comparison as the default. An explicitly selected effects
lane may continue a returned-category difference only when checked actual state
and completion effects still agree with the independent model. Preserve exact
completion arithmetic and identity checks. Separate forbidden acceptance,
missing/corrupt completion and lifecycle-state divergence from error-precedence
diagnostics; neither setup failure nor a build failure is a detected defect.
Disclose fields without public observation and challenge them through subsequent
actions instead of claiming complete internal-state visibility.

The standalone engine supports explicit experimental selections:
`replay ABS_TRACE strict|effects` and
`campaign MODE RNG BUDGET ABS_SEEDS ABS_OUTPUT baseline|lifecycle strict|effects`.
Omitting them preserves `baseline` mutations and the strict oracle. Execute through
an admitted offline supervisor with current controls and sealed input bytes;
these CLI examples authorize no unguarded host invocation. The standard supervisor
continues to exercise its default lane; a separately frozen study recipe owns
experimental selectors and additional private fault payload admission.

For each private fault, retain the exact single source change, successful build,
legal neighboring control and deterministic intended witness through the public
target API. Never modify canonical production guards. Common training inputs must
pass every selected fault and oracle before search; keep known triggering inputs
in qualification only. A previously useful lifecycle seed may cease to be neutral
when a new fault is added.

Freeze the mutation recipe, probability, input limits and paired RNG blocks before
comparison. The experimental lifecycle operator preserves a bounded prefix and
appends request succession, obsolete cancellation and completion actions; its
selection is 1/4 per mutation. It uses no oracle queries. Hold semantic feedback
and initial input bytes fixed when comparing mutations, and compare strategies
within each oracle/fault. Report per-class misses and paired outcomes instead of
hiding a regression in a pooled score. Preserve callback budgets, startup calls,
censoring and actual action counts; equal callback counts do not imply equal work.

Retain a bounded first nonfatal discrepancy input and its step/category details
independently of coverage admission, before it can be overwritten by later calls.
Keep the first fatal witness separately. Replay identical bytes on faulty and
healthy candidates from fresh state and retain typed diagnoses. Record qualification
and replay calls separately from the campaign budget. A diagnostic trace which
later fails is still an effect failure at that reached step; a later hypothetical
acceptance is not evidence. Successful-trace action totals omit the failing prefix
unless a separate attempted-action counter actually measures it.

Use a declared stopping point. Multiple hand-selected faults establish broader
bounded sensitivity, not a representative mutation score or general superiority.
Keep a specialized operator opt-in when it helps one class but harms others; add
the useful witnesses to appropriate regression coverage, then resume product work.

## Rebuild and re-admit changed inputs

The current task coordinator owns this operation within the authorized change;
its independent verification owner reviews the exact candidate and receipts.
This is candidate review, not an extra human approval round for routine work.
There is no automatic cross-machine installer or policy-repin command.

1. Preserve the previous installation and controls. Start a fresh private source
   snapshot using the source inventory in the current installation manifest:
   root workspace inheritance, the three pure crates, wrapper, engine/harness and
   the engine lockfile. Check that source bytes remain stable while copied.
2. For explicitly authorized acquisition only, use a private Cargo home to fetch
   the locked graph and `cargo vendor --locked --versioned-dirs`. Verify package
   checksums and every vendor file; do not refresh shared stores. The current
   host's acquisition/build recipes and commands are retained with the
   [task's private installation evidence](../../../../../tasks/s1.1-deterministic-harness.md#follow-up-execution-and-limits).
   Inspect those recipes before reuse; their paths/configuration are host-specific.
3. Create the snapshot's Cargo source replacement to that vendor directory.
   Through `scripts/analysis-runtime.py`, run `cargo build --release --frozen
   --target x86_64-unknown-linux-gnu --manifest-path <snapshot-engine-Cargo.toml>`.
   Set the explicit admitted `RUSTC`, `RUSTDOC`, frozen `RUSTC_WRAPPER` and owned
   `STATE_GUIDANCE_WRAPPER_LOG`; bind source/vendor/toolchain readonly and use the
   engine directory as cwd. Retain source/compiler inventories before and after.
   The wrapper bounds linker threads; reduce jobs within twelve when needed,
   preserving the unchanged process allowance.
4. Keep instrumented build/engine-test targets separate from unwrapped harness
   unit/Clippy targets. Run the independent controls, check actual wrapper scope,
   and retain declared private sensitivity variants separately from canonical
   sources. Never suppress compiler failures to call a control behavioral RED.
5. Install only owned artifacts by atomic replacement in a new private prefix.
   Record manifest/source/runtime hashes and a private schema-1 pointer, then
   deliberately update checkout admission bindings. Run fresh supervised controls;
   pin that completed receipt for the same manifest/recipe before comparison.
   Retain old results and report advisory/platform gaps separately.

## Keep the backend choice proportional

The existing cargo-fuzz/libFuzzer lane can already accept structured action traces
and comparison-guided inputs. First identify the actual search gap. AFL++ IJON
and LibAFL are overlapping candidates for richer semantic feedback; a concrete
custom-observer need can justify LibAFL, while selected IJON annotations may fit a
simpler existing AFL++ target. Ordinary generation and state models alone do not
require either engine. Keep the tool graph separate from the shipping graph.

| Backend | Admission detail that changes the claim |
| --- | --- |
| AFL++ IJON | Pin the Rust binding, bundled engine, compiler and runtime together; the binding need not bundle the latest standalone AFL++. Compile and reach the exact selected annotation API. |
| LibAFL | Pin crate/features and executor. Its customizable framework supplies no automatic state model or actual-work observation. Platform support depends on the selected executor; Unix forkserver support is not native Windows evidence. |

For LibAFL 0.16.1, `TimeFeedback` records runtime metadata and never independently
retains an input for being slower. Use semantic novelty or explicitly measured
counter feedback. `MaxMapFeedback` configuration can depend on the SIMD feature;
qualify representation and precision before interpreting wide counter maxima.
For either backend, bucketed coverage counts are not exact work counters.

For AFL++ v5.03c, distinguish semantic novelty (`SET`), virtual context (`STATE`
or `CTX`), comparison progress and selected extrema (`MAX`/`MIN`). Inspect the
pinned runtime: `STATE` uses XOR updates rather than ordinary assignment, so
repeated equal updates can cancel. Balance/reset feedback context between
independent traces. Current distance helpers differ from the older paper's
distance semantics. Their bounds and parameter types need version-specific checks.
Do not copy macro examples into Rust without a compiling and reached control.

The IJON maximum-value area is excluded from ordinary `afl-showmap` tuple output.
Qualify that channel through the selected version's retained maxima/state and
independent replay; a normal coverage dump alone proves no MAX feedback.

## Qualify the comparison before discovery

- Keep the target, driver, independent oracle and input format identical between
  arms. Where possible change only feedback selection. If different engines,
  instrumenters or mutation policies are compared, retain those confounders and
  make the narrower observation the experiment actually supports.
- Use a healthy trace and independently specified distinguishable target states.
  Demonstrate that correct feedback distinguishes them and that omitted/wrong
  feedback or a stale per-input context invalidates that claim. A feedback control
  does not replace a deliberate oracle/guard fault detected at the actual consumer.
- Qualify ordinary coverage against a reached code-path control. Record the exact
  instrumented source scope; no action-type bitmap, callback count or source-work
  reservation may be relabelled LLVM edge coverage.
- Keep actual target executions separate from engine iterations, generated bytes,
  rejected traces and corpus size. Repeat equivalent bounded trials when claiming
  an advantage; a single seeded witness is evidence only for that witness.
- Bind imported corpus bytes to format/version, target/harness, build/features,
  input limits and reset contract. Replay through the destination oracle before
  trusting an import. Save crash, semantic-state and cost witnesses separately;
  shrinking must preserve the selected failure or interestingness property.

Native authority, production NTS authentication, concurrency schedules and
uninstrumented hardware performance retain separate qualification. Use controlled
valid session/fixture inputs to reach deep crypto-dependent states; state guidance
does not solve authentication or justify patched production guards.

Primary semantics: [AFL++ v5.03c IJON](https://github.com/AFLplusplus/AFLplusplus/blob/v5.03c/docs/IJON.md),
[IJON runtime](https://github.com/AFLplusplus/AFLplusplus/blob/v5.03c/instrumentation/afl-compiler-rt.o.c),
[LibAFL 0.16.1 map feedback](https://github.com/AFLplusplus/LibAFL/blob/0.16.1/crates/libafl/src/feedbacks/map.rs),
[LibAFL time feedback](https://github.com/AFLplusplus/LibAFL/blob/0.16.1/crates/libafl/src/feedbacks/mod.rs),
[Rust structure-aware fuzzing](https://rust-fuzz.github.io/book/cargo-fuzz/structure-aware-fuzzing.html).
