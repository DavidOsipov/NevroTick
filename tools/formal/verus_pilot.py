#!/usr/bin/env python3
"""Run at most three closed, offline Verus compatibility commands.

This candidate-local guard changes no shared runtime, SDK or shipping profile.
Readiness controls cannot qualify an application theorem or adopt a verifier.
The Core task owns the separate, still-unexecuted actual-source proof budget.
"""
from __future__ import annotations

import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import stat
import sys
import time


ROOT = Path(__file__).resolve().parents[2]
STORE = Path.home() / ".local/share/nevrotick/formal-verus-pilot-20261105-v1"
SDK = Path.home() / ".rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu"
# Filled from the independently reviewed authenticated acquisition, before any
# downloaded executable is allowed to run. Empty means execution is forbidden.
MANIFEST_SHA256 = "a30e8e26fea24af0f2f5977d1597cd69d964e7ecbc296ac3709ab8bd5e1e6e6d"
HELPERS = {
    "/usr/bin/env": "886e5fa8be716b03eeda48856e0f04493f2969fc148fb7812594ffac647fa050",
    "/usr/bin/strace": "28f957c227012de0b18d1bd7fff2d396cb693ea60ed8013be68de071e84b5001",
    "/usr/bin/x86_64-linux-gnu-readelf": "64c58e15274bbbb5153f31078e455e9e77ee5f51489e709bba5bb788ce9df2b0",
}
SOURCES = (
    "tools/formal/verus_pilot.py", "tools/formal/verus_pilot_acquire.py",
    "tools/formal/verus_compatibility.rs", "scripts/analysis-runtime.py",
    "scripts/check-analysis.py", "scripts/check-environment.py", "scripts/analysis-controls.py",
    "policy/analysis-tools.json", "policy/tool-manifest.json",
    "tools/crypto-aes-siv-witness/fuzz/input_custody.py",
    "crates/time-core/src/projection_enclosure.rs",
)


def module(name, path):
    """Import only coordinator-owned repository code, never payload Python."""
    spec = importlib.util.spec_from_file_location(name, path)
    result = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(result)
    return result


ACQUIRE = module("verus_acquisition", ROOT / "tools/formal/verus_pilot_acquire.py")
CHECK = module("verus_analysis", ROOT / "scripts/check-analysis.py")
require = ACQUIRE.require


def source_hashes():
    """Bind the exact current producer, admission and execution configuration."""
    return {name: CHECK.digest(ROOT / name) for name in SOURCES}


def write(path, value):
    """Create a fresh private receipt without truncating or following an alias."""
    data = (json.dumps(value, indent=2, sort_keys=True, default=str) + "\n").encode()
    descriptor = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_CLOEXEC | os.O_NOFOLLOW, 0o600)
    with os.fdopen(descriptor, "wb") as stream:
        stream.write(data)


def ancestors(folder):
    """Reject another UID's rename authority over the selected private store."""
    result = {}
    for path in (folder, *folder.parents):
        info = path.lstat()
        require(stat.S_ISDIR(info.st_mode) and info.st_uid in (0, os.geteuid())
                and not stat.S_IMODE(info.st_mode) & 0o022, "Untrusted store ancestor")
        result[str(path)] = (info.st_dev, info.st_ino, info.st_mode, info.st_uid, info.st_gid)
    return result


def admit():
    """Fresh complete Verus/SDK/helper admission; never trust version or mtime alone."""
    require(re.fullmatch(r"[0-9a-f]{64}", MANIFEST_SHA256) is not None, "Manifest is not pinned")
    require(STORE == STORE.resolve(strict=True) and SDK == SDK.resolve(strict=True), "Store/SDK redirects")
    before = ancestors(STORE)
    data = CHECK.read_once(STORE / "manifest.json", maximum=1024 * 1024)
    require(hashlib.sha256(data).hexdigest() == MANIFEST_SHA256, "Candidate manifest drift")
    manifest = CHECK.decode(data)
    require(manifest["version"] == ACQUIRE.VERSION and manifest["commit"] == ACQUIRE.COMMIT
            and manifest["archive"] == {"url": ACQUIRE.ARCHIVE_URL, "bytes": ACQUIRE.ARCHIVE_BYTES,
                                        "sha256": ACQUIRE.ARCHIVE_SHA256}, "Candidate provenance differs")
    payload = STORE / "payload"
    require(payload == payload.resolve(strict=True), "Payload redirects")
    require(ACQUIRE.inventory(payload) == manifest["payload"], "Complete Verus payload drift")
    stable = CHECK.admit(ROOT, profile="stable-core")
    require(stable["stable"] == SDK and set(stable["readonly_roots"]) == {
        SDK / name for name in CHECK.STABLE_CORE_SCOPES}, "Unexpected SDK/mount scopes")
    for name, expected in HELPERS.items():
        require(CHECK.digest(Path(name)) == expected, "Public helper differs: " + name)
    require(before == ancestors(STORE) and CHECK.digest(STORE / "manifest.json") == MANIFEST_SHA256,
            "Store authority/manifest changed during admission")
    return stable, payload


