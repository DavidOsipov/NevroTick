# Bounded arithmetic proofs

[OD-10](../../docs/nevrotick-owner-decisions.md#14-od-10--mandatory-mathematical-proofs)
and [architecture §20.1.1](../../docs/windows-time-architecture.md#2011-mandatory-mathematical-proofs--5-november-2026)
require source-bound mathematical proofs. This driver covers only its named
suite obligations; no suite covers every product algorithm.
The [bounded Verus addition pilot](../../tasks/s1.3-source-selection-readiness.md#verus-addition-pilot--5-november-2026)
exhausted its three readiness commands: the pinned private candidate rejects
the required `u64::checked_shl` call before SMT. Its result is **DEFER**, with no
product proof or adoption. Source guards/tests and private raw receipts preserve
the actual failure and the earlier repaired probe type error. Do not restart the
exhausted candidate or assume a missing standard-library contract. The card and
[TOOLING.md](../../TOOLING.md#additional-rust-analysis-tools) own its future return
trigger; the Kani commands below select no Verus candidate.
Track actual completion and remaining constructor/caller and other mathematical
obligations in the [existing S1.1c task](../../tasks/s1.1-time-domain.md).
The [Core task](../../tasks/s1.3-source-selection-readiness.md#independent-endpoint-finish-proof-checkpoint--5-november-2026)
owns the separate endpoint-finish proof and its omitted producer/composition obligations.
Its [Fraction constructor checkpoint](../../tasks/s1.3-source-selection-readiness.md#fraction-constructor-proof-checkpoint--5-november-2026)
owns full raw admission, exact retention and zero canonicalization separately.
The same task's [comparison contract](../../tasks/s1.3-source-selection-readiness.md#fraction-comparison-proof-contract--5-november-2026)
owns checked-pair rational ordering and lossless machine alignment separately.
Its [split checkpoint](../../tasks/s1.3-source-selection-readiness.md#split-terminal-qualification--5-november-2026)
qualifies full raw `ScaledValue::split` admission, exact decomposition and errors
with healthy/three-defect/restored evidence. The separate
[6 November representation checkpoint](../../tasks/s1.3-source-selection-readiness.md#representation-terminal-qualification--6-november-2026)
qualified full raw-bit `from_bits` admission/exact Q33 decomposition and raw-payload
`signed_whole` magnitude/sign/fraction independence, with one complete
healthy/three-defect/restored group per leaf through `scaled-value-representation`.
Addition, `EndpointParts::new`, full enclosure, joint Garde admission/callers and
Core/P1 remain open. Reuse unchanged proofs; follow the Core task's remaining queue.

The [6 November addition contract-mode assessment](../../tasks/s1.3-source-selection-readiness.md#addition-contract-mode-terminal-assessment--6-november-2026)
is terminal **DEFER**: the first primitive mechanism command reached CBMC, which
aborted with status 134 on an internal closure type inconsistency and exported
empty checks. Zero product-proof commands or callee certificates followed; the
checkpoint stopped without retries, tool patches or adoption. Existing ordinary
Kani proofs retain their scopes and full mathematical obligations remain unchanged.

The [ordinary partition assessment](../../tasks/s1.3-source-selection-readiness.md#ordinary-addition-partition-terminal-assessment--6-november-2026)
is terminal **DEFER**. The union and cases 0–2 completed 631 checks: 575 `Success`,
22 satisfied covers and 34 accounted unreachable checks. Case 3 timed out at
60,018 ms with empty checks; later leaves, fault controls and restorations did not
run. These four completed commands retain their individual scopes.

The [full-addition compute-budget assessment](../../tasks/s1.3-source-selection-readiness.md#full-addition-compute-budget-terminal-assessment--6-november-2026)
is terminal **DEFER**: its first healthy command timed out at 300,011 ms with zero
checks; negative controls and restoration did not run. All 37 source inputs, the
60-entry snapshot and tool/solver identities remained intact. The larger budget
leaves addition unproved; valid ordinary proofs retain their scopes and all holds
remain. This card is closed, with no automatic new profile, retry or SDK patch.

The [manual composition assessment](../../tasks/s1.3-source-selection-readiness.md#manual-composition-terminal-assessment--6-november-2026)
is terminal **DEFER / incomplete** after 14 of at most 18 commands. The first
13 qualified 1,415 checks: 1,315 `Success`, 69 satisfied covers, nine intended
failures and 22 accounted unreachable checks. This includes fresh unstubbed
actual-constructor/comparison/complement Post and actual-return seed proofs over
their full admitted domains. The pure-composition rule and compiler name
resolution are explicit trusted boundaries. Addition command 14 timed out at
60,016 ms with zero checks; commands 15–18 did not run.

During command 14, an audit imported frozen Python modules without suppressing
bytecode writes and added four `.pyc` files to the source island. All 45 bound
source files, command-14 inputs and three earlier certificate receipts remained
unchanged, preserving only the earlier scoped lemma evidence. The custody
failure and timeout leave addition unproved and the whole card closed: no
adoption, certificate import or retry. Retain the 23 MiB raw bundle. The separate
query-split checkpoint below owns the subsequent hypothesis; the original card
and its incident remain preserved.

The [query-split terminal assessment](../../tasks/s1.3-source-selection-readiness.md#fraction-query-split-terminal-assessment--6-november-2026)
is **DEFER / incomplete** after 14 of 20 commands. Its fresh prefix qualified
1,415 checks: 1,315 `Success`, 69 satisfied covers, nine intended failures and
22 accounted unreachable checks, establishing only the scoped actual-callee
Post/actual-return seed premises and controls. Universal command 14 timed out at
60,018 ms with zero checks; commands 15–20, including every direct-witness and
restored lane, did not run. Addition remains unproved and Core/P1 incomplete.

All 49 source files, 14 input/permission bookends, three certificate seals and
tool identity passed final custody checks; the observer bytecode failure was
prevented. The 67 Python checks qualify preparation/gating only, and the old
eighteen-job kit and ordinary defaults remain unchanged. Retain the
26,947,584-byte raw bundle. Query splitting reduced the reported VCC count from
2,714 to 664 and completed SAT calls from 33 to nine (44.63 s to 2.81486242 s),
but supplies no property attribution or UNSAT result; the terminal tail reports
only `Running propositional reduction`. Next work is source-only diagnosis and
cross-verifier correspondence assessment. The existing Verus no-cheating gate
still rejects `external_body`; no profile weakening, new experiment, SDK patch
or adoption is authorized. Both cards are closed and all holds remain.

The prepared [source-only bridge candidate](../../tasks/s1.3-source-selection-readiness.md#fraction-verus-bridge-source-preparation--6-november-2026)
uses `fraction_verus_bridge_projection.py` and `fraction_verus_bridge_spec.rs`.
It retains the actual six executable predicates, inserts only reversible
declaration annotations, and records the complete nine-span source closure.
Its focused Python checks establish correspondence preparation, not verified
Verus compatibility or mathematical equivalence. The candidate starts no tools
and changes no admission profile; six body-to-spec proofs and semantic controls
must precede any separately reviewed caller-import execution.

Focused source-preparation checks (23 distinct checks pass, retaining all14 old
checks and nine dependency-insertion controls):

```sh
python3 -X dev -Werror -B -m unittest discover -s tools/formal -p 'test_fraction_verus_bridge_projection.py' -v
```

The [first pure-bridge card](../../tasks/s1.3-source-selection-readiness.md#fraction-verus-bridge-terminal-assessment--6-november-2026)
is closed DEFER after seven private-`open` errors. The exact seven-modifier
correction changes no mathematical/executable body. The subsequent
[private-spec v2 card](../../tasks/s1.3-source-selection-readiness.md#private-spec-verus-bridge-v2-terminal-assessment--6-november-2026)
reached Z3 without VIR errors but closed DEFER after five failed equalities.
Constructor Post's success is conditional on failed helpers; only three Clone
bodies otherwise succeeded. Zero jobs qualified; faults/restoration did not run.
Do not invoke either fixed attempt again or remove its work root.

The source-only diagnostic-consumer repair passes eighteen synthetic tests:

```sh
python3 -X dev -Werror -B -m unittest discover -s tools/formal -p 'test_verify_fraction_verus_bridge.py' -v
```

`fraction_verus_bridge_lemmas.rs` contains the full-raw bit-length/power lemma. The additive
[dependency insertion path](../../tasks/s1.3-source-selection-readiness.md#bit-length-dependency-insertion-preparation--6-november-2026)
uses `dependency_input_names()` and `prepare_bit_length()` to bind eight sources,
insert two fixed pre-subtraction ghost blocks, and erase to the exact original
661-byte predicate slice. It retains the complete prefix and product closure;
the old seven-input full-bridge path produces identical Rust bytes.
The fixed consumer `verify_fraction_bit_length.py` reuses the existing guarded
controller through an immutable closed recipe. Its
[five-job terminal campaign](../../tasks/s1.3-source-selection-readiness.md#bit-length-dependency-terminal-assessment--6-november-2026)
passed: full-raw lemma/two predicate equalities, three distinct contract defects
and byte-identical restored health, with exactly one proof/five exec results per
job. All62 focused source/consumer checks pass, including21 new mixed-mode
controls. The22-source closure and final source/input/receipt/tool bookends are
bound. This fixed attempt is closed; do not invoke it again or delete its root.
The subsequent closed `prepare_constructor_post()` path retains the first1,202
original bytes, all three annotations and the same two proof hooks, with exact
inverse erasure. Its fixed consumer `verify_fraction_constructor_post.py`
completed the [constructor-Post campaign](../../tasks/s1.3-source-selection-readiness.md#constructor-post-terminal-assessment--6-november-2026):
five jobs, seven exact modes/results each, full-raw Post equality/lemma/helpers,
three isolated Post defects and restored health. All93 current focused Python
checks pass (33 projector;18 legacy,21 bit-length and21 constructor consumers).
The24-source candidate binds all raw Result/Ok/error values without a checked-
value or constructor-produced-result premise. Both older preparation outputs
remain exact. This attempt is closed; do not invoke its fixed consumer again.
The subsequent `prepare_complement_post()` path retains original spans
`[0,1202)`/`[2684,3731)`, four annotations and five exact proof hooks. Two new
total guarded lemmas bridge shift/power and checked rejection outside1..53;
neither restricts the raw Post domain. The captured-source
`verify_fraction_complement_post.py` v2 [campaign](../../tasks/s1.3-source-selection-readiness.md#complement-v2-terminal-qualification--6-november-2026)
passed all seven jobs, including five isolated defects and restored health.
Ten exact function identities report on every job; all143 selected Python
checks pass. Thirty source files and eleven executed-Python records are bound.
V1 stopped before SMT because opaque pow did not reduce under compute_only;
the correction proves its literal through the existing shift lemma and a
primitive bit-vector fact. Both roots are closed; preserve them without reruns.
Older output Rust pins remain exact. Next prepare rational-order alignment via
pinned primitive cmp models. Complete bridge, Ordering PartialEq/comparison Post,
caller import and actual addition remain open.

The new consumer's focused synthetic checks execute no compiler or solver:

```sh
python3 -X dev -Werror -B -m unittest discover -s tools/formal -p 'test_verify_fraction_bit_length.py' -v
```

Constructor-Post consumer checks likewise execute no compiler or solver:

```sh
python3 -X dev -Werror -B -m unittest discover -s tools/formal -p 'test_verify_fraction_constructor_post.py' -v
```

Complement/source-binding consumer checks execute no verifier:

```sh
python3 -X dev -Werror -B -m unittest discover -s tools/formal -p 'test_verify_fraction_complement_post.py' -v
python3 -X dev -Werror -B -m unittest discover -s tools/formal -p 'test_verify_fraction_source_binding.py' -v
```

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
