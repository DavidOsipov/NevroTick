"""Host-only controls for native-test admission; never launch Windows or Cargo."""
from __future__ import annotations

import contextlib
import difflib
import hashlib
import importlib.util
import io
import json
import os
from pathlib import Path
import re
import tempfile
import unittest
from unittest.mock import patch
import uuid

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("engine_native_under_test", ROOT / "scripts/verify-engine-native.py")
NATIVE = importlib.util.module_from_spec(SPEC)
# Imported helpers declare unused home-relative locator defaults. Resolve those
# only during this test import to a synthetic location; keep HOME absent and
# expose no real home/passwd files in the isolated runner-control command.
with patch.object(Path, "home", return_value=Path("/tmp/nevrotick-native-control-home")):
    SPEC.loader.exec_module(NATIVE)
HASH = "a" * 64
REQUEST = {"image_sha256": HASH, "child_sha256": HASH, "gate": "good", "mode": "control-healthy",
           "image_path": "C:\\fixture\\NevroTick-" + "a" * 32 + ".exe"}
CONSOLE = "C:\\Windows\\System32\\conhost.exe"
FACADE_ROWS = {
    "facade-arguments": "scope=engine-ke-prototype eligible=0 fresh_sync=0 stage=admission cause=arguments "
                        "cleanup_stage=none cleanup_cause=none admission_stage=none admission_cause=none\n",
    "facade-entered-runtime": "scope=engine-ke-prototype eligible=0 fresh_sync=0 stage=caller-context "
                              "cause=entered-runtime retirement_confirmed=0\n",
}


def member(pid, role, basename):
    return {"pid": pid, "image_role": role, "basename": basename,
            "system_console": role == "system-console", "read_failure_type": None}


def pid_event(kind, pid):
    return {"kind": kind, "pid": pid, "basename": None, "image_role": None,
            "system_console": False, "read_failure_type": None}


def observation(authored=2, helpers=1):
    """Synthetic intended facts, not a claimed Windows process observation."""
    identities = [member(12, "launcher", "NevroTick-" + "a" * 32 + ".exe")]
    if authored == 2:
        identities.append(member(13, "child", "gate-control.exe"))
    identities += [member(14 + index, "system-console", "conhost.exe") for index in range(helpers)]
    before = [dict(identities[0])] + ([dict(identities[-1])] if helpers else [])
    events = [dict(item, kind=6) for item in identities]
    events += [pid_event(7, item["pid"]) for item in identities] + [pid_event(4, 0)]
    token = {"user_sid": "S-1-5-21-1", "elevated": False, "administrator": False, "elevation_type": 3}
    return {
        "schema_version": 1, "status": "completed", "registry_writes": False,
        "image_settings_before": [], "image_settings_after": [],
        "child_image_settings_before": [], "child_image_settings_after": [],
        "image_sha256_before": HASH, "image_sha256_after": HASH,
        "child_image_sha256_before": HASH, "child_image_sha256_after": HASH,
        "supervisor_sha256": HASH, "request_sha256": HASH,
        "os": {"is_64_bit": True}, "available_memory_kib": 2097152,
        "runtime_inputs_unchanged": True, "adapter_sha256": HASH,
        "runtime_files_sha256": {"fixed-runtime": HASH, CONSOLE: HASH}, "system_console_path": CONSOLE,
        "execution": {
            "pid": 12, "exit_code": 0, "failure": None, "cleanup_failure": None,
            "process_limit": 4,
            "output_overflow": False, "cleanup_complete": True, "streams_released": True,
            "process_handle_released": True, "gate_sent": True, "stdin_encoding": "utf-8",
            "stdin_preamble_hex": "EF-BB-BF", "active_processes_before_cleanup": 0,
            "active_processes_after_cleanup": 0, "total_processes": len(identities),
            "total_processes_before_gate": len(before), "active_processes_before_gate": len(before),
            "assigned_before_gate": len(before), "listed_before_gate": len(before),
            "members_before_gate": before, "membership_read_failure_type": None,
            "job_events": events, "event_reader_joined": True, "event_port_closed": True,
            "event_limit_reached": False, "event_job_handle_retained": False,
            "event_read_failure_type": None, "event_shutdown_failure_type": None,
            "parent_token": token, "child_token": dict(token), "stdout": "",
            "stderr": "launcher-child-pid=13\n" if authored == 2 else "launcher-stage=gate;os-error=none\n",
        },
    }


def nested_observation(mode="job-native-foreign", helpers=3):
    """Three fixed authored images; kernel-shaped facts, not native evidence."""
    value = observation(helpers=helpers)
    result = value["execution"]
    child_name = "engine-tests.exe" if not mode.startswith("control-") else "gate-control.exe"
    for item in result["job_events"]:
        if item["pid"] == 13 and item["kind"] == 6: item["basename"] = child_name
    index = 2 + helpers
    result["job_events"].insert(index, dict(member(20, "child", child_name), kind=6))
    result["job_events"].insert(-1, pid_event(7, 20))
    result["total_processes"] += 1
    result["process_limit"] = 6
    return value


def facade_observation(mode, helpers=1):
    """Fixed ordinary-child evidence and independent no-effect diagnostic facts."""
    value = observation(helpers=helpers)
    value["status"] = "incomplete"
    value["execution"]["exit_code"] = 2 if mode == "facade-arguments" else 1
    value["execution"]["stderr"] += FACADE_ROWS[mode]
    for event in value["execution"]["job_events"]:
        if event["kind"] == 6 and event["pid"] == 13:
            event["basename"] = "engine-facade-consumer.exe"
    return value


class NativeNestedProcessAdmission(unittest.TestCase):
    def test_exact_nested_graph_has_a_healthy_old_flat_neighbor(self):
        NATIVE.validate_console(observation(), REQUEST, HASH, HASH)
        request = dict(REQUEST, mode="job-native-foreign")
        NATIVE.validate_console(nested_observation(), request, HASH, HASH, expected_processes=3)

    def test_only_six_fixed_new_modes_can_use_the_nested_budget(self):
        modes = ("job-native-foreign", "provider-native-once", "provider-native-foreign",
                 "control-nested-healthy", "control-nested-fail", "control-nested-wait")
        self.assertEqual(set(NATIVE.NESTED_TEST_MODES.values()), set(NATIVE.HELD_KE_TESTS))
        for mode in modes:
            for helpers in range(4):
                with self.subTest(mode=mode, helpers=helpers):
                    NATIVE.validate_console(nested_observation(mode, helpers), dict(REQUEST, mode=mode),
                                            HASH, HASH, expected_processes=3)
        for mode in ("cpu", "list", "ke", "control-healthy", "control-fail", "control-wait"):
            self.assertEqual(NATIVE.mode_profile(mode)[:2], (4, 2))
            with self.subTest(mode=mode), self.assertRaises(NATIVE.Incomplete):
                NATIVE.validate_console(nested_observation(), dict(REQUEST, mode=mode), HASH, HASH, expected_processes=3)
        for mode in ("job-native-arbitrary", "--exact", "control-nested-child", True, None, ["ke"]):
            with self.subTest(mode=mode), self.assertRaises(NATIVE.Incomplete): NATIVE.mode_profile(mode)

    def test_missing_extra_or_foreign_nested_identity_never_passes(self):
        request = dict(REQUEST, mode="job-native-foreign")
        for fault in ("missing", "extra", "duplicate", "unreadable", "foreign", "console-lookalike", "marker-alias", "wrong-cap"):
            value = nested_observation(helpers=0)
            events = value["execution"]["job_events"]
            nested = next(item for item in events if item["kind"] == 6 and item["pid"] == 20)
            if fault == "missing": events.remove(nested)
            elif fault == "extra":
                events.insert(3, dict(member(21, "child", "engine-tests.exe"), kind=6))
                events.insert(-1, pid_event(7, 21)); value["execution"]["total_processes"] += 1
            elif fault == "duplicate": events.insert(3, dict(nested))
            elif fault == "unreadable": nested["read_failure_type"] = "Win32Exception"
            elif fault == "foreign": nested["image_role"] = "unknown"
            elif fault == "console-lookalike": nested.update(image_role="system-console", basename="conhost.exe", system_console=False)
            elif fault == "marker-alias": value["execution"]["stderr"] = "launcher-child-pid=12\n"
            else: value["execution"]["process_limit"] = 4
            with self.subTest(fault=fault), self.assertRaises(NATIVE.Incomplete):
                NATIVE.validate_console(value, request, HASH, HASH, expected_processes=3)
        value = nested_observation(helpers=4)
        with self.assertRaises(NATIVE.Incomplete):
            NATIVE.validate_console(value, request, HASH, HASH, expected_processes=3)

    def test_nested_gate_has_no_authored_child_and_request_cannot_supply_a_cap(self):
        for gate in ("bad", "missing"):
            value = observation(authored=1)
            value["status"] = "incomplete"
            value["execution"].update(exit_code=126, gate_sent=False, process_limit=6)
            request = dict(REQUEST, gate=gate, mode="job-native-foreign")
            NATIVE.validate_console(value, request, HASH, HASH, expected_exit=126, expected_processes=1)
            for pid in (13, 20):
                bad = json.loads(json.dumps(value))
                bad["execution"]["members_before_gate"][-1] = member(pid, "child", "engine-tests.exe")
                with self.subTest(gate=gate, pid=pid), self.assertRaises(NATIVE.Incomplete):
                    NATIVE.validate_console(bad, request, HASH, HASH, expected_exit=126, expected_processes=1)
        for field, raw in (("active_limit", 100), ("authored", 2), ("arguments", ["--exact", "arbitrary"]), ("child_path", "C:\\other.exe")):
            with self.subTest(field=field), self.assertRaises(NATIVE.Incomplete):
                NATIVE.validate_console(nested_observation(), dict(REQUEST, mode="job-native-foreign", **{field: raw}),
                                        HASH, HASH, expected_processes=3)

    def test_nested_failure_and_timeout_preserve_completion_cleanup_distinction(self):
        failed = nested_observation("control-nested-fail")
        failed["status"] = "incomplete"
        failed["execution"]["exit_code"] = 7
        NATIVE.validate_console(failed, dict(REQUEST, mode="control-nested-fail"), HASH, HASH,
                                expected_exit=7, expected_processes=3)
        waiting = nested_observation("control-nested-wait")
        waiting["status"] = "incomplete"
        waiting["execution"].update(exit_code=1, active_processes_before_cleanup=6,
            failure="TimeoutException: Owned child deadline exceeded.")
        request = dict(REQUEST, mode="control-nested-wait")
        NATIVE.validate_console(waiting, request, HASH, HASH, expected_exit=1, expected_processes=3, timeout_control=True)
        for field, bad in (("active_processes_after_cleanup", 1), ("streams_released", False),
                           ("event_reader_joined", False), ("event_job_handle_retained", True),
                           ("event_port_closed", False), ("event_read_failure_type", "Win32Exception")):
            value = json.loads(json.dumps(waiting)); value["execution"][field] = bad
            with self.subTest(field=field), self.assertRaises(NATIVE.Incomplete):
                NATIVE.validate_console(value, request, HASH, HASH, expected_exit=1, expected_processes=3, timeout_control=True)

    def test_nested_notifications_keep_the_existing_bound_and_historical_exit_join(self):
        request = dict(REQUEST, mode="job-native-foreign")
        for fault in ("sixteen-events", "unknown-exit", "nested-before-gate", "reader-race"):
            value = nested_observation()
            events = value["execution"]["job_events"]
            if fault == "sixteen-events":
                events.extend(pid_event(4, 0) for _ in range(16 - len(events)))
            elif fault == "unknown-exit":
                next(item for item in events if item["kind"] == 7)["pid"] = 99
            elif fault == "nested-before-gate":
                value["execution"]["members_before_gate"][-1] = member(20, "child", "engine-tests.exe")
            else:
                value["execution"]["event_read_failure_type"] = "Win32Exception"
            with self.subTest(fault=fault), self.assertRaises(NATIVE.Incomplete):
                NATIVE.validate_console(value, request, HASH, HASH, expected_processes=3)

    def test_exact_nested_test_requires_one_success_and_the_full_filtered_roster(self):
        names = sorted([*NATIVE.HELD_KE_TESTS, *(f"synthetic_neighbor_{index}" for index in range(389))])
        roster = {"tests": names, "held_ke_tests": list(NATIVE.HELD_KE_TESTS)}
        for mode, name in NATIVE.NESTED_TEST_MODES.items():
            output = f"running 1 test\ntest {name} ... ok\n\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 391 filtered out; finished in 0.01s\n"
            result = NATIVE.validate_nested_execution(output, roster, mode)
            self.assertEqual((result["test"], result["tests_executed"], result["filtered_tests"]), (name, 1, 391))
            for bad in (output.replace("391 filtered", "390 filtered"), output.replace("1 passed", "0 passed"),
                        output.replace("0 ignored", "1 ignored"), output.replace("... ok", "... FAILED"),
                        output.replace(name, "unrelated_test")):
                with self.subTest(mode=mode, bad=bad), self.assertRaises(NATIVE.Incomplete):
                    NATIVE.validate_nested_execution(bad, roster, mode)


