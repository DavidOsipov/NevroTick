# Engine KE checked-caller mutation equivalences

The existing [EP2 task](engine-transport-pilot.md#ep2--protected-request-custody-and-s2c-source-contract--11-november-2026)
owns implementation, execution, survivors and qualification. This is one ordinary
source-review record under REQ-8/BND-2, AC-04 and
CHECK-6/8/9/17/21/25/26/27. No formal compiler or solver work is selected.

**Source review accepted 11 November 2026:** `/root`,
`/root/ep1_graph_readiness`, `/root/ep1_source_review` and
`/root/deletion_reference_audit` challenged all eight checked-caller arguments.
Current complete product candidate: `1cb52d94c0b9b073b137a4ae90de8898a5040ea004482a58708ee76c7581f46c`.
Current ledger source: `1c853b6f062b5288bb485bab52caad275b41ad0646886b4594b80845592a92fe`.
Fresh complete enumeration receipt: `4403f170f4738a66109b89485da5015b5efb7e63a3748d339e79ef6c6cb86653`
(`ep2-enumeration-v2`, eight owners, 283 descriptors). Its ledger bytes match
this candidate; the later test-only scalar assertion repair changes no descriptor.

These are source-bound conditional caller equivalences. The existing reader
must still match each complete typed descriptor to an actual current
`MissedMutant`; a caught, unviable, missing or altered case cannot receive this
classification. Every raw survivor remains visible and receives zero kills.
The 95-percent adjusted per-file requirement remains unchanged.

## Shared domain, counterexamples and invalidation

The mutated boundaries are methods of the actual private `AssociationOwner`,
not a raw arithmetic or staging helper. `AssociationOwner::receive` establishes
its checked material and fresh private ledger. Only `reserve` issues opaque,
non-Clone tickets; publication, preparation, explicit return and invocation
preserve the pool, original Control, finite allowances and occupied slot.
Every earlier wrong-ticket, phase, allocation, cancellation, expiry and malformed
packet failure remains unchanged. Arguments below partition those unchanged
failures from the later admitted state where the mutated expression executes.

`stage_replacement` has a broader helper domain and separate capacity tests.
Its six survivors are not covered by these arguments. Invalid checked-owner
states are not forged merely to make the redundant caller guards reject.
No test absence, unreached Windows branch, arbitrary exclusion or compiler
failure establishes equivalence.

Counterexample search includes the eighth returned cookie, 131,072 total pool
bytes, the 32nd checkout, largest representable outgoing cookie, duplicate used
values, wrong/consumed tickets, short/error/unknown/panicking adapters,
cancellation, expiry and failure before the final reply guard. Return byte
comparisons changed to equality/inclusive are deliberately excluded: equality
is admitted at the owner-component boundary and those alternatives differ.

The compared predicates are total, bounded, read-only count/capacity/value reads
on private owned state. Short-circuit differences do not mutate state, allocate,
invoke a collaborator or observe a clock. Exact instruction count, allocator
timing and physical latency are not claimed equal. Both variants retain the
same clock-observation sites and order, with equal dispositions for corresponding
clock/cancellation/callback observations in this synchronous prototype. Equal
physical sample values across separate runs are not assumed. Native scheduling/completion remains independently unqualified.

Every admission must pin the complete current product candidate, exact ledger
source, normalized seven-field descriptor and this document. This includes the
actual `association`, `Control`, encoder, wire bounds, configurations and toolchain
inputs. New construction or mutation paths, async staging, multiple active
requests, callbacks able to mutate the owner, a new clock-observation seam,
changed limits/encoding, native custody assumptions or changed sources invalidate
the records and require independent reconciliation before reuse.

## KE-RETURN-POOL-BYTES-OR

Exact current descriptor: `crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs:654:17: replace || with && in AssociationOwner<'a>::return_unused`.
Normalized seven-field descriptor SHA256: `62bfc478a60a9cf3549bd5e13557a0fc9cac355b35c90fcf5bad57276b5492df`.

After the unchanged ticket/currentness/phase checks, one exact cookie is removed
from a previously checked pool of at most eight. No method changes that available
pool before this explicit return. Thus available count is at most seven and
available-plus-reserved bytes remain at most 131,072. Removal preserves the
original Vec capacity, which exceeds its current length. All three rejection
terms are false; both operator forms return the same cookie without copying or
renewing an allowance. Earlier rejected arguments and dropped values are unchanged.

## KE-RETURN-POOL-CAPACITY-OR

Exact current descriptor: `crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs:655:17: replace || with && in AssociationOwner<'a>::return_unused`.
Normalized seven-field descriptor SHA256: `92eef5b246e812e8b94af9891a7549c143450a877442f2ca37d6fd43dd946f09`.

The same checked return domain makes count, bytes and capacity rejection terms
false. In particular no accepted return can have `capacity <= available.len()`:
the removed cookie's original slot remains allocated. At exact maximum bytes
the original strict byte comparison remains false. Reordering pure false reads
changes neither disposition, returned allocation, retirement nor budget.

## KE-SEND-ARCHIVE-BYTES-OR

Exact current descriptor: `crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs:739:17: replace || with && in AssociationOwner<'a>::invoke_sender`.
Normalized seven-field descriptor SHA256: `ea608b8f7042eda800cb96df7f57d2be35674b867abb3dfbc62b0f34b05fb0dd`.

At most 32 original checkouts exist. The current prepared checkout has not yet
been archived, so archive length is at most 31; initial capacity is at least 32
and never shrinks. Checkout filters every value already in this archive and
exclusive mutable ownership permits no intervening archive publication.
The actual formatter bounds a sendable cookie to 3968 bytes: 4096 packet bytes
minus the 128-byte minimum header, UID, cookie-header and algorithm-15 authenticator
overhead. Including the current cookie, at most 32 bodies total at most 126,976
bytes, strictly below 131,072. Therefore all four rejection terms are false.
Both forms archive once before the same adapter invocation; every adapter outcome
retains its original no-reuse and completion custody.

## KE-SEND-ARCHIVE-CAPACITY-OR

Exact current descriptor: `crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs:740:17: replace || with && in AssociationOwner<'a>::invoke_sender`.
Normalized seven-field descriptor SHA256: `37299ec3edfaa5d5a50b4759d12221908d89c702eb1cbe5650189ebab3bdbfa4`.

The preceding send-domain argument applies independently to the changed connector.
Archive capacity is at least 32 while pre-push length is at most 31. Count, byte,
capacity and used-value checks are all false. The reserved value moves once, the
request becomes Invoked before exposure, and failed/short/unknown/panicking
callbacks cannot restore cookie permission in either variant.

## KE-SEND-ARCHIVE-VALUE-OR

Exact current descriptor: `crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs:741:17: replace || with && in AssociationOwner<'a>::invoke_sender`.
Normalized seven-field descriptor SHA256: `bdd0f93496279fcaf772f8e59c1590bc56d97ccee8b1ded0c106eea6f6f6cbac`.

`reserve` removes known used values before moving the selected exact cookie.
No other invocation can append an archive value while that request owns the
single slot. Consequently the final used-value predicate is false, alongside
the count/byte/capacity predicates. The change may skip a pure bounded comparison;
it changes no checked value, error, callback, archive entry or lifecycle fact.

## KE-SEND-ARCHIVE-BYTES-EQUALITY

Exact current descriptor: `crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs:739:26: replace > with == in AssociationOwner<'a>::invoke_sender`.
Normalized seven-field descriptor SHA256: `33f5824cf14656f9bced0ab67a09c7584a416eb6fff68c0cce8990ac13fe1e96`.

Every encoded cookie is at most 3968 bytes and at most 32 can reach this archive.
The checked sum is at most 126,976, so both `sum > 131072` and `sum == 131072`
are false throughout this caller's domain, with no integer overflow. This does
not weaken a raw/helper aggregate-capacity contract at its independent boundary.

## KE-SEND-ARCHIVE-BYTES-INCLUSIVE

Exact current descriptor: `crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs:739:26: replace > with >= in AssociationOwner<'a>::invoke_sender`.
Normalized seven-field descriptor SHA256: `c5c0c083e28b8d0d38e99d755f99ced7f3de4f6964c4516723ca38b6dbd7d6c5`.

The same strict upper bound, 126,976, excludes equality and excess. Both
comparisons are false for every admitted prepared invocation, including the
32nd checkout. A changed packet size, minimum encoding extent or attempt limit
invalidates this argument even if current tests still pass.

## KE-RECEIVE-STABLE-SLOT-OR

Exact current descriptor: `crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs:918:40: replace || with && in AssociationOwner<'a>::receive_reply`.
Normalized seven-field descriptor SHA256: `b99adc69f1b8bf4a99c8671891eed5fc921dbc6d9efc3951c5de2620eb34d8f4`.

The initial guarded charge requires Pending and saves that slot's serial.
Subsequent synchronous authentication/staging only immutably borrows the owner;
no await, external owner-mutating callback or active-slot replacement occurs.
Staging failure returns before the final closure. On success, the final original
Control guard rejects cancellation/expiry before entering that closure. Therefore
the active serial and Pending phase equal their captured values when the changed
expression executes. Both mismatch predicates are false and both variants commit
the same cookies/receipt atomically once. No future asynchronous or native
completion path inherits this argument.
