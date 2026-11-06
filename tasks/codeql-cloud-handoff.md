# Cloud continuation: Rust data-flow audit

Owner request: move the reviewed work into the active local `main` checkout and
hand unfinished CodeQL analysis to a cloud AI. **Do not resume local analysis.**
This continues the existing [GUI audit/evidence record](slint-gui-handoff.md#rust-data-flow-audit-and-text-range-lock-repair--6-november-2026),
not a new product implementation plan.

## First message to the cloud AI

> Continue the Rust data-flow audit in this repository using
> `.agents/skills/testing/qa-data-flow-analysis/SKILL.md`. Read this handoff,
> `AGENTS.md`, `docs/README.md` and the testing contract first. Preserve other
> workers' changes. The AccessKit deadlock repair is already committed; the local
> CodeQL runner, portable queries and evidence summary are in this checkout.
> Qualify an isolated cloud environment with adequate measured resources, freeze
> the exact candidate, then complete the selected data-flow questions and their
> independent controls. Investigate unexpected paths in actual callers, reproduce
> credible defects behaviorally and fix only validated issues. Report incomplete
> stages honestly; inventories and empty results are not security clearance.
> Keep formal proof work, Windows execution, fuzzing and storage holds intact.

## Candidate and code already handled

- Signed commit **`bd1f7c4a44e4a8dcf31fa6ca650b3a0cea1dd83e`** is on local
  `main`. It repairs inherited AccessKit same-range comparisons that could acquire
  one read lock twice while a writer waits. Both native comparison consumers use
  the same alias-aware helper; distinct operands retain simultaneous guards.
- Its regression distinguishes the old two-read sequence from the repair;
  eleven portable ingress tests, strict Clippy and the UI text gate passed.
  Native Windows COM/concurrent-mutation qualification remains open. See the
  existing audit record for the exact scope and Python check counts.
- The subsequent runner changes select one Cargo compilation job, a 4096 MB
  CodeQL RAM hint, 1800 CPU seconds and 4096 descriptors. The shared runtime's
  default remains 256 descriptors. They retain twelve analyzer threads,
  8 GiB per-process address space, 128 tasks, 300-second MCP requests,
  600-second extraction/finalization, offline execution, immutable tool/library
  pins, the tool allowlist and source/alias guards. A bounded finalization command
  reuses retained extraction after an import failure; it does not rerun Cargo.
- **56 focused Python controls passed** for the runner/resource/argument changes.
  These tests qualify those controls, not product data-flow or native behavior.
  The proposed 6144 MB/auxiliary-JVM adjustment was interrupted before adoption;
  it is **not** the checked-in or qualified profile.
- Another AI owns unfinished rational-seconds/engine changes in the active local
  working tree. They were excluded from the frozen analysis. Preserve them;
  inspect current Git state and manifests before choosing the cloud candidate.
  Existing local dirty work is not automatically present in a remote clone.

## What local CodeQL actually established

The local CLI was **2.27.3**, `codeql/rust-all` **0.2.24**, Rust **1.99.1**.
The library source pin is
`a1575b98a8cc19bcfa7bb6cb41be688cb3dae65c`.

The frozen **`bd1f7c4` product projection** retained 343 exact Git files, including
227 Rust sources, all six workspace crates, both local derivatives and their
build/resource inputs. It omitted unrelated tool/formal/design workspaces;
no Cargo selection was changed. Its source-manifest SHA-256 is
`d30a884b739a9a6d829f0b4814119cc2dfc22334322b349dd5107161141149a6`.

| Stage | Actual result |
| --- | --- |
| Extraction and finalization | Passed after runner corrections; successful import took about 90 seconds and peaked at about 3.2 GiB RSS. |
| Function inventory | 16,699 result rows; complete decoding to a file succeeded. Includes tests and generated declarations, not 16,699 independently reviewed functions. |
| Source/sink selectors | All **12** rows exactly matched independently prepared expectations. |
| `LocalFlow.ql` | Compiled, but evaluation failed with **ENOSPC** in derived query cache. No admitted flow result. |
| `GlobalFlow.ql`, `GlobalPaths.ql` | Not run after the failed prerequisite. |
| Broken-selector controls | Prepared and reviewed; not run on this candidate. |

Host/implicit-all-features/test-cfg extraction is not Windows qualification. Windows-only
modules, archived native modules and some macro expansions were unavailable,
including a Tokio `select` expansion. Inventory membership alone does not prove
all call targets or generated/native effects were modeled.

A separate hosted scan of predecessor `2685981` used CodeQL 2.27.4 and returned
16 detections. Source review found test fixtures or nonce buffers fully overwritten
before use; no actionable vulnerability was established from those detections.
That is separate evidence, not a successful local product flow scan. The AccessKit
finding came from independent source/concurrency review, not a CodeQL alert.

## Portable material and independent expectations

Use [the query bundle](../tools/codeql-audit/README.md). It includes exact authored
queries, pinned dependency closure, [expected rows](../tools/codeql-audit/queries/expected.json)
and a [compact evidence summary](../tools/codeql-audit/evidence-summary.json).
No local database, library binaries or vendor directory are bundled.

For the original candidate:

- `Selectors.ql`: exactly twelve endpoint rows.
- `LocalFlow.ql`: exactly `ui_raw` and `engine_bytes`; `ui_field` and
  `engine_slot` absent.
- `GlobalFlow.ql`: exactly `replace_raw`, across
  `UiDrafts::replace → RawText::parse → to_owned` receiver; `replace_field` absent.
- `GlobalPaths.ql`: inspect the same global endpoint pair and reconcile the path
  with the table result. This is the same question, not another guarantee.
- Broken queries: empty results; comparing them against the healthy expectations
  must fail. Contradictory selectors may be optimized away, so this establishes
  selector/oracle sensitivity, not execution of a faulty program or complete
  qualification of the data-flow engine.

No custom sanitizer or additional flow step was invented. Validation is not a
barrier for expected value propagation. Field/slot negatives concern value flow,
not control dependence or authorization. Tooling/documentation-only commits may
retain the same product inputs: verify byte/configuration equality before reusing
these facts. If relevant inputs change, derive new endpoint facts from source
before execution; never learn expectations from an observed empty result.

## Cloud execution sequence and finite exit

1. Inspect the actual cloud environment and applicable environment skill. The
   local launcher assumes a separately installed WSL tool store and installation
   records; its absolute paths do **not** exist automatically in the cloud.
   Qualify equivalent isolation/admission with the pinned tools before executing
   Rust extraction: `--build-mode=none` can still run build scripts/proc macros.
   Keep credentials/network outside the execution boundary, source immutable,
   dependencies/features/licences admitted and outputs privately owned.
2. Freeze one candidate and its manifests, lockfile, query/model bytes, target,
   features/cfgs and expected controls. Keep custody hashes outside writable
   analyzer output. Rebuild rather than silently reuse a stale database. A prefix
   match or pre/post finalization snapshot does not establish historical database
   freshness; bind any reused extraction to its original protected source receipt.
3. Preflight RAM and scratch storage. Local failure modes were compiler-thread
   exhaustion, importer heap exhaustion, descriptor exhaustion, stdout buffering,
   then query-cache disk exhaustion. These were tool/environment failures, not
   product counterexamples. Reuse the corrected compilation and descriptor
   settings rather than replaying the failed bootstrap configuration.
4. Run the twelve selectors first, then local/global flow and discriminating
   controls, one query at a time. Retain exit codes, BQRS metadata, complete decoded
   rows, query hashes, paths and exclusions. Write large decoding output to a file
   or paginate; the full inventory exceeded the tool's stdout buffer.
5. For a credible unexpected path, inspect the actual producer, checked result,
   state/guard ordering and consumer. Challenge a competing explanation. Require
   an independent behavioral RED, healthy neighbor and smallest causal GREEN
   repair. Resume broader packet-admission, catalog/publication, state and output
   questions through the existing data-flow skill after this pilot works.
6. Stop at completed selected questions or a documented bounded environmental
   failure. Do not substitute helper checks, repeated setup loops, source-only
   inventories or a favorable count for the requested product result. Record
   remaining work in the existing audit chain, and report the exact candidate.

The 4096 MB hint yielded only a **1800 MB query-worker heap** locally, below the
pinned CLI's 2 GiB stage-cache threshold. Logs showed cache churn; derived cache
reached about **4.3 GiB** and temporary demand exhausted free space. A larger cloud
RAM hint/worker heap is a **proposal requiring environment qualification**, not a
previous pass. Keep auxiliary JVMs distinct from the analysis worker, verify
actual heap/RSS and retain finite wall/CPU/process/file budgets. Any cloud-specific
profile change needs its corresponding policy/guard controls.

`--max-disk-cache=1024` is a possible cache-trimming setting, **not a hard quota**:
essential live pages can exceed it. `--min-disk-free` is ignored when an explicit
maximum is set. Monitor actual free space and retain a stop reserve; do not wait
for ENOSPC again. The [official query-run options](https://docs.github.com/en/code-security/reference/code-scanning/codeql/codeql-cli-manual/query-run)
and [cache cleanup modes](https://docs.github.com/en/code-security/reference/code-scanning/codeql/codeql-cli-manual/database-cleanup)
explain these controls.

## Local checkpoint and preservation

All local analysis jobs were stopped. The failed derived cache was removed after
verifying all 958 imported extensional-relation files stayed unchanged. The copied
vendor cohort was verified after use and removed; original vendor files were
untouched and temporary Cargo configuration was restored. No unrelated cleanup,
Windows product run, formal job, fuzz campaign or alert dismissal follows from
this handoff.

Raw local evidence remains under
`/home/david_osipov/.local/share/nevrotick/codeql-work/results/product-bd1f7c4/`;
the completed database is under `codeql-work/databases/product-bd1f7c4-serial`.
These are local references, not cloud inputs. Rebuild in the cloud unless David
separately transfers and you re-admit them. The portable summary preserves the
key results; do not copy stale caches or require a multi-gigabyte upload merely
to continue.

OD-10's post-V1 formal deferral and all existing Windows/fuzz/storage/UI/package
holds remain. This audit grants no production sample eligibility, clock authority,
Windows enforcement, complete security certification or release approval.
