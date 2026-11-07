"""Source delivery contracts are checked independently of production encoders."""

from __future__ import annotations

import importlib
import dataclasses
import json
import shutil
import sys
import unittest
from unittest import mock
import xml.etree.ElementTree as ET

from test_pipeline import REPOSITORY, LITERAL_SENTINEL, SourceFixture, json_data


class ProducerDeliveryCliTests(SourceFixture):
    def copy_pipeline(self) -> None:
        """Use the real isolated CLI with an owned changed-producer fixture."""
        for relative in ("metadata/__init__.py",):
            destination = self.root / relative
            destination.write_bytes((REPOSITORY / relative).read_bytes())
        shutil.copytree(REPOSITORY / "metadata/pipeline", self.root / "metadata/pipeline")
        self.entrypoint = self.root / "metadata/pipeline/__main__.py"

    def test_wrong_producer_mapping_is_refused_before_publication(self) -> None:
        self.copy_pipeline()
        self.require_success(self.cli("generate", output=self.output))
        producer = self.root / "metadata/pipeline/generate.py"
        text = producer.read_text()
        old = '("PUBLISHER", model.product.publisher)'
        self.assertEqual(text.count(old), 1)
        producer.write_text(text.replace(old, '("PUBLISHER", model.product.name)'))
        invalid = self.base / "invalid"
        self.require_rejection(self.cli("generate", output=invalid))
        self.assertFalse(invalid.exists())

    def test_xml_encoding_defect_is_refused_before_publication(self) -> None:
        self.copy_pipeline()
        self.require_success(self.cli("generate", output=self.output))
        # The independent consumer receives bytes, including their declaration.
        healthy = (self.output / "authoring/display.xml").read_bytes()
        self.assertEqual(ET.fromstring(healthy).tag,
                         "{urn:nevrotick:metadata:authoring:1}metadata")
        producer = self.root / "metadata/pipeline/generate.py"
        text = producer.read_text()
        old = 'return ET.tostring(root, encoding="utf-8", xml_declaration=True) + b"\\n"'
        self.assertEqual(text.count(old), 1)
        changed = ('return (ET.tostring(root, encoding="utf-8", xml_declaration=True) '
                   '+ b"\\n").replace(b"encoding=\'utf-8\'", b"encoding=\'utf-16\'", 1)')
        producer.write_text(text.replace(old, changed))
        invalid = self.base / "invalid-xml"
        self.require_rejection(self.cli("generate", output=invalid))
        self.assertFalse(invalid.exists())

    def test_description_edit_reaches_required_views_and_old_tree_stays_stale(self) -> None:
        self.require_success(self.cli("generate", output=self.output))
        old = json_data(self.output / "metadata.json")["product"]["description"]
        new = LITERAL_SENTINEL + "\nSecond source line"
        self.replace_once('description = ' + json.dumps(old),
                          'description = ' + json.dumps(new, ensure_ascii=False))
        fresh = self.base / "fresh"
        self.require_success(self.cli("generate", output=fresh))
        self.require_success(self.cli("check", output=fresh))
        self.require_rejection(self.cli("check", output=self.output))
        self.assertEqual(json_data(self.output / "metadata.json")["product"]["description"], old)
        self.assertEqual(json_data(fresh / "metadata.json")["product"]["description"], new)
        # Independent literal/tree/cell observers, rather than the production checker.
        from test_pipeline import rust_string
        import csv
        import io
        import xml.etree.ElementTree as ET
        self.assertEqual(rust_string((fresh / "rust/product.rs").read_text(), "PRODUCT_DESCRIPTION"), new)
        xml = ET.fromstring((fresh / "authoring/display.xml").read_bytes())
        self.assertEqual(xml.find("{*}product/{*}description").text, new)
        rows = list(csv.reader(io.StringIO((fresh / "authoring/store.csv").read_text(), newline="")))
        self.assertEqual([row for row in rows if row[0] == "product.description"],
                         [["product.description", new, "derived"]])
        self.assertEqual(json_data(fresh / "authoring/msix-inputs.json")["description"], new)


