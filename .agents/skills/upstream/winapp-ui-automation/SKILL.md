---
name: "winapp-ui-automation"
description: "Use an available, qualified Windows winapp CLI to inspect and exercise a specific native application through UI Automation, then verify its observed state. Supplements NevroTick accessibility and executable UI tests."
license: "MIT"
---

## NevroTick scope

Use alongside project-owned `qa-accessibility` and `qa-e2e-tests`; their requirements and canonical architecture control assessment. Native Win32 UIA supported. Selects no WinUI, .NET, Windows App SDK or MSIX for NevroTick.

Run only on qualified Windows test machine with already available `winapp` CLI and approved isolated application/test session. Record OS, CLI version, candidate and target identity; check installed help/JSON shapes before using examples. Retained JSON reference describes features added in v0.3.1 and v0.6.3+, not verified compatibility with every later CLI.

Resolve explicitly scoped application; use its HWND for supported actions. Recheck ownership when windows change. Examples use placeholder targets. No authorization for tool/SDK installation, elevation, Sandbox enablement, DevTools attachment/injection, system-wide shortcuts or other-application actions. Optional upstream skills are references, not bundled dependencies or permission to expand task.

Verify each action through target's resulting UIA state, screenshot or keyboard/focus behavior. Exit zero insufficient: pinned keyboard shim calls `PostMessage` without checking Boolean result; queued messages may not process as intended. Inspect Narrator speech separately from UIA assertions. Without Windows/tool environment, provide qualified commands; report runtime evidence unavailable.

**If unable to run `winapp`** (no shell/command denied), give exact project `winapp` command(s), not only steps.

## When to use
- Inspecting a running Windows app's UI from the command line
- AI agents interacting with Windows applications (clicking buttons, reading text, taking screenshots)
- Verifying UI state during development or testing
- Automating UI workflows without Playwright or Selenium
- Debugging WinUI 3, WPF, WinForms, Win32, or Electron app UIs by what the user sees and can operate
- For a separately scoped WinUI 3 application's XAML internals, see the optional upstream [winui-devtools skill](https://github.com/microsoft/winappCli/blob/31fc2ac52b9340846d9b5baa0b9be634ca77a8d1/plugins/winapp/skills/winui-devtools/SKILL.md); it is not required for NevroTick's Win32 UI.

