#!/usr/bin/env python3
"""Run the fixed five actual-source Cargo/Garde constructor proofs; install nothing."""
from __future__ import annotations

import argparse
import importlib.util
import json
import os
from pathlib import Path
import re
import sys
import time


ROOT = Path(__file__).resolve().parents[2]


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    value = importlib.util.module_from_spec(spec)
    sys.modules[name] = value
    spec.loader.exec_module(value)
    return value


CODEGEN = load("era_constructor_codegen_helpers", ROOT / "tools/formal/verify_era_codegen.py")
COPY = load("era_constructor_copy_helpers", ROOT / "tools/formal/inspect_era_model.py")
FORMAL = CODEGEN.FORMAL
Incomplete = FORMAL.Incomplete
write = FORMAL.write_json
CONSUMER = "era-constructor-proof-v1"
UNWIND = 32
FLAGS = ("--lib", "--default-unwind", "32", "--output-format", "regular", "--solver", "cadical",
         "-Z", "unstable-options", "--run-sanity-checks", "--harness-timeout", "60s")
POINTER = Path.home() / ".local/share/nevrotick/analysis-installation.json"
MAX_JSON = 32 * 1024 ** 2
MAX_ARTIFACT = 256 * 1024 ** 2
CRATE = "nevrotick_era_constructor_proof"
FILTER_SOURCE = "https://github.com/model-checking/kani/blob/0d2328a93f0e0ff66132d6bfa1a7d884877cf862/kani-compiler/src/kani_middle/codegen_units.rs#L82-L94"


def input_names() -> list[str]:
    vendor = load("era_constructor_vendor_closure", ROOT / "tools/formal/era_vendor_admission.py")
    return sorted({*CODEGEN.input_names(vendor), "tools/formal/additional_suites.py",
        "tools/formal/test_additional_suites.py", "tools/formal/inspect_era_model.py",
        "tools/formal/verify_era_constructor.py", "tools/formal/test_verify_era_constructor.py"})


def admit_tools(check, frozen: Path, receipt: Path) -> dict:
    started = time.monotonic()
    pointer = check.read_once(POINTER, maximum=4096, private=True)
    admitted = check.admit_era_constructor_proof(frozen)
    if admitted.get("admission_consumer") != CONSUMER:
        raise Incomplete("Constructor proof admission returned a different consumer")
    payload, compiler = FORMAL.proof_readonly_roots(admitted, "kani-proof")
    if (check.read_once(POINTER, maximum=4096, private=True) != pointer
            or admitted.get("installation_pointer_sha256") != FORMAL.digest(pointer)):
        raise Incomplete("Constructor proof installation pointer changed")
    executables = {"driver": payload / "bin/kani-driver", "goto_cc": payload / "bin/goto-cc",
                   "goto_instrument": payload / "bin/goto-instrument", "cbmc": payload / "bin/cbmc",
                   "cargo": compiler / "bin/cargo", "rustc": compiler / "bin/rustc", "rustdoc": compiler / "bin/rustdoc"}
    record = {"admitted": admitted, "consumer": CONSUMER, "flags": FLAGS,
              "unwind_bound": UNWIND, "solver": "cadical", "solver_workers": 1,
              "executable_paths": executables,
              "executable_sha256": {key: check.digest(path) for key, path in executables.items()},
              "admission_seconds": time.monotonic() - started,
              "allocation_model": "Pinned Kani assumes successful allocations; no host OOM recovery claim.",
              "host_runtime_trust": "Existing trusted Python/stdlib, OS and public GCC/runtime; not a complete public-tool/library inventory."}
    write(receipt, record)
    return record


def validate_project(check, output: Path, project: Path, prepared: dict) -> None:
    expected = dict(prepared["generated_files_sha256"])
    expected["correspondence.json"] = prepared["correspondence_sha256"]
    snapshot = check.input_snapshot(output, [project])
    files = {name: value for name, value in snapshot.items() if value["kind"] == "file"}
    if set(files) != {(project / name).relative_to(output).as_posix() for name in expected}:
        raise Incomplete("Constructor Cargo preparation has an unreviewed file set")
    for name, checksum in expected.items():
        if files[(project / name).relative_to(output).as_posix()]["sha256"] != checksum:
            raise Incomplete("Constructor Cargo preparation differs from declared source bytes")


