# TUF proposal: Windows architecture fit assessment

**Date:** 2026-10-28. **Scope:** read-only assessment of the attached proposal against RWTC-ARCH-001 revision 1.4. **Decision status:** no adoption, dependency selection, or maintained-plan change. **Reviewer:** independent Windows-fit worker; no implementation or Windows runtime evidence.

## First-pass record

This first-pass analysis was preserved before exchanging technical findings with peer reviewers. The coordinator reiterated the requested machine trust-state writer and privileged-handoff question; no other reviewer verdict was used to derive the findings below.

**Assessment:** TUF can fit as an additional authorization layer for acquiring the one complete MSI. It does not replace the installer, publisher signature, service lifecycle, or clock safety policy. Integration is substantial because the current plan expressly has no bespoke privileged auto-updater (§19.5). The existing design does not have a machine update-trust owner, protected download staging contract, or authenticated-target-to-MSI handoff. Their absence is an adoption gap, not a defect in the currently selected manual signed-installer design.

The strongest transferable requirement is to preserve the identity of the authorized release all the way from signed metadata to the bytes Windows Installer consumes. Keeping networking and update authority out of the clock broker is equally important. The proposal's “keep the installed app usable” rule transfers only when the installed time client still satisfies its current safety prerequisites.

### Evidence and limits

- Read the user attachment in full and the relevant plan sections 3–6, 10–15, 18–22; also inspected the current Markdown ASVS profile and selected full clauses in the pinned ASVS JSON.
- Read the pinned website AGENTS.md, testing contract, security contract and candidate amendments, plan-independent-review, plan-architecture, security-build-deployment, and qa-fault-recovery. Read the architecture authority, module-boundary and data-contract references. Two leaf reference paths (build/deployment implementation and fault-recovery phase-contract) are absent at their advertised paths in this pinned snapshot; no runtime testing depends on them here.
- Both workspace and pinned guidance directory reported “not a git repository.” This assessment therefore identifies inputs by file paths and content hashes in findings.json; no clean-tree claim is made.
- Ten focused web opens were attempted, including repeated bounded reads of the TUF specification. The GitHub issue fetch was blocked by the search service; the complete supplied attachment is the direct proposal evidence used by this worker. The coordinator's separate source review should reconcile the live issue.
- No installs, builds, tests, probes, clock changes, external writes, dependency changes, or maintained-document edits occurred. The proposed checks below are unexecuted.

## 1. What the proposal would add

The official TUF specification defines authenticated metadata, role/key continuity, ordered verification and persistence, and target hashes/lengths. It expressly leaves package formats and actual installation to the integrating updater. Application-specific target fields remain the application's responsibility. This assessment uses the live specification labelled version 1.0.37, dated 8 October 2026; it is not an immutable source pin. [W1]

For this product, a target should be a **complete signed MSI**, with an enforced binding to product, release version, architecture, supported OS policy, release channel, and package format. A “launcher” and separate “apps” delegation model is not directly needed: this product deliberately has one lifecycle. Delegations might eventually separate release channels or package formats, but require a demonstrated operational need. The proposal does not justify splitting UI, engine, broker or helper distributions.

Signed release authorization and distribution location should remain separate concepts. An update URL, GitHub response, redirect, filename or checksum from the same unauthenticated discovery path cannot substitute for target authorization. HTTPS, destination policy and bounded HTTP processing still matter even when target authenticity is independently checked: they constrain privacy, resource use and unintended access. These are proposed integration requirements, not claims that TUF itself mandates the product's entire HTTP policy.

## 2. Component ownership options

No option below is selected. They make the authority tradeoff reviewable.

