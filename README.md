# NevroTick
<img width="1300" height="576" alt="nevrotick-illustrated-for-light" src="https://github.com/user-attachments/assets/a594b72f-adda-45dd-ac07-925476f23bac" />

**System essentials, under control.**

> **Early development:** NevroTick does not yet synchronize your system clock.
> The GUI uses sample data; no production-supported stable release is declared.

NevroTick is my personal Rust project for secure timekeeping and, later,
encrypted DNS. Its intended initial platforms are **Windows 11 26H2 x64** and
**Windows Server 2025 x64 with Desktop Experience**, with **Slint** as the selected
native UI toolkit and chrony-derived numerical algorithms. The
[platform matrix](tasks/windows11-integration.md#432-two-family-windows-qualification-matrix--5-november-2026)
records the remaining qualification. The selected Windows CPU baseline is
**x86-64-v3**; native hardware/OS qualification remains open. These are intended
product requirements, not a claim of tested platform support.

## Why I am building this

Windows 11's built-in time synchronization has repeatedly let me down, including
on my own laptop. I am building NevroTick for myself first: a relatively simple
Windows utility that prioritizes security, reliability, clear status and a very
small CPU, memory and disk footprint over a large feature set. A Linux port is
not planned; chrony and NTPsec already serve that ecosystem. New features require
an explicit decision and careful, potentially lengthy implementation and
qualification; a roadmap entry is not a promise to ship.

Thank you to the developers of
[BktTimeSync](https://www.maniaradio.it/en/bkttimesync.html) for their work; I used BktTimeSync for almost a year.
My goal is a simpler app with **NTS** (Network Time Security).
GPS synchronization, external reference-clock hardware and amateur-radio
integrations are outside the planned timekeeping scope.

## What is being built

- **Windows focus:** authenticated network time, chrony-derived estimation,
  source agreement and a lightweight Slint UI. The planned engine and clock
  broker have separate privileges and ship in one complete MSIX. The selected
  [packaging plan](docs/plans/windows-msix-store.md) targets strong process and
  package protections, followed by a Microsoft Store submission with services;
  packaging, native qualification and Store acceptance remain future work.
- **Longer-term plans:** an Android companion and **DNS over QUIC (DoQ)**
  with leaf/intermediate certificate pinning remain planned, not implemented or
  promised for a particular release. Their existing roadmap decisions, design
  work and qualification gates apply. DoH and DoT are outside the DNS scope.

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

[Slint is the owner-selected default](docs/nevrotick-owner-decisions.md#17-od-13--slint-as-the-default-ui-toolkit).
The checked-in sample-data GUI uses Slint and the [current v0.4 design](docs/nevrotick-design-v0.4/README.md).
Its [handoff](tasks/slint-gui-handoff.md) records the implemented fixture workflows,
original GUI GPL-3.0-only grant, branch checks and remaining backend, native and
critical-text approval gaps. Combined-tree qualification remains separate.

The seven-member Rust workspace contains portable protocol/numerical fixtures,
bounded loopback acquisition and address rounds, portable service contracts and
the sample-data UI. Current Core work includes finite phase plans and
achieved-baseline feedback. These are development components, not an integrated
time synchronization service.

The Engine's default-off NTS prototype now owns the strict Windows KE path,
private associations, protected-request custody and authenticated-response
admission. Its existing probe remains one-shot: the default constructs a
never-sent preview, while explicit `--protected-datagram` selects one bounded
UDP operation. The [12 November Engine checkpoint](tasks/engine-transport-pilot.md#materialized-cutoff-control-gnu-checkpoint--12-november-2026)
records bounded recovery and receipt/history custody source, scoped component
evidence and the remaining gates; recurring native operation is not enabled by
the probe.

Experimental results remain ineligible for clock control. The complete native
KE/TLS/CNG/UDP journey, production NTP/NTS acquisition and Core admission,
installed services, clock changes, Android, DoQ, pinning and the complete
installer remain future work.

See the [implementation summary](docs/windows-time-architecture.md#1-current-state-capsule)
and [task index](tasks/README.md) for component evidence and qualification gaps.
The [development plan](docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent)
owns the current integration checkpoint and next bounded task. Test, coverage
and mutation results apply to their recorded candidate and scope; they do not
establish end-to-end security, Windows-native behavior or release readiness.

No production-supported stable release is declared. The
[release policy](docs/release-policy.md) records current support status, the
proposed first-stable baseline and the preparation required for bad-release recovery.

## Repository overview

| Crate | Purpose |
| --- | --- |
| [nevrotick-ui](crates/nevrotick-ui/) | Sample-data model and native Windows UI prototype. |
| [ntp-wire](crates/ntp-wire/) | Bounded NTP decoding/framing and inert NTS-KE negotiation/encoding and NTS envelope/plaintext readers. |
| [time-domain](crates/time-domain/) | Checked timestamp, era, offset and elapsed-time arithmetic. |
| [ntp-exchange](crates/ntp-exchange/) | Unauthenticated fixture association and deterministic scenarios. |
| [time-core](crates/time-core/) | Pure time-estimation, source-selection/combination and modeled correction fixtures. |
| [nevrotick-engine](crates/nevrotick-engine/) | Bounded experimental loopback/address acquisition and a feature-gated NTS-KE/protected-datagram prototype; no production service or clock authority. |
| [windows-platform](crates/windows-platform/) | Portable service lifecycle, SCM projection and exact-role privilege contracts; laboratory consumers remain separate from production/native qualification. |

## Getting started with development

```sh
git clone https://github.com/DavidOsipov/NevroTick.git
cd NevroTick
```

Follow [TOOLING.md](TOOLING.md#starting-implementation) to configure the pinned
Rust **1.99.1** toolchain, Python **3.11+**, dependencies and verification tools.
Once the documented prerequisites and tool-admission checks are satisfied, run:

```sh
python3 -B scripts/verify.py
```

This runs the configured verification entry point for formatting, compilation,
Clippy, documentation and selected tests. It is not an installation command or
a complete release-qualification run.
TOOLING.md also covers dependency checks and Windows cross-builds for
`x86_64-pc-windows-msvc`; native Windows testing is separate.

## Documentation and contributing

| Topic | Start here |
| --- | --- |
| Documentation map | [Guide](docs/README.md) |
| Architecture and decisions | [Architecture](docs/windows-time-architecture.md), [owner decisions](docs/nevrotick-owner-decisions.md) |
| UI and artwork | [Current design v0.4](docs/nevrotick-design-v0.4/README.md), [interactive preview](docs/nevrotick-design-v0.4/NevroTick-UI-v0.4.html), [brand kit](docs/NevroTick-AVIF-kit-v1/README.md) |
| Contributing and review | [CONTRIBUTING.md](CONTRIBUTING.md), [agent guide](AGENTS.md) |
| Community expectations | [Code of Conduct](CODE_OF_CONDUCT.md) |
| Accessibility and reporting barriers | [ACCESSIBILITY.md](ACCESSIBILITY.md) |
| Private vulnerability reporting | [SECURITY.md](SECURITY.md) |
| Release support and recovery | [Release policy](docs/release-policy.md): proposed support baseline, approval, withdrawal and incident procedures |

Start with [CONTRIBUTING.md](CONTRIBUTING.md) before proposing changes. It routes
contributors to the existing plans, repository skills and verification contracts.
Use [GitHub Issues](https://github.com/DavidOsipov/NevroTick/issues) for ordinary
bugs, accessibility barriers and suggestions. **Do not disclose vulnerabilities
in public issues:** use
[GitHub private vulnerability reporting](https://github.com/DavidOsipov/NevroTick/security/advisories/new)
first, or the encrypted-email fallback described in [SECURITY.md](SECURITY.md).

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

**Core and inherited workspace license:** [GNU GPL v2.0 only](crates/ntp-wire/COPYING), SPDX identifier
[`GPL-2.0-only`](https://spdx.org/licenses/GPL-2.0-only.html), as declared in
[Cargo.toml](Cargo.toml).

Original GUI source in `nevrotick-ui` uses [GNU GPL v3.0 only](crates/nevrotick-ui/LICENSE),
SPDX `GPL-3.0-only`. Its [licensing and provenance record](crates/nevrotick-ui/LICENSING.md)
keeps that grant separate from Chrony-derived Core code and third-party material.

Chrony-derived code retains its upstream notices and
[provenance requirements](docs/nevrotick-owner-decisions.md#3-od-01--source-derived-port-with-independent-verification).
Dependencies and imported material retain their own notices; artwork has
[separate usage terms](docs/NevroTick-AVIF-kit-v1/BRAND-LICENSE.txt).
