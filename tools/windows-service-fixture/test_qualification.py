#!/usr/bin/env python3
"""Controls for the service-local portable host admission and sandbox adapter."""
from __future__ import annotations

import fcntl
import importlib.util
import json
from pathlib import Path
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import mock_open, patch

SPEC = importlib.util.spec_from_file_location("fixture_qualification_controls", Path(__file__).with_name("qualification.py"))
Q = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(Q)


class HostAdmissionTests(unittest.TestCase):
    """Machine relocation must work while changed inputs and unsafe mounts fail closed."""

    def test_current_helpers_do_not_need_historical_ubuntu_checksums(self):
        """Current public executables are recorded without comparing an old host pin."""
        with patch.object(Q.C.PRODUCT, "sha256", return_value="a" * 64):
            helpers = Q.host_helpers()
        self.assertEqual(set(helpers), {"bwrap", "prlimit", "cc", "ld", "python3"})
        self.assertTrue(all(item["sha256"] == "a" * 64 for item in helpers.values()))
        self.assertTrue(all(Path(item["resolved"]).is_relative_to("/usr") for item in helpers.values()))

    def test_missing_helper_is_not_a_host_execution_fallback(self):
        """A missing namespace helper is an incomplete environment, never direct execution."""
        with patch.object(Q.shutil, "which", return_value=None):
            with self.assertRaisesRegex(Q.Incomplete, "helper is missing"):
                Q.host_helpers()

    def test_compiler_snapshot_records_contained_links_and_rejects_escape(self):
        """Observe the whole selected payload and reject redirected library custody."""
        with tempfile.TemporaryDirectory(prefix="service-tool-control-") as directory:
            stable = Path(directory)
            for name in ("bin", "etc", "lib", "libexec", "share"):
                (stable / name).mkdir()
            (stable / "bin/compiler").write_bytes(b"compiler-control")
            (stable / "lib/owned").write_bytes(b"library-control")
            (stable / "lib/alias").symlink_to("owned")
            found = Q.compiler_snapshot(stable)
            self.assertEqual(set(found), {"bin/compiler", "lib/owned", "lib/alias"})
            self.assertEqual(found["lib/alias"], {"kind": "symlink", "target": "owned"})
            (stable / "lib/escape").symlink_to("/usr/bin/python3")
            with self.assertRaisesRegex(Q.Incomplete, "escapes"):
                Q.compiler_snapshot(stable)

    def test_command_argv_isolated_inputs_and_resource_profiles(self):
        """Keep the existing namespace/custody protections with actual host helpers."""
        for mutation, source_name in ((False, "source"), (True, "source"),
                                      (False, "worker-variant-000"), (True, "worker-variant-000"),
                                      (False, "native-source"), (True, "native-source")):
            with self.subTest(mutation=mutation, source=source_name), tempfile.TemporaryDirectory(prefix="service-runtime-control-") as directory:
                parent = Path(directory)
                work = parent / "work"
                work.mkdir(mode=0o700)
                source = work / source_name
                source.mkdir()
                vendor = parent / "vendor"
                vendor.mkdir()
                stable = parent / "compiler"
                stable.mkdir()
                config = work / "cargo-home/config.toml"
                config.parent.mkdir()
                config.write_text("source-control\n")
                admitted = {"stable": stable, "extensions": {}, "helpers": Q.host_helpers()}
                use_scratch_guard = not mutation and source_name == "source"
                with patch.object(Q, "check_core_dumps"), patch.object(Q, "check_service_tools") as check, patch.object(
                    Q.C.PRODUCT.ENV, "run", return_value={"exit_code": 0, "output": "controlled", "failure": None}
                ) as execute, patch.object(Q, "resource_stop_reason", return_value=None) as headroom:
                    result = Q.run_command(work, [str(stable / "bin/cargo"), "--version"], admitted,
                                           vendor, source, timeout=180, mutation=mutation, guard_scratch=use_scratch_guard)
                    checked_root = work if use_scratch_guard else None
                    headroom.assert_called_once_with(checked_root)
                    headroom.reset_mock()
                    self.assertIsNone(execute.call_args.kwargs["resource_guard"]())
                    headroom.assert_called_once_with(checked_root)
                self.assertFalse((work / "target").exists(), "Cargo must create its own cache directory and marker")
                self.assertTrue(all((work / name).is_dir() for name in ("home", "tmp", "cargo-home")))
                self.assertEqual(check.call_count, 2)
                self.assertEqual(result["namespace_argv"], ["/toolchain/bin/cargo", "--version"])
                args = execute.call_args.args[0]
                self.assertEqual([args[i:i+3] for i, value in enumerate(args[:-2]) if value == "--bind"],
                                 [["--bind", str(work), "/work"]])
                self.assertIn("--unshare-all", args)
                self.assertIn("--clearenv", args)
                self.assertIn("--cap-drop", args)
                self.assertIn("--as=8589934592", args)
                self.assertIn("--cpu=180", args)
                self.assertIn("--nproc=512" if mutation else "--nproc=128", args)
                self.assertIn("--nofile=4096" if mutation else "--nofile=256", args)
                self.assertIn("--fsize=268435456", args)
                self.assertIn(["--ro-bind", str(source), "/work/source"], [args[i:i+3] for i in range(len(args)-2)])
                self.assertIn(["--ro-bind", str(source), "/work/" + source_name],
                              [args[i:i+3] for i in range(len(args)-2)])
                self.assertIn(["--ro-bind", str(vendor), "/vendor"], [args[i:i+3] for i in range(len(args)-2)])
                settings = {args[i+1]: args[i+2] for i, value in enumerate(args[:-2]) if value == "--setenv"}
                self.assertEqual(settings["CARGO_NET_OFFLINE"], "true")
                self.assertEqual(settings["CARGO_BUILD_JOBS"], "1")
                self.assertEqual(settings["RUST_TEST_THREADS"], "1")
                self.assertEqual(settings.get("CARGO_TARGET_DIR"), None if mutation else "/work/target")
                self.assertEqual(settings.get("RUST_TEST_NOCAPTURE"), "1" if mutation else None)
                self.assertEqual(execute.call_args.kwargs["environment"], {"PATH": "/usr/bin:/bin", "LANG": "C.UTF-8"})

    def test_native_profile_writes_only_selected_file_through_both_readonly_aliases(self):
        for subject in ("host", "error", "diagnostics"):
            with self.subTest(subject=subject), tempfile.TemporaryDirectory(prefix="service-native-control-") as directory:
                parent = Path(directory)
                work = parent / "work"
                work.mkdir(mode=0o700)
                source = work / "native-source"
                fixture = source / Q.TOOL
                (fixture / "src").mkdir(parents=True)
                for name in ("host.rs", "error.rs", "diagnostics.rs", "host_tests.rs"):
                    (fixture / "src" / name).write_text("immutable neighbor\n")
                (fixture / "Cargo.toml").write_text("immutable manifest\n")
                vendor, stable = parent / "vendor", parent / "compiler"
                vendor.mkdir(); stable.mkdir()
                config = work / "cargo-home/config.toml"
                config.parent.mkdir(); config.write_text("immutable config\n")
                admitted = {"stable": stable, "extensions": {"cargo-mutants": Path("/selected/cargo-mutants")},
                            "helpers": Q.host_helpers()}
                command = Q.native_mutation_command(admitted, subject)
                self.assertNotIn("--jobs", command)
                self.assertEqual(command.count("--in-place"), 1)
                self.assertEqual(command[command.index("--jobserver-tasks") + 1], "1")
                with patch.object(Q, "check_core_dumps"), patch.object(Q, "check_service_tools"), patch.object(
                        Q.C.PRODUCT.ENV, "run", return_value={"exit_code": 0, "output": "controlled", "failure": None}) as execute:
                    Q.run_command(work, command, admitted, vendor, source, mutation=True,
                                  cwd=fixture, native_subject=subject)
                args = execute.call_args.args[0]
                writable = [args[i:i+3] for i, value in enumerate(args[:-2]) if value == "--bind"]
                selected = source / Q.TOOL / Q.NATIVE_MUTATION_FILES[subject]
                self.assertEqual(writable, [["--bind", str(work), "/work"],
                    ["--bind", str(selected), "/work/source/" + Q.TOOL + "/" + Q.NATIVE_MUTATION_FILES[subject]],
                    ["--bind", str(selected), "/work/native-source/" + Q.TOOL + "/" + Q.NATIVE_MUTATION_FILES[subject]]])
                self.assertIn(["--ro-bind", str(source), "/work/source"], [args[i:i+3] for i in range(len(args)-2)])
                self.assertIn(["--ro-bind", str(source), "/work/native-source"], [args[i:i+3] for i in range(len(args)-2)])
                settings = {args[i+1]: args[i+2] for i, value in enumerate(args[:-2]) if value == "--setenv"}
                self.assertEqual(settings["CARGO_TARGET_DIR"], "/work/target")
                self.assertEqual(settings["CARGO_BUILD_JOBS"], "1")
                for defect in ("mode", "command", "path", "hardlink", "symlink"):
                    with self.subTest(defect=defect), patch.object(Q.C.PRODUCT.ENV, "run") as process:
                        options = {"mutation": True, "cwd": fixture, "native_subject": subject}
                        bad_command = list(command)
                        bad_source = source
                        if defect == "mode": options["mutation"] = False
                        if defect == "command": bad_command.extend(["--jobs", "1"])
                        if defect == "path": bad_source = work / "source"; bad_source.mkdir(exist_ok=True)
                        if defect == "hardlink": (fixture / "src/alias.rs").hardlink_to(selected)
                        if defect == "symlink": selected.unlink(); selected.symlink_to("host_tests.rs")
                        with self.assertRaises(Q.Incomplete):
                            Q.run_command(work, bad_command, admitted, vendor, bad_source, **options)
                        process.assert_not_called()
                        if defect == "hardlink": (fixture / "src/alias.rs").unlink()
                        if defect == "symlink": selected.unlink(); selected.write_text("immutable neighbor\n")
        for subject in (True, "engine_worker", "../host", "", 0):
            with self.subTest(subject=subject), self.assertRaises(Q.Incomplete):
                Q.native_mutation_command({}, subject)
        with self.assertRaises(Q.Incomplete):
            Q.run_command(Path("/not-used"), ["cargo-mutants", "--in-place"], {}, Path("/not-used"), Path("/not-used"))

    def test_bad_command_and_budget_are_rejected_before_execution(self):
        """Reject empty/ambiguous input before touching a process or owned directory."""
        for command, options in (([], {}), (["tool\0name"], {}), (["tool"], {"timeout": float("inf")}),
                                 (["tool"], {"timeout": True}), (["tool"], {"mutation": "yes"})):
            with self.subTest(command=command, options=options), patch.object(Q.C.PRODUCT.ENV, "run") as execute:
                with self.assertRaises(Q.Incomplete):
                    Q.run_command(Path("/not-used"), command, {}, Path("/not-used"), Path("/not-used"), **options)
                execute.assert_not_called()


