"""Prepare complete byte-bound clock arithmetic without alternate construction.

API: prepare_clock(source_root, inputs, case, harness_path) -> metadata containing
the canonical inputs/proof.rs entrypoint, correspondence, harness contracts and a
ten-job healthy/control/restored schedule. No compiler or solver runs here.
"""
from __future__ import annotations

import hashlib
import json
from pathlib import Path


CLOCK_SHA256 = "beca67da84fc7a4717dd6f35325ec1308d4a7e5b6c2ad30dfc65392f94bbde51"
ERROR_ENUM_SHA256 = "de0f3f6abf313b4c726f9f0cd2edbc232121d8d727d270799cd07eb4b4d215af"
DOMAINS = {"monotonic": ("MonotonicNs", "MonotonicDurationNs", "monotonic"),
           "active_awake": ("ActiveAwakeNs", "ActiveAwakeDurationNs", "active-awake")}
DEFECTS = ("always_error", "wrong_duration", "wrong_error")


def digest(data: bytes) -> str:
    """Identify exact source bytes without newline or source normalization."""
    return hashlib.sha256(data).hexdigest()


def harness_contracts() -> dict:
    """Declare obligations independently of Kani JSON or production arithmetic."""
    result = {}
    for domain, (_, _, label) in DOMAINS.items():
        result[f"{domain}_full_domain"] = {
            "domain": domain,
            "successful_assertions": [f"{label} {name}" for name in (
                "reading exact", "duration exact result", "reverse classification",
                "elapsed admission exact", "addition exact success", "addition overflow classification")],
            "covers": [f"{label} {name}" for name in (
                "zero endpoints", "maximum duration", "reversed endpoints",
                "maximum plus zero", "full-span addition", "first addition overflow")],
        }
        for harness, assertion in (("control_zero", "duration required success"),
                                   ("control_duration", "duration exact result"),
                                   ("control_overflow", "addition overflow classification")):
            result[f"{domain}_{harness}"] = {
                "domain": domain, "intended_failure": f"{label} {assertion}", "covers": []}
    return result


def recommended_schedule() -> list[dict]:
    """Two full proofs, six distinct defects, then both restored healthy proofs."""
    result = []
    for restored in (False, True):
        if restored:
            for domain in DOMAINS:
                for defect, control in zip(DEFECTS, ("zero", "duration", "overflow"), strict=True):
                    harness = f"{domain}_control_{control}"
                    result.append({"subject": "clock", "case": f"{domain}_{defect}",
                                   "harness": harness, "restored_healthy": False,
                                   "intended_failure": harness_contracts()[harness]["intended_failure"]})
        for domain in DOMAINS:
            result.append({"subject": "clock", "case": "healthy", "harness": f"{domain}_full_domain",
                           "restored_healthy": restored, "intended_failure": None})
    return result


def prepare_clock(source_root: Path, inputs: Path, case: str, harness_path: Path) -> dict:
    """Copy the full actual module and actual enum, or one named compiled defect.

    Full-domain harnesses quantify three unrestricted u64 readings. u128 supplies
    the independent exact oracle; typed elapsed values use only the actual public
    duration_since API. No private-field assembly, assumptions, garde or external
    dependency graph is substituted. Origin, suspend semantics and native clock
    observation remain outside this arithmetic contract.
    """
    domain_root = source_root / "crates/time-domain/src"
    original = (domain_root / "clock.rs").read_bytes()
    if digest(original) != CLOCK_SHA256:
        raise ValueError("Clock source differs from reviewed complete-module correspondence")
    lib = (domain_root / "lib.rs").read_bytes()
    first = b"/// Fixed redacted domain failures"
    last = b"impl std::fmt::Display for DomainError"
    if lib.count(first) != 1 or lib.count(last) != 1:
        raise ValueError("Actual DomainError extraction is ambiguous")
    error_enum = lib[lib.index(first):lib.index(last)]
    if digest(error_enum) != ERROR_ENUM_SHA256:
        raise ValueError("Actual DomainError enum changed")
    production = original
    mutation = None
    if case != "healthy":
        selected = [(owner, defect) for owner in DOMAINS for defect in DEFECTS
                    if case == f"{owner}_{defect}"]
        if len(selected) != 1:
            raise ValueError("Unknown clock control")
        owner, defect = selected[0]
        reading, duration, _ = DOMAINS[owner]
        begin = f"impl {reading} {{".encode()
        end = f"impl {duration} {{".encode()
        if original.count(begin) != 1 or original.count(end) != 1:
            raise ValueError("Clock implementation selection is ambiguous")
        start, stop = original.index(begin), original.index(end)
        body = original[start:stop]
        if defect == "wrong_error":
            old, new = b".ok_or(DomainError::ArithmeticOverflow)?", b".ok_or(DomainError::DurationOutOfOrder)?"
        else:
            old = f"Ok({duration} {{ nanoseconds }})".encode()
            new = (b"Err(DomainError::DurationOutOfOrder)" if defect == "always_error" else
                   f"Ok({duration} {{ nanoseconds: nanoseconds ^ 1 }})".encode())
        if body.count(old) != 1:
            raise ValueError("Clock mutation point is not unique")
        production = original[:start] + body.replace(old, new, 1) + original[stop:]
        mutation = {"domain": owner, "owner_impl": reading, "before": old.decode(), "after": new.decode()}
    harness = harness_path.read_bytes()
    inputs.mkdir(parents=True, exist_ok=True)
    (inputs / "clock.rs").write_bytes(production)
    (inputs / "proof.rs").write_bytes(b"#![allow(dead_code)]\n" + error_enum
        + b"\nmod clock;\n" + harness.replace(b"//!", b"//"))
    details = {
        "entrypoint": inputs / "proof.rs", "original_clock_sha256": digest(original),
        "compiled_subject_sha256": digest(production), "domain_error_enum_sha256": digest(error_enum),
        "proof_spec_sha256": digest(harness), "mutation": mutation,
        "correspondence": [{"source": "crates/time-domain/src/clock.rs", "sha256": digest(original),
                            "selection": "complete byte-exact file before the named defect"}],
        "preparation_api": "prepare_clock(source_root, inputs, case, harness_path)",
        "harness_contracts": harness_contracts(), "recommended_schedule": recommended_schedule(),
        "assumptions": [], "omitted": [],
        "scope": "complete two-domain raw-u64 arithmetic; no native clock semantics or origin certification",
    }
    (inputs / "correspondence.json").write_text(json.dumps({**details, "entrypoint": "proof.rs"}, indent=2) + "\n")
    return details
