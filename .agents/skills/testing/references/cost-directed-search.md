# Optional search for expensive inputs

Use for one real consumer whose work depends materially on input structure and
whose expensive cases are not already explained by useful bounds. Small fixed
capacities and explicit work bounds can make ordinary targeted cases sufficient.
Keep one bounded experiment in the existing task; this reference adopts no new
fuzzer, instrumentation or production performance threshold.

## Choose meaningful feedback

Define the admitted input domain and an actual-work metric: reached comparisons,
iterations, visits or allocations at the selected operation. Keep input size
separate; fix size or declare size classes so the search does not merely select
larger legal inputs. Source reservations, precharged call profiles and upper-bound
recipes are not observations of inner work. Wall time alone can reward scheduler
noise or setup costs rather than expensive input structure.

Qualify actual forwarding and counters with independent cheap/heavy controls,
repeated identical inputs and reset checks. Preserve output/effect semantics under
instrumentation; detect overflow/saturation and state retained across independent
cases. Reuse [harness qualification](harness-qualification.md). A growing counter
does not establish that the intended production consumer ran.

Retain inputs that improve the declared feedback and preserve functional validity.
PerfFuzz uses multidimensional per-location execution feedback; maximizing a
single scalar count is a simpler cost-directed experiment, not the same method.
Retain relevant cost-vector improvements rather than hiding one location's growth
behind an aggregate. The original C/C++ AFL-based implementation supplies no
admitted Rust/Windows integration merely because its technique is useful.

## Bound discovery and verify the witness

- Fix case/time/input-size, memory, workers, output/corpus and reduction limits.
  Keep accepted, rejected, crashed and timed-out inputs distinct. Preserve seeds,
  exact input bytes, cost vectors, metric definition and candidate/tool identities.
- Independently verify functional behavior. Reduce a heavy witness only while
  retaining its declared expensive property and valid domain; do not turn it into
  a different rejection path or change the criterion after seeing results.
- Freshly replay useful candidates and compare ordinary performance/resource
  observations under the same environment. Counter instrumentation supplies no
  uninstrumented timing, energy, native Windows or sanitizer guarantee.
- Report the greatest cost observed in the selected domain/campaign and remaining
  unexplored scope. Search supplies no global maximum, asymptotic proof or complete
  resource qualification. Keep existing CHECK-21 budgets and CHECK-19/25 harness
  obligations distinct.

Primary method: [PerfFuzz paper](https://people.eecs.berkeley.edu/~ksen/papers/perffuzz.pdf),
[official implementation and platform scope](https://github.com/carolemieux/perffuzz).
