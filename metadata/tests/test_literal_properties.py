"""Bounded source-fact properties with independent readers and literal answers."""

from __future__ import annotations

import csv
import dataclasses
import importlib
import io
import json
import sys
import unittest
import xml.etree.ElementTree as ET

from test_pipeline import REPOSITORY, SourceFixture, rc_string, rust_string


# Fixed construction is reproducible and finite. Adjacent atoms expose escaping
# interactions that a single combined sentinel cannot discriminate on its own.
ATOMS = (
    '"', "\\", '\\"', r"\0", r"\n", r"\u{10400}", '",\\',
    "<tag a=\"x\">&data</tag>", "<!-- literal -->", "<?literal data?>",
    "café", "e\u0301", "Я漢字", "\U00010400", " # * 0123 ", "comma,value",
)
SINGLE_LINE_VALUES = (
    "ordinary control",
    *("A" + atom + ATOMS[(index + shift) % len(ATOMS)] + "Z"
      for index, atom in enumerate(ATOMS) for shift in (1, 7)),
)
MULTILINE_VALUES = (
    "first\nsecond", "first\tsecond", 'first\n"second"\\third\tend',
    "first\nЯ漢字\te\u0301", "first\n\U00010400\tend", " first\nlast ",
)
XML_NAMESPACE = "{urn:nevrotick:metadata:authoring:1}"


