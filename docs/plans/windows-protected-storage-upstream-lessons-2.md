# Windows protected storage: second upstream review and agent adaptations

**Document:** NV-STORAGE-UPSTREAM-002, revision 1.0<br>
**Date:** 3 November 2026<br>
**Owner:** David Osipov<br>
**Status:** Completed bounded source review; retained implementation guidance, conditional candidates and unexecuted qualification cases. Documentation and agent-task integration is owner-authorized; the construction freeze and native experiment pause remain.<br>
**Research baseline:** [`6b28711f5510e3550039d5fcf8bbcf3ccba791a5`][nv-head], read through an API-derived partial snapshot, not a local Git checkout.

This supplement extends [NV-STORAGE-UPSTREAM-001][nv-lessons]. The [agent task][nv-task] assigns its bounded follow-up; the [development workstream][nv-development] retains phase ordering. Section numbers here are reading aids, not additional acceptance IDs.

## 1. Recommendation and scope

The most useful results are a narrow additional Windows mitigation, precise persistence-failure semantics, and implementation patterns that make existing contracts difficult to violate accidentally. Prioritize the storage operation outcome model, directory authority, per-call CNG admission and persistence simulator/monitor. Evaluate RedirectionGuard in the existing mitigation matrix. Keep remote attestation, hardware rollback barriers, chunked snapshots and a database backend as separate conditional choices.

The [current architecture][nv-architecture] owns technical contracts; the [protected-storage plan][nv-plan] owns construction, threat scenarios and execution gates. The two source supplements preserve evidence and adaptation limits; the [agent task][nv-task] owns work items and candidate-specific execution receipts. This report creates no new acceptance IDs or release blockers. It does not select a provider, key-wrapping construction, library, database, Windows support floor or mitigation exception. Most findings refine requirements already present; calling them missing security methodologies would be inaccurate.

Research combined three independently assigned source-search lanes, root source verification, a fresh critical review, and a focused reserve investigation of hardware rollback barriers. GitHub code and test bodies were inspected at immutable revisions. Search hits, README claims and test names were screened against implementation. No upstream code was built or run. No Windows keys, services, policies, tools or dependencies were installed or changed. The owner's experiment pause remains in effect. This is not a comprehensive security audit of NevroTick or the upstream projects, and no source concern below is presented as a reproduced vulnerability.

### Priority map

| Priority | Mechanism | Increment over the current design | Intended owner |
| --- | --- | --- | --- |
| High | Explicit indeterminate commit and poisoned owner | Gives existing recovery states precise API semantics; prevents unsafe generic error retries | Protected-store owner |
| High | Live/durable model plus low-level protocol monitor | Concrete realization of existing model/sequence and crash requirements | Existing storage test harness |
| High | Directory capability and validation before destructive effects | Concrete Windows path-resolution and handle-lifetime reference | Narrow Windows filesystem adapter |
| High | Per-call CNG silence, bounded property decoding and independent crypto fixtures | Exposes assumptions hidden by high-level wrapper success | Narrow CNG adapter/provider qualification |
| High | Mitigation startup transitions and distinct ACG probes | Concrete release enforcement and native evidence design | Existing process-policy owner |
| Candidate | RedirectionGuard | Additional mechanism absent from the research-baseline canonical docs; candidate only | Per-role mitigation matrix |
| Medium | Pipe-name ownership and completion-aware close | Concrete refinement of existing IPC/native-custody contract | IPC adapter |
| Medium | Status-preserving independent observer | Candidate qualification tool, with known convenience-API pitfalls | Native qualification harness |
| Conditional | Attestation/origin, hardware rollback epochs | Investigations into acknowledged evidence/rollback gaps | Later explicit owner decision |
| Conditional | Guarded secret arenas, crash-output privacy, authenticated stream completion | Targeted implementation references, not new library mandates | Secret-memory/report/restore adapters |

## 2. RedirectionGuard: one additional Windows defense worth investigating

Windscribe's [Windows mitigation header][windscribe] really sets `ProcessRedirectionTrustPolicy` with `EnforceRedirectionTrust`. Microsoft's [MSRC explanation][ms-redirection] describes an opt-in barrier against filesystem junctions marked as created or modified by a non-administrator. Legacy junctions lacking that metadata and administrator-created junctions may still pass. Arbitrary path selection, traversal and other out-of-scope redirection remain application concerns. The [policy structure][ms-redirection-struct] distinguishes audit from enforcement.

For NevroTick, investigate this as an additional row for the file-owning service and elevated lifecycle helper. Preserve the retained-directory and no-follow controls below. Required/optional disposition, initialization phase, query semantics and supported OS/build/account must be qualified before adoption. The struct reference's minimum-OS table and the MSRC rollout discussion do not establish a reliable product support floor by themselves.

