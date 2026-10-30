# Security policy

## Reporting a vulnerability

Please report potential security vulnerabilities privately to **David Osipov** at
[personal@david-osipov.vision](mailto:personal@david-osipov.vision), with the subject
**NevroTick security report**.

Avoid posting exploit details in public issues or pull requests before the report
has been assessed and disclosure coordinated. Ordinary bugs and feature requests
can go to [GitHub Issues](https://github.com/DavidOsipov/NevroTick/issues).

Include what you have available:

- The affected commit, version or artifact, including local modifications.
- Your operating system, build target and relevant configuration.
- Reproduction steps or a small proof of concept, with expected and observed
  behavior.
- The possible security impact and any prerequisites an attacker would need.
- Relevant logs or crash information with credentials, keys, cookies and personal
  data removed.

An incomplete report is still welcome. If sensitive material is necessary to
explain the issue, ask about a suitable transfer method before sending it.

The maintainer will assess the report, request further information where needed,
and coordinate remediation and disclosure with the reporter. A fixed response or
remediation deadline is not currently established. If you have not received an
acknowledgement, please follow up by email.

## Development status and supported versions

NevroTick is in early development. This policy does not establish a
production-supported release series or a backport schedule. Please identify the
exact revision you assessed; reproducing an issue on current development code is
helpful, but reports about older snapshots are also welcome.

The current workspace contains a fixture-only native Windows UI and portable
protocol, arithmetic, exchange-fixture and numerical components. It does not
provide live NTP/NTS communication, production IPC, installed services or system
clock adjustment. The initial product target is Windows 11 x64; Linux/WSL checks
and Windows cross-builds provide narrower evidence than native Windows tests.

The [product roadmap](ROADMAP.md) also records future Android timekeeping and
DNS-over-QUIC with leaf/intermediate certificate pinning. Those capabilities are
not implemented or security-qualified. DoH and DoT are outside the planned DNS
feature; pin provisioning, rotation and recovery still require a detailed design.

The [development plan](docs/plans/development-plan.md) and linked task records
describe implemented scope and remaining qualification. Planned controls and
successful individual checks do not establish complete security or release
readiness.

## Security review scope

Reports are welcome for issues in application code, native Windows adapters,
protocol and numerical handling, dependencies, build and CI tooling, and included
previews or development tools when they affect project security. Include the
reachable input, affected component and resulting impact where possible.

Examples include memory-safety defects, validation bypasses, unbounded resource
use, sensitive-data exposure, compromised build or release provenance, misleading
trust status, and numerical errors that could affect timekeeping. Unexpected
network, privilege or clock effects in a fixture-only component are also
reportable.

For the future Android and DoQ designs, concerns about the local SNTP hop,
resolver identity, pinning bypasses, unsafe pin updates or transport downgrades
are welcome as design reports. Their presence in the roadmap does not establish
a currently reachable implementation.

The full product design separates the network-facing engine, privileged clock
broker, normal-user UI and administrative helper. Its security requirements
include bounded input handling, authorization at protected effects, preservation
of source identity and freshness, accurate trust reporting, and no silent
authentication downgrade. These are requirements for the planned integrations,
not a claim that those components or protections are already implemented.

Use the canonical [threat model](docs/windows-time-architecture.md#14-threat-model),
[security invariants](docs/windows-time-architecture.md#142-protected-assets-and-security-invariants)
and [UI-M0 scope](docs/windows-time-architecture.md#165-restricted-first-ui-milestone-ui-m0)
for the detailed boundaries. Unresolved decisions and unfinished qualification
remain visible in those documents; this policy introduces no new reporting
exclusions or accepted-risk exceptions.

Please use systems and fixtures you own or are authorized to assess. Experiments
that alter clocks, Windows services or installation state belong in disposable
environments with a restoration plan.
