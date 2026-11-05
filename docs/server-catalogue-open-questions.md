# Server catalogue: open decisions and expected DNSSEC protection

**Recorded:** 6 November 2026, Asia/Tbilisi.

**Inspected upstream:** `3de77a3f50324d5400042811127c85efe3cd4c5b`.

**Status:** owner-requested consolidation of existing open questions, plus a proposed DNSSEC continuity rule. This is a decision index and preparation note; canonical requirements and execution stay with the linked owners. No production roster, C encoding, DNSSEC implementation or new release blocker is selected here.

## 1. Existing direction and implementation

The catalogue is already designed in the [binary refinement](plans/server-catalogue-binary-format.md) and [DATA-01–24 inventory](plans/server-catalogue-and-statistics-draft.md). Its [completed documentation review](../tasks/server-catalogue-and-statistics.md#7-binary-catalogue-and-future-collector-refinement--4-november-2026) must not be reopened as an unexplored architecture question.

- Preserve **Roughtime-first provisional-date bootstrap, then full NTS**, with Khronos/Ananke as the conditional reserve when Roughtime cannot supply a usable date. [NV-TA-001](plans/time-assurance-integration.md) owns the exact flow; a catalogue supplies neither authenticated time nor clock-write authority.
- Deliver a reviewed NTP/pool, NTS and Roughtime catalogue with the complete signed application release; initially refresh it through application releases. Binary shipment is preferred. Generated immutable Rust tables and constrained CBOR remain candidates, with no measured winner or separate hot updater.
- The first schema includes Roughtime public-key bytes/algorithm, exact protocol profile, source/endpoint authorization bindings and provenance. Stable source/operator/endpoint/key/binding identities, typed variants and lifecycle invariants are already described. The 13-key documentary inventory is not an approved production roster.
- Catalogue membership, reviewed preset, configured acquisition population and active work are separate. Existing default-four/maximum-16 ordinary active sources and maximum-two helpers do not define catalogue cardinality or unlimited pending work. Q/H history remains advisory; mandatory E policy and trust state retain their owners.
- The [offline R collector](../tools/server-catalogue/README.md) already implements three local adapters, exact-input checks, unapproved candidate snapshots and revalidated semantic diff. Its `claim_id` identifies captured evidence, not a permanent product source. Approval, C generation and actual client admission remain subsequent work.

## 2. Existing open questions

This register expands the existing [OPEN-CAT-01–03 entries](plans/server-catalogue-and-statistics-draft.md#6-open-decisions-and-owner-questions) for convenient continuation. It does not rename them or turn every pending mechanism into a new prerequisite for current Core work.

### 2.1 OPEN-CAT-01 — representation, construction and release trust

| Decision still needed | Existing constraint and required result | Canonical owner |
| --- | --- | --- |
| Generated immutable tables or a constrained binary resource? | Compare equivalent admitted data and whole-product size, startup/RAM/CPU and update costs. Keep Q/H storage comparisons separate; JSON used by R does not select the client format. | [Binary plan §§3/8](plans/server-catalogue-binary-format.md#3-representation-candidates) |
| Exact schema, ID allocation, normalization, widths and limits? | Freeze logical-to-wire mapping, namespace/collision and transition rules, host/IDNA semantics, aggregate bounds and accepted/refused fixtures. Enforce reference/key/profile invariants through the real Rust consumer or trusted generated-data construction. | [Binary plan §4](plans/server-catalogue-binary-format.md#4-logical-schema-before-field-number-freeze); DATA-02–09; CHECK-25 |
| Early reader's Roughtime representation? | Choose recognized inert records or a separately identified reviewed compatible projection. No silent dropping of unknown semantics; metadata availability does not claim an implemented verifier or activate RGT-0. | [Binary plan §4.3](plans/server-catalogue-binary-format.md#43-source-records-do-not-construct-live-authority); OPEN-CAT-02; TA-W02/03/06 |
| Exact authenticated artifact and installed admission path? | Bind reviewed input, generator/schema/export, emitted bytes and admitted release. Decide whether an additional project signature is justified; it is not automatically mandatory. Preserve immutable generation/backing-storage lifetime and actual package identity. | [Binary plan §§6–7](plans/server-catalogue-binary-format.md#6-future-python-collection-and-build-tool); package owners |

### 2.2 OPEN-CAT-02 — source approval, presets and acquisition

| Decision still needed | Existing constraint and required result | Canonical owner |
| --- | --- | --- |
| Which operators/endpoints are admitted for each role? | Complete use-permission/load, unsmeared-scale, dependence, destination and protocol evidence. Separate Roughtime bootstrap, ordinary NTS and any authorized plain/helper/reserve role. Published public-use permission can be evidence; do not impose a universal bespoke written-consent requirement. Pool/vendor prerequisites remain where applicable. | [OD-03](nevrotick-owner-decisions.md#5-od-03--the-pool-preference-and-an-nts-preset); [R-04](nevrotick-research-handoff.md#6-r-04--default-servers-and-the-supplied-nts-list); TA-W06 |
| Exact Roughtime profile/root bindings and usable independent groups? | Qualify actual wire versions, keys, transports, rotation/withdrawal authority and cold-resolution behavior. Count independent usable groups after filtering, not rows, addresses, keys or protocol aliases. Preserve the selected bootstrap priority. | [Binary plan §5](plans/server-catalogue-binary-format.md#5-roughtime-verification-material-is-required-now); TA-W02/03/06/08 |
| Where does the preset live and how does policy select it? | Freeze C-versus-authorized-policy ownership, ordered stable-ID binding and effective C/E composition. Reordering C or adding unselected records must not change ordinary traffic. The four-operator NTS shortlist is still a candidate, not an accepted shipped preset. | [Binary plan §4.3](plans/server-catalogue-binary-format.md#43-source-records-do-not-construct-live-authority); R-04; OD-03; OMI-R01/02 |
| Is automatic cross-source replacement wanted? | If selected, define eligible population, triggers, stable traversal, exhaustion, continuing provider budgets and actual pending/retired-resource accounting. Otherwise report insufficiency under the selected policy. No catalogue-wide scan, weaker agreement or plaintext fallback follows from an empty slot. | OPEN-CAT-02; [retained scenarios](../tasks/server-catalogue-and-statistics.md#72-retained-scenarios-under-existing-owners); acquisition/R-02 owners |
| How does reviewed evidence become approved build input? | Specify the operator-evidence overlay and exact snapshot/schema/review binding before C construction. A parsed record, successful probe or claimed DNSSEC support does not approve a provider. Optional acquisition requires its own bounded profile. | [Catalogue task §8.5](../tasks/server-catalogue-and-statistics.md#85-remaining-work-under-existing-owners); R/DATA-01/24 |

### 2.3 OPEN-CAT-03 — freshness, withdrawal and recovery

| Decision still needed | Existing constraint and required result | Canonical owner |
| --- | --- | --- |
| Fresh/offline first installation and protected revision floors? | Distinguish authentic bytes from current eligibility. Define supported old-release behavior and trustworthy age/revision evidence when system time is wrong; select no arbitrary expiry duration here. | [Binary plan §7](plans/server-catalogue-binary-format.md#7-compatibility-failure-and-release-lifecycle); time assurance |
| Key/source/policy withdrawal and authorized recovery? | Define who changes bindings, how installed clients learn restrictions, detection/reaction limits and valid corrective transitions. Publishing a withdrawal does not prove offline adoption. Signed old packages cannot erase restrictions already held in mandatory E. | OPEN-CAT-01/02/03; TA-W02/06; [release policy](release-policy.md) |
| Package replacement, rollback, reset and retained references? | Freeze compatible C/E combinations, stale-result refusal, surviving recovery authority and interpretation of old Q/H references. Optional history repair must not restore withdrawn authority or delete mandatory state. | OMI-R02/03/07/08; [catalogue lifecycle](plans/server-catalogue-binary-format.md#7-compatibility-failure-and-release-lifecycle); W11/ST owners |

Q/H retention, scoring, automatic exclusion, optional pinning and persistence remain with their existing OPEN-DATA/OPEN-PIN/OPEN-ST owners. They are adjacent work, not implied decisions in this register.

## 3. Proposed addition: expected DNSSEC protection

### 3.1 Intent and accurate security meaning

On 6 November David proposed checking DNSSEC when preparing the supplied source list, remembering that protection is expected, and raising an error if it is absent later. Record this as a **per-source DNSSEC continuity proposal**. Exact coverage, default enablement, implementation stage and recovery policy remain open.

DNSSEC protects DNS data through validated signatures and a chain to an admitted trust anchor. Finding a DNSKEY or RRSIG, setting DO, or receiving AD is not by itself that proof. A signed child without an authenticated delegation can still be classified insecure. The existing [DNS contract §3.2](plans/system-dns-cache-design.md#32-dnssec-contract) already recommends local validation and explicitly separates secure, insecure, bogus and indeterminate results. [D1–D3]

A protection failure is **not conclusive evidence of MiTM**. Bogus data can result from attack, misconfiguration or corruption. Operators can also deliberately remove DNSSEC delegation; RFC 8078 describes such transitions. The useful additional rule is that a source marked **RequireSecure** remains unusable through an insecure DNS result even if the validator correctly establishes that the current delegation is insecure. That is a policy mismatch; a legitimate DNS change does not automatically authorize weakening NevroTick policy. [D1, D4]

### 3.2 Keep observation, policy and current evidence separate

Names below are illustrative logical meanings, not a selected serialization.

| Layer | Proposed information and authority |
| --- | --- |
| Publisher evidence R | Normalized queried name and record types; alias/delegation scope; validation outcome and reason; validator/trust-anchor-policy identity; trustworthy observation time and viewpoint; relevant evidence/digests and signature/TTL limits. Preserve operator statements separately. A release-time observation is dated evidence, not permanent security or availability. No live check is performed by this document or the current offline collector. |
| Reviewed catalogue C plus authorized E policy | A scoped requirement such as `RequireSecure`, with source/endpoint/service binding and policy revision. A historical `observed_secure` fact must not silently become policy. No additional per-source requirement must also be explicit; that case does not disable ordinary DNSSEC validation or authorize bogus answers. Unknown evidence must remain unknown. |
| Resolver/engine result | Current validated answer/status and operational reason, bound to exact query/content, network view, trust/time context and policy generations. DATA-08/09 references the resolver's owned evidence; Q/H history is not a second validation cache or authority source. |

Pin the **requirement**, not the currently observed domain DNSKEY/DS fingerprint. Properly authenticated zone-key rollovers should continue to work. Trust-anchor maintenance, algorithm policy and any exceptional key pinning have their own existing owners; this proposal chooses no new trust anchor. [D2, D4]

### 3.3 Proposed behavior for RequireSecure

Apply the rule at the consumer that admits DNS-derived destinations, after bounded complete resolution/validation. Missing records in one packet, AD=0 or a generic SERVFAIL do not identify the final DNSSEC state. Preserve validator classification and operational failure separately. [D1–D3]

| Established result | Proposed source behavior |
| --- | --- |
| Secure usable answer | Permit the DNS admission step while its scope, validity and current context remain applicable; continue ordinary destination, Roughtime/NTS, source-agreement and broker checks. |
| Insecure, including authenticated absence of a secure delegation | Refuse that DNS-derived destination under RequireSecure and report a protection-policy mismatch. Do not label the unsigned data cryptographically bogus merely because stricter application policy rejects it. |
| Bogus | Refuse; preserve the actual reason. An attack is possible, not proven. No retry may silently disable validation. |
| Indeterminate or no established validation result | Refuse this required-protection path; report the actual missing trust material, capability or operational evidence. A timeout alone is not a fabricated DNSSEC verdict. |
| No qualified time basis | Report validation unavailable due to time confidence; use only the separately authorized bootstrap path. Do not call the domain unsigned solely because the local clock makes a signature appear expired or premature. |
| Secure authenticated NXDOMAIN/NODATA | Respect the proven absence. It supplies no address and no MiTM finding. Source availability and any alternative acquisition remain with the existing policy. |

Suggested status: **“Expected DNSSEC protection could not be confirmed; this source endpoint was not used.”** Include a bounded reason such as insecure delegation, signature-validation failure, unavailable validation or unqualified time. Do not display “MiTM detected” without independent evidence.

A failure must not automatically clear RequireSecure, select plaintext DNS/NTP, change resolver/trust anchors, promote cached history, or relax configured agreement. A reviewed policy change or compatible release can address a legitimate operator transition while preserving any stricter authorized local E restriction; authorization and rollback protection retain E/OPEN-CAT-03 ownership. A new DNS failure does not by itself revoke a still-admitted authenticated association or block independent eligible sources: their current trust, freshness and policy owners decide continued use.

### 3.4 Scope, bootstrap and integration questions

1. **Name coverage:** bind requirements to the actual service identity and resolution path. Review CNAME/DNAME chains, terminal A/AAAA data, applicable denial/wildcard evidence and split-DNS/network views. Security of a parent name or original alias does not automatically cover a different target. A complete validator must also handle legitimate synthesized records rather than require a separate signature on every packet record. [D2]
2. **NTS delegation:** distinguish KE authentication identity, configured/actual KE peer and negotiated NTP destination. An independently named delegated destination needs its own admitted resolution rule; authentic NTS-KE does not supply a DNSSEC verdict. Preserve BND-2 and do not invent another DNS lookup where the protocol uses the actual KE peer.
3. **Roughtime-first bootstrap:** DNSSEC signature validity and DoQ certificate validity require usable time. Do not make obtaining the initial Roughtime date recursively depend on that same unavailable date. Freeze the permitted bootstrap mechanism—such as reviewed address candidates bound to existing Roughtime identity, or another explicitly admitted discovery path—under [NV-TA-001 §7.1](plans/time-assurance-integration.md#71-admission-and-independence). It must preserve destination admission and protocol/root verification; this note authorizes no weaker DNS path or preliminary global clock step. DNSSEC supplies no independent time authority. [D2]
4. **Validation owner:** reuse the existing local-validation direction. A Windows lookup returning addresses or raw DNS bytes is not a locally verified DNSSEC proof. A remote-validation trust arrangement would need its own authenticated channel, trusted validator and accurately limited claim; DoQ transport alone does not prove validation. The inspected Windows result structures expose query data, not a documented local-validation proof; an exact implementation/build still needs qualification. The future whole-machine DNS-2 stage is not silently made a new prerequisite for the initial time-client release. [D3, D5; DNS plan §§3/12]
5. **Policy lifetime:** freeze the approved domains/roles, evidence refresh and revalidation triggers, policy changes, offline behavior, legitimate operator disablement, diagnostic privacy and finite retry/work budgets before dependent implementation. Current observations cannot rewrite C/E policy; old in-flight results cannot publish after relevant policy, network, time or trust changes.

## 4. Future discriminating controls and next action

These are proposed consumer observations, not executed tests or new acceptance IDs. Use OPEN-CAT-01/02/03, DATA-01/04/06/08/09, BND-2, TA-W02/06 and existing DNS-C03/04/05/11/12.

| Paired case | Required distinction |
| --- | --- |
| Valid secure answer and valid authenticated key rollover / stripped or invalid signature under a secure chain | Useful secure resolution continues; the failing result cannot create a destination or time authority. |
| Current secure delegation / authenticated DS removal with RequireSecure retained | The latter is refused as policy mismatch even when classified Insecure; no unsupported MiTM conclusion or silent policy rewrite. |
| Scoped ordinary policy for an insecure name / RequireSecure for that name | Distinct policy outcomes; neither case permits bogus data or changes NTS/Roughtime verification. |
| Valid complete alias chain / secure alias to an insecure terminal name | RequireSecure covers the needed answer path; an authenticated prefix cannot launder an unprotected destination. |
| Qualified time and valid current proof / unavailable time or expired proof | Preserve time-unavailable versus actual validation failure; exercise the authorized Roughtime bootstrap without circular dependence or a clock write from DNS. |
| Authentic resolver evidence / forged AD or generic failure with no established state | No packet flag or transport success manufactures a validation result. |
| Current-policy completion / delayed completion after policy/network/trust change | Only the current admissible result can publish; retained memory and history grant no exemption. |

Continue from [catalogue task §8.5](../tasks/server-catalogue-and-statistics.md#85-remaining-work-under-existing-owners) when that work is selected. Before implementing this DNSSEC proposal, its source-policy, DNS and time-assurance owners must settle the specific scope and bootstrap/deployment arrangement above. Current [development §7](plans/development-plan.md#7-next-executable-task-for-a-development-agent), Core/proof priority, existing native/fuzz/storage/UI holds, mandatory mathematical proofs and optional enhanced release provenance remain unchanged. This note authorizes no source probe, provider contact, catalogue fetch, runtime implementation or Windows configuration change.

## 5. Primary references

Retrieved 6 November 2026, Asia/Tbilisi; these standards explain semantics, not the current DNSSEC state of any listed provider.

- **D1:** [RFC 4033 §5](https://www.rfc-editor.org/rfc/rfc4033.html#section-5), [RFC 4035 §4.3](https://www.rfc-editor.org/rfc/rfc4035.html#section-4.3) and [RFC 9499 §11](https://www.rfc-editor.org/rfc/rfc9499.html#section-11): validation states. The original definitions differ in their treatment of indeterminate; retain actual validator evidence and explicit operational reasons.
- **D2:** [RFC 4035 §§5.2–5.4](https://www.rfc-editor.org/rfc/rfc4035.html): authenticated delegations, RRset validation, signature-time/TTL constraints and denial of existence; [RFC 6672 §§5.3.1–5.3.3](https://www.rfc-editor.org/rfc/rfc6672.html#section-5.3.1) for DNAME synthesis. Apply relevant updates through the admitted validator.
- **D3:** [RFC 6840 §§5.7–5.9](https://www.rfc-editor.org/rfc/rfc6840.html#section-5.7): AD trust and signaling; [RFC 4035 §3.2.1](https://www.rfc-editor.org/rfc/rfc4035.html#section-3.2.1): DO controls requested DNSSEC material, not local validation.
- **D4:** [RFC 8078 §§1.2/4](https://www.rfc-editor.org/rfc/rfc8078.html): legitimate DNSSEC disablement and delegation maintenance. Such a DNS transition is distinct from authorizing a weaker application policy.
- **D5:** Microsoft Learn: [DNSSEC validation and client roles](https://learn.microsoft.com/en-us/azure/dns/dnssec#dnssec-validation), [DNS_QUERY_RESULT](https://learn.microsoft.com/en-us/windows/win32/api/windns/ns-windns-dns_query_result) and [DNS_QUERY_RAW_RESULT](https://learn.microsoft.com/en-us/windows/win32/api/windns/ns-windns-dns_query_raw_result). Inspected through Learn MCP; these API return contracts alone establish no qualified local validator.