Do not copy Windscribe's policy wholesale. Its helper logs individual setter failures and later emits a general enabled message. Its comments describe registry/symlink coverage more broadly than the Microsoft contract, and its target-specific mitigation opt-outs are not NevroTick policy. A TLS callback is not proof that all prior imports or loader-time behavior were protected.

**Distinguishing evidence to plan:** successful ordinary storage/provider operations; audit versus enforce behavior for an in-scope unprivileged junction; a legacy or administrator-created control; an out-of-scope path still rejected by NevroTick; unsupported/failed queries remain Unknown. Use existing ST-T12/17 and filesystem/service ownership checks. This is a new candidate defense, not a completed native result.

## 3. Borrow directory authority and delayed destruction from cap-std

cap-std's [manual path walker][cap-manual] and [Windows relative-open adapter][cap-create] retain directory handles and use `NtCreateFile` with `RootDirectory`. The [opened-object handling][cap-open] validates type/no-follow conditions before manually truncating. This ordering matters: opening with a destructive option can change the object before a later check gets a chance to reject it.

The useful abstraction is an admitted store-directory capability plus a finite namespace of relative object names. The adapter receives that authority, rather than arbitrary absolute paths. Initial root admission still needs owner, ACL, identity and supported-filesystem checks. Subsequent operations validate the actual opened object before mutation. For a small controlled NevroTick namespace, rejecting link/reparse objects can be simpler than reproducing cap-std's permitted in-root symlink semantics.

Two limits materially affect reuse. The [directory sharing policy][cap-dirs] deliberately omits `FILE_SHARE_DELETE`, because some operations reconstruct paths. The [Windows rename implementation][cap-rename] does exactly that before calling `std::fs::rename`. Thus the whole library is not universally handle-relative. Retained handles can also block installer rename/repair until the service releases them. Existing shutdown and native custody rules must resolve that lifecycle, rather than casually relaxing the sharing policy.

The [symlink tests][cap-symlinks] have early-return paths when unsupported; [rename_basics][cap-rename-tests] is ignored on Windows. Test presence supplies cases to borrow, not Windows proof.

**Planned cases:** parent or leaf substitution, unexpected reparse/hard-link/type/owner, absolute/UNC/device/ADS/traversal names, prohibited truncation, and installer rename while the owner is active. A legitimate regular-file save must succeed. Keep this under ST-T03/05/10/17 and the existing Windows adapter contract; no general filesystem crate is selected.

## 4. An error is not always evidence that nothing happened

redb's [transaction implementation][redb-transactions] poisons partially failed catalogue mutations. Its commit contract separately warns that some commit errors can occur after changes are already durable; they must not be treated as rollback. A rejected poisoned transaction can be rolled back without automatically poisoning the whole database; these outcomes must remain distinct. Its [failed-integrity-check test][redb-repair] prevents further writes after failed reconstruction and confirms that destruction does not silently clear recovery-required state. The repair-ID test in transactions.rs ensures recovery's committed transaction ID is not reissued to the next ordinary transaction.

For NevroTick, preserve two distinct dimensions:

| Dimension | Useful distinctions |
| --- | --- |
| Outcome of one operation | Definitely not applied; committed with a receipt; outcome requires reconciliation |
| Usability of the owner/transaction | Ready; partially modified or poisoned; recovery required; retired/closed |

A caller that catches an error must not automatically obtain permission to create another root set, erase the catalogue, repeat a migration, reuse a generation, or refund a correction reservation. Reconciliation concerns the same operation identity and the existing exact-envelope digest/epoch. A confirmed complete recovery can construct a new usable owner; merely logging the failure cannot do so.

This is especially valuable for agent-written code: represent these outcomes in narrow private APIs so the unsafe next step is unavailable or explicitly rejected. Do not copy redb's recovery choice as permission to accept an older authentic NevroTick snapshot. Mandatory continuity, privacy generations and NTS eligibility remain independent of database readability. Native work may still be pending while reconciliation is required; its resource charges and sensitive buffers remain owned until actual completion.

The [Windows file backend][redb-io] provides a small adjacent lesson: zero-byte progress becomes `WriteZero` or `UnexpectedEof`. A loop that only handles `Err` can otherwise spin on `Ok(0)`.

**Planned cases:** partial catalogue mutation followed by attempted commit; an error after possible durable publication; recovery interrupted twice; repair followed by a new monotonically advancing ID; concurrent retry under the same operation identity; healthy authorized recovery. Keep ST-T06/07/10/16/17 distinctions, including mandatory versus optional data.

## 5. Model durability separately, then monitor the operation protocol

