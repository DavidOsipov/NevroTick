# Search for variants of a confirmed defect

Use after established root cause when related occurrences could affect current
regression scope. Bounded search; no initial audit or every-similar-expression
repair mandate. Before Rust source exists, record future search obligation;
invent no scan result.

## Establish the search contract

Record known witness, violated invariant, relevant inputs/state, reachable
operation, existing guards, healthy control. Freeze candidate/configuration
identity. Define allowed source areas, semantic axes, query/match/time budgets,
next decision; related owners outside original module may remain in scope.
Explicitly note excluded/unavailable areas.

First search must find known faulty location. Zero-hit baseline leaves
pattern/setup unqualified; do not generalize or report no variants. Use existing
Rust search/symbol tools; text matching locates candidates, proves no Rust types,
data flow or reachability.

## Expand one dimension at a time

Derive axes from existing code and the same root cause: other callers, equivalent
conversions or state transitions, different spellings and boundary inputs.
Hypothetical NevroTick examples include an unchecked unit conversion in another
DTO path, stale authority reused across another await, or a native work permit
released when its awaiting future ends. These examples assert no current defect.

Change one restriction per search, inspect the newly admitted matches, and record
the query and why that expansion is relevant. Broaden from literal spelling to
structure or semantics only as needed. If unrelated matches dominate, narrow the
pattern or stop with the unsearched remainder recorded; no fixed noise percentage
establishes coverage. Do not install a scanner or add a CI rule merely to run this
method.

## Try to refute each candidate

Read owner/types/callers/feature/target configuration/protection path. Check prior
validation, private construction, current-state checks, different units,
unreachable values, different operation semantics. Similar text behind effective
guard is no confirmed variant.

Classify each inspected match as confirmed in the declared configuration, refuted
with its actual protection/reason, or unresolved with the missing observation.
Keep a potentially unsafe but unreachable path separate from a demonstrated
reachable defect; name what would enable it. A search match alone is no behavioral
RED. For confirmed candidates, select independent failure and healthy controls
through the applicable entrypoint, preserving host/native evidence distinctions.

Finish: attempted queries/axes, inspected/pending matches, candidate
evidence/refutation, selected regression checks, scope exclusions, budget
consumption. Exhausted budget means partial search; no clean-repository claim.
Return repair decisions to current owner/authorized scope.

## Attribution and adaptation

Adapted from Axel Mierczuk / Trail of Bits' [variant-analysis skill and linked
references](https://github.com/trailofbits/skills/tree/bf23bfb21f10a2a4b0406086d690dc8cb1b3ccf3/plugins/variant-analysis/skills/variant-analysis),
at commit `bf23bfb21f10a2a4b0406086d690dc8cb1b3ccf3`, inspected 28 October 2026.
This reference is licensed under
[CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/); the
[full retained license](../../provenance/licenses/trailofbits/LICENSE) accompanies
it. Local changes add Rust/Windows ownership examples, explicit budgets and
candidate/evidence limits; omit the parallel workflow, automatic CI/report
generation and scanner resources; and replace automatic treatment of unreachable
code as a vulnerability with explicit reachability classification. This
attribution does not imply endorsement or select the application license.