class EngineArtifactAdmission(unittest.TestCase):
    """Synthetic old/new Cargo layouts preserve exact Windows libtest custody."""

    def test_old_and_new_selector_require_exact_owner_profile_and_canonical_file(self):
        for folder in ("deps", "build/nevrotick-engine/" + "a" * 16 + "/out"):
            with self.subTest(folder=folder), tempfile.TemporaryDirectory() as temporary:
                work = Path(temporary)
                name = "/work/target/x86_64-pc-windows-msvc/debug/" + folder + "/nevrotick_engine-" + "b" * 16 + ".exe"
                local = work / name.removeprefix("/work/"); local.parent.mkdir(parents=True); local.write_bytes(b"synthetic executable")
                record = {"reason": "compiler-artifact", "package_id": "path+file:///work/fixture/crates/nevrotick-engine#0.0.0",
                    "manifest_path": "/work/fixture/crates/nevrotick-engine/Cargo.toml",
                    "target": {"name": "nevrotick_engine", "kind": ["lib"], "crate_types": ["lib"]},
                    "profile": {"test": True, "opt_level": "0", "debug_assertions": True, "overflow_checks": True}, "features": ["ke-association-prototype"],
                    "executable": name, "filenames": [name]}
                for features in ((), ("ke-association-prototype",)):
                    record["features"] = list(features)
                    self.assertEqual(NATIVE.engine_artifact(json.dumps(record), work, features=features), local)
                with self.assertRaises(NATIVE.Incomplete):
                    NATIVE.engine_artifact(json.dumps(record), work, features=())
                for fault in ("package", "manifest", "profile", "inventory", "extra_executable", "host", "duplicate", "symlink"):
                    bad = json.loads(json.dumps(record))
                    if fault == "package": bad["package_id"] += "wrong"
                    if fault == "manifest": bad["manifest_path"] = "/outside/Cargo.toml"
                    if fault == "profile": bad["profile"]["overflow_checks"] = False
                    if fault == "inventory": bad["filenames"] = []
                    if fault == "extra_executable": bad["filenames"].append(name.replace("nevrotick_engine", "unattributed"))
                    if fault == "host": bad["executable"] = name.replace("/x86_64-pc-windows-msvc", "")
                    if fault == "symlink":
                        moved = local.with_name("other.exe"); local.rename(moved); local.symlink_to(moved)
                    text = json.dumps(bad)
                    if fault == "duplicate": text += "\n" + text
                    with self.subTest(fault=fault), self.assertRaises(NATIVE.Incomplete):
                        NATIVE.engine_artifact(text, work)


class EngineLibraryAdmission(unittest.TestCase):
    """A facade consumes normal Windows libraries from the frozen Cargo graph."""

    def fixture(self, temporary, layout="deps"):
        work, source = Path(temporary) / "work", Path(temporary) / "source"
        work.mkdir(); source.mkdir()
        vendor_manifest = source.parent / "vendor/tokio-1.54.1/Cargo.toml"
        vendor_manifest.parent.mkdir(parents=True)
        vendor_manifest.write_text('[package]\nname="tokio"\nversion="1.54.1"\n')
        manifest = source / "crates/nevrotick-engine/Cargo.toml"
        manifest.parent.mkdir(parents=True)
        manifest.write_text('[package]\nname="nevrotick-engine"\nversion="0.0.0"\n'
                            '[dependencies]\ntokio={version="=1.54.1",default-features=false,features=["rt"]}\n'
                            '[features]\nke-association-prototype=[]\n')
        (source / "Cargo.lock").write_text(
            'version=4\n[[package]]\nname="nevrotick-engine"\nversion="0.0.0"\n'
            'dependencies=["tokio"]\n[[package]]\nname="tokio"\nversion="1.54.1"\n'
            'source="registry+https://github.com/rust-lang/crates.io-index"\n'
            'checksum="' + "a" * 64 + '"\n')
        records, libraries = [], {}
        for name, package in (("nevrotick_engine", "nevrotick-engine"), ("tokio", "tokio")):
            folder = ("deps" if layout == "deps" else
                      "build/" + package + "/" + "a" * 16 + "/out")
            filename = "lib" + name + "-" + "b" * 16 + ".rlib"
            if layout == "root" and name == "nevrotick_engine":
                folder, filename = "", "libnevrotick_engine.rlib"
            virtual = "/work/target/x86_64-pc-windows-msvc/debug/" + (folder + "/" if folder else "") + filename
            local = work / virtual.removeprefix("/work/")
            local.parent.mkdir(parents=True, exist_ok=True); local.write_bytes(b"synthetic normal rlib")
            libraries[name] = local
            records.append({"reason": "compiler-artifact",
                "package_id": ("path+file:///work/fixture/crates/nevrotick-engine#0.0.0" if name == "nevrotick_engine"
                               else "registry+https://github.com/rust-lang/crates.io-index#tokio@1.54.1"),
                "manifest_path": ("/work/fixture/crates/nevrotick-engine/Cargo.toml" if name == "nevrotick_engine"
                                  else "/vendor/tokio-1.54.1/Cargo.toml"),
                "target": {"name": name, "kind": ["lib"], "crate_types": ["lib"]},
                "profile": {"test": False, "opt_level": "0", "debug_assertions": True, "overflow_checks": True},
                "features": ["ke-association-prototype"] if name == "nevrotick_engine" else ["rt"],
                "executable": None, "filenames": [virtual]})
        return work, source, records, libraries

    @staticmethod
    def encoded(records):
        return "\n".join(json.dumps(record) for record in records)

    def test_normal_libraries_keep_root_old_and_new_layout_neighbors(self):
        for layout in ("deps", "build", "root"):
            with self.subTest(layout=layout), tempfile.TemporaryDirectory() as temporary:
                work, source, records, expected = self.fixture(temporary, layout)
                libraries, directories = NATIVE.engine_libraries(self.encoded(records), work, source,
                                                                 vendor=source.parent / "vendor")
                self.assertEqual(libraries, expected)
                self.assertEqual(directories, sorted({path.parent for path in expected.values()}))
                for record in records:
                    rmeta = record["filenames"][0].removesuffix(".rlib") + ".rmeta"
                    if layout == "root" and record["target"]["name"] == "nevrotick_engine":
                        rmeta = "/work/target/x86_64-pc-windows-msvc/debug/deps/libnevrotick_engine-" + "b" * 16 + ".rmeta"
                    local = work / rmeta.removeprefix("/work/")
                    local.parent.mkdir(parents=True, exist_ok=True)
                    local.write_bytes(b"synthetic normal metadata")
                    record["filenames"].append(rmeta)
                self.assertEqual(NATIVE.engine_libraries(self.encoded(records), work, source,
                                                        vendor=source.parent / "vendor")[0], expected)

    def test_libtest_host_or_foreign_graph_cannot_supply_normal_libraries(self):
        faults = ("libtest", "boolean-profile", "missing-profile", "overflow-off", "assertions-off", "optimized",
                  "executable", "host-target", "wrong-package",
                  "wrong-manifest", "wrong-kind", "wrong-crate-type", "missing-feature", "extra-feature",
                  "tokio-version", "tokio-source", "tokio-manifest", "tokio-runtime", "missing-tokio", "duplicate")
        for fault in faults:
            with self.subTest(fault=fault), tempfile.TemporaryDirectory() as temporary:
                work, source, records, _ = self.fixture(temporary)
                if fault == "libtest": records[0]["profile"]["test"] = True
                elif fault == "overflow-off": records[0]["profile"]["overflow_checks"] = False
                elif fault == "assertions-off": records[0]["profile"]["debug_assertions"] = False
                elif fault == "optimized": records[0]["profile"]["opt_level"] = "3"
                elif fault == "boolean-profile": records[0]["profile"]["test"] = 0
                elif fault == "missing-profile": del records[0]["profile"]["test"]
                elif fault == "executable": records[0]["executable"] = records[0]["filenames"][0]
                elif fault == "host-target":
                    records[0]["filenames"][0] = records[0]["filenames"][0].replace("/x86_64-pc-windows-msvc", "")
                    host = work / records[0]["filenames"][0].removeprefix("/work/")
                    host.parent.mkdir(parents=True); host.write_bytes(b"synthetic existing host rlib")
                elif fault == "wrong-package": records[0]["package_id"] += "-foreign"
                elif fault == "wrong-manifest": records[0]["manifest_path"] = "/outside/Cargo.toml"
                elif fault == "wrong-kind": records[0]["target"]["kind"] = ["bin"]
                elif fault == "wrong-crate-type": records[0]["target"]["crate_types"] = ["cdylib"]
                elif fault == "missing-feature": records[0]["features"] = []
                elif fault == "extra-feature": records[0]["features"].append("foreign-feature")
                elif fault == "tokio-version": records[1]["package_id"] = records[1]["package_id"].replace("1.54.1", "1.54.2")
                elif fault == "tokio-source": records[1]["package_id"] = records[1]["package_id"].replace("crates.io-index", "foreign-index")
                elif fault == "tokio-manifest": records[1]["manifest_path"] = "/vendor/foreign/Cargo.toml"
                elif fault == "tokio-runtime": records[1]["features"] = []
                elif fault == "missing-tokio": records.pop()
                else: records.append(json.loads(json.dumps(records[0])))
                with self.assertRaises(NATIVE.Incomplete):
                    NATIVE.engine_libraries(self.encoded(records), work, source, vendor=source.parent / "vendor")

    def test_library_membership_and_real_files_are_mandatory(self):
        for fault in ("absent", "directory", "symlink", "parent-symlink", "outside", "second-library", "unknown-file"):
            with self.subTest(fault=fault), tempfile.TemporaryDirectory() as temporary:
                work, source, records, libraries = self.fixture(temporary)
                local = libraries["nevrotick_engine"]
                if fault == "absent": local.unlink()
                elif fault == "directory": local.unlink(); local.mkdir()
                elif fault == "symlink":
                    target = local.with_suffix(".actual"); local.rename(target); local.symlink_to(target)
                elif fault == "parent-symlink":
                    target = local.parent.with_name("actual-deps"); local.parent.rename(target); local.parent.symlink_to(target, target_is_directory=True)
                elif fault == "outside": records[0]["filenames"] = ["/outside/libnevrotick_engine.rlib"]
                elif fault == "second-library":
                    other = local.with_name("libnevrotick_engine-" + "c" * 16 + ".rlib"); other.write_bytes(b"foreign normal rlib")
                    records[0]["filenames"].append("/work/" + str(other.relative_to(work)))
                else:
                    other = local.with_suffix(".unattributed"); other.write_bytes(b"unattributed output")
                    records[0]["filenames"].append("/work/" + str(other.relative_to(work)))
                with self.assertRaises(NATIVE.Incomplete):
                    NATIVE.engine_libraries(self.encoded(records), work, source, vendor=source.parent / "vendor")

    def test_frozen_manifest_and_lock_choose_the_actual_normal_graph(self):
        for fault in ("engine-version", "tokio-version", "tokio-source", "duplicate-tokio", "manifest-version"):
            with self.subTest(fault=fault), tempfile.TemporaryDirectory() as temporary:
                work, source, records, _ = self.fixture(temporary)
                lock = source / "Cargo.lock"
                if fault == "engine-version": lock.write_text(lock.read_text().replace('version="0.0.0"', 'version="0.0.1"'))
                elif fault == "tokio-version": lock.write_text(lock.read_text().replace('version="1.54.1"', 'version="1.54.2"'))
                elif fault == "tokio-source": lock.write_text(lock.read_text().replace("crates.io-index", "foreign-index"))
                elif fault == "duplicate-tokio":
                    lock.write_text(lock.read_text() + '\n[[package]]\nname="tokio"\nversion="1.54.1"\n'
                                    'source="registry+https://github.com/rust-lang/crates.io-index"\n')
                else:
                    manifest = source / "crates/nevrotick-engine/Cargo.toml"
                    manifest.write_text(manifest.read_text().replace("=1.54.1", "=1.54.2"))
                with self.assertRaises(NATIVE.Incomplete):
                    NATIVE.engine_libraries(self.encoded(records), work, source, vendor=source.parent / "vendor")

    def test_registry_library_uses_the_declared_crate_alias_and_exact_manifest(self):
        for fault in (None, "target-name", "declared-name", "package-name", "package-version",
                      "manifest-path", "manifest-link", "directory-link", "macro-kind", "macro-string"):
            with self.subTest(fault=fault), tempfile.TemporaryDirectory() as temporary:
                work, source, records, expected = self.fixture(temporary)
                vendor = source.parent / "vendor"
                manifest = vendor / "rustls-webpki-0.103.16/Cargo.toml"
                manifest.parent.mkdir()
                declared = ('[package]\nname="rustls-webpki"\nversion="0.103.16"\n'
                            '[lib]\nname="webpki"\n')
                manifest.write_text(declared)
                lock = source / "Cargo.lock"
                lock.write_text(lock.read_text() + '\n[[package]]\nname="rustls-webpki"\nversion="0.103.16"\n'
                                'source="registry+https://github.com/rust-lang/crates.io-index"\n')
                virtual = "/work/target/x86_64-pc-windows-msvc/debug/deps/libwebpki-" + "c" * 16 + ".rlib"
                (work / virtual.removeprefix("/work/")).write_bytes(b"synthetic declared webpki library")
                alias = json.loads(json.dumps(records[1]))
                alias.update(package_id="registry+https://github.com/rust-lang/crates.io-index#rustls-webpki@0.103.16",
                             manifest_path="/vendor/rustls-webpki-0.103.16/Cargo.toml", filenames=[virtual], features=[])
                alias["target"]["name"] = "webpki"
                records.append(alias)
                if fault == "target-name": alias["target"]["name"] = "rustls_webpki"
                elif fault == "declared-name": manifest.write_text(declared.replace('name="webpki"', 'name="foreign"'))
                elif fault == "package-name": manifest.write_text(declared.replace('name="rustls-webpki"', 'name="foreign"'))
                elif fault == "package-version": manifest.write_text(declared.replace('version="0.103.16"', 'version="0.103.15"'))
                elif fault == "manifest-path": alias["manifest_path"] = "/vendor/foreign/Cargo.toml"
                elif fault == "manifest-link":
                    target = manifest.with_name("actual.toml"); manifest.rename(target); manifest.symlink_to(target)
                elif fault == "directory-link":
                    target = manifest.parent.with_name("actual-webpki"); manifest.parent.rename(target)
                    manifest.parent.symlink_to(target, target_is_directory=True)
                elif fault == "macro-kind": manifest.write_text(declared + 'proc-macro=true\n')
                elif fault == "macro-string": manifest.write_text(declared + 'proc-macro="false"\n')
                if fault is None:
                    self.assertEqual(NATIVE.engine_libraries(self.encoded(records), work, source, vendor=vendor)[0], expected)
                else:
                    with self.assertRaises(NATIVE.Incomplete):
                        NATIVE.engine_libraries(self.encoded(records), work, source, vendor=vendor)


