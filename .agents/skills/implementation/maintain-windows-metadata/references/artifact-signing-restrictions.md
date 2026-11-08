# Artifact signing restrictions

Use for signing-configuration research or a later admitted SignPath path.
Source review: 8 November 2026. Keep [OD-05 and the signing guide](../../../../../docs/windows-time-signing-guide.md)
authoritative: complete MSIX/Store remains selected; SignPath/MSI is a retained
alternative. This reference selects no provider, certificate, account or new
release gate. Reuse MD-P4–MD-P5, W11-W10/P6 and CHECK-18/20/23/26/27; record
execution in the existing task. No signing, upload or Windows execution follows
from reading it.

## Contents

- [Supported checks](#supported-checks)
- [Selectors and release expectations](#selectors-and-release-expectations)
- [Signature operations](#signature-operations)
- [Discriminating cases](#discriminating-cases)

## Supported checks

Treat restrictions as checks on existing artifacts. Generate metadata through
the [current pipeline](../../../../../metadata/README.md) and retain independent
[readback](generation-and-verification.md); never repair a mismatch by copying
the observed wrong value into the expected record.

The [SignPath reference](https://docs.signpath.io/artifact-configuration/reference#file-metadata-restrictions)
lists these restriction attributes:

| Element | Attributes |
| --- | --- |
| `pe-file` | `product-name`, `product-version`, `file-version`, `company-name`, `copyright`, `original-filename` |
| `msi-file` | `subject`, `author` |
| `xml-file` | `root-element-name`, `root-element-namespace` |

An `include` restriction overrides its file-set counterpart. Review the
effective expectation for each selected file, including permitted exceptions.

Do not invent additional SignPath attributes from the much larger Windows field
catalogue. Preserve these independent obligations:

- Read PE fixed numeric versions and admitted localized string resources. The
  provider reference does not settle which representation or locale it compares,
  or promise wildcard matching inside metadata values. Qualify these details
  before relying on them.
- For conditional MSI, inspect Summary Information and the Property table
  separately. Confirm the provider's extraction; the two restrictions do not
  establish ProductCode, UpgradeCode, PackageCode or ProductVersion correctness.
- An XML root check is insufficient for schema, semantic or payload validation.
  The [CycloneDX example](https://docs.signpath.io/artifact-configuration/examples#metadata-restrictions)
  does not admit an XML SBOM lane or replace NevroTick's existing JSON inventory
  and its independent completeness checks.
- Apply the existing MSIX manifest/PRI and installed readback owners. Support for
  signing MSIX does not imply equivalent metadata restrictions on that element.

## Selectors and release expectations

The [syntax](https://docs.signpath.io/artifact-configuration/syntax) defines
case-insensitive paths; `*` stays within a path segment and `**` occupies a whole
segment for recursive matching. Wildcard `min-matches` and `max-matches` default
to one; zero minimum permits absence and `unbounded` removes the maximum.
Parameter names are case-sensitive. Declare them in the initial `parameters`
block and reference `${parameterName}`. `required` defaults to false; true
requires a supplied nonempty value. Recheck selected-service availability.

Apply these project-specific review rules:

1. Derive selectors from the approved shipping inventory, including component
   ownership and exact expected multiplicity. Review optional or recursive
   matches explicitly. Reject ambiguous aliases and case-colliding payloads in
   the package inventory; do not guess provider collision behavior.
2. Independently reconcile every actual package entry, including unselected
   content. A successful selected-file match cannot establish package completeness
   or absence of extra executable content; do not assume a deny-unlisted policy.
3. Bind expected values to the reviewed release candidate and existing
   per-surface version mappings before reading the artifact under test. An
   arbitrary request parameter, even when required, is not that authority.
   Avoid placeholder defaults for required release identity/version values.
4. Keep current `product.toml` schema ownership. Add no ad hoc signing fields or
   parallel version source; future request/configuration IDs belong with their
   admitted provider owner and candidate receipt.
5. Review the [example's `original-filename` binding](https://docs.signpath.io/artifact-configuration/examples#metadata-restrictions)
   before using `${file.name}`. Windows [OriginalFilename](https://learn.microsoft.com/en-us/windows/win32/menurc/stringfileinfo-block)
   preserves the original name and can expose renaming. NevroTick's current
   [field map](../../../../../docs/plans/windows-package-metadata.md) selects the
   actual shipped basename. Reconcile delivery naming with that contract before
   building; never rewrite emitted VERSIONINFO or its expected record merely to
   make a signing example pass.
6. Review sample-generated configurations against independently approved
   expectations. The [overview](https://docs.signpath.io/artifact-configuration/)
   warns that generated instructions can include unwanted third-party signing.
   Neither an uploaded sample nor its generated configuration supplies authority
   for its own contents.

## Signature operations

The [reference](https://docs.signpath.io/artifact-configuration/reference)
documents `description`/`description-url`, `append=false` replacement and append
support for PE/CAB/catalog. Run `authenticode-verify` before a modifying signing
directive when verifying an existing signature. Its Windows trust checks do not
document an expected-project/publisher pin.

They also do not document the effective `EnableCertPaddingCheck` policy or
embedded/catalog selection. Apply [signing guide §5.2](../../../../../docs/windows-time-signing-guide.md#52-pe-certificate-padding-and-exact-final-bytes)
for applicable provider evidence or independent strict Windows verification,
the harmless padding-negative case and approved full-file hash custody. Keep
metadata generation before signing; certificate padding is not an authenticated
metadata/configuration store. This PE qualification does not replace package
trust or require a different signing provider or RSA padding algorithm.

Reuse the existing separate description/URL fields; [SignTool](https://learn.microsoft.com/en-us/windows/win32/seccrypto/signtool)
exposes them as sign-command `/d` and `/du`. They are not PE FileDescription or
certificate identity. Preserve third-party provenance and approved signatures;
an example that adds or replaces a signature supplies no authorization to do so.
Never substitute CompanyName, a filename pattern or any generally trusted chain
for the approved project/release identity.

The [deep-signing example](https://docs.signpath.io/artifact-configuration/examples#deep-signing-an-msi-installer-with-nested-executables)
signs inner PE files, rebuilds the MSI and signs its enclosing bytes. Reuse the
signing guide's artifact custody and independent final-output inspection. Keep
submitted, returned and Store-delivered hashes distinct. Do not transfer MSI
mechanics or append support to a different format without qualification.

For ordinary app-package signing without publisher bridging,
[Microsoft's certificate rules](https://learn.microsoft.com/en-us/windows/msix/package/create-certificate-package-signing)
require the signing certificate subject to match manifest Identity Publisher;
the [signing digest must match AppxBlockMap](https://learn.microsoft.com/en-us/windows/msix/package/sign-app-package-using-signtool).
Preserve the selected package
identity; do not change it to fit an available certificate. Inspect the actual
package at its proper stage; provider support does not qualify Store-delivered
identity, extracted PE trust or privileged-helper admission.
Treat any future [publisher-bridging migration](https://learn.microsoft.com/en-us/windows/msix/package/persistent-identity)
as a separately reviewed identity/update path; this reference does not admit it.

## Discriminating cases

When that route is selected and execution is admitted, pair each relevant faulty
case with the intended healthy candidate. Keep actual provider observations
separate from offline configuration review and native artifact verification.

| Change to the candidate or request | Required observation |
| --- | --- |
| Wrong/missing product or version value | Rejection attributable to the changed restriction; healthy values accepted |
| Missing required binary, extra match or unselected extra executable | Selector/cardinality or independent inventory rejection; exact intended inventory accepted |
| Weaker per-file override, case collision or wrong parameter case | Effective configuration/request review rejects unintended scope or expectation |
| Missing/empty required version, or a supplied version inconsistent with the approved release | Missing-input or independent release-binding rejection; no sample-default rescue |
| Divergent numeric/string/localized versions; renamed delivery file | Observe provider extraction and compare all required native fields; preserve authorized healthy mappings |
| Trusted signature from a different project, modified signed payload, or unintended upstream-signature replacement | Independent identity, integrity or provenance rejection; expected final artifact accepted |

Record source/configuration revision, parameter provenance, resolved file set,
artifact hashes, effective restrictions, reader/provider versions and observed
outcomes with the existing receipt. Unresolved extraction or matching semantics
remain a gap in that provider configuration, not permission to claim a pass or
an additional blocker for the selected Store route.
