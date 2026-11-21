"""Exercise durable fixture metric storage, cancellation and actual archive receiving."""
import argparse
import contextlib
import importlib.util
import io
import json
import os
from pathlib import Path
import sys
import tempfile
import time
import unittest
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location("durable_fixture_control", Path(__file__).with_name("metrics.py"))
M = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(M)


class DurableMetricControls(unittest.TestCase):
    def test_real_ram_work_or_receipts_refuse_before_any_command(self):
        with tempfile.TemporaryDirectory() as disk, tempfile.TemporaryDirectory(dir="/dev/shm") as ram:
            for deadline in (float("nan"), float("inf"), -1, True):
                with self.subTest(deadline=deadline), self.assertRaises(M.Q.Incomplete):
                    M.Q.wall_timeout(5, deadline)
            for work, records in ((ram, disk), (disk, ram)):
                with self.subTest(work=work), self.assertRaisesRegex(M.Incomplete, "root mount"):
                    M.durable_roots(Path(work), Path(records), "sample")
            self.assertEqual(list(Path(ram).iterdir()), [])

    def test_disk_root_when_supported(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            work, receipts = root / "work", root / "receipts"
            work.mkdir()
            receipts.mkdir()
            with self.assertRaisesRegex(M.Incomplete, "separate"):
                M.durable_roots(root, receipts, "sample")
            try:
                observed = M.durable_roots(work, receipts, "sample")
            except M.Incomplete as error:
                self.skipTest("This host has no supported distro disk: " + str(error))
            self.assertEqual(observed["filesystem"], "ext4")
            self.assertTrue(Path(observed["device"]).is_block_device())

    def test_nested_mount_in_selected_work_archive_stage_or_lock_refuses(self):
        table_path = Path("/proc/self/mountinfo")
        table = table_path.read_bytes()
        original_open = Path.open
        with tempfile.TemporaryDirectory(prefix="fixture durable ") as temporary:
            root = Path(temporary)
            work, records = root / "cache", root / "records"
            work.mkdir()
            records.mkdir()
            for nested in (work / "run/work/target", records / "run-evidence/raw",
                           records / "durable-metric-stages", work / "writer.lock"):
                encoded = str(nested).replace(" ", r"\040")
                amended = table + f"9876543 1 0:99 / {encoded} rw - tmpfs none rw\n".encode()
                def observed(path, *args, **kwargs):
                    return io.BytesIO(amended) if path == table_path else original_open(path, *args, **kwargs)
                with self.subTest(nested=nested), patch.object(Path, "open", observed), \
                        self.assertRaisesRegex(M.Incomplete, "nested mount"):
                    M.durable_roots(work, records, "run")

    def test_stage_binds_before_resource_refusal_and_retries_keep_one_boot_and_deadline(self):
        with tempfile.TemporaryDirectory() as directory, contextlib.ExitStack() as stack:
            root = Path(directory)
            cache, records = root / "cache", root / "records"
            cache.mkdir(mode=0o700)
            records.mkdir()
            args = argparse.Namespace(lane="coverage", label="attempt-a", equivalence_review=None, direct_inventory=None,
                                      durable_stage="frozen", deadline_monotonic=time.monotonic() + 60)
            stack.enter_context(patch.object(M.Q, "CACHE", cache))
            stack.enter_context(patch.object(M.Q, "RECORDS", records))
            stack.enter_context(patch.object(M.Q, "disable_core_dumps", return_value={}))
            stack.enter_context(patch.object(M, "durable_roots", return_value={"root": "observed"}))
            stack.enter_context(patch.object(M.shutil, "disk_usage", return_value=argparse.Namespace(free=1)))
            def bound_under_lock(*params, **options):
                with (cache / "writer.lock").open("rb") as independent:
                    with self.assertRaises(BlockingIOError):
                        M.fcntl.flock(independent, M.fcntl.LOCK_EX | M.fcntl.LOCK_NB)
                return original_bind(*params, **options)
            original_bind = M.bind_stage
            stack.enter_context(patch.object(M, "bind_stage", side_effect=bound_under_lock))
            with self.assertRaisesRegex(M.Incomplete, "6 GiB"):
                M.run(args)
            binding = records / "durable-metric-stages/frozen/binding.json"
            original = binding.read_bytes()
            args.label = "attempt-b"
            with self.assertRaisesRegex(M.Incomplete, "6 GiB"):
                M.run(args)
            deadline = args.deadline_monotonic
            args.deadline_monotonic += 10
            with self.assertRaisesRegex(M.Incomplete, "stage boot or absolute deadline changed"):
                M.run(args)
            args.deadline_monotonic = deadline
            with patch.object(M, "boot_id", return_value="00000000-0000-0000-0000-000000000000"), \
                    self.assertRaisesRegex(M.Incomplete, "stage boot or absolute deadline changed"):
                M.run(args)
            self.assertEqual(binding.read_bytes(), original)
            self.assertFalse((cache / args.label).exists())
            (records / "durable-metric-stages/alias").symlink_to(binding.parent)
            args.durable_stage = "alias"
            with self.assertRaises((ValueError, OSError, M.Incomplete)):
                M.run(args)
            self.assertEqual(binding.read_bytes(), original)

            binding.rename(binding.with_suffix(".retained"))
            args.durable_stage = "frozen"
            with self.assertRaises((OSError, M.Incomplete)):
                M.run(args)
            self.assertFalse(binding.exists())

    def test_deadline_at_actual_group_owner_stops_child_and_refuses_next_dispatch(self):
        q = M.Q
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            work, vendor, stable = root / "work", root / "vendor", root / "tools"
            source = work / "source"
            work.mkdir(mode=0o700)
            for path in (source, vendor, stable, work / "cargo-home"):
                path.mkdir(mode=0o700)
            (work / "cargo-home/config.toml").write_text("control")
            admitted = {"stable": stable, "extensions": {}, "helpers": q.host_helpers()}
            owner = q.C.PRODUCT.ENV.run
            marker = work / "ready"
            program = "import time; from pathlib import Path; Path('ready').write_text('ready'); time.sleep(10)"
            def command(_argv, **options):
                return owner([sys.executable, "-c", program], **dict(options, cwd=work))
            with patch.object(q, "check_core_dumps"), patch.object(q, "check_service_tools"), \
                    patch.object(q, "resource_stop_reason", return_value=None), \
                    patch.object(q.C.PRODUCT.ENV, "run", side_effect=command) as dispatch:
                deadline = time.monotonic() + 0.5
                options = {"timeout": 5, "deadline_monotonic": deadline}
                result = q.run_command(work, [str(stable / "bin/cargo")], admitted, vendor, source, **options)
                self.assertEqual(marker.read_bytes(), b"ready")
                self.assertLess(result["exit_code"], 0)
                self.assertIn("wall timeout", result["failure"])
                with self.assertRaisesRegex(q.Incomplete, "deadline exhausted"):
                    q.run_command(work, [str(stable / "bin/cargo")], admitted, vendor, source, **options)
                self.assertEqual(dispatch.call_count, 1)

                program = ("import os,time; from pathlib import Path; "
                           "Path('ready').write_text(str(os.getpid())); "
                           "print('retained timeout observation', flush=True); time.sleep(10)")
                deadline = time.monotonic() + 0.3
                argv = [str(stable / "bin/cargo")]
                (source / q.TOOL).mkdir(mode=0o700, parents=True)
                session = M.Session.__new__(M.Session)
                session.work, session.source, session.vendor, session.admitted = work, source, vendor, admitted
                session.args, session.record = argparse.Namespace(deadline_monotonic=deadline), {"commands": []}
                postchecks = []
                session.check = lambda: postchecks.append("source check")
                with patch.object(M.shutil, "disk_usage", return_value=argparse.Namespace(free=2**33)), \
                        self.assertRaisesRegex(M.Incomplete, "deadline exhausted during bounded child"), \
                        M.stage_time(deadline, "bounded child"):
                    session.run("bounded child", argv, 5)
                    self.fail("expired command returned to its caller")
                self.assertFalse(Path("/proc", marker.read_text()).exists())
                self.assertEqual(len(session.record["commands"]), 1)
                retained = session.record["commands"][0]
                self.assertEqual(retained["label"], "bounded child")
                self.assertEqual(retained["argv"], argv)
                self.assertEqual(retained["command"], [sys.executable, "-c", program])
                self.assertEqual(retained["namespace_argv"], ["/toolchain/bin/cargo"])
                self.assertEqual(retained["output"], "retained timeout observation\n")
                self.assertLess(retained["exit_code"], 0)
                self.assertIn("wall timeout", retained["failure"])
                self.assertEqual(postchecks, ["source check"])
                self.assertEqual(dispatch.call_count, 2)

                # A physical interrupt while the real owned child is running
                # must unwind through the group owner's cleanup before returning.
                program = ("import os,signal,time; from pathlib import Path; "
                           "Path('ready').write_text(str(os.getpid())); "
                           f"assert os.getppid() == {os.getpid()}; "
                           "os.kill(os.getppid(), signal.SIGINT); time.sleep(10)")
                with self.assertRaises(KeyboardInterrupt):
                    q.run_command(work, [str(stable / "bin/cargo")], admitted, vendor, source,
                                  timeout=5, deadline_monotonic=time.monotonic() + 5)
                self.assertFalse(Path("/proc", marker.read_text()).exists())

    def test_failed_late_and_stale_runs_preserve_originals_and_unknown_siblings(self):
        for effect in ("passed", "late-admission", "late-bookends", "late-export", "refused", "interrupted",
                       "interrupt-bookends", "exit-bookends", "interrupt-export", "stale"):
            with self.subTest(effect=effect), tempfile.TemporaryDirectory() as directory, contextlib.ExitStack() as stack:
                root = Path(directory)
                cache, records = root / "cache", root / "records"
                cache.mkdir(mode=0o700)
                records.mkdir()
                sibling = cache / "unreceipted-sibling"
                sibling.write_bytes(b"unclassified")
                original = cache / "sample/dirty-overlay.rs"
                archive = records / "sample-evidence/raw/dirty-overlay.rs"
                receipt = records / "sample.json"
                args = argparse.Namespace(lane="coverage", label="sample", equivalence_review=None, direct_inventory=None,
                                          durable_stage="test-stage", deadline_monotonic=time.monotonic() + 3)
                for module, name, value in ((M.Q, "CACHE", cache), (M.Q, "RECORDS", records),
                        (M.Q, "ROOT", root), (M.E, "RECORDS", records)):
                    stack.enter_context(patch.object(module, name, value))
                stack.enter_context(patch.object(M.Q, "disable_core_dumps", return_value={}))
                stack.enter_context(patch.object(M.shutil, "disk_usage", return_value=argparse.Namespace(free=2 ** 33)))
                stack.enter_context(patch.object(M, "durable_roots", return_value={"work_root": str(cache)}))
                session = stack.enter_context(patch.object(M, "Session"))
                def workload(_session):
                    original.write_bytes(b"exact failed derivative")
                    active_record = session.call_args.args[1]
                    if effect in ("interrupt-bookends", "exit-bookends"):
                        active_record["metrics_passed"] = True
                    if effect in ("late-export", "interrupt-export"):
                        active_record["pilot_passed"] = True
                    if effect == "interrupted":
                        raise KeyboardInterrupt()
                    if effect == "refused":
                        raise M.Incomplete("fixture refusal")
                stack.enter_context(patch.object(M, "execute", side_effect=workload))
                if effect == "stale":
                    receipt.write_bytes(b"old")
                    with self.assertRaisesRegex(M.Incomplete, "already owns evidence"):
                        M.run(args)
                    self.assertEqual(receipt.read_bytes(), b"old")
                    session.assert_not_called()
                else:
                    if effect.startswith("late-"):
                        args.deadline_monotonic = time.monotonic() + 0.15
                    if effect == "late-admission":
                        def stalled_admission(*_args):
                            original.write_bytes(b"exact failed derivative")
                            time.sleep(2)
                            self.fail("Admission continued after its deadline")
                        session.side_effect = stalled_admission
                    if effect == "late-bookends":
                        def stalled_bookends():
                            time.sleep(2)
                            self.fail("Bookends continued after their deadline")
                        session.return_value.finish.side_effect = stalled_bookends
                    if effect in ("interrupt-export", "late-export"):
                        args.lane = "worker-replay-pilot"
                        args.direct_inventory = root / "inventory"
                        args.reviewed_inventory = root / "review"
                        copier = KeyboardInterrupt if effect == "interrupt-export" else (lambda *_: time.sleep(2))
                        stack.enter_context(patch.object(M.E, "copy_regular", side_effect=copier))
                    if effect in ("interrupt-bookends", "exit-bookends"):
                        session.return_value.finish.side_effect = KeyboardInterrupt if effect == "interrupt-bookends" else SystemExit
                    if effect in ("interrupted", "interrupt-bookends", "exit-bookends", "interrupt-export"):
                        with self.assertRaises(SystemExit if effect == "exit-bookends" else KeyboardInterrupt):
                            M.run(args)
                    else:
                        status = M.run(args)
                        self.assertEqual(status, "passed" if effect == "passed" else "incomplete")
                    data = json.loads(receipt.read_bytes())
                    self.assertEqual(original.read_bytes(), b"exact failed derivative")
                    self.assertEqual(data["deadline_monotonic"], args.deadline_monotonic)
                    pin = (records / "durable-metric-stages/test-stage/binding.json").read_bytes()
                    self.assertEqual(data["durable_stage"]["binding_sha256"], M.hashlib.sha256(pin).hexdigest())
                    self.assertEqual(data["durable_stage"]["boot_id"], M.boot_id())
                    if effect in ("interrupted", "interrupt-bookends", "exit-bookends", "interrupt-export"):
                        error_type = "SystemExit" if effect == "exit-bookends" else "KeyboardInterrupt"
                        self.assertIn("interrupted: " + error_type, str(data["errors"]))
                    if effect in ("interrupt-bookends", "exit-bookends"):
                        self.assertFalse(data["metrics_passed"])
                        self.assertFalse(data["bookends_passed"])
                    if effect in ("late-export", "interrupt-export"):
                        self.assertFalse(data["pilot_passed"])
                    if effect == "interrupt-export":
                        self.assertNotIn("export_complete", data)
                    elif effect.startswith("late-"):
                        self.assertIn("deadline exhausted during", str(data["errors"]))
                        self.assertNotIn("export_complete", data)
                        if effect == "late-admission":
                            M.execute.assert_not_called()
                            session.return_value.finish.assert_not_called()
                    else:
                        self.assertTrue(data["export_complete"])
                        self.assertEqual(archive.read_bytes(), original.read_bytes())
                    self.assertEqual(data["status"] == "passed", effect == "passed")
                self.assertEqual(sibling.read_bytes(), b"unclassified")


if __name__ == "__main__":
    unittest.main()