class FacadeFaultPreparation(unittest.TestCase):
    """Source-only faults have independent literal bodies, never a kill oracle."""

    FILE = "crates/nevrotick-engine/src/nts_ke_experiment/mod.rs"
    # Exact historical FILE bytes from 9cbdc249e8cf43aa5399386a8a491d183f3d650d.
    FIXTURE = ROOT / "scripts/tests/fixtures/engine-facade-fault-v19.rs.txt"
    SOURCE_SHA = "fda1bb48ac9751bb70a31f9501f7f8cca04de82b4481c1beab9c5e456c173287"
    # Retained cargo-mutants hunk: stdlib SequenceMatcher can align the identical
    # final braces differently even when the complete mutated source is exact.
    ENTRY_DIFF = (
        b"--- crates/nevrotick-engine/src/nts_ke_experiment/mod.rs\n"
        b"+++ replace entry -> std::process::ExitCode with Default::default()\n"
        b"@@ -1250,25 +1250,10 @@\n"
        b" \n"
        b" /// Admit two bounded CLI arguments and an optional exact protected-datagram flag.\n"
        b" ///\n"
        b" /// Successful counts and ordinary operation failures follow actual joined retirement.\n"
        b" /// Post-submission collection refusals report unconfirmed completion while permanent\n"
        b" /// custody retains the actual job; native cleanup remains separately unqualified.\n"
        b" #[cfg(windows)]\n"
        b" pub(super) fn entry(mut arguments: impl Iterator<Item = OsString>) -> std::process::ExitCode {\n"
        b"-    use std::process::ExitCode;\n"
        b"-    let _program = arguments.next();\n"
        b"-    let config = match Config::from_arguments(arguments) {\n"
        b"-        Ok(config) => config,\n"
        b"-        Err(primary) => return report_failure(&Failed::from(primary), ExitCode::from(2)),\n"
        b"-    };\n"
        b"-    match run(config) {\n"
        b"-        Ok(completion) => {\n"
        b"-            let mut output = io::stdout().lock();\n"
        b"-            if output::write_completion(&completion.projection(), &mut output).is_err() {\n"
        b"-                return ExitCode::FAILURE;\n"
        b"-            }\n"
        b"-            ExitCode::SUCCESS\n"
        b"-        }\n"
        b"-        Err(failure) => report_run_failure(&failure),\n"
        b"-    }\n"
        b"+    Default::default() /* ~ changed by cargo-mutants ~ */\n"
        b" }\n"
    )
    SPECS = (
        ("run-default", "run", "-> Result<datagram::ReceivedCompletion, RunFailed>", 889, 897, 894, 896, 7,
         "Ok(Default::default())", "286aa05c9850dc196b7802b37c71ce420ba93bb9b5666b92fd784dee21593f92",
         b"    run_synchronously_with(config, &PROCESS_WORKER, None, |slot, prepared| {\n        slot.try_submit(prepared)\n    })"),
        ("report-run-failure-default", "report_run_failure", "-> std::process::ExitCode", 947, 952, 950, 951, 36,
         "Default::default()", "70f2389f06be3dd4072feafcd04aec04cb90913f02824c063a445954a80d83a6",
         b"    let _diagnostic_result = output::write_run_failure(failure, &mut io::stderr().lock());\n    std::process::ExitCode::FAILURE"),
        ("report-failure-default", "report_failure", "-> std::process::ExitCode", 1063, 1068, 1066, 1067, 9,
         "Default::default()", "3e05f53f0fd4cc424b1ae2dfa7482a30bddb075e3a3e5a192f47bedee6c4b5b7",
         b"    let _diagnostic_result = output::write_failure(failure, &mut io::stderr().lock());\n    code"),
        ("entry-default", "entry", "-> std::process::ExitCode", 1251, 1274, 1258, 1273, 6,
         "Default::default()", "aba04af7225bf2d9850e76db7aea2c5606175442f4790acfd501d4308f89bdd5",
         b"    use std::process::ExitCode;\n    let _program = arguments.next();\n    let config = match Config::from_arguments(arguments) {\n        Ok(config) => config,\n        Err(primary) => return report_failure(&Failed::from(primary), ExitCode::from(2)),\n    };\n    match run(config) {\n        Ok(completion) => {\n            let mut output = io::stdout().lock();\n            if output::write_completion(&completion.projection(), &mut output).is_err() {\n                return ExitCode::FAILURE;\n            }\n            ExitCode::SUCCESS\n        }\n        Err(failure) => report_run_failure(&failure),\n    }"),
    )

    def facts(self):
        source = self.FIXTURE.read_bytes()
        self.assertEqual(hashlib.sha256(source).hexdigest(), self.SOURCE_SHA)
        records, expected = [], []
        for identifier, function, returns, first, last, start, end, column, replacement, diff_sha, old in self.SPECS:
            descriptor = {"name": f"{self.FILE}:{start}:5: replace {function} {returns} with {replacement}",
                "package": "nevrotick-engine", "file": self.FILE, "genre": "FnValue", "replacement": replacement,
                "function": {"function_name": function, "return_type": returns,
                             "span": {"start": {"line": first, "column": 1}, "end": {"line": last, "column": 2}}},
                "span": {"start": {"line": start, "column": 5}, "end": {"line": end, "column": column}}}
            self.assertEqual(source.count(old), 1)
            changed = source.replace(old, b"    " + replacement.encode() + b" /* ~ changed by cargo-mutants ~ */", 1)
            diff = "".join(difflib.unified_diff(source.decode().splitlines(keepends=True),
                changed.decode().splitlines(keepends=True), fromfile=self.FILE,
                tofile=f"replace {function} {returns} with {replacement}", n=8)).encode()
            if function == "entry": diff = self.ENTRY_DIFF
            self.assertEqual(hashlib.sha256(diff).hexdigest(), diff_sha)
            raw_path = f"diff/crates__nevrotick-engine__src__nts_ke_experiment__mod.rs_line_{start}_col_5.diff"
            records.append({"summary": "MissedMutant", "scenario": {"Mutant": descriptor}})
            expected.append({"id": identifier, "mutant": descriptor, "source": changed, "diff": diff,
                             "raw_diff_path": raw_path, "source_sha256": hashlib.sha256(changed).hexdigest(),
                             "diff_sha256": diff_sha})
        return source, {"outcomes": records}, expected

    def test_all_four_overlays_match_literal_bodies_and_retained_diff_hashes(self):
        source, record, expected = self.facts()
        before = json.dumps(record, sort_keys=True)
        actual = NATIVE.facade_fault_sources(source, record)
        self.assertEqual(actual, expected)
        self.assertEqual(json.dumps(record, sort_keys=True), before)
        self.assertEqual(hashlib.sha256(source).hexdigest(), self.SOURCE_SHA)
        for row in actual:
            self.assertEqual(set(row), {"id", "mutant", "source", "diff", "raw_diff_path", "source_sha256", "diff_sha256"})

    def test_missing_extra_duplicate_and_nonmissed_faults_refuse(self):
        source, healthy, _ = self.facts()
        for fault in ("missing", "extra", "duplicate", "caught", "unviable", "poison"):
            record = json.loads(json.dumps(healthy))
            if fault == "missing": record["outcomes"].pop()
            elif fault in ("extra", "duplicate"):
                extra = json.loads(json.dumps(record["outcomes"][0]))
                if fault == "extra":
                    extra["scenario"]["Mutant"]["name"] += " foreign"
                    record["outcomes"].append(extra)
                else: record["outcomes"][-1] = extra
            elif fault == "poison": record["outcomes"][0]["scenario"] = {"Mutant": "secret untrusted payload"}
            else: record["outcomes"][0]["summary"] = "CaughtMutant" if fault == "caught" else "Unviable"
            with self.subTest(fault=fault), self.assertRaises(NATIVE.Incomplete):
                NATIVE.facade_fault_sources(source, record)

    def test_every_descriptor_owner_and_span_is_exact(self):
        source, healthy, _ = self.facts()
        for path, value in (("name", "foreign"), ("package", "foreign"), ("file", "../mod.rs"),
                            ("genre", "BinaryOperator"), ("replacement", "Ok(())"),
                            ("function.function_name", "foreign"), ("function.return_type", "-> ()"),
                            ("function.span.start.line", 890), ("function.span.start.column", True),
                            ("function.span.end.column", 3), ("span.start.line", 895),
                            ("span.start.column", False), ("span.end.line", 897), ("span.end.column", 8)):
            record = json.loads(json.dumps(healthy)); target = record["outcomes"][0]["scenario"]["Mutant"]
            fields = path.split(".")
            for field in fields[:-1]: target = target[field]
            target[fields[-1]] = value
            with self.subTest(path=path), self.assertRaises(NATIVE.Incomplete):
                NATIVE.facade_fault_sources(source, record)
        for location in ("descriptor", "function"):
            record = json.loads(json.dumps(healthy)); target = record["outcomes"][0]["scenario"]["Mutant"]
            if location == "function": target = target["function"]
            target["unrecognized"] = "untrusted"
            with self.subTest(location=location), self.assertRaises(NATIVE.Incomplete):
                NATIVE.facade_fault_sources(source, record)

    def test_stale_source_and_malformed_record_shapes_refuse(self):
        source, record, _ = self.facts()
        for stale in (source + b"\n", source.replace(b"fn run(", b"fn fun(", 1), b"", source.decode(), None):
            with self.subTest(source_type=type(stale).__name__), self.assertRaises(NATIVE.Incomplete):
                NATIVE.facade_fault_sources(stale, record)
        for malformed in (None, [], {}, {"outcomes": None}, {"outcomes": {}}, {"outcomes": [None]}):
            with self.subTest(record=malformed), self.assertRaises(NATIVE.Incomplete):
                NATIVE.facade_fault_sources(source, malformed)

    def test_raw_log_paths_cannot_choose_overlay_paths(self):
        source, record, expected = self.facts()
        for row in record["outcomes"]:
            row.update(diff_path="../../outside", log_path="/absolute/private", id="foreign")
        self.assertEqual(NATIVE.facade_fault_sources(source, record), expected)

    def physical_fixture(self, temporary):
        source, record, expected = self.facts()
        base = Path(temporary); root, raw = base / "root", base / "raw"
        for name in NATIVE.EXTRAS:
            path = root / name; path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes((ROOT / name).read_bytes())
        path = root / self.FILE; path.parent.mkdir(parents=True, exist_ok=True); path.write_bytes(source)
        raw.mkdir(); outcomes = raw / "outcomes.json"
        outcomes.write_text(json.dumps(record, sort_keys=True) + "\n")
        for row in expected:
            path = raw / row["raw_diff_path"]; path.parent.mkdir(parents=True, exist_ok=True); path.write_bytes(row["diff"])
        return root, outcomes, base / "output", expected

    def invoke(self, root, outcomes, output, *, raw_digest=None, drift=None):
        calls = 0
        def admitted(_root):
            nonlocal calls
            calls += 1
            if calls == 2 and drift:
                paths = {"source": root / self.FILE, "helper": root / NATIVE.EXTRAS[0],
                         "raw": outcomes.parent / "diff/crates__nevrotick-engine__src__nts_ke_experiment__mod.rs_line_894_col_5.diff",
                         "output": output / "run-default/mod.rs"}
                path = paths[drift]; path.write_bytes(path.read_bytes() + b"\n")
            return {}, {self.FILE: hashlib.sha256((root / self.FILE).read_bytes()).hexdigest()}
        if raw_digest is None: raw_digest = hashlib.sha256(outcomes.read_bytes()).hexdigest()
        with (patch.object(NATIVE, "ROOT", root),
              patch.object(NATIVE, "FACADE_FAULT_OUTCOMES_SHA256", raw_digest),
              patch.object(NATIVE.PRODUCT, "admit_product", side_effect=admitted),
              patch.object(NATIVE.sys, "argv", ["verify-engine-native", "--lane", "prepare-facade-faults",
                           "--raw-outcomes", str(outcomes), "--output-dir", str(output)]),
              contextlib.redirect_stdout(io.StringIO())):
            return NATIVE.main()

    def test_cli_physical_healthy_neighbor_writes_only_exact_source_artifacts(self):
        with tempfile.TemporaryDirectory() as temporary:
            root, outcomes, output, expected = self.physical_fixture(temporary)
            self.assertEqual(self.invoke(root, outcomes, output), 0)
            report = json.loads((output / "verification.json").read_text())
            self.assertEqual(report["status"], "prepared-not-executed")
            for row in expected:
                self.assertEqual((output / row["id"] / "mod.rs").read_bytes(), row["source"])
                self.assertEqual((output / row["id"] / "mutation.diff").read_bytes(), row["diff"])
            for field in ("source_custody_unchanged", "helper_custody_unchanged", "raw_custody_unchanged", "output_custody_unchanged"):
                self.assertIs(report[field], True)

    def test_cli_refuses_redirected_or_changed_physical_inputs(self):
        for fault in ("raw-link", "raw-parent-link", "raw-directory", "raw-fifo", "raw-oversized",
                      "raw-duplicate-key", "diff-link", "changed-diff", "source-link", "occupied-output"):
            with self.subTest(fault=fault), tempfile.TemporaryDirectory() as temporary:
                root, outcomes, output, expected = self.physical_fixture(temporary)
                digest = hashlib.sha256(outcomes.read_bytes()).hexdigest()
                if fault == "raw-link":
                    actual = outcomes.with_name("actual.json"); outcomes.rename(actual); outcomes.symlink_to(actual)
                elif fault == "raw-parent-link":
                    actual = outcomes.parent.with_name("actual-raw"); outcomes.parent.rename(actual)
                    outcomes.parent.symlink_to(actual, target_is_directory=True)
                elif fault == "raw-directory": outcomes.unlink(); outcomes.mkdir()
                elif fault == "raw-fifo": outcomes.unlink(); os.mkfifo(outcomes)
                elif fault == "raw-oversized":
                    with outcomes.open("wb") as stream: stream.truncate(NATIVE.ALLOCATION.MAX_JSON + 1)
                elif fault == "raw-duplicate-key":
                    text = outcomes.read_text().rstrip()
                    outcomes.write_text(text[:-1] + ',"outcomes":[]}\n')
                    digest = hashlib.sha256(outcomes.read_bytes()).hexdigest()
                elif fault in ("diff-link", "changed-diff"):
                    path = outcomes.parent / expected[0]["raw_diff_path"]
                    if fault == "changed-diff": path.write_bytes(b"poisoned diff\n")
                    else:
                        actual = path.with_name("actual.diff"); path.rename(actual); path.symlink_to(actual)
                elif fault == "source-link":
                    path = root / self.FILE; actual = path.with_name("actual.rs"); path.rename(actual); path.symlink_to(actual)
                else: output.mkdir(); (output / "existing").write_bytes(b"keep")
                self.assertEqual(self.invoke(root, outcomes, output, raw_digest=digest), 1)
                if fault == "occupied-output": self.assertEqual((output / "existing").read_bytes(), b"keep")
                if fault == "raw-duplicate-key":
                    report = json.loads((output / "verification.json").read_text())
                    self.assertIn("duplicate", report["error"].lower())

    def test_cli_final_bookends_refuse_source_helper_raw_and_output_drift(self):
        for drift in ("source", "helper", "raw", "output"):
            with self.subTest(drift=drift), tempfile.TemporaryDirectory() as temporary:
                root, outcomes, output, _ = self.physical_fixture(temporary)
                self.assertEqual(self.invoke(root, outcomes, output, drift=drift), 1)
                report = json.loads((output / "verification.json").read_text())
                self.assertEqual(report["status"], "incomplete")
                self.assertIs(report[drift + "_custody_unchanged"], False)
        with self.subTest(primary="raw-digest", secondary="helper"), tempfile.TemporaryDirectory() as temporary:
            root, outcomes, output, _ = self.physical_fixture(temporary)
            self.assertEqual(self.invoke(root, outcomes, output, raw_digest="f" * 64, drift="helper"), 1)
            report = json.loads((output / "verification.json").read_text())
            self.assertEqual(report["error"], "Facade outcomes differ from the exact retained V19 bytes.")
            self.assertIs(report["helper_custody_unchanged"], False)
            self.assertIn("helper_custody_unchanged", report["custody_errors"])


