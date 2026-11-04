# Offline server-catalogue candidates (R contract 1)

This standard-library Python utility prepares **unapproved publisher evidence**
from explicit local snapshots. It collects the three formats below and compares
two hash-bound candidate snapshots. It performs no network/DNS/protocol requests,
upstream code execution, approval, signing, production C emission or client
integration. Python and these JSON artifacts are not MSIX runtime dependencies.
JSON here is an R evidence representation; it selects no shipped C representation.

The existing catalogue task's R / DATA-01 / DATA-24 work owns this bounded slice.
Production IDs, codecs, readers, source/default policy, permission, independence,
root transitions, clocks and runtime profile qualification remain separate.

## Commands and input contract

Use an explicitly selected Python 3.12 interpreter. Examples omit host-specific
absolute interpreter paths:

```sh
python3.12 -I -B -S build_catalogue.py collect --manifest inputs.json --output candidates.json
python3.12 -I -B -S build_catalogue.py diff --before old.json --before-sha256 HEX --after new.json --after-sha256 HEX --output comparison.json
```

Both commands require a fresh output filename in an existing, owned directory.
There is no overwrite, fetch, probe, approval or build switch. Manifest and source
files must be regular files. Source `file` values are basenames in the manifest's
directory, not URLs, absolute paths or traversals. The directory must be controlled
by the caller; this is not a hostile-filesystem sandbox. Final-component symlinks
are refused on the supported POSIX host route. No source body/path is printed.

The manifest is duplicate-aware strict UTF-8 JSON with exactly these fields:

```json
{
  "schema": "nevrotick-r-input-v1",
  "sources": [{
    "id": "example-nts",
    "adapter": "jauderho-nts-markdown-v1",
    "file": "saved.md",
    "sha256": "64 lowercase hexadecimal digits",
    "bytes": 1234,
    "origin": {
      "uri": "https://example.invalid/source",
      "revision": null,
      "git_blob": null,
      "license_uri": null,
      "related": []
    }
  }]
}
```

Every shown field is required. `id` is a unique, caller-assigned lowercase ASCII
slug (1–64 characters, letters/digits/hyphens, starting with a letter).
`sha256` and `bytes` bind the exact source bytes before decoding. `revision` and
`git_blob` are null or lowercase 40-digit Git identities. A supplied Git blob is
also checked against the actual bytes; a revision/URI is retained attribution,
not a verified repository ancestry or signature. `uri` is an inert HTTP(S) URL;
`license_uri` is such a URL or null. `related` contains at most eight closed
`{kind, uri, git_blob}` objects; kind is `canonical-input`, `generator` or
`use-document`, and the blob is nullable. Those references are not fetched or
treated as independently verified content. They can record the upstream YAML,
converter and use-document provenance without executing or parsing those files.

Unknown members, duplicate object members, floats/exponent numbers, non-finite
numbers, malformed/trailing JSON and wrong exact types are refused. In particular,
booleans are not integers. There is no implicit schema migration. All source
snapshots are explicit inputs; the tool cannot discover that an operator's prose
changed elsewhere. Repository redistribution terms are not service-use permission.

## Three closed adapters

| Adapter | Accepted source shape |
| --- | --- |
| `jauderho-nts-markdown-v1` | Exactly one `## The List` and one subsequent `## Star History`; five cells `Hostname`, `Stratum`, `Location`, `Owner`, `Notes`. |
| `jauderho-public-ntp-markdown-v1` | Same framing; six cells `Hostname`, `AS`, `Stratum`, `Location`, `Owner`, `Notes`. |
| `tannerryan-ecosystem-json-v1` | Root object containing only a nonempty `servers` array. Each row has exactly `name`, `version`, `publicKeyType`, `publicKey`, `addresses`; each address exactly `protocol`, `address`. |

Markdown permits blank lines, the matching column separator line (at least three
dashes per cell, optional alignment colons), literal `||` between groups, and an
optional second table introduced by exactly:

