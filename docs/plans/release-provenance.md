# Optional enhanced build and Store provenance

**Date:** 4 November 2026  
**Status:** accepted optional direction; research and possible experiment only  
**Authority:** [OD-05 §7.5](../nevrotick-owner-decisions.md#75-optional-enhanced-build-provenance--4-november-2026)
and [release policy §3.3](../release-policy.md#33-optional-enhanced-provenance).

## 1. Owner decision and existing requirements

David asked to retain the research on connecting a release to its repository,
without making the additional assurance a mandatory release blocker. GitHub/
Sigstore attestations, independent reproducible application builds and a public
source-to-Store evidence bundle are **optional**. Their absence, an unperformed
experiment or a failure to reproduce a build does not by itself block P6/P7,
the Store attempt, release approval or current Core work. This record introduces
no CHECK, mandatory qualification row or hidden prerequisite.

Existing source/build/artifact custody, corresponding-source and licence duties,
required SBOM/payload reconciliation, package signatures, publisher/product
identity, tamper checks and installed/native/lifecycle qualification retain their
required scope. Their owners remain the [release policy](../release-policy.md),
[signing guide](../windows-time-signing-guide.md), [architecture §19.4](../windows-time-architecture.md#194-future-complete-msix-gate)
and [MSIX plan §7](windows-msix-store.md#7-bounded-continuation-and-existing-acceptance-owners).
Reproducible generation of admitted public leap/calendar and catalogue data is a separate contract
from an independent bit-for-bit rebuild of the complete application.

An optional investigation can discover a real failure of an existing required
integrity or security property. That finding follows its existing owner and gate;
optional status does not turn observed tampering into an acceptable release.
Conversely, missing enhanced evidence is not evidence of tampering. Only a later
explicit owner amendment can make this additional assurance mandatory.

## 2. Current evidence and claim boundaries

The inspected repository snapshot is `d066d59c546fbe4877634859e9a3dc654cc7ef27`.
Its Rust and security workflows [NT-1, NT-2] establish no implemented
release-attestation or Store-submission pipeline. This document adds none.
No complete independently reproduced NevroTick release, public proof bundle,
Store acceptance or source-to-delivered-byte comparison is claimed.

| Evidence | What it can support | What it does not establish |
| --- | --- | --- |
| Source revision and reviewed build inputs | Identity and custody of the intended source/configuration | That a particular binary was actually compiled from them |
| Package signature and package/catalog integrity | Trust and content integrity under the admitted Windows package policy | GitHub source origin, exact helper authorization or benign behavior |
| Verified artifact attestation | A particular digest is bound to an authenticated workflow/repository claim | Independent observation of compilation or an uncompromised workflow |
| Independently matching rebuild | Matching bytes from the recorded inputs and independent build attempt | Safe source, safe dependencies or absence of shared toolchain compromise |
| Delivered-payload comparison | Equality or identified differences between the measured file sets | Unmeasured versions/devices, loaded-image behavior or lifecycle correctness |
| SBOM and licence records | Recorded component/rights inventory and lineage within their scope | Completeness without reconciliation, or freedom from malicious components |

GitHub explicitly describes attestations as provenance rather than a guarantee
that an artifact is secure. Sigstore authenticates a signing identity under its
trust assumptions; a compromised account or identity provider can still sign.
Neither Store certification, a valid signature, a clean scan nor matching
rebuilds proves absence of malicious behavior. [GH-1, SG-1]

The threat path of interest is source/build/publishing substitution: an attacker
changes an input, build action, runner output or distributed artifact while the
recipient assumes it belongs to an approved release. Digest verification can
expose substitution after attestation. It cannot repair an already compromised
source review, build execution or release authority. Existing installed-product
and helper/IPC authorization remain separate from a broad Store signer class.

## 3. Optional GitHub provenance approach

If later selected for implementation, attest the exact candidate outputs and a
detached payload inventory through a reviewed release workflow. A useful record
would retain the source commit, workflow revision/run, target/features, lockfile,
toolchain/SDK/linker identities and actual artifact digests alongside the existing
custody record. These fields describe evidence; they are not new release gates.

Current upstream guidance recommends `actions/attest` for new integrations;
`attest-build-provenance` v4 is a wrapper. The inspected v4.2.2 release, published
4 August 2026, resolves to `1e69f48acb82d1966a394da916b4c1698aa569d6`.
It uses Node24. This dated source reference is not an adopted dependency pin.
Official binary examples use `contents: read`, `id-token: write` and
`attestations: write`; no workflow permission is changed here. [GH-2, GH-3]

Windows is not excluded by the evidence reviewed: upstream v4.2.0 corrected
Windows checksum-file parsing. Explicit artifact paths avoid that particular
caller-supplied checksum format. No NevroTick Windows attestation run was
performed, and a hosted Windows build would not itself qualify the required
Windows 11 installed behavior. [GH-4]

An optional verifier could admit the expected repository, signer workflow and
workflow revision, source commit/ref and runner category, while hashing the
actual artifact. GitHub CLI exposes those policy controls. Expected identities
need an admitted release source rather than values copied unquestioningly from
the download being checked. Detached/offline verification also benefits from
checking the stable numeric repository ID against an independently retained
identity; names can be reused after renames. [GH-5, GH-6]

The important limit is the trusted workflow: GitHub CLI distinguishes certificate
fields and verified timestamps from predicate data that workflow execution can
falsify. Authenticated source-context metadata is not an independent witness
that Cargo compiled the intended checkout. Reviewed checkout/build instructions,
immutable action revisions, constrained inputs, token authority and cache/artifact
custody therefore affect the strength of any claim. A trusted reusable workflow
can improve isolation; merely adding an action or moving YAML does not establish
NevroTick's SLSA Level 3 conformance. [GH-5, GH-7, SL-1]

Public repositories can use GitHub attestations on current plans; private/internal
repository use requires Enterprise Cloud. Public attestations use Sigstore's
public transparency infrastructure; GitHub's private instance has no public
transparency log. Account/plan eligibility was not established for this work,
and lack of eligibility does not create a release blocker. [GH-1, GH-2]

## 4. Store processing and an optional detached inventory

Microsoft re-signs accepted MSIX/AppX packages, replacing an existing package
signature. The signature forms part of the package bytes, so equality between
the submitted and delivered MSIX hashes is not a valid general expectation.
Store package trust remains useful, but it does not carry a GitHub source/build
attestation automatically. [MS-1, MS-2]

The inspected MSIX submission/certification documentation establishes no Store
procedure that enforces SLSA/Sigstore proof of source-to-binary correspondence.
This is a bounded documentation finding, not proof about every Microsoft program.
`Notes for certification` can link evidence for a human reviewer but are not
public, and do not require Microsoft to perform the proposed verification. An
app website can link a public evidence page; such publication remains optional.
[MS-3, MS-4, MS-5]

For a stronger comparison, an optional detached inventory could name every
package entry by canonical package-relative path, size and SHA-256,
including EXEs, DLLs, resources, configuration and `AppxManifest.xml`. It could
also identify the submitted package/version, source revision and existing SBOM.
Attesting that inventory authenticates its exact bytes. An external verifier
would then recompute each listed file's digest and compare the full measured
path set, including added, removed or duplicated entries. Verifying the inventory
alone does not verify the package contents.

The MSIX payload/footprint distinction is not permission to exclude all metadata.
The manifest controls identity, dependencies, capabilities and extensions.
Preserve the measured manifest, block map, signature and any catalogue/metadata
differences by exact path; verify the signature separately. Do not blanket-ignore
`AppxMetadata/*`, remove PE regions or accept unexplained added/deleted files as
equivalent. Failure of this optional comparison withholds its stronger claim;
existing mandatory integrity/identity failures retain their own gates. [MS-6, MS-7]

Keep three identities distinct: the submitted archive, its unpacked payload and
the actual Store-delivered/installed payload. The final package hash is recorded
separately from payload hashes. Embedding an attestation of the package's own
final bytes creates a circular binding; detached evidence avoids that problem.
Build metadata embedded before packaging can identify a revision, but an
application's own About screen or self-check is not independent verification.

No Microsoft guarantee that every inner file remains byte-identical was found.
Actual delivered-byte measurement is pending. The optional comparison would
report exact matches and differences, with any explained Store transformation
kept explicit. Do not assume that every difference is malicious or harmless,
or silently normalize executable content/signature regions to manufacture a
match. If equivalence cannot be established, omit the stronger claim and retain
the observation. Existing required delivered integrity, identity, loaded-image
and lifecycle checks still apply under their current owners.

## 5. Independent rebuilding and public evidence

An optional independent rebuild would start from the admitted source revision
and recorded Rust/native inputs in another clean environment. Comparison should
distinguish unsigned payloads from later signing/package transformations and
document any sources of nondeterminism. Matching only after unspecified byte
exclusions is not a demonstrated bit-for-bit rebuild. A mismatch can reflect
timestamps, paths, toolchain inputs or a defect; its cause remains unresolved
until examined. The mismatch alone is not a new release blocker.

Record the complete input closure: dependency contents, compiler/Cargo/LLVM,
target/features/profiles, linker/native compiler, Windows SDK/CRT, resources,
packaging tools, generated data, flags/environment and any admitted PGO profile.
The second builder should obtain sources independently and consume no shared
writable compiled outputs. Shared compiler or workflow compromise can survive
two matching builds. These are conditions on the optional reproducibility claim,
not requirements newly imposed on ordinary release. [RB-1, SL-2]

Cargo `--frozen` requires locked resolution and puts Cargo offline; it is not an OS
sandbox for build scripts or procedural macros. Rust path remapping is best
effort and does not normalize linker-generated PDB paths/arguments. Preserve
NevroTick's mitigation flags, `debug="limited"`, `strip="none"` and matching
symbols; do not obtain a match by silently dropping hardening or executable
regions. Declare and investigate any PDB/package nondeterminism separately.
[RS-1, RS-2, RS-3, RS-4; development-hardening §6]

If a public evidence bundle is later useful, it could contain the release/source
identities, detached inventory, attestation bundles, verification policy,
rebuild/comparison observations and links to corresponding source and notices.
It supplements the existing custody/SBOM/licence records; publication of this
additional bundle is optional. Publish only the evidence actually obtained and
review it for secrets, private paths and unnecessary personal data. A public
bundle is useful only when its claims are checked against the bytes received.

## 6. Bounded possible experiment and continuation

No experiment is scheduled or executed by this record. If later chosen, one
bounded trial could use a single complete candidate for `x86_64-pc-windows-msvc`
and the existing W11-W10/release evidence owner:

- Freeze one source/build tuple and compare one original payload with one clean
  independent rebuild; retain the exact comparison scope and unresolved causes.
- Generate and externally verify an optional detached inventory/attestation;
  compare an unchanged artifact with a deliberately altered copy and a wrong
  expected workflow identity to check that the verifier discriminates them.
- When a real Store-delivered candidate exists through the authorized submission
  path, compare its measured payload with the recorded submitted payload and
  retain the signature/packaging differences. A local test signature supplies
  no Store-delivered observation.
- Close the bounded trial as demonstrated, inconclusive or unperformed for each
  optional claim. Continue ordinary phase work without demanding a successful
  experiment; route discovered required-property failures to their existing owner.

Use [development §7](development-plan.md#7-next-executable-task-for-a-development-agent)
for current priority and [W11 task §4.23](../../tasks/windows11-integration.md#423-optional-build-provenance-research--4-november-2026)
for this research's continuation. Existing Windows execution, fuzz/storage and
UI-M0 dispositions remain. This document authorizes no build, installation,
account change, signing/submission, Store publication, workflow edit or new skill.

## 7. Primary references

External documentation was checked on 4 November 2026; recheck changing service
support and action releases when an optional implementation is actually selected.

- **NT-1:** [NevroTick Rust workflow at the inspected revision](https://github.com/DavidOsipov/NevroTick/blob/d066d59c546fbe4877634859e9a3dc654cc7ef27/.github/workflows/rust.yml).
- **NT-2:** [NevroTick security workflow at the inspected revision](https://github.com/DavidOsipov/NevroTick/blob/d066d59c546fbe4877634859e9a3dc654cc7ef27/.github/workflows/security.yml).
- **GH-1:** [GitHub artifact attestations: guarantees, SLSA and public/private infrastructure](https://docs.github.com/en/actions/concepts/security/artifact-attestations).
- **GH-2:** [GitHub: generating attestations and plan availability](https://docs.github.com/en/actions/how-tos/secure-your-work/use-artifact-attestations/use-artifact-attestations).
- **GH-3:** [Pinned actions/attest definition](https://github.com/actions/attest/blob/1e69f48acb82d1966a394da916b4c1698aa569d6/action.yml), [v4.2.2 release](https://github.com/actions/attest/releases/tag/v4.2.2) and [wrapper migration](https://github.com/actions/attest-build-provenance#usage).
- **GH-4:** [actions/attest v4.2.0: Windows checksum parsing correction](https://github.com/actions/attest/releases/tag/v4.2.0).
- **GH-5:** [GitHub CLI: attestation verification, identity policy and predicate limits](https://cli.github.com/manual/gh_attestation_verify).
- **GH-6:** [GitHub: offline verification and repository IDs](https://docs.github.com/en/actions/how-tos/secure-your-work/use-artifact-attestations/verify-attestations-offline).
- **GH-7:** [GitHub: reusable workflows and SLSA Build Level 3](https://docs.github.com/en/actions/how-tos/secure-your-work/use-artifact-attestations/increase-security-rating).
- **SG-1:** [Sigstore threat model](https://docs.sigstore.dev/about/threat-model/).
- **SL-1:** [SLSA v1.2 build requirements and isolation limits](https://slsa.dev/spec/v1.2/build-requirements).
- **MS-1:** [Microsoft: MSIX Store package requirements and replacement signing](https://learn.microsoft.com/en-us/windows/apps/publish/publish-your-app/msix/app-package-requirements).
- **MS-2:** [Microsoft: package troubleshooting and changed bytes after resigning](https://learn.microsoft.com/en-us/windows/win32/appxpkg/troubleshooting).
- **MS-3:** [Microsoft: MSIX submission options and private certification notes](https://learn.microsoft.com/en-us/windows/apps/publish/publish-your-app/msix/manage-submission-options).
- **MS-4:** [Microsoft: website and support information](https://learn.microsoft.com/en-us/windows/apps/publish/publish-your-app/msix/support-info).
- **MS-5:** [Microsoft: submission API schema](https://learn.microsoft.com/en-us/windows/uwp/monetize/manage-app-submissions).
- **MS-6:** [Microsoft: MSIX payload and footprint](https://learn.microsoft.com/en-us/windows/msix/overview).
- **MS-7:** [Microsoft: app package manifest](https://learn.microsoft.com/en-us/uwp/schemas/appxpackage/appx-package-manifest).
- **RB-1:** [Reproducible Builds: definition and declared artifact scope](https://reproducible-builds.org/docs/definition/).
- **SL-2:** [SLSA v1.2 FAQ: independent platforms and reproducibility limits](https://slsa.dev/spec/v1.2/faq).
- **RS-1:** [Cargo build: locked/offline/frozen](https://doc.rust-lang.org/cargo/commands/cargo-build.html).
- **RS-2:** [Cargo build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html).
- **RS-3:** [Rust procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html).
- **RS-4:** [Rust source-path remapping and PDB limitations](https://doc.rust-lang.org/rustc/remap-source-paths.html).
