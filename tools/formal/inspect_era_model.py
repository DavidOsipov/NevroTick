#!/usr/bin/env python3
"""Dump one pinned healthy EraWindow model to inspection material, never a proof."""
from __future__ import annotations

import argparse
import ast
import codecs
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import stat
import sys
import time


ROOT = Path(__file__).resolve().parents[2]


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    value = importlib.util.module_from_spec(spec)
    sys.modules[name] = value
    spec.loader.exec_module(value)
    return value


CODEGEN = load("era_model_codegen_helpers", ROOT / "tools/formal/verify_era_codegen.py")
FORMAL = CODEGEN.FORMAL
Incomplete = FORMAL.Incomplete
write = FORMAL.write_json
CONSUMER = "era-constructor-model-inspection-v1"
SOURCE_ROOT = Path("/home/david_osipov/.local/share/nevrotick/formal-era-constructor-codegen-20261105-v3")
SOURCE_PINS = {
    "summary.json": "e812649c46ffbb5c1817886f805647da69201316130cdbadaa0e4d581cd7b62c",
    "work/receipts/preparation.json": "d8bd97b8c78578b1a4d69507e8e461f11dbe301a8926e76a507d8a65a57a5506",
    "work/receipts/artifacts.json": "bf54a79cf5fed2ab8b09ca132c50f9a1278be2ce81642c008a4d85236bbe6932",
    "work/final-admission.json": "32c01edf36c52877df75be33e7460573313d2031eee7bb476caeb73237c82689",
}
HELPER_OLD_SHA256 = "928e3e599e649218385a68ffd52eb6f4d43e8e6777a7c4f7698e720170b2be22"
HELPER_NEW_SHA256 = "73c98136a9474238e63f0641f6516424a259f9993c24072cefd23a25190a759a"
INSPECTOR_SHA256 = "3b4ac9c5d41f72db7c455d45f9f7de5dc7240d3b020a1c68a6b210ad435d4c0c"
INSPECTOR_BYTES = 15_559_744
MAX_CONTEXT = 4 * 1024 ** 2
MAX_MODEL = MAX_C_DUMP = 256 * 1024 ** 2
POINTER = Path.home() / ".local/share/nevrotick/analysis-installation.json"
CLI_SOURCE = "https://github.com/diffblue/cbmc/blob/820ff0f555b43fb78e0cd9332e498461bd14244b/src/goto-instrument/goto_instrument_parse_options.cpp#L727-L792"


def input_names() -> list[str]:
    vendor = load("era_model_vendor_source_closure", ROOT / "tools/formal/era_vendor_admission.py")
    return sorted({*CODEGEN.input_names(vendor), "tools/formal/inspect_era_model.py",
                   "tools/formal/test_inspect_era_model.py"})


def regular(path: Path, maximum: int):
    if not path.is_absolute() or path != path.resolve(strict=True):
        raise Incomplete("Inspection path redirects")
    value = path.lstat()
    if (not stat.S_ISREG(value.st_mode) or value.st_uid != os.geteuid() or value.st_nlink != 1
            or not 0 < value.st_size <= maximum):
        raise Incomplete("Inspection inputs/output must be bounded owned nonempty regular single-link files")
    return value


def file_identity(value):
    return (value.st_dev, value.st_ino, value.st_size, value.st_mtime_ns, value.st_ctime_ns,
            value.st_mode, value.st_uid, value.st_nlink)


def read(check, path: Path, maximum=MAX_CONTEXT) -> bytes:
    before = regular(path, maximum)
    data = check.read_once(path, maximum=maximum)
    if file_identity(regular(path, maximum)) != file_identity(before):
        raise Incomplete("Inspection source changed while reading")
    return data


