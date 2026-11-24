# Custom Windows VBS enclave: boundary and feasibility plan

**Date:** 24 November 2026.<br>
**Status:** Requested planning; conditional product proposal, with the next source-only stage defined below. No custom enclave is implemented or admitted by this document.<br>
**Inspected NevroTick baseline:** `5a142812b260c33493f8ec970e68d2d223333dbc`, initially clean.<br>
**Evidence and continuation owner:** [the existing protected-storage task](../../tasks/windows-protected-storage.md#8-custom-enclave-planning--24-november-2026). This supporting plan adds no task queue or acceptance catalogue.

## 1. Purpose, authority and current evidence

David requested planning after supplying Microsoft's [development guide][E2].
The proposed useful boundary is **NTS association custody and its fixed packet
operations**: retain admitted directional keys inside an enclave, protect a
request and authenticate its reply without returning those keys. Protecting one
request is the first prospective vertical slice; association admission, reply
handling and retirement are required surrounding contracts, not unrelated demos.
This is a candidate to assess, not a decision to move the whole engine into VTL1.

[Architecture §12.1.2](../windows-time-architecture.md#1212-protected-optional-storage-and-explicit-fallback)
still owns optional storage. Its [C1 card](../../tasks/windows-protected-storage.md#7-st-w01-documentary-card--10-november-2026)
proposes CNG-held wrapping/signing roots; that protects different material and
does not isolate TLS exporters or working NTS keys. C1's byte, account, provider
and durability freeze remains open. The custom enclave neither replaces C1 nor
inherits its proposed VBS/TPM/no-cache fallback as a live-secret policy.

The accepted [administrative boundary](../windows-time-architecture.md#421-service-management-admission-and-administrative-recovery)
and [threat scope](../windows-time-architecture.md#141-method-scope-and-evidence)
remain. A future enclave could enforce its **own protected-key operation**
against a hostile host under qualified isolation and authorization assumptions.
It cannot force compromised host code to obey a returned decision, prevent an
administrator using other clock-setting routes, or make all machine
configuration immutable. Changing that threat/authorization policy needs
[NV-DEC-001](../nevrotick-owner-decisions.md#6-od-04--decisions-when-evidence-makes-them-concrete)
before the dependent product adoption; it does not block preparing this plan.

### Actual source and future consumers

| Inspected owner | Established behavior and prospective seam |
| --- | --- |
| [Engine feature gate](../../crates/nevrotick-engine/Cargo.toml) and [facade](../../crates/nevrotick-engine/src/lib.rs) | `ke-association-prototype` is default-off and scoped to the existing experiment. No custom-enclave composition or production secret supplier is present in this inspected chain. |
| [KE transport](../../crates/nevrotick-engine/src/nts_ke_experiment/transport.rs) | Host Rustls produces both exporters into `Zeroizing<[u8; 32]>`; `InternalCandidate::publish` transfers them with the same connection/profile/endpoint. A later enclave import cannot undo that host exposure. |
| [Association owner](../../crates/nevrotick-engine/src/nts_ke_experiment/association.rs) and [ledger](../../crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs) | Complete keys, cookies, source/profile, original expiry and request custody belong together. Future opaque enclave association/request identities must preserve those relationships and consume-once rules. |
| [Directional request/reply operations](../../crates/nevrotick-engine/src/nts_ke_experiment/request.rs) | `seal_context` and `open_response` are bounded, direction-specific seams with staged authenticated outputs. Their keys currently occupy engine memory; the existing fixture/provider graph is not qualified inside an enclave. |
| [Source policy](../../crates/nevrotick-engine/src/source_policy.rs), protected datagram and measurement owners | Currentness, actual send/receive, endpoint policy and eventual sample admission remain distinct consumers. An enclave result grants no destination enforcement, independent source vote, UTC accuracy or broker authority. |
| [Slint policy workflow](../../crates/nevrotick-ui/src/slint_host/policy_workflow.rs) | Fixture selection/review has no production Apply port. There is no implemented production configuration handler to relocate. P3 and P5/D2 retain that work. |

This is a source observation, not a new native execution receipt. Current
[development §7](development-plan.md#resume-in-dependency-order--23-november-2026),
the storage pause, scoped native/fuzz holds, D1–D3 triggers, direct MSIX decision
and post-V1 formal-work deferral retain their existing scope. Planning adds no
enclave prerequisite to V1 or a place in the accepted post-V1 ordering.

## 2. Claim and data boundaries to freeze

The prospective claim is limited to **confidential custody and authorized use
of the selected enclave-owned association material**. State exactly when its
protection begins and which outputs become public. No result is qualified yet.

| Data or authority | Required owner and explicit limit |
| --- | --- |
| Directional keys, exporter inputs and key schedules | Enumerate creation, temporary copies, import, use, replacement and destruction. Existing host exporters permit only an explicitly labelled after-import experiment. A never-host-exposed product claim needs a feasible enclave-owned TLS/exporter path or another independently authenticated confidential ingress. No such route is selected; moving TLS/certificate verification is substantial separate feasibility work. |
| Cookies and packets | Keep association/cookie mutation with the protected operation. Allow only the protocol-required outgoing packet and admitted reply fields; some cookies/fields necessarily appear on the wire. No generic secret export, decryption or arbitrary signing interface. |
| Machine policy and policy changes | The broker retains ordinary machine-policy authority. An enclave must independently validate the exact policy governing its own operation. A host-supplied `authorized`, `current` or `validated` assertion, a replaceable public key, package signature or UAC result is insufficient against a hostile host. |
| Personal preferences and public configuration | Keep ordinary preferences with the UI; hiding theme or display settings supplies no useful secret boundary. Inventory sensitive fields individually instead of encrypting all configuration under one claim. |
| Mandatory journals, trusted floors and correction allowances | Retain broker/continuity owners. Enclave loss, optional reset, update or reacquisition cannot erase or refund them. Public compiled leap data keeps its release/domain owner. |
| Time, freshness and rollback evidence | Host wall time and host-asserted generations are untrusted for the stronger hostile-host claim. Specify which enclave-owned or independently evidenced facts establish lifetime/current policy. Sealing, code identity, an attestation report and an image security version alone do not establish newest application state or resistance to full VM/environment rollback. |

**Change authority has two materially different options.** Preserving existing
elevated-administrator authority permits an authorized administrator to request
allowed changes; the enclave still validates their shape, exact content and
current transaction. Refusing a machine-controlling administrator requires an
authorization root that administrator cannot replace or invoke arbitrarily,
plus provisioning, revocation, rotation and unattended recovery. An external
signer, enterprise authority or interactive user-bound mechanism is an
unselected alternative, not an implicit solution. The source-only card must
compare these options and identify the dependent owner decision. No mandatory
Windows Hello, remote attestation service or new key role follows from this plan.

The platform/secure-kernel/hypervisor and selected code/signing trust basis must
remain intact. A hostile host can withhold work, kill the process, remove required
platform support, control network/storage delivery and observe allowed outputs.
Retain availability, traffic/timing and side-channel limits without promising
complete-memory secrecy or administrative control of the whole product.

## 3. Operation and interface contracts

The names below are documentary operations, **not selected Rust APIs or ABI
exports**. The next card freezes one versioned interface, actual byte/count/work
limits and enforcement owners before implementation; no sample numeric defaults
are adopted. Reuse the existing NTS/domain rules rather than reimplementing
protocol cryptography or adding an arbitrary command dispatcher.

| Operation and caller | Required input/current-state guard | Output, effect and failure contract |
| --- | --- | --- |
| Admit association — future qualified KE owner | Complete provenance-bearing transfer: identity/endpoint, trust/profile revision, algorithm, both directions, bounded cookies and original lifetime; qualified ingress and current enclave policy | Publish one private association and opaque instance/generation-bound identity, or publish none. Raw keys, partial associations and caller-declared provenance cannot escape or become Ready. |
| Protect one request — existing association/request owner, then datagram sender | Exact admitted association and operation identity, bounded supported header/context, current policy/lifetime and remaining cookie/request budget; qualified RNG/nonce ownership | Return only one bounded protocol packet and its bound request identity after charging the operation. Preserve spent state on lost output. No arbitrary plaintext/algorithm/provider request and no send/clock authority. |
| Authenticate one reply — datagram receiver | Copied bounded packet bound to the exact outstanding request, source/association/direction and current generation; authenticate before semantic publication | Return only admitted protocol fields; retain cookie/key mutation inside the owner. Invalid, duplicate or stale input cannot publish plaintext, replenish capacity or become a fresh sample. Existing measurement/Core guards remain outside. |
| Change operation policy — future admitted policy transaction | Exact versioned content, installation/purpose, expected revision and independently verified change authority at the enclave | Atomic policy transition and affected-association invalidation; rejected precommit input preserves prior state. A missing acknowledgement requires same-transaction reconciliation, not assumed rollback or replay. |
| Retire — association/service lifecycle owner | Exact instance/association/request generation; serialized revocation of new work | Prevent stale publication before acknowledging retirement. Preserve buffers/charges until actual completion; wiping or failed deletion cannot be reported as completed native teardown. |
| Optional seal/restore — later S2.3/S2.4 storage owner | Admitted purpose, identity/version, source/trust/lifetime/policy and current continuity; bounded authenticated bytes | Restore only into enclave-private state after eligibility checks. Never restore an outstanding exchange. Unavailable/corrupt/old state follows its declared recovery policy and cannot reset mandatory state. |

**Memory and generated-code admission:** copy the fixed call context, validate
length arithmetic and per-call/aggregate limits **before allocation or unpacking**,
then copy into enclave-owned memory and validate that representation. Never
retain or reread a host pointer as trusted state; a copy can contain hostile or
racing bytes and grants no origin, snapshot consistency or authorization.
Authenticate exact copied content where the contract requires it. Validate
callback results, aliasing, output lengths and transformed values; expose only
allowlisted bounded outputs after commit/currentness checks. Rust private checked
construction, shared domain rules, release-active guards and explicit typed
errors apply at the actual consumer, including generated and FFI paths.

**Concrete upstream adoption gate:** at Microsoft pin [E3],
`call_vtl1_export_from_vtl1` copies the host context and allocates
`vec![0u8; in_size]` before calling the developer handler; its callback-return
path likewise allocates using host `ret_size` before unpacking [E4]. A bound in
the business handler is too late for those allocations. Before adopting that
runtime, the selected ABI owner must supply reviewed early byte/nesting/work/
output and aggregate-allocation admission or choose a narrower qualified path.
EDL generation and a successful round trip do not satisfy this gate. This is an
upstream source observation, not a vulnerability finding in NevroTick.

Use strict memory and the supported copy accessors; the restriction API can
relax access at runtime, so final source/import/call-path inspection must exclude
a production relaxation path [E5]. No general host callbacks, logger objects,
filesystem/network handles or ambient policy readers belong inside the trusted
operation. Any necessary callout must have its own bounded untrusted-result and
reentrancy contract before selection.

## 4. Failure, lifecycle and recovery

Reuse ST-W02/ST-W06's separation of **operation outcome**, **owner usability**,
publication eligibility and actual native completion. Distinguish proven
pre-effect refusal, confirmed same-operation commitment and indeterminate
outcome requiring reconciliation. A failed transport/copy-out after commitment
does not refund a cookie, request identity, sequence or policy revision.

| Condition | Required prospective behavior |
| --- | --- |
| Unsupported/unavailable platform, failed evidence query, image/import/signature/debug/strict-memory/mitigation/init refusal | Do not admit the enclave owner. Report the specific bounded evidence/readiness class; Unknown is not a stronger mode or proof of deliberate VBS disabling. No automatic ordinary-memory, CNG, TPM, DPAPI or plaintext substitute satisfies this live-secret goal. The unavailable-mode policy remains an owner decision. |
| Malformed, foreign, stale, revoked, overbudget or reentrant request | Refuse before its protected effect; preserve prior valid state, bounded error/diagnostics and usable legitimate work. Check aggregate outstanding work as well as each call. |
| Cancellation, deadline, policy change or disconnect around a call | Revoke publication and new work at the correct generation; retain native call/input/output custody and charges until actual completion. A timeout is no proof of cancellation or noncommit. |
| Lost committed result or copy-out failure | Reconcile the original operation within a frozen bounded receipt policy; never mint a new operation to conceal uncertainty. Retired or ambiguous state cannot regain eligibility through retry. |
| Crash during create/load/init/use/seal/commit/acknowledgement/retire/recovery | Retain issued/live/durable/acknowledged distinctions. Repeated recovery failure stays RecoveryRequired; no fresh-install inference, silent key recreation, duplicate protected effect or mandatory-state reset. |
| Upgrade, author/key rotation, stale sealing key, account/machine change or old valid state | Apply explicitly selected reader identity/version and migration policy; preserve old evidence until qualified reconciliation. Refuse incompatible restoration while permitting authorized fresh acquisition only under unchanged NTS/trust policy. Full-environment rollback remains separately unresolved. |

`CallEnclave(TRUE)` can wait for an available enclave thread;
`TerminateEnclave(FALSE)` returns without waiting, and `DeleteEnclave` can fail
while enclave threads remain [E6]. Freeze wait/fail policy, finite admission,
reentrancy, shutdown deadline, poisoned-owner handling and cleanup observations.
Do not copy the sample's indefinite panic loop or infer completed destruction
from a dropped wrapper. Observe application cleanup before fixture teardown.

If persistence is later selected, `EnclaveUnsealData` must use a checked
**enclave-private plaintext destination**; the API also permits host-memory
output. Its stale-key flag needs a bounded reseal/recovery rule because old
platform keys are retained only finitely [E7]. Exact-code, primary-code, image,
family and author sealing policies admit different readers [E8]. Freeze the
chosen scope, minimum accepted version, revoked-release behavior, rollback
limit and every interrupted migration before writing a real record. No sealing
policy is selected merely because it makes upgrades convenient.

## 5. Platform, build and signing admission

These are separate gates, checked against current primary sources on
24 November 2026. Older individual API minima do not widen the current supported
VBS-enclave deployment row. End users do not need developer tools.

| Gate / existing owner | Exact source-supported requirement and unresolved qualification |
| --- | --- |
| Runtime — Windows/platform, W11-W05 | Learn specifies Windows 11 **26100.2314+** or Windows Server **2025+**, with VBS/HVCI enabled [E1]. Qualify actual Windows 11 26H2 x64 and Server 2025 x64 Desktop Experience separately under [OD-14's matrix](../../tasks/windows11-integration.md#432-two-family-windows-qualification-matrix--5-november-2026), final service identity, CPU/VM and effective protections. A client/Sandbox/version-string result is no Server or installed-service pass. |
| Basic developer path — build/platform | Learn lists MSVC/VS 2022 **17.9+**, SDK **10.0.22621.3233+**, VEIID and signing tools [E1]. Freeze exact compiler/linker/SDK libraries/bindings and tool hashes, not just minimum versions. |
| Current SDK/Rust candidate — dependency/build | Microsoft pin **23716f9639a64050a0133b65c3edc5f886d1c54a** advertises Rust **1.88+**, a Windows 11 **26100.3916+** SDK row and SDK **26100.7463+** for user-binding APIs [E3]. Select only a justified subset; its client-only table supplies no Server SDK qualification or Hello requirement. No package/graph is admitted. |
| Enclave ABI/runtime — build/native adapter | Define `IMAGE_ENCLAVE_CONFIG`, primary image, family/image identifiers, image/security versions, finite memory/thread capacity and `DllMain`; freeze calling convention, pointer/length widths, allocator, panic/failure behavior and enclave-compatible imports. Microsoft's Rust sample uses `no_std` plus `alloc`; its bindings rewrite selected imports to `vertdll.dll` [E9]. Ordinary `std`, Windows bindings, TLS or fixture dependencies cannot be assumed compatible. |
| Actual crypto and panic graph — Engine/S2.2, dependency/build | The current Engine prototype selects Rustls with `std`, `rustls-cng-crypto`, `rustls-platform-verifier` and the path-only `nevrotick-s115-aes-siv-fixture`; workspace release uses `panic = "unwind"`. Freeze the complete reachable TLS/exporter, certificate-verifier, RNG, NTS AEAD algorithm-15, allocator and panic/import graph for the enclave. General Rust/BCrypt support qualifies none of that composition; no blanket migration, algorithm replacement or silent panic-policy change is selected. |
| Compiler/linker and imports — build/security | Guide requires `/ENCLAVE`, `/NODEFAULTLIB`, `/INCREMENTAL:NO`, `/INTEGRITYCHECK`, CFG and `/GUARD:MIXED`; link enclave CRT/runtime/UCRT plus supported `vertdll`/`bcrypt`, with VEIID binding platform import identities [E2]. Qualify compiler instrumentation and final PE metadata/imports, not flags alone. Reconcile with [current Rust 1.100/v3/bundled-LLD policy](../../.cargo/config.toml), strict CET and the full required host profile. No silent linker substitution, omitted protection or ambient newest-SDK selection; the upstream helper does select installed SDK/MSVC dynamically [E10]. |
| Isolation and debugging — platform/security | Both image debuggability and host creation debug permission must be absent in production; verify effective strict memory and all reached dependencies/callouts. SDK debug/release defaults differ [E2, E3, E5]. Test-signed/debug evidence cannot qualify the production claim. |
| Enclave signing — release/W11-W10 | Enclave DLL needs page-hash signing and code-signing, enclave and author EKUs. The documented production route is the dedicated VBS-enclave profile, now described under Artifact Signing [E2, E11]. Establish eligibility, author identity continuity, protected custody and real loader acceptance separately; ordinary Authenticode, MSIX signatures or an assumed provider account do not prove them. |
| Complete product — W11-W05/W11-W10, P6/P7 | Finish metadata and VEIID before inner enclave signing, preserve/verify that signature through complete direct MSIX signing, then inspect exact delivered/installed bytes, activation and per-role mitigations. Keep [all-file signature](../windows-time-architecture.md#197-signatures-for-the-complete-shipped-inventory), provision/update/repair/removal and release gates. Production signing may remain deferred while independent source preparation proceeds. |

No build, certificate/key creation, dependency installation, test-signing/Secure
Boot change, service operation or native experiment is authorized by this plan.
A later disposable-lab card must separately admit its tools, synthetic secrets,
both platform rows, resource/time bounds and restoration; it cannot weaken the
developer host or customer policy to manufacture a passing result.

## 6. Discriminating cases and evidence layers

All cases below are **planned, not executed**. Each starts from a useful admitted
neighbor and reaches the challenged owner; failure before the intended guard
does not count. Freeze finite cases/sequence bounds and independent expected
bytes/state/effects before selecting tools. Reuse the [ST-T mapping](windows-protected-storage.md#7-focused-threat-scenarios-and-acceptance-mapping),
including THR-10/17/25/26 and the existing BND-1/2/5/6/8 relationships; there is no
new acceptance-ID family.

| Case family / existing owners | Adverse case, independent expected result and forbidden effect | Useful neighbor and cheapest adequate evidence |
| --- | --- | --- |
| ABI/resource admission — ST-T11/16; CHECK-15/25/27 | Oversized/truncated/wrong-version/overflowing or racing/aliased host buffers, deep generated records and hostile callback lengths refuse before unbounded allocation/unpacking; no secret output or extra work slot. | A finite valid request reaches the real handler. Source/contract inspection now; actual decoder tests, bounded properties/fuzz and native accessor/ABI cases later. |
| Configuration authority — ST-T04/05/11/17; CHECK-2/3/15/25/27 | Well-formed wrong-authority, substituted exact bytes, stale/replayed/revoked revision and cross-installation/purpose requests preserve current policy and cannot use its keys. Host approval Boolean does not authorize. | Independently authorized fresh content commits once with the specified outcome. Source authority comparison now; direct-handler and policy/operation integration cases after the authority decision. |
| Keys and outputs — ST-T13/14/18; ST-W07 | Wrong direction/provenance, import/replace/retire or copy-out failure cannot leak keys, raw cookies or plaintext to prohibited DTO/log/temp/export paths or claim never-exposed origin. | Independent protocol/crypto expectations and useful redacted diagnostics succeed; source-to-sink inventory now, synthetic-secret real-output and selected collector checks later. Allowed wire fields are explicitly excluded from the secrecy claim. |
| Association and freshness — ST-T08/09/11/18; CHECK-6/8/9/15/25/27 | Valid tag with wrong request/UID/source/trust/epoch, expired or unknown age and retired association cannot publish a reply, cookie, source vote or plain fallback. | A complete current request/reply is admitted once. Reuse independent NTS vectors and actual association/consumer controls; enclave crypto success alone does not qualify sample admission. |
| Sealing, replay and upgrade — ST-T05/06/07/13; ST-W02 | Wrong reader/debug/purpose/version, old authentic record, stale key and partial migration cannot become current through unseal success. Retain the full-environment rollback gap. | An admitted current record and permitted upgrade recover without secret export or mandatory-state reset. Independent state/byte model first, actual sealing/installed migration later. |
| Concurrency and completion — ST-T10/11/16; ST-W06; CHECK-27 | Saturation, reentrancy, timeout, policy change, close and late completion cannot release live charges, reuse identities or publish stale results. Lost committed output remains charged/reconcilable. | Normal completion, draining and subsequent admitted work remain useful. Independent sequential model/deterministic interleavings first; reached real native thread/teardown observations later. |
| Crash and recovery — ST-T06/07/10/17; ST-W02/03/06 | Interrupt each reached create/init/operation/persist/ack/retire boundary, then interrupt recovery again. No false success, fresh-install inference, duplicate effect or refunded mandatory authority. | Same-operation healthy reconciliation reaches its declared state; observe resources before harness cleanup. Portable fault model first; Windows/filesystem/sealing lifecycle later. |
| Platform and complete package — ST-T01/02/12/13/17; W11-W05/10; CHECK-13/16/18/20/21/23/26/27 | Failed queries, missing support, wrong/debug image, host/enclave mismatch, denied import/mitigation, no-login/start/upgrade/repair/removal faults prevent false readiness and silent weaker execution. | Qualified nondebug useful operation under final tokens and required profile on both families. Cross-build, test-signed lab, production-loader and final installed-package evidence stay separate. |

The next source stage uses review and case design only. Later implementation
uses direct entrypoint unit/component/contract tests, external misuse/legal-use
controls for claimed Rust boundaries, independent lifecycle models and generated
properties. Structured fuzzing belongs at actual ABI/decoder/conversion/error
seams after its existing admission; current fuzz holds are not bypassed. Use
meaningful guard faults and the existing **90% security-primitive line coverage**
and **95% per-critical-file mutation** gates at their due checkpoints. Generated,
unsafe and native behavior omitted by a metric needs named alternative evidence,
not a changed denominator or invented kill. Native memory/resource mechanisms
need supported instrumented controls and separate uninstrumented CHECK-21 costs.
Formal execution remains post-V1 under OD-10. No tool pass substitutes for
hostile-host isolation, authority, trusted ingress or final artifact evidence.

## 7. One next stage and phased integration

**Next permitted action — ST-W01 source-only enclave boundary/feasibility card.**
The storage/architecture coordinator joins Engine/S2.2, security, Windows/build
and QA owners in the existing task. Starting from this source map, finish one
card for **Protect one NTS request** with its admit/reply/retire dependencies:

1. Trace every selected key/exporter/cookie/policy input and temporary copy to
   its real producer and allowed consumer; state after-import versus
   never-host-exposed feasibility without relabelling the existing host route.
2. Specify one finite versioned call contract, exact encoding and limits,
   private/current guard owners, outputs, charged transitions and uncertain
   outcomes; compare the pinned generated runtime with a narrowly qualified ABI
   only for the concrete early-admission/import gap.
3. Produce the change-authority/freshness/rollback decision card and exact
   SDK/crypto/import/linker/signing/support evidence table, identifying every
   unresolved input and the dependent action it blocks. No arbitrary bound,
   identity, algorithm, trust root or unavailable-mode default is filled in.
4. Bind the eight case families above to exact independent bytes/sequences,
   enforcing owner, accepted neighbor and evidence layer; obtain one fresh
   stage review and record `ready`, `revise` or `blocked` for the **named next
   engineering slice**, not for all future stages.

**Observable exit:** a fresh reviewer can trace the selected operation from
secret/config origin through protected state to each allowed output, identify
each authority/currentness/resource guard and its paired falsifier, and find an
exact source-supported disposition or owned blocker for every admission field.
Precise blocked feasibility is a valid documentary result. Stop after that card
and review; do not start a tool installation, throwaway enclave or native probe
to remove an unknown. The present plan is sufficient to start this source-only
stage; it is not an executable ABI, source-qualified construction or product
security acceptance.

| Later join / existing owner | Entry and observable exit; current disposition |
| --- | --- |
| Portable contracts and one enclave implementation slice — ST-W02/04/05/06/07, Engine/S2.2 and build/security | Requires ready relevant contracts, resolved dependent decisions, admitted graph/toolchain and implementation scope. Enforce early ABI bounds, complete association ownership and uncertainty; paired actual-consumer checks pass. **Not ready/admitted here.** |
| Disposable native qualification — Windows/security | Requires applicable pause/resume authorization plus exact frozen runtime/build/image/signing/profile rows and bounded restoration card. Observe useful operation and hostile neighbors, actual isolation/completion/resources on both families. **Held; no native result.** |
| Live NTS and optional restore — S2.2/S2.3/S2.4; P3/S3.3 and W11-W05 | Requires qualified trusted ingress, real consumer/authority and lifetime rules; persistence additionally needs its exact sealing/durability/migration freeze. Source-only or host-import fixtures cannot close the stronger claim. Existing OPEN-8 and mandatory-state owners remain. |
| Truthful status/configuration — P5/D2, W11-W08 | At existing D2 re-entry, bind actual current evidence and uncertain outcomes to authenticated status. Separate enclave custody, storage mode, authentication and clock readiness; stale/host-reported status supplies no independent hostile-host attestation. |
| Complete direct MSIX and release — P6/P7, W11-W10 | Only if adopted for the target release: preserve inner signatures/import identity, qualify supported install/boot/update/repair/removal and resources under final controls; reconcile all reached CHECK/ST-T gates and owner approval. No general V1 blocker or release pass is created now. |

## 8. Dated primary sources

Inspected 24 November 2026. Microsoft Learn pages are mutable; recheck the
selected deployment row before dependent engineering. The pinned successor
below supplements the historical `ec3ea90...` source in architecture S124; it
does not invalidate that earlier receipt or establish package/source parity.

- **E1:** [Microsoft VBS enclave overview][E1], updated 4 March 2026: current runtime and basic developer prerequisites.
- **E2:** [Supplied development guide][E2], updated 20 November 2024: host/image/ABI, compiler/linker/import binding, signing and debugging requirements.
- **E3:** [Microsoft tooling README at `23716f9`][E3], blob `b2f041f1e9bc8b4aefe9a29ba7d043f8e2d78542`: Rust, SDK-specific support and strict-memory build distinctions.
- **E4:** [Pinned generated Rust call helpers][E4], blob `e5b44932e15f6dba1a4abe5db18c783a5cc04bd3`: allocation and unpacking precede the developer handler; callback-return admission is also relevant.
- **E5:** [Strict-memory API][E5] and [copy-in API](https://learn.microsoft.com/en-us/windows/win32/api/winenclaveapi/nf-winenclaveapi-enclavecopyintoenclave): access restriction, supported copies and runtime mutability.
- **E6:** [CallEnclave][E6], [TerminateEnclave](https://learn.microsoft.com/en-us/windows/win32/api/enclaveapi/nf-enclaveapi-terminateenclave) and [DeleteEnclave](https://learn.microsoft.com/en-us/windows/win32/api/enclaveapi/nf-enclaveapi-deleteenclave): wait and actual-termination distinctions.
- **E7:** [EnclaveUnsealData][E7]: output-memory choice, stale-key and reader-policy contracts.
- **E8:** [Sealing identity policies][E8]: code/image/family/author reader scopes.
- **E9:** [Pinned Rust enclave sample][E9] and [binding build source](https://github.com/microsoft/VbsEnclaveTooling/blob/23716f9639a64050a0133b65c3edc5f886d1c54a/rust/sdk/crates/libs/windows-enclave/build.rs): `no_std`/allocation, sample panic policy and import remapping.
- **E10:** [Pinned Rust build helper][E10]: enclave link inputs and ambient SDK/MSVC selection requiring explicit product admission.
- **E11:** [Artifact Signing profiles][E11], updated 8 January 2026: distinct VBS enclave profile and identity-validation/role prerequisites; no account eligibility was tested.

[E1]: https://learn.microsoft.com/en-us/windows/win32/trusted-execution/vbs-enclaves
[E2]: https://learn.microsoft.com/en-us/windows/win32/trusted-execution/vbs-enclaves-dev-guide
[E3]: https://github.com/microsoft/VbsEnclaveTooling/blob/23716f9639a64050a0133b65c3edc5f886d1c54a/README.md
[E4]: https://github.com/microsoft/VbsEnclaveTooling/blob/23716f9639a64050a0133b65c3edc5f886d1c54a/rust/edlcodegen/crates/libs/edlcodegen-enclave/src/enclave_helpers.rs
[E5]: https://learn.microsoft.com/en-us/windows/win32/api/winenclaveapi/nf-winenclaveapi-enclaverestrictcontainingprocessaccess
[E6]: https://learn.microsoft.com/en-us/windows/win32/api/enclaveapi/nf-enclaveapi-callenclave
[E7]: https://learn.microsoft.com/en-us/windows/win32/api/winenclaveapi/nf-winenclaveapi-enclaveunsealdata
[E8]: https://learn.microsoft.com/en-us/windows/win32/api/ntenclv/ne-ntenclv-enclave_sealing_identity_policy
[E9]: https://github.com/microsoft/VbsEnclaveTooling/blob/23716f9639a64050a0133b65c3edc5f886d1c54a/rust/edlcodegen/crates/samples/helloworld/enclave/src/lib.rs
[E10]: https://github.com/microsoft/VbsEnclaveTooling/blob/23716f9639a64050a0133b65c3edc5f886d1c54a/rust/edlcodegen/crates/libs/edlcodegen-tools/src/lib.rs
[E11]: https://learn.microsoft.com/en-us/azure/artifact-signing/concept-resources-roles
