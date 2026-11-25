---
name: "wsl-cleanup"
description: "Analyze WSL disk usage and perform authorized cleanup of history, toolchains, caches, worktrees, verification artifacts and unused Docker data while preserving working checkouts and live services."
---

# WSL disk cleanup

Use for WSL development-host storage maintenance. This does not qualify the
NevroTick Windows product or require product AC/CHECK gates. Do not install tools,
change dependencies or modify application code merely to enable cleanup.

## Establish scope and ownership

An analysis request stays read-only. For cleanup, carry forward the user's
existing path/category approvals and exclusions; do not ask again for an already
authorized action. Resolve only material ambiguity, such as unused Docker data
versus the running database, after presenting concrete paths, sizes and effects.
Past cleanup receipts and this skill grant no new deletion authority.

Resolve the checkout root and inspect applicable AGENTS instructions. Inventory
affected repositories, including nested `.git` directories/files and registered
worktrees. Preserve staged, unstaged and untracked work unless explicitly included
in the deletion scope. A directory named `cache`, `audit`, `tmp` or `test-results`
can contain tracked assets, a dirty checkout or retained evidence.

For affected dirty repositories, record HEAD, porcelain status, staged/unstaged
diff hashes and index identity before mutation. Use `GIT_OPTIONAL_LOCKS=0` for
observational Git commands. If untracked contents could be affected, record their
file identities/content hashes as appropriate. Keep receipts small, outside
deletion targets; in-chat observations suffice for bounded work.

## Measure allocated storage

Confirm the WSL generation, distro, filesystem and mount boundaries before scanning. Useful
read-only starting points, adapted to the current environment:

```sh
df -B1 -T / /home /tmp /var
df -i / /tmp
findmnt -T /
du -x -B1 --max-depth=1 /home
```

Drill into the largest relevant directories with `du -x`; do not recursively scan
Windows mounts, `/proc`, `/sys` or every user's inaccessible tree. Report unreadable
scope. Use context-mode for large output when available. If ENOSPC prevents its
temporary files, fall back to bounded shell/Python reads without creating reports
or installing tools. Dry-run commands can also fail for lack of temporary space.

Distinguish block exhaustion, inode exhaustion, free blocks unavailable to normal
users, and deleted-but-open files. `statvfs` free/available counts and readable
`lsof +L1` or `/proc` descriptors help explain discrepancies; inaccessible processes
remain a limitation. Do not change ext4 reservations to conceal the shortage.

Compare allocated bytes with apparent size for sparse files. Docker volume totals,
database file lengths and shared image-layer sizes can greatly overstate physical
recovery. Separate measurements may double-count hard links; do not sum overlapping
parents/children. Count an inode once and check for links outside the removal set
when estimating recovery; removing one name cannot free blocks held by another.
Report the gap between same-filesystem `du -x` and `df` where relevant. Attribute
it only with evidence. Docker storage, inaccessible files and deleted-open files
are possible contributors. Final filesystem deltas determine actual recovery.

## Classify candidates by their real owner

| Candidate | Required distinction |
| --- | --- |
| npm temporary downloads, VSIX downloads, compiler/package caches | Regeneratable content versus active installations, downloads and open files |
| npx, Playwright, Rust toolchains, Cargo binaries, pipx/uv tools | Installed or running tools; inspect actual consumers before removing anything |
| Cargo targets, Node compilation output, test runner caches | Exact owned output path, tracked membership and active build/test use |
| Audit bundles, test traces, SQLite state, source snapshots | Retained replay evidence and nested dirty checkouts; size/age alone authorizes nothing |
| Git worktrees and tracked cache assets | User work; preserve exclusions and use Git's worktree management when removal is authorized |
| Journals, disabled Snap revisions, APT candidates | Log/rollback loss and privileges; review package-removal simulation before any uninstall |

Inspect relevant process cwd, executable/arguments and open descriptors, not just
directory names. No matching readable descriptor does not prove abandonment.
Present ranked targets with allocated size, purpose, current use, proposed deletion
boundary, protected children and regeneration/retention consequences.

