# GUI source licensing and provenance

The original first-party source in `nevrotick-ui` is licensed under the
**GNU General Public License, version 3 only (`GPL-3.0-only`)**. This implements
David Osipov's GUI-specific licence decision under
[OD-11](../../docs/nevrotick-owner-decisions.md#15-od-11--replace-the-win32-ui-with-slint).
It covers the original GUI application, fixture model and presentation code,
GUI adapters, Slint views and GUI-specific tests in this crate. Copyright remains
with the respective authors. Existing third-party notices and grants take
precedence for the material they identify.

This declaration does not change the licence of Chrony, Chrony-derived code,
Core or other workspace crates. Their `GPL-2.0-only` provenance remains in force.
It also does not withdraw permissions already granted for earlier GUI versions.

## First-party source inventory

The source review started at repository commit
`d0e6ca21c12eca0d55fee5e536d46672c0792a9f` and included the subsequent S0.5 GUI
source additions. The migration and exact integrated candidate remain owned by
[S0.5](../../tasks/s0.5-ui-comparison.md).

| Source | Provenance and scope |
| --- | --- |
| `src/lib.rs`, `src/main.rs`, `src/model.rs`, `src/native.rs`, `src/native/controls.rs`, `src/native/view.rs`, `tests/preview.rs` and the original crate manifest | Introduced by David Osipov in commit `8a7d9a9839687eb72f38c56837b957ce095b5e0c`, “Add fixture-only native UI and isolated Rust verification”. Its message records adaptation of the supplied design into an original native UI-M0. The retained Win32 adapter is migration source, not a separate third-party toolkit implementation. |
| `src/presentation.rs`, `tests/presentation.rs` and original `ui/*.slint` components | First-party S0.5 GUI work over the existing checked fixture model. These implement presentation, navigation and fixture editing, using the [v0.4 design](../../docs/nevrotick-design-v0.4/README.md) as the composition reference. Their original source falls within the GUI-specific GPL-3.0-only grant. |
| Later original GUI bindings and adapters in this crate | The same GUI-specific grant applies, subject to the provenance and dependency review below. Adding a file does not grant rights to third-party material copied into it. |

The pre-Slint Rust imports use the standard library, GUI-local modules,
`garde` and Windows `windows-sys`; the new bindings add Slint. Neither GUI manifest links
`time-core`, `time-domain`, `ntp-wire`, `ntp-exchange`, `nevrotick-engine` or other
workspace crates. The inspected model contains bounded sample-field validation
and in-memory edits; no Chrony algorithm, Chrony source inclusion or third-party
copy/adaptation notice was found. The About page's Chrony description is an
informational provenance statement, not a source import or runtime dependency.

Commit authorship, import inspection and the absence of notices are source
evidence; they do not establish ownership of undisclosed external material.
Preserve any discovered notices and resolve their actual grants before adopting
such material. New Core linkage or copied Chrony-derived logic needs a fresh
compatibility review; the GUI decision supplies no permission to relicense it.

## Framework, dependencies and resources

- Slint is third-party framework code. The proposed 1.18.1 source baseline is
  `372cf0ee5577c3dfec309a45e7b778ba4e81b734`; its GPL-3.0-only option is the
  intended framework grant for this GPLv3 GUI. Slint runtime, compiler,
  proc-macros, generated widget code and their dependencies retain their own
  copyright and applicable notices. This document does not relicense them.
- The current Slint views import buttons, text editors, scrolling, checkboxes
  and comboboxes from
  `std-widgets.slint`. These are framework inputs supplied by Slint, not vendored
  first-party components. Generated output can contain framework-derived
  material; it must not be described as wholly original GUI code.
- The existing `garde` and `windows-sys` inputs, the proposed Winit/software
  renderer and AccessKit path, and every future runtime/build dependency retain
  their own licences. An Apache-2.0 dependency can be compatible with GPLv3;
  that fact supplies no complete graph, notice, security or distribution pass.
  Exact versions, features, archives, execution inputs and package notices remain
  subject to the existing dependency-admission and S0.5 evidence chain.
- The Windows view names the system font `Segoe UI`; it redistributes no
  Microsoft font. The optional private Linux rendering fixture separately
  includes unmodified Inter from `i-slint-common` 1.18.1, its original sidecar
  and complete OFL text under [`tests/fixtures/fonts`](tests/fixtures/fonts/README.md).
  These third-party font bytes remain OFL-licensed. Their test-only use supplies
  no Windows font or final resource qualification.
- The compatible [`spin_on` repair](../../third-party/spin_on/NEVROTICK-PATCH.md)
  is separately licensed third-party compiler/macro dependency source. Its
  original MIT/Apache notices are preserved, with MIT selected; the original
  GUI's GPL3 declaration does not replace its grant. Exact modified source,
  routing and dependency qualification remain owned by S0.5.
- NevroTick artwork and brand assets retain the separate
  [brand terms](../../docs/nevrotick-design-v0.4/assets/BRAND-LICENSE.txt).
  The fixed `ui/assets/head.png` is an unchanged supplied PNG, matched by SHA-256
  to the AVIF kit's recorded original pixel source. Its provenance, original
  brand licence and preserved kit font notice accompany it in
  [ui/assets](ui/assets/README.md). The GUI's GPL-3.0-only grant does not
  relicense those proprietary artwork bytes.
  Their font and other third-party notices remain separate. The GUI source
  licence grants no additional artwork or trademark rights.
- The localization repertoire tables are derived from pinned Unicode 15.1 /
  Emoji 15.1 data. Their original Unicode licence and exact source provenance
  accompany the initial codec in `src/localization/unicode`; this data is
  separately licensed and is not relicensed by the GUI's GPL-3.0-only grant.

Keep the GUI and GPL-2.0-only components' grants and notices distinct in any
combined package. A separate process or package entry alone does not establish
that two programs are independent works. Review the actual code reuse, linkage
and integration before making a distribution compatibility claim.

## Licence-text custody

[LICENSE](LICENSE) contains the unmodified full GNU GPL version 3 text, copied
byte-for-byte from the official Slint repository at the immutable source above:

<https://raw.githubusercontent.com/slint-ui/slint/372cf0ee5577c3dfec309a45e7b778ba4e81b734/LICENSES/GPL-3.0-only.txt>

The retrieved file is 34,670 bytes with SHA-256
`b6a8bf63ae2c95b0aa256ee774680d7bd18f21f457da75466f7610b3b16acc4e`.
The text is the standard GPLv3, without a GUI-specific exception or added
restriction. `GPL-3.0-only` is the selected grant for the original GUI; the
licence's example instructions for applying it to other programs do not change
that selection. The upstream file's location supplies text provenance, not a
claim that Slint owns this crate's original source.
