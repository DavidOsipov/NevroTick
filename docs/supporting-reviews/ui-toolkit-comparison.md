# Win32 versus egui: NevroTick comparison

**Date:** 30 October 2026, Asia/Tbilisi

**Disposition:** comparison closed; retain Win32.

**Execution record:** [S0.5 comparison task](../../tasks/s0.5-ui-comparison.md).

## Decision

David rejected the tested egui configuration because approximately **145 MiB of
private memory is unacceptable for this simple UI**. Keep the existing Win32
implementation and resource targets. Reduced application code and styling/testing
benefits do not authorize a larger memory budget or further egui experiments in
the current task.

The result applies to the tested **eframe/egui 0.37.0 + Glow + AccessKit** Windows
configuration. It is not a claim that every egui renderer or integration has the
same footprint. The current architecture and [UI requirements](../windows-time-architecture.md#16-minimal-accessible-user-experience)
remain authoritative; this report records the decision and its evidence.

## What was compared

An isolated egui candidate reused the existing, byte-identical 295-line portable
model and deterministic fixtures. Both candidates presented Status, Sources and
Settings, persistent sample-data labeling, disabled protected actions and a
transient source editor. There was no production network, IPC, persistence,
service, elevation or clock integration.

Both were built using Rust 1.99.1, edition 2024, `x86_64-pc-windows-msvc`, O3,
ThinLTO, one codegen unit, checked overflow, unwind and limited debugging symbols.
The current x64/CFG/ASLR/DEP compiler flag vector was retained. This establishes
matching build settings, not complete runtime mitigation qualification.

The egui candidate disabled default features and selected only `accesskit`,
`default_fonts` and `glow`. In particular, it did not enable persistence, browser
links, system-font discovery or the inspection TCP server. Default wgpu was not
built or measured.

Native observations ran sequentially, unelevated, on the same Windows host:
OS build `10.0.26300.0`, 16 logical processors, system DPI 144. Each selected run
used a fixed three-second warmup and ten-second visible/minimized idle intervals.
Win32 was DPI-unaware with window DPI 96; egui was per-monitor aware with window
DPI 144. Cache state, rendering configuration and window sizes were not normalized.

## Results

| Measure | Win32 | Egui / Glow / AccessKit |
| --- | ---: | ---: |
| Private memory, selected warmed visible/restored/page snapshots | **2.05–2.20 MiB** | **133.04–144.88 MiB** |
| Release executable, excluding PDB | **179,200 bytes / 175 KiB** | **6,489,088 bytes / 6.19 MiB** |
| Selected registry packages | 16 | 124 |
| Handwritten UI files counted | 1,432 lines of native adapter/controls/view | 319 lines of app + 135 lines of editor session |
| Status / Sources / Settings content | All three observed | All three observed |
| Minimize / restore / normal Close | Observed; exit 0 | Observed; exit 0 |
| Visible-window detection proxy, selected run | 423 ms | 1,780 ms |
| Launch-to-UIA-semantic-readiness proxy, selected run | 781 ms | 2,183 ms |

The tested egui process exceeded the proposed **48 MiB visible-UI private-memory
target** by roughly three times in this pilot. Win32 had substantial headroom.
These are process-private bytes, rather than working-set or total-system memory
measurements. GPU memory and complete driver/system overhead were not measured;
GPU/driver identity was not recorded.

The timing proxies include process/token/module/window selection and UIA-provider
overhead. They are not controlled cold-start results or p95 acceptance tests.
Memory values are snapshots, not sustained p95, leak or soak evidence. Source-line
counts do not establish equal visual polish, maintainability or defect rates;
egui shifts native ownership into its dependencies.

There is **no reliable CPU winner** from these short samples. Win32's selected
visible interval measured about 0.16% of one logical core; egui measured 0% in its
final interval and about 1.25% in an earlier valid interval. Final minimized
intervals accumulated zero CPU time for both. Quantization, caches and background
work prevent a stable CPU or power conclusion.

## Implementation and testing tradeoffs

Egui required less handwritten application UI code. Its actual AccessKit nodes
provided `TogglePattern` actions for the navigation buttons, and all three views
were reached through those actions. This is useful evidence for its Rust/UI
automation ergonomics, without proving full accessibility.

The managed UIA client-side proxy registration failed in the test environment.
Win32 navigation therefore used an explicitly recorded, owned and bounded native
`BM_CLICK` fallback. This demonstrates the view transitions; it does not establish
missing accessibility support in Win32 controls or a Narrator advantage for egui.

The existing Win32 candidate passed the configured Windows compiler, Clippy,
linking and boundary/inventory checks. The egui candidate passed nine portable
model/session tests, three doctests, Windows Clippy, formatting after a
whitespace-only repair, and the final Windows release build. The portable tests
did not execute egui widgets; `egui_kittest` was not exercised.

Native editor valid-update, rejection-preservation and Cancel were not exercised
in this comparison. Full keyboard/Narrator, contrast/text scaling, mixed-DPI,
tray, UAC, renderer/device loss, RDP, power, complete MSI and long-run checks also
remain unperformed. The pilot completes neither S0.5 nor CHECK-21/22 and changes
none of UI-M0's D1–D3 deferrals.

## Dependency-license concern

Egui/eframe's own `MIT OR Apache-2.0` grant does not describe the entire native
graph. The resolved normal runtime paths included Apache-2.0-only `winit 0.30.13`,
`unicode-general-category 1.1.0`, `accesskit_winit 0.34.2` and the selected glutin
branch. These were checked against package manifests and notices, with an
independent path review; the [task record](../../tasks/s0.5-ui-comparison.md#dependency-and-license-finding)
retains versions, paths and primary-source links.

This raises the compatibility/adoption concern already covered by
[architecture §18.3](../windows-time-architecture.md#183-licensing-and-provenance)
and [ASF's GPL compatibility guidance](https://www.apache.org/licenses/GPL-compatibility.html)
for the current GPL-2.0-only combination. Switching to wgpu removes the glutin
branch but retains winit and Unicode-category code. Disabling AccessKit alone
does not resolve the whole graph.

The experiment was private and unshipped. It grants no product dependency
admission, distribution clearance, relicensing or automatic legal clearance from
a process/IPC split. This issue was additional to the owner's memory-based
rejection; no license or resource-policy change was adopted.

## Evidence quality and reuse

Retain the observer failures so future work does not repeat false conclusions:

- `.NET Process.MainWindowHandle` initially selected a 22×22 Winit helper window.
  Its apparent navigation/Close failures were observer errors. Final selection
  used the owned, visibly titled NevroTick window and targeted that exact HWND.
- Hidden-directory handling and failed managed UIA proxy registration caused
  setup failures before launch; neither established a product defect.
- The observer initially omitted `TogglePattern` for egui's Button roles. The
  corrected final run observed all three pages. Unavailable nonfinite structural
  bounds were serialized as null in the final helper.

There were two actual Win32 launches and three actual egui launches; setup-only
attempts launched neither. The selected final runs both closed normally with
unchanged candidate/helper hashes. An independent reviewer checked the common
model/profile, observer corrections and final raw records.

Screenshots were captured for all Win32 pages and egui Sources/Settings; egui
Status capture was skipped when foreground/occlusion checks failed. WSL's Windows
drive mount became unavailable during reconciliation. The screenshots were not
visually reviewed, so this report gives no visual-fidelity verdict. Full final
PowerShell stdout receipts were retained and reviewed from the Linux filesystem.

Durable private evidence is stored at:

`/home/david_osipov/.local/share/nevrotick/ui-comparison-20261030-rok_tvh7/`

It includes `comparison-summary.json`, the final observations, artifact/source
hashes, the source/log/receipt ZIP and the private experimental executable.
The [execution record](../../tasks/s0.5-ui-comparison.md#observer-corrections-and-evidence-custody)
contains exact artifact identities, ZIP/source-map hashes and original temporary
and Windows paths. The ZIP SHA-256 is
`eeb4869556097643cd79581569cd26752a08bb783d554204444bff1ff1f84e7d`.

Use these results for the recorded configuration and decision. Continue the
existing Win32 plan. Any future egui investigation needs a new explicit task and
must address the owner's footprint requirement and the actual dependency graph.