class ResourceHeadroomTests(unittest.TestCase):
    """Independently challenge exact disk/memory boundaries and missing observations."""

    def test_declared_reserves_have_exact_healthy_neighbors(self):
        for free_disk, memory_kib, expected in ((4 * 1024**3, 1048576, None),
                (4 * 1024**3 - 1, 1048576, "DISK_RESERVE"),
                (4 * 1024**3, 1048575, "FREE_MEMORY")):
            with self.subTest(disk=free_disk, memory=memory_kib), patch.object(
                    Q.shutil, "disk_usage", return_value=type("Disk", (), {"free": free_disk})()), patch.object(
                    Q.Path, "open", mock_open(read_data=f"MemAvailable: {memory_kib} kB\n".encode())):
                self.assertEqual(Q.resource_stop_reason(), expected)

    def test_missing_memory_observation_is_not_permission_to_continue(self):
        with patch.object(Q.shutil, "disk_usage", return_value=type("Disk", (), {"free": 2**60})()), patch.object(
                Q.Path, "open", mock_open(read_data=b"MemFree: 999999999 kB\n")):
            self.assertEqual(Q.resource_stop_reason(), "RESOURCE_OBSERVATION")

    def test_checkout_and_selected_scratch_each_keep_the_existing_reserve(self):
        scratch = Path("/selected-work")
        minimum = 4 * 1024 ** 3
        cases = ((minimum, minimum, None), (minimum - 1, minimum, "DISK_RESERVE"),
                 (minimum, minimum - 1, "WORK_DISK_RESERVE"))
        for checkout_free, scratch_free, expected in cases:
            with self.subTest(checkout=checkout_free, scratch=scratch_free):
                free = {Q.ROOT: checkout_free, scratch: scratch_free}
                with patch.object(Q.shutil, "disk_usage", side_effect=lambda path: SimpleNamespace(free=free[path])), patch.object(
                        Q.Path, "open", mock_open(read_data=b"MemAvailable: 1048576 kB\n")):
                    self.assertEqual(Q.resource_stop_reason(scratch), expected)

    def test_selected_scratch_admission_preserves_defaults_and_rejects_aliases(self):
        with tempfile.TemporaryDirectory(prefix="service-scratch-control-") as directory:
            parent = Path(directory)
            root, cache, scratch, public = (parent / name for name in ("repo", "cache", "scratch", "public"))
            for path in (root, cache, scratch, public):
                path.mkdir(mode=0o700)
            public.chmod(0o755)
            alias = parent / "alias"
            alias.symlink_to(scratch, target_is_directory=True)
            with patch.object(Q, "ROOT", root), patch.object(Q, "CACHE", cache):
                self.assertEqual(Q.admit_scratch_root(None, "fixture-test"), cache)
                self.assertEqual(Q.admit_scratch_root(scratch, "privilege-checkpoint"), scratch)
                for path, lane in ((scratch, "fixture-test"), (public, "graph"), (alias, "graph"),
                                   (root, "graph"), (cache, "graph"), (Path("relative"), "graph")):
                    with self.subTest(path=path, lane=lane), self.assertRaises(Q.Incomplete):
                        Q.admit_scratch_root(path, lane)

    def test_existing_work_leaf_releases_shared_writer_without_a_receipt(self):
        with tempfile.TemporaryDirectory(prefix="service-scratch-control-") as directory:
            parent = Path(directory)
            root, cache, scratch = (parent / name for name in ("repo", "cache", "scratch"))
            records = root / "records"
            records.mkdir(parents=True)
            scratch.mkdir(mode=0o700)
            label = "occupied"
            work = scratch / label
            work.mkdir(mode=0o700)
            marker = work / "preserve"
            marker.write_bytes(b"prior work")
            with patch.object(Q, "ROOT", root), patch.object(Q, "CACHE", cache), patch.object(Q, "RECORDS", records), patch.object(
                    Q, "disable_core_dumps"), patch.object(Q.shutil, "disk_usage", return_value=SimpleNamespace(free=2**60)):
                with self.assertRaises(FileExistsError):
                    Q.run(SimpleNamespace(label=label, lane="graph", scratch_root=scratch))
            self.assertEqual(marker.read_bytes(), b"prior work")
            self.assertEqual(list(records.iterdir()), [])
            with (cache / "writer.lock").open("rb") as lock:
                fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)

    def test_scratch_cannot_place_derived_files_in_vendor_or_toolchain(self):
        with tempfile.TemporaryDirectory(prefix="service-scratch-control-") as directory:
            parent = Path(directory)
            root, cache, vendor, compiler = (parent / name for name in ("repo", "cache", "vendor", "compiler"))
            records = root / "records"
            records.mkdir(parents=True)
            vendor.mkdir(mode=0o700)
            compiler.mkdir(mode=0o700)
            tools = {"readonly_roots": [compiler], "extensions": {}}
            with patch.object(Q, "ROOT", root), patch.object(Q, "CACHE", cache), patch.object(Q, "RECORDS", records), patch.object(
                    Q, "disable_core_dumps"), patch.object(Q, "admit_service_tools", return_value=tools), patch.object(
                    Q.shutil, "disk_usage", return_value=SimpleNamespace(free=2**60)):
                for scratch in (vendor, compiler):
                    with self.subTest(scratch=scratch), self.assertRaisesRegex(Q.Incomplete, "overlaps a read-only input"):
                        Q.run(SimpleNamespace(label="protected", lane="graph", scratch_root=scratch, vendor_dir=vendor))
                    self.assertEqual(list(scratch.iterdir()), [])
            self.assertEqual(list(records.iterdir()), [])


