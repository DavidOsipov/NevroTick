# Bounded arithmetic proofs

[OD-10](../../docs/nevrotick-owner-decisions.md#14-od-10--mandatory-mathematical-proofs)
and [architecture §20.1.1](../../docs/windows-time-architecture.md#2011-mandatory-mathematical-proofs--5-november-2026)
require source-bound mathematical proofs. This driver covers only its named
suite obligations; no suite covers every product algorithm.
The [bounded Verus addition pilot](../../tasks/s1.3-source-selection-readiness.md#verus-addition-pilot--5-november-2026)
is a separate queued evaluation of the same open addition contract. No Verus
payload, guarded execution entrypoint or product proof is admitted yet. Follow
that card and [TOOLING.md](../../TOOLING.md#additional-rust-analysis-tools) before
acquisition/execution; the Kani commands below select no Verus candidate.
Track actual completion and remaining constructor/caller and other mathematical
obligations in the [existing S1.1c task](../../tasks/s1.1-time-domain.md).
The [Core task](../../tasks/s1.3-source-selection-readiness.md#independent-endpoint-finish-proof-checkpoint--5-november-2026)
owns the separate endpoint-finish proof and its omitted producer/composition obligations.
Its [Fraction constructor checkpoint](../../tasks/s1.3-source-selection-readiness.md#fraction-constructor-proof-checkpoint--5-november-2026)
owns full raw admission, exact retention and zero canonicalization separately.
The same task's [comparison contract](../../tasks/s1.3-source-selection-readiness.md#fraction-comparison-proof-contract--5-november-2026)
owns checked-pair rational ordering and lossless machine alignment separately.
Use the [proof workflow](../../.agents/skills/testing/references/formal-verification.md)
for acceptance; preparing this runner does not establish a completed campaign.

The actual public `EraWindow::new`/Garde constructor has a separate closed Cargo
code-generation preflight:

```sh
python3 -B tools/formal/verify_era_codegen.py --output-dir /absolute/private/parent/new-era-codegen-evidence
```

This freezes and dispatches the consumer/helpers, authenticates and copies only
the fourteen selected vendor packages, and runs one offline `--only-codegen`
command with real build scripts and proc macros. It uses the complete existing
Kani/private-compiler inventory through its distinct codegen API, four narrow
readonly roots and a sealed generic host-linker configuration. The immutable
project retains its own package editions and lockfile; no standalone edition
flag, inherited Cargo/Rust override, solver or proof unwind profile is applied.
Limits are 180 seconds, 8 GiB address space, 128 tasks and two build jobs.

A passing preflight requires actual four-harness metadata/models, all fourteen
package feature fingerprints and complete source/vendor/tool/receipt bookends.
It retains the emitted binary models and maps for inspection; it does not promise
readable loop IR. Generated-code inspection, a justified unwind profile and the
healthy/defect/restored constructor proofs remain separate exits under the
[existing task](../../tasks/s1.1-time-domain.md#remote-s21-handoff-and-public-constructor-preflight--5-november-2026).
Kani's allocation-success assumption establishes no host OOM recovery. The
preflight installs nothing and changes no global Cargo/tool configuration.

The separate five-job constructor proof controller has a reviewed provisional
unwind profile and full validation/safety/sanity checks. Its first actual healthy
attempt stopped in Kani's GOTO model validation, before solver execution, on
inconsistent padded-union member types. The constructor theorem remains open.
Do not repeat the unchanged campaign or disable checks to bypass this failure.
Follow the [constructor proof checkpoint](../../tasks/s1.1-time-domain.md#public-constructor-proof-checkpoint--5-november-2026)
for the retained failure, bounded tool diagnostics and verifier-repair gates.
The isolated compiler repair and SDK loader fix now let the private candidate
reach CBMC, but its fixed 120-second attempt returned no completed checks.
The [compiler repair source kit](kani-padded-union/README.md) retains the reviewed
patch, regression tests, exact source/tool prerequisites and reproduction steps
in this repository. Tool binaries, SDKs, vendors and campaign outputs remain
external. The kit does not select a new default verifier or qualify the
constructor; the same task owns those remaining gates.
The follow-on target-refinement and raw-property-ID driver patches are held at
source preparation. They have not been built, executed or adopted. The current
checkpoint authorizes no additional Kani engineering or constructor solver campaign;
independent product proofs may continue through the existing admitted lanes.

The separately admitted model inspector is bound to the qualified local
third-run receipt and its healthy linked model; it accepts no alternate source
or native flags:

```sh
python3 -B tools/formal/inspect_era_model.py --output-dir /absolute/private/parent/new-era-model-inspection
```

It verifies the receipt/source/model identities, copies only the required model
and small provenance, then uses bundled `goto-instrument --dump-c` in an isolated
namespace with file output. A possible public GCC version query remains within
the existing host runtime trust boundary. The result is bounded UTF-8 inspection
text: allocator/drop-loop analysis, a justified unwind profile and constructor
proofs are separate gates. A changed source checkpoint requires a new reviewed
binding. Raw qualified source receipts remain immutable.

Run from the checkout with the already installed, admitted Kani 0.68.0 payload:

```sh
python3 -B tools/formal/verify.py --tool-admission-profile kani-proof --output-dir /absolute/private/parent/new-proof-evidence
```

The output leaf must not exist. Its parent must exist at a canonical absolute
path, outside the checkout. The driver creates mode-0700 evidence with a private
umask. `--subject rounding` selects seven jobs; `--subject era` selects five;
the default `--subject all` selects twelve. Each job runs once, sequentially.
The driver installs nothing and changes no global tool configuration.

Additional explicit suites use the same admitted execution profile:

| `--subject` | Fixed commands | Mathematical scope |
| --- | --- | --- |
| `coordinate-exchange` | 10 | Raw-i128 coordinate admission; exact four-coordinate arithmetic, producer bounds and representable nanosecond scaling |
| `clock` | 10 | Both raw-u64 clock domains: construction, exact elapsed time and checked addition |
| `leap-calendar` | 7 | Full-u64 month-start helper equivalence and independent Gregorian oracle laws |
| `relative-range` | 5 | Full-i128 admission/rejection of the actual private transformed-output envelope predicate |
| `exchange-composition` | 10 | Full-domain phase and RTT producer theorems plus the actual scaling-expression theorem; all three conjuncts required |
| `calendar-composition` | 11 | Full-u64 prefix normalization, reviewed exact-source composition, complete cycle-day representation, actual suffix and Gregorian laws; all four conjuncts required |
| `leap-ticks` | 5 | Full-u64 whole-second/Q32 conversion and exact success/retention/error through the actual coordinate constructor |
| `calendar-finite-laws` | 5 | Seven actual-Rust Gregorian year/month/epoch laws and four original covers; the separate arbitrary-cycle period leaf is still required |
| `leap-coverage` | 5 | Coverage prefix over all admitted Q32 windows and optional metadata; point-membership and gap-witness theorems, separate from constructors and ledger traversal |
| `endpoint-finish` | 5 | Actual Core integer endpoint floor/ceiling over every raw i128 component, residual sign and direction; exact output/rounding and complete overflow/range errors |
| `fraction-constructor` | 5 | Actual Core private Fraction constructor over every raw u64/u32 pair; complete admission/rejection, exact retention, canonical zero and output invariants |
| `fraction-comparison` | 5 | Actual Core Fraction comparison over every checked pair; exact rational order, required success, factory completeness and machine-alignment bounds |
| `fraction-arithmetic` | 10 | Actual bounded complement followed by addition over every checked fraction/pair; exact admission/error, complement value, carry/residual and canonical-field oracle bounds |

Each includes compiled defects and restored healthy proofs. Review its actual
source correspondence, assumptions and omissions before declaring a finite
checkpoint budget.

The current `fraction-arithmetic` checkpoint qualified complement only. Addition's
first healthy command timed out with zero checks; the cohort is incomplete and
its later four jobs did not run. Follow the [recorded checkpoint](../../tasks/s1.3-source-selection-readiness.md#arithmetic-terminal-checkpoint--complement-qualified-addition-open)
before further work; do not repeat its unchanged monolithic attempt or substitute
larger limits/stubs for the required full-domain result.

`exchange-composition` closes the original exchange contract only when its three
universal theorems, four defects and three restored proofs all qualify under
matching inputs. It imports no theorem as an assumption. The scaling expression
projection proves its selected operation, not the whole rounding function.

`calendar-composition` requires all four universal nodes, three distinct defects
and all four restored healthy proofs. The generated-year theorem covers every
cycle day only through the separate proved total representation; the source
binding uses a closed, reviewed structural rule for the exact pure prefix and
suffix, with pinned enclosing context and matching fragments for every compiled
fault. That source-correspondence rule is an explicit trusted step; no separate
machine-checked `H == factor` theorem is claimed. All prefix arithmetic and
Gregorian obligations still require their machine proofs. The suite imports no
proved node as a Kani assumption and changes no product implementation. Earlier
monolithic `leap-calendar` timeouts remain incomplete records.

The auxiliary period has a separate narrow consumer:

```sh
python3 -B tools/formal/verify_calendar_period.py --output-dir /absolute/private/parent/new-period-evidence
```

It uses the pinned Z3 sidecar directly, with two endpoint SAT controls, a healthy
UNSAT query, three semantic-mutant SAT models independently checked, and restored
UNSAT. The exact fixed Rust F/G expression trees are mapped to integers through
reviewed trusted correspondence; all typed machine bounds and exact widenings
are conclusions. These mutations qualify the auxiliary SMT interface, not
compiled product Rust. The single solver root, frozen helper policy, read-only
job tree, pre/post/final hashes and cumulative artifact audits are enforced.
Neither this period leaf nor `calendar-finite-laws` alone establishes original
D or full calendar acceptance. Joint qualification must also preserve the
actual Rust A/B/C proofs, actual-helper defects and restored evidence.

Preparation tests run without a compiler or solver:

```sh
python3 -B -m unittest discover -s tools/formal -p 'test_*.py' -q
```

`--start-at N` explicitly selects a nonempty suffix of the chosen fixed schedule.
It preserves original job indices, sets `complete_schedule` false when N exceeds
one, and qualifies only commands actually selected. It imports no old receipts.
The coordinator must preserve interrupted runs and independently reconcile any
earlier evidence and relevant source/tool differences. A passed suffix is no
claim that the omitted jobs ran or the whole schedule passed in one invocation.
`--only-jobs 1 3 4 5 9` similarly selects the coordinate healthy/defect/restored
sequence from `coordinate-exchange`, retaining the original indices. Duplicates,
invalid indices and combining explicit indices with a suffix are rejected. Its
partial summary does not qualify omitted exchange jobs.

The fixed schedule proves both rounding scales over raw `i128`, executes three
phase defect controls, then restores both full-domain healthy proofs. The era
schedule proves the selected resolver over valid immutable windows, executes
three defect controls, then restores its full-domain healthy proof. A failed
qualification stops further jobs. The session attempt ceiling remains an owner
obligation: this invocation adds at most twelve proof commands and never retries.

Every proof command first calls `scripts/check-analysis.py` admission. Full
admission remains the default. Explicit `--tool-admission-profile kani-proof`
uses the dedicated complete Kani/compiler inventory plus the existing shipping
executable/bootstrap checks and pointer bookends. Native execution mounts only
the two returned Kani scopes; the whole base, stable store and unchecked private
rustup settings remain unavailable. Do not use this role for generic Cargo or
other analysis consumers. Final closeout repeats the selected admission profile.
The existing namespace runner
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
native exit/output/failure, raw Kani JSON and qualification. Later commands seal
all earlier proof input islands, the frozen source and required JSON receipts;
closeout audits those retained bytes again. Compiler targets are disposable and
are excluded from the retained custody roots. Artifact writes refuse redirected
parents, symlink leaves and hardlink aliases before truncation. No existing receipt
is accepted as a current proof. The installed pointer, tool/compiler hashes,
inventory closures and profile identities are recorded for each admission.

`summary.json` is compact. Individual `jobs/*/execution.json` and `kani.json`
retain failed checks, covers, unreachable/sanity checks, tool metadata and
property accounting. Admission, preparation, execution and reported solver
times are separate. `execute_other_seconds` includes compilation, namespace
setup and Kani overhead; it is not an isolated compiler measurement.

A healthy result requires a real zero exit, completed JSON, exactly the selected
nonempty harness, successful named mathematical assertions and every fixed named
cover satisfied. The closed registry declares the additional suites' obligations;
unknown scopes and empty obligations fail closed. Defects require a real
non-signal nonzero exit and
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
