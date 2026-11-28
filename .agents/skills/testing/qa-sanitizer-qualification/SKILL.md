---
name: "qa-sanitizer-qualification"
description: "Select and qualify applicable ASan, LSan, TSan, MSan and native UBSan checks at coherent module completion, before V1 release or for a named memory/race diagnosis; routine local edits need no sanitizer campaign."
---

# Qualify sanitizer evidence

Read [the testing contract](../CONTRACT.md) and
[shared selection and controls](../references/sanitizers.md). Apply
[OD-22](../../../../docs/nevrotick-owner-decisions.md#26-od-22--sanitizer-qualification-cadence)
and the existing module/release task; this skill owns sanitizer qualification,
not a second product plan or an unrestricted execution lane.

## 1. Identify the checkpoint and actual scope

- **During implementation:** retain focused behavioral checks and cheap required
  checks. Schedule routine sanitizer qualification after the coherent module's
  declared behavior and cases are complete. Earlier work needs a named diagnostic
  or prerequisite question; do not launch all profiles after every edit.
- **Module finalization:** freeze the module candidate, its relevant consumers,
  features and dependency graph; run the applicable required profiles and record
  gaps before claiming that checkpoint complete. Renaming a slice cannot evade
  its existing exit obligations.
- **Before V1 release:** reconcile the complete applicable matrix against the
  integrated release candidate, including shipped native components and supported
  Windows configurations. Execute missing or invalidated required rows. Earlier
  evidence needs the testing contract's explicit identity and validity argument;
  preserve any required fresh integrated execution.

Locate current task/incident authority before execution. The checkpoint policy
does not lift Windows, scoped fuzz, storage or UI holds, activate post-V1 formal
work, or admit tools/dependencies. Continue independent authorized work when an
applicable row is blocked.

## 2. Derive applicability before selecting tools

Inspect the actual source and resolved build graph, not just the crate's unsafe
lint or a single-threaded runtime label. Include dependencies, native libraries,
allocator families, FFI ownership, callbacks/helper threads, assembly, subprocesses
and relevant accepted/rejected/recovery paths. A safe caller does not establish
safety inside its dependencies; a Linux pass cannot exercise Windows-only code.

Consider every family below. Read the corresponding manual only for a selected
lane or an unresolved applicability decision.

| Family / manual | Selection question |
| --- | --- |
| [ASan](references/asan.md) | Which reached address, lifetime and allocator boundaries need dynamic evidence? |
| [LSan](references/lsan.md) | Which allocation-owning lifecycles need unreachable-leak evidence? |
| [TSan](references/tsan.md) | Which actual shared-memory operations and synchronization need race evidence? |
| [MSan](references/msan.md) | Which initialization-sensitive paths require sufficiently instrumented graph coverage? |
| [Native UBSan](references/ubsan.md) | Which compiled C/C++ operations require selected language-rule checks? |

Decide applicability for the actual language, target and execution scope from
primary support evidence. Demonstrated upstream lack of a native mechanism on
that target may be bounded N/A for that mechanism; retain the underlying race,
initialization, memory or resource obligation with its existing owner, named
alternative evidence and unresolved gaps. This is no clean detector result.
Missing installation, admission, instrumentation or permission for an otherwise
applicable supported profile stays blocked/incomplete. If an accepted requirement
explicitly names the unsupported mechanism, leave it open for its owner to resolve;
do not waive it. Do not manufacture native UBSan work for a graph with no compiled
native C/C++ source. Reconsider applicability when the graph or target changes.
Other mechanisms such as HWASan/MTE, data-flow instrumentation or real-time checks
need their own relevant platform/contract; "all" is not every LLVM flag.

Retain complementary [Miri/model/fuzz evidence](../references/configured-tooling.md#additional-rust-analysis),
[Windows diagnostics](../references/windows-memory.md) and
[resource measurements](../qa-performance-resources/SKILL.md). Sanitizers cannot
establish RFC conformance, cryptographic correctness, bounded reachable retention,
schedule completeness, clock authority or a correct high-level state protocol.

## 3. Keep one matrix in the existing task

| Component / consumer | Family and defect question | Target, profile and instrumented graph | Detector controls | Product candidate and cases | Disposition / evidence | Gap owner and next condition |
| --- | --- | --- | --- | --- | --- | --- |

Use: **planned; executed with result; qualified evidence reused;
blocked/incomplete; N/A with applicability evidence; deferred by a cited accepted
decision**. "Executed" must include the actual outcome below. A new required V1
row cannot be deferred by the worker's preference. Keep detector admission,
application instrumentation, reached product behavior and native Windows evidence
as separate claims even when they share a matrix row.

## 4. Admit the application profile before running it

1. Bind source/configuration/lockfile, selected package/bin/test, feature vector,
   explicit target, compiler/LLVM/runtime and linker/CRT identities. Record the
   actual executable and standard-library/native instrumentation. Target-source
   support bits establish code generation support, not distributed runtimes or
   a working detector.
2. Select an existing admitted route or record the precise missing admission.
   A preinstrumented Rust target and a nightly rebuilt-standard-library route are
   distinct configurations. Neither alone qualifies all native dependencies.
   Separate host build scripts/procedural macros from target instrumentation;
   mixed-language runtime linking requires its own compatibility evidence.
3. Preserve complete effective flags, mitigations, runtime options, suppressions
   and invocation path. Ambient `RUSTFLAGS` can override configured flags;
   cargo-fuzz launch defaults need not apply to a directly launched executable.
   Record exclusions as coverage limits rather than suppressing them into success.
4. Assign one owner per writable output and native image profile. Use the admitted
   credentialless, isolated runner and finite aggregate case/time/CPU/memory/
   process/output budget, including controls and retries. Follow
   [campaign accounting](../references/campaign-budget.md). A detector's admitted
   virtual-address-space exception does not remove other limits.
5. Keep ASan, TSan and MSan in separate configurations. ASan with LSan is conditional
   on demonstrated support and effective leak checking. Native ASan and the
   existing AppVerifier/PageHeap profile are separate by default, with their own
   runtime, debugger and restoration evidence. Diagnostic binaries and costs do
   not replace shipping-artifact and uninstrumented performance qualification.

The existing [standalone sanitizer checker](../../../../TOOLING.md#standalone-sanitizer-detector-controls)
accepts fixed detector fixtures. Its five-profile `all` selection supplies no
application runner, instrumented full `std`/dependency graph or Windows result.
Do not expand its inputs, relax custody or substitute an unrestricted shell run
to obtain product evidence. Historical controls remain evidence for their stated
identities; this skill installs and executes nothing by itself.

## 5. Execute selected cases and classify the evidence

Establish the shared reference's distinct healthy input-delivery, semantic-oracle
and detector-mechanism controls. Reuse valid detector receipts within their exact
compiler/runtime/target/instrumentation assumptions; repeat affected controls when
those assumptions change. Use existing owned isolated fixtures, keeping deliberate
undefined behavior outside product and pure `forbid(unsafe_code)` owners.

Run the admitted product cases through actual selected entry points. Include
useful healthy inputs, rejection and affected failure/recovery/cleanup behavior.
Inspect resource release before harness teardown conceals it. Preserve selected
and executed counts, native exit status, symbolized diagnostic/site, effective
options, logs, first failures and consumed budget.

Classify each outcome as **pass within recorded scope; sanitizer finding; semantic
assertion failure; invalid detector/harness setup; unsupported target/API/graph;
timeout/resource exhaustion; unclassified crash; or not executed**. An unsupported
or unexecuted required row remains incomplete. A panic, nonzero exit, matching
exit code or `libFuzzer: deadly signal` alone is no detector-specific finding.
Retain legitimate semantic evidence without promoting it to memory detection.

For a finding, use [failure triage](../qa-failure-triage/SKILL.md), preserve the
original evidence and make the authorized causal repair. Retain a bounded
regression and healthy neighbor; refreeze and rerun affected evidence without
resetting campaign consumption. An unresolved required failure blocks that
qualification claim.

## 6. Close the checkpoint honestly

Return the reconciled matrix, actual evidence references, repairs/regressions,
remaining gaps and next condition. At V1, account for every shipped component
and relevant configuration; controls-only, held native execution or an unknown
instrumentation graph cannot become release readiness. Pass the result to
[final qualification](../qa-final-qualification/SKILL.md). A clean bounded run
proves neither absence of all defects nor complete product qualification.

Technical sources and version disagreements are retained in the
[dated source register](../SOURCES.md); research dates are not execution dates.
