#!/usr/bin/env python3
"""One admitted, offline Cargo/Garde code-generation preflight; never a proof.

Keep emitted models/maps for later inspection. This profile executes real build
scripts and proc macros, but performs no solver run or unwinding qualification.
"""
from __future__ import annotations

import argparse
import importlib.util
import json
import os
from pathlib import Path
import re
import stat
import sys
import time
import tomllib


ROOT = Path(__file__).resolve().parents[2]


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    loaded = importlib.util.module_from_spec(spec)
    sys.modules[name] = loaded
    spec.loader.exec_module(loaded)
    return loaded


FORMAL = load("era_codegen_shared_formal", ROOT / "tools/formal/verify.py")
Incomplete = FORMAL.Incomplete
write = FORMAL.write_json
CONSUMER = "era-constructor-codegen-v1"
FLAGS = ("--lib", "--only-codegen", "--keep-temps", "--verbose", "--output-format", "regular")
CRATE = "nevrotick_era_constructor_proof"
HARNESSES = {"era_constructor_full_domain", "era_constructor_control_singleton",
             "era_constructor_control_retention", "era_constructor_control_error"}
POINTER = Path.home() / ".local/share/nevrotick/analysis-installation.json"
MAX_ENTRIES = 20_000
MAX_ARTIFACT = 256 * 1024 ** 2
MAX_RETAINED = 1024 ** 3
MAX_JSON = 8 * 1024 ** 2
SOURCE_EVIDENCE = {
    "source_ref": "kani-0.68.0",
    "source_commit": "0d2328a93f0e0ff66132d6bfa1a7d884877cf862",
    "dispatch": "https://github.com/model-checking/kani/blob/0d2328a93f0e0ff66132d6bfa1a7d884877cf862/kani-driver/src/main.rs#L243-L267",
    "codegen_skip_verification": "https://github.com/model-checking/kani/blob/0d2328a93f0e0ff66132d6bfa1a7d884877cf862/kani-driver/src/main.rs#L116-L122",
    "private_cargo": "https://github.com/model-checking/kani/blob/0d2328a93f0e0ff66132d6bfa1a7d884877cf862/kani-driver/src/session.rs#L462-L479",
    "cargo_frontend": "https://github.com/model-checking/kani/blob/0d2328a93f0e0ff66132d6bfa1a7d884877cf862/kani-driver/src/call_cargo.rs",
    "model_linking": "https://github.com/model-checking/kani/blob/0d2328a93f0e0ff66132d6bfa1a7d884877cf862/kani-driver/src/project.rs",
    "emitted_files": "https://github.com/model-checking/kani/blob/0d2328a93f0e0ff66132d6bfa1a7d884877cf862/kani-compiler/src/codegen_cprover_gotoc/utils/file_writing_pool.rs#L109-L115",
    "limit": "Official pinned-source review is not a binary/source attestation.",
}


def input_names(vendor) -> list[str]:
    """Freeze the complete fixed consumer, projection and admission authorities."""
    names = {
        "tools/formal/verify_era_codegen.py", "tools/formal/test_verify_era_codegen.py",
        "tools/formal/verify.py", "tools/formal/era_constructor_projection.py",
        "tools/formal/era_constructor_harness.rs", "scripts/check-analysis.py",
        "scripts/analysis-runtime.py", "scripts/analysis-controls.py", "scripts/check-environment.py",
        "policy/analysis-tools.json", "policy/tool-manifest.json", "rust-toolchain.toml",
        ".agents/skills/testing/references/formal-verification.md",
    }
    names.update(vendor.input_names())
    return sorted(names)


def require_sources(check, frozen: Path, records: dict, names: list[str], *, original_root: Path) -> None:
    if sorted(records) != names:
        raise Incomplete("Codegen source closure changed")
    for name, expected in records.items():
        for base in (original_root, frozen):
            path = base / name
            if path != path.resolve(strict=True):
                raise Incomplete("Codegen source path redirects")
            data = check.read_once(path)
            if {"bytes": len(data), "sha256": FORMAL.digest(data)} != expected:
                raise Incomplete("Original or frozen codegen source changed: " + name)


