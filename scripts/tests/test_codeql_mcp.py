"""Independent launcher/inventory controls using tiny owned payload fixtures.

No installed analyzer inventory is traversed and no MCP/CodeQL payload executes.
"""

import copy
import hashlib
import importlib.util
import io
import json
import os
from pathlib import Path
import shutil
import tempfile
import unittest
from unittest.mock import patch


ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("codeql_mcp_launcher", ROOT / "scripts/codeql-mcp.py")
LAUNCH = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(LAUNCH)


def sha256(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


class LauncherControls(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-codeql-admission-")
        self.addCleanup(self.temporary.cleanup)
        self.scratch = Path(self.temporary.name)
        self.checkout = self.scratch / "checkout"
        (self.checkout / "policy").mkdir(parents=True)
        self.base = self.scratch / "payload"
        self.work = self.scratch / "work"
        self.work.mkdir(mode=0o700)
        self.home = self.scratch / "home"
        self.stable = self.scratch / "stable"
        self.stable.mkdir()
        (self.base / "evidence").mkdir(parents=True)
        (self.base / "runtime").mkdir()
        (self.base / "runtime/node").write_bytes(b"nonexecutable node fixture")
        self.policy = json.loads((ROOT / "policy/codeql-mcp.json").read_text())
        self.policy["node_sha256"] = sha256(self.base / "runtime/node")
        self.policy["inventories"] = [self.make_inventory(name) for name in ("server", "rust-tools", "cli/codeql")]
        (self.work / "packs").mkdir()
        shutil.copytree(self.base / "rust-tools", self.work / "packs/rust-tools")
        for name in ("queries", "results", "policy/codeql", "home/.config/codeql", "fixtures"):
            (self.work / name).mkdir(parents=True, exist_ok=True)
        config = self.work / "policy/codeql/config"
        config.write_text("--ram 1024\n--threads 12\n")
        self.policy["config_sha256"] = sha256(config)
        (self.work / "home/.config/codeql/config").symlink_to("../../../policy/codeql/config")
        self.pointer = self.home / ".local/share/nevrotick/codeql-installation.json"
        self.pointer.parent.mkdir(parents=True)
        self.pointer.write_text(json.dumps({"schema_version": 1, "status": "installed",
                                            "root": str(self.base), "work": str(self.work)}))
        self.pointer.chmod(0o600)
        (self.checkout / "policy/analysis-tools.json").write_text(json.dumps({"inventories": [{"root": "stable"}]}))
        self.write_policy()
        self.patches = [patch.object(LAUNCH, "ROOT", self.checkout),
                        patch.object(Path, "home", return_value=self.home),
                        patch.object(LAUNCH.GUARD, "load_installation", return_value=(self.scratch / "analysis", self.stable)),
                        patch.object(LAUNCH.GUARD, "check_inventory", return_value=({}, {})),
                        patch.object(LAUNCH.RUNTIME, "_helpers", return_value={})]
        for item in self.patches:
            item.start()
            self.addCleanup(item.stop)

    def make_inventory(self, name):
        folder = self.base / name
        (folder / "bin").mkdir(parents=True)
        (folder / "bin/entry").write_bytes((name + " controlled bytes").encode())
        manifest = {"schema_version": 1, "installation": str(folder),
                    "files": {"bin/entry": sha256(folder / "bin/entry")}, "symlinks": {}}
        spec = {"root": name, "path": "evidence/" + name.replace("/", "-") + ".json"}
        self.write_inventory(spec, manifest)
        return spec

    def write_inventory(self, spec, manifest):
        path = self.base / spec["path"]
        path.write_text(json.dumps(manifest))
        spec["sha256"] = sha256(path)

    def manifest(self, index=0):
        spec = self.policy["inventories"][index]
        return spec, json.loads((self.base / spec["path"]).read_text())

    def write_policy(self):
        (self.checkout / "policy/codeql-mcp.json").write_text(json.dumps(self.policy))

    def state(self):
        return {"base": self.base, "work": self.work, "stable": self.stable,
                "policy": self.policy, "inventories": []}

    def test_complete_owned_installation_is_admitted_without_execution(self):
        with patch.object(LAUNCH.SUPERVISOR, "serve") as serve, patch.object(LAUNCH.RUNTIME.ENV, "run") as run:
            state = LAUNCH.admit()
        serve.assert_not_called()
        run.assert_not_called()
        self.assertEqual(state["work"], self.work)
        self.assertEqual({report["root"] for report in state["inventories"]}, {"server", "rust-tools", "cli/codeql"})

    def test_literal_publisher_symlink_inventory_and_file_drift(self):
        spec, manifest = self.manifest()
        alias = self.base / "server/bin/alias"
        alias.symlink_to("entry")
        manifest["symlinks"]["bin/alias"] = "entry"
        self.write_inventory(spec, manifest)
        self.assertEqual(LAUNCH.inventory(self.base, spec)["links"], 1)
        (self.base / "server/bin/entry").write_bytes(b"substituted payload")
        with self.assertRaisesRegex(LAUNCH.Incomplete, "file changed"):
            LAUNCH.inventory(self.base, spec)

    def test_inventory_bytes_are_authenticated_before_decode(self):
        spec, _ = self.manifest()
        (self.base / spec["path"]).write_bytes(b"not JSON")
        with patch.object(LAUNCH.GUARD, "decode", side_effect=AssertionError("must not parse")), \
                self.assertRaisesRegex(LAUNCH.Incomplete, "inventory digest"):
            LAUNCH.inventory(self.base, spec)

    def test_inventory_rejects_added_missing_and_special_files(self):
        spec, _ = self.manifest()
        extra = self.base / "server/unlisted"
        extra.write_bytes(b"not inventoried")
        with self.assertRaisesRegex(LAUNCH.Incomplete, "Unlisted"):
            LAUNCH.inventory(self.base, spec)
        extra.unlink()
        os.mkfifo(extra)
        with self.assertRaisesRegex(LAUNCH.Incomplete, "Special file"):
            LAUNCH.inventory(self.base, spec)
        extra.unlink()
        (self.base / "server/bin/entry").unlink()
        with self.assertRaisesRegex(LAUNCH.Incomplete, "incomplete"):
            LAUNCH.inventory(self.base, spec)

    def test_inventory_root_schema_and_traversal_cannot_be_substituted(self):
        spec, original = self.manifest()
        values = [dict(original, installation=str(self.scratch)), dict(original, schema_version=2),
                  dict(original, files={"../outside": "a" * 64})]
        for value in values:
            with self.subTest(value=value), self.assertRaises(LAUNCH.Incomplete):
                self.write_inventory(spec, value)
                LAUNCH.inventory(self.base, spec)

    def test_symlink_text_and_resolved_target_must_match_inventory(self):
        spec, manifest = self.manifest()
        alias = self.base / "server/bin/alias"
        alias.symlink_to("./entry")
        manifest["symlinks"]["bin/alias"] = "entry"
        self.write_inventory(spec, manifest)
        with self.assertRaisesRegex(LAUNCH.Incomplete, "link identity"):
            LAUNCH.inventory(self.base, spec)
        alias.unlink()
        outside = self.scratch / "outside"
        outside.write_bytes(b"outside")
        alias.symlink_to(outside)
        manifest["symlinks"]["bin/alias"] = str(outside)
        self.write_inventory(spec, manifest)
        with self.assertRaisesRegex(LAUNCH.Incomplete, "link identity"):
            LAUNCH.inventory(self.base, spec)

    def test_root_directory_redirection_is_rejected(self):
        spec, _ = self.manifest()
        original = self.base / "server"
        moved = self.scratch / "moved"
        original.rename(moved)
        original.symlink_to(moved, target_is_directory=True)
        with self.assertRaisesRegex(LAUNCH.Incomplete, "escapes or redirects"):
            LAUNCH.inventory(self.base, spec)

    def test_copied_runtime_pack_is_checked_against_same_pinned_bytes(self):
        spec = self.policy["inventories"][1]
        copied = self.work / "packs/rust-tools"
        self.assertEqual(LAUNCH.inventory(self.base, spec, copy=copied)["files"], 1)
        (copied / "bin/entry").write_bytes(b"copy drift")
        with self.assertRaisesRegex(LAUNCH.Incomplete, "file changed"):
            LAUNCH.inventory(self.base, spec, copy=copied)

    def test_profile_transport_and_tool_drift_fail_before_inventory(self):
        original = copy.deepcopy(self.policy)
        changes = [lambda p: p.update(transport="http"), lambda p: p.update(ci_codeql="enabled"),
                   lambda p: p.update(server_version="latest"), lambda p: p.update(cli_version="latest"),
                   lambda p: p.update(rust_library="latest"), lambda p: p["profile"].update(threads=100),
                   lambda p: p["allowed_tools"].append("codeql_database_create"),
                   lambda p: p["allowed_tools"].pop(),
                   lambda p: p["allowed_tools"].append(p["allowed_tools"][0])]
        for change in changes:
            self.policy = copy.deepcopy(original)
            change(self.policy)
            self.write_policy()
            with self.subTest(change=change), patch.object(LAUNCH, "inventory") as inventory, \
                    self.assertRaises(LAUNCH.Incomplete):
                LAUNCH.admit()
            inventory.assert_not_called()

    def test_private_pointer_mode_and_config_redirection_fail_closed(self):
        self.pointer.chmod(0o644)
        with self.assertRaisesRegex(LAUNCH.Incomplete, "mode0600"):
            LAUNCH.admit()
        self.pointer.chmod(0o600)
        link = self.work / "home/.config/codeql/config"
        link.unlink()
        replacement = self.work / "alternate-config"
        replacement.write_bytes((self.work / "policy/codeql/config").read_bytes())
        link.symlink_to("../../../alternate-config")
        with self.assertRaisesRegex(LAUNCH.Incomplete, "configuration redirects"):
            LAUNCH.admit()

    def test_payload_checkout_and_work_overlap_is_rejected(self):
        record = json.loads(self.pointer.read_text())
        record["root"] = str(self.work)
        self.pointer.write_text(json.dumps(record))
        with self.assertRaisesRegex(LAUNCH.Incomplete, "must be separate"):
            LAUNCH.admit()

    def test_command_contains_only_explicit_env_and_readonly_inputs(self):
        with patch.dict(os.environ, {"CODEQL_TEST_SECRET": "must-not-copy"}), \
                patch.object(LAUNCH.RUNTIME, "sandbox_command", return_value=["sandbox"]) as sandbox:
            self.assertEqual(LAUNCH.command(self.state(), ["owned-argv"]), ["sandbox"])
        options = sandbox.call_args.kwargs
        self.assertEqual(options["readonly"], [self.base, self.checkout, self.stable])
        self.assertTrue({self.work / "policy", self.work / "packs"}.issubset(options["sealed"]))
        self.assertEqual(options["environment"]["TRANSPORT_MODE"], "stdio")
        self.assertEqual(options["environment"]["CODEQL_QUERY_LOG_DIR"], "/work/results")
        self.assertEqual(options["environment"].get("CODEQL_THREADS"), "12")
        self.assertEqual(options["environment"]["JAVA_TOOL_OPTIONS"],
                         "-Xmx1024m -XX:ActiveProcessorCount=12")
        self.assertNotIn("CODEQL_TEST_SECRET", options["environment"])
        self.assertEqual(options["cpu_seconds"], LAUNCH.PROFILE["cpu_seconds"])

    def test_normal_server_start_emits_no_preflight_json_on_protocol_stdout(self):
        output = io.StringIO()
        with patch.object(LAUNCH, "admit", return_value=self.state()), \
                patch.object(LAUNCH, "command", return_value=["bounded-command"]), \
                patch.object(LAUNCH.SUPERVISOR, "serve", return_value=0) as serve, \
                patch("sys.stdout", output):
            self.assertEqual(LAUNCH.main([]), 0)
        self.assertEqual(output.getvalue(), "")
        self.assertEqual(set(serve.call_args.args[1]), LAUNCH.ALLOWED_TOOLS)
        self.assertEqual(serve.call_args.kwargs["environment"], {"PATH": "/usr/bin:/bin", "LANG": "C.UTF-8"})

    def test_failed_admission_never_connects_and_reports_only_stderr(self):
        self.pointer.chmod(0o644)
        output, errors = io.StringIO(), io.StringIO()
        with patch.object(LAUNCH.SUPERVISOR, "serve") as serve, patch.object(LAUNCH.RUNTIME.ENV, "run") as run, \
                patch("sys.stdout", output), patch("sys.stderr", errors):
            self.assertEqual(LAUNCH.main([]), 1)
        serve.assert_not_called()
        run.assert_not_called()
        self.assertEqual(output.getvalue(), "")
        self.assertIn("CodeQL MCP incomplete", errors.getvalue())

    def test_extraction_requires_existing_cargo_inputs_and_new_destination(self):
        with patch.object(LAUNCH, "admit", return_value=self.state()), \
                patch.object(LAUNCH.RUNTIME.ENV, "run") as run, patch("sys.stderr", io.StringIO()):
            self.assertEqual(LAUNCH.main(["--create-database", str(self.checkout), "--name", "fixture"]), 1)
        run.assert_not_called()
        self.assertFalse((self.checkout / "Cargo.toml").exists())
        self.assertFalse((self.checkout / "Cargo.lock").exists())

    def test_results_symlink_cannot_alias_query_or_expected_input(self):
        protected = self.work / "queries/test.expected"
        protected.write_text("authored expectation\n")
        (self.work / "results/output").symlink_to("../queries/test.expected")
        # The general work guard allows internal file links, so the launcher
        # needs the narrower result/input separation before invoking a tool.
        LAUNCH.RUNTIME._owned_work(self.work, self.checkout)
        with patch.object(LAUNCH.RUNTIME, "sandbox_command") as sandbox, self.assertRaises(LAUNCH.Incomplete):
            LAUNCH.command(self.state(), ["owned-argv"])
        sandbox.assert_not_called()
        self.assertEqual(protected.read_text(), "authored expectation\n")

    def test_results_hardlink_cannot_alias_query_or_expected_input(self):
        protected = self.work / "queries/test.expected"
        protected.write_text("authored expectation\n")
        (self.work / "results/output").hardlink_to(protected)
        LAUNCH.RUNTIME._owned_work(self.work, self.checkout)
        with patch.object(LAUNCH.RUNTIME, "sandbox_command") as sandbox, self.assertRaises(LAUNCH.Incomplete):
            LAUNCH.command(self.state(), ["owned-argv"])
        sandbox.assert_not_called()
        self.assertEqual(protected.read_text(), "authored expectation\n")

    def test_result_guard_accepts_regular_nested_outputs(self):
        nested = self.work / "results/run-1"
        nested.mkdir()
        (nested / "output.bqrs").write_bytes(b"owned output")
        LAUNCH.check_results(self.work)

    def test_results_directory_redirect_and_special_entries_are_rejected(self):
        results = self.work / "results"
        results.rmdir()
        results.symlink_to("queries", target_is_directory=True)
        with self.assertRaisesRegex(LAUNCH.Incomplete, "canonical directory"):
            LAUNCH.check_results(self.work)
        results.unlink()
        results.mkdir()
        os.mkfifo(results / "fifo")
        with self.assertRaisesRegex(LAUNCH.Incomplete, "special files"):
            LAUNCH.check_results(self.work)


if __name__ == "__main__":
    unittest.main()
