# Tracing, reducing and locating a regression

Use causal tracing for an unexplained bad effect; use reduction or historical
search only when that narrower question will help. Keep original input/artifacts immutable, the
same independent semantic predicate, a healthy control and finite candidate,
time, preparation and retry allowances. These procedures add no installation,
repository mutation or live clock/service permission.

## Find the first broken boundary

Start at the observed incorrect output or effect and trace its actual predecessor
calls and state transitions. For each suspect boundary record its owner, incoming
value and units, expected invariant, outgoing value/result, current authority and
candidate identity. Across processes or async jobs retain request/generation and
correlation identity; a caller stack alone cannot establish the remote producer.

If existing observations cannot distinguish competing causes, add the smallest
bounded pre/post observation at the suspect operation in the admitted fixture.
Use redacted structured values, event order and a stack or correlation identifier
where useful. Avoid raw packets, cookies, keys or environment dumps; retain the
diagnostic limits and any timing disturbance. Missing output from a suppressed
logger is an observation gap, not proof the operation never ran.

Replay the original failure and healthy control with the same expectations.
Find where a valid input/state first becomes invalid, including an earlier missed
admission or lost acknowledgement; preserve downstream protection. Distinguish
that evidence from a hypothesis about the eventual repair. Keep diagnostic code
only when it has a continuing, reviewed purpose.

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

Qualify the predicate on the original failure and healthy control before search.
Remove coarse units, then smaller or structure-aware units, preserving required
identities, relationships, raw encoding and event order. Accept a candidate only
when the same TARGET_FAIL remains and controls remain healthy; do not change the
predicate or replace the actual entrypoint to make progress.

Each candidate starts from fresh owned state, while preserving state between
events within its own sequence. For order-dependent failures, keep the victim
fixed and reduce its ordered predecessors. Multiple predecessors can jointly
pollute state even when every predecessor/victim pair passes; test groups and
complements before blaming one predecessor. A failed reset invalidates later
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

Freshly replay the reported bad revision and relevant passing parent/boundary,
then inspect the actual diff and merge context. The first observed bad commit
does not establish the causal mechanism. Preserve logs before ending the owned
search and restoring its starting state; reconcile only owned temporary changes.

Donor workflows and adaptation scope are recorded in
[testing sources](../SOURCES.md) and [provenance](../../PROVENANCE.md).
The boundary-tracing method also adapts Superpowers'
[systematic debugging](https://github.com/obra/superpowers/blob/2ce8aa15d05bd57213e6f746f953aafb7ded7a8b/skills/systematic-debugging/SKILL.md)
and [root-cause tracing](https://github.com/obra/superpowers/blob/2ce8aa15d05bd57213e6f746f953aafb7ded7a8b/skills/systematic-debugging/root-cause-tracing.md), with its
[MIT notice](../../provenance/licenses/superpowers/LICENSE).
