# Local adaptations

Upstream: [po4yka/rust-skills, rust-send-sync](https://github.com/po4yka/rust-skills/tree/5d5f4706d6bf1d03366a1fb463aa1732de2dbbd2/skills/rust-send-sync),
commit `5d5f4706d6bf1d03366a1fb463aa1732de2dbbd2`.
Source files: `skills/rust-send-sync/SKILL.md` and
`skills/rust-send-sync/references/phantomdata-markers.md`.
The repository-root BSD-3-Clause [LICENSE](LICENSE) is retained unchanged,
including copyright 2026 Nikita Pochaev. Adapted for NevroTick on 2026-10-28.

Changes from that pin:

- JSON-quoted frontmatter scalars and narrowed discovery to actual Send/Sync obligations.
- Added repository/architecture precedence, real owner/caller and Windows adapter boundaries,
  local `!Send` allowance, and the continuing `tokio::spawn` Send bound. Prevented a diagnostic
  from implicitly authorizing runtime, owner, Arc/Rc, lock or manual unsafe-impl changes.
- Marked upstream compiler/macOS diagnostics as historical illustrations, with no local
  execution or Windows qualification claim.
- Required the selected compiler/target/features and actual consumer, valid controls and
  causal negative diagnostics through existing NevroTick QA skills.
- Replaced universal future Send/lint instructions with real-consumer assertions and optional,
  focused Clippy use; removed automatic manifest lint edits and expectation annotations.
- Qualified the Arc-to-Rc suggestion and signature-only repair statement by actual contracts.
- Made absent related skills nonblocking through project implementation and QA fallbacks;
  tools, runtime and dependency changes remain outside this skill's installation.
- Corrected the generic marker table: `PhantomData<Cell<T>>` retains Send only when `T: Send`,
  and `PhantomData<MutexGuard<'static, T>>` retains Sync only when `T: Sync`. Clarified that a
  marker cannot restore a trait excluded by another field.
- Added `agents/openai.yaml` interface metadata with `$rust-send-sync`; no invocation policy
  override, tools, executable helpers or new skill dependencies.

Primary sources checked for these adaptations:

- [Rust Sync definition and reference rules](https://doc.rust-lang.org/std/marker/trait.Sync.html).
- [Rust MutexGuard Send/Sync implementations](https://doc.rust-lang.org/std/sync/struct.MutexGuard.html).
- [Rust Cell Send/Sync implementations](https://doc.rust-lang.org/std/cell/struct.Cell.html).
- [Rustonomicon PhantomData behavior and marker patterns](https://doc.rust-lang.org/nomicon/phantom-data.html).
- [Tokio spawn bounds and runtime behavior](https://docs.rs/tokio/1.54.1/tokio/task/fn.spawn.html).
- [Clippy future_not_send scope and limitations](https://clippy.rust-lang.org/master/index.html#future_not_send).
- [Rustdoc compile-fail examples](https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html).

Only skill documents and interface metadata were adapted. No Rust example, external code,
Cargo command, target runner or Windows/native operation was executed for this adaptation.

Later local change: compressed instructional prose; preserved code, metadata,
headings, links, technical conditions and licence texts. Upstream sources/pins unchanged.
