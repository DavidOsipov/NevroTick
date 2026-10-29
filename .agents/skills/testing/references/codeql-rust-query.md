# Author and debug a Rust flow query

Use after [database and environment admission](codeql-data-flow.md). This is a
small query-development recipe, not a product-wide vulnerability rule. Choose
one concrete path and independently specify its expected endpoints first.

## Match the installed API

The admitted profile pins CLI **2.27.3** and `codeql/rust-all` **0.2.24**; the
library's build metadata identifies source
`a1575b98a8cc19bcfa7bb6cb41be688cb3dae65c`. Resolve packs and inspect their source
again after an update. Context7 and current web examples locate useful concepts,
but may describe a later library API.

- In this pin, `DataFlow::Node.asExpr()` returns an AST `Expr` directly. The newer
  online guide's `node.asExpr().getExpr()` is incompatible with this pack.
- `Call.getPositionalArgument(0)` selects the first positional argument;
  a method's receiver is separate (`MethodCall.getReceiver()`). Select the
  correct value, not the entire call or a coincidentally adjacent argument.
- Enumerate resolved function identities (`getCanonicalPath()`), locations and
  relevant call targets. Static resolution alone may miss dynamic dispatch;
  inspect `getARuntimeTarget()` and the actual library model where applicable.
- Modern configuration uses `DataFlow::ConfigSig` plus `DataFlow::Global` or
  `TaintTracking::Global`. Do not copy legacy `Configuration` subclasses or a
  JavaScript barrier example into a Rust query.

