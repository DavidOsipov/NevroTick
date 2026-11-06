# Slint GUI source

The Windows entry selects Slint with Winit, the software renderer and Fluent-style
widgets. Views follow the [v0.4 prototype](../../../docs/nevrotick-design-v0.4/README.md).
The old Win32 adapter remains unexported historical source. This is still a
fixture application: source implementation and portable rendering do not qualify
live services, clock effects or native Windows behavior.

Start at the [AI handoff](../../../tasks/slint-gui-handoff.md), then the
[current S0.5 evidence](../../../tasks/s0.5-ui-comparison.md#current-nativetext2-integration--gui30-and-later)
and [hardening handoff](../../../tasks/s0.5-ui-comparison.md#planned-hardening-gui-readiness-review--6-november-2026).
Earlier S0.5 stages describe earlier candidates; do not treat them as current
ownership or qualification.

## Entry points and owners

- `app.slint` exports the real `AppWindow`, typed pages and projected source rows.
  Simple includes Status, Settings and About; Power adds Sources, Statistics and
  Diagnostics. The separate `preview.slint` is a fixed development-tool fixture.
- `../src/slint_host.rs` owns bindings and UI-thread state. Its workflow modules
  guard actual callbacks using private route, phase, modal and raw-draft state.
  Generated properties, translated labels and `host-ready` confer no authority.
- `SourceDrafts`, `UiDrafts`, `UiPreferences`, policy/recovery owners and
  `NavigationState` own accepted values and transitions. `UiSession` is a retained
  portable presenter, not the reached source editor's second authority.
- Guarded editors and selectors return the complete accepted raw value on
  rejection. Submissions independently match the owned draft before domain
  mutation; invalid input preserves prior accepted state. End state borrows
  before toolkit setters and observable model notifications.
- `create()` initializes the portable fixture without native workers or observers.
  `create_native()` selects Winit/software, installs the bounded native input
  filter, then installs the native host. Native source includes one shared UI
  timer, opt-in tray ownership and an explicit sample test-notification adapter.
  Their actual Windows behavior remains unqualified.

## Text, assets and boundaries

Typed Rust text feeds immutable Slint copy models, native tray/toast text and
accessibility consumers. Editable [English JSON](../locales/en.json) passes strict
UTF-8, schema, repertoire and compatibility checks before CBOR preparation.
Current schema/text contract/catalog revision are each 2: 509 active identities,
596 stable ordinals and 426 immutable output rows. GUI build 2 is informational,
not the compatibility key. See [text development](../../../docs/nevrotick-gui-text-development.md)
for generation and [OD-15](../../../docs/plans/ui-localization.md) for authority.

The production semantic approval ledger is empty. Parser, generator, signature
and coverage success do not approve critical wording or external language packs.
The test-only catalog grant cannot become a production bypass. No catalog can
select commands, permission, severity, routes or a repository URL.

The original brand PNG is rendered with its [asset notices](assets/README.md).
No AVIF decoder or artwork conversion is required. Font and Unicode data retain
their own provenance and licences; see [GUI licensing](../LICENSING.md).

Sources, interval, policy, recovery and diagnostic choices are local previews.
Custom intervals accept positive whole minutes, with raw byte and numeric bounds.
Protected Apply, real measurement requests, service control, Windows Time
handover, sign-in registration, persistence and diagnostic collection/export stay
unavailable. Preference acceptance does not establish native readiness.
UI exit never means the background services stopped; cleanup failure keeps the
window available for explicit recovery.

## Verification and remaining work

Use the configured `scripts/verify.py` lane with admitted vendor/tool/SDK inputs;
the [handoff](../../../tasks/slint-gui-handoff.md) records commands and exact
evidence. The capture example renders the real AppWindow offscreen through
Slint's public software platform; it is not a native Windows test.

The historical GUI41 checkpoint passed 183 GUI tests, 17 GUI doctests, 143 source
controls and 50 captures. Windows-target type checking and Clippy ran on Linux;
Windows SDK/CRT/link, actual input/clipboard/COM/UIA/Narrator/DPI/tray/toast,
teardown and resource observations remain open. D1–D3, backend integration,
process-mitigation startup, package qualification and independent critical-text
approval retain their existing gates. Source preparation lifts none of the
native, fuzz or storage holds.
