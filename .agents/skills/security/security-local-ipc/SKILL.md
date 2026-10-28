---
name: "security-local-ipc"
description: "Implement or review NevroTick local IPC endpoint identity, Windows named-pipe permissions, bounded messages, per-operation authorization and current-generation effects."
---

# Secure local request-to-effect flow

Read [the security contract](../CONTRACT.md), architecture sections 4–6 and the relevant [native proof requirements](../references/native-proof.md). Use [implement-rust-contracts](../../implementation/implement-rust-contracts/SKILL.md) for strict decoder/domain implementation.

1. Trace each endpoint separately: public redacted status/refresh, administrative policy and private engine corrections. Endpoint access is not authorization for every command. Keep message kinds and permitted effects finite; do not expose arbitrary Win32 calls, shell commands, paths, addresses or service names.
2. Use the selected named-pipe constraints: explicit protected DACL, local-client restriction, first-instance creation and precise data rights. `FILE_GENERIC_WRITE` also grants pipe-instance creation; do not use it for ordinary clients. Failure to acquire the genuine endpoint blocks service readiness.
3. Authenticate both peers. Obtain OS client identity under identification-level client QoS, fail closed on token/reversion errors and authorize sensitive operations at their owner. A wire `is_admin`, process name or UI signature is no authority. Client-side genuine-server proof must handle service restart/PID reuse with stable handles; first-instance creation alone cannot defeat a pre-existing squatter connection. Retain the architecture's local-RPC fallback gate if that proof fails.
4. Bound header/frame bytes before allocation, per-client and aggregate work, deadlines and queue sizes. Use the architecture's currently selected profile, marking proposed numbers as such. Admit one complete versioned representation with strict fields/integers; preserve duplicate keys/raw bytes through the actual parser seam.
5. At the serialized effect owner recheck operation rights, policy/authorization revision, engine/clock generation, monotonic freshness, sequence/request reuse, ownership and cumulative correction budget. Bind exceptional approval to its exact correction/evidence/identity/revisions and finite single-use validity. Product-grant revocation and already-issued Windows group tokens have different limits; repeatedly inspecting the same token cannot promise immediate OS account revocation.
6. Define cancellation/disconnect/partial-read behavior and actual acknowledgement. Reconcile ambiguous outcome before retrying non-idempotent effects. Reconnect/restart/cancel/sign changes must not refund correction authority or resurrect consumed approvals.

Choose [qa-contract-tests](../../testing/qa-contract-tests/SKILL.md), [qa-security-boundaries](../../testing/qa-security-boundaries/SKILL.md) and lifecycle checks by changed claim. Include allowed command/identity controls, denied neighboring identity/command, squatting/restart races, stale/replayed requests and observed absence of forbidden clock/policy effects. Mock identities prove dispatch logic only. Return exact endpoint/rights, current guard, cancellation/reconciliation and evidence limits.
