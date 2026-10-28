# TUF dependency assessment — independent first pass

Reviewed 2026-10-28. Scope: the attached proposal as a candidate for the chrony-derived Rust Windows 11 client, against architecture revision 1.4 §§18.3, 18.6–18.10, OPEN-1 and OPEN-8. This is read-only dependency discovery and source review. No dependency is chosen or admitted; no package was installed, graph resolved, source executed, Windows test run, or external state changed.

## Decision-relevant result

TUF remains a credible protocol-level candidate, but **the inspected Rust clients are not presently admitted dependencies for this product**. Tough 0.24.0 has three material integration gates: its unconditional metadata-crypto dependency retains the held AWS-LC/native licensing question; its verification clock is hardwired to wall time and a persisted highest-observed-time record; and its filesystem persistence implementation does not establish the product's required crash-safe trust-state transaction.

The hypothesis that **tough itself is Apache-2.0-only is false for the inspected 0.24.0 package**. The published manifest says `MIT OR Apache-2.0`, its package contains an MIT license, and its source headers use the same alternative expression. Its MIT option does not select licenses for its dependencies. A separate updater process is not treated here as an automatic cure for the GPL-2.0-only combination question. The accepted source/provenance decision and OPEN-1 remain unchanged. [D1–D4, D11–D13]

Publisher-side use of `tuftool` or `tuf-on-ci` is a different dependency boundary from linking a client into the distributed application. Their signing and CI dependencies need their own qualification; they do not inherently become runtime prerequisites of the installed Windows product. Nothing in this report clears a hypothetical final distribution or authorizes a new process/package architecture.

## Provenance and independence

The attachment SHA-256 is `5b2bf4eabb183dcec647232e0cf1e8533ccef2281db59532d0bd6dd35a0e0694`. The architecture read was `a9bd1432ecb2d04e55b77e1a270eb1a135027d9f4a7988d58d9299367a6ca075`. Relevant conclusions were derived from the proposal, current plan constraints and primary source before peer reconciliation. No peer verdict was used as evidence. The parent requested clarification of the time/persistence call order after receiving the first-pass observation; that clarification did not change the observation.

Guidance read: pinned website `AGENTS.md`, testing contract, `plan-independent-review`, `dependency-updates` in discover mode, security contract and conditional amendments, and `security-crypto-secrets`. Missing operations and crypto assurance references were read from website commit `6530d3a8d44cbe4f829b1e09edd3a1c90cffc087`. Website npm execution instructions were not imported into this design-only Rust assessment. The scratch working directory is not a Git repository (`git status --short` returned that fact); task writes are only this report and `findings.json`.

Twelve focused technical lookup batches were used, with independent primary-resource fetches grouped inside them. Failed exact-version docs.rs fetches were recorded as retrieval gaps, never as evidence of package absence. Registry entries, source commits, source-file Git blobs and archive checksums are kept distinct.

## 1. Current package and source identity