redb's [crash regression backend][redb-crash] keeps live bytes, durable bytes and the latest header separately. It reconstructs a file-growth failure where a new header survives while its referenced extension does not. SQLite's [crash VFS][sqlite-crash] models buffered writes, selective survival and sector damage under explicit device assumptions. Its [crash tests][sqlite-crash-tests] include healthy controls where the requested failure point is not reached.

A NevroTick model should track issued writes, process-visible state, successful durability barriers and acknowledged product operations independently. Exercise cuts across preparing ciphertext, file growth, flush, replacement, catalogue update, publication and cleanup. Add failure during recovery, not only during ordinary saving. The oracle must preserve acknowledged mandatory reservations and privacy barriers; unacknowledged work may have a small explicitly allowed set of outcomes.

Do not mistake all I/O fault fuzzing for a power-loss model. redb's [fuzz backend][redb-fuzz] explicitly makes sync a no-op and says it does not simulate actual crashes. Its targeted crash regression also continues returning success after an artificial durability freeze to construct a selected crash image. A NevroTick trace should stop at the crash cut or disregard later returns, rather than legitimizing loss of acknowledged durable state.

SQLite adds a complementary mechanism: [test_journal.c][sqlite-monitor] wraps the VFS and checks ordering while execution proceeds. It records original page checksums and monitors which writes are allowed after journal activity. This can expose an unsafe ordering even when one sampled restart happens to look correct.

Adapt the monitor to NevroTick's own protocol, with independently derived events for prepared-content identity, exact committed digest, current generation, native completion, retirement and acknowledgement. Separate `barrier requested`, `barrier succeeded` and `barrier failed`: SQLite's test wrapper updates its bookkeeping before calling the underlying sync, so its exact counter is not a completed-durability receipt. It also explicitly excludes synchronous-off operation. Its pager protocol does not establish an atomic transaction across CNG objects and application files.

**Planned controls:** a compiled defect that acknowledges before the required successful barrier; a wrong digest/epoch; premature retirement; a late native result after privacy revocation; a healthy commit and healthy cleanup. The monitor should identify the first invalid event. Reuse the project's existing action-sequence/model and seeded-defect workflow. Model success remains separate from native Windows filesystem/provider evidence under ST-T05/10/15/16/18.

## 6. CNG admission: per-call silence, real lengths and explicit parameters

### 6.1 Silence is an operation contract

GoDaddy's [key adapter][gd-key] offers silent open/create variants. In its [encryption path][gd-encrypt], one mode uses silent opening but later import/agreement calls use default flags. That shows the flags supplied by source; no unwanted prompt was reproduced. Microsoft's [SecretAgreement contract][ms-agreement], like several other NCrypt operations, separately defines `NCRYPT_SILENT_FLAG` and a refusal result when UI would be needed.

Refine NevroTick's existing unattended-startup requirement into a per-native-call table: API, documented flags, actual key policy, service account/session, successful result and refusal result. Do not assume silence is inherited from an opened handle or attach an undocumented flag indiscriminately to every API. Never retry a refusal by stripping the requirement. An interactive Hello feature, if ever chosen, remains a separate user-presence design.

### 6.2 Property queries need bounded decoding

GoDaddy's [UI-policy adapter][gd-policy] illustrates a real variable-size native property: a fixed structure can be followed by inline strings. It uses two calls to obtain and fill the required buffer and has ignored-by-default native tests for matching and mismatched policy.

It is a useful reference with important adaptation limits. The reviewed implementation accepts arbitrary query errors when expected policy is None, allocates the reported size without a product cap, does not validate the second returned length before prefix access and casts a byte-vector pointer to a native structure. These are source-level review observations, not reproduced faults. A NevroTick decoder should cap both request and returned sizes, validate version/flags, read the required fields safely and preserve absence/unsupported/error distinctions. The expected policy comes from the admitted owner/catalogue, not an untrusted sidecar. The [Windows structure contract][ms-ui] concerns strong-key UI; it does not itself establish biometric verification.

### 6.3 Independent crypto expectations catch shared mistakes

GoDaddy explicitly supplies HASH/SHA256 parameters. Microsoft's [BCryptDeriveKey contract][ms-kdf] confirms that HASH defaults to SHA1 when its hash parameter is omitted and that the returned byte count can be shorter than the supplied buffer. GoDaddy's source records this as a historical failure pattern. Its software test does exercise real encryption, but despite a full-roundtrip comment it only asserts envelope shape. The TPM roundtrip returns early when unavailable.

For NevroTick's eventually selected construction, freeze algorithm, hash/KDF, padding, encoding, context and exact result lengths. Pair roundtrips with independent known-answer or interoperability fixtures. Encrypt and decrypt can agree while sharing the same wrong default. A mutation removing the parameter or changing the context should fail the actual declared contract. This lesson does not select ECIES or SHA256 as the project's wrapping construction.

