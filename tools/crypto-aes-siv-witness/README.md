# AES-SIV host witness

This standalone, GPL-2.0-only fixture observes the low-level buffer behavior of
the held `aes-siv` 0.8.0 candidate and prototypes private authenticate-then-frame
storage. It is not a product dependency or provider-adoption decision. The fixture copies no algorithm
implementation. Its public test values are from [RFC 5297 Appendix A.1](https://www.rfc-editor.org/rfc/rfc5297.html#appendix-A.1).
The pinned dependencies retain their own MIT or Apache-2.0 grants.

The six tests cover the independent known-answer vector; changed associated data;
masked and unmasked tag-bit changes; 126 headers with a healthy self-consistency
neighbor; and rejection at 127 headers. In the retained GNU host run, changed
associated data caused authentication to fail while the disjoint output still
contained the known plaintext. The in-place output was restored to ciphertext on
that tag-mismatch path. The 127-header rejection left the candidate plaintext in
both forms. These are observations of this exact low-level fixture and do not by
themselves establish an upstream API violation or an application disclosure.
The NTS AES-SIV mapping uses two associated-data components; any future
integration must keep candidate plaintext private and wipe owned output on
failure before publication.

The separate `staged` module consumes the actual borrowed outer-envelope views,
uses the original associated data followed by the declared nonce, and decrypts
in place into exclusively borrowed 4096-byte scratch. Only successful
authentication followed by complete bounded inner framing invokes the consumer.
A private drop guard wipes the entire scratch array on return or ordinary Rust
unwinding; consumer output cannot retain a borrow into that storage. This covers
neither abort/process death, caller-owned key storage, library internals nor
copies deliberately made by the consumer. It supplies no key provenance, trust,
UID/replay, cookie, sample or clock authority.

Six independent adapter tests use direct encryption with literal associated-data
components, then the real outer parser. They cover success, empty plaintext,
tampering, short tags, authenticated malformed fields, explicit budgets, the
maximum reachable packet and callback unwinding, checking all scratch bytes.
These generated packets test mapping and self-consistency, not independent NTS
interoperability. The original RFC 5297 known-answer test remains separate.
The defensive scratch-capacity rejection is unreachable through the current
sealed outer reader; no forged raw-parts constructor is introduced to test it.
Three doctest consumers cover legal use, borrow escape and scratch aliasing.

The S120 receipt records formatting, GNU and MSVC frozen dependency-policy
checks, and GNU host check, Clippy, and all six tests for the four files in this
directory. MSVC was a policy-check target, not a native test. Re-execution
requires the separately admitted toolchain, exact sources and dependency graph,
fresh advisory/index snapshot, isolated environment, and applicable resource and
custody checks. See the [crypto feasibility task](../../tasks/s0.1-crypto-feasibility.md)
and [architecture section 18.7.3](../../docs/windows-time-architecture.md#1873-nts-packet-aead-entropy-and-parser-reuse)
for the held candidate and remaining qualification gates.

S120 predates the adapter and its two new direct dependency edges. It does not
qualify this candidate. Recheck the owned manifest/lock/features and both frozen
policy rows before compiling; the current task record owns later results.
All Windows execution is stopped under its unresolved Defender incident hold.
