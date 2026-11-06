"""Prepare exact-source, full-raw-domain fixture request-timing proofs only."""
from __future__ import annotations

import hashlib
import json
import math
from pathlib import Path
import time


PREPARATION_STARTED_UNIX_NS = 1793939040629208775
PREPARATION_STARTED_MONOTONIC_NS = 35708462632598
PREPARATION_DEADLINE_MONOTONIC_NS = 39308462632598
PREPARATION_BOOT_ID = "d3fc5615-e867-45cd-b74f-4a7413afc5ab"
TOTAL_SECONDS = 3600
SOURCE_SHA256 = {
    "crates/ntp-exchange/src/context.rs": "3d84e94e9d930315a0bbc9983c21d97841cb17ed4e01c4f5e2a30e69037f10c7",
    "crates/ntp-exchange/src/lib.rs": "68811f208aff164ddc3ef911beb85bff470a1469bacaa7b5e5e5708db08b1143",
    "crates/ntp-exchange/Cargo.toml": "ddcfb2276783c3583a5c69b1e9027a7f342612fe15ad177a8c05339e6333e18f",
    "crates/time-domain/src/clock.rs": "beca67da84fc7a4717dd6f35325ec1308d4a7e5b6c2ad30dfc65392f94bbde51",
    "crates/time-domain/src/lib.rs": "a6b697df2267e6e72f99df1bcb9b0798dada3af11fbbaa24160415ae73b0861b",
    "crates/time-domain/Cargo.toml": "e0cb947e288754b7a85d3ca059f048d931bfca6d1264b7dd472fdb62569163ac",
}
SPANS = {
    "ceiling": {
        "source": "crates/ntp-exchange/src/context.rs", "offsets": (474, 609),
        "sha256": "ef92a9944c7cbc4efc2eaa52cf0b9dd5b0b445a31b7a697cd923070d24b5acfb",
        "start": b"/// Experimental request-lifetime ceiling",
        "following": b"/// Nonzero fixture source identity",
    },
    "declaration": {
        "source": "crates/ntp-exchange/src/context.rs", "offsets": (11079, 12013),
        "sha256": "53f0c893138307867fec714b4c9000b5f433715431058645e9f2660cf1902652",
        "start": b"/// Positive monotonic request lifetime", "following": b"impl RequestTiming {",
    },
    "implementation": {
        "source": "crates/ntp-exchange/src/context.rs", "offsets": (12013, 13079),
        "sha256": "af536319061ce21f9f5d2b2fe3ae59d7722a8c8c9af95668e1234a40902ee5ef",
        "start": b"impl RequestTiming {", "following": b"/// Fixed lifetime failures",
    },
    "timing_error": {
        "source": "crates/ntp-exchange/src/context.rs", "offsets": (13079, 13405),
        "sha256": "4c348dad259bb9620625ef7016d29bc0a3dc36992c0f8fcce0d4627583f7b812",
        "start": b"/// Fixed lifetime failures", "following": b"impl fmt::Display for TimingError",
    },
    "domain_error": {
        "source": "crates/time-domain/src/lib.rs", "offsets": (1080, 1922),
        "sha256": "de0f3f6abf313b4c726f9f0cd2edbc232121d8d727d270799cd07eb4b4d215af",
        "start": b"/// Fixed redacted domain failures",
        "following": b"impl std::fmt::Display for DomainError",
    },
}
FULL_HARNESS = "request_timing::request_timing_full_domain"
CONTROLS = {
    "zero_lifetime": {
        "harness": "request_timing::request_timing_control_zero_lifetime",
        "intended_failure": "timing nonpositive classification",
        "before": "if lifetime == 0 {", "after": "if false {",
    },
    "excess_lifetime": {
        "harness": "request_timing::request_timing_control_excess_lifetime",
        "intended_failure": "timing excess classification",
        "before": "if lifetime > FIXTURE_MAX_LIFETIME_NS {",
        "after": "if lifetime > FIXTURE_MAX_LIFETIME_NS + 1 {",
    },
    "retained_send": {
        "harness": "request_timing::request_timing_control_retained_send",
        "intended_failure": "timing exact send retained",
        "before": "Ok(Self { send, deadline })",
        "after": "Ok(Self { send: deadline, deadline })",
    },
}
REQUIRED_ASSERTIONS = {FULL_HARNESS: (
    "timing typed send exact", "timing typed deadline exact", "timing success iff contract",
    "timing nonpositive classification", "timing excess classification",
    "timing exact send retained", "timing exact deadline retained", "timing exact returned lifetime",
)}
REQUIRED_COVERS = {FULL_HARNESS: (
    "timing one tick succeeds", "timing equal endpoints reject", "timing reversed neighbors reject",
    "timing exact ceiling succeeds", "timing first excess rejects", "timing near maximum succeeds",
    "timing full positive span rejects", "timing reversed raw extremes reject",
)}


def digest(data: bytes) -> str:
    """Identify complete or selected original bytes without normalization."""
    return hashlib.sha256(data).hexdigest()


def remaining_budget() -> float:
    """Enforce the single backdated preparation clock without resetting it."""
    if Path("/proc/sys/kernel/random/boot_id").read_text().strip() != PREPARATION_BOOT_ID:
        raise ValueError("Request-timing preparation belongs to a different host boot")
    monotonic_elapsed = (time.monotonic_ns() - PREPARATION_STARTED_MONOTONIC_NS) / 1e9
    wall_elapsed = (time.time_ns() - PREPARATION_STARTED_UNIX_NS) / 1e9
    remaining = TOTAL_SECONDS - max(monotonic_elapsed, wall_elapsed)
    if (monotonic_elapsed < 0 or wall_elapsed < 0
            or not math.isfinite(remaining) or remaining <= 0):
        raise ValueError("Request-timing preparation/execution budget expired or clock moved backwards")
    return remaining


