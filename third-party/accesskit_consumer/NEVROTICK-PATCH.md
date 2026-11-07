# AccessKit consumer word-navigation correction

This derivative retains accesskit_consumer 0.38.0 from AccessKit revision
`c88605b96d04431f9c3c792464a0f2f253480e94`. The published crate archive SHA-256 is
`5d10a236f96f87d70732e44520046785431ef01d5bcd6b041317bfadd2f88245`.
All twelve original archive members are retained; `Cargo.toml.upstream` preserves
the original normalized manifest. The original crate lockfile is inert archive
material. The product's root lockfile owns application resolution.

## Correction and regression

`Position::backward_to_word_start` now compares its `usize` character position
against widened word-start offsets. Narrowing an inclusive run endpoint of 256
to `u8` previously wrapped it to zero and could return the beginning of the
document instead of the last word. The binary search still selects the previous
word at an exact word start, retains existing cross-run fallback and allocates
no additional storage. This changes navigation, not text admission or limits.

The GPL-3.0-only `src/text/nevrotick_word_navigation_tests.rs` exercises the actual
consumer API with valid trees: endpoint 256, neighboring 255, exact word starts,
document start, a single word, word offset 255, UTF-16 surrogate pairs and a
second text run. Four independent expectations failed before the correction
while four neighboring controls passed. All 213 consumer library tests passed
afterward in ordinary and optimized Linux builds on pinned Rust 1.99.1.

Vendored dependencies are excluded from workspace-member test selection. The
consumer library regressions therefore require explicit package selection
(`cargo test --locked --offline -p accesskit_consumer --lib`) in the admitted
isolated host lane. The same package with `--release` checks optimized behavior.
The recorded repair checks used read-only source/vendor mounts, the configured
namespace/resource boundary and exact archive-verified dependency inputs.

These checks establish portable consumer behavior. They do not execute Slint
layout, Windows UIA/COM/Narrator or lift existing native/accessibility holds.
Remove this local patch only after a selected upstream release includes the
correction and these regressions pass against its actual consumer API.

## Licences

Original dual-licensed sources retain their Apache-2.0 OR MIT grant and notices;
the MIT option is selected for this integration. LICENSE-MIT and LICENSE-APACHE
are retained from the same immutable AccessKit revision, as in the existing
Windows derivative, because the published consumer archive omitted them.
The new original regression module is GPL-3.0-only; its licence text is retained
in LICENSE-GPL-3.0-only. The composite Cargo licence records these separate
components and does not relicense the upstream code.
