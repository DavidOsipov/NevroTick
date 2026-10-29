# Error handling, safe diagnostics and recovery

Use when changing error types, propagation/conversion, native failure capture,
retry/recovery, fallible cleanup, panic boundaries or error-to-output mappings.
This is the implementation procedure for architecture
[§3.4](../../../../../docs/windows-time-architecture.md#34-shared-windows-adapters-and-diagnostics),
[§6.5](../../../../../docs/windows-time-architecture.md#65-errors-diagnostics-and-proof-obligations),
[§12.3](../../../../../docs/windows-time-architecture.md#123-logging-and-support)
and §18.1. Preserve the authorized slice and existing dependency graph. These
logical owners need not be separate crates; do not scaffold future components or
adopt an error/logging library merely to follow this guide.

## 1. Establish the failure contract and its owner

Trace operation → failure capture → typed propagation → owner decision → safe
public/event projection → actual sink or acknowledgement. In the existing task or
API contract, identify the caller, rejected condition, permitted context, state
before/after failure, effect certainty, cleanup obligation and detecting check.
Cover private helpers and direct callers without duplicating obvious type rules.

| Owner | Required responsibility |
| --- | --- |
| Domain/protocol module | Specific typed failures and invariants; no logging setup, Windows dependency or generic recovery policy |
| `windows-platform` role-specific adapter | Correct native status/error capture, resource lifetime and bounded typed outcome; no clock policy or arbitrary command surface |
| `diagnostics` | Shared stable event IDs, typed allowlisted fields, privacy/projection rules, finite profiles, bounds and sink interfaces; no service/clock authority |
| Receiving operation/state owner | Current authorization, effect/acknowledgement interpretation, retry or reconciliation and one useful failure event |
| Executable composition | Process subscriber/sinks, actual filter routing and finite shutdown; libraries do not install a subscriber |

Centralize reusable native mechanics and diagnostic policy in those shared
boundaries. Keep domain errors and recovery decisions at their existing owners;
do not flatten them into a universal error/recovery layer that absorbs their
invariants. Shared public/event code catalogs are appropriate safe projections,
not replacements for those typed failures. Keep dependencies acyclic: portable
domain/diagnostics must not import Windows adapters. Shared code grants no
additional authority.

## 2. Preserve actionable typed failures

- Use a specific enum/struct and `Result` for expected rejection/OS failures.
  Match variants and reviewed codes for control flow, never localized messages,
  `Display`, `Debug` or substring tests. Preserve meaningful absence separately
  from failure; never convert a failure to empty data, stale success or `.ok()`
  merely to ignore it. Review discarded results explicitly.
- Keep conversions at the abstraction owner. Preserve a useful typed cause or
  native code when needed; do not copy entire rejected inputs or secret-bearing
  library reports into an error. Retained causes have resource/privacy limits
  too. `Error::source()` exposes a cause; it does not authorize emitting it.
  Avoid repeating the same cause in both `Display` and its source chain.
- Handwritten error implementations are valid. Where already admitted,
  `thiserror` provides ordinary `Error`/`Display`/conversion implementations;
  `#[from]` implies a source. `#[error(transparent)]` forwards formatting/source
  behavior and supplies no redaction. Review separately derived `Debug`.
- Do not replace domain/protocol/IPC contracts with an unstructured report.
  If an admitted application boundary uses `anyhow`, its context can preserve
  downcasting, but formatted chains/backtraces remain unsafe output by default.
  A safe outer message does not make `{:#}`, `{:?}` or `{:#?}` safe. No dependency
  addition follows from these examples.
- Public IPC/UI errors use stable product codes and bounded permitted fields;
  human/localized messages are presentation. Do not serialize internal error
  layouts or expose authentication internals. Compatibility changes still use
  [data-contract evolution](../../evolve-data-contracts/SKILL.md).

## 3. Project safe fields before formatting or buffering

At the producer, map typed failure to the shared permitted event/public fields.
Use reviewed codes, constant operation identifiers, bounded counters and allowed
context. Allowlisted schema paths may identify a user field; attacker-selected
names become a generic location. `serde_path_to_error` paths and garde reports
need this mapping; neither is a sanitizer or permission to serialize its report.
Validation and authorization remain with their respective owners.

Do not accept arbitrary `impl Display`/`dyn Error`/preformatted strings as a
supposedly safe diagnostic payload. Central regex replacement or truncation
cannot reliably remove secrets already embedded in text. A safe projection is
not the same thing as input validation, escaping or a redacted `Debug` wrapper.

Inspect every reachable route: `%error`, `?error`, `.to_string()`, error sources,
derived `Debug`, tracing argument capture, dependency/log bridges, panic hooks,
UI/status/export and sink fallback. Never send keys, cookies, credentials,
approval tokens, raw packets/requests or arbitrary memory at any level. Review
private endpoints, user names and paths against the sink's field policy; do not
attach them automatically as diagnostic context.

Bound traversal, allocations and collection of failures before constructing a
report. Bound permitted text and encode/escape control characters for the actual
sink; enforce the encoded byte limit after escaping, preserve valid text and
signal omitted details while retaining rejection. Bound field counts, chain
depth if inspected, queue bytes, rate-key cardinality and counter arithmetic.
Take qualified limits from the owner contract. Architecture's 16-field/4 KiB
validation proposal and aggregate 10 MiB diagnostic proposal are not qualified
implementation constants; name unresolved limits rather than silently adopting
or multiplying them per process.

Log the failure once at the decision owner. Lower layers return typed failures.
Keep required-event routing independent of optional verbosity, including release
features and global/callsite filters. Required means a bounded sink attempt, not
guaranteed delivery or durable acknowledgement. UI state comes from authoritative
snapshots/acks, never parsed logs. Follow
[privacy/logging](../../../security/security-privacy-logging/SKILL.md) for changed
sinks, export, profiles or retention.

## 4. Decide recovery from state and actual effect

Distinguish rejection before an effect, a confirmed completed effect, and an
unknown/partial outcome in the operation contract. `Err`, timeout, disconnect,
cancellation request or lost acknowledgement alone establishes no rollback.
Preserve prior valid state on rejected updates; do not publish synchronized,
authorized or applied status without its actual evidence.

One operation owner decides retries from typed failure **and** current state.
Retry only when the contract establishes no effect or a qualified idempotency/
reconciliation mechanism prevents duplicate effects. Use finite attempts,
deadlines and resource budgets; apply backoff/jitter where the operation needs
it. Avoid retries at multiple layers multiplying work. Recheck authority,
generation, freshness and cumulative budgets after waiting and before effects.
No recovery path silently weakens authentication/quorum, refunds correction
allowances or invents an accepted clock bound.

Retain custody, permits and buffers through real native completion, including
late completion after an awaiting future exits. A started `spawn_blocking` job
cannot be stopped by aborting its handle. An ambiguous protected effect goes to
its state/journal owner for reconciliation before a repeat, not an adapter's
generic `retryable` flag. Use [Windows boundaries](rust-windows.md) and
[fault recovery](../../../testing/qa-fault-recovery/SKILL.md) for reached paths.

## 5. Capture native failures and finish resources correctly

Check the selected Windows API's documented convention, including meaningful
success statuses. Capture `GetLastError` immediately on the same native calling
thread when the API defines it, before any await or thread handoff; cleanup,
logging and intervening calls can overwrite it. Keep code families
(Win32/HRESULT) distinct and attach only reviewed bounded context. Nonnegative
HRESULTs can carry distinct success meaning; blanket conversion to `()` can lose
it. A false BOOL is not universally a failure with a meaningful last error.

Do not assume `windows::core::Error` formatting is safe: native messages may
include attached error information. Convert to the adapter's typed outcome at
the boundary; keep raw message formatting out of diagnostics. Resource-specific
RAII remains the fallback, with an explicit fallible finish/flush/sync/close when
the contract must observe failure. Observing `Drop` run proves no successful
persistence; some destructors ignore errors. Preserve the primary operation
failure and a bounded cleanup failure/outcome without masking either or
inventing successful rollback.

Expected malformed input/OS failures must not use panic/unwrap/expect as recovery.
Review each panic site against the process fault policy. Never unwind across
Windows FFI; `catch_unwind` catches only unwinding panics and the panic hook runs
first. It neither catches aborts nor repairs invalid state or native faults.
Keep panic hooks/reports subject to the same output policy. Do not continue
clock correction with an inconsistent model. Abort/process death skips promised
destructor cleanup; recovery requires the established owner/journal/SCM contract
and separate native evidence.

Respect fatal native-boundary exceptions to ordinary cleanup. In particular,
failed impersonation reversion follows architecture
[§5.4](../../../../../docs/windows-time-architecture.md#54-native-ipc-implementation-and-qualification):
stop request execution through the qualified fatal path, without attempting
privileged clock cleanup under an uncertain identity. Logging, flushing or
unwinding must not be prerequisites for that stop. Raw dumps/EVTX and externally
collected OS traces retain their separate access/content policy; ordinary-output
redaction does not qualify them.

## 6. Verify the reached paths, then report

Select affected rows only. Use the existing task's acceptance IDs and
[testing contract](../../../testing/CONTRACT.md); these are checks for an actual
implementation change, not a mandate to implement production diagnostics now.

| Changed guarantee | Independent observation and healthy control |
| --- | --- |
| Typed rejection/conversion (CHECK-25) | Actual caller receives the expected code/variant; rejected raw input has no protected effect and prior valid state survives; a valid neighboring request succeeds |
| Privacy/usefulness (CHECK-17) | Recognizable synthetic secrets through success, rejection, dependency causes and reached panic/export routes appear in no selected ordinary output at any enabled level; useful permitted code/context remains |
| Reporting bounds (CHECK-17/21/25) | Hostile field names, control characters, oversized/nested reports and repeated failures stay within actual allocation/work/encoded-output bounds; legitimate field association still works |
| Recovery/lifetime (CHECK-14/27 and effect-specific checks) | Timeout/lost ack/late completion/cleanup failure cannot duplicate an effect, leak custody or manufacture success; a confirmed success and eligible recovery still complete |
| Sink/filter failure (CHECK-17/21) | Full/denied/missing sink, queue overflow and flush failure yield bounded loss/status without recursive logging, authority changes or blocked safe cleanup; required INFO still reaches its required sink attempt when optional output is off |

Inspect actual emitters → projection → subscriber/filters → queue → sink/export,
including fallback output. Unused sanitizers, source grep and empty CodeQL results
prove no runtime path. Fake sinks can establish reached portable mapping/filter/
queue contracts; they and cross-builds cannot establish actual native sink behavior.
Use [negative paths](../../../testing/qa-negative-paths/SKILL.md), privacy/logging and
fault-recovery skills as applicable; native tokens, FFI/SCM and Event Log need
Windows observations. Record unreachable/unavailable cases as gaps. Preserve
UI-M0's fixture-only scope and deferred production diagnostics.

Review is incomplete if ownership is unclear, raw formatted errors reach an
ordinary output, a report allocates unbounded work before truncation, retry
ignores uncertain effects, contract-required cleanup results disappear, or
evidence omits a reached output/consumer. Correct within the authorized scope or
record the precise gap; do not label it passing. Report the failure path,
enforcing owner/type, safe projection, state/effect outcome, selected checks and
actual evidence limits in
the existing task. No parallel report tree or automatic release claim.

Original project guidance; dated primary documentation and pinned GitHub findings:
[implementation sources](../../SOURCES.md#error-handling-and-safe-diagnostics).
