# Optional executable specification and Rust trace conformance

Use when a maintained executable lifecycle specification can simplify a named
effect, acknowledgement or recovery obligation. Prefer the existing small
independent model when sufficient. Quint/Quint Connect are optional candidates,
not an adopted dependency or installation requirement. Read the parent
[model-based workflow](../qa-model-based-testing/SKILL.md) and preserve its
[harness controls](harness-qualification.md).

## Choose the evidence before adopting the stack

Define the canonical invariant, state/action domains, initial state, observable
effects and abstraction to Rust. Broker stale acknowledgements, interrupted
effects and successor recovery are plausible targets; this does not invent
production correction bounds or complete CHECK-3/10/12/27.

Separate deterministic specification tests, random simulation, model checking
and implementation trace conformance. Record exact mode, backend, assumptions,
finite domains, depth/steps and completion status. Bounded Apalache checking,
finite-state TLC exploration and an inductive proof have different claims;
neither a random trace count nor successful replay is an implementation proof.
Review the selected version's supported modes before stating completeness.

Admit the tool, backend and Rust driver dependency/feature/license graph through
the existing controlled lane before execution. Prevent implicit backend downloads
in frozen/offline verification. Documentation examples and generated drivers do
not qualify that chain; do not replace an existing Kani/Loom check without a
specific evidence or maintenance benefit.

## Qualify the trace adapter independently

- Preserve concrete action arguments, input identity, state observations and
  output/effect observations. Define how pending work, failed/rejected actions,
  time/generation changes and acknowledgements map to the specification. Reuse
  independently defined expected facts rather than production transition helpers.
- Reject unknown actions, missing observations and failed comparisons; do not
  silently skip them. Check initial state and required post-action state/effects.
  In Quint Connect, `Driver::State = ()` disables state checking: it cannot
  support a state-conformance claim merely because actions return successfully.
- Use a healthy trace and controlled wrong-action delivery, wrong state
  projection and stale-acknowledgement acceptance faults. Each must fail at the
  intended seam. State abstraction can hide a defect, so challenge omitted
  authority/effect fields and any intentionally excluded nondeterminism.
- Bound samples, trace length, resources and time. Retain concrete traces, seed,
  source/spec/driver/tool identities and first failure. Minimize and freshly replay
  failures while preserving the invariant; do not assume the selected connector
  automatically shrinks traces. Simulation replay establishes only reached cases.
- Use a simulated effect backend for routine model tests. Thread scheduling,
  real token/SCM/IPC authority, persistence and actual clock acknowledgements need
  their own applicable evidence; a pure adapter grants no native authority.

Keep specification proof/checking, driver qualification, reached Rust conformance
and remaining native gaps separate in the existing task record. A faithful model
and passing adapter still leave a correspondence and execution-coverage gap.

Primary behavior: [Quint model-based testing](https://quint.sh/docs/model-based-testing),
[Quint CLI and backend modes](https://quint.sh/docs/quint),
[Quint Connect Driver API, version 0.1.2](https://docs.rs/quint-connect/0.1.2/quint_connect/trait.Driver.html),
[trace runner controls, version 0.1.2](https://docs.rs/quint-connect/0.1.2/quint_connect/attr.quint_run.html).
Reconcile these versioned examples with the selected dependency before adoption.
