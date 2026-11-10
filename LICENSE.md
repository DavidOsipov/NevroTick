# Licensing map

NevroTick contains material under different licences. This file summarizes the
existing scoped grants and points to their terms and provenance. It does **not**
relicense any material, replace its notices or grant additional rights. Do not
interpret the entries below as a choice of licence for the entire repository.

## Application source

| Scope | Existing licence and terms |
| --- | --- |
| Core, engine and other crates inheriting the workspace licence | **GPL-2.0-only**, as declared in [Cargo.toml](Cargo.toml). Complete GNU GPL version 2 texts are retained in [ntp-wire](crates/ntp-wire/COPYING), [ntp-exchange](crates/ntp-exchange/COPYING), [time-domain](crates/time-domain/COPYING), [time-core](crates/time-core/COPYING) and [nevrotick-engine](crates/nevrotick-engine/COPYING). The `windows-platform` manifest also inherits the workspace licence. Preserve applicable upstream attribution and notices. |
| Original first-party GUI source in `nevrotick-ui` | **GPL-3.0-only**, as declared in its [manifest](crates/nevrotick-ui/Cargo.toml). See the complete [GNU GPL version 3 text](crates/nevrotick-ui/LICENSE) and the [scope and provenance record](crates/nevrotick-ui/LICENSING.md). This grant does not relicense Core, Chrony-derived code, framework code or artwork, and does not withdraw permissions already granted for earlier GUI versions. |

The [source-derived-port decision (OD-01)](docs/nevrotick-owner-decisions.md#3-od-01--source-derived-port-with-independent-verification)
retains the current Chrony-derived GPL-2.0-only obligations. The planned
independent replacement and future public distribution licence are not a current
licence change or an upstream permission grant.

The [GUI decision (OD-13)](docs/nevrotick-owner-decisions.md#gui-licencecomposition-decision--6-november-2026)
keeps the GPL-3.0-only GUI grant separate from GPL-2.0-only Core/engine. Actual
code reuse, linkage, shared code, dependencies and distribution composition need
their applicable compatibility and notice review. A process or package split
alone does not establish independent works or distribution compatibility. This
map is not a release or whole-product licence clearance.

## Artwork and brand assets

NevroTick artwork and brand assets retain their separate proprietary terms:

- [AVIF brand kit terms](docs/NevroTick-AVIF-kit-v1/BRAND-LICENSE.txt).
- [Design v0.4 asset terms](docs/nevrotick-design-v0.4/assets/BRAND-LICENSE.txt).
- [GUI asset provenance and accompanying notices](crates/nevrotick-ui/ui/assets/README.md).

The source-code licences do not grant additional artwork or trademark rights.
Third-party font notices remain separate from the brand terms.

## Third-party and imported material

Dependencies, imported source, data, fonts, tools and skills retain their own
applicable licences, copyright notices and provenance records. In particular:

- [GUI licensing and provenance](crates/nevrotick-ui/LICENSING.md) identifies
  framework, generated-code, dependency and resource boundaries.
- [GUI Unicode data licence](crates/nevrotick-ui/src/localization/unicode/LICENSE.txt)
  and [fixture font provenance](crates/nevrotick-ui/tests/fixtures/fonts/README.md)
  accompany the material they cover.
- Vendored [spin_on](third-party/spin_on/NEVROTICK-PATCH.md),
  [accesskit_windows](third-party/accesskit_windows/NEVROTICK-PATCH.md) and
  [accesskit_consumer](third-party/accesskit_consumer/NEVROTICK-PATCH.md) retain
  their licence files, notices and local patch records.
- [IANA leap-data terms](tools/leap-data/LICENSE.iana),
  [UI text tooling terms](scripts/UI-TEXT-LICENSE.md),
  [dependency notices](policy/dependency-notices/) and
  [skill provenance](.agents/skills/PROVENANCE.md) document additional scoped
  material; this list is not a substitute for the notices accompanying each file.

GitHub's automatic licence detection may not recognize this multi-licence map as
one repository-wide licence. The scoped terms and accompanying notices, not a
repository badge or checklist, determine the applicable grants.