def readonly_directory(check, path: Path, work: Path) -> Path:
    """Reject broad/redirected roots and every overlap with writable work."""
    value = check.RUNTIME._directory(path, "Codegen readonly root")
    check.RUNTIME._narrow(value)
    if value.is_relative_to(work) or work.is_relative_to(value):
        raise Incomplete("Codegen readonly root overlaps writable work")
    return value


def tool_identity(admitted: dict) -> dict:
    return {key: admitted[key] for key in (
        "admission_profile", "admission_consumer", "base", "readonly_roots", "policy_sha256",
        "inventories", "installation_pointer", "installation_pointer_sha256")}


def admit_tools(check, frozen: Path, output: Path) -> dict:
    """Use the dedicated codegen consumer over the unchanged complete Kani closure."""
    started = time.monotonic()
    before = check.read_once(POINTER, maximum=4096, private=True)
    admitted = check.admit_era_constructor_codegen(frozen)
    if admitted.get("admission_consumer") != CONSUMER:
        raise Incomplete("Tool admission returned a different consumer")
    FORMAL.proof_readonly_roots(admitted, "kani-proof")
    if (check.read_once(POINTER, maximum=4096, private=True) != before
            or admitted.get("installation_pointer_sha256") != FORMAL.digest(before)):
        raise Incomplete("Installation pointer changed during codegen admission")
    payload, compiler = admitted["readonly_roots"]
    paths = {"driver": payload / "bin/kani-driver", "goto_cc": payload / "bin/goto-cc",
             "cargo": compiler / "bin/cargo", "rustc": compiler / "bin/rustc",
             "rustdoc": compiler / "bin/rustdoc"}
    hashes = {name: check.digest(path) for name, path in paths.items()}
    value = {"admitted": admitted, "executable_paths": paths, "executable_sha256": hashes,
             "admission_seconds": time.monotonic() - started,
             "consumer": CONSUMER, "codegen_flags": FLAGS, "source_evidence": SOURCE_EVIDENCE,
             "proof_profile_applied": False, "unwind_bound": None,
             "host_runtime_trust": "Existing trusted Python/stdlib, OS loader and public libraries; not rehashed here."}
    write(output, value)
    return value


def command_and_mounts(check, tools: dict, source: Path, vendor: dict, work: Path, project: Path):
    payload, compiler = FORMAL.proof_readonly_roots(tools["admitted"], "kani-proof")
    roots = (payload, compiler, source, vendor["readonly_root"])
    for path in roots:
        readonly_directory(check, path, work)
    if any(a != b and (a.is_relative_to(b) or b.is_relative_to(a)) for a in roots for b in roots):
        raise Incomplete("Codegen readonly roots overlap")
    if (len(set(roots)) != 4 or roots[3] != source.parent / "vendor-island"
            or vendor["vendor_root"] != roots[3] / "vendor"
            or vendor["config_path"] != roots[3] / "cargo-config.toml"
            or vendor["manifest_path"] != roots[3] / "manifest.json"):
        raise Incomplete("Codegen requires exactly four narrow readonly roots")
    manifest = project / "Cargo.toml"
    if not manifest.is_file() or manifest != manifest.resolve(strict=True) or project != source / "project":
        raise Incomplete("Codegen manifest is outside its frozen source island")
    command = [str(payload / "bin/kani-driver"), "kani", "--manifest-path", str(manifest),
               *FLAGS, "--target-dir", "/work/target"]
    environment = {"RUSTC": str(compiler / "bin/rustc"), "RUSTDOC": str(compiler / "bin/rustdoc"),
                   "RUSTUP_TOOLCHAIN": "nightly-2026-08-21"}
    return command, roots, [payload / "bin", compiler / "bin"], environment


