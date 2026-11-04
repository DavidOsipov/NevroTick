# Bounded arithmetic proofs

[OD-10](../../docs/nevrotick-owner-decisions.md#14-od-10--mandatory-mathematical-proofs)
and [architecture §20.1.1](../../docs/windows-time-architecture.md#2011-mandatory-mathematical-proofs--5-november-2026)
require source-bound mathematical proofs. This driver covers only its named
rounding/era obligations; its twelve jobs do not cover every product algorithm.
Track actual completion and remaining constructor/caller and other mathematical
obligations in the [existing S1.1c task](../../tasks/s1.1-time-domain.md).
Use the [proof workflow](../../.agents/skills/testing/references/formal-verification.md)
for acceptance; preparing this runner does not establish a completed campaign.

Run from the checkout with the already installed, admitted Kani 0.68.0 payload:

```sh
python3 -B tools/formal/verify.py --output-dir /absolute/private/parent/new-proof-evidence
```

The output leaf must not exist. Its parent must exist at a canonical absolute
path, outside the checkout. The driver creates mode-0700 evidence with a private
umask. `--subject rounding` selects seven jobs; `--subject era` selects five;
the default `--subject all` selects twelve. Each job runs once, sequentially.
The driver installs nothing and changes no global tool configuration.

The fixed schedule proves both rounding scales over raw `i128`, executes three
phase defect controls, then restores both full-domain healthy proofs. The era
schedule proves the selected resolver over valid immutable windows, executes
three defect controls, then restores its full-domain healthy proof. A failed
qualification stops further jobs. The session attempt ceiling remains an owner
obligation: this invocation adds at most twelve proof commands and never retries.

Every proof command first calls the existing full `scripts/check-analysis.py`
admission. Final closeout repeats full admission. The existing namespace runner
retains read-only admitted tool stores, sealed inputs, disabled network access,
8 GiB address-space limit per process, 128-task limit, two build jobs and one
solver command at a time. These are rlimits, not aggregate cgroup quotas. The
unchanged Kani profile uses Cadical, unwind 1, sanity checks and a 60-second
harness timeout; each command has a 180-second outer wall/CPU limit. Explicit
Rust flags are `--edition=2024 --check-cfg=cfg(test)`.

The driver freezes complete `time-domain` Rust sources, the actual error enum,
crate/workspace/toolchain/configuration inputs, proof specifications, projection
generator, runner code and tool policies. It refuses drift at preparation,
before/after each proof and final closeout. Every job retains its own complete
frozen runner/source copies and generated proof inputs, before/after input
hashes, full admission, original-source manifest, exact command/environment,
native exit/output/failure, raw Kani JSON and qualification. No existing receipt
is accepted as a current proof. The installed pointer, tool/compiler hashes,
inventory closures and profile identities are recorded for each admission.

`summary.json` is compact. Individual `jobs/*/execution.json` and `kani.json`
retain failed checks, covers, unreachable/sanity checks, tool metadata and
property accounting. Admission, preparation, execution and reported solver
times are separate. `execute_other_seconds` includes compilation, namespace
setup and Kani overhead; it is not an isolated compiler measurement.

A healthy result requires a real zero exit, completed JSON, exactly the selected
nonempty harness, successful checks and all named covers satisfied: six per
rounding proof and three per era proof. Defects require a real nonzero exit and
only their intended proof assertion failures. Timeouts, missing/partial JSON,
empty selections, changed inputs, solver errors, unknown outcomes and unrelated
failures leave qualification incomplete. JSON property summaries must reconcile
with the individual checks. Raw reachability results are retained.

The rounding oracle independently specifies required success, exact scaling
versus destination errors, signed residual reconstruction, nearest rounding and
even ties. The actual complete rounding source is copied byte for byte for
healthy jobs; each defective copy changes one documented expression.

The era projection preserves reviewed exact source spans for constants, the
private window type/getters and `unfold_ntp`, plus the complete actual coordinate
module and actual `DomainError` enum. Its span hashes and omissions are retained
in each job. `era_projection.prepare_era(source_root, inputs, case, harness_path)`
is the narrow preparation hook; it must return `inputs/proof.rs` and explicit
correspondence. It assembles a private proof window only after real coordinate
construction succeeds. The sole full-domain era assumption is ordered window
endpoints; the signed-96-bit coordinate range is encoded by a high-i32/low-u64
representation. This proves no public garde constructor path or anchor trust.

These finite arithmetic proofs complement existing behavioral checks. They do
not establish whole-crate correctness, a verified shipping compiler/binary,
authenticated exchanges, real UTC, clock authority, native Windows behavior or
release qualification. A driver exit alone is insufficient without its retained
nonempty proof results and controls. New source bytes require reviewing the
specification and correspondence before another run; pinned era span hashes
intentionally reject silent source drift.
