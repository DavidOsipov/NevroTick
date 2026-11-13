"""Challenge metadata acquisition custody without network, Cargo or executable fixtures."""
import contextlib
import fcntl
import hashlib
import importlib.util
import io
import json
import os
from pathlib import Path
import sys
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("refresh_product", ROOT / "scripts/refresh-product-advisories.py")
REFRESH = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(REFRESH)


class MetadataAcquisitionControls(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="product-refresh-control-")
        self.addCleanup(temporary.cleanup)
        self.base = Path(temporary.name)
        self.root = self.base / "checkout"
        (self.root / "policy").mkdir(parents=True)
        (self.root / ".cargo").mkdir()
        (self.root / ".cargo/config.toml").write_bytes((ROOT / ".cargo/config.toml").read_bytes())
        (self.root / "Cargo.lock").write_text("synthetic admitted lock bytes\n")
        self.toolchain = self.base / "toolchain"
        (self.toolchain / "bin").mkdir(parents=True)
        self.deny = self.base / "cargo-deny"
        self.deny.write_text("synthetic deny bytes; never executable")
        compiler_hashes = {}
        for name in ("rustc", "cargo"):
            path = self.toolchain / "bin" / name
            path.write_text("synthetic pinned " + name)
            compiler_hashes[name] = hashlib.sha256(path.read_bytes()).hexdigest()
        policy = {"tools": [{"name": "cargo-deny", "binary_sha256": hashlib.sha256(self.deny.read_bytes()).hexdigest()}],
                  "rust": {"linux_binary_sha256": compiler_hashes}}
        (self.root / "policy/tool-manifest.json").write_text(json.dumps(policy))
        self.home = self.base / "metadata"
        self.quota = self.base / "quota"
        self.quota.mkdir(mode=0o700)
        self.receipt = self.base / "receipt.json"
        self.arguments = ["refresh-product-advisories.py", "--cargo-home", str(self.home),
                          "--acquisition-root", str(self.quota),
                          "--receipt", str(self.receipt), "--deny", str(self.deny),
                          "--toolchain-dir", str(self.toolchain)]

    def acquisition(self, command, **kwargs):
        transient = Path(kwargs["environment"]["CARGO_HOME"])
        self.assertTrue(transient.is_relative_to(self.quota))
        self.assertFalse(self.home.exists(), "No final export before acquisition terminates")
        self.assertEqual(set(kwargs["environment"]), {"PATH", "CARGO_HOME", "RUSTC", "LANG", "TMPDIR"})
        self.assertTrue(Path(kwargs["environment"]["TMPDIR"]).is_relative_to(self.quota))
        if command == [str(self.toolchain / "bin/cargo"), "update", "--workspace", "--locked"]:
            name, output = "registry/index/cache", "synthetic index observation"
        else:
            self.assertEqual(command, [str(self.deny), "--locked", "fetch", "db"])
            self.assertTrue((transient / "registry/index/cache").is_file())
            name, output = "advisory-dbs/reviewed-db/advisory.toml", "synthetic database observation"
        path = transient / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text("controlled public metadata")
        return {"command": command, "exit_code": 0, "failure": None, "output": output}

    def invoke(self, effect=None, quota_error=None):
        with (patch.object(REFRESH, "ROOT", self.root), patch("sys.argv", self.arguments),
              patch.object(REFRESH.SNAPSHOT, "quota_root", side_effect=quota_error,
                           return_value={"bytes": 512 * 1024 * 1024, "inodes": 16384, "device": 1}),
              patch.object(REFRESH.SNAPSHOT, "run", side_effect=effect or self.acquisition) as run,
              contextlib.redirect_stdout(io.StringIO()) as output):
            status = REFRESH.main()
        return status, run, output.getvalue()

    def test_complete_acquisition_records_lock_tools_files_and_freshness(self):
        status, run, _ = self.invoke()
        self.assertEqual(status, 0)
        self.assertEqual(run.call_count, 2)
        record = json.loads(self.receipt.read_text())
        self.assertEqual(record["cargo_configuration"], {
            "cwd": str(self.root.resolve()),
            "config_sha256": hashlib.sha256((self.root / ".cargo/config.toml").read_bytes()).hexdigest(),
        })
        self.assertEqual(record["lockfile_sha256"], hashlib.sha256((self.root / "Cargo.lock").read_bytes()).hexdigest())
        self.assertEqual(record["tool_sha256"], hashlib.sha256(self.deny.read_bytes()).hexdigest())
        self.assertEqual(record["files"], REFRESH.inventory(self.home))
        self.assertGreater(record["acquired_unix"], 0)
        self.assertEqual(record["acquired_unix"], record["commands"][0]["started_unix"])
        self.assertGreaterEqual(record["completed_unix"], record["acquired_unix"])
        self.assertEqual([item["command"] for item in record["commands"]],
                         [call.args[0] for call in run.call_args_list])
        self.assertEqual([item["output"] for item in record["commands"]],
                         ["synthetic index observation", "synthetic database observation"])
        self.assertEqual(record["command"], [str(self.deny), "--locked", "fetch", "db"])
        self.assertEqual(record["output"], "synthetic database observation")
        self.assertEqual(record["cargo_sha256"], hashlib.sha256((self.toolchain / "bin/cargo").read_bytes()).hexdigest())
        self.assertEqual(self.receipt.stat().st_mode & 0o777, 0o600)

    def test_age_override_fails_before_tools_or_network_and_redacts_value(self):
        value = "synthetic-secret-age-value"
        with patch.dict(os.environ, {"CARGO_REGISTRY_GLOBAL_MIN_PUBLISH_AGE": value}):
            status, run, output = self.invoke()
        self.assertEqual(status, 1)
        run.assert_not_called()
        self.assertIn("CARGO_REGISTRY_GLOBAL_MIN_PUBLISH_AGE", output)
        self.assertNotIn(value, output)
        self.assertFalse(self.receipt.exists())

    def test_ancestor_registry_override_cannot_change_metadata_refresh_policy(self):
        ancestor = self.base / ".cargo"
        ancestor.mkdir()
        (ancestor / "config.toml").write_text('[registries.crates-io]\nmin-publish-age="0"\n')
        status, run, _ = self.invoke()
        self.assertEqual(status, 1)
        run.assert_not_called()

    def test_config_drift_during_metadata_acquisition_prevents_export(self):
        def changed(command, **kwargs):
            result = self.acquisition(command, **kwargs)
            path = self.root / ".cargo/config.toml"
            path.write_text(path.read_text().replace('"14 days"', '"0"'))
            return result
        status, run, _ = self.invoke(effect=changed)
        self.assertEqual(status, 1)
        self.assertEqual(run.call_count, 1)
        self.assertFalse(self.home.exists())
        self.assertFalse(self.receipt.exists())

    def test_separate_metadata_commands_refresh_index_without_fetching_sources(self):
        status, run, _ = self.invoke()
        self.assertEqual(status, 0)
        self.assertEqual([call.args[0] for call in run.call_args_list], [
            [str(self.toolchain / "bin/cargo"), "update", "--workspace", "--locked"],
            [str(self.deny), "--locked", "fetch", "db"],
        ])

    def test_successful_fetch_cannot_publish_crate_source_payload(self):
        def sources(command, **kwargs):
            result = self.acquisition(command, **kwargs)
            path = Path(kwargs["environment"]["CARGO_HOME"]) / "registry/src/unexpected/Cargo.toml"
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text("unexpected duplicate source body")
            return result
        status, run, output = self.invoke(sources)
        self.assertEqual(status, 1)
        self.assertIn("source or archive payload", output)
        run.assert_called_once()
        self.assertFalse(self.receipt.exists())
        self.assertFalse(self.home.exists())

    def test_substituted_tool_is_rejected_before_process_execution(self):
        self.deny.write_text("substituted executable")
        status, run, output = self.invoke()
        self.assertEqual(status, 1)
        self.assertIn("executable digest", output)
        run.assert_not_called()
        self.assertFalse(self.receipt.exists())

    def test_existing_snapshot_and_receipt_are_preserved(self):
        self.home.mkdir()
        sentinel = self.home / "existing"
        sentinel.write_text("preserve")
        self.receipt.write_text("preserve receipt")
        status, run, _ = self.invoke()
        self.assertEqual(status, 1)
        run.assert_not_called()
        self.assertEqual(sentinel.read_text(), "preserve")
        self.assertEqual(self.receipt.read_text(), "preserve receipt")

    def test_failed_acquisition_cannot_emit_success_receipt(self):
        def failure(command, **kwargs):
            return {"command": command, "exit_code": 17, "failure": None, "output": "controlled network failure"}
        status, run, output = self.invoke(failure)
        self.assertEqual(status, 1)
        self.assertIn("controlled network failure", output)
        run.assert_called_once()
        self.assertEqual(run.call_args.args[0],
                         [str(self.toolchain / "bin/cargo"), "update", "--workspace", "--locked"])
        self.assertFalse(self.receipt.exists())

    def test_second_stage_failure_does_not_publish_a_partial_index(self):
        def failure(command, **kwargs):
            if command[0] == str(self.deny):
                return {"command": command, "exit_code": 17, "failure": None,
                        "output": "controlled database failure"}
            return self.acquisition(command, **kwargs)
        status, run, output = self.invoke(failure)
        self.assertEqual(status, 1)
        self.assertIn("controlled database failure", output)
        self.assertEqual(run.call_count, 2)
        self.assertFalse(self.receipt.exists())
        self.assertFalse(self.home.exists())
        self.assertEqual(list(self.quota.iterdir()), [])

    def test_zero_database_exit_without_database_is_incomplete(self):
        def omitted(command, **kwargs):
            if command[0] == str(self.deny):
                return {"command": command, "exit_code": 0, "failure": None, "output": "no database"}
            return self.acquisition(command, **kwargs)
        status, run, output = self.invoke(omitted)
        self.assertEqual(status, 1)
        self.assertIn("omitted the RustSec database", output)
        self.assertEqual(run.call_count, 2)
        self.assertFalse(self.receipt.exists())

    def test_second_stage_cannot_publish_crate_archive_payload(self):
        def archive(command, **kwargs):
            result = self.acquisition(command, **kwargs)
            if command[0] == str(self.deny):
                path = Path(kwargs["environment"]["CARGO_HOME"]) / "registry/cache/unexpected.crate"
                path.parent.mkdir(parents=True)
                path.write_bytes(b"unneeded source archive")
            return result
        status, run, output = self.invoke(archive)
        self.assertEqual(status, 1)
        self.assertIn("source or archive payload", output)
        self.assertEqual(run.call_count, 2)
        self.assertFalse(self.receipt.exists())
        self.assertFalse(self.home.exists())

    def test_zero_exit_with_missing_required_snapshot_is_incomplete(self):
        def empty(command, **kwargs):
            return {"command": command, "exit_code": 0, "failure": None, "output": "no metadata"}
        status, _, output = self.invoke(empty)
        self.assertEqual(status, 1)
        self.assertIn("omitted", output)
        self.assertFalse(self.receipt.exists())

    def test_lock_drift_invalidates_successful_acquisition(self):
        def changed(command, **kwargs):
            result = self.acquisition(command, **kwargs)
            (self.root / "Cargo.lock").write_text("changed graph")
            return result
        status, _, output = self.invoke(changed)
        self.assertEqual(status, 1)
        self.assertIn("Lockfile changed", output)
        self.assertFalse(self.receipt.exists())

    def test_lock_drift_after_database_invalidates_both_stages(self):
        def changed(command, **kwargs):
            result = self.acquisition(command, **kwargs)
            if command[0] == str(self.deny):
                (self.root / "Cargo.lock").write_text("changed graph")
            return result
        status, run, output = self.invoke(changed)
        self.assertEqual(status, 1)
        self.assertIn("Lockfile changed", output)
        self.assertEqual(run.call_count, 2)
        self.assertFalse(self.receipt.exists())
        self.assertFalse(self.home.exists())

    def test_drift_between_stage_checks_prevents_database_invocation(self):
        original = REFRESH.inventory
        def changed(directory):
            files = original(directory)
            (self.root / "Cargo.lock").write_text("changed between stage checks")
            return files
        with patch.object(REFRESH, "inventory", side_effect=changed):
            status, run, output = self.invoke()
        self.assertEqual(status, 1)
        self.assertIn("Lockfile changed before", output)
        run.assert_called_once()
        self.assertFalse(self.receipt.exists())

    def test_linked_metadata_is_not_followed_into_receipt(self):
        def linked(command, **kwargs):
            result = self.acquisition(command, **kwargs)
            (Path(kwargs["environment"]["CARGO_HOME"]) / "unexpected").symlink_to(self.root / "Cargo.lock")
            return result
        status, _, output = self.invoke(linked)
        self.assertEqual(status, 1)
        self.assertIn("linked or special", output)
        self.assertFalse(self.receipt.exists())

    def test_ambient_credentials_do_not_reach_acquisition(self):
        with patch.dict(os.environ, {"NEVROTICK_SYNTHETIC_CREDENTIAL": "never-forward"}):
            status, run, _ = self.invoke()
        self.assertEqual(status, 0)
        self.assertNotIn("NEVROTICK_SYNTHETIC_CREDENTIAL", run.call_args.kwargs["environment"])

    def test_unbounded_acquisition_filesystem_refuses_before_execution(self):
        status, run, output = self.invoke(quota_error=ValueError("filesystem byte or inode capacity"))
        self.assertEqual(status, 1)
        self.assertIn("capacity", output)
        run.assert_not_called()
        self.assertFalse(self.home.exists())
        self.assertFalse(self.receipt.exists())

    def test_failed_export_removes_only_the_new_partial_snapshot(self):
        unrelated = self.base / "preserve"
        unrelated.write_bytes(b"unrelated")
        def failed(source, destination, expected):
            (destination / "partial").write_bytes(b"partial")
            raise ValueError("controlled export failure")
        with patch.object(REFRESH.SNAPSHOT, "export", side_effect=failed):
            status, run, output = self.invoke()
        self.assertEqual(status, 1)
        self.assertIn("controlled export failure", output)
        self.assertEqual(run.call_count, 2)
        self.assertFalse(self.home.exists())
        self.assertFalse(self.receipt.exists())
        self.assertEqual(list(self.quota.iterdir()), [])
        self.assertEqual(unrelated.read_bytes(), b"unrelated")

    def test_failed_fetch_clears_only_owned_transient_metadata(self):
        def failed(command, **kwargs):
            self.acquisition(command, **kwargs)
            return {"command": command, "exit_code": 17, "failure": None, "output": "refusal"}
        status, _, _ = self.invoke(failed)
        self.assertEqual(status, 1)
        self.assertEqual(list(self.quota.iterdir()), [])
        self.assertFalse(self.home.exists())
        self.assertFalse(self.receipt.exists())

    def test_cleanup_failure_retains_the_first_fetch_failure(self):
        def failed(command, **kwargs):
            return {"command": command, "exit_code": 17, "failure": None,
                    "output": "first controlled fetch failure"}
        def cleanup_failure(directory):
            # Actual TemporaryDirectory cleanup detaches this finalizer before
            # removal; preserve that behavior in the controlled removal fault.
            directory._finalizer.detach()
            raise OSError("secondary controlled cleanup failure")
        with patch.object(REFRESH.tempfile.TemporaryDirectory, "cleanup", autospec=True,
                          side_effect=cleanup_failure):
            status, _, output = self.invoke(failed)
        self.assertEqual(status, 1)
        self.assertIn("first controlled fetch failure", output)
        self.assertIn("secondary controlled cleanup failure", output)
        self.assertFalse(self.receipt.exists())
        descriptor = os.open(self.quota, os.O_RDONLY | os.O_DIRECTORY)
        try:
            fcntl.flock(descriptor, fcntl.LOCK_EX | fcntl.LOCK_NB)
        finally:
            os.close(descriptor)

    def test_oversized_snapshot_file_is_refused_before_content_reads(self):
        self.home.mkdir()
        oversized = self.home / "too-large"
        with oversized.open("wb") as stream:
            stream.truncate(64 * 1024 * 1024 + 1)
        # The sparse extent proves metadata refusal without allocating its body.
        with patch.object(Path, "read_bytes", return_value=b"not-read") as read:
            with self.assertRaisesRegex(ValueError, "byte bound"):
                REFRESH.inventory(self.home)
        read.assert_not_called()

    def test_snapshot_depth_is_refused_before_content_reads(self):
        leaf = self.home
        for _ in range(17):
            leaf = leaf / "nested"
        leaf.mkdir(parents=True)
        (leaf / "record").write_bytes(b"bounded synthetic metadata")
        with self.assertRaisesRegex(ValueError, "depth bound"):
            REFRESH.inventory(self.home)


