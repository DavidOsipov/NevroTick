# Slint on Windows: ownership and integration

Read for native windows, handles, tray behavior, accessibility integration or
installed resources. Apply [the current UI contract](../../../../../docs/windows-time-architecture.md#161-toolkit-and-interaction-model)
and [UI-M0](../../../../../tasks/ui-m0.md) first. UI-M0 remains an unelevated,
fixture-only prototype; this reference activates no D1/D2/D3 or optional-tray work.

The source baseline is Slint **1.18.1**, commit
`372cf0ee5577c3dfec309a45e7b778ba4e81b734`. Recheck version-dependent behavior when
changing the admitted graph. Source review establishes no native Windows pass.

## Contents

- [Choose the narrowest native interface](#choose-the-narrowest-native-interface)
- [Own window and event-loop lifetime](#own-window-and-event-loop-lifetime)
- [Qualify optional tray recovery](#qualify-optional-tray-recovery)
- [Keep the UI in the user session](#keep-the-ui-in-the-user-session)
- [Re-enter DPI and accessibility at their gates](#re-enter-dpi-and-accessibility-at-their-gates)
- [Resolve installed resources explicitly](#resolve-installed-resources-explicitly)
- [Diagnose Windows-specific failures](#diagnose-windows-specific-failures)

## Choose the narrowest native interface

| Requirement | Interface and constraint |
| --- | --- |
| Show/hide, close requests, size, position, minimize | Prefer the public `slint::Window` API. Check applicable errors and coordinate units. |
| A Windows API needs an actual window handle | Consider `raw-window-handle-06`; resolve the current handle inside the reviewed Windows adapter. |
| A requirement needs winit-specific events or options | Consider `slint::winit_030` through `unstable-winit-030`; document why the stable interface is insufficient. |

Create components and run the Slint event loop on the same main thread. Keep
callbacks bounded; dispatch worker results through the event-loop handoff.
`slint::Window` and its `WindowHandle` are not `Send` or `Sync`. Do not manufacture
thread safety by extracting an integer HWND or adding an unsafe implementation.
See [the pinned threading and winit API](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/slint/lib.rs).

Native window creation is deferred: a handle may be unavailable until an event-loop
iteration after `show()`. Handle absence and platform errors; re-resolve after
window recreation instead of treating a cached HWND as lifetime authority.
[`Window::window_handle`](https://docs.slint.dev/latest/docs/rust/slint/struct.Window#method.window_handle)
is feature- and backend-dependent. Borrow only for the admitted native operation.

Use `WinitWindowAccessor::with_winit_window()` for temporary access. Retaining the
`Arc<winit::Window>` returned by `winit_window().await` can keep a supposedly hidden
window on screen. Use Slint's matching winit re-export; the `winit_030` module is
explicitly unstable across minor Slint releases. Avoid direct internal-backend
dependencies as a shortcut. See [the pinned accessor contract](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/backends/winit/lib.rs).

Return `EventResult::Propagate` unless a specific event is intentionally consumed.
The [pinned winit dispatcher](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/backends/winit/winitwindowadapter.rs)
returns on `PreventDefault` before AccessKit and normal Slint handling. A broad
filter can suppress input, focus, resizing and lifecycle delivery. Native hooks
need narrowly specified ownership, error handling and restoration.

## Own window and event-loop lifetime

Keep the application's authoritative component owner alive until intentional
shutdown. Capture weak component handles in registered callbacks. Do not use
process exit to conceal cycles, retained native windows or unfinished workers.
Refer to [lifecycle verification](testing-and-performance.md#observe-lifecycle-and-resource-cost).

Preserve the [product interaction contract](../../../../../docs/windows-time-architecture.md#161-toolkit-and-interaction-model):
use a small ordinary window with progressive disclosure. Fullscreen demos,
animated dashboards and permanent redraw loops are not product requirements.

| User action or state | Required distinction |
| --- | --- |
| Close window, tray disabled | Exit the UI according to the active milestone; future services continue independently. |
| Close window, user-enabled tray | Hide only if that optional lifecycle is implemented and qualified; retain a usable restore/quit route. |
| Quit UI | End the UI loop and owned UI work; do not equate this with stopping synchronization. |
| Stop synchronizing | A distinct future authorized command, unavailable as a real effect in UI-M0. |

In 1.18.1 a visible `SystemTrayIcon` keeps `run_event_loop()` alive after the last
window closes. `run_event_loop_until_quit()` also stays alive when no window or
tray is visible and therefore needs an explicit `quit_event_loop()` path.
Do not select it merely to fix accidental premature exit; first identify the
intended owner and lifecycle. See [the pinned loop contracts](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/slint/lib.rs).

Slint supplies `SystemTrayIcon` behind `system-tray`; evaluate that API before
adding another tray dependency. It is a top-level component with its own globals,
separate from a window instance. Share state explicitly through the checked Rust
model. Keep the tray instance alive, supply a non-empty icon and preserve explicit
hide/show/quit ownership. Windows left-click and right-click behavior differs;
tray menu shortcuts do not act as window shortcuts. The
[official tray example](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/examples/system-tray/main.rs)
illustrates wiring, not NevroTick error, authorization or resource policy.

## Qualify optional tray recovery

At the optional-tray stage, test show/hide/restore, icon replacement, quit,
repeated cycles and Explorer restart in the admitted Windows fixture. Observe
recovery with the main window both visible and hidden; retain a usable exit.

**Version-bound source concern:** the pinned
[Windows tray backend](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/core/items/system_tray/windows.rs)
creates its callback window with `HWND_MESSAGE`, while its `TaskbarCreated`
handler is intended to re-add the icon after Explorer restarts. Microsoft states
that [message-only windows do not receive broadcasts](https://learn.microsoft.com/windows/win32/winmsg/window-features#message-only-windows),
and that [TaskbarCreated is broadcast to top-level windows](https://learn.microsoft.com/windows/win32/shell/taskbar#taskbar-creation-notification).
This is a source-level recovery concern, not a reproduced native failure. Check
the selected version and actual behavior before claiming recovery. Do not infer
success from a handler's presence, silently add a second tray owner, or turn this
later case into a UI-M0 blocker.

## Keep the UI in the user session

Run Slint as the ordinary interactive user. The engine, privileged clock broker
and elevated lifecycle helper retain separate authority. Do not put Slint into a
service process, request elevation for ordinary UI work, or treat a control's
enabled state as permission for a protected operation.

Microsoft's [Interactive Services guidance](https://learn.microsoft.com/windows/win32/services/interactive-services)
states that modern services cannot interact directly with users and describes a
separate user-session GUI. Its obsolete interactive-service instructions are not
a recipe for new code. Future IPC must use NevroTick's existing identity,
admission and current-authority contracts; UI-M0 continues to use fixtures.

## Re-enter DPI and accessibility at their gates

Keep basic widget keyboard/editing behavior and meaningful labels now. D1
theme/contrast/text/DPI and D3 full keyboard/Narrator/UIA remain deferred under
[architecture §16.5](../../../../../docs/windows-time-architecture.md#165-restricted-first-ui-milestone-ui-m0).
Fluent appearance establishes neither WinUI use nor inherited Win32-control
accessibility. Check the actual feature graph at the later gate.

The pinned winit backend enables AccessKit only with its accessibility feature.
AccessKit's [Windows adapter implements UI Automation](https://accesskit.dev/how-it-works/);
this does not establish correct names, roles, actions, focus or spoken output for
our screens. Use [native accessibility testing](https://learn.microsoft.com/windows/apps/design/accessibility/accessibility-testing)
at D3, including real keyboard and Narrator observations.

At D1, inspect manifest and backend DPI ownership before adding native handlers.
Winit already handles scale-factor and theme events. Microsoft recommends a
manifest for [process-default DPI awareness](https://learn.microsoft.com/windows/win32/api/winuser/nf-winuser-setprocessdpiawarenesscontext);
a manual setter must precede DPI-dependent operations and UI creation. Do not
call competing setters after window creation. Keep logical and physical pixels
distinct in adapter APIs, screenshots, automation coordinates and measurements.

## Resolve installed resources explicitly

Use admitted embedded assets or a qualified installed-resource path. Do not
assume the current working directory is the executable or repository directory.
Preserve brand-kit asset notices and reviewed conversion provenance. UI-M0 does
not gain persistence merely because a toolkit sample writes settings or logs.

Microsoft documents [read-only package files and app-type-dependent virtualization](https://learn.microsoft.com/windows/msix/desktop/desktop-to-uwp-behind-the-scenes)
and [changed working-directory assumptions](https://learn.microsoft.com/windows/msix/desktop/desktop-to-uwp-prepare).
Never write into the installed package or generalize one app type's virtualization
to every packaged desktop process.

At the package gate, use [NevroTick's complete MSIX plan](../../../../../docs/plans/windows-msix-store.md).
Inspect actual DLL imports, architecture, assets and runtime dependencies, and
test the intended activation with package identity. A directly launched EXE or
upstream single-executable manifest does not qualify the complete service package.
The [Slint packaging example](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/docs/astro/src/content/docs/guide/platforms/desktop/windows/packaging.mdx)
supplies examples, not release authority or replacement MSIX contracts.

## Diagnose Windows-specific failures

For missing console diagnostics, inspect the executable subsystem and existing
logging route. For `STATUS_STACK_OVERFLOW`, investigate the actual build and stack
use before adopting the [upstream linker workaround](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/docs/astro/src/content/docs/guide/platforms/desktop/windows/general.mdx).
Do not replace existing Rust flags, mitigations or resource budgets to copy a demo.
Record Windows-specific results through the existing task, with the precise
candidate, token, backend, renderer and remaining native evidence limits.
