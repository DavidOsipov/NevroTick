---
name: "security-protocol-inputs"
description: "Implement or review bounded NevroTick NTP/NTS parsing, DNS and negotiated destinations, configuration admission, protected-file reads and persistence resource limits."
---

# Admit protocol and stored input before state mutation

Read [the security contract](../CONTRACT.md), architecture sections 5–8, 12 and relevant section 18.7 decisions. Read [persistence/resources](../references/persistence-and-resources.md) only for affected storage or aggregate-budget work.

1. Trace exact bytes → framing/decoder → protocol or domain admission → current policy → state/effect. Preserve the representation needed to challenge that seam. Raw duplicate JSON members, invalid UTF-8, lengths, extension ordering and numeric spellings cannot be tested after a lossy map/text conversion.
2. Bound packet/frame/file bytes, field counts, nesting, work, in-flight exchanges, per-source history, queues and aggregate retained memory before allocation or expensive crypto. Use checked arithmetic/conversions and explicit units. Per-message limits do not bound all clients, nested work or task retention.
3. Keep NTP/NTS parsing free of unsafe code and protected effects. Verify correlation, allowed versions/modes, lengths/extension rules and authenticated protected contents before mutating source state. A matching address or timestamp is not authentication; rejecting every packet cannot qualify the parser. Use independent numeric/protocol expectations rather than roundtripping only the same code.
4. Treat configured identities, resolved addresses and NTS-negotiated destinations as distinct policy facts. Preserve server identity and association provenance across DNS changes, re-establishment and redirection; check the actual endpoint used. Do not blanket-block all private/LAN servers when administrator policy permits them, or let negotiation bypass the configured egress policy. Held certificate/revocation retrieval choices in OPEN-8 need explicit resolution, not an assumed absence of HTTP/network consumers.
5. Keep unauthenticated NTP-only mode, NTS-only tracking and opt-in bootstrap hints visibly separate. Loss, DNS/TLS/AEAD/identity failures never silently enable plain discipline, reduce two-helper requirements or alter normal source statistics/drift/leap state through hints.
6. For protected files, use strict bounded versioned records, fixed authorized locations, actual ACL/handle/path checks and atomic replacement semantics at the current owner. Reject corrupt mandatory policy/journals; optional priors follow explicit reacquisition. A safe-looking path string is not proof against reparse/replacement or check/open races.

Select real-entry [qa-adversarial-inputs](../../testing/qa-adversarial-inputs/SKILL.md), [qa-structured-fuzzing](../../testing/qa-structured-fuzzing/SKILL.md), contract and resource tests. Pair every hostile family with useful accepted neighbors and verify forbidden state/effects, bounded rejection/diagnostics and cleanup. Record raw inputs/options and reached seam; harness rejection is not product evidence.
