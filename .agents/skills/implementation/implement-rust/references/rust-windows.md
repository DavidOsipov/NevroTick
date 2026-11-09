# Ownership, async and native boundaries

Read canonical architecture sections 9–11, 17.1 and 18.1 before boundary changes.
Choices implement those responsibilities, not replacement service design.

- [Unsafe and handle ownership](#unsafe-and-handle-ownership)
- [Directional secret APIs](#directional-secret-apis)
- [Async, bounded work and state](#async-bounded-work-and-state)
- [Checks that correspond to claims](#checks-that-correspond-to-claims)

For newer pointer/provenance, initialization, atomic or blocking-library APIs,
use [modern stable Rust](modern-rust.md). Preserve this reference's ownership and
completion contracts when substituting a newer mechanism.

For SCM dispatch, status/control callbacks, account qualification or native-worker
stop/restart, read [Windows service adapters](windows-service-adapters.md).
Load its bounded S0.2/CHECK-27 procedure only for those boundaries.

## Unsafe and handle ownership

Confine raw handles/pointers, Win32 structures and unsafe calls to narrow adapters.
State each unsafe block's applicable pointer validity, buffer length, encoding,
thread/apartment, aliasing, ownership and lifetime obligations. Check selected API's
success/error convention: Win32 errors are not uniformly HRESULT or `GetLastError`.
Capture last-error only where API defines it.

For native error mapping, uncertain effects, cleanup failures or panic boundaries,
apply [error handling and safe diagnostics](error-handling.md). Capture the
documented status before intervening work; formatted OS errors are not safe fields.

Wrap actually owned resources; release exactly once through matching API.
Borrowed/sentinel handles are not automatically owned. Numeric handle/process IDs
prove no enduring caller identity. Check capacities, UTF-16 terminators and byte/
element lengths. Never transmute representations or add unsafe Send/Sync to bypass
failed design.

Use standard handle owners/borrows only where their destructor, rights and thread
contract fit. Preserve explicit fallible completion when close failure affects
acceptance; generic RAII alone does not preserve that outcome or thread affinity.

Declare unsafe operations even inside unsafe functions; separate caller obligations
from implementation's unsafe blocks. No unwinding across Windows FFI. Recoverable
input/OS errors return explicit results. Chosen abort/SCM recovery needs Windows
proof; RAII promises no cleanup after abort/process death. Critical persistent
effects need recovery records/reconciliation at existing owner.

## Directional secret APIs

For same-shaped directional secrets, consider one private exclusive pair with
direction-specific operations such as request sealing and response opening.
Separate non-Clone key types are an alternative when real consumers need distinct
custody; do not introduce them mechanically. Keep construction at the actual
secret producer (the same-connection exporter for NTS), without public raw
constructors/getters or generic key selectors that bypass the intended boundary.
Retain byte/cookie bounds and cleanup.

Direction does not prove correct exporter context, same-association binding,
current trust/generation or authentication. Preserve those checks at their real
producer and serialized effect owners. At the actual receiver seam, pair useful
accepted behavior with independently specified wrong-context/wrong-key rejection.
Use compile-fail controls only for capabilities the real API excludes; if an
encapsulated pair removes caller key selection, invent no surrogate selector to
obtain a type error. Type rejection cannot validate a buggy private constructor.

## Async, bounded work and state

Identify authoritative state/serialized transitions. Capture expected revision/
generation before work; recheck before publication/effect. Future cancellation
proves no native-operation stop. Started Tokio `spawn_blocking` tasks cannot abort
through dropping/aborting JoinHandle; timeout may stop waiting while work continues.

Bound admission before spawning; retain permits through actual operation lifetime,
including late native completion. Bound blocking queues, outstanding OS requests,
connections/retries as well as futures. Define shutdown drain/stop/late-completion
behavior. Hold no state lock across arbitrary blocking/async calls unless owner
deliberately requires it with understood deadlock/responsiveness consequences.

If the selected worker boundary uses bounded Tokio `mpsc`, consider
`Sender::try_reserve_owned` before submission to reserve completion capacity
without waiting. Admit job count, completion slot and result-byte allowance as
one transaction; bound admission attempts instead of accumulating reserve waiters.
Arrange issuance or explicit not-started rollback so pre-submission refusal leaks
no charge. Dropping a non-dropping actual-work permit does not roll it back, and
its physical-completion operation must not report work that never started.

Move completion capacity and actual-work custody with the real job. Channel
permit destruction proves neither native completion nor result retirement. Keep
result/secret bytes owned and bounded until acceptance or rejection and disposal;
delivery, current-generation acceptance and physical completion are distinct.
`OwnedPermit::send` returns a `Sender`, not a delivery acknowledgement or an
unsent-value recovery result. Receiver drop, worker panic and stale completion
must reach the real completion/recovery owner without losing that custody.

`Receiver::close()` still permits already-reserved sends: retain the receiver and
drain until `recv()` returns `None`. A stuck native job can retain its permit and
prevent that drain from finishing; preserve truthful diagnosed shutdown/recovery,
not an assumed finite termination guarantee. Fixed result slots or retained/joined
job owners may be simpler for a small fixed population; add no channel just to use
this API. Exercise the chosen real admission/retirement path with a held job,
cancellation, close versus drop, panic and a healthy neighbor. Observe work/byte
bounds, late-result retirement and denied stale effects independently; a channel
demonstration alone qualifies no native stop or association integration.

Use monotonic deadlines/injectable deterministic clocks for pure scheduling tests.
Distinguish elapsed monotonic, Windows UTC and active-awake semantics; qualify
platform suspend separately on Windows. Sleep/resume, external steps, policy changes
or reconnects invalidate appropriate generations.

## Checks that correspond to claims

Once Cargo exists, use selected toolchain/features/targets. TOOLING.md commands are
starting points, not proof every Windows file compiled/executed. Avoid unconditional
`--all-features` with incompatible backends. Check claim-relevant file/CFG admission,
release panic policy, emitted PE protections and SDK/linker inputs. No blanket
Clippy restriction/nursery groups or broad allows; narrow expectations need reasons
and stale-expectation checks.

For native heap/handle/FFI or resource-lifetime evidence, read
[Windows memory diagnostics](../../../testing/references/windows-memory.md).
Application Verifier/full PageHeap detect selected reached native defects;
measure repeated lifecycle retention separately. Keep setup elevation separate
from ordinary-user execution and verify restoration of owned per-image settings.

Primary sources/reviewed findings: [SOURCES](../../SOURCES.md).
