#!/usr/bin/env python3
"""Qualify the admitted NTP header and outer-framing portable analysis scopes.

No downloads, installations, host Cargo fallback or product dependency changes.
The full candidate is sealed; cargo-mutants mutates only its owned temporary copy.
The separate test-only libFuzzer graph requires exact source/native-grant admission.
Deterministic generated tests remain a separate evidence layer.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import shutil
import sys
import time
import tomllib

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[1]
PACKAGE = "ntp-wire"
HEADER = "crates/ntp-wire/src/header.rs"
DATAGRAM = "crates/ntp-wire/src/datagram.rs"
SUBJECTS = {"header": HEADER, "datagram": DATAGRAM}
POLICY = "policy/ntp-wire-analysis.json"
FUZZ_CARGO_CONFIG = ('[source.crates-io]\nreplace-with="fuzz-reviewed"\n'
                     '[source.fuzz-reviewed]\ndirectory="/work/fuzz-vendor"\n')


def load_module(name: str, path: Path):
    """Load reviewed repository tooling without running its CLI entry point."""
    specification = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


COMMON = load_module("wire_portable_shared", ROOT / "scripts/portable-analysis.py")
ANALYSIS, PRODUCT, PREPARE = COMMON.ANALYSIS, COMMON.PRODUCT, COMMON.PREPARE
RUNTIME, Incomplete = COMMON.RUNTIME, COMMON.Incomplete
SCOPE = COMMON.Scope(PACKAGE, tuple(SUBJECTS.items()), POLICY)


def read_policy(root: Path) -> dict:
    """Reject changed scope, thresholds or resource caps before running tools."""
    policy = json.loads((root / POLICY).read_text())
    required = {"schema_version": 2, "package": PACKAGE, "critical_files": list(SUBJECTS.values()),
                "coverage_minimum_line_percent": 90, "mutation_minimum_viable_killed_percent": 95,
                "maximum_mutants": 512, "coverage_wall_seconds": 300,
                "mutation_wall_seconds": 1200, "mutation_build_seconds": 90,
                "mutation_test_seconds": 60, "jobs": 1, "test_threads": 2}
    if "critical_file" in policy or any(type(policy.get(key)) is not type(value) or policy.get(key) != value
           for key, value in required.items()):
        raise Incomplete("Unreviewed NTP analysis scope, threshold or budget.")
    fuzz = policy.get("coverage_guided_fuzzing", {})
    if fuzz.get("status") != "admitted-test-only-source" or fuzz.get("fuzzer_flags") != list(RUNTIME.SANITIZER_FLAGS):
        raise Incomplete("The fixed test-only libFuzzer source/profile admission is absent.")
    manifest = tomllib.loads((root / "policy/verification-manifest.toml").read_text())
    for filename in SUBJECTS.values():
        if manifest.get("critical_files", []).count(filename) != 1:
            raise Incomplete("A parser is absent or duplicated in the reviewed critical-file inventory.")
        if not any(module.get("package") == PACKAGE and filename in module.get("files", [])
                   for module in manifest.get("security_modules", [])):
            raise Incomplete("A parser is absent from the security-module inventory.")
    return policy


def select_subjects(subject: str) -> dict[str, str]:
    """Return only the fixed reviewed file mapping; all includes both owners."""
    if subject == "all":
        return dict(SUBJECTS)
    if subject not in SUBJECTS:
        raise Incomplete("Unknown NTP analysis subject.")
    return {subject: SUBJECTS[subject]}


def selected_file(filename: str) -> str:
    """Delegate unchanged wire evidence to the explicit immutable scope."""
    return SCOPE.selected_file(filename)


def require_unchanged(root: Path, identity: dict[str, str]) -> None:
    """Reject changed, removed, linked or replaced admitted input bytes."""
    if any(PRODUCT.sha256(root / name) != expected for name, expected in identity.items()):
        raise Incomplete("Source, policy or analysis input changed during execution.")


def require_current_candidate(root: Path, identity: dict[str, str]) -> None:
    """Re-admit the complete candidate set so additions cannot evade old hashes."""
    if PRODUCT.admit_product(root)[1] != identity:
        raise Incomplete("The complete candidate input set changed during analysis.")


def coverage_counts(report: dict, expected_file: str) -> dict:
    """Delegate unchanged wire evidence to the explicit immutable scope."""
    return COMMON.coverage_counts(SCOPE, report, expected_file)


def mutation_inventory(items: list, maximum: int, expected_file: str = HEADER) -> dict[str, dict]:
    """Delegate unchanged wire evidence to the explicit immutable scope."""
    return COMMON.mutation_inventory(SCOPE, items, maximum, expected_file)


def mutation_counts(report: dict, expected: dict[str, dict], expected_file: str = HEADER) -> dict:
    """Delegate unchanged wire evidence to the explicit immutable scope."""
    return COMMON.mutation_counts(SCOPE, report, expected, expected_file)


def combined_mutant_count(inventories: dict[str, dict], maximum: int) -> int:
    """Delegate unchanged wire evidence to the explicit immutable scope."""
    return COMMON.combined_mutant_count(inventories, maximum)


def remaining_timeout(deadline: float, maximum: int) -> int:
    """Delegate unchanged wire evidence to the explicit immutable scope."""
    return COMMON.remaining_timeout(deadline, maximum)


def test_count(output: str) -> int:
    """Delegate unchanged wire evidence to the explicit immutable scope."""
    return COMMON.test_count(output)


def tool_path(root: Path, name: str) -> Path:
    """Select only the installed hash-pinned extension; never acquire a tool."""
    found = shutil.which(name)
    if not found:
        raise Incomplete(f"Missing installed analysis extension: {name}.")
    path = Path(found).resolve(strict=True)
    entry = next(item for item in json.loads((root / "policy/tool-manifest.json").read_text())["tools"]
                 if item["name"] == name)
    PRODUCT.ENV.require_admitted_binary(path, entry["binary_sha256"], name)
    return path


def privacy_error(result: dict, expected: str) -> None:
    """Require the intended compiler privacy diagnostic, not any compile failure."""
    codes = set(re.findall(r"error\[(E\d+)\]", result.get("output", "")))
    if (result.get("failure") or result.get("exit_code") == 0 or codes != {expected}):
        raise Incomplete("External consumer did not fail for its intended privacy restriction.")


def contract_examples() -> dict[str, tuple[str, str | None]]:
    """Keep independent legal, privacy and borrow specimens for both wire owners."""
    setup = ('let Ok(limits) = ntp_wire::FrameLimits::new(76, 1) else { return; }; '
             'let mut input = vec![0u8; 76]; input[51] = 28; ')
    frame = ('let Ok(frame) = ntp_wire::frame_no_legacy_mac_datagram(&input, limits) '
             'else { return; }; ')
    field = 'let Some(mut field) = frame.extensions().next() else { return; }; '
    return {
        "legal": (
            'fn main() { let Ok(header) = ntp_wire::decode_fixed_header(&[0; 48]) else { return; }; '
            'let _ = (header.version(), header.receive_timestamp().seconds()); '
            + setup + frame +
            'let _ = (frame.header().version(), frame.extension_count()); '
            'for field in frame.extensions() { let _ = (field.field_type(), field.bytes(), field.body()); } }', None),
        "raw_header_construction": (
            'fn main() { let _ = ntp_wire::RawHeader { bytes: [0; 48] }; }', "E0451"),
        "raw_header_mutation": (
            'fn main() { let Ok(mut header) = ntp_wire::decode_fixed_header(&[0; 48]) else { return; }; '
            'header.bytes[0] = 1; }', "E0616"),
        "timestamp_construction": (
            'fn main() { let _ = ntp_wire::NtpTimestamp { bytes: [0; 8] }; }', "E0451"),
        "frame_limits_construction": (
            'fn main() { let _ = ntp_wire::FrameLimits { max_datagram_bytes: 48, max_extension_fields: 0 }; }',
            "E0451"),
        "raw_datagram_construction": (
            'fn main() { let Ok(header) = ntp_wire::decode_fixed_header(&[0; 48]) else { return; }; '
            'let _ = ntp_wire::RawDatagram { header, tail: &[], count: 0 }; }', "E0451"),
        "raw_extension_construction": (
            'fn main() { let _ = ntp_wire::RawExtension { bytes: &[0; 28] }; }', "E0451"),
        "extension_fields_construction": (
            'fn main() { let _ = ntp_wire::ExtensionFields { remaining: &[] }; }', "E0451"),
        "frame_limits_mutation": (
            'fn main() { let Ok(mut limits) = ntp_wire::FrameLimits::new(48, 0) else { return; }; '
            'limits.max_extension_fields = 99; }', "E0616"),
        "raw_datagram_mutation": (
            'fn main() { ' + setup + frame.replace('Ok(frame)', 'Ok(mut frame)') + 'frame.count = 99; }',
            "E0616"),
        "raw_extension_mutation": (
            'fn main() { ' + setup + frame + field + 'field.bytes = &[]; }', "E0616"),
        "live_input_borrow_mutation": (
            'fn main() { ' + setup + frame + field + 'input[0] = 1; let _ = field.bytes(); }', "E0502"),
        "datagram_borrow_escape": (
            'fn escape() -> Result<ntp_wire::RawDatagram<\'static>, ntp_wire::FrameError> { '
            'let input = [0u8; 48]; let limits = ntp_wire::FrameLimits::new(48, 0)?; '
            'ntp_wire::frame_no_legacy_mac_datagram(&input, limits) } fn main() {}', "E0515"),
        "frame_limits_default": (
            'fn main() { let _: ntp_wire::FrameLimits = Default::default(); }', "E0277"),
        "raw_datagram_debug": (
            'fn main() { ' + setup + frame + 'let _ = format!("{frame:?}"); }', "E0277"),
        "raw_extension_debug": (
            'fn main() { ' + setup + frame + field + 'let _ = format!("{field:?}"); }', "E0277"),
        "extension_fields_debug": (
            'fn main() { ' + setup + frame
            + 'let fields = frame.extensions(); let _ = format!("{fields:?}"); }', "E0277"),
    }


def fuzz_seeds() -> dict[str, bytes]:
    """Declare bounded literal word-framing neighbors independently of the parser."""
    def extension(field_type: int, length: int) -> bytes:
        """Encode one finite fixture field; lengths are this helper's trusted literals."""
        return field_type.to_bytes(2, "big") + length.to_bytes(2, "big") + bytes(length - 4)

    header = bytes(48)
    seeds = {"length-" + str(length): bytes(length) for length in (0, 1, 47, 48, 49, 4096)}
    seeds.update({"value-" + str(value): bytes([value]) * 48 for value in (0, 1, 127, 128, 255)})
    seeds["single-28"] = header + extension(0xabcd, 28)
    seeds["chain-16-28"] = header + extension(0x8001, 16) + extension(0x0404, 28)
    for count in (63, 64, 65):
        seeds["fields-" + str(count)] = header + extension(0x8001, 16) * (count - 1) + extension(0x0404, 28)
    seeds["malformed-final-16"] = header + extension(0xabcd, 16)
    seeds["malformed-good-prefix-tail"] = header + extension(0xabcd, 28) + b"\x00"
    for length in (4092, 4096):
        seeds["byte-edge-" + str(length)] = header + extension(0xabcd, length - 48)
    return seeds


