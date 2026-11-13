"""Real admitted Cargo publication-age controls beside documented exceptions.

Reuse the existing sparse-registry sandbox and inert archive generator. Each
finite group runs its HTTP peer and Cargo inside one isolated network namespace.
These metadata-only cases never execute or download dependency package code.
"""
from __future__ import annotations

from datetime import datetime, timedelta, timezone
import hashlib
import http.server
import importlib.util
import json
import os
from pathlib import Path
import shutil
import sys
import tempfile
import threading
import tomllib
import unittest


def load(name, path):
    specification = importlib.util.spec_from_file_location(name, path)
    if specification is None or specification.loader is None:
        raise RuntimeError("Required pinned cooldown fixture helper is unavailable.")
    module = importlib.util.module_from_spec(specification)
    sys.modules[name] = module
    specification.loader.exec_module(module)
    return module


ADVISORY = load("cooldown_existing_sparse_fixture", Path(__file__).with_name("test_advisory_cargo_refresh.py"))
DIRECT, UPSTREAM, LEAF, ONLY, MISSING = (
    "cooldown-direct", "cooldown-upstream", "cooldown-leaf", "cooldown-only", "cooldown-missing")
POLICY = '[registry]\nglobal-min-publish-age = "14 days"\n[resolver]\nincompatible-publish-age = "deny"\n'
AGE_ENV = "CARGO_RESOLVER_INCOMPATIBLE_PUBLISH_AGE"


