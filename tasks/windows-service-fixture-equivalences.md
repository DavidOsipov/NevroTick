# Selected service-fixture mutation equivalences

Companion to the [existing W11-W05/S3.1 qualification](windows11-integration.md#s31-four-owner-metric-preparation--21-november-2026), CHECK-27.
These are source-bound equivalences over the actual private owner domain, reviewed
on 21 November 2026. They supply no behavioral kills, native service evidence or
parent-phase completion. The original native06 run remains incomplete; exact raw
survivors and independent receiving admission are required before denominator
exclusions in a separately identified adjudication. No public campaign policy is
opened. Rust 1.100.0, GNU target, current standalone default graph; changes to the
named invariants or sources require new review.

## WSF-EQ-INERT-POLL-FROM

src/host.rs:149:9: replace <impl Worker for InertWorker>::poll_prepare -> Poll<Result<(), WorkerError>> with Poll::from(Ok(()))

Source host SHA-256: `e5bd7e4391d232c245fdfc0fb7ee416381dad8fd4ce9765a6dd756e42c732c69`. Candidate: `75328629358e6159cde07debf7395cf17217e3a396e76dd82ff565577ba91960`. Exact retained native06 raw outcome: MissedMutant; diff SHA-256: `51003bc608b6195815f7bdd306ec91617921f83d896c019b5023e208733e65b4`. This source equivalence has zero kill credit; the original incomplete run and all raw outcomes remain unchanged.

The changed Worker::poll_prepare boundary admits any valid InertWorker and WorkerWake. Both bodies ignore their arguments and return Poll<Result<(), WorkerError>>::Ready(Ok(())). Pinned Rust 1.100.0 core task/poll.rs:218-228 implements From<T> for Poll<T> as Poll::Ready(t); here T is Result<(), WorkerError>, not Poll. There is no allocation, self mutation, wake, error, or native/lifecycle effect.



Counterexample search: Both cancelled=false and cancelled=true receivers give the same immediate readiness; the body never inspects that bit. No failure/pending/wake endpoint distinguishes the expressions.

Assumptions and invalidation: Invalidate on return-type/From implementation change or introducing self/wake-dependent preparation.

Reviewed independently by /root/s31_metrics_plan_review through source and retained raw evidence only; no proof/compiler/test job was run by this reviewer. Source/invariant changes require re-review.

## WSF-LOGGER-EQ-INERT-POLL-FROM

Fixture candidate `51746b3327bb953c9224e2ab2d773d438a0fb3a35a66a48109572ffa4a090987`,
host source `18a35f1796827a6f3fa0a9f18d00ad9c4da3f6dd68877c0519a4d46ed3bc38be`.
Mutation37 records the exact `host.rs:150:9` replacement as `MissedMutant`.
Rendered diff SHA-256 `7f90e7479276d224b4fa7344494bcbaad468462f197df8229feeba147ac3cd04`.

Pinned Rust 1.100.0 implements `Poll::from(value)` as `Poll::Ready(value)`.
The typed value is `Ok(())` for every admitted worker and wake state. Both bodies
ignore their arguments. Neither performs a wake, changes state or emits an event.
The independent review checked both cancelled and uncancelled states and the
current caller. Changing the conversion or letting preparation depend on state
invalidates this equivalence.

## WSF-LOGGER-EQ-JOIN-TAIL-OR-AND

The same fixture candidate and host source apply. Mutation37 records
`host.rs:890:50` replacing `||` with `&&` as `MissedMutant`.
Rendered diff SHA-256 `e80b68e6e49fa796d11ab6380846e741afd4c194151eed07a64822e1b4c30dec`.

The real stop call precedes the Joining path in this poll. Reversal implies
overdue, and overdue with a trusted observation entails an absent or elapsed
deadline. Only overdue without reversal can distinguish the branches.
Both return the fixed join-tail wait in that state. The variant additionally
repeats the deadline failure call after stop has recorded the primary failure
and set the diagnostic publication flags. The repeated call changes no failure
slot, emitted record, counter, status or custody. Each path pumps diagnostics
once. The independent review challenged a future deadline with overdue set,
which the checked stop transition excludes. Allowing that state or a second
diagnostic publication invalidates this equivalence.

## WSF-LOGGER-EQ-JOIN-TAIL-POSITIVE-GUARD

The same fixture candidate and host source apply. Mutation37 records
`host.rs:896:40` replacing `!remaining.is_zero()` with `true` as `MissedMutant`.
Rendered diff SHA-256 `cf9b9f882a00fae17cb52d4402fb9e810c8c5b5f3b3be14eaba17ed180df880e`.

A Joining poll reaches this match arm only after a successful stop observation
with a trusted coordinate before the deadline. Both guards select the same
positive duration and the same bounded wait. At equality, expiry, reversal or
absent deadline, the preceding stop transition sends execution to the outer
wait branch. The independent review challenged an equal deadline without the
overdue flag, which that transition excludes. Both paths pump diagnostics once.
An intervening observation or an expiry check that admits equality invalidates
this equivalence. All three current equivalences receive zero kill credit.

## WSF-EQ-JOIN-TAIL-OR-AND

src/host.rs:827:50: replace || with && in ServiceInvocation<R, P>::poll

Source host SHA-256: `e5bd7e4391d232c245fdfc0fb7ee416381dad8fd4ce9765a6dd756e42c732c69`. Candidate: `75328629358e6159cde07debf7395cf17217e3a396e76dd82ff565577ba91960`. Exact retained native06 raw outcome: MissedMutant; diff SHA-256: `cb837e2b3d1e13e3dbe5bc48e00d253e8791225df8ebc5a17f85942a33796fd6`. This source equivalence has zero kill credit; the original incomplete run and all raw outcomes remain unchanged.

For (overdue,reversed)=(false,false), both execute identical positive-deadline arithmetic. (false,true) is excluded by the enforced owner invariant. (true,true) selects JOIN_TAIL_WAIT in both. Only (true,false) changes branch selection: original immediately returns the fixed 20ms join-tail allowance; mutant computes None or zero, sets already-true overdue, calls the idempotent failure publication described below, and returns exactly the same allowance. checked_duration_since and boolean reads are pure; the discarded new DeadlineElapsed enum has no custom Drop or allocation. Consequently output, retained causes, all failure slots, diagnostics and loss accounting, clock reads, reporter/worker calls, work-charge and admission/lifecycle state are identical.

ServiceInvocation has private fields, no Clone/Default construction, and one start_invocation_with_diagnostics construction owner (start_invocation delegates). Preparing/Dormant initializes overdue=false; stopping is entered by poll through begin_stop once. take_result cannot remove the primary failure until Finished, and Finished polls return before the mutated branch.

The host phase_deadline and ServiceLifecycle shutdown deadline use the same checked LAB_PENDING_MS=10000 and the same admitted observation. A reporter failure changes reporting status but publish installs the current StopPending state before attempting its external reporter. Duplicate lifecycle stop requests do not renew its deadline; the host poll prevents repeated begin_stop while Stopping.

ObservedClock accepts only nondecreasing Instant observations; reversal is sticky, retains the last trusted value, clears the host phase_deadline, and closes admission with a published first failure. Clock/Worker/reporter callbacks may change external state but cannot alias the private invocation fields. No clock read or callback occurs between stop return and the join-tail condition.

At the join-tail branch custody=Joining implies that this same poll executed stop and received WorkerFinish::JoinTailPending. stop observes the clock, checks the original pending lifecycle deadline, and marks overdue for absent/equal/past host deadlines or rejected observation before returning. While Stopping the host deadline is not renewed.

At this branch reversed implies overdue and phase_deadline=None. If overdue is false, reversal is false, a representable deadline exists, and last_trusted is strictly before it. If overdue is true and reversal false, the fixed deadline is absent or at/before last_trusted: entry resets stale preparing/running overdue when establishing a fresh representable shutdown budget; later accepted observations cannot move backwards.

An absent deadline without reversal arises from checked shutdown deadline overflow, whose request_stop error (or existing earlier failure) is retained and published. An equal/past representable shutdown deadline yields DeadlineElapsed from the aligned lifecycle check and is published before this branch. Thus overdue=true/reversed=false at Joining already has a primary failure and closed admission.

Every populated primary/secondary failure slot is published at its source assignment. The diagnosed flags are set before each diagnostic emit. Repeating fail(DeadlineElapsed) with a retained primary neither replaces it nor populates a secondary slot; publish_failures then emits no event and cannot change diagnostic sequence, optional loss, required-delivery state, lifecycle reporter calls, worker/permit custody, or return result.

Counterexample search: Checked None overflow, equality, strictly expired deadline, repeated expired polls, backwards time, reporter failure, preexisting worker/cleanup failure, terminal-marker versus Pending/Finished, startup/run expiry followed by fresh shutdown, and take_result boundaries. None/equality/past have already published failure; reversal takes the same branch. A fabricated overdue=true,reversed=false,future deadline with 1ms remaining would distinguish 20ms from 1ms; a fabricated failure=None could add a diagnostic. These are counterexamples over raw unconstrained fields, not valid ServiceInvocation values: private construction plus the once-only shutdown transition, aligned deadlines and monotonic observations prohibit them. Deliberate repeated direct calls to private begin_stop would violate its stated attempt-once transformation and fixed-deadline invariant; this is not a new admitted public API. If that operation is generalized, this equivalence must be reopened.

Assumptions and invalidation: Invalidate if any alternate construction/field mutation is admitted; if begin_stop can renew a Stopping invocation; if overdue is set for a nondeadline cause while its shutdown deadline remains future; if lifecycle and host deadline anchors/budgets diverge; if observations cease to be monotonic/sticky; or if fail/publish obtains a repeated side effect.

Reviewed independently by /root/s31_metrics_plan_review through source and retained raw evidence only; no proof/compiler/test job was run by this reviewer. Source/invariant changes require re-review.

## WSF-EQ-JOIN-TAIL-POSITIVE-GUARD

src/host.rs:833:40: replace match guard !remaining.is_zero() with true in ServiceInvocation<R, P>::poll

Source host SHA-256: `e5bd7e4391d232c245fdfc0fb7ee416381dad8fd4ce9765a6dd756e42c732c69`. Candidate: `75328629358e6159cde07debf7395cf17217e3a396e76dd82ff565577ba91960`. Exact retained native06 raw outcome: MissedMutant; diff SHA-256: `43e7d22e5cb4e69ba043188c954417321e150e91a06a9afd61362c90ea3fcb55`. This source equivalence has zero kill credit; the original incomplete run and all raw outcomes remain unchanged.

The match arm is reached only with overdue=false and reversed=false after this poll stop call. The enforced state invariant gives Some(deadline) with deadline>last_trusted; checked_duration_since therefore returns a strictly positive Duration. Original !remaining.is_zero() and replacement true both select remaining.min(JOIN_TAIL_WAIT). The omitted Duration predicate is pure. At actual zero/equality, past, None or reversed clock boundaries, stop has already marked overdue (and reversal when relevant), so both programs choose the outer fixed allowance. No return value, error, diagnostic, state, charge, clock/worker/reporter effect changes.

ServiceInvocation has private fields, no Clone/Default construction, and one start_invocation_with_diagnostics construction owner (start_invocation delegates). Preparing/Dormant initializes overdue=false; stopping is entered by poll through begin_stop once. take_result cannot remove the primary failure until Finished, and Finished polls return before the mutated branch.

The host phase_deadline and ServiceLifecycle shutdown deadline use the same checked LAB_PENDING_MS=10000 and the same admitted observation. A reporter failure changes reporting status but publish installs the current StopPending state before attempting its external reporter. Duplicate lifecycle stop requests do not renew its deadline; the host poll prevents repeated begin_stop while Stopping.

ObservedClock accepts only nondecreasing Instant observations; reversal is sticky, retains the last trusted value, clears the host phase_deadline, and closes admission with a published first failure. Clock/Worker/reporter callbacks may change external state but cannot alias the private invocation fields. No clock read or callback occurs between stop return and the join-tail condition.

At the join-tail branch custody=Joining implies that this same poll executed stop and received WorkerFinish::JoinTailPending. stop observes the clock, checks the original pending lifecycle deadline, and marks overdue for absent/equal/past host deadlines or rejected observation before returning. While Stopping the host deadline is not renewed.

At this branch reversed implies overdue and phase_deadline=None. If overdue is false, reversal is false, a representable deadline exists, and last_trusted is strictly before it. If overdue is true and reversal false, the fixed deadline is absent or at/before last_trusted: entry resets stale preparing/running overdue when establishing a fresh representable shutdown budget; later accepted observations cannot move backwards.

An absent deadline without reversal arises from checked shutdown deadline overflow, whose request_stop error (or existing earlier failure) is retained and published. An equal/past representable shutdown deadline yields DeadlineElapsed from the aligned lifecycle check and is published before this branch. Thus overdue=true/reversed=false at Joining already has a primary failure and closed admission.

Every populated primary/secondary failure slot is published at its source assignment. The diagnosed flags are set before each diagnostic emit. Repeating fail(DeadlineElapsed) with a retained primary neither replaces it nor populates a secondary slot; publish_failures then emits no event and cannot change diagnostic sequence, optional loss, required-delivery state, lifecycle reporter calls, worker/permit custody, or return result.

Counterexample search: Exact equality, one positive tick, one tick expired, nonadvancing valid clocks, arbitrary forward/reverse Clock sequences and pending/join-tail transitions were traced. Equality is fenced by now>=deadline before the guard. A manually forged Joining invocation with overdue=false and last_trusted=deadline distinguishes zero from 20ms, but violates the immediately preceding stop observation/expiry invariant.

Assumptions and invalidation: Invalidate on any bypass of stop before Joining, relaxed >= expiry check, mutable/renewed phase deadline, observation change between check and arithmetic, alternate construction, or side effects in the zero predicate.

Reviewed independently by /root/s31_metrics_plan_review through source and retained raw evidence only; no proof/compiler/test job was run by this reviewer. Source/invariant changes require re-review.
