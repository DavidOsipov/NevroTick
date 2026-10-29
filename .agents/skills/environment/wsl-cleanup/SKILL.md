---
name: "wsl-cleanup"
description: "Analyze WSL disk usage and perform authorized cleanup of caches, build outputs, retained artifacts and unused Docker data while preserving working checkouts and live services."
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
parents/children. Final filesystem deltas determine actual recovery.

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

### Managed history and evidence

Delete retained evidence or conversation history only within the selected scope.
Preserve authored reports if requested and disclose loss of raw replay artifacts.
For managed history, use the installed application's supported deletion API;
do not unlink its history files or SQL-delete index rows independently. Serialize
database mutations and use bounded retries only for transient locks. Any selected
database integrity/compaction check must respect active writers and ownership.

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
