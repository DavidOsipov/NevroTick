# Strict boundary patterns

Architecture section 6 is the requirement owner. The constraints below are easy
to lose when translating schema-based website workflows to Rust.

## Decoding without representation loss

Decode bounded raw JSON directly, not through `serde_json::Value` or a map that may
have replaced duplicate keys. Authority-bearing records require map-only object
shape at every level: derived struct deserialization can also accept positional
arrays. `deny_unknown_fields` does not make a struct object-only. Use the reviewed
map-only DTO adapter and prove substitution fails.

Reject duplicate recognized fields (including optional/discriminator fields and
escaped spellings of the same key) and unknown nested keys through the real
selected decoder. Bounded dynamic maps need duplicate detection before insertion.
Do not add `flatten`, `untagged`, catch-all variants, generic patching, raw-value
bypasses or arbitrary extensions without a reviewed versioned contract.

Consume exactly one document plus permitted trailing whitespace. A raw
`Deserializer` or `serde_path_to_error` flow explicitly calls `end()`; test trailing
objects/scalars/garbage and partial frames. Do not disable recursion protection.
The product's 16-container limit is an adapter/schema requirement, not an assumed
serde_json arbitrary-depth setter.

The architecture limits IPC to 64 KiB/frame, 16 clients and four in-flight requests
per client. Preserve its source/bootstrap caps and freeze field/file limits before
inputs are enabled. Count bounded collections while decoding; do not reserve from
attacker-controlled size hints. Post-parse `len()` alone does not bound allocations.
Parser scratch, escaped strings, errors and queued responses consume the budget.

Overflow safety and a resource bound are separate obligations: checked size
arithmetic followed by unlimited allocation remains unbounded, and saturation
does not authorize allocation of the saturated size. Charge aggregate operation
work at one owner rather than resetting allowances in nested helpers. For late
native work distinguish requested cancellation, invalidated result, operation
completion and released resources; a dropped future proves none of the latter.

## Exact values and domain construction

Use fixed-width integer DTO fields for exact times, durations, revisions/counters,
with documented units. The selected syntax rejects strings, fractions, exponent
forms, negative zero and out-of-range values. Test these spellings through actual
serde_json behavior; a Rust integer type alone does not prove the complete syntax
contract. Never round through `f64`, architecture-sized `usize` or unchecked `as`.

Model field presence independently when absent and null differ; `Option<T>` often
maps both to None. Never replace an invalid supplied authority field with a
default. Freeze enum discriminators and supported versions. Canonicalize identities
once under the domain rule and use that form for checks/effects; display labels
carry no network/path authority.

Private types enforce pure field and cross-field rules through fallible
construction. Do not derive unrestricted Deserialize over an invariant-bearing
type. Prefer separate DTO → explicit conversion; an approved direct form uses
`#[serde(try_from = "WireType")]` or an audited equivalent. Public tuple fields,
mutable inner access, unchecked `From` and permissive Default can bypass a
constructor. Test-only bypasses stay under test configuration.

Finite checks are explicit for floating calculations and FFI outputs; ordinary
range comparisons do not reject NaN. Checked arithmetic and unit conversions have
post-conversion bounds. Test neighboring integers above 2^53, signed extrema,
overflow, rounding and the public positive-offset sign through the adapter.

## Context, persistence and diagnostics

Only the protocol-verification owner constructs authenticated evidence. A wire
`authenticated`, `admin` or `validated` flag grants nothing. A current guard uses
trusted OS identity and broker-maintained policy/budgets, including after async
completion. No timeless `Validated<T>` proves network destination admission,
source independence, UTC truth or current permission.

Read persisted input through the canonical protected path/handle procedure before
decoding. Validate schema, version, integrity/provenance, source binding, age and
migration as required by that file. Invalid mandatory state blocks protected
effects; failed updates retain the prior valid revision. Discarding optional
statistical cache never resets broker budgets or authorizes NTP downgrade.

Return stable codes and allowlisted bounded field paths. Parser Debug output,
raw offending fields/values and secret-bearing errors are not product messages.
Architecture proposes at most 16 field errors/4 KiB total; qualify the chosen
adapter and text-boundary truncation. `serde_path_to_error` supplies location only;
it provides neither redaction, semantic rules nor complete consumption.

Primary source observations are in [SOURCES](../../SOURCES.md); current locked
versions/adapters must be qualified, not inferred from that source review.
