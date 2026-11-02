# Staged AES-SIV fuzz experiment

This separate, unpublished workspace exercises the parent fixture's actual
authenticate-then-frame adapter. It adds no dependency to the main workspace.
The [crypto task](../../../tasks/s0.1-crypto-feasibility.md) owns executed
results, graph admission and the Windows execution hold. Target source review
and retained seeds do not establish a completed fuzz campaign.

Each input has six selector bytes: lane, corruption, a big-endian byte budget,
and a big-endian field budget. Remaining bytes are the payload. Raw mode keeps
at most 3996 payload bytes; constructed mode puts at most 3988 bytes into two
valid inner fields. Payload selection is independent of the budgets. A separate
word scanner models complete framing, error precedence and exact field spans.
Actual AES-SIV encryption produces the outer packet for the real borrowed reader
and staged adapter. Fixed keys/nonces are public test data, with no production
nonce, trust or association-policy claim.

The versioned `seeds` cover accepted empty/two-field/maximum cases, malformed
field lengths, budget neighbors and altered authentication inputs. The selected
healthy seed is `0100000c0002deadbeef`. Its bounded public witness is emitted
only after actual callback, byte/span, result and full-scratch checks.

Five feature variants deliberately break rejection, forwarding, comparison or
budget handling. They qualify this harness; they are not product mutations or
ASan detector controls. The no-oracle variant needs a fresh-process replay and
external detection of its missing witness. The wrong-budget variant uses
`0100000b0002deadbeef`.

The isolated `deny.toml` retains parent advisory/source restrictions and adds
exact fuzz build owners and a source-bound libFuzzer license clarification.
Complete source/archive/native parity and the separate LLVM notice must still
be checked. This policy requires actual frozen target-row validation before a
build; its presence proves no admission.

`input_custody.py` supplies bounded streaming reads for the qualification
controller. It keeps the historical helper immutable, retains path/descriptor
identity and size checks, and hashes every byte without caching. Run its
self-contained tests from the repository root:

```sh
python3 -B -m unittest discover -s tools/crypto-aes-siv-witness/fuzz/tests -v
```

The task's explicitly selected historical comparison also sets
`NEVROTICK_LEGACY_READER` to the retained, internally hash-pinned S44 recipe.
Requested missing/drifted input fails; the ordinary suite requires no historical
file. No Windows, host clock, service or signing operation belongs to this lane.
