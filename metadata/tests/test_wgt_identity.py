"""Source-only WGT identity admission and internal/public delivery controls."""
from __future__ import annotations

import dataclasses
import hashlib
import importlib
import json
from pathlib import Path
import re
from subprocess import CompletedProcess
import sys
from types import ModuleType
from typing import TYPE_CHECKING, ClassVar

from test_pipeline import REJECTED_SENTINEL, REPOSITORY, SourceFixture, json_data, snapshot

if TYPE_CHECKING:
    from metadata.pipeline.contract import FieldValue, Metadata


IDENTITIES: tuple[str, ...] = (
    "project.distribution.community.source_identity",
    "project.distribution.community.package_identifier",
    "project.distribution.msix.package_family_name",
)
MSIX_IDENTITIES: tuple[str, ...] = (
    "catalogue.B.6.1.Package/Identity/@Name",
    "catalogue.B.6.1.Package/Identity/@Publisher",
)
PUBLIC: tuple[str, ...] = (
    "metadata.json", "rust/product.rs", "rust/ui.rs", "rust/observe.rs",
    "rc/ui.rc", "rc/observe.rc",
)


def _record(value: object) -> dict[str, object]:
    """Read only the string-keyed object shape needed by the fixture observer."""
    if not isinstance(value, dict) or any(not isinstance(key, str) for key in value):
        raise AssertionError("Expected a JSON object with string keys")
    return value


def _records(value: object) -> list[dict[str, object]]:
    """Check the list shape and return the original mutable row objects."""
    if not isinstance(value, list):
        raise AssertionError("Expected a JSON list of objects")
    return [_record(row) for row in value]


def _field(record: dict[str, object], key: str) -> dict[str, object]:
    """Find exactly one row so duplicate projections cannot satisfy a check."""
    matches: list[dict[str, object]] = [row for row in _records(record["fields"]) if row["key"] == key]
    if len(matches) != 1:
        raise AssertionError(f"Expected one field row for {key}")
    return matches[0]


