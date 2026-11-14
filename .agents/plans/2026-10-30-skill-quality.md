# Skill implementation and evaluation improvements

User authorization: implement the recommendations from the skill-quality review
of 30 October 2026. This is instruction/fixture work, with no product repair,
dependency admission, native effects, staging or publication. Existing S1.2 work
belongs to its other owner. Initial staged diff SHA-256:
`b7fc8da84d1f5d4efa3166a923b4e5f350e1640ca230e0f073308fe71cc3f84f`.

Canonical contracts remain architecture §§6, 7 and 20 and the existing development
plan. CHECK-6/7/19/20/25 motivate these procedures; their application obligations
are not completed by this task. The local acceptance IDs below cover this change.

| ID | Deliverable / independent expected result | Owner / environment |
| --- | --- | --- |
| SKQ-1 | Numerical method and real upstream caller-path recipe; preserve task-specific policy and separate exact, floating and physical claims | Implementation writer; read-only source and portable math exercise |
| SKQ-2 | Failure-atomic state-update example with current-context and external-effect limits | Coordinator; isolated host Rust exercise |
| SKQ-3 | Exact-input/harness perturbation procedure, domain-specific mutation proof, aggregate campaign worksheet | Testing writer; independent synthetic review/calculation |
| SKQ-4 | Discoverable original `qa-skill-evaluations`, reusable fixture and independent outcome grader; distinguish exercise success from measured improvement | Coordinator; source review, host Rust, Python standard library |
| SKQ-5 | Inventory/links/checks, fresh behavioral exercises, no index mutation or product/native claim | Coordinator reconciles actual integrated instruction candidate |

One writer per path. Implementation writer owns numerical/Chrony references and
their Rust/numerical routing. Testing writer owns harness/mutation/budget references
and their seven QA leaves. Coordinator owns contract example, new evaluation leaf,
inventories/indexes/source notes and this record. Fresh evaluators receive bounded
raw tasks without desired diagnoses; grader ownership remains separate.

Verification plan: skill-pack checker; checker regression tests if its covered
inventory contracts change; new grader controls and independent task exercise;
fresh numerical/test-procedure reviews; whitespace and final source/index custody.
At most three fresh task exercises, with one affected rerun per corrected defect;
owned temporary output directories, no product builds or package installations.
Grader controls allow at most six invocations: original fixture twice if a runner
correction requires rebinding, repaired fixture, reject-all variant, invalid Rust
and unavailable compiler. At most five named checks run per compiled candidate;
setup controls execute none. Two additional CLI precondition controls may test
missing input/existing output without launching a compiler. The repair worker
has at most six compile/test invocations; numerical review uses a bounded
125-case synthetic family; the test-record review uses at most two calculations.
Missing environments or unsuccessful exercises stay visible. Full cross-model
or statistical skill-effectiveness comparison is outside this bounded task.

## Results

SKQ-1 through SKQ-5 completed for the instruction/fixture scope. Added the original
`qa-skill-evaluations` leaf and conditional numerical, harness, mutation and budget
procedures; improved actual upstream caller extraction and failure-atomic state
updates. Existing routes select the new detail only when relevant. The root index
now routes through current manifests/tasks instead of describing only UI-M0.
No product source, Cargo graph, toolchain, imported skill inventory or policy
threshold changed. No staging, commit or publication performed.

The 27 changed skill files are bound in
`/tmp/nevrotick-skill-evals-20261030-tv5kzzjm/skill-candidate.json`, SHA-256
`c886b912a11e327d3cb8dfa0016b54f98858f698b45c7576ffdb550d09a20489`.
This record is excluded from that map to avoid a self-referential digest.
Raw exercise artifacts under that temporary directory may be removed by normal
host cleanup; the outcomes and limitations below are retained here. Other
concurrent task edits were outside this writable scope.

### Structural and source checks

- `python3 -B .agents/skills/scripts/check_skills.py`: PASS, **60 skills**, metadata,
  UI prompts, local links/anchors and inventories. No checker implementation change.
- `python3 -B .agents/skills/scripts/test_check_skills.py -v`: **25 tests passed**.
- Skill-creator validation passed for the new leaf; writers also validated their
  changed leaves. Integrated `git diff --check` and whitespace checks of all 27
  changed/new skill files passed.
- Implementation writer checked six independent exact coefficient cases, three
  rounded controls and 49 separately rounded operations in the documented witness.
  An initial exploratory host reduction did not reproduce the specified order;
  the operation-level model resolved it. No Rust numerical execution is claimed.

### Three fresh-context exercises

1. **Numerical proposal review:** a fresh reviewer used an independently authored
   Fraction/operation model, four rounding controls and 125 bounded divisor
   triples. It derived a new admitted constant-series witness with divisors
   `(0.25, 0.5, 2)`, distinguished exact truth from rounded execution, retained
   healthy zero/linear controls and a nonconstant `4/3` output, and correctly
   required unspecified tolerance/output policy before implementation. Verdict
   `revise` concerns the supplied faulty proposal; it is successful detection in
   this skill exercise. No product Rust run or whole-domain error bound follows.
2. **Rust repair:** a fresh agent received only the copied task/candidate and
   applicable implementation guidance, with no diagnosis, grader or sibling
   outputs. It retained original bytes, added six tests and moved validation ahead
   of eviction. Four compiler/test invocations out of six allowed: optimized host
   Rust 1.99.1 / edition 2021 compiled both candidates; RED executed **4 passing,
   2 failing** tests (exit 101), GREEN **6 passing** (exit 0), with identical test
   bytes. The coordinator inspected the actual diff/tests and separately graded
   the repaired source using Rust 1.99.1 / edition 2024: **5/5 passed**. All 201
   allowed input integers had healthy controls in the agent's own tests; this
   remains a small synthetic state-update exercise, not product qualification.
3. **Testing-record review:** a fresh reviewer identified the substituted empty
   malformed input, separated the raw-helper non-equivalent bound mutation from
   the checked-caller equivalent predicate mutation, and computed **415 planned /
   261 actual** candidate calls. It recognized that arithmetic headroom 154 does
   not authorize more than the remaining 80-call invocation, and that a compiler
   failure may already consume that slot. One read-only Python calculation checked
   2,401 coordinate tuples and the accounting. No Rust mutant or parser was run.

The third reviewer also inspected the new evaluation workflow/runner. It found
an overstatement about forged success output: libtest reporting shares a process
with the candidate. Corrected the documentation to require code review for test
bypasses, early termination and report manipulation, and to state that the runner
does not authenticate hostile output. The reviewer confirmed closure by inspecting
the corrected wording. No spoofing experiment was executed or claimed.

### Grader controls and consumed work

The coordinator executed six allowed grader invocations, plus two CLI setup
controls. The first original-fixture run preceded a CLI error-classification fix;
its result remains separate. The final runner SHA-256 is
`5a79f6fe1cc8c1aa5ee180b66d8a5c479db10d7208126f0dfa9c3e786036a71c`.

| Control | Actual result |
| --- | --- |
| Original fixture, initial and final runner | Each compiled, discovered/executed five tests: 4 pass / 1 intended state-preservation failure; test exit 101, grader exit 1 |
| Reject-all variant | Compiled, five executed tests fail; healthy controls prevent rejection-only GREEN; grader exit 1 |
| Agent's repaired fixture | Compiled, five executed tests pass; grader exit 0 |
| Invalid Rust | Compile failure, no tests executed, `incomplete`, grader exit 2 |
| Missing compiler | Setup failure, no tests executed, `incomplete`, grader exit 2 |
| Missing input and existing output directory | Both rejected before compilation with CLI exit 2; existing output preserved |

Thus grader controls executed **20 tests across four runs** (13 pass / 7 expected
failures); the separate repair agent executed **12 tests across RED/GREEN** (10 pass
/ 2 expected failures). These counts do not multiply into product tests. Candidate
calls inside tests, math-operation counts and test counts are distinct units.

Final staged diff SHA-256 remained
`b7fc8da84d1f5d4efa3166a923b4e5f350e1640ca230e0f073308fe71cc3f84f`.
The review examples are related to published teaching cases; fresh context and a
changed numeric witness do not establish blind generalization. No comparable old
skill baseline, repeated-model trial or statistical improvement estimate was run.
These exercises establish useful execution of the changed guidance on selected
cases. Future substantive skill changes should retain these controls and add an
independent case for the changed decision; application/native/release gates remain
with their existing owners.

## TokenEconomizer workflow adaptation — 2 November 2026

User requested critical assessment and implementation of the attached proposal.
This extension reuses the skill-quality chain. Scope is repository instructions
and an original offline observer; no product source, Cargo graph, plugin install,
global configuration, automatic memory, tool pruning or publication. Main
development remains with its existing owner. Read-only starting snapshot:
`/tmp/nevrotick-workflow-observer-baseline-20261102.json`, HEAD
`82fc967fdb96333ea4bcc479f9b494c971129dce`, 46 pre-existing dirty/untracked files.

The assessment supports accounting for accepted work, per-worker repeated-failure
advice, fact applicability/invalidation and shadow comparison. Most underlying
retry/checkpoint advice already existed. Upstream source at
`7bbad83b5215ee46c75f89370efda820590298af` confirms three concrete integration
limits: the parser coerces string `"false"` to true; verification recognition
misses our Python wrappers and its no-check success proxy is unsuitable for
acceptance; the controller excludes subagents. Shadow still has observer state
and possible model costs. Current Codex pruning is narrow and protects tracked
configuration. These support borrowing bounded ideas rather than installing
defaults. Published benchmark benefits are not a NevroTick savings forecast.
Primary sources and the scoped decision live in
[workflow observation](../skills/testing/references/workflow-observation.md).

| ID | Independent expected result | Owner / environment |
| --- | --- | --- |
| SKQ-OBS-1 | Confirm consequential upstream claims against pinned code and healthy/malformed controls; retain actual scope | Source reviewer; read-only public source and isolated Python controls |
| SKQ-OBS-2 | Offline observer accounts for every assigned outcome and partial costs, attributes worker loops, uses pinned outer receipts and never grants qualification | Helper writer; bounded local JSON fixtures and Python CLI |
| SKQ-OBS-3 | Existing evaluation/triage/checkpoint skills route to one reference; facts retain applicability, contradiction and invalidation; no automatic memory/policy change | Coordinator; skill/link review |
| SKQ-OBS-4 | Meaningful healthy/fault controls, independent integrated review, package/whitespace checks and preserved unrelated work/staging | Coordinator and independent evaluator; no product/native campaign |

Controls cover repeated/unrepeated failures, worker separation/new evidence,
expected RED/mutation/rejection cases, enumeration versus qualification,
incomplete/missing/drifted receipts, strict booleans and incomplete accounting.
The helper analyzes explicit sanitized ledgers, not raw sessions; supplied labels
and visibility need review. Actual usage for current agents is unavailable here,
so no real-project cost baseline, intervention improvement or savings is claimed.

Pinned-source assessment executed **12 narrow extracted-function observations**:
seven judge-parser values, four command-recognition cases and one no-check
success-proxy case. Healthy JSON false/true stayed false/true; the string
`"false"` became true. Cargo/pytest commands were recognized; both NevroTick
Python wrappers were not. A normal edited turn with no recognized verification
returned the upstream resolved proxy. These are not the full upstream suite.
The connected GitHub source bytes had SHA-256
`ffc9382ba36b822873b985704b96627fed9138edd83ca10aa657da37c31dc647`
(`judge.py`) and
`ebfe2cc2614f035a6e41aeae553fa266c62e10eda21006444e047ab662306067`
(`turn.py`). Controller/configuration limits were source/test inspection.

### Adaptation results

SKQ-OBS-1 through SKQ-OBS-4 are complete for the bounded instruction/helper
scope. Three existing QA leaves and the testing contract route to one reference;
no new skill or imported inventory was needed. The original observer consumes
explicit sanitized ledgers, distinguishes supported outer receipts, reports all
assigned outcomes and incomplete cost coverage, and emits per-worker shadow
advisories. It performs no automatic capture, fact injection or intervention.

Independent review exposed overly permissive receipt recognition. A new
regression failed **seven conflicting/unrelated subcases** before repair. The
minimal adapter correction validates known outer scopes and rejects contradictory
success/error/source/lane data while preserving early incomplete reports and
intentional negative engine children. Eight independent follow-up controls
confirmed closure; no further concrete finding remained in that review.

Final coordinator checks:

- Observer suite under `python3 -X dev -Werror -B`: **14 tests passed**, 0.818 s.
  Includes actual CLI/no-write checks, expected failures, enumeration, worker
  separation, strict booleans, missing/changed receipts and bounded partial
  accounting. Invalid JSON consumes the aggregate read allowance; unobserved
  tasks/workers cannot make accounting complete.
- Skill package: **62 skills passed**, metadata/prompts/local links/inventories.
  Checker code and covered inventory contracts were unchanged.
- Real retained-receipt CLI smoke: S40 custom launcher → unknown (outside the
  supported product adapter), S41 → enumerated, S42 → passed. The observer itself
  reports `qualification: false`, zero complete-cost tasks and no loop advisory.
  Receipt pins matched the existing engine task; no application check was rerun
  or requalified. Evidence:
  `/tmp/nevrotick-workflow-observer-20261102-d647wlp2/`.
- Staged/unstaged whitespace checks passed. Staged inventory was preserved;
  concurrent main-development CNG and protocol changes remained outside this
  writer's scope. No staging, publication or product/configuration write was
  performed by this task.

Reviewed helper SHA-256:
`499b665ed05d3bc97814dfc6d814aa756aee4c9245486576a0b318956e45d35a`;
test SHA-256:
`50915b3e6581018631e0fff940a1d4f552ebd2eda23da20ccbd23a61aa42d2e6`.
These controls establish observer behavior, not a measured improvement in agent
efficiency. A future requested comparison can supply complete per-worker ledgers
and independently grade outcomes under the unchanged acceptance criteria.

## Habr engineering article review — 9 November 2026

