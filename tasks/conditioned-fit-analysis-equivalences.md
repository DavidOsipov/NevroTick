# N2 conditioned-fit survivor dispositions

These are ordinary source-bound single-mutant equivalence arguments, not
machine-checked product proofs. No formal tool runs. They do not qualify N2 by
themselves or supply generated kill credit. Raw native survivors remain visible.

Current strengthened-test candidate: 385 product inputs,
`2fc732f1c1a6872b2458236a9e49867cf360f67886ecee17cf99481a647c278c`.
The two production owners are unchanged from the first complete campaign:

| Owner | SHA256 |
| --- | --- |
| `crates/time-core/src/conditioned_fit.rs` | `dd82bc4a3aaf1c11d23fa45a25e9b35da595b3edb9d6209fd4ef2604a5784de8` |
| `crates/time-core/src/conditioned_fit/arithmetic.rs` | `c55d6e4080a7bab100f77991ef3ebff212e556b3fc35b2b37f9bc83413af5e0e` |
| Actual N1 `conditioned_window.rs` construction owner | `7b2c34fe1c1a4d5a815d048248b120bb4c92b310998b84c0f29c5363d935c374` |

The first complete campaign, `n2-mutation-v1`, has 143 fit variants (83 caught,
57 compiler/lint-unviable, three raw survivors) and 161 arithmetic variants
(96 caught, 64 unviable, one raw survivor), with no timeout. Both baselines execute
482 actual tests. It remains incomplete. The full typed descriptors in the policy
must match re-enumeration; this note grants no campaign admission or fresh pass.

## Shared admitted domain and enforcing chain

`prepare_conditioned_suffix` takes an immutable actual
`PreparedConditionedBatch`, not a raw population count or an insufficient batch.
Its private N1 construction guarantees `3 <= n <= 64`. Private fields, no mutable
view, no unchecked conversion/default/deserialization and the separately typed
insufficient result preserve that invariant. The function admits every `usize`
start `s`, including equality and `usize::MAX`.

The sole local raw candidate stores exactly `(s, parent.sample_count())`; its
only custom Garde rule evaluates the maintained predicate:

```text
V(s,n) = (3 <= n <= 64) && (s <= n)
```

The actual fixed Garde implementation invokes that rule. The shared predicate is
pure and has no external/contextual failure source. Validation returns success
exactly when `V` holds. A failed validation enters `map_err`; successful validation
does not invoke that closure. Thus, over the complete admitted parent/start domain:

```text
validation success: s <= n
validation failure: s > n
```

This is a function-local enforcement argument over all admitted inputs, rather
than an assumption about the values supplied by today's callers. The actual
validation invocation has a separate omission control: removing it changes the
required invalid-range error classification. These equivalences neither remove
that obligation nor make validation optional.

## N2-SUFFIX-ERROR-EQUALITY

Descriptor: `crates/time-core/src/conditioned_fit.rs:417:18: replace > with >= in prepare_conditioned_suffix`.

The changed comparison is inside the validation-error closure. For every `s > n`,
both `s > n` and `s >= n` are true, and both versions construct the identical
`SuffixRange { start: s, count: n }`. For every `s <= n`, the closure is not
executed. In particular, equality `s == n` passes validation and returns the same
empty metadata-retaining insufficient suffix in both versions.

All returned values, errors, input immutability, allocation/error-report handling
and effects therefore agree. No invalid-parent witness belongs to the function's
admitted domain; simultaneously breaking validation is a different, multiple-fault
program and supplies no single-mutant counterexample.

## N2-SUFFIX-SHIELDED-RULE

Descriptor: `crates/time-core/src/conditioned_fit.rs:422:19: delete ! in prepare_conditioned_suffix`.

This condition is an `else if` after the closure's `if s > n` branch. Every
validation failure in the admitted domain has `s > n`, so the first branch
constructs `SuffixRange` before the changed expression can be evaluated.
Successful validation never invokes the closure. Deleting `!` therefore changes
no returned value, error, state, report handling or other effect over that domain.

Both arguments require the actual N1 construction invariant, exact `(s,n)`
association and unchanged sole validation predicate/dispatch. New construction or
mutation routes, broader accepted parent counts or changed Garde rules invalidate
them and require fresh review. Policy bindings must retain the actual N1 owner,
selected source hash, current candidate, complete typed mutant and this note.

## Non-equivalent survivors retained as regression gaps

The `original_index` `<` to `<=` survivor is not equivalent: with a legitimate
three-row ready suffix, local index3 changes from `None` to `Some(3)`, and its
actual `ArithmeticContext::row` consumer incorrectly succeeds. Downstream getters
mask the difference; their behavior does not narrow this helper's contract.

The arithmetic `Interval::sub` `&&` to `||` survivor is not equivalent:
`[0,1]-[1,1]` must enclose `[-1,0]`, while the mutant returns `[-1,-1]`.
`[-1,0]-[1,1]` similarly loses `-2`. Both use legitimate checked intervals.

The appended private regressions challenge those actual boundaries with valid
neighbors. Their exact mutants must fail in the fresh campaign; proposed tests,
ordinary GREEN or these arguments alone give no generated kill credit.
