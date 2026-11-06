"""Prepare three universal exchange conjuncts without importing proof assumptions.

Healthy jobs copy complete product modules. A proof-only Option wrapper in the
rounding copy contains a byte-exact, separately hashed expression from round().
That expression projection does not verify the complete rounding function.
No compiler, solver or proof execution occurs during preparation.
"""
from __future__ import annotations

import hashlib
import json
from pathlib import Path


SOURCE_SHA256 = {
    "coordinate.rs": "63f7230f915491973d4d682ee66afa5dc0c22bd8bc38fcc375d1840b3c046e80",
    "exchange.rs": "d7208286fb97303a64b719aa880ee07ec2411fef8c17abbda9acfb5e2b33e7b6",
    "rounding.rs": "387fa926a079aa7b87477c97dea8b79049595ac8a793ead449b19abb8b0b5519",
}
ERROR_ENUM_SHA256 = "a63f855634262baf2586e21312378f935a52e9e0ea02e77805ffd1083c01fd35"
SCALE_EXPRESSION_SHA256 = "f59283c6bf96143ddc4305ca4ec51ea5d42fe5854b82a5aa2b87eeb602762f74"
RELATIVE_LIMIT = 158_456_325_028_528_675_187_087_900_670
SCALE = 1_000_000_000
SUBJECT = "exchange_composition"

REQUIRED_ASSERTIONS = {
    "exchange_phase_full_domain": ("phase required success", "exact phase numerator", "phase producer envelope"),
    "exchange_rtt_full_domain": ("round trip required success", "exact signed round trip", "round trip producer envelope"),
    "exchange_scaling_full_domain": ("bounded scaling succeeds",),
}
REQUIRED_COVERS = {
    "exchange_phase_full_domain": ("phase zero exchange", "phase positive endpoint", "phase negative endpoint", "phase odd half-tick"),
    "exchange_rtt_full_domain": ("round trip zero exchange", "round trip positive endpoint", "round trip negative endpoint", "round trip negative tick"),
    "exchange_scaling_full_domain": ("scaling zero succeeds", "scaling positive caller endpoint", "scaling negative caller endpoint", "scaling interior succeeds"),
}
CONTROLS = {
    "always_error": {"module": "exchange.rs",
                     "before": "if !relative_in_range(offset_numerator) || !relative_in_range(round_trip_numerator) {",
                     "after": "if true {", "harness": "exchange_composition_control_success",
                     "intended_failure": "phase required success"},
    "wrong_phase": {"module": "exchange.rs", "before": ".checked_add(transmit_delta)",
                    "after": ".checked_sub(transmit_delta)", "harness": "exchange_composition_control_phase",
                    "intended_failure": "exact phase numerator"},
    "wrong_rtt": {"module": "exchange.rs", "before": ".checked_sub(peer_elapsed)",
                  "after": ".checked_add(peer_elapsed)", "harness": "exchange_composition_control_rtt",
                  "intended_failure": "exact signed round trip"},
    "wrong_scale": {"module": "rounding.rs:projected-scaling-expression",
                    "before": ".checked_mul(1_000_000_000)", "after": ".checked_mul(2_000_000_000)",
                    "harness": "exchange_composition_control_scaling", "intended_failure": "bounded scaling succeeds"},
}


def schedule() -> list[dict]:
    """Require three healthy, four distinct faults and all three restored proofs."""
    result = []
    for harness in REQUIRED_ASSERTIONS:
        result.append({"subject": SUBJECT, "case": "healthy", "harness": harness,
                       "intended_failure": None, "restored_healthy": False})
    for case, control in CONTROLS.items():
        result.append({"subject": SUBJECT, "case": case, "harness": control["harness"],
                       "intended_failure": control["intended_failure"], "restored_healthy": False})
    for harness in REQUIRED_ASSERTIONS:
        result.append({"subject": SUBJECT, "case": "healthy", "harness": harness,
                       "intended_failure": None, "restored_healthy": True})
    return result


def digest(data: bytes) -> str:
    """Bind exact source bytes, without normalizing executable code."""
    return hashlib.sha256(data).hexdigest()


def scaling_expression(rounding: bytes) -> tuple[bytes, tuple[int, int]]:
    """Select the actual checked-multiplication RHS, excluding error mapping."""
    first = b"    let scaled = "
    last = b"\n        .ok_or(DomainError::ArithmeticOverflow)?;"
    if rounding.count(first) != 1:
        raise ValueError("Actual rounding scaling expression selection is ambiguous")
    start = rounding.index(first) + len(first)
    stop = rounding.find(last, start)
    if stop < start:
        raise ValueError("Actual rounding scaling expression end is absent")
    expression = rounding[start:stop]
    if digest(expression) != SCALE_EXPRESSION_SHA256:
        raise ValueError("Actual rounding scaling expression changed")
    return expression, (start, stop)


