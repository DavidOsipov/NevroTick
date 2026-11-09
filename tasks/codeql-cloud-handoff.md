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


## Local text-inventory and analyzer-output repairs — 7 November 2026

David supplied two additional tooling findings and authorized continuing the
issue-repair workflow. The quoted `/workspace` handoff at line981 is not present
in this checkout: its tracked predecessor has175 lines. Its quoted four-open count is
not adopted as a current local status. This checkpoint independently checks the
two named paths. The preceding AccessKit source repair and BH-TOOL-01 have their
own current records; native qualification and the other existing holds remain.
No new CodeQL scan, Rust/analyzer campaign, formal or Windows execution occurred.

Owners: GUI localization/source inventory under OD-15 and the source-preparation
subset of CHECK-22/25; verification admission and independent evidence under
CHECK-24/25. Pinned Linux/WSL Python3.12 is the required environment. Independent
expectations are rejection of missing/unowned live text and controlled incomplete
reporting for invalid/linked/over-budget output, with hashes of the same admitted
bytes. CHECK-20/26 source/graph obligations and every native/product gate remain.

**Slint inventory — confirmed and repaired.** `_nodes` only examined200 characters
before an opening brace. Longer whitespace made a live native control typeless,
losing its text-binding custody. An actual full509-ID source snapshot first
passed healthy; three injected, long-spaced Text variants (missing/unowned/literal
text) also passed incorrectly, producing the intended RED. The retained83-test
run has25 failed subcases, not25 separate product defects. The repair scans the
immediate header tokens backwards under existing source/node/depth/edge limits,
retains named-instance identity and shares that header with literal-false parsing.
It does not rescan the whole preceding source or become a general Slint compiler.

All85 focused text-coverage tests pass, covering long spaces/comments/newlines,
named gaps around `:=`, inheritance/instances, exact literal-false versus dynamic
conditions, long type tokens and inert comments/strings. Source bookends match.
The real `check-ui-text.py --root` entry also passes509 active/509 used IDs,
zero errors, with unchanged semantic fingerprint
`34c97f8a74a9559ae998f55389bedd9f92bb3f878e72df84d090372053a69e80`.
This is source-inventory evidence, not independent translation approval, rendering
or native accessibility qualification. Parser SHA256:
`7e08b0c7f9b881b5e72198a9e4e89562b9d463227d78f944d960fd528f4e48b8`.

**Analyzer outputs — confirmed and repaired.** The real shared analysis-lane
entry accepted linked coverage/outcomes reports after sandbox execution. The
original124-test RED retained exactly two intended linked-file failures and a
healthy coverage/mutation neighbor; no resource-exhaustion experiment ran.
Generated-file reads now traverse directories through no-follow descriptors,
accept only one owned regular inode, bound bytes before reading and recheck
file/directory attachment and metadata. JSON is strict UTF-8, duplicate/nonfinite
rejecting and bounded before decoding:64MiB JSON,16MiB logs, depth64 and4million
structural markers. These are admission ceilings, not measured host RSS/time
limits. Paths are capped at4096 UTF-8 bytes/64 components, with raw strings bounded
before Path construction/opening. Healthy coverage above1MiB remains accepted;
inspected retained coverage reaches4.44MiB, so a1MiB blanket limit was rejected.

The shared lane, dedicated rational/window coverage, Core per-outcome/accounting
and engine/domain/window baseline paths all use this boundary. All three Core
log-hash branches and dedicated baseline hashes derive from the same retained
bytes, without a second path read. Coverage/mutation thresholds, native exits,
required test/doctest counts, descriptors/equivalences and operation reservations
remain enforced. Policy/inventory decoders and deferred formal readers were not
changed. Shared helper SHA256:
`d4693f94f665fffd62a9afd526921a134443f97aa9b835665cfa20fcfafff3b8`.

