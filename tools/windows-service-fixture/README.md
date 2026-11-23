# S0.2 Windows service source fixture

This standalone experimental workspace prepares real SCM entry points and their
shared owner for W11-W05/S0.2/S3.1, CHECK-27. It is outside the product workspace
and implements no production service, account provisioning or clock authority.
The existing Windows task owns the source card, scoped Windows execution,
and installation holds.

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

## Prepare the disabled-service MSIX pack inputs from WSL

Supply the fixed two-role linker receipt, its expected digest, the admitted
inert launcher and the previously admitted MakeAppx input directory. Use a
fresh output path under an owned external parent.

```sh
python3 -I -B tools/windows-service-fixture/msix/prepare_payload.py \
  --receipt "$role_link_receipt" --receipt-sha256 "$role_link_sha256" \
  --inert "$inert_launcher" --inert-sha256 "$inert_sha256" \
  --sdk-source "$makeappx_inputs" --output "$pack_input_output"
```

The output has `payload/`, `sdk/` and `input-set.json`. The JSON printed by
the command includes the inventory digest consumed by the existing offline
packer. Omitting `--sdk-source` emits just the payload tree and its file
digests. Exit 3 means the tree was published and the final directory flush
failed. Read back its contents before attempting another output.
The [S0.6 task](../../tasks/windows11-integration.md#publish-the-disabled-service-pack-inputs-as-one-tree--22-november-2026)
records the pinned inputs, independent archive controls and Windows gates.

Run the payload publication and unsigned package reader regressions with
`python3 -I -B -m unittest discover -s tools/windows-service-fixture/msix`.
The reader tests build inert archives and check the CLI's digest, payload, ZIP,
block-map and XML decisions without installing a package.

For the held §7.5 pre-start probe, the
[installed identity companion](msix/installed_identity.psm1) uses the same
two fixed laboratory roles. Run it in a fresh 64-bit Windows PowerShell
process under the admitted deployment user. The supervisor supplies
`RunId`, `ExpectedUserSid`, `PackageFullName`, `SignedPackageSha256`
and each role's `Bytes` and `Sha256` values using the `Engine` and
`Broker` parameter prefixes. Derive them from independent inspection
of the exact final signed lab artifact. The companion emits one readback
record after both role and registration checks. Schema 2 includes the matched
package full name. Retain native errors privately.

The [pre-start coordinator](msix/prestart_supervisor.py) supplies the fixed
observation sequence through `supervise_prestart`. Its Windows window owner
must admit the independent signed package and role members, keep the original
deployment window current and bind each capture to the retained child process
and its independently observed effective token. The deployment context
supplies identity and the two-role observation. The proposed provisioning
actor and standard user supply their own access captures. The coordinator
validates each receipt before the next selector can start. An invalid child
receipt retains precedence over a later window refusal. The native window
owner must still be wired before the installed probe can execute.

The [Windows child collector](msix/prestart_child.psm1) reads bounded raw
stdout and stderr from the launch owner's original `Process`. It requires
both EOFs and observed exit under the supplied monotonic deadline. The owner
retains process containment and must settle any outstanding reads attached
to the caught exception. The control script exercises this component with
fixed inert children in 64-bit Windows PowerShell. It performs no package
installation or service operation. [Native controls](msix/test_prestart_child.ps1)

The collector also supplies `Invoke-PrestartObserver` for the three fixed
SCM observers. Use a dedicated 64-bit Windows PowerShell process with
serialized child launches and an exclusively owned local input directory.
Use `Open-PrestartObserverActor` to retain the launcher primary token and
pass that actor as `-Actor`. Dispose the actor when its owner finishes
observing that context. Role eligibility remains a separate input to the
installed window.
Supply the approved `nevrotick-prestart-observer.exe`, its exact length and
SHA-256, the selector, and the original running Stopwatch and deadline.
The launcher retains the executable and observes the original suspended
process in its one-child job before resuming it. It compares the observed
primary-token attributes at that boundary and keeps the query-only child
token through capture. Its result contains raw
streams and the original PID and exit code after verified retirement and
input readback. A failed capture can carry the still-owned child for
recovery. This is the same-actor launch primitive for the future installed
window owner. [Native controls](msix/test_prestart_observer.ps1)

For readback of already owned captures, use `join_prestart` in
[the receipt reader](msix/prestart_receipts.py). Both paths apply the same
joined checks and preserve the raw service-type observations.
The [fixture task](../../tasks/windows11-integration.md#bind-disabled-scm-roles-to-their-installed-msix-images)
records the companion's path and file controls and the held setup.

## Fixture contracts and execution

Both roles use the explicit laboratory card limits: pending wait hint 1,000 ms,
startup/stop allowance 10,000 ms and maximum Running interval 60,000 ms. These are
not product defaults or measured Windows latency guarantees. The Running deadline
is anchored to the Ready observation, before control-loop work admission, and is
never renewed. Pending/Running deadlines cannot interrupt an arbitrary native
call. One startup permit owns synchronous self-token observation through explicit
cleanup attempts; unknown capture/cleanup refuses Ready in either role. Both roles
compare the complete present privilege set before worker preparation: Engine
requires only ChangeNotify; Broker requires ChangeNotify plus disabled Systemtime.
Missing or extra privileges, including unknown or disabled extras, refuse startup.
ChangeNotify may be enabled or disabled. These are checked representation facts,
not a native token or installed-identity qualification. A later sequential permit
owns the actual control loop and the broker's read-only observations and
completes before terminal reporting. These permits do not overlap or enlarge the
one-slot capacity.

The [8 November source handoff](../../tasks/windows11-integration.md#4358-service-source-handoff-and-integration-prerequisites--8-november-2026)
owns reuse for bounded S3.1 integration preparation. The
[9 November portable privilege checkpoint](../../tasks/windows11-integration.md#portable-service-privilege-checkpoint-final-qualification--9-november-2026)
closes the selected platform/caller ordinary and per-file coverage/mutation gates
on its recorded Linux candidate. Product worker/composition adoption, supported
identities/provisioning and native Windows evidence remain open; this fixture is
not a completed production service.

The source path is fixed executable → one dispatcher attempt → role-specific
`ServiceMain` → fixed handler registration → private non-cloneable status token →
the existing portable `ServiceLifecycle` and `ScmStatus` → checked native report.
Argument zero is ignored SCM metadata. Counts above one are rejected without
reading the vector or strings. The process command line selects no role, path,
endpoint, policy or control privilege.

Running acknowledges the admitted control loop. The Engine advertises Stop.
The broker also advertises time-change and power notices through its bound
static handler. `HandlerEx` ignores event and context pointers. Stop closes the broker
mailbox, publishes the existing sticky bit and wakes the owner. A delivered
time-change notice advances the live revision and wakes the same thread. A
delivered suspend blocks capture until the broker observes automatic resume,
which opens a fresh revision. The owner rechecks Stop and the original
monotonic deadline after every wake.

Under its original permit the broker can retain a checked raw QPC/FILETIME read
for the initial revision and for later notices. A one-second lab cadence also
observes consecutive brackets during quiet periods. A changed frequency,
counter regression, or backward FILETIME retires the original revision.
A final revision check orders reads and sampled invalidations against
concurrent notices. `retained_broker_clock_read` exposes the final historical
attempt, and `retained_broker_clock_break` preserves the latest sampled cause
after recovery. Physical delivery, UTC mapping
and installed service qualification have separate exits. See the
[SCM time-change checkpoint](../../tasks/windows11-integration.md#broker-scm-time-change-and-renewable-raw-observation--22-november-2026).

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

The private S3.1 worker interface additionally distinguishes unmarked Pending,
a terminal return marker awaiting physical thread exit/join, and actual Finished.
Finished carries separate optional operation and cleanup facts; health carries
the same tagged kinds and never acknowledges completion. Stop fences admission
before cancellation and captures both Finished facts even when health was never
observed. The first cause retains priority; one distinct later operation and one
later cleanup fact occupy fixed slots beside the existing status failures.
`ServiceFailure::operation()` exposes the secondary operation fact;
`completion()` continues to expose cleanup. Required event 9 reports each retained
operation slot with ERROR severity and fixed worker cause codes. Event IDs 1–8,
nine-byte records and each sixteen-record route retain their meanings and limits.

The v2 diagnostic catalogue adds required ERROR/High event 10 for the first
native cleanup failure observed while an earlier primary error and unresolved
worker custody are both present. Event 5 retains its separate actual-completion
meaning. The live host can route the pending observation before a terminal receipt.
The fixed laboratory ServiceMain exposes its portable snapshot only after a receipt,
so an unresolved invocation still has no externally available native delivery.
The 128-byte v2 layout and IDs 1–9 retain their meanings. An older strict reader
refuses the new ID. The first route attempt per ID remains reserved within the
existing 32-attempt window (ten first-ID credits and 22 shared repeat credits).

For the test-only real-thread composition, a current terminal marker permits a
distinct join-tail wait through the owner's existing wait primitive. Its requested
duration is at most 20 ms, clamped to the original remaining phase deadline before
expiry. Exact expiry first records failure; subsequent marker-gated waits observe
only actual exit/join and grant no new allowance, work, progress or Clean outcome.
The request is neither a minimum polling interval nor a measured rate/latency
guarantee: spurious/early wakes remain possible. Unmarked Pending stays event
driven after expiry. On sticky clock reversal the host discards its phase deadline
and performs no further Clock reads or fallback deadline arithmetic. Shared
`ServiceLifecycle` failed-stop bookkeeping still consumes the historical last
trusted observation; it supplies no trusted host allowance. Only this marked
real-thread tail may request the fixed relative cleanup wait with unusable time.
The default inert worker still completes without that wait. Neither path provides
native recovery, termination or performance qualification.

The native executable defaults create or change no service, account, token, privilege, protected
file, policy, Windows clock, network connection, DNS request, IPC endpoint or
persistent state. It has no installer, shell command execution, wrapper, automatic
restart, elevation or additional runtime.

The exact external Windows edge is existing `windows-sys` 0.61.2, defaults off,
with `Win32_Foundation`, `Win32_Security`, `Win32_System_Services` and
`Win32_System_Threading`. The other direct edge is the existing path-owned
`windows-platform`. The Windows executable graph also admits the existing
local `windows-platform-native` leaf for the broker's clock reads. The private
managed loopback composition adds the local Engine as a dev-dependency with
defaults disabled and exercises owned loopback UDP and one joined thread. The
Windows ETW and Time binding features remain test-only.
The lock preserves admitted registry identities. The retained
graph/ordinary-check receipts cover their frozen candidate; changed source or
features require refreshed admission before dependent checks.

The historical Rust 1.99.1 source inspection selected the futex parker on the
standard Windows target; current builds use the repository Rust 1.100 admission.
In that inspected source, `Thread::unpark` uses its existing parker, a Release
atomic swap and, when needed,
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

`qualification.py privilege-checkpoint` batches the existing twelve format,
Clippy, debug/release test, doctest and rustdoc checks under one admission. The
fixture metric consumer separately requires a current single-command
`fixture-test` receipt. `metrics.py coverage` selects host, error, diagnostics,
the private Engine worker and the portable threaded writer. The writer's unit
controls live in a separate source file so they have no writer line credit.
`enumerate`/`mutation` retain the existing
three native cargo-mutants owners. Native execution uses the pinned supported
`--in-place` mode in a complete private derivative, with one mutation thread
and only the selected file writable through both aliases. Manifests, tests,
dependencies and the original base remain read-only; full restoration is checked
after each owner. The worker is omitted by ordinary discovery;
`worker-enumerate` admits its separate complete direct-file inventory, and
`worker-replay-pilot`/`worker-replay` require an independently reviewed current
inventory. They retain raw direct identities and actual Build/Test results, never
native cargo-mutants outcomes. A pilot earns no full-file score. Full replay
retains exact worker overlays, complete reconstruction identities and command
links while retiring only checked private derivatives. Independently verify all
archived overlays after the last subprocess before accepting qualification.
`writer-enumerate`, `writer-replay-pilot` and `writer-replay` use the same
bounded direct replay with the exact `src/diagnostics_thread.rs` owner, a
separate schema and a separate overlay archive. Writer behavioral credit
requires the named queue or host writer control at its corresponding test
file. The roster, complete enumeration and review bind the exact candidate.

The runner keeps a 4-GiB disk and 1-GiB available-memory reserve during commands;
metric exports explicitly preserve 4 GiB before and during copying. Original inputs stay read-only; explicit private native mutation outputs are
confined to the selected file at both namespace aliases. Each replay
starts from an absent/empty private target, then requires successful package-only
Cargo cleaning and fresh fixture libtest artifacts for variants and restoration.

For `graph` and `privilege-checkpoint`, `--scratch-root` can select an existing
private, user-owned directory outside the checkout and read-only inputs. For
example, create a private child of the WSL tmpfs with
`mktemp -d /mnt/wsl/nevrotick-service.XXXXXXXX` and pass its absolute path.
The runner creates a new label directory there and checks the 4-GiB reserve
on both filesystems. Raw receipts stay in the existing main-repository
directory, and the shared cache lock still serializes writers. Retire the
owned derived work only after process exit and receipt inspection. Metrics
and Windows cross-link retain their existing storage routes.
Platform-only measurements do not qualify these callers. The existing W11 task
retains the exact candidates, first failures and native/production limits.

When changing a collector, start with its own selection. Run the fixture
reader controls with `python3 -IB -m unittest discover -s tools/windows-service-fixture`
and the shared service reader with
`python3 -IB -m unittest discover -s scripts/tests -p test_verify_service_analysis.py`.
Use the raw Build and Test logs from an admitted run to check the revised
reader before scheduling another build. A worker replay pilot exercises
direct-file generation and restoration. The three ordinary cargo-mutants
owners instead use the pinned tool's in-place Build/Test output. Every viable
variant, including a reviewed equivalent, must show a new build of each role
containing the mutation and a launch of the corresponding test executable.
On GNU, host and diagnostics affect the libtest. The common error module also
feeds the normal library and its dependents. Cargo can reuse unaffected
artifacts. An abort without a selected assertion still needs its separate
supervised observation.

For a changed producer or environment, check tool discovery, the admitted
graph, formatting and Clippy before an expensive cohort. Follow a real
healthy/fault/restored selection through the intended reader and export.
Reuse a matching receipt when those inputs and consumers still match.

For a fixture metric stage on WSL, run the existing `metrics.py` lane with
`--durable-stage <stage-id> --deadline-monotonic <absolute-deadline>` on the
distro disk. Supply the existing graph, test, and any required inventory
receipts as usual. Use a fresh output label for each attempt and the same
stage ID and deadline for retries. The first attempt binds the deadline and
Linux boot ID under `durable-metric-stages/`. After a reboot or authorized
budget amendment, record the prior usage and remaining allowance before
starting the named successor. This route requires 6 GiB free at launch,
keeps the 4-GiB execution/export reserve, and leaves the original disk
workspace accessible after an interruption or incomplete export. A child
may still have output in its own buffers when interrupted. An incomplete
receipt cannot establish a passing score.

The configured Linux runner admits the observed core collector before abort
controls. File collectors use process RLIMIT_CORE 0; pipe collectors use the
Linux sentinel 1 that refuses before launching the collector. Both soft and hard
limits are checked. Socket collectors remain unadmitted. This changes no host
core_pattern, trust or protection setting. The matching WSL kernel source is
[Microsoft coredump_pipe](https://github.com/microsoft/WSL2-Linux-Kernel/blob/linux-msft-wsl-6.18.58.1/fs/coredump.c#L917-L942);
child native signal/core observations remain execution evidence, not a universal
crash-handler qualification.
