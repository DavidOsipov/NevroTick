# Local adaptation record

Upstream owner/repository: `microsoft/winappCli`.
Immutable source commit: `31fc2ac52b9340846d9b5baa0b9be634ca77a8d1`.
Adapted on 28 October 2026 for NevroTick.

Source files:

- [SKILL.md](https://github.com/microsoft/winappCli/blob/31fc2ac52b9340846d9b5baa0b9be634ca77a8d1/plugins/winapp/skills/winapp-ui-automation/SKILL.md)
- [references/ui-json-envelope.md](https://github.com/microsoft/winappCli/blob/31fc2ac52b9340846d9b5baa0b9be634ca77a8d1/plugins/winapp/skills/winapp-ui-automation/references/ui-json-envelope.md)
- [Root MIT LICENSE](https://github.com/microsoft/winappCli/blob/31fc2ac52b9340846d9b5baa0b9be634ca77a8d1/LICENSE), retained as `LICENSE`.

Local changes:

- Quoted frontmatter name/description, narrowed automatic routing to qualified
  native UIA command use, and recorded the MIT license.
- Added NevroTick scope: project-owned accessibility/E2E requirements remain
  authoritative; retain native Win32 and the selected complete-MSI architecture.
  The skill does not adopt WinUI, .NET, Windows App SDK or MSIX.
- Required a qualified Windows/CLI environment, approved isolated test session
  and explicitly scoped application/HWND. Loading the skill does not authorize
  installation, elevation, Sandbox enablement, DevTools attachment/injection,
  system-wide shortcuts or unrelated application interaction.
- Required observed target effects and separate Narrator evidence. CLI exit zero
  cannot establish message delivery or accessibility conformance.
- Corrected four upstream keyboard descriptions: the transport comment,
  PostMessage's claimed cross-integrity behavior, SendInput's reversed integrity
  direction, and the per-character cross-integrity claim. Both transports are
  subject to UIPI. Clarified that posted input depends on target processing.
- Replaced the blanket any-app/no-setup claim with actual CLI, target/provider
  and access prerequisites.
- Converted optional sibling skills to pinned upstream GitHub links and marked
  them unbundled. Pinned external repository documentation links in both
  Markdown files; kept the required JSON-envelope reference local and linked it.
- Marked the retained Sandbox example conditional on separate preparation and
  authorization. Marked upstream MSIX packaging outside NevroTick's selected
  complete-MSI lifecycle.
- Replaced the unsupported universal PowerShell `&&` deadlock claim with
  explicit conditional/unconditional chaining and prerequisite-failure handling.
- Added `agents/openai.yaml` interface metadata without an invocation-policy
  override. Added this adaptation record. No executable helpers, hooks or
  runtime configuration were installed.

Evidence for the input corrections:

- [PostMessageA API](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-postmessagea)
  documents UIPI restrictions and the Boolean result.
- [SendInput API](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendinput)
  permits injection only into applications at equal or lower integrity.
- [Pinned KeyStringParser.cs](https://github.com/microsoft/winappCli/blob/31fc2ac52b9340846d9b5baa0b9be634ca77a8d1/src/winapp-CLI/WinApp.UIAutomation/Input/KeyStringParser.cs)
  independently describes both transports' UIPI limits.
- [Pinned KeyboardInput.cs](https://github.com/microsoft/winappCli/blob/31fc2ac52b9340846d9b5baa0b9be634ca77a8d1/src/winapp-CLI/WinApp.UIAutomation/Input/KeyboardInput.cs)
  calls `PInvoke.PostMessage` from a void adapter without checking its result.
- [PowerShell pipeline-chain operators](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_pipeline_chain_operators)
  documents `&&` support from PowerShell 7 and conditional execution.

Limits: this is a documentation/metadata adaptation. No Windows application,
winapp runtime, UIA interaction, capture or Narrator behavior was executed.
The upstream JSON reference describes v0.3.1 and v0.6.3+ feature changes; this
record does not certify release compatibility or the full command reference.
Check the actual installed CLI's version, command help and observed JSON output
on the selected Windows test machine before relying on those commands.

Later local change: compressed instructional prose; preserved code, metadata,
headings, links, technical conditions and licence texts. Upstream sources/pins unchanged.
