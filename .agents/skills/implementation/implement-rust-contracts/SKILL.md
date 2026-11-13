---
name: "implement-rust-contracts"
description: "Implement or review NevroTick function/module contracts, mandatory garde-backed domain construction, strict Serde admission and checked input-to-effect paths."
---

# Implement Rust contracts

Read [the implementation contract](../CONTRACT.md) and
[contract enforcement](references/contract-enforcement.md). Canonical architecture
sections 5–6 and 18.5 select strict Serde DTOs, mandatory shared garde validation
inside private fallible construction, and separate current effect guards.
Selected garde 0.23.0 defaults-off/derive baseline has
[exact-graph admission evidence](../../../../tasks/s0.1-source-readiness.md#refreshed-graph-and-current-evidence).
Validate each current candidate against code-owned policy; changed graphs,
features or target scopes require separate admission. Version selection alone
qualifies no candidate.
Preserve review/planning/tests-only modes.

For API, method and representation selection, apply
[Rust's strongest-applicable-contract rule](../implement-rust/SKILL.md).

Trace actual raw bytes → decoding → construction → contextual guard → effect →
acknowledgement. Identify owners and all constructor/mutation bypasses. Cover
every affected function/module contract: preconditions, postconditions, failure,
resources and permitted effects/lifecycle. Name enforcing type/constructor,
checked operation or live guard and its consumer; plain typed calls need no
invented serialization boundary.

When synchronous internal APIs must bind objects to one evaluation, consider
[generative evaluation brands](references/generative-evaluation-brands.md).
Use only when a narrow trial improves the contract; ordinary borrow lifetimes
prove no evaluation identity, and brands replace no current-authority guard.

Choose invariant-bearing types using
[Parse, don't validate](references/contract-enforcement.md#parse-dont-validate-preserve-established-invariants)
to work backward from consumer-required facts. Preserve checked results;
structural validity is neither authentication nor current permission.
For identity, normalization or comparison changes, also define
[domain equivalence and trait contracts](references/contract-enforcement.md#domain-equivalence-and-trait-contracts).
Two valid representations can still denote the same domain value.

For Rust 1.100 APIs expressing these guarantees, apply
[explicit feature contracts](../implement-rust/references/rust-1.100-contracts.md).
Preserve admitted entry and completion in named guards, real typed refusals,
mandatory delimiters, fallible allocation and identity-bearing shared ownership.
State the mistake prevented and valid behavior that must remain.

For uncertain constructor, transformation or guard bypasses across functions,
use [CodeQL data-flow analysis](../../testing/references/codeql-data-flow.md) to
investigate raw-input sources and protected sinks. Model checked-result use
separately from original DTO use; successful construction does not establish
current authorization after asynchronous work. Confirm paths in real callers.

Define changed guarantees: representation/version, exact unit/sign/range,
presence versus null, identity/normalization, cross-field rules, provenance,
resource limits and current-state requirements. Equal shapes need not imply equal
authority. Read [strict boundary patterns](references/strict-boundaries.md) before
JSON/persistence adapter changes; binary NTP/NTS needs bounded protocol-specific
parsing/checks.

Implement at the real owner:

1. Authenticate/admit transport or protected file reads; bound work/bytes before
   expensive allocation. Decode one complete supported representation into inert
   wire records; no network/file/service/clock effects inside decoders.
2. Construct private domain values through checked `TryFrom` or named constructors
   that always invoke shared garde/domain rules. Every entry path, including
   in-process callers, builders, mutation, migration and FFI conversion, must use
   that path. Consumers use its checked result, never the original DTO.
   Disable `allow_unvalidated`; justify each `skip` by actual invariant owner
   and independent review. Each derived field needs a real rule, `dive` or
   justified `skip`; rename/diagnostic metadata alone is not validation coverage.
   Maintain one rule implementation. A borrowed
   validation result cannot outlive a mutation or changed validation context.
3. Receiving service uses OS-established identity, current policy,
   revision/generation, replay/freshness/ownership and cumulative limits. Serialize
   guard/effect; recheck after async work. Validated wrappers cannot preserve
   permission indefinitely.
4. Check transformed outputs and unit/arithmetic postconditions before returning
   or publishing them. Use `Result` for rejected input and preserve prior valid
   state; release-disabled assertions cannot enforce required rejection.
   Apply checked Windows conversions and reconcile actual acknowledgements.
   Invalid input has no protected effect; ambiguous outcomes do not justify
   automatic repetition.

For history, cookie, pending-request or configuration replacement, use the
[failure-atomic state-update example](references/contract-enforcement.md#prepare-and-commit-owned-state)
to locate the commit point and rejection observations. Stage bounded results
before changing live state; preserve current-context checks at publication.

For error types/conversions, validation-error payloads or failure/recovery changes,
apply [error handling](../implement-rust/references/error-handling.md): preserve
typed causes under their privacy/resource contract, map only permitted bounded
fields to public errors/events, and verify failure atomicity at the actual owner.

Before edits, identify detecting check and valid neighboring control. Verify actual
raw adapter/checked-output consumer, constructor non-bypassability, exact integer
round trips, post-transform bounds and failure atomicity. Use
[qa-contract-tests](../../testing/qa-contract-tests/SKILL.md) and selected QA owners;
source-text matching alone is insufficient. CHECK-25 plans whole-path qualification;
real pipe identity/clock effects need Windows lanes.

For changed parser/conversion resilience, select bounded cargo-fuzz at the actual
admission seam. Kani proofs of finite arithmetic/domain invariants resume after
V1 under the deferral below; retain reachable construction and justified assumptions.
Preserve compile-fail consumer checks for privacy guarantees. Use the
[configured analysis lanes](../../testing/references/configured-tooling.md#additional-rust-analysis)
and qualify each product harness independently of tool controls.

For product mathematics, apply [OD-10’s first-release deferral](../../../../docs/nevrotick-owner-decisions.md#141-first-release-deferral--6-november-2026).
Formal proof work, including pending admission/metadata repairs, resumes after
V1; missing or invalidated proofs are no V1 blocker. Retain historical exact-scope
evidence and track affected obligations in existing post-V1 backlog. Continue
runtime/garde/private invariants, checked arithmetic and independent contract
checks. Use the [proof workflow](../../testing/references/formal-verification.md)
on post-V1 resumption.

Use [evolution](../evolve-data-contracts/SKILL.md) before acceptance, normalization,
meaning, version, storage or consumer-compatibility changes. Share pure UI/service
validators; receiver independently decodes, constructs and authorizes. Report changed
owner/path, failures, affected consumers and executed/blocked checks. Schema/compiler
success proves no security.

Adapted David Osipov skill material: [licence and notices](THIRD_PARTY_NOTICES.md).