class PreparedFacadeFaultAdmission(unittest.TestCase):
    """The build consumes a complete physical preparation, not caller labels."""

    def prepared(self, temporary):
        fixture = FacadeFaultPreparation()
        root, outcomes, output, expected = fixture.physical_fixture(temporary)
        digest = hashlib.sha256(outcomes.read_bytes()).hexdigest()
        self.assertEqual(fixture.invoke(root, outcomes, output), 0)
        inputs = {fixture.FILE: fixture.SOURCE_SHA}
        extras = {name: hashlib.sha256((root / name).read_bytes()).hexdigest() for name in NATIVE.EXTRAS}
        return root, output / "verification.json", expected, inputs, extras, digest

    def test_reader_requires_all_four_exact_physical_faults_and_current_maps(self):
        with tempfile.TemporaryDirectory() as temporary:
            root, receipt, expected, inputs, extras, digest = self.prepared(temporary)
            with (patch.object(NATIVE, "ROOT", root),
                  patch.object(NATIVE, "FACADE_FAULT_OUTCOMES_SHA256", digest)):
                for row in expected:
                    actual = NATIVE.read_prepared_facade_fault(receipt, row["id"], inputs, extras)
                    self.assertEqual(actual["id"], row["id"])
                    self.assertEqual(actual["mutant"], row["mutant"])
                    self.assertEqual(actual["source"], row["source"])
                    self.assertEqual(actual["source_sha256"], row["source_sha256"])
                    self.assertEqual(actual["diff_sha256"], row["diff_sha256"])
                    self.assertEqual(actual["receipt_sha256"], hashlib.sha256(receipt.read_bytes()).hexdigest())
                    self.assertIn(receipt, actual["custody_inputs"])
                with self.assertRaises(NATIVE.Incomplete):
                    NATIVE.read_prepared_facade_fault(receipt, "foreign", inputs, extras)

    def test_reader_refuses_poisoned_receipts_manifests_files_and_redirects(self):
        faults = ("status", "lane", "custody", "error", "count", "manifest-hash", "candidate", "extra-field",
                  "missing-fault", "duplicate-fault", "mutant", "source-bytes", "diff-bytes", "original-source-bytes",
                  "receipt-link", "manifest-link", "source-link", "source-parent-link", "inputs", "extras")
        for fault in faults:
            with self.subTest(fault=fault), tempfile.TemporaryDirectory() as temporary:
                root, receipt, expected, inputs, extras, digest = self.prepared(temporary)
                record = json.loads(receipt.read_text()); manifest_path = receipt.parent / "facade-faults.json"
                manifest = json.loads(manifest_path.read_text()); row = manifest["faults"][0]
                if fault == "status": record["status"] = "passed"
                elif fault == "lane": record["lane"] = "build"
                elif fault == "custody": record["raw_custody_unchanged"] = False
                elif fault == "error": record["error"] = "retained primary failure"
                elif fault == "count": record["prepared_fault_count"] = True
                elif fault == "manifest-hash": record["manifest_sha256"] = "f" * 64
                elif fault == "candidate": manifest["candidate_sha256"] = "f" * 64
                elif fault == "extra-field": manifest["unrecognized"] = "untrusted"
                elif fault == "missing-fault": manifest["faults"].pop()
                elif fault == "duplicate-fault": manifest["faults"][-1] = json.loads(json.dumps(row))
                elif fault == "mutant": row["mutant"]["replacement"] = "Ok(())"
                elif fault == "source-bytes": (receipt.parent / row["source_path"]).write_bytes(b"foreign source")
                elif fault == "diff-bytes": (receipt.parent / row["diff_path"]).write_bytes(b"foreign diff")
                elif fault == "original-source-bytes":
                    source = root / FacadeFaultPreparation.FILE
                    source.write_bytes(source.read_bytes() + b"\n")
                elif fault == "inputs": inputs[FacadeFaultPreparation.FILE] = "f" * 64
                elif fault == "extras": extras[NATIVE.SELF] = "f" * 64
                manifest_path.write_text(json.dumps(manifest, indent=2) + "\n")
                if fault != "manifest-hash": record["manifest_sha256"] = hashlib.sha256(manifest_path.read_bytes()).hexdigest()
                receipt.write_text(json.dumps(record, indent=2) + "\n")
                if fault in ("receipt-link", "manifest-link", "source-link", "source-parent-link"):
                    path = receipt if fault == "receipt-link" else manifest_path if fault == "manifest-link" else receipt.parent / row["source_path"]
                    if fault == "source-parent-link": path = path.parent
                    target = path.with_name(path.name + "-actual"); path.rename(target)
                    path.symlink_to(target, target_is_directory=fault == "source-parent-link")
                physical = fault in ("receipt-link", "manifest-link", "source-link", "source-parent-link")
                refusal = (self.assertRaisesRegex(NATIVE.PE.Incomplete, "violates file custody") if physical
                           else self.assertRaises(NATIVE.Incomplete))
                if fault == "original-source-bytes":
                    refusal = self.assertRaisesRegex(NATIVE.Incomplete, "exact unchanged Engine module")
                with (patch.object(NATIVE, "ROOT", root),
                      patch.object(NATIVE, "FACADE_FAULT_OUTCOMES_SHA256", digest), refusal):
                    NATIVE.read_prepared_facade_fault(receipt, expected[0]["id"], inputs, extras)

    def test_fault_record_cannot_become_native_baseline_by_relabeling_status(self):
        with tempfile.TemporaryDirectory() as temporary:
            root, prepared, _, inputs, extras, _ = self.prepared(temporary)
            receipt = prepared.parent / "healthy-build.json"
            expected = {key: {"synthetic": HASH} for key in NATIVE.FACADE_HEALTHY_FIELDS}
            expected.update(product_source_sha256=inputs, extra_inputs_sha256=extras,
                            candidate_sha256=NATIVE.ALLOCATION.canonical_digest(inputs),
                            product_target_flags=NATIVE.ENV.MSVC_FLAGS, facade_target_flags=NATIVE.ENV.MSVC_FLAGS)
            diagnostic = list(NATIVE.ENV.MSVC_FLAGS)
            diagnostic[diagnostic.index("target-cpu=x86-64-v3")] = "target-cpu=x86-64"
            expected["diagnostic_target_flags"] = diagnostic
            record = self.healthy_record(receipt, expected)
            args = NATIVE.argparse.Namespace(build_receipt=receipt, native_dir=Path("/must-not-allocate"),
                                            facade_consumer=True, output_dir=prepared.parent)
            with (patch.object(NATIVE, "ROOT", root),
                  patch.object(NATIVE.PRODUCT, "admit_product", return_value=({}, inputs)),
                  patch.object(NATIVE, "native_directory", side_effect=NATIVE.Incomplete("ordinary admission sentinel")) as allocate):
                with self.assertRaisesRegex(NATIVE.Incomplete, "ordinary admission sentinel"):
                    NATIVE.native(args, {})
                allocate.assert_called_once_with(args.native_dir)
                for metadata in ({"facade_fault": "report-run-failure-default"},
                                 {"variant_source_sha256": inputs}, {"prepared_faults_sha256": HASH}):
                    allocate.reset_mock()
                    receipt.write_text(json.dumps(dict(record, **metadata), default=str))
                    with self.assertRaisesRegex(NATIVE.Incomplete, "Fault-build metadata is not admitted"):
                        NATIVE.native(args, {})
                    allocate.assert_not_called()

    def healthy_record(self, receipt, expected):
        record = dict(expected, schema_version=1, status="passed", lane="build", facade_consumer=True,
                      bookends_passed=True, input_bookends_passed=True, native_windows_execution=False,
                      normal_library_bookends=True, exported_artifact_bookends=True, custody_errors=[],
                      input_bookends={name: True for name in ("product", "helpers", "copied_source", "control",
                                      "vendor", "tools", "stable_payload", "windows_libraries", "sdk_records")},
                      artifacts={})
        for name in ("gated-launcher.exe", "cpu-v3-probe.exe", "gate-control.exe", "engine-facade-consumer.exe"):
            image = receipt.parent / name; image.write_bytes(("independent PE " + name).encode())
            pdb = image.with_suffix(".pdb"); pdb.write_bytes(("independent PDB " + name).encode())
            record["artifacts"][name] = {"path": str(image), "sha256": NATIVE.PRODUCT.sha256(image),
                                         "pdb_sha256": NATIVE.PRODUCT.sha256(pdb)}
        receipt.write_text(json.dumps(record, default=str))
        return record

    def test_healthy_parent_requires_matching_current_environment_and_physical_pairs(self):
        faults = (None, "status", "fault-metadata", "candidate", "tool-map", "bool-alias", "flags", "diagnostic-flags",
                  "input-failed", "normal-unreached", "export-failed", "custody-error", "missing-bookend",
                  "extra-bookend", "false-bookend", "bool-bookend", "fault-bookend",
                  "missing-image", "image-bytes", "pdb-bytes", "image-link", "path-redirect")
        for fault in faults:
            with self.subTest(fault=fault), tempfile.TemporaryDirectory() as temporary:
                receipt = Path(temporary) / "verification.json"
                expected = {key: {"independent": HASH} for key in NATIVE.FACADE_HEALTHY_FIELDS}
                expected.update(profile="debug", diagnostic_profile="release",
                                windows_inputs={"library_files": 1},
                                product_target_flags=NATIVE.ENV.MSVC_FLAGS,
                                facade_target_flags=NATIVE.ENV.MSVC_FLAGS)
                diagnostic = list(NATIVE.ENV.MSVC_FLAGS)
                diagnostic[diagnostic.index("target-cpu=x86-64-v3")] = "target-cpu=x86-64"
                expected["diagnostic_target_flags"] = diagnostic
                record = self.healthy_record(receipt, expected)
                image = receipt.parent / "engine-facade-consumer.exe"
                if fault == "status": record["status"] = "fault-build-completed"
                elif fault == "fault-metadata": record["facade_fault"] = "entry-default"
                elif fault == "candidate": record["candidate_sha256"] = "f" * 64
                elif fault == "tool-map": record["tools_sha256"] = {}
                elif fault == "bool-alias": record["windows_inputs"] = {"library_files": True}
                elif fault == "flags": record["facade_target_flags"] = []
                elif fault == "diagnostic-flags": record["diagnostic_target_flags"] = []
                elif fault == "input-failed": record["input_bookends_passed"] = False
                elif fault == "normal-unreached": record["normal_library_bookends"] = "not-reached"
                elif fault == "export-failed": record["exported_artifact_bookends"] = False
                elif fault == "custody-error": record["custody_errors"] = ["helpers"]
                elif fault == "missing-bookend": record["input_bookends"].pop("helpers")
                elif fault == "extra-bookend": record["input_bookends"]["unrecognized"] = True
                elif fault == "false-bookend": record["input_bookends"]["helpers"] = False
                elif fault == "bool-bookend": record["input_bookends"]["helpers"] = 1
                elif fault == "fault-bookend": record["input_bookends"]["fault_inputs"] = True
                elif fault == "missing-image": image.unlink()
                elif fault == "image-bytes": image.write_bytes(b"changed PE")
                elif fault == "pdb-bytes": image.with_suffix(".pdb").write_bytes(b"changed PDB")
                elif fault == "image-link":
                    moved = image.with_suffix(".actual"); image.rename(moved); image.symlink_to(moved)
                elif fault == "path-redirect": record["artifacts"][image.name]["path"] = str(image.with_name("foreign.exe"))
                receipt.write_text(json.dumps(record))
                if fault is None:
                    actual = NATIVE.read_healthy_facade_build(receipt, expected)
                    self.assertEqual(actual["receipt"], record)
                    self.assertEqual(len(actual["custody_inputs"]), 9)
                else:
                    with self.assertRaises((NATIVE.Incomplete, NATIVE.PE.Incomplete)):
                        NATIVE.read_healthy_facade_build(receipt, expected)

    def build_case(self, *, drift=None, refuse=False, bad_parent=None):
        """Use real preparation/build readers and orchestration; external tools are observations."""
        with tempfile.TemporaryDirectory() as temporary:
            fixture = FacadeFaultPreparation()
            root, outcomes, preparation, rows = fixture.physical_fixture(temporary)
            base = Path(temporary)
            names = ("Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "policy/tool-manifest.json",
                     "crates/nevrotick-engine/Cargo.toml", fixture.FILE)
            for name in names[:-1]:
                path = root / name; path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes((ROOT / name).read_bytes())
            def product(_root):
                return {}, {name: NATIVE.PRODUCT.sha256(root / name) for name in names}
            digest = hashlib.sha256(outcomes.read_bytes()).hexdigest()
            with (patch.object(NATIVE, "ROOT", root),
                  patch.object(NATIVE, "FACADE_FAULT_OUTCOMES_SHA256", digest),
                  patch.object(NATIVE.PRODUCT, "admit_product", side_effect=product),
                  patch.object(NATIVE.sys, "argv", ["verify-engine-native", "--lane", "prepare-facade-faults",
                               "--raw-outcomes", str(outcomes), "--output-dir", str(preparation)]),
                  contextlib.redirect_stdout(io.StringIO())):
                self.assertEqual(NATIVE.main(), 0)
            vendor, toolchain, sdk = (base / name for name in ("vendor", "toolchain", "sdk"))
            for path in (vendor, toolchain, sdk): path.mkdir()
            markers = {name: directory / "identity" for name, directory in
                       (("vendor", vendor), ("tools", toolchain), ("stable_payload", base))}
            for name, path in markers.items(): path.write_bytes(("independent " + name).encode())
            sdk_file = sdk / "windows.lib"; sdk_file.write_bytes(b"independent Windows library")
            config, inventory = base / "xwin.toml", base / "xwin.json"
            config.write_text("synthetic SDK config"); inventory.write_text("{}")
            def stable():
                return {"stable": toolchain, "inventories": [toolchain / "inventory.json"],
                        "readonly_roots": [toolchain], "policy_sha256": NATIVE.PRODUCT.sha256(markers["stable_payload"]),
                        "installation_pointer_sha256": HASH}
            active, commands, normal = {}, [], {}
            selected = next(row for row in rows if row["id"] == "report-failure-default")
            def mutate():
                paths = {"product": root / "Cargo.lock", "helpers": root / NATIVE.EXTRAS[0],
                         "copied_source": active["output"] / "source" / fixture.FILE,
                         "control": active["output"] / "source/gate-control.rs",
                         "windows_libraries": sdk_file, "sdk_records": inventory,
                         "fault_inputs": preparation / selected["id"] / "mod.rs", **markers}
                if drift in paths:
                    path = paths[drift]; path.write_bytes(path.read_bytes() + b"\nchanged custody input\n")
                elif drift == "fault_input_link":
                    path = preparation / selected["id"] / "mod.rs"
                    actual = path.with_name("physical-original.rs"); path.rename(actual); path.symlink_to(actual)
            def command(argv, **_kwargs):
                output = active["output"]
                copied = (output / "source" / fixture.FILE).read_bytes()
                cargo = argv[0] == "cargo"
                if active["fault"] and cargo:
                    self.assertEqual(copied, selected["source"])
                elif not active.get("overlay_seen"):
                    self.assertEqual(hashlib.sha256(copied).hexdigest(), fixture.SOURCE_SHA)
                if cargo:
                    active["overlay_seen"] = active["fault"]
                    if active["fault"]:
                        mutate()
                        if refuse:
                            return {"command": argv, "exit_code": 101, "failure": None,
                                    "output": "error[E0277]: synthetic compiler refusal at selected facade\n"}
                commands.append((active["fault"], list(argv)))
                if "-o" in argv:
                    path = output / "work" / argv[argv.index("-o") + 1].removeprefix("/work/")
                    path.parent.mkdir(parents=True, exist_ok=True); path.write_bytes(b"synthetic emitted PE or metadata")
                    if path.suffix == ".exe": path.with_suffix(".pdb").write_bytes(b"synthetic emitted PDB")
                    if active["fault"] and path.name == "engine-facade-consumer.exe" and drift == "normal_libraries":
                        normal["nevrotick_engine"].write_bytes(b"changed normal library")
                return {"command": argv, "exit_code": 0, "failure": None, "output": "synthetic external observation\n"}
            def libraries(_raw, work, _source, *, vendor, evidence):
                for name in ("nevrotick_engine", "tokio"):
                    path = work / "target" / NATIVE.TARGET / "debug/deps" / ("lib" + name + "-" + "b" * 16 + ".rlib")
                    path.parent.mkdir(parents=True, exist_ok=True); path.write_bytes(("normal " + name).encode())
                    normal[name] = path
                    evidence["/work/" + str(path.relative_to(work))] = {"sha256": NATIVE.PRODUCT.sha256(path)}
                return dict(normal), sorted({path.parent for path in normal.values()})
            def pe(image):
                if active["fault"] and drift == "exported_artifacts" and image.name == "engine-facade-consumer.exe":
                    (active["output"] / "gated-launcher.exe").write_bytes(b"changed exported image")
                return {"synthetic": True}
            def args(output, fault):
                output.mkdir()
                return NATIVE.argparse.Namespace(output_dir=output, vendor_dir=vendor, toolchain_dir=toolchain,
                    xwin_config=config, xwin_inventory=inventory, test_roster=None, facade_consumer=True, jobs=6,
                    facade_fault=selected["id"] if fault else None,
                    prepared_faults=preparation / "verification.json" if fault else None,
                    healthy_build_receipt=base / "healthy/verification.json" if fault else None)
            with (patch.object(NATIVE, "ROOT", root),
                  patch.object(NATIVE, "FACADE_FAULT_OUTCOMES_SHA256", digest),
                  patch.object(NATIVE.PRODUCT, "admit_product", side_effect=product),
                  patch.object(NATIVE.PRODUCT, "admit_vendor", side_effect=lambda *_args: {"vendor": NATIVE.PRODUCT.sha256(markers["vendor"])}),
                  patch.object(NATIVE.PRODUCT, "admit_tools", side_effect=lambda *_args: {"compiler": NATIVE.PRODUCT.sha256(markers["tools"])}),
                  patch.object(NATIVE.PRODUCT, "windows_libraries", side_effect=lambda *_args: (sdk, {"windows.lib": NATIVE.PRODUCT.sha256(sdk_file)})),
                  patch.object(NATIVE.PRODUCT, "library_inventory", side_effect=lambda *_args: {"windows.lib": NATIVE.PRODUCT.sha256(sdk_file)}),
                  patch.object(NATIVE.PRODUCT, "sandbox", side_effect=lambda _tool, _work, _source, _vendor, argv, *_rest: argv),
                  patch.object(NATIVE.PRODUCT, "boundary_control"),
                  patch.object(NATIVE.ANALYSIS, "admit", side_effect=lambda *_args, **_kw: stable()),
                  patch.object(NATIVE.ENV, "require_admitted_compiler"),
                  patch.object(NATIVE.ENV, "run", side_effect=command),
                  patch.object(NATIVE.PE, "pe_readback", side_effect=pe),
                  patch.object(NATIVE, "engine_libraries", side_effect=libraries)):
                healthy_args = args(base / "healthy", False)
                active.update(output=healthy_args.output_dir, fault=False, overlay_seen=False)
                healthy = {"schema_version": 1, "lane": "build"}; NATIVE.build(healthy_args, healthy)
                self.assertEqual(healthy["status"], "passed")
                self.assertTrue(healthy["bookends_passed"])
                if bad_parent == "fault": healthy["facade_fault"] = "entry-default"
                elif bad_parent == "stale": healthy["tools_sha256"] = {"compiler": "f" * 64}
                healthy_args.output_dir.joinpath("verification.json").write_text(json.dumps(healthy, default=str))
                fault_args = args(base / "fault", True)
                active.update(output=fault_args.output_dir, fault=True, overlay_seen=False)
                report = {"lane": "build"}
                before = len(commands)
                if bad_parent or refuse or drift:
                    message = "Command failed: cargo: 101" if refuse else "Healthy parent|Healthy facade" if bad_parent else "Build custody incomplete"
                    with self.assertRaisesRegex(NATIVE.Incomplete, message): NATIVE.build(fault_args, report)
                    self.assertEqual(report["status"], "incomplete")
                    self.assertFalse(report["bookends_passed"])
                    if bad_parent:
                        self.assertEqual(len(commands), before)
                    if refuse:
                        self.assertEqual(report["normal_library_bookends"], "not-reached")
                        self.assertEqual(report["exported_artifact_bookends"], "not-reached")
                    if drift:
                        label = "fault_inputs" if drift == "fault_input_link" else drift
                        self.assertIn(label, report["custody_errors"])
                    else:
                        self.assertTrue(report["input_bookends_passed"])
                else:
                    NATIVE.build(fault_args, report)
                    self.assertEqual(report["status"], "fault-build-completed")
                    self.assertTrue(report["bookends_passed"])
                    self.assertTrue(report["input_bookends_passed"])
                    self.assertIs(report["normal_library_bookends"], True)
                    self.assertEqual(report["variant_source_sha256"][fixture.FILE], selected["source_sha256"])
                    self.assertNotEqual(report["variant_candidate_sha256"], report["candidate_sha256"])
                self.assertTrue(any(is_fault and argv[0] == "python3" for is_fault, argv in commands) or bad_parent)

    def test_real_build_admits_ordinary_and_exact_single_overlay_neighbors(self):
        self.build_case()

    def test_real_build_refuses_fault_or_stale_healthy_parent_before_execution(self):
        for fault in ("fault", "stale"):
            with self.subTest(parent=fault): self.build_case(bad_parent=fault)

    def test_compiler_refusal_runs_every_input_bookend_without_claiming_artifacts(self):
        self.build_case(refuse=True)
        for drift in ("product", "helpers", "copied_source", "control", "vendor", "tools", "stable_payload",
                      "windows_libraries", "sdk_records", "fault_inputs", "fault_input_link"):
            with self.subTest(drift=drift): self.build_case(drift=drift, refuse=True)

    def test_successful_compilation_still_refuses_normal_library_and_export_drift(self):
        for drift in ("normal_libraries", "exported_artifacts"):
            with self.subTest(drift=drift): self.build_case(drift=drift)


