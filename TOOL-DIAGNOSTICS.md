# Tool updates and diagnostics — 28 October 2026

Updates use the existing installation channels. This report describes fresh
process checks; the current chat retains previously loaded Serena code and
SocratiCode skills until a session restart.

| Tool | Before | After | Diagnostic outcome |
| --- | --- | --- | --- |
| Serena | 1.7.0 | 2.1.0 | Native Rust health-check passes in an isolated fixture; fresh MCP initializes and lists 23 tools. NevroTick health-check exits 1 because no analyzable source exists yet. |
| SocratiCode plugin | 1.16.0 | 1.16.1 | Infrastructure, index, retrieval, and Rust parser checks pass. MCP runtime was already 1.16.1. Ruby parser warning remains. |
| Probe | 0.6.0-rc344 | 0.6.0-rc344, already latest | Official LSP doctor, daemon ping, fresh MCP, Rust search/extraction, and ignore checks pass. |
| Context-mode | 1.0.169 | 1.0.169, already latest | Updater completed. Server, SQLite/FTS5, and Rust execution pass. Two doctor warnings remain, explained below. |

## Serena

Updated with `uv tool upgrade serena-agent`. The existing uv tool environment
and command entrypoints were retained. `uv pip check` reports all 74 installed
Python packages compatible.

- `serena project index /home/david_osipov/NevroTick --log-level INFO` succeeds
  and starts rust-analyzer.
- Native `project health-check` against this checkout exits 1:
  **No analyzable files found**. The repository has no Cargo project or Rust
  source. This is not a passing application health-check.
- The same native health-check passes against an isolated Rust fixture, including
  symbol overview, definition lookup, and one actual reference.
- The exact configured command,
  `serena start-mcp-server --context=codex --project-from-cwd`, initializes as
  Serena 2.1.0 and exposes 23 Codex tools. Only the diagnostic subprocess was closed.

Serena 2.1 adds a session argument to project activation: call
`initial_instructions()` first, then pass its Serena session ID to
`activate_project(project="/home/david_osipov/NevroTick", session=<Serena ID>)`.
The argument is `session`, not `session_id` or an MCP/client session identifier.
The repository agent instructions have been updated accordingly.

Serena 2.1 automatically re-saved its registered project configurations on first
load. It migrated `~/.serena/serena_config.yml` plus `.serena/project.yml` under:

- `/home/david_osipov`
- `/home/david_osipov/NevroTick`
- `/home/david_osipov/ai-trading-automation`
- `/home/david_osipov/nplg-dspace-mcp`
- `/home/david_osipov/pdfcraft`
- `/home/david_osipov/personal-website-templated-1`
- `/home/david_osipov/telemt`

Observed schema additions include API inclusion/exclusion lists, agent-interface
defaults, refreshed comments, and a global authentication secret. NevroTick's
Rust selection, target exclusion, initial prompt, and local override were
preserved. Pre-migration hashes were captured, but full pre-migration copies
were not, so exact byte diffs are unavailable.

Post-migration copies of all eight files are restricted to mode 0600 under
`/tmp/serena-2.1-migration-receipt-ch_c0gr6`. These copies can contain private
configuration; they are not included in this report.

Native diagnostic logs:

- Checkout: `.serena/logs/health-checks/health_check_20261028-185335.log`
- Rust fixture: `/tmp/nevrotick-serena-health-99q8rjrf/.serena/logs/health-checks/health_check_20261028-185337.log`
- Fresh MCP: `/tmp/serena-2.1-mcp-1j7ridjj.log`

