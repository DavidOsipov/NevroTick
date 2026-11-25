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
digests. Exit 3 means rename completed with unconfirmed durability, path
or descriptor cleanup. Reconcile the retained tree before attempting
another output.
The [S0.6 task](../../tasks/windows11-integration.md#publish-the-disabled-service-pack-inputs-as-one-tree--22-november-2026)
records the pinned inputs, independent archive controls and Windows gates.

Run the payload publication and unsigned package reader regressions with
`python3 -I -B -m unittest discover -s tools/windows-service-fixture/msix`.
The reader tests build inert archives and check the CLI's digest, payload, ZIP,
block-map and XML decisions without installing a package.

The Windows [archive reader](msix/prestart_archive_facts.cs) measures the
package full name and the lengths and SHA-256 digests of both fixed service
members. Supply bounded bytes from the admitted pack or signed-artifact
readback and its independently expected archive digest. Inspect checks a
private snapshot and gives those same bytes to the Windows Appx reader.
The [native controls](msix/test_prestart_archive_facts.ps1) use two previously
checked unsigned packages with the same manifest identity and different
service bytes. Signature and signer verification remain separate from
these archive observations.

For the held §7.5 pre-start probe, the
[installed identity entry](msix/observe_installed_identity.ps1) uses the
[companion module](msix/installed_identity.psm1) for the two fixed laboratory
roles. The window owner must hold the 64-bit Windows PowerShell interpreter,
entry script and companion module under the admitted deployment user. It supplies
`RunId`, `ExpectedUserSid`, `PackageFullName`, `SignedPackageSha256`
and each role's `Bytes` and `Sha256` values using the `Engine` and
`Broker` parameter prefixes. Derive them from independent inspection
of the exact final signed lab artifact. The entry writes one bounded
BOM-free UTF-8 schema-2 line to raw stdout after both role and registration
checks and both image handles have been released. Retain native errors
privately. [Identity controls](msix/test_installed_identity.ps1)

For owned capture, `Invoke-PrestartIdentity` in
[the child module](msix/prestart_child.psm1) uses the same suspended process,
token, deadline, job, and retirement checks as the fixed observers. Open the
archive with `Open-PrestartArchive` using its independently admitted length
and SHA-256. Pass the returned `Archive` lease, the original clock and timeout,
and the independently admitted carrier digest as `ExpectedArchiveSha256`.
The launcher derives both role arguments and the child identity from the
held archive. For the signature-gated lab capture, also supply
`ExpectedSignerSha256`, the independently admitted SHA-256 of the signer's
DER certificate. This selects Windows signature verification with the held
archive handle and path, no interactive UI and cache-only URL retrieval.
The current lab profile selects one primary signature with zero secondaries.
The verification and child launch share the original actor, lease and
deadline. The archive retains the time remaining at admission on a private
monotonic clock. Use the unreset original clock and keep the lease through
collection and actual retirement. The owning process must bound the
synchronous verifier and retire on unconfirmed verifier cleanup. Early disposal revokes the capture
and the child retains the file until retirement. After release, the
collector rechecks the original archive under the same lock as
withdrawal before publishing the capture. Omitting the signer pin
selects archive observation for inert controls. The installed window owner
still owns role authorization, continuous quiescence and installed admission.
Precompile
[`prestart_image_path.cs`](msix/prestart_image_path.cs) as the x64 .NET
Framework library `prestart_image_path.dll` before launching the identity
child. Supply the approved original System32 PowerShell path and the lengths
and hashes for the interpreter and all three files in the owned staging
directory. The entry imports the required cmdlets from the interpreter's
Windows module directory. Capture preserves both raw streams, original PID,
and observed exit code. Owned identity and observer captures also report the
two SHA-256 observations of the held companion module or observer executable,
respectively. They leave actor and run admission to the window owner.

The [pre-start coordinator](msix/prestart_supervisor.py) supplies the fixed
observation sequence through `supervise_prestart`. Its Windows window owner
must admit the independent signed package and role members, keep the original
deployment window current and bind each capture to the retained child process
and its independently observed effective token. The deployment context
supplies identity and the two-role observation. The proposed provisioning
actor and standard user supply their own access captures. The coordinator
validates each receipt before the next selector can start. Before either
access capture, it checks the observation's raw SCM types against the
ordered Engine and Broker baseline supplied by the original window. An invalid child
receipt retains precedence over a later window refusal. After the last
currentness check, the coordinator returns the observation only when the
owner confirms retirement of that run. It preserves the first failure
and any independent retirement failure. The native window owner must
still be wired before the installed probe can execute.

Use `supervise_deployment` for the two deployment captures alone. Its
window supplies the original run and opening SCM sample without supplying
access actors. The coordinator checks run identity before each dispatch
and after receipt validation, and returns only after confirmed retirement.
The full pre-start result still requires both access captures. The native
adapter must keep the original window live through the final currentness
check and settle the same attempt at retirement.

The deployment owner can use `Open-PrestartScmWindow` to hold the two fixed
lab service objects in one Windows process across point checks. Supply the
retained `Actor`, the independently admitted canonical `ExpectedDeploymentSid`
and the original running clock and timeout. The owner verifies that actor
before each SCM operation and before returning a point sample. `Initial`
retains the opening samples. `Check()` reads both original handles and
permanently refuses the window after actor withdrawal or any failed observation.
Inspect the result of `Retire()` before accepting the enclosing operation.
It preserves an earlier failure and attaches later close errors. Complete
native owner retirement before package cleanup if close is unconfirmed.
The samples supplement the separate signed-artifact, actor and continuous
quiescence evidence. [Paired SCM controls](msix/test_prestart_scm_window.ps1)

Use `Invoke-PrestartIdentity -Window` with the original SCM window, held
archive and independent archive and signer pins. The window supplies its
actor, SID and clock, verifies the signature, and keeps the original lease
and pins for the fixed observation. After the coordinator validates that
identity receipt, use `Invoke-PrestartVerifiedScmObservation` with the same
window and the independently pinned observer image. The window permits
one attempt for each child and requires a successfully released identity
capture before the second. The enclosing deployment owner supplies
continuous quiescence, separate actor admissions and retirement of the
full attempt, including children whose cleanup is unconfirmed.

After validating both receipts, call `ConfirmDeployment()` on the original
window before `Retire()`. Confirmation requires the original two successful
child captures. It samples the same SCM handles and rechecks the archive
and actor with the retained deadlines. The enclosing owner holds the window
and archive through this check and explicitly confirms retirement before
returning the joined result.

For the paired private-pipe route, use
[`supervise_deployment_session`](msix/prestart_deployment_session.py) with the
admitted run and the original process owner's pipe ends. The dedicated
Windows process invokes
[`Invoke-PrestartDeploymentSession`](msix/prestart_deployment_session.psm1)
as its final action with the original window, archive, actor and deadlines.
The session requests native confirmation after both receipt checks. It
returns only after a clean retirement reply, pipe EOF and independently
observed process and job retirement. The outer launcher must supply that
process and job custody and enforce its recovery deadline.

The [Windows guardian](msix/prestart_deployment_guardian.ps1) launches the
admitted `prestart_deployment_owner.ps1` inside its private job. Keep the
entry and its transitive inputs in the admitted local staging directory.
Provide its owned `prestart-temp` directory and load the pinned x64
`prestart_deployment_helpers.dll` with `Import-PrestartDeploymentHelpers`
before opening the actor or window. The guardian's first cutoff covers the
entire cooperative owner lifetime, including wire retirement. Its second
cutoff bounds forced recovery. The WSL caller supplies private stdin and
stdout, retains separate diagnostics and observes the original guardian
through `GuardedDeploymentProcess`.

For an independent cutoff, build the x64
[`prestart_deployment_watchdog.exe`](msix/prestart_deployment_watchdog.cs)
from its entry source and the same five C# sources used by the guardian.
The watchdog holds the original guardian in an outer Windows job and
owns its work and retirement cutoffs. From WSL, call
`launch_deployment_watchdog` with the independently pinned executable,
guardian, interpreter and owner, then pass the original process through
`GuardedDeploymentProcess` to the receipt supervisor. Keep the local
staging paths and transitive inputs protected through use and direct
diagnostics to a separate owned regular file. On expiry the watchdog
requests termination and checks original exit and job accounting. An
unconfirmed recovery withholds the result and requires the lab owner to
retain recovery authority.

Render the fixed `prestart_deployment_owner.ps1` with
[`render_deployment_owner`](msix/prestart_deployment_owner.py) from the
pinned [entry template](msix/prestart_deployment_owner.ps1.in). Stage the
returned bytes under that filename and give their returned pin to
`launch_deployment_watchdog`. Stage the pinned archive as
`prestart_fixture.msix`. Keep the three installed-identity inputs in
`identity/` and the two modules, deployment helper and observer beside
the entry. Use the SID and launch token for the same independently
admitted deployment context represented in the supervisor's run. Protect
the staging namespace through retirement.

The owner keeps the checked modules open, loads the pinned helper, and
opens the actor, archive and original SCM window under one clock. It
passes them to the existing typed session for both captures and cleanup.
Signed-archive admission, continuous quiescence and the separate access
actors remain prerequisites of the installed probe.

For the full handle-only sequence, first use
[`render_prestart_access_owner`](msix/prestart_deployment_owner.py) with
the base `DeploymentRun` to render each fixed role entry. Stage each
as `prestart_deployment_owner.ps1` in its own protected directory with
the four pinned inputs and private `prestart-temp`. Use the existing
watchdog from each independently admitted primary-token context. The
native role entry checks the expected SID before the observer starts.
The standard-user path also requires the standard token profile.

Use [`launch_prestart_session`](msix/prestart_deployment_session.py) with
the independently admitted deployment run, three fixed role launch attempts
and a fresh private `PrestartCaptures` instance. The coordinator starts the
two access owners and puts their original process references in the full
`Run` for the deployment launcher with `full_prestart=True`. It checks all six pipe
ends and the work cutoff before handing off to the existing receipt
supervisor. If acquisition or admission fails, it signals every
attempted owner before checking settlement with the original retirement
cutoff.

Each independently admitted launcher retains its actual context, process,
job and private diagnostics from before activation through confirmed
retirement. This includes an attempt that raised before returning a
process. The caller supplies the original absolute deadlines before the
first launch, and the external custodians enforce them across launch and
idle waiting. The watchdogs' own relative clocks start at their process
entries.

The deployment window checks its original SCM objects before each access
probe. Each role accepts its one fixed command followed by input EOF. The
supervisor checks the resulting receipt and the original role's EOF and
retirement before progressing. Final confirmation follows both valid
captures. A refusal prevents later captures, closes the access pipes and
settles the original deployment owner. The default paired path retains
its two captures and final confirmation. The full installed run requires
the actual eligible role processes, signed artifact and continuous
quiescence.

The lab owner retains `PrestartCaptures` for the attempt and examines its
private `records` after success or refusal. Each decoded original frame is
bound to its probe and original pipe owner. Preserve the exact bounded
stdout and stderr as local diagnostic evidence, including the access
target, mask, stage and raw error on a refused probe. The joined result
still requires receipt validation and retirement. Worker refusal frames,
undecodable data and replies drained solely for retirement supply no capture.

When rendering an access entry with `private_capture_diagnostics=True`,
the launcher retains its original distinct private regular stderr file.
If the child captured both streams and exit but native cleanup or reply
delivery fails, the opted-in entry can add one bounded
`nevrotick-prestart-access-capture` line after the redacted failure. Preserve
the file under the original role and process through readback. After the
custodian confirms that its writers have settled, call
[`read_private_access_diagnostic`](msix/prestart_deployment_session.py) with
the original retained file, admitted run and fixed probe. It checks the run,
role, observer pin and bounded streams without reopening a path. Input
digests can be absent if confirmation refused. A complete diagnostic
records historical bytes even after a failed native exit. Receipt admission
and retirement still follow the original process and protocol.

A launcher that retains the full native watchdog exit status can select
`observe_access_settlement=True` with a fresh private read/write file.
The watchdog reports exit `3` only for an original nonzero guardian exit
followed by observed empty outer-job accounting, with no outer recovery
attempt or recorded outer cleanup failure. This settled refusal permits
private diagnostic readback after independent writer settlement. The
ordinary receipt supervisor still requires success. A WSL `Popen`
return code can lose the high bits of a Windows status and is insufficient
to establish the native exit `3`.

[`PrestartAccessAttempt`](msix/prestart_access_process.py) implements the
existing pre-start launch interface for one independently admitted access
actor. Render the role with `render_prestart_access_owner` from the original
run, admitted SID and intended protected stage. Stage its exact `contents`
at the returned `path`, and supply that same rendered owner to the attempt.
The coordinator checks the two original role entries before either launch.
Each attempt checks its entry again at launch. The lab owner supplies the
trusted renderer inputs and protects the staging directory throughout use.

Build [`prestart_access_parent.exe`](msix/prestart_access_parent.cs)
with the same shared x64 sources as the deployment watchdog and select
`NevroTick.PrestartAccessParent` as its entry. The access attempt selects
the fixed supervision mode and supplies the pinned parent image and the
independently admitted subject SID. The outer process compares that SID
with its retained primary token and checks the ordinary-user profile for
the standard role before starting any child. It creates the original inner
parent suspended in its own job and then
verifies the held image, job membership, x64 process and actor before
resume. The inner parent runs the pinned watchdog in the same primary-token
context and passes through the original protocol pipes. The watchdog
writes diagnostics to the original staged `prestart-access-diagnostics.log`.
The fixed PowerShell access owner acquires its query-only primary-token
reference before reading the request. It lends that same actor to the native
observer for currentness and role checks at launch. It disposes the actor
before a capture or refusal frame, including cleanup after unused EOF or
malformed input.
The host owner holds the empty file on a read-only descriptor from before
launch and independently admits its staging directory and transitive inputs.
The provider checks the descriptor's OS access mode. The native parent owns
the writer and opens the file with read sharing.

The outer owner keeps the inner exit record on a private pipe until the
original inner process has exited zero, the outer job is empty and the
private stream reaches EOF. It reserves time to terminate and observe its
job when the inner parent stalls. Failed or forced retirement withholds
the native record. The host checks the original relay exit, full watchdog
exit and matching diagnostic bytes. A settled watchdog exit `3` preserves
refusal and permits `private_diagnostic()` after retirement. Only watchdog
exit `0` can complete a successful access capture. Rollback closes input
first and drains a bounded remainder of the original output. If the drain
refuses, the host retains the refusal while observing the original relay
within its remaining deadline. The [live WSL fixture](../../tasks/windows11-integration.md#settle-the-original-access-relay-through-wsl)
exercises native settlement under the current actor with inert receipts.
Cross-context actor launch and signed installed execution remain open.

If the private pipe fails before retirement can be acknowledged, the fixed
owner exits unsuccessfully and reports the first operation refusal, transport
failure and cleanup codes on its separate diagnostic stream. The
[PowerShell controls](msix/test_prestart_deployment_session.ps1) exercise the
session with closed streams and controlled retirement outcomes.

The [Windows child collector](msix/prestart_child.psm1) reads bounded raw
stdout and stderr from the launch owner's original `Process`. It requires
both EOFs and observed exit under the supplied monotonic deadline. The owner
retains process containment and must settle any outstanding reads attached
to the caught exception. The control script exercises this component with
fixed inert children in 64-bit Windows PowerShell. It performs no package
installation or service operation. [Native controls](msix/test_prestart_child.ps1)

On refusal the owned collector records a termination request error separately
from the primary collection error. It observes the original process, job and
pending reads before disposal. A failed observation or a refusal before release
begins retains the original child for recovery in this process. The
`PrestartObserverChild` exception entry is a one-element reference array.
`DisposalStarted` describes entry into release and does not confirm release.
The [controlled retirement cases](msix/test_prestart_retirement.ps1) use the
real collector with controlled streams and child state.

The collector also supplies `Invoke-PrestartObserver` for the three fixed
SCM observers. Use a dedicated 64-bit Windows PowerShell process with
serialized child launches and an exclusively owned local input directory.
Each native observer launch captures its remaining time from the original
clock. Job creation, suspended process creation, resume and capture
publication after retirement each recheck that cutoff. Keep the original
clock running and retain the enclosing window deadline across launches.
Use `Open-PrestartObserverActor` to retain the launcher primary token and
pass that actor as `-Actor`. Dispose the actor when its owner finishes
observing that context. Role eligibility remains a separate input to the
installed window.
Supply the approved `nevrotick-prestart-observer.exe`, its exact length and
SHA-256, the selector, and the original running Stopwatch and deadline.
For the provisioning access selector, supply `-ExpectedProvisioningUserSid`
from the window owner's independent account admission. Before creating a job
or process, the launcher requires an exact match with the current primary
token's user SID.
For the standard-user denial selector, also supply `-ExpectedStandardUserSid`
from the window owner's independent account admission. Before creating a job
or process, the launcher requires that SID to match the current primary token,
with default elevation type, no elevation, and no Administrators SID in the
token groups, including deny-only membership.
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
fixture metric consumer accepts a current single-command `fixture-test`
receipt or the ordinary fixture test result from the full checked
`privilege-checkpoint`. `metrics.py coverage` selects host, error, diagnostics,
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
