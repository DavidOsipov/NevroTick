"""Prepare the exact auxiliary calendar period as bounded integer SMT inputs.

This executes no solver and admits no direct-SMT profile. The small translation
from two fixed Rust expression patterns to Int is trusted correspondence code;
it is not a general Rust translator or a proof of the product calendar kernel.
"""
from __future__ import annotations

import hashlib
import json
from pathlib import Path


SOURCE = "tools/formal/calendar_composition_harness.rs"
SOURCE_SHA256 = "5a8ab5f79eabcb619d58c0d902152293cfb7ffe71d39c5cd000b1dfa20ad8b00"
MODEL_PREFIX_BYTES = 4_059
MODEL_PREFIX_SHA256 = "b88bd85b68baf3ca59089e77dffbb10c14adf7633cc164969b5c49d2d403e1b6"
U64_MAX = 18_446_744_073_709_551_615
U128_MAX = 340_282_366_920_938_463_463_374_607_431_768_211_455
U32_MAX = 4_294_967_295
MAX_YEARS = 7_378_697_629_483_820_646_399
CYCLE_DAYS = 146_097
FORWARD_SOURCE = b'''fn forward_start(years: u128) -> u128 {
    assert!(
        years <= 400 * u128::from(u64::MAX) + 399,
        "forward year representation bound"
    );
    365 * years + years / 4 - years / 100 + years / 400
}
'''
CYCLE_SOURCE = b'''fn cycle_start(year: u32) -> u32 {
    assert!(year <= 400, "cycle year representation bound");
    365 * year + year / 4 - year / 100 + year / 400
}
'''
CALL_SOURCE = b'''    assert_eq!(
        forward_start(400 * u128::from(cycles) + u128::from(year)),
        u128::from(CYCLE_DAYS) * u128::from(cycles) + u128::from(cycle_start(year)),
        "arbitrary cycle forward law"
    );
'''
CYCLE_DECLARATION = b"const CYCLE_DAYS: u32 = 146_097;\n"
CONTROLS = {
    "wrong_year_coefficient": {
        "location": "forward_start expression", "before": "365 * years", "after": "364 * years",
        "witness_forward": 145_697, "witness_rhs": 146_097,
    },
    "omit_century_subtraction": {
        "location": "forward_start expression", "before": " - years / 100", "after": "",
        "witness_forward": 146_101, "witness_rhs": 146_097,
    },
    "wrong_period": {
        "location": "right-side cycle-day coefficient", "before": "146097", "after": "146096",
        "witness_forward": 146_097, "witness_rhs": 146_096,
    },
}
MODEL_TERMS = ("cycles", "year", "cycle_year", "forward_input", "forward_result",
               "cycle_result", "cycle_independent_result", "rhs_result")
PENDING = (
    "Independently review this fixed Rust-expression/Int correspondence and its machine-range bridge.",
    "Admit a direct SMT consumer with the pinned Z3 payload before/after each bounded command.",
    "Qualify the candidate SMT-LIB logic/options and strict status/model parser; no native command has run.",
    "Use only sealed inputs, owned output, the exact admitted solver root and existing public host runtime.",
    "Require both endpoint SAT controls, healthy UNSAT, three intended semantic SAT faults and restored UNSAT.",
    "Reconcile exact source/model identities with the finite Gregorian Rust/Kani companion and its four covers.",
    "Complete source prefix/suffix, representation, finite Gregorian laws and product defect/restored controls.",
)


def digest(data: bytes) -> str:
    """Return the exact source or generated artifact identity."""
    return hashlib.sha256(data).hexdigest()


def expression_tree(text: str, variable: str) -> tuple:
    """Recognize only the two reviewed literal patterns, with Rust associativity.

    Multiplication/division bind first; addition/subtraction associate to the
    left. Parentheses, other names, other operators and whitespace drift require
    review rather than interpretation. This never evaluates source text.
    """
    if variable not in {"years", "year"}:
        raise ValueError("Unreviewed source variable")
    expected = f"365 * {variable} + {variable} / 4 - {variable} / 100 + {variable} / 400"
    if text != expected:
        raise ValueError("Unreviewed fixed Rust expression pattern")
    value = ("var", variable)
    return ("add", ("sub", ("add", ("mul", ("int", 365), value),
                                         ("div", value, ("int", 4))),
                               ("div", value, ("int", 100))),
                    ("div", value, ("int", 400)))


