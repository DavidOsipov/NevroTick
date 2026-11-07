# Engine source provenance

The engine inherits GPL-2.0-only from the workspace. Its original transport
experiment and conditional measurement/relative/series owners retain their exact
source and evidence in [the engine task](../../tasks/engine-transport-pilot.md).
This record does not transfer their Linux fixture evidence to native Windows,
production acquisition or release qualification.

## Q1 conditional basic-client quality

The selected equations in `src/quality.rs` adapt Chrony 4.9 at immutable commit
`8df4f1263e206585e6c4e61352d042c2dd12916a`:

| Source | Selected body | SHA256 |
| --- | --- | --- |
| `ntp_core.c` | `process_response`, lines2061,2161–2199,2221–2225 | `ca206410bddc03c56996bba2bddbca2ab01029f9c5b619d4a55025f37d8c4ac0` |
| `util.c` | `UTI_Ntp32ToDouble`, lines651–654; `UTI_Log2ToDouble`, lines908–920 | `c0e80e237ceb9705fb42407ab9dd4dffbb29c711e48ccca30f925d716d0cd9dd` |

Both originals carry Copyright (C) Richard P. Curnow 1997–2003.
`ntp_core.c` additionally carries Copyright (C) Miroslav Lichvar 2009–2024;
`util.c` carries Copyright (C) Miroslav Lichvar 2009, 2012–2023. Both grant GNU GPL
version 2, without an “or later” grant, and disclaim warranty. Preserve their
source notices, this adaptation record, [COPYING](COPYING) and corresponding
source/build obligations when distributing derived work.

Local adaptation by David Osipov with AI assistance, 7 November 2026: checked
Rust construction; borrowed actual transport/assumption association; exact
corrected interval differences and separate conversion bounds; explicit numeric
assumptions/limits; finite intermediate guards and typed rejection. The contract
permits normal binary64 gradual underflow and promises computed nominal quality,
not a conservative physical or total arithmetic error enclosure. No upstream
default, interleaved/symmetric/PTP mode, delay adaptation, authentication,
eligibility or production history is supplied. See the exact
[port ledger](../../tasks/s0.1-chrony-ledger.json) and
[receiving contract](../../tasks/engine-transport-pilot.md#receiving-transaction-reconciliation--7-november-2026).

Source authoring and planned controls establish no executed qualification.