class CoreDumpAdmissionTests(unittest.TestCase):
    """Piped collectors must not bypass the process-only crash-test refusal."""

    def test_zero_limit_is_not_accepted_for_a_pipe_collector(self):
        with patch.object(Q, "core_dump_policy", return_value={"limit_bytes": 1}), patch.object(
                Q.resource, "getrlimit", return_value=(0, 0)):
            with self.assertRaisesRegex(Q.Incomplete, "exact inherited process refusal"):
                Q.check_core_dumps()

    def test_exact_pipe_sentinel_and_zero_file_limit_remain_legal(self):
        for limit in (0, 1):
            with self.subTest(limit=limit), patch.object(
                    Q, "core_dump_policy", return_value={"limit_bytes": limit}), patch.object(
                    Q.resource, "getrlimit", return_value=(limit, limit)):
                self.assertEqual(Q.check_core_dumps(), {"limit_bytes": limit})

    def test_process_refusal_sets_both_limits_without_host_writes(self):
        with patch.object(Q, "core_dump_policy", return_value={"limit_bytes": 1}), patch.object(
                Q.resource, "setrlimit") as set_limit, patch.object(
                Q.resource, "getrlimit", return_value=(1, 1)):
            self.assertEqual(Q.disable_core_dumps(), {"limit_bytes": 1})
        set_limit.assert_called_once_with(Q.resource.RLIMIT_CORE, (1, 1))


