# Accessibility

NevroTick aims to make time status, configuration, errors and recovery actions
usable without relying solely on a mouse, vision or color perception. This is a
project goal, not a claim that the current prototype meets an accessibility
standard or has completed assistive-technology qualification.

## Current scope and intended environments

NevroTick is in development. The initial product scope is Windows 11 26H2 x64
and Windows Server 2025 x64 with Desktop Experience, on the selected x86-64-v3
CPU baseline. These are intended targets, not a declaration of a currently
supported or qualified production release; see the
[release policy](docs/release-policy.md) and
[platform decision](docs/nevrotick-owner-decisions.md#18-od-14--compact-windows-platform-scope).
Linux/WSL checks and Windows cross-compilation do not establish native desktop
or assistive-technology support.

The current Windows UI entrypoint uses Slint. UI work remains a fixture-only,
unelevated prototype, not a live clock-management interface. The
[GUI handoff](tasks/slint-gui-handoff.md) and
[S0.5 task](tasks/s0.5-ui-comparison.md) track current source and verification.
Historical Win32-control results and HTML design previews do not qualify the
Slint application.

## Known limitations and planned checks

Full keyboard, Narrator and UI Automation qualification remains open. The
original UI-M0 milestone explicitly defers full accessibility work (D3) and
contrast, text-scaling and DPI work (D1) until their recorded re-entry triggers.
Basic toolkit behavior, meaningful labels, source-level adapters or portable
tests are not evidence that the whole interface is accessible. Native/backend
and accessibility results must be established on the actual Windows candidate.

The planned checks include:

- Keyboard-only navigation, tab order, visible focus, cancellation and focus
  restoration after dialogs or elevation prompts.
- UI Automation names, roles, values, states and events, separately checked
  against actual Narrator output.
- Status, authentication, freshness and error meanings conveyed without relying
  on color alone, with understandable field errors and bounded announcements.
- Contrast themes, text scaling, DPI changes and long text.
- Access to critical settings and recovery without a tray-only interaction.

These checks are owned by [architecture section 16](docs/windows-time-architecture.md),
CHECK-22 and the [accessibility verification skill](.agents/skills/testing/qa-accessibility/SKILL.md).
The [UI-M0 task](tasks/ui-m0.md) records deferrals and their triggers. Relevant
WCAG criteria interpreted through WCAG2ICT can guide evaluation, but this project
currently makes no WCAG conformance or accessibility-certification claim.

## Report a barrier

Open an [issue](https://github.com/DavidOsipov/NevroTick/issues) and identify it as
an accessibility report, or contact
[personal@david-osipov.vision](mailto:personal@david-osipov.vision) if you prefer
not to report publicly. You do not need to disclose a disability or medical
information.

Include whatever you can: the commit/build, Windows version, assistive technology
and version, input method, relevant display/text settings, steps, and expected
versus actual behavior. Screenshots or recordings are optional; remove personal
data first. Reports about documentation barriers are welcome too. Incomplete
reports are useful, and you may describe the task you could not complete instead
of diagnosing the cause.

Reports will inform the existing UI and documentation work. No fixed resolution
deadline is promised. If a barrier also exposes a security vulnerability, use
[SECURITY.md](SECURITY.md) rather than publishing exploit details.