The final shared suite passes136 tests. A further causal path-budget RED and its
correction are retained. Controller RED showed a healthy baseline alongside the
unwanted second hash read; repaired engine controls pass52 tests including that
case, dedicated baseline controls pass2, and focused Core output/accounting
controls pass4. The first controller integration exposed a missing hashlib import
and remains failed; the import was corrected and the affected cases rerun.
Together with the Slint suite, the final selected sets contain279 distinct passing
Python tests; filtered repetitions receive no additional count. Strict workspace
policy and changed-line whitespace checks pass. Four genuine retained coverage/
outcome reports also pass the new reader; this is compatibility replay, not new
Rust execution or a fresh mutation/coverage score.

**Limits and retained failures.** The broad19-test rational/window suites are not
green: historical374/370-input admissions reject the current377-input tree before
many cases reach their targets. One newly changed rational object-error wording
expectation was separately corrected to the same specific semantic rejection and
rechecked in the passing dedicated tests. All other initial binding failures stay
recorded; no historical recipe or score was rewritten. An overbroad Core suite
was interrupted as incomplete using exact argv/cwd/stdout/PID-start ownership;
the four changed-consumer controls then ran to completion. Independent reviewer
`tooling_findings_review` (initial `fork_turns="none"`) inspected source, causal
logs and final routing, finding no remaining material defect in this bounded
repair. The reviewer ran no jobs and its agreement adds no executions.

Evidence root:
`/home/david_osipov/.local/share/nevrotick/dependency-refresh-20261106-v1/bughunt-text-and-reports-v1/`.
It retains all RED/GREEN/partial logs, process-interruption custody, actual CLI
result and report replay. `policy-binding.json` binds current377-input candidate
`23667e60971adc5d3bc1da4e4dfdcba58d8fc4cb6f400ad103ad192979b36f5d`;
older native receipts retain their original helpers/candidates. Existing staged
work remains separate. No staging, commit or push was performed by this work.
Resume the ordinary product queue; no Windows, fuzz, storage, UI, package or
post-V1 formal disposition is lifted.

Final guard refinement removes generated-path `resolve()` calls before bounded
admission; leaf no-follow checks retain explicit errors and the shared reader
enforces every ancestor. Policy/receipt readers keep their original behavior.
The fresh final baseline controls pass4 cases and final Core controls pass all
four in separate owned processes; the engine suite again passes52. The actual
resolve/hash spies execute their originals if reached, so they are not substitute
oracles. `closeout.json` is the final identity/result record; the earlier
`final-source.json` predates this last guard refinement. Its source values remain
preserved as an intermediate snapshot. The tracked predecessor's175 lines were
verified with Git; the initial custody note's166-line estimate was inaccurate.

The continuation rechecked both repaired production helper hashes against these
retained identities and reran the focused suites with the pinned Python3.12:
all85 text-inventory and136 shared-analysis tests pass (221 distinct tests).
This is a fresh Python regression check, with no resource-exhaustion experiment,
Rust/analyzer campaign or Windows execution. No staging, commit or push follows.

Two further read-only reviews confirmed the current parser/helper hashes above
and found no remaining bypass in these selected paths. The hostile Slint cases
call the real `checker.check` consumer used by the CLI; the retained public CLI
execution is its healthy control. Shared reader/test bytes still match the repair
receipt. Later Core/Engine additions have separate candidates: the old controller
results do not qualify those complete current scripts. After the N3 role update,
all65 selected Engine/series admission controls pass in
`dependency-refresh-20261106-v1/n3-series-python-controls-v2.log`. This adds current
Python admission evidence, with no new native, resource-exhaustion or whole-product
claim. The quoted remote four-open count remains unreconciled locally.

## Retired local extraction data — 7 November 2026

