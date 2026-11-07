"""Black-box source metadata guarantees; no Cargo or native execution."""

from __future__ import annotations

import csv
import dataclasses
import errno
import hashlib
import importlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import unittest
from unittest import mock
import xml.etree.ElementTree as ET


REPOSITORY = Path(__file__).resolve().parents[2]
ENTRYPOINT = REPOSITORY / "metadata/pipeline/__main__.py"
EXPECTED_FILES = frozenset(
    {
        "metadata.json",
        "field-register.json",
        "field-coverage.json",
        "rust/product.rs",
        "rust/ui.rs",
        "rust/observe.rs",
        "rc/ui.rc",
        "rc/observe.rc",
        "authoring/msix-inputs.json",
        "authoring/service-inputs.json",
        "authoring/event-inputs.json",
        "authoring/display.xml",
        "authoring/store.csv",
        "authoring/project-inputs.json",
        "receipt.json",
    }
)
INPUT_FILES = (
    "metadata/product.toml",
    "Cargo.toml",
    "crates/nevrotick-ui/Cargo.toml",
    "crates/nevrotick-engine/Cargo.toml",
)
LITERAL_SENTINEL = 'Source "quoted" \\ path & <literal> Я 漢字 café 𐐀'
REJECTED_SENTINEL = "PRIVATE-REJECTED-CONTENT-934781"


def snapshot(directory: Path) -> dict[str, bytes]:
    """Observe exact regular-file content without following output links."""
    result = {}
    for path in sorted(directory.rglob("*")):
        if path.is_symlink():
            result[path.relative_to(directory).as_posix()] = b"SYMLINK:" + os.fsencode(
                os.readlink(path)
            )
        elif path.is_file():
            result[path.relative_to(directory).as_posix()] = path.read_bytes()
    return result


def json_data(path: Path) -> dict:
    """Parse a real output independently of production helpers."""
    return json.loads(path.read_text(encoding="utf-8"))


def rust_string(source: str, name: str) -> str:
    """Observe one complete generated Rust literal without generator helpers."""
    matches = re.findall(
        rf'(?m)^pub const {re.escape(name)}: &str = ("(?:[^"\\]|\\.)*");$', source
    )
    if len(matches) != 1:
        raise AssertionError(f"Expected one complete Rust constant {name}")
    literal = matches[0][1:-1]
    result: list[str] = []
    index = 0
    while index < len(literal):
        if literal[index] != "\\":
            result.append(literal[index])
            index += 1
            continue
        match = re.match(r"\\u\{([0-9a-fA-F]{1,6})\}", literal[index:])
        if match:
            result.append(chr(int(match[1], 16)))
            index += len(match[0])
            continue
        index += 1
        if index == len(literal) or literal[index] not in '\\"nrt0':
            raise AssertionError("Unexpected Rust escape")
        result.append({"\\": "\\", '"': '"', "n": "\n", "r": "\r", "t": "\t", "0": "\0"}[literal[index]])
        index += 1
    return "".join(result)


def rc_string(source: str, name: str) -> str:
    """Observe a complete RC VALUE string from its literal-source projection."""
    matches = re.findall(
        rf'(?m)^\s*VALUE "{re.escape(name)}", "((?:[^"\\]|\\.|"")*)"[ \t]*$', source
    )
    if len(matches) != 1:
        raise AssertionError(f"Expected one complete RC string {name}")
    literal = matches[0]
    result: list[str] = []
    index = 0
    while index < len(literal):
        if literal[index:index + 2] == '""':
            result.append('"')
            index += 2
        elif literal[index] == "\\":
            index += 1
            if index == len(literal) or literal[index] not in "\\nrt0":
                raise AssertionError("Unexpected RC escape")
            result.append({"\\": "\\", "n": "\n", "r": "\r", "t": "\t", "0": "\0"}[literal[index]])
            index += 1
        else:
            result.append(literal[index])
            index += 1
    value = "".join(result)
    if not value.endswith("\0") or "\0" in value[:-1]:
        raise AssertionError("Expected exactly one terminal RC NUL")
    return value[:-1]


class SourceFixture(unittest.TestCase):
    """Own every source/output fixture and invoke the public inert CLI."""

    def setUp(self) -> None:
        temporary = tempfile.TemporaryDirectory(prefix="nevrotick-metadata-test-")
        self.addCleanup(temporary.cleanup)
        self.base = Path(temporary.name)
        self.root = self.base / "source"
        self.root.mkdir()
        for relative in INPUT_FILES:
            source = REPOSITORY / relative
            destination = self.root / relative
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_bytes(source.read_bytes())
        self.record = self.root / "metadata/product.toml"
        self.original_record = self.record.read_text(encoding="utf-8")
        self.output = self.base / "generated"
        self.entrypoint = ENTRYPOINT

    def cli(self, command: str, *extra: str, output: Path | None = None) -> subprocess.CompletedProcess:
        args = [
            sys.executable,
            "-I",
            "-B",
            str(self.entrypoint),
            command,
            "--root",
            str(self.root),
        ]
        if output is not None:
            args.extend(("--output", str(output)))
        args.extend(extra)
        return subprocess.run(
            args,
            cwd=self.base,
            capture_output=True,
            text=True,
            encoding="utf-8",
            timeout=15,
            check=False,
        )

    def require_success(self, result: subprocess.CompletedProcess) -> dict:
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stderr, "")
        return json.loads(result.stdout)

    def require_rejection(self, result: subprocess.CompletedProcess) -> None:
        self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
        self.assertEqual(result.stdout, "")
        self.assertTrue(result.stderr.startswith("Metadata error:"), result.stderr)
        self.assertLessEqual(len(result.stderr.encode("utf-8")), 4096)
        self.assertNotIn(str(self.base), result.stderr)
        self.assertNotIn(REJECTED_SENTINEL, result.stderr)

    def generate(self) -> dict:
        return self.require_success(self.cli("generate", output=self.output))

    def write_record(self, text: str) -> None:
        self.record.write_text(text, encoding="utf-8")

    def replace_once(self, old: str, new: str) -> None:
        text = self.record.read_text(encoding="utf-8")
        self.assertEqual(text.count(old), 1, f"Fixture selector not unique: {old!r}")
        self.write_record(text.replace(old, new, 1))