def copy_file(source: Path, target: Path, expected: dict, maximum: int) -> None:
    """Copy one verified inode to a new private file, without aliases or truncation."""
    before = regular(source, maximum)
    if target.parent != target.parent.resolve(strict=True) or before.st_size != expected["bytes"]:
        raise Incomplete("Inspection copy path/size differs")
    incoming = os.open(source, os.O_RDONLY | os.O_CLOEXEC | os.O_NOFOLLOW | os.O_NONBLOCK)
    outgoing = None
    try:
        if file_identity(os.fstat(incoming)) != file_identity(before):
            raise Incomplete("Inspection source inode changed before copy")
        outgoing = os.open(target, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW | os.O_CLOEXEC, 0o600)
        checksum = hashlib.sha256()
        count = 0
        while chunk := os.read(incoming, 1024 ** 2):
            count += len(chunk)
            if count > maximum or count > expected["bytes"]:
                raise Incomplete("Inspection copy exceeded its declared byte bound")
            checksum.update(chunk)
            view = memoryview(chunk)
            while view:
                written = os.write(outgoing, view)
                if written <= 0:
                    raise Incomplete("Inspection copy made no progress")
                view = view[written:]
        if (count != expected["bytes"] or checksum.hexdigest() != expected["sha256"]
                or file_identity(os.fstat(incoming)) != file_identity(before)
                or file_identity(regular(source, maximum)) != file_identity(before)):
            raise Incomplete("Inspection source bytes/identity changed during copy")
        os.fchmod(outgoing, 0o400)
    finally:
        os.close(incoming)
        if outgoing is not None:
            os.close(outgoing)


def core_tool_identity(admitted: dict) -> dict:
    value = FORMAL.serializable(CODEGEN.tool_identity(admitted))
    value.pop("admission_consumer")
    return value


def prepare_provenance(check, island: Path) -> dict:
    """Admit precisely V3's pinned receipts/source and one healthy linked model."""
    if SOURCE_ROOT != SOURCE_ROOT.resolve(strict=True):
        raise Incomplete("Pinned V3 source root redirects")
    context = island / "context"
    context.mkdir(mode=0o700)
    values, paths, expected_snapshot = {}, [], {}

    def remember(path, facts):
        expected_snapshot[path.relative_to(SOURCE_ROOT).as_posix()] = {
            "kind": "file", "bytes": facts["bytes"], "sha256": facts["sha256"]}
    for name, checksum in SOURCE_PINS.items():
        source = SOURCE_ROOT / name
        data = read(check, source)
        if FORMAL.digest(data) != checksum:
            raise Incomplete("Pinned V3 provenance receipt changed: " + name)
        values[name] = check.decode(data)
        paths.append(source)
        facts = {"bytes": len(data), "sha256": checksum}
        remember(source, facts)
        copy_file(source, context / name.replace("/", "__"), facts, MAX_CONTEXT)
    summary, preparation, artifacts, admission = (values[name] for name in SOURCE_PINS)
    if (summary.get("status") != "codegen-preflight-passed"
            or summary.get("consumer") != "era-constructor-codegen-v1"
            or summary.get("all_retained_inputs_and_receipts_unchanged") is not True
            or summary.get("codegen_process_succeeded") is not True
            or summary.get("proof_qualified") is not False
            or artifacts.get("artifacts") != summary.get("artifacts")
            or artifacts.get("constructor_metadata") != summary.get("constructor_metadata")
            or admission.get("consumer") != "era-constructor-codegen-v1"
            or admission.get("admitted", {}).get("admission_consumer") != "era-constructor-codegen-v1"):
        raise Incomplete("Pinned V3 provenance does not establish the fixed codegen candidate")
    vendor = load("era_model_provenance_vendor", ROOT / "tools/formal/era_vendor_admission.py")
    old_names = CODEGEN.input_names(vendor)
    manifest = preparation["source_manifest"]
    if sorted(manifest) != old_names:
        raise Incomplete("Pinned V3 source manifest differs from the complete old closure")
    for name, expected in manifest.items():
        source = SOURCE_ROOT / "source-island/frozen" / name
        old = read(check, source)
        current = read(check, ROOT / name)
        paths.append(source)
        if {"bytes": len(old), "sha256": FORMAL.digest(old)} != expected:
            raise Incomplete("V3 frozen source bytes changed: " + name)
        remember(source, expected)
        if name == "scripts/check-analysis.py":
            if FORMAL.digest(old) != HELPER_OLD_SHA256 or FORMAL.digest(current) != HELPER_NEW_SHA256:
                raise Incomplete("The approved additive inspection API bridge changed")
            old_tree, tree = ast.parse(old), ast.parse(current)
            additions = [node for node in tree.body if isinstance(node, ast.FunctionDef)
                         and node.name == "admit_era_constructor_model_inspection"]
            if len(additions) != 1:
                raise Incomplete("Inspection API bridge is not one added function")
            tree.body.remove(additions[0])
            if ast.dump(tree, include_attributes=False) != ast.dump(old_tree, include_attributes=False):
                raise Incomplete("Inspection API bridge changes existing source semantics")
            copy_file(source, context / "old-check-analysis.py", expected, MAX_CONTEXT)
        elif current != old:
            raise Incomplete("Current source differs from V3 beyond the approved API bridge: " + name)
    project = SOURCE_ROOT / "source-island/project"
    generated = dict(preparation["preparation"]["generated_files_sha256"])
    generated["correspondence.json"] = preparation["preparation"]["correspondence_sha256"]
    for name, checksum in generated.items():
        source = project / name
        data = read(check, source)
        if FORMAL.digest(data) != checksum:
            raise Incomplete("V3 generated Cargo/source context changed")
        paths.append(source)
        facts = {"bytes": len(data), "sha256": checksum}
        remember(source, facts)
        copy_file(source, context / ("project__" + name.replace("/", "__")),
                  facts, MAX_CONTEXT)
    harnesses = summary["constructor_metadata"]["proof_harnesses"]
    chosen = [item for item in harnesses if item.get("pretty_name") == "proof::era_constructor_full_domain"]
    if len(chosen) != 1:
        raise Incomplete("Pinned V3 does not identify exactly one healthy constructor harness")
    model = Path(chosen[0]["goto_file"])
    if not model.is_absolute() or not model.is_relative_to("/work/target") or not str(model).endswith(".symtab.out"):
        raise Incomplete("Healthy harness model is outside its declared target")
    relative = model.relative_to("/work").as_posix().removesuffix(".symtab.out") + ".out"
    expected = summary["artifacts"].get(relative)
    if type(expected) is not dict or artifacts["artifacts"].get(relative) != expected:
        raise Incomplete("Healthy linked model lacks pinned artifact facts")
    source = SOURCE_ROOT / "work" / relative
    paths.append(source)
    remember(source, expected)
    # Every fact was anchored when validated, before subsequent copies or
    # source reads. A late snapshot must match those originals, never rebase it.
    if check.input_snapshot(SOURCE_ROOT, paths) != expected_snapshot:
        raise Incomplete("V3 source/provenance changed after validation")
    copy_file(source, island / "model.out", expected, MAX_MODEL)
    if check.input_snapshot(SOURCE_ROOT, paths) != expected_snapshot:
        raise Incomplete("V3 source/model changed during preparation")
    return {"source_root": SOURCE_ROOT, "source_receipt_pins": SOURCE_PINS,
            "source_paths": paths, "source_snapshot": expected_snapshot, "selected_harness": chosen[0],
            "linked_model_relative": relative, "model": expected,
            "old_core_tool_identity": core_tool_identity(admission["admitted"]),
            "helper_bridge": {"old_sha256": HELPER_OLD_SHA256, "new_sha256": HELPER_NEW_SHA256,
                              "change": "only the added inspection admission function"}}


