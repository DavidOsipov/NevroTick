"""Host-only controls for native-test admission; never launch Windows or Cargo."""
from __future__ import annotations

import contextlib
import importlib.util
import io
import json
import os
from pathlib import Path
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
                    "profile": {"test": True, "opt_level": "0", "debug_assertions": True, "overflow_checks": True},
                    "executable": name, "filenames": [name]}
                self.assertEqual(NATIVE.engine_artifact(json.dumps(record), work), local)
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



class EvidenceCustody(unittest.TestCase):
    """Evidence reads and Windows staging refuse escapes before allocation/effects."""

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