def snapshot_raw(check, output: Path, target: Path, export: Path) -> dict:
    roots = [path for path in (target, export) if path.exists() or path.is_symlink()]
    return {"roots": roots, "snapshot": check.input_snapshot(output, roots)}


def require_raw(check, output: Path, raw: dict) -> None:
    if check.input_snapshot(output, raw["roots"]) != raw["snapshot"]:
        raise Incomplete("Raw constructor output changed after native closeout")


def require_ancestry(island: Path, output: Path) -> None:
    for path in (island / ".cargo", island / "projects/.cargo", output / ".cargo"):
        if path.exists() or path.is_symlink():
            raise Incomplete("Unselected constructor Cargo ancestor configuration")


def observe_selected(check, output: Path, work: Path, target: Path, project: Path,
                     evidence: Path, raw: dict, job: dict, prepared: dict) -> dict:
    """Bind one compiler-selected harness/model and all actual dependency features."""
    files = raw["snapshot"]
    target_prefix = target.relative_to(output).as_posix() + "/"
    candidates = [output / name for name, fact in files.items()
                  if fact["kind"] == "file" and name.startswith(target_prefix) and name.endswith(".kani-metadata.json")]
    selected = []
    for path in candidates:
        value = check.decode(check.read_once(path, maximum=8 * 1024 ** 2))
        if type(value) is dict and value.get("crate_name") == CRATE:
            selected.append((path, value))
    if len(selected) != 1:
        raise Incomplete("Expected exactly one constructor compiler metadata file")
    metadata_path, metadata = selected[0]
    harnesses = metadata.get("proof_harnesses")
    if (type(harnesses) is not list or len(harnesses) != 1
            or metadata.get("test_harnesses") != [] or metadata.get("unsupported_features") != []
            or metadata.get("contracted_functions") != [] or metadata.get("autoharness_md", False) is not None):
        raise Incomplete("Constructor compiler metadata is not the selected ordinary harness")
    harness = harnesses[0]
    attributes = {"kind": "Proof", "should_panic": False, "solver": None, "unwind_value": None,
                  "stubs": [], "verified_stubs": []}
    if (type(harness) is not dict or harness.get("pretty_name") != job["harness"]
            or harness.get("crate_name") != CRATE or harness.get("attributes") != attributes
            or harness.get("contract", False) is not None
            or any(harness.get(name) is not False for name in (
                "has_loop_contracts", "is_automatically_generated", "is_bounded", "is_ctor_based"))
            or harness.get("original_file") not in ("src/proof.rs", str(project / "src/proof.rs"))):
        raise Incomplete("Constructor compiler attributes/source override the fixed proof profile")
    source = Path(harness["original_file"])
    source = source if source.is_absolute() else project / source
    if source != source.resolve(strict=True) or source != project / "src/proof.rs":
        raise Incomplete("Constructor compiler source path redirects")
    namespace_target = Path("/work") / target.relative_to(work)
    model = Path(harness.get("goto_file", ""))
    if not model.is_absolute() or not model.is_relative_to(namespace_target) or not str(model).endswith(".symtab.out"):
        raise Incomplete("Constructor compiler model escapes the fresh job target")
    host_model = work / model.relative_to("/work")
    stem = str(host_model).removesuffix(".symtab.out")
    copies = {"compiler-metadata.json": metadata_path, "model.symtab.out": host_model, "model.out": Path(stem + ".out")}
    for suffix in (".type_map.json", ".pretty_name_map.json", ".restrictions.json"):
        path = Path(stem + suffix)
        if path.relative_to(output).as_posix() in files:
            copies["model" + suffix] = path
    graph = {item["name"]: item for item in prepared["registry_graph"]}
    if len(graph) != 14:
        raise Incomplete("Constructor proof graph is not the reviewed fourteen packages")
    features = {name: [] for name in graph}
    for name, fact in files.items():
        path = output / name
        if fact["kind"] != "file" or not name.startswith(target_prefix) or path.parent.name != "fingerprint":
            continue
        if not path.name.startswith("lib-") or not path.name.endswith(".json"):
            continue
        match = re.fullmatch(r"kani/(?:x86_64-unknown-linux-gnu/)?debug/build/([^/]+)/[0-9a-f]{16}/fingerprint/lib-([^/]+)\.json",
                             path.relative_to(target).as_posix())
        if (not match or match[1] not in {*graph, CRATE.replace("_", "-")}
                or match[2] != match[1].replace("-", "_")):
            raise Incomplete("Constructor fingerprint ancestry/package/library differs")
        if match[1] in graph:
            value = check.decode(check.read_once(path, maximum=256 * 1024))
            encoded = value.get("features") if type(value) is dict else None
            if type(encoded) is not str or check.decode(encoded.encode()) != graph[match[1]]["expected_features"]:
                raise Incomplete("Constructor compiled dependency features differ")
            features[match[1]].append(graph[match[1]]["expected_features"])
            copies["fingerprint-" + match[1] + "-" + str(len(features[match[1]])) + ".json"] = path
    if any(not values for values in features.values()):
        raise Incomplete("Constructor proof did not compile all fourteen dependency feature sets")
    facts = {}
    for name, path in copies.items():
        expected = files.get(path.relative_to(output).as_posix())
        if type(expected) is not dict or expected.get("kind") != "file":
            raise Incomplete("Constructor model/metadata lacks original raw output facts")
        COPY.copy_file(path, evidence / name, expected, MAX_ARTIFACT)
        facts[name] = expected
    require_raw(check, output, raw)
    return {"compiler_metadata": metadata, "observed_package_features": features,
            "retained_files": facts, "compiler_filter_source": FILTER_SOURCE}