class TestRosterAdmission(unittest.TestCase):
    """Independent source-bound identities are never inferred from native discovery."""

    def roster(self):
        selected = ["nts_ke_experiment::first", "nts_ke_experiment::second"]
        ke = sorted([*NATIVE.HELD_KE_TESTS, *selected])
        return {"schema_version": 1, "package": "nevrotick-engine", "target": NATIVE.TARGET,
                "features": ["ke-association-prototype"], "candidate_sha256": HASH,
                "extra_inputs_sha256": {name: HASH for name in NATIVE.EXTRAS},
                "tests": ["common::neighbor", *ke], "ke_tests": ke,
                "selected_ke_tests": selected, "held_ke_tests": list(NATIVE.HELD_KE_TESTS)}

    def admit(self, value):
        return NATIVE.validate_test_roster(value, HASH, self.roster()["extra_inputs_sha256"])

    def test_exact_source_target_feature_and_complete_selected_subset(self):
        value = self.roster()
        self.assertIs(self.admit(value), value)
        faults = {
            "schema_version": True, "package": "time-core", "target": "x86_64-unknown-linux-gnu",
            "features": [], "candidate_sha256": "b" * 64, "extra_inputs_sha256": {},
            "tests": [], "ke_tests": ["nts_ke_experiment::first"],
            "selected_ke_tests": ["nts_ke_experiment::first"], "held_ke_tests": [],
        }
        for field, wrong in faults.items():
            with self.subTest(field=field), self.assertRaises(NATIVE.Incomplete):
                self.admit(dict(value, **{field: wrong}))
        for names in (["common::only"], ["nts_ke_experiment::same"] * 2,
                      ["nts_ke_experiment::z", "nts_ke_experiment::a"],
                      ["nts_ke_experiment::bad\nname"], ["nts_ke_experiment::" + "x" * 4096],
                      [True], ["nts_ke_experiment::same"] * 4097):
            with self.subTest(names=str(names)[:90]), self.assertRaises(NATIVE.Incomplete):
                self.admit(dict(value, tests=names))
        with self.assertRaises(NATIVE.Incomplete):
            self.admit(dict(value, unknown="cannot redefine the scope"))

    def test_bounded_manifest_read_preserves_bytes_and_refuses_link_or_rebinding(self):
        value = self.roster()
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "roster.json"
            raw = (json.dumps(value, indent=2) + "\n").encode()
            path.write_bytes(raw)
            self.assertEqual(NATIVE.read_test_roster(path, HASH, value["extra_inputs_sha256"]), (value, raw))
            linked = path.with_name("linked.json"); linked.symlink_to(path)
            with self.assertRaises(NATIVE.PE.Incomplete):
                NATIVE.read_test_roster(linked, HASH, value["extra_inputs_sha256"])
            parent_link = path.parent / "linked-parent"; parent_link.symlink_to(path.parent, target_is_directory=True)
            with self.assertRaises(NATIVE.PE.Incomplete):
                NATIVE.read_test_roster(parent_link / path.name, HASH, value["extra_inputs_sha256"])
            for bad in (b"{" + b" " * NATIVE.MAX_TEST_ROSTER_BYTES,
                        b'{"schema_version":1,"schema_version":1}', b'\xff'):
                path.write_bytes(bad)
                with self.subTest(raw=bad[:50]), self.assertRaises((NATIVE.Incomplete, NATIVE.PE.Incomplete, ValueError)):
                    NATIVE.read_test_roster(path, HASH, value["extra_inputs_sha256"])

    def test_listing_requires_exact_full_source_bound_identities(self):
        value = self.roster()
        names_only = "\n".join(name + ": test" for name in value["tests"]) + "\n"
        self.assertEqual(NATIVE.validate_test_listing(names_only, value), value["tests"])
        output = names_only + "\n6 tests, 0 benchmarks\n"
        self.assertEqual(NATIVE.validate_test_listing(output, value), value["tests"])
        for bad in (output.replace("common::neighbor", "common::same_count_substitute"),
                    output.replace("nts_ke_experiment::second", "nts_ke_experiment::first"),
                    output.replace("6 tests", "5 tests"), output + "unattributed output\n",
                    output + "6 tests, 0 benchmarks\n", output.replace("0 benchmarks", "1 benchmark")):
            with self.subTest(output=bad[:100]), self.assertRaises(NATIVE.Incomplete):
                NATIVE.validate_test_listing(bad, value)

    def test_selected_results_require_exact_successes_and_filtered_count(self):
        value = self.roster()
        output = "running 2 tests\n" + "\n".join("test " + name + " ... ok" for name in value["selected_ke_tests"])
        output += "\n\ntest result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.01s\n"
        NATIVE.validate_ke_execution(output, value)
        NATIVE.validate_ke_execution(output.replace("\n", "\r\n"), value)
        for bad in (output.replace("::second", "::first"), output.replace("::second", "::unexpected"),
                    output.replace("4 filtered", "3 filtered"), output.replace("0 ignored", "1 ignored"),
                    output.replace("second ... ok", "second ... ignored"),
                    output + "test common::unexpected ... ok\n", output + output,
                    output.replace("2 passed", "0 passed")):
            with self.subTest(output=bad[:100]), self.assertRaises(NATIVE.Incomplete):
                NATIVE.validate_ke_execution(bad, value)

    def test_missing_stale_or_changed_manifest_refuses_before_windows_staging(self):
        inputs = {"actual-source": HASH}
        candidate = NATIVE.ALLOCATION.canonical_digest(inputs)
        extras = self.roster()["extra_inputs_sha256"]
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            manifest = dict(self.roster(), candidate_sha256=candidate)
            roster_path = directory / NATIVE.TEST_ROSTER_NAME
            raw = json.dumps(manifest).encode(); roster_path.write_bytes(raw)
            build = {"status": "passed", "lane": "build", "product_source_sha256": inputs,
                     "candidate_sha256": candidate,
                     "extra_inputs_sha256": extras,
                     "test_roster": {"path": str(roster_path), "sha256": "b" * 64}}
            args = type("Arguments", (), {"build_receipt": directory / "verification.json",
                                         "native_dir": directory / "never-created"})()
            for fault in ("missing", "changed", "stale"):
                bad = json.loads(json.dumps(build))
                if fault == "missing": del bad["test_roster"]
                if fault == "stale":
                    roster_path.write_text(json.dumps(dict(manifest, candidate_sha256=HASH)))
                with self.subTest(fault=fault), patch.object(NATIVE, "read_record", return_value=bad), \
                     patch.object(NATIVE.PRODUCT, "admit_product", return_value=({}, inputs)), \
                     patch.object(NATIVE.PRODUCT, "sha256", return_value=HASH), \
                     patch.object(NATIVE, "native_directory") as staging:
                    with self.assertRaises(NATIVE.Incomplete):
                        NATIVE.native(args, {})
                    staging.assert_not_called()

    def test_fixed_launcher_selection_matches_exact_required_nested_child_holds(self):
        source = (ROOT / NATIVE.LAUNCHER).read_text()
        before, after = "            Self::Ke => (", "            Self::ControlHealthy =>"
        self.assertEqual(source.count(before), 1)
        body = source.split(before, 1)[1].split(after, 1)[0]
        expected = ["engine-tests.exe", "nts_ke_experiment::", "--test-threads=1"]
        for name in NATIVE.HELD_KE_TESTS:
            expected += ["--skip", name]
        self.assertEqual(re.findall(r'"([^"\n]*)"', body), expected)
        self.assertEqual(len(NATIVE.HELD_KE_TESTS), 3)


