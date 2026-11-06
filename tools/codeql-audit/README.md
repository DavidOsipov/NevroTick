# Portable CodeQL Rust audit queries

Start with [the cloud handoff](../../tasks/codeql-cloud-handoff.md). These are
bounded propagation questions, not a complete vulnerability query suite or a
security certificate. The original product candidate is signed commit `bd1f7c4`.

The `.ql`/`.qll` files and `queries/expected.json` preserve the exact authored and
independently reviewed bytes used during local preparation. `files.sha256` binds
the portable bundle. `evidence-summary.json` distinguishes completed inventory and
selector results from failed/not-run flow stages. No CodeQL library, binary,
database, vendor cache or private installation record is included.

The pack requires `codeql/rust-all` 0.2.24 and the retained exact lock closure.
Match CLI 2.27.3 and the handoff's source/model/configuration identity; a different
version requires compatibility and control evidence. Frozen expectations describe
`bd1f7c4` product inputs. Verify relevant byte/configuration equality before reuse
on a tooling-only successor; independently update facts when product inputs change.

## Inside an already qualified isolated cloud runner

The following is a command shape, not permission to bypass repository admission
or run build scripts against an unreviewed dependency graph. Use the environment's
qualified equivalents and privately owned new output directories. Do not paste
WSL installation paths into a cloud environment.

```sh
# Set these to the admitted query pack, database and new output locations.
NV_AUDIT=/absolute/checkout/tools/codeql-audit
NV_DB=/absolute/owned/database
NV_OUT=/absolute/new/private/output

# After exact library/dependency admission and database extraction:
codeql query run "$NV_AUDIT/queries/Selectors.ql" \
  --database="$NV_DB" --output="$NV_OUT/selectors.bqrs"
codeql bqrs info "$NV_OUT/selectors.bqrs"
codeql bqrs decode "$NV_OUT/selectors.bqrs" --format=json \
  --output="$NV_OUT/selectors.json"
```

Apply reviewed finite RAM/thread/time/disk controls through the qualified runner.
Run `LocalFlow.ql`, `GlobalFlow.ql`, optional `GlobalPaths.ql` and the broken
variants only after endpoint membership matches. Decode complete results to files
or bounded pages and reconcile them with BQRS row counts. Compare rows as exact
sets against `queries/expected.json`; preserve expected absences and require the
broken outputs to fail the healthy comparison. Never accept an empty healthy
result as a new baseline.

The existing local MCP supports evaluator logging level 1 and
`tuple-counting:false` to reduce unnecessary logging. Its allowlist and sandbox
remain authoritative. Large native decoder stdout exceeded an upstream buffer;
file output succeeded for all 16,699 inventory rows. The relevant flow results
remain incomplete—do not interpret the inventory as a passed data-flow audit.
