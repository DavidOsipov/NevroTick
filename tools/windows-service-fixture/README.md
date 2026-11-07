# Inert S0.2 Windows service source fixture

This standalone experimental workspace prepares real SCM entry points and their
shared owner for W11-W05/S0.2/S3.1, CHECK-27. It is outside the product workspace
and implements no production service, account provisioning or clock authority.
All Windows execution and installation remain held. The existing Windows task
owns the source card, admission and later native permission; this README grants
none of them.

The two executable compositions select only these laboratory identities:

| Executable | Fixed laboratory SCM identity |
| --- | --- |
| `nevrotick-lab-s02-engine` | `NevroTick-Lab-S02-Engine` |
| `nevrotick-lab-s02-broker` | `NevroTick-Lab-S02-Broker` |

Final product technical names and supported MSIX account/provisioning remain
pending in the existing metadata/task owners. No account is selected here.
There is no public raw service-name, handle, callback or dispatcher API.
GNU entry points return the typed `WindowsOnly` error and exit unsuccessfully;
they provide no console service fallback.

Both roles use the explicit laboratory card limits: pending wait hint 1,000 ms,
startup/stop allowance 10,000 ms and maximum Running interval 60,000 ms. These are
not product defaults or measured Windows latency guarantees. The Running deadline
is anchored to the Ready observation, before control-loop work admission, and is
never renewed. Pending/Running deadlines cannot interrupt an arbitrary native
call. The only admitted work permit owns the genuine inert control loop; it
completes before terminal reporting.

The source path is fixed executable → one dispatcher attempt → role-specific
`ServiceMain` → fixed handler registration → private non-cloneable status token →
the existing portable `ServiceLifecycle` and `ScmStatus` → checked native report.
Argument zero is ignored SCM metadata. Counts above one are rejected without
reading the vector or strings. The process command line selects no role, path,
endpoint, policy or control privilege.

Running means only that the inert control loop can receive Stop. HandlerEx reads
no event/context pointer and invokes no application closure. It decodes through
the existing `StopNotification`, coalesces Stop and unparks the already-published
owner thread. Interrogate and unsupported controls cause no status report. The
owner rechecks the sticky bit and original monotonic deadline after every wake,
including spurious wakes and Stop delivered immediately before parking.

The owner thread handle and notification have process-lifetime static custody,
without reset or callback-context deallocation. Each report initializes all seven
numeric fields. The registration token is not a `CloseHandle` resource. Its only
owner removes it before the terminal attempt, including failure, and never retries
that context. All required loop cleanup precedes that attempt. The original native
error is captured immediately on its failing thread, before formatting or cleanup.

Only an immutable owner-result receipt is published after the owner returns; it
does not perform more operational work or use the terminal status context. Success
cannot precede actual status acknowledgement. If the dispatcher returns before
that receipt is available, the entry fails with `OutcomeUnavailable`. That honest
conservative race outcome remains part of future native qualification.
An actual dispatcher failure has its own immutable custody and takes precedence
over any owner receipt, including success; it never overwrites that receipt.

Expected failures are fallible and produce nonzero service-specific terminal code
1 where status reporting remains possible. The original typed cause is separately
retained in the result. Unexpected Rust panic in `ServiceMain` aborts this inert
process without unwinding across its ABI or resuming work. This chooses no shipping
privileged-process fault/recovery policy and promises no destructor on abort.
Both native entries use the same private Rust guard exercised by isolated GNU
child-process tests. Its captured panic payload remains owned until abort. The
Rust panic hook runs before the guard can catch an unwinding panic; this guard
establishes neither bounded/redacted hook behavior nor native-fault recovery.
The selected child-test runner must impose a finite deadline and disable core
dumps. GNU child outcomes establish no Windows ABI or native execution result.

The approved tool-only negative cleanup rule is narrower than normal time use.
If a monotonic observation reverses, the fixture retains the last actually trusted
observation solely for failed-stop bookkeeping, exits its control loop and
completes that actual permit. It performs no further clock read, ordinary work,
wait or budget renewal. The historical observation does not assert current time,
restored clock validity or timely cleanup. `ClockReversed` remains the result
when it is the first failure; an earlier original cause retains priority.
Terminal status must be Failed in either case. This inert fixture exception is
neither product clock recovery nor an adopted native fault policy.

The fixture creates or changes no service, account, token, privilege, protected
file, policy, Windows clock, network connection, DNS request, IPC endpoint or
persistent state. It has no installer, shell command execution, wrapper, automatic
restart, elevation or additional runtime.

The exact external Windows edge is existing `windows-sys` 0.61.2, defaults off,
with `Win32_Foundation` and `Win32_System_Services` only. The other direct edge is
the existing path-owned `windows-platform`. The new lock is seeded from the root
lock before offline resolution to preserve admitted versions. Resolution/build
need their separate source/graph review and coordinator go-ahead.

Pinned Rust 1.99.1 source selects the futex parker on the standard Windows target.
`Thread::unpark` uses its existing parker, a Release atomic swap and, when needed,
`WakeByAddressSingle`; the inspected path has no allocator, Rust mutex, waiting,
formatting or user callback. `OnceLock::get` is nonblocking. Thread/TLS initialization
occurs in `ServiceMain` before handler registration, never in HandlerEx. This is
source evidence, not a universal latency guarantee or a native pass.

Primary references:

- [Rust Thread::unpark](https://doc.rust-lang.org/std/thread/struct.Thread.html#method.unpark)
  and [park_timeout](https://doc.rust-lang.org/std/thread/fn.park_timeout.html).
- Pinned local `rust-src`: `std/src/thread/thread.rs`,
  `std/src/sys/sync/thread_parking/{mod,futex}.rs`,
  `std/src/sys/sync/futex/windows.rs` and `std/src/sync/once_lock.rs`.
- [SCM dispatcher](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-startservicectrldispatcherw),
  [handler registration](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-registerservicectrlhandlerexw),
  [HandlerEx](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nc-winsvc-lphandler_function_ex)
  and [status reporting](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-setservicestatus).

Host tests exercise the actual private hosting core with injected registration,
status, monotonic observations and finite waits, plus both real GNU executable
entries. They establish no Windows API failure, callback delivery, token, native
latency, MSIX, installed-service or product qualification result. Compiler/lint,
coverage/mutation, graph and actual native/package evidence remain separate.
