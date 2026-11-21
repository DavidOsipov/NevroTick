#!/usr/bin/env python3
"""Cross-link the two fixed inert lab images and inspect their declarations.

Linux execution only. Never execute, sign, install or package a Windows image.
Authoritative code, raw records and copied PE/PDB outputs stay in the checkout;
the private persistent cache holds derived build inputs and compiler outputs.
Existing GPL-2.0-only repository licence applies.
"""
from __future__ import annotations

import sys

if sys.flags.optimize:
    raise SystemExit("Service cross-link admission refuses optimized Python.")

import argparse
import fcntl
import importlib.util
import json
import os
from pathlib import Path
import re
import resource
import shutil
import time
import tomllib

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[1]
SELF = "scripts/cross-link-service-fixture.py"
SPEC = importlib.util.spec_from_file_location("service_fixture_source_custody", ROOT / "tools/windows-service-fixture/qualification.py")
Q = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(Q)
C = Q.C
Incomplete = Q.Incomplete
TARGET = "x86_64-pc-windows-msvc"
BINS = ("nevrotick-lab-s02-engine", "nevrotick-lab-s02-broker")
LIBRARY_SCOPES = ("crt/lib/x86_64", "sdk/lib/um/x86_64", "sdk/lib/ucrt/x86_64")
PE_READER = "scripts/windows_pe.py"
PE_SPEC = importlib.util.spec_from_file_location("windows_pe", ROOT / PE_READER)
PE = importlib.util.module_from_spec(PE_SPEC)
PE_SPEC.loader.exec_module(PE)


def source_prerequisites(path):
    """Receive complete current ordinary source evidence, never summary flags alone."""
    path = path.resolve(strict=True)
    Q.require(path.parent == Q.RECORDS, "Source checkpoint must be a main-repository record.")
    checkpoint = C.read_analysis_json(path)
    policy, identity = Q.source_identity()
    Q.require(checkpoint.get("current_source_matches") is True
              and checkpoint.get("source_sha256") == identity
              and checkpoint.get("candidate_sha256") == Q.canonical(identity)
              and checkpoint.get("recipe_sha256") == C.PRODUCT.sha256(ROOT / Q.SELF),
              "Source checkpoint does not describe the current complete compiler inputs.")
    expected = {"graph", "fixture-test", "platform-test", "platform-clippy", "fixture-clippy", "windows-clippy",
                "platform-fmt", "fixture-fmt", "platform-doc", "fixture-doc", "fixture-rustdoc",
                "platform-optimized", "fixture-optimized"}
    received, records = set(), {}
    for row in checkpoint["records"]:
        record_path = ROOT / row["path"]
        Q.require(record_path.parent == Q.RECORDS and record_path == record_path.resolve(strict=True)
                  and C.PRODUCT.sha256(record_path) == row["sha256"], "Prerequisite raw record changed.")
        record = C.read_analysis_json(record_path)
        lane = record.get("lane")
        Q.require(lane in expected and lane not in received and record.get("status") == "passed"
                  and record.get("bookends_passed") is True and record.get("index_bytes_unchanged") is True
                  and not record.get("custody_error") and record.get("source_sha256") == identity
                  and record.get("candidate_sha256") == checkpoint["candidate_sha256"]
                  and record.get("index_before") == record.get("index_after"), "A source prerequisite is incomplete.")
        commands = record.get("commands")
        Q.require(type(commands) is list and len(commands) == (4 if lane == "graph" else 1)
                  and all(type(command.get("exit_code")) is int and command["exit_code"] == 0
                          and "failure" in command and command["failure"] is None for command in commands),
                  "A required actual source command is missing or failed.")
        cargo = record["tools"]["stable"] + "/bin/cargo"
        if lane == "graph":
            Q.receive_graph(record, record["graph_binding"], policy, cargo)
        else:
            Q.require([command["argv"] for command in commands] == Q.lane_commands(lane, cargo),
                      "Source prerequisite command differs from its literal lane.")
        received.add(lane)
        records[str(record_path.relative_to(ROOT))] = row["sha256"]
    Q.require(received == expected, "Source checkpoint omits a required lane.")
    for owner, count in (("platform_debug", 39), ("fixture_debug", 37), ("platform_release", 39),
                         ("fixture_release", 37), ("platform_doctests", 8)):
        Q.require(len(checkpoint["actual_tests"][owner]) == count, "Frozen source cohort changed.")
    return policy, identity, checkpoint, records