Under David's unused-evidence cleanup instruction, the obsolete
`codeql-work/databases/product-bd1f7c4-serial/db-rust` extraction relations were
removed. The surrounding metadata and existing `src.zip`, diagnostics, logs,
source receipt and all query results remain. This historical directory is no
longer a queryable database without rebuilding; the cloud rebuild direction
above remains current. The source receipt binds343 committed files to
`bd1f7c4a44e4a8dcf31fa6ca650b3a0cea1dd83e` and retains SHA256
`d30a884b739a9a6d829f0b4814119cc2dfc22334322b349dd5107161141149a6`.

Fresh MCP metadata lists only the separate October29 `/work/database` fixture;
it, installed tools and running services were preserved. No matching readable
process argv/cwd/fd/map or explicit mount used the retired product directory;
three inaccessible process scopes remain a limitation of that sampled check.
All410 protected files and the Git index were unchanged across deletion. Actual net recovery
is574398464 bytes; available5628305408 bytes was observed after deletion, before
later build work. No new archive, compression, unregister or process stop occurred.
Receipt `dependency-refresh-20261106-v1/codeql-derived-index-cleanup-v1.json`
SHA256: `273510a445b4f87cdf9ea3e19a46dc48032bb45bc0ceed678896e0745e1d435d`.

## Imported full-repository findings — 9 November 2026

David supplied five findings observed at `6b947c5`; this repair starts from clean
`fb57533ac0094e6f376ae314ea9a08ad641e86d4`. CSV input SHA256:
`8365609c47be9d944fbf7dc3755fbc3c5194684914affa3e7b29d390d12fbfa3`.
These are imported source findings, not a newly executed CodeQL scan. Coordinator
owns integration, shared policy/workflows and compiler execution; complementary
workers own the AccessKit, download and advisory boundaries. Formal work remains
post-V1; the current Sandbox application-control block, S134, fuzz, storage and
UI deferrals remain in force.

| Imported occurrence suffix | Confirmed mechanism and source repair | Independent expected result |
| --- | --- | --- |
| `16baccac32171bfb1c7b3f4b` | AccessKit's callback used an externally mutable HWND property as a Rust pointer. Owner-thread registrations replace it; callback-held `Rc` retains state across reentrant Drop. | Replacing one window's property with another live owner's address cannot select the other owner. Normal focus and teardown still work. |
| `63e6b7c74fb0e4d99ebdd8c0` | Numeric `WM_NCDESTROY` suppressed teardown. Removal now checks the actual registration, native owner thread and chain head; later subclasses retain state-free forwarding. | A forged message never authorizes freeing a still-addressable callback target. Drop, replacement generations, later chains and failures preserve safe custody. |
| `2182802f85a15ac68451c386` | Setup/CI accepted unbounded response bodies before hashing. Python/PowerShell helpers stream within explicit limits, preserve digest/TLS checks and publish only after acceptance. | Missing/false length headers and oversize bodies cannot exceed the admitted body extent; failure preserves the previous destination and stops downstream extraction. |
| `8a42e166996c147529486ce4` | Fresh advisory/index acquisition had no aggregate filesystem quota; inventory/receipt consumption was unbounded. Require a dedicated quota filesystem, bounded writer supervision and shared finite namespace/body/receipt/export custody. | Unbounded/nonprivate roots and excessive files, bytes, depth, linked/special or changing data refuse before acceptance; partial work cannot emit a valid receipt. |
| `e8021239cf1bda9895a4782d` | Mutable terms HTML selected an unrestricted request URL. Xwin now requests only the reviewed policy-owned DOCX URL paired with its unchanged digest. | Hostile HTML cannot cause a new request authority; the pinned document remains usable. |

