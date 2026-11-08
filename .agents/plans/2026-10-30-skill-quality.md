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