def prepare_host_linker(check, frozen: Path, work: Path) -> dict:
    """Select the admitted public C linker for real host build/proc-macro jobs.

    A generic host field merges with Kani's own generic host.rustflags. The
    project configuration remains byte-identical to its vendor correspondence.
    """
    helpers = check.RUNTIME._helpers(frozen)
    home = work / "cargo-home"
    home.mkdir(mode=0o700)
    path = home / "config.toml"
    data = ("[host]\nlinker = " + json.dumps(helpers["cc"]) + "\n").encode()
    descriptor = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW | os.O_CLOEXEC, 0o600)
    try:
        with os.fdopen(descriptor, "wb", closefd=False) as stream:
            stream.write(data)
        os.fchmod(descriptor, 0o400)
    finally:
        os.close(descriptor)
    return {"path": path, "sha256": FORMAL.digest(data), "content": data.decode(),
            "cc": helpers["cc"], "cc_sha256": check.digest(Path(helpers["cc"])),
            "ld": helpers["ld"], "ld_sha256": check.digest(Path(helpers["ld"])),
            "host_runtime_trust": "Policy-pinned cc/ld plus existing trusted public GCC/OS runtime; not a closed GCC library inventory."}


def require_cargo_configuration(check, project: Path, vendor: dict, work: Path, host: dict) -> None:
    """Admit only the two explicit immutable Cargo configs, including home ancestry."""
    for path in (work / ".cargo", work / "controls/.cargo", work / "cargo-home/config",
                 work / "cargo-home/credentials", work / "cargo-home/credentials.toml"):
        if path.exists() or path.is_symlink():
            raise Incomplete("Unselected Cargo ancestry/home configuration")
    expected = work / "cargo-home/config.toml"
    if host["path"] != expected or expected != expected.resolve(strict=True):
        raise Incomplete("Host Cargo configuration path redirects")
    data = check.read_once(expected, maximum=4096)
    if (FORMAL.digest(data) != host["sha256"]
            or tomllib.loads(data.decode()) != {"host": {"linker": host["cc"]}}):
        raise Incomplete("Host Cargo linker configuration changed")
    if check.read_once(project / ".cargo/config.toml") != check.read_once(vendor["config_path"]):
        raise Incomplete("Project Cargo configuration differs from admitted vendor configuration")


