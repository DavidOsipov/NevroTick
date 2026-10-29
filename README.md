# NevroTick
System essentials, under control.

The current implementation is **UI-M0**: one Rust crate with a portable preview
model and an unelevated, fixture-only native Windows UI. It has no live network,
production IPC, persistence, service or clock effects. D1 display/theme, D2
production state and D3 full accessibility work remain deferred under
[architecture §16.5](docs/windows-time-architecture.md#165-restricted-first-ui-milestone-ui-m0).

With the [development inputs configured](TOOLING.md#starting-implementation),
run the shared portable check from the repository root:

```sh
python3 -B scripts/verify.py
```

The same entry point has explicit `dependencies` and `windows` lanes. The
[UI-M0 task record](tasks/ui-m0.md) records actual native observations and remaining
qualification gaps; a successful cross-build alone does not complete UI-M0.

## Start here

| Need | Read |
| --- | --- |
| AI agent instructions and authority | [AGENTS.md](AGENTS.md) |
| Documentation map, architecture and feature plan | [Documentation guide](docs/README.md) |
| Accepted owner choices and decisions still pending | [Owner decisions](docs/nevrotick-owner-decisions.md) |
| Research questions, evidence gates and NTS server candidates | [Research handoff](docs/nevrotick-research-handoff.md) |
| Development setup and adopted tooling | [TOOLING.md](TOOLING.md) |
| UI prototype, specification and screen references | [Design v0.2](docs/nevrotick-design-v0.2/README.md) |
| Brand artwork and asset variants | [AVIF brand kit v1](docs/NevroTick-AVIF-kit-v1/README.md) |
| Current implementation scope, acceptance and evidence | [UI-M0 task record](tasks/ui-m0.md) |

Owner decisions, proposed behavior and executed evidence are recorded separately.
The documents describe the intended product; they do not establish a qualified
Windows application or release.