def source_correspondence(source_root: Path) -> dict:
    """Reject any source drift and bind complete helpers, caller and model prefix."""
    data = (source_root / SOURCE).read_bytes()
    if digest(data) != SOURCE_SHA256:
        raise ValueError("Reviewed complete calendar harness changed")
    if digest(data[:MODEL_PREFIX_BYTES]) != MODEL_PREFIX_SHA256:
        raise ValueError("Reviewed shared calendar model prefix changed")
    records = []
    for role, selected in (("forward_start", FORWARD_SOURCE), ("cycle_start", CYCLE_SOURCE),
                           ("period_call", CALL_SOURCE), ("cycle_days", CYCLE_DECLARATION)):
        if data.count(selected) != 1:
            raise ValueError("Reviewed exact source pattern is missing or ambiguous: " + role)
        start = data.index(selected)
        records.append({"role": role, "byte_offsets_half_open": [start, start + len(selected)],
                        "sha256": digest(selected), "exact_source": selected.decode()})
    forward = expression_tree(FORWARD_SOURCE.splitlines()[-2].strip().decode(), "years")
    cycle = expression_tree(CYCLE_SOURCE.splitlines()[-2].strip().decode(), "year")
    return {"source": SOURCE, "whole_sha256": SOURCE_SHA256,
            "shared_model_prefix_bytes": MODEL_PREFIX_BYTES,
            "shared_model_prefix_sha256": MODEL_PREFIX_SHA256,
            "selected": records, "forward_ast": forward, "cycle_ast": cycle,
            "units": {"inputs": "whole Gregorian years/cycles", "outputs": "whole civil days",
                      "cycle_days": CYCLE_DAYS, "years_per_cycle": 400}}


def schedule() -> list[dict]:
    """Seven future commands, including separately required feasibility controls."""
    jobs = [{"subject": "calendar_period_smt", "case": case, "expected_status": "sat",
             "restored_healthy": False, "intended_failure": None}
            for case in ("endpoint_min", "endpoint_max")]
    jobs.append({"subject": "calendar_period_smt", "case": "healthy", "expected_status": "unsat",
                 "restored_healthy": False, "intended_failure": None})
    jobs.extend({"subject": "calendar_period_smt", "case": case, "expected_status": "sat",
                 "restored_healthy": False, "intended_failure": "period_identity"}
                for case in CONTROLS)
    jobs.append({**jobs[2], "restored_healthy": True})
    return jobs


def input_names() -> tuple[str, ...]:
    """Repository inputs this preparation reads; consumer owns its other closure."""
    return (SOURCE, "tools/formal/calendar_period_smt.py")


def _range(value: str, width: int) -> str:
    """Express an unsigned machine representation as two mathematical bounds."""
    return f"(and (<= 0 {value}) (<= {value} {(1 << width) - 1}))"


class _FixedTreeEmitter:
    """Translate only internal fixed trees; accumulate every operation's bounds."""

    def __init__(self):
        self.definitions = []
        self.nodes = []
        self.checks = []

    def check(self, label: str, expression: str):
        self.checks.append({"label": label, "expression": expression})

    def emit(self, tree: tuple, bindings: dict[str, str], width: int, prefix: str) -> str:
        if tree[0] == "int":
            if type(tree[1]) is not int or not 0 <= tree[1] <= (1 << width) - 1:
                raise ValueError("Internal literal does not fit its Rust type")
            return str(tree[1])
        if tree[0] == "var":
            return bindings[tree[1]]
        if tree[0] not in {"add", "sub", "mul", "div"} or width not in {32, 128}:
            raise ValueError("Unreviewed internal arithmetic node")
        left = self.emit(tree[1], bindings, width, prefix)
        right = self.emit(tree[2], bindings, width, prefix)
        if tree[0] == "mul" and tree[1][0] != "int" and tree[2][0] != "int":
            raise ValueError("Only fixed constant multiplication is reviewed")
        if tree[0] == "div" and (tree[2][0] != "int" or tree[2][1] <= 0):
            raise ValueError("Only fixed positive division is reviewed")
        name = prefix + "_" + str(len(self.nodes))
        operation = {"add": "+", "sub": "-", "mul": "*", "div": "div"}[tree[0]]
        expression = f"({operation} {left} {right})"
        self.definitions.append(f"(define-fun {name} () Int {expression})")
        self.nodes.append({"name": name, "operation": tree[0], "left": left, "right": right,
                           "rust_width": width, "smt_expression": expression})
        self.check(name + "_fits_u" + str(width), _range(name, width))
        if tree[0] == "div":
            self.check(name + "_positive_divisor", f"(> {right} 0)")
            self.check(name + "_nonnegative_numerator", f"(<= 0 {left})")
        return name

    def alias(self, name: str, value: str):
        self.definitions.append(f"(define-fun {name} () Int {value})")