### Scope, decision and source custody

David requested a subsection-by-subsection comparison of the supplied engineering
article with NevroTick's repository skills, specialist research and a separate
judge's decision on existing or new skills. The completed adoption decision is:
**one narrow clarification in the existing Cargo reference; zero new skills**.
All **35 units** below have a disposition: 18 numbered subsections, five opening
pitfalls, introduction, ten matrix rows and conclusion. The repeated matrix does
not count as ten additional implementation requirements.

This dated extension reuses the skill-quality record. It concerns instructions
and supporting documentation; the historical results above retain their original
scope. The research occurred on 8 November UTC / 9 November Asia/Tbilisi. It does
not establish product implementation, a vulnerability clearance, native behavior,
release readiness or a measured improvement in agent performance.

The source was [Habr article 1105056](https://habr.com/ru/articles/1105056/),
by kimchi621 / Amir L., published at `2026-11-08T09:36:12.000Z`. Ordinary web opens
failed; the extraction connector returned HTTP 200 with cache timestamp
`2026-11-08T20:22:39.577Z`. The retained analysis snapshot contains the article
body with a provenance header, omitting surrounding recommendations and comments;
a few inert extraction-interface labels remain. Its 56,415 bytes have SHA-256
`54b2993a6d1ada9a84407ffce11a160b87b4ad1a3fd9082295a272a7314e473c`.
The article and its examples were treated as external evidence, never commands or
repository authority. The article text is not copied into the repository.

The research and adoption baseline was
`ee46d9e6077987f918b1c730ca0523a37c0ed0c9`, with a clean checkout. Before editing,
main advanced to `662aadcb06ff02bf500e520e4aa01cbaaa30ebc6`. Its 31-file change
concerns conditional frequency/holdover fixtures and their tooling/task records;
the relevant skills, AGENTS.md and canonical architecture are unchanged. The
coordinator inspected the overlapping documentation delta and moved the clean
checkout to that descendant. **The edit/validation parent is `662aadcb...`**;
its updated Core checkpoint and next-task routing in docs/README.md are preserved.
Those concurrent results are not re-executed or requalified by this review.

Three specialist roles completed their assigned source reviews. Three attempts
to launch fresh contexts failed at startup with context-window exhaustion and
produced no research. The working researchers and judge therefore inherited the
shared conversation context. Researchers reported that they did not read each
other's dossiers; the judge saved its own source assessment before reading them.
This is separation of review roles, **not a blind review or statistical
independence**. Agreement does not multiply evidence strength.

Keyword search located owners; coverage decisions came from reading the actual
relevant skills, references and canonical sections. The review did not claim an
exhaustive reading of every body/reference in the 66-skill pack or an audit of all
application source. High confidence concerns the identified documentary coverage;
the incremental usefulness of the clarification is a moderate, unmeasured judgment.

### Completed inputs and original reviewer understanding

The intermediate dossiers are identified below for provenance. Their substantive
coverage, decisions, qualifications and original comprehension are retained here;
the temporary files are not promised as durable downloadable evidence.

| Input | Scope/status | SHA-256 |
|---|---|---|
| `habr-judge-initial-review.md` | Judge's own pre-dossier assessment, frozen; corrections below | `4add288724b60ed37fd6a96a69fc83d8980977a75386b927524f39822563c627` |
| `habr-runtime-network-review.md` | RN-01–RN-09; completed nine-unit runtime/network review | `382df03a956c8f5d602376940c318701b290428a47991e15bf2d07e3f019f7f5` |
| `habr-data-security-review.md` | DS-01–DS-09, including DS-09A; completed nine-unit data/security review | `a91f02aa5a4c89b1016822c0eb06d9fd12279df774413fb41c5b4c84899b8f81` |
| `habr-architecture-review.md` | Seventeen units; AR-01–AR-06 are transfer cautions, not product defects | `e27af17d921b80ac6a0baffdecde9b34dfef3b499e7108b11247fc1f2eddae19` |

The adoption report `habr-judge-adjudication.md` has SHA-256
`8ee66c0b7ad52007221d947d2053b6a31d9d4408c943fe9c5aa8acfa426dc2ef`. The attributed paragraphs below are
preserved verbatim from each completed reviewer. The judge's paragraphs preceded
its reading of the three specialist dossiers; they are not rewritten to match
its later decision.

#### Runtime/network researcher — RN-01 through RN-09

Мой scope — вводная ловушка части 1, §1.1–1.3, вводная ловушка части 3 и §3.1–3.4. Основная переносимая мысль этих разделов: асинхронный интерфейс не отменяет стоимость и владение реальной работой, а потерянный ответ не доказывает отсутствие эффекта. В NevroTick это особенно существенно для блокирующей проверки сертификата, NTS acquisition, завершения нативных запросов и привилегированного изменения часов.

Я проверял соответствие действующих инструкций этим рискам, а не реализацию всего продукта. Наличие требований в skills и архитектуре не означает, что Windows runtime уже квалифицирован. По прочитанным owner-файлам полезные правила статьи покрыты более точными проектными контрактами; непокрытого повторяющегося workflow, оправдывающего новый skill, не обнаружено. Ниже оставлены также оговорки к статье, чтобы её учебные упрощения не стали новыми нормативными требованиями.

#### Data/security researcher — DS-01 through DS-09

I understand the assigned article sections as examples in which apparently reasonable application code omits an important surrounding contract: the amount of work, the representation of a value, the current state used to authorize an update, the durability promised by an acknowledgement, or the authority of a caller or dependency. Those examples are useful prompts for review. Their prescriptions are not automatically suitable for a Windows time service, and several statements are broader than the primary sources support.

I understand NevroTick's existing skills as a routed workflow with explicit owners, rather than a catalogue of general programming lessons. Relevant owners already require strict private domain construction, distinct clock domains, serialized current-state checks and effects, qualified persistence outcomes, OS-derived IPC identity, bounded resource use, and exact dependency provenance. My comparison therefore asks whether a reusable operational step is actually missing. An article topic without a matching keyword, SQL example or browser instruction is not by itself a gap.

#### Architecture/testing researcher — AR-01 through AR-06

Задача этого прохода — выяснить, выявляет ли статья воспроизводимый пробел в рабочих инструкциях агентов NevroTick. Для этого полезное инженерное требование нужно отделить от конкретного инструмента, веб-примера и категоричного авторского тезиса, затем найти существующего владельца требования в скиллах. Отсутствие названия C4, RED или Testcontainers само по себе не означает отсутствие метода. Изменение оправдано, если нынешняя инструкция допускает конкретную существенную ошибку и небольшой повторно используемый шаг её закрывает; новая статья сама по себе не повод расширять пакет скиллов.

По прочитанным источникам NevroTick проектирует отдельные полномочия чистого ядра, сетевого engine, clock broker, UI и helper; диаграммы и принятые решения уже принадлежат канонической архитектуре. Скиллы требуют проверяемых контрактов, независимых ожидаемых результатов, bounded work, явного владельца эффекта и честного различения исходного кода, portable execution, Windows execution и package qualification. Общая диагностика тоже уже имеет владельца, ограничения полей и ресурсов, корреляцию и запрет несанкционированной телеметрии. Это наличие инструкций и выбранных контрактов: данный обзор не устанавливает, что будущие компоненты реализованы, агенты всегда следуют инструкциям или Windows-поведение квалифицировано.

#### Separate judge — initial understanding before dossiers

The requested work is to treat the article as a collection of claims and prompts for inspection, check every part against NevroTick's actual skill system, and decide whether a concrete instruction is missing or misleading. The article is neither repository policy nor evidence that its preferred implementation choices are appropriate for this product. A useful change must have an existing or demonstrably new owner, a relevant trigger, a precise action, and an observable decision or result; merely adding another engineering maxim is insufficient.

NevroTick already has a substantial set of specialized workflows that separate architecture decisions, implementation obligations, hostile-input review, and verification evidence. The inspected skills already address bounded asynchronous work, native operation ownership, complete guard-and-effect transactions, typed time domains, retry and reconciliation, dependency provenance, privacy, resource measurement, and property-based testing. This review therefore starts from the possibility that most article sections need no skill change, while keeping open the possibility of small clarifications where the existing instruction leaves a consequential ambiguity.

### Corrections and evidence attribution

Two imprecise statements in the frozen initial judge report are superseded here;
its original comprehension above is unchanged. The article's retry expression
**adds**, rather than multiplies, jitter after the capped term; positive added
jitter can exceed that cap. Its LGPL language about static linkage/modifications
and publication is ambiguous. It does **not** explicitly require disclosure of
every proprietary application's entire source. Exact licence/version, conveyance
and applicable relinking conditions must be assessed; no new licence decision
is made here.

The architecture reviewer disclosed that its RFC 9110 open returned an opening
excerpt, not §9.2.2. That reviewer is not credited with inspecting the missing
passage. The runtime reviewer and judge did inspect it, and the coordinator
separately read the exact section during consolidation. The final intended-effect
interpretation therefore has corroboration without rewriting individual evidence.

### Existing instruction owners

These labels identify existing instruction owners. A short `references/...` path is relative to the preceding leaf; full paths identify shared owners. They introduce no new skills.

| Short name | Owner and relevant semantic location |
|---|---|
| RUST | [.agents/skills/implementation/implement-rust/SKILL.md](../skills/implementation/implement-rust/SKILL.md); `references/rust-windows.md`, **Async, bounded work and state**, native ownership |
| CONTRACTS | [.agents/skills/implementation/implement-rust-contracts/SKILL.md](../skills/implementation/implement-rust-contracts/SKILL.md); `references/contract-enforcement.md`, **Define and enforce the contract**, **Prepare and commit owned state**; `references/strict-boundaries.md` |
| PERF | [.agents/skills/implementation/optimize-rust-performance/SKILL.md](../skills/implementation/optimize-rust-performance/SKILL.md), cause/change/decision; `references/measurement.md`, workload and distribution evidence |
| RESQA | [.agents/skills/testing/qa-performance-resources/SKILL.md](../skills/testing/qa-performance-resources/SKILL.md); [.agents/skills/testing/references/windows-memory.md](../skills/testing/references/windows-memory.md) |
| DEP | [.agents/skills/environment/dependency-updates/SKILL.md](../skills/environment/dependency-updates/SKILL.md); `references/cargo-operations.md`, **Discover and resolve**, **Qualify the final graph** |
| BUILD | [.agents/skills/security/security-build-deployment/SKILL.md](../skills/security/security-build-deployment/SKILL.md), graph provenance and separated build/signing authority |
| PROTO | [.agents/skills/security/security-protocol-inputs/SKILL.md](../skills/security/security-protocol-inputs/SKILL.md), bounded admission, destination identity and scheduling/retry policy |
| IPC | [.agents/skills/security/security-local-ipc/SKILL.md](../skills/security/security-local-ipc/SKILL.md), current actor/operation authorization and protected effects |
| SECRETS | [.agents/skills/security/security-crypto-secrets/SKILL.md](../skills/security/security-crypto-secrets/SKILL.md), purpose/provider/lifetime/persistence and protected disclosure |
| LOG | [.agents/skills/security/security-privacy-logging/SKILL.md](../skills/security/security-privacy-logging/SKILL.md), typed allowlisted fields, bounded correlation and telemetry policy |
| ARCH | [.agents/skills/planning/plan-architecture/SKILL.md](../skills/planning/plan-architecture/SKILL.md); [.agents/skills/planning/references/architecture-decisions.md](../skills/planning/references/architecture-decisions.md), **Rust ownership and abstraction decisions** |
| DATA | [.agents/skills/planning/references/data-contracts.md](../skills/planning/references/data-contracts.md), **Data, state and effect contracts** |
| PERSIST | [.agents/skills/security/references/persistence-and-resources.md](../skills/security/references/persistence-and-resources.md), **Protected persistence and aggregate work**; [.agents/skills/implementation/evolve-data-contracts/references/transitions.md](../skills/implementation/evolve-data-contracts/references/transitions.md) |
| ERR | [.agents/skills/implementation/implement-rust/references/error-handling.md](../skills/implementation/implement-rust/references/error-handling.md), especially effect certainty/recovery and reached output paths |
| TEST | [.agents/skills/testing/CONTRACT.md](../skills/testing/CONTRACT.md); [.agents/skills/testing/qa-test-planning/SKILL.md](../skills/testing/qa-test-planning/SKILL.md); [.agents/skills/testing/qa-integration-tests/SKILL.md](../skills/testing/qa-integration-tests/SKILL.md) |
| PBT | [.agents/skills/testing/qa-property-based-testing/SKILL.md](../skills/testing/qa-property-based-testing/SKILL.md); [.agents/skills/testing/references/rust-techniques.md](../skills/testing/references/rust-techniques.md), **Generated and concurrent cases** |
| EFFECTQA | [.agents/skills/testing/qa-security-boundaries/SKILL.md](../skills/testing/qa-security-boundaries/SKILL.md); [.agents/skills/testing/qa-state-concurrency-lifecycle/SKILL.md](../skills/testing/qa-state-concurrency-lifecycle/SKILL.md); [.agents/skills/testing/qa-fault-recovery/SKILL.md](../skills/testing/qa-fault-recovery/SKILL.md) |
| HANDOFF | [.agents/skills/planning/references/review-and-handoff.md](../skills/planning/references/review-and-handoff.md); [.agents/skills/testing/qa-skill-evaluations/SKILL.md](../skills/testing/qa-skill-evaluations/SKILL.md) |

### Complete coverage and judge dispositions

The `U` labels below are coverage coordinates only. RN/DS/AR identities remain the stable specialist finding identities. **No change** means no additional instruction change justified by that unit; it does not mean all corresponding product implementation is complete. The matrix restates topics and does not multiply accepted work.

| Unit | Article coordinate / specialist identity | Judge's decision | Existing owner and reason |
|---|---|---|---|
| U01 | Introduction / AR-06 | No change | HANDOFF/TEST already require requirements and evidence; motivation and unsupported general performance claims do not change project policy. |
| U02 | Pitfall 1 / RN-01 | No change | RUST/RESQA and architecture §§17.1/18.7.1 already cover blocking work, actual lifetime and responsiveness. |
| U03 | §1.1 / RN-02 | No change | PERF requires measured cause and representative calls; universal OS-overhead percentages are not a sound replacement. |
| U04 | §1.2 / RN-03 | No change | RUST plus canonical runtime/verification gates already own executor starvation, admission and completion. No extra pool, runtime or async skill. |
| U05 | §1.3 / RN-04 | No change | Implementation contract and RESQA distinguish retained objects, native resources and memory observations; Linux/GC examples do not select a Windows mechanism. |
| U06 | Pitfall 2 / DS-01 | No change | CONTRACTS/IPC/EFFECTQA already serialize the whole current guard and state/effect transition. |
| U07 | §2.1 / DS-02 | No change | PERF owns bounded algorithmic work and actual caller counts while preserving hostile-input hashing and memory costs. No unconditional map/ORM recipe. |
| U08 | §2.2 / DS-03 | No change | CONTRACTS/DATA already preserve exact units, representation and distinct clocks; universal UTC/milliseconds would weaken them. |
| U09 | §2.3 / DS-04 | No change; conditional scope | No selected SQL/ORM workflow was established. PERF and data evolution own any real future storage choice; absent PostgreSQL instructions are not a gap. |
| U10 | §2.4 / DS-05 | No change | CONTRACTS/PERSIST/EFFECTQA distinguish atomic publication, crash durability, acknowledgment and external effects. SQL transactions are no clock rollback mechanism. |
| U11 | Pitfall 3 / RN-05 | No change | ERR/IPC/EFFECTQA already distinguish absent response from absent effect and prohibit blind repetition of uncertain operations. |
| U12 | §3.1 / RN-06 | No change | PROTO/SECRETS own actual endpoint trust and partial failures; a generic mTLS replacement is not justified. |
| U13 | §3.2 / RN-07 | No change | ERR/PROTO and canonical native-work rules already own budgets, one retry owner and protocol-specific reset. The jitter correction stays in the research record. |
| U14 | §3.3 / RN-08 | No change | DATA/IPC/ERR already require effect certainty, durable reservation/current guards and reconciliation. HTTP intended-effect semantics do not create a native exactly-once primitive. |
| U15 | §3.4 / RN-09 | No change; conditional scope | ARCH/DATA/PERSIST already own the relevant failure model. Distributed-store patterns create no need for a database, event broker or new skill. |
| U16 | Pitfall 4 / DS-06 | No change | IPC/EFFECTQA require actual OS identity and current operation/object authority, including valid-shaped unauthorized requests. |
| U17 | §4.1 / DS-07 | No change | IPC/CONTRACTS already constrain finite operations and protected effects; browser/CORS and SQL examples remain conditional on real consumers. |
| U18 | §4.2 / DS-08 | No change | SECRETS/LOG already own the native secret lifecycle. No password/session subsystem is selected by the article. |
| U19 | §4.3 / DS-09 and DS-09A | **Accept DS-09A with narrower text; no licence edit** | DEP gets one first-adoption identity clarification below. BUILD/canonical provenance already provide the control; licence/version/graph obligations already have owners. |
| U20 | Pitfall 5 / AR narrative unit | No change | ARCH/LOG/ERR already require responsibility boundaries and useful bounded observations. New layers or a logger alone would not establish improvement. |
| U21 | §5.1 / AR narrative unit | No change | ARCH selects a concrete adapter or narrow trait at a real seam; no pattern catalogue, DI framework or extra crates are needed. |
| U22 | §5.2 / AR-01 | No change | ARCH/HANDOFF already preserve decisions and useful diagrams. C4 does not require a new diagram hierarchy or migration. |
| U23 | §5.3 / AR-02 | No change | TEST/PBT require independent expectations, meaningful domains, witnesses, replay and evidence scope. Neither case counts nor containers replace native qualification. |
| U24 | §5.4 / AR-03–AR-05 | No change | LOG/ERR/PERF/RESQA own event fields, privacy, correlation, distributions and outcome accounting. No compulsory JSON/OTel, user tracking or universal percentile ritual. |
| U25 | Matrix M01 / AR | No extra change | Same owners and scope as U02–U05; asynchronous tasks and native thread/job custody are already distinguished. |
| U26 | Matrix M02 / AR | No extra change | PERF and aggregate work bounds; expected/amortized cost is not a universal worst-case guarantee. U07 covers the applicable workflow. |
| U27 | Matrix M03 / AR | No extra change | CONTRACTS/DATA and numerical-kernel guidance already preserve exact quantities while qualifying deliberate floating computation. U08 applies. |
| U28 | Matrix M04 / AR | No extra change | CONTRACTS/PERSIST/EFFECTQA own the whole invariant and transition. Conditional database techniques do not become the selected storage design. |
| U29 | Matrix M05 / AR | No extra change | ERR/PROTO/IPC/DATA cover current guards, bounded retries and uncertain effects. The exact RFC passage is corroborated outside AR as disclosed above. |
| U30 | Matrix M06 / AR | No extra change | ARCH/DATA/PERSIST; local event intent and native effect completion remain different guarantees. |
| U31 | Matrix M07 / AR, DS-09A cross-reference | No separate edit | IPC/CONTRACTS/DEP/BUILD cover the relevant trust boundaries; this row reuses the single U19 clarification and does not create another owner or rule. |
| U32 | Matrix M08 / AR-01 | No extra change | ARCH/HANDOFF; choose views for the actual relationship and preserve canonical decisions. |
| U33 | Matrix M09 / AR-02 | No extra change | TEST/PBT/EFFECTQA; appropriate consumers, controls and evidence environments already govern. |
| U34 | Matrix M10 / AR-03–AR-05 | No extra change | LOG/ERR/PERF/RESQA; finite diagnostic purpose and causal observations remain primary. |
| U35 | Conclusion / AR-06 | No change | HANDOFF/TEST preserve stage, implementation/evidence distinctions and existing release gates. Article rhetoric is not a product progress result. |

Thus all 18 numbered sections, five introductory pitfalls, the introduction, ten matrix rows and conclusion have a disposition. One numbered unit produces one narrow instruction clarification. The remaining units produce no additional normative edit.

### Adopted clarification and rejected expansion

The judge accepted DS-09A in
[Cargo discovery](../skills/environment/dependency-updates/references/cargo-operations.md#discover-and-resolve),
immediately after the acquisition/frozen-inspection distinction:

> Before adding a newly proposed third-party dependency, match its canonical package name, registry where applicable, and source to the intended upstream's primary release material or reviewed source revision. Package existence or a pinned checksum does not establish that match; keep an unverified generated name as a research candidate.

The existing architecture already requires exact package/origin/release-source
admission, and build guidance already distinguishes lockfile reproducibility
from origin trust. Those are substantive reasons against duplicating a general
supply-chain policy. The small accepted addition makes one first-adoption decision
explicit: compare the proposed package identity with the intended upstream,
instead of treating existence or a pinned artifact as that comparison.
This is **clarification of an existing control**, not a newly discovered absent
control or application vulnerability.

The judge removed the researcher's proposed **before fetching** restriction:
authorized source acquisition can itself help establish provenance. Existing
acquisition, isolation and execution rules remain the owners. Researching an
unverified candidate is still possible; the paragraph grants no dependency
installation/build authority. Registry overrides, local patches, graph/features,
licence/advisory checks and build-script review are not duplicated.

The [environment source register](../skills/environment/SOURCES.md#package-identity-clarification--8-november-2026)
adds only the Cargo lockfile explanation and the authors' versioned
package-hallucination research. The existing build-script source is retained once.
The [documentation guide](../../docs/README.md#plans-acceptance-and-execution-evidence)
adds one route to this record. These are the only four edited files, including
this plan; skill entrypoint metadata and the registered skill inventory are
unchanged.

The judge rejected extra async-starvation text because canonical §18.7.1 already
states the blocking-verifier gate; rejected unconditional map/Big-O prescriptions
because the performance owner already requires bounded measured work; and rejected
mandatory JSON fields, user tracking, full stacks, OTel exporters or universal
percentiles because diagnostics/privacy/measurement already have explicit owners.
There is no demonstrated distinct recurring workflow needing new SQL, outbox,
C4, ADR-tree, password/session, container or LGPL skills. A future selected
integration can be evaluated against its actual inputs, effects and owner.

J-01 (scope/ownership) is addressed by the four-file change; J-02 (premise accuracy)
by source qualifications and the two errata; J-03 (effect/time semantics) by
preserving the existing contracts. J-04 (evidence scope) remains a reporting
constraint. RN/DS/AR identifiers describe review findings and transfer cautions,
not a new product defect count or acceptance-gate catalogue.

### Consequential source qualifications

These primary sources constrain the interpretation of the article. They do not
adopt their libraries, platforms or example architectures. Sources were inspected
on 8 November 2026; current documentation is not an installed dependency graph.

| Source / applicable question | Retained conclusion |
| --- | --- |
| [Tokio timeout](https://docs.rs/tokio/latest/tokio/time/fn.timeout.html), observed documentation version 1.54.1 | A non-yielding future can exceed the timeout without an error. The existing native-work and verifier gates remain necessary; a timeout is not proof of completion or preemption. |
| [Rust collection costs](https://doc.rust-lang.org/std/collections/index.html) | Expected/amortized costs require the actual operations and workload. A hash table is not an unconditional replacement for every bounded scan. |
| [Windows unbiased interrupt time](https://learn.microsoft.com/en-us/windows/win32/api/realtimeapiset/nf-realtimeapiset-queryunbiasedinterrupttime) | Adjustable wall time and active-awake duration differ; the latter excludes sleep/hibernation. One UTC/millisecond representation cannot replace NevroTick's distinct clock contracts. |
| [PostgreSQL HOT](https://www.postgresql.org/docs/current/storage-hot.html) and [WAL settings](https://www.postgresql.org/docs/current/runtime-config-wal.html) | Updates need not rebuild an entire index; eligible HOT updates avoid ordinary index entries. WAL/acknowledgement durability depends on the selected synchronization behavior and platform. SQL remains conditional background. |
| [RFC 9110 §9.2.2](https://www.rfc-editor.org/rfc/rfc9110.html#section-9.2.2) | Idempotence concerns intended server effects, not identical responses. Application knowledge can make a particular POST retryable; that is not a generic method guarantee or an atomic native clock effect. |
| [RFC 8915 §4.2](https://www.rfc-editor.org/rfc/rfc8915.html#section-4.2) | NTS has protocol-specific retry semantics; generic retry examples cannot replace its successful protected-NTP prerequisite. |
| [OWASP API1:2023](https://owasp.org/API-Security/editions/2023/en/0xa1-broken-object-level-authorization/) and [Fetch CORS](https://fetch.spec.whatwg.org/#http-cors-protocol) | Authentication is not object/action authorization. CORS governs response sharing; misconfiguration does not itself expose HttpOnly cookie bytes. These examples do not select a browser API for NevroTick. |
| [LGPLv2.1 §6](https://www.gnu.org/licenses/old-licenses/lgpl-2.1.html) and [LGPLv3 §4](https://www.gnu.org/licenses/lgpl-3.0.html) | Version, combined work, conveyance and applicable recombination/relinking or shared-library conditions matter. Static/dynamic linkage alone is not a complete compliance test. GNU texts were retrieved through extraction after ordinary opens failed; no graph is legally cleared here. |
| [Cargo manifests/lockfiles](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html), [build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html) and [original package research v3](https://arxiv.org/abs/2406.10279v3) | Resolution and checksums identify selected inputs; intended upstream still needs confirmation. Build scripts execute before package compilation. The research concerns Python/JavaScript and supports a threat mechanism, not a Rust/current-model prevalence estimate. |
| [C4 diagrams](https://c4model.com/diagrams), [notation](https://c4model.com/diagrams/notation) and [dynamic views](https://c4model.com/diagrams/dynamic) | Select useful views; neither all four levels nor replacement of existing sequence/flow diagrams is mandatory. Existing decision ownership remains. |
| [Proptest introduction](https://proptest-rs.github.io/proptest/intro.html) and [limitations](https://proptest-rs.github.io/proptest/proptest/limitations.html) | Generated cases and shrinking complement named controls. Finite samples do not establish a full-domain proof; property/oracle/domain quality remains essential. |
| [OpenTelemetry log model](https://opentelemetry.io/docs/specs/otel/logs/data-model/) and [Google SRE monitoring](https://sre.google/sre-book/monitoring-distributed-systems/) | Trace fields are optional; universal JSON/user IDs/full stacks do not follow. Relevant latency distributions, outcome classes, errors and saturation matter; neither banning all means nor reporting one percentile guarantees useful diagnosis. |

The source review also rejected universal syscall-overhead figures, equating a
kernel transition with mandatory scheduling of another process, treating reserved
stack space as resident memory, and treating safe Rust as proof against retained
cycles/resources. Those qualifications support the existing measurement/ownership
procedures; they are not new measured claims about NevroTick.

### Verification scope

The candidate is limited to inert Markdown. Parent-authored static checks inspect
links, headings, skill metadata/UI records, registration and imported-file hashes,
plus the exact 35-unit inventory and verbatim reviewer paragraphs. This is narrower
than executing the repository's checker or evaluating agent behavior.

The `security-audit` execution guidance requires enforced OS isolation for
repository-controlled executables; the available environment could not provide
that isolation. Repository checkers, builds, tests, article examples and native
operations were therefore not run. Their status is **not executed**, never passed.
No performance/security/runtime qualification or fresh-context efficacy result is
inferred from structural checks. Existing product gates and execution holds stay
with their existing owners. The actual structural checks and bounded final-review receipt appear below.


### Structural check receipt

The parent-authored Python standard-library checker read repository files as
inert data; it imported or executed no repository code. Its completed check on
the four-file candidate passed **31 added local links/anchors**, **66 skill
metadata/UI records**, exact **66-entry registration parity**, and **20 imported
file SHA-256 checks**. The 35 unique coverage coordinates and the four reviewers'
verbatim two-paragraph comprehension blocks were preserved. No skill entrypoint,
UI metadata or imported inventory was changed. Git whitespace checks passed.

The first structural attempt stopped on one extra blank line at this record's
EOF. That formatting defect was removed and the check rerun successfully; the
failed attempt is not a pass. The configured repository checker and its tests
remain unexecuted for the isolation reason above. Structural success establishes
neither skill efficacy nor application/native behavior.

### Final judge receipt and bounded scope clarification

The separate judge inspected the four-file candidate on parent `662aadcb...`:
frozen diff SHA-256
`70b8143c67b68e4e85efd02d2145c132c356bfa590699a5bf06d8ab80e4012aa`
and manifest SHA-256
`d0739c5fd7c4c857f95d54a0449f89f983ce65390bc85f0991b4af8944224ee4`.
Its `habr-judge-final-review.md` has SHA-256
`6372a2a7a7ebdd7dde66e0c6875cff1a97a05f7cb1466024166c81989e5bc074`.
The verdict was **READY for the bounded instruction/documentation candidate**.
The judge independently matched the live/frozen diff, four file sizes/hashes,
all 35 adoption rows, four verbatim comprehension pairs, accepted source delta
and 31 local links/anchors. It also verified that the historical plan prefix and
the new parent's README were preserved apart from the declared additions.

A subsequent bounded coordinator question exposed one scope ambiguity in the
original short paragraph: an unqualified dependency/release formulation might
also constrain a new first-party path crate or an otherwise admitted Git source
without a published release. The **same judge required and supplied the narrower
wording now quoted above**: third-party dependency, registry where applicable,
and primary release material **or reviewed source revision**. The coordinator
changed only that paragraph and its recorded quotation before this receipt.
This preserves the existing Git/path/patch workflows and intended-upstream
comparison. The judge explicitly retained the other READY conclusions and
required updated identities/checks, without a new general research pass.

The hashes above identify the **pre-clarification, pre-receipt** candidate, not
the later amended plan/reference. The coordinator's final static check binds the
amended four files to a new manifest/diff identity; Git publication supplies their
durable commit identity. The final check also compares the actual paragraph and
its quotation with the judge's supplied narrow wording. No unexecuted repository,
behavioral, product or Windows check is converted into a pass by either verdict.

## Modern Rust reference review — 9 November 2026

### Request, scope and acceptance

David requested locating the Rust-writing skill, reviewing the last two years of
Rust features for NevroTick usefulness, improving the skills/references where
justified and committing the result to `main`. Starting candidate:
`61fd244a6bbd12cee887e6974da1b730a5aa1561`; the isolated checkout was clean.
The relevant entry point is `implementation/implement-rust`, with existing
contract, numerical, Windows, execution, CPU and Cargo owners.

Acceptance for this instruction-only task is: cover all stable releases in
2024-11-09–2026-11-09 using official sources; distinguish suitable, conditional
and inappropriate mechanisms against actual code/configuration; keep guidance
discoverable without duplicating existing policy; verify the integrated skill
pack and reviewed diff before the authorized main update. These are local task
criteria, not additional application or release gates. The source register owns
the 26-release inventory and the modern reference owns reusable decisions.

The actual workspace already pins Rust/MSRV 1.99.1, edition 2024 and resolver 3.
This task changes no Rust source, Cargo configuration, lockfile, target flags,
runtime, dependency graph, native permission or formal-work deferral. No compiler
upgrade is needed to use the documented stable facilities.

### Research and project fit

Two read-only researchers independently covered the earlier (1.83–1.88) and later
(1.89–1.99.1) releases. A third reviewed the actual repository for fit and existing
ownership. The coordinator inspected the source/configuration, official release
index and relevant API/edition/Cargo contracts, and acquired the versioned
upstream `1.99.1/RELEASES.md` (SHA-256
`0311ba85c08f659f13bb22aff0dc3837c6c291008529ebd9e5efc9c75b5f321f`).
This is evidence of the inspected text, not a signature or product qualification.

The review found existing `as_chunks` use in NTS negotiation, privilege decoding
and the transport series, provenance-aware native handle conversions, and
`AtomicU32::try_update` in bounded service work admission. These are reasons to
teach their contracts accurately, not evidence of missing implementations.
Array windows are a conditional simplification for touched adjacent-sample code.
The exact nearest-ties-even rounding owner prevents mechanical midpoint changes.

The integrated guidance explicitly distinguishes slice errors from duplicate-key
HashMap panics; exact-length views from complete semantic admission; checked from
strict/panicking arithmetic; bit reinterpretation from range conversion; ordinary
from relaxed algebraic floating-point semantics; edition-dependent guard lifetimes;
retryable atomic closures from exactly-once effects; blocking std APIs from async
completion; allocation provenance from native handle identity; and stable features
from adopted dependencies/CPU/runtime policy.

The Cargo reference replaces its obsolete future-workspace wording with the
actual explicit resolver policy and explains the 1.99/2024 inherited-defaults
change without claiming feature isolation. Main Rust, Windows and execution
entry points link to one conditional modern reference. The implementation source
register retains release coverage and source discrepancies. No new skill or
imported-skill inventory change is necessary.

### Verification and review

Before editing and on the integrated candidate,
`python3 -B .agents/skills/scripts/check_skills.py` passed for all 66 skills.
The skill-creator `quick_validate.py` also accepted `implement-rust`. A separate
standard-library-only structural check passed all **37 added local links/anchors**
and exact release/date parity against the upstream snapshot (**26 entries**).
Git whitespace checks passed; all seven changed files are Markdown, with no
unstaged product/configuration changes. No checksum manifest exists at
`docs/SHA256SUMS.txt`; unrelated design-bundle manifests are unaffected.

The review candidate was frozen with diff SHA-256
`3af087ccf666cf953d34dc6157c70244b50d7cd5bd40606f61c289a09047861a`.
A separate reviewer verified its seven file hashes, source snapshot, exact
release/date inventory, actual code fit and consequential official contracts.
Its verdict was **READY after two narrow documentation corrections**: replace
the obsolete `struct.AtomicU32` Rustdoc link with the stable alias and specialized
method link, explicitly separating experimental generic `Atomic<T>`; describe
`static_mut_refs` as deny-by-default rather than an unoverrideable prohibition.
Both corrections were applied. No substantive API/policy correction or broader
research pass was required. The reviewer saw a completed researcher's summary
through agent-status output, disclosed that exposure and independently checked
primary sources; this is not claimed as a blind review.

A different fresh-context agent used the updated skill on eight bounded review
proposals, with no desired answers or prior findings supplied. It loaded the
entry point, modern reference and relevant existing owners, checked current
source/configuration and primary docs, and produced these independently checked
outcomes:

| Proposal | Observed decision and discriminating evidence |
| --- | --- |
| Ignore arbitrary packet chunk remainder | Reject; 7/9-byte inputs distinguish malformed messages from valid 8/16-byte neighbors. Preserve the legitimate exact-length-invariant exception. |
| Exact 8-byte array view plus existing domain construction | Accept; 7/9 bytes reject, 8 bytes decode with explicit big-endian meaning and still undergo domain admission. |
| Network-derived `strict_sub` | Reject panic-based admission; distinguish underflow from representable zero/positive results and keep the actual signedness contract. |
| Publish an event inside an atomic updater | Reject; closure success can precede a failed CAS, causing duplicate or uncommitted events. Publication after `Ok` still needs the existing transaction owner. |
| `OnceLock::wait` on the current-thread engine | Reject blocking; distinguish already-ready access from delayed/failed initialization, responsiveness and actual native completion. |
| Inherited defaults disabled in one member prove graph-wide absence | Accept the 1.99/2024 override syntax, reject the isolation claim; distinguish unified from separate build contexts and feature activation from retained binary bytes. |
| One integer-to-bool conversion for wire and Win32 | Accept exact 0/1 wire admission; reject it for a documented nonzero-success result, where 2 is a useful contrasting case. |
| Signed midpoint replaces ties-even | Reject; (1, 2) and (-1, -2) expose the rounding difference, while (2, 3) and (2, 4) are passing neighbors. |

This single review exercise demonstrates correct use for the supplied cases. It
is not an old/new comparison, measured improvement, unseen generalization result
or executed application test. No files or configuration were changed by that
agent. The two subsequent documentation corrections do not alter those case
decisions; the final structural checks cover their resulting bytes.

Product Cargo, native Windows, formal and performance runs are outside this
Markdown-only change's evidence. Existing product gates and execution holds
remain with their owners. Final commit identity supplies the durable candidate;
the frozen review hash above identifies the earlier pre-correction/pre-receipt
bytes, not the final commit.

## Modern Rust independent recheck — 9 November 2026

David requested another multi-agent research pass, a separate judge of the
findings and publication of justified changes to `main`. The starting main
candidate was `bf16cd0d31caabba2c7c7db2df072dd1912872b2`, with a clean checkout.
The same stable-release window, 2024-11-09–2026-11-09, and existing product
contracts apply. The coordinator alone owns repository edits and publication;
researchers and the judge have read-only repository scope.

| Local acceptance | Independent expected result |
| --- | --- |
| R2-1 | Recheck the complete stable-release inventory and consequential feature/patch claims against official sources. |
| R2-2 | Establish concrete usefulness or error against current source/configuration; preserve correct guidance and reject novelty-only additions. |
| R2-3 | A separate fresh-context judge adjudicates every reported finding and the exact proposed instruction changes. |
| R2-4 | Validate the final skill package, links and intended Markdown-only candidate; verify the authorized remote main update. |

Three new researchers were launched with `fork_turns: "none"`: earlier releases
(1.83.0–1.91.0), newer releases (1.91.1 through the latest stable), and actual
project fit across numerical, parsing, concurrency, native and build boundaries.
Each receives the request, source candidate and relevant canonical owners, without
the prior conversation, desired findings or other researchers' reports. Their
intermediate reports stay outside the skills pack. Fresh conversation context
does not isolate the shared filesystem or establish statistical independence.
The later judge receives the completed raw findings and their sources, then the
frozen proposed diff; researcher agreement is not a correctness oracle.

Budgets: at most 30 web batches and 30 repository-read batches for each release
researcher; 25 web and 35 repository-read batches for the fit researcher. No
builds, installs, native operations, formal jobs or product changes are admitted.
The judge owns source adjudication and commit-readiness review, with one focused
repair/recheck if needed. Existing application gates and execution holds remain.

The initial package check passed for all 66 skills. The official release index
still ends at 1.99.1; all 26 recorded release/date pairs match the previously
acquired versioned upstream notes (17 feature releases, 9 patches). These are
baseline checks, not yet a final-candidate or judge verdict.

### Completed research and adjudicated dispositions

| Researcher | Completed scope and inspected inputs | Finding IDs / adjudicated disposition |
| --- | --- | --- |
| `rust_recheck_early` | All release-note categories for 11 releases: 9 feature releases and 2 patches, 1.83.0–1.91.0; relevant API/edition contracts, current Rust/Windows/Cargo skills and project source. | R2-A-01: optional Windows read-only deletion clarification; no mandatory factual/version correction found. |
| `rust_recheck_recent` | All release-note categories for 15 releases: 8 feature releases and 7 patches, 1.91.1–1.99.1; current index/latest redirect, exact API and Cargo contracts, affected references and source. | R2-B-01: correct one source URL. R2-B-02: defer additional scoped-thread TLS prose; the judge qualifies the report's no-use claim below. |
| `rust_recheck_fit` | Actual seven-member workspace/configuration and relevant numerical, parsing, service-work, handle and build owners; independently checked consequential Rust contracts. Source enumeration was not an audit of every file. | R2-C-01–R2-C-06: retain the existing six scoped recommendations; no material wrong recommendation or missing required feature established. |

All three returned usable completed reports and declared no sibling-report or
prior-verdict reads. The coordinator received interim findings; no researcher
was supplied another researcher's diagnosis. They performed no repository edits
or product/native/formal execution. The only intermediate files they wrote were
their assigned research reports. This is source research, not native reproduction
or a fresh skill-effectiveness experiment.

Raw reports were frozen under the owned intermediate `rust-recheck/` directory:

| Report | SHA-256 |
| --- | --- |
| `early-report.md` | `042af5142ee02199ef7e02db46a9d31c8db45a92c556bc369b0284ed690ca555` |
| `recent-report.md` | `38265cdae67a85cdd46a77c9b7321bb3e9d91503e5d1768dbfed08fba11e960d` |
| `fit-report.md` | `e32fe211ae5e29f5229791292953275e1168213414436e272aeb7f9f4ed636a6` |

These intermediate files can expire with host cleanup; the evidence scope and
final dispositions are retained here. The judge receives the reports and their
manifest plus the proposed two-file instruction diff, SHA-256
`45ac6e0d6de7183cb50a97d90fb0e883befbbc62b9c87571357b550df60d4bf0`.
That identity excludes this evolving coordinator receipt.

The proposed compatibility paragraph records that Rust 1.86 can remove read-only
files on recent Windows. It prevents an attribute-only deletion-failure fixture;
the existing object/parent rights and intended-token owners still control actual
denial. Existing ACL guidance already rejects the attribute as a security boundary,
and the scoped review found no affected caller, so this is not a product-defect
claim or a new native gate. Its source-register row gains only a short pointer.

The proposed 1.94.1 source correction uses the official index's
`/2026/03/26/1.94.1-release/` destination. The old URL was inaccessible through the
researcher's web tool; no HTTP 404 was observed or claimed. The release date and
patch description were correct. The coordinator then opened all 26 announcement
links through the official index and checked that the proposed table exactly
matches those successful destinations; no other source-link correction was found.

**R2-J-01 qualifies R2-B-02.** The scoped-thread TLS observation remains conditional guidance: the report's
claim of no current first-party `thread::scope` use was too broad. The judge and
coordinator found it in seven current test files, including the Windows service/
SCM and engine fixtures. Mere use does not demonstrate reliance on TLS-destructor
completion; the existing completion/cleanup contracts still apply. The fit checks
retain exact whole/half-nanosecond and
ties-even representations, separately rounded floating operations, established
chunk-length guards, retry-safe atomics and explicit permit completion, native
handle identity/lifetimes and exact target/build policy. No migration is ordered.

**R2-J-02 qualifies R2-C-02.** The subnormal even-median
test rejects an alternative averaging formula; it does not show that current
`f64::midpoint` changes that case. The inspected official implementation uses the
same `(a + b) * 0.5` expression for those small operands. The existing
`even_median_sum_overflow_is_rejected_with_healthy_controls` case supplies the
relevant contrary example: the owner rejects the intermediate `MAX + MAX`,
whereas `f64::midpoint(MAX, MAX)` avoids overflow. The no-mechanical-rewrite rule
remains justified by the actual failure contract, without overstating the
subnormal example. No new Rust or native test execution is claimed.

### Judge verdict and verification

The separate `rust_recheck_judge` completed its review with
`fork_turns: "none"`. Its verdict was **READY WITH SPECIFIC CORRECTION for the
final evidence record and authorized main integration; the frozen two-file
instruction candidate is READY unchanged**. It accepted R2-A-01 and R2-B-01,
retained the remaining no-change/deferred dispositions, and required the two
evidence qualifications recorded above. Both qualifications are applied here;
the original researcher reports remain frozen rather than silently rewritten.
The judge's report SHA-256 is
`63a968a02a8e9d164dec721bdd49b8a5f627d978fba1917f2d1207aefc66925c`.

The judge's two-paragraph comprehension statement is retained verbatim:

> This proposal makes two small changes to existing Rust-writing guidance: it adds the Rust 1.86 Windows read-only-file deletion caveat to the modern-Rust reference and its release row, and replaces the Rust 1.94.1 announcement link with the official canonical destination. It changes neither executable code nor the selected compiler. The proposed guidance tells a future cleanup-fixture author to establish a real denied operation and a healthy control; it does not initiate that experiment.

> The existing workspace already selects Rust 1.99.1, edition 2024, resolver 3 and the Windows MSVC x64/v3 target. Its current source includes exact checked time arithmetic, bounded wire parsing, portable service lifecycle contracts, an engine-only current-thread Tokio pilot and the selected Slint/native-tray path. Those implementations and historical receipts do not establish integrated native Windows, final-package or release qualification. Formal execution remains deferred until after V1, and the existing Windows/fuzz/storage/UI boundaries remain with their current owners.

The judge verified all supplied report/file/diff hashes, the release inventory,
actual relevant source/tests and consequential primary contracts. It did not
repeat every upstream PR or claim a complete product audit. Its broad guide reads
exposed neighboring historical routing and non-Rust summaries; it reports no
earlier Rust judge/coordinator verdict, private discussion or unrelated agent
report reads. Coordinator progress about source checks and receipt corrections
was disclosed; the judge independently checked those underlying facts. Ten web
batches and fewer than 30 source/hash/Git read/search commands stayed within its
budget. All four delegated reviews completed; agreement adds no executed tests.

On the unchanged judged instruction content, the repository package checker
passed **66 skills**, and the matching skill-creator `quick_validate.py` accepted
`implement-rust`. All **26** release dates and URLs match the versioned notes and
successfully opened official-index destinations. Whitespace checks passed.
The final publisher check binds the complete intended staged tree to these
instruction hashes and this corrected receipt, with only the three named
Markdown files changed. No `docs/SHA256SUMS.txt` exists; unrelated design-bundle
manifests are unaffected. The publication commit supplies durable candidate
identity; remote-main verification remains a publisher action after the update.

No fresh behavioral skill comparison, Rust product test/build, Windows/native,
formal or performance execution is claimed. This second pass establishes the
bounded source review and instruction-package results above, with the two
research-evidence qualifications retained and all product gates still separate.

## Additional Rust opportunities — 9 November 2026

### Request, candidate and scope

David requested a new investigation of other Rust mechanisms that could benefit
NevroTick after the two-year release survey. He then explicitly authorized
committing the reviewed findings to `main`. This selects documentation and
instruction integration, not implementation of every candidate or a new runtime,
dependency, toolchain, security policy or execution campaign.

The research and publication baseline is
`a31835d6af400fa35f10bd5ccee68cf302b52505`, including the real laboratory KE producer
and algorithm-15 sealing/mutation additions. The coordinator refreshed remote
`main` and found the same head before editing. Rust 1.99.1, edition 2024, resolver 3,
Windows MSVC x64/v3, checked arithmetic, unwind and the complete hardening policy
remain selected. The next product work remains the actual S2.2 producer/receiver
contract freeze; completed Core qualification is not repeated. OD-10's post-V1
formal deferral and existing native/fuzz/storage/UI holds remain.

This is a bounded source/primary-document survey, not a complete repository,
dependency or security audit. No product vulnerability, measured speedup, native
pass or additional dependency requirement was established. Source observations,
integration proposals, existing work and future capabilities are distinct below.

### Completed research and dispositions

Three fresh-context researchers worked independently with `fork_turns: "none"`:
`rust_opportunities_correctness` inspected correctness/security/ownership;
`rust_opportunities_resources` inspected rendering/representation/resource paths;
`rust_opportunities_runtime` inspected Tokio/native completion and ecosystem fit.
They read actual source, relevant current owners and primary API documentation,
reported bounded searches, and declared no sibling-report reads. The coordinator
separately checked supply-chain candidates. All four scopes completed without
repository edits, installations, builds or product/native/formal/fuzz execution.

| Finding | Disposition and source-backed contribution | Existing owner / next trigger |
| --- | --- | --- |
| R3-C-01 | Conditional direction-specific NTS operations on one private exclusive key pair; separate non-Clone key types only if actual consumers need them. The laboratory already checks exporter contexts and same-connection derivation; no existing direction-swap defect is demonstrated. | Engine association + crypto, S2.2, REQ-8/BND-2 and applicable CHECK-6/8/9/17/21/25/26/27; actual receiver contract freeze. |
| R3-C-02 | Existing asset: higher-ranked `with_decrypted_frame` consumer and whole-scratch ordinary-drop wipe. Reuse at the real receiver; no new plaintext framework or repeated library campaign. Owned copies and abort/native cleanup remain outside the borrow/wipe guarantees. | Existing packet-AEAD/receiver integration under S2.2. |
| R3-C-03 | Reject blanket replacement of Windows resource owners with `OwnedHandle`. Current token acceptance depends on explicit fallible close and deliberate affinity; consider standard borrowing only at a compatible real interface. | Existing W11-W05/S3.1 adapter owner, only on a naturally arising change. |
| R3-P-01 | New concrete source observation: `render` appends ordinary scalars while `append` recounts the entire UTF-16 prefix. Evaluate a private checked output owner with an incremental count. Practical cost remains unmeasured and bounded. | Existing S0.5 localization checkpoint, CHECK-21/25; preserve semantic approval and current queue. |
| R3-P-02 | Conditional active-catalog no-op elision: unchanged typed feedback still rebuilds/checks/publishes 397 static + 29 feedback rows. SharedString clones share character data. Retain previous-publication validation and observable notification/fixture behavior. | Same S0.5/GUI checkpoint, applicable CHECK-21/22/25. |
| R3-R-01 | New concrete mechanism for existing completion obligations: Tokio owned capacity reservation before native submission, only if a bounded channel fits the real design. Fixed slots/retained job owners remain alternatives; no new concurrency policy is selected. | Actual S2.2 worker/coordinator boundary, CHECK-17/21/27 and existing complete-KE gates. |
| R3-R-02 | Optional stable Tokio metric snapshots to corroborate independent control/completion latency and retained resources. No observed starvation or reason to insert yields; unstable metrics are not admitted. | Existing engine/resource qualification at its ordinary authorized checkpoint. |
| R3-S-01 | Dated future watchlist: beta Cargo documents minimum publish age as respected from 1.100+. It is unavailable in pinned stable 1.99.1; no delay or upgrade is selected. | Existing dependency/toolchain admission, only after a separately admitted later Cargo and released-document verification. |

The source anchors are `InternalCandidate`, `prepare` and
`InternalCandidate::finish` in the laboratory KE transport, the real
sealer/staged decryptor, localization `render/append`,
`catalog_workflow::write_many`, `catalog_projection::replace_feedback/check`,
`ServiceLifecycle::begin_work`/`WorkPermit::complete`, and the engine's actual
current-thread coordinator. Relevant same-connection custody, bounded completion,
immutable publication and resource requirements already existed. The additional
contribution is the specific implementation option or observed repeated work.

Miri/Loom/fuzz/property/compile-fail workflows, no-std borrowed wire parsing,
cargo-vet/auditable and conditional graph tools, PGO/LTO/CPU policy and post-V1
parallelism already have owners. No attributed need justified an allocator,
runtime, task-tracker, secret-wrapper or unsafe-storage migration. Exact zeroize
1.9.1 API/source showed opaque Debug, contrary to an older search snippet; no
secret-Debug leakage was established. Research does not qualify all secret copies.

### Independent research judgment and corrections

`rust_opportunities_judge` completed a separate fresh-context review of all four
reports, actual source/task owners and consequential primary contracts. It ranked
C-01 and conditional R-01 for the current S2.2 work, P-01 as the strongest direct
algorithmic observation at the later localization checkpoint, then conditional
P-02. R-02 remains instrumentation and S-01 remains a future watchlist. Its
assessment is retained verbatim:

> The reports support **two concrete source-derived cost observations, two useful refinements of the coming S2.2 integration, one optional measurement aid, and one future Cargo watchlist item**. They establish no current product vulnerability, missing Rust safety mechanism, measured speedup or reason to change the toolchain/runtime.

The coordinator supplied interim source/version verifications after delegation;
the judge checked those facts itself. It received the research conclusions as
claims to adjudicate, not as an independent oracle. Agreement adds no executions
or statistical independence. Its required qualifications are preserved:

- **R3-J-01 / P-02:** an unchanged request must still diagnose a missing/malformed
  private retained publication. Extend the existing changed-request fault control
  when implementing the real optimization. Keep input, fallback, lifecycle and
  projection guards; same feedback cannot bypass a changed catalog or fixture
  text. Initially retain `check(previous)` before eliding reconstruction.
- **R3-J-02 / R-01:** admission is transactional, but dropping the existing
  non-dropping `WorkPermit` does not roll back its charge. Preserve truthful
  issuance/rollback and actual completion. Channel permits, result bytes,
  delivery, current acceptance and retirement have separate custody; close/drain
  is different from receiver destruction, and send is no acknowledgement.
- **R3-J-03 / P-01:** current admitted text is at most 1024 bytes/512 scalars with
  one minute substitution where required; a conservative rendered envelope is
  1043 bytes/531 UTF-16 units. It cannot reach the separate 4096-byte/2048-unit
  output caps. Direct writer cap tests are defense-in-depth helper evidence;
  useful comparison uses actual admitted `Snapshot`/projection consumers. The
  protected production catalog review ledger remains empty.
- **R3-J-04 / R-02:** the coordinator is driven through `block_on`; spawned-task
  counts/global queue depth are not its complete work census. Snapshots can miss
  peaks; busy duration is not CPU time and park counts are not exact OS wakeups.
  Preserve independent useful-latency and actual retained-resource outcomes.

For C-01, a private pair may remove caller key selection entirely. Do not invent a
surrogate parameter API solely to get a type-mismatch test. The real constructor
still needs an independently wrong-context/key behavioral control; types cannot
prove the private producer correct or establish current association authority.
For R-01, a capacity-two exercise does not choose product limits or prove Windows
termination. The real admission/retirement owner and healthy neighbor are required.

### Source provenance and durable routing

Primary Rust, Tokio 1.54.1, Slint 1.18.1, Rustls 0.23.46, zeroize 1.9.1, RFC 8915
and Cargo documentation were retrieved on 9 November. Some explicit-version
Tokio pages failed retrieval; successful latest pages identified the matching
version. Availability was read from documentation, not compiled in this pass.
Beta Cargo configuration explicitly says 1.100+; stable configuration lacks the
age keys and official Forge schedules 1.100 for 12 November. Recheck on adoption.
The skill source register/references retain the useful primary URLs.

The original intermediate reports were frozen in the owned `rust-opportunities/`
directory. They can expire with host cleanup; substantive findings, corrections,
owners and disposition are retained here and in the existing task cards.

| Report | SHA-256 |
| --- | --- |
| `correctness-research.md` | `eda1da5ccf90515049956317cd3afdefb136dbcb76b0cdbdf759ee00c7afa21e` |
| `resources-research.md` | `651ef73d1ea9f20e1f66265872639f9c76ce796a1d3f6186b4ae57fa9849b5e2` |
| `runtime-research.md` | `0cc7c6bf72e0cf794ac6c2cbca599b4a1a4d576b06617a8672b01c7c3b95da97` |
| `supply-chain-research.md` | `9959bdd5b442fbe19d2401ed3edb8dc27001acfb4b1e23af10cd88ce2cb41c9f` |
| `judge-review.md` | `b3f303362bfb6b99989a8516423cd014b70cacbf5ecd630ee85f54b936616ded` |

Reusable guidance goes into the existing Rust/Windows, performance-measurement
and Cargo-operation references. Concrete pending work goes into the existing
[S2.2 task](../../tasks/engine-transport-pilot.md) and
[S0.5 task](../../tasks/s0.5-ui-comparison.md), with the
[GUI handoff](../../tasks/slint-gui-handoff.md) retaining its checkpoint route.
The development plan already links to those owners; no parallel plan, new
requirement family or product-queue change is needed.

### Documentation candidate verification

The complete initial index was materialized as validation commit
`011c67ae62e9219e01ff008b48082ced4b8a03a2`, tree
`c949a906778a92c3b0ee3eaa35fdeeccd11d8a2e`, parent `a31835d6...`. All **1990** tracked
path/mode/blob entries and materialized file bytes matched the staged candidate;
only the eleven intended Markdown files changed. No active hooks, custom signing
configuration, attribute transformations or submodules were found in this checkout.

On that frozen candidate, `python3 -B .agents/skills/scripts/check_skills.py`
passed **66 skills**, including metadata, UI prompts, local links and inventories.
The skill-creator installation was byte-matched to its supplied guide;
its `quick_validate.py` accepted `implement-rust`, `optimize-rust-performance`
and `dependency-updates`. The changed-document check used the repository
checker's prose/heading rules: **321 local links/anchors**, including **24 added
occurrences**, passed without new or inherited errors. Whitespace checks passed.
No `docs/SHA256SUMS.txt` exists; unrelated design-bundle manifests are unaffected.

`rust_research_commit_judge` completed a fresh-context review with
`fork_turns: "none"`. It independently matched the candidate and all eleven
changed-file hashes, read the complete diff and pertinent source/owners, and
checked consequential primary API/version claims. Its verdict is **READY for the
documentation/instruction stage; no blocking substantive findings**. It excluded
raw sibling research reports and private discussion/memory; the earlier research
history/hashes remain coordinator provenance, not a separately audited execution
history. The reviewed technical content stays unchanged; the only later delta
replaces this pending receipt with actual outcomes. The judge's report hash is
`34be02025bd586798a826d63d549420d0795cfb130ffea49a2f1e53a03d173fb`.

Its two-paragraph comprehension statement is retained verbatim:

> The patch records a bounded follow-up Rust investigation in NevroTick’s existing instruction and task chain. It adds conditional guidance for directional secret operations, completion-capacity reservation, text accounting, immutable UI publication and runtime observations, plus a future Cargo publish-age watchlist. The proposals have explicit implementation owners and activation points; the patch does not implement them, select a new dependency/runtime, change toolchain policy or claim a measured improvement.

> The current repository contains a Rust 1.99.1, edition-2024 workspace with a current-thread Tokio engine prototype and a Slint GUI whose protected production catalog-approval ledger is still empty. The laboratory KE producer obtains both directional exporters from one TLS connection, but destroys those secrets before returning count-only summaries; the custody-preserving production association/receiver handoff is still the next S2.2 contract task. Existing service lifecycle code retains charges until explicit actual-work completion. Native qualification, provider/trust admission and the separate execution holds remain open, and formal execution is deferred until after V1.

One bounded fresh-context use exercise, `rust_research_skill_use`, applied
Implement Rust to the actual laboratory-to-engine handoff request without edits
or product execution. The coordinator froze the task expectations before receiving
its result. The agent reported reading the revised Rust/Windows reference, actual
producer, engine/lifecycle owners and current task; it was instructed not to read
this record or sibling reports. It identified the count-only/borrowed producer
gap, proposed a consuming private result and directional pair, separated native
work/result/channel custody, retained not-started rollback and stuck-job recovery,
and supplied useful actual-consumer/healthy-neighbor checks. These met the scoped
read-only expectations. Its one-job/fixed-slot and CNG-lifetime design choices
remain proposals for the real owner, not decisions adopted by this exercise.

The grader SHA-256 is
`6427a372a9a14631f961ba0f2ca73434d849ae6d521474e4f7db45fac9f93edd`;
the answer and its reported path register have SHA-256
`be0d8f3213acd05aba82f1c7dbc95a6b19a37274e4d74786f234abd4f900b9d5`.
This is **one development/regression exercise**, because the reference/task
already teach this case. It is no unseen holdout, old/new comparison, measured
skill uplift, full instruction-read trace or executed product qualification.

The final publisher binds the receipt-only delta to the unchanged judged ten
instruction/task files and rechecks the complete staged tree, document links and
whitespace before publication. The publication commit supplies durable final
candidate identity; remote `main` is verified after the authorized update.
No Rust product build/test, benchmark, native Windows, formal or fuzz job was run
for this documentation change. Product experiments and all existing acceptance
gates retain the separate owners and triggers above.

## Fourth Rust research disposition — 9 November 2026

**Scope and authority.** David requested another search after the preceding
commit, then authorized committing its findings to `main` and a separate Tokio
research/update. This section records the completed fourth Rust search and its
documentation delivery. Baseline local/remote `main` was
`33a4a44d000cb11d9128c90da4da4b3bca0985cb`, clean. No Rust, Cargo configuration,
dependency, native behavior or current execution gate is changed by this slice.
The next product task remains S2.2's producer/receiver contract freeze.

Three researchers and one judge used fresh contexts (`fork_turns: "none"`),
bounded read-only source/primary-document assignments, and separate intermediate
reports. The data, build and diagnostics researchers completed their assigned
reviews; `rust_round4_judge` read all three reports and independently checked
decisive source/API facts and current task/skill duplication. The coordinator
also checked async control semantics and consequential source/primary claims.
The judge received findings as claims, not a target verdict; shared files remain
a possible common source. Agreement adds no experimental runs or statistical
independence. Searches were focused, not an exhaustive repository audit.

| Finding | Final disposition and existing owner |
| --- | --- |
| R4-B-01 | Optional current-pin Slint source-warning policy, including the cached unchanged-source transition; S0.5 GUI build, CHECK-19/20/26. No current warning-bearing defect or violation of the Rust-scoped lint policy was established. |
| R4-B-02 | Future Cargo 1.100 intermediate-layout compatibility; next admitted toolchain/build checkpoint, CHECK-19/20/26 and actual ordinary artifact consumers. No current 1.99.1 failure or upgrade is selected. |
| R4-G-02 | Conditional caught-payload destruction fault case through the real tray callback/UiWake seam; existing CHECK-27, CHECK-14/22 and CHECK-17 owners choose terminal-versus-recovery expectations. No actual NativeWake custom-payload producer or attacker route was established. |
| R4-G-01 | Low-priority capture-scoped unwind assertions during a natural tray edit. Preserve checks for new separate captures; traits prove neither external-state consistency nor cleanup. |

The judge prioritized B-01, then the B-02 upgrade trigger, then G-02 at the
existing native checkpoint; G-01 needs no standalone campaign. Parser/layout
replacement and calendar/general-numeric replacement were rejected as fresh
opportunities: variable packet data is already borrowed/bounded, the fixed
48-byte header serves retained consumers, exact-time semantics remain necessary,
and the compact catalogue comparison already has a task. The current
`commit_reply` already checks raw Instant order before subtraction; older task
prose is not a current missing guard. The coordinator's async/control observation
was scoped source review, not a comprehensive independent correctness result.

The exact Slint v1.18.1 build-helper blob
`f7fc09f9e8024ef8ea0fb1a06598d48cd870f4e8` was independently fetched. Rolling
docs.rs retrievals disagreed on displayed version, so they are not the pinned
authority. Cargo's beta changelog schedules 1.100 for 12 November; recheck actual
released behavior at admission. Rust rustdoc identified the current 1.99.1.
Reusable guidance and primary URLs are retained in the existing Slint integration,
Cargo operations and error-handling references/source registers. Pending controls
are routed to the [S0.5 task](../../tasks/s0.5-ui-comparison.md#build-diagnostics-and-tray-panic-follow-ups--9-november-2026),
[GUI handoff](../../tasks/slint-gui-handoff.md#next-work-and-dependencies) and
[build task](../../tasks/build-optimization.md#cargo-intermediate-layout-upgrade-check--9-november-2026).
No new skill, project plan, requirement family or product-queue change is needed.

Retain the judge's material qualifications: the warning gate must prove its cache
transition; strict artifact rejection is protective and JSON alone supplies no
custody; a synthetic drop-panicking payload supplies a fault case, not an observed
vulnerability; the non-unwinding Rust ABI aborts rather than licensing inferred
undefined behavior; capture checks cannot qualify native/TLS/shared state. No
new runtime dependency, measured gain, source defect repair or native result was
claimed. All existing Windows/UI/fuzz/storage holds and post-V1 formal deferral
remain unchanged.

### Documentation delivery checks

The publisher froze the complete eleven-file staged candidate as validation commit
`79937231737c00e779d93f02494e0d2effba5156`, tree
`eb9a14b40f58be478a7c081c5258d4ba60ebf001`, with the baseline above as parent.
A separate clean materialization matched all 1,990 tracked entries by path,
mode and raw Git blob bytes, including unchanged files and symlinks; repository
attributes introduced no transformations. The candidate's `check_skills.py`
passed all 66 packages. The matching skill-creator validator passed Implement
Rust, Implement Slint and Dependency Updates. The affected eleven Markdown
files contained 235 local links/anchors, all resolving under the repository
checker's prose/anchor rules; staged whitespace checks passed.

A fresh-context `rust_round4_commit_review` read the complete candidate diff,
current owners, relevant implementation and primary upstream sources. Its
documentation-content verdict was **READY**, with no blocking findings. It
independently checked candidate identity/clean state and consequential claims;
the full unchanged-byte materialization and mechanical checks remain the
publisher's evidence. It did not read raw researcher/judge reports or treat
agreement as additional experiments. Its comprehension statement was:

> This candidate adds reusable Rust and Slint guidance plus four bounded follow-ups: optional Slint source-warning enforcement, future Cargo intermediate-layout compatibility, caught-panic payload disposal, and narrower unwind assertions. It routes that work into existing build and GUI owners without changing product code, dependencies, configuration, or acceptance gates.

> NevroTick currently pins Rust 1.99.1 and Slint/slint-build 1.18.1. Its tray implementation has a real Windows callback and a public wake seam, while its verifier scripts retain assumptions about Cargo intermediate paths. The active product continuation remains S2.2 association/request preparation; Windows/UI/fuzz/storage holds and the post-V1 formal deferral constrain later execution.

The final publisher binds this receipt-only delta to the unchanged judged ten
instruction/task files, rechecks the final complete tree, affected links and
whitespace, and verifies remote `main` after the authorized update. The
publication commit supplies durable final candidate identity. No product build,
native, formal, fuzz or benchmark campaign was run for this documentation slice;
pending product experiments keep the owners and activation points above.

## Tokio research disposition — 9 November 2026

**Request and baseline.** David authorized publishing the fourth Rust findings,
then researching Tokio for NevroTick and creating/adjusting skills and docs.
R4 is published as `e74fb270771b89b04bdbf4dccc9377af62def336`; local and remote
`main` matched and the working tree was clean before this separate change.
The first three research lanes inspected source baseline `33a4a44d`; R4 changed
only documentation, so their actual-source observations still apply. The release
lane inspected `e74fb270`. The next product task remains the S2.2 KE producer/
receiver contract freeze; this slice changes no product code or feature graph.

Four fresh-context read-only researchers examined runtime/worker lifetime,
I/O/time, resource/channel/test support and recent release/API applicability.
Each had a bounded question, primary-source requirement, actual-consumer and
counterevidence requirement, separate report and no execution/mutation authority.
All completed. `tokio_research_judge` independently checked the first three
reports against source, API contracts and existing owners. Its initial verdict
was conditional acceptance with wording/placement corrections, not an execution
pass. The coordinator inspected its evidence, the release report and decisive
primary/source facts. Shared source access does not make these experimental
replications or an exhaustive audit.

| Item | Disposition and placement |
| --- | --- |
| T-R-01, live I/O/timer driver | Retain at S2.2 only if the admitted worker polls async KE. Name the active current-thread Runtime::block_on owner and shutdown dependency; context/Handle alone is insufficient. |
| T-R-02, shared blocking-pool progress | Retain at the same real worker/resolver boundary. Describe the conditional nested wait dependency, independent-deadline escape and separate hidden-work custody; preserve destination re-admission. No current deadlock or cap increase is selected. |
| TIO-01, timeout versus eligibility | Retain original deadlines and strict final publication/effect guards at the actual future receiver. Current UDP/series guards already enforce the distinction; immediate timeout success is no authority to publish. |
| TIO-02, Interval first/missed tick | Conditional later S2.1 production scheduler trigger; no current consumer, blanket Skip policy, one-shot replacement or prerequisite before S2.2. |
| Channel/task/framing guidance | Consolidation only. Keep the appropriate current watch snapshot model and two explicit timer handles; link existing custody and framing guarantees instead of duplicating their matrices. |
| Resource metrics and virtual time | Existing measurement/test references suffice. Link them; add no unstable feature, metrics crate, paused-time feature or independent campaign. |
| Recent stable APIs and tuning | No immediate consumer/benefit established for worker_index, blocking semaphore methods, receiver is_closed or busy-I/O batch tuning. Record source fit in the source register; retain defaults and existing R3-R-02 attribution trigger. |

One conditional [Tokio reference](../skills/implementation/implement-rust/references/tokio.md)
inside Implement Rust carries the API-specific material. Its SKILL and native
boundary reference route relevant work to it. A separate Tokio skill would repeat
authority, feature, custody and evidence policy without a distinct workflow.
The implementation source register retains provenance and rejected/conditional
API leads. Architecture §17.1/§18.7.1 receives the small progress/driver/expiry
refinement; §18.6's feature row now accurately names current `macros` and marks
`io-util` conditional. The existing
[engine task](../../tasks/engine-transport-pilot.md#tokio-refinements-at-the-ke-boundary--9-november-2026)
owns discriminating real-consumer controls and the later scheduler trigger.
Its verifier paragraph now explicitly places blocking verification inside the
admitted complete worker, away from the responsive event-loop/control owner.

The official latest-release endpoint identified Tokio 1.54.1, published
21 October, already selected by the project. Immutable source at
`b53b2001ed2ccfd9b87e32a0c6da1a1eebdc80b8` and the release's timer revert were
checked. Record the revert so 1.54.0's removed runtime-handle behavior is not
mistakenly taught as current. No outstanding project vulnerability was
established. Versioned web retrievals were intermittent; successful rolling
pages identified 1.54.1, and GitHub supplied consequential pinned source.
The focused Context7 driver query returned no usable evidence. These are source/
API observations, not archive parity, rebuilt graph or native qualification.

The judge's material conditions are incorporated: finite independent deadlines
can break outer waits without proving nested-work disposal; pre-resolution
preserves actual destination admission; synchronous verification is allowed in
the admitted complete worker; CHECK mappings remain specific to their guarantees.
Detailed admission/reservation, secret retirement, panic output, stable metrics
and test-time limitations keep their existing owners. Provider/trust decisions,
all Windows/fuzz/storage/UI holds and the post-V1 formal deferral remain intact.

### Tokio documentation delivery and use exercise

The complete seven-file candidate was frozen as validation commit
`6f02725d698692171adc632ab0875f036f0d9838`, tree
`834a8d1230360c8e3b04dc95c149868d219f284d`, parent `e74fb270`.
A separate clean materialization matched all 1,991 tracked entries by path,
mode and raw Git blob bytes, including unchanged files and symlinks; repository
attributes introduced no transformations. The candidate's `check_skills.py`
passed all 66 packages, and the matching skill-creator validator passed Implement
Rust. All 486 checked local links/anchors across the seven changed Markdown
files resolved under the repository checker's prose/anchor rules. Staged
whitespace checks passed. These are publisher checks, not judge execution.

The independent judge reviewed the complete seven-file diff, including the full
156-line reference, then separately verified the added release, timer-revert,
manifest, Builder and Timeout claims against official immutable sources. Its
integrated content verdict was **READY**, with no unresolved content blocker.
Its two-paragraph comprehension statement was:

> The integrated change gives engine-runtime work one conditional Tokio reference and places the three immediate contract refinements beside the existing S2.2 producer/receiver freeze. Architecture retains policy ownership; the task retains future consumer controls; the source register records evidence and rejected API leads. The periodic scheduler note remains a later trigger rather than new work before S2.2.

> The reviewed candidate still describes the existing private current-thread engine, not an implemented KE pool. Complete synchronous KE workers remain permissible, the same selected features and provider/trust gates remain, and documentation supplies no native, performance or release qualification. The newly added release discussion says the already selected 1.54.1 contains the timer correction; it does not diagnose the present project as vulnerable or select an upgrade.

One fresh-context `tokio_skill_use` planning/review exercise applied the revised
skill to a proposed KE integration, with prompt/grader frozen before execution.
The task agent was instructed to exclude this record and sibling researcher/
judge/grader artifacts. Its supplied references explicitly teach this case, so
this is **one development/regression exercise**, not unseen holdout or measured
uplift. One delegated timer-source helper supported the same case; it adds no
independent trial. Shared filesystem access is not an isolation boundary.

The complete answer met the eight frozen semantic expectations: it distinguished
current code from the proposed worker; found driver shutdown and nested-pool
progress dependencies with the finite-deadline qualification; required fresh
strict eligibility at the real receiver; preserved native/result custody; kept
the allowed synchronous route and two explicit timer handles; identified the
lab's count-only transfer gap; and proposed discriminating real-consumer cases
with healthy neighbors and existing holds. No product behavior was executed.
The agent reported reading Implement Rust, the Tokio/native-boundary references,
shared contracts, relevant architecture/task and actual producer/receiver/timer
source. Its path register is reported reading evidence, not a complete
independently authenticated instruction trace.

The exercise also reported direct source searches without Serena/Probe, despite
the broader skill's configured navigation instructions. Thus it establishes the
scoped semantic result, **not complete workflow compliance**. Unrelated pure-Rust
trigger behavior, old/new comparisons and runtime/native correctness were not
exercised. No semantic repair or broader campaign follows from this result.

The frozen prompt SHA-256 is
`7fc7bef21f5b3f93e00df627df3afe090756537a36636e060c2b41af1919210a`;
grader `cc17603e488b6031f1499c9f9ca8c628c494d2f7c9e57e343977132372b37cb3`;
answer `b333bd62efc5ebc02b978c2ff264d5c46415aa9f58bc949c4a81df2a8108da16`;
Tokio reference `47275f0f000531bc16a3b77f01e51e37c3d026793fe689506edaa1d1cd204efa`.
The final publisher binds this receipt-only delta to the unchanged six judged
instruction/task files, rechecks the final tree, affected links and whitespace,
and verifies remote `main` after the authorized update. The publication commit
supplies durable final candidate identity. No product build, runtime experiment,
native, formal, fuzz or benchmark campaign was run for this documentation slice.

### Tokio Windows source follow-up — 9 November 2026

The user's Windows compatibility/hidden-bug question extends the authorized
Tokio guidance work. Baseline is published `main` at
`913a6e8d8cb72c1ce80a419a84dba71aee6a83c2`, initially clean. Three bounded
read-only research lanes covered runtime/time, networking and upstream fixes/
advisories; a separate judge independently checked selected sources, current
callers and existing owners. No numerical confidence is inferred from agreement.

The integrated change preserves the runtime/pins and current S2.2 contract-freeze
priority. It adds the selected IOCP/QPC resume-progress qualification to W11-W02,
and the conditional concurrent-poller TCP readiness case to the existing KE
driver choice. Final validity and timely wakeup remain distinct. Conditional
thread-creation failure stays at T-R-02; unused ancillary, pipe and foreign-waker
APIs stay in the dated existing OS review, not the reusable skill inventory.
That review retains all ten current three-package RustSec exclusions and the
remaining native evidence; it is no whole-graph audit or bug-free assurance.

The judge corrected the upstream lane's socket2 #673 disposition: maintainer
reproduction and merged #674 substantiate the alias violation, but the fix is
absent from 0.6.5. Both Miri models are upstream modeled evidence, not a Windows
failure; current engine callers remain excluded. The scratch lane was corrected
before integration. The two principal findings survived source/caller challenge;
no runtime, dependency, feature, timer workaround or native experiment was adopted.

Serena/Probe and context-mode were not exposed in this session. Focused direct
source reads were used as the documented fallback; a failed exact-version
Context7 query supplied no evidence. Microsoft Learn/pinned upstream sources
support the material rules. The existing skill routing is unchanged; only its
conditional Tokio reference and source register are extended. Validation and
integrated-review results follow on the final documentation candidate.

The complete seven-file candidate was frozen as validation commit
`2db1f0c54048f3c8d69078323d32b0ed07652f94`, tree
`4efa7b49aeffdf07750d8410016f13939cddea22`, parent `913a6e8d`.
A separate clean materialization matched all 1,991 tracked paths, modes and
raw blob bytes; repository attributes introduced no transformations. Its skill
checker passed all 66 packages and the matching skill-creator validator passed
Implement Rust. All 704 local links/anchors across the seven changed Markdown
files resolved. The initial heading-only link check flagged an existing explicit
HTML compatibility anchor; verifying that anchor resolved the checker limitation
without a repository link change. Staged whitespace checks passed.

The independent judge read the complete seven-file diff, full revised Tokio
reference and relevant canonical context. Its integrated content verdict was
**READY**, with no required content fixes. The two decision-comprehension
paragraphs were:

> I would preserve the original deadline and fresh control checks at the actual pilot/series publication boundary, while treating post-resume execution latency as a separate unanswered question. When W11-W02's native gate permits it, I would compare an ordinary successful exchange with a quiet exchange suspended across expiry, recording the deadline, actual notification, first owner execution and refusal/retirement before a deliberately added wake could mask the delay. A short-suspend case must test generation invalidation even when expiry has not occurred, followed by fresh reacquisition. Same-driver Sleep controls, paused Tokio time and the 30-second laboratory cap would supply neither that evidence nor a service-stop bound.

> I would freeze the actual TCP/TLS poller, reactor, resolver and synchronous-verifier ownership. An active Runtime::block_on on the same current-thread runtime is necessary for a worker using Handle::block_on, but the resulting concurrent polling arrangement also makes the Windows readiness concern relevant. At the admitted selected-adapter checkpoint, I would compare healthy complete KE with controlled partial/paused reads reaching WouldBlock while writable, observing CPU, useful progress, controls and retirement under an independent watchdog. A same-owner comparison remains a bounded fixture control; it does not authorize blocking certificate verification on the production control thread. The complete synchronous route retains its own permits, deadlines, actual completion and stop obligations.

This is a reviewed document/decision-comprehension result, not an unseen skill
trial, measured uplift or executed product qualification. The judge ran no checks;
the validations above are publisher observations. The final publisher binds this
receipt-only addition to the unchanged six judged guidance/task documents,
rechecks the final tree, links and whitespace, and verifies remote `main` after
the already-authorized commit. The publication commit supplies the durable final
identity. Native progress, stop, resources and service behavior remain unqualified;
all existing execution holds and post-V1 formal deferral are preserved.

## Rust 1.100 feature contract skills — 13 November 2026

David requested updating the skills to make agents use the reviewed Rust 1.100
features correctly, with explicit contracts and prevention of coding mistakes.
This slice changes instruction/routing/source records only. The current NevroTick
base is `8251880a7223e5aa591a317c51196c13316fb7f6`, tree
`af5b394579ac582d3a315c6faa8e885039de6e1e`; all 2,082 snapshot blobs matched their
Git object hashes before edits. Rust source is
`0ef1bc386059596dada8fb65b50425aab2ea637a`, Cargo source
`3acc79c0c841844675bc53a605922dbfc441e750`. The 9 November survey remains historical.

| Acceptance | Required result | Owner / environment |
| --- | --- | --- |
| SKQ-R100-1 | Discoverable feature rules preserve acquisition/release order, typed failure, mandatory parsing, fallible allocation and shared identity; correct the allocator stability boundary. | Implementation editor; source and pinned API review. |
| SKQ-R100-2 | QA rules distinguish Cargo/rustc/Clippy, preserve exact inheritance and feature carriers, and require intended diagnostics with valid neighbors; proposed denies are not claimed as current policy. | QA editor; source review. |
| SKQ-R100-3 | Integrated pack/link/whitespace checks and bounded fresh-context instruction-use review, with actual outcomes and limits recorded. | Coordinator / independent reviewers; Python standard library and read-only review. |

The shared [feature reference](../skills/implementation/implement-rust/references/rust-1.100-contracts.md)
is routed from Rust implementation/contracts and the skill index. The
[lint reference](../skills/testing/references/rust-1.100-lints.md) is routed from
static QA and Cargo operations. Preserve existing task ownership and native,
fuzz, storage, UI and post-V1 formal holds. No product source, dependency graph,
toolchain, Cargo policy or checker implementation is changed by this slice.

### Completed instruction and package checks

The nine guidance/routing/source files were frozen with SHA-256
`e5463294601a726d9342171513914864e3f7a2ad64e491c361839347eb862a71`
over their sorted path-to-SHA-256 map (compact JSON); this evidence record is
excluded to avoid a self-referential digest. The instruction-use exercise input
has SHA-256 `fabc166e66e5efabda9c76e421702c2164be387c3490b6d0cccc99955fc7cae4`.

- The inspected standard-library-only package checker ran as
  `env -i PATH=/usr/bin:/bin LANG=C.UTF-8 python3 -I -B .agents/skills/scripts/check_skills.py`:
  **PASS, 66 skills**, metadata, UI prompts, local links and inventories.
- A separate check of all ten changed Markdown files resolved **280 local
  links/anchors**, including explicit HTML anchors; no errors. The no-index Git
  whitespace check emitted no diagnostics (exit 1 denotes differing trees).
  Checker implementation, inventories and metadata schemas were unchanged.
- A fresh independent reviewer inspected the complete diff, current call sites
  and immutable upstream APIs/lints: **READY**, no required corrections. That
  reviewer saw no instruction-use exercise or grader inputs and executed no code.
- One separate fresh-context agent used the three updated Rust/contracts/static-QA
  skills on six groups of proposed changes. It received requirements and raw
  fragments without the coordinator's expected dispositions or earlier reviews.
  Its source-review answers matched the preselected semantic controls below.

| Exercise group | Observed disposition |
| --- | --- |
| Reentrant publication | Rejected guard-before-acquisition and publication-before-release; preserved publication after ordinary work errors and identified the separate concurrent-caller assumption. |
| Parsing | Retained mandatory-prefix rejection; accepted optional single-suffix removal with raw checks. |
| Construction | Accepted bounded fixed-slot `from_fn`; retained typed allocation refusal for receive storage. |
| Shared provenance | Retained `Arc::clone` for the required same-allocation check; rejected mapped equal-looking replacement. |
| End states | Preserved successful Stop and fallible shutdown; treated `Infallible` to `!` as spelling only. |
| Manifest/diagnostics | Rejected Cargo `all`, inheritance opt-outs, dev-dependency coverage overclaim and disabled feature activation; retained coordinated named-rule adoption and existing Clippy selection. |

These are instruction-use regression examples derived from the reviewed failure
classes, not unseen holdouts or an old/new comparison. One successful source-review
exercise proves only those dispositions; no statistical improvement is claimed.
Raw task/report material is transient outside the maintained pack; this record
retains the source identities, actual scope, outcomes and limitations. Final
publication must preserve the nine reviewed guidance bytes and verify the remote
commit's ten-file scope; the publication commit supplies the durable final identity.
SKQ-R100-1 through SKQ-R100-3 are complete for instruction/package scope. No Rust
compilation, native execution, future refactor or release qualification occurred.

## Rust API selection and compact skills — 14 November 2026

David approved an explicit strongest-applicable-contract rule and requested
caveman-compress for the affected skills. Base: `d2c2909a0fe43eeed70c54c1e845779246a6d948`,
tree `457e5a7cd3cf717b733808d50d660a1701206ff6`; all 2,084 snapshot blobs matched
their Git object hashes before edits. Keep the rule in
[Implement Rust](../skills/implementation/implement-rust/SKILL.md), routed from
[Rust contracts](../skills/implementation/implement-rust-contracts/SKILL.md) and
[contract enforcement](../skills/implementation/implement-rust-contracts/references/contract-enforcement.md).
Compress those three files and the existing
[Rust 1.100 reference](../skills/implementation/implement-rust/references/rust-1.100-contracts.md).

| Acceptance | Required result | Owner / evidence |
| --- | --- | --- |
| SKQ-API-1 | Stable/admitted API, method and representation enforce strongest applicable guarantees; preserve legitimate inputs, failure behavior and ownership. Justify necessary weaker forms and identify remaining enforcement; avoid invented restrictions. | Coordinator; approved rule and routing review. |
| SKQ-API-2 | Shorter prose preserves every rule, condition, exception and protected syntax; names, headings, examples, links and metadata remain intact. | Two scoped editors; frozen uncompressed inputs, caveman validator and independent semantic judge. |
| SKQ-API-3 | Final skill pack and changed-file scope pass; publish without overwriting concurrent work. | Coordinator; package/link/whitespace checks and verified remote tree. |

The approved rule/routing additions were frozen before compression: four files,
55,896 bytes and 6,530 whitespace-delimited words. Verified originals stay outside
the skill tree. Compression measurements compare against that updated baseline,
not against the earlier files lacking the new policy. Product Rust, Cargo policy,
dependencies, checker implementations, imported inventories and execution holds
remain outside this instruction-only slice. Validation results follow below.

### Compression and validation results

Final four-file guidance: **54,580 bytes / 6,220 words**, reductions of 1,316 bytes
(2.35%) and 310 words (4.75%) against the updated uncompressed baseline. Compared
with the published files before the new rule, net reduction is 282 bytes and
184 words. These are bytes/whitespace-delimited words, not model-token counts or
measured agent-performance improvements.

The sorted path-to-SHA-256 map (compact JSON) has SHA-256
`ded46fa4927bd3791bfbd5af20990f7ed6c3cbebee6aeb4699ee4114f4b8d4f6`;
this evidence record is excluded. All four final caveman validations pass with
zero errors/warnings, preserving metadata, headings, code spans/blocks, numeric
tokens, links and table structure. The contract editor restored one path-like
prose token's capitalization and potentially ambiguous shortening within two
targeted repair passes. The other pair passed on first validation.

The unchanged package checker passes **66 skills**, including metadata, UI
prompts, local links and inventories. The no-index whitespace check emits no
diagnostics; exit 1 denotes differing trees. All **45 local links/anchors** in
this plan resolve, including explicit HTML anchors. The complete candidate has
exactly the four guidance files plus this existing record changed, no added or
deleted paths; the frozen guidance remains unchanged.

A fresh independent judge read every approved input, complete candidate and diff:
**READY**, no lost requirements, new operational ambiguities or corrective edits.
It independently compared protected syntax and structure and confirmed the same
frozen digest map. This is semantic-preservation review, not an upstream API
revalidation, agent-performance experiment or product test. SKQ-API-1 and
SKQ-API-2 are complete. The publisher must verify this five-file remote tree and
advance the unchanged base without force for SKQ-API-3; the publication commit
supplies final identity. No Rust compilation, native execution, formal job or
product-compliance claim follows from these instruction checks.

## Contract and integration hardening guidance — 14 November 2026

David requested applying the reviewed function/module hardening methods to the
appropriate skills and preparing a standalone Markdown handoff for his
implementation agent. Source baseline: `e72ad375d7c1d015c91c222ab9cf50dfe6470a29`,
tree `0c3434e3a2988e58738258600800f9e2c0ee7b12`; all 2,084 snapshot blobs matched
their Git object hashes before edits. This slice updates four existing guidance
files and this evidence record. Product implementation remains delegated.

| Guidance owner | Added enforcement emphasis |
| --- | --- |
| [Contract enforcement](../skills/implementation/implement-rust-contracts/references/contract-enforcement.md) | Preserve typed units through helpers and witnesses; bind admission and consumption to actual owners/values; retain historical facts without renewing authority; make semantic field additions force equality review; narrow worker kernels; represent combined termination/cleanup observations; refine phase-owned data only with a justified transition/custody map. |
| [Implement Rust](../skills/implementation/implement-rust/SKILL.md) | Route cross-module handoffs and equality-sensitive structural changes to the maintained contract rules. |
| [Contract QA](../skills/testing/qa-contract-tests/SKILL.md) | Challenge the intended production privacy scope, real producer/adapter/consumer composition, complete state oracles, wrong-but-well-typed inputs, and legitimate neighboring behavior. |
| [Execution evidence](../skills/implementation/implement-rust/references/execution-evidence.md) | Prove the actual feature-selected implementation and consumer ran; retain default coverage; distinguish local runner support from hosted invocation; check the integrated final candidate. |

These additions preserve stable/admitted API selection, required failure
semantics, intentional extension points, spent resources, unknown-effect custody,
and existing task ownership and execution holds. They do not prescribe blanket
validation annotations, typestate conversion, trait bans or all-features testing.
The handoff contains nine scoped work items with source evidence and acceptance
checks; conditional UI/ledger work and native/formal holds remain explicit. It is
a separate conversation deliverable, not another project qualification ledger.

### Compact guidance and package checks

The semantic additions were frozen before compression outside the loaded skill
tree: **49,496 bytes / 5,859 whitespace-delimited words**. Final guidance is
**48,747 bytes / 5,697 words**, removing 749 bytes and 162 words (2.77%). These
measurements compare the expanded and compact updated instructions; they are not
model-token counts or measurements of agent effectiveness.

The final ordered file-record array, serialized as compact JSON with sorted object
keys, has SHA-256
`9d8ccb980f00c8feb3fa0ef5fca417ca31ed357df2572e5797d6a94b46c9ec12`.
Records contain each guidance path, SHA-256, byte count and word count; this
evidence record is excluded. All four final caveman validations pass with zero
errors and warnings. Each editor used one targeted repair pass for path-like
prose tokens; protected syntax and material meaning were preserved.

The unchanged `check_skills.py` passes **66 skills**, covering metadata, UI
prompts, local links and inventories. The current skill-creator `quick_validate.py`
passes the three affected skill packages. A fresh independent judge compared all
four final files with their expanded inputs and published baseline, checked the
shared contracts and routing, and verified the frozen hashes: **READY**, with no
material required correction. This was source/semantic review, not execution of
product code.

### Fresh instruction-use exercise and handoff review

A separate fresh-context reviewer received only the updated skills and three raw
synthetic review cases, without the coordinator's intended answers, earlier
conversation or the independent judge's notes. Its source-review output correctly
identified the following bounded controls:

| Case | Observed disposition |
| --- | --- |
| New authority field omitted from manual equality | Required explicit field comparison and exhaustive patterns; rejected direct derived equality where it would lose required floating-point bit semantics; retained independent effect observations and a legal control. |
| Broad worker helper and an external E0603 fixture | Distinguished outer-module privacy from production sibling access; required one private kernel, narrow test forwarding and a negative caller at the intended production scope. |
| Optional implementation omitted by default CI | Required separate default and admitted feature rows, actual implementation/consumer selection and real hosted job execution; rejected zero-job startup failure as execution evidence and blanket all-features replacement. |

The exercise involved no compilation or test execution. It checks those three
instruction-use examples only; it is not an unseen holdout, old/new comparison,
statistical evaluation or measured defect-reduction claim. An additional
independent reviewer checked the complete handoff against current source and
owners: **GO**, no material revision required.

Final publication must preserve the four frozen guidance files, verify exactly
these files plus this record in the remote diff, and advance the unchanged base
without force. The resulting commit supplies the durable publication identity.
Product Rust, manifests, lockfiles, CI configuration and checker implementations
are unchanged. No Rust build, product test, native probe, formal job or release
qualification was executed by this guidance task.

## Explicit-contract instruction continuation — 14 November 2026

David accepted the targeted follow-up methods and requested updating the skills
and appropriate documentation. The repository advanced before this edit to
`9cbdc249e8cf43aa5399386a8a491d183f3d650d`, tree
`032b544277c45d828d49ded06f889dda1b5ec3f9`. All 2,109 snapshot blobs matched their
Git object hashes before editing. Three files above the contents endpoint's
inline limit were fetched through their actual blob objects and verified; an
empty contents response was not accepted as source. The connector snapshot has
no local Git index, so this record makes no local staged-tree claim.

This authorized documentation slice modifies five existing guidance files,
the development-hardening policy, development-plan routing, the existing Engine
handoff and this record: nine existing Markdown files, with no new repository
paths or product/configuration changes. The separate implementation handoff keeps
its [existing identity](https://chatgpt.com/library?file_id=libfile_3efc95be08ac8191a4e8400df7321194&account_id=personal)
and is updated for current H01–H09 status plus scoped H10–H16.

| Guidance owner | Refinement |
| --- | --- |
| [Rust entry](../skills/implementation/implement-rust/SKILL.md) and [contract enforcement](../skills/implementation/implement-rust-contracts/references/contract-enforcement.md) | Important outcome types across wrappers; deliberately absent capabilities at the actual owner; fresh sequential observations; conditional narrow admission leaves. |
| [Contract testing](../skills/testing/qa-contract-tests/SKILL.md) | Intended diagnostic plus legal consumer; separate trait controls without incidental masking; meaningful observations across transitions. |
| [Independent review](../skills/testing/qa-independent-review/SKILL.md) | Accepted-baseline comparison of verification meaning and review of new/widened lint suppressions, preserving legitimate rebinding and scoped generated-code exceptions. |
| [Role API restrictions](../skills/testing/references/role-api-restrictions.md) | The two elapsed methods as indirect current-clock observations; independent supplied-timestamp refusals, explicit-input and permitted-adapter neighbors. |

The [canonical follow-up](../../tasks/engine-transport-pilot.md#explicit-contract-prevention-follow-up--14-november-2026)
keeps H10–H13 targeted to existing owners. Suppression review applies now;
automated admission, the retry seam and FitPoint leaf move remain conditional at
their next owner edits. No current caller defect, implemented elapsed ban or new
product test is claimed. Current task records supersede the old handoff's blanket
native stop: bounded Core/Engine main-Windows tests are authorized, with all
stated system-effect exclusions and unrelated holds retained. H08's Actions
trust proposal remains pending; H09 source routing is implemented, hosted
acceptance open. This slice changes no product stage-only instruction.

### Compression, independent review and checks

Full expanded intended files were frozen outside the auto-loaded tree before
in-place caveman compression. Final totals are **62,684 → 61,222 bytes** and
**7,437 → 7,154 words**: 283 fewer words (about 3.8%), not a tokenizer measurement
or reduction relative to the smaller pre-update policy. All five final validator
pairs pass with zero errors and warnings. Frontmatter, protected code, identifiers,
links, headings and structural invariants are retained.

The first implementation compression check produced prose-path classification
warnings, repaired locally. An independent full semantic review then required
two scope repairs: preserve the prohibition on changing real transition semantics,
and retain the judgment that a descendant-construction risk must warrant a
stronger leaf boundary. Both were repaired within the bounded compression process,
revalidated against the unchanged expanded inputs and independently rechecked.
Final semantic disposition: **READY**, no remaining required repairs.

Final guidance SHA-256 identities:

| File, relative to `.agents/skills/` | SHA-256 |
| --- | --- |
| `implementation/implement-rust-contracts/references/contract-enforcement.md` | `4b3430b40375be96713df1beca49ea2ccab86e84048b4150e5ba49264eef9830` |
| `implementation/implement-rust/SKILL.md` | `1eaf52ff7e8f9d6638cecb1fcba0731412724b8ea33240af081c1c3d60bf2016` |
| `testing/qa-contract-tests/SKILL.md` | `710d08dc4cb7315070e831829d213e768dbcf2a638fcd5dbe9a9ae82193ae02e` |
| `testing/qa-independent-review/SKILL.md` | `3939b5aa598cc2b5b210288bf289b0600e887078b2fd2f5d76dc4193f221e0dd` |
| `testing/references/role-api-restrictions.md` | `6efa179d31b4358f8c0232b837328f84c8be33f683202702b3e529395b0e4be0` |

The package checker passes **66 skills**, including metadata, UI prompts, local
links and inventories. Skill-creator validation passes all four changed entrypoint
packages. An earlier author-side package check overlapped a canonical-document
write and reported a missing unchanged §9.11 anchor; the settled integrated check
passes without changing that anchor or checker. No checker implementation or
inventory contract changed, so no new checker regression campaign was required.
All 778 local links/anchors across the three changed canonical documents and
this existing record resolve. No root `docs/SHA256SUMS.txt` is present; existing
archived design-bundle manifests cover none of these changed documents.

A fresh-context instruction-use reviewer received four raw synthetic cases and
the skills, without the coordinator's expected diagnoses or other review notes.
It correctly distinguished the discarded wrapper outcome, masked combined/outer
trait fixtures, an elapsed fixture masked by forbidden `now`, and acceptance
weakening disguised as hash rebinding. It retained legitimate copied observations,
current correct types, explicit-input/adapter behavior and a genuine hash-only
neighbor. This was source review, not executed compiler evidence. The exercise
used the initial compact candidate; the subsequent two scope-wording repairs
were independently reviewed and did not change those case mechanisms. These
four practice cases are not an unseen holdout, old/new comparison, statistical
evaluation or measured reduction in agent defects.

A separate source-based document judge reviewed the full revised handoff and
canonical additions: **READY**, no required repairs. Its optional conditional-
seam wording was made uniform locally. Current-source and owner reconciliation
retains scoped historical product identities, pending H08/H09 hosted acceptance,
conditional work and current native boundaries. Final delivery replaces the
handoff's publication placeholder with the verified commit and pins its new
guidance links to that same commit.

Before publication, verify the exact nine-file remote diff, unchanged product,
configurations and historical receipts, current branch head and final guidance
hashes; advance the expected parent without force. The resulting commit supplies
the publication identity. No Rust compilation, product test, native execution,
formal job, CI settings write or release qualification was performed by this
instruction/documentation task.
