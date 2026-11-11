# Engine KE checked-caller mutation equivalences

The existing [EP3 task](engine-transport-pilot.md#ep3--private-protected-datagram-source-integration--11-november-2026)
owns this ordinary source-domain review under REQ-8/BND-2, AC-04 and
CHECK-6/8/9/17/21/25/26/27. No formal compiler, solver or verifier work is selected.

**Current source-domain review, 11 November 2026:** `/root`,
`/root/ep1_graph_readiness`, `/root/ep1_source_review` and
`/root/deletion_reference_audit` independently challenged these eight arguments.
Current complete product candidate: `c5a663d4ac9d680f41001c7fb07175ee373c5165eaa11087b569cffe189b1ff8`.
Current ledger source: `832166ef6fcea61756a874984a9a24f98eb232c9096484fd24ae89e0bfbe5a83`.
The typed discovery objects below match the unchanged ledger source in
`ep3-enumeration-v2`; fresh execution must re-enumerate and match them again.
The two new helper-boundary tests alter no production owner or descriptor.

The historical EP2 version remains in signed Git commit `08f274c` at this same
path (SHA256 `a09603a31912e57c0c5ec803d4293c58405ff67ac27c328e153de2a68d3871a7`).
Its candidate, outcomes and exclusions retain their historical scope. They do not
automatically qualify this continuation.

## Admission and shared domain

These are conditional checked-caller equivalences, not kills or an automatic
score adjustment. Each record must match the complete current candidate, source,
seven-field descriptor, this document and an actual current `MissedMutant`.
A caught, unviable, missing or altered case receives no such classification.
Raw survivors remain visible with zero kill credit; the 95% gate is unchanged.

The actual private association constructor checks its original Config, Control,
provider, endpoint, cookie population and ledger before publication. Opaque
non-Clone tickets and one occupied slot preserve the original budgets. The
observed path reserves before its one 56-byte entropy batch; zero wire refuses,
and accepted preparation retains UID history and a bounded encodable packet.
The adapter's readiness awaits retain exclusive mutable association custody.
They cannot change its cookie pool, archive or slot; cancellation changes only
Control. Dropping a ticket/future leaves custody occupied and does not refund.

The mutated ledger methods themselves remain synchronous. Return is allowed
only before invocation. Sender archive checks precede exposure, and the new
concrete nonblocking syscall guard neither creates another archive entry nor
permits reuse after refusal. Response opening/staging has no await or callback
able to mutate the owner; the final Control guard rejects stop/expiry before
any protected publication. Actual native completion and ordinary trust remain
independently unqualified.

Broader helper contracts are excluded from these records. In particular,
`finish_preparation` at line 603 and `stage_ready` at line 656 admit legitimate checked
Reserved/Preparing states whose misuse must be rejected. Their two observed
survivors are non-equivalent test gaps with new direct regressions. Windows-erased
faults and absent generation likewise receive no equivalence or kill credit.
No invalid checked owner fields are forged to challenge these eight bounds.

Counterexample searches include eight-entry pools and 131,072-byte totals, the 32nd checkout,
empty/unaligned/oversize cookies, duplicate initial/replacement values, wrong or
consumed tickets, failed/short/unknown/panicking senders, async wait/drop,
cancellation and expiry. The formatter rejects unaligned cookies and cookies shorter than 12 bytes,
and copies admitted bodies exactly, so raw archive equality denotes
the same outgoing wire cookie value without a padding alias. Return-byte
`>`→`==`/`>=` changes are NOT covered: exact pool equality is admitted there.

The compared expressions are bounded read-only predicates. Short-circuit
differences do not allocate, mutate state, invoke collaborators or read a clock.
Semantic equality uses corresponding clock/cancel/syscall observations; identical
physical latency or observations across separate executions are not asserted.
New construction/mutation paths, limits/encoding, multiple pending requests,
owner-reentrant callbacks, changed staging/currentness seams or source/candidate
changes invalidate these records until independently reconciled.

## KE-RETURN-POOL-BYTES-OR

Exact current descriptor: `crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs:730:17: replace || with && in AssociationOwner<'a>::return_unused`.
Normalized seven-field descriptor SHA256: `a87a555a0b7320813b941ba57d58075a93ab321a88c369969d5465380569f6f5`.

After the unchanged ticket/currentness/phase checks, one exact cookie is
removed from a previously checked pool of at most eight. No method changes that
available pool before this explicit return. Thus available count is at most
seven and available-plus-reserved bytes remain at most 131,072. Removal
preserves the original Vec capacity, which exceeds its current length. All
three rejection terms are false; both operator forms return the same cookie
without copying or renewing an allowance. Earlier rejected arguments and
dropped values are unchanged.

## KE-RETURN-POOL-CAPACITY-OR

Exact current descriptor: `crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs:731:17: replace || with && in AssociationOwner<'a>::return_unused`.
Normalized seven-field descriptor SHA256: `24c66ab6eee2fad0aecd1302e40c65dc238c3e67998a3c45ddce6cc4ddcd66f5`.

The same checked return domain makes count, bytes and capacity rejection terms
false. In particular no accepted return can have `capacity <= available.len()`:
the removed cookie's original slot remains allocated. At exact maximum bytes
the original strict byte comparison remains false. Reordering pure false reads
changes neither disposition, returned allocation, retirement nor budget.

## KE-SEND-ARCHIVE-BYTES-OR

Exact current descriptor: `crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs:808:17: replace || with && in AssociationOwner<'a>::invoke_sender`.
Normalized seven-field descriptor SHA256: `32a040453ca0049cf9103d4105d6f363f41ee1411c8a931f39688e916f066732`.

At most 32 original checkouts exist. The current prepared checkout has not yet
been archived, so archive length is at most 31; initial capacity is at least 32
and never shrinks. Checkout filters every value already in this archive and
exclusive mutable ownership permits no intervening archive publication. The
actual formatter bounds a sendable cookie to 3968 bytes: 4096 packet bytes
minus the 128-byte minimum header, UID, cookie-header and algorithm-15
authenticator overhead. Including the current cookie, at most 32 bodies total
at most 126,976 bytes, strictly below 131,072. Therefore all four rejection
terms are false. Both forms archive once before the same adapter invocation;
every adapter outcome retains its original no-reuse and completion custody.

## KE-SEND-ARCHIVE-CAPACITY-OR

Exact current descriptor: `crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs:809:17: replace || with && in AssociationOwner<'a>::invoke_sender`.
Normalized seven-field descriptor SHA256: `977e1c46a5e0da419cb93e0be69a984779995e32ffd3c80b795d850cdc9e3983`.

The preceding send-domain argument applies independently to the changed
connector. Archive capacity is at least 32 while pre-push length is at most 31.
Count, byte, capacity and used-value checks are all false. The reserved value
moves once, the request becomes Invoked before exposure, and
failed/short/unknown/panicking callbacks cannot restore cookie permission in
either variant.

## KE-SEND-ARCHIVE-VALUE-OR

Exact current descriptor: `crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs:810:17: replace || with && in AssociationOwner<'a>::invoke_sender`.
Normalized seven-field descriptor SHA256: `2914d98c7d5e21de13aef309017273e41863e2696a0f95d2bae05be7309646dc`.

`reserve` removes known used values before moving the selected exact cookie. No
other invocation can append an archive value while that request owns the single
slot. Consequently the final used-value predicate is false, alongside the
count/byte/capacity predicates. The change may skip a pure bounded comparison;
it changes no checked value, error, callback, archive entry or lifecycle fact.

## KE-SEND-ARCHIVE-BYTES-EQUALITY

Exact current descriptor: `crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs:808:26: replace > with == in AssociationOwner<'a>::invoke_sender`.
Normalized seven-field descriptor SHA256: `98757b5c7862a715cc935369fe81dacef876f5fd324d4cf6d822703a4385be55`.

Every encoded cookie is at most 3968 bytes and at most 32 can reach this
archive. The checked sum is at most 126,976, so both `sum > 131072` and `sum ==
131072` are false throughout this caller's domain, with no integer overflow.
This does not weaken a raw/helper aggregate-capacity contract at its
independent boundary.

## KE-SEND-ARCHIVE-BYTES-INCLUSIVE

Exact current descriptor: `crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs:808:26: replace > with >= in AssociationOwner<'a>::invoke_sender`.
Normalized seven-field descriptor SHA256: `8fb414d9ef6a6150f633581e22b821abff96b49a7401e94c5863e7c53c3c2191`.

The same strict upper bound, 126,976, excludes equality and excess. Both
comparisons are false for every admitted prepared invocation, including the
32nd checkout. A changed packet size, minimum encoding extent or attempt limit
invalidates this argument even if current tests still pass.

## KE-RECEIVE-STABLE-SLOT-OR

Exact current descriptor: `crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs:1011:40: replace || with && in AssociationOwner<'a>::receive_reply`.
Normalized seven-field descriptor SHA256: `0a3777dfd406bd58e49d6bd5eb095a7cf7243915799150c82c0fd15ddd1254a6`.

The initial guarded charge requires Pending and saves that slot's serial.
Subsequent synchronous authentication/staging only immutably borrows the owner;
no await, external owner-mutating callback or active-slot replacement occurs.
Staging failure returns before the final closure. On success, the final
original Control guard rejects cancellation/expiry before entering that
closure. Therefore the active serial and Pending phase equal their captured
values when the changed expression executes. Both mismatch predicates are false
and both variants commit the same cookies/receipt atomically once. The current
adapter's waits and native calls occur outside this synchronous receiving
transaction. This argument supplies no native completion, trust or future
asynchronous-staging qualification.