> The following servers are known to be virtualized and may be less accurate. YMMV.

Each present table must have a header, separator and at least one data row. A
separator cannot lead, repeat or terminate a table. No other content is accepted
inside the list section. Missing end marker, extra tables/columns, changed headers
or partial sections refuse the whole source. Surrounding document text is bounded
and retained by source digest but not interpreted. A complete, valid source can
still deliberately omit rows; this utility cannot prove upstream completeness
beyond the declared grammar. It reports such differences for human review.

Cells preserve source text after trimming outer ASCII spaces. Raw owner,
location, notes, AS and stratum survive, including `Distro`, `All`, `Unknown`,
comma-separated AS claims and use restrictions. The first table is **unmarked**
for virtualization, not proof of physical hardware. The second carries only a
source-claimed virtualized classification. A hostname cell is a bare endpoint or
one whole `[endpoint](http-or-https-URL)` link; its **label** is the endpoint and
its target remains inert information. Other cell markup is retained as data.
An `NTS` note in the NTP list does not change its family.

Roughtime preserves the original record and address order. `protocol` is exactly
`udp` or `tcp`. Ed25519 rows accept an integer version 1–65535 or the exact string
`IETF-Roughtime`. Numeric 1 remains a maximum-version advertisement; other numeric
versions are unsupported profile evidence. The generic IETF label is unresolved.
No declared profile becomes a supported/approved profile. Quoted `"1"`, booleans,
floats and other strings are refused; no operator compatibility adapter is included.

Ed25519 public material must be canonical padded standard Base64 and exactly
32 decoded bytes. The known experimental pair `publicKeyType: "ml-dsa-44"`,
`version: "ml-dsa-44"` remains an unsupported record with canonical Base64 material
(1–2048 decoded bytes); only encoding/size is checked for that row. Other algorithm
or label combinations explicitly refuse the source. There is no cryptographic
verification, algorithm adoption or key-transition inference. Every address stays
inside its original key/profile record; records are never merged into a Cartesian
product. Ancillary `sources`/`reports` fields are not in this adapter version.

Pinned documentary shapes used to design this contract:

| Source | Commit | Input Git blob |
| --- | --- | --- |
| `jauderho/nts-servers/README.md` | `247a47ee6d0b4da95b19257dc70027378f6f8c76` | `dda2668919380ed7fe5f87c0a2c8d8a0545d20f2` |
| `jauderho/public-ntp-servers/README.md` | `fa07c5440f4c2301697e978b2be36b743d3dd8f2` | `63e8a490a55a8c12dc3496f02ca6bec6dc2df372` |
| `tannerryan/roughtime/ecosystem.json` | `09ffa30abfb8c9ea22b81858edf393f6c5a7ddef` | `cd9eb5043fd7ff77845333ffe97b8318303ff94e` |

Later snapshots with the same complete shape may be captured explicitly; these
pins describe the reviewed grammar, not an automatic trust or update mechanism.

## Normalization and unknowns

All endpoint tokens use ASCII syntax. DNS comparison is lowercase with exactly
one terminal root dot; each label has 1–63 letters/digits/hyphens, no edge hyphen,
and at least two labels. The undotted name is at most 253 characters. Unicode,
userinfo, URL/path syntax, whitespace and zone identifiers are refused. ASCII
punycode-looking labels receive no IDNA qualification. Invalid numeric IP-like
forms cannot fall back to DNS. IPv4 uses the standard library's strict dotted
decimal parser; IPv6 must be bracketed in source tokens, with a valid literal,
and is emitted compressed/lowercase without brackets. Unbracketed IPv6, broken
brackets and ambiguous host/port spellings are refused.

Ports are canonical decimal 0–65535 with no leading zeros. Roughtime requires an
explicit port. A table endpoint may omit it: NTS then records TCP 4460 with
`adapter-default:rfc8915-section4`; NTP records UDP 123 with
`adapter-default:ntp-udp123`. The original explicit port is separately null or
integer. Zero is retained as an unusable endpoint claim, never defaulted. These
are parsing derivations, not admitted destinations; no negotiated NTP endpoint
is invented from an NTS-KE port.

