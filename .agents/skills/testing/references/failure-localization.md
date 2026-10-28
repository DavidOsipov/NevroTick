# Tracing, reducing and locating a regression

Use causal tracing for unexplained bad effect; reduction/historical search only
for useful narrower question. Keep immutable original input/artifacts, same
independent semantic predicate, healthy control, finite candidate/time/preparation/
retry allowances. No added installation/repository mutation/live clock/service permission.

## Find the first broken boundary

From observed wrong output/effect trace actual predecessor calls/state
transitions. Each suspect boundary: owner, incoming value/units, expected invariant,
outgoing value/result, current authority, candidate identity. Across processes/async
jobs retain request/generation/correlation identity; caller stack alone cannot
establish remote producer.

If observations cannot distinguish competing causes, add smallest bounded
pre/post observation at suspect operation in admitted fixture. Use redacted
structured values/event order/useful stack or correlation identifier. No raw
packets/cookies/keys/environment dumps; retain diagnostic limits/timing disturbance.
Suppressed logger output is observation gap; does not prove operation never ran.

Replay original failure/healthy control with same expectations. Find first
valid-input/state → invalid transition, including earlier missed admission/lost
acknowledgement; preserve downstream protection. Separate evidence/repair
hypothesis. Retain diagnostic code only for continuing reviewed purpose.

## Semantic outcomes

| Outcome | Required observation |
| --- | --- |
| PASS | Preparation and the intended action completed, and the selected independent check ran without the mismatch. |
| TARGET_FAIL | The intended action was reached and the same named semantic check failed. |
| UNRESOLVED | The target was not observed, a different failure occurred, a control/precondition broke, or setup, cleanup or runner timeout prevented a decision. |

A nonzero exit, parser error or runner cutoff is not automatically the target.
A reached operation exceeding its specified deadline can be TARGET_FAIL when
that deadline is the actual contract. Retain primary and secondary failures.

## Reduce a witness

Before search qualify predicate on original failure/healthy control. Remove
coarse, then smaller/structure-aware units; preserve required identities,
relationships, raw encoding, event order. Accept candidate only with same
TARGET_FAIL/healthy controls; no predicate change/actual-entrypoint replacement
for progress.

Each candidate starts fresh owned state; preserve within-sequence state. For
order-dependent failures fix victim/reduce ordered predecessors. Predecessors may
jointly pollute state despite passing predecessor/victim pairs; test groups/
complements before single-predecessor blame. Failed reset invalidates later
attempts until resolved.

Independently replay the original, reduced witness and healthy control through
the real entrypoint outside reducer caches. Retain exact bytes/events, schedules,
outcomes and allowance consumption. Claim only a reduced witness unless all
named permitted single-unit removals were checked; unresolved removals preclude
even that limited minimality claim. Do not promise a unique smallest input.

## Locate a historical boundary

First reproduce qualified good and bad endpoints using the same input, action
and predicate. Verify real relevant repository ancestry and objects; shallow or
missing history remains a limit. A synthetic snapshot history cannot identify an
actual upstream introduction. Reverts or repeated introductions can invalidate
a single-transition search assumption.

Use an owned isolated checkout/worktree and isolated writable resources; preserve
the active user checkout. Inspect each revision's toolchain, lockfile, features,
fixtures and generated/native prerequisites before execution. A current test
importing a symbol absent in old source does not prove old behavior was bad.
Keep the narrow driver outside files replaced during checkout, and qualify it
at both endpoints. Any historical adapter must preserve the observed semantics.

Translate semantic outcomes deliberately using the installed
[Git bisect protocol](https://git-scm.com/docs/git-bisect), rather than forwarding
arbitrary shell exits. A known revision-specific inability to test may be skipped
with its evidence; unexplained driver/control/isolation trouble stops the search.
Retain every tested revision and skip reason; unresolved revisions at the boundary
leave an ambiguous candidate set. A build failure is bad only when building is
the selected contract, rather than a prerequisite to another behavior.

Freshly replay reported bad revision/relevant passing parent/boundary; inspect
actual diff/merge context. First observed bad commit proves no causal mechanism.
Preserve logs before ending owned search/restoring starting state; reconcile
only owned temporary changes.

Donor workflows and adaptation scope are recorded in
[testing sources](../SOURCES.md) and [provenance](../../PROVENANCE.md).
The boundary-tracing method also adapts Superpowers'
[systematic debugging](https://github.com/obra/superpowers/blob/2ce8aa15d05bd57213e6f746f953aafb7ded7a8b/skills/systematic-debugging/SKILL.md)
and [root-cause tracing](https://github.com/obra/superpowers/blob/2ce8aa15d05bd57213e6f746f953aafb7ded7a8b/skills/systematic-debugging/root-cause-tracing.md), with its
[MIT notice](../../provenance/licenses/superpowers/LICENSE).