**Combined planned controls:** no-login native operation; UI-requiring root refusal without retry; long-but-bounded properties; short/changed/oversized results; expected-None plus access denial; wrong actual key policy; exact independent expected bytes; unavailable hardware distinguished from pass. These refine ST-T01/05/12/13/16/18.

## 7. Mitigations need ordered startup and behavioral evidence

Chromium's [mitigation implementation][cr-mitigations] distinguishes starting policy, repeated tightening and once-only child lockdown. Its [death tests][cr-order] exercise invalid ordering. The ordering checks in those functions are `DCHECK`s; the actual policy setter also has a `PCHECK`. Do not characterize all mitigation enforcement as debug-only. NevroTick should use private release-checked transitions, with actual effective-policy evidence kept separate from bookkeeping. A provider error cannot authorize returning to a weaker initialization phase.

Chromium's [dynamic-code tests][cr-acg] exercise four distinct paths: executable/writable allocation, modifying existing executable-page protection, anonymous executable/writable mappings and file-backed mappings. They use healthy controls and expected denial status, separately from policy readback. Some tests are excluded for ASan/debug builds, and other [image-policy tests][cr-image] have component-build and loader-specific exceptions. A general green test count cannot qualify the final product profile.

The future native matrix should record requested policy, successful query, prohibited-operation denial and legitimate CNG/TLS/file/IPC success for the exact artifact, account and process role. Include cold startup and first lazy provider use. NevroTick's existing no-opt-out/no-remote-downgrade ACG, CIG and other required controls stay intact.

Google Project Zero's [NtCoreLib query implementation][nt-process] is a useful independent-observer reference, but its convenience overload maps three statuses—invalid parameter, unsupported and process terminating—to zero. The status-bearing overload preserves the uncertainty. If the tool is later admitted, retain native status, individual fields, process identity, build and observation time. Some accessible-object cmdlets actively open or create objects; they are not a license for broad probing of a live machine.

Chromium's [restricted-token tests][cr-token] also supply concrete same-account challenges: `MAXIMUM_ALLOWED`, owner `WRITE_DAC`, handle duplication and a healthy self-access control. Borrow the test shapes for the existing second-LocalService gate. Its browser-child/random-SID topology is not an already-qualified SCM service model.

## 8. Native close must mean what its callers think it means

go-winio's [pipe listener][winio-pipe] retains a separate disconnected first pipe instance for the listener lifetime, protecting continuous name ownership while other instances handle clients. During shutdown, close remains authoritative even if a pending connect races with success or `ERROR_NO_DATA`: the result is drained without starting new retries. Listener callers share a completion channel. Its [tests][winio-tests] exercise close/connect races, concurrent close and accept-after-close.

Its [native file wrapper][winio-file] cancels pending I/O but still waits for completion before releasing the first owner's resources. There is an important scope difference: subsequent concurrent generic file Close calls can return before that winning close finishes, whereas every Listener.Close caller waits on the shared `doneCh` until the listener routine completes. Do not generalize either API's semantics to every close acknowledgement.

For NevroTick, specify separate request-to-close, result-publication revocation and actual-drained completion. Consumers that need a completed barrier must await the shared completion. Native permits, key handles and secret buffers stay charged through any recovery until the actual operation completes. Default SQOS/access-mask choices from a generic pipe library do not satisfy NevroTick's caller-identity contract automatically. This is a concrete reference for already-written ownership rules.

## 9. Origin, attestation and hardware rollback: conditional investigations

### 9.1 Preserve the exact evidence claim

Microsoft's [TPM attestation sample][ms-att-sample] distinguishes the certified public key and TPM attributes including `fixedTPM`, `fixedParent` and `sensitiveDataOrigin`. Older [TSS property extraction][tss-properties] calls the first two NON_MIGRATABLE; that is a different claim from generation inside TPM. The sample uses an external attestation service and a placeholder nonce, and does not implement the relying party's full decision.

A future NevroTick evidence object should preserve actual public identity, intended use, present custody, generation/import origin, trusted certification authority, fresh binding and native readiness separately. Flags read from unauthenticated data do not prove origin. Neither present nonexportability nor a successful signature proves the key was never exposed outside hardware. The existing origin gap remains open.

Hyperion's [challenge endpoint][hyperion-endpoint] and [proof verifier][hyperion-verifier] offer a useful transcript pattern: a single-use challenge plus a domain-separated proof over session, nonce and the exact claim hash. The reviewed path has partial-trust limitations, including a claim-without-nonce classification and an unanchored client-supplied IDKS path. Borrow transcript binding as a concept, not the reassuring final boolean. A disposable VBS probe does not establish durable root reopen, service isolation or mitigation compatibility.

