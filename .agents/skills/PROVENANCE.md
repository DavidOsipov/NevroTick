# Adaptation provenance

Prepared for NevroTick on 28 October 2026 with AI assistance. The primary donor
is David Osipov's repository skill collection in
`personal-website-templated-1/.agents`. [ADAPTATION.json](ADAPTATION.json) records
each original entrypoint and digest. Original source files remain separate from
these Rust and Windows adaptations.

The additional donor is David Osipov's [PdfCraft PR 2912](https://github.com/storytold/pdfcraft/pull/2912),
reviewed at the immutable head
[`c9a02b378271f92a73cb44be9fd9e4acc6921516`](https://github.com/DavidOsipov/pdfcraft/tree/c9a02b378271f92a73cb44be9fd9e4acc6921516/.agents/skills/QA).
The PR was closed without merging; this is a record of source material, not an
upstream endorsement. Its Rust workflow, data-flow reasoning, persistence and
commit-preparation guidance informed the applicable implementation, planning,
security and testing references. PDF-specific assumptions and application code
were not carried into NevroTick.

David's contribution attributes original skill material to David Osipov,
personal@david-osipov.vision, [david-osipov.vision](https://david-osipov.vision),
and grants MIT permission for his owned material. Its full
[copyright and permission notice](provenance/PDFCRAFT-SKILL-LICENSE.txt) is
retained from the pinned
[source notice](https://github.com/DavidOsipov/pdfcraft/blob/c9a02b378271f92a73cb44be9fd9e4acc6921516/.agents/skills/QA/pdfcraft-rust-implementation/THIRD_PARTY_NOTICES.md).
This does not select or replace the application's license, extend the grant to
third-party material, or resolve the architecture's dependency license gates.

The website's original [donor notices and license texts](provenance/WEBSITE-NOTICES.md)
remain available for provenance. Those notices preserve source-specific terms
and uncertain license status; they do not impose a new license on user-authored
work. That website/PdfCraft adaptation did not bundle upstream runtimes, private
exports, fonts, document corpora or third-party skill implementations. Group
`SOURCES.md` files distinguish researched tools from adopted product decisions.
Links to standards and upstream projects remain references to those owners' material.

Four additional [upstream instructional skills](upstream/README.md) are now
vendored separately under `upstream/`. Their BSD-3-Clause and MIT copyright and
permission notices remain in each leaf's `LICENSE`; David's grant does not
replace those licenses or assert authorship of those copies. Immutable source
commits, original source digests, installed-file hashes and local-change notes
are recorded by [UPSTREAM.json](UPSTREAM.json). No upstream runtime, plugin/MCP
configuration or executable helper was installed with those documents.

Selected method adaptations retain these sources and notices:

- Superpowers at commit `2ce8aa15d05bd57213e6f746f953aafb7ded7a8b`:
  [systematic debugging](https://github.com/obra/superpowers/blob/2ce8aa15d05bd57213e6f746f953aafb7ded7a8b/skills/systematic-debugging/SKILL.md),
  [root-cause tracing](https://github.com/obra/superpowers/blob/2ce8aa15d05bd57213e6f746f953aafb7ded7a8b/skills/systematic-debugging/root-cause-tracing.md)
  and [review intake](https://github.com/obra/superpowers/blob/2ce8aa15d05bd57213e6f746f953aafb7ded7a8b/skills/receiving-code-review/SKILL.md)
  inform bounded redacted boundary diagnosis and evidence-based feedback handling.
  Jesse Vincent's [full MIT notice](provenance/licenses/superpowers/LICENSE) is retained.
- GSD's [pause-work method](https://github.com/gsd-build/get-shit-done/blob/bdcaab2c752d9a33a1a1ca9acf3a3c81fb991815/get-shit-done/workflows/pause-work.md),
  at commit `bdcaab2c752d9a33a1a1ca9acf3a3c81fb991815`, informs observed-failure
  handoffs in context checkpoints. Lex Christopherson's
  [full MIT notice](provenance/GSD-LICENSE.txt) is retained.
- The optional [defect-variant reference](testing/references/defect-variants.md)
  adapts Axel Mierczuk / Trail of Bits' variant analysis at commit
  `bf23bfb21f10a2a4b0406086d690dc8cb1b3ccf3`. It carries its own attribution,
  modification notice and [full CC BY-SA 4.0 license](provenance/licenses/trailofbits/LICENSE).
  Its terms do not select the product license or relicense independent documents
  that link to it.

These adaptations exclude donor installers, runtimes, automatic commits and
unrelated workflow rules. BMAD and its official Test Architect were researched
without copying their implementations.

When copying a skill outside this repository, include the shared contract and
references it actually needs and retain applicable copyright and permission
notices. Do not describe an isolated leaf as self-contained without its links.
