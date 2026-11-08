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

For Windows certificate validation, apply
[R-05](../../../../docs/nevrotick-research-handoff.md#7-r-05--windows-trust-with-a-wrong-system-date)
and the [ordinary S2.2 trust/native-work card](../../../../tasks/s0.1-crypto-feasibility.md#ordinary-s22-trust-profile-and-native-work--5-november-2026).
Keep the crypto provider, TLS implementation and certificate-verifier policy
distinct; OS-backed primitives do not establish Schannel policy or certificate
trust. Resolve actual wrapper controls and service context before claiming
offline or bounded verification. For blocking calls and CNG owners, retain
inputs, handles and admission charges through actual completion under
[§17.1](../../../../docs/windows-time-architecture.md#171-bounded-async-and-native-work).
Use the existing crypto task's initialization/shutdown/concurrency cases;
a private-initializer cleanup result does not qualify cached or shutdown paths.

Review equality/verification of secret authenticators, tags and bearer evidence
for secret-dependent timing. Prefer the selected provider's authentication or
verification API; ordinary equality or a handwritten loop is not a substitute
where timing independence is required. Identify public/fixed-length assumptions
and review early returns, length handling and surrounding caller control flow;
a primitive's guarantee does not automatically cover its caller. Keep functional
rejection and healthy controls; statistical timing tests alone cannot establish
constant-time behavior. Preserve [ASVS V11.2.4's unresolved provider/platform scope](../../../../docs/asvs-5.0.0-applicability.md#v500-1124):
this review admits no new comparison crate or blanket constant-time claim.

For cached associations, keep accepted-observation freshness, cookie receipt,
directional-key age, original full certificate-authentication age and trust-policy
revision distinct. Cookie renewal and TLS resumption do not reset full-auth age.
Use the explicit lifetime/invalidation policy, bounded trust-change detection and
qualified elapsed-age basis; unknown age is not fresh authentication. Research
R-05 owns unresolved limits and reauthentication/resumption choices; never import
Chrony's four-week default or certificate `notAfter` as an unreviewed product
constant. Test fresh protected UDP continuing while required reauthentication or
current trust policy fails, with no downgrade. See [the research contract](../../../../docs/nevrotick-research-handoff.md#association-and-full-authentication-lifetime).

A saved time floor also needs its uncertainty assumptions and provenance. An
intact authenticated floor may be semantically wrong; follow only the approved
[supersession procedure](../../../../docs/nevrotick-research-handoff.md#floor-construction-and-supersession),
preserving consumed correction authority. A historical predictor or successful
KE/cookie exchange grants no fresh independent vote. Keep that distinction in
tests, status and downstream broker proposals.

Enforce the NTS requirement at the actual ordinary-time-query sender: usable
current association, required cookies/keys and successful protected encoding are
necessary; constructor success alone cannot survive invalidated policy/generation.
Exhaust cookies, fail/cancel KE, restore partial state and invalidate trust between
construction and send. Inspect outgoing datagrams to prove there is no plain-query
fallback and that a healthy protected exchange succeeds. Explicit plain profiles
and authorized bootstrap hints retain separate roles. Use architecture
[section 7.4](../../../../docs/windows-time-architecture.md#74-nts-implementation-contract)
and its complete-association publication contract; no expiry threshold is adopted
from an upstream default (CHECK-6/9/27).

Route needed changes through [dependency-updates](../../environment/dependency-updates/SKILL.md); use [qa-security-boundaries](../../testing/qa-security-boundaries/SKILL.md) for actual trust/cipher/provider paths. Report trust/nonce/lifetime contract, changed consumers, executed cases and unresolved OS/provider limits. NTS authenticates packets under its trust basis; it removes no delay attacks and proves no exact UTC.

When secret copies or trust-state consumers are uncertain, use
[CodeQL data-flow analysis](../../testing/references/codeql-data-flow.md) to
trace key/cookie/approval and provisional-trust sources to persistence, logs,
ordinary-session state or broker proposals. Model those facts separately;
reachability analysis neither verifies a cryptographic primitive nor proves
zeroization of all copies or platform trust behavior.