class WgtIdentityTests(SourceFixture):
    contract: ClassVar[ModuleType]
    generator: ClassVar[ModuleType]
    delivery: ClassVar[ModuleType]

    @classmethod
    def setUpClass(cls) -> None:
        sys.path.insert(0, str(REPOSITORY))
        try:
            cls.contract = importlib.import_module("metadata.pipeline.contract")
            cls.generator = importlib.import_module("metadata.pipeline.generate")
            cls.delivery = importlib.import_module("metadata.pipeline.delivery")
        finally:
            sys.path.remove(str(REPOSITORY))

    def field_line(self, key: str) -> str:
        matches: list[str] = re.findall(
            r"(?m)^" + re.escape(json.dumps(key)) + r"\s*=.*$", self.original_record,
        )
        self.assertEqual(len(matches), 1)
        return matches[0]

    def test_current_source_reports_five_distinct_pending_facts(self) -> None:
        self.generate()
        register: dict[str, object] = _record(json_data(self.output / "field-register.json"))
        for key in (*IDENTITIES, *MSIX_IDENTITIES):
            with self.subTest(key=key):
                field: dict[str, object] = _field(register, key)
                self.assertEqual((field["value"], field["status"], field["owner"]),
                                 ("", "pending", "W11-W10/P6"))
                for member in ("source", "reason"):
                    self.assertIsInstance(field[member], str)
                    self.assertTrue(field[member])
        project_record: dict[str, object] = _record(json_data(self.output / "authoring/project-inputs.json"))
        for key in IDENTITIES:
            with self.subTest(project_key=key):
                field = _field(project_record, key)
                self.assertEqual((field["value"], field["status"], field["owner"]),
                                 ("", "pending", "W11-W10/P6"))
        msix: dict[str, object] = _record(json_data(self.output / "authoring/msix-inputs.json"))
        self.assertIsNone(msix["identity"])
        self.assertIs(msix["complete_manifest"], False)
        self.require_success(self.cli("check", output=self.output))
        report: dict[str, object] = _record(self.require_success(self.cli("inspect-msix", output=self.output)))
        self.assertIsNone(report["package_identity"])
        self.assertIs(report["assembly_ready"], False)
        self.assertEqual(report["roles"], {
            "ui": {"source_state": "fixture", "artifact_status": "not-admitted"},
            "engine": {"source_state": "planned", "artifact_status": "not-admitted"},
            "broker": {"source_state": "planned", "artifact_status": "not-admitted"},
            "helper": {"source_state": "planned", "artifact_status": "not-admitted"},
        })

    def test_raw_admission_refuses_unowned_dispositions_without_effects(self) -> None:
        self.generate()
        before: dict[str, bytes] = snapshot(self.output)
        for key in IDENTITIES:
            line: str = self.field_line(key)
            cases: tuple[str, ...] = (
                line.replace('status = "pending"', 'status = "derived"'),
                line.replace('status = "pending"', 'status = "not-applicable"'),
                line.replace('value = ""', f'value = "{REJECTED_SENTINEL}"'),
                line.replace('status = "pending"', 'status = "populated"').replace('value = ""', f'value = "{REJECTED_SENTINEL}"'),
                re.sub(r'source = "[^"]*"', 'source = ""', line),
                line + '\n' + line,
                line + '\n' + line.replace('"project.', '"\\u0070roject.', 1),
                line.replace(key, key + ".wrong"),
                '',
                line.replace('value = ""', 'value = 42'),
                line.replace('value = ""', 'value = "", owner = "Other"'),
            )
            for changed in cases:
                with self.subTest(key=key, changed=changed):
                    self.assertNotEqual(changed, line)
                    self.write_record(self.original_record.replace(line, changed, 1))
                    candidate: Path = self.base / "rejected"
                    result: CompletedProcess[str] = self.cli("generate", output=candidate)
                    self.require_rejection(result)
                    self.assertNotIn(REJECTED_SENTINEL, result.stdout + result.stderr)
                    self.assertFalse(candidate.exists())
                    existing: CompletedProcess[str] = self.cli("generate", output=self.output)
                    self.require_rejection(existing)
                    self.assertNotIn(REJECTED_SENTINEL, existing.stdout + existing.stderr)
                    self.assertEqual(snapshot(self.output), before)
        self.write_record(self.original_record)
        self.require_success(self.cli("check", output=self.output))

    def test_direct_model_cannot_bypass_pending_only_invariants(self) -> None:
        model: Metadata = self.contract.load_metadata(self.root)
        self.generator.build_outputs(model)
        patches: tuple[dict[str, str | None], ...] = (
            {"status": "derived"}, {"status": "not-applicable"},
            {"value": "FixtureOnly"}, {"status": "populated", "value": "FixtureOnly"},
            {"owner": "unowned"}, {"source": ""}, {"value": None},
        )
        for key in IDENTITIES:
            target: FieldValue = next(field for field in model.fields if field.key == key)
            for patch in patches:
                with self.subTest(key=key, patch=patch):
                    changed: FieldValue = dataclasses.replace(target, **patch)
                    fields: tuple[FieldValue, ...] = tuple(
                        changed if row.key == key else row for row in model.fields
                    )
                    with self.assertRaises(self.contract.MetadataError):
                        self.generator.build_outputs(dataclasses.replace(model, fields=fields))

    def test_independent_delivery_rejects_forged_identity_and_public_leak(self) -> None:
        model: Metadata = self.contract.load_metadata(self.root)
        outputs: dict[str, bytes] = self.generator.build_outputs(model)
        self.delivery.verify_delivery(model, outputs)
        for name, key in (("field-register.json", IDENTITIES[0]),
                          ("authoring/project-inputs.json", IDENTITIES[1])):
            forged: dict[str, bytes] = outputs.copy()
            value: dict[str, object] = _record(json.loads(forged[name]))
            _field(value, key)["value"] = "FixtureOnly"
            forged[name] = json.dumps(value).encode()
            with self.subTest(name=name), self.assertRaises(self.contract.MetadataError):
                self.delivery.verify_delivery(model, forged)
        forged = outputs.copy()
        value = _record(json.loads(forged["authoring/project-inputs.json"]))
        _field(value, IDENTITIES[0])["key"] = IDENTITIES[0] + ".wrong"
        forged["authoring/project-inputs.json"] = json.dumps(value).encode()
        with self.assertRaises(self.contract.MetadataError):
            self.delivery.verify_delivery(model, forged)
        forged = outputs.copy()
        value = _record(json.loads(forged["metadata.json"]))
        public_metadata: dict[str, object] = _record(value["public_metadata"])
        public_metadata["private_sentinel"] = "PrivateIdentitySentinel"
        forged["metadata.json"] = json.dumps(value).encode()
        with self.assertRaises(self.contract.MetadataError):
            self.delivery.verify_delivery(model, forged)
        self.delivery.verify_delivery(model, outputs)

    def test_rebound_receipt_does_not_admit_a_forged_project_identity(self) -> None:
        self.generate()
        path: Path = self.output / "authoring/project-inputs.json"
        receipt_path: Path = self.output / "receipt.json"
        original: bytes = path.read_bytes()
        receipt_original: bytes = receipt_path.read_bytes()
        value: dict[str, object] = _record(json.loads(original))
        _field(value, IDENTITIES[0])["value"] = REJECTED_SENTINEL
        altered: bytes = (json.dumps(value, ensure_ascii=False, indent=2, sort_keys=True) + "\n").encode()
        receipt: dict[str, object] = _record(json.loads(receipt_original))
        item: dict[str, object] = next(
            item for item in _records(receipt["files"]) if item["path"] == "authoring/project-inputs.json"
        )
        self.assertEqual(item["sha256"], hashlib.sha256(original).hexdigest())
        item.update(bytes=len(altered), sha256=hashlib.sha256(altered).hexdigest())
        path.write_bytes(altered)
        receipt_path.write_text(json.dumps(receipt, ensure_ascii=False, indent=2, sort_keys=True) + "\n")
        forged: dict[str, bytes] = snapshot(self.output)
        result: CompletedProcess[str] = self.cli("check", output=self.output)
        self.require_rejection(result)
        self.assertNotIn(REJECTED_SENTINEL, result.stdout + result.stderr)
        self.assertEqual(snapshot(self.output), forged)
        path.write_bytes(original)
        receipt_path.write_bytes(receipt_original)
        self.require_success(self.cli("check", output=self.output))

    def test_internal_sentinel_never_reaches_any_public_projection(self) -> None:
        sentinel: str = "PrivateIdentitySentinel97531"
        line: str = self.field_line(IDENTITIES[0])
        replaced: str = re.sub(r'source = "[^"]*"', f'source = "{sentinel}"', line)
        self.assertNotEqual(line, replaced)
        self.write_record(self.original_record.replace(line, replaced, 1))
        self.generate()
        register: dict[str, object] = _record(json_data(self.output / "field-register.json"))
        project: dict[str, object] = _record(json_data(self.output / "authoring/project-inputs.json"))
        self.assertEqual(_field(register, IDENTITIES[0])["source"], sentinel)
        self.assertEqual(_field(project, IDENTITIES[0])["source"], sentinel)
        descriptor: dict[str, object] = _record(json_data(self.output / "metadata.json"))
        self.assertNotIn(sentinel, json.dumps(descriptor, ensure_ascii=False))
        product: dict[str, object] = _record(descriptor["product"])
        self.assertEqual(product["name"], "NevroTick")
        coverage: dict[str, object] = _record(json_data(self.output / "field-coverage.json"))
        report: dict[str, object] = _record(coverage["delivery"])
        public_artifacts: object = report["public_artifacts"]
        if not isinstance(public_artifacts, list):
            self.fail("Expected a list of public artifact paths")
        self.assertCountEqual(public_artifacts, PUBLIC)
        for name in PUBLIC:
            with self.subTest(name=name):
                self.assertNotIn(sentinel, (self.output / name).read_text(encoding="utf-8"))
        self.require_success(self.cli("check", output=self.output))

    def test_changed_identity_source_makes_the_old_tree_stale(self) -> None:
        self.generate()
        before: dict[str, bytes] = snapshot(self.output)
        line: str = self.field_line(IDENTITIES[1])
        source: str = "Independent fixture source"
        changed: str = re.sub(r'source = "[^"]*"', f'source = "{source}"', line)
        self.assertNotEqual(line, changed)
        self.write_record(self.original_record.replace(line, changed, 1))
        self.require_rejection(self.cli("check", output=self.output))
        self.assertEqual(snapshot(self.output), before)
        fresh: Path = self.base / "fresh"
        self.require_success(self.cli("generate", output=fresh))
        self.require_success(self.cli("check", output=fresh))
        project: dict[str, object] = _record(json_data(fresh / "authoring/project-inputs.json"))
        self.assertEqual(_field(project, IDENTITIES[1])["source"], source)
