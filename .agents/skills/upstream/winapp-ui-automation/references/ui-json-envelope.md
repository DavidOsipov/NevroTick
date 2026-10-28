# `winapp ui --json` envelopes

`--json` output for `winapp ui` uses envelopes below. Inspect/search/wait-for/get-focused reshaped in v0.3.1; DPI context and typed get-property element available in v0.6.3+.

## `ui inspect --json`

Top-level shape (elements nested under `windows[]`, not flat):

```json
{
  "depth": 0,
  "interactive": false,
  "hideDisabled": false,
  "hideOffscreen": false,
  "windows": [
    {
      "hwnd": 123456,
      "title": "...",
      "className": "...",
      "windowDpi": 144,
      "scale": 1.5,
      "dpiAwareness": "per-monitor-aware",
      "coordinateSpace": "physical-screen-pixels",
      "elementCount": 0,
      "elements": [
        {
          "selector": "btn-save-c3d4",
          "name": "Save",
          "type": "Button",
          "isEnabled": true,
          "isOffscreen": false,
          "x": 100,
          "y": 200,
          "width": 120,
          "height": 32,
          "isInvokable": true
        }
      ]
    }
  ]
}
```

Pre-0.3.1: `{ "elements": [...] }`. Per-element `id`, `parentSelector`, `windowHandle` **removed**; `selector` is public handle.

`windowDpi`: target window's effective DPI from `GetDpiForWindow(hwnd)`, not unconditional monitor DPI. `scale`: `windowDpi / 96`. `dpiAwareness`: `unaware`, `system-aware` or `per-monitor-aware`. `GetDpiForWindow` reports 96 for unaware window, system DPI for system-aware, current monitor DPI for per-monitor-aware. Unreadable HWND/DPI context fails with error, never substitutes 96.

Selected target remains fail-fast. Later popup/secondary window disappearing after UIA collection stays in `windows[]` with `dpiError`, without four DPI context fields; other trees remain available.

Element `x`, `y`, `width`, `height`: numbers in physical screen pixels. `0,0,0,0`: UI Automation's empty/no-displayed-UI rectangle here. `isOffscreen` independent; offscreen bounds may be nonzero. `isEditable: true` marks writable value; `set-value` may work on others.

## `ui inspect --ancestors --json`

Ancestors nested parent → child, keyed by `Depth=i` (previously sibling roots).

## `ui inspect --interactive`

Non-interactive ancestors collapsed into surviving descendants' `ancestorPath`. `+more` marks truncated subtrees in text and JSON.

## `ui get-focused --json`

Successful queries: envelope, never bare value:

- No focused element can be verified as belonging to the target: `{ "hasFocus": false }`
- With focus: `{ "hasFocus": true, "element": { ... } }`

`-a` includes selected process windows. `-w` requires exact window, not same-process sibling/owned popup. May verify ownership through parent window when control lacks own process ID.

Focus/window-ownership query failure: nonzero exit, standard `{ "error": { ... } }` on stderr, no `hasFocus` result. Retry `get-focused`; if window closed, rediscover with `list-windows`.

Pre-0.3.1: bare `null` without focus.

## `ui search --json`

Search returns envelope, not bare array:

```json
{
  "matchCount": 1,
  "hasMore": false,
  "matches": [
    {
      "selector": "txt-save-label-a1b2",
      "name": "Save",
      "type": "Text",
      "isEnabled": true,
      "isOffscreen": false,
      "x": 100,
      "y": 200,
      "width": 80,
      "height": 24,
      "isInvokable": false,
      "invokableAncestor": {
        "selector": "btn-save-c3d4",
        "name": "Save button",
        "type": "Button",
        "isEnabled": true,
        "isOffscreen": false,
        "x": 90,
        "y": 196,
        "width": 120,
        "height": 32,
        "isInvokable": true
      }
    }
  ]
}
```

Match may include element-shaped `invokableAncestor`: nearest parent supporting `InvokePattern`, useful for non-invokable hits such as button labels.

## `ui wait-for --json`

When the condition succeeds:

```json
{
  "found": true,
  "waitedMs": 125,
  "element": {
    "selector": "txt-status-a1b2",
    "name": "Ready",
    "type": "Text",
    "isEnabled": true,
    "isOffscreen": false,
    "x": 100,
    "y": 200,
    "width": 80,
    "height": 24,
    "isInvokable": false
  },
  "timedOut": false
}
```

Timeout: parseable stdout result; exit 1:

```json
{
  "found": false,
  "waitedMs": 5000,
  "timedOut": true
}
```

`--gone` success after element disappears:

```json
{
  "found": false,
  "waitedMs": 125,
  "timedOut": false
}
```

## `ui get-property --json`

`elementId` and string-valued `properties` remain. Added `element` contains same scrubbed typed projection as search/wait-for:

```json
{
  "elementId": "btn-save-c3d4",
  "element": {
    "selector": "btn-save-c3d4",
    "name": "Save",
    "type": "Button",
    "isEnabled": true,
    "isOffscreen": false,
    "x": 100,
    "y": 200,
    "width": 80,
    "height": 24,
    "isInvokable": true
  },
  "properties": {
    "Name": "Save",
    "IsEnabled": "True",
    "BoundingRectangle": "100,200,80,24"
  }
}
```

Typed `element`: canonical geometry/boolean-state consumer interface. `properties` intentionally remain strings for backward compatibility.

### Whole-document formatting

