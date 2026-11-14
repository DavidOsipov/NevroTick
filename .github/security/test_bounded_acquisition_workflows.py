"""Execute real acquisition step shells with inert download/extraction probes."""
from pathlib import Path
import json
import subprocess
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]


def workflow_step(file, title):
    """Select one named step without evaluating workflow expressions."""
    text = (ROOT / file).read_text()
    return text.split("      - name: " + title + "\n", 1)[1].split("\n      - name: ", 1)[0]


def acquisition_step(file, title):
    """Extract one literal run block; do not evaluate workflow expressions."""
    section = workflow_step(file, title)
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
    def run_product_step(self, title, exit_code=0):
        """Exercise the actual shell with a recorder instead of the Rust runner."""
        file = ".github/workflows/rust.yml"
        section = workflow_step(file, title)
        raw_environment = dict(line.strip().split(": ", 1) for line in
                               section.split("        env:\n", 1)[1].split("        run:", 1)[0].splitlines())
        with tempfile.TemporaryDirectory(prefix="product-profile-shell-") as name:
            base = Path(name)
            probe = base / "python3"
            probe.write_text(
                "#!/usr/bin/python3\nimport json,os,sys\n"
                "with open(os.environ['PROBE_LOG'],'a') as output:\n"
                " json.dump({'args':sys.argv[1:],'rustup':os.environ.get('RUSTUP_TOOLCHAIN'),"
                "'flags':os.environ.get('RUSTFLAGS')},output);output.write('\\n')\n"
                "sys.exit(int(os.environ['PROBE_EXIT']))\n"
            )
            probe.chmod(0o700)
            environment = {"PATH": str(base) + ":/usr/bin:/bin", "RUSTUP_HOME": str(base / "rustup"),
                           "RUSTUP_TOOLCHAIN": "ambient-manager-selection", "RUSTFLAGS": "must-remain-visible",
                           "PROBE_LOG": str(base / "calls.jsonl"), "PROBE_EXIT": str(exit_code)}
            environment.update({key: value.replace("${{ runner.temp }}", str(base))
                                for key, value in raw_environment.items()})
            run = subprocess.run(["/bin/bash", "--noprofile", "--norc", "-euo", "pipefail", "-c",
                                  acquisition_step(file, title)], cwd=base, env=environment,
                                 stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=10)
            calls = [json.loads(line) for line in (base / "calls.jsonl").read_text().splitlines()]
            return run, calls, environment, raw_environment

    def test_product_rows_route_default_and_ke_profiles_to_exact_fresh_inputs(self):
        rows = (
            ("Check the frozen dependency graph before package code executes", "dependencies", None,
             "ui-m0-dependencies.json"),
            ("Verify actual portable product code in the isolated row", "portable", None,
             "ui-m0-portable.json"),
            ("Check the explicit KE dependency graph before its package code executes", "dependencies",
             "engine-ke-association", "engine-ke-association-dependencies.json"),
            ("Verify actual portable KE ownership in the isolated explicit profile", "portable",
             "engine-ke-association", "engine-ke-association-portable.json"),
        )
        for title, lane, profile, receipt in rows:
            with self.subTest(step=title):
                run, calls, environment, raw_environment = self.run_product_step(title)
                self.assertEqual(run.returncode, 0, run.stderr.decode())
                expected_environment = {"PRODUCT_SCRATCH": "${{ runner.temp }}",
                                        "VENDOR_DIR": "${{ runner.temp }}/ui-m0-vendor"}
                expected = ["-B", "scripts/verify.py", "--lane", lane]
                if profile is not None:
                    expected += ["--profile", profile]
                expected += ["--toolchain-dir", environment["RUSTUP_HOME"] +
                             "/toolchains/1.100.0-x86_64-unknown-linux-gnu",
                             "--vendor-dir", environment["VENDOR_DIR"],
                             "--scratch-root", environment["PRODUCT_SCRATCH"]]
                if lane == "dependencies":
                    expected_environment.update(METADATA_RECEIPT="${{ runner.temp }}/ui-m0-metadata.json",
                                                DENY_BINARY="${{ runner.temp }}/deny-tool/cargo-deny",
                                                DEPENDENCY_RECEIPT="${{ runner.temp }}/" + receipt)
                    expected += ["--deny", environment["DENY_BINARY"], "--advisory-receipt",
                                 environment["METADATA_RECEIPT"], "--json", environment["DEPENDENCY_RECEIPT"]]
                else:
                    expected_environment.update(NEXTEST_BINARY="${{ runner.temp }}/nextest-tool/cargo-nextest",
                                                PRODUCT_RECEIPT="${{ runner.temp }}/" + receipt)
                    expected += ["--nextest", environment["NEXTEST_BINARY"], "--json", environment["PRODUCT_RECEIPT"]]
                self.assertEqual(raw_environment, expected_environment)
                self.assertEqual(calls, [{"args": expected, "rustup": None, "flags": "must-remain-visible"}])

    def test_ke_rows_propagate_refusal_and_remain_required_after_acquisition(self):
        dependency = "Check the explicit KE dependency graph before its package code executes"
        portable = "Verify actual portable KE ownership in the isolated explicit profile"
        workflow = (ROOT / ".github/workflows/rust.yml").read_text()
        self.assertLess(workflow.index("Acquire the reviewed vendor bytes without executing package code"),
                        workflow.index(dependency))
        self.assertLess(workflow.index("Acquire current advisory and registry metadata separately"),
                        workflow.index(dependency))
        self.assertLess(workflow.index(dependency), workflow.index(portable))
        self.assertNotIn("continue-on-error:", workflow)
        for title in (dependency, portable):
            with self.subTest(step=title):
                self.assertNotIn("        if:", workflow_step(".github/workflows/rust.yml", title))
                run, calls, _, _ = self.run_product_step(title, exit_code=29)
                self.assertEqual(run.returncode, 29, run.stderr.decode())
                self.assertEqual(len(calls), 1)

    def test_profile_admission_controls_run_without_expanding_hosted_authority(self):
        with tempfile.TemporaryDirectory(prefix="profile-admission-shell-") as name:
            base = Path(name)
            probe = base / "python3"
            probe.write_text("#!/usr/bin/python3\nimport json,os,sys\n"
                             "with open(os.environ['PROBE_LOG'],'a') as output:\n"
                             " json.dump(sys.argv[1:],output);output.write('\\n')\n")
            probe.chmod(0o700)
            run = subprocess.run(["/bin/bash", "--noprofile", "--norc", "-euo", "pipefail", "-c",
                                  acquisition_step(".github/workflows/rust.yml",
                                                   "Challenge product admission and result accounting")],
                                 cwd=base, env={"PATH": str(base) + ":/usr/bin:/bin",
                                                "PROBE_LOG": str(base / "calls.jsonl")},
                                 stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=10)
            self.assertEqual(run.returncode, 0, run.stderr.decode())
            calls = [json.loads(line) for line in (base / "calls.jsonl").read_text().splitlines()]
            expected = ["-X", "dev", "-Werror", "-B", "-m", "unittest", "discover", "-s", "scripts/tests",
                        "-p", "test_engine_ke_dependency_profile.py", "-v"]
            self.assertEqual(calls.count(expected), 1)
        workflow = (ROOT / ".github/workflows/rust.yml").read_text()
        self.assertIn("    name: UI-M0 portable model\n    runs-on: ubuntu-24.04\n    timeout-minutes: 35\n"
                      "    permissions:\n      contents: read\n", workflow)
        self.assertNotIn("--all-features", workflow)
        self.assertNotIn("--lane windows", workflow)

    def test_facade_step_removes_only_rustup_selection_before_pinned_direct_execution(self):
        with tempfile.TemporaryDirectory(prefix="facade-ci-environment-") as name:
            root = Path(name)
            probe = root / "python3"
            probe.write_text(
                "#!/usr/bin/python3\nimport os,sys\n"
                "assert 'RUSTUP_TOOLCHAIN' not in os.environ, 'manager selection reached direct runner'\n"
                "assert os.environ['RUSTFLAGS']=='rejection-control-remains-visible'\n"
                "assert '--toolchain-dir' in sys.argv\n"
                "assert sys.argv[sys.argv.index('--toolchain-dir')+1]=='/pinned-rustup/toolchains/1.100.0-x86_64-unknown-linux-gnu'\n"
            )
            probe.chmod(0o700)
            environment = {"PATH": str(root) + ":/usr/bin:/bin", "RUSTUP_TOOLCHAIN": "1.99.1",
                           "RUSTUP_HOME": "/pinned-rustup", "RUSTFLAGS": "rejection-control-remains-visible",
                           "ACCESSKIT_CONTROLS": str(root / "result")}
            run = subprocess.run(["/bin/bash", "--noprofile", "--norc", "-euo", "pipefail", "-c",
                                  acquisition_step(".github/workflows/rust.yml",
                                      "Exercise the actual AccessKit subclass through the portable facade")],
                                 cwd=root, env=environment, stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=10)
            self.assertEqual(run.returncode, 0, run.stderr.decode())

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
