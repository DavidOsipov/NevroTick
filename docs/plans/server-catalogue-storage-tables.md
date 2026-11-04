# Server catalogue and statistics — storage and format tables

**Recorded:** 4 November 2026  
**Status:** product-storage discussion reference; C/Q/H formats, runtime dependencies and quotas remain proposals. The separately implemented R input stage is identified below.  
**Source baseline:** `3f074edf6b9d4c8cd9fdf37a6e20832233b4946f`

These tables preserve the storage discussion and its format-review refinements.
The [catalogue and statistics draft](server-catalogue-and-statistics-draft.md)
owns the detailed DATA-01–DATA-24 inventory, proposed allocation, open decisions
and comparison card. The [existing task](../../tasks/server-catalogue-and-statistics.md)
owns preparation and evidence. This document is a readable companion to that
chain, not another execution plan.

The later [binary catalogue refinement](server-catalogue-binary-format.md)
records the owner's preference for binary shipment and first-schema Roughtime
key/profile fields. The [offline Python input stage](../../tools/server-catalogue/README.md)
now belongs under existing `tools/`, with candidate-specific evidence in
[task §8](../../tasks/server-catalogue-and-statistics.md#8-offline-publisher-collector--4-november-2026).
These refinements concern C and publisher work R; they select no final C encoding
and do not replace the separate Q0/Q-J comparison.

The [architecture](../windows-time-architecture.md), especially §§6, 12 and 17,
controls existing data, authority and resource contracts. A value labelled
**proposed** below is neither measured nor an enforced product maximum. Format
names do not establish authenticity, compatibility, compression ratio or speed.
No benchmark, product implementation or Windows qualification was performed to
produce these tables.

**Selected leap-policy integration, 4 November 2026:** [architecture §10.5](../windows-time-architecture.md#105-leap-seconds-and-time-scales)
and [NV-LP-001](leap-second-policy.md) govern the public leap dataset and derived
observation context. The [S1.4 task](../../tasks/s1.4-leap-policy.md) owns future
implementation. Its release-bound immutable compiled dataset is separate from
the source-catalogue and Q/H format candidates below; their format, quota and
implementation choices remain open. Earlier source/review baselines describe
their recorded candidates, not qualification of this amendment.

## 1. Reading the tables

| Term | Meaning |
| --- | --- |
| **C — catalogue** | Immutable public source definitions delivered as a binary resource or generated read-only image data with the application release. Reviewed preset, configured and active membership remain separate under OPEN-CAT-02/R-04/OD-03; an entry alone grants no use or replacement permission. The current direction is one complete hardened MSIX; the retained MSI alternative needs its own installed-file trust and lifecycle qualification. Exact representation remains unselected. |
| **Q — quality store** | Mutable advisory summaries, historical dictionaries, coverage and sparse events. SQLite typed rows and a bounded versioned JSON snapshot remain alternatives. |
| **H — optional history** | Justified detailed observations or proof material, stored as completed external segments or batches inside Q. It does not supply live authentication or clock authority. |
| **E — existing owners** | Separately owned policy, broker journals, trusted bounds, recovery records, drift prior, public leap dataset and optional secret storage. E does not mean one combined database or one confidentiality requirement. |
| **U — user data** | Preferences, display projections and explicitly saved support exports. |
| **R — publisher records** | Curation, release evidence and signing custody outside the installed client. |
| **Format specification** | Identify the storage mechanism, record encoding, batch/file container and compression separately. For example, `SQLite BLOB + versioned Protobuf batch + Zstd` names three different layers and remains only a candidate. |
| **Day-30 volume** | Distinguish retained data on day 30, bytes cumulatively written during 30 days, and peak RAM or temporary disk space. They are not interchangeable. |

**Shared illustrative budgets:** Q = **32 MiB for its complete file family**;
external H = **seven qualified days or 64 MiB**, whichever limit is reached first;
rotating local diagnostic logs = **10 MiB aggregate per installation**. All three
numbers are proposals. Q/H rows share their respective allowance; each row does
not receive another allowance. H inside Q consumes Q's allowance. Event Log,
exports and independently owned mandatory stores are separate.

## 2. Table A — catalogue, policy and live state

| What is retained | Where | Data format | Why here | Day-30 volume / maximum status | Security model |
| --- | --- | --- | --- | --- | --- |
| **Release catalogue:** sources, operators, dependence groups, country/region or explicit unknown, endpoints, protocols, optional addresses and public provenance. Each Roughtime record requires public-key algorithm/bytes and exact profile binding from the first schema. DATA-02–05; public subset of DATA-01. | C: immutable package resource or generated read-only installed-image data; bounded admitted index in RAM. | **If a separate resource is chosen:** constrained, schema-defined CBOR is the recommended comparison candidate. Generated immutable Rust tables remain a leading release-bound alternative; Protobuf is conditional. JSON may be a publisher input or comparison reference, not the preferred shipped format. Final schema and encoding remain unselected. | Infrequently updated shared source definitions for hundreds of records; reviewed preset membership and authorized local policy remain separate. | Catalogue byte/record caps are **unset**. It does not grow per poll. The 16-active-source bound does not cap the complete catalogue or authorize active objects for all its entries. | Authenticate the exact artifact under the release trust contract; bound resource loading or validate generated construction, then enforce identities, endpoints, profiles and current policy. Recognizing inert Roughtime metadata does not enable or qualify its runtime. A separate project signature is unselected. Authenticity does not establish freshness or defeat whole-release rollback. |
| **Reviewed leap history, future coverage and effective UTC rule.** Separate domain owner; DATA-02 references identity, DATA-09 applicability. | E: immutable compiled data in the same authenticated complete release; bounded read-only domain view. | Normalized versioned data generated from reviewed pinned inputs, with exact dataset/rule revision and digest. This selects no C/Q/H encoding. | Public authority for interpreting events and the continuous-UTC rule effective 20 May 2027; independent of optional secret restoration and source announcements. | No per-poll or day-30 growth. Import byte/record/date/work limits are frozen by S1.4 before implementation. | Existing release integrity/version-transition rules; no runtime external leap-file downloader, arbitrary input path or new trust root. Absent/expired future coverage preserves valid history and may remain `Unknown`; corrupted package bytes remain an integrity failure. Updates cannot reset protected correction continuity. |
| **Authorized machine/source/security policy, local overrides and future NTS pin policy.** DATA-06–07 and authoritative local definitions. | E: broker or other explicitly authorized policy owner; narrow validated RAM projections. | **Strict versioned UTF-8 JSON is already selected for machine policy.** Future pinset schema remains unselected; choosing a new catalogue encoding does not replace this policy contract. | Authority-bearing settings need their own writer, revision and recovery lifecycle. | Total byte cap **unset**; bounded policy state rather than a 30-day observation archive. | Authorized writes, revision/CAS checks, protected file and parent-directory rights, qualified durable replacement and migration. Invalid mandatory policy cannot silently become permissive defaults. |
| **Resolution, network, time-base and applicability-generation contexts.** DATA-08–09. | Current contexts in RAM; only necessary historical dictionaries and bindings in Q/H. | Typed RAM IDs/maps; typed SQLite rows or JSON snapshot records in Q; fields/dictionaries of the chosen H representation. | Avoid repeating descriptions in every observation while preserving compatibility and provenance. | Shared Q/H budgets. Historical context and dictionary cardinality caps remain **unset**. | Historical addresses, generations and prior verification results do not authorize a current connection or prove freshness. Re-admit live state and reject incompatible mappings. |
| **Current exchanges, recent samples and estimator state.** Live DATA-10–12 and DATA-15. | Bounded engine/core RAM. | Rust structs/enums, arrays and bounded rings; no hot-state compression by default. | Predictable live access and mutation without database or codec work on timing-critical paths. | Up to **16 × 64 = 1,024 regression samples** under the architecture's count bounds. Total bytes are not frozen; pending native work, helper state and other buffers are additional. No 30-day accumulation in this live window. | Checked units, source identity, correlation, generations and private domain construction. Do not restore pending exchanges. Logical retirement does not release native handles, buffers or permits before actual native completion. |
| **Qualified drift prior.** Durable subset of DATA-15. | E: a separate bounded engine-owned restart record. | Versioned structured record; exact disk encoding/schema remains **unselected**. | A limited restart hint has different validity and retention needs from a detailed archive. | Record byte cap **unset**; not a daily append-only history. | Check provenance, uncertainty, time basis and continuity under the prior-loader contract. Discard an invalid optional prior. A prior supplies no independent clock-write or authentication authority. |
| **NTS association secrets and optional protected snapshots.** DATA-22. | Private bounded working RAM; optional separate protected store. | Private RAM buffers/association objects. Persistent candidate: a versioned encrypted and authenticated envelope; exact canonical encoding and cryptographic suite remain **unselected**. | Minimize secret lifetime and keep secret custody separate from catalogue, statistics and diagnostics. | Byte/lifetime limits **unset**. `NoPersistentCache` means zero application-created secret snapshots, not zero nonsecret lifecycle metadata or proof of no OS paging/dump copies. | Follow the qualified protected-storage roots, exact-envelope binding, epochs and recovery rules. No secret copies in C/Q/H/UI/logs. Public-key wrapping plus AEAD alone does not establish independently trusted-writer authenticity. |

## 3. Table B — quality, evidence and optional history

Source profiles and historical contexts retain provenance for unsmeared UTC,
announcement behavior separately, the admitted leap dataset/effective-rule
revision, all supported date/era hypotheses, scale segment and mapping generation.
Observation summaries retain raw LI, the possible exchange span and bounded
admission reason. All-LI=0 can remain useful; a demonstrably contradictory
announcement excludes that observation, without removing healthy peers or
preventing fresh consistent re-entry. Unknown future coverage alone permits
ordinary Tracking outside the calendar-derived guard; it is not a storage error.
Guard transitions, post-guard acquisition and residual corrections retain their
existing fresh-quorum and actual-acknowledgement rules. Q/H cannot supply leap
authority, merge incompatible segments or restore a pending exchange.

| What is retained | Where | Data format | Why here | Day-30 volume / maximum status | Security model |
| --- | --- | --- | --- | --- | --- |
| **Quality aggregates and coverage:** counts, stage denominators, compatible numeric summaries and evaluator-versioned vectors. DATA-16; derived from DATA-10–14. | Bounded RAM accumulators and Q. | **Candidates:** typed SQLite rows or structured versioned JSON snapshot. Do not turn searchable scalar fields into opaque encoded blobs without a query requirement. | Small mutable summaries can answer defined questions without permanent raw history. | Shares proposed Q **32 MiB**. The illustrative day-30 model gives **3,168 rows ≈ 1.55 MiB payload** at 512 bytes/row; this is not a measured database size or maximum. | Keep metric-specific validity, gaps and denominators; merge only disjoint compatible populations. Unknown is distinct from zero. Quality summaries do not prove server honesty, UTC truth or live authentication. |
| **Operational preference, cooldown, selection/reference and acknowledged-effect links.** DATA-14 and DATA-17. | Current state in RAM; sparse descriptive records in Q. | Typed state machine in RAM; versioned records/events in the selected Q encoding. | Explain operational decisions without persisting a duplicate event on every sample. | Shared Q allowance; event rate, cardinality and retention limits **unset**. | Revalidate restart hints against current policy and continuity. Historical preferences cannot clear mandatory restrictions. The broker's actual outcome remains independently authoritative. |
| **Bootstrap and conflict evidence.** DATA-13 and DATA-18. | Bounded live state; Q summary/index; justified detail in H. Any mandatory cross-restart minimum has a separate E owner. | Q's chosen format for summaries; necessary **exact protocol bytes** with versioned metadata in the chosen evidence container. Schemas remain unselected. | Preserve attributable evidence and uncertainty without treating all archived observations as live proof. | Optional detail shares Q/H allowances. The mandatory cross-restart minimum, cap, missing-state behavior and clearance remain **TA-W02 / OPEN-DATA-05** work. | Expiry, proof loss or Q/H reset cannot mean a conflict was resolved. No blanket durable quarantine store is adopted by this table. Historical signed responses retain their original proof scope. |
| **Optional detailed observation/attempt history.** Detailed DATA-10–12 and DATA-14. | H: completed external segments **or** bounded compressed batches inside Q. | **Candidates:** Parquet with internal Zstd; framed JSONL/CBOR batches; conditional framed Protobuf batches; Avro Object Container Files as a secondary option. For a SQLite BLOB, separately identify its batch encoding and codec. Select one primary representation. | Detailed replay, support or analytical queries when the product need justifies retaining detail. | Zero optional detail when disabled. Otherwise illustrative **seven qualified days or 64 MiB**, first limit reached, for external H. At 16 sources and a 64-second interval, seven days at 192 bytes/slot gives **27.69 MiB uncompressed payload**. H inside Q shares Q's 32 MiB proposal. | Protect the complete file family from creation; bound input/output, counts, strings, nesting, decoder traversal, codec window/workspace, concurrency and work. A completed or sealed segment is not automatically authenticated or encrypted. No live-estimator restoration from old detail. |
| **Manifests, dictionaries, checkpoints, idempotency, migrations, explicit gaps and collection/resource health.** DATA-19–20. | Q; H metadata; bounded live counters in RAM. | Typed records in Q's selected encoding; chosen H headers/metadata with explicit versions, identities and content bindings. | Make retained data interpretable and recovery attributable; explain incomplete collection. | Included in Q/H budgets, **not free overhead**. Caps for historical contexts, files and pending work remain to be frozen. | External segment publication and a Q commit are distinct operations. Reconcile exact identities and bytes within bounded recovery. A colocated digest is not independent authentication; resource pressure must not become a false server-quality failure. |

## 4. Table C — mandatory state, diagnostics and other owners

| What is retained | Where | Data format | Why here | Day-30 volume / maximum status | Security model |
| --- | --- | --- | --- | --- | --- |
| **Mandatory authority and recovery:** broker outcomes/journals, trusted bounds, restoration and lifecycle state. DATA-21. | E: separate broker, trust and lifecycle stores. | **Strict versioned UTF-8 JSON for broker journals.** Supporting records keep their existing owners' schema/encoding decisions. | Must survive according to authority and recovery obligations, independently of optional history. | Total byte cap **unset**. Active authority or unresolved effects cannot be deleted merely because 30 days elapsed. | Protected ownership, durable ordering, bounded conservative recovery and qualified migration. Clearing Q/H cannot refund consumed authority, settle an uncertain effect or erase required restoration state. |
| **Local diagnostic logs.** DATA-23. | Bounded component sinks and queues. | Structured versioned events; exact persistent file codec remains **unselected**. JSONL is not automatically adopted. | Support diagnosis with controlled verbosity and bounded retained output. | **Proposed 10 MiB rotating aggregate per installation**, divided into component allocations; not multiplied by users or processes. | Redact before formatting; typed fields, protected service sinks, coalescing and locally expiring authorized DEBUG. No keys, cookies or unrestricted payload dumps. |
| **Windows Event Log; optional external traces and dumps.** DATA-23. | OS-owned channels and separately configured collectors. | EVTX for Event Log; ETL for applicable ETW trace files; dump format depends on the selected collector. | OS integration and explicitly scoped diagnostic evidence have different owners and lifecycles. | Separate quotas **unset here**. These are not automatically covered by the local 10 MiB proposal. | Qualify channel/collector access and capture scope. Raw traces and dumps are not automatically redacted and may expose sensitive memory. |
| **UI preferences and status projections.** DATA-23. | U: user profile; bounded display state in RAM. | Typed RAM view model; applicable selected IPC uses versioned UTF-8 JSON. Preferences-file format remains **unselected**. | Keep user presentation settings separate from machine authority. | Byte caps **unset**; current display state does not accumulate for 30 days. | No direct privileged database access or policy authority. Projections are bounded; stale values are explicitly historical. |
| **Explicit support exports.** DATA-23. | U: user-selected output written by the unelevated UI. | Format **unselected**; needs version/schema, declared contents and redaction rules. | A deliberate portable support artifact, not an automatic raw database copy. | Per-export cap **unset**. Arbitrarily retained user copies have no defined installation-wide 30-day total. | Preview and redact; exclude secrets and unbounded dumps. Do not introduce a privileged arbitrary-path writer. Export does not restore trusted state. |
| **Future DNS learning and optional snapshots.** Separate DATA-22 boundary. | Separate DNS owner/store/privacy lifecycle. | Data schema and encoding **unselected**. Protected snapshots have their own authenticated-envelope contract. | Different privacy, forgetting and validity requirements from time-source statistics. | Zero for a profile without the feature; future limits **unset**. | Separate privacy epochs, forget barriers and secret custody. Do not merge DNS learning into Q merely because both classes are called statistics. |
| **Publisher curation, release evidence and signing custody.** DATA-01 and DATA-24. | R: outside the installed client; offline input tool under existing `tools/`, with approved-input build/signing still separate. | **Current input stage:** closed versioned UTF-8 JSON manifest, candidate and semantic diff; original synthetic Markdown/JSON fixtures. **Other R owners:** reviewed approvals, release hashes/signatures and protected key objects; the collector takes no signing secrets. The future builder emits the selected C representation from the exact approved normalized snapshot; no single encoding applies to all R. | Explain releases and preserve signing authority independently of client reset or uninstall. | Zero client-held publisher private signing keys, full research corpus or Python runtime introduced for collection. Publisher retention/capacity is a separate policy. | Bound and validate untrusted source inputs. Bind approval to the reviewed snapshot and relevant adapter/schema/generator identities. Collection, review approval, signing and publication remain separate acts with their existing owners. The client receives only required public verification material. |

## 5. Reviewed format candidates

The following is a workload-based shortlist, not a universal size/speed ranking.
The current Q/H comparison card remains in draft §10; the binary refinement
supplies the later C comparison. This table does not automatically expand either
experiment scope or adopt a dependency.

| Format / mechanism | Relevant workload | Tradeoff to qualify | Current recommendation |
| --- | --- | --- | --- |
| **JSON** | Publisher input/reference catalogue comparison; bounded Q snapshot and existing selected security boundaries. | Actual schema size, strict admission, complete decode/rewrite cost and recovery. | Optional C reference, not the preferred shipped catalogue. Q-J and existing strict JSON policy/IPC/journal contracts remain authoritative in their stated scopes. |
| **Schema-defined CBOR** | Catalogue and bounded sequential history batches. | Numeric keys versus positional representation, presence, permitted types, evolution, aggregate limits and strict decoder behavior. | Recommended binary-resource candidate for C, pending comparison and admission. Textual keys are not required; no size/speed advantage is measured here. |
| **Generated immutable Rust tables** | Catalogue coupled to an application release. | Deterministic generation, stable IDs, checked values, actual image size/relocations/resident pages and current-policy checks; omitting a runtime data parser does not by itself prove lower total cost. | Serious C alternative under the [binary refinement](server-catalogue-binary-format.md); exact layout and toolchain remain unselected. |
| **Protobuf** | Conditional catalogue or explicitly framed history batches. | Signed-number types, field presence, unknown/duplicate fields, stable schema meanings, framing, generated code and runtime. | Additional conditional candidate when schema tooling, size or interoperability supplies a concrete reason. No measured advantage over suitable CBOR is established. |
| **MessagePack** | Alternative catalogue/batch encoding. | Equivalent semantics and actual value distributions; parser, dependency and compatibility costs. | Reserve candidate if implementation or interoperability gives a material benefit. |
| **FlatBuffers** | Frequent direct reads from a retained serialized buffer. | Verification, immutable-buffer lifetime, indexing, layout and actual Rust/Windows implementation. | Reconsider when avoiding repeated object construction becomes a demonstrated requirement. |
| **Cap'n Proto** | Direct-access structured data; potentially intensive interchange. | Pointer/traversal limits, alignment, packing/unpacking, buffer ownership and actual implementation. | Conditional future candidate; no demonstrated current workload advantage. |
| **Avro Object Container File** | Sequential, schema-bearing detailed archives. | Writer/reader schema admission, bounded schema resolution, block sizing and actual codec support. | Secondary H candidate if its container solves a named archive requirement. |
| **Parquet with internal Zstd** | Detailed history with selective analytical reads. | Row-group, dictionary, footer and codec costs; query patterns and peak memory. | Retain as an H candidate. Do not wrap the complete file in outer Zstd as the default design. |
| **SQLite typed rows** | Mutable summaries, context dictionaries, coverage, sparse events and checkpoints. | Whole-family storage, indexing, durability, query and maintenance costs. | Retain as a main Q candidate against the bounded snapshot alternative. |

### 5.1 Corrections to generic format claims

| Claim | Correct qualification |
| --- | --- |
| CBOR must repeat textual field names because it is schema-less. | CBOR supports integer map keys and positional arrays; CDDL can describe the agreed structure. Type/length information still has a cost. [F1, F2] |
| Protobuf is general-purpose compression with only one- or two-byte tags. | It is an encoding; tag length and value size vary. Compression, if selected, is a separate layer. [F3] |
| FlatBuffers provides instant, zero-cost reads. | It avoids materializing a separate object graph, but safe Rust root access verifies the buffer first. I/O, semantic checks and application work remain. [F6] |
| Cap'n Proto eliminates all encode/decode and CPU work. | Direct access still needs pointer and traversal safeguards. Packed representations require unpacking; ordinary application work remains. [F7] |
| Avro requires generated code or an external schema registry. | Code generation is optional. An Object Container File carries its writer schema and block organization without requiring a registry. [F8, F9] |
| MessagePack is universally smaller than CBOR. | Shortest encodings differ by value: integer `24` is 1 byte in MessagePack and 2 in CBOR; `-129` is 3 and 2 respectively. These are specification-derived examples, not file benchmarks. [F1, F10] |

### 5.2 Small encoding example

For one value `-1` in agreed units, excluding version headers, framing, signatures
and compression:

| Representation | Bytes |
| --- | ---: |
| Protobuf field 1, `int64 = -1` | 11 |
| Protobuf field 1, `sint64 = -1` | 2 |
| CBOR map `{1: -1}` | 3 |
| CBOR positional record `[-1]` | 2 |

These counts follow the specifications [F1, F3]. The alternatives have different
evolution contracts; this example demonstrates schema sensitivity, not an overall
format winner.

## 6. Capacity model and accounting

Assumptions: 30 days of 86,400 seconds each, a half-open observation interval,
one scheduled slot per source per polling interval, 192 hypothetical bytes per
detail record, and 512 hypothetical bytes per aggregate row. Summaries have one
compatible population per source/bucket, seven retained days of hourly buckets,
and up to 180 retained days of daily buckets. A record per slot is a calculation
assumption, not a persistence requirement. MiB = 2^20 bytes; GiB = 2^30 bytes.

| Model | Scheduled slots in 30 days | Uncompressed 30-day detail payload | Retained seven-day detail payload | Day-30 summary rows / payload |
| --- | ---: | ---: | ---: | ---: |
| 4 sources, 64-second interval | 162,000 | 29.66 MiB | 6.92 MiB | 792 / 0.387 MiB |
| 16 sources, 64-second interval | 648,000 | 118.65 MiB | 27.69 MiB | 3,168 / 1.547 MiB |
| 16 sources, one-second synthetic stress interval | 41,472,000 | 7.416 GiB | 1.730 GiB | 3,168 / 1.547 MiB, only under the same single-population assumption |

The 16-source seven-day calculation is
`16 × (7 × 86,400 / 64) × 192 = 29,030,400 bytes`.
At day 30, summary count is `16 × (7 × 24 + 30) = 3,168`.
At mature 180-day retention, the analogous count is
`16 × (7 × 24 + 180) = 5,568`, or **2.719 MiB** at 512 bytes/row.
Do not double-credit observations when querying overlapping hourly and daily
representations.

| Budget or measurement | Interpretation |
| --- | --- |
| Q 32 MiB + external H 64 MiB + local logs 10 MiB = **106 MiB** | Proposed subtotal for these classes only. Not an installation maximum or a measured result. |
| Q 32 MiB + local logs 10 MiB = **42 MiB** | Corresponding proposed subtotal when H is absent or shares Q as BLOBs. A Q-contained archive has no additional independent 64 MiB allowance. |
| Catalogue, E, user data, Event Log, traces/dumps and exports | Separate or unresolved capacity. They cannot be assumed to fit the subtotals above. |
| 16 active sources and 64 regression samples/source | Count bounds; not a byte cap on the catalogue, retired contexts, pending native work or all RAM. |
| Architecture's proposed 32 MiB engine-plus-broker private-memory target | Different from Q's proposed 32 MiB disk allowance. It has its own steady-state measurement conditions; report peaks separately. |
| 512 MiB comparison-process ceiling | Experimental process limit in the comparison proposal, not a production memory target. |
| A rolling retention allowance | Does not multiply by 30 months/days. Cumulative writes can exceed retained bytes many times. |

The payload estimates omit variable-size fields, schema/container headers,
indexes, SQLite page overhead, compression, WAL/journals, temporary files,
migration copies, retries, bootstrap/KE work, sparse proofs and multiple
compatible populations. They establish neither a physical maximum nor a
compression ratio. Charge simultaneous encoded/decoded copies and codec
workspace to peak RAM; charge staging, maintenance and incomplete cleanup to
their complete file-family budgets.

## 7. Format-independent security qualifications

| Boundary | Required distinction |
| --- | --- |
| Exact artifact authentication | Verify the exact captured bytes under the admitted release/envelope contract, then parse those same immutable bytes. Protobuf's noncanonical serialization does not prevent authenticating a specific artifact; casual decode/reserialize is not a stable signature procedure. [F5; draft §5.5] |
| Presence and duplicate fields | Preserve zero, missing, unknown, not run and not retained. Protobuf ordinary decoding can merge repeated singular fields; implicit presence can hide absent/default differences. Freeze acceptance before decoding destroys required evidence. [F3, F4] |
| Schema evolution | Admit explicit versions and writer/reader relationships. Embedded schemas and generated types do not independently authorize semantics, units, defaults or new authority. Avro schema resolution requires its own controlled contract. [F9] |
| Direct buffer access | Structural verification and domain checks still apply. Keep verified backing bytes immutable; bound traversal, nesting and retained memory. [F6, F7] |
| Secret envelopes | Record encoding is distinct from cryptographic protection, writer authentication, key custody, epochs and rollback behavior. Follow the [protected-storage plan](windows-protected-storage.md); no weaker fallback is selected here. |
| Compression | Bound compressed input, decompressed output, decoder window/workspace, counts, work and concurrency. Compression supplies no authenticity. |
| SQLite and archive protection | Protect parent directories, sidecars, temporary files and migration outputs as well as the principal file. Structural integrity is not authorship, truthful observations or freshness. Q/H remain advisory. [Draft §§5.4–5.6] |
| Actual implementation | Qualify the concrete Rust library, generator/runtime combination, features, codecs and Windows target. Official Protobuf Rust currently uses C/C++ kernels; a Rust API does not by itself describe the complete implementation. [F11] |

## 8. Review provenance and references

The preceding chat review used three fresh-context agents: format/performance,
security/data contracts, and storage/workload accounting. All completed their
scoped source reviews. They supported the allocation for continued discussion,
identified qualifications to the external format comparison, and established no
implementation vulnerability or measured performance winner. The capacity
arithmetic was independently checked. These tables incorporate the substantive
qualifications; they are not a replacement for the existing task's raw earlier
proposal-audit records or its open prerequisites.

Project authorities:

- [Catalogue and statistics draft](server-catalogue-and-statistics-draft.md), especially §§5–7 and 10.
- [Existing catalogue/statistics task](../../tasks/server-catalogue-and-statistics.md).
- [Architecture](../windows-time-architecture.md), especially §§6, 12 and 17.
- [Protected-storage plan](windows-protected-storage.md), especially §4.

Primary format references checked during the 4 November 2026 discussion:

- **F1:** [RFC 8949 — CBOR](https://www.rfc-editor.org/rfc/rfc8949.html).
- **F2:** [RFC 8610 — CDDL](https://www.rfc-editor.org/rfc/rfc8610.html).
- **F3:** [Protocol Buffers encoding](https://protobuf.dev/programming-guides/encoding/).
- **F4:** [Protocol Buffers field presence](https://protobuf.dev/programming-guides/field_presence/).
- **F5:** [Protobuf serialization is not canonical](https://protobuf.dev/programming-guides/serialization-not-canonical/).
- **F6:** [FlatBuffers Rust guide](https://flatbuffers.dev/languages/rust/).
- **F7:** [Cap'n Proto encoding and security considerations](https://capnproto.org/encoding.html).
- **F8:** [Apache Avro documentation and introduction](https://avro.apache.org/docs/).
- **F9:** [Apache Avro 1.12.0 specification](https://avro.apache.org/docs/1.12.0/specification/).
- **F10:** [MessagePack specification](https://github.com/msgpack/msgpack/blob/master/spec.md).
- **F11:** [Protobuf Rust design decisions](https://protobuf.dev/reference/rust/rust-design-decisions/).
- **F12:** [Parquet file layout](https://parquet.apache.org/docs/file-format/) and [internal compression](https://parquet.apache.org/docs/file-format/data-pages/compression/).
- **F13:** [SQLite record/file format](https://www.sqlite.org/fileformat2.html) and [application uses](https://www.sqlite.org/whentouse.html).

No format selection, independent updater, new default retention, extra service,
dependency installation, benchmark or lifting of existing execution holds follows
from saving these tables.