class MetadataCliTests(SourceFixture):
    def test_source_validates_with_explicit_stage(self) -> None:
        report = self.require_success(self.cli("validate", "--stage", "source"))
        self.assertEqual(report["status"], "valid-source")
        self.assertFalse(self.output.exists())

    def test_source_stage_is_the_default(self) -> None:
        default = self.require_success(self.cli("validate"))
        explicit = self.require_success(self.cli("validate", "--stage", "source"))
        self.assertEqual(default, explicit)

    def test_release_rejects_current_unresolved_record(self) -> None:
        self.require_rejection(self.cli("validate", "--stage", "release"))

    def test_release_generate_creates_no_partial_output(self) -> None:
        self.require_rejection(self.cli("generate", "--stage", "release", output=self.output))
        self.assertFalse(self.output.exists())

    def test_generation_has_exact_paths_and_successful_check(self) -> None:
        report = self.generate()
        self.assertEqual(report["status"], "generated-source")
        self.assertEqual(frozenset(snapshot(self.output)), EXPECTED_FILES)
        checked = self.require_success(self.cli("check", output=self.output))
        self.assertEqual(checked["status"], "checked-source")

    def test_two_fresh_trees_are_byte_identical(self) -> None:
        self.generate()
        other = self.base / "second"
        self.require_success(self.cli("generate", output=other))
        self.assertEqual(snapshot(self.output), snapshot(other))

    def test_generation_does_not_mutate_source(self) -> None:
        before = snapshot(self.root)
        self.generate()
        self.assertEqual(snapshot(self.root), before)

    def test_check_is_observational(self) -> None:
        self.generate()
        before = snapshot(self.output)
        stats = {name: (self.output / name).stat().st_mtime_ns for name in before}
        self.require_success(self.cli("check", output=self.output))
        self.assertEqual(snapshot(self.output), before)
        self.assertEqual(
            {name: (self.output / name).stat().st_mtime_ns for name in before}, stats
        )

    def test_check_detects_changed_value_without_repair(self) -> None:
        self.generate()
        target = self.output / "metadata.json"
        target.write_bytes(target.read_bytes().replace(b"NevroTick", b"WrongTick", 1))
        before = snapshot(self.output)
        self.require_rejection(self.cli("check", output=self.output))
        self.assertEqual(snapshot(self.output), before)

    def test_check_detects_missing_file_without_repair(self) -> None:
        self.generate()
        (self.output / "rc/ui.rc").unlink()
        before = snapshot(self.output)
        self.require_rejection(self.cli("check", output=self.output))
        self.assertEqual(snapshot(self.output), before)

    def test_check_detects_unexpected_file_without_removal(self) -> None:
        self.generate()
        (self.output / "unexpected.txt").write_text("unexpected", encoding="utf-8")
        before = snapshot(self.output)
        self.require_rejection(self.cli("check", output=self.output))
        self.assertEqual(snapshot(self.output), before)

    def test_check_detects_source_change_without_repair(self) -> None:
        self.generate()
        self.record.write_bytes(self.record.read_bytes() + b"\n# changed source identity\n")
        before = snapshot(self.output)
        self.require_rejection(self.cli("check", output=self.output))
        self.assertEqual(snapshot(self.output), before)

    def test_check_detects_cargo_change_without_repair(self) -> None:
        self.generate()
        cargo = self.root / "crates/nevrotick-ui/Cargo.toml"
        cargo.write_bytes(cargo.read_bytes() + b"\n# changed Cargo source identity\n")
        before = snapshot(self.output)
        self.require_rejection(self.cli("check", output=self.output))
        self.assertEqual(snapshot(self.output), before)

    def test_check_missing_tree_does_not_create_it(self) -> None:
        self.require_rejection(self.cli("check", output=self.output))
        self.assertFalse(self.output.exists())

    def test_existing_output_is_preserved_on_generate_rejection(self) -> None:
        self.generate()
        before = snapshot(self.output)
        self.require_rejection(self.cli("generate", output=self.output))
        self.assertEqual(snapshot(self.output), before)

    def test_invalid_input_preserves_previous_output(self) -> None:
        self.generate()
        before = snapshot(self.output)
        self.write_record(f'unknown = "{REJECTED_SENTINEL}"\n' + self.original_record)
        self.require_rejection(self.cli("generate", output=self.output))
        self.assertEqual(snapshot(self.output), before)

    def test_invalid_stage_is_an_argument_error(self) -> None:
        result = self.cli("validate", "--stage", "installed")
        self.assertEqual(result.returncode, 2)
        self.assertFalse(self.output.exists())

    def test_output_inside_source_is_rejected(self) -> None:
        target = self.root / "generated"
        before = snapshot(self.root)
        self.require_rejection(self.cli("generate", output=target))
        self.assertFalse(target.exists())
        self.assertEqual(snapshot(self.root), before)

    def test_output_parent_must_exist(self) -> None:
        target = self.base / "missing-parent/generated"
        self.require_rejection(self.cli("generate", output=target))
        self.assertFalse(target.parent.exists())

    def test_source_outside_root_is_rejected(self) -> None:
        other = self.base / "foreign.toml"
        other.write_text(self.original_record, encoding="utf-8")
        self.require_rejection(self.cli("validate", "--source", str(other)))

    def test_duplicate_toml_key_is_rejected_without_raw_value(self) -> None:
        self.write_record(self.original_record + f'\nschema_version = "{REJECTED_SENTINEL}"\n')
        self.require_rejection(self.cli("validate"))

    def test_unknown_top_level_key_is_rejected_without_raw_value(self) -> None:
        self.write_record(f'unknown = "{REJECTED_SENTINEL}"\n' + self.original_record)
        self.require_rejection(self.cli("validate"))

    def test_invalid_utf8_is_rejected(self) -> None:
        self.record.write_bytes(b"\xff\xfe" + self.record.read_bytes())
        self.require_rejection(self.cli("validate"))

    def test_oversize_source_is_rejected(self) -> None:
        self.record.write_bytes(self.record.read_bytes() + b"#" + b"x" * (256 * 1024))
        self.require_rejection(self.cli("validate"))

    def test_symlink_source_is_rejected(self) -> None:
        foreign = self.base / "foreign.toml"
        foreign.write_text(self.original_record, encoding="utf-8")
        self.record.unlink()
        self.record.symlink_to(foreign)
        self.require_rejection(self.cli("validate"))

    def test_symlink_source_directory_is_rejected(self) -> None:
        moved = self.base / "foreign-metadata"
        (self.root / "metadata").rename(moved)
        (self.root / "metadata").symlink_to(moved, target_is_directory=True)
        self.require_rejection(self.cli("validate"))

    def test_hardlink_source_is_rejected(self) -> None:
        foreign = self.base / "hardlink.toml"
        os.link(self.record, foreign)
        self.require_rejection(self.cli("validate"))

    def test_symlink_cargo_is_rejected(self) -> None:
        cargo = self.root / "crates/nevrotick-ui/Cargo.toml"
        foreign = self.base / "foreign-cargo.toml"
        cargo.rename(foreign)
        cargo.symlink_to(foreign)
        self.require_rejection(self.cli("validate"))

    def test_output_symlink_is_rejected(self) -> None:
        other = self.base / "foreign-output"
        other.mkdir()
        (other / "private.txt").write_text(REJECTED_SENTINEL, encoding="utf-8")
        before = snapshot(other)
        self.output.symlink_to(other, target_is_directory=True)
        self.require_rejection(self.cli("generate", output=self.output))
        self.assertEqual(snapshot(other), before)

    def test_output_parent_symlink_is_rejected(self) -> None:
        other = self.base / "other"
        other.mkdir()
        parent = self.base / "linked-parent"
        parent.symlink_to(other, target_is_directory=True)
        self.require_rejection(self.cli("generate", output=parent / "generated"))
        self.assertEqual(list(other.iterdir()), [])

    def test_check_rejects_output_symlink_file(self) -> None:
        self.generate()
        foreign = self.base / "foreign.rs"
        foreign.write_text(REJECTED_SENTINEL, encoding="utf-8")
        target = self.output / "rust/ui.rs"
        target.unlink()
        target.symlink_to(foreign)
        self.require_rejection(self.cli("check", output=self.output))
        self.assertEqual(foreign.read_text(encoding="utf-8"), REJECTED_SENTINEL)

    def test_check_rejects_output_hardlink_file(self) -> None:
        self.generate()
        target = self.output / "rust/ui.rs"
        os.link(target, self.base / "foreign-hardlink.rs")
        before = snapshot(self.output)
        self.require_rejection(self.cli("check", output=self.output))
        self.assertEqual(snapshot(self.output), before)

    def test_unknown_nested_key_is_rejected(self) -> None:
        self.replace_once('[product]\n', f'[product]\nunknown = "{REJECTED_SENTINEL}"\n')
        self.require_rejection(self.cli("validate"))

    def test_unknown_component_is_rejected(self) -> None:
        self.write_record(self.original_record + '\n[components.extra]\nrole = "ui"\n')
        self.require_rejection(self.cli("validate"))

    def test_wrong_schema_type_is_rejected(self) -> None:
        self.replace_once("schema_version = 1", 'schema_version = "1"')
        self.require_rejection(self.cli("validate"))

    def test_boolean_schema_version_is_not_an_integer(self) -> None:
        self.replace_once("schema_version = 1", "schema_version = true")
        self.require_rejection(self.cli("validate"))

    def test_unknown_schema_version_is_rejected(self) -> None:
        self.replace_once("schema_version = 1", "schema_version = 2")
        self.require_rejection(self.cli("validate"))

    def test_cpu_floor_cannot_be_downgraded(self) -> None:
        self.replace_once('cpu_floor = "x86-64-v3"', 'cpu_floor = "x86-64"')
        self.require_rejection(self.cli("validate"))

    def test_cpu_floor_cannot_be_changed_to_ambient_native(self) -> None:
        self.replace_once('cpu_floor = "x86-64-v3"', 'cpu_floor = "native"')
        self.require_rejection(self.cli("validate"))

    def test_platform_target_cannot_be_reassigned(self) -> None:
        self.replace_once('target = "x86_64-pc-windows-msvc"', 'target = "x86_64-unknown-linux-gnu"')
        self.require_rejection(self.cli("validate"))

    def test_product_identity_cannot_be_reassigned(self) -> None:
        self.replace_once('[product]\nname = "NevroTick"', '[product]\nname = "AnotherProduct"')
        self.require_rejection(self.cli("validate"))

    def test_publisher_identity_cannot_be_reassigned(self) -> None:
        self.replace_once('publisher = "David Osipov"', 'publisher = "Another Publisher"')
        self.require_rejection(self.cli("validate"))

    def test_ordinary_product_description_change_is_valid(self) -> None:
        self.replace_once(
            'description = "A Windows time synchronization client under development."',
            'description = "Current source authoring under review."',
        )
        self.require_success(self.cli("validate"))

    def test_unknown_owned_pending_key_is_rejected(self) -> None:
        self.replace_once(
            'path = "release.version"',
            f'path = "release.version"\nunknown = "{REJECTED_SENTINEL}"',
        )
        self.require_rejection(self.cli("validate"))

    def test_required_pending_rows_cannot_be_removed(self) -> None:
        self.write_record(self.original_record.split("[[pending]]", 1)[0])
        self.require_rejection(self.cli("validate", "--stage", "release"))
        self.require_rejection(self.cli("validate", "--stage", "source"))

    def test_one_required_pending_row_cannot_be_removed(self) -> None:
        blocks = self.original_record.split("[[pending]]")
        self.write_record(blocks[0] + "[[pending]]" + "[[pending]]".join(blocks[2:]))
        self.require_rejection(self.cli("validate"))

    def test_pending_stage_cannot_be_downgraded(self) -> None:
        self.write_record(self.original_record.replace('required_stage = "release"', 'required_stage = "source"'))
        self.require_rejection(self.cli("validate", "--stage", "release"))

    def test_pending_owner_cannot_be_empty(self) -> None:
        self.replace_once('owner = "W11-W05"', 'owner = ""')
        self.require_rejection(self.cli("validate"))

    def test_pending_next_action_cannot_be_empty(self) -> None:
        self.replace_once(
            'next_action = "Resolve the existing provisioning and identity contract."',
            'next_action = ""',
        )
        self.require_rejection(self.cli("validate"))

    def test_planned_component_cannot_be_omitted(self) -> None:
        self.write_record(re.sub(r'(?ms)^\[components\.broker\]\n.*?(?=^\[|\Z)', '', self.original_record))
        self.require_rejection(self.cli("validate"))

    def test_planned_component_cannot_claim_production_state(self) -> None:
        self.write_record(self.original_record.replace('state = "planned"', 'state = "production"'))
        self.require_rejection(self.cli("validate"))

    def test_planned_component_cannot_acquire_a_fake_binary(self) -> None:
        self.replace_once('[components.broker]\n', '[components.broker]\nbinary_name = "fake-broker"\n')
        self.require_rejection(self.cli("validate"))

    def test_manifest_path_traversal_is_rejected(self) -> None:
        self.replace_once('cargo_manifest = "crates/nevrotick-ui/Cargo.toml"', 'cargo_manifest = "../foreign.toml"')
        self.require_rejection(self.cli("validate"))

    def test_manifest_absolute_path_is_rejected(self) -> None:
        self.replace_once(
            'cargo_manifest = "crates/nevrotick-ui/Cargo.toml"',
            'cargo_manifest = ' + json.dumps(str(self.root / 'crates/nevrotick-ui/Cargo.toml')),
        )
        self.require_rejection(self.cli("validate"))

    def test_description_control_character_is_rejected(self) -> None:
        self.replace_once(
            'description = "A Windows time synchronization client under development."',
            'description = "' + REJECTED_SENTINEL + '\\u0000"',
        )
        self.require_rejection(self.cli("validate"))

    def test_description_bidi_override_is_rejected(self) -> None:
        self.replace_once(
            'description = "A Windows time synchronization client under development."',
            'description = "' + REJECTED_SENTINEL + '\\u202E"',
        )
        self.require_rejection(self.cli("validate"))

    def test_xml_forbidden_characters_are_rejected_before_generation(self) -> None:
        for codepoint in (0xFFFE, 0xFFFF):
            with self.subTest(codepoint=f"U+{codepoint:04X}"):
                self.write_record(self.original_record.replace(
                    'description = "A Windows time synchronization client under development."',
                    'description = ' + json.dumps('Source XML ' + chr(codepoint), ensure_ascii=False),
                    1,
                ))
                output = self.base / f"forbidden-{codepoint:04x}"
                result = self.cli("generate", output=output)
                # If the candidate wrongly publishes, independently observe the
                # actual XML before the rejection assertion reports the defect.
                observed_xml = "no generated XML"
                if output.exists():
                    try:
                        ET.parse(output / "authoring/display.xml")
                        observed_xml = "unexpectedly parseable XML"
                    except ET.ParseError:
                        observed_xml = "published XML fails independent parsing"
                self.assertEqual(result.returncode, 1, observed_xml)
                self.require_rejection(result)
                self.assertFalse(output.exists())

    def test_display_name_utf16_boundary_and_neighbor(self) -> None:
        # Non-BMP characters use two UTF-16 code units per character.
        for count, accepted in ((128, True), (129, False)):
            with self.subTest(utf16_units=count * 2):
                self.write_record(self.original_record.replace(
                    'display_name = "NevroTick Observe"',
                    'display_name = "' + '𐐀' * count + '"',
                ))
                result = self.cli("validate")
                if accepted:
                    self.require_success(result)
                else:
                    self.require_rejection(result)

    def test_component_description_boundary_and_neighbor(self) -> None:
        for count, accepted in ((1024, True), (1025, False)):
            with self.subTest(utf16_units=count):
                self.write_record(self.original_record.replace(
                    'description = "Fixture-only NevroTick UI-M0 prototype"',
                    'description = "' + 'x' * count + '"',
                ))
                result = self.cli("validate")
                if accepted:
                    self.require_success(result)
                else:
                    self.require_rejection(result)

    def test_file_version_revision_bounds_and_valid_neighbor(self) -> None:
        for revision, accepted in ((0, True), (1, True), (65535, True), (65536, False), (-1, False), (True, False)):
            with self.subTest(revision=revision):
                raw = str(revision).lower()
                self.write_record(self.original_record.replace(
                    "file_version_revision = 0", "file_version_revision = " + raw,
                ))
                result = self.cli("validate")
                if accepted:
                    self.require_success(result)
                else:
                    self.require_rejection(result)

    def test_changed_parent_initializer_invalidates_generated_output(self) -> None:
        runner = self.base / "owned-tool"
        for relative in (
            "metadata/__init__.py", "metadata/pipeline/__init__.py",
            "metadata/pipeline/__main__.py", "metadata/pipeline/contract.py",
            "metadata/pipeline/fields.py", "metadata/pipeline/text_policy.py",
            "metadata/pipeline/delivery.py", "metadata/pipeline/generate.py",
        ):
            destination = runner / relative
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_bytes((REPOSITORY / relative).read_bytes())
        self.entrypoint = runner / "metadata/pipeline/__main__.py"
        self.generate()
        before = snapshot(self.output)
        parent_initializer = runner / "metadata/__init__.py"
        parent_initializer.write_bytes(parent_initializer.read_bytes() + b"\n# changed executed initializer\n")
        self.require_rejection(self.cli("check", output=self.output))
        self.assertEqual(snapshot(self.output), before)

    def test_source_fifo_is_rejected_without_blocking(self) -> None:
        self.record.unlink()
        os.mkfifo(self.record, 0o600)
        self.require_rejection(self.cli("validate"))

    def test_output_unsafe_parent_permissions_are_rejected(self) -> None:
        parent = self.base / "shared-parent"
        parent.mkdir(mode=0o777)
        parent.chmod(0o777)
        self.require_rejection(self.cli("generate", output=parent / "generated"))
        self.assertEqual(list(parent.iterdir()), [])

    def test_homepage_url_valid_neighbor_and_unsafe_values(self) -> None:
        for url, accepted in (
            ("https://davidosipov.com", True),
            ("http://davidosipov.com", False),
            ("https://127.0.0.1/private", False),
            ("https://[::1]/private", False),
            ("https://2130706433/private", False),
            ("https://owned.local/private", False),
            ("https://owned.internal/private", False),
            ("https://example.com", False),
            ("https://user:password@davidosipov.com", False),
            ("https://davidosipov.com/path with space", False),
        ):
            with self.subTest(url=url):
                self.write_record(self.original_record.replace(
                    'homepage = ""', 'homepage = ' + json.dumps(url), 1
                ))
                result = self.cli("validate")
                if accepted:
                    self.require_success(result)
                else:
                    self.require_rejection(result)

    def test_csv_formula_prefix_rejects_generation_with_plain_neighbor(self) -> None:
        for description, accepted in (
            ("Source text = 1 + 1", True),
            ("=1+1", False), ("+1+1", False), ("-1+1", False),
            ("@SUM(1,1)", False), ("\t=1+1", False),
        ):
            with self.subTest(description=description):
                self.write_record(self.original_record.replace(
                    'description = "A Windows time synchronization client under development."',
                    'description = ' + json.dumps(description), 1,
                ))
                result = self.cli("generate", output=self.base / f"formula-{len(description)}-{accepted}")
                if accepted:
                    self.require_success(result)
                else:
                    self.require_rejection(result)

    def test_cargo_package_identity_is_checked(self) -> None:
        cargo = self.root / "crates/nevrotick-ui/Cargo.toml"
        cargo.write_text(cargo.read_text(encoding="utf-8").replace(
            'name = "nevrotick-ui"', 'name = "foreign-ui"', 1
        ), encoding="utf-8")
        self.require_rejection(self.cli("validate"))

    def test_cargo_license_authority_is_checked(self) -> None:
        cargo = self.root / "crates/nevrotick-ui/Cargo.toml"
        cargo.write_text(cargo.read_text(encoding="utf-8").replace(
            'license = "GPL-3.0-only"', 'license = "MIT"', 1
        ), encoding="utf-8")
        self.require_rejection(self.cli("validate"))

    def test_cargo_repository_authority_is_checked(self) -> None:
        cargo = self.root / "Cargo.toml"
        cargo.write_text(cargo.read_text(encoding="utf-8").replace(
            'repository = "https://github.com/DavidOsipov/NevroTick"',
            'repository = "https://github.com/another/foreign"', 1
        ), encoding="utf-8")
        self.require_rejection(self.cli("validate"))

    def test_cargo_observe_binary_source_is_checked(self) -> None:
        cargo = self.root / "crates/nevrotick-engine/Cargo.toml"
        cargo.write_text(cargo.read_text(encoding="utf-8").replace(
            'path = "src/bin/nevrotick-observe.rs"', 'path = "src/bin/foreign.rs"', 1
        ), encoding="utf-8")
        self.require_rejection(self.cli("validate"))