def command(payload, source, trace, version=False):
    """Closed literal launch: the SDK loader path is set only inside the namespace.

    The shared runner deliberately rejects generic loader-path injection. This
    reviewed candidate permits exactly the two hash-admitted readonly SDK dirs,
    after bwrap/prlimit start, with no user-controlled flags or inherited values.
    """
    library_path = str(SDK / "lib") + ":" + str(SDK / "lib/rustlib/x86_64-unknown-linux-gnu/lib")
    options = ["--version"] if version else [
        source, "--sysroot=" + str(SDK), "--crate-type=lib", "--num-threads=1",
        "--output-json", "--time", "--error-format=json", "--no-cheating", "--smt-option", "timeout=60000",
    ]
    return ["/usr/bin/strace", "-f", "-e", "trace=execve,execveat", "-s", "4096", "-o", trace,
            "/usr/bin/env", "LD_LIBRARY_PATH=" + library_path, str(payload / "verus"), *options]


def check_trace(path, payload):
    """Reject unexpected successful/attempted executable paths, retain the raw trace."""
    text = CHECK.read_once(path, maximum=2 * 1024**2).decode("utf-8", errors="strict")
    calls = re.findall(r'\bexecve\(("(?:[^"\\]|\\.)*")', text)
    require(len(calls) == len(re.findall(r'\bexecve\(', text)), "Unparsed executable trace call")
    paths = [json.loads(value) for value in calls]
    allowed = {"/usr/bin/env", *(str(payload / name) for name in ACQUIRE.EXECUTABLES)}
    require(paths and all(path in allowed for path in paths) and "execveat(" not in text,
            "Observed child executable closure differs")
    return paths


def static_elf(payload):
    """Inspect ELF metadata with trusted readelf, never ldd or a payload entrypoint."""
    records = []
    for name in sorted(ACQUIRE.EXECUTABLES):
        argv = ["/usr/bin/x86_64-linux-gnu-readelf", "-lWd", str(payload / name)]
        result = CHECK.RUNTIME.ENV.run(argv, cwd=ROOT, timeout=20,
                                       environment={"PATH": "/usr/bin:/bin", "LANG": "C.UTF-8"})
        CHECK.require_result(result)
        text = result["output"]
        require("/lib64/ld-linux-x86-64.so.2" in text, "Unexpected ELF interpreter")
        paths = re.findall(r'\((?:RUNPATH|RPATH)\).*?\[([^\]]*)\]', text)
        require(all((part == "$ORIGIN" or part.startswith("$ORIGIN/"))
                    and (payload / part.replace("$ORIGIN", ".", 1)).resolve().is_relative_to(payload)
                    for path in paths for part in path.split(":")), "Unexpected packaged ELF runtime path")
        needed = re.findall(r'\(NEEDED\).*?\[([^\]]*)\]', text)
        system = {"libc.so.6", "libm.so.6", "libgcc_s.so.1", "libstdc++.so.6",
                  "libdl.so.2", "libpthread.so.0", "librt.so.1", "ld-linux-x86-64.so.2"}
        sdk_libraries = {p.name for directory in (SDK / "lib", SDK / "lib/rustlib/x86_64-unknown-linux-gnu/lib")
                         for p in directory.iterdir() if p.is_file()}
        require(needed and all(name in system or name in sdk_libraries for name in needed),
                "Unexpected direct ELF library dependency")
        records.append(result)
    return records


