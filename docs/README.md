# NevroTick — documentation guide

**Prepared for:** David Osipov  
**Snapshot date:** 29 October 2026  
**Canonical architecture:** RWTC-ARCH-001, revision 1.4

The maintained set contains ten principal Markdown documents and seven supporting independent reviews. The owner-decision/research addition is a scoped amendment, not a repeated full-product qualification. The feature plan translates the architecture into basic user capabilities, screens, defaults, acceptance cases and delivery slices. The architecture and development-hardening guide include 29 October 2026 amendments making upstream-code verification and stress testing explicit, adopting garde-backed construction plus explicit function/module contracts, and integrating installed native memory diagnostics into agent workflows. Proposed behavior and planned checks are not implementation evidence.

## Plans, acceptance and execution evidence

Agents start with [AGENTS.md](../AGENTS.md), choose the existing task and relevant
rows below, then load the matching [repository skills](../.agents/skills/README.md).
Use the [testing contract](../.agents/skills/testing/CONTRACT.md) to select evidence.
The plan is distributed by responsibility; these links locate its owners without
creating another specification.

| Question | Source of truth |
| --- | --- |
| Which product capabilities and user journeys are intended? | [Feature specifications, F-01–F-10](windows-time-features.md#2-feature-specifications) and [essential journeys](windows-time-features.md#6-essential-user-journeys). |
| What owns the behavior, authority and state? | [Component architecture](windows-time-architecture.md#3-component-architecture-and-authority), the task-relevant technical sections and [accepted owner decisions](nevrotick-owner-decisions.md). |
| Which phase is ready, and what must it prove? | [Architecture §21](windows-time-architecture.md#21-delivery-phases-and-readiness-gates): P0–P7, including the bounded S0.1–S0.7 experiments. |
| What integrated slice should the task deliver? | [Feature plan §9](windows-time-features.md#9-delivery-slices-for-agentic-development): UI-M0 and S-0–S-6, with prerequisites, owners and exit evidence. |
| What must a user-visible product scenario demonstrate? | [Feature plan §8](windows-time-features.md#8-acceptance-scenarios-and-evidence): AC-01–AC-20, mapped to technical checks. |
| What technical/security result is required? | [Architecture §20.2](windows-time-architecture.md#202-acceptance-catalog): CHECK-1–CHECK-27, independent expected results, protected effects, valid controls and stage/owner. |
| What is in scope for the current UI prototype? | [Architecture §16.5](windows-time-architecture.md#165-restricted-first-ui-milestone-ui-m0): M0-01–M0-08 and D1–D3 deferrals/re-entry gates; [UI-M0 task](../tasks/ui-m0.md) for implementation ownership and evidence. |
| Which unresolved decisions or investigations gate dependent work? | [Open decisions](windows-time-architecture.md#22-open-decisions-and-release-blockers), [owner-decision procedure](nevrotick-owner-decisions.md) and [research assignments](nevrotick-research-handoff.md). |
| How are checks run, and where are results kept? | [TOOLING.md](../TOOLING.md#starting-implementation) and [configured tool execution](../.agents/skills/testing/references/configured-tooling.md); the existing task record owns candidate-specific results and gaps. |

Keep the identifiers distinct: S-0 is a delivery slice; S0.1–S0.7 are P0
experiments. UI-M0's M0 checks do not complete the AC/CHECK catalogs or P0/P5.
Select only the criteria applicable to the authorized task and retain deferred
requirements with their owners and restoration triggers.

The current [UI-M0 task](../tasks/ui-m0.md#remaining-evidence-and-next-work)
records an implemented, observed prototype with required full native-memory
instrumentation still incomplete. Read its later candidate addenda before reusing
earlier results, and verify relevant source/environment identity before relying on
them. A task receipt records execution; a catalog specifies expectations.

## Current baseline and amendments

**Upstream code is not presumed bug-free or vulnerability-free.** Chrony and other reused projects, libraries and tools require independent verification of the behavior NevroTick relies on. See [architecture section 20.1](windows-time-architecture.md#201-independent-numerical-truth) and [the test and stress matrix](windows-time-development-hardening.md#97-verification-of-upstream-code-and-stress-testing).

**Development baseline, 29 October 2026:** The checkout now contains a minimal real [Cargo workspace](../Cargo.toml), one `nevrotick-ui` crate and its locked UI-M0 dependency graph. [The tooling record](../TOOLING.md#starting-implementation) describes the shared portable, dependency and Windows cross-build checks; [the task record](../tasks/ui-m0.md) owns current implementation and native observations. The frozen graph passed advisory, license, source and build-script checks for Linux and Windows. This admission is limited to UI-M0; the [planned runtime dependency refresh](windows-time-architecture.md#186-runtime-dependency-register) still leaves held AWS-LC/Schannel providers and the production graph unresolved.

**Native memory diagnostics, 29 October 2026:** Application Verifier, GFlags/PageHeap and CDB are installed and configured; healthy, heap-overrun and invalid-handle bootstrap controls passed. [Development hardening section 9.5](windows-time-development-hardening.md#95-fuzzing-and-native-memory-diagnostics) defines when agents must select corruption checks and separate retention measurements; [the shared skill workflow](../.agents/skills/testing/references/windows-memory.md) provides execution and cleanup rules. These are tool controls, not application leak/soak or supported-product qualification. Native runs remain owned manual checks until a separate CI lane is qualified.

**Performance guidance:** [Architecture section 17](windows-time-architecture.md#17-resource-and-performance-targets) owns the proposed budgets. Use [performance optimization](../.agents/skills/implementation/optimize-rust-performance/SKILL.md) to locate and improve a measured bottleneck, paired with [resource QA](../.agents/skills/testing/qa-performance-resources/SKILL.md) for comparable measurements and acceptance. This workflow preserves correctness/security and does not claim measured product performance before implementation.

**Contract adoption, 29 October 2026:** [DEC-18 and section 6](windows-time-architecture.md#61-validation-architecture-and-enforcement-order) select garde for mandatory shared DTO/settings validation inside private fallible construction. [Every function/module contract](windows-time-architecture.md#631-contracts-for-every-function-and-module) must establish preconditions, checked outputs, failure/effect behavior and current-state obligations through types and the relevant checks. The [agent enforcement rules](windows-time-development-hardening.md#34-mandatory-contracts-for-agent-written-code) preserve one rule owner and prohibit construction bypasses. The UI-M0 model now implements checked preview construction using the admitted garde graph; its behavioral evidence remains distinct from the original design adoption and from future production contracts.

**Windows mitigation refinement, 29 October 2026:** [architecture section 15.4](windows-time-architecture.md#154-per-process-mitigation-profiles-and-startup-admission) defines process roles, activation limits, startup failure handling and the mitigation manifest; [hardening section 6.4](windows-time-development-hardening.md#64-per-process-mitigation-qualification) supplies the native qualification matrix. Existing CFG/ASLR/DEP configuration and bootstrap evidence are distinguished from pending application enforcement. Read these before adding process policies or treating every mitigation as a runtime switch.

**Diagnostics and shared Windows adapters, 29 October 2026:** [architecture section 3.4](windows-time-architecture.md#34-shared-windows-adapters-and-diagnostics) distinguishes reusable OS adapters, portable IPC contracts and per-process composition. [Section 12.3](windows-time-architecture.md#123-logging-and-support) owns logger structure, INFO defaults, authorized bounded DEBUG sessions, required events, redaction and resource/failure rules; [hardening section 9.8](windows-time-development-hardening.md#98-diagnostics-and-shared-adapter-verification) supplies the acceptance matrix. Those amendments remain component contracts; the fixture-only UI-M0 implementation does not implement the production diagnostics or shared service/IPC adapters.

**IPC external comparison, 29 October 2026:** [architecture section 5.4](windows-time-architecture.md#54-native-ipc-implementation-and-qualification) records refinements from Microsoft guidance and pinned Tokio/Mio, Kestrel and OpenVPN source: caller-token association/reversion, effective rights, continuous pipe ownership and cancellation/completion. [Hardening section 9.9](windows-time-development-hardening.md#99-ipc-identity-framing-and-native-lifetime-verification) maps them to existing native acceptance checks. The transport, runtime, dependency and UI-M0 decisions remain; external comparison is not implementation evidence.

**Current UI milestone:** [UI-M0, architecture §16.5](windows-time-architecture.md#165-restricted-first-ui-milestone-ui-m0), is a restricted native fixture-only prototype. D1 display/theme, D2 production service/policy-state and D3 full keyboard/Narrator/UIA work are explicitly **Deferred**, not prerequisites for UI-M0. Restore them at their specified integration/preview gates. Other workstreams and eventual product/release requirements remain unchanged; completing UI-M0 does not complete P0 toolkit qualification.

**Latest owner instructions:** [nevrotick-owner-decisions.md](nevrotick-owner-decisions.md) records accepted reuse/recovery/release choices and unresolved default-profile policy. [nevrotick-research-handoff.md](nevrotick-research-handoff.md) assigns the remaining investigations, including the supplied NTS directory and proposed operator candidates. Read these before interpreting older unconditional signing language in retained historical reviews.

**Engine runtime clarification, 29 October 2026:** [architecture section 18.6.1](windows-time-architecture.md#1861-engine-runtime-decision-and-alternatives) records the rationale, component boundaries and reconsideration criteria for the existing minimal current-thread Tokio prototype. Section 18.7.1 keeps a bounded synchronous NTS-KE worker available alongside the held async transport candidate. Runtime choice establishes no measured resource, security or timing result; existing dependency and Windows qualification gates apply.

## Reading guide

| Order | Document | Scope and status |
| --- | --- | --- |
| A | [nevrotick-owner-decisions.md](nevrotick-owner-decisions.md) | Owner instructions of 29 October; accepted choices, recommendations and unresolved policy distinguished. |
| B | [nevrotick-research-handoff.md](nevrotick-research-handoff.md) | Bounded future research, NTS operator candidates, evidence requirements and owner decision cards. |
| 1 | [windows-time-features.md](windows-time-features.md) | V1 product feature plan: ten capabilities, main journeys, authority, recovery, twenty acceptance scenarios and staged delivery. |
| 2 | [windows-time-architecture.md](windows-time-architecture.md) | Canonical creation specification, revision 1.4: technical contracts, threat model, NTP/NTS, Windows authority and release gates. |
| 3 | [windows-time-diagrams.md](windows-time-diagrams.md) | Editable Mermaid architecture views aligned with revision 1.4. |
| 4 | [windows-time-development-hardening.md](windows-time-development-hardening.md) | Strict Rust/Windows agentic development policy, CI, auditors, tool qualification and evidence requirements. See the current tooling record for adopted bootstrap checks; application/native/release qualification remains pending. |
| 5 | [asvs-5.0.0-applicability.md](asvs-5.0.0-applicability.md) | All 345 ASVS 5.0.0 requirements mapped to proposed applicability/verification; not completed compliance. |
| 6 | [windows-time-signing-guide.md](windows-time-signing-guide.md) | Practical MSI signing application, artifact workflow and verification guide; provider acceptance remains open. |
| 7 | [windows-time-update-assessment.md](windows-time-update-assessment.md) | TUF/update research and constraints; no automatic updater or TUF architecture has been adopted. |
| 8 | [windows-time-review.md](windows-time-review.md) | Independent architecture review and revision 1.4 reconciliation. Its earlier reviewed-baseline label is intentional. |

Accepted owner decisions govern their stated scope; recommendations and open questions remain proposals. The architecture controls the corresponding technical/security contracts. The feature plan supplies proposed presentation and product scope; unresolved provider, numerical, Windows, licensing and signing decisions retain their gates. The complete application remains one MSI. The preferred Authenticode route and owner-approved initial OpenPGP alternative have separate evidence and Windows compatibility claims; neither is qualified by this documentation.

## UI design and brand references

David has designated these existing bundles as references for agents working on
the UI. Use design v0.2 for screen composition and interaction flows, and the AVIF
kit for brand artwork and variants.

| Reference | Start here |
| --- | --- |
| Design guidance | [Design v0.2 README](nevrotick-design-v0.2/README.md), then [UI specification](nevrotick-design-v0.2/docs/02-ui-spec.md). |
| Visual and interaction examples | [Interactive HTML preview](nevrotick-design-v0.2/NevroTick-UI-preview.html) and [screen/scene catalogue](nevrotick-design-v0.2/screens/README.md). |
| Brand assets | [AVIF kit README](NevroTick-AVIF-kit-v1/README.md) and [asset manifest](NevroTick-AVIF-kit-v1/manifest.json), including transparent, light/dark and size variants. |
| Asset rights and provenance | [Brand notice](NevroTick-AVIF-kit-v1/BRAND-LICENSE.txt) and the kit's per-image metadata; keep these separate from the application source-code licence. |

Apply the references within the current UI-M0 scope and D1–D3 deferrals. The React
viewer illustrates the design; its validators, dependencies and mock outcomes do
not define the Rust/Win32 implementation. Rendered HTML/screenshots may predate
specification corrections. The live canonical architecture and accepted owner
decisions retain precedence over archived copies in the design bundle.

## Supporting independent reviews

These preserve first-pass findings and correction checks, not competing specifications or claims of runtime qualification.

- [tuf-protocol-review.md](supporting-reviews/tuf-protocol-review.md): protocol, time assurance and recovery.
- [tuf-dependency-review.md](supporting-reviews/tuf-dependency-review.md): Rust clients, crypto graphs and evidence limits.
- [tuf-windows-review.md](supporting-reviews/tuf-windows-review.md): authority, staging, MSI handoff and lifecycle.
- [signing-guide-review.md](supporting-reviews/signing-guide-review.md): provider onboarding and signing-guide corrections.
- [development-compiler-review.md](supporting-reviews/development-compiler-review.md): compiler/lint/profile and Windows mitigation review.
- [development-evidence-review.md](supporting-reviews/development-evidence-review.md): CI trust, isolation, mutation contract and correction verification.
- [basic-features-review.md](supporting-reviews/basic-features-review.md): integrated product feature, UX, authority and acceptance review.

Some records retain workspace paths, initial hashes and source identifiers for provenance. Read correction addenda alongside the first-pass findings. Main documents control current requirements; the tooling record identifies completed tool controls separately from pending application, native-product and signing qualification.

## Format and integrity

All documents are UTF-8 Markdown. Mermaid remains editable fenced source. Keep the extracted directory together for relative document links.

An exported bundle may supply `SHA256SUMS.txt` for byte-integrity comparison; this checkout currently has no root documentation manifest, so it establishes no bundle-integrity result. The design subdirectories maintain their own separate manifests. Checksums do not authenticate a publisher or replace a digital signature.

Older PDF/CSV exports, intermediate candidates, helper scripts, credentials and executable tools are excluded. No application build, Windows installation, clock change, scanner execution, signing request or conformance test was performed to create the feature-plan addition.