| Option | Discovery and verification | Authoritative machine state | Installation authority | Fit and cost |
| --- | --- | --- | --- | --- |
| A: On-demand worker plus installed helper | Normal-user worker fetches and verifies a preview; helper independently performs bounded offline verification before commitment | Installed elevated helper is the sole writer of protected update trust and version-policy state | Existing signed helper invokes complete MSI lifecycle after final staged-byte checks | Avoids a new persistent service; increases the helper's bounded parsing/crypto surface. Preview is explicitly nonauthoritative |
| B: Dedicated restricted update worker identity | Separate on-demand worker/service identity owns network, TUF parsing and a narrow update-state directory | Dedicated identity writes its own protected state; installer must explicitly trust or independently reverify its evidence | Existing helper, never the clock broker | Better process separation from interactive users, but new identity, IPC, service provisioning and TCB questions. Not free merely because it has no clock privilege |
| C: Future managed complete MSIX | Store/App Installer/enterprise deployment owns its native package acquisition and update flow | Managed deployment owns its state; application cannot silently interpose TUF over every path | Supported platform deployment mechanism | Requires §19.4 first. TUF publication or a voluntary precheck alone would not establish client enforcement on every managed path |
| Current baseline retained | User/admin obtains complete signed installer | Existing installer state, without claiming TUF freshness | Existing installer/helper | Remains the selected plan until an update design is authorized |

### Concrete option A handoff

1. The installed helper's identity, binary location and publisher are already protected by the single-product installation. The normal-user worker uses bounded network input, has no machine-policy, SCM, code-directory or clock rights, and downloads only to its untrusted acquisition cache.
2. A preview may use a readable copy of the latest machine root and metadata, plus its own tentative verification state. It cannot advance authoritative root/version state. The UI must distinguish “candidate found” from “authorized for this machine.” Any serialized VerifiedTarget or success flag from this worker remains an untrusted claim.
3. On an authorized user-initiated installation, the installed helper obtains the machine update lock and loads the latest protected trust state itself. It receives bounded candidate metadata and bytes, not an executable path or arbitrary command to run. It independently authenticates the candidate's metadata continuity from that state, enforces target selection/version policy and performs the required persistence steps. A network fetch is not part of its privileged routine; missing required data returns a bounded refetch/retry request or failure.
4. Helper-controlled final staging is a distinct protected directory with fixed ownership and restrictive ACLs. Copying from the worker cache is not verification. The helper completely hashes and size-checks the final staged file, validates its publisher/signature under the product policy, and seals its identity for the transaction. It rejects reparse/hardlink/parent substitution and treats user-supplied filenames as data, not destinations.
5. The helper invokes Windows Installer on that final local path using fixed, allowlisted parameters. It holds the appropriate file/directory protections and verifies the actual Windows Installer path/reopen behavior in the supported environment. Only after these preconditions does the existing lifecycle quiesce synchronization and perform takeover/replacement.
6. The installed-version record changes only according to the committed MSI outcome; an unknown result is reconciled from authoritative installer and product state before retry. New TUF trust already accepted during this attempt is not reverted simply because installation failed.

The helper thus owns authority during the elevated install. Discovery performs no canonical machine trust-state writes. A future operation that refreshes canonical metadata without installing must be separately named, authenticated and use the same sole writer; it must not silently convert the normal UI into that writer.

This option puts an offline verifier in the helper's trusted computing base. It is a real cost. Use the same reviewed verifier implementation rather than inventing a second protocol; still enforce bounds before parsing. An isolated verifier process is a possible further option only after its own identity, state-write and output-authenticity contract is specified. A worker result cannot become authoritative solely because it arrived over IPC or used a Rust type.

### Clock components retain their current boundaries

- The clock broker receives no update metadata, downloader, archive parser, package installer, arbitrary filesystem path or process-launch operation. Its existing final clock authorization and lifecycle quiescence roles remain narrow.
- The synchronization engine is not the default place to add HTTP/TUF dependencies merely because it already networks. That would couple update availability, parsing and TLS-provider choices to time recovery and enlarge its source/destination surface.
- The UI presents status and initiates the permitted operation; it does not own machine rollback floors or install permission.
- All additional owned binaries, if chosen, belong inside the same complete MSI and version/lifecycle. No separate updater prerequisite or second user-visible product is introduced.

## 3. Actual bytes at the privileged boundary

Microsoft documents WinVerifyTrust as performing a selected trust-provider action; only zero is success. Its Authenticode provider establishes publisher/integrity evidence for the specified file. That is not an application release/channel/rollback decision. The product must separately enforce the expected publisher and relevant signing policy. [W2]

