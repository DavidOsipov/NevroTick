# Original synthetic fixture expectations

These reduced documents were written for this utility, not copied from upstream.
The endpoint names are reserved `.invalid` examples; the IPv6 literal is the
documentation address `2001:db8::33`. Keys contain only fixed public test bytes.
The one-byte ML-DSA-labelled value deliberately proves **unsupported evidence
accounting**, not ML-DSA key validation.

Independent expectations frozen before parser execution:

- NTS: three records, two unmarked and one source-claimed virtualized. Link label
  yields DNS `nts.example.invalid.`; `https://info.invalid/docs` stays information.
  TCP port 4460 is a separately attributed default. `Distro use only`, `Distro`,
  `All`, empty notes and `Unknown` survive without permission/time inference.
- NTP: three records. `Unknown` stratum/AS stay source claims; `AS1, AS2` creates
  no operator votes. All three endpoints use separately attributed UDP 123.
  Blank notes on the Google-like example leave time behavior unqualified.
- Roughtime: three records and four original address bindings. Alpha has numeric
  version 1 and exactly 32 zero bytes, with UDP DNS and TCP IPv6 scopes. Beta has
  exactly 32 bytes of value 1 and an unresolved IETF label. Gamma is retained as
  unsupported algorithm evidence. No profile or key is approved.
- Whole fixture set: nine records from three complete sources. Repetition with
  the same captured inputs produces identical bytes. Row reordering changes
  provenance but does not add/remove semantic claims. A changed Alpha key is a
  key diff; an absent Beta is an absent claim, never retirement.
- Wrong shape/types, duplicate JSON keys, short/noncanonical Ed25519 material,
  broken/ambiguous endpoint syntax, truncation, empty source, digest mismatch,
  forged derived fields and each declared bound-plus-one must refuse their
  respective source/command; a valid neighboring source remains reviewable.
- Publication cannot overwrite an existing file. A reached pre-publication I/O
  failure leaves the final name absent. Failure to remove a temporary name after
  a successful link is reported as a cleanup warning after published success.

One lexical test additionally uses the public list's `0.amazon.pool.ntp.org`
spelling to distinguish actual Pool-zone classification from a `.invalid`
lookalike. It performs no resolution or network operation and grants no vendor
assignment or permission.
