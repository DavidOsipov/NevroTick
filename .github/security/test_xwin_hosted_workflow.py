"""Exercise the actual hosted shell with inert mount/acquisition commands."""
from pathlib import Path
import json
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
PROBE = '''#!/usr/bin/python3
import json,os,pathlib,sys
name=pathlib.Path(sys.argv[0]).name
args=sys.argv[1:]
mode=os.environ['PROBE_MODE']
with pathlib.Path(os.environ['PROBE_LOG']).open('a') as stream:
    stream.write(json.dumps({'program':name,'args':args})+'\\n')
if name=='sudo':
    if args[:2]==['mount','-t'] and (mode=='probe-mount' or (mode in ('mount','mount-ambiguous') and args[-1].endswith('/payload'))):sys.exit(17)
    if args[0]=='umount' and args[-1].endswith('/payload') and mode in ('unmount','setup-unmount'):sys.exit(29)
elif name=='python3':
    if '--assert-unmounted' in args and mode in ('mount-ambiguous','nested-left'):sys.exit(47)
    if 'scripts/setup-xwin.py' in args and mode in ('setup','setup-unmount'):sys.exit(37)
    if '.github/security/check_xwin_hosted_receipt.py' in args and mode=='readback':sys.exit(41)
    if '.github/security/check_xwin_quota.py' in args and '--assert-unmounted' not in args and mode=='quota':sys.exit(43)
else:sys.exit('Unexpected probe program')
sys.exit(0)
'''


def step():
    text = (ROOT / ".github/workflows/xwin-hosted.yml").read_text()
    body = text.split("      - name: Acquire and independently inspect the quota-bound SDK\n", 1)[1]
    body = body.split("        run: |\n", 1)[1]
    lines = []
    for line in body.splitlines():
        if line and not line.startswith("          "):
            break
        lines.append(line[10:])
    return "\n".join(lines) + "\n"


class HostedShellControls(unittest.TestCase):
    def run_step(self, mode):
        temporary = tempfile.TemporaryDirectory(prefix="xwin-workflow-controls-")
        self.addCleanup(temporary.cleanup)
        root = Path(temporary.name)
        binaries = root / "bin"
        binaries.mkdir()
        runner = root / "runner"
        runner.mkdir(mode=0o700)
        for name in ["sudo", "python3"]:
            executable = binaries / name
            executable.write_text(PROBE)
            executable.chmod(0o700)
        log = root / "calls.jsonl"
        environment = {"PATH": str(binaries) + ":/usr/bin:/bin", "LANG": "C.UTF-8",
                       "RUNNER_TEMP": str(runner), "GITHUB_WORKSPACE": str(ROOT),
                       "PROBE_MODE": mode, "PROBE_LOG": str(log)}
        result = subprocess.run(["/bin/bash", "--noprofile", "--norc", "-euo", "pipefail", "-c", step()],
                                cwd=ROOT, env=environment, stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=10)
        calls = [json.loads(line) for line in log.read_text().splitlines()] if log.exists() else []
        profiles = list(runner.iterdir())
        return result, calls, profiles

    @staticmethod
    def python_calls(calls):
        return [row["args"][1] for row in calls if row["program"] == "python3"
                and '--assert-unmounted' not in row["args"]]

    def test_healthy_shell_uses_tiny_control_then_exact_bounded_acquisition_and_readback(self):
        result, calls, profiles = self.run_step("healthy")
        self.assertEqual(result.returncode, 0, result.stderr.decode())
        self.assertEqual(self.python_calls(calls), [".github/security/check_xwin_quota.py",
                         "scripts/setup-xwin.py", ".github/security/check_xwin_hosted_receipt.py"])
        mounts = [row["args"] for row in calls if row["program"] == "sudo" and row["args"][:2] == ["mount", "-t"]]
        self.assertEqual(len(mounts), 2)
        self.assertIn("size=1m,nr_inodes=16,nodev,nosuid,noexec,mode=700", mounts[0][4])
        self.assertIn("size=2g,nr_inodes=65536,nodev,nosuid,noexec,mode=700", mounts[1][4])
        self.assertEqual(profiles, [])
        self.assertIn(b"XWIN_HOSTED_OWNED_CLEANUP_OK", result.stdout)

    def test_mount_refusal_prevents_acquisition_and_readback(self):
        for mode, expected_python in [("probe-mount", []), ("mount", [".github/security/check_xwin_quota.py"])]:
            with self.subTest(mode=mode):
                result, calls, profiles = self.run_step(mode)
                self.assertEqual(result.returncode, 17)
                self.assertEqual(self.python_calls(calls), expected_python)
                self.assertEqual(profiles, [])

    def test_quota_control_refusal_prevents_real_acquisition(self):
        result, calls, profiles = self.run_step("quota")
        self.assertEqual(result.returncode, 43)
        self.assertEqual(self.python_calls(calls), [".github/security/check_xwin_quota.py"])
        self.assertEqual(profiles, [])

    def test_failed_setup_never_reaches_receipt_consumer(self):
        result, calls, profiles = self.run_step("setup")
        self.assertEqual(result.returncode, 37)
        self.assertNotIn(".github/security/check_xwin_hosted_receipt.py", self.python_calls(calls))
        self.assertEqual(profiles, [])

    def test_receipt_refusal_preserves_failure_after_cleanup(self):
        result, calls, profiles = self.run_step("readback")
        self.assertEqual(result.returncode, 41)
        self.assertEqual(len(self.python_calls(calls)), 3)
        self.assertEqual(profiles, [])

    def test_failed_unmount_keeps_owned_tree_and_cannot_report_cleanup(self):
        for mode, status in [("unmount", 1), ("setup-unmount", 37)]:
            with self.subTest(mode=mode):
                result, calls, profiles = self.run_step(mode)
                self.assertEqual(result.returncode, status)
                self.assertEqual(len(profiles), 1)
                self.assertTrue((profiles[0] / "payload").is_dir())
                self.assertNotIn(b"XWIN_HOSTED_OWNED_CLEANUP_OK", result.stdout)
                self.assertTrue(all(row["args"][-1].startswith(str(profiles[0]) + "/")
                                    for row in calls if row["program"] == "sudo"))

    def test_ambiguous_or_nested_mount_state_prevents_recursive_deletion(self):
        for mode, status in [("mount-ambiguous", 17), ("nested-left", 1)]:
            with self.subTest(mode=mode):
                result, calls, profiles = self.run_step(mode)
                self.assertEqual(result.returncode, status)
                self.assertEqual(len(profiles), 1)
                self.assertTrue(any('--assert-unmounted' in row['args'] for row in calls))
                self.assertNotIn(b"XWIN_HOSTED_OWNED_CLEANUP_OK", result.stdout)

    def test_cleanup_observation_is_independent_of_the_primary_outcome(self):
        for mode, primary, complete, final in [("healthy", 0, True, 0), ("setup", 37, True, 37),
                                              ("unmount", 0, False, 1), ("setup-unmount", 37, False, 37)]:
            with self.subTest(mode=mode):
                result, _, profiles = self.run_step(mode)
                observations = [json.loads(line.split(" ", 1)[1]) for line in result.stdout.decode().splitlines()
                                if line.startswith("XWIN_HOSTED_CLEANUP ")]
                self.assertEqual(observations, [{"complete": complete, "primary_exit": primary, "final_exit": final}])
                self.assertEqual(result.returncode, final)
                self.assertEqual(not profiles, complete)


if __name__ == "__main__":
    unittest.main()
