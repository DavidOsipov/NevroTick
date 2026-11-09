"""Execute real acquisition step shells with inert download/extraction probes."""
from pathlib import Path
import json
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]


def acquisition_step(file, title):
    """Extract one literal run block; do not evaluate workflow expressions."""
    text = (ROOT / file).read_text()
    section = text.split("      - name: " + title + "\n", 1)[1]
    body = section.split("        run: |\n", 1)[1]
    lines = []
    for line in body.splitlines():
        if line and not line.startswith("          "):
            break
        lines.append(line[10:])
    return "\n".join(lines) + "\n"


PROBE = '''#!/usr/bin/python3
import json,os,pathlib,sys
name=pathlib.Path(sys.argv[0]).name
log=pathlib.Path(os.environ['PROBE_LOG'])
rows=[json.loads(x) for x in log.read_text().splitlines()] if log.exists() else []
row={'program':name,'args':sys.argv[1:]}
with log.open('a') as f:f.write(json.dumps(row)+'\\n')
if name=='timeout':
    args=sys.argv[1:]
    if '--maximum-bytes' not in args or '--sha256' not in args or '--allowed-origin' not in args:
        sys.exit(19)
    count=sum(x['program']=='timeout' for x in rows)
    mode=os.environ['PROBE_MODE']
    if mode=='refuse' or (mode=='recover' and count<2):sys.exit(17)
elif name=='sha256sum':
    sys.stdin.read()
elif name=='python3':
    sys.exit('Unexpected direct Python route; downloads need the outer deadline.')
sys.exit(0)
'''


class WorkflowAcquisitionControls(unittest.TestCase):
    def run_step(self, file, title, mode):
        with tempfile.TemporaryDirectory(prefix="bounded-acquisition-shell-") as name:
            base = Path(name)
            binaries = base / "bin"
            binaries.mkdir()
            for program in ("timeout", "curl", "python3", "sha256sum", "tar", "mkdir", "sleep"):
                path = binaries / program
                path.write_text(PROBE)
                path.chmod(0o700)
            environment = {"PATH": str(binaries), "GITHUB_WORKSPACE": str(ROOT),
                           "PROBE_LOG": str(base / "calls.jsonl"), "PROBE_MODE": mode,
                           "LANG": "C.UTF-8"}
            run = subprocess.run(["/bin/bash", "--noprofile", "--norc", "-euo", "pipefail", "-c",
                                  acquisition_step(file, title)], cwd=base, env=environment,
                                 stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=10)
            calls = [json.loads(x) for x in (base / "calls.jsonl").read_text().splitlines()]
            return run, calls

    def test_refused_download_stops_before_verification_or_extraction(self):
        for file, title in (
            (".github/workflows/rust.yml", "Acquire pinned test and dependency tools and verify before execution"),
            (".github/workflows/security.yml", "Acquire and verify the fixed CodeQL bundle before execution"),
            (".github/workflows/security.yml", "Acquire and verify Gitleaks before execution"),
        ):
            with self.subTest(step=title):
                run, calls = self.run_step(file, title, "refuse")
                self.assertNotEqual(run.returncode, 0, run.stderr.decode())
                self.assertEqual(sum(x["program"] == "timeout" for x in calls), 3)
                self.assertFalse(any(x["program"] in ("curl", "tar", "sha256sum") for x in calls))

    def test_healthy_real_steps_pass_exact_pins_budgets_and_origins(self):
        expected = (
            (".github/workflows/rust.yml", "Acquire pinned test and dependency tools and verify before execution",
             ("33554432", "33554432")),
            (".github/workflows/security.yml", "Acquire and verify the fixed CodeQL bundle before execution",
             ("1073741824",)),
            (".github/workflows/security.yml", "Acquire and verify Gitleaks before execution", ("33554432",)),
        )
        for file, title, budgets in expected:
            with self.subTest(step=title):
                run, calls = self.run_step(file, title, "healthy")
                self.assertEqual(run.returncode, 0, run.stderr.decode())
                downloads = [x["args"] for x in calls if x["program"] == "timeout"]
                self.assertEqual(tuple(x[x.index("--maximum-bytes") + 1] for x in downloads), budgets)
                self.assertFalse(any(x["program"] == "curl" for x in calls))
                for args in downloads:
                    self.assertIn("--kill-after=5s", args)
                    self.assertIn(str(ROOT / "scripts/bounded_download.py"), args)
                    self.assertIn("https://github.com", args)
                    self.assertIn("https://release-assets.githubusercontent.com", args)
                    self.assertRegex(args[args.index("--sha256") + 1], r"^[0-9a-f]{64}$")

    def test_transient_failure_has_finite_retry_and_useful_success(self):
        run, calls = self.run_step(".github/workflows/security.yml",
                                   "Acquire and verify Gitleaks before execution", "recover")
        self.assertEqual(run.returncode, 0, run.stderr.decode())
        self.assertEqual(sum(x["program"] == "timeout" for x in calls), 3)
        self.assertEqual(sum(x["program"] == "tar" for x in calls), 1)


if __name__ == "__main__":
    unittest.main()