The AccessKit threats require an allowed peer at equal/lower target integrity;
Microsoft documents cross-process property changes under
[SetPropW's UIPI rule](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-setpropw).
The safe old-source controls demonstrate wrong-owner selection and skipped
teardown without dereferencing a freed object. They do not demonstrate a native
peer-process exploit or privilege escalation. The repair adds only the existing
Windows dependency's `Win32_System_Threading` feature and a checked owner-thread
query; registry versions, lockfile and shipping roles remain unchanged. Exact
derivative bytes and the expanded Windows feature row are re-admitted separately.

Download controls use tiny synthetic responses, including the real setup entry
point and its actual consumer. Review additionally reproduced a cache replacement
between acquisition and consumer reopening; the immutable returned bytes now
receive the expected hash check. Another review rejected the new PowerShell
helper's assumption that Framework `ReadAsync` cancellation interrupts a pending
body read. It now requires synchronous read-timeout support and checks elapsed
time before/after reads. This is not a measured strict wall-time/native-completion
guarantee; timeout-incapable streams fail closed. No Windows/PowerShell execution,
installation, elevation, protection relaxation or host fallback occurred.

CI archives retain their existing hashes, three-attempt limits and outer
deadlines. Explicit body caps are 32 MiB for nextest, cargo-deny and Gitleaks,
and 1 GiB for CodeQL's retained preparation. Official release metadata reports
12,155,366 / 4,936,832 / 8,230,402 / 695,275,440 bytes respectively. This changes
no CodeQL execution deferral. Xwin uses separate code-owned archive/catalog/terms
ceilings; no mutable HTML expands its authority.

Advisory acquisition requires total filesystem capacity ≤512 MiB and ≤16,384
inodes, not merely sampled free space. The bounded consumer admits ≤64 MiB per
file, ≤512 MiB in aggregate, depth16, paths1024 UTF-8 bytes and an 8-MiB receipt.
The retained 1,550-file healthy snapshot remains within those bounds and matches
its receipt, but belongs to a different standalone laboratory lockfile. It is
not a current-main dependency admission pass. The caller owns the quota mount;
CI prepares it and preserves a
primary acquisition failure if cleanup also fails. Actual new quota-backed
cargo-deny acquisition and inode/byte exhaustion have not been executed locally.

Evidence is under the existing local maintenance root
`/home/david_osipov/.local/share/nevrotick/security-findings-20261109-v1/`.
The bounded actual-module facade runs on Linux with inert Win32/accesskit APIs;
it does not compile a Linux AccessKit backend. Its maintained entry point is
`scripts/verify-accesskit-subclass.py`, also wired into CI. The baseline projection
executes only three safe controls; the unsafe old reentrant-lifetime cases are
never run. Original controller/accounting failures remain recorded separately
from intended behavioral RED. Native USER32/UIPI/UIA, actual HWND-generation and
subclass-chain behavior, Application Verifier/PageHeap, resource retirement and
the applicable critical-file coverage/mutation gates remain unqualified. A source
repair and host facade pass do not close those Windows/release obligations.
The restoration-failure control catches a Drop panic outside FFI; a restoration
failure reached through `WM_NCDESTROY` can still abort at the extern boundary.
General nested activation-state access also retains the pre-existing `RefCell`
panic limitation. The reentrant Drop repair establishes neither of those broader
reentrancy guarantees; retain them in the existing native/lifecycle review scope.

**Integrated source result:** all five supplied mechanisms have causal repairs.
The unchanged baseline projection passes the healthy control and fails both
targeted AccessKit assertions; the repaired actual-module facade passes all
13 discovered cases, including reentrant Drop, later-chain forwarding,
constructor/restoration failures and detached update refusal. No ignored or
filtered cases enter that count. Selected Python regression suites pass334
distinct tests after correcting one stale fixture's exact file inventory; its
original four errors remain in the receipt. The configured actionlint/ShellCheck/
offline pedantic zizmor checks pass both workflows and their healthy/rejection
controls. Both actual frozen Cargo metadata/tree target rows pass all20 commands;
they establish the reviewed Windows feature delta, not compilation or advisory
analysis. A controller initially selected the wrong tree-scope argument and then
overselected an unmounted advisory executable; those setup failures remain
separate from the successful graph commands. The current-lock fresh dependency
row and actual Windows-target build/native/metric gates remain unfinished;
no missing prerequisite was bypassed to execute package code.
