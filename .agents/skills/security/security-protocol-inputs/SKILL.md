---
name: "security-protocol-inputs"
description: "Implement or review bounded NevroTick NTP/NTS parsing, DNS and negotiated destinations, configuration admission, protected-file reads and persistence resource limits."
---

# Admit protocol and stored input before state mutation

Read [the security contract](../CONTRACT.md), architecture sections 5–8, 12 and relevant section 18.7 decisions. Read [persistence/resources](../references/persistence-and-resources.md) only for affected storage/aggregate-budget work.

1. Trace exact bytes → framing/decoder → protocol or domain admission → current policy → state/effect. Preserve seam-challenging representation. Lossy map/text conversion prevents testing raw duplicate JSON members, invalid UTF-8, lengths, extension ordering and numeric spellings.
2. Bound packet/frame/file bytes, field counts, nesting, work, in-flight exchanges, per-source history, queues and aggregate retained memory before allocation/expensive crypto. Use checked arithmetic/conversions and explicit units. Per-message limits cannot bound all clients, nested work or task retention.
3. Keep NTP/NTS parsing free of unsafe code/protected effects. Verify correlation, allowed versions/modes, lengths/extension rules and authenticated protected contents before source-state mutation. Matching addresses/timestamps prove no authentication; rejecting every packet cannot qualify parser. Use independent numeric/protocol expectations, not only same-code roundtrips.
4. Distinguish configured identities, resolved addresses and NTS-negotiated destinations. Preserve server identity/association provenance across DNS changes, re-establishment and redirection; check actual endpoint used. Neither blanket-block administrator-permitted private/LAN servers nor let negotiation bypass configured egress policy. Resolve OPEN-8's held certificate/revocation retrieval choices explicitly; assume no absence of HTTP/network consumers.
5. Keep unauthenticated NTP-only mode, NTS-only tracking and opt-in bootstrap hints visibly separate. Loss, DNS/TLS/AEAD/identity failures never silently enable plain discipline, reduce two-helper requirements or change normal source statistics/drift/leap state through hints.
6. Protected files need strict bounded versioned records, fixed authorized locations, actual ACL/handle/path checks and atomic replacement at current owner. Reject corrupt mandatory policy/journals; optional priors follow explicit reacquisition. Safe-looking paths prove no protection against reparse/replacement or check/open races.

Select real-entry [qa-adversarial-inputs](../../testing/qa-adversarial-inputs/SKILL.md), [qa-structured-fuzzing](../../testing/qa-structured-fuzzing/SKILL.md), contract and resource tests. Pair every hostile family with useful accepted neighbors; verify forbidden state/effects, bounded rejection/diagnostics and cleanup. Record raw inputs/options and reached seam; harness rejection proves no product behavior.
