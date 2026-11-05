"""Private solver-store admission controls; never execute Z3 or Kani."""
from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import stat
import subprocess
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch


SPEC = importlib.util.spec_from_file_location("solver_admission_test", Path(__file__).with_name("solver_admission.py"))
SOLVER = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(SOLVER)


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


class SolverAdmissionTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-private-solver-test-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.checkout = self.base / "checkout"
        (self.checkout / "policy").mkdir(parents=True)
        self.folder = self.base / "solver"
        self.folder.mkdir(mode=0o700)
        (self.folder / "bin").mkdir(mode=0o700)
        self.policy = copy.deepcopy(SOLVER.EXPECTED_POLICY)
        self.policy["default_root"] = str(self.folder)
        self.contents = {"bin/z3": b"fixture solver: never execute", "LICENSE.txt": b"fixture license"}
        for name, body in self.contents.items():
            self.policy["files"][name].update(bytes=len(body), sha256=sha(body))
        self.manifest = {
            "schema_version": 1, "status": "acquired-and-statically-inspected-not-executed",
            "executed": False, "version": "5.3.3", "asset_id": self.policy["release"]["asset_id"],
            "source_commit": self.policy["release"]["commit"], "source_url": self.policy["release"]["url"],
            "archive_verified": {key: self.policy["release"][key] for key in ("bytes", "sha256")},
            "payload_files": {name: {"bytes": len(body), "sha256": sha(body),
                                      "mode": oct(int(self.policy["files"][name]["mode"], 8)),
                                      "owner_uid": os.geteuid(), "hard_links": 1}
                              for name, body in self.contents.items()},
        }
        self.bind_manifest()
        self.write_policy()
        self.policy_patch = patch.object(SOLVER, "EXPECTED_POLICY", self.policy)
        self.policy_patch.start()
        self.addCleanup(self.policy_patch.stop)

    def bind_manifest(self):
        body = json.dumps(self.manifest).encode()
        self.contents["manifest.json"] = body
        self.policy["files"]["manifest.json"].update(bytes=len(body), sha256=sha(body))
        for name, data in self.contents.items():
            path = self.folder / name
            if path.exists():
                path.chmod(0o600)
            path.write_bytes(data)
            path.chmod(int(self.policy["files"][name]["mode"], 8))

    def write_policy(self):
        (self.checkout / SOLVER.POLICY).write_text(json.dumps(self.policy))

    def admit(self):
        return SOLVER.admit(self.checkout)

    def test_healthy_store_returns_only_exact_canonical_root_and_observed_hashes(self):
        with patch.object(subprocess, "run") as run, patch.object(subprocess, "Popen") as popen:
            result = self.admit()
        run.assert_not_called()
        popen.assert_not_called()
        self.assertEqual(result["readonly_root"], self.folder)
        self.assertEqual(result["executable"], self.folder / "bin/z3")
        self.assertEqual(result["path_dir"], self.folder / "bin")
        self.assertEqual((result["name"], result["version"]), ("z3", "5.3.3"))
        self.assertEqual(result["executable_sha256"], sha(self.contents["bin/z3"]))
        self.assertEqual(result["manifest_sha256"], sha(self.contents["manifest.json"]))
        self.assertEqual(set(result["inventory"]), set(self.contents))
        self.assertFalse(result["closure"]["per_command_library_hashes_checked"])
        self.assertFalse(result["executed"])
        self.assertEqual(SOLVER.admit(self.checkout, self.folder)["readonly_root"], self.folder)

    def test_every_payload_is_freshly_hashed_despite_same_length_and_restored_mtime(self):
        self.admit()
        for name, original in self.contents.items():
            path = self.folder / name
            before = path.stat()
            path.chmod(0o600)
            path.write_bytes(bytes([original[0] ^ 1]) + original[1:])
            path.chmod(int(self.policy["files"][name]["mode"], 8))
            os.utime(path, ns=(before.st_atime_ns, before.st_mtime_ns))
            with self.subTest(name=name), self.assertRaisesRegex(SOLVER.Incomplete, "SHA256"):
                self.admit()
            path.chmod(0o600)
            path.write_bytes(original)
            path.chmod(int(self.policy["files"][name]["mode"], 8))
            self.admit()

    def test_extra_root_and_binary_members_are_rejected(self):
        for relative in ("extra", ".hidden", "bin/z3-wrapper", "bin/.hidden"):
            path = self.folder / relative
            path.write_bytes(b"unadmitted member")
            with self.subTest(relative=relative), self.assertRaisesRegex(SOLVER.Incomplete, "membership"):
                self.admit()
            path.unlink()
            self.admit()

    def test_missing_symlink_fifo_and_hardlink_payloads_are_rejected(self):
        for name, original in self.contents.items():
            path = self.folder / name
            for fault in ("missing", "symlink", "fifo", "hardlink"):
                peer = self.base / "outside-alias"
                path.unlink()
                if fault == "symlink":
                    peer.write_bytes(original)
                    path.symlink_to(peer)
                elif fault == "fifo":
                    os.mkfifo(path)
                elif fault == "hardlink":
                    peer.write_bytes(original)
                    peer.chmod(int(self.policy["files"][name]["mode"], 8))
                    path.hardlink_to(peer)
                with self.subTest(name=name, fault=fault), \
                     self.assertRaises((SOLVER.Incomplete, OSError, RuntimeError)):
                    self.admit()
                path.unlink(missing_ok=True)
                peer.unlink(missing_ok=True)
                path.write_bytes(original)
                path.chmod(int(self.policy["files"][name]["mode"], 8))
                self.admit()

    def test_exact_file_and_directory_modes_are_required(self):
        modes = {**self.policy["directories"], **{n: p["mode"] for n, p in self.policy["files"].items()}}
        for name, expected in modes.items():
            path = self.folder / name
            path.chmod(int(expected, 8) | 0o022)
            with self.subTest(name=name), self.assertRaisesRegex(SOLVER.Incomplete, "mode"):
                self.admit()
            path.chmod(int(expected, 8))
            self.admit()

    def test_wrong_owners_are_rejected_for_directories_and_files(self):
        original_lstat = Path.lstat
        for name in ("", "bin", *self.contents):
            target = self.folder / name

            def wrong_owner(path, *, follow_symlinks=True):
                info = original_lstat(path)
                if path != target:
                    return info
                fields = {key: getattr(info, key) for key in
                          ("st_dev", "st_ino", "st_mode", "st_uid", "st_gid", "st_nlink",
                           "st_size", "st_mtime_ns", "st_ctime_ns")}
                fields["st_uid"] += 1
                return SimpleNamespace(**fields)

            with self.subTest(name=name), patch.object(Path, "lstat", wrong_owner), \
                 self.assertRaisesRegex(SOLVER.Incomplete, "owner"):
                self.admit()
        self.admit()

    def test_canonical_scope_and_repository_separation_are_required(self):
        alias = self.base / "solver-alias"
        alias.symlink_to(self.folder, target_is_directory=True)
        inside = self.checkout / "inside-solver"
        inside.mkdir(mode=0o700)
        for root in (alias, self.folder / "bin/..", Path("relative"), self.base, inside):
            with self.subTest(root=root), self.assertRaises((SOLVER.Incomplete, OSError)):
                SOLVER.admit(self.checkout, root)
        self.admit()

    def test_path_separators_and_controls_reject_before_any_file_hash_or_policy_read(self):
        for value in (str(self.folder) + ":extra", str(self.folder) + "\x01",
                      str(self.folder) + "\x7f", str(self.folder) + "\x85"):
            with self.subTest(value=repr(value)), \
                 patch.object(SOLVER.CUSTODY, "digest") as digest, \
                 patch.object(SOLVER.CUSTODY, "read_file") as read, \
                 self.assertRaisesRegex(SOLVER.Incomplete, "separator or control"):
                SOLVER.admit(self.checkout, value)
            digest.assert_not_called()
            read.assert_not_called()
        self.admit()

    def test_foreign_or_nonsticky_writable_ancestors_reject_before_hashing(self):
        original_lstat = Path.lstat
        for fault in ("owner", "writable"):
            def unsafe(path, *, follow_symlinks=True):
                info = original_lstat(path)
                if path != self.base:
                    return info
                fields = {key: getattr(info, key) for key in
                          ("st_dev", "st_ino", "st_mode", "st_uid", "st_gid", "st_nlink",
                           "st_size", "st_mtime_ns", "st_ctime_ns")}
                if fault == "owner":
                    fields["st_uid"] = os.geteuid() + 1
                else:
                    fields["st_mode"] |= 0o002
                return SimpleNamespace(**fields)

            with self.subTest(fault=fault), patch.object(Path, "lstat", unsafe), \
                 patch.object(SOLVER.CUSTODY, "digest") as digest, \
                 self.assertRaisesRegex(SOLVER.Incomplete, "ancestor"):
                self.admit()
            digest.assert_not_called()
        self.admit()

    def test_existing_reader_uses_nofollow_nonblocking_descriptors(self):
        original_open = os.open
        with patch.object(SOLVER.CUSTODY.os, "open", wraps=original_open) as opened:
            self.admit()
        self.assertGreaterEqual(opened.call_count, 6)
        for call in opened.call_args_list:
            self.assertEqual(call.args[1] & os.O_NOFOLLOW, os.O_NOFOLLOW)
            self.assertEqual(call.args[1] & os.O_NONBLOCK, os.O_NONBLOCK)

    def test_binary_directory_symlink_cannot_redirect_to_identical_bytes(self):
        original = self.folder / "bin"
        peer = self.base / "same-binary-directory"
        original.rename(peer)
        original.symlink_to(peer, target_is_directory=True)
        with self.assertRaisesRegex(SOLVER.Incomplete, "directory"):
            self.admit()
        original.unlink()
        peer.rename(original)
        self.admit()

    def test_fixed_policy_schema_pins_types_and_duplicate_keys_are_enforced(self):
        path = self.checkout / SOLVER.POLICY
        healthy = path.read_bytes()
        mutations = [lambda p: p.update(schema_version=True), lambda p: p.update(extra=True),
                     lambda p: p.update(version="other"), lambda p: p["release"].update(asset_id=True),
                     lambda p: p["files"]["bin/z3"].update(sha256="0" * 64),
                     lambda p: p["files"]["manifest.json"].update(bytes=True),
                     lambda p: p["directories"].update(bin="0755")]
        for mutate in mutations:
            candidate = copy.deepcopy(self.policy)
            mutate(candidate)
            path.write_text(json.dumps(candidate))
            with self.subTest(mutate=mutate), self.assertRaisesRegex(SOLVER.Incomplete, "policy"):
                self.admit()
        path.write_text('{"schema_version":1,"schema_version":1}')
        with self.assertRaisesRegex(SOLVER.Incomplete, "Duplicate JSON key"):
            self.admit()
        path.write_bytes(healthy)
        self.admit()

    def test_even_rebound_manifest_must_describe_the_pinned_release_and_payload(self):
        self.manifest["source_commit"] = "0" * 40
        self.bind_manifest()
        self.write_policy()
        with self.assertRaisesRegex(SOLVER.Incomplete, "provenance"):
            self.admit()

    def test_store_replacement_after_hashing_is_detected_by_final_identity_bookend(self):
        original = SOLVER.CUSTODY.digest
        target = self.folder / "LICENSE.txt"
        changed = False

        def replace(path, maximum):
            nonlocal changed
            result = original(path, maximum=maximum)
            if path == target and not changed:
                changed = True
                target.unlink()
                target.write_bytes(self.contents["LICENSE.txt"])
                target.chmod(0o400)
            return result

        with patch.object(SOLVER.CUSTODY, "digest", side_effect=replace), \
             self.assertRaisesRegex(SOLVER.Incomplete, "identity changed"):
            self.admit()
        self.admit()

    def test_policy_drift_after_payload_read_is_rejected(self):
        original = SOLVER.CUSTODY.digest
        policy_path = self.checkout / SOLVER.POLICY

        def drift(path, maximum):
            result = original(path, maximum=maximum)
            if path == self.folder / "bin/z3":
                policy_path.write_bytes(policy_path.read_bytes() + b" ")
            return result

        with patch.object(SOLVER.CUSTODY, "digest", side_effect=drift), \
             self.assertRaisesRegex(SOLVER.Incomplete, "policy changed"):
            self.admit()
        self.write_policy()
        self.admit()


if __name__ == "__main__":
    unittest.main()