| Candidate | Observed published identity | Source identity and limit | Disposition in this assessment |
| --- | --- | --- | --- |
| `tough` | 0.24.0, published 2026-07-10; registry checksum `35b378d98765c2ae9cdc3e9963ea7e670da8cdd9ee39611b8d722083c7f1ac11`; not yanked | Package VCS metadata points to `98d8eb8b2ce63515d9b4981c938ef6453c5b5771`, path `tough`; normalized manifest and MIT file inspected on docs.rs; source inspected at that commit. Full archive-to-source comparison was not performed. | Research candidate; held on graph, time and persistence gates. |
| `tuftool` | 0.17.0, published 2026-07-10; registry checksum `b6ad45e0e6b070d202dbaaa163d0ba2062b14f79a273b0cf5603a1a4481b2aa9`; not yanked | Same repository release-era source declares 0.17.0. Repository latest release is **tuftool**, not a new tough release. Tuftool package VCS/archive parity was not checked. | Publisher/tooling candidate only. |
| `tuf` / rust-tuf | Latest registry entry observed: **0.3.0-beta14**, 2025-10-20; checksum `52690b52ac55d07c9e2448be7589b7f17f35e09ec24aa5f15127577699484059`; not yanked; Rust 1.80.0 | Release-era source `37b51666633b100a51eb1e6eaf700314e17ccfb4` declares beta14; exact package VCS/archive parity was unavailable. Current develop `219ca7d05818d5dd44ec4e32ab47c5ce64a7bf44` declares beta15/Rust 1.88.0, which is not the published package. | Reasonable comparison candidate; prerelease and ring/time/storage gates remain. |
| `libdd-tuf` | 0.3.1, 2026-08-19; checksum `4f7e7c44ae16b25063087e576b0ec25b46ef3a84c8a2239ea85ef7685fd46fb6`; not yanked | Registry and public API documentation inspected. The guessed DataDog repository's default branch did not match this package identity; no source association is asserted. | Secondary fork lead; no reason established to prefer it; artifact/source/maintenance/license parity unresolved. |
| `tuf-on-ci` / signer | 0.21.0 release, 2026-09-15 | Annotated tag object `39bc129d717f4cd4b3db444c9d323e4bb4016db6` resolves to commit `662382a8295b89a9d538390ce8b9bb85b207a93d`; repo and signer manifests and repo MIT license inspected. No wheel/source comparison or signature verification performed. | Publisher/CI candidate only. |

Sources: D1–D3, D14–D20. The registry checks above establish observed entries, not independent package integrity verification. A source version field or repository default branch is insufficient to identify a registry archive.

## 2. Tough's license and crypto boundary

The package's own MIT option removes the proposed **direct Apache-only** objection. It does not clear the actual linked graph. The normalized 0.24.0 manifest declares mandatory `aws-lc-rs = "1"` and `rustls = "0.23"`, both with dependency defaults enabled. No tough default feature list automatically enables HTTP, but disabling HTTP does not remove those dependencies. Metadata public-key verification in `tough/src/schema/key.rs` and target hashing in `tough/src/io.rs` call AWS-LC directly. [D1, D8, D9]

There is an important distinction: `HttpTransportBuilder::crypto_provider` **does exist**, added in 0.23.0, and can customize TLS cryptography. It cannot substitute for the hardwired metadata signature/digest implementation or erase mandatory manifest edges. Merely using the architecture's CNG TLS provider or a custom `Transport` therefore does not resolve tough's graph gate. [D7, D10]

The previously reviewed AWS-LC 1.18.2 wrapper defaults select `aws-lc-sys`; its source manifest names sys 0.46.0. The sys license expression contains mandatory `AND Apache-2.0`. The inspected ring 0.17.14 manifest similarly says `Apache-2.0 AND ISC`. These are not all-file permissive alternatives. Apache documents GPLv2 incompatibility. This preserves the architecture's existing admission hold; it is not a legal opinion that independently analyzes every possible artifact or combination. Tough's ranges were not resolved, so this report does not pretend 1.18.2/0.46.0 is an actual newly generated tough lockfile. [D11–D13]

An upstream-supported compatible metadata-crypto backend or a separately reviewed source change could be investigated later, but neither has been demonstrated here. Creating a cryptographic implementation or fork is not a small configuration fix, and none is proposed as an accepted dependency.

## 3. Time is a substantial product-specific mismatch

The public `RepositoryLoader` exposes transport, limits, datastore and expiration enforcement; it has no caller-supplied verification-time parameter. At pinned source `Repository::load`, line 343 calls `datastore.system_time()` **before** `load_root` and the remaining metadata validation. `Datastore::system_time` reads `latest_known_time.json`, samples `jiff::Timestamp::now()`, rejects a sample below the stored value, writes the new sample, and then returns it. Safe target reads sample the same routine again. [D4–D6]

This yields two source-supported liveness concerns for software whose purpose includes repairing a bad clock:

