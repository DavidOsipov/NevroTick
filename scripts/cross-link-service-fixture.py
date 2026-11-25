#!/usr/bin/env python3
"""Cross-link fixed inert lab images or the service fixture library's libtest.

Linux execution only. Never execute, sign, install or package a Windows image.
Authoritative code, raw records and copied PE/PDB outputs stay in the checkout;
derived build inputs and compiler outputs may use an explicit private scratch root.
Existing GPL-2.0-only repository licence applies.
"""
from __future__ import annotations

import sys

if sys.flags.optimize:
    raise SystemExit("Service cross-link admission refuses optimized Python.")

import argparse
import fcntl
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import tempfile
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
FIXTURE_PACKAGE = "nevrotick-windows-service-fixture"
FIXTURE_LIBRARY = "nevrotick_windows_service_fixture"
LIBRARY_SCOPES = ("crt/lib/x86_64", "sdk/lib/um/x86_64", "sdk/lib/ucrt/x86_64")
BASELINE_CLANG = Path("/usr/lib/llvm-18/bin/clang")
BASELINE_CLANG_INPUTS = {
    BASELINE_CLANG: "8ef402d453d1ba4902e4ee0f0f847f6cfa01400c95aa43c24e97818b9c0e3f45",
    BASELINE_CLANG.parent.parent / "lib/libclang-cpp.so.18.1":
        "b9592746f7f8ff1156680fb1fad5faf5486df7a67aca8327cc959463416c6718",
    BASELINE_CLANG.parent.parent / "lib/libLLVM.so.1":
        "5c3a63957e28c15cdf1503892074f032d29b72e36a9824a77dfa87e48a0dafb0",
}
SERVICE_ENTRY_SOURCE = Q.TOOL + "/msix/service_entry.c"
# Both original fixed-role images use this direct import inventory. The
# actual loader resolution and initialization are separate native obligations.
SERVICE_PREENTRY_IMPORTS = (
    "advapi32.dll", "api-ms-win-core-console-l1-1-0.dll",
    "api-ms-win-core-errorhandling-l1-1-0.dll", "api-ms-win-core-errorhandling-l1-1-1.dll",
    "api-ms-win-core-heap-l1-1-0.dll", "api-ms-win-core-interlocked-l1-1-0.dll",
    "api-ms-win-core-libraryloader-l1-2-0.dll", "api-ms-win-core-libraryloader-l1-2-1.dll",
    "api-ms-win-core-localization-l1-2-0.dll",
    "api-ms-win-core-processenvironment-l1-1-0.dll", "api-ms-win-core-profile-l1-1-0.dll",
    "api-ms-win-core-rtlsupport-l1-1-0.dll", "api-ms-win-core-string-l1-1-0.dll",
    "api-ms-win-core-synch-l1-2-0.dll", "api-ms-win-core-sysinfo-l1-1-0.dll",
    "api-ms-win-core-sysinfo-l1-2-0.dll", "api-ms-win-core-sysinfo-l1-2-4.dll",
    "api-ms-win-crt-heap-l1-1-0.dll", "api-ms-win-crt-locale-l1-1-0.dll",
    "api-ms-win-crt-math-l1-1-0.dll", "api-ms-win-crt-runtime-l1-1-0.dll",
    "api-ms-win-crt-stdio-l1-1-0.dll", "api-ms-win-crt-string-l1-1-0.dll",
    "kernel32.dll", "ntdll.dll", "vcruntime140.dll",
)
PE_READER = "scripts/windows_pe.py"
PE_SPEC = importlib.util.spec_from_file_location("windows_pe", ROOT / PE_READER)
PE = importlib.util.module_from_spec(PE_SPEC)
PE_SPEC.loader.exec_module(PE)
SERVICE_ENTRY_NAME: str = PE.SERVICE_ENTRY_NAME
SERVICE_ENTRY_OBJECT: str = PE.SERVICE_ENTRY_OBJECT


