# Server catalogue, observations and source-quality history — discussion draft

**Status:** DRAFT; a logical inventory and storage-design discussion, not an adopted schema or implementation authorization.  
**Recorded:** 4 November 2026, Asia/Tbilisi.  
**Documentation baseline:** main commit 715a56561012d83ca0d667ef5ed1fd4ede555ad4; tree 0e454ff69e1b400dd3a8271f2ef6d4b4f1690eb9. The authoring workspace contains a verified selection of documentation, not a full source checkout or an implementation audit.  
**Requested workflow:** inventory author → independent inventory reviewer → storage-allocation author → independent critical reviewer → coordinator reconciliation.

## 1. Purpose, authority and decision status

David asked to record the discussion about safely supplied NTS, NTP and Roughtime server lists; observations and source reliability; compact storage; catalogue authentication; local database protection; and possible future NTS certificate pinning. This document identifies what information those features need, why they need it, who may produce and consume it, and what cannot be inferred from it. Logical inventory does not mean every field is persisted on every attempt.

**Later catalogue refinement, 4 November 2026:** the [binary catalogue design](server-catalogue-binary-format.md) records the owner's preference for compact binary shipment, hundreds of source records, Roughtime key/profile fields in the first schema and a future Python collection/build tool under existing `tools/`. It refines C and publisher work R; this inventory retains DATA and OPEN ownership. Exact encoding, bounds, providers and implementation remain open. Historical review reports in §9 retain their original candidates and bytes.

The [owner decisions](../nevrotick-owner-decisions.md), [architecture](../windows-time-architecture.md), [time-assurance coordination plan](time-assurance-integration.md), [protected-storage plan](windows-protected-storage.md), [MSIX plan](windows-msix-store.md), [roadmap](../../ROADMAP.md) and their existing tasks retain authority. This draft creates no second policy owner, clock writer, update mechanism or execution queue. The current Core priority, optional-storage experiment pause and Windows/fuzz execution holds remain unchanged.

