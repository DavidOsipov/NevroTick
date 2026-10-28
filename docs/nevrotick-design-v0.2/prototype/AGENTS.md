# Prototype Instructions

## NevroTick design scope

The user selected the third original mascot: white hair streak on the viewer's left, asymmetric eyes, angular face, slate-blue jacket and two watches. Preserve the supplied image identity.

This is an interactive design prototype of a small native Windows time utility, not the product runtime. The production architecture remains Rust/Win32; the time services, UAC, MSI, NTP and NTS are simulated here. V1 has no DNS features and no adopted automatic updater. Use the source architecture revision 1.4 in ../references/source-docs.

Keep manual server entry and visible NTS/NTP selection. Preserve separate draft/applied state, NTS-only versus NTP-only gating, no silent downgrade, and explicit bootstrap recovery. Keep user-facing app copy in English; Russian labels outside the application window explain prototype scenes.

Run the local server yourself and open the preview in the browser available to this environment. Do not give the user server-start instructions when you can run it.

Before making substantial visual changes, use the Product Design plugin's `get-context` skill when the visual source is unclear or no longer matches the current goal. When the user gives durable prototype-specific design feedback, preferences, or decisions, record them in `AGENTS.md`.

When implementing from a selected generated mock, treat that image as the source of truth for layout, component anatomy, density, spacing, color, typography, visible content, and hierarchy.

Build app UI in `src/`. Keep `.openai/hosting.json`, `worker/index.js`, `scripts/prepare-sites-build.mjs`, and `tests/sites-worker.test.mjs` intact so the same local prototype can be handed to Sites. Before a Sites handoff, run `npm run build` and `npm run test:sites`; the build must leave `dist/client/index.html`, `dist/server/index.js`, and `dist/.openai/hosting.json`.

## Feature-plan revision for design v0.2

Use RWTC-FEATURES-001 revision 1.0 at ../references/source-docs/windows-time-features.md as the proposed UX contract under architecture 1.4. Distinguish status connection from confirmed service health, keep separate evidence ages and retained recovery provenance, and never turn a refresh request into clock authority. Main/Supporting/Not used are user labels; unapplied entries remain Draft. Default appearance follows the system. Tray and sign-in observer are separate opt-ins. The prototype remains illustrative and does not qualify native execution.