def _mutated_forward(tree: tuple, case: str) -> tuple:
    """Apply one declared semantic fault to a copied tree, never to Rust files."""
    if case == "wrong_year_coefficient":
        return ("add", ("sub", ("add", ("mul", ("int", 364), ("var", "years")),
                                             ("div", ("var", "years"), ("int", 4))),
                                   ("div", ("var", "years"), ("int", 100))), tree[2])
    if case == "omit_century_subtraction":
        return ("add", tree[1][1], tree[2])
    return tree


def _independent_gregorian_prefix(years: int) -> int:
    """Count at most 400 explicit civil years, independently of the SMT tree."""
    days = 0
    for civil_year in range(1, years + 1):
        leap = civil_year % 4 == 0 and (civil_year % 100 != 0 or civil_year % 400 == 0)
        days += 366 if leap else 365
    return days


def evaluate_witness(case: str, cycles: int, year: int, cycle_year: int) -> dict:
    """Independently evaluate a proposed SAT witness; never parse/eval Rust or SMT.

    Python's unbounded integers and divmod expose the exact machine intermediates.
    Gregorian prefix expectations use an explicit bounded civil-year count. This
    is witness checking and preparation evidence, not a universal proof.
    """
    if case not in {"healthy", "endpoint_min", "endpoint_max", *CONTROLS}:
        raise ValueError("Unknown period case")
    if (any(type(value) is not int for value in (cycles, year, cycle_year))
            or not 0 <= cycles <= U64_MAX or not 0 <= year < 400 or not 0 <= cycle_year <= 400):
        raise ValueError("Witness is outside the complete declared input domain")
    input_product = 400 * cycles
    years = input_product + year
    coefficient = 364 if case == "wrong_year_coefficient" else 365
    product = coefficient * years
    quarter, _ = divmod(years, 4)
    century, _ = divmod(years, 100)
    final_cycle, _ = divmod(years, 400)
    subtotal = product + quarter
    after_century = subtotal if case == "omit_century_subtraction" else subtotal - century
    forward = after_century + final_cycle
    cycle_values = []
    for value in (year, cycle_year):
        multiple = 365 * value
        fourth, _ = divmod(value, 4)
        hundredth, _ = divmod(value, 100)
        four_hundredth, _ = divmod(value, 400)
        addition = multiple + fourth
        subtraction = addition - hundredth
        result = subtraction + four_hundredth
        cycle_values.append((multiple, fourth, hundredth, four_hundredth, addition, subtraction, result))
    cycle_result, independent_result = (values[-1] for values in cycle_values)
    period = 146_096 if case == "wrong_period" else 146_097
    right_product = period * cycles
    rhs = right_product + cycle_result
    wide_values = (400 * U64_MAX, MAX_YEARS, input_product, years, product, quarter, century,
                   final_cycle, subtotal, after_century, forward, right_product, rhs)
    machine_ok = (years <= MAX_YEARS and all(0 <= value <= U128_MAX for value in wide_values)
                  and all(0 <= value <= U32_MAX for values in cycle_values for value in values))
    if (cycle_result != _independent_gregorian_prefix(year)
            or independent_result != _independent_gregorian_prefix(cycle_year)):
        raise ValueError("Independent civil-year evaluator disagrees with fixed cycle expression")
    return {"cycles": cycles, "year": year, "cycle_year": cycle_year,
            "forward_input": years, "forward_result": forward, "cycle_result": cycle_result,
            "cycle_independent_result": independent_result, "rhs_result": rhs,
            "machine_bounds_hold": machine_ok, "period_holds": forward == rhs,
            "violated_property": None if forward == rhs else "period_identity"}


def check_model(case: str, model: dict) -> dict:
    """Reject an untrusted SAT export unless every fixed value independently agrees."""
    if not isinstance(model, dict) or set(model) != set(MODEL_TERMS):
        raise ValueError("SAT model has missing or unexpected terms")
    if any(type(value) is not int for value in model.values()):
        raise ValueError("SAT model terms must be mathematical integers")
    if case == "healthy":
        raise ValueError("The full healthy proof requires UNSAT, never a SAT model")
    observed = evaluate_witness(case, model["cycles"], model["year"], model["cycle_year"])
    if any(model[name] != observed[name] for name in MODEL_TERMS):
        raise ValueError("SAT model differs from independent arithmetic evaluation")
    if not observed["machine_bounds_hold"]:
        raise ValueError("SAT semantic control violates a machine bound instead of its intended property")
    if case in {"endpoint_min", "endpoint_max"}:
        required = (0, 0, 0) if case == "endpoint_min" else (U64_MAX, 399, 400)
        if tuple(model[name] for name in ("cycles", "year", "cycle_year")) != required or not observed["period_holds"]:
            raise ValueError("Endpoint feasibility does not retain its required positive point")
    elif observed["period_holds"] or observed["violated_property"] != "period_identity":
        raise ValueError("SAT control does not exhibit its intended period failure")
    return observed


