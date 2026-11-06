# Guarded target refinements: source candidate

**Not built, not executed, not adopted.** This directory contains a separate
Kani-driver source candidate. It does not replace the qualified compiler repair
in the parent directory, consume another diagnostic reserve, or establish an
EraWindow constructor proof. Both retained constructor diagnostics timed out
before completed checks; a refinement is a new candidate requiring review and
fresh admission.

The optional `--vtable-target-refinements PATH` flag applies supplied nonempty
subsets at exact generated callsites. The original compiler map remains intact.
The existing CBMC restriction transformation remains responsible for every
pointer equality branch and its unmatched-target `ASSERT(false)` followed by
`ASSUME(false)`. A passing proof must check every generated assertion. Reaching
an omitted actual target must fail; termination after the assertion is never a
reason to discard that failure.

## Source and application

Use the complete Kani 0.68.0 source at commit
[`0d2328a93f0e0ff66132d6bfa1a7d884877cf862`](https://github.com/model-checking/kani/tree/0d2328a93f0e0ff66132d6bfa1a7d884877cf862).
The portable [patch](proposed-target-refinements-v1.patch) changes only
`kani-driver/src/args/mod.rs`, `metadata.rs`, and `call_goto_instrument.rs`.
[Provenance](provenance.json) records their exact original Git blobs and
before/after SHA-256 hashes. No machine-local path, binary, dependency addition,
manifest change or lockfile change is part of the patch.

Apply only to a fresh private source copy after checking every original hash.
Review with the separate raw-property-ID/export candidate before preparing a
joint driver build. The parent padded-union patch continues to own its original
three compiler/binding files and historical evidence. A new driver executable
changes the admitted tool candidate; historical driver or compiler receipts
cannot qualify the combined result.

## Input contract

~~~json
{
  "version": 1,
  "refinements": [
    {
      "callsite": "<exact-generated-function>.<exact-generated-label>",
      "targets": ["<exact-generated-method-symbol>"]
    }
  ]
}
~~~

The bounds are 256 KiB of complete UTF-8 JSON, 1–64 entries, 1–64 targets per
entry, and 1–1024 UTF-8 bytes per name. Entries and targets are counted during
decoding without trusting sequence size hints. Objects must be JSON objects;
positional arrays are rejected. Unknown fields, duplicate recognized fields
(including escaped spellings), duplicate callsites, duplicate targets, unsupported
versions, nulls, trailing documents and malformed input are rejected. No `Value`
normalization or second schema engine is used. Serde's recursion limit remains.

Every requested site must exist in the generated complete linked map, and every
requested target must belong to that site's original choices. Equal nonempty
sets are legal. Every unselected site and its target array remains unchanged.
The generated original map and serialized refined map are independently bounded
at 8 MiB in this optional path.

Inputs must be regular files in the caller's owned/sealed source environment.
They are read once through a bounded handle and the exact request bytes are
retained. This is not a hostile-filesystem race defense. Artifact paths belong
to a fresh model output directory; existing receipt paths cause rejection rather
than overwrite. A failed write may leave a partial artifact and prevents model
transformation; retain it as a failed attempt.

## Driver envelope and artifacts

The flag requires `-Z restrict-vtable`, `--keep-temps`, one `--harness` with
`--exact`, `--run-sanity-checks`, structured `--export-json`, all default checks
and assertion reachability checks. It rejects raw `--cbmc-args` (including
property filters), disabled assertion overrides, contract suppression, ignored
assembly, safety-only mode, compile-only modes, playback, coverage, synthesis and
the Lean backend. The actual consumer additionally requires an ordinary
unstubbed proof without a panic expectation or function/loop contracts, and a
generated restriction artifact. Validation is rechecked at the model consumer.

After linking the original map and before the sole existing restriction call,
the driver writes:

- `*.linked-restrictions.json`: the original complete compiler map, unchanged.
- `*.target-refinements.request.json`: the exact consumed request bytes.
- `*.refined-linked-restrictions.json`: the complete checked applied map.

All remain temporary artifacts governed by the existing `--keep-temps` custody.
The supplied input path is never registered for deletion. With the option absent,
the existing map, invocation and cleanup behavior are retained.

## Required controls and remaining work

Inline unit controls are included in the patch but **have not run**. They cover
strict decoding, byte/name/collection neighbors, subset/equal-set acceptance,
unknown or added targets, unchanged other sites, retained bytes, no overwrite,
rejection before artifact creation and the conditional CLI envelope.

The [fixture](controls.rs) is preparation for a fixed four-command native control
sequence, also **not run**:

1. `guarded_singleton_healthy`: derive each call's own singleton from the actual
   emitted vtable. Require its two semantic assertions and two covers.
2. `guarded_singleton_defect`: retain the correct singletons. Require the sole
   intended failure `guarded target semantic defect`.
3. `guarded_singleton_healthy`: change only the first method's singleton to the
   other method from the **same original trait/slot set**. It remains a legal
   subset. Require the actual generated unmatched-target guard to fail, identify
   its raw property ID in the retained GBF, and match that ID to the raw export.
   A generic nonzero exit or arbitrary missing-definition failure is insufficient.
4. Restore the byte-identical healthy source and correct request. Require the
   original assertions and covers again.

The source control names do not prescribe mangled symbols. Bind requests to the
actual frozen emitted model; do not guess target correspondence from names.
Preserve all safety, sanity and unwinding checks in every command. No native
schedule, new budget, limits increase or execution permission follows from this
preparation document.

Before any constructor proof, independently reconcile original/refined map
membership, unchanged sites, generated guarded call structure, every required
raw property ID and complete exported outcomes. Preserve the full raw i128
domain and all existing success, rejection, retention, sensitivity and restored
healthy obligations. Protocol validation and source inspection cannot supply
missing solver outcomes or product acceptance.

The Kani-derived patch retains Apache-2.0 OR MIT terms; preserve upstream headers
and the parent [Apache](../LICENSE-APACHE) and [MIT](../LICENSE-MIT) notices.
