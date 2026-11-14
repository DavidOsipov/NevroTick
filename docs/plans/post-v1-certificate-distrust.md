# Post-V1 certificate distrust and revocation

**Document:** NV-TRUST-001, revision 1.0

**Roadmap workstream:** TRUST-0

**Prepared:** 14 November 2026

**Status:** planned; all implementation and native qualification deferred until
the first software version, V1, has actually been released.

**Post-V1 rank:** second, after MATH-0/MATH-C01 and before MT-0/MT-C01;
KHR-0, GEO-0 and Android follow in their existing order.

**Local exit:** TRUST-C01, finite qualification and explicit disposition.

**Inspected product:** `bad2db638bb112496d16bb1172835473440b421e`.

## 1. Goal, authority and activation

David requests a post-V1 plan for both Mozilla OneCRL and CRLite, following a
review of the AdGuard certificate-interception issue and Windows/browser trust
differences. The intended result is timely additional rejection of compromised
or distrusted certificate paths used for NevroTick's own NTS authentication.
Investigate supplementary public-root removals and distrust-after constraints as
well: adopting the two feeds alone must not be described as Firefox-equivalent
certificate policy.

[OD-20](../nevrotick-owner-decisions.md#24-od-20--supplementary-certificate-distrust-after-v1)
and [TRUST-0](../../ROADMAP.md#trust-0--post-v1-certificate-distrust-and-revocation)
own the activation and ordering: actual V1 release → MATH-C01 → TRUST-C01 →
MT-C01 → KHR-C01 → GEO-C01 → Android. Existing V1 bounded workers are sufficient
for the initial qualification design; MT-0 is not a prerequisite.

This plan adds no V1 release gate, dependency, updater, trust-store mutation,
background activity or experimental execution. Continue the existing
[development plan](development-plan.md#7-next-executable-task-for-a-development-agent).
Actual V1 release, the roadmap's selected predecessor, a scoped execution card
and applicable environment admission are prerequisites for implementation.
The existing Windows, fuzz, storage and UI holds retain their separate owners.

The [architecture](../windows-time-architecture.md#1871-license-gate-and-finite-provider-choice),
[R-05](../nevrotick-research-handoff.md#7-r-05--windows-trust-with-a-wrong-system-date),
[crypto task](../../tasks/s0.1-crypto-feasibility.md) and
[engine task](../../tasks/engine-transport-pilot.md) retain technical authority.
Reuse OPEN-8, CRYPTO-04 and CHECK-6/8/9/15/26/27 as applicable; TRUST identifiers
are local planning identifiers, not replacements for canonical requirements.
Deferring these enhancements does not weaken existing V1 trust, revocation,
wrong-date, egress, association-lifetime or actual-native-completion obligations.

## 2. Evidence and its limits

The research supports additional protection, not a claim that Windows is always
slower or that every Windows-accepted chain would pass NevroTick's eventual
production policy. No reviewed incident is a demonstrated NevroTick exploit.

| Evidence | Supported conclusion and limitation |
| --- | --- |
| AdGuard CoreLibs issue 2110 [S01] | A TLS interceptor can conceal an origin chain from a browser's independent distrust policy. NevroTick is an NTS client, not a browser interception proxy; the relevant analogy is divergence between certificate-policy authorities. |
| eMudhra, October 2026 [S02–S03] | Mozilla reports notification on 17 October at 20:06 UTC and OneCRL distribution complete at 22:59 UTC for standard Firefox configurations. Microsoft records publication of an emergency Disallowed CTL at 11:00 Pacific on 20 October, or 18:00 UTC. The 67-hour-1-minute difference compares unequal milestones, not measured device exposure or a general update SLA. |
| Further eMudhra actions [S02–S03] | Microsoft's 26 October root changes and announced 16 November remaining-root removal are separate from its 20 October intermediate-key block. Mozilla also records an Android bundled-blocklist startup correction on 21 October. Publication, distribution and actual client enforcement must remain separate. |
| Entrust [S04–S06] | Mozilla's TLS issuance cutoff was 30 November 2024; NSS 3.107 carried the change on 21 November. Microsoft's 25 February 2025 deployment selected 16 April 2025. These cutoff dates differ by 137 calendar days; this is policy divergence, not a measured rollout or compromise interval. |
| e-Tugra [S07–S09] | NSS 3.92 removed the identified roots in July 2023; Microsoft's 2025 notice applies a 15 September issuance cutoff to the same RSA v3/ECC v3 roots. Full root removal and a new-issuance restriction have different semantics. Match certificate identities, not vendor names alone. |
| Windows cache semantics [S10–S13] | Native validation may reuse still-valid CRL/OCSP evidence. Fail-closed handling of unknown status does not establish newest evidence. CTL updates, CA revocation publication, local caches and application policy have different owners. |
| Contrary Windows evidence [S03, S11] | Microsoft has an emergency Disallowed CTL path and documents daily CTL retrieval capability. Do not claim a universal monthly/weekly delay. Local delivery still depends on policy and connectivity. |

The two Mozilla feeds complement each other but share ecosystem dependencies.
CRLite still depends on CA reporting and CT ingestion; neither feed discovers
every unreported compromise. An incident-specific timing comparison cannot
establish coverage or latency for the eventual NevroTick peer population.

## 3. Protection scope and selected initial policy

| Mechanism | Planned role | What it does not establish |
| --- | --- | --- |
| Windows chain and SSL policy | Retain the selected native trust, chain, name, EKU, signature, time and revocation checks, including enterprise policy and explicit distrust | Identical Mozilla policy, newest published intelligence, or current application-owned evidence age |
| OneCRL | Apply authenticated issuer/serial and subject/public-key matching semantics to the applicable validated path, including intermediate and cross-signed cases | A complete public-root trust store or an exhaustive leaf-revocation service |
| CRLite | Additional revocation evidence for covered, enrolled public TLS end-entity certificates | General root/intermediate distrust, unlogged/private-CA coverage, or validity of the rest of the chain |
| Supplementary public-root constraints | Investigate explicit public-root removal, usage restrictions and distrust-after semantics absent from those feeds | Automatic import of Mozilla roots, a selected data source, or approval to override enterprise/local trust |

OneCRL has two paired match forms: **issuer AND serial**, or **subject AND
`SHA-256(DER SubjectPublicKeyInfo)`** [S15]. Preserve Mozilla's decoded byte
representations for the DER-encoded issuer/subject names and certificate serial;
do not normalize display names or convert the serial through a numeric type.
The second form is neither a hash of the raw public key nor a global SPKI-only
block. Apply matching to leaf, intermediate and root certificates on the accepted
path. This scope still does not supply every Mozilla root-program constraint.

**Initial integration is rejection-only.** A valid applicable overlay rejection
vetoes native success; overlay absence or `Good` never overrides native failure.
Keep native known-revoked, explicit-distrust and unrelated policy failures fatal.
An ordinary accepted chain still needs every pre-existing product requirement.

Do not infer trust from absence in OneCRL or infer revocation from absence in a
public-root list. Distinguish explicit distrust from unsupported membership.
For root constraints, freeze the intended public-PKI scope, exact root/key
identity, cross-sign/alternate-path treatment, certificate-time field and cutoff
comparison, usage, exceptions and enterprise/private-root interaction. Evaluate
a maintained authenticated source and update path before choosing a mechanism.
Do not scrape announcement prose into a live security policy.

Whether fresh overlays become mandatory for a named assurance mode is an open
owner decision. In an additional-veto mode, native success without usable overlay
coverage may remain permitted only with the approved explicit status. In a
mandatory-overlay mode, unavailable required evidence yields holdover/Blocked.
This document selects neither an offline grace period nor a weaker fallback.

## 4. Current source and the integration boundary

Reinspect the actual released tree at activation; the following map is a dated
source observation, not an instruction to preserve a laboratory topology.

| Current owner | Observed behavior and required handoff |
| --- | --- |
| `crates/nevrotick-engine/Cargo.toml`, `src/lib.rs` | `ke-association-prototype` is default-off; actual TLS transport is Windows-gated. This source is not a public production NTS-to-clock path. |
| `crates/nevrotick-engine/src/nts_ke_experiment/transport.rs::connection` | Constructs explicit CNG, rustls TLS 1.3, `ntske/1`, ordinary platform verifier and no extra roots; disables resumption/early data. `.dangerous()` installs the custom verifier and does not by itself disable verification. |
| Vendored `verification/windows.rs::verify_certificate` | Builds a native chain, supplies stapled OCSP when present, evaluates SSL policy and returns success only for zero policy error. Its cache-only flags and zero ignore flags do not provide a separate product evidence-age guarantee. |
| Same verifier's `verify_server_cert` | Returns rustls's success marker, not a validated-path receipt. The input peer intermediates are not necessarily the selected Windows path. Adding a wrapper around this marker alone cannot recover path identity. |
| `transport.rs::InternalCandidate::publish`, `association.rs::AssociationCandidate` / `AssociationOwner::receive` | Move same-connection exporters/cookies through a private complete candidate and serialized publication guard. Future trust evidence must join this custody chain; a disconnected boolean or global last-verification slot is insufficient. |
| Request ledger, protected sender and production acquisition owners | Recheck current authority at use. The existing invocation/expiry controls are not evidence that production trust-feed revision and full-authentication-age suppliers already exist. |

Implement at the selected Windows chain/verifier owner while the actual path is
available, before success can authorize exporter/association publication. Retain
bounded immutable certificate identities/path material or a sealed equivalent
needed for later trust-change evaluation. Do not export native handles beyond
their valid owner or reconstruct the issuer from untrusted peer ordering.

Bind the result to the connection, server identity, selected chain, native-policy
context, trust-policy generation, feed identities and evidence-time basis.
Publication must recheck current policy after any concurrent update. The engine
owns networking and association state; the broker receives no feed-fetch or
certificate-policy bypass authority, and the UI only projects truthful status.

Preserve the distinction between observation freshness, cookie receipt, key age,
original full certificate authentication and current trust revision. Feed refresh,
cookie renewal and TLS resumption do not reset full-authentication age. Route
detected distrust, expiry or unknown age through R-05's bounded invalidation and
reauthentication policy, including already-active UDP associations and queued
work. A stale receipt must not survive an update between preparation and send.

## 5. Typed state and ownership contracts

The names below describe proposed semantics, not existing Rust types. Use private
fallible construction and the existing contract-enforcement conventions.

| Proposed state | Required distinction |
| --- | --- |
| `FeedIdentity` | Publisher, collection, channel, format, signer-policy identity and revision; identities cannot be interchanged |
| `AdmittedSnapshot` | Verified signed metadata, exact attachments, parsed bounded representation, revision/coverage and admission provenance |
| `RefreshState` | `Changed`, `UnchangedAuthenticated`, `Unavailable`, `Invalid`, `RollbackRejected`; transport success alone cannot mint authenticated currentness |
| `OneCrlDecision` | Applicable match, no match, or unavailable/invalid policy; no match is not a certificate-validity assertion |
| `CrliteDecision` | `Revoked`, `GoodCovered`, `NotCovered`, `NotEnrolled`, `Unavailable`, `Stale`, `Invalid`; preserve the cause of inconclusive evidence |
| `RootConstraintDecision` | Applicable explicit prohibition, applicable satisfied constraint, out of selected scope, or unresolved policy |
| `CertificateTrustReceipt` | Same-verification native result, actual path identities, all applicable overlay outcomes and revision/time basis |
| `AssociationTrustState` | Original authentication receipt plus current invalidation/full-auth-age guards; historical success alone grants no present send authority |

Keep authenticated publication/revision time, last successful refresh observation,
data coverage horizon and elapsed local observation age separately. A feed that
has not changed is not automatically stale; a legitimate empty changeset can
retain the same revision. Conversely, replaying an old signed response or reading
an unchanged disk file must not renew its freshness indefinitely. Define what
authenticated evidence permits an unchanged refresh and which age it advances.

The update owner admits one complete immutable revision atomically. Readers hold
a consistent snapshot during a decision; installation cannot mix generations or
drop a necessary delta. Association publication and use compare against the
current required policy, with defined invalidation on changes. The source-specific
snapshot store is mandatory security state if the selected mode depends on it;
it must not inherit an optional-cache fallback merely because the bytes are public.

## 6. Authenticated update and lifecycle design

OneCRL/CRLite transport is a security-data distribution path, distinct from the
unadopted application auto-updater/TUF research. Select endpoints, protocol and
dependencies explicitly; do not introduce a general URL-fetch capability.

For Mozilla Remote Settings, implement the applicable current protocol [S14]:

1. Verify the expected collection and signer against approved independent trust
   anchors; validate the signing chain, allowed algorithms and signed content.
   The current reference uses canonical collection JSON, the content-signature
   domain separator and ECDSA P-384/SHA-384. HTTPS or a hash supplied by the same
   unverified response is not equivalent to that verification.
2. Merge changes and tombstones correctly before checking the collection
   signature. Bind attachment hash and exact size to authenticated metadata;
   reject a missing/replaced filter, wrong channel or unsupported encoding.
3. Bound compressed/raw bytes, records, strings, nesting, integer conversions,
   attachments, memory and work before allocating or parsing. Signed data is
   still hostile input. Use strict typed decoding and checked transformations.
4. Preserve anti-rollback state and source identity across crashes, restart,
   reinstall/upgrade and interrupted installation. Keep a known valid snapshot
   until a replacement is fully admitted; classify rollback, corruption and
   genuinely new publisher recovery explicitly rather than resetting history.
5. Qualify signer/root rotation, authenticated record removal, collection reset,
   snapshot/delta replacement and algorithm/format transition. A failed update
   cannot silently disable known applicable distrust or publish partial state.
6. Restrict DNS, destinations, redirects, HTTP behavior and delegated Windows
   retrieval under OPEN-8. Apply bounded backoff and server retry guidance; own
   actual completion, cancellation, files, handles and admission permits.

**Wrong time:** signing-certificate validity, HTTPS, CTL/CRL validation and feed
age may depend on different clocks. A provisional TLS-time argument does not
move every OS check. Define trusted time/uncertainty and monotonic-age handling
for startup, suspend, rollback and restart. If required currentness cannot be
established, use the selected unavailable/Blocked behavior; no global clock step,
date-search bypass or silent authentication downgrade follows from this plan.

Pin first-install trust independently of the downloaded content. Account for
offline distribution, update-service failure and recovery without assuming a
working browser, interactive account, or user-warmed Cryptnet cache. Qualify
storage ACLs and service identity; public security data still needs integrity.
Telemetry may expose bounded revision, age class, decision and failure cause;
do not log keys, cookies, raw certificate payloads or sensitive query history.

## 7. CRLite correctness and dependency candidates

The inspected implementation uses Clubcard, not the original Bloom-cascade
format. Encoding 3 uses optional bincode; encoding 4 has a separate codec [S16].
Choose supported encodings from actual distribution evidence and migration
needs. Old-format rejection must be an explicit compatibility result.

Construct the key from the issuer of the validated leaf and the exact DER serial
integer-content bytes, preserving required leading bytes. Verify SCT signatures
against authenticated CT-log keys and bind coverage to that leaf/issuer. The
rustls 0.23 callback has no separate SCT input; embedded-SCT parsing and the
trusted-log-key lifecycle need an explicit owner. Freeze the qualifying SCT
count/log/time/coverage rules instead of copying an unexplained browser setting.

Current Firefox's CT-enabled path passes `verifiedScts`; its CT-disabled path
deliberately permits unverified embedded timestamps [S15]. That exception and
the diagnostic CLI's unchecked extraction are not adopted here. Log-policy/key
changes must invalidate dependent coverage evidence as defined by the owner.

Aggregate every applicable full/incremental filter: any `Revoked` wins over a
`Good` from another filter. Preserve `NotCovered` and `NotEnrolled` as inconclusive.
Expired certificates can be omitted from filters; independent certificate-date
validation remains required. Clubcard exactness applies inside its encoded
universe, not to arbitrary input outside qualified coverage [S15–S17].

Select the all-revocation-reasons `default` channel for initial qualification.
The inspected `compat` channel includes only key compromise, cessation of
operation and privilege withdrawal; it must not silently replace that scope
[S18]. Record coverage for the actual NTS peer population. CT ingestion lag can
affect certificates older than a brief new-issuance interval [S20].

| Candidate | Source-backed fit and remaining review |
| --- | --- |
| Mozilla `clubcard-crlite` 0.6.2 / `clubcard` 0.3.4 [S16–S17] | MPL-2.0 filter libraries. `with_hash` permits a caller-supplied SHA-256 key; disabling default `sha2` offers a CNG-digest path. Qualify exact features, format support, licence composition, parser/resource behavior and all transitive code; no graph is admitted here. |
| `rustls-upki` / `upki` at the inspected revision [S21] | Useful reference/status-layer candidate, not a drop-in Windows verifier: it selects WebPKI roots, has permissive uncovered/no-SCT defaults and reads SCT timestamps without signature verification. UPKI also contains portable software SHA-256; its manifest/hash fetch path is not the required signed Remote Settings updater. |
| Mozilla diagnostic `rust-query-crlite` [S18] | Operator inspection tool with an accept-all certificate-fetch verifier and a different updater assurance scope. Do not copy it as production TLS verification or update authentication. |

CNG-compatible hashing is only part of admission: review P-384 signatures, SCT
algorithms, parser dependencies, build scripts, target/CPU/features, maintained
source, GPL-2.0-only engine composition and native cleanup through existing
crypto/dependency owners. No new primitive implementation is requested.

Mozilla's August 2025 report described approximately 4 MB snapshots and 300 kB
per day [S19]; these are historical deployment measurements. Current aggregate
attachment size, selected-channel download, parsed resident memory, staging
peak, disk retention and update traffic are different quantities. Measure them
on the actual candidate; adopt no historical number as a product budget.

## 8. Staged implementation after activation

All rows are **DEFERRED until actual V1 release and the selected roadmap slot**.
Keep implementation/results in the existing crypto and engine tasks, with this
document owning the cross-component contract and TRUST-C01 disposition.
Within these shared stages, implement and qualify OneCRL first, then CRLite.
Reuse admitted update infrastructure where applicable; OneCRL alone does not
complete TRUST-C01 while both feeds remain in the selected scope.

| Stage | Owner and bounded work | Observable exit |
| --- | --- | --- |
| TRUST-W01 — freeze the released baseline | Coordinator with security/platform: record released source/artifacts, actual verifier/association path, service identity, OS matrix, feeds, current primary sources and all decisions below | One reconciled scope and finite budget; no inherited prototype or historical native pass |
| TRUST-W02 — source and policy admission | Crypto/security/dependency owners: admit exact OneCRL/CRLite formats and graph, trust anchors/log keys, typed decisions, rejection-only policy and root-constraint investigation | Reviewed contracts and independent fixtures; unresolved choices recorded before dependent implementation |
| TRUST-W03 — bounded updater and storage | Engine/platform/security: implement authenticated snapshots, anti-rollback, unchanged refresh semantics, atomic installation, lifecycle and resource limits | Actual update-entry tests show healthy installation and hostile/recovery refusal without partial policy |
| TRUST-W04 — selected-path and association integration | Crypto/engine: check overlays against actual validated paths and bind sealed receipts through real KE/exporters/publication/send invalidation | Actual consumers reject applicable distrust/stale authority, preserve healthy NTS and never downgrade |
| TRUST-W05 — Windows and installed qualification | Existing Windows/crypto/package owners: controlled native chains, service caches, egress, wrong time, crash/restart, final protections and resource measurements | Candidate-specific two-family and installed-lifecycle evidence with independent observations and cleanup |
| TRUST-W06 — independent review and disposition | Coordinator and independent reviewer: reconcile scope, security coverage, resource results, root-constraint findings and remaining limits | TRUST-C01: qualified chosen-scope integration or explicit owner defer/replan; no automatic expansion into another research campaign |

Do source-only parsing and state tests before native integration. Windows runs
use owned disposable environments and controlled peers, with existing holds
resolved through their owners. No public NTS endpoint, user's trust store or
production service is an experimental target. Retain 90% security-primitive
coverage and per-critical-file 95% mutation gates where applicable; neither a
green percentage nor a cross-build establishes native policy correctness.

## 9. Required discriminating cases

Each case needs an independent expected result, a useful accepted neighbor,
the actual production entry/consumer and exact source/environment identity.

| Case family | Required observation |
| --- | --- |
| Native success + OneCRL match | Revoke issuer/serial and subject/key paths, including cross-signs and alternate issuer ordering; unrelated valid chain succeeds |
| Native success + CRLite revocation | Covered leaf is rejected despite still-valid older Windows evidence; a covered non-revoked leaf retains ordinary checks |
| Native rejection + overlay `Good` | Revoked, untrusted, wrong-name, wrong-EKU, invalid-date and bad-signature chains remain rejected |
| Inconclusive CRLite | Unlogged/private CA, missing/unverified SCT, uncovered interval, unenrolled issuer, expired leaf and unsupported version remain distinct; no `Good` fabrication |
| Multi-filter conflicts | A later applicable `Revoked` overrides an earlier `Good`; missing delta, duplicate/cross-channel state and partial rollover cannot claim complete coverage |
| SCT/key binding | Wrong leaf, issuer, log key, signature, timestamp and serial representation are detected beside an independently valid input |
| Signed update admission | Wrong signer/collection, altered metadata, invalid chain, bad signature, mismatched hash/size and malformed bounded payload cannot replace valid state |
| Unchanged versus stale | Legitimate authenticated unchanged response retains its proper status; replay, frozen responses and mtime changes cannot renew unavailable evidence |
| Rollback and recovery | Older signed revision, crashes between writes, missing anti-rollback state, corruption, publisher reset and signer rotation follow explicit policy without partial install |
| Native cache/identity | Cold, warm, expired and still-valid old evidence under the actual service identity; disabled CTL updates, stale mirror and blocked delegated egress are observable |
| Root constraints | Match exact roots/keys, pre/post-cutoff leaf dates, usage and alternate chains; private/enterprise exceptions cannot silently create public trust or evade explicit distrust |
| Wrong time and lifecycle | Forward/backward UTC error, uncertain startup, suspend/restart and clock rollback cannot fabricate feed or authentication freshness |
| Association update races | Distrust arrives after TLS validation, before KE publication, before protected send and during active UDP; queued work and repeated cookies cannot retain invalid authority |
| Resource and stop behavior | Oversize/corrupt inputs, slow responses, redirect loops, repeated failures and concurrency pressure stay bounded; cancellation retains custody until actual completion |
| Product status and recovery | UI/diagnostics distinguish rejection, unavailable evidence, holdover and Blocked; accepted recovery restores only qualified authority with no plaintext fallback |

## 10. Open decisions and resource budget

Resolve the following in the existing owner-decision procedure before the
dependent post-V1 stage; this planning record invents no accepted constants.

- Freshness/maximum data age per source; proof of authenticated unchanged
  refresh; outage behavior, first-install policy and offline distribution.
- Update cadence/backoff, detection/reaction delay, active-association invalidation
  and full-reauthentication rules, including unknown age across restart.
- Trusted time/uncertainty for signature chains, feed expiry and SCT coverage;
  acceptable recovery when OS and provisional-time checks disagree.
- Supplementary public-root policy source, exact scope and enterprise/private
  trust behavior; no Firefox-parity claim without those decisions and evidence.
- Required SCT coverage semantics, supported filter encodings, rotation/migration
  and all-reasons channel continuity.
- Numerical maximum bytes/records/filters, RAM/disk/staging, CPU and blocking
  time, concurrency, retries and bandwidth. Measure representative data first.
- **Effort budget: TBD.** The activated execution card must set finite total
  preparation/review/implementation/test time and attempts under
  [development §8–9](development-plan.md#8-performance-limits-and-qualification-environments).
  Preserve consumed effort across retries/handoffs; stop at the declared bound.

### Optional later leaf-evidence substitution

Evaluate separately only after the rejection-only integration qualifies. A fresh
authenticated CRLite `GoodCovered` might replace **only missing native leaf
revocation evidence**, while all other native and overlay checks remain required.
Windows exposes separate end-certificate and CA/root unknown-revocation controls
[S22], but enabling an ignore flag globally is not an accepted implementation.

Require a named owner decision, exact per-element failure attribution and native
controls proving that known revocation/distrust, intermediate uncertainty and
unrelated errors remain fatal. `NotCovered`, `NotEnrolled` and stale data cannot
activate this alternative. This is not a TRUST-C01 prerequisite, current V1
change, or permission to relax OPEN-8/R-05 while that alternative is unqualified.

## 11. TRUST-C01 exit and handoff

TRUST-C01 closes only with a finite, independently reviewed outcome against the
activated candidate: qualified integration of both feeds for the chosen scope,
or an explicit documented owner decision to defer/replan under the existing
decision procedure. Both feeds remain planned until such a decision changes
their disposition. Planning, source review or inconclusive tests do not count as
an implemented feed. A blocked/incomplete attempt cannot silently advance the
queue; deliberate deferral/reordering requires the owner, not an automatic retry.

The closeout records the actual chain-to-association consumer path, source and
dependency identities, independent healthy/hostile results, native/service and
installed scope, measured cost, decision/freshness limits and operational update
recovery. Include the supplementary-root investigation's disposition and make
remaining Firefox-policy differences explicit. Keep optional leaf substitution
separate. No whole-product security, browser parity or release approval follows
from a local feed check.

**Current evidence:** repository/source and primary-document review only.
No product implementation, admitted dependency, dataset installation, native
experiment, coverage/mutation result or feed-enabled release is established.
Before activation, work remains documentation maintenance. TRUST-W01 must
reinspect the released source and dated upstream evidence rather than resume
from these pins without review.

**Independent documentation review, 14 November 2026:** the reviewer approved
the assigned planning scope after the exact OneCRL paired matching and
accepted-path coverage were clarified. The review covers deferral/ordering,
preserved V1 duties, source limitations, update lifecycle and the finite exit;
it supplies no implementation or native qualification evidence.

## 12. Primary sources and inspected revisions

Living pages were inspected on 14 November 2026. Repository URLs below are
immutable inspection pins; they are not dependency-adoption instructions.

- **S01 — original analogy:** [AdGuard CoreLibs issue 2110](https://github.com/AdguardTeam/CoreLibs/issues/2110).
- **S02 — Mozilla eMudhra response:** [20 October 2026 report and Android correction](https://blog.mozilla.org/security/2026/10/20/protecting-firefox-users-following-the-compromise-of-emudhra-intermediate-certificates/).
- **S03 — Microsoft eMudhra response:** [Announcements at `87e1355d`](https://github.com/TrustedRootProgram/Program-Requirements/blob/87e1355dc6acae4fae4f161d218a90cfe4410739/Announcements.md). Use the dated body/table, not a commit title as an effective-date source.
- **S04 — Mozilla Entrust policy:** [31 July 2024 decision](https://groups.google.com/a/mozilla.org/g/dev-security-policy/c/jCvkhBjg9Yw).
- **S05 — Entrust implementation:** [NSS 3.107 release notes](https://firefox-source-docs.mozilla.org/security/nss/releases/nss_3_107.html).
- **S06 — Microsoft Entrust policy:** [February 2025 deployment notice](https://learn.microsoft.com/en-us/security/trusted-root/2025/february-2025).
- **S07 — e-Tugra implementation:** [NSS 3.92 release notes](https://firefox-source-docs.mozilla.org/security/nss/releases/nss_3_92.html).
- **S08 — exact e-Tugra roots:** [Mozilla bug 1842928](https://bugzilla.mozilla.org/show_bug.cgi?id=1842928).
- **S09 — Microsoft e-Tugra policy:** [August 2025 deployment notice](https://learn.microsoft.com/en-us/security/trusted-root/2025/august-2025).
- **S10 — Windows evidence caching:** [Crypt32 CRL semantics](https://learn.microsoft.com/en-us/windows/win32/seccrypto/certificate-revocation-list-semantics).
- **S11 — CTL distribution and policy:** [Trusted roots and disallowed certificates](https://learn.microsoft.com/en-us/windows-server/identity/ad-cs/configure-trusted-roots-disallowed-certificates).
- **S12 — native chain controls:** [CertGetCertificateChain](https://learn.microsoft.com/en-us/windows/win32/api/wincrypt/nf-wincrypt-certgetcertificatechain) and [CERT_CHAIN_PARA](https://learn.microsoft.com/en-us/windows/win32/api/wincrypt/ns-wincrypt-cert_chain_para).
- **S13 — service/cache observation:** [Certificate endpoint validation](https://learn.microsoft.com/en-us/windows-server/identity/ad-cs/configure-certificate-path-validation-revocation-policy).
- **S14 — authenticated collection updates:** [Remote Settings client specifications](https://remote-settings.readthedocs.io/en/stable/client-specifications.html).
- **S15 — Firefox `c72617c0`:** [trust-domain/SCT verification](https://github.com/mozilla-firefox/firefox/blob/c72617c016491f426f33f84b8822aad7fdd532c2/security/certverifier/NSSCertDBTrustDomain.cpp), [revocation storage](https://github.com/mozilla-firefox/firefox/blob/c72617c016491f426f33f84b8822aad7fdd532c2/security/manager/ssl/cert_storage/src/lib.rs) and [RemoteSecuritySettings](https://github.com/mozilla-firefox/firefox/blob/c72617c016491f426f33f84b8822aad7fdd532c2/security/manager/ssl/RemoteSecuritySettings.sys.mjs).
- **S16 — `clubcard-crlite` 0.6.2 at `054c1799`:** [manifest](https://github.com/mozilla/clubcard-crlite/blob/054c1799ae71ff6f7d98a804fa98a4f4b4bedd11/Cargo.toml), [query/encoding](https://github.com/mozilla/clubcard-crlite/blob/054c1799ae71ff6f7d98a804fa98a4f4b4bedd11/src/query.rs) and [codec](https://github.com/mozilla/clubcard-crlite/blob/054c1799ae71ff6f7d98a804fa98a4f4b4bedd11/src/codec.rs).
- **S17 — Clubcard and interoperable format:** [`clubcard` 0.3.4 at `ecca8d1a`](https://github.com/mozilla/clubcard/tree/ecca8d1a9e92f9f3b4b910c05fe11c190836501e) and [C2SP UPKI revocation v1.0.0](https://c2sp.org/upki-revocation@v1.0.0); that format specification does not supply updater authentication.
- **S18 — Mozilla CRLite `36ec3bee`:** [channels and diagnostic use](https://github.com/mozilla/crlite/blob/36ec3beecd5ba6ad0bb27b274689a9505dbdfd08/rust-query-crlite/README.md) and [diagnostic source](https://github.com/mozilla/crlite/blob/36ec3beecd5ba6ad0bb27b274689a9505dbdfd08/rust-query-crlite/src/main.rs).
- **S19 — historical resources:** [Mozilla CRLite deployment report, August 2025](https://hacks.mozilla.org/2025/08/crlite-fast-private-and-comprehensive-certificate-revocation-checking-in-firefox/).
- **S20 — coverage counterevidence:** [Mozilla CRLite issue 392, maintainer explanation](https://github.com/mozilla/crlite/issues/392#issuecomment-5998938521); dated observations and issue closure do not prove current complete coverage.
- **S21 — UPKI `83291b16`:** [rustls verifier](https://github.com/rustls/upki/blob/83291b16cd0ca0d1df5154be799817cc2ac55747/rustls-upki/src/lib.rs), [fetcher](https://github.com/rustls/upki/blob/83291b16cd0ca0d1df5154be799817cc2ac55747/upki/src/fetch.rs), [manifest](https://github.com/rustls/upki/blob/83291b16cd0ca0d1df5154be799817cc2ac55747/upki/src/data.rs) and [software SHA-256](https://github.com/rustls/upki/blob/83291b16cd0ca0d1df5154be799817cc2ac55747/upki/src/sha256.rs).
- **S22 — distinct native policy flags:** [CERT_CHAIN_POLICY_PARA](https://learn.microsoft.com/en-us/windows/win32/api/wincrypt/ns-wincrypt-cert_chain_policy_para).
