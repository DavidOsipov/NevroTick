---
name: "security-crypto-secrets"
description: "Implement or review NevroTick TLS/NTS trust, crypto-provider and RNG choices, NTS keys/cookies, protected secret persistence and isolated wrong-date recovery."
---

# Preserve the actual cryptographic trust basis

Read [the security contract](../CONTRACT.md), architecture sections 7–8, 12 and 18.5–18.7. Use current resolved library/provider APIs; prototype version tables prove no qualified graph. Skill use grants no new primitive, verifier, random generator or dependency.

1. State security purpose, protocol, identity, key/nonce/UID scope, lifetime and failure result. Reuse maintained TLS/AEAD/OS randomness through selected owner. Pin actual provider/features; inspect license/platform/build behavior. Never rewrite crypto or treat statistical distribution tests as entropy proof.
2. Couple NTS-KE identity/algorithm/exporter context and authenticated NTP association state to real peer/provenance. Protect fresh request identifiers, key directions and replay/correlation checks. Cookie/key caches prove no freshness without fresh protected responses. Public NTP correlation has explicit weaker threat model/unresolved ASVS entropy interpretation; never claim NTS-equivalent authentication silently.
3. Preserve normal certificate checks. Wrong-date bootstrap: administrator-enabled, strict-default disabled, limited to architecture's isolated provisional TLS time provider. Plain hints never write global time or enter normal source statistics. Require bounded interval/floor/era/agreement checks, protected NTS observations and fresh isolated candidate-time validation before any broker proposal; retain bootstrap history and expired-compromised-key residual risk.
4. Time providers do not disable identity, trust-anchor, signature, algorithm or revocation policy. Windows CRL/trust evaluation may still depend on real system time, network retrieval and platform soft-failure behavior. Preserve OPEN-8's exact provider/offline/revocation/egress qualification gate; remain holdover/Blocked when required checks cannot be met. Never share relaxed resumption state with ordinary sessions.
5. Keep secrets in association owner with restricted service-SID ACLs and qualified Windows-protected persistence. Machine-scoped DPAPI alone provides no per-service isolation: other machine users with blob access can decrypt under documented scope. Inspect restart/account/machine migration and recovery. Minimize clones/plaintext lifetime; use reviewed redacted wrappers and selected zeroization, claiming no elimination of all prior copies/crash dumps.
6. Exclude keys, cookies, bearer approvals and raw sensitive packets from `Debug`, errors, traces, diagnostics and exports. Test negative decryption/trust/AEAD cases and valid neighbors without logging test secrets.

Route needed changes through [dependency-updates](../../environment/dependency-updates/SKILL.md); use [qa-security-boundaries](../../testing/qa-security-boundaries/SKILL.md) for actual trust/cipher/provider paths. Report trust/nonce/lifetime contract, changed consumers, executed cases and unresolved OS/provider limits. NTS authenticates packets under its trust basis; it removes no delay attacks and proves no exact UTC.
