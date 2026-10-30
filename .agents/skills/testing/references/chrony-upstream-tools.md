# Chrony simulation and fuzzing references

Use when selecting upstream scenarios, captured trajectories or fuzz seeds.
[Architecture §20.1](../../../../docs/windows-time-architecture.md#201-independent-numerical-truth)
controls the evidence boundary. Reuse the explicit Rust simulation seam and the
existing admitted fuzz targets; these upstream projects are research references,
not installed tools, admitted dependencies or an instruction to run a campaign.

## Source pins and rights

Checked 30 October 2026; never substitute moving branch heads for these inputs.

| Project | Inspected revision and source |
| --- | --- |
| [clknetsim](https://gitlab.com/chrony/clknetsim) | `e9f11e60c72f2c0674a072c4ddcfe343f8634e58`, the existing architecture S53 pin. GitLab and the maintainer mirror advertise the same revision; rate-limited raw retrieval was replaced by the [pinned maintainer README](https://github.com/mlichvar/clknetsim/blob/e9f11e60c72f2c0674a072c4ddcfe343f8634e58/README). |
| [chrony-fuzz](https://gitlab.com/chrony/chrony-fuzz) | `904a8789164ac294ab7e85523792ed906a7b5a57`, inspected through a read-only source checkout after GitLab raw/API rate limiting. Its [README](https://gitlab.com/chrony/chrony-fuzz/-/blob/904a8789164ac294ab7e85523792ed906a7b5a57/README.adoc) and patches, not the project summary, define the harness. |
| Chrony reference | Keep `8df4f1263e206585e6c4e61352d042c2dd12916a` (4.9) and its [simulation tests](https://github.com/mlichvar/chrony/tree/8df4f1263e206585e6c4e61352d042c2dd12916a/test/simulation). Do not advance the numerical reference merely to fit a fuzz patch set. |

clknetsim's inspected `README`, `client.c`, `server.cc`, `generator.cc` and
`clknetsim.bash` notices, and the chrony-fuzz README, state GPL version 2 or later;
selected Chrony numerical source is GPL-2.0-only. Keep file-specific notices,
modification/source obligations and the exact scenario/seed/patch origin. There
is no separately established blanket corpus grant here. Record generated seeds'
producer inputs and rights as well as their output bytes before importing them.
These observations do not clear a complete tool/dependency graph for execution.

## clknetsim: trajectories and scenario design

The Linux client interposition library uses `LD_PRELOAD` and a local Unix socket
to communicate with a C++ simulation server. It models clock offsets/drift/steps
and directed network delay/loss. The [Makefile](https://github.com/mlichvar/clknetsim/blob/e9f11e60c72f2c0674a072c4ddcfe343f8634e58/Makefile)
uses Make, C/C++ compilers, Linux/libc headers, dynamic loading and math libraries;
optional visualization and Chrony's TLS tests have additional dependencies.

Prefer the initial lane: run a qualified pinned Chrony build under the simulator,
capture its per-event observations, and compare them through NevroTick's explicit
pure event interface. Begin with healthy drift/convergence, asymmetric delay,
majority loss/falsetickers and external-step cases. Derive expected facts
independently; freeze sign/units, vote policy and numerical tolerances before
comparison. Compare membership/reasons, uncertainty, polling and the complete
correction trajectory, not only final offset. Explain intentional deviations.

Direct Rust execution under preload is a separate compatibility experiment.
The pinned [client.c](https://github.com/mlichvar/clknetsim/blob/e9f11e60c72f2c0674a072c4ddcfe343f8634e58/client.c)
interposes select/poll and selected timer/socket/clock APIs, but has no epoll or
io_uring wrappers. Its generic `syscall()` handles a small allowlist and asserts
otherwise; socket creation flags also need checking. Static linkage, direct
syscalls or a particular Rust/Tokio path cannot be assumed intercepted. Prove
the actual reached calls and forbidden host effects with healthy/fault controls;
the existence of wrappers or successful compilation is insufficient. Do not
change the selected runtime just to fit this tool.

Freeze explicit nonzero random seeds and their per-process mapping: the server
otherwise falls back to host time, and client zero-seed behavior can use host
randomness. Retain simulator/client/reference binaries, toolchain/libc, date,
configuration, inputs, event ordering and output identities. Verify repeat replay
before calling the run deterministic; a seed alone gives no cross-platform
bit-for-bit guarantee. Preserve upstream skip/failure outcomes and challenge the
actual comparator with a semantic mismatch and a valid neighbor.

The simulator models instantaneous CPU work and simplified rate/kernel behavior;
some identity/capability wrappers are no-op successes. It proves no real latency,
resource budget, Windows token/SCM/clock enforcement, power/resume or MSI behavior.
The [Chrony NTS simulation](https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/test/simulation/139-nts)
has its own provider/certificate setup; it does not qualify NevroTick's TLS graph
or Windows trust policy.

## chrony-fuzz: reachability cases, with explicit semantic limits

This project supplies AFL++/clknetsim scenarios and invasive Chrony patches;
it is not a drop-in Rust libFuzzer target. Its test drivers generate seeds from
literal inputs, selected Chrony examples/tests and simulated traffic. Do not
assume a ready, independently licensed raw-NTP corpus or import an outer harness
record as if it were a packet. Identify each target's framing and extraction.
`scripts/pcap2input` can emit raw payloads or big-endian 16-bit-length-prefixed
records, depending on the target; the latter are not raw datagrams. Its fixed
Ethernet/IPv4/header assumptions do not constitute a general packet decoder.

Two patch groups materially change the tested semantics:

- [0009-simplify-protocol-checks.patch](https://gitlab.com/chrony/chrony-fuzz/-/blob/904a8789164ac294ab7e85523792ed906a7b5a57/patches/0009-simplify-protocol-checks.patch)
  replaces origin/duplicate/authentication/UID checks with simplified controls.
- [0013-bypass-SIV-and-TLS.patch](https://gitlab.com/chrony/chrony-fuzz/-/blob/904a8789164ac294ab7e85523792ed906a7b5a57/patches/0013-bypass-SIV-and-TLS.patch)
  substitutes SIV behavior and TLS handshake/ALPN/transport/exporter behavior.

These changes can improve path reachability. They cannot establish real
authentication, replay protection, NTS key derivation or exchange/KoD acceptance,
and the modified peer is not an unmodified semantic oracle. Inventory every patch
and disabled guard before interpreting coverage or transferring a scenario.
Never copy these bypasses into NevroTick production admission or use their
accepted results as expected security outcomes.

Pinning this repository alone is insufficient: [scripts/prep](https://gitlab.com/chrony/chrony-fuzz/-/blob/904a8789164ac294ab7e85523792ed906a7b5a57/scripts/prep)
defaults Chrony and clknetsim to floating `master` before applying patches.
Freeze all three revisions, resulting patched tree, AFL++, native compiler,
libraries/sanitizers, flags and seeds. Compatibility with our Chrony 4.9 pin is
unproven; do not silently advance it or execute that preparation script unchanged.

For useful reuse, map selected input families to the actual Rust owner: raw
header/extension framing in [ntp-wire's target](../../../../policy/ntp-wire-fuzz-target.rs),
state transitions/correlation in exchange scenarios, and future KE/AEAD/cookie
boundaries only once their production entry points exist. This upstream campaign
provides no dedicated regression/selection target or independent numerical oracle;
domain/core keep their analytic/rational controls and existing fuzz targets.
Chrony server, peer, PTP and chronyc features do not expand NevroTick's scope.
Extend the existing harness where justified.
Retain original and minimized bytes, rights, target representation and independent
valid/reject facts. Qualify exact forwarding and reset of all relevant state
between cases; freeze a bounded event sequence where stateful testing is intended.
The patches defer AFL initialization; source inspection did not qualify the
resulting forkserver lifecycle or a persistent Rust reset protocol. Its client
fuzz adapter keeps state within a process; inspect the actual selected lifecycle
before claiming isolation across inputs.

Use the admitted Rust fuzz engine and sandbox/resource controls. AFL++ itself and
the upstream C/build graph remain separate admission decisions; no switch from
existing libFuzzer is implied. Upstream `-m none`, `nproc` and shell defaults do
not supply our finite memory/artifact/worker/campaign bounds; preserve the existing
12-core tool budget and any stricter admitted lane limits. Include accepted seeds, intended-fault controls and
observable no-state-change assertions for invalid input. No crash proves neither
semantic correctness nor absence of forbidden effects. Cryptographic boundaries
also need real-provider known-answer, negative and interoperability cases.

## Phase routing and evidence

P1/S1.4–S1.5 owns abstract discipline trajectories and target/harness qualification;
P2 owns real NTP/NTS/KoD/cookie integration and independently checked protocol
semantics. Relevant CHECK-6/7/8/10/19/20/25/26/27 retain their original scopes.
Current wire/domain/core fixtures can gain targeted seeds without claiming P1/P2
completion. Execution identity, finite budgets, actual case/skip counts and gaps
belong in the [existing task chain](../../../../tasks/s0.1-protocol-comparison.md#upstream-simulation-and-fuzz-tool-follow-up),
not this reusable reference. Source inspection alone runs neither project.