class DefaultEngineCohortAdmission(unittest.TestCase):
    """Synthetic controls for a real fixed selection, not claimed Windows evidence."""

    def roster(self):
        selected = sorted(NATIVE.DEFAULT_COHORT_CASES.values())
        return {"schema_version": 1, "package": "nevrotick-engine", "target": NATIVE.TARGET,
                "features": [], "candidate_sha256": HASH,
                "extra_inputs_sha256": {name: HASH for name in NATIVE.EXTRAS},
                "tests": sorted(["common::healthy_neighbor", *selected]),
                "selected_default_tests": selected}

    def admit(self, value):
        return NATIVE.validate_test_roster(value, HASH, self.roster()["extra_inputs_sha256"],
                                         default_cohort=True)

    def test_default_roster_requires_every_fixed_id_and_the_actual_empty_feature(self):
        value = self.roster()
        self.assertIs(self.admit(value), value)
        for field, bad in (("features", ["ke-association-prototype"]),
                           ("selected_default_tests", value["selected_default_tests"][:-1]),
                           ("tests", value["tests"][:-1]), ("candidate_sha256", "b" * 64)):
            with self.subTest(field=field), self.assertRaises(NATIVE.Incomplete):
                self.admit(dict(value, **{field: bad}))
        for change in ({"ke_tests": []}, {"selected_ke_tests": []}, {"arbitrary_filter": "source_policy"}):
            with self.subTest(change=change), self.assertRaises(NATIVE.Incomplete):
                self.admit(dict(value, **change))
        substituted = json.loads(json.dumps(value))
        substituted["selected_default_tests"][0] = "source_policy::tests::same_count_substitute"
        with self.assertRaises(NATIVE.Incomplete): self.admit(substituted)
        with self.assertRaises(NATIVE.Incomplete):
            NATIVE.validate_test_roster(value, HASH, value["extra_inputs_sha256"])
        ke = TestRosterAdmission().roster()
        with self.assertRaises(NATIVE.Incomplete): self.admit(ke)

    def test_default_result_requires_one_exact_success_and_full_filtered_count(self):
        value = self.roster()
        for mode, name in NATIVE.DEFAULT_COHORT_CASES.items():
            output = (f"running 1 test\ntest {name} ... ok\n\n"
                      f"test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; {len(value['tests']) - 1} filtered out; finished in 0.01s\n")
            self.assertEqual(NATIVE.validate_default_execution(output, value, mode), name)
            for bad in (output.replace(name, "same_count::substitute"), output.replace("1 passed", "0 passed"),
                        output.replace("0 ignored", "1 ignored"), output.replace("11 filtered", "10 filtered")):
                with self.subTest(mode=mode, bad=bad), self.assertRaises(NATIVE.Incomplete):
                    NATIVE.validate_default_execution(bad, value, mode)
        with self.assertRaises(NATIVE.Incomplete):
            NATIVE.validate_default_execution(output, value, "caller-selected-filter")

    def test_default_command_is_offline_closed_and_excludes_the_ke_feature(self):
        default = NATIVE.engine_test_command(6, default_cohort=True)
        ke = NATIVE.engine_test_command(6)
        self.assertIn("--no-default-features", default)
        self.assertNotIn("--features", default)
        self.assertNotIn("ke-association-prototype", default)
        self.assertEqual(ke[ke.index("--features") + 1], "ke-association-prototype")
        for command in (default, ke):
            self.assertIn("--frozen", command)
            self.assertIn("--no-run", command)
            self.assertEqual(command[command.index("--package") + 1], "nevrotick-engine")
        for jobs in (True, 0, 13, "6"):
            with self.subTest(jobs=jobs), self.assertRaises(NATIVE.Incomplete):
                NATIVE.engine_test_command(jobs, default_cohort=True)
        for selected in (1, None, "true"):
            with self.subTest(selected=selected), self.assertRaises(NATIVE.Incomplete):
                NATIVE.engine_test_command(6, default_cohort=selected)

    def test_default_selection_cannot_overlap_facade_or_expand_process_caps(self):
        args = type("Arguments", (), {"default_cohort": True, "facade_consumer": False})()
        self.assertTrue(NATIVE.default_cohort_requested(args))
        args.facade_consumer = True
        with self.assertRaises(NATIVE.Incomplete): NATIVE.default_cohort_requested(args)
        for mode in NATIVE.DEFAULT_COHORT_CASES:
            self.assertEqual(NATIVE.mode_profile(mode), (4, 2, "engine-tests.exe"))
        with self.assertRaises(NATIVE.Incomplete): NATIVE.mode_profile("default-arbitrary")

    def test_default_launcher_exact_names_and_supervisor_closed_modes_match(self):
        source = (ROOT / NATIVE.LAUNCHER).read_text()
        body = source.split("impl DefaultCase {", 1)[1].split("/// Closed selection", 1)[0]
        actual = re.findall(r'"([^"\n]*)"', body.split("fn arguments", 1)[1])
        expected = []
        for name in NATIVE.DEFAULT_COHORT_CASES.values():
            expected += ["--exact", name, "--test-threads=1"]
        self.assertEqual(actual, expected)
        supervisor = NATIVE.native_supervisor()
        for mode in NATIVE.DEFAULT_COHORT_CASES:
            self.assertIn("'" + mode + "'", supervisor)
            self.assertIn('case "' + mode + '":', supervisor)
        self.assertNotIn("-EncodedCommand", supervisor)


