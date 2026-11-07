#!/usr/bin/env python3
"""Qualify the admitted S1.1c/S1.4 portable domain analysis scope.

GPL-2.0-only, David Osipov with AI assistance, 29 October 2026. Orchestration
adapts the repository's verify-wire-analysis.py; shared custody/accounting lives
in portable-analysis.py. No downloads, installations or host Cargo fallback.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import math
from pathlib import Path
import re
import shutil
import sys
import time
import tomllib

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("time_domain_shared", ROOT / "scripts/portable-analysis.py")
COMMON = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(COMMON)
ANALYSIS, PRODUCT, PREPARE = COMMON.ANALYSIS, COMMON.PRODUCT, COMMON.PREPARE
RUNTIME = COMMON.RUNTIME
Incomplete = COMMON.Incomplete
LEGACY_SUBJECTS = {name: f"crates/time-domain/src/{name}.rs"
                   for name in ("coordinate", "era", "exchange", "clock", "rounding")}
LEAP_SUBJECTS = {
    "leap": "crates/time-domain/src/leap.rs",
    "leap_validation": "crates/time-domain/src/leap/validation.rs",
    "leap_interpretation": "crates/time-domain/src/leap/interpretation.rs",
    "leap_bundled": "crates/time-domain/src/leap/bundled.rs",
}
SUBJECTS = {**LEGACY_SUBJECTS, **LEAP_SUBJECTS}
GENERATED_DATA = "crates/time-domain/src/leap/generated.rs"
POLICY = "policy/time-domain-analysis.json"
SCOPE = COMMON.Scope("time-domain", tuple(SUBJECTS.items()), POLICY)
REGISTRY = "registry+https://github.com/rust-lang/crates.io-index"


def registry_blocks(text):
    """Retain original reviewed registry blocks with no dependency-edge rewriting."""
    result = {}
    for raw in text.split("[[package]]")[1:]:
        block = "[[package]]\n" + raw.strip() + "\n"
        item = tomllib.loads(block)["package"][0]
        if "source" not in item:
            continue
        key = (item.get("name"), item.get("version"))
        if (any(type(value) is not str or not value for value in key)
                or key in result or item.get("source") != REGISTRY):
            raise Incomplete("Ambiguous or unreviewed locked registry identity.")
        result[key] = (item, block)
    return result


def resolve_registry_dependency(reference, blocks):
    """Resolve exact Cargo lock references; a bare name must have one version."""
    match = (re.fullmatch(r"([A-Za-z0-9_-]+)(?: ([^\s()]+)(?: \(([^\s()]+)\))?)?", reference)
             if type(reference) is str else None)
    if match is None or match[3] not in (None, REGISTRY):
        raise Incomplete("Malformed or unreviewed registry dependency reference.")
    candidates = [key for key in blocks if key[0] == match[1]
                  and (match[2] is None or key[1] == match[2])]
    if len(candidates) != 1:
        raise Incomplete("Garde locked dependency closure is missing or ambiguous.")
    return candidates[0]


def projected_registry_blocks(selected):
    """Drop only redundant pure-scope labels after proving identical exact edges.

    The standalone closure has one version per name. Qualified references needed
    only by the surrounding GUI graph are therefore redundant here; preserve all
    other block bytes and independently reparse to detect unintended rewrites.
    """
    identities = {(item["name"], item["version"]): (item, block) for item, block in selected.values()}
    result = {}
    for name, (item, block) in selected.items():
        references = item.get("dependencies", [])
        keys = [resolve_registry_dependency(reference, identities) for reference in references]
        if len(set(keys)) != len(keys):
            raise Incomplete("Pure dependency projection may not duplicate an exact dependency edge.")
        dependencies = [key[0] for key in keys]
        projected = dict(item)
        if references != dependencies:
            match = re.search(r"(?m)^[ \t]*dependencies[ \t]*=[ \t]*(\[[\s\S]*?\])", block)
            if match is None:
                raise Incomplete("Pure dependency projection cannot locate its retained edge array.")
            edges = match[1]
            for reference, dependency in zip(references, dependencies, strict=True):
                if reference != dependency:
                    before = json.dumps(reference)
                    if edges.count(before) != 1:
                        raise Incomplete("Pure dependency projection cannot preserve its source block.")
                    edges = edges.replace(before, json.dumps(dependency), 1)
            block = block[:match.start(1)] + edges + block[match.end(1):]
            projected["dependencies"] = dependencies
            if tomllib.loads(block)["package"][0] != projected:
                raise Incomplete("Pure dependency projection changed more than exact edge labels.")
        result[name] = (projected, block)
    return result


def domain_registry_closure(product_text):
    """Select actual registry dependencies of the checked time-domain local owner."""
    product = registry_blocks(product_text)
    local = [item for item in tomllib.loads(product_text)["package"] if item["name"] == "time-domain"]
    if len(local) != 1 or local[0].get("source") or local[0].get("version") != "0.0.0" or local[0].get("dependencies") != ["garde"]:
        raise Incomplete("The actual checked product-local owner is absent.")
    selected, pending = {}, ["garde"]
    while pending:
        key = resolve_registry_dependency(pending.pop(), product)
        name = key[0]
        if name in selected:
            if selected[name][0]["version"] != key[1]:
                raise Incomplete("The pure Garde closure may not select multiple versions of one name.")
            continue
        selected[name] = product[key]
        dependencies = product[key][0].get("dependencies", [])
        if type(dependencies) is not list:
            raise Incomplete("Malformed pure registry dependency list.")
        resolved = [resolve_registry_dependency(reference, product) for reference in dependencies]
        if len(set(resolved)) != len(resolved):
            raise Incomplete("The pure Garde closure contains duplicate exact dependency edges.")
        pending.extend(dependencies)
    return selected


def admitted_domain_products(product_text, policy):
    """Bind only the pure owner's reached nodes to selected source/grant admission.

    Unrelated engine/runtime and metadata-only lock entries are not parity owners
    for the separately admitted original libFuzzer control graph.
    """
    selected = domain_registry_closure(product_text)
    try:
        records = PRODUCT.validated_dependency_records(policy, include_metadata=False)
    except PRODUCT.ENV.Incomplete as error:
        raise Incomplete(str(error)) from error
    admitted = {(item["name"], item["version"]): item for item in records}
    if len(admitted) != len(records):
        raise Incomplete("Selected product source admission contains duplicate identities.")
    result = {}
    for name, (locked, _block) in selected.items():
        key = (name, locked["version"])
        if key not in admitted:
            raise Incomplete("A reached pure dependency is not a selected admitted source.")
        item = admitted[key]
        if any(locked.get(key) != item[key] for key in ("version", "checksum")):
            raise Incomplete("A reached pure dependency differs from its selected source admission.")
        result[name] = item
    return result


def reviewed_fuzz_lock(product_text, control_text):
    """Preserve Garde's reviewed closure and all original libFuzzer registry blocks."""
    selected = projected_registry_blocks(domain_registry_closure(product_text))
    original = registry_blocks(control_text)
    for (name, _version), entry in original.items():
        if name in selected and selected[name][0] != entry[0]:
            raise Incomplete("Shared product/fuzz registry identity or dependency edges disagree.")
        selected[name] = entry
    blocks = {name: entry[1] for name, entry in selected.items()}
    # These are actual new local identities, not a renamed control package.
    blocks["time-domain"] = '[[package]]\nname = "time-domain"\nversion = "0.0.0"\ndependencies = ["garde"]\n'
    blocks["time-domain-fuzz"] = ('[[package]]\nname = "time-domain-fuzz"\nversion = "0.0.0"\n'
                                  'dependencies = ["libfuzzer-sys", "time-domain"]\n')
    return "version = 4\n\n" + "\n".join(blocks[name] for name in sorted(blocks))


def encode_case(*, raw_coordinate=0, raw_lower=0, raw_upper=0, values=(0, 0, 0, 0), bits=0,
                era_lower=0, era_upper=0, clock=(0, 0, 0)):
    """Encode only reviewed fixed-width seed literals; no product formatter is used."""
    signed = (raw_coordinate, raw_lower, raw_upper, *values)
    result = b"".join(value.to_bytes(16, "big", signed=True) for value in signed)
    result += bits.to_bytes(8, "big")
    result += era_lower.to_bytes(16, "big", signed=True) + era_upper.to_bytes(16, "big", signed=True)
    result += b"".join(value.to_bytes(8, "big") for value in clock)
    if len(result) != 176:
        raise Incomplete("Numeric seed framing changed.")
    return result


