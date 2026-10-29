# Account for repeated verification work

Use for a campaign with multiple profiles, candidates, fault controls, supervisor
runs or generated replays. A small single-run test can retain a concise command
and bound instead. Keep the worksheet in the existing task, not a second report
tree. This supports bounded execution and CHECK-19 evidence; it proves no product
resource budget, native lifecycle or CHECK-21 soak result.

## Distinguish the bound from the consumed work

Choose units that describe the claim: candidate calls, events, bytes, elapsed
time, retained output or another relevant resource. Keep different units separate.
State which limit is **per invocation** and which is **for the complete campaign**.
Profiles, repaired candidates, fault runs, runner controls, shrink/replay attempts
and retries can each execute the product again; enumerate the selected dimensions
instead of assuming the ordinary healthy run covers their cost.

For planned groups, `aggregate ceiling = Σ(invocation limit × per-invocation
ceiling)` in each unit. Account for conditional retries by a finite ceiling. This
is an execution envelope, not permission to consume unused allowance on unrelated
work. Time/memory/retention bounds may need separate treatment: concurrent peak
memory is not the sum of sequential process peaks.

## Worked worksheet

Toy counts below demonstrate accounting; they are not repository defaults. One
campaign measures candidate events and allows at most two repaired-profile runs.

| Invocation group | Maximum invocations | Events per invocation ceiling | Aggregate event ceiling | Observed invocation counts |
| --- | ---: | ---: | ---: | --- |
| Healthy ordinary and optimized profiles | 2 | 100 | 200 | 92, 94 |
| Targeted fault controls | 3 | 8 | 24 | 6, 6, 6 |
| Supervisor propagation control | 1 | 100 | 100 | 43 before failure |
| Repaired-profile reruns | 2 | 100 | 200 | 92; second run not used |
| Total | 8 | — | 524 | 339 events in 7 invocations |

The healthy per-profile ceiling is 100; the complete ceiling is 524, and actual
consumption is 339. The supervisor's intended failure consumes 43 events; it is
neither zero work nor a passing candidate. A later retry does not erase that
consumption. Unused capacity is 185 events only while every individual invocation
and remaining invocation-count limit also holds.

Validation-only controls can contribute zero candidate events when they actually
do not call it; they still consume their own time/storage. A format/compiler
failure before candidate execution similarly contributes zero candidate calls,
not a zero-cost invocation. Unknown partial execution remains unknown: preserve
the native exit and strongest observed lower/upper bound instead of recording
zero. When the runner launches a full suite, charge its reached work, not just
the case whose fault motivated the run.

Before starting another invocation, reconcile its identity, required bound and
remaining allowance. Preserve consumed totals through candidate repairs, worker
handoffs and interruption. If a planned bound proves incorrect, retain the
original wording and observed variance, then revise the remaining plan within
the user's scope before dependent execution; changing a label cannot make the
earlier campaign fit. Do not repeat unaffected expensive checks merely to fill a
worksheet.

Method extracted from the corrected
[S1.1e workload account](../../../../tasks/s1.1-deterministic-harness.md#e-account-01--workload-envelope-correction-and-observed-variance).
Its numerical limits remain local to that historical task.