def build_script(correspondence: dict, case: str) -> tuple[bytes, dict]:
    """Emit one fixed query, with no assumed quotient or period identity."""
    allowed = {"healthy", "endpoint_min", "endpoint_max", *CONTROLS}
    if case not in allowed:
        raise ValueError("Unknown period case")
    emitter = _FixedTreeEmitter()
    integer, variable = lambda value: ("int", value), lambda name: ("var", name)
    upper_tree = ("add", ("mul", integer(400), integer(U64_MAX)), integer(399))
    bound = emitter.emit(upper_tree, {}, 128, "upper")
    input_tree = ("add", ("mul", integer(400), variable("cycles")), variable("year"))
    forward_input = emitter.emit(input_tree, {"cycles": "cycles", "year": "year"}, 128, "input")
    emitter.alias("forward_input", forward_input)
    emitter.check("forward_source_precondition", f"(<= forward_input {bound})")
    forward_tree = _mutated_forward(correspondence["forward_ast"], case)
    forward = emitter.emit(forward_tree, {"years": "forward_input"}, 128, "forward")
    emitter.alias("forward_result", forward)
    cycle = emitter.emit(correspondence["cycle_ast"], {"year": "year"}, 32, "cycle")
    emitter.alias("cycle_result", cycle)
    emitter.check("cycle_source_precondition", "(<= year 400)")
    independent_cycle = emitter.emit(correspondence["cycle_ast"], {"year": "cycle_year"}, 32, "extra_cycle")
    emitter.alias("cycle_independent_result", independent_cycle)
    emitter.check("independent_cycle_source_precondition", "(<= cycle_year 400)")
    period = 146_096 if case == "wrong_period" else CYCLE_DAYS
    rhs_tree = ("add", ("mul", integer(period), variable("cycles")), variable("cycle_result"))
    rhs = emitter.emit(rhs_tree, {"cycles": "cycles", "cycle_result": "cycle_result"}, 128, "rhs")
    emitter.alias("rhs_result", rhs)
    conversions = (
        ("u128::from(u64::MAX)", str(U64_MAX), 64),
        ("u128::from(cycles)", "cycles", 64), ("u128::from(year)", "year", 32),
        ("u128::from(CYCLE_DAYS)", str(period), 32),
        ("u128::from(cycle_start(year))", "cycle_result", 32),
    )
    for index, (_, value, source_width) in enumerate(conversions):
        emitter.check(f"exact_unsigned_widening_{index}",
                      f"(and {_range(value, source_width)} {_range(value, 128)})")
    checks = "\n    ".join(check["expression"] for check in emitter.checks)
    emitter.definitions.append("(define-fun machine_bounds_ok () Bool (and\n    " + checks + "))")
    emitter.definitions.append("(define-fun period_identity () Bool (= forward_result rhs_result))")
    domain = (f"(and (<= 0 cycles) (<= cycles {U64_MAX}) (<= 0 year) (<= year 399) "
              "(<= 0 cycle_year) (<= cycle_year 400))")
    expected = "unsat" if case == "healthy" else "sat"
    feasibility = case in {"endpoint_min", "endpoint_max"}
    endpoint = None
    if feasibility:
        endpoint = (0, 0, 0) if case == "endpoint_min" else (U64_MAX, 399, 400)
    commands = ["; Preparation only: auxiliary oracle Int/range leaf; native profile unqualified.",
                "; Exact calendar harness SHA256: " + SOURCE_SHA256,
                "(set-logic QF_LIA)", "(set-option :timeout 60000)"]
    if expected == "sat":
        commands.append("(set-option :produce-models true)")
    commands.extend("(declare-const " + name + " Int)" for name in ("cycles", "year", "cycle_year"))
    commands.extend(emitter.definitions)
    commands.append("(assert " + domain + ")")
    if endpoint is not None:
        commands.append(f"(assert (and (= cycles {endpoint[0]}) (= year {endpoint[1]}) (= cycle_year {endpoint[2]})))")
    commands.append("(assert " + ("(and machine_bounds_ok period_identity)" if feasibility else
                                   "(not (and machine_bounds_ok period_identity))") + ")")
    commands.append("(check-sat)")
    if expected == "sat":
        commands.append("(get-value (" + " ".join(MODEL_TERMS) + "))")
    commands.append("(exit)")
    witness = None
    if endpoint is not None:
        witness = evaluate_witness(case, *endpoint)
        if not witness["machine_bounds_hold"] or not witness["period_holds"]:
            raise ValueError("Endpoint preparation does not satisfy the independent contract")
    elif case in CONTROLS:
        witness = evaluate_witness(case, 1, 0, 0)
        control = CONTROLS[case]
        if (not witness["machine_bounds_hold"] or witness["period_holds"]
                or (witness["forward_result"], witness["rhs_result"]) !=
                (control["witness_forward"], control["witness_rhs"])):
            raise ValueError("Semantic control lacks its independent intended witness")
    details = {
        "case": case, "expected_status": expected, "endpoint_feasibility": feasibility,
        "required_model_terms": list(MODEL_TERMS) if expected == "sat" else [],
        "domain": {"cycles": [0, U64_MAX], "year": [0, 399], "cycle_year": [0, 400]},
        "input_domain_correspondence": "The original arbitrary u64 cycles supplies every cycle count; u16%400 supplies every year 0..399. The pair covers every forward input 0..400*u64MAX+399 by Euclidean division by 400. Independent cycle_year retains the cycle helper's complete 0..400 precondition.",
        "source_precondition_proved_in_conclusion": "forward_input <= 400*u64MAX+399",
        "source_ast": {"forward": correspondence["forward_ast"], "cycle": correspondence["cycle_ast"]},
        "query_forward_ast": forward_tree, "query_rhs_period": period,
        "arithmetic_nodes": emitter.nodes, "required_machine_checks": emitter.checks,
        "unsigned_widenings": [{"source": source, "value": value, "from_width": width,
                                 "to_width": 128, "interpretation": "unchanged mathematical integer"}
                                for source, value, width in conversions],
        "mutation": ({**CONTROLS[case], "kind": "SMT semantic control only; Rust source unchanged"}
                     if case in CONTROLS else None),
        "independently_evaluated_witness": witness, "assumed_results": [],
        "solver_logic_candidate": "QF_LIA with fixed positive divisors", "timeout_ms_candidate": 60_000,
        "correspondence_limit": "Rust unsigned +,-,*,/ and widening are mapped to Int only for these exact fixed trees; every operation's range and division premises are required conclusions. Correspondence code is trusted and needs independent review.",
    }
    return ("\n".join(commands) + "\n").encode(), details