The [allocation in §5](#5-proposed-storage-allocation) maps all 24 data groups to their proposed live/durable homes. It recommends compact RAM, small advisory summaries and optional detailed archives, then answers the catalogue-signature, RAM-compression, SQLite-protection and NTS-pinning questions. Existing mandatory and secret-state contracts remain separate.

### 1.1 What the discussion establishes

| Item | Status and consequence |
| --- | --- |
| A reviewed NevroTick catalogue of default-source candidates | Requested direction: accommodate hundreds of NTP/pool, NTS and Roughtime records; use external directories as discovery inputs, review provenance/operator use constraints, and distribute a controlled catalogue with the signed application release. Catalogue cardinality does not enlarge the active-source set, and a listing alone does not admit a source. |
| Initial catalogue delivery | Bundle with the complete signed MSIX and refresh through application releases. MSIX is the selected preparation target; MSI remains a delivery alternative under OD-05, not a newly selected parallel product. No independent catalogue updater is adopted. |
| Source and endpoint metadata | Retain identities, operator/dependency information, supported protocols, actual transport endpoints and provenance; country/region and optional IP hints are qualified metadata. |
| Observations and reliability assessment | Requested fields include observation time, server, operator, country/region, RTT, offset, uncertainty, verification result and protocol/version; maintain useful quality history to support exclusion or reduced preference. The scoring, retention and exclusion policy remain to be designed. |
| Future bootstrap relationship | Preserve preferred Roughtime candidate-date bootstrap, full NTS validation/acquisition, and only the conditionally admitted Khronos/Ananke-inspired NTP reserve. This draft does not implement that future profile or relax current source defaults. |
| Shipped format, signatures, compressed RAM and database protection | Prefer binary catalogue delivery over JSON. Constrained CBOR and generated immutable tables are candidates; final encoding, cryptographic provider, persistent-secret fallback and resource budgets remain unselected. The Q/H comparison and fixed JSON boundaries keep their separate scope. |
| Roughtime schema and publisher preparation | Include the public-key algorithm/bytes, exact profile, endpoint binding and provenance in the first catalogue schema. Runtime Roughtime verification remains future work. Plan a Python collector/build tool under existing `tools/`; collection does not approve sources or publish a release. |
| NTS intermediate/root pinning | Future proposal to design and qualify. It is not a current requirement to pin every public server, disable ordinary PKIX checks, or inherit the separate DoQ verifier automatically. |

The earlier discussion considered JSON/CBOR or another bounded catalogue representation. The later binary refinement makes JSON a possible publisher input or comparison reference, not the preferred shipped catalogue. SQLite for mutable operational state and compact summaries, optional Parquet with internal compression for detailed history, compressed batches and recent observations kept only in RAM remain proposals compared in §5. The illustrative seven-day/hourly and 180-day/daily windows, 32 MiB statistics budget, and seven-day/64 MiB raw-history budget are **not adopted limits**. The architecture's existing 10 MiB diagnostic-log budget has a different scope.

Existing fixed IPC/configuration/persisted-record boundaries retain their adopted bounded direct Serde/JSON decoding and mandatory private validation contracts. Considering formats for a new catalogue or history artifact does not authorize a change to those boundaries. The public **source catalogue** in this draft is also distinct from the protected-storage owner's **root/key catalogue**, which records store authority and key lifecycle.

### 1.2 Inventory classifications

Each DATA identifier is stable across the subsequent review/allocation stages. Fields are conceptual; names do not freeze a serialization or table schema.

| Mark | Meaning |
| --- | --- |
| Required | Logical information needed to satisfy the requested feature or preserve an existing contract. Its representation and persistence may still be conditional. |
| Proposed | Engineering recommendation for review; no implementation or owner acceptance claimed. |
| Conditional | Needed only when the named protocol, feature, evidence mode or persistence mode is enabled. |
| Existing owner | Adjacent state already owned by the architecture; reference and bind to it rather than duplicate it in quality history. |
| Excluded | Must not appear in ordinary catalogue/history/log/status records; a named separate owner may need it. |

Sensitivity distinguishes public release data, local operational metadata, security-sensitive policy/evidence, and secrets. A public server name is not a secret, but a history linking that name to a particular device, network and time can reveal activity. Derived values retain the provenance and limitations of their inputs.

## 2. Shared meaning and authority rules

1. **Identity is layered.** Keep logical source, operator, endpoint, actual address, protocol association and observation distinct. Several IPs or protocol names do not create independent votes. Unknown operator or upstream relationships remain unknown.
2. **Time is typed and qualified.** UTC/calendar time, NTP wire time, monotonic time, active-awake duration, signed phase, frequency and uncertainty are different quantities. Positive public offset means Windows is behind the reference. Store each quantity with a schema-defined unit, range and sign; do not assume signed 64-bit Unix nanoseconds span the project's eventual supported date range. Preserve checked conversions and any era ambiguity. Architecture §6 owns this contract.
3. **Observation time is not collection time.** Capture when the event occurred, when it was processed, and the evaluation instant where relevant. Wall-clock values from the clock being repaired carry a trust label or are unavailable. Monotonic values require their clock/boot generation and elapsed-time semantics; they cannot be compared blindly across reboot, suspend or VM restoration.
4. **Presence is explicit.** Zero is a value. Unavailable, unknown, not applicable, not run, rejected and intentionally not retained are distinct where they affect interpretation. A failed handshake has no NTP offset; an inactive reserve is not an agreeing observer.
5. **Current authority cannot be reconstructed from optional history.** Catalogue admission, administrator policy, cryptographic verification, mandatory conflict state and broker correction authority have their own owners. A persisted success flag, a good historical score, an imported archive or a valid file signature supplies none of those authorities by itself.
6. **One observation has one identity.** Processing, exporting, replaying or summarizing it must not create additional support or double-count an outcome. Counts describe controlled attempts/stages, not arbitrary numbers of hostile packets.
7. **Resource loss is observable.** Queue overflow, local cancellation, disk pressure and missing telemetry are gaps or local failures. They are not silent zeroes, successful samples or evidence that the server lied.
8. **All decoded data is bounded input.** Previously saved, authenticated or administrator-supplied records still need schema, unit, range, length, count, current-generation and authorization checks at their actual consumer. Authenticity is not semantic correctness.

## 3. Logical data inventory

### DATA-01 — Curation inputs, admission research and service-use permission

**Class:** Required provenance for a reviewed catalogue; detailed research is publisher-side. **Producer:** catalogue maintainers. **Consumers:** authorized release reviewer and bounded catalogue generator; the client needs only the admitted subset and provenance references. **Sensitivity:** mostly public; private operator correspondence stays outside the client.

| Field group | Meaning and presence |
| --- | --- |
| Discovery provenance | Input project/operator URL, exact commit or immutable revision where available, retrieval date, content digest, input schema/version and parser/reviewer revision. A retrieval timestamp is a research timestamp, not trusted client time. |
| Service identity evidence | Operator-controlled documentation, supported protocol/endpoint declarations, root-key publication provenance, time-scale statements, and unresolved discrepancies. A community directory supplies a candidate, not verified operator authority. |
| Use permission | Public-use versus restricted audience, fleet/vendor restrictions, polling/rate conditions, geographic/service restrictions, evidence URL and review state. Repository licensing and permission to send traffic are separate. |
| Pool admission | Assigned vendor-zone evidence when applicable, permitted name pattern, role in the product, sampling/population provenance and review status. No vendor allocation is asserted here. Do not build or ship an attempted enumeration of the entire public Pool. |
| Curation outcome | Admitted/rejected/pending/withdrawn, reason code, reviewer/review identity, affected logical source IDs, replacement/supersession link and uncertainty. Raw contact details are unnecessary in release metadata. |

**Raw/derived and lifetime:** retain enough publisher evidence to explain admission/removal and reproduce a release. Re-review changed endpoints, keys, restrictions and dependencies; “once listed” is not perpetual permission or health. Keep unresolved findings rather than converting missing evidence into approval. Sources: OD-03; [directory](https://github.com/jauderho/nts-servers); [Pool vendor guidance](https://www.ntppool.org/vendors.html).

### DATA-02 — Catalogue artifact, release binding and verification metadata

**Class:** Required catalogue identity; signature/update-specific fields conditional. **Producer:** authorized release process. **Consumers:** package/build verification and bounded engine catalogue loader. **Sensitivity:** public integrity metadata; trust-policy changes remain security-sensitive.

| Field group | Meaning and presence |
| --- | --- |
| Artifact identity | Catalogue ID, schema version, monotonically ordered catalogue revision within a defined lineage, release/package identity, exact payload length/digest, record counts and compatible reader range. Revision ordering is not proof of freshness after rollback. |
| Content declarations | Source/operator/endpoint/key records, dictionary identity, generator/review identity, public provenance references and explicit feature/version compatibility. Unknown required features cause rejection rather than partial permissive interpretation. |
| Authenticity envelope | If a separate signature is selected: signed context/domain, algorithm, verification-key ID, exact captured covered bytes, signature and envelope version. Any deterministic publisher encoding is specified before signing; verification does not decode and re-encode into different bytes. If release-bound digest verification is selected: its expected value must come from an independently trusted release component. |
| Key lifecycle | Public trust-root IDs, authorized successor/overlap relations, withdrawn key/revision metadata, activation/retirement rules and recovery provenance. Release signing and Roughtime server keys are different authorities. |
| Freshness inputs | Optional creation/expiry fields with their time basis, minimum accepted revision/lineage where policy requires it, and outcome for unknown current time, stale catalogue, incompatible reader or withdrawn key. Exact policy is open. |

**Raw/derived and lifetime:** authenticity metadata is raw release evidence; the loader's verification result is a current derived result bound to the exact artifact and trust generation. Revalidate on release/policy change. Retain the identity of catalogue revisions referenced by retained summaries without requiring indefinite copies of every full catalogue. Signed data can be old or false; offline/VM rollback and first-install freshness remain separate design problems. No TUF or runtime updater is selected. Sources: time-assurance §7; [MSIX plan](windows-msix-store.md); [MSIX signing](https://learn.microsoft.com/en-us/windows/msix/package/signing-package-overview).

### DATA-03 — Logical sources, operators, dependencies and geography

**Class:** Required identities, dependency provenance and a country/region field with an explicit unknown state; finer location enrichment is proposed where useful. **Producer:** catalogue curation or authorized local policy. **Consumers:** source admission, selection and explanatory UI.

| Field group | Meaning and presence |
| --- | --- |
| Stable identities | Logical source ID, operator ID, human label and endpoint IDs. Identifier lifecycle prevents reuse for an unrelated operator/key; display names are not keys. |
| Independence relationships | Known common operator, administrative control, upstream clock, hosting/site or other relevant dependency groups; relationship type, evidence reference, observed/reviewed date and confidence category. Missing relationships mean unassessed, not independent. |
| Location | Country/region as advertised, measured or explicitly unknown; optional finer site information, provenance/method and applicability to an endpoint rather than automatically to its operator. Distinguish operator domicile, server location and a coarse IP estimate; anycast geography may be variable or unknown. A required field does not justify inventing a location. |
| Time behavior | Declared UTC/leap handling, leap-smear policy identity/window/function where known, reference-clock claims, compatibility status and provenance. Self-reported stratum does not verify a physical reference clock. |
| Curation lifecycle | First admitted revision, last changed revision, withdrawn/replaced IDs and reason. Keep sufficient dictionary/tombstone information for old records to remain intelligible. |

**Sensitivity/lifetime:** release entries are public; local user sources may reveal organizational infrastructure. Curation is evidence-backed policy metadata, not a conclusion from one local observation. Changes can invalidate independence, compatibility or aggregate grouping. Geography is a selection/diagnostic hint, not authentication or an independent vote. Sources: OD-03; architecture §6; time-assurance §5 and §7; [RFC 8633](https://www.rfc-editor.org/rfc/rfc8633.html).

### DATA-04 — Declared protocol endpoints and intended service identity

**Class:** Required for admitted endpoints. **Producer:** reviewed catalogue or authorized configuration. **Consumers:** destination admission, resolver, protocol association owner.

| Field group | Meaning and presence |
| --- | --- |
| Endpoint definition | Endpoint ID, logical source ID, protocol and exact supported wire/profile identifiers, transport, normalized host/IP and explicit port. A documentation URL is a separate field; an NTP hostname is not automatically an HTTP URL. |
| NTS split | Expected NTS-KE service identity and KE host/port; allowed destination-policy reference for the NTP service negotiated by KE. Later records separately capture the negotiated NTP hostname/port and actual connected address. |
| Network constraints | Allowed address families, protocol-specific transport alternatives, destination scope/admission restrictions and public bootstrap-address hints if deliberately supplied. Optional hints include provenance/revision and invalidation rules; they are not authenticated DNS truth. |
| Compatibility | Advertised versus qualified protocol/version/algorithm capabilities, time-policy ID, status and test/review provenance. An advertisement does not prove successful current negotiation. |

**Sensitivity/lifetime:** public for shipped defaults; restricted operational metadata for local sources. Declarations are configuration, not observations. Changes create relevant source/association generations; cached addresses and negotiated destinations must be re-admitted. Do not silently equate one operator's KE name, time-service destination, TLS name and all returned addresses. DNS/bootstrap ownership stays with the existing resolver and time-assurance contracts. Source: [RFC 8915](https://www.rfc-editor.org/rfc/rfc8915.html), especially KE negotiation; architecture §§6–7.

### DATA-05 — Roughtime public-key, wire-version and time-policy profiles

**Class:** Required in the first catalogue schema for each Roughtime record; runtime consumption remains conditional on the future qualified Roughtime profile. **Producer:** reviewed catalogue/key-policy authority. **Consumers:** catalogue admission and, later, the Roughtime verifier and bootstrap evidence policy. **Sensitivity:** public keys; changes are security-sensitive.

Recognizing a complete, valid Roughtime metadata record is distinct from enabling
or qualifying its acquisition runtime. The [binary refinement](server-catalogue-binary-format.md)
owns the inert-record/compatible-projection alternatives for an early NTP/NTS
reader; unknown required semantics cannot be silently dropped. Schema support
does not construct authenticated evidence or make RGT-0 a current prerequisite.

| Field group | Meaning and presence |
| --- | --- |
| Trust identity | Logical source/operator and endpoint IDs, configured long-term public-key algorithm and exact bytes, stable key ID, catalogue provenance and key generation; an optional fingerprint is derived diagnostic metadata. A usable Roughtime record cannot omit its root key. No private server or release keys belong here. |
| Version profile | Exact admitted wire versions and transports, negotiated/observed version separately, timestamp/radius units and encoding bounds for each profile, supported date/era range and conversion policy. A generic “IETF Roughtime” label is insufficient. |
| Key lifecycle | Active/next/withdrawn states, authorized rotation evidence, overlap and invalidation conditions; delegated verification material is not automatically a new configured root. |
| Time semantics | Time-scale/leap policy, usable-interval/aging policy reference, evidence lifetime and bootstrap-eligibility conditions. These are policy references rather than hard-coded unreviewed thresholds. |

**Raw/derived and lifetime:** configured keys are admission input; a verified response binds its exact key and profile generations. Key withdrawal invalidates affected future use under the mandatory policy rule even if optional statistics are cleared. Do not learn replacement roots from unauthenticated resolution or treat catalogue signing as evidence that reported UTC is correct. The dated ecosystem census remains in time-assurance §8; the [binary refinement's key inventory](server-catalogue-binary-format.md#5-roughtime-verification-material-is-required-now) records a narrower documentary key check, not an approved roster or a replacement live census. Sources: time-assurance §§7–8; [RFC 10049](https://www.rfc-editor.org/rfc/rfc10049.html).

### DATA-06 — Authorized source/security policy and local overrides

**Class:** Existing mandatory policy owner plus conditional new fields. **Producer:** broker after an authorized helper request. **Consumers:** engine admission and broker guards; UI receives a bounded projection. **Sensitivity:** security-sensitive, potentially private source identities.

| Field group | Meaning and presence |
| --- | --- |
| Policy identity | Strict schema, revision/compare-and-swap token, authorized-change record, catalogue lineage binding and effective policy generation. |
| Source choices | Enabled profiles, admitted local sources/endpoints, explicit disables, required authentication, compatible time scales, configured minima and dependency constraints. Keep administrator restrictions separate from measured quality. |
| Recovery policy | Bootstrap branch eligibility, Roughtime and reserve policy references, trusted-bound requirements, failure classification and supported true-date/era range. No historical score grants a new reserve or preliminary clock-write path. |
| Mandatory exclusions | Explicit source/key withdrawal and any persistent security quarantine required by a separately adopted policy, with its resolution record. The existing policy owner must define quarantine authority and persistence; a quality threshold cannot create them. Define reset/uninstall/migration ownership so “clear statistics” cannot remove an applicable restriction. |
| Configuration provenance | Default-versus-user origin, import/review identity, superseded revision and bounded reason. No arbitrary executable command, SQL or file path is configuration authority. |

**Raw/derived and lifetime:** this is current authorized policy, not a statistical inference. Replacement follows architecture §12.1 durability/ownership rules; corruption must not silently select permissive defaults. Catalogue refresh and local override precedence need an explicit contract before implementation. Optional quality history may reduce preference within policy, never broaden admission, remove revocations or lower security minima.

### DATA-07 — Future per-NTS pin policy and verification evidence

**Class:** Conditional future proposal, not enabled by this draft. **Producer:** authenticated catalogue/local policy authority through DATA-06. **Consumers:** actual NTS-KE certificate verifier and association owner. **Sensitivity:** public pins in security-sensitive policy.

| Field group | Meaning and presence |
| --- | --- |
| Scope | Exact expected NTS-KE service identity, logical source and applicable port/profile; separate from negotiated NTP destination and from DoQ resolver pins. |
| Pin representation | Full-certificate fingerprint versus SPKI digest, digest algorithm, certificate role (leaf/intermediate/root) and exact matching semantics. Algorithm and representation remain unselected. |
| Policy logic | Required levels, any/all/threshold relation within and across pinsets, ordinary PKIX/name/revocation requirements and accepted-chain matching rule. A flat pinset must not claim to require both leaf and intermediate matches. |
| Rotation | Active/backup pins, authorized transition evidence, overlap, pin-policy generation, expiry/review state and recovery procedure. Expiry, mismatch and failed updates do not silently disable the additional restriction. |
| Result | Policy generation, verifier/chain-policy identity, checked/not-run/fail state and matched pin IDs/roles on the accepted path. Full certificate blobs are unnecessary ordinary history. |

**Raw/derived and lifetime:** policy pins are trusted inputs; observed certificate fingerprints are evidence, not a replacement policy. Reused sessions/associations must remain bound to current pin policy. A root restriction covers a broader issuance scope than a specific intermediate/leaf restriction; neither replaces hostname, date or ordinary path validation. SPKI continuity and alternate/cross-signed path behavior require qualification. An intermediate promoted to a trust anchor changes validation semantics and is not the proposed additive pin check. Do not label a pin failure a date error or trigger weaker authentication automatically. Sources: existing [DoQ design direction](../../ROADMAP.md#5-planned-dns-over-quic-and-certificate-pinning); [RFC 9525](https://www.rfc-editor.org/rfc/rfc9525.html); [Microsoft chain-engine configuration](https://learn.microsoft.com/en-us/windows/win32/api/wincrypt/ns-wincrypt-cert_chain_engine_config).

### DATA-08 — Resolved destinations and local network context

**Class:** Required current destination binding; historical context proposed and bounded. **Producer:** existing resolver/association/network observers. **Consumers:** destination admission, quality segmentation and current-state diagnostics.

| Field group | Meaning and presence |
| --- | --- |
| Resolution binding | Query/service identity, resolution-generation/result reference, selected address, address family, port, resolution provenance and applicable lifetime/expiry basis. Reference the resolver's owned result rather than mirror its entire cache or DNSSEC design. |
| Actual peer | NTS-KE peer address, negotiated NTP destination and actual time-packet peer as separate values; endpoint/address generation and transport. Actual peers may differ across associations. |
| Local context | Opaque installation-local network-context ID, route/interface generation, coarse connectivity/VPN/address-family context where needed, and change reason. Bound cardinality and avoid stable cross-device tracking identifiers. |
| Optional environment | Coarse latency path class, supported timestamp-source capability and relevant network transition markers. Do not collect SSID, BSSID, MAC, full routing tables or location history by default. |

**Sensitivity/lifetime:** addresses and network changes are local operational metadata; exact local/public client IPs need a specific purpose before retention. Remote IP is useful for endpoint-level diagnosis but not an operator identity. Address churn, VPN/path changes and DNS generations segment quality history; they do not prove a new independent source. Context retirement must leave old retained records interpretable without retaining every obsolete address forever.

### DATA-09 — Event identity, time bases and applicability generations

**Class:** Required common record contract. **Producer:** engine/protocol owner; broker supplies actual-effect identities. **Consumers:** all observation, estimator, aggregation and evidence consumers. **Sensitivity:** local linkable metadata.

| Field group | Meaning and presence |
| --- | --- |
| Unique identity | Opaque installation/run/boot identity as required, attempt/exchange ID, observation ID, acquisition/selection round ID and monotonically ordered event sequence within its declared scope. Do not use device serial numbers. |
| Generations | Engine incarnation, boot/power/clock mapping, applied-correction, association, source/catalogue, source-policy/key/pin and network/resolution generations when relevant. Do not duplicate a generation merely under a second name. |
| Observation timing | Qualified monotonic send/receive/event instants and clock-source identity; active-awake versus elapsed-age interpretation; optional qualified UTC/calendar value or interval and its trust/era provenance. |
| Processing timing | Completion/ingestion/evaluation instants, deadline, maximum evidence age and uncertainty growth basis. Late ingestion does not make an old observation fresh. |
| Representation | Schema-defined integer/fixed-point units, ranges, sign convention and explicit missing-value reason. Preserve a supported date representation across century-scale recovery rather than assuming one convenient epoch width is sufficient. |

**Raw/derived and lifetime:** raw instants and generation bindings support later derived comparisons. A reboot, unsuitable suspend/resume, external step, ambiguous acknowledgement or restored snapshot invalidates applicable continuity; only a qualified transformation can preserve it. A raw wall-clock timestamp cannot order retention, prove freshness or resolve an NTP era alone. Hashing an identifier does not remove its linkability.

### DATA-10 — Attempt lifecycle, stage outcomes and complete denominators

**Class:** Required attempt accounting and transient exchange state; full per-attempt persistence is not required. **Producer:** scheduler/association owner at controlled stage boundaries. **Consumers:** protocol admission, aggregate quality, bounded diagnostics and operational status.

| Field group | Meaning and presence |
| --- | --- |
| Planned work | Attempt/round/source/context IDs, requested stage (resolution, KE, time exchange, bootstrap), reason for scheduling, effective deadline and relevant policy generation. |
| Live correlation and custody | Exact current wire request UID/origin token, association/key reference, endpoint/socket binding, send/deadline and current-generation references, and pending/accepted/retired state. Keep native-operation custody/completion separate from cancellation of its waiter. A correlation token need not encode the actual local transmit time. |
| Progress | Scheduled, admitted, sent, locally completed, response received, parse-valid, authenticated, protocol-valid, estimator-admitted and selected stages as separate counts/states. Each denominator has a documented population. |
| Outcome | Success, peer timeout, network unreachable, local send/receive error, rate limiting, protocol rejection, authentication failure, policy exclusion, local cancellation, resource refusal or incomplete/unknown outcome. Preserve stage and bounded code. |
| Not attempted | Disabled/not needed/not yet due/no admitted endpoint/no budget/offline/superseded, distinguished from a sent request that received no acceptable response. |
| Accounting | Retry number, bounded packet/byte counts, duplicate/late/unmatched-response counts, telemetry loss marker and terminal-status identity. Packet flood volume must not become an arbitrary count of failed scheduled polls. |

**Sensitivity/lifetime:** local operational history. Outstanding exchange state is bounded and transient: neither history nor a secret cache restores a pending request, and no unbounded archive of prior UIDs is needed. Raw stage transitions are ephemeral unless diagnostic retention is justified; mergeable counts and sparse failure evidence may suffice. Completion uncertainty remains explicit. The denominator must include applicable failures, not only stored successful measurements, while excluding stages never run. Replaying/exporting/reprocessing an attempt must not increment it twice. Sources: architecture §§6 and 7.4.

### DATA-11 — Time measurements, residuals and uncertainty components

**Class:** Required measurement meaning; retain only fields needed by current computation or chosen summaries. **Producer:** admitted protocol sample and pure estimator. **Consumers:** estimator, selection, quality summaries and explanatory diagnostics.

| Field group | Meaning and presence |
| --- | --- |
| Timestamp inputs | During NTP sample construction, actual local transmit T1 and receive T4, admitted peer-reported receive T2 and transmit T3, and the separate wire-correlation value. Preserve local timestamp capture source, monotonic/UTC mapping, era interpretation and uncertainty provenance under DATA-09. Retaining these inputs beyond current computation requires a specific diagnostic/evidence purpose; this is not routine full-packet history. |
| Delay measures | Monotonic request/response RTT with timestamp points defined; NTP four-timestamp delay separately; KE/connect duration separately. They describe different intervals and must not share one undifferentiated “latency” column. |
| Phase measures | Raw observed Windows-relative offset, public sign convention, and optional normalized residual against a named reference at a common evaluation instant/clock frame. Reference or continuity absent means residual unavailable. |
| Uncertainty | Interval endpoints or radius with defined meaning; deterministic bound, protocol-advertised dispersion, model error or statistical estimate distinguished. Record included network-delay/asymmetry, timestamp, oscillator/aging and model components where used. |
| Numeric validity | Unit, finite/range checks, saturation/overflow status, quantization and conversion provenance. Negative/pathological delays are rejected or handled only by the qualified protocol rule, not silently changed to a plausible zero. |
| Derived variation | Jitter, frequency estimate/skew, residual spread and relevant sample counts/window/evaluator version if computed. Statistical spread is not an authenticated absolute-error bound. |

**Raw/derived and lifetime:** retain raw versus adjusted values as different fields; adjusted values name DATA-14's actual-effect/reference dependencies. A large raw offset can mean the local clock is wrong. Measurements from different clock epochs, time scales, paths or references are not automatically mergeable. NTS authentication does not establish symmetric network delay. Sources: architecture §6; [RFC 5905](https://www.rfc-editor.org/rfc/rfc5905.html); [RFC 8915](https://www.rfc-editor.org/rfc/rfc8915.html).

### DATA-12 — NTP/NTS verification and eligibility summaries

**Class:** Conditional on the protocol; required current verification semantics. **Producer:** protocol admission and NTS-KE verifier. **Consumers:** estimator/admission, quality classification and bounded status. **Sensitivity:** local security evidence; association secrets excluded.

| Field group | Meaning and presence |
| --- | --- |
| Protocol context | NTP mode/version, stratum, leap indicator, root delay/dispersion, relevant precision/poll fields and bounded reference/KoD classification only where used for eligibility or explanation. Server declarations remain claims. |
| Exchange checks | Request-match, duplicate/replay, timestamp/era, endpoint, packet-length/extension and freshness outcomes; authentication not applicable/not run/pass/fail/unknown, with stage and bounded reason. |
| NTS-KE evidence | Expected verified identity, actual negotiated protocol/AEAD and endpoint binding, full-authentication event ID/age, certificate/trust/pin-policy generations, validation-time basis and bootstrap ancestry. Public cert/key identifiers can replace full chains in ordinary history. |
| Protected NTP evidence | Association generation, admitted AEAD/authentication result, source/endpoint binding, key-age/full-authentication-age status and final eligible/rejected reason. Cookies, traffic keys, session tickets and exporters are excluded. |
| Time-dependent verification | Which chain/name/date/revocation/pin checks ran, their policy/results, evaluation time basis and limitations. A successful API call is not necessarily a successful policy verdict. |

**Raw/derived and lifetime:** checked domain samples are created by current verifiers, not by loading an “authenticated” boolean. Historical summaries can explain what happened but cannot rebuild live authenticated authority. Invalid MACs and malformed packets are unauthenticated evidence about the exchange/path; they do not establish misconduct by the named operator. Keep OPEN-8's Windows candidate-date and revocation limitations visible. Sources: architecture §§7–8; time-assurance §9; [RFC 8915](https://www.rfc-editor.org/rfc/rfc8915.html); [CertGetCertificateChain](https://learn.microsoft.com/en-us/windows/win32/api/wincrypt/nf-wincrypt-certgetcertificatechain).

### DATA-13 — Roughtime and reserve bootstrap evidence

**Class:** Conditional future profile. **Producer:** qualified Roughtime or reserve procedure. **Consumers:** single bootstrap evidence-policy owner and isolated NTS verification; never a new direct clock writer.

| Field group | Meaning and presence |
| --- | --- |
| Roughtime result | Attempt/source/key/profile generations, supported wire version, nonce/request-binding and signature/delegation verification outcomes, authenticated midpoint/radius or normalized interval, units, measured elapsed time and aging assumptions. |
| Reserve result | Candidate date/era interval, population/catalogue revision, admitted and actually sampled source sets, sampling procedure/RNG instance or round reference, known dependency groups, policy filtering and resulting limitations. Do not retain live RNG secrets. |
| Branch lifecycle | Why preferred bootstrap ran or did not run; unavailable/unusable/conflicting/stale/date-incompatible status; reserve eligibility reason, bounded attempts and final evidence outcome. Healthy strict NTS is not dependent on a new Roughtime round. |
| Applicability | Observation IDs, common evaluation instant, supported time-scale/era interpretation, evidence lifetime, policy generations and original ancestry. Future full NTS must retain the date-hypothesis ancestry and revalidation result. |

**Sensitivity/lifetime:** operational evidence, with persistent conflict handled by DATA-18. Ordinary records retain verification summaries; complete protocol proofs are conditional evidence, not universal packet history. Roughtime deliberately permits invalid “greased” replies; an invalid signature cannot identify a particular packet as deliberate grease or establish operator dishonesty. Authenticated disagreement can show a conflict without identifying which participant is wrong. Keep compatible-time and common-reference assumptions explicit. Sources: time-assurance §§4–7; [RFC 10049 §§7–8](https://www.rfc-editor.org/rfc/rfc10049.html#section-7); [RFC 9523](https://www.rfc-editor.org/rfc/rfc9523.html).

### DATA-14 — Reference, selection and acknowledged correction provenance

**Class:** Required for interpreting residuals and security decisions; detailed retained snapshots proposed. **Producer:** pure selection/core and broker for actual effects. **Consumers:** correction guards, estimator, quality evaluator and status.

| Field group | Meaning and presence |
| --- | --- |
| Selection snapshot | Snapshot/round ID, configured minimum, eligible/rejected sets and reasons, selected principal/supporters, independent group count/unknowns, common instant/frame, uncertainty and policy/evaluator versions. |
| Comparison reference | Reference estimate/interval ID, contributing observation IDs, dependency groups, time scale and bootstrap ancestry; whether the assessed source also influenced its comparison reference. |
| Decision meaning | Agrees/contradicts/insufficient evidence/stale/not assessed with precise interval/comparison rule and applicability. A broad overlapping interval need not provide useful discrimination. |
| Correction link | Proposal ID versus actual broker operation/outcome ID, actual mode/rate/phase, applied instant/quantization and before/after clock generation as needed by the qualified transformation. An uncertain outcome is not a confirmed correction. |
| Explanation | Bounded exclusion/admission reason, reference independence limitations and any qualified rebase reference. Avoid retaining full duplicated sample sets when stable IDs suffice. |

**Raw/derived and lifetime:** selection and residuals are derived; dependencies must survive as interpretable references or their limitations must be explicit. The requested correction cannot substitute for the actual acknowledged effect when adjusting old offsets. A reference that includes the same source can bias its apparent quality; independent comparison or explicit dependence is required before treating a residual as evidence against that source. Broker journal retention remains DATA-21's concern. Sources: architecture §§6 and 10; time-assurance §5.

### DATA-15 — Active estimator state, bounded history and restart priors

**Class:** Existing engine/core state; only a bounded validated drift prior is the initial persistent warm-start direction. **Producer:** estimator and lifecycle owner. **Consumers:** current estimator; optional restoration loader through existing validity checks. **Sensitivity:** operational model data.

| Field group | Meaning and presence |
| --- | --- |
| Current source model | Bounded admitted sample window, current offset/frequency/skew, covariance or sufficient statistics when the chosen model needs them, age/uncertainty and source/clock generation bindings. |
| System model | Current aggregate estimate, oscillator/drift prior, model/version, units, confidence/validity conditions, source provenance and last independent measurement time. |
| Restart binding | Saved schema/model version, actual capture event, applicable boot/hardware/VM/power/clock context, elapsed-age basis, uncertainty growth and compatibility/invalidation result. |
| Restoration outcome | Accepted bounded prior/discarded/incompatible/continuity unknown with reason. Restored prior is distinguishable from fresh acquired measurements. |

**Raw/derived and lifetime:** this is derived working state, not a second server-reputation store. Full historical sample restoration is not adopted; same boot alone is insufficient continuity. Reboot/Fast Startup/suspend, external time changes, hardware/VM changes and policy changes require the existing applicability rule. Corrupt optional priors are discarded and acquisition restarts; they neither restore current synchronization nor replenish correction authority. Source: architecture §12.1.1.

### DATA-16 — Quality vectors, coverage and mergeable summaries

**Class:** Requested quality history; exact model/retention is proposed. **Producer:** bounded aggregation from DATA-10–14. **Consumers:** policy-constrained operational preference/cooldown, diagnostics and UI. **Sensitivity:** local behavioral/network metadata.

| Field group | Meaning and presence |
| --- | --- |
| Grouping key | Source/endpoint, protocol/version, relevant network/context and time-scale/reference compatibility, catalogue/source generation and evaluator version. Operator roll-ups preserve endpoint/path coverage. |
| Coverage | Observed interval or monotonic segment, attempts and stage denominators, valid/authenticated/eligible counts, known gaps, missing reasons, observation density and last usable evidence age. No observations means unknown, not perfect availability. |
| Quality dimensions | Reachability under observed conditions, response latency/delay, admissible measurement yield, residual stability, uncertainty, authentication/protocol failure categories and selected/held-out coverage. Keep dimensions separate from security admission. |
| Mergeable numeric data | For each compatible metric: count, mean, M2 or another specified mergeable dispersion state, min/max and invalid/missing counts. Histogram/sketch/quantiles are optional and need algorithm, resolution, error and bounded-size metadata. |
| Derived preferences | If used: score/rank, inputs/weights/model revision, effective sample size, confidence/coverage limitations, reason and evaluation instant. This is not a calibrated probability of server honesty. |

**Raw/derived and lifetime:** raw attempts are inputs; aggregates, intervals, scores and UI percentages are derived. Never average percentages/percentiles or merge overlapping hourly/daily windows as independent data. EWMA-like state needs defined ordering/decay and elapsed-time basis. Compare only compatible epochs/references or retain the segmentation. A long selected-only history is biased by selection and missing probes. No probability of compromise or expected vulnerability rate can be computed from these metrics alone.

### DATA-17 — Contextual source state, cooldown, reprobe and transitions

**Class:** Proposed operational policy; mandatory security state referenced separately. **Producer:** qualified state machine from current admissible evidence. **Consumers:** scheduler and source preference, under DATA-06.

| Field group | Meaning and presence |
| --- | --- |
| State identity | Source/endpoint/context scope, state-machine version/generation, operational state such as unassessed/eligible/degraded/cooling down, and separate administrator/security restriction. Final vocabulary remains open. |
| Transition | Old/new state, bounded reason, triggering evidence/aggregate IDs, threshold-policy revision, observation coverage and authority class. |
| Timing | Qualified monotonic/elapsed basis, transition instant, cooldown/retry deadline, next reprobe and backoff state. Reboot or unknown continuity invokes a conservative defined rule. |
| Recovery | Hysteresis/probation criteria, required successful fresh evidence, retry budget and reason for continued exclusion. Excluded sources need controlled remeasurement if automatic recovery is intended. |

**Sensitivity/lifetime:** local operational state. Path-specific degradation should not automatically become a global operator ban; an adopted conflict policy may require DATA-18/21's stronger durable rule. Defaults for never-seen and insufficiently sampled sources must avoid both permanent starvation and unjustified trust. Deleting quality history may reset disposable preference but cannot clear explicit revocation, pins or a quarantine required by the adopted security policy. Reputation pruning/reweighting of a Khronos random population changes the sampling assumptions and needs separate analysis before use. Sources: time-assurance §5.1 and §7; [RFC 9523](https://www.rfc-editor.org/rfc/rfc9523.html).

### DATA-18 — Sparse evidence and unresolved security conflicts

**Class:** Optional diagnostic detail plus conditional safety state for a separately frozen conflict/quarantine rule. The time-assurance design requires conflict preservation across branch changes and timeouts; its restart/persistence and clearance contract remains TA-W02 work. **Producer:** verifying/evidence-policy owner. **Consumers:** that owner, authorized investigation and bounded redacted status. **Sensitivity:** security-sensitive and potentially private.

| Field group | Meaning and presence |
| --- | --- |
| Incident identity | Incident/evidence ID, severity/category, first/last observation sequence and qualified time, source/key/context/generation references, current unresolved/resolved state and resolution authority. |
| Verification evidence | Minimal exact signed material, request/nonce binding, delegation/chain/proof components and comparison assumptions needed to independently validate a retained conflict when the protocol permits it. Public signatures are not traffic keys. |
| Other diagnostics | Bounded sampled failure-stage data, reproducibility/version metadata and explicit truncation/redaction marker. Do not retain arbitrary full peer packets, certificates or error dumps simply because an attempt failed. |
| Causality and attribution | Authenticated versus unauthenticated material, participating identities/dependencies, compatible evaluation frame and known limits on assigning fault. |
| Retention/clearance | Optional-detail expiry; whether the selected conflict policy requires protected cross-restart state and its minimum content; safe reacquisition/refusal when required evidence is missing or cannot be recorded; authorized resolution/supersession and branch/timeout continuity. Exact persistence and clearance rules remain open. |

**Raw/derived and lifetime:** signatures/proofs can be raw evidence; “conflict” is a derived conclusion with premises. A branch change or timeout cannot erase a recorded authenticated conflict. Optional-history pruning/reset cannot supply the safety owner's required resolution. If the adopted profile requires cross-restart conflict/quarantine state, retain its minimal safety record under that same policy owner separately from bulky optional diagnostics; do not reconstruct authority from quality history. TA-W02 and OPEN-DATA-05 must still freeze persistence, clearance and missing-state behavior. This draft adopts no automatic global server ban or permanent quarantine. No default upload or public accusation is authorized. Roughtime proof collection may require bounded sequential linking; parallel ordinary probes are not automatically a cryptographic inconsistency proof. Sources: time-assurance §§5–7; [RFC 10049 §8](https://www.rfc-editor.org/rfc/rfc10049.html#section-8).

### DATA-19 — History/export manifests, dictionaries, migrations and gaps

**Class:** Conditional on persisted/exported detailed history or summaries. **Producer:** authorized storage/export owner. **Consumers:** that loader/aggregator and explicit analysis/support import; not live source authentication.

| Field group | Meaning and presence |
| --- | --- |
| Segment/snapshot identity | Unique artifact/segment ID, originating installation/run scope where needed, schema and producer/evaluator versions, exact record/byte count and optional verified digest. A colocated writable digest alone does not authenticate origin. |
| Coverage and deduplication | Source/context dictionaries, included sequence/attempt ranges, first/last qualified times, explicit gaps, retention/truncation markers and superseded segment IDs. |
| Processing checkpoint | Last durably accepted event/watermark, aggregate generation, export/import completion state and idempotency identity. Different physical stores are not one atomic transaction merely because their records share an ID. |
| Interpretation | Units/sign/time-scale conventions, dictionary/catalogue/policy references and sufficient labels/tombstones for records to remain meaningful after catalogue pruning. Referenced absent evidence is marked unavailable. |
| Migration | Source/target schema and model versions, migration identity/outcome, compatibility/rebuild rule, incomplete-operation marker and bounded failure recovery. Recomputed scores retain both new model identity and original observation provenance. |

**Sensitivity/lifetime:** manifests can expose server/network usage even when payloads are encrypted. A crash, partial write, duplicate import or lost manifest must not double-apply observations or fabricate continuity. Optional descriptive history may be dropped/rebuilt under its policy; mandatory security state may not use that recovery shortcut. Exports are descriptive, never a route to restore current auth flags, trusted floors or correction budgets.

### DATA-20 — Retention policy, resource accounting and collection health

**Class:** Required boundedness; exact budgets are open. **Producer:** authorized configuration and runtime/storage accounting owners. **Consumers:** scheduler, storage admission and diagnostics. **Sensitivity:** local resource telemetry.

| Field group | Meaning and presence |
| --- | --- |
| Scope and limits | Per-class time/count/byte budgets, aggregate installation budget, context/source cardinality, work queues, active records and optional collection mode. Distinguish installed catalogue, current model, statistics, evidence, secrets, logs and export scratch. |
| Actual usage | Logical retained records versus physical allocation, in-flight buffers/jobs, compressed/decoded/workspace peak where applicable, and total owned file-family/temp/compaction/export usage where applicable. |
| Loss/pressure events | Queue drops, rejected writes, disk-full/permission errors, checkpoints/cleanup deferred, quota exhaustion and affected sequence ranges. Preserve incomplete measurement coverage. |
| Lifecycle actions | Pruning/reset/migration/export request, authorization where needed, affected optional classes, completion/uncertainty and retained mandatory-state references. |

**Raw/derived and lifetime:** usage counters are derived accounting, not provider quality. Retention cannot depend exclusively on an untrusted system wall clock; use qualified age/order and hard capacity limits. The existing 16 active-source limit does not bound a catalogue or unlimited historical contexts by itself. Optional telemetry backpressure must not starve acquisition, cancellation or mandatory security writes. Existing diagnostic retention remains separate; hypothetical statistics/archive limits are not adopted.

### DATA-21 — Mandatory clock/security state: references to existing owners

**Class:** Existing mandatory state, not a new statistics feature. **Producer:** the architecture's broker/trust-policy owners and authorized installer/helper paths. **Consumers:** only the named current authority checks. **Sensitivity:** critical integrity/availability state.

| State family | Information that must remain distinct from quality history |
| --- | --- |
| Trusted bounds/floor | Evidence and uncertainty assumptions, supported time/era interpretation, source/trust-policy lineage, supersession and compatibility status; architecture §8.1. |
| Correction journal | Reservation/operation identity, requested and actual/uncertain effect, cumulative consumed authority, outcome ordering, boot/clock context and remaining permitted budget; architecture §10.2.1. |
| Mandatory source/security policy | Current policy revision, withdrawals, pins if later adopted, and any conflict/quarantine state required by the applicable frozen policy, with authorized resolution; DATA-06/07/18 reference the owning contract. |
| Lifecycle protection | Installation/upgrade/service-recreation migration ownership, recovery/rollback limitations and protected durable-state validity. Optional per-service lifetime must not silently delete these obligations. |
| Product and clock ownership | Application/package/schema identity, installation witness and lifecycle transaction identity, service/clock-controller ownership and the exact prior W32Time state needed for the existing reversible handover/restoration contract. These are owned lifecycle records, not unrestricted system-configuration snapshots or a new UI feature. |

**Lifetime and recovery:** clearing statistics, exporting history, reinstalling an optional cache or restoring a drift prior cannot clear consumed authority or security restrictions. Corruption and uncertain effects use the existing fail-closed rules rather than resetting counters. A MAC, signature or protected key does not prove evidence truth or prevent an old valid disk/VM snapshot from being restored. Keep any hardware anti-rollback decision separate. Sources: architecture §§8.1, 10.2.1 and 12.1; [protected-storage task](../../tasks/windows-protected-storage.md).

### DATA-22 — NTS association secrets and optional protected-storage metadata

**Class:** Existing secret owner; persistence conditional on separately qualified storage. **Producer/consumer:** engine NTS association owner and admitted secret-storage provider only. **Sensitivity:** secret; excluded from ordinary catalogue, statistics, logs, UI and support bundles.

| Field group | Meaning and presence |
| --- | --- |
| Secret association payload | Client-to-server/server-to-client traffic keys, cookie material, applicable TLS/session/exporter secrets and current secret lifecycle state as actually required in working memory. The optional restart payload is a separate bounded allowlist; TLS/session-resumption persistence is not adopted. No speculative retention of every derivable secret. |
| Binding metadata | Source/expected identity, negotiated endpoint, algorithms, association/key generation, separate cookie issuance/receipt ages, key age, last full certificate-authentication event/age and cumulative resumption age with qualified elapsed bases, trust/pin policy revision and bootstrap provenance. Unknown ages remain explicit. |
| Storage envelope | If persistence is enabled: domain/schema, installation/store/purpose, root epoch, association/data epoch, privacy/policy generations and snapshot sequence; provider/key, wrapped-DEK, nonce/ciphertext/tag/algorithm and independent authentication metadata as required by the admitted construction; migration/version and actual completion outcome. |
| Store authority | Existing installation/store/purpose identity, provisioning/recovery witness, active/prepared/retired root-key catalogue and policy/privacy generation. Explicit no-persistent-cache state differs from unexplained loss of an expected key/catalogue. |
| Committed save | Under the existing protected-storage contract, the root/key catalogue binds the selected root/data/privacy identity and snapshot sequence to the digest of the exact committed envelope, or to explicit initialized empty state. An independently admitted verification identity cannot be replaced by one supplied in the envelope. Initialized empty state with admitted roots differs from NoPersistentCache with no expected root or secret snapshot. |
| Failure/lifecycle | Key unavailable, restore invalid, expired/unknown age, stale policy, partial/indeterminate write and cleanup result; reacquisition and explicit disabled-cache state. Never persist a half-valid association. |

**Lifetime and recovery:** secret retention is minimized and separately bounded; historical cookie/key copies are not a reliability dataset. Cached secrets do not make measurements fresh, and cookie renewal, protected UDP traffic or resumed TLS cannot renew the original full-authentication age. Restoring an association never restores an outstanding exchange or reuses its request identity; new queries require fresh protocol correlation. Exact lifetime limits remain OPEN-8 work. The current preferred optional-protection research is VBS, then separately qualified TPM, then no persistent cache where allowed; no automatic DPAPI/software/plaintext fallback is adopted. Storage protection, rollback resistance and whole-process memory protection are different claims. Sources: architecture §§7.4 and 12.1.2; [protected-storage plan](windows-protected-storage.md).

The protected-storage plan separately covers future DNS demand/history snapshots, their authentication, independently admitted keys, privacy generations and forget/retirement barriers. Calling those values “statistics” does not permit moving them into an ordinary time-source quality store, sharing their keys, or dropping their existing controls. DATA-16 proposes only nonsecret time-source quality history; it does not replace the DNS store's contract.

### DATA-23 — Logs, UI/status projections and authorized support exports

**Class:** Existing diagnostics/status ownership plus conditional history views. **Producer:** bounded component sinks and coherent status publisher. **Consumers:** authorized users/support tools through narrow projections, not direct access to service databases/secrets.

| Field group | Meaning and presence |
| --- | --- |
| Current status | Snapshot generation/age, active sources, authentication/agreement/uncertainty, actual bootstrap route, collection/coverage state, next action and bounded last-failure code. Reopening UI does not refresh measurement age. |
| History view | Paged bounded summaries by source/context/window with units, missing-data markers, evaluator version and coverage; distinguish configured, observed and verified properties. |
| Logs | Stable event codes, component/generation/operation references, bounded redacted fields and authorized temporary DEBUG expiry. No arbitrary packet/IPC/debug-object or secret dumps. |
| Support export | Explicit scope/consent, data classes included/excluded, redaction policy/version, creation time basis, format/schema, provenance and omission/truncation markers; no background upload implied. |
| User presentation | Per-user display/filter preferences and last-view state where useful. These cannot change source admission, pins, clock policy or collection authority. |

**Sensitivity/lifetime:** local source names, server IPs, timing and network context can reveal usage and enterprise infrastructure. Pseudonymized IDs can remain linkable. Logs remain within the architecture's aggregate 10 MiB diagnostic budget; a new historical view does not silently enlarge it or make logs authoritative. Support data must not contain NTS/storage/release private keys or unrestricted database copies. Source: architecture §§6, 12.1 and 12.3.

### DATA-24 — Publisher-only signing material and release evidence

**Class:** Required release-process ownership; excluded from all client data. **Producer/consumer:** authorized publisher/signing/release infrastructure. **Sensitivity:** private signing material is secret; reproducibility/approval records may be public or restricted.

| Field group | Meaning and presence |
| --- | --- |
| Private authority | Release/catalogue signing private keys, service credentials, recovery material and key-custody/access policy. None is bundled in MSIX/MSI, placed in client RAM for catalogue verification, or saved in source history. |
| Build/review evidence | Exact input revisions and approved normalized-snapshot identity, adapter/generator/schema identity, reviewed catalogue diff, operator-permission review, release approval and artifact digest/signature identity. The future Python collector/build tool belongs to R under existing `tools/`; it builds the exact reviewed snapshot, and changed parsing/normalization/schema inputs invalidate that data approval. Candidate outputs remain separate from approved release inputs. |
| Qualification evidence | Decoder/resource/negative-test results, actual package/signature/content-integrity verification and provider/key-rotation evidence when implemented, bound to the candidate artifact and environment. Planned checks are not completed qualification. |
| Recovery/change record | Authorized key rotation/withdrawal and publication lineage; public counterparts go into the relevant trusted release metadata, not an unauthenticated downloadable replacement. |

**Lifetime:** publisher retention follows release/key governance and provenance needs; it is not a client telemetry retention choice. Client catalogue verification requires only the admitted public trust material or trusted expected digest. Microsoft package signing does not require supplying NevroTick publisher secrets to the installed application and does not authenticate time-server behavior.

## 4. Interpretation constraints for the later storage design

The allocation review must preserve the distinctions above instead of collapsing everything into one database or one “trust score.”

| Tempting inference | Required interpretation |
| --- | --- |
| “This server had a large offset.” | It differed from a particular local clock/reference under a particular uncertainty and frame. A wrong local clock can explain the offset. |
| “Many packets failed authentication.” | Controlled exchange outcomes show invalid/unmatched material or path interference; they do not authenticate the accused server as its sender. Roughtime greasing adds a protocol-specific reason not to infer dishonesty from one invalid response. |
| “Roughtime, NTS and the reserve agree.” | Trace independent source groups and original bootstrap ancestry. Three consumers of one premise are not three independent witnesses. |
| “The server has a 99% score.” | State denominator, observation conditions, coverage, model and uncertainty. This is not a universal honesty, compromise or future-availability probability. |
| “The history file is authentic.” | It may still be stale, semantically wrong, incomplete or replayed. Current policy and authority checks remain necessary. |
| “The database passed its integrity check.” | Structural validity does not establish approved origin or truthful observations. A checksum next to a writable payload does not create a trust boundary. |
| “Clear all history to repair it.” | Only disposable classes may be reset. Keep source/pin policy, withdrawal/conflict requirements, trusted bounds and consumed correction authority under their existing recovery rules. |
| “Only fast/reliable Pool servers will be sampled.” | Selection changes can invalidate the reserve's random-population assumptions. Qualify the changed model before claiming its protections. |

## 5. Proposed storage allocation

**Recommendation for discussion:** keep current work in compact uncompressed RAM, use a small transactional store for advisory quality summaries, and add detailed cold history only for a demonstrated product or investigation need. Retain immutable release data, mandatory authority, optional secrets and publisher records under their respective owners. This allocation selects no dependency, serialized contract, protection provider, default or numerical limit.

### 5.1 Locations, owners and complete allocation matrix

The location names below are conceptual; existing contracts do not imply implemented stores. Example filenames are not an approved Windows namespace, owner/DACL, package manifest or migration contract. The existing installer/service owners must provision and qualify actual locations; do not infer trust from a hard-coded WindowsApps path.

| Location | Proposed representation and protection boundary |
| --- | --- |
| **C — Release catalogue** | Prefer binary delivery. If a separate package resource is chosen, constrained CBOR is the recommended comparison candidate; generated immutable tables in the installed image remain a leading release-bound alternative. Exact representation remains open under the [binary refinement](server-catalogue-binary-format.md), with release binding and admission from §5.5. JSON may serve publisher inputs or reference comparisons. No client mutation of the packaged default. |
| **Q — Quality store** | Candidate `quality.sqlite` in a fixed installer-provisioned engine data directory; one engine writer, bounded typed records and service-specific file protection. Advisory data only. |
| **H — Optional detail** | Engine-owned bounded completed segments, or compressed BLOB batches in Q; sparse evidence and explicit archives under §§5.3–5.4. “Sealed” means complete and immutable after publication, not automatically encrypted or cryptographically authenticated. |
| **E — Existing stores** | Current policy, journal, trusted-floor, drift-prior and secret-storage owners, each retaining its own schema, protection and failure rules. This label does not combine them into one database. |
| **U — User presentation/output** | Per-user preferences, bounded status projections and explicitly saved redacted exports; no direct service-database access. |
| **R — Publisher records** | Release/curation evidence and separately protected signing custody outside the client. Client quotas and history-reset commands do not govern R. |

All Q/H rows inherit the access, resource and crash rules in §§5.3–5.6. Their lifetime is limited by both the chosen retention policy and physical capacity. Existing-owner rows retain their canonical lifecycle rather than adopting Q's disposable recovery.

| Data and authoritative owner | Hot RAM representation | Durable allocation, if any | Retention, invalidation and restart |
| --- | --- | --- | --- |
| **DATA-01 — Maintainers/release reviewer** | No client research corpus; admitted provenance references only. | R: versioned source evidence, permissions and curation decisions; C: bounded public references. | Publisher retention explains each release. Re-review changed evidence; retain supersession. Private correspondence stays outside C/Q/H. |
| **DATA-02 — Release owner; engine loader** | Bounded captured artifact, then immutable validated header/index. | C: catalogue and release binding; Q/H: referenced revision identity only. | Authenticate each admitted artifact/version. Retain enough metadata to interpret surviving history; stale/rollback behavior remains OPEN-CAT-03. No “newest file” selection. |
| **DATA-03 — Curation; authorized policy owner** | Interned source/operator/dependency records and bounded indexes. | C/E: authoritative definitions; Q: minimal versioned dictionaries/tombstones for retained history. | Changes segment affected aggregates. Delete unused historical dictionary entries only after references expire; Q never becomes current independence policy. |
| **DATA-04 — Configuration/association owner** | Immutable admitted endpoint records; current connection bindings separately. | C/E: endpoint declarations; Q/H: necessary version/address references. | Source/endpoint changes invalidate affected work. On restart re-admit destinations; historical addresses cannot authorize new connections. |
| **DATA-05 — Roughtime key-policy owner** | Read-only admitted public keys/profile tables. | C/E: public trust/profile records; Q/H: referenced key/version identity. | Withdrawal/rotation follows current policy even if history is cleared. Never restore root authority from a historical response or dictionary. |
| **DATA-06 — Broker/authorized helper** | Validated current policy snapshot; narrow engine projection. | E: existing protected versioned policy contract, including authorized local overrides. | Preserve revision/CAS, durability and migration. Invalid mandatory policy does not reset to catalogue defaults. Q contains explanations/references only. |
| **DATA-07 — Future NTS trust-policy/verifier owner** | Scoped immutable pinsets and current verification result. | E/C if adopted: authenticated pin policy; Q: bounded matched-ID/result summaries. | Pin changes affect association/resumption generations. Historical success supplies no current pass; recovery/rotation needs §5.7's contract. |
| **DATA-08 — Resolver/network/association owner** | Bounded resolution results and opaque context IDs. | Q: only needed context/remote-address dictionaries; H: conditional detailed bindings. No duplicate resolver cache. | Expiry and network changes invalidate live bindings. Cap retired contexts; re-resolve/re-admit after unknown continuity and preserve historical labels only while referenced. |
| **DATA-09 — Event/generation owners** | Typed instants, finite counters and current generation references. | Q/H: per-record or batch bindings; E retains authoritative effect identities. | Retain with dependent records. A new boot/clock mapping cannot reuse old live coordinates; unknown age stays unknown after restore or migration. |
| **DATA-10 — Scheduler/association owner** | Fixed pending slots, terminal counters and bounded handoff queue. | Q: stage denominators/outcomes and sparse transitions; H: optional attempt detail. | Request UIDs/pending exchanges are transient and never restored. Retire request eligibility according to the protocol's completion, timeout, cancellation and invalidation rules; retain native-operation permits, handles and buffers until actual terminal completion. Crashes/dropped handoffs leave explicit incomplete coverage. |
| **DATA-11 — Protocol/core owner** | Checked samples and fixed recent rings; distinct metrics/units. | Q: compatible numeric summaries; H: selected detailed measurements when justified. | Evict old detail after required computation/aggregation. History is descriptive; it cannot repopulate the live estimator as fresh samples. |
| **DATA-12 — Protocol/NTS verifier** | Current checked admission evidence and association references. | Q: bounded verification/eligibility summaries; H: exceptional evidence, without secrets. | Results bind to their original policy/time basis. Loading a recorded pass never constructs live authenticated evidence. Expire detail under its privacy/evidence policy. |
| **DATA-13 — Bootstrap evidence-policy owner** | Bounded current intervals, ancestry and branch state. | Q: branch/outcome summaries; H: minimal retained proof if selected; C/E retain keys/policy. | Preserve cross-branch conflicts. Restart safety content and clearance remain TA-W02 work; archive expiry cannot resolve a conflict. |
| **DATA-14 — Core/selection; broker for actual effects** | Coherent current selection/reference snapshot and acknowledged-effect links. | Q: sparse decision/transition references; E: authoritative broker outcome. | Keep provenance while derived residuals remain interpretable, or mark it unavailable. Never use Q to settle an uncertain correction or restore consumed authority. |
| **DATA-15 — Engine/core and prior loader** | Existing bounded model/sample window; no compressed hot history. | E: bounded drift-prior record only after qualification under the existing contract. | Restore through §12.1.1 validity checks; discard an invalid optional prior. Full-history restoration is unadopted, including from H or Q. |
| **DATA-16 — Engine quality aggregator** | Bounded counters and compatible current accumulators. | Q: sparse hourly/daily summaries, coverage and evaluator-versioned vectors. | Roll up disjoint compatible populations; prune within limits. Corruption/loss yields unavailable history or a fresh descriptive generation, never improved trust. |
| **DATA-17 — Policy-constrained scheduler** | Current contextual state, hysteresis and qualified elapsed deadlines. | Q: optional restart hints and transition history; E: separate mandatory restrictions. | Revalidate hints/current policy. Unknown deadline continuity uses an explicit conservative rule; clearing preference cannot clear withdrawals or any adopted safety restriction. |
| **DATA-18 — Evidence-policy owner** | Bounded unresolved-conflict state and minimal current evidence. | H/Q: optional proof detail/index. Any required cross-restart safety minimum belongs to E only after TA-W02 freezes it. | No new mandatory quarantine store is adopted. Preserve current branch/timeout conflict rules; optional pruning supplies no clearance. Keep proof loss and resolution distinct. |
| **DATA-19 — Storage/export owner** | Bounded publication/migration job and manifest cache. | Q: manifests, dictionaries, idempotency/checkpoint records; H: complete segment metadata. | Retain with referenced data. Reconcile partial publication by exact IDs/bytes; reject incompatible readers and expose missing detail rather than double-apply it. |
| **DATA-20 — Configuration/resource owners** | Fixed usage counters, queue limits and pressure state. | E: authorized limits where applicable; Q: bounded watermarks/health/gap records. | Recount admitted physical objects at recovery within a finite budget. Unknown usage stops new optional growth; counters never become provider-quality failures. |
| **DATA-21 — Existing broker/trust/lifecycle owners** | Current validated authority snapshots and operation references. | E: separate protected policy, journals, trusted bounds and restoration records. | Existing fail-closed reconciliation/migration controls apply through update, reset and removal. Never reconstruct these records from Q/H or place their sole copy in disposable service state. |
| **DATA-22 — Engine secret-storage owner** | Private bounded NTS secrets and working buffers; minimized lifetime. | E only: separately qualified optional authenticated snapshots, root/key catalogue and witness, or explicit no-persistent-cache state. | Preserve exact committed envelope, root/data/privacy epochs and full-authentication ages. No pending exchange restore; no secret copies in C/Q/H/U. |
| **DATA-23 — Component/status owner; user for output** | Coherent bounded projections, event queues and paged history views. | Existing bounded logs; U: preferences and explicit redacted exports. | Enforce independent log/export caps and DEBUG expiry. Stale snapshots stay labelled historical; exports neither restore authority nor grant privileged write paths. |
| **DATA-24 — Publisher/signing owner** | No publisher private material in client RAM. | R: protected key custody plus exact release/review/approval evidence; client receives public verification material only. | Publisher governance controls rotation, recovery and evidence retention. Client reinstall/reset cannot govern signing authority; no Microsoft private key is needed by the client. |

### 5.2 Hot RAM and useful aggregate state

**Do not compress hot RAM by default.** Prefer compact typed records, interned bounded strings/IDs, bitsets/enums for finite states and fixed-capacity rings. Keep timestamp capture, sample admission and clock-critical work free of compression, database calls and expensive formatting. A bounded handoff can feed the same engine's storage owner; no new daemon or runtime selection follows. Track queue loss without blocking acquisition or mandatory cleanup.

Compression is worth reconsidering only after measured benefit includes compressed and decoded copies, indexes, allocator overhead, codec workspace, peak concurrency and latency/CPU cost. It does not encrypt secrets. No OS-wide compression, paging or dump-policy change is proposed. Illustratively, `16 × 128 × 192 bytes = 393,216 bytes = 384 KiB` for record payloads alone. These are hypothetical sizes, not measurements or a new history limit: the architecture still caps the core history at **64 samples per source** and active logical sources at **16**. Other rings and helpers need separately counted capacity.

For DATA-16, keep attempt/stage denominators separate from each numeric metric's valid-observation count. A candidate accumulator retains `n`, mean, M2, min and max for a declared compatible population; `n = 0` has no mean or extrema. Define `M2 = sum((x - mean)^2)`: summed squared deviations, not the variance itself. Sample variance is `M2/(n-1)` only for `n > 1` and the declared statistical model; it is not a clock-error bound. Merging two disjoint compatible populations needs their counts and means plus the between-mean contribution to M2, not an average of variances. Missing/rejected counts remain separate.

Compatibility includes units, clock/reference frame, source/path context, time scale, evaluator and observation population. Do not combine the same samples' hourly and daily summaries, average percentiles, or reinterpret an old model's score under new weights. Mark unavailable statistics explicitly. Any future sketch/quantile implementation needs bounded state and a declared error model.

### 5.3 Small mutable summaries; optional detailed history

**SQLite is a proposal for Q**, because the mutable dictionaries, aggregate/checkpoint updates, sparse transition events and bounded queries benefit from one transaction owner. Prefer sparse buckets only for observed populations. Persisting a row for every poll is unnecessary by default if aggregate counters and selected events answer the product questions. Database dependencies, schema, journal mode and durability settings still need admission; WAL is an option, not a decision. [SQLite use cases](https://www.sqlite.org/whentouse.html)

Ordinary SQLite has no built-in general page-compression switch. SQLite's [ZIPVFS](https://sqlite.org/zipvfs/doc/trunk/www/index.wiki) is a separate compressed-database extension, not an adopted feature here. Start by reducing repetition, indexes, cardinality and unnecessary detail; aggregation and bounds may save more than compressing everything. If detailed retention is justified, compare:

| H candidate | Benefit and cost to evaluate |
| --- | --- |
| **Completed Parquet with internal Zstd** | Column-oriented analysis and selective reads; use Parquet's internal page compression, not an outer `.parquet.zst` wrapper. Needs bounded row groups, dictionaries, footer metadata and codec workspace; small-file overhead may dominate. No incomplete active file is published as a finished archive. |
| **Compressed JSONL or CBOR batches** | Simpler bounded sequential batches; typed units/schema still required. Seeking/filtering may require more decoding or a separate bounded index. Define complete-batch framing and recovery; compression supplies no authenticity. |
| **Compressed BLOB batches inside SQLite** | Can commit a batch and its metadata within Q's transaction; avoids a separate-file publication step for that batch. Fine-grained querying and correction become harder, and decoded batches/recompression/WAL still consume resources. |

[Parquet's compression specification](https://parquet.apache.org/docs/file-format/data-pages/compression/) includes Zstd; its [error-recovery notes](https://parquet.apache.org/docs/file-format/data-pages/errorrecovery/) explain why incomplete/corrupt file metadata matters. Select one primary raw representation, with only bounded temporary duplication during migration/export. Do not permanently retain the same raw stream in SQLite, JSONL and Parquet.

Dropping raw measurements prevents exact later replay, new unanticipated metrics and some forensic attribution. Rollups cannot recover sample order, tails, missing timestamps or cryptographic proofs. Preserve coverage/model identity and specifically justified sparse evidence; otherwise say that replay or proof is unavailable. Keeping every packet “just in case” adds privacy and resource cost without making unauthenticated material trustworthy.

### 5.4 Retention, physical bounds and interrupted work

The following are **illustrative, unmeasured and unadopted** sizing examples: seven days of hourly and 180 days of daily aggregates; a **32 MiB complete Q family**, including reserved journal/temp/maintenance capacity; optional detailed history capped at seven qualified days **or 64 MiB**, whichever is reached first. They are not additions to the architecture's separate **10 MiB aggregate diagnostic-log budget**. Freeze actual useful windows and total installation/RAM headroom from measured workloads before implementation.

Bound every physical family and work phase: main database, indexes, WAL/shared-memory or rollback journals, temporary sorting/output, checkpoints, old/new migration copies, backup/export staging, archive segments/footers, sparse proofs and orphan-cleanup slots. Also bound files, sources/retired contexts, dictionary cardinality, open readers/connections, transactions, queues, compressed/decompressed bytes, codec workspace and concurrent jobs. An active-source cap alone bounds none of the historical multiplicities. Reserve maintenance and mandatory-state headroom before admitting optional growth; a failing mandatory write still follows its own refusal rule.

`max_page_count` limits the database file; `journal_size_limit` concerns journal/WAL size left after the relevant commit/reset, not all live storage. Long readers can delay WAL checkpoint progress. Ordinary DELETE may leave reusable free pages without shrinking the file, and VACUUM can require substantial additional space—documented as up to twice the original size in free space. These settings are not an installation disk quota. Bound transactions/read lifetimes, account actual families and refuse optional work before its reserved ceiling; defer compaction when headroom is insufficient. Qualify the actual chosen build/VFS and crash behavior. [PRAGMA reference](https://www.sqlite.org/pragma.html), [WAL](https://www.sqlite.org/wal.html), [temporary files](https://www.sqlite.org/tempfiles.html), [VACUUM](https://www.sqlite.org/lang_vacuum.html)

Calendar buckets require qualified UTC. During a bad clock or uncertain reboot continuity, use labelled monotonic/run segments and committed ordering, not invented UTC days. Freeze an explicit unknown-age rule: conservative discard or separately bounded descriptive retention, never fresh eligibility. Hard byte/count limits remain effective when age cannot be proved. Expiry does not imply secure erasure from backups, snapshots or storage remnants.

For an external H segment, prepare bounded private bytes under a unique operation/segment ID, complete and validate its metadata, then perform the qualified flush/publication sequence. Commit the Q manifest and any associated aggregate/checkpoint update with that same idempotency identity. **File publication and the database commit are not one atomic transaction.** Freeze recovery for each cut: unfinished staging, completed orphan, manifest without readable content, committed batch with missing acknowledgement, and interrupted retirement. Reconcile attributable identities and exact bytes before retry; never infer “not applied” from an error or count an imported batch twice. A digest in Q helps bind bytes but is not independent authentication.

If raw inputs were intentionally volatile, recovery cannot promise to replay the lost tail. Preserve known gaps or an unknown suffix; do not fabricate complete denominators. A missing archive may leave valid descriptive aggregates but makes detailed replay unavailable. Imports/recomputations use explicit provenance/model generations and cannot enter live authenticated state.

Migration needs a bounded old/new compatibility plan, one writer, validated output and attributable activation/recovery. Preserve or explicitly discard only optional classes; never erase mandatory state because a new schema or package fails. MSIX repair/reset, service delete/recreate and uninstall have different lifetimes. The [MSIX lifecycle owner](windows-msix-store.md#5-recovery-data-and-update-semantics) must protect mandatory recovery through removal. Keep heavy I/O/compression off critical timing work and retain real operation custody through cancellation and reconciliation.

### 5.5 Does a bundled binary catalogue need a signature?

**Binary encoding provides no authenticity.** JSON, CBOR and another binary format all need an authenticated origin plus strict admission. For the selected release-only delivery direction, a plausible lightweight candidate is an expected catalogue digest/length/revision embedded in an already authenticated executable, binding that release to the exact catalogue bytes. A checksum delivered beside an equally writable catalogue is insufficient. This proposal depends on the actual executable/package trust chain; it does not establish a format or digest algorithm.

A separate project signature, potentially a bounded [COSE](https://www.rfc-editor.org/rfc/rfc9052.html) envelope, becomes a stronger lifecycle candidate if catalogues later travel independently of executable releases or require separate publication authority. It adds verification-key bootstrap, signing custody, rotation, withdrawal, rollback and offline recovery obligations. It is not required merely because the payload is binary. Do not invent access to Microsoft's signing private key; Store package verification and a project-owned catalogue signature are different mechanisms.

For a separate catalogue resource, proposed loader order is **bounded read of exact bytes → authenticate those bytes → bounded decompression if used → strict semantic admission → immutable publication**. Before authentication parse only the minimum bounded framing needed for an allowlisted verification method. If compressed, authenticate the declared compressed artifact and metadata before expansion; cap input/output, records, strings, nesting, dictionaries and decoder work even for validly signed malicious content. Consume the same captured immutable bytes rather than reopening a pathname after checking it. Reject unsupported versions, inconsistent counts/IDs, unsafe endpoints and unknown required semantics before publication; no partial trust downgrade. Generated tables retain the same logical invariants and current-policy checks; any build-time validation depends on the actual installed-image trust contract described in the binary refinement.

Keep explicit MSIX `PackageIntegrity` / `Content Enforcement="on"` in the actual final manifest and qualify delivered behavior; Microsoft's documented default equals off. [MSIX integrity](https://learn.microsoft.com/en-us/windows/msix/desktop/tamper-protection) The retained MSI alternative's package/cabinet signature verification does not establish the same installed-content enforcement; it needs qualified installed-file ACLs, immutable code and actual catalogue revalidation. [Windows Installer signatures](https://learn.microsoft.com/en-us/windows/win32/msi/digital-signatures-and-windows-installer) CIG/image-signature policy restricts executable image loading, not arbitrary catalogue/database authenticity. [Image-signature policy](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_binary_signature_policy)

An authentic catalogue can still be obsolete, withdrawn, or restored together with an older signed application. Revision floors and timestamps need trusted lifecycle/time rules; first install, long offline periods and whole-system rollback remain OPEN-CAT-03. Rejecting an update does not authorize an unreviewed downloaded replacement, TOFU or an independent updater. No currentness or correct-UTC claim follows from a successful signature/digest check.

### 5.6 How can unauthorized SQLite changes be prevented?

Ordinary SQLite accesses files inside its caller's process; it is not a separately authenticated database server. [SQLite serverless model](https://www.sqlite.org/serverless.html) The proposed first boundary is the engine's actual identity, protected owner/DACL and service-SID isolation, including parent-directory create/delete/replace rights and child-file rights. Qualify another service sharing an account, ordinary users, stale handles, reparse/hardlink replacement and installer maintenance. A read-only database attribute or an ACL on the main file alone is insufficient. [Windows file security](https://learn.microsoft.com/en-us/windows/win32/fileio/file-security-and-access-rights)

Apply that boundary to the entire Q/H family, temporary files, backups and migrations from creation onward. UI clients obtain bounded service projections, not database handles or arbitrary SQL/path access. Admit only fixed reviewed SQL/prepared statements with bound data values and finite query/work limits. Evaluate defensive mode, `trusted_schema=OFF`, disabled extension loading and unused schema capabilities against the exact SQLite build; schema admission/validated domain construction remain necessary. Verify effective settings and compile-time availability rather than treating an unrecognized PRAGMA's lack of error as success. Treat a previously writable database as hostile. [SQLite security guidance](https://www.sqlite.org/security.html), [extension loading](https://www.sqlite.org/c3ref/enable_load_extension.html), [PRAGMA behavior](https://www.sqlite.org/pragma.html)

`integrity_check` checks structural consistency; it does not prove approved authorship, truthful measurements, completeness or freshness. Separate optional tamper-evidence and secrecy requirements. [SQLCipher](https://www.zetetic.net/sqlcipher/design/) offers page encryption and MACs, but requires a separately admitted dependency, key/custody lifecycle and verification of every actual temporary/output path. Application MACs likewise need independent key custody, exact covered identities/content and a defined set/sequence commitment—not an unprotected checksum/key stored beside the data. Neither option alone defeats whole valid-database rollback, an authorized compromised writer or an administrator controlling the process/keys. Do not call either an immutable audit trail.

Any new encryption-key persistence must respect the existing qualified-VBS / separately qualified-TPM-for-fresh-state / no-persistent-secret-cache direction; this draft adds no DPAPI, software-key or plaintext-secret fallback. Q remains advisory even if encrypted. The existing NTS/DNS secret snapshots, privacy generations, exact committed-envelope binding and forget barriers stay in their own stores; “statistics” is not permission to merge DNS learning into Q.

### 5.7 Can future NTS intermediate/root pins resist interception?

**Potentially, as an additional restriction with independently authenticated pins.** They can reject an otherwise trusted interception path that fails the pin policy; they do not prove correct UTC, prevent delay attacks or survive compromise of the pinned authority by themselves. Keep ordinary PKIX path, hostname, date and configured revocation checks, including OPEN-8's Windows limitations. Pin failure is not a date-recovery signal. NTS pins are separate from DoQ pins and apply to the intended NTS-KE identity, not every negotiated NTP address. [RFC 8915](https://www.rfc-editor.org/rfc/rfc8915.html), [RFC 9525](https://www.rfc-editor.org/rfc/rfc9525.html)

Compare full-certificate fingerprints with SPKI digests: the former bind certificate bytes; the latter bind the encoded public-key information and can tolerate certificate renewal retaining that SPKI. Match a leaf/intermediate/root at its specified role on the **same accepted certification path**, not an unrelated peer-supplied certificate. A root restriction admits broader CA issuance than a specific intermediate or leaf restriction; intermediate pinning must not silently promote that certificate to a trust anchor.

A clear proposed rule is **OR within each level's active/backup pinset, AND across all required levels**. A leaf-plus-intermediate policy therefore needs both; a flat any-match list cannot claim that result. Exact roles/algorithms and alternate/cross-signed-path behavior remain OPEN-PIN-01. Enforce the chosen rule in the actual TLS verifier and bind associations, session caches and resumption eligibility to current trust/pin generations.

Plan authenticated overlapping rotation, backup pins, withdrawal and recovery before activation. Expired/mismatching pins or failed catalogue updates cannot trigger automatic TOFU, pin replacement, disabled verification or plaintext downgrade. A successful old handshake cannot silently authorize a new policy generation. The implementation must qualify legitimate renewal beside interception, stale-session and wrong-path failures; this draft does not enable pinning or select its operational default.

## 6. Open decisions and owner questions

These are discussion items; they do not request a new approval before completing this draft.

| ID | Unresolved choice and why it matters |
| --- | --- |
| OPEN-CAT-01 | Exact binary resource or generated-table representation/schema, parser limits, release digest/signature strategy and trust-root lifecycle. The owner prefers binary shipment; decide the final representation after size/complexity and update requirements are concrete. |
| OPEN-CAT-02 | Initial admitted provider roster, operator permission, Pool vendor allocation, precise Roughtime wire/key compatibility and cold bootstrap resolution path. Source/default owners R-04 and OD-03 also retain the distinction between catalogue, reviewed preset, configured and active membership: freeze the preset's C/policy binding and whether automatic replacement is in scope before the dependent schema or acquisition work. If selected, replacement needs an eligible population, trigger, stable selection/reselection, finite traversal and continuing provider budgets; this question adopts no default or traversal permission. Existing source/time-assurance owners retain this work. |
| OPEN-CAT-03 | Catalogue revision floors, stale/offline first-install behavior, key withdrawal and rollback threat model under an unreliable clock. Release updates alone do not resolve every freshness case. |
| OPEN-DATA-01 | Useful default quality dimensions, minimum coverage, source-versus-path scope and threshold/hysteresis/reprobe behavior. Define before enabling automated exclusions. |
| OPEN-DATA-02 | Whether detailed raw history is needed by default at all; the minimum retained evidence and aggregates that answer the actual user/support questions. |
| OPEN-DATA-03 | Age/count/byte/cardinality limits and resource budgets, including total physical/temporary/RAM peak costs. Earlier numeric examples remain unselected. |
| OPEN-DATA-04 | Qualified UTC/date representation, supported true-date interval, monotonic/power continuity, leap-smear compatibility and aggregate segmentation rules. Reuse architecture/time-assurance owners. |
| OPEN-DATA-05 | Whether the adopted conflict/quarantine rule needs protected cross-restart state; its exact minimum, missing-state/durable-write failure behavior and authorized resolution. TA-W02 owns this freeze, distinguished from optional diagnostics and quality preferences; no blanket durable quarantine is selected here. |
| OPEN-DATA-06 | Reset/uninstall/reinstall/migration semantics and privacy export/redaction rules for each class. Do not solve optional-history recovery by deleting mandatory state. |
| OPEN-PIN-01 | Whether to implement per-NTS pins, chosen representation/levels, accepted-chain semantics, key rotation and recovery UX; the selected TLS/Windows verifier must enforce the actual policy. |
| OPEN-ST-01 | Q/H formats, SQLite build/binding/features and journal mode, effective owner/DACL and complete file-family protection. Any additional cryptographic key custody needs separate admission under the existing protected-storage direction, qualification gates and pause. |
| OPEN-ST-02 | Aggregate durability/loss tolerance, exact external-segment publication and idempotency protocol, bounded migration/cleanup, and recovery from ambiguous outcomes. Decide what can be replayed when raw inputs are deliberately dropped; metadata alone cannot recreate them. |

## 7. Qualification scope and next implementation prerequisites

This draft is documentation work only. It establishes no native Windows, package, cryptographic-provider, protocol-interoperability, resource-bound or security enforcement result.

Before implementing a chosen slice, its existing owner must freeze the relevant contract and independently expected outcomes. Retain valid neighboring cases beside each refusal; the following are proposed evidence cases, not executed tests or a replacement acceptance catalogue.

| Boundary | Independent expected distinction |
| --- | --- |
| Catalogue admission | Admit the valid exact release-bound artifact beside wrong digest/key/lineage, duplicate IDs, post-check path replacement and validly authenticated excessive expansion. Old-but-valid and unknown-time cases follow the explicitly frozen freshness policy. |
| Measurement/aggregation | Known fixtures preserve zero versus missing, correct stage denominators and M2 across disjoint compatible merges. Reject duplicate credit, overlapping windows, incompatible clock/reference/leap frames and dependence laundering; uncertain correction and dropped telemetry remain visible. |
| SQLite and owned files | Valid final-identity access succeeds beside wrong-token, parent-directory replacement, hostile schema, extension and sidecar/temp exposure cases. Verify effective selected settings; a structurally valid substituted database cannot gain authority. |
| Capacity and retention | Long readers, full WAL/temp space, too many contexts/files, oversized decoded batches, unknown age and insufficient VACUUM/migration headroom trigger bounded optional refusal/expiry with explicit gaps. Acquisition and mandatory-state rules remain intact. |
| Publication and migration | Cut execution before/after file publication, database commit, acknowledgement and retirement, including a second recovery failure. Reconcile the same operation; neither duplicate import nor a lost reply increments aggregates twice or fabricates missing raw detail. |
| NTS pins and generations | A legitimate accepted-path/backup-pin case succeeds beside unrelated matching certificates, wrong required level, invalid ordinary PKIX/name/date policy, rotation and stale session/association cases. No failure disables pins or enables plaintext. |
| Authority/privacy separation | Clear or corrupt Q/H without clearing current policy, consumed authority, trusted floors or secret/DNS privacy barriers. Preserve existing authenticated-conflict behavior; TA-W02 must separately define any new cross-restart quarantine and missing-state rule before that feature is claimed. |

Include malformed protocol/version/era input, invalid Roughtime responses and bounded redacted exports in the applicable existing CHECK/TA/ST owners' fixture scope. No catalogue roster, pin profile, quality threshold or protection mechanism becomes qualified by this table.

Use controlled finite fixtures and the repository's [testing contract](../../.agents/skills/testing/CONTRACT.md). Public server probes, package/service experiments, dependency installation and clock changes are not authorized by this document. Preserve the actual held/ready/deferred status in the existing task chain; do not open a competing execution plan merely because inventory fields are now listed.

## 8. Reference map

Primary links support the stated protocol/platform facts; field groupings and storage questions are design proposals. Source documentation is not implementation qualification.

### Existing NevroTick authority

- [Owner decisions](../nevrotick-owner-decisions.md), especially OD-02/03/05.
- [Architecture](../windows-time-architecture.md), especially §§6–8, 10.2.1 and 12.
- [Time-assurance integration](time-assurance-integration.md) and [existing task](../../tasks/time-assurance-integration.md).
- [Binary catalogue refinement](server-catalogue-binary-format.md): first-schema Roughtime keys, representation candidates and future publisher tooling; the [catalogue task](../../tasks/server-catalogue-and-statistics.md) owns the shared continuation.
- [Protected-storage plan](windows-protected-storage.md) and [existing task](../../tasks/windows-protected-storage.md).
- [Complete MSIX plan](windows-msix-store.md), [update assessment](../windows-time-update-assessment.md) and [roadmap](../../ROADMAP.md).

### Protocols and source discovery

- [jauderho/nts-servers](https://github.com/jauderho/nts-servers): candidate discovery input; pin its actual revision for a future curation run.
- [NTP Pool vendor guidance](https://www.ntppool.org/vendors.html): redistribution/fleet use and vendor zones.
- [RFC 5905](https://www.rfc-editor.org/rfc/rfc5905.html): NTP measurements and protocol fields.
- [RFC 8633](https://www.rfc-editor.org/rfc/rfc8633.html): operational source/time-policy considerations.
- [RFC 8915](https://www.rfc-editor.org/rfc/rfc8915.html): NTS authentication, KE and negotiated NTP destinations.
- [RFC 9523](https://www.rfc-editor.org/rfc/rfc9523.html): Khronos sampling and its assumptions.
- [RFC 10049](https://www.rfc-editor.org/rfc/rfc10049.html): current experimental Roughtime wire protocol, greasing and inconsistency evidence. Older draft ecosystems require explicit version qualification.
- [RFC 9525](https://www.rfc-editor.org/rfc/rfc9525.html): TLS service-identity verification.

### Storage and verification sources

- [SQLite application use cases](https://www.sqlite.org/whentouse.html), [file format](https://www.sqlite.org/fileformat2.html), [security guidance](https://www.sqlite.org/security.html), [WAL](https://www.sqlite.org/wal.html), [temporary files](https://www.sqlite.org/tempfiles.html), [PRAGMA reference](https://www.sqlite.org/pragma.html) and [VACUUM](https://www.sqlite.org/lang_vacuum.html).
- [SQLite serverless model](https://www.sqlite.org/serverless.html), [extension-loading controls](https://www.sqlite.org/c3ref/enable_load_extension.html) and [ZIPVFS compressed-database extension](https://sqlite.org/zipvfs/doc/trunk/www/index.wiki): distinct file-access, feature-admission and optional compression mechanisms; none is selected by this draft.
- [Apache Parquet file format](https://parquet.apache.org/docs/file-format/), [internal compression](https://parquet.apache.org/docs/file-format/data-pages/compression/) and [error recovery](https://parquet.apache.org/docs/file-format/data-pages/errorrecovery/).
- [RFC 8878, Zstandard](https://www.rfc-editor.org/rfc/rfc8878.html), [RFC 8949, CBOR](https://www.rfc-editor.org/rfc/rfc8949.html), [RFC 9052, COSE](https://www.rfc-editor.org/rfc/rfc9052.html), [SQLCipher design](https://www.zetetic.net/sqlcipher/design/) and [TUF specification](https://theupdateframework.github.io/specification/latest/index.html). Listing these alternatives selects none of them.
- Microsoft Learn: [MSIX signing](https://learn.microsoft.com/en-us/windows/msix/package/signing-package-overview), [MSIX content tamper protection](https://learn.microsoft.com/en-us/windows/msix/desktop/tamper-protection), [Windows Installer signatures](https://learn.microsoft.com/en-us/windows/win32/msi/digital-signatures-and-windows-installer), [file security](https://learn.microsoft.com/en-us/windows/win32/fileio/file-security-and-access-rights), [service SID configuration](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/ns-winsvc-service_sid_info), [new-object ownership](https://learn.microsoft.com/en-us/windows/win32/secauthz/owner-of-a-new-object), [chain-engine configuration](https://learn.microsoft.com/en-us/windows/win32/api/wincrypt/ns-wincrypt-cert_chain_engine_config) and [chain verification time](https://learn.microsoft.com/en-us/windows/win32/api/wincrypt/nf-wincrypt-certgetcertificatechain).
- [Windows image-signature mitigation policy](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_binary_signature_policy): image-loading scope; it is not a catalogue/database authentication mechanism.

## 9. Review and reconciliation record

The owner requested four distinct roles. All four completed their assigned
documentation stages. The independent reviews used fresh delegated contexts
(`fork_turns: "none"`), the actual candidate and relevant canonical requirements;
neither inherited the author's conversation or used the private continuity KB.
The [repository review procedure](../../.agents/skills/planning/plan-independent-review/SKILL.md)
governed stage scope and finding reconciliation. Read scopes below are the
reviewers' returned receipts; the coordinator did not receive a complete replay
of their per-tool traces and separately checked the candidate changes, relevant
canonical evidence, hashes and links.

Initial preparation calls for roles 2–4 encountered context-length errors and
were retried with bounded source reads. Those failed preparations supplied no
review result; the completed candidate reviews and targeted recheck below are
the evidence used for this handoff. No review verdict grants implementation,
native-experiment, package or release qualification.

### 9.1 Stage completion and frozen candidates

| Stage and agent | Completed result and candidate identity |
| --- | --- |
| 1 — `storage_inventory_author` | Wrote DATA-01–DATA-24 and the decision/meaning inventory; physical allocation remained pending. SHA-256: `1a540f31b20f123fefc6c2392ca3dd1567269c93a4baf8e4ceea7ad4cdd4abee`. |
| 2 — `storage_inventory_review` | Fresh independent inventory review; INV-R01–INV-R05 corrected and verified. Ready for allocation. Revised SHA-256: `f07d06eabc73960205173b435405c1064fa519d3f021c03454859fd4880eba2c`. |
| 3 — `storage_allocation_author` | Added one allocation row for each of the 24 DATA groups and the storage/security/lifecycle comparison. Preserved §§2–4. SHA-256: `d62e15ca67d5f9300947a86e0b3235bf304dfac8e9083ea005087e5e74f5beb4`. |
| 4 — `storage_allocation_critical_review` | Fresh read-only integrated review found ALLOC-R01 and returned REVISE. After the coordinator's one-row correction, the targeted recheck verified it fixed and returned READY for publication as a discussion draft. Reviewed corrected SHA-256: `81ab2c2ca4572a8604438312bde711273f3e2f4d9dcbec7cd6f4d4b5f0e984bc`. |
| Coordinator | Added these exact review records and reconciliation metadata after the successful recheck; no further design change. The unchanged README discovery candidate has SHA-256 `96ccb02f83fc0de0e934a4d873c2072be15ff4b4719bc09b6d270778dc9befc5`. |

The semantic body is frozen independently of this appendix: SHA-256
`badc72a328120611367a6b3c703e5901892f52f1d1e6ad40b894a249c0023e72` covers the UTF-8 bytes before the first
`## 9. Review and reconciliation record` heading. Earlier whole-document hashes
identify their actual review stages, before this appendix was appended.

### 9.2 Inventory review — verbatim

<details>
<summary>Completed stage-2 review and five dispositions</summary>

The candidate organizes a release-bundled time-source catalogue and local source-quality evidence into 24 logical data groups. It distinguishes curation and endpoint declarations, live acquisition/model state, quality summaries, optional detailed evidence, mandatory authority, secrets and publisher records. Scores describe observed conditions and coverage; they cannot authenticate a source or authorize a correction. The inventory identifies meaning and ownership while leaving physical allocation, formats, signatures, numeric retention policies and implementation undecided.

The inspected NevroTick authorities place acquisition in the unprivileged engine, numerical state in the core and final clock authority in the broker. Source independence, bootstrap ancestry, time domains and actual acknowledged corrections remain distinct. Roughtime-first cold bootstrap and its conditional reserve are future qualified extensions; existing strict NTS and current policy remain controlling. Optional cache loss cannot reset mandatory authority. Protected-storage construction and conflict-clearance work still have their existing owners and gates; this documentation stage does not displace the S157/S1.3 priority or execution holds.

## Review identity and scope

- **Reviewer/stage:** independent inventory reviewer, stage 2; completed, including permitted corrections and verification of the revised text.
- **Context mode:** fresh delegated task context with the neutral `review-inputs.md` packet and canonical sources. No parent discussion or private KB was retrieved. No author verdict was used as evidence.
- **Source context:** selected-documentation snapshot identified as `715a56561012d83ca0d667ef5ed1fd4ede555ad4`, not a full checkout or implementation audit.
- **Candidate:** `docs/plans/server-catalogue-and-statistics-draft.md`; the supplied before hash was independently matched before reading/editing.
- **Before SHA-256:** `1a540f31b20f123fefc6c2392ca3dd1567269c93a4baf8e4ceea7ad4cdd4abee` — 69,811 bytes.
- **After SHA-256:** `f07d06eabc73960205173b435405c1064fa519d3f021c03454859fd4880eba2c` — 73,109 bytes.

**Actual inspected scope:** complete candidate; complete `AGENTS.md`, testing `CONTRACT.md`, `plan-independent-review/SKILL.md` and `review-and-handoff.md`. Targeted canonical reads covered `docs/README.md` authority/current amendments; owner decisions OD-02/03/05; architecture §§3.1–3.3, §6 state types, §7.4, §8.1, §§12.1–12.3 and principal §19.4 constraints; time-assurance plan §§1, 3–7 and task §§1–5; protected-storage plan owners, construction, exact commits, privacy/status and task ST-W01/ST-W07/reserves; MSIX trust/integrity, role protections and lifecycle; development §7/current S157 and §8 limits; roadmap DNS/pinning context; features F-06–F-09 and §§3.2–5; update assessment §§1–3 and §5. Primary checks also inspected [RFC 5905 §8](https://www.rfc-editor.org/rfc/rfc5905.html#section-8) and [RFC 10049 §§7–8](https://www.rfc-editor.org/rfc/rfc10049.html#section-7) for timestamp inputs, greasing and linked measurements. This was not an exhaustive read of every repository document.

## Original findings and disposition

### INV-R01 — Medium — Live exchange inputs were implicit

**Location:** DATA-10 attempt fields/lifetime and DATA-11 measurement fields. The original inventory named internal attempt identities, outcome summaries, RTT and offset without explicitly inventorying current wire correlation/custody or the four timestamp inputs. Architecture §6's `Exchange` and §7.4 require those distinct values, including real local T1 separately from a correlation token. An allocator could omit necessary working state or confuse derived values with reconstructible exchange evidence. **Smallest correction:** add one live-correlation/custody row and one timestamp-input row, including capture provenance and explicit transient lifetime. No routine packet archive or outstanding-request restoration follows. **Disposition:** applied and verified by rereading both revised groups against the cited contracts.

### INV-R02 — Medium — Secret lifetime and restart scope needed separation

**Location:** DATA-22 secret payload, binding metadata and lifetime. Full-authentication/key ages were present, but cookie issuance/receipt and cumulative resumption ages were not explicit. Listing TLS/session/exporter secrets under a persistence-conditional heading could also suggest a wider restart cache than the protected-storage plan permits. Architecture §7.4 and protected-storage §2 distinguish these ages and do not adopt TLS resumption persistence. **Smallest correction:** list separate ages, identify working secrets versus a bounded restart allowlist, preserve unknown ages and prohibit renewal of full-authentication age through cookie/UDP/resumption activity. **Disposition:** applied and verified; exact limits remain OPEN-8 work.

### INV-R03 — Medium — Exact committed-snapshot identity was incomplete

**Location:** DATA-22 storage envelope/store authority. Generic provider, nonce, tag and generation metadata did not explicitly carry separate root/data epochs, snapshot sequence or the catalogue's binding to exact committed bytes. Architecture §12.1.2 and protected-storage §4.2 make these consequential: another valid snapshot under the same root is not the committed save. **Smallest correction:** add those envelope identities and a committed-save row, including independently admitted verification identity and initialized-empty versus NoPersistentCache distinctions. **Disposition:** applied and verified. The row names the protected root/key catalogue; it selects no database, algorithm or provider.

### INV-R04 — Medium — Future conflict persistence appeared already mandatory

**Location:** DATA-06 mandatory exclusions; DATA-17 lifetime; DATA-18 classification/clearance; DATA-21 policy row; OPEN-DATA-05. Original wording treated cross-restart conflict/quarantine persistence as current mandatory state. Time-assurance §6 requires preserving conflict across branch changes/timeouts, while TA-W02 and the existing task leave the exact admission, clearance and recovery contract future work. The wording could prematurely adopt durable quarantine or widen a quality-derived restriction. **Smallest correction:** retain the existing conflict-preservation requirement, make new persistent quarantine conditional on its adopted policy, and expose restart/missing-state behavior as an open freeze. **Disposition:** applied and verified across all five semantic locations; optional history still cannot supply a required safety resolution.

### INV-R05 — Low — Requested geography was classified as optional enrichment

**Location:** DATA-03 classification and Location row. The neutral request explicitly includes country/region, but the original classification described geography as proposed where useful. **Smallest correction:** require the logical country/region field with an explicit unknown state while keeping finer site enrichment conditional. This satisfies inventory coverage without inventing physical location, treating anycast as fixed or collecting extra client location. **Disposition:** applied and verified against the request and revised row.

## Verification and handoff

All five findings are corrected at the after hash. The complete original was read; changed fields and affected authority distinctions were reread after editing. Byte comparison confirms that only §§3 and 6 changed, all DATA-01–DATA-24 identifiers remain, and §5's allocation placeholder is byte-identical. No implementation tests, service probes, target execution or external writes occurred. No necessary inventory-review input remains unavailable; explicitly open design choices remain with their owners.

**Verdict: ready for stage 3 storage allocation.** The next permitted action is to map the revised DATA groups to proposed storage and failure/recovery behavior while preserving their existing, proposed and conditional classifications. This verdict establishes documentation-stage readiness only.

</details>

### 9.3 Critical allocation review — verbatim

<details>
<summary>Completed initial stage-4 review: REVISE for ALLOC-R01</summary>

The draft proposes a reviewed source catalogue shipped with application releases, compact current state in RAM, a small advisory quality store, and optional compressed detail only where its value justifies retention. Its 24 DATA groups separate identities, observations, aggregates and supporting evidence from current policy, clock authority, NTS secrets and publisher signing custody. SQLite, archive formats, catalogue authentication details, retention budgets and NTS pins remain design proposals.

The inspected project contracts retain one engine policy path and one privileged clock broker. The future cold-bootstrap direction is Roughtime followed by full NTS, with a conditional provisional-date reserve; healthy retained NTS gains no new bootstrap dependency. Mandatory policy, correction journals and trusted floors have independent recovery rules. Optional NTS/DNS persistence has a separate protected-storage lifecycle. Complete hardened MSIX preparation is selected, while implementation qualification, the current Core priority and applicable execution holds remain unchanged.

## Identity, independence and completion

- **Stage:** independent critical review for publication as a discussion draft only.
- **Context:** fresh reviewer context; read-only inspection followed by this authorized scratch-report write. No author discussion, prior draft reviews, private KB, target-code execution, server probes or additional agents were used.
- **Baseline:** selected-documentation snapshot identified as commit `715a56561012d83ca0d667ef5ed1fd4ede555ad4`, tree `0e454ff69e1b400dd3a8271f2ef6d4b4f1690eb9`. This directory is not a full Git checkout or an implementation audit.
- **Candidate:** `docs/plans/server-catalogue-and-statistics-draft.md`, all 606 lines and 103,900 bytes inspected; SHA-256 verified as `d62e15ca67d5f9300947a86e0b3235bf304dfac8e9083ea005087e5e74f5beb4`.
- **Discovery change:** `docs/README.md`, new table row at line 28 inspected; SHA-256 verified as `96ccb02f83fc0de0e934a4d873c2072be15ff4b4719bc09b6d270778dc9befc5`.
- **Completion:** completed; one correctable documentary finding below. Pending coordinator entries in §9 are expected workflow bookkeeping, not a design finding.

## Actual canonical scope

Required instructions were read: `AGENTS.md`, `docs/README.md`, `.agents/skills/testing/CONTRACT.md`, `.agents/skills/planning/plan-independent-review/SKILL.md` and `.agents/skills/planning/references/review-and-handoff.md`, plus the supplied review-input packet.

Substantive inspection covered:

- `docs/nevrotick-owner-decisions.md`: OD-02/03 and amended OD-05; `docs/plans/time-assurance-integration.md`: §1 and §§3–7; `tasks/time-assurance-integration.md`: §§1, 3–5.
- `docs/windows-time-architecture.md`: §§3.1–3.3, 4, 6's domain/state definitions, 7.4, 8.1, 10.1, 10.2.1, 11, 12.1, 12.3.1–12.3.4, 17/17.1, 19.4 and 22.
- `docs/plans/windows-protected-storage.md`: §§1–2 and 4.2–4.3; `tasks/windows-protected-storage.md`: §1 and ST-W01; `docs/plans/windows-msix-store.md`: §1, §3's trust introduction, §§3.1, 3.3 and 5.
- Relevant current-work, hold and resource passages in `docs/plans/development-plan.md` §§7, 8 and 8.4; KHR-0/RGT-0 direction and §5 in `ROADMAP.md`; F-03–F-06, F-09–F-10 and §5 in `docs/windows-time-features.md`; status, time provenance, authority/handoff and persistent-trust passages in `docs/windows-time-update-assessment.md` §§1, 3.3–5.

## Allocation coverage and critical assessment

Every DATA group has an explicit allocation. The following assessment concerns the documented allocation, not an observed implementation.

| DATA groups | Assessment |
| --- | --- |
| 01, 24 | Publisher research, private correspondence and signing custody stay outside client stores; only appropriate public release references/material reach the client. |
| 02–05 | Immutable catalogue definitions and current authoritative policy remain distinct from historical dictionaries. Artifact identity, operator uncertainty, endpoint re-admission and key withdrawal are retained. |
| 06, 07, 21 | Current source/pin policy, trusted floors, consumed authority and recovery journals remain with existing owners; statistics reset cannot clear them. |
| 08–12 | Current destinations, generations, exchanges, measurements and verification stay bounded in RAM; retained summaries remain descriptive. DATA-10's retirement wording needs ALLOC-R01. |
| 13, 14, 18 | Bootstrap ancestry, actual correction outcomes and conflicts remain distinct. Branch changes/timeouts cannot clear conflict; cross-restart safety storage and clearance remain TA-W02 choices rather than a newly adopted permanent quarantine. |
| 15 | Only the separately qualified bounded drift prior is a persistent warm-start candidate; archives cannot restore full sample history as fresh evidence. |
| 16, 17, 19, 20 | Advisory vectors/preferences, manifests, gaps and resource accounting have bounded homes. Disjoint populations, idempotency, unknown age, contextual bias and physical maintenance costs are explicitly addressed. |
| 22 | Secret association payloads and exact committed snapshots retain their separate owner, generations, authentication ages and no-cache semantics. DNS learning cannot be moved into Q by renaming it statistics. |
| 23 | Logs, status, per-user preferences and explicit exports retain separate authority, privacy, expiry and capacity rules. UI projections do not expose database handles or secrets. |

The draft correctly treats external archive publication and the Q transaction as separate durability events, preserves ambiguous outcomes, and acknowledges that volatile lost input cannot be replayed. It makes migrations, offline age, total physical allocation and optional-history loss explicit without promising rollback resistance. Catalogue origin verification is separated from freshness and server correctness. Pins remain an additional accepted-path restriction with authenticated rotation, not a replacement trust anchor or weaker recovery path.

Primary-source checks supported the consequential distinctions examined: SQLite's [PRAGMA behavior](https://www.sqlite.org/pragma.html), [WAL reader/checkpoint interaction](https://www.sqlite.org/wal.html), [temporary-file families](https://www.sqlite.org/tempfiles.html), [VACUUM headroom](https://www.sqlite.org/lang_vacuum.html) and [hostile-input guidance](https://www.sqlite.org/security.html); [ZIPVFS's separate extension status](https://sqlite.org/zipvfs/doc/trunk/www/index.wiki); and [SQLCipher's page protection and temporary-output caveats](https://www.zetetic.net/sqlcipher/design/). Apache documents [internal Parquet/Zstd compression](https://parquet.apache.org/docs/file-format/data-pages/compression/) and [metadata-dependent recovery limits](https://parquet.apache.org/docs/file-format/data-pages/errorrecovery/). Microsoft confirms [explicit MSIX integrity enforcement/default-off behavior](https://learn.microsoft.com/en-us/windows/msix/desktop/tamper-protection) and the [changed semantics of treating an intermediate as a trust anchor](https://learn.microsoft.com/en-us/windows/win32/api/wincrypt/ns-wincrypt-cert_chain_engine_config). [RFC 10049 §§7–8](https://www.rfc-editor.org/rfc/rfc10049.html#section-7) supports the greasing and sequential-proof distinctions. These checks establish documentary support, not runtime enforcement.

## ALLOC-R01 — Separate request retirement from native-resource lifetime

**Severity:** Medium — conflicting lifecycle instruction in a security-relevant allocation row.  
**Disposition:** Open; small documentation correction required before publication.

**Location:** `docs/plans/server-catalogue-and-statistics-draft.md`, §5.1, DATA-10 allocation row, line 437: after identifying request UIDs/pending exchanges, the row says, “Retire only with actual operation completion.”

**Evidence:** `docs/windows-time-architecture.md` §7.4 requires genuine timeout, cancellation and lifecycle invalidation to retire requests independently. Section 17.1 separately retains native-job permits/resources until actual completion and prohibits cancelled, expired or obsolete results from publication. The candidate's own DATA-10 live-correlation entry at line 189 recognizes the separation, but the allocation row's unqualified retirement instruction collapses it.

**Consequence:** An implementer could retain request eligibility until a delayed native operation finishes. Cancellation before the request deadline, without an unrelated generation change, illustrates why request invalidation cannot depend on native completion. Conversely, logical retirement must not release a still-running operation's resources. This is an evidenced documentary ambiguity with a plausible implementation consequence, not an observed runtime defect.

**Smallest correction:** Replace the quoted sentence with: “Retire request eligibility according to the protocol's completion, timeout, cancellation and invalidation rules; retain native-operation permits, handles and buffers until actual terminal completion.” Keep the existing incomplete-coverage statement. The allocation author/coordinator can make this correction under existing authority; no policy decision, target test or native experiment is needed for this review stage.

## Verdict and permitted next action

**REVISE for publication as a discussion draft**, solely for ALLOC-R01. No other material publication defect was found within the inspected scope.

Exact formats, numerical budgets, provider roster, catalogue freshness/rollback policy, conflict persistence/clearance, SQLite build/mode, archive protocol and NTS pin adoption remain legitimate open implementation choices. Their future evidence gates are not prerequisites for this draft's publication. No implementation, package, security or release qualification is asserted.

**One permitted next action:** correct ALLOC-R01 in the allocation row, verify the changed wording against §§7.4/17.1 and record the resulting candidate identity and disposition in the existing coordinator reconciliation before the already-authorized documentation publication.

</details>

### 9.4 Targeted recheck — verbatim

# Targeted allocation review recheck

**Completion:** completed. **Scope:** ALLOC-R01 and preservation of the previously reviewed discussion-draft scope. This was a targeted read-only comparison followed by this authorized report write; no broad review, runtime test, server probe or implementation qualification was repeated.

## Verified identities and change scope

| Artifact | Verified SHA-256 |
| --- | --- |
| Revised `docs/plans/server-catalogue-and-statistics-draft.md` | `81ab2c2ca4572a8604438312bde711273f3e2f4d9dcbec7cd6f4d4b5f0e984bc` |
| Frozen original draft | `d62e15ca67d5f9300947a86e0b3235bf304dfac8e9083ea005087e5e74f5beb4` |
| `docs/README.md` | `96ccb02f83fc0de0e934a4d873c2072be15ff4b4719bc09b6d270778dc9befc5` |
| Original critical-review report, unchanged | `38eb85f835806a471d09bf479e2bfead128d0836e5e14a321bea893985956f5c` |

The revised draft contains 606 lines and 104,054 bytes. A complete textual comparison against the frozen original confirmed exactly one changed row: §5.1, DATA-10, line 437. The README and original review report retain their previously verified identities. Unaffected conclusions and open choices retain the scope of the original completed review.

## ALLOC-R01 disposition

**Verified fixed.** The corrected row now says:

> Retire request eligibility according to the protocol's completion, timeout, cancellation and invalidation rules; retain native-operation permits, handles and buffers until actual terminal completion.

I re-read the relevant passages of `docs/windows-time-architecture.md` §§7.4 and 17.1. The replacement preserves independent logical request retirement while keeping actual native-work custody until completion. It therefore removes the conflicting instruction identified in ALLOC-R01. The surrounding no-request-restoration and incomplete-coverage statements remain intact. This verifies a documentary correction; it does not assert implemented cancellation behavior.

## Publication verdict

**READY for publication as a discussion draft.** No material finding remains open from this review. Existing implementation choices and qualification gates remain unchanged; they do not block this documentation stage.

**Permitted next action:** append the exact review/recheck records and coordinator metadata, verify the final publication identity and preserved design scope, then complete the already-authorized publication of the two documentation files.

### 9.5 Coordinator reconciliation and documentary checks

| Requirement or boundary | Reconciliation |
| --- | --- |
| Owner's catalogue and reliability-data request | Recorded all requested source/endpoint/region/protocol and observation fields. Country/region permits explicit unknown; finer location is conditional. The admitted production roster and precise exclusion policy remain open. |
| Existing source and clock authority | Preserved Roughtime-first future bootstrap, full NTS, conditional reserve, source independence and acknowledged-effect provenance. Optional statistics create no authentication, extra vote or broker correction authority. |
| Live state versus restart state | INV-R01–R03 clarify transient exchange inputs, distinct NTS ages and exact committed protected snapshots. No pending exchange or full estimator-history restoration is adopted. |
| Conflict policy | INV-R04 preserves conflict across branches/timeouts without inventing a permanent or cross-restart quarantine. TA-W02 still owns persistence, clearance and missing-state behavior. |
| Native-operation lifetime | ALLOC-R01 now separates immediate logical request retirement from retention of real operation resources until terminal completion; the independent targeted recheck verified that correction. |
| Storage mechanisms and boundaries | Q/H/C/U/R are proposed allocations, not selected dependencies, paths, formats or providers. Existing JSON contracts, mandatory stores, NTS/DNS protection/privacy rules and owner-controlled lifecycle remain in force. |
| Resources and evidence | Numerical examples remain unmeasured and unadopted; existing 16-source/64-sample core caps and the separate diagnostic budget remain distinct. No timing, security-enforcement, native or resource benchmark was executed. |
| Development and delivery | The current S157/S1.3 priority and Windows/fuzz/storage holds are retained. The selected MSIX preparation direction and owner Store attempt gain no new approval or qualification from this draft. |

The coordinator verified the frozen stage hashes, the stage-2 inventory's
preservation through allocation, exactly one allocation row for each of the 24
DATA IDs, and the one-row scope of the final semantic correction. Internal
Markdown targets/anchors were checked against the untruncated baseline tree and
available target documents. All other retrieved baseline documents were preserved;
the public change is this new draft plus its single `docs/README.md` discovery row.
The raw reviews and recheck above are retained without rewriting their findings
or comprehension paragraphs. No material documentary finding remains open;
§6's explicitly unresolved design choices remain with their existing owners.

Final publication uses the real remote base tree and an expected-head lease,
not a synthetic local repository. The coordinator reports actual publication
and remote-content verification separately. These documentary checks are not
execution of the proposed cases in §7 or completion of any product phase.

## 10. Minimum record profile and bounded comparison card

**Addendum: 4 November 2026. Status: discussion proposal; no exact format, dependency, retention default or security policy adopted.** This develops OPEN-CAT-01, OPEN-DATA-02/03/04 and OPEN-ST-01/02. At this addendum's original checkpoint, sections 1–9 were preserved byte-for-byte from commit `d6bb2bdcd2145950586407698490b61d9e12ced9` (tree `3635c70a72cd32ca0a7f66cc545719c1aae46ece`). The later binary-catalogue refinement updates current catalogue passages; §9's historical reports remain unchanged and review neither this addendum nor the later refinement. The [task and review record](../../tasks/server-catalogue-and-statistics.md) carries the same workstream forward. No benchmark has run.

### 10.1 Minimum logical records

The six families below refine the DATA inventory; they are not six required tables, files or public API types. C/Q/H retain §5's conceptual meanings. The engine owns advisory quality writes; the core remains independent of files and ambient clocks; UI receives bounded service projections. Existing strict JSON/domain-construction boundaries remain unchanged. A format experiment does not replace their parsers or authorize a Cargo dependency.

| Family and proposed identity | Minimum information and lifecycle |
| --- | --- |
| **Catalogue/source profile — C.** Artifact `(lineage, revision)`; stable source/operator/endpoint IDs within it. | Schema/reader compatibility, release binding, payload length/digest/counts, review and provenance references. Source operator/dependence identities, host/port/transport, exact protocol/version, time-scale/leap policy and country/region with method or explicit unknown. Separate NTS KE identity from negotiated-destination policy. The Roughtime variant requires algorithm, root-key bytes and exact wire/profile binding from the first schema; runtime verification remains future work. NTS pin policy remains conditional. Authentication-envelope fields depend on OPEN-CAT-01. Changed definitions require fresh admission; Q labels/tombstones do not become current source authority. DATA-01–07. |
| **Immutable observation context — RAM; minimal Q.** `(history_generation, context_id)` under a local origin namespace. | Source/endpoint/definition/catalogue references; applicable protocol, association, policy/key/pin, network/resolution, boot/power/clock-mapping and actual applied-effect generations; timestamp provenance and transformation/evaluator references. Do not multiply one authority into several fictitious independent generations. Bound retired dictionary entries; retain only those needed by surviving records. Missing context makes interpretation unavailable, rather than substituting today's definitions. No duplicate resolver cache or client-location history. DATA-03/04/08/09/19. |
| **Attempt outcome/observation — normally RAM.** `(engine_incarnation, attempt_id)`; accepted measurements have distinct observation IDs. | Context/round, reason/stage, applicable send/arrival/processing/deadline instants, stage outcomes, terminal category and bounded reason. Distinguish scheduled/admitted/sent/received/validated/authenticated/eligible; a stage not run is not a failure. RTT, NTP four-timestamp delay, signed phase and defined uncertainty exist only when produced. Later selection belongs to an assessment; unassessed is not unselected. Drop unnecessary packets/T1–T4 after computation. Never restore pending exchanges; logical request retirement does not release native buffers/handles/permits before actual completion. DATA-10–12. |
| **Compatible population aggregate — Q.** `(history_generation, compatibility_key, population_id, evaluator_revision)`. | Source/path/protocol/time-scale and relevant reference/clock context; disjoint covered observations; coverage bounds/time basis, stage denominators, metric counts, gaps and usable-observation age basis. Per metric: count, mean, M2, min/max and missing/rejected counts. With no values, mean/extrema are absent; sample variance requires at least two. Keep quality dimensions separate. Merge only compatible disjoint populations; hourly/daily representations cannot both receive credit. Unknown UTC uses labelled run segments. No scalar honesty probability is required. DATA-16. |
| **Sparse assessment/transition/evidence reference — conditional Q/H.** `(producer_incarnation, assessment_id)`. | Evaluation instant, context, assessed observation and reference/support IDs, dependence limitations/bootstrap ancestry, verdict/reason and evaluator revision. Record whether the source influenced its comparison reference. A selection assessment cannot create another observation. Qualified rebasing references the actual acknowledged broker effect. Future cooldown variants need scope, old/new state, evidence, rule revision, deadline and recovery; they do not remove administrator restrictions. Safety-conflict ownership and conditional durability remain TA-W02/OPEN-DATA-05. DATA-13/14/17/18. |
| **Coverage/commit/segment manifest — Q; optional H reference.** `(history_generation, commit_id)` and a segment ID where detail exists. | Producer/event ranges, aggregate/checkpoint generation, dictionary/schema references, exact counts/bytes, reconciliation state, known gaps or unknown suffix, loss and supersession. An aggregate update and its coverage share one attributable durable acceptance identity. External-file publication and Q commit are separate operations; reconcile exact content and ambiguous outcomes without double credit. A digest does not authenticate an attacker-controlled record. A watermark cannot reconstruct discarded raw inputs. DATA-19/20. |

Local identities are opaque, not hardware serials, and cannot be reused across reset to revive old data. Facts eligible for aggregation carry producer-incarnation sequence information. Versioned presence tags distinguish unknown, unavailable, not applicable, not run, rejected and intentionally not retained; **zero remains a value**. Bound context cardinality independently of the existing 16 active-source and 64 regression-sample-per-source caps. Incompatible contexts split populations unless a qualified compatibility rule permits a merge.

Proposed relative units are nanoseconds for durations, RTT, NTP delay, signed phase and uncertainty; positive phase means Windows behind. M2 has squared units and counts are exact integers. Uncertainty retains its bound/estimate meaning. Production widths, ranges and absolute calendar/era representation remain OPEN-DATA-04; signed 64-bit Unix nanoseconds are not assumed to cover the recovery goal. Event, processing and evaluation instants remain distinct; monotonic values require clock/generation/suspend semantics. See [architecture §6](../windows-time-architecture.md#6-domain-types-and-state-authority).

Mandatory policy, trusted floors, correction authority/journals and clock/W32Time lifecycle stay with their existing owners. NTS/DNS secrets, exact committed-envelope identity, root/data/privacy generations and separate authentication ages remain with [protected storage](windows-protected-storage.md). No Q/H reset refunds authority, clears uncertain effects, revives requests or proves a conflict resolved. This profile adds no durable/global quarantine, certificate pin, automatic exclusion or new bootstrap policy. Ordinary NTP/NTS data work does not depend on completing RGT-0; requiring Roughtime schema fields now preserves the current RGT-0 → CHR-0 implementation sequence.

### 10.2 Compare equivalent capabilities

| Lane | Mutable advisory state Q | Optional detail H | Question |
| --- | --- | --- | --- |
| C-J (reference) / C-C | Not applicable | Not applicable | Same immutable catalogue as an optional JSON reference and constrained CBOR candidate: bounded admission, bytes and decode cost. JSON is not the preferred shipped format. |
| Q0 | SQLite typed rows | Absent | Can dictionaries, aggregates, gaps and sparse events answer the required questions? |
| Q-J | Bounded versioned JSON snapshot | Absent | Can simpler full-snapshot persistence provide the same Q operations and recovery contract? |
| H-R | Same Q contract | SQLite typed event rows | Cost of detailed filtering/replay. |
| H-B | Same Q contract | SQLite compressed BLOB batches | Same detail, including decoding and filtering. |
| H-P | Same Q contract | Completed Parquet segments, internal Zstd | Same detail with column selection. |
| H-S | Same Q contract | Completed JSONL batches, Zstd | Simple sequential-detail baseline. |

Q0 and Q-J expose identical summary/update/expiry/query results and explicitly unavailable raw replay. Charge Q-J's full decode/rewrite/publication cost. For an H comparison, freeze one common Q implementation and identical detail fields/ordering/retention; use identical JSONL input bytes for H-B/H-S. One raw representation suffices. H-B counts toward its entire database family, not a second free allowance. Parquet uses [internal page compression](https://parquet.apache.org/docs/file-format/data-pages/compression/), not an outer wrapper. These lanes compare mechanisms; none selects a product library or promises smaller/faster results.

The [binary refinement](server-catalogue-binary-format.md) adds generated immutable tables as a serious C alternative. Freeze an equivalent corpus and one binary candidate at a time before a later catalogue comparison; account for complete image/package costs as well as data bytes. This refinement neither expands §10.4's finite envelope nor executes a lane. The Q0/Q-J no-raw pilot remains separate.

### 10.3 Synthetic inputs and independent expectations

All quantities in §§10.3–10.6 are **proposed fixture/run settings, not product defaults or measurements**. Freeze literal schema, presence tags, sort order, numerical tolerances, oracle and resulting fixture digests before execution. No real endpoint, signing key, NTS secret or network call belongs in these fixtures. Synthetic verification labels never establish protocol authentication.

Catalogue recipes use 16, 256 or 4,096 entries: index `i`, operator `floor(i/4)`, hostname `s{six-digit-i}.invalid`, protocol tag `i mod 3`, region `i mod 4`, revision 1 and provenance text of `1+(i mod 256)` repeated `a` characters. Every compared representation retains identical fields and meanings. Freeze remaining valid port/transport/profile fields and synthetic Roughtime algorithm/key-byte bindings in the fixture schema; protocol tags and public test keys are fixture variants, not supported-server claims or production trust material. Measure bounded admission separately from the unresolved release-authentication construction.

For quality/history, use sources `s=0…15`, tick `k` and scheduled-slot identity `16k+s`, delivered in that order. A missing outcome has a gap marker, not a fabricated attempt result. Delivered outcome ID is `o:<slot>`; a successful measurement is `m:<slot>`. Let `r=k mod 50`: `r=0` is a telemetry gap, `r=1,2` timeout, `r=3,4` verification rejection, `r=5` cancellation, all others success. These categories imply counts analytically:

| Profile | Ticks per source; interval | Scheduled slots | Delivered outcomes | Successful measurements |
| --- | --- | ---: | ---: | ---: |
| First pilot | 200; 64 s | 3,200 | 3,136 | 2,816 |
| Ordinary | 9,450; 64 s | 151,200 | 148,176 | 133,056 |
| Context churn | 9,450; 64 s | 151,200 | 148,176 | 133,056 |
| Burst | 86,400; 1 s | 1,382,400 | 1,354,752 | 1,216,512 |

For received rejection/success outcomes, `RTT_ns = 1,000,000 + 1,000s + 17(k mod 101)`. Success additionally has `offset_ns = 1,000(s−8) + 25((k mod 31)−15)` and `uncertainty_ns = floor(RTT_ns/2)+1,000`. All other metric fields are absent with reason. RTT and offset therefore have different valid counts. Use operator `floor(s/4)` and evaluator 1; these labels grant no independence.

Ordinary elapsed time is `64k` seconds, starting at synthetic UTC `2026-01-01T00:00:00Z`; context label is `floor(elapsed/86,400) mod 2`, catalogue revision `1+floor(elapsed/172,800)`. Complete context identity also includes source, run and catalogue revision. Churn instead uses context label `floor(k/50)`—189 labels and 3,024 source/context pairs—and resets monotonic time at `k=6,750`; remaining outcomes have a new run and unknown UTC. Preserve those run segments without inventing UTC or using the fixture author's hidden elapsed time to assign freshness.

Burst uses elapsed `k` seconds and the ordinary context/revision rules. For less repetitive values, initialize unsigned 32-bit `x=0x4E545031`; advance once per scheduled slot using `x ^= x<<13; x ^= x>>17; x ^= x<<5`, truncating to 32 bits after each step. Replace received RTT with `1,000,000+(x mod 100,001)` and successful offset with `(x mod 200,001)−100,000`; recompute uncertainty and preserve absence rules. Patterned synthetic data cannot predict real compression ratios.

**Independent oracle:** use a literal six-slot ledger before the generated corpus:

| Slot | Expected fact | RTT ns | Offset ns |
| --- | --- | ---: | ---: |
| 0 | One telemetry gap; no delivered outcome | Absent | Absent |
| 16 | Timeout | Absent | Absent |
| 48 | Verification rejection | 1,000,051 | Absent |
| 80 | Cancellation | Absent | Absent |
| 96 | Success | 1,000,102 | −8,225 |
| 248 | Success | 1,008,255 | 0 |

Expect six scheduled slots, five outcomes, two successes, three RTT values and two offset values. This ledger is a separate tiny fixture, not a contiguous workload. QA independently derives generated counts/sums/squared sums and compatible partitions with exact integers/rationals. A separate moment control `[-20,0,20] ns` has `n=3`, mean 0, M2 `800 ns²`, sample variance `400 ns²`. Valid controls include no-valid-value buckets and exact integer `9,007,199,254,740,993`; never lose it through floating conversion. Pair valid disjoint merges/current-generation records with duplicate credit, overlapping rollups, wrong generation and incompatible-frame refusals. Shared codec round trips or agreement between adapters are not the oracle.

### 10.4 Small pilot and broader cases

The first executable candidate compares **only Q0 versus Q-J**, without raw history, using the literal ledger and 200-tick profile. Use two acknowledged batches of 1,600 scheduled slots, retaining their exact commit receipts throughout the pilot. Exercise update/read/restart, idempotent same-identity recovery and refusal of the same identity with different content. Outside a future bounded receipt window, refuse replay or explicitly report unavailable reconciliation; do not infer that an old batch is new. Read latest summaries for sources 0/7/15, all-source summaries and coverage; raw replay is explicitly unavailable.

Pilot envelope: **15 minutes total**, one worker, two adapter trials of at most 60 seconds each, no automatic reruns. Preparation within the run, controls, interruptions, recovery and cleanup all count. Record prerequisite/harness preparation separately; an unready harness is not a benchmark result. Proposed process ceiling is 512 MiB and total output ceiling 256 MiB. Stop on a correctness mismatch, resource ceiling or missing equivalent durability contract; retain partial/failure results. A pilot supplies feasibility/accounting evidence, not a statistically supported winner.

Optional later coverage exists only for a named unresolved capability/capacity question. Larger profiles use batches of 4,096 slots plus the final partial batch; fixture retention is seven qualified days hourly, 180 days daily and seven days detail with a 1.5-million-slot ceiling. Unknown-age detail shares that ceiling and retains at most two runs. Apply identical logical expiry and charge physical slack. Query classes add last-day summaries, context gaps, source-0 last-60-tick detail, slot-96 lookup and full retained-detail scan; derive each answer independently. Advancing fixture time by 8 and 181 days checks known-age expiry without inventing age for unknown segments.

Select relevant interruption points around payload preparation/flush/publication, Q commit/acknowledgement, retirement, compaction, migration copy/activation and recovery. Reconcile orphan/missing segments and lost replies; include disk exhaustion, insufficient migration headroom and one interrupted recovery. Mark inapplicable external-file stages as such. Proposed v1→v2 adds nullable `path_kind` with old values unknown; reject unsupported majors, preserve admissible old state and separately owned mandatory-state sentinels. Parser controls include duplicate IDs, wrong types, truncation, unsupported versions and oversized expansion. These are future cases under §7, not completed ST-T/CHECK evidence.

A possible **upper envelope**, never a required next campaign: at most 54 workload trials (six Q/H lanes × three profiles × three repetitions) at 90 seconds, 18 catalogue trials at 10 seconds, and 72 fault traces plus 24 parser controls at 10 seconds. Those invocation caps total 100 minutes; complete campaign cap is 150 minutes including setup/control/cleanup, one worker and 2 GiB total output. The 512-MiB process ceiling still applies. Choose only relevant cases, rotate lane order deterministically and record planned/consumed totals. No automatic retry or tuning sweep; expansion requires a new explicit question and recorded remaining budget, not a blanket new permission gate.

### 10.5 Fair persistence and resource accounting

Freeze identical logical fields, indexes, retention, acknowledgement and recovery obligations before ranking. Proposed SQL settings are 4-KiB pages, WAL, `synchronous=FULL`, 8-MiB suggested page cache, disabled mmap/autocheckpoint/auto-vacuum, file-backed temporaries and explicit checkpoints every eight batches plus final drain. Read back effective settings, including compile-time overrides. [SQLite's synchronization contract](https://www.sqlite.org/pragma.html#pragma_synchronous) distinguishes FULL/NORMAL durability in WAL; changing the setting changes the comparison. A cache setting is not a process-memory ceiling.

For later compressed lanes, freeze one admitted Zstd implementation/version, level 3, one thread and no trained dictionary; cap decoded batches at 8 MiB. Proposed Parquet layout is one row group per batch, 64-KiB target pages and a 256-KiB dictionary ceiling, with declared fallback encodings and metadata. Charge all buffers. This is a candidate recipe, not an admitted library graph.

Every acknowledged batch must survive the **declared** restart model; an unacknowledged one may be absent or committed once after reconciliation. Snapshot/external-segment adapters need explicit flush/publication barriers. Portable process termination and injected barrier failure do not establish power-loss behavior, Windows DACL enforcement or package qualification. Without equivalent demonstrated adapter semantics, report no durability-equivalent performance ranking.

Report logical retained bytes, logical file lengths, allocated filesystem bytes and peak complete-family footprint separately: database/indexes, WAL/shared memory/journals, temporaries, staging, orphans, segment metadata and simultaneous old/new migration copies. Charge candidate and harness memory separately and combined, including decode copies, codec workspace, imports/startup and allocator overhead. Record CPU/wall time, bytes read/written, flushes, commit/query latency and recovery work. Label first-open/repeat reads; a fresh process is not proof of a cold OS cache. Python/Arrow process costs cannot be presented as Rust-on-Windows application costs.

For scale only, `148,176 × 192 = 28,449,792 bytes` (about 27.13 MiB) assumes a hypothetical 192-byte payload per delivered outcome; it excludes gaps, dictionaries, indexes and publication overhead. It is not a measured file size. Earlier 32/64-MiB discussion examples remain unselected and separate from the existing diagnostic budget.

### 10.6 Decision and continuation rule

Correctness, bounded work and equivalent persistence obligations precede performance ranking. Preserve failures/skips/cancellations; report every later repetition. Differences within 10% or unstable ordering justify no winner under this proposed practical screen, which is not a significance test. Raw history needs a useful question that neither Q0 nor Q-J answers before its cost is justified. Catalogue authentication/freshness, Q/H selection, loss tolerance, production budgets and pins stay with their existing OPEN decisions.

The [task](../../tasks/server-catalogue-and-statistics.md) records the next bounded preparation, owner mapping and actual review. Current Core priority and the scoped protected-storage, Windows and fuzz holds remain. The protected-storage pause is not a global prohibition on unrelated admitted portable work. This documentation closes no implementation/qualification gate and selects no new cryptographic mechanism.