## Execute the authorized subset

Before each deletion, recheck the exact canonical path, symlink/mount boundary,
owner and relevant file identity. Exclude protected children explicitly. If an
approved bundle contains a protected checkout, leave the containing directory and
checkout intact while removing only the authorized siblings. Avoid broad globs,
blanket home/cache deletion, `git clean` and resets.

Quiesce only attributable artifact writers when cleanup includes their outputs.
Revalidate PID start identity and process-group membership; stop the exact owned
server/supervisor gracefully with a bounded wait and verify exit. Do not kill a
shared IDE, MCP server, Docker daemon or unrelated website process to free files.
For temporary downloads, snapshot entries, skip open or changed files, and leave
concurrently created entries for a later pass. Preserve newly observed work.

Use one mutation owner. Read-only workers may inventory independent scopes.
After a deletion failure or unexpected identity/reference change, stop that target,
report the partial result and reassess; do not escalate to broader force/removal.

### Git worktrees

Use `git worktree list --porcelain` per owning repository. Distinguish the main
checkout, registered linked worktrees, stale registrations and independent
clones. Recheck missing paths on the owning host. A sandbox may hide a live
`/tmp`, `/dev/shm` or WSL mount. Inspect locks, active writers, nested repositories,
ignored artifacts, staged and unstaged changes, and commits reachable only from
the worktree. A pushed main branch establishes committed state. Local files and
independent Git metadata still require their own disposition. For a reconstructed
copy, compare the complete index and actual file bytes and modes with the retained
commit, and account for untracked files, refs and reflogs.

Use Git's native removal for exact linked paths; use force only when the authorized
scope includes discarding their local changes and artifacts. Before removing the
last reference to a detached commit, preserve an agreed recovery ref or establish
another retained ref. Verify the exact remaining registrations and the main
checkout's HEAD, status and index. Before acting on a claim that work is pushed,
check the actual remote head when it matters to the decision.

### Docker

Inspect `docker system df`, `docker image ls --all --no-trunc`, container identities
and mounts. Preserve the running database's container, image, volume, services and
packages when excluded by the user. Measure a sparse volume's allocated bytes
through an admitted read-only route; never delete Docker/containerd storage files
directly or infer permission to prune volumes from "clean Docker".

Prefer reviewed exact image IDs and non-forced removal after checking all container
references. Prune only authorized unused build cache. `--no-prune` can leave hidden
intermediate images: inspect all images and remaining references afterward, then
remove only authorized unused parents. Reclaimable totals and shared layers are
estimates, not guaranteed savings. Do not stop or recreate the live database.

### Rustup toolchains

Check `rustup toolchain list`, `rustup show active-toolchain`, `rustup default`,
and `rustup override list`. Look for repository pins, CI and scripts invoking
`cargo +VERSION` or setting `RUSTUP_TOOLCHAIN`, documented rollback versions,
and active compiler processes. Old installs may come from an MSRV search rather
than a project pin. An override for a missing directory is a separate stale
configuration entry. Clear that exact override only within the cleanup scope.
Use `rustup toolchain uninstall` for the confirmed versions, then verify the
remaining list and run the retained project compiler's version command.

### Editor and MCP caches

Distinguish VS Code extension downloads in `CachedExtensionVSIXs` from installed
extensions, workspace state and editor history. Confirm which edition and remote
host own the data. Use the Remote WSL window's **Local History: Delete All**
command when that history is in scope. If the available client cannot invoke
it, report the remaining bytes and the editor action.

For a retired MCP server, enumerate registrations and live launchers before
removing its package cache. Remove the intended registration through the owning
client, stop only attributable workers if authorized, then use the package
manager's supported exact-cache removal where available. For npm, match the
installed package to its `npm cache npx` entry. Recheck after a session or
daemon launch. A running client can retain an older configuration, and another
launcher or skill can re-fetch the package. If the cache reappears, check install
logs and process lineage before retrying removal.

### Managed history

