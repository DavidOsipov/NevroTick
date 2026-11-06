"""Independent native SDK custody controls; no SDK library is loaded."""
import hashlib
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("sdk", ROOT / "scripts/slint_linux_sdk.py")
SDK = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(SDK)


class SdkControls(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.sdk = self.root / "sdk"
        (self.sdk / "lib").mkdir(parents=True)
        (self.sdk / "lib/native.so.1").write_bytes(b"Inert custody fixture; never loaded")
        (self.sdk / "lib/native.so").symlink_to("native.so.1")
        self.policy = self.root / "policy.json"
        self.tool = self.root / "tool"
        self.tool.write_bytes(b"Inert tool fixture; never executed")
        data = dict(schema_version=1,scope="Inert test control",**SDK.inventory(self.sdk),
                    pkg_config_libdir="/native/usr/lib/x86_64-linux-gnu/pkgconfig:/native/usr/share/pkgconfig",
                    pkg_config_sysroot_dir="/native",system_runtime_files={},
                    system_runtime_aliases={},source_files={},
                    pkg_config={"path":str(self.tool),"sha256":SDK.digest(self.tool)})
        self.policy.write_text(json.dumps(data))
        pin = hashlib.sha256(self.policy.read_bytes()).hexdigest()
        self.patcher = patch.object(SDK,"REVIEWED_POLICY_SHA256",pin)
        self.patcher.start()
        self.addCleanup(self.patcher.stop)

    def test_exact_regular_files_and_relative_alias_admit(self):
        report = SDK.admit(self.sdk,self.policy)
        self.assertEqual(report["inventory"]["symlinks"],{"lib/native.so":"native.so.1"})
        self.assertEqual(report["inventory"]["directories"],["lib"])

    def test_self_supplied_policy_cannot_change_the_independent_pin(self):
        self.policy.write_text(self.policy.read_text()+" ")
        with self.assertRaisesRegex(SDK.Incomplete,"independently"):
            SDK.admit(self.sdk,self.policy)

    def test_host_library_alias_and_source_inputs_are_independently_bound(self):
        library = self.root / "runtime.so.1"
        library.write_bytes(b"Inert runtime fixture")
        alias = self.root / "runtime.so"
        alias.symlink_to("runtime.so.1")
        font = self.root / "font.ttf"
        font.write_bytes(b"Inert font fixture; never parsed")
        policy = json.loads(self.policy.read_text())
        policy.update(system_runtime_files={str(library):SDK.digest(library)},
                      system_runtime_aliases={str(alias):"runtime.so.1"},
                      source_files={"font.ttf":SDK.digest(font)})
        self.policy.write_text(json.dumps(policy))
        with patch.object(SDK,"REVIEWED_POLICY_SHA256",SDK.digest(self.policy)):
            SDK.admit(self.sdk,self.policy,self.root)
            for target in (library,font):
                original = target.read_bytes()
                target.write_bytes(b"Substitution")
                with self.assertRaisesRegex(SDK.Incomplete,"differ"):
                    SDK.admit(self.sdk,self.policy,self.root)
                target.write_bytes(original)
            alias.unlink()
            alias.symlink_to("different.so")
            with self.assertRaisesRegex(SDK.Incomplete,"alias"):
                SDK.admit(self.sdk,self.policy,self.root)

    def test_unlisted_font_never_reaches_the_scanned_fixture_directory(self):
        font_name = "crates/nevrotick-ui/tests/fixtures/fonts/admitted.ttf"
        font = self.root / font_name
        font.parent.mkdir(parents=True)
        font.write_bytes(b"Inert admitted font fixture; never parsed")
        policy = json.loads(self.policy.read_text())
        policy["source_files"] = {font_name:SDK.digest(font)}
        self.policy.write_text(json.dumps(policy))
        with patch.object(SDK,"REVIEWED_POLICY_SHA256",SDK.digest(self.policy)):
            SDK.admit(self.sdk,self.policy,self.root)
            (font.parent / "extra.ttf").write_bytes(b"Unreviewed font-parser input")
            with self.assertRaisesRegex(SDK.Incomplete,"font directory"):
                SDK.admit(self.sdk,self.policy,self.root)

    def test_changed_added_file_or_tool_reject(self):
        for target in (self.sdk/"lib/native.so.1",self.sdk/"extra",self.tool):
            with self.subTest(path=target):
                existed = target.exists()
                before = target.read_bytes() if existed else None
                target.write_bytes(b"Substitution")
                with self.assertRaisesRegex(SDK.Incomplete,"differ"):
                    SDK.admit(self.sdk,self.policy)
                if existed: target.write_bytes(before)
                else: target.unlink()

    def test_absolute_escaping_dangling_and_special_entries_reject(self):
        link = self.sdk/"lib/native.so"
        for target in (str(self.tool),"../../tool","missing"):
            with self.subTest(target=target):
                link.unlink()
                link.symlink_to(target)
                with self.assertRaises((SDK.Incomplete,OSError)):
                    SDK.admit(self.sdk,self.policy)
        link.unlink()
        import os
        os.mkfifo(link)
        with self.assertRaisesRegex(SDK.Incomplete,"special"):
            SDK.admit(self.sdk,self.policy)

    def test_entry_and_aggregate_byte_budgets_apply_before_admission(self):
        for field,value in (("MAX_FILES",1),("MAX_BYTES",1)):
            with self.subTest(bound=field),patch.object(SDK,field,value):
                with self.assertRaises(SDK.Incomplete): SDK.inventory(self.sdk)

    def test_sandbox_selects_only_fixed_native_paths(self):
        args = SDK.sandbox_arguments(self.sdk)
        self.assertEqual(args[:3],["--ro-bind",str(self.sdk),"/native"])
        settings = {args[index+1]:args[index+2] for index,item in enumerate(args) if item=="--setenv"}
        self.assertEqual(settings["LD_LIBRARY_PATH"],"/native/usr/lib/x86_64-linux-gnu")
        self.assertEqual(settings["PKG_CONFIG_SYSROOT_DIR"],"/native")
        self.assertEqual(settings["PKG_CONFIG"],"/usr/bin/pkgconf")
        self.assertEqual(settings["FONTCONFIG_FILE"],"/work/fixture/crates/nevrotick-ui/tests/fixtures/fonts.conf")
        self.assertTrue(settings["SLINT_DEFAULT_FONT"].endswith("/Inter-VariableFont.ttf"))


if __name__ == "__main__":
    unittest.main()