Remote attestation may be unnecessary for a local optional cache. Any future selection must avoid introducing a hidden online/correct-time bootstrap dependency. PCR binding also remains a reserve: the [TSS PCR example][tss-pcr] supplies a useful control where public encryption succeeds but private use fails under wrong measurements. Its legacy algorithms, overwrite creation and recovery conventions are not a product template.

### 9.2 Separate snapshot sequence from hardware revocation epoch

Canonical secboot's [current update API][secboot-update] permits either reusing the actual TPM NV counter value or preparing its value plus one for a later revocation. Its legacy path instead advances the policy sequence on each edit. The [v3 policy][secboot-policy] checks `hardware counter <= signed policy sequence`; it binds the counter Name and role into the authorized policy reference. This revokes earlier policies but does not identify the latest application snapshot. Even a signed prepared future policy can satisfy the inequality.

A possible NevroTick adaptation is a separate hardware revocation epoch, advanced only at an explicitly authorized retirement or policy transition. Ordinary optional-cache saves could share an epoch. This reduces hardware updates while leaving an explicit same-epoch replay gap when both the catalogue and its envelope can be replaced. The existing exact committed-envelope digest remains necessary. A prepared object must never become eligible solely through its sequence number. A counter establishes ordering, not UTC or NTS authentication freshness.

The update and revocation are separate actions. An uncertain increment requires reconciliation against the actual value before another irreversible attempt. A future design must bound the target gap and work, refuse overflow, and define missing/wrong/unavailable-anchor recovery. No final commit ordering or adoption decision is made here.

### 9.3 Recovery capacity bounds mutation concurrency

The fixed-revision [Fuchsia PinWeaver consumer][fuchsia-pending] permits one chip mutation ahead of durable disk state because its stated replay capacity is one operation. Credential mutations acquire the chip lock and drain earlier pending disk commits first. Its indefinitely retried disk commit is not an appropriate NevroTick work-budget policy.

The transferable requirement is small: reserve attributable recovery capacity before changing the durable anchor, and do not overwrite the only evidence that can reconcile it. If an advanced anchor cannot be matched to an admitted transition, optional restore must stop. This reinforces the existing one-manager and finite-slot plan; it does not justify adopting a Merkle tree, PinWeaver or a new privileged service.

### 9.4 Windows integration and failure limits remain open

The [TPM specification, v185 Part 1 §34.7.1][tcg-architecture], permits an interrupted NV write to invalidate the affected index. An expected missing anchor therefore needs explicit recovery semantics; it cannot mean initial counter zero or a fresh installation. Microsoft's [owner-authorization guidance][ms-tpm-owner] says ordinary Windows provisioning does not retain the owner password. Administrator membership and a CNG key handle do not establish the provisioning authority needed by secboot's construction.

This research establishes no supported bridge between direct `PolicyNV` operations and NevroTick's selected VBS/CNG root design, and no protection against a full VM/vTPM rollback. Hardware epochs remain a separately reviewed reserve. secboot's inspected files are GPLv3: only the protocol ideas are proposed for study, with the existing source/dependency licence gate preserved.

## 10. Smaller and conditional lessons

### Guarded sensitive buffers and actual fault witnesses

libsodium's [memory implementation][sodium-memory] uses Windows page allocation/protection and wipes before unlocking. Its allocator intentionally ignores lock failure, so allocated is not the same as successfully locked. The [guard tests][sodium-tests] contain platform/sanitizer branches that print the expected result without executing the deliberate fault; that is portability logic, not native guard evidence.

A small checked, non-executable sensitive arena and exact page-boundary FFI fixtures are useful candidates. Preserve quotas, explicit error policy, native lifetime and healthy controls. Guard pages do not encrypt memory or exclude arbitrary code in the same process. The [VirtualLock guarantee][ms-lock] is limited to the successfully locked pages while locked; it does not cover every stack copy, dump or hibernation path. Do not add privilege or change machine paging policy automatically. `secrecy` and `zeroize` are already in NevroTick's architecture and are not new dependency recommendations.

### Crash/support output is its own privacy boundary

Crashpad's [sanitized snapshot tests][crashpad-tests] contrast allowed annotations/memory with rejected neighbors. This is useful for synthetic key/cookie/domain sentinels in actual output artifacts. Its documented sanitization CLI path is [Linux-only][crashpad-handler], and the reviewed wrapper delegates some streams unchanged. It is not a ready Windows redaction switch. Keep the project's explicit dump policy; do not enable raw dumps merely to use this tool.

The Windows [WER excluded-memory API][ms-wer] is a bounded, fallible collector-specific exclusion. It is not protection for all copies or all dump collectors. Any future use needs allocation lifecycle and error handling, and cannot upgrade the UI to a whole-memory protection claim.

