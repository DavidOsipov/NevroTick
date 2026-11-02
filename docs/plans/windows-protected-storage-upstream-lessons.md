# Windows protected storage: upstream lessons and adaptation

**Document:** NV-STORAGE-UPSTREAM-001, revision 1.0<br>
**Date:** 2 November 2026<br>
**Owner:** David Osipov<br>
**Status:** Completed bounded source inventory; proposed adaptations and unexecuted qualification cases. The exact design/provider freeze remains open.<br>
**NevroTick baseline:** [`1e58ef9255e13d62a9fdcd2542c4472e4ea3c8ea`](https://github.com/DavidOsipov/NevroTick/tree/1e58ef9255e13d62a9fdcd2542c4472e4ea3c8ea), tree `063dd1559ecb7e86c563aab17c0cc35fcff0ccb0`, read through an API-derived partial snapshot, not a local Git checkout.

## 1. Purpose, authority and source inventory

This supplement retains useful mechanisms from the completed GitHub research and
turns them into concrete adaptation questions. [Architecture §12.1.2](../windows-time-architecture.md#1212-protected-optional-storage-and-explicit-fallback)
owns the contract; the [protected-storage plan](windows-protected-storage.md)
owns construction, threat scenarios, decisions and execution gates. Its ST-T
scenarios already map to CHECK and DNS-C requirements. Section numbers here are
reading aids, not another acceptance catalogue.

The narrow candidate remains qualified CNG VBS wrapping/signing roots, then
policy-authorized qualified TPM roots for **fresh optional state**, then no
application-managed persistent secret cache where continuation is permitted.
Custom application enclaves remain a separate future choice. None of the
reviewed implementations establishes this complete construction under NevroTick's
final service account, process protections and durable catalogue protocol.

| Reviewed source | Immutable revision | Observed licence | Relevant scope |
| --- | --- | --- | --- |
| Microsoft MPSS | `722dd7627251988ba7457845da08892fee06abb1` | [MIT][mpss-license] | Actual CNG key lookup, VBS classification, signing and mocked failure cases. |
| Tailscale | `57ca6e1e2aeaa8794911c355c124c390e17dcd40` | [BSD-3-Clause][ts-license] | Raw TPM-sealed application state, Windows file replacement and serialization boundaries. |
| Chromium | `7fd43f722dcad28ccbcfb3b6df297f378ffc1965` | [BSD 3-clause style][cr-license] | Windows signing-key admission, scheduling, cancellation and provider tests. |
| Microsoft VbsEnclaveTooling | `ec3ea90ddac245da7c96112dea0e27ee1a78f986` | [MIT][ve-license]; [component notices][ve-notice] | Rust/C++ ABI, memory custody, image policy, Hello and TLS samples. |
| Smallstep crypto | `516f3a2010a5c63c4e71357a3f813e0b9e84d090` | [Apache-2.0][ss-license] | CNG addressing, certificate metadata, handle lifecycle and TPM records. |
| Google go-attestation | `6f394f59b7c0f85d226c51785caab7f4adb19d5f` | [Apache-2.0][ga-license] | Windows PCP creation/reopen, attestation and explicit implementation/CI limits. |

The linked files identify the inspected functions and tests below. Upstream source
and test definitions were reviewed; no upstream code was built, installed or run,
and no native key, service or mitigation experiment was performed. Test presence,
CI configuration and upstream demonstration statements are not our execution
evidence. No source, dependency, algorithm, provider default or weaker fallback
is adopted. Later reuse requires file-level provenance/notices and compatibility
review against the [current chrony-derived licence gate](../windows-time-architecture.md#1871-license-gate-and-finite-provider-choice); the licence table grants no
blanket copying permission.

## 2. Concrete lessons

### 2.1 Admit the actual roots, including after reopen

**Observed:** MPSS's [Windows adapter][mpss-win], `GetVirtualIsolation`, reads
`NCRYPT_USE_VIRTUAL_ISOLATION_PROPERTY` (`Virtual Iso`) on the actual key.
`try_open_key` checks algorithm and rejects private-export and one-time archiving
permissions. Chromium's [Windows implementation][cr-win], `FromWrappedKeyImpl`,
checks intended use and reconstructs the public representation after import.
These mechanisms are stronger than a provider-name or capability-only label.

**Adapt:** evaluate the MPSS property as one actual-key evidence candidate for
each wrapping/signing root; verify returned size, supported semantics and
error distinctions. MPSS does not check the returned DWORD size and treats an
unsupported property as nonisolated. Neither its implementation nor the symbol's
existence establishes a universal provider/build contract, attestation, or
cross-boot proof. Unproven evidence remains Unknown/RecoveryRequired. MPSS's
[README][mpss-readme] also disclaims established generated/imported origin.
Current nonexportability cannot prove the key was never host-visible.
MPSS's reviewed keys are current-user scoped, and its README permits other
programs under that user to open/use/delete them. Neither this model nor a
machine-keyset flag establishes service isolation. Its ECDSA signing support
also leaves the separate wrapping-root operation unqualified.

**Planned cases:** actual-root identity/use/export/archiving mismatch, unreadable
isolation and mixed root modes must fail the corresponding claims; a qualified
reopened pair must retain its admitted fingerprints and evidence. Use ST-T01/05/13
and CHECK-14/15/16/17/22/25, with native reopen evidence still required.

### 2.2 Keep lookup, collision and capability failures distinct

**Observed:** MPSS `OpenKeyInProvider` and [key-probe types][mpss-probe] distinguish
found, not found and operational error. `CreateKeyInProvider` and `create_key`
refuse collisions, including `NTE_EXISTS` at finalization. The
[mock tests][mpss-tests] include unreadable existence probes, names held in two
providers, and a name taken with the VBS flag that must not be retried without it.

**Adapt:** retain typed native status plus operation/phase; bind lookup to the
catalogue's exact provider and root identity. Absence is an adapter observation:
an initialized installation missing an expected key still requires recovery.
Preflight absence cannot authorize overwrite or defeat a competing creator.
Reconcile the same authorized provisioning transaction after a collision.

**Avoid:** MPSS's TPM → VBS → software ladder and broad creation-error fallback.
Its `OutcomeFromStatus` distinguishes collision from other failures broadly;
`SoftwareFallbackSucceedsWithoutSignalingDowngrade` documents its own policy.
Neither that ladder nor automatic provider discovery is NevroTick policy.

**Planned cases:** denial, unknown status, corrupt identity and late collision
must not strip REQUIRE flags, overwrite or select a weaker backend; separately
admitted capability absence may follow existing policy. ST-T02/17;
CHECK-2/3/9/13/15/23/25; DNS-C13/15/17. Mock cases precede native classification.

### 2.3 Carry complete key addresses and linear handle ownership

**Observed:** Smallstep [CAPI operations][ss-capi] preserve provider, container and
keyset scope. `TestCAPIKMS_SearchCertificates_keyContainerName` in the
[Windows tests][ss-tests] deliberately associates a certificate with a nonexistent
container: metadata lookup is not possession. Its [native wrapper][ss-native],
`nCryptDeleteKey`, treats successful deletion as consuming the handle and frees
the handle after failure. Google's [PCP creation][ga-pcp], `winPCP.newKey`, deletes
newly created keys after later failure and frees the handle if deletion fails.

**Adapt:** use an admitted address plus independently checked public identity;
represent handle-consuming deletion explicitly. Preserve raw SECURITY_STATUS,
rather than Smallstep's string-assisted `isBadKeyset` recognition. A destructor
or freed handle cannot establish persisted-object deletion. Failed cleanup must
remain attributable and occupy the bounded retired slot.

Smallstep's [TPM records][ss-tpm], `WasAttested`, tests a recorded name, not an
attestation proof. Google's [Windows reload][ga-tpm], `loadKey`, opens a stored
name but derives reported public identity from serialized bytes without the
required comparison against that newly opened handle in the reviewed function.
Smallstep's experimental Software-KSP import prechecks name existence, but its
source says import can replace that name. The two-step sequence therefore does
not establish atomic no-overwrite against another actor; this is source-derived
reasoning, not a reproduced race. Generated-root provisioning must use its own
qualified collision-safe creation/finalization protocol. Import success must
remain a different origin category from controlled generation.

**Planned cases:** stale metadata, wrong scope/provider/public key and failed
delete cannot gain authority or erase cleanup accounting; correct reopen/delete
neighbors succeed. ST-T05/10/13/16; CHECK-12/13/15/17/18/21/23/25/27.

### 2.4 Verify Windows security across file replacement

**Observed:** Tailscale's [WriteFile][ts-atomic] writes a neighboring temporary file,
syncs and closes it before rename. Its [Windows replacement][ts-win], `rename`
and `replaceFile`, use `ReplaceFileW`, falling back to ordinary rename only for
`ERROR_FILE_NOT_FOUND`. [TestPreserveSecurityInfo][ts-acl-test] creates a distinctive
harmless deny ACE and compares the destination DACL before and after replacement.

**Adapt:** reuse that independent DACL-preservation test shape. Separately qualify
initial creation, parent-directory security, final owner/inheritance, replacement
rights, owner-mediated ACL changes and another LocalService service's denial.
The source explicitly notes that POSIX-style file mode arguments are ignored on
Windows; a `0600` argument supplies no such evidence.

`ReplaceFileW` is a candidate publication primitive, not proof of crash-safe
catalogue/provider multi-object commit. Preserve the installation witness,
prepared-content ordering and digest binding of the **exact committed envelope**.
An older correctly signed snapshot under the same epoch is not interchangeable.

**Planned cases:** intended writer succeeds; second-service access/replacement
fails; DACLs survive save; each prepare/flush/replace/acknowledge interruption
preserves declared eligibility. ST-T03/05/10; CHECK-1/12/13/15/25/27;
DNS-C13/15. These need native account/filesystem observations.

### 2.5 Make plaintext serialization difficult and loss explicit

**Observed:** Tailscale [tpm.go][ts-store] separates `decryptedData` from
`encryptedData`; `decryptedData.MarshalJSON` deliberately returns an error.
`newStore` handles an absent file separately from read failure, malformed JSON,
missing fields, unseal failure and authentication failure. Those existing-file
errors do not silently generate replacement state.

**Adapt:** keep private secret-bearing types outside generic persistence,
diagnostic, export and status DTOs; expose only bounded purpose-specific encrypted
records or redacted status. The serialization barrier is useful defense in depth,
not protection from arbitrary copies, formatting or host-memory compromise.
Distinguish authorized empty optional state, missing optional snapshot and missing
expected roots/catalogue through the existing witness/catalogue protocol.

**Avoid:** the store's cache-before-persistence publication, reused symmetric key
across ordinary writes, and [bidirectional migration][ts-migration] based on JSON
shape. Its Windows backend uses [raw TPM transport][ts-transport], not CNG PCP;
secretbox and raw sealing do not qualify our wrapping/signing construction.
Migration can intentionally produce plaintext; this is not automatic recovery
after failed TPM unseal and must not be described as such.

**Planned cases:** sentinel secrets cannot escape via serialization, logs, staging
or exports; corruption cannot masquerade as first use; an authorized empty store
permits the existing fresh-acquisition policy. ST-T02/05/14;
CHECK-9/15/17/22/25; DNS-C13/14.

### 2.6 Retain native custody after cancellation

**Observed:** Chromium's [BackgroundLongTaskScheduler][cr-scheduler] separates one
running task from priority queues and measures wait/run/total time.
[CancelPendingTask and CancelRunningTask][cr-scheduler-tests] distinguish a queued
operation that never starts from running work whose callback is suppressed while
its completion counter still advances. The [task manager][cr-manager] separately
tracks operation errors/retries.

**Adapt:** bound queue entries/bytes, running jobs, retry work and retained results
at admission. Retain job charges, handles and sensitive buffers through any
recovery state until the real native operation completes. An explicit custody
transfer moves those obligations to the responsible owner; it does not release
them. On policy/root/privacy invalidation, suppress publication and wipe owned results;
dropping the awaiting future does not release native custody. Preserve fairness
and bounded redacted local timing/error diagnostics.

The inspected scheduler has no queue admission cap, and retry logic retains a
backoff TODO. This is a scoped observation, not a claim about all Chromium.
Browser telemetry, retry counts and unbounded queues are not copied.

**Planned cases:** pending cancellation performs no operation; running cancellation
retains its charge until actual completion; delayed success cannot publish after
invalidation; accepted neighboring work and shutdown remain usable. ST-T10/16;
CHECK-12/13/15/17/21/25/27; DNS-C12/13/15. Model and native evidence stay separate.

### 2.7 Qualify specific provider operations and expose skipped coverage

**Observed:** Chromium [key tests][cr-tests] parameterize provider/algorithm
combinations and use `GTEST_SKIP` for missing hardware/support. MPSS's relevant
Windows tests mock CNG/TBS. Tailscale [TPM tests][ts-tests] skip unavailable devices
and use a mock provider for migration. Google's [Windows key][ga-key],
`windowsKey20.decrypt`, returns unimplemented; its [Windows CI job][ga-ci] runs
`go build ./...` despite a step named Test. These are different evidence layers.

**Adapt:** require explicit rows for wrapping, signing, create/open/use/retire,
algorithm/padding/hash/encoding, generated/imported origin, account, Windows/SDK
build and required process profile. Record Passed, Failed, Unsupported and Not
run accurately, with skips and controls. A signing round trip, simulator, build
or selected-provider `IsHardwareBacked` label cannot qualify an unwrap root or
actual VBS origin. Chromium `GenerateKeyImpl`'s explicit usage/scheme/hash choices
are useful prompts, not our approved algorithm suite.

**Planned cases:** unsupported rows remain incomplete; fake/model successes cannot
populate native columns; both actual roots must satisfy the selected row and
healthy controls. ST-T01/12/13; CHECK-14/15/16/17/18/20/22/23. No reviewed project
closes the required Windows pair, silent-service or cross-boot gate.

### 2.8 Copy hostile enclave input before parsing, cap before allocation

**Observed:** VbsEnclaveTooling's Rust [enclave_helpers.rs][ve-helpers],
`call_vtl1_export_from_vtl1` and `call_vtl0_callback_from_vtl1`, copy host context
and payload before unpacking and converting typed values. The
[C++ helpers][ve-cpp-helpers] follow a corresponding boundary. This limits
host mutation during interpretation and makes buffer ownership explicit.

**Adapt only if a full enclave is selected:** validate copied lengths with
overflow-safe arithmetic and hard byte/element/depth/aggregate budgets **before**
allocation; then structural decoding, private semantic construction, current
authorization and generation checks. The inspected Rust helpers allocate vectors
from copied host lengths before FlatBuffers validation, without our required cap
in those functions. C++ allocation precedes unpacking too. A later application
response cap cannot bound earlier marshalling allocations.
The Rust [boundary test mocks][ve-mocks] replace enclave-copy APIs with ordinary memory
copies and synthetic bounds. Such tests can exercise decoder behavior and
ownership sequencing, while native strict-memory enforcement requires a separate
observation. A structurally valid request still needs current purpose, key,
policy and generation authorization at the effect.

The [allocator][ve-allocator] panics on allocation failure; the
[HelloWorld sample][ve-rust-hello] supplies an infinite-loop panic handler.
This source combination is a product-adaptation concern, not a reproduced hang.
Ordinary vector destruction is not a complete secret-wiping contract.

**Planned cases:** oversized/truncated/racing inputs and malformed nested fields
fail before excess allocation or effects; bounded valid neighbors succeed;
failed/invalidated outputs are cleared. ST-T11/14/16;
CHECK-15/17/21/22/25/27. Generated ABI tests alone cannot prove VTL isolation.

### 2.9 Reject reentry without spinning and charge internal work

**Observed:** the C++ TLS enclave [TlsSample_RunScenario][ve-tls] uses a
nonspinning atomic admission flag. Its `RunGuard` covers session destruction and
the transport-close callback. The accompanying rationale identifies reentry during
an untrusted host callback: spinning while the outer call waits can deadlock.
`FindScenario` retains endpoint, trust and response policy inside the enclave;
the exposed result contains bounded facts, not decrypted response bytes.

The [TLS driver][ve-tls-driver], `BioContext::Charge`, counts top-level retry work
and individual BIO callbacks, bounds receive requests and checks returned lengths.

**Adapt:** keep a narrow operation/purpose interface with trusted policy and
generation-scoped state. Reject prohibited nested/concurrent calls, retain
ownership through final callbacks, and charge internal callbacks/loops rather
than only top-level requests. A counter cannot force a hostile callback to return;
wall-clock and shutdown claims still need completion evidence. The same
preallocation gap from §2.8 applies to callback returns.
The driver also distinguishes authenticated TLS `close_notify` from bare TCP
EOF. The transferable lesson is to report completion only from the protocol's
actual success condition; transport closure or a completed host callback alone
cannot certify the protected operation's result.

**Planned cases:** nested entry fails promptly without mutating the outer
operation; callback floods/invalid lengths exhaust a bounded allowance; permitted
work completes. ST-T11/16; CHECK-15/17/21/25/27. The sample disables time facilities
and does not establish our NTS certificate-time/revocation rules or adopt Mbed TLS.

### 2.10 Reject repeated-key/fixed-nonce and plaintext-output sample patterns

**Observed:** the Hello [user-bound sample][ve-userbound],
`EnsureUserBoundKeyLoaded`, caches its key; encryption repeatedly supplies
`crypto::zero_nonce`. The [crypto helper][ve-crypto] uses
`BCRYPT_AES_GCM_ALG_HANDLE`. A cached AES-GCM key with a fixed nonce for distinct
messages is an unsafe repeated-use construction under [NIST SP 800-38D §8][nist-gcm]'s
key/IV uniqueness requirement. This is a source-level conclusion, not a tested
exploit or claim about every SDK operation.

**Adapt:** retain the plan's fresh-DEK-per-independent-attempt and frozen
nonce/RNG-use contract, including concurrency, retries, failed publication and
restored state. Do not treat the sample's `NoCache` authentication setting as
per-operation verification of its retained application key. No suite or constant
is selected here.

The sample also returns plaintext to the [host][ve-userhost], which prints it;
enclave debug output includes plaintext. Interactive Hello and debug creation are
not an unattended-service recipe or complete live-secret protection.

**Planned cases:** different/failed/retried/restored writes obey the admitted
key/nonce lifetime; RNG failure stops persistence; sentinel plaintext remains
absent from host outputs and diagnostics wherever the chosen boundary promises
that protection. ST-T14/18; CHECK-15/17/22/25/27; DNS-C14.

### 2.11 Admit the final enclave image and process profile independently

**Observed:** current [VbsEnclaveTooling][ve-readme] contains Rust and C++ support;
the Rust sample uses `no_std`/`alloc` and a restricted import surface.
[Rust restriction code][ve-ffi] varies with debug assertions, while the
[C++ sample image][ve-dll] includes both strict-memory and debuggable policy.
The [signing script][ve-sign] runs VEIID/page-hash signing but can exit successfully
without a certificate and accepts a warning exit status. These are sample choices.

**Adapt if selected:** freeze compiler, SDK, generator, generated ABI, imports,
production signing and actual image/create/runtime policy. Inspect the final
artifact and qualify its effective behavior under the unchanged role-specific
mitigation profile. Neither a Release label nor `/GUARD:MIXED` demonstrates
ACG/CIG/StrictCFG/CET compatibility. Current Rust support also does not admit
arbitrary Rust dependencies into an enclave; obsolete proof-of-concept limitations
must not be used to reject current support wholesale.

**Planned cases:** missing/unadmitted signature, debug identity, wrong imports or
weakened required profile fail admission; the exact supported image succeeds in
the intended service environment. ST-T12/13; CHECK-15/16/18/20/23. No test-signing,
interactive prompt, extra privilege or machine-policy workaround is authorized.

### 2.12 Keep durable generations and privacy above library convenience

**Observed:** MPSS [generation helpers][mpss-generations] invalidate handles for
deletions recorded within the current process. Smallstep `TPM.DeleteKey` removes
its metadata even when underlying deletion fails, returning `PartialDeleteError`.
Tailscale's migration/export surface serves its own format transitions. These
mechanisms do not implement NevroTick's durable retirement and privacy protocol.

**Adapt:** preserve separate root/data/privacy epochs, exact committed-envelope
identity and bounded active/prepared/retired slots. A failed cleanup remains
attributable; another decryptable or newly enumerated object cannot become
current. VBS return creates only an admitted fresh epoch, without resurrecting
retired DNS history or claiming that rewrapping removes previous exposure.

**Planned cases:** forget, exclusion, pause and persistence-off invalidate retained
readers, inaccessible VBS blobs, prepared writes and delayed results before the
existing completion acknowledgement; valid new learning follows current policy.
Test an intact catalogue against an older valid signature and the exact committed
snapshot. ST-T05/06/07/10/15/17; CHECK-2/3/10/11/12/13/15/23/25/27;
DNS-C11/13/14/15/17. Optional loss/reset never repairs the mandatory broker ledger,
refunds correction authority, clears a trusted floor or weakens NTS/DNS rules.

## 3. Priority, remaining gates and handoff

The source inventory is **complete for this bounded research question**. It adds
concrete failure cases and a candidate per-key VBS signal, but the
[source-qualified design freeze](windows-protected-storage.md#8-staged-execution-and-evidence-gates)
is **not complete**. Keep remaining work in that existing stage:

For every selected case, record the admitted input, independent expected result,
protected and forbidden effects, valid neighboring control, source owner and
required environment in the existing task. Preserve selected/executed counts,
skips, setup failures, cancellations and cleanup failures. A source-derived test
idea becomes evidence only when the relevant candidate and environment actually
execute it; failed setup remains incomplete rather than a behavioral failure or
pass.

1. **Storage/security/platform: freeze one exact candidate row or its blocker.**
   Establish both wrapping and signing operations, actual-root/reopen evidence,
   algorithm/padding/hash/encoding, canonical signed bytes and committed digest,
   RNG/nonce limits, Windows/SDK/provider builds, service account, descriptor and
   finalization order, and unchanged required mitigation profile. MPSS narrows
   the evidence question; it does not settle cross-boot `Virtual Iso` semantics.
2. **Storage/platform: finish the bounded lifecycle specification.** Bind initial
   witness consumption, explicit no-root state, collision/recovery classification,
   exact durable boundaries, handle ownership, queue/inventory budgets and failed
   cleanup reconciliation. Pair hostile controls with a valid useful neighbor.
   Source/model/native/service/package evidence must occupy separate columns.
3. **Owner decisions: prepare only consequential unresolved choices.** Use the
   existing NV-DEC-001 procedure for product default/strict-policy selection,
   a materially different construction or stronger live-secret isolation goal.
   Research completion is neither owner approval nor risk acceptance.

Native root qualification remains **held by the experiment pause**. When that
pause is lifted and a row is ready, use owned disposable Windows fixtures,
dummy namespaces, final tokens and bounded cleanup. Required observations include
second-service denial, no-login SCM operation, actual create/use/reopen/retire,
VBS-off/return, key/account/TPM loss, crash/disk-full boundaries, and the intact
required profile. No public time service or real clock change is necessary.

The [development plan](development-plan.md) retains current phase priority.
Subsequent NTS restoration stays in S2.3/S2.4, persistence/lifecycle in S3.3,
future DNS work under its own DNS-C gates, and notifications in P5/D2. UI-M0
remains fixture-only; D2 remains deferred, including its first authenticated
connection notice. Mandatory ledger continuity and package/release evidence stay
independent. Changed providers, key origin, algorithms, serialization, account,
Windows build, mitigation or package invalidate the corresponding evidence and
require scoped requalification; this document supplies no executed acceptance.

[mpss-license]: https://github.com/microsoft/mpss/blob/722dd7627251988ba7457845da08892fee06abb1/LICENSE
[mpss-win]: https://github.com/microsoft/mpss/blob/722dd7627251988ba7457845da08892fee06abb1/mpss/impl/windows/mpss_impl.cpp
[mpss-probe]: https://github.com/microsoft/mpss/blob/722dd7627251988ba7457845da08892fee06abb1/mpss/impl/key_probe.h
[mpss-tests]: https://github.com/microsoft/mpss/blob/722dd7627251988ba7457845da08892fee06abb1/tests/mpss_win32_mock_tests.cpp
[mpss-readme]: https://github.com/microsoft/mpss/blob/722dd7627251988ba7457845da08892fee06abb1/README.md
[mpss-generations]: https://github.com/microsoft/mpss/blob/722dd7627251988ba7457845da08892fee06abb1/mpss/impl/windows/win_utils.cpp
[ts-license]: https://github.com/tailscale/tailscale/blob/57ca6e1e2aeaa8794911c355c124c390e17dcd40/LICENSE
[ts-store]: https://github.com/tailscale/tailscale/blob/57ca6e1e2aeaa8794911c355c124c390e17dcd40/feature/tpm/tpm.go
[ts-transport]: https://github.com/tailscale/tailscale/blob/57ca6e1e2aeaa8794911c355c124c390e17dcd40/feature/tpm/tpm_windows.go
[ts-migration]: https://github.com/tailscale/tailscale/blob/57ca6e1e2aeaa8794911c355c124c390e17dcd40/ipn/store/stores.go
[ts-atomic]: https://github.com/tailscale/tailscale/blob/57ca6e1e2aeaa8794911c355c124c390e17dcd40/atomicfile/atomicfile.go
[ts-win]: https://github.com/tailscale/tailscale/blob/57ca6e1e2aeaa8794911c355c124c390e17dcd40/atomicfile/atomicfile_windows.go
[ts-acl-test]: https://github.com/tailscale/tailscale/blob/57ca6e1e2aeaa8794911c355c124c390e17dcd40/atomicfile/atomicfile_windows_test.go
[ts-tests]: https://github.com/tailscale/tailscale/blob/57ca6e1e2aeaa8794911c355c124c390e17dcd40/feature/tpm/tpm_test.go
[cr-license]: https://github.com/chromium/chromium/blob/7fd43f722dcad28ccbcfb3b6df297f378ffc1965/LICENSE
[cr-win]: https://github.com/chromium/chromium/blob/7fd43f722dcad28ccbcfb3b6df297f378ffc1965/crypto/unexportable_key_win.cc
[cr-scheduler]: https://github.com/chromium/chromium/blob/7fd43f722dcad28ccbcfb3b6df297f378ffc1965/components/unexportable_keys/background_long_task_scheduler.cc
[cr-scheduler-tests]: https://github.com/chromium/chromium/blob/7fd43f722dcad28ccbcfb3b6df297f378ffc1965/components/unexportable_keys/background_long_task_scheduler_unittest.cc
[cr-manager]: https://github.com/chromium/chromium/blob/7fd43f722dcad28ccbcfb3b6df297f378ffc1965/components/unexportable_keys/unexportable_key_task_manager.cc
[cr-tests]: https://github.com/chromium/chromium/blob/7fd43f722dcad28ccbcfb3b6df297f378ffc1965/crypto/unexportable_key_unittest.cc
[ss-license]: https://github.com/smallstep/crypto/blob/516f3a2010a5c63c4e71357a3f813e0b9e84d090/LICENSE
[ss-capi]: https://github.com/smallstep/crypto/blob/516f3a2010a5c63c4e71357a3f813e0b9e84d090/kms/capi/capi.go
[ss-native]: https://github.com/smallstep/crypto/blob/516f3a2010a5c63c4e71357a3f813e0b9e84d090/kms/capi/ncrypt_windows.go
[ss-tests]: https://github.com/smallstep/crypto/blob/516f3a2010a5c63c4e71357a3f813e0b9e84d090/kms/capi/capi_windows_test.go
[ss-tpm]: https://github.com/smallstep/crypto/blob/516f3a2010a5c63c4e71357a3f813e0b9e84d090/tpm/key.go
[ga-license]: https://github.com/google/go-attestation/blob/6f394f59b7c0f85d226c51785caab7f4adb19d5f/LICENSE
[ga-pcp]: https://github.com/google/go-attestation/blob/6f394f59b7c0f85d226c51785caab7f4adb19d5f/attest/pcp_windows.go
[ga-tpm]: https://github.com/google/go-attestation/blob/6f394f59b7c0f85d226c51785caab7f4adb19d5f/attest/tpm_windows.go
[ga-key]: https://github.com/google/go-attestation/blob/6f394f59b7c0f85d226c51785caab7f4adb19d5f/attest/key_windows.go
[ga-ci]: https://github.com/google/go-attestation/blob/6f394f59b7c0f85d226c51785caab7f4adb19d5f/.github/workflows/test.yml
[ve-license]: https://github.com/microsoft/VbsEnclaveTooling/blob/ec3ea90ddac245da7c96112dea0e27ee1a78f986/LICENSE
[ve-notice]: https://github.com/microsoft/VbsEnclaveTooling/blob/ec3ea90ddac245da7c96112dea0e27ee1a78f986/NOTICE.md
[ve-readme]: https://github.com/microsoft/VbsEnclaveTooling/blob/ec3ea90ddac245da7c96112dea0e27ee1a78f986/README.md
[ve-helpers]: https://github.com/microsoft/VbsEnclaveTooling/blob/ec3ea90ddac245da7c96112dea0e27ee1a78f986/rust/edlcodegen/crates/libs/edlcodegen-enclave/src/enclave_helpers.rs
[ve-mocks]: https://github.com/microsoft/VbsEnclaveTooling/blob/ec3ea90ddac245da7c96112dea0e27ee1a78f986/rust/edlcodegen/crates/tests/edlcodegen/tests/mocks/mock_functions.rs
[ve-cpp-helpers]: https://github.com/microsoft/VbsEnclaveTooling/blob/ec3ea90ddac245da7c96112dea0e27ee1a78f986/src/ToolingSharedLibrary/Includes/VbsEnclaveABI/Enclave/EnclaveHelpers.h
[ve-allocator]: https://github.com/microsoft/VbsEnclaveTooling/blob/ec3ea90ddac245da7c96112dea0e27ee1a78f986/rust/edlcodegen/crates/libs/edlcodegen-enclave/src/enclave_global_allocator.rs
[ve-rust-hello]: https://github.com/microsoft/VbsEnclaveTooling/blob/ec3ea90ddac245da7c96112dea0e27ee1a78f986/rust/edlcodegen/crates/samples/helloworld/enclave/src/lib.rs
[ve-tls]: https://github.com/microsoft/VbsEnclaveTooling/blob/ec3ea90ddac245da7c96112dea0e27ee1a78f986/SampleApps/Tls/Cpp/TlsEnclave/tls_enclave.cpp
[ve-tls-driver]: https://github.com/microsoft/VbsEnclaveTooling/blob/ec3ea90ddac245da7c96112dea0e27ee1a78f986/SampleApps/Tls/Cpp/Common/TlsMbedTlsDriver.cpp
[ve-userbound]: https://github.com/microsoft/VbsEnclaveTooling/blob/ec3ea90ddac245da7c96112dea0e27ee1a78f986/SampleApps/SampleApps/SampleEnclave/UserBound/userbound_keys.cpp
[ve-crypto]: https://github.com/microsoft/VbsEnclaveTooling/blob/ec3ea90ddac245da7c96112dea0e27ee1a78f986/src/VbsEnclaveSDK/src/veil_enclave_lib/crypto.vtl1.h
[ve-userhost]: https://github.com/microsoft/VbsEnclaveTooling/blob/ec3ea90ddac245da7c96112dea0e27ee1a78f986/SampleApps/SampleApps/HostAppUserBound/main.cpp
[ve-ffi]: https://github.com/microsoft/VbsEnclaveTooling/blob/ec3ea90ddac245da7c96112dea0e27ee1a78f986/rust/edlcodegen/crates/libs/edlcodegen-enclave/src/enclave_ffi.rs
[ve-dll]: https://github.com/microsoft/VbsEnclaveTooling/blob/ec3ea90ddac245da7c96112dea0e27ee1a78f986/SampleApps/SampleApps/SampleEnclave/dllmain.cpp
[ve-sign]: https://github.com/microsoft/VbsEnclaveTooling/blob/ec3ea90ddac245da7c96112dea0e27ee1a78f986/rust/scripts/sign-enclave.ps1
[nist-gcm]: https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-38d.pdf