1. A future wall-clock sample can be persisted before the subsequent metadata validation rejects the update. After the product corrects Windows time backward, later update attempts can fail on the stored time floor until wall time reaches it again.
2. Even a legitimate smaller backward clock correction can make a previously stored sample temporarily exceed wall time. This is more restrictive than only checking a trusted release's metadata expiry.

These are **source-derived potential failures, not reproduced failures of an installed product**. No time manipulation or library execution occurred. An attacker model, reachable time ranges, duration and recovery behavior still need actual selected-artifact tests. The important planning conclusion is that tough's saved clock sample is not automatically the architecture's authenticated, uncertainty-bounded time evidence.

Turning `ExpirationEnforcement::Unsafe` on is not an adequate resolution: upstream explicitly describes it as disabling expiry security guarantees, and the `load` path samples/persists time regardless of that setting. Deleting the time file or trusted metadata as routine recovery would change a security policy and needs a deliberate, authenticated recovery design; it is not recommended here. A TUF-clock interface, if later introduced or supplied by another client, would still leave TLS certificate time, Windows revocation/root retrieval and OPEN-8 separately unresolved. [D4–D6]

## 4. Persistent directory, root reload and durability are separate obligations

The proposal correctly identifies the default: if `datastore` is absent, tough creates a `TempDir`; its lifetime is owned by the datastore through shared references and cleanup occurs when that owner is dropped. Supplying an existing persistent directory preserves files between processes but does not itself supply protected ACLs, recovery, installation identity or a durable transaction. [D4, D6]

The reload rule also requires explicit integration. `Repository::load` passes the caller's `loader.root` to `load_root`; that function deserializes the supplied bytes at lines 704–705. It does **not** first substitute the datastore's `root.json`. The implementation writes the newly trusted root into the datastore, but the caller must load the retained trusted root on later starts. Repeatedly passing the embedded bootstrap root is not an acceptable substitute. [D5]

Durability must not be inferred from the API's phrase “persistent filesystem.” The inspected `Datastore::create` uses direct `tokio::fs::write`; root updates explicitly remove and then create `root.json`. These statements do not establish an atomic replace, a flushed file/directory transaction, or power-loss recovery. In-process locks belong to each datastore object and do not establish cross-process exclusion. Also, an unparsable saved time value is not taken into the backward-time comparison (`if let Some(Ok(...))`). These observations do not establish an exploit, but they make corrupt/missing state, independent loader instances and interrupted writes concrete admission cases. [D5, D6]

The application needs one protected trust-state owner and an explicit fail-closed/recovery contract for missing, corrupt and unwritable state after bootstrap. The exact fit may require upstream changes or another backend; it has not been proven to be a thin wrapper. Successful Rust filesystem writes are not Windows crash/power-loss qualification.

## 5. Streaming, transport and Windows support

The attachment's complete-stream warning is correct. `read_target` returns bytes before checksum validation. `DigestAdapter` checks the digest only at end-of-stream; dropping the stream, stopping after a prefix, or accepting a successful stream-construction future is insufficient. `save_target` loops until the stream is exhausted, propagates chunk errors, then persists a temporary file. That is useful behavior, not proof of protected staging, exact length, exclusive installer consumption or crash durability in this product. [D5, D9]

The inspected common stream helper enforces a **maximum** byte count. It does not itself establish equality with the signed length. Exact-size behavior of the full selected target path must therefore be included in the independent integration fixtures; no whole-library length-bypass finding is asserted from this partial trace. Cached/resumed downloads require the same completed verification and byte identity at installer handoff.

When enabled, tough's HTTP feature adds reqwest 0.13 with defaults disabled and `stream`/`rustls-no-provider`, plus platform-verifier 0.7. HTTP/2 is a separate feature. Its default transport handles file/HTTP/HTTPS according to enabled support; this is not an application allowlist. The HTTP builder supports finite timeout/retry settings and a custom TLS provider, installs AWS-LC as process default if none exists, and documents proxy environment support. Product policy must separately cover redirects, destination restrictions, proxy environment, TLS verification time, service trust stores and native verification work. [D1, D7]

