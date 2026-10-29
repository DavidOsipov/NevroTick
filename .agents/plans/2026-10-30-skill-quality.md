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
