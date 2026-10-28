# Dependency workflow sources

Reviewed 28 October 2026. Source skill:
`/home/david_osipov/personal-website-templated-1/.agents/skills/environment/dependency-updates/SKILL.md`.
Retains scoped discovery/cohort/review/qualification and dirty-work custody.
Cargo metadata, lockfile/source/features/MSRV/targets and build scripts replace
npm/ncu, peer graphs and node_modules assumptions. Source read, never modified;
npm comparison helper not copied.

[canonical architecture](../../../docs/windows-time-architecture.md), sections
18.2–18.10, controls licensing, Serde/TLS/native decisions and admission. Skill
research neither installs, pins nor qualifies versions/optional tools.

| Primary source inspected | Applied finding |
| --- | --- |
| [Cargo update](https://doc.rust-lang.org/cargo/commands/cargo-update.html) | Package-scoped updates are conservative but can change required transitives; --precise selects a version/revision; inspect the actual lock delta and don't default to blanket update |
| [Cargo tree](https://doc.rust-lang.org/cargo/commands/cargo-tree.html) | Feature/reverse graphs explain feature activation and duplicates; graph display is not a guarantee of exact compiled/shipped bytes |
| [Cargo features](https://doc.rust-lang.org/stable/cargo/reference/features.html) | Additive feature unification means disabling one direct default does not prove the resolved feature is disabled |
| [Rust 2024 resolver](https://doc.rust-lang.org/stable/edition-guide/rust-2024/cargo-resolver.html) | Edition 2024 implies resolver 3 for a package; a virtual workspace needs an explicit resolver decision; fallback does not prove MSRV compilation |
| [Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html) | A shared lockfile and explicit member metadata inheritance have different meanings; inspect each supported member's actual declaration |
| [Cargo build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html) | Build scripts execute during compilation and may compile native components; preflight build/proc-macro behavior, not only runtime packages |
| [Cargo build](https://doc.rust-lang.org/cargo/commands/cargo-build.html) | --locked preserves resolution; --offline restricts Cargo retrieval. Neither confines arbitrary build-script behavior |

Also read the user's pinned [PdfCraft vendored-maintenance skill](https://github.com/DavidOsipov/pdfcraft/blob/c9a02b378271f92a73cb44be9fd9e4acc6921516/.agents/skills/QA/pdfcraft-vendored-dependency-maintenance/SKILL.md)
and [Rust workflow](https://github.com/DavidOsipov/pdfcraft/blob/c9a02b378271f92a73cb44be9fd9e4acc6921516/.agents/skills/QA/pdfcraft-rust-implementation/references/rust-workflow.md)
through the GitHub connector. Preserve each actual local patch's upstream identity,
diff/hash, regression and retirement condition; excluded vendored crates need
explicit test accounting. Published-base audits do not attest to local changes.
The full David Osipov MIT notice is retained in the dependency leaf.

Adaptation added, upgraded, downloaded/built no dependencies. Prospective commands
need selected-toolchain checks. Advisory freshness, license compatibility and
Windows runtime remain real-graph admission/qualification tasks.