WINTRUST_FILE_INFO can identify an already-open file handle, but MsiInstallProductW takes a package path and property string. It even permits URL/network paths. Consequently, “verified an open handle, then called MSI” is not sufficient proof that the same bytes are installed. **Inference:** the application must constrain the supported call to its final protected local staging path and qualify all subsequent reopens/reads; blindly allowing the API's wider inputs defeats the intended handoff. [W3, W4]

A normal-user cache, even under a per-user ACL, is not a privileged installation authority against a malicious process running as that user. An attacker can change bytes, replace paths, fabricate metadata receipts or race a copy. Required checks concern the complete final artifact after any copy/resume/cache operation, and preserve its identity until the installer has consumed it. The installer must not re-download an artifact by URL after verifying a different preliminary copy.

All required product payloads should remain inside the complete MSI target. The existing restrictions on hostile public properties, transforms, repair sources, custom-action inputs and DLL search remain in force. TUF authorization of a package does not authorize arbitrary invocation parameters, transformations or external code. If additional executable material can be obtained during installation, it requires an explicit authorized-artifact contract and would reopen the current complete-package requirement.

Publisher signing should finish before the repository records the MSI's target hash and length. Any later signing/timestamping or other byte mutation requires recomputing the metadata for the final distributed artifact. This is an identity consequence of hashing exact bytes, not a replacement signing scheme. Microsoft's installer documentation also notes that administrative installation can remove package signatures because it modifies the package; do not assume every transformed installation source retains the original signature or digest. [W5]

## 4. Separate persistence classes and rollback meanings

Name a new **UpdateTrustState** rather than overloading §6's existing time TrustState. They have different meanings and writers.

| State | Authority and lifetime | Required future behavior |
| --- | --- | --- |
| Embedded bootstrap root | Signed initial distribution; initialization only | Used for a genuinely new trust domain, not silently substituted after any error |
| Accepted TUF root and role metadata | Sole update-trust writer; protected machine directory | Persist/reload in protocol order; validate crash recovery; protect from ordinary users and engine writes |
| Product version/channel/security policy | Product/security owner; enforced by helper | Distinguish metadata version, installed release, minimum permitted release and explicitly authorized recovery |
| Candidate/cache files | Downloader; untrusted until final admission | May be deleted/refetched; no authority is lost by treating a cache as disposable |
| Final staged MSI and install transaction | Helper/installer | Bound to target identity, current trust/policy state and transaction; prevent mutation/reuse under changed context |
| Time policy, ownership journal and correction budgets | Existing broker/installer owners | Update cannot reset them, refund correction authority or revive an old clock owner |

TUF's workflow saves accepted root, timestamp, snapshot and targets state, and includes specific state changes during key rotation. The exact library/protocol state machine must be preserved; an informal “all counters always increase” rewrite can conflict with prescribed compromise-recovery semantics. [W1]

The product must distinguish:

1. **Metadata rollback:** reject improperly replayed trust metadata across process restart and machine reboot using retained state.
2. **Application downgrade:** decide which older application versions remain eligible; a higher metadata version can authorize an older application version unless the product adds an explicit rule.
3. **MSI transaction rollback:** recover the compatible prior package/state after a failed update under the existing journal and ownership policy.
4. **Repair or reinstall:** restore integrity without silently reinitializing the update trust domain, time policy or consumed correction authority.

The updater must not set an “installed version” floor merely because metadata for a newer release was accepted; the install may subsequently fail. Conversely, preserving installation availability must not undo accepted root rotation or reinstate a known-revoked release. Define a preauthorized compatible rollback set, a separate committed installed-version record, and an explicit recovery policy. If the preceding release is now forbidden by a security decision, the safe outcome may be blocked repair rather than automatic restoration of that release.

UpdateTrustState must survive ordinary upgrade, repair and compatible package rollback. An older repaired binary must either understand the current state or fail update operations safely; it cannot overwrite newer state with its embedded root. Multiple users and simultaneous updaters need one lock/transaction owner. Missing/corrupt previously initialized state and persistence failures block update admission rather than rebootstrap.

