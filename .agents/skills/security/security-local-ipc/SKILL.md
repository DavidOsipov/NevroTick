---
name: "security-local-ipc"
description: "Implement or review NevroTick local IPC endpoint identity, Windows named-pipe permissions, bounded messages, per-operation authorization and current-generation effects."
---

# Secure local request-to-effect flow

Read [the security contract](../CONTRACT.md), architecture sections 4–6 and relevant [native proof requirements](../references/native-proof.md). Use [implement-rust-contracts](../../implementation/implement-rust-contracts/SKILL.md) for strict decoder/domain implementation.

1. Trace endpoints separately: public redacted status/refresh, administrative policy, private engine corrections. Endpoint access grants no blanket command authority. Keep message kinds/effects finite; expose no arbitrary Win32 calls, shell commands, paths, addresses or service names.
2. Use selected named-pipe constraints: explicit protected DACL, local-client restriction, first-instance creation, precise data rights. `FILE_GENERIC_WRITE` also grants pipe-instance creation; never grant ordinary clients this right. Failure to acquire genuine endpoint blocks service readiness.
3. Authenticate both peers. Obtain OS client identity under identification-level client QoS; fail closed on token/reversion errors. Authorize sensitive operations at owner. Wire `is_admin`, process names or UI signatures grant no authority. Genuine-server proof must handle restart/PID reuse with stable handles; first-instance creation alone cannot defeat pre-existing squatter connections. Retain architecture's local-RPC fallback gate if proof fails.
4. Bound header/frame bytes before allocation, per-client/aggregate work, deadlines and queues. Use architecture's current selected profile; label proposed numbers. Admit one complete versioned representation with strict fields/integers; preserve duplicate keys/raw bytes through actual parser seam.
   Dispatch consumes private domain values from mandatory shared garde/domain
   construction. Direct module callers must take the same path; UI validation,
   a derived trait or a sender-supplied validated marker grants no bypass.
5. Serialized effect owner rechecks operation rights, policy/authorization revision, engine/clock generation, monotonic freshness, sequence/request reuse, ownership and cumulative correction budget. Bind exceptional approval to exact correction/evidence/identity/revisions and finite single-use validity. Product-grant revocation and issued Windows group tokens have different limits; repeated inspection of same token cannot promise immediate OS account revocation.
6. Define cancellation/disconnect/partial-read behavior and actual acknowledgement. Reconcile ambiguous outcomes before retrying non-idempotent effects. Reconnect/restart/cancel/sign changes must not refund correction authority or resurrect consumed approvals.

Select [qa-contract-tests](../../testing/qa-contract-tests/SKILL.md), [qa-security-boundaries](../../testing/qa-security-boundaries/SKILL.md) and lifecycle checks by changed claim. Include allowed command/identity controls, denied neighboring identity/command, squatting/restart races, stale/replayed requests and observed absence of forbidden clock/policy effects. Mock identities prove dispatch logic only. Report exact endpoint/rights, current guard, cancellation/reconciliation and evidence limits.

For uncertain request-to-effect paths, use
[CodeQL data-flow analysis](../../testing/references/codeql-data-flow.md) to
trace decoded command fields and OS-derived identity separately through dispatch,
current guards and broker sinks. Correlate each process's modeled path at the
actual IPC seam; do not assume one database crosses processes or proves pipe
ACLs, token rights, guard/effect atomicity or current-generation enforcement.