Sources: [supported installation/update commands](https://oraios.github.io/serena/02-usage/010_installation.html),
[release changes](https://github.com/oraios/serena/blob/main/CHANGELOG.md).

## SocratiCode

Updated only this plugin with
`codex plugin add socraticode@personal-marketplace --json`.
Codex confirms the bundle installed and enabled at
`~/.codex/plugins/cache/personal-marketplace/socraticode/1.16.1`.
No standalone registration was added.

The official `codebase_health` checks pass for Docker, Qdrant, external Ollama,
and `nomic-embed-text`. Before this report was written, the repository index had
673 chunks, an active automatic watcher, and seven context artifacts with 668
chunks. Fresh-process context retrieval and Rust grammar/import parsing pass.
The source graph is ready with zero nodes/edges, as expected without Rust source.
Index counts can increase when this report is indexed.

The full parser diagnostic reports an unavailable Ruby parser on this x64 host.
The installed `@ast-grep/lang-ruby 0.0.6` package contains an ARM64 prebuild and no
locally built x64 parser. Rust parsing works. No unrelated parser dependency was
changed to suppress that warning.

Source: [official 1.16.1 release](https://github.com/giancarloerra/SocratiCode/releases/tag/v1.16.1).

## Probe

The installed npm wrapper matches npm's current `latest` tag:
`0.6.0-rc344`. Refreshed its ordinary npx resolution with `--prefer-online`;
no unused global installation was added. The native binary reports
`probe-code 0.6.0`, build git `f442c01`.

- `probe lsp doctor`: exit 0, **Doctor completed successfully**.
- `probe lsp ping`: responsive, 0 ms in the observed check.
- Rust/rust-analyzer availability: pass.
- Fresh stdio MCP: initializes as `@probelabs/probe 0.6.0-rc344` with
  `search_code`, `extract_code`, and `grep`.
- Isolated Rust AST search, complete function extraction, Cargo target exclusion,
  and actual repository document search: pass.

The diagnostic created an idle LSP daemon; doctor restarted that diagnostic
daemon and left it responsive. The existing MCP connection was not terminated.
No custom `~/.probe/settings.json` was found; defaults remain in use.

Source: [official rc344 release](https://github.com/probelabs/probe/releases/tag/v0.6.0-rc344).

## Context-mode

Ran the command returned by `ctx_upgrade`:

```sh
node /home/david_osipov/.codex/plugins/cache/context-mode/context-mode/1.0.169/cli.bundle.mjs upgrade --platform codex
```

The updater refreshed its marketplace and confirmed version 1.0.169 is already
latest. npm's live latest tag and the GitHub latest release agree.
It backed up `~/.codex/config.toml`; the parsed settings remained unchanged.

The updater added six native user hooks even though the plugin already provides
those events. Removed only those six new Context-mode entries from
`~/.codex/hooks.json`, preserving plugin hooks and any unrelated entries.
The final user-hook file contains an empty hooks object. The pre-cleanup file
and a separate configuration backup are retained under
`/tmp/nevrotick-tool-update-g6aoajaj`. Duplicate-hook doctor warnings are gone.

Two warnings remain:

- Bun is absent. Rust execution works without it; no additional runtime was
  installed.
- The plugin-root lookup uses Node's default subprocess buffer. It fails with
  `ENOBUFS` on the 1,712,796-byte `codex plugin list` output. The same command
  with a 16 MiB buffer succeeds and confirms Context-mode installed/enabled at
  1.0.169. No installed plugin source was patched.

An independent Rust sandbox assertion passes after the updater.
Hook registration was checked; actual hook-event execution was not independently
exercised.

Final native MCP doctor output:

```text
context-mode doctor

[OK] Runtimes: 6/11 (55%) — javascript, shell, python, go, rust, perl
[WARN] Performance: NORMAL — install Bun for 3-5x speed boost
[OK] Storage sessions: /home/david_osipov/.codex/context-mode/sessions (default)
[OK] Storage content: /home/david_osipov/.codex/context-mode/content (default)
[OK] Storage stats: /home/david_osipov/.codex/context-mode/sessions (default)
[OK] Server test: PASS
[OK] FTS5 / SQLite: PASS — native module works
[OK] Codex CLI binary: codex --version resolved to codex-cli 0.164.0-alpha.11.1
[OK] Codex hooks feature flag: [features].hooks enabled in /home/david_osipov/.codex/config.toml
[WARN] Codex plugin root: context-mode@context-mode is enabled, but `codex plugin list` did not report its runtime root — fix: Restart Codex or verify `codex plugin list` shows context-mode@context-mode installed and enabled
[OK] PreToolUse hook: PreToolUse hook provided by context-mode@context-mode plugin
[OK] PostToolUse hook: PostToolUse hook provided by context-mode@context-mode plugin
[OK] SessionStart hook: SessionStart hook provided by context-mode@context-mode plugin
[OK] PreCompact hook: PreCompact hook provided by context-mode@context-mode plugin
[OK] UserPromptSubmit hook: UserPromptSubmit hook provided by context-mode@context-mode plugin
[OK] Stop hook: Stop hook provided by context-mode@context-mode plugin
[OK] Hook scripts: no direct .mjs script paths to verify
[OK] Version: v1.0.169
```

Source: [official 1.0.169 release](https://github.com/mksglu/context-mode/releases/tag/v1.0.169).

## Repository verification and next session

All nine entries in `docs/SHA256SUMS.txt` still verify.
No application scaffold, application dependency changes, or commits were made.
Serena's generated diagnostic logs are excluded by `.serena/.gitignore`.

Restart the Codex session to load Serena 2.1.0 and SocratiCode's updated skills.
Probe and Context-mode did not change versions.

