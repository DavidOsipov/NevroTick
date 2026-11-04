# Binary server catalogue, Roughtime key records and future collection tooling

**Document:** NV-CAT-BIN-001, discussion draft 0.1

**Recorded:** 4 November 2026 (Asia/Tbilisi)

**Inspected project baseline:** `7550f83292f959a6d0acbb2d396e964a6bcf4b01`

**Scope:** C remains a design proposal: no implemented release catalogue, benchmark, live-server verification or release approval. The later [offline R input stage](../../tools/server-catalogue/README.md) implements candidate collection and semantic diff under [task §8](../../tasks/server-catalogue-and-statistics.md#8-offline-publisher-collector--4-november-2026); its fixture evidence does not qualify C or the client runtime.

## 1. Direction, authority and present scope

David's latest requirements are to minimize delivered/installed size, background CPU and RAM while retaining strong security and native Windows integration; avoid JSON as the preferred **shipped catalogue** representation; accommodate hundreds of NTP/pool, NTS and Roughtime records with relevant metadata; include the Roughtime verification material in the schema now; and later implement a Python collection/build tool in the repository's tools area.

This records those requirements, not a measured format winner. The two leading candidates are generated immutable Rust tables for release-coupled data and constrained, schema-defined CBOR when a separate resource is justified. CBOR is the recommendation to investigate for that separate-resource case; neither candidate has priority from measured product evidence. Protobuf remains a conditional alternative. Exact wire field numbers, production widths/caps, crate/features, signature scheme and final representation are not selected by this draft. Upstream JSON/Markdown/HTML may be ingested by publisher tooling; that does not require shipping JSON or a Python runtime to clients.

The existing [catalogue/statistics draft](server-catalogue-and-statistics-draft.md) owns DATA-01–DATA-24 and its open decisions; the [storage tables](server-catalogue-storage-tables.md) remain its readable allocation companion. This document refines C and the R/publisher pipeline. The [existing task](../../tasks/server-catalogue-and-statistics.md) owns implementation preparation and review evidence. [Development §7](development-plan.md#7-next-executable-task-for-a-development-agent) still owns the current Core priority and actual execution/admission boundaries, including the later scoped Sandbox disposition. The storage pause and independently held artifacts/runners retain their stated scope.

Preserve the architecture's selected strict JSON IPC, machine-policy and broker-journal boundaries. A binary catalogue introduces no replacement schema engine at those boundaries. Q/H remain advisory and cannot restore authentication, clock authority, cleared conflict or consumed correction allowances. The ordinary NTP/NTS catalogue can be prepared before RGT-0; reserving complete Roughtime records now neither implements RGT-0 nor changes RGT-0 → CHR-0. The [time-assurance plan](time-assurance-integration.md) owns Roughtime-first provisional-date bootstrap, conditional reserve, full NTS and conflict behavior.

## 2. Resource model and physical allocation

The complete catalogue is distinct from the active association set. Hundreds of records do not authorize hundreds of sockets, timers, TLS states or estimator histories. Existing default-four/maximum-16 ordinary active-source and maximum-two helper bounds remain separate from catalogue cardinality and future admitted bootstrap work.

| Class | Placement and representation | Resource and security consequence |
| --- | --- | --- |
| C: immutable release catalogue | One admitted MSIX resource, or generated read-only data in the installed engine image | Resolve actual package identity/location; validate once before immutable publication. Do not duplicate the entire catalogue into each user's writable directory or periodically reparse it during polling. |
| Active state | Bounded typed engine RAM, IDs and rings | Instantiate mutable protocol/quality objects only for admitted active work. Count source-index, string/key buffers, decode overlap and native work in the same resource accounting. |
| Q: advisory summaries and historical context | Protected engine-owned machine storage; existing SQLite-versus-bounded-snapshot comparison | Catalogue IDs and historical definitions remain interpretable across revisions. No UI direct database access or mandatory authority reconstructed from Q. |
| H: optional justified detail | Existing bounded batch/segment candidates | No mandatory detailed archive is introduced. Optional-history deletion cannot clear safety conflicts; TA-W02 retains the required cross-restart minimum decision. |
| E: policy, journals, trusted bounds and secrets | Existing separate authorized owners and lifecycle contracts | Catalogue refresh/repair/reset cannot delete or renew E authority. NTS secret persistence retains the qualified VBS/TPM/no-persistent-cache contract. |
| R: publisher collection/provenance | Maintainer workspace and reviewed repository inputs/evidence, outside installed runtime | Raw downloaded pages, full collection logs, Python tooling and signing secrets are not client catalogue contents. Keep only client-required public provenance. |

**Leap-policy integration, 4 November 2026:** [NV-LP-001](leap-second-policy.md#3-knowledge-and-distribution-contract)
owns the separate public E dataset: reviewed history, future coverage and the
effective UTC rule, compiled immutably into the same authenticated complete
release. Its revision/digest and rule provenance do not select C's encoding or
make optional Q/H or secret-cache availability a dependency.

Do not put the sole mandatory restoration record in automatically disposable service state. Actual paths, owners, replacement rights and removal/Reset recovery remain under the existing architecture/MSIX contracts. Choosing ProgramData alone is no enforcement evidence.

As arithmetic only, 300 records at an assumed 512 bytes each equal 153,600 bytes (150 KiB), excluding indexes and runtime overhead. No measured average or maximum is implied. Catalogue caps are unset. The existing 32-MiB target is combined engine/broker **steady-state private bytes**, with peaks reported separately; the 20-MiB compressed complete-MSIX target includes all required payloads. Both remain unmeasured product goals here. Q's separate proposed disk allowance is not another RAM budget, and private bytes alone do not account for mapped/image residency.

Compression is a separate decision. Start the catalogue comparison without an added compression dependency; measure complete package/image costs before introducing one. Live state is not compressed by default. Moving bytes into mapped/image pages does not eliminate their physical-memory cost.

## 3. Representation candidates

Encoding, installed placement and runtime access are separate choices. An embedded binary blob still needs its bounded decoding/admission path; generated typed tables can avoid that decoder without removing policy, identity or lifecycle checks. A separate file is not required merely because the delivered representation is binary.

| Candidate | Purpose and advantage | Required contrary case |
| --- | --- | --- |
| Constrained CBOR resource | Explicit binary artifact; integer field labels, byte strings, bounded type-directed loading and a documented schema | Decoder/code footprint, input-plus-output overlap and allocation strategy may exceed generated tables. Generic CBOR is not automatically compact or strict. |
| Generated immutable Rust tables | Release-bound data can avoid runtime deserialization and per-record owned strings/vectors | Alignment, pointers, relocation and image layout may increase size/residency. Compile-time initialization does not validate server policy or establish fresh network evidence. |
| Protobuf | Useful schema tooling and independently evolving consumers where actually needed | Compare a real Rust implementation and emitted code; ordinary parser defaults/merging do not meet every strict-admission rule. |
| JSON reference input | Optional publisher/source-review or comparison representation | Not the preferred shipped catalogue after the owner clarification. Its use by an upstream list does not select it for NevroTick delivery. |
| Parquet/SQLite catalogue | No present C-specific requirement justifies these mechanisms | Q/H have different workloads. Do not add catalogue query/analytics machinery solely because the list contains hundreds of entries. |

CBOR's integer labels and native byte strings allow a compact profile [C1]. Use CDDL to describe accepted structure, with actual Rust admission enforcing it [C2]. Suitable Rust statics can be compile-time initialized and read-only [C3]. These source facts establish feasibility, not a numerical ranking.

For a CBOR candidate, specify one deterministic encoding profile, definite lengths, shortest accepted integer/length encodings, an explicit map-key ordering, a closed type set, unique keys and no trailing data. Integer labels must have one allowed CBOR type; a text label or floating-point equivalent is not the same admitted field. Define absent/null separately, validate UTF-8, and reject unselected tags, bignums, floats and generic extension containers. Unknown fields/features are refused unless that exact schema declares a bounded, semantics-preserving extension rule. Detect duplicates before map insertion loses them. No unrestricted generic value tree before size checks. Cap source bytes, records, references, keys, strings, nesting, intermediate allocations and total work; use checked arithmetic and validate every index. A logical field table or CDDL check is not an executable admission path.

For generated tables, require deterministic input ordering and stable IDs, reproducible generator/tool identities, no arbitrary source evaluation, checked generated values and the same endpoint/key/domain admission obligations. Native struct layout is not an external wire format; this proposal selects no unsafe byte-to-struct reinterpretation, unchecked offsets or packed-layout shortcut. Avoid shipping both source and generated catalogue accidentally. Build-time validation can discharge immutable pure invariants only where the installed-image trust contract preserves them; current policy/context checks remain at consumption.

Authenticate exact artifact bytes under the selected release contract. A separate project signature is still OPEN-CAT-01, not automatically selected. If introduced, cover the format/version/domain context and payload without decode/re-encode verification. Freeze the length/digest/signature coverage so no field circularly hashes itself or remains unintentionally uncovered; expected digests and verification keys come from the admitted release authority, not from an untrusted colocated field. Protobuf noncanonical serialization does not prevent signing a particular file [C4]. Determinism assists reproducibility; it supplies neither authenticity nor freshness.

## 4. Logical schema before field-number freeze

These are proposed logical records, not finalized CDDL, Rust types or database tables. Names below are documentation labels; they need not be repeated as strings in the binary payload. Assign stable identifiers independently of array positions. Freeze exact types, bounds and compatibility before implementation.

### 4.1 Records and their authority

| Record | Required meaning | Admission and lifecycle |
| --- | --- | --- |
| Catalogue envelope | Format/schema identity; catalogue namespace and lineage/revision; release binding; required schema features and declared data profiles; exact payload identity; declared collection counts; provenance reference | Schema version, ordered data revision, application version and runtime generation have different meanings. Validate declared counts against bounded decoded contents; do not allocate from them unchecked. Unknown incompatible schema semantics reject the candidate atomically. A colocated digest alone is no authentication. |
| Source | Stable source ID; fixed-service or dynamic-pool kind; nominal operator/dependence references or explicit unknown; service-definition references; usage and provenance references | A display name, hostname or current IP is not the permanent identity. One source can expose several protocol services without gaining independent votes. A pool alias does not declare the operators of all future DNS answers. |
| Operator and dependence metadata | Stable nominal operator ID; typed relationships, evidence and uncertainty | Common administration, upstream time, site/hosting and implementation are distinct relationships. Unknown stays explicit; neither geography nor a shared key proves independent operators. Selection authority remains with its existing owner. |
| Geography and time behavior | Required country/region value or explicit unknown at the appropriate source/endpoint scope; time-scale/leap/smear profile with operator/configuration provenance and compatibility status; separate announcement-behavior claim | Keep operator domicile, advertised service location, anycast scope and IP-derived estimates distinct. V1 requires admitted unsmeared-UTC provenance; unknown announcement behavior does not mean unknown scale. LI=0 remains an ordinary candidate even if every source is silent, but proves neither scale nor absence of an event. Preserve DATA-03/05 and NV-LP-001; finer location stays optional. |
| Leap-policy reference | Exact admitted release-bound leap-dataset revision/digest and effective-rule revision/provenance needed to interpret time profiles and retained observations | Reference the separate compiled domain dataset, not a mutable C/Q/H copy. Keep historical coverage, finite future coverage, `Unknown` and `ContinuousFrom` distinct under NV-LP-001 and DATA-02/09. |
| Endpoint | Stable endpoint ID; exact service/protocol role; transport; typed normalized host/address; explicit port; profile reference | Separate DNS identity, literal address and optional hints. An endpoint cannot mix protocol fields merely because their primitive types match. Its destination remains subject to the current resolver/network policy. |
| Plain NTP service | Fixed operator endpoint versus dynamic pool alias; explicit permitted query-role policy reference; usage/vendor-zone provenance | Ordinary plain synchronization, helper hints and a future reserve are different policy roles. DNS results are observations, not permanent trusted pool membership. Listing the service enables none of these roles by itself. |
| NTS service | Expected NTS-KE authentication identity; KE endpoint; claimed versus admitted support; allowed negotiated-destination policy reference | Keep the KE connection target, verified identity, negotiated NTP destination/port and actual peer separate. Actual KE results belong to the association owner. Never infer an NTP port from the KE port; certificate observations are not pins [C6]. |
| Protocol profile | Stable immutable data-profile ID bound to a reviewed specification/revision, wire discriminator, transport/framing, algorithm and time-conversion semantics | Reference a recognized declaration; qualified acquisition support is a separate runtime capability. A catalogue cannot redefine signature contexts, units or parsers through arbitrary strings. Advertisement, qualification and observed negotiation remain separate evidence. |
| Roughtime root and authorization binding | Key ID and algorithm/exact public bytes; separate binding identity for authorized source, endpoint, profile and lifecycle scope; provenance | Required for a usable Roughtime record from the first schema. No rootless production path or implicit cross-product of unrelated keys/endpoints/profiles. Details in §5. |
| Public provenance and use constraints | Client-needed evidence identifier/digest, concise origin/status references, and any admitted machine-readable use constraint the runtime must enforce | Detailed raw sources and retrieval logs remain R. Do not omit required polling/use restrictions merely because their full evidence is publisher-side; unresolved mandatory restrictions prevent admission. Publication, operator statements, probes and release approval are separate axes. |
| Optional address hint | Address family/bytes and explicit origin/applicability | Never replaces TLS hostname/key identity, TTL/refresh policy or destination admission. A catalogue date does not make a DNS answer current. |
| Policy overlay/reference | Stable namespace-qualified target binding to separately authorized local overrides | Package changes do not silently overwrite authorized machine policy. Local sources cannot collide with release IDs; NTS pins, disable decisions and mandatory restrictions retain E ownership. A missing target needs the declared reconciliation rule, not reassignment to a similarly named source. |

Each service definition is an explicit tagged variant: plain NTP, NTS or Roughtime, with only its own required fields. A source may reference several variants; a variant must not be inferred from a port, a key's presence or a missing TLS field. Reject mixed variants, unknown discriminators, missing required fields and invalid supplied optional values. The client never turns a malformed NTS variant into plain NTP.

### 4.2 Stable identities, references and normalization

IDs are unique within their declared namespace and record kind, immutable in meaning and never reused for an unrelated entity. Freeze their exact representation/allocation before implementation; no hash width or numeric capacity is selected here. A changed display label preserves identity. An unrelated service/operator gets a new source identity. A reviewed root rotation may retain the logical source but gets a distinct key identity and applicability context; history does not automatically transfer to the new key. Changes to an endpoint's transport, canonical target, port or role require a new endpoint ID or an explicitly versioned endpoint identity under one frozen transition rule. Normal DNS address churn does not rewrite the configured DNS identity. Alias/merge/split decisions retain provenance and do not silently collapse operator groups or historic measurements.

Compact array indices may be internal to one fully admitted catalogue, but are not stable IDs. Validate uniqueness before building maps, reference kind and existence, source ownership, exact endpoint/profile/key relationships and all index/length arithmetic. Derived lookup tables cannot conceal a normalization collision. Cross-release Q/H references retain catalogue lineage/revision plus stable IDs and the relevant historical definitions; a current array position cannot interpret an old observation. Tombstones and ID allocation history may remain in R or bounded historical dictionaries rather than bloating every client copy.

| Value | Required normalization/identity contract before the wire freeze |
| --- | --- |
| DNS host | One declared absolute DNS comparison form with pinned ASCII/IDNA handling. Resolve with absolute-name semantics, not local search suffixes. Case and trailing-root-dot equivalents follow that one rule; unsupported Unicode forms, embedded URL/path/userinfo syntax and ambiguous input are rejected. Preserve source spelling in R when needed for evidence. Exact IDNA policy remains open. |
| Literal address | Distinct IPv4/IPv6 variants with exact address bytes; textual aliases use the shared checked parser. Brackets belong to a source host/port grammar, not address bytes. Never split IPv6 at an arbitrary colon. Zone identifiers and address scope need the destination owner's explicit support; the RFC Roughtime list does not permit zones [C5]. |
| Port and transport | Store the effective port and explicit transport separately. An adapter may supply a documented protocol default only with recorded provenance; zero, omitted and default are different states. Source syntactic range and a usable admitted remote destination are separate checks. |
| Key and profile | Compare decoded algorithm/key material and immutable profile identity, not Base64 spelling, hostname, fingerprint display or array position. Verify material equality on a reused key ID; a digest match never substitutes for the declared ID-collision rule. |
| Sets and ordered choices | Deduplicate/sort only collections declared unordered. Preserve explicit priority, negotiation order and transition relationships. Canonical output ordering must not change their meaning. |

Preserve absence, zero, unknown, not applicable and deliberately omitted values where they mean different things. Freeze exact integer units/ranges and reject lossy conversions; no floating intermediate or platform-sized persisted ID. Time fields declare epoch, unit and time basis; publisher retrieval time is not authenticated client time. A diagnostic URL is inert provenance, not a fetch instruction. Resource pressure, parse failure and missing evidence are not a false quality accusation.

### 4.3 Source records do not construct live authority

Catalogue membership, reviewed preset membership, configured acquisition membership and currently active work have different meanings. OPEN-CAT-02, R-04 and OD-03 must identify where the reviewed preset is represented (C or separately authorized policy), how it binds to source IDs, and whether automatic replacement is in scope. No default or replacement behavior is selected here. If replacement is later selected, freeze its eligible population, trigger, stable selection/reselection rule, finite traversal and provider-budget continuity before implementation. Catalogue order, an unused valid entry, low latency or an empty active slot grants no selection or traversal permission; the separate Khronos reserve supplies no implicit sampling guarantee for this path.

The initial production artifact contains only reviewed, explicitly admitted service/profile/binding combinations whose data semantics its reader recognizes. Conflicting or ambiguous source claims stay in R. Recognition of a Roughtime record is separate from RGT-0 implementation, verifier qualification or policy activation. The proposed pre-RGT reader may validate recognized Roughtime metadata and root bindings, retain them inertly and continue ordinary NTP/NTS acquisition; it cannot select those records or create authenticated Roughtime evidence without the existing runtime gates. Metadata recognition requires no running Roughtime exchange or prematurely adopted crypto dependency.

Unknown required schema features, record kinds or data-profile meanings still reject the candidate. If an early reader cannot represent Roughtime records at all, the builder must produce an explicitly reviewed compatible NTP/NTS projection rather than let that reader silently drop them. The projection has its own bound artifact/export identity, declared contents, counts, references and semantic diff; retain complete Roughtime source material in the approved publisher corpus. OPEN-CAT-01/02 owns the concrete pre-RGT reader/export rule before implementation. A policy requesting an unavailable acquisition capability receives the existing explicit unavailable/Blocked result for that work; absence of RGT support neither weakens that policy nor makes RGT-0 a new prerequisite for independent ordinary NTP/NTS work.

Bounded decoding produces inert records. A single private checked construction path enforces the shared domain rules and returns an immutable catalogue view; consumers receive that result, not the original record. Generated tables must discharge the same invariants through their trusted build/image path. At use, the engine applies current policy, destination, key/revision, role and budget guards; after asynchronous resolution or protocol work it checks the relevant generations again. Successful catalogue admission supplies configuration, never authenticated time evidence, operator independence or clock-write authority. CHECK-25 owns the constructor/consumer distinction, with DATA-01–09 and the existing protocol/time-assurance owners.

## 5. Roughtime verification material is required now

### 5.1 Root keys and protocol profiles

For RFC 10049 v1, the trust input is a configured **32-byte Ed25519 long-term public key**. Responses carry a signed delegation and an online public key; that online key is not a catalogue trust root. The v1 SRV value is derived from the root key as the first 32 bytes of SHA-512 over `0xff || public_key` [C5]. It is a protocol selector, distinct from the catalogue key ID and a diagnostic fingerprint. Preserve protocol-specific verification at the RGT owner.

| Roughtime field | Proposed requirement |
| --- | --- |
| `key_id` | Stable identity for one algorithm/key-material pair, never an array position or mutable alias for the current key. Multiple authorized uses of the same material reference it without creating additional independent votes. |
| `public_key_algorithm` and `public_key_bytes` | Explicit algorithm and exact bytes; v1 Ed25519 requires 32 bytes. Source Base64 is strictly decoded during import; the shipped record uses bytes. Length checks alone establish neither mathematical key validity nor operator authorization. The selected verifier retains its algorithm-specific key/signature checks. |
| `protocol_profile_id` | Reference an immutable recognized data profile, distinguishing RFC-v1, specifically identified drafts, legacy Google and unsupported experiments. Runtime use additionally requires its qualified verifier/capability. An ambiguous `IETF-Roughtime` label, a port or the same 32-byte key length cannot select one. |
| `binding_id`, `source_id` and `endpoint_id` | Authorize an exact key/source/endpoint/profile combination with explicit transport and scope. Multiple endpoints or roots are a bounded set of such bindings, not an implicit Cartesian product. |
| `provenance_refs` | Field-level origin for key, endpoint and profile claims, pinned source identity where available, observation and reviewer disposition. Merged evidence retains disagreements and does not attribute a community claim to an operator. |
| Curation state versus binding lifecycle | Candidate/rejected/admitted are publisher review dispositions. Active, approved-next, overlap, retired and revoked describe the authorized binding transition. Approved-next is not usable merely because its bytes ship; exact transition conditions remain to be frozen. |
| `replacement_refs` and applicability | Explicit predecessor/successor binding identities, authorized change evidence and transition scope. Reject missing, cross-source or cyclic replacement relations unless the owning reviewed transition explicitly defines their meaning. Never overwrite old key bytes under the old key ID. |
| Generation binding | Keep immutable catalogue lineage/revision separate from the current effective key-policy/runtime generation. Results bind to the exact root, profile, endpoint and generations they used; key withdrawal or policy change invalidates affected results under TA-W02/06. |
| Optional diagnostic fingerprint | Derived from algorithm/key bytes for comparison; not an independent trust anchor. Recompute or verify it on import. |

The proposed RFC-v1 profile binds wire version `1`, the RFC's framing/signature contexts, little-endian unsigned 64-bit UTC timestamps in seconds since the Unix epoch, and nonzero unsigned 32-bit radius values in seconds [C5]. Those are protocol facts, not catalogue integer-width choices or adoption of the entire wire date range; TA-W02 still freezes NevroTick's supported interval and checked conversions. Draft/legacy profiles need their own pinned units, epoch, framing and context contract; a shared timestamp parser or guessed microsecond conversion cannot establish compatibility. Catalogue fields reference those reviewed contracts rather than redefining cryptographic parameters.

Never provision a replacement root from an unauthenticated response or ordinary DNS TXT result. A known root authenticates statements within its proof scope; it does not by itself establish UTC correctness or independence. Rotation authorization, emergency retirement, root compromise, old-release rollback and missing-key recovery remain OPEN-CAT-01/02/03 and TA-W02/06 work.

The publisher must represent approved old/new overlap with bounded cardinality and exact binding scope, without counting two keys of one operator as two sources. An observed second key is a conflicting candidate until the transition is reviewed. Withdrawal is an explicit decision, not an inference from a missing list row or failed fetch. Define activation/retirement evidence that works with an unreliable client clock; unqualified wall-time windows cannot activate a successor. Clearing Q/H cannot reactivate a withdrawn binding. Missing usable roots makes the Roughtime branch unavailable under the existing time-assurance policy; no rootless verifier or automatic legacy-profile retry supplies replacement evidence. The existing owner still controls delegated-key validity, request binding, response-version acceptance, uncertainty, conflicts and bootstrap failure.

### 5.2 Initial public-key evidence inventory — 4 November 2026

The following are **documentary candidates, not an approved shipped roster**. All 13 Ed25519 rows come from the exact [ecosystem snapshot][C12] (Git blob `cd9eb5043fd7ff77845333ffe97b8318303ff94e`); current fetched bytes matched that pinned blob. Each Base64 value was strictly decoded and checked to contain 32 bytes. No network exchange, signature verification, availability/accuracy test or operator-independence qualification was performed.

The two RFC-v1 entries were also checked against their operators' pages; Cloudflare and roughtime.se have their own narrower documentary confirmations. Other rows remain list-derived in this check. Base64 below is a readable documentation spelling of **public** material, not the proposed runtime encoding.

| Nominal operator | Published endpoint | Published long-term public key, Base64 | Profile/transport evidence | Provenance/status |
| --- | --- | --- | --- | --- |
| Tanner Ryan | `time.txryan.com:5319` | `iBVjxg/1j7y1+kQUTBYdTabxCppesU/07D4PMDJk2WA=` | RFC 10049 v1 advertised; UDP/TCP :5319 | Operator page + pinned list [C8], [C12] |
| SIDN Labs / TimeNL | `rough.time.nl:5319` | `v2CievhgKsxzlWPwIkYFUXeA51Akhkv5uhJCj1/kbiY=` | RFC 10049 v1 advertised; UDP/TCP :5319 on operator page | Operator page + pinned list [C7], [C12] |
| Cloudflare | `roughtime.cloudflare.com:2003` | `0GD7c3yP8xEc4Zl2zeuN2SlLvDVVocjsPSL8/Rl/7zg=` | UDP :2003; exact current RFC-v1 support unresolved | Operator endpoint/key + ambiguous list profile [C9], [C12] |
| roughtime.se / STUPI-hosted | `roughtime.se:2002` | `S3AzfZJ5CjSdkJ21ZJGbxqdYP/SoE8fXKY0+aicsehI=` | Operator advertises draft-19; UDP in list | Operator page + pinned list [C11], [C12] |
| Netnod | `sth1.roughtime.netnod.se:2002` | `9l1JN4HakGnG44yyqyNNCb0HN0XfsysBbnl/kbZoZDc=` | IETF-Roughtime label; exact profile unresolved | Pinned community list only in this check [C12] |
| Netnod | `sth2.roughtime.netnod.se:2002` | `T/xxX4ERUBAOpt64Z8phWamKsASZxJ0VWuiPm3GS/8g=` | IETF-Roughtime label; exact profile unresolved | Pinned community list only in this check [C12] |
| teax.dev | `time.teax.dev:2002` | `84pMADvKUcSOq5RNbVRjVrjiU16Dxo2XV2Qkm+4DRTg=` | IETF-Roughtime label; exact profile unresolved | Pinned community list only in this check [C12] |
| Sturdy Statistics | `roughtime.sturdystatistics.com:2002` | `NqIjwLopQn6yQChtE21Mb97dAbAPe5UOuTa0tOakgD8=` | IETF-Roughtime label; exact profile unresolved | Pinned community list only in this check [C12] |
| Inutile | `fr.ntp.inutile.pro:2002` | `caVakKYAW26s2mDSfY21nI1ctN/w3kPBL8oGRAqLxRg=` | IETF-Roughtime label; exact profile unresolved | Pinned community list only in this check [C12] |
| Inutile | `es.ntp.inutile.pro:2002` | `/z+14WcSjMU4evfyQoMKU8GGa5KSF9sW69tu3c+KBXs=` | IETF-Roughtime label; exact profile unresolved | Pinned community list only in this check [C12] |
| Inutile | `uk.ntp.inutile.pro:2002` | `bUjS5xMA6FBRQzhriEs71uyQu0+JDf/TeZ0IFNTdJZ0=` | IETF-Roughtime label; exact profile unresolved | Pinned community list only in this check [C12] |
| Inutile | `us.ntp.inutile.pro:2002` | `V7hnCRDc7iR2uv0VrmGu37Yn3ovdHjcDW8oiFGC7hh4=` | IETF-Roughtime label; exact profile unresolved | Pinned community list only in this check [C12] |
| Alastyr | `roughtime.alastyr.com:2002` | `/2Al59PlwUTJHas/VS+FN6BhO9ify/wIjoH2Ezv4V7w=` | IETF-Roughtime label; exact profile unresolved | Pinned community list only in this check [C12] |

TimeNL's operator page additionally advertises TCP on 5319, although the pinned community row lists UDP only. Retain both observations and their sources; do not let a narrower community row erase operator information automatically [C7], [C12]. The RFC service endpoints are separately documented from compatibility port 2002 [C7], [C8].

**Negative import control:** Cloudflare's old `roughtime.cloudflare.com:2002` key `gD63hSj3ScS+wuOeGrubXlq35N1c5Lby/S+T7MNTjxo=` is listed as deprecated on 30 June 2024. Preserve the deprecation evidence; never resurrect it by merging an older list [C10]. Cloudflare describes its current service as beta and says its key may change [C9].

The pinned ecosystem also contains an experimental ML-DSA-44 entry. It is excluded from the 13 Ed25519 rows and supplies an unsupported-algorithm/size control, not an adopted post-quantum feature. This inventory is neither exhaustive nor a count of independent usable RFC-v1 operators. Recheck sources when building a release.

## 6. Future Python collection and build tool

**Location:** [`tools/server-catalogue/build_catalogue.py`](../../tools/server-catalogue/build_catalogue.py), using the existing lowercase `tools/` directory. The [R contract and CLI](../../tools/server-catalogue/README.md) now implement the offline input stage described in §6.4. Approval, C generation and package integration remain future work. This tool introduces no scheduled crawler, extra dependency or client Python runtime.

The tool belongs to R/publisher preparation. It collects candidate NTP/pool, NTS and Roughtime definitions, normalizes them, produces a reviewable semantic diff and eventually emits the selected binary resource or generated tables from approved inputs. Collection is not default admission, signing or publication.

The collector preserves source-scale and announcement claims with their evidence;
it cannot promote LI, agreement or a fetched list into leap knowledge. Offline
leap ingestion remains under [S1.4](../../tasks/s1.4-leap-policy.md), from reviewed
pinned inputs. This adds no runtime downloader, implicit network build/probe,
new trust root or clock authority; §6.2's explicitly bounded publisher fetch stays separate.

### 6.1 Source adapters

| Family | Initial input candidates | Required handling |
| --- | --- | --- |
| NTS | [jauderho/nts-servers][C13], then linked operator declarations | Community list discovers candidates. Preserve operators, names/roles/ports, source version and experimental/usage status; do not manufacture missing identity or a pin from a scraped certificate. |
| NTP/pools | [Companion public-NTP directory][C15]; operator-published service lists; official Pool documentation and an assigned vendor-zone configuration | The first adapter reads the directory's pinned generated Markdown, retaining raw AS/owner/notes as claims. A public DNS answer is not a harvestable permanent pool roster. Preserve dynamic aliases and usage constraints; project-default Pool use needs its own vendor arrangement [C14]. |
| Roughtime | Pinned ecosystem lists and operator publications in §5 | Decode key bytes strictly, preserve exact profile evidence and key/endpoint pairings, distinguish unsupported candidates and deprecations; never infer profile from port alone. |

RFC 10049's common JSON list uses an integer `version` for the highest advertised protocol version; that is not a catalogue schema version or an enumeration of every supported version [C5]. Preserve it as a source claim. A source-specific adapter may accept a documented quoted-version variant, but must label that compatibility rule rather than call it the RFC spelling. Boolean `true`, floating `1.0`, exponent notation and arbitrary strings must not become integer `1` by Python coercion. An ambiguous `IETF-Roughtime` label remains unresolved; a higher advertised version does not establish support for every lower version. The raw field and source identity remain available in R.

Adapter output carries separate input-schema/adapter identity, normalized candidate identity, exact source claims and their provenance, and review disposition. It cannot manufacture an admitted-profile marker or resolve a disagreement by copying the most recent row. The collector's upstream JSON is independent of NevroTick's future binary schema. Any upstream `sources`, `reports` or documentation URLs remain evidence data unless separately admitted for the specific fetch operation; they grant no recursive fetch or report-submission authority.

### 6.2 Bounded pipeline and publication boundary

This is the complete publisher pipeline contract. §6.4 identifies the implemented local-input subset; acquisition, approval, artifact generation and consumer/package admission below remain future stages.

1. **Acquire or reuse pinned source artifacts.** Default parsing/build tests use saved fixtures. Optional fetch is explicit and bounded by allowlisted source origin, time, response size, redirects, concurrency and total work. Preserve retrieval URL/final origin, source commit/blob or digest, retrieval time and applicable usage/licence evidence. Validate HTTPS normally; a broken maintainer clock is no reason to disable TLS validation.
2. **Parse as untrusted data.** No upstream code execution, shell interpolation, unsafe deserialization or fetching arbitrary URLs embedded in candidate records. Bound structures before materializing collections; reject duplicate object members before Python dictionaries or other normalization discard the evidence. Require complete supported source shapes/types and distinguish allowed ancillary fields from adapter/schema drift. Network-time probes are a separately scoped optional activity, not a side effect of parsing a list.
3. **Normalize without hiding conflicts.** Use §4's host/address/port, stable-ID, presence and cross-reference contracts. Strict Base64 checks include the declared alphabet/padding and decoded length; verify the accepted canonical spelling or reject noncanonical aliases under the adapter's frozen rule. Keep raw evidence in R. Coalesce only semantically equivalent claims with all provenance retained. Same endpoint with conflicting keys/profiles stays unresolved unless a separately approved binding transition covers it; fetching both roots does not approve overlap.
4. **Compare with approved input.** Bind the diff to the exact predecessor and candidate. Show additions/removals, key rotations, protocol/destination/time-policy changes, use restrictions, operator/dependence claims, experimental/retired status, normalization collisions and lost provenance. Source fetch failure is not retirement evidence. Empty/partial results cannot silently replace the last approved catalogue; any deliberately reduced export is a reviewed subset, with counts/references and availability implications recomputed.
5. **Review and approve data through the existing owner.** Bind accepted records, exclusions and dispositions to the exact normalized snapshot, schema and relevant source evidence. Approval of a hostname is not approval of every future key/profile at that name. The script does not turn successful parsing, probes or HTTPS retrieval into security approval, modify NTS-only/default policy or settle OD-03.
6. **Build deterministically from approved snapshots.** Refuse changed inputs after review; identical approved records/schema/export profile/toolchain yield identical payload bytes. Keep volatile fetch logs/build wall time outside the deterministic payload unless deliberately versioned input. Generated Rust must render typed data with safe literal encoding, never splice upstream text as executable syntax. Emit the semantic report, input/generator identities, exact artifact digest/length, required schema features and declared data profiles; do not self-approve the output.
7. **Admit through the real consumer and package workflow.** Python checks are preliminary, not a second authority that bypasses the shared Rust contract. The actual Rust loader or trusted generated-data construction must exercise complete decoding, reference/profile invariants and useful accepted/refused controls. Stage the complete candidate before publication; failed validation or interrupted output leaves no apparently complete replacement. Actual package/release signing retains its current owner. The tool never receives client secrets or private signing keys by default.

A future collector may accept upstream JSON while delivering binary output. Do not ship Python, raw scrape caches or all source pages in MSIX merely because the builder used them. Do not add automatic client updating/TUF or a background crawler through this work item.

### 6.3 Proposed outputs and acceptance controls

Before each dependent stage, freeze its output location/schema and update policy. The R input stage now has that contract in the tool README; C and approval artifacts remain unimplemented. Full-pipeline proposed outputs: normalized candidate records, provenance manifest, rejected/conflicting-record report, semantic diff, exact reviewed input snapshot and deterministic artifact built from that snapshot. Candidate, admitted input and packaged artifact are distinct states with bound identities. Fetch/parse drift reports a partial/failing source result instead of a successful empty list. They may initially share one tool and a small fixture set; no additional daemon or database is required.

Required paired controls include: a valid published 32-byte root and a truncated/malformed encoding; valid explicit v1 and ambiguous draft labels or coercible wrong types; duplicate consistent claims versus conflicting IDs/material; correctly parsed bracketed IPv6 versus ambiguous host/port; a legitimate operator declaration versus an unsupported experimental service; approved root overlap versus an unreviewed second key; the retired Cloudflare tuple versus its documented successor; source outage versus explicit retirement; valid old catalogue versus incompatible schema or unknown profile; complete replacement versus interrupted staging; and a valid operator-approved default versus an unapproved Pool alias.

Add independent controls for variant-field substitution, dangling/cross-source references, reordered tables with unchanged stable IDs, normalization collisions, absent/zero/unknown time policy, local-ID collision, and a changed candidate after its review. Pair recognized inert Roughtime records with an unknown data-profile rejection, and prove that disabling/unqualifying the RGT runtime leaves ordinary NTP/NTS behavior within its existing policy. Confirm that a valid neighboring catalogue remains usable and a failed candidate has no publication or network effect. These are future checks at the actual source adapter and Rust consumer, not claims of executed tests or proof from a shared serializer round trip.

Probe results, if later added, record protocol/key binding, result stage, observation time/basis and failure reason. A successful response is evidence for that observation only; it is neither a lifetime availability promise nor a new root-authority grant.

### 6.4 Implemented offline R input stage

The [tool README](../../tools/server-catalogue/README.md) is the concrete version-1 manifest, candidate, comparison and failure contract. The [task receipt](../../tasks/server-catalogue-and-statistics.md#8-offline-publisher-collector--4-november-2026) records exact candidates, host-fixture results and review limits. These R formats do not freeze C's wire schema or stable product IDs.

| Implemented boundary | Meaning and remaining limit |
| --- | --- |
| Three closed source adapters | NTS and companion NTP generated Markdown, plus ecosystem Roughtime JSON, from explicit local snapshots. SHA-256, expected length and optional Git-blob checks precede parsing. No fetch, DNS resolution or protocol probe. |
| Unapproved candidate snapshot | Preserve original source claims, provenance, unknown permission/independence/scale, exact Roughtime public-key bytes and profile/address scope. Known unsupported experiments remain visible; a complete candidate means source parsing completed, not source admission. |
| Semantic comparison | Revalidate exact input snapshots and all derived fields. Preserve duplicate multiplicity and provenance changes; failed, missing or incompatible sources block the relevant comparison rather than inventing retirement. R claim locators and diff joins are not permanent runtime identities. |
| Finite local work and output | Explicit bytes/count/depth/string/output caps; no-clobber publication into an owned directory. The tested POSIX link boundary and cleanup outcome do not establish Windows durability or a hostile-filesystem sandbox. |
| Publisher-only JSON | Versioned manifests, candidates and diffs are review evidence R. The client's preferred binary C, signatures, approval binding, Rust admission and release delivery remain with their existing owners. |

Source-scale and announcement text remains raw evidence. This adapter deliberately reports normalized time behavior as unqualified; it cannot admit the unsmeared-UTC profile required by NV-LP-001. A later explicit operator-evidence/approval stage must supply that provenance. Collection also does not apply deprecations or rotations as authority: a known older key remains an unapproved claim until its lifecycle evidence is reviewed.

## 7. Compatibility, failure and release lifecycle

Catalogue C is currently release-bound. Schema version describes representation and meaning; catalogue revision orders reviewed data within its lineage/export identity; application version identifies the package; runtime generation identifies one admitted in-process view. They are not substitutes for one another. A revision/export identity cannot label conflicting semantic contents. Exact artifact bytes have their own release-bound digest; a matching revision number is not an authenticity check.

Before dependent implementation, record the following combinations as supported, deliberately unsupported or unverified, with concrete fixtures and an independently expected result. This is a compatibility obligation, not a claim that a reader or migration exists.

| Combination | Required behavior |
| --- | --- |
| Same recognized schema, newer reviewed data | Admit only after complete integrity, structure, reference, profile and current-policy checks; publish one coherent view. Same identity with conflicting content is refused, never last-writer-wins. |
| Old reader, new schema/required semantics | Reject atomically unless that exact combination has a reviewed compatible encoding/projection. An added optional field is not automatically compatible with a strict old reader. |
| New reader, retained old schema/data | Use an explicitly supported bounded decoder/migration through the same checked construction path, or report unsupported. Do not manufacture missing keys, units, permissions, provenance or fresh ages from defaults. |
| Recognized Roughtime data, acquisition disabled or unqualified | Retain admitted records as inert configuration under §4.3. No network use or authenticated evidence follows; a requested unavailable branch reports its actual capability/policy state. |
| Old process, replaced package/catalogue | Keep each consumer's data, indices, key bindings and backing storage tied to its own admitted view. Recheck current effective policy/generation at use; no cross-generation lookup or stale-result publication. |
| Older application/package restored | Apply the separately frozen rollback/freshness and mandatory-policy compatibility rules, with the [release-policy correction procedure](../release-policy.md#6-mandatory-state-compatibility-and-owned-recovery). A valid old package/signature cannot remove a current key withdrawal, lower a protected catalogue revision floor or renew consumed authority. Catalogue enforcement remains OPEN-CAT-03 work. |
| Q/H referring to an older catalogue | Retain bounded historical definitions and exact contextual identities, or mark interpretation unavailable under the declared retention rule. Never reinterpret a stable ID through the current table merely to produce a plausible label/score. |

Old and new processes may coexist during update. Stage and admit the complete replacement before publication, account for concurrent old/new backing storage within the resource budget, and retire it only after its actual consumers finish. Immutable borrowing or generated tables must preserve that lifetime; dropping a loader task does not establish retirement. Q/H historical migration remains independent of installing a new C. No hot catalogue updater is introduced by this lifecycle contract.

An invalid initial catalogue produces an explicit source-catalogue failure under current acquisition policy. It must not silently select plain NTP, accept unknown keys, create a fresh policy or clear mandatory conflicts. A failed replacement preserves the prior valid in-process generation where its trust/lifecycle contract still permits use; prior validity is not eternal eligibility.

A badly wrong clock makes wall-time-only catalogue expiry circular. Authenticity, catalogue freshness, key retirement and whole-package rollback require separately defined evidence and recovery. OPEN-CAT-01 owns format/trust-root strategy, OPEN-CAT-02 owns roster/profile/cold-resolution admission, and OPEN-CAT-03 owns freshness/withdrawal/rollback, alongside time assurance. Do not introduce an arbitrary maximum age or assume a valid signature prevents replay.

The [release policy](../release-policy.md) now owns support/withdrawal records and
the operational path to a compatible complete-package correction. A published
withdrawal does not prove that delayed/offline clients learned it, and reverting
code cannot erase mandatory authority. This routing selects no catalogue update
channel, runtime denial mechanism or new support commitment.

Unknown future leap coverage in an otherwise valid supported release is not an
invalid catalogue or a global tracking veto. NV-LP-001 owns all-date-hypothesis
admission, bounded possible-month-end guards, contradictory-LI observation
exclusion and fresh re-entry; provisional date acquisition must remain possible
without first obtaining discipline-eligible samples.

For optional state, distinguish safe discard from required historical interpretation. A new catalogue must not reinterpret an old source ID, transfer its score to a different operator/key, or reset enforced restrictions. Retired dictionaries can be reclaimed only after their retained references are handled under the Q/H contract.

## 8. Open questions and smallest subsequent work

These are scoped preparation gaps, not new blockers for current Core development. The existing task assigns their resolution before dependent C, approval or runtime work. The offline R input stage in §6.4 has its own frozen contract and bounded verification; production implementation and resource comparisons remain future work.

| Question | Existing owner / dependency | Required next evidence |
| --- | --- | --- |
| CBOR resource or generated tables; final dependency/features | OPEN-CAT-01; catalogue/build owner | One representative approved corpus, same semantics, complete image/package and startup/RAM accounting; no universal-format ranking. |
| Exact schema, numeric widths, caps, host normalization and extension policy | DATA-02–09; OPEN-CAT-01; contract owner | Concrete logical-to-wire mapping and accepted/refused fixtures, including aggregate limits, stable ID allocation/collision rules, relationship checks and the reader/version matrix. |
| Pre-RGT emission, metadata recognition and runtime-capability separation | OPEN-CAT-01/02; source/schema and TA-W02/03/06 owners | Freeze the recognized inert-data path or explicitly identified compatible projection; no RGT implementation prerequisite for independent NTP/NTS and no silent loss of unknown semantics. |
| Release trust, optional independent signature, revision replay and bad-clock freshness | OPEN-CAT-01/03; package/time-assurance owners | Actual artifact trust path and exact coverage, compromised/retired key and old-release scenarios, supported recovery without disabling TLS; apply the release-policy incident/compatibility cases without treating publication as installed adoption. |
| Roughtime operator/profile selection, rotation and evidence threshold | TA-W02/03/06/08 and OPEN-CAT-02/03; trust-root mechanics under OPEN-CAT-01 | Operator confirmations, qualified consumer/protocol controls and explicit scoped key transition; no admission by list membership or publication alone. |
| NTP default/vendor-zone and candidate permissions | OD-03; source/default owner | Official arrangement or operator terms for the actual intended defaults; no invented vendor hostname. |
| Catalogue, reviewed preset, configured and active membership; automatic replacement scope | OPEN-CAT-02; R-04; OD-03; source/default and acquisition owners | Identify the preset's C/policy owner and stable-ID binding before the relevant schema freeze. If replacement is selected, declare eligible population, trigger, stable reselection, finite traversal and continuing provider budgets. Reordering C or adding unselected entries must not change ordinary traffic; selected-source loss follows the authorized policy or reports insufficiency. No default, traversal permission or new Core prerequisite is adopted. |
| Q/H retention, context cardinality and catalogue migration | OPEN-DATA-02/03/04/06; storage owner | Required questions and compatible populations, whole-family limits and recovery; preserve unresolved prior DATA/EXP/SEC audit findings. |
| Optional detailed-history default and irreversible loss of detail | OPEN-DATA-02/03; product/storage owner | Named questions requiring detail versus summaries; mandatory conflict minimum remains TA-W02. |
| Publisher acquisition, approved-input build and private tooling scope | R/DATA-01/24; catalogue task §8 | Local source adapters and revalidated semantic diff now have a concrete R contract. Next define explicit operator evidence and approval binding; acquire only under a separately frozen fetch profile. C generation still depends on its schema/representation and consumer contract. |
| Final installed/removal lifecycle and physical placement | Existing W11/ST-W owners | Actual owner/ACL/update/Reset/uninstall observations when admitted; source research alone closes no native gate. |

The owner-requested five-role documentation review and integration are complete for this discussion stage. The [existing task](../../tasks/server-catalogue-and-statistics.md#71-completed-documentation-review-and-dispositions) retains the original reports, candidate identities, verified documentary corrections and still-open owner assignments. That documentation milestone included no format benchmark, Python implementation or live-server campaign. The subsequent owner-requested continuation implements only §6.4's offline R stage. The relevant logical-to-wire, preset/policy and compatibility contracts remain open before dependent C/runtime implementation.

Future catalogue measurements compare JSON only as a reference if useful, then one binary candidate at a time. Record encoding, delivered placement and runtime access separately. Measure actual incremental linked code and generated data, compressed package and installed allocation, old/new update staging, startup admission/copies/allocations, steady lookups and CPU, private bytes, relevant mapped/image residency and peak overlap without double-counting shared pages. Preserve equal semantics and security requirements; do not turn this into a catalogue-only serialization ranking. Q/H write/flush amplification, checkpoint/rewrite and migration headroom stay in their separate workload accounting. Q0/Q-J retains its existing preparation scope; this catalogue preference does not silently rename or execute that pilot.

## 9. Evidence and primary references

**Completed for the initial draft:** read current selected project documents at the stated commit; checked primary protocol/operator publications; matched the pinned ecosystem blob; strictly decoded the 13 public-key strings and checked 32-byte lengths. These are source and static-data checks only. Final review dispositions and exact integrated identities belong in the existing task.

[C1]: https://www.rfc-editor.org/rfc/rfc8949.html
[C2]: https://www.rfc-editor.org/rfc/rfc8610.html
[C3]: https://doc.rust-lang.org/reference/items/static-items.html
[C4]: https://protobuf.dev/programming-guides/serialization-not-canonical/
[C5]: https://www.rfc-editor.org/rfc/rfc10049.html
[C6]: https://www.rfc-editor.org/rfc/rfc8915.html
[C7]: https://nts.time.nl/
[C8]: https://time.txryan.com/
[C9]: https://developers.cloudflare.com/time-services/roughtime/usage/
[C10]: https://developers.cloudflare.com/time-services/roughtime/deprecation/
[C11]: https://roughtime.se/
[C12]: https://github.com/tannerryan/roughtime/blob/09ffa30abfb8c9ea22b81858edf393f6c5a7ddef/ecosystem.json
[C13]: https://github.com/jauderho/nts-servers
[C14]: https://www.ntppool.org/en/vendors.html
[C15]: https://github.com/jauderho/public-ntp-servers/blob/fa07c5440f4c2301697e978b2be36b743d3dd8f2/README.md

- **C1–C4:** binary representation, schema and generated-data possibilities; not implementation measurements.
- **C5–C6:** Roughtime and NTS protocol boundaries; current project owners retain complete admission/verification contracts.
- **C7–C11:** operator pages checked on 4 November 2026; statements and keys may change and do not prove live availability.
- **C12:** pinned community ecosystem data, matching fetched Git blob as recorded above; nominal group labels are not independence proof.
- **C13:** discovery input proposed by David, not an independently authenticated authority for all listed services.
- **C14:** official Pool vendor/default requirements; no vendor-zone application or external communication was performed.
- **C15:** pinned discovery input for the first offline NTP adapter; owner/AS/notes are source claims, not verified use permission, independence, no-smear evidence or current availability.