def fuzz_metadata(metadata: dict, policy: dict) -> dict:
    """Check exact GNU-reachable feature/build closure before dependency code runs."""
    packages = {item["id"]: item for item in metadata["packages"]}
    nodes = {item["id"]: item for item in metadata["resolve"]["nodes"]}
    reachable, pending = set(), [metadata["resolve"]["root"]]
    while pending:
        key = pending.pop()
        if key in reachable:
            continue
        if key not in nodes or key not in packages:
            raise Incomplete("Fuzz metadata has an unresolved graph node.")
        reachable.add(key)
        pending.extend(item["pkg"] for item in nodes[key]["deps"])
    selected = policy["coverage_guided_fuzzing"]
    expected = {**selected["gnu_reachable_features"], "ntp-wire": [], "ntp-wire-fuzz": []}
    actual = {packages[key]["name"]: sorted(nodes[key]["features"]) for key in reachable}
    if len(actual) != len(reachable) or actual != expected:
        raise Incomplete("The resolved GNU fuzz feature closure differs from its exact admission.")
    versions = {item["name"]: item["version"] for item in selected["locked_registry_packages"]}
    if any(packages[key]["version"] != versions.get(packages[key]["name"], "0.0.0")
           for key in reachable):
        raise Incomplete("The resolved fuzz graph has an unreviewed package version.")
    return {"target": "x86_64-unknown-linux-gnu", "reachable_features": actual}


