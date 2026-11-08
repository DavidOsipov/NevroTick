# Windows evidence layers

Use canonical [architecture](../../../../docs/windows-time-architecture.md): selected accounts/endpoints/clock APIs/package lifecycle/performance budgets. Evidence routing grants no host-change authority.

## Native execution and isolation

Record Windows edition/build, architecture, toolchain/SDK/linker, service identities, security profile, features, candidate hashes, VM/hardware characteristics. Windows-target WSL compilation proves only selected-configuration compilation. VMs exercise services/ACLs/packages; may distort oscillator/suspend/resource measurements. Name limits; use CHECK-21-required named hardware.

When an OS advisory is relevant, bind its vendor affected/fixed range to the
actual component, edition/build/UBR, architecture and servicing/KB state of each
selected target. Family names, package signatures and database no-match results
cannot establish a patched baseline; retain unresolved applicability explicitly.
Reuse the existing two-family qualification matrix, not a new universal CVE gate.

Disposable clock/service/installer machine: capture owned baseline service/W32Time configuration, clock/package state, recovery conditions. Ensure cleanup after service/process death; destructor cannot clean after abort. Account for other-user sessions/successors before restoration. No blind rate reset under successor controller ownership.

## Tokens, IPC and effects

Exercise real engine/broker/UI/helper tokens/allowed verbs (CHECK-1–3). Include standard user, filtered/elevated admin, delegated/revoked application grants, old sessions, remote clients, fake server. Disabled button/denied constructor is no endpoint authorization test. Inspect actual named-pipe ACL/server identity; attempt valid-shaped protected-effect requests.