SOURCE_CARGO = "/toolchain/bin/cargo"
SOURCE_CONFIG_SHA256 = hashlib.sha256(
    b'[source.crates-io]\nreplace-with="reviewed"\n[source.reviewed]\ndirectory="/vendor"\n'
).hexdigest()
METRICS_READER = Q.TOOL + "/metrics.py"
METRICS_EXPORT_HELPER = "scripts/run-service-analysis.py"
# The fixture roster includes the broker clock, paired SCM observations and managed address rounds.
# Platform and fixture doctest scopes retain their independently reviewed roster.
SOURCE_TEST_ROSTERS = {
    "fixture-test": "ce9a04cfa97ed4bc38b884386c6cda09c382935378145d738e37cec2fecdd925",
    "platform-test": "f7bd2453a7c5ad3084791c92c2093d2c49a2c81047efec2d2d2350bda4de01b2",
    "platform-doc": "a4773395aa848cd76e6793dff9a9c9c5feaa2d7f7c6fa2ae70412c897972d780",
    "fixture-doc": "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945",
}


def read_source_record(path, readers):
    """Decode and hash the same bounded, canonical receipt snapshot."""
    Q.require(path.is_absolute() and path.parent == Q.RECORDS,
              "Source receipt must be a main-repository record.")
    data = C.read_analysis_bytes(path, maximum=C.ANALYSIS_JSON_MAX_BYTES)
    try:
        return readers.decode_record(data), hashlib.sha256(data).hexdigest()
    except readers.Incomplete as error:
        raise Incomplete(str(error)) from error


def require_source_epoch(record, lane, identity, candidate, index):
    """Require producer scope, actual success and unchanged source/index custody."""
    Q.require(type(record.get("schema_version")) is int and record["schema_version"] == 1
              and record.get("lane") == lane and record.get("scope") == "source-only inert service fixture"
              and record.get("status") == "passed" and record.get("bookends_passed") is True
              and record.get("index_bytes_unchanged") is True and record.get("native_execution") is False
              and not record.get("custody_error"), f"The {lane} source receipt is incomplete.")
    Q.require(record.get("source_sha256") == identity and record.get("candidate_sha256") == candidate
              and record.get("recipe_sha256") == identity[Q.SELF],
              f"The {lane} receipt does not describe the current complete source and recipe.")
    Q.require(record.get("index_before") == record.get("index_after") == index,
              f"The {lane} receipt belongs to a different or incomplete index epoch.")


def source_prerequisites(path):
    """Receive the current configured graph and batched ordinary source results."""
    readers = C.load_module("service_cross_link_receipt_readers", ROOT / METRICS_READER)
    checkpoint, checkpoint_digest = read_source_record(path.absolute(), readers)
    policy, identity = Q.source_identity()
    candidate, index = Q.canonical(identity), Q.index_identity()
    require_source_epoch(checkpoint, "privilege-checkpoint", identity, candidate, index)
    binding_keys = ("candidate_sha256", "recipe_sha256", "config_sha256", "feature_delta", "vendor", "tools")
    binding = {key: checkpoint.get(key) for key in binding_keys}
    Q.require(checkpoint.get("graph_binding") == binding and binding["config_sha256"] == SOURCE_CONFIG_SHA256
              and binding["feature_delta"] == Q.manifest_admission(policy)
              and type(binding["vendor"]) is dict and type(binding["tools"]) is dict,
              "Checkpoint graph binding differs from the admitted source configuration.")

    graph_path = checkpoint.get("received_graph_path")
    Q.require(type(graph_path) is str, "Checkpoint omits the raw graph receipt path.")
    graph, graph_digest = read_source_record(Path(graph_path), readers)
    Q.require(checkpoint.get("received_graph_sha256") == graph_digest, "Checkpoint graph receipt digest changed.")
    require_source_epoch(graph, "graph", identity, candidate, index)
    Q.require({key: graph.get(key) for key in binding_keys} == binding,
              "Raw graph receipt fields differ from the checkpoint's graph binding.")
    Q.receive_graph(graph, binding, policy, SOURCE_CARGO)
    Q.require(all(row.get("namespace_argv") == row["argv"] for row in graph["commands"]),
              "Actual graph namespace command differs from its configured invocation.")

    commands = checkpoint.get("commands")
    expected = Q.lane_commands("privilege-checkpoint", SOURCE_CARGO)
    Q.require(type(commands) is list and len(commands) == len(expected),
              "A required actual checkpoint command is missing from the configured checkpoint commands.")
    output_by_command = {}
    for row, argv in zip(commands, expected, strict=True):
        Q.require(type(row) is dict and row.get("argv") == row.get("namespace_argv") == argv
                  and type(row.get("exit_code")) is int and row["exit_code"] == 0
                  and "failure" in row and row["failure"] is None and type(row.get("output")) is str,
                  "A required actual checkpoint command changed or failed.")
        output_by_command[tuple(argv)] = row["output"]
    for lane, roster in (
        ("platform-test", "platform-test"), ("platform-optimized", "platform-test"),
        ("fixture-test", "fixture-test"), ("fixture-optimized", "fixture-test"),
        ("platform-doc", "platform-doc"), ("fixture-doc", "fixture-doc"),
    ):
        argv, = Q.lane_commands(lane, SOURCE_CARGO)
        try:
            names = readers.executed_tests(output_by_command[tuple(argv)], allow_empty=lane == "fixture-doc")
        except readers.Incomplete as error:
            raise Incomplete(f"{lane}: {error}") from error
        Q.require(Q.canonical(names) == SOURCE_TEST_ROSTERS[roster], f"The {lane} executed test roster changed.")
    return policy, identity, checkpoint, {graph_path: graph_digest}, checkpoint_digest