def prepare_calendar_period(source_root: Path, inputs: Path, case: str, *,
                            restored_healthy: bool = False) -> dict:
    """Create read-only SMT/metadata inputs; execute and admit nothing."""
    if case not in {"healthy", "endpoint_min", "endpoint_max", *CONTROLS}:
        raise ValueError("Unknown period case")
    if restored_healthy and case != "healthy":
        raise ValueError("Only the full healthy query can be restored")
    if inputs.exists() or inputs.is_symlink():
        raise ValueError("Refusing to overwrite existing period inputs")
    correspondence = source_correspondence(source_root)
    script, query = build_script(correspondence, case)
    details = {
        "entrypoint": "proof.smt2", "preparation_only": True,
        "native_profile_qualified": False, "native_commands_executed": 0,
        "expected_status": query["expected_status"], "required_model_terms": query["required_model_terms"],
        "restored_healthy": restored_healthy, "correspondence": correspondence, "query": query,
        "script_sha256": digest(script), "generator_sha256": digest(Path(__file__).read_bytes()),
        "required_schedule": schedule(), "remaining_prerequisites": list(PENDING),
        "scope": "Auxiliary forward Gregorian period and exact unsigned machine-range bridge only.",
        "omitted": ["product Rust calendar kernel proof", "finite Gregorian Rust/Kani laws and four covers",
                    "source mapping/representation/suffix composition", "Garde/ledger/date provenance",
                    "direct solver admission, native execution, strict status/model consumer qualification"],
    }
    details = json.loads(json.dumps(details))
    metadata = (json.dumps(details, indent=2) + "\n").encode()
    inputs.mkdir(mode=0o700, parents=True)
    for name, data in (("proof.smt2", script), ("correspondence.json", metadata)):
        with (inputs / name).open("xb") as output:
            output.write(data)
        (inputs / name).chmod(0o400)
    inputs.chmod(0o500)
    return {**details, "entrypoint": inputs / "proof.smt2", "metadata_sha256": digest(metadata)}


def prepare(source_root: Path, inputs: Path, case: str, *, restored_healthy: bool = False) -> dict:
    """Consumer-facing alias; preparation still performs no execution/admission."""
    return prepare_calendar_period(source_root, inputs, case, restored_healthy=restored_healthy)