class EngineCompositionGraphTests(unittest.TestCase):
    """Synthetic declared graph roles exercise the real reader without live product admission."""

    def fixture(self):
        """Declare independent local targets and separate normal/development trees."""
        policy = {"packages": [], "rows": {Q.TARGETS[0]: {"package_scopes": {
            "windows-platform": {"features": {}, "local_features": {
                "windows-platform@0.0.0": {"target": [[]]}}},
            "nevrotick-engine": {"features": {}, "local_features": {
                "nevrotick-engine@0.0.0": {"target": [[]]}}},
        }}}}
        target_paths = {
            "nevrotick-windows-service-fixture": [
                ("nevrotick_windows_service_fixture", "lib", "lib", "src/lib.rs"),
                ("nevrotick-lab-s02-engine", "bin", "bin", "src/bin/engine.rs"),
                ("nevrotick-lab-s02-broker", "bin", "bin", "src/bin/broker.rs"),
                ("entrypoints", "test", "bin", "tests/entrypoints.rs")],
            "windows-platform": [("windows_platform", "lib", "lib", "src/lib.rs"),
                *[(name, "test", "bin", "tests/" + name + ".rs") for name in (
                    "service", "scm", "privilege_observation", "privilege_observation_contracts",
                    "diagnostics")]],
            "nevrotick-engine": [("nevrotick_engine", "lib", "lib", "src/lib.rs"),
                ("nevrotick-observe", "bin", "bin", "src/bin/nevrotick-observe.rs"),
                *[(name, "test", "bin", "tests/" + name + ".rs") for name in (
                    "execute_entered", "prepare_entered", "prepare_release", "quality",
                    "runtime_boundary", "series", "session")]],
        }
        packages, nodes = [], []
        for name, targets in target_paths.items():
            owner = "tools/windows-service-fixture" if name == "nevrotick-windows-service-fixture" else "crates/" + name
            prefix = "/work/source/" + owner
            packages.append({"id": name, "name": name, "version": "0.0.0", "source": None,
                             "license": "GPL-2.0-only", "manifest_path": prefix + "/Cargo.toml",
                             "targets": [{"name": n, "kind": [k], "crate_types": [c], "src_path": prefix + "/" + path}
                                         for n, k, c, path in targets]})
            deps = [{"pkg": "windows-platform", "dep_kinds": [{"kind": None, "target": None}]},
                    {"pkg": "nevrotick-engine", "dep_kinds": [{"kind": "dev", "target": None}]}]
            nodes.append({"id": name, "features": [], "deps": deps if name == "nevrotick-windows-service-fixture" else []})
        private_status = (
            ("garde", "0.23.0", ["derive"]),
            ("memchr", "2.8.3", ["alloc", "std"]),
            ("serde", "1.0.229", ["std"]),
            ("serde_core", "1.0.229", ["result", "std"]),
            ("serde_json", "1.0.154", ["std"]),
            ("zmij", "1.0.23", []),
        )
        policy["rows"][Q.TARGETS[0]]["package_scopes"]["windows-platform"]["features"] = {
            "garde@0.23.0": {"target": [["derive"]]},
        }
        for name, version, features in private_status:
            policy["packages"].append({
                "name": name, "version": version, "license": "MIT OR Apache-2.0",
                "links": None, "build_script": False, "proc_macro": False,
            })
            packages.append({
                "id": name, "name": name, "version": version, "license": "MIT OR Apache-2.0",
                "source": "registry+https://github.com/rust-lang/crates.io-index",
                "targets": [{"kind": ["lib"]}],
            })
            nodes.append({"id": name, "features": features, "deps": []})
        nodes[0]["deps"].extend(
            {"pkg": name, "dep_kinds": [{"kind": "dev", "target": None}]}
            for name in ("garde", "serde", "serde_json")
        )
        data = {"packages": packages, "resolve": {"nodes": nodes}}
        normal = ("nevrotick-windows-service-fixture v0.0.0 (/work/source/tools/windows-service-fixture)|\n"
                  "`-- windows-platform v0.0.0 (/work/source/crates/windows-platform)|\n"
                  "    `-- garde v0.23.0|derive\n")
        development = normal + ("[dev-dependencies]\n"
                               "|-- nevrotick-engine v0.0.0 (/work/source/crates/nevrotick-engine)|\n"
                               "|-- garde v0.23.0|derive\n"
                               "|-- serde v1.0.229|std\n"
                               "|   `-- serde_core v1.0.229|result,std\n"
                               "`-- serde_json v1.0.154|std\n"
                               "    |-- memchr v2.8.3|alloc,std\n"
                               "    `-- zmij v1.0.23|\n")
        return policy, data, normal, development

    def receive(self, fixture, target=Q.TARGETS[0]):
        """Use actual metadata, target and both tree-role checks, with no mocked reader."""
        policy, data, normal, development = fixture
        return Q.graph_row(policy, target, json.dumps(data), normal, development)

    def windows_fixture(self):
        """Supply independently specified MSVC edges and separate feature sets."""
        policy, data, normal, development = self.fixture()
        policy["rows"][Q.TARGETS[1]] = {"package_scopes": {
            **policy["rows"][Q.TARGETS[0]]["package_scopes"],
            "nevrotick-engine": {"features": {}, "local_features": {
                "nevrotick-engine@0.0.0": {"target": [[]]},
                "windows-platform-native@0.0.0": {"target": [[]]},
            }},
        }}
        leaf = "windows-platform-native"
        leaf_path = "/work/source/crates/" + leaf
        data["packages"].append({"id": leaf, "name": leaf, "version": "0.0.0", "source": None,
                                 "license": "GPL-2.0-only", "manifest_path": leaf_path + "/Cargo.toml",
                                 "targets": [
                                     {"name": "windows_platform_native", "kind": ["lib"], "crate_types": ["lib"],
                                      "src_path": leaf_path + "/src/lib.rs"},
                                     {"name": "subscription_lifecycle", "kind": ["test"], "crate_types": ["bin"],
                                      "src_path": leaf_path + "/tests/subscription_lifecycle.rs"},
                                 ]})
        data["resolve"]["nodes"].append({"id": leaf, "features": [], "deps": []})
        data["resolve"]["nodes"][2]["deps"].append({"pkg": leaf, "dep_kinds": [
            {"kind": None, "target": "cfg(windows)"},
        ]})
        data["resolve"]["nodes"][0]["deps"].append({"pkg": leaf, "dep_kinds": [
            {"kind": None, "target": "cfg(windows)"},
        ]})
        normal_features = ["Win32", "Win32_Foundation", "Win32_Security", "Win32_Storage",
                           "Win32_Storage_FileSystem", "Win32_System", "Win32_System_IO",
                           "Win32_System_Pipes", "Win32_System_Services", "Win32_System_Threading"]
        test_features = sorted([*normal_features, "Win32_Security_Authorization", "Win32_System_Diagnostics",
                                "Win32_System_Diagnostics_Etw", "Win32_System_Time"])
        for name, version, features in (("windows-link", "0.2.1", []),
                                         ("windows-sys", "0.61.2", test_features)):
            policy["packages"].append({"name": name, "version": version, "license": "MIT OR Apache-2.0",
                                       "links": None, "build_script": False, "proc_macro": False})
            data["packages"].append({"id": name, "name": name, "version": version,
                                     "source": "registry+https://github.com/rust-lang/crates.io-index",
                                     "license": "MIT OR Apache-2.0", "targets": [{"kind": ["lib"]}]})
            data["resolve"]["nodes"].append({"id": name, "features": features, "deps": []})
        data["resolve"]["nodes"][0]["deps"].append({"pkg": "windows-sys", "dep_kinds": [
            {"kind": None, "target": "cfg(windows)"}, {"kind": "dev", "target": "cfg(windows)"},
        ]})
        windows_tree = (f"|-- {leaf} v0.0.0 ({leaf_path})|\n"
                        "|-- windows-sys v0.61.2|{}\n|   `-- windows-link v0.2.1|\n`-- windows-platform")
        normal = normal.replace("`-- windows-platform", windows_tree.format(",".join(normal_features)))
        development = development.replace("`-- windows-platform", windows_tree.format(",".join(test_features)))
        engine_edge = "|-- nevrotick-engine v0.0.0 (/work/source/crates/nevrotick-engine)|\n"
        development = development.replace(engine_edge, engine_edge + f"|   `-- {leaf} v0.0.0 ({leaf_path})|\n")
        development = development.replace(
            "`-- serde_json v1.0.154|std\n    |-- memchr v2.8.3|alloc,std\n    `-- zmij v1.0.23|",
            "|-- serde_json v1.0.154|std\n|   |-- memchr v2.8.3|alloc,std\n|   `-- zmij v1.0.23|",
        )
        development += f"`-- windows-sys v0.61.2|{','.join(test_features)}\n    `-- windows-link v0.2.1|\n"
        return policy, data, normal, development

    def test_windows_private_bindings_are_admitted_only_in_development(self):
        result = self.receive(self.windows_fixture(), Q.TARGETS[1])
        normal = result["contexts"]["windows-sys@0.61.2"]["target"][0]
        test = result["development_contexts"]["windows-sys@0.61.2"]["target"][0]
        development_only = {"Win32_Security_Authorization", "Win32_System_Diagnostics",
                            "Win32_System_Diagnostics_Etw", "Win32_System_Time"}
        self.assertEqual(set(test) - set(normal), development_only)
        policy, data, normal_tree, development = self.windows_fixture()
        with self.assertRaisesRegex(Q.Incomplete, "Unexpected metadata package"):
            self.receive((policy, data, normal_tree, development))
        # Use the Windows test closure behind the production edges.
        with self.assertRaisesRegex(Q.Incomplete, "feature contexts"):
            self.receive((policy, data, development.split("[dev-dependencies]", 1)[0], development), Q.TARGETS[1])
        # Test-only availability must not mask a missing production binding.
        for missing in ("Win32_Storage_FileSystem", "Win32_System_IO", "Win32_System_Pipes"):
            with self.subTest(missing=missing), self.assertRaisesRegex(Q.Incomplete, "feature contexts"):
                missing_normal = normal_tree.replace(f",{missing},", ",")
                self.assertNotEqual(missing_normal, normal_tree)
                self.receive((policy, data, missing_normal, development), Q.TARGETS[1])
        # Keep the unified metadata closure while an individual test-tree binding is missing.
        for missing in sorted(development_only):
            with self.subTest(missing=missing), self.assertRaisesRegex(Q.Incomplete, "feature contexts"):
                incomplete_tree = development.replace(f",{missing},", ",")
                if incomplete_tree == development:
                    incomplete_tree = development.replace(f",{missing}\n", "\n")
                self.assertNotEqual(incomplete_tree, development)
                self.receive((policy, data, normal_tree, incomplete_tree), Q.TARGETS[1])

    def test_windows_etw_requires_exact_resolved_edges(self):
        fixture = self.windows_fixture()
        edges = fixture[1]["resolve"]["nodes"][0]["deps"][-1]["dep_kinds"]
        edges.reverse()
        self.receive(fixture, Q.TARGETS[1])
        for invalid in ({"kind": None, "target": "cfg(windows)"}, {"kind": "dev", "target": None}):
            with self.subTest(invalid=invalid):
                edges[0] = invalid
                with self.assertRaisesRegex(Q.Incomplete, "dependency kinds"):
                    self.receive(fixture, Q.TARGETS[1])

    def test_native_observer_requires_a_direct_windows_normal_edge_in_metadata(self):
        fixture = self.windows_fixture()
        result = self.receive(fixture, Q.TARGETS[1])
        identity = "windows-platform-native@0.0.0"
        self.assertEqual(result["contexts"][identity], {"target": [[]]})
        self.assertEqual(result["development_contexts"][identity], {"target": [[]]})
        gnu = self.receive(self.fixture())
        self.assertNotIn(identity, gnu["contexts"])
        self.assertNotIn(identity, gnu["development_contexts"])
        changed_edge = next(dep for dep in fixture[1]["resolve"]["nodes"][0]["deps"]
                            if dep["pkg"] == "windows-platform-native")
        for wrong_kinds in ([], [{"kind": "dev", "target": "cfg(windows)"}],
                            [{"kind": "build", "target": "cfg(windows)"}],
                            [{"kind": None, "target": None}],
                            [{"kind": None, "target": "cfg(windows)"}] * 2):
            with self.subTest(wrong_kinds=wrong_kinds):
                changed_edge["dep_kinds"] = wrong_kinds
                with self.assertRaisesRegex(Q.Incomplete, "dependency kinds"):
                    self.receive(fixture, Q.TARGETS[1])

    def test_native_observer_trees_require_one_direct_normal_edge(self):
        policy, data, normal, development = self.windows_fixture()
        self.receive((policy, data, normal, development), Q.TARGETS[1])
        direct = "|-- windows-platform-native v0.0.0 (/work/source/crates/windows-platform-native)|\n"
        normal_without_direct = normal.replace(direct, "")
        development_without_direct = development.replace(direct, "")
        final_child = direct.replace("|-- ", "`-- ", 1)
        cases = (
            ("indirect normal", normal_without_direct + "    " + final_child, development),
            ("indirect development", normal, development_without_direct),
            ("normal build edge", normal_without_direct + "[build-dependencies]\n" + final_child, development),
            ("development build edge", normal, development_without_direct.replace(
                "[dev-dependencies]\n", "[build-dependencies]\n" + final_child + "[dev-dependencies]\n")),
            ("development dev edge", normal, development_without_direct.replace(
                "[dev-dependencies]\n", "[dev-dependencies]\n" + direct)),
            ("duplicate normal", normal.replace(direct, direct + direct), development),
            ("duplicate development", normal, development.replace(direct, direct + direct)),
        )
        for defect, normal_tree, development_tree in cases:
            with self.subTest(defect=defect):
                with self.assertRaisesRegex(Q.Incomplete, "native observer dependency"):
                    self.receive((policy, data, normal_tree, development_tree), Q.TARGETS[1])

    def test_healthy_distinct_graphs_keep_engine_test_only(self):
        result = self.receive(self.fixture())
        self.assertFalse(result["engine_in_production"])
        self.assertTrue(result["engine_in_tests"])
        self.assertNotIn("nevrotick-engine@0.0.0", result["contexts"])
        self.assertIn("nevrotick-engine@0.0.0", result["development_contexts"])

    def test_default_only_substitution_does_not_prove_engine_execution(self):
        policy, data, normal, development = self.fixture()
        with self.assertRaisesRegex(Q.Incomplete, "exact direct test-only edges"):
            self.receive((policy, data, normal, normal))
        without_engine = development.replace(
            "|-- nevrotick-engine v0.0.0 (/work/source/crates/nevrotick-engine)|\n", "",
        )
        with self.assertRaisesRegex(Q.Incomplete, "feature contexts"):
            self.receive((policy, data, normal, without_engine))

    def test_shared_diagnostics_target_requires_its_actual_declared_source(self):
        """Admit the real new consumer, refusing absent, foreign or gated substitutes."""
        self.assertTrue(self.receive(self.fixture())["engine_in_tests"])
        for defect in ("absent", "foreign-source", "wrong-kind", "gated"):
            with self.subTest(defect=defect):
                fixture = self.fixture()
                targets = fixture[1]["packages"][1]["targets"]
                diagnostic = next(target for target in targets if target["name"] == "diagnostics")
                if defect == "absent":
                    targets.remove(diagnostic)
                elif defect == "foreign-source":
                    diagnostic["src_path"] = "/work/source/crates/windows-platform/tests/service.rs"
                elif defect == "wrong-kind":
                    diagnostic["kind"] = ["example"]
                else:
                    diagnostic["required-features"] = ["unadmitted-profile"]
                with self.assertRaisesRegex(Q.Incomplete, "target set|gated local target"):
                    self.receive(fixture)

    def test_promoting_engine_edge_to_production_is_refused(self):
        fixture = self.fixture()
        fixture[1]["resolve"]["nodes"][0]["deps"][1]["dep_kinds"][0]["kind"] = None
        with self.assertRaisesRegex(Q.Incomplete, "development-only"):
            self.receive(fixture)

    def test_duplicate_root_dependency_does_not_hide_another_role(self):
        fixture = self.fixture()
        deps = fixture[1]["resolve"]["nodes"][0]["deps"]
        deps.insert(0, {"pkg": "nevrotick-engine", "dep_kinds": [{"kind": None, "target": None}]})
        with self.assertRaisesRegex(Q.Incomplete, "Duplicate standalone dependency"):
            self.receive(fixture)

    def test_production_tree_cannot_contain_hidden_test_edge(self):
        policy, data, _normal, development = self.fixture()
        with self.assertRaisesRegex(Q.Incomplete, "Dev edge in production"):
            self.receive((policy, data, development, development))

    def test_private_json_edges_remain_test_only(self):
        """Refuse promotion, omitted test edges and derive activation at real graph readers."""
        policy, data, normal, development = self.fixture()
        readback = self.receive((policy, data, normal, development))
        self.assertNotIn("serde_json@1.0.154", readback["contexts"])
        self.assertEqual(readback["development_contexts"]["serde@1.0.229"], {"target": [["std"]]})
        wrong_normal = normal + "`-- serde_json v1.0.154|std\n"
        for refusal, tree in (
            ("exact direct test-only edges", development.replace("|-- garde v0.23.0|derive\n", "")),
            ("feature contexts", development.replace("|-- serde v1.0.229|std", "|-- serde v1.0.229|derive,std")),
        ):
            with self.subTest(refusal=refusal), self.assertRaisesRegex(Q.Incomplete, refusal):
                self.receive((policy, data, normal, tree))
        with self.assertRaisesRegex(Q.Incomplete, "Unexpected tree identity"):
            self.receive((policy, data, wrong_normal, development))
        serde = next(row for row in data["resolve"]["nodes"][0]["deps"] if row["pkg"] == "serde")
        serde["dep_kinds"][0]["kind"] = None
        with self.assertRaisesRegex(Q.Incomplete, "development-only edges"):
            self.receive((policy, data, normal, development))

    def test_optional_ke_feature_is_not_admitted_by_a_dev_edge(self):
        fixture = self.fixture()
        fixture[1]["resolve"]["nodes"][2]["features"] = ["ke-association-prototype"]
        with self.assertRaisesRegex(Q.Incomplete, "Metadata features"):
            self.receive(fixture)

    def test_local_source_substitution_is_refused(self):
        fixture = self.fixture()
        fixture[1]["packages"][2]["manifest_path"] = "/work/source/foreign/Cargo.toml"
        with self.assertRaisesRegex(Q.Incomplete, "local metadata source"):
            self.receive(fixture)

    def test_injected_engine_target_is_refused(self):
        fixture = self.fixture()
        fixture[1]["packages"][2]["targets"].append({
            "name": "extra", "kind": ["bin"], "crate_types": ["bin"],
            "src_path": "/work/source/crates/nevrotick-engine/src/bin/extra.rs"})
        with self.assertRaisesRegex(Q.Incomplete, "target set"):
            self.receive(fixture)

    def test_graph_command_roster_requires_both_trees_on_each_target(self):
        commands = Q.graph_commands("/toolchain/bin/cargo")
        self.assertEqual(len(commands), 6)
        for offset, target in ((0, Q.TARGETS[0]), (3, Q.TARGETS[1])):
            self.assertIn(target, commands[offset])
            self.assertEqual(commands[offset + 1][commands[offset + 1].index("--edges") + 1], "normal,build")
            self.assertEqual(commands[offset + 2][commands[offset + 2].index("--edges") + 1], "normal,build,dev")


if __name__ == "__main__":
    unittest.main()
