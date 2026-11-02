# DNS upstream lessons and adaptation plan

**Document:** NV-DNS-REUSE-001, revision 1.1<br>
**Date:** 2 November 2026<br>
**Owner:** David Osipov<br>
**Status:** Owner-requested source-attributed research and future implementation guidance. The mechanisms below are recommendations or conditional experiments, not implemented capabilities, admitted dependencies or completed acceptance.<br>
**Original NevroTick baseline:** [f8d47a7ea943de451abf7f7706975c38e827380a](https://github.com/DavidOsipov/NevroTick/tree/f8d47a7ea943de451abf7f7706975c38e827380a).<br>
**DNSSEC/cache follow-up baseline:** [1eaeccfa4c57ae4c9f1695ce27939bf5844d6ed3](https://github.com/DavidOsipov/NevroTick/tree/1eaeccfa4c57ae4c9f1695ce27939bf5844d6ed3), inspected 2 November 2026; the owner requested recording the deeper source findings after the initial supplement.

## 1. Purpose and authority

The owner asked which successful mechanisms from Unbound, AdGuard Home, Pi-hole and dnscrypt-proxy could improve NevroTick, then requested that the findings and their origins be recorded. This document preserves that provenance and explains what would change when adapting each mechanism to NevroTick.

Read the [system-wide Windows DNS and adaptive-cache design](system-dns-cache-design.md) first. It owns the future DNS proposal, DNS-C01–DNS-C18 acceptance cases and DNS-0–DNS-6 stages. The [roadmap](../../ROADMAP.md#5-planned-dns-over-quic-and-certificate-pinning), [owner decisions](../nevrotick-owner-decisions.md), [architecture](../windows-time-architecture.md) and [development execution plan](development-plan.md) retain their authority. DNS-R01–DNS-R18 below identify research lessons only; they do not replace canonical CHECK requirements or create another execution campaign.

### Reconciliation with the existing plan

| Existing decision or obligation | Treatment in this supplement |
| --- | --- |
| Whole-computer Windows DNS; DoQ-only upstream with leaf/intermediate pinning | Preserved. Local DNS ingress and remote transport remain different hops. No alternative-protocol fallback or universal application-interception claim. |
| DNSSEC using qualified hardware acceleration and multiple CPU cores | Preserved. Cumulative work accounting and fair scheduling refine its resource contract; no provider, worker count or speedup is selected. |
| Full query/context keys, coalescing, cancellation and generation barriers | Preserved. Add concrete cross-packet, shared-job and old-connection completion scenarios. |
| TTL/signature validity independent of popularity | Preserved and clarified. Separate chain-material reuse from an already validated answer's lifetime; separate active-operation custody from fresh-cache eligibility. Reject upward TTL clamping as a borrowed default. |
| Protected demand aggregates, privacy generations and bounded persistence | Preserved. Extend explicit handling to queued diagnostic/statistical events and export consumers. |
| Current Windows P0–P7 work, UI-M0 deferrals and first KHR-0 follow-up | Preserved. These DNS recommendations do not add blockers to current fixtures or reorder delivery. |
| Source review versus execution evidence | Preserved. No product repair, installation, DNS enrollment, crypto execution, benchmark or Windows qualification is established here. |

The main additions are precise failure mechanisms and candidate implementation contracts. The existing plan already contains general resource limits, cache correctness, coalescing and privacy requirements; those are not newly discovered omissions. Proposed numerical settings and algorithms still require evidence. Use the existing owner-decision procedure for a material unresolved choice before its dependent implementation, without repeating approval for accepted product scope.

Revision 1.1 adds DNS-R11–DNS-R18: exact-content validation reuse, chain and delegation evidence, cycle-safe dependency sharing, active-material custody, individual response construction, cause-specific failure caching, an optional SIEVE comparison and time/trust transitions. It qualifies the earlier dependency-expiry and key-prefetch wording. Existing acceptance IDs, hardware/multicore direction and current delivery order are retained; recording these findings does not select a crypto provider, eviction implementation or stale-serving policy.

## 2. Inspected sources and limits of attribution

The comparison uses these immutable source revisions. Release dates identify the inspected baseline; mutable release pages and documentation must be rechecked when implementation starts.

| Project | Inspected version and release date | Immutable source identity |
| --- | --- | --- |
| Unbound | 1.26.2; 21 October 2026 | [f0cd7067d950921427c91ade4ecf060e944f88eb](https://github.com/NLnetLabs/unbound/tree/f0cd7067d950921427c91ade4ecf060e944f88eb); [official release history](https://www.nlnetlabs.nl/projects/unbound/download/). |
| AdGuard Home | v0.107.79; 18 August 2026 | [05ba17b282da1c4393d6a4ba4db0cf519194a362](https://github.com/AdguardTeam/AdGuardHome/tree/05ba17b282da1c4393d6a4ba4db0cf519194a362); [release](https://github.com/AdguardTeam/AdGuardHome/releases/tag/v0.107.79). |
| AdGuard dnsproxy | v0.83.2, the dependency selected by the inspected AdGuard Home release | [acf2b30e05202e171bbbaf4a3176ea07edb964b3](https://github.com/AdguardTeam/dnsproxy/tree/acf2b30e05202e171bbbaf4a3176ea07edb964b3); [AdGuard Home go.mod](https://github.com/AdguardTeam/AdGuardHome/blob/05ba17b282da1c4393d6a4ba4db0cf519194a362/go.mod). Findings attributed to this library are not claims about every later AdGuard Home build. |
| Pi-hole FTL | v6.8; 17 October 2026 | [29c8b13f4bba0c4f4f47598e45a65cbf1cedac88](https://github.com/pi-hole/FTL/tree/29c8b13f4bba0c4f4f47598e45a65cbf1cedac88); [release](https://github.com/pi-hole/FTL/releases/tag/v6.8). Distinguish FTL code from its embedded dnsmasq code. |
| dnscrypt-proxy | 2.1.20; 22 October 2026 | [c33302bc279e4d298fa19b99be8b35778db9a901](https://github.com/DNSCrypt/dnscrypt-proxy/tree/c33302bc279e4d298fa19b99be8b35778db9a901); [release](https://github.com/DNSCrypt/dnscrypt-proxy/releases/tag/2.1.20). |

Each lesson separates an observed upstream mechanism, a proposed NevroTick adaptation and future discriminating checks. Source inspection supports attribution and design rationale; it does not prove the complete upstream system correct or establish NevroTick runtime behavior. Published upstream defects are examples for regression design, not findings against NevroTick. Agreement with a translated upstream implementation is not an independent oracle.

BIND was outside the requested comparison. The earlier plan's Technitium comparison remains relevant to adaptive-refresh costs but is not repeated as a fifth implementation review here.

DNSSEC responsibilities differ across these projects. Unbound has a local validator. Pi-hole enables its embedded dnsmasq validator through [dns.dnssec configuration](https://github.com/pi-hole/FTL/blob/29c8b13f4bba0c4f4f47598e45a65cbf1cedac88/src/config/dnsmasq_config.c); this is conditional behavior, not a claim about every installation. AdGuard's [enable_dnssec setting](https://adguard-dns.io/kb/adguard-home/configuration/) and dnsproxy's [addDO/Resolve path](https://github.com/AdguardTeam/dnsproxy/blob/acf2b30e05202e171bbbaf4a3176ea07edb964b3/proxy/proxy.go) request DNSSEC records and handle upstream AD state, without establishing local signature/chain verification. dnscrypt-proxy's [configureSourceRestrictions](https://github.com/DNSCrypt/dnscrypt-proxy/blob/c33302bc279e4d298fa19b99be8b35778db9a901/dnscrypt-proxy/config_loader.go) maps require_dnssec to an advertised server property. Requesting DNSSEC records, receiving AD and completing local validation remain distinct facts.

## 3. Mechanisms worth adapting

DNS-R01–DNS-R10 retain the initial cross-project lessons. The [focused DNSSEC/cache follow-up starts at DNS-R11](#dns-r11-reuse-verification-results-bound-to-exact-content-and-proof-scope); the [qualification map](#4-qualification-map-and-implementation-order) locates all eighteen lessons in the existing acceptance inventory.

### DNS-R01: Cumulative work and fair scheduling

**Origin and observed behavior.** Unbound's [packet parser](https://github.com/NLnetLabs/unbound/blob/f0cd7067d950921427c91ade4ecf060e944f88eb/util/data/msgparse.c), especially **parse_packet**, passes one decompression budget through the question, answer, authority and additional sections. The [official advisory](https://www.nlnetlabs.nl/projects/unbound/security-advisories/) for CVE-2026-96405 explains why per-name limits were insufficient.

In the [validator](https://github.com/NLnetLabs/unbound/blob/f0cd7067d950921427c91ade4ecf060e944f88eb/validator/validator.c), **val_restart** preserves validation/hash attempt counters across a state reset, while **val_can_restart** refuses another restart after relevant work limits are exhausted. **validate_suspend_setup_timer** and its callback support bounded suspension and continuation of signature validation. These are separate observed mechanisms, not proof that Unbound has one universal budget covering every transitive operation.

**Proposed adaptation.** Define cumulative accounting across NevroTick's parsing, dependency acquisition, validation and retry lifecycles. Counters may cover pointer traversal, key matching, digest/signature operations, denial-proof work, dependency depth, network attempts and created jobs. Specify each counter's charging owner and exhaustion outcome; a helper call, alternate resolver or retry cannot replenish credit.

Distinguish a waiting client's deadline from a shared job's lifetime and budget. Attaching another waiter must not reset the job budget. Cancelling one waiter must not cancel work still needed by others. Global admission and actual native-operation custody remain necessary; a single per-request limit does not bound a flood of requests, and dropping a future does not establish native completion.

Use bounded validation batches or another qualified fair scheduling policy so a finite expensive job cannot monopolize all workers. Preserve capacity for foreground DNS and the independent time service. Cooperative yielding does not preempt a synchronous crypto operation. Upstream constants and timing choices are not NevroTick defaults.

**Future checks.** Many individually acceptable compressed names exceed the aggregate packet budget; repeated resolver/validator attempts cannot replenish work; shared DNSKEY work survives one waiter's cancellation without double charging/refunding; expensive validation does not starve valid ordinary requests. Include valid packets and successful requests near each chosen limit. Measure actual work and latency, not only task cancellation.

**Placement:** DNS-1/DNS-2, with runtime qualification in the owning stage; refines DNS-C02/C10/C12/C16 and preserves DNS-C18's reserved time-service work.

### DNS-R02: Request-owned packet and reply state

**Origin and observed behavior.** Pi-hole FTL's [GHSA-xj4x-gf2g-r6m7](https://github.com/pi-hole/FTL/security/advisories/GHSA-xj4x-gf2g-r6m7), fixed in v6.8, describes EDNS data left in a process-wide record and later applied to another query or reply. The affected paths include early returns and packets without the expected pseudoheader.

The pinned [src/edns0.c](https://github.com/pi-hole/FTL/blob/29c8b13f4bba0c4f4f47598e45a65cbf1cedac88/src/edns0.c) resets validity when parsing a new packet and makes **getEDNS** check the internal request/log identifier before consuming the record. That identifier is not simply the 16-bit DNS wire ID.

**Proposed adaptation.** Give parsed question/EDNS data, policy context, validation outcome and reply-construction state an explicit request owner. Pass immutable or narrowly mutable request values through the actual pipeline. Avoid process-wide mutable "current packet", "current client" or "current reply" state. Safe Rust and a mutex alone cannot prevent semantically using one request's data for another.

Distinguish the internal operation identity, downstream DNS transaction identity and DoQ stream identity. Reused wire IDs must not create shared authority. Early rejection and response rewriting need the same isolation as the ordinary success path.

**Future checks.** A packet with ECS/EDE followed by an early-return or malformed packet and then a packet without EDNS; an upstream reply without OPT arriving after another client's EDNS-bearing request; reused IDs and deterministic concurrent interleavings. A valid neighboring request retains its own semantics in every schedule.

**Placement:** DNS-1 and later affected transformations; refines DNS-C02/C04/C10/C11. This is a regression-shaped fixture family for the existing model/sequence-testing approach, not a new fuzzer requirement.

### DNS-R03: DoQ connection ownership and delayed failure

**Origin and observed behavior.** AdGuard dnsproxy's [upstream/doq.go](https://github.com/AdguardTeam/dnsproxy/blob/acf2b30e05202e171bbbaf4a3176ea07edb964b3/upstream/doq.go) caches a QUIC connection per upstream and opens a stream for each exchange. **Exchange** makes one additional attempt after failure on a cached connection. **closeConnWithError** clears the cached connection reference only when it still identifies the connection being closed.

That identity check protects the cached reference. It does not prove every side effect of an old error is identity-guarded: the inspected function also handles an early-data rejection by resetting QUIC configuration/token state.

**Proposed adaptation.** Bind connection/session state to resolver identity, relevant network context and current trust/pin generation. An old operation may retire its own connection but must not clear or change a replacement's state. Apply this rule to all affected side effects, including resumption/token state and publication authority.

Reuse connections within their valid scope, bound stream admission and make reconnect attempts consume the overall request deadline and work budget. Retiring a generation stops new submissions; late results cannot regain authority. The main plan already requires resumption invalidation; this lesson supplies concrete ownership rules.

Do not automatically copy historical ALPN compatibility, early-data/0-RTT use or per-attempt timeout constants. Preserve the selected RFC 9250 DoQ mapping and the existing certificate-time, chain and pin contract.

**Future checks.** Connection A fails; another request installs B; A's late failure cannot evict B or reset B's current trust/session state. Repeat around pin changes, network changes, sleep/resume, cancellation and exhausted stream capacity. Verify bounded cleanup and successful exchanges on the valid replacement.

**Placement:** DNS-1/DNS-3; refines DNS-C03/C10/C11/C12 and the applicable DOQ-C01 evidence.

### DNS-R04: Endpoint health and restrained upstream selection

**Origin and observed behavior.** Unbound's [services/cache/infra.c](https://github.com/NLnetLabs/unbound/blob/f0cd7067d950921427c91ade4ecf060e944f88eb/services/cache/infra.c), including **infra_rtt_update** and **infra_get_lame_rtt**, maintains infrastructure timing/failure information and controlled probing separately from DNS answers. Its authoritative-server, zone and EDNS-specific fields are not all useful to a DoQ forwarder.

dnscrypt-proxy's [serversInfo.go](https://github.com/DNSCrypt/dnscrypt-proxy/blob/c33302bc279e4d298fa19b99be8b35778db9a901/dnscrypt-proxy/serversInfo.go) supplies two different lessons. **getWeightedCandidate** compares scores for two randomly selected distinct servers and selects one; the choice itself does not send the DNS question to both. **recoverDormantServers**, called during selection, relaxes excessive RTT penalties for sufficiently inactive servers. It is score recovery, not by itself a circuit breaker or an autonomous one-probe monitor.

**Proposed adaptation.** Start with one small endpoint-health/selection policy, not several competing controllers. Keep bounded recent observations, cooldown and a limited recovery opportunity per relevant endpoint/context. Key or invalidate state when resolver, route, network or trust policy changes. A burst of clients should not each rediscover the same failed endpoint through a full timeout.

Separate eligibility from performance ranking. A resolver that fails authentication/pinning is not made eligible by a high score. Classify transport failure, DNS-level outcomes and DNSSEC results separately; a valid NXDOMAIN is not a transport failure. A local cache hit does not measure upstream speed. DNS completion latency and QUIC transport RTT are distinct measurements; application recovery must not fight QUIC's loss-recovery controller.

For a small configured set, simple eligibility/backoff/recovery may suffice. Compare weighted selection only if it solves an observed problem. Prefer one permitted upstream per attempt; any hedged or parallel request policy needs its own bounded privacy/cost rationale. Neither source authorizes silent protocol fallback or searching indefinitely for a convenient answer.

**Future checks.** A failed endpoint and many simultaneous clients produce bounded attempts; a qualified recovery attempt restores service; a network change prevents irrelevant old observations from dominating. Correct negative replies are not penalized as network failures. A fast but inadmissible response is rejected. Retries, speculative refresh and probes share the relevant absolute budgets.

**Placement:** basic lifecycle in DNS-1/DNS-3; optional selection comparison later. Refines DNS-C03/C10/C11/C12/C18. No weighting coefficients, pause interval or speedup is adopted.

### DNS-R05: Separate the meaning and lifetime of cached state

**Origin and observed behavior.** Unbound separates reusable data through [services/cache/dns.c](https://github.com/NLnetLabs/unbound/blob/f0cd7067d950921427c91ade4ecf060e944f88eb/services/cache/dns.c), [validator/val_kcache.c](https://github.com/NLnetLabs/unbound/blob/f0cd7067d950921427c91ade4ecf060e944f88eb/validator/val_kcache.c), [validator/val_neg.c](https://github.com/NLnetLabs/unbound/blob/f0cd7067d950921427c91ade4ecf060e944f88eb/validator/val_neg.c) and [services/cache/infra.c](https://github.com/NLnetLabs/unbound/blob/f0cd7067d950921427c91ade4ecf060e944f88eb/services/cache/infra.c). Response assembly, RRsets, validation keys, denial material and endpoint observations have different uses and lifetime rules.

**Proposed adaptation.** Start with logical ownership and independent limits; do not assume a desktop resolver needs elaborate concurrent cache implementations.

| State | Permitted reuse | Governing limits |
| --- | --- | --- |
| Ready responses and reusable RRsets | Answer or assemble compatible questions | Applicable RRset/signature limits, required answer components, question-specific proof applicability and context |
| DNSKEY/DS acquisition and chain state | Avoid repeating eligible chain work | Chain/trust provenance, material's own reuse lifetime, policy generation and revalidation rules |
| Completed validation results | Skip eligible repeated checks | Exact content identity, validation scope, applicable trust/time context and publication guards; see DNS-R11 |
| Validated denial material | Synthesize only questions actually covered | Proof coverage, wildcard/delegation rules, TTL/signature validity and trust boundaries |
| Resolution-failure state | Suppress repeated unsuccessful work | Cause, compatible context, short local lifetime, controlled recovery and bounded capacity; see DNS-R16 |
| Endpoint observations | Select/recover permitted upstreams | Observation age and resolver/network/route/trust context |
| Demand aggregates | Rank bounded refresh candidates | Real-demand provenance, decay, evidence sufficiency, retention and privacy policy |

Popular names cannot prolong security validity. Random-name answer churn should not consume all capacity reserved for useful validation material. Conversely, retaining a key does not authorize reuse after its trust context expires. Persistent demand statistics remain distinct from persistent DNS answers.

Cached key/proof material cannot be newly reused outside its declared eligibility. Expiry of chain-building material alone does not revoke an independently eligible previously validated RRset. TTL=0 material may complete the transaction for which it was obtained but is not reusable fresh cache data; pinning preserves ownership, not TTL or signature validity. DNS-R14 defines the current-transaction boundary.

**Future checks.** Response eviction preserves only independently eligible shared material; expired/wrong-context material cannot be reused for a new validation; eligible prior validated answers survive unrelated chain-cache expiry. Random-name floods stay within each admitted resource class; demonstrate any claimed benefit from sharding or specialized stores before adopting them.

**Placement:** DNS-1/DNS-2/DNS-4; refines DNS-C04/C05/C10/C11/C12. This concretizes cache ownership already present in the main plan.

### DNS-R06: Reuse validated proofs of nonexistence

**Origin and observed behavior.** [RFC 8198](https://www.rfc-editor.org/rfc/rfc8198.html) describes synthesizing answers from DNSSEC-validated denial and wildcard evidence. Unbound's [val_neg_getmsg](https://github.com/NLnetLabs/unbound/blob/f0cd7067d950921427c91ade4ecf060e944f88eb/validator/val_neg.c) illustrates general NSEC reuse and a specialized NSEC3 DS path. The inspected source explicitly does not implement general aggressive NSEC3 answer synthesis.

Ordinary negative caching usually reuses a prior matching question's result. A suitable validated denial proof can cover another question, avoiding another upstream exchange. Some internal Unbound paths accept signed-but-unchecked material for subsequent processing; obtaining a synthesized message from a helper is not independently proof that it is authenticated.

**Proposed adaptation.** After the ordinary validator/cache is qualified, consider a bounded NSEC proof-reuse pilot. Expose an explicit validated-proof contract, or require complete subsequent validation before publication. Preserve exact denial semantics, wildcard/empty-nonterminal/delegation handling, trust-anchor boundaries and expiry. General NSEC3 support is a separate decision and work budget.

A locally validating forwarder may receive sufficient proof material from its DoQ upstream; full authoritative recursion is not a prerequisite for evaluating this idea. Benefit depends on actual signed-zone and proof coverage. Do not extrapolate it into a universal improvement or use unvalidated upstream flags as proof.

**Future checks.** A covered absent name avoids an upstream exchange; an uncovered name does not. Positive/wildcard, empty-nonterminal and delegation controls are not falsely denied. Expiry and trust/context changes stop reuse. Compare extra memory/validation work with saved exchanges on controlled signed zones.

**Placement:** optional experiment after the relevant DNS-2 baseline; refines DNS-C04/C05/C11/C12/C18 only when selected. It is not a new prerequisite for basic DNS delivery.

### DNS-R07: Namespace routing and validation dependencies

**Origin and observed behavior.** AdGuard dnsproxy's [proxy/upstreams.go](https://github.com/AdguardTeam/dnsproxy/blob/acf2b30e05202e171bbbaf4a3176ea07edb964b3/proxy/upstreams.go) provides domain-specific routing and **getUpstreamsForDS**. The latter selects using the name without its first label, reflecting parent-side DS placement; [proxy/proxy.go](https://github.com/AdguardTeam/dnsproxy/blob/acf2b30e05202e171bbbaf4a3176ea07edb964b3/proxy/proxy.go) uses it in upstream selection. This is a forwarding-rule implementation, not a general zone-cut discovery algorithm.

The same proxy has a distinct private reverse-resolution path that avoids the ordinary fallback branch. Its [recursion detector](https://github.com/AdguardTeam/dnsproxy/blob/acf2b30e05202e171bbbaf4a3176ea07edb964b3/proxy/recursiondetector.go) is a bounded recent-signature heuristic for a narrow path, not a universal proof of loop freedom.

**Proposed adaptation.** Define one routing contract covering exact/subdomain precedence, validation-dependency queries, private names, trust anchors and applicable Windows/VPN policy. Include the resulting context in cache lookup, shared work, refresh eligibility and publication. An unavailable private route must not silently disclose names to a public resolver.

Do not copy "strip the first label" as the validator's general delegation algorithm, or treat a default-route exclusion as public-fallback permission. All admitted upstream routes remain within DoQ policy. Report an unsupported private/VPN integration explicitly when the required authorized route is unavailable.

For loops, qualify configuration/endpoint ownership, bootstrap dependencies and bounded runtime work. DoQ uses zero message IDs and legitimate repeated questions exist; copying a recent DNS-ID/name heuristic could reject valid traffic. Private reverse resolution is conditional on its admitted scope.

**Future checks.** The same name under public and VPN contexts has independent results; switching policy cannot publish old cached or in-flight data into the new context. At a controlled delegation, A/DNSKEY/DS use their declared routes. A loop back to the owned endpoint or a bootstrap cycle terminates within budget without private-name fallback.

**Placement:** DNS-0 contracts, DNS-1/DNS-2 fixtures and DNS-3 native integration; refines DNS-C01/C03/C04/C10/C11/C12.

### DNS-R08: Admission based on work, with honest localhost limits

**Origin and observed behavior.** Pi-hole's [configuration reference](https://docs.pi-hole.net/ftldns/configfile/#dnsratelimit) and pinned [src/config/config.c](https://github.com/pi-hole/FTL/blob/29c8b13f4bba0c4f4f47598e45a65cbf1cedac88/src/config/config.c), [src/dnsmasq_interface.c](https://github.com/pi-hole/FTL/blob/29c8b13f4bba0c4f4f47598e45a65cbf1cedac88/src/dnsmasq_interface.c) and [src/gc.c](https://github.com/pi-hole/FTL/blob/29c8b13f4bba0c4f4f47598e45a65cbf1cedac88/src/gc.c) describe per-client rate limiting, REFUSED responses and recovery. The default 1,000 requests per 60 seconds and possible continued suppression are upstream settings, not proposed desktop limits.

**Proposed adaptation.** Bound incoming parsing, newly created upstream work, waiters, DNSSEC operations and speculative refresh separately, while retaining aggregate process/system caps. A shared loopback source may represent many applications through the Windows resolver. An IP-based quota cannot establish per-process identity or fairness.

Define whether cheap valid cache hits can continue when upstream-work credit is exhausted; they still need their own bounded ingress/response path. Count accepted, rejected and internally generated work distinctly. Choose overload response semantics explicitly instead of falsely returning nonexistence. A permit follows real work and is returned at most once on each completion/cancellation/error path.

**Future checks.** Multiple applications sharing one local address, bursts around window boundaries, retries after rejection, full queues and unavailable upstreams. Observe bounded created work, valid cached service where promised, eventual recovery and absence of leaked or double-refunded counters. No reliable per-application isolation claim follows without reliable attribution.

**Placement:** DNS-1/DNS-2/DNS-3, extended when DNS-5 adds refresh; refines DNS-C02/C10/C12/C16.

### DNS-R09: Statistics and privacy through the full event lifecycle

**Origin and observed behavior.** Pi-hole's [queries_to_database](https://github.com/pi-hole/FTL/blob/29c8b13f4bba0c4f4f47598e45a65cbf1cedac88/src/database/query-table.c) uses phased snapshots and places main query writes outside the shared-memory lock. Linking-table INSERTs remain inside the first locked phase; this source does not establish that all SQLite operations are lock-free.

The pinned [datastructure accessors](https://github.com/pi-hole/FTL/blob/29c8b13f4bba0c4f4f47598e45a65cbf1cedac88/src/datastructure.c), query records in [dnsmasq_interface.c](https://github.com/pi-hole/FTL/blob/29c8b13f4bba0c4f4f47598e45a65cbf1cedac88/src/dnsmasq_interface.c), and database code apply privacy handling to query-associated data and persistence. The [privacy documentation](https://docs.pi-hole.net/ftldns/privacylevels/) distinguishes hiding stored identifiers from disabling detailed logging. Do not import its older restart-behavior claims without checking the selected implementation.

[api_info_metrics](https://github.com/pi-hole/FTL/blob/29c8b13f4bba0c4f4f47598e45a65cbf1cedac88/src/api/info.c) distinguishes capacity evictions, expired records and valid/stale content. These are useful diagnostic categories without requiring domain-history export.

**Proposed adaptation.** Extend the main plan's bounded snapshots to optional analytical events, diagnostics and exports: no disk/DB wait or slow sink while holding a DNS-state lock. Use finite queues, bounded snapshots and a declared overflow/coalescing policy. Optional replaceable analytics may lose events with visible loss counters. Required security/operational routing keeps the existing [diagnostics contract](../windows-time-architecture.md#123-logging-and-support); this recommendation does not silently make required records optional or promise guaranteed delivery.

Apply field/privacy policy before formatting and buffering, and recheck the current privacy generation at consumption, persistence and export. Prefer coarse permitted counters over raw names in queues. Centralized permitted representations must cover API, UI, snapshot, diagnostics and support export, without exposing a shared machine history as one user's history.

The privacy generation already specified in the main plan remains necessary: moving writes to a background worker does not prevent old queued events or snapshots from republishing forgotten/excluded history. Keep query handling, optional learning and authenticated management authority separate.

**Future checks.** A stalled log sink, busy database/file and full event queue cannot cause unbounded memory or delay DNS beyond the declared contract. Strengthen privacy after a domain was already observed, then query it again; all affected consumers obey the new policy. Forget/exclude while an event or snapshot is delayed; old completion cannot restore the history.

Distinguish TTL expiry from eviction of still-valid data, and measure whether evicted data was subsequently useful. High eviction counts alone do not prove undersizing: one-use random names can produce churn without lost useful hits. Link cache misses, useful refresh intervals, unused work and dropped-observation counts to the existing DNS-C18 experiment.

**Placement:** enforce relevant diagnostics rules when first introduced; DNS-4 for learning/storage and DNS-5 for refresh metrics. Refines DNS-C12/C13/C14/C15/C18.

### DNS-R10: Authenticated resolver catalogs as a later convenience

**Origin and observed behavior.** dnscrypt-proxy's [sources.go](https://github.com/DNSCrypt/dnscrypt-proxy/blob/c33302bc279e4d298fa19b99be8b35778db9a901/dnscrypt-proxy/sources.go), especially **checkSignature**, **fetchFromCache** and **fetchWithCache**, checks signatures over downloaded and locally cached catalog bytes and supports multiple source URLs.

A valid catalog signature establishes provenance under the configured signing key, not resolver honesty, pin compatibility, current version or permission to replace owner-selected policy. Local cache scheduling uses file modification time; it is not cryptographic freshness or rollback protection. The content and signature files are committed separately, so do not claim an atomic two-file update from that helper.

**Conditional adaptation.** Keep explicit resolver configuration initially. If an updateable DoQ catalog later solves a real maintenance problem, design its trust root, versions/rollback protection, key lifecycle, bounded parser, crash recovery, permitted policy changes and bootstrap/time dependencies. Catalog authenticity must not automatically authorize resolver or pin replacement. Review any existing project update policy before adding an updater.

**Future checks if selected.** Wrong key, altered content, replayed older valid catalog, mismatched content/signature after interruption, unavailable mirrors and uncertain time. Preserve the declared valid configuration and required freshness policy without disabling verification. Measure operational benefit against added maintenance and trusted-state complexity.

**Placement:** optional later proposal, not a DNS-0–DNS-5 prerequisite. Link its future task to applicable DNS-C03/C11/C12/C13/C17 cases and add specific requirements only if the feature is selected.

### DNS-R11: Reuse verification results bound to exact content and proof scope

**Origin and observed behavior.** Unbound's [val_verify_rrset](https://github.com/NLnetLabs/unbound/blob/f0cd7067d950921427c91ade4ecf060e944f88eb/validator/val_utils.c) can skip cryptographic verification when an RRset is already secure or the cache supplies that status. Its [rrset_check_sec_status](https://github.com/NLnetLabs/unbound/blob/f0cd7067d950921427c91ade4ecf060e944f88eb/services/cache/rrset.c) requires a nonexpired matching RRset; rrsetdata_equal binds reuse to the data. The same file's rrset_update_sec_status compares content again before publishing a completed check, preventing a result for replaced data from approving the replacement.

The validator deliberately excludes a secure expanded wildcard from ordinary RRset-status promotion because surrounding proof context can change. The [worker's answer_from_cache](https://github.com/NLnetLabs/unbound/blob/f0cd7067d950921427c91ade4ecf060e944f88eb/daemon/worker.c) checks referenced RRsets and answer-chain consistency before serving assembled cached data as secure. These mechanisms do not imply that every upstream configuration or caller has identical validity rules.

**Proposed adaptation.** Distinguish a signature-verification result, an authenticated RRset and an authenticated answer to a particular question. Reuse a result only for its exact admitted content and applicable trust/policy/time context. Define content identity over the relevant records/signatures, not an entire client-specific wire packet or only the name/type/class. An immutable content/version identity and a current publication guard must prevent late completion from blessing replacement data.

Keep required CNAME/DNAME components and question-specific denial/wildcard evidence explicit when assembling an answer. A cached cryptographic success is not by itself proof that a wildcard is applicable to a new question. Retain validation provenance across unchanged data; invalidated context or changed content requires the corresponding reconsideration, not an unconditional global cache flush.

Define admission/replacement rules alongside reuse. Unbound's [need_to_update_rrset](https://github.com/NLnetLabs/unbound/blob/f0cd7067d950921427c91ade4ecf060e944f88eb/services/cache/rrset.c) considers expiry, security and trust rather than simple last-arrival order. NevroTick needs a small forwarding-specific transition table: distinguish unchecked, locally validated, proven insecure and failed results, legitimate replacements and expired arrivals. Do not copy the recursive resolver's entire glue/delegation hierarchy or retain secure data indefinitely.

**Future checks.** Identical eligible RRset reuse avoids a crypto-counter increment; changed records/signatures or context do not inherit it. Start checking A, replace the cache entry with B, then complete A: B remains unapproved by A. Change a CNAME component or wildcard proof context and verify whole-answer eligibility independently. Include a legitimate new secure replacement and a response rewrite that cannot inherit the original authentication claim.

**Placement:** DNS-1 data ownership and DNS-2 validation; refines DNS-C04/C05/C10/C11. Count avoided cryptographic operations separately from accelerated executions; no speedup is established by this source review.

### DNS-R12: Resume from eligible chain state and preserve typed delegation evidence

**Origin and observed behavior.** Unbound's [key_cache_obtain](https://github.com/NLnetLabs/unbound/blob/f0cd7067d950921427c91ade4ecf060e944f88eb/validator/val_kcache.c) searches the requested name and successive ancestors for a nonexpired entry. [processInit](https://github.com/NLnetLabs/unbound/blob/f0cd7067d950921427c91ade4ecf060e944f88eb/validator/validator.c) reconciles that entry with the applicable trust anchor and distinguishes good keys, established insecurity and bad-chain state; a more-specific trust anchor must not be bypassed by an ancestor shortcut.

Pi-hole's embedded dnsmasq [dnssec_validate_ds, cache_neg_ds and zone_status](https://github.com/pi-hole/FTL/blob/29c8b13f4bba0c4f4f47598e45a65cbf1cedac88/src/dnsmasq/dnssec.c) preserve whether authenticated absence of DS occurs at an actual zone cut. No DS at a label that is not a delegation does not establish that the subtree is unsigned. The file also contains domain-specific routing compatibility assumptions; those are not authenticated denial proofs.

**Proposed adaptation.** Resume new validation from the closest eligible established chain state, with trust-anchor and namespace boundaries intact. Different hostnames in the same zone may reuse applicable chain work; sharing a suffix or registrable domain alone does not establish common DNSSEC authority.

Use distinct evidence variants for authenticated DS, authenticated unsigned delegation, authenticated absence of a delegation at this label, and an unresolved/failed operation. Handle unsupported algorithms or digests under the adopted current DNSSEC rules; do not silently collapse them into an arbitrary error or unsigned result. Each evidence variant carries its provenance, scope and reuse lifetime. Missing parent routing, a timeout or absent proof must not manufacture authenticated insecurity. Any local policy exception needs a separate explicit identity.

Expiry of a cached DNSKEY/DS limits using that material for later validation; it does not by itself revoke an earlier authenticated RRset whose own TTL/signature and current trust-policy conditions remain eligible. Required components of a complete answer and chain-building material are different dependencies; [RFC 4035 section 5.3.3](https://www.rfc-editor.org/rfc/rfc4035.html#section-5.3.3) owns authenticated RRset TTL limits.

**Future checks.** Several names in one controlled zone reuse established chain work; an expired chain entry forces the appropriate new acquisition. Distinguish absence of DS at a delegation from absence of the delegation itself. A DNSKEY/DS timeout cannot become proven insecurity. A more-specific trust anchor is honored, and chain-cache eviction alone does not invalidate an independently eligible prior answer.

**Placement:** DNS-2; refines DNS-C04/C05/C11/C12. DNS-5 may later evaluate demand-derived dependency warming, using genuine client demand without counting internal key fetches as additional interest.

### DNS-R13: Share validation dependencies without creating wait cycles

**Origin and observed behavior.** In dnsmasq's [dnssec_validate](https://github.com/pi-hole/FTL/blob/29c8b13f4bba0c4f4f47598e45a65cbf1cedac88/src/dnsmasq/forward.c), an answer missing DNSKEY/DS material is stashed while the prerequisite is obtained. lookup_frec can find existing prerequisite work and attach a dependent. Before attaching, the code walks blocking_query to reject a cyclic dependency. New subqueries consume the original request's work allowance; the original operation is protected while allocating its dependency. Bogus prerequisite handling also prevents blindly repeating an uncached failed prerequisite.

**Proposed adaptation.** Model shared DNSSEC prerequisite work independently of the original client-question key. Compatible questions can share a DNSKEY/DS acquisition or verification node; context compatibility includes applicable routing, trust and policy. Check actual dependency ancestry before introducing an edge. Several consumers waiting for one node is ordinary sharing, not a cycle.

Give nodes terminal outcomes, bounded subscriber lists and explicit custody. Cancelling one waiter must not cancel work still required by another; a late completion cannot publish into a different context. Bound graph size, depth, subscribers, created jobs and actual cumulative work. A shared job needs declared charging ownership so additional waiters neither replenish budgets nor double-refund permits. Propagate terminal prerequisite failure without a refetch loop.

**Future checks.** Different names in one zone share a compatible key acquisition. Cancel one consumer while another remains. Inject a dependency cycle and observe bounded termination, alongside a valid acyclic graph with repeated shared nodes. A failed child terminates the affected parents without repeated acquisition; context change and completion races preserve current authority and exact resource accounting.

**Placement:** DNS-2, using DNS-1's ownership and admission primitives; refines DNS-C04/C10/C11/C12. This specifies cases for existing model/concurrency tools rather than selecting another scheduler or test engine.

### DNS-R14: Retain active validation material without inflating cache freshness

**Origin and observed behavior.** dnsmasq's [cache_insert](https://github.com/pi-hole/FTL/blob/29c8b13f4bba0c4f4f47598e45a65cbf1cedac88/src/dnsmasq/cache.c) raises short DNSKEY/DS TTLs to a minimum because its validation continuation retrieves prerequisites from the cache. Its comment identifies reference-counted retention until the requesting validation completes as an alternative. This exposes an ownership problem worth solving explicitly; the upstream TTL extension is not a NevroTick default.

**Proposed adaptation.** Let a bounded validation context retain the material it is currently using, independently of ordinary cache eviction. Separate physical ownership, eligibility for new cache reuse, and DNSSEC signature/trust validity. Retaining bytes proves none of the latter conditions. Specify what remains admissible when material expires while an operation is in progress and before its result is published.

[RFC 1035 section 3.2.1](https://www.rfc-editor.org/rfc/rfc1035.html#section-3.2.1) permits TTL=0 records for the current transaction without ordinary cache reuse. Preserve that usable current-transaction path instead of repeatedly losing and refetching a zero-TTL prerequisite. Define which already participating consumers belong to a legitimately shared transaction; keeping an object allocated must not create an indefinitely renewable group that admits new requests. Conversely, legitimate existing waiters need not each trigger another network acquisition solely because they share one result.

Use ownership appropriate to the chosen Rust/library graph; no particular smart pointer or native adapter is selected. Active custody ends with actual completion/cleanup, not merely cancellation of an awaiting future. A retained key cannot relax its signature-time or trust-policy checks.

**Future checks.** TTL=0 and short-TTL material completes the permitted current operation without becoming reusable fresh data for later requests. Exercise expiry near completion, eviction pressure, multiple waiters, partial cancellation and final release. Distinguish physical retention from permissible publication, and demonstrate that an operation cannot remain alive indefinitely by accumulating new consumers.

**Placement:** DNS-2; refines DNS-C04/C05/C10/C12. This sharpens DNS-R05's lifetime rules and preserves the rejection of upward TTL clamping.

### DNS-R15: Share complete data while constructing each client response independently

**Origin and observed behavior.** dnsproxy's [Resolve](https://github.com/AdguardTeam/dnsproxy/blob/acf2b30e05202e171bbbaf4a3176ea07edb964b3/proxy/proxy.go) saves original request flags/size, may add upstream DO, caches a complete response before client filtering, then filters and scrubs it. [filterMsg and filterRRSlice](https://github.com/AdguardTeam/dnsproxy/blob/acf2b30e05202e171bbbaf4a3176ea07edb964b3/proxy/cache.go) retain explicitly requested DNSSEC RR types even with DO=0, remove stored OPT and adapt AD/record inclusion. [DNSContext.scrub](https://github.com/AdguardTeam/dnsproxy/blob/acf2b30e05202e171bbbaf4a3176ea07edb964b3/proxy/dnscontext.go) prepares the current client's representation and size. These are proxy mechanisms, not local-validation evidence.

Unchecked-data handling differs: dnsproxy's cacheWorks bypasses requests with CD and Resolve restricts insertion; dnscrypt-proxy's [computeCacheKey](https://github.com/DNSCrypt/dnscrypt-proxy/blob/c33302bc279e4d298fa19b99be8b35778db9a901/dnscrypt-proxy/plugin_cache.go) partitions by DO-related state and CD as well as question identity. dnsmasq's [answer_request](https://github.com/pi-hole/FTL/blob/29c8b13f4bba0c4f4f47598e45a65cbf1cedac88/src/dnsmasq/rfc1035.c) also distinguishes a locally validated ordinary cached answer from sufficient DNSSEC material for a DO client.

**Proposed adaptation.** Store immutable reusable records, validation results and available proof material independently of any one client packet. Here an internal canonical result means a client-neutral representation; it is not a claim that cached wire bytes are universally suitable or a substitute for DNSSEC canonicalization. Decide separately whether that result is trusted and whether it contains enough material to satisfy the requested downstream answer.

Cache hits, misses, background refreshes and coalesced waiters must all reach current-client response construction. Compute ID/question handling, remaining TTL, AD/CD/DO behavior, DNSSEC-record inclusion, OPT/EDNS options, size and truncation for each waiter. Sharing upstream work must not share a leader's already filtered or truncated packet. Preserve answer-changing namespace/ECS/trust context separation, and never mutate the reusable store through client projection.

Define unchecked-result admission explicitly: bypass the validated-result store or maintain separate qualified trust classes. An unchecked result from a CD exchange must not acquire locally authenticated status through cache reuse; only qualifying local validation can confer that status. If a DO request needs proof material that was not retained, follow the declared acquisition/miss behavior rather than treating a security boolean as those records. Source helper inspection does not qualify all upstream coalescing branches; NevroTick must establish its own end-to-end invariant.

**Future checks.** Sequential and concurrent DO=0/DO=1, different AD/CD settings, small UDP versus larger EDNS/TCP, and different OPT options. An explicit DNSKEY/DS question with DO=0 retains its requested answer. Client projection leaves shared content intact. A CD=1 invalid result followed by CD=0 cannot inherit local authentication. Include both cached and newly coalesced results and a complete valid large-response control.

**Placement:** DNS-1 response ownership and DNS-2 validation/proof completeness; refines DNS-C02/C04/C05/C10/C11. This extends DNS-R02 beyond metadata isolation to the complete shared-result-to-client path.

### DNS-R16: Cache failure causes with bounded local lifetimes and recovery

**Origin and observed behavior.** Unbound's [validator completion](https://github.com/NLnetLabs/unbound/blob/f0cd7067d950921427c91ade4ecf060e944f88eb/validator/validator.c) assigns a local bogus_ttl after its bounded restart handling, preventing endless revalidation of the same failed data. The [worker cache path](https://github.com/NLnetLabs/unbound/blob/f0cd7067d950921427c91ade4ecf060e944f88eb/daemon/worker.c) can return SERVFAIL for cached bogus data when validation is required. dnsproxy's [cacheTTL](https://github.com/AdguardTeam/dnsproxy/blob/acf2b30e05202e171bbbaf4a3176ea07edb964b3/proxy/cache.go) separately classifies positive, negative and SERVFAIL responses; its particular constants and shape checks do not establish a complete local DNSSEC validator.

[RFC 9520 sections 3.2 and 3.4](https://www.rfc-editor.org/rfc/rfc9520.html#section-3.2) require resolution-failure caching, including DNSSEC validation failures. Resolution failures must be cached for at least one second and no longer than five minutes; the RFC recommends bounded backoff and resource-exhaustion controls. These are standards bounds, not a selected NevroTick default. Invalid received TTLs cannot be trusted to choose a validation-failure lifetime.

**Proposed adaptation.** Keep cause-specific resolution-failure state distinct from authenticated NXDOMAIN/NODATA, proven insecurity and endpoint-health observations. A failure entry suppresses repeated unsuccessful work for a compatible question/context; it does not assert that a name does not exist. Match failure scope to its actual cause and define client CD behavior. An ineligible unchecked result cannot contaminate the validated store.

Choose short local lifetimes and bounded capacity under the adopted standard; limit both stored entries and repeated recovery work. Do not renew a failure indefinitely just because clients keep asking. Expiry allows a controlled retry; relevant changes to time confidence, trust or routing can invalidate a cause before that expiry. Do not apply the resolution-failure TTL range indiscriminately to every endpoint-health cooldown. Explain how an independently eligible still-valid answer and a failed refresh interact without extending the answer's life or silently replacing it with false nonexistence.

**Future checks.** Repeated queries for one broken signed result cause bounded crypto/network work. Neighboring valid results remain usable. Bogus never becomes authenticated nonexistence or insecurity. After failure expiry or applicable context repair, a valid corrected answer recovers. Random unique failures respect capacity and work bounds; cached failures and coalesced waiters cannot create an unbounded retry loop.

**Placement:** DNS-1 general failure semantics and DNS-2 DNSSEC causes; refines DNS-C04/C05/C10/C11/C12. Use the main plan's same outcome/resource accounting when DNS-5 adds refresh.

### DNS-R17: Evaluate SIEVE and sharding as residency policies

**Origin and observed behavior.** dnscrypt-proxy's [cache plugin](https://github.com/DNSCrypt/dnscrypt-proxy/blob/c33302bc279e4d298fa19b99be8b35778db9a901/dnscrypt-proxy/plugin_cache.go) uses the vendored sharded SIEVE cache. [SieveCache.Get and Evict](https://github.com/DNSCrypt/dnscrypt-proxy/blob/c33302bc279e4d298fa19b99be8b35778db9a901/vendor/github.com/jedisct1/go-sieve-cache/pkg/sievecache/sievecache.go) use visited bits and deferred eviction work instead of moving an entry through an LRU order on every hit. [NewShardedWithShards](https://github.com/DNSCrypt/dnscrypt-proxy/blob/c33302bc279e4d298fa19b99be8b35778db9a901/vendor/github.com/jedisct1/go-sieve-cache/pkg/sievecache/sharded.go) partitions the configured capacity without increasing its total. The actual [SyncSieveCache.Get](https://github.com/DNSCrypt/dnscrypt-proxy/blob/c33302bc279e4d298fa19b99be8b35778db9a901/vendor/github.com/jedisct1/go-sieve-cache/pkg/sievecache/sync.go) takes an exclusive lock; this implementation is not established as lock-free.

The [original NSDI 2024 SIEVE research](https://www.usenix.org/conference/nsdi24/presentation/zhang-yazhuo) reports web-cache workload results. Those results and its prototype's concurrency properties are not measured DNS or NevroTick outcomes.

**Conditional adaptation.** Compare a simple qualified baseline with SIEVE/sharding only for a measured residency or contention problem. Keep the implementation behind the same validity/admission contracts. Eviction decides which objects remain resident; it does not authenticate data, extend TTL or choose autonomous refresh. Preserve separate budgets for bytes, entries and active validation material. A fixed entry count alone does not select a suitable memory budget for variable-size DNSSEC responses.

Measure useful misses, cache-hit CPU cost, lock contention, skewed per-shard pressure and total memory under equal workloads and limits. Do not import the upstream shard count, promise multicore scaling or add an elaborate cache because it exists elsewhere. Internal lookups and prefetch still cannot self-heat the demand model. Keep failure, proof and active-operation ownership semantics when eviction occurs.

**Future checks.** Replacement preserves valid behavior and capacity under hot-key skew, changing demand, large/small responses and random-name churn. Eviction cannot release material still owned by a live validation or make expired material eligible. A controlled comparison includes CPU/memory and lost useful hits, not only throughput or hit percentage.

**Placement:** optional DNS-6 refinement after the baseline; DNS-C12 and DNS-C18 when included in the selected comparison. No SIEVE library, dependency, implementation or performance target is adopted here.

### DNS-R18: Reconsider cached evidence when time or trust assumptions change

**Origin and observed behavior.** dnsmasq's [is_check_date](https://github.com/pi-hole/FTL/blob/29c8b13f4bba0c4f4f47598e45a65cbf1cedac88/src/dnsmasq/dnssec.c) can transition from its timestamp-file bootstrap mode to checking signature times and queue a cache purge. This illustrates invalidation when validation assumptions change; it does not qualify that timestamp heuristic or disabled-time-check mode for NevroTick.

Stale paths need separate analysis. dnsproxy's [optimistic retrieval and response filtering](https://github.com/AdguardTeam/dnsproxy/blob/acf2b30e05202e171bbbaf4a3176ea07edb964b3/proxy/cache.go) can reuse stored upstream AD without checking RRSIG absolute expiry in those helpers. dnsmasq [excludes DNSKEY/DS from ordinary stale reuse](https://github.com/pi-hole/FTL/blob/29c8b13f4bba0c4f4f47598e45a65cbf1cedac88/src/dnsmasq/cache.c), but that alone does not establish current signature eligibility for every [cached ordinary answer](https://github.com/pi-hole/FTL/blob/29c8b13f4bba0c4f4f47598e45a65cbf1cedac88/src/dnsmasq/rfc1035.c). These are scope limits of source evidence, not reproduced upstream vulnerability findings.

**Proposed adaptation.** Apply the main plan's separate elapsed-time freshness, trusted-UTC signature checks and policy/context generations to validation reuse and failure causes. Reconsider affected evidence when assumptions change; ordinary refresh with unchanged trust does not automatically require a global flush. A backward clock correction cannot resurrect an expired TTL, while either direction can alter signature-time eligibility. Preserve the explicit bootstrap authority contract rather than deriving trustworthy time from a file timestamp.

The first adaptive pilot still uses fresh answers. If stale serving is later selected, separately specify permission to serve stale data, AD eligibility, current signature-time treatment, required proof/component availability, DO/CD presentation, TTL/EDE behavior and recovery. Physical retention, active-operation pinning and a historic secure label do not establish all those conditions. [RFC 8767](https://www.rfc-editor.org/rfc/rfc8767.html) supplies stale-serving guidance; it does not replace the project's selected authentication policy.

**Future checks.** Correct time forward/backward, cross signature inception/expiry, suspend/resume, and change relevant trust/policy while a cache hit or validation completion is pending. Stale candidates, if selected, distinguish elapsed data TTL with still-valid signatures from expired signatures. Reconsider a clock-caused failure after repaired time without reviving expired ordinary data. Include unchanged-context controls that retain eligible cached work.

**Placement:** DNS-0 time/trust contracts, DNS-2 validator lifecycle and relevant DNS-3 native observations; refines DNS-C04/C05/C11. This supplies concrete scenarios for an existing requirement, not a new authority to skip checks or alter the clock.

## 4. Qualification map and implementation order

The following maps new scenario details into the existing acceptance inventory. These are future checks, not passing results. Apply a scenario when its owner first appears; do not postpone basic connection/request isolation until autonomous refresh exists.

| Lesson | Independent expected result to demonstrate | Existing acceptance owners |
| --- | --- | --- |
| DNS-R01 | Whole-operation work remains finite across helper calls, retries and shared waiters; fair scheduling preserves ordinary service. | DNS-C02/C10/C12/C16/C18 |
| DNS-R02 | EDNS/reply/context state cannot cross requests through early returns, malformed input, ID reuse or concurrency. | DNS-C02/C04/C10/C11 |
| DNS-R03 | Late failure of retired connection A cannot change replacement B; retries and cleanup stay bounded. | DNS-C03/C10/C11/C12; applicable DOQ-C01 |
| DNS-R04 | Failed endpoints do not trigger a retry storm or permanent exclusion; ranking never overrides eligibility. | DNS-C03/C10/C11/C12/C18 |
| DNS-R05 | Each data class obeys its own expiry, trust/context, custody and capacity rules; popularity adds no authority. | DNS-C04/C05/C10/C11/C12 |
| DNS-R06 | Only an applicable validated proof avoids an upstream exchange; uncovered/expired/wrong-context evidence cannot synthesize an accepted answer. | DNS-C04/C05/C11/C12/C18, only for the selected pilot |
| DNS-R07 | Query and validation routes preserve namespace policy during VPN/configuration changes and failure. | DNS-C01/C03/C04/C10/C11/C12 |
| DNS-R08 | Overload creates bounded work, honest outcomes and recovery without assuming localhost identifies an application. | DNS-C02/C10/C12/C16 |
| DNS-R09 | Slow telemetry cannot monopolize DNS resources; current privacy governs every queued/persisted/exported consumer. | DNS-C12/C13/C14/C15/C18 |
| DNS-R10 | Only catalog changes allowed by the selected authenticity/freshness/authorization contract affect configuration. | Future catalog task; applicable DNS-C03/C11/C12/C13/C17 |
| DNS-R11 | Exact-content validation reuse skips eligible repeated crypto; old results cannot approve replacements, and whole-answer proof context remains enforced. | DNS-C04/C05/C10/C11 |
| DNS-R12 | Eligible chain state is reused within trust boundaries; distinct no-DS/delegation evidence drives distinct outcomes; prior answers are not automatically revoked by chain-cache expiry. | DNS-C04/C05/C11/C12 |
| DNS-R13 | Compatible prerequisites are shared without wait cycles, cancellation interference, repeat-fetch loops or lost resource custody. | DNS-C04/C10/C11/C12 |
| DNS-R14 | Active material survives required ownership without gaining fresh-cache authority; TTL=0 current-transaction use and later requests remain distinct. | DNS-C04/C05/C10/C12 |
| DNS-R15 | Every client gets its own flags, proof inclusion, EDNS, size and TTL from unchanged reusable data; unchecked data cannot inherit local validation. | DNS-C02/C04/C05/C10/C11 |
| DNS-R16 | Failure causes suppress bounded repeated work, preserve DNS outcome distinctions and permit controlled recovery after expiry or relevant context repair. | DNS-C04/C05/C10/C11/C12 |
| DNS-R17 | A selected residency policy respects validity, active ownership and memory caps and demonstrates workload-specific benefit. | DNS-C12; DNS-C18 when part of the selected comparison |
| DNS-R18 | Changed time/trust assumptions reconsider affected evidence without TTL resurrection or automatic destruction of unchanged eligible work. | DNS-C04/C05/C11; stale-specific cases only if selected |

Recommended order within the future DNS workstream:

1. **DNS-0 contracts:** choose accounting/ownership boundaries, permitted routing and overload semantics. Inventory exact libraries and enforceable seams when implementation begins.
2. **DNS-1 baseline and DNS-2 validation:** implement cumulative limits, request/connection ownership, minimal endpoint recovery and logical cache separation alongside the ordinary protocol/validator path. Define client-specific response construction, content-bound verdict publication, typed chain/delegation evidence, shared prerequisite custody and cause-specific failure recovery at the component's first appearance. Preserve the required hardware/multicore qualification and measure avoided work separately.
3. **DNS-3 integration and DNS-4 learning:** qualify network/VPN/lifecycle behavior, bounded event delivery, native storage/privacy and useful cache diagnostics. Preserve the main plan's native identity and restoration requirements.
4. **DNS-5 adaptive comparison:** reuse those controls when adding autonomous refresh; compare with ordinary caching and demand-triggered prefetch under the same admitted transport/trust/resource assumptions.
5. **Later selected experiments:** measure NSEC proof reuse, more elaborate selection or cache structures such as SIEVE/sharding after a baseline exists. Demand-derived DNSKEY/DS warming in a forwarder needs its own comparison; Unbound's recursive prefetch-key paths do not establish that benefit. An authenticated catalog needs its own concrete maintenance need and trust design.

Use existing admitted model, property, concurrency, fuzz and mutation tools for named gaps. Maintain independent expected results, valid neighboring controls and actual enforcing consumers. Pin a future integration's full dependency/features/target tuple; translate source examples only after reviewing assumptions. No new tool installation or whole-product test campaign follows from this document.

## 5. Behaviors not selected for transfer

| Upstream behavior or tempting shortcut | Treatment for NevroTick |
| --- | --- |
| Raising short TTLs to a configured minimum | Do not transfer. The inspected dnscrypt-proxy [example configuration](https://github.com/DNSCrypt/dnscrypt-proxy/blob/c33302bc279e4d298fa19b99be8b35778db9a901/dnscrypt-proxy/example-dnscrypt-proxy.toml) sets cache_min_ttl to 2400 seconds; [getMinTTL](https://github.com/DNSCrypt/dnscrypt-proxy/blob/c33302bc279e4d298fa19b99be8b35778db9a901/dnscrypt-proxy/dnsutils.go) clamps upward, and [PluginCacheResponse.Eval](https://github.com/DNSCrypt/dnscrypt-proxy/blob/c33302bc279e4d298fa19b99be8b35778db9a901/dnscrypt-proxy/plugin_cache.go) uses it for expiry/returned TTLs. That conflicts with the existing freshness model. This is an example-setting observation, not a claim about every installation. |
| Enabling optimistic/stale answers because another product does | Keep the main plan's separate future stale-serving policy and RFC 8767 qualification. Cache retention, stale service and fresh-answer benefit remain distinct. A retained security label does not qualify current signature or AD semantics; see DNS-R18. |
| Treating Unbound's prefetch-key as a directly demonstrated forwarding optimization | Its inspected ordinary forwarding branch bypasses the recursive initialization prefetch path; [iterator.c](https://github.com/NLnetLabs/unbound/blob/f0cd7067d950921427c91ade4ecf060e944f88eb/iterator/iterator.c) also invokes key prefetch during referral processing. Evaluate demand-derived dependency warming for the actual DoQ forwarder separately. |
| Attributing one strict expiry formula to every upstream configuration | Follow the adopted DNSSEC rules and explicit NevroTick time policy. Unbound's [adjust_ttl/check_dates](https://github.com/NLnetLabs/unbound/blob/f0cd7067d950921427c91ade4ecf060e944f88eb/validator/val_sigcrypt.c) include configurable minimum-TTL, skew and date-override behavior; their presence is not an adopted NevroTick exception. |
| Copying DNSKEY/DS minimum-TTL extension or a startup no-time-check heuristic | Use explicit active-material custody and the existing time/trust contract; see DNS-R14/R18. Neither allocation lifetime nor a persisted timestamp creates new freshness or time authority. |
| Per-IP rate-limit constants from a LAN resolver | Do not treat a shared localhost address as an application identity. Select measured work limits and honest overload behavior. |
| Full Unbound recursion, authoritative-server algorithms or zone-transfer service | Outside the selected forwarding architecture unless separately requested. Borrow relevant mechanisms with their context limits. |
| Copying dnsproxy early-data behavior, old ALPN compatibility or narrow recursion heuristics wholesale | Qualify the actual NevroTick transport and lifecycle contract. A similar function name does not establish equivalent semantics. |
| Treating DO/AD flags or authenticated transport as local DNSSEC proof | Preserve actual local validation and explicit result semantics from the main plan. |
| Racing every upstream on every request | No default adoption. Compare any bounded racing policy against costs, privacy and the single-attempt baseline. |
| Adding ad blocking, DHCP, broad dashboards or alternative DNS transports | Separate product choices; not prerequisites for these lessons. |

## 6. Source reuse and maintenance

This document records mechanisms and independently stated contracts. It copies no upstream implementation and grants no dependency or license compatibility conclusion. NevroTick's chrony-derived provenance and GPL-2.0-only obligations remain controlling.

The inspected source trees have distinct license owners: [Unbound's three-clause BSD terms](https://github.com/NLnetLabs/unbound/blob/f0cd7067d950921427c91ade4ecf060e944f88eb/LICENSE), [AdGuard Home's GPLv3 text](https://github.com/AdguardTeam/AdGuardHome/blob/05ba17b282da1c4393d6a4ba4db0cf519194a362/LICENSE.txt), [dnsproxy's Apache-2.0 text](https://github.com/AdguardTeam/dnsproxy/blob/acf2b30e05202e171bbbaf4a3176ea07edb964b3/LICENSE), [FTL's EUPL 1.2 text](https://github.com/pi-hole/FTL/blob/29c8b13f4bba0c4f4f47598e45a65cbf1cedac88/LICENSE) and [dnscrypt-proxy's ISC terms](https://github.com/DNSCrypt/dnscrypt-proxy/blob/c33302bc279e4d298fa19b99be8b35778db9a901/LICENSE). Embedded [dnsmasq headers](https://github.com/pi-hole/FTL/blob/29c8b13f4bba0c4f4f47598e45a65cbf1cedac88/src/dnsmasq/config.h) have their own GPLv2-or-GPLv3 grant. Do not apply one root label to every component or assume translation into Rust removes provenance obligations. Any later code/fixture transfer requires the repository's exact-file/license/dependency review.

Before implementation, recheck the selected versions and any relevant advisories; record whether a changed upstream mechanism alters the recommendation. Preserve the distinction between an upstream fact, a NevroTick adaptation, an experiment and observed results. The future stage task owns executed evidence and disposition: retain, revise, defer or reject with reasons. Documentation publication alone completes none of DNS-C01–DNS-C18, DOQ-C01 or the canonical release checks.
