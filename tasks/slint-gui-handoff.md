# Slint GUI handoff for the next AI

Prepared 6 November 2026. English working notes; keep product copy readable.
This is a continuation guide, not a second implementation plan. The
[S0.5 task](s0.5-ui-comparison.md) owns candidate history and evidence.
Remote review: [PR10](https://github.com/DavidOsipov/NevroTick/pull/10).

**Current scheduling — 27 November 2026:** [development §7](../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent)
selects the [original joined-receipt source child](engine-transport-pilot.md#consume-once-admission-handoff-for-original-joined-protected-receipts--27-november-2026).
Physical-reference qualification is [HELD/BLOCKED](windows11-integration.md#v1-reference-routing-and-documentation-reconciliation--27-november-2026),
with no recurring hardware inventory or procurement on resumption. Independent
GUI source work continues under its own prerequisites; live backend, D1–D3,
native and security/metric gates remain. [V1 NTP/NTS/Roughtime scope](../docs/nevrotick-owner-decisions.md#v1-protocol-and-hardware-scope--27-november-2026)
adds no GPS/PPS/refclock runtime or UI requirement.

The [9 November imported-findings repair](codeql-cloud-handoff.md#imported-full-repository-findings--9-november-2026)
adds process-owned AccessKit subclass custody and standalone actual-module facade
controls. Reuse that exact source/evidence chain; this memory-safety correction
does not activate D1–D3 or establish native USER32/UIPI/UIA, resource-retirement,
critical-file metric or release qualification.

## Read first

Read the current checkout's `AGENTS.md` and [documentation guide](../docs/README.md),
then S0.5's active goal, [current native/text2 integration](s0.5-ui-comparison.md#current-nativetext2-integration--gui30-and-later)
and [hardening handoff](s0.5-ui-comparison.md#planned-hardening-gui-readiness-review--6-november-2026).
Consult the [v0.4 reference](../docs/nevrotick-design-v0.4/README.md),
[GUI source guide](../crates/nevrotick-ui/ui/README.md),
[input audit](../docs/nevrotick-gui-input-admission-audit.md),
[native residuals](../docs/nevrotick-gui-native-input-residuals.md) and
[text development guide](../docs/nevrotick-gui-text-development.md).
Historical checkpoints retain their original failures, scope and holds.

For the current calendar dependency, follow the OD-13 decision and the
build evidence. The desktop Slint Core derivative uses `time 0.3.55`.
Keep the default DatePicker grammar and bounded Gregorian representation
when updating the patch. Native Windows local-date checks still own
midnight, timezone changes and daylight-saving behavior.
[OD-13 calendar decision](../docs/nevrotick-owner-decisions.md#slint-calendar-dependency--24-november-2026)
[Build evidence](build-optimization.md#replace-chrono-in-the-selected-slint-desktop-graph)

## About cancellation candidate — 27 November 2026

The fixture candidate adds distinct Cancel check and Cancel update actions and an
inline cancelled result. It retires the current timer before publication and
requires the current About page, generation and a pre-install phase. A fresh check
is available after cancellation. Installation refuses cancellation. Stale callback
and tick controls include a replacement check and an accepted replacement update.
This remains an in-memory sample; it performs no download or installation.

The candidate adds three update meanings (ordinals 610–612), raising the active
roster to 525 with a matching map ceiling. Schema 2, text contract 4, English
revision 4 and the four-million decoder work bound remain. Compact formatting in
seven argument-free catalog sections gives 3,998,450 calculated JSON work units;
actual Rust JSON/CBOR admission and the new callback controls remain unverified
until the selected UI compiler and test lane runs on this exact candidate.
The v2/v3 fixtures retain the earlier incompatible-version controls. Native UI,
critical-text approval and production updater qualification remain open.

For the revised, independently reviewed candidate, one integrated default-GNU
UI verification attempt starts at 2026-11-26 21:49 UTC and ends by 22:04 UTC.
It selects the admitted isolation, text and source controls, formatting,
compiler, Clippy, UI tests and doctests, including cancellation and the actual
JSON/CBOR consumers. Four build jobs use an owned Windows-volume scratch path;
the Linux disk and memory floors remain active. The earlier six-minute attempt
on the narrower candidate stopped before tests and is not reused as a pass for
this changed source. Native Windows and production updater gates remain open.

The integrated attempt passed its source controls, dependency inventory,
format, compiler, Clippy and Rustdoc, then stopped while building the UI
test binary: rustc/LLVM reported allocation failure and aborted before any
UI tests ran. The sandbox had a 4-GiB per-process address-space limit; host
available-memory observations stayed above 5.51 GB near the failure and do
not establish which allocation boundary caused it. The monitor did not stop
the run, and the owned processes retired. The incomplete receipt is
`%LOCALAPPDATA%/NevroTick/ui-cancel-final-2711.jN9aWjYj/receipt.json`,
SHA-256 `a9ee59f73264afa08157a5697ad6b86f50fdca3bd276f5cf84e57c254e7d3734`;
its 765-input inventory SHA-256 is
`620a8d64a5f06c2836db6e80318199a7cc066ebdb5a748ea6e7856fc1ae6cdac`.
The callback, Rust JSON/CBOR and doctest results remain unverified for this
candidate; the earlier candidate's completed lanes do not fill that gap.

## Manual updater contract and acceptance — 25 November 2026

**Status: accepted product/design handoff; bounded Rust/Slint About preview implemented
on 26 November; production updater and native qualification remain open.**
Use the [implementation checkpoint](#implement-the-one-consent-about-simulation--26-november-2026)
for the delivered fixture and its exact evidence.
Use [OD-05 §7.10](../docs/nevrotick-owner-decisions.md#710-tuf-authenticated-direct-msix-updater--25-november-2026),
[architecture §19.8](../docs/windows-time-architecture.md#198-tuf-authenticated-direct-msix-updater),
the [adopted update assessment](../docs/windows-time-update-assessment.md#12-adopted-direct-msix-updater--25-november-2026)
and [W11 §4.40](windows11-integration.md#440-tuf-authenticated-updater-integration--25-november-2026)
for trust, signing, privileged storage/deployment and surviving recovery. This
section owns the UI seam and observable cases within existing S0.5/P5, not a
second updater plan. The [product journey](../docs/windows-time-features.md#manual-update-journey--25-november-2026)
and [critical-copy contract](../docs/plans/ui-localization.md#updater-consent-and-result-copy--25-november-2026)
describe the same one-consent behavior. Earlier proposal text with separate
Check, Download and Install decisions is superseded by David's correction.

### HTML updater prototype — 25 November 2026

David requested the accepted journey in the existing
[v0.4 HTML design reference](../docs/nevrotick-design-v0.4/README.md), starting
from `2454f115d9494b722d90e55b54e88d375fd39825`. This is a separate in-memory
presentation sample within the existing S0.5/P5 handoff. The native source
readback and planned UP-UI acceptance below retain their own scope.

About is shared by Simple, Power user and the existing tray destination.
**Check for updates** simulates checking, then presents the exact eligible
offer with **Update now / Later**. One acceptance closes the offer and
automatically progresses through **Downloading → Verifying → Installing**
to an inline result. There is no separate Download action or routine second
installation confirmation. Later, Escape or unaccepted-offer dismissal grants
no transfer intent. **Review update** reopens the retained offer; **Check again**
starts a fresh sample check. **Cancel check** and **Cancel update** are available
at their respective pre-install stages; installation offers no cancellation.
**Check installation status** reconciles an unknown result without another
installation. **Restart app** is shown only after a known Installed result when
the running version is older; it changes that in-memory running-version sample.
A required Windows restart remains a separate unfinished-maintenance state and
cannot be cleared by restarting the app. The versions
`1.0.0 → 1.1.0`, package facts, trust results and installed outcomes are explicitly
demonstration data. They establish no actual released or installed product.

The Russian reviewer control outside the application selects one of 14 next-sample
outcomes. The default available-update route has contrasting no-update,
offline/metadata/time/policy/management, package-verification refusal, UAC-denial,
unknown-installed-identity, Windows-restart, unknown-outcome and recovery cases.
The two unknown-outcome samples reconcile to a completed installation or confirmed
absence of installation. Target/terms invalidation has model controls, with no
separate reviewer control exposed. The
[reviewer guide](../docs/nevrotick-design-v0.4/README-RU.md#обновление-приложения-из-about)
describes their use. The sample makes no network request, transfers no package,
writes no updater file and invokes no Windows, service, UAC, signing, hashing
or installation operation. Its finite timers drive presentation only.

**Executed checks on the integrated HTML candidate:**

| Check | Observed result |
| --- | --- |
| `node build.cjs` | Pass; standalone HTML is 902673 bytes. |
| `node --check src/model.js`, `node --check src/views.js`, `node --check src/app.js` | All three syntax checks pass. |
| `node --test tests/*.test.cjs` | 232/232 pass: 207 existing tests, 17 updater model tests and eight updater view tests. |
| `tests/render-fixtures.cjs` and `tests/check_structure.py` | 1561 serialized HTML fixtures; zero structural errors, CSP hash matches the final bundled script, zero external standalone dependencies and original AVIF bytes preserved. |
| `sha256sum --check SHA256SUMS.txt` from the bundle directory | All 185 current entries match; the original imported receipt remains unchanged. |

The new updater view tests and structural fixtures cover both modes and all
14 selected outcomes. Source review checked focus transfer to status when
Check starts; actual browser focus placement remains unverified. The independent
review exercised the persisted-`pageshow` handler with controlled Node timers
and a stub DOM, not an actual browser history cache.

**Independent review:** `updater_proto_judge` began with a fresh context
(`fork_turns="none"`) and read the baseline and applicable contracts before the
implementation results. Its own bounded checks passed 28 journeys (14 outcomes
in each mode), 14 decline controls and 22 stale/cancel/renewed-consent controls.
Four cases executed the actual controller with controlled timers and a stub DOM:
automatic completion after one Yes, persisted-page resumption, cancellation
remaining stopped after restoration, and reset-guard pause/resume. These are
controller checks, not rendering or browser-focus evidence. The reviewer also
checked the 134 updater records within the 1561-record structural artifact and
independently bound the generated script/CSP to the final source and original
AVIF bytes. The 232-test full run and 185-entry manifest check above were
coordinator-executed, not additional independent runs.

The review found and verified two narrow corrections: **J-UP-01**, the delivered
HTML footer now identifies its Chromium evidence as historical and discloses
the current browser-verification gap; **J-UP-02**, reset disclosure now includes
updater progress/results and pending or unknown updater samples, without
claiming to reconcile a real installation. Cancelling reset preserved updater
state in the reviewer's Installing and Outcome unknown controls. The final
verdict found no unresolved material issue within the reviewed source, model,
controller, rendered-structure and offline-package scope. The reviewed standalone
SHA-256 is `a449b123fd3072c2c3a229590b6f27c0075c229011fd9923a4f032c16ee91a00`.

**Current browser verification is incomplete:** `/usr/bin/chromium`
points to a missing executable, the official pinned-browser download returned
HTTP 403, and the cloud browser could not reach the local preview before an
explicit URL-policy refusal. No successful browser/visual or native updater
qualification is claimed. The retained 5 November status/notification and tray reports
remain evidence only of their original candidates. They are not a current
updater pass. Documentation/handoff used `plan-code-change` and
`qa-context-checkpoint` to bind this scope to the existing source owners;
`qa-regression-selection` identified About, tray navigation, generated HTML/CSP
and shared dialog/layout consumers for the coordinator's checks.

No Rust/Slint updater, TUF client, signature verifier, protected staging,
surviving maintenance owner or native updater result is added by this HTML
change. UP-UI-01–12 and their actual effect/receiver/native gates below remain
planned. D1–D3, critical-text approval, current product priority and existing
execution holds retain their owners and re-entry conditions.

#### Remaining HTML browser review

**Owner/status:** the existing S0.5/P5 UI and accessibility/QA owners retain this
bounded, open prototype checkpoint. Start from the published HTML source in
[`da8d0e3`](https://github.com/DavidOsipov/NevroTick/commit/da8d0e3b52ecb0dd49dd0564bd81e360b4ff268d)
and the artifact identity above; bind any later changed candidate separately.
Resume when a permitted working browser environment is available. This is not
a new Core prerequisite, native D1–D3 re-entry or production UP-UI completion gate.

Use one bounded browser pass and record browser/version, candidate, entry point,
actual observations and any unavailable case. Inspect these behaviors:

1. Both standalone and multi-file entries, Simple/Power About and tray → About;
   one Update now reaches the sample result without another product confirmation.
2. Real keyboard/focus behavior: Check to status, initial Later focus, Tab/Escape,
   offer dismissal and focus restoration, plus reachable cancel/error/reconcile
   actions. An unaccepted offer starts no simulated download or installation
   progression.
3. All 14 result choices at a desktop width, with representative offer, progress,
   failure and unknown/restart states at a narrow width and enlarged text.
   Required facts/actions remain visible; inspect overflow, labels and status text.
4. Cancel during Check/Download/Verify, navigation and mode changes during work,
   retained-offer reopening, and cancel-versus-confirm reset during Installing
   and Outcome unknown. Late callbacks must not resume cancelled/reset work.
5. Actual persisted-page restoration where supported: current work resumes once,
   cancelled work stays stopped and stale results cannot replace a newer operation.
   A synthetic `pageshow` call alone does not close this browser observation.
6. Both unknown-result reconciliation outcomes, separate installed/running
   versions and the Windows-restart requirement. Observe browser errors and
   unexpected requests; the sample makes no updater network requests, package
   transfer or native effects. Local CSS/JS/AVIF loads in the multi-file preview
   are expected.

Keep unavailable browser observations open. A defect gets
a focused source repair and affected rerun; absent evidence does not justify
repeating unrelated suites or changing browser/host policy. Attach the bounded
result here, preserving the existing source/Node/structure receipt. Neither this
HTML pass nor its current gap qualifies or blocks unrelated native product work.

**Browser availability follow-up — 27 November 2026.** A cached Chromium
executable was found outside PATH at
`~/.cache/ms-playwright/chromium-1248/chrome-linux64/chrome`, SHA-256
`377a9fc07b495c3b8d0ab8ec1fdfd153641759f9756fea34d8dfce4de82f5025`.
An installed Playwright alpha driver names browser revision 1247, so its
compatibility with this binary and both tool trees' admission remain open.
The current standalone SHA-256 matches the published value above; the bundle
manifest's 185 entries also match. Neither a cached executable nor these
source checks establish a working or permitted browser environment. The one
prototype browser pass remains unspent and no browser was launched in this
follow-up. An inert, separately bounded preflight must establish exact tool
identity, offline credential-free isolation and owned process cleanup before
the complete browser review; no package download or host profile is selected.
The inert readiness stage runs `04:13:39Z`–`04:28:39Z` on 27 November 2026,
including preparation, independent review, at most one causal correction and
one bounded blank-page browser attempt. It binds exact browser/driver/runtime
trees, forbids external networking and host credential/profile mounts, retains
first failure and retires its owned scope. Root and scratch must retain the
existing four-GiB filesystem floor and host memory the one-GiB floor; finite
CPU, memory, process, output and wall limits must be admitted before launch.
This preflight executes no prototype and does not consume or pass the separate
one-pass browser review. If its prerequisites or launch fail, stop with an
incomplete result; do not silently retry or relax the boundary.
Source inspection of the installed driver found that it disables Chromium's
back-forward cache by default. The later prototype pass must narrowly account
for that default and observe a genuine persisted back/forward restoration;
a synthetic event or ordinary launch is not that evidence. The first inert
launcher draft failed independent source review: its timeout/failure cleanup
did not establish retirement of all scope descendants, its final tool
bookend occurred only on success, and its namespace/network/identity checks
were not an admission handshake before loading the driver. No browser was
started and those gaps must be resolved before the one inert attempt.
The stage stopped **incomplete** after that review; no browser or prototype
test ran. Its private draft under
`~/.local/share/nevrotick/browser-inert-preflight-20261127-0414/` retains
`driver.cjs` SHA-256 `a6ca440bfc627af189feaaa137aa64cf77e57bb5274f024a564f19ae76972554`,
`supervise.py` SHA-256 `ae455264fc4624dc8bbb4471de685438bb4bf7ee121603544ab5f8c90a59383a`,
and the tool inventory SHA-256
`a2e192fd50102f522763043dec25ac35c17a0e6f31963da5e55019e7a32aa46a`.
The earlier isolated supervisor admits a different fixed Python suite and
resource profile; it cannot be reused as Node/Chromium admission. Re-entry
requires a separately reviewed pinned-process identity gate and exact-scope
watchdog, or an already admitted browser environment, plus fresh tool and
resource checks. Two inventoried library links also lead to unmounted
`/etc/alternatives`; whether Chromium needs those libraries is unknown and the
runtime closure must be checked without broadening host mounts by assumption.
The prototype's one browser pass remains unspent.

**Documentation routing follow-up — 25 November 2026:** based on
`a4a3bb043fdb09126d3659595d3ff8b20cdc8198`, the existing indexes, feature/update
assessment, S0.5/UI-M0/P5 queue and W11 task now distinguish delivered HTML,
open browser review and pending Rust/Slint work. `plan-code-change` preserved the
existing owners/order; `qa-regression-selection` selected documentation and
unchanged-artifact checks rather than rerunning code suites. The coordinator
verified 23 added local links/anchors, all 185 unchanged prototype-manifest
entries and preservation of the upstream W11 receipts before §4.40; no root
`docs/SHA256SUMS.txt` exists. Product/prototype source, skills and AGENTS remain
unchanged. Fresh-context `updater_doc_judge` established expectations before
scout conclusions, reviewed the ten-file delta and independently passed
`git diff --check`. JD-01/JD-02 were corrected: browser decline observes no
simulated progression, and local preview assets are distinguished from forbidden
updater traffic/effects. No unresolved material documentation finding remained;
no code, browser or native qualification was executed by that review.

### Current source and next boundary

**Current disposition — 26 November:** the [implemented About preview](#implement-the-one-consent-about-simulation--26-november-2026)
and its [Rust receiver](../crates/nevrotick-ui/src/slint_host/update_workflow.rs)
provide typed in-memory offer/progress state and Check/Yes/Later callbacks.
Reuse that bounded fixture; its recorded callback checks do not close the full
UP-UI-01–12 matrix. Broader production target/intent and worker/helper contracts,
trust/time/protected storage, live integration, native observations and independent
critical-text approval remain open under the existing phase/trigger owners.

**Historical predecessor readback:** the table and next-slice description below
retain the source state and proposed work at
`59838326655cd8272e0c3ced593e43c44d99d2b8`, before the Rust/Slint implementation.
They are not the current fixture queue; the HTML sample itself added none of
these Rust/Slint paths.

| Existing source | Historical fact and proposed work at that baseline |
| --- | --- |
| [`ui/about.slint`](../crates/nevrotick-ui/ui/about.slint), [`ui/app.slint`](../crates/nevrotick-ui/ui/app.slint) and [`ui/fixture-copy.slint`](../crates/nevrotick-ui/ui/fixture-copy.slint) | About exists in the shared Simple/Power shell and has a fixed repository action. Version status is development/fixture wording; GUI build and text-contract numbers are separate rows. No Check-for-updates callback, live installed-product/version query, updater offer or progress/result consumer exists in this surface. Extend it and the existing tray About destination. |
| [`src/slint_host.rs`](../crates/nevrotick-ui/src/slint_host.rs) and its workflow modules | Rust `BindingState`, `Rc<RefCell<…>>`, bounded models, weak window callbacks and one Slint event loop own fixture state/projections. Add a separately checked update-operation/intent projection at the admitted stage; generated widget values and `host_ready` cannot become updater authority. Existing source-editor “Update preview” callbacks are unrelated to software updates. |
| [`src/main.rs`](../crates/nevrotick-ui/src/main.rs) and [`Cargo.toml`](../crates/nevrotick-ui/Cargo.toml) | The Windows entry calls the Slint host; the UI package is `0.0.0` and described as fixture-only. The UI has no Tokio dependency. Neither the Cargo version nor a running GUI identifies the currently installed complete product. No updater client, privileged updater endpoint or package-apply implementation is supplied by this source review. |
| Existing catalog and [`native_notifications_windows.rs`](../crates/nevrotick-ui/src/native_notifications_windows.rs) | Typed catalog publication and sample notification lifecycle exist. `Package::Current()` guards notification identity; it does not supply an updater's actual installed-product/version reconciliation. The updater still needs code-owned messages, independent critical-content approval and current typed result producers. Sample toast/portable receipts supply no live updater or native delivery pass. |

At that predecessor, the next Rust/Slint source slice was a bounded in-memory operation model and
About fixture using that established ownership, with sample labels and disabled
live/protected effects. Freeze its typed states, exact-target intent, finite
work/text budgets, independent cases and message-contract changes first. Keep
one graph/schema owner. Do not insert a hidden live adapter, actual DNS/download,
UAC or package action into UI-M0. Preserve the normal development queue. The
earlier adoption-documentation stage changed no source or dependency; the later
HTML implementation above changes no Rust/Slint source or dependency.

### One consent, independently checked effects

1. **Check for updates** starts on-demand, ordinary-user metadata retrieval and
   bounded verification. It transfers zero MSIX package bytes, advances no
   machine trust history and grants no installation intent. Offer only a target
   eligible under the admitted preview trust/time/policy contract. Failure or
   uncertain time is a failed/blocked check, not “Up to date.” Initial scope is
   one stable channel and complete packages, without scheduled checking.
2. **Update to {version} now?** shows actual installed identity, exact offered
   version/channel, complete download size, bounded literal notes and material
   consequences. Explain known service interruption, app closure/restart, UAC/OS
   prompts, possible/known OS restart and current metered/roaming/management
   restrictions before **Yes, update now**. Unknown required eligibility is a
   block; unknown unavoidable consequences must be stated, not hidden by a
   reassuring default. **Not now**, Escape or dismissing the unaccepted offer
   authorizes no package transfer or install.
3. One **Yes** creates intent for that operation/target and disclosed terms. It
   automatically drives **Downloading → Verifying → Installing**, subject to
   every independent gate. Do not insert a separate Download button, a routine
   Install-now button or another pre-install confirmation. Mandatory UAC/Windows
   prompts remain possible. At offer time only metadata is verified; full-file
   hash/length, expected package/publisher identity and Windows signatures are
   verified after transfer and again at their trusted admission boundary.
4. The privileged lifecycle receiver independently consumes bounded raw input,
   revalidates current TUF/time/policy/installed state and exact protected package
   bytes, and serializes effects under architecture §19.8. A UI Boolean,
   serialized “verified” target or Yes is neither cryptographic proof nor
   receiver authorization. The GUI supplies no arbitrary path, URL, command,
   installer argument or new clock-broker verb. TUF and Windows MSIX verification
   are the selected trust layers; there is no automatic PGP key generation or
   extra detached-signature user workflow.
5. A different target or changed material consequences, stale/expired intent or
   invalid current authority ends the automatic sequence. Present the refreshed
   admissible offer and obtain a new Yes before a new attempt. Refreshing the
   same still-valid target/terms is not a reason for routine duplicate consent.
   Required policy denial cannot be bypassed by requesting another Yes.

Bind intent to the operation/observer generation, exact target version, channel,
package identity, digest and length, current installed identity, reviewed material
terms and the applicable trust/policy/time/maintenance revisions. Define expiry
and invalidation with the trusted owners before implementation. A changed
installed product in another user session invalidates the stale intent. UI
presentation generations also bind catalog revision and the selected view;
neither controls machine trust. The privileged check repeats current guards at
apply and never installs “whatever is latest” under an earlier Yes.

### Threads, results and actual completion

Keep callbacks short and all Slint components, `Rc` models and mutations on their
owning UI thread. On-demand work receives owned bounded data; network, parsing,
hashing, certificate verification and deployment do not run inside callbacks.
Use the existing reviewed weak-handle/event-loop delivery mechanism, checking
operation/observer generation and current intent again before publication. Bound
queued bytes/results and coalesce replaceable progress. A queued callback is not
an applied result. No new UI/helper Tokio runtime, nested executor or unsafe
`Send`/`Sync` workaround follows from adopting an updater library.

Cancel revokes future publication/effect admission. The native operation owner
keeps buffers, staging, handles, permits and charged resources until actual
completion or durable recovery; dropping a future or closing the window proves
no cancellation. Before maintenance effects, the owner must confirm cancellation
before promising an unchanged installation. During irreversible/committed work,
show truthful pending/unknown status and supported actions, never a misleading
Cancel guarantee. UI shutdown must not orphan maintenance or a late privileged
effect. The qualified surviving lifecycle owner, not an in-memory UI task, owns
reconciliation after UI/helper termination or reboot.

The closed presentation states are **Not checked, Checking, No eligible update,
Available, Downloading, Verifying, Installing, Restart required, Installed,
Failed, Cancelled before effects, Outcome unknown, Reconciling, Recovery required,
Managed externally** with bounded typed reasons. Preserve these distinctions:

- A failed check is not a successful no-update result. Last successful check,
  latest failed attempt and age quality are separate; a bad wall clock cannot
  produce a reassuring freshness label.
- Installed package identity, running GUI identity, pending restart, service
  readiness and completed maintenance are separate observations. OS request
  acceptance alone proves none of the later outcomes.
- A lost acknowledgement requires read-only reconciliation with the durable
  operation/actual package state before retry; never automatically repeat install,
  claim rollback, lower a security floor or refund clock authority. A definite
  pre-effect denial may state no installation; uncertainty may not.
- Closing/reopening the observer or reconnecting never replays a cached Yes.
  Late results may support the owning operation's reconciliation but cannot
  overwrite a newer offer, silently retry or recreate an expired intent.
- Managed-externally status needs admitted policy; merely finding WinGet is
  insufficient. No WinGet, release-page or App Installer fallback follows from
  a failed in-app verification.

Use in-window About/Diagnostics and the existing tray About navigation. No new
background watcher or rich update-toast release scope is admitted. If update
notifications are adopted later, their fixed action only opens current status
after session/generation checks; it cannot choose a target or grant consent.
Keep observer lifetime, OS permission, submission and actual display distinct.
Emit bounded typed, redacted progress/refusal/outcome events through the existing
logging owner; no raw metadata/notes/headers/tokens, arbitrary paths or remote
error strings. Logs do not supply authoritative UI state; log loss cannot change
a denial into acceptance.

### Planned acceptance and evidence routing

All cases below are **planned, not executed**. Pair each refusal with a useful
valid neighbor at the actual receiving boundary. Freeze finite scenario/work
budgets and candidate/graph identity before execution. Portable fixture checks
cover only their model and callbacks; privileged and Windows observations require
the admitted integration/lab and do not follow from a screenshot or cross-build.

| Case / requirement | Action and independently observable expected result | Owner / cheapest adequate layer and later gate |
| --- | --- | --- |
| UP-UI-01 — healthy one-consent journey; AC-23 | A qualified metadata result offers a complete target. Count one affirmative product decision; it drives real download, full verification and installation without a second routine prompt. Observe exact admitted bytes/identity and authoritative installed/maintenance result; account separately for UAC/OS prompts. | UI model/callback cases first; updater/helper/package integration and Windows lifecycle for effects. CHECK-3/13/14/18/22/23/27. |
| UP-UI-02 — decline/no prefetch; AC-23 | Observe Check, then Not now, Escape and unaccepted-offer close. Metadata reads may occur; package bytes, installer invocation and privileged effects remain zero. A fresh subsequent Check/Yes still works. | UI/transport admission counters and controlled retrieval seam; actual native/request observations at integration. CHECK-3/14/18/25. |
| UP-UI-03 — independent helper refusal; AC-24 | Bypass widgets and submit a forged UI “verified” flag with a hash/length mismatch, then invalid/wrong-publisher package signature. The actual privileged receiver refuses before maintenance; a valid same-policy neighbor succeeds. Do not count rejection by an unrelated earlier guard as signature evidence. | Security/helper owner; source contract controls followed by actual verifier/protected-stage/native caller tests. CHECK-3/16/18/23/25/26. |
| UP-UI-04 — stale selection and exact intent; AC-24 | Replace the offered target or relevant policy/time/installed identity after Yes, including another user's update. The old intent cannot install the new target or proceed stale. New material terms/target require a new offer/Yes; unchanged still-valid revalidation adds no routine prompt. | Deterministic UI/helper transition cases, then serialized multi-user Windows route. CHECK-3/14/18/23/27. |
| UP-UI-05 — cancellation and late delivery; AC-25 | Cancel/replace the operation, close its view and deliver older check/download/verification callbacks. No current view overwrite, re-enabled action or future apply occurs. Pending native work keeps charged resources until observed completion; the new operation remains useful. | UI/lifecycle owner; deterministic barriers and actual callback/resource completion at Windows gate. CHECK-14/21/25/27. |
| UP-UI-06 — UAC denial; AC-25 | Deny/cancel UAC before privileged dispatch. Show the confirmed denial, preserve installation and return keyboard focus; do not claim installation or auto-repeat elevation. A later deliberate current attempt succeeds. | UI/helper owner; native ordinary-user and alternate-admin scenarios, with keyboard/Narrator. CHECK-3/14/22/23. |
| UP-UI-07 — lost acknowledgement; AC-25 | Let deployment be accepted, lose the acknowledgement and reconnect/reopen. Show outcome unknown/reconciling, prevent automatic reinstallation and query durable/actual state. Resolve both completed and definite no-effect controls honestly; an old running GUI cannot overwrite new installed identity. | Helper/package owner with UI projection; fault-controlled Windows lifecycle. CHECK-12/13/14/18/23/27. |
| UP-UI-08 — surviving recovery; AC-15/25 | Terminate UI/helper, fail new-helper activation or reboot during maintenance. The qualified owner retains staging/trust history/recovery and establishes one coherent installed state or explicit recovery block, with current clock ownership and consumed authority preserved. Reopening UI observes that result. | W11-W05/W10 and P6, UI consumer; owned disposable Windows recovery scenarios. CHECK-5/12/13/18/23/27. |
| UP-UI-09 — unavailable eligibility/management; AC-26 | Offline retrieval, expired metadata, insufficient trusted time, unknown installed identity and admitted external policy yield their distinct failure/block/managed states; none says Up to date or falls back to another updater. Pair with a successful current no-eligible-update result. | Updater/time/policy and UI owners; deterministic inputs plus actual qualified provider/management path. CHECK-3/14/18/25/27. |
| UP-UI-10 — disclosed consequences; AC-23/26 | Before Yes, inspect exact size, service/app/OS consequences and metered/roaming/policy restrictions. Change a material condition after Yes; stop and refresh consent or report policy denial. No silent direct/proxy-policy escape, unexpected substituted target or hidden additional product confirmation. | UI/transport/lifecycle owners; model and controlled networking, then Windows consequence observations. CHECK-3/13/14/18/22/25/27. |
| UP-UI-11 — hostile notes and critical copy; AC-26 | Signed oversized/control/markup/link notes remain bounded literal data or reject with safe text; they cause no fetch/action. A missing/lying/mixed translated offer falls back to the entire approved English group or remains unavailable. Language change and stale prepared text cannot change accepted terms. Retain boundary-sized healthy notes and approved complete groups. | Catalog/security/UI owner; actual decode/formatter/Slint consumers, native sink budgets later. CHECK-14/17/21/22/25/26. |
| UP-UI-12 — accessible, private result flow; AC-17/26 | At D1/D3 re-entry, complete Check/Yes/decline/Cancel/UAC/refusal/unknown/recovery using keyboard and actual Narrator/UIA, with long text, contrast and scaling. Meaningful progress is rate-limited; required terms/actions stay reachable. Inspect bounded redacted diagnostics; any later notice only navigates to current status within observer/permission scope. | P5 UI/accessibility, W11-W08/W09, native supported profiles; no current rich-toast or accessibility pass. CHECK-14/17/21/22. |

### Bound the About fixture's text-contract transition

The independent source review at `a4a3bb043fdb` found 509 active
catalog meanings and a 512-entry map limit. The first complete sample
needs separate meanings for the check, check progress, offer terms,
affirmative action, decline, download, verification, installation and
an explicitly simulated result. Count typed version, channel and size
projections and any needed labels before freezing the roster. Reuse the
existing About destination and sample marker. Use one in-memory offer
and at most one active preview operation. Bind acceptance to the
currently published offer generation and its exact fixture target and
terms. Cover Later, dismissed and superseded offers, repeated or forged
activation and a useful fresh acceptance through the receiving callback.

Retain schema 2 and advance the semantic text contract for the new
required meanings. Preserve the existing ordinals and both historical
generation fixtures. Select the new map ceiling using the exact roster.
A source calculation found 3,790,306 charged JSON work units for the
current English bytes, against 4,000,000. The exact successor must pass
the original work budget or justify a reviewed bounded decoder change.
Exercise the new ceiling plus one, decoded duplicate keys, the old
contract's refusal and complete new JSON/CBOR publication. Adding
FixtureCopy slots shifts later aliases, so regenerate and check the
whole immutable projection. Independent exact-copy approval remains
open and the release ledger stays empty. See the [text workflow](../docs/nevrotick-gui-text-development.md).

This is planning and independent source review for UP-UI-01/02/04/11.
The fixture owner still has to freeze the exact messages and facts,
implement the accepted state and bind it to About. This checkpoint
executes no fixture, decoder, native or live-updater acceptance case.

#### Freeze the bounded About preview inputs — 26 November 2026

For the first checked one-consent path, append thirteen `ui.update` text
meanings at ordinals 597–609. Use `check`, `checking`, `offer`, `from`,
`target`, `channel`, `size`, `confirm`, `decline`, `download`, `verify`,
`install` and `complete` in that order. Retain the sample marker and use a
dedicated updater offer group for the offer, its fact labels and both
actions. The offer makes the preview effects, decline and Windows restart
consequence explicit. It discloses the fixed sample assumptions of an
unmetered connection, no roaming and no external management. The completion
message is “Update simulation complete.” Keep schema 2, advance the text
contract to 3 and retain all original ordinals and historical fixtures.

The Rust owner publishes installed sample version `1.0.0`, offered sample
version `1.1.0`, the stable channel and `12,582,912` package bytes from the
same typed offer consumed by acceptance. The declaration binds material
terms and the About visit to one generation. Check starts one simulation.
Not now, dismissal and leaving About close the unaccepted offer. A fresh
check creates a distinct offer. The receiving callback admits the active
About visit and current unconsumed offer once, then drives the three simulated
progress states and completion. A repeated, stale or direct call outside that
state fails without progress, and a late callback cannot recreate consent.
The preview owns one pending in-memory operation and no production port.

The schema-shaped draft includes each text entry's required empty `args`
map. At 522 active meanings it projects to 66,672 JSON bytes, 43,442 decoded
text bytes, 6,852 lexical tokens, 1,583 fields, 1,584 value nodes and
3,962,233 charged JSON work units. Its modeled canonical CBOR size is
47,222 bytes. The draft fits the original work, size, depth and Unicode
contract with a 522-entry map ceiling. The evidence is source-linked arithmetic
for a draft whose fingerprint is deliberately unadmitted. The exact copy and
record are in `~/.local/share/nevrotick/about-preview-roster-2611.84fvbqeu/`.
`reviewed-roster-budget.json` hashes to
`baabd112745671fb8eebf60a6c992196fc0df3a61d10b9905b02116a967610d3`.

The code-change and localization workflows separated the prepared schema
from admitted text and traced the current About receiver. Independent review
confirmed the append-only range, budget arithmetic and requirement to bind
typed fact rows to the same offer. At implementation, challenge 523 entries,
an unknown ID substituted into exactly 522 entries, decoded duplicates and
incompatible contract 2 and 4 candidates. Verify complete new JSON/CBOR
publication, regenerated source sinks and approval behavior for the adopted
locale. The critical production approval ledger stays empty.

#### Implement the one-consent About simulation — 26 November 2026

The About buttons and Escape route forward requests to the UI-thread owner.
Check issues a fresh generation in the current About visit. Yes receives that
generation and consumes the immutable offer, including its fixed material
terms. Not now, Escape, accepted navigation away and closing to tray retire
unconfirmed work. Accepted simulations continue through download,
verification and installation while another page is open. One Slint timer
belongs to the current check or progress arm. Generation and phase guards
reject stale callbacks. The fixture scope permits simulated effects only.
[Receiver](../crates/nevrotick-ui/src/slint_host/update_workflow.rs)

At that checkpoint the catalog admitted 522 messages at schema 2 and text
contract 3. The checker traced all 522 meanings to their renderers or reached feedback sinks.
The offer and both choices use the same critical group. The four displayed
package facts come from the same typed offer consumed by Yes. The v2 CBOR
fixture remains available for the incompatible-version refusal control.

The guarded default-profile GNU UI run passed formatting, all-target compiler
and Clippy checks, rustdoc, 220 ordinary tests and 17 doctests on the 761-input
source identity
`4e5847a769aa7d49f894a6f41cc9864d161da4bd01ca3aee1c27576d85a61755`.
All five new tests exercised the generated Slint callbacks. They covered
wrong-page requests, stale and repeated generations, decline, navigation
before and after consent, and the actual Slint timer dispatcher on the
software platform. The checks exercised complete JSON and CBOR admission,
exact bounds, incompatible catalogs and the release-facing approval refusal.
The configured source controls passed 85 text coverage, 15 code generation,
41 native-text source and 16 native-input source checks. The 17 original
design-reference updater tests also passed.

The GNU receipt `gnu-ui-documented.json` is under
`~/.local/share/nevrotick/about-preview-qualification-2611.xhFSU2ry/`.
Its SHA-256 is
`49b7722f01a02b2fd96598c5bfd8a5eee84f5552202e946db457c8aac784f4bf`.
The companion `msvc-ui-retry.json` records the MSVC all-targets source
checks and dependency and tool identities in the same directory.

Native Windows rendering, keyboard and accessibility observations and
independent production approval of critical text remain open. Live checking
and installation belong to [W11 §4.40](windows11-integration.md#440-tuf-authenticated-updater-integration--25-november-2026).

### Readiness, skill application and retained gates

The documentation stage applied `plan-code-change` to reconcile the accepted
decision against current About/host/manifests; `implement-native-ui` and
`implement-slint` to preserve UI-thread/model/weak-callback ownership and UI-M0;
`security-native-ui` to separate intent from trusted effects and literal sinks;
`qa-test-planning` and `qa-state-concurrency-lifecycle` to define direct-receiver,
stale/late/unknown and real-completion controls; and `qa-i18n-localization` plus
`qa-accessibility` to route critical copy and native evidence. The concrete
outputs are the source map, behavior contract and UP-UI cases above. Reading or
writing these requirements proves no executed acceptance.

**Next owner/action:** S0.5/P5 retains the bounded fixture contract and affected
catalog messages within the [existing updater handoff](s0.5-ui-comparison.md#manual-updater-ui-handoff--25-november-2026),
following the normal queue. D2 freshness/currentness returns before live metadata
status and D2 policy/result/receiver rules before update commands. The live-apply
join needs both qualified trust/time/protected storage and a surviving native
deployment/recovery route, with signing/Windows trust prerequisites met. D1/D3
return before general-user preview/release or earlier claims under architecture §16.5.
Unresolved client/dependency, publisher/key operations, time, installed identity,
native route, recovery, critical-text approval, resources and release checks
remain open with their existing owners. Existing native, scoped fuzz and storage
holds remain. Native deployment and signing stay with their existing owners.

## Branch and integration state

**Merged continuation — 6 November 2026:** PR10 merged as
`3ff7e91c18f4c478b81c016cdefe31538dcf8086`; its final head was
`5bc22f5a884067f4dbee82d1073d94bc80a2a6c4`. The later numerical checkpoint
`5dcc465b534399ccdc4c9be58e3579a1df62c636` changes no GUI product source.
The [hosted reconciliation](#hosted-merge-reconciliation--6-november-2026)
below records the two GUI commits separately from that later workspace candidate.

The following bullets retain the historical PR-preparation environment:

- Worktree: `/workspace/NevroTick-slint`; branch: `ui/slint-gui`. The PR head
  identifies committed source. Main worktree `/workspace/NevroTick` was kept clean;
  Core, protocol, time-domain and mathematical/proof source were not edited by
  this GUI implementation.
- GUI work started at `d0e6ca21c12eca0d55fee5e536d46672c0792a9f`.
  That earlier local commit brings the v0.4 HTML prototype, UI guidance and
  localization design; it was not present on remote `main` when preparing this PR.
- Remote `main` observed during handoff:
  `b808eb74fd82f91f207fc03b9c890441a73af4fb`. Common ancestor:
  `8a66af924c509e898a9bab9bb3282f11b24fba93`. Main has newer engine, policy,
  metadata, platform and documentation work. The PR now integrates that main snapshot through a conflict-resolution merge.
  The ordinary combined-candidate checks are recorded below; native and release
  qualification remain separate.
- Owner-decision collisions are resolved: published OD-11 SOCKS5, OD-12 GeoIP
  and OD-14 platform scope stay intact. OD-13 now retains both Slint selections
  and the accepted GUI licence/composition amendment; localization is OD-15.
  Maintained routes use those canonical identities; old receipts keep their scope.
- Remote main also defers formal proof execution until after V1 under its OD-10
  amendment. Preserve that newer decision when integrating; this branch's older
  mandatory-proof wording is historical and must not restore a V1 proof blocker.
  The GUI work ran no formal compiler/solver/proof jobs.

## Implemented and verified within fixture scope

| Area | Delivered behavior |
| --- | --- |
| Slint shell | Windows entry selects Slint/Winit/software/Fluent. Simple has Status, Settings, About; Power adds Sources, Statistics, Diagnostics. About describes the program, author, fixed GitHub repository and component libraries. Original branded PNG and separate notices retained. |
| About update preview | [26 November implementation](#implement-the-one-consent-about-simulation--26-november-2026): bounded in-memory one-consent sample with typed offer/copy, guarded Check/Yes/Later callbacks and simulated progress. No live metadata, package, helper or installation effects; full UP-UI and native qualification remain open. |
| Source workflow | Single checked source owner; add/edit/remove/undo, stable identities, search, details and change review. Protocol/default-port/manual-port provenance stays explicit. Edits are local previews; Apply remains unavailable. |
| Settings and recovery | Grouped synchronization/service/notifications/time-policy/personal preferences. Custom interval accepts positive whole minutes with bounded raw/numeric admission. Policy and Roughtime/helper recovery previews preserve drafts and distinguish NTS, plain NTP, observe-only, provisional date and clock authority. |
| Status and diagnostics | Closed typed presentation separates authentication, agreement, freshness, service liveness and clock effects. Historical observations cannot show current Tracking. Diagnostics and correction review show unavailable/sample facts and safe navigation. |
| Shell/tray/toast source | UI-only quit warning preserves service continuity. Opt-in native tray and six fixed menu rows, explicit sample test notification, bounded activation token, current-context guards and acknowledged cleanup source are implemented. OS acceptance does not imply a visible/read notification. Native behavior remains unqualified. |
| Hostile input | Seven raw fields retain at most 5248 content bytes; original field caps precede retained allocation. Receivers independently check private route/phase/modal/current raw snapshot. Fixed redacted rejection preserves accepted state. Native keyboard/IME guards and reviewed AccessKit conversion/range admission are integrated; earlier OS/toolkit allocation remains outside those bounds. |
| Localization | Strict original UTF-8 JSON, duplicate/unknown-field rejection, closed schema, canonical bounded CBOR, Unicode/emoji policy, typed placeholders, literal sinks and atomic catalog publication. Missing/blank/orphan consumer checks run before Cargo. Catalogs cannot choose actions, severity, permissions, routes or URIs. |
| Dependencies/tooling | One pinned graph and schema3 admission policy; Windows-only Winit/AccessKit/WinRT/Shell features, narrow Linux software test profile, reviewed local `spin_on` and AccessKit patches, guarded verifier/captures and negative checker controls. No new Core runtime or broad execution exception. |

**Candidate text identities — 27 November:** schema 2, text contract 4, English
revision 4 and informational GUI build 2; 525 active messages. The
[protected contract](../crates/nevrotick-ui/src/localization/contract.rs) and
[English catalog](../crates/nevrotick-ui/locales/en.json) carry the exact current
roster and identity. Current fingerprint:
`8636c6e87302c241efa7060644df0734491f1bc96cd36acc3a61771749bc0c69`.
The [earlier text2 cohort](s0.5-ui-comparison.md#current-nativetext2-integration--gui30-and-later)
retains its historical 509 active IDs, 596 stable ordinals, 87 inactive identities,
426 immutable output rows and fingerprint
`34c97f8a74a9559ae998f55389bedd9f92bb3f878e72df84d090372053a69e80`.
Do not reuse retired ordinals or equate the GUI build number with compatibility.

GUI original code is GPL-3.0-only; inherited Core/workspace code remains
GPL-2.0-only. Preserve [GUI provenance/licensing](../crates/nevrotick-ui/LICENSING.md),
patch notices and separate brand/font/Unicode terms. The 9 November graph's 533 registry
identities include inert resolution entries; that historical number is not the current runtime
package count. Distribution obligations remain separate from private checks.

**Current source identity — 26 November 2026:** inspect
[`rust-toolchain.toml`](../rust-toolchain.toml), the
[`UI manifest`](../crates/nevrotick-ui/Cargo.toml), [`Cargo.lock`](../Cargo.lock)
and [`dependency policy`](../policy/ui-m0-dependencies.json) for the actual
candidate. The toolchain now selects Rust 1.100.0 and the UI still selects Slint
1.18.1. The 24 November calendar and URL changes in
[the build record](build-optimization.md#replace-chrono-in-the-selected-slint-desktop-graph)
supersede the graph identities below. Reconcile the exact current inputs and
their affected checks before replay; no historical GUI pass transfers by version
name alone.

**Historical graph identity — 9 November 2026:** Slint runtime/build pin:
1.18.1; Rust: 1.99.1. Recorded lock SHA256:
`fa9103e455e401c39f349c04b10a2d7058408d25210763321f56b7dad9c1afba`.
Recorded schema3 policy anchor:
`29f6a647b6038bdbbdc2bf407875a43af03257382821f13bda174e6147186b88`.
The 9 November readback matched that candidate's lock, policy lock binding and checker's
canonical JSON anchor. This was identity reconciliation, not a fresh GUI/native
qualification. The later [AccessKit callback/advisory repair](codeql-cloud-handoff.md#imported-full-repository-findings--9-november-2026)
records actual-module facade and final hosted portable/security passes at
`f2f0f52`; native USER32/UIA, resources and GUI critical-file metrics remain open.
The historical 7 November lock
`9f31370aaf3649527926acf48a927ce2b627d8de633d8f7a50b467cf2f07c24e`
and policy anchor
`c1735811cefab5f108906b44c40bb634a18bed3490a253e724bc429c4a6dc5cf`
retain their earlier scopes. That documentation/source reconciliation changed only the UI package
description and its bound manifest hash. It removed an unsupported unelevated-token
claim; versions, features, grants, package scopes, patches and the lock stayed fixed
within that historical amendment.
The prior anchor
`6cf46ae3b59889507f5e78942ea68cd68f0d9c7afe0169d4cd7f04caaa98488b`
identifies the [6 November dependency refresh](s0.1-source-readiness.md#refreshed-graph-and-current-evidence),
including seven transitive updates and the engine-scoped Tokio-macro Syn 3
closure. The policy anchor is the checker's canonical JSON digest, not the raw
file SHA256. A metadata amendment does not transfer historical execution receipts
to the changed candidate. Earlier lock `56833523c30ef64a3300fcc905d99039470fa1b88fc22ce295ada364bfd8a286`
and anchor `a3a008828e6d8c72b7441c29bafbc67f8aa22a4c9efddbb99e32bf705cfda3f6`
remain historical identities; their receipts are not transferred to the refresh.
That graph includes main's normal engine→time-domain edge and direct engine→garde
derive/defaults-off edge. Registry identities, features, grants, workspace and
non-engine package scopes retain their reviewed boundaries. The AccessKit
derivative pins include the range-lock repair below. The earlier GUI41–43
receipts retain the old graph and cannot qualify later candidates.

## Not implemented or not qualified

1. **Production backend:** no secured production UI IPC, coherent live status,
   service/clock/protected Apply, Windows Time handover, real incident producer,
   sign-in registration, persistence or diagnostic collection/export. Current
   fixture ports cannot become production authority through an implementation
   swap; restore D2 contracts and independent receiver authorization first.
   Real Critical/High notifications continue under the
   [W11-W09 incident handoff](windows11-integration.md#w11-w09-add-truthful-native-notifications-at-the-deferred-ui-stage),
   using the W11-W08 catalogue and existing incident design. Preserve the sample-only
   boundary, rich-toast release deferral and D2/D3/native prerequisites; completing
   the test notification does not complete that production continuation.
2. **Native Windows:** the retained 23 November
   [configured release-link checkpoint](#require-the-windows-desktop-image-load-policy-before-slint--23-november-2026)
   and [guarded startup/Close observation](#observe-guarded-slint-startup-and-caption-close--23-november-2026)
   record their exact admitted build inputs and elevated unpackaged Windows
   fixture. They do not qualify ordinary-user or installed activation.
   Those cases remain open, together with keyboard/IME cancellation,
   clipboard, COM/proxies, menu timing, tray/toast callbacks, UIA/Narrator, DPI,
   focus, shutdown and retained heap/handle/GDI/USER/task resources. Windows-target
   checking on Linux is not native qualification; the scoped native receipts do
   not close this wider matrix.
3. **Process/package hardening:** the desktop image-load guard has
   [cross-build and selected native policy evidence](#require-the-windows-desktop-image-load-policy-before-slint--23-november-2026).
   Full process-profile startup/readback, required-protection refusal, actual loader/activation
   paths and complete hardened direct MSIX qualification remain with their existing
   process/UI/package/release owners and applicable P0–P7 gates. OD-05's
   6 November clarification makes every applicable protection mandatory at the
   strongest supported compatible setting; unimplemented/untested is open work.
   The current Windows build selects v3 and the expanded hardening vector under
   [the existing build task](build-optimization.md#required-v3-and-maximum-compatible-hardening--6-november-2026).
   Do not copy headless Win32k/child restrictions into the GUI or add user-facing
   mitigation-disabling switches.
   `host_ready` means fixture binding readiness, not security readiness.
4. **Critical text approval:** production `REVIEWS` is deliberately empty. Exact
   critical English/catalog/bootstrap meaning needs independent human approval
   bound to content and contract. Test-only grants, signatures and parser success
   cannot approve production packs. Do not disable this gate to make an import work.
   The historical private bundle at
   `/tmp/nevrotick-localization-contract-v2/human-review-v1/` covers the former
   contract 2 roster of 509 active meanings, 87 inactive identities and eight
   separate bootstrap words. It is insufficient for current contract 3 approval,
   and its availability in the next cloud is not guaranteed. At the existing
   critical-content approval gate, prepare and verify a bundle against the
   [current protected contract](../crates/nevrotick-ui/src/localization/contract.rs),
   catalog and bootstrap bytes through the existing exact-content workflow,
   including the updater meanings. No current approval bundle or human approval
   is established by this handoff.
5. **Remaining polish/provenance:** scope the service administrator badge to
   changes rather than read-only inspection; explain the consumed session test
   identity after the first notification attempt. PR #10 review reconciled
   Unicode `derivation.json` to the current `repertoire.rs` hash after independently
   rederiving all 81 hard ranges and checking retained source identities and the
   English positive set. No Rust policy bytes or historical generator claims
   changed; see the review receipt below.

The existing S134/general native, separate fuzz no-retry and storage experiment
holds remain. D1/D2/D3 retain canonical re-entry triggers. This PR does not certify
ASVS/accessibility, full GUI completion, native cutover or release readiness.
GUI security-module coverage, critical-file mutation and native resource/soak
qualification are not established by the ordinary test or source-control counts.

## Evidence and replay

### Hosted merge reconciliation — 6 November 2026

Live GitHub checks were read at **14:15:59 UTC**; no alert or incident was
dismissed. The connected job-step reads confirm successful dependency admission
and isolated actual-product verification in both portable jobs.

| Exact candidate | Observed hosted result |
| --- | --- |
| Final PR head `5bc22f5` | [Portable](https://github.com/DavidOsipov/NevroTick/actions/runs/42228676978/job/127928626695), [Security gate](https://github.com/DavidOsipov/NevroTick/actions/runs/42228676593/job/127928980992) and [secret scan](https://github.com/DavidOsipov/NevroTick/actions/runs/42228676593/job/127928624233) passed. Default Actions/Rust/C++/Python analyses passed. Aggregate CodeQL and GitGuardian failed. |
| Merge `3ff7e91` | [Portable](https://github.com/DavidOsipov/NevroTick/actions/runs/42230373102/job/127934412937), [Security gate](https://github.com/DavidOsipov/NevroTick/actions/runs/42230373254/job/127934786357) and [secret scan](https://github.com/DavidOsipov/NevroTick/actions/runs/42230373254/job/127934414509) passed. Default Actions/C++/Python analyses passed; Rust remained in progress at the observation. |

The failed [PR CodeQL aggregate](https://github.com/DavidOsipov/NevroTick/runs/127928935171)
contains 11 Python path annotations and a missing-Rust-configuration warning.
It completed before the successful final-head Rust analysis, so that warning
does not establish a Rust extraction failure; the aggregate remains failed.
Alerts [#105](https://github.com/DavidOsipov/NevroTick/security/code-scanning/105)
and [#106](https://github.com/DavidOsipov/NevroTick/security/code-scanning/106)
remain open on merged main. They flag `lstat`/`os.open` in `scripts/ui_source_io.py`
through the same developer-selected root and fixed generator paths previously
reviewed below; earlier #95/#96 instances are marked fixed after those reads moved
into the shared helper. No new unauthorized path access was demonstrated under
that local CLI contract. External disposition remains open.

The failed [GitGuardian check](https://github.com/DavidOsipov/NevroTick/runs/127928604764)
still reports [incident 39281800](https://dashboard.gitguardian.com/workspace/311486/incidents/39281800?occurrence=310809530),
the public fixture regex at `.gitleaks.toml:55`, matching the earlier assessment.
The incident remains triggered. These individual passes do not make overall CI
uniformly green or qualify native Windows, GUI coverage/mutation, critical text,
backend, package or release behavior.

### Local source continuation — 6 November 2026

After the signed/pushed numerical checkpoint `5dcc465`, the local read-only
text-source gate passed with 509 active/509 used IDs, zero errors, schema/text
contract/build 2 and the unchanged fingerprint above. The selected
`test_ui_text_native` and `test_native_input_source` controls passed 53/53 under
`-X dev -Werror` in 28.943 seconds, with zero errors or failures. Raw log:
`/tmp/nevrotick-merged-gui-source-controls-20261106-v1.log`.
These source controls ran no Rust product, rendered GUI or native Windows path.
Hosted checks for `5dcc465` are not yet qualified; earlier hosted results remain
bound to their recorded final-head/merge commits. The ordinary fixture source
does not close backend, exact-text approval, coverage/mutation or native gates.

### Historical cloud receipts

Historical canonical receipts under private cloud directory
`/workspace/.nevrotick-cloud/slint-uia-fork.2020wssl/`:

| Receipt | Observed result / SHA256 |
| --- | --- |
| `gui41-integrated.json` | Passed format/compiler/Clippy/rustdoc; 736 ordinary tests, 164 doctests, 447 optimized tests. GUI: 183 tests/17 doctests; 143 pre-Cargo controls, 509/509 text consumers, 50 captures. SHA256 `badd0f7ee0cba45fc009065e2add507985b0ba22b35714ad51b13d71091c5dcc`. |
| `windows-source7.json` | Linux-hosted Windows-target all-target type checking and Clippy passed; no SDK/CRT/link/run. SHA256 `27f1e89885a69171fa45d9e225ca50e21cc286dd5fae4e9b922444bf8b027732`. |
| `gui38-dependencies.json` | Both-target graph/scanner checks passed at their admitted advisory snapshot. SHA256 `db98db71574953fe5a09b16c9f55a4e133214d3b9937d3ef2d57a62e9cab4f8c`. Advisory freshness must be re-established for a new dependency check. |

**Historical PR preparation:** `gui42-pr-portable.json` passed the complete configured
portable/capture row, with the same 736/164/447 test totals, 183 GUI tests,
17 GUI doctests,143 controls and50 captures. Receipt SHA256:
`62a286f37888cc0e3df83b7cb7920c6863ad114946d4c5a2c3c1a0b7e796a77e`.
Compared with GUI41, its366-path source map changes only the updated UI README.
Root and an independent reviewer rehashed both sets of actual PPM originals;
all50 are byte-identical, preserving the existing visual-review lineage.

After that run, PR hygiene removed one trailing ASCII space from
`scripts/check_ui_text_coverage.py` and two redundant EOF newline bytes from
`scripts/ui_text_bindings.py`. Those are the only final source-map deltas;
restoring those bytes reproduces GUI42's exact hashes. The affected source
controls were then rerun with the canonical admission/sandbox and no Cargo
selection: `gui43-pr-source-controls.json`, SHA256
`0ae7c2158c3556c13ed62e50e029f953412600bd18ccd9d5f28d1a88b397038c`,
passed all143 controls and509/509 coverage. Its365 admitted sources matched that
recorded candidate; the optional SDK policy belongs to the full GUI42 row.
GUI43 is a Python-only diagnostic, not another product/native test run.

**Historical combined candidate, GUI44:** this ordinary source checkpoint incorporated
main `b808eb7`. `gui44-merge-portable.json`, SHA256
`e72e4ba6f3e15aac80c15a7d3118b3cecd59e4405456e8913cf85a02398b2d0b`,
passes all configured build/Clippy/rustdoc,759 ordinary tests,174 doctests,
470 optimized tests,143 source controls and50 captures. GUI remains183 tests/
17 doctests; main's engine now has47 ordinary/47 optimized tests and12 doctests.
All368 source entries (367 product plus SDK policy) matched its recorded candidate.
Its 50 actual capture files were byte-identical to the reviewed GUI42 lineage.

Both-target dependency/scanner row `gui44-merge-dependencies.json` passes under a
new lock-bound public advisory snapshot, SHA256
`1f2dbe65f19447bfa15927d26939efabb89bbc628fd7298eecacb91b8e21f7b5`.
The separately captured18 metadata/tree observations and independent review admit
only the named engine/domain closure. No registry grant, patch pin or non-engine
scope was expanded. Focused inert tooling controls pass223 cases; the skill
checker admits65 leaves and its25 tests pass. Private Python receipt
`/tmp/nevrotick-merge-observation-v1/python-targeted.json`, SHA256
`8a43b9a5bff80339e431dd6e5b21c34bd568fb849ac146e5e9b633af353ff038`.

An optional broader Core campaign-control run is **incomplete** (exit137), with
inherited checked-in-campaign failures before termination. Its unchanged policy
binds158 files; main already selects168 and has11 stale bound hashes. The merged
product selects367. This is not a new Core runtime defect or a current campaign
pass. Eight affected inert Core registry-projection checks and five domain
identity checks pass; do not rebind historical campaign/proof metadata or resume
deferred formal work merely to clear that broader run.

S0.5 records exact later deltas, failed attempts and independently viewed image
lineage. GUI41's 50 captures matched the reviewed GUI40 set; GUI37's full 50-frame
set and two emergency frames were viewed separately. These are software-rendered
fixtures, not Windows screenshots. Handoff preparation updates the previously
stale UI README; account for that documentation input when comparing source maps.

Raw receipts, images, review reports and the human-review bundle live outside
Git. The listed `/workspace/.nevrotick-cloud` directory and private `/tmp` review
bundle/targeted receipt are absent in the current local checkout environment.
Their input maps and image bytes were not revalidated here. Preserve them through
an approved artifact channel or reproduce the affected configured checks. Never
manufacture a missing receipt or claim a new
test/native pass from this summary.

Use [configured tooling](../TOOLING.md#starting-implementation), not bare Cargo.
In this cloud, after validating installed inputs, the portable replay is:

```sh
python3 -B scripts/verify.py --lane portable --capture-slint \
  --vendor-dir /workspace/.nevrotick-cloud/slint-uia-fork.2020wssl/vendor \
  --scratch-root /workspace/.nevrotick-cloud/scratch \
  --toolchain-dir /workspace/.nevrotick-cloud/rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu \
  --nextest /workspace/.nevrotick-cloud/cargo/bin/cargo-nextest \
  --linux-sdk-dir /workspace/.nevrotick-cloud/fontconfig-native-sdk.astjd5d_/work/native \
  --output-dir /new/owned/external/artifacts \
  --json /new/owned/external/receipt.json
```

Paths are machine-specific, not installation instructions. The verifier checks
exact tools, graph, SDK and sources; it uses offline execution, cleared allowlisted
environment, read-only inputs, owned scratch and fixed resource/time bounds.
Keep acquisition separate, proxy/TLS intact, negative controls and actual test
counts visible. Missing prerequisites are incomplete evidence; do not relax pins,
limits, lints, graph roles or trust to obtain green output.

## Next work and dependencies

At the next applicable source/native checkpoints, carry the
[R4 build-diagnostic and tray-panic follow-ups](s0.5-ui-comparison.md#build-diagnostics-and-tray-panic-follow-ups--9-november-2026)
into their existing owners: optional cache-aware Slint warning denial, explicit
caught-payload fault policy, and narrow unwind assertions during a natural edit.
Their controls remain proposed; they add no current defect, release gate or
execution exception and do not displace the
[current development continuation](../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent).

### Desktop delivery hardening — 25 November 2026

A focused source/plan review at `6701df99fc6a34e0aa21d6f0258d220decf9d2f0`
retains the existing process-mitigation/token gates and finds the following narrow
follow-ups. These are planned source/evidence refinements, not runtime changes,
observed exploits or native qualification. Existing section 7 priorities,
UI-M0/D1–D3 triggers and the basic-release rich-toast deferral remain.

| Reached owner / checkpoint | Bounded follow-up and exit |
| --- | --- |
| Next applicable tray-source checkpoint; S0.5, CHECK-21/27 | Apply the [TaskbarCreated recovery budget](../docs/nevrotick-gui-native-input-residuals.md#tray-recovery-admission--25-november-2026) before native registration work. Pair legitimate later Explorer recovery with repeated hints and stop/cleanup pressure; downstream wake coalescing alone is insufficient. |
| Next selected native-notification boundary edit | Establish the [explicit mirroring policy](windows11-integration.md#w11-w09-add-truthful-native-notifications-at-the-deferred-ui-stage) before sample submission, preserving no-Show preparation failure and ordinary GUI usability. Carry existing H10's type-level CloseOutcome must-use follow-up through wrappers in the same natural boundary edit; no new history/cold-start adapter is implied. |
| Production incident/delivery join; W11-W09, P5/package owners | Freeze bounded observer coordination and selected activation entry; qualify exact owned history retirement separately from Hide/callback detachment. Late old-instance cleanup must preserve successor/unrelated notices. Existing rich-toast sequencing remains. |
| Windows logger and future diagnostic Copy; W11-W08/S5.3/ST-W07 | Retain [actual native status and conditional cross-session evidence](windows11-integration.md#native-delivery-refinements--25-november-2026). At the future Copy adapter, establish [history/cloud exclusions](../docs/windows-time-architecture.md#1234-privacy-sinks-and-resource-limits) before redacted text publication and report partial failures honestly. Current diagnostic actions remain unavailable. |

The existing dependency sequence continues below; this amendment starts no held
native, service, package or production-UI execution.

#### Bound tray recovery at the Windows worker

S0.5 and CHECK-21/27. The worker's one explicit initial registration
comes after creation of the window and icon. Later restart hints and
actual failed Modify operations can leave one pending recovery. A raw
control wake or a projection change while unregistered cannot create or
refill that intent. The existing 100 ms owner poll wakes the worker for
an unacknowledged projection or a due recovery. The original worker
resamples its own time and authority before submitting native work.

The provisional source policy allows two attempts in a fixed half-open
two-second window. A successful attempt preserves that window and budget.
The next attempt starts at least 500 ms after the preceding recovery
sequence returned, including across window boundaries. This bounds each
window to four positive registration calls. Each Add and SetVersion
checks the original cutoff and current Stop and loss. A call already
entered can return later. It acknowledges only while Stop and loss still
match. A later independent hint can open a fresh expired window. The
ordinary window remains reachable when registration is unavailable.

A newer hint during a publication retains one successor and invalidates
the older acknowledgement. Hints inside a popup defer native recovery to
its return. An update asks to dismiss a popup once. The actual Stop has
its own one-shot exit and still retains cleanup custody through join,
including Delete after a possibly effective earlier Add. Recovery uses
the existing native worker, window and UI poll.

The extracted portable check ran the exact recovery state and shared
registration sequence with the five new tests on Rust 1.100.0. Four
isolated variants each reached the intended failure after removing the
charge, spacing from actual completion, between-call check or final
acknowledgement check. The first harness link stopped before execution
because its working directory was read-only. The retry used owned
scratch and the process-local core limit. That receipt is
`state-check-final/result.json`, SHA-256
`65bb2100e71ffdf1fc5c794cca218fbe8823dfadda146662e1261d931773c868`.
The corrected test contracts and checked boundary arithmetic passed the
same baseline and four fault controls again. Evidence under
`~/.local/share/nevrotick/tray-recovery-2611.ryFoYvBI/state-check-lint/`
is bound by `result.json` SHA-256
`a94465949504ea765c8b86879feddd51e420df98ba51b9afd7279923ad70c3e7`.
The earlier configured Windows run passed the compiler and stopped at
Clippy on documentation, test time arithmetic, naming and the tray
pump's function length. Its `windows-current.json` SHA-256 is
`4d120d9006fba3d80d58666c4a83401e17d1995ba28a37373731b49eb2bb6d94`.
The repaired candidate passed all 35 configured MSVC lanes with 772
bound source inputs, including strict Clippy, both x64 release links
and the VERSIONINFO and static import readbacks. The retained UI image
SHA-256 is `bf4b30a76411f963b755eeef5bf52eb148aac8b8a09beef15837e5436576d9ed`.
`windows-checked.json` SHA-256 is
`aff1f70ce6f7571b339bcad78ca21ff5063a291ad04c4dfcbbbd30796e9083e1`.
The MSIX inspection retained `assembly-inputs-incomplete`.
The GNU run passed compilation, strict Clippy, all 214 ordinary UI
tests including the five tray controls, and all 17 UI doctests. The
independent roster readback matched the three tray source hashes in
both configured receipts. The optimized Engine cohort finished 312/313.
Its `public_managed_round_live_endpoint_refusal_preserves_history_and_later_source_use`
case produced `Exhausted` where the fixture expected `Control` at
[the first round assertion](../crates/nevrotick-engine/tests/private/source_policy.rs#L335).
The same case passed in the ordinary cohort. The GNU receipt remains incomplete
at 42 of 43 executed lanes. Its fail-fast exit left the final
`windows-platform-native` test and doctest rows unexecuted.
`portable-checked.json` SHA-256 is
`c91e937d28c4fd3059aeaf4d90005ae8cd36484339f28565e3c6826998297841`.
The first isolated launch stopped before test execution because nextest's
nested offline Cargo invocation lacked the vendor configuration. Its
`repeat-engine.json` SHA-256 is
`1998a3b3ff52680e9e6aa6996eda4449626dfea4fb72be2a7d09ff601b09f62d`.
The corrected launch used the configured private Cargo home and nextest
store and selected the exact case with one test thread. It passed five
times, with individual test times from 56 to 70 ms. The retained
`engine-isolated/repeat-engine.json` SHA-256 is
`7438d0549b537eb745fab79cf0f647f0aadfc1fe98b5b6a65968b40de55955eb`.
The original parallel cohort remains incomplete.

The [Engine contention diagnostic](engine-transport-pilot.md#diagnose-loss-of-a-selected-managed-reply-on-release--26-november-2026)
now owns the follow-up. The source review identified both a deadline path
and a policy-custody path consistent with the original output.

This scope leaves actual Explorer/display-change behavior, native
call counts, popup reentry and disable/Quit cleanup for Windows evidence.
The earlier elevated Slint startup and caption-Close observer does not
exercise the default-off tray. Start the native tray cohort with an admitted
ordinary-user token and an observer for enable, hide, restore and cleanup
on the original process before adding shell restarts and popup reentry.
Observe the tray worker and window retiring and measure GUI objects and
kernel handles before disposing of the guest. Application Verifier and
debugger coverage of the changed Shell/FFI paths remains pending.

1. Resume from the actual checkout and
   [development section 7](../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent),
   whose current source child connects the consume-once admission handoff for
   original joined receipts after the completed laboratory timing pair.
   Physical-reference qualification stays HELD/BLOCKED under the W11 changed-input
   re-entry rule; no device inventory or procurement is a GUI-source prerequisite.
   PR10 and the numerical checkpoint above
   retain their dated evidence; they are not the current checkout or next task.
   Preserve the canonical decision identities. Keep
   lock, manifest and graph policy changes with one owner; re-admit affected
   graphs before target execution.
2. Preserve the local text/native-source-control pass on `5dcc465`, then select
   the next configured product checkpoint against its frozen inputs.
   Reuse unchanged evidence only after matching all relevant inputs; historical
   GUI42/43/44 counts do not establish results for later workspace bytes. Retain
   the two named wording-polish follow-ups above for a separate text revision.
   At that localization checkpoint, consider the [R3-P-01/P-02 source experiments](s0.5-ui-comparison.md#localization-cost-experiments--9-november-2026)
   with their approval, invariant-failure and notification controls; they are
   unexecuted proposals and do not displace that current development continuation.
3. Complete the independent exact-text review; revalidate any wording changes and
   their native/action/accessibility consumers. The supplied review bundle is
   review material, not approval.
4. Resume the remaining native qualification with an exact admitted fixture GUI
   once its existing Windows holds/prerequisites are resolved. Reuse the recorded
   scoped observations only while their relevant inputs match. Exercise the
   remaining lifecycle/resource
   matrix; restore D1/D3 only at their triggers. Neither Linux captures nor
   Windows metadata checks replace those observations.
5. In parallel where prerequisites permit, prepare authenticated status/incident
   producers and narrow service/helper requests. Activate live adapters only at
   D2 using the existing hardening table. Preserve outcome-unknown, privacy,
   current-generation and actual-completion semantics; repeat affected native
   production journeys after integration. Final package, mitigation and release
   gates remain with their existing owners.

PR10 is merged. This continuation authorizes no deployment, Store submission or
OS/service changes; existing Windows/fuzz/storage holds and D1–D3 triggers remain.

## PR #10 detection review and corrections — 6 November 2026

Reviewed PR head `a3640f83b7f97f1062d2adb2fb72f4391699d031` against main
`b808eb74fd82f91f207fc03b9c890441a73af4fb`, including all 11 inline CodeQL
findings, the GitGuardian/Socket reports and failed hosted checks. Corrections
were prepared in the isolated `codex/pr10-audit` worktree. The concurrent staged
Core work in the main checkout was outside this task and was not modified.

Scope: ordinary source/tooling repair under OD-13/15 and CHECK-19/20/22/24/25/26;
explicit fixture/native/release limits remain. Expected results were bounded
file rejection before blocking, exact admitted archive acceptance without a
general limit increase, useful visible preview fixtures, and secret detection
outside exact independently checked public-data lines. No formal execution,
proof-consumer repair, Windows execution or fuzz campaign was selected.

### Reported detections

| Detection | Assessment and disposition |
| --- | --- |
| CodeQL #94, `check_structure.py:12` | The developer explicitly selects the local rendered-fixture file. This reference-only checker is not a server or privileged reader; no lower-trust filename is appended to a protected directory. No unauthorized path traversal was established. No scanner suppression or behavioral change. |
| CodeQL #95–98, generator input/root operations | The CLI selects the repository; the contract/template paths are fixed, symlinks are rejected and complete source identities are checked before code emission. No demonstrated path-injection exploit. A distinct regular-file replacement race was reproduced and repaired below. |
| CodeQL #99–104, generator output operations | The CLI selects an external output parent; the generator uses a new private directory, a closed basename set and exclusive no-follow creation. Cleanup targets its own new directory. No demonstrated path-injection exploit. No output-policy weakening or suppression. |
| GitGuardian incident 39281800 | The reported `.gitleaks.toml:55` line is an exact regex for a public unauthenticated u64 test fixture, identical to the reviewed main bytes. It is not a credential; no rotation or history rewrite is warranted. External incident state was not changed. |
| Gitleaks retained-history/working-tree failure | Both matches are public source digests: native-input Rust token content and FreeType `ftmodapi.h`. The first was recomputed from current Rust; the second exactly matches the public [FreeType VER-2-13-3 header](https://github.com/freetype/freetype/blob/VER-2-13-3/include/freetype/ftmodapi.h). Added rule/path/full-line exceptions with adjacent-secret, same-line-secret and wrong-path controls. Both local scans pass. |
| Socket reports | Hosted checks report no new dependency alerts. Scores and successful report generation are not independent licence/advisory qualification; exact graph and existing distribution gates remain. |
| Portable hosted job | Preparation rejects the newly selected `windows 0.62.2` source archive at the old 80 MiB expansion ceiling. Confirmed on the exact cached archive and corrected as below. The local repair is not a hosted build pass. |

These path-warning conclusions apply to current explicit local CLI use and fixed
source/output contracts. A future remotely driven or privileged caller needs a
new assessment. GitHub alert/incident dispositions were not silently changed;
the PR's historical red check results remain distinct from the local corrections.
The initial missing Rust CodeQL configuration warning cleared when its default
setup Rust analysis finished successfully; it reported no additional inline
finding in this review snapshot. Prepared custom CodeQL lanes remain deferred.

### Confirmed corrections

- **PR10-R1 — source-reading denial of service:** replacing a checked regular
  file with a FIFO blocked codegen, protected JSON reading, source coverage
  snapshotting and native-input source reading before their checks could reject
  it. Healthy regular-file controls pass; new regressions against the original
  sources fail by bounded child timeout. All four readers now share
  `ui_source_io.read_regular`: nonblocking/no-follow open, descriptor type and
  identity checks, bounded bytes and post-read custody checks. Existing root,
  count and aggregate guards remain. The helper enters the verifier source and
  GUI-tool licence inventories; semantic oracles remain separately owned.
- **PR10-R2 — fresh dependency preparation fails:** the exact `windows 0.62.2`
  archive has 114,755,072 tar bytes, 114,212,919 file bytes and 701 regular files.
  Its archive and file-map identities match policy. Admission now uses those
  measured limits only for that complete four-field identity. Default limits,
  aggregate limits, member/type/path checks and final source identity remain.
  The real cached archive changes from rejection to successful bounded parsing;
  synthetic controls reject altered identities and exceeded ceilings.
- **PR10-R3 — standalone source preview is stale:** its three rows did not reach
  `visible-sources`, and the migrated status projection was empty. The preview
  now binds visible/selected fixture state and existing status copy while keeping
  `host-ready: false`. Source review only; no new compiled/rendered preview pass.
- **PR10-R4 — current design instructions point at the wrong decision:** corrected
  the v0.4 AGENTS/README UI links from the former unpublished OD-11 to canonical
  OD-13. Updated only their current manifest entries; imported receipts remain.
- **PR10-R5 — current Unicode provenance hash is stale:** reconciled the Rust
  hash after independent checks of all six source identities, the 81 derived
  hard-denial intervals, the positive English set and its disjointness/assigned
  noncontrol properties. No Rust/table bytes or historical generator claims
  changed. This is data provenance, not resumed formal-proof work.

### Independent review and executed checks

All three workers used fresh contexts (`fork_turns: none`) and the named PR/base,
current contracts and explicit read/write ownership. Their results completed:

| Reviewer | Inspected scope and result |
| --- | --- |
| `pr10_rust_contract_review` | Portable Rust model/domain owners, handwritten localization/parsers/formatters/approval, catalog projection, all capture source and selected integration/binding controls. Independently matched 509 active specifications, 596 ordinals, 426 output slots and the exact 46,303-byte CBOR fixture. No new Rust defect confirmed. Subsequently reviewed the final correction delta and ran seven focused controls successfully. |
| `pr10_native_slint_review` | Active Windows tray/toast/input/lifecycle and host paths, modified AccessKit and complete spin_on repair, maintained Slint pages, HTML build/app and selected model/view/test flows. Found/fixed R3 and independently checked R5. Embedded HTML JavaScript/CSS and CSP hash agree. No further actionable defect confirmed. |
| `pr10_ci_triage` | Dependency/schema/graph admission, archive identity, patch custody, verifier command/results, CI and changed Core/domain projections. Reproduced/fixed R2, reproduced sibling R1 readers and identified the two secret-scan matches. No formal consumers were executed or repaired. |

Executed Linux Python checks: 14 generator, 78 text-coverage, 14 native-input,
26 acquisition, 118 verifier, 28 dependency-profile and eight SDK controls pass;
24 security-helper tests and 25 skill-checker tests pass. The skill inventory
admits 65 leaves. The real source gate covers 509/509 active text consumers with
zero errors; all 183 current design-manifest entries match. The pinned secret
scanner passes its healthy/fault/restored/disposition controls and both retained
history and working-tree scans. Both workflows pass actionlint, ShellCheck and
pedantic offline zizmor with two healthy and three rejection controls. The focused reviewer controls are corroboration,
not additional product coverage. Earlier failed original-source regressions are
retained as the causal basis, not counted as healthy passes.

Coverage limits: not every assertion, historical screenshot or unchanged vendored
upstream line was audited. The HTML protocol switch's handling of manually entered
default ports has no located explicit v0.4 preservation contract and was not
promoted to a confirmed defect. No Cargo/native/rendered product replay, current
full dependency/advisory lane, critical-text approval or package/release result
follows. Historical GUI44 execution receipts retain their original input maps;
new source/checker bytes require fresh admission at the next configured product
checkpoint. The archive and secret repairs do not themselves clear hosted checks.

### Final documentation and hosted-build follow-up

Corrected the eight production JSON seed/visitor `# Errors` comments that
described test-control preparation. They now identify actual decoding/type,
integer-range, text-policy, duplicate-key and shared resource-budget failures.
Independent source review confirmed that executable Rust is unchanged; the
509/509 text-source gate still passes with the same semantic fingerprint.

At `9606631`, hosted secret scanning and the security gate pass. The required
portable job then reaches compilation but cannot find `fontconfig.pc`. Its
ordinary host-native route already uses read-only OS mounts; it does not select
the optional, separately pinned Debian SDK. The workflow now installs Ubuntu's
authenticated `pkg-config`/`libfontconfig-dev` host prerequisites and checks
Fontconfig availability with a cleared environment. Product graph,
compiler and optional SDK pins remain. A fresh complete hosted portable result
was then required before merge; the later final-head and merge successes are
recorded above. This prerequisite repair supplied no Windows or packaged-product
qualification.

The workflow correction passes 24 security-helper tests, eight SDK controls and
the pinned actionlint/ShellCheck/pedantic offline zizmor checks over both
workflows, including two healthy and three rejection controls.

The first follow-up run installed Fontconfig successfully but the additional
partial-filesystem sandbox probe exited 139 before any product checks. Restored
the previously passing full read-only-root sandbox capability smoke check and
kept Fontconfig availability as a separate cleared-environment host preflight;
the ordinary verifier still owns its complete sandbox and product execution.
The setup failure remains a failed run, not evidence of product behavior.

## Rust data-flow audit and text-range lock repair — 6 November 2026

This bounded follow-up applies CHECK-19/20/24/25 and preserves CHECK-22's native
qualification boundary. It uses `qa-data-flow-analysis`, the CodeQL reference,
`qa-subagent-dispatch` and the concurrency/lifecycle skill. No formal, fuzz or
Windows product execution is authorized by this receipt.

**Custody and candidates.** Main was actively owned by another AI. The reviewed
snapshot is committed base `268598191a3a50a41bd51849daa9d0dfda2969b5` plus the
15 then-current uncommitted files, copied consistently into the isolated
`codeql-work/fixtures/rust-audit-20261106` worktree. Their SHA-256 identities are
in the external `codeql-work/results/rust-audit-20261106/candidate.json` receipt.
The repair is separately prepared on `codex/rust-dataflow-fix-20261106` in
`/home/david_osipov/NevroTick-rust-dataflow-fix`, based on that same committed
base. It does not include the other AI's dirty engine/policy work. The main
checkout/index was not edited, staged, committed, pulled or pushed by this audit.

### Reviewed flows and independent review

Three fresh-context workers performed complementary source reviews; all completed.
`rust_flow_protocol` covered wire/KE framing, request encoding, association receipt
and era/arithmetic/rounding. `rust_flow_engine_core` covered the real transport,
measurement/CLI outputs and Core history, principal/combination transaction,
readiness and modeled correction/rate receiving seams. `rust_flow_ui` covered
raw drafts, source submission, catalog publication, clipboard and native callback
consumers, then the selected AccessKit derivative. Their inspections were
read-only; they did not independently execute product tests or CodeQL.

- Packet bytes reach inert wire fields and checked numerical outputs. Association
  receipt refusal precedes pending-slot consumption. The engine has its own
  direct framing/measurement path: association guards are not barriers for it.
  The current engine has no runtime Core connection; observations remain
  unauthenticated and ineligible. No new defect was confirmed in these paths.
- Raw UI text reaches private bounded `RawText` construction only after field
  limits and character admission. Source edits check the current raw snapshot,
  route/modal/editor context and checked source construction. JSON/CBOR catalog
  admission is bounded/strict, with approval required before atomic publication.
  No new bypass was demonstrated in these paths. These source reviews do not
  close native clipboard, marshaling, receiving-service or production gates.
- A same-range COM comparison reaches the range's state lock twice, while range
  mutations can wait for that lock exclusively. This inherited dependency defect
  is corrected below. `rust_flow_protocol` independently reviewed the integrated
  repair and found no additional actionable defect, including in COM error
  ordering, distinct-state snapshot semantics, guard cleanup and regression scope.

### Confirmed inherited range-state deadlock

**7 November supersession:** this section records the earlier alias-only repair.
Its retained left-then-right contract for distinct owners is superseded by the
[distinct-range repair](#distinct-range-lock-order-repair--7-november-2026)
below. Historical tests and candidate identities retain their original scope.

In the retained `accesskit_windows` source, `Compare` could take two read guards
for the same state when both interfaces identify one range. `CompareEndpoints`
explicitly recognized that identity and still acquired the same lock twice. A
writer queued between those reads blocks the second read; the first read prevents
that writer completing. A waiting range mutation can also retain a tree read
guard, delaying later tree updates. The pinned Rust 1.99.1 source selects the same
writer-preferring futex lock for Linux and the selected ordinary Windows target.
This is the interleaving documented by [Rust's RwLock contract](https://doc.rust-lang.org/std/sync/struct.RwLock.html).
The pinned COM implementation advertises agility; apartment serialization is not
an established exclusion. No actual Windows deadlock was executed.

Both native consumers now use the same small `range_state::with_read_pair`
helper. Identical state owners provide one guard to both comparison operands;
distinct owners retain both guards throughout comparison and the previous
left-then-right order. Fallible implementation casting, context checks, invalid
endpoint errors and comparison of old weak endpoints remain at their original
owners. Acquisition failures skip comparison and release earlier guards. Native
callers preserve the existing poison-failure policy. No new COM or action
authority is supplied by the helper.

The exact helper is included by the existing GUI AccessKit ingress test. The
queued-writer regression uses real shared locks: it holds the first read, starts
a writer and observes reader exclusion with a bounded `try_read` probe. An
erroneous second acquisition returns a failure instead of hanging the runner.
An isolated helper variant retaining the original two-read sequence compiled and
passed its healthy comparison control, then failed this regression with two
acquisitions instead of one (exit 101). The repaired helper passes that same case.
This is behavioral evidence for the shared acquisition algorithm, not a native
COM reproduction or an execution of the complete original Windows methods.

Source/licence admission now accounts for the one new original GPL-3.0-only
helper and 27 derivative files. The original AccessKit revision, archive and
source-map identities, licences, package graph, features and Windows-only
normal dependency role remain fixed. The independent missing-helper rejection
and existing preserved-source/grant rejection controls remain active.

### CodeQL observations and limits

Local admission passed for CLI 2.27.3 and `codeql/rust-all` 0.2.24. A fresh
`/work/databases/rust-audit-20261106` database extracted the frozen dirty candidate.
The function inventory selected 2,983 functions in 95 source files across all six
product crates, including test/inactive-code scope rather than an exclusively
production denominator. A separate endpoint inventory returned 20 functions;
archived measurement, raw-draft and AccessKit text source bytes matched the frozen
files. The default extraction is the host/all-features/test-cfg configuration,
with dependency signatures, not a Windows or full dependency-body qualification.
Toolchain-discovery and macro-expansion warnings remain coverage gaps.

The existing interprocedural control query was re-executed on its owned fixture:
the positive input-to-sink route through `relay` produced one row and the constant
clean caller produced none. This qualifies that control, not product analysis.
The product global selector query exceeded its request budget and closed the MCP
transport. After reconnecting through the same guarded launcher, a narrower
same-callable selector diagnostic compiled but execution was killed (exit 137;
cause not established). It produced no admitted selector/path results. Further
local-flow work was interrupted after that failed prerequisite. A second offline
extraction with an independently admitted copy of the 534-package vendor cache
also hit the unchanged 600-second deadline and remains unusable. The temporary
CodeQL Cargo configuration was restored; tool budgets/policies were not weakened.
These incomplete stages establish neither absent flow nor absence of defects.

Separately, GitHub's current hosted analysis **2112729222** scans the exact
committed base `268598191a3a50a41bd51849daa9d0dfda2969b5`, with CodeQL **2.27.4**.
Its SARIF has **16 Rust results**, all critically inspected against six
byte-identical frozen/base files. The 15 dirty snapshot files are outside that
hosted candidate. The live API reports zero open Rust alerts, which describes
alert disposition and does not erase those results or establish a clean scan.
The connector lacks scanning-report reads, so the read-only GitHub API supplied
the analysis/SARIF; no alert disposition or suppression was changed.

| Hosted results | Current source-to-sink assessment |
| --- | --- |
| Two AES-SIV hard-coded keys | Public known-answer material inside unit/integration tests; the actual staged API takes its caller's key. No product graph edge to these separate tool workspaces. |
| CNG AEAD zero-initialized nonce | Test-only Wycheproof storage; all 12 bytes are overwritten from the case nonce before encryption/decryption. |
| CNG TLS 1.2 zero-initialized nonce | Both contiguous nonce spans are overwritten from the session IV and checked record nonce before `open`; TLS 1.2 is also disabled in the declared lifecycle cohort. No constant nonce reaches the reported sink. |
| Four state-guidance path results | All are inside `#[cfg(test)]`, from private temporary storage to an exclusively created fixed-name/PID witness and assertion reads. The configured runner enforces private temporary storage; they are not CLI input paths. |
| Eight engine runtime-boundary path results | Test-only private temporary capture directory; creation must succeed exclusively before ownership, stdout/stderr names are fixed, reads are bounded after child exit and cleanup deletes only those owned files/directory. No peer-provided path or authority crossing was demonstrated. |

No actionable vulnerability was confirmed from these 16 detections under their
current scopes. The independent source/lock review found the inherited range
comparison defect above; it was not a CodeQL-reported discovery. Hosted analysis,
local inventories, fixture controls, failed product flow queries and behavioral
repair evidence remain separate. Raw SARIF, comparisons, query hashes and BQRS
are retained in the external evidence directory. On 7 November, authorized WSL
cleanup retired six verbose evaluator/tuple/performance traces from the three
failed or interrupted `LocalSelectors`, `selectors` and `LocalFlow` jobs. Their
commands, timestamps, all 89 warning lines and failure context remain in
`dependency-refresh-20261106-v1/q1-space-preflight-v2/codeql-traces.json` under
the same external NevroTick evidence store, with deletion results in that
directory's `execution.json`. Unique source, queries, meaningful failure receipts,
successful result artifacts and hosted SARIF were preserved. Per-predicate and
performance replay detail was discarded; no failed job became a flow result or
qualification pass. Active CodeQL services and their caches were not changed.

### Executed verification and remaining work

The admitted pinned compiler ran the exact existing GUI ingress integration-test
source directly as a focused test binary in the configured read-only, offline
product sandbox. All 11 tests passed (three range-state and eight existing value
admission tests), with no ignored cases. The same source passed Clippy with every
current workspace Rust/Clippy deny rule. This focused direct compilation supplies
neither a full Cargo workspace pass nor Windows target/COM qualification.
Python controls passed: 14 AccessKit path-patch, 28 Slint dependency-profile and
118 product-verifier tests. Rust formatting, exact canonical policy admission,
all 27 derivative-file hashes and `git diff --check` passed. The UI text gate
also passed 509/509 active consumers with its unchanged semantic fingerprint.

Retained evidence is outside the skills pack under
`/home/david_osipov/.local/share/nevrotick/codeql-work/results/rust-audit-20261106/`:
`range-checks/report.json` records command/tool/source identities and the
RED/healthy/GREEN results; individual raw outputs remain beside it. The patch
notice records the helper's purpose, grants and native limitations.

The main AI should integrate only this bounded repair, reconcile the existing
`policy/ui-m0-dependencies.json` derivative hashes and the canonical policy anchor
with its concurrent engine graph edits, then run the affected integrated Cargo
and Windows cross-compilation rows. Native self-comparison/concurrent-mutation
and continued tree-update cases remain in CHECK-22's existing qualified Windows
lane. This audit lifts no Windows, fuzz, storage, GUI-integration or release hold.

### Commit integration — 6 November 2026

The owner requested committing this repair into main. Integration is based on
`cb223c57efb292f3038fc1fd84e1666dcd490a5a`, which already commits the separate nominal-engine
measurement work. Only the AccessKit derivative inventory/allowlist and the
canonical policy anchor are reconciled with that newer dependency policy; its
engine edges, packages, features, grants and manifest/lock identities are retained.
The initial audit receipts above retain their original candidates and limits.
Further local CodeQL analysis is a separate continuation, not a retrospective
pass for the failed product queries. No native Windows or formal execution resumes.

### Cloud continuation — 6 November 2026

The owner stopped local analysis because resources were low. The reviewed runner
corrections and portable queries are carried into active main; the other AI's
rational-seconds/engine work is preserved separately. The
[cloud handoff](codeql-cloud-handoff.md) is the next execution entry point.
Local product extraction/import and all twelve endpoint expectations passed;
the local data-flow evaluation ended with disk exhaustion, and global/negative
query execution remains incomplete. No completed-flow, native or release pass is
established by these setup results. Do not restart local analysis from this note.

## AccessKit password-disclosure applicability — 6 November 2026

The dependency refresh independently reviewed AccessKit
[PR841](https://github.com/AccessKit/accesskit/pull/841) and exact commit
[`d62a49614ffbb0d85ce2d34e0340df711eb86e46`](https://github.com/AccessKit/accesskit/commit/d62a49614ffbb0d85ce2d34e0340df711eb86e46),
included in upstream `accesskit_windows 0.35.2`/`accesskit_consumer 0.39.2`.
The substantive safeguards are in the consumer: mask unmasked password values
and text ranges, suppress word starts and normalize character metrics. The
Windows adapter change itself only replaces its password-role predicate.
Upstream still assigns the toolkit responsibility for supplying masked text;
its repeated-character heuristic is not a complete secrecy guarantee.

**Current disposition: no concrete affected NevroTick password path found.**
The selected Windows entrypoint runs the compiled Fluent Slint UI. Its seven
code-owned editable fields are source label, host, port, source search, custom
minutes and two helper identities
([field owner](../crates/nevrotick-ui/src/ui_drafts.rs)). All use
[GuardedLineEdit](../crates/nevrotick-ui/ui/guarded-line-edit.slint), which exposes
no input-type selector and leaves the stock `InputType` at its Text default.
Inspection of all 33 maintained `.slint` sources found no password-mode binding
or password/credential control. Input/catalog text cannot choose that code-owned
control type. The existing generated `app.rs` contains stock password-icon
branches, which does not establish an application password field; it was
inspected without rebuilding or claiming a fresh source-to-generated binding.

The Slint 1.18.1 AccessKit emitter source also overwrites a text input's value
with its display-derived text before layout lookup and derives text runs from
that displayed layout; `RenderString` masks text in password mode. This is
source evidence for toolkit-side masking, not native leakage qualification.
The older selected consumer 0.38.0 still writes supplied values/text runs without
the new defensive masking. Our local Windows adapter's value getter calls that
consumer, and its text-range `GetText` calls the consumer's range writer. The
existing local SetValue admission, interface-type and range-lock repairs address
different paths and must not be described as password-output protection.

Keep the present local 0.34.0 patch. Selected `accesskit_winit 0.33.2` requires
Windows `^0.34.0` and AccessKit `^0.24.1`; Slint 1.18.1's core also selects
AccessKit `^0.24`. Upstream Windows adapter 0.35.2 instead requires AccessKit 0.25.2,
consumer 0.39.2 and hashbrown 0.17. There is no newer 0.34.x release. Substituting
0.35.2 is an incompatible family/source migration, and that source still lacks
the independent NevroTick ingress/type/lock repairs. This review selects no bump
or backport and changes no licence grant: original GUI/added local helpers retain
GPL-3.0-only; upstream dual MIT/Apache material and additional notices retain
their existing separately selected terms.

**Trigger and owner:** before the GUI owner introduces a password/credential
control, reveal mode, or service-secret value/text-run producer, reconcile
toolkit and consumer masking at that actual producer under existing GUI security,
CHECK-22 and privacy owners. Independently challenge value, text-range and
changed-value/metrics outputs with secret and ordinary-text neighbors; do not
rely solely on a Password role or the upstream heuristic. Qualify the actual
selected Windows/renderer/UIA path when its existing execution gate permits.
The present source finding adds no UI-M0 blocker and lifts no D1–D3, Windows,
backend, package or release hold.

Exact upstream response and a compact source/hash review are retained under
`/home/david_osipov/.local/share/nevrotick/dependency-refresh-20261106-v1/direct-research/password-applicability/`.
This assessment ran no Cargo, native Windows, UIA, solver or product disclosure
test. Reassess after relevant control, toolkit, consumer, profile or secret-flow
changes; the current absence of a password producer is not a general dependency
security certification.


## BH-NATIVE-01 popup cleanup repair — 7 November 2026

David authorized analysis/repair of the supplied logic-bug handoff. Independent
source review confirmed the conditional failure-accounting path: `DestroyMenu`
returning zero became a drained ordinary failure, allowing later cleanup to be
acknowledged without that negative result. No actual leaked handle or native
failure was reproduced.

The actual destruction result now enters the private sticky bridge cleanup
acknowledgement before transient notification. Failed destruction returns the
existing cleanup error; a later successful call cannot clear uncertainty. The
popup entry rejects before another allocation. Existing worker completion,
`try_finish`, `cleanup_confirmed` and Slint `poll_stop` all retain that failure.
Ordinary creation/build/foreground errors remain distinct when destruction
succeeds. No destructive retry, new unsafe block or FFI argument change was added.
Two authored portable tests cover failure/drain/restoration/later-success and a
successful neighboring destruction. They have not executed.

Independent reviewer `bugfix_independent_review`, created with `fork_turns="none"`,
completed source review through the actual popup/worker/host stop chain and found
no material repair defect. A subsequent formatter-only signature change leaves
its behavioral conclusions unchanged. Current source hashes:

- `src/native_tray.rs`: `0ffc9a81df1f418400cead00448b94d517c0992b34e1e388c38e917fc42a8ea2`.
- `src/native_tray/windows.rs`: `14cd1bc3b46c4e7596a29b8344a4ddb7e38c13aef80f0db102a35ea837d8bbca`.

Evidence is retained under
`/home/david_osipov/.local/share/nevrotick/dependency-refresh-20261106-v1/`.
`bughunt-native-source-controls-v1.log` passes14 Python source-custody controls,
not Rust/native behavior. `bughunt-format-v1` retains its formatting-only failure;
`bughunt-format-v2/verification.json` passes admitted boundary controls and full
workspace formatting on377-input `44509d32073d69ea61a30ae4da9c0e838c8c02aa9102d8d90f4ec84d468ce482`,
with unchanged source/tool/vendor bookends. No compiler/test pass follows from
formatting. The working tree and existing staging remain separate; nothing was
staged, committed or pushed by these repairs.

**Still unqualified:** GUI compiler/Clippy/portable Rust tests lack the admitted
host SDK here: read-only preflight found no Fontconfig/Freetype pkg-config entries
or development headers; the retained private Debian13 SDK is not qualified for
this Ubuntu host. No direct-Cargo or altered-SDK fallback ran. Native Windows
execution remains held. When admitted, exercise the real popup-to-worker-to-host
stop chain with controlled failed destruction, healthy wake/subsequent cleanup,
blocked further popup allocation and failed final cleanup acknowledgement, plus
a successful-destruction neighbor. Observe actual USER resources separately.
This source repair lifts no UI/native/package/Windows hold.


## Distinct-range lock-order repair — 7 November 2026

David supplied the remaining-source audit on historical5ef9853 and requested
continuing the bug-fix workflow. Its private `/tmp` artifacts are absent here;
those reported experiments were not treated as current execution. A fresh
read-only reviewer and a separate repair owner inspected the actual unchanged
4427ab42... helper, active `Compare`/mutation paths, cloned same-context ranges,
pinned std futex backend and generated COM agility. Opposite comparisons could
retain different first read guards while queued writers exclude their second
reads. Context tree read guards do not supply serialization. This is distinct
from the earlier same-lock alias repair; no Windows hang was reproduced.

The helper now detects aliases first and uses one guard. Distinct live owners
are acquired by increasing address, with both guards retained through the single
comparison and values restored to original left/right callback order. Addresses
supply only a temporary lock order, never COM/persistent identity. Acquisition
failure skips comparison and releases earlier guards; callback unwind releases
both. Caller casts, same-context/endpoint checks, poison policy, dependency graph,
features, versions, licences and Windows-only role remain unchanged. No unsafe
code, cloning or snapshot replacement was introduced.

**Actual RED/GREEN:** the isolated Linux fixture uses the exact std-only helper
source, the re-admitted shipping1.99.1 compiler and existing verifier sandbox,
cleared environment, read-only inputs, bounded output/processes and60-second
outer command limits. RED retained the actual original production prefix and
added the independent tests: compilation succeeded, seven tests executed, five
passed and exactly the two new acquisition-order assertions failed at native101.
The raw wrapper retained `incomplete` because nocapture panic diagnostics broke
its row parser. Separate `red-v1-reviewed.json` adjudicates the complete raw
records without replacing that receipt or rerunning the tests. An independent
reviewer checked those exact failures and accepted behavioral RED. Assertions
following the first queued-writer failure were not executed in RED.

After repair, green-v3 passes all seven ordinary and seven optimized tests,
workspace-equivalent Rust/Clippy deny rules, formatting and both Windows-target
compiler/Clippy metadata checks with the full configured flag vector. No linking
or Windows execution occurred. Cases cover both operand orientations, physical
acquisition-rank poison, first/second acquisition failure, simultaneous real
protection, alias queued-writer behavior and callback unwind. The forced opposing
writer probes deliberately refuse their second nonblocking acquisitions and
verify common first-owner selection; they do not demonstrate successful blocking
COM comparisons under actual native contention. The healthy noncontended helper
controls preserve callback values/order. Source/tool/driver bookends are unchanged.
Green-v1 retains a formatter failure; green-v2 retains seven ordinary passes and
test-only Clippy failures, subsequently repaired with fallible setup and two
reasoned, narrowly scoped intentional-panic expectations. No failed attempt was
relabelled as a pass.

Evidence root:
`/home/david_osipov/.local/share/nevrotick/dependency-refresh-20261106-v1/bughunt-distinct-range-v1/`.
Final helper SHA256 is
`cb6bb3924d2e6bd99edf7269b2ab6ec59ac42bed9313579f1ba6108b5497b473`;
patch-notice SHA256 is
`907b63541dd48cb12b14e8c66d0780176c174e25d25fe230e093a5c629f03b7e`.
Fresh-context reviewer `accesskit_pair_independent` independently reviewed the
contracts, RED records and final repair/test changes; it ran no jobs or edits
and found no additional actionable defect. Actual native scheduling, compatibility,
full-GUI coverage/mutation, resources and teardown remain open CHECK-22/27 work.
The full Slint build remains subject to its SDK/native holds; this std-only
control establishes no full GUI, package or release qualification.

**Source admission integrated:** only the helper and patch notice changed within
the27-file local derivative. Original archive/revision/source-map/grant fields,
modified-file allowlist, registry identities, lock/manifests and selected
features/roles remain unchanged. The derivative map and code-owned policy pin
were rebound; canonical dependency-policy digest is
`cbce0fe93225659982120df573e98a1276155f2a9f5c11af7c02a8e45885cdb5`.
`source-adoption.json` retains the exact delta and377-input current map
`f6764d23bc6169c6d9d18be6ce4f83de3e4326342efa987294b7d98f9593cfdb`.
Python controls pass14 AccessKit-patch,28 Slint-profile,120 verifier and51 engine
consumer cases. Prior numeric/native receipts keep their executed candidates;
rebinding supplies no new Rust coverage/mutation score. No commit/push or native,
formal, fuzz, storage or runtime-policy waiver follows. Resume the existing
product queue and its pending N1 contract review after this bounded repair.


**Concurrent guidance integration:** HEAD advanced externally fromef3c0ab to
c8df999, changing six testing-skill guidance files. The coordinator read the
applicable testing-contract/index delta; meaningful-repair RED/GREEN and existing
qualification boundaries remain compatible with this work. All377 tested/adopted
product input hashes remain identical. The index was also updated externally;
its new staged state was preserved, not rolled back. This agent issued no stage,
commit or push. `closeout.json` records both HEAD/index identities and scoped reuse:
after helper execution only the unused Slint canonical-policy pin changed among
its Python helper inputs; the actual Rust helper/config/compiler/tool bytes stay
unchanged. Current source admission and consumer controls are separate fresh
checks. These are bounded source/host results, not a new full-product score.


## Text-inventory checker repair — 7 November 2026

The [current tooling follow-up](codeql-cloud-handoff.md#local-text-inventory-and-analyzer-output-repairs--7-november-2026)
records the long-spacing live-control omission, actual full509-ID RED/GREEN,
85 passing text-inventory tests and the real509/509 CLI result. Header parsing
now retains named/live control custody under existing source work limits. Source
checking grants no translated-consent approval, rendered/native/accessibility
pass or new UI scope. The current helper/candidate and retained failures are in
that same record; no additional implementation plan or execution waiver follows.

## AccessKit inclusive word-endpoint repair — 7 November 2026

Accepted the fourth remaining-code review's navigation finding after reproducing
it against the actual `accesskit_consumer` 0.38.0 public API. A valid 256-character
run admits its inclusive endpoint, but `backward_to_word_start` narrowed that
`usize` position to `u8`. The multiword endpoint returned 0 instead of 254.
This is a text-navigation correctness defect under CHECK-22, not demonstrated
memory corruption or a privilege crossing.

The [local consumer derivative](../third-party/accesskit_consumer/NEVROTICK-PATCH.md)
widens the word-start keys before binary search. Root Cargo patch routing and the
lockfile select that derivative. Exact-start previous-word semantics, ordinary
cross-run fallback and input limits remain. Original archive members, immutable
revision/checksum, original manifest and MIT/Apache notices are retained; the
eight new original Rust regression cases carry their separate GPL-3.0-only grant.

Evidence: `/tmp/nevrotick-review-fixes-20261107-z36bdzau/`. The initial fixture
compile error, E0716, is retained as setup failure, not behavioral RED. The corrected
REDv2 fixture compiled, then failed four intended assertions while four neighboring
controls passed. Unchanged test bytes pass after the causal search repair among
all 213 library tests in both ordinary and optimized Linux builds on Rust 1.99.1.
Doctests selected zero tests and supply no additional coverage. Cases include
endpoint 256/neighbor 255, exact word starts, document start, a single word,
word offset 255, UTF-16 surrogate pairs and a second run's actual origin.
The configured namespace boundary control passed; source/vendor mounts stayed
read-only, dependency archives matched their locked checksums, and no host
network, Windows UIA/COM/Narrator or Slint layout execution occurred.

The [dependency admission follow-up](s0.1-source-readiness.md#accesskit-consumer-local-source-admission--7-november-2026)
records the three-patch graph. Vendored packages are excluded from workspace-member
test selection: before changing/removing this consumer patch, run its named tests
in a separately admitted pure host scope with explicit consumer package selection,
ordinary and optimized builds. The existing full GUI command matrix is unchanged;
this repair adds no portable-row admission shortcut or zero-doctest waiver.
Native caller behavior remains with the existing admitted Windows/accessibility
owner. Historical GUI receipts do not qualify the modified combined candidate.

Independent fresh-context review, `review_two_dependency_repairs`, accepted the
scoped correction without finding an introduced defect:

> Independent read-only review found no introduced defect in the consumer correction. The non-narrowing search preserves previous-word semantics at exact starts and the existing cross-run fallback. Eight valid public-API cases retain independent literal expectations, including endpoint 256, neighboring positions, offset 255 and UTF-16 surrogate pairs. Raw REDv2 evidence records four behavioral failures with four healthy controls; unchanged regression bytes pass in the ordinary and optimized 213-test GREEN snapshots. Final source hashes match those snapshots; these results qualify portable consumer behavior only.

### Bound notification attempt hardening — 14 November 2026

Owner explicitly requests tightening the local notification admission/completion
API under the existing [three-interface coordinator child](engine-transport-pilot.md#owner-requested-api-relationship-hardening--14-november-2026).
A private affine guard must bind the actual session and immutable prepared notice;
preparation abandonment remains reusable, while every actual Show attempt stays
spent, including uncertain acceptance/unwind. Preserve existing native checks,
activation/privacy/cleanup and fixture-only behavior. This source child lifts no
UI-M0/D1–D3/native/backend/package hold and supplies no new display or production
notification capability. Portable tests and Windows-source compilation must be
reported separately from actual Windows execution; the implementation and checks
remain unqualified until their current source is executed/reviewed.

The integrated source child now passes all nine selected portable notification
tests and GNU all-target Clippy; Windows-target UI Clippy in WSL also compiles the
actual changed native caller. Private-owner compiler controls accept legal guard
use and reject removed unbound completion (E0599) and simultaneous session loans
(E0499). Preparation remains unspent, while accepted/indeterminate/unwinding Show
callbacks consume the exact bound notice once; another session stays independent.
See the coordinator's 603-input `902f133b` source, raw results, independent review
and preserved holds. No native Show, Windows cleanup, accessibility, release or
production-incident qualification is claimed; fresh applicable per-file metrics
remain with the existing GUI owner.

### Structural equality and combined tray finish — 14 November 2026

The owner's [additional handoff](engine-transport-pilot.md#additional-contract-and-integration-handoff--14-november-2026)
authorizes local H02/H06 contract work. RestoreToken keeps generation plus
Arc allocation identity, with exhaustive field decisions. NativeTray now returns
one latched observation: Pending, CleanupConfirmed retaining the existing worker
result, or CleanupUnconfirmed retaining that result/custody refusal and its cause.
The sole Slint Stop consumer handles this closed observation without a second
cleanup query. Operational Native failure with confirmed cleanup still permits
the existing cleanup barrier; panic/destructive failure never fabricates an ack.
Missing-worker refusal does not imply a join. The old generic latch's u8 fixture
is replaced by cases through the actual typed tray latch/classifier. Preserve
the notification barrier, finite wait, effect custody and all native/UI holds;
source compilation/portable cases supply no actual Windows cleanup pass.

Current `1cbddfb3` source passes GNU UI Clippy, Windows-target UI Clippy and all
13 selected portable tray tests. Actual RestoreToken field addition fails at both
comparison consumers and compiles after an explicit field decision. The current
terminal classifier preserves operation/cleanup distinctions and sticky outcomes;
its exact native producer and Slint consumer compile for Windows. No Windows
execution, AppVerifier/PageHeap, UIA, cleanup or release acceptance is inferred.

### Preview admission contract controls — 23 November 2026

CR-01 and CR-03 cover the selected preview seams under CHECK-25. The fixture
`UiSession::update_preview` propagates the typed model error before returning
a reset, and `PreviewModel::update_selected` publishes the selected row only
after `SourcePreview::parse` succeeds. The checked constructor bounds each
field before allocating a Garde report and copies the same validated borrowed
label and host into private owned fields. In the Slint path, Submit binds the
current raw draft to the current editor target, then calls `SourceDrafts::add`
or `edit`. Both use `DraftSource::parse` and check current identity uniqueness
before publishing an accepted change. The callback updates accepted projections
and editor ownership on success. Refusal retains the accepted rows and presents the
bounded error with the raw draft still available.

Under the Rust 1.100.0 GNU sandbox, the exact model and draft source copies
passed five preview tests, twelve draft tests, three doctests and Clippy with
the reviewed Garde 0.23.0 derive-only dependency closure. A legal external
consumer compiled and ran. Direct external construction received E0451 and
mutation received E0616. Separate ordinary same-crate sibling controls received
E0616 at the `SourcePreview` and `PreviewModel` private field writes, with a
compiling checked sibling. These scopes cover callers outside the model owner.
Descendant access and code within the defining module remain source-review
obligations.

Three runs called the actual constructor, fixture
update and source-draft Edit. The original refused `" bad"` with
`InvalidLabel` and admitted `"Sample"` and `"Sentinel"`. A Garde-only rule
sentinel additionally refused `"Sentinel"` at the same consumers. Omitting
the real validation call from that specimen compiled under denied warnings
and made both previously rejected labels reach the committed state. Rejection
preserved the accepted model and draft inventory. The focused generated Slint
callback test stages `" bad"` successfully as raw text and verifies that Submit
refuses it, preserves accepted rows and selection, and leaves the editor open
with the same raw draft.

The actual default GNU UI package passed package formatting, Clippy for its
tests and 25 tests across preview, source drafts, fixture presentation and
generated Slint bindings. The presentation suite includes seven tests with
the actual `UiSession::update_preview` failure and success paths. The guarded
run admitted the exact vendor inventory and selected GNU graph, then compiled
and exercised the generated Slint callback with Slint 1.18.1 on its public
software platform. An isolated run removed the leading-space rule from the
actual constructor's label validator. The same generated callback test ran
and failed its accepted-row equality assertion after publishing the bad label.
The supervisor restored the original bytes and rechecked the complete source,
tool and vendor identities. Windows-target and native UI behavior retain their
own evidence requirements.

The source control base is commit
`70586c24ba2ebad6d531db6be5e6fa669c72625d`, tree
`96ff3089ff1fc528278759894ec892838203bfd9`. The only product-package overlay
is the callback test, SHA-256
`3d092adc244bd975742d2995f8e04469be8e29d4641e943376400a788ab97970`.
The 651-input UI snapshot digest is
`f69e6d16abf1914f1840748a550f4684edfe7631c8f7c6039a37884d18f808db`.
The test and Clippy-generated `app.rs` copies both hash to
`d0d88a132e9e6405625dcea4dd22946ca745f9b7e40daf18516d66a80d15c038`.
The `slint-controls.json` receipt, including setup attempts retained alongside
it, is under `~/.local/share/nevrotick/preview-admission-20261123.GGlhxCrZ/`.
Its SHA-256 is
`f8f851731ba6700fa4147927fc1d87d9833da6a0b19ae4a056845eb2cf47a50f`.
Mechanism and sibling privacy receipts in the same directory hash to
`bc53c51277d01cd7e86c87b208d011ca667df9df79e1571725233bb20a27d5fe`
and `2887695bed2ff3525a0dae9ebbcf00afafc6fb7ac768ba149cf682ce128b065c`.

The follow-up source report includes `SourceDrafts::from_baseline`, which
retains the checked path from fixture startup and recovery. Independent
review traced both callers and confirmed the capacity, validation and
publication order. The [build record](build-optimization.md#bind-preview-exceptions-to-their-reviewed-seams)
retains the selected syntax controls and exact exception checks.

H10 stays with the next native notification boundary edit. The
`CloseOutcome` type still needs the type-level warning carried through the
actual Windows wrappers at that trigger.

### Private Apartment trait controls — H11, 23 November 2026

S0.5 and CHECK-25. `WindowsToastPort::open` obtains the private `Apartment`
on the UI thread. The port retains it as its last field so its own COM interfaces
drop before the guard's matching `RoUninitialize`. The trait controls apply to
[the actual Apartment definition](../crates/nevrotick-ui/src/native_notifications_windows.rs)
within that production module. This makes the affinity marker decisive even when
other fields of the containing port carry their own restrictions.

For replay, freeze the product inputs and use the
[product verifier](../scripts/verify.py) to admit tools, the vendor inventory
and the default Windows target graph. Select the `dependency-metadata-*` and
`dependency-tree-*` entries from `commands("windows", schema_version=3,
patched_packages=tuple(p["name"] for p in policy["patched_packages"]))`.
These commands use `cargo metadata --frozen --offline --format-version 1
--filter-platform x86_64-pc-windows-msvc` and the admitted workspace and
per-package `cargo tree` selectors. Apply the verifier's metadata, tree and
selected-graph validation before compiling. In its network-isolated sandbox,
preserve the target flags and check the actual Windows library with

```sh
cargo check -p nevrotick-ui --lib --target x86_64-pc-windows-msvc \
  --frozen --offline --jobs 3 --message-format=json
```

Begin by appending the healthy case at the end of the notification module in
the owned copy.

```rust
const _: fn() -> Result<(), NativeFailure> = || {
    let on_owner_thread: Apartment = Apartment::open()?;
    drop(on_owner_thread);
    Ok(())
};
```

Compile that copy first. Append each following block to a separate copy of the
healthy specimen.

```rust
const _: () = {
    /// Require exactly the selected capability from the actual Apartment.
    fn require_apartment_send<T: Send>() {}
    let _: fn() = require_apartment_send::<()>;
    let _: fn() = require_apartment_send::<Apartment>;
};
```

```rust
const _: () = {
    /// Require exactly the selected capability from the actual Apartment.
    fn require_apartment_sync<T: Sync>() {}
    let _: fn() = require_apartment_sync::<()>;
    let _: fn() = require_apartment_sync::<Apartment>;
};
```

Require E0277 at the `Apartment` bound in each copy, with `Rc<()>` named
in the trait obligation. Then replace `PhantomData<Rc<()>>` with
`PhantomData<()>` and remove the newly unused `rc::Rc` import within each
negative specimen. Each must compile under the same lint policy. Restore the
original bytes and verify the source, tool, target library and vendor identities
against their inputs. These are type checks of the owner and its compilation
configuration. Native COM calls and shutdown completion need native evidence.

The Rust 1.100.0 default MSVC cross-check on commit
`70586c24ba2ebad6d531db6be5e6fa669c72625d` and tree
`96ff3089ff1fc528278759894ec892838203bfd9` passed all five controls. The
legal case exited 0. Send and Sync each exited 101 with one E0277 at the
intended owner-local bound. Each marker-erasure case then exited 0. The
first Send erasure attempt failed on the unused `Rc` import, and the
replay removed the import without changing warning levels.

The 651-input product snapshot hashes to
`1ab2565c690fd38903b6c4c3dafbc19562178627397067da74e30fdd46ba5905`.
The actual notification module hashes to
`10f73a7addcbbd5b3145ec2650eae30f014bd0571c754e9d273cdf5e44196efe`.
The selected graph and 45-file MSVC target-library inventory hash to
`48c0ab19e6d1033aa91f84936f72427b324bdae1d8fe806f2ad6c5e5a156da9a`
and `30e5e4dfeea950fdaf44161aa53b3678a4b4b539e53df223326d06c9d8d4cddb`.
The runnable `check-apartment.py`, both attempts and the final
`h11-apartment.json` are in
`~/.local/share/nevrotick/apartment-h11-20261123.UkUUtURf/`.
From the checkout, pass the retained driver to `python3 -X dev -Werror -B`
with two new private directories as arguments, first for receipts and then
for scratch. The driver replays the frozen candidate above.
The final receipt SHA-256 is
`eafe9a6b0ee09733770ae67aec6ba490e64171882885b206c1e6f6f239a8b0a1`.

## Require the Windows desktop image-load policy before Slint — 23 November 2026

W11-W10, S0.6/P6, WIN-3/WIN-4 and CHECK-16/18/20/23/26. The actual
Windows `main` reaches the image-load guard through `run_native` and
`create_native` before native backend selection. It reads the current policy,
adds the three required enforcement bits when needed, and checks the effective
result. Already enforced settings require only a query. The guard preserves
inherited recognized bits and refuses a native error, unknown flags, or missing
effective settings. Refusal reaches the native entry point without initializing
Slint. The portable fixture constructor does not apply a process policy.

The Rust implementation, contracts, Windows-UI security and QA checks
reviewed the fixed policy and the actual startup caller. The private Windows
adapter keeps the existing GUI dependency graph and GPL-3.0-only boundary.
The Kernel32 declarations use the pinned binding/SDK ABI and fixed current
process and DWORD layout. The source controls distinguish omitted, deferred,
and ignored startup checks. All 15 source controls and seven portable guard
tests passed. The directed incomplete-mask control failed at the intended
missing-bit assertion while the healthy neighbor passed. It carries zero
generated mutation credit.

The exact source-linked x64-v3 fixture exercised one native setter and
readback in a fresh inert process in offline Windows Sandbox build 26100.
Its baseline lacked at least one required enforcement bit. The process
observed all three enforcement bits and its inherited flags after the call.
The child, both streams and the mapped input hashes were checked before
export and retirement. Receipt SHA-256
`32143bcfa1fcf91d44f4d1da181fcc1de98f3f5b43e922f80a28777484a61dd8`
is retained in `~/.local/share/nevrotick/ui-image-policy-native-fh7gxyj6/`.

The configured Windows `default` row on tree
`738d097b3eeaeb84d2de3f2db35e2e50eed03ef9` passed all 35 lanes, including
the pinned graph, MSVC compiler, all-target Clippy, release UI link and
MSIX inspection. Independent PE readback found the setter and getter in the
actual release UI imports. The build receipt SHA-256 is
`11a7754ad9e745f7e2d513eb033abba09c3e566240fd305ea433f1e38a79b452` under
`~/.local/share/nevrotick/ui-process-policy-windows-pzaj1r4j/`. The first
cross-build was cancelled before a receipt when the initial local lint check
reported missing contract sections. The final build used the remaining
allowance and retired its scratch. Independent source review checked the
startup, inherited/error paths, ABI and exact fault oracles.

The registered adapter retains its 90% executable-line and 95% critical-file
mutation obligations. Actual Slint startup/compatibility, effective behavior
for remote images and System32 search, early static loads, structured failure
delivery, and packaged activation remain open. The selected local Low-label
case is recorded below. The other process profiles, signing and release retain
their existing owners.

### Require permanent strict handles before Slint — 26 November 2026

W11-W10/S6.1, WIN-3/WIN-4 and CHECK-16/18/20/23/26. Architecture
§15.1/15.4 requires separate GUI strict-handle qualification. The existing
startup owner now has a bounded child to require both documented fields
before backend selection. Initial word `3` needs no setter. Initial
`0`, `1` or `2` requires one exact `3` request and a fresh exact readback;
unknown flags and native or incomplete-readback failures must retain the
first typed cause and return before Slint. No weaker retry is permitted.
The ordinary portable fixture remains free of process-policy effects.
The private GUI adapter and GPL-3.0-only graph remain the owners; importing
the GPL-2.0-only native-platform crate is not selected.

The stage starts at `2026-11-26T16:15:54Z` and ends no later than
`17:20:54Z`, with 65 minutes total: 20 for source/focused controls/review,
25 for the unchanged configured Windows build, 15 for native controls,
and five for final readback/cleanup. Every setup failure, repair and
partial run consumes that same allowance. Root owns Python enforcement,
integration and the original offline guest; a private-worktree worker
owns only Rust startup changes and a separate worker prepares the inert
native capsule. Fresh-context review assesses the contract and combined
candidate independently.

| Stage and skill owner | Requirement, action and observable evidence |
| --- | --- |
| Rust/contracts, native UI and native-UI security | Fixed current-process DWORD ABI, checked word `3`, bounded typed first-failure propagation through the real startup caller, source-visible types and unchanged dependency/licence boundary. |
| Static, contract, unit and boundary QA | Actual source consumer refuses missing/ignored/deferred/late startup calls beside the maintained caller. Ordered literal native-call controls cover recognized and reserved words and every failed stage. A weakened effective-state guard must fail its intended case. Source enforcement is not a runtime proof. |
| Configured build and regression QA | Format the completed source before freezing. Re-admit the current default Windows graph, run every configured guarded-UI row and retain the exact PE/PDB/object/map and receipts. Use the corrected `Assets` input and admitted vendor/tool custody. |
| Native, lifecycle and package QA | A fresh source-linked process calls the production guard and independently reads effective state. The exact guarded GUI opens its fixture window, is observed under permanent strict handles and exits through its original close owner. Retain original process identity, streams, native result and retirement; no host clock, services, trust, signing or COM10 effects. |
| Independent review and final qualification | Inspect source, real consumer and native/artifact outputs. Preserve failures and missing observations. Record reached scope and next action in this existing owner, not a separate plan. |

The selected exit is the source/build/native startup child, not full UI
qualification. The registered startup adapters still require their 90%
executable-line and 95% critical-file mutation gates at their existing
qualification owner. Actual invalid-handle fault behavior, debugger/
Verifier conditions, ordinary-user and installed activation, complete
mitigation compatibility and deferred D1–D3 retain their own exits.
No generated mutation score, full accessibility pass or release claim
follows from directed controls or one successful fixture close.

The integrated source and bounded observations reached their selected
exit. The actual startup caller propagates the typed strict-handle
refusal before backend selection. Six new literal controls cover the
recognized words, all thirty reserved positions, every native failure
and fixed diagnostic text. The complete source-linked policy suite
passed 15 portable tests. Its private bit-zero-only effective-state
guard failed exactly the incomplete-word case, with fourteen healthy
neighbors. This supplies no generated mutation percentage. The actual
source checker passed all eighteen controls after its new missing-guard
case first exposed the prior acceptance gap.

The first source-control dispatch had an isolated-import setup failure;
explicit unittest discovery then reached the intended source rejection.
The first full Windows build caught two unsafe calls in one block in
the independent test query. Separating the borrowed pseudohandle from
the query preserved its semantics and warning policy. The corrected
candidate passed all 38 configured guarded default-profile Windows
rows. Its 778 source hashes match Main, and the exact UI/PDB/entry
object/map remain in the export. The earlier receipts and source-linked
suite remain attributed to their earlier test-helper bytes.

Independent review required stricter native result consumers before
execution: exact field sets and integer/Boolean kinds, complete normal
retirement, known image flags and coherent elapsed bounds. The original
readers accepted fifty invalid specimens among 171 data controls. The
bound corrected readers passed all 171, including nine useful healthy
neighbors and exact returned flag pairs. The leaf admission also binds
the coordinator's observed producer exit zero and completed source/SDK
bookends; successful command rows from its finally receipt alone are
insufficient. These are evidence-consumer controls, not product test
executions or authenticated numeric-process identities.

The first native leaf observer stopped at its module import because
its invocation omitted the session execution policy used by the
existing GUI recipe. No product child or testcase started. Its primary
record and empty streams were preserved despite the initial host
reader's nonempty-output refusal. Exact cleanup observed no process
using the sole pinned image, held and hashed it, removed only the
original nonce directory and retired its transfers. The corrected
invocation uses the existing process-scoped `RemoteSigned` selection;
it changes no registry execution policy.
[PowerShell session parameter](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_powershell_exe?view=powershell-5.1#-executionpolicy-executionpolicy)

In the original offline build-26100 guest, the fresh source-linked
process 1568 independently observed `0 → 3`, passed its one exact
selected test and retired its process, both streams and private scratch.
The exact guarded UI
`99b4b122e12d444478735d4190bfd5dfbbd44f7ac53b24d4f63942ac175c12ae`
then opened the owned sample window as process 7892. It reported strict
handles `3`, image flags `7` and debugger absent. Startup took 360 ms;
normal Close, exit zero and complete collection took 518 ms. The
supported-v3 probe, original process/creation identity, source and input
bookends, EOF/reaping and empty transfer directories passed independent
raw review. Both launcher and child were elevated, elevation type `1`
and High integrity `12288`: this is the unpackaged elevated lab profile,
not ordinary-user qualification. Application Verifier state and actual
invalid-handle fault behavior were not established.

The Windows-only `HostError` cause test compiled in the all-target
checkpoint but was not executed. The meaningful must-use annotations
are present and compile; their actual ignored-wrapper diagnostic control
remains unverified. The 90%/95% file gates, ordinary-user/installed
activation and complete mitigation/accessibility/package qualification
remain open. This new UI is not present in the preceding unsigned lab
archive; retain that archive's original bytes and candidate scope.
Further assembly needs current common-source/role admission at its P6
checkpoint. David requested a safe PC-restart point: save source,
untracked files, artifacts and the temporary vendor before notifying him;
after reboot re-admit resources and the replacement guest before any
dependent native action.

The private producer worktree, both leaf-build copies and the empty
Windows build scratch were retired after byte-checked export. Main's
index was preserved, and the three new untracked Rust files are included
in the durable candidate patch. The temporary vendor was retained as
`vendor-before-restart.tar.gz`; restoring that cache still requires
private extraction and normal source admission. All owned build, native,
cleanup and archival jobs are terminal, and the original transfer
directories are empty. No further job is started before the requested
restart handoff.

**Post-restart reentry — 26 November 2026.** David confirmed the PC restart
and a newly started Sandbox. HOST inventory identified the sole new guest
`43ca91ed-2a92-4302-b75d-ac977584fe82` with unchanged `wsb` version
`0.8.114.0`; the previous guest's observations retain their original scope.
The retained vendor archive matched its recorded SHA-256 and was privately
extracted after rejecting linked, escaping, duplicate and unsupported entries.
Its extracted payload still needs the normal vendor-content admission. The
ten-minute reentry allowance, `17:27:47Z`–`17:37:47Z`, ended incomplete before
that admission and the new guest's effective configuration checks. No product
command or guest payload ran in this attempt. The original restoration and
closeout records are under
`~/.local/share/nevrotick/restart-reentry-2611.7xwmkxm5/`.
Keep that incomplete attempt; a dependent native child needs a separately
reviewed fitting approach and current environment admission. Independent
service source work continues.

The follow-up read-only vendor admission checked the restored payload against
the current dependency policy using the configured verifier. It accepted
29,510 files. Inventory SHA-256 is
`f1d5d9736121beee9ee3ad510fd641179c0f47a3453982936230fb90772dfa82`.
`restored-vendor-admission.json` in the same directory records the policy,
lockfile, verifier and extraction bindings. The current guest's reduced
offline profile was also observed for the separate
[service feasibility experiment](windows11-integration.md#privilege-assisted-service-ownership-candidate--pending-owner-disposition).
The UI actor, package, toolchain and resource admissions still apply to
the next UI run.

A later storage check moved only the generated Cargo target from the completed
`s21-terminal-fence-green-checkpoint-2611` run to an archive on the local Windows
volume. A full restore rehearsal matched all 2,576 entries, file hashes, modes,
timestamps and hardlink groups. WSL gained 729,903,104 available bytes and
passed the service runner's 4 GiB floor. The original receipt, source and
Git state were preserved. `cargo-target-relocation.json` in the same reentry
directory binds the archive location and hash to the completed restore check.

| Record under `~/.local/share/nevrotick/ui-strict-handles-2611.m_d46orh/` | SHA-256 |
| --- | --- |
| `windows-default-guarded-r2.json` | `3ae15bcef34cdf325fe09d9ca7ad3b2fd4c580704d69a8e3612d4e1b35c030a3` |
| `policy-leaf-current/receipt.json` | `deb49ce3d7a3d7198fbd22d6f6d740168cfddbf552dd89b148d6873657f40ff6` |
| `readback-green-r4/result.json` | `0aacbcc1495e8d2b6dcbb9a92f2c05b6719d29bb0f47f84f00da1fdf1d66185d` |
| `native-preparation/retire-failed-leaf-880f778b/execution.json` | `8e06851c371725ef62f96747daf0e62df0fb30aa3714921c82e0d0924aba51f0` |
| `native-preparation/bound-leaf-r4/results/verification.json` | `1513eb968ede74ca194ac9be8e5f7aaf107247504aeb8340332ab5b15489664b` |
| `native-preparation/bound-ui/native/receipt.json` | `99926413a143ca81bf6520c13c37dcc1afa43747ba72a66dd445d254ffa5882f` |
| `review/final-native-readback-review.md` | `1b73ce32a526da4b416ae16f29d172a9882a1e6ab49fd32d3d1714a79c6e5239` |
| `vendor-before-restart.tar.gz` | `2339af490d8fa1908af2d1bb64948c0d56609abcd81ff596d1790df1af4f3096` |

### Configure the default DLL search before Slint backend selection

W11-W10/S6.1, WIN-3/WIN-4 and CHECK-16/18/20. The actual Windows
`create_native()` applies the existing image policy first and then sets
the current process's default DLL search to Application Directory and
System32. The setter uses the existing pinned Windows binding.
Failure preserves the original Win32 code and returns before Slint
backend selection. The portable fixture constructor has no process
policy calls.
[Microsoft loader contract](https://learn.microsoft.com/windows/win32/api/libloaderapi/nf-libloaderapi-setdefaultdlldirectories)

The DLL search and UI-source controls exercise the fixed `0x0A00` mask,
successful setter, exact failure code, and omitted, ignored or late
startup calls. All 16 source controls and nine portable policy tests
passed. The exact startup module also linked as an x64-v3 MSVC policy
fixture. Fresh-context review checked the actual entry point and pinned
Windows binding.

| Record under `~/.local/share/nevrotick/s61-runtime-dll.8hEu7nr1/` | SHA-256 |
| --- | --- |
| `ui-runtime-dll-search.patch` | `e0e25cd5a70ed1532b3f60c1b36dca424e33953ebd83948990a3ac7e9402ff1b` |
| `source-tests.log` | `f25fd27cba8306d54ab9de8227c10d9192a7e2f19c4e069f3673281de6aba4a6` |
| `leaf/receipt.json` | `991ccda41cafc988cf14a16ee38cac138018d36fc1d4fb2bc22c5e9a399bda2a` |

The configured Windows release build passed all 35 lanes on the isolated
tree `cbc20ef868c432e0492820a178dc0f242632699e`. All 768 build source
pins matched Main after integration. Independent PE readback located
`SetDefaultDllDirectories` and the existing image-policy imports in
the actual Slint fixture. The linked image and PDB were rehashed against
the checked export. The build verification receipt is
`4a6cc97fbfbc282620deb9aef4c90d21818855f9ad4e146aec8ceb5310075d4d`
under `%LOCALAPPDATA%/NevroTick/ui-runtime-dll-policy.B8HBaWlH/`.

The exact release fixture opened its sample window and exited cleanly
after a Close request in the offline Windows build-26100 guest. The observer
matched the window to the launched process, observed exit zero and both stream
completions, and confirmed reaping and staged-file retirement.
The guest ran the elevated unpackaged profile. Ordinary-user
and installed activation retain their own checkpoints. The native
`receipt.json` SHA-256 is
`5589c8baeaa185bf23d011dbbd9cf2deb3486f0c212ae07a611ff8a00d4ea868`
under that directory's `native/`.

The default applies to later DLL searches without explicit loader flags.
Pre-entry imports, explicit loads, the installed dependency closure,
loaded-path observations and affected-file qualification retain their
own checks.

#### Observe the loaded modules at Slint window startup

W11-W10/S6.1, WIN-3/WIN-4 and CHECK-16/18. In the same owned offline
Windows build-26100 guest, the exact linked fixture opened its sample
window in the elevated unpackaged profile. The bounded observer recorded
39 module paths on the original live child after the window appeared and
before requesting Close. The child exited zero, both streams completed,
and the observer reaped it and retired the staged files. The snapshot
contained the staged executable, 37 direct System32 DLL paths, and one
Common Controls DLL in WinSxS. The original reader refused the WinSxS
row and retained the complete raw snapshot.

A separate read-only check in that guest observed the exact reported
`comctl32.dll` and its supplying manifest. It checked the path ancestors
and file attributes for reparse points, the bounded file hashes against
the host's exact component, and the manifest identity
`Microsoft.Windows.Common-Controls` amd64 `5.82.26100.8328`. The guest
exited zero and the observer retired its staged files. The independent
review compared the path with the direct Common Controls import in the
original executable. Windows can resolve DLLs through side-by-side
assemblies independently of ordinary search order.
[Activation contexts](https://learn.microsoft.com/windows/win32/sbscs/activation-contexts)

| Result under `~/.local/share/nevrotick/s61-loaded-images.RRGxJaQM/` | SHA-256 |
| --- | --- |
| `native-20261124T193759Z-ur634hby/receipt.json` | `8ebbc202b0d811ce6f2bebbd651b6b7167ed27f7c294cb9dc6ba7be74dc297d4` |
| `native-20261124T193759Z-ur634hby/modules.json` | `e795362b8b79ff1577993a6f734c7a06b0104a3c2c19fe93ffbcb7554bb8a3de` |
| `common-controls-readback/receipt-dk3u8frv/receipt.json` | `6e3f040e4bd9938d283f6c2f9730852ddc082a46d99bcac2b895f15a796637d0` |
| `common-controls-readback/receipt-dk3u8frv/component.json` | `1aa019d87333e0fd754b4395063726d04759b8d3eeb17f13960e9f2988df9f93` |

This establishes the sampled paths and corroborates that one Windows
component's separate file identity. The original reader's refusal remains
in its receipt. Mapped image byte identity, catalog authority, transient
or later loads, the causal search path of each dependency, and the ordinary
installed role retain their own qualification. Apply the
[updated Windows security profile cases](../docs/plans/windows11-integration-follow-up.md#windows-security-compilation--24-november-2026)
when qualifying the final service and package profile.

#### Gate the selected Slint fixture before its CRT

W11-W10/S6.1, OD-07 and CHECK-16/18/19/20/26. The Windows artifact
export can now select `--ui-baseline-entry`. A separate GPL-3.0-only C
entry checks the cumulative CPU features and XCR0 before calling the
existing `mainCRTStartup`. An unsupported CPU or OS state takes the
original process to `ExitProcess(50)` before the CRT call. The exact
UI link receives a digest-named read-only object. The Rust payload
keeps its x86-64-v3 build and GUI subsystem.

The CPU-dispatch and Slint skills informed the ABI and timing of the
handoff. Regression and artifact QA selected 25 hosted CPU controls,
including missing feature bits, absent register leaves and missing
extended state. Three temporary faults tested missing OSXSAVE, weakened
XCR0 checking and lost CRT status. Each reached its intended assertion.
All 192 verifier controls and 25 PE controls passed. The Windows build on
`a281ec3b` plus the retained overlay passed all 38 rows on Rust 1.100.0
with 772 bound inputs. An initial launcher invocation selected the wrong
working directory and stopped before verification. Its stderr is retained.

Independent readback reconciled the raw COFF object and LLD map with the
linked PE and matching PDB. The PE entry at RVA `0xe366b0` matches the
baseline object's 128-byte instruction sequence after three relocations.
Its guarded direct call reaches the original CRT at `0xe36d90`. The
other two calls resolve to the same `KERNEL32.dll!ExitProcess` IAT slot.
The PE retains a zero TLS callback slot and has no delay imports. It
declares 27 direct imports. The loader can initialize those dependencies
before the PE entry.

In the owned offline build-26100 guest, the independent CPU probe accepted
the platform and exited zero before UI dispatch. The original elevated,
unpackaged UI process opened its sample window with image-load policy
`0x07`, accepted normal Close, returned zero and completed both streams.
Both process observations and the input and output cleanup passed. This
qualifies the supported startup of this fixture. Cold refusal on an
independently masked platform, pre-entry DLL closure, ordinary-user and
packaged activation, and affected per-file metric gates remain open.

| Record under `%LOCALAPPDATA%/NevroTick/ui-baseline-entry.UYbr9My4/` | SHA-256 |
| --- | --- |
| `exports/verification.json` | `9f5e7a5bca65410ce9a5364adf46adda4b27a7e75f596e22f6579d8bab7fbe8c` |
| `exports/nevrotick-ui.exe` | `94ea9e41f996b2a523473d22a39a62f9233b70ade4d2b547ff4da8b1c170d337` |
| `evidence/linked-entry-readback.json` | `59a9d69ee46f23d09042117318afbc34c8ae8023b12a12e48346a4af338c74e5` |
| `native/receipt.json` | `4a3ec8a93492ad8c5c87af377f8283879d77be44037141e1c13d856b98f1f645` |

The private shelf includes the matching PDB, compiler object, compressed
link map and raw native observations. The link-map pin refers to the
decompressed bytes. The [combined ArtworkLab archive readback](windows11-integration.md#pack-and-verify-the-guarded-slint-and-service-combination)
binds this same guarded UI image to the unsigned carrier.

The maintained Windows export now retains the raw `nevrotick-ui-entry.map`
inside the atomic handoff. It checks the map digest against the recorded
link and binds the record to the exported UI image. A portable replay sent
the exact archived UI PE/PDB and map through the exporter and the ArtworkLab
stager after removing the original scratch. Maintained controls also refused
changed map or image bytes before publication and a crossed CRT declaration
during handoff readback. The historical native result applies to its original
link and runner.

##### Check debugger readiness for the pre-entry module observation

W11-W10/S6.1 and CHECK-16/18/26. The bounded next observation would
capture resolved module paths at the initial debugger breakpoint and
then check the original process and verified PE entry address before
the first entry instruction. The initial breakpoint precedes DLL
initialization in a debugger-launched process. A matching subsequent
stop could distinguish the modules present before the gate. Confirm
original process retirement in both cases and retain the earlier
ordinary native startup separately.
[Microsoft initial breakpoint documentation](https://learn.microsoft.com/windows-hardware/drivers/debugger/initial-breakpoint)

Read-only preflight on the owned offline build-26100 guest found the
current-user debugger configuration absent. The exact host SDK
`dbgeng.dll` also directly imports `DbgModel.dll`, which the current
tool inventory does not pin. Scope the companion admission and the
debugger/child lifetime owner before a run. The debugger and UI were
both unlaunched in this probe. The before/after guest observations and
configuration refusal are in `preentry-readiness.json` under
`~/.local/share/nevrotick/w11-timing-placement.Jc5XPQSG/`, SHA-256
`c2cd10f7cce052aadc6dd26b30a28fabb6f4f91779137c42da6e8a995a3eb63d`.

##### Observe both pre-entry module snapshots on the guarded Slint image

W11-W10/S6.1 and CHECK-16/18/26. Package-artifact and Sandbox QA bound
the observed image to the retained unsigned ArtworkLab archive and
the guarded Slint export. The exact `nevrotick-ui.exe` bytes in both
hash to `94ea9e41f996b2a523473d22a39a62f9233b70ade4d2b547ff4da8b1c170d337`.
The existing CDB and SDK library pins were reused. Host Authenticode
readback verified the same-version Microsoft-signed `DbgModel.dll` as
a scoped debugger input. The guest used the pinned offline debugger
configuration and a private command pipe bound to its original relay.

The owned offline Windows build-26100 guest stopped the same original
UI process at the initial debugger breakpoint and then at its linked
entry RVA `0xe366b0`, before the gate's first instruction. The latter
stop matched both the enabled breakpoint and the original image base
plus that RVA. Independent readback counted **28 images at each stop**,
including the UI and 27 DLLs. Every literal DLL named in the PE import
table was present. The paths and mapping ranges matched across the
two stops. The DLL rows identify 25 System32 paths, one WinSxS
Common Controls path and `ntdll.dll` by basename. API-set import
contracts do not appear as separate image paths.
[Initial breakpoint](https://learn.microsoft.com/windows-hardware/drivers/debugger/initial-breakpoint)
[Debugger module paths](https://learn.microsoft.com/windows-hardware/drivers/debuggercmds/lm--list-loaded-modules-)

The observer ended the debug session at the entry breakpoint with
`q`. The debugger and relay exited 0, the original target exited
`0xC0000354`, and both captured streams reached EOF. The observer
confirmed retirement without fallback termination and restored guest
configuration, scratch and transfer custody. The parser refused three
in-memory controls covering a missing entry table, wrong entry PID
and debugger command error. A fresh reviewer checked the original
archive, entry address and raw tables independently.

The first attempt captured the initial table before a UTF-8 preamble
corrupted the command. Two console-based remedies refused before
launch. The private pipe then reached the initial stop, but the
execute breakpoint was never observed. The next attempt verified
that a software breakpoint was enabled at the exact entry address.
Its reader refused the valid line because CDB included its prompt.
Both attempts requiring separate cleanup have removal receipts. The
successful retry used the same image and corrected prompt handling.

The two stops establish module presence on one debugger-launched,
unpackaged image. They leave mapped-byte identity, transient modules,
the CPU instructions executed by DLL initializers, cold refusal,
ordinary-user activation and signed installed roles open. The earlier
ordinary startup and Close observation remains separate.

| Record under `~/.local/share/nevrotick/s61-preentry-observation.HkG1hccv/` | SHA-256 |
| --- | --- |
| `dbgmodel-host-signature.json` | `6f94c1f367a7e951c4f1880620c400ff8e6a8f564b2b08d608da2be3236f056f` |
| `observed-06/inputs.json` | `97174494a78832f111714c16280c8555bc0b1b7ddb77b87aa89af060a8f19aff` |
| `observed-06/verification.json` | `80743ec585133412975a419f0766b8f8def49de1258703b557e705c31b83b42f` |
| `observed-06/cdb.stdout` | `e2f3e7f433dbefb860a8ac20ea8ae29a04ac258d60160880665cb5ec5bf3094d` |
| `observed-06/readback.json` | `ffb32f288d78198bccd5ff058039b496f93b87151e8a7044412e91922a2d4882` |

### Observe the Low-integrity image boundary — 23 November 2026

W11-W10, WIN-3/WIN-4 and CHECK-16/26. The security-boundary, integration and
Sandbox checks exercised the exact staged desktop policy adapter through
`LoadLibraryExW` in three fresh source-linked x64-v3 processes in the owned
offline Windows Sandbox (build 26100, `wsb` 0.8.114.0). Each process observed
initial policy `0x00` and applied the common `0x05` settings. The actual guard
added only bit `0x02` in the two guarded cases. Both positive cases verified the
loaded module path, executed the fixed canary export and released the module.

| Case | Effective policy | Native result |
| --- | --- | --- |
| Baseline Low | `0x05` | Loaded the exact local DLL and returned `0x4e564c42` |
| Guarded Low, same file | `0x07` | Null module handle and immediate Win32 `5` (`ERROR_ACCESS_DENIED`) |
| Guarded Medium, identical bytes | `0x07` | Loaded the intended local DLL and returned `0x4e564c42` |

Independent label readback confirmed explicit `S-1-16-4096` and
`S-1-16-8192` single-ACE labels with the same mask. The two DLLs had equal
bytes, owners and discretionary ACLs at both bookends. The same offline
guest launcher supplied the inherited context. Native child exits were
`0`, `10` (the observed failed loader call), and `0`. Guest scratch and
the mapped file contents were removed after completion and export.

The independent review matched native streams, selector outcomes, inputs,
PE imports and canary export. The exact staged tree was
`58203d2108abe3a933f38d7dacc2c0299b3e14fc`. The retained build, observer
sources and raw native receipt are under
`~/.local/share/nevrotick/ui-low-image-load-native-z1f3prz8/`. The receipt
SHA-256 is `5b10ff906d43b01dac4dcee2fb0191020f68c6b73aae41fbc5404db23d93dbde`.
This covers late local loading in the selected fixture. Individual complete
child tokens and a post-denial module list were outside its observations.
Normal Slint startup, remote images, System32 preference, early loader behavior,
per-file metrics and signed package activation retain their separate exits.

### Observe guarded Slint startup and caption Close — 23 November 2026

W11-W10, WIN-3/WIN-4 and CHECK-16/26. Preflight and package-artifact
checks used the current default x64 MSVC release link, with the admitted
v3 flags, vendor graph, SDK libraries and generated version resource.
The source tree `e440a58277c1b4e608441c29bac24b27cf99feda`
binds 664 inputs and preserves the verified pending Core and service
changes. The selected UI graph matches the earlier Windows admission.
Link and PE/PDB readback passed. The exact image hashes to
`1cc87bb6643e85aecab0ab97ba84a70baa0c49699ce9d37f973e3f89ba24ceba`.
Its matching PDB hashes to
`855858edafbafe50582e74a8828664ca1532c38a8d6583c654759b34d66ebef1`.

The Sandbox and Windows evidence workflows exercised the real fixture
entrypoint on offline build 26100 through `wsb` 0.8.114.0. The live
child displayed the visible `NevroTick — sample preview` window under
its original PID. The read-only process observation confirmed effective
image-load flags `0x07`. Caption Close reached exit 0 and EOF on both
streams without forced termination. The input and offline bookends
passed and the owned share files were retired. The guest launcher and
child both reported elevated tokens. This is unpackaged startup and
normal close in the elevated guest context. The non-elevated desktop
case, native lifecycle diagnostics, other loader cases, per-file gates
and installed MSIX activation remain open.

The current build and native receipts hash to
`97ba92160512dfe47df4fb0f35bcea6c18f7e6a17337beec211536c2b06b12b5`
and `a29c7ec47efc302abe66c3326ef9ed2317358ba941b73865f96eeb4f263fec3d`.
The exact inputs, raw observations and unshared binary archive pointer
are in `~/.local/share/nevrotick/slint-ordinary-startup-20261123.z6jp4ztp/`.
The retained image supports the next admitted desktop-token check
without another link while its source, tool and graph inputs match.
