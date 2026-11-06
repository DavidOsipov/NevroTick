"""Controller-source race controls; no admission, verifier or native job runs."""
from __future__ import annotations

import importlib.util
import os
from pathlib import Path
import shutil
import sys
import tempfile
import unittest
from unittest.mock import patch


ROOT = Path(__file__).resolve().parents[2]
CONTROLLER = "tools/formal/verify_fraction_verus_bridge.py"


def load(path, name):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


BASE = load(ROOT / CONTROLLER, "source_binding_regression_baseline")


class SourceBindingRegressionTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="nevrotick-source-binding-regression-")
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.addCleanup(self.restore_permissions)
        self.checkout = self.root / "checkout"
        names = set(BASE.SOURCE_NAMES) | {
            "tools/formal/verify_fraction_bit_length.py",
            "tools/formal/verify_fraction_constructor_post.py",
            "tools/formal/test_verify_fraction_bit_length.py",
            "tools/formal/test_verify_fraction_constructor_post.py",
            "tools/formal/fraction_verus_bridge_lemmas.rs",
        }
        for name in names:
            target = self.checkout / name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(ROOT / name, target)
        self.bridge = load(self.checkout / CONTROLLER, "source_binding_regression_copy")

    def restore_permissions(self):
        for directory, folders, files in os.walk(self.root):
            os.chmod(directory, 0o700)
            for name in folders:
                os.chmod(Path(directory) / name, 0o700)
            for name in files:
                os.chmod(Path(directory) / name, 0o600)

    def freeze(self, leaf):
        work = self.root / leaf
        work.mkdir(mode=0o700)
        with patch.object(self.bridge, "WORK", work), \
             patch.object(self.bridge, "remaining", return_value=100):
            return self.bridge.freeze_sources(work)

    def test_unchanged_loaded_sources_freeze_successfully(self):
        original, frozen = self.freeze("healthy")
        self.assertEqual(original[CONTROLLER]["sha256"], frozen[CONTROLLER]["sha256"])
        self.assertEqual(set(self.bridge.SOURCE_BINDING.snapshot()), {
            CONTROLLER, "tools/formal/python_source_binding.py",
            "tools/formal/fraction_verus_bridge_projection.py", "tools/formal/verus_pilot.py",
            "tools/formal/verus_pilot_acquire.py", "scripts/check-analysis.py",
            "scripts/analysis-runtime.py", "scripts/check-environment.py",
            "scripts/analysis-controls.py", "tools/crypto-aes-siv-witness/fuzz/input_custody.py",
        })

    def test_changed_controller_after_import_is_rejected_before_freezing(self):
        path = self.checkout / CONTROLLER
        data = path.read_bytes()
        anchor = b'    modes = ({name: "exec" for name in FUNCTIONS}'
        self.assertEqual(data.count(anchor), 1)
        path.write_bytes(data.replace(anchor, b'    raise RuntimeError("changed validator")\n' + anchor))
        with self.assertRaisesRegex(RuntimeError, "Loaded Python source changed"):
            self.freeze("changed-controller")

    def test_changed_immediate_and_transitive_helpers_are_rejected(self):
        for name in ("tools/formal/fraction_verus_bridge_projection.py",
                     "scripts/check-environment.py",
                     "tools/crypto-aes-siv-witness/fuzz/input_custody.py"):
            with self.subTest(name=name):
                path = self.checkout / name
                data = path.read_bytes()
                try:
                    path.write_bytes(data + b"\n# edited after module execution\n")
                    with self.assertRaisesRegex(RuntimeError, "Loaded Python source changed"):
                        self.freeze("changed-" + path.stem)
                finally:
                    path.write_bytes(data)

    def test_both_entrypoints_bind_themselves_and_bypass_a_cached_bridge(self):
        for leaf, recipe_name in (("bit_length", "_BIT_LENGTH_RECIPE"),
                                  ("constructor_post", "_CONSTRUCTOR_POST_RECIPE")):
            with self.subTest(leaf=leaf):
                relative = "tools/formal/verify_fraction_" + leaf + ".py"
                path = self.checkout / relative
                with patch.dict(sys.modules, {"verify_fraction_verus_bridge": object()}):
                    entry = load(path, "source_binding_entrypoint_" + leaf)
                bridge = entry._bridge
                recipe = getattr(bridge, recipe_name)
                bridge.original_sources(_recipe=recipe)
                self.assertIn(relative, bridge.SOURCE_BINDING.snapshot())
                self.assertIn(CONTROLLER, bridge.SOURCE_BINDING.snapshot())
                data = path.read_bytes()
                try:
                    path.write_bytes(data + b"\n# entrypoint edited after loading\n")
                    with self.assertRaisesRegex(RuntimeError, "Loaded Python source changed"):
                        bridge.original_sources(_recipe=recipe)
                finally:
                    path.write_bytes(data)


if __name__ == "__main__":
    unittest.main()