For the two runtime services, use
[architecture §4.1.1](../../../../docs/windows-time-architecture.md#411-service-privilege-profiles)
and [the profile matrix](../../../../docs/windows-time-development-hardening.md#service-privilege-profile-qualification).
Capture the whole primary-token privilege set, attributes, local LUID/name
mapping and process lifetime. Compare exact role policy: extra enabled/disabled,
unknown entry, missing requirement and incomplete/failed observation deny normal
work beside a useful intended-token control. Three forbidden names or enabled-only
inspection do not prove a full allowlist. Distinguish configured names, service
restart, actual SID/token state and object/owner rights. Inspect identity-only
client capture and checked reversion separately; a client token is not a service
profile. Broker enable/use/disable, `ERROR_NOT_ALL_ASSIGNED` and uncertain
cleanup need their own paired controls under existing clock/recovery ownership.
Use current held/admitted execution status; do not perform a clock denial probe
on a token already observed to retain forbidden clock privilege.

Observe the true Windows result: applied sign/rate/phase, quantization, privilege behavior and acknowledgement. Include cancellation before/during native work, native call failure, delayed completion, stale generations, lost acknowledgement and cumulative budgets. Verify no unwinding crosses FFI and no replacement permit is fabricated while cancelled work still owns one (CHECK-10/27).

Exercise actual SCM start/stop pending/running status, failure/preshutdown controls, registration failure, restart, no-login startup. UI exit must not stop background service. Attribute handle/thread/resource counts to actual owner; unclear global counts insufficient.

## Certificate and crypto-provider evidence

For reached TLS/certificate/provider paths, use
[R-05](../../../../docs/nevrotick-research-handoff.md#7-r-05--windows-trust-with-a-wrong-system-date)
and the [S2.2 native-work card](../../../../tasks/s0.1-crypto-feasibility.md#ordinary-s22-trust-profile-and-native-work--5-november-2026)
under CHECK-8/9/26/27. Bind observations to the actual service principal, verifier/
provider build, effective stores, candidate and system time, retrieval settings
and observed accepted/unknown/refused result. Use hardening
[§9.11's paired retrieval controls](../../../../docs/windows-time-development-hardening.md#911-windows-integration-qualification-refinements);
attribute direct and delegated traffic separately. Delayed native completion
retains inputs/resources and admission charges while stale results stay ineligible.
Use the existing CNG initialization/shutdown cases at their actual owner;
an isolated initializer pass or process exit proves no broader cleanup guarantee.

## Native memory and resource ownership

For changed or suspect heap, handle, FFI or callback ownership, select
[Windows memory diagnostics](windows-memory.md). Application Verifier/PageHeap
with CDB observes reached native faults; repeated allocation, kernel-handle,
GDI/USER-object, thread and internal task/permit measurements address retention.
Record allocator/API coverage, actual test token, effective per-image settings
and verified restoration. Keep instrumented runs separate from the uninstrumented
hardware budgets and sustained recovery/soak obligations in CHECK-21. Native tool
controls establish no product memory qualification or permission for extra host
effects; UI-M0 retains its fixture-only scope and D1/D2/D3 deferrals.

## Installation and delivered artifacts

The selected target is one complete hardened MSIX prepared for David's Store trial with services under [architecture §19.4](../../../../docs/windows-time-architecture.md#194-future-complete-msix-gate) and [OD-05](../../../../docs/nevrotick-owner-decisions.md#7-od-05--signing-and-release-ownership). Validate actual component/service inventory, supported pre-start provisioning, effective privileges, immutable code/data separation, PE/runtime mitigations, DLL loading and package/release/helper identity. Use [MSIX plan §3](../../../../docs/plans/windows-msix-store.md#3-package-trust-and-the-privileged-helper) for the exact integrity/CIG/per-role profile. Neither the virtual-account reference design nor two LocalService processes is a qualified MSIX mapping merely because the direction is selected.

Follow [MSIX plan §7](../../../../docs/plans/windows-msix-store.md#7-bounded-continuation-and-existing-acceptance-owners):

1. Inspect the actual local candidate, then qualify test-signed mechanics in the owned disposable Windows environment when prerequisites permit. Record substituted trust and which CIG/catalog/activation claims await genuine Store bytes. Current Core priority, Windows execution stop, separate fuzz hold and storage pause are unchanged.
2. Prepare the complete package and truthful certification notes for David's Store trial. Prior approval or a Store signature cannot be required before the first submission. The owner performs account/submission/publication actions; local mechanics establish no certification or broad-release pass.
3. Inspect the actual Store-delivered artifact before broad release. Record package/publisher/release identity, manifest/catalog/images and final hashes separately from submitted bytes. Observe allowed imports/provider workflows and denied/tampered/wrong-project neighbors under the selected enforced CIG profile, with early-loader/SCM activation limits. A Store-signed unrelated binary can pass a signer-class policy while remaining unauthorized for NevroTick helper/IPC operations.

Test install, startup before login, update, repair/Reset, downgrade rejection, removal before first UI launch, other-user/last-user transitions, interrupted stages/reboot, hostile lifecycle inputs/paths and old-generation cleanup (CHECK-13/16/18/23). Inspect external effects and journal/restoration state before and after each fault. MSIX transactions, service stop and file removal do not prove application-owned recovery; the last necessary recovery code/record must survive until a durable authorized outcome. Correct prior W32Time state and successor ownership must survive maintenance/removal; do not fight managed policy (CHECK-5).

Apply [the required shipped-artifact signature inventory](../../../../docs/windows-time-signing-guide.md#41-required-signatures-for-the-shipped-inventory) on every route, including supported nonbinary files and nested PE, in addition to package trust. Verify expected project/upstream identities and actual post-Store inner signatures; missing capability blocks release, not source preparation. MSI delivery remains a later owner choice, with its own custom-action/rollback evidence. The unsigned-MSI/OpenPGP exception is superseded; manual detached verification supplies no substitute. SignPath is only a possible signer, including for MSIX inner files. TUF/custom-updater research remains unadopted; certification never waives technical evidence.

## Native UI, accessibility and presentation

[OD-13](../../../../docs/nevrotick-owner-decisions.md#17-od-13--slint-as-the-default-ui-toolkit) selects Slint as the default and complete Win32 UI replacement. The checked-in Windows entrypoint selects Slint; retained Win32 source and receipts are historical. [S0.5](../../../../tasks/s0.5-ui-comparison.md) and the [GUI handoff](../../../../tasks/slint-gui-handoff.md) own current scope/evidence and remaining native dependencies. The measured winit/software/Fluent pilot does not qualify a product feature graph. Identify the actual Slint accessibility feature/bridge and inspect UI Automation properties/control patterns/events with keyboard and actual Narrator output. Prefer available Accessibility Insights for Windows; Inspect is a legacy diagnostic fallback. Slint-drawn widgets provide no inherited Win32 child-control accessibility pass (CHECK-22). The pilot's disabled accessibility feature is not the product policy; preserve UI-M0's D1/D2/D3 deferrals until their existing task re-entry gates.

Check critical status/source/settings/diagnostics/elevation/tray flows, clear trust/freshness/uncertainty, stale snapshot behavior, accessible error association, tab order, visible focus, Escape/cancel and focus return after dialogs/UAC. Distinguish quitting UI from stopping synchronization; tray-only access must not be required.

Exercise contrast themes, system text scaling, at least 200% display scaling, moving between different-DPI monitors and changing DPI while running. A manifest declaration or screenshot at one DPI cannot prove correct change handling. Check long/localized/non-ASCII/bidi labels, keyboard access and stable focused row order. Rate-limit meaningful announcements; do not narrate every offset update.

Initial adopted locale English; Russian requires later explicit decision. Locale never changes wire/policy IDs/units/sign/authorization. UIA assertions/Narrator observations/screenshots: separate evidence. Apply WCAG2ICT guidance for relevant WCAG criteria; no standalone certification claim.
