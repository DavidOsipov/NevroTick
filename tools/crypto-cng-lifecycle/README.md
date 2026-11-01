# Held CNG lifecycle fixture

This GPL-2.0-only standalone test tool is outside the product Cargo workspace.
It prepares the first CRYPTO-01/02 public-API observations in
[`tasks/s0.1-crypto-feasibility.md`](../../tasks/s0.1-crypto-feasibility.md).
The source has passed the pinned formatter and independent source/API review; it
has not been compiled, tested or executed. Licence/bans,
the exact derivative dependency graph, build and native execution admission
remain held. The earlier S46 advisory/source pass is not this fixture's admission.

The only direct Windows dependencies are exact Rustls 0.23.46 with defaults off
and `std`, and CNG 0.1.2 with defaults off. No verifier or AEAD root is present.
Root must freeze this new graph from the retained lock and verified cache without
new versions. It is not the former 54-external-package proposal; package counts
and any duplicate allowance must come from the fixture's actual selected graph.
The fixture lockfile is now frozen from the verified cache with no new registry
versions/checksums or feature additions (53 lock records, 41 metadata records).
This is resolution evidence; licence/bans and dependency execution remain held.
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

No TLS connection, certificate/trust evaluation, NTS-KE, socket, DNS, service,
clock, independent entropy assessment, secret-erasure or cleanup qualification
is supplied by this fixture. CRYPTO-01–04 and provider/product adoption remain
open. Existing native memory/lifecycle requirements apply to any later admitted
Windows experiment.

The owned deny policy allows only the exact hash-bound presence of three upstream
`untrusted 0.9.0` CI helper files. They are absent from selected Cargo build,
proc-macro and runtime invocation paths. Their install/network/shell effects
are not admitted; all global interpreter/archive checks remain enabled.
