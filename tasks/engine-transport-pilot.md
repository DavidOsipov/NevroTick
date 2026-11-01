# Engine transport/runtime pilot

**Status:** [nevrotick-engine](../crates/nevrotick-engine/Cargo.toml) is now the
sixth main workspace member. The S25 114-input candidate passes the
configured dependency, complete GNU portable and Windows cross-build lanes,
plus twenty-four real GNU CLI cases and eight ordinary-user Windows CLI cases.
Its [main checkpoint](#main-workspace-checkpoint) and
[native checkpoints](#native-windows-cli-checkpoint) retain exact candidates and
the earlier owned-prototype evidence separately. Four additional public-API
consumer cases now observe socket release while their Windows children remain
alive. Native completion/heap diagnostics and the remaining qualification gates
are still open. This is the engine-only P0
experiment in architecture §18.6.1, preparing S2.1 under the
[development plan](../docs/plans/development-plan.md). It does not close P1/P2.

The [S27 checkpoint](#s27-terminal-outcomes-and-cargo-discovered-cli-consumers)
implements QF-01's private terminal-outcome coupling and adds Cargo-discovered
current-worker CLI consumers. Its 115-input candidate passes seven ordinary
tests, seven optimized tests and two doctests. S25's native/full-workspace and
S26's processing-unit results below retain their recorded candidates. The
[QF-03 fuzz checkpoint](s0.1-protocol-comparison.md#qf-03-variable-offer-fuzz-checkpoint)
now qualifies the variable-offer harness. Engine critical-file coverage/mutation,
affected Windows regression and the remaining native/resource gates are next.

The current [S29 Cargo baseline](#s29-engine-analysis-admission-and-cargo-test-layout)
binds 118 inputs after separating the API test binaries. Both default Cargo
profiles pass seven unit/integration tests plus two doctests; formatting,
compilation, Clippy and custody checks pass. This is a serial Cargo observation,
not engine coverage/mutation or ten-worker resource qualification.

## Authority and current inputs

Use architecture [§7.3](../docs/windows-time-architecture.md#73-ntp-client-behavior),
[§18.6.1](../docs/windows-time-architecture.md#1861-engine-runtime-decision-and-alternatives)
and §17.1; REQ-8, BND-1 and the applicable CHECK-6/21/25/27 subsets.
The [S0.1 task](s0.1-protocol-comparison.md#pending-ntpv4-negotiation-implementation--1-november-2026)
records the frozen 110-input pure negotiation checkpoint. Existing
`ntp-exchange` types carry explicit fixture assertions and a fixture timeout;
neither establishes native transport provenance or production limits.

The new owner is `crates/nevrotick-engine`, with the headless diagnostic
entrypoint `nevrotick-observe`. Tokio stays in this owner. Core,
broker, UI and helper gain no runtime dependency. UI-M0 remains unchanged.

## Dependency gate before compilation

The baseline proposal is Tokio 1.54.1, defaults disabled, with `rt`, `net`,
`time`, `sync`, plus getrandom 0.4.3 with defaults disabled and existing local
`ntp-wire`. It needs neither TLS nor `io-util`. Live primary version metadata
confirmed the Tokio identity; archive availability is not source admission.

Use Tokio's `macros` feature for `select!` in the serialized owner. The reviewed
compatibility choice is macro version 2.7.1: it uses the existing Syn 2.0.119
feature closure, including `full`, already selected by garde. Version 2.7.2
moves to Syn 3 and conflicts with the integrated duplicate-version ban; the
reviewed upstream change supplies no `select!` behavior repair. Enforce the
chosen version through the exact lockfile/checksum and frozen graph checks,
without an unused direct product dependency or private macro API calls.
Preserve the existing sixteen-package cohort and held TLS choices.

Resolve and acquire only in an owned external metadata proposal. Cargo metadata
may issue admitted compiler version/target queries; it must not compile
dependencies, run build scripts or execute proc macros. Record Linux/Windows
feature trees, duplicate trees, checksums and unexpected target/lock nodes.
The earlier 62-package crypto proposal does not prove this minimal graph.

Then review exact source/grants, new unsafe/native paths and build scripts,
refresh the configured advisory evidence, and admit the final graph before any
build. Minimal Tokio `net` still exposes native Windows I/O paths. getrandom
uses platform RNG mechanisms; its Linux fallback/file lifetime and Windows
`ProcessPrng` path need review. The metadata proposal is not a build, advisory,
licence, entropy or product-graph pass. The generated dependency-row configuration
already provides scoped reviewed allowances; do not weaken the root bootstrap
deny policy or add a blanket build-script allowance.

## Current acquisition evidence — 1 November 2026

The external evidence root is
`~/.local/share/nevrotick/s12e-composition-20261030T082648Z/`.
The baseline proposal and the selected `s24-udp-graph-macros-syn2-pin-v3`
proposal each completed ten metadata/tree commands: two explicit acquisition
rows and eight frozen offline rows across Linux GNU and Windows MSVC. The
selected proposal has twelve Linux and thirteen Windows external packages,
fourteen in their union. Its lock SHA-256 is
`f5b98f5ed71b38b374b400a0a346034abfedd5be26e30356aaf57543c2927c5e`.
All selected archives match that lock; the admitted compiler/tool inventories,
launcher, helper sources and main repository inputs remained unchanged.
No dependency was compiled and no build script or proc macro was executed.

The nine-package baseline source census compared 1,525 archive members against
extracted source and reviewed grants, getrandom's build script and libc's
selected build behavior. The macro review separately compared the actual 2.7.1
archive, its ten members and complete proc-macro source; the four shared
Syn/quote/proc-macro2/unicode-ident archives match the existing main lock.
These source observations do not replace graph-policy or native evidence.

`s24-engine-advisory-acquisition-v1/refresh-receipt.json` records the configured
fresh RustSec/index acquisition for the selected lock, with 3,317 inventoried
files and RustSec revision `2295466c4e9df26647fd67d8fa1c705ac2fc7f81`.
The whole snapshot matched its receipt, and owned/main inputs were unchanged.
The configured fetch also acquired the locked crate archives and extracted
sources: the snapshot includes sixteen archives, including two manifest-only
resolution dependencies (`r-efi` and `wasi`) outside the selected Linux/Windows
graphs. Their presence is acquisition, not runtime/build admission.
An earlier baseline receipt binds a different lock and is not substituted.
The first frozen per-target check passed its sandbox boundary control, then
stopped during Linux Cargo metadata: a vendor view of only the fourteen selected
packages omitted `r-efi`, which Cargo needs to resolve the full lock. No target
policy check passed. Preserve `s24-engine-dependency-check-v1/verification.json`;
complete that manifest-resolution view from the already acquired immutable
snapshot before retrying. No online fetch or policy relaxation belongs in the
frozen check. The corrected `s24-engine-dependency-check-v2` passed its boundary
control and both target policy rows. That fourteen-package result remains
separate from the later complete workspace result below. Cargo-vet
remains the optional review system described in architecture §18.8; no audit or
exemption is invented to clear this graph.

## Current implementation and verification

The source implements an explicitly controlled two-peer laboratory operation,
not a production source profile. A synchronous checked `prepare` owns OS entropy
and standard-socket setup outside the active window. Public synchronous `execute`
owns one private current-thread runtime with I/O/time enabled and two permitted
blocking threads; no public ambient-runtime entry point or raw handle bypass is
provided. At most two finite control timer tasks belong to that owner. Peer
requests and both control timers require checked private construction.

The CLI requires two explicit `--peer` addresses and `--timeout-ms`; omission of
the timeout is rejected before preparation. Its optional stop/invalidation timers
are experiment controls. The source review caught and removed an unintended
one-second fallback. The independent four-case omission/healthy-control harness
now passes on both loopback families: missing timeout returns code 2 with no
queries, while explicit timeout returns code 0 with one query to each peer.
The earlier version was not executed, so this correction has no behavioral RED
claim. Every result visibly declares `authentication=none eligible=0`.

`s24-integrated-engine-graph-v2` completed twelve actual metadata/tree commands,
including two unchanged five-member baseline rows and both integrated targets.
There are twenty-one selected Linux and twenty-two selected Windows external
packages, twenty-three in the union and twenty-five in complete lock resolution.
The integrated lock SHA-256 is
`416b349d87cc0e53562d79d494316fd20c210ede2e45c2568f5973652753ec8f`.
All original sixteen version/checksum records remain unchanged. Syn 2's features
remain unchanged; added Windows bindings follow the reviewed Tokio feature union
and confer no privilege or additional application caller.

The integrated configured refresh acquired and inventoried 3,756 files, including
all twenty-five archives, against the same RustSec revision listed above. The
complete source/grant projection compared 2,232 archive members with extracted
source and preserved the original sixteen admitted file-map hashes. The first
integrated policy attempt correctly rejected the new manifest's versionless
local `ntp-wire` path. Adding exact `=0.0.0`, matching existing local dependencies,
preserved the lock and bans. `s24-integrated-engine-dependency-check-v2` then passed
its boundary control and both frozen target checks with all custody bookends.
Its receipt SHA-256 is
`674441be23f98d83385bf96ed2e66e8974e7889d446c61fd0a607a000c0bcb57`.

Configured sandbox loopback round trips succeeded for IPv4 and IPv6 before
compilation; those are environment evidence, not application tests. The first
focused candidate, `s24-engine-host-focused-v3`, passed format/compiler checks and
ran all three `runtime_boundary` integration cases: two healthy neighboring
controls passed, while synchronous startup inside an entered Tokio runtime failed
the independent required-rejection expectation. A first-statement context guard
before entropy/socket operations repaired that bypass. The next candidate,
`s24-engine-host-focused-v4`, passed format/compiler and all three cases with no
skips, then stopped at twenty-four Clippy diagnostics. The next candidate passed
the three cases and library lint fixes, then exposed four missing CLI error
contracts and one missing assertion-test panic contract. After those repairs,
`s24-engine-host-focused-v6` passed all seven focused rows: boundary control,
format, compiler, all three regression cases, Clippy, actual binary build and
rustdoc generation. No doctest cases were selected. The old receipts remain
unchanged. Its GNU dev binary SHA-256 is
`8b038d6374e44f6ffac10d95b140d2fad99eaabf7027d7211a3529cb8b4ee613`.

The shared 2 GiB compiler preflight stopped an intervening launch before execution.
It was not weakened. Exactly two closed development targets were retired after
retaining all twenty-four native executable/shared-image paths with hashes and
relocation records; only regeneratable compiler intermediates were removed.
Sources, logs, receipts, admitted stores and the mapped main target were preserved.

`s24-engine-runtime-v1` then executed one hundred pure observer-sensitivity calls
(ten legal transcripts accepted and ninety intended rejections), separately from
twenty-four actual GNU-host CLI process cases. All passed in the configured
private user/PID/network namespace, with no Cargo/build commands or retries.
The four timeout-admission controls above and twenty independent IPv4/IPv6 cases
covered healthy replies, wrong source/origin, empty/47-byte truncation,
49/65,000-byte oversize followed by healthy traffic, stratum-zero control,
cross-slot replay, delayed expiry, invalidation and stop during invalid traffic.
The raw oversize observations retain both rejections and a subsequent stratum-one
header; stop retains a previously completed historical header beside the stopped
pending slot. Each selected peer saw one query per admitted operation. Missing
timeout saw none. The runtime receipt SHA-256 is
`0d8ef9112d2fa591c6610663ba884b7e4d4fadb9bb3510d67e554fe5307d5960`.

A delayed peer alone does not prove receipt-before-deadline/processing-after-
deadline rejection; that seam still needs a discriminating reached check.
Observed port release follows process exit and precedes fixture peer teardown;
it proves less than graceful in-process native teardown. Startup RNG bounds,
actual Windows AFD completion, resource qualification, applicable final
coverage/mutation gates and the full S2.1 obligations remain open. The
[main workspace checkpoint](#main-workspace-checkpoint) now integrates the
member, lock and permanent verification policy, retaining the exact twenty-three
selected versus two metadata-only source roles and actual CLI observations.

## Bounded implementation contract

- Admit explicit controlled loopback `SocketAddr` endpoints with nonzero ports
  and declared experiment budgets. No DNS, public defaults, delegation, polling
  or automatic retry. These are laboratory limits, not production policy.
- Own at most two associations, one pending query and distinct ephemeral socket
  per association. One serialized engine owner accepts completions against the
  saved endpoint/socket/request and current generation.
- Send one real minimal NTPv4 client packet with OS-generated correlation
  entropy. Preserve actual local observations separately from the wire token.
  RNG failure prevents sending; no caller-supplied fixture token substitutes.
- Bound input bytes, processing attempts and lifetime. Invalid source/origin,
  malformed and oversized traffic cannot consume a healthy pending query.
  Enforce processing-time expiry, not only receipt before the deadline.
- Stop must remain responsive and prevent old-generation arrivals from being
  newly committed. Completed slot reports are historical terminal observations:
  a later stop/invalidation retires pending slots without erasing an earlier
  accepted transport fact. Neither the returned report nor its CLI projection
  grants current confidence, sample eligibility or effect authority. Retain
  native resource custody through actual teardown; dropping a future alone is
  not native Windows completion evidence.
- Return bounded typed unauthenticated transport facts or control/expiry/stop/
  rejection/I/O outcomes. Stratum-zero traffic stops as control information.
  No result is an eligible sample, agreement, UTC-accuracy assertion, correction
  proposal or authenticated state. Timestamp placement reports local software
  observations, not an unmeasured kernel/wire arrival time.

No estimator, clock write, service installation, production IPC or UI integration
belongs in this pilot. OPEN-9/S0.7's protocol-constrained entropy exception remains
held for production qualification; laboratory traffic grants no ASVS exception.

## Observable exit and continuation

Run the actual executable against owned IPv4 and IPv6 loopback peers. Inspect
emitted packets and distinct sockets. Pair a valid reply with wrong-source,
wrong-origin, truncated, oversized, delayed and replayed traffic; exercise timeout,
stop and continued control responsiveness. Observe socket release before harness
teardown. Retain source/graph/tool identities, actual counts, first failures and
resource measurements. Host and native Windows evidence remain separate.

Full S2.1 still needs reviewed KoD behavior, DNS/native-job custody, production
eligibility/era/root-property contracts, accepted production bounds and native
port lifecycle. NTS adds its held provider/trust/exporter/endpoint gates. Continue
those owners without promoting this pilot into full acquisition qualification.

## Main workspace checkpoint

**1 November 2026.** Candidate
`83a13d0de0069c0fa7e5a6204cda617bfc725f570b4922cbc391f7dd833b5594`
binds all 114 product source/configuration/helper inputs. The external evidence
root above contains `s25-candidate-83a13d0d-source-v1.tar.gz`, SHA-256
`05be6ab3f36e1e7b88b898b5b9da2ec32bfaff2c51168056a704f06c4c460d5d`,
a lossless snapshot of those exact inputs. The main member append preserves every
other workspace/profile setting; all original lock records and sixteen source/grant
records remain unchanged. No staging, signing, publication or release occurred.

The dependency policy now explicitly distinguishes 23 selected registry packages
from two inert metadata-only resolution sources. Both complete source inventories
and actual per-target package/features are enforced. GNU selects 21 packages and
five reviewed build scripts; MSVC selects 22 and four. The root bootstrap deny
policy remains unchanged. Its new configured checks passed all five rows, including
the boundary control and separate metadata/cargo-deny rows for both targets.
`s25-main-engine-dependencies-v1.json` has SHA-256
`2f912de7b063688429a133b26480501677837bbc23fa3870e41890a2e843dacf`.

The complete portable lane passed all 22 native commands: **348 ordinary tests,
82 doctests and 274 optimized tests** across the six packages. Engine contributes
three runtime-boundary cases, two genuine construction doctests and three optimized
executions, including the actual entered-runtime rejection discriminator.
The CLI is checked, linted, built and retained before temporary build disposal.
`s25-main-engine-portable-v1.json` has SHA-256
`c3997c4f3eeeb63cae326befc8155db2582eb38e75e8f0f6e97d6276c466aa63`;
the retained GNU dev image has SHA-256
`82cd40f83cf6f98d0c47f3724de28decd193e91ca308a6439353fc0f651c8536`.

`s25-engine-runtime-v1/verification.json`, SHA-256
`dd4805bd1646487c614e64342cc3acfc811f6e22bee97efc9619d3399f0b9a0e`,
then passed the unchanged 100 observer controls and 24 actual process cases against
that newly exported binary. The body took 3.75 seconds, excluding admission and
bookends. Representative raw outputs were inspected: healthy peers each received
one query; IPv6 oversize recorded two rejections before a healthy header; invalid
traffic stopped with the neighboring historical header preserved. Every result
still says `authentication=none eligible=0`. All source/binary/vendor/tool/stable
custody bookends passed. This is controlled-loopback GNU execution.

The Windows cross lane passed all five rows, including separate UI and observe
CLI release links and complete prevalidation of both executable/PDB pairs.
`s25-main-engine-windows-v1.json` has SHA-256
`3920faa8458ff93437cf7bb20f65a6606cd78de74c5baa697dfb5d75e18440e5`.
The observe image has SHA-256
`8e0cfcd30186ae9a2c0d41ddcf59f3abf33384e5059eb6acf5ac96a1dfa09ffb`.
No native Windows execution is established by those cross-build rows.

Verifier/acquisition regressions preserved causal failures: the old verifier had
no engine owner or explicit retained CLI build; acquisition omitted metadata-only sources; an inherited
workspace alias bypassed the UI runtime-owner guard; and the portable producer
discarded its built image. Their repairs passed 95 verifier and 21 acquisition
Python controls. Those use declared fixtures/mocked native bodies and are separate
from the actual commands above. Pure fuzz source parity now follows only the
unchanged fourteen-package Garde closure, preserving its original 23-package
standalone lock and independent libFuzzer libc identity. Fifteen domain, twelve
exchange and 171 Core Python controls pass after intentional current binding updates.

The Core complete-product census was already stale at 106 after the earlier four
wire additions; it is now exactly 114, including those and the engine inputs.
The six domain and seven Core existing mathematical equivalences were substantively
re-reviewed for this source context before updating current declarations. Source
and proof bytes, exact fault identities and all thresholds remain unchanged.
The review and prior-policy archive are retained in
`s24-pure-equivalence-reuse-review-v1.json` and
`s25-pre-equivalence-rebinding-v1.tar.gz`. This grants no new equivalent fault,
mutation execution, score or current-candidate coverage/mutation pass. Historical
native receipts retain their actual original candidate.

The [native CLI checkpoint](#native-windows-cli-checkpoint) below supplies the
subsequent ordinary-user Windows execution. The
[live-process checkpoint](#native-live-process-port-release-checkpoint) supplies
the additional socket observation. Next work is the processing-time expiry seam,
fault/resource controls and the engine files' required coverage/mutation gates.
Native AFD completion/heap diagnostics, bounded startup entropy, full S2.1, NTS,
source eligibility, services, clock authority and packaging remain separate open
obligations. No full P0/P1/P2 or product/release completion is claimed.

## Native Windows CLI checkpoint

**1 November 2026.** The same main candidate `83a13d0d…` and all 114 bound inputs
were rechecked unchanged after the cleanup cycle. The actual Windows release
image remains `8e0cfcd3…`, bound to `s25-main-engine-windows-v1.json` above.
The admitted Microsoft-signed inbox PowerShell 5.1.26100.9818 ran the observer
on Windows build 26300 with the intended ordinary-user token. No execution-policy
bypass, service, clock, trust-store or installation change was used.

The preserved v3 run passed its eight transcript controls and first six CLI
cases. Its seventh child, `ipv6-stop-invalid`, exited normally with code zero:
slot 0 stopped after 19 invalid replies, and its neighbor retained a valid
header. The observer rejected a 54 ms final quiet interval against its declared
50 ms validity limit. That case has **no port-release credit** because rejection
preceded those probes; case 8 did not run. The actual cause remains unresolved:
the recorded observation could not distinguish delayed managed processing,
scheduling or a genuinely later process exit. This establishes no engine defect.
`observations-v3/verification.json` has SHA-256
`e5664ad1b2dd8a46064eaa7b0d44afa8c660727f7ab8271cbb239fa722e68115`.

Observer v4 polls for exit before capture and peer work, freezes its first
completed true poll and retains bounded operation timing. The 50 ms predicate
is unchanged. All **eight transcript controls and nine separate exit-order
controls** passed, including rejection of the retained 54 ms counterexample.
All **eight actual CLI cases** passed: IPv4 and IPv6 each exercised healthy
replies, two oversized replies followed by a healthy reply, stop during invalid
traffic, and invalidation. Each process exited normally with code zero; each
case independently rebound its two original ports after exit and before peer
teardown. Actual request bytes and representative output transcripts were
inspected separately from the summary. Every result remains
`authentication=none eligible=0`.

The v4 native receipt is
`C:\Users\perso\AppData\Local\NevroTick\s25-engine-native-f82dd739e54c\observations-v4\verification.json`,
SHA-256 `6fa131b1a8a699e76ead9c170ca3fc132b893647198bd3cd994e2cc14456dc54`.
The external evidence root also retains
`s25-native-engine-cli-v4-launch.json` and the unchanged v3 launch/failure.
V4 took 4.314 seconds inside the observer and 5.829 seconds for its launch.
Passing v4 does not identify the original v3 timing cause.

These are actual bounded native CLI observations, including **post-exit** port
reuse. They do not establish release while a process remains alive, completion
of all native AFD jobs, heap/handle retention, startup RNG bounds, steady-state
budgets, a one-hour baseline or a 72-hour soak. The external public-API consumer
under `s24-engine-live-port-fixture-v1` supplies the separate bounded lifetime
observation below. Full S2.1, held
NTS/trust integration, source eligibility, services, clock effects and packaging
remain open.

## Native live-process port-release checkpoint

**1 November 2026.** An external public-API consumer now exercises the exact
unchanged engine library from candidate `83a13d0d…` under the same release
profiles, lints, CPU/mitigation configuration and admitted compiler. Each of four
fresh processes calls `prepare` and `execute` **once**, preserving the documented
per-executable preparation contract: IPv4/IPv6 × healthy/stopped. The fixture
source SHA-256 is
`0cace70fe5629e5aea4ee6e116e9a39f2e2055bbbc2ff88026a4675690255bd1`.
It retains its returned report and control handle through an acknowledged live
checkpoint. This consumer is separate from the previously observed product CLI.

The standalone dependency graph is an exact reviewed subset: twelve selected
external packages on GNU, thirteen on MSVC, fourteen across both, and the same
two inert metadata-only resolution sources. Its lock SHA-256 is
`023224da48ab185d2a890bfce4755cc8e17ac4c851469bfd7e4ea57679cbc81a`.
The existing fresh advisory snapshot was explicitly reused with exact source/
version/checksum subset provenance; no new acquisition receipt was manufactured.
Offline resolution and both actual frozen metadata/cargo-deny rows preceded
compilation. All ten build rows passed, including format, Windows compiler,
Clippy and release link. All custody checks passed; the exported executable/PDB
were checked before and after retiring 162,504,704 allocated bytes of temporary
compiler output. No build row counts as a behavioral test.

`s25-live-port-fixture-windows-build-v2/verification.json` has SHA-256
`423b251372d316b0d80a43ba75934952e85ec41b3bd1d9fa850be6d6f5b57626`.
The fixture PE has SHA-256
`d41b14a4632cb1c1a81f1e3c0dc7ed9972fc88cfe0be6140600e4fa79d11d853`;
its matching PDB is retained alongside it. All 114 main inputs remained unchanged.

The independent Windows parent passed **35 helper controls**, kept separate from
the **four actual native cases**. Actual case receipts and raw ASCII streams were
reconciled independently: four distinct ordinary-user children exited normally
with code zero; no forced kill, stderr, cleanup or custody failure occurred.
The parent observed eight minimal 48-byte queries, four correlated healthy
replies and twelve exact ordered checkpoint acknowledgements.

Before each `execute` call, its eight exclusive bind probes failed specifically with
Winsock error **10048** against the two prepared endpoints per child. After
`execute` returned, its eight exclusive binds succeeded as four simultaneous
pairs while those exact children, reports and control handles remained alive.
It sampled the live child while holding each pair, then closed both probes before
acknowledging release. These are parent observations of the original endpoints,
not credit from the child's `expected=met` records or process teardown.

All twelve resource samples preceded their corresponding ACK. Handle samples
were **77 → 79 → 77** for the first child and **71 → 73 → 71** for the other
three. Private bytes remained **48–104 KiB higher** than their individual
baselines. Retain that increase as an observation; these fresh-process samples
establish no repeated-lifecycle retention or leak verdict. The observer body took
4.226 seconds; launch took 5.796 seconds.

The native receipt is
`C:\Users\perso\AppData\Local\NevroTick\s25-live-port-cb285560\observations-v1\verification.json`,
SHA-256 `d414efef6d8eaad2d180d872dd5ee0d6cccc2ffd64b94037f0e3203e60dbbf9a`.
The stage directory retains its initial preflight label; the actual source and
image hashes are the final values above. The external evidence root retains
`s25-native-live-port-v1-launch.json` and every individual raw stream/case receipt.

First failures remain available. The initial build recipe expected a nonexistent
projection field and stopped before Cargo/output creation. Its replacement
passed graph/format/compiler checks but Clippy rejected twenty fixture issues;
that attempt's error projection also omitted its command field. The exact failed
recipes, raw Clippy output and incomplete receipts remain. Repairs added the
required error contracts, preserved the report's post-ACK observability, and
corrected argument/error accounting without changing the thirteen-record success
protocol or lowering any lint. A read-only AST preflight also retained its initial
hidden-directory lookup failure before adding `Get-Item -Force`; the observer
itself already used that inspection flag. None was a behavioral engine RED.

This closes the bounded **live-process UDP port-reuse observation**. Complete AFD/
IOCP completion, native misuse/heap diagnostics, repeated recovery/retention,
startup entropy bounds, engine coverage/mutation, steady-state budgets and the
one-hour/72-hour qualifications remain open. The next implementation checkpoint
is deterministic processing-time expiry/current-control evidence through the real
consumer guard, then the remaining fault/resource and critical-file gates. Full
S2.1/NTS, production eligibility, services, clock authority and packaging remain
under their original plan requirements.

The coordinator receipt `s25-engine-main-native-checkpoint-v1.json`, SHA-256
`e2f21384551052ebf8b5d9e8f59ba1809f1a177b2e00ee1ace7915e1afc6b70a`,
binds the nine underlying admission/execution receipts, preserved incomplete
attempts, current source archive, actual counts and next action.

## S26 processing and current-control checkpoint

**1 November 2026.** Candidate
`5dcb333a01b216bbf6287fb956970bf3d776a10f3c2868a6cb45638ac2e6bb92`
binds 114 inputs. Only the engine library changed from S25: its complete original
production prefix remains byte-identical (`c08a2264…`), followed by one test-only
module. The formatted whole-file SHA-256 is
`2e4f42b59251e5aa36c73099b42aec058bd12af8c783d010a707734261cf1090`.

The aggregate unit reaches Pending through real bounded sends and reads using
owned loopback peers and distinct nonzero OS-generated tokens. It first commits
and snapshots the healthy neighbor, then processes the target's valid reply
through the actual `handle_wake` → `process_reply` consumer. Four independent
expectations cover expired processing despite earlier receipt, a future deadline,
delivered stop and advanced generation. Assertions check rejection accounting,
absence/presence of committed facts, and all saved neighbor clock/payload fields.
The fixture creates no clock hook or fabricated send/read provenance and does
not repeat public `prepare`.

`s26-engine-host-focused-v2/verification.json`, SHA-256
`a2d1582339be0259f3d5ab44c2c084307f0bd0a3c0dd1c67d76f7f9c4ff89f8d`,
passed all nine selected rows: boundary control, format, compiler, Clippy,
ordinary/optimized discovery and execution, and doctests. Each profile discovered
and executed the same **four tests**: the one aggregate library unit plus three
existing runtime-boundary integrations. The four logical unit scenarios are
not four additional test invocations. Both JUnit artifacts have no failure,
error or skip; both construction doctests executed. All four isolated nextest
namespaces observed owned IPv4/IPv6 loopback readiness. The body took 26.362 seconds.

The unchanged graph/license evidence was reused after checking every relevant
manifest, lock, feature/policy/configuration and complete 25-source vendor input.
The fresh advisory snapshot was reused without a new acquisition claim. Every
source, tool, vendor, advisory and owned-Cargo-home bookend passed. The first
attempt remains incomplete: Clippy rejected an overlong helper, and Cargo created
bookkeeping in an initially empty scratch home. Splitting only the test assertions
and seeding four exact already-admitted bookkeeping files repaired those causes;
the full Cargo-home equality check was retained. No tests ran in that attempt.

The terminal receipt and raw JUnit/logs were reconciled after the execution-tool
session handle became unavailable; no runner/compiler remained alive and no run
was restarted merely to recover its console output. Per-command exit codes are
retained in the receipt. A concurrent external commit/staged plan update was
preserved; this agent performed no staging or commit.

This is GNU execution of the named cases, with unchanged production bodies. It
does not discriminate exact-deadline `>=` versus `>`, inject a control change
during parsing, establish complete branch/critical-file coverage or qualify a
new Windows artifact. S25 native observations retain their actual identities.
The current plan's [QF-01](../docs/plans/development-plan.md#qf-01--couple-terminal-engine-outcomes-to-their-payloads)
now precedes the final engine coverage/mutation candidate. Keep test helper code
out of production coverage accounting; CLI mutation testing also needs a real
Cargo-discovered consumer of each worker's built CLI. The original 90%/95%,
native completion/heap/resource, S2.1/NTS and full product gates remain open.

## S27 terminal outcomes and Cargo-discovered CLI consumers

**1 November 2026.** QF-01 couples each private terminal outcome to its matching
payload, including native I/O with an optional platform code versus an anomalous
short send. Public getters, independent send/rejection history, one-way completion
and socket-drop ownership remain unchanged. This is a representational improvement;
no previously reachable production defect was established. Independent source review
compared all 35 public function declarations and the actual producer/report paths.

Private library tests now live under `tests/private/`, retaining their module names
and keeping test helpers outside the existing production coverage denominator.
The processing unit adds correlated control and rejected-origin/healthy-header
neighbors. A second unit checks three synthetic I/O projections and terminal history;
it does not establish actual operating-system failure or short-send branch execution.
Two integration consumers use Cargo's worker-built CLI and independent literal packet
and output expectations. The permanent verifier selects both library and integration
tests and rejects JUnit that omits the original processing guard. Its focused Python
repair passed **98 controls** after retained failing omission/selection controls.

The first S27 host candidate, `ae3ba63e…`, passed formatting and compilation, then
Clippy stopped before any tests: three private enum fields lacked documentation and
two test bindings triggered `similar_names`. Its incomplete receipt and frozen source
remain in `s27-engine-host-focused-v1/`; all custody checks passed. The repair adds
only those field descriptions and local-variable renames, without a lint allowance.
The second attempt reached the integration Clippy checks, which required an error
contract and two condition collapses; it also executed no tests. The third attempt
passed all static checks and executed seven ordinary tests: six passed and the
output-sink consumer failed. Its read-only descriptor fixture did not deliver a
write error through Rust's standard-stream wrappers. Five direct observations of
the same unchanged CLI image distinguished healthy output, both suppressed
bad-descriptor cases and both real broken-pipe cases. The exact broken-stdout
diagnostic/exit and broken-stderr exit already behaved correctly. This is a harness
repair, not a demonstrated production defect. The helper now closes the pipe's
sole reader before spawn and independently requires `BrokenPipe` from an actual
write; the original child exit/output assertions remain unchanged. The diagnostic
receipt `s27-cli-sink-controls-v1/verification.json` has SHA-256
`69317edf339b724c9d5dbf7b165b5a70f4ad3fbd7e1d9109b0c4b9c04ff92643`.

Final candidate
`be5734cfc20b1b18569ac3a66c15a51d4120e60ab3eee479d26bd17dd16189b5`
binds 115 inputs. `s27-engine-host-focused-v4/verification.json`, SHA-256
`fda7c1b00ef41d2cc044e8d798aea8dc4335accd7d16015af33cbacac7b05862`,
passed all nine selected rows. Both profiles discovered and executed the same
**seven tests**: two private library units and five integrations, including the
two current-worker CLI consumers. Both JUnit artifacts contain zero failures,
errors or skips; both doctests executed. All four nextest namespaces observed
owned IPv4/IPv6 loopback readiness. The body took **28.066 seconds**. All current/
owned source, index, tools, vendor, advisory and complete Cargo-home bookends
passed, with unchanged 512 MiB target and 2 GiB/1.5 GiB free-space guards. The
three incomplete attempts, their frozen sources and raw results remain separate.

These results establish the named GNU observations. Current Windows regression,
native completion/heap/resource and per-critical-file 90% line/95% mutation gates
remain open. The later [QF-03 checkpoint](s0.1-protocol-comparison.md#qf-03-variable-offer-fuzz-checkpoint)
records its completed bounded fuzz lane and **47 Python controls**. QF-02 remains at
its declared next accounting-recipe/counter-family change. Full S2.1/NTS,
production eligibility, clock authority, services and packaging remain ahead.
The later [118-input binding/context review](s1.3-source-selection-readiness.md#engine-test-candidate-binding-and-equivalence-context-reuse)
reconciles the analysis declarations without manufacturing another historical
coverage/mutation execution or new equivalence credit.

Before further compiler work, an authorized host-cleanup cycle removed 30 obsolete
npm temporary downloads and 259 compiler intermediates from four closed runs. Actual
net recovery was **556,806,144 bytes (531.0 MiB)**, leaving **3.35 GiB** available at
that checkpoint. Git HEAD/index/diffs/changed-file custody, 1,256 protected files,
the running Qdrant container and all 33 collections were unchanged. Executables,
native fixtures, profiles, source snapshots and raw verification records remain.
The external receipt `cleanup-cycle-20261101T1305Z-result.json` has SHA-256
`41c7fa02b8cbd4aa0488f5dc507850f86e163fd11ee51cf5473e55d5ef4c09ab`.
After the S27 checkpoint, 321 additional compiler intermediates from its four
closed attempts were retired, preserving all executables and verification inputs/
outputs. This recovered another **220,823,552 bytes (210.6 MiB)** net and left
**3.14 GiB** available after the intervening builds. Git and Qdrant custody and
all 1,208 selected protected files passed again. The receipt
`s27-closed-compiler-retirement-v1-result.json` has SHA-256
`3dc1243f3858c7911a10520d2fcd637c9a1f263be908adea3464875f37ca6393`.

## S29 engine analysis admission and Cargo test layout

**1 November 2026.** The new `scripts/verify-engine-analysis.py` and
`policy/engine-analysis.json` bind the two existing engine critical files through
the shared full tool-admission profile. Thresholds remain 90% executable lines
and 95% mutation per file. Enumeration is a separate bounded lane that retains
complete observed descriptors and cannot claim campaign or qualification success.
Mutation inventories start empty and exclusions are empty; campaign execution
requires reviewed exact inventories. The one-worker resource/readiness proposal
has not yet established ten-worker readiness.

Independent review found a command-admission race between the policy read and
Session snapshot. Three inert native-collaborator controls reproduced it: changed
product identity, copied input and copied policy each admitted a command when
zero were expected. The healthy neighbor passed. The repair checks all three
captured values before commands, inside final custody cleanup. All four unchanged
controls then passed; all **24 engine Python controls** and the combined **210
Core/domain/engine controls** passed. These are verification-infrastructure results,
not executed engine coverage or mutation gates. The original RED/GREEN receipts
are `s28-engine-snapshot-red-v1.json` and `s28-engine-snapshot-green-v1.json`.

The native Cargo baseline also needed a fixture correction before execution.
`prepare()` permits one call per executable run. Nextest's process isolation made
the S27 invocation valid, while Cargo's default libtest grouping would place two
successful startup tests in one executable. The three original API tests now live
in `prepare_release.rs`, `prepare_entered.rs` and `execute_entered.rs`; their names,
bodies and assertions are unchanged. The CLI integration binary retains both
existing CLI consumers and their full helpers. Each API test executable now has
one public `prepare()` attempt, while CLI children own their individual startup.

The formatter passed without changing the four moved-layout files. Its receipt
is `s29-engine-test-layout-format-v1.json`, SHA-256
`066e6e7426b2d649acc401d2c02ce6b1d79377af3942665ce973dfb3bcedf00c`.
The current 118-input product candidate is
`4b0841c3d056f1bf701555a7bb2a0ff99dccad4903303efcaef4a11e2a7afb9b`.
The actual default Cargo ordinary and release selections each executed the same
**seven unit/integration tests and two doctests**, with zero failures, ignored or
filtered cases. Raw output identifies the three separate API executables and the
CLI integration executable in both profiles. The empty CLI unit-test harness is
not counted as CLI behavioral evidence; the two real CLI consumers provide it.
All six selected rows passed: isolation control, format, compiler, Clippy and
both Cargo profiles. All source/index, vendor, advisory, tool/stable, complete
Cargo-home and engine-policy bookends passed. The receipt
`s29-engine-cargo-baseline-v1/verification.json` has SHA-256
`3e8975f70cdd1fe5528a4f8d725b928b495735aca0de872ab5c0f5bee3a3de59`.

With one Cargo build/test slot, the body took **75.170 seconds**. The sampled
target peak was **291,405,824 bytes** and minimum ordinary-user availability was
**1,943,072,768 bytes**, above the retained 1.5 GiB stop guard. These observations
do not qualify aggregate mutation-worker disk/RAM use or ten-worker readiness.
The current low storage margin must be resolved before a ten-worker campaign;
actual descriptor enumeration and critical-file coverage/mutation remain next.
S27 and QF-03 receipts retain their own exact candidates. Native Windows,
completion/heap/resource, TLS and complete P0–P7 remain open.