def qualify_cargo(report: dict, native: dict, job: dict, project: Path, target: Path, work: Path) -> dict:
    metadata = report.get("project") if type(report) is dict else None
    namespace_target = Path("/work") / target.relative_to(work)
    if (type(metadata) is not dict or metadata.get("workspace_root") != str(project)
            or metadata.get("crate_name") != [CRATE] or type(metadata.get("output_dir")) is not str
            or not Path(metadata["output_dir"]).is_absolute()
            or not Path(metadata["output_dir"]).is_relative_to(namespace_target)):
        raise Incomplete("Cargo export project identity differs from the frozen constructor project")
    harnesses = report.get("harness_metadata")
    if (type(harnesses) is not list or len(harnesses) != 1 or harnesses[0].get("crate_name") != CRATE
            or any(harnesses[0].get(name) is not False for name in (
                "is_automatically_generated", "is_bounded", "is_ctor_based"))):
        raise Incomplete("Cargo export is not the ordinary full-raw-domain constructor harness")
    return FORMAL.qualify(report, native, job, "src/proof.rs", expected_solver="cadical")


def execute_job(check, output: Path, original_root: Path, frozen: Path, island: Path, bundle: Path,
                vendor, vendor_before: dict, host: dict, records: dict, names: list[str],
                project: Path, prepared: dict, job: dict, index: int, retained: list,
                campaign_identity: dict | None) -> dict:
    started = time.monotonic()
    work = output / "work"
    folder = work / "jobs" / f"{index:02d}"
    pre, evidence = folder / "pre", folder / "evidence"
    pre.mkdir(mode=0o700, parents=True)
    evidence.mkdir(mode=0o700)
    target = work / "targets" / f"{index:02d}"
    export = evidence / "kani.json"
    result = {"job": job, "index": index, "expected_outcome_qualified": False,
              "native_top_level_attempts": 0, "native_top_level_returned": 0, "output": folder}
    raw = before_tools = None
    try:
        CODEGEN.require_sources(check, frozen, records, names, original_root=original_root)
        require_ancestry(island, output)
        FORMAL.audit_retained(check, output, retained)
        CODEGEN.require_cargo_configuration(check, project, vendor_before, work, host)
        current_vendor = vendor.admit(frozen, bundle, work_root=work)
        if vendor.identity(current_vendor) != vendor.identity(vendor_before):
            raise Incomplete("Constructor vendor identity changed before proof")
        write(pre / "vendor.json", current_vendor)
        before_tools = admit_tools(check, frozen, pre / "admission.json")
        result["tool_identity"] = CODEGEN.tool_identity(before_tools["admitted"])
        if campaign_identity is not None and result["tool_identity"] != campaign_identity:
            raise Incomplete("Constructor tool identity changed between proof jobs")
        payload, compiler = FORMAL.proof_readonly_roots(before_tools["admitted"], "kani-proof")
        readonly = (payload, compiler, island, bundle)
        for path in readonly:
            CODEGEN.readonly_directory(check, path, work)
        if len(set(readonly)) != 4 or any(a != b and (a.is_relative_to(b) or b.is_relative_to(a)) for a in readonly for b in readonly):
            raise Incomplete("Constructor proof requires exactly four disjoint readonly roots")
        command = [str(payload / "bin/kani-driver"), "kani", "--manifest-path", str(project / "Cargo.toml"),
                   "--harness", job["harness"], "--exact", *FLAGS,
                   "--export-json", "/work/" + export.relative_to(work).as_posix(),
                   "--target-dir", "/work/" + target.relative_to(work).as_posix()]
        # Admission itself must not change any already authenticated input or
        # earlier proof receipt before the next command consumes this candidate.
        CODEGEN.require_sources(check, frozen, records, names, original_root=original_root)
        require_ancestry(island, output)
        CODEGEN.require_cargo_configuration(check, project, vendor_before, work, host)
        FORMAL.audit_retained(check, output, retained)
        if target.exists() or target.is_symlink():
            raise Incomplete("Constructor cold target must be absent before proof submission")
        result.update(command=command, native_top_level_attempts=1)
        write(folder / "execution.json", result)
        current_roots = [pre, folder / "execution.json"]
        before = check.input_snapshot(output, current_roots)
        candidates = [path for item in retained for path in item["roots"] if path.is_relative_to(work)] + current_roots
        sealed = []
        for path in sorted(set(candidates), key=lambda item: len(item.parts)):
            if not any(path.is_relative_to(parent) for parent in sealed):
                sealed.append(path)
        execute_started = time.monotonic()
        try:
            native = check.RUNTIME.run(frozen, work, command, readonly=readonly,
                path_dirs=[payload / "bin", compiler / "bin"], cwd=project, sealed=sealed,
                environment={"RUSTC": str(compiler / "bin/rustc"), "RUSTDOC": str(compiler / "bin/rustdoc"),
                             "RUSTUP_TOOLCHAIN": "nightly-2026-08-21"},
                timeout=180, cpu_seconds=180, address_space_bytes=8 * 1024 ** 3, process_limit=128, build_jobs=2)
        finally:
            result["execute_seconds"] = time.monotonic() - execute_started
        result.update(native=native, native_top_level_returned=1)
        check.RUNTIME._owned_work(work, original_root)
        # This first snapshot precedes ALL interpretation of export/compiler JSON.
        raw = snapshot_raw(check, output, target, export)
        write(folder / "raw-custody.json", raw)
        if check.input_snapshot(output, current_roots) != before:
            raise Incomplete("Constructor pre-command receipts changed")
        FORMAL.audit_retained(check, output, retained)
        schema_error = None
        try:
            observed = observe_selected(check, output, work, target, project, evidence, raw, job, prepared)
            report = check.decode(check.read_once(export, maximum=MAX_JSON))
            qualified = qualify_cargo(report, native, job, project, target, work)
            write(folder / "artifacts.json", observed)
            # Raw export + accepted compiler/model copies are immutable before
            # post-admission can have any effect on their facts.
            FORMAL.retain_artifacts(check, output, retained, [evidence, folder / "artifacts.json", folder / "raw-custody.json"])
        except Exception as error:
            schema_error = error
        post_vendor = vendor.admit(frozen, bundle, work_root=work)
        write(folder / "post-vendor.json", post_vendor)
        post_tools = admit_tools(check, frozen, folder / "post-admission.json")
        require_raw(check, output, raw)
        if check.input_snapshot(output, current_roots) != before:
            raise Incomplete("Constructor pre-command receipts changed during post-admission")
        FORMAL.audit_retained(check, output, retained)
        CODEGEN.require_sources(check, frozen, records, names, original_root=original_root)
        require_ancestry(island, output)
        CODEGEN.require_cargo_configuration(check, project, vendor_before, work, host)
        if (vendor.identity(post_vendor) != vendor.identity(vendor_before)
                or CODEGEN.tool_identity(post_tools["admitted"]) != CODEGEN.tool_identity(before_tools["admitted"])):
            raise Incomplete("Constructor tool/vendor identity changed during proof")
        if schema_error is not None:
            raise schema_error
        result.update(qualified, expected_outcome_qualified=True)
    except Exception as error:
        result["qualification_error"] = f"{type(error).__name__}: {error}"
    finally:
        result["job_total_seconds"] = time.monotonic() - started
        try:
            write(folder / "execution.json", result)
            if raw is not None and not result["expected_outcome_qualified"]:
                # Preserve failed-schema native data for read-only diagnosis;
                # it is never replaced by a later compiler invocation.
                retained.append(raw)
            FORMAL.retain_artifacts(check, output, retained, [folder])
        except Exception as error:
            result.update(expected_outcome_qualified=False, persistence_error=f"{type(error).__name__}: {error}")
            result.setdefault("qualification_error", result["persistence_error"])
    return result


