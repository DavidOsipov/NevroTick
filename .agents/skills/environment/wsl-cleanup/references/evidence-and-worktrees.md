# Evidence on WSL and mounted Windows drives

Apply the user's current cleanup scope. This reference supplies no deletion,
Windows Sandbox, process termination, tool uninstall or VHD maintenance authority.
Use a finite discovery/removal budget and one mutation owner. Keep one compact
cleanup ledger outside deletion targets; do not replace deleted evidence with
another large report or source snapshot.

## Find the complete footprint

1. Measure Linux root, its relevant mounts and inodes with `df`, `findmnt` and
   allocated-byte `du`. Follow the existing task/configuration paths to relocated
   artifacts. WSL evidence can reside on a mounted Windows drive rather than
   under Linux home; a free D: does not establish free Linux filesystem space.
2. Establish the actual Windows user/profile and mounted drive paths. Inspect
   known project-owned roots beneath Windows Local AppData and explicit C:/D:
   scratch/export destinations. Do not hardcode a previous username, assume the
   current drive letters, or recursively scan entire Windows drives.
   General project/evidence cleanup includes discovering these known locations
   even when the prompt mentions only WSL; discovery does not enlarge deletion
   authorization.
3. Rank top-level allocated usage, then inspect all requested evidence roots,
   including their direct archive files and small children. Follow relocation
   manifests, old task paths and source/tool configuration. A small current run
   directory can coexist with a much larger retired archive or receipt image.
4. Classify mixed bundles by child boundaries. Preserve the exact current vendor,
   SDK, compiler, tool environment and active workspace; retire eligible sibling
   logs, old targets and source snapshots. Inspect nested Git metadata, symlinks,
   mounts, hardlinks and actual consumers before promoting a parent to removal.
   Folder names, age, apparent duplication and a marker-free top level are not
   sufficient. Old-named inputs can still be selected by current configuration.
5. Cover the entire requested scope. Report remaining protected boundaries and
   the concrete reason for each, rather than stopping after a few large wins or
   labelling every mixed bundle permanently untouchable. Do not silently broaden
   evidence cleanup to conversation history, installed runtimes or source work.

## Retire images and Windows artifacts precisely

Compare apparent size, allocated blocks and filesystem availability. A receipt
image may reserve gigabytes while containing only a tiny report; another image
may be sparse and recover very little. Use an existing read-only filesystem
reader to inspect contents when needed. Never mount, execute or repair an image
merely to identify its contents. An evidence-only content result does not prove
that the image is unmounted or unused.

Check Linux descriptors, process arguments, mappings and mount/loop ownership
within readable scope. For C:/D: files, Linux observations alone do not establish
Windows handle clearance. An authorized read-only Windows process inventory can
identify consumers; output only the relevant PID/name/path facts, not arbitrary
command lines containing credentials. Keep inspection gaps explicit.

For an exact evidence file, an appropriate native exclusive-open/delete-on-close
operation can combine Windows sharing refusal and deletion of the held file.
Check the exact path, content/identity and authorization before requesting that
operation. It is a mutation, not a harmless lock probe. Refused exclusive access
means preserve the file; do not terminate an unknown owner, loosen sharing or
detach a volume. A plain Linux open or successful metadata read is insufficient.
See Microsoft's [FileShare](https://learn.microsoft.com/en-us/dotnet/api/system.io.fileshare),
[FileOptions](https://learn.microsoft.com/en-us/dotnet/api/system.io.fileoptions)
and [delete-on-close](https://learn.microsoft.com/en-us/windows-hardware/drivers/ifs/delete-on-close)
contracts. Select an API available in the installed host environment; documentation
is not an executed sharing control or native product qualification.

An active Sandbox's mapped inputs, pending output or guest data remain with its
actual owner. Host evidence cleanup does not require a Sandbox launch or a
product-native qualification campaign. Existing lab holds do not make ordinary
authorized host file cleanup into such a campaign; Sandbox effects still use
their own skill and current authorization.

## Remove worktrees without losing source

Enumerate `git worktree list --porcelain` for each repository in scope, including
owners discovered under agent-managed worktree directories and old scratch
roots. Distinguish linked worktrees, independent clones, main repositories,
stale registrations and leftover sibling evidence. Resolve the common Git
directory and actual `.git` pointer; never infer ownership from a directory name.

Before removal, check readable current consumers, locks, HEAD reachability,
staged/unstaged/untracked changes and nested repositories. Preserve otherwise
unreachable commits under recovery refs. If worktree removal is authorized but
source discard is not, preserve a small source recovery set in the owning
repository's private Git metadata: base HEAD, separate staged and working binary
diffs, index state, exact changed/untracked source bytes, modes, link targets and
deletion markers. Exclude known disposable caches/evidence, not unfamiliar source.
Verify the copies and original identities immediately before removal. A patch
does not include ordinary untracked files; an empty working diff does not prove
an empty index. Preserve recovery inputs required by split indexes if present.

Use native `git worktree remove`. Use force only when the authorized boundary
includes the disposable contents and source recovery has been verified. Do not
stash, stage, commit, reset or alter signing/hooks in the main checkout merely
to clear another worktree. Check each main checkout's original status/index and
source bytes afterward; do not overwrite concurrent edits to restore a hash.

`git worktree prune` is repository-wide and accepts no target-path operand.
Before using it, coordinate known common-Git writers and review its complete
fresh `--dry-run` output with the exact expiry options intended for mutation.
Establish ownership and authorization for every candidate, including paths on
currently unavailable mounts; preserve their unique HEADs and remaining index
recovery data. If any candidate is outside scope or unverified, defer pruning
rather than implying a targeted operation. Recheck when registrations or mounts
change, and reconcile the actual result; a dry run alone is no atomic guarantee.
A recorded path may now exist as a different directory or repository; pruning
does not authorize deleting that replacement. Worktree pruning is not `git prune`
or an aggressive object garbage-collection request. Keep source recovery refs
until explicitly retired.
After native removal, inspect leftover agent worktree parents for ordinary
evidence; do not leave large sibling artifacts solely because the worktree is gone.

## Complete and report the actual cleanup

Reconcile every removed, skipped, changed and partially removed boundary. On a
permission failure, retain the completed-path ledger and follow the main skill's
scoped owner-permission rule; do not retry a broader recursive removal. Stop on
unexplained identity, writer, ownership or mount changes. Remove only a reviewed
snapshot of cache entries, leaving newly created or active entries for a later pass.

Measure Linux, C: and D: independently after cleanup. Report available-space
deltas, actual worktrees removed, registrations pruned, source recovery locations,
protected inputs and missing inspection. Do not sum nested directories or treat
all image length as reclaimed storage. Linux free space does not establish a
smaller Windows VHDX. Deleting qualification evidence removes later replay/reuse
capability; preserve historical attribution without claiming the raw data remains.