```bash
winapp ui get-property Document -a myapp --property FontWeight --json
```

Envelope unchanged; `properties` contains requested formatting value, e.g. `"FontWeight": "700"`. Omit `--property` for six [whole-document formatting attributes](https://github.com/microsoft/winappCli/blob/31fc2ac52b9340846d9b5baa0b9be634ca77a8d1/docs/ui-automation.md#whole-document-text-formatting). Reference defines units, omission behavior and `Mixed`, `NotSupported`, `Unavailable` states.

Property names case-sensitive; unknown names fail with `invalid_arguments` on stderr, not successful null. Omit `--property` to discover valid names.

## `ui status --json`

Resolved target also reports window DPI context:

```json
{
  "processId": 1234,
  "processName": "MyApp",
  "windowTitle": "My App",
  "hwnd": 123456,
  "windowDpi": 144,
  "scale": 1.5,
  "dpiAwareness": "per-monitor-aware",
  "coordinateSpace": "physical-screen-pixels"
}
```

Process resolved before top-level window: `hwnd` remains `0`; omit four DPI fields. Nonzero HWND with failed DPI read: error, not silent 96-DPI fallback.

Internal `id`, `parentSelector`, `windowHandle` **scrubbed** from typed results, top-level and nested `invokableAncestor`. Do not depend on them; use `selector` handle.

## `ui invoke --json`

```powershell
winapp ui invoke AgreeCheckbox -a myapp --action toggle-on --json
```

```json
{
  "elementId": "AgreeCheckbox",
  "pattern": "TogglePattern",
  "requestedAction": "toggle-on",
  "performedAction": "toggle",
  "hwnd": 12345
}
```

`requestedAction` is the supplied action, or `"auto"` when `--action` is omitted.
`performedAction` is `"invoke"`, `"select"`, `"toggle"`, `"expand"`, or `"collapse"`.
An already-correct `toggle-on` or `toggle-off` returns `"none"` without toggling;
`pattern` still identifies the pattern used to read the state.
`elementId` identifies the element acted on; automatic mode can report an
ancestor, while explicit mode never acts on an ancestor.
`hwnd` identifies that element's source window, which can differ from the main
window when an app-scoped command selects a control in a secondary window.
Failures emit the error envelope below on stderr, with a nonzero exit code and
no success result. See the [action reference](https://github.com/microsoft/winappCli/blob/31fc2ac52b9340846d9b5baa0b9be634ca77a8d1/docs/ui-automation.md#invoke)
for action semantics and recovery.

## Error envelope

Every `winapp ui` command writes errors to **stderr** as:

```json
{
  "error": {
    "code": "element_not_found",
    "message": "…",
    "selector": "btn-save-c3d4",
    "details": "…",
    "suggestions": ["…"],
    "recoveryHint": "…"
  }
}
```

Only `code` and `message` always present; omit inapplicable fields. `suggestions` lists up to two command names for unknown `winapp ui` command (e.g. `ui dump` suggests `inspect`).

### Desktop coordination

Concurrent `winapp ui` workflows take cooperative desktop turns (skill coordination section). Four additional codes may appear:

| `code` | Meaning |
|---|---|
| `invalid_ui_workflow_id` | `WINAPP_UI_WORKFLOW_ID` is set but empty/whitespace or longer than 256 characters. Fails before any UI side effect. |
| `desktop_coordination_unavailable` | Coordination state could not be read, published, or safely rebuilt — including state written by a newer `winapp`. Mutating commands fail closed rather than acting uncoordinated. If the coordination folder is only access-denied, commands run uncoordinated instead of returning this error. |
| `queue_capacity_exceeded` | 64 commands from other workflows are already waiting for the desktop. Counts live foreign waiters, so entries left by commands that exited or were killed do not occupy a slot. |
| `ui_turn_busy` | `ui yield` was run while this same workflow still has a command running or queued, so its turn is not idle. Nothing was released, and the running command is unaffected. Distinct from `invalid_arguments` (the request was well formed) and from `desktop_coordination_unavailable` (coordination is working — this is a valid request at an unsafe moment). Carries a `recoveryHint`: wait for or stop this workflow's other `winapp ui` commands — typically a `record` started with the same `WINAPP_UI_WORKFLOW_ID` — then retry `yield`. |
| `cancelled` | Native Ctrl+C while the command was still waiting for its turn. The command never ran, so it has no UI side effects. Exit code **130**. |

`ui yield` emits command-level `invalid_arguments` when `WINAPP_UI_WORKFLOW_ID` absent; not `invalid_ui_workflow_id`, meaning present but malformed. Success: `{ "released": true }`, or `{ "released": false }` if nothing held (both exit **0**).

npm `AbortSignal` differs: Node force-terminates child, usually no envelope/exit code 130; wrapper rejects with `AbortError`. UI effects may already exist if abort followed desktop acquisition.

`cancelled` (optionally other coordination errors) carries added `coordination`:

```json
{
  "error": {
    "code": "cancelled",
    "message": "UI turn wait was cancelled.",
    "coordination": {
      "waitedMs": 1234,
      "queuePosition": 2
    }
  }
}
```

Queued cancellation always includes `waitedMs`. `queuePosition` one-based among live waiters; **omitted** if unreliable, including wait behind own workflow's earlier command. Never expose raw/hashed workflow identities.

Cancellation *after* turn acquisition preserves existing command behavior: Ctrl+C during `ui record` still finalizes recording and returns normal success.
