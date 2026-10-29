# Review equivalence at the mutated boundary

Use when a surviving mutant is proposed as equivalent or excluded. The mutation
skill and configured policy retain campaign accounting and admission authority;
this reference helps distinguish a missing assertion from an actual equivalence.
It supports function/module contracts and CHECK-25, not an alternative score gate.

## A compact proof record

Keep the record in the existing task/equivalence artifact and bind it to the
exact source, mutant replacement/diff, configuration and actual outcome. Address:

- **Boundary and domain:** Which function or helper changed? What inputs and
  states does its own contract admit? Identify construction owners that enforce
  narrower invariants; do not infer a private helper's precondition solely from
  today's callers or passing generated cases.
- **Counterexample search:** Try discriminating endpoints, just-outside values,
  sign/unit changes and changed error/effect paths within that domain. A concrete
  counterexample refutes equivalence even if ordinary public callers cannot
  currently generate it.
- **Argument over the domain:** Show why original and mutant produce the same
  return values, errors, effects and relevant lifecycle behavior for all admitted
  executions. Include overflow/rounding and short-circuit effects where relevant.
  An algebraic identity over exact reals does not prove floating-point equivalence.
- **Assumptions and invalidation:** Name the types, guards and callers that make
  the argument true. Source or invariant changes require reconciliation. Have a
  reviewer challenge those assumptions before the exclusion affects a score.

Unreached code, a passing fuzz run, no found counterexample or a missing test is
not an equivalence proof. If evidence remains insufficient, retain the survivor
or unresolved disposition under the existing policy; do not invent an exclusion.

## Worked pair: raw helper versus checked caller

These small integer limits are teaching values, not NevroTick product policy.

A pure helper `in_range(v: i128)` promises to accept exactly `[-30,30]` for every
`i128` input. A mutant changes its bound to `34`. Checked callers happen to
supply only sums in `[-30,30]`; the public test suite may therefore miss it.
The helper's contract still rejects `31` and `-31`, while the mutant accepts
them. It is non-equivalent. Exercise the real helper with independently derived
endpoints, valid neighbors and invalid neighbors, preserving legal public
construction. Then run the actual mutant: adding a test alone proves no kill.

Now consider a caller that constructs two differences from immutable checked
coordinates in `[-8,7]`. Each difference lies in `[-15,15]`; their sum and
difference each lie in `[-30,30]`, with no `i128` overflow. If the caller rejects
when `!in_range(sum) || !in_range(difference)`, replacing `||` with `&&` is
equivalent **on this caller's checked-coordinate domain**: both operands are
always false. This argument also requires the evaluated helper to be total and
pure, so the changed short-circuit behavior adds no observable error or effect.
Verify those premises in actual source and all construction paths.

The caller proof does not narrow the raw helper's contract or excuse its separate
bound mutation. Conversely, do not forge invalid checked coordinates just to
make a redundant caller guard reachable. If a helper contract really requires a
narrower input domain, revise its actual enforcement deliberately; do not invent
that precondition to improve the mutation score.

For a non-equivalent survivor, retain its concrete witness, independent expected
result and healthy control, then establish the meaningful failure of the exact
mutant. For an equivalent mutant, retain the source-bound proof and reviewed
accounting; equivalent, unviable and killed remain different dispositions.

Method extracted from the paired
[S1.1c equivalence review](../../../../tasks/s1.1-time-domain-equivalences.md#non-equivalent-exchange-limit-survivor).
Its dispositions apply only to the recorded source and invariant owners.