def verification(native, fault, source, line, *, combined=False):
    """Require whole-crate results, both observed functions and the intended fault.

    Pinned upstream main.rs emits the aggregate object after JSON diagnostics.
    Counts alone do not establish function inventory or failure meaning.
    """
    require(not combined or fault, "Combined control must include the intended fault")
    require(native["failure"] is None and native["exit_code"] == (1 if fault else 0),
            "Compatibility command failed or lost custody")
    decoder = json.JSONDecoder(object_pairs_hook=CHECK.unique_object)
    output = native["output"]
    objects = []
    position = 0
    while position < len(output):
        opening = output.find("{", position)
        if opening < 0:
            break
        try:
            item, length = decoder.raw_decode(output, opening)
        except json.JSONDecodeError:
            position = opening + 1
            continue
        if isinstance(item, dict):
            objects.append(item)
        position = length
    reports = [item for item in objects if "verification-results" in item]
    require(len(reports) == 1, "Missing/ambiguous Verus aggregate report")
    report = reports[0]
    expected = {"encountered-error": fault, "encountered-vir-error": False,
                "success": not fault, "verified": 2 if combined else (1 if fault else 2),
                "errors": 1 if fault else 0, "is-verifying-entire-crate": True}
    results = report["verification-results"]
    require(all(type(results.get(key)) is type(value) and results[key] == value
                for key, value in expected.items()), "Incomplete or unexpected whole-crate results")
    functions = {}
    for group in report["times-ms"]["smt"]["smt-run-module-times"]:
        for item in group["function-breakdown"]:
            require(type(item["function"]) is str and type(item["success"]) is bool,
                    "Malformed per-function verification status")
            name = item["function"].rsplit("::", 1)[-1]
            require(name not in functions, "Duplicate function timing/status")
            functions[name] = item["success"]
    expected_functions = {"propagated_failure": True, "compatibility_healthy": combined or not fault}
    if combined:
        expected_functions["compatibility_fault"] = False
    require(functions == expected_functions,
            "Expected verified function inventory differs")
    errors = [item for item in objects if item.get("$message_type") == "diagnostic"
              and item.get("level") == "error"]
    primary = [item for item in errors if any(span.get("is_primary") for span in item.get("spans", []))]
    if fault:
        require(len(primary) == 1 and "assertion failed" in primary[0].get("message", "").lower(),
                "Seeded failure is not the intended assertion")
        spans = [span for span in primary[0]["spans"] if span.get("is_primary")]
        require(len(spans) == 1 and spans[0].get("file_name") == source
                and spans[0].get("line_start") == line, "Seeded failure primary span differs")
        require(all("assertion failed" in item.get("message", "").lower()
                    or item.get("message", "").lower().startswith("aborting due to") for item in errors),
                "Seeded assertion has a competing compiler/verifier error")
    else:
        require(not errors, "Healthy proof has compiler/verifier errors")
    return {"verification_results": results, "functions": functions, "qualified": True}


