# SPDX-License-Identifier: GPL-2.0-only
"""Independent command-line and programmatic admission contracts."""

from __future__ import annotations

import contextlib
import importlib
import io
import json
import shutil
import subprocess
import sys

from test_pipeline import REPOSITORY, SourceFixture


class CliContractTests(SourceFixture):
    def test_imported_entrypoint_does_not_run_the_command_parser(self) -> None:
        """Standard runpy import aliases retain an inert module lifecycle."""
        script = (
            "import runpy,sys\n"
            "sys.path.insert(0,sys.argv[1])\n"
            "options={} if sys.argv[3]=='<run_path>' else {'run_name':sys.argv[3]}\n"
            "namespace=runpy.run_path(sys.argv[2],**options)\n"
            "assert callable(namespace['main'])\n"
        )
        for name in ("<run_path>", "__a", "metadata.pipeline.__main__"):
            with self.subTest(name=name):
                result = subprocess.run([sys.executable, "-I", "-B", "-Wignore::SyntaxWarning",
                                         "-c", script, str(REPOSITORY), str(self.entrypoint), name],
                                        capture_output=True, text=True, timeout=15, check=False)
                self.assertEqual(result.returncode, 0, result.stderr)
                self.assertEqual(result.stdout, "")
                self.assertEqual(result.stderr, "")
                self.assertFalse(self.output.exists())

    def test_script_bootstrap_prefers_owned_source_to_a_prior_decoy(self) -> None:
        """Bootstrap source priority is separate from containing hostile code."""
        decoy = self.base / "decoy"
        package = decoy / "metadata"
        package.mkdir(parents=True)
        marker = self.base / "decoy-was-imported"
        (package / "__init__.py").write_text(
            "from pathlib import Path\nPath(" + repr(str(marker)) + ").write_text('wrong source')\n"
            "raise RuntimeError('unowned metadata source selected')\n", encoding="utf-8")
        script = (
            "import runpy,sys\n"
            "sys.path.insert(0,sys.argv[1])\n"
            "namespace=runpy.run_path(sys.argv[2],run_name='__a')\n"
            "assert callable(namespace['main'])\n"
        )
        result = subprocess.run([sys.executable, "-I", "-B", "-Wignore::SyntaxWarning",
                                 "-c", script, str(decoy), str(self.entrypoint)],
                                capture_output=True, text=True, timeout=15, check=False)
        self.assertFalse(marker.exists(), "the entrypoint imported the decoy package")
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stdout, "")
        self.assertEqual(result.stderr, "")

    def test_omitted_root_uses_the_entrypoints_own_checkout(self) -> None:
        """An ambient working directory cannot replace the declared default root."""
        shutil.copyfile(REPOSITORY / "metadata/__init__.py", self.root / "metadata/__init__.py")
        shutil.copytree(REPOSITORY / "metadata/pipeline", self.root / "metadata/pipeline",
                        ignore=shutil.ignore_patterns("__pycache__", "*.pyc"))
        entry = self.root / "metadata/pipeline/__main__.py"
        result = subprocess.run([sys.executable, "-I", "-B", str(entry), "validate"],
                                cwd=self.base, capture_output=True, text=True, timeout=15,
                                check=False)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stderr, "")
        report = json.loads(result.stdout)
        self.assertEqual(report["status"], "valid-source")
        self.assertEqual(report["component_count"], 5)
        self.assertFalse(self.output.exists())

    def test_output_argument_errors_use_parser_exit_without_effects(self) -> None:
        """Invalid command/output combinations fail before the source operation."""
        for command, extra in (("validate", ["--output", str(self.output)]),
                               ("generate", []), ("check", [])):
            with self.subTest(command=command):
                result = subprocess.run([sys.executable, "-I", "-B", str(self.entrypoint),
                                         command, "--root", str(self.root), *extra],
                                        capture_output=True, text=True, timeout=15, check=False)
                self.assertEqual(result.returncode, 2, result.stderr)
                self.assertEqual(result.stdout, "")
                self.assertIn("validate has no output; generate/check require --output", result.stderr)
                self.assertNotIn("Traceback", result.stderr)
                self.assertFalse(self.output.exists())

    def test_programmatic_commands_follow_equality_with_legal_string_subclasses(self) -> None:
        """The public list[str] entry point does not substitute lexical ordering."""
        class ReverseOrder(str):
            def __ge__(self, other: str) -> bool:
                return not str.__ge__(self, other)

        sys.path.insert(0, str(REPOSITORY))
        try:
            cli = importlib.import_module("metadata.pipeline.__main__")
            generator = importlib.import_module("metadata.pipeline.generate")
        finally:
            sys.path.remove(str(REPOSITORY))
        # Preparation uses the actual independent generation API, rather than
        # asking the command-routing function to perform its own test setup.
        generator.generate(self.root, self.output)
        other_output = self.base / "second-generated"
        for command, extra, wanted in (
                ("validate", [], "valid-source"),
                ("check", ["--output", str(self.output)], "checked-source"),
                ("generate", ["--output", str(other_output)], "generated-source")):
            with self.subTest(command=command):
                stdout, stderr = io.StringIO(), io.StringIO()
                with contextlib.redirect_stdout(stdout), contextlib.redirect_stderr(stderr):
                    status = cli.main([ReverseOrder(command), "--root", str(self.root), *extra])
                self.assertEqual(status, 0, stderr.getvalue())
                self.assertEqual(stderr.getvalue(), "")
                self.assertEqual(json.loads(stdout.getvalue())["status"], wanted)
        self.assertTrue(other_output.is_dir())

    def test_programmatic_equality_result_uses_its_logical_value(self) -> None:
        """Python rich comparisons can return truth-valued objects, not bool singletons."""
        class TruthValue:
            def __init__(self, value: bool) -> None:
                self.value = value

            def __bool__(self) -> bool:
                return self.value

            def __ne__(self, other: object) -> bool:
                return self.value != bool(other)

        class Command(str):
            def __eq__(self, other: object) -> TruthValue:
                return TruthValue(str.__eq__(self, other) is True)

        sys.path.insert(0, str(REPOSITORY))
        try:
            cli = importlib.import_module("metadata.pipeline.__main__")
        finally:
            sys.path.remove(str(REPOSITORY))
        stdout, stderr = io.StringIO(), io.StringIO()
        with contextlib.redirect_stdout(stdout), contextlib.redirect_stderr(stderr):
            status = cli.main([Command("validate"), "--root", str(self.root)])
        self.assertEqual(status, 0, stderr.getvalue())
        self.assertEqual(stderr.getvalue(), "")
        self.assertEqual(json.loads(stdout.getvalue())["status"], "valid-source")