def pe_readback(path):
    """Use the shared static PE reader with the selected fixture console role."""
    try:
        return PE.pe_readback(path, subsystem="console")
    except ValueError as error:
        raise Incomplete(str(error)) from error

def run(args):
    """Admit current source/SDK/tools, then perform only the explicitly selected Linux lane."""
    Q.no_ambient_flags()
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
    Q.require(re.fullmatch(r"[a-z0-9][a-z0-9-]{0,63}", args.label) is not None, "Invalid fresh label.")
    destination = Q.RECORDS / ("cross-" + args.label + ".json")
    Q.require(not destination.exists() and shutil.disk_usage(ROOT).free >= Q.DISK_FLOOR,
              "Cross record exists or less than 2GiB free.")
    Q.CACHE.mkdir(mode=0o700, parents=True, exist_ok=True)
    Q.require(Q.CACHE == Q.CACHE.resolve() and Q.CACHE.stat().st_mode & 0o077 == 0, "Derived cache is not private.")
    writer = os.open(Q.CACHE / "writer.lock", os.O_CREAT | os.O_RDWR | os.O_CLOEXEC | os.O_NOFOLLOW, 0o600)
    fcntl.flock(writer, fcntl.LOCK_EX | fcntl.LOCK_NB)
    work = Q.CACHE / ("cross-" + args.label)
    work.mkdir(mode=0o700)
    record = {"scope": "Linux cross-link and static PE declarations only", "lane": args.lane, "status": "incomplete",
              "commands": [], "bookends_passed": False, "native_execution": False, "started_unix": time.time(),
              "work": str(work), "limits": {"jobs": 1, "cpu_seconds": 180, "wall_seconds": 180, "core_bytes": 0}}
    try:
        policy, identity, checkpoint, prerequisites = source_prerequisites(args.source_checkpoint)
        record.update(source_sha256=identity, candidate_sha256=checkpoint["candidate_sha256"],
                      recipe_sha256=C.PRODUCT.sha256(ROOT / SELF), pe_reader_sha256=C.PRODUCT.sha256(ROOT / PE_READER),
                      prerequisite_records_sha256=prerequisites,
                      checkpoint_path=str(args.source_checkpoint.resolve()), checkpoint_sha256=C.PRODUCT.sha256(args.source_checkpoint.resolve()),
                      index_before=Q.index_identity())
        admitted = C.ANALYSIS.admit(ROOT, profile="stable-core")
        stable = admitted["stable"]
        sdk_config, sdk_inventory = args.sdk_config.resolve(strict=True), args.sdk_inventory.resolve(strict=True)
        sdk_hashes = {str(path): C.PRODUCT.sha256(path) for path in (sdk_config, sdk_inventory)}
        C.read_analysis_json(sdk_config)
        C.read_analysis_json(sdk_inventory)
        libraries, library_map = C.PRODUCT.windows_libraries(ROOT, stable, sdk_config, work, sdk_inventory)
        library_roots = [libraries / scope for scope in LIBRARY_SCOPES]
        llvm = stable / "lib/rustlib/x86_64-unknown-linux-gnu/bin"
        linker, inspector = llvm / "rust-lld", llvm / "llvm-readobj"
        Q.require(inspector.is_file() and os.access(inspector, os.X_OK), "Admitted LLVM readback tool is missing.")
        record["tools"] = {"stable": str(stable), "inventories": admitted["inventories"],
                           "pointer_sha256": admitted["installation_pointer_sha256"],
                           "linker_sha256": C.PRODUCT.sha256(linker), "inspector_sha256": C.PRODUCT.sha256(inspector)}
        record["sdk"] = {"root": str(libraries), "record_sha256": sdk_hashes,
                         "library_files": len(library_map), "library_map_sha256": Q.canonical(library_map)}
        flags = tomllib.loads((ROOT / ".cargo/config.toml").read_text())["target"][TARGET]["rustflags"]
        Q.require(flags == C.PRODUCT.ENV.MSVC_FLAGS, "Complete required v3/hardening target flags changed.")
        record["windows_rustflags"] = flags
        record["target_cpu"] = "x86-64-v3"
        paths = [str(linker), "-flavor", "link", *("/libpath:" + str(path) for path in library_roots)]
        wrapper = work / "linker"
        wrapper.write_text("#!/usr/bin/python3 -I\nimport json,os,sys\nargv=" + repr(paths)
                           + " + sys.argv[1:]\nwith open('/work/linker-calls.jsonl','a',encoding='utf-8') as stream:\n"
                           + " stream.write(json.dumps(argv)+'\\n')\nos.execv(argv[0],argv)\n")
        wrapper.chmod(0o500)
        wrapper_sha = C.PRODUCT.sha256(wrapper)
        record["linker_prefix_argv"] = paths
        vendor = args.vendor_dir.resolve(strict=True)
        vendor_map = C.PRODUCT.admit_vendor(vendor, policy)
        record["vendor"] = {"path": str(vendor), "inventory_sha256": Q.canonical(vendor_map)}
        source = work / "source"
        source.mkdir(mode=0o700)
        for name in identity:
            target = source / name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, target)
        (work / "cargo-home").mkdir(mode=0o700)
        config = work / "cargo-home/config.toml"
        config.write_text('[source.crates-io]\nreplace-with="reviewed"\n[source.reviewed]\ndirectory=' + json.dumps(str(vendor)) + '\n')
        config_sha = C.PRODUCT.sha256(config)

        def check():
            """Keep current source, script, SDK receipt, prerequisites and derived inputs unchanged."""
            Q.require(Q.source_identity()[1] == identity and C.PRODUCT.inventory(source) == identity,
                      "Current or sealed source changed.")
            Q.require(C.PRODUCT.sha256(ROOT / SELF) == record["recipe_sha256"]
                      and C.PRODUCT.sha256(ROOT / PE_READER) == record["pe_reader_sha256"]
                      and C.PRODUCT.sha256(config) == config_sha and C.PRODUCT.sha256(wrapper) == wrapper_sha,
                      "Cross recipe, sealed configuration or linker changed.")
            Q.require(Q.index_identity() == record["index_before"], "Staging bytes changed during cross lane.")
            for name, digest in {**sdk_hashes, **{str(ROOT / name): sha for name, sha in prerequisites.items()},
                                 record["checkpoint_path"]: record["checkpoint_sha256"]}.items():
                Q.require(C.PRODUCT.sha256(Path(name)) == digest, "Prerequisite or SDK receipt changed.")

        def execute(command):
            """Run one finite literal host command with sealed inputs and preserve its actual result."""
            check()
            Q.require(shutil.disk_usage(ROOT).free >= Q.DISK_FLOOR, "Less than 2GiB free before cross command.")
            result = C.RUNTIME.run(ROOT, work, command, timeout=180,
                                   readonly=[*admitted["readonly_roots"], vendor, *library_roots],
                                   sealed=[source, config, wrapper], path_dirs=[stable / "bin", llvm],
                                   environment={"RUSTC": str(stable / "bin/rustc"), "RUSTDOC": str(stable / "bin/rustdoc"),
                                                "RUST_TEST_THREADS": "1"}, cwd=source, cpu_seconds=180, build_jobs=1)
            record["commands"].append({"argv": command, **result})
            check()
            Q.require(result.get("failure") is None and type(result.get("exit_code")) is int,
                      "Cross command lost runtime custody.")
            return result

        if args.lane == "ready":
            results = [execute([str(linker), "-flavor", "link", "--version"]), execute([str(inspector), "--version"])]
            succeeded = all(result["exit_code"] == 0 for result in results)
        else:
            command = [str(stable / "bin/cargo"), "--verbose", "--verbose", "--config", f'target.{TARGET}.linker="/work/linker"',
                       "build", "--manifest-path", "/work/source/" + Q.MANIFEST, "--release", "--target", TARGET,
                       "--frozen", "--offline", "--jobs", "1", "--bin", BINS[0], "--bin", BINS[1],
                       "--message-format=json-render-diagnostics"]
            result = execute(command)
            succeeded = result["exit_code"] == 0
            if succeeded:
                emitted = {}
                for line in result["output"].splitlines():
                    if not line.startswith("{"):
                        continue
                    item = C.ANALYSIS.decode(line.encode("utf-8"))
                    if item.get("reason") != "compiler-artifact" or item.get("executable") is None:
                        continue
                    name = item["target"]["name"]
                    expected = f"/work/target/{TARGET}/release/{name}.exe"
                    Q.require(name in BINS and name not in emitted and item["target"]["kind"] == ["bin"]
                              and item["executable"] == expected and item["profile"]["opt_level"] == "3"
                              and item["profile"]["debug_assertions"] is False, "Unexpected actual Cargo executable.")
                    emitted[name] = work / Path(expected).relative_to("/work")
                Q.require(set(emitted) == set(BINS), "Actual Cargo output omits a fixed composition.")
                linker_log = C.read_analysis_text(work / "linker-calls.jsonl", maximum=1024 * 1024)
                calls = [C.ANALYSIS.decode(line.encode("utf-8")) for line in linker_log.splitlines()]
                Q.require(len(calls) == 2 and all(type(call) is list and call[:len(paths)] == paths
                          and all(type(argument) is str for argument in call) for call in calls),
                          "Actual linker call ledger differs from the two fixed link invocations.")
                record["actual_linker_argv"] = calls
                artifacts = Q.RECORDS / ("cross-" + args.label + "-artifacts")
                artifacts.mkdir(mode=0o700)
                record["artifacts"] = {}
                for name, image in emitted.items():
                    pdb_names = {name + ".pdb", name.replace("-", "_") + ".pdb"}
                    pdbs = [image.parent / value for value in pdb_names if (image.parent / value).is_file()]
                    Q.require(len(pdbs) == 1 and not pdbs[0].is_symlink(), "Matching emitted PDB is missing or ambiguous.")
                    pdb = pdbs[0]
                    readback = pe_readback(image)
                    image_hash = readback["image_sha256"]
                    Q.require(readback["codeview_path"].replace("\\", "/").split("/")[-1] == pdb.name,
                              "Selected PDB does not match the image's CodeView basename.")
                    observed = execute([str(inspector), "--file-headers", "--coff-load-config", "--coff-debug-directory",
                                        "--coff-imports", str(Path("/work") / image.relative_to(work))])
                    Q.require(observed["exit_code"] == 0, "Independent LLVM PE inspection failed.")
                    Q.require(C.PRODUCT.sha256(image) == image_hash, "PE changed after hardening readback.")
                    pdb_hash = C.PRODUCT.sha256(pdb)
                    for original, digest in ((image, image_hash), (pdb, pdb_hash)):
                        copied = artifacts / original.name
                        shutil.copyfile(original, copied)
                        Q.require(C.PRODUCT.sha256(copied) == C.PRODUCT.sha256(original) == digest,
                                  "Copied artifact changed.")
                    (artifacts / (name + "-llvm-readobj.txt")).write_text(observed["output"])
                    record["artifacts"][name] = {"image": str((artifacts / image.name).relative_to(ROOT)),
                                                 "image_sha256": image_hash,
                                                 "pdb": str((artifacts / pdb.name).relative_to(ROOT)),
                                                 "pdb_sha256": pdb_hash, "raw_pe": readback}
                Q.require(len({item["image_sha256"] for item in record["artifacts"].values()}) == 2,
                          "Distinct fixed-role compositions unexpectedly have identical bytes.")
        check()
        Q.require(C.PRODUCT.library_inventory(libraries) == library_map
                  and C.PRODUCT.admit_vendor(vendor, policy) == vendor_map, "SDK/vendor bytes changed during cross lane.")
        after = C.ANALYSIS.admit(ROOT, profile="stable-core")
        Q.require(all(after[key] == admitted[key] for key in ("stable", "inventories", "readonly_roots",
                    "installation_pointer", "installation_pointer_sha256", "policy_sha256")), "Stable tools changed.")
        check()
        record.update(bookends_passed=True, index_bytes_unchanged=True, index_after=Q.index_identity(),
                      status="passed" if succeeded else "failed")
    except Exception as error:
        record["custody_error"] = str(error)
        raise
    finally:
        record["finished_unix"] = time.time()
        try:
            Q.write_new(destination, record)
            print(json.dumps({"record": str(destination), "status": record["status"], "commands": len(record["commands"])}), flush=True)
        finally:
            os.close(writer)
    return record["status"]


def main():
    """Select only readiness or the two-composition Linux cross-link lane."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("lane", choices=("ready", "link"))
    parser.add_argument("--label", required=True)
    parser.add_argument("--source-checkpoint", type=Path, default=Q.RECORDS / "source-checkpoint-v2.json")
    parser.add_argument("--vendor-dir", type=Path, required=True)
    parser.add_argument("--sdk-config", type=Path, default=Path.home() / ".local/share/nevrotick/xwin.json")
    parser.add_argument("--sdk-inventory", type=Path,
                        default=Path("/mnt/c/Users/perso/AppData/Local/NevroTick/ui-m0/xwin-installed-inventory.json"))
    args = parser.parse_args()
    try:
        status = run(args)
    except (Incomplete, C.PRODUCT.ENV.Incomplete, OSError, ValueError, KeyError) as error:
        print(str(error), file=sys.stderr)
        return 1
    return 0 if status == "passed" else 1


if __name__ == "__main__":
    raise SystemExit(main())
