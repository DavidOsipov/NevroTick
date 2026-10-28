# Independent compiler and Windows-hardening review

**Reviewed:** RWTC-DEV-HARDENING-001 revision 1.0, `deliverables/windows-time-development-hardening.md`, against RWTC-ARCH-001 revision 1.4, especially sections 18 and 20–23.  
**Reviewer:** independent compiler/configuration reviewer; did not author the target document and did not read the research-worker reports.  
**Date:** 28 October 2026.  
**Reviewed file SHA-256:** `3d0965b73aa20943d9df282df76f924b047c4e9f7178a757aa434d93da580227`.  
**Verdict:** **READY as a proposed configuration and qualification specification within this review scope.** No blocking compiler/configuration finding was substantiated. This does not qualify a development environment, product binary, Windows mitigation, or release.

## Scope and method

Read the target draft before any research report. Read the pinned website `AGENTS.md`, planning independent-review skill, testing contract, QA independent-review/static-check skills, security contract, candidate security amendments, and build/deployment security guidance. Applied their rules about independent expectations, frozen inputs, honest evidence, native effects, and preserving required gates. Website commands and candidate amendments were not adopted as Rust requirements.

Examined the candidate toolchain, workspace lint inheritance, lint levels and exceptions, release/custom profiles, target rustflags, environment/configuration override controls, Windows mitigation claims, and their relationship to the architecture's phase and release requirements. Used three primary-source lookup batches; used no Context7 calls.

Local checks used Python's existing `tomllib` only to parse the five fenced TOML snippets. All five parsed successfully, with root tables `toolchain`, `workspace`, `lints`, `profile`, and `target`, respectively. This establishes TOML syntax only, not acceptance or effectiveness under Cargo/Rust 1.99.1. No compiler, Cargo command, scanner, application build, installation, or Windows test was run. `git status` reported that the shared working directory is not a Git repository, so the content hash is the available document identity; no HEAD/branch identity is asserted.

## Preserved first-pass assessment

Before consulting upstream documentation, the draft appeared internally coherent. It already distinguished proposed controls from executed evidence; protected target flags from global replacement; host tools from target artifacts; `deny` from `forbid`; test-harness panic behavior from shipping processes; and PE metadata from runtime enforcement. No initial confirmed defect was found.

The first-pass questions were whether Cargo supports the lint/profile blueprint as described, whether the override priority was stated correctly, whether Rust 1.99.1 and its cited FFI correction were real, and whether the CFG/CET/linker claims overstated their effect. These questions were resolved by the primary sources below. They are retained as reviewed questions, not converted into invented findings.

## Findings and optional clarification

**Blocking findings: none.** No confirmed high-, medium-, or low-severity defect was found in the assigned scope.

### O-C01 — Optional informational clarification: the `exit` lint has a deliberate exception

