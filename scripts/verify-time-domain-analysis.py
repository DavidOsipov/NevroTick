#!/usr/bin/env python3
"""Qualify the admitted S1.1c portable numerical/domain analysis scope.

GPL-2.0-only, David Osipov with AI assistance, 29 October 2026. Orchestration
adapts the repository's verify-wire-analysis.py; shared custody/accounting lives
in portable-analysis.py. No downloads, installations or host Cargo fallback.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
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
SUBJECTS = {name: f"crates/time-domain/src/{name}.rs"
            for name in ("coordinate", "era", "exchange", "clock", "rounding")}
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
        name = item["name"]
        if name in result or item.get("source") != REGISTRY:
            raise Incomplete("Ambiguous or unreviewed locked registry identity.")
        result[name] = (item, block)
    return result


def reviewed_fuzz_lock(product_text, control_text):
    """Preserve Garde's reviewed closure and all original libFuzzer registry blocks."""
    product = registry_blocks(product_text)
    original = registry_blocks(control_text)
    local = [item for item in tomllib.loads(product_text)["package"] if item["name"] == "time-domain"]
    if len(local) != 1 or local[0].get("source") or local[0].get("version") != "0.0.0" or local[0].get("dependencies") != ["garde"]:
        raise Incomplete("The actual checked product-local owner is absent.")
    selected, pending = {}, ["garde"]
    while pending:
        name = pending.pop()
        if name in selected:
            continue
        if name not in product:
            raise Incomplete("Garde locked dependency closure is incomplete.")
        selected[name] = product[name]
        pending.extend(value.split(" ")[0] for value in product[name][0].get("dependencies", []))
    for name, entry in original.items():
        if name in selected and selected[name][0] != entry[0]:
            raise Incomplete("Shared product/fuzz registry identity or dependency edges disagree.")
        selected.setdefault(name, entry)
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
    required = {"schema_version": 1, "package": "time-domain", "critical_files": list(SUBJECTS.values()),
                "coverage_minimum_line_percent": 90, "mutation_minimum_viable_killed_percent": 95,
                "maximum_mutants": 512, "coverage_wall_seconds": 300, "mutation_wall_seconds": 1200,
                "mutation_build_seconds": 90, "mutation_test_seconds": 60, "contracts_wall_seconds": 300,
                "jobs": RUNTIME.MAX_JOBS, "test_threads": RUNTIME.MAX_JOBS}
    if any(type(policy.get(key)) is not type(value) or policy.get(key) != value for key, value in required.items()):
        raise Incomplete("Unreviewed time-domain analysis scope, threshold or budget.")
    COMMON.admit_mutation_equivalences(root, SCOPE, policy)
    inventory = tomllib.loads((root / "policy/verification-manifest.toml").read_text())
    for filename in SUBJECTS.values():
        if (inventory.get("critical_files", []).count(filename) != 1
                or sum(module.get("package") == "time-domain" and filename in module.get("files", [])
                       for module in inventory.get("security_modules", [])) != 1):
            raise Incomplete("A numeric owner is absent/duplicated in the security/critical inventory.")
    selected = policy["coverage_guided_fuzzing"]
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
             'let rtt = exact.round_trip().round_nanoseconds_ties_even()?;')
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
                 "--frozen", "--jobs", str(RUNTIME.MAX_JOBS), "--lib", "--message-format=json"], COMMON.remaining_timeout(deadline, 120))
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
            text = run("external-legal-execution", ["/work/contracts/" + name], COMMON.remaining_timeout(deadline, 30),
                       sealed=[consumers, dependency_copy])
            if "legal-time-domain-contracts:11" not in text:
                raise Incomplete("Legal external consumer did not reach every selected representation.")
    report["contracts"] = {"legal_consumers_compiled_and_executed": 1, "library_artifact": artifact,
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
    actual = reviewed_fuzz_lock((ROOT / "Cargo.lock").read_text(), control.read_text())
    if actual.encode() != (ROOT / selected["lockfile"]).read_bytes():
        raise Incomplete("Standalone graph no longer preserves the original reviewed package blocks.")
    products = {item["name"]: item for item in session.dependency_policy["packages"]}
    old = {item["name"]: item for item in original["locked_registry_packages"]}
    if len(products) != 16 or len(old) != 10:
        raise Incomplete("Product/control source cohorts changed.")
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
            admitted = next(entry for entry in session.dependency_policy["packages"] if entry["name"] == item["name"])
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
            report["fuzz"] = {"observed_runs": 256, "subjects": dict(SUBJECTS), "seed_files_sha256": seed_identity,
                              "seed_lengths": [176], "seeded_fault": "pending", "sanitizer": "address", "target": ANALYSIS.PLATFORM}
        else:
            report["fuzz"]["seeded_fault"] = "observed intended exact phase-offset assertion failure"
        if any(PRODUCT.sha256(path) != expected for path, expected in retained.items()):
            raise Incomplete("Retained ASan executable changed.")
        if COMMON.fuzz_input_snapshot(session.work, sealed) != before:
            raise Incomplete("Fuzz manifest/lock/source/vendor/config inventory changed.")
    admit_fuzz_sources(session, selected)
    report["fuzz"]["fault_artifacts_sha256"] = PRODUCT.inventory(session.work / "fuzz-artifacts")


def execute(args, report):
    """Run only the reviewed admitted fixed scope and preserve partial observations."""
    policy = read_policy(ROOT)
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
    parser.add_argument("--lane", choices=("contracts", "coverage", "mutation", "fuzz", "all"), default="all")
    parser.add_argument("--subject", choices=(*SUBJECTS, "all"), default="all")
    parser.add_argument("--vendor-dir", type=Path)
    parser.add_argument("--output-dir", type=Path, required=True)
    args = parser.parse_args(argv)
    report = {"status": "incomplete", "scope": "S1.1c pure time-domain portable contracts/coverage/mutation/numeric fuzz",
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