Full uninstall and later reinstall require an explicit retention/reset policy. An administrator intentionally deleting state, offline disk rollback, or a VM snapshot can defeat ordinary filesystem rollback protection; this is already outside the plan's prevention boundary (§12.1). Do not claim permanent device-wide antirollback without additional mechanisms. A documented fresh-install reset and a normal repair must not be conflated.

Microsoft's rollback model attempts restoration after failed installation, but direct external custom-action effects need authored reversal and rollback actions do not run when rollback is disabled. This confirms that TUF does not remove the existing §11/19 transaction obligations. [W6]

## 5. Availability, time and managed-package semantics

An update metadata, network, publisher, persistence or download failure should leave an otherwise valid installed time client running. Before installation admission, there is no reason to stop the services. Report update health separately from synchronization authentication, agreement, freshness and uncertainty. Do not show an update failure as a loss of NTS trust, or a successful update check as proof of correct UTC.

“Keep usable” cannot override §10 ownership checks, invalid mandatory broker policy/journal, exhausted correction budgets, competing W32Time/GPO control, unsupported mixed component versions, or a known-invalid installed release. An admitted complete-product update deliberately quiesces services; a failed one uses the existing recovery transaction and may need an honest blocked state. Never create two running clock owners to preserve apparent availability.

TUF needs a defensible fixed update-start time. The clock client already distinguishes wall-clock values, monotonic deadlines, authenticated-time history and weak bootstrap hints. A separate review must define the updater's time source, provenance and long-offline policy. Neither a broker RPC nor a UI boolean proves true UTC. Updates must not acquire a raw clock-setting route, relax certificate validity or treat metadata expiry as permission to change system time. This is a cross-worker seam, not a resolved time-design decision here.

For a future complete MSIX, Microsoft documents that App Installer settings can block activation, allow up/down-level updates, use fallback URIs, and be overridden by management settings. These controls have their own owner and policy. **Inference:** an app-level TUF precheck cannot promise enforcement on every Store/App Installer/managed repair path; any such claim needs an actual platform interception/ownership design. Do not silently select activation blocking or downgrade behavior inconsistent with the time-service availability policy, and do not override managed deployment ownership. [W7]

## 6. Exact maintained surfaces if adoption is later authorized

These are a change map, not edits or accepted design decisions. Reuse current BND/THR/CHECK meanings; allocate new identifiers only during an approved revision.

| Current surface | Required delta |
| --- | --- |
| §2 DEC-9/13/15; §1 REQ-4/9 | Preserve complete MSI and publisher signing; record update-channel, trust-owner and lifecycle decision separately |
| §§3–5; BND-5/6/8; F8/F9 | Add worker/helper update boundary, effective identities, allowed operation and immutable staged-artifact handoff; retain broker exclusion |
| §§6.1–6.5 | Define bounded metadata/target records, private authenticated-target construction, signed field meanings and final contextual checks; do not blindly impose the product's JSON schema rules on TUF without protocol/library compatibility review |
| §§8–10 | Define verified update-time source and error behavior; preserve monotonic runtime deadlines and all clock-authority limits |
| §§11–12 | Add UpdateTrustState writer, protected paths, lock/commit/recovery, repair persistence and distinct metadata/application/install rollback contracts |
| §14 assets/actors/flows and invariants | Extend A-6/F9 to include repository, root/release keys, metadata availability and downloader; model delivery compromise, stale metadata, malicious cache/UI, worker compromise, handoff race and repair reset |
| Existing threats | Extend THR-6/9/15/16/17/20/23/25/26; preserve THR-11/12/13/19 clock recovery boundaries. Add explicit update freshness/root continuity/authorized-target threats if existing entries become too broad |
| §15 | Qualify new executables and signature/provider paths, temporary/install DLL loads, effective token/egress/child-process policies |
| §18.2–18.4, 18.6–18.10 | Qualify exact client/publisher dependency graph, compatible license/provider/features, crypto/time/network surfaces, supply-chain evidence and expanded critical-file manifest |
| §19.2–19.5 | Define authorization before quiescence, exact final signed package, signature rotation, target selection, persistence, rollback/reinstall/repair and managed-package exclusions |
| §20 CHECK-13/15/18/23/25/26/27 | Add end-to-end installer handoff, trust persistence and failure cases; CHECK-1/2/3/16/17/21/22 retain relevant isolation/diagnostic/resource/UI evidence |
| §21 S0.6, P3/P4/P6/P7 | Add a bounded non-production update experiment and dependent admission gates; do not mark original P0 blocked merely because a future updater is undecided |
| §22 OPEN-1/3/4/8/11/14 | Extend existing license/identity/IPC/TLS/signing/lifecycle questions; add one scoped update-lifecycle decision for root bootstrap, ownership, time, offline window, metadata renewal and key recovery |
| Diagrams, review and ASVS profile | Update affected flows and companion diagram parity if adopted; independently review the exact revision, preserving unimplemented/unexecuted status |

