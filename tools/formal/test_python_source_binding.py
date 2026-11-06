"""Source-execution custody controls; no compiler, solver or admission commands."""
from __future__ import annotations

import importlib.util
import os
from pathlib import Path
import py_compile
import sys
import tempfile
import types
import unittest


class PythonSourceBindingTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-python-binding-")
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.helper_path = self.root / "python_source_binding.py"
        self.helper_path.write_bytes(Path(__file__).with_name("python_source_binding.py").read_bytes())
        spec = importlib.util.spec_from_file_location("private_source_binding_helper", self.helper_path)
        self.helper = importlib.util.module_from_spec(spec)
        # Load the test helper from source too; preserve the same module filename.
        exec(compile(self.helper_path.read_bytes(), str(self.helper_path), "exec", dont_inherit=True),
             self.helper.__dict__)
        self.controller = self.root / "controller.py"
        self.controller.write_text("import sys\nbinding = helper.SourceBinding(ROOT, sys._getframe())\n")
        namespace = {"__file__": str(self.controller), "helper": self.helper, "ROOT": self.root}
        exec(compile(self.controller.read_bytes(), str(self.controller), "exec", dont_inherit=True), namespace)
        self.binding = namespace["binding"]
        self.module_names = []
        self.addCleanup(self.clear_modules)

    def clear_modules(self):
        for name in self.module_names:
            sys.modules.pop(name, None)

    def write(self, name, source):
        path = self.root / name
        path.write_text(source)
        return path

    def load(self, path, name="private_binding_subject"):
        self.module_names.append(name)
        return self.binding.load_module(name, path)

    def inventory(self):
        return tuple(self.binding.snapshot())

    def legacy_parent(self):
        return self.write("legacy.py", """import importlib.util
from pathlib import Path
spec = importlib.util.spec_from_file_location('private_legacy_child', Path(__file__).with_name('child.py'))
child = importlib.util.module_from_spec(spec)
spec.loader.exec_module(child)
""")

    def stale_bytecode(self):
        child = self.write("child.py", "value = 1\n")
        before = child.stat()
        py_compile.compile(str(child), doraise=True, invalidation_mode=py_compile.PycInvalidationMode.TIMESTAMP)
        child.write_text("value = 2\n")
        os.utime(child, ns=(before.st_atime_ns, before.st_mtime_ns))
        # Prove that this is a discriminating stale-cache fixture.
        spec = importlib.util.spec_from_file_location("old_cache_control", child)
        cached = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(cached)
        self.assertEqual(cached.value, 1)
        return child

    def test_full_module_closure_and_independent_snapshot(self):
        child = self.write("child.py", "value = 7\n")
        parent = self.load(self.legacy_parent())
        self.assertEqual(parent.child.value, 7)
        self.assertIs(parent.__source_binding__, self.binding)
        expected = {"python_source_binding.py", "controller.py", "legacy.py", "child.py"}
        self.assertEqual(set(self.binding.snapshot()), expected)
        self.binding.verify(expected, self.binding.snapshot())
        altered = self.binding.snapshot()
        altered[child.name]["sha256"] = "0" * 64
        self.binding.verify(expected)
        with self.assertRaisesRegex(self.helper.SourceBindingError, "Frozen"):
            self.binding.verify(expected, altered)
        with self.assertRaisesRegex(self.helper.SourceBindingError, "missing from inventory"):
            self.binding.verify(expected - {"child.py"})

    def test_direct_loader_ignores_valid_stale_bytecode_and_existing_module(self):
        child = self.stale_bytecode()
        name = "private_binding_stale_direct"
        old = types.ModuleType(name)
        old.value = 99
        sys.modules[name] = old
        loaded = self.load(child, name)
        self.assertEqual(loaded.value, 2)
        self.assertIs(sys.modules[name], loaded)
        self.binding.verify(self.inventory())

    def test_transitive_legacy_loader_rejects_stale_bytecode_before_body(self):
        self.stale_bytecode()
        with self.assertRaisesRegex(self.helper.SourceBindingError, "Executing Python code differs"):
            self.load(self.legacy_parent())
        self.assertNotIn("private_binding_subject", sys.modules)

    def test_controller_constants_nested_functions_and_classes_match_full_code(self):
        baseline = self.controller.read_text()
        for original, changed in (
            ("VALUE = 1\n", "VALUE = 2\n"),
            ("def outer():\n    def inner():\n        return 1\n    return inner\n",
             "def outer():\n    def inner():\n        return 2\n    return inner\n"),
            ("class Item:\n    value = 1\n", "class Item:\n    value = 2\n"),
        ):
            with self.subTest(original=original):
                old_code = compile(baseline + original, str(self.controller), "exec", dont_inherit=True)
                self.controller.write_text(baseline + changed)
                with self.assertRaisesRegex(self.helper.SourceBindingError, "Executing Python code differs"):
                    exec(old_code, {"helper": self.helper, "ROOT": self.root})
                # The corresponding actual source remains a healthy neighbor.
                exec(compile(self.controller.read_bytes(), str(self.controller), "exec", dont_inherit=True),
                     {"helper": self.helper, "ROOT": self.root})

    def test_postload_controller_helper_and_transitive_comment_drift_rejected(self):
        self.write("child.py", "value = 1\n")
        self.load(self.legacy_parent())
        for name in ("controller.py", "python_source_binding.py", "child.py"):
            path = self.root / name
            before = path.read_bytes()
            with self.subTest(name=name):
                path.write_bytes(before + b"# later edit\n")
                with self.assertRaisesRegex(self.helper.SourceBindingError, "Loaded Python source changed"):
                    self.binding.verify(self.inventory())
                path.write_bytes(before)
                self.binding.verify(self.inventory())

    def test_wrapper_binding_and_nested_source_load(self):
        self.write("child.py", "value = 3\n")
        wrapper = self.write("wrapper.py", """import sys
from pathlib import Path
__source_binding__.bind_frame(sys._getframe())
child = __source_binding__.load_module('private_nested_child', Path(__file__).with_name('child.py'))
""")
        self.module_names.append("private_nested_child")
        loaded = self.load(wrapper)
        self.assertEqual(loaded.child.value, 3)
        self.binding.verify(self.inventory())
        self.assertIn("wrapper.py", self.inventory())

    def test_transitive_module_cannot_hide_behind_a_foreign_code_filename(self):
        self.write("child.py", "value = 1\n")
        parent = self.write("foreign_loader.py", """from pathlib import Path
child = Path(__file__).with_name('child.py')
exec(compile(child.read_bytes(), '/tmp/unrelated-origin.py', 'exec'), {'__file__': str(child)})
""")
        with self.assertRaisesRegex(self.helper.SourceBindingError, "Executing Python code differs"):
            self.load(parent)

    def test_prior_profile_and_global_loader_restored_on_success_and_failure(self):
        previous = sys.getprofile()
        calls = []

        def observer(frame, event, argument):
            if len(calls) < 20:
                calls.append(event)

        original_loader = importlib.util.spec_from_file_location
        sys.setprofile(observer)
        try:
            self.load(self.write("good.py", "value = 1\n"))
            self.assertIs(sys.getprofile(), observer)
            with self.assertRaisesRegex(RuntimeError, "intentional body failure"):
                self.load(self.write("bad.py", "raise RuntimeError('intentional body failure')\n"))
            self.assertIs(sys.getprofile(), observer)
            self.assertIs(importlib.util.spec_from_file_location, original_loader)
            self.assertTrue(calls)
        finally:
            sys.setprofile(previous)

    def test_failure_restores_previous_module_and_profile_even_when_hook_removed(self):
        name = "private_binding_old_module"
        old = types.ModuleType(name)
        sys.modules[name] = old
        before = sys.getprofile()
        with self.assertRaisesRegex(RuntimeError, "intentional"):
            self.load(self.write("bad.py", "raise RuntimeError('intentional')\n"), name)
        self.assertIs(sys.modules[name], old)
        with self.assertRaisesRegex(self.helper.SourceBindingError, "observation was replaced"):
            self.load(self.write("unhook.py", "import sys\nsys.setprofile(None)\n"), name)
        self.assertIs(sys.getprofile(), before)
        self.assertIs(sys.modules[name], old)

    def test_alias_special_outside_and_oversize_inputs_are_rejected(self):
        target = self.write("good.py", "value = 1\n")
        alias = self.root / "alias.py"
        alias.symlink_to(target)
        with self.assertRaises(self.helper.SourceBindingError):
            self.load(alias)
        alias.unlink()
        os.link(target, alias)
        with self.assertRaisesRegex(self.helper.SourceBindingError, "aliased"):
            self.load(target)
        alias.unlink()
        fifo = self.root / "fifo.py"
        os.mkfifo(fifo)
        with self.assertRaisesRegex(self.helper.SourceBindingError, "special"):
            self.load(fifo)
        with self.assertRaises(self.helper.SourceBindingError):
            self.load(Path(__file__).resolve())
        large = self.root / "large.py"
        with large.open("wb") as stream:
            stream.truncate(self.helper.MAX_SOURCE_BYTES + 1)
        with self.assertRaisesRegex(self.helper.SourceBindingError, "oversized"):
            self.load(large)
        self.assertEqual(self.load(target).value, 1)


if __name__ == "__main__":
    unittest.main()