def cooldown_driver(host_network_namespace, group):
    """Observe selection/refusal and its limits using bounded local metadata."""
    assert group in {"selection", "exceptions"}
    assert os.readlink("/proc/self/ns/net") != host_network_namespace
    assert AGE_ENV not in os.environ
    environment = load("isolated_cooldown_environment", Path("/probe/check-environment.py"))
    cooldown = load("isolated_cooldown_resolution_owner", Path("/probe/dependency_cooldown.py"))
    assert cooldown.APPROVED_EXCEPTIONS == {}
    started = datetime.now(timezone.utc)
    stamp = lambda age: (started - timedelta(days=age)).isoformat(timespec="seconds").replace("+00:00", "Z")
    old, young = stamp(15), stamp(13)
    catalog = {}
    for name, versions in ((DIRECT, ("1.0.0", "1.1.0")), (UPSTREAM, ("1.0.0", "1.1.0")),
                           (LEAF, ("1.0.0", "2.0.0")), (ONLY, ("1.0.0",)),
                           (MISSING, ("1.0.0", "1.1.0"))):
        for version in versions:
            archive = ADVISORY.crate(version, name=name)
            assert len(archive) < 16384
            dependency = ([] if name != UPSTREAM else [{"name": LEAF,
                "req": "1" if version == "1.0.0" else "2", "features": [], "optional": False,
                "default_features": True, "target": None, "kind": "normal", "registry": None}])
            row = {"name": name, "vers": version, "deps": dependency,
                   "cksum": hashlib.sha256(archive).hexdigest(), "features": {}, "yanked": False}
            if name != MISSING:
                row["pubtime"] = young if (name, version) in {
                    (DIRECT, "1.1.0"), (LEAF, "2.0.0"), (ONLY, "1.0.0")} else old
            catalog.setdefault(name, []).append(row)
    visible = {name: list(rows) for name, rows in catalog.items()}
    requests, phases, server_errors = [], [], []
    def identity(name, version):
        row = next(row for row in catalog[name] if row["vers"] == version)
        return {"name": name, "version": version, "source": cooldown.REGISTRY, "checksum": row["cksum"]}
    expected_delta = {"added": [identity(LEAF, "2.0.0"), identity(UPSTREAM, "1.1.0")],
                      "removed": [identity(LEAF, "1.0.0"), identity(UPSTREAM, "1.0.0")]}
    urgent_exception = {"decision_id": "FIXTURE-OD04-URGENT-ONCE", "decision_ref": "fixture://cooldown/one-child",
        "decision_sha256": hashlib.sha256(b"inert fixture approval, never a product exception").hexdigest(),
        "requested_package": UPSTREAM, "requested_version": "1.1.0", "reason": "Test-only urgent child boundary",
        "primary_fix_evidence": "https://fixture.invalid/verified-fix", "targets": ["x86_64-unknown-linux-gnu"],
        "features": [], "delta_sha256": cooldown.canonical(expected_delta)}

    class Registry(http.server.BaseHTTPRequestHandler):
        def setup(self):
            super().setup()
            self.connection.settimeout(2)

        def log_message(self, *_):
            pass

        def do_GET(self):
            if len(requests) >= 128 or len(self.path) > 256:
                server_errors.append("request budget exceeded")
                self.send_error(429)
                return
            requests.append(self.path)
            body = None
            if self.path == "/index/config.json":
                body = json.dumps({"dl": base + "/crate/{crate}/{version}.crate"}).encode()
            else:
                for name, rows in visible.items():
                    if self.path == "/index/" + name[:2] + "/" + name[2:4] + "/" + name:
                        body = b"".join(json.dumps(row).encode() + b"\n" for row in rows)
                        break
            assert body is None or len(body) < 16384
            self.send_response(200 if body is not None else 404)
            body = body if body is not None else b"unadmitted fixture path"
            self.send_header("Content-Length", str(len(body)))
            self.send_header("Connection", "close")
            self.end_headers()
            self.wfile.write(body)

    with http.server.HTTPServer(("127.0.0.1", 0), Registry) as server:
        base = f"http://127.0.0.1:{server.server_port}"
        worker = threading.Thread(target=server.serve_forever, kwargs={"poll_interval": 0.05}, daemon=True)
        worker.start()
        try:
            def fixture(label, name, requirement, *, alternate=False, shorter=False):
                directory = Path("/work/fixture") / label
                (directory / "src").mkdir(parents=True)
                (directory / ".cargo").mkdir()
                dependency = (json.dumps(requirement) if not alternate else
                              '{ version = ' + json.dumps(requirement) + ', registry = "controlled" }')
                (directory / "Cargo.toml").write_text(
                    '[workspace]\nmembers = ["."]\nresolver = "3"\n[package]\nname = "cooldown-control"\n'
                    'version = "0.0.0"\nedition = "2024"\npublish = false\n'
                    f'[dependencies]\n{name} = {dependency}\n')
                (directory / "src/lib.rs").write_text("pub fn inert_fixture() {}\n")
                configuration = POLICY
                if alternate:
                    configuration += '[registries.controlled]\nindex = "sparse+' + base + '/index/"\n'
                    if shorter:
                        configuration += 'min-publish-age = "0"\n'
                else:
                    configuration += '[source.crates-io]\nreplace-with = "controlled-sparse"\n'
                    configuration += '[source.controlled-sparse]\nregistry = "sparse+' + base + '/index/"\n'
                (directory / ".cargo/config.toml").write_text(configuration)
                return directory

            def cargo(label, directory, arguments, *, allow=False, refusal=False, home=None, offline=False):
                selected_home = home or Path("/work") / (label + "-cargo-home")
                if home is None:
                    selected_home.mkdir()
                configuration = (directory / ".cargo/config.toml").read_bytes()
                lockfile = directory / "Cargo.lock"
                before = lockfile.read_bytes() if lockfile.exists() else None
                first = len(requests)
                if arguments == ["generate-lockfile"]:
                    command = cooldown.resolution_command("/toolchain/bin/cargo", "initial")
                elif len(arguments) == 5 and arguments[:2] == ["update", "--package"] and arguments[3] == "--precise":
                    command = cooldown.resolution_command("/toolchain/bin/cargo", "precise", arguments[2], arguments[4])
                else:
                    assert arguments in (["update", "--workspace", "--locked"],
                                         ["update", "--workspace", "--locked", "--offline"])
                    command = ["/toolchain/bin/cargo", *arguments]
                assert cooldown.APPROVED_EXCEPTIONS == {}
                try:
                    if allow:
                        # This heap-only inert approval exercises the production
                        # constructor. No product decision/configuration is changed.
                        cooldown.APPROVED_EXCEPTIONS[urgent_exception["decision_id"]] = cooldown.canonical(urgent_exception)
                    selected = cooldown.resolution_environment("/toolchain", selected_home, "/tmp",
                        exception=urgent_exception if allow else None)
                    assert (AGE_ENV in selected) is allow and AGE_ENV not in os.environ
                    result = environment.run(command, cwd=directory,
                        environment=selected, timeout=25, max_output=128 * 1024)
                finally:
                    cooldown.APPROVED_EXCEPTIONS.clear()
                assert cooldown.APPROVED_EXCEPTIONS == {}
                assert result["failure"] is None, result
                if refusal:
                    assert result["exit_code"] == 101, result
                    assert "too new" in result["output"] and "minimum age 14 days" in result["output"], result
                    assert (lockfile.read_bytes() if lockfile.exists() else None) == before
                else:
                    environment.require_ok(result)
                assert configuration == (directory / ".cargo/config.toml").read_bytes()
                assert AGE_ENV not in os.environ
                seen = requests[first:]
                assert (not seen) if offline else any(path.startswith("/index/") for path in seen), (label, seen)
                assert not any(path.endswith(".crate") for path in seen), (label, seen)
                locked = [] if not lockfile.exists() else tomllib.loads(lockfile.read_text())["package"]
                graph = {row["name"]: row["version"] for row in locked if row["name"] != "cooldown-control"}
                phases.append({"label": label, "arguments": arguments, "requests": seen,
                    "exit_code": result["exit_code"], "output": result["output"],
                    "single_child_urgent_override": allow, "persistent_override_absent": AGE_ENV not in os.environ,
                    "configuration_sha256": hashlib.sha256(configuration).hexdigest(), "graph": graph,
                    "lockfile_sha256": None if not locked else hashlib.sha256(lockfile.read_bytes()).hexdigest()})
                return graph, selected_home

            if group == "selection":
                direct = fixture("eligible-direct", DIRECT, "1")
                assert cargo("eligible-direct", direct, ["generate-lockfile"])[0] == {DIRECT: "1.0.0"}
                visible[UPSTREAM] = catalog[UPSTREAM][:1]
                updating = fixture("updating-legitimate", UPSTREAM, "1")
                assert cargo("baseline-direct", updating, ["generate-lockfile"])[0] == {
                    UPSTREAM: "1.0.0", LEAF: "1.0.0"}
                visible[UPSTREAM] = catalog[UPSTREAM]
                cargo("young-transitive-refusal", updating,
                      ["update", "--package", UPSTREAM + "@1.0.0", "--precise", "1.1.0"], refusal=True)
                fallback = fixture("eligible-closure", UPSTREAM, "1")
                assert cargo("eligible-closure", fallback, ["generate-lockfile"])[0] == {
                    UPSTREAM: "1.0.0", LEAF: "1.0.0"}
                refused = fixture("young-exact", ONLY, "=1.0.0")
                cargo("young-exact", refused, ["generate-lockfile"], refusal=True)
                legal = fixture("old-exact", DIRECT, "=1.0.0")
                assert cargo("old-exact", legal, ["generate-lockfile"])[0] == {DIRECT: "1.0.0"}
            else:
                visible[UPSTREAM] = catalog[UPSTREAM][:1]
                updating = fixture("urgent-legitimate", UPSTREAM, "1")
                baseline, _ = cargo("urgent-baseline", updating, ["generate-lockfile"])
                original_registry = cooldown.registry_records((updating / "Cargo.lock").read_bytes())
                visible[UPSTREAM] = catalog[UPSTREAM]
                urgent, _ = cargo("urgent-once", updating,
                    ["update", "--package", UPSTREAM, "--precise", "1.1.0"], allow=True)
                assert baseline == {UPSTREAM: "1.0.0", LEAF: "1.0.0"}
                assert urgent == {UPSTREAM: "1.1.0", LEAF: "2.0.0"}
                current_registry = cooldown.registry_records((updating / "Cargo.lock").read_bytes())
                actual_delta = {"added": [item for item in current_registry if item not in original_registry],
                                "removed": [item for item in original_registry if item not in current_registry]}
                assert actual_delta == expected_delta
                assert cooldown.canonical(actual_delta) == urgent_exception["delta_sha256"]
                before = (updating / "Cargo.lock").read_bytes()
                locked, home = cargo("locked-young-retained", updating, ["update", "--workspace", "--locked"])
                assert locked == urgent and (updating / "Cargo.lock").read_bytes() == before
                assert cargo("frozen-young-retained", updating, ["update", "--workspace", "--locked", "--offline"],
                             home=home, offline=True)[0] == urgent
                normal = fixture("normal-after-urgent", ONLY, "=1.0.0")
                cargo("normal-after-urgent", normal, ["generate-lockfile"], refusal=True)
                missing = fixture("missing-publication-time", MISSING, "1")
                assert cargo("missing-publication-time", missing, ["generate-lockfile"])[0] == {MISSING: "1.1.0"}
                alternate = fixture("alternate-global", DIRECT, "1", alternate=True)
                assert cargo("alternate-global", alternate, ["generate-lockfile"])[0] == {DIRECT: "1.0.0"}
                shorter = fixture("alternate-shorter", DIRECT, "1", alternate=True, shorter=True)
                assert cargo("alternate-shorter", shorter, ["generate-lockfile"])[0] == {DIRECT: "1.1.0"}
                phases[1]["reviewed_entire_exception_delta"] = {
                    name: {"old": baseline[name], "new": urgent[name]} for name in sorted(urgent)}
                phases[1]["full_registry_delta"] = actual_delta
                phases[1]["test_only_exception"] = urgent_exception
                phases[-3]["age_coverage"] = "unverified: registry omitted pubtime; Cargo accepts this exception"
                phases[-1]["scope"] = "isolated precedence countercontrol; forbidden by ordinary project admission"
            finished = datetime.now(timezone.utc)
            assert timedelta(0) <= finished - started < timedelta(minutes=2)
            assert not server_errors and not any(path.endswith(".crate") for path in requests)
            assert cooldown.APPROVED_EXCEPTIONS == {}
            files = [path for path in Path("/work").rglob("*") if path.is_file()]
            assert not [path for path in files if path.suffix in {".rlib", ".rmeta", ".o", ".so", ".a", ".crate"}
                        or path.name.startswith(("build-script-", "build_script_build"))]
            assert len(list(Path("/work").rglob("*"))) < 256
            assert sum(path.stat().st_size for path in files) < 1024 * 1024
            print(json.dumps({"status": "passed", "group": group, "policy": {"days": 14, "incompatible": "deny"},
                "scope": "Cargo semantics only; locked imports and missing timestamps are not fresh graph approval",
                "resolution_utc_interval": [started.isoformat(), finished.isoformat()],
                "cutoff_utc_interval": [(started - timedelta(days=14)).isoformat(),
                                        (finished - timedelta(days=14)).isoformat()],
                "independent_publication_times": {"old_15_days": old, "young_13_days": young},
                "phases": phases, "requests": len(requests), "package_archives_downloaded": 0,
                "compiled_artifacts": 0, "persistent_override_absent": AGE_ENV not in os.environ}))
        finally:
            server.shutdown()
            worker.join(timeout=3)
            assert not worker.is_alive(), "owned sparse-registry thread did not terminate"