### Concrete proposed checks

| Family | Hostile/fault observation | Legitimate neighboring observation |
| --- | --- | --- |
| Trust persistence | Accept root/key update, terminate before/after durable stages, restart/repair with old embedded root; no trust reset or stale metadata acceptance | Valid continuity and a subsequent permitted release still work |
| Worker/helper authority | Fabricated VerifiedTarget, modified preview metadata, different user, concurrent old worker; no install or machine trust write | Authorized helper re-verifies and commits valid complete MSI |
| Byte identity | Cache swap, changed resumed file, copy race, reparse/hardlink/parent swap, post-hash mutation, URL re-download; no unverified executable reaches MSI | Actual observed installer consumes the authorized final artifact |
| Signature separation | Valid TUF target with wrong publisher; valid publisher on unapproved/old target; signed package plus hostile transforms/properties | Correct target and publisher pass with permitted parameters |
| Version/recovery | Failed update after accepted metadata; repair from older source; unsupported downgrade; old/new process overlap; no trust-floor or step-budget reset | Qualified rollback preserves one clock owner and compatible state |
| Update availability | Expired metadata, bad clock, network outage, full disk, UAC cancel; existing valid sync does not gain false status or lose safety | Fresh valid attempt recovers; healthy time sync remains independent |
| Publisher operation | Wrong role/threshold, missing intermediate roots, stale metadata publication, compromised online key, post-signing byte mutation | Independent approvals, rotation, renewal and published final hashes work |
| Managed route | Alternate platform update/repair path bypasses voluntary app check or conflicts with policy | Selected managed complete-package lifecycle passes its own gates |

Use independently generated expected artifact bytes/hash, exact release/metadata fixtures, actual Windows identities and protected effects, healthy controls, fault stage identities and authoritative receipts. Unit verification cannot qualify MSI path behavior, service state restoration or native ACLs. These remain proposed tests only.

## 7. ASVS 5.0.0 delta scope

No worksheet classification is changed here. IDs below were checked against the current profile; the disputed URL/HTTP/path/crypto/TOCTOU clauses were read in the pinned JSON. Existing status remains unimplemented/unexecuted.

| Exact rows (all v5.0.0) | Reassessment if update client is adopted |
| --- | --- |
| 1.2.2 | Current Proposed N/A explicitly assumes no dynamic URL construction; reopen for metadata/target URL construction and allowed schemes |
| 4.2.1, 4.2.2, 4.2.3, 4.2.5, 12.2.1, 15.3.2 | Currently Needs decision for certificate retrieval; add actual update HTTP parser/version/framing/length/TLS/redirect owner and evidence. No HTTP server is implied |
| 1.2.5, 1.5.2, 1.5.3, 2.2.1, 2.2.2 | Metadata/parser limits, signed representation, helper input and fixed installer invocation; no normalization that changes signature meaning |
| 5.3.2, 8.2.1, 8.3.1, 13.2.2, 15.2.5, 15.4.2 | Protected staging, real helper authority, least privilege and actual verified-byte handoff. Do not present a serialized success flag as authorization |
| 2.3.1, 2.3.2, 2.3.3, 2.3.4, 15.4.1, 15.4.3 | Ordered trust persistence, single writer, concurrent update exclusion, unknown install result and atomic lifecycle transitions |
| 11.2.1, 11.4.1, 11.6.1, 12.3.2 | Selected verifier/provider, hashes/signature algorithms and ordinary transport certificate policy; publisher signing remains separate |
| 13.2.4, 13.2.5 | Metadata/target hosts, redirects and any delegated certificate retrieval; downloads do not inherit unrestricted network authority |
| 15.2.1, 15.2.2, 15.2.3, 15.2.4 | Exact update dependencies, maintained versions, bounded root/delegation/download work, no release backdoors and origin/promotion controls |
| 16.2.1, 16.3.3, 16.3.4, 16.5.1, 16.5.2, 16.5.3 | Update provenance/error codes separate from time status; safe diagnostics and failure/recovery behavior |
| 2.3.5, 4.1.5, 13.3.1, 13.3.3 | Explicitly assess publisher approval and signing-key custody. Threshold metadata signatures do not settle every high-impact action, hardware-isolation or IPC-signature question |
| 5.3.3; 14.2.1 | Conditional review only: new untrusted archive extraction could create a native adaptation; sensitive URL material could reopen confidentiality scope. TUF alone does not introduce either surface |