def prepare_exchange_composition(source_root: Path, inputs: Path, case: str,
                                 harness_path: Path) -> dict:
    """Prepare byte-bound modules and the selected expression's proof wrapper.

    Complete exchange/coordinate bodies are used without alternate construction.
    Healthy rounding.rs remains a complete unchanged prefix. The wrapper's RHS
    is copied from its actual checked-multiplication expression. The scaling
    fault alters that selected expression copy only; the product-module prefix
    remains unchanged. All other faults alter one actual exchange expression.
    The caller owns execution, custody, controls and composite acceptance.
    """
    if case != "healthy" and case not in CONTROLS:
        raise ValueError("Unknown exchange-composition control")
    domain = source_root / "crates/time-domain/src"
    original = {name: (domain / name).read_bytes() for name in SOURCE_SHA256}
    for name, expected in SOURCE_SHA256.items():
        if digest(original[name]) != expected:
            raise ValueError(f"Reviewed complete module changed: {name}")
    library = (domain / "lib.rs").read_bytes()
    first = b"/// Fixed redacted domain failures"
    last = b"impl std::fmt::Display for DomainError"
    if library.count(first) != 1 or library.count(last) != 1:
        raise ValueError("Actual DomainError extraction is ambiguous")
    error_enum = library[library.index(first):library.index(last)].rstrip(b"\n") + b"\n"
    if digest(error_enum) != ERROR_ENUM_SHA256:
        raise ValueError("Actual DomainError enum changed")
    expression, offsets = scaling_expression(original["rounding.rs"])
    compiled_expression = expression
    compiled = dict(original)
    mutation = None
    if case != "healthy":
        mutation = dict(CONTROLS[case])
        before, after = mutation["before"].encode(), mutation["after"].encode()
        if case == "wrong_scale":
            if compiled_expression.count(before) != 1:
                raise ValueError("Projected scaling mutation point is not unique")
            compiled_expression = compiled_expression.replace(before, after, 1)
        else:
            name = mutation["module"]
            if compiled[name].count(before) != 1:
                raise ValueError("Actual exchange mutation point is not unique")
            compiled[name] = compiled[name].replace(before, after, 1)
    wrapper = (b"\n/// Proof-only expression projection, not a proof of the complete round function.\n"
               b"pub(crate) fn proof_nanosecond_scale(numerator: i128) -> Option<i128> {\n    "
               + compiled_expression + b"\n}\n")
    compiled["rounding.rs"] = original["rounding.rs"] + wrapper
    harness = harness_path.read_bytes()
    relative_declaration = b"const RELATIVE_LIMIT: i128 = 158_456_325_028_528_675_187_087_900_670;"
    if harness.count(relative_declaration) != 1:
        raise ValueError("Exchange-composition caller bound differs from the reviewed contract")
    proof = (b"#![allow(dead_code)]\n" + error_enum
             + b"\nmod coordinate;\nuse coordinate::UtcCoordinateQ32;\n"
             + b"mod rounding;\nuse rounding::{RoundedPhaseOffsetNs, RoundedRoundTripNs};\n"
             + b"mod exchange;\n" + harness.replace(b"//!", b"//"))
    correspondence = [{"source": f"crates/time-domain/src/{name}", "compiled_module": name,
                       "original_sha256": digest(original[name]), "compiled_sha256": digest(compiled[name]),
                       "original_bytes": len(original[name]),
                       "original_prefix_unchanged": compiled[name].startswith(original[name])}
                      for name in original]
    details = {
        "entrypoint": inputs / "proof.rs", "case": case, "mutation": mutation,
        "correspondence": correspondence, "original_library_sha256": digest(library),
        "domain_error_enum_sha256": digest(error_enum), "harness_sha256": digest(harness),
        "scaling_expression": {"source": "crates/time-domain/src/rounding.rs",
                               "source_byte_offsets_half_open": list(offsets),
                               "original_sha256": digest(expression), "compiled_sha256": digest(compiled_expression),
                               "original_bytes": expression.decode(), "compiled_bytes": compiled_expression.decode(),
                               "selection": "actual checked_mul RHS only; no complete round() proof"},
        "composite_contract": {
            "required_healthy_theorems": list(REQUIRED_ASSERTIONS),
            "required_controls": {case: control["intended_failure"] for case, control in CONTROLS.items()},
            "required_restored_theorems": list(REQUIRED_ASSERTIONS), "fixed_jobs": schedule(),
            "relative_limit": RELATIVE_LIMIT, "nanoseconds_per_second": SCALE,
            "source_sha256": dict(SOURCE_SHA256), "scaling_expression_sha256": SCALE_EXPRESSION_SHA256,
            "acceptance": "all ten fixed jobs with matching source/bound/scale identities; no individual theorem closes the composite",
            "composition": "two universal producer theorems establish both scaling premises; the universal guarded scaling theorem applies to both outputs",
        },
        "assumptions": [],
        "omitted": ["DomainError Display/Error implementations", "complete rounding verification in these selected harnesses",
                    "raw relative-predicate admission", "native clock/origin/trust/association and chronology policy"],
        "scope": "all four independent admitted signed-96-bit coordinates plus a full raw-i128 guarded scaling implication",
    }
    inputs.mkdir(mode=0o700, parents=True, exist_ok=True)
    destinations = [inputs / name for name in (*compiled, "proof.rs", "correspondence.json")]
    if any(path.exists() or path.is_symlink() for path in destinations):
        raise ValueError("Refusing to overwrite exchange-composition proof inputs")
    for name, data in compiled.items():
        (inputs / name).write_bytes(data)
    (inputs / "proof.rs").write_bytes(proof)
    (inputs / "correspondence.json").write_text(
        json.dumps({**details, "entrypoint": "proof.rs"}, indent=2) + "\n")
    return details