class SnapshotBoundaryControls(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="product-metadata-boundary-")
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.snapshot = self.root / "snapshot"
        self.snapshot.mkdir()
        self.owner = REFRESH.SNAPSHOT

    def test_small_stable_tree_hashes_full_contents(self):
        (self.snapshot / "a").write_bytes(b"alpha")
        self.assertEqual(self.owner.inventory(self.snapshot), {"a": hashlib.sha256(b"alpha").hexdigest()})

    def test_entry_and_aggregate_bounds_precede_content_hashing(self):
        (self.snapshot / "a").write_bytes(b"alpha")
        (self.snapshot / "b").write_bytes(b"beta")
        for constant, maximum in (("MAX_ENTRIES", 1), ("MAX_TOTAL_BYTES", 8)):
            with self.subTest(constant=constant), patch.object(self.owner, constant, maximum):
                with patch.object(self.owner.CUSTODY, "digest") as digest:
                    with self.assertRaisesRegex(ValueError, "bound"):
                        self.owner.inventory(self.snapshot)
                    digest.assert_not_called()

    def test_path_budget_precedes_content_hashing(self):
        (self.snapshot / "too-long").write_bytes(b"x")
        with patch.object(self.owner, "MAX_PATH_BYTES", 4):
            with self.assertRaisesRegex(ValueError, "path byte bound"):
                self.owner.inventory(self.snapshot)

    def test_fifo_and_linked_directory_are_refused_without_opening_contents(self):
        os.mkfifo(self.snapshot / "fifo")
        with self.assertRaisesRegex(ValueError, "linked or special"):
            self.owner.inventory(self.snapshot)
        (self.snapshot / "fifo").unlink()
        (self.snapshot / "directory").symlink_to(self.root, target_is_directory=True)
        with self.assertRaisesRegex(ValueError, "linked or special"):
            self.owner.inventory(self.snapshot)

    def test_growth_or_replacement_during_body_read_is_refused(self):
        (self.snapshot / "a").write_bytes(b"alpha")
        original = self.owner.CUSTODY.digest
        def mutation(path, maximum):
            (self.snapshot / "later").write_bytes(b"later")
            return original(path, maximum)
        with patch.object(self.owner.CUSTODY, "digest", side_effect=mutation):
            with self.assertRaisesRegex(ValueError, "changed during hashing"):
                self.owner.inventory(self.snapshot)

    def test_receipt_raw_bound_and_duplicate_fields(self):
        receipt = self.root / "receipt"
        receipt.write_bytes(b"123456789")
        with patch.object(self.owner, "MAX_RECEIPT_BYTES", 8):
            with self.assertRaisesRegex(ValueError, "byte bound"):
                self.owner.read_receipt(receipt)
        receipt.write_text('{"files":{},"files":{}}')
        with self.assertRaisesRegex(ValueError, "Duplicate"):
            self.owner.read_receipt(receipt)

    def test_quota_requires_dedicated_backing_capacity(self):
        self.snapshot.chmod(0o700)
        with self.assertRaisesRegex(ValueError, "dedicated quota filesystem"):
            self.owner.quota_root(self.snapshot)

    def test_quota_uses_total_capacity_not_a_small_free_space_sample(self):
        self.snapshot.chmod(0o700)
        original = Path.stat
        def distinct_parent(path, *args, **kwargs):
            observed = original(path, *args, **kwargs)
            if path == self.snapshot.parent:
                return SimpleNamespace(st_dev=observed.st_dev + 1)
            return observed
        cases = (
            (self.owner.MAX_TOTAL_BYTES, self.owner.MAX_ENTRIES, True),
            (self.owner.MAX_TOTAL_BYTES + 1, self.owner.MAX_ENTRIES, False),
            (self.owner.MAX_TOTAL_BYTES, self.owner.MAX_ENTRIES + 1, False),
            (0, self.owner.MAX_ENTRIES, False),
            (self.owner.MAX_TOTAL_BYTES, 0, False),
        )
        for byte_capacity, inode_capacity, accepted in cases:
            capacity = SimpleNamespace(f_frsize=1, f_blocks=byte_capacity,
                                       f_files=inode_capacity, f_bavail=1, f_favail=1)
            with (self.subTest(bytes=byte_capacity, inodes=inode_capacity),
                  patch.object(Path, "stat", distinct_parent),
                  patch.object(self.owner.os, "statvfs", return_value=capacity)):
                if accepted:
                    self.assertEqual(self.owner.quota_root(self.snapshot)["bytes"], byte_capacity)
                else:
                    with self.assertRaisesRegex(ValueError, "capacity"):
                        self.owner.quota_root(self.snapshot)