def observe_artifacts(check, work: Path, project: Path, graph: list[dict]) -> dict:
    """Inspect bounded real compiler metadata, models/maps and Cargo fingerprints."""
    target = work / "target"
    if not target.is_dir() or target != target.resolve(strict=True):
        raise Incomplete("Codegen produced no canonical target directory")
    wanted = {item["name"]: item for item in graph}
    if len(wanted) != 14:
        raise Incomplete("Codegen requires the reviewed fourteen-package graph")
    files, metadata, fingerprints = {}, [], {name: [] for name in wanted}
    entries = 0
    for directory, subdirs, leaves in os.walk(target, followlinks=False):
        for name in subdirs + leaves:
            entries += 1
            if entries > MAX_ENTRIES:
                raise Incomplete("Codegen output entry budget exceeded")
            path = Path(directory) / name
            info = path.lstat()
            if stat.S_ISLNK(info.st_mode) or not (stat.S_ISDIR(info.st_mode) or stat.S_ISREG(info.st_mode)):
                raise Incomplete("Codegen artifacts contain an alias or special file")
            if not stat.S_ISREG(info.st_mode):
                continue
            selected = name.endswith((".kani-metadata.json", ".symtab.out", ".symtab.json", ".out",
                                      ".type_map.json", ".pretty_name_map.json", ".restrictions.json"))
            package = None
            if path.parent.name == "fingerprint" and name.startswith("lib-") and name.endswith(".json"):
                # Pinned Cargo 1.100 separates host and explicit-target builds;
                # package/hash/fingerprint replaces the old .fingerprint tree.
                match = re.fullmatch(
                    r"kani/(?:x86_64-unknown-linux-gnu/)?debug/build/([^/]+)/[0-9a-f]{16}/fingerprint/(lib-[^/]+\.json)",
                    path.relative_to(target).as_posix())
                if not match or match[1] not in {*wanted, CRATE.replace("_", "-")}:
                    raise Incomplete("Unreviewed Cargo fingerprint ancestry or package")
                if name != "lib-" + match[1].replace("-", "_") + ".json":
                    raise Incomplete("Unexpected selected package fingerprint name")
                if match[1] in wanted:
                    package = match[1]
                    selected = True
            if not selected:
                continue
            if info.st_size <= 0 or info.st_size > MAX_ARTIFACT:
                raise Incomplete("Codegen selected artifact is empty or oversized")
            files[path.relative_to(work).as_posix()] = {"bytes": info.st_size, "sha256": check.digest(path)}
            if sum(item["bytes"] for item in files.values()) > MAX_RETAINED:
                raise Incomplete("Codegen selected artifact byte budget exceeded")
            if name.endswith(".kani-metadata.json"):
                metadata.append((path, check.decode(check.read_once(path, maximum=MAX_JSON))))
            if package is not None:
                value = check.decode(check.read_once(path, maximum=256 * 1024))
                features = value.get("features") if type(value) is dict else None
                if type(features) is not str:
                    raise Incomplete("Cargo fingerprint lacks its actual feature list")
                features = check.decode(features.encode())
                if type(features) is not list or features != wanted[package]["expected_features"]:
                    raise Incomplete("Observed Cargo features differ: " + package)
                fingerprints[package].append({"path": path.relative_to(work).as_posix(), "features": features})
    if any(not values for values in fingerprints.values()):
        raise Incomplete("Actual Cargo fingerprints do not observe all fourteen packages")
    observed = [value for _, value in metadata if type(value) is dict and value.get("crate_name") == CRATE]
    if len(observed) != 1:
        raise Incomplete("Expected exactly one constructor crate metadata record")
    value = observed[0]
    harnesses = value.get("proof_harnesses")
    if (type(harnesses) is not list or len(harnesses) != 4 or value.get("test_harnesses") != []
            or value.get("unsupported_features") != []):
        raise Incomplete("Constructor codegen metadata is incomplete or unsupported")
    names = []
    for harness in harnesses:
        if (type(harness) is not dict or harness.get("crate_name") != CRATE
                or type(harness.get("pretty_name")) is not str
                or harness.get("original_file") not in (str(project / "src/proof.rs"), "src/proof.rs")):
            raise Incomplete("Constructor harness source identity differs")
        original = Path(harness["original_file"])
        original = original if original.is_absolute() else project / original
        if original != original.resolve(strict=True) or original != project / "src/proof.rs":
            raise Incomplete("Constructor harness source path redirects")
        names.append(harness["pretty_name"])
        model = Path(harness.get("goto_file", ""))
        if not model.is_absolute() or not model.is_relative_to("/work/target") or not str(model).endswith(".symtab.out"):
            raise Incomplete("Harness model escapes the bounded Cargo target")
        relative = model.relative_to("/work").as_posix()
        linked = relative.removesuffix(".symtab.out") + ".out"
        if relative not in files or linked not in files:
            raise Incomplete("A constructor harness model or linked output is missing")
    if len(set(names)) != 4 or {name.rsplit("::", 1)[-1] for name in names} != HARNESSES:
        raise Incomplete("Constructor harness set differs from the reviewed source")
    return {"artifacts": files, "selected_bytes": sum(item["bytes"] for item in files.values()),
            "constructor_metadata": value, "observed_harness_names": names,
            "observed_package_features": fingerprints, "fingerprint_version_binding": "Immutable Cargo.lock and admitted vendor graph; fingerprint records actual feature compilation.",
            "readable_loop_ir_available": any(name.endswith(".symtab.json") for name in files),
            "loop_allocator_inspection_complete": False, "proof_qualified": False}


