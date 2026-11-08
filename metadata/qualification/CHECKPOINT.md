# Qualification restart checkpoint — 7 November 2026

## Current owner amendment — 8 November 2026

David explicitly requested fixing the seven-quote metadata defect. The finite
repair is now implemented: the lexical scanner consumes at most two additional
matching quote characters after a multiline closing triple, preserving parser
syntax ownership and all budgets. The original regression remains unchanged;
all 247 current unit tests pass, with separate healthy/hostile lexical-depth,
quoted-bracket, escape and invalid-syntax controls. Strict typing and four
compile-valid causal fault controls pass. Fresh coverage completes all 247 tests,
observes all 283 CLI children, excludes 14 altered source-file observations and
passes every declared 90% security-module line gate. The repaired contract is
580/636 lines (91.195%), with 307/360 branches reported separately. The first
600-second attempt remains incomplete; the unchanged-source retry completed in
670.237 seconds under its finite 1,800-second bound. Exact results and final
publication are recorded in the
[existing W11-W10 task](../../tasks/windows11-integration.md#w11-w10-re-enter-conditional-mechanisms-only-for-a-concrete-need).

The historical evidence directory named below is absent as of 8 November.
Its raw typing, coverage, 641 closed mutation outcomes and interrupted jobs
cannot be freshly admitted, reused as current outcomes or resumed from the
missing checkpoint. Fresh repair records use the existing coordinator chain's
`phase-plan-20261108-v1/metadata-preflight-repair-v1` directory. Full per-file
95% mutation qualification remains open; bounded causal controls add no score.
The owner reopened this repair and its affected verification, not an automatic
rerun of the entire historical 8,890-variant campaign. All native/package,
formal, fuzz, storage and UI holds remain.

## Historical snapshot — 7 November 2026

**Status: stopped for resource pressure; full qualification is incomplete.**
David asked to stop when PC resources were constrained and to resume only when
he starts the work again. At the checkpoint the WSL filesystem was 96% full
and the load average was 13.25. Memory was not exhausted: about 7 GiB remained
available. Only this qualification's owned jobs are stopped; concurrent product
work and its staging are preserved.

Continue through the existing
[W11-W10 task](../../tasks/windows11-integration.md#w11-w10-re-enter-conditional-mechanisms-only-for-a-concrete-need)
and [metadata plan](../../docs/plans/windows-package-metadata.md#137-full-portable-source-tool-qualification--7-november-2026).
The evidence root is:

```text
/tmp/nevrotick-metadata-qualification-20261107-zu2t_iqg/
```

Preserve that directory until the unfinished evidence is reconciled. It contains
frozen candidates, exact identities, first failures, raw outcomes, compact
coverage data, reviewed exclusions and the mutation stop/resume inventory.
The original checkout baseline was `e5d9a0112e198ab41a50ee0b0057e06eab68ab84`;
HEAD alone does not identify the modified candidate. Use the source/test/input
hashes in `source-freeze-corpus-v4.json` and the later stopped-file records.

## Historical completed evidence

- Strict mypy checking passed all eight production modules and a legal consumer;
  five intended negative controls discriminated. Tool custody and configuration
  are recorded in `typing/qualified4/result.json`.
- Frozen corpus v4 passed 231 tests. Corrected, independently controlled coverage
  readback passed every security-module 90% line threshold:

  | Module | Executed lines | Branches |
  | --- | --- | --- |
  | `__main__` | 42/46 | 9/10 |
  | `contract` | 571/631 | 298/354 |
  | `delivery` | 275/296 | 153/172 |
  | `generate` | 357/376 | 119/136 |
  | `text_policy` | 39/39 | 20/20 |
  | `fields` | 17/18 | 1/2 |

  All 275 CLI children were observed. Fourteen altered source observations were
  excluded. Both package markers have zero executable statements. The passing
  result is `coverage/reconciled-full03-v4/coverage-result.json`; previous
  collector/report failures remain preserved and were not rewritten as passes.
- Actual frozen-source validate/generate/check succeeded; release mode refused
  the unfinished release. Independent readback checked all 15 paths, source
  identities, role licences, v3 target and public/private separation.
- Later tests caught exact surviving CLI, quote-scanner and read-budget faults.
  Their supplemental results do not replace the raw full-campaign outcomes.

## Historical required next work

1. **Fix the demonstrated healthy-source defect first.**
   `test_preflight_contracts.PreflightContractTests.test_seventeenth_container_after_single_literal_quote_is_refused_before_parser`
   retains the seven-consecutive-quote regression. Valid triple-basic and
   triple-literal TOML can leave the lexical scanner in the wrong quote state;
   the seventeenth container then reaches `tomllib` before the decoded-depth
   guard rejects it. The early resource guarantee fails. Do not skip or weaken
   this test. Read `contract-regression-tests/HANDOFF.md` and its preserved RED
   witness before repairing the quote scanner.
2. Re-freeze the repaired source and run affected static, ordinary, coverage and
   mutation evidence. The old source-bound results do not automatically qualify
   the repair. The current live corpus has 244 explicit methods in 12 files;
   it is larger than the completed 231-test coverage cohort and includes the
   known RED above. No complete current-corpus GREEN is claimed.
3. Resume or regenerate mutation work from `mutation-full-v4`'s stop checkpoint.
   Its inventory contains 8,890 default faults. The stop retained 641 closed
   outcomes after 44 minutes 18 seconds, with 8,249 pending, including ten
   interrupted active faults. Five timeouts are separate unresolved outcomes.
   See `mutation-full-v4/checkpoint/checkpoint.json`. Full execution and the 95%
   per-file qualification are unfinished. The saved resume command targets the
   historical v4 source only; fix/reconcile the defect before using it as current
   evidence. Check exact source, test, tool and
   configuration identities before reusing completed outcomes. Preserve partial,
   canceled, timed-out, syntax, startup, killed and surviving outcomes separately.
4. Reconcile the exact supplemental kills, reviewed equivalences and remaining
   survivors. Relevant records include `fields-independent-dispositions.json`,
   `cli-contract-sensitivity/`, `mutation-uncertainty-replay-v4/`,
   `contract-survivor-review/`, and `contract-regression-tests/`.
5. Integrate the independently observed effective-nonblocking FIFO assertion
   described in `parser-progress-review/REPORT.md`, then replay the four exact
   faults. The checksum exponent mutant's excessive intermediate arithmetic and
   three EOF-probe variants remain unresolved. Timeouts receive no kill credit.
6. Obtain final independent reconciliation against the repaired integrated
   candidate and update the existing W11-W10 receipt. Do not report a full
   mutation score or full qualification before these obligations close.

MD-P4–MD-P5, actual compiled resources, native Windows, installed/Store artifacts
and release approval remain later work. This checkpoint resumes no formal jobs
and lifts no native, fuzz, storage or UI execution hold.
