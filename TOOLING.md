# Rust development tools

Prepared on 28 October 2026. The repository currently contains planning documents;
it has no `Cargo.toml` or Rust source yet. Tool setup does not create an application
scaffold or establish that Windows behavior has been tested.

## Available tools

| Tool | Project setup | Verified behavior |
| --- | --- | --- |
| Serena | `.serena/project.yml` enables Rust/rust-analyzer and excludes `**/target/**`. | A fresh `serena project index /home/david_osipov/NevroTick --log-level INFO` loaded the config, started rust-analyzer, and completed with zero source files. |
| SocratiCode | `.socraticodeignore` excludes Serena state and Cargo output; `.socraticodecontextartifacts.json` names the documentation sources and their authority. | Local infrastructure healthy; initial document and context indexes completed; semantic retrieval returned the canonical architecture. This checkout's automatic watcher is active. |
| Probe | Uses the existing MCP connection and `.gitignore`. No project index is required. | Rust function search and AST extraction passed in an isolated temporary fixture; `target/` content was excluded. Repository documentation search also passed. |
| Context-mode | Uses the existing plugin. Storage is outside this checkout under `~/.codex/context-mode/`. | Rust sandbox execution, SQLite/FTS5, architecture indexing, and scoped retrieval passed. |

Initial preparation reused all four existing connections. Subsequent tool
updates and fresh diagnostics are recorded in [TOOL-DIAGNOSTICS.md](TOOL-DIAGNOSTICS.md),
including Serena 2.1.0 and SocratiCode 1.16.1.

**Restart this Codex/Serena session once** to apply Serena's new Rust setting to
the current connection. The existing Serena process caches the previous project
configuration; activating the same project again does not reload it.

The host already provides Rust/Cargo 1.94.0, rust-analyzer, rust-src, rustfmt, and
Clippy. These are observed host versions, not a selected project MSRV or toolchain
pin. Choose the project's toolchain when establishing its Cargo workspace.

Context-mode's doctor reported working runtimes, storage, server, and hook
configuration. Its plugin-root lookup warning was traced to the diagnostic
subprocess's output buffer: a larger-buffer check confirmed the plugin installed
and enabled. Hook event execution was not independently exercised. Bun is optional
and was not installed as part of this setup.

## Working in the repository

Start with `docs/README.md` and the canonical architecture. Use SocratiCode's named
context artifacts to distinguish architecture from retained reviews and proposed
update designs. Context-mode also has a searchable architecture source named
`NevroTick architecture RWTC-ARCH-001` in its current session.

Pass the checkout's absolute path to code tools to avoid searching another
project. SocratiCode follows edits in its configured automatic mode. If its
watcher is unavailable, check status and run `codebase_update` for this checkout.
Recheck context artifacts after changing documentation; an index is retrieval
support, not proof of the current file's contents.

After a Cargo project exists, run checks appropriate to the change from its
workspace root:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Use the project's eventual feature/target matrix for additional checks. Windows
service, IPC, installer, and clock behavior require appropriate Windows execution;
host tool smoke checks do not cover those behaviors.

Verify the existing documentation bundle from `docs/` with:

```sh
sha256sum --check SHA256SUMS.txt
```
