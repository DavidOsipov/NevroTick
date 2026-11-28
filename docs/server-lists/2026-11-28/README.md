# NTS and Roughtime discovery — 28 November 2026

**Owner instruction:** collect the supplied NTS directory and find Roughtime
servers first. The other eight audit topics remain for joint discussion in the
[existing backlog](../../../tasks/development-backlog.md#joint-production-decisions--28-november-2026).
**Subsequent decision, 29 November:** [OD-23](../../nevrotick-owner-decisions.md#27-od-23--production-quorum-and-evidence-policy)
accepts item 2's default quorum of three fresh operator groups; remaining profile
choices and items 3–9 are still open. This does not admit a provider preset.
The [catalogue task](../../../tasks/server-catalogue-and-statistics.md) and
[R-04 research](../../nevrotick-research-handoff.md#6-r-04--default-servers-and-the-supplied-nts-list)
own this collection. These are discovery candidates, not an approved shipping
preset. No time-server queries, DNS probes, client execution or provider contacts
were performed.

## Captured sources and provenance

| Source | Pinned revision | Local copies |
| --- | --- | --- |
| jauderho/nts-servers | `97a5116fc1a31c309b5d2bb79bab38863f3f8f09` — 23 November 2026 | [Complete NTS list](nts-servers-README.md#the-list), [canonical YAML](nts-sources.yml), [chrony](chrony.conf) and [ntpd-rs](ntp.toml) examples, [BSD-3-Clause license](nts-servers-LICENSE) |
| tannerryan/roughtime | `7454b45d337b413d3639da88acf5559d08d7af27` — 26 November 2026 | [Ecosystem JSON](tannerryan-roughtime-ecosystem.json), [BSD-2-Clause license](tannerryan-roughtime-LICENSE) |
| cloudflare/roughtime | `75645289794cfbd71a08f0e7ecf9bc4f3f87d133` — 28 April 2026 | [Ecosystem JSON](cloudflare-roughtime-ecosystem.json), [ecosystem descriptions](cloudflare-roughtime-ecosystem.md), [Apache-2.0 license](cloudflare-roughtime-LICENSE) |

[provenance.json](provenance.json) records each of the ten original files'
immutable upstream URL, path, revision, Git blob ID, byte length, SHA-256 and
license. All saved bytes reproduce their upstream Git blobs. Original source
text and formatting are unchanged; NevroTick's annotations are in this README.
The generated example configurations are evidence, not activated configuration.

[inputs.json](inputs.json) uses the existing
[offline R input contract](../../../tools/server-catalogue/README.md) for the NTS
README and both Roughtime JSON files. Both ecosystem files use the same declared
`tannerryan-ecosystem-json-v1` shape; this does not change their provenance.
Overlapping records remain separate evidence. No collector execution or new
collector test result is claimed. The manifest selects no runtime format,
approved root set or default-source policy.

Tanner's JSON is byte-identical to the earlier `09ffa30abfb8c9ea22b81858edf393f6c5a7ddef`
revision in [binary catalogue §5.2](../../plans/server-catalogue-binary-format.md#52-initial-public-key-evidence-inventory--4-november-2026):
both have Git blob `cd9eb5043fd7ff77845333ffe97b8318303ff94e`.
The newer repository revision does not imply changed roster content. Earlier
collection and verification receipts retain their original scope.

## Complete NTS list

The captured [README list](nts-servers-README.md#the-list) and
[YAML](nts-sources.yml) contain **113 unique hostname rows: 89 main rows and 24
marked virtualized**. Both generated examples have the same hostname set.
An unmarked row is not proof of physical hardware. All source owner labels,
stratum values, locations, notes and use restrictions remain attached.
The 47 distinct owner labels are not 47 independently qualified operators;
repeated operators and Cambridge's separate department labels do not establish
independent votes. Two names occur only in notes:
`ntp-0-hq.rdem-systems.com` and `ntspool.time.nl`; they are not extra roster rows.
These are counts of the [pinned upstream data](https://github.com/jauderho/nts-servers/blob/97a5116fc1a31c309b5d2bb79bab38863f3f8f09/nts-sources.yml).

All four Ubuntu rows preserve **Distro use only**, and all 24 VM rows preserve
the upstream caution. No row specifies a KE port; RFC 8915 assigns TCP 4460,
which is a protocol default rather than a per-host measurement. Authenticated
NTP uses the negotiated endpoint/port. See [RFC 8915](https://www.rfc-editor.org/rfc/rfc8915.html).
The directory's broad NTP/NTS mixing statement does not replace NevroTick's
accepted helper and selection contracts.

### NTS operator annotations


These are targeted primary-source annotations, not a comprehensive provider policy audit. They do not grant product-scale approval.

| Service/operator | Additional evidence and practical interpretation | Primary source |
|---|---|---|
| Cloudflare | Official documentation says its time service is free and **does not leap smear**. Time services are subject to the linked Cloudflare Website and Online Services Terms of Use. No new client-scale permission determination was made. | https://developers.cloudflare.com/time-services/ntp/ ; https://developers.cloudflare.com/time-services/nts/ ; https://developers.cloudflare.com/time-services/tos/ |
| TimeNL / SIDN Labs | Official NTS page names the four `ntppool[1-4].time.nl` hosts as the unicast pool. Locations are Arnhem, Ede, Amsterdam, Amsterdam. It recommends 1/2 for production and describes 3/4 as UTC(VSL)-derived; certified traceability is explicitly still under development. `ntspool.time.nl` is a pool name, not a sixth independent operator/source family. | https://nts.time.nl/ |
| TimeNL / SIDN Labs | Operator says it does not leap smear; its public service is free subject to fair use, can be filtered/rate limited, changed or discontinued, and has no liability guarantee. `any.time.nl` is described as **experimental** anycast. The vendor section forbids large-scale firmware hard-coding of `ntppool.time.nl` without contacting the operator for an agreed vendor name. Do not broaden this to an independently established legal ruling about every hostname, or infer clearance for NevroTick defaults. | https://time.nl/index_en.html |
| Ubuntu | Source roster expressly marks all four entries distro-only. Ubuntu documentation confirms those four pool names in its default NTS configuration and states **TCP 4460** for KE and **UDP 123** for NTP. The documentation describes a separate bootstrap pool; that bootstrap name is not part of the 113-row upstream roster. | Pinned source list; https://ubuntu.com/server/docs/how-to/networking/chrony-client/ |
| Meta | Operator announcement dated 2026-10-06 calls `nts.meta.com` a free public service and describes **TCP 4460** KE. It is **KE-only**: the response negotiates an NTP responder, with examples `time1.meta.com` through `time5.meta.com`, using UDP 123. Those hosts belong to the same Meta service and must not be counted as independent operators or added as five independently verified KE endpoints. | https://engineering.fb.com/2026/10/06/production-engineering/nts-authenticated-time-at-meta/ |
| Meta timescale caveat | The 2020 public Facebook NTP article documents approximately 18-hour post-event leap smearing. The 2026 NTS announcement links that earlier service history but does not specify the new NTS service’s leap policy. Treat the current NTS leap policy as **unconfirmed/pending**, with historical smear evidence; do not assert present UTC compatibility or a current exact smear policy. | https://engineering.fb.com/2020/03/18/production-engineering/ntp-service/ ; 2026 announcement above |
| Netnod | Official roster matches all 11 source hostnames. Operator says NTS is free for anyone worldwide and gives **TCP 4460** KE. It documents legacy KE ports 3443/4443 for backward compatibility. Its how-to says authenticated NTP uses negotiated **UDP 4123**, so a consumer must not hard-code UDP 123 after KE. These documents do not demonstrate present reachability from NevroTick clients. | https://www.netnod.se/nts/network-time-security ; https://www.netnod.se/netnod-time/how-to-use-nts |

Except where the above operator sources state a policy, leap treatment and additional provider terms remain unreviewed/unknown. “Public list membership” alone is not default-distribution permission, broad-scale eligibility, or verification of an independent source.

## Roughtime inventory summary

Tanner's list contains **13 Ed25519 records plus one experimental ML-DSA-44
record**. Cloudflare adds int08h, producing **14 distinct Ed25519 keys** across
the union. Every Ed25519 value strictly decodes to 32 bytes. That checks
representation, not ownership, signatures, operational status or UTC accuracy.
The ML-DSA-44 extension is not an adopted V1 crypto profile or an additional
operator. Exact original records remain in the two JSON snapshots [TAN], [CF].

Only two operator sites retrieved in this pass explicitly advertise RFC 10049:
Tanner Ryan and TimeNL, both UDP/TCP 5319. Generic IETF labels and implementation
test fixtures cannot establish the deployed profile of the other servers [TX], [NL].

## Roughtime endpoints and public keys

All keys below are long-term Ed25519 public keys in RFC 4648 standard Base64, copied from pinned source inventories. The roster is descriptive. Endpoint reachability, successful signature verification, clock correctness, uptime, and final compatibility were not tested. Operator-specific overlays below take precedence over a list's broad version label.

| Inventory name | Hostname | Nominal operator group | Documented Roughtime transport/port | Long-term Ed25519 public key (Base64) | Protocol evidence | Region evidence | Sources |
| --- | --- | --- | --- | --- | --- | --- | --- |
| time.txryan.com | time.txryan.com | Tanner Ryan | UDP + TCP :5319; alternate :2002 | `iBVjxg/1j7y1+kQUTBYdTabxCppesU/07D4PMDJk2WA=` | Explicit operator RFC 10049; :2002 flex supports RFC, IETF drafts, Google | Beauharnois, Québec, Canada; OVH AS16276 | [TAN], [TX] |
| TimeNL-Roughtime | rough.time.nl | SIDN Labs / TimeNL | UDP + TCP :5319; alternate :2002 | `v2CievhgKsxzlWPwIkYFUXeA51Akhkv5uhJCj1/kbiY=` | Explicit operator RFC 10049; :2002 adds IETF drafts through 19 and Google | Netherlands; exact Roughtime host city not stated | [TAN], [NL] |
| Cloudflare-Roughtime-2 | roughtime.cloudflare.com | Cloudflare | UDP :2003 | `0GD7c3yP8xEc4Zl2zeuN2SlLvDVVocjsPSL8/Rl/7zg=` | Operator ecosystem text: Google-Roughtime + IETF draft08; exact deployed 2026 profile unverified | Global anycast, IPv4 + IPv6 | [TAN], [CF], [CFM], [CFU] |
| roughtime.se | roughtime.se | roughtime.se; hosted by STUPI AB | UDP :2002 | `S3AzfZJ5CjSdkJ21ZJGbxqdYP/SoE8fXKY0+aicsehI=` | Operator page: draft-19; not an RFC 10049 claim | Stockholm, Sweden (community description) | [TAN], [CF], [CFM], [SE] |
| sth1.roughtime.netnod.se | sth1.roughtime.netnod.se | Netnod (same group as sth2) | UDP :2002 | `9l1JN4HakGnG44yyqyNNCb0HN0XfsysBbnl/kbZoZDc=` | Current list label IETF-Roughtime; Netnod project docs historical draft07 | Stockholm label in historical/public descriptions; current site not independently rechecked | [TAN], [NN], [NND] |
| sth2.roughtime.netnod.se | sth2.roughtime.netnod.se | Netnod (same group as sth1) | UDP :2002 | `T/xxX4ERUBAOpt64Z8phWamKsASZxJ0VWuiPm3GS/8g=` | Current list label IETF-Roughtime; Netnod project docs historical draft07 | Stockholm label in historical/public descriptions; current site not independently rechecked | [TAN], [NN], [NND] |
| time.teax.dev | time.teax.dev | teaishealthy / teax.dev | UDP :2002 | `84pMADvKUcSOq5RNbVRjVrjiU16Dxo2XV2Qkm+4DRTg=` | Generic IETF-Roughtime list label; exact deployed profile unresolved | Operator-published gist: Hetzner VPS, Nuremberg, Germany | [TAN], [TEAX] |
| roughtime.sturdystatistics.com | roughtime.sturdystatistics.com | Sturdy Statistics, LLC | UDP :2002 | `NqIjwLopQn6yQChtE21Mb97dAbAPe5UOuTa0tOakgD8=` | Operator page: IETF revision 15, wire value 0x8000000c | Region not stated in retrieved operator material | [TAN], [SS] |
| Inutile-Roughtime-FR | fr.ntp.inutile.pro | Inutile (same group across four hosts) | UDP :2002 | `caVakKYAW26s2mDSfY21nI1ctN/w3kPBL8oGRAqLxRg=` | Generic IETF-Roughtime list label; exact deployed profile unresolved | FR is a list/hostname label; physical site unverified | [TAN] |
| Inutile-Roughtime-ES | es.ntp.inutile.pro | Inutile (same group across four hosts) | UDP :2002 | `/z+14WcSjMU4evfyQoMKU8GGa5KSF9sW69tu3c+KBXs=` | Generic IETF-Roughtime list label; exact deployed profile unresolved | ES is a list/hostname label; physical site unverified | [TAN] |
| Inutile-Roughtime-UK | uk.ntp.inutile.pro | Inutile (same group across four hosts) | UDP :2002 | `bUjS5xMA6FBRQzhriEs71uyQu0+JDf/TeZ0IFNTdJZ0=` | Generic IETF-Roughtime list label; exact deployed profile unresolved | UK is a list/hostname label; physical site unverified | [TAN] |
| Inutile-Roughtime-US | us.ntp.inutile.pro | Inutile (same group across four hosts) | UDP :2002 | `V7hnCRDc7iR2uv0VrmGu37Yn3ovdHjcDW8oiFGC7hh4=` | Generic IETF-Roughtime list label; exact deployed profile unresolved | US is a list/hostname label; physical site unverified | [TAN] |
| roughtime.alastyr.com | roughtime.alastyr.com | Alastyr Telekomünikasyon A.Ş. | UDP :2002 only | `/2Al59PlwUTJHas/VS+FN6BhO9ify/wIjoH2Ezv4V7w=` | Operator page: draft-19, wire value 0x8000000c; roughtimed 1.3.0 | Bornova / İzmir, Türkiye | [TAN], [ALA] |
| int08h-Roughtime | roughtime.int08h.com | Stuart Stock / int08h | UDP :2002 published; :5319 only in new implementation usage examples | `AW5uAoTSTDfG5NfY1bTh08GUnOqlRb+HVhbJ3ODJvsE=` | Deployment unresolved: older operator blog + Cloudflare list vs new RFC 10049 software README | 2018 blog: GCE us-central1; 2024 Cloudflare entry: DigitalOcean NYC; current placement unverified | [CF], [CFM], [INT], [ROUGH] |

The normal and flex endpoints of one service share an operator and key; they are alternate transports/profiles, not extra votes. Exact TCP support on alternate port 2002 is not uniformly documented on each operator page; UDP 2002 is independently present in prior lists. Do not copy the software's generic listener capability into an operator-specific endpoint claim. For TimeNL the primary JSON snippet uses the string `"1"`; Any later normalization needs its own explicit adapter contract; do not rewrite the evidence snapshot or feed the string version to the existing integer-only branch. [NL], [TX], [TANR]

## Primary-operator overlays and material caveats

### Tanner Ryan

The operator site lists Beauharnois, Québec, Canada; OVH AS16276; IPv4 149.56.214.17 and IPv6 2607:5300:205:200::f3b0. Its RFC endpoint is UDP/TCP 5319; 2002 is labeled flex for RFC, drafts, and Google. It publishes the same Ed25519 key as both ecosystem lists. The site permits individual users, asks for contact before high-volume infrastructure use, and offers no uptime or accuracy guarantee. No contact was made. Its ordinary clock service is stratum 2; current upstream independence needs separate review. [TX]

### SIDN Labs / TimeNL

The operator explicitly describes Roughtime as an experiment, says the server runs tannerryan/roughtime v1.28.0, and documents RFC 10049 on 5319 plus RFC/drafts through 19/Google on 2002. The page explicitly lists UDP and TCP 5319. It warns users to include at least two other servers and points to an older Roughenough interoperability issue as fixed. The page separately lists several Dutch NTP/NTS sites; those locations should not be asserted as the exact rough.time.nl host location without further evidence. [NL]

### Cloudflare

The usage page publishes UDP roughtime.cloudflare.com:2003 and key 0GD7...; it describes IPv4/IPv6 anycast and beta status, including possible future root-key changes. The pinned ecosystem prose claims Google-Roughtime and IETF draft08. This is the strongest exact operator profile statement retrieved here, but it is not a contemporary packet-level verification or proof that later profiles are absent. The old :2002 key gD63... is expressly deprecated. [CFU], [CFD], [CFM], [CFA]

### roughtime.se

The operator site says draft-19, port 2002, STUPI AB hosting, and direct attachment to atomic clocks tracking UTC. Its key matches both maintained inventories. Cloudflare's description identifies Stockholm and roughtimed, with no availability guarantee. A January 18, 2026 operator reply by Marcus Dansarie denies supposed downtime and explains silent dropping of incompatible versions and routing constraints through Telia/Arelion; this is historical context, not a present route guarantee. Do not label this service as a Netnod-operated vote solely from contributor affiliation. [SE], [CFM], [MAIL]

### int08h / Roughenough discrepancy

The operator's February 17, 2018 blog gives UDP 2002, the AW5u... key in both Base64 and hex, and GCE us-central1 hosting. Cloudflare's May 2024 update instead describes DigitalOcean NYC, Google/Amazon/NIST upstream NTP, leap smearing, and a two-second radius. Neither historical placement should be treated as independently confirmed current hosting. [INT], [CFM]

Roughenough repository head 04ca59bf321b46b650574aa9e67900f1a9c216a5, dated 2026-10-17, documents RFC 10049 software and examples using roughtime.int08h.com:5319. That demonstrates the author's software/documentation claim, not independent current deployment. Current Tanner ecosystem omits int08h. Its continued presence in Cloudflare's older list is a source disagreement to preserve. The Roughenough test fixture serverlist-cloudflare.json labels Cloudflare, roughtime.se and others with integer version 1; because it is test data and conflicts with primary deployment docs, do not use it as compatibility evidence. [ROUGH], [ROUGHF], [TAN], [CF]

A November 2025 mailing-list message assigns int08h the key iBVjxg/1j7y1+kQUTBYdTabxCppesU/07D4PMDJk2WA=, which is also listed there for Tanner. This contradicts the operator blog and both old/current Cloudflare entries. Preserve this as a likely attribution/transcription problem, not a verified int08h key rotation; do not merge the two operator identities or select the inconsistent key. [MAIL], [INT], [CF]

### Netnod

Netnod's own vadarklockan repository pins both sth1 and sth2 on UDP 2002 with keys matching Tanner. The repository head is f47d1f486e7af8b018e89cb36589168910856f9e, dated 2023-11-05; its project docs describe draft07. That is primary operator-affiliated historical evidence, not current RFC support. The same example ecosystem contains lab and deliberately false-time servers; it is unsuitable for wholesale import into a bootstrap trust store. Both sth hosts remain one nominal Netnod operator group. [NN], [NND]

### teax.dev

Direct retrieval of https://time.teax.dev/ failed in the web tool. An operator-published GitHub gist, last active January 23, 2026, contains the page source, the exact matching key, and port 2002. Its raw link identifies gist revision 364c051f2ba12c0f1477b7f3ba7ad2a4cf5e8fca (raw fetch itself was restricted). It says Hetzner Nuremberg and Cloudflare NTP over NTS as the time source, with 24-hour leap smearing. Treat this as upstream dependence on Cloudflare and unresolved exact deployed Roughtime profile; independent hostname/key does not establish an independent time source. [TEAX]

### Sturdy Statistics

The operator page publishes UDP 2002 and the matching NqIj... key, explicitly calling its profile IETF Revision 15 / 0x8000000c. This is not an RFC version-1 claim. Its page includes illustrative/live example responses; these were not executed or independently verified. No geography, capacity commitment, or current independence claim was established. [SS]

### Inutile

The four keys/endpoints are directly captured in Tanner's current maintained inventory. Its July 5, 2026 commit introduced them, but the associated PR has no substantive description. Direct retrieval of https://ntp.inutile.pro/ failed. Current exact protocol support, operator identity beyond the nominal Inutile group, physical sites, upstreams, key-management guarantees, and capacity terms remain unresolved. Country prefixes are labels, not independent location verification. Keep all four in one provisional operator group. [TAN], [INPR]

### Alastyr

The operator page publishes UDP-only 2002, the matching key in Base64 and hex, İzmir, and draft-19 / 0x8000000c via roughtimed 1.3.0. It describes its GNSS/PPS source, offline long-term key and annual delegation. Its prose still predicts RFC publication and port allocation, despite RFC 10049 already existing; do not silently upgrade the server's profile from that stale standards prose. The page discloses one-year retention of request metadata, including source IP/port and nonce, and says polling more often than once a minute is unnecessary. The claimed single GNSS reference plus fallback should be assessed as an upstream dependency, not assumed fault independence. [ALA]

## Retired bindings, verification and remaining decisions

Cloudflare's old UDP `roughtime.cloudflare.com:2002` binding and public key
`gD63hSj3ScS+wuOeGrubXlq35N1c5Lby/S+T7MNTjxo=` have a published deprecation
date of 30 June 2024. The replacement is UDP 2003 with the different 0GD7 key
in the table. Do not revive the old binding from historical lists [CFD], [CFU].
Cloudflare's prose also records historical unreachable Google-Sandbox,
Chainpoint and Mixmin services; they are not current JSON rows or fallback
recommendations [CFM]. Netnod's historical examples include lab and deliberately
false-time servers and must not be imported wholesale [NN], [NND].

The existing [root/profile contract](../../plans/server-catalogue-binary-format.md#51-root-keys-and-protocol-profiles)
controls replacement roots. Ordinary DNS TXT results do not authorize root
changes. Preserve exact endpoint/key/profile bindings and shared-operator
relationships when reviewing candidates.

Verification covers saved-byte SHA-256/Git blob identities, identical NTS
hostname sets in all four formats, the 89/24 split, strict JSON parsing and
Ed25519 key lengths. All ten raw files retain their original licenses.
Redistribution licenses do not themselves grant third-party service permission.

This is a dated discovery inventory, not an exhaustive global census. Present
reachability, actual protocol exchanges/signature verification, regional
coverage, operator/upstream independence, permitted product load, timescale
compatibility and key-transition policy remain unqualified. No runtime code or
configuration was changed. Collection closes only the requested research step;
production source selection, quality scoring, statistics retention and bootstrap
thresholds remain with existing owners and the
[joint-discussion TODO](../../../tasks/development-backlog.md#joint-production-decisions--28-november-2026).

## Sources

Operator pages were retrieved on 28 November 2026. Deployment statements remain
attributed claims, not independent measurements. Websites may change after this
check; captured repository files use immutable revisions.

[TAN]: https://github.com/tannerryan/roughtime/blob/7454b45d337b413d3639da88acf5559d08d7af27/ecosystem.json
[TANR]: https://github.com/tannerryan/roughtime/blob/7454b45d337b413d3639da88acf5559d08d7af27/README.md
[TANH]: https://api.github.com/repos/tannerryan/roughtime/commits?path=ecosystem.json&per_page=15
[CF]: https://github.com/cloudflare/roughtime/blob/75645289794cfbd71a08f0e7ecf9bc4f3f87d133/ecosystem.json
[CFM]: https://github.com/cloudflare/roughtime/blob/75645289794cfbd71a08f0e7ecf9bc4f3f87d133/ecosystem.md
[CFH]: https://api.github.com/repos/cloudflare/roughtime/commits?path=ecosystem.json&per_page=20
[CFL]: https://github.com/cloudflare/roughtime/blob/4b43192f1fc0229069a249d498b87b6550cc7ec9/ecosystem.json
[CFU]: https://developers.cloudflare.com/time-services/roughtime/usage/
[CFD]: https://developers.cloudflare.com/time-services/roughtime/deprecation/
[CFA]: https://groups.google.com/a/chromium.org/g/proto-roughtime/c/vbmjoudG184
[TX]: https://time.txryan.com/
[NL]: https://nts.time.nl/
[SE]: https://roughtime.se/
[INT]: https://int08h.com/post/public-roughtime-server/
[ROUGH]: https://github.com/int08h/roughenough/blob/04ca59bf321b46b650574aa9e67900f1a9c216a5/README.md
[ROUGHF]: https://github.com/int08h/roughenough/blob/04ca59bf321b46b650574aa9e67900f1a9c216a5/crates/roughenough-client/testdata/serverlist-cloudflare.json
[NN]: https://github.com/Netnod/vadarklockan/blob/f47d1f486e7af8b018e89cb36589168910856f9e/examples/python/ecosystem.json
[NND]: https://vadarklockan.readthedocs.io/en/latest/
[NNESP]: https://github.com/Netnod/vadarklockan/blob/f47d1f486e7af8b018e89cb36589168910856f9e/examples/esp32/sketch_aug16b/sketch_aug16b.ino
[TEAX]: https://gist.github.com/teaishealthy/b214d95f6fc0a001278478dc9c37e806
[SS]: https://roughtime.sturdystatistics.com/
[INPR]: https://github.com/tannerryan/roughtime/pull/9
[ALA]: https://ntp.alastyr.com/roughtime.html
[GOOG]: https://roughtime.googlesource.com/roughtime/+/dd529367052d2d4e723407525887310fe866ddd8/roughtime-servers.json
[MAIL]: https://groups.google.com/a/chromium.org/g/proto-roughtime/c/7aY2sU1Alys
