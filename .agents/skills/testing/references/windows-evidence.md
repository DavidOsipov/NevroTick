# Windows evidence layers

Use the canonical [architecture](../../../../docs/windows-time-architecture.md) for selected accounts, endpoints, clock APIs, package lifecycle and performance budgets. This file routes evidence; it does not authorize changing the host.

## Native execution and isolation

Record Windows edition/build, architecture, toolchain/SDK/linker, service identities, security profile, feature set, candidate hashes and VM/hardware characteristics. Windows-target compilation on WSL establishes only compilation for the selected configuration. A VM can exercise services/ACLs/packages but may distort oscillator, suspend and resource measurements; name these limits. Use named hardware where CHECK-21 requires it.

For a disposable clock/service/installer test machine, capture owned baseline service/W32Time configuration, clock state, package state and recovery conditions. Ensure cleanup can run after service/process death; a test process destructor cannot clean up after an abort. Account for other-user sessions and successors before restoring state. Avoid scripts that blindly reset a rate now owned by another controller.

## Tokens, IPC and effects

Exercise real engine/broker/UI/helper tokens and their allowed verbs (CHECK-1–3). Include standard user, filtered/elevated admin, delegated/revoked application grants, old sessions, remote clients and a fake server. A disabled button or denied constructor is not the endpoint authorization test. Inspect the actual named-pipe ACL/server identity and attempt protected effects with valid-shaped requests.

Observe the true Windows result: applied sign/rate/phase, quantization, privilege behavior and acknowledgement. Include cancellation before/during native work, native call failure, delayed completion, stale generations, lost acknowledgement and cumulative budgets. Verify no unwinding crosses FFI and no replacement permit is fabricated while cancelled work still owns one (CHECK-10/27).

Exercise actual SCM start/stop pending and running status, failure/preshutdown controls, registration failure, restart and no-login startup. UI exit must not stop the background service. Handle/thread/resource counts belong to their actual owner, not a global count whose meaning is unclear.

## Installation and delivered artifacts

V1 is one complete signed per-machine MSI. Validate its actual component/service inventory, privileges, fixed paths, PE/runtime mitigations, dependency DLL loading and signature/publisher identity. Test install, update, repair, downgrade rejection, uninstall without UI, interrupted stages, disabled rollback, reboot, hostile properties/paths and concurrent/old-generation cleanup (CHECK-13/16/18/23).

Inspect external system effects and journal/restoration state before and after each fault. Windows Installer rollback does not automatically reverse arbitrary service/system changes from custom actions. Validate those compensations, including mid-action interruption and rollback-disabled behavior. Correct prior W32Time state and successor ownership must survive repair/removal; do not fight managed policy (CHECK-5).

An unsigned/test-signed P0 artifact may demonstrate its declared mechanics in isolation. It does not establish production publisher trust, final signed-byte identity or clean-machine release acceptance. TUF research and a possible future MSIX are not adopted product implementations.

## Native UI, accessibility and presentation

The current candidate is Win32 common controls via windows/windows-sys, not a webview. Use UI Automation properties/control patterns/events with keyboard and actual Narrator output. Prefer available Accessibility Insights for Windows; Inspect is a legacy diagnostic fallback. Do not assert application accessibility solely because controls are native (CHECK-22).

Check critical status/source/settings/diagnostics/elevation/tray flows, clear trust/freshness/uncertainty, stale snapshot behavior, accessible error association, tab order, visible focus, Escape/cancel and focus return after dialogs/UAC. Distinguish quitting UI from stopping synchronization; tray-only access must not be required.

Exercise contrast themes, system text scaling, at least 200% display scaling, moving between different-DPI monitors and changing DPI while running. A manifest declaration or screenshot at one DPI cannot prove correct change handling. Check long/localized/non-ASCII/bidi labels, keyboard access and stable focused row order. Rate-limit meaningful announcements; do not narrate every offset update.

English is the initial adopted locale; Russian is conditional on a later explicit locale decision. Locale never alters wire/policy IDs, units, sign or authorization. UIA assertions, Narrator observations and screenshots are separate evidence. Apply WCAG2ICT as guidance for relevant WCAG criteria without claiming a standalone certification.