The pinned upstream workflow includes `windows-latest` in build and integration jobs, with FIPS setup. This is evidence that upstream exercises Windows configurations, **not evidence that those jobs passed on the relevant artifact**, and not a Windows 11 x64/ARM64 support promise. Exact toolchain/native build prerequisites, service-context persistence, path semantics, protected staging and packaging need product qualification. [D10]

## 6. Alternatives and publisher tools

### Rust clients

Rust-tuf is the most direct alternative to compare. Its own project is dual licensed and explicitly warns that its software/API is beta. The published beta14 registry dependencies include mandatory ring 0.17, so the existing ring admission hold remains. Release-era source exposes explicit `update_with_start_time` and `fetch_target_with_start_time`; this is a useful difference from tough. However, its constructor's local-metadata loading uses `Utc::now()`, and the convenience update/fetch methods also sample wall time. A supplied-time method is not proof that the entire lifecycle uses one trusted clock. Storage is supplied through repository interfaces, with memory and filesystem options; persisted latest-root and corruption behavior still need complete trace and tests. The inspected `with_trusted_local` starts from numbered root 1 and has an upstream FIXME concerning latest-root bootstrap, so its name must not be treated as proof of the desired restart contract. [D15, D16]

The `libdd-tuf` fork exposes similar supplied-time APIs but also has mandatory ring 0.17 in its registry entry. It is not an established licensing escape, and its exact source/package relationship remains unresolved here. The `tuf-no-std` search lead belongs to a research repository that describes a DER-format embedded implementation; it is not established as a ready substitute for the JSON repository/client ecosystem in the proposal. No dependency is selected from these leads. [D17, D18]

### Publisher-side tooling

`tuftool` 0.17.0 provides repository generation/signing and declares `MIT OR Apache-2.0`. Its manifest includes AWS-LC, AWS SDK KMS/SSM, Rustls/reqwest, Tokio and tough with HTTP. This is a substantial publisher tool graph, not a minimal client library. Using it in CI does not by itself add those packages to the Windows program; shipping it would require a separate distribution decision. [D14]

TUF-on-CI 0.21.0's repository and signer manifests declare MIT. Repo tooling requires Python >=3.10 and signer >=3.9, plus Python TUF, securesystemslib and chosen integration extras. It is intended to run in CI/signing environments. These are build/release-tool requirements, not a reason to add Python or a CI SDK to the product. [D19, D20]

Its online timestamp/snapshot choices include Google Cloud KMS, Azure Key Vault, AWS KMS and **experimental Sigstore**, expressly not supported by every client. Version 0.21 also adds ML-DSA-related signing support. Tough's inspected metadata key parser/verification supports Ed25519, RSA-PSS/SHA-256 and P-256 ECDSA; it does not expose Sigstore or ML-DSA key variants. Selecting a publisher by the shared label “TUF” is insufficient: exact key types, schemes, key IDs, canonical serialization, delegation patterns, rotations and threshold policy need round-trip compatibility fixtures with the eventual client. [D8, D19, D21]

## 7. Security baseline and unresolved admission gates

AWS's April 2026 bulletin fixes CVE-2026-6966, -6967 and -6968 in tough 0.22.0 and tuftool 0.15.0: delegated threshold uniqueness, delegated metadata validation, and path handling were affected. The observed 0.24.0/0.17.0 releases are above those vendor fix floors. This does not establish a clean advisory status for the complete unresolved graph, validate every delegated behavior, or justify an old release to avoid a provider constraint. The 0.24.0 changelog also fixes metadata reserialization after the jiff migration, relevant to retained signed state. [D10, D22]

The remaining gates are concrete:

