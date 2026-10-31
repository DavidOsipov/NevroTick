# Type the verification machinery

Use for Python accounting, collectors or report consumers where field/type drift
could misclassify evidence. Start with a bounded helper and its immediate callers;
do not require a repository-wide migration. Select one admitted, pinned checker
(mypy is a candidate), Python target, configuration and source/import closure.
No checker is installed or qualified by this reference. Pair static controls with
[harness qualification](harness-qualification.md) and existing runtime admission.

## Represent different evidence as different types

Define precise parameters/returns and records for reserved upper bounds, observed
counts, and complete/partial/missing observations. Preserve their actual semantics:
a complete counter row is neither a successful native assertion nor qualification.
Keep required fields on complete variants rather than a single bag of optional
results. Use literal-tagged unions and explicit exhaustive handling, or checked
nominal records where appropriate.

Different `TypedDict` names do not create distinct nominal types: compatible
shapes can interchange. Use incompatible literal discriminants or nominal types
when reservation-as-observation must be rejected. Mypy permits indexing some
nonrequired `TypedDict` fields despite possible `KeyError`; `total=False` alone
does not establish safe complete-result access. Challenge these claims directly.

## Close unchecked seams deliberately

- Inspect the pinned checker's strict bundle and selected error codes; `--strict`
  is version-dependent and does not enable every `Any` or exhaustiveness check.
  Annotate important definitions and callers. Review untyped imports, `Any`,
  casts, ignores, stub/implementation drift and config exclusions; do not hide the
  evidence pipeline behind `dict[str, Any]` or blanket missing-import ignores.
- Use `assert_never` or the selected checker's supported explicit exhaustive-match
  rule for closed variants. Demonstrate that a new unhandled status is rejected;
  a tagged union alone need not make every consumer exhaustive.
- At `importlib` boundaries, a `Protocol`, stub or cast describes an interface but
  does not prove the loaded module implements it. Check the implementation and
  consumer adapter, bind actual module identity, and challenge an incompatible
  implementation. Runtime-checkable protocols check attribute presence, not
  signature correctness. Do not treat a cast from dynamic `Any` as enforcement.
- Treat JSON/log input as untrusted until bounded runtime validation constructs
  the internal record. Annotations, `TypedDict`, `NewType` and casts do not validate
  bytes, exact keys, statuses, ranges or provenance. Preserve exact-integer checks
  where counts reject booleans: Python's `bool` is an `int` subtype. Preserve
  malformed/partial/missing data as rejected or incomplete, never a passing result.

## Qualify the selected check

Through the admitted checker entry point, retain a healthy typed consumer and
intended failures for changed guarantees: wrong field/value type, reservation or
partial data supplied as complete observations, incompatible loaded interface,
and an unhandled status. Match intended diagnostics; import/setup failures do not
count. Preserve runtime controls for malformed records and failure propagation.
Choose only relevant cases; no product build is needed solely to test annotations.

Record actual checked files/imports, version/configuration, exceptions, diagnostics
and exit propagation in the existing task. A static pass proves neither runtime
validation nor native execution. This supports CHECK-19/24/25 without changing
their qualification thresholds or admitting another tool automatically.

Primary semantics: [TypedDict compatibility and nonrequired fields](https://mypy.readthedocs.io/en/stable/typed_dict.html),
[dynamic typing and Any](https://mypy.readthedocs.io/en/stable/dynamic_typing.html),
[checker flags](https://mypy.readthedocs.io/en/stable/command_line.html),
[exhaustiveness](https://mypy.readthedocs.io/en/stable/literal_types.html#exhaustiveness-checking),
[Python typing and runtime protocols](https://docs.python.org/3/library/typing.html#typing.runtime_checkable),
[Python numeric types](https://docs.python.org/3/library/stdtypes.html#numeric-types-int-float-complex).