def fuzz_seeds():
    """Literal independent exact arithmetic/era/rounding/clock neighbors."""
    minimum, maximum, modulus, umax = -(1 << 95), (1 << 95) - 1, 1 << 64, (1 << 64) - 1
    # Literal saved-real-T1 and explicit unfolded coordinates from the retained
    # exchange vectors, rather than merely equivalent translated trajectories.
    base = 0xe000000000000000
    seeds = {
        "ordinary-symmetric": encode_case(values=(base, base + 300647710, base + 343597383, base + 472446402),
                                           bits=base + 300647710, era_lower=base, era_upper=base + (1 << 32)),
        "randomized-wire-token": encode_case(values=(base, base + 300647710, base + 343597383, base + 472446402),
                                              raw_coordinate=0x0102030405060708, bits=base + 300647710,
                                              era_lower=base, era_upper=base + (1 << 32)),
        "asymmetric-path": encode_case(values=(base, base + 214748364, base + 257698037, base + 472446402),
                                       bits=base + 214748364, era_lower=base, era_upper=base + (1 << 32)),
        "era-rollover-explicit": encode_case(values=(0xfffffffff8000000, modulus + 0x08000000,
                                                       modulus + 0x18000000, modulus + 0x28000000),
                                              bits=0x08000000, era_lower=modulus, era_upper=modulus + (1 << 32)),
        "maximum-phase": encode_case(values=(minimum, maximum, maximum, minimum)),
        "maximum-rtt": encode_case(values=(minimum, maximum, minimum, maximum)),
        "negative-rtt": encode_case(values=(0, 0, 2, 1)),
        "era-negative-unique": encode_case(era_lower=-modulus, era_upper=-modulus),
        "era-negative-none": encode_case(era_lower=-modulus + 1, era_upper=-1),
        "era-negative-ambiguous": encode_case(era_lower=-modulus, era_upper=0),
        "era-negative-last-fraction": encode_case(bits=umax, era_lower=-1, era_upper=-1),
        "era-negative-last-ambiguous": encode_case(bits=umax, era_lower=-modulus - 1, era_upper=-1),
        "era-half-shaped": encode_case(bits=modulus // 2, era_lower=-modulus // 2, era_upper=modulus // 2),
        "era-positive-unique": encode_case(era_lower=1, era_upper=modulus),
        "era-positive-none": encode_case(era_lower=1, era_upper=modulus - 1),
        "era-wide": encode_case(era_lower=minimum, era_upper=maximum),
        "window-inverted": encode_case(raw_lower=1, raw_upper=0),
        "window-invalid-lower": encode_case(raw_lower=minimum - 1),
        "window-invalid-upper": encode_case(raw_upper=maximum + 1),
        "clock-add-overflow": encode_case(clock=(0, 1, umax)),
        "clock-maximum-duration": encode_case(clock=(0, umax, 0)),
        "clock-reversed": encode_case(clock=(umax, 0, 0)),
        "clock-zero-at-maximum": encode_case(clock=(umax, umax, umax)),
        "clock-exact-add-maximum": encode_case(clock=(0, 1, umax - 1)),
    }
    for name, value in [("below-minimum", minimum - 1), ("minimum", minimum),
                        ("maximum", maximum), ("above-maximum", maximum + 1),
                        ("i128-minimum", -(1 << 127)), ("i128-maximum", (1 << 127) - 1)]:
        seeds["coordinate-" + name] = encode_case(raw_coordinate=value)
    for prefix, first, third, upper, lower in [
            ("phase", 8388608, 25165824, 79228162514264337589, -79228162514264337597),
            ("rtt", 4194304, 12582912, 39614081257132168794, -39614081257132168798)]:
        for name, numerator in [("tie-positive", first), ("tie-negative", -first),
                                ("tie-odd-positive", third), ("tie-odd-negative", -third),
                                ("below-tie", first - 1), ("above-tie", first + 1),
                                ("negative-below-tie", -first - 1), ("negative-above-tie", -first + 1),
                                ("i64-upper", upper), ("i64-upper-rejected", upper + 1),
                                ("i64-lower", lower), ("i64-lower-rejected", lower - 1),
                                ("zero", 0), ("small-negative", -1)]:
            values = (0, numerator, 0, 0) if prefix == "phase" else (0, 0, 0, numerator)
            seeds[prefix + "-" + name] = encode_case(values=values)
    return seeds


def fuzz_metadata(metadata, selected):
    """Reject wrong identities, feature sets and complete reachable dependency edges."""
    packages = {item["id"]: item for item in metadata["packages"]}
    nodes = {item["id"]: item for item in metadata["resolve"]["nodes"]}
    if (len(packages) != len(metadata["packages"]) or len(nodes) != len(metadata["resolve"]["nodes"])
            or len({item["name"] for item in packages.values()}) != len(packages)):
        raise Incomplete("Duplicate fuzz graph identity.")
    locked = {item["name"]: item for item in selected["locked_registry_packages"]}
    if len(locked) != len(selected["locked_registry_packages"]):
        raise Incomplete("Duplicate locked registry admission.")
    local = {"time-domain", "time-domain-fuzz"}
    for item in packages.values():
        expected = locked.get(item["name"])
        if expected:
            if item.get("source") != expected["source"] or item["version"] != expected["version"]:
                raise Incomplete("Fuzz metadata package source/version changed.")
        elif item["name"] not in local or item.get("source") is not None or item["version"] != "0.0.0":
            raise Incomplete("Unreviewed fuzz metadata package.")
    root_id = metadata["resolve"]["root"]
    if root_id not in packages or packages[root_id]["name"] != "time-domain-fuzz":
        raise Incomplete("Fuzz metadata identifies a different root.")
    reachable, pending = set(), [root_id]
    while pending:
        key = pending.pop()
        if key in reachable:
            continue
        if key not in packages or key not in nodes:
            raise Incomplete("Unresolved reachable fuzz dependency.")
        reachable.add(key)
        pending.extend(item["pkg"] for item in nodes[key]["deps"])
    features = {packages[key]["name"]: sorted(nodes[key]["features"]) for key in reachable}
    edges = {packages[key]["name"]: sorted(packages[edge["pkg"]]["name"] for edge in nodes[key]["deps"])
             for key in reachable}
    expected_features = {**selected["gnu_reachable_features"], "time-domain": [], "time-domain-fuzz": []}
    if len(features) != len(reachable) or features != expected_features or edges != selected["gnu_dependency_edges"]:
        raise Incomplete("Exact GNU fuzz features or dependency edges differ from admission.")
    return {"target": ANALYSIS.PLATFORM, "reachable_features": features, "dependency_edges": edges}


def read_policy(root):
    """Fixed thresholds and complete source/manifest/graph admission precede execution."""
    policy = json.loads((root / POLICY).read_text())
    required = {"schema_version": 4, "package": "time-domain", "critical_files": list(SUBJECTS.values()),
                "source_file_count": 166, "tool_admission_profile": "stable-core",
                "mutation_jobs": 10, "mutation_process_limit": 512, "mutation_message_format": "json",
                "mutation_equivalences": [],
                "mutation_error_values": {"leap_bundled": [
                    "{ let _ = (CONTINUOUS_FROM, DATA_REVISION, FINITE_FUTURE_BEFORE, "
                    "HISTORY_COMPLETE_BEFORE, LEAP_RECORDS, PROVENANCE_SHA256, RAW_SHA256, "
                    "LeapContinuousFrom::new, LeapDataIdentity::new, LeapFutureExpiry::new, "
                    "LeapHistoryBefore::new); LeapError::InvalidRecords }"]},
                "coverage_minimum_line_percent": 90, "mutation_minimum_viable_killed_percent": 95,
                "maximum_mutants": 512, "coverage_wall_seconds": 300, "mutation_wall_seconds": 1200,
                "mutation_build_seconds": 90, "mutation_test_seconds": 60, "contracts_wall_seconds": 300,
                "jobs": 6, "test_threads": RUNTIME.MAX_JOBS}
    if (type(policy) is not dict or set(policy) != {*required, "coverage_guided_fuzzing"}
            or SCOPE.subjects != COMMON.DOMAIN_LEAP_SUBJECTS
            or any(type(policy.get(key)) is not type(value) or policy.get(key) != value
                   for key, value in required.items())):
        raise Incomplete("Unreviewed time-domain analysis scope, threshold or budget.")
    COMMON.admit_mutation_equivalences(root, SCOPE, policy)
    inventory = tomllib.loads((root / "policy/verification-manifest.toml").read_text())
    for filename in SUBJECTS.values():
        if (inventory.get("critical_files", []).count(filename) != 1
                or sum(module.get("package") == "time-domain" and filename in module.get("files", [])
                       for module in inventory.get("security_modules", [])) != 1):
            raise Incomplete("A numeric owner is absent/duplicated in the security/critical inventory.")
    if (GENERATED_DATA in inventory.get("critical_files", [])
            or any(GENERATED_DATA in module.get("files", []) for module in inventory.get("security_modules", []))):
        raise Incomplete("Constant-only generated leap data has no executable denominator.")
    selected = policy["coverage_guided_fuzzing"]
    if type(selected) is not dict or selected.get("subjects") != LEGACY_SUBJECTS:
        raise Incomplete("Legacy numeric fuzzing must identify exactly its original five owners.")
    if selected.get("status") != "admitted-test-only-source" or selected.get("fuzzer_flags") != list(COMMON.RUNTIME.SANITIZER_FLAGS):
        raise Incomplete("Exact finite sanitizer profile admission is missing.")
    paths = {"manifest": "policy/time-domain-fuzz-manifest.toml", "lockfile": "policy/time-domain-fuzz.lock",
             "target_source": "policy/time-domain-fuzz-target.rs"}
    expected_inputs = {*paths.values(), "policy/ui-m0-dependencies.json", "policy/ntp-wire-analysis.json",
                       "policy/dependency-notices/libfuzzer-sys-0.4.13-LLVM-LICENSE.txt", "tasks/s1.1-exchange-vectors.json"}
    if any(selected.get(key) != value for key, value in paths.items()) or set(selected["inputs_sha256"]) != expected_inputs:
        raise Incomplete("The exact complete fuzz source/graph/rights input inventory differs.")
    for name, digest in selected["inputs_sha256"].items():
        if PRODUCT.sha256(root / name) != digest:
            raise Incomplete("Fuzz source, literal graph, selected grants or manifest changed.")
    lock = tomllib.loads((root / selected["lockfile"]).read_text())["package"]
    registry = [item for item in lock if "source" in item]
    if registry != selected["locked_registry_packages"] or len(registry) != 23:
        raise Incomplete("The exact standalone 23-registry graph differs from admission.")
    locals_ = [item for item in lock if "source" not in item]
    expected_local = [{"name": "time-domain", "version": "0.0.0", "dependencies": ["garde"]},
                      {"name": "time-domain-fuzz", "version": "0.0.0", "dependencies": ["libfuzzer-sys", "time-domain"]}]
    if locals_ != expected_local:
        raise Incomplete("The standalone graph's actual local owners differ.")
    if not selected.get("gnu_reachable_features") or not selected.get("gnu_dependency_edges"):
        raise Incomplete("Resolved source/feature admission has not been supplied.")
    return policy


def contract_examples():
    """Precise external legal, private-construction/mutation and clock-unit cases."""
    setup = ('let utc = UtcCoordinateQ32::new(0)?; '
             'let exact = exchange_arithmetic(utc, utc, utc, utc)?; '
             'let mono = MonotonicNs::new(1).duration_since(MonotonicNs::new(0))?; '
             'let awake = ActiveAwakeNs::new(1).duration_since(ActiveAwakeNs::new(0))?; '
             'let phase = exact.offset().round_nanoseconds_ties_even()?; '
             'let rtt = exact.round_trip().round_nanoseconds_ties_even()?; '
             'let _ = (&utc, &exact, &mono, &awake, &phase, &rtt);')
    legal = ('let window = EraWindow::new(0, 0)?; assert_eq!(unfold_ntp(0, 0, &window)?.ticks(), 0); '
             'assert_eq!(exact.offset().numerator(), 0); assert_eq!(exact.round_trip().numerator(), 0); '
             'assert_eq!(MonotonicNs::new(0).checked_add(mono)?.nanoseconds(), 1); '
             'assert_eq!(ActiveAwakeNs::new(0).checked_add(awake)?.nanoseconds(), 1); '
             'assert_eq!(phase.nanoseconds(), 0); assert_eq!(rtt.nanoseconds(), 0); '
             'assert_eq!(phase.residual_denominator(), 1_i128 << 33); '
             'assert_eq!(rtt.residual_denominator(), 1_i128 << 32); '
             'println!("legal-time-domain-contracts:11");')
    examples = {"legal": {"statement": legal, "code": None, "tokens": []}}
    representations = [
        ("coordinate", "UtcCoordinateQ32", "ticks", 'UtcCoordinateQ32 { ticks: 0 }', 'UtcCoordinateQ32::new(0)?', '0'),
        ("window", "EraWindow", "earliest", 'EraWindow { earliest: utc, latest: utc }', 'EraWindow::new(0, 0)?', 'utc'),
        ("monotonic", "MonotonicNs", "nanoseconds", 'MonotonicNs { nanoseconds: 0 }', 'MonotonicNs::new(0)', '0'),
        ("awake", "ActiveAwakeNs", "nanoseconds", 'ActiveAwakeNs { nanoseconds: 0 }', 'ActiveAwakeNs::new(0)', '0'),
        ("mono_duration", "MonotonicDurationNs", "nanoseconds", 'MonotonicDurationNs { nanoseconds: 0 }', 'mono', '0'),
        ("awake_duration", "ActiveAwakeDurationNs", "nanoseconds", 'ActiveAwakeDurationNs { nanoseconds: 0 }', 'awake', '0'),
        ("phase", "PhaseOffsetQ33", "numerator", 'PhaseOffsetQ33 { numerator: 0 }', 'exact.offset()', '0'),
        ("rtt", "SignedRoundTripQ32", "numerator", 'SignedRoundTripQ32 { numerator: 0 }', 'exact.round_trip()', '0'),
        ("exchange", "ExchangeArithmetic", "offset", 'ExchangeArithmetic { offset: exact.offset(), round_trip: exact.round_trip() }', 'exact', 'exact.offset()'),
        ("rounded_phase", "RoundedPhaseOffsetNs", "residual_numerator", 'RoundedPhaseOffsetNs { nanoseconds: 0, residual_numerator: 0 }', 'phase', '0'),
        ("rounded_rtt", "RoundedRoundTripNs", "residual_numerator", 'RoundedRoundTripNs { nanoseconds: 0, residual_numerator: 0 }', 'rtt', '0'),
    ]
    for name, typename, field, constructor, value, replacement in representations:
        examples[name + "_construction"] = {"statement": 'let _ = ' + constructor + ';', "code": "E0451", "tokens": [typename, field]}
        examples[name + "_mutation"] = {"statement": f'let mut value = {value}; value.{field} = {replacement};', "code": "E0616", "tokens": [typename, field]}
        examples[name + "_default"] = {"statement": f'let _: {typename} = Default::default();', "code": "E0277", "tokens": [typename, "Default"]}
    mixed = [
        ("monotonic_awake", 'let _ = MonotonicNs::new(0).duration_since(ActiveAwakeNs::new(0));', ["MonotonicNs", "ActiveAwakeNs"]),
        ("awake_monotonic", 'let _ = ActiveAwakeNs::new(0).duration_since(MonotonicNs::new(0));', ["ActiveAwakeNs", "MonotonicNs"]),
        ("utc_monotonic", 'let _ = MonotonicNs::new(0).duration_since(utc);', ["MonotonicNs", "UtcCoordinateQ32"]),
        ("utc_awake", 'let _ = ActiveAwakeNs::new(0).duration_since(utc);', ["ActiveAwakeNs", "UtcCoordinateQ32"]),
        ("monotonic_awake_duration", 'let _ = MonotonicNs::new(0).checked_add(awake);', ["MonotonicDurationNs", "ActiveAwakeDurationNs"]),
        ("awake_monotonic_duration", 'let _ = ActiveAwakeNs::new(0).checked_add(mono);', ["ActiveAwakeDurationNs", "MonotonicDurationNs"]),
        ("raw_exchange", 'let _ = exchange_arithmetic(0, 1, 2, 3);', ["UtcCoordinateQ32", "integer"]),
        ("rounded_units", 'let _: RoundedPhaseOffsetNs = rtt;', ["RoundedPhaseOffsetNs", "RoundedRoundTripNs"]),
    ]
    for name, statement, tokens in mixed:
        examples[name] = {"statement": statement, "code": "E0308", "tokens": tokens}
    for name, typename in [("coordinate_from", "UtcCoordinateQ32"), ("phase_from", "PhaseOffsetQ33"),
                           ("rtt_from", "SignedRoundTripQ32"), ("window_from", "EraWindow")]:
        examples[name] = {"statement": f'let _: {typename} = 0_i128.into();', "code": "E0277", "tokens": [typename, "i128"]}
    for item in examples.values():
        item["source"] = '\n'.join(['use time_domain::*;', 'fn main() -> Result<(), DomainError> {', setup,
                                    item.pop("statement"), 'Ok(())', '}']) + '\n'
        item["line"] = 4
    examples["legal"]["success_marker"] = "legal-time-domain-contracts:11"
    examples.update(leap_contract_examples())
    return examples


def leap_contract_examples():
    """External checked leap consumers; every rejection has one owned cause line.

    The independent legal consumer reaches constructors, bundled input and the
    complete-interval interpretation. Boxed errors permit both existing domain
    and leap errors without an unrelated conversion diagnostic.
    """
    setup = (
        "let records = [(2_272_060_800_u64, 10_i32)]; "
        "let history_before = LeapHistoryBefore::new(2_272_060_802)?; "
        "let expiry = LeapFutureExpiry::new(2_272_060_803)?; "
        "let continuous = LeapContinuousFrom::new(2_272_060_803)?; "
        "let identity = LeapDataIdentity::new(1, [1; 32], [2; 32])?; "
        "let dates = EraWindow::new(2_272_060_800_i128 << 32, 2_272_060_800_i128 << 32)?; "
        "let scope = LeapDateScope::new(dates)?; "
        "let knowledge = LeapKnowledge::new(&records, history_before, Some(expiry), Some(continuous), identity)?; "
        "let answer = knowledge.interpret(dates, scope)?; "
        "let _ = (&records, history_before, expiry, continuous, identity, dates, scope, &knowledge, answer);"
    )
    marker = "legal-leap-contracts:checked-history-and-whole-interval"
    legal = (
        "assert_eq!(knowledge.records(), &[(2_272_060_800, 10)]); "
        "assert_eq!(knowledge.history_before().ntp_seconds(), 2_272_060_802); "
        "assert_eq!(knowledge.expiry(), Some(expiry)); "
        "assert_eq!(knowledge.continuous_from(), Some(continuous)); "
        "assert_eq!(knowledge.identity(), identity); "
        "assert_eq!(identity.revision(), 1); "
        "assert_eq!(identity.raw_source_sha256(), [1; 32]); "
        "assert_eq!(identity.provenance_sha256(), [2; 32]); "
        "assert_eq!(scope.dates(), dates); "
        "assert_eq!(answer.dates(), dates); "
        "assert_eq!(answer.identity(), identity); "
        "assert_eq!(answer.offset(), LeapOffsetKnowledge::Known(10)); "
        "assert_eq!(answer.events(), LeapEventKnowledge::NoEvents); "
        "assert_eq!(answer.known_transition_count(), 0); "
        "let bundled = bundled_leap_knowledge()?; "
        "assert_eq!(bundled.records().last(), Some(&(3_692_217_600, 37))); "
        'println!("' + marker + '");'
    )
    examples = {"leap_legal": {"statement": legal, "code": None, "tokens": [], "success_marker": marker}}
    representations = [
        ("leap_history", "LeapHistoryBefore", "second", "history_before", "0_u64"),
        ("leap_expiry", "LeapFutureExpiry", "second", "expiry", "0_u64"),
        ("leap_continuous", "LeapContinuousFrom", "second", "continuous", "0_u64"),
        ("leap_identity", "LeapDataIdentity", "revision", "identity", "(0_u64, [1_u8; 32], [2_u8; 32])"),
        ("leap_scope", "LeapDateScope", "dates", "scope", "dates"),
        ("leap_knowledge", "LeapKnowledge<'_>", "records", "knowledge", "()"),
        ("leap_answer", "LeapInterpretation", "offset", "answer", "()"),
    ]
    for name, typename, field, value, raw in representations:
        literal_type = typename.split("<", 1)[0]
        examples[name + "_construction"] = {
            "statement": f"let _ = {literal_type} {{ ..{value} }};",
            "code": "E0451", "tokens": ["private", field]}
        examples[name + "_mutation"] = {
            "statement": f"let mut value = {value}; value.{field} = value.{field};",
            "code": "E0616", "tokens": ["private", field]}
        examples[name + "_default"] = {
            "statement": f"let _: {typename} = Default::default();",
            "code": "E0277", "tokens": [literal_type, "Default"]}
        examples[name + "_from"] = {
            "statement": f"let _: {typename} = {raw}.into();",
            "code": "E0277", "tokens": [literal_type, "From"]}
    examples["leap_metadata_units"] = {
        "statement": "let _ = LeapKnowledge::new(&records, expiry, None, None, identity);",
        "code": "E0308", "tokens": ["LeapHistoryBefore", "LeapFutureExpiry"]}
    examples["leap_records_borrow_mutation"] = {
        "statement": "records[0].1 = 11; let _ = knowledge.records(); assert_eq!(records[0].1, 11);",
        "code": "E0506", "tokens": ["records", "borrowed"]}
    for name, specimen in examples.items():
        owned_setup = setup.replace("let records =", "let mut records =", 1) if name == "leap_records_borrow_mutation" else setup
        specimen["source"] = "\n".join([
            "use time_domain::*;", "fn main() -> Result<(), Box<dyn std::error::Error>> {",
            owned_setup, specimen.pop("statement"), "Ok(())", "}"]) + "\n"
        specimen["line"] = 4
    return examples


def copy_contract_dependencies(session, artifact):
    """Copy actual emitted dependency artifacts with no hardlink or mutable alias."""
    destination = session.work / "contract-deps"
    destination.mkdir(mode=0o700)
    source = session.work / artifact.removeprefix("/work/")
    actual = source.resolve(strict=True)
    if actual != source or not actual.is_file() or not source.is_relative_to(session.work / "target"):
        raise Incomplete("Actual emitted contract library has redirected or escaped.")
    dependencies = session.work / "target/debug/deps"
    for item in sorted(dependencies.iterdir()):
        if item.suffix not in {".rlib", ".rmeta", ".so"}:
            continue
        digest = PRODUCT.sha256(item)
        target = destination / item.name
        shutil.copyfile(item, target); target.chmod(0o400)
        if PRODUCT.sha256(target) != digest:
            raise Incomplete("Contract dependency changed while its immutable copy was prepared.")
    library = destination / "libtime_domain.rlib"
    digest = PRODUCT.sha256(source)
    shutil.copyfile(source, library); library.chmod(0o400)
    if PRODUCT.sha256(library) != digest:
        raise Incomplete("The copied contract library differs from Cargo's actual artifact.")
    session.protect(destination)
    return destination


def contracts(session, policy, report):
    """Build the real dependency-bearing library, then inspect external compiler causes."""
    deadline = time.monotonic() + policy["contracts_wall_seconds"]
    run = session.run
    output = run("contract-library", [str(session.stable / "bin/cargo"), "build", "--package", "time-domain",
                 "--frozen", "--jobs", str(policy["jobs"]), "--lib", "--message-format=json"], COMMON.remaining_timeout(deadline, 120))
    artifact = COMMON.library_artifact(output, "time-domain", "/work/source/crates/time-domain/Cargo.toml")
    dependency_copy = copy_contract_dependencies(session, artifact)
    consumers = session.work / "contract-inputs"; consumers.mkdir(mode=0o700)
    (session.work / "contracts").mkdir(mode=0o700)
    examples = contract_examples()
    for name, specimen in examples.items():
        (consumers / (name + ".rs")).write_text(specimen["source"])
    session.protect(consumers)
    for name, specimen in examples.items():
        command = [str(session.stable / "bin/rustc"), "--edition", "2024", "--crate-name", name,
                   "--error-format=json", "/work/contract-inputs/" + name + ".rs",
                   "--extern", "time_domain=/work/contract-deps/libtime_domain.rlib",
                   "-L", "dependency=/work/contract-deps", "--out-dir", "/work/contracts"]
        run("external-consumer-" + name, command, COMMON.remaining_timeout(deadline, 60),
            sealed=[consumers, dependency_copy], specimen=specimen if specimen["code"] else None)
        if specimen["code"] is None:
            label = "external-legal-execution" if name == "legal" else "external-legal-execution-" + name
            text = run(label, ["/work/contracts/" + name], COMMON.remaining_timeout(deadline, 30),
                       sealed=[consumers, dependency_copy])
            if specimen["success_marker"] not in text:
                raise Incomplete("Legal external consumer did not reach every selected representation.")
    report["contracts"] = {"legal_consumers_compiled_and_executed": sum(item["code"] is None for item in examples.values()),
                           "library_artifact": artifact,
                           "immutable_dependencies_sha256": PRODUCT.inventory(dependency_copy),
                           "required_diagnostics": {name: {key: item[key] for key in ("code", "tokens", "line")}
                                                    for name, item in examples.items() if item["code"]}}


def admit_fuzz_sources(session, selected):
    """Reconcile original native/control sources, product maps/grants and exact lock."""
    original = json.loads((ROOT / "policy/ntp-wire-analysis.json").read_text())["coverage_guided_fuzzing"]
    if PRODUCT.sha256(ROOT / original["llvm_notice"]) != original["llvm_notice_sha256"]:
        raise Incomplete("The complete selected native LLVM-exception notice changed.")
    control = session.base / "common/controls-source/fuzz/Cargo.lock"
    if PRODUCT.sha256(control) != original["control_lock_sha256"]:
        raise Incomplete("Original libFuzzer control graph changed.")
    product_text = (ROOT / "Cargo.lock").read_text()
    actual = reviewed_fuzz_lock(product_text, control.read_text())
    if actual.encode() != (ROOT / selected["lockfile"]).read_bytes():
        raise Incomplete("Standalone graph no longer preserves the original reviewed package blocks.")
    products = admitted_domain_products(product_text, session.dependency_policy)
    old = {item["name"]: item for item in original["locked_registry_packages"]}
    if len(old) != 10 or len(original["locked_registry_packages"]) != 10:
        raise Incomplete("Original control source cohort changed.")
    if PRODUCT.admit_vendor(session.vendor, session.dependency_policy) != session.vendor_identity:
        raise Incomplete("Product vendor changed before fuzz dependency publication.")
    grants = selected["selected_grants"]
    if set(grants) != {item["name"] for item in selected["locked_registry_packages"]}:
        raise Incomplete("Selected grant inventory is incomplete or has unexpected entries.")
    for item in selected["locked_registry_packages"]:
        name = item["name"]
        if name not in products and name not in old:
            raise Incomplete("An unreviewed registry source entered the standalone graph.")
        if name in products:
            admitted = products[name]
            if (item["checksum"] != admitted["checksum"] or item["version"] != admitted["version"]
                    or grants[name] != admitted["selected_license"]):
                raise Incomplete("Garde product graph/grant parity changed.")
        if name not in old:
            continue
        admitted = old[name]
        if any(item[key] != admitted[key] for key in ("name", "version", "source", "checksum")) or grants[name] != admitted["selected_license"]:
            raise Incomplete("Original fuzz identity/grant parity changed.")
        directory_name = name + "-" + item["version"]
        sources = list((session.base / "common/cargo/registry/src").glob("*/" + directory_name))
        archives = list((session.base / "common/cargo/registry/cache").glob("*/" + directory_name + ".crate"))
        if len(sources) != 1 or len(archives) != 1 or PRODUCT.sha256(archives[0]) != item["checksum"]:
            raise Incomplete("Original fuzz source/archive is missing or ambiguous.")
        inventory = PRODUCT.inventory(sources[0])
        digest = hashlib.sha256(json.dumps(inventory, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
        manifest = tomllib.loads((sources[0] / "Cargo.toml").read_text())["package"]
        if (digest != admitted["source_files_sha256"] or manifest.get("license") != admitted["declared_license"]
                or manifest.get("build", False) != admitted["build_script"]):
            raise Incomplete("Original fuzz source/build/grant identity changed.")
        if name in products:
            files = {key: value for key, value in inventory.items() if key not in {".cargo-ok", ".cargo-checksum.json"}}
            if PREPARE.files_digest(files) != products[name]["files_sha256"]:
                raise Incomplete("Shared Garde/fuzz source maps disagree.")
        if name == "libfuzzer-sys":
            if PRODUCT.inventory(sources[0] / "libfuzzer") != {key: value["sha256"] for key, value in original["native_files"].items()}:
                raise Incomplete("Pinned complete native libFuzzer source changed.")
    return original


def fuzz(session, selected, report):
    """Admit before building; reuse one target cache and seal distinct ASan binaries."""
    original = admit_fuzz_sources(session, selected)
    project = session.work / "fuzz"; project.mkdir(mode=0o700)
    for origin, destination in [(selected["manifest"], "Cargo.toml"), (selected["lockfile"], "Cargo.lock"),
                                (selected["target_source"], "target.rs")]:
        shutil.copyfile(ROOT / origin, project / destination)
    vendor = session.work / "fuzz-vendor"; vendor.mkdir(mode=0o700)
    old = {item["name"]: item for item in original["locked_registry_packages"]}
    for item in selected["locked_registry_packages"]:
        name = item["name"] + "-" + item["version"]
        destination = vendor / name
        if item["name"] in old:
            source = next((session.base / "common/cargo/registry/src").glob("*/" + name))
            archive = next((session.base / "common/cargo/registry/cache").glob("*/" + name + ".crate"))
            COMMON.prepare_fuzz_vendor(source, archive, old[item["name"]], destination)
        else:
            source = session.vendor / name
            shutil.copytree(source, destination, copy_function=shutil.copyfile)
            if PRODUCT.inventory(source) != PRODUCT.inventory(destination):
                raise Incomplete("Reviewed product dependency changed while copied for fuzzing.")
            copied = PRODUCT.inventory(destination)
            checksum = json.loads((destination / ".cargo-checksum.json").read_text())
            copied.pop(".cargo-checksum.json")
            admitted = next(entry for entry in session.dependency_policy["packages"]
                            if (entry["name"], entry["version"]) == (item["name"], item["version"]))
            if (checksum.get("package") != item["checksum"] or checksum.get("files") != copied
                    or PREPARE.files_digest(copied) != admitted["files_sha256"]):
                raise Incomplete("Copied product dependency no longer matches its admitted archive map.")
    session.configure(COMMON.FUZZ_CARGO_CONFIG)
    sealed = [project, vendor]
    before = COMMON.fuzz_input_snapshot(session.work, sealed)
    session.protect(project); session.protect(vendor)
    night = session.base / session.admitted["policy"]["common_nightly"]
    tools = session.output / "fuzz-tools"; tools.mkdir(mode=0o700)
    launcher = tools / "cargo"
    launcher.write_text('#!/usr/bin/python3\nimport os,sys\n'
                        'if len(sys.argv)<2 or sys.argv[1] not in ("build","check","rustc","metadata"):\n'
                        ' raise SystemExit("unreviewed fuzz Cargo subcommand")\n'
                        f'os.execv({str(night / "bin/cargo")!r}, [{str(night / "bin/cargo")!r},sys.argv[1],"--frozen",*sys.argv[2:]])\n')
    launcher.chmod(0o500)
    shutil.copyfile(session.base / "common/bin/cargo-fuzz", tools / "cargo-fuzz")
    (tools / "cargo-fuzz").chmod(0o500)
    if PRODUCT.sha256(tools / "cargo-fuzz") != session.admitted["policy"]["tools"]["fuzz"]["sha256"]:
        raise Incomplete("The copied cargo-fuzz executable differs from admitted tool bytes.")
    session.protected_tools[tools] = PRODUCT.inventory(tools)
    metadata = session.run("fuzz-graph", [str(night / "bin/cargo"), "metadata", "--frozen", "--format-version", "1",
                           "--filter-platform", ANALYSIS.PLATFORM, "--manifest-path", "/work/fuzz/Cargo.toml"],
                           120, compiler=night, extra_tools=tools, sealed=sealed, cwd=project)
    report["fuzz_graph"] = fuzz_metadata(json.loads(metadata), selected)
    corpus = session.work / "fuzz-corpus"; corpus.mkdir(mode=0o700)
    seeds = fuzz_seeds()
    for name, data in seeds.items():
        (corpus / name).write_bytes(data)
    seed_identity = PRODUCT.inventory(corpus)
    (session.work / "fuzz-artifacts").mkdir(mode=0o700)
    binaries = session.work / "fuzz-binaries"; binaries.mkdir(mode=0o700)
    retained = {}
    for case in ("healthy", "defect"):
        command = [str(tools / "cargo-fuzz"), "fuzz", "build", "numeric", "--fuzz-dir", "/work/fuzz",
                   "--sanitizer", "address", "--target", ANALYSIS.PLATFORM, "--target-dir", "/work/target-fuzz"]
        if case == "defect":
            command.extend(["--features", "seeded-defect"])
        session.run("fuzz-build-" + case, command, 600, compiler=night, extra_tools=tools,
                    sealed=[*sealed, *retained], cwd=project)
        built = session.work / "target-fuzz" / ANALYSIS.PLATFORM / "release/numeric"
        digest = PRODUCT.sha256(built)
        if digest in retained.values():
            raise Incomplete("Healthy and seeded numeric builds emitted the same executable.")
        binary = binaries / case
        shutil.copyfile(built, binary); binary.chmod(0o500)
        if PRODUCT.sha256(binary) != digest:
            raise Incomplete("Immutable numeric executable differs from its real build.")
        retained[binary] = digest
        session.protect(binaries)
        report.setdefault("fuzz_binaries_sha256", {})[case] = digest
        report.setdefault("fuzz_binary_paths", {})[case] = str(binary)
        text = session.run("fuzz-" + case, ["/work/fuzz-binaries/" + case, "/work/fuzz-corpus",
                           "-artifact_prefix=/work/fuzz-artifacts/", *COMMON.RUNTIME.SANITIZER_FLAGS],
                           90, compiler=night, extra_tools=tools, sealed=[*sealed, *retained], cwd=project,
                           mode="sanitizer", negative="independent exact phase-offset oracle" if case == "defect" else None)
        if case == "healthy":
            count = re.search(r"Done (\d+) runs", text)
            if not count or int(count[1]) != 256:
                raise Incomplete("Healthy numeric fuzz target did not complete its finite 256 runs.")
            report["fuzz"] = {"observed_runs": 256, "subjects": dict(LEGACY_SUBJECTS),
                              "not_fuzzed": dict(LEAP_SUBJECTS), "status": "legacy-five-owner-only",
                              "seed_files_sha256": seed_identity,
                              "seed_lengths": [176], "seeded_fault": "pending", "sanitizer": "address", "target": ANALYSIS.PLATFORM}
        else:
            report["fuzz"]["seeded_fault"] = "observed intended exact phase-offset assertion failure"
        if any(PRODUCT.sha256(path) != expected for path, expected in retained.items()):
            raise Incomplete("Retained ASan executable changed.")
        if COMMON.fuzz_input_snapshot(session.work, sealed) != before:
            raise Incomplete("Fuzz manifest/lock/source/vendor/config inventory changed.")
    admit_fuzz_sources(session, selected)
    report["fuzz"]["fault_artifacts_sha256"] = PRODUCT.inventory(session.work / "fuzz-artifacts")


# A new exact singleton uses full tool custody; legacy leap/fuzz roles stay fixed.
RATIONAL_SECONDS_SUBJECT = "rational_seconds"
RATIONAL_SECONDS_POLICY = "policy/rational-seconds-analysis.json"
RATIONAL_SECONDS_FILE = "crates/time-domain/src/rational_seconds.rs"
RATIONAL_SECONDS_TEST_FILE = "crates/time-domain/tests/rational_seconds.rs"
RATIONAL_SECONDS_PRIVATE_TEST_FILE = "crates/time-domain/tests/private/rational_seconds.rs"
RATIONAL_SECONDS_SCOPE = COMMON.Scope(
    "time-domain", ((RATIONAL_SECONDS_SUBJECT, RATIONAL_SECONDS_FILE),), RATIONAL_SECONDS_POLICY)
RATIONAL_SECONDS_BOUND_INPUTS = (
    "Cargo.toml", "Cargo.lock", "crates/time-domain/Cargo.toml", "crates/time-domain/src/lib.rs",
    RATIONAL_SECONDS_FILE, RATIONAL_SECONDS_TEST_FILE, RATIONAL_SECONDS_PRIVATE_TEST_FILE,
    "scripts/verify-time-domain-analysis.py", "scripts/tests/test_rational_seconds_analysis.py",
    "scripts/portable-analysis.py", "scripts/analysis-runtime.py", "scripts/check-analysis.py",
    "scripts/verify.py", "scripts/check-environment.py", "scripts/check-strict.py",
    "scripts/prepare-product.py", "policy/analysis-tools.json", "policy/verification-manifest.toml",
)
RATIONAL_SECONDS_FIXED = {
    "schema_version": 1, "package": "time-domain", "source_file_count": 374,
    "critical_files": [RATIONAL_SECONDS_FILE], "tool_admission_profile": "full",
    "coverage_minimum_line_percent": 90, "mutation_minimum_viable_killed_percent": 95,
    "coverage_wall_seconds": 300, "enumeration_wall_seconds": 240,
    "mutation_wall_seconds": 1200, "mutation_build_seconds": 90, "mutation_test_seconds": 60,
    "jobs": 6, "test_threads": 1, "coverage_test_threads": 1,
    "mutation_jobs": 10, "mutation_process_limit": 512, "mutation_message_format": "json",
    "maximum_mutants": 512, "maximum_listed_per_file": 4096, "maximum_listed_total": 8192,
}
RATIONAL_SECONDS_REQUIRED_TESTS = frozenset({
    "rational_seconds::tests::transformed_seconds_faults_have_valid_scalar_neighbors",
    "rational_seconds::tests::transformed_bounds_require_power_and_exactness_association",
    "rational_seconds::tests::raw_roundoff_rule_rejects_invalid_normals_without_publication_guards",
    "rational_seconds::tests::zero_publication_rejects_noncanonical_associations",
    "rational_seconds::tests::checked_helper_faults_do_not_use_truncated_shifts",
    "rational_seconds::tests::diagnostics_omit_numeric_payloads_and_use_fixed_errors",
    "rational_seconds::tests::raw_unsigned_magnitude_is_admitted_before_sign_application",
    "rounds_exact_ratio_once_without_operand_double_rounding",
    "literal_ratios_cover_full_integer_envelope_and_tie_parities",
    "adjacent_rational_neighbors_preserve_ties_and_original_binade_bound",
    "equivalent_unreduced_ratios_retain_original_tuples",
    "engine_denominator_literals_preserve_seconds_without_nanosecond_rounding",
    "counted_binary_scalings_span_full_unsigned_shift_domain",
})
RATIONAL_SECONDS_REQUIRED_DOCTESTS = (
    ("rational_seconds::RoundedRationalSeconds", False, 1),
    ("rational_seconds::RoundedRationalSeconds", True, 4),
)


def rational_seconds_document(path, *, maximum=512 * 1024):
    """Decode bounded canonical regular input without duplicate/nonfinite fields."""
    if path != path.resolve(strict=True):
        raise Incomplete("Rational-seconds JSON cannot use a linked path.")
    raw = ANALYSIS.read_once(path, maximum=maximum)

    def pairs(items):
        result = {}
        for name, value in items:
            if name in result:
                raise Incomplete("Rational-seconds JSON contains a duplicate field.")
            result[name] = value
        return result

    def reject_constant(_token):
        raise Incomplete("Rational-seconds JSON contains a nonfinite number.")

    def finite_float(token):
        value = float(token)
        if not math.isfinite(value):
            reject_constant(token)
        return value

    return json.loads(raw, object_pairs_hook=pairs, parse_constant=reject_constant,
                      parse_float=finite_float)


def rational_seconds_inventory_bindings(policy, *, required=False):
    """Accept no review yet or the exact nonempty singleton descriptor binding."""
    bindings = policy.get("mutation_inventories")
    if type(bindings) is not dict:
        raise Incomplete("Rational-seconds inventory bindings must be an object.")
    if not bindings and not required:
        return bindings
    if set(bindings) != {RATIONAL_SECONDS_SUBJECT}:
        raise Incomplete("Rational-seconds mutation requires its complete reviewed singleton inventory.")
    row = bindings[RATIONAL_SECONDS_SUBJECT]
    if (type(row) is not dict or set(row) != {"count", "sha256"}
            or type(row["count"]) is not int or not 0 < row["count"] <= policy["maximum_mutants"]
            or type(row["sha256"]) is not str or not re.fullmatch(r"[0-9a-f]{64}", row["sha256"])):
        raise Incomplete("Rational-seconds inventory requires a positive count and exact SHA-256.")
    return bindings


def read_rational_seconds_policy(root):
    """Admit a current singleton without altering any historical leap/fuzz role."""
    policy = rational_seconds_document(root / RATIONAL_SECONDS_POLICY)
    extra = {"candidate_sha256", "inputs_sha256", "mutation_inventories", "mutation_equivalences", "scope"}
    if (type(policy) is not dict or set(policy) != set(RATIONAL_SECONDS_FIXED) | extra
            or any(type(policy.get(key)) is not type(value) or policy[key] != value
                   for key, value in RATIONAL_SECONDS_FIXED.items())
            or type(policy.get("scope")) is not str or not 0 < len(policy["scope"]) <= 2048):
        raise Incomplete("Unreviewed rational-seconds scope, profile, threshold or budget.")
    _, identity = PRODUCT.admit_product(root)
    if (len(identity) != policy["source_file_count"]
            or type(policy["candidate_sha256"]) is not str
            or policy["candidate_sha256"] != COMMON.canonical_sha256(identity)):
        raise Incomplete("Rational-seconds requires the exact current complete 374-input product candidate.")
    pins = policy["inputs_sha256"]
    if (type(pins) is not dict or set(pins) != set(RATIONAL_SECONDS_BOUND_INPUTS)
            or any(type(digest) is not str or re.fullmatch(r"[0-9a-f]{64}", digest) is None
                   or PRODUCT.sha256(root / name) != digest for name, digest in pins.items())):
        raise Incomplete("Rational-seconds protected source/helper bindings changed.")
    manifest = tomllib.loads((root / "policy/verification-manifest.toml").read_text())
    if (manifest.get("critical_files", []).count(RATIONAL_SECONDS_FILE) != 1
            or sum(row.get("package") == "time-domain" and RATIONAL_SECONDS_FILE in row.get("files", [])
                   for row in manifest.get("security_modules", [])) != 1):
        raise Incomplete("Rational-seconds needs one exact critical/security owner.")
    if re.search(r"#\s*\[\s*test\s*\]", (root / RATIONAL_SECONDS_FILE).read_text()):
        raise Incomplete("Rational-seconds production coverage cannot contain inline test bodies.")
    rational_seconds_inventory_bindings(policy)
    COMMON.admit_mutation_equivalences(root, RATIONAL_SECONDS_SCOPE, policy)
    return policy


def rational_seconds_test_execution(text, *, doctests):
    """Require real named healthy tests and source-owned legal/privacy doctests."""
    records, pending = [], None
    header = re.compile(r"test ([^\r\n]+?) \.\.\. (.*)")
    trace = re.compile(r"RATIONAL_SECONDS_PROFILE(?:[ \t].*)?")
    for line in text.splitlines():
        if pending is not None:
            if line == "ok":
                records.append(pending)
                pending = None
            elif trace.fullmatch(line) is None:
                raise Incomplete("Rational-seconds native test lacks its named successful completion.")
            continue
        match = header.fullmatch(line)
        if match is not None:
            if match[2] == "ok":
                records.append(match[1])
            elif trace.fullmatch(match[2]) is not None:
                pending = match[1]
            else:
                raise Incomplete("Rational-seconds native test record has no successful outcome.")
        elif line in {"ok", "FAILED", "ignored"}:
            raise Incomplete("Rational-seconds native completion lacks its test identity.")
    if pending is not None:
        raise Incomplete("Rational-seconds native test completion is truncated.")
    summaries = [line for line in text.splitlines() if line.startswith("test result:")]
    pattern = r"test result: ok\. (\d+) passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;.*"
    counts = [re.fullmatch(pattern, line) for line in summaries]
    if (not summaries or any(match is None for match in counts) or len(records) != len(set(records))
            or sum(int(match[1]) for match in counts if match is not None) != len(records)
            or not RATIONAL_SECONDS_REQUIRED_TESTS <= set(records)):
        raise Incomplete("Rational-seconds execution lacks complete required actual passing test records.")
    docs = [name for name in records if re.fullmatch(
        r"[^\r\n]+\.rs - (?:[^\r\n]+ )?\(line [1-9][0-9]*\)(?: - compile fail)?", name)]
    if doctests:
        if not RATIONAL_SECONDS_REQUIRED_DOCTESTS:
            raise Incomplete("Rational-seconds legal/privacy doctest identities await source review.")
        for owner, compile_fail, count in RATIONAL_SECONDS_REQUIRED_DOCTESTS:
            pattern = (r"(?:[^\r\n]+/)?crates/time-domain/src/rational_seconds\.rs - "
                       + re.escape(owner) + r" \(line [1-9][0-9]*\)"
                       + (r" - compile fail" if compile_fail else ""))
            if sum(re.fullmatch(pattern, name) is not None for name in docs) != count:
                raise Incomplete("Rational-seconds baseline omits its actual legal/privacy doctests.")
        selected_docs = [name for name in docs if re.match(
            r"(?:[^\r\n]+/)?crates/time-domain/src/rational_seconds\.rs - ", name)]
        if len(selected_docs) != sum(row[2] for row in RATIONAL_SECONDS_REQUIRED_DOCTESTS):
            raise Incomplete("Rational-seconds baseline includes an unreviewed source-owned doctest.")
    elif docs:
        raise Incomplete("Rational-seconds coverage unexpectedly includes doctests.")
    return {"tests_executed": len(records), "required_tests": sorted(RATIONAL_SECONDS_REQUIRED_TESTS),
            "selected_doctests": sum(row[2] for row in RATIONAL_SECONDS_REQUIRED_DOCTESTS) if doctests else 0,
            "all_package_doctests": len(docs)}


def rational_seconds_baseline(work):
    """Read the actual owned tool baseline, without crediting listing or setup."""
    directory = work / "mutation" / RATIONAL_SECONDS_SUBJECT / "mutants.out"
    outcomes = COMMON.read_analysis_json(directory / "outcomes.json", maximum=64 * 1024 * 1024)
    if type(outcomes) is not dict or type(outcomes.get("outcomes")) is not list:
        raise Incomplete("Rational-seconds baseline outcomes have an invalid shape.")
    rows = [row for row in outcomes["outcomes"]
            if type(row) is dict and row.get("scenario") == "Baseline"]
    if len(rows) != 1 or rows[0].get("summary") != "Success":
        raise Incomplete("Rational-seconds mutation needs exactly one successful native baseline.")
    relative = COMMON.analysis_output_path(rows[0].get("log_path", ""))
    if relative.is_absolute() or not relative.parts or ".." in relative.parts:
        raise Incomplete("Rational-seconds baseline log escapes owned output.")
    path = directory / relative
    if path.is_symlink():
        raise Incomplete("Rational-seconds baseline log cannot use links.")
    raw = COMMON.read_analysis_bytes(path, maximum=16 * 1024 * 1024)
    return {**rational_seconds_test_execution(raw.decode("utf-8"), doctests=True),
            "log_sha256": hashlib.sha256(raw).hexdigest()}


def enumerate_rational_seconds(session, policy, report):
    """Retain complete bounded discovery; launch no baseline or mutant execution."""
    command = [str(session.tools / "cargo-mutants"), "mutants", "--package", "time-domain",
               "--file", RATIONAL_SECONDS_FILE, "--test-workspace", "false", "--cap-lints", "false",
               "--jobs", "10", "--jobserver", "true", "--jobserver-tasks", "10",
               "--cargo-arg=--message-format=json"]
    COMMON.mutation_campaign_command(RATIONAL_SECONDS_SCOPE, session.tools, RATIONAL_SECONDS_SUBJECT,
        [*command, "--baseline", "run", "--build-timeout", "90", "--timeout", "60",
         "--output", "/work/mutation/" + RATIONAL_SECONDS_SUBJECT],
        expected_jobs=10, expected_message_format="json", policy=policy)
    raw = session.run("mutation-enumeration:" + RATIONAL_SECONDS_SUBJECT,
                      [*command, "--list", "--json"], min(policy["enumeration_wall_seconds"], 120))
    report["mutation_enumeration"] = {RATIONAL_SECONDS_SUBJECT: {"raw_output": raw}}
    items, warning = COMMON.parse_mutation_listing(raw, expected_jobs=10)
    inventory = COMMON.mutation_inventory(RATIONAL_SECONDS_SCOPE, items,
                                         policy["maximum_listed_per_file"], RATIONAL_SECONDS_FILE)
    total = COMMON.combined_mutant_count({RATIONAL_SECONDS_SUBJECT: inventory}, policy["maximum_listed_total"])
    report["mutation_enumeration"][RATIONAL_SECONDS_SUBJECT]["warning"] = warning
    report["listed_inventories"] = {RATIONAL_SECONDS_SUBJECT: inventory}
    report["listed_inventory_bindings"] = {RATIONAL_SECONDS_SUBJECT: {
        "count": total, "sha256": COMMON.canonical_sha256({
            name: {key: value for key, value in item.items() if key != "diff"}
            for name, item in inventory.items()})}}
    report.update(listed_total=total, campaign_cap_exceeded=total > policy["maximum_mutants"],
                  campaign_admission=False, qualification_passed=False)


def execute_rational_seconds(args, report):
    """Execute the exact new owner through existing custody, retaining legacy roles."""
    if (args.subject != RATIONAL_SECONDS_SUBJECT or args.lane not in {"enumerate", "coverage", "mutation", "all"}
            or args.tool_admission_profile != "full"
            or any(getattr(args, name, None) for name in
                   ("inventory_admission", "cohort_plan", "cohort_id", "cohort_receipt"))):
        raise Incomplete("Rational-seconds admits only its singleton full-profile enumerate/coverage/mutation/all role.")
    policy = read_rational_seconds_policy(ROOT)
    if args.lane in {"mutation", "all"}:
        rational_seconds_inventory_bindings(policy, required=True)
    session_args = argparse.Namespace(**vars(args))
    if args.lane == "enumerate":
        session_args.lane = "mutation"
    report.update(subjects=RATIONAL_SECONDS_SCOPE.select(RATIONAL_SECONDS_SUBJECT),
                  candidate_sha256=policy["candidate_sha256"], qualification_passed=False,
                  historical_family_bindings_reused=False)
    session = COMMON.Session(ROOT, session_args, report, RATIONAL_SECONDS_SCOPE, policy,
                             RATIONAL_SECONDS_BOUND_INPUTS)
    try:
        if (len(session.product_identity) != policy["source_file_count"]
                or COMMON.canonical_sha256(session.product_identity) != policy["candidate_sha256"]
                or rational_seconds_document(session.source / RATIONAL_SECONDS_POLICY) != policy
                or any(PRODUCT.sha256(session.source / name) != policy["inputs_sha256"][name]
                       for name in RATIONAL_SECONDS_BOUND_INPUTS)):
            raise Incomplete("Captured rational-seconds candidate/policy/protected inputs differ.")
        if args.lane == "enumerate":
            enumerate_rational_seconds(session, policy, report)
        if args.lane in {"coverage", "all"}:
            command = [str(session.tools / "cargo-llvm-cov"), "llvm-cov", "--package", "time-domain",
                       "--frozen", "--jobs", "6", "--lib", "--test", "rational_seconds", "--json",
                       "--output-path", "/work/coverage.json", "--failure-mode", "any",
                       "--ignore-filename-regex", "(^|/)tests/|/rustlib/src/rust/library/",
                       "--", "--test-threads=1"]
            raw = session.run("coverage", command, policy["coverage_wall_seconds"])
            report["coverage_execution"] = rational_seconds_test_execution(raw, doctests=False)
            coverage = COMMON.read_analysis_json(session.work / "coverage.json")
            report["coverage"] = {RATIONAL_SECONDS_SUBJECT: COMMON.coverage_counts(
                RATIONAL_SECONDS_SCOPE, coverage, "/work/source/" + RATIONAL_SECONDS_FILE)}
        if args.lane in {"mutation", "all"}:
            def run(label, command, timeout, **options):
                raw = session.run(label, command, timeout, **options)
                if label == "mutation-campaign:" + RATIONAL_SECONDS_SUBJECT:
                    report["baseline_execution"] = rational_seconds_baseline(session.work)
                return raw
            COMMON.run_analysis_lanes(RATIONAL_SECONDS_SCOPE, report["subjects"], policy,
                                      session.work, session.tools, run, report, "mutation")
    finally:
        session.finish()
    if args.lane != "enumerate":
        report["qualification_passed"] = True


def main_rational_seconds(args):
    """Report the narrow gate independently of historical leap/fuzz roles."""
    report = {"status": "incomplete", "selected_lane": args.lane,
              "scope": "Selected pure rational-seconds owner " + args.lane,
              "qualification_passed": False, "native_qualification": "not performed"}
    try:
        if args.vendor_dir is None:
            args.vendor_dir = Path(json.loads(PRODUCT.POINTER.read_text())["vendor_dir"])
        execute_rational_seconds(args, report)
        report["status"] = "enumerated" if args.lane == "enumerate" else "passed"
    except (Incomplete, PRODUCT.ENV.Incomplete, OSError, ValueError, KeyError, TypeError, StopIteration) as error:
        report["qualification_passed"] = False
        report["error"] = str(error)
    if report.get("owned_output") == str(args.output_dir):
        (args.output_dir / "verification.json").write_text(json.dumps(report, indent=2, allow_nan=False) + "\n")
    print(report["status"].upper() + ": " + report["scope"], flush=True)
    if report.get("error"):
        print(report["error"], file=sys.stderr)
    return 0 if report["status"] in {"enumerated", "passed"} else 1


def execute(args, report):
    """Run only the reviewed admitted fixed scope and preserve partial observations."""
    if args.subject == RATIONAL_SECONDS_SUBJECT:
        return execute_rational_seconds(args, report)
    if args.lane == "enumerate":
        raise Incomplete("Enumeration is admitted only for the exact rational-seconds role.")
    policy = read_policy(ROOT)
    COMMON.admission_profile(args, SCOPE, policy)
    report["fuzz"] = {"status": "not-performed", "subjects": dict(LEGACY_SUBJECTS),
                      "not_fuzzed": dict(LEAP_SUBJECTS)}
    selected = policy["coverage_guided_fuzzing"]
    extra = ("scripts/verify-time-domain-analysis.py", *selected["inputs_sha256"])
    session = COMMON.Session(ROOT, args, report, SCOPE, policy, extra)
    subjects = SCOPE.select(args.subject)
    report["subjects"] = subjects
    try:
        if args.lane in {"contracts", "all"}:
            contracts(session, policy, report)
        COMMON.run_analysis_lanes(SCOPE, subjects, policy, session.work, session.tools, session.run, report, args.lane)
        if args.lane in {"fuzz", "all"}:
            fuzz(session, selected, report)
    finally:
        session.finish()


def main(argv=None):
    """Explicit fixed lanes, fresh private outputs, and honest incomplete status."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--lane", choices=("contracts", "enumerate", "coverage", "mutation", "fuzz", "all"), default="all")
    parser.add_argument("--subject", choices=(*SUBJECTS, RATIONAL_SECONDS_SUBJECT, "all"), default="all")
    parser.add_argument("--tool-admission-profile", choices=("full", "stable-core"), default="full")
    parser.add_argument("--vendor-dir", type=Path)
    parser.add_argument("--output-dir", type=Path, required=True)
    args = parser.parse_args(argv)
    if args.subject == RATIONAL_SECONDS_SUBJECT:
        return main_rational_seconds(args)
    if args.tool_admission_profile == "stable-core" and args.lane not in {"contracts", "coverage", "mutation"}:
        parser.error("--tool-admission-profile stable-core requires contracts, coverage or mutation")
    report = {"status": "incomplete", "scope": "S1.1c/S1.4 pure domain contracts/coverage/mutation; leap fuzzing not performed",
              "native_qualification": "not performed"}
    try:
        if args.vendor_dir is None:
            args.vendor_dir = Path(json.loads(PRODUCT.POINTER.read_text())["vendor_dir"])
        execute(args, report)
        report["status"] = "passed"
    except (Incomplete, PRODUCT.ENV.Incomplete, OSError, ValueError, KeyError, TypeError, StopIteration) as error:
        report["error"] = str(error)
    if report.get("owned_output") == str(args.output_dir):
        (args.output_dir / "verification.json").write_text(json.dumps(report, indent=2) + "\n")
    print(report["status"].upper() + ": " + report["scope"], flush=True)
    if report.get("error"):
        print(report["error"], file=sys.stderr)
    return 0 if report["status"] == "passed" else 1


if __name__ == "__main__":
    raise SystemExit(main())
