# Independent public packet fixtures

These newly generated answers use PyCryptodome **4.4.0**, independently of the
RustCrypto provider and NevroTick formatter. They are not published NTS packet
KATs, live traffic or proof of entropy, KE trust, cookie permission or currentness.
All values are deliberately public. No external implementation code is copied.

Inputs: header `23` followed by 47 zero bytes; UID/key `00..1f`; Cookie `a0..ab`;
nonce `10..1f`. AAD is the literal header, UID field `01040024` + UID, Cookie
field `02040010` + Cookie, and (for `sealing-one.bin`) one `03040010` field with
12 zero body bytes. Encrypt with public `AES.MODE_SIV`, key32, supplied nonce16,
one `update(AAD)`, then `encrypt_and_digest(plaintext)`. The packet's ciphertext
is the returned tag followed by encrypted body. Its authenticator header is
`0404`, length, nonce length16 and tag-plus-body length, all big endian.

| File | Packet / AAD bytes | Plaintext | SIV tag | SHA-256 |
| --- | --- | --- | --- | --- |
| `sealing-zero.bin` | 140 / 100 | empty | `62667410631636e24c92b7109b7742d3` | `98466947920c13450f73487cc44373bd43c17ff133b66eb0fe62c8d774291a35` |
| `sealing-one.bin` | 156 / 116 | empty | `95cf1a38e57840a162034b4a328f00b8` | `abe1e2d881e9bd76aff63e0138594aa5b3278413d315d14eeb253fae5074cc4d` |
| `sealing-nonempty.bin` | 152 / 100 | `1234000402040008deadbeef` | `63e5402c59594785b42641883630926f` | `b1fece68247bbaffca32793451c85c65be13ecf5b9e7d40c999c56b4490a47bc` |

The admitted [exact wheel](https://pypi.org/pypi/pycryptodome/4.4.0/json) has
SHA-256 `32c1a3d98c93d92c2f2a45d6cc9c79fb8d7552aca1bd9a764209a3309bf6a7b1`.
The producer first passed independent RFC 5297 Appendix A.1 encryption and
decryption against its published literal answer, then produced these packets
and checked same-tool decryption. That decryption is a control, not another
independent implementation. Empty plaintext tests cannot detect changes confined
to the CTR key half; the nonempty decoder case separately challenges that half.

Retained producer SHA-256:
`60778beac6042a6ea2141b631d4bb37287556608d39f9476a74ec379b28a8588`;
vector output `872f5b420a9937a1123c17e98965d5be69f0030a4fa21dcf6368f07411263eff`;
verification `8b467eacf9ed0bbd7b102ebe2efe0a797968d4eca3308a54979f19bcbb582958`.
The [existing crypto task](../../../../tasks/s0.1-crypto-feasibility.md#s22-laboratory-request-sealing--source-contract-9-november-2026)
records raw paths, loaded-module/native identities, containment and earlier setup
failures. The oracle is a development tool, not a product dependency. Its source
and license inventory confer no constant-time, memory-erasure or provider pass.