class AcquisitionSupervisorControls(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="product-metadata-supervisor-")
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.owner = REFRESH.SNAPSHOT

    def run_child(self, code):
        return self.owner.run([sys.executable, "-B", "-c", code], cwd=self.root,
                              environment={"PATH": os.defpath, "LANG": "C.UTF-8"})

    def test_healthy_command_is_collected_with_actual_exit(self):
        result = self.run_child('print("healthy")')
        self.assertEqual(result["exit_code"], 0)
        self.assertIsNone(result["failure"])
        self.assertEqual(result["output"], "healthy\n")

    def test_output_budget_refuses_without_retaining_oversized_chunk(self):
        with patch.object(self.owner, "MAX_OUTPUT_BYTES", 8):
            result = self.run_child('print("x" * 32)')
        self.assertEqual(result["failure"], "output budget exceeded")
        self.assertLessEqual(len(result["output"]), 8)

    def test_timeout_kills_and_reaps_owned_command(self):
        with patch.object(self.owner, "MAX_SECONDS", 1):
            result = self.run_child('import time; time.sleep(10)')
        self.assertEqual(result["failure"], "wall timeout")
        self.assertNotEqual(result["exit_code"], 0)

    def test_per_file_hard_limit_reaches_real_write(self):
        with patch.object(self.owner, "MAX_FILE_BYTES", 8):
            result = self.run_child('from pathlib import Path; Path("bounded").write_bytes(b"x" * 32)')
        self.assertNotEqual(result["exit_code"], 0)
        self.assertLessEqual((self.root / "bounded").stat().st_size, 8)

    def test_live_descendant_after_leader_exit_is_a_failed_group(self):
        result = self.run_child(
            'import os,time; p=os.fork(); '
            'os._exit(0) if p else None; os.close(1); os.close(2); time.sleep(10)')
        self.assertEqual(result["exit_code"], 0)
        self.assertEqual(result["failure"], "descendant processes outlived command")


if __name__ == "__main__":
    unittest.main()