### Stream completion and database integration

If snapshots later become chunked, age's [stream implementation/tests][age-stream] authenticate final-chunk state and reject truncation/trailing data. Its streaming reader intentionally releases authenticated chunks before the entire stream is validated; a later read can still fail. A NevroTick restore must not publish partial state as a complete admitted snapshot. Current bounded whole-record admission remains simpler.

If a database is justified later, SQLCipher's [compile-time integration checks][cipher] and executed-hook-aware memory status are useful references. But its Windows locking failure can be logged without invalidating that status, and temp-store mode 2 still permits disk-temp selection. Database encryption neither integrates the VBS root pair nor solves malicious rollback. This report does not select SQLite, SQLCipher, redb or a new crypto provider.

## 11. What to exclude from immediate adoption

- [WireGuard's LocalSystem/driver-privilege topology][wireguard-security] and [Mullvad's permissive local-management threat model][mullvad-security] differ from NevroTick's selected account/authority boundaries. They are comparative sources, not drop-in service designs. Mullvad still names anti-LPE and remote/browser-origin restrictions.
- A general Rust RAII impersonation guard is not sufficient proof of thread affinity or fatal release-mode reversion. The reviewed [interprocess guard][interprocess-guard] calls a [debug-only error check][interprocess-debug] after native reversion. Preserve the existing synchronous/fatal-revert contract.
- GoDaddy's [metadata tag][gd-tag] and [migration witness][gd-marker] corroborate NevroTick's existing installation witness; deleting a tag must not reopen first-use authorization. Its Credential Manager backend and unavailable-store policy are not the chosen fallback.
- [KeePassXC's alternative save modes][keepass-save] and [memguard's global failure behavior][memguard-buffer] are not NevroTick error policy. Scoped static concerns do not establish a vulnerability verdict on those projects.
- None of the reviewed sources replaces per-file provenance, exact dependency/license review, actual provider/key admission or native account/mitigation qualification. The project's chrony-derived licence gate remains authoritative.

## 12. Handoff into the existing workstream

Continue through [tasks/windows-protected-storage.md][nv-task], starting with its bounded source/contract preparation and recording unresolved owner choices in the existing decision workflow. Its ST-W identifiers are work items mapped to the existing ST-T/CHECK/DNS-C owners, not acceptance replacements. When implementation is authorized, first refine the typed storage/adapter contracts and the independent model. Add the concrete controls above to existing ST-T scenarios rather than inventing another acceptance catalogue. Then qualify the selected native Windows artifact, service account, filesystem, provider pair and unchanged mitigation graph.

The evidence table should distinguish source reference, executable portable test, actual Windows execution, unavailable/skipped environment and unresolved design decision. A simulated crash, a compiled native binding, a source-defined test and a successful real private-key operation are different observations. No section of this research advances the current native gate or resolves the storage construction freeze.

## Source index

Links below identify the exact inspected source revision; API pages state current published contracts rather than a tested implementation matrix.

