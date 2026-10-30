# NevroTick
<img width="1300" height="576" alt="nevrotick-illustrated-for-light" src="https://github.com/user-attachments/assets/a594b72f-adda-45dd-ac07-925476f23bac" />

**System essentials, under control.**

NevroTick is my personal Rust project for secure timekeeping and, later,
encrypted DNS. It starts on **Windows 11 x64**, with a small native interface
and chrony-derived numerical algorithms.

## Why I am building this

Windows 11's built-in time synchronization has repeatedly let me down, including
on my own laptop. I am building NevroTick for myself first: dependable
timekeeping, clear status and modest resource use.

Thank you to the developers of
[BktTimeSync](https://www.maniaradio.it/en/bkttimesync.html) for their work, which I used for a year almost.
My goal is a simpler app with **NTS** (Network Time Security).
GPS synchronization, external reference-clock hardware and amateur-radio
integrations are outside the planned timekeeping scope.

## What is being built

- **Windows first:** authenticated network time, chrony-derived estimation,
  source agreement and a lightweight native UI. The planned engine and clock
  broker have separate privileges and ship in one installer.
- **Android next:** an NTS-backed companion supplying a local SNTP endpoint to
  Android's existing time service, subject to device qualification.
- **Encrypted DNS later:** **DNS over QUIC (DoQ)** with leaf and intermediate
  certificate pinning. DoH and DoT are outside this feature.

See the [product roadmap](ROADMAP.md) and
[Windows feature plan](docs/windows-time-features.md) for the design and open work.

## AI-assisted development and learning

AI coding agents develop the project; I act as **architect and guide**, defining
requirements and directing architecture, implementation and verification.
This is also my study project in secure Rust development for Windows and Android,
applying security best practices and independently testing the results.

You may find useful workflows in [.agents/skills/](.agents/skills/): secure Rust,
threat modeling, CodeQL data-flow analysis, adversarial testing and optimization.
Start with the [skills index](.agents/skills/README.md) for ideas to adapt.

## Project status

**Early development — the current software does not synchronize your clock.**

The workspace contains a sample-data Windows UI and portable protocol/numerical
fixtures, with no live time-server traffic, services or clock changes.
NTS, Android, DoQ, pinning and the complete installer remain future work.
See the [development plan](docs/plans/development-plan.md) and
[UI-M0 task](tasks/ui-m0.md) for current results and qualification gaps.

## Repository overview

| Crate | Purpose |
| --- | --- |
| [nevrotick-ui](crates/nevrotick-ui/) | Sample-data model and native Windows UI prototype. |
| [ntp-wire](crates/ntp-wire/) | Bounded NTP headers and packet framing. |
| [time-domain](crates/time-domain/) | Checked timestamp, era, offset and elapsed-time arithmetic. |
| [ntp-exchange](crates/ntp-exchange/) | Unauthenticated fixture association and deterministic scenarios. |
| [time-core](crates/time-core/) | Regression, residuals, normalization, bounded history and fixture selection. |

## Getting started with development

```sh
git clone https://github.com/DavidOsipov/NevroTick.git
cd NevroTick
```

Follow [TOOLING.md](TOOLING.md#starting-implementation) to configure the pinned
Rust **1.99.1** toolchain, Python **3.11+**, dependencies and verification tools.
Then run:

```sh
python3 -B scripts/verify.py
```

This checks formatting, compilation, Clippy, documentation and selected tests.
TOOLING.md also covers dependency checks and Windows cross-builds for
`x86_64-pc-windows-msvc`; native Windows testing is separate.

## Documentation and contributing

| Topic | Start here |
| --- | --- |
| Documentation map | [Guide](docs/README.md) |
| Architecture and decisions | [Architecture](docs/windows-time-architecture.md), [owner decisions](docs/nevrotick-owner-decisions.md) |
| UI and artwork | [Design v0.2](docs/nevrotick-design-v0.2/README.md), [brand kit](docs/NevroTick-AVIF-kit-v1/README.md) |
| Vulnerability reporting | [SECURITY.md](SECURITY.md) |

Contributors should read [AGENTS.md](AGENTS.md) and the
[testing contract](.agents/skills/testing/CONTRACT.md), preserve upstream
attribution and include relevant verification results. Use
[GitHub Issues](https://github.com/DavidOsipov/NevroTick/issues) for ordinary bugs
and suggestions; report sensitive issues privately through SECURITY.md.

## Author

**Author:** This project was architected and directed by **David Osipov**, an
AI-Driven B2B Lead Product Manager. You can learn more about my work and philosophy
at [david-osipov.vision](https://david-osipov.vision/).

- **ISNI:** 0000 0005 1802 960X ([International Standard Name Identifier](https://isni.org/isni/000000051802960X))
- **ORCID:** [0009-0005-2713-9242](https://orcid.org/0009-0005-2713-9242)
- **VIAF:** [139173726847611590332](https://viaf.org/viaf/139173726847611590332/)
- **Wikidata:** [Q130604188](https://www.wikidata.org/wiki/Q130604188)
- **Contact:** [personal@david-osipov.vision](mailto:personal@david-osipov.vision)

## License and acknowledgements

**License:** [GNU GPL v2.0 only](crates/ntp-wire/COPYING), SPDX identifier
[`GPL-2.0-only`](https://spdx.org/licenses/GPL-2.0-only.html), as declared in
[Cargo.toml](Cargo.toml).

Chrony-derived code retains its upstream notices and
[provenance requirements](docs/nevrotick-owner-decisions.md#3-od-01--source-derived-port-with-independent-verification).
Dependencies and imported material retain their own notices; artwork has
[separate usage terms](docs/NevroTick-AVIF-kit-v1/BRAND-LICENSE.txt).