def run(output: Path, *, vendor_root=None) -> dict:
    """Freeze data through the live bootstrap, then dispatch only frozen code."""
    try:
        island = output / "source-island"
        island.mkdir(mode=0o700)
        frozen = island / "frozen"
        check = load("era_codegen_bootstrap_analysis", ROOT / "scripts/check-analysis.py")
        vendor = load("era_codegen_bootstrap_vendor", ROOT / "tools/formal/era_vendor_admission.py")
        names = input_names(vendor)
        records = FORMAL.freeze_inputs(check, frozen, names)
        consumer = load("era_codegen_frozen_dispatch", frozen / "tools/formal/verify_era_codegen.py")
    except Exception as error:
        summary = {"status": "incomplete", "consumer": CONSUMER, "codegen_only": True,
                   "proof_qualified": False, "unwind_bound": None, "maximum_native_commands": 1,
                   "native_top_level_attempts": 0, "native_top_level_returned": 0,
                   "error": f"{type(error).__name__}: {error}"}
        write(output / "summary.json", summary)
        return summary
    # Prepared-stage failures retain their actual in-memory/durable attempt
    # accounting; the zero-attempt bootstrap error handler must not rebase it.
    return consumer.run_prepared(output, original_root=ROOT, records=records, names=names,
                                 vendor_root=vendor_root)