class RealCargoCooldownControls(unittest.TestCase):
    def run_group(self, group):
        root = Path(__file__).resolve().parents[2]
        product = load("dependency_cooldown_product", root / "scripts/verify.py")
        policy = json.loads((root / "policy/tool-manifest.json").read_text())
        toolchain = Path(os.environ.get("RUSTUP_HOME", str(Path.home() / ".rustup"))) / "toolchains" / (
            policy["rust"]["channel"] + "-" + policy["rust"]["host"])
        tools = product.admit_tools(root, toolchain, None, "windows")
        with tempfile.TemporaryDirectory(prefix="nevrotick-cooldown-") as folder:
            scratch = Path(folder)
            work, projection = scratch / "work", scratch / "probe"
            (work / "fixture").mkdir(parents=True)
            (work / "cargo-home").mkdir()
            projection.mkdir()
            driver = Path(__file__).resolve()
            inputs = [driver, driver.with_name("test_advisory_cargo_refresh.py"), root / "scripts/check-environment.py",
                      root / "scripts/dependency_cooldown.py"]
            for path in inputs:
                shutil.copyfile(path, projection / path.name)
            command = product.ENV.sandbox_command(toolchain, work, ["python3", "-X", "dev", "-Werror", "-B",
                "/probe/" + driver.name, "--cooldown-driver", os.readlink("/proc/self/ns/net"), group])
            boundary = command.index("--")
            command[boundary:boundary] = ["--ro-bind", str(projection), "/probe"]
            result = product.ENV.run(command, cwd=root, timeout=120, max_output=2 * 1024 * 1024)
            observed = json.loads(product.ENV.require_ok(result))
            self.assertEqual(observed["status"], "passed")
            self.assertEqual(observed["group"], group)
            self.assertEqual(len(observed["phases"]), 6 if group == "selection" else 8)
            self.assertEqual(product.admit_tools(root, toolchain, None, "windows"), tools)
            for path in inputs:
                self.assertEqual(path.read_bytes(), (projection / path.name).read_bytes())
            observed.update(toolchain=str(toolchain), tool_identities_sha256=tools,
                inputs_sha256={str(path.relative_to(root)): hashlib.sha256(path.read_bytes()).hexdigest()
                               for path in inputs},
                tool_policy_sha256=hashlib.sha256((root / "policy/tool-manifest.json").read_bytes()).hexdigest())
            print(json.dumps(observed))
        self.assertFalse(scratch.exists(), "owned fixture output must be retired")

    def test_old_direct_and_complete_transitive_closure_before_lock_publication(self):
        self.run_group("selection")

    def test_transient_urgent_override_and_documented_cargo_coverage_limits(self):
        self.run_group("exceptions")


if __name__ == "__main__":
    if len(sys.argv) == 4 and sys.argv[1] == "--cooldown-driver":
        cooldown_driver(sys.argv[2], sys.argv[3])
    else:
        unittest.main()
