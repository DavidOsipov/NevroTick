# Rust data-flow analysis with the local CodeQL MCP

Use this reference for a concrete source-to-sink question across Rust functions,
validation, state or trust boundaries. [TOOLING.md](../../../../TOOLING.md) owns
installation and qualification status; [the local policy](../../../../policy/codeql-mcp.json)
owns pins, allowed tools, limits and retained advisories. Hosted CodeQL remains
deferred. A local MCP connection does not establish a hosted scan or product pass.

## Prepare the evidence

1. State the suspected flow: external or secret source, value/taint propagation,
   intended guards, observable sink and affected owners. Distinguish value flow,
   taint flow, control-flow reachability and authorization; they answer different
   questions. Keep the applicable architecture REQ/BND/THR/CHECK identities.
2. Record checkout/source identity, manifest/lockfile, target, features, cfgs,
   macro/build-script inputs and tool/query/model versions. Inspect extraction
   logs and unresolved code. Retain hashes of authored queries, fixtures and
   expected results outside the runtime work directory; compare them after each
   run. Upstream compilation/test artifacts share a writable workspace, so
   launcher admission alone does not freeze those inputs during a session.
   Rust extraction can execute Cargo build scripts and
   proc macros even with `--build-mode=none`; use the isolated offline launcher.
3. Admit the environment with `python3 -B scripts/codeql-mcp.py --check`. For an
   actual Cargo tree, create a new database using
   `python3 -B scripts/codeql-mcp.py --create-database /absolute/source --name NAME`.
   Sources must be this checkout or its owned external fixtures. Do not create
   a product workspace just to satisfy analysis. Missing cached dependencies,
   unsupported targets or unresolved extraction remain incomplete.
4. Inspect `codeql_resolve_database` and query a nonempty inventory of the exact
   source/functions needed. A database directory or a successful command alone
   does not show that the intended code was extracted. Rebuild after relevant
   source/configuration changes; do not silently reuse a stale database.

The configured server is `codeql_dev`. Discover its current exposed schema after
restarting Codex; do not invent calls if this session has not loaded the server.
The private host workspace is recorded in
`~/.local/share/nevrotick/codeql-installation.json`; it appears as `/work` inside
the server. Use host paths for file edits and `/work/...` paths in MCP arguments.
Keep query packs in `/work/queries`, outputs/logs below `/work/results`, and use
the admitted offline tool pack `/work/packs/rust-tools` explicitly for built-ins.

## Trace and challenge the flow

- Use `search_ql_code`, LSP definitions/references/diagnostics and the pinned
  `codeql/rust-all` sources to confirm current Rust query APIs. `PrintAST`,
  `PrintCFG` and call-graph queries through `codeql_query_run` help locate code;
  their diagrams do not establish a taint path.
- Write a small `DataFlow::Global` or `TaintTracking::Global` configuration with
  explicit sources, sinks and any required additional steps. Model a barrier
  only when the actual checked value/branch blocks that particular flow.
  Deserialization, a type name, an error-returning function or a logging wrapper
  is not automatically a sanitizer. Preserve bounded Serde admission, private
  fallible construction and current authorization/generation/effect guards.
- Compile with `codeql_query_compile` (warnings fail), passing `query` only.
  The installed server advertises a `database` option for this tool, but CLI
  2.27.3 rejects it; provide the database to execution instead. Then use
  `codeql_query_run` against the identified database and inspect actual BQRS
  rows/path endpoints with `codeql_bqrs_info` and `codeql_bqrs_decode`.
  `codeql_bqrs_interpret` can emit supported graph/SARIF results. Compilation or
  zero results alone cannot substantiate an absence-of-flow claim.
- Use independently specified reachable unsafe and clean fixtures. Verify the
  expected positive path survives helper calls and the clean control is absent;
  challenge a proposed guard with a bypass case. Use `codeql_test_run` for owned
  query tests when appropriate, with authored expected results. The supervisor
  blocks learning/accepting baselines and arbitrary CLI argument overrides.
- Confirm each reported path in current source/callers. For missing paths,
  distinguish genuinely blocked flow from omitted files, unsupported syntax,
  macros, cfg branches, absent models or query mistakes. Findings remain leads
  until their effect and reachability are independently established.

Useful NevroTick flows include IPC bytes → DTO → domain constructor → current
authorization → broker effect; NTP/NTS bytes → parser → selection/correction;
and secrets → formatting → logs/exports. Cross-process IPC, Windows tokens,
native FFI and actual clock effects need separate contracts/models and native
observations. A Rust query does not automatically bridge those boundaries.
Apply UI-M0's current deferrals before selecting UI/service integration flows.

## Finish and retain limits

Report the database/source/target identity, query/model version, extracted
denominator, executed controls, actual paths, confirmed guards and unresolved
coverage. Pair the result with focused behavioral tests; select Miri, fuzzing,
Loom, Kani, Lockbud or cargo-vet through
[configured tooling](configured-tooling.md) when their distinct guarantee matters.

The server has an explicit tool allowlist, bounded protocol and an offline OS
sandbox. Its bundled Zod advisory remains recorded; these restrictions do not
make dependencies vulnerability-free. The launcher enforces process limits and
terminates timed-out work; RAM hints are not an aggregate memory quota. Keep
timeouts, unavailable checks and extraction warnings visible. Do not bypass the
launcher, enable hidden tools, fetch packages or accept test baselines to turn a
failed or incomplete analysis into a pass.
