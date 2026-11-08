---
name: "security-protocol-inputs"
description: "Implement or review bounded NevroTick NTP/NTS parsing, DNS and negotiated destinations, configuration admission, protected-file reads and persistence resource limits."
---

# Admit protocol and stored input before state mutation

Read [the security contract](../CONTRACT.md), architecture sections 5–8, 12 and relevant section 18.7 decisions. Read [persistence/resources](../references/persistence-and-resources.md) only for affected storage/aggregate-budget work.

1. Treat user-supplied configuration and external programmatic input as potentially malicious, including peer/DNS/IPC data, files and helper/native results at implemented seams. Local, authenticated, administrator or previously accepted origin grants no payload-safety or current-effect exemption. Trace exact bytes → bounded framing/decoder → mandatory shared garde/private domain construction or protocol-specific admission → current trusted-owner policy/budget guard → state/effect. Preserve seam-challenging representation. Lossy map/text conversion prevents testing raw duplicate JSON members, invalid UTF-8, lengths, extension ordering and numeric spellings.
2. Bound packet/frame/file bytes, field counts, nesting, work, in-flight exchanges, per-source history, queues and aggregate retained memory before allocation/expensive crypto. Use checked arithmetic/conversions and explicit units. Per-message limits cannot bound all clients, nested work or task retention.
3. Keep NTP/NTS parsing free of unsafe code/protected effects. Verify correlation, allowed versions/modes and lengths/extension rules before admitting effects. Ordinary NTS time/cookie publication additionally requires authenticated protected contents. Apply §7.4's separate correlated KoD/NAK admission to bounded control/recovery effects; it grants no time sample, cookie acceptance, authenticated freshness or plain fallback. Explicit plain profiles retain their own admission. Matching addresses/timestamps prove no authentication; rejecting every packet cannot qualify parser. Use independent numeric/protocol expectations, not only same-code roundtrips.
4. Distinguish configured identities, resolved addresses and NTS-negotiated destinations. Preserve server identity/association provenance across DNS changes, re-establishment and redirection; check actual endpoint used. Neither blanket-block administrator-permitted private/LAN servers nor let negotiation bypass configured egress policy. Resolve reached certificate/revocation retrieval through [R-05](../../../../docs/nevrotick-research-handoff.md#7-r-05--windows-trust-with-a-wrong-system-date): identify direct/delegated consumers, actual wrapper controls and their cache/revocation-timeout scope. A valid peer or signed PKI object grants no unrestricted retrieval; retain actual-job admission through completion under architecture §17.1. Assume no absence of HTTP/network consumers.
5. Keep unauthenticated NTP-only mode, NTS-only tracking and opt-in bootstrap hints visibly separate. Loss, DNS/TLS/AEAD/identity failures never silently enable plain discipline, reduce two-helper requirements or change normal source statistics/drift/leap state through hints.
6. Protected files need strict bounded versioned records, fixed authorized locations, actual ACL/handle/path checks and atomic replacement at current owner. Reject corrupt mandatory policy/journals; optional priors follow explicit reacquisition. Safe-looking paths prove no protection against reparse/replacement or check/open races.

For leap metadata or LI admission, apply
[architecture §10.5](../../../../docs/windows-time-architecture.md#105-leap-seconds-and-time-scales)
and [NV-LP-001](../../../../docs/plans/leap-second-policy.md). Bound offline dataset
ingestion; `#h` is a consistency checksum, not publisher authentication. Separate
historical validity from future coverage; add no runtime leap-file path, downloader
or trust root. Preserve raw LI through authentication and current semantic admission.
LI=0 silence remains valid; exclude a demonstrated contradictory observation
without resetting healthy identities. Require explicit unsmeared-scale provenance,
complete exchange-span and current policy/mapping-generation checks before effects.
LI claims authorize neither kernel event changes nor synthetic clock corrections.

Select real-entry [qa-adversarial-inputs](../../testing/qa-adversarial-inputs/SKILL.md), [qa-structured-fuzzing](../../testing/qa-structured-fuzzing/SKILL.md), contract and resource tests. Include malformed and valid-shaped unauthorized/stale cases, plus repeated individually valid inputs that exhaust aggregate budgets. Pair every hostile family with useful accepted neighbors; verify forbidden state/effects, bounded redacted rejection/diagnostics and cleanup. Record raw inputs/options and reached seam; harness rejection proves no product behavior.

At live acquisition/scheduling integration, trace every trigger through the engine's
single scheduling policy: ordinary/acquisition poll, refresh, resume/network change,
DNS/KE retry and admitted KoD. Preserve separate operation deadlines and budgets;
coalesce duplicate triggers and check current generations at the actual effect.
KE retry success requires protected NTP with the keys/data from that KE transaction,
not TLS completion or an unrelated old association's reply. Test actual sends under
flapping and refresh/NAK storms with a healthy neighbor (CHECK-6/9/21/27).

Before S2.1 sockets, apply architecture §7.3's association port-isolation and
lifecycle contract. Inspect actual sockets/datagrams; mocks cannot qualify native
port behavior. Preserve the separate entropy and source-independence requirements.

For KE, split representative bounded valid messages at every byte boundary and
coalesce records; retain valid noncritical-record variations. Separately test
permitted NTP packet nonce/padding/cookie forms for the negotiated AEAD.
Parse into inert staged state and publish a complete association atomically after
negotiation, keys, endpoint and current-policy checks. Failed replacement preserves
only a still-valid old association. The inspected NTPsec 2,048-byte read buffer and
in-place state writes cannot replace NevroTick's >=65,536-byte acceptance target
and atomic-publication contract; RFC 8915's size language is SHOULD. See the
[pinned assessment](../../../../tasks/s0.1-protocol-comparison.md#chrony-and-ntpsec-mechanism-assessment).

For uncertain parser, destination or persisted-input propagation, use
[CodeQL data-flow analysis](../../testing/references/codeql-data-flow.md).
Name raw bytes/negotiated names/stored fields as sources and allocations,
connections, file operations or state mutation as sinks. Validate bounds and
identity checks on each reached path; parsing and authentication sanitize only
their specific properties. Turn credible path findings into focused raw-input
regressions or bounded cargo-fuzz targets through the selected QA owner.
