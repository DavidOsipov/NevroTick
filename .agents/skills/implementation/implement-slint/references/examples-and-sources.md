# Slint examples and primary-source research

Read to find an implementation pattern, verify an API claim or refresh this
skill's evidence. Adapt the smallest relevant mechanism to NevroTick's current
owner contracts; examples supply neither permission nor product qualification.

## Contents

- [Choose the owner before the example](#choose-the-owner-before-the-example)
- [Use the pinned upstream patterns](#use-the-pinned-upstream-patterns)
- [Learn from real desktop applications](#learn-from-real-desktop-applications)
- [Adapt examples without inheriting their assumptions](#adapt-examples-without-inheriting-their-assumptions)
- [Repeat the primary-source research](#repeat-the-primary-source-research)
- [Resolve version and documentation conflicts](#resolve-version-and-documentation-conflicts)

## Choose the owner before the example

The baseline below was inspected on **5 November 2026**: Slint **1.18.1**,
[`372cf0ee5577c3dfec309a45e7b778ba4e81b734`](https://github.com/slint-ui/slint/tree/372cf0ee5577c3dfec309a45e7b778ba4e81b734).
Keep upstream version evidence separate from the admitted NevroTick version.
Recheck newer work against the selected source and resolved graph.

Start with [OD-13](../../../../../docs/nevrotick-owner-decisions.md#17-od-13--slint-as-the-default-ui-toolkit),
the current [S0.5 task](../../../../../tasks/s0.5-ui-comparison.md),
[GUI handoff](../../../../../tasks/slint-gui-handoff.md) and actual manifests/source.
Use [design v0.4](../../../../../docs/nevrotick-design-v0.4/README.md), its current
audits and brand kit for composition within the admitted milestone; historical
v0.2 screenshots and HTML behavior do not supersede canonical technical contracts.
The M0-SL rows below describe the original fixture stages; current S0.5 addenda
own later admitted GUI source and its remaining native/text-approval dependencies.

| Workstream | How an example may help |
| --- | --- |
| M0-SL-01 / dependency and security | Inspect exact features, licences, build inputs and payloads; retain any graph-admission blocker. |
| M0-SL-02 / UI and Rust contracts | Trace fixture → widget → callback → checked model → view; specify input, lifecycle and observation boundaries. |
| M0-SL-03–05 / UI and QA | Reuse admitted patterns only after predecessor gates; preserve fixture-only behavior and exact-candidate evidence. |
| P5 S5.1–S5.4 / later UI owners | Revisit production snapshots/policy, tray/export and full accessibility/display work at their existing triggers. |
| P6 and W11 package owners | Consume exact runtime/asset/licence inventory; a demo executable does not qualify the complete MSIX. |

UI-M0 remains unelevated and fixture-only: no network/DNS, production IPC,
persistence, service/elevation or clock effects. Keep D1 display, D2 production
state and D3 full keyboard/Narrator/UIA work deferred. Basic labels and toolkit
editing behavior remain required; a reference to a richer example adds no scope.

## Use the pinned upstream patterns

Use the [official example index](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/examples/README.md)
as a directory, then inspect the actual source and manifests. Do not assume a
web demo and the Windows software-renderer candidate exercise the same graph.

| NevroTick need | Pinned source | Borrow and constrain |
| --- | --- | --- |
| Server table | [Gallery table page](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/examples/gallery/ui/pages/table_view_page.slint) | `StandardTableView`, columns and explicit sort/filter boundary. Preserve stable server IDs and validate indices after sorting/filtering. |
| Bounded list or event view | [Gallery list page](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/examples/gallery/ui/pages/list_view_page.slint) | Standard/custom row composition and scroll policy. Virtualization does not bound retained rows, bytes or update rate. |
| Settings controls | [Controls page](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/examples/gallery/ui/pages/controls_page.slint) and [input page](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/examples/gallery/ui/pages/input_page.slint) | Standard widgets and bindings; route accepted edits through the shared checked model, retaining disabled protected actions in UI-M0. |
| Domain/view separation | [Todo MVC adapter](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/examples/todo-mvc/rust/src/ui/task_list_adapter.rs) and [controller/model](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/examples/todo-mvc/rust/src/mvc/controllers/task_list_controller.rs) | Map domain rows to presentation rows; notify changes/additions/removals; test controller behavior separately. Replace unchecked casts and panic-based example errors. |
| Offset/history visualization | [Plotter Rust example](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/examples/plotter/main.rs) | `SharedPixelBuffer` to `slint::Image`. Bound dimensions/history/cadence and qualify any plotting dependency; do not copy synchronous expensive rendering into callbacks. |
| Later tray integration | [Tray Rust wiring](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/examples/system-tray/main.rs) and [tray component](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/examples/system-tray/system-tray.slint) | `SystemTrayIcon`, weak handles and explicit state forwarding. S5.3 owns tray/close/service semantics; do not enable tray behavior during UI-M0. |
| Small form/state exercises | [7GUIs](https://github.com/slint-ui/slint/tree/372cf0ee5577c3dfec309a45e7b778ba4e81b734/examples/7guis) | Study CRUD, editing and timer interactions; keep checked domain construction and time semantics product-owned. |
| Basic Rust build integration | [Official Rust template](https://github.com/slint-ui/slint-rust-template/tree/b93312d04648f2cdc6f14b8b8839d68a8e1272d2) | External `.slint`, `build.rs`, generated modules and callbacks. Integrate into existing ownership; do not replace NevroTick with the template. |

The tray example explicitly forwards state because separate component instances
do not automatically share globals. It distinguishes window hiding from event
loop termination; a visible tray can keep the loop alive. Apply the product's
close/quit contract rather than inheriting the example's hide-to-tray behavior.
Native lifecycle checks remain with [Windows integration](windows-integration.md).

Use the plotting example as one optional rendering mechanism. It demonstrates
pixel-buffer integration, not accurate time-domain calculations, suitable
resource budgets or a reason to add Plotters. Preserve domain units, signs,
missing values and existing proof obligations when projecting measurements.

## Learn from real desktop applications

Treat a public application as evidence of a use case and inspectable design,
not evidence of NevroTick's resource use, security or Windows compatibility.
Record its own revision and graph before drawing comparisons.

### Cargo UI

At [`d763b36fdefa9a40375b09c87010d1c266794ac4`](https://github.com/slint-ui/cargo-ui/tree/d763b36fdefa9a40375b09c87010d1c266794ac4),
[Cargo UI](https://github.com/slint-ui/cargo-ui/blob/d763b36fdefa9a40375b09c87010d1c266794ac4/README.md)
is an upstream-maintained Cargo frontend with dependency-tree and build-result
interfaces. Slint's example index identifies its background-thread use.
Study long-running-work presentation, model structure and bounded handoff ideas.

Its [manifest](https://github.com/slint-ui/cargo-ui/blob/d763b36fdefa9a40375b09c87010d1c266794ac4/Cargo.toml)
uses Slint 1.15-era dependencies, older backend aliases and broad Tokio features.
Do not import these settings or its command-execution authority into NevroTick.
The README distinguishes permissive source-file grants from the GPLv3 program
as a whole; per-file reuse does not settle the complete distribution's licence.

### WSL Dashboard

At [`8379fe8c0baa28c832f19cca84b01c1ffe1a2354`](https://github.com/owu/wsl-dashboard/tree/8379fe8c0baa28c832f19cca84b01c1ffe1a2354),
[WSL Dashboard](https://github.com/owu/wsl-dashboard/blob/8379fe8c0baa28c832f19cca84b01c1ffe1a2354/README.md)
provides a Rust/Slint Windows administration dashboard with tray integration.
Study its composition and separation of OS-related actions from presentation.

Its [manifest](https://github.com/owu/wsl-dashboard/blob/8379fe8c0baa28c832f19cca84b01c1ffe1a2354/Cargo.toml)
pins Slint `=1.17.1` and enables Skia and Tokio. Those differ from NevroTick's
selected baseline. Maintainer memory-use figures are not transferable benchmark
results; privileged actions, scheduling and shutdown behavior add no UI scope.

Use [Slint's showcase](https://slint.dev/showcase) to discover further projects,
then verify claims in the maintainer's source. Marketing language and screenshots
do not establish the renderer, workload, runtime graph or supported release.

## Adapt examples without inheriting their assumptions

1. Identify the mechanism and current owner before copying any code; read the
   file's licence/header and preserve notices when actual material is reused.
2. Trace raw widget values through checked construction and current model state.
   Replace unchecked indices/conversions; controls cannot confer authorization.
3. Keep allocation, row count, decoded pixel size and queued updates bounded.
   A small visible viewport is not a bound on underlying work or retained data.
4. Preserve useful failure information and prior accepted state. Example
   `unwrap()`/`expect()` and silent returns are not NevroTick's error policy.
5. Keep expensive/native work off the UI thread using the admitted ownership
   mechanism; neither a demo nor the engine's Tokio choice selects a UI runtime.
6. Preserve close/cancel and actual completion ownership. Weak handles avoid
   cycles but do not cancel work or prove resource release.
7. Verify the adapted behavior in the selected environment; distinguish model,
   headless, rendered, native and installed-package evidence. See
   [testing and performance](testing-and-performance.md).

## Repeat the primary-source research

Use available relevant MCPs for their distinct evidence surfaces. Tool presence
is not a successful connection; inspect current schemas and report missing
coverage. Do not install or configure new services to satisfy this reference.

1. Read NevroTick's current authority and exact task/manifest first. Frame a
   bounded question and its target version, feature set and Windows environment.
2. Use GitHub MCP to inspect the upstream release/tag, resolve its commit, and
   read that revision's API source, manifests, examples, licence files and tests.
3. Use Context7 `resolve_library_id`, then focused `query_docs` with the returned
   library/version. Respect call limits; verify consequential snippets against
   the selected revision rather than trusting the search result's version.
4. Use official Slint documentation and versioned docs.rs pages for public API
   contracts. Follow definitions to source when behavior or feature gating matters.
5. Use Microsoft Learn MCP for Windows contracts; fetch the relevant full page
   after search. Windows API documentation does not prove Slint exposes that API.
6. Use Exa, Firecrawl developer search or web search to find missing examples,
   issue discussions and contradictions; return to primary evidence before
   recording a technical claim. Do not treat a historical issue as a current bug.
7. Record the source URL/revision, exact claim, inference and evidence limit.
   Keep reusable guidance here; place candidate measurements in the existing task.

## Resolve version and documentation conflicts

Prefer the selected release's actual API/feature definitions when a guide,
generated example, `latest` page or indexed snippet disagrees. Record both
sources and verify the affected behavior; do not generalize a source reading
into a native pass. Useful regression reminders from this baseline:

| Apparent shortcut | Required correction |
| --- | --- |
| `image-default-formats` lists AVIF | Check the actual decoder: `image 0.25.10` requires separate `avif-native`/libdav1d. |
| A font guide asks for `software-renderer-systemfonts` | Check the pinned manifest: it is deprecated; `std` plus software rendering already enables system fonts. |
| Internal compiler code mentions `EmbedAllResources` | Use the public build API's `EmbedFiles` variant at this revision. |
| Any headless test proves layout | Ordinary testing backend renders no pixels; MCP-oriented headless rasterization is a separate unstable mode. |
| The template's dependency string selects the product graph | Inspect matching runtime/build resolution, defaults, host/target edges and licences. |

The first three discrepancies and their exact sources are detailed in
[dependencies and assets](dependencies-and-assets.md); the headless distinction
is detailed in [testing and performance](testing-and-performance.md).
