# Strict boundary patterns

Architecture section 6 owns requirements. Preserve these constraints when
translating website schema workflows to Rust.

## Decoding without representation loss

Decode bounded raw JSON directly, never through `serde_json::Value` or maps that
may replace duplicates. Authority-bearing records need map-only objects at every
level: derived structs can accept positional arrays. `deny_unknown_fields` cannot
ensure object-only shape. Use reviewed map-only DTO adapter; prove substitution fails.

Reject duplicate recognized fields, including optional/discriminator fields and
escaped equivalent keys, plus unknown nested keys through real selected decoder.
Bounded dynamic maps detect duplicates before insertion. No `flatten`, `untagged`,
catch-all variants, generic patching, raw-value bypasses or arbitrary extensions
without reviewed versioned contract.

Consume exactly one document plus permitted trailing whitespace. Raw
`Deserializer` or `serde_path_to_error` flows explicitly call `end()`; test trailing
objects/scalars/garbage and partial frames. Keep recursion protection. Product's
16-container limit belongs to adapter/schema, not assumed serde_json depth setter.

Architecture caps IPC at 64 KiB/frame, 16 clients and four in-flight requests/client.
Preserve source/bootstrap caps; freeze field/file limits before enabling inputs.
Count collections during decoding; never reserve attacker-supplied size hints.
Post-parse `len()` alone cannot bound allocation. Charge parser scratch, escaped
strings, errors and queued responses to budget.

Overflow safety differs from resource bounds: checked arithmetic followed by
unlimited allocation stays unbounded; saturation authorizes no saturated allocation.
One owner charges aggregate work; nested helpers must not reset allowances.
For late native work distinguish cancellation requested, result invalidated,
operation completed and resources released; dropped futures prove no latter states.

## Exact values and domain construction

Use fixed-width integer DTO fields for exact times, durations, revisions/counters;
document units. Selected syntax rejects strings, fractions, exponent forms,
negative zero and out-of-range values. Test actual serde_json spellings; Rust
integer types alone prove no full syntax contract. Never round through `f64`,
architecture-sized `usize` or unchecked `as`.

Model absent/null separately where meanings differ; `Option<T>` often maps both to
None. Never default invalid supplied authority fields. Freeze enum discriminators/
versions. Canonicalize identities once under domain rule; use canonical form for
checks/effects. Display labels carry no network/path authority.

Private types enforce pure field/cross-field rules through fallible construction.
No unrestricted Deserialize for invariant-bearing types. Prefer separate DTO →
explicit conversion; approved direct forms use `#[serde(try_from = "WireType")]`
or audited equivalent. Public tuple fields, mutable inner access, unchecked `From`
and permissive Default can bypass constructors. Test-only bypasses stay test-only.

Explicitly check finite floating/FFI results; ordinary ranges do not reject NaN.
Checked arithmetic/unit conversions need post-conversion bounds. Test neighboring
integers above 2^53, signed extrema, overflow, rounding and public positive-offset
sign through adapter.

## Context, persistence and diagnostics

Only protocol-verification owner constructs authenticated evidence. Wire
`authenticated`, `admin` or `validated` flags grant nothing. Current guards use
trusted OS identity and broker policy/budgets, including after async completion.
Timeless `Validated<T>` proves no destination admission, source independence, UTC
truth or current permission.

Read persisted input through canonical protected path/handle before decoding.
Validate file-required schema, version, integrity/provenance, source binding, age
and migration. Invalid mandatory state blocks protected effects; failed updates
retain prior valid revision. Optional statistical-cache discard never resets
broker budgets or authorizes NTP downgrade.

Return stable codes/allowlisted bounded field paths. Parser Debug, raw offending
fields/values and secret-bearing errors are no product messages. Architecture
proposes at most 16 field errors/4 KiB total; qualify selected adapter/text-boundary
truncation. `serde_path_to_error` supplies location, never redaction, semantic rules
or complete consumption.

[SOURCES](../../SOURCES.md) retains primary observations; qualify current locked
versions/adapters rather than infer them from source review.