def run():
    """Version, healthy APIs, seeded assertion: stop on the first failed gate."""
    deadline = time.monotonic() + 3600
    before_sources = source_hashes()
    store_identity = ancestors(STORE)
    # A fresh output is the attempt lock. Never restart a terminal/partial trial.
    work = STORE / "work"
    work.mkdir(mode=0o700)
    summary = {"status": "DEFER", "product_proofs": 0, "adopted": False, "jobs": [],
               "source_sha256": before_sources, "max_preparation_commands": 3,
               "started_unix_seconds": time.time(), "integration_limit_seconds": 3600,
               "proof_commands_executed": 0, "retries": 0}
    try:
        stable, payload = admit()
        summary["elf_metadata"] = static_elf(payload)
        bootstrap_started = json.loads((STORE / "manifest.json").read_text())["bootstrap_started_unix_seconds"]
        require(0 <= time.time() - bootstrap_started < 600, "Bootstrap wall budget exhausted before compatibility")
        probe = (ROOT / "tools/formal/verus_compatibility.rs").read_bytes()
        require(probe.count(b"assert(shifted == 6);") == 1, "Sensitivity control source differs")
        completed = []
        for index, case in enumerate(("version", "healthy", "assertion_fault"), 1):
            require(time.monotonic() < deadline, "Finite integration clock exhausted")
            ACQUIRE.capacity(STORE, deadline)
            require(source_hashes() == before_sources and ancestors(STORE) == store_identity, "Coordinator source/authority drift")
            started = time.monotonic()
            stable, payload = admit()
            folder = work / "jobs" / f"{index:02d}-{case}"
            inputs = folder / "inputs"
            inputs.mkdir(parents=True, mode=0o700)
            data = probe.replace(b"assert(shifted == 6);", b"assert(shifted == 5);") if case == "assertion_fault" else probe
            (inputs / "proof.rs").write_bytes(data)
            os.chmod(inputs / "proof.rs", 0o400)
            relative = "/work/" + folder.relative_to(work).as_posix()
            argv = command(payload, relative + "/inputs/proof.rs", relative + "/exec.trace", case == "version")
            record = {"index": index, "case": case, "command": argv, "qualified": False,
                      "admission_seconds": time.monotonic() - started,
                      "input_sha256": hashlib.sha256(data).hexdigest()}
            # Retain complete source/helper/SDK identities; no full SDK is copied.
            write(folder / "admission.json", stable)
            sealed = [*completed, inputs, folder / "admission.json"]
            before = CHECK.input_snapshot(work, sealed)
            summary["jobs"].append(record)
            tick = time.monotonic()
            try:
                native = CHECK.RUNTIME.run(ROOT, work, argv, readonly=[*stable["readonly_roots"], payload],
                    sealed=sealed, cwd=inputs, environment={"VERUS_USE_RUSTUP": "0", "VERUS_ROOT": str(payload),
                        "VERUS_Z3_PATH": str(payload / "z3")}, timeout=min(180, deadline-time.monotonic()),
                    cpu_seconds=180, address_space_bytes=8 * 1024**3, build_jobs=1, process_limit=128)
                record.update(native=native, execute_seconds=time.monotonic()-tick)
                CHECK.RUNTIME._owned_work(work, ROOT)
                record["observed_executables"] = check_trace(folder / "exec.trace", payload)
                require(before == CHECK.input_snapshot(work, sealed), "Sealed compatibility source/receipt drift")
                admit()
                require(before_sources == source_hashes() and ancestors(STORE) == store_identity, "Post-execution source/authority drift")
                ACQUIRE.capacity(STORE, deadline)
                require(native["failure"] is None, "Native custody/resource failure")
                if case == "version":
                    require(native["exit_code"] == 0 and ACQUIRE.VERSION in native["output"], "Version/loader gate failed")
                    require(time.time() - bootstrap_started < 600, "Bootstrap wall budget exhausted")
                    record["qualified"] = True
                else:
                    line = data.split(b"\n").index(b"    assert(shifted == 5);") + 1 if case == "assertion_fault" else 0
                    record.update(verification(native, case == "assertion_fault", relative + "/inputs/proof.rs", line))
            except Exception as error:
                record["error"] = type(error).__name__ + ": " + str(error)
                raise
            finally:
                write(folder / "execution.json", record)
            completed.append(folder)
        summary["status"] = "compatibility-controls-passed-not-product-proof"
        summary["remaining_scope"] = "Actual new/compare/complement/add_fractions proof and product fault controls are unexecuted"
    except Exception as error:
        summary["error"] = type(error).__name__ + ": " + str(error)
    finally:
        summary["elapsed_seconds"] = 3600 - max(0, deadline-time.monotonic())
        write(STORE / "compatibility.json", summary)
    print(json.dumps({"status": summary["status"], "jobs": len(summary["jobs"]),
                      "product_proofs": 0, "error": summary.get("error"), "receipt": str(STORE / "compatibility.json")}))
    return 0 if summary["status"] != "DEFER" else 1


