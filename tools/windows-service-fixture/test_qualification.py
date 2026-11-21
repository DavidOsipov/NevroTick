#!/usr/bin/env python3
"""Controls for the service-local portable host admission and sandbox adapter."""
from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import tempfile
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
                with patch.object(Q, "check_core_dumps"), patch.object(Q, "check_service_tools") as check, patch.object(
                    Q.C.PRODUCT.ENV, "run", return_value={"exit_code": 0, "output": "controlled", "failure": None}
                ) as execute:
                    result = Q.run_command(work, [str(stable / "bin/cargo"), "--version"], admitted,
                                           vendor, source, timeout=180, mutation=mutation)
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
                    "service", "scm", "privilege_observation", "privilege_observation_contracts")]],
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
        data = {"packages": packages, "resolve": {"nodes": nodes}}
        normal = ("nevrotick-windows-service-fixture v0.0.0 (/work/source/tools/windows-service-fixture)|\n"
                  "`-- windows-platform v0.0.0 (/work/source/crates/windows-platform)|\n")
        development = normal + ("[dev-dependencies]\n"
                               "`-- nevrotick-engine v0.0.0 (/work/source/crates/nevrotick-engine)|\n")
        return policy, data, normal, development

    def receive(self, fixture):
        """Use actual metadata, target and both tree-role checks, with no mocked reader."""
        policy, data, normal, development = fixture
        return Q.graph_row(policy, Q.TARGETS[0], json.dumps(data), normal, development)

    def test_healthy_distinct_graphs_keep_engine_test_only(self):
        result = self.receive(self.fixture())
        self.assertFalse(result["engine_in_production"])
        self.assertTrue(result["engine_in_tests"])
        self.assertNotIn("nevrotick-engine@0.0.0", result["contexts"])
        self.assertIn("nevrotick-engine@0.0.0", result["development_contexts"])

    def test_default_only_substitution_does_not_prove_engine_execution(self):
        policy, data, normal, _development = self.fixture()
        with self.assertRaisesRegex(Q.Incomplete, "feature contexts"):
            self.receive((policy, data, normal, normal))

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
