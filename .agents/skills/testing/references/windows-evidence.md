# Windows evidence layers

Use canonical [architecture](../../../../docs/windows-time-architecture.md): selected accounts/endpoints/clock APIs/package lifecycle/performance budgets. Evidence routing grants no host-change authority.

## Native execution and isolation

Record Windows edition/build, architecture, toolchain/SDK/linker, service identities, security profile, features, candidate hashes, VM/hardware characteristics. Windows-target WSL compilation proves only selected-configuration compilation. VMs exercise services/ACLs/packages; may distort oscillator/suspend/resource measurements. Name limits; use CHECK-21-required named hardware.

Disposable clock/service/installer machine: capture owned baseline service/W32Time configuration, clock/package state, recovery conditions. Ensure cleanup after service/process death; destructor cannot clean after abort. Account for other-user sessions/successors before restoration. No blind rate reset under successor controller ownership.

## Tokens, IPC and effects

Exercise real engine/broker/UI/helper tokens/allowed verbs (CHECK-1–3). Include standard user, filtered/elevated admin, delegated/revoked application grants, old sessions, remote clients, fake server. Disabled button/denied constructor is no endpoint authorization test. Inspect actual named-pipe ACL/server identity; attempt valid-shaped protected-effect requests.

Observe the true Windows result: applied sign/rate/phase, quantization, privilege behavior and acknowledgement. Include cancellation before/during native work, native call failure, delayed completion, stale generations, lost acknowledgement and cumulative budgets. Verify no unwinding crosses FFI and no replacement permit is fabricated while cancelled work still owns one (CHECK-10/27).

Exercise actual SCM start/stop pending/running status, failure/preshutdown controls, registration failure, restart, no-login startup. UI exit must not stop background service. Attribute handle/thread/resource counts to actual owner; unclear global counts insufficient.

## Installation and delivered artifacts

V1 is one complete signed per-machine MSI. Validate its actual component/service inventory, privileges, fixed paths, PE/runtime mitigations, dependency DLL loading and signature/publisher identity. Test install, update, repair, downgrade rejection, uninstall without UI, interrupted stages, disabled rollback, reboot, hostile properties/paths and concurrent/old-generation cleanup (CHECK-13/16/18/23).

Inspect external system effects and journal/restoration state before and after each fault. Windows Installer rollback does not automatically reverse arbitrary service/system changes from custom actions. Validate those compensations, including mid-action interruption and rollback-disabled behavior. Correct prior W32Time state and successor ownership must survive repair/removal; do not fight managed policy (CHECK-5).

Isolated unsigned/test-signed P0 artifact may demonstrate declared mechanics; proves no production publisher trust/final signed-byte identity/clean-machine release acceptance. TUF research/possible future MSIX are unadopted.

## Native UI, accessibility and presentation

The current candidate is Win32 common controls via windows/windows-sys, not a webview. Use UI Automation properties/control patterns/events with keyboard and actual Narrator output. Prefer available Accessibility Insights for Windows; Inspect is a legacy diagnostic fallback. Do not assert application accessibility solely because controls are native (CHECK-22).

Check critical status/source/settings/diagnostics/elevation/tray flows, clear trust/freshness/uncertainty, stale snapshot behavior, accessible error association, tab order, visible focus, Escape/cancel and focus return after dialogs/UAC. Distinguish quitting UI from stopping synchronization; tray-only access must not be required.

Exercise contrast themes, system text scaling, at least 200% display scaling, moving between different-DPI monitors and changing DPI while running. A manifest declaration or screenshot at one DPI cannot prove correct change handling. Check long/localized/non-ASCII/bidi labels, keyboard access and stable focused row order. Rate-limit meaningful announcements; do not narrate every offset update.

Initial adopted locale English; Russian requires later explicit decision. Locale never changes wire/policy IDs/units/sign/authorization. UIA assertions/Narrator observations/screenshots: separate evidence. Apply WCAG2ICT guidance for relevant WCAG criteria; no standalone certification claim.
