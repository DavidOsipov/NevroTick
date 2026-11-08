# Development-environment hardening for the Rust Windows time client

**Owner clarification, 6 November 2026:** OD-07's required Windows CPU baseline
is now `x86-64-v3` in the target configuration and exact admission controls.
OD-05 requires every applicable final-product protection at the strongest
supported compatible setting. Sections 6.2–6.4 select the concrete stable
Rust/bundled-LLD vector and retain mandatory image, runtime and package
qualification. Missing implementation or evidence is open work, not an optional
control. The [existing build task](../tasks/build-optimization.md#required-v3-and-maximum-compatible-hardening--6-november-2026)
owns this change; native/fuzz/storage/UI holds and the post-V1 formal-proof
deferral remain.

**Selected-MSIX amendment, 3 November 2026, with 8 November signing clarification:** [OD-05](nevrotick-owner-decisions.md#7-od-05--signing-and-release-ownership) selects one complete hardened MSIX for David's first Store attempt with services. The [MSIX plan](plans/windows-msix-store.md) owns package integrity, Store-compatible CIG, role profiles and independent controls. [Architecture §19.7](windows-time-architecture.md#197-signatures-for-the-complete-shipped-inventory) additionally requires the signed package and every supported individual file signature, including nonbinary files. Qualify actual Store preservation; missing signing capability under the declared admitted profile blocks release, not source preparation. SignPath may supply inner signatures if qualified; MSI delivery needs a later decision. Current Core priority, Windows/fuzz/storage/UI holds remain; no production implementation, signing or native result is claimed.

**Historical owner amendment, 29 October 2026:** [NV-DEC-001](nevrotick-owner-decisions.md) confirms source-derived reuse with independent verification. Its original unsigned-MSI/OpenPGP permission was superseded by OD-05 §7.7 on 8 November; detached signatures cannot substitute for required native signatures. All other applicable compiler, lint, audit, evidence and hardening requirements remain. No application test is claimed by this documentation.

**Document:** RWTC-DEV-HARDENING-001, revision 1.0  
**Research date:** 28 October 2026  
**Baseline:** Product architecture RWTC-ARCH-001 revision 1.4  
**Status:** Enforcement specification and qualification plan with partially adopted development tooling. The original research created no application or execution evidence. The current [tooling record](../TOOLING.md) distinguishes installed tools and bootstrap controls from pending application, native-product and release qualification.

**Documentation amendments:** 29 October 2026 - explicit upstream-code verification and stress-testing policy; owner-directed garde adoption and mandatory function/module contract discipline; adopted native memory diagnostics and agent workflow; selected Windows x64 optimization settings with performance and native qualification still pending. Application gates remain pending implementation and qualification.

**Scoped amendment, 3 November 2026:** the [second protected-storage source review](plans/windows-protected-storage-upstream-lessons-2.md)
adds concrete adapter, durability and mitigation cases to section 6.4. The
[storage task](../tasks/windows-protected-storage.md) records follow-up and actual
evidence. Source definitions remain separate from execution; storage experiments
are paused and no new tool, dependency or required mitigation is admitted.

The [Windows integration review](plans/windows11-integration.md) adds the bounded
qualification refinements in section 9.11. They retain existing CHECK owners and
separate configured state, effective enforcement and measured behavior; optional
API/tool candidates are not admitted by this documentation.

The [Microsoft Learn MCP follow-up](plans/windows11-integration-follow-up.md)
adds API-specific lifetime, publication, permission and evidence cases to those
same owners. Its source findings are not executed native results. The current
Windows execution stop, separate fuzz no-retry hold and storage pause remain
governed by the [development plan](plans/development-plan.md#7-next-executable-task-for-a-development-agent).

## 1. Purpose and recommended baseline

The development environment should make unsafe changes difficult to introduce, policy bypasses visible, and missing evidence impossible to report as a pass. The strongest useful setup combines a pinned compiler, strict coherent lint rules, dependency admission, isolated execution, independent behavioral tests, protected CI and verification of the actual Windows artifacts.

There is no defensible percentage by which this toolset will reduce bugs before we have measured the project. Rust, linting and scanners cannot establish correct clock control, source selection, NTS authentication, IPC authorization or Windows privilege boundaries by themselves. Those need explicit contracts and independently observed behavior.

### Adopt and qualify first

| Layer | Proposed required baseline | What it contributes |
| --- | --- | --- |
| Compiler | Exact stable Rust/MSVC toolchain, checked release arithmetic, explicit fault policy | Reproducible diagnostics and safer failure behavior |
| Source policy | rustfmt; compiler warnings; Clippy `all`, `pedantic` and selected restrictions; rustdoc; unsafe boundaries | Catches common mistakes and makes exceptions reviewable |
| Function/module contracts | Precise typed APIs; private fallible construction invoking shared garde rules; checked postconditions and current-state guards | Makes omitted field rules visible and prevents ordinary callers from constructing invalid domain values through public APIs |
| Dependency policy | Exact lock/feature graph; cargo-deny; native/build-script/proc-macro inventory | Known advisories, sources, licenses and unwanted capability checks |
| CI policy | actionlint, zizmor, secret scanning; protected policy and immutable action identities | Detects workflow errors and reduces credential/artifact substitution paths |
| Static analysis | Qualified CodeQL Rust analysis with documented gaps | Adds data-flow checks beyond the compiler and Clippy |
| Behavioral evidence | nextest, separate doctests, independent examples, proptest, bounded fuzzing | Checks real requirements and rare boundary behavior |
| Native memory and resources | Application Verifier Basics, full PageHeap and CDB on affected Windows paths; separate lifecycle retention measurements | Detects reached native heap/API misuse and challenges resource ownership; neither installed tools nor crash-free runs establish absence of leaks |
| Test sensitivity | At least 90% executable-line coverage per security module; at least 95% mutation score per critical source file | Preserves the accepted architecture's thresholds and exposes weak tests |
| Windows qualification | Native x64 MSVC execution, PE inspection, runtime mitigation queries, privileged-effect and installer tests | Establishes behavior that portable Rust tests cannot prove |
| Release | Complete artifact inventory/SBOM, independent exact-candidate review, owner-controlled submission/publication | Connects verified source and tests to the actual Store-delivered complete MSIX; any later selected MSI alternative needs its own evidence |

Advanced tools are admitted for a named invariant: Miri for suitable Rust memory/aliasing behavior, Windows ASan for a qualified native surface, Loom for a small concurrency protocol, and Kani for a bounded arithmetic/admission property. Requiring all of them everywhere would add unsupported configurations and false assurance.

The compiler, analyzers and test tools are development dependencies and should not enlarge the installed service/UI payload. Runtime contracts use bounded Serde decoding, mandatory shared garde validation inside private fallible construction, and effect-time authorization under architecture section 6. Garde is a selected runtime dependency whose exact product graph still needs admission; it does not replace the decoder or trusted-state guards.

## 2. Authority, scope and unchanged decisions

The product is a Chrony-derived Rust Windows 11 client with a network engine, a narrow privileged clock broker, an ordinary-user UI and an administrative helper. The selected delivery direction is **one complete hardened MSIX**, with Microsoft Store as the first submission attempt; required services and payloads stay inside the complete product. The complete MSI design is retained only for a later owner decision. The initial native target is `x86_64-pc-windows-msvc`. ARM64 needs its own admitted toolchain, runtime and packaging evidence.

The architecture remains authoritative for product behavior and unresolved choices. In particular:

- Preserve selected Chrony provenance and the GPL-2.0-only compatibility review. The held AWS-LC/ring production graphs remain held pending authoritative review of the exact distribution.
- Preserve separate network and clock authority, bounded correction/bootstrap policies, real Windows token/pipe checks and complete installer lifecycle requirements.
- Do not adopt TUF, an automatic privileged updater, a different crypto provider, a package split or an ASVS exception through a development-tool change.
- Treat all existing CHECK-1 through CHECK-27 requirements as planned until their actual evidence exists. A configured scanner does not complete ASVS Level 3 verification.

The pinned website guidance at commit `6530d3a8d44cbe4f829b1e09edd3a1c90cffc087` informed the agent, security, testing and independent-review rules. Its TypeScript/Astro/npm commands are not Rust requirements. The security-audit guidance was used in design/guidance mode; this is not a completed codebase security audit.

**Policy language:** “required” describes the expected behavior of a gate or workflow, not proof that it exists or has passed. Explicitly adopted workflows apply within their documented scope; proposed gate implementations still need admission. Existing architecture requirements are preserved; new quantitative choices, such as the advisory refresh interval, are labelled as proposals. Use the tooling record and actual configuration to distinguish current enforcement from pending application evidence.

### 2.1 Restricted first UI milestone applicability

[Architecture section 16.5](windows-time-architecture.md#165-restricted-first-ui-milestone-ui-m0) incorporates the owner's NV-UI-M0-001 scope decision. [OD-13](nevrotick-owner-decisions.md#17-od-13--slint-as-the-default-ui-toolkit) selects complete replacement of the Win32 UI with Slint; the old Win32 adapter and observations remain historical baseline evidence. The current Windows entry selects Slint/Winit/software against v0.4, while native Windows qualification remains open. UI-M0 retains its unelevated, fixture-only scope with deterministic checked-in sample data, Status/Sources/Settings, source details and an ephemeral source-edit preview. Keep the sample-data label visible; local validation, **Update preview** and Cancel affect only in-memory state discarded on exit. Sync now is disabled with an explanation; protected actions are disabled or omitted. No live status, production IPC, NTP/NTS/DNS traffic, service operations, policy persistence, UAC, Apply or clock effects are permitted in this UI slice. The toolkit decision grants no dependency, native, packaging or release qualification and lifts no execution hold.

D1 (theme/contrast/text/DPI qualification), D2 (production freshness/reconnect, policy drafts/revisions/apply and administrative results) and D3 (full keyboard/focus, Narrator/UIA qualification) are **Deferred**, not Passed, Removed or N/A. Agents must not implement or qualify them or block UI-M0 on them. Retain toolkit-provided basic keyboard/editing behavior and meaningful labels in one documented reference display configuration; Slint widgets do not inherit Win32 child-control accessibility. Carry the canonical deferral ledger and triggers into the backlog/next-stage record: restore the relevant D2 rules before live status or real policy/protected commands, and D1/D3 before general-user preview/release or earlier support/qualification claims.

Retain applicable compiler/format/lint/dependency/review policies on the real UI crate, narrow documented unsafe FFI, correct native ownership/string/callback/error lifetimes and teardown, and the admitted toolchain/linker direction. Preserve exact Slint version/features/backend, generated-code safety/lint boundaries, licence and runtime/build/transitive graph admission under the accepted separately GPL-3.0-only GUI direction; Core/engine stay GPL-2.0-only and the toolkit choice is settled. One owner controls shared schemas/Cargo graph, Slint event loop/view/callback state, weak references and Windows OS adapters; serialize shared desktop interaction and build outputs. M0-01–08 require small editor behavioral checks, meaningful native window/mouse-flow observations at the reference configuration and independent restricted-scope review; cross-compilation alone does not complete them. Do not force broad fuzz/stress/soak, production security coverage/mutation, service, accessibility, signing or package qualification into this fixture-only milestone. Those requirements retain their later applicability. Other workstreams remain independent; UI-M0 completes neither P0 toolkit qualification, CHECK-22 nor release gates.

The [current GUI handoff](../tasks/slint-gui-handoff.md) and
[S0.5 source/evidence record](../tasks/s0.5-ui-comparison.md#current-nativetext2-integration--gui30-and-later)
own the later complete-v0.4 fixture additions, including Simple/Power views,
bounded local policy/recovery previews and opt-in tray/sample-notification source.
Portable source/rendering checks do not establish actual Windows behavior or
re-enter D1–D3. Preserve weak callback/native lifetime ownership and the distinction
between original M0 receipts, current fixture evidence and later production gates.

## 3. Development threat model and execution boundaries

### 3.1 Assets, actors and failure paths

Protect source and requirement integrity, maintainer credentials, dependency provenance, release artifacts, signing authority, test evidence and the clocks/services of development machines. Consider both malicious inputs and an agent making a plausible but incorrect change.

**Contributor catalog clarification, 6 November 2026:** every JSON/CBOR input,
including English, generated output and signed/package-delivered data, remains
untrusted at admission. Use the [localization threat controls](plans/ui-localization.md)
for raw-file custody, strict bounded decoding/formatting, literal sinks and
semantic review. Translation-only checks execute an independently admitted
validator/typed contract; candidate changes cannot redefine their own authority.
Keep candidate-code tests in a separately constrained lane. Use short-lived
credential-free offline conversion with enforced resource/filesystem/process
bounds, task-owned outputs and atomic successful publication. Do not execute PR
artifacts or promote their executable caches into release. Runtime independently
checks final catalog bytes; package signing supplies no parser or wording proof.
These are required design controls; this requirement text supplies no runtime or
qualification evidence. The [current GUI handoff](../tasks/slint-gui-handoff.md)
and S0.5 record own implemented catalog loaders, repertoire guards and hostile-
payload checks with their exact scope. Production semantic approval remains
separate; source checks establish no complete CI isolation or native Windows pass.

| Threat | Concrete failure path | Required control and remaining limit |
| --- | --- | --- |
| Untrusted contribution weakens its own check | PR edits workflow, lint allowlist or coverage manifest, then receives a green status | Independently protected policy review and required checks on the exact candidate; repository settings must be evidenced separately |
| Dependency executes during analysis | `build.rs`, a macro or generator reads credentials or downloads code | Credential-free OS-isolated execution, denied egress after acquisition, bounded filesystem/process access; Cargo flags alone are insufficient |
| Hardening flags disappear | Agent adds global `RUSTFLAGS` for warnings and replaces target CFG/link flags | Admit effective environment/configuration, inspect actual commands and final PE/runtime state |
| Dependency graph changes covertly | Feature unification activates a held crypto backend or server capability | Exact package/target/feature graph review and a failing forbidden-path fixture |
| Evidence is stale or incomplete | Wrong revision, omitted module, zero tests, missing shard or report-only scanner is called a pass | Identity-bound evidence, explicit expected scope, verified exit propagation and bad/good gate controls |
| Cache or artifact poisoning | Untrusted job populates an executable cache or substitutes a similarly named release package | Separate trust domains; exact hashes, origin and artifact manifests; no executable promotion from untrusted output |
| Test harness masks a product bug | Production computes expected output; tests use a repaired mock; mutant build failure counts as killed | Independent oracle, actual entry point/effect, raw outcome accounting and separate review |
| Agent damages the host | Clock/SCM/installer tests run elevated on a shared workstation | Disposable guest, one state owner, externally enforced bounds, recovery independent of guest time |
| Tool or metadata ages silently | Frozen compiler is vulnerable or advisory DB is stale | Reviewed tool updates and fresh security metadata alongside reproducibility snapshots |
| Authorized release authority is compromised | Correct-looking release passes a stolen approver/provider identity | MFA, narrow roles, independent approval, origin enforcement and rehearsed revocation; this residual risk is not eliminated |

### 3.2 Proposed build and release trust flow

```mermaid
flowchart TD
    A["Candidate source and dependencies"] --> B["Isolated checks without release credentials"]
    B --> C["Identity-bound evidence"]
    A --> D["Independent source and policy review"]
    C --> D
    D --> E["Protected commit"]
    E --> F["Admitted release build"]
    F --> G["Identified complete MSIX candidate"]
    G --> H["Local candidate and Windows qualification"]
    H --> I["Owner's controlled Store submission"]
    I --> J["Store certification and signing"]
    J --> K["Actual Store artifact and Windows qualification"]
    K --> L["Owner-approved broad release"]
```

This is the proposed flow for the selected complete MSIX. Source/design preparation precedes the depicted candidate stages and does not require speculative Store preapproval; local native qualification remains subject to the current Windows stop. David submits the candidate and controls its audience. Store certification/signing is an external outcome, and final Store-artifact evidence is produced after initial submission, before broad release. A refusal returns to the owner decision, without automatic package splitting, weaker protections or MSI fallback. See the [MSIX plan's sequence](plans/windows-msix-store.md#7-bounded-continuation-and-existing-acceptance-owners).

Keep source/build, locally tested and Store-delivered identities separately. Rebuilding or transformation after testing requires demonstrated equivalence or repeated relevant validation; final individual-signature, catalog/package and installed-file checks use actual delivered bytes. Follow [signing guide §4.1](windows-time-signing-guide.md#41-required-signatures-for-the-shipped-inventory): finish metadata, sign/verify inner files, then containers, and verify required signatures again after Store processing. A signing/submission stage must never run arbitrary build scripts with its credential. If SignPath is pursued for required signatures, its hosted-runner and approved artifact-lineage rules apply to that work regardless of the selected package format.

### 3.3 Agent workstation and sandbox rules

Run ordinary agents and editors without administrator rights. Keep the supported Windows host patched and retain Defender and the chosen Windows application-control/exploit protections. Do not introduce broad antivirus exclusions or disable security features to improve build times. Measure performance with the intended protections enabled.

Treat opening a repository in an IDE as an execution decision: build-script/proc-macro support, workspace tasks and extensions can invoke tools. Start with untrusted-workspace execution disabled, review task and extension configuration, and enable needed analysis inside the assigned boundary. Pin the admitted rust-analyzer/editor extensions separately where used; editor convenience checks are not authoritative CI evidence.

The execution boundary must provide an allowlisted environment, unprivileged identity, task-owned writable paths, restricted network, no host credential mounts or shared privileged sockets, and CPU/RAM/process/disk/wall-time limits. Tool caches should be immutable or separately owned where feasible. An ordinary folder, Git worktree, `cargo --offline`, or a container with a privileged host socket does not meet that boundary by itself.

**Hosted does not mean unprivileged.** GitHub-hosted Windows VMs run as administrators with UAC disabled. Their x64 Server build images do not by themselves qualify either intended [client/Server product profile](../tasks/windows11-integration.md#432-two-family-windows-qualification-matrix--5-november-2026), and supply no Windows 11 client result. Qualify an actual reduced-token/execution boundary and its filesystem/network controls, or resolve a provider-approved alternative; the runner label alone cannot satisfy this requirement. Keep each claimed OS profile's user/elevation/security behavior in its separate supported-OS laboratory. [SC32]

No signing/publishing token, cloud credential, privileged OIDC grant, SSH agent, production trust-store access or unrelated checkout belongs in a build/analyzer sandbox. If a hosted integration requires a status-upload token, confine it to a separate minimal data-only upload stage. It must not execute scripts from the report artifact.

Use separate worktrees/output directories for parallel agents and one owner for `Cargo.lock`, shared contracts, tool policy and native lab state. A worktree does not isolate service names, registry keys, the VM clock or machine-wide settings. Do not schedule concurrent mutable-state experiments on the same guest.

### 3.4 Mandatory contracts for agent-written code

Apply the [hostile-input assumption](windows-time-architecture.md#61-validation-architecture-and-enforcement-order) to user-entered product data and external programmatic data alike. Identify each implemented admission path and its bounds; UI controls, local/admin origin, authenticated channels and prior acceptance do not establish payload safety. Require real-handler adversarial checks, including UI bypass, well-formed unauthorized/stale requests and aggregate resource pressure, with accepted neighboring controls and no partial protected effects on rejection.

Apply [architecture section 6.3.1](windows-time-architecture.md#631-contracts-for-every-function-and-module) to every function and module, including private helpers, trait implementations and callbacks. Encode preconditions and output invariants in types wherever possible. Document the remaining semantic units, ranges, resource/lifecycle obligations, permitted effects, failures and postconditions at their owner. An unchanged immutable validated type carries its invariant across a call; a transformation, mutation, deserialization or changed trusted context requires the corresponding check. Do not substitute repeated serialization or decorative validation annotations for this analysis.

DTO/settings validators use garde, called by the private fallible constructor before a domain value can be returned. UI feedback and service admission share the same pure rule definitions; the receiver independently decodes and constructs its input. No `allow_unvalidated`; each field needs actual validation/recursive validation or a `skip` with a nearby invariant-owner explanation and independent review. Metadata-only annotations do not count as validation. Inspect all constructors, `Deserialize` implementations, builders, defaults, mutable accessors and interior-mutable handles for bypasses. Keep checks active in release builds; `debug_assert!` and test-only validation do not enforce production contracts.

For each changed contract, identify its enforcement point and independent observation before implementation. Choose the applicable checks: runtime valid/invalid neighbors through the real constructor/consumer; external consumer compile-fail tests for forbidden construction/mutation with a passing supported-use control; checked-transform/postcondition and failure-atomicity cases; property tests for combinations; bounded Kani proofs for selected finite invariants. Compile-fail must arise from the intended restriction, not a broken fixture/import or unsupported toolchain. Trace source-to-effect bypasses with the admitted CodeQL workflow when cross-function reachability is uncertain; scanner success alone does not prove contract completeness.

Reviewers must reject a path that bypasses required validation, consumes the unchecked original instead of the checked result, silently drops a postcondition, or treats a stale authorization marker as current permission. Missing applicable evidence remains incomplete; agents cannot clear it by removing rules, weakening a constructor, broadening a skip or shrinking the selected test scope. Retain the actual small change scope: a trivial total helper need not acquire a new report or validator when its type and implementation already establish the complete contract.

The selected initial declaration is `garde = { version = "=0.23.0", default-features = false, features = ["derive"] }`; section 18.5 of the architecture owns dependency admission. The current [workspace](../Cargo.toml) and [lockfile](../Cargo.lock) exist; garde is declared by the UI, time-domain and time-core manifests. The [implementation summary](windows-time-architecture.md#1-current-state-capsule) routes their bounded evidence to the existing tasks. Those results do not establish every planned constructor/consumer contract, enforced product CI or completed product qualification. UI-M0's D1/D2/D3 deferrals remain in force; validate its actual fixture-only paths without introducing deferred production integrations.

## 4. Pinning and admitting the toolchain

### 4.1 Initial stable candidate

Start qualification with **Rust 1.99.1**. The official 15 October 2026 release fixes an x86_64 FFI `bool` return miscompilation, directly relevant to native boundary work. This is a researched candidate, not evidence that our future dependency graph and Windows application already build correctly under it. [C01]

Candidate `rust-toolchain.toml`:

```toml
[toolchain]
channel = "1.99.1"
profile = "minimal"
components = ["clippy", "rustfmt"]
targets = ["x86_64-pc-windows-msvc"]
```

Use edition 2024 after compatibility qualification. Record the package MSRV explicitly and test it separately if it differs from the release compiler. A pinned version is a reproducibility input, not permission to ignore future security or correctness fixes.

Record the actual compiler commit/LLVM, Cargo, Clippy, rustfmt, rustup, native target, Windows SDK, linker path/version, CRT/native libraries and runner image. Rustup overrides can supersede the repository file; inspect the effective toolchain, not just its text. Only one admitted toolchain file/configuration may govern the build. [C03]

Nightly analysis has a separate exact dated pin, compatible components, target directory and artifact label. Never use `RUSTC_BOOTSTRAP` to make a production build accept unstable options. Nightly instrumented artifacts must not be promoted as the stable release binary.

### 4.2 Tool admission record

Every executable tool needs an entry in a reviewed tool manifest:

| Field | Required evidence |
| --- | --- |
| Identity | Exact version/commit, upstream origin and downloaded artifact digest/signature |
| Compatibility | Host, target, Rust/SDK/runtime dependencies and known unsupported modes |
| Behavior | Effective configuration, rule/query set, input scope and whether it executes target-controlled code |
| Isolation | Filesystem/network/credential boundary and resource limits |
| Gate semantics | Expected exits/reports, failure propagation, empty-scope handling and positive/negative fixture |
| Maintenance | Owner, update schedule, relevant advisories and requalification triggers |

Do not insert invented action SHAs, SDK versions or tool checksums into a “ready” workflow. Their exact values are admission outputs. Existing architecture versions for cargo-deny/cargo-auditable remain candidates until qualified; a floating `latest` or an unverified install script is not a replacement.

Tool upgrades should be reviewed proposals: inspect release notes and relevant advisories, update pins, run affected tool controls and the necessary integrated checks. Group versions only when compatibility requires it. Keep evidence of the prior admitted configuration; rollback to a known vulnerable version needs its own decision.

## 5. Compiler, Clippy and API policy

### 5.1 Workspace policy with explicit inheritance

Cargo workspace lints require every member to opt in. Root declarations do not automatically govern all members or external dependencies. Broad lint groups should have priority `-1` and individual rules priority `0`, allowing explicit resolution of group membership. Check actual first-party members, targets, generated code and build scripts. [C04, C05]

Candidate root `Cargo.toml` fragment:

```toml
[workspace.lints.rust]
warnings = { level = "deny", priority = -1 }
unsafe_code = "deny"
unsafe_op_in_unsafe_fn = "deny"
unused_unsafe = "deny"
unused_must_use = "deny"
let_underscore_drop = "deny"
unused_macro_rules = "deny"
unused_lifetimes = "deny"
unfulfilled_lint_expectations = "deny"
unknown_lints = "deny"
renamed_and_removed_lints = "deny"
improper_ctypes = "deny"
improper_ctypes_definitions = "deny"
unexpected_cfgs = "deny"

[workspace.lints.clippy]
all = { level = "deny", priority = -1 }
pedantic = { level = "deny", priority = -1 }
undocumented_unsafe_blocks = "deny"
allow_attributes_without_reason = "deny"
allow_attributes = "deny"
multiple_unsafe_ops_per_block = "deny"
unused_result_ok = "deny"
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
todo = "deny"
unimplemented = "deny"
unreachable = "deny"
dbg_macro = "deny"
let_underscore_must_use = "deny"
string_slice = "deny"
as_conversions = "deny"
mem_forget = "deny"
exit = "deny"

[workspace.lints.rustdoc]
broken_intra_doc_links = "deny"
private_intra_doc_links = "deny"
```

Every member must contain:

```toml
[lints]
workspace = true
```

The repository checker must reject missing inheritance and prove that the intended target actually runs through the lint gate. A changed manifest must not quietly exclude the broker or a generated first-party module.

Clippy explicitly discourages blanket enabling `restriction` and `nursery`; choose the relevant restrictions individually. The supplied extra rules address unchecked conversions, ignored results, panic placeholders, unsafe documentation and hidden process/resource escapes. Their exact diagnostics and exceptions still need qualification against the pinned toolchain. [C02, C07]

The repository's [adoptable policy fragment](../policy/rust-workspace.toml) additionally rejects permanent local `allow` attributes, multiple unsafe operations in one block and discarded `Result::ok()` values. Its isolated pinned-tool controls accept fulfilled reasoned expectations and consumed optional results, and reject each corresponding violation. These are bootstrap enforcement observations; application target/feature and native qualification remain pending. See [the current tooling record](../TOOLING.md).

The owner-selected four-rule cohort adds `let_underscore_drop`,
`unused_macro_rules`, `unused_lifetimes` and Clippy `string_slice` individually.
The three compiler lints detect a discarded destructor-bearing temporary,
unused arms of an otherwise used macro and unused lifetime parameters; the
Clippy restriction detects potentially panicking string slicing. They are
allow-by-default and require explicit selection, beyond warnings and
`all`/`pedantic`. [Rust compiler reference](https://doc.rust-lang.org/stable/rustc/lints/listing/allowed-by-default.html),
[Clippy reference](https://clippy.rust-lang.org/stable/index.html#string_slice).
Use named ownership or deliberate `drop` according to the intended lifetime,
exercise/remove unused macro arms, preserve necessary borrow relationships and
use checked string access. The current tooling record owns actual healthy and
rejection results. `panic_in_result_fn` remains deferred until intentional
assertions in Result-returning tests have an appropriate scope; this cohort
selects no blanket restriction group or panic-free guarantee.

### 5.2 Unsafe, conversions and error behavior

Use `#![forbid(unsafe_code)]` at the roots of the pure numerical core, protocol/domain and other unsafe-free crates. The Windows/native adapter may relax the workspace `deny` for the smallest justified module/item with an explicit safety contract and independent review. A global `forbid` cannot be relaxed for necessary FFI; that is why the workspace baseline uses `deny` and the pure crates add `forbid`.

Safe wrappers must establish ownership, layout, alignment, aliasing, lifetime, callback/thread rules, cleanup and error handling. `undocumented_unsafe_blocks` checks for documentation; it cannot verify the argument. The unsafe inventory includes dependencies and native inputs, not just first-party `unsafe` tokens.

Use private validated types and `TryFrom`/`TryInto` at trust boundaries. Reject nonfinite floats, invalid units and out-of-range fixed-point/integer conversions explicitly. Name intentional NTP modulo/era arithmetic and test its edge cases independently. Do not move a dangerous `as` cast into an opaque helper just to silence the lint.

Select `indexing_slicing`, `arithmetic_side_effects` and disallowed API/type lists for the critical modules where they provide useful signal. Direct clock writes, unsafe ownership transfer and raw authority-sensitive calls should have one reviewed adapter boundary. A lint cannot guarantee that aliases, macros or native functions never bypass it; native effect tests and source review remain required.

Do not force secret-bearing types to implement `Debug` to satisfy a blanket style rule. Review logs for secret material, unbounded attacker strings and unintended path/address disclosure. Never discard lock-poisoning errors or background-task failures merely to avoid `unwrap`.

Test code can need assertions and fixture extraction. Permit these only in genuinely test-only scope with a reason; prove the production counterpart still fails. An intentional fatal handler needs a reviewed fault/cleanup policy. Ordinary assertions, indexing, allocation and dependency code may still panic even when `clippy::panic` is denied; this is not a panic-free proof.

`clippy::exit` also permits calls from `main`; review entry-point shutdown explicitly rather than treating that lint as a universal process-exit prohibition. Qualify the intended fatal-path behavior with its own control. [C07]

### 5.3 Exceptions and documented interfaces

Prefer narrowly scoped `#[expect(..., reason = "...")]` where supported, with denied `unfulfilled_lint_expectations` so obsolete exceptions fail. `forbid` cannot be lowered. A broad `allow` added by an agent to make CI pass is a policy change requiring independent review, not an automatic repair. [C06]

Require public boundary documentation: accepted representations, error cases, units/ranges, ownership, side effects, authorization and cancellation. Rustdoc runs as its own checked lane; passing Clippy alone does not validate documentation. Generated platform declarations may need a scoped documentation exception; hand-written privileged APIs do not inherit that exception.

All exceptions need the exact rule/path, reason, independent evidence, owner, approver and a review/expiry trigger. Do not grant a permanent waiver to an entire crate because one FFI call or test fixture is legitimate.

## 6. Build profiles and Windows mitigations

### 6.1 Checked arithmetic and an explicit fault policy

Selected release/verification settings and the separately proposed service fault profile:

```toml
[profile.release]
# Optimization candidate; measurements and native qualification remain required.
opt-level = 3
lto = "thin"
codegen-units = 1
overflow-checks = true
debug-assertions = false
panic = "unwind"
incremental = false
debug = "limited"
strip = "none"

[profile.verify-release]
inherits = "release"
debug-assertions = true

[profile.service-abort]
inherits = "release"
panic = "abort"
```

The initial explicit unwind profile is for qualification while the component fault policy is resolved. `service-abort` is a **candidate**, not the selected shipping service profile. Abort avoids unwinding but skips destructors; safe shutdown, SCM recovery and persisted clock budgets must be qualified before choosing it. No restart may reset a security budget or resume inconsistent actuation.

Cargo's ordinary release defaults omit overflow checks. Enabling them makes integer overflow panic; it does not validate floating-point finiteness, units or time semantics. Runtime security conditions must use checked branches and typed errors before a panic can occur. `debug_assert!` remains a development invariant, never the only authorization, length or correction-limit check. [C08]

Cargo cannot set `panic` through a per-package profile override, and test harness behavior does not establish the production executable's panic strategy. Separate component profiles require named builds and artifact identities. Run the exact shipping executables through their fault cases. Rust unwinding must not cross Windows callbacks; `catch_unwind` neither catches abort/all native faults nor makes corrupted state safe to resume. [C08, C09]

Run optimized invariant checks and the actual shipping profile. The selected optimization candidate uses `opt-level=3`, thin LTO and one codegen unit; this selection establishes no measured improvement. Section 6.5 defines its trade-offs and qualification limits. Compare size, CPU, power, precision and regression trajectories before retaining a claimed gain or changing to `opt-level="s"`/`"z"`, fat LTO or another codegen-unit count. CRT and stripping choices remain separate. Retain matching debugging symbols with a source-path/privacy policy. Do not enable fast-math behavior that invalidates estimator assumptions.

For optimization tasks, use [the performance optimization skill](../.agents/skills/implementation/optimize-rust-performance/SKILL.md) with [resource QA](../.agents/skills/testing/qa-performance-resources/SKILL.md). Start from a measured cost and owning path, define comparable baseline/candidate identities and a decision rule, attribute the bottleneck, then make and verify a causal change. Preserve raw repeated samples, correctness/hostile-input controls and memory, tail-latency, wakeup and recovery trade-offs. Profiling locates a cause; representative runs establish a claimed improvement. Preserve mandatory validation, checked arithmetic, security settings and bounded native-work custody. Architecture section 17 owns budgets; CHECK-21 and UI-M0 retain their existing applicability. The [measurement reference](../.agents/skills/implementation/optimize-rust-performance/references/measurement.md) documents conditional tools and evidence limits; it installs no profiler or benchmark dependency.

### 6.2 Target-specific Windows compiler/linker settings

The following required target settings are selected in
[`.cargo/config.toml`](../.cargo/config.toml) and its exact admission controls.
Use the [selected LLVM+xwin route](../TOOLING.md#windows-development-route):
pinned Rust 1.99.1, bundled LLD 23.1.1 and admitted Microsoft CRT/SDK inputs,
retaining the MSVC ABI. The new CET switches are supported by that exact LLD
source; an arbitrary native `link.exe` is not an interchangeable admitted lane.
Record and verify the actual linker identity. Historical std-only PE/native
fixtures do not qualify this vector, current application binaries or runtime
enforcement. [C41]

```toml
[target.x86_64-pc-windows-msvc]
rustflags = [
  "-C", "target-cpu=x86-64-v3",
  "-C", "control-flow-guard=yes",
  "-C", "jump-tables=no",
  "-C", "link-arg=/GUARD:CF",
  "-C", "link-arg=/DYNAMICBASE",
  "-C", "link-arg=/HIGHENTROPYVA",
  "-C", "link-arg=/NXCOMPAT",
  "-C", "link-arg=/FIXED:NO",
  "-C", "link-arg=/LARGEADDRESSAWARE",
  "-C", "link-arg=/CETCOMPAT",
  "-C", "link-arg=/CETCOMPATSTRICT",
  "-C", "link-arg=/DEPENDENTLOADFLAG:0xA00",
  "-C", "link-arg=/WX",
]
```

`/WX` makes linker warnings fail, including warnings about ignored settings.
A different linker needs explicit admission and equivalent emitted-image
evidence; never drop a required switch or warning failure to accept it.
`/FIXED:NO` and `/LARGEADDRESSAWARE` explicitly preserve relocation/address-space
prerequisites already supplied by the pinned LLD's x64 defaults. They document
the ASLR contract rather than adding two independent mitigations. [C17, C41, C45]

**Critical bypass:** Cargo's flag sources are alternatives, not a simple append chain. `RUSTFLAGS="-D warnings"` or `CARGO_ENCODED_RUSTFLAGS` can replace the configured target flags and remove the intended hardening. With explicit `--target`, target flags also do not apply to host build scripts/proc macros. [C18]

The trusted build entry point must reject unreviewed environment/configuration overrides, wrappers and path substitutions. Admit `RUSTUP_TOOLCHAIN`, `RUSTFLAGS`, encoded flags, profile variables, `RUSTC_WRAPPER`, workspace wrappers, target linker settings, rustdoc flags and applicable Cargo configuration deliberately. Check ancestor/home/legacy Cargo config and command-line overrides. Record relevant effective values without dumping ambient secrets. Ordinary builds must reject lint-capping overrides; the bounded mutation instrument described later is a separately admitted lane.

### 6.3 What each protection actually proves

| Protection | Required handling | Evidence and limitation |
| --- | --- | --- |
| Control Flow Guard | Stable Rust `control-flow-guard=yes`, compatible native compilation/linking | Inspect load config and an admitted indirect-call fixture. `nochecks` is prohibited. A CFG image flag does not retrofit checks into prebuilt native objects or the supplied standard library. [C10, C11] |
| DEP | `/NXCOMPAT`; no product need for writable-executable generated code | Check every shipped image and applicable runtime policy. [C12] |
| ASLR | `/DYNAMICBASE`, `/HIGHENTROPYVA`, `/FIXED:NO`, `/LARGEADDRESSAWARE` | Inspect actual relocation/address properties and runtime policy; explicit defaults alone establish no effective randomization observation. [C13, C14, C41, C45] |
| CET shadow stack | Selected `/CETCOMPAT` **and** `/CETCOMPATSTRICT`; require strict enforcement on the supported CPU/OS/VM class | Inspect both extended image bits and effective HSP/strict/audit state. The flags mark compatibility; they do not instrument every module or prove enabled hardware enforcement. Strict mode and non-CET/EHCONT module restrictions are separate policies. [C15, C38, C41] |
| Jump-table suppression | Stable Rust `jump-tables=no` is selected | Disables generated jump tables in code compiled under the option; it does not eliminate all indirect branches, instrument prebuilt inputs or replace CFG. Inspect the emitted code and retain correctness/resource qualification; no measured security or performance gain is claimed. [C44] |
| Static-import search | Selected `/DEPENDENTLOADFLAG:0xA00`; require `0x800` for a role whose actual imports permit it | Inspect `DependentLoadFlags` and the complete legitimate/denied-load paths. This is static-import policy, not a universal DLL allowlist or a replacement for runtime loading controls. [C41, C42] |
| Native stack-buffer protection | Preserve/check native MSVC `/GS` in applicable native compilation | Passing a native option to a Rust link does not instrument Rust functions. [C16] |
| Native Spectre mitigation | Require applicable `/Qspectre` compilation and compatible libraries through the native owner | This native compiler mitigation has limited pattern coverage and cost; it is not a Rust linker switch. Record the actual threat, native coverage and remaining limitations. [C20] |

`0xA00` combines application-directory (`0x200`) and System32 (`0x800`)
search, preserving prospective protected app-local CRT/DLL imports. The final
CRT/import layout is still unqualified; data assets do not justify the extra
directory. Tighten each role to `0x800` when its actual dependency closure
permits System32-only directory search. Admit any necessary app-local code only
inside the sealed declared package/dependency graph. Package-graph resolution,
delayed/explicit loads and DLL subdirectories need their own observations.
Packaged-app standard search already excludes ordinary CWD/PATH fallbacks;
do not attribute that existing property solely to this flag. Pinned LLD requires
an available, sufficiently large load-config structure to write the field, so
successful flag parsing is not sufficient evidence. [C41, C42]

Keep unsupported or incompatible additions explicit rather than claiming that
every option name can be combined:

| Excluded from this stable vector | Concrete reason and remaining owner |
| --- | --- |
| `/GUARD:EHCONT` | Rust 1.99.1's explicit generation is the false-by-default unstable `-Z ehcont-guard`. A linker-only flag cannot supply missing Rust continuation metadata; an empty declared table differs from an absent legacy table. Require supported complete generation before selecting it; do not bypass stable-channel admission. [C43] |
| Rust stack protector / LLVM CFI | Unstable/internal interfaces and target/graph constraints require a separately supported compiler configuration. This is a recorded toolchain limitation, not claimed protection or permission to remove other controls. [C19, C21, C43] |
| `/GUARD:XFG`, `/SAFESEH` | The pinned LLD guard parser does not support XFG; SafeSEH is the x86 mechanism, not the x64 exception model. [C41, C46] |
| `/GUARD:LONGJMP` as an additional protection | Pinned LLD's `/GUARD:CF` already selects its long-jump handling. Verify actual emitted metadata instead of counting the duplicate switch. [C41] |
| `/CETIPVALIDATIONRELAXED`, `/CETDYNAMICAPISINPROC` | These advertise relaxed context-IP validation or an in-process dynamic-API allowance; they do not strengthen the selected profile. [C38, C41] |
| `/INTEGRITYCHECK` | The package/signing owner must resolve its supported per-image signature contract and enable it in the final profile where compatible. The selected MSIX catalog route and unsigned development fixtures do not establish that compatibility. Keep this gate open; package/CIG/trust qualification is not proof of this separate control. [C46] |

The v3 CPU floor does not imply CET hardware availability. Record the concrete
CPU/OS/VM applicability condition, and enforce the strongest supported CET
profile when that condition holds. An unavailable mechanism is not a successful
enforcement result; this selection introduces no separate CET hardware floor.

Verify three layers: **effective build inputs, emitted images and effective runtime policy**. Use supported Windows process-policy APIs and handle failures. UI and broker can need different settings; do not turn on every process policy without testing legitimate operation. DLL search/load restrictions, valid-load controls, unexpected DLL canaries and the architecture's no-sideloading checks remain required. [C22; architecture CHECK-16/18/23]

### 6.4 Per-process mitigation qualification

Use [architecture section 15.4](windows-time-architecture.md#154-per-process-mitigation-profiles-and-startup-admission) as the canonical role/activation/failure contract and manifest schema. This 29 October refinement incorporates useful specificity from the supplied PDFCraft proposal without adopting its CRT, dependencies or UI architecture. Existing tooling observations remain scoped to their recorded fixture. All application-level cases below are **planned / not executed**.

The same startup record owns [effective unattended error behavior](windows-time-architecture.md#1234-privacy-sinks-and-resource-limits),
separately from mitigation bits. W11-W05 establishes actual process/thread policy
and bounded critical-error progress; W11-W08/ST-W07 retain useful diagnostics,
collector privacy and administrator-policy boundaries. A source flag or no-UI
observation supplies no dump/upload or whole-profile qualification.

The **6 November owner clarification** makes the strongest applicable supported
compatible profile mandatory for each delivered role. An unimplemented setter,
missing observation or failed qualification remains open required work, not an
optional control. The selected build vector installs no production per-role
mitigation initializer or effective-state observer; their implementation and
native acceptance remain open. Use the existing platform adapter and mitigation
manifest rather than adding a second policy owner. The [MSIX plan sections
3.1–3.4](plans/windows-msix-store.md#3-package-trust-and-the-privileged-helper)
own sealed executable content, explicit `uap10:Content Enforcement="on"` under
`uap10:PackageIntegrity`, Store-compatible CIG, role targets and independent
controls. Package integrity and per-process image admission are separate
protections. Keep all executable product content inside the protected declared
package/dependency graph, with no executable mutable or external content. Use
section 9.11's packaging cases for actual enforcement and repair observations;
manifest text alone does not close CHECK-16/18/23/26. [C39]

Qualify the engine and broker's headless/leaf restrictions against their actual
SCM launch and provider graph. Give the normal-user Slint Windows UI and elevated helper
separate profiles that preserve their narrowly permitted activation, callbacks
and maintenance work. Do not copy a headless profile to the GUI, set every flag
blindly, or change a shared installer host's policy. A required-control failure
denies the affected protected work; compatibility recovery cannot silently drop
the control or expand privilege. Any necessary exception is narrow, supported
by a reproduced valid case and visible in the canonical role record with its
technical owner and evidence. David must approve a weaker release profile;
an unresolved compatibility problem is not permission for silent fallback.

The required targets include ACG without thread opt-out/remote downgrade,
Store-compatible CIG, Strict CFG for the supported executable-module closure,
strict HSP on the supported hardware class, image-load restrictions, safe DLL
search, applicable extension-point restrictions, strict handles and heap
corruption termination. Qualify headless Win32k and leaf child-process denial
for engine/broker; preserve required Win32k and approved activation in the UI.
Unsupported mechanisms or necessary-feature conflicts need a concrete recorded
applicability decision, not an indiscriminate bitmask or an optional-work label.

For every delivered executable and applicable DLL, inspect the exact final PE
and load configuration with `dumpbin /headers /loadconfig` or an admitted
equivalent, including the extended DLL-characteristics debug data. Record CFG
`Guard`, `CF Instrumented` and `FID table present` indicators where applicable,
DEP/ASLR properties, relocations, large-address awareness, both selected CET
bits and the actual `DependentLoadFlags` value. These checks must agree with
effective build inputs and the native/prebuilt/static/standard-library inventory;
a CFG-positive executable or one negative fixture does not prove complete
linked-code coverage. `nochecks` remains prohibited. No Visual Studio installation
is required merely to obtain `dumpbin`. [C10, C11, C15, C41, C42]

Pair effective process-policy queries with independently observed behavior. The inspection tool must identify the exact running binary and launch path; saved registry configuration, a screenshot label or a successful setter is insufficient. Keep the expected profile under trusted build/install ownership. A support mask or unrecognized query result must not turn a missing required setting into a pass. Preserve query status when an observer's convenience API maps selected unsupported/error/terminating statuses to zero. Ordinary CFG, Strict CFG and hardware shadow-stack enforcement are separate observations. The [Chromium/NtCoreLib examples](plans/windows-protected-storage-upstream-lessons-2.md#7-mitigations-need-ordered-startup-and-behavioral-evidence) supply concrete cases, not an admitted observer or final-artifact evidence.

Record compile/link, creation and early-runtime activation separately. Microsoft
documents that `EnableControlFlowGuard`, `EnableUserShadowStack` and
`SetContextIpValidation` cannot be changed through `SetProcessMitigationPolicy`;
they cannot all be supplied by a late runtime initializer. Readbacks, image
metadata and real native behavior must agree on the supported OS/hardware.
If self-opt-in is used for a supported control, inventory the modules loaded
before it and qualify that earlier path separately. A child-process creation
attribute does not establish how SCM or packaged activation creates a role.
This specification selects no new launcher or global policy rewrite. [C37, C38]

For CIG, record the actual supported configuration layer and Store allowance.
Exploit Protection's `MicrosoftSignedOnly` with `AllowStoreSignedBinaries` is
distinct from strict Microsoft-only. The similarly named native policy fields
are not an assumed bit-for-bit mapping; use the selected SDK and the [plan's
configuration contract](plans/windows-msix-store.md#32-store-compatible-code-integrity-guard).
Check final Store-signed images and their full observed import/DLL closure,
including cold startup, delayed loads and crypto/storage providers. Local test
signing does not establish Store signer-category admission. [C40]

| Case | Required observation | Legitimate control and scope |
| --- | --- | --- |
| Required-policy startup admission | Inject a failed setting/query, mismatched required field and audit-only result through the real initialization decision path; normal readiness and new protected work remain unavailable. Challenge invalid order and attempted return to a weaker phase in release builds | Expected enforced state admits work. A documented hardware applicability condition is distinguished from a failed required control and never reported as enforcement. Pure tests prove branching only; real OS failure/launch cases still require Windows. |
| ACG | Separately exercise harmless executable/writable allocation, protection change on existing executable memory, anonymous executable/writable mapping and file-backed mapping under the no-opt-out/no-downgrade profile; record native denial and effective bits | Ordinary heap use and exact engine/broker/provider workflows succeed, including cold startup and first lazy use. Do not execute injected payloads. Excluded/ignored instrumentation configurations remain unexecuted, not green. |
| Image loading | Separate controlled remote-image and low-mandatory-label canaries fail to load; CWD/PATH canaries exercise section 15.2 separately | The intended protected dependency loads from its expected path. Mark of the Web is not the low-label fixture; System32 preference is not a directory allowlist. |
| Store-compatible CIG | Under the actual enforced role policy, controlled unsigned or non-admitted signer-class DLLs fail to load. Record effective state and actual SCM/package/helper activation, including the pre-opt-in module inventory where applicable | Expected Store-catalog-backed product images and approved Microsoft system DLLs load through legitimate cold/lazy paths. A separately valid unrelated Store image may pass CIG but must fail NevroTick helper/IPC authorization; do not report CIG as product-identity enforcement. |
| Trusted Launch on the supported MSIX profile | With the [documented build/manifest prerequisites](plans/windows-msix-store.md#311-trusted-launch-for-the-supported-package-profile), an unlisted harmless executable cannot run under the package's identity. Inspect actual catalog inclusion and identity for UI, SCM services and elevated helper separately | Intended catalog-listed executables launch under their expected identities. Unknown namespace, unsupported build, ordinary process success or a UI-only check proves no other role's enforcement. Retain the published source example's casing defect as a schema-validation concern; repeat on actual Store-delivered bytes. |
| Strict CFG and handles | Target Strict CFG where the complete executable-module graph supports it; an incompatible code-bearing DLL is denied. Strict-handle failure occurs in an isolated native fixture | Qualified executable modules/resource-only DLLs and valid handle ownership still work. Record debugger/Application Verifier presence and actual fields; do not use a non-permanent verifier state as permanent-enforcement evidence. |
| Headless/child restrictions | Prove selected Win32k denial and actual child-creation denial for an SCM-launched service under the installed profile; harmless requests for `PROCESS_CREATE_PROCESS`/`PROCESS_VM_WRITE` on dummy peers and inherited/duplicated-handle review must also establish the required isolation | Final service APIs, permitted read-only queries, boot-before-login, stop/restart and recovery work. A custom-parent fixture or queried child-policy bit does not close the SCM/composition gap. Test approved UI/helper activation separately; do not apply the headless profile to the GUI or shared installer host. |
| CET/HSP | Reconcile the selected `/CETCOMPAT` and `/CETCOMPATSTRICT` bits with CPU/OS/VM support, effective enabled/strict state and audit disabled. Qualify the strongest supported module restrictions against actual metadata and the complete closure; retain section 6.3's EHCONT compiler gate | Legitimate native callbacks/unwinding paths work. Source selection is complete; hardware/OS enforcement is not yet qualified. An unsupported hardware condition is recorded without inventing CET support from the v3 CPU floor. |
| Lifecycle and mitigation failures | Preflight blocks an unusable required profile before clock takeover; reached update/fault cases preserve ownership, budgets, cleanup and bounded SCM recovery | A valid complete-MSIX install/update/repair/remove completes; a later owner-selected MSI alternative needs separate evidence. Existing CHECK-12/13/18/23 remain authoritative. |
| RedirectionGuard applicability | Resolve support and applicable file-owning roles in the canonical record; qualify enforcement for an in-scope unprivileged junction alongside unsupported/query failure and administrator-created/legacy controls | Ordinary file/provider/lifecycle operations succeed; application path checks still reject out-of-scope forbidden paths. The exact supported profile remains open required assessment, not an assumed universal junction boundary or an extra UI-M0 gate. |

Run native denial/termination tests only in the owned disposable Windows environment of architecture section 14.9, with independent observation, resource/time bounds and cleanup. No test changes the user's clock or global Exploit Protection policy. Audit-first trials apply only to controls with a documented audit mode; an audit event never passes an enforcement case. Preserve UI-M0 deferrals: full production accessibility/IME integration tests run at the existing reentry gates.

Tie records to the complete payload/SBOM and exact package. Preserve submitted and Store-delivered identities separately; repeat affected readbacks, loaded-image observations and negative/valid controls on the actual final Store artifact before broad release. Repeat affected cases when a toolchain, native provider, linked module, startup/hosting path, Windows support matrix or installer policy changes. Use the existing diagnostics and artifact records rather than a new background scanner. Do not copy `+crt-static`, admit held cryptographic providers or add a launcher solely to make a mitigation checklist appear complete.

#### 6.4.1 Protected-storage provider qualification

The [optional-storage contract](windows-time-architecture.md#1212-protected-optional-storage-and-explicit-fallback)
and [staged plan](plans/windows-protected-storage.md) add actual provider work to
the existing matrix. These cases are **planned / not executed**; the owner's
storage-experiment pause remains in effect. No new service, dependency graph,
launch path or process-policy relaxation is admitted.

| Boundary / check | Required discriminating observation | Healthy neighbor |
| --- | --- | --- |
| Origin/reopen; CHECK-15/17 | Ordinary unprotected software keys, wrong-provider, unproven-origin and partial/mixed-protection sets cannot receive a stronger VBS/TPM label; imported keys cannot claim never-host-exposed history; missing expected key is recovery | Complete admitted wrapping/signing set reopens with supported evidence under final identity and no-login startup |
| Ownership; CHECK-1/15 | Another LocalService service and ordinary user cannot use/replace keys or exploit owner-mediated changes; protect catalogue/files/directories separately | Intended engine can provision/use/retire without broader rights or prompts |
| Envelope/save; CHECK-15/25 | Validly formed unauthorized public-wrap/AEAD envelope and old signed snapshot fail against intact current anchors/content binding | Exact committed snapshot restores with original trust/age metadata |
| Native lifecycle; CHECK-15/17/27 | Prepare/commit/ack/retire interruptions, saturation and late invalidated results have bounded truthful recovery | Save/restart/rotation completes and releases actual resources; NoPersistentCache requires no root |
| Mitigations; CHECK-16/18/20/23 | Provider work never relaxes required ACG/CFG/StrictCFG/CET/CIG/image/Win32k/child fields; unknown/audit-only results cannot pass | Final provider graph works under installed SCM token/profile; full-enclave graph is a separate conditional row |
| Recovery; CHECK-9/11/12/15 | VBS loss/return, TPM clear and account change cannot restore retired state, weaken trust or reset broker authority | Fresh admissible KE and declared wrong-date/offline key paths work; owned cleanup remains available |
| UI/privacy; CHECK-14/17/22, DNS-C14/15 | Missed boot events, stale status, forgotten inaccessible snapshots and late saves cannot create false protection or resurrect history | First authenticated UI connection shows current mode/reason; authorized demand and redacted status work |
| Operation outcome/owner; ST-T06/07/10/16/17 | Distinguish a rejected poisoned transaction from a possibly durable commit error; unsafe retry, generation reuse, new-root inference and premature resource release fail | Attributable confirmed commit or valid rollback/reconciliation permits only the resulting owner operations; mandatory authority stays separate |
| Directory authority; ST-T03/05/10/17 | Parent/leaf substitution, prohibited link/reparse/type/owner and absolute/device/UNC/ADS/traversal inputs fail before destructive change | Intended relative regular-file save and authorized lifecycle rename succeed with correct handle release; no filesystem library is presumed universally handle-relative |
| CNG calls/properties; ST-T01/02/05/12/13/16 | Each private operation preserves documented no-UI behavior; short/changed/oversized property results, wrong version/flags and expected-None plus denial cannot pass admission | Bounded actual property and operation results qualify under the final account/session; unsupported flags are never indiscriminately added or stripped |
| Exact crypto expectations; ST-T05/18 | A wrong hash/KDF/context/encoding or output length fails an independent suite-specific fixture, even if both roundtrip sides share the error | Correct expected bytes and normal authenticated save/restore succeed; the source example selects no ECIES/KDF construction |
| Durable model and protocol monitor; ST-T05/06/07/10/15/16/18 | Independently track issued/live/durable/acknowledged state and requested/succeeded/failed barriers; reject early acknowledgement, wrong digest/epoch, premature retirement and late privacy-invalidated publication, including failure during recovery | Healthy commit/restart/cleanup preserves declared mandatory reservations and privacy barriers; stop the trace at a simulated crash cut rather than counting later artificial-success returns |
| Memory/output privacy; ST-T14/16, DNS-C14 | Failed locking is distinct from successful allocation; native custody delays wiping/release; synthetic key/cookie/domain sentinels remain absent from actual permitted output artifacts | Bounded valid allocation/use/cleanup and useful redacted diagnostics succeed; guard-page portability skips and collector-specific exclusions prove no whole-memory protection |

Freeze API/phase outcomes, provider/build/algorithm/identity rows, limits,
owned dummy paths and cleanup before a future native run. Dummy evidence proves
that probe's path, not another key's origin. Preserve anchored authentication and
offline-anchor/rollback limits. A healthy-clock sample, setter, binding or
CET image marker is insufficient. Existing held TLS/CNG fixture receipts retain
their narrower scope.

If a full-enclave row is later selected, keep the runtime build/VBS/HVCI evidence
separate from developer tooling and special production-signing admission in the
[storage plan](plans/windows-protected-storage.md#5-platform-admission-failure-classification-and-privacy).
None establishes support for the distinct CNG VBS-root path, or lifts its pause.

The [second source supplement](plans/windows-protected-storage-upstream-lessons-2.md)
owns attribution and adaptation limits for these refinements. Portable models
and event monitors qualify their declared protocol/assumptions only; real Windows
filesystem/provider durability remains separate. IPC close tests must distinguish
request-to-close, publication revocation and actual drained completion, retaining
resources through recovery. Record actual selected/executed/skipped controls;
upstream test names and a healthy roundtrip establish no independent expected
bytes or complete native coverage. Keep hardware counters/attestation/chunked
snapshots/database selection in the task's conditional reserve.

The [Learn follow-up](plans/windows11-integration-follow-up.md#4-diagnostics-replacement-and-protected-storage)
concretizes the existing outcome and privacy rows: model `ReplaceFileW` errors
1175/1176/1177 and generic failure with/without an admitted backup slot; inspect
actual objects and permissions before cleanup, then qualify selected-filesystem
durability separately. Add named WER/LocalDumps/debugger collector cases rather
than assuming NOHEAP or exclusions protect every artifact. Per-service state
locations remain a placement comparison whose service-deletion lifetime cannot
erase required continuity. VBS claims and side-channel-policy fields remain
[conditional](plans/windows11-integration-follow-up.md#6-conditional-mechanisms);
neither an API result nor requested-bit readback proves their broader claim.


### 6.5 Windows x64 optimization policy and evidence limits

The selected workspace release candidate is `opt-level=3`, `lto="thin"`, `codegen-units=1` and `incremental=false`; `verify-release` inherits those settings and enables debug assertions. Overflow checks, unwind policy, retained symbols and Windows mitigations remain. Ordinary development retains its incremental workflow. Thin LTO enables cross-crate optimization with less link work than fat LTO; one codegen unit reduces parallel code generation and can increase build time. Neither setting guarantees faster code, and level 3 can be slower than level 2. Compare useful runtime work, startup, tail latency, memory, binary size and build cost on the declared workload before claiming a benefit. No new profile, PGO pipeline or performance result is established here. [C08]

The supplied **Optimized build of the compiler** guide concerns rebuilding `rustc`, including `bootstrap.toml`, its allocator and `opt-dist`; it is not an application build recipe. Its Windows LTO warning links open issue **rust-lang/rust#109114**, checked 29 October 2026: unstable `-Zdylib-lto` with ThinLTO and dynamic Rust-library imports on MSVC. That issue does not establish a general failure of ordinary stable Cargo application ThinLTO. Keep the unstable/dynamic-library experiment outside the selected configuration, and qualify the actual compiler/linker/target/graph rather than assuming either immunity or universal breakage. [C24, C25]

The selected `target-cpu=x86-64-v3` implements
[OD-07](nevrotick-owner-decisions.md#11-od-07--x64-baseline-and-runtime-optimization)
in the target configuration and exact checker expectations. Section 6.7 retains
required prelaunch admission and supported CPU/OS/VM qualification; older generic
receipts do not qualify this floor. Runtime-selected higher-feature functions
remain separate measured work. Global v4 and build-host `native` compilation are
excluded. Application flags do not rebuild the supplied standard library or
prebuilt native objects; rebuilding `std` is a separate unstable workflow and is
not adopted. Preserve section 6.2's complete selected hardening vector, including
jump-table suppression, during optimization comparisons. [C10, C28, C44]

PGO remains an optional future experiment. Rust's pinned profiler runtime includes Windows/MSVC support, but a selected Windows instrumentation/link/runtime lane and compatible `llvm-profdata` must be admitted and observed. Train the actual Windows executable on representative bounded inputs; retain source, lock/features, complete effective flags, binary/tool identities, workload/seed, raw profiles and merged-profile hash. Use absolute paths, fresh owned profile output and matching generate/use settings; LLVM raw profiles have no forward/backward format guarantee, so prefer tools from the pinned toolchain. Missing/stale profiles or unexecuted training provide no result. [C26]

Preserve the complete hardening flag vector during PGO/CPU experiments: `RUSTFLAGS` or encoded flags replace configured target flags rather than appending automatically. Use a reviewed target-specific configuration or explicitly composed admitted vector, with `--target` separating target flags from host build scripts/proc macros. Ordinary Rust LTO does not automatically provide cross-language LTO for native dependencies; that requires a separately admitted compatible LLVM/native/linker arrangement. [C18, C29]

BOLT's documented inputs are X86-64/AArch64 ELF binaries; this target produces PE/COFF. BOLT is not selected for Windows application binaries. None of these settings closes CHECK-21's native performance/soak evidence, the emitted-image/runtime mitigation checks, or package/release gates. Reuse older receipts only for unchanged inputs; a profile or target-flag change requires fresh affected evidence. [C23, C27]

**Windows DLL-import qualification addendum, 29 October 2026:** The preceding reproducer distinction is not an immunity argument. Open issue **#27438** covers broader Rust/native `dllimport` handling, including native global-data declarations and build-script library selection. Historical **#81408** affected an ordinary Rust executable with cross-crate static data, ThinLTO and LLD. Later changes **#129079** and **#122790** revise that handling; the pinned compiler contains self-import generation and distinguishes regular in-process ThinLTO from linker-plugin LTO. It also retains a Windows regression test that executes binaries built with both `link` and `rust-lld`. Source/test presence is useful evidence, not a local execution of that test, a blanket fix for #27438, or qualification of this application's native graph. [C36]

ThinLTO remains an **unqualified optimization candidate**. Before a Windows release claim, compare `lto="off"` and `"thin"` through relevant native correctness and useful-work cases on the actual compiler/linker/graph; include cross-crate static/string use and reached Windows/CRT import paths, with isolated imported-data controls where that mechanism is being qualified. No Rust DLL in one artifact, successful linking, Linux tests or a different linker alone proves Windows runtime correctness. Turning LTO off does not repair incorrect FFI import declarations, and fat LTO is no presumed workaround. Retain CHECK-19/20/21 and existing native gates without treating this source investigation as a pass. [C36]

### 6.6 LLVM settings for this workload

For the pinned **Rust 1.99.1 / LLVM 23.1.1** Windows x64 toolchain, section 6.5's checked O3/ThinLTO/one-codegen-unit configuration is an **unmeasured optimization candidate**, not a Windows correctness qualification or proof of fastest code. Keep additional `llvm-args` and custom pass lists empty. The pinned compiler enables loop and SLP vectorization at O3 and builds LLVM's normal optimization/LTO pipelines, including their inlining, unrolling and target-cost decisions. This permits legal optimizations; it does not promise every loop will vectorize or every call will inline. [C30]

| Disposition | Setting or family | NevroTick constraint |
| --- | --- | --- |
| Recommended default | Selected checked profile; ordinary LLVM pipelines and cost models | Preserve exact timestamp arithmetic, admission, finite-result checks and useful failure paths. Keep the default loop/SLP/inliner/unroll decisions rather than a hand-written pass sequence. [C30] |
| Recommended default | Windows x64 target ABI, default relocation/code models and section 6.5 CPU baseline | Keep unwind metadata, CFG, ASLR/DEP, symbols and existing linker-warning failures. Do not transplant Unix, kernel, alternate calling-convention or nonrelocatable settings to the application. [C10, C23, C28] |
| Measurement only | O2 versus O3, LTO/codegen-unit alternatives and the admitted PGO experiment | Use the [performance measurement reference](../.agents/skills/implementation/optimize-rust-performance/references/measurement.md) for bounded candidate comparisons. Require unchanged numerical/hostile-input results and report runtime, tail latency, startup, retained resources, artifact size and build cost; section 6.5's native/release limits remain. |
| Avoid as blanket tuning | Forced vector widths, interleave/unroll factors, arbitrary inliner thresholds and disabled vectorizers | First identify a real hotspot and inspect the emitted code/remarks. Any narrowly justified experiment must use the exact pinned compiler's supported interface and retain independent numerical checks; no such override is selected. LLVM's cost model already accounts for profitability and code-size/register-pressure trade-offs. [C30, C34] |
| Avoid | Fast-math, no-NaN/no-infinity assumptions, reassociation, approximate arithmetic and global flush-to-zero/denormals-are-zero changes | These can change numerical behavior or the floating-point environment. Preserve the declared operation, rounding, special-value and subnormal contracts. This is not a claim that FMA is globally disabled: explicitly fused operations and compiler-permitted transformations must be assessed under their actual semantics. [C35] |
| Avoid as a stable configuration | Removed `-C inline-threshold`, unstable/internal `-Z tune-cpu`, raw LLVM switches and replacement pass pipelines | The pinned option table rejects `inline-threshold`; `tune-cpu` is an unstable option. Raw LLVM arguments/pass names have no general rustc CLI stability guarantee. A copied option name, successful parse or newer manual does not qualify its meaning on the admitted toolchain. [C31] |

For ordinary optimized MSVC-flavor links, the pinned rustc already requests `/OPT:REF,ICF`; unoptimized links request `/OPT:REF,NOICF`. Retained debug information causes `/DEBUG`, which the pinned LLD interprets as a full PDB request. That LLD derives nonincremental linking when REF/ICF are active; Cargo's `incremental=false` separately controls compiler caching. Do not duplicate these flags as a presumed performance improvement. A full PDB packages the information emitted: `debug="limited"` still omits Rust type/variable-level information. ICF can merge function/read-only-data addresses and complicate names/breakpoints; address uniqueness is not a safe identity contract. [C10, C32, C33]

Do not add `/LTCG` as a substitute for Rust LTO or `/DEBUG:FASTLINK` as PDB boilerplate. The pinned LLD warns and substitutes full PDB for FASTLINK, conflicting with `/WX`; native MSVC LTCG and cross-language LLVM LTO have their own input/toolchain requirements. Future native dependencies need their own admitted compiler flags, graph and emitted-object evidence; Rust flags do not retrofit their protections. Record effective inputs and exact artifacts under CHECK-19/20, and retain CHECK-21's separate native performance/soak obligations. These tuning rules add no raw LLVM override or performance result; section 6.2 owns the selected code-generation and linker vector. [C17, C29, C32, C33]

### 6.7 v3 baseline and runtime CPU dispatch

[OD-07](nevrotick-owner-decisions.md#11-od-07--x64-baseline-and-runtime-optimization)
makes CPU optimization a required workstream. The Windows x64 minimum
is `x86-64-v3`, now selected in the build configuration and exact admission
controls. The [existing workstream](plans/development-plan.md#81-x64-baseline-and-runtime-optimization-workstream)
still owns prelaunch CPU admission, native qualification and useful optimization.
The [research report](supporting-reviews/x86-runtime-optimization.md) records the
sources and limits. Source selection establishes no native pass, measured gain
or runtime-dispatched kernel implementation.

| Owner/guarantee | Required implementation and independent observation |
| --- | --- |
| Build owner: baseline and artifacts | Preserve the selected v3 target entry, exact checker/control expectations and complete section 6.2 vector together with profiles, overflow checks, unwind policy and symbols. Inspect effective Rust/native/generated-object flags and final payloads; do not replace the vector through ambient `RUSTFLAGS`. Audit standard-library and prebuilt inputs separately. |
| Lifecycle/package owner: supported machines | Declare actual CPU/OS/VM rows. A prerequisite check that itself runs on the rejected machine must precede execution of any v3 payload. Observe useful behavior on a v3-capable baseline and clear refusal below it, including masked guest features. Keep one complete product package; catching an illegal-instruction crash is not admission. Windows 11 branding alone proves no complete feature set. |
| Kernel/dispatch owner: available functions | Compile optional specializations ahead of time with explicit `#[target_feature]` sets. Use Rust runtime feature detection, including its OS vector-state checks; select only a function whose whole enabled set is available. `avx512f` alone is not full v4. The baseline path remains reachable; never globally enable AVX-512, alter XCR0, or treat detection as runtime recompilation. |
| Safety/algorithm owner: unchanged contracts | Keep unsafe code out of the pure core. Any necessary feature-specific unsafe boundary needs a narrow reviewed owner and checked safe interface outside that prohibition. Preserve length/alignment/bounds/lifetimes, exact integer time arithmetic, chronological floating-point operations, intermediate checks and error/failure atomicity. New reductions or FMA rounding require separately justified algorithm changes; no fast-math, reassociation or FTZ/DAZ shortcuts. |
| Crypto owner: independent capabilities | AES/PCLMUL/VAES are not guaranteed by v2/v3/v4. Use the admitted provider's feature dispatch and fallback; retain provider/licence/trust gates and qualify real small-packet and KE workloads separately. |
| Performance/QA owner: benefit and limits | Identify a real hotspot and predeclare useful-work metrics, noise/acceptance rule and finite campaign bounds. Compare baseline and each supported specialization on native Windows with normal protections. Require correctness/rejection parity and observe startup, tail latency, CPU/wakeups, retained resources and binary size. Hardware availability alone does not justify selecting AVX-512 over AVX2. |

Dispatch tests cover v3 fallback and supported AVX-512 subsets, required
feature absent, CPU feature masked by a VM, and OS vector state unavailable.
Distinguish the layers: unavailable baseline AVX state rejects v3 startup;
unavailable AVX-512 state with usable v3 selects the v3 fallback. A disabled
baseline feature must never be treated as merely a slower optional path.
An injected feature inventory may select/test a branch without executing an
unsupported instruction. A forced optimized execution must first satisfy the
real detector; a forced baseline mode may always disable an optimization. Retain
native supported/unsupported rows in addition to such selection controls.
Shared dispatch/API tests do not replace numerical checks of each implementation.

Implement only measured kernels and a small auditable selection seam; do not
scaffold a generic dispatch framework or product crates solely for this policy.
The workstream must evaluate meaningful candidates; an unused feature probe or
empty wrapper is no optimization result. Record a non-beneficial candidate and
its revisit trigger rather than shipping it as faster. Preserve CHECK-19/20
build/artifact evidence, CHECK-25 numerical/contract controls, CHECK-26 graph
admission and CHECK-21's independent native resource/soak obligations. Missing
native rows or a changed algorithm remain incomplete, not passing fallbacks.

## 7. Dependencies, auditors and supply-chain policy

### 7.1 Exact graph before execution

Commit `Cargo.lock`. Maintain a feature matrix for the portable core, production engine/provider, broker, UI/helper, packaging and each admitted native target. Record ordinary, build and macro dependencies plus native/generated/bundled material. Cargo features can unify through other paths; a direct `default-features=false` does not prove that a transitive path cannot enable defaults. [SC19]

Do not use `--all-features` as a universal security check. It can activate mutually exclusive providers or capabilities outside the product. Analyze and test every supported row explicitly, and demonstrate that a transitive feature cannot restore a held provider.

A dependency proposal must state what complexity or bug class it removes, exact versions/source, feature changes, build/macro/native behavior, license obligations, advisory evidence, target/MSRV compatibility and affected consumers. Verify source archive/tag parity when relevant. Popularity and an empty advisory report do not establish soundness.

Separate acquisition from execution. Command families include `cargo fetch --locked` to prepare exact inputs and `cargo build --frozen` for locked/offline Cargo resolution. Actual wrappers supply the admitted target/package/feature row. `--frozen` is **not** a network or filesystem sandbox for code executed by the build. [SC01–SC03]

After verified acquisition, deny egress independently and build from staged inputs with no automatic fallback downloads. Native archives, code generators and tool packages need the same provenance discipline as Rust crates. Build scripts' conventional `OUT_DIR` usage is not a security boundary.

#### 7.1.1 Windows API and wrapper admission

Apply [architecture §18.6.2](windows-time-architecture.md#1862-windows-component-reuse-and-binding-coverage)
when the existing adapter consumer is reached. Extend its CHECK-26 record with
operation/owner, selected crate and published checksum/source, feature/target
row, required SDK symbols and any pinned generator/metadata/output. Review the
actual consumer path and remaining rights, bounds, error, callback and completion
contract; a smaller source file is not sufficient evidence of equivalent behavior.

Do not mix the planned `windows` 0.62.2/`windows-sys` 0.61.2 generation with
current upstream namespaces or core types by assumption. Publication, source
parity and a compilable target are separate checks. Resolve the documented
VBS-flag gap at ST-W01/ST-W04 before its dependent implementation, retaining
REQUIRE semantics and actual-key/native support evidence. Prefer a compatible
published binding or narrow reviewed generation over hand-transcribed ABI or
unverified constants; do not add generation to every build without need.

For a proposed wrapper replacement, pair a legitimate operation with the relevant
error/oversize/denial/reentrancy/late-completion control. Preserve SCM status
errors and progress reporting, window-loop error distinctions, registry access
and allocation limits, correct resource destruction and finite native-job custody.
Run actual native checks only under the existing authorized fixture and holds.
UI alternatives additionally need equivalent workloads, measured resource cost,
accessibility, package/runtime inventory and mitigation compatibility at S0.5/P5.
Conditional candidates and source-only gaps add no new current-Core or global
release gate; existing CHECK-21/22/26/27 apply only to reached selected surfaces.

### 7.2 Tool selection and overlap

| Tool | Decision | Important limit |
| --- | --- | --- |
| cargo-deny | Primary blocking dependency policy after qualification | Match exact graph, licenses, source/advisory policy and configured scope. [SC06] |
| cargo-audit | Optional distinct artifact scan with cargo-auditable; no redundant lockfile gate required | Repeating RustSec checks is not independent vulnerability evidence. [SC07, SC08] |
| cargo-auditable | Build-only candidate after PE/packaging qualification | Verify embedded inventory survives final transformations; it is not a signature or complete native SBOM. [SC08] |
| One SPDX/CycloneDX generator | Required reconciled release inventory; cargo-cyclonedx is a candidate | Include native objects, generated source, resources, tools and complete package contents beyond Cargo metadata. [SC14] |
| cargo-vet | Optional staged audit-record system | Needs real maintained review criteria/imports/deltas; exemptions are not performed audits. [SC15] |
| cargo-crev | Defer as an alternative review ecosystem | Running two overlapping trust systems at kickoff is not necessary. [SC16] |
| cargo-geiger | Optional unsafe inventory | Statistics do not prove soundness or cover all native/FFI risk. [SC17] |
| cargo-machete | Optional dependency hygiene | `--with-metadata` may modify the lockfile and uses all features; keep it out of an assumed immutable gate. [SC18] |

The repository bootstrap now installs pinned `cargo-cyclonedx` 0.5.9 in a
separate release-tool lane. Its [configuration and offline wrapper](../TOOLING.md#release-sbom-tooling)
produce Rust component inventories and reject target/feature graph mismatches;
they do not qualify a complete native/package SBOM. Every published version must
attach the reconciled, validated SBOM with its complete package and final signed-byte
checksums. S6.4/CHECK-26 retain package reconciliation and protected publication.

### 7.3 cargo-deny outcomes, not copied policy folklore

Require approved origins/revisions; applicable advisory disposition; denied unreviewed yanks/unmaintained paths; explicit license choices; bounded duplicate exceptions; forbidden capability/provider paths; and a complete graph scope including relevant build/dev tooling.

Current cargo-deny documentation contains configuration drift: an advisory sample includes `unsound`, while another section describes it as removed. Some historical license keys are also obsolete. Therefore this guide intentionally does not offer a guessed production `deny.toml`. Generate and verify the schema against the **admitted exact binary/source/help**, then exercise representative forbidden and allowed inputs. Set build/dev scope explicitly; current license defaults differ. [SC20–SC23]

SPDX `MIT OR Apache-2.0` permits a choice; `Apache-2.0 AND ISC` requires both. A broad allowlist accepting each token does not establish compatibility of the combined product. Preserve the architecture's exact-graph license gate, including native components. The generic `ring` clarification in cargo-deny examples is not authoritative evidence for our inspected release and must not be copied to erase an obligation. [SC21, SC22, SC24]

Standalone development-tool licensing and linked runtime licensing are different questions. Conversely, generated or copied material can affect distribution even when its producer was a development tool. Record the actual relationship rather than inferring clearance from the dependency category.

### 7.4 Advisory freshness and reproducibility

**Proposed policy:** a protected job refreshes security metadata daily, and merge/release decisions require a successful authenticated refresh receipt no older than **24 hours**. Retain its exact database commit/digest for reproducibility. All lanes in one decision consume the same snapshot; a later release also checks current evidence.

Distinguish upstream freshness from file modification/cache access time. Record origin, retrieval receipt, tool/configuration, graph/artifact identity and exceptions. A long-unchanged advisory entry is not stale merely because its original publication date is old.

Cargo-deny's documented database-age limit is checked when fetching is disabled and has its own timestamp semantics. Qualify that mechanism; it does not substitute for our refresh receipt. If current acquisition is unavailable, dependent merge/release decisions are **BLOCKED**, not green using silent stale data. Clearly labelled local offline results may still help development. [SC23]

RustSec primarily covers known Rust ecosystem records. Native libraries, SDK/installer tools, actions and analyzers require their own advisory tracking. An immutable vulnerable compiler remains vulnerable.

## 8. Static analysis, workflow checks and CI authority

### 8.1 CodeQL must run inside the execution boundary

Current GitHub documentation supports Rust CodeQL using `build-mode: none`. Despite the name, its Rust extractor uses rust-analyzer to compile/run build scripts and compile macro code. Treat it as target-controlled execution, with no release credentials or privileged OIDC. Do not invent a Rust manual-build extractor configuration. [SC04]

Qualify the extractor/query packs against real source, generated code, target/cfg and Windows paths. Seed independent safe/unsafe data-flow fixtures for packet/IPC/config inputs reaching length/allocation/path/FFI boundaries and sensitive effects. Record analyzed-file coverage and unsupported models. Standard queries may not understand our clock-authority protocol; add a reviewed project query or retain the distinct native/manual gate.

Relevant findings must block until corrected or independently dispositioned. Removing queries/excluding a module is a policy change. A green SARIF upload does not prove a relevant query ran or its finding could block the current commit. [SC04, SC09]

### 8.2 Focused workflow and secret checks

Use actionlint for workflow semantics and zizmor for selected security rules. Pin both and record modes/rules/suppressions. **zizmor SARIF output intentionally suppresses finding exit codes**; use an enforcing invocation or a tested report-policy evaluator in addition to publication. Offline mode skips online audits. Give online metadata access only to a separately scoped job, not to an untrusted compiler. [SC05, SC10, SC11]

Use a pinned Gitleaks CLI plus hosted secret scanning/push protection where available. Scan changes/tree on PRs, retained history at initial qualification and scheduled reviews, and final staging contents. Redact reports; report skipped/oversized files and history gaps. Synthetic fixtures test detection. Real exposure requires owner-led revocation/rotation, not merely deleting the current line. [SC12, SC13]

Run untrusted PR compilation with read-only access and no secrets. Prohibit privileged `pull_request_target` or `workflow_run` handlers that check out and execute untrusted PR code. Never interpolate untrusted titles/branch names into shell source. A platform default is defense in depth, not our policy boundary. [SC25, SC26]

### 8.3 Policy files, actions, caches and promotion

Protect workflow/gate code, lint/advisory exceptions, critical manifests, release permissions, lockfiles and tool pins with independent ownership review. Hosted branch/ruleset and environment settings must be evidenced, not inferred from YAML. Dismiss obsolete approval after relevant changes and bind results to the exact candidate/merge commit. An agent may propose a gate change; it cannot approve its own weakening.

Require an always-scheduled final gate with explicit dependencies, using GitHub's `always()` condition with `needs`. It must validate that every expected job/matrix shard has an explicit successful result and complete identity-bound evidence. Skipped, neutral, cancelled, missing or merely uploaded reports cannot satisfy a required analysis. GitHub can treat skipped jobs as successful, including a dependent job skipped after a failure, so checking a superficially green status is insufficient. Protect the aggregator itself; ensure applicable PR/merge-queue events invoke it and that its expected check source is enforced. [SC31]

Pin direct actions/reusable workflows to reviewed full SHAs and verify repository identity. GitHub's newer workflow dependency lockfiles also address transitive actions, but remain **public preview**. Called workflows use their own repository's lockfile. Evaluate compatibility and behavior before adoption; retain direct pins and explicit transitive review when the preview is unavailable. [SC26, SC27]

Partition caches by trust domain, toolchain, lock/configuration, target and features. Untrusted runs must not supply trusted release executables. Prefer a fresh release build from verified inputs. Treat artifacts and report archives as untrusted data: fixed schemas/size limits, safe extraction, no path traversal and no scripts executed by the importer.

SBOM, provenance attestation and package/catalog or applicable Authenticode signatures answer different questions. Verify subject hashes and identities; reconcile source/build evidence with the documented Store or other selected signing transformation and verify the final payload/package again. No attestation proves the code is bug-free. [SC28]

Under [OD-05 §7.5](nevrotick-owner-decisions.md#75-optional-enhanced-build-provenance--4-november-2026),
GitHub/Sigstore attestations, independent bit-for-bit builds and a public
source-to-Store proof bundle are optional; their absence is not a release gate.
Apply attestation verification if that evidence is supplied. Required SBOM,
source/licence custody and package/installed trust checks retain their existing
scope. The [focused record](plans/release-provenance.md) owns the optional work.

## 9. Tests, coverage and evidence of bug detection

### 9.1 Meaningful execution and doctests

Use nextest for ordinary tests with explicit `--no-tests=fail`, and run `cargo test --doc` separately because nextest excludes doctests. Commands below illustrate the default-feature workspace only; the final gate wrapper must use the reviewed package/target/feature matrix and sandboxed inputs. [V01]

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --frozen --target x86_64-pc-windows-msvc
cargo doc --workspace --no-deps --frozen --target x86_64-pc-windows-msvc
cargo nextest run --workspace --frozen --target x86_64-pc-windows-msvc --no-tests=fail
cargo test --workspace --doc --frozen --target x86_64-pc-windows-msvc
```

These commands are not claims of available tools or a valid all-workspace product feature combination. Pin the tools first and replace broad selection where a row requires narrower packages/features. Use corresponding portable-core rows on Linux. Compiler warnings come from the lint policy; do not replace mitigation flags with a convenience `RUSTFLAGS` setting.

Record discovered tests, executed assertions, skips and subprocess results. A crate without executable doctests may legitimately have zero, if explicitly inventoried. Losing required doctests or filtering every relevant test must fail. Domain API checks should include a constructor/privacy compile-fail case and a legitimate construction case where appropriate.

### 9.2 Independent properties and oracles

Persist minimized proptest failures and generation seeds/configuration. Random generation is unlikely to find every isolated value; explicitly include zero/sign neighbors, min/max, era wrap, NaN/infinity, exact integers, duplicate/escaped keys and one-byte-over-limit inputs. [V10, V11]

The simulator owns independent true UTC, oscillator error, asymmetry and source faults. Production code must not compute its expected answer. Preserve analytic cases and protocol/crypto vectors alongside pinned Chrony trajectory comparison. For example, the architecture's `T1=0, T2=70, T3=80, T4=110` millisecond fixture yields +20 ms offset and 100 ms network delay. Include an asymmetric-delay case demonstrating bias; authentication does not remove that physical limitation.

The [S2.1 controls](../tasks/engine-transport-pilot.md#independent-controls-and-proof-obligations)
separate that ideal decimal example from exactly encoded Q32/Q33 vectors and
independently reconstructed real-UDP observations. Changing only the correlation
token must leave arithmetic unchanged. Fixed sleeps or injected timestamps prove
no native capture accuracy; inspect actual returned measurements and, at the
later admitted consumer, actual retained history and fit. Fixture weights remain
fixture inputs until the statistical owner justifies their production derivation.

Separate recorded-packet replay from first delivery delayed within an outstanding
authenticated request. Apply [architecture section 7.4](windows-time-architecture.md#74-nts-implementation-contract)
and the [protocol qualification matrix](../tasks/s0.1-protocol-comparison.md#replay-and-delayed-response-qualification)
under CHECK-6/7/8/9/21: completion/expiry, queued processing at the deadline,
restart with restored keys/cookies, suspend/resume and protected-response races
need discriminating rejection and healthy controls. Exercise real NTS admission
with independent crypto vectors before claiming authenticated replay protection;
the unauthenticated fixture cannot supply that evidence. For an otherwise
admissible first-delivery delay, demonstrate the independently expected bias and
uncertainty; rejection of every delayed sample is no proof of useful operation.

Compare selected/supporting/rejected identities, reason codes, uncertainty, polling and full correction accounting. Agreement with a translated upstream algorithm can reproduce a shared bug. Require an independently valid input that produces the intended effect and invalid/stale/unauthorized inputs that produce no forbidden effect. Round trips, mocks and a harness that repairs the output cannot establish production behavior.

### 9.3 Coverage: at least 90% for every security module

Maintain a reviewed manifest mapping security modules to production paths, targets and feature rows. Export machine-readable coverage and reconcile **every expected module** before computing at least 90% executable-line coverage for each one. A high workspace total cannot hide an uncovered authorization module.

cargo-llvm-cov documents working x64/i686 MSVC coverage and ARM64 MSVC default-setup limitations. Branch and doctest instrumentation remain nightly/unstable. Do not label Linux-only coverage as coverage of `cfg(windows)` code. Do not use `--ignore-run-fail` to turn a failed test run into a successful report. [V02]

Publish branch measurements separately with tool limitations and keep named adversarial branch tests regardless of instrumentation. Missing files, stale data, zero executable denominators and unsupported required modules are incomplete. Exclusions need review; test code/generated declarations do not stand in for production behavior. Preserve raw profiles and build/source identities.

### 9.4 Mutation: at least 95% for each critical source file

Keep the critical manifest covering parsers, authentication, selection/controller safety, IPC authorization, clock budgets, persistence migrations and lifecycle transactions. Changed-file campaigns qualify an affected change; the release gate still requires complete current critical scope.

For each file, retain generated/admitted mutants and these separate outcomes:

| Symbol | Meaning | Treatment |
| --- | --- | --- |
| K | Caught by a behavioral test | Counts as caught |
| M | Missed, with complete viable execution | Requires stronger tests or independently justified equivalence/exclusion |
| U | Unviable build | Not a test kill; explain instrumentation/build limitation |
| T | Timeout | Investigate; incomplete until resolved |
| X | Unexecuted/error/missing shard | Incomplete |
| E | Independently justified equivalence/exclusions, a subset of M | Report exact excluded cases and requirement-grounded rationale |

Report raw `K / (K + M)` and adjusted `K / (K + M - E)`. Require at least **95% adjusted for every file** and complete accounting. Every survivor requires stronger tests or independently justified equivalence/exclusion, as the architecture already requires. A backlog entry, generic risk acceptance or a passing numerical score does not discharge that obligation. An empty denominator is unresolved. A known non-equivalent security defect cannot be accepted merely because the percentage passes. Do not add build failures or timeouts to K or hide functions to improve the score. [Architecture §18.4; V03–V09]

Reconcile a second inventory: each critical requirement-bearing function/guard,
whether it generated mutants, why any behavior was skipped/unmodeled and the
independent alternative evidence. cargo-mutants 27.1.0 automatically skips
`unsafe fn` and generally cannot inspect macro/build-generated bodies; a mixed
file can have a nonempty 100% score while its critical callback generated none.
Absence from generation is outside K/M/E, not an equivalent missed mutant. Observe
derived Serde/garde admission and native callbacks through independently specified
raw inputs, handler/fault controls and reached native checks as applicable. Keep
unsupported obligations visible beside the score; do not retroactively invalidate
unchanged pure safe-core receipts based on a hypothetical unsafe callback.
See architecture S118 and the [mutation skill](../.agents/skills/testing/qa-mutation-testing/SKILL.md).

cargo-mutants' exit code does not implement this custom gate: retain the native result, parse its admitted report schema, and distinguish missed mutants from failed baseline/timeouts/tool errors. A complete missed-mutant report may be evaluated under the explicit threshold; arbitrary nonzero results must never become success through `|| true`. Windows-only campaigns need appropriate native hosts; enumeration of a `cfg(windows)` mutant on Linux is not execution. [V04, V05, V08]

Strict warnings can make useful mutations unviable. Upstream documents mutation-only `--cap-lints=true`; qualify that separate instrumented subprocess with an external build timeout because lint capping also suppresses long-running const-evaluation warnings. Ordinary lint/release policy stays strict. Record effective flags and prove that an injected behavioral defect compiles and is caught. This is a test instrument, not a production lint waiver. [V06, V07]

### 9.5 Fuzzing and native memory diagnostics

Use pinned Linux x64 nightly fuzzing for the portable core initially: NTP/NTS-KE parsing, strict DTO/domain admission, IPC framing, persisted state and numeric conversions. Seed valid inputs, assert semantic outcomes, and bound input bytes, duration, per-input time, RSS, jobs and artifact storage. A proposed 60-second changed-target smoke run is only a quick regression budget; scheduled campaigns need their own owner/budget and cannot promise exhaustive exploration.

Preserve and replay every failure. A reject-all parser or a harness that swallows every panic is not a passing fuzz target. Relevant minimized cases must run on the actual native product target as well.

**Windows support requires careful wording.** The cargo-fuzz README still says Unix-like systems only, but the Rust Fuzz Book documents Windows/MSVC AddressSanitizer setup. Rust 1.99.1's x64 MSVC target source declares AddressSanitizer support, while sanitizer selection remains unstable. The sources do not justify either blanket Windows impossibility or claiming our stack already works. [V12–V16, V18, V22–V24]

Qualify Windows x64 MSVC nightly fuzz/ASan separately against exact Rust, cargo-fuzz/libfuzzer-sys, native compiler, linker, CRT and sanitizer runtime. Require a deliberately faulty local fixture to produce its expected diagnostic and a valid neighbor to run. Inspect Rust and relevant native instrumentation. Do not infer ARM64 support from x64, install unsupported stable flags, or bypass the channel using `RUSTC_BOOTSTRAP`.

#### 9.5.1 Adopted native memory workflow

Application Verifier, GFlags/PageHeap and CDB from Windows SDK **10.0.28000.2957** are installed for development. The [tooling record](../TOOLING.md#native-windows-memory-diagnostics) owns installation, commands and observed bootstrap results; [the native-memory skill reference](../.agents/skills/testing/references/windows-memory.md) owns the agent procedure. Exact tool, DLL, debugger-configuration and image-profile admission is defined in [the policy](../policy/windows-memory.json) and enforced by [the guarded helper](../scripts/windows-memory.ps1). Do not copy installation snapshots into an application pass claim.

Select this lane when changing unsafe Windows adapters, native buffer/string/callback lifetimes, handle ownership or cleanup, or investigating a native crash. Trace acquisition, transfer, use and release through success, partial initialization, cancellation and recovery; name the paths the test must reach. UI-M0 keeps its bounded native-window and FFI scope: a relevant memory check does not activate D1–D3, production integrations or broad soak requirements.

1. Freeze the candidate and its symbols/configuration, select an owned Windows fixture and unique image basename admitted by the helper, and declare test inputs, duration, memory/output limits, cleanup owner and stop conditions. Run read-only Preflight as the ordinary-user debugger account. Tool presence does not authorize arbitrary privileged or hostile target execution.
2. Enable the exact per-image verifier/full-PageHeap profile in the separate elevated configuration shell before starting the candidate. Run CDB and the ordinary test candidate unelevated with the admitted current-user debugger configuration and local symbols. Real service/privilege experiments still require section 10's separately owned native lab and intended service identity. System-wide GFlags settings, unrelated process names and arbitrary extra verifier layers are outside this profile.
3. Observe effective settings and reached cases. Retain the first verifier stop/exception, stack, raw log, native exits and healthy behavior. A fault caught by CDB can still yield exit zero; classify actual diagnostics and independently observed outcomes. On tool/profile or relevant allocator/API changes, replay healthy and deliberately faulty controls for the claim. Bootstrap heap-overrun and invalid-handle controls do not qualify every allocator, Leak layer or application path.
4. Stop and account for owned processes, then Disable and verify both IFEO registry views against the protected ownership receipt, including after failure or timeout. Do not overwrite unexpected state or declare cleanup complete after an ambiguous restoration. The helper configures an image; it does not launch, sandbox or supervise the candidate. Its configuration-command timeout and post-collection output check do not enforce the test's resource limits.

PageHeap observes the heap paths it instruments and changes allocation cost and timing. Its settings persist until explicitly removed. Application Verifier's Leak layer reports tracked resources at DLL unload; outstanding resources alone are not proof of a leak. Use the dedicated lifecycle measurements below for retention, and retain the independent ASan/Miri admission limits above. [Microsoft PageHeap documentation](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/gflags-and-pageheap), [Application Verifier layers](https://learn.microsoft.com/en-us/windows-hardware/drivers/devtest/application-verifier-tests-within-application-verifier).

**Low-resource simulation remains an unadmitted candidate.** The current policy
admits Basics/full PageHeap, not arbitrary fault layers; Microsoft's `/faults`
shortcut does not cover every native resource failure. A later named allocation,
handle or registration failure question needs an exact qualified profile,
healthy/fault controls, truthful service/effect observations and verified cleanup
through this existing workflow. The [research proposal](plans/windows11-integration.md#7-diagnose-actual-windows-behavior-and-inject-native-failures)
changes no tool policy and claims no executed low-resource coverage.

#### 9.5.2 Retention and resource ownership

For changes to long-lived state, queues/caches, native resources or task cancellation, identify each allocation, handle, thread/task and permit owner, its bound and its release condition. Rust ownership and garde validation do not establish bounded retention: reference cycles, intentionally forgotten allocations, unbounded collections and abandoned work need explicit lifecycle contracts and observations. Native work retains custody and capacity permits until actual completion or qualified recovery, even if its waiting future is dropped.

Measure declared workloads after warm-up and after each repeated lifecycle/quiescent checkpoint, before process exit or fixture teardown can mask growth. Record private/committed memory, relevant allocation attribution, handles, threads/tasks, queues and permits; include USER/GDI objects when UI ownership changes. Require useful healthy work and bounded behavior on failures/cancellation, comparing retained state with the owner's declared budget rather than treating every outstanding allocation or cached page as a leak. Record unsupported resource/allocator observations as gaps.

Keep corruption instrumentation separate from representative performance measurements: full PageHeap changes memory usage and timing. Retain CHECK-21's one-hour baseline and 72-hour reconnect/fault soak on named hardware for the applicable product stage; neither a short instrumented run nor nextest's process/output `leak-timeout` replaces those observations. No application leak/soak result exists merely because the tool controls passed.

### 9.6 Bounded use of Miri, Loom and Kani

| Tool | Good initial question | Claim boundary |
| --- | --- | --- |
| Miri | Does a suitable core/unsafe abstraction test execute UB under the interpreter? | Significant FFI/system-call limits; cannot establish real SCM, token, pipe, CNG, power or clock enforcement. [V17] |
| Loom | Can cancellation release a native-job permit before the real job finishes, or publish a stale generation? | Only modeled synchronization/dependencies and stated scheduling/memory bounds; production correspondence must be reviewed. [V19] |
| Kani | Can a bounded fixed-width conversion or cumulative-budget admission violate its invariant? | Standard installation is not native Windows; finite inputs/unwind limits and satisfiable assumptions define the proof. [V20, V21] |

Add a tool only when it addresses a concrete uncovered invariant. Keep interpreter isolation, model a legitimate reachable witness, inject a fault to obtain a counterexample, and record unsupported operations. Reducing a proof bound or excluding a failing FFI case reduces the claim; it does not prove the omitted behavior.

#### 9.6.1 Mandatory product mathematical proof gate — 5 November 2026

**Current policy, 6 November 2026:** [OD-10 §14.1](nevrotick-owner-decisions.md#141-first-release-deferral--6-november-2026)
defers formal proof work until after the first released software version (V1).
The proof-first instructions and receipts below are historical; missing proofs
are no V1 prerequisite, acceptance blocker or release gate. Do not execute formal
compiler/solver/proof jobs, extend verifiers or repair pending proof metadata
consumers before V1. Preserve exact historical proof scope and track remaining
obligations in the existing tasks for post-V1 bounded resumption. Continue the
ordinary product plan. Runtime/garde/private invariants, checked arithmetic,
independent behavioral/property/boundary tests, security, 90% coverage/95% mutation
within their required scopes, native/package/release gates and all existing
Windows/fuzz/storage/UI holds remain unchanged.

**Historical policy and receipts before the amendment:**

[OD-10](nevrotick-owner-decisions.md#14-od-10--mandatory-mathematical-proofs)
requires machine-checked proofs of product mathematical algorithms, as specified by
[architecture §20.1.1](windows-time-architecture.md#2011-mandatory-mathematical-proofs--5-november-2026).
Maintain a per-algorithm obligation inventory in the existing task under
CHECK-7/8/25 and CHECK-10 where applicable. Define the theorem independently of
the implementation: full admitted domain, required success/error behavior,
invariant preservation, checked machine operations, units, rounding/error bounds
and promised termination/work bounds. Constructor and caller assumptions need
identified owners and their own proof coverage; proving a consumer does not
prove its input admission or physical/trust assumptions.

Use the [formal-verification workflow](../.agents/skills/testing/references/formal-verification.md)
and an admitted tool/profile. Bind actual executable source and proof/model
correspondence, dependencies/configuration, compiler/verifier/solver and all
assumptions to the receipt. Require independently reviewed specifications,
nonempty completed checks, reachable controls, distinct compiled defects that
violate promised properties, and restored healthy passes. Preserve failures and
resource/attempt accounting; unsupported syntax, timeout, empty selection or
unsatisfied unwinding leaves the obligation incomplete. An unsupported verifier
may justify another bounded tool evaluation; it does not waive the proof gate.

The [5 November Core Verus pilot](../tasks/s1.3-source-selection-readiness.md#verus-addition-pilot--5-november-2026)
was such a conditional evaluation after the actual addition timeout and David's
request to use Verus if better. Its three-command readiness budget is exhausted:
the final command rejected required `u64::checked_shl` during VIR translation,
before SMT execution. The candidate is **DEFER**, with zero product-proof commands;
addition acceptance stays open. The independent Kani
[`ScaledValue::split` checkpoint](../tasks/s1.3-source-selection-readiness.md#split-terminal-qualification--5-november-2026)
qualified that full-raw-domain leaf. The
[6 November representation checkpoint](../tasks/s1.3-source-selection-readiness.md#representation-terminal-qualification--6-november-2026)
also qualified full raw-bit `from_bits` admission/exact Q33 decomposition and
raw-payload `signed_whole` magnitude/sign/fraction independence, with a complete
healthy/three-defect/restored group for each leaf. Addition, `EndpointParts::new`,
full enclosure, joint Garde admission/callers and Core/P1 remain open. Return to
this Verus route only after official
support for the unchanged API and a newly reviewed finite checkpoint. Preserve
completed Kani evidence, held extensions and the complete actual-body scope; use
no product assumptions, alternate executable body or narrowed domain to obtain a
pass. Broad tool/CI adoption remains deferred. No shipping compiler change or
execution-hold waiver follows.

The [6 November addition contract-mode assessment](../tasks/s1.3-source-selection-readiness.md#addition-contract-mode-terminal-assessment--6-november-2026)
is terminal **DEFER** after one primitive mechanism command: CBMC aborted with
status 134 on an internal closure type inconsistency and exported empty checks.
Zero product-proof commands or callee certificates followed. The checkpoint
stopped without retries, tool patches or adoption. Ordinary Kani proofs retain
their unchanged scopes and the full mathematical obligations remain.

The [ordinary partition assessment](../tasks/s1.3-source-selection-readiness.md#ordinary-addition-partition-terminal-assessment--6-november-2026)
is terminal **DEFER**. The union and cases 0–2 completed 631 checks: 575 `Success`,
22 satisfied covers and 34 accounted unreachable checks. Case 3 timed out at
60,018 ms with empty checks; later leaves, fault controls and restorations did not
run. These four completed commands retain their individual scopes.

The [full-addition compute-budget assessment](../tasks/s1.3-source-selection-readiness.md#full-addition-compute-budget-terminal-assessment--6-november-2026)
is terminal **DEFER**: its first healthy command timed out at 300,011 ms with zero
checks; negative controls and restoration did not run. All 37 source inputs, the
60-entry snapshot and tool/solver identities remained intact. The larger budget
leaves addition unproved; valid ordinary proofs retain their scopes and all holds
remain. This card is closed, with no automatic new profile, retry or SDK patch.

The [manual composition assessment](../tasks/s1.3-source-selection-readiness.md#manual-composition-terminal-assessment--6-november-2026)
is terminal **DEFER / incomplete**: the first 13 commands supplied scoped
actual-callee Post/actual-return seed proofs and controls; addition command 14
timed out at 60,016 ms with zero checks, and commands 15–18 did not run. During
that command an audit imported frozen Python without suppressing bytecode writes,
adding four `.pyc` files to the source island. All 45 bound sources, addition
inputs and three earlier certificate receipts remained unchanged; retain only
the earlier lemma scopes. The reviewed pure-composition rule and compiler name
resolution remain explicit trusted boundaries. Preserve the raw receipts and
closed-card failure: no addition adoption, certificate import or retry. Its
incident remains preserved alongside the subsequent query-split checkpoint.

The [query-split terminal assessment](../tasks/s1.3-source-selection-readiness.md#fraction-query-split-terminal-assessment--6-november-2026)
is **DEFER / incomplete** after 14 of 20 commands: the fresh 13-command prefix
supplied only scoped actual-callee Post/seed premises and controls. Universal
command 14 timed out at 60,018 ms with zero checks; commands 15–20, including all
direct-witness and restored lanes, did not run. All 49 source files, 14 complete
input/permission bookends, three certificate seals and tool identity passed final
custody checks; observer bytecode suppression and read-only input protection
prevented recurrence of the earlier incident. Preserve both raw bundles. Solver
query reductions prove no universal property or addition/Core/P1 acceptance.
The subsequent
[pure-bridge card](../tasks/s1.3-source-selection-readiness.md#fraction-verus-bridge-terminal-assessment--6-november-2026)
closed DEFER after one healthy command found seven private-`open` annotation
errors before SMT. Zero jobs qualified; semantic controls/restoration did not run.
Its exact seven-modifier source-only correction passes fourteen source checks.
The later [private-spec v2 card](../tasks/s1.3-source-selection-readiness.md#private-spec-verus-bridge-v2-terminal-assessment--6-november-2026)
reached Z3 without VIR errors but closed DEFER after five failed equalities.
Constructor Post's success remains conditional on failed helpers; three Clone
bodies also succeeded. Zero jobs qualified, and controls/restoration did not run.
The complete six-equality bridge remains incomplete. The subsequent
[bit-length dependency campaign](../tasks/s1.3-source-selection-readiness.md#bit-length-dependency-terminal-assessment--6-november-2026)
completed five jobs: full-raw lemma/two predicate equalities, three intended
contract defects and restored health. All62 focused Python checks pass;22 source
files, exact executable erasure and final source/input/receipt/tool bookends are
bound. Reuse unchanged scoped proofs for remaining arithmetic/representation
work. Complete bridge, native Ordering correspondence/import and actual addition
remain open; no broader Core/P1 qualification follows.
The subsequent [constructor-Post campaign](../tasks/s1.3-source-selection-readiness.md#constructor-post-terminal-assessment--6-november-2026)
completed five jobs with seven exact function results each: full-raw Post/
lemma/helpers, three isolated Post defects and restored health. All93 focused
Python checks pass;24 source files, exact erasure and final custody/tool bookends
are bound. Both earlier Rust outputs and frozen proofs remain preserved.
No checked-Result premise or imported product certificate was added. Constructor
execution/caller integration remained separate. The later
[complement v2 qualification](../tasks/s1.3-source-selection-readiness.md#complement-v2-terminal-qualification--6-november-2026)
passed seven jobs with ten exact function identities: four full-raw predicate
equalities/three arithmetic lemmas, five contract failures and restoration.
All143 selected checks pass;30 source files/11 executed-Python rows and final
guards bind the candidate. V1's pre-SMT failure remains historical. Subsequent
[rational-order qualification](../tasks/s1.3-source-selection-readiness.md#rational-order-terminal-qualification--6-november-2026)
passed nineteen jobs: five full-raw predicate equalities/seven total lemmas,
seventeen intended defects and restored health. All161 selected Python checks
pass;33 sources/11 executed-Python records and final guards bind its candidate.
Source inspection confirms missing pinned checked-shift support. Native Ordering
equality/comparison Post, caller and full bridge/addition/Core/P1 stay open.
Independent [RequestTiming qualification](../tasks/s1.1-exchange-association.md#request-timing-terminal-qualification--6-november-2026)
passed five full raw-pair/three-defect/restored Kani jobs and158 Python checks;
34 inputs/nine executed-Python records and final guards bind the candidate.
Safe owned target cleanup preserves raw evidence and low-reserve failure status.
Subsequent [actual append_depth qualification](../tasks/s1.3-source-selection-readiness.md#selection-append-depth-terminal-qualification--6-november-2026)
passed six full raw-state/four-defect/restored jobs,671 checks and208 Python
checks;34 inputs/nine actual Python records and final guards bind the candidate.
The subsequent [actual lifecycle ordinal proof](../tasks/s1.3-source-selection-readiness.md#lifecycle-ordinal-terminal-qualification--6-november-2026)
passed five full raw-u64/three-defect/restored jobs,263 checks and248 Python
checks;38 inputs/nine executed-Python records and complete23-enum closure bind
the actual successor/exhaustion theorem. Subsequent [actual source-counter
qualification](../tasks/s1.3-source-selection-readiness.md#lifecycle-source-counter-terminal-qualification--6-november-2026)
passed five full raw2^80/three-defect/restored jobs,145 properties and291 Python
checks;32 inputs/nine executed-Python records bind the actual constants/predicate/
From/min. The shared consumer uses exactly four immutable captured-wrapper-
selected roles and strict per-role cleanup; all five source-counter targets were
removed, retaining about4.35MiB including directories. The later [constructor
source checkpoint](../tasks/s1.3-source-selection-readiness.md#source-state-constructor-source-checkpoint--6-november-2026)
prepares genuine Garde/callback/constructor/Cargo source, authenticates14 selected
vendor packages and passes51 source checks plus independent review. No runner,
tool admission, compiler/proof or private-patch adoption follows. The cache is
unsealed; source-specific sealing/admission, target features, allocator/drop/
unwinding and constructor theorem stay open. Transactional
lifecycle, prefix/depth/topology/whole-selection and native authority remain
separate obligations.
The existing no-cheating gate rejects `external_body`;
no profile weakening, SDK patch, native flags or adoption follows. The old kit,
ordinary defaults and all holds remain unchanged.

Reuse unchanged evidence and run required affected proofs at stable candidate
checkpoints. The [S1.1c rounding/era handoff](../tasks/s1.1-time-domain.md#mandatory-proof-continuation-and-remote-integration--5-november-2026)
owns the first campaign; other domain, leap, exchange and Core mathematics remain
open until their own proofs pass. Keep shipping compiler checks, runtime guards,
independent tests, fuzzing, per-file coverage/mutation and native/package evidence.
Mandatory algorithm proofs do not adopt a new tool globally or change OD-05
§7.5's optional release-build provenance.

The [S2.1 bridge inventory](../tasks/engine-transport-pilot.md#independent-controls-and-proof-obligations)
routes capture/conversion, era/exchange, leap and later uncertainty/Core
mathematics to those existing owners. Its concrete controls are unexecuted
expectations, not proof receipts; physical timestamp placement, continuity and
native completion retain separate evidence requirements.

### 9.7 Verification of upstream code and stress testing

Chrony and every other upstream codebase, crate, native library, tool, copied/ported implementation and generated component must be treated as potentially containing defects or vulnerabilities. Reputation, prior deployment, upstream tests, signatures and absent advisories do not exempt adopted behavior from independent verification. This is a verification requirement, not a claim that a particular upstream revision has a known defect. Apply [architecture sections 20.1 and 20.3](windows-time-architecture.md#201-independent-numerical-truth) to the exact revision, selected features, supported targets and actual NevroTick integration.

For each adopted component, record the relied-on requirements, trust/resource boundaries, independent expected results and applicable test families. Exercise production entry points and retained upstream algorithms as well as NevroTick adapters; testing only a wrapper is insufficient evidence for algorithm correctness. Reuse upstream suites and reference trajectories, but keep independently authored expectations so shared defects can be detected.

| Test family | Required question where applicable |
| --- | --- |
| Unit, boundary and property/model tests | Do valid cases, exact limits, arithmetic extremes and state transitions satisfy an independent contract, including combinations that ordinary examples miss? |
| Differential and golden-vector tests | Does the implementation match independently justified protocol/numerical facts and the intended upstream behavior? Diagnose disagreements; upstream parity alone cannot settle correctness. |
| Fuzzing, adversarial and negative tests | Do malformed, valid-shaped hostile, replayed, stale and oversized inputs preserve authority and resource limits, while valid neighbors still succeed? |
| Integration, interoperability and native tests | Does the selected code behave correctly through real module/process/protocol boundaries and supported Windows APIs, features and artifacts? |
| Concurrency, fault injection and recovery | Do cancellation, restart, suspend/resume, lost acknowledgements and unavailable/exhausted collaborators preserve ownership, cleanup and safe recovery under relevant interleavings? |
| Load, stress and soak tests | Do steady and burst workloads, limits and controlled overload, repeated faults and prolonged operation preserve correctness, measured budgets, responsiveness and recovery? |
| Regression, coverage and mutation checks | Do minimized failures remain reproducible, critical paths execute and independently specified assertions detect injected behavioral defects? Preserve the existing per-module/file thresholds and denominator rules. |

Stress campaigns must include relevant mixes of legitimate and malformed traffic, peer/client churn, loss/delay/reordering/replay, reconnect storms, extreme clock/drift conditions and repeated lifecycle events. Exercise resource exhaustion in owned fixtures. Declare workload, concurrency, duration, seeds, resource ceilings, stop conditions and external timeouts before execution. Measure CPU, memory, wakeups, handles/threads, queue/log growth, latency and recovery; observe forbidden clock/security effects and leaks before harness cleanup. Retain CHECK-21's one-hour baseline and 72-hour reconnect/fault soak on named hardware; do not substitute a short smoke or report only that nothing crashed.

Select techniques according to failure modes and supported environments; this does not mandate every technique for every file or authorize effects on shared hosts or public services. Preserve healthy controls, exact source/build/feature identities, first failures, seeds and minimized regression cases. A confirmed upstream defect requires a requirement-grounded correction and port/dependency-ledger entry, not weakened expectations to preserve parity. Reassess affected tests and stress cases after upstream fixes, version/feature changes or integration changes. Unsupported or unexecuted required cases remain gaps; test success never establishes absence of all bugs or vulnerabilities.

### 9.8 Diagnostics and shared-adapter verification

Implement the canonical [shared Windows adapter contract](windows-time-architecture.md#34-shared-windows-adapters-and-diagnostics) and [diagnostics contract](windows-time-architecture.md#123-logging-and-support). Inspect real emitters, subscribers, filters, writer queues and export consumers; unused safe wrappers/redaction helpers prove no reached path. These are application implementation gates, not checks performed by this documentation amendment.

| Check family | Independent expected observation |
| --- | --- |
| Required versus optional events | With optional output filtered to WARN/ERROR/OFF in the harness, an INFO-severity required event still reaches its required sink attempt. Exercise release feature/callsite/global-filter behavior and admitted dependency/log bridges; no duplicate records or compiled-out promised DEBUG. |
| Privacy and usefulness | Recognizable synthetic secrets through success, rejection, dependency errors, panic/report paths and export appear in no selected ordinary output at any enabled level. Bounded legitimate context remains useful. Verify actual field/counter/text encoding; raw dump/EVTX policy remains separate. |
| Privileged failure-side writes; CHECK-13/15/17/23 | With ST-W03 and W11-W05/08/10, trigger a harmless principal-operation failure with a substituted service-writable parent/leaf used by error, product-owned crash, rollback or cleanup reporting. An unrelated sentinel remains unchanged; no fallback opens an unadmitted destination. Pair with useful diagnostics through an admitted sink and a legitimate operation. Cover reached create/append/rotate/replace/delete/permission paths and preserve bounded sink-loss behavior. |
| Control authority | A permitted administrator can start/stop scoped service collection; ordinary status/refresh users, stale revisions, malformed profiles, arbitrary paths/filter strings and neighboring unauthorized commands have no protected effect. Native tokens/peer identity remain CHECK-2 evidence. |
| Session state and expiry | Replayed starts do not renew; stale stops/replies cannot alter a successor. A helper exits after acknowledged admission without ending collection; UI closure does not renew it. Test partial activation, lost acknowledgements, broker-to-target control loss, broker/target restart, authorization revocation, duration/byte caps, UTC jumps and suspend/resume. Local expiry and failed-reset guard stop verbose capture; status never reports unacknowledged global success. |
| Clock independence | A verbosity-only change leaves the active source policy, estimator, valid correction and cumulative allowance unchanged. Log failure cannot authorize a request or block required cleanup. |
| Resource exhaustion and lifecycle | Concurrent component writers respect aggregate file allocations and bounded record/memory/queue/rate limits. Full/missing/denied sinks, floods, rotation/flush failures and stop/crash paths yield bounded loss/status without recursion, critical-path I/O waits or leaked native work. Include a healthy-event control. |
| Unattended failures and crash policy | With W11-W05, observe bounded selected critical-error paths during startup/work/stop beside ordinary start and useful safe diagnostics. Record inherited/process/thread policy and establishment phase separately from WER invocation, UI, queue/upload and LocalDumps/debugger policy. Observe no modal wait and truthful required-event availability/loss; a no-UI result establishes neither dump nor upload confidentiality. Reuse W11-W08/ST-W07 synthetic-secret cases, without altering global administrator policy. |
| Windows and feature boundaries | Verify actual provider/channel resource identity, access and retention under section 15.3; required/success and denied/failure paths on Windows. Inspect per-binary features/imports and correct resource-specific cleanup: sharing an adapter must not add GUI/TLS/clock-write composition to unrelated roles. |
| Optional trace/counter evidence | ETW timing/lifecycle traces retain monotonic correlation and loss counts; absent events in a lossy trace cannot prove absence of effects. If PerfLib V2 is selected, qualified aggregate counters preserve trust/freshness semantics, privacy and bounded overhead; coarse counter collection is not profiling. |

Tie results to the exact candidate, enabled features, process roles and declared finite limits. Pure tests exercise records, mappings, bounded state transitions and fake sinks; native Windows tests establish pipe/token/SCM, callback lifetime and Event Log behavior. CHECK-21 measures INFO and bounded DEBUG cost under supported host security, including flood pressure and disabled-callsite overhead. Reuse CHECK-1/2/3/13/14/15/17/21/23/26/27 instead of creating parallel gate identities. Inspect cleanup before the harness hides leaks, and preserve unavailable/native cases as gaps. This work does not enable production diagnostics in UI-M0.

### 9.9 IPC identity, framing and native lifetime verification

Apply [architecture section 5.4](windows-time-architecture.md#54-native-ipc-implementation-and-qualification) with `security-local-ipc`, `implement-rust-contracts`, `qa-contract-tests` and `qa-security-boundaries`. Freeze endpoint roles, selected byte/message mode, actual native API/wrapper path, security descriptors, token tracking, process/endpoint generations, finite limits and feature/lockfile identity before qualification. Bind each case to the production entry point and an independent expected effect; the existence of upstream tests proves no product behavior.

| Area and checks | Hostile/fault case and required result | Legitimate control |
| --- | --- | --- |
| Listener ownership — CHECK-2/27 | Pre-created name, unauthorized extra instance, idle/disconnect storms, replacement-accept failure and last-handle loss cannot silently transfer the active namespace; lost ownership blocks admission/reacquires through first-instance checks | Concurrent/repeated permitted connections and later genuine restart succeed |
| Genuine peers — CHECK-1/2 | Same owner SID with a foreign instance, reused/stale PID, SCM pending-state ambiguity, wrong service, anonymous caller and failed identity query produce no privileged effect or trusted status | Genuine current service and intended client identity are established through held native objects |
| Protected application admission — CHECK-1/2/15/18 | Same effective principal with a foreign application, copied/unapproved component, unrelated valid signature/package, failed identity query or replacement during acquisition cannot obtain section 4.2.1 protected admission. Challenge inherited/duplicated handles, injection and proxying against the exact claimed boundary; an admitted compromised component still cannot exceed its finite rights | Intended current installed component and authorized actor complete the permitted verb; existing public status/argument-free refresh remain usable |
| Management routes and prior authority — CHECK-1/2/13/23/27 | Ordinary client cannot gain SCM lifecycle/configuration/delete or security-descriptor mutation rights, or use custom controls to bypass product IPC. Observe prior handles across DACL tightening; no immediate-revocation claim. Separate remote pipe rejection from administrative remote SCM policy | Exact query rights needed for server verification work; deliberate elevated maintenance/recovery succeeds, including unavailable UI/normal service; current ownership and successor protections remain enforced |
| Request/token association — CHECK-2/25 | Fragmented/pipelined reads and changed security context cannot attribute one request to another caller; token acquisition fails without ambient/self fallback; no await/dispatch occurs while impersonating | Authorized identification-level client works, and the worker has reverted before processing the next request |
| Effective authority and reversion — CHECK-2/12 | Filtered admin, disabled/deny-only group, restricted token and wrong service SID fail their prohibited verbs; injected failed reversion stops process request execution and uses qualified external crash recovery | Current elevated/authorized request and healthy reversion succeed; owned recovery preserves clock limits |
| Wrapper configuration — CHECK-1/2/26 | Generic-rights/default-descriptor mismatch cannot cause ACL widening; unauthorized client cannot create instances, alter ownership/security or inherit privileged handles | Exact reviewed requested rights, DACL and QoS work together through actual selected APIs |
| Framing and overload — CHECK-3/25 | Cancellation at header/body/write boundaries, EOF, duplicate prefix, trickled bytes, stalled receiver and queue saturation preserve framing/correlation or close the connection; fixed deadlines/caps prevent indefinite retention or starvation | Valid maximum-size frames and permitted concurrent requests complete; late/partial replies never repeat an ambiguous clock effect |
| Native completion — CHECK-3/27 | Pending connects/reads/writes, cancellation-versus-success races and stop/disconnect retain operation storage/permits until actual completion; no unbounded write flush, replacement work or cleanup blockage | Correct connected/pending/partial native outcomes and post-recovery traffic succeed; resources release before harness teardown |
| Reached remote boundary — CHECK-2 | Controlled remote client with otherwise sufficient credentials is rejected by local-only pipe enforcement; firewall rejection or an unreachable peer alone is not evidence | Intended local request with matching product permissions succeeds |
| Compromised authorized engine — CHECK-3/25 | Actual engine identity submits oversize/truncated frames, wrong verbs, stale/replayed and overbudget proposals; bounds apply before forbidden allocation/effect | Current bounded engine request still receives the proper acknowledgement |

Pure tests can establish framing progress, strict decoding and substituted-token decisions. Tokens/DACLs, actual pipe peer attribution, pending-instance ownership, native cancellation and fatal reversion need isolated Windows fixtures. Real clock/fatal-broker cases obey section 14.9's disposable-VM limits and external observation/restoration plan; no experiments on the user's workstation or public services. Keep exact source/configuration, reached boundary, native outcomes, valid controls and remaining P0/native gaps. Do not call IPC hardened until the affected proofs pass; UI-M0 remains fixture-only.

### 9.10 Contract-review qualification matrix

The [30 October assessment](supporting-reviews/architecture-contract-review.md)
refines existing requirements; no rows below are executed product evidence.
Apply [architecture §20.2.1](windows-time-architecture.md#2021-contract-review-acceptance-refinements)
and freeze each policy/representation before its dependent interface is enabled.

| Owner and gate | Required independent evidence |
| --- | --- |
| Platform/security, S0.2/P3, CHECK-1/15 | Actual default owner, descriptor/inheritance and effective rights after runtime file creation/staging/replacement and relevant object creation. A second LocalService service cannot use owner rights to widen access; intended service access succeeds. |
| Broker/storage, S0.4/P4, CHECK-10/12/15 | Reservation write/flush → guarded clock call → outcome write/flush → acknowledgement. Inject faults on both sides of every seam, replay the same identity and inspect durable ledger plus external effects. Unknown outcomes stay conservatively charged; no duplicate step or refund from missing reply. Slew reservation, delivery and settlement remain separate. |
| Clock adapter, S0.4/P4, CHECK-10/11/12 | Read back mode and value and measure trajectories for owned base, owned transient and explicit handover. Cancellation/holdover/expiry retains the qualified owned mode; foreign/ambiguous ownership prevents resets. |
| Protocol/security, S0.7/P2, CHECK-8/9/15 | Fresh UDP/cookies/resumed tickets cannot reset full-authentication age. Exercise expired/unknown association age, failed full reauthentication, trust-policy changes and healthy full authentication under frozen lifetime/detection rules. |
| Core/security, S0.7/P2/P4, CHECK-9/15 | Wrong but intact persisted floor, followed by correct independent evidence, reaches the qualified narrowly authorized supersession. Preserve old record, budgets, step counts and installation ownership; unauthorized reset is rejected. |
| Core/protocol, P1/P2, CHECK-7/8/14 | Freeze independent groups, configured/eligible vote denominator, timeouts, fresh observation limits and exceptional quorum. Four fresh, two missing, duplicate operators, 2–2 disagreement and one fresh plus old predictors produce independently expected decisions. |
| Platform/installer, S0.4/P4/P6, CHECK-11/12/13/23 | Distinguish cold boot/restart, sleep/hibernate, Fast Startup and VM restore epochs. Queued SCM restarts during maintenance/re-enable do not bypass durable startup inhibition or silently replenish allowances. Restore only owned settings. |
| Tooling/QA, P1/P7, CHECK-19/25 | Mixed safe and unsafe/generated critical behavior proves that generated-mutant accounting cannot conceal missing functions/guards. Retain the required alternative behavioral evidence and unchanged 95% threshold. |
| Release/security, S0.6/P6, CHECK-9/18/23 | Execute the [signing matrix](windows-time-signing-guide.md#51-wrong-clock-and-offline-activation-matrix): installed startup/helper, first install, update/repair and uninstall/restoration under wrong dates, offline/stale caches and bounded native verification. Enforce key policy and project-specific identity independently of signature success/publisher text. |

Pure fixtures can qualify transition logic and fault-seam expectations. Filesystem
durability, actual token/owner access, SCM, clock mode/trajectory and signing
retrieval require supported Windows observations in owned disposable fixtures.
No production TTL, quorum or recovery allowance is supplied by this matrix.

### 9.11 Windows integration qualification refinements

The [3 November Microsoft-source review](plans/windows11-integration.md) and
[agent task](../tasks/windows11-integration.md) route these future cases to
existing owners. Each selected candidate needs a healthy control alongside its
failure case. These are planned refinements, not native results, new release
gates or permission to execute paused experiments.

The [S2.1 provenance preparation](../tasks/windows11-integration.md#426-s21-timestamp-provenance-preparation--5-november-2026)
is partial W11-W01 source work. Preserve application-call placement, mapping
uncertainty and socket-level oversize accounting without attributing an error to
an unreturned peer identity. The selected SDK/backend, finite mapping profile and
actual native custody controls remain open under the existing checks below.

The [4 November leap policy](plans/leap-second-policy.md#6-windows-contract-and-limits)
adds a selected S0.4/P4 contract under W11-W02, with pure prerequisites in
[S1.4](../tasks/s1.4-leap-policy.md) after S1.3. The kernel alone executes explicit
events; v1 qualifies default compatibility presentation and ordinary residual
correction. No production global leap changes, event injection or runtime `w32tm`
parser are admitted; separate synthetic fixtures retain S0.4's bounded lab rules.
Source documentation and process readback do not establish the kernel's
exact event knowledge or a native pass.

| Existing owner / checks | Independent observation at the relevant stage |
| --- | --- |
| Time/network adapter; CHECK-6/11/21/27 | For optional packet timestamps, distinguish actual software/NIC source and RX/TX interface attribution on two adapters. Route/socket/counter changes, stale or missing TX records and buffer saturation cannot publish a falsely precise current sample; the qualified application-boundary fallback remains usable. |
| Platform/clock; CHECK-10/11/12/27 | Observe selected suspend/Modern Standby and delayed-timer paths without assuming every service suspends. Sleep-excluding clock progress is not proof the broker executed cleanup; generations, residual rate and uncertain cleanup remain honest. Healthy wake/reacquisition works without replenishing authority. |
| Windows mapping; S0.4/P4, CHECK-10/11/27 | Record/verify engine and broker default process presentation through supported APIs; do not opt into `:60`. Challenge changed/unsupported mode, OS-known history and conversion ranges against healthy controls. Keep FILETIME/UTC/NTP distinct, preserve sub-millisecond precision outside guards and invalidate changed mappings; no fixed epoch shortcut or universal 37-second adjustment. |
| Leap guard/recovery; S0.4/P4, CHECK-10/11/12/27 | Exercise both signs with OS-handled, missing and unexpected events and healthy all-LI=0 sources. Observe actual cancellation/base rate, trajectories and durable outcomes through guard and sleep/restart. No synthetic ±1-second act or broker acknowledgement for a kernel event; fresh post-guard quorum corrects only measured residual under existing budgets. Missing native knowledge limits exact event-time UTC, while future-calendar uncertainty alone permits qualified tracking outside guards. |
| TLS/trust/native crypto; OPEN-8/CRYPTO-04, CHECK-8/9/26/27 | Apply [R-05's retrieval refinement](nevrotick-research-handoff.md#7-r-05--windows-trust-with-a-wrong-system-date) in the actual service/verifier context. Pair cold/stale caches, wrong dates and prohibited retrieval with an admitted healthy path; distinguish configured cache/revocation-timeout scope from observed direct/delegated effects and actual job completion. Existing CNG initialization/shutdown cases retain their own evidence scope. |
| Installer/security; CHECK-1/9/16/23/27 | If WSH is selected, preserve the admitted service SID profile and prove permitted/denied networking under actual identities, both address families, rule delay, delegated work and managed conflicts. CHECK-9 applies to delegated certificate retrieval, not to a new general firewall gate. Configuration success or a single process firewall rule is insufficient. Custom WFP needs a demonstrated remaining gap. |
| Service/installer; CHECK-1/11/12/13/23/27 | Distinguish configured settings, documented service-start/system-start activation and observed effective state, including first install. Boot/manual/SCM/Restart Manager starts cannot bypass durable maintenance admission; commit/rollback work with RM enabled or unavailable. An optional preshutdown budget expires truthfully without waiting for optional cache writes. |
| Service startup; CHECK-1/2/13/15/27 | Apply architecture §12.2 to process arguments, transient SCM arguments and durable policy. Ordinary installed start and recovery work without prior transient options; conflicting/malformed/excessive input cannot change protected role, endpoint, root or maintenance/clock authority. Verify actual wrapper decoding, not an assumed callback vector. Inventory actual environment/profile/path inputs and no-login failures; reuse SEC-A02 for live temporary files. |
| Product coordination; CHECK-1/5/12/13/15/27 | Under architecture §11.1, pair trusted creation/open and intended cross-session sharing with precreation, wrong type and denied access. Test both actual abandoned ownership and fresh recreation after last-handle loss; reconcile durable state, consumed allowance and current controller before effects. Verify native-thread ownership and separate task serialization. Refusal alone proves no reservation availability or clock authority. |
| Reached startup dependencies; CHECK-13/21/27 | If policy/installer/SCM/provider waits occur, challenge the complete reachable wait graph beside a healthy ordering. Observe bounded progress/failure and valid recovery after release; no fake progress, premature Running or protected effect with unknown required policy. The pinned Tailscale example is conditional source evidence, not a selected NevroTick locking scheme. |
| Resource/crypto owners; CHECK-21/26/27 and existing DNS-C owners | Select power/cost/memory hints, scoped EcoQoS or CNG SIMD batching only for a measured workload. Pressure, unknown cost, small loads and adversarial work preserve hard caps, foreground latency, crypto truth and completion custody; batch SIMD is distinct from bounded multicore scheduling. |
| UI/service; CHECK-14/17/22 at D2/D3 re-entry | Build-supported UIA/local notifications reflect current state obtained through authenticated service communication, distinguish time-authentication/readiness states, deduplicate meaningful changes and revalidate actions. Actual Narrator/deployment behavior remains to be qualified; no UI-M0 integration or package identity follows from this candidate. |

The [second Microsoft-source pass](plans/windows11-integration-follow-up.md)
adds the following distinguishing cases when their existing adapter is selected:

| Existing owner / checks | Refinement and healthy control |
| --- | --- |
| Timestamp/socket adapter; CHECK-6/11/21/27, CHECK-25 for affected construction | Stable-route timestamp-mode changes and queued old metadata cannot mislabel samples. Payload/control truncation, stale reused outputs and dual-stack IPv4/IPv6 metadata are checked separately; valid neighboring inputs still work. Direct-overlapped immediate/pending/close races retain exactly one terminal settlement and real custody without assuming a Tokio-future drop calls native cancellation. |
| COM adapter; CHECK-16/27 | Correct apartment and allowed activation work under the final profile; incompatible apartment, late process-security initialization, cross-thread interface misuse and callback/reentrant teardown fail or drain under the declared contract. No generic COM framework or wider token is inferred. |
| Service observer; CHECK-13/14/23/27 | Select the subscription or APC contract explicitly. Healthy new-instance reconciliation works; zero-mask/churn/delete-recreate and callback-active stop cannot deadlock, retain uninstall-blocking handles or publish stale readiness. SCM Running is not authenticated application readiness. |
| Installer/filesystem; CHECK-1/13/15/23 | If the owner later selects the retained MSI route, a selected declarative MSI descriptor secures owned resources under actual account/SID ordering. Verify inheritance and runtime replacement with allowed and neighboring-denied access; conflicting/invalid authoring fails usefully. In every package route, service-deletion state loss cannot erase mandatory restoration or consumed authority. |
| Diagnostics; CHECK-1/17/21/27 | With two ETW controllers, expire product authorization while another keeps the provider enabled; capture-state respects field/resource admission and required events remain useful. Observe permitted/denied provider/session/ETL access separately; local emission expiry proves neither exclusive-session delivery nor deletion of captured data. |

Pin the selected SDK/generated bindings and reconcile the follow-up's inconsistent
timestamp-unregistration return prose and ancillary-header field definitions
before FFI implementation. Full-page retrieval through MCP is source evidence,
not ABI/build qualification. The actual I/O-wait timeout, unattended resume and
adjacent cross-thread QPC cases belong to existing W11-W01/02 timing checks.

For the **selected complete hardened MSIX**, the [Store plan](plans/windows-msix-store.md)
and [W11-W10 assignment](../tasks/windows11-integration.md#48-owner-selected-hardened-msix--3-november-2026)
reuse the existing checks below. All native cases remain planned and held. Source
preparation and capability justification need no speculative Store preapproval;
David owns the initial submission and the Store decides acceptance. Local
test-signed mechanics precede submission as applicable; final Store-artifact
qualification follows signing and precedes broad release.

| Existing owner / checks | Required independent observation and valid control |
| --- | --- |
| Package/security; CHECK-16/18/23/26 | Inspect the final manifest's explicit `uap10:PackageIntegrity` / `uap10:Content Enforcement="on"` and absence of executable mutable/external content. Challenge the real launch/load paths with tampering and writable-path canaries; intact installed content and supported repair still work. Use [plan section 3.1](plans/windows-msix-store.md#31-package-integrity-and-sealed-executable-content); a declaration alone is not runtime proof. |
| Service/provisioning; CHECK-1/5/23/27 | Verify actual supported least-privilege accounts, service SIDs, privileges and allowed/denied object access at first install and boot before UI. Normal administrative consent remains required; no external service prerequisite, default LocalSystem expansion or assumed virtual-account bridge. Valid provisioning and authorized service startup must succeed. |
| Process/security; CHECK-16/18/20/23/26 | Read back the required per-role profiles under actual SCM, packaged UI and helper activation. Pair Store-compatible CIG and other denial cases with expected system/Store DLL loads and valid workflows, including cold/lazy provider paths. Use [section 6.4](#64-per-process-mitigation-qualification) and plan sections 3.2–3.3; strict Microsoft-only and Store-allowing CIG are distinct. |
| Helper/IPC identity; CHECK-1/2/15/18 | Reject an unrelated valid Store-signed package, copied-out helper or unapproved release for NevroTick authority beside the intended current installed helper. Apply [management admission](windows-time-architecture.md#421-service-management-admission-and-administrative-recovery) to the actual connected peer and effective actor, including same-user foreign applications, acquisition/restart ambiguity and admitted-process abuse. A signer-class pass is not product authorization; use actual package/catalog, separately required per-file signatures and protected identity. Preserve deliberate administrative recovery and public-operation policy. |
| Lifecycle/recovery; CHECK-5/12/13/15/23/27 | Observe ordinary Settings removal, Reset/repair, interrupted update and last-user transitions before UI launch. Mandatory recovery completes before its last journal or executable disappears; a replaced package or deleted service is not recovery evidence. Valid maintenance preserves ownership, restores W32Time as required and supports a corrective higher-version release without stale successor overwrite. |
| Release/inventory; CHECK-18/23/26 | Preserve submitted and Store-delivered package/catalog/manifest/payload identities, every required per-file signature under guide §4.1 and the validated SBOM. Qualify required inner-signature preservation and full format/nesting coverage on the actual delivered artifact; repeat affected readbacks, loaded-image and negative/valid controls. Test signing or a clean source build cannot substitute. |
| Artifact content trust; CHECK-18/23/26, CHECK-25 for reached consumers | Apply [architecture section 19.6](windows-time-architecture.md#196-authenticated-artifact-content-and-consumer-trust) and [signing guide section 5.2](windows-time-signing-guide.md#52-pe-certificate-padding-and-exact-final-bytes): record actual signature source/effective policy, applicable padding rejection and approved complete-file custody; excluded application data cannot authorize a command, destination or payload. Pair harmless changed-content cases with healthy admission; keep consumer and native verifier evidence separate under the existing W11-W10/S6.4 owner. |
| Compiled leap knowledge; CHECK-18/23/26 | Reproduce normalized calendar/policy generation from reviewed pinned inputs and bind its revision/digest and generator identity to the complete authenticated release. A mismatched or corrupt replacement cannot erase the previous admitted release or protected correction continuity. A valid bundle with unknown future coverage remains distinct from an integrity failure; no mutable runtime file, network build fetch, downloader or new trust root is introduced. |
| Activation/trust; CHECK-9/18/23 | Apply [signing section 5.1](windows-time-signing-guide.md#51-wrong-clock-and-offline-activation-matrix) to initial installation, installed startup, helper activation, maintenance and recovery, with bounded retrieval and absent/stale caches. Valid offline/wrong-clock recovery must coexist with rejection of unverified code; no weakened verifier or unconditional trust. |

The [5 November provisioning refinement](plans/windows-msix-store.md#75-pre-start-provisioning-authority-and-admission--5-november-2026)
orders the service/provisioning observations above. Before setters, query actual
deployment-created identity, raw service type and security, then open/close only
the proposed actor's required handles; descriptor rights are conditional on its
actual operations. No start/stop call belongs to that first probe. Separately
prove supported no-UI invocation/reentry. Later controlled management start needs
creation-time process/thread/token protection already established, then fresh
lifetime-bound admission before normal work. Qualify runtime owner/defaults and
replacement objects separately, reject stale process evidence, and distinguish
update from same-version repair/re-registration. W11-W05/W11-W10 and their
existing CHECK owners retain these planned cases; the separate removal pair,
current execution holds and optional enhanced-provenance policy are unchanged.

For recovery, [architecture §11.2](windows-time-architecture.md#112-upgrade-rollback-and-uninstall)
and [MSIX plan §7.6](plans/windows-msix-store.md#76-independent-removal-with-outstanding-recovery--5-november-2026)
govern the preliminary held inert pair: begin both removal cases with the same
durable dummy obligation pending, then observe fresh independent activation,
actual effect and durable outcome beside clean-STOP/successor controls. A verified
retained invocable path is intermediate evidence, not completed restoration.
Record observer effects without supplying recovery or artificial code retention;
retain CHECK-27/native-memory instrumentation with its appropriate control.
Retained-package/task composition remains unselected; this inert evidence measures
no clock-rate recovery envelope or full lifecycle qualification.

The [plan's threat matrix](plans/windows-msix-store.md#34-threats-and-independent-controls)
owns the detailed cases. Failed required protection blocks affected authority;
it does not authorize weaker loading, privilege expansion or automatic MSI
fallback. Record an actual Store rejection or concrete blocker for the owner to
reconsider. The selected preparation direction changes no execution hold.

Sections 6.4/6.4.1 retain the child-handle, stricter-CET and conditional-enclave
owners; section 9.5.1 retains low-resource profile admission; section 9.8 retains
trace/counter evidence. Preserve one complete MSIX, DoQ-only DNS, optional-storage
versus mandatory-continuity separation and UI-M0 deferrals. The current portable
Core checkpoint, broad Windows execution stop, separate fuzz no-retry hold and
storage pause remain under the [next-task plan](plans/development-plan.md#7-next-executable-task-for-a-development-agent).

## 10. Native Windows qualification and safe experiment ownership

Real Windows tests remain mandatory for service tokens/privileges, pipe identity and impersonation, protected files/registry, reparse/handle lifetime, process mitigations, DLL loading, clock effects, power/resume, accessibility and complete-package lifecycle. Cross-compilation or a Linux model cannot close those checks.

Before any privileged experiment, complete the architecture's experiment card: owned guest, exact source/artifacts, allowed operations, absolute/rate/attempt limits, test endpoints, stop condition, external timeout, restoration steps and independent observation. An unspecified bound means the effect is not authorized. Ordinary agents remain unelevated; only the qualified lab task receives its necessary scoped authority.

Use one clock/state owner per disposable guest. Control host/guest time synchronization and keep recovery reachable independently of guest time. Mutation or fault injection cannot use the code being mutated as its only safety guard; the VM/lab boundary must enforce external limits.

Observe real protected effects and cleanup before fixture teardown or guest destruction can hide leaks. Account for processes, threads, handles, files, services and active clock adjustments. Restore only state whose ownership remains established; ambiguous successor ownership requires read-only/blocked recovery rather than overwriting another owner.

Retain first failures, retry reasons and cleanup evidence. A clean rerun after destroying the failed guest is additional evidence, not erasure of the original failure. Performance/resource and accessibility claims need their own measured tests with the supported security settings enabled.

## 11. Required CI tiers and checks on the checks

### 11.1 Ordered tiers

Independent checks within a tier may run concurrently in isolated owned environments. Every later tier consumes evidence for the identified candidate lineage, recording any Store/signing transformation and the final delivered artifact separately.

| Tier | Required work | When and authority |
| --- | --- | --- |
| T0 — admission | Source/policy/tool identity, lock/feature/source/license/advisory disposition, workflow/secret checks, scope inventory | Before target-controlled execution; tools that invoke Cargo already need the sandbox |
| T1 — fast contracts | Formatting, compiler/Clippy/rustdoc, applicable constructor/consumer and external compile-fail controls, unit/property/API tests, doctests and qualified SAST | Every relevant PR/configuration change; checked postconditions and no privileged clock effects |
| T2 — affected behavior | Regression corpus, focused fuzzing, affected coverage/mutation, native integration, applicable native-memory and retention checks, lifecycle/accessibility checks | Based on changed requirements and milestone; missing affected evidence blocks that change |
| T3 — release candidate | Complete pre-submission candidate matrix, required critical-file gates, local native authority/mitigation/package mechanics, resource measurements, inventory/SBOM and independent review | Exact source/artifact candidate; explicitly retain Store-signature-dependent cases for T4. Local test signing does not pass those cases, and unrelated older runs do not establish candidate coverage |
| T4 — signing/publication | Owner-controlled Store submission, external certification/signing, then final delivered package/catalog/payload and lifecycle qualification; a later selected MSI alternative uses its own qualified signing flow | Preparation needs no speculative Store preapproval; broad release follows actual final-artifact evidence and David's exact-candidate decision |

Scheduled advisory/secret metadata refresh and long fuzz campaigns run separately and can reopen admitted inputs. They do not modify the lockfile or auto-accept risk.

This tier table specifies required evidence, not current CI coverage. The adopted native-memory helpers currently support owned manual Windows runs; no hosted application memory gate is established by installing them. A future CI lane must qualify its own identity/isolation, controls, diagnostic classification and restoration before reporting this evidence automatically.

### 11.2 Gate qualification controls

Run these when admitting a checker or changing its relevant configuration. Use isolated fixtures through the **same entry point and required CI status**, never the production release. Each failing case needs a legitimate neighboring case that passes.

| Gate | Required failing control | Required passing observation |
| --- | --- | --- |
| Scope/lints | Missing member inheritance, excluded critical target, unknown lint, stale expectation | All required members analyzed; permitted local exception remains narrow |
| Unsafe/error rules | Unsafe core block; ignored `Result` including `let _ =`; unchecked conversion; production placeholder panic | Safe/validated path and actual error handling succeed |
| Flags/artifacts | Injected override removes CFG; wrong/stale DLL or weakened PE fixture | Effective settings and every admitted payload reconcile with expected protections |
| Graph/license | Unapproved registry, held transitive provider, unresolved mandatory license conjunction | Reviewed exact graph and valid alternative grant accepted |
| Advisories | Known matched fixture, stale/missing refresh receipt, expired exception | Controlled unaffected fixture and current recorded snapshot accepted |
| Workflow/secrets | Unsafe trigger/checkout, shell injection, unpinned action, synthetic secret | Narrow workflow and ordinary similar data accepted without secret disclosure |
| SAST/reporting | Relevant seeded defect; SARIF-only success; empty/unsupported input | Intended query/file analyzed and failure reaches protected gate |
| Final CI status | Failed dependency, skipped required job, cancelled/missing shard or forged report | Always-scheduled protected aggregator accepts only complete successful required scope for the exact candidate |
| Hosted isolation | Sandbox child inherits administrator/escape authority or can read protected runner state | Independently checked reduced authority, bounded files/network and supported-OS lab behavior |
| Tests | Zero required selection, failed assertion, missing shard or hidden child error | Actual test reaches assertion; expected scope and child status preserved |
| Coverage | Required module omitted, below threshold, zero denominator or stale report | Known line/branch fixture reconciles and each module passes its own threshold |
| Mutation | Missed mutant, unviable build, timeout, zero mutants, unknown JSON, missing shard | Correct baseline and caught behavioral mutation; per-file math/outcomes remain distinct |
| Fuzz/model tools | Reject-all harness, swallowed failure, impossible assumptions or insufficient bounds | Valid accepted input/reachable witness and expected diagnostic/counterexample |
| Native memory diagnostics | Reached heap-overrun or invalid-handle control; disabled/wrong image profile or lost diagnostics | Healthy neighboring control completes, effective settings and intended fault/stack are observed, diagnostics cannot become an exit-zero pass, and owned settings are restored |
| Native effects | Unauthorized/stale operation, leak or failed restoration | Authorized bounded effect independently observed and owned cleanup established |
| Promotion | Wrong-source/wrong-workflow/tampered artifact | Approved exact artifact reaches only the authorized signing route |

Do not broaden tests merely to accumulate volume after the concrete risk is resolved. Keep meaningful focused checks during development and the required full release scope at its gate.

### 11.3 Evidence receipt

Each required run records:

- Candidate commit **and relevant dirty/generated content**, lock/configuration/fixture hashes and requirement IDs.
- Exact toolchain/SDK/tool/query/database identities and target/package/feature/profile selection.
- Actual commands, admitted environment overrides, discovered/ran/skipped scope and resource limits.
- Original subprocess exits, first failures, retries, cancellations and raw result/artifact hashes.
- Gate control results, exclusions, unsupported scope, reviewer and cleanup observations.
- Final state: **PASS, FAIL, BLOCKED, NOT RUN, or justified N/A**; UI-M0's explicitly scoped D1–D3 use **Deferred** with their re-entry triggers.

Skipped, timed-out, unsupported or missing required cases cannot be summarized as PASS. Reviewer agreement does not multiply executions or authorize risk acceptance. A green status for a previous commit is not evidence for the current candidate.

## 12. Repository files, adoption work and completion criteria

### 12.1 Repository files and remaining blueprint

| Path | Purpose and owner |
| --- | --- |
| `AGENTS.md` | Project-specific scope, authority, evidence and stop rules; engineering/security owners |
| `rust-toolchain.toml`, root/member `Cargo.toml`, `Cargo.lock` | Pins, workspace lint inheritance, profiles and exact graph; toolchain/dependency owner |
| `.cargo/config.toml`, scoped `clippy.toml` | Admitted compiler/linker policy and justified lint configuration |
| `deny.toml` | Exact-version validated dependency policy; security/license review |
| `.config/nextest.toml` | Bounded test execution, process limits and explicitly isolated native groups |
| `policy/tool-manifest.json` | Tool origins, exact versions/digests, target support and controls |
| `policy/windows-memory.json`, `policy/windows-debugger.xml` | Adopted native-tool pins, exact image profile and current-user debugger restrictions; native-tool owner |
| `scripts/setup-windows-memory.ps1`, `scripts/windows-memory.ps1`, `scripts/tests/test_windows_memory.ps1` | Adopted acquisition/configuration, guarded per-image lifecycle and helper controls; actual candidate execution/measurement remains separately owned |
| `policy/verification-manifest.toml` | Requirements to modules, critical files, feature rows and native tests |
| `policy/exceptions/` | Narrow reviewed exceptions with owner, evidence and expiry/reopen conditions |
| `verification/gate-fixtures/` | Independent deliberately bad/good checker controls |
| `verification/` scripts or a reviewed `xtask` crate | Gate orchestration, outcome/denominator checking and evidence receipts |
| `.github/workflows/`, `.github/CODEOWNERS` | Isolated checks, protected ownership, release separation and recorded hosted settings |
| `docs/toolchain-and-ci.md` | Future configuration documentation in this blueprint; not present. Use the adopted configuration and `TOOLING.md` meanwhile |
| [docs/release-policy.md](release-policy.md) | Written operational release/recovery specification; first-stable support commitment is proposed and actual account/native/recovery rehearsals remain pending |

The table combines adopted bootstrap files and the remaining implementation blueprint; inspect the checkout and [tooling record](../TOOLING.md) before assuming a file or gate exists. Prefer one authoritative policy source with generated reports to contradictory duplicated rules. Gate implementation itself is security-relevant code and needs negative controls and independent review.

### 12.2 Finite adoption tasks

| Task | Exit evidence | Dependencies and limits |
| --- | --- | --- |
| DEV-01: Pin and isolate | Admitted stable tools/SDK, actual identities, sandbox preflight and bounded execution fixture | Can start with repository bootstrap; no production clock effects |
| DEV-02: Enforce lint/profile policy | Member inventory, strict lint/rustdoc controls, safe/unsafe and override fixtures, parsed configurations | Tool admission first; component panic policy remains a shipping gate |
| DEV-03: Admit dependency/CI policy | Exact graph/source/license dispositions, freshness receipt, workflow/secret/CodeQL controls | Preserve S0.1 crypto/license hold; actual held dependency blocks only dependent product work |
| DEV-04: Qualify behavioral evidence | Independent fixtures, nextest/doctests, 90% module and 95% file checkers with complete denominators | P1 core implementation and reviewed critical manifest; no empty green bootstrap |
| DEV-05: Qualify native analysis/lab | x64 Windows evidence, effective mitigations, bounded lab ownership; native-memory controls and candidate paths; separate retention, ASan and coverage results | AppVerifier/PageHeap/CDB bootstrap controls passed; application/native-product, Leak-layer, soak and ASan qualification remain pending. Tool controls cannot waive required native behavior |
| DEV-06: Protect promotion | Complete candidate inventory, owner-controlled submission, independent review, final delivered package verification and the [release-operations evidence](release-policy.md#9-qualification-matrix-and-remaining-decisions) under the selected trust profile | OPEN-11/14, approved support/transition terms, applicable account/access/recovery rehearsals and post-Store qualification before broad release; no automatic public release from an agent |

Bound unresolved tool experiments by time, owner and decision criterion. If a tool cannot analyze a required surface, record the gap and choose an independently justified alternative; do not remove the requirement. Optional model tools need not block starting bounded P0 work.

### 12.3 What counts as ready

The environment is ready for a particular development stage when its required tools and configurations are admitted, isolation is demonstrated, checker controls pass, scope/ownership is recorded and the next task has an independent acceptance oracle. Later-stage requirements can remain explicitly pending until the relevant implementation exists.

The environment is ready for public release only when the complete artifact-specific matrix, license and signing decisions, native behavior, resource/accessibility checks and release recovery requirements are satisfied. A comprehensive document and strict compiler settings are preparation, not completed verification.

Use the [release policy](release-policy.md) for support status, containment,
publishing-authority incidents and compatible state-preserving correction. Its
matrix refines existing CHECK owners; documentation review is not a rehearsal,
and a support proposal does not settle the separately required dependency
remediation deadlines in ASVS 15.2.1/CHECK-26.

## 13. Research and review record

Three independent research workers examined compiler/Windows mitigations, dependency/CI security, and testing/verification. They read the relevant pinned guidance and canonical requirements and preserved their initial reports. Context7 was used for three targeted Cargo, cargo-deny and nextest documentation queries, followed by primary-source checks. No tool version was inferred to be qualified merely because its documentation was available through Context7.

Material ambiguities were retained explicitly: cargo-deny's rolling schema examples, cargo-fuzz's README versus Windows documentation, unstable sanitizer support, mutation under strict lints, hosted signing versus native lab execution, and analyzer coverage of Windows-only paths. This research did not execute product code, scanner fixtures or native experiments and makes no certification claim.

The sources below are upstream/vendor documentation and source repositories retrieved on 28 October 2026. Rolling documentation describes current behavior; implementation admission must bind it to exact versions. The architecture supplies project-specific thresholds and constraints. Additional fresh review findings and corrections are recorded with the document bundle's research evidence.

## 14. Primary sources

### Compiler, lint and Windows sources

- **C01:** [Announcing Rust 1.99.1](https://blog.rust-lang.org/2026/10/15/Rust-1.99.1/).
- **C02:** [Clippy lint groups](https://doc.rust-lang.org/clippy/lints.html).
- **C03:** [Rustup overrides](https://rust-lang.github.io/rustup/overrides.html).
- **C04:** [Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html#the-lints-table).
- **C05:** [Cargo manifest lint section](https://doc.rust-lang.org/cargo/reference/manifest.html#the-lints-section).
- **C06:** [Rust diagnostic attributes](https://doc.rust-lang.org/reference/attributes/diagnostics.html).
- **C07:** [Official Clippy lint catalog](https://clippy.rust-lang.org/stable/index.html).
- **C08:** [Cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html).
- **C09:** [Rustonomicon FFI and unwinding](https://doc.rust-lang.org/nomicon/ffi.html#ffi-and-unwinding).
- **C10:** [rustc codegen options](https://doc.rust-lang.org/rustc/codegen-options/index.html#control-flow-guard).
- **C11:** [MSVC /guard](https://learn.microsoft.com/en-us/cpp/build/reference/guard-enable-control-flow-guard?view=msvc-170).
- **C12:** [MSVC /NXCOMPAT](https://learn.microsoft.com/en-us/cpp/build/reference/nxcompat-compatible-with-data-execution-prevention?view=msvc-170).
- **C13:** [MSVC /DYNAMICBASE](https://learn.microsoft.com/en-us/cpp/build/reference/dynamicbase-use-address-space-layout-randomization?view=msvc-170).
- **C14:** [MSVC /HIGHENTROPYVA](https://learn.microsoft.com/en-us/cpp/build/reference/highentropyva-support-64-bit-aslr?view=msvc-170).
- **C15:** [MSVC /CETCOMPAT](https://learn.microsoft.com/en-us/cpp/build/reference/cetcompat?view=msvc-170).
- **C16:** [MSVC /GS](https://learn.microsoft.com/en-us/cpp/build/reference/gs-buffer-security-check?view=msvc-170).
- **C17:** [MSVC linker /WX](https://learn.microsoft.com/en-us/cpp/build/reference/wx-treat-linker-warnings-as-errors?view=msvc-170).
- **C18:** [Cargo configuration and rustflags](https://doc.rust-lang.org/cargo/reference/config.html#buildrustflags).
- **C19:** [Rust unstable stack_protector](https://doc.rust-lang.org/unstable-book/compiler-flags/stack-protector.html).
- **C20:** [MSVC /Qspectre](https://learn.microsoft.com/en-us/cpp/build/reference/qspectre?view=msvc-170).
- **C21:** [Rust unstable sanitizers: CFI](https://doc.rust-lang.org/unstable-book/compiler-flags/sanitizer.html#controlflowintegrity).
- **C22:** [GetProcessMitigationPolicy](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessmitigationpolicy).
- **C23:** [Rust Windows MSVC target support](https://doc.rust-lang.org/rustc/platform-support/windows-msvc.html).
- **C24:** [Optimized build of the Rust compiler](https://rustc-dev-guide.rust-lang.org/building/optimized-build.html). Compiler-build scope, not an application recipe.
- **C25:** [Rust issue #109114: unstable dylib ThinLTO on Windows MSVC](https://github.com/rust-lang/rust/issues/109114). Open when checked 29 October 2026; scope and status must be rechecked for dependent changes.
- **C26:** [Rust PGO workflow](https://doc.rust-lang.org/rustc/profile-guided-optimization.html), [Rust 1.99.1 profiler runtime build](https://github.com/rust-lang/rust/blob/1.99.1/library/profiler_builtins/build.rs), and [LLVM raw profile format/version limits](https://llvm.org/docs/InstrProfileFormat.html#raw-profile-format).
- **C27:** [BOLT input binary requirements](https://github.com/llvm/llvm-project/blob/main/bolt/README.md#input-binary-requirements).
- **C28:** [Rust 1.99.1 Windows x64 target source](https://github.com/rust-lang/rust/blob/1.99.1/compiler/rustc_target/src/spec/targets/x86_64_pc_windows_msvc.rs), [LLVM x86 microarchitecture levels](https://clang.llvm.org/docs/UsersManual.html#x86), and [Cargo's separate unstable build-std workflow](https://doc.rust-lang.org/cargo/reference/unstable.html#build-std).
- **C29:** [Rust cross-language/linker-plugin LTO and Windows toolchain constraints](https://doc.rust-lang.org/rustc/linker-plugin-lto.html).
- **C30:** Pinned Rust 1.99.1 [vectorization selection](https://github.com/rust-lang/rust/blob/58cf8f9c35b355199727627d07ae34a4e2846d67/compiler/rustc_codegen_ssa/src/back/write.rs#L229-L237), [LLVM optimization inputs](https://github.com/rust-lang/rust/blob/58cf8f9c35b355199727627d07ae34a4e2846d67/compiler/rustc_codegen_llvm/src/back/write.rs#L589-L599), and [default pipeline construction](https://github.com/rust-lang/rust/blob/58cf8f9c35b355199727627d07ae34a4e2846d67/compiler/rustc_llvm/llvm-wrapper/PassWrapper.cpp#L900-L938).
- **C31:** Pinned Rust 1.99.1 [removed inline-threshold option](https://github.com/rust-lang/rust/blob/58cf8f9c35b355199727627d07ae34a4e2846d67/compiler/rustc_session/src/options.rs#L2242-L2245), [unstable tune-cpu definition](https://github.com/rust-lang/rust/blob/58cf8f9c35b355199727627d07ae34a4e2846d67/compiler/rustc_session/src/options.rs#L2922-L2923), and [raw LLVM argument stability limits](https://doc.rust-lang.org/rustc/codegen-options/index.html#llvm-args).
- **C32:** Pinned Rust 1.99.1 [MSVC-flavor optimization flags](https://github.com/rust-lang/rust/blob/58cf8f9c35b355199727627d07ae34a4e2846d67/compiler/rustc_codegen_ssa/src/back/linker.rs#L1012-L1023), [PDB request](https://github.com/rust-lang/rust/blob/58cf8f9c35b355199727627d07ae34a4e2846d67/compiler/rustc_codegen_ssa/src/back/linker.rs#L1075-L1084), and its pinned LLVM submodule's [PDB handling](https://github.com/rust-lang/llvm-project/blob/1b9c0d5ff9bbe7634aead059efe6b11a7eeba145/lld/COFF/Driver.cpp#L1803-L1831) / [incremental-link default](https://github.com/rust-lang/llvm-project/blob/1b9c0d5ff9bbe7634aead059efe6b11a7eeba145/lld/COFF/Driver.cpp#L2261-L2270).
- **C33:** Microsoft [COMDAT removal/folding and identity/debugger caveats](https://learn.microsoft.com/en-us/cpp/build/reference/opt-optimizations?view=msvc-170), [PDB modes](https://learn.microsoft.com/en-us/cpp/build/reference/debug-generate-debug-info?view=msvc-170), and [native LTCG input requirements](https://learn.microsoft.com/en-us/cpp/build/reference/ltcg-link-time-code-generation?view=msvc-170).
- **C34:** LLVM [vectorizer profitability, width and unroll cost models](https://llvm.org/docs/Vectorizers.html).
- **C35:** LLVM [fast-math flag semantics, including contraction and reassociation](https://llvm.org/docs/LangRef.html#fast-math-flags), and Clang [floating-point and denormal-environment controls](https://clang.llvm.org/docs/UsersManual.html#controlling-floating-point-behavior).
- **C36:** Broader [Windows dllimport issue #27438](https://github.com/rust-lang/rust/issues/27438), [2024 native-global-data discussion](https://github.com/rust-lang/rust/issues/27438#issuecomment-2360332201), ordinary-application [ThinLTO/LLD issue #81408](https://github.com/rust-lang/rust/issues/81408), later changes [#129079](https://github.com/rust-lang/rust/pull/129079) and [#122790](https://github.com/rust-lang/rust/pull/122790), pinned [import handling](https://github.com/rust-lang/rust/blob/58cf8f9c35b355199727627d07ae34a4e2846d67/compiler/rustc_codegen_llvm/src/consts.rs), [self-import generation](https://github.com/rust-lang/rust/blob/58cf8f9c35b355199727627d07ae34a4e2846d67/compiler/rustc_codegen_ssa/src/back/write.rs#L2294-L2315), and the [native dual-linker regression test](https://github.com/rust-lang/rust/blob/58cf8f9c35b355199727627d07ae34a4e2846d67/tests/run-make/msvc-lld-thinlto-imp-symbols/rmake.rs).

Additional primary Microsoft sources checked for the 3 November selected-MSIX
amendment; these establish documented semantics, not executed native results:

- **C37:** [CFG policy fields and runtime-setting limits](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_control_flow_guard_policy).
- **C38:** [User shadow-stack policy fields and runtime-setting limits](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_user_shadow_stack_policy).
- **C39:** [MSIX PackageIntegrity](https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/element-uap10-packageintegrity) and [Content enforcement](https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/element-uap10-content).
- **C40:** [Code Integrity Guard and Store allowance](https://learn.microsoft.com/en-us/defender-endpoint/exploit-protection-reference#code-integrity-guard), [Exploit Protection PowerShell names](https://learn.microsoft.com/en-us/defender-endpoint/customize-exploit-protection#powershell-reference), [binary-signature policy fields](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_binary_signature_policy) and [process-creation mitigation attributes](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-updateprocthreadattribute).

Primary sources checked for the 6 November required v3/hardening selection;
compiler/linker source support does not establish Windows enforcement:

- **C41:** Exact Rust LLVM pin `1b9c0d5ff9bbe7634aead059efe6b11a7eeba145`: [LLD COFF options](https://github.com/rust-lang/llvm-project/blob/1b9c0d5ff9bbe7634aead059efe6b11a7eeba145/lld/COFF/Options.td), [option parsing and defaults](https://github.com/rust-lang/llvm-project/blob/1b9c0d5ff9bbe7634aead059efe6b11a7eeba145/lld/COFF/Driver.cpp), [extended CET bits and load-config writing](https://github.com/rust-lang/llvm-project/blob/1b9c0d5ff9bbe7634aead059efe6b11a7eeba145/lld/COFF/Writer.cpp), [guard-option parser](https://github.com/rust-lang/llvm-project/blob/1b9c0d5ff9bbe7634aead059efe6b11a7eeba145/lld/COFF/DriverUtils.cpp#L99-L114) and [PE bit definitions](https://github.com/rust-lang/llvm-project/blob/1b9c0d5ff9bbe7634aead059efe6b11a7eeba145/llvm/include/llvm/BinaryFormat/COFF.h).
- **C42:** Microsoft [/DEPENDENTLOADFLAG static-import scope](https://learn.microsoft.com/en-us/cpp/build/reference/dependentloadflag?view=msvc-170), [LoadLibraryEx flag values](https://learn.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-loadlibraryexw) and [packaged/unpackaged DLL search order](https://learn.microsoft.com/en-us/windows/win32/dlls/dynamic-link-library-search-order).
- **C43:** Pinned Rust [unstable option declarations, including EHCONT and stack protectors](https://github.com/rust-lang/rust/blob/58cf8f9c35b355199727627d07ae34a4e2846d67/compiler/rustc_session/src/options.rs), [conditional EHCONT LLVM module flag](https://github.com/rust-lang/rust/blob/58cf8f9c35b355199727627d07ae34a4e2846d67/compiler/rustc_codegen_llvm/src/context.rs#L486-L489) and Microsoft [EH continuation metadata requirements and empty-table semantics](https://learn.microsoft.com/en-us/cpp/build/reference/guard-enable-eh-continuation-metadata?view=msvc-170).
- **C44:** Pinned Rust [stable jump-tables documentation](https://github.com/rust-lang/rust/blob/58cf8f9c35b355199727627d07ae34a4e2846d67/src/doc/rustc/src/codegen-options/index.md#jump-tables), [option declaration](https://github.com/rust-lang/rust/blob/58cf8f9c35b355199727627d07ae34a4e2846d67/compiler/rustc_session/src/options.rs#L2251) and [LLVM attribute lowering](https://github.com/rust-lang/rust/blob/58cf8f9c35b355199727627d07ae34a4e2846d67/compiler/rustc_codegen_llvm/src/attributes.rs#L330).
- **C45:** Microsoft [/FIXED and relocation generation](https://learn.microsoft.com/en-us/cpp/build/reference/fixed-fixed-base-address?view=msvc-170) and [/LARGEADDRESSAWARE](https://learn.microsoft.com/en-us/cpp/build/reference/largeaddressaware-handle-large-addresses?view=msvc-170). For the selected linker's actual defaults, use C41's pinned driver; its comment explicitly distinguishes observed EXE behavior from Microsoft's `/FIXED` default wording.
- **C46:** Microsoft [/SAFESEH architecture restriction](https://learn.microsoft.com/en-us/cpp/build/reference/safeseh-image-has-safe-exception-handlers?view=msvc-170) and [/INTEGRITYCHECK signing contract](https://learn.microsoft.com/en-us/cpp/build/reference/integritycheck-require-signature-check?view=msvc-170).

### Dependency and CI sources

- **SC01:** [Cargo fetch and locked/offline/frozen semantics](https://doc.rust-lang.org/cargo/commands/cargo-fetch.html).
- **SC02:** [Cargo build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html).
- **SC03:** [Rust procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html).
- **SC04:** [CodeQL build options for compiled languages](https://docs.github.com/en/code-security/reference/code-scanning/codeql/build-options-for-compiled-languages).
- **SC05:** [zizmor usage, modes, exit codes and personas](https://docs.zizmor.sh/usage/).
- **SC06:** [cargo-deny checks](https://embarkstudios.github.io/cargo-deny/checks/index.html).
- **SC07:** [RustSec overview and tooling](https://rustsec.org/).
- **SC08:** [cargo-auditable upstream README](https://github.com/rust-secure-code/cargo-auditable).
- **SC09:** [CodeQL system requirements](https://codeql.github.com/docs/codeql-overview/system-requirements/).
- **SC10:** [actionlint upstream README](https://github.com/rhysd/actionlint).
- **SC11:** [zizmor audit rules](https://docs.zizmor.sh/audits/).
- **SC12:** [Gitleaks upstream README](https://github.com/gitleaks/gitleaks).
- **SC13:** [GitHub secret scanning](https://docs.github.com/en/code-security/concepts/secret-security/secret-scanning).
- **SC14:** [CycloneDX Cargo upstream README](https://github.com/CycloneDX/cyclonedx-rust-cargo).
- **SC15:** [Cargo Vet introduction](https://mozilla.github.io/cargo-vet/).
- **SC16:** [cargo-crev upstream repository](https://github.com/crev-dev/cargo-crev).
- **SC17:** [cargo-geiger intended use](https://github.com/geiger-rs/cargo-geiger).
- **SC18:** [cargo-machete README](https://github.com/bnjbvr/cargo-machete).
- **SC19:** [Cargo features](https://doc.rust-lang.org/cargo/reference/features.html).
- **SC20:** [cargo-deny sources config](https://embarkstudios.github.io/cargo-deny/checks/sources/cfg.html).
- **SC21:** [cargo-deny licenses interpretation](https://embarkstudios.github.io/cargo-deny/checks/licenses/index.html).
- **SC22:** [cargo-deny licenses config](https://embarkstudios.github.io/cargo-deny/checks/licenses/cfg.html).
- **SC23:** [cargo-deny advisories config](https://embarkstudios.github.io/cargo-deny/checks/advisories/cfg.html).
- **SC24:** [SPDX 3.0.1 license expression specification](https://spdx.github.io/spdx-spec/v3.0.1/annexes/spdx-license-expressions/).
- **SC25:** [GitHub securely using pull_request_target](https://docs.github.com/en/actions/reference/security/securely-using-pull_request_target).
- **SC26:** [GitHub secure use reference](https://docs.github.com/en/actions/reference/security/secure-use).
- **SC27:** [GitHub workflow dependency lockfiles](https://docs.github.com/en/actions/concepts/security/workflow-dependency-lockfiles).
- **SC28:** [GitHub artifact attestations](https://docs.github.com/en/actions/concepts/security/artifact-attestations).
- **SC29:** [cargo-deny common CLI options](https://embarkstudios.github.io/cargo-deny/cli/common.html).
- **SC30:** [cargo-deny check CLI](https://embarkstudios.github.io/cargo-deny/cli/check.html).
- **SC31:** [GitHub required status checks and skipped-job behavior](https://docs.github.com/en/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks).
- **SC32:** [GitHub-hosted runner images and administrative privileges](https://docs.github.com/en/actions/reference/runners/github-hosted-runners).

### Verification sources

- **V01:** [cargo-nextest: Running tests](https://nexte.st/docs/running/).
- **V02:** [cargo-llvm-cov README](https://github.com/taiki-e/cargo-llvm-cov).
- **V03:** [cargo-mutants: Using the results](https://mutants.rs/using-results.html).
- **V04:** [cargo-mutants: Getting started](https://mutants.rs/getting-started.html).
- **V05:** [cargo-mutants: Exit codes](https://mutants.rs/exit-codes.html).
- **V06:** [cargo-mutants: Strict lints](https://mutants.rs/lints.html).
- **V07:** [cargo-mutants: Hangs and timeouts](https://mutants.rs/timeouts.html).
- **V08:** [cargo-mutants: Stability](https://mutants.rs/stability.html).
- **V09:** [cargo-mutants: The mutants.out directory](https://mutants.rs/mutants-out.html).
- **V10:** [Proptest: Failure Persistence](https://proptest-rs.github.io/proptest/proptest/failure-persistence.html).
- **V11:** [Proptest: Limitations of Property Testing](https://proptest-rs.github.io/proptest/proptest/limitations.html).
- **V12:** [cargo-fuzz README](https://github.com/rust-fuzz/cargo-fuzz).
- **V13:** [Rust Fuzz Book: Fuzzing on Windows](https://rust-fuzz.github.io/book/cargo-fuzz/windows.html).
- **V14:** [Rust Fuzz Book: Windows Setup](https://rust-fuzz.github.io/book/cargo-fuzz/windows/setup.html).
- **V15:** [LLVM: libFuzzer](https://llvm.org/docs/LibFuzzer.html).
- **V16:** [Rust Unstable Book: sanitizer](https://doc.rust-lang.org/unstable-book/compiler-flags/sanitizer.html).
- **V17:** [Miri README](https://github.com/rust-lang/miri).
- **V18:** [Microsoft: AddressSanitizer](https://learn.microsoft.com/en-us/cpp/sanitizers/asan?view=msvc-170).
- **V19:** [Loom 0.7.2 crate documentation](https://docs.rs/loom/0.7.2/loom/).
- **V20:** [Kani: Installation](https://model-checking.github.io/kani/install-guide.html).
- **V21:** [Kani: Loops, unwinding, and bounds](https://model-checking.github.io/kani/tutorial-loop-unwinding.html).
- **V22:** [Rust 1.99.1 x86_64_pc_windows_msvc target source](https://github.com/rust-lang/rust/blob/1.99.1/compiler/rustc_target/src/spec/targets/x86_64_pc_windows_msvc.rs).
- **V23:** [Rust sanitizer stabilization tracking issue 123615](https://github.com/rust-lang/rust/issues/123615).
- **V24:** [Rust 1.99.1 compiler options source](https://github.com/rust-lang/rust/blob/1.99.1/compiler/rustc_session/src/options.rs).

Related documents: [architecture revision 1.4](windows-time-architecture.md), [package signing guide](windows-time-signing-guide.md), [ASVS applicability worksheet](asvs-5.0.0-applicability.md), [independent architecture review](windows-time-review.md) and [unadopted update-system assessment](windows-time-update-assessment.md).

