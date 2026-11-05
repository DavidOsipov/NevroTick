# Engine transport/runtime pilot

**Current S2.1 preparation, 5 November 2026:** the [integration card](#s21-preparation--real-transport-to-an-admitted-time-measurement--5-november-2026)
separates the next experimental real-UDP measurement from later uncertainty and
actual Core admission. W11-W01 is partially prepared; the [S1.1c proof-first
queue](../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent)
remains first. Historical engine receipts below retain their original candidates.

The [address-family continuation](#address-family-admission-and-bounded-acquisition-progress--5-november-2026)
adds local-path admission and bounded alternative progress to later S2.1/S2.2
acquisition. Its implementation and native qualification remain pending.

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

The [S29 Cargo baseline](#s29-engine-analysis-admission-and-cargo-test-layout)
binds 118 inputs after separating the API test binaries. Both default Cargo
profiles pass seven unit/integration tests plus two doctests; formatting,
compilation, Clippy and custody checks pass. This is a serial Cargo observation,
not engine coverage/mutation or ten-worker resource qualification.

The historical [S32 contract checkpoint](#s32-focused-contracts-and-verified-cargo-checkpoint)
adds six focused cases. Its 118-input candidate passes all thirteen tests and two
doctests in both Cargo profiles, with compiler/Clippy and all custody checks.
Its [line-coverage gate](#s33-current-engine-line-coverage) passed on that
recorded source; later qualification is candidate-specific.

The current [S40 contract checkpoint](#s40-mandatory-completion-owners-and-current-native-baseline)
passes 24 GNU unit/integration tests plus two doctests in both Cargo profiles,
compiler/Clippy and 213 verification-harness controls. Fresh enumeration lists
185 library and 31 CLI variants. Current line coverage and both complete
[per-file mutation gates](#s43-complete-current-engine-mutation-checkpoint) pass.
Earlier receipts retain their original candidates.

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

## S30 inventory and S31 first engine coverage

**1 November 2026.** On the unchanged S29 118-input candidate, the configured
enumeration lane completed both native commands with exit zero and full
tool/source/vendor custody. It retained **176 library and 34 CLI descriptors**,
210 total, without exclusions. The normalized complete inventory bindings are
`28d483d667bb05732d6649739571e145ccb2f5519d3367d1c74ae2441a0c5455`
and `bbcd99c0016069f8be87fb242bd0205d9658e27de7559322aaf046667df3d673`.
The receipt `s30-engine-mutation-enumeration-v1/verification.json` has SHA-256
`4b847627286ed1a344e686801ad805f19617f358a207f0aec108647063663d90`.
The jobs=10 warning remains in both raw listings. Enumeration grants neither
mutation kills nor a campaign/resource qualification.

Two independent reviews reconciled every raw descriptor field and identified
two absent constructors: `Config::new` and `ControlSchedule::new`. The pinned
[generator explicitly skips methods named `new`](https://github.com/sourcefrog/cargo-mutants/blob/v27.1.0/src/visit.rs#L425-L445).
Its [macro limitations](https://github.com/sourcefrog/cargo-mutants/blob/v27.1.0/book/src/macros.md)
also leave async macro bodies outside generated faults. Constructor boundary
tests and real control/transport trajectories supply separate evidence; native
Windows, dependency internals and macro expansion retain their own obligations.
Ungenerated behavior is never added to kills or equivalent-mutant exclusions.

The first actual coverage command compiled successfully and passed all seven
required unit/integration tests, then exited **1** at the unchanged file gate:
library **613/741 lines, 82.726%**; CLI **86/86 lines, 100%**. The retained native
JSON, raw test output and receipt identify this as a coverage failure rather than
a test/setup failure. `s31-engine-coverage-v1/verification.json` has SHA-256
`8dd7afd0557bce1eb3859165d9ecce5985b7513fc1cb9fd36b4555c3abe76749`.
All final full-tool custody checks passed. Missing constructor, control-generation,
public observation and redacted-failure cases now drive the next focused test
change under REQ-8/BND-1 and the applicable CHECK-6/21/25/27 subsets. The required
environment remains the admitted isolated GNU host graph and controlled loopback
peers; no threshold, production behavior or native-Windows claim changes.

## S32 focused contracts and verified Cargo checkpoint

**1 November 2026.** The current 118-input product candidate is
`4cb124a75adcfa6f6b83d3ed07d6b70dcbdd032a7e1754085b16cdf519456470`.
Only `tests/private/processing_guards.rs` changed from S29's product map;
production source, manifests, dependencies and runtime policy are unchanged.
Its SHA-256 is
`8a8d20d6ad68cf400cdf3722f29f80e5f22b50b69bc286222e325355b4cde162`.

The six added tests establish independently expected constructor rejection and
legal neighbors (including IPv6 scope/flow and exact timeout limits), stop latching
and checked generation exhaustion through real watch delivery, distinct fixed
redacted error projections, immediate timer delivery and actual cancellation/reap,
and seven hostile literal-header variants followed by the original real UDP
healthy result. Existing checks also compare public timestamps, endpoints and
facts with observations saved before later control. Synthetic near-maximum
generations, operation errors and the deliberately reversed software observation
are explicit representation controls; they do not claim induced OS failures.

The analysis receiver now requires all thirteen actual unit/integration names.
A healthy thirteen-case transcript passed while the prior receiver incorrectly
accepted each of six omitted-case transcripts; adding the six required names
made all unchanged omission controls reject. All **25 engine Python controls**
pass. The final combined **211 Core/domain/engine Python controls** pass under
`s32-binding-python-controls-v2.json`, SHA-256
`84929968283918192b72e913a5601560d075aa40b42a49196425fcd9f14f3171`.
The thirteen existing Core/domain equivalence arguments were independently
re-admitted for the tests-only context change: source, mathematical arguments,
proof bytes and descriptors are unchanged. Historical mutation outcomes remain
historical; no Rust campaign was replayed by this binding update.

The first native attempt passed isolation, formatting and compilation, then
Clippy rejected a similar helper name and a 101-line helper against its 100-line
limit. It executed no Rust tests. The retained receipt is
`s32-engine-cargo-baseline-v1/verification.json`, SHA-256
`4df486c55b1a166218edcb164263bb3bfb13cb705f47894bc86ab65ee49a8473`.
A clearer buffer initializer and one local rename repaired both without lint
suppression or changed expectations. The final formatter made no further change;
`s32-engine-contract-tests-format-v2.json` has SHA-256
`cd0102abcbaa930d9da71e661a481b30e54550f195dff2822b9a4006cf9dd914`.

All six final native rows pass: isolation, format, compiler, Clippy and the default
Cargo ordinary/release selections. Each profile executed the same **thirteen
unit/integration tests plus two doctests**, with zero failures, ignored or filtered
cases. All nine source/index, Cargo-home, vendor, advisory, tool/stable and policy
bookends pass. `s32-engine-cargo-baseline-v2/verification.json` has SHA-256
`b32285e456d9d0e81fc4297c1da0d8fae543b2967216cfcc87b79a5035e513bd`.
Its serial native body took **81.939 seconds**, with target peak **293,027,840
allocated bytes** and minimum ordinary-user availability **17,451,433,984 bytes**.
This remains one-worker evidence, not aggregate ten-worker readiness.

The removed historical directory also removed the old execution lock. Current
Root executions serialize on `s12e-composition-20261030T082648Z/execution.lock`;
no historical receipt is rewritten or deleted directory reconstructed. The active
vendor, admitted tool payloads and current receipts remain required inputs.
Next: current-candidate line coverage, then complete per-file mutation with actual
worker-resource observation. Constructor and macro alternatives, real native
failure/completion, Windows diagnostics, fairness and retention remain distinct
obligations; neither this checkpoint nor a generated score closes P0–P7.

## S33 current engine line coverage

**1 November 2026.** The admitted coverage lane passes on S32 candidate
`4cb124a75adcfa6f6b83d3ed07d6b70dcbdd032a7e1754085b16cdf519456470`:
library **684/741 executable lines, 92.308%**; CLI **86/86, 100%**. All thirteen
required unit/integration cases actually passed; this coverage selection excludes
doctests, which passed separately in both S32 Cargo profiles. The original 741/86
production denominators and 90% per-file threshold are unchanged. Native JSON
reports zero branch records, so these results establish no measured branch gate.

The native command exited zero, and final full source/vendor/tool admission
passed. `s33-engine-coverage-v2/verification.json` has SHA-256
`5c18d5140a83daf9dfb898508b1224609cbb2ad82676cc730995e443db3bc628`;
the raw `work/coverage.json` has SHA-256
`202a48ba7da835d6c3d309bca80f831238c386f7931da8e59002d1fad9b4d0a8`.
S31's failed attempt remains retained. The complete CLI mutation gate has now
started at `s34-engine-cli-mutation-v1`, with ten configured workers and passive
whole-work disk/RSS/process observations. Its terminal outcomes and actual
concurrency are pending; this launch grants no score or ten-worker pass.

## S34 CLI mutation and observed ten-worker load

**1 November 2026.** The complete CLI campaign on S32 candidate `4cb124a7…`
retained all **34 generated variants**: **16 caught, 11 missed, seven unviable,
zero timeouts**; successful baseline: thirteen unit/integration tests and two
doctests. The raw/adjusted score is **16/27, 59.259%**, with zero exclusions.
The native tool exited **2** and the configured receiver exited **1**, correctly
leaving the custom gate incomplete. All final source/vendor/full-tool custody
checks pass. `s34-engine-cli-mutation-v1/verification.json` has SHA-256
`6fdf340adcd5a2220b16bd3afec9e6b9eb736d440b5a74295102c5c83414a4b5`.
All seven unviable logs contain actual compiler trait/ordinary-lint errors;
none is credited as a kill. The campaign's native interval was 29.496 seconds.

The passive observer recorded **ten concurrent Cargo processes**, peak **191
namespace threads / 42 processes**, and the effective **512-process** limit.
Summed observed RSS peaked at **4,239,300 KiB**; host MemAvailable stayed at least
**7,195,260 KiB** and ordinary-user disk availability at least **15,599,632,384
bytes**. Largest sampled work allocation was **1,668,333,568 bytes**, but that
sample was partial; the largest complete snapshot was **782,811,136 bytes**.
Sampling cannot establish a hard aggregate maximum. There were 23 missing-RSS
process samples, 24 executable-read gaps, 16 disk-read races/errors and unreadable
unattributed processes. The observer completed without exceptions, window or
scan-bound exhaustion; `observation_has_gaps` remains true. It sampled for
119.264 seconds including admission/closeout, with 109 matched-executable samples.
`s34-engine-cli-mutation-resources-v1.json` has SHA-256
`4ed32c20f096842aa3fc5ab35d885a75c08f89e39bb35457e2255e52155a9651`.
These observations support retaining ten configured workers for the next bounded
engine check under fresh headroom admission; they do not establish product
resource/soak qualification or a successful mutation score.

Independent triage found five concrete missing process witnesses that distinguish
six variants: an exact eight-digit duration rejected by Config rather than syntax,
a malformed third peer, repeated stop/invalidate options, and an exact 128-byte
value. These are being added with parent-owned loopback peers. Three other missed
variants have potential full-prefix equivalence arguments; none is adopted.
Two variants bypass early option-size rejection yet eventually return the same
error after processing an oversized key. Same exit code is insufficient evidence
for their bounded-work obligation. The next source slice gives pair and text
budgets explicit mandatory owners with direct boundary controls, followed by new
source admission, enumeration and actual gates. S32/S33 remain exact-candidate
historical passes while this newer source is under development; no exclusions or
threshold relaxations close the S34 failure.

## S35 explicit CLI resource owners

**1 November 2026.** The current **119-input** product candidate is
`295bd1708a70b577148aed66282722ec7a2eeeb606a2892181e8e89ffc5f283e`.
Relative to S32, the CLI and runtime consumer changed and
`tests/private/argument_guards.rs` was added. The engine library, existing private
processing controls, dependency graph, compiler/lint profiles and native paths
are unchanged.

The CLI now must charge a private six-pair allowance before decoding each option
or fetching its value. Checked exhaustion preserves zero on repeated failure.
A closed option/value role selects mandatory 32/128 encoded-byte admission before
UTF-8 conversion; the parser consumes the returned borrowed text. Tests cover
six legal charges/seventh rejection, exact byte neighbors, multibyte text and
original borrowing, plus Unix nontext rejection. They live outside production
coverage files. Five additional real current-worker CLI invocations exercise
exact duration/value boundaries, malformed third-peer ownership and repeated
stop/invalidation, using parent-owned peers for potentially accepted fault cases.
The grammar, finite safe errors and existing limits are preserved; no new
equivalence exclusion or relaxed threshold is adopted.

The receiver requires all sixteen GNU unit/integration names. Three independent
omission controls reproduced acceptance of missing resource-owner records with
the prior required set; adding those three names made the unchanged controls
reject. All **212 Core/domain/engine Python controls** pass; raw log
`s35-binding-python-controls-v1.log` has SHA-256
`cfbc336fa9e6431cc206fb60d91027f1a59c51df9aeb33e45ac449e5473a868f`.
The Core census/full input map and thirteen existing equivalence-context pins
follow the legitimate extra test source; mathematical source/proof bytes and
historical outcomes remain unchanged. The old CLI inventory bindings are empty
until the changed source is independently re-enumerated.

The formatter completed on all three changed/new files; receipt
`s35-engine-argument-guards-format-v1.json` has SHA-256
`180dc712ebf8d4cefeb9f5106dc720bd0be640caaf1d469a2cf773fe83301007`.
An initial copied launcher stopped before creating output or executing Rust
because its output-prefix check still named S32. The original launcher is
retained; the corrected invocation uses a fresh S35-v2 output.

All six S35-v2 native rows pass: isolation, format, compiler, Clippy and both
default Cargo profiles. Each profile executed the same **sixteen unit/integration
tests plus two doctests**, with zero failed/ignored/filtered cases. All nine
source/index, vendor, Cargo-home, advisory, tool/stable and policy bookends pass.
`s35-engine-cargo-baseline-v2/verification.json` has SHA-256
`fa4c111a5875cfeb2d8c043c1b4be910b8881cbc42f1dab632b372f14735496b`.
The serial native body took **77.562 seconds**, sampled target peak was
**297,734,144 bytes**, and minimum ordinary-user availability was
**16,934,043,648 bytes**. These remain one-worker observations.

Fresh enumeration is running at `s36-engine-argument-guard-enumeration-v1`.
Current critical-file coverage and mutation are next; S33/S34 retain their own
source identity. Unix OS-string controls do not qualify Windows nontext behavior,
and resource-owner units do not establish native completion or product soak.

## S36–S38 current coverage and CLI mutation

**1 November 2026.** Fresh complete enumeration on the S35 119-input candidate
retains **176 library and 31 CLI descriptors**, 207 total. The library list is
byte-identical to S30; the CLI's 34→31 change removes ten old monolithic operator
variants and adds seven explicit resource-owner variants. Every raw descriptor,
source span and old diff hunk was independently reconciled. No filter, skip,
threshold or equivalence change caused the count difference. Old S34 misses are
not relabelled as new kills. Receipt
`s36-engine-argument-guard-enumeration-v1/verification.json` has SHA-256
`e5395b61a9824e77692f04d479196c5f9152ac11992ab5626d98653c9f70dd11`.
Normalized bindings are library
`28d483d667bb05732d6649739571e145ccb2f5519d3367d1c74ae2441a0c5455`
and CLI `c6d2bb87dcbb9a3b0144cfee12c563611e6c3ff04900ae2f1e9371ce9120ef0f`.

Current coverage passes: **684/741 library lines, 92.308%**, and **101/101 CLI
lines, 100%**. All sixteen GNU unit/integration cases passed. New test bodies
remain outside the production denominator. There are zero native branch records,
so no branch-coverage claim follows. Full source/vendor/tool bookends pass.
`s37-engine-argument-guard-coverage-v1/verification.json` has SHA-256
`4007494cddfba790587914f91592ea1746d9834457b2849044a7ea65e963682a`.

The complete current CLI mutation gate passes: **31 generated, 20 viable/caught,
zero missed/timeouts/exclusions and eleven unviable**, with an unchanged successful
baseline of sixteen tests and two doctests. The native tool and receiver exit
zero, and all final full-admission checks pass. The raw/adjusted viable kill score
is **100%**; unviable variants receive no kill credit. Receipt
`s38-engine-cli-mutation-v2/verification.json` has SHA-256
`30bbf7f463dfc27372c47e991c7b3c8f0064f2cc591b4598a925693a069741eb`.

The observer again recorded **ten Cargo processes**, peak 174 namespace threads
and 37 processes. Summed observed RSS peaked at 4,545,668 KiB; minimum host
MemAvailable was 7,559,808 KiB and ordinary-user disk availability 15,153,262,592
bytes. Largest sampled work allocation was 1,624,551,424 bytes (partial), versus
1,159,651,328 bytes for the largest complete snapshot. Thirty missing-RSS samples,
31 executable-read gaps and eight disk-read races/errors remain explicit. The
observer finished after 111.390 seconds including admission/closeout; sampling
provides no hard resource maximum. Its receipt
`s38-engine-cli-mutation-resources-v2.json` has SHA-256
`4ac276062b21a49bdada19c5c47d3649611087ec33db0a387e78100d253bd1a2`.

The complete library campaign is now running at `s39-engine-library-mutation-v1`
with the same fixed ten workers and current candidate. Its outcome remains
pending. Constructor, closure/macro, native failure/completion and platform/soak
alternatives retain their separate obligations regardless of generated scores.

## S39 library faults and next contract slice

**1 November 2026.** The library campaign on S35 source actually completed all
**176 generated variants**: **56 caught, 22 missed, 96 unviable and two timeouts**.
The native baseline passed sixteen tests and two doctests. An independent raw
review matched every actual descriptor and diff to S36. The native tool exited
**3** for timeouts; the receiver exited **1** and did not publish a custom mutation
score or accepted baseline accounting. Full final source/vendor/tool custody
passed. `s39-engine-library-mutation-v1/verification.json` has SHA-256
`5e096a307737d843e16fc5495ce5468d33bc3b6c4e3cfdf5264a95db1821ae88`.
The raw `work/mutation/library/mutants.out/outcomes.json` and individual logs retain
all 177 outcomes including the successful baseline. Neither timeouts nor
unviable variants receive kill or equivalent credit.

The two timed-out mutations changed `can_send` to always true and deleted the
send guard's negation. Both built successfully, then stalled in a private fixture
at an immediately-ready await loop. Its Tokio timeout could not preempt that
synchronous progress cycle. The fixture now independently limits readiness to
65 decisions, reports failure on nonretirement, and checks every confirmed send
has an actual attempt count in 1..64. This changes no production budget.

The 22 misses expose entropy/local-endpoint validation, elapsed reporting,
receive-cap and retired-read accounting, send-result branches, platform-specific
oversize classification, exact commit/stratum boundaries and cleanup aggregation.
Same output on GNU is insufficient equivalence for Windows-only behavior. The
next coherent source slice makes the relevant checked transformations and native
result receivers mandatory private owners, with direct positive/negative cases.
Synthetic completed I/O results, platform-code cases and exact-instant boundaries
will remain distinguished from actual OS failures and real UDP observations.
No equivalence disposition or threshold change has been adopted.

The real CLI timeout observer also now requires its reported active elapsed to
cover the independently requested 120ms unanswered lifetime; zero/one-nanosecond
transcript controls accompany the actual healthy stream. The current source is
under development and is not qualified by the older S35/S37/S38 receipts. Finish
this slice, format and freeze it before the next complete affected checks.


## S40 mandatory completion owners and current native baseline

**1 November 2026.** The 119-input candidate is
`509affa74871944548317221b58b45d878ee2b75d15103cc9bc2bde694be30a1`.
Private checked owners now receive generated request entropy, bound local
endpoints, completed send results, platform-specific read results and completed
cleanup results. Their actual producers remain mandatory callers; the public API,
native timestamp order, one-way slot history and timer reaping remain unchanged.
Eight additional library units challenge those alternatives and exact receive-cap,
selected-read accounting and commit-time boundaries with accepted neighbors.
Synthetic native errors and Windows oversize codes are labelled synthetic; they
establish no induced OS failure or Windows I/O qualification. The real CLI timeout
observer checks the independently requested 120ms lifetime, with zero/one elapsed
transcript controls. The private ready-loop bound remains independent of the
production send guard.

The required native baseline now includes all **24 unit/integration names**.
An omission control first demonstrated eight incorrectly accepted omissions;
after adding the required names it passes. All **213** engine/core/domain Python
controls pass on the current candidate, with raw log
`s40-binding-python-controls-v3.log`, SHA-256
`bf2a68c71bf6cfb883eac2e2017432ff930b47e0c33b64d34e286497a8a0cfd4`.
Core/domain candidate-context pins were refreshed without changing their
mathematical source, equivalence proofs or historical execution identities.

Native baseline v1 stopped at six test-only Clippy findings. The lint repair
missed one renamed local in a slice expression; v2 stopped at that compiler
error. Both attempts retain their failures and executed zero Rust tests. After
the exact repair and formatting, **all six v3 rows pass**: isolation, format,
compiler, Clippy and default Cargo ordinary/release tests. Each profile executes
**24 unit/integration tests plus two doctests**, with no failed/ignored/filtered
cases. All nine custody bookends pass. The serial native body took **77.992
seconds**; this is host execution, with no native Windows claim. Receipt
`s40-engine-cargo-baseline-v3/verification.json` has SHA-256
`6222247a63d46336c64a60ac2c5508fb581831f10c943cad0c763466e4eb5d53`.

Fresh complete S41 enumeration retains **185 library and 31 CLI descriptors**,
216 total, below the unchanged campaign cap. Receipt
`s41-engine-completion-guard-enumeration-v1/verification.json` has SHA-256
`a1af3d24540aa254c7dd29a4f2170afa103e3ff5983dc2ea38b33821b5833ffd`.
Independent review reconciled every descriptor, source span and old diff hunk.
The 176→185 library change consists of prepare/local admission +3, send completion
+2, receive/classification +3 and cleanup +1; other function counts are unchanged.
The CLI inventory, including diffs, is identical to S36. Reviewed bindings are
library `2f03a757eaa5e6c78635a69900c91f2b7ddfe9ca7da4eecf7196117e916d1583`
and CLI `c6d2bb87dcbb9a3b0144cfee12c563611e6c3ff04900ae2f1e9371ce9120ef0f`.
The policy adopts those exact inventories; all 27 focused engine Python controls
pass afterward. No equivalences, thresholds, filters, tool custody or command
budgets have changed. S38/S39 retain their earlier source identities.

S42 coverage v1 stopped in preflight with zero native commands and no output:
copying the enumerator launcher accidentally changed its retained-receipt pin to
the fresh coverage output path. V1 is preserved; v2 corrects that one input path
and uses a fresh output. Both critical-file mutation gates will share the existing
`--subject all` Session, preserving independent baselines and 95% per-file gates.
This uses two full tool-admission bookends instead of four separate-session
bookends, with ten workers per sequential file campaign. It changes no cache or
integrity implementation and claims no measured runtime saving yet.


## S42 current critical-file coverage

**1 November 2026.** On the unchanged S40 candidate, coverage v2 passes
**754/786 library lines (95.929%)** and **101/101 CLI lines (100%)**.
All **24** GNU unit/integration cases executed; this lane runs no doctests.
Test source remains outside the production denominator. Both files have zero
branch records, so this establishes executable-line coverage only. The actual
coverage command exits zero and full final admission reports unchanged inputs.
`s42-engine-completion-guard-coverage-v2/verification.json` has SHA-256
`efa53b3ab377575b75757fceb96cb0a768a0b4b2c61dc522a02f4c358ff8dde2`.

The complete all-subject mutation campaign is prepared at
`s43-engine-completion-guard-mutation-all-v1`, with the existing passive observer,
full custody, ten workers and separate complete inventories/baselines per file.
No mutation result is claimed before the native receiver finishes.


## S43 complete current engine mutation checkpoint

**1 November 2026.** Both complete critical-file campaigns pass on the unchanged
S40 candidate and reviewed S41 inventories. The library has **185 generated,
81 viable/caught and 104 unviable**; the CLI has **31 generated, 20 viable/caught
and eleven unviable**. Each file has **zero survivors, timeouts and exclusions**,
and a **100% viable kill score**. Unviable variants receive no kill credit.
Each actual native baseline passes the same **24 unit/integration tests plus two
doctests**. Both enumeration commands and both native campaigns exit zero; final
full tool/source admission reports unchanged inputs. Receipt
`s43-engine-completion-guard-mutation-all-v1/verification.json` has SHA-256
`0999525f4696b24002aa0956017f19e69f3b79511e9639ea5c008167543b1591`.
The old S39 misses/timeouts retain their original descriptors/source; none is
relabeled as a kill or equivalence on that older candidate.
Independent review matched all 216 actual descriptors and stored diff bytes to
S41, every caught variant's successful build and meaningful failed test, and all
115 unviable variants' ordinary compile/lint diagnostics (48 dead-code, 39 E0277,
25 unused-variable and three unused-import primary causes). No resource,
infrastructure, signal or compiler-internal failure was found. Both native
baseline streams and exact full custody inventories were reconciled; no native
Windows or ungenerated-behavior credit follows.

The single existing all-subject Session executes separate file campaigns and
retains both full custody bookends, avoiding a second initial/final admission
pair. Native tool wall times were **80.979 seconds library** and **30.557 seconds
CLI**. The observer's complete window was **203.637 seconds**, including setup,
admission and closeout; the difference is not an exclusive admission profile or
a controlled runtime comparison.

The observer recorded **ten Cargo processes**, peaks of **227 threads and 51
processes**, and summed observed RSS of **4,954,580 KiB**. Minimum host
MemAvailable was **7,584,416 KiB** and minimum ordinary-user disk availability
**14,303,563,776 bytes**. Largest sampled owned-work allocation was
**1,761,726,464 bytes**, also the largest complete snapshot. Missing RSS samples
(125), executable-read gaps (126), a status-read gap and 33 disk-read races remain
explicit, alongside 19,855 unattributed process-read errors. The sampler finished
with `measurement_incomplete=false` and `observation_has_gaps=true`; these sampled
observations establish no hard resource maximum or product soak. Resource receipt
`s43-engine-completion-guard-mutation-resources-v1.json` has SHA-256
`7e56696da4793e7b1dfb4d1dcd9c2718a2221719e97bfa14ccb25cd97fa192b8`.

This completes the current **GNU host baseline, executable-line coverage and
generated-mutant checkpoint**. The inherited constructor/closure/macro,
ungenerated and actual native failure/completion obligations remain distinct.
Native Windows regression, memory/lifetime, retention/soak and production
acquisition/TLS/service/clock/package qualification remain open. Continue the
existing S0.1/R-01 TLS feasibility chain by restoring the same frozen proposal's
missing transitive source cohort, then completing its declared admission gates;
this engine result adopts no provider or trust policy.

## S44 external checked-constructor regression — qualified

**5 November 2026; ALIGN-2.** The owner-supplied Rust-skill audit correctly found
that `Config` and `ControlSchedule` have private checked representations but the
engine verification chain observes only positive construction examples. Current
privacy is intact; missing external negative consumers are a regression gap.
The bounded repair adds direct construction and each-field mutation, absent
Default/unchecked From and raw-value checked-API probes, plus an inert legal
consumer with admitted and rejected constructor neighbors. Expected compiler
causes must occur at the actual attempted bypass; setup errors do not qualify.

The existing policy receives a closed `external_contracts` child for current
constructor evidence only. It uses the admitted stable payload, lib-only scope,
166 current product inputs, six build jobs and a 300-second bound. Runtime,
preparation, sockets, timers, network and Windows operations are not executed by
its legal consumer. Parent S40–S43 source/candidate/inventory bindings remain
historical and unchanged; this child does not qualify current engine coverage,
mutation, resource/lifecycle or production behavior.

Actual GNU-host compiler qualification passed on the current 166-input product,
canonical product-map SHA256
`2234690e73ef709e2548655117fb00896ef8ca506e8d588f71a033a9b615500f`.
The configured `--lane contracts --subject library --tool-admission-profile
stable-core` invocation ran 15 commands: one real library build, 13 external
consumer compilations and one inert legal execution. Both whole-struct privacy
cases require both private field names; four field mutations, two absent Default,
two absent From and two raw-API misuses produced their intended E0451/E0616/
E0277/E0308 causes at the attempted source line. The legal consumer observed
valid timeout/schedule limits and invalid neighboring constructors without
starting a runtime or creating a socket. No setup or unrelated compiler error
was credited.

Receipt `engine-privacy-contracts-v1/verification.json` under the existing S12e
evidence root has SHA256
`2d74ee084b9dd21eb084ec212f0b29b89148e31ce98cee8f85c3535f0fa9bb4c`.
All 174 bound source/policy/tool inputs match current bytes; final stable payload
and source/vendor custody passed. The receipt deliberately retains
`qualification_passed=false` and `contracts_qualification_passed=true`.
All 178 integrated domain/engine/common Python controls also pass (0.887s),
alongside portable V6's 22 product lanes. This closes ALIGN-2's named regression
gap only; S43's historical coverage/mutation scores remain on their old candidate.

## S2.1 preparation — real transport to an admitted time measurement — 5 November 2026

**Disposition:** prepared contract/integration handoff; implementation and qualification remain open. Follow [development §7][s21-development]: the [S1.1c rounding/era continuation][s21-domain-task] remains the first executable task. The selected settled-rate Core checkpoint remains complete within its Linux fixture scope. This card does not authorize another campaign or promote the loopback pilot.

**Source checkpoint and preparation:** upstream `8b8d0142ab1daae50509b6ec2cea800ada033449`; synthetic local baseline `3316356d58480dcf86b08132272bed613f86ca4f` is not an upstream commit. Three completed bounded research lanes—GitHub primary source, standards/web and Microsoft Learn MCP—fed a fourth analyst, then this fifth documentation integration. Initial context-limit launches supplied no usable result; completed bounded retries supplied the research and synthesis. Intermediate reports do not replace the durable source links below. This records source/documentation work, with no product build, test, proof or native execution. At this preparation handoff independent integration review had not run; any later review outcome is recorded separately below.

**Owner and requirements:** engine/protocol owns actual acquisition and final consume-once admission; `time-domain` owns checked mathematical transformations; existing S1.2/S1.3 owners receive observations; S1.4 owns leap/scale policy; W11-W01 owns timestamp provenance and later platform qualification. Reuse REQ-8/BND-1 and applicable CHECK-6/7/8/11/14/25/27, with CHECK-21 resource evidence and later CHECK-10 clock effects at their existing scopes.

### Existing path and bounded objective

`prepare` → `attempt_send`/`complete_send` → `selected_read` → `stage_reply` → `process_reply`/`commit_reply` currently ends in diagnostic `TransportFacts`. The engine manifest has no exchange/domain/Core dependency. Its separate fixture counterpart, `Association::receive`, preserves header metadata and invokes `unfold_ntp`/`exchange_arithmetic`. `FixtureObservation::new` then checks numerical fixture invariants, not native provenance or eligibility. The existing S1.1e `Composition::deliver` consumes a genuine fixture completion, checks binding/generations/mapping, constructs `FixtureObservation` at fixture arrival with divisor `1.0`, and calls `FixtureHistory::try_append`; `Composition::record` reads actual retained rows/statistics. This test-only chain establishes neither a production reference-time rule nor statistical weighting. [Engine][s21-engine], [association][s21-association], [normalization][s21-normalization], [scenario][s21-scenario]

**First bounded implementation candidate:** preserve complete staged evidence and deliver one current controlled-loopback exchange through checked mapping/era/arithmetic into a typed experimental measurement and diagnostic projection. Keep `authentication=none`, `eligible=0`; no statistical vote, correction proposal or clock action. Define the capture convention, explicit experimental date scope and error/unsupported outcomes before its implementation. If mapping evidence is insufficient, retain a diagnosed transport observation instead of inventing coordinates or certainty.

**Dependent full integration:** a genuinely admitted observation must reach the actual history/estimator consumer under a separately frozen reference-time and uncertainty-to-dimensionless-divisor contract. Reaching the first measurement exit does not complete S2.1. DNS lifecycle, reviewed KoD scheduling, endpoint/port policy, remaining resource/native evidence and that consumer contract stay open. No public endpoint, default source, automatic retry or new runtime framework is selected by this preparation.

**Diagnostic exit:** extend the existing `nevrotick-observe` projection to distinguish transport-only observations, successful experimental measurements and later Core insufficient/ready/rejected outcomes. A successful measurement exposes its actual checked phase and signed raw RTT with units and exact representation or explicit rounding residual/error, acquisition/reference age, and capture-placement, mapping-validity and error qualifiers. The first exit retains `authentication=none eligible=0`. Emit bounded allowlisted reasons for unavailable mapping, era/arithmetic failure, stale context/deadline and later statistical refusal. An absent measurement or estimate remains absent, never numeric zero or success. Preserve bounded, redacted diagnostics: no packet dumps or raw error strings. Exact field spelling and the later uncertainty formula remain with their existing owners; this selects no new schema or framework.

### Preserve evidence before interpreting it

The lossless staged envelope must retain the following facts or their equally lossless immutable representation. These are contract obligations, not adopted Rust type names:

| Evidence | Producer/consumer obligation |
| --- | --- |
| Complete wire representation | Retain raw LI, version/mode/stratum, poll/precision exponents, root-delay/dispersion bits, reference ID/timestamp and origin/T2/T3 bits; retain any applicable authenticated coverage. Framing does not establish trust. |
| Exchange identity | Bind logical source, actual destination/receive peer, socket lifetime, request ID, separately saved real T1 and opaque wire token. Numeric handle/address reuse creates no new identity or independent vote. |
| Local timing | Retain send-before/after, receive-return and processing readings; original units/domain; capture-placement category; mapping/conversion identity and error. Preserve any later selected packet timestamp separately. |
| Current context | Carry configuration, association and clock-mapping generations, relevant policy/calendar identity, allowed trust/scale profile, independent date/era evidence and full possible exchange span. |
| Checked result | Preserve exact Q33 phase, signed Q32 raw RTT, conversion residual/error, acquisition identity/reference time and age; do not replace them with formatted CLI text. |

[S1.4 §2][s21-leap-task] already owns LI preservation. This continuation extends that handoff to the whole metadata envelope and actual numerical consumer. The present pilot does not retain LI, precision and root/reference fields through staging/reporting. The bounded follow-up resolves the pinned implementations' behavior: Chrony 4.9 and ntpd-rs decode ordinary root delay **and dispersion as unsigned 16.16**; `0xffff0000` represents +65,535 s. The local fixture's signed-format comment disagrees, but its raw `u32` storage loses no bits and establishes no runtime decoder bug. Carry these observed semantics and high-bit controls into the future converter contract; record any legacy-compatibility decision explicitly, without repeating the survey or changing source in this documentation stage. RFC 5905's format and older RFC 4330 wording retain distinct provenance. [Chrony converter][s21-chrony-conversion] and [header calls][s21-chrony-header], [ntpd-rs converter][s21-ntpdrs-conversion] and [header calls][s21-ntpdrs-header], [RFC 5905 §6][s21-rfc-format], [RFC 4330 §4][s21-rfc4330]

### Honest local timestamps and final admission

`SoftwareClock::capture` reads `Instant` then `SystemTime`; it establishes neither simultaneous sampling nor a stable UTC mapping. The send bracket encloses an application call, not demonstrated wire egress. Receive-return is later than possibly queued ingress; even a before/after receive-call bracket would not enclose earlier queued arrival. Do not infer a wire-time error bound from either syscall duration. W11-W01 may evaluate a bracketing mapping capture, but its uncertainty, valid interval, adjustments and invalidation must be qualified independently. QPC continuity alone proves no unchanged UTC mapping or active-awake delivery. [Engine][s21-engine], [QPC][s21-qpc], [recvfrom][s21-recvfrom], [sendto][s21-sendto]

Keep real local T1 separate from the randomized origin-matching token. Establish compatible UTC scale for all four coordinates and continuity of the local T1/T4 mapping; the remote clock does not share the local calibration. Monotonic clocks own ordering, age and deadlines, not UTC substitution. Optional kernel/NIC timing and a direct-overlapped adapter remain conditional. Supported, configured and actually observed timestamp capabilities are different facts. [Winsock timing][s21-timestamping]

Stage fallible work, then recheck current source/socket/request/endpoint, cancellation/retirement, relevant generations and fresh **processing** time at one exclusive publication point. Preserve `send ≤ arrival ≤ processing < deadline`; arrival before expiry alone is insufficient. No await or callback may intervene between the final current-state guard and commit. Bind delayed TX evidence to its originating request; it cannot attach to a replacement request or rewrite transmitted bytes. [Association][s21-association], [engine][s21-engine]

Malformed/uncorrelated traffic preserves healthy pending state/history; independently reached lifecycle/budget transitions may retire work. Freeze the disposition of a correlated completion that later fails statistical admission: it cannot enable repeated replies until one passes a threshold. Keep stratum-zero controls outside time arithmetic/history and enforce reviewed RATE/DENY/RSTR effects at the actual next-send guard. Reuse the [existing KoD source-to-scheduler ledger](s0.1-protocol-comparison.md#kod-source-to-scheduler-review), including unknown/forged/stale controls and a healthy neighboring peer; select no poll constant here. [P2 contract](../docs/plans/development-plan.md#p2--secure-bounded-ntpnts-communication), [RFC 8633 §5.4][s21-kod]

Bound complete datagrams and retained metadata. The existing Windows 10040 branch can charge socket-level consumption, but its error carries no successfully returned peer identity: do not assign peer blame/freshness or parse a valid-looking truncated prefix. Future ancillary data needs independent length/flag/framing admission. Logical cancellation/close is distinct from actual native completion; retain buffers, identity and permits until terminal settlement. These Microsoft contracts do not establish Tokio's actual backend behavior. [recvfrom][s21-recvfrom], [WSARecvMsg][s21-recvmsg], [CancelIoEx][s21-cancel], [closesocket][s21-close]

### Era, leap and checked Core bridge

Use explicit supported date/era evidence over the full possible exchange span; raw min/max T1–T4 and a damaged host date cannot supply it. `EraWindow` validates numerical bounds, not anchor truth. `unfold_ntp` must retain its zero/one/multiple-candidate classification. Authentication supplies no era number, guaranteed symmetry or source-scale truth. [Era source][s21-era], [RFC 5905 §8][s21-rfc-math]

Reuse S1.4's two distinct questions: LI consistency for the applicable UTC claim day, and exchange-span compatibility outside the current guard. A short event-free exchange can validly announce that day's later midnight event. Event membership is not crossing: `[T,T]` can contain a transition while using one after-event offset. Keep useful unsmeared LI=0 inputs and unknown future coverage under the selected same-segment rules. Contradictory/unknown-scale observations cannot erase healthy peers, extend guards or create fresh quorum. Kernel-only explicit leaps and measured-residual recovery remain unchanged. [S1.4][s21-leap-task], [interpretation][s21-leap-interpretation], [verified EID 4504][s21-li]

Preserve signed raw RTT separately from adjusted delay, root uncertainty, capture error, fit residuals and arithmetic error. No universal negative-RTT clamp/rejection, perfect certainty from zero advertised dispersion, or arbitrary divisor follows. The statistical owner must freeze the physical assumptions, exact units, reference instant, admissible envelope, divisor derivation and checked outputs before genuine estimator integration. Chrony's precision/frequency treatment and midpoint reference are adaptation evidence, not adopted thresholds or a ready-made local contract. [Exchange source][s21-exchange], [Chrony pin][s21-chrony]

Use actual immutable checked outputs through history, normalization and estimation; do not round Q33 phase to nanoseconds and then silently recreate an exact zero-error observation. Preserve original observation/source identity, age and chart/mapping through projection and selection. Re-evaluation never creates freshness. One or two observations remain insufficient for the current regression API. Its fixture bounds, including phase ±1 s and the admitted count/span, are not production limits or the large-recovery policy. Existing `Composition` may demonstrate a clearly labelled fixture consumer with declared fixture divisors; it cannot close this uncertainty/admission contract. [Normalization][s21-normalization], [scenario][s21-scenario]

### Acceptance and status

Use the independent controls and proof inventory below in this existing task. Bind future results to actual source/configuration/tool/fixture identities and distinguish portable arithmetic, real controlled UDP, native Windows and product eligibility. Keep S44 and earlier receipts on their original candidates. [W11-W01 §4.26](windows11-integration.md#426-s21-timestamp-provenance-preparation--5-november-2026) records partial source preparation; its complete provenance card and native qualification remain open.

Full dependent mathematical acceptance requires OD-10 proofs; prepared files, a model alone or historical fixtures do not suffice. Preserve NTS-only baseline/OD-03, held provider/default choices, all applicable native/S134 restrictions and separately admitted Sandbox scope, fuzz/storage holds and UI-M0 deferrals. Complete MSIX remains selected; enhanced release provenance remains optional under OD-05 §7.5. Documentation preparation lifts none of these holds and requires no new policy adoption.

### Independent controls and proof obligations

All controls are **specified, not executed**. Controlled clocks/fixtures and real packet observations are separate evidence layers.

| Control | Independent expected result and useful neighbor |
| --- | --- |
| Analytic sign/units | Ideal relative `T1=0,T2=70,T3=80,T4=110 ms`: `θ=((70−0)+(80−110))/2=+20 ms`; `δ=(110−0)−(80−70)=100 ms`. Positive means Windows is behind. Decimal milliseconds are not generally exact Q32 wire values. |
| Canonical exact dyadic seam | Use nonzero admitted common base `B`, explicit era and `T=(B,B+7/128,B+8/128,B+11/128) s`. Exact `θ=+1/64 s`, signed raw `δ=5/64 s`; Q33 numerator `134217728`, Q32 numerator `335544320`. Changing only the matched wire token changes neither result. Reducing both peer timestamps by `1/32 s` gives `−1/64 s` with unchanged RTT. |
| Half units and rounding | Preserve offset numerator ±1: these are ±half a Q32 tick, not zero. Ties-even conversion gives 0 ns with residual numerator ±1,000,000,000 over `2^33` in nanoseconds. For Q33 numerator ±`2^23`, ±976562.5 ns rounds to ±976562 ns; residual ±`2^32`. Numerator `3×2^23` instead gives 2929687.5 ns, rounding upward to even 2929688 ns. Use legal operation-produced values, not an unchecked constructor. These are arithmetic-seam controls. |
| Real UDP | Record independently inspectable actual request/response bytes, socket/request identities and software capture/mapping evidence. Independently calculate rational expected results/error from those admitted observations; compare real returned checked values. Sleeps cannot force the dyadic tuple or demonstrate physical arrival accuracy. Name any injected timestamp seam explicitly. |
| Diagnostic projection | Drive `nevrotick-observe` with healthy controlled real UDP and a refused neighbor. Independently reconstruct the checked result from captured observations and peer bytes; require emitted phase/RTT, units, exactness/residual and age/provenance qualifiers to match. Refusal emits the expected allowlisted reason and absent result, with no added observation or refreshed history/estimate; the healthy first exit retains `authentication=none eligible=0`. Later Core cases distinguish insufficient/ready/rejected without inventing a fit. Observe output and consumer state; diagnostics never perform the claimed transition. |
| Physical asymmetry | With true +20 ms source offset, forward/reverse delay 30/70 ms and 10 ms peer processing, the ideal estimate is 0 ms while raw network RTT is 100 ms. Correct arithmetic and authentication do not prove true UTC accuracy. |
| Lossless metadata/current admission | Distinct LI/precision/root/reference values survive staging, including high-bit raw root fields. Wrong endpoint/socket/token, changed generation, old TX ID, duplicate completion, truncation and cancellation give no new history observation. A fresh current complete reply succeeds once. Processing at the deadline rejects; a current just-before-deadline neighbor succeeds. |
| Root representation | Pinned unsigned-16.16 decoding gives `0x00010000 → 1 s`, `0x80000000 → 32768 s`, `0xffff0000 → 65535 s`; these are conversion oracles, not admissible-quality thresholds. Keep raw signed exchange RTT separate. Do not copy ntpd-rs's unrelated `to_seconds` helper as an exact Q32 oracle: it divides by `u32::MAX`, while the inspected ordinary-header consumer uses `.into()`. |
| Era and leap endpoints | With raw `r=1` Q32 tick and era width `E=2^64` ticks, `[r,r]` admits one candidate, `[r,r+E]` is ambiguous, `[r+1,r+E−1]` admits none. At compiled transition `T=3692217600` NTP seconds, `[T,T]` has one event with known after-offset 37; `[T−1 Q32 tick,T]` has varying offsets. Claim-day LI1/2 can be valid during a short `NoEvents` span. Neither calendar result alone proves guard eligibility. |
| Actual Core consumer | Reuse S1.1e composition with independent declared fixture weights: after 1–2 genuine delivered observations, no fitted estimate; after 3+ legal increasing observations in one context with sufficient span, inspect actual retained rows/reference and actual fit. Current normalization permits 3–64 rows with minimum 62,500,000 ns span; these are existing fixture constraints only. Later real admission must supply the separately justified divisor/reference. Duplicate/report replay adds no row and cannot renew age. |
| Native continuation | Queued ingress must not become receive-return accuracy; unsupported/missing metadata cannot become zero. Cancellation losing to completion settles once and stale completion changes no model while releasing its actual capacity. A fresh healthy exchange/shutdown succeeds within the same budget. These remain future Windows observations, not source-proof results. |

Mathematical contracts include required success **and** each rejection class, representability, exact units, rounding/error, preserved state and promised bounded work. Reachable healthy/error witnesses, distinct compiled defect controls and restored healthy passes complement independently reviewed full-domain proofs. Do not silently narrow domains or credit always-error behavior. [OD-10][s21-decisions], [formal workflow][s21-proof]

| Existing proof owner | Exact obligation/remaining evidence |
| --- | --- |
| S1.1c, CHECK-7/8/25 | Complete/reconcile the already-owned rounding/era schedule first. Public coordinate/window constructors, raw rejection, exchange-produced bounds, four-coordinate arithmetic and clock conversions remain separate obligations. Anchor truth is environmental evidence. |
| Engine/platform with domain | Any selected counter/epoch conversion, bracket/mapping interval, outward error and full-span transformation needs source correspondence and complete numerical contract. Native placement/continuity/callback behavior remains separate evidence. |
| S1.4, CHECK-7/8/25 | Checked ledger/date/coverage constructors, calendar arithmetic, whole-interval offset/event interpretation and bounded traversal; compiled-data/import correspondence and current source-scale/guard authority remain separate. |
| Protocol/statistics bridge | Specify root/precision conversion using the completed pinned-source comparison, with explicit compatibility scope and independent high-bit/unit controls. Negative-delay treatment, reference time and uncertainty/divisor derivation still need their own frozen contract before proof or dependent consumer acceptance. |
| S1.2/S1.3, CHECK-7/8/25 | Normalization/regression/residuals, uncertainty bounds, history/chart projections and any reached selection/combination mathematics, with actual machine/floating semantics and caller invariants. Selected fixture passes do not establish these proofs or production eligibility. |

### Source references for this continuation

Local links identify inspected owners at the recorded checkpoint; external links
retain the primary evidence and upstream source pins. Source behavior does not
establish a product or native execution result.

[s21-engine]: ../crates/nevrotick-engine/src/lib.rs
[s21-association]: ../crates/ntp-exchange/src/association.rs
[s21-scenario]: ../crates/ntp-exchange/tests/support/scenario_driver.rs
[s21-normalization]: ../crates/time-core/src/regression/normalization.rs
[s21-exchange]: ../crates/time-domain/src/exchange.rs
[s21-era]: ../crates/time-domain/src/era.rs
[s21-leap-interpretation]: ../crates/time-domain/src/leap/interpretation.rs
[s21-domain-task]: s1.1-time-domain.md#mandatory-proof-continuation-and-remote-integration--5-november-2026
[s21-leap-task]: s1.4-leap-policy.md#2-protocol-and-discipline-integration
[s21-development]: ../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent
[s21-decisions]: ../docs/nevrotick-owner-decisions.md#14-od-10--mandatory-mathematical-proofs
[s21-proof]: ../.agents/skills/testing/references/formal-verification.md
[s21-chrony]: https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/ntp_core.c
[s21-chrony-header]: https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/ntp_core.c#L2017-L2023
[s21-ntpdrs-header]: https://github.com/pendulum-project/ntpd-rs/blob/18a5bd6a87a65033a41c36dc8b4b737c98b5523f/ntp-proto/src/packet/mod.rs#L199-L218
[s21-chrony-conversion]: https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/util.c#L650-L679
[s21-ntpdrs-conversion]: https://github.com/pendulum-project/ntpd-rs/blob/18a5bd6a87a65033a41c36dc8b4b737c98b5523f/ntp-proto/src/time_types.rs#L213-L278
[s21-rfc-format]: https://www.rfc-editor.org/rfc/rfc5905.html#section-6
[s21-rfc-math]: https://www.rfc-editor.org/rfc/rfc5905.html#section-8
[s21-rfc4330]: https://www.rfc-editor.org/rfc/rfc4330.html#section-4
[s21-kod]: https://www.rfc-editor.org/rfc/rfc8633.html#section-5.4
[s21-li]: https://www.rfc-editor.org/errata/eid4504
[s21-qpc]: https://learn.microsoft.com/en-us/windows/win32/sysinfo/acquiring-high-resolution-time-stamps
[s21-recvfrom]: https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-recvfrom
[s21-sendto]: https://learn.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-sendto
[s21-timestamping]: https://learn.microsoft.com/en-us/windows/win32/winsock/winsock-timestamping
[s21-recvmsg]: https://learn.microsoft.com/en-us/windows/win32/api/mswsock/nc-mswsock-lpfn_wsarecvmsg
[s21-cancel]: https://learn.microsoft.com/en-us/windows/win32/api/ioapiset/nf-ioapiset-cancelioex
[s21-close]: https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-closesocket


### Independent documentation review and verification — 5 November 2026

**Final documentation disposition: READY; S21-R01 CLOSED.** This records the
completed review of the eight-file S2.1 preparation and its one bounded correction.
Implementation, mathematical proof, native Windows and release qualification
retain the open scopes above. The first executable task remains S1.1c.

Reviewer `/root/s21_review` was created with explicit `fork_turns: none`, without
the author research/analysis reports, private discussion or memory. Its initial
launch failed at the context limit and supplied no usable review (`not run`, not
zero findings). One bounded retry completed the initial review with **REVISE**
and the stable medium finding S21-R01: define the real diagnostic consumer and
observable successful/refused output. The integrator added only the diagnostic
exit paragraph and its paired control. The same independent reviewer then
verified that correction, closed S21-R01 and returned **READY** for documentation;
unaffected findings were reused without another full audit.

The initial frozen diff was
`18fc6c66ca1fc5039e423f7c570e64fc46ddad75a270d1352231c4db59ac3a63`;
the corrected reviewed diff is
`c77152a73b0cfe7f86051fc0f256f46698101af51ddb10a9ea1e297d5015adc4`.
The corrected engine task before this append is 97,293 bytes, SHA-256
`236e174fdf1fd225e699b4b3a1dce46a8b882db09f4fd717e397dfaed51d49be`.
The other seven documents retain the initial review hashes below. This receipt
appends completed evidence to that reviewed body; it changes no integration
contract. Both raw reviewer reports, including the initial own-word comprehension
paragraphs, are retained verbatim in the following quotations.

#### Initial independent report — completed, REVISE

> The proposed amendment adds an S2.1 integration handoff to the existing engine task and routes its status through seven related documents. It describes how preserved transport evidence should become a checked experimental measurement, followed separately by justified admission to the existing Core consumer. It also connects the work to timestamp provenance, leap interpretation and mandatory mathematical proofs while leaving the current S1.1c continuation first in the execution order.
>
> The current runtime performs bounded exchanges with two explicitly configured loopback peers and returns transport facts; it has no dependency on the exchange, time-domain or Core crates. A separate test composition already consumes genuine fixture exchange completions into history and regression using asserted fixture timing and a fixed divisor. Checked arithmetic and pure leap interpretation exist, but production capture/mapping, uncertainty admission, the runtime-to-Core connection and their Windows qualification remain future work. The amendment preserves that distinction and the existing execution restrictions.
>
> # Independent S2.1 documentation review
>
> ## Candidate and requested stage
>
> **Verdict: REVISE — one bounded documentation correction, S21-R01.** This verdict applies only to the eight-file documentation preparation and handoff. It is not a product-implementation, mathematical-proof, native-Windows or release verdict. The review completed with usable findings; it is not a zero-result launch or a blocked retrieval reported as success.
>
> | Identity | Reviewed value |
> | --- | --- |
> | Repository | `DavidOsipov/NevroTick` |
> | Checkout | `/workspace/scratch/68e2481482dd/NevroTick-s21` |
> | Supplied actual upstream | `8b8d0142ab1daae50509b6ec2cea800ada033449` |
> | Supplied upstream tree | `70103638b117960a91a259a740a8ed13c8afe8f2` |
> | Observed local HEAD / synthetic comparison baseline | `3316356d58480dcf86b08132272bed613f86ca4f`; this is not upstream history |
> | Frozen diff SHA-256 | `18fc6c66ca1fc5039e423f7c570e64fc46ddad75a270d1352231c4db59ac3a63` |
> | Stage | Documentation preparation for the real-transport → measurement → Core handoff; no implementation or execution |
>
> I independently hashed the eight current files, compared their hashes and lengths with the manifest, and compared the actual `git diff` byte-for-byte with the frozen diff. All matched. `git status --short` showed exactly the eight expected modified Markdown paths. Relevant predecessor passages were retrieved with `git show HEAD:path` for every changed path.
>
> | Candidate path | SHA-256 |
> | --- | --- |
> | `AGENTS.md` | `5bcd3ac709aa2cca2d8932df92d04573afc99493694e9b90cc3daaf7ad43558a` |
> | `docs/README.md` | `85497713724fedc0c021e205e704ec78ab78677a77635b16fdbc9ce9e9570220` |
> | `docs/plans/development-plan.md` | `adcccd2fb8ebba7b4da150025263b54c3948c9a27c92a103090d6e0954d17bfd` |
> | `docs/windows-time-architecture.md` | `bd09572c2ca8c66f6fb2e5b8397597fbc445234c18bdfa0499619e04f93de031` |
> | `docs/windows-time-development-hardening.md` | `f3ab09724decafbc4d0556d17a498e1084dfd4963cfeebf9fee1a681f63d6269` |
> | `tasks/engine-transport-pilot.md` | `532231a6757ea60c5b8d2cd162d7e05112ea8c0e4ed4ae37426c6d3f409248af` |
> | `tasks/s1.4-leap-policy.md` | `9d010b6b31c263098bf4035335f186c4eb9512b0125c817491c6d2998859b723` |
> | `tasks/windows11-integration.md` | `6cb2b333660f3919ba0accd3d81a28adfea27e961d3a0bf15e2d236f60089fb9` |
>
> ## Independence and actual read scope
>
> This review used the supplied review request, frozen diff/manifest, candidate text, current repository instructions, relevant source and independently retrieved primary external evidence. I did not read `s21-github.md`, `s21-web.md`, `s21-learn.md`, `s21-analysis.md`, author discussion, memory or private personal context. I created no agents and performed no repository edits, builds, tests, proofs, product execution, installations, native experiments or remote writes. Only this review report was written.
>
> The read scope was targeted rather than a full reading of the large documentation bundle:
>
> - **Instructions and requirements:** complete root `AGENTS.md`, the testing contract and `qa-independent-review/SKILL.md`; documentation-guide routing and current amendments; OD-05 §7.5, OD-09 and OD-10; development-plan P2, §7 and changed routing passages; architecture §6's domain/construction/guard contracts, §7.3, relevant timestamp passages in §10.1, §10.5, §20.1.1 and the applicable CHECK rows. The S2.1 row and network-integration requirements in the supplied audit were also inspected.
> - **Changed documentation:** all changed hunks in the eight-file frozen diff, with the engine card and W11-W01/§4.26 read directly in current files, relevant surrounding text and predecessor excerpts. This did not re-review all historical receipts or all unchanged content in those files.
> - **Source:** engine manifest; `SoftwareClock`, `TransportFacts`, send/receive/staging/final-commit and Windows oversize handling in `crates/nevrotick-engine/src/lib.rs`; the actual `nevrotick-observe::write_report` consumer; `ntp-exchange` association admission, metadata, receipt/timing context and `Composition::deliver`/`record`; complete `time-domain` exchange, era, rounding and leap-interpretation source; the generated 2017 transition; Core observation construction, normalization bounds and history's actual append/statistics path.
> - **Task status:** engine pilot authority and S44 ending; S1.1c mandatory-proof continuation; S1.1e scope/routing; the final selected four-owner settled-rate receipt in S1.3; S1.4's current continuation and protocol handoff; W11-W01 and the new §4.26. Historical task statements were treated according to their recorded candidate and later continuation, not as current whole-product evidence.
> - **Primary external evidence:** the pinned Chrony `util.c` root conversion and `ntp_core.c` header/measurement consumers; pinned ntpd-rs `time_types.rs`, `packet/mod.rs` and `source.rs` ordinary consumers; RFC 5905 format/arithmetic, RFC 4330 root-delay wording, RFC 8633 §5.4, and verified erratum 4504; Microsoft QPC, recvfrom, sendto, Winsock timestamping, WSARecvMsg, CancelIoEx and closesocket contracts.
>
> The supplied snapshot is a partial materialization, reported by the packet as 723 verified files from a 1,153-blob upstream inventory. I did not independently re-materialize or re-audit that whole inventory. I did not rerun the coordinator's 1,099-link check or its preservation scripts; those remain supplied coordinator results. The independent byte/diff comparison above is my own observation. The research-lane and launch/retry account was not independently audited through author reports, which were expressly excluded.
>
> ## Material finding
>
> ### S21-R01 — Define the observable diagnostic exit for the new measurement
>
> **Severity / disposition:** medium handoff-completeness issue; documentation revision required. No product defect or missing native pass is alleged.
>
> **Requirement.** The supplied audit explicitly asks what real diagnostic output counts as success. The existing development-plan P2 contract, lines 428–432, requires a diagnostic consumer to expose actual admitted results and failure reasons. The new card is intended to turn those general requirements into a concrete integration handoff.
>
> **Location.** `tasks/engine-transport-pilot.md`, the first bounded implementation candidate at line 1142, the checked-result row at line 1156, acceptance at lines 1182–1186, and the real-UDP/Core controls at lines 1197 and 1202.
>
> **Evidence.** The card specifies an experimental measurement and “diagnostic projection,” retained checked values and comparisons against real returned values. It does not specify the minimum observable diagnostic result or tie the existing diagnostic consumer to those values. The present implementation, `crates/nevrotick-engine/src/bin/nevrotick-observe.rs:164–189`, writes slot/status, `authentication=none`, `eligible=0`, send/receive/rejection counts, optional stratum and final elapsed time. It does not expose phase, signed RTT, mapping/error status or a specific rejection reason. The new card's checked-result retention requirement is useful but does not itself require those results to reach the diagnostic output. The actual returned-value checks can pass while the CLI remains at its old transport-only output.
>
> **Consequence.** The documentation leaves the externally observable success criterion ambiguous: an implementation can produce correct internal arithmetic and retain the old status-only projection without demonstrating the requested diagnostic measurement. Likewise, an unavailable mapping or rejected numerical admission can remain indistinguishable at the diagnostic surface except through generic transport status/counts. This is a missing handoff requirement now, distinct from correctly deferred uncertainty derivation or native qualification later.
>
> **Smallest correction / owner.** The engine/protocol documentation owner should add one compact diagnostic acceptance paragraph or row to this existing card. Name the consumer (`nevrotick-observe` or an explicitly identified replacement), distinguish transport-only, experimental-measurement and later Core readiness outcomes, and require the successful projection to expose the actual checked phase and signed raw RTT with units plus the relevant acquisition/reference-age and capture/mapping/error qualifiers. Specify bounded allowlisted reasons when mapping, era/arithmetic, context/deadline or later statistical admission prevents a result; absent measurements must not become numeric zero or success. Bind a healthy real-UDP output check to the independently reconstructed checked result, and a refused neighbor to the expected reason with no fresh history/estimate. Preserve `authentication=none eligible=0` for the first exit. Exact field spelling and the later production uncertainty formula can remain with their existing owners; no source change, build or native probe is needed to fix this documentation omission.
>
> ## Findings checked and not raised
>
> The central source description is accurate. The engine manifest contains `ntp-wire`, Tokio and getrandom, with no exchange/domain/Core dependencies. Its staged/runtime facts lose the metadata identified by the card. The fixture association retains LI, precision, root and reference bits, unfolds explicit era windows, uses saved real T1 and performs exact arithmetic. The separate composition consumes a completion once, checks binding/generations/mapping, uses fixture arrival and divisor `1.0`, and actually appends to history. The card correctly refuses to promote that fixture recipe into a production reference-time or uncertainty contract.
>
> The numerical examples are internally consistent by independent calculation and source inspection: the dyadic tuple gives phase `1/64 s` and raw RTT `5/64 s`, with the stated Q33/Q32 numerators; the half-unit and ties-to-even examples preserve signed residuals; the asymmetric-delay example has zero estimated phase despite a true positive offset; the three era windows distinguish one, multiple and no candidates. Leap interpretation explicitly uses inclusive event membership but applies the after-event offset at a transition lower endpoint, supporting the `[T,T]` distinction. Core normalization admits sufficient counts from 3 through 64 and uses the stated 62,500,000 ns minimum span. These are reviewed examples and current source behavior, not executed controls or whole-domain proofs.
>
> The external representation claim is supported by the actual pinned decoders: [Chrony `UTI_Ntp32ToDouble`](https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/util.c#L650-L679) and its [ordinary header calls](https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/ntp_core.c#L2017-L2023), together with [ntpd-rs short conversion](https://github.com/pendulum-project/ntpd-rs/blob/18a5bd6a87a65033a41c36dc8b4b737c98b5523f/ntp-proto/src/time_types.rs#L229-L233) and [header decoding](https://github.com/pendulum-project/ntpd-rs/blob/18a5bd6a87a65033a41c36dc8b4b737c98b5523f/ntp-proto/src/packet/mod.rs#L199-L218), treat both ordinary root fields as unsigned. The pinned [ntpd-rs consumer](https://github.com/pendulum-project/ntpd-rs/blob/18a5bd6a87a65033a41c36dc8b4b737c98b5523f/ntp-proto/src/source.rs#L714-L739) does use `.into()`. The candidate accurately retains the distinction from the signed root-delay wording in [RFC 4330 §4](https://www.rfc-editor.org/rfc/rfc4330.html#section-4), and does not claim a runtime decoder defect from the local raw-bit field's comment.
>
> The timestamp/native distinctions are also supported. [Microsoft's QPC contract](https://learn.microsoft.com/en-us/windows/win32/sysinfo/acquiring-high-resolution-time-stamps) separates QPC from system time and UTC. [recvfrom](https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-recvfrom) can return queued data and discard excess UDP data on WSAEMSGSIZE. The actual Rust error variant has no returned peer address. [CancelIoEx](https://learn.microsoft.com/en-us/windows/win32/api/ioapiset/nf-ioapiset-cancelioex) and [closesocket](https://learn.microsoft.com/en-us/windows/win32/api/winsock2/nf-winsock2-closesocket) do not make cancellation/close equivalent to completed native I/O. The candidate treats these as API contracts and proposed controls, and expressly leaves Tokio's selected backend and native observations open.
>
> The changes preserve the current priority and authority boundaries: S1.1c remains first; OD-10's full-domain mathematical proofs remain mandatory; OD-09's silent-source/leap and Windows ownership decisions remain intact; OD-05 §7.5's enhanced release provenance remains optional. W11-W01 is consistently labelled partial preparation. Historical selected Linux fixture completion does not become P1, S2.1, native or production qualification.
>
> ## Intentionally pending work and evidence limits
>
> The finite capture convention, mapping validity/error/invalidation profile, supported experimental date scope, selected SDK/backend details and unsupported outcomes must still be frozen before dependent implementation. Later genuine Core admission also needs the reference-time and uncertainty/divisor contract. DNS, scheduling/KoD integration, socket policy, full mathematical proofs, native completion/resource evidence and production eligibility remain with their existing owners. Their absence is correctly disclosed and is not an additional finding against this preparation stage.
>
> Initial web opens of the four pinned GitHub files failed with `DisabledError`; the GitHub connector subsequently returned their exact pinned source, resolving that retrieval limitation. Direct erratum 4504 retrieval failed on the RFC Editor redirect; the official RFC Editor search result and its inline verified-errata rendering supplied the verified status and corrected current-day wording. Other required primary claims were retrieved. No product/native observation was inferred from documentation retrieval, and no historical test receipt was reproduced.
>
> **One permitted next action:** make the documentation-only correction for S21-R01 in the existing engine card and submit that changed text for a separately assigned bounded correction verification. Keep the other accepted content and the proof-first execution order; this review grants no product execution or publication authority.

#### Targeted correction verification — completed, READY

> # S2.1 — bounded correction verification
>
> **Documentation-stage verdict: READY.** **S21-R01 is CLOSED** on the corrected candidate identified below. The two insertions supply the observable diagnostic contract and paired consumer checks missing from the initial handoff. This verification reuses the unaffected findings and limits of the completed independent review; it is not a repeated whole-candidate audit or an implementation, proof, native-Windows or release pass.
>
> ## Exact candidate
>
> | Identity | Value |
> | --- | --- |
> | Repository / checkout | `DavidOsipov/NevroTick`, `/workspace/scratch/68e2481482dd/NevroTick-s21` |
> | Supplied upstream | `8b8d0142ab1daae50509b6ec2cea800ada033449` |
> | Supplied upstream tree | `70103638b117960a91a259a740a8ed13c8afe8f2` |
> | Synthetic comparison baseline | `3316356d58480dcf86b08132272bed613f86ca4f`; not upstream history |
> | Corrected manifest | `research/s21-corrected-candidate.json` |
> | Corrected full diff SHA-256 | `c77152a73b0cfe7f86051fc0f256f46698101af51ddb10a9ea1e297d5015adc4` |
> | Correction-only diff SHA-256 | `6786c17cad2e7bb85c62d91131fd192892edd2989c1a348b51058c3d2f0a73a9` |
> | Corrected engine-task SHA-256 / length | `236e174fdf1fd225e699b4b3a1dce46a8b882db09f4fd717e397dfaed51d49be` / 97,293 bytes |
> | Initial engine-task SHA-256 | `532231a6757ea60c5b8d2cd162d7e05112ea8c0e4ed4ae37426c6d3f409248af` |
> | Immutable initial review | `research/s21-independent-review.md`, SHA-256 `6e9ce14ea3d0d1867f6f42bbd506b357753e4f4adb9f52f4de36258b2e19f110` |
>
> I independently checked the current bytes against all eight top-level corrected-manifest entries. All hashes and lengths match. The other seven documents are byte-identical to the initial reviewed candidate. The correction has two nonblank additions and no deletions: one paragraph with its separating blank line and one table row. Removing exactly those additions in memory recovers the initial engine-task hash. The actual `git diff` matches the corrected full-diff hash, and the correction-only diff matches its recorded hash. The initial review report retains its original hash.
>
> ## S21-R01 disposition
>
> **Original requirement:** the integration card must identify the real diagnostic consumer and define observable successful and refused outcomes, rather than allowing transport status/counts to stand in for the new checked measurement. The initial review located the gap in the new first-measurement/acceptance wording and the actual `nevrotick-observe::write_report` consumer.
>
> | Affected guarantee | Corrected text and verification |
> | --- | --- |
> | Named consumer and distinct stages | The new **Diagnostic exit** paragraph names `nevrotick-observe` and separates transport-only observations, successful experimental measurements and later Core insufficient/ready/rejected outcomes. |
> | Actual measurement reaches output | The paragraph requires checked phase and signed raw RTT, explicit units, exact representation or rounding residual/error, acquisition/reference age and capture/mapping/error qualifiers. The new control requires emitted values and qualifiers to match independently reconstructed observations and peer bytes. Internal arithmetic alone can no longer satisfy the stated diagnostic exit. |
> | Meaningful refusal and honest absence | The paragraph requires bounded allowlisted reasons for unavailable mapping, era/arithmetic failure, stale context/deadline and later statistical refusal. Missing measurement or estimate remains absent. The paired control checks the expected reason, absent result and no added observation or refreshed history/estimate. |
> | Existing authority and stage limits | Both additions retain `authentication=none eligible=0` for the first exit. Later Core outcomes must not invent a fit. The diagnostic observer must not perform the claimed transition. Raw error strings and packet dumps are excluded from the diagnostic projection; independent captured test evidence remains covered by the separate real-UDP control. |
>
> The inspected `write_report` source still emits the existing transport-only fields. The new wording correctly instructs a future extension; it does not claim that the source already emits these measurements. Its surrounding acceptance section continues to label the controls **specified, not executed**. No change to the production uncertainty formula, schema/framework selection, proof-first order or execution permissions is introduced.
>
> **Disposition:** the documentation correction fully addresses S21-R01 within its original scope. No further material finding was identified in the two insertions. No product repair or execution result is credited by closing this documentation finding.
>
> ## Actual read and check scope
>
> This targeted pass read the correction-only diff, the corrected manifest, the initial candidate manifest for identity comparison, the current engine card around the two insertions and acceptance controls, and `crates/nevrotick-engine/src/bin/nevrotick-observe.rs`'s actual `write_report` body. It performed read-only hashing, in-memory reconstruction, `git diff` and `git status` checks. The status remains exactly the eight expected modified Markdown paths.
>
> The initial independent review supplies the unchanged requirements, source tracing, numerical/primary-source review and evidence limitations. No author research or analysis reports, private context or memory were read. No external research was repeated because the correction adds no new consequential external factual claim. There were no unavailable required inputs in this targeted pass.
>
> The corrected manifest's nested `validation.links.sha256` still carries the initial engine-task hash. I treat that nested validation record as retained initial evidence, not a newly executed check of the corrected engine bytes. The two additions introduce no links, anchors or diagrams; this targeted verification independently checks the actual corrected bytes and does not claim a fresh full link-validation run.
>
> Only this correction-verification report was written. The initial report and repository files were left unchanged. No product code, build, test, proof, native operation, installation, agent launch or remote write was executed.
>
> ## Remaining limits and next action
>
> READY applies to the eight-file **documentation preparation and handoff** after the correction. The finite capture/mapping/date profile, production reference-time and uncertainty/divisor contract, actual diagnostic implementation, runtime-to-Core integration, mathematical proof obligations and native/resource evidence retain their existing owners and remain open as previously recorded. W11-W01 remains partial source preparation; S1.1c remains first in the development queue.
>
> The coordinator may record this closure against the exact corrected candidate and continue its already-authorized documentation handoff. This verification adds no execution, publication or release authority.

#### Coordinator checks and evidence boundaries

The corrected eight-file documentation candidate passed `git diff --check` and
the focused consistency checks. The refreshed link check covered **1,099 local
links/headings**, with no new or pre-existing failures and no unavailable anchor
targets. This refreshed run binds the corrected engine bytes; the initial nested
validation hashes discussed in the targeted report remain historical evidence.

The original engine body from its authority section through S44, W11 historical
receipts §4.1–§4.25, the proof-first queue prefix, all five architecture Mermaid
blocks, canonical REQ/BND/DEC/THR/CHECK/OPEN identities and existing architecture
source entries were preserved. Exactly the eight approved Markdown paths changed;
product code, dependency and skill-package files did not change.

The first preservation-script run incorrectly included W11's editable current
§4 evidence table in the historical prefix and reported a failure. The coordinator
retained that initial result, corrected the check to the actual historical receipt
boundary at §4.1, and reran it successfully without changing repository bytes to
satisfy the check. This was a checker-scope correction, not a product or historical
receipt repair. The full upstream inventory contains no `docs/SHA256SUMS.txt`;
no exported-bundle integrity claim is made.

The coordinator independently recomputed the dyadic and ideal-decimal examples
with exact rational arithmetic. That checks the written oracle values, not the
product implementation or the mandatory whole-domain proofs. No product build,
test, solver, native Windows, clock/service or public-time-server experiment ran
for this documentation delivery. The full upstream tree remains the publication
base; the partial synthetic checkout is never published as repository history.

## Address-family admission and bounded acquisition progress — 5 November 2026

**Disposition:** required future acquisition contract; specified, not implemented
or qualified. This addendum starts from `8a66af924c509e898a9bab9bb3282f11b24fba93`
and implements the documentation handoff for
[architecture §7.3](../docs/windows-time-architecture.md#address-family-admission-and-alternative-progress).
The [S1.1c proof-first queue][s21-development] remains first. The current two-peer
loopback pilot and its first experimental measurement retain their bounded scope;
neither gains production DNS, retry, NTS or local-path qualification here.

### Contract and owner handoff

Engine/protocol owns candidate admission, attempt ordering, actual next-send
guards and source/association state. The Windows platform owner supplies bounded
local observations and their validity; reuse the
[network reconciliation contract](../docs/plans/windows11-integration.md#exclusive-loopback-endpoints-and-reconciled-network-changes)
and current network/resume generations. Configuration's allowed families and
destination scope remain separate admission rules. Resolve and bound the admitted
A/AAAA candidate set; multiple addresses of one logical source add no votes.

Before a candidate attempt, distinguish these conceptual dispositions; this table
does not select Rust types or Windows APIs:

| Local evidence for this candidate | Required decision |
| --- | --- |
| Explicitly disabled | Current authoritative interface/family state excludes this local path: skip it. One disabled interface must not exclude another usable interface of that family. |
| No local path | Current destination-specific route/source-address evidence conclusively establishes no usable admitted local path: skip this candidate until relevant reconciliation. Dormant/on-demand state alone is not such evidence. This is not proof that the peer failed. |
| Eligible | A current permitted interface, usable source address and destination route support an attempt. Merely enumerated tentative, duplicate or expired addresses do not establish usability; local eligibility does not establish remote reachability, TLS success or usable protected UDP. |
| Unknown | Missing, failed, incomplete or stale observations do not mean disabled. Refresh or attempt within bounds under current destination policy; inspection success is not a prerequisite, and repeated observation failures cannot indefinitely suppress an otherwise permitted candidate. |

Dormant/on-demand paths may become active through ordinary permitted traffic.
Retain a bounded attempt or resolution/recovery path where that OS-configured
activation is applicable; repeatedly inspecting an inactive path must not prevent
its normal activation. This authorizes no adapter/VPN enablement, configuration
change or destination-policy bypass.

Bind observations to the relevant configuration/resolution and network/interface/
route generation, source-address validity and candidate destination. Do not use a
single machine-wide IPv4/IPv6 Boolean. Cache bounded snapshots outside the packet
hot path; do not enumerate adapters/routes for each datagram. Initial acquisition,
network/address/route/VPN changes and resume reconcile actual state through the
existing coalesced owner. Recheck relevant validity before scheduling/publication;
old callbacks cannot restore a superseded snapshot or retire a healthy successor.
An observation failure remains distinguishable from a confirmed unavailable path.
Treat multi-call native observations as potentially non-atomic. Freeze a bounded
recheck policy for skipped/unknown candidates even when no new notification
arrives, so missed notifications cannot make a local exclusion permanent.

A permitted LAN/VPN destination can be usable without general Internet status or
a default Internet route. Connectivity hints, DNS filtering and address presence
alone do not prove destination reachability or justify a global family ban.
Preserve the existing
[negative-Internet-hint rule](../docs/plans/windows11-integration-follow-up.md#native-ip-helper-hints-may-satisfy-a-narrower-cost-requirement).
Report a local skip with a bounded reason, without charging it as a failed peer
exchange or weakening source agreement. Observation work still consumes its own
resource budget. Enumeration, source/route selection and resolver-filter methods
remain candidates until their exact SDK bindings, semantics and native behavior
are qualified; no API choice or new dependency follows from this table.

### Bounded progress and NTS boundaries

Freeze bounded ordering or interleaving, per-attempt deadlines and an acquisition
progress bound before implementation. Within a permitted round, a stalled first
address must yield an attempt to another admitted usable alternative within that
bound; repeated refresh/DNS events cannot restart the first choice indefinitely.
Unchanged or merely reordered DNS results do not reset alternative progress.
Cover both A-before-AAAA and AAAA-before-A, including several addresses of one
family. Unknown local state is not silently removed from recovery consideration.
Global denial/exhaustion produces bounded diagnosed deferral/unavailability, not
extra retries. Retain shared earliest-operation, backoff, per-source/aggregate,
socket/task/byte and native-completion budgets. Select no timeout/race constants,
full Happy Eyeballs implementation, default parallel race or dependency here.

Track NTS-KE TCP/TLS reachability separately from the negotiated protected-UDP
destination. A successful handshake cannot indefinitely pin acquisition to an
unusable UDP path. Each alternative remains bound to the relevant authenticated
server identity, that transaction's negotiated hostname/port and destination
policy, current association/key/request bindings and consume-once admission.
Newly negotiated destinations require current re-admission; no arbitrary
redirected endpoint or plain-NTP fallback is permitted. Preserve atomic
replacement and a still-valid old
association. Only the already-required KE plus authenticated UDP success using
that transaction's keys/data can reset its backoff. Family/address changes create
neither independent sources nor fresh accepted measurements by themselves.

### Next implementation sequence and independent controls

After the current S1.1c continuation, follow the existing S2.1 integration order:
retain the first controlled measurement exit, then freeze this production
acquisition policy with engine/protocol and platform before dependent DNS/socket
work. First implement the bounded portable decision/state owner and real sender
guards; then qualify the selected Windows observation/reconciliation adapter with
controlled peers when its execution prerequisites permit. S2.2 adds the actual
KE-to-protected-UDP alternative/replacement path after its existing trust/provider
gates. Retain required source-bound proofs for any reached mathematical algorithm;
portable controls do not establish native observation or complete acquisition.

The following expected outcomes come from the contract, not a translated
implementation. Each defect control must reach its named guard and retain the
healthy neighbor. These are planned cases, with no execution result recorded:

| Case and healthy/defect pair | Independent expected result | Evidence owner and existing checks, only as reached |
| --- | --- | --- |
| Disabled family versus enabled usable interface; same family usable through another interface | No socket attempt on the confirmed disabled path; the admitted usable neighbor can proceed. No peer-failure increment or extra source vote follows from the skip. | Engine/platform: pure admission controls, then actual Windows configuration/attempt observations; CHECK-25/27, CHECK-8 for vote effects. |
| Permitted dormant/on-demand path versus a genuinely disabled path | Bounded ordinary activity can reach the allowed activation path; preflight-only polling cannot strand it. The genuinely disabled path remains skipped, with no peer-failure charge or application change to adapter/VPN configuration. | Platform/engine: pure classification/activation controls, then actual selected Windows demand-activation behavior; CHECK-25/27. |
| LAN/VPN route with negative Internet hint versus no admissible local source/route | The permitted reachable peer remains attemptable without public Internet; confirmed local absence suppresses only the affected candidate. | Platform/engine: pure decision pair plus controlled native LAN/VPN route and emitted-attempt evidence; CHECK-25/27. |
| Fresh evidence versus failed/stale or repeatedly failing inspection while an admitted controlled endpoint works; later current snapshot restores a path | Unknown metadata cannot fabricate disabled state or permanent exclusion. Bounded permitted attempts still reach the working peer without successful inspection; current reconciled evidence restores the corresponding path decision. | Platform/engine: pure transitions plus native inspection failures, actual attempts and change notifications; CHECK-25/27. |
| Usable source address versus tentative/duplicate/expired address; missed notification versus bounded recheck | Enumeration alone cannot admit an unusable local address; fresh bounded re-observation can restore a recovered path without waiting forever for an event. | Platform/engine: pure validity/refresh controls, then actual selected native observations; CHECK-25/27. |
| First A/AAAA address blackholes versus a responding alternative, with reversed family/order and same-family alternatives | The alternative receives an attempt within the frozen progress bound; repeated triggers cannot monopolize the first address or enlarge the budget. No promised success when every admitted path fails. | Engine: deterministic scheduling controls, then controlled real traffic; CHECK-21/27 for reached bounds and lifetime, without claiming their full soak exit. |
| KE succeeds but protected UDP fails versus a working authorized alternative; wrong-key/old-context reply versus current authenticated reply | No false acquisition success or retry reset; permitted alternative recovery preserves identity/key/destination bindings. Invalid replies publish no time/cookies; the current valid reply completes once. | Protocol/platform: pure state controls first, then actual TLS/AEAD/UDP integration; CHECK-6/27, CHECK-9 only where certificate/association recovery is exercised. |
| Current network snapshot/completion versus route change, resume or cancellation before publication | Old snapshot, attempt and late completion cannot affect the successor; actual native work retains its permits until completion and a fresh neighbor can proceed. | Engine/platform: controlled interleavings plus actual native callback/completion ownership; CHECK-25/27. |
| Several addresses for one source plus repeated reconnect events versus ordinary healthy acquisition | At most one vote per eligible logical source, with no new freshness from address changes; shared limits contain attempts, snapshots, callbacks and diagnostics, while admitted healthy work and shutdown retain progress. | Engine/core/platform: pure identity/scheduler controls and measured native resources before teardown; CHECK-8/21/27. |

Record future candidate/configuration/tool/fixture identities, actual attempts,
first failures, cleanup and remaining gaps in this task. Native adapter/resource
coverage and execution holds remain with their existing Windows/CHECK owners.
Historical receipts above establish none of these new results; the coordinator's
documentation review receipt is separate from future implementation acceptance.

### Documentation integration and review receipt

**Candidate:** base `8a66af924c509e898a9bab9bb3282f11b24fba93`, with additive
changes to architecture §7.3/S126–S127, development-plan P2/next-task routing, this
task and Windows task §4.27. The corrected contract diff SHA-256 is
`08f9df6b73d688fe217a3315fe0be17dea13a7b9609f953f4adf6ac98334dfe2`;
this receipt follows that reviewed contract and records no new product behavior.

**Review completion and context:** `hifitime_practices` completed the bounded
Microsoft/RFC source review; `hifitime_review_writer` authored this task card.
The separate read-only `hifitime_algorithms` reviewer completed the integrated
four-file review with retained prior research context, then verified its scoped
correction. It is not a fresh-context review. Two explicit fresh-context launches
(`network_contract_review`, `network_contract_review_v2`) failed with context-limit
errors and returned no usable review; their review scope is incomplete, with no
input-read or zero-finding claim. No further retry or cold-review result is claimed.

**Completed reviewer's comprehension, preserved verbatim:**

> The amendment adds local interface/address/route admission and bounded progress across permitted IPv4/IPv6 destinations to the existing acquisition plan. The engine owns scheduling and source identity; a platform adapter supplies observations with explicit validity and unknown outcomes. NTS-KE and protected UDP remain separate stages, while address changes preserve the existing authentication, association, budget and single-vote rules.

> The current engine remains a two-peer controlled-loopback transport pilot with one query per association and no DNS or retry machinery. Its first experimental measurement remains unauthenticated and ineligible for discipline. The new contract belongs to later S2.1/S2.2 integration; it does not implement that behavior, expand the first measurement, advance native qualification or replace the S1.1c proof-first continuation.

**AF-R01 — Medium documentation liveness; closed on the corrected candidate.**
The initial diff `ea9e44d84fac6f4dd6c7c2f012cb9f51a57bac7ed0f83d016d0da65785a39b8a`
did not explicitly distinguish dormant/on-demand activation from conclusive local
unavailability. The reviewer identified that observation-only polling could
prevent the ordinary network activity needed for activation. The coordinator
independently checked S126's `IfOperStatusDormant` and
`NET_IF_CONNECTION_DEMAND` descriptions. Architecture, disposition, paired control
and plan/Windows routing now preserve bounded ordinary permitted activation while
retaining genuinely disabled-path skipping and no configuration authority. The
same reviewer inspected those corrections and returned **READY for documentary
integration**, reusing its unaffected conclusions. This is no observed runtime
defect, native pass or implemented recovery claim.

**Documentation checks:** `git diff --check` passed. A focused check resolved
**570 local link targets** across the four changed files; Markdown fences are
balanced, all **127** architecture source IDs are unique/contiguous and cited
source IDs are defined. All historical lines in the four files are preserved.
The first link-check attempt misread an indented list continuation as a reference
definition; the scoped checker was corrected and rerun successfully without
changing repository text for that false positive. The tracked tree contains no
root `docs/SHA256SUMS.txt`; the unchanged design-kit manifests are outside this
delta. The coordinator rechecks the receipt and final publication bytes separately.

**Current exit:** the required contract, enforcing owners and nine paired future
controls are recorded. No product build/test, mathematical proof, native Windows
operation or public-server probe ran for this documentation change. Complete the
existing S1.1c continuation first, then freeze the finite acquisition profile at
its dependent S2.1/S2.2 integration and record actual implementation/native results
here under the existing checks and execution prerequisites.

## Network path history and user routing — 5 November 2026

**Scope and authority:** David's follow-up asks for upstream IPv4/IPv6 availability,
reuse of successful connections and interface/proxy/family controls. This extends
the preceding address-family card and architecture
[observed path history and user network policy](../docs/windows-time-architecture.md#observed-path-history-and-user-network-policy).
Starting main: `8f1a3b8afab82ea736fb9ea44c99160c90a5075c`, including the intervening
proof-workflow changes. The current two-peer loopback pilot implements none of
this card's learned routing or proxy behavior. Keep the existing first controlled
measurement and development §7's public-constructor proof continuation separate.

**Later scope disposition:** [OD-11](../docs/nevrotick-owner-decisions.md#15-od-11--socks5-scope-and-deferral)
and [SOCKS-0](../ROADMAP.md#socks-0--optional-socks5-time-acquisition) defer every
proxy-specific portion of this card beyond first Windows P0–P7 delivery.
Direct family/interface behavior, live path reuse and shared policy/storage
contracts remain current P2/P5 work. The [limited future profile and check plan](#socks5-feasibility-and-deferral--5-november-2026)
supersede the earlier broad first-profile proposal; historical reviews below keep
their original scope and do not qualify that newer subset.

### Contract freeze and implementation order

1. **Engine/domain owner, before dependent S2.1 scheduling:** define the single
   acquisition profile, hard/soft family semantics, stable adapter identity,
   direct immediate-hop meaning, finite path-history key,
   monotonic age/cooldown/reprobe/hysteresis rules and aggregate bounds. Learned
   state can reorder permitted paths, never grant permission, fresh time or votes.
   Reuse DATA-06 machine policy, DATA-08/09 contexts, DATA-10 outcomes and DATA-17
   scheduler state through [architecture §12.1.3](../docs/windows-time-architecture.md#1213-network-state-reuses-existing-storage-owners).
   Add no second cache owner/database or duplicate generation. Initial hints are
   RAM-only; later Q history remains with the existing catalogue/statistics task.
   Include raw configuration/IPC bypasses and checked arithmetic/proof obligations
   under CHECK-2/3/6/8/21/25/27. No guessed timer constant becomes a release promise.
2. **Scheduler owner:** reuse a still-current working path; classify local exclusion,
   transport failure, name resolution, certificate/authentication and protocol
   rejection separately. Bound every alternate and recovery probe through the
   existing scheduling owner. Keep a reachable healthy alternative; invalid metadata
   or a single silent endpoint cannot exclude a family globally. Cache expiry,
   unchanged DNS refreshes and network notifications do not refill budgets.
   One attempt/stage fact updates live state and the bounded optional-history
   handoff; no database/codec write blocks the timing path. Keep not-run stages,
   local exclusions and telemetry loss separate from actual peer failures.
3. **Windows adapter owner:** qualify explicit binding and actual source/route,
   initial and changed network context, stale indices and failure races through
   [Windows §4.28](windows11-integration.md#428-explicit-network-profile-qualification--5-november-2026).
   Platform observations do not establish ISP reachability. Preserve permitted
   local/VPN/on-demand paths and the prior timestamp/native-work custody contracts.
4. **Protocol/security owner, deferred SOCKS-0:** after ordinary S2.2 and first
   Windows delivery, freeze the limited manual loopback/no-auth/local-DNS
   SOCKS5 profile, exact licensed dependency choice
   if any, relay-address policy, bounded parser, TCP-control ownership, UDP wrapper
   and inner packet/timestamp handling. Qualify NTS-KE and protected UDP separately,
   preserving TLS identity, delegation, association and the full KE retry-reset
   rule. Resolve the known/unknown actual-KE-peer and default-endpoint boundary
   through architecture §7.4 before admitting a proxy route. A proxy's reply or
   transport handshake is not an accepted time sample.
5. **Resolver/trust/secret owners, proxy portions deferred to SOCKS-0:** the first
   future subset uses local DNS and numeric proxy/target addresses. Any later
   proxy DNS or proxy-host bootstrap resolution must be explicit; audit selected verifier retrieval before claiming a strict
   proxy-only acquisition path. Enforce destination scope before target I/O;
   reject unsupported family/address restrictions with domain forwarding. Preserve
   OPEN-8 and service-owned secret/storage gates;
   no weaker certificate checks, ambient user credentials or plaintext fallback.
   Only a later authenticated SOCKS5 extension adds a separately qualified
   credential purpose to ST-W01; the first subset has none. The NTS/DNS
   optional-cache matrix does not authorize missing-password
   recovery. Reuse protected mechanisms, not their payloads or secret identities.
6. **UI owner, full-product settings/P5:** implement the
   [feature §3.3.1 controls](../docs/windows-time-features.md#331-time-network-settings--5-november-2026)
   for Direct and the direct/shared parts of AC-21/22; proxy controls remain
   deferred to SOCKS-0. Use authorized policy Apply, with requested/effective state and
   current remedies. UI-M0, HTML design references and public-server probing are
   not part of this documentation slice.
   Persist the existing machine-policy revision separately from activation;
   qualify old-generation I/O retirement and truthful pending/blocked/effective
   acknowledgements. A saved revision cannot prove already submitted traffic was
   recalled or that another process changed its route.

First Windows delivery uses Direct, with Automatic or a pinned interface and
Automatic/Only IPv4/Only IPv6/Prefer IPv6/Prefer IPv4. The first future SOCKS-0
subset uses the architecture's numeric loopback endpoint, Automatic interface,
no authentication and explicitly local DNS. Proxy DNS/remote endpoints/credentials
are separate later SOCKS5 extensions. No hidden direct escape, split KE-only proxy,
PAC/system import, SOCKS4 or HTTP proxy support is part of this direction.
Unsupported profiles remain visibly unavailable, including on import/downgrade;
do not interpret a rejected proxy policy as Direct.

### Paired acceptance cases — planned, not executed

Each row requires the real relevant producer/consumer and a healthy neighbor.
Portable policy/codec cases precede native routes and controlled proxy fixtures;
neither mocks nor source inspection close Windows or timing evidence.
After OD-11, direct/shared portions retain their current owners; every proxy-only
portion is deferred to SOCKS-0. Proxy-DNS and credential portions additionally
require their later extension. The first subset's local DNS is explicitly allowed,
not the silent resolver escape rejected by PATH-08. The applicability split does
not turn a deferred case into a pass or weaken current direct-path acceptance.

| Case | Failure/control and required observation | Existing owners |
| --- | --- | --- |
| PATH-01 | Both families locally usable; IPv6 upstream black hole with working IPv4, then reverse. Bounded fallback works and is reused. A concurrent permitted IPv6 LAN/VPN target remains usable. | CHECK-6/9/21/27; AC-21 |
| PATH-02 | One failed endpoint versus a healthy endpoint in the same family. A failed TCP/UDP port or certificate does not become an ISP-wide family prohibition. | CHECK-6/9; AC-21 |
| PATH-03 | TCP/proxy-login/KE success followed by blocked UDP versus a current accepted NTP reply. Include omitted NTS Server Negotiation with an unknown proxy-resolved KE peer versus a known admitted KE target and valid default, and valid explicit delegation. Architecture §7.4 endpoint admission must succeed; no proxy-bind substitution or blind name re-resolution. Only the appropriate stage becomes successful; history supplies no measurement/authentication freshness or KE budget reset. | CHECK-6/8/9/14 |
| PATH-04 | Warm history versus expired DNS membership, changed attached network on the same adapter, address/route/VPN/resume, proxy/resolver or policy revision. Stale paths/completions cannot publish; valid unchanged history avoids repeated cold-start delays. | CHECK-6/14/21/27 |
| PATH-05 | Only-family mode versus soft preference with a recent working fallback; restore the preferred path. No forbidden-family send; bounded recovery and hysteresis avoid a timeout on every poll or continuous switching. | CHECK-6/9/21; AC-21/22 |
| PATH-06 | Selected adapter disappears or its index is reused versus the same valid identity at a current index. Fail visibly without migrating to another adapter; Automatic remains useful. | CHECK-6/25/27; AC-22 |
| PATH-07 | SOCKS TCP CONNECT succeeds but UDP ASSOCIATE is unsupported or UDP cannot pass versus a working relay. Retain the separate control connection; its loss invalidates old relay work and triggers bounded reacquisition. | CHECK-6/9/21/27 |
| PATH-08 | Proxy/DNS failure versus a healthy strict proxy route. Observe actual sends: no direct time packets, silent local target lookup, alternate proxy or NTS-to-NTP downgrade. Include proxy-host bootstrap resolution and verifier retrieval. | CHECK-2/6/9/17; AC-22 |
| PATH-09 | IPv4 client-to-proxy with IPv6 destination and the reverse, including separate relay/control addresses. Enforce each owned leg's profile; local family absence does not veto a reachable proxy-side family. | CHECK-6/9/25 |
| PATH-10 | Proxy-resolved name plus an unenforceable hard target-family/address or network-scope rule versus admitted typed numeric destinations under an explicit resolver policy. Include a name resolving to a forbidden private/loopback target beside an explicitly permitted LAN target. Require an enforcing owner before target I/O; later address discovery is insufficient. Invalid combinations cannot Apply or silently leak resolution; unsupported runtime outcomes preserve the profile. | CHECK-2/3/6/25; AC-22 |
| PATH-11 | Malformed/oversize relay address, header, reserved/fragment fields, wrong relay/source/association and late replay versus bounded valid standalone datagrams. Both wrapper and inner NTP/NTS guards must accept useful traffic and reject unauthorized effects. | CHECK-6/9/21/25/27 |
| PATH-12 | Loopback proxy available versus user logout, missing listener, physical-adapter pin that cannot constrain proxy egress, or a different local process claiming the port. Show service-context limits and retain end-server authentication; loopback is not trust. | CHECK-2/6/9/12/14 |
| PATH-13 | Direct versus encapsulated timestamp placement, excess delay/asymmetry and changed paths with old delay history. Preserve independent numeric truth, uncertainty and source/operator counts; adding a route grants no extra vote, and a shared relay remains a shared path. | CHECK-7/8/9/14/27 |
| PATH-14 | Supported proxy authentication versus unoffered/downgraded method, unavailable protected credential, logging/export and restart failure. Preserve credentials, service authority and current policy; pair each refusal with the qualified healthy profile. | CHECK-2/9/15/17/25 |
| PATH-15 | Working RAM path state with Q disabled, blocked, full or corrupt versus useful enabled history. Actual acquisition remains bounded and usable; the same attempt/stage completion updates each eligible consumer at most once. Local exclusions/not-run stages and history queue loss do not become peer failures or false complete coverage. | DATA-10/16/17/19/20; CHECK-6/9/17/21/27 |
| PATH-16 | Initial restart with no path hints; a future descriptive Q row with wrong context or unknown monotonic age; and a healthy current RAM hit. Re-admit current candidates without restoring old cooldown deadlines, pending requests, authentication or votes. No Q/file-format dependency is required for ordinary acquisition. | DATA-08/09/17; CHECK-6/9/15/25 |
| PATH-17 | Destination/interface/proxy/context churn, retained readers, queued history and native work reach the frozen total limit versus healthy bounded reuse/recovery. Include eviction and repeated reset/refresh. No new per-cache quota or unbounded retained copies; eviction cannot clear independent retry/security guards or starve permitted recovery. | DATA-20; CHECK-9/17/21/27 |
| PATH-18 | Wrong-purpose, old-revision, missing or partly replaced proxy credential, changed proxy recipient or loopback-port takeover versus the qualified current credential/channel. Policy contains only its opaque reference; NTS/DNS envelopes cannot satisfy the proxy purpose, and end-server NTS does not authenticate the proxy recipient. Unavailable restoration cannot enable no-authentication, Direct or another proxy, and logs/exports reveal no value. | ST-T03/05/07/10/14/17; CHECK-2/9/15/17/25 |
| PATH-19 | Apply Direct-to-proxy, adapter, resolver or credential changes while old sends/DNS/verifier/relay jobs are queued. Pair a proven precommit validation/persistence refusal with lost acknowledgement/uncertain commit, postcommit activation failure and a successful qualified transition. Only proven precommit rejection preserves the prior policy without reconciliation; unknown outcomes cannot imply rollback or blind retry. Saved and effective revisions remain distinct. No old revision starts new owned I/O after the established cutover; late results cannot publish. Retain actual native custody. | CHECK-2/3/6/14/25/27; AC-07/22 |
| PATH-20 | Clear optional history during a delayed Q write/read/export, then receive fresh legitimate outcomes. Retired data cannot republish as current or duplicate observations; gaps stay visible. The operation does not erase machine policy, credentials, NTS full-authentication age, DNS privacy barriers, mandatory conflicts or broker authority. | DATA-17/19/20/21/22; CHECK-2/9/15/17/25/27 |

### Integration and evidence status

The predecessor's local exclusions, unknown/on-demand recovery, bounded alternatives,
one-source vote and NTS transaction rules remain in force. This refinement adds
observed reachability/history and explicit user routing; it does not upgrade a
local socket/route observation into end-to-end success. The intervening proof
commit remains the current source for §7 sequencing; older receipt dates/hashes
in this task retain their historical scope. Primary sources S127–S131 are linked
from the canonical architecture; the Firefox screenshot supplies a UI example,
not evidence of UDP or native-service support.

The coordinator performed source research and authored this amendment. Fresh
research delegates `dual_stack_path_policy` and `ntp_proxy_contract` both failed
to start with a context-capacity error and returned no usable work; no inspected
inputs, independent findings or successful review are attributed to them. An
independent plan review remains **not run** and is required before the dependent
implementation handoff is called ready. Coordinator self-review and document
checks supply only their stated evidence. No product, native Windows, actual
proxy or new formal-proof execution is claimed by this card.

**Document checks on the authored candidate:** contract diff SHA-256
`4ee0993bdd7e9ee5401625b822551dc3c2bebd0c0f016cca51e06d183670ad77`
before this receipt. `git diff --check` passed. Focused validation checked 619
repository link targets, including headings in 30 Markdown targets, and 131
unique contiguous architecture source identifiers with no missing cited source.
Seven links initially appeared missing because their targets were excluded by
the sparse checkout; their skip-worktree entries and exact base-tree paths were
verified without creating replacement files. All pre-existing lines of the six
edited documents remain in their original order, and Markdown fences balance.
The subsequent heading-depth correction preserves its validated anchor; this
receipt adds no links or product semantics. These are documentation checks and
coordinator self-review, not independent review, executed PATH-01–14 cases or
proof of a supported proxy/interface implementation.

### Storage reuse self-review — 5 November 2026

**Request and candidate:** David required the network proposals to reuse the
repository's documented caches, database and storage ownership, followed by
coordinator self-review and a fresh-context AI review using available applicable
MCP. Base main is `8ac407a367bd2231076d12fe6e3db66e425f1281`. The twelve-file
contract candidate before this receipt has diff SHA-256
`5f05200e033c1370259133558a074ed4a7c26025ddcaf09e6374dbcdcaf9a346`.
This is a documentation repair and future-case preparation, not implementation
of a cache, database, proxy or configuration handler.

The coordinator inspected current architecture §§6/7.3/7.4/12.1/12.2/17.1,
DATA-06/08/09/10/15–23 and their RAM/C/Q/H/E/U allocation, the storage tables,
catalogue/statistics and protected-storage tasks, the DNS storage/privacy
contract, feature controls and Windows/UI handoffs. Current engine source and
its manifest establish the bounded loopback pilot; the reviewed learned-routing,
Q/H and production secret-store behavior remains planned. No whole-repository
runtime audit or completed provider/native qualification is claimed.

| Finding | Evidence and consequence in the predecessor | Applied documentary correction and future discriminator |
| --- | --- | --- |
| STORE-N01 — ownership gap | The §7.3 path-history paragraph introduced a cache without binding it to existing DATA-08/09/10/17 and Q. An implementer could create duplicate contexts, outcomes or a second store. | §12.1.3 and existing catalogue task now map one policy/context/outcome/scheduler chain; optional history uses the same Q writer and Q0/Q-J decision. PATH-15/16 distinguish live reuse from optional-storage availability. |
| STORE-N02 — credential-purpose gap | The phrase “existing service-owned secret lifecycle” did not identify that the admitted NTS/DNS payloads and optional-state matrix do not cover proxy passwords. Treating lost credentials like regenerable NTS state could imply an invalid fallback. | Separate-purpose preparation under ST-W01, opaque policy reference, no secret payload/root sharing by implication, explicit unavailable authenticated profile. PATH-14/18 cover supported credentials beside wrong-purpose, stale and missing state. |
| NET-N03 — saved/effective ambiguity | Atomic Apply described durable policy but not the retirement of old sends, resolver/verifier jobs or relay work. A UI could report the new route effective while old work still starts. | §7.3, feature and Windows/UI handoffs distinguish saved revision from qualified activation, final send guards and actual completion. PATH-19 covers successful cutover, precommit refusal and postcommit blocked/indeterminate activation. No packet-recall claim. |
| STORE-N04 — consumer and capacity gap | Cache-size wording did not connect retained readers, context multiplicities, optional Q pressure, duplicate stage outcomes and reset races to existing DATA-10/19/20 owners. | One bounded handoff, explicit gaps/local failure categories, shared total accounting and no restored cross-incarnation deadlines or refunded independent guards. PATH-15–17/20 challenge those boundaries beside useful reuse/recovery. |
| DOC-N05 — navigation drift | The documentation guide still advertised AC-01–AC-20 after the prior amendment added AC-21/22. | The existing guide now routes to AC-01–AC-22; acceptance rows retain planned status. |

These are coordinator findings and source-backed plan corrections, not observed
runtime defects or independent reviewer findings. No new database format, RAM
compression, numerical quota, cache daemon, provider or product default is
selected. The current public-constructor proof continuation and all scoped
Windows/storage/fuzz/UI holds remain. Mathematical age/accounting algorithms
still require OD-10 source-bound proofs at their dependent implementation exit.

**Independent-review attempts:** `storage_owner_inventory` and
`network_storage_cold_review` were both launched with `fork_turns: "none"`; the
latter received only an on-disk neutral request and frozen patch identity.
Both returned the platform error “Codex ran out of room in the model's context
window” and no usable analysis. Their inspected inputs, comprehension and findings
are unknown, not zero. A separate Liner MCP research-agent request supplied one
user message containing the public repository and exact patch, with no conversation
history. It returned `payment_method_required` and explicitly said the request was
not performed; it supplied no review. No billing change or repeated charge attempt
was made. The required fresh-context plan review remains **not run**; this receipt
does not make the dependent implementation handoff ready.

**Source/tool evidence:** GitHub MCP verified repository/base identity. Microsoft
Learn search and full fetch confirmed `CancelIoEx` requests cancellation without
waiting for completion; the existing architecture S109 is reused. Context7
resolution and query retrieved official SQLite journal/temporary-file guidance;
the primary PRAGMA page confirms `max_page_count` concerns the main file and
`journal_size_limit` is not an aggregate live-family quota. These support existing
contracts, not a selected SQLite dependency. Applicable local Git/file reads and
primary-web checks were used; Serena, SocratiCode, Probe and Context-mode were not
exposed in this session, and no local Codex/Claude/Gemini CLI was found. Unrelated
MCP applications cannot supply repository-review evidence.

**Document validation before the repository-hosted review attempt:**
`git diff --check` passed. The focused check resolved 935 active relative links
across 107 target paths, read heading targets in 37 Markdown documents, and found
no missing path/anchor. Fourteen pre-existing fenced blocks, including verbatim
historical reviews, remained byte-identical. The architecture retains 131 unique
contiguous source IDs with all cited IDs defined; PATH-01–20 occur once each and
remain planned. `docs/SHA256SUMS.txt` is absent, so no bundle-manifest pass is
claimed. The pre-receipt diff SHA-256 is
`9752935cc113824119af45bd83b16b4d4e3a03663162f94d0c2d529684e78d41`.
No Rust, proxy, storage, Windows-native or new formal-proof execution was needed
or claimed for these documentation edits. A later review result must identify
its actual candidate and inspected coverage before changing review readiness.

**Repository-hosted fallback:** a GitHub MCP request named only
`copilot-pull-request-reviewer[bot]` on PR #5, candidate commit
`a13801dfbc4ff96ba7eeeae4d0bfee68660be489`, tree
`ce03f2dfd313cb86b9814bd643195bf1bc472380`. The connector returned “Action
completed”, but read-back at 09:34 UTC showed no requested reviewers, no review
request event and no review result. Dispatch is therefore **unconfirmed**, not a
completed review or evidence of a licence problem. No human reviewer was requested.
No MCP-use trace or independent comprehension/findings can be attributed to it.

The coordinator's final wording check also clarified that each real retry has
its own DATA-10 identity, while duplicate processing does not count it twice,
and that proxy DNS leaves the remote address unknown rather than fabricating a
local resolver result for the path key. The frozen review-attempt identities above
remain historical; they do not review these subsequent clarifications.

**Next permitted action:** a working fresh session reads AGENTS.md, the current
network card, architecture §§7.3/7.4/12.1–12.2/17.1, DATA-06/08–10/16–22, storage
tables and protected-storage/DNS/current-source owners. It states its own
comprehension of the proposal and actual system before critique, checks the
current commit plus dirty bytes, uses available applicable MCP and returns a
scoped `ready`/`revise`/`blocked` verdict with inspected-input and tool evidence.
Reconcile actual findings in this same record. Until then, fresh independent
plan review is **NOT RUN** and the dependent implementation handoff is not ready;
completed documentation repairs and the separate current proof queue remain valid.

### Fresh-agent review handoff — 5 November 2026

**Owner request:** David clarified that the review must use native subagents,
then requested an English handoff for another agent after those launches failed.
The contract candidate is main commit
`27c1df1336bd0b6febd237d77bc94b3774de1790`, tree
`40c65f10cf5b95cc92831cf7815af1ad9ac2480c`; its storage-refinement predecessor is
`8ac407a367bd2231076d12fe6e3db66e425f1281`. The earlier routing amendment is that
predecessor's diff from `8f1a3b8afab82ea736fb9ea44c99160c90a5075c`. Read the current
main and dirty state before proceeding; preserve subsequent unrelated work and
identify any changed review inputs. This handoff adds no product requirement or
implementation result and does not replace the current development §7 queue.

**Latest native dispatch receipt:** the following three workers were each
created with `fork_turns: "none"`. They received a repository path, immutable
candidate, bounded read-only assignment and evidence/output requirements, with
no inherited conversation or coordinator verdict.

| Worker | Assigned boundary | Actual result |
| --- | --- | --- |
| `review_storage_fresh` | Existing data/storage owners, Q0/Q-J, lifecycle, optional history, ST-W01 and PATH-15–20 | Not run: context-capacity error; no usable analysis. |
| `review_network_fresh` | Family/interface/path reuse, NTP/NTS and SOCKS, DNS and policy activation, PATH-01–20 | Not run: context-capacity error; no usable analysis. |
| `review_contracts_fresh` | Cross-document ownership, credential purpose/recipient, UI/broker consumers, evidence and stage consistency | Not run: context-capacity error; no usable analysis. |

Each worker returned the same raw failure:

```text
Agent errored: Codex ran out of room in the model's context window. Start a new thread or clear earlier history before retrying.
```

The available results do not establish which repository inputs, if any, were
read or which MCP calls were made. This is not three clean reviews. No finding,
comprehension paragraph or review-based repair is attributed to these workers.
The earlier failed/unconfirmed attempts above retain their original status.

**Next coordinator's bounded assignment:**

1. Read AGENTS.md, docs/README.md, the testing contract and repository
   `plan-independent-review` plus `review-and-handoff` guidance. Freeze the actual
   commit and relevant dirty bytes. Compare both amendment diffs above and read
   their surrounding canonical owners; a patch alone is insufficient.
2. With working native delegation, create three fresh workers for the boundaries
   in the table. Explicitly disable conversation inheritance. Supply the user's
   requirements, frozen sources, relevant acceptance IDs and read-only scope;
   withhold the coordinator's proposed answers. Have each begin with two original
   paragraphs describing the proposed change and the actual implemented system.
   Use independent temporary output paths and a single repository-editing owner.
   Do not repeat this unchanged failed launch configuration in the same session.
3. Read architecture §§7.3/7.4/12.1–12.2/17.1, the DATA inventory and storage
   tables, protected-storage and catalogue tasks, the affected feature/Windows/UI
   consumers and current engine source. Inspect relevant DNS owners when the
   path crosses their boundaries. Review PATH-01–20 and AC-21/22 as planned
   discriminators, not executed tests. Verify uncertain protocol/API claims
   through available applicable MCP and primary sources, recording actual
   calls and limitations; tool availability must be rediscovered.
4. Require scoped `ready`/`revise`/`blocked` results with inspected inputs, exact
   locations, severity, evidence, consequences, minimum correction and a useful
   valid neighbor for each material invalid case. Missing implementation of an
   explicitly planned feature is not an observed runtime defect. Preserve raw
   reviewer results and actual completion status in this existing record.
5. Reconcile findings against the integrated candidate. Retain stable existing
   STORE-N/NET-N/DOC-N identities when a finding is semantically the same; label
   new findings separately. Apply justified documentation corrections to their
   canonical owner and affected task/feature consumers. Verify affected findings
   after repair; do not rewrite independent reports to match author intent.
6. Check the resulting documentation diff, active links/anchors, canonical IDs
   and historical evidence preservation. Read docs/SHA256SUMS.txt only if present;
   no absent-manifest pass. Use code/native/formal checks only for an actual
   authorized changed guarantee. Preserve scoped Windows/storage/fuzz holds,
   UI-M0 deferrals, mandatory proof obligations and unrelated current work.

**Exit:** an actual completed, candidate-bound review; evidence-backed dispositions
for material findings; verified corrections where needed; and a named next stage
with remaining prerequisites. An unavailable necessary reviewer remains not run.
Documentation consistency does not execute PATH cases, qualify Windows/proxy/
storage behavior, complete product proofs or grant release approval. David's
existing authorization for ordinary documentation repairs and integration to
main remains; verify final remote bytes before claiming publication and report
CI results honestly. This handoff itself leaves independent review **NOT RUN**.

## Incoming coordinator review and corrections — 5 November 2026

**Scope:** completed incoming-coordinator source review of the frozen network and
storage contracts, followed by bounded documentation repairs. The authorized
fallback was used after all three requested native delegates failed. This section
supersedes the earlier *current* review-status summaries; the dated attempts,
author findings, original comprehension and receipts above remain historical.
It does not turn those attempts into completed reviews.

### Dispatch, identity and raw result

| Reviewer | Requested scope | Context mode | Actual result |
| --- | --- | --- | --- |
| `/root/storage_review` | Storage ownership and lifecycle | Explicit `fork_turns: "none"` | **NOT RUN**; no usable report or inspected-input trace |
| `/root/network_review` | Network and protocol behavior | Explicit `fork_turns: "none"` | **NOT RUN**; no usable report or inspected-input trace |
| `/root/contracts_review` | Cross-document contracts and consumers | Explicit `fork_turns: "none"` | **NOT RUN**; no usable report or inspected-input trace |
| Incoming coordinator | All three source-review scopes | Handoff-informed, not blind; distinct from the original author | **Completed** baseline review; post-repair verification is the same coordinator's self-verification |

Each native dispatch returned the following failure, with no original
comprehension or findings. The unchanged failed setup was not relaunched:

```text
Agent errored: Codex ran out of room in the model's context window. Start a new thread or clear earlier history before retrying.

This agent's turn failed. If you still need this agent, use the available collaboration tools to give it another task.
```

The reviewed contract is commit `27c1df1336bd0b6febd237d77bc94b3774de1790`, tree
`40c65f10cf5b95cc92831cf7815af1ad9ac2480c`. GitHub main and the clean separate
checkout were `68a0a2b5d25cc589d62403344d4af32e89269a84`, tree
`82d21c4fcf4f1e92019da2abfcadf2a20d761686`. GitHub comparison confirmed only the
80-line handoff was added after the contract candidate. No staged, unstaged or
untracked bytes were inherited. Remote main was rechecked before repairs.

The raw incoming review below was frozen before repairs. Its exact UTF-8 bytes,
including the two original opening paragraphs, have SHA-256
`ff62d2fe49529e838d3cc05d37abc72c27e1bd6f4941bc1397f4648720019cde`.
The initial **revise** verdict is deliberately retained.

<details>
<summary>Raw incoming-coordinator review, preserved verbatim</summary>

```text
The proposal adds local address-family admission, experience-based ordering of permitted paths, bounded recovery, and an authorized interface/family/proxy profile. It places live preferences in the existing scheduler and carries outcome/context references into optional quality history. It also distinguishes a saved policy from effective acquisition and conditionally extends the existing secret owner for proxy credentials.

The implemented engine is a bounded observe-only experiment with two explicitly supplied loopback peers, a private engine-local Tokio runtime, fixed admission and completion guards, and no clock authority. Pure NTS framing/negotiation exists separately; it does not provide production endpoint resolution or proxy acquisition. Learned routing, SOCKS transport, Q/H persistence and production proxy-credential storage remain proposals.

## Incoming coordinator review — raw result, 5 November 2026

**Stage:** source/contract review before dependent network/storage implementation; baseline verdict **revise**. This report was frozen before the coordinator's repairs. Findings describe design/documentation defects, not observed runtime vulnerabilities.

**Candidate:** contract commit `27c1df1336bd0b6febd237d77bc94b3774de1790`, tree `40c65f10cf5b95cc92831cf7815af1ad9ac2480c`. Review checkout `/workspace/scratch/cf0c4376f5a7/NevroTick` was a separate copy of the surviving checkout, at `68a0a2b5d25cc589d62403344d4af32e89269a84`, tree `82d21c4fcf4f1e92019da2abfcadf2a20d761686`. Staged, unstaged and untracked work were empty. GitHub read-back and comparison established that the later commit adds only the 80-line handoff to the engine task; the product contracts are identical. Both amendment diffs and the relevant surrounding owners were inspected. Remote main was still `68a0a2b5d25cc589d62403344d4af32e89269a84` at the pre-repair check.

**Method/context:** incoming coordinator, not the proposal's author, using this thread's handoff and direct source inspection. This is not a blind review: the handoff includes author history. Findings were formed from the contracts and primary sources, then reconciled with the existing IDs. Three native read-only review workers (`storage_review`, `network_review`, `contracts_review`) were actually dispatched with `fork_turns: "none"` and bounded source packets. All failed with the context-capacity error before returning comprehension, findings or inspected-input evidence. Their status is **NOT RUN**, not zero findings. The allowed incoming-coordinator fallback is the only completed review method. Post-repair checks by the same coordinator will be self-verification, not a second independent review.

### Inspected inputs and tools

- Governance: root `AGENTS.md`, `docs/README.md`, planning `plan-independent-review`, `review-and-handoff`, `plan-code-change`, `plan-architecture`, `plan-threat-model`, testing `CONTRACT` and `qa-subagent-dispatch`, implementation `CONTRACT`, `evolve-data-contracts` and the data-contract reference. Environment/tooling guidance was also read. No additional nested AGENTS instructions were found in the applicable paths.
- All 12 files in the storage amendment were inspected through their actual diffs and relevant owners: `docs/README.md`; `docs/plans/development-plan.md`; `docs/plans/server-catalogue-and-statistics-draft.md`; `docs/plans/server-catalogue-storage-tables.md`; `docs/plans/windows-protected-storage.md`; `docs/windows-time-architecture.md`; `docs/windows-time-features.md`; `tasks/engine-transport-pilot.md`; `tasks/server-catalogue-and-statistics.md`; `tasks/ui-m0.md`; `tasks/windows-protected-storage.md`; `tasks/windows11-integration.md`.
- Architecture focus: §§7.3/7.4, 12.1–12.2, 16.5, 17.1, runtime/verifier boundaries in §18, and THR-21. Storage focus: DATA-06/08/09/10/16/17/19–22, complete allocation mappings and Q0/Q-J, shared capacity, reset and protected-storage purpose rules. Consumers: engine contract card and PATH-01–20, Windows §§4.27–4.28, F-06/§3.3.1/AC-07/21/22, UI-M0 deferrals, development §§7/8.4/8.5, and OD-10. The existing DNS design/privacy owners and `tasks/s0.1-protocol-comparison.md` were consulted where resolver and NTS endpoint semantics cross owners.
- Implementation inspection: workspace and engine manifests, lockfile, `crates/nevrotick-engine/src/lib.rs` and its observe binary, plus the pure NTS negotiation boundary. Tokio is pinned to `1.54.1` in manifest and lockfile. Source inspection is not execution of the pilot or its tests.
- GitHub MCP: repository metadata, main/commit/tree read-back, comparison of contract and handoff commits, and main workflow results. These establish identity and publication/CI observations only. The original partial checkout could not be cloned without an unavailable promisor object; a filesystem copy preserved it unchanged. Local Git/rg/targeted file reads supplied current byte evidence.
- Microsoft Learn MCP: searches and full relevant pages for `IPPROTO_IP`/`IPPROTO_IPV6` socket options and `CancelIoEx`. Outbound interface selection does not by itself establish receive provenance; cancellation is a request, not completed custody. Existing Windows/engine qualification contracts already preserve both boundaries.
- Primary web reads: RFC 8915 §§4.1.7–4.2, RFC 1928 request/reply and UDP-association rules, RFC 8305 address ordering/history, RFC 1929 authentication, RFC 9110 CONNECT, RFC 9298 UDP proxying, and SQLite `journal_size_limit`. They support specific protocol/capacity checks, not Windows or provider qualification.
- Context7: resolved Tokio, selected `/tokio-rs/tokio`, and queried blocking-job cancellation/runtime shutdown. Results were unversioned upstream examples; an indexed alternative was 1.49.0 and the direct docs.rs 1.54.1 page was unavailable. No exact-1.54.1 qualification is attributed to that lookup, and no correction depends on transferring those examples to the pinned runtime.
- Serena, SocratiCode, Probe and Context-mode were not exposed in the available tool registry; their absence is not evidence that implementation is absent. No tools were installed, external reviewers requested, provider/native experiments run, or service/clock/security settings changed.

### Material findings

#### NET-N06 — Medium — proxy target knowledge does not close NTS endpoint admission

**Boundary/location:** requirement to preserve useful strict proxy acquisition and NTS/DNS destination authority; architecture §7.3 proxy-resolution paragraphs (baseline around lines 918–936), §7.4 final destination admission and THR-21; DATA-08 actual-peer row; engine PATH-03/10. The existing protocol task's pending-negotiation paragraph already assigns an absent server record to the saved actual KE peer IP.

**Evidence:** RFC 8915 §4.1.7 binds the absent-server default to the KE server IP. RFC 1928's CONNECT `BND.ADDR` is a proxy-side bind address, not that remote peer. The new profile permits an unknown proxy-resolved target while retaining the old actual-peer/default and final resolved-address admission obligations. Forwarded hostname identity alone supplies neither missing fact.

**Consequence:** an adapter could substitute the outer proxy address, misread `BND.ADDR`, or re-resolve the name to a different server; alternatively it could claim network-scope admission without an enforcing owner before sending. This is a contract-composition gap, not a demonstrated defect in implemented SOCKS code.

**Smallest correction:** specify the endpoint-knowledge boundary at §§7.3/7.4, retain typed provenance/unknown state in DATA-08, and extend existing PATH-03/10. A required pre-send restriction needs an admitted numeric target or a separately qualified enforcing resolution path; receiving an address later is insufficient. Where the NTS default cannot be established, classify that route as unsupported and recover only within policy. Preserve the original TLS service identity and explicit authenticated delegation. No proxy trust waiver, resolver choice or new store is adopted.

**Control pair:** admitted numeric CONNECT target with retained TLS identity and a valid default NTP endpoint remains useful; proxy-DNS with a valid explicit delegation can remain useful only when its target restrictions are enforceable. Unknown KE peer with no server record, fabricated BND/outer-peer substitution, and a name resolving to a forbidden destination must not authorize a send or hidden local-DNS/Direct fallback. Explicitly permitted LAN destinations remain admitted.

#### NET-N03 — Medium — residual unconditional rollback promise in legacy consumers

**Boundary/location:** saved/applying/effective policy; architecture §12.2 baseline line 1895; F-06 line 146, journey D line 423 and AC-07 line 462; contrast §7.3 cutover and PATH-19. This reopens the existing semantic finding rather than inventing a duplicate ID.

**Evidence:** §12.2 says failed validation or persistence leaves the prior policy active; F-06/AC-07 include an unqualified storage-failure guarantee. The new cutover contract instead recognizes a committed revision, blocked activation and indeterminate outcome without silent old-route resumption. No evidence of an atomic storage-plus-native-I/O transaction reconciles the two.

**Consequence:** the UI/helper/engine could interpret a lost acknowledgement, uncertain commit or postcommit failure as safe rollback and continue the old Direct profile after a proxy-only revision was saved. The claim is a design ambiguity; no such production Apply implementation was found.

**Smallest correction:** preserve the old effective policy only for a proven precommit rejection. Route committed or uncertain outcomes through the existing policy owner and operation/revision reconciliation; report saved and effective state separately and retain the cutover's I/O restrictions. Reconcile F-06, journey D, AC-07, DATA-06 and PATH-19 without creating a new transaction database or promising impossible wire recall.

**Control pair:** cancelled UAC, invalid/stale drafts and a known not-committed write refusal leave the prior policy active; a successful qualified transition becomes effective. Lost/indeterminate commit results and postcommit activation failure cannot be reported as rollback, retried blindly or used to resume the old route.

#### DOC-N05 — Low — remaining acceptance-catalogue navigation drift

**Boundary/location:** `docs/README.md` reading-guide row, baseline line 449.

**Evidence/consequence:** the maintained feature catalogue defines AC-01–AC-22, but this second guide entry still promises twenty scenarios. The earlier ID addresses the same navigation defect; the first corrected entry did not cover this consumer.

**Smallest correction/control pair:** use the actual AC-01–AC-22 identifier range. Resolve all 22 defined entries and their links; reject the obsolete twenty-scenario description. This is documentation validation, not execution of the scenarios.

### Storage and neighboring-case assessment

No additional material storage defect was substantiated within this inspection. The existing mapping supplies one producer-to-consumer chain: policy DATA-06 admits work; DATA-08/09 identify its live context; DATA-10 produces attempt/stage facts; DATA-17 consumes them in bounded RAM; the optional bounded handoff lets Q's existing owner retain descriptive/quality history; bounded projections serve diagnostics/UI. Q absence or failure does not gate acquisition. Actual retries are distinct observations, while duplicate processing is not another observation. Context eviction/reset cannot clear independent retry, authentication, DNS-privacy or broker guards.

Architecture §12.1.3 and DATA-20 already count live/retired contexts, readers, queues, native custody and maintenance in aggregate bounds. Q0/Q-J remains unselected; SQLite's journal limit is not a peak total-file-family quota. Existing proxy credentials require their own ST-W01 purpose/recipient/revision/lifecycle, with no implicit admission by the NTS/DNS payload matrix. PATH-15–20 and ST-T mappings cover useful RAM-only acquisition, stale restoration, pressure/reset, valid credentials beside wrong-purpose/recipient failures, and late work. These are adequate documentary boundaries for the reviewed stage, not evidence that implementations pass them.

Local family admission is not remote reachability; stage/destination-specific failures cannot become a global family prohibition. Existing expiry, context invalidation, bounded preferred-path recovery and hysteresis preserve a healthy alternate without treating RTT as time authority. No current numerical quota, reprobe interval, proxy method or database format needs to be invented to repair the three findings. Native interface/receive provenance, real cancellation completion, verifier traffic and exact credential channels remain explicitly pending qualifications rather than silently accepted residual risks.

**Verdict/limits:** **revise** for the documentation/contract handoff on the identified baseline. After corrections, the same coordinator must verify the affected consumer chain on the revised bytes. The three requested delegated reviews did not occur. The review does not establish implemented proxy support, Windows qualification, mathematical proof completion or release readiness. The current EraWindow/Garde proof continuation, OD-10, UI-M0 D1–D3, scoped Windows/Sandbox restrictions, storage pause and separate fuzz hold remain unchanged. Main-68 CI was observed as Rust product checks success and Repository security failure; no failure cause is inferred.

**One permitted next action:** apply the three bounded documentation corrections at existing owners, preserve this raw result and all earlier receipts, then perform source-bound documentation checks before normal authorized integration.
```

</details>

### Dispositions and consumer verification

The finding IDs were reconciled by meaning, not reviewer count. NET-N03 and
DOC-N05 reopen residual parts of the author's existing issues; NET-N06 is new.
No unavailable experiment or planned feature was relabelled accepted residual risk.

| Finding | Disposition on corrected documents | Canonical correction and consumers |
| --- | --- | --- |
| NET-N06, Medium | **Verified corrected at the documentation stage** | Architecture §§7.3/7.4 bind endpoint knowledge and pre-send scope admission. DATA-08 preserves separate identities/provenance/unknown state. The engine card and PATH-03/10 carry the failing and useful routes; feature §3.3.1 carries truthful unavailable-state reporting. The existing pure-negotiation task already requires the actual KE peer and remains unchanged. |
| NET-N03, Medium | **Verified corrected at the documentation stage** | Architecture §12.2 distinguishes precommit rejection, committed revision and uncertain outcome. §§6.2/6.4 and the §20.4 errors/recovery row had the same unconditional retention language; these additional consumers were found during reconciliation and now point to §12.2. DATA-06, F-06, journey D, AC-07 and PATH-19 carry the same distinction. |
| DOC-N05, Low | **Verified corrected at the documentation stage** | The remaining README reading-guide entry uses AC-01–AC-22. |
| Prior STORE-N01/02/04 | No additional material defect substantiated within the inspected scope | Existing owners, conditional proxy-secret purpose, aggregate capacity and reset/consumer rules remain adequate documentary constraints. No Q format, quota, storage qualification or runtime result is selected or inferred. |

The following are **source walkthroughs of required state transitions**, not
executed PATH, native, provider or model tests:

| Boundary | Invalid/uncertain neighbor | Useful neighbor and enforcing owner |
| --- | --- | --- |
| Endpoint knowledge | Unknown KE peer with omitted server record; proxy bind substituted for target; forbidden destination hidden by domain forwarding | A known admitted numeric target retains TLS identity and the existing NTS endpoint contract. Explicit delegation remains usable when destination restrictions can be enforced before I/O. DATA-08 supplies facts; §7.4 admission owns permission. |
| Policy outcome | Lost commit acknowledgement or postcommit activation failure cannot prove rollback or authorize a duplicate Apply/old-route resumption | Known precommit refusal preserves the prior policy; a qualified successful transition becomes effective. DATA-06 and §12.2 own reconciliation, §7.3 owns I/O cutover, UI reports their outcome. |
| Live acquisition and optional history | Q disabled/full/corrupt, stale restored context or unknown age cannot create freshness, votes or peer failures | DATA-10 feeds current DATA-17 RAM independently of the bounded optional Q handoff; useful acquisition and current successful-path reuse remain possible. |
| Restart, churn and reset | Late Q writers/readers, retired contexts and eviction cannot revive current state or clear independent guards | Fresh DATA-08/09 contexts and current outcomes can become useful under shared DATA-20 limits; DATA-21 authority and DNS privacy barriers remain separate. |
| Proxy credentials | Wrong purpose/revision/recipient or missing secret cannot select no-authentication, Direct or an NTS/DNS envelope | A future qualified credential/channel under the existing ST-W01 purpose can work. Its implementation and protection experiments remain pending/held. |
| Interface and completion | Outgoing socket selection or a cancellation request alone does not prove receive provenance or completed native work | Existing Windows §4.28 and architecture §17.1 retain actual route/receive/completion qualification beside permitted healthy routes. |

### Primary inputs and tool limits

Actual source inputs and MCP uses are listed in the raw report. The principal
external references checked were [RFC 8915](https://www.rfc-editor.org/rfc/rfc8915.html#section-4.1.7),
[RFC 1928](https://www.rfc-editor.org/rfc/rfc1928.html),
[RFC 8305](https://www.rfc-editor.org/rfc/rfc8305.html#section-4),
[RFC 1929](https://www.rfc-editor.org/rfc/rfc1929.html),
[RFC 9110 CONNECT](https://www.rfc-editor.org/rfc/rfc9110.html#section-9.3.6),
[RFC 9298](https://www.rfc-editor.org/rfc/rfc9298.html),
[SQLite PRAGMA](https://www.sqlite.org/pragma.html#pragma_journal_size_limit), and
the full Microsoft Learn pages for
[IPv4 socket options](https://learn.microsoft.com/en-us/windows/win32/winsock/ipproto-ip-socket-options),
[IPv6 socket options](https://learn.microsoft.com/en-us/windows/win32/winsock/ipproto-ipv6-socket-options)
and [CancelIoEx](https://learn.microsoft.com/en-us/windows/win32/api/ioapiset/nf-ioapiset-cancelioex).
The additional resolver owner was
[system DNS design §9](../docs/plans/system-dns-cache-design.md#9-privacy-and-user-facing-controls)
and its current native-query/bootstrap contracts; resolver-retained state was not
repurposed as time-path history. Context7's unversioned Tokio examples do not
qualify the pinned 1.54.1 implementation.

GitHub MCP also read baseline workflow/job states and the active repository
rulesets. The branch-protection endpoint returned 403 and the branch-rules route
was unsupported by that connector; no configuration or alternate credential was
used to bypass those read limits. The rulesets and repository were otherwise
readable. Publication must preserve current controls and use the normal authorized
integration path; a connected tool or successful merge is not CI evidence.

### Revised byte identity and checks

These Git blob identities bind the corrected normative inputs before this
evidence-only append. For the engine task, the listed blob/hash identifies its
entire prefix immediately before this heading, including the corrected card and
PATH rows. The containing Git commit/tree binds this complete record and its raw
report without a self-referential hash claim.

| Input | Git blob identity |
| --- | --- |
| `docs/README.md` | `03929db3836693ee3ed84da84509e0a8b7057197` |
| `docs/plans/development-plan.md` | `7ecd0b89e89fc9b9ce3601e68851133c6d6c0c60` |
| `docs/plans/server-catalogue-and-statistics-draft.md` | `789b7ea9df9482971df8d99beb5897ac56a44139` |
| `docs/windows-time-architecture.md` | `c458bcf7f33195fe0b1a5855925252f295e7a064` |
| `docs/windows-time-features.md` | `ab4963338ec15df141e3686548336eabc5679d54` |
| `tasks/engine-transport-pilot.md (prefix)` | `96aa83afcf5e4928c5303aa1e3a86d1119fdcf75` |

Engine prefix: 168,090 bytes, SHA-256 `8d7ea5925db5aaca640bc51253cfc8c1d272fd6423c1c2d96dd97903a25852b0`.

**Executed documentation checks on these revised bytes:** `git diff --check`
passed. Active relative-link/definition validation covered **779 occurrences**,
**166 distinct repository targets** and heading references into **34 Markdown
targets**, excluding fenced reports/code and inline-code examples. Git-tree paths
were used for sparse exclusions; no replacement document was created. The check
confirmed **131 unique contiguous source IDs**, exactly **PATH-01–20** and
**AC-01–22**, with no missing architecture source reference.

All **nine pre-existing fenced blocks in the six edited files** remain
byte-identical and in order; the new failure receipt and raw review add two blocks.
The raw report matches its recorded hash, and the engine prefix plus all five
other normative blobs match the table. Development §7 is byte-identical to main-68.
The diff changes Markdown only; source, manifests, UI assets, owner decisions,
protected-storage and execution-hold owners are untouched. `docs/SHA256SUMS.txt`
is absent in the Git tree. Targeted rereads checked the affected positive and
negative paths above and all changed hunks. These checks executed no product,
native, formal-proof, provider, storage or fuzz campaign. The temporary validator
was adjusted for the repository's source-ID and Markdown-reference syntax before
its successful pass; its early parser mismatches were not product findings.

### Stage verdict and continuation

**READY at the corrected source/documentation review stage**, using the disclosed
incoming-coordinator fallback and subsequent coordinator self-verification.
The baseline report's `revise` verdict is resolved only for the three documented
findings on the revised inputs above. The three requested delegated reviews remain
**NOT RUN**; no independent reviewer verified the coordinator's final edits.
Required CI is not waived by this stage verdict, and dependent product
implementation is not qualified by these documentation checks.

The next overall development stage remains
[development §7's public EraWindow/Garde proof continuation](../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent).
For the later network work, the next permitted design step is the existing finite
profile/transition freeze in this task; dependent implementation still needs its
named proof, native, proxy/resolver/verifier and secret qualification. Q0/Q-J
remains the existing storage decision, not a newly chosen database. UI-M0 D1–D3,
OD-10 and the exact Windows/Sandbox, storage-experiment and separate fuzz holds
remain unchanged. This review supplies no production proxy support, Windows
qualification, complete formal verification or release approval.

**CI observation on the baseline only:** main-68's
[Rust product checks](https://github.com/DavidOsipov/NevroTick/actions/runs/42007199512)
completed successfully; [Repository security](https://github.com/DavidOsipov/NevroTick/actions/runs/42007199304)
failed, including its Security gate and full-history/working-tree secret scan.
These statuses were read from GitHub. A subsequent read of secret-scan job
`127176845247` showed its healthy/synthetic/restored/prose controls passed and its
full retained-history scan returned 1. Matched findings are deliberately not
logged by the existing checker; their cause and disposition were not established
by this bounded read. No scanner, allowlist or required check was changed and
no result is transferred to the revised candidate. Final publication/CI requires
read-back of the actual integrated commit and its checks; the PR/final delivery
receipt supplies that exact identity. No all-green claim follows from this record.

## SOCKS5 feasibility and deferral — 5 November 2026

**Request and candidate:** David asked for a concrete SOCKS5-only profile and
checks, with permission to defer the feature if implementation/support cost is
disproportionate. Baseline main is `b9fb17d32c64418bdc524e966e1cab4389575dd6`.
This source/documentation amendment selects deferral under
[OD-11](../docs/nevrotick-owner-decisions.md#15-od-11--socks5-scope-and-deferral).
[SOCKS-0](../ROADMAP.md#socks-0--optional-socks5-time-acquisition) owns future timing
and SOCKS-C01; this existing engine task owns the bounded child, without a new
proxy task, cache or database. First Windows delivery remains Direct. Current
P2/P5 direct-family/interface, path-history and shared Apply/storage work stays
in place; development §7's public-constructor proof continuation is unchanged.

### Concrete future child and evidence owners

Use the canonical [limited profile](../docs/windows-time-architecture.md#limited-socks5-profile--deferred):
manual `127.0.0.1`/`::1` plus port; method 0x00 only; explicit local DNS; numeric
admitted targets; Automatic interface; concrete same-IP/nonzero-port UDP relay.
The example port 10808 is not a default or tested service. Domain/wildcard/other
relay addresses, remote proxies, credentials and proxy DNS are outside this first
subset. Keep an observation-only diagnostic over the admitted effective profile;
it must not create another unbounded probe or clock-authority path.

The engine/transport owner should freeze one active UDP-association owner for the
single profile, with its own bound UDP socket and monitored TCP control stream.
Reuse the existing policy/network/association generations and total resource
budget; do not introduce a second policy owner. Each bounded KE stream retains
its original TLS identity and numeric target. Advertise the actual bound client
UDP endpoint, admit the returned relay before connecting/sending, then demultiplex
numeric inner sources through existing source-specific exchange owners. Existing
NTS cookies, crypto, replay, endpoint and timeout rules remain authoritative.

Before implementation, freeze finite parser/buffer/connection/queue limits and
timeout/backoff/recovery transitions through the current engine owner. A failed
control/relay retires only the affected work without refilling source/security
budgets. Retain actual native completion and truthful saved/effective activation.
The first subset creates no secret purpose; authentication is a later ST-W01
extension. DATA-06 stores authorized policy; DATA-08/09/10/17 retain live state;
optional Q is not needed for either cold acquisition or live path reuse.

| Bounded check group | Independently expected observation | Existing owner / acceptance |
| --- | --- | --- |
| Profile and no-authentication wire contract | Valid numeric loopback/port and method 0x00 succeed. Zero/out-of-range port, remote/domain endpoint, incompatible hard family or physical-adapter pin, bad versions/reserved fields, method 0x02/0xff or unoffered method fail before unauthorized effects. Imported unsupported proxy policy never becomes Direct. | Domain/IPC/protocol; CHECK-2/3/6/25, PATH-10/12/14/19, AC-22 |
| Full NTP/NTS path | A bounded independent fixture completes TCP CONNECT/KE and real UDP ASSOCIATE plus correlated valid time packets; a TCP-only proxy or successful KE with blocked UDP cannot produce a working-path result. Closing KE leaves the separate UDP control usable. Ordinary NTP remains labelled unauthenticated. | Engine/protocol; PATH-03/07/08, CHECK-6/8/9 |
| Relay and packet boundary | Concrete same-IP relay works. Domain/wildcard/wrong-family/wrong-IP/zero-port replies fail before relay I/O. Wrong outer sender, unexpected inner numeric source, fragment/reserved errors, truncation, oversized data and stale association cannot publish or cause an out-of-bounds copy. A valid neighboring packet still works. | Codec/transport/security; PATH-09/11/17, CHECK-6/9/21/25/27 |
| NTS endpoint and resolver identity | Local DNS may return several admitted numeric candidates; the original TLS name survives CONNECT. An IPv4 loopback proxy can reach an admitted IPv6 target even with no usable local IPv6 path; the reverse case is distinct. Resolver filtering must not erase those remote candidates. Omitted Server Negotiation uses the chosen KE IP, never proxy BND.ADDR or a new name lookup. Valid explicit delegation works; forbidden scope, TLS identity or authentication fails with no plain/direct recovery. Local DNS is disclosed and cannot silently become domain forwarding. | Resolver/protocol/trust; PATH-03/08/09/10, THR-21, OPEN-8, CHECK-6/8/9 |
| Actual egress | Observe time/helper packets only at the qualified relay and expected local DNS under its explicit policy. Audit actual verifier retrieval; required AIA/OCSP/CRL must follow the admitted route or the profile is unavailable. Exercise resolver/verifier failures with a healthy complete path. No unapproved direct packet or disabled verifier check repairs a failed proxy. | Resolver/verifier/platform; PATH-08/10/19, CHECK-2/6/9/17/27 |
| Loss and lifecycle | Control EOF/reset, silent UDP loss, proxy starts late/restarts, service starts before login, logout and resume produce bounded observable recovery. Retired packets cannot complete a new exchange. Verify under the actual Windows service token, not a browser/user-shell session. | Engine/Windows; PATH-04/07/12/17, CHECK-6/9/12/14/21/27 |
| Policy cutover and retained work | Apply with queued sends, DNS/verifier jobs and old relay/control I/O: distinguish precommit rejection from unknown/postcommit outcomes. Saved state is not effective state; old results cannot publish, and retirement retains real native charges. Unsupported downgraded/imported profiles cannot resume Direct. | Policy/native/IPC; PATH-19, AC-07/22, CHECK-2/3/6/14/25/27 |
| Timing and resources | Compare controlled direct and encapsulated packet timestamps against independent fixture truth; include delay/asymmetry and route changes. No false accuracy, extra source vote or lower quorum. Measure sockets, tasks, buffers and wakeups across loss/retry/cutover; warm reuse works with Q absent/full/corrupt. | Numerical/engine/QA; PATH-13/15/16/17/20, CHECK-7/8/9/14/21/27 |

Portable policy/codec cases come first, then a controlled local server fixture,
then actual Windows service evidence with one pinned maintained proxy and the
qualified NTS/verifier graph. Those are staged future checks, not permission to
run host experiments now. Existing native/Sandbox, storage and fuzz holds retain
their exact scope. Only enable product controls after the supported envelope and
SOCKS-C01 evidence exist. Remote proxy/auth/proxy-DNS extensions receive their
additional cases then; they do not block Direct or the first no-authentication
subset. No implementation schedule or quantified maintenance estimate is claimed.

### Source feasibility assessment

The protocol split/control lifetime and endpoint constraints are grounded in
[RFC 1928 §§4–7](https://www.rfc-editor.org/rfc/rfc1928.html#section-4) and
[RFC 8915 §4.1.7](https://www.rfc-editor.org/rfc/rfc8915.html#section-4.1.7), already
retained as architecture S129/S20. Local DNS is a deliberate limitation: it avoids
inventing a SOCKS domain-family selector or actual-KE-peer metadata that the
protocol does not provide. It does not promise proxy-DNS privacy or universal
compatibility. A no-authentication subset is not full RFC method support.

Two public Rust candidates were read through GitHub MCP at pinned commits:

| Candidate / inspected source | Finding and consequence for this bounded profile |
| --- | --- |
| `tokio-socks` 0.5.3, `3f2c090cedb4546434a237f71103cf02c4da27c0`; [README](https://github.com/sticnarf/tokio-socks/blob/3f2c090cedb4546434a237f71103cf02c4da27c0/README.md) and [manifest](https://github.com/sticnarf/tokio-socks/blob/3f2c090cedb4546434a237f71103cf02c4da27c0/Cargo.toml) | CONNECT is checked, ASSOCIATE is unchecked in the actual Markdown. It is not a complete TCP-plus-UDP adapter for time acquisition. A rendered list that loses checkbox state is insufficient evidence of UDP support. |
| `fast-socks5` 1.0.0, `bae2fc05f993d3a247b5435ba7da221f3fd80a3c`; [client source](https://github.com/dizda/fast-socks5/blob/bae2fc05f993d3a247b5435ba7da221f3fd80a3c/src/client.rs) and [manifest](https://github.com/dizda/fast-socks5/blob/bae2fc05f993d3a247b5435ba7da221f3fd80a3c/Cargo.toml) | Has UDP ASSOCIATE and a supplied-socket path. Inspected `bind_internal` advertises `[::]:0`, resolves/connects the returned relay internally before the caller can admit it. Inspected `recv_from` copies `data.len()` into the caller slice without a local capacity guard. The UDP owner retains a private control stream, but the inspected receive path does not monitor its EOF. These require adaptation/qualification for NevroTick's endpoint, bounded-buffer and lifecycle contracts. |

These are source observations, not reproduced runtime failures, a full upstream
audit or product vulnerabilities; neither candidate is selected or installed.
The point is that a small library API does not remove the required integration
and review work. [3proxy's SOCKS documentation](https://3proxy.org/doc/man8/socks.8.html)
describes UDP support and is a possible later interoperability input, not a tested
or selected NevroTick proxy.

Microsoft Learn MCP was also used to read
[WinHttpGetIEProxyConfigForCurrentUser](https://learn.microsoft.com/en-us/windows/win32/api/winhttp/nf-winhttp-winhttpgetieproxyconfigforcurrentuser).
Its remarks distinguish interactive-user settings from service-account settings
and restrict service use without impersonation. This supports keeping manual
service policy; it does not prove loopback connectivity, select WinHTTP or qualify
any proxy/service combination.

### Review and continuation status

Coordinator source research and self-review checked the TCP-only false-positive,
control-stream lifetime, local-DNS disclosure, exact NTS default target, relay
admission before I/O, buffer bounds, actual service context, no-authentication
scope, existing data ownership and the current/future task split. Review identified
that a loopback proxy cannot honor a selected physical onward interface and that
an endpoint literal cannot implement a preference by inventing an alternative;
the limited profile now rejects/labels those combinations explicitly. Strict
relay-address limits remain an interoperability limitation to qualify later.

One fresh bounded read-only subagent dispatch, `socks5_scope_review`, used
`fork_turns: none`. It failed with a context-capacity error before returning any
usable analysis. No independent review pass or inspected inputs are attributed
to it, and the same failed configuration was not retried. Independent review
remains required before the later implementation handoff is called ready.
The earlier incoming-coordinator report applies to its recorded older inputs;
this amendment does not silently extend its verdict to the new profile.

The final documentation receipt records checks on this amendment separately.
No product/proxy/native, new mathematical proof or storage execution is claimed.
Next overall task remains development §7; next proxy action occurs only at
SOCKS-0's future entry, starting with the finite profile/transition freeze and
independent review above.

**Documentation receipt:** `git diff --check` and focused validation passed on
this 14-Markdown-file amendment: 1,090 active inline relative-link occurrences,
110 distinct target paths, 131 contiguous architecture source IDs, PATH-01–20
and AC-01–22. All 22 pre-existing fenced blocks across the changed files remain
byte-identical; development §7 and OD-10 are unchanged. Source, manifests, lockfile,
CI configuration and UI assets are unchanged. The containing commit/tree binds
the integrated documentation; these checks establish no product, proxy, native,
formal-proof or independent-review pass. Publication/CI status must be read back
for that actual commit, separately from the historical observations above.

## Ordinary full NTS composition — S2.2 preparation, 5 November 2026

**Disposition:** source preparation against [upstream `5e98c79`](https://github.com/DavidOsipov/NevroTick/commit/5e98c79901c90de77db6c6047cb483ec10b8cbd5).
Implementation, qualification and independent integration review remain open at
this handoff. [Development §7](../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent)
continues to own the current proof/Slint sequence; this card grants no execution
permission. Engine/protocol/crypto/platform owners reuse REQ-8/BND-2,
CHECK-6/8/9/17/21/25/26/27 and OPEN-1/5/6/8. Numerical consumers additionally
retain CHECK-7 and OD-10.

**Full exit:** fresh, ordinarily verified normal-date KE publishes one complete
current association; its protected UDP exchange completes once; admitted evidence
reaches the actual history/estimator consumer. Observe a useful eligible exchange
and an authentic but numerically refused exchange separately. Transport-only or
authentication-only success is an intermediate receipt, not full S2.2 or P2.
No clock writes, source-default selection or UI integration is required.

The [engine manifest](../crates/nevrotick-engine/Cargo.toml) still contains only
`getrandom`, `ntp-wire` and Tokio; real UDP ends in diagnostic `TransportFacts`.
[Wire kernels](../crates/ntp-wire/src/lib.rs) provide inert KE/framing/exporter
metadata, not a protected-NTP writer, authenticated association or Core bridge.
Reuse the [S2.1 measurement and consumer card](#s21-preparation--real-transport-to-an-admitted-time-measurement--5-november-2026),
[path admission](#address-family-admission-and-bounded-acquisition-progress--5-november-2026),
[Direct routing](#network-path-history-and-user-routing--5-november-2026) and
[protocol checklist](s0.1-protocol-comparison.md#kod-reuse-and-rfc-8915-alignment-follow-up).
Their contracts remain with those owners.

### Ordered implementation handoff

1. **Freeze dependent contracts.** Select and admit one exact TLS/provider/AEAD
   graph and execution arrangement, normal-validation lab profile, packet semantics,
   stage and total acquisition deadlines, and finite allocation/work budgets.
   Include submitted/queued jobs, per-endpoint admission, pending requests,
   active/staged/reserved secret bytes and fair progress. Record finite ephemeral
   key/association validity and invalidation, cookie issuance/reuse and the still
   **pending local zero-replacement-cookie acceptance/completion/retry decision**.
   Existing 128 KiB, 20-second and two-job figures remain qualification candidates;
   the existing ≥65,536-byte acceptance target retains RFC SHOULD strength.
   No runtime values are selected here.
2. **Engine acquires fresh KE.** Capture the configured verification identity and
   admitted actual connected KE peer separately, with source/configuration,
   network/path and trust revisions. Apply ordinary certificate validation, the
   admitted TLS-1.3-only profile and exact `ntske/1`. Encode with
   `encode_nts_ke_ntpv4_request`, retain the actual offer and flush the complete
   request. Feed bounded TLS plaintext into `NtsKeAccumulator`, finalize once and
   call `negotiate_nts_ke_ntpv4` against that offer. Preserve Error/Warning outcomes
   and useful unknown-noncritical neighbors. Truncation before EOM or a failed
   required check publishes no association. Freeze post-EOM error/EOF and shutdown
   transitions: EOM, `finish()`, transport EOF and TLS closure are distinct.
   Sending `close_notify` does not imply an unconditional RFC duty to await the
   peer alert; use the [protocol clarification](s0.1-protocol-comparison.md#ordinary-s22-composition-clarifications--5-november-2026).
3. **Association owner builds privately, then publishes together.** Bind the exact
   offer/negotiation to its TLS job. Use `NtsKeNtpv4Negotiation::exporter_context`,
   `NtsKeExporterDirection` and `NtsKeExporterContext::label()`/`context_bytes()`
   at that connection’s regular exporter: protocol-0/algorithm-15 contexts
   `00 00 00 0f 00` and `00 00 00 0f 01`, separate 32-byte C2S/S2C keys.
   Metadata alone establishes neither key length nor connection authority.
   Bind keys, bounded initial cookies, AEAD, TLS identity/trust basis, actual KE
   peer, admitted UDP endpoint, lifetime and generations into one candidate.
   Validate `server_hint()` as an allowed address/FQDN before resolution; printable
   ASCII is not endpoint authority. Admit explicit delegation/`port_hint()` under
   current Direct policy; absent server uses the saved actual KE IP, absent port
   uses 123. A later family/address alternative needs its own bounded KE or
   admitted explicit delegation; do not send an old candidate’s default keys/
   endpoint provenance to a different IP. Serialize final current-state checks
   with complete publication. Exporter, DNS, endpoint, cancellation or generation
   failure discards the candidate; old state survives only while independently valid.
4. **Protocol/crypto producer and engine sender protect requests.** Implement the
   bounded protected writer and exclusive association/cookie owner. Fixture
   `WireToken`, `CompletionFacts` and plain `ntp-exchange` results grant no NTS
   authority. Issue a fresh ≥32-octet CSPRNG UID; save real T1 independently,
   request/socket/endpoint identity, monotonic deadline and generations.
   Reserve cookies through construction/send outcomes; prevent concurrent duplicate
   issuance and implicit reuse after ambiguous sends. Any allowed reuse needs its
   existing privacy-policy disposition. Bound retained bytes, copies and cleanup.
   At actual send, recheck association, destination/path and protected encoding.
   Exhaustion or failure enters bounded recovery, never ordinary plain fallback.
5. **Receiver admits the complete packet.** Reuse `frame_no_legacy_mac_datagram`,
   `read_nts_authenticator_parts` and, after S2C authentication/decryption,
   `frame_nts_plaintext`. The protocol/crypto owner still enforces UID/field
   placement/cardinality, AAD, selected-AEAD nonce/padding and cookie limits.
   Stage cookies and observation evidence until full packet admission; well-framed
   trailing unauthenticated fields have no semantic influence. Finally serialize
   saved endpoint/socket/request and configuration/association/clock checks with
   `send <= arrival <= processing_now < deadline`, publication and consume-once
   completion. Malformed/wrong-tag/wrong-UID traffic leaves a healthy pending
   request usable; genuine lifecycle invalidation retires it separately.
6. **Scheduler and numerical consumers retain separate outcomes.** Reset KE retry
   history only after protected NTP succeeds using that KE transaction’s keys/data;
   TLS completion or delayed old-key success cannot reset a replacement’s failures.
   The separately admitted KoD/NTSN path grants only bounded control/recovery effects.
   Packet/cookie completion does not imply eligible-observation freshness. Reuse
   S2.1’s lossless timestamp/header envelope, mapping, era/arithmetic and LI/scale
   checks, then its separately frozen reference-time/uncertainty-to-divisor contract
   and actual Core consumer. Preserve exact phase, signed raw RTT, units, errors
   and age; no fabricated zero, divisor `1.0`, first-sample readiness or
   fixture-derived production threshold.

### Observable exit and Windows boundary

Extend the existing bounded `nevrotick-observe` projection with actual KE/
association outcome, named trust-profile basis, packet authentication/current
completion, numerical eligibility/refusal and Core insufficient/ready/rejected
outcome. Include allowlisted stage/reason, provenance/age and resource/retry results;
exact field names stay with existing owners. Missing results remain absent.
Exclude keys, cookies, raw sensitive packets/UIDs and unsanitized native errors.

The [ordinary trust-profile card](s0.1-crypto-feasibility.md#ordinary-s22-trust-profile-and-native-work--5-november-2026)
and [Windows continuation](windows11-integration.md#430-ordinary-s22-windows-qualification--5-november-2026)
own OPEN-8 and CHECK-9/26/27. Keep CNG-first/Schannel-second candidate order.
Async `tokio-rustls` and synchronous `StreamOwned` remain alternatives to qualify;
either can block in a synchronous verifier. Retain permits and native inputs in
the actual job until completion; cancellation removes publication eligibility,
not capacity accounting. No replacement pools or detached retry growth.
Use [§17.1/THR-26](../docs/windows-time-architecture.md#171-bounded-async-and-native-work)
for stalled-work stop/escalation. Inventory actual delegated PKI fetch owners;
the NTS socket’s Direct binding cannot establish their egress.

### Paired acceptance evidence — planned, not run

| Boundary / existing CHECK owners | Useful control and discriminating refusal observation |
| --- | --- |
| KE/exporter; 6/9/17/26 | Complete exchange with independently compared exports/vector, split/coalesced/noncritical forms. Pair wrong identity/ALPN, truncation/Error, swapped context/key and first-export-only failure: no partial publication; secret cleanup observed. |
| Publication/path; 6/8/9/21/27 | Healthy warm replacement beside failure/race at each candidate stage; old state survives only if valid. Changed DNS without delegation retains actual KE IP. Allowed LAN/delegation succeeds; forbidden scope/port fails. KE-success/UDP-blocked remains incomplete; permitted alternatives make bounded progress with their own valid provenance. |
| Packet/lifecycle; 6/9/17/26/27 | Independent protected reply succeeds once beside wrong UID/tag, malformed inner/outer fields, duplicate, stale generation and processing after deadline. Rejects cause no forbidden packet/cookie effect and preserve a healthy pending reply. Exercise trailing fields/replenishment; zero-cookie expectations await the recorded local decision. |
| Recovery/resources; 6/9/21/27 | Exhaustion, ambiguous send, NTSN/refresh storms, old-key success and late native completion beside healthy progress. Observe actual sends, no plain fallback, same-transaction retry reset, retained permits and bounded teardown. |
| Windows trust; 9/26/27 | Selected-profile valid chain beside wrong usage/date/name, revoked/distrusted, removed root, unavailable revocation, slow retrieval and policy cutover. Expected acceptance/refusal follows the frozen profile; observe actual fetch owners and responsiveness. |
| Consumer; 6/7/8/25 | Authentic eligible evidence reaches actual history/estimator; authentic numerical refusal creates no vote/correction. Demonstrate sufficient history separately from first-observation insufficiency. Retain S2.1 numerical oracles and full-domain proof obligations. |

Use controlled-peer facts and independent vectors/oracles, not only same-code
round trips. Bind evidence to artifact/features/profile/Windows build and reached
production seam. P2 retains its two-independent-implementation interoperability
aim where practical; an unavailable second peer remains an evidence gap.

**Remaining gates:** complete graph/licence and selected provider/Windows trust
admission, finite runtime contracts including the local zero-cookie disposition,
the real numerical bridge and its proofs, and actual interoperability/resource/
native evidence. A declared normal-validation controlled lab peer/root can prove
that narrow path; final installed service/store/enforcement checks remain P3/P6.
Full MSIX, Slint migration and all P3 work are not prerequisites for the bounded
S2.2 prototype. Cached/resumed associations, wrong-date recovery, source defaults
and clock effects retain their later owners. Direct-first, OD-11/SOCKS-0,
complete MSIX, all native/S134/fuzz/storage holds, limited Sandbox scope and
optional enhanced release provenance remain unchanged. Source agreement closes
no CHECK; the [standards appendix](s0.1-protocol-comparison.md#ordinary-s22-composition-clarifications--5-november-2026)
also retains the unverified errata-coverage limit.

### S2.2 preparation integration and independent review — 5 November 2026

**Scope and predecessor:** this is the next ordinary-NTS source-preparation
handoff from the original audit. Three completed research lanes inspected pinned
GitHub implementations, primary RFC/web material and Microsoft Learn/API
contracts; a fourth agent wrote the synthesis Markdown and a fifth integrated
it into the existing engine/protocol/crypto/Windows tasks and documentation routes.
Initial context-window launch failures produced no usable research or approval;
only the completed retries and inspected sources support this handoff.

The source research used upstream `5e98c79901c90de77db6c6047cb483ec10b8cbd5`.
The coordinator reconciled ten intervening commits since the preceding S2.1
publication and then retained the additional Slint-skill commit
[`e68d953`](https://github.com/DavidOsipov/NevroTick/commit/e68d953c68573195427b00324ae6bb95856e328f)
before freezing this candidate. Its added skill files, AGENTS route and UI-M0
record remain upstream-owned; this handoff changes no skill or production source.

| Reconciled obligation | Disposition and affected owner |
| --- | --- |
| Current proof and UI order | Development §7's complete proof/Slint prefix is preserved. Scoped endpoint/Fraction proofs, the remaining public era-constructor obligation and the next `Fraction::compare` continuation remain with their existing owners; held Kani follow-ups are not restarted. |
| Direct/network, storage and reuse | Actual-KE-IP provenance, address-family progress, Direct-first/OD-11, network-state storage owners and the separate S2.3 lifetime/recovery scope are retained. No new proxy, persistence or ordinary-validation policy is selected. |
| Windows/product decisions | Selected Slint, complete MSIX, Windows binding/component reuse, all native/S134/fuzz/storage holds, limited Sandbox scope and optional enhanced release provenance are retained. The declared S2.2 lab path does not qualify installed services. |
| Current-state drift | AGENTS, the architecture capsule and guide routes now reflect the completed four-owner settled-rate fixture receipt and current §7 continuation; the capsule includes existing pure leap knowledge. Historical qualification records are not rewritten. |
| Added preparation | The existing S2.2 owner now has ordered production transitions, actual-consumer and paired refusal exits. The local zero-cookie disposition, ordinary trust profile, finite runtime/lifetime contracts and their implementation evidence stay explicitly open. |

**Frozen review identity:** actual upstream base
`e68d953c68573195427b00324ae6bb95856e328f`, full tree
`a2f6f9a6252edeecc8cc98fd87f244da06ac0e23`; ten changed Markdown files,
47,269-byte patch, SHA-256
`29233adde5b972b2d5743e993ef367fe043521c2d89b445c97efa59d902c9cf6`.
The local `6034d8c5a773bfa1b7d2e0d8e5904540d440f498` is a synthetic
partial-snapshot identity, not upstream history or a publishable repository tree.
The coordinator matched all ten baseline file blobs to the complete upstream tree.

**Review completion:** `s22_review` was dispatched with `fork_turns: none` and
read-only bounded scope. The initial launch failed with a context-window error
and supplied no review (`not run`). One bounded retry completed the requested
review (`completed`, `READY` for this documentation handoff). The report below
records its own comprehension, inputs actually inspected and evidence limits;
no parent discussion, private KB or current author verdict supplied its conclusion.
No material finding was established, so there is no `S22-R` resolution campaign.
Raw-report SHA-256:
`8a403464682cd4831790ded6023fc75c7dd95f3509f658a667df5d4206ef7322`.

**Coordinator verification before review:** changed-file whitespace checks and
1,266 internal Markdown link occurrences passed, with no missing target/heading
or unavailable anchor content. The five architecture Mermaid blocks, canonical
REQ/BND/DEC/THR/CHECK/OPEN identifiers, existing source entries, task histories and
current proof/UI prefix are preserved. All ten candidate hashes stayed frozen
through the review. No product tests, proof/native/fuzz/interoperability campaign
or dependency admission was performed. The complete upstream inventory has no
root `docs/SHA256SUMS.txt`; separate versioned design/asset manifests cover other
files and remain unchanged. No documentation-bundle integrity claim is made.

The verdict covers the frozen preparation patch, not this subsequently appended
receipt. The coordinator preserves the complete raw review below verbatim;
appending the evidence record does not convert source consistency into runtime,
Windows, P2 or release qualification. Implementation and its required reviews
remain open under the card above.

#### Verbatim independent review

> The change prepares the ordinary S2.2 acquisition path: a normal-date, certificate-validated NTS-KE transaction produces one complete association, that association protects a UDP exchange, and separately admitted time evidence reaches the real numerical consumer. It assigns the missing runtime transitions to the existing engine, protocol, crypto and Windows owners, with useful and refused controls at each boundary. It changes documentation and current-state routing; it does not implement these transitions or select the outstanding provider, trust profile, resource values or zero-replacement-cookie policy. The principal handoff is `tasks/engine-transport-pilot.md`, “Ordinary full NTS composition — S2.2 preparation, 5 November 2026.”
>
> The inspected workspace already contains six crates, inert NTS wire/negotiation kernels, a real two-peer loopback transport observer and a separate fixture exchange-to-Core chain. The transport currently ends in `TransportFacts`; it has no domain/Core dependency or authenticated association. Pure leap knowledge is present, and the existing settled-rate receipt records completion only for four Linux fixture owners. Production eligibility, the numerical bridge and their remaining proofs are still planned. `docs/plans/development-plan.md` §7 controls the current proof continuation; Slint is selected but its migration remains separately gated, Direct is the first-delivery route, SOCKS5 is deferred, and provider/native/storage/fuzz holds remain effective.
>
> ## Scoped verdict
>
> **READY for integration as the ordinary S2.2 documentation/research handoff at the frozen candidate identified below.** No material finding was established in this bounded review. The producer, state owner, consumer and future qualification relationships are coherent, and the inspected source and primary requirements support the consequential new assertions. This verdict grants no implementation admission, native experiment, product-security qualification, P2 closure or release approval.
>
> ## Candidate identity and review context
>
> | Item | Observed identity or scope |
> | --- | --- |
> | Review date | 5 November 2026 |
> | Upstream base recorded in the review packet | `e68d953c68573195427b00324ae6bb95856e328f` |
> | Upstream tree recorded in the review packet | `a2f6f9a6252edeecc8cc98fd87f244da06ac0e23` |
> | Actual local HEAD | `6034d8c5a773bfa1b7d2e0d8e5904540d440f498`, a synthetic partial-snapshot commit, not upstream history |
> | Frozen patch | 47,269 bytes; SHA-256 `29233adde5b972b2d5743e993ef367fe043521c2d89b445c97efa59d902c9cf6` |
> | Working candidate verification | The working `git diff --no-ext-diff` exactly matched the frozen patch. All ten manifest file byte counts, SHA-256 values and Git blob identities matched. Git reported only those ten modified Markdown files. |
> | Reviewer | Independent reviewer task `s22_review`; fresh-context review assignment, using the supplied request and inspected artifacts. No parent dialogue, private KB, saved memory or current analyst/integrator verdict was retrieved. |
> | Completion | Completed bounded read-only review. The earlier launch supplied no usable review and is not credited as evidence. |
>
> The upstream base/tree provenance comes from the supplied verified-snapshot packet. I verified the local candidate identities, not the entire remote upstream tree or omitted files. Missing unrelated snapshot files were not classified as deletions.
>
> ### Changed files actually inspected
>
> I read every hunk of the frozen patch, including the current-state corrections, across:
>
> - `AGENTS.md`
> - `docs/README.md`
> - `docs/plans/development-plan.md`
> - `docs/windows-time-architecture.md`
> - `tasks/README.md`
> - `tasks/development-backlog.md`
> - `tasks/engine-transport-pilot.md`
> - `tasks/s0.1-crypto-feasibility.md`
> - `tasks/s0.1-protocol-comparison.md`
> - `tasks/windows11-integration.md`
>
> ### Required context and source actually inspected
>
> | Input group | Actual inspection |
> | --- | --- |
> | Review requirements | Supplied independent-review request and candidate manifest; applicable root `AGENTS.md`; `docs/README.md` authority/routing and pertinent current amendments; `.agents/skills/planning/plan-independent-review/SKILL.md`; `.agents/skills/planning/references/review-and-handoff.md`; `.agents/skills/testing/qa-independent-review/SKILL.md`; `.agents/skills/testing/CONTRACT.md`. |
> | Canonical contracts and decisions | `docs/windows-time-architecture.md` §§1, 7.4, 17.1 and 18.7; `docs/nevrotick-owner-decisions.md` OD-10, OD-11, OD-13 and OD-05 §7.5; `docs/plans/development-plan.md` P2 and §7. |
> | Existing handoffs | `tasks/engine-transport-pilot.md` S2.1 measurement/consumer card, address-family admission/progress card and network-path/Direct routing card; `tasks/s0.1-protocol-comparison.md` RFC-alignment checklist and new ordinary-S2.2 appendix; `tasks/s0.1-crypto-feasibility.md` provider/native source leads, R-05 lifetime card and new ordinary profile card; `tasks/windows11-integration.md` W11-W01/04/05/08 ownership and §4.30. |
> | Qualification status | `tasks/s1.3-source-selection-readiness.md`, “Settled-rate final qualification — four selected owners,” including its exact declared scope, receipt identities and remaining exclusions. This was inspection of the retained receipt, not re-execution or independent reproduction of its campaigns. |
> | Workspace/dependency facts | Root `Cargo.toml` and the engine, wire, exchange, domain and Core manifests. The engine has only `getrandom`, `ntp-wire` and Tokio dependencies; Core is a dev dependency of the exchange fixture, not an engine runtime edge. No lockfile resolution or complete dependency audit was performed. |
> | Runtime/wire source | `crates/nevrotick-engine/src/lib.rs` transport facts and stage/commit/processing owners; `crates/ntp-wire/src/lib.rs` exports; pertinent `nts_ke.rs`, `nts_negotiation.rs`, `nts_packet.rs` and plaintext-framing declarations/contracts. Checked the concrete accumulator, negotiation, endpoint-hint and exporter-context APIs named in the handoff. |
> | Domain and consumer source | `crates/time-domain/src/lib.rs` and the leap source inventory; `crates/ntp-exchange/src/association.rs` receive/completion boundary; `crates/ntp-exchange/tests/support/scenario_driver.rs` actual `Composition::deliver`/`record` consumer path; `crates/time-core/src/regression/normalization.rs` fixture admission/provenance limits. |
>
> The three available research-lane reports were not used as review oracles. I inspected the cited primary sources directly. I did not read the current synthesis report.
>
> ### Primary external evidence actually inspected
>
> - [RFC 8915](https://www.rfc-editor.org/rfc/rfc8915.html), the affected KE completion, record, endpoint-default, size-guidance, retry, exporter and protected-packet/cookie requirements in §§4–5.7.
> - [RFC 8446 §6.1](https://www.rfc-editor.org/rfc/rfc8446.html#section-6.1), closure-alert requirements, and the regular-versus-early exporter distinction associated with §7.5.
> - [RFC 5297 §§3 and 6.1](https://www.rfc-editor.org/rfc/rfc5297.html#section-3), nonce placement and algorithm-15 key/nonce lengths.
> - [Chrony pinned `nts_ntp_client.c`](https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/nts_ntp_client.c), lines 200–225 and 404–547: saved-address fallback, cookie extraction and response-authentication completion.
> - [ntpd-rs pinned response consumer](https://github.com/pendulum-project/ntpd-rs/blob/46ec9bb4d5b6cb24f814f5543d85b9138afb4cba/ntp-proto/src/source.rs), lines 759–836; [KE result construction](https://github.com/pendulum-project/ntpd-rs/blob/46ec9bb4d5b6cb24f814f5543d85b9138afb4cba/ntp-proto/src/nts/mod.rs), lines 363–420; and [spawner resolution](https://github.com/pendulum-project/ntpd-rs/blob/46ec9bb4d5b6cb24f814f5543d85b9138afb4cba/ntpd/src/daemon/spawn/nts.rs), lines 124–165.
> - [Pinned rustls-platform-verifier Windows source](https://github.com/rustls/rustls-platform-verifier/blob/252e25161a91af476cbea620e29d277592d27ec7/rustls-platform-verifier/src/verification/windows.rs), bounded slices covering the SSL-policy flags, default engine/store, chain flags and timeout, date conversion, policy-status check and `ServerCertVerified` return.
> - Microsoft contracts for [CertGetCertificateChain](https://learn.microsoft.com/en-us/windows/win32/api/wincrypt/nf-wincrypt-certgetcertificatechain), [CertVerifyCertificateChainPolicy](https://learn.microsoft.com/en-us/windows/win32/api/wincrypt/nf-wincrypt-certverifycertificatechainpolicy), [system-store locations](https://learn.microsoft.com/en-us/windows/win32/seccrypto/system-store-locations), [CryptRetrieveObjectByUrlW](https://learn.microsoft.com/en-us/windows/win32/api/wincrypt/nf-wincrypt-cryptretrieveobjectbyurlw), and [BCryptGenerateSymmetricKey](https://learn.microsoft.com/en-us/windows/win32/api/bcrypt/nf-bcrypt-bcryptgeneratesymmetrickey).
>
> ## Material findings
>
> **None established.** No `S22-R` finding ID is allocated. The following are the principal checks supporting that scoped result, rather than additional approval gates or claims of executed behavior.
>
> | Reviewed boundary | Assessment and evidence |
> | --- | --- |
> | Current state versus planned system | The new capsule and engine card correctly describe absent authenticated acquisition and the production numerical bridge. The manifests and actual `TransportFacts` producer support this distinction. The settled-rate correction is limited to the receipt's four-owner Linux fixture scope; it does not close OD-10 or P1. |
> | KE parsing and exporter authority | Engine handoff steps 2–3 match the existing accumulator/negotiation API boundaries. `NtsKeExporterContext` owns five context bytes and a label but neither a connection nor keys. Requiring same-job binding, both 32-byte direction keys and complete publication prevents the handoff from treating inert metadata as authentication. Error/non-error cardinality and TLS closure remain explicitly distinguished. |
> | RFC details and upstream differences | The size target retains SHOULD strength, and the closure wording does not invent a mandatory peer-alert wait. The zero-replacement-cookie distinction is accurate: the server's response obligation is separate from the explicit client UID/authentication checks. The cited Chrony extractor rejects when no acceptable replacement is extracted; the inspected ntpd-rs consumer processes new cookies after acceptance. The document leaves NevroTick's acceptance/completion/retry disposition pending instead of selecting policy from this difference. |
> | Endpoint and replacement ownership | Step 3 retains the actual KE peer IP when no server is negotiated and port 123 when absent, consistent with §7.4 and RFC 8915 §§4.1.7–4.1.8. New address/family progress needs its own valid KE/delegation provenance. Private candidate construction, complete publication and independently valid old-state retention agree with the existing address-family and Direct contracts. |
> | Packet completion and numerical eligibility | Steps 4–6 keep fresh UID issuance, exclusive cookie ownership, protected send, complete packet admission and consume-once state at identified owners. The current envelope reader intentionally lacks those semantic guarantees. The paired controls retain a healthy pending reply after malformed/uncorrelated traffic, while authentic numerical refusal creates no vote. The actual fixture consumer's `1.0` divisor is correctly excluded as production uncertainty evidence. |
> | Windows trust and native work | The profile freeze and truthful opaque-result language match both the inspected verifier and §18.7.2. A null default chain engine is current-user context; successful API execution is separate from successful chain-policy status. The source checks leaf revocation, suppresses unknown-revocation policy errors and exposes an opaque success marker. The handoff does not convert this into confirmed non-revocation or automatically require a new verifier. Real-job permit/input custody, queued-job bounds and delegated-fetch ownership remain with §17.1 and W11 owners. |
> | Stage and authority | Development §7 keeps the current proof queue. The new Windows card distinguishes a declared ordinary-validation lab path from final installed-service qualification. No complete-MSIX or Slint prerequisite is added to the bounded S2.2 prototype; no existing installed-product gate is marked complete. OD-11's Direct/SOCKS split and OD-05's optional enhanced provenance remain intact. |
>
> ## Evidence limits and retained open work
>
> This review performed read-only Git/hash checks, documentation/source inspection and bounded primary-source retrieval. It did not run Cargo, native Windows, mathematical-proof, fuzz, interoperability or resource campaigns; install anything; alter trust stores; contact NTS peers; modify the candidate; or publish anything. The raw report is the only output written.
>
> The review establishes consistency of the named handoff, not whole-repository correctness, complete source coverage, dependency/licence clearance or an independently repeated historical qualification. The partial snapshot's unrelated omissions are outside the finding set. I did not establish current RFC verified-errata coverage; the candidate already states that limit without claiming that no errata exist.
>
> Exact provider/transport admission, the ordinary Windows principal/store/revocation/egress profile, finite lifetimes and aggregate budgets, post-EOM runtime transitions, the zero-cookie disposition, the justified numerical consumer contract and proofs, and actual portable/native/interoperability evidence remain open at their declared dependent stages. Their absence is not a demonstrated product failure or a contradiction in this source-preparation handoff. Any change to the reviewed semantics or relevant candidate bytes requires rechecking the affected conclusions.
>
> ## One permitted next action
>
> Record this completed, candidate-scoped READY review verbatim in the existing engine task as part of integrating the ordinary S2.2 documentation handoff.