Names at/below `pool.ntp.org` are marked `dynamic-pool-alias`; other names are
`not-classified`. No alias is expanded or assigned to its nominal owner/AS set.
In every record structured geography, service-use permission and independence
are `unknown`, and time-scale/leap/smear behavior is `unqualified`. Empty notes
cannot create ordinary UTC, no-smear or unrestricted-use evidence. This version
has no operator-evidence overlay or lifecycle approval input.

## Candidate, failure and comparison contract

The single output schema is `nevrotick-r-candidate-v1`. Its closed root contains
`schema`, `authority`, `producer_sha256`, `manifest_sha256`, `status`, `sources`,
`record_count`, and `review_groups`. Authority is always
`unapproved-publisher-evidence`; status is `complete-candidate` only when every
source is complete and nonempty, otherwise `incomplete-candidate`. Producer hash
binds the actual utility source. Neither a hash nor a status authenticates data.

Each source contains `id`, `adapter`, `artifact` (expected digest/length/origin),
`observed` (actual digest/length or null), `status`, `error`, and `records`.
Only `complete` or `failed` source status exists. A failed source has a static
error code and no records; useful neighboring sources remain in the incomplete
snapshot. Unknown manifest schemas, invalid manifest metadata or aggregate limits
refuse the command before output. Source read/hash/shape/record failures are
accounted as failed sources. There is no apparently successful empty source.

Each record contains `claim_id`, `locator`, `raw`, and `meaning`. Locator is the
one-based Markdown line plus table class, or `/servers/N` JSON pointer. Claim ID
is SHA-256 of canonical `{source_id, artifact_sha256, locator, raw}`. It identifies
one raw claim in one captured artifact, **not** a future permanent product source,
endpoint or key. `meaning` is recomputed by one shared normalization path and
contains family, exact scoped endpoint list, information URI, table class, raw
metadata claims, explicit unknowns, declared profile/key material and disposition.
No authority-bearing extension fields are accepted.

`review_groups` derives endpoint overlaps from every retained claim. Equal
meanings are `equivalent-claims`; disagreements are `differing-claims`. All claim
IDs and original bindings survive. These groups neither resolve a disagreement
nor establish independent votes. Sources are sorted by source ID, records by
locator, and review groups by normalized endpoint. Those orderings grant no
priority. Address order within a Roughtime row remains source order.

`diff` requires canonical emitted bytes and the exact SHA-256 of both files, parses them with the same bounds,
and **revalidates all closed shapes, raw claims, derived fields, counts and groups**.
It rebuilds normalized records instead of trusting generated-file origin. It
produces `nevrotick-r-diff-v1`, review-only authority, exact before/after digest and
length, producer-change status and per-source comparisons. It never writes a
candidate or updates its baseline.

Within a matching, complete, same-adapter source, semantic joins use family plus
normalized table host, or the exact Roughtime display-name claim. These are review
anchors, not proven permanent identity: a rename/host change appears as absent
and added groups. Multiple claims under one anchor remain a multiset. A changed
key, declared profile, endpoint/transport/port, metadata or multiplicity is
reported with before/after meanings and affected fields. Original ordering/row
locators alone do not create semantic additions/removals; source digest/origin and
raw evidence differences remain visible as provenance changes. A missing/failed
source or changed adapter is `blocked` for row comparison, never mass retirement.
`absent` means absent documentary claims, not withdrawal, retirement or revocation.

Normal-return exit codes: **0** complete candidate or completed comparison; **3** published
incomplete candidate or comparison containing blocked sources; **2** invocation,
manifest, snapshot or output failure with no new completed output. Diagnostics
are fixed codes/counts, never arbitrary raw bodies, paths, HTML or Markdown. JSON
strings use ASCII escapes and additionally escape HTML-sensitive characters;
consumers must continue treating strings as data, not render them as markup.
Diagnostic writes/flushes are best effort: a caught stream error after publication
does not change its committed 0/3 result. Abnormal interpreter shutdown (including
another stream flush) or OS termination can obscure process acknowledgement;
diagnostic delivery and universal process-exit behavior are not guaranteed.

