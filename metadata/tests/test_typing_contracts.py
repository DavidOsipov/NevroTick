"""Resolve the authored annotation interfaces through an independent consumer."""

from __future__ import annotations

import ast
import importlib
import inspect
from pathlib import Path
import sys
import typing
import unittest


REPOSITORY = Path(__file__).resolve().parents[2]
MODULES = ("metadata", "metadata.pipeline", "metadata.pipeline.__main__",
           "metadata.pipeline.contract", "metadata.pipeline.delivery", "metadata.pipeline.fields",
           "metadata.pipeline.generate", "metadata.pipeline.text_policy")
PUBLIC_FUNCTIONS = {
    "metadata.pipeline.__main__": {"main"},
    "metadata.pipeline.contract": {"load_metadata", "validate_model"},
    "metadata.pipeline.delivery": {"delivery_contract", "verify_delivery"},
    "metadata.pipeline.generate": {"field_counts", "build_outputs", "generate", "check"},
    "metadata.pipeline.text_policy": {"require_text", "validate_source"},
}


class AnnotationContractTests(unittest.TestCase):
    def test_owned_annotations_are_resolved_and_public_interfaces_are_present(self) -> None:
        """Every authored definition remains readable by real type-hint consumers."""
        sys.path.insert(0, str(REPOSITORY))
        try:
            modules = {name: importlib.import_module(name) for name in MODULES}
        finally:
            sys.path.remove(str(REPOSITORY))
        observed = {}
        for module_name, module in modules.items():
            relative = module_name.replace(".", "/")
            expected = REPOSITORY / (relative + ("/__init__.py" if hasattr(module, "__path__") else ".py"))
            self.assertEqual(Path(module.__file__).resolve(), expected.resolve())
            source = ast.parse(expected.read_text(encoding="utf-8"))
            authored_functions = {node.name for node in source.body if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef))}
            authored_classes = {node.name for node in source.body if isinstance(node, ast.ClassDef)}
            owned = {name: value for name, value in inspect.getmembers(module)
                     if (inspect.isfunction(value) or inspect.isclass(value)) and value.__module__ == module_name}
            self.assertEqual({name for name, value in owned.items() if inspect.isfunction(value)}, authored_functions)
            self.assertEqual({name for name, value in owned.items() if inspect.isclass(value)}, authored_classes)
            self.assertLessEqual(PUBLIC_FUNCTIONS.get(module_name, set()), authored_functions)
            before = {name: id(value) for name, value in vars(module).items()}
            for name, value in owned.items():
                hints = typing.get_type_hints(value, globalns=vars(module), include_extras=True)
                with self.subTest(module=module_name, definition=name):
                    observed[(module_name, name)] = hints
                    if inspect.isfunction(value):
                        self.assertIsNot(inspect.signature(value).return_annotation, inspect.Signature.empty)
                        self.assertIn("return", hints)
            self.assertEqual({name: id(value) for name, value in vars(module).items()}, before)
        self.assertTrue(observed)
        contract = modules["metadata.pipeline.contract"]
        generator = modules["metadata.pipeline.generate"]
        # Named callers retain their independently specified shape, beside the
        # whole authored-definition resolution check above.
        self.assertEqual(observed[("metadata.pipeline.__main__", "main")],
                         {"arguments": list[str] | None, "return": int})
        self.assertEqual(observed[("metadata.pipeline.contract", "load_metadata")],
                         {"root": Path, "source": Path | None, "stage": str, "return": contract.Metadata})
        self.assertEqual(observed[("metadata.pipeline.generate", "generate")],
                         {"root": Path, "output": Path, "source": Path | None, "stage": str,
                          "return": generator.CommandReport})


if __name__ == "__main__":
    unittest.main()
