---
name: "implement-slint"
description: "Use when implementing, migrating or reviewing NevroTick Slint views, .slint files, Rust bindings, models, callbacks, layouts or Windows UI build profiles."
---

# Implement the Slint UI

Slint is the selected complete Win32 UI replacement under
[OD-11](../../../../docs/nevrotick-owner-decisions.md#15-od-11--replace-the-win32-ui-with-slint).
Start the current child in [S0.5](../../../../tasks/s0.5-ui-comparison.md), then read
[native UI guidance](../implement-native-ui/SKILL.md) and the
[implementation contract](../CONTRACT.md). Existing Win32 source is the migration
baseline. Toolkit selection is settled; dependencies and native behavior still
need evidence. Source-only preparation and admitted fixture work may parallel Core.

## Choose the relevant reference

| Change | Read |
| --- | --- |
| Components, Rust callbacks, models, validation, layout or navigation | [Rust and view boundary](references/rust-ui.md) |
| Cargo/build profile, renderer, assets, licences or Windows qualification | [Windows profile](references/windows-profile.md) |
| Translation catalogs, language changes or binary locale resources | [OD-12 localization design](../../../../docs/plans/ui-localization.md) and the Rust/view reference |
| API/version uncertainty or changed upstream | [Pinned sources and lookup method](SOURCES.md) |

Reuse verified evidence while the relevant version/profile is unchanged. For new
API questions prefer available Context7 library resolution/documentation and
GitHub MCP source reads at an immutable revision. Check the actual tool catalog;
report unavailable connectors and use official HTTPS sources when needed. A live
documentation URL or example version does not override the selected manifest.

## Work through the boundary

1. Freeze the selected slice and UI/graph/model ownership. Consult
   [design v0.4](../../../../docs/nevrotick-design-v0.4/README.md), its current audit
   and brand kit. Map the reached controls and states; port composition rather
   than copying JavaScript validation or every simulated workflow.
2. Before adopting dependencies, record matching runtime/build versions, features,
   backend/renderer, licence basis and generated/native/asset inputs through
   [dependency admission](../../environment/dependency-updates/SKILL.md).
   The researched 1.18.1 profile is a reference, not an adopted Cargo graph.
3. Keep checked Rust domain state authoritative. Slint properties/models project
   accepted values; callbacks pass bounded raw editor input to the existing
   checked constructor/update. Rejection preserves accepted state and raw drafts.
4. Own components, models and mutations on the UI event-loop thread. Use weak
   component captures, observable model updates and bounded queued data. Define
   timer/worker cancellation and teardown; closing must not retain a window cycle.
5. Preserve the current fixture boundary: sample labels, transient preview edits,
   unavailable protected actions and no live network/IPC, persistence, services,
   elevation or clock effects. Keep D1–D3 at their existing re-entry triggers.
   Full Slint migration does not make live P5 ready.
6. Select checks through the [testing contract](../../testing/CONTRACT.md).
   Exercise accepted/rejected edits and direct callback/model entrypoints. Use
   admitted build and Windows observations at the applicable stage; HTML passes,
   compiler success and Slint accessibility support qualify different things.

Report the exact source/profile, changed view-to-domain path, actual checks,
remaining gaps and next S0.5 action. Preserve existing execution holds. A skill
invocation authorizes neither dependency installation nor a native experiment.