def audit_provenance(check, value: dict) -> None:
    for path in value["source_paths"]:
        regular(path, MAX_MODEL if path == SOURCE_ROOT / "work" / value["linked_model_relative"] else MAX_CONTEXT)
    if check.input_snapshot(SOURCE_ROOT, value["source_paths"]) != value["source_snapshot"]:
        raise Incomplete("Original V3 source, provenance receipt or model changed")


def admit_tools(check, frozen: Path, output: Path, provenance: dict) -> dict:
    started = time.monotonic()
    pointer = read(check, POINTER, 4096)
    admitted = check.admit_era_constructor_model_inspection(frozen)
    if admitted.get("admission_consumer") != CONSUMER:
        raise Incomplete("Inspection tool admission returned a different consumer")
    payload, _ = FORMAL.proof_readonly_roots(admitted, "kani-proof")
    if (read(check, POINTER, 4096) != pointer
            or admitted.get("installation_pointer_sha256") != FORMAL.digest(pointer)
            or core_tool_identity(admitted) != provenance["old_core_tool_identity"]):
        raise Incomplete("Inspection tool/pointer identity differs from qualified V3")
    executable = payload / "bin/goto-instrument"
    if executable.stat().st_size != INSPECTOR_BYTES or check.digest(executable) != INSPECTOR_SHA256:
        raise Incomplete("Inspection executable differs from pinned CBMC 6.11 payload")
    value = {"admitted": admitted, "consumer": CONSUMER, "executable": executable,
             "executable_sha256": INSPECTOR_SHA256, "admission_seconds": time.monotonic() - started,
             "cli_source": CLI_SOURCE, "tool_profile": "one fixed top-level C dump; no solver or proof flags",
             "host_runtime_trust": "Existing trusted Python/stdlib, OS and public GCC/runtime; helper hashes do not inventory all public tools/libraries."}
    write(output, value)
    return value


