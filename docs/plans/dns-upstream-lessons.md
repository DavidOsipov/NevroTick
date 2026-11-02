# DNS upstream lessons and adaptation plan

**Document:** NV-DNS-REUSE-001, revision 1.0<br>
**Date:** 2 November 2026<br>
**Owner:** David Osipov<br>
**Status:** Owner-requested source-attributed research and future implementation guidance. The mechanisms below are recommendations or conditional experiments, not implemented capabilities, admitted dependencies or completed acceptance.<br>
**NevroTick baseline:** [f8d47a7ea943de451abf7f7706975c38e827380a](https://github.com/DavidOsipov/NevroTick/tree/f8d47a7ea943de451abf7f7706975c38e827380a).

## 1. Purpose and authority

The owner asked which successful mechanisms from Unbound, AdGuard Home, Pi-hole and dnscrypt-proxy could improve NevroTick, then requested that the findings and their origins be recorded. This document preserves that provenance and explains what would change when adapting each mechanism to NevroTick.

Read the [system-wide Windows DNS and adaptive-cache design](system-dns-cache-design.md) first. It owns the future DNS proposal, DNS-C01–DNS-C18 acceptance cases and DNS-0–DNS-6 stages. The [roadmap](../../ROADMAP.md#5-planned-dns-over-quic-and-certificate-pinning), [owner decisions](../nevrotick-owner-decisions.md), [architecture](../windows-time-architecture.md) and [development execution plan](development-plan.md) retain their authority. DNS-R01–DNS-R10 below identify research lessons only; they do not replace canonical CHECK requirements or create another execution campaign.

### Reconciliation with the existing plan

| Existing decision or obligation | Treatment in this supplement |
| --- | --- |
| Whole-computer Windows DNS; DoQ-only upstream with leaf/intermediate pinning | Preserved. Local DNS ingress and remote transport remain different hops. No alternative-protocol fallback or universal application-interception claim. |
| DNSSEC using qualified hardware acceleration and multiple CPU cores | Preserved. Cumulative work accounting and fair scheduling refine its resource contract; no provider, worker count or speedup is selected. |
| Full query/context keys, coalescing, cancellation and generation barriers | Preserved. Add concrete cross-packet, shared-job and old-connection completion scenarios. |
| TTL/signature/dependency validity independent of popularity | Preserved. Reject upward TTL clamping as a borrowed default; consider validated denial reuse only within its proof bounds. |
| Protected demand aggregates, privacy generations and bounded persistence | Preserved. Extend explicit handling to queued diagnostic/statistical events and export consumers. |
| Current Windows P0–P7 work, UI-M0 deferrals and first KHR-0 follow-up | Preserved. These DNS recommendations do not add blockers to current fixtures or reorder delivery. |
| Source review versus execution evidence | Preserved. No product repair, installation, DNS enrollment, crypto execution, benchmark or Windows qualification is established here. |

The main additions are precise failure mechanisms and candidate implementation contracts. The existing plan already contains general resource limits, cache correctness, coalescing and privacy requirements; those are not newly discovered omissions. Proposed numerical settings and algorithms still require evidence. Use the existing owner-decision procedure for a material unresolved choice before its dependent implementation, without repeating approval for accepted product scope.

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

## 3. Mechanisms worth adapting

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

**Proposed adaptation.** Start with logical ownership and independent limits; do not assume a desktop resolver needs five elaborate concurrent cache implementations.

| State | Permitted reuse | Governing limits |
| --- | --- | --- |
| Ready responses and reusable RRsets | Answer or assemble compatible questions | TTL, dependencies, validation result, question semantics and context |
| DNSKEY/DS and validation state | Avoid repeating eligible acquisition/checks | Chain/trust provenance, expiry, relevant policy generation and revalidation rules |
| Validated denial material | Synthesize only questions actually covered | Proof coverage, wildcard/delegation rules, TTL/signature validity and trust boundaries |
| Endpoint observations | Select/recover permitted upstreams | Observation age and resolver/network/route/trust context |
| Demand aggregates | Rank bounded refresh candidates | Real-demand provenance, decay, evidence sufficiency, retention and privacy policy |

Popular names cannot prolong security validity. Random-name answer churn should not consume all capacity reserved for useful validation material. Conversely, retaining a key does not authorize reuse after its trust context expires. Persistent demand statistics remain distinct from persistent DNS answers.

**Future checks.** Response eviction preserves only independently eligible shared material; an expired or wrong-context key/proof cannot authorize an answer; random-name floods stay within each admitted resource class; demonstrate any claimed benefit from sharding or specialized stores before adopting them.

**Placement:** DNS-1/DNS-2/DNS-4; refines DNS-C04/C05/C11/C12. This concretizes cache ownership already present in the main plan.

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

## 4. Qualification map and implementation order

The following maps new scenario details into the existing acceptance inventory. These are future checks, not passing results. Apply a scenario when its owner first appears; do not postpone basic connection/request isolation until autonomous refresh exists.

| Lesson | Independent expected result to demonstrate | Existing acceptance owners |
| --- | --- | --- |
| DNS-R01 | Whole-operation work remains finite across helper calls, retries and shared waiters; fair scheduling preserves ordinary service. | DNS-C02/C10/C12/C16/C18 |
| DNS-R02 | EDNS/reply/context state cannot cross requests through early returns, malformed input, ID reuse or concurrency. | DNS-C02/C04/C10/C11 |
| DNS-R03 | Late failure of retired connection A cannot change replacement B; retries and cleanup stay bounded. | DNS-C03/C10/C11/C12; applicable DOQ-C01 |
| DNS-R04 | Failed endpoints do not trigger a retry storm or permanent exclusion; ranking never overrides eligibility. | DNS-C03/C10/C11/C12/C18 |
| DNS-R05 | Each data class obeys its own expiry, trust/context and capacity rules; popularity adds no authority. | DNS-C04/C05/C11/C12 |
| DNS-R06 | Only an applicable validated proof avoids an upstream exchange; uncovered/expired/wrong-context evidence cannot synthesize an accepted answer. | DNS-C04/C05/C11/C12/C18, only for the selected pilot |
| DNS-R07 | Query and validation routes preserve namespace policy during VPN/configuration changes and failure. | DNS-C01/C03/C04/C10/C11/C12 |
| DNS-R08 | Overload creates bounded work, honest outcomes and recovery without assuming localhost identifies an application. | DNS-C02/C10/C12/C16 |
| DNS-R09 | Slow telemetry cannot monopolize DNS resources; current privacy governs every queued/persisted/exported consumer. | DNS-C12/C13/C14/C15/C18 |
| DNS-R10 | Only catalog changes allowed by the selected authenticity/freshness/authorization contract affect configuration. | Future catalog task; applicable DNS-C03/C11/C12/C13/C17 |

Recommended order within the future DNS workstream:

1. **DNS-0 contracts:** choose accounting/ownership boundaries, permitted routing and overload semantics. Inventory exact libraries and enforceable seams when implementation begins.
2. **DNS-1 baseline and DNS-2 validation:** implement cumulative limits, request/connection ownership, minimal endpoint recovery and logical cache separation alongside the ordinary protocol/validator path.
3. **DNS-3 integration and DNS-4 learning:** qualify network/VPN/lifecycle behavior, bounded event delivery, native storage/privacy and useful cache diagnostics. Preserve the main plan's native identity and restoration requirements.
4. **DNS-5 adaptive comparison:** reuse those controls when adding autonomous refresh; compare with ordinary caching and demand-triggered prefetch under the same admitted transport/trust/resource assumptions.
5. **Later selected experiments:** measure NSEC proof reuse, more elaborate selection or cache structures. An authenticated catalog needs its own concrete maintenance need and trust design.

Use existing admitted model, property, concurrency, fuzz and mutation tools for named gaps. Maintain independent expected results, valid neighboring controls and actual enforcing consumers. Pin a future integration's full dependency/features/target tuple; translate source examples only after reviewing assumptions. No new tool installation or whole-product test campaign follows from this document.

## 5. Behaviors not selected for transfer

| Upstream behavior or tempting shortcut | Treatment for NevroTick |
| --- | --- |
| Raising short TTLs to a configured minimum | Do not transfer. The inspected dnscrypt-proxy [example configuration](https://github.com/DNSCrypt/dnscrypt-proxy/blob/c33302bc279e4d298fa19b99be8b35778db9a901/dnscrypt-proxy/example-dnscrypt-proxy.toml) sets cache_min_ttl to 2400 seconds; [getMinTTL](https://github.com/DNSCrypt/dnscrypt-proxy/blob/c33302bc279e4d298fa19b99be8b35778db9a901/dnscrypt-proxy/dnsutils.go) clamps upward, and [PluginCacheResponse.Eval](https://github.com/DNSCrypt/dnscrypt-proxy/blob/c33302bc279e4d298fa19b99be8b35778db9a901/dnscrypt-proxy/plugin_cache.go) uses it for expiry/returned TTLs. That conflicts with the existing freshness model. This is an example-setting observation, not a claim about every installation. |
| Enabling optimistic/stale answers because another product does | Keep the main plan's separate future stale-serving policy and RFC 8767 qualification. Cache retention, stale service and fresh-answer benefit remain distinct. |
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
