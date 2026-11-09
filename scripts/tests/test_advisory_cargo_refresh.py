"""Real pinned Cargo metadata-only acquisition beside its archive-fetch control.

The HTTP sparse registry and Cargo live in the same isolated network namespace.
No public endpoint, product workspace, compilation or build script is involved.
Missing pinned tools or isolation are failures, never skips or host fallbacks.
"""
from __future__ import annotations

import hashlib
import http.server
import importlib.util
import io
import json
import os
from pathlib import Path
import shutil
import sys
import tarfile
import tempfile
import threading
import tomllib
import unittest

NAME = "advisory-fixture-dep"


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError("Required pinned fixture helper is unavailable.")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def crate(version):
    """Return a tiny inert Cargo-format archive, solely for this controlled peer."""
    stream = io.BytesIO()
    with tarfile.open(fileobj=stream, mode="w:gz") as archive:
        files = {"Cargo.toml": f'[package]\nname = "{NAME}"\nversion = "{version}"\nedition = "2024"\nbuild = false\n',
                 "src/lib.rs": "pub fn inert_fixture() {}\n"}
        for name, text in files.items():
            payload = text.encode()
            entry = tarfile.TarInfo(f"{NAME}-{version}/{name}")
            entry.size = len(payload)
            archive.addfile(entry, io.BytesIO(payload))
    return stream.getvalue()


def sparse_driver(host_network_namespace):
    """Run only inside the configured namespace, serving finite local fixtures."""
    assert os.readlink("/proc/self/ns/net") != host_network_namespace
    environment = load("isolated_refresh_environment", Path("/probe/check-environment.py"))
    fixture = Path("/work/fixture")
    archives = {version: crate(version) for version in ("1.0.0", "1.1.0")}
    assert all(len(body) < 16384 for body in archives.values())
    visible_versions = ["1.0.0"]
    requests = []
    phases = []

    class Registry(http.server.BaseHTTPRequestHandler):
        def setup(self):
            super().setup()
            self.connection.settimeout(2)

        def log_message(self, *_):
            pass

        def do_GET(self):
            assert len(requests) < 128 and len(self.path) <= 256
            requests.append(self.path)
            body = None
            if self.path == "/index/config.json":
                body = json.dumps({"dl": base + "/crate/{crate}/{version}.crate"}).encode()
            elif self.path == "/index/ad/vi/" + NAME:
                body = b"".join(json.dumps({"name": NAME, "vers": version, "deps": [],
                    "cksum": hashlib.sha256(archives[version]).hexdigest(), "features": {},
                    "yanked": False}).encode() + b"\n" for version in visible_versions)
            elif self.path in {f"/crate/{NAME}/{version}.crate" for version in archives}:
                body = archives[self.path.rsplit("/", 1)[1].removesuffix(".crate")]
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
            (fixture / "src").mkdir()
            (fixture / ".cargo").mkdir()
            (fixture / "src/lib.rs").write_text("pub fn inert_workspace() {}\n")
            (fixture / "Cargo.toml").write_text(
                '[workspace]\nmembers = ["."]\nresolver = "3"\n[package]\nname = "refresh-control"\n'
                'version = "0.0.0"\nedition = "2024"\npublish = false\n'
                f'[dependencies]\n{NAME} = "1"\n')
            (fixture / ".cargo/config.toml").write_text(
                '[source.crates-io]\nreplace-with = "controlled-sparse"\n'
                f'[source.controlled-sparse]\nregistry = "sparse+{base}/index/"\n')

            def cargo(label, arguments):
                home = Path("/work") / (label + "-cargo-home")
                home.mkdir()
                begin = len(requests)
                selected = dict(os.environ, CARGO_HOME=str(home), RUSTC="/toolchain/bin/rustc",
                    CARGO_NET_RETRY="0", CARGO_HTTP_TIMEOUT="8", CARGO_BUILD_JOBS="1",
                    RUST_TEST_THREADS="1", CARGO_REGISTRIES_CRATES_IO_PROTOCOL="sparse")
                result = environment.run(["/toolchain/bin/cargo", *arguments], cwd=fixture,
                    environment=selected, timeout=25, max_output=128 * 1024)
                environment.require_ok(result)
                seen = requests[begin:]
                assert any(path.startswith("/index/") for path in seen), (label, seen)
                phases.append({"label": label, "requests": seen, "command": result["command"],
                    "exit_code": result["exit_code"], "output": result["output"]})
                return home, [path for path in seen if path.endswith(".crate")]

            _, initial_archives = cargo("initial", ["generate-lockfile"])
            assert initial_archives == []
            lockfile = fixture / "Cargo.lock"
            original = lockfile.read_bytes()
            assert len(original) <= 8192
            packages = {item["name"]: item["version"] for item in tomllib.loads(original.decode())["package"]}
            assert packages == {"refresh-control": "0.0.0", NAME: "1.0.0"}, packages
            visible_versions.append("1.1.0")
            _, refreshed_archives = cargo("metadata-only", ["update", "--workspace", "--locked"])
            assert refreshed_archives == [], refreshed_archives
            assert lockfile.read_bytes() == original, "metadata refresh changed the exact admitted lock"
            fetched_home, fetched_archives = cargo("fetch-countercontrol", ["fetch", "--locked"])
            assert fetched_archives == [f"/crate/{NAME}/1.0.0.crate"], fetched_archives
            assert lockfile.read_bytes() == original
            cached = list((fetched_home / "registry/cache").glob("*/*.crate"))
            assert len(cached) == 1 and cached[0].read_bytes() == archives["1.0.0"]
            compiled = [p for p in Path("/work").rglob("*") if p.is_file()
                and (p.suffix in {".rlib", ".rmeta", ".o", ".so", ".a"}
                     or p.name.startswith("build-script-"))]
            assert compiled == [], "metadata commands must not produce compiled artifacts"
            assert len(list(Path("/work").rglob("*"))) < 256
            assert sum(p.stat().st_size for p in Path("/work").rglob("*") if p.is_file()) < 1024 * 1024
            print(json.dumps({"status": "passed", "lockfile_sha256": hashlib.sha256(original).hexdigest(),
                "locked_version": packages[NAME], "additional_visible_version": "1.1.0", "phases": phases}))
        finally:
            server.shutdown()
            worker.join(timeout=3)
            assert not worker.is_alive(), "owned sparse-registry thread did not terminate"