class DeliveryObservationTests(SourceFixture):
    @classmethod
    def setUpClass(cls) -> None:
        sys.path.insert(0, str(REPOSITORY))
        try:
            cls.contract = importlib.import_module("metadata.pipeline.contract")
            cls.generator = importlib.import_module("metadata.pipeline.generate")
            cls.delivery = importlib.import_module("metadata.pipeline.delivery")
        finally:
            sys.path.remove(str(REPOSITORY))

    def setUp(self) -> None:
        super().setUp()
        self.model = self.contract.load_metadata(self.root)
        self.outputs = self.generator.build_outputs(self.model)

    def reject(self, outputs: object) -> None:
        with self.assertRaises(self.contract.MetadataError):
            self.delivery.verify_delivery(self.model, outputs)

    def json_change(self, name: str, change) -> dict[str, bytes]:
        outputs = self.outputs.copy()
        value = json.loads(outputs[name])
        change(value)
        outputs[name] = json.dumps(value, ensure_ascii=False).encode()
        return outputs

    def test_source_contract_reports_reached_artifacts_and_future_gaps(self) -> None:
        self.delivery.verify_delivery(self.model, self.outputs)
        report = json.loads(self.outputs["field-coverage.json"])["delivery"]
        self.assertEqual(report, self.delivery.delivery_contract(self.model))
        self.assertEqual(report["scope"], "generated-source")
        self.assertEqual(report["obligation_count"], len(report["obligations"]))
        self.assertEqual({row["artifact"] for row in report["artifact_contracts"]}, set(self.outputs))
        self.assertEqual(set(report["public_artifacts"]) & set(report["internal_only_artifacts"]), set())
        self.assertFalse(report["release_qualified"])
        for key in ("compiled_artifact_readback", "native_installed_readback", "package_privacy_readback"):
            self.assertEqual(report[key], "not-run")
        self.assertTrue(any(row["source"] == "platform.cpu_floor" for row in report["obligations"]))
        self.assertTrue(any(row["component"] == "ui" and row["artifact"] == "rc/ui.rc"
                            for row in report["obligations"]))
        self.assertTrue(all("value" not in row for row in report["obligations"]))

    def test_critical_report_bindings_have_independent_meaning(self) -> None:
        report = json.loads(self.outputs["field-coverage.json"])["delivery"]
        bindings = {
            (row["source"], row["component"], row["artifact"],
             row["locator"], row["representation"])
            for row in report["obligations"]
        }
        # Literal requirements for the current fixture/observer roles, rather
        # than values or expectations returned by delivery_contract().
        required = {
            ("product.publisher", "product", "metadata.json", "/product/publisher", "identity"),
            ("product.publisher", "ui", "rc/ui.rc", "CompanyName", "identity"),
            ("product.name", "observe", "rc/observe.rc", "ProductName", "identity"),
            ("platform.target", "product", "rust/product.rs", "TARGET", "identity"),
            ("platform.cpu_floor", "product", "metadata.json", "/cpu_floor", "identity"),
            ("component.ui.license", "ui", "rust/ui.rs", "LICENSE", "identity"),
            ("component.observe.license", "observe", "rust/observe.rs", "LICENSE", "identity"),
            ("component.ui.file_version", "ui", "rc/ui.rc", "FILEVERSION", "identity"),
            ("component.observe.version", "observe", "rc/observe.rc", "FileVersion", "identity"),
            ("component.ui.binary_name", "ui", "rust/ui.rs", "ORIGINAL_FILENAME",
             "append-.exe-if-absent"),
            ("component.ui.binary_name", "ui", "rc/ui.rc", "OriginalFilename",
             "append-.exe-if-absent"),
            ("component.ui.display_name", "ui", "authoring/display.xml",
             "/metadata/components/ui/name", "identity"),
            ("component.broker.scm.name", "broker", "authoring/service-inputs.json",
             "/services/broker/technical_name", "resolved-value-or-null"),
        }
        self.assertLessEqual(required, bindings)
        self.assertEqual(len(bindings), len(report["obligations"]))
        self.assertEqual(set(report["public_artifacts"]), {
            "metadata.json", "rust/product.rs", "rust/ui.rs", "rust/observe.rs",
            "rc/ui.rc", "rc/observe.rc",
        })
        self.assertEqual(set(report["internal_only_artifacts"]), {
            "field-register.json", "field-coverage.json", "authoring/display.xml",
            "authoring/msix-inputs.json", "authoring/service-inputs.json",
            "authoring/event-inputs.json", "authoring/store.csv", "authoring/project-inputs.json",
        })
        self.assertFalse(any(row["artifact"].startswith(("rust/engine", "rc/engine",
                                                       "rust/broker", "rc/broker",
                                                       "rust/helper", "rc/helper"))
                             for row in report["obligations"]))

    def test_agreeing_projections_cannot_replace_source_truth(self) -> None:
        outputs = self.json_change("metadata.json", lambda data: data["product"].__setitem__("publisher", "Wrong"))
        outputs["rust/product.rs"] = outputs["rust/product.rs"].replace(b'"David Osipov"', b'"Wrong"')
        for name in ("rc/ui.rc", "rc/observe.rc"):
            outputs[name] = outputs[name].replace(b'"David Osipov\\0"', b'"Wrong\\0"')
        self.reject(outputs)

    def test_same_count_component_and_license_swaps_are_refused(self) -> None:
        outputs = self.outputs.copy()
        outputs["rust/ui.rs"], outputs["rust/observe.rs"] = outputs["rust/observe.rs"], outputs["rust/ui.rs"]
        self.reject(outputs)
        outputs = self.outputs.copy()
        outputs["rust/ui.rs"] = outputs["rust/ui.rs"].replace(b'"GPL-3.0-only"', b'"GPL-2.0-only"')
        self.reject(outputs)

    def test_public_json_rejects_private_or_extra_fields(self) -> None:
        for name in ("metadata.json",):
            with self.subTest(name=name):
                self.reject(self.json_change(name, lambda data: data["public_metadata"].__setitem__(
                    "project.author.email", "PRIVATE-EMAIL@example.test")))

    def test_duplicate_json_keys_and_boolean_integer_confusion_are_refused(self) -> None:
        outputs = self.outputs.copy()
        outputs["metadata.json"] = b'{"schema_version":1,' + outputs["metadata.json"][1:]
        self.reject(outputs)
        self.reject(self.json_change("metadata.json", lambda data: data.__setitem__("schema_version", True)))

    def test_internal_register_omissions_and_duplicate_rows_are_refused(self) -> None:
        self.reject(self.json_change("field-register.json", lambda data: data["fields"].pop()))
        self.reject(self.json_change("authoring/project-inputs.json", lambda data: data["fields"].__setitem__(
            0, data["fields"][1])))

    def test_rust_and_resource_comments_cannot_publish_private_values(self) -> None:
        for name in ("rust/product.rs", "rc/ui.rc"):
            with self.subTest(name=name):
                outputs = self.outputs.copy()
                outputs[name] += b"// PRIVATE-EMAIL@example.test\n"
                self.reject(outputs)

    def test_rust_and_resource_unknown_statements_are_refused(self) -> None:
        for name, source in (("rust/ui.rs", b'include!("other.rs");\n'),
                             ("rc/ui.rc", b'#include "other.rc"\n')):
            with self.subTest(name=name):
                outputs = self.outputs.copy()
                outputs[name] += source
                self.reject(outputs)

    def test_duplicate_rust_constants_and_resource_values_are_refused(self) -> None:
        for name, marker in (("rust/product.rs", 'pub const PRODUCT_NAME:'),
                             ("rc/ui.rc", 'VALUE "ProductName",')):
            outputs = self.outputs.copy()
            text = outputs[name].decode()
            line = next(line for line in text.splitlines() if marker in line)
            if name.endswith(".rs"):
                line = "/// Reviewed metadata projection for `PRODUCT_NAME`.\n" + line
            outputs[name] = (text + line + "\n").encode()
            self.reject(outputs)

    def test_xml_unclaimed_text_entities_comments_and_extra_components_are_refused(self) -> None:
        original = self.outputs["authoring/display.xml"]
        changes = [
            original.replace(b"<product>", b"<product>PRIVATE"),
            original.replace(b"</product>", b"</product>PRIVATE"),
            original.replace(b"<product>", b"<!-- PRIVATE --><product>"),
            original.replace(b"<product>", b"<?unexpected PRIVATE?><product>"),
            original.replace(b"<product>", b'<!DOCTYPE metadata [<!ENTITY x "PRIVATE">]><product>'),
            original.replace(b'id="ui"', b'id="observe"'),
        ]
        for changed in changes:
            with self.subTest(change=changed[:100]):
                self.assertNotEqual(changed, original)
                outputs = self.outputs.copy(); outputs["authoring/display.xml"] = changed
                self.reject(outputs)

    def test_xml_encoding_declaration_matches_the_actual_byte_contract(self) -> None:
        name = "authoring/display.xml"
        original = self.outputs[name]
        self.assertEqual(ET.fromstring(original).tag,
                         "{urn:nevrotick:metadata:authoring:1}metadata")
        for encoding in (b"utf-16", b"iso-8859-1", b"unknown-private-charset"):
            with self.subTest(encoding=encoding):
                changed = original.replace(b"encoding='utf-8'",
                                           b"encoding='" + encoding + b"'", 1)
                self.assertNotEqual(changed, original)
                self.reject({**self.outputs, name: changed})

    def test_xml_generated_utf8_declaration_neighbors_remain_readable(self) -> None:
        name = "authoring/display.xml"
        original = self.outputs[name]
        for changed in (
            original.replace(b"encoding='utf-8'", b"encoding='UTF-8'", 1),
            original.replace(b"version='1.0' encoding='utf-8'",
                             b'version="1.0" encoding="UTF-8"', 1),
            original.replace(b"?>\n", b"?>\r\n", 1),
        ):
            with self.subTest(prefix=changed[:80]):
                self.assertNotEqual(changed, original)
                self.assertEqual(ET.fromstring(changed).tag,
                                 "{urn:nevrotick:metadata:authoring:1}metadata")
                self.delivery.verify_delivery(self.model, {**self.outputs, name: changed})

    def test_xml_requires_the_declared_generated_utf8_source_form(self) -> None:
        name = "authoring/display.xml"
        original = self.outputs[name]
        for changed in (b"\xef\xbb\xbf" + original, original.split(b"\n", 1)[1],
                        original.replace(b"version='1.0'", b"version='1.1'", 1),
                        original.replace(b"<product>", b"<product>\xff", 1)):
            with self.subTest(prefix=changed[:80]):
                self.assertNotEqual(changed, original)
                self.reject({**self.outputs, name: changed})

    def test_csv_duplicate_field_or_same_count_wrong_identity_is_refused(self) -> None:
        for replacement in (b'"product.name"', b'"wrong.name"'):
            outputs = self.outputs.copy()
            outputs["authoring/store.csv"] = outputs["authoring/store.csv"].replace(b'"product.publisher"', replacement)
            self.reject(outputs)

    def test_direct_observer_requires_exact_plain_types_paths_and_budgets(self) -> None:
        self.reject({})
        self.reject({**self.outputs, "extra.json": b"{}"})
        self.reject({**self.outputs, "metadata.json": bytearray(self.outputs["metadata.json"])})
        self.reject({**self.outputs, "metadata.json": b" " * (1024 * 1024 + 1)})
        class ForeignDict(dict):
            pass
        self.reject(ForeignDict(self.outputs))

    def test_direct_observer_and_report_cannot_bypass_model_contract(self) -> None:
        model = dataclasses.replace(self.model, cpu_floor="native")
        with self.assertRaises(self.contract.MetadataError):
            self.delivery.delivery_contract(model)
        with self.assertRaises(self.contract.MetadataError):
            self.delivery.verify_delivery(model, self.outputs)

    def test_read_only_check_independently_rechecks_actual_payloads(self) -> None:
        self.generate()
        with mock.patch.object(self.generator, "verify_delivery", wraps=self.delivery.verify_delivery) as observe:
            self.generator.check(self.root, self.output)
        # Once during expected generation, again for the actual held-file bytes.
        self.assertEqual(observe.call_count, 2)
        self.assertNotIn("receipt.json", observe.call_args.args[1])


if __name__ == "__main__":
    unittest.main()