def inspect_dump(path: Path) -> dict:
    before = regular(path, MAX_C_DUMP)
    descriptor = os.open(path, os.O_RDONLY | os.O_CLOEXEC | os.O_NOFOLLOW | os.O_NONBLOCK)
    try:
        if file_identity(os.fstat(descriptor)) != file_identity(before):
            raise Incomplete("C dump inode changed before inspection")
        checksum, decoder, count = hashlib.sha256(), codecs.getincrementaldecoder("utf-8")("strict"), 0
        nonblank = False
        while chunk := os.read(descriptor, 1024 ** 2):
            count += len(chunk)
            if count > MAX_C_DUMP:
                raise Incomplete("C dump exceeds the fixed output byte cap")
            checksum.update(chunk)
            text = decoder.decode(chunk)
            if "\0" in text:
                raise Incomplete("C dump contains a NUL byte")
            nonblank |= bool(text.strip())
        nonblank |= bool(decoder.decode(b"", final=True).strip())
        if (not nonblank or count != before.st_size or file_identity(os.fstat(descriptor)) != file_identity(before)
                or file_identity(regular(path, MAX_C_DUMP)) != file_identity(before)):
            raise Incomplete("C dump is empty or changed during inspection")
        return {"bytes": count, "sha256": checksum.hexdigest(), "encoding": "strict UTF-8",
                "c_dump_generated": True, "loop_analysis_complete": False,
                "proof_qualified": False, "unwind_bound": None}
    finally:
        os.close(descriptor)


def run(output: Path) -> dict:
    try:
        island = output / "inputs"
        island.mkdir(mode=0o700)
        check = load("era_inspection_bootstrap", ROOT / "scripts/check-analysis.py")
        names = input_names()
        records = FORMAL.freeze_inputs(check, island / "frozen", names)
        consumer = load("era_inspection_frozen_dispatch", island / "frozen/tools/formal/inspect_era_model.py")
    except Exception as error:
        value = {"status": "incomplete", "native_top_level_attempts": 0, "native_top_level_returned": 0,
                 "c_dump_generated": False, "loop_analysis_complete": False, "proof_qualified": False,
                 "unwind_bound": None, "error": f"{type(error).__name__}: {error}"}
        write(output / "summary.json", value)
        return value
    return consumer.run_prepared(output, original_root=ROOT, records=records, names=names)


