---
name: "security-crypto-secrets"
description: "Implement or review NevroTick TLS/NTS trust, crypto-provider and RNG choices, NTS keys/cookies, protected secret persistence and isolated wrong-date recovery."
---

# Preserve the actual cryptographic trust basis

Read [the security contract](../CONTRACT.md), architecture sections 7–8, 12 and 18.5–18.7. Use current resolved library/provider APIs; prototype version tables are not a qualified graph. No new primitive, verifier, random generator or dependency follows merely from this skill.

1. State the security purpose, protocol, identity, key/nonce/UID scope, lifetime and failure result. Reuse maintained TLS/AEAD/OS randomness through the selected owner. Pin the actual provider/features and inspect their license/platform/build behavior. Do not rewrite crypto or treat statistical distribution tests as entropy proof.
2. Keep NTS-KE identity/algorithm/exporter context and authenticated NTP association state coupled to their real peer/provenance. Protect fresh request identifiers, key directions and replay/correlation checks. Cookie/key cache possession does not establish freshness without fresh protected responses. Public NTP correlation has an explicit weaker threat model and unresolved ASVS entropy interpretation; do not silently claim NTS-equivalent authentication.
3. Preserve normal certificate checks. Wrong-date bootstrap is administrator-enabled, disabled by strict default and limited to the architecture's isolated provisional TLS time provider. Plain hints never write global time or enter normal source statistics. Require bounded interval/floor/era/agreement checks, protected NTS observations and fresh isolated candidate-time validation before any broker proposal; retain bootstrap history and expired-compromised-key residual risk.
4. A time provider does not disable identity, trust-anchor, signature, algorithm or revocation policy. Windows CRL/trust evaluation can still depend on real system time, network retrieval and platform soft-failure behavior. Preserve OPEN-8's exact provider/offline/revocation/egress qualification gate; remain holdover/Blocked when required checks cannot be met. Do not share relaxed resumption state with ordinary sessions.
5. Keep secrets in the association owner, with restricted service-SID ACLs and qualified Windows-protected persistence. Machine-scoped DPAPI alone is not per-service isolation: other machine users with blob access can decrypt under its documented scope. Inspect restart/account/machine migration and recovery behavior. Minimize clones/plaintext lifetime; use reviewed redacted wrappers and zeroization where selected, without claiming all prior copies or crash dumps are eliminated.
6. Exclude keys, cookies, bearer approvals and raw sensitive packets from `Debug`, errors, traces, diagnostics and exports. Test negative decryption/trust/AEAD cases and valid neighbors without logging the secrets used.

Route changes through [dependency-updates](../../environment/dependency-updates/SKILL.md) when needed and [qa-security-boundaries](../../testing/qa-security-boundaries/SKILL.md) for actual trust/cipher/provider paths. Return trust/nonce/lifetime contract, changed consumers, executed cases and unresolved OS/provider limits. NTS authenticates packets under its trust basis; it does not remove delay attacks or prove exact UTC.
