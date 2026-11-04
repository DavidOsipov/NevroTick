# Pinned leap-data inputs

This directory owns the small local inputs for
[S1.4 pure leap knowledge](../../tasks/s1.4-leap-policy.md#11-pure-knowledge-contract-preparation--4-november-2026).
The offline Python generator is implemented in `generate.py`; the separate Rust
knowledge owner and its qualification remain with S1.4. These pins establish
input identity and source review, not product or release qualification.
Builds must use reviewed local bytes and must not fetch mutable network data.

`provenance.json` binds the IANA 2026f leap list and its public-domain licence.
The complete raw file is 5,065 bytes, with 28 records. A second local readback
verified both SHA256 values; the supplied compatibility checksum also matches.
Its SHA-1 is an unkeyed format-consistency value, not an authentication mechanism.
The existing complete authenticated release remains the distribution boundary.

The explicit historical-completeness endpoint is 6 July 2026 at 00:00 UTC,
supported conservatively by IERS Bulletin C 72. It is separate from the last
record, file update and finite expiry. The selected BIPM rule starts on 20 May
2027 and creates no event or offset impulse. It cannot fill a preceding gap in
offset knowledge. The provenance JSON records the primary URLs and exact NTP
seconds; its rule fields are reviewed facts, not a hash of the upstream web page.
Production-supported date/era policy is not selected here: the domain API must
require an explicit finite caller scope.

## Grammar to enforce before parser acceptance

Use the limits in `provenance.json`, without truncating or repairing input.
Bytes are printable ASCII plus horizontal tab and LF/CRLF line endings. Reject
NUL, other controls, non-ASCII and bare CR. A final line without LF is permitted.
Blank lines may contain spaces/tabs. Nonblank records and comments start in
column zero; reject leading whitespace rather than silently hiding metadata.

An unsigned decimal timestamp contains 1–19 ASCII digits and is at most
`i64::MAX`. A signed offset has an optional ASCII `+`/`-`, at least one decimal
digit, at most 11 total characters and an `i32` value. No decimal sign, base
prefix or `infinity` spelling is accepted for timestamps. Preserve original
digit lexemes for the compatibility checksum before numeric normalization.

- A record is a timestamp, one or more spaces/tabs, an offset, optional
  spaces/tabs and an optional `#` comment. Consume the complete record.
- `#$` and `#@` each introduce exactly one timestamp, separated by spaces/tabs,
  with optional trailing spaces/tabs and a `#` comment. Each kind occurs at most
  once. Absence remains absence; zero is an explicit finite value, not absence.
- `#h` contains exactly five space/tab-separated hexadecimal words, each 1–8
  digits without sign or `0x` prefix. Expand each word to four big-endian bytes.
  Permit only trailing spaces/tabs and an optional `#` comment afterward.
- Other column-zero `#` lines are bounded comments. Semantic metadata is never
  recognized inside a comment, and duplicate metadata is rejected.
  An initial `#$`, `#@` or `#h` byte pair reserves a semantic line, including
  malformed adjacent text such as `#@x`; it cannot fall back to an ordinary comment.

Compute the compatibility digest in physical line order. For each `#$`/`#@`
line take the bytes after its two-character marker; for each record take the
whole line. Stop each contribution at its first subsequent `#`, retain only
ASCII digits and concatenate them. Exclude `#h` and ordinary comments. Compare
the SHA-1 digest with the five decoded words when supplied; distinguish absent,
malformed and mismatching checksums. Use this convention only; do not import
upstream truncation, permissive integer parsing or runtime leap learning.

Before publishing output, validate the entire bounded ledger and construct the
complete deterministic result. Failure preserves the previous output and checked
knowledge. The generator must record explicit reviewed historical completeness;
parser success or a matching checksum cannot manufacture that fact.

The project's original generator/domain code uses the repository code licence.
The copied leap list and `LICENSE.iana` retain their separate public-domain
status and notices. No NTPsec source code is copied into this directory.

## Offline generation and checks

Run from the repository root with Python 3.11 or later; only its standard library
is used:

```sh
python3 -B tools/leap-data/generate.py --write
python3 -B tools/leap-data/generate.py --check
python3 -X dev -Werror -B tools/leap-data/test_generate.py -v
```

The default artifact is `tools/leap-data/generated-data.rs`. It contains
`LEAP_RECORDS: &[(u64, i32)]` including the distinct first baseline,
`HISTORY_COMPLETE_BEFORE: u64`, `FINITE_FUTURE_BEFORE: Option<u64>`,
`CONTINUOUS_FROM: Option<u64>`, `DATA_REVISION` and raw/provenance SHA256 arrays.
All record seconds are bounded by `i64::MAX`. Calendar validation uses the
Gregorian 400-year cycle, so a structurally valid wide timestamp does not acquire
an accidental Python year-9999 limit or packet-era wrapping.

For a separate normalized JSON artifact, use an explicit local output:

```sh
python3 -B tools/leap-data/generate.py --format json --write --output /existing/directory/leap-data.json
python3 -B tools/leap-data/generate.py --format json --check --output /existing/directory/leap-data.json
```

`--provenance` selects an explicit local manifest for a reviewed candidate.
The loader rejects unknown or duplicate JSON fields, incorrect exact types,
unrecognized limits, mismatched date spellings, missing files and inconsistent
raw/licence pins or metadata inventories. It never supplies omitted review
facts. The manifest has its own 16,384-byte and eight-container-depth bounds;
every text field is printable ASCII and at most 1,024 characters. A raw or
licence read requests at most 65,537 bytes; the raw ledger's other limits remain
those frozen above and in the unchanged provenance. POSIX input descriptors are
opened nonblocking and verified regular before reading, so a FIFO cannot wait
for a writer. Symlink inputs and symlink output leaves are rejected.

Both output forms preserve separate history, optional finite future coverage and
the selected continuous-rule boundary. Missing expiry stays absent; an explicit
zero stays finite zero. No rule creates a transition or fills an earlier unknown
offset interval. Supplied SHA-1 is checked with `usedforsecurity=False`; raw and
provenance SHA256 identify inputs and reviewed semantics, without authenticating
a release.

`--check` compares the complete expected bytes, including any unexpected suffix.
`--write` validates all inputs and constructs the bounded complete result before
creating a temporary file beside the output, then atomically replaces the output.
Validation or publication failure preserves the prior output and removes an
unpublished temporary file. Pinned input paths cannot be output targets.
This is candidate replacement atomicity, not package rollback or crash-durability
qualification. Diagnostics are bounded to 240 message characters plus a fixed
prefix. There is no network request, build-time retrieval or runtime file reload.