class GeneratedFactTests(SourceFixture):
    def test_xml_replacement_character_neighbor_is_preserved(self) -> None:
        description = "Source XML \ufffd"
        self.replace_once(
            'description = "A Windows time synchronization client under development."',
            'description = ' + json.dumps(description, ensure_ascii=False),
        )
        self.generate()
        root = ET.parse(self.output / "authoring/display.xml").getroot()
        namespace = {"m": "urn:nevrotick:metadata:authoring:1"}
        self.assertEqual(root.findtext("m:product/m:description", namespaces=namespace), description)

    def test_event_projection_preserves_unresolved_diagnostic_owner(self) -> None:
        self.generate()
        events = json_data(self.output / "authoring/event-inputs.json")
        self.assertIsNone(events["provider_name"])
        self.assertIsNone(events["provider_guid"])
        self.assertIsNone(events["event_schema_version"])
        self.assertIsNone(events["message_resource"])
        self.assertEqual(
            [(row["path"], row["owner"], row["required_stage"]) for row in events["pending"]],
            [("diagnostics.provider", "W11-W08", "release")],
        )

    def test_real_component_versions_roles_and_licenses(self) -> None:
        self.generate()
        data = json_data(self.output / "metadata.json")
        self.assertEqual(data["product"]["name"], "NevroTick")
        self.assertEqual(data["product"]["publisher"], "David Osipov")
        self.assertEqual(data["product"]["repository"], "https://github.com/DavidOsipov/NevroTick")
        self.assertEqual(data["target"], "x86_64-pc-windows-msvc")
        self.assertEqual(data["cpu_floor"], "x86-64-v3")
        components = {row["id"]: row for row in data["components"]}
        self.assertEqual(set(components), {"ui", "observe", "engine", "broker", "helper"})
        for identity, binary, license_id, state in (
            ("ui", "nevrotick-ui", "GPL-3.0-only", "fixture"),
            ("observe", "nevrotick-observe", "GPL-2.0-only", "experiment"),
        ):
            with self.subTest(component=identity):
                row = components[identity]
                self.assertEqual(row["binary_name"], binary)
                self.assertEqual(row["license"], license_id)
                self.assertEqual(row["version"], "0.0.0")
                self.assertEqual(row["file_version"], [0, 0, 0, 0])
                self.assertEqual(row["state"], state)
        for identity in ("engine", "broker", "helper"):
            with self.subTest(planned=identity):
                row = components[identity]
                self.assertEqual(row["state"], "planned")
                for field in ("binary_name", "version", "file_version", "license", "cargo_manifest"):
                    self.assertIsNone(row[field])
        self.assertEqual(data["qualification"], {
            "scope": "source-authoring-projections", "native": "not-run", "release": "not-qualified"
        })

    def test_receipt_covers_exact_outputs_and_frozen_inputs(self) -> None:
        self.generate()
        receipt = json_data(self.output / "receipt.json")
        recorded = {row["path"]: row for row in receipt["files"]}
        self.assertEqual(set(recorded), EXPECTED_FILES - {"receipt.json"})
        self.assertEqual(len(receipt["files"]), len(recorded))
        for relative, row in recorded.items():
            with self.subTest(file=relative):
                raw = (self.output / relative).read_bytes()
                self.assertEqual(row["sha256"], hashlib.sha256(raw).hexdigest())
                self.assertEqual(row["bytes"], len(raw))
        actual_inputs = {row["path"]: row["sha256"] for row in receipt["inputs"]}
        self.assertEqual(actual_inputs, {
            relative: hashlib.sha256((self.root / relative).read_bytes()).hexdigest()
            for relative in INPUT_FILES
        })
        self.assertTrue(receipt["tools"])
        self.assertEqual(
            {row["path"] for row in receipt["tools"]},
            {"metadata/__init__.py", "metadata/pipeline/__init__.py",
             "metadata/pipeline/__main__.py", "metadata/pipeline/contract.py",
             "metadata/pipeline/fields.py", "metadata/pipeline/text_policy.py",
             "metadata/pipeline/delivery.py", "metadata/pipeline/generate.py"},
        )
        for row in receipt["tools"]:
            self.assertEqual(
                row["sha256"], hashlib.sha256((REPOSITORY / row["path"]).read_bytes()).hexdigest()
            )

    def test_public_outputs_do_not_leak_machine_or_output_paths(self) -> None:
        self.generate()
        for relative, raw in snapshot(self.output).items():
            with self.subTest(file=relative):
                self.assertNotIn(os.fsencode(self.base), raw)
                self.assertNotIn(os.fsencode(REPOSITORY), raw)

    def test_rc_has_explicit_ordinary_application_fields(self) -> None:
        self.generate()
        for identity in ("ui", "observe"):
            with self.subTest(component=identity):
                source = (self.output / f"rc/{identity}.rc").read_text(encoding="utf-8")
                self.assertIn("#pragma code_page(65001)", source)
                self.assertRegex(source, r"\bFILEVERSION\s+0\s*,\s*0\s*,\s*0\s*,\s*0\b")
                self.assertRegex(source, r"\bPRODUCTVERSION\s+0\s*,\s*0\s*,\s*0\s*,\s*0\b")
                self.assertRegex(source, r"\bFILETYPE\s+0x1L\b")
                self.assertRegex(source, r"\bFILESUBTYPE\s+0x0L\b")
                self.assertRegex(source, r"\bFILEFLAGS\s+0x2L\b")
                self.assertIn('"040904B0"', source)
                self.assertEqual(rc_string(source, "ProductName"), "NevroTick")
                self.assertEqual(rc_string(source, "CompanyName"), "David Osipov")
                self.assertEqual(rc_string(source, "OriginalFilename"), {
                    "ui": "nevrotick-ui.exe", "observe": "nevrotick-observe.exe"
                }[identity])

    def test_literal_sentinels_survive_each_reached_sink(self) -> None:
        self.replace_once(
            'description = "A Windows time synchronization client under development."',
            'description = ' + json.dumps(LITERAL_SENTINEL, ensure_ascii=False),
        )
        self.replace_once(
            'description = "Fixture-only NevroTick UI-M0 prototype"',
            'description = ' + json.dumps(LITERAL_SENTINEL, ensure_ascii=False),
        )
        self.generate()
        data = json_data(self.output / "metadata.json")
        self.assertEqual(data["product"]["description"], LITERAL_SENTINEL)
        ui = next(row for row in data["components"] if row["id"] == "ui")
        self.assertEqual(ui["description"], LITERAL_SENTINEL)
        product_rs = (self.output / "rust/product.rs").read_text(encoding="utf-8")
        ui_rs = (self.output / "rust/ui.rs").read_text(encoding="utf-8")
        self.assertEqual(rust_string(product_rs, "PRODUCT_DESCRIPTION"), LITERAL_SENTINEL)
        self.assertEqual(rust_string(ui_rs, "DESCRIPTION"), LITERAL_SENTINEL)
        self.assertEqual(
            rc_string((self.output / "rc/ui.rc").read_text(encoding="utf-8"), "FileDescription"),
            LITERAL_SENTINEL,
        )
        xml_root = ET.parse(self.output / "authoring/display.xml").getroot()
        namespace = {"m": "urn:nevrotick:metadata:authoring:1"}
        self.assertEqual(xml_root.tag, "{urn:nevrotick:metadata:authoring:1}metadata")
        self.assertEqual(xml_root.findtext("m:product/m:description", namespaces=namespace), LITERAL_SENTINEL)
        self.assertEqual(len(list(xml_root.iter())), 22)
        with (self.output / "authoring/store.csv").open(encoding="utf-8", newline="") as file:
            rows = list(csv.DictReader(file))
        description_rows = [row for row in rows if row["field"] == "product.description"]
        self.assertEqual(len(description_rows), 1)
        self.assertEqual(description_rows[0]["value"], LITERAL_SENTINEL)

    def test_json_unknown_cargo_fields_do_not_become_public_exports(self) -> None:
        cargo = self.root / "crates/nevrotick-ui/Cargo.toml"
        cargo.write_text(cargo.read_text(encoding="utf-8") + '\n[package.metadata.private]\nsecret = "' + REJECTED_SENTINEL + '"\n', encoding="utf-8")
        self.generate()
        for relative, raw in snapshot(self.output).items():
            with self.subTest(file=relative):
                self.assertNotIn(REJECTED_SENTINEL.encode(), raw)

    def test_multiline_product_text_preserves_record_and_row_cardinality(self) -> None:
        description = 'Source line one\nLine two\t"quoted", value'
        self.replace_once(
            'description = "A Windows time synchronization client under development."',
            'description = ' + json.dumps(description),
        )
        self.generate()
        self.assertEqual(
            rust_string((self.output / "rust/product.rs").read_text(encoding="utf-8"), "PRODUCT_DESCRIPTION"),
            description,
        )
        self.assertEqual(json_data(self.output / "metadata.json")["product"]["description"], description)
        with (self.output / "authoring/store.csv").open(encoding="utf-8", newline="") as file:
            rows = list(csv.DictReader(file))
        expected_fields = {
            "product.name", "product.publisher", "product.description", "product.repository",
            "release.version", "package.identity", "services.identities", "diagnostics.provider",
            "public.destinations", "release.qualification",
            "catalogue.B.9.1.StoreAdditional.Developed by",
        }
        # The expanded author register adds one legitimate Store alias. Check
        # its independent meaning and the exact field set, not a looser row cap.
        self.assertEqual({row["field"] for row in rows}, expected_fields)
        self.assertEqual(len(rows), len(expected_fields))
        self.assertTrue(all(set(row) == {"field", "value", "status"} for row in rows))
        self.assertEqual([row["value"] for row in rows if row["field"] ==
                          "catalogue.B.9.1.StoreAdditional.Developed by"], ["David Osipov"])
        self.assertEqual([row["value"] for row in rows if row["field"] == "product.description"], [description])

    def test_cargo_prerelease_and_numeric_windows_version_are_preserved(self) -> None:
        cargo = self.root / "crates/nevrotick-ui/Cargo.toml"
        cargo.write_text(cargo.read_text(encoding="utf-8").replace(
            'version = "0.0.0"', 'version = "1.2.3-beta.2+owned"', 1
        ), encoding="utf-8")
        self.replace_once("file_version_revision = 0", "file_version_revision = 7")
        self.generate()
        ui = next(row for row in json_data(self.output / "metadata.json")["components"] if row["id"] == "ui")
        self.assertEqual(ui["version"], "1.2.3-beta.2+owned")
        self.assertEqual(ui["file_version"], [1, 2, 3, 7])
        self.assertEqual(rc_string((self.output / "rc/ui.rc").read_text(encoding="utf-8"), "FileVersion"), "1.2.3-beta.2+owned")