def run_prepared(output: Path, *, original_root: Path, records: dict, names: list[str]) -> dict:
    started = time.monotonic()
    island, work, frozen = output / "inputs", output / "work", ROOT
    work.mkdir(mode=0o700)
    receipts = work / "receipts"
    receipts.mkdir(mode=0o700)
    value = {"status": "incomplete", "consumer": CONSUMER, "native_top_level_attempts": 0,
             "native_top_level_returned": 0, "maximum_native_commands": 1,
             "c_dump_generated": False, "loop_analysis_complete": False, "proof_qualified": False,
             "unwind_bound": None, "original_root": original_root, "frozen_root": frozen}
    check = provenance = before_tools = None
    retained = []
    try:
        if (frozen != island / "frozen" or original_root == frozen
                or Path(FORMAL.__file__) != frozen / "tools/formal/verify.py"
                or Path(CODEGEN.__file__) != frozen / "tools/formal/verify_era_codegen.py"):
            raise Incomplete("Inspection requires actual frozen consumer/helpers")
        check = load("era_inspection_analysis", frozen / "scripts/check-analysis.py")
        if (Path(check.__file__) != frozen / "scripts/check-analysis.py"
                or Path(check.RUNTIME.__file__) != frozen / "scripts/analysis-runtime.py" or input_names() != names):
            raise Incomplete("Inspection analysis/runtime/closure differs from frozen inputs")
        CODEGEN.require_sources(check, frozen, records, names, original_root=original_root)
        provenance = prepare_provenance(check, island)
        write(island / "provenance.json", provenance)
        for path in sorted(island.rglob("*"), reverse=True):
            path.chmod(0o500 if path.is_dir() else 0o400)
        island.chmod(0o500)
        FORMAL.retain_artifacts(check, output, retained, [island])
        before_tools = admit_tools(check, frozen, receipts / "pre-admission.json", provenance)
        FORMAL.retain_artifacts(check, output, retained, [receipts / "pre-admission.json"])
        payload, compiler = FORMAL.proof_readonly_roots(before_tools["admitted"], "kani-proof")
        readonly = (payload, compiler, island)
        for path in readonly:
            CODEGEN.readonly_directory(check, path, work)
        if len(set(readonly)) != 3 or any(a != b and (a.is_relative_to(b) or b.is_relative_to(a)) for a in readonly for b in readonly):
            raise Incomplete("Inspection requires exactly three disjoint readonly roots")
        command = [str(before_tools["executable"]), "--dump-c", str(island / "model.out"), "/work/era-constructor.c"]
        value.update(command=command, readonly_roots=readonly, native_top_level_attempts=1)
        write(receipts / "execution.json", value)
        receipt_before = check.input_snapshot(output, [receipts])
        audit_provenance(check, provenance)
        FORMAL.audit_retained(check, output, retained)
        CODEGEN.require_sources(check, frozen, records, names, original_root=original_root)
        execute_started = time.monotonic()
        try:
            native = check.RUNTIME.run(frozen, work, command, readonly=readonly,
                path_dirs=[payload / "bin", compiler / "bin"], cwd=work, sealed=[receipts],
                environment={}, timeout=180, cpu_seconds=180, address_space_bytes=8 * 1024 ** 3,
                process_limit=128, build_jobs=2)
        finally:
            value["execute_seconds"] = time.monotonic() - execute_started
        value.update(native=native, native_top_level_returned=1)
        check.RUNTIME._owned_work(work, original_root)
        if check.input_snapshot(output, [receipts]) != receipt_before:
            raise Incomplete("Inspection pre-command receipts changed")
        FORMAL.audit_retained(check, output, retained)
        audit_provenance(check, provenance)
        completed = (type(native) is dict and {"exit_code", "failure", "output"}.issubset(native)
                     and type(native["exit_code"]) is int and type(native["output"]) is str
                     and native["exit_code"] == 0 and native["failure"] is None)
        dump_error = None
        if completed:
            try:
                dump = inspect_dump(work / "era-constructor.c")
                write(receipts / "dump.json", dump)
                FORMAL.retain_artifacts(check, output, retained, [work / "era-constructor.c", receipts / "dump.json"])
            except Exception as error:
                dump_error = error
        after = admit_tools(check, frozen, receipts / "post-admission.json", provenance)
        FORMAL.retain_artifacts(check, output, retained, [receipts / "post-admission.json"])
        if core_tool_identity(after["admitted"]) != core_tool_identity(before_tools["admitted"]):
            raise Incomplete("Inspection tool identity changed after execution")
        if not completed:
            raise Incomplete("Model inspection command did not complete successfully")
        if dump_error is not None:
            raise dump_error
        value.update(dump, status="c-dump-generated")
    except Exception as error:
        value["error"] = f"{type(error).__name__}: {error}"
    finally:
        if check is not None and provenance is not None:
            try:
                write(receipts / "execution.json", value)
                execution_snapshot = check.input_snapshot(output, [receipts / "execution.json"])
                admit_tools(check, frozen, work / "final-admission.json", provenance)
                check.RUNTIME._owned_work(work, original_root)
                CODEGEN.require_sources(check, frozen, records, names, original_root=original_root)
                audit_provenance(check, provenance)
                FORMAL.audit_retained(check, output, retained)
                if check.input_snapshot(output, [receipts / "execution.json"]) != execution_snapshot:
                    raise Incomplete("Final admission changed the inspection execution receipt")
                if value["status"] == "c-dump-generated":
                    value["all_inputs_model_and_receipts_unchanged"] = True
            except Exception as error:
                value.update(status="incomplete", final_error=f"{type(error).__name__}: {error}")
        value["total_seconds"] = time.monotonic() - started
        write(output / "summary.json", value)
    return value


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", required=True, help="fresh private canonical directory outside the checkout")
    args = parser.parse_args(argv)
    requested = Path(args.output_dir)
    if requested.is_relative_to(SOURCE_ROOT) or SOURCE_ROOT.is_relative_to(requested):
        parser.error("Inspection output must be separate from the immutable V3 source receipt/model")
    os.umask(0o077)
    value = run(FORMAL.fresh_output(args.output_dir))
    print(json.dumps({key: value[key] for key in ("status", "native_top_level_attempts", "native_top_level_returned", "proof_qualified")}))
    return 0 if value["status"] == "c-dump-generated" else 1


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, ValueError, Incomplete) as error:
        print(f"Era constructor model inspection incomplete: {error}", file=sys.stderr)
        raise SystemExit(1)
