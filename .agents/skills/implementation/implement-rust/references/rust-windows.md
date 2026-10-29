# Ownership, async and native boundaries

Read canonical architecture sections 9–11, 17.1 and 18.1 before boundary changes.
Choices implement those responsibilities, not replacement service design.

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

Declare unsafe operations even inside unsafe functions; separate caller obligations
from implementation's unsafe blocks. No unwinding across Windows FFI. Recoverable
input/OS errors return explicit results. Chosen abort/SCM recovery needs Windows
proof; RAII promises no cleanup after abort/process death. Critical persistent
effects need recovery records/reconciliation at existing owner.

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