def schedule() -> list[dict]:
    """Return exactly healthy, three semantic faults and identical restoration."""
    jobs = [{"subject": "request_timing", "case": "healthy", "harness": FULL_HARNESS,
             "intended_failure": None, "restored_healthy": False}]
    for case, control in CONTROLS.items():
        jobs.append({"subject": "request_timing", "case": case, "harness": control["harness"],
                     "intended_failure": control["intended_failure"], "restored_healthy": False})
    jobs.append({"subject": "request_timing", "case": "healthy", "harness": FULL_HARNESS,
                 "intended_failure": None, "restored_healthy": True})
    return jobs


def harness_contracts() -> dict:
    """Declare nonempty full assertions/covers and each sole intended failure."""
    return {
        FULL_HARNESS: {"successful_assertions": list(REQUIRED_ASSERTIONS[FULL_HARNESS]),
                       "covers": list(REQUIRED_COVERS[FULL_HARNESS])},
        **{control["harness"]: {"intended_failure": control["intended_failure"], "covers": []}
           for control in CONTROLS.values()},
    }


def _extract(originals: dict[str, bytes], name: str) -> bytes:
    """Require unique reviewed anchors, exact offsets and the selected-byte pin."""
    span = SPANS[name]
    original = originals[span["source"]]
    if original.count(span["start"]) != 1 or original.count(span["following"]) != 1:
        raise ValueError("Actual request-timing source extraction is ambiguous: " + name)
    first, last = span["offsets"]
    selected = original[first:last]
    if (original.index(span["start"]) != first
            or original.index(span["following"]) != last
            or digest(selected) != span["sha256"]):
        raise ValueError("Reviewed request-timing source span changed: " + name)
    return selected


def prepare_request_timing(source_root: Path, inputs: Path, case: str,
                           harness_path: Path) -> dict:
    """Copy complete actual owners and at most one constructor-only defect.

    Every raw send/deadline pair reaches actual public clock construction and
    actual subtraction. This function executes no compiler, solver or frozen
    Python source. All source and output-collision checks precede writes.
    """
    if type(case) is not str or case != "healthy" and case not in CONTROLS:
        raise ValueError("Unknown request-timing control")
    originals = {name: (source_root / name).read_bytes() for name in SOURCE_SHA256}
    for name, expected in SOURCE_SHA256.items():
        if digest(originals[name]) != expected:
            raise ValueError("Reviewed complete source changed: " + name)
    selected = {name: _extract(originals, name) for name in SPANS}
    implementation = selected["implementation"]
    mutation = None
    if case != "healthy":
        mutation = dict(CONTROLS[case])
        before, after = mutation["before"].encode(), mutation["after"].encode()
        if implementation.count(before) != 1:
            raise ValueError("Reviewed request-timing mutation point is not unique")
        implementation = implementation.replace(before, after, 1)
    definitions = (b"// SPDX-License-Identifier: GPL-2.0-only\n"
                   b"// Exact selected actual definitions; see correspondence.json.\n"
                   + selected["ceiling"] + selected["declaration"]
                   + implementation + selected["timing_error"])
    harness = harness_path.read_bytes()
    proof = (b"#![allow(dead_code)]\n" + selected["domain_error"]
             + b"\nmod clock;\nmod request_timing {\nuse crate::clock::MonotonicNs;\n"
             + b'include!("request_timing_selected.rs");\n' + harness + b"\n}\n")
    destinations = [inputs / name for name in
                    ("proof.rs", "clock.rs", "request_timing_selected.rs", "correspondence.json")]
    if any(path.exists() or path.is_symlink() for path in destinations):
        raise ValueError("Request-timing output already exists")
    clock_name = "crates/time-domain/src/clock.rs"
    correspondence = [
        {"owner": name, "source": span["source"],
         "byte_offsets_half_open": list(span["offsets"]), "sha256": span["sha256"],
         "selection": "exact original bytes before the named constructor-only defect"}
        for name, span in SPANS.items()]
    correspondence.append({"owner": "complete_clock", "source": clock_name,
                           "byte_offsets_half_open": [0, len(originals[clock_name])],
                           "sha256": SOURCE_SHA256[clock_name],
                           "selection": "complete byte-exact actual module"})
    details = {
        "entrypoint": inputs / "proof.rs", "original_sources_sha256": dict(SOURCE_SHA256),
        "compiled_definitions_sha256": digest(definitions),
        "compiled_implementation_sha256": digest(implementation),
        "complete_clock_sha256": digest(originals[clock_name]),
        "proof_spec_sha256": digest(harness), "proof_sha256": digest(proof),
        "mutation": mutation, "correspondence": correspondence,
        "preparation_api": "prepare_request_timing(source_root, inputs, case, harness_path)",
        "harness_contracts": harness_contracts(), "recommended_schedule": schedule(),
        "assumptions": [],
        "omitted": ["native/shared origin and generation truth", "native suspend semantics",
                    "current association lifecycle and production policy", "unselected context owners",
                    "error formatting and error-trait implementations"],
        "scope": "complete two-raw-u64 fixture request-timing admission and exact retained endpoints",
    }
    inputs.mkdir(mode=0o700, parents=True, exist_ok=True)
    (inputs / "clock.rs").write_bytes(originals[clock_name])
    (inputs / "request_timing_selected.rs").write_bytes(definitions)
    (inputs / "proof.rs").write_bytes(proof)
    (inputs / "correspondence.json").write_text(
        json.dumps({**details, "entrypoint": "proof.rs"}, indent=2) + "\n")
    return details
