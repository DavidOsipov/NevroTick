#!/usr/bin/env python3
"""Build then observe a test-only Windows allocator consumer (GPL-2.0-only).

No acquisition, installation, elevation, registry mutation or product effects.
Build and native lanes are separate. Native execution is an explicit second
invocation using the frozen build receipt; no cross-build is a native result.
The allocator's logical requested-payload observations are artifact-specific,
not OS heap usage, a corruption detector, or general proof that no leak exists.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path, PureWindowsPath
import re
import shutil
import struct
import sys
import time
import tomllib
import uuid

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[1]


def load(name, filename):
    """Import a repository helper without invoking its command entry point."""
    specification = importlib.util.spec_from_file_location(name, ROOT / "scripts" / filename)
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


PRODUCT = load("allocation_product", "verify.py")
ANALYSIS = load("allocation_payload_admission", "check-analysis.py")
ENV = PRODUCT.ENV
Incomplete = ENV.Incomplete
TARGET = "x86_64-pc-windows-msvc"
OBSERVER = "policy/time-domain-windows-allocation-observer.rs"
RUNNER = "scripts/verify-time-domain-windows-allocation.py"
EXTRA_INPUTS = (OBSERVER, RUNNER, "scripts/check-analysis.py", "scripts/analysis-runtime.py",
                "scripts/analysis-controls.py", "policy/analysis-tools.json")
CASES = ("valid", "earliest_invalid", "latest_invalid", "both_invalid", "inverted")
BATCHES = (1, 32, 256, 4096)
COUNTERS = ("alloc_requests", "zeroed_requests", "realloc_requests", "requested_bytes",
            "successful_requests", "successful_requested_bytes", "failed_requests", "deallocations")
RAW_FIELDS = (*COUNTERS, "live_payload_bytes", "peak_live_payload_bytes", "counter_error")
PROFILE_DIRECTORY = {"debug": "debug", "release": "release", "verify-release": "verify-release"}
MAX_JSON = 1024 * 1024


def canonical_digest(value):
    """Hash a JSON value with one stable UTF-8 serialization."""
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":")).encode()).hexdigest()


def decode(data):
    """Reject oversized/nonfinite JSON and duplicate keys before using evidence."""
    if len(data) > MAX_JSON:
        raise Incomplete("Native JSON exceeds its fixed byte budget.")
    def pairs(values):
        result = {}
        for key, value in values:
            if key in result:
                raise Incomplete("Duplicate evidence key.")
            result[key] = value
        return result
    return json.loads(data, object_pairs_hook=pairs,
                      parse_constant=lambda _: (_ for _ in ()).throw(Incomplete("Nonfinite evidence.")))


def profile_flags(profile):
    """Mirror the admitted Cargo profiles for direct rustc/clippy-driver calls."""
    if profile not in PROFILE_DIRECTORY:
        raise Incomplete("Unadmitted observer build profile.")
    optimized = profile != "debug"
    flags = ["-C", "opt-level=" + ("3" if optimized else "0"),
             "-C", "debug-assertions=" + ("no" if profile == "release" else "yes"),
             "-C", "overflow-checks=yes", "-C", "panic=unwind",
             "-C", "debuginfo=" + ("1" if optimized else "2"), "-C", "strip=none"]
    if optimized:
        flags.extend(["-C", "lto=thin", "-C", "codegen-units=1"])
    return flags


def lint_flags(workspace):
    """Carry adopted workspace Rust/Clippy lints into the external direct build."""
    result = []
    for namespace in ("rust", "clippy"):
        values = workspace["workspace"]["lints"][namespace]
        ordered = sorted(values.items(), key=lambda pair: (
            pair[1].get("priority", 0) if isinstance(pair[1], dict) else 0, pair[0]))
        for name, value in ordered:
            level = value["level"] if isinstance(value, dict) else value
            if level != "deny":
                raise Incomplete("Unexpected observer lint policy level.")
            result.extend(["--deny", ("clippy::" if namespace == "clippy" else "") + name])
    return result


def artifact_path(output, profile):
    """Attribute exactly one actual library emitted for the selected owner/target."""
    expected = f"/work/target/{TARGET}/{PROFILE_DIRECTORY[profile]}/libtime_domain.rlib"
    found = []
    for line in output.splitlines():
        if not line.startswith("{"):
            continue
        item = decode(line.encode())
        if item.get("reason") != "compiler-artifact" or item.get("target", {}).get("name") != "time_domain":
            continue
        observation = item.get("profile", {})
        if (item.get("manifest_path") != "/work/fixture/crates/time-domain/Cargo.toml"
                or item.get("target", {}).get("kind") != ["lib"]
                or item.get("target", {}).get("crate_types") != ["lib"]
                or observation.get("opt_level") != ("0" if profile == "debug" else "3")
                or observation.get("debug_assertions") is not (profile != "release")
                or observation.get("overflow_checks") is not True or observation.get("test") is not False
                or [name for name in item.get("filenames", []) if name.endswith(".rlib")] != [expected]):
            raise Incomplete("Emitted observer dependency has the wrong owner, profile or target.")
        found.append(expected)
    if found != [expected]:
        raise Incomplete("No unique actual time-domain library artifact was emitted.")
    return expected


def emitted_dependency_artifacts(output, profile, source, vendor, policy):
    """Attribute the complete actual Cargo host/target library and proc-macro set."""
    directory = PROFILE_DIRECTORY[profile]
    locked = tomllib.loads((source / "Cargo.lock").read_text())["package"]
    packages = {item["name"]: item for item in locked}
    if len(packages) != len(locked):
        raise Incomplete("Ambiguous locked package names need separate artifact admission.")
    required, pending = set(), ["time-domain"]
    while pending:
        name = pending.pop()
        if name in required:
            continue
        if name not in packages:
            raise Incomplete("Emitted dependency owner is absent from the locked graph.")
        required.add(name)
        pending.extend(value.split()[0] for value in packages[name].get("dependencies", []))
    external = {item["name"]: item for item in policy["packages"]}
    if any(name != "time-domain" and (name not in external or external[name]["version"] != packages[name]["version"])
           for name in required):
        raise Incomplete("Artifact closure differs from admitted external package identities.")
    manifests = {}
    for name in required:
        version = packages[name]["version"]
        manifest = source / "crates/time-domain/Cargo.toml" if name == "time-domain" else vendor / f"{name}-{version}/Cargo.toml"
        manifests[name] = tomllib.loads(manifest.read_text())
    result, units, seen_packages = {}, set(), set()
    for line in output.splitlines():
        if not line.startswith("{"):
            continue
        item = decode(line.encode())
        if item.get("reason") != "compiler-artifact":
            continue
        package_id = item.get("package_id")
        owners = [name for name in required if package_id == (
            "path+file:///work/fixture/crates/time-domain#" + packages[name]["version"] if name == "time-domain"
            else f"registry+https://github.com/rust-lang/crates.io-index#{name}@{packages[name]['version']}")]
        if len(owners) != 1:
            raise Incomplete("Cargo emitted an unknown or ambiguous dependency package identity.")
        name = owners[0]
        prefix = "/work/fixture/crates/time-domain" if name == "time-domain" else f"/vendor/{name}-{packages[name]['version']}"
        if item.get("manifest_path") != prefix + "/Cargo.toml":
            raise Incomplete("Cargo dependency manifest attribution differs from its admitted owner.")
        manifest = manifests[name]
        target, observed_profile = item.get("target", {}), item.get("profile", {})
        kind, crate_types = target.get("kind"), target.get("crate_types")
        files = item.get("filenames")
        if not isinstance(files, list) or not files or any(not isinstance(path, str) for path in files) or len(set(files)) != len(files):
            raise Incomplete("Missing, duplicate or invalid emitted dependency filenames.")
        if (observed_profile.get("test") is not False or observed_profile.get("overflow_checks") is not True
                or observed_profile.get("debug_assertions") is not (profile != "release")):
            raise Incomplete("Emitted dependency profile differs from checked observer construction.")
        if kind == ["custom-build"]:
            build_path = manifest.get("package", {}).get("build", "build.rs")
            if (name == "time-domain" or external[name].get("build_script") is not True
                    or not isinstance(build_path, str) or target.get("name") != "build-script-build"
                    or crate_types != ["bin"] or target.get("src_path") != prefix + "/" + build_path
                    or observed_profile.get("opt_level") != "0" or len(files) != 1
                    or not re.fullmatch(r"/work/target/" + re.escape(directory + "/build/" + name)
                                        + r"-[a-f0-9]{16}/build-script-build", files[0])):
                raise Incomplete("Unexpected build-script artifact in observer dependency output.")
            # These admitted executables already ran under Cargo; they are not
            # Rust library-search inputs and are retained in the raw Cargo row.
            continue
        library = manifest.get("lib", {})
        macro = library.get("proc-macro", False)
        crate = library.get("name", name.replace("-", "_"))
        expected_kind = "proc-macro" if macro else "lib"
        if (type(macro) is not bool or kind != [expected_kind] or crate_types != [expected_kind]
                or target.get("name") != crate or target.get("src_path") != prefix + "/" + library.get("path", "src/lib.rs")):
            raise Incomplete("Emitted library/proc-macro kind differs from its admitted source manifest.")
        host_root = f"/work/target/{directory}/deps/"
        target_root = f"/work/target/{TARGET}/{directory}/deps/"
        role = "host" if all(path.startswith(host_root) for path in files) else "target"
        expected_root = host_root if role == "host" else target_root
        if macro and role != "host" or (name == "time-domain" and role != "target"):
            raise Incomplete("Proc-macro/first-party dependency has the wrong host/target artifact root.")
        expected_opt = "0" if role == "host" or profile == "debug" else "3"
        if observed_profile.get("opt_level") != expected_opt:
            raise Incomplete("Emitted host/target artifact uses an unexpected optimization profile.")
        suffixes = {".so"} if macro else {".rlib", ".rmeta"}
        if {Path(path).suffix for path in files} != suffixes or len(files) != len(suffixes):
            raise Incomplete("Incomplete emitted library or host proc-macro artifact set.")
        unit = (name, role, expected_kind)
        if unit in units:
            raise Incomplete("Duplicate emitted dependency compilation unit.")
        units.add(unit); seen_packages.add(name)
        for path in files:
            local_rlib = f"/work/target/{TARGET}/{directory}/libtime_domain.rlib"
            if not (name == "time-domain" and path == local_rlib):
                if not re.fullmatch(re.escape(expected_root + "lib" + crate) + r"-[a-f0-9]{16}\.(?:rlib|rmeta|so)", path):
                    raise Incomplete("Emitted dependency filename escapes its exact host/target crate root.")
            if path in result:
                raise Incomplete("Duplicate emitted dependency path.")
            result[path] = {"package_id": package_id, "manifest_path": item["manifest_path"],
                            "kind": expected_kind, "role": role, "target_name": crate,
                            "features": item.get("features", [])}
    if seen_packages != required or not result:
        raise Incomplete("Cargo did not emit the complete locked dependency/library/proc-macro closure.")
    return result


def copy_dependency_artifacts(artifacts, work, destination):
    """Copy only attributed emitted files into an independent immutable island."""
    destination.mkdir(mode=0o700)
    copied = set()
    for path in sorted(artifacts):
        original = work / path.removeprefix("/work/")
        if (not path.startswith("/work/target/") or original != original.resolve(strict=True)
                or not original.is_file() or original.name in copied):
            raise Incomplete("Emitted dependency escaped or aliases another copied artifact.")
        checksum = PRODUCT.sha256(original)
        target = destination / original.name
        shutil.copyfile(original, target); target.chmod(0o400)
        if PRODUCT.sha256(target) != checksum or target.stat().st_nlink != 1 or original.stat().st_ino == target.stat().st_ino:
            raise Incomplete("Observer dependency copy is not independent and identical.")
        copied.add(original.name)
    if not copied or "libtime_domain.rlib" not in copied:
        raise Incomplete("No actual first-party dependency library was copied.")
    return PRODUCT.inventory(destination)


def interval(value):
    """Recompute every interval from raw counters; reject wrap, error or substitution."""
    if not isinstance(value, dict) or set(value) != {*COUNTERS, "live_baseline_bytes", "retained_delta_bytes",
                                                   "peak_above_baseline_bytes", "before", "after"}:
        raise Incomplete("Incomplete allocation interval.")
    before, after = value["before"], value["after"]
    for raw in (before, after):
        if not isinstance(raw, dict) or set(raw) != set(RAW_FIELDS) or raw["counter_error"] is not False:
            raise Incomplete("Allocation counters are incomplete or inconclusive.")
        if any(type(raw[name]) is not int or not 0 <= raw[name] < 1 << 64 for name in RAW_FIELDS[:-1]):
            raise Incomplete("Non-u64 raw allocation counter.")
        if raw["successful_requests"] + raw["failed_requests"] != sum(raw[name] for name in COUNTERS[:3]):
            raise Incomplete("Allocation attempts do not reconcile.")
        if raw["peak_live_payload_bytes"] < raw["live_payload_bytes"]:
            raise Incomplete("Allocation peak is below live payload.")
    expected = {name: after[name] - before[name] for name in COUNTERS}
    if any(amount < 0 for amount in expected.values()):
        raise Incomplete("Allocation counter decreased.")
    expected.update(live_baseline_bytes=before["live_payload_bytes"],
                    retained_delta_bytes=after["live_payload_bytes"] - before["live_payload_bytes"],
                    peak_above_baseline_bytes=after["peak_live_payload_bytes"] - before["live_payload_bytes"])
    if any(type(value.get(name)) is not int or value[name] != amount for name, amount in expected.items()):
        raise Incomplete("Reported allocation metric differs from raw endpoints.")
    if after["failed_requests"] != before["failed_requests"]:
        raise Incomplete("A measured allocation request failed.")
    return expected


def validate_observation(value, claims):
    """Validate workload/calibrations independently, including unsampled warmup retention."""
    expected_workload = {"cases": 5, "cold_calls_per_case": 1, "warmups_per_case": 64,
                         "batches": list(BATCHES), "held_box_product_calls": 32,
                         "total_product_calls": 22282, "warmup_mismatches": 0,
                         "cold_definition": "first_observation_per_case_in_shared_process_before_any_product_warmup"}
    if (type(value.get("schema_version")) is not int or value["schema_version"] != 1
            or value.get("purpose") != "test_only_time_domain_allocator_requests"
            or value.get("status") != "observed" or value.get("runner_claims_unverified") != claims
            or canonical_digest(value.get("workload")) != canonical_digest(expected_workload)
            or value.get("build_observations") != {"target": TARGET, "debug_assertions": claims["profile"] != "release",
                                                    "external_build_identity_required": True}):
        raise Incomplete("Observer identity, workload, build observation or result is incomplete.")
    samples = value.get("samples")
    expected = [(case, "cold", 1) for case in CASES] + [
        (case, "warm_batch", calls) for case in CASES for calls in BATCHES]
    if not isinstance(samples, list) or len(samples) != len(expected):
        raise Incomplete("Missing, extra or empty allocation sample selection.")
    statistics = []
    for sample, (case, phase, calls) in zip(samples, expected):
        if (sample.get("case") != case or sample.get("phase") != phase or type(sample.get("calls")) is not int
                or sample["calls"] != calls or type(sample.get("product_mismatches")) is not int
                or sample["product_mismatches"] != 0):
            raise Incomplete("Wrong allocation workload order/count or actual product mismatch.")
        measured = interval(sample["metrics"])
        if measured["retained_delta_bytes"] != 0:
            raise Incomplete("Observed retained delta in a constructor interval.")
        statistics.append({"case": case, "phase": phase, "calls": calls, **measured})
    calibrations = value.get("calibrations", {})
    for name, calls in (("before_product_box", 0), ("held_box_product_after_cold", 32), ("after_product_box", 0)):
        control = calibrations[name]
        stages = {stage: interval(control[stage])
                  for stage in ("held", "after_product_while_held", "after_explicit_drop")}
        held, kept, dropped = (stages[stage] for stage in stages)
        if (control.get("discriminates") is not True or control.get("contents_match") is not True
                or type(control.get("expected_payload_bytes")) is not int or control["expected_payload_bytes"] != 4096
                or type(control.get("product_calls")) is not int or control["product_calls"] != calls
                or type(control.get("product_mismatches")) is not int or control["product_mismatches"] != 0
                or held["retained_delta_bytes"] < 4096
                or held["successful_requests"] < 1 or kept["retained_delta_bytes"] != held["retained_delta_bytes"]
                or dropped["retained_delta_bytes"] != 0 or dropped["deallocations"] <= held["deallocations"]
                or any(control[stage]["before"] != control["held"]["before"] for stage in stages)):
            raise Incomplete("Held/released allocation calibration does not discriminate.")
    vector = calibrations["safe_vec"]
    stages = {stage: interval(vector[stage]) for stage in ("zeroed", "grown", "shrunk", "after_explicit_drop")}
    zeroed, grown, shrunk, dropped = (stages[stage] for stage in stages)
    if (vector.get("discriminates") is not True or vector.get("contents_match") is not True
            or zeroed["zeroed_requests"] < 1 or grown["realloc_requests"] <= zeroed["realloc_requests"]
            or shrunk["realloc_requests"] <= grown["realloc_requests"]
            or not 0 < shrunk["retained_delta_bytes"] < grown["retained_delta_bytes"]
            or not 0 < zeroed["retained_delta_bytes"] < grown["retained_delta_bytes"]
            or dropped["retained_delta_bytes"] != 0
            or any(vector[stage]["before"] != vector["zeroed"]["before"] for stage in stages)):
        raise Incomplete("Zero/grow/shrink/release forwarding calibration does not discriminate.")
    # ALLOC-REV-1: those 320 calls occur outside the 25 sampled intervals.
    # Reconstruct their retained delta rather than trusting status=observed.
    warmup_before = calibrations["held_box_product_after_cold"]["after_explicit_drop"]["after"]
    warmup_after = samples[5]["metrics"]["before"]
    warmup_delta = warmup_after["live_payload_bytes"] - warmup_before["live_payload_bytes"]
    if warmup_delta != 0:
        raise Incomplete("Observed retained delta across the unsampled warmup calls.")
    final = value["final_counters_before_output"]
    if (set(final) != set(RAW_FIELDS) or final.get("counter_error") is not False
            or any(type(final[name]) is not int or not 0 <= final[name] < 1 << 64 for name in RAW_FIELDS[:-1])
            or final != calibrations["after_product_box"]["after_explicit_drop"]["after"]):
        raise Incomplete("Final pre-output accounting differs from the final explicit release.")
    return {"samples": 25, "product_calls": 22282, "warmup_retained_delta_bytes": warmup_delta,
            "statistics": statistics,
            "limit": "Observed requested payload in this calibrated artifact before exit; no universal leak claim."}


def windows_path(path):
    """Map an explicit local DrvFS path; no shell or UNC/network path is admitted."""
    path = Path(path)
    if (not path.is_absolute() or len(path.parts) < 5 or path.parts[1] != "mnt"
            or not re.fullmatch("[a-z]", path.parts[2]) or any("\\" in part or ":" in part for part in path.parts[3:])
            or path != path.resolve(strict=False)):
        raise Incomplete("Expected a canonical local Windows drive path.")
    return str(PureWindowsPath(path.parts[2].upper() + ":\\", *path.parts[3:]))


def pe_header(path):
    """Inspect bounded PE identity/declared protections, not effective runtime policy."""
    with path.open("rb") as stream:
        header = stream.read(64)
        if len(header) != 64 or header[:2] != b"MZ":
            raise Incomplete("Observer has no DOS header.")
        offset = struct.unpack_from("<I", header, 60)[0]
        if not 64 <= offset <= 1024 * 1024:
            raise Incomplete("Observer PE header offset exceeds bounds.")
        stream.seek(offset)
        data = stream.read(96)
    if len(data) != 96 or data[:4] != b"PE\0\0" or struct.unpack_from("<H", data, 4)[0] != 0x8664:
        raise Incomplete("Observer is not AMD64 PE.")
    if struct.unpack_from("<H", data, 24)[0] != 0x20b:
        raise Incomplete("Observer is not PE32+.")
    flags = struct.unpack_from("<H", data, 94)[0]
    if flags & 0x4160 != 0x4160:
        raise Incomplete("Observer PE omits a required declared link protection.")
    return {"machine": "AMD64", "optional_header": "PE32+", "dll_characteristics": flags,
            "terminal_server_aware_observed": bool(flags & 0x8000),
            "limit": "Declared flags only; no complete runtime mitigation qualification."}


def new_output(path):
    """Create one private fresh external Linux evidence directory without redirects."""
    if (not path.is_absolute() or path.parent != path.parent.resolve(strict=True)
            or path.is_relative_to(ROOT) or ROOT.is_relative_to(path) or path.exists() or path.is_symlink()
            or str(path).startswith("/mnt/") or len(path.parts) < 4):
        raise Incomplete("Evidence output must be a fresh canonical external Linux directory.")
    path.mkdir(mode=0o700)
    return path


def protect_dependencies(invocation, directory):
    """Add the fixed child read-only mount after its owned writable work parent."""
    if (directory.name != "observer-deps" or not directory.is_absolute() or not directory.is_dir()
            or directory != directory.resolve(strict=True)):
        raise Incomplete("Expected the canonical owned observer dependency directory.")
    separator = invocation.index("--")
    header = invocation[:separator]
    parents = [index for index in range(len(header) - 2)
               if header[index:index + 3] == ["--bind", str(directory.parent), "/work"]]
    if len(parents) != 1 or "/work/observer-deps" in header:
        raise Incomplete("Observer dependencies need one owned work parent and no previous child mount.")
    return [*header, "--ro-bind", str(directory), "/work/observer-deps", *invocation[separator:]]


DEPENDENCY_BOUNDARY = (
    "import errno,pathlib\n"
    "owned=pathlib.Path('/work/allocator-write-control')\n"
    "owned.write_bytes(b'owned output'); assert owned.read_bytes()==b'owned output'; owned.unlink()\n"
    "library=pathlib.Path('/work/observer-deps/libtime_domain.rlib')\n"
    "assert library.is_file() and library.stat().st_size>0\n"
    "operations=(lambda:library.chmod(0o600),lambda:library.unlink(),"
    "lambda:(library.parent/'forbidden-write-control').write_bytes(b'must fail'))\n"
    "for operation in operations:\n"
    " try: operation()\n"
    " except OSError as error:\n"
    "  if error.errno!=errno.EROFS: raise\n"
    " else: raise SystemExit('observer dependency mount accepted an owner write')\n"
    "print('ALLOCATION_DEPENDENCIES_READ_ONLY:3')\n"
)


def dependency_boundary(run, directory):
    """Require actual owner-write rejection plus writable-output success before compilation."""
    output = run("sealed-dependency-boundary", ["python3", "-c", DEPENDENCY_BOUNDARY], readonly_dependencies=directory)
    if output.strip() != "ALLOCATION_DEPENDENCIES_READ_ONLY:3":
        raise Incomplete("Observer dependency read-only boundary control did not complete.")


def build(args, report):
    """Freeze/admit real inputs before executing bounded offline guarded builds."""
    if any(getattr(args, name) is None for name in ("vendor_dir", "toolchain_dir", "xwin_config", "xwin_inventory")):
        raise Incomplete("Build requires explicit vendor, toolchain and complete xwin admission paths.")
    policy, product_inputs = PRODUCT.admit_product(ROOT)
    extras = {name: PRODUCT.sha256(ROOT / name) for name in EXTRA_INPUTS}
    vendor, toolchain = args.vendor_dir.resolve(strict=True), args.toolchain_dir.resolve(strict=True)
    vendor_inputs = PRODUCT.admit_vendor(vendor, policy)
    tools = PRODUCT.admit_tools(ROOT, toolchain, None, "windows")
    # Reuse only the stable payload inventory; unrelated analyzers are not run.
    analysis_policy = ANALYSIS.decode(ANALYSIS.read_once(ROOT / "policy/analysis-tools.json"))
    ANALYSIS.validate_policy(analysis_policy)
    pointer = args.analysis_installation or Path.home() / ".local/share/nevrotick/analysis-installation.json"
    base, stable = ANALYSIS.load_installation(pointer, ROOT)
    if stable != toolchain:
        raise Incomplete("Observer toolchain differs from the admitted complete stable payload.")
    specifications = [item for item in analysis_policy["inventories"] if item.get("root") == "stable"]
    if len(specifications) != 1:
        raise Incomplete("Missing unique stable compiler/std inventory.")
    stable_inventory, _ = ANALYSIS.check_inventory(base, specifications[0], stable)
    output = args.output_dir
    report.update(profile=args.profile, product_source_sha256=product_inputs, extra_inputs_sha256=extras,
                  candidate_sha256=canonical_digest(product_inputs), tools_sha256=tools,
                  stable_payload_inventory=stable_inventory, toolchain=str(toolchain), lanes=[])
    for original, name in ((base / specifications[0]["path"], "stable-payload-inventory.json"),
                           (args.xwin_config, "xwin-acquisition.json"), (args.xwin_inventory, "xwin-inventory.json")):
        shutil.copyfile(original, output / name)
        if PRODUCT.sha256(original) != PRODUCT.sha256(output / name):
            raise Incomplete("Admission inventory changed during copying.")
    source, work = output / "source", output / "work"
    source.mkdir(mode=0o700); work.mkdir(mode=0o700)
    (work / "cargo-home").mkdir(mode=0o700)
    (work / "fixture").mkdir(mode=0o700)
    for name, digest in {**product_inputs, **extras}.items():
        destination = source / name
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(ROOT / name, destination)
        if PRODUCT.sha256(destination) != digest:
            raise Incomplete("Observer input changed while freezing the source snapshot.")
    config_hash, inventory_hash = PRODUCT.sha256(args.xwin_config), PRODUCT.sha256(args.xwin_inventory)
    libraries, library_inputs = PRODUCT.windows_libraries(ROOT, toolchain, args.xwin_config, work, args.xwin_inventory)
    report["windows_inputs"] = {"config_sha256": config_hash, "inventory_sha256": inventory_hash,
                                "complete_library_map_sha256": canonical_digest(library_inputs),
                                "library_files": len(library_inputs)}
    (work / "cargo-home/config.toml").write_text(
        '[source.crates-io]\nreplace-with="reviewed"\n[source.reviewed]\ndirectory="/vendor"\n')

    def run(label, command, *, readonly_dependencies=None):
        invocation = PRODUCT.sandbox(toolchain, work, source, vendor, command, None, libraries)
        before = None
        if readonly_dependencies is not None:
            before = PRODUCT.inventory(readonly_dependencies)
            invocation = protect_dependencies(invocation, readonly_dependencies)
        result = ENV.run(invocation, cwd=ROOT, timeout=300,
                         environment={"PATH": "/usr/bin:/bin", "NEVROTICK_SYNTHETIC_CREDENTIAL": "synthetic-control"})
        record = dict(result, lane=label, selected_command=command)
        if before is not None:
            record["readonly_dependencies_before_sha256"] = before
            record["readonly_dependencies_after_sha256"] = PRODUCT.inventory(readonly_dependencies)
        report["lanes"].append(record)
        (output / (label + ".json")).write_text(json.dumps(record, indent=2) + "\n")
        if before is not None and record["readonly_dependencies_after_sha256"] != before:
            raise Incomplete("Observer dependency artifacts changed during a sealed command.")
        return ENV.require_ok(result)

    PRODUCT.boundary_control(run)
    compiler_info = run("compiler-version", ["rustc", "-vV"])
    rust_policy = json.loads((ROOT / "policy/tool-manifest.json").read_text())
    ENV.require_admitted_compiler(rust_policy["rust"]["channel"], compiler_info, rust_policy)
    run("observer-format", ["rustfmt", "--edition", "2024", "--check", "/work/fixture/" + OBSERVER])
    cargo_profile = "dev" if args.profile == "debug" else args.profile
    cargo = ["cargo", "--config", f'target.{TARGET}.linker="/work/linker"', "build",
             "--package", "time-domain", "--lib", "--frozen", "--jobs", "1", "--target", TARGET,
             "--profile", cargo_profile, "--message-format=json"]
    cargo_output = run("library-build", cargo)
    actual = artifact_path(cargo_output, args.profile)
    attributed = emitted_dependency_artifacts(cargo_output, args.profile, source, vendor, policy)
    actual_host = work / actual.removeprefix("/work/")
    sealed = work / "observer-deps"
    dependency_inputs = copy_dependency_artifacts(attributed, work, sealed)
    report["emitted_dependency_artifacts"] = {path: dict(details, sha256=dependency_inputs[Path(path).name])
                                              for path, details in attributed.items()}
    dependency_boundary(run, sealed)
    profile = profile_flags(args.profile)
    workspace = tomllib.loads((source / "Cargo.toml").read_text())
    flags = ["--edition=2024", "--crate-name", "time_domain_allocation_observer", "--target", TARGET,
             "--extern", "time_domain=/work/observer-deps/libtime_domain.rlib", "-L", "dependency=/work/observer-deps",
             "-C", "linker=/work/linker", *ENV.MSVC_FLAGS, *profile, *lint_flags(workspace)]
    report["direct_profile_flags"] = profile
    report["direct_target_flags"] = ENV.MSVC_FLAGS
    run("observer-clippy", ["clippy-driver", *flags, "--emit=metadata", "-o", "/work/observer.rmeta",
                            "/work/fixture/" + OBSERVER], readonly_dependencies=sealed)
    if PRODUCT.inventory(sealed) != dependency_inputs:
        raise Incomplete("Read-only observer dependencies changed during Clippy.")
    basename = "NevroTick-" + uuid.uuid4().hex + ".exe"
    run("observer-link", ["rustc", *flags, "-o", "/work/" + basename, "/work/fixture/" + OBSERVER],
        readonly_dependencies=sealed)
    binary = work / basename
    declared_pe = pe_header(binary)
    artifact = output / basename
    shutil.copyfile(binary, artifact); artifact.chmod(0o400)
    report["artifact"] = {"path": str(artifact), "sha256": PRODUCT.sha256(artifact), "pe": declared_pe}
    report["dependency_artifacts_sha256"] = dependency_inputs
    report["emitted_library"] = {"path": actual, "sha256": PRODUCT.sha256(actual_host)}
    if (PRODUCT.admit_product(ROOT)[1] != product_inputs
            or any(PRODUCT.sha256(ROOT / name) != checksum for name, checksum in extras.items())
            or PRODUCT.admit_vendor(vendor, policy) != vendor_inputs or PRODUCT.inventory(sealed) != dependency_inputs
            or PRODUCT.library_inventory(libraries) != library_inputs or PRODUCT.sha256(args.xwin_config) != config_hash
            or PRODUCT.sha256(args.xwin_inventory) != inventory_hash
            or ANALYSIS.check_inventory(base, specifications[0], stable)[0] != stable_inventory
            or PRODUCT.admit_tools(ROOT, toolchain, None, "windows") != tools):
        raise Incomplete("Source, compiler/std, dependency artifacts, vendor or Microsoft inputs drifted.")
    report["status"] = "passed"


NATIVE_SUPERVISOR = r'''# Test-only console supervision; no elevation or configuration writes.
[CmdletBinding()]
param([Parameter(Mandatory=$true)][string]$RequestPath)
Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'
$request = Get-Content -LiteralPath $RequestPath -Raw | ConvertFrom-Json
$destination = Split-Path -LiteralPath $RequestPath
$env:TEMP = $destination
$env:TMP = $destination
$report = [ordered]@{schema_version=1; status='incomplete'; registry_writes=$false;
    instrumentation='No AppVerifier/PageHeap configuration selected for requested-payload counting';
    started_utc=[DateTime]::UtcNow.ToString('o')}
function Inspect-ImageSettings([string]$ImageName, [string]$ImagePath) {
    $findings = @()
    foreach ($hive in @([Microsoft.Win32.RegistryHive]::LocalMachine, [Microsoft.Win32.RegistryHive]::CurrentUser)) {
        foreach ($view in @([Microsoft.Win32.RegistryView]::Registry64, [Microsoft.Win32.RegistryView]::Registry32)) {
            $base = [Microsoft.Win32.RegistryKey]::OpenBaseKey($hive, $view)
            try {
                foreach ($scope in @('Image File Execution Options', 'SilentProcessExit')) {
                    $name = 'SOFTWARE\Microsoft\Windows NT\CurrentVersion\' + $scope + '\' + $ImageName
                    $key = $base.OpenSubKey($name, $false)
                    if ($null -ne $key) {
                        try { $findings += @{hive=$hive.ToString();view=$view.ToString();scope=$scope;key_present=$true} }
                        finally { $key.Dispose() }
                    }
                }
                $key = $base.OpenSubKey('SOFTWARE\Microsoft\Windows NT\CurrentVersion\AppCompatFlags\Layers', $false)
                if ($null -ne $key) {
                    try {
                        if ($null -ne $key.GetValue($ImagePath, $null)) {
                            $findings += @{hive=$hive.ToString();view=$view.ToString();scope='AppCompatLayers';key_present=$true}
                        }
                    } finally { $key.Dispose() }
                }
            } finally { $base.Dispose() }
        }
    }
    $base = [Microsoft.Win32.RegistryKey]::OpenBaseKey([Microsoft.Win32.RegistryHive]::LocalMachine, [Microsoft.Win32.RegistryView]::Registry64)
    try {
        $key = $base.OpenSubKey('SYSTEM\CurrentControlSet\Control\Session Manager', $false)
        if ($null -eq $key) { throw 'Cannot inspect inherited Session Manager flags.' }
        try {
            $flags = $key.GetValue('GlobalFlag', 0)
            if (($flags -is [string] -and $flags -notmatch '^0*$') -or ($flags -isnot [string] -and $flags -ne 0)) {
                $findings += @{scope='SystemGlobalFlag';nonzero=$true}
            }
        } finally { $key.Dispose() }
    } finally { $base.Dispose() }
    return $findings
}
try {
    $image = Get-Item -LiteralPath $request.image_path
    if ($image.PSIsContainer -or $image.FullName.StartsWith('\\') -or
        $image.Name -cnotmatch '^NevroTick-[a-f0-9]{32}\.exe$' -or
        (($image.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0)) { throw 'Unadmitted native image.' }
    $ancestor = $image.Directory
    while ($null -ne $ancestor) {
        if (($ancestor.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0) { throw 'Redirected native image ancestor.' }
        $ancestor = $ancestor.Parent
    }
    if (@(Get-Process -Name $image.BaseName -ErrorAction SilentlyContinue).Count -ne 0) { throw 'Image basename is already active.' }
    $report.image_settings_before = @(Inspect-ImageSettings $image.Name $image.FullName)
    if ($report.image_settings_before.Count -ne 0) { throw 'Inherited image/instrumentation settings are not empty.' }
    $report.image_sha256_before = (Get-FileHash -LiteralPath $image.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($report.image_sha256_before -cne $request.image_sha256) { throw 'Native image differs from frozen build.' }
    $report.supervisor_sha256 = (Get-FileHash -LiteralPath $PSCommandPath -Algorithm SHA256).Hash.ToLowerInvariant()
    $report.request_sha256 = (Get-FileHash -LiteralPath $RequestPath -Algorithm SHA256).Hash.ToLowerInvariant()
    $report.os = @{version=[Environment]::OSVersion.Version.ToString();is_64_bit=[Environment]::Is64BitOperatingSystem;
        powershell=$PSVersionTable.PSVersion.ToString();clr=[Environment]::Version.ToString()}
    $report.runtime_files_sha256 = @{}
    $runtime = [Runtime.InteropServices.RuntimeEnvironment]::GetRuntimeDirectory()
    foreach ($file in @((Join-Path $runtime 'csc.exe'), (Join-Path $runtime 'csc.rsp'), (Join-Path $runtime 'csc.exe.config'), [object].Assembly.Location,
                       [Diagnostics.Process].Assembly.Location, [System.Management.Automation.PSObject].Assembly.Location,
                       [Microsoft.CSharp.CSharpCodeProvider].Assembly.Location,
                       [Diagnostics.Process]::GetCurrentProcess().MainModule.FileName)) {
        if (Test-Path -LiteralPath $file) {
            $report.runtime_files_sha256[$file] = (Get-FileHash -LiteralPath $file -Algorithm SHA256).Hash.ToLowerInvariant()
        }
    }
    $adapterPath = Join-Path $destination 'native-supervisor-adapter.dll'
    Add-Type -OutputAssembly $adapterPath -OutputType Library -TypeDefinition @'
using System;
using System.ComponentModel;
using System.Diagnostics;
using System.IO;
using System.Runtime.InteropServices;
using System.Security.Principal;
using System.Text;
using System.Threading;
using System.Threading.Tasks;

public static class NevroTickAllocationSupervisor {
    public sealed class Token {
        public string user_sid; public bool elevated; public bool administrator; public int elevation_type;
    }
    public sealed class Result {
        public int pid; public int? exit_code; public string failure; public string cleanup_failure;
        public bool gate_sent; public bool cleanup_complete; public bool output_overflow; public bool process_handle_released;
        public bool streams_released;
        public string stdin_encoding; public string stdin_preamble_hex;
        public uint? active_processes_before_handle_release; public uint? active_processes_after_cleanup;
        public uint? active_processes_before_cleanup; public Token parent_token; public Token child_token;
        public string stdout = ""; public string stderr = "";
    }
    [StructLayout(LayoutKind.Sequential)] struct BasicLimit {
        public long processTime, jobTime; public uint flags;
        public UIntPtr minWorking, maxWorking; public uint activeLimit;
        public UIntPtr affinity; public uint priority, scheduling;
    }
    [StructLayout(LayoutKind.Sequential)] struct IOCount {
        public ulong readOps, writeOps, otherOps, readBytes, writeBytes, otherBytes;
    }
    [StructLayout(LayoutKind.Sequential)] struct ExtendedLimit {
        public BasicLimit basic; public IOCount io;
        public UIntPtr processMemory, jobMemory, peakProcessMemory, peakJobMemory;
    }
    [StructLayout(LayoutKind.Sequential)] struct Accounting {
        public long userTime, kernelTime, periodUserTime, periodKernelTime;
        public uint faults, totalProcesses, activeProcesses, terminatedProcesses;
    }
    [DllImport("kernel32.dll", CharSet=CharSet.Unicode, SetLastError=true)]
    static extern IntPtr CreateJobObjectW(IntPtr attributes, string name);
    [DllImport("kernel32.dll", SetLastError=true)]
    static extern bool SetInformationJobObject(IntPtr job, int kind, ref ExtendedLimit info, uint length);
    [DllImport("kernel32.dll", SetLastError=true)]
    static extern bool QueryInformationJobObject(IntPtr job, int kind, out Accounting info, uint length, IntPtr returned);
    [DllImport("kernel32.dll", SetLastError=true)] static extern bool AssignProcessToJobObject(IntPtr job, IntPtr process);
    [DllImport("kernel32.dll", SetLastError=true)] static extern bool TerminateJobObject(IntPtr job, uint code);
    [DllImport("kernel32.dll", SetLastError=true)] static extern bool CloseHandle(IntPtr handle);
    [DllImport("advapi32.dll", SetLastError=true)] static extern bool OpenProcessToken(IntPtr process, uint rights, out IntPtr token);
    [DllImport("advapi32.dll", SetLastError=true)] static extern bool GetTokenInformation(IntPtr token, int kind, out int value, int size, out int returned);
    static void Check(bool ok) { if (!ok) throw new Win32Exception(Marshal.GetLastWin32Error()); }
    // Query only this owned job. A failed query never supplies an empty-job observation.
    static uint ActiveProcesses(IntPtr job) {
        Accounting accounting;
        Check(QueryInformationJobObject(job, 1, out accounting, (uint)Marshal.SizeOf(typeof(Accounting)), IntPtr.Zero));
        return accounting.activeProcesses;
    }
    // Retain the last actual count; process references may delay native accounting.
    static bool WaitForEmptyJob(IntPtr job, out uint last) {
        Stopwatch watch = Stopwatch.StartNew();
        while (true) {
            last = ActiveProcesses(job);
            if (last == 0) return true;
            if (watch.ElapsedMilliseconds >= 5000) return false;
            Thread.Sleep(25);
        }
    }
    // Cleanup diagnostics supplement, and never erase, the first execution failure.
    static void CleanupFailure(Result result, Exception exception) {
        string message = exception.GetType().Name + ": " + exception.Message;
        result.cleanup_failure = result.cleanup_failure == null ? message : result.cleanup_failure + "; " + message;
        if (result.failure == null) result.failure = "Cleanup: " + message;
    }
    // Never dispose a reader concurrently with its blocking capture task. A
    // missed drain deadline remains incomplete even if process teardown succeeds.
    static bool ReleaseReader(StreamReader reader, Task capture, Result result) {
        if (reader == null) return false;
        if (capture != null && !capture.IsCompleted) {
            CleanupFailure(result, new TimeoutException("Reader still active after bounded drain."));
            return false;
        }
        try { reader.Dispose(); return true; }
        catch (Exception exception) { CleanupFailure(result, exception); return false; }
    }
    // Process.Dispose deliberately leaves redirected streams open on Framework.
    // Release all three explicitly, without short-circuiting another release.
    static bool ReleaseStreams(StreamWriter input, StreamReader stdout, StreamReader stderr,
                               Task output, Task error, Result result) {
        bool inputReleased = false;
        if (input != null) {
            try { input.Dispose(); inputReleased = true; }
            catch (Exception exception) { CleanupFailure(result, exception); }
        }
        bool outputReleased = ReleaseReader(stdout, output, result);
        bool errorReleased = ReleaseReader(stderr, error, result);
        return inputReleased && outputReleased && errorReleased;
    }
    static Token ObserveToken(IntPtr process) {
        // TOKEN_QUERY | TOKEN_DUPLICATE: WindowsPrincipal.IsInRole internally
        // duplicates a primary token for identification-only membership checks.
        // Only the supervisor and its owned child are observed; no impersonation
        // or token/privilege adjustment is performed by this fixture.
        IntPtr token; Check(OpenProcessToken(process, 8 | 2, out token));
        try {
            int elevation, type, returned;
            Check(GetTokenInformation(token, 20, out elevation, 4, out returned));
            if (returned != 4) throw new InvalidOperationException("Unexpected token size.");
            Check(GetTokenInformation(token, 18, out type, 4, out returned));
            if (returned != 4) throw new InvalidOperationException("Unexpected token size.");
            using (WindowsIdentity identity = new WindowsIdentity(token)) {
                return new Token {user_sid=identity.User.Value, elevated=elevation!=0, elevation_type=type,
                    administrator=new WindowsPrincipal(identity).IsInRole(WindowsBuiltInRole.Administrator)};
            }
        } finally { Check(CloseHandle(token)); }
    }
    sealed class Capture {
        public int overflow;
        public string Read(StreamReader stream) {
            char[] buffer = new char[4096]; StringBuilder text = new StringBuilder(); int count;
            while ((count=stream.Read(buffer, 0, buffer.Length)) != 0) {
                if (text.Length + count > 1048576) { Interlocked.Exchange(ref overflow, 1); return text.ToString(); }
                text.Append(buffer, 0, count);
            }
            return text.ToString();
        }
    }
    public static Result Run(string image, string directory, string[] arguments) {
        Result result = new Result(); Process process = null; IntPtr job = IntPtr.Zero;
        Task<string> output = null, error = null; Capture capture = new Capture(); bool assigned = false;
        StreamWriter input = null; StreamReader stdout = null, stderr = null;
        try {
            using (Process parent = Process.GetCurrentProcess()) result.parent_token = ObserveToken(parent.Handle);
            if (result.parent_token.elevated || result.parent_token.administrator) throw new InvalidOperationException("Elevated supervisor token.");
            // Framework Process starts its stdin StreamWriter with AutoFlush=true,
            // which emits this preamble before our later G. Admit the exact fixture
            // encoding; never change console settings or accept arbitrary prefixes.
            Encoding inputEncoding = Console.InputEncoding;
            result.stdin_encoding = inputEncoding.WebName;
            result.stdin_preamble_hex = BitConverter.ToString(inputEncoding.GetPreamble());
            if (result.stdin_encoding != "utf-8" || result.stdin_preamble_hex != "EF-BB-BF")
                throw new InvalidOperationException("Unsupported fixture stdin encoding.");
            job = CreateJobObjectW(IntPtr.Zero, null); Check(job != IntPtr.Zero);
            ExtendedLimit limits = new ExtendedLimit(); limits.basic.flags = 0x2000;
            Check(SetInformationJobObject(job, 9, ref limits, (uint)Marshal.SizeOf(typeof(ExtendedLimit))));
            ProcessStartInfo start = new ProcessStartInfo(image, String.Join(" ", arguments));
            start.UseShellExecute = false; start.CreateNoWindow = true; start.WorkingDirectory = directory;
            start.RedirectStandardInput = true; start.RedirectStandardOutput = true; start.RedirectStandardError = true;
            start.StandardOutputEncoding = new UTF8Encoding(false, true); start.StandardErrorEncoding = new UTF8Encoding(false, true);
            start.EnvironmentVariables.Clear(); string windows = Environment.GetFolderPath(Environment.SpecialFolder.Windows);
            start.EnvironmentVariables["SystemRoot"] = windows; start.EnvironmentVariables["WINDIR"] = windows;
            start.EnvironmentVariables["PATH"] = Path.Combine(windows, "System32");
            start.EnvironmentVariables["TEMP"] = directory; start.EnvironmentVariables["TMP"] = directory;
            process = new Process(); process.StartInfo = start;
            if (!process.Start()) throw new InvalidOperationException("Owned process did not start.");
            result.pid = process.Id;
            input = process.StandardInput; stdout = process.StandardOutput; stderr = process.StandardError;
            output = Task.Run(() => capture.Read(stdout));
            error = Task.Run(() => capture.Read(stderr));
            Check(AssignProcessToJobObject(job, process.Handle)); assigned = true;
            result.child_token = ObserveToken(process.Handle);
            if (result.child_token.elevated || result.child_token.administrator ||
                result.child_token.user_sid != result.parent_token.user_sid) throw new InvalidOperationException("Unexpected child token.");
            // The observer has at most the BOM and still needs G; token capture
            // therefore precedes every measured/product operation.
            input.Write('G'); input.Flush(); input.Close(); result.gate_sent = true;
            Stopwatch watch = Stopwatch.StartNew();
            while (!process.WaitForExit(25)) {
                if (watch.ElapsedMilliseconds > 30000) throw new TimeoutException("Owned child deadline exceeded.");
                if (Volatile.Read(ref capture.overflow) != 0) throw new InvalidOperationException("Owned child output budget exceeded.");
            }
            result.exit_code = process.ExitCode;
            if (!Task.WaitAll(new Task[] {output,error}, 5000)) throw new TimeoutException("Output drain deadline exceeded.");
            result.stdout = output.GetAwaiter().GetResult(); result.stderr = error.GetAwaiter().GetResult();
            result.output_overflow = Volatile.Read(ref capture.overflow) != 0;
            if (result.output_overflow) throw new InvalidOperationException("Owned child output budget exceeded.");
            result.streams_released = ReleaseStreams(input, stdout, stderr, output, error, result);
            if (!result.streams_released) throw new InvalidOperationException("Owned redirected streams were not released.");
            result.active_processes_before_handle_release = ActiveProcesses(job);
            // An exited process remains in ActiveProcesses until all references
            // are released. Preserve its exit/output before closing our handle.
            process.Dispose(); process = null; result.process_handle_released = true;
            uint remaining;
            bool empty = WaitForEmptyJob(job, out remaining);
            result.active_processes_before_cleanup = remaining;
            if (!empty) throw new TimeoutException("Owned job did not become empty after process-handle release.");
        } catch (Exception exception) {
            if (result.failure == null) result.failure = exception.GetType().Name + ": " + exception.Message;
        }
        finally {
            bool cleanupOk = true;
            if (process != null) {
                try {
                    // Failure cleanup closes every owned writer, including a
                    // possible descendant, before waiting for reader drains.
                    if (assigned) Check(TerminateJobObject(job, 1));
                    else if (!process.HasExited) process.Kill();
                    if (!process.WaitForExit(5000)) throw new TimeoutException("Owned child cleanup deadline exceeded.");
                    result.exit_code = process.ExitCode;
                    if (output != null && error != null) {
                        if (!Task.WaitAll(new Task[] {output,error}, 5000)) throw new TimeoutException("Cleanup output drain deadline exceeded.");
                        result.stdout = output.GetAwaiter().GetResult(); result.stderr = error.GetAwaiter().GetResult();
                    }
                    result.output_overflow = Volatile.Read(ref capture.overflow) != 0;
                } catch (Exception exception) { cleanupOk = false; CleanupFailure(result, exception); }
                finally {
                    result.streams_released = ReleaseStreams(input, stdout, stderr, output, error, result);
                    if (!result.streams_released) cleanupOk = false;
                    try { process.Dispose(); result.process_handle_released = true; }
                    catch (Exception exception) { cleanupOk = false; CleanupFailure(result, exception); }
                    process = null;
                }
            }
            if (job != IntPtr.Zero) {
                try {
                    // This recovery may kill remaining owned members, but cannot
                    // change the independently retained pre-cleanup observation.
                    if (ActiveProcesses(job) != 0) Check(TerminateJobObject(job, 1));
                    uint remaining;
                    bool empty = WaitForEmptyJob(job, out remaining);
                    result.active_processes_after_cleanup = remaining;
                    if (!empty) throw new TimeoutException("Owned job cleanup deadline exceeded.");
                } catch (Exception exception) { cleanupOk = false; CleanupFailure(result, exception); }
                finally {
                    if (!CloseHandle(job)) {
                        cleanupOk = false; CleanupFailure(result, new Win32Exception(Marshal.GetLastWin32Error()));
                    }
                }
            }
            result.cleanup_complete = cleanupOk && result.streams_released && result.process_handle_released
                && result.active_processes_after_cleanup == 0;
        }
        return result;
    }
}
'@
    $report.adapter_sha256 = (Get-FileHash -LiteralPath $adapterPath -Algorithm SHA256).Hash.ToLowerInvariant()
    Add-Type -Path $adapterPath
    $arguments = @($request.claims.candidate_sha256, $request.claims.observer_source_sha256,
        $request.claims.build_receipt_sha256, $request.claims.profile)
    foreach ($argument in $arguments[0..2]) { if ($argument -cnotmatch '^[a-f0-9]{64}$') { throw 'Unadmitted native identity claim.' } }
    if ($arguments[3] -cnotin @('debug','release','verify-release')) { throw 'Unadmitted native profile.' }
    $imageLock = [IO.File]::Open($image.FullName, [IO.FileMode]::Open, [IO.FileAccess]::Read, [IO.FileShare]::Read)
    try {
        $report.execution = [NevroTickAllocationSupervisor]::Run($image.FullName, $destination, $arguments)
        $report.image_sha256_after = (Get-FileHash -LiteralPath $image.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
    } finally { $imageLock.Dispose() }
    $report.image_settings_after = @(Inspect-ImageSettings $image.Name $image.FullName)
    foreach ($file in @($report.runtime_files_sha256.Keys)) {
        if ((Get-FileHash -LiteralPath $file -Algorithm SHA256).Hash.ToLowerInvariant() -cne $report.runtime_files_sha256[$file]) {
            throw 'PowerShell/compiler/runtime input changed during observation.'
        }
    }
    if ((Get-FileHash -LiteralPath $adapterPath -Algorithm SHA256).Hash.ToLowerInvariant() -cne $report.adapter_sha256) {
        throw 'Compiled native supervisor adapter changed during observation.'
    }
    $report.runtime_inputs_unchanged = $true
    if ($report.image_settings_after.Count -ne 0 -or $report.image_sha256_after -cne $report.image_sha256_before -or
        $null -ne $report.execution.failure -or $null -ne $report.execution.cleanup_failure -or $report.execution.exit_code -ne 0 -or
        -not $report.execution.gate_sent -or -not $report.execution.cleanup_complete) { throw 'Native observation incomplete; raw result retained.' }
    $report.status = 'completed'
} catch { $report.error = $_.Exception.Message }
$report.completed_utc = [DateTime]::UtcNow.ToString('o')
$json = $report | ConvertTo-Json -Depth 16
[IO.File]::WriteAllText((Join-Path $destination 'native-supervision.json'), $json, [Text.UTF8Encoding]::new($false))
if ($report.status -eq 'completed') { exit 0 } else { exit 1 }
'''


def validate_native_result(value, image_hash, supervisor_hash, request_hash):
    """Reject premature/bad-token/unbounded/unclean execution even with an exit zero."""
    if (type(value.get("schema_version")) is not int or value["schema_version"] != 1 or value.get("status") != "completed"
            or value.get("registry_writes") is not False or value.get("image_settings_before") != []
            or value.get("image_settings_after") != [] or value.get("image_sha256_before") != image_hash
            or value.get("image_sha256_after") != image_hash or value.get("supervisor_sha256") != supervisor_hash
            or value.get("request_sha256") != request_hash or value.get("os", {}).get("is_64_bit") is not True
            or value.get("runtime_inputs_unchanged") is not True
            or not re.fullmatch("[a-f0-9]{64}", value.get("adapter_sha256", ""))
            or not isinstance(value.get("runtime_files_sha256"), dict) or not value["runtime_files_sha256"]):
        raise Incomplete("Native identity, inherited settings or completed observation is incomplete.")
    result = value.get("execution", {})
    if (type(result.get("pid")) is not int or result["pid"] <= 0
            or type(result.get("exit_code")) is not int or result["exit_code"] != 0
            or result.get("failure") is not None or result.get("gate_sent") is not True
            or result.get("cleanup_complete") is not True or result.get("output_overflow") is not False
            or result.get("streams_released") is not True
            or result.get("cleanup_failure") is not None or result.get("process_handle_released") is not True
            or result.get("stdin_encoding") != "utf-8" or result.get("stdin_preamble_hex") != "EF-BB-BF"
            or type(result.get("active_processes_before_handle_release")) is not int
            or not 0 <= result["active_processes_before_handle_release"] <= 0xffffffff
            or type(result.get("active_processes_before_cleanup")) is not int
            or result["active_processes_before_cleanup"] != 0
            or type(result.get("active_processes_after_cleanup")) is not int
            or result["active_processes_after_cleanup"] != 0 or result.get("stderr") != ""):
        raise Incomplete("Native child failed, skipped the gate, exceeded a bound or did not clean up.")
    parent, child = result.get("parent_token", {}), result.get("child_token", {})
    for token in (parent, child):
        if (not re.fullmatch(r"S-1-\d+(?:-\d+)+", token.get("user_sid", ""))
                or token.get("elevated") is not False or token.get("administrator") is not False
                or type(token.get("elevation_type")) is not int or token["elevation_type"] not in (1, 3)):
            raise Incomplete("Native child or supervisor did not have an admitted ordinary token.")
    if child["user_sid"] != parent["user_sid"]:
        raise Incomplete("Native child token belongs to a different user.")
    if not isinstance(result.get("stdout"), str):
        raise Incomplete("Native observer output is absent.")
    return decode(result["stdout"].encode("utf-8"))


def native(args, report):
    """Run only the frozen owned Windows observer, with no system configuration effects."""
    if args.build_receipt is None or args.native_dir is None:
        raise Incomplete("Native lane needs an explicit frozen build receipt and fresh Windows directory.")
    receipt_hash = PRODUCT.sha256(args.build_receipt)
    built = decode(args.build_receipt.read_bytes())
    expected = PRODUCT.admit_product(ROOT)[1]
    if (built.get("schema_version") != 1 or built.get("status") != "passed" or built.get("lane") != "build"
            or built.get("purpose") != "test_only_time_domain_allocator_requests"
            or built.get("product_source_sha256") != expected or built.get("candidate_sha256") != canonical_digest(expected)
            or built.get("extra_inputs_sha256") != {name: PRODUCT.sha256(ROOT / name) for name in EXTRA_INPUTS}
            or built.get("direct_profile_flags") != profile_flags(built.get("profile"))
            or built.get("direct_target_flags") != ENV.MSVC_FLAGS
            or [lane.get("lane") for lane in built.get("lanes", [])] != [
                "boundary-control", "compiler-version", "observer-format", "library-build", "sealed-dependency-boundary",
                "observer-clippy", "observer-link"]
            or any(lane.get("failure") is not None or lane.get("exit_code") != 0 for lane in built["lanes"])):
        raise Incomplete("Native input is not a successful matching guarded observer build receipt.")
    original = Path(built["artifact"]["path"])
    if PRODUCT.sha256(original) != built["artifact"]["sha256"] or pe_header(original) != built["artifact"]["pe"]:
        raise Incomplete("Built observer artifact has changed.")
    directory = args.native_dir
    windows_path(directory)
    if directory.exists() or directory.is_symlink() or directory.parent != directory.parent.resolve(strict=True):
        raise Incomplete("Use a fresh canonical local Windows fixture directory.")
    directory.mkdir()
    image = directory / ("NevroTick-" + uuid.uuid4().hex + ".exe")
    shutil.copyfile(original, image)
    image_hash = PRODUCT.sha256(image)
    if image_hash != built["artifact"]["sha256"]:
        raise Incomplete("Native observer copy differs from its actual built artifact.")
    claims = {"candidate_sha256": built["candidate_sha256"], "observer_source_sha256": built["extra_inputs_sha256"][OBSERVER],
              "build_receipt_sha256": receipt_hash, "profile": built["profile"]}
    request = directory / "native-request.json"
    request.write_text(json.dumps({"image_path": windows_path(image), "image_sha256": image_hash, "claims": claims}, indent=2) + "\n")
    supervisor = directory / "native-supervisor.ps1"
    supervisor.write_text(NATIVE_SUPERVISOR, encoding="utf-8")
    hashes = {"image": image_hash, "request": PRODUCT.sha256(request), "supervisor": PRODUCT.sha256(supervisor)}
    powershell = Path("/mnt/c/Windows/System32/WindowsPowerShell/v1.0/powershell.exe")
    report.update(build_receipt_sha256=receipt_hash, native_directory=str(directory), claims=claims,
                  powershell_sha256=PRODUCT.sha256(powershell), native_inputs_sha256=hashes)
    result = ENV.run([str(powershell), "-NoLogo", "-NoProfile", "-NonInteractive", "-File", windows_path(supervisor),
                      "-RequestPath", windows_path(request)], cwd=directory, timeout=90,
                     environment={"PATH": "/usr/bin:/bin", "WSLENV": ""})
    report["native_command"] = result
    report["native_execution"] = "attempted; see actual supervision and child exit records"
    supervision = directory / "native-supervision.json"
    if supervision.is_file():
        report["native_supervision"] = decode(supervision.read_bytes())
    # Raw failure and the partial native receipt remain retained before raising.
    ENV.require_ok(result)
    value = validate_native_result(report["native_supervision"], image_hash, hashes["supervisor"], hashes["request"])
    report["observer_output"] = value
    report["assessment"] = validate_observation(value, claims)
    if (PRODUCT.sha256(image) != image_hash or PRODUCT.sha256(original) != image_hash
            or PRODUCT.sha256(request) != hashes["request"] or PRODUCT.sha256(supervisor) != hashes["supervisor"]
            or PRODUCT.sha256(args.build_receipt) != receipt_hash or PRODUCT.admit_product(ROOT)[1] != expected
            or built["extra_inputs_sha256"] != {name: PRODUCT.sha256(ROOT / name) for name in EXTRA_INPUTS}
            or PRODUCT.sha256(powershell) != report["powershell_sha256"]):
        raise Incomplete("Observer, claims, helper, candidate, build receipt or native tool changed during execution.")
    report["native_execution"] = "completed ordinary-token Windows allocator observation; no product clock/network/IPC/service effect"
    report["status"] = "passed"


def main(argv=None):
    """Select one explicit lane; preserve incomplete receipts and never fetch tools."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--lane", choices=("build", "native"), required=True)
    parser.add_argument("--profile", choices=tuple(PROFILE_DIRECTORY), default="release")
    parser.add_argument("--output-dir", type=Path, required=True)
    for name in ("vendor-dir", "toolchain-dir", "xwin-config", "xwin-inventory", "analysis-installation",
                 "build-receipt", "native-dir"):
        parser.add_argument("--" + name, type=Path)
    args = parser.parse_args(argv)
    report = {"schema_version": 1, "purpose": "test_only_time_domain_allocator_requests",
              "lane": args.lane, "status": "incomplete", "started_unix": time.time(),
              "native_execution": "not performed"}
    destination = None
    try:
        os.umask(0o077)
        destination = new_output(args.output_dir)
        (build if args.lane == "build" else native)(args, report)
    except (Incomplete, ANALYSIS.Incomplete, OSError, ValueError, KeyError, TypeError, struct.error) as error:
        report["status"], report["error"] = "incomplete", str(error)
    report["completed_unix"] = time.time()
    if destination:
        (destination / "verification.json").write_text(json.dumps(report, indent=2) + "\n")
    print(report["status"].upper() + ": test-only time-domain allocation " + args.lane)
    if report.get("error"):
        print(report["error"], file=sys.stderr)
    return 0 if report["status"] == "passed" else 1


if __name__ == "__main__":
    raise SystemExit(main())