- **Location:** section 5.1, Clippy policy, and section 5.2, fault behavior.
- **Severity/status:** informational; optional, not a blocker.
- **Observation:** the official `clippy::exit` entry says that it detects `std::process::exit` outside `main`; a call inside `main` is permitted. The draft does not promise a complete prohibition and already requires fault-policy review, so this is not an enforcement contradiction.
- **Concrete optional correction:** add one short statement that `clippy::exit` is only a backstop and that process/service entry-point termination remains part of the reviewed component fault policy. If an implementation chooses to forbid `exit` in `main` too, that stricter rule needs its own explicit checker and valid/invalid controls; do not imply that this lint supplies it.
- **Primary source:** [Official Clippy `exit` lint](https://clippy.rust-lang.org/stable/index.html#exit).

## Substantive checks and disposition

| Draft section | Review result | Primary basis or canonical requirement |
| --- | --- | --- |
| 4.1 — Rust 1.99.1 | The official release announcement is dated 15 October 2026 and describes the x86_64 FFI `bool` return miscompilation. Calling the version a qualification candidate is accurate. Its existence is not compatibility evidence for the future graph. | [Rust 1.99.1 announcement](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/) |
| 5.1 — lints and priorities | Cargo documents namespaced lint tables, lower numeric priorities preceding stronger overrides, and package-local applicability. Explicit member inheritance and independent scope checking are appropriate. The document does not claim this checks third-party code. | [Cargo manifest lint section](https://doc.rust-lang.org/cargo/reference/manifest.html#the-lints-section); architecture 18.1 |
| 5.1–5.3 — selected restrictions and unsafe exceptions | The checked `allow_attributes_without_reason`, `let_underscore_must_use`, and `exit` lints exist. Separating pure-crate `forbid(unsafe_code)` from a workspace `deny` that permits narrowly reviewed FFI exceptions avoids an impossible global policy. The draft does not mislabel Clippy as proof of panic freedom or safe FFI. | [Official Clippy catalog](https://clippy.rust-lang.org/stable/index.html); architecture 18.1 |
| 6.1 — profile semantics | `debug = "limited"`, `strip = "none"`, overflow checking, and inherited custom profiles are consistent with the documented settings. Cargo cannot set `panic` in profile package overrides. Test/build-script/proc-macro behavior does not demonstrate the shipping panic strategy. The draft accounts for these limits. | [Cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html); architecture 18.1–18.2 |
| 6.2 — environment and configuration bypass | Cargo selects the first applicable extra-rustflags source: encoded environment flags, ordinary environment flags, joined matching target entries, then build flags. Explicit target selection separates host tools. The draft describes this correctly and covers profile, wrapper, toolchain, linker, rustdoc, ancestor/home/legacy and command-line overrides. | [Cargo configuration](https://doc.rust-lang.org/cargo/reference/config.html#buildrustflags) |
| 6.2–6.3 — CFG | Stable `-C control-flow-guard=yes` and native compiler/linker controls are documented. Rust's `nochecks` emits metadata without enforcement checks; prohibiting it is sound. Microsoft's documentation explicitly permits mixing protected and unprotected objects, so the draft correctly refuses to infer complete protection from a flag alone. | [rustc codegen options](https://doc.rust-lang.org/rustc/codegen-options/index.html#control-flow-guard); [MSVC CFG](https://learn.microsoft.com/en-us/cpp/build/reference/guard-enable-control-flow-guard?view=msvc-170) |
| 6.2 — linker warnings | MSVC's linker `/WX` makes linker warnings fail output generation. The draft appropriately requires qualification of replacement linkers and effective flags. | [MSVC linker `/WX`](https://learn.microsoft.com/en-us/cpp/build/reference/wx-treat-linker-warnings-as-errors?view=msvc-170) |
| 6.3 — CET and Rust stack protection | `/CETCOMPAT` is an x64 compatibility marker, not evidence of enabled shadow-stack enforcement. The Rust stack-protector documentation identifies the feature as likely internal. Keeping these separate from an already-qualified stable release configuration is appropriately cautious. | [MSVC `/CETCOMPAT`](https://learn.microsoft.com/en-us/cpp/build/reference/cetcompat?view=msvc-170); [Rust stack protector](https://doc.rust-lang.org/unstable-book/compiler-flags/stack-protector.html) |
| 6.3, 10–12 — actual effect evidence | The specification requires effective inputs, emitted PE images, runtime policy, controlled indirect-call/DLL fixtures and native protected effects. It explicitly leaves all executions pending. No important missing effect proof is being claimed as completed in this scope. | Architecture CHECK-16, CHECK-18, CHECK-19, CHECK-23, CHECK-24 and CHECK-27; phases P0/P6/P7 |
| 2 and 12 — scope and stage coherence | The initial x64 MSVC target, one complete MSI, GPL-2.0-only derivation, held AWS-LC/ring graph, independent signing acceptance and unadopted TUF decision are preserved. Stage-specific readiness is distinguished from release readiness. | Architecture 18, 20–23 |

## Required implementation evidence remains pending

The absence of a document finding does not waive DEV-01/DEV-02/DEV-05 or the architecture's release gates. A future implementation must still demonstrate the exact compiler/SDK/linker and dependency identities; scope/lint negative and positive controls; rejected environment/profile/linker overrides; real panic and callback behavior; actual native object coverage; emitted PE characteristics; runtime mitigation policies; and valid/invalid protected-effect and complete-MSI behavior. Those are appropriately identified as future qualification work in the reviewed draft.

The next permitted action is for the coordinator to reconcile this report with the other independent reviews, preserve the document's evidence qualifications, and optionally apply O-C01. No target deliverable was edited by this reviewer.

## Post-write source-identity check

A final identity check detected a coordinator-side draft change: the current file hash became `763286b2bfa4c19271314039de187710f9667e4bedaf2bbd36ded591328078b5`. The reviewer immediately informed the coordinator and re-read sections 4–6; the compiler, lint, profile and mitigation text in that scope remained unchanged. The original verdict above remains bound to the original recorded input. The coordinator must identify/reconcile the remaining changed sections before applying that verdict to the entire new document; this record does not silently carry approval across an unidentified delta.

## Final bounded correction recheck

The coordinator subsequently identified the complete correction scope: section 3.3 adds hosted-Windows privilege/isolation and separate Windows 11 laboratory qualifications; section 8.3 adds a protected `always()`/`needs` final gate and explicit job/shard success requirements; section 9.4 restores the architecture's exact obligation for every surviving mutant; section 11.2 adds controls for hosted isolation and the final CI status; sources SC31/SC32 are appended; and section 5.2 adds O-C01's lint limitation. No profile, toolchain or TOML blueprint was changed.

The reviewer directly re-read those current sections and the appended source entries. The section 9.4 wording now expressly requires stronger tests or independently justified equivalence/exclusion, matching canonical architecture section 18.4. The isolation/final-gate additions preserve the proposed-control versus executed-evidence distinction and create no conflict with the reviewed compiler/override policy. Their provider-specific facts and full CI semantics remain within the coordinator/CI review scope; this addendum is not a second implementation or live-provider qualification claim.

The exact section 5.2 sentence observed was:

> `clippy::exit` also permits calls from `main`; review entry-point shutdown explicitly rather than treating that lint as a universal process-exit prohibition. Qualify the intended fatal-path behavior with its own control. [C07]

That wording correctly reflects the official lint behavior checked during the initial review. **O-C01 is applied and closed.** No new compiler/configuration finding arose from the bounded changes, and no additional research or target execution was needed.

**Final reviewed SHA-256:** `fdc0e88f04d31ebca05c739f79155fd14216c28733b1876ff99c78526a54c904`.  
**Final scoped verdict:** **READY as the proposed compiler/configuration/Windows-hardening qualification specification at this hash; no open blocking or optional finding in this review scope.** All actual tool admission, compile/lint controls, Windows mitigation/effect evidence and release gates remain pending as stated in the draft. This final addendum reconciles the earlier identity warning while preserving the original review record.