def admit_fuzz_sources(base: Path, policy: dict) -> None:
    """Check archive/source maps, every native file and its complete selected notice."""
    selected = policy["coverage_guided_fuzzing"]
    if PRODUCT.sha256(ROOT / selected["llvm_notice"]) != selected["llvm_notice_sha256"]:
        raise Incomplete("The selected complete LLVM-exception notice changed.")
    if PRODUCT.sha256(ROOT / selected["target_source"]) != selected["target_source_sha256"]:
        raise Incomplete("The independent fuzz target changed without source admission.")
    control = base / "common/controls-source/fuzz/Cargo.lock"
    if PRODUCT.sha256(control) != selected["control_lock_sha256"]:
        raise Incomplete("The admitted libFuzzer control graph changed.")
    locked = [item for item in tomllib.loads(control.read_text())["package"] if "source" in item]
    recorded = selected["locked_registry_packages"]
    if [{key: item[key] for key in ("name", "version", "source", "checksum")} for item in locked] != [
            {key: item[key] for key in ("name", "version", "source", "checksum")} for item in recorded]:
        raise Incomplete("The full locked fuzz registry graph differs from admission.")
    for item in recorded:
        name = item["name"] + "-" + item["version"]
        sources = list((base / "common/cargo/registry/src").glob("*/" + name))
        archives = list((base / "common/cargo/registry/cache").glob("*/" + name + ".crate"))
        if len(sources) != 1 or len(archives) != 1 or PRODUCT.sha256(archives[0]) != item["checksum"]:
            raise Incomplete("Fuzz package source/archive identity is absent or ambiguous.")
        files = PRODUCT.inventory(sources[0])
        digest = hashlib.sha256(json.dumps(files, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
        manifest = tomllib.loads((sources[0] / "Cargo.toml").read_text())["package"]
        if (digest != item["source_files_sha256"] or manifest.get("license") != item["declared_license"]
                or manifest.get("build", False) != item["build_script"]):
            raise Incomplete("Fuzz package files, declared grant or build owner changed.")
        if item["name"] == "libfuzzer-sys":
            native = PRODUCT.inventory(sources[0] / "libfuzzer")
            if native != {name: entry["sha256"] for name, entry in selected["native_files"].items()}:
                raise Incomplete("The complete pinned native libFuzzer source differs.")


def fuzz_lock(control: str) -> str:
    """Retain exact registry entries; change only the two owned local harness nodes."""
    blocks = control.split("[[package]]")
    packages = {}
    for block in blocks[1:]:
        item = tomllib.loads("[[package]]" + block)["package"][0]
        if "source" in item:
            packages[item["name"]] = "[[package]]\n" + block.strip() + "\n"
    packages["ntp-wire"] = '[[package]]\nname = "ntp-wire"\nversion = "0.0.0"\n'
    packages["ntp-wire-fuzz"] = ('[[package]]\nname = "ntp-wire-fuzz"\nversion = "0.0.0"\n'
                                     'dependencies = ["libfuzzer-sys", "ntp-wire"]\n')
    return "version = 4\n\n" + "\n".join(packages[name] for name in sorted(packages))


def fuzz_input_snapshot(work: Path, inputs: list[Path]) -> dict:
    """Admit only the exact owned fuzz Cargo config, retaining ancestry guards.

    The bootstrap snapshot helper deliberately forbids every Cargo-home config;
    this product caller needs one fixed, sealed vendor mapping. Inspect that
    exception explicitly, then reuse the unchanged helper on each input island.
    All other work/home configuration remains forbidden and aggregate bounds stay
    equal to the bootstrap helper's limits.
    """
    if len(inputs) != 2 or set(inputs) != {work / "fuzz", work / "fuzz-vendor"}:
        raise Incomplete("The complete pair of fuzz input islands is required.")
    for unexpected in (work / ".cargo", work / "controls/.cargo", work / "home/.cargo",
                       work / "cargo-home/config"):
        if unexpected.exists() or unexpected.is_symlink():
            raise Incomplete("Unexpected Cargo ancestry/home configuration in fuzz work.")
    config = work / "cargo-home/config.toml"
    if ANALYSIS.read_once(config, maximum=4096) != FUZZ_CARGO_CONFIG.encode():
        raise Incomplete("The owned fuzz Cargo-home configuration differs from its fixed admission.")
    result = {"cargo-home/config.toml": {"kind": "file", "bytes": config.stat().st_size,
                                         "sha256": ANALYSIS.digest(config)}}
    for island in inputs:
        if island not in (work / "fuzz", work / "fuzz-vendor") or island.parent != work:
            raise Incomplete("Unexpected fuzz input island.")
        for relative, entry in ANALYSIS.input_snapshot(island, [island]).items():
            key = (Path(island.name) / relative).as_posix()
            if key in result:
                raise Incomplete("Duplicate fuzz input island entry.")
            result[key] = entry
    if len(result) > 100000 or sum(entry.get("bytes", 0) for entry in result.values()) > 2 * 1024 ** 3:
        raise Incomplete("Fuzz input snapshot exceeds aggregate entry/byte bounds.")
    return result


def prepare_fuzz_vendor(original: Path, archive: Path, package: dict, destination: Path) -> None:
    """Prepare one new owned package; reject source/archive drift before publication."""
    cached = PRODUCT.inventory(original)
    # These are Cargo-generated cache records, not published package source.
    source_files = {name: digest for name, digest in cached.items()
                    if name not in {".cargo-ok", ".cargo-checksum.json"}}
    specification = {**package, "files_sha256": PREPARE.files_digest(source_files)}
    data = ANALYSIS.read_once(archive, maximum=PREPARE.MAX_ARCHIVE)
    files = PREPARE.archive_files(data, specification)
    destination.mkdir(mode=0o700)
    for name, contents in files.items():
        target = destination / name
        target.parent.mkdir(parents=True, exist_ok=True, mode=0o700)
        target.write_bytes(contents)
    checksums = {"files": {name: PREPARE.sha(contents) for name, contents in files.items()},
                 "package": package["checksum"]}
    (destination / ".cargo-checksum.json").write_text(json.dumps(checksums, sort_keys=True) + "\n")


def execute(args, report: dict) -> None:
    """Freeze graph/tools, execute the selected isolated lane, then reject drift."""
    policy = read_policy(ROOT)
    subjects = select_subjects(args.subject)
    report["subjects"] = subjects
    dependency_policy, identity = PRODUCT.admit_product(ROOT)
    product_identity = dict(identity)
    extra = (POLICY, "scripts/portable-analysis.py", "scripts/verify-wire-analysis.py", "scripts/prepare-product.py", "scripts/analysis-runtime.py",
             "scripts/check-analysis.py", "policy/analysis-tools.json", "policy/verification-manifest.toml")
    extra += (policy["coverage_guided_fuzzing"]["target_source"],
              policy["coverage_guided_fuzzing"]["llvm_notice"])
    identity.update({name: PRODUCT.sha256(ROOT / name) for name in extra})
    admitted = ANALYSIS.admit(ROOT)
    base, stable = admitted["base"], admitted["stable"]
    vendor = args.vendor_dir.resolve(strict=True)
    vendor_identity = PRODUCT.admit_vendor(vendor, dependency_policy)
    output = args.output_dir
    if (not output.is_absolute() or output.parent != output.parent.resolve(strict=True)
            or output.is_relative_to(ROOT) or output.is_relative_to(Path("/mnt"))):
        raise Incomplete("Use a fresh external canonical private Linux output directory.")
    output.mkdir(mode=0o700)
    report["owned_output"] = str(output)
    work = output / "work"
    work.mkdir(mode=0o700)
    source = work / "source"
    source.mkdir()
    for name in identity:
        destination = source / name
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(ROOT / name, destination)
    require_unchanged(source, identity)
    mutations = tomllib.loads((source / ".cargo/mutants.toml").read_text())
    if mutations.get("additional_cargo_args") != ["--frozen"] or mutations.get("cap_lints") is not False:
        raise Incomplete("Mutation commands require the unchanged frozen Cargo and lint policy.")
    tools = output / "tools"
    tools.mkdir(mode=0o700)
    selected = (("cargo-llvm-cov", "cargo-mutants") if args.lane == "all" else
                ("cargo-llvm-cov",) if args.lane == "coverage" else
                ("cargo-mutants",) if args.lane == "mutation" else ())
    tool_identity = {}
    for name in selected:
        origin = tool_path(ROOT, name)
        shutil.copyfile(origin, tools / name)
        (tools / name).chmod(0o500)
        tool_identity[name] = PRODUCT.sha256(origin)
    if any(PRODUCT.sha256(tools / name) != digest for name, digest in tool_identity.items()):
        raise Incomplete("An analysis extension changed while its sealed copy was prepared.")
    protected_tools = {tools: PRODUCT.inventory(tools)}
    (work / "cargo-home").mkdir()
    (work / "cargo-home/config.toml").write_text(
        '[source.crates-io]\nreplace-with="reviewed"\n[source.reviewed]\ndirectory=' + json.dumps(str(vendor)) + '\n')
    report.update(source_sha256=identity, tools_sha256=tool_identity, lanes=[],
                  analysis_policy_sha256=admitted["policy_sha256"], work=str(work),
                  coverage_guided_fuzzing=policy["coverage_guided_fuzzing"])

    def run(label: str, command: list[str], timeout: int, *, expected_error=None, sealed=(),
            compiler=None, extra_tools=None, mode="ordinary", negative=None, cwd=None, mutation_subject=None):
        """Run immutable selected source with cleared environment and finite custody."""
        if mutation_subject is not None:
            if (label != "mutation-campaign:" + mutation_subject or expected_error or negative
                    or compiler is not None or extra_tools is not None or mode != "ordinary" or cwd is not None):
                raise Incomplete("Mutation campaign cannot borrow another execution mode or failure handler.")
            COMMON.mutation_campaign_command(SCOPE, tools, mutation_subject, command)
        require_unchanged(ROOT, identity)
        require_current_candidate(ROOT, product_identity)
        require_unchanged(source, identity)
        if any(PRODUCT.inventory(folder) != expected for folder, expected in protected_tools.items()):
            raise Incomplete("An admitted analysis extension changed before execution.")
        compiler = compiler or stable
        environment = {"RUSTC": str(compiler / "bin/rustc"),
                       "RUSTDOC": str(compiler / "bin/rustdoc"), "RUST_TEST_THREADS": "2"}
        if compiler != stable:
            environment.update(RUSTUP_HOME=str(base / "common/rustup"), RUSTUP_TOOLCHAIN=ANALYSIS.NIGHTLY)
        readonly = [base, stable, vendor, tools, *([extra_tools] if extra_tools else [])]
        paths = [*([extra_tools] if extra_tools else []), compiler / "bin", tools]
        result = RUNTIME.run(ROOT, work, command, readonly=readonly,
                             sealed=[source, work / "cargo-home/config.toml", *sealed], path_dirs=paths, cwd=cwd or source,
                             environment=environment, mode=mode,
                             timeout=timeout, cpu_seconds=min(timeout, 600))
        report["lanes"].append({"lane": label, "command": command, **result})
        (output / "progress.json").write_text(json.dumps(report, indent=2) + "\n")
        require_unchanged(source, identity)
        require_unchanged(ROOT, identity)
        require_current_candidate(ROOT, product_identity)
        if any(PRODUCT.inventory(folder) != expected for folder, expected in protected_tools.items()):
            raise Incomplete("An admitted analysis extension changed during execution.")
        if mutation_subject is not None:
            return COMMON.mutation_campaign_result(SCOPE, tools, mutation_subject, command, result)
        if expected_error:
            privacy_error(result, expected_error)
            return result["output"]
        if negative:
            if result["failure"] or result["exit_code"] == 0 or negative not in result["output"]:
                raise Incomplete("The seeded fuzz control did not reach its intended failure.")
            return result["output"]
        return PRODUCT.ENV.require_ok(result)

    if args.lane in {"contracts", "all"}:
        consumers = work / "contract-inputs"
        consumers.mkdir()
        examples = contract_examples()
        for name, (text, _) in examples.items():
            (consumers / (name + ".rs")).write_text(text + "\n")
        (work / "contracts").mkdir()
        rustc = str(stable / "bin/rustc")
        run("contract-library", [rustc, "--edition", "2024", "--crate-name", "ntp_wire",
                                  "--crate-type", "rlib", "/work/source/crates/ntp-wire/src/lib.rs",
                                  "--out-dir", "/work/contracts"], 120, sealed=[consumers])
        for name, (_, diagnostic) in examples.items():
            run("external-consumer-" + name,
                [rustc, "--edition", "2024", "--crate-name", name,
                 "/work/contract-inputs/" + name + ".rs",
                 "--extern", "ntp_wire=/work/contracts/libntp_wire.rlib", "--out-dir", "/work/contracts"],
                120, sealed=[consumers],
                expected_error=diagnostic)
        report["contracts"] = {"legal_consumers_compiled": 1,
                               "privacy_bypasses_rejected": sum(
                                   diagnostic in {"E0451", "E0616"} for _, diagnostic in examples.values()),
                               "subjects": dict(SUBJECTS),
                               "construction_or_mutation_bypasses_rejected": sum(
                                   diagnostic in {"E0451", "E0616"} for _, diagnostic in examples.values()),
                               "borrow_bypasses_rejected": sum(
                                   diagnostic in {"E0502", "E0515"} for _, diagnostic in examples.values()),
                               "default_or_payload_debug_bypasses_rejected": sum(
                                   diagnostic == "E0277" for _, diagnostic in examples.values()),
                               "required_diagnostics": {name: diagnostic for name, (_, diagnostic) in examples.items()
                                                        if diagnostic is not None}}
        print("PASSED: ntp-wire external-consumer contracts", flush=True)
    COMMON.run_analysis_lanes(SCOPE, subjects, policy, work, tools, run, report, args.lane)
    if args.lane in {"fuzz", "all"}:
        admit_fuzz_sources(base, policy)
        fuzz = work / "fuzz"
        fuzz.mkdir()
        (fuzz / "Cargo.toml").write_text(
            '[package]\nname="ntp-wire-fuzz"\nversion="0.0.0"\nedition="2024"\npublish=false\n'
            '[workspace]\n[package.metadata]\ncargo-fuzz=true\n[features]\nseeded-defect=[]\n'
            '[dependencies]\nlibfuzzer-sys="=0.4.13"\nntp-wire={path="../source/crates/ntp-wire"}\n'
            '[[bin]]\nname="wire"\npath="target.rs"\ntest=false\ndoc=false\nbench=false\n')
        (fuzz / "Cargo.lock").write_text(fuzz_lock((base / "common/controls-source/fuzz/Cargo.lock").read_text()))
        shutil.copyfile(ROOT / policy["coverage_guided_fuzzing"]["target_source"], fuzz / "target.rs")
        fuzz_vendor = work / "fuzz-vendor"
        fuzz_vendor.mkdir()
        for item in policy["coverage_guided_fuzzing"]["locked_registry_packages"]:
            name = item["name"] + "-" + item["version"]
            original = next((base / "common/cargo/registry/src").glob("*/" + name))
            archive = next((base / "common/cargo/registry/cache").glob("*/" + name + ".crate"))
            prepare_fuzz_vendor(original, archive, item, fuzz_vendor / name)
        (work / "cargo-home/config.toml").write_text(FUZZ_CARGO_CONFIG)
        sealed = [fuzz, fuzz_vendor]
        sealed_before = fuzz_input_snapshot(work, sealed)
        night = base / admitted["policy"]["common_nightly"]
        fuzz_tools = output / "fuzz-tools"
        fuzz_tools.mkdir(mode=0o700)
        # cargo-fuzz 0.13.2 has no frozen CLI flag. This fixed launcher adds the
        # flag to every permitted underlying Cargo command; no general shell runs.
        launcher = fuzz_tools / "cargo"
        launcher.write_text('#!/usr/bin/python3\nimport os,sys\n'
                            'if len(sys.argv)<2 or sys.argv[1] not in ("build","check","rustc","metadata"):\n'
                            ' raise SystemExit("unreviewed fuzz Cargo subcommand")\n'
                            f'os.execv({str(night / "bin/cargo")!r}, [{str(night / "bin/cargo")!r},sys.argv[1],"--frozen",*sys.argv[2:]])\n')
        launcher.chmod(0o500)
        shutil.copyfile(base / "common/bin/cargo-fuzz", fuzz_tools / "cargo-fuzz")
        (fuzz_tools / "cargo-fuzz").chmod(0o500)
        fuzz_tool_hashes = PRODUCT.inventory(fuzz_tools)
        protected_tools[fuzz_tools] = fuzz_tool_hashes
        report["fuzz_launcher_sha256"] = fuzz_tool_hashes
        metadata = run("fuzz-graph", [str(night / "bin/cargo"), "metadata", "--frozen", "--format-version", "1",
                                     "--filter-platform", ANALYSIS.PLATFORM, "--manifest-path", "/work/fuzz/Cargo.toml"],
                       120, compiler=night, extra_tools=fuzz_tools, sealed=sealed, cwd=fuzz)
        report["fuzz_graph"] = fuzz_metadata(json.loads(metadata), policy)
        corpus = work / "fuzz-corpus"
        corpus.mkdir()
        seeds = fuzz_seeds()
        for name, data in seeds.items():
            (corpus / name).write_bytes(data)
        seed_identity = PRODUCT.inventory(corpus)
        (work / "fuzz-artifacts").mkdir()
        (work / "fuzz-binaries").mkdir()
        retained_binaries = {}
        for case in ("healthy", "defect"):
            command = [str(fuzz_tools / "cargo-fuzz"), "fuzz", "build", "wire", "--fuzz-dir", "/work/fuzz",
                       "--sanitizer", "address", "--target", ANALYSIS.PLATFORM,
                       "--target-dir", "/work/target-fuzz"]
            if case == "defect":
                command.extend(["--features", "seeded-defect"])
            # Only identical dependency compilation is reused. Preserve each
            # feature-specific executable as an independent immutable byte copy.
            run("fuzz-build-" + case, command, 600, compiler=night, extra_tools=fuzz_tools,
                sealed=[*sealed, *retained_binaries], cwd=fuzz)
            built_binary = work / "target-fuzz" / ANALYSIS.PLATFORM / "release/wire"
            binary_sha256 = PRODUCT.sha256(built_binary)
            if binary_sha256 in retained_binaries.values():
                raise Incomplete("Healthy and seeded-control builds emitted the same executable.")
            binary = "/work/fuzz-binaries/" + case
            actual_binary = work / binary.removeprefix("/work/")
            shutil.copyfile(built_binary, actual_binary)
            actual_binary.chmod(0o500)
            if PRODUCT.sha256(actual_binary) != binary_sha256:
                raise Incomplete("The retained fuzz executable differs from its actual build.")
            retained_binaries[actual_binary] = binary_sha256
            report.setdefault("fuzz_binaries_sha256", {})[case] = binary_sha256
            report.setdefault("fuzz_binary_paths", {})[case] = str(actual_binary)
            text = run("fuzz-" + case, [binary, "/work/fuzz-corpus", "-artifact_prefix=/work/fuzz-artifacts/",
                                        *RUNTIME.SANITIZER_FLAGS], 90, compiler=night, extra_tools=fuzz_tools,
                       sealed=[*sealed, *retained_binaries], cwd=fuzz, mode="sanitizer",
                       negative="independent datagram field-count oracle" if case == "defect" else None)
            if case == "healthy":
                count = re.search(r"Done (\d+) runs", text)
                if not count or int(count[1]) != 256:
                    raise Incomplete("The actual healthy fuzz target did not complete its finite 256 runs.")
                report["fuzz"] = {"observed_runs": 256, "subjects": dict(SUBJECTS),
                                  "seed_lengths": sorted({len(data) for data in seeds.values()}),
                                  "seed_files_sha256": seed_identity,
                                  "seeded_fault": "pending", "sanitizer": "address", "target": ANALYSIS.PLATFORM}
            else:
                report["fuzz"]["seeded_fault"] = "observed intended datagram field-count failure"
            if any(PRODUCT.sha256(path) != digest for path, digest in retained_binaries.items()):
                raise Incomplete("A retained ASan executable changed during a build or execution.")
            if (fuzz_input_snapshot(work, sealed) != sealed_before
                    or PRODUCT.inventory(fuzz_tools) != fuzz_tool_hashes):
                raise Incomplete("Fuzz source, dependency or fixed Cargo launcher bytes changed.")
        admit_fuzz_sources(base, policy)
        report["fuzz"]["fault_artifacts_sha256"] = PRODUCT.inventory(work / "fuzz-artifacts")
        print("PASSED: ntp-wire bounded ASan/libFuzzer target and seeded control", flush=True)
    require_unchanged(ROOT, identity)
    require_current_candidate(ROOT, product_identity)
    require_unchanged(source, identity)
    if PRODUCT.admit_vendor(vendor, dependency_policy) != vendor_identity:
        raise Incomplete("Vendor bytes changed during parser qualification.")
    if any(PRODUCT.sha256(tools / name) != digest for name, digest in tool_identity.items()):
        raise Incomplete("An admitted analysis extension changed during execution.")
    after = ANALYSIS.admit(ROOT)
    if any(after[key] != admitted[key] for key in ("base", "stable", "policy_sha256", "inventories")):
        raise Incomplete("Analysis installation or payload identities changed during execution.")


def main(argv=None) -> int:
    """Run only explicit fixed lanes, retaining partial results when a gate fails."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--lane", choices=("contracts", "coverage", "mutation", "fuzz", "all"), default="all")
    parser.add_argument("--subject", choices=("header", "datagram", "all"), default="all")
    parser.add_argument("--vendor-dir", type=Path)
    parser.add_argument("--output-dir", type=Path, required=True)
    args = parser.parse_args(argv)
    report = {"status": "incomplete", "scope": "ntp-wire header/outer-framing portable contracts/coverage/mutation/fuzz",
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