def pe_readback(path):
    """Use the shared static PE reader with the selected fixture console role."""
    try:
        return PE.pe_readback(path, subsystem="console")
    except ValueError as error:
        raise Incomplete(str(error)) from error


def read_guarded_service_entry(data: bytes, map_text: str) -> dict[str, object]:
    """Bind the raw entry, zero pre-entry callbacks and existing CRT symbol."""
    try:
        observed: dict[str, object] = PE.service_link_preentry_declarations(data, map_text)
    except ValueError as error:
        raise Incomplete(str(error)) from error
    Q.require(observed["direct_import_modules"] == list(SERVICE_PREENTRY_IMPORTS),
              "Fixed service loader import inventory changed.")
    return observed


def libtest_artifact(output, work):
    """Require one fresh, package-bound release libtest at its private Cargo path."""
    package_root = "/work/source/" + Q.TOOL
    expected_path = (
        re.escape(f"/work/target/{TARGET}/release/build/{FIXTURE_PACKAGE}/")
        + r"([0-9a-f]{16})/out/" + FIXTURE_LIBRARY + r"-\1\.exe"
    )
    found = None
    finished = False
    for line in output.splitlines():
        if not line.startswith("{"):
            continue
        item = C.ANALYSIS.decode(line.encode("utf-8"))
        Q.require(not finished, "Cargo emitted JSON after its libtest build completion.")
        if item.get("reason") == "build-finished":
            Q.require(item.get("success") is True, "Cargo reports unsuccessful libtest compilation.")
            finished = True
        if item.get("reason") != "compiler-artifact" or item.get("executable") is None:
            continue
        Q.require(found is None, "Expected one actual emitted fixture libtest executable.")
        target, profile = item.get("target"), item.get("profile")
        Q.require(item.get("package_id") == f"path+file://{package_root}#{FIXTURE_PACKAGE}@0.0.0"
                  and item.get("manifest_path") == package_root + "/Cargo.toml"
                  and type(target) is dict and target.get("name") == FIXTURE_LIBRARY
                  and target.get("kind") == target.get("crate_types") == ["lib"]
                  and target.get("src_path") == package_root + "/src/lib.rs"
                  and target.get("edition") == "2024" and target.get("test") is True
                  and type(profile) is dict and profile.get("test") is True
                  and profile.get("opt_level") == "3" and profile.get("debug_assertions") is False
                  and profile.get("overflow_checks") is True
                  and type(profile.get("debuginfo")) is int and profile["debuginfo"] == 1
                  and item.get("features") == [] and item.get("fresh") is False,
                  "Fixture libtest source or release profile differs from the admitted target.")
        executable = item["executable"]
        Q.require(type(executable) is str and re.fullmatch(expected_path, executable),
                  "Unexpected fixture libtest output path.")
        pdb = str(Path(executable).with_suffix(".pdb"))
        Q.require(item.get("filenames") in ([executable], [executable, pdb], [pdb, executable]),
                  "Cargo filenames differ from the selected fixture libtest outputs.")
        found = item
    Q.require(finished and found is not None, "Expected one actual emitted fixture libtest executable.")
    image = work / Path(found["executable"]).relative_to("/work")
    Q.require(image.is_file() and image.resolve(strict=True) == image,
              "Fixture libtest output path must resolve to the private emitted executable.")
    return image, found