Identical manifest/source bytes and utility source produce identical candidate
bytes. Payloads contain no wall-clock timestamps or absolute local paths.
Diffs are likewise deterministic for their exact two inputs and this contract.

## Finite engineering caps

These are R-tool workload limits, not production C bounds or measured product
resource results. Counts are checked before growing the relevant collection;
source sizes and the manifest aggregate are checked before reading sources.
JSON lexical limits precede `json.loads` and duplicate members are rejected before
dictionary normalization. Encoded string tokens allow at most twelve characters
per decoded character because a non-BMP character can use two surrogate escapes;
the decoded 4,096-character cap remains mandatory. Generated candidate and diff
bytes must pass the same strict JSON limits before publication, in addition to
the encoded-size cap. A valid input may therefore require refusal when generated
fan-out would exceed the output's independent lexical/structural limits.

| Boundary | Maximum |
| --- | ---: |
| Manifest bytes | 65,536 |
| Sources | 8 |
| Source bytes each / declared aggregate | 524,288 / 2,097,152 |
| Records per source / all sources | 1,024 / 2,048 |
| Roughtime addresses per record / related provenance references | 8 / 8 |
| Decoded string characters / Markdown line characters | 4,096 / 8,192 |
| JSON container depth / members per object / items per array | 12 / 32 / 4,096 |
| JSON lexical tokens / integer token characters | 500,000 / 12 |
| Candidate or diff bytes (also each imported candidate) | 16,777,216 |

UTF-8 must be strict and BOM-free. LF and CRLF documents are supported; bare CR,
terminal controls, surrogates and Unicode format/control characters are refused.
JSON structural whitespace is accepted, but decoded strings cannot contain such
controls. Source metadata remains bounded even when uninterpreted.

Output is fully validated and encoded first, written to a restrictive temporary
regular file in the output directory, flushed and file-synced, then published
with a same-directory hard link that fails if the destination exists. The
temporary name is removed after publication. If that cleanup fails after the
successful link, the command acknowledges the published result with its normal
0/3 status and a fixed cleanup warning; it does not falsely report no output.
Unsupported hard-link filesystems
fail explicitly. Interruption before publication can leave a temporary file;
it cannot replace a prior result. The caller owns its directory and temporary
cleanup. No universal crash durability, hostile-directory safety or native
Windows filesystem qualification is claimed; directory-entry durability is not
promised. A completed link is the publication point.

## Verification route and remaining limits

Committed fixtures are original synthetic `.invalid` examples with fixed public
bytes. `test_build_catalogue.py` loads this exact adjacent source file and checks
public collection/comparison paths and discriminating boundaries; it does not
import an ambient installed package. Documentary smoke inputs remain outside
the repository and are separately hash-bound.

The initial scoped host route uses Python 3.12.14 at
`/opt/codex/runtimes/codex-primary-runtime/dependencies/python/bin/python3.12`,
SHA-256 `fa67443527ed9647f760d807e2a38f26340757123e643c4639cf273ed15d5ea7`,
with `-I -B -S -X dev -W error`. The separately recorded supervisor has one worker,
zero retries, 30 seconds wall time per child, 10 seconds CPU, 512 MiB address space
and 2 MiB combined captured output. It terminates/reaps an over-budget owned child
and preserves nonzero results. An audit-hook test rejects reached socket/process
attempts; import/source review checks that the collector has neither route.
These controls are not OS network isolation or a hostile-code security sandbox.

Actual source/test/fixture/supervisor hashes, commands, failures, selected counts
and observations belong to the task receipt. There is no historical Python-policy,
mypy, Cargo, full-regression, native Windows, runtime crypto or release pass claim.