class ObserverControlTests(unittest.TestCase):
    def test_rust_observer_detects_wrong_missing_and_extra_literal(self) -> None:
        self.assertEqual(rust_string('pub const NAME: &str = "known";', "NAME"), "known")
        self.assertNotEqual(rust_string('pub const NAME: &str = "wrong";', "NAME"), "known")
        for source in ('pub const OTHER: &str = "known";', 'pub const NAME: &str = "known"; injected();'):
            with self.subTest(source=source), self.assertRaises(AssertionError):
                rust_string(source, "NAME")

    def test_rc_observer_detects_wrong_missing_and_extra_literal(self) -> None:
        self.assertEqual(rc_string('VALUE "Name", "known\\0"', "Name"), "known")
        self.assertEqual(rc_string('VALUE "Name", "a ""quote"" \\\\ path\\0"', "Name"), 'a "quote" \\ path')
        self.assertNotEqual(rc_string('VALUE "Name", "wrong\\0"', "Name"), "known")
        for source in ('VALUE "Other", "known\\0"', 'VALUE "Name", "known\\0" trailing', 'VALUE "Name", "unterminated"'):
            with self.subTest(source=source), self.assertRaises(AssertionError):
                rc_string(source, "Name")


class ModelBoundaryTests(SourceFixture):
    @classmethod
    def setUpClass(cls) -> None:
        # Isolation intentionally excludes cwd. Import only this checked-out
        # implementation, never modules from an inspected fixture repository.
        sys.path.insert(0, str(REPOSITORY))
        try:
            cls.contract = importlib.import_module("metadata.pipeline.contract")
            cls.generator = importlib.import_module("metadata.pipeline.generate")
        finally:
            sys.path.remove(str(REPOSITORY))

    def setUp(self) -> None:
        super().setUp()
        self.model = self.contract.load_metadata(self.root)

    def reject_model(self, model: object) -> None:
        with self.assertRaises(self.contract.MetadataError):
            self.generator.build_outputs(model)

    def test_nested_models_are_frozen(self) -> None:
        for value, field, replacement in (
            (self.model, "cpu_floor", "x86-64"),
            (self.model.product, "name", "Wrong"),
            (self.model.components[0], "state", "production"),
            (self.model.pending[0], "owner", "Wrong"),
            (self.model.inputs[0], "sha256", "0" * 64),
        ):
            with self.subTest(type=type(value).__name__), self.assertRaises(dataclasses.FrozenInstanceError):
                setattr(value, field, replacement)
        self.assertIsInstance(self.model.components, tuple)
        self.assertIsInstance(self.model.pending, tuple)
        self.assertIsInstance(self.model.inputs, tuple)

    def test_direct_api_legal_use_produces_known_product(self) -> None:
        outputs = self.generator.build_outputs(self.model)
        self.assertEqual(json.loads(outputs["metadata.json"])["product"]["name"], "NevroTick")

    def test_direct_api_cannot_bypass_cpu_floor(self) -> None:
        self.reject_model(dataclasses.replace(self.model, cpu_floor="x86-64"))

    def test_direct_api_cannot_bypass_product_identity(self) -> None:
        wrong = dataclasses.replace(self.model.product, name="Wrong")
        self.reject_model(dataclasses.replace(self.model, product=wrong))

    def test_direct_api_cannot_bypass_text_validation(self) -> None:
        wrong = dataclasses.replace(self.model.product, description="Bad\0text")
        self.reject_model(dataclasses.replace(self.model, product=wrong))

    def test_direct_api_cannot_bypass_xml_character_validation(self) -> None:
        for codepoint in (0xFFFE, 0xFFFF):
            with self.subTest(codepoint=f"U+{codepoint:04X}"):
                wrong = dataclasses.replace(self.model.product, description="Source XML " + chr(codepoint))
                self.reject_model(dataclasses.replace(self.model, product=wrong))

    def test_direct_api_rejects_equality_spoofed_fixed_scalars(self) -> None:
        class EqualitySpoof(str):
            def __eq__(self, other: object) -> bool:
                return True

            def __ne__(self, other: object) -> bool:
                return False

            __hash__ = str.__hash__

        for field, value in (
            ("role", "unapproved-role"), ("state", "unapproved-state"),
            ("cargo_manifest", "../../foreign.toml"),
            ("binary_name", "../../foreign.exe"), ("license", "MIT"),
        ):
            with self.subTest(component_field=field):
                components = tuple(
                    dataclasses.replace(component, **{field: EqualitySpoof(value)})
                    if component.id == "ui" else component
                    for component in self.model.components
                )
                self.reject_model(dataclasses.replace(self.model, components=components))

        for field, value in (("owner", "unapproved-owner"), ("required_stage", "unapproved-stage")):
            with self.subTest(pending_field=field):
                pending = (dataclasses.replace(self.model.pending[0], **{field: EqualitySpoof(value)}), *self.model.pending[1:])
                self.reject_model(dataclasses.replace(self.model, pending=pending))

        for field, value in (("target", "unapproved-target"), ("cpu_floor", "x86-64")):
            with self.subTest(platform_field=field):
                self.reject_model(dataclasses.replace(self.model, **{field: EqualitySpoof(value)}))

    def test_direct_api_cannot_bypass_release_prerequisites(self) -> None:
        self.reject_model(dataclasses.replace(self.model, stage="release", pending=()))

    def test_direct_api_cannot_omit_required_pending(self) -> None:
        self.reject_model(dataclasses.replace(self.model, pending=self.model.pending[1:]))

    def test_direct_api_cannot_omit_planned_component(self) -> None:
        components = tuple(component for component in self.model.components if component.id != "broker")
        self.reject_model(dataclasses.replace(self.model, components=components))

    def test_direct_api_cannot_use_mutable_component_container(self) -> None:
        self.reject_model(dataclasses.replace(self.model, components=list(self.model.components)))

    def test_direct_api_cannot_use_mutable_pending_container(self) -> None:
        self.reject_model(dataclasses.replace(self.model, pending=list(self.model.pending)))

    def test_direct_api_cannot_use_mutable_input_container(self) -> None:
        self.reject_model(dataclasses.replace(self.model, inputs=list(self.model.inputs)))

    def test_direct_api_cannot_inject_component_filename(self) -> None:
        components = tuple(
            dataclasses.replace(component, binary_name="../../outside") if component.id == "ui" else component
            for component in self.model.components
        )
        self.reject_model(dataclasses.replace(self.model, components=components))

    def test_direct_api_cannot_inject_boolean_numeric_version(self) -> None:
        components = tuple(
            dataclasses.replace(component, file_version=(True, 0, 0, 0)) if component.id == "ui" else component
            for component in self.model.components
        )
        self.reject_model(dataclasses.replace(self.model, components=components))

    def test_direct_api_cannot_inject_oversize_numeric_version(self) -> None:
        components = tuple(
            dataclasses.replace(component, file_version=(65536, 0, 0, 0)) if component.id == "ui" else component
            for component in self.model.components
        )
        self.reject_model(dataclasses.replace(self.model, components=components))

    def test_direct_api_cannot_mislabel_component_license(self) -> None:
        components = tuple(
            dataclasses.replace(component, license="MIT") if component.id == "ui" else component
            for component in self.model.components
        )
        self.reject_model(dataclasses.replace(self.model, components=components))

    def test_direct_api_cannot_pass_an_untyped_record(self) -> None:
        self.reject_model({"cpu_floor": "x86-64-v3"})

    def test_parent_rename_cannot_redirect_temporary_writes(self) -> None:
        parent = self.base / "parent"
        parent.mkdir(mode=0o700)
        moved = self.base / "moved-parent"
        foreign = self.base / "foreign"
        foreign.mkdir(mode=0o700)
        (foreign / "private.txt").write_text(REJECTED_SENTINEL, encoding="utf-8")
        before = snapshot(foreign)
        original_open = self.generator._open_directory
        original_identity = self.generator._code_identity
        swapped = False
        during_generation: list[dict[str, bytes]] = []

        def swap_after_admission(path: Path, *, private: bool = False) -> int:
            nonlocal swapped
            descriptor = original_open(path, private=private)
            if path == parent and private and not swapped:
                parent.rename(moved)
                parent.symlink_to(foreign, target_is_directory=True)
                swapped = True
            return descriptor

        def observe_before_cleanup() -> list[dict[str, str]]:
            if swapped:
                during_generation.append(snapshot(foreign))
            return original_identity()

        with mock.patch.object(self.generator, "_open_directory", side_effect=swap_after_admission), mock.patch.object(
            self.generator, "_code_identity", side_effect=observe_before_cleanup
        ):
            try:
                self.generator.generate(self.root, parent / "generated")
            except (self.contract.MetadataError, OSError):
                pass
        self.assertTrue(swapped, "The actual parent-admission seam must be exercised")
        self.assertTrue(during_generation, "Observe foreign state before cleanup can mask writes")
        self.assertEqual(during_generation, [before])
        self.assertEqual(snapshot(foreign), before)

    def test_source_file_changed_during_read_is_rejected(self) -> None:
        original_read = self.contract.os.read
        changed = False

        def change_after_read(descriptor: int, length: int) -> bytes:
            nonlocal changed
            raw = original_read(descriptor, length)
            if not changed and raw.startswith(b"# SPDX-License-Identifier:"):
                self.record.write_bytes(self.record.read_bytes() + b"\n# changed during read\n")
                changed = True
            return raw

        with mock.patch.object(self.contract.os, "read", side_effect=change_after_read):
            with self.assertRaises(self.contract.MetadataError):
                self.contract.load_metadata(self.root)
        self.assertTrue(changed)

    def test_actual_partial_writes_are_completed(self) -> None:
        original_write = self.generator.os.write
        writes = 0

        def partial_write(descriptor: int, data: bytes) -> int:
            nonlocal writes
            writes += 1
            return original_write(descriptor, data[:max(1, len(data) // 2)])

        with mock.patch.object(self.generator.os, "write", side_effect=partial_write):
            result = self.generator.generate(self.root, self.output)
        self.assertEqual(result["status"], "generated-source")
        self.assertGreater(writes, len(EXPECTED_FILES))
        self.require_success(self.cli("check", output=self.output))

    def test_write_failure_removes_owned_staging_without_publication(self) -> None:
        before = snapshot(self.base)
        with mock.patch.object(self.generator.os, "write", side_effect=OSError("owned-write-failure")):
            with self.assertRaises(OSError):
                self.generator.generate(self.root, self.output)
        self.assertFalse(self.output.exists())
        self.assertEqual(snapshot(self.base), before)
        self.assertEqual({path.name for path in self.base.iterdir()}, {"source"})

    def test_staging_open_failure_removes_created_directory(self) -> None:
        original_open = self.generator.os.open
        reached_staging_open = False

        def fail_created_staging(path, flags, mode=0o777, *, dir_fd=None):
            nonlocal reached_staging_open
            if isinstance(path, str) and path.startswith(".nevrotick-metadata-") and dir_fd is not None:
                reached_staging_open = True
                raise OSError(errno.EMFILE, "owned descriptor exhaustion")
            return original_open(path, flags, mode, dir_fd=dir_fd)

        with mock.patch.object(self.generator.os, "open", side_effect=fail_created_staging) as patched_open:
            # Preserve the real capability prerequisite so this fault reaches
            # open-after-mkdir rather than the earlier platform-admission guard.
            with mock.patch.object(self.generator.os, "supports_dir_fd", self.generator.os.supports_dir_fd | {patched_open}):
                with self.assertRaises(OSError) as raised:
                    self.generator.generate(self.root, self.output)
        self.assertEqual(raised.exception.errno, errno.EMFILE)
        self.assertTrue(reached_staging_open)
        self.assertFalse(self.output.exists())
        self.assertEqual({path.name for path in self.base.iterdir()}, {"source"})

    def test_source_change_during_generation_prevents_publication(self) -> None:
        original_write = self.generator._write_payload
        changed = False

        def change_after_one_output(directory: int, relative: str, data: bytes) -> None:
            nonlocal changed
            original_write(directory, relative, data)
            if not changed:
                self.record.write_bytes(self.record.read_bytes() + b"\n# changed during generation\n")
                changed = True

        with mock.patch.object(self.generator, "_write_payload", side_effect=change_after_one_output):
            with self.assertRaises(self.contract.MetadataError):
                self.generator.generate(self.root, self.output)
        self.assertTrue(changed)
        self.assertFalse(self.output.exists())
        self.assertEqual({path.name for path in self.base.iterdir()}, {"source"})

    def test_output_enumeration_stops_at_entry_budget(self) -> None:
        self.generate()
        for number in range(100):
            (self.output / f"unexpected-{number:03}.txt").write_bytes(b"owned extra")
        original_scandir = self.generator.os.scandir
        visited = 0

        class CountingEntries:
            def __init__(inner, directory: int) -> None:
                inner.entries = original_scandir(directory)

            def __enter__(inner):
                inner.entries.__enter__()
                return inner

            def __exit__(inner, *args):
                return inner.entries.__exit__(*args)

            def __iter__(inner):
                return inner

            def __next__(inner):
                nonlocal visited
                visited += 1
                if visited > 65:
                    raise AssertionError("The output observer exceeded the fixed directory budget")
                return next(inner.entries)

        with mock.patch.object(self.generator.os, "scandir", side_effect=CountingEntries):
            with self.assertRaises(self.contract.MetadataError):
                self.generator.check(self.root, self.output)
        self.assertEqual(visited, 65)


if __name__ == "__main__":
    unittest.main()