def require_disk_headroom(stage, *, pending_bytes=0):
    """Keep the checkout's reserve after any pending artifact copies."""
    available = shutil.disk_usage(ROOT).free
    required = Q.DISK_FLOOR + pending_bytes
    Q.require(available >= required,
              f"Cross-link disk reserve before {stage}: required {required} bytes, available {available} bytes.")


def check_scratch_headroom(scratch, stage):
    """Enforce the service runner's disk and memory budget on relocated work."""
    reason = Q.resource_stop_reason(scratch)
    Q.require(reason is None, f"Cross-link scratch resource stop before {stage}: {reason}.")


def run(args):
    """Admit current source/SDK/tools, then perform only the explicitly selected Linux lane."""
    Q.no_ambient_flags()
    core_policy = Q.disable_core_dumps()
    Q.require(args.lane in ("ready", "link", "libtest-link"), "Unknown service cross-link lane.")
    baseline_entry: bool = getattr(args, "baseline_entry", False)
    Q.require(not baseline_entry or args.lane == "link",
              "Baseline entry is only available for the two fixed service roles.")
    Q.require(re.fullmatch(r"[a-z0-9][a-z0-9-]{0,63}", args.label) is not None, "Invalid fresh label.")
    destination = Q.RECORDS / ("cross-" + args.label + ".json")
    Q.require(not destination.exists() and not destination.is_symlink(), "A prior cross-link record already exists.")
    require_disk_headroom("admission")
    scratch = args.scratch_root
    if scratch is not None:
        scratch = C.RUNTIME._directory(scratch, "Service cross-link scratch root")
        C.RUNTIME._owned_work(scratch, ROOT)
        Q.require(not C.RUNTIME._overlap(scratch, Q.CACHE), "Cross-link scratch overlaps the shared writer cache.")
        check_scratch_headroom(scratch, "admission")
    Q.CACHE.mkdir(mode=0o700, parents=True, exist_ok=True)
    Q.require(Q.CACHE == Q.CACHE.resolve() and Q.CACHE.stat().st_mode & 0o077 == 0, "Derived cache is not private.")
    writer = os.open(Q.CACHE / "writer.lock", os.O_CREAT | os.O_RDWR | os.O_CLOEXEC | os.O_NOFOLLOW, 0o600)
    try:
        fcntl.flock(writer, fcntl.LOCK_EX | fcntl.LOCK_NB)
        Q.require(not destination.exists() and not destination.is_symlink(), "A prior cross-link record already exists.")
        work = (scratch or Q.CACHE) / ("cross-" + args.label)
        Q.require(not work.exists() and not work.is_symlink(), "Cross-link work path already exists.")
    except BaseException:
        os.close(writer)
        raise
    record = {"scope": "Linux cross-link and static PE declarations only", "lane": args.lane, "status": "incomplete",
              "commands": [], "bookends_passed": False, "native_execution": False, "started_unix": time.time(),
              "work": str(work), "limits": {"jobs": 1, "cpu_seconds": 180, "wall_seconds": 180,
                         "core_bytes": core_policy["limit_bytes"], "core_dump_policy": core_policy,
                         "disk_floor_bytes": Q.DISK_FLOOR}}
    if scratch is not None:
        record["limits"].update(scratch_disk_floor_bytes=Q.DISK_FLOOR,
                                memory_available_floor_bytes=Q.MEMORY_FLOOR)
    staging = None
    try:
        metrics_helper = ROOT / METRICS_EXPORT_HELPER
        metrics_helper_sha256 = C.PRODUCT.sha256(metrics_helper)
        policy, identity, checkpoint, prerequisites, checkpoint_digest = source_prerequisites(args.source_checkpoint)
        record.update(source_sha256=identity, candidate_sha256=checkpoint["candidate_sha256"],
                      recipe_sha256=C.PRODUCT.sha256(ROOT / SELF), pe_reader_sha256=C.PRODUCT.sha256(ROOT / PE_READER),
                      metrics_export_helper_sha256=metrics_helper_sha256,
                      prerequisite_records_sha256={str(Path(path).relative_to(ROOT)): digest
                                                   for path, digest in prerequisites.items()},
                      checkpoint_path=str(args.source_checkpoint.absolute()), checkpoint_sha256=checkpoint_digest,
                      index_before=Q.index_identity())
        Q.require(record["index_before"] == checkpoint["index_after"], "Source checkpoint index changed before cross admission.")
        admitted = C.ANALYSIS.admit(ROOT, profile="stable-core")
        stable = admitted["stable"]
        Q.require(checkpoint["tools"].get("stable") == str(stable)
                  and checkpoint["tools"] == Q.tool_identity(Q.admit_service_tools()),
                  "Cross tools differ from the current source checkpoint.")
        sdk_config, sdk_inventory = args.sdk_config.resolve(strict=True), args.sdk_inventory.resolve(strict=True)
        sdk_hashes = {str(path): C.PRODUCT.sha256(path) for path in (sdk_config, sdk_inventory)}
        sdk_settings = C.read_analysis_json(sdk_config)
        C.read_analysis_json(sdk_inventory)
        vendor = args.vendor_dir.resolve(strict=True)
        if scratch is not None:
            readonly_inputs = [*admitted["readonly_roots"], vendor,
                               Path(sdk_settings["root"]).resolve(strict=True), sdk_config, sdk_inventory]
            Q.require(not any(C.RUNTIME._overlap(scratch, path) for path in readonly_inputs),
                      "Cross-link scratch overlaps a read-only input.")
        work.mkdir(mode=0o700)
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
        entry_object: Path = work / SERVICE_ENTRY_OBJECT
        entry_sha256: str | None = None
        entry_setup: str = ""
        if baseline_entry:
            Q.require(SERVICE_ENTRY_SOURCE in identity, "Baseline entry is missing from the admitted source.")
            # This recipe admits one normal output per fixed role. Response
            # files or a preexisting entry/map could hide a competing argument.
            entry_setup = (
                "import re\n"
                "outs = [item[5:] for item in args if item.lower().startswith('/out:')]\n"
                "out = re.fullmatch("
                "r'/work/target/x86_64-pc-windows-msvc/release/build/nevrotick-windows-service-fixture/"
                "[0-9a-f]{16}/out/(nevrotick_lab_s02_(?:engine|broker))\\.exe', outs[0]) if len(outs) == 1 else None\n"
                "if out is None or any(item.startswith('@') or "
                "item.lower().startswith(('/entry', '-entry', '/map', '-map')) for item in args):\n"
                " raise SystemExit('Unadmitted baseline service link arguments.')\n"
                f"args += ['/entry:{SERVICE_ENTRY_NAME}', '/map:/work/' + out.group(1) + '.map', "
                f"'/work/{SERVICE_ENTRY_OBJECT}']\n"
            )
        wrapper.write_text("#!/usr/bin/python3 -I\nimport json,os,sys\nargs=sys.argv[1:]\n"
                           + entry_setup + "argv=" + repr(paths)
                           + " + args\nwith open('/work/linker-calls.jsonl','a',encoding='utf-8') as stream:\n"
                           + " stream.write(json.dumps(argv)+'\\n')\nos.execv(argv[0],argv)\n")
        wrapper.chmod(0o500)
        wrapper_sha = C.PRODUCT.sha256(wrapper)
        record["linker_prefix_argv"] = paths
        vendor_map = C.PRODUCT.admit_vendor(vendor, policy)
        record["vendor"] = {"path": str(vendor), "inventory_sha256": Q.canonical(vendor_map)}
        Q.require(checkpoint["vendor"] == {**record["vendor"], "files": len(vendor_map)},
                  "Cross vendor differs from the current source checkpoint.")
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
            Q.require(Q.check_core_dumps() == core_policy, "Cross core collector policy changed.")
            Q.require(Q.source_identity()[1] == identity and C.PRODUCT.inventory(source) == identity,
                      "Current or sealed source changed.")
            Q.require(C.PRODUCT.sha256(ROOT / SELF) == record["recipe_sha256"]
                      and C.PRODUCT.sha256(ROOT / PE_READER) == record["pe_reader_sha256"]
                      and C.PRODUCT.sha256(metrics_helper) == metrics_helper_sha256
                      and C.PRODUCT.sha256(config) == config_sha and C.PRODUCT.sha256(wrapper) == wrapper_sha,
                      "Cross recipe, sealed configuration or linker changed.")
            Q.require(Q.index_identity() == record["index_before"], "Staging bytes changed during cross lane.")
            if baseline_entry:
                Q.require(all(path.resolve(strict=True) == path and C.PRODUCT.sha256(path) == digest
                              for path, digest in BASELINE_CLANG_INPUTS.items())
                          and (entry_sha256 is None or C.PRODUCT.sha256(entry_object) == entry_sha256),
                          "Baseline entry compiler or object changed.")
            for name, digest in {**sdk_hashes, **prerequisites,
                                 record["checkpoint_path"]: record["checkpoint_sha256"]}.items():
                Q.require(C.PRODUCT.sha256(Path(name)) == digest, "Prerequisite or SDK receipt changed.")

        def execute(command):
            """Run one finite literal host command with sealed inputs and preserve its actual result."""
            check()
            require_disk_headroom("command execution")
            options = dict(readonly=[*admitted["readonly_roots"], vendor, *library_roots],
                           sealed=[source, config, wrapper, *([entry_object] if entry_sha256 else [])],
                           path_dirs=[stable / "bin", llvm],
                           environment={"RUSTC": str(stable / "bin/rustc"), "RUSTDOC": str(stable / "bin/rustdoc"),
                                        "RUST_TEST_THREADS": "1"}, cwd=source, cpu_seconds=180, build_jobs=1)
            if scratch is None:
                result = C.RUNTIME.run(ROOT, work, command, timeout=180, **options)
            else:
                check_scratch_headroom(work, "command execution")
                namespace = C.RUNTIME.sandbox_command(ROOT, work, command, **options)
                result = C.RUNTIME.ENV.run(
                    namespace, cwd=ROOT, timeout=180, environment={"PATH": "/usr/bin:/bin", "LANG": "C.UTF-8"},
                    stdin=subprocess.DEVNULL, resource_guard=lambda: Q.resource_stop_reason(work),
                )
            record["commands"].append({"argv": command, **result})
            check()
            Q.require(result.get("failure") is None and type(result.get("exit_code")) is int,
                      f"Cross command lost runtime custody: {result.get('failure')}.")
            if scratch is not None:
                check_scratch_headroom(work, "receipt readback")
            return result

        if args.lane == "ready":
            results = [execute([str(linker), "-flavor", "link", "--version"]), execute([str(inspector), "--version"])]
            succeeded = all(result["exit_code"] == 0 for result in results)
        else:
            libtest = args.lane == "libtest-link"
            if baseline_entry:
                produced = execute([str(BASELINE_CLANG), "--no-default-config", "--target=" + TARGET, "-std=c17", "-O2",
                    "-Wall", "-Wextra", "-Werror", "-Wpedantic", "-ffreestanding", "-fno-lto",
                    "-fstack-protector-strong", "-funwind-tables", "-Xclang", "-cfguard", "-march=x86-64",
                    "-c", "/work/source/" + SERVICE_ENTRY_SOURCE, "-o", "/work/" + SERVICE_ENTRY_OBJECT])
                Q.require(produced["exit_code"] == 0 and not entry_object.is_symlink(),
                          "Baseline entry object compilation failed.")
                entry_sha256 = C.PRODUCT.sha256(entry_object)
                record["baseline_entry"] = {"compiler_inputs": {str(path): digest
                                                                for path, digest in BASELINE_CLANG_INPUTS.items()},
                                           "object_sha256": entry_sha256}
            subcommand = "test" if libtest else "build"
            selection = ["--lib", "--no-run"] if libtest else ["--bin", BINS[0], "--bin", BINS[1]]
            command = [str(stable / "bin/cargo"), "--verbose", "--verbose", "--config", f'target.{TARGET}.linker="/work/linker"',
                       subcommand, "--manifest-path", "/work/source/" + Q.MANIFEST, "--release", "--target", TARGET,
                       "--frozen", "--offline", "--jobs", "1", *selection,
                       "--message-format=json-render-diagnostics"]
            result = execute(command)
            if libtest:
                namespace_argv = result.get("command")
                Q.require(type(namespace_argv) is list and namespace_argv[-len(command) - 1:] == ["--", *command],
                          "The executed namespace argv differs from the ordered Cargo command.")
            succeeded = result["exit_code"] == 0
            if succeeded:
                if libtest:
                    image, artifact = libtest_artifact(result["output"], work)
                    emitted = {FIXTURE_LIBRARY: image}
                    record["cargo_libtest_artifact"] = artifact
                else:
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
                Q.require(len(calls) == len(emitted) and all(type(call) is list and call[:len(paths)] == paths
                          and all(type(argument) is str for argument in call) for call in calls),
                          "Actual linker call ledger differs from the selected link invocations.")
                if libtest:
                    linked = [argument[5:] for argument in calls[0] if argument.lower().startswith("/out:")]
                    Q.require(linked == [artifact["executable"]],
                              "The linker output differs from the Cargo-attributed fixture libtest.")
                record["actual_linker_argv"] = calls
                artifacts = Q.RECORDS / ("cross-" + args.label + "-artifacts")
                staging = tempfile.TemporaryDirectory(prefix=".cross-publish-", dir=Q.RECORDS)
                staged = Path(staging.name) / "artifacts"
                staged.mkdir(mode=0o700)
                artifact_rows, admitted_hashes = {}, {}
                if baseline_entry:
                    object_data: bytes = C.read_analysis_bytes(entry_object, maximum=64 * 1024)
                    Q.require(hashlib.sha256(object_data).hexdigest() == entry_sha256, "Entry object drifted.")
                    (staged / entry_object.name).write_bytes(object_data)
                    admitted_hashes[entry_object] = entry_sha256
                for name, image in emitted.items():
                    pdb_names = {image.with_suffix(".pdb").name} if libtest else {
                        name + ".pdb", name.replace("-", "_") + ".pdb",
                    }
                    pdbs = [image.parent / value for value in pdb_names if (image.parent / value).is_file()]
                    Q.require(len(pdbs) == 1 and not pdbs[0].is_symlink(), "Matching emitted PDB is missing or ambiguous.")
                    pdb = pdbs[0]
                    readback = pe_readback(image)
                    image_hash = readback["image_sha256"]
                    try:
                        image_snapshot, symbol_snapshot, symbol_identity = PE.read_export_pair(
                            image, pdb, subsystem="console",
                        )
                    except ValueError as error:
                        raise Incomplete(str(error)) from error
                    Q.require(image_snapshot[1] == image_hash, "PE changed after hardening readback.")
                    if baseline_entry:
                        map_path: Path = work / (name.replace("-", "_") + ".map")
                        map_data: bytes = C.read_analysis_bytes(map_path, maximum=2 * 1024 * 1024)
                        entry: dict[str, object] = read_guarded_service_entry(image_snapshot[0], map_data.decode("utf-8"))
                        map_sha256: str = hashlib.sha256(map_data).hexdigest()
                        (staged / map_path.name).write_bytes(map_data)
                        admitted_hashes[map_path] = map_sha256
                        readback["baseline_entry"] = {**entry, "map_sha256": map_sha256}
                    observed = execute([str(inspector), "--file-headers", "--coff-load-config", "--coff-debug-directory",
                                        "--coff-imports", str(Path("/work") / image.relative_to(work))])
                    Q.require(observed["exit_code"] == 0, "Independent LLVM PE inspection failed.")
                    Q.require(C.PRODUCT.sha256(image) == image_hash, "PE changed after LLVM inspection.")
                    if scratch is not None:
                        staging_bytes = len(image_snapshot[0]) + len(symbol_snapshot[0]) + len(observed["output"].encode())
                        require_disk_headroom("artifact staging", pending_bytes=staging_bytes)
                    for original, (snapshot, digest) in ((image, image_snapshot), (pdb, symbol_snapshot)):
                        with (staged / original.name).open("xb") as copied:
                            copied.write(snapshot)
                        admitted_hashes[original] = digest
                    # Release this pair's snapshots before reading the next pair.
                    del image_snapshot, symbol_snapshot, snapshot
                    (staged / (name + "-llvm-readobj.txt")).write_text(observed["output"])
                    artifact_rows[name] = {"image": str((artifacts / image.name).relative_to(ROOT)),
                                           "image_sha256": image_hash,
                                           "pdb": str((artifacts / pdb.name).relative_to(ROOT)),
                                           "pdb_sha256": symbol_identity["sha256"], "pdb_identity": symbol_identity,
                                           "raw_pe": readback}
                Q.require(len({item["image_sha256"] for item in artifact_rows.values()}) == len(emitted),
                          "Distinct fixed-role compositions unexpectedly have identical bytes.")
        check()
        Q.require(C.PRODUCT.library_inventory(libraries) == library_map
                  and C.PRODUCT.admit_vendor(vendor, policy) == vendor_map, "SDK/vendor bytes changed during cross lane.")
        after = C.ANALYSIS.admit(ROOT, profile="stable-core")
        Q.require(all(after[key] == admitted[key] for key in ("stable", "inventories", "readonly_roots",
                    "installation_pointer", "installation_pointer_sha256", "policy_sha256")), "Stable tools changed.")
        check()
        if staging is not None:
            for original, digest in admitted_hashes.items():
                Q.require(C.PRODUCT.sha256(staged / original.name) == C.PRODUCT.sha256(original) == digest,
                          "Copied or emitted artifact changed before publication.")
        index_after = Q.index_identity()
        Q.require(index_after == record["index_before"], "Staging bytes changed before cross-lane publication.")
        if staging is not None:
            if scratch is not None:
                require_disk_headroom("artifact publication")
                check_scratch_headroom(work, "artifact publication")
            Q.require(not artifacts.exists() and not artifacts.is_symlink(),
                      "Cross artifact destination already exists.")
            # The cache writer lock serializes cross-lane publication.
            staged.rename(artifacts)
            record["artifacts"] = artifact_rows
        record.update(bookends_passed=True, index_bytes_unchanged=True, index_after=index_after,
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
            try:
                if staging is not None:
                    staging.cleanup()
            finally:
                os.close(writer)
    return record["status"]


def main():
    """Select readiness, fixed compositions or the isolated fixture libtest."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("lane", choices=("ready", "link", "libtest-link"))
    parser.add_argument("--baseline-entry", action="store_true",
                        help="Use the admitted baseline-first startup for both fixed service roles")
    parser.add_argument("--label", required=True)
    parser.add_argument("--source-checkpoint", type=Path, required=True,
                        help="Current qualification.py privilege-checkpoint receipt and its bound graph")
    parser.add_argument("--vendor-dir", type=Path, required=True)
    parser.add_argument("--scratch-root", type=Path,
                        help="Existing private directory for derived link inputs and compiler outputs")
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
