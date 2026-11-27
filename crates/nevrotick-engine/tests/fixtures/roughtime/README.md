# Roughtime verification fixtures

The `appendix-b-original-*` files contain the three requests, responses, and
public roots from [RFC 10049 Appendix B](https://www.rfc-editor.org/rfc/rfc10049.html#appendix-B).
The `erratum-9238-proposal-*` files contain the three corresponding exchanges
and the two nonce-chain `rand` values from [RFC Editor erratum 9238](https://errata.rfc-editor.org/eid9238/),
published as a Reported erratum. Unnumbered files belong to the first exchange.
The notice below applies to all of these IETF test vector files.

Copyright (c) 2026 IETF Trust and the persons identified as authors of the code.
All rights reserved.

Redistribution and use in source and binary forms, with or without modification,
is permitted pursuant to, and subject to the license terms contained in, the
Revised BSD License set forth in Section 4.c of the IETF Trust’s Legal Provisions
Relating to IETF Documents (https://trustee.ietf.org/license-info).

The `tree-*` vectors were generated independently for the NevroTick tests.
The accompanying keys are public fixture verification keys.

The depth-two vector reuses `tree-left-request.bin` at index 2 of a four-leaf
construction. Its first sibling is the original right request leaf. Its second
sibling is the parent of two copies of the original left frame with distinct
nonce bytes filling their `NONC` fields (`c0` and `c1`, respectively).

The depth-32 vector uses the original left request with 1,400 zero padding
bytes (1,536 total bytes) and index `0xffffffff`. Its consecutive siblings are
perfect subtrees of height 0 through 31 whose leaves are hashes of the original
right request. This produces the signed 1,468-byte response. The two new replies
reuse the original delegation and sign their complete `SREP` values using
OpenSSL Ed25519 and the same public fixture seed, bytes 32 through 63. Python
SHA-512 provided the truncated 32-byte leaf and parent hashes for both vectors.