## Prerequisites
For separately scoped WinUI 3 application, optional upstream [winui-devtools skill](https://github.com/microsoft/winappCli/blob/31fc2ac52b9340846d9b5baa0b9be634ca77a8d1/plugins/winapp/skills/winui-devtools/SKILL.md) and `winapp devtools` cover XAML source/tree, bindings and dependency properties. Requires attached target or explicit injection authorization for `--attach`; PID alone is not consent. AutomationId selectors work in both tools; generated slugs do not transfer to DevTools. Use `winapp ui` for cross-framework actions, screenshots, waits and accessibility.

Measure WinUI scenario work with [optional upstream performance skill](https://github.com/microsoft/winappCli/blob/31fc2ac52b9340846d9b5baa0b9be634ca77a8d1/plugins/winapp/skills/winapp-performance/SKILL.md): bracket UI actions with capture markers.

- UIA mode requires available qualified CLI, accessible running target and relevant UIA provider/patterns; access restrictions or missing providers can prevent inspection.
- For input-injecting verbs (`click`, `hover`, `drag`, `touch`, `pen`, `scroll --wheel`/`--wheel-horizontal`, `send-keys --via send-input`): an **unlocked, interactive desktop** with the target window foregroundable. On a locked/secure desktop they fail fast with `no_interactive_desktop`. The UIA-pattern verbs (`inspect`, `search`, `get-*`, `wait-for`, `set-value`, `invoke`, `scroll --direction/--to`) are headless/locked-session friendly — prefer them in CI.
- `screenshot` is **not** in that group: it always takes an exclusive turn, so it queues behind other UI workflows, and capture can need a usable interactive desktop — the engine restores the target if it is minimized, and falls back to foregrounding it when frame capture is unavailable or `--capture-screen` is used.
- `--capture-screen` needs **exactly one window**. `-w <hwnd>` gives it one: that window's screen region, including anything visibly on top of it. If `-a` matches several top-level or owned windows the command fails with `invalid_arguments` before capturing; run `winapp ui list-windows -a <app>` and retry with `-w <hwnd>`.
- **If UI workflows may overlap**, set one id per logical workflow (below). Without id, commands still work but are not grouped.

## Coordinating with other UI workflows

Windows has one foreground window, keyboard focus, cursor and input stream. `winapp ui` desktop-driving commands always take **cooperative turns** to prevent concurrent focus theft/menu dismissal. Read-only commands never wait.

Desktop custody *across* commands is opt-in: without id, each command releases desktop immediately on finish:

```powershell
# Set once per logical UI workflow — same value for cooperating calls, different values for
# independent workflows (even from the same agent).
$env:WINAPP_UI_WORKFLOW_ID = [guid]::NewGuid().ToString()
```

Agent workflow rules:

- **Each tool call usually gets fresh shell**; no process-ancestry fallback. Commands grouped ONLY by injected id; without it, every call is separate workflow.
- **Workflow with id keeps turn for four seconds** after last command: covers consecutive script commands, deliberately expires during reasoning. Fallback when unable to signal done, not completion method.
- **Run `winapp ui yield` after known sequence.** Hands desktop over immediately, avoiding unnecessary four-second grace. Repeated yield or yield after grace harmlessly succeeds.
- **After reasoning gap, replay setup.** Another workflow may have used desktop: reopen menu/re-navigate, re-resolve element, then act. Do not assume transient UI survived.
- **Prefer one tight script over many round trips** for a known sequence: `winapp ui invoke View -w
  $hwnd; winapp ui search "Status bar" -w $hwnd; winapp ui click "Status bar" -w $hwnd; winapp ui yield`.
- **`record` shares own workflow's turn**: captures concurrent clicks/typing only when both commands carry same id. `record` without id blocks everyone else throughout.
- **Ordering: owner affinity, then FIFO.** Active workflow may issue commands ahead of waiters; after yield/grace expiry, serve waiters in arrival order.
- **Waiting indefinite, cancellable.** After one second, stderr names desktop holder/duration (silent under `--json` and `--quiet`); Ctrl+C exits `130`, error `cancelled`, command never ran.
- **No hard cap**: long script, unbounded recording or failure loop can block other mutations until completion/stop.

Commands that never wait: `status`, `list-windows`, `inspect`, `search`, `get-*`, `wait-for`.
Commands waiting for turn without taking desktop (headless/locked-session friendly): `set-value`, `scroll-into-view`, `scroll --direction`/`--to`, `record`. App mutations queue behind other workflows. Within own workflow, overlap with shared work lets `record` capture `set-value`; still wait behind earlier own-workflow `DesktopExclusive`, preserving `click` then `set-value` order.
Commands that take the desktop exclusively: `invoke`, `click`, `drag`, `hover`, `scroll --wheel`/`--wheel-horizontal`,
`touch`, `pen`, `focus`, `send-keys`, `screenshot`.

```powershell
# Finish a workflow deliberately rather than leaving the desktop reserved for four more seconds.
winapp ui yield
```

## Common patterns

### Read within a specific container

```powershell
winapp ui search "Welcome to MyApp" -a myapp --root MailRow --type Text --class-name TextBlock
winapp ui get-value Subject -a myapp --root MailRow --type TextBox
winapp ui wait-for Subject -a myapp --root MailRow --type Edit --value Ready --timeout 10000
winapp ui search --class-name CalendarSurface -a myapp   # no text selector needed
```

Use `--root`, `--type`, and `--class-name` together or separately on `search`,
`get-property`, `get-value`, `wait-for`, and the commands that act on one
selected element (`invoke`, `set-value`, `click`, `focus`, and so on; not `send-keys --target`).
`--type` and `--class-name` work without a selector; commands other than `search` then need
exactly one match, and an ambiguous match lists the first five with their slugs. `--root` alone is not a query. For `set-value`, pass `""` as the selector.
For `click`, `hover`, `touch`, and `pen`, filters can't be combined with `--at` or `--path`. The root must be unique; only its
descendants match. `wait-for` re-resolves it every poll, including when it is
initially absent. Type names and literal whole ClassName values ignore case.
The only type aliases are `TextBox` → `Edit` and `TextBlock` → `Text`.
See [Scoped and typed queries](https://github.com/microsoft/winappCli/blob/31fc2ac52b9340846d9b5baa0b9be634ca77a8d1/docs/ui-automation.md#scoped-and-typed-queries)
for the full type vocabulary, boundaries, and error behavior.

### Discover and interact
```powershell
# See what's clickable, then screenshot for context
winapp ui inspect -a myapp --interactive; winapp ui screenshot -a myapp

# Click and verify the page changed
winapp ui invoke btn-settings-a1b2 -a myapp; winapp ui wait-for pn-settingspage-c3d4 -a myapp --timeout 3000; winapp ui screenshot -a myapp

# Fill a form and submit
winapp ui set-value txt-searchbox-e5f6 "hello" -a myapp; winapp ui invoke btn-submit-7a90 -a myapp; winapp ui screenshot -a myapp
```

### Find visible text and click it
```powershell
# Search by text — output shows invokable ancestor
winapp ui search "Save changes" -a myapp
# Output:
#   lbl-savechanges-a1b2 "Save changes" (120,40 80x20)
#         ^ invoke via: btn-save-c3d4 "Save"

# Invoke by text — auto-walks to parent Button
winapp ui invoke 'Save changes' -a myapp
```

### Navigate multi-page apps
```powershell
# Click nav item, wait for page, inspect what's available
winapp ui invoke itm-samples-3f2c -a myapp; winapp ui wait-for pn-samplespage-b4e7 -a myapp; winapp ui inspect -a myapp --interactive
```

### Choose an exact action in tests
```powershell
winapp ui invoke SettingsCategory -a myapp --action select
winapp ui invoke AgreeCheckbox -a myapp --action toggle-on --json
winapp ui invoke Open -w <dialog-HWND> --type Button --action invoke
```

Use `--action` to prevent automatic pattern/ancestor fallback; omit for existing automatic behavior. `--root`, `--type` or `--class-name` use read-query matching: exactly one match in selected app/window, otherwise fail without acting; skip ancestor fallback. See [action reference](https://github.com/microsoft/winappCli/blob/31fc2ac52b9340846d9b5baa0b9be634ca77a8d1/docs/ui-automation.md#invoke) for actions, scope, idempotent toggles, failure recovery; [JSON envelope](references/ui-json-envelope.md#ui-invoke---json) for results.

### Disambiguate duplicate elements
```powershell
# Text matches by substring, but an element named exactly the text wins
winapp ui invoke Submit -a myapp      # acts on Button "Submit", not Button "Submit Order"
# When several elements share that name, the error shows slugs for each — pick the right one
winapp ui invoke Save -a myapp
# → Selector matched 2 elements:
#   [0] Button "Save" → btn-save-a1b2
#   [1] Button "Save" → btn-save-c3d4
# Use the slug: winapp ui invoke btn-save-c3d4 -a myapp
```

## Key concepts
- **Selector brackets**: `inspect` and `search` output shows selectors in `[brackets]` — use the bracketed value with other `ui` commands. Selectors are either AutomationId (stable, developer-set) or generated slug (e.g., `btn-name-hash`).
- **AutomationId selectors**: When an element has a unique AutomationId, it becomes the selector directly (e.g., `[MinimizeButton]`). These survive layout changes and localization — preferred for stable targeting.
- **Slug selectors**: When no unique AutomationId exists, a generated slug is used (e.g., `[btn-close-a2b3]`). Format: `prefix-name-hash`. May go stale after UI changes.
- **Plain text search**: `search` and `invoke` accept plain text — `search Minimize` finds elements with "Minimize" in their Name or AutomationId (substring, case-insensitive). No special syntax needed. Commands that act on one element prefer an exact AutomationId match, then the only element whose Name is exactly the text (ignoring case); `search` still lists every match.
- **`--interactive` flag**: Filters to elements you can invoke, click, or set-value (including text documents) with auto-depth 8 — the fastest way to see what you can act on
- **Invokable ancestor surfacing**: When a search result isn't invokable, the nearest invokable parent is shown with its selector
- **`;` chaining**: Chain commands with `;` to run multiple operations in one call, reducing agent round-trips
- **`-a` vs `-w`**: Use `-a` to find apps by name/title/PID. Use `-w <HWND>` for stable window targeting
- **Element markers**: `[on]`/`[off]` for toggles, `[collapsed]`/`[expanded]`, `[scroll:v]`/`[scroll:h]`/`[scroll:vh]` for scrollable containers, `[offscreen]`, `[disabled]`, `value="..."` for editable elements

## Usage

### Connect and discover
```powershell
# Core loop and examples: `winapp ui --help`. Connect and see interactive elements in one call:
winapp ui status -a myapp; winapp ui inspect -a myapp --interactive
```

### Inspect element tree
```powershell
winapp ui inspect -a myapp --interactive      # elements you can invoke, click, or set-value; auto-depth 8
winapp ui inspect -a myapp --depth 5          # deeper tree at depth 5
winapp ui inspect txt-searchbox-e5f6 -a myapp  # subtree rooted at element
winapp ui inspect btn-settings-a1b2 -a myapp --ancestors  # walk up from element to root
winapp ui inspect -a myapp --hide-offscreen   # hide offscreen elements
```

### Find elements
```powershell
winapp ui search Close -a myapp               # finds elements with "Close" in name or automationId
winapp ui search Button -a myapp              # finds elements with "Button" in name (also matches type names)
winapp ui search image -a myapp               # case-insensitive substring match
```

### Screenshot
```powershell
# App screenshot (may include owned windows in a labeled composite)
winapp ui screenshot -a myapp --output page.png

# Crop to element; capture with popups visible
winapp ui screenshot txt-searchbox-e5f6 -a myapp --output search.png
winapp ui list-windows -a myapp # use the main window's HWND below
winapp ui screenshot -w <hwnd> --capture-screen --output with-popups.png

# Bring window to foreground first (matches what the user is currently seeing)
winapp ui screenshot -a myapp --focus --output focused.png
```

Default capture includes owned windows even with explicit main HWND: one labeled composite, not separate files. Selector for open menu/flyout/tooltip/teaching-tip item crops item's popup; no `--capture-screen` needed. Scope/on-screen overlay placement: [Screenshot](https://github.com/microsoft/winappCli/blob/31fc2ac52b9340846d9b5baa0b9be634ca77a8d1/docs/ui-automation.md#screenshot). With `--on sandbox`, reported path is delivered host destination. Attached WinUI DevTools toolbar, markers, comment panel and inspector excluded from screenshots.

### Record video (H.264 MP4)
Record window/element region to MP4. Prefer positive `--duration-sec N` for agents/scripts; otherwise waits for stop signal. npm helpers require `durationSec` (integer 1–86400); abort signal forcibly cancels, not graceful stop.
```powershell
# Record a window for 10s at 15 fps
winapp ui record -a myapp --duration-sec 10 --fps 15 --output demo.mp4

# Recommended agent evidence: MP4 plus timestamped JPEGs and an NDJSON index
winapp ui record -a myapp --frames --duration-sec 10 --fps 10 --output evidence.mp4 --json

# Include overlays/popups (captures from screen DC; may include occluding windows)
winapp ui record -a myapp --capture-screen --duration-sec 5 --output with-popups.mp4

# Show the mouse pointer and click rings (for demos of click-driven flows)
winapp ui record -a myapp --cursor --duration-sec 10 --output demo.mp4

# Programmatic stop: pipe a newline to stop and finalize the MP4 (for agent/script callers)
"" | winapp ui record -a myapp --json --output capture.mp4
```
- Default `--duration-sec 0` records until Ctrl+C, a newline, or EOF on redirected stdin.
- `--frames` writes `<output-name>.frames` with a manifest, NDJSON index, and changed JPEGs. It supports 1-30 fps and `--max-edge` 64-4096 (default 1280), with a 1 GiB cap. Use `elapsedMs` to bound transitions.
- `--cursor` draws the pointer and a ring at each click; add `--no-click-highlight` to keep the pointer without rings. With `--frames`, each `frames.ndjson` line gains `cursor` (`x`, `y`, `visible`, `inFrame`) and `clicks` (`button`, `kind`, `elapsedMs`, `x`, `y`) in image pixels.
- Existing outputs rejected by default. Use fresh path, or explicitly request `--overwrite` to replace them after new take finishes. Previous frame directories archived, not deleted. Partial failure: retain reported evidence; follow `recoveryHint`.
- Default recording draws open menus, flyouts and tooltips over the window in place; no flag is needed. They're clipped to the window's bounds, and popups from other processes need `--capture-screen` (see [Menus and flyouts in recordings](https://github.com/microsoft/winappCli/blob/31fc2ac52b9340846d9b5baa0b9be634ca77a8d1/docs/ui-automation.md#menus-and-flyouts-in-recordings)).
- `--capture-screen` captures from the screen DC so overlays and popups are included; the window is brought to the foreground first. When WGC is unavailable and `--capture-screen` is not passed, the CLI returns an error — re-run with `--capture-screen` to consent to screen-DC capture. Because the screen DC captures whatever is genuinely in front, the target's foreground is **verified immediately before capture**; if activation was refused the command fails with `foreground_not_target` and writes nothing rather than returning an image of the wrong window.
- Providing a selector that doesn't match any element fails immediately with `element_not_found` (rather than silently recording the whole window).
- A selector for an item in an open menu, flyout, tooltip, or teaching tip records from that item's popup window, not the window behind it.
- `--json` writes the final result to stdout and one JSON event per line to stderr.

### Hover (for tooltips, flyouts, hover states)
`--dwell-time <ms>` sets how long to wait after hovering (default: 800, range: 0–10000).
```powershell
winapp ui list-windows -a myapp # use the main window's HWND below, not the tooltip's

# Hover to trigger tooltip, then capture it (default 800ms dwell)
winapp ui hover btn-info-a1b2 -a myapp; winapp ui screenshot -w <hwnd> --capture-screen --output tooltip.png

# Longer dwell for apps with slow tooltip timers
winapp ui hover btn-info-a1b2 -a myapp --dwell-time 1200; winapp ui screenshot -w <hwnd> --capture-screen
```

### Click or hover a screen point
For point instead of element center (WebView2 page/part of large control), use `--at x,y` with `ui inspect` screen coordinates instead of selector. `--at` cannot combine with selector.
```powershell
winapp ui click --at 1520,780 -a myapp            # add --right or --double as needed
winapp ui hover --at 1520,780 -a myapp
```

### Send keyboard input
Synthesize keystrokes, keyboard counterpart to `click`: arrow/Tab/Enter navigation, shortcuts, per-keystroke typing (versus `set-value` atomic write). Whitespace-separated tokens: named keys (`enter`, `down`, `tab`, `esc`, `f5`), modifier combos (`ctrl+shift+t`), literal text (`hello`), raw virtual keys (`vk=0xNN`).
```powershell
# Keyboard navigation then commit
winapp ui send-keys "down down enter" -a myapp

# Type the literal words "down down enter" instead of pressing those keys (text= escapes each token)
winapp ui send-keys "text=down text=down text=enter" -a myapp

# Same intent, less typing: --verbatim types the whole argument literally (and keeps exact whitespace)
winapp ui send-keys "down down enter" -a myapp --verbatim

# Shortcut: select all and delete
winapp ui send-keys "ctrl+a delete" -a myapp

# Focus a field, then type text into it
winapp ui send-keys "Hello world" --target txt-name-a1b2 -a myapp

# Transport: --via post-message (default, HWND-targeted) or send-input (OS-wide); both are subject to UIPI
winapp ui send-keys "enter" -a myapp --via send-input

# Fire a global hotkey: win+... is refused by default (acts on the shell); opt in with --allow-system-keys
winapp ui send-keys "win+shift+v" -a myapp --via send-input --allow-system-keys

# Screen-reader command: hold capslock or insert (Narrator+Ctrl+F12 toggles Narrator developer mode)
winapp ui send-keys "ctrl+capslock+f12" -a myapp --via send-input
```
- Screen-reader commands hold `capslock` or `insert` as a modifier (`ctrl+capslock+f12`) and require `--via send-input`; post-message errors because screen readers don't receive posted keys. If no screen reader is running, holding `capslock` turns Caps Lock on or off (the command warns); hold `insert` to avoid this.
- A token that starts with a modifier but has an unknown middle segment (`ctrl+a+b`) is an error, not literal text; use `text=` or `--verbatim` to type it.
- Default `post-message` is HWND-targeted and subject to UIPI: a lower-integrity caller cannot post to a higher-integrity target. It can't fire `WH_KEYBOARD_LL` global hotkeys. It automatically retargets to the **focused child control** of the target window, so classic Win32/WinForms child-window controls (e.g. an edit box) receive the input. **WinUI 3 / UWP / XAML controls are windowless and ignore posted `WM_CHAR`/`WM_KEYDOWN`** — post-message can't deliver keys *or* text to them (the command emits a warning and still exits 0, because the CLI does not establish that the target processed the posted message). Use **`--via send-input`** for WinUI 3 / UWP / XAML apps.
- A token that collides with a key/modifier name (e.g. `enter`, `down`, `ctrl+a`) is pressed as that key. Prefix it with `text=` to type it as literal text instead — `text=enter` types the word "enter"; chain `text=` tokens to type a literal phrase like `text=down text=down text=enter`. Backslash escapes inside a `text=` value type whitespace the tokenizer would otherwise collapse: `\s`→space, `\t`→tab, `\n`→newline, `\r`→CR, `\\`→backslash (e.g. `text=a\s\sb` → "a  b"). When the *whole* argument is literal text, pass `--verbatim` instead of escaping each token: it types the entire keys argument as-is (no key/combo/`vk=`/`text=` parsing) and preserves exact whitespace — `send-keys "down down enter" --verbatim` types the words. (`--verbatim` does not decode backslash escapes; use a `text=` token for control characters.)
- `send-input` injects into the foreground window and is subject to UIPI: callers can inject only into applications at equal or lower integrity, so a normal caller cannot drive an elevated target. Additional desktop or application restrictions may still prevent delivery. It **rejects system-reserved combos** (`win+l`, `alt+f4`, `ctrl+shift+esc`, `ctrl+alt+del`, `alt+tab`, …) because those act on the OS/shell, not just the target — pass **`--allow-system-keys`** to opt in (e.g. to fire a global hotkey such as PowerToys' `win+shift+v` or `win+r`), or use `--via post-message` (window-scoped) to send one straight to the window. **`win+l` and `ctrl+alt+del` stay blocked even with `--allow-system-keys`** — `win+l` locks the workstation via `LockWorkStation()` (unrecoverable from automation), and `ctrl+alt+del` is a Secure Attention Sequence (SAS) that Windows drops from injected input regardless of the flag, so it errors (`invalid_arguments`) instead of falsely reporting success. On a locked/secure desktop `send-input` fails fast with `no_interactive_desktop`.
- Per-keystroke events: named keys/combos fire a real `KeyDown` on both transports. For literal typed text, `--via send-input` maps each char to its VK (+Shift) so each character fires a real `KeyDown` + OS-composed `WM_CHAR` (`TextChanged`) — use it when downstream logic keys off `KeyDown` (e.g. WinUI 3/WPF `TextBox`); bring the target window to the foreground first. `--via post-message` posts a `WM_CHAR` per character to the focused child control (can raise `TextChanged` in classic Win32 controls when UIPI permits posting and the target processes it) but does not fire a per-character `KeyDown`, and — because WinUI 3 / UWP / XAML controls are windowless — does **not** reach them at all (named keys and text alike); use `--via send-input` there.
- Long literal text on `--via send-input` is **auto-throttled**: the text is split into small character chunks injected one `SendInput` at a time with a brief pause between the chunks of that one run, so the target can drain its input queue and every character lands. A single unbroken burst overruns the queue and silently drops characters even though the command reports success. The pacing is scoped to a single long text run — short text, key names, and modifier combos (including sequences like `ctrl+a delete`) inject with no added delay, and the command emits an informational warning when a payload is large enough to be throttled. Because each paced chunk lands on whatever window is foreground, `send-input` re-verifies the target still owns the foreground before every continuation chunk and **aborts with `foreground_not_target`** if focus leaves the target mid-injection, so a focus change partway through can't spray the rest of the text into another window (a focus-changing chord such as `alt+tab` is exempt and is never treated as drift). For large bulk text, prefer `set-value` (atomic, no keystrokes or foreground needed) on controls that support it.

### Drag (reorder, resize, sliders, drag-and-drop)
Press, move, release with `drag <from> <to>`; endpoints: element selector (center) or screen `x,y` from `ui inspect`. Uses `SendInput` intermediate moves for realistic `WM_MOUSEMOVE` stream.
```powershell
# Reorder one item onto another (center → center)
winapp ui drag itm-card-9f8e itm-slot-2c1a -a myapp

# Element center → screen coordinates (as reported by `ui inspect`)
winapp ui drag itm-card-9f8e 300,400 -a myapp

# Raw screen coordinates → screen coordinates
winapp ui drag 120,200 480,200 -a myapp

# Right-button drag
winapp ui drag itm-card-9f8e itm-trash-0001 -a myapp --right
```
- A selector drags from/to the element's center; `x,y` are screen coordinates in the same space `ui inspect`/`search` report. Element endpoints are re-resolved just before the drag and fail with `target_moved` if still animating; on a locked/secure desktop the drag fails with `no_interactive_desktop`.

### Touch gestures (tap, swipe, pinch, stretch, long-press)
Inject synthetic touch at element selector's center or explicit `--at x,y` screen coordinate. Prefer modern synthetic-pointer device; fall back to legacy touch-injection API.
```powershell
# Tap an element center; or tap explicit screen coordinates
winapp ui touch btn-ok-1a2b -a myapp
winapp ui touch -a myapp --at 320,240

# Long-press, swipe, and two-finger pinch/stretch (zoom)
winapp ui touch tile-photo-7b3c -a myapp --gesture long-press --hold-ms 600
winapp ui touch -a myapp --at 100,300 --gesture swipe --to-point 400,300
winapp ui touch img-map-9f8e -a myapp --gesture pinch --distance 200
winapp ui touch img-map-9f8e -a myapp --gesture stretch --distance 200 --angle 90
```
- Gestures: `tap` (default), `double-tap`, `long-press`, `swipe`, `pinch`, `stretch`. `--fingers` 1–10 (pinch/stretch always 2). `--angle` 0–359 sets the pinch/stretch finger axis (0 horizontal, 90 vertical). Refuses without a non-zero foregrounded target (`no_target`/`foreground_not_target`/`no_interactive_desktop`); every coordinate is checked against the target window, and a point outside it produces a non-fatal warning (`warnings[]` in `--json`) while injection still proceeds — consistent with the mouse verbs. If injected touch is unsupported on the device, the command surfaces the real Win32 error rather than a false success. In a Remote Desktop/VM session delivery isn't guaranteed even on exit 0 — the command appends a delivery-uncertainty warning (`warnings[]` in `--json`); verify the effect with a screenshot.

### Pen / stylus (taps and ink strokes)
Inject synthetic pen input (Windows 10 1809+) at element center, explicit `--at` or full `--path` ink stroke; pressure/tilt/eraser control.
```powershell
# Pen tap at element center; firm tap at explicit coords
winapp ui pen canvas-1a2b -a myapp
winapp ui pen -a myapp --at 320,240 --pressure 0.8

# Draw an ink stroke, or erase along one
winapp ui pen -a myapp --path "100,100 150,120 210,140 260,120"
winapp ui pen -a myapp --path "100,100 260,100" --eraser
```
- `--pressure` 0.0–1.0, `--tilt-x`/`--tilt-y` ±90°, `--eraser` for the eraser end. Same injection safety as `touch`: requires a non-zero foregrounded target window and checks every ink point (out-of-window → non-fatal `warnings[]` advisory, injection still proceeds). Pen routing is especially unreliable over Remote Desktop — exit 0 can mean the call succeeded but no pen reached the app; the command appends a delivery-uncertainty warning (`warnings[]` in `--json`). Validate pen flows on a local interactive desktop.

### Read element state
```powershell
# Read text/value content (works for RichEditBox, TextBox, ComboBox, Slider, labels)
winapp ui get-value doc-texteditor-53ad -a notepad
winapp ui get-value SearchBox -a myapp
winapp ui get-value CmbTheme -a myapp              # reads ComboBox selected item via SelectionPattern

# Check toggle/selection state, value, scroll position
winapp ui get-property chk-agreecheckbox-b2c3 -a myapp --property ToggleState
winapp ui get-property txt-textbox-a4b1 -a myapp --property Value
winapp ui get-property cmb-modellist-d5e6 -a myapp --property IsSelected

# Read formatting across the whole text document (not the selection)
winapp ui get-property Document -a myapp --property FontWeight --json

# See what has keyboard focus
winapp ui get-focused -a myapp
```

`get-focused`: `-a` includes selected process windows; `-w` restricts focus to exact window, excluding owned popups. Control may belong through parent window despite missing own process ID. See `references/ui-json-envelope.md` for no-match/query-error handling.

Check blank/cleared field: `winapp ui wait-for SearchBox -a myapp --value ""`. See [empty-value behavior](https://github.com/microsoft/winappCli/blob/31fc2ac52b9340846d9b5baa0b9be634ca77a8d1/docs/ui-automation.md#get-value) for JSON output and separate accessibility-label read.

`get-property` also accepts `FontName`, `FontSize`, `ForegroundColor`, `IsItalic`, `StrikethroughStyle`. Omit `--property` for all six formatting attributes. `Mixed`, `NotSupported`, `Unavailable` are distinct states, not formatting values. Names case-sensitive; unknown names fail with `invalid_arguments`. [formatting reference](https://github.com/microsoft/winappCli/blob/31fc2ac52b9340846d9b5baa0b9be634ca77a8d1/docs/ui-automation.md#whole-document-text-formatting): units/state meanings; `references/ui-json-envelope.md`: JSON shape.

### Set values
Prepare keyboard input with `winapp ui focus <selector> -a <app>` (retain `--on sandbox` in Sandbox). Restores minimized window, activates control's window; verifies foreground and keyboard focus before success. If target reported minimized (e.g. Calculator hides UI until restored), run `winapp ui focus -a <app>` (no selector), then retry. No screenshot focus workaround. If activation refused, inspect blocking dialog; ask user to activate intended window. Do not loop retries or switch to local desktop. See [focus reference](https://github.com/microsoft/winappCli/blob/31fc2ac52b9340846d9b5baa0b9be634ca77a8d1/docs/ui-automation.md#focus) for failure codes/recovery.

`set-value` writes programmatically (no keystrokes, no foreground) via a fallback chain: ValuePattern → RangeValuePattern (numeric) → LegacyIAccessible `put_accValue` for TextPattern-only edit controls.
```powershell
winapp ui set-value txt-searchbox-e5f6 "hello" -a myapp        # TextBox/ComboBox via ValuePattern
winapp ui set-value sld-volume-b2c3 75 -a myapp                # Slider via RangeValuePattern
winapp ui set-value doc-compose-9f3a "hello" -a myapp          # RichEdit/compose box via LegacyIAccessible
```
- The LegacyIAccessible fallback reaches rich-edit/compose controls that expose no ValuePattern, as long as their accessibility implements `put_accValue` (native Win32 rich-edit and Chromium/Electron/WebView2 compose boxes typically do).
- **WinUI 3 `RichEditBox` and WPF `RichTextBox` don't support programmatic value-setting** — by design they're read-only to UI Automation's value APIs (Text pattern, no settable Value pattern). `set-value` fails on them with a clear error — use `send-keys` (needs an unlocked, foregrounded desktop) to type into those instead. Note `get-value` can still *read* them via TextPattern.

### Scroll containers
```powershell
# Find scrollable containers — look for [scroll:v] (vertical) or [scroll:h] (horizontal)
winapp ui search scroll -a myapp
# Output:
#   pn-scrollview-bfef Pane "scrollView" [scroll:v] (2127,296 1191x965)
#   pn-scrollviewer-bfb1 Pane "scrollViewer" [scroll:h] (2127,296 1191x216)

# Scroll vertically
winapp ui scroll pn-scrollview-bfef --direction down -a myapp

# Scroll to top/bottom
winapp ui scroll pn-scrollview-bfef --to bottom -a myapp

# Scroll and then inspect for newly visible elements
winapp ui scroll pn-scrollview-bfef --direction down -a myapp; winapp ui search TargetItem -a myapp

# Synthesize real mouse-wheel input over the element (1 = one notch up, -1 = down) — tests wheel handlers (zoom, custom scroll)
winapp ui scroll img-map-a1b2 --wheel -1 -a myapp

# Horizontal (tilt) wheel: 1 = one notch right, -1 = left — pans timelines and other horizontal-wheel handlers
winapp ui scroll pn-timeline-c3d4 --wheel-horizontal 3 -a myapp
```

### Wait for UI state
```powershell
winapp ui wait-for btn-submit-a1b2 -a myapp --timeout 5000
winapp ui wait-for itm-status-c3d4 -a myapp --value "Complete" --timeout 5000
```

## Tips
- Use `--interactive` with `inspect` as your first command — it shows only what you can act on
- Chain commands with `;` to reduce round-trips (see note below on why not `&&`)
- Use slugs from output to target specific elements — they're hash-validated and shell-safe
- Use plain text search to find elements: `search Minimize`, `invoke Submit`
- When text matches several elements, the only element named exactly that text wins; if several still match, the error shows slugs for each — pick the right one
- Use `get-property --property ToggleState` to verify checkbox/toggle state after invoke
- `scroll` auto-finds the nearest scrollable parent
- To capture one item in an open menu, flyout, tooltip, or teaching tip, pass its selector to `screenshot` or `record`; it is captured from its popup window
- Follow [Screenshot](#screenshot) to select a window with `-w <hwnd> --capture-screen` when you need popups in place over the window, or overlays the app doesn't own
- Follow [Hover](#hover-for-tooltips-flyouts-hover-states) to capture tooltips and hover-triggered UI in place
- Use `--focus` to foreground the target window before capture without switching to screen-DC capture (default capture path uses Windows.Graphics.Capture and works while occluded)
- Use `--hide-disabled` and `--hide-offscreen` to reduce noise

### Why `;` instead of `&&`
Examples use `;`: subsequent commands unconditional. Check `$LASTEXITCODE` and observed result before each dependent action; do not continue mutations after failed prerequisite. Diagnostic screenshot may help after failed action. PowerShell 7 supports conditional `&&`; choose control flow deliberately, not from assumed universal native-CLI deadlock.

### File dialog workaround
Standard Windows file open/save dialogs support UIA; use existing commands:
```powershell
# 1. Trigger the dialog (e.g., click "Open File" button)
winapp ui invoke btn-openfilebtn-a2b3 -a myapp

# 2. Find the dialog window
winapp ui list-windows -a myapp
# → Shows the main window + the dialog HWND
# Note: untitled zero-size windows are hidden by default; use --show-hidden to include them

# 3. Target the dialog, type the file path, and confirm
winapp ui set-value txt-1148-c4d5 "C:\path\to\file.png" -w <dialog-hwnd>
winapp ui invoke btn-open-e6f7 -w <dialog-hwnd>
```
Standard file-dialog filename input typically has AutomationId `1148`. Discover actual slugs with `inspect -w <dialog-hwnd> --interactive`.

## JSON output envelopes

`--json` envelope for `ui inspect`, `ui get-focused`, `ui search`, `ui wait-for` reshaped in v0.3.1. DPI context and typed `get-property` element available in v0.6.3+. Highlights:

- `ui inspect --json` now nests elements under `windows[].elements[]` (was a flat `elements[]`).
- Each inspected window and the `ui status --json` target reports `windowDpi`, `scale`, `dpiAwareness`, and `coordinateSpace: "physical-screen-pixels"`. This is the target window's DPI context. The selected target fails fast on an unreadable DPI instead of defaulting to 96; a secondary window that disappears mid-walk carries `dpiError` and omits the four context fields.
- Successful `ui get-focused --json` queries emit `{ "hasFocus": false }` or `{ "hasFocus": true, "element": {...} }` (was bare `null`); query failures exit nonzero with the standard JSON error on stderr, not a `hasFocus` result.
- `ui search --json` returns `{ "matchCount", "hasMore", "matches" }`; `ui wait-for --json` returns `{ "found", "waitedMs", "element"?, "timedOut" }`.
- `ui get-property --json` preserves `elementId` and its string-valued `properties` map, and adds a typed, scrubbed `element`.
- Typed elements use `type` (not `controlType`) and numeric `x`, `y`, `width`, and `height` in physical screen pixels. `0,0,0,0` is UIA's empty/no-displayed-UI rectangle; `isOffscreen` remains independent.
- Search and wait-for elements may include an `invokableAncestor` field (element-shaped).
- Per-element `id`, `parentSelector`, and `windowHandle` are **removed** — use `selector` as the public handle.

Read [the JSON envelopes](references/ui-json-envelope.md) when parsing command output.

## Automating in Windows Sandbox

Optional retained upstream example. Use only already approved/prepared Sandbox; creation/enablement not authorized here.

```powershell
winapp run . --on sandbox --detach
winapp ui inspect --on sandbox -a MyApp
winapp ui screenshot --on sandbox -a MyApp -o .\result.png
```

Use `--detach` before follow-up UI commands; retain `--on sandbox` with guest PIDs/window handles. Inject same `WINAPP_UI_WORKFLOW_ID` in cooperating guest calls; after all complete, finish with `winapp ui yield --on sandbox`.

Optional upstream [winapp-sandbox skill](https://github.com/microsoft/winappCli/blob/31fc2ac52b9340846d9b5baa0b9be634ca77a8d1/plugins/winapp/skills/winapp-sandbox/SKILL.md): setup consent, connected-client requirements, brief setup/reconnect focus changes, capture recovery. App screenshots/recordings, default filenames and `--frames` directories return to host.

## Related skills
These upstream workflows neither bundled nor required by Win32 UIA skill:

- [winapp-sandbox](https://github.com/microsoft/winappCli/blob/31fc2ac52b9340846d9b5baa0b9be634ca77a8d1/plugins/winapp/skills/winapp-sandbox/SKILL.md) describes optional Sandbox automation.
- [winapp-setup](https://github.com/microsoft/winappCli/blob/31fc2ac52b9340846d9b5baa0b9be634ca77a8d1/plugins/winapp/skills/winapp-setup/SKILL.md) describes optional SDK setup.
- [winapp-package](https://github.com/microsoft/winappCli/blob/31fc2ac52b9340846d9b5baa0b9be634ca77a8d1/plugins/winapp/skills/winapp-package/SKILL.md) describes MSIX packaging, outside NevroTick's selected complete-MSI lifecycle.

## Troubleshooting
| Error | Cause | Solution |
|---|---|---|
| "No running app found" | Wrong name or app not running | Try process name, window title, or PID |
| "Multiple windows match" | Several windows match `-a` | Use `-w <HWND>` from the listed options |
| "Selector matched N elements" | Text query matches multiple elements | Use a slug from the suggestions shown in the error, or from `inspect` output |
| "Element may have changed" | Slug hash doesn't match current element | Re-run `inspect` to get fresh slugs |
| "does not support any invoke pattern" | Element can't be invoked | The error shows the invokable ancestor slug if one exists — use that |
| "No UIA window found" | UIA can't see the window | Use `list-windows` to find HWND, then `-w` |
| Popup not in screenshot | Default capture path doesn't include unowned overlays | For one item in an open menu or flyout, pass its selector. To show popups in place or capture unowned overlays, follow [Screenshot](#screenshot) to select a window with `-w <hwnd> --capture-screen` |
| `foreground_not_target` from `--capture-screen` | Windows refused the activation (focus-stealing prevention, UAC prompt, another window activating itself), so a screen capture would have recorded the wrong window | Click the target window, close the window that stole focus, then retry — or drop `--capture-screen` to capture the window directly |
| `element_not_found` during record | Selector given but element not in tree | Re-run `inspect` or `search` to get a fresh selector |
| `ambiguous_selector` during record | Plain-text selector matched multiple elements | Use a slug from the suggestions in the error message, or from `inspect` output |
| WGC unavailable during record | WGC capture init failed; no silent fallback | Check GPU/driver; use `--capture-screen` to explicitly request screen DC capture |

## CLI reference

Current options: `winapp <command> --help`; complete machine-readable schema: `winapp --cli-schema`.