class ConsoleAdmission(unittest.TestCase):
    """False completion cannot replace custody, identity, token or actual cleanup."""

    def check(self, value, **kwargs):
        return NATIVE.validate_console(value, REQUEST, HASH, HASH, **kwargs)

    def test_healthy_neighbor(self):
        value = observation()
        self.assertIs(self.check(value), value["execution"])

    def test_required_facts_cannot_be_replaced_by_exit_zero(self):
        faults = (
            ("status", "incomplete"), ("child_image_sha256_after", "b" * 64),
            ("child_image_settings_before", ["inherited debugger"]),
            ("runtime_inputs_unchanged", False), ("available_memory_kib", 2097151),
            ("execution.cleanup_complete", False), ("execution.streams_released", False),
            ("execution.gate_sent", False), ("execution.total_processes", 1),
            ("execution.active_processes_after_cleanup", 1),
            ("execution.child_token.elevated", True),
            ("execution.child_token.user_sid", "S-1-5-21-2"),
        )
        for path, bad in faults:
            with self.subTest(path=path):
                value = observation()
                fields = path.split(".")
                target = value
                for key in fields[:-1]:
                    target = target[key]
                target[fields[-1]] = bad
                with self.assertRaises(NATIVE.Incomplete):
                    self.check(value)

    def test_malformed_shapes_have_typed_failure(self):
        with self.assertRaises(NATIVE.Incomplete):
            self.check([])
        for path in ("os", "execution", "execution.parent_token", "execution.child_token"):
            value = observation()
            fields = path.split(".")
            target = value if len(fields) == 1 else value[fields[0]]
            target[fields[-1]] = []
            with self.subTest(path=path), self.assertRaises(NATIVE.Incomplete):
                self.check(value)

    def test_booleans_do_not_supply_observed_counts(self):
        for key in ("active_processes_before_cleanup", "active_processes_after_cleanup"):
            value = observation()
            value["execution"][key] = False
            with self.subTest(key=key), self.assertRaises(NATIVE.Incomplete):
                self.check(value)
        value = observation()
        value["execution"]["child_token"]["elevation_type"] = True
        with self.assertRaises(NATIVE.Incomplete):
            self.check(value)

    def test_gate_refusal_requires_no_descendant(self):
        value = observation(authored=1)
        value["status"] = "incomplete"
        value["execution"].update(exit_code=126, gate_sent=False)
        request = dict(REQUEST, gate="missing")
        NATIVE.validate_console(value, request, HASH, HASH, expected_exit=126, expected_processes=1)
        value["execution"]["job_events"].append(dict(member(13, "child", "gate-control.exe"), kind=6))
        value["execution"]["total_processes"] += 1
        with self.assertRaises(NATIVE.Incomplete):
            NATIVE.validate_console(value, request, HASH, HASH, expected_exit=126, expected_processes=1)

    def test_timeout_is_an_expected_failure_with_actual_cleanup(self):
        value = observation()
        value["status"] = "incomplete"
        value["execution"].update(exit_code=1, active_processes_before_cleanup=2,
                                  failure="TimeoutException: Owned child deadline exceeded.")
        self.check(value, expected_exit=1, timeout_control=True)
        value["execution"]["cleanup_failure"] = "Owned job did not become empty."
        with self.assertRaises(NATIVE.Incomplete):
            self.check(value, expected_exit=1, timeout_control=True)


    def test_bound_system_console_helpers_are_counted_separately(self):
        for helpers in (0, 1, 2):
            with self.subTest(helpers=helpers):
                self.check(observation(helpers=helpers))
        value = observation(authored=1, helpers=2)
        value["status"] = "incomplete"
        value["execution"].update(exit_code=126, gate_sent=False)
        with self.assertRaises(NATIVE.Incomplete):
            NATIVE.validate_console(value, dict(REQUEST, gate="missing"), HASH, HASH,
                                    expected_exit=126, expected_processes=1)

    def test_missing_duplicate_and_unreadable_new_identity_refuse(self):
        for fault in ("missing", "duplicate", "unreadable", "unknown", "lookalike", "wrong-child"):
            value = observation()
            events = value["execution"]["job_events"]
            if fault == "missing":
                del events[1]
            elif fault == "duplicate":
                events.insert(1, dict(events[0]))
            elif fault == "unreadable":
                events[1]["read_failure_type"] = "Win32Exception"
            elif fault == "unknown":
                events[1]["image_role"] = "unknown"
            elif fault == "lookalike":
                events[2]["system_console"] = False
            else:
                events[1]["pid"] = 99
            with self.subTest(fault=fault), self.assertRaises(NATIVE.Incomplete):
                self.check(value)

    def test_child_marker_needs_exact_bounded_pid_and_kernel_join(self):
        for text in ("", "launcher-child-pid=0\n", "launcher-child-pid=4294967296\n",
                     "launcher-child-pid=12\n", "launcher-child-pid=99\n",
                     "launcher-child-pid=13\nlauncher-child-pid=13\n", "prefix launcher-child-pid=13\n"):
            value = observation()
            value["execution"]["stderr"] = text
            with self.subTest(marker=text), self.assertRaises(NATIVE.Incomplete):
                self.check(value)

    def test_child_and_unaccounted_identity_cannot_appear_before_gate(self):
        for bad in (member(13, "child", "gate-control.exe"), member(99, "system-console", "conhost.exe")):
            value = observation()
            value["execution"]["members_before_gate"][1] = bad
            with self.subTest(pid=bad["pid"]), self.assertRaises(NATIVE.Incomplete):
                self.check(value)

    def test_collector_faults_and_unretired_resources_refuse(self):
        faults = {"event_reader_joined": False, "event_port_closed": False,
                  "event_limit_reached": True, "event_job_handle_retained": True,
                  "event_read_failure_type": "InvalidDataException",
                  "event_shutdown_failure_type": "TimeoutException",
                  "membership_read_failure_type": "Win32Exception"}
        for name, bad in faults.items():
            value = observation()
            value["execution"][name] = bad
            with self.subTest(field=name), self.assertRaises(NATIVE.Incomplete):
                self.check(value)

    def test_exit_pids_join_historical_new_without_reopening(self):
        value = observation()
        self.check(value)
        value["execution"]["job_events"][-2]["pid"] = 99
        with self.assertRaises(NATIVE.Incomplete):
            self.check(value)
        value = observation()
        value["execution"]["job_events"][-2]["basename"] = "invented.exe"
        with self.assertRaises(NATIVE.Incomplete):
            self.check(value)

    def test_console_file_provenance_cannot_be_missing(self):
        value = observation()
        del value["runtime_files_sha256"][CONSOLE]
        with self.assertRaises(NATIVE.Incomplete):
            self.check(value)

    def test_memory_preflight_preserves_the_closed_observed_cause(self):
        # Minimal facts from ep3-native-run-v1's genuine pre-launch failure.
        value = {"schema_version": 1, "status": "incomplete", "available_memory_kib": 888656,
                 "error": "Main Windows available memory is below 2 GiB."}
        with self.assertRaisesRegex(NATIVE.Incomplete, "below 2 GiB; no test process started"):
            self.check(value)
        value["error"] = "arbitrary supplied secret"
        with self.assertRaises(NATIVE.Incomplete) as raised:
            self.check(value)
        self.assertNotIn("arbitrary supplied secret", str(raised.exception))



