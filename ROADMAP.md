# NevroTick roadmap

**Updated:** 30 October 2026  
**Android status:** planned; no Android implementation or handset qualification is established  
**Reviewed repository baseline:** [a54c04382cf5e5cb2ae641d7356ce67fe24bc1ab](https://github.com/DavidOsipov/NevroTick/commit/a54c04382cf5e5cb2ae641d7356ce67fe24bc1ab)

## 1. Direction and authority

David requested this roadmap for an Android application that runs a local time
server and redirects Android's existing NTP client to it, without root or an
always-active VPN interface.

Here, **redirecting the client means changing its configured NTP endpoint through
an explicitly authorized ADB setup operation**. It does not mean replacing Android
framework binaries, patching the system service, or transparently intercepting
arbitrary applications' packets. The Android system remains responsible for
applying time to its own clock.

| Workstream | Planning owner and status |
| --- | --- |
| Windows client | [Development execution plan](docs/plans/development-plan.md): current work, P0–P7, prerequisites and acceptance. This roadmap neither duplicates nor reorders that plan. |
| Android local SNTP bridge | This document, stages AND-0–AND-5. Future work with device-specific feasibility gates. |
| Android-to-Windows time service | AND-6 below. Retains the original companion-to-NevroTick idea as a separately gated future server extension. |

The [architecture](docs/windows-time-architecture.md),
[owner decisions](docs/nevrotick-owner-decisions.md) and
[documentation guide](docs/README.md) retain their existing authority. Windows v1
remains client-only; this roadmap does not add a Windows NTP/NTS listener to v1.
Existing Windows execution and its current fixture work continue independently.

At the reviewed baseline, portable Rust protocol/numerical fixtures are reuse
candidates, not a production Android or NTS engine. Preserve Chrony-derived
provenance and GPL-2.0-only obligations. Admit any Android/JNI, TLS, AEAD, runtime
and build dependencies separately. The Windows x64 CPU policy in OD-07 does not
apply to Android ABIs or impose x86 compiler flags on shared portable code.

## 2. Intended behavior and limits

The target behavior is:

1. Install an ordinary Android APK.
2. Start a local SNTP responder bound exclusively to an explicit loopback address
   and an unprivileged UDP port, initially `127.0.0.1:10123`.
3. Obtain and validate remote time through NTS, maintaining a local estimate with
   an explicit age, uncertainty and authentication status.
4. After a readiness check, use ADB to configure Android's system NTP service to
   query that local endpoint.
5. Let Android's existing time services accept the sample and decide whether to
   adjust the system clock.
6. Provide a verified ADB restoration procedure before planned disabling or
   uninstalling the companion.

No root, bootloader unlocking, VPN/TUN interface, packet-redirection rules,
Device Owner enrollment, direct `SET_TIME` permission or persistent Shizuku
bridge is part of this baseline. If the baseline is unsupported, report that
result instead of silently introducing a different privilege model.

### Proposed responsibilities

These are logical owners, not a decision to create a particular crate/module tree.

| Owner | Inputs and result | Boundary |
| --- | --- | --- |
| Android UI and lifecycle adapter | User intent and platform events; local service state and diagnostics | Ordinary app identity; no arbitrary shell or direct system-clock API. Select Kotlin/Java and any Rust bridge after the feasibility stage. |
| NTS acquisition and estimate | Validated source configuration and authenticated observations; bounded UTC estimate anchored to monotonic time | Owns source identity, keys/cookies, freshness and uncertainty; never accepts silent plain-NTP fallback. |
| Local SNTP responder | Bounded client requests; correctly correlated time replies only while the estimate is eligible | Explicit loopback bind; no LAN listener, public service or generic proxy. |
| ADB setup/restoration tool | Selected device, captured original configuration and fixed validated operations; readback and restoration receipt | Temporary user-authorized shell access; no permanent ADB connection is required for the intended Android 14+ runtime. |
| Android time services | Local SNTP response; network-time suggestion and policy-dependent clock effect | Source selection, thresholds and actual clock actuation remain Android/OEM behavior. |
| Optional Windows responder | Accepted Windows-engine estimate; independently authenticated time service for the companion | Future AND-6 component; incoming traffic and TLS never enter the privileged Windows clock broker. |

### Honest security and accuracy claims

- NTS authenticates the companion's remote exchange. The last loopback SNTP hop
  does not authenticate the APK to Android, and Android does not verify an NTS
  proof. Another local app may occupy the configured high port while the
  companion is absent. Exclusive binding without port reuse and checking
  ownership before activation reduce exposure; they do not create authenticated
  local IPC. Evaluate this residual risk before a supported release.
- A failure in the companion must prevent a healthy synchronized reply from
  that companion. It does not guarantee that the whole OS rejects other time:
  Android may retain an older network sample or use telephony/another origin.
  Inspect and describe the actual firmware policy. [A1, A4]
- AOSP derives its network uncertainty from the SNTP round-trip measurement.
  With a local bridge that measurement can be tiny even when upstream
  uncertainty or holdover drift is large. Track those errors independently;
  truthful root-delay/dispersion fields alone do not fix this AOSP limitation.
  Never advertise Android's reported uncertainty as end-to-end accuracy. [A2, A3]
- A live listener, an authenticated observation, an eligible estimate, Android's
  acceptance of a fresh sample and a system-clock change are different states.
  Diagnostics must distinguish them; an unchanged clock can be correct when the
  adjustment threshold has not been crossed.

## 3. Compatibility assumptions to prove

The first physical-device target is the **Redmi Note 12 Pro family**. Exact
4G/5G variant, Android version, regional MIUI/HyperOS build and current behavior
remain unconfirmed. Keep serials and other unique device identifiers out of
public test receipts.

| Platform evidence | Planning consequence |
| --- | --- |
| AOSP Android 14 supports a persistent `ntp_server` value with an `ntp://host:port` URI. [A2] | Start qualification with Android 14+; this is a candidate baseline, not a declared minimum supported version or proof of Xiaomi compatibility. |
| The inspected Android 13 implementation has a persistent hostname setting, but obtains a non-default port through an in-memory test override. [A5] | Allow a bounded Android 13 experiment; do not promise persistence or unattended post-reboot support through that override. |
| Shell service commands vary across releases. [A1, A6] | Read the device's actual `help`; choose a known version adapter and reject unsupported output/operations. |
| AOSP checks that an active network is connected even when the server is on loopback. [A2] | Test Wi-Fi, mobile, transitions and no-network state. Offline failure is not by itself evidence of a broken UDP server. |
| Android and Xiaomi impose background-start, foreground-service and power restrictions. [A7, A8] | Prove lifecycle behavior on the selected firmware; a successful foreground demonstration is not background qualification. |

Do not append public/plain NTP servers to the redirected setting as an invisible
fallback. The protected remote acquisition belongs to the companion. A separate
explicit plain profile, if ever requested, needs its own policy and visible trust
label; it must not be entered automatically on NTS failure.

## 4. Ordered Android stages

All implementation and runtime checkboxes below are **open**. The current request
creates the plan; it does not establish a built APK, device test or release.

### AND-0 — prove the system-to-loopback path

**Owner:** Android platform implementer, with an independent observer.  
**Outcome:** determine whether the exact firmware supports the intended path
before committing to the full application.

- [ ] Record exact model/variant, Android/API level, build, target SDK of the
  test app, automatic-time state and relevant shell command capabilities.
- [ ] Inspect the existing persistent NTP setting and any active test override.
  Capture presence separately from value; do not treat printed `null` as a
  lossless backup format.
- [ ] Build the smallest ordinary-UID test APK with a loopback-only high-port
  SNTP listener. Do not use a shell-UID server as the only proof of app support.
- [ ] First test protocol behavior in an emulator/test device. On a daily-use
  phone, read-only discovery precedes any explicit configuration change; do not
  inject fictitious dates or large clock jumps.
- [ ] Apply a reversible temporary server override using the device's supported
  command, with an active network and a known-valid time source.
- [ ] Observe a real system-service request at the intended endpoint, a fresh
  accepted sample, and its submission to `time_detector`. Observe the selected
  source and either the intended clock effect or its valid no-adjustment reason.
- [ ] Challenge occupied port, absent listener, malformed/unsynchronized reply,
  no active network and an unsupported command. Confirm bounded failure and
  restoration without replacing someone else's configuration.

**Exit AND-C01:** retain exact APK/source/device identity, relevant redacted
service state, fresh sample evidence, negative controls and restoration readback.
Exit code zero from `force_refresh` alone is insufficient: the command can print
`false` while returning zero. A previous cached sample is not a fresh success.
If the ordinary-APK path fails, record the unsupported case; do not progress by
adding root, VPN or a permanent shell bridge.

### AND-1 — implement the bounded local SNTP responder

**Depends on:** AND-C01.  
**Outcome:** a protocol-correct local server backed by an injected estimate;
no claim of authenticated production time yet.

- [ ] Support the actual Android request format, including NTP version 3,
  client mode 3 and its 48-byte base packet. Decide any additional version or
  extension support explicitly; NTS upstream uses a separate association. [A3]
- [ ] Echo the original request transmit timestamp into the reply's originate
  field **bit for bit**. Do not lose Android's randomized submillisecond bits
  by decoding to milliseconds and reconstructing it.
- [ ] Produce mode-4 replies with valid, truthful reference, receive and transmit
  timestamps, leap/stratum/root fields and era interpretation.
- [ ] Derive served time from a validated UTC estimate plus a monotonic clock
  that includes suspend. Avoid feeding the same adjustable Android wall clock
  back into its own source. Reject cross-boot monotonic anchors.
- [ ] Define maximum estimate age and uncertainty before using them as gates.
  Ineligible state yields no synchronized success response; verify the selected
  refusal behavior against Android.
- [ ] Bound datagram size, allocations, parser work, queue, response rate and
  aggregate local-client load. Reject invalid modes/lengths; provide no control,
  discovery or reflection/amplification service.
- [ ] Keep explicit IPv4 loopback binding initially; any IPv6 support needs its
  own bind and test. Never fall back to wildcard/LAN binding after an error.

**Exit AND-C02:** independent packet vectors and native app socket observations
cover healthy neighbors, exact correlation, hostile packets, startup conflicts,
aging, suspend and wall-clock changes. A mock/client-only test does not establish
acceptance by Android's system service.

### AND-2 — connect authenticated NTS acquisition

**Depends on:** AND-C02 and the admitted Android dependency graph.  
**Outcome:** eligible remote time reaches the local responder through one checked
time model.

- [ ] Evaluate reuse of the portable NevroTick protocol/numerical owners and the
  planned NTS client. Preserve source provenance, license obligations and
  function/module contracts; qualify Android ABIs and FFI boundaries separately.
- [ ] Select and verify Android TLS/AEAD, trust-store, key/cookie storage and
  execution owners. Existing Windows provider/runtime decisions do not select
  Android implementations.
- [ ] Implement TLS 1.3+ NTS-KE with verified server identity, protocol-compliant
  key/cookie handling and authenticated NTPv4 exchanges; bound retries and
  concurrent work. Admit complete validated results atomically. [A9]
- [ ] Define source policy consistently with the canonical NTS/source-selection
  rules. A deliberately single-source laboratory fixture is not the production
  quorum/default profile. Do not ship unresolved provider presets.
- [ ] Feed only currently eligible authenticated observations into the estimate;
  preserve units, source generation, boot identity, freshness and uncertainty.
  Do not initiate an uncontrolled upstream query for every local UDP request.
- [ ] On TLS/authentication/UDP failure, retain only still-eligible state and
  eventually refuse synchronized replies. Never downgrade to plain NTP.
- [ ] Qualify certificate-time bootstrap before automatic recovery claims. A
  bounded prototype may stop and request correction of an obviously wrong
  approximate date; it must not disable certificate verification or set the OS
  clock from an unauthenticated hint. Preserve the existing recovery research as
  a separately qualified path.

**Exit AND-C03:** controlled independent NTS-peer tests and real Android
integration cover valid responses, wrong identity, corrupt tags, replay, stale
cookies, clock/era ambiguity, cancellation, network replacement and upstream
outage. Public NTS providers are not stress-test targets.

### AND-3 — make ADB activation and restoration reliable

**Depends on:** AND-C03 for normal activation; AND-0 retains its bounded fixture
exception.  
**Outcome:** enable and undo the system endpoint change without hidden privileges.

- [ ] Select the target device explicitly and verify the APK package/signing
  identity. Use fixed command arguments and validated values, never a general
  user-supplied shell-command surface.
- [ ] Store the original setting's presence and exact restorable representation,
  the selected device association, and any test override. Filter captured state
  to required fields; do not publish a full settings dump or device serial.
- [ ] Refuse mutation if the original representation or an existing override
  cannot be safely understood/restored. A literal `null`, nullable stored value
  and absent key must not be conflated. [A10]
- [ ] Verify exclusive listener readiness and an eligible authenticated estimate
  before installing a valid persistent local-only URI.
- [ ] Read back both the saved setting and the service's **effective** server
  configuration. A test override can mask the persistent setting.
- [ ] Verify a fresh system-service sample after activation. Restore on setup
  failure while the authorized ADB host remains available.
- [ ] Make repeated setup/restoration idempotent and detect conflicting edits.
  Recheck expected values before mutation; the CLI does not provide an atomic
  compare-and-swap guarantee. Never overwrite a known later user/admin change.
- [ ] Before planned stop/uninstall, restore the saved value, or delete the key
  only if it was originally absent; remove only test overrides owned by this
  setup. Verify effective configuration and any fresh restored-source sample.
- [ ] Provide host-side recovery instructions that work even after the APK is
  gone. Explain that removing an override does not itself erase cached samples
  or undo an already-applied clock change.

**Exit AND-C04:** successful and interrupted setup, wrong/multiple devices,
pre-existing override, absent/custom/ambiguous original values, concurrent edits,
APK upgrade and planned uninstall all have verified outcomes. The normal APK
cannot guarantee automatic restoration after force-stop, crash or uninstall;
do not promise it. After successful persistent setup, disconnect ADB and prove
that runtime synchronization does not depend on that connection.

### AND-4 — qualify background service and power behavior

**Depends on:** AND-C04.  
**Outcome:** define a supported operating envelope rather than claiming an
invisible permanent daemon.

- [ ] Choose a legitimate foreground-service type, permissions, notification
  behavior and launch path for the actual target SDK. `dataSync` is not an
  assumed perpetual-listener solution: Android 15+ target rules impose time
  budgets; `specialUse` needs a justified use case and, for Google Play,
  review. Do not assume acceptance. [A7]
- [ ] Test Doze, App Standby and OEM restrictions independently. A foreground
  service or wake lock does not by itself establish unrestricted network/CPU
  access. Determine which supported user-granted battery/autostart settings are
  necessary and document their actual effect. [A7, A8]
- [ ] Test screen-off/unplugged operation, applicable service timeouts,
  notification denial, low-memory process death, user stop/force-stop, package
  upgrade, reboot and before/after first unlock.
- [ ] On restart, reconcile actual binding/configuration and invalidate old
  monotonic anchors. Handle persistent endpoint configuration while the APK is
  temporarily unavailable; do not report healthy service during that gap.
- [ ] Measure bounded memory, wakeups, CPU, battery and network use. Start with
  a bounded overnight test and extend only for a concrete lifecycle/resource
  question; no continuous wake lock as a substitute for a design.
- [ ] Document observed loss of service, cached Android time and fallback
  behavior. Do not promise fail-closed behavior of the entire Android OS.

**Exit AND-C05:** reproducible lifecycle evidence on each claimed firmware row,
including recovery and consumption measurements. If reliable background service
cannot be achieved under the agreed constraints, retain the foreground experiment
as such; do not label that device supported for unattended synchronization.

### AND-5 — deliver a usable Android preview

**Depends on:** AND-C01–AND-C05 for each supported row.  
**Outcome:** a signed, documented preview with truthful setup, state and recovery.

- [ ] Provide setup status, upstream identity/authentication, observation age,
  uncertainty, listener state, last known Android acceptance and actionable errors.
  Where privileged diagnostic access is unavailable, display Android acceptance
  as unknown/last verified rather than claiming live observation.
- [ ] Include a guided activation and restoration workflow; explain the need for
  ADB when restoring global settings and the consequences of forced removal.
- [ ] Qualify application signing/update identity, dependency inventory,
  license/source delivery, secret exclusion from backup/export and redacted
  support diagnostics. Android APK signing is separate from Windows MSI signing.
- [ ] Maintain a compatibility table with exact model/variant/build, Android and
  target SDK, required settings, observed gates and remaining limitations.
- [ ] Publish only claims supported by the identified candidate. Distribution
  channel, any store policy and release approval remain explicit release work.

**Exit AND-C06:** exact signed-APK installation, upgrade, rollback/restoration and
uninstall evidence; completed selected compatibility rows and independently
reviewed trust claims. A successfully built APK is not this exit.

### AND-6 — optionally serve time from NevroTick on Windows

**Depends on:** Android acquisition/consumer qualification and the relevant
qualified Windows engine, plus a separately adopted server contract.  
**Outcome:** preserve the originally discussed phone-to-PC use case without
turning it into a hidden v1 requirement.

- [ ] Define an optional server/responder outside the privileged clock broker,
  consuming the engine's accepted time estimate and synchronization state.
- [ ] For an NTS-protected phone-to-PC path, implement and qualify an actual
  NTS-KE/NTP server, server identity/certificate lifecycle, destination/port
  policy, firewall exposure, rate limits and failure semantics. An NTS client
  alone cannot provide that service.
- [ ] Verify pairing/trusted server identity, PC sleep/offline/reboot, stale
  estimate rejection and changes in reachability when the phone leaves the LAN.
  No automatic public-port exposure.
- [ ] Distinguish this from an ordinary LAN SNTP gateway: protected upstream
  acquisition at the PC does not authenticate a plain-NTP PC-to-phone hop.
- [ ] Keep direct Android-to-NTS operation available as an independent design
  path; do not require a permanently reachable Windows computer for every mode.

**Exit AND-C07:** separately reviewed and adopted server architecture plus actual
two-device authenticated interoperability and lifecycle evidence. Windows v1's
client-only contract is changed only by the explicit later scope decision.

## 5. Command reference for the feasibility stage

These are **version-scoped examples**, not an instruction to change a phone
before the listener, captured configuration and restoration path exist.

Read-only discovery (replace the serial placeholder locally; keep it private):

~~~text
adb -s DEVICE_SERIAL shell getprop ro.product.model
adb -s DEVICE_SERIAL shell getprop ro.build.version.release
adb -s DEVICE_SERIAL shell getprop ro.build.version.sdk
adb -s DEVICE_SERIAL shell settings get global ntp_server
adb -s DEVICE_SERIAL shell settings get global auto_time
adb -s DEVICE_SERIAL shell cmd network_time_update_service help
adb -s DEVICE_SERIAL shell dumpsys network_time_update_service
adb -s DEVICE_SERIAL shell dumpsys time_detector
~~~

Capture setting presence separately using a filtered listing/provider read; the
`get` result alone is not a complete restoration record. Dumpsys output may
contain sensitive context: retain only the fields needed for qualification.

Modern AOSP temporary override, **after** AND-0 preflight and listener startup:

~~~text
adb -s DEVICE_SERIAL shell cmd network_time_update_service set_server_config_for_tests --server ntp://127.0.0.1:10123 --timeout_millis 5000
adb -s DEVICE_SERIAL shell cmd network_time_update_service force_refresh
~~~

Remove the temporary override created by this experiment:

~~~text
adb -s DEVICE_SERIAL shell cmd network_time_update_service reset_server_config_for_tests
~~~

Verified Android 13 test syntax uses
`set_server_config --hostname 127.0.0.1 --port 10123 --timeout_millis 5000`;
its reset is `set_server_config` without options. It is not a persistent
high-port deployment method. [A5, A6]

The Android 14+ **persistent candidate**, only after the activation gate, is:

~~~text
adb -s DEVICE_SERIAL shell settings put global ntp_server ntp://127.0.0.1:10123
~~~

Restore the exact captured value through the version-aware tool; when the key
was confirmed originally absent, restoration uses
`adb -s DEVICE_SERIAL shell settings delete global ntp_server`.
Do not substitute a hardcoded public server, blindly write `null`, reset an
unrelated test override or interpret setting readback as proof of synchronization.

`force_refresh` can submit a clock-affecting suggestion when automatic time is
enabled. Check its printed boolean and fresh service state asynchronously;
do not infer success from its exit code. USB ADB `reverse` is not an automatic
UDP tunnel; this design uses local UDP and the companion's own remote transport.
[A1, A6, A11]

## 6. Decision points and evidence discipline

The next implementation stage is **AND-0**, starting with device identification
and read-only capability discovery. Dates for later stages are intentionally not
promised before that result.

Resolve these choices when the dependent stage is ready, using the existing
[owner-decision procedure](docs/nevrotick-owner-decisions.md#6-od-04--decisions-when-evidence-makes-them-concrete):

- Supported Android versions/builds and the reliability claim after the probe.
- Android dependency/provider/runtime choice, FFI and ABI support.
- Numerical age/uncertainty/holdover and production source-policy parameters.
- Admissible FGS/lifecycle/distribution design and residual local-port
  impersonation risk.
- Any requirement that the whole OS prohibit alternate time origins.
- Whether to add the separately exposed Windows NTS server in AND-6.

Do not let an unresolved later choice block read-only discovery or a controlled
earlier fixture. Do not silently resolve a security-policy exception while
implementing an earlier stage.

AND-C01–AND-C07 are local roadmap exit IDs, not replacements for existing
REQ/BND/DEC/THR/CHECK IDs. Apply the relevant canonical protocol, trust,
input-contract, provenance, diagnostics and resource obligations; do not claim
Windows-native checks qualify Android behavior.

At implementation start, create/reuse one stage task in `tasks/` and link it
from this roadmap. Record candidate commit/APK hash, exact environment, inputs,
independent expected outcomes, actual results, restoration, open findings and
next action. Repository planning/security/testing skills guide the applicable
boundaries; add Android-specific guidance only when an actual gap requires it.
Documentation review and upstream source evidence are not device-test passes.

## 7. Primary evidence

Reviewed for this roadmap on 30 October 2026. Versioned AOSP sources support the
specific version claims; mutable current sources and Android/OEM policies must
be rechecked for the eventual target build. No cited source establishes tested
support for the user's particular handset.

- **A1 — Android network time:** [network time detection](https://source.android.com/docs/core/connect/time/network-time-detection).
- **A2 — endpoint parsing, connectivity and estimate uncertainty:** [AOSP Android 14 NtpTrustedTime](https://github.com/aosp-mirror/platform_frameworks_base/blob/android-14.0.0_r1/core/java/android/util/NtpTrustedTime.java) and [current source](https://android.googlesource.com/platform/frameworks/base/+/HEAD/core/java/android/util/NtpTrustedTime.java).
- **A3 — real SNTP wire behavior:** [AOSP SntpClient](https://android.googlesource.com/platform/frameworks/base/+/refs/heads/main/core/java/android/net/SntpClient.java).
- **A4 — system source priority:** [Android time source priority](https://source.android.com/docs/core/connect/time-source).
- **A5 — Android 13 compatibility:** [NtpTrustedTime](https://github.com/aosp-mirror/platform_frameworks_base/blob/android-13.0.0_r1/core/java/android/util/NtpTrustedTime.java) and [NetworkTimeUpdateServiceShellCommand](https://github.com/aosp-mirror/platform_frameworks_base/blob/android-13.0.0_r1/services/core/java/com/android/server/NetworkTimeUpdateServiceShellCommand.java).
- **A6 — current test/refresh commands:** [NetworkTimeUpdateServiceShellCommand](https://android.googlesource.com/platform/frameworks/base/+/refs/heads/main/services/core/java/com/android/server/timedetector/NetworkTimeUpdateServiceShellCommand.java).
- **A7 — Android background operation:** [FGS types](https://developer.android.com/develop/background-work/services/fgs/service-types), [timeouts](https://developer.android.com/develop/background-work/services/fgs/timeout), [background-start restrictions](https://developer.android.com/develop/background-work/services/fgs/restrictions-bg-start), and [Doze/App Standby](https://developer.android.com/training/monitoring-device-state/doze-standby).
- **A8 — Xiaomi controls, not model-specific qualification:** [HyperOS autostart policy](https://dev.mi.com/xiaomihyperos/documentation/detail?pId=1624) and [Xiaomi background-operation guidance](https://www.mi.com/global/support/faq/details/KA-700752/).
- **A9 — NTS protocol and security limits:** [RFC 8915](https://www.rfc-editor.org/rfc/rfc8915.html), especially certificate-time bootstrap, delay attacks and downgrade resistance.
- **A10 — settings output/representation:** [SettingsService](https://github.com/aosp-mirror/platform_frameworks_base/blob/main/packages/SettingsProvider/src/com/android/providers/settings/SettingsService.java) and [SettingsState](https://github.com/aosp-mirror/platform_frameworks_base/blob/main/packages/SettingsProvider/src/com/android/providers/settings/SettingsState.java).
- **A11 — ADB transports and commands:** [official ADB man page](https://android.googlesource.com/platform/packages/modules/adb/+/refs/heads/main/docs/user/adb.1.md).
