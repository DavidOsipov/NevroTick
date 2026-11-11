# Verify generated Rust API and dependency claims

Use when a generated or unfamiliar Rust API, import, feature or dependency fails
to resolve, or before admitting its first use. Applies to all contributors;
compiler failure does not identify an author or establish a hallucination.
The [anti-pattern index](anti-patterns.md), [actual baseline](modern-rust.md#check-the-actual-baseline)
and [dependency workflow](../../../environment/dependency-updates/SKILL.md)
remain authoritative. This procedure adds no dependency, model or benchmark runner.

## Diagnose before changing the graph

1. Preserve the failing candidate and actual diagnostic. Record the package,
   source revision, admitted toolchain, target, profile and selected features in
   the existing task's evidence. Distinguish source/type errors from failed
   acquisition, missing native SDK/linker, resource exhaustion or runner failure.
2. Classify the claimed name before searching a registry: standard-library item,
   local/workspace module, external package, library crate name, Cargo dependency
   alias, trait method or generated/platform binding. A `use` path is not a Cargo
   dependency list; a package name may differ from its Rust crate identifier.
3. Inspect the receiver type, required trait import/bounds and exact admitted
   source/docs. Then check version, feature, target and visibility gates. E0599
   is not proof that a package is missing or that an API was invented. A docs.rs
   page for `latest` or an all-features documentation build does not prove the
   API exists in the selected product configuration.
4. Prefer the smallest correct code/import adjustment that preserves the
   contract. If a graph change is genuinely needed, route it through dependency
   admission; do not upgrade, enable broad defaults/all-features, delete the
   lockfile, add a similarly named package, or invent a local lookalike shim just
   to make the error disappear. A shim is an implementation with its own contract,
   review and tests, not evidence the claimed upstream API exists.
5. For a new dependency, verify canonical package/source identity, exact version,
   origin, features and licensing through primary upstream material before
   acquisition/build under the configured workflow. Existence, downloads,
   checksums or a plausible name do not alone establish intended-upstream identity.
   Review build scripts, proc macros and relevant transitive execution surfaces.
6. After admission, run the existing frozen compiler/Clippy and behavioral checks
   for the affected target/feature rows. Retain the causal failure and a valid
   neighboring configuration where needed. Do not convert an environment failure,
   excluded target or zero executed tests into a code success. Compilation resolves
   only the demonstrated compile claim; API semantics and effects need their
   contract's tests.

Use a compact record in the existing task, not a new permanent registry:
`claim -> exact version/source/target/features -> diagnostic cause -> correction
-> executed check/result -> remaining semantic/native evidence`.
If the source cannot be verified, leave the proposed API/dependency unadmitted;
uncertainty is not permission to try arbitrary packages in the product build.

## Bound retries and preserve evidence

A model's second answer is a new candidate, not independent verification. Search
primary documentation with the diagnostic and concrete type, then check the
proposed correction in the same admitted lane. Preserve the first failure and
subsequent attempts; report per-attempt failures separately from eventual success.
Do not repeatedly prompt or broaden the graph without a new causal explanation.
Stop the bounded investigation when its prerequisite is unavailable and record
the specific missing source/environment instead of fabricating a working example.

For an unfamiliar diagnostic classifier, check both faulty and healthy controls:
missing trait import, unavailable feature/target, renamed dependency, nonexistent
API, transport/setup failure and a compiling implementation with wrong semantics.
Compiler text matching can assist triage but is not a root-cause oracle. Do not
install a new regex-based "hallucination lint" merely to label compiler failures.

## Untrusted generated projects and evaluation runners

Treat generated manifests and source as untrusted executable input. `cargo check`
can compile/execute build scripts and procedural macros. A timeout, temporary
working directory, `--locked`, `--offline` or `--frozen` is not an operating-system
sandbox and does not remove ambient credentials or restrict arbitrary subprocess
network/filesystem access. Do not run an external benchmark's generated projects
in the user's checkout or signed-in development environment.

If a separately authorized evaluation is worthwhile, require an owned disposable
sandbox with least privilege, no secrets/host mounts, explicit network policy,
bounded CPU/memory/disk/process/time and cleanup. Admit the runner, dependency
inputs and compiler first; preserve source/manifests, lockfiles, complete failures
and timeouts. Compare first-attempt, repair-assisted and behavioral results
separately. An external benchmark is research input, never a replacement for
NevroTick's actual target/feature, mutation, security or native QA gates.

## Primary technical sources

- [Rust E0599](https://doc.rust-lang.org/error_codes/E0599.html) and
  [method lookup](https://doc.rust-lang.org/reference/expressions/method-call-expr.html).
- [Cargo dependency renaming](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#renaming-dependencies-in-cargotoml)
  and [features](https://doc.rust-lang.org/cargo/reference/features.html).
- [docs.rs metadata](https://docs.rs/about/metadata): documentation build targets
  and features can differ from the consumer's admitted configuration.
- [Cargo build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html)
  and [Rust procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html):
  compilation has execution surfaces; provenance and containment are separate.

The [source register](../../SOURCES.md) records the bounded empirical motivation
and its limitations. No externally supplied runner or example implementation is
copied into this reference.