- Exact distributed/client and publisher graphs, compatible selected grants, native obligations, package checksums/source parity, build scripts and advisory assessment; preserve OPEN-1/CHECK-26.
- Product-owned verification-time contract with legitimate backward correction, bad future clock, restart, expired metadata and authenticated recovery cases; preserve OPEN-8 rather than treating a wall-clock floor as trusted time.
- Protected bootstrap/latest-root loading and durable state transitions with corruption, interruption, concurrent access and MSI lifecycle recovery.
- Complete target verification and exact signed size, cache/resume handling, path/delegation policy and verified-byte installer handoff.
- Windows 11 target/toolchain/service-context qualification and bounded HTTP/TLS/native verification behavior.
- Chosen publisher/client algorithm and metadata interoperability, root-chain retention, expiry renewal, independent signer custody and compromise recovery.

**Permitted next step:** retain the proposal as design input and narrow a read-only/upstream feasibility comparison around compatible metadata cryptography, explicit time injection and durable storage. No production admission, dependency installation or architecture change follows from this assessment.

## Primary source registry

- **D1:** [Published tough 0.24.0 normalized manifest](https://docs.rs/crate/tough/latest/source/Cargo.toml), fetched while latest resolved to 0.24.0; [registry entry](https://github.com/rust-lang/crates.io-index/blob/master/to/ug/tough). Snapshot identifiers recorded above.
- **D2:** [Published tough VCS metadata](https://docs.rs/crate/tough/latest/source/.cargo_vcs_info.json).
- **D3:** [Published tough MIT license](https://docs.rs/crate/tough/latest/source/LICENSE-MIT).
- **D4:** [RepositoryLoader 0.24.0 API](https://docs.rs/tough/0.24.0/tough/struct.RepositoryLoader.html).
- **D5:** [Pinned tough lib.rs](https://github.com/awslabs/tough/blob/98d8eb8b2ce63515d9b4981c938ef6453c5b5771/tough/src/lib.rs), Git blob `e741cd1257978ed6a494ac7b8d3ab171a790a21b`; loader 176–248, load 332–420, reads/saves 448–632, supplied root 690–708, root persistence 808–832.
- **D6:** [Pinned datastore.rs](https://github.com/awslabs/tough/blob/98d8eb8b2ce63515d9b4981c938ef6453c5b5771/tough/src/datastore.rs), Git blob `de166d301d5536cecdb37565c93cb881eca98803`; lifecycle 15–33, create 59–70, system time 86–118.
- **D7:** [HTTP transport](https://github.com/awslabs/tough/blob/98d8eb8b2ce63515d9b4981c938ef6453c5b5771/tough/src/http.rs) and [transport routing](https://github.com/awslabs/tough/blob/98d8eb8b2ce63515d9b4981c938ef6453c5b5771/tough/src/transport.rs).
- **D8:** [Metadata key types and direct AWS-LC verification](https://github.com/awslabs/tough/blob/98d8eb8b2ce63515d9b4981c938ef6453c5b5771/tough/src/schema/key.rs).
- **D9:** [Digest/max-size stream adapters](https://github.com/awslabs/tough/blob/98d8eb8b2ce63515d9b4981c938ef6453c5b5771/tough/src/io.rs) and [fetch composition](https://github.com/awslabs/tough/blob/98d8eb8b2ce63515d9b4981c938ef6453c5b5771/tough/src/fetch.rs).
- **D10:** [Pinned CI](https://github.com/awslabs/tough/blob/98d8eb8b2ce63515d9b4981c938ef6453c5b5771/.github/workflows/rust.yml), [changelog](https://github.com/awslabs/tough/blob/98d8eb8b2ce63515d9b4981c938ef6453c5b5771/tough/CHANGELOG.md), [0.24.0 release](https://github.com/awslabs/tough/releases/tag/tough-v0.24.0).
- **D11:** [AWS-LC wrapper manifest](https://github.com/aws/aws-lc-rs/blob/v1.18.2/aws-lc-rs/Cargo.toml), Git blob `5514e9bc65a320258fc0a77306038851440e21f3`; [native manifest](https://github.com/aws/aws-lc-rs/blob/v1.18.2/aws-lc-sys/Cargo.toml), Git blob `a558fde46866bfa4f3535c17f72856094c676817`.
- **D12:** [ring 0.17.14 associated manifest](https://github.com/briansmith/ring/blob/2723abbca9e83347d82b056d5b239c6604f786df/Cargo.toml), Git blob `8f8da17dad7dc081290c6b9fee8c90b214cf4148`.
- **D13:** [Apache Software Foundation's GPL compatibility explanation](https://www.apache.org/licenses/GPL-compatibility.html). GNU FAQ retrieval timed out; no unread FAQ was used to clear process separation.
- **D14:** [Tuftool source manifest](https://github.com/awslabs/tough/blob/98d8eb8b2ce63515d9b4981c938ef6453c5b5771/tuftool/Cargo.toml), [registry entry](https://github.com/rust-lang/crates.io-index/blob/master/tu/ft/tuftool), [release](https://github.com/awslabs/tough/releases/tag/tuftool-v0.17.0).
- **D15:** [Rust-tuf registry entry](https://github.com/rust-lang/crates.io-index/blob/master/3/t/tuf); [release-era beta14 manifest](https://github.com/theupdateframework/rust-tuf/blob/37b51666633b100a51eb1e6eaf700314e17ccfb4/tuf/Cargo.toml); [project README/beta warning](https://github.com/theupdateframework/rust-tuf/blob/develop/README.md). Published archive parity remains unknown.
- **D16:** [Release-era beta14 client lifecycle](https://github.com/theupdateframework/rust-tuf/blob/37b51666633b100a51eb1e6eaf700314e17ccfb4/tuf/src/client.rs), Git blob `fcac1fb4f37468c944a5441e06984706640c81ac`; [separate current develop](https://github.com/theupdateframework/rust-tuf/tree/219ca7d05818d5dd44ec4e32ab47c5ce64a7bf44).
- **D17:** [libdd-tuf registry entry](https://github.com/rust-lang/crates.io-index/blob/master/li/bd/libdd-tuf) and [0.3.1 API](https://docs.rs/libdd-tuf/latest/tuf/client/struct.Client.html); no source commit asserted.
- **D18:** [Fraunhofer bt2x research repository](https://github.com/Fraunhofer-AISEC/bt2x), describing DER-based no-std implementation. No Windows/repository interoperability claim derived.
- **D19:** [TUF-on-CI 0.21.0 release](https://github.com/theupdateframework/tuf-on-ci/releases/tag/v0.21.0); [repo manifest](https://github.com/theupdateframework/tuf-on-ci/blob/662382a8295b89a9d538390ce8b9bb85b207a93d/repo/pyproject.toml).
- **D20:** [Signer manifest](https://github.com/theupdateframework/tuf-on-ci/blob/662382a8295b89a9d538390ce8b9bb85b207a93d/signer/pyproject.toml) and [repo MIT license](https://github.com/theupdateframework/tuf-on-ci/blob/662382a8295b89a9d538390ce8b9bb85b207a93d/repo/LICENSE).
- **D21:** [Pinned online signing setup](https://github.com/theupdateframework/tuf-on-ci/blob/662382a8295b89a9d538390ce8b9bb85b207a93d/docs/ONLINE-SIGNING-SETUP.md).
- **D22:** [AWS bulletin 2026-019](https://aws.amazon.com/security/security-bulletins/2026-019-aws/), [delegated threshold advisory](https://github.com/awslabs/tough/security/advisories/GHSA-8m7c-8m39-rv4x), [delegated validation advisory](https://github.com/awslabs/tough/security/advisories/GHSA-4v58-8p28-2rq3), [path traversal advisory](https://github.com/awslabs/tough/security/advisories/GHSA-v57p-gppj-p9vg).
