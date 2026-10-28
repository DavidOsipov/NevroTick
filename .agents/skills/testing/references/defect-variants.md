# Search for variants of a confirmed defect

Use after a root cause is established and related occurrences could affect the
current regression scope. This is a bounded search method, not an initial audit
or a requirement to repair every similar expression. Before Rust source exists,
record the future search obligation rather than inventing a scan result.

## Establish the search contract

Record the known witness, violated invariant, relevant inputs/state, reachable
operation, existing guards and healthy control. Freeze candidate/configuration
identity. Define allowed source areas, semantic axes, query/match/time budgets and
the next decision; related owners may be outside the original module while still
inside the task's scope. Note excluded or unavailable areas explicitly.

Start with a search that actually finds the known faulty location. A zero-hit
baseline means the pattern or search setup is unqualified; do not generalize it
and report no variants. Use existing Rust search/symbol tools; text matching
locates candidates and does not prove Rust types, data flow or reachability.

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

Read its owner, types, callers, feature/target configuration and protection path.
Look for prior validation, private construction, current-state checks, different
units, unreachable values or an operation with different semantics. Similar text
behind an effective guard is not a confirmed variant.

Classify each inspected match as confirmed in the declared configuration, refuted
with its actual protection/reason, or unresolved with the missing observation.
Keep a potentially unsafe but unreachable path separate from a demonstrated
reachable defect; name what would enable it. A search match alone is no behavioral
RED. For confirmed candidates, select independent failure and healthy controls
through the applicable entrypoint, preserving host/native evidence distinctions.

Finish with queries/axes attempted, inspected and pending matches, evidence or
refutation per candidate, selected regression checks, scope exclusions and budget
consumption. Exhausting a budget yields a partial search, not a clean repository.
Return repair decisions to the current owner and authorized change scope.

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