class LiteralPropertyTests(SourceFixture):
    @classmethod
    def setUpClass(cls) -> None:
        sys.path.insert(0, str(REPOSITORY))
        try:
            cls.contract = importlib.import_module("metadata.pipeline.contract")
            cls.generator = importlib.import_module("metadata.pipeline.generate")
        finally:
            sys.path.remove(str(REPOSITORY))

    def setUp(self) -> None:
        super().setUp()
        self.base_model = self.contract.load_metadata(self.root)

    def model_with_descriptions(
        self, product: str, ui: str, observe: str,
    ):
        """Construct the explicit current aliases without production resolution."""
        values = {
            "catalogue.B.6.2.Package/Properties/Description": product,
            "component.ui.pe.Comments": ui,
            "component.ui.pe.FileDescription": ui,
            "component.observe.pe.Comments": observe,
            "component.observe.pe.FileDescription": observe,
        }
        return dataclasses.replace(
            self.base_model,
            product=dataclasses.replace(self.base_model.product, description=product),
            components=tuple(dataclasses.replace(component, description={"ui": ui, "observe": observe}.get(
                component.id, component.description)) for component in self.base_model.components),
            fields=tuple(dataclasses.replace(field, value=values[field.key])
                         if field.key in values else field for field in self.base_model.fields),
        )

    def assert_descriptions(self, outputs: dict[str, bytes], product: str, ui: str, observe: str) -> None:
        """Observe each promised value, not an encoder-generated expected string."""
        public = json.loads(outputs["metadata.json"])
        components = {row["id"]: row for row in public["components"]}
        self.assertEqual(public["product"]["description"], product)
        self.assertEqual(components["ui"]["description"], ui)
        self.assertEqual(components["observe"]["description"], observe)
        self.assertEqual(components["ui"]["license"], "GPL-3.0-only")
        self.assertEqual(components["observe"]["license"], "GPL-2.0-only")
        self.assertNotIn("project.author.email", public["public_metadata"])
        self.assertNotIn("fields", public)
        self.assertEqual(rust_string(outputs["rust/product.rs"].decode(), "PRODUCT_DESCRIPTION"), product)
        for identity, value in (("ui", ui), ("observe", observe)):
            rust = outputs[f"rust/{identity}.rs"].decode()
            resource = outputs[f"rc/{identity}.rc"].decode()
            self.assertEqual(rust_string(rust, "DESCRIPTION"), value)
            self.assertEqual(rust_string(rust, "COMMENTS"), value)
            self.assertEqual(rc_string(resource, "FileDescription"), value)
            self.assertEqual(rc_string(resource, "Comments"), value)
            self.assertEqual(rc_string(resource, "OriginalFilename"),
                             {"ui": "nevrotick-ui.exe", "observe": "nevrotick-observe.exe"}[identity])
        xml = ET.fromstring(outputs["authoring/display.xml"])
        self.assertEqual(xml.findtext(XML_NAMESPACE + "product/" + XML_NAMESPACE + "description"), product)
        xml_components = {node.attrib["id"]: node for node in xml.find(XML_NAMESPACE + "components")}
        self.assertEqual(xml_components["ui"].findtext(XML_NAMESPACE + "description"), ui)
        self.assertEqual(xml_components["observe"].findtext(XML_NAMESPACE + "description"), observe)
        rows = list(csv.DictReader(io.StringIO(outputs["authoring/store.csv"].decode(), newline="")))
        self.assertEqual([row["value"] for row in rows if row["field"] == "product.description"], [product])
        self.assertEqual(json.loads(outputs["authoring/msix-inputs.json"])["description"], product)
        fields = {row["key"]: row for row in json.loads(outputs["field-register.json"])["fields"]}
        for key, value in (
            ("catalogue.B.6.2.Package/Properties/Description", product),
            ("component.ui.pe.Comments", ui), ("component.ui.pe.FileDescription", ui),
            ("component.observe.pe.Comments", observe), ("component.observe.pe.FileDescription", observe),
        ):
            self.assertEqual(fields[key]["value"], value)

    def test_generated_single_line_interactions_preserve_each_role_and_view(self) -> None:
        self.assertLessEqual(len(SINGLE_LINE_VALUES) + len(MULTILINE_VALUES), 100)
        for value in SINGLE_LINE_VALUES:
            with self.subTest(value=repr(value)):
                texts = ("Product:" + value, "UI:" + value, "Observer:" + value)
                model = self.model_with_descriptions(*texts)
                self.assert_descriptions(self.generator.build_outputs(model), *texts)

    def test_generated_multiline_product_values_preserve_rows_and_text(self) -> None:
        # Component descriptions intentionally remain single-line. Product
        # LF/tab admission is a different partition, not a filter on failures.
        for value in MULTILINE_VALUES:
            with self.subTest(value=repr(value)):
                texts = (value, "UI single-line control", "Observer single-line control")
                model = self.model_with_descriptions(*texts)
                self.assert_descriptions(self.generator.build_outputs(model), *texts)

    def test_nonbmp_description_limit_uses_utf16_units_before_projection(self) -> None:
        product = "p" * 4094 + "\U00010400"  # 4,096 UTF-16 units.
        ui = "u" * 1022 + "\U00010400"  # 1,024 UTF-16 units.
        model = self.model_with_descriptions(product, ui, "Observer control")
        self.assert_descriptions(self.generator.build_outputs(model), product, ui, "Observer control")
        for texts in ((product + "x", ui, "Observer control"),
                      (product, ui + "x", "Observer control")):
            with self.subTest(lengths=tuple(len(value) for value in texts)):
                with self.assertRaises(self.contract.MetadataError):
                    self.generator.build_outputs(self.model_with_descriptions(*texts))

    def test_control_characters_fail_at_the_actual_typed_generation_boundary(self) -> None:
        for value in ("a\0b", "a\rb", "a\u200bb", "a\u202eb"):
            with self.subTest(value=repr(value)):
                with self.assertRaises(self.contract.MetadataError):
                    self.generator.build_outputs(self.model_with_descriptions(value, "UI control", "Observe control"))
        for value in ("a\nb", "a\tb", "a\rb"):
            with self.subTest(component_value=repr(value)):
                with self.assertRaises(self.contract.MetadataError):
                    self.generator.build_outputs(self.model_with_descriptions("Product control", value, "Observe control"))

    def test_private_contact_is_not_promoted_with_optional_component_rights(self) -> None:
        copyright_value = 'Copyright example contributor "quoted" \\ literal'
        fields = tuple(
            dataclasses.replace(field, value=value, status="populated",
                                source="Independent rights/contact fixture", reason="Scope control")
            if (value := {"project.author.email": "private+fixture@example.test",
                          "component.ui.pe.LegalCopyright": copyright_value}.get(field.key)) is not None
            else field
            for field in self.base_model.fields
        )
        outputs = self.generator.build_outputs(dataclasses.replace(self.base_model, fields=fields))
        self.assertEqual(rust_string(outputs["rust/ui.rs"].decode(), "COPYRIGHT"), copyright_value)
        self.assertEqual(rc_string(outputs["rc/ui.rc"].decode(), "LegalCopyright"), copyright_value)
        self.assertNotIn(b"private+fixture@example.test", outputs["metadata.json"])
        self.assertNotIn(b"private+fixture@example.test", outputs["rust/product.rs"])
        self.assertNotIn(b"private+fixture@example.test", outputs["rc/ui.rc"])
        self.assertIn(b"private+fixture@example.test", outputs["authoring/project-inputs.json"])
        self.assertNotIn("COPYRIGHT", {
            line.split()[2].rstrip(":") for line in outputs["rust/observe.rs"].decode().splitlines()
            if line.startswith("pub const ")
        })

    def test_representative_literal_properties_cross_the_real_source_cli(self) -> None:
        for index, value in enumerate((
            'Quote:" Slash:\\ Pair:\\" Text:\\0\\n <&data>',
            "First\nSecond\tЯ漢字 e\u0301 \U00010400",
        )):
            with self.subTest(value=repr(value)):
                old = 'description = "A Windows time synchronization client under development."'
                self.assertEqual(self.original_record.count(old), 1)
                # TOML uses scalar Unicode escapes; JSON's default surrogate
                # pairs are not a valid way to author a non-BMP TOML string.
                self.write_record(self.original_record.replace(
                    old, "description = " + json.dumps(value, ensure_ascii=False)))
                target = self.base / f"literal-{index}"
                self.require_success(self.cli("generate", output=target))
                self.require_success(self.cli("check", output=target))
                outputs = {path.relative_to(target).as_posix(): path.read_bytes()
                           for path in target.rglob("*") if path.is_file()}
                self.assert_descriptions(outputs, value, "Fixture-only NevroTick UI-M0 prototype",
                                         "Bounded unauthenticated, observe-only transport experiment")


if __name__ == "__main__":
    unittest.main()
