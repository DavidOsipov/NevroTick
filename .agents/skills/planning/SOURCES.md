# Planning adaptation provenance

Checked 28 October 2026. The canonical owner is `docs/windows-time-architecture.md`; external research supports choices without overriding it. The source skills were read from the website checkout's working tree. No claim is made that those bytes equal its historical pinned commit, and no source website files were modified.

| Website source under `.agents/skills/planning/` | NevroTick target | Retained and adapted |
| --- | --- | --- |
| `plan-code-change` | `plan-code-change` | Small-task routing, one plan, source custody, bounded investigation, phased integrated exits, existing authorization; Rust/Windows environments replace website gates. |
| `plan-architecture` | `plan-architecture` | Owner/consumer flow, authority, data contracts, failure/lifecycle and decision tradeoffs; Rust modules/enums/traits/FFI replace TypeScript/class-pattern catalogue. |
| `plan-independent-review` | `plan-independent-review` | Fresh source review, stage-scoped verdicts, independent oracles, material findings, explicit uncertainty. |
| `plan-threat-model` | `plan-threat-model` | Concrete attacker path, actual control owner, pinned ASVS applicability, valid/hostile checks and residual-risk owner. |

Useful source decisions consolidated into shared conditional references; website-only paths, TypeScript examples, web-security policy, source evidence archives, full design-pattern catalogue not copied. Plan files use existing location or `.agents/plans/` to preserve checksummed architecture bundle.

| Primary source inspected | Finding used in adaptation | Limit |
| --- | --- | --- |
| [Rustonomicon FFI](https://doc.rust-lang.org/nomicon/ffi.html) | Unsafe ABI/lifetime/thread requirements belong behind reviewed safe interfaces and owned cleanup. | Does not prove a new Windows adapter sound. |
| [Cargo build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html) | Build scripts execute on the host; host/target assumptions require inspection during cross-compilation. | No Cargo workspace/toolchain is selected by this pack. |
| [Windows required service privileges](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/ns-winsvc-service_required_privileges_infow) | Startup filtering and effective process tokens matter; service-sharing unions privileges and retains the documented exception. | Account labels/configuration alone do not prove isolation. |
| [Windows pipe access rights](https://learn.microsoft.com/en-us/windows/win32/ipc/named-pipe-security-and-access-rights) | Explicit DACLs and precise rights matter; generic write can include pipe-instance creation. | Requires native tests and separate operation/server authentication. |
| [RFC 8915](https://www.rfc-editor.org/rfc/rfc8915.html) | NTS combines TLS key establishment and authenticated NTP; wrong-date bootstrapping and delay attacks retain limits. | NevroTick's narrower recovery requirements remain canonical. |
| [ntpd-rs README](https://github.com/pendulum-project/ntpd-rs/blob/main/README.md) and [protocol crate entrypoint](https://github.com/pendulum-project/ntpd-rs/blob/main/ntp-proto/src/lib.rs) | A real Rust NTP/NTS project separates protocol/algorithm code; that entrypoint forbids unsafe and explicitly disclaims a stable public interface. | Reference evidence, not a selected dependency, licensed code import or Windows qualification. |

Local authority consulted: `docs/README.md`, architecture sections 3–8, 10–15, 18–24, `docs/asvs-5.0.0-applicability.md`, and the status of `docs/windows-time-update-assessment.md`. Standards attribution and pinned clause text stay with the existing profile; this pack does not reproduce the standard or claim conformance.

The user's prior Rust adaptation, [PdfCraft PR 2912](https://github.com/storytold/pdfcraft/pull/2912), was also consulted at pinned unmerged head `c9a02b378271f92a73cb44be9fd9e4acc6921516`: [architecture/data-flow guidance](https://github.com/DavidOsipov/pdfcraft/blob/c9a02b378271f92a73cb44be9fd9e4acc6921516/.agents/skills/QA/pdfcraft-architecture-data-flows/references/architecture.md). Its distinction between observed edges, tool candidates and inference, including target/features/generated code/trait dispatch, strengthens current-source tracing. PDF graph, renderer, save semantics, existing crate names and historical toolchain facts were not transferred. The proposal is provenance, not an adopted NevroTick requirement.