[nv-head]: https://github.com/DavidOsipov/NevroTick/tree/6b28711f5510e3550039d5fcf8bbcf3ccba791a5
[nv-architecture]: ../windows-time-architecture.md#1212-protected-optional-storage-and-explicit-fallback
[nv-plan]: windows-protected-storage.md
[nv-lessons]: windows-protected-storage-upstream-lessons.md
[windscribe]: https://github.com/Windscribe/Desktop-App/blob/36c69999439d46745d68eeeebb5ee6520c1d24fc/src/client/client-common/utils/wsprocessmitigations.h
[ms-redirection]: https://www.microsoft.com/en-us/msrc/blog/2025/06/redirectionguard-mitigating-unsafe-junction-traversal-in-windows
[ms-redirection-struct]: https://learn.microsoft.com/en-us/windows/win32/api/winnt/ns-winnt-process-mitigation-redirection-trust-policy
[cap-manual]: https://github.com/sunfishcode/cap-std/blob/b21121159cbbdca93c0d213dfc8e6f66577d6657/cap-primitives/src/fs/manually/open.rs
[cap-create]: https://github.com/sunfishcode/cap-std/blob/b21121159cbbdca93c0d213dfc8e6f66577d6657/cap-primitives/src/windows/fs/create_file_at_w.rs
[cap-open]: https://github.com/sunfishcode/cap-std/blob/b21121159cbbdca93c0d213dfc8e6f66577d6657/cap-primitives/src/windows/fs/open_unchecked.rs
[cap-dirs]: https://github.com/sunfishcode/cap-std/blob/b21121159cbbdca93c0d213dfc8e6f66577d6657/cap-primitives/src/windows/fs/dir_utils.rs
[cap-rename]: https://github.com/sunfishcode/cap-std/blob/b21121159cbbdca93c0d213dfc8e6f66577d6657/cap-primitives/src/windows/fs/rename_unchecked.rs
[cap-symlinks]: https://github.com/sunfishcode/cap-std/blob/b21121159cbbdca93c0d213dfc8e6f66577d6657/tests/symlinks.rs
[cap-rename-tests]: https://github.com/sunfishcode/cap-std/blob/b21121159cbbdca93c0d213dfc8e6f66577d6657/tests/rename.rs
[redb-transactions]: https://github.com/cberner/redb/blob/edbbbe3aef08f8ab45786ba7442247d8fd7dfae7/src/transactions.rs
[redb-repair]: https://github.com/cberner/redb/blob/edbbbe3aef08f8ab45786ba7442247d8fd7dfae7/tests/check_integrity_failure.rs
[redb-io]: https://github.com/cberner/redb/blob/edbbbe3aef08f8ab45786ba7442247d8fd7dfae7/src/tree_store/page_store/file_backend/optimized.rs
[redb-crash]: https://github.com/cberner/redb/blob/edbbbe3aef08f8ab45786ba7442247d8fd7dfae7/tests/crash_consistency.rs
[redb-fuzz]: https://github.com/cberner/redb/blob/edbbbe3aef08f8ab45786ba7442247d8fd7dfae7/fuzz/fuzz_targets/fuzz_redb.rs
[sqlite-crash]: https://github.com/sqlite/sqlite/blob/7b667bbf9fadc419b56b953dbe066afe534e2646/src/test6.c
[sqlite-crash-tests]: https://github.com/sqlite/sqlite/blob/7b667bbf9fadc419b56b953dbe066afe534e2646/test/crash.test
[sqlite-monitor]: https://github.com/sqlite/sqlite/blob/7b667bbf9fadc419b56b953dbe066afe534e2646/src/test_journal.c
[gd-key]: https://github.com/godaddy/hardware-enclave/blob/3b4ac1bcb637fb60ac18d4cd9877dba989c46dba/crates/hardware-enclave/src/internal/windows/key.rs
[gd-encrypt]: https://github.com/godaddy/hardware-enclave/blob/3b4ac1bcb637fb60ac18d4cd9877dba989c46dba/crates/hardware-enclave/src/internal/windows/encrypt.rs
[gd-policy]: https://github.com/godaddy/hardware-enclave/blob/3b4ac1bcb637fb60ac18d4cd9877dba989c46dba/crates/hardware-enclave/src/internal/windows/ui_policy.rs
[ms-agreement]: https://learn.microsoft.com/en-us/windows/win32/api/ncrypt/nf-ncrypt-ncryptsecretagreement
[ms-ui]: https://learn.microsoft.com/en-us/windows/win32/api/ncrypt/ns-ncrypt-ncrypt_ui_policy
[ms-kdf]: https://learn.microsoft.com/en-us/windows/win32/api/bcrypt/nf-bcrypt-bcryptderivekey
[cr-mitigations]: https://github.com/chromium/chromium/blob/b3bde7a25634077b37fb3904885ee90635faf471/sandbox/win/src/process_mitigations.cc
[cr-order]: https://github.com/chromium/chromium/blob/b3bde7a25634077b37fb3904885ee90635faf471/sandbox/win/src/process_mitigations_deathtest.cc
[cr-acg]: https://github.com/chromium/chromium/blob/b3bde7a25634077b37fb3904885ee90635faf471/sandbox/win/src/process_mitigations_dyncode_unittest.cc
[cr-image]: https://github.com/chromium/chromium/blob/b3bde7a25634077b37fb3904885ee90635faf471/sandbox/win/src/process_mitigations_unittest.cc
[cr-token]: https://github.com/chromium/chromium/blob/b3bde7a25634077b37fb3904885ee90635faf471/sandbox/win/src/restricted_token_test.cc
[nt-process]: https://github.com/googleprojectzero/sandbox-attacksurface-analysis-tools/blob/852b1586b8c4527f8be3f486801e4830220c76d0/NtCoreLib/NtProcess.cs
[winio-pipe]: https://github.com/microsoft/go-winio/blob/62629cf958d03fbc744fd4b6ca1ae6a19f5a79dc/pipe.go
[winio-file]: https://github.com/microsoft/go-winio/blob/62629cf958d03fbc744fd4b6ca1ae6a19f5a79dc/file.go
[winio-tests]: https://github.com/microsoft/go-winio/blob/62629cf958d03fbc744fd4b6ca1ae6a19f5a79dc/pipe_test.go
[ms-att-sample]: https://github.com/microsoft/Attestation-Client-Samples/blob/84a502dcbedb674b505262f3eee022fd88ebc3ef/attestation/sample_tpm_key_att.cpp
[tss-properties]: https://github.com/microsoft/TSS.MSR/blob/52cb9f432318e8e95cdfeaf98b824ece89370744/PCPTool.v11/dll/TPM20.cpp
[tss-pcr]: https://github.com/microsoft/TSS.MSR/blob/52cb9f432318e8e95cdfeaf98b824ece89370744/PCPTool.v11/misc/PCRBoundKeyTest.cmd
[hyperion-endpoint]: https://github.com/fsquirt/Hyperion/blob/615eddd9481b19370a3370fd7fbfa95048c312d2/Server/Api/VbsEndpoints.cs
[hyperion-verifier]: https://github.com/fsquirt/Hyperion/blob/615eddd9481b19370a3370fd7fbfa95048c312d2/Server/Services/VbsRuntimeVerifier.cs
[sodium-memory]: https://github.com/jedisct1/libsodium/blob/c5717543ad05c70dcc97df7ea08b497d5afc2c86/src/libsodium/sodium/utils.c
[sodium-tests]: https://github.com/jedisct1/libsodium/blob/c5717543ad05c70dcc97df7ea08b497d5afc2c86/test/default/sodium_utils2.c
[ms-lock]: https://learn.microsoft.com/en-us/windows/win32/api/memoryapi/nf-memoryapi-virtuallock
[crashpad-tests]: https://github.com/chromium/crashpad/blob/86aed0d4e037f9af6347b8b21daa37ea779234b2/snapshot/sanitized/process_snapshot_sanitized_test.cc
[crashpad-handler]: https://github.com/chromium/crashpad/blob/86aed0d4e037f9af6347b8b21daa37ea779234b2/handler/crashpad_handler.md
[ms-wer]: https://learn.microsoft.com/en-us/windows/win32/api/werapi/nf-werapi-werregisterexcludedmemoryblock
[age-stream]: https://github.com/FiloSottile/age/tree/b74dce4cdbe35b5e5f66c06d9612b72f89028758/internal/stream
[cipher]: https://github.com/sqlcipher/sqlcipher/blob/355d4db92d3404e71faa8088d071a71d767807f4/src/sqlcipher.c
[secboot-update]: https://github.com/canonical/secboot/blob/88bdf1129f58423094889c4504979e5f391f1219/tpm2/update.go
[secboot-policy]: https://github.com/canonical/secboot/blob/88bdf1129f58423094889c4504979e5f391f1219/tpm2/policy_v3.go
[fuchsia-pending]: https://fuchsia.googlesource.com/fuchsia/+/e984fb5d50f74c952f9d580108514bf8f2b53c8e/src/security/bin/credential_manager/src/credential_manager.rs
[tcg-architecture]: https://medias.cdn.trustedcomputinggroup.org/production/Trusted_Platform_Module_2_0_Library_Part_1_Architecture_Version_185_pub_2a3392dc49.pdf
[ms-tpm-owner]: https://learn.microsoft.com/en-us/windows/security/hardware-security/tpm/change-the-tpm-owner-password
[wireguard-security]: https://github.com/WireGuard/wireguard-windows/blob/6ece77bc487c8aa697e3c092197621c4f3e5ccb8/docs/attacksurface.md
[mullvad-security]: https://github.com/mullvad/mullvadvpn-app/blob/b0a8f9e9dc91d772b67c6aee89767f776197b302/docs/security.md
[interprocess-guard]: https://github.com/kotauskas/interprocess/blob/885d96bd89a94fa03e7f813c3e3017ebdd8f9a38/src/os/windows/impersonation_guard.rs
[interprocess-debug]: https://github.com/kotauskas/interprocess/blob/885d96bd89a94fa03e7f813c3e3017ebdd8f9a38/src/misc.rs
[gd-tag]: https://github.com/godaddy/hardware-enclave/blob/3b4ac1bcb637fb60ac18d4cd9877dba989c46dba/crates/hardware-enclave/src/internal/windows/meta_tag.rs
[gd-marker]: https://github.com/godaddy/hardware-enclave/blob/3b4ac1bcb637fb60ac18d4cd9877dba989c46dba/crates/hardware-enclave/src/internal/windows/meta_migration_marker.rs
[keepass-save]: https://github.com/keepassxreboot/keepassxc/blob/c263a116d96326a07efb3ea9de47f7b917f82687/src/core/Database.cpp
[memguard-buffer]: https://github.com/awnumar/memguard/blob/a5d81463bf244cf7b7867ecc599028659effe072/core/buffer.go
[nv-task]: ../../tasks/windows-protected-storage.md
[nv-development]: development-plan.md#84-protected-optional-storage-design-and-qualification
