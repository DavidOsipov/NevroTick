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
