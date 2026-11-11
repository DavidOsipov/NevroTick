# CNG lifecycle feasibility fixture

This GPL-2.0-only standalone test tool is outside the product Cargo workspace.
It supplies the first CRYPTO-01/02 public-API observations in
[`tasks/s0.1-crypto-feasibility.md`](../../tasks/s0.1-crypto-feasibility.md).
The task's S48–S58 checkpoints record dependency policy, Windows builds, an actual
native cleanup RED and its owning-guard correction. All six corrected native
cases pass: three initialization/error controls and three public X25519 cases.
The earlier S52 outer-launch accounting gap remains historical. Native memory,
cached-error/shutdown, concurrency and broader provider qualification remain open.
Use the task's exact candidates and evidence limits when reusing observations.

**11 November EP1 source preparation:** `nevrotick-nts-ke-probe` now delegates to
the Engine-owned private KE/association/request implementation through the
explicit, default-off `ke-association-prototype` feature. The old tool module was
removed after relocation; peer/init binaries remain separate. The explicit
[compiler consumers](tests/contracts/README.md) now challenge the actual external
library boundary. Graph admission and fresh compiler/behavioral qualification of
this migration remain pending under the [existing EP1 task](../../tasks/engine-transport-pilot.md#ep1--actual-ke-producer-into-engine-ownership).
Passes and graph descriptions below retain their exact historical source scopes;
they do not qualify this changed candidate. Native execution remains separately
held.

The historical S51 public-API checkpoint's direct Windows dependencies were exact
Rustls 0.23.46 with defaults off and `std`, and CNG 0.1.2 with defaults off.
That candidate had no verifier or AEAD root. Its frozen lock contained 53 records
and metadata contained 41 records, without new registry versions/checksums or
feature additions. Its Windows normal/build graph had 34 unique identities, 33 external;
the actual S51 compiler records match it, with no ring/AWS-LC compiler artifact.
That minimal graph required no duplicate-version allowance. It is separate from
the former 54-external-package proposal and admits no product dependency graph.
Do not run the upstream CNG test manifests or
dev graph: those explicitly request AWS-LC and are outside this experiment.

The fixed CLI is `--case healthy`, `--case peer-short` or `--case peer-long`.
Each case requires a fresh child process so its process-global provider cache is
fresh. Argument admission precedes CNG startup. Unknown/missing/extra arguments
return exit two. A non-Windows executable returns `status=error`,
`stage=platform`, `cause=unsupported`, all counts zero and exit one.

On Windows, `healthy` creates exactly one `ShutdownHandle` first, starts two
native X25519 exchanges, checks distinct exact 32-byte public shares, completes
each against the other's share, and checks equal, nonzero 32-byte secrets. The
exchange/secret owners drop before the shutdown owner. This checks a key-agreement
property; the public API supplies no fixed private key for an independent known
answer vector, and this tool claims no algorithm interoperability qualification.

`peer-short` and `peer-long` each consume one exchange with an explicitly nonzero
31- or 33-byte peer encoding. The independent expected cause is the provider's
typed `PeerMisbehaved(InvalidKeyShare)`: recovered CNG `src/kx.rs:172–177`
rejects every X25519 length other than 32 before importing a peer key. This is
not a zero-peer oracle. A fresh healthy pair then runs in the same process to
check that the rejected exchange has not disabled its neighbor. Acceptance,
another error or a failed neighbor makes the entire case fail.

The single ASCII result contains only the case, status, stage, closed cause and
actual provider-API call counters, not all internal BCrypt calls. A successful healthy case has two starts/completions and
zero rejections. A successful malformed case has three starts/completion attempts,
two completions and one rejection. Counters record actual attempted/returned
calls, including partial progress on failure. No public-share or secret bytes,
hashes or raw provider error strings are emitted. A write failure returns one.
Dependency panics and native faults are never caught as expected success; the
parent must retain the native exit and stderr, even when no result is produced.

An admitted ordinary-user Windows parent must own one child at a time, bound
wall time and output, drain its pipes, and retain it through actual exit/reaping.
Native calls are synchronous and have no timeout inside this fixture. A timeout
does not prove completion or cleanup. The final report follows the shutdown drop,
but the upstream destructor discards native close status, so neither exit zero
nor process HandleCount proves algorithm-handle release or native leak freedom.

CNG documents one shutdown owner created at program start and dropped at its end.
This tool follows that obligation; it does not test early/repeated shutdown or
post-shutdown reuse as valid caller behavior. CRYPTO-02's partial-initialization
failure/cleanup and cached-initializer panic still need separately reviewed owned
fault instrumentation or correction that preserves actual native failure causes.

Those public-API observations supply no TLS connection, certificate/trust
evaluation, NTS-KE, socket, DNS, service, clock, independent entropy assessment,
secret-erasure or cleanup qualification. CRYPTO-01–04 and provider/product adoption remain
open. Existing native memory/lifecycle requirements apply to any later admitted
Windows experiment.

## Bounded ordinary-validation KE producer source

**9 November handoff source amendment:** the newly authored private receiver
consumes the actual same-connection C2S key and first exact cookie through the
existing algorithm-15 witness, after successful TLS closure and before CNG
shutdown. It constructs one untransmitted packet using CNG UID32/nonce16 entropy;
S2C remains separately owned. The prepared packet retains its original control
until the fresh final commit. Cancellation, expiry or refusal returns no success;
all application-owned secret buffers retire before reporting counts. The fixed
zero-T1 header is a never-send laboratory input. Successful output adds
`sealed_packet_bytes` and `transmitted=0`, never packet/key/cookie bytes.

The current combined graph passes exact Linux/Windows admission; the receiver
passes 40 units plus two actual CLI cases per ordinary/verify-release/release
profile, both-target lint/compiler checks and nine actual privacy consumers.
The [ordinary checkpoint](../../tasks/s0.1-crypto-feasibility.md#actual-handoff-ordinary-checkpoint--9-november-2026)
retains the initial failures and exact scope. The
[final source/host checkpoint](../../tasks/s0.1-crypto-feasibility.md#handoff-sourcehost-final-qualification--9-november-2026)
passes the two selected line gates (51/54 handoff; 25/25 request) and the separate
eight-fault-per-file manual instrument. Generated outcomes remain handoff seven
caught/two unviable and request one unviable, with no request score. Native
qualification remains open. Earlier results below retain their historical
source scopes; product manifests/provider selection, ordinary trust/native
cleanup and complete S2.2 remain separate. See the
[reviewed handoff contract](../../tasks/s0.1-crypto-feasibility.md#s22-actual-ke-to-request-laboratory-handoff--9-november-2026).

`nevrotick-nts-ke-probe` adds the actual Windows CNG/Rustls TLS-1.3-only connection,
ordinary strict-profile verifier, NTS-KE exchange and both regular exporters.
It admits exactly a numeric loopback peer with nonzero port and a separately
checked verification identity. There is no DNS, alternative address, client
certificate, resumption, early data, fake-root path or plain-NTP fallback.
This is source under qualification: **do not run it while Windows execution is
held**. The current checkpoint passes 24 host helper tests plus two actual CLI
integration tests in ordinary,
`verify-release` and `release` profiles, host Clippy,
Windows-target compilation and Clippy. Earlier private Windows-target rustdoc
and seven retained actual-module compiler consumers retain their recorded
legal-use/private-owner refusal scope, without execution. A primary TLS-error precedence repair
has actual-code RED/GREEN: later alert failures preserve the earlier invalid-data
cause, while sticky faults still refuse nominal success. These checks execute no
Windows TLS, verifier or exporter. Native permission, remaining contract controls
and source-bound coverage/mutation remain separate
gates in the [existing S2.2 source contract](../../tasks/s0.1-crypto-feasibility.md#s22-real-laboratory-keexporter-producer--source-contract-9-november-2026).
The later [10 November native refusal checkpoint](../../tasks/s0.1-crypto-feasibility.md#native-ke-refusal-checkpoint--10-november-2026)
adds the exact non-test release/v3 link, separate static readback and five actual
guest argument/connect/TLS-refusal cases. Malformed TLS and EOF preserve distinct
primary errors and their secondary cleanup diagnostics; they establish no
successful TLS closure, certificate verification, healthy KE/exporter or native
cleanup result. The first build remains incomplete at its post-link five-GiB
floor. The prepared Windows 26100 guest, configured SAC Off and process-only
PowerShell `RemoteSigned` are laboratory scope, not the release OS/security
profile. The [healthy fixture checkpoint](../../tasks/s0.1-crypto-feasibility.md#source-preparation-and-resource-stop--10-november-2026)
owns the new peer-only binary, guest store helper and unexecuted controls.
The peer changes only this tool's binary inventory; its dependencies and the
qualified client/verifier remain unchanged. Preparation stopped before guest
dispatch on insufficient host memory. Compile/native work awaits restored
headroom and David's continuation; all remaining gates stay open.

The authored peer CLI is `nevrotick-nts-ke-peer LEAF_DER LEAF_PKCS8_DER READY_PATH`.
All three paths belong to the owned guest's private NTFS directory. It accepts
one loopback connection, requires the literal 16-byte offer, replies with the
literal 24-byte one-cookie response and checks TLS close-notify. Readiness is
only `port=<number>\n`; the final stdout reports fixed counts. No certificate,
cookie, exporter or signing-key bytes are logged. The original key read buffer
is wiping-owned; the DER copy consumed by the provider and native copies remain
unqualified. This is uncompiled fixture source, not a runnable/admitted peer.
The separately [reviewed guest controller](../windows-sandbox/nts-ke-healthy-peer.ps1)
owns the fixed untrusted → missing-status → healthy → wrong-identity sequence,
bounded child outputs and exact temporary-store restoration. It remains
unparsed/unexecuted; unknown child completion retains recovery custody and
cannot become a successful result or automatic next case.

The raw host coverage survey also exercises the actual unsupported-platform
entrypoint and failed diagnostic sink; it does not execute Windows transport.
Its exact 108-item mutation listing identifies constructor, generic-helper and
Windows-generation gaps. Three isolated host controls detect the selected
propagation/entrypoint defects. Cargo-discovered `host_refusal` now catches the
listed default-success entrypoint through the same command that passes all 24
healthy tests: the defective binary compiles, all 22 units pass and both actual
CLI tests fail for exit zero versus required exit one. Future admitted mutation
selection must retain both `--bin nevrotick-nts-ke-probe --test host_refusal`.
Four private constructor controls additionally exercise the raw peer budget,
typed verification identity, origin and deadline bindings. All 32 host-present
generated descriptors are now accounted: 14 caught, 18 compiler-unviable with
no kill credit, no survivor or timeout. An EOF detector now rejects a second
read after EOF instead of hanging on the defective variant. The other 76
Windows-only descriptors remain unexecuted; the generic resolver's empty viable
denominator has no passing score. Fresh raw host coverage retains the actual
CLI counter, inline-test contribution and additional legacy executable's failed
threshold observation. No complete critical-file coverage/mutation or native
gate is qualified by this bounded host checkpoint.

The synchronous job retains real native custody; timeout never means a native
verifier stopped. Connect/full handshake share the original origin plus twenty
seconds; KE commit and ephemeral expiry use that same origin plus sixty seconds.
Equality refuses. Stop and commit have explicit serialized outcomes. Response
bytes, record count, scratch and copied cookie/key payloads have independent
limits. Ordinary-drop zeroization covers application-owned buffers, not all
library/OS copies or abort/crash cleanup. Known local close-notify flush failure
prevents commit; EOM does not require an unbounded wait for the peer's alert.

Only closed cause labels and bounded historical counts leave the job, after
secret/TLS/shutdown owners have dropped. A successful laboratory result creates
no current production authentication, Core eligibility, UDP, service or clock
authority. The prospective endpoint remains bound to the saved actual peer and
pre-admitted identity mapping. Production source/network/trust generation and
installed principal/store/egress enforcement remain open before adoption.

The source graph now names the owned verifier 0.7.1, `ntp-wire`, and exact already
selected `rustls-pki-types` 1.15.1 / `zeroize` 1.9.1. The latter pure dependencies
support host-side admission/response controls; they provide no host TLS route.
The complete producer projection passes frozen Windows and host graph/policy
checks against the retained fresh advisory/index snapshot: 39 Windows identities
(four local, 35 registry), four host identities and no host TLS provider. The
actually generated and tested 85-package lock (four local, 81 registry records)
is now published in this standalone fixture. The exact Windows-link 0.1.3/0.2.1
pair retains its reviewed parent/checksum contract; all other bans remain in
force. Re-admit changed candidates before dependent checks; this is no native
or product-provider qualification.

The owned deny policy allows only the exact hash-bound presence of three upstream
`untrusted 0.9.0` CI helper files. They are absent from selected Cargo build,
proc-macro and runtime invocation paths. Their install/network/shell effects
are not admitted; all global interpreter/archive checks remain enabled.

## Private-initialization diagnostic and scoped correction

The owned MIT derivative at `vendor/rustls-cng-crypto` preserves the recovered
0.1.2 production source and original licence. Its `NOTICE` binds upstream revision
`955b52e27ac76041e04459122ab4652875ab39a8`, archive SHA256
`ce24a0492dd515a9def8c39fc6b4ea6797336ca12b2fb1c1f3d05063a9cb9973`
and individual upstream input hashes. Original normalized/original manifests are
retained. The derivative removes upstream test/dev execution edges, keeps all
normal dependencies and existing features/defaults, and adds only the default-off
`fixture-init-probe` diagnostic feature. No upstream dev lock, separate test
directory, CI or AWS-LC dev graph is copied; internal cfg(test) text remains
outside the selected build. The registry copy is unchanged. The derivative is its own
workspace, preventing implicit addition to this fixture's workspace.

The root's existing two exact dependency declarations are unchanged. A local
`[patch.crates-io]` selects the owned derivative. Root feature `init-probe` forwards
the diagnostic feature, and binary `nevrotick-cng-init-probe` requires it. The old
public-API `src/main.rs` is unchanged. No product workspace is modified. The frozen
fixture lock replaces only the old CNG registry node with its owned path node;
the coordinator publishes the actually generated lock through the task's explicit
bridge. No registry version/checksum is added. Earlier licence/build/native
receipts retain their earlier source identities.

The fixed new CLI cases are `--case healthy-load`, `--case open-invalid-once` and
`--case property-invalid-once`, one safe diagnostic call per fresh process. A
process-wide atomic admission rejects a second call before native work. The safe
entry calls the derivative's actual private `load_algorithm`; it is not a copied
proxy helper. Only feature-gated wrappers change the first native input to a
static terminated UTF-16 invalid name and record the actual returned NTSTATUS.
They return that same status to the initializer. A private algorithm owner now
guards each successful open until transfer or one close attempt. Property failure
closes through that owner and preserves the primary and any close failure;
ownership is disarmed before close, preventing a retry after uncertain cleanup.
Unwind fallback attempts cleanup but cannot return its status to the caller.
Failure means signed NTSTATUS < 0;
positive informational statuses are not misclassified as errors.

Each fault case disarms the override, saves the initial snapshot, attempts fixture
recovery, then calls the same actual initializer once with normal ECDH/25519 inputs
as a healthy neighbor. The healthy-only control calls it once. Fixed storage
admits at most two successful-open handles and eight native
events; the selected sequences need at most two opens, two property calls and two
close attempts. No key exchange, key/secret material or raw handle value is used.

Successfully returned handles belong to the fixture caller and are closed with
role `fixture-close-returned`. Provider error cleanup has the separate role
`provider-close-error`, before the initializer returns. After that initial result
and any normal caller close, a bounded snapshot saves pending unreturned handles.
Only afterward does `fixture-close-recovery` try once to close those orphans. Its
actual status is retained and never credited as provider cleanup. A failed close,
unexpected native output, missing intended native failure or failed healthy
neighbor yields an incomplete observation. A failed open's unexpected nonnull
output is not assumed owned and is not blindly closed.

The corrected property case emits `status=property-failure-cleaned` and exits zero
only after actual property failure, successful provider cleanup, a zero-orphan
snapshot without fixture recovery and a healthy neighbor. The original native
failure remains archived. The diagnostic still emits `status=baseline-red` and
exits three when the actual
open succeeded, the actual property status failed, the initializer returned an
error without ownership transfer, the healthy neighbor succeeded, and the saved
single orphan was successfully closed by subsequent fixture recovery. This is a
causal error-path cleanup witness, not provider qualification or general native
leak proof. Healthy and correctly propagated open-failure controls exit zero;
incomplete/native/output failures exit nonzero. All records have fixed labels,
opaque ordinals and actual signed/hex statuses; no pointers or provider strings.
The snapshot is saved before recovery, but its output is emitted after the safe
probe returns, so it is not a live-process acknowledgement checkpoint. The later
healthy-neighbor result appears only in the final result row, not the snapshot.

These cases construct no `ShutdownHandle`, call no cached initializer and catch
no panic as success. The original cached unwrap and shutdown-init panic behavior
remain a separate gap. The first-party binary retains `#![forbid(unsafe_code)]`;
only the owned provider's narrow native module contains diagnostic FFI. S56
qualifies the declared builds with the diagnostic feature both enabled and
disabled; S57/S58 record six actual native cases. These observations establish
neither general leak freedom nor full provider/TLS/product qualification.