class RealCargoAdvisoryRefreshControls(unittest.TestCase):
    def test_locked_workspace_update_refreshes_only_metadata_beside_archive_fetch(self):
        root = Path(__file__).resolve().parents[2]
        product = load("advisory_refresh_product", root / "scripts/verify.py")
        policy = json.loads((root / "policy/tool-manifest.json").read_text())
        toolchain = Path(os.environ.get("RUSTUP_HOME", str(Path.home() / ".rustup"))) / "toolchains" / (
            policy["rust"]["channel"] + "-" + policy["rust"]["host"])
        # Select the existing base-tool admission branch, without an extension
        # executable. This does not select or execute a Windows verification row.
        tools = product.admit_tools(root, toolchain, None, "windows")
        with tempfile.TemporaryDirectory(prefix="nevrotick-cargo-refresh-") as folder:
            scratch = Path(folder)
            work, projection = scratch / "work", scratch / "probe"
            (work / "fixture").mkdir(parents=True)
            (work / "cargo-home").mkdir()
            projection.mkdir()
            driver = Path(__file__).resolve()
            shutil.copyfile(driver, projection / driver.name)
            shutil.copyfile(root / "scripts/check-environment.py", projection / "check-environment.py")
            command = product.ENV.sandbox_command(toolchain, work, ["python3", "-X", "dev", "-Werror", "-B",
                "/probe/" + driver.name, "--sparse-driver", os.readlink("/proc/self/ns/net")])
            boundary = command.index("--")
            command[boundary:boundary] = ["--ro-bind", str(projection), "/probe"]
            result = product.ENV.run(command, cwd=root, timeout=120, max_output=2 * 1024 * 1024)
            output = product.ENV.require_ok(result)
            observed = json.loads(output)
            self.assertEqual(observed["status"], "passed")
            self.assertEqual([phase["exit_code"] for phase in observed["phases"]], [0, 0, 0])
            self.assertEqual(product.admit_tools(root, toolchain, None, "windows"), tools)
            self.assertEqual(driver.read_bytes(), (projection / driver.name).read_bytes())
            self.assertEqual((root / "scripts/check-environment.py").read_bytes(),
                             (projection / "check-environment.py").read_bytes())
            print(output)


if __name__ == "__main__":
    if len(sys.argv) == 3 and sys.argv[1] == "--sparse-driver":
        sparse_driver(sys.argv[2])
    else:
        unittest.main()