class FacadeConsoleAdmission(unittest.TestCase):
    """Exact facade refusals preserve the existing observed Windows custody rules."""

    def check(self, value, mode, **kwargs):
        return NATIVE.validate_console(value, dict(REQUEST, mode=mode), HASH, HASH,
                                       expected_exit=2 if mode == "facade-arguments" else 1, **kwargs)

    def test_only_two_fixed_facade_modes_use_the_four_process_profile(self):
        for mode in FACADE_ROWS:
            self.assertEqual(NATIVE.mode_profile(mode), (4, 2, "engine-facade-consumer.exe"))
        for mode in ("facade", "facade-arguments-extra", "facade-entered-runtime-extra", True, None, ["facade-arguments"]):
            with self.subTest(mode=mode), self.assertRaises(NATIVE.Incomplete):
                NATIVE.mode_profile(mode)
        self.assertEqual(NATIVE.mode_profile("ke"), (4, 2, "engine-tests.exe"))
        self.assertEqual(NATIVE.mode_profile("control-healthy"), (4, 2, "gate-control.exe"))

    def test_pid_marker_and_exact_facade_row_allow_either_arrival_order(self):
        for mode, row in FACADE_ROWS.items():
            for stderr in ("launcher-child-pid=13\n" + row, row + "launcher-child-pid=13\n"):
                with self.subTest(mode=mode, stderr=stderr):
                    self.assertEqual(NATIVE.observed_child_pid(stderr, 2, mode=mode), 13)
                    value = facade_observation(mode)
                    value["execution"]["stderr"] = stderr
                    self.assertIs(self.check(value, mode), value["execution"])

    def test_missing_foreign_duplicate_or_extra_diagnostics_refuse(self):
        marker = "launcher-child-pid=13\n"
        for mode, row in FACADE_ROWS.items():
            foreign = next(text for name, text in FACADE_ROWS.items() if name != mode)
            for bad in (marker, row, marker + foreign, marker + row + row, marker + marker + row,
                        marker + row + "unexpected\n", "prefix " + marker + row,
                        marker + row.replace("eligible=0", "eligible=1"),
                        marker + row.replace("fresh_sync=0", "fresh_sync=1"),
                        marker + row.replace("\n", "\r\n"),
                        marker + row.removesuffix("\n")):
                with self.subTest(mode=mode, stderr=bad), self.assertRaises(NATIVE.Incomplete):
                    NATIVE.observed_child_pid(bad, 2, mode=mode)
                value = facade_observation(mode); value["execution"]["stderr"] = bad
                with self.subTest(mode=mode, consumer="console", stderr=bad), self.assertRaises(NATIVE.Incomplete):
                    self.check(value, mode)

    def test_facade_row_cannot_expand_a_libtest_or_control_mode(self):
        for mode, row in FACADE_ROWS.items():
            for old in (None, "ke", "list", "control-healthy"):
                with self.subTest(mode=mode, old=old), self.assertRaises(NATIVE.Incomplete):
                    NATIVE.observed_child_pid("launcher-child-pid=13\n" + row, 2, mode=old)
        self.assertEqual(NATIVE.observed_child_pid("launcher-child-pid=13\n", 2), 13)
        for mode in FACADE_ROWS:
            for authored in (True, False):
                with self.subTest(mode=mode, authored=authored), self.assertRaises(NATIVE.Incomplete):
                    NATIVE.observed_child_pid("launcher-child-pid=13\n" + FACADE_ROWS[mode], authored, mode=mode)

    def test_exact_facade_failure_still_needs_child_identity_and_cleanup(self):
        for mode in FACADE_ROWS:
            for fault in ("wrong-image", "libtest-image", "foreign-marker", "before-gate",
                          "cleanup", "joined", "token", "stdout", "wrong-exit"):
                value = facade_observation(mode); result = value["execution"]
                child = next(event for event in result["job_events"] if event["kind"] == 6 and event["pid"] == 13)
                if fault == "wrong-image": child["basename"] = "foreign.exe"
                elif fault == "libtest-image": child["basename"] = "engine-tests.exe"
                elif fault == "foreign-marker": result["stderr"] = result["stderr"].replace("pid=13", "pid=99")
                elif fault == "before-gate": result["members_before_gate"][1] = dict(child)
                elif fault == "cleanup": result["active_processes_after_cleanup"] = 1
                elif fault == "joined": result["event_reader_joined"] = False
                elif fault == "token": result["child_token"]["elevated"] = True
                elif fault == "stdout": result["stdout"] = "invented success\n"
                else: result["exit_code"] = 0
                with self.subTest(mode=mode, fault=fault), self.assertRaises(NATIVE.Incomplete):
                    self.check(value, mode)

    def test_facade_observed_counts_do_not_accept_boolean_aliases(self):
        for mode in FACADE_ROWS:
            for field in ("process_limit", "total_processes", "total_processes_before_gate", "active_processes_before_gate",
                          "assigned_before_gate", "listed_before_gate", "active_processes_before_cleanup", "active_processes_after_cleanup"):
                value = facade_observation(mode); value["execution"][field] = False
                with self.subTest(mode=mode, field=field), self.assertRaises(NATIVE.Incomplete):
                    self.check(value, mode)

    def test_launcher_and_supervisor_keep_the_exact_facade_inventory(self):
        launcher = (ROOT / NATIVE.LAUNCHER).read_text()
        supervisor = NATIVE.native_supervisor()
        for mode in FACADE_ROWS:
            self.assertIn('"' + mode + '"', launcher)
            self.assertIn("'" + mode + "'", supervisor)
        self.assertIn('"engine-facade-consumer.exe"', launcher)
        self.assertIn("'engine-facade-consumer.exe'", supervisor)
        self.assertIn("limits.basic.activeLimit = ProcessLimit(mode)", supervisor)


class EvidenceCustody(unittest.TestCase):
    """Evidence reads and Windows staging refuse escapes before allocation/effects."""

    def test_facade_build_keeps_sdk_and_normal_library_bookends_distinct(self):
        """Real orchestration reaches both independent final custody consumers."""
        names = ("Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "policy/tool-manifest.json",
                 "crates/nevrotick-engine/Cargo.toml")
        inputs = {name: NATIVE.PRODUCT.sha256(ROOT / name) for name in names}

        def run_case(fault):
            with tempfile.TemporaryDirectory() as temporary:
                base = Path(temporary); output = base / "output"; output.mkdir()
                vendor, toolchain, sdk = (base / name for name in ("vendor", "toolchain", "sdk"))
                for path in (vendor, toolchain, sdk): path.mkdir()
                sdk_file = sdk / "windows.lib"; sdk_file.write_bytes(b"independent Windows SDK input")
                sdk_map = {"windows.lib": NATIVE.PRODUCT.sha256(sdk_file)}
                config, inventory = base / "xwin.toml", base / "xwin.json"
                config.write_text("synthetic SDK configuration"); inventory.write_text("{}")
                args = NATIVE.argparse.Namespace(output_dir=output, vendor_dir=vendor, toolchain_dir=toolchain,
                    xwin_config=config, xwin_inventory=inventory, test_roster=None, facade_consumer=True, jobs=6)
                stable = {"stable": toolchain, "inventories": [], "readonly_roots": [],
                          "policy_sha256": HASH, "installation_pointer_sha256": HASH}
                normal = {}

                def normal_libraries(_raw, work, _source, *, vendor, evidence):
                    for name in ("nevrotick_engine", "tokio"):
                        path = work / "target/x86_64-pc-windows-msvc/debug/deps" / ("lib" + name + "-" + "b" * 16 + ".rlib")
                        path.parent.mkdir(parents=True, exist_ok=True); path.write_bytes(("normal " + name).encode())
                        normal[name] = path
                        evidence["/work/" + str(path.relative_to(work))] = {"sha256": NATIVE.PRODUCT.sha256(path)}
                    self.assertNotEqual(evidence, sdk_map)
                    return dict(normal), sorted({path.parent for path in normal.values()})

                def external_command(command, **_kwargs):
                    if "-o" in command:
                        path = output / "work" / command[command.index("-o") + 1].removeprefix("/work/")
                        path.parent.mkdir(parents=True, exist_ok=True); path.write_bytes(b"synthetic emitted artifact")
                        if path.suffix == ".exe": path.with_suffix(".pdb").write_bytes(b"synthetic matching PDB")
                        if path.name == "engine-facade-consumer.exe":
                            if fault == "sdk-drift": sdk_file.write_bytes(b"changed SDK input")
                            elif fault == "normal-drift": normal["nevrotick_engine"].write_bytes(b"changed normal library")
                    return {"exit_code": 0, "failure": None, "output": "synthetic external observation\n"}

                build = NATIVE.build
                if fault == "old-shadow":
                    source = (ROOT / NATIVE.SELF).read_text()
                    body = source[source.index("def build("):source.index("\nNATIVE_JOB_OBSERVER =")]
                    self.assertIn("normal_library_inputs = {}", body)
                    namespace = dict(vars(NATIVE))
                    exec(compile(body.replace("normal_library_inputs", "windows_library_inputs"),
                                 "isolated-old-sdk-shadow-control", "exec"), namespace)
                    build = namespace["build"]
                report = {}
                with (patch.object(NATIVE.PRODUCT, "admit_product", return_value=({}, inputs)),
                      patch.object(NATIVE.PRODUCT, "admit_vendor", return_value={"vendor": HASH}),
                      patch.object(NATIVE.PRODUCT, "admit_tools", return_value={"compiler": HASH}),
                      patch.object(NATIVE.PRODUCT, "windows_libraries", return_value=(sdk, sdk_map)),
                      patch.object(NATIVE.PRODUCT, "library_inventory", side_effect=lambda _path: {"windows.lib": NATIVE.PRODUCT.sha256(sdk_file)}),
                      patch.object(NATIVE.PRODUCT, "sandbox", side_effect=lambda _tool, _work, _source, _vendor, command, *_rest: command),
                      patch.object(NATIVE.PRODUCT, "boundary_control"),
                      patch.object(NATIVE.ANALYSIS, "admit", return_value=stable),
                      patch.object(NATIVE.ENV, "require_admitted_compiler"),
                      patch.object(NATIVE.ENV, "run", side_effect=external_command),
                      patch.object(NATIVE.PE, "pe_readback", return_value={"synthetic": True}),
                      patch.object(NATIVE, "engine_libraries", side_effect=normal_libraries)):
                    if fault == "old-shadow":
                        namespace["engine_libraries"] = NATIVE.engine_libraries
                    if fault:
                        message = "normal_libraries" if fault == "normal-drift" else "windows_libraries"
                        with self.assertRaisesRegex(NATIVE.Incomplete, message): build(args, report)
                    else:
                        build(args, report)
                        self.assertEqual(report["status"], "passed")
                        self.assertTrue(report["bookends_passed"])
                        self.assertFalse(report["native_windows_execution"])
                        self.assertEqual(set(report["artifacts"]), {"gated-launcher.exe", "cpu-v3-probe.exe",
                                         "gate-control.exe", "engine-facade-consumer.exe"})
                        self.assertEqual(report["normal_library_inputs"].keys(),
                                         {"/work/" + str(path.relative_to(output / "work")) for path in normal.values()})

        for fault in (None, "old-shadow", "sdk-drift", "normal-drift"):
            with self.subTest(fault=fault): run_case(fault)

    def test_native_staging_refuses_traversal_with_valid_neighbor(self):
        prefix = Path("/mnt/c/Users/perso/AppData/Local/NevroTick")
        name = "test-control-" + uuid.uuid4().hex
        good = prefix / name
        self.assertEqual(NATIVE.native_directory(good), good)
        for bad in (prefix, prefix / ".." / name, prefix / "nested" / ".." / ".." / name):
            with self.subTest(path=bad), self.assertRaises(NATIVE.Incomplete):
                NATIVE.native_directory(bad)

    def test_bounded_regular_json_and_refused_links_fifo_size_duplicates(self):
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            record = directory / "record.json"
            record.write_text('{"fixture":true}')
            self.assertEqual(NATIVE.read_record(record), {"fixture": True})
            linked = directory / "linked.json"
            linked.symlink_to(record)
            fifo = directory / "fifo"
            os.mkfifo(fifo)
            for invalid in (linked, fifo, directory):
                with self.subTest(path=invalid), self.assertRaises(NATIVE.PE.Incomplete):
                    NATIVE.read_record(invalid)
            with record.open("wb") as stream:
                stream.truncate(NATIVE.ALLOCATION.MAX_JSON + 1)
            with self.assertRaises(NATIVE.PE.Incomplete):
                NATIVE.read_record(record)
            record.write_text('{"fixture":true,"fixture":false}')
            with self.assertRaises(NATIVE.PE.Incomplete):
                NATIVE.read_record(record)

    def test_changed_reused_supervisor_fails_closed(self):
        original = NATIVE.ALLOCATION.NATIVE_SUPERVISOR
        source = NATIVE.native_supervisor()
        self.assertIn("result.total_processes = TotalProcesses(job)", source)
        self.assertIn("$childLock.Dispose()", source)
        self.assertEqual(NATIVE.ALLOCATION.NATIVE_SUPERVISOR, original)
        changed = original.replace("input.Write('G');", "input.Write('Z');")
        with patch.object(NATIVE.ALLOCATION, "NATIVE_SUPERVISOR", changed):
            with self.assertRaises(NATIVE.Incomplete):
                NATIVE.native_supervisor()

    def test_payload_admission_failure_retains_partial_receipt(self):
        with tempfile.TemporaryDirectory() as temporary:
            output = Path(temporary) / "output"
            argv = ["verify-engine-native", "--lane", "build", "--output-dir", str(output)]
            with (patch.object(NATIVE.sys, "argv", argv),
                  patch.object(NATIVE, "build", side_effect=NATIVE.ANALYSIS.Incomplete("control admission failure")),
                  contextlib.redirect_stdout(io.StringIO())):
                self.assertEqual(NATIVE.main(), 1)
            report = json.loads((output / "verification.json").read_text())
            self.assertEqual(report["status"], "incomplete")
            self.assertEqual(report["error"], "control admission failure")


if __name__ == "__main__":
    unittest.main()
