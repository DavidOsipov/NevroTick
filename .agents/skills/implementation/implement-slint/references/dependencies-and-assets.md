# Slint dependencies, licences and assets

Read before proposing Cargo, compiler, renderer, codec, font or bundled-asset
changes. Follow [dependency admission](../../../environment/dependency-updates/SKILL.md)
and the [UI migration task](../../../../../tasks/ui-m0.md#slint-migration-handoff--5-november-2026).
This guide supplies source evidence; it admits no product dependency or licence route.

## Contents

- [Establish the candidate](#establish-the-candidate)
- [Separate dependency graphs](#separate-dependency-graphs)
- [Preserve the licence hold](#preserve-the-licence-hold)
- [Choose an asset path deliberately](#choose-an-asset-path-deliberately)
- [Select embedding and fonts](#select-embedding-and-fonts)
- [Review the resulting candidate](#review-the-resulting-candidate)

## Establish the candidate

The source baseline inspected on **5 November 2026** is Slint **1.18.1**, commit
[`372cf0ee5577c3dfec309a45e7b778ba4e81b734`](https://github.com/slint-ui/slint/tree/372cf0ee5577c3dfec309a45e7b778ba4e81b734).
The [release](https://github.com/slint-ui/slint/releases/tag/v1.18.1) was published
on 21 September 2026. Recheck the selected version when doing future work;
`latest`, a search index and upstream `master` can describe different APIs.

[OD-13](../../../../../docs/nevrotick-owner-decisions.md#17-od-13--slint-as-the-default-ui-toolkit)
selects Slint and the measured winit/software/Fluent starting configuration.
M0-SL-01 still owns exact graph and licence admission. Source preparation can
record a concrete blocker; that does not unblock M0-SL-03 implementation.

Inspect the real UI manifest, lockfile, toolchain, build script and current
task before changing versions. Keep `slint` and `slint-build` on the same selected
release and verify the resolution, including generated-code compatibility.
The inspected [workspace manifest](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/Cargo.toml)
declares Rust **1.92**; this is a version-specific minimum, not permission to
change NevroTick's toolchain. Retain existing locked-build and custody rules.

## Separate dependency graphs

Inventory dependencies by execution and distribution role; one feature list
does not establish all four graphs below.

| Graph | Record and verify |
| --- | --- |
| Windows runtime | Exact Slint/winit/renderer versions, features, transitive/native libraries, supported target and distributed payloads. |
| Host build and proc macros | `slint-build`, compiler/macros, build-host codecs/fonts and generated Rust; inspect host execution and licence obligations separately. |
| Development and tests | Preview, interpreter, testing backend and optional introspection features; prevent their accidental inclusion in the release graph. |
| Assets | Source and derivative files, fonts, notices, licences, dimensions, hashes and actual embedding/package paths. |

Use the repository's admitted Cargo inspection/check entry points for the exact
`x86_64-pc-windows-msvc` package and feature selection. Inspect feature
unification and host/target edges, not just the direct dependency declaration.
Do not run `--all-features` or add dependencies merely to make an example compile.

The inspected [runtime manifest](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/slint/Cargo.toml)
defaults to `std`, `backend-default`, `renderer-femtovg`, `renderer-software`,
`accessibility`, `compat-1-2` and `system-tray`. A plain dependency declaration
therefore does not express the selected minimal winit/software profile.

- Select the admitted backend and renderer explicitly; `backend-winit` enables
  `std`. Account for every retained default or deliberately disabled feature.
- In this release, `compat-1-2` includes `compat-1-18` and conditional LinuxKMS
  compatibility edges. Do not delete the mandatory compatibility feature or
  treat an alias name as proof of a particular final graph.
- Keep accessibility selection with its current owner. D3 is deferred, and the
  pilot's omitted accessibility feature is not permanent product policy.
- `system-tray` already enables Slint's `SystemTrayIcon`; choose it only when
  the admitted milestone needs a tray. It does not authorize tray behavior now.
- Add `raw-window-handle-06` only for an actual reviewed native adapter need.
  Window handles retain their platform and lifetime obligations.

Do not inherit Qt, Skia, extra GPU renderers, a Tokio runtime or all image codecs
from a demo. Tokio's engine selection grants no UI-runtime dependency. If the
software profile cannot meet an actual requirement, document the exact failed
requirement for its owner instead of silently switching the renderer.

Keep backend/renderer and widget style distinct. The inspected
[style guide](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/docs/astro/src/content/docs/reference/std-widgets/style.mdx)
defines `fluent` as a Slint style and Windows `native` as its alias; this does not
instantiate WinUI or inherited Win32 child controls. Record the compile-time
style setting and actual renderer used for every relevant observation.

An environment request such as `SLINT_BACKEND=winit-software` is not evidence
that the requested renderer was used: the pinned feature documentation describes
fallback when a selection is unavailable. Check selection errors and the actual
backend/renderer. The [renderer guide](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/docs/astro/src/content/docs/guide/backends-and-renderers/backends_and_renderers.mdx)
lists software-renderer limits involving item transforms, shadows, rounded
clipping, text outlining and text scripts. Verify the required design and glyphs
on the selected candidate; do not infer support from a browser or GPU demo.

## Preserve the licence hold

Read the exact [licence map](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/LICENSE.md),
selected grants, per-file headers and REUSE metadata. Framework, example and
asset licences are separate; a repository badge cannot qualify the binary.

| Material or route | Source-grounded obligation to resolve |
| --- | --- |
| Slint framework | Select and record an applicable GPL-3.0-only, Royalty-free 2.0 or Software 3.0 grant; selection is not implicit. |
| Documentation/examples | Generally MIT at this revision; inspect copied files and retain required notices. |
| Third-party source/assets | Follow their own headers/REUSE metadata and transitive licence inventory. |
| NevroTick brand assets | Preserve their proprietary asset notices independently of the application's code licence. |

The [Royalty-free 2.0 text](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/LICENSES/LicenseRef-Slint-Royalty-free-2.0.md)
covers desktop/mobile/web applications with conditions, excludes embedded
systems, and requires one of its specified attribution routes. An `AboutSlint`
widget must be reachable as described by that grant; the alternative public
badge route has its own placement requirement. Also inspect its limitations
on standalone redistribution, exposed APIs and retained notices.

Keep M0-SL-01's Slint and Apache-2.0-only winit compatibility questions unresolved
until the exact route is admitted against retained chrony/GPL-2.0-only
obligations. A framework grant, source-file MIT header, process split or owner
toolkit preference is not a blanket compatibility finding. Record a concrete
OD-04 blocker/card where needed; infer no paid agreement or licence exception.

## Choose an asset path deliberately

Start from the [AVIF brand kit](../../../../../docs/NevroTick-AVIF-kit-v1/README.md)
and its [manifest](../../../../../docs/NevroTick-AVIF-kit-v1/manifest.json).
Keep original assets canonical. Do not assume a browser's ability to decode
AVIF proves the chosen native Slint build can decode it.

**AVIF requires an explicit decoder check.** The inspected
[Slint image guide](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/docs/astro/src/content/docs/reference/property-types/images.mdx)
lists AVIF under `image-default-formats`, but
[Slint's core manifest](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/core/Cargo.toml)
only forwards that feature to `image/default-formats`.
The inspected [lockfile](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/Cargo.lock)
uses `image 0.25.10`. Its [codec documentation](https://docs.rs/image/0.25.10/image/codecs/index.html)
requires `avif-native`/libdav1d for decoding, while its
[feature graph](https://docs.rs/crate/image/0.25.10/features) enables the ravif
encoder through default `avif`; `dav1d` and `mp4parse` are separate dependencies.
That flag alone therefore does not establish native AVIF decoding. Recheck this
source discrepancy against the eventual graph and an actual asset fixture.

Prefer appropriately sized, deterministic **offline PNG derivatives** for
bundled branding when they satisfy the design. This avoids admitting runtime
AVIF codecs solely for fixed artwork. If AVIF is required, qualify its actual
decoder, target support, native payload, failure handling and licence graph.
Treat this as a recommendation, not a change to the canonical brand kit.

For each derivative, retain source hash/path, conversion tool/version/options,
output hash, dimensions, alpha/color handling and applicable author/AI/licence
notices. Preserve the notices in the asset inventory if conversion changes
embedded metadata. Inspect the result at intended sizes with the selected
renderer; bound decoded dimensions and pixel memory, not only file size.

## Select embedding and fonts

Use the public names from the inspected [build API](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/build/lib.rs):

| `EmbedResourcesKind` | Consequence |
| --- | --- |
| `AsAbsolutePath` | Loads from build-machine absolute paths at runtime; debugging only, unsuitable as an installed-asset delivery contract. |
| `EmbedFiles` | Embeds encoded files as-is and decodes them at runtime; prove runtime codec availability. |
| `EmbedForSoftwareRenderer` | Pre-processes images/fonts into pixel data at build time; requires its build feature and works only with the software renderer. |

`EmbedAllResources` is an internal compiler mapping, not the public build-API
variant at this revision. Do not copy internal enum names into product examples.
Inspect [build-helper features](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/build/Cargo.toml)
separately: build-time codec support is not implied by a runtime feature.

Do not use `CompilerConfiguration::with_scale_factor` as a desktop screenshot or
DPI workaround. The same build API specifies it for MCU use and says it disables
runtime scale-factor changes. Keep runtime desktop scaling possible; full D1
qualification remains at its existing trigger.

The [font guide](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/docs/astro/src/content/docs/guide/development/fonts.mdx)
supports system fonts and imported TTF/TTC/OTF files. Check actual required
glyphs, fallback, font redistribution rights and selected embedding sizes.
Do not bundle operating-system fonts merely because they are installed locally.
Its `software-renderer-systemfonts` advice is stale for this baseline: the
runtime manifest marks that feature deprecated because `std` plus
`renderer-software` already enables system fonts. Resolve source conflicts by
version, then observe the actual candidate; do not silently broaden D1 work.

## Review the resulting candidate

Record the exact graph, grants, build/style/embedding configuration, assets and
native payloads in the existing admission/task record. Preserve generated-code
unsafe/lint review and installed-package/resource gates. Source inspection and
upstream examples establish neither a build pass nor Windows behavior.
Use [testing and performance](testing-and-performance.md) for evidence layers
and optional development tooling; keep release and debug graphs separate.
