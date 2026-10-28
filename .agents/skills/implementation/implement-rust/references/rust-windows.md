# Ownership, async and native boundaries

Read the canonical architecture sections 9–11, 17.1 and 18.1 before changes to
these boundaries. The following choices implement its responsibilities, not a
replacement service design.

For SCM dispatch, service status/control callbacks, account qualification or native
worker stop/restart behavior, read [Windows service adapters](windows-service-adapters.md).
It gives the bounded S0.2/CHECK-27 procedure; load it only for those boundaries.

## Unsafe and handle ownership

Keep raw handles, pointers, Win32 structures and unsafe calls in narrow adapters.
For each unsafe block state the specific pointer validity, buffer length, encoding,
thread/apartment, aliasing, ownership and lifetime obligations that apply. Check
the selected API's success/error convention; Win32 errors are not uniformly HRESULT
or `GetLastError`. Capture a last-error value only when that API defines it.

Use an owning wrapper for resources actually owned and release them exactly once
with the matching API; borrowed or sentinel handles are not automatically owned.
Do not use a numeric handle/process ID as enduring proof of caller identity.
Buffer capacities, UTF-16 terminators and byte versus element lengths are checked.
Do not transmute representations or add unsafe Send/Sync to bypass a failed design.

Declare unsafe operations explicitly even inside unsafe functions; keep the
caller's safety obligations distinct from the implementation's unsafe blocks.
No unwinding crosses Windows FFI. Recoverable input/OS errors use explicit results.
Chosen abort/SCM recovery behavior needs Windows evidence; RAII cannot promise
cleanup after abort or process death. Critical persistent effects need recovery
records and reconciliation at their existing owner.

## Async, bounded work and state

Identify the authoritative state and its serialized transitions. Capture expected
revision/generation before work and check them again before publication/effect.
Cancellation of a future does not prove the native operation stopped. In
particular, a started Tokio `spawn_blocking` task cannot be aborted by dropping or
aborting its JoinHandle. A timeout can stop waiting while real work continues.

Bound admission before spawning; retain a permit for the actual operation's
lifetime, including late native completion. Bound blocking queues, outstanding OS
requests, connections and retries as well as logical futures. Define shutdown
drain/stop behavior and late-completion handling. Do not hold a state lock across
arbitrary blocking/async calls unless the owner deliberately requires it and the
deadlock/responsiveness consequences are understood.

Use monotonic deadlines and an injectable deterministic clock for pure scheduling
tests. Distinguish elapsed monotonic from Windows UTC and active-awake semantics;
the platform's suspend behavior needs its own Windows test. A sleep/resume,
external step, policy change or reconnection invalidates the appropriate generation.

## Checks that correspond to claims

Once Cargo exists, use the repository's selected toolchain, feature combinations
and target matrix. The ordinary commands in TOOLING.md are starting points, not
proof every Windows-only file was compiled or executed. Avoid unconditional
`--all-features` when features select incompatible backends. Check actual file/CFG
admission, release panic policy, emitted PE protections and SDK/linker inputs where
those claims matter. Do not add blanket Clippy restriction/nursery groups or broad
allows; narrow expectations have reasons and stale-expectation checks.

Primary sources and reviewed findings are in [SOURCES](../../SOURCES.md).