def run(output: Path, *, vendor_root=None) -> dict:
    try:
        island = output / "source-island"
        island.mkdir(mode=0o700)
        check = load("era_constructor_bootstrap", ROOT / "scripts/check-analysis.py")
        names = input_names()
        records = FORMAL.freeze_inputs(check, island / "frozen", names)
        consumer = load("era_constructor_frozen_dispatch", island / "frozen/tools/formal/verify_era_constructor.py")
    except Exception as error:
        summary = {"status": "incomplete", "jobs": [], "native_top_level_attempts": 0,
                   "native_top_level_returned": 0, "proof_qualified": False, "error": f"{type(error).__name__}: {error}"}
        write(output / "summary.json", summary)
        return summary
    return consumer.run_prepared(output, original_root=ROOT, records=records, names=names, vendor_root=vendor_root)


def run_prepared(output: Path, *, original_root: Path, records: dict, names: list[str], vendor_root=None) -> dict:
    started = time.monotonic()
    frozen, island, bundle, work = ROOT, output / "source-island", output / "vendor-island", output / "work"
    work.mkdir(mode=0o700)
    summary = {"status": "incomplete", "consumer": CONSUMER, "jobs": [], "fixed_jobs": 5,
               "diagnostic_reserve": 2, "automatic_retries": 0, "proof_qualified": False,
               "unwind_bound": UNWIND, "flags": FLAGS, "native_top_level_attempts": 0,
               "native_top_level_returned": 0, "compiler_cache_reused": False}
    check = vendor = vendor_before = host = None
    retained = []
    campaign_identity = None
    try:
        if (frozen != island / "frozen" or original_root == frozen
                or Path(FORMAL.__file__) != frozen / "tools/formal/verify.py"
                or Path(COPY.__file__) != frozen / "tools/formal/inspect_era_model.py"):
            raise Incomplete("Constructor proof requires actual frozen consumer/helpers")
        check = load("era_constructor_analysis", frozen / "scripts/check-analysis.py")
        if (Path(check.__file__) != frozen / "scripts/check-analysis.py"
                or Path(check.RUNTIME.__file__) != frozen / "scripts/analysis-runtime.py" or input_names() != names):
            raise Incomplete("Constructor proof analysis/runtime/source closure differs")
        CODEGEN.require_sources(check, frozen, records, names, original_root=original_root)
        FORMAL.retain_artifacts(check, output, retained, [frozen])
        vendor = load("era_constructor_vendor", frozen / "tools/formal/era_vendor_admission.py")
        vendor.prepare(frozen, bundle, vendor_root=vendor_root, work_root=work)
        vendor_before = vendor.admit(frozen, bundle, work_root=work)
        if (vendor_before["readonly_root"] != bundle or vendor_before["vendor_root"] != bundle / "vendor"
                or vendor_before["config_path"] != bundle / "cargo-config.toml"):
            raise Incomplete("Constructor vendor root/config differ from the selected bundle")
        FORMAL.retain_artifacts(check, output, retained, [bundle])
        projection = load("era_constructor_projection", frozen / "tools/formal/era_constructor_projection.py")
        schedule = projection.schedule()
        expected = [
            ("healthy", "proof::era_constructor_full_domain", None, False),
            ("reject_singleton", "proof::era_constructor_control_singleton", "singleton public window admitted", False),
            ("swap_retained_endpoints", "proof::era_constructor_control_retention", "ordered lower endpoint retained", False),
            ("wrong_validation_error", "proof::era_constructor_control_error", "invalid endpoint mapped to public window error", False),
            ("healthy", "proof::era_constructor_full_domain", None, True),
        ]
        if ([(job.get("case"), job.get("harness"), job.get("intended_failure"), job.get("restored_healthy")) for job in schedule] != expected
                or any(job.get("subject") != "era_constructor" for job in schedule)):
            raise Incomplete("Constructor proof schedule differs from its fixed five commands")
        projects = []
        for index, job in enumerate(schedule, 1):
            project = island / "projects" / f"{index:02d}"
            prepared = projection.prepare_era_constructor(frozen, project, job["case"],
                frozen / "tools/formal/era_constructor_harness.rs", vendor_root=vendor_before["vendor_root"])
            if prepared.get("entrypoint") != project / "Cargo.toml":
                raise Incomplete("Constructor projection returned a different Cargo manifest")
            validate_project(check, output, project, prepared)
            FORMAL.retain_artifacts(check, output, retained, [project])
            projects.append((project, prepared))
        if projects[0][1]["compiled_era_sha256"] != projects[-1][1]["compiled_era_sha256"]:
            raise Incomplete("Restored constructor source does not match initial healthy source")
        require_ancestry(island, output)
        host = CODEGEN.prepare_host_linker(check, frozen, work)
        write(work / "preparation.json", {"source_manifest": records, "schedule": schedule,
                                           "projects": [prepared for _, prepared in projects], "vendor": vendor_before})
        FORMAL.retain_artifacts(check, output, retained, [host["path"], work / "preparation.json"])
        for path in sorted(island.rglob("*"), reverse=True):
            path.chmod(0o500 if path.is_dir() else 0o400)
        island.chmod(0o500)
        for index, (job, (project, prepared)) in enumerate(zip(schedule, projects), 1):
            result = execute_job(check, output, original_root, frozen, island, bundle, vendor, vendor_before,
                host, records, names, project, prepared, job, index, retained, campaign_identity)
            if campaign_identity is None:
                campaign_identity = result.get("tool_identity")
            summary["jobs"].append(result)
            summary["native_top_level_attempts"] += result["native_top_level_attempts"]
            summary["native_top_level_returned"] += result["native_top_level_returned"]
            write(output / "summary.json", summary)
            if not result["expected_outcome_qualified"]:
                raise Incomplete(f"Stopped at first unqualified constructor job: {index}")
        summary.update(status="passed", proof_qualified=True)
    except Exception as error:
        summary["error"] = f"{type(error).__name__}: {error}"
    finally:
        if check is not None:
            try:
                final_admission = admit_tools(check, frozen, work / "final-admission.json")
                final_tool = check.input_snapshot(output, [work / "final-admission.json"])
                if (campaign_identity is not None
                        and CODEGEN.tool_identity(final_admission["admitted"]) != campaign_identity):
                    raise Incomplete("Final constructor tool identity differs from campaign")
                if vendor is not None and vendor_before is not None:
                    final_vendor = vendor.admit(frozen, bundle, work_root=work)
                    write(work / "final-vendor.json", final_vendor)
                    if vendor.identity(final_vendor) != vendor.identity(vendor_before):
                        raise Incomplete("Final constructor vendor identity differs")
                check.RUNTIME._owned_work(work, original_root)
                CODEGEN.require_sources(check, frozen, records, names, original_root=original_root)
                require_ancestry(island, output)
                FORMAL.audit_retained(check, output, retained)
                if check.input_snapshot(output, [work / "final-admission.json"]) != final_tool:
                    raise Incomplete("Final constructor tool receipt changed during vendor closeout")
                if summary["status"] == "passed":
                    summary["all_required_evidence_unchanged"] = True
            except Exception as error:
                summary.update(status="incomplete", proof_qualified=False, final_error=f"{type(error).__name__}: {error}")
        summary["total_seconds"] = time.monotonic() - started
        write(output / "summary.json", summary)
    return summary


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", required=True, help="fresh private canonical directory outside the checkout")
    parser.add_argument("--vendor-root", type=Path, help="canonical source cache; only the fixed fourteen packages are copied")
    args = parser.parse_args(argv)
    os.umask(0o077)
    summary = run(FORMAL.fresh_output(args.output_dir), vendor_root=args.vendor_root)
    print(json.dumps({key: summary[key] for key in ("status", "native_top_level_attempts", "native_top_level_returned", "proof_qualified")}))
    return 0 if summary["status"] == "passed" else 1


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, ValueError, Incomplete) as error:
        print(f"Era constructor proof incomplete: {error}", file=sys.stderr)
        raise SystemExit(1)