Delete conversation history only within the selected scope.
For managed history, use the installed application's supported deletion API;
do not unlink its files or SQL-delete index rows independently.

- **Codex compression.** Check installed CLI and daemon versions and the app-server
  protocol for native cold rollout compression. Where supported, `rollout/compress`
  can reclaim space while retaining readable sessions. The acknowledgment only
  schedules a pass. Verify completion, retained thread records and representative
  compressed files. In Codex 0.172.0, this RPC requires experimental capability
  negotiation and takes no parameters. The local control socket expects
  WebSocket RPC, even through the raw `app-server proxy`.
- **Codex deletion.** Freeze approved conversation IDs and their linked families
  before any mutation. Include spawned and attached threads when checking
  boundaries. Use recorded recency and rollout mtime across the family, plus a
  fresh loaded-thread and writer check. Filename dates can be old for a recently
  resumed conversation. Protect active families, pinned or otherwise retained
  threads, and out-of-scope links. Verify the installed operation's cascade
  semantics. A delete can terminate loaded sessions in some versions.
  Where supported, use `codex delete` by exact UUID or app-server `thread/delete`.
  Serialize requests and verify disappearance of the selected rollout, session
  index entry and projected history.
- **Partial deletion and compaction.** An API error can occur after some files
  or rows are gone. Reconcile the original manifest with the remaining state.
  Retry only the exact affected family through the native operation if that
  version supports resuming the partial deletion. A recurring transaction-depth
  failure is distinct from a transient lock. Where supported, a fresh native
  client can retry with a new database connection after the same activity checks.
  Stop if the remaining state is uncertain. Read live SQLite state with a
  read-only connection that sees the WAL. Keep WAL/SHM under application custody.
  Report reusable SQLite pages (`freelist_count * page_size`) separately from
  filesystem bytes recovered. Use read-only integrity checks. A history database
  may remain the same size after records are removed. If the installed version
  cannot compact safely with clients active, use an authorized maintenance
  window and verify from the host that all clients using the database are stopped.

### Verification artifacts

Inventory the largest children and trace each to task records, admission
manifests, checksums and actual build or replay scripts. A bundle named for an
old compiler can still hold the vendor tree used by a later qualification
command. Check Cargo source replacement, environment overrides and checksum
inventories before separating vendor trees from generated caches and superseded
attempts.
Treat pinned vendor trees, archives used as acquisition evidence, and exact-path
copies of binaries and symbols as inputs until their consumers are accounted for.

Use byte comparisons and independently retained Git objects to establish
reconstructibility. Duplicate content can still be required at two paths.
Hardlink deduplication changes file identity and may fail source admission.
Preserve authored reports and current raw evidence within the applicable
retention contract. Record what raw replay or symbol access is lost when retiring
a historical input. Archive only where the applicable retention policy and
authorized scope allow it, and verify the restore path before removing originals.

## Verify and report

Re-measure the same filesystem and selected targets. Check protected Git state
against its baseline; do not overwrite concurrent changes to restore a hash.
Confirm stopped artifact writers have exited. For a preserved database, compare
container ID, running/start state, image and volume mount, then check application
health and relevant collection/count metadata through a read-only endpoint.

Report actual net allocated bytes recovered, available space and usage before/after,
removed categories, preserved exclusions, skipped targets and permission gaps.
Use current observations rather than old receipts or the sum of deletion estimates.
No full application build/test campaign is needed for host cleanup.

## Keep Windows disk reclamation separate

For WSL 2, Linux free space and the Windows VHDX's physical size are distinct. Discover the
current distro/VHDX path instead of assuming a drive, distro name or old receipt.
`fstrim` output is not an additive recovery measurement and does not prove that
Windows storage shrank. Verify installed `wsl --help` and current Microsoft guidance
before proposing VHD operations. Offline compaction or resizing can interrupt WSL
sessions; execute only within the user's authorization and measure the host result
separately. Root filesystem conversion, shrinking, trust changes and service
replacement are separate tasks, not implied cleanup steps.
