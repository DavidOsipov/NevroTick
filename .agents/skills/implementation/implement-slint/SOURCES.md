# Slint implementation sources

Reviewed 5 November 2026 for original local instructions. No upstream skill,
implementation or example code is copied. The selected framework comes from
[OD-13](../../../../docs/nevrotick-owner-decisions.md#17-od-13--slint-as-the-default-ui-toolkit);
[S0.5](../../../../tasks/s0.5-ui-comparison.md) owns migration, graph admission
and candidate evidence. These references establish neither dependency adoption
nor licence clearance, application behavior or native qualification.

**Current owner amendment:** OD-13 includes David's approved GPL-3.0-only
licence direction for the original GUI, with Chrony-derived Core/engine retained
under GPL-2.0-only. The inspected source observations below retain their dated
scope. Current graph/composition and execution evidence belongs to S0.5 and the
[GUI handoff](../../../../tasks/slint-gui-handoff.md); the source register supplies
no blanket package, native or release clearance.

## Retrieval and immutable baseline

The user requested Context7 and GitHub MCP. Both were unavailable in the exposed
tool catalog despite the requested integrations; no successful MCP query is
claimed. Actual source reads used official public GitHub REST and raw HTTPS,
with inherited proxy/CA trust and TLS verification retained. No tool installation,
dependency resolution/build, application execution or Windows probe was performed.

GitHub's checked [latest-release response][release-api] identified **v1.18.1**,
published 21 September 2026, with `draft=false` and `prerelease=false`.
The checked [tag ref][tag-api] pointed directly to immutable commit
**`372cf0ee5577c3dfec309a45e7b778ba4e81b734`**. This is the research baseline;
future workers inspect the actual admitted version rather than infer adoption.
Rolling release metadata may change after this dated observation.

## Rust API and documentation lookup

The [Rust API overview](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/slint/lib.rs#L62-L211),
[Weak/event-loop implementation](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/core/api.rs#L1165-L1293)
and [model implementation](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/core/model.rs#L505-L878)
support the [Rust/view reference](references/rust-ui.md); its remaining technical
claims link their exact source sections. The pinned API overview still shows
`1.16.0` dependency examples despite the inspected workspace being 1.18.1. Match
the actual selected runtime/build versions rather than copying that example.
The checked [Rust documentation index](https://docs.slint.dev/latest/docs/rust/slint/)
displayed 1.18.1, but a version-shaped URL redirected to mutable `/latest/`.
Use those pages for discovery and the immutable source for version-sensitive facts.
When Context7/GitHub MCP tools are available, resolve the Slint Rust library,
query the needed API and reconcile the answer with the selected GitHub revision;
keep live-document retrieval separate from dependency or native qualification.

## Translation catalogs

OD-15's [catalog design](../../../../docs/plans/ui-localization.md) uses an
application-owned Rust boundary for separate CBOR resources; that is a design,
not a native Slint file format or an implemented loader. The following additional
HTTPS source checks use the same immutable 1.18.1 revision:

- [Translation guide](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/docs/astro/src/content/docs/guide/development/translations.mdx#L20-L320):
  literal `@tr`, context, indexed/plural placeholders and POT/PO extraction.
  Ordinary Rust strings are not automatically extracted from `.slint` files.
- [Bundling configuration](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/build/lib.rs#L197-L225)
  and [PO reader](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/compiler/translations.rs#L48-L179):
  build-time PO → generated tables, not external CBOR/MO loading.
- [Runtime gettext implementation](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/core/translations.rs#L186-L273)
  and [lookup guard](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/core/translations.rs#L315-L329):
  actual gettext lookup is Unix-gated; the non-Unix binding wrapper returning
  `Ok(())` is not evidence of Windows MO support.
- [Bundled selection](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/core/translations.rs#L380-L462):
  register the first component before selection; exact bundled language names,
  reevaluation and Slint decimal-separator changes have their own behavior.
  This API does not reload application-owned CBOR or copied tray/toast strings.
- [Private API warning](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/slint/private_unstable_api.rs#L4-L8):
  internal translator capabilities or C++ tests are not a stable public Rust
  integration contract. Use ordinary presentation properties/models for the
  selected application-owned catalog design.

No codec, formatter, locale pack or runtime dependency is admitted by these reads.

## Runtime, build and Windows profile

| Inspected source | Finding used and limit |
| --- | --- |
| [Workspace package][workspace] | Version 1.18.1; Rust MSRV 1.92; framework licence expression. Runtime/build crates use edition 2024 and inherit package metadata. Check the repository's admitted toolchain separately. |
| [Runtime defaults and compatibility][runtime-defaults] | Defaults include `std`, `backend-default`, `renderer-femtovg`, `renderer-software`, `accessibility`, `compat-1-2` and `system-tray`. `compat-1-18` is the new mandatory compatibility feature; `compat-1-2` includes it plus historical LinuxKMS feature behavior. |
| [Runtime backend documentation][backends] | Winit supplies event-loop/window integration and supports Windows. Multiple compiled backends/renderers allow runtime selection and fallback; an environment variable alone does not enforce one selected profile. |
| [Software renderer feature][software-feature] | Explicit software rendering is available. Its resource/performance behavior still requires observations on the selected Windows candidate. |
| [Tray feature][tray] | System-tray support is now default. A fixture dependency proposal should state its exclusion instead of silently gaining it through default features. |
| [Build feature definitions][build-features] | `slint-build` independently defaults to `compat-1-18` and `renderer-software`. Build software support enables preprocessing resources; extra image formats are a separate, nondefault feature. |
| [Build execution inputs][build-inputs] | `slint-build` selects compiler Rust generation, diagnostics and translation support; software resource processing adds fontique. Inspect host build inputs as well as target runtime inputs. |
| [Compiler software inputs][compiler-inputs] | Software preprocessing adds image/resvg/swash/skrifa/fontique/rayon inputs. These manifest edges are inventory evidence, not a resolved or cleared transitive graph. |
| [Resource embedding API][embedding] | Absolute runtime paths are for debugging; `EmbedFiles` embeds encoded bytes for runtime decoding; `EmbedForSoftwareRenderer` preprocesses images/fonts and is usable only by the software renderer. |
| [Software font features][fonts] | Software-renderer `std` enables path and system-font support. Do not copy an obsolete separate system-font feature from an older example; still review actual font custody and fallback behavior. |
| [Development control features][dev-features] | Live preview, an MCP HTTP server and remote system-testing control are opt-in development features. Their environment variables activate behavior only when relevant features are compiled. Broad feature selection must not admit them accidentally. |

## Accessibility and lifecycle evidence

| Inspected source | Finding used and limit |
| --- | --- |
| [Accessibility feature wording][accessibility] | The feature attempts OS accessibility exposure; enabling it proves no application-specific UIA/Narrator/keyboard result. |
| [Winit accessibility dependencies][accesskit-deps] and [build guard][accesskit-guard] | AccessKit is selected for non-web accessibility builds, including Windows. Its independently licensed packages remain part of graph review. |
| [AccessKit adapter][accesskit-adapter] | Maintains Slint item/node relationships and owns the AccessKit Winit adapter; native accessibility requests and focus require actual application semantics. |
| [Adapter creation ordering][adapter-order] | The native window is initially hidden so the accessibility adapter exists before display. This is a source mechanism, not evidence from a running Windows fixture. |
| [Software context/surface ownership][surface] and [suspend][suspend] | The renderer owns its softbuffer context/surface; suspend drops surface before context. Observe actual reached teardown and failure paths before claiming resource release. |
| [Hide behavior][hide] and [adapter drop][adapter-drop] | Normal Windows hide sets visibility false; explicit destruction/suspend follows a different path. Adapter drop unregisters the window. Hide/minimize/Close/process exit establish different lifecycle observations. |

Keep native evidence within the existing CHECK-21/22 and UI-M0 ownership.
Source, portable model checks, Windows cross-builds and HTML design tests cannot
substitute for renderer/resource or assistive-technology observations.

## Image/AVIF discrepancies

The [runtime feature documentation][image-feature] lists AVIF among extra formats,
while the [workspace image edge][image-edge] enables only PNG/JPEG by default.
Slint's [native decoding path][image-path] delegates encoded data to the image
crate. The upstream Slint [lock entry][image-lock] names image 0.25.10; this is
upstream's resolution, not an admitted NevroTick graph.

The independently checked image v0.25.10 tag peels to immutable commit
**`76e57184f22772dad1138e96954e57945406b15e`**. Its [features][image-formats]
put `avif` in `default-formats`, but that enables the encoder. `avif-native`
separately adds mp4parse/dav1d, including the native libdav1d dependency.
The [AVIF module][image-avif] separates those guards, and the actual
[decoder match][image-decoder] requires `avif-native`. Consequently Slint's
`image-default-formats` label alone does not establish native AVIF decoding.
Do not add a decoder, convert an asset or claim brand rendering automatically.
The integration owner must choose the actual admitted codec/resource strategy,
preserving original assets, rights, metadata and any derivative provenance.

A second discrepancy is the [compiler comment][compiler-comment] saying that
slint-build enables extra formats by default. The inspected [build feature array
and accompanying documentation][build-features] explicitly exclude them.
Use actual selected feature edges and decoder code when reconciling examples.

## Framework grants and provenance

[LICENSE.md][licence-summary] distinguishes triple-licensed framework/runtime/
tooling, MIT documentation/examples and separately licensed third-party content.
The [listed grant choices][licence-choices] are GPL-3.0-only, Slint Royalty-free
2.0 and Slint Software 3.0. The [royalty-free text][royalty-free] grants specified
uses as part of desktop/mobile/web applications, sets attribution alternatives,
and restricts standalone distribution, embedded use and exposing Slint APIs.
Source licence notices must remain intact. These are reported source terms;
they do not relicense Chrony-derived GPL-2.0-only code or qualify a process split
or the exact runtime/build/transitive distribution graph. The approved original
GUI grant and remaining composition checks are recorded above and in OD-13.

Raw-file SHA-256 values below identify inspected bytes; they are not Cargo archive
checksums, downloaded-package verification or a complete provenance inventory.

| Slint file at the pinned revision | SHA-256 |
| --- | --- |
| `Cargo.toml` | `4d310173b3d954f72731f4635a3c1c570d351295bb25fb364e496e5dd3e44242` |
| `api/rs/slint/Cargo.toml` | `f85edae2d3b000c3b63610890d63898a64ab27674a84e15b0744ec3066d1a9fe` |
| `api/rs/build/Cargo.toml` | `295eab534d64b2b19eb05464dceb16d688a76d786c2aa1ed4763ab48e5bc2838` |
| `LICENSE.md` | `d767912198851eaeaabb8390515ae4eb7e2954f8b39e9cbc4711a5b057b26be0` |

[release-api]: https://api.github.com/repos/slint-ui/slint/releases/latest
[tag-api]: https://api.github.com/repos/slint-ui/slint/git/ref/tags/v1.18.1
[workspace]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/Cargo.toml#L82-L85
[runtime-defaults]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/slint/Cargo.toml#L23-L40
[backends]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/slint/Cargo.toml#L137-L184
[software-feature]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/slint/Cargo.toml#L208-L209
[tray]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/slint/Cargo.toml#L87-L91
[build-features]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/build/Cargo.toml#L20-L40
[build-inputs]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/build/Cargo.toml#L47-L59
[compiler-inputs]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/compiler/Cargo.toml#L41-L59
[embedding]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/build/lib.rs#L76-L95
[fonts]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/renderers/software/Cargo.toml#L18-L36
[dev-features]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/slint/Cargo.toml#L106-L135
[accessibility]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/slint/Cargo.toml#L81-L85
[accesskit-deps]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/backends/winit/Cargo.toml#L64-L64
[accesskit-guard]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/backends/winit/build.rs#L8-L16
[accesskit-adapter]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/backends/winit/accesskit.rs#L29-L67
[adapter-order]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/backends/winit/winitwindowadapter.rs#L595-L599
[surface]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/backends/winit/renderer/sw.rs#L20-L25
[suspend]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/backends/winit/renderer/sw.rs#L209-L212
[hide]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/backends/winit/winitwindowadapter.rs#L1719-L1737
[adapter-drop]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/backends/winit/winitwindowadapter.rs#L2306-L2310
[image-feature]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/slint/Cargo.toml#L99-L104
[image-edge]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/Cargo.toml#L123-L123
[image-path]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/core/graphics/image.rs#L623-L639
[image-lock]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/Cargo.lock#L4051-L4070
[image-formats]: https://github.com/image-rs/image/blob/76e57184f22772dad1138e96954e57945406b15e/Cargo.toml#L67-L92
[image-avif]: https://github.com/image-rs/image/blob/76e57184f22772dad1138e96954e57945406b15e/src/codecs/avif/mod.rs#L6-L18
[image-decoder]: https://github.com/image-rs/image/blob/76e57184f22772dad1138e96954e57945406b15e/src/io/image_reader_type.rs#L179-L183
[compiler-comment]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/compiler/Cargo.toml#L51-L54
[licence-summary]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/LICENSE.md#L4-L13
[licence-choices]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/LICENSE.md#L17-L35
[royalty-free]: https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/LICENSES/LicenseRef-Slint-Royalty-free-2.0.md#L9-L37
