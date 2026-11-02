# Windows protected optional storage: execution and qualification plan

**Document:** NV-STORAGE-PLAN-001, revision 1.2<br>
**Date:** 3 November 2026<br>
**Owner:** David Osipov<br>
**Status:** Maintained design and future qualification plan. Recording this direction is authorized; implementation, native experiments, exact cryptographic choices and the product default are not approved by this document.<br>
**Initial design baseline:** [`de3777230d2adc30fa9eea9895e7dcbb370dec1e`](https://github.com/DavidOsipov/NevroTick/tree/de3777230d2adc30fa9eea9895e7dcbb370dec1e). Revision 1.1 records the first upstream research against [`1e58ef9255e13d62a9fdcd2542c4472e4ea3c8ea`](https://github.com/DavidOsipov/NevroTick/tree/1e58ef9255e13d62a9fdcd2542c4472e4ea3c8ea). Revision 1.2 incorporates the second source review and agent-task routing against [`6b28711f5510e3550039d5fcf8bbcf3ccba791a5`](https://github.com/DavidOsipov/NevroTick/tree/6b28711f5510e3550039d5fcf8bbcf3ccba791a5). These were API-derived partial snapshots, not local checkouts.

## 1. Authority, purpose and decision status

[Architecture §12.1.2](../windows-time-architecture.md#1212-protected-optional-storage-and-explicit-fallback) owns the technical contract. This plan owns staged evidence, review reconciliation and the next bounded work. The [development plan](development-plan.md), [owner-decision procedure](../nevrotick-owner-decisions.md), [process mitigation contract](../windows-time-architecture.md#154-per-process-mitigation-profiles-and-startup-admission) and [DNS plan](system-dns-cache-design.md) retain their authority. Scoped ST-T references below map to existing BND/THR/CHECK and DNS-C owners; they create no competing acceptance catalogue.

The predecessor is the independently reconciled `working-design-v1.md`, SHA-256 `f5f5fca6faee869b5350ad26a3b3c31be42ab541bd93a19faf78e96ae4876a76`. The completed report, *NevroTick protected-storage design review — 2 November 2026*, has SHA-256 `917cb5234f1c498de1dcac37c0831774a6942c9da6c5696047aa969d04bed4c2`. Its audit and hardening reconciliations closed documentary findings at that candidate; they established no implementation or Windows result.

The preferred candidate protects optional saved NTS associations and, separately, future DNS learning state using qualified CNG VBS-protected root keys. Previously authorized compatibility policy may permit a separately qualified TPM root set for **fresh optional state** when VBS is unavailable, then operation without an application-managed persistent secret cache. Automatic persistent DPAPI, ordinary software-key and plaintext fallback are excluded from this direction. The earlier DPAPI-first DNS snapshot proposal is superseded **as the preferred candidate to evaluate**, not by an assertion that its replacement is implemented or qualified. VBS/TPM ordering is a product preference with different threat boundaries, not a universal security ranking. [P1–P6]

| Decision or claim | Current disposition |
| --- | --- |
| Record VBS-unavailable/off fallback and visible notification | Requested planning direction; retained here and in architecture. |
| Narrow root-key protection goal | Candidate scope recorded for qualification; no promise that the whole engine runs in an enclave. |
| Separate wrapping and signing roots | Preferred construction to evaluate; algorithms, provider support and supported alternative constructions remain open. |
| `PreferVbsAllowTpm + ContinueWithoutRestore` default | Engineering recommendation, not an accepted owner choice. |
| Stronger live-secret isolation in a custom application enclave | Separate future decision and workstream. |
| Native support, durability, performance, ASVS or release readiness | Unqualified; source review and reviewer agreement cannot establish them. |

A CNG VBS-protected key is distinct from a custom application VBS enclave. Working DEKs, TLS exporters, NTS keys and plaintext may remain in ordinary engine RAM. Protecting storage roots does not satisfy complete-memory encryption or all-cryptographic-operations-isolated requirements. Host compromise, authorized malicious key use, selective offline replacement of ACL-only trust anchors and full-environment rollback remain explicit limitations; recording them is not owner acceptance of residual risk. [P4–P6, P11–P14]

The architecture and original plan were recorded before upstream research. The owner then requested that the completed findings also be recorded: [upstream implementation lessons](windows-protected-storage-upstream-lessons.md) now preserves pinned source observations, proposed adaptations, unsuitable sample policies and future checks. This completes the initial source inventory, not the construction/provider/build/account/profile freeze. It starts no implementation, dependency acquisition, key creation, service operation, experiment or process-policy change. Existing held TLS/CNG dependency and fixture decisions remain held.

**3 November follow-up:** the owner requested the [second upstream review](windows-protected-storage-upstream-lessons-2.md)
be integrated into documentation, this plan and [the agent task](../../tasks/windows-protected-storage.md).
Its concrete refinements concern operation outcomes, admitted directory access,
durability modelling, per-call CNG behavior and mitigation evidence. RedirectionGuard
is a candidate; attestation, hardware revocation epochs, chunking and a database
remain conditional reserves. This documentation authorization does not lift the
experiment pause, choose a product default or advance a native acceptance gate.

## 2. Owners, data classes and restoration

| Owner | State and permitted effect | Required separation |
| --- | --- | --- |
| Installer/elevated lifecycle helper | Installation identity, service identity, fixed namespaces, owner/DACL baseline and authorized provisioning/recovery transaction | Existing narrow verbs and policy revisions; no arbitrary path/provider interface. |
| Engine storage manager | Provider operations, catalogue, bounded root/snapshot transitions and optional NTS snapshots under the final service identity | No clock/debug privilege or general privileged decryption service. |
| Broker | Mandatory policy, correction ledger, ownership and durable reservation/outcome reconciliation | No engine secrets; optional cache loss never refunds spent/uncertain authority. |
| Future DNS state owner | Independently admitted keys, catalogue, aggregates and privacy generation | Process topology remains the DNS owner's decision; shared source does not imply shared keys or authority. |
| Unelevated UI | Authenticated redacted status and notice acknowledgement | Cannot authorize fallback, reset, provider choice or provisioning. |

Optional NTS restoration must pass [§7.4](../windows-time-architecture.md#74-nts-implementation-contract), [§12.1.1](../windows-time-architecture.md#1211-restoration-validity) and current source/trust/algorithm/policy admission. Preserve complete association state and independent key/full-authentication/resumption ages; unknown elapsed age follows the conservative policy. Restore no outstanding exchange, reuse no old request identity, and obtain fresh admissible observations. Decryption does not establish current authentication, agreement, UTC truth or clock authority. Exact lifetime choices remain with their existing owners.

Loss of optional NTS state permits fresh acquisition only under unchanged NTS and bootstrap rules: cached association where admitted, strict KE, then the separately qualified opt-in provisional-time path. It never enables implicit plain NTP, weaker identity checks, a discarded inconvenient authenticated floor or a clock write. TLS resumption persistence is not added here. Saved floors, leap/trust material and drift priors retain their own provenance, expiry and supersession contracts.

DNS demand history is optional private optimization state. Loss may produce an empty learning model; it cannot relax DoQ, certificate pins, DNSSEC or routing policy. Ordinary DNS service remains independent of statistics persistence.

Use two nonsecret durable control records. The lifecycle owner's **installation witness** distinguishes `Provisioning(transaction-id)`, `Initialized` and explicitly authorized `Recovery(transaction-id)`. Matching initial catalogue publication is acknowledged durably and consumes that provisioning authority; an interruption reconciles the same transaction. Missing expected catalogue/key in an initialized installation means recovery, while a missing optional snapshot may simply mean fresh acquisition.

The sensitive-state owner's **catalogue** binds installation/store/purpose, actual provider and key identities, public fingerprints, generated/imported origin, use/export policy, evidence category, policy/privacy revisions and active/prepared/retired disposition. It admits no enumerated “newest” key or envelope-supplied verification identity. Copying its assertions into broker storage would not make them attested.

`NoPersistentCache` explicitly represents **no expected provider key and no optional secret snapshot**. It must work when no usable root can be generated. Nonsecret lifecycle/privacy metadata can remain necessary to retire old state, preserve privacy generations and distinguish deliberate absence from unexplained key loss.

## 3. Complete policy product and truthful state

Keep platform capability, authorized policy, evidence for the actual roots, persistence readiness, association/learning readiness and required process protection separate. Mechanisms are `VbsKeyProtected`, `TpmKeyProtected` and `NoPersistentCache`. Lifecycle states include Uninitialized, Preparing, Ready, Unavailable, RecoveryRequired and Blocked; absent trusted evidence is Unknown, not another backend.

The two proposed axes are permitted persistence (`PreferVbsAllowTpm`, `VbsOnly`, `PersistenceDisabled`) and behavior when it is unavailable (`ContinueWithoutRestore`, `SuspendSensitiveOwner`). All six pairs have explicit outcomes:

| Policy pair | When permitted persistence is unavailable | NTS and DNS consequence |
| --- | --- | --- |
| PreferVbsAllowTpm + ContinueWithoutRestore | Admit a fresh qualified TPM epoch only for classified capability fallback; otherwise no persistent cache | Fresh KE/current admissible live association may continue; optional learning follows current nonpersistent privacy policy. |
| PreferVbsAllowTpm + SuspendSensitiveOwner | Qualified TPM is allowed after admitted capability fallback; otherwise wait until an allowed root set is Ready | Suspend affected acquisition/use/publication and optional learning while neither is ready; resume only with current root/data generation and ordinary admission. |
| VbsOnly + ContinueWithoutRestore | No TPM persistence; operate without restoration until qualified VBS returns | Fresh KE/current admissible associations and permitted memory-only learning may continue. |
| VbsOnly + SuspendSensitiveOwner | Required VBS condition blocks the affected owner | Suspend new acquisition/use/publication, invalidate affected work and withdraw source readiness; suspend optional learning. |
| PersistenceDisabled + ContinueWithoutRestore | Explicitly no persistent secret cache | Continue under ordinary NTS and authorized nonpersistent learning/forget rules. |
| PersistenceDisabled + SuspendSensitiveOwner | **Rejected contradictory combination** | No implicit interpretation or hidden default. |

The table governs first admission, restart and live loss. Stop new affected persistence and invalidate stale workers before applying continuation/suspension policy. Strict startup can initialize an explicitly empty store; it does not require existing NTS secrets to create the root that permits fresh acquisition. Ordinary DNS and ownership-aware cleanup remain available under the applicable independent contracts. Preserve native-job custody through actual completion.

Required process-mitigation failure independently blocks normal affected role work. Storage fallback cannot compensate for it. Configuration/status/API shapes are proposals to be frozen later, not protocol constants introduced by this plan.

## 4. Candidate construction, commit and restart

### 4.1 Authenticated records and independent identity

Evaluate separate provider-held nonexportable private roots for DEK wrapping and whole-envelope signing. Encrypt each independent snapshot attempt with a fresh DEK under an admitted AEAD; authenticate the **exact canonical envelope** under an independently admitted verification identity. Both roots must satisfy the claimed protection mode. A mixed root set cannot display the stronger mode. [P1–P6]

The signed representation binds domain separation, format/version, installation/store/purpose, root/data/privacy/policy epochs, snapshot sequence, algorithm identifiers, wrapped DEK, nonce, ciphertext and tag. The verification identity comes from the admitted catalogue/provisioning chain, never an attacker-replaceable public key in the envelope. Public-key wrapping plus AEAD alone allows an attacker to choose a DEK and construct an internally valid invented record. A smaller independent-secret authentication alternative may be evaluated only if it meets the same trusted-writer property.

Freeze suite, sizes, padding, signature/prehash encoding, digest, canonical bytes, RNG, nonce issuance/use limits, sequence arithmetic and input budgets before implementation. Signature verification and committed-content hashing must cover the same unambiguous representation. Failed publication, retries, concurrency and restored state must not reuse a forbidden key/nonce pair; RNG/nonce failure blocks persistence, without weak seeds or alternate primitives. Complete-environment RNG/state rollback remains a separate limitation.

For that selected suite, require exact returned-length checks and independent
known-answer or interoperability fixtures beside roundtrips. A shared wrong
hash/KDF/context/encoding must fail the declared contract. The [CNG examples](windows-protected-storage-upstream-lessons-2.md#6-cng-admission-per-call-silence-real-lengths-and-explicit-parameters)
do not select ECIES or introduce a KDF absent from the chosen construction.

Admission order is bounded outer framing, fixed allowlist/current key-reference admission, independent envelope authentication, protected DEK opening, AEAD verification, bounded typed semantic validation and current-generation publication. The envelope selects no arbitrary provider, loader, path or algorithm. Apply the repository's strict decoder, mandatory private construction and current-effect guards; authenticated malformed or unauthorized input still fails.

### 4.2 Epochs, durable publication and bounded inventory

A **root epoch** identifies long-lived provider roots; a **data/association epoch** identifies current NTS/DNS state; a **snapshot sequence** identifies one save. Routine KE refresh/snapshot publication uses the existing qualified root and fresh data-key handling, without elevation or root rotation. Mode changes, root compromise, installation/account changes and root rotation are separate lifecycle operations.

One storage manager serializes transitions. Bound inventory to one active root set, one prepared set and at most one attributable retired set, with equivalent bounded snapshot slots. Failed cleanup occupies its slot; capacity exhaustion stops further transitions.

1. Admit current policy/profile/lifecycle transaction, fixed destination and available slot; retain bounded intent.
2. Create roots without overwrite, under the final token and provider-supported restrictive descriptor/finalization sequence. Verify origin/use/export/access policy; imported keys cannot claim never-host-exposed origin.
3. Prepare a bounded private destination snapshot or explicit empty store. Prefer fresh KE and rebuilt/empty DNS learning over migration of previously exposed state.
4. Durably publish one catalogue generation selecting the roots and exact eligible snapshot or empty state. Bind authenticated store/root/data/privacy identity and snapshot sequence to a digest of the **exact committed envelope**. This is the logical commit point for restore eligibility and reported active mode.
5. Revoke old eligibility, retire and clean only attributable objects; report residual cleanup failure. Acknowledge only after the declared durable point.

Ordinary saves use the same prepared-content/durable-reference/current-generation pattern without root rotation. An old validly signed snapshot under the same root/data epoch must fail against an intact current catalogue; the exact committed snapshot must succeed. This protects selective snapshot replacement only while the catalogue remains trustworthy.

| Interrupted condition | Required restart outcome |
| --- | --- |
| Prepared roots/snapshot without committed reference | Keep current generation; reconcile or clean attributable preparation, never adopt by existence. |
| Commit complete, cleanup incomplete | Only new generation restores; old decryptable state remains retired. |
| Initialized installation with absent/corrupt/conflicting catalogue or expected key | RecoveryRequired; no key enumeration, overwrite, fresh-install inference or downgrade. |
| Native completion after policy/privacy/root invalidation | Retain custody until actual completion, reject publication and wipe owned sensitive results. |

Windows does not gain a claimed atomic multi-object transaction from this logical protocol. Exact flush/replace/provider durability, crash reconciliation and witness acknowledgement remain gates. Encryption/signatures do not defeat authenticated rollback, authorized signer compromise, selective offline catalogue/witness replacement or whole-machine snapshots. An independent rollback-resistant anchor would require a separate design. Mandatory broker state never joins this transaction.

### 4.3 Operation outcomes and owner usability

Apply the architecture's separate operation and owner contracts: definitely not
applied, confirmed committed receipt, or outcome requiring reconciliation;
independently, the affected transaction/owner may be usable, poisoned,
recovery-required or closed. A possibly durable error must reconcile the same
operation identity and exact committed-content evidence before a dependent
retry. It grants no root recreation, destructive migration, generation reuse or
refunded authority. A rejected poisoned transaction is not automatically a
poisoned whole store. Hold real native resources through reconciliation until
completion. [redb's source pattern](windows-protected-storage-upstream-lessons-2.md#4-an-error-is-not-always-evidence-that-nothing-happened)
informs ST-T06/07/10/16/17; database readability cannot grant NTS eligibility.

Use an independent bounded model of issued writes, live bytes, durable bytes
and acknowledged effects, plus a monitor of the publication/retirement protocol.
Distinguish requested, successful and failed barriers. Exercise a failure during
recovery and a second failure; stop at each simulated crash cut so later
artificial-success returns create no false durability obligation. Preserve
mandatory continuity and privacy acknowledgements under their existing owners.
[redb/SQLite examples](windows-protected-storage-upstream-lessons-2.md#5-model-durability-separately-then-monitor-the-operation-protocol)
define useful cases, not native Windows durability results.

## 5. Platform admission, failure classification and privacy

Qualify finite API/phase/build/provider/algorithm/profile rows. Required-VBS creation uses `NCRYPT_REQUIRE_VBS_FLAG`; `PREFER_VBS` may fall back to software. Provider-wide properties, names and unrelated probe keys do not prove protection of the actual key. Microsoft's example uses Microsoft Software KSP for required-VBS keys; the provider name alone proves neither outcome. `NCryptOpenKey` documents no REQUIRE_VBS reopen flag. Controlled origin, bound public identity and supported cross-boot reopen evidence must establish both actual roots; otherwise report Unknown/RecoveryRequired. [P1–P5]

| Failure class | Treatment |
| --- | --- |
| Qualified VBS unavailable/off/unsupported result in an authorized fresh optional epoch | Policy may admit fresh TPM or NoPersistentCache, with actual mode/reason. |
| Existing VBS state becomes inaccessible | Keep it unavailable; any permitted fallback commits distinct fresh state. Never keep a weaker second wrap of the same secrets. |
| Signature/tag/schema/key identity failure, denial or missing expected key | RecoveryRequired; no backend downgrade caused by that error. Policy may allow fresh volatile acquisition or explicit same-protection recovery. |
| Transient/slow native operation or I/O failure | Bound same-generation handling, stop affected publication and retain custody; timeout is not cancellation completion. |
| VBS returns | Serialized new root/data epoch if policy permits; retired blobs do not regain eligibility. Rewrapping cannot erase prior exposure. |
| Mandatory state continuity or required process protection fails | Existing class-specific refusal/recovery; no extra trust, correction budget or relaxed profile. |

Examples remain contextual: `NTE_VBS_UNAVAILABLE` at required creation does not prove deliberate disabling; `NTE_EXISTS` requires reconciliation; opening an expected key with `NTE_BAD_KEYSET` is recovery. Unknown errors/bad flags must not strip required options, try arbitrary providers or prompt interactively from a service. Avoid oscillation and per-packet capability probing. [P1, P3]

Freeze documented flags, actual key policy, account/session and refusal behavior
per native call; silent opening does not qualify later private use. Bound the
property-query allocation and both advertised/actual returned lengths; validate
version/flags and safely decode fields, retaining absence/unsupported/denied/error
distinctions. The future [hardening matrix](../windows-time-development-hardening.md#641-protected-storage-provider-qualification)
includes these controls and exact-parameter fixtures.

Preserve [architecture §15.4](../windows-time-architecture.md#154-per-process-mitigation-profiles-and-startup-admission) and [hardening §6.4](../windows-time-development-hardening.md#64-per-process-mitigation-qualification). Qualify create/open/wrap/sign/unprotect/teardown with the actual SCM path, final account, no-login boot and required controls. Keep ACG no-opt-out/no-remote-downgrade, ordinary versus Strict CFG, effective CET/HSP versus image marking, CIG signer classes, Win32k/extension points, child restrictions and image-load policy distinct. Do not introduce a permissive launcher, extra privilege, interactive Hello dependency, or machine-policy reset. Respect the documented NCrypt service-start caveat. Full-enclave imports/ABI/signing/debug/strict-memory support is a separate gate if that path is selected; neither blanket Rust incompatibility nor compatibility has been established. [P1, P7–P12, P15–P16]

Independently qualify file, directory, catalogue and provider-key owners/DACLs, inheritance, handle rights and replacement. Challenge another LocalService service, including owner-mediated ACL changes; service SID and machine key scope alone prove no isolation. Use the already allowed distinct restricted engine account if the required shared-account isolation cannot be established. [P15]

Give save/restore only admitted directory authority and bounded relative object
names. Validate actual opened objects before destructive effects; qualify
replacement and installer lifecycle with retained handles. [cap-std's example](windows-protected-storage-upstream-lessons-2.md#3-borrow-directory-authority-and-delayed-destruction-from-cap-std)
is not a claim that every library operation is handle-relative. Separately
investigate [RedirectionGuard](windows-protected-storage-upstream-lessons-2.md#2-redirectionguard-one-additional-windows-defense-worth-investigating)
under architecture §15.4; its narrow junction scope does not replace these checks
or establish a new required policy/support floor.

DNS forget/pause/exclusion/persistence-off uses its durable privacy generation and completion barrier. Before acknowledging completion, invalidate active/prepared/retired snapshots, inaccessible VBS blobs, readers, queued writes and delayed results. Asynchronous deletion must obey that completion contract and report residuals. VBS return cannot resurrect forgotten names. Missing/conflicting generation state requires empty current-policy reacquisition. No forensic archive is proposed; failed cleanup remains bounded and visible. DNS-C11/13/14/15 retain these obligations.

NoPersistentCache excludes application-created restart snapshots, not Windows pagefile, dumps, hibernation, provider caches or backups. No plaintext staging or secret-bearing logs/DTOs is permitted. Checked VirtualLock and wiping may reduce exposure of specific allocations, not all copies or whole-process memory. BitLocker, paging/dump policy and other machine settings are environmental controls, not silent repairs. [P13]

## 6. Production status and notification

At D2 re-entry, use bounded authenticated status carrying store class, service/boot instance, sequence, policy revision, public epochs, actual mechanism/evidence category, persistence and owner readiness, reason and freshness. Distinguish last success from current unavailability. Exclude raw key/container names, secrets, DNS history and arbitrary provider diagnostics. An authenticated service report is not attestation against a compromised service.

The VBS message must explain that storage keys are protected while working data is processed in service memory. TPM fallback must identify fresh-state protection and lost restoration; no-cache mode must name NevroTick's restart cache. Unknown/stale/disconnected status cannot look protected or current. Explain what continues or is suspended without inferring that a user deliberately disabled VBS.

Show persistent status and a meaningful transition notice. On the **first subsequent authenticated UI connection**, present the current fallback/recovery/blocked condition even if it arose during headless boot. Deduplicate by store/state generation/reason; acknowledgement suppresses repeated announcements, never the persistent indicator or policy. Preserve [UI-M0](../windows-time-architecture.md#165-restricted-first-ui-milestone-ui-m0): no live status, persistence or production controls are introduced now.

## 7. Focused threat scenarios and acceptance mapping

These are future distinguishing controls, not executed results. Each rejection case needs an admissible neighbor at the actual consumer. BND-6 owns persisted-state admission; BND-3/4/5/7/8 remain independent authority and delivery boundaries.

| Reference and threat | Required distinguishing observation | Existing mapping |
| --- | --- | --- |
| ST-T01: false VBS claim | Ordinary software keys, TPM roots with fresh DEKs, PREFER fallback and stale/unproven reopen status never show VBS; qualified actual roots do. | THR-18; BND-4/6; CHECK-14/16/17/22 |
| ST-T02: failure-induced downgrade | Corruption, denial and missing expected keys cannot select a weaker backend; classified authorized capability fallback can. | THR-3/17/18; CHECK-9/15/25; DNS-C13 |
| ST-T03: shared-account access | Second LocalService token cannot read/use/replace keys or rewrite owner-controlled ACLs; intended service succeeds. | THR-6/9/10; BND-6; CHECK-1/15 |
| ST-T04: compromised key consumer | Malicious valid-shaped engine requests cannot exceed broker authority; permitted bounded operation succeeds. Root nonexportability does not prevent authorized key use. | THR-5/10/19; CHECK-1/3/10/12 |
| ST-T05: record/key/context substitution | Validly encrypted attacker-created envelope, substituted signer and cross-store/root/epoch records fail; independently authenticated original succeeds. | THR-9/17/25; CHECK-15/25 |
| ST-T06: authentic rollback | Unknown continuity after old-state/VM restoration blocks unsafe actuation and preserves charged authority; legitimate restart follows current rules. Undetectable authentic rollback/offline anchor replacement remain residuals. | THR-17/19; CHECK-10/11/12/15 |
| ST-T07: optional reset applied to mandatory state | Missing/corrupt journal and failed durable reservation block unsafe effects; healthy reservation/effect/outcome works. | THR-12/16/17/19; CHECK-10/12/15 |
| ST-T08: stale NTS trust | Unknown/expired ages, changed trust/source and restored pending exchanges fail; compatible state sends only current protected traffic. | THR-1/3/4/11/17; CHECK-6/9/11/15/27 |
| ST-T09: wrong-date bootstrap bypass | Cache loss and wrong-name/chain certificates never authorize plain discipline or erase floors; qualified authenticated recovery works. | THR-3/4/17; CHECK-4/8/9 |
| ST-T10: crash/late-completion race | Every prepare/flush/commit/retire interruption preserves declared eligibility, bounded resources and no stale publication; healthy commit survives restart. | THR-16/17/25/26; CHECK-12/13/15/25/27; DNS-C13/15 |
| ST-T11: enclave-interface confusion | If selected, malformed/racing shared inputs cannot disclose secrets or cross operation authority; bounded copy-in/validation/copy-out succeeds. | THR-10/25/26; CHECK-15/25/27 |
| ST-T12: weakened mitigation | Storage incompatibility cannot relax required controls; actual protected SCM/provider workflow and denial controls qualify together. | THR-14/15; BND-8; CHECK-16/18/20/23 |
| ST-T13: false origin/upgrade identity | Imported roots cannot claim never-host-exposed origin; unadmitted debug/wrong-version readers fail; supported upgrade and retirement work. | THR-10/14/15/17; CHECK-15/18/23 |
| ST-T14: secret/statistics disclosure | Sentinel keys/cookies/names stay absent from default logs, temp/backup/status/export artifacts; permitted aggregate status remains useful. | THR-9/10/18; CHECK-15/17/22; DNS-C14 |
| ST-T15: forgotten history returns | Delayed results, retained readers and inaccessible old VBS blobs cannot cross the privacy barrier; fresh permitted learning works. | THR-17/25/26; DNS-C11/14/15 |
| ST-T16: parser/provider exhaustion | Bounded parsing, queues, jobs, retries, handles and logs retain charges until real completion; admitted work and shutdown remain usable. | THR-8/26; CHECK-17/21/27; DNS-C12/13 |
| ST-T17: destructive recovery deputy | Unauthorized/stale reset, account/path change and deletion have no effect; authorized optional reset preserves mandatory state. | THR-6/9/16/20/22/23; CHECK-2/3/13/15/23; DNS-C15/17 |
| ST-T18: AEAD key/nonce lifetime | Concurrent/failed/retried/restored writes obey the frozen suite; RNG failure blocks persistence and healthy independent writes recover. | THR-10/17/25; CHECK-15/25/27 |

## 8. Staged execution and evidence gates

Follow the repository [planning guide](../../.agents/skills/planning/README.md) and [testing contract](../../.agents/skills/testing/CONTRACT.md). The [storage task](../../tasks/windows-protected-storage.md) is the single bounded execution/evidence chain, with exact source/dirty candidate, scope, owners, controls, environment, resource/time budgets, observed outcomes and cleanup. No new product crate is needed to validate this documentation.

| Stage and owner | Work and observable exit | Readiness |
| --- | --- | --- |
| **Source-qualified design freeze** — storage/security/platform | Use the [first](windows-protected-storage-upstream-lessons.md) and [second](windows-protected-storage-upstream-lessons-2.md) pinned source inventories. Freeze one construction/provider/algorithm/build/account/profile row, or retain a precise blocker. Include exact signed bytes/digest, no-root state, key reopen evidence, directory authority, descriptor/finalization order, operation outcomes and recovery/durability design. Prepare concrete unresolved owner choices. | Both bounded source inventories recorded; design freeze remains open. Research approves no dependency, default or native execution. |
| **Portable owner/adapter model** — storage/testing | After relevant contracts and implementation scope are admitted, exercise private construction, independent error/outcome and live/durable models, protocol monitor and compiled healthy/defect controls. Include crash-during-recovery, exact crypto fixtures and late-result privacy rejection. | Planned; implementation needs its own authorization and admitted inputs. Portable evidence neither lifts the native pause nor qualifies actual provider/filesystem behavior. |
| **Bounded native root qualification** — platform/security | Only after the experiment pause is lifted and the frozen row is ready, use owned disposable Windows fixtures and dummy namespaces. Demonstrate intended create/use/reopen/retire, unauthorized second-service denial, actual mode evidence, no-login SCM operation and intact required profile. Record supported and unsupported rows. | Held; no experiment currently authorized. |
| **S2.3/S2.4 association restoration** — NTS/security | Integrate admitted storage with complete association/lifetime/current-trust rules and strict/bootstrap recovery. Demonstrate useful compatible restart beside inaccessible, stale and well-formed unauthorized records; preserve bounded native work. | Depends on provider/construction and existing NTS/OPEN-8 gates; CHECK-6/8/9/17/25/26/27 as applicable. |
| **S3.3 persistence and lifecycle** — platform/service/security | Qualify witness consumption, catalogue/exact-envelope commit, migration/retirement, every interruption and actual handler authorization. Simulated clock backend suffices for storage work; broker authority remains separate. | CHECK-1/2/3/14/15/17/25/27; real clock effects remain P4. |
| **Future DNS storage** — DNS/privacy/platform | Use independent keys and owner; exercise ordinary DNS beside learning failure, privacy barriers and retained inaccessible blobs. | DNS-C11–15/17 within the DNS phase plan; no reordering of current Windows work. |
| **P5/D2 status** — UI/service/security | Integrate coherent current status, first-connection notice, reconnect, strict-policy suspension and acknowledgement behavior; retain accessible bounded announcements. | D2 remains deferred until its existing re-entry trigger; CHECK-14/17/22/25. |
| **Package/release qualification** — build/release/platform | Reconcile final payload, providers/imports, actual mitigation manifest and account/upgrade/repair/removal behavior for declared release profiles. | CHECK-13/16/18/20/21/23/26; no release evidence supplied here. |

Before native execution, explicitly freeze the provider's **wrapping and signing** support, Windows/SDK/CPU/VM builds, generated versus imported origin, cross-boot/silent operation and evidence for each reopened key. API base-version minima, full-enclave minima and CNG VBS support are different facts. Earlier source review found different generic-enclave/tooling support statements and a prerelease notice on NCrypt VBS flags; requalify the selected path rather than inherit a universal minimum. [P1, P11–P12]

The native matrix must cover VBS-off/re-enable, TPM/account/key loss, wrong-date/offline startup, disk-full, creation collisions, prepared/retired inventory exhaustion, supported restart/upgrade neighbors and every durable boundary. Test exact committed snapshot acceptance beside an older valid signature under the same root/data epoch. Preserve genuine native completion, inspect artifacts/resources before teardown, and restore all owned resources. No public time service or global clock mutation is needed for this storage qualification.

Pure state-machine/decoder/model evidence, cross-builds, native provider results, installed-service results and final-package results stay separate. Use relevant configured tooling and existing coverage/mutation obligations when implementation exists; a source review cannot pass those gates. Requalify affected evidence when provider, algorithms, key origin, account, serialization, Windows build, mitigation profile or package changes.

Use the task's ordered work items to refine existing cases: transaction/durability
work under ST-T05/06/07/10/15/16/18; directory authority under ST-T03/05/10/17;
CNG properties, silence and independent crypto expectations under ST-T01/02/05/12/13/16/18;
mitigation startup/behavioral evidence under ST-T12; completion and output privacy
under ST-T10/14/15/16. The mapping retains section 7's CHECK/DNS-C authority and
creates no new acceptance catalogue. The next documentation action is one
source-qualified construction/error/ownership decision record, not tool acquisition
or a native experiment.

## 9. Review reconciliation and remaining choices

| Review identity | Maintained correction | Status carried forward |
| --- | --- | --- |
| A1 / HR-09 | Narrow NoPersistentCache promise, explicit absent roots, OS persistence limits and truthful UI | Closed in reviewed specification; schema/memory evidence still open. |
| A2 / HR-02/04/07 | Lifecycle witness, consumed provisioning, real key/file ownership, separate epochs, bounded slots and commit/restart rules | Documentary gap closed; native isolation/durability unqualified. |
| A3 / HR-10 | Five defined policy pairs, one rejected pair; startup/restart/live effects, strict cold-start path and independent DNS/cleanup | Closed at final reviewed hash; default remains unselected. |
| A4 | First authenticated UI connection displays already-active fallback/recovery/blocked notice | Closed future D2 contract; no UI-M0 implementation claim. |
| A5 | Signature includes sequence; committed exact-envelope digest and identity apply to ordinary saves too | Closed design correction; selective replacement of catalogue/witness remains residual. |
| HR-01 | Independent whole-envelope authentication and anchored verification identity | Construction candidate retained; exact suite/provider proof open. |
| HR-03/05/06/08 | Independent mitigations, actual-key evidence, host-memory boundary and rollback/offline-anchor limits | Retained invariants; no native or ASVS closure. |
| ST-T15 / ST-T18 | Inaccessible-history privacy barrier and explicit DEK/nonce/RNG lifecycle | Design obligations retained; execution open. |
| 3 November second source review | Exact operation outcomes, directory/native-adapter contracts, durability monitor and mitigation cases; [agent task](../../tasks/windows-protected-storage.md) owns follow-up | Source lessons recorded; future implementation/native evidence remains open. |
| Conditional reserves | RedirectionGuard disposition; attestation/origin, hardware epochs and bounded anchor/disk recovery; chunking/database only if justified | No selected mechanism or dependency. Same-epoch replay, offline anchor replacement and full VM/vTPM rollback limits remain; no new mandatory-broker construction. |

The independent audit closed A1–A5; the independent hardening reconciliation resolved HR-10 and found no remaining blocking contradiction within its bounded final review. These findings apply to the identified predecessor, not an unexecuted future implementation. This maintained condensation preserves their obligations; its documentation integration receives its own scoped consistency review.

Before dependent adoption, use NV-DEC-001 for the product default/strict-policy choice, any stronger in-use goal, unsupported construction alternative or changed trust boundary. Routine documentary corrections need no additional approval. Exact cryptographic parameters, provider/build support, native durability and ownership evidence remain engineering gates, not invented accepted owner decisions.

## 10. Primary-source bibliography

Sources below were inspected in the completed 2 November 2026 design review and are retained for traceability. They state platform contracts; the source-qualified freeze must establish applicability to its selected row. The [first upstream implementation review](windows-protected-storage-upstream-lessons.md) and [second source review](windows-protected-storage-upstream-lessons-2.md) preserve their separate completed inventories, exact revisions and adaptation limits. Source inventories supply no native execution evidence.

| Ref | Primary source and relevant boundary |
| --- | --- |
| P1 | [NCryptCreatePersistedKey](https://learn.microsoft.com/en-us/windows/win32/api/ncrypt/nf-ncrypt-ncryptcreatepersistedkey): VBS flags, persistence, collisions and service-start caveat. |
| P2 | [Key storage properties](https://learn.microsoft.com/en-us/windows/win32/seccng/key-storage-property-identifiers): provider/key properties and scope. |
| P3 | [NCryptOpenKey](https://learn.microsoft.com/en-us/windows/win32/api/ncrypt/nf-ncrypt-ncryptopenkey): reopen flags and silent operation. |
| P4 | [NCryptDecrypt](https://learn.microsoft.com/en-us/windows/win32/api/ncrypt/nf-ncrypt-ncryptdecrypt), [NCryptSignHash](https://learn.microsoft.com/en-us/windows/win32/api/ncrypt/nf-ncrypt-ncryptsignhash), [NCryptVerifySignature](https://learn.microsoft.com/en-us/windows/win32/api/ncrypt/nf-ncrypt-ncryptverifysignature): caller buffers and algorithm-specific authentication. |
| P5 | [VBS key protection](https://learn.microsoft.com/en-us/windows/security/book/identity-protection-advanced-credential-protection), [Microsoft CNG VBS example](https://techcommunity.microsoft.com/blog/windows-itpro-blog/advancing-key-protection-in-windows-using-vbs/4050988): narrow key boundary and provider example. |
| P6 | [How Windows uses TPM](https://learn.microsoft.com/en-us/windows/security/hardware-security/tpm/how-windows-uses-the-tpm), [CNG providers](https://learn.microsoft.com/en-us/windows/win32/seccertenroll/cng-key-storage-providers): key custody/provider model. |
| P7 | [Dynamic-code policy](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_dynamic_code_policy): ACG and weakening options. |
| P8 | [CFG policy](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_control_flow_guard_policy): CFG/Strict CFG distinction. |
| P9 | [User shadow-stack policy](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_user_shadow_stack_policy): effective CET/HSP state. |
| P10 | [Binary-signature policy](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_binary_signature_policy): actual signer predicates. |
| P11 | [VBS enclaves](https://learn.microsoft.com/en-us/windows/win32/trusted-execution/vbs-enclaves), [development guide](https://learn.microsoft.com/en-us/windows/win32/trusted-execution/vbs-enclaves-dev-guide): separate enclave requirements. |
| P12 | [VbsEnclaveTooling at ec3ea90](https://github.com/microsoft/VbsEnclaveTooling/blob/ec3ea90ddac245da7c96112dea0e27ee1a78f986/README.md): pinned Rust/tooling/build/strict-memory support. |
| P13 | [VirtualLock](https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtuallock): limited locked-page guarantee. |
| P14 | [EnclaveUnsealData](https://learn.microsoft.com/en-us/windows/win32/api/winenclaveapi/nf-winenclaveapi-enclaveunsealdata), [sealing identity policy](https://learn.microsoft.com/en-us/windows/win32/api/ntenclv/ne-ntenclv-enclave_sealing_identity_policy): plaintext and code-identity boundaries. |
| P15 | [Service SID](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/ns-winsvc-service_sid_info), [object ownership](https://learn.microsoft.com/en-us/windows/win32/secauthz/owner-of-a-new-object), [image-load policy](https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process_mitigation_image_load_policy): distinct access/load boundaries. |
| P16 | [Exploit protection reference](https://learn.microsoft.com/en-us/defender-endpoint/exploit-protection-reference): mitigation context; the repository role manifest remains authoritative. |