The canonical current worksheet is the Markdown revision 1.4. The nearby CSV retains older rationales in inspected rows; it must not silently override the current Markdown or be regenerated as part of this evaluation. Any future authorized reconciliation should preserve the documented revision history.

## 8. Open choices and permitted next step

The evaluation supports a bounded design comparison or disposable prototype **if later authorized**, focused on option A's canonical state writer and actual MSI handoff. It does not establish implementation readiness or justify adopting a particular Rust client.

Unresolved owners: product/security choose update and downgrade policy; release/security choose root/release thresholds, key custody, renewal/offline support and compromise recovery; installer/platform choose the sole trust writer, staging and MSI handoff; protocol/security define defensible update time; dependency/license owners qualify a compatible library graph. Publisher operations and long-offline recovery are continuing obligations, not a one-time library integration.

## Primary-source registry

- **W1:** The Update Framework specification, live 1.0.37: https://theupdateframework.github.io/specification/latest/ — §§1.4, 4.5, 5.1–5.7, 6.1. Source for limited protocol facts; Windows integration proposals are reviewer reasoning.
- **W2:** Microsoft WinVerifyTrust: https://learn.microsoft.com/en-us/windows/win32/api/wintrust/nf-wintrust-winverifytrust — Authenticode provider action and exact success result.
- **W3:** Microsoft WINTRUST_FILE_INFO: https://learn.microsoft.com/en-us/windows/win32/api/wintrust/ns-wintrust-wintrust_file_info — optional open-file handle plus required path.
- **W4:** Microsoft MsiInstallProductW: https://learn.microsoft.com/en-us/windows/win32/api/msi/nf-msi-msiinstallproductw — path/URL and property input, not a verified-handle installer contract.
- **W5:** Microsoft digital signatures and Windows Installer: https://learn.microsoft.com/en-us/windows/win32/msi/digital-signatures-and-windows-installer — package/resource signatures and administrative-install mutation.
- **W6:** Microsoft rollback custom actions: https://learn.microsoft.com/en-us/windows/win32/msi/rollback-custom-actions — direct system-change reversal, interruption and disabled rollback.
- **W7:** Microsoft App Installer auto-update and repair: https://learn.microsoft.com/en-us/windows/msix/app-installer/auto-update-and-repair--overview — activation/update/repair and management policy surfaces.
- **P1:** Proposal supplied by user: /workspace/scratch/af0a4bbf5e7b/upload/Pasted text(20261028-141654).txt. Referenced issue: https://github.com/storytold/craft-launcher/issues/888 (fetch unavailable to this worker).
- **P2:** Current architecture: /workspace/scratch/af0a4bbf5e7b/deliverables/windows-time-architecture.md, revision 1.4.
- **P3:** Current applicability: /workspace/scratch/af0a4bbf5e7b/deliverables/asvs-5.0.0-applicability.md; original clauses: /workspace/scratch/af0a4bbf5e7b/architecture-research/asvs-5.0.0.json, canonical source pin 5cf9b032440be53ce345ab3c130fda46ba1ce7a2, https://github.com/OWASP/ASVS/blob/5cf9b032440be53ce345ab3c130fda46ba1ce7a2/5.0/docs_en/OWASP_Application_Security_Verification_Standard_5.0.0_en.json.