Primary API owners: [nodes](https://github.com/github/codeql/blob/a1575b98a8cc19bcfa7bb6cb41be688cb3dae65c/rust/ql/lib/codeql/rust/dataflow/internal/Node.qll),
[calls](https://github.com/github/codeql/blob/a1575b98a8cc19bcfa7bb6cb41be688cb3dae65c/rust/ql/lib/codeql/rust/elements/internal/CallImpl.qll),
[flow and barriers](https://github.com/github/codeql/blob/a1575b98a8cc19bcfa7bb6cb41be688cb3dae65c/rust/ql/lib/codeql/rust/dataflow/DataFlow.qll).

## Small executable example

Keep owned query files in a fresh directory under `/work/queries`. Use an
admitted query pack with `codeql/rust-all: 0.2.24` and its reviewed lockfile;
resolve the installed dependency closure rather than fetching packs implicitly.
Host paths come from the installation record; `/work` paths are for MCP calls.

This illustrative fixture has two callers of a harmless stand-in sink:

```rust
fn relay(value: i32) -> i32 { value }
fn effect_sink(value: i32) -> i32 { std::hint::black_box(value) }
fn positive(untrusted: i32) -> i32 { effect_sink(relay(untrusted)) }
fn negative(_untrusted: i32) -> i32 { effect_sink(relay(7)) }
fn main() { assert_eq!(positive(11), 11); assert_eq!(negative(99), 7); }
```

Expected extraction: all five functions in `src/main.rs`. Expected selection:
the first parameter of each caller and the argument of each `effect_sink` call.
Expected flow: the `positive` parameter reaches its sink through `relay`;
the `negative` parameter does not reach the constant-valued sink. The fixture's
names describe query controls, not a real security vulnerability.

Put the shared selectors in `FixtureModel.qll` (a different name from its module
avoids an import ambiguity warning, which this profile treats as an error):

```ql
import rust
import codeql.rust.dataflow.DataFlow

module FixtureConfig implements DataFlow::ConfigSig {
  predicate isSource(DataFlow::Node node) {
    exists(Function f |
      f.getLocation().getFile().getRelativePath() = "src/main.rs" and
      f.getName().getText() = ["positive", "negative"] and
      node.asParameter() = f.getParam(0)
    )
  }

  predicate isSink(DataFlow::Node node) {
    exists(Call call |
      call.getLocation().getFile().getRelativePath() = "src/main.rs" and
      call.getTargetName() = "effect_sink" and
      node.asExpr() = call.getPositionalArgument(0)
    )
  }
}
```

These name selectors are deliberately limited to the tiny fixture. For product
queries, use the enumerated resolved identity and appropriate implementation/
location constraints; challenge a same-named unrelated function. An API argument
is a valid source when the question concerns the caller's untrusted input;
external input need not originate at `main` or a built-in threat-model source.

Before computing flow, run this separate `Selectors.ql`:

```ql
import FixtureModel
from string role, DataFlow::Node node
where
  role = "source" and FixtureConfig::isSource(node)
  or
  role = "sink" and FixtureConfig::isSink(node)
select role, node, node.getLocation().getStartLine()
```

Inspect identities as well as counts. In larger programs, several nodes can map
to one expression; do not equate row count with distinct sources or functions.
For an initial function inventory, `import rust`, select `Function f`, constrain
its file and return `f.getCanonicalPath()` plus `f.getLocation()`.

For one callable, try `DataFlow::localFlow(source, sink)` (value-preserving) or
`TaintTracking::localTaint(source, sink)` (derived influence). The fixture crosses
a helper boundary, so use a global path query in `Flow.ql`:

```ql
/**
 * @name Fixture input reaches the effect argument
 * @description Check interprocedural propagation and a constant clean control.
 * @kind path-problem
 * @id rust/nevrotick/flow-skill-control
 * @problem.severity warning
 * @precision high
 */
import FixtureModel
import codeql.rust.dataflow.TaintTracking

module Flow = TaintTracking::Global<FixtureConfig>;
import Flow::PathGraph

from Flow::PathNode source, Flow::PathNode sink
where Flow::flowPath(source, sink)
select sink.getNode(), source, sink, "Fixture input reaches the effect argument."
```

Use `DataFlow::Global<FixtureConfig>` instead when the question requires preserved
values. Taint tracking includes influence through modeled transformations; it
does not prove exact numerical equality. The path query's first column is its
alert location, followed by source and sink path nodes and the message. Merely
selecting two ordinary nodes will not supply path explanations. Metadata such as
`@precision high` describes the query author's claim, not an accuracy measurement.

## Execute through the admitted MCP

Inspect the currently exposed tool schema. The following is an ordered argument
recipe, with your actual directory/database/results paths substituted:

| Operation | Relevant arguments |
| --- | --- |
| `codeql_resolve_packs` | `{"search-path":"/work/queries"}`; record resolved versions. |
| `codeql_resolve_database` | `{"database":"/work/databases/NAME"}`; check source root and language. |
| `codeql_query_compile` | `{"query":"/work/queries/NAME/Flow.ql"}`; warnings fail. Do **not** pass the advertised `database` argument on CLI 2.27.3. |
| `codeql_query_run` | `query`, `database`, and an owned `logDir` under `/work/results`; run inventory/selectors before the path query. Use returned result paths. |
| `codeql_bqrs_info` | `file`: actual BQRS path; discover result-set schema and counts. |
| `codeql_bqrs_decode` | `files`: array of BQRS paths, `format:"json"`, `result-set:"#select"`, `entities:"url,string"`; the default labels alone may hide endpoint identity. |
| `codeql_bqrs_interpret` | `file`, `database`, `format:"sarifv2.1.0"`, owned `output`, and `t` metadata including `kind=path-problem` and the exact query `id`. |

`query_run` can also interpret path results automatically; inspect the returned
artifacts before running a redundant interpretation. SARIF `codeFlows` supplies
path steps; BQRS `#select` supplies alert rows, and the path-graph result sets have
different denominators. Preserve BQRS/logs and reconcile decoded pages against
their totals. For pagination, use `rows` with the returned `next` byte offset (or
offsets from BQRS info) as `start-at`; it is not a row number. Bound SARIF
`max-paths` explicitly when exploring many alternatives
and report it; a displayed route is not every possible route.

Inline LSP diagnostics cannot resolve imported packs in this installed server;
compile the actual file in its pack. Do not substitute a heuristic validator for
compiler success. An MCP tool error, native nonzero exit, timeout or missing
result remains a failed/incomplete stage even if another stage succeeds.
Follow native command chains to their final outcome: this pinned CLI's wrapper
uses internal exit 70 to chain into `execute queries`. A log of that handoff
alone is neither failure nor completion; check the matching execution log and
outer MCP outcome. Do not generalize that special case to arbitrary nonzero exits.

## Test the query, then investigate the program

For a new reusable query/model, retain independently authored expected endpoints
and absences. The minimum example must select both callers/sinks, return the
unsafe pair, and exclude the constant clean pair. A separate deliberately broken
query (wrong selector or overbroad barrier) must fail that expectation. Preserve
the failure; do not change the expected result to zero.

For durable query tests, use a test pack compatible with the admitted libraries,
source fixture, `.ql`/`.qlref`, and reviewed `.expected` results through
`codeql_test_run`; follow [the official test-pack layout](https://docs.github.com/en/code-security/how-tos/find-and-fix-code-vulnerabilities/scan-from-the-command-line/test-custom-queries).
The invoked `.ql` or `.qlref` and its `.expected` must share a basename. A `.qlref`
points to the query; it is not the oracle. Inspect
`.actual`/`.expected` differences and selected/executed test counts. Local policy
rejects `learn:true`; automatic acceptance is not independent validation. A
direct database run with a separately authored endpoint comparison is also
useful for a bounded exploration; identify which form actually ran.

Add controls for the mechanisms the query claims: derived strings, checked
result versus original input, ignored validation return, alternate constructor,
guard bypass, wrong method receiver/argument and unrelated same-name calls.
An independently observed path can justify a finding only after current source
and a bounded behavioral case establish reachability and consequence.

## Diagnose a missing path

1. Check extraction warnings and actual semantic inventory, target/features/cfg,
   source archive identity, source/sink membership and argument positions.
2. Inspect local flow at the last known hop. Check helper resolution, trait
   implementation, macro expansion, field store/read and dependency models.
3. Restrict to one source and use bounded partial flow. On the pinned API:

   ```ql
   int explorationLimit() { result = 5 }
   module Explore = Flow::FlowExplorationFwd<explorationLimit/0>;
   from Explore::PartialPathNode source,
        Explore::PartialPathNode node, int distance
   where Explore::partialFlow(source, node, distance)
   select source.getNode(), node.getNode(), distance
   ```

   In a separate diagnostic query, replace the entire final
   `from … where … select` block with this snippet and remove
   `import Flow::PathGraph` and the path-problem metadata for the plain table.
   Forward exploration disregards sinks.
   Reverse exploration is available from sinks; a rendered partial path needs
   `Explore::PartialPathGraph`. Neither establishes a complete flow or absence.
4. Add `isAdditionalFlowStep` or a library summary only for an independently
   understood missing semantic edge. Prefer existing models; do not connect all
   arguments to all results or raise branch limits blindly to force an answer.
   Save diagnostic variants separately and bound their cost.

## Model only the property the guard establishes

`isBarrier` is not a list of functions with reassuring names. Trace whether the
checked representation and accepting branch reach the sink; use the pinned
`DataFlow::BarrierGuard` facilities for conditional validation when appropriate.
Authentication, Serde decoding and garde success do not confer arbitrary
authorization, freshness or secrecy on every value. Recheck mutable context at
the effect; data-flow presence alone cannot establish temporal authority.

For external-library gaps, the pinned Rust library exposes source, sink, summary,
neutral, barrier and barrier-guard model predicates. Use canonical paths and
`manual` provenance with independently reviewed model controls. Its pack declares
`/**/*.model.yml` extensions; the newer documentation's JSON guidance must not be
assumed compatible. A neutral model can suppress generated behavior, so it also
needs justification. Record the model's actual loading and selected behavior;
merely adding a YAML file proves neither.

Use the [pinned model declarations](https://github.com/github/codeql/blob/a1575b98a8cc19bcfa7bb6cb41be688cb3dae65c/rust/ql/lib/codeql/rust/dataflow/internal/ModelsAsData.qll)
and [pack manifest](https://github.com/github/codeql/blob/a1575b98a8cc19bcfa7bb6cb41be688cb3dae65c/rust/ql/lib/qlpack.yml).
IPC/native FFI/persistence seams require explicit cross-boundary reasoning and
separate enforcement evidence. Finish with the
[trace/evidence record](codeql-data-flow.md#finish-and-retain-limits).