def complete_preparation():
    """Use only slot three after the retained, source-identified E0308 harness error.

    This explicit causal repair carries the original clock and immutable prefix;
    it cannot restart the controller, add a fourth command or adopt Verus.
    """
    previous_hash = "876b7053c2f73e89afd76aecc93f3bcfb8765abf10ae9210e376a0f52b6204b8"
    previous_bytes = CHECK.read_once(STORE / "compatibility.json", maximum=2 * 1024**2)
    require(hashlib.sha256(previous_bytes).hexdigest() == previous_hash, "Unexpected preparation prefix")
    previous = CHECK.decode(previous_bytes)
    require(len(previous["jobs"]) == 2 and previous["jobs"][0]["case"] == "version"
            and previous["jobs"][0]["qualified"] is True
            and previous["jobs"][1]["case"] == "healthy"
            and previous["jobs"][1]["native"]["exit_code"] == 1
            and previous["jobs"][1]["native"]["failure"] is None
            and '"E0308"' in previous["jobs"][1]["native"]["output"], "Prefix is not the reviewed harness failure")
    remaining = previous["started_unix_seconds"] + 3600 - time.time()
    require(0 < remaining <= 3600, "Original integration clock exhausted")
    deadline = time.monotonic() + remaining
    sources = source_hashes()
    changed = {key for key, value in sources.items() if previous["source_sha256"][key] != value}
    require(changed == {"tools/formal/verus_pilot.py", "tools/formal/verus_compatibility.rs"},
            "Causal repair changed an unrelated source/tool input")
    frozen_previous = STORE / "preparation-sources-v1"
    require(all(ACQUIRE.CUSTODY.digest(frozen_previous / name, maximum=1024**2) == checksum
                for name, checksum in previous["source_sha256"].items()), "Original execution source custody differs")
    work = STORE / "work"
    CHECK.RUNTIME._owned_work(work, ROOT)
    folder = work / "jobs/03-healthy-and-assertion-fault"
    folder.mkdir(mode=0o700)  # Fresh attempt lock: no second completion/retry.
    record = {"index": 3, "case": "healthy_and_assertion_fault", "qualified": False}
    summary = {"status": "DEFER", "adopted": False, "product_proofs": 0,
               "previous_receipt_sha256": previous_hash, "source_sha256": sources,
               "original_started_unix_seconds": previous["started_unix_seconds"],
               "harness_repairs": 1, "unchanged_input_retries": 0,
               "max_preparation_commands": 3, "proof_commands_executed": 0,
               "jobs": [*previous["jobs"], record]}
    authority = ancestors(STORE)
    try:
        ACQUIRE.capacity(STORE, deadline)
        tick = time.monotonic()
        stable, payload = admit()
        inputs = folder / "inputs"
        inputs.mkdir(mode=0o700)
        data = ACQUIRE.CUSTODY.read_file(ROOT / "tools/formal/verus_compatibility.rs", maximum=64*1024)
        require(data.count(b"fn compatibility_fault()") == 1
                and data.count(b"compatibility_fault") == 1
                and data.count(b"    assert(false);") == 1, "Separate uncalled detector control differs")
        (inputs / "proof.rs").write_bytes(data)
        os.chmod(inputs / "proof.rs", 0o400)
        line = data.split(b"\n").index(b"    assert(false);") + 1
        relative = "/work/" + folder.relative_to(work).as_posix()
        argv = command(payload, relative + "/inputs/proof.rs", relative + "/exec.trace")
        record.update(command=argv, input_sha256=hashlib.sha256(data).hexdigest())
        write(folder / "admission.json", stable)
        sealed = [work / "jobs/01-version", work / "jobs/02-healthy", inputs, folder / "admission.json"]
        before = CHECK.input_snapshot(work, sealed)
        record["admission_seconds"] = time.monotonic()-tick
        tick = time.monotonic()
        native = CHECK.RUNTIME.run(ROOT, work, argv, readonly=[*stable["readonly_roots"], payload],
            sealed=sealed, cwd=inputs, environment={"VERUS_USE_RUSTUP": "0", "VERUS_ROOT": str(payload),
                "VERUS_Z3_PATH": str(payload / "z3")}, timeout=min(180, deadline-time.monotonic()),
            cpu_seconds=180, address_space_bytes=8*1024**3, build_jobs=1, process_limit=128)
        record.update(native=native, execute_seconds=time.monotonic()-tick)
        CHECK.RUNTIME._owned_work(work, ROOT)
        record["observed_executables"] = check_trace(folder / "exec.trace", payload)
        require(before == CHECK.input_snapshot(work, sealed), "Current/prior source or receipt custody drift")
        admit()
        require(sources == source_hashes() and authority == ancestors(STORE)
                and CHECK.digest(STORE / "compatibility.json") == previous_hash,
                "Post-execution source/store/prefix drift")
        ACQUIRE.capacity(STORE, deadline)
        record.update(verification(native, True, relative + "/inputs/proof.rs", line, combined=True))
        summary["status"] = "compatibility-controls-passed-not-product-proof"
    except Exception as error:
        record["error"] = type(error).__name__ + ": " + str(error)
        summary["error"] = record["error"]
    finally:
        write(folder / "execution.json", record)
        summary["elapsed_from_original_start_seconds"] = time.time()-previous["started_unix_seconds"]
        write(STORE / "compatibility-final.json", summary)
    print(json.dumps({"status": summary["status"], "preparation_commands": 3,
                      "product_proofs": 0, "error": summary.get("error"),
                      "receipt": str(STORE / "compatibility-final.json")}))
    return 0 if record["qualified"] else 1


if __name__ == "__main__":
    require(sys.argv[1:] in ([], ["--complete-preparation"]),
            "This pilot accepts no tool, command or environment overrides")
    sys.exit(complete_preparation() if sys.argv[1:] else run())
