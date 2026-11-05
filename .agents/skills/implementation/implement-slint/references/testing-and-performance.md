# Slint testing and performance evidence

Read when selecting tests, evaluating an automation tool, migrating UI-M0 evidence
or measuring resource cost. Apply [the testing contract](../../../testing/CONTRACT.md)
and [UI-M0 acceptance](../../../../../tasks/ui-m0.md#acceptance-and-checks).
Use the smallest layer that exercises the changed guarantee; a documentation
change does not require application compilation or a native campaign.

Version-dependent APIs below were checked against Slint **1.18.1**, commit
`372cf0ee5577c3dfec309a45e7b778ba4e81b734`. Recheck the admitted versions and feature
graph before adding a test dependency. Keep receipts in the existing task chain.

## Contents

- [Choose the evidence layer](#choose-the-evidence-layer)
- [Use editor and preview tools within scope](#use-editor-and-preview-tools-within-scope)
- [Use the preliminary testing backend deliberately](#use-the-preliminary-testing-backend-deliberately)
- [Keep developer transports separately admitted](#keep-developer-transports-separately-admitted)
- [Replace Win32-specific observation assumptions](#replace-win32-specific-observation-assumptions)
- [Observe lifecycle and resource cost](#observe-lifecycle-and-resource-cost)
- [Preserve deferred gates and report limits](#preserve-deferred-gates-and-report-limits)

## Choose the evidence layer

| Layer | Useful evidence | Does not establish |
| --- | --- | --- |
| Pure Rust model/handler checks | Checked inputs, transformations, projections, state transitions and forbidden effects | Widget wiring, rendering or native interaction |
| Compiled Slint with normal testing backend | Properties, callbacks, model notifications, simulated interaction and time | Real pixels, native font metrics, Windows behavior |
| MCP-oriented headless software rendering | Actual rasterized layout/screenshots and toolkit introspection in the declared fixture | HWND lifecycle, Windows UIA/Narrator, packaged activation or Windows resource cost |
| Native Windows Slint executable | Real window/editor/lifecycle observations and reached Windows integrations | Untested pages, configurations, later D1/D2/D3 or final package behavior |
| Final installed package | Declared activation, included resources/dependencies and package-specific behavior | Release approval or capabilities outside the exercised scenario |

Distinguish an HTML/React design preview from the compiled `.slint` application.
Use it for composition references, not native evidence. A compile or cross-build
also proves less than launching and exercising the actual Windows candidate.
The [pinned testing guide](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/backends/testing/README.md)
explicitly distinguishes the normal testing backend from MCP headless rendering.

## Use editor and preview tools within scope

Use an existing admitted, version-matched tool rather than installing the latest
global binary. Check its actual help/version before relying on an option. The
[pinned editor guide](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/docs/astro/src/content/docs/guide/tooling/manual-setup.mdx)
and [viewer guide](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/docs/astro/src/content/docs/guide/tooling/slint-viewer.mdx)
provide these useful paths:

| Tool operation | Use and evidence limit |
| --- | --- |
| `slint-lsp` in an editor | Language diagnostics, navigation and formatting. Tool support does not establish generated Rust or application behavior. |
| `slint-lsp format <file>` | Inspect formatted output on stdout; `-i` writes the file. Review a focused formatting diff. |
| `slint-viewer --check <file>` | Compile/check the Slint input without opening a window; it does not build the Rust adapter or exercise its controller. |
| `slint-viewer --screenshot <output> <file>` | Render a windowless preview image without starting the application's MCP server; record style, renderer, scale, assets and mock data. |
| Editor preview or viewer auto-reload | Iterate on composition with fixture data; separately check the compiled application's bindings and handlers. |

Do not turn a viewer pass into an application pass: preview/mock-data branches,
property injection and interpreter configuration can differ from generated Rust.
The viewer also supports shell-command callback handlers, file output and remote
network mode. Keep those separate from an ordinary visual/check operation; do
not copy `--on`, `--remote` or persistence recipes into UI-M0. A viewer screenshot
does not require enabling the application's `mcp` or `system-testing` features.

## Use the preliminary testing backend deliberately

`i-slint-backend-testing` is an internal, preliminary crate without semver
compatibility guarantees. If the dependency owner admits it for tests, require
the exact matching Slint version, including `=` in its version requirement.
Keep it out of the runtime dependency graph. Do not generalize this test-only
exception into permission to import arbitrary `i-slint-*` internals.

| Initializer | Intended use and constraint |
| --- | --- |
| `init_no_event_loop()` | Local tests without a running loop; simulated time. It cannot establish worker handoff or running event-loop behavior. |
| `init_integration_test_with_mock_time()` | Integration event loop with explicitly advanced simulated time. |
| `init_integration_test_with_system_time()` | Integration event loop using system time. |

Initialize before any call that would select a backend. Event-loop initializers
are once-per-process: follow upstream's one `#[test]` per integration-test binary
pattern. Re-initialization is a harness error, not a product failure. Use
`mock_elapsed_time()` for explicit mock-time advancement; bound async completion
and shutdown. See [the pinned initializer contracts](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/backends/testing/lib.rs).

The normal testing backend renders **no pixels** and measures text with fixed
metrics. `ElementHandle` can locate elements and invoke accessible actions;
simulated `single_click()`/`double_click()` return futures and need a running
event loop. These are Slint-level observations, not Windows UIA or Narrator tests.

Test actual adapter wiring as well as the pure model: correct row identity after
sorting/filtering, accepted and rejected edits, Update/Cancel, visible rejection,
and unchanged valid state. Challenge receiving handlers directly to expose
control-bypass paths. Use admitted fixture bounds and valid neighboring controls;
do not create live network, persistence, privilege or clock effects for a UI test.

## Keep developer transports separately admitted

The [pinned public feature manifest](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/slint/Cargo.toml)
documents developer features with materially different effects:

| Feature | Activating setting | Additional behavior |
| --- | --- | --- |
| `mcp` | `SLINT_MCP_PORT` at runtime | HTTP server offering UI inspection and control |
| `system-testing` | `SLINT_TEST_SERVER` at runtime | Connection to a test server with UI introspection/control |
| `live-preview` | `SLINT_LIVE_PREVIEW` during compilation | Development reload of `.slint` files |

Exclude these from the product/release graph. A separate development fixture
needs an explicit admitted purpose, graph, effects, bounded lifetime and cleanup.
Current UI-M0's no-live-network restriction is not relaxed by this reference or
by a request to research through MCPs. Do not automatically start a server, add a
port-forward, modify client configuration or attach a production UI.

For an admitted MCP fixture, upstream requires compile-time
`SLINT_EMIT_DEBUG_INFO=1` for element introspection. Its server binds to
`127.0.0.1`, validates local Origins and has **no authentication**. Loopback and
Origin checks do not authenticate another local process. Keep the fixture free
of real credentials, privileged controls and sensitive product state; preserve
the binding and record actual shutdown. See [the pinned transport/security design](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/docs/development/mcp-server.md).

Window and element handles are distinct generational identities. Discover current
handles and reject stale or wrong-kind handles; do not replay captured identifiers
after teardown. An automation command's successful dispatch does not establish
the intended visible state or domain effect; inspect the resulting state.

In this automation path, `headless`, `headless-software` and `headless-skia` select
an unstable windowless rasterizing backend. The unqualified `headless` choice
can select Skia when enabled; otherwise it uses the built-in software renderer.
The documented automatic headless fallback after graphical initialization failure
is another reason to record the **actual** backend. A headless screenshot cannot
be relabelled as a Windows screenshot. Consult [the pinned headless instructions](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/backends/testing/README.md).

## Replace Win32-specific observation assumptions

The existing `scripts/ui-m0-windows.ps1` uses `GetDlgItem` and native text messages.
Slint-drawn controls do not provide the former Win32 child-control contract;
retaining those probes unchanged can miss every relevant widget. Follow the
[Slint migration handoff](../../../../../tasks/ui-m0.md#slint-migration-handoff--5-november-2026)
when adapting observation. Preserve candidate/process identity checks and exact
expectations while replacing the toolkit-specific access method.

Exercise the compiled Windows UI through the appropriate admitted native input
or semantic observation route. Inspect every changed screen and its navigation,
representative invalid/valid input, fixture labels, disabled protected actions,
editor commit/cancel, focus return and close behavior. A direct callback test
does not establish that a user can reach and activate the control.

Inspect actual rendered output for clipping, overlap, missing assets and hidden
controls. Retain a native screenshot with candidate, backend, client geometry and
scale context. Distinguish these basic observations from deferred theme/DPI or
comprehensive accessibility qualification. Use the existing
[Windows evidence guide](../../../testing/references/windows-evidence.md).

## Observe lifecycle and resource cost

Freeze the candidate bytes, source/dirty diff, lockfile, features, generated-code
configuration, target, renderer, style, asset set and fixture. Record Windows
build, token, power mode, client geometry, DPI and focus/visibility conditions.
Do not compare unequal layouts/workloads as equivalent toolkit performance.

Measure explicit phases: startup, visible idle, representative editor/navigation
work, minimized or hidden idle, restoration, repeated open/close cycles and final
shutdown. Qualify optional tray separately when activated. Bound durations,
sample counts and retries before running; preserve failed or partial trials.

| Quantity | Interpretation |
| --- | --- |
| Private bytes | Private committed memory; not total resident physical memory |
| Working set | Resident pages, including shared/file-backed pages; do not add to private bytes |
| Process CPU-time delta | Divide by monotonic elapsed time and declare one-core versus machine-wide convention |
| Handles, GUI objects, tasks/timers | Observe relevant ownership and repeated-cycle retention separately |
| GPU/driver/system allocations | State whether measured; process memory counters do not include every cost |

Observe cleanup and retention **before process exit** and before harness teardown.
Repeat lifecycle transitions to distinguish a cache that reaches a plateau from
continuing growth. Check weak callbacks after component destruction, pending UI
updates, retained native-window references and worker/timer completion. A normal
exit or operating-system cleanup cannot prove leak-free ownership. Follow
[Windows memory diagnostics](../../../testing/references/windows-memory.md)
for the reached native resource boundary and required detector coverage.

Report zero CPU-time increments as below the observed counter resolution during
that interval, not zero work or negligible battery cost. Reuse the historical
[Slint pilot](../../../../../tasks/s0.5-ui-comparison.md#slint-software-renderer-live-host-result--5-november-2026)
only within its recorded scope; its Status-only measurements do not qualify the
new editor, workload, feature graph, long-duration behavior or full resource gate.

## Preserve deferred gates and report limits

Keep D1 theme/contrast/text/DPI, D2 production freshness/reconnect/policy/admin
results and D3 full keyboard/Narrator/UIA at their existing activation gates.
At D3, inspect actual native UIA names, roles, patterns, actions and focus, then
test keyboard workflows and Narrator separately. A Slint accessible-action test,
an AccessKit-enabled graph or a visual screenshot is insufficient by itself.
See [Microsoft's accessibility testing guidance](https://learn.microsoft.com/windows/apps/design/accessibility/accessibility-testing).

Report the exercised guarantee, exact candidate/environment, actual selected and
executed cases, independent expected results, failures, cleanup and substitutions.
Separate source review, model execution, headless rendering, native Windows and
installed-package results. Preserve unavailable checks as incomplete; do not
convert research, setup success, ignored tests or a different toolkit's receipt
into a Slint qualification pass.