def run_prepared(output: Path, *, original_root: Path, records: dict, names: list[str], vendor_root=None) -> dict:
    """Execute the frozen consumer with distinct original/frozen source checks."""
    started = time.monotonic()
    work, island, bundle = output / "work", output / "source-island", output / "vendor-island"
    work.mkdir(mode=0o700)
    frozen, project = ROOT, island / "project"
    receipts = work / "receipts"
    receipts.mkdir(mode=0o700)
    summary = {"status": "incomplete", "consumer": CONSUMER, "codegen_only": True,
               "proof_qualified": False, "unwind_bound": None, "maximum_native_commands": 1,
               "native_top_level_attempts": 0, "native_top_level_returned": 0,
               "readable_loop_ir_available": False, "loop_allocator_inspection_complete": False,
               "inherited_environment": "Runtime clearenv; no ambient Cargo/Rust/tool overrides copied.",
               "standalone_proof_flags_applied": False, "flags": FLAGS}
    summary.update(original_root=original_root, frozen_root=frozen)
    check = vendor = None
    readonly_before = tools_before = vendor_before = None
    host_linker = None
    anchored = []
    retained = []
    try:
        if (frozen != output / "source-island/frozen" or original_root == frozen
                or original_root != original_root.resolve(strict=True)
                or Path(FORMAL.__file__) != frozen / "tools/formal/verify.py"):
            raise Incomplete("Codegen prepared stage requires the actual frozen consumer/helpers")
        check = load("era_codegen_analysis", frozen / "scripts/check-analysis.py")
        if (Path(check.__file__) != frozen / "scripts/check-analysis.py"
                or Path(check.RUNTIME.__file__) != frozen / "scripts/analysis-runtime.py"):
            raise Incomplete("Codegen analysis/runtime helpers were not loaded from frozen source")
        require_sources(check, frozen, records, names, original_root=original_root)
        vendor = load("era_codegen_vendor_frozen", frozen / "tools/formal/era_vendor_admission.py")
        if input_names(vendor) != names:
            raise Incomplete("Frozen codegen input closure differs from bootstrap")
        vendor.prepare(frozen, bundle, vendor_root=vendor_root, work_root=work)
        vendor_before = vendor.admit(frozen, bundle, work_root=work)
        projection = load("era_codegen_projection", frozen / "tools/formal/era_constructor_projection.py")
        prepared = projection.prepare_era_constructor(frozen, project, "healthy",
            frozen / "tools/formal/era_constructor_harness.rs", vendor_root=vendor_before["vendor_root"])
        if prepared.get("entrypoint") != project / "Cargo.toml":
            raise Incomplete("Projection returned a different Cargo manifest")
        if check.read_once(project / ".cargo/config.toml") != check.read_once(vendor_before["config_path"]):
            raise Incomplete("Project Cargo configuration differs from admitted vendor configuration")
        for path in sorted(frozen.rglob("*"), reverse=True):
            path.chmod(0o500 if path.is_dir() else 0o400)
        frozen.chmod(0o500)
        island.chmod(0o500)
        readonly_before = check.input_snapshot(output, [island])
        write(receipts / "preparation.json", {"source_manifest": records, "preparation": prepared,
                                             "vendor_admission": vendor_before})
        FORMAL.retain_artifacts(check, output, retained, [island, receipts / "preparation.json"])
        tools_before = admit_tools(check, frozen, receipts / "pre-admission.json")
        FORMAL.retain_artifacts(check, output, retained, [receipts / "pre-admission.json"])
        host_linker = prepare_host_linker(check, frozen, work)
        FORMAL.retain_artifacts(check, output, retained, [host_linker["path"]])
        command, readonly, paths, environment = command_and_mounts(check, tools_before, island,
                                                                   vendor_before, work, project)
        require_sources(check, frozen, records, names, original_root=original_root)
        if check.input_snapshot(output, [island]) != readonly_before:
            raise Incomplete("Frozen source/project changed before codegen")
        vendor_pre = vendor.admit(frozen, bundle, work_root=work)
        if vendor.identity(vendor_pre) != vendor.identity(vendor_before):
            raise Incomplete("Vendor payload changed before codegen")
        write(receipts / "pre-vendor.json", vendor_pre)
        FORMAL.retain_artifacts(check, output, retained, [receipts / "pre-vendor.json"])
        require_cargo_configuration(check, project, vendor_before, work, host_linker)
        summary.update(command=command, readonly_roots=readonly, path_dirs=paths,
                       explicit_environment=environment, host_linker=host_linker)
        # A submitted top-level attempt is durable before namespace setup or
        # native execution; return/process results remain separately counted.
        summary["native_top_level_attempts"] = 1
        write(receipts / "execution.json", summary)
        sealed = [receipts, host_linker["path"]]
        # This distinct profile deliberately owns one sealed Cargo-home config;
        # snapshot relative to the outer custody root, not generic empty homes.
        before_receipts = check.input_snapshot(output, sealed)
        execute_start = time.monotonic()
        try:
            native = check.RUNTIME.run(frozen, work, command, readonly=readonly, path_dirs=paths, cwd=project,
                sealed=sealed, environment=environment, timeout=180, cpu_seconds=180,
                address_space_bytes=8 * 1024 ** 3, process_limit=128, build_jobs=2)
        finally:
            summary["execute_seconds"] = time.monotonic() - execute_start
        summary.update(native=native, native_top_level_returned=1)
        check.RUNTIME._owned_work(work, original_root)
        if check.input_snapshot(output, sealed) != before_receipts:
            raise Incomplete("Pre-command codegen receipts changed")
        if check.input_snapshot(output, [island]) != readonly_before:
            raise Incomplete("Frozen source/project changed during codegen")
        require_sources(check, frozen, records, names, original_root=original_root)
        require_cargo_configuration(check, project, vendor_before, work, host_linker)
        completed = (type(native) is dict and {"exit_code", "failure", "output"}.issubset(native)
                     and type(native.get("exit_code")) is int and type(native.get("output")) is str
                     and native["exit_code"] == 0 and native.get("failure") is None)
        artifact_error = None
        if completed:
            summary["codegen_process_succeeded"] = True
            try:
                artifacts = observe_artifacts(check, work, project, prepared["registry_graph"])
                write(receipts / "artifacts.json", artifacts)
                summary.update(artifacts)
                FORMAL.retain_artifacts(check, output, retained,
                    [receipts / "artifacts.json", *(work / name for name in artifacts["artifacts"])])
            except Exception as error:
                artifact_error = error
        vendor_after = vendor.admit(frozen, bundle, work_root=work)
        write(receipts / "post-vendor.json", vendor_after)
        FORMAL.retain_artifacts(check, output, retained, [receipts / "post-vendor.json"])
        if vendor.identity(vendor_after) != vendor.identity(vendor_before):
            raise Incomplete("Vendor payload identity changed during codegen")
        tools_after = admit_tools(check, frozen, receipts / "post-admission.json")
        FORMAL.retain_artifacts(check, output, retained, [receipts / "post-admission.json"])
        if tool_identity(tools_after["admitted"]) != tool_identity(tools_before["admitted"]):
            raise Incomplete("Tool payload identity changed during codegen")
        if not completed:
            raise Incomplete("Code-generation command did not complete successfully")
        if artifact_error is not None:
            raise artifact_error
        summary["status"] = "codegen-preflight-passed"
    except Exception as error:
        summary["error"] = f"{type(error).__name__}: {error}"
    finally:
        # No later native command may rewrite the selected facts. Final tool and
        # vendor admission precede the final audit, including their side effects.
        if check is not None and records is not None:
            try:
                write(receipts / "execution.json", summary)
                custody_roots = [receipts] + ([] if host_linker is None else [host_linker["path"]])
                anchored = check.input_snapshot(output, custody_roots)
                final_tools = admit_tools(check, frozen, work / "final-admission.json")
                final_tool_snapshot = check.input_snapshot(output, [work / "final-admission.json"])
                if tools_before is not None and tool_identity(final_tools["admitted"]) != tool_identity(tools_before["admitted"]):
                    raise Incomplete("Final tool payload identity differs")
                if vendor is not None and vendor_before is not None:
                    final_vendor = vendor.admit(frozen, bundle, work_root=work)
                    write(work / "final-vendor.json", final_vendor)
                    if vendor.identity(final_vendor) != vendor.identity(vendor_before):
                        raise Incomplete("Final vendor payload identity differs")
                check.RUNTIME._owned_work(work, original_root)
                if host_linker is not None and vendor_before is not None:
                    require_cargo_configuration(check, project, vendor_before, work, host_linker)
                require_sources(check, frozen, records, names, original_root=original_root)
                if readonly_before is not None and check.input_snapshot(output, [island]) != readonly_before:
                    raise Incomplete("Final frozen source/project identity differs")
                if check.input_snapshot(output, custody_roots) != anchored:
                    raise Incomplete("Retained codegen receipts changed during final admission")
                if check.input_snapshot(output, [work / "final-admission.json"]) != final_tool_snapshot:
                    raise Incomplete("Final tool receipt changed during vendor closeout")
                try:
                    FORMAL.audit_retained(check, output, retained)
                except Exception as error:
                    raise Incomplete("Retained codegen inputs, artifacts or receipts changed during final admission") from error
                if "artifacts" in summary:
                    for name, expected in summary["artifacts"].items():
                        path = work / name
                        if path.stat().st_size != expected["bytes"] or check.digest(path) != expected["sha256"]:
                            raise Incomplete("Retained codegen artifact changed during final admission")
                if summary["status"] == "codegen-preflight-passed":
                    summary["all_retained_inputs_and_receipts_unchanged"] = True
            except Exception as error:
                summary.update(status="incomplete", final_error=f"{type(error).__name__}: {error}")
        summary["total_seconds"] = time.monotonic() - started
        write(output / "summary.json", summary)
    return summary


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", required=True, help="fresh private canonical directory outside the checkout")
    parser.add_argument("--vendor-root", type=Path, help="canonical source cache; only selected packages are copied")
    args = parser.parse_args(argv)
    os.umask(0o077)
    output = FORMAL.fresh_output(args.output_dir)
    summary = run(output, vendor_root=args.vendor_root)
    print(json.dumps({key: summary[key] for key in ("status", "native_top_level_attempts", "native_top_level_returned", "proof_qualified")}))
    return 0 if summary["status"] == "codegen-preflight-passed" else 1


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, ValueError, Incomplete) as error:
        print(f"Era constructor codegen incomplete: {error}", file=sys.stderr)
        raise SystemExit(1)
