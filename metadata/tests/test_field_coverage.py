"""Check declared catalogue coverage and real expanded-pipeline consumers."""

from __future__ import annotations

from collections import Counter
import csv
import dataclasses
import hashlib
import importlib
import json
import re
import sys
import unittest

from test_pipeline import REPOSITORY, SourceFixture, json_data, rc_string, rust_string


def catalogue_keys() -> set[str]:
    """Read the declared field tables independently of the production registry."""
    lines = (REPOSITORY / "docs/supporting-reviews/windows-service-metadata-catalogue.md").read_text(
        encoding="utf-8"
    ).splitlines()
    section = ""
    keys: list[str] = []
    index = 0
    fixed = {"A.5", "A.7", "A.8", "B.5", "B.7", "C.3", "C.4", "C.5", "C.6", "C.7", "C.8"}
    prefixes = ("A.9.1", "A.9.2", "A.9.3", "A.9.4", "B.4.", "B.6.", "B.8.", "B.9.")
    while index < len(lines):
        heading = re.match(r"^#{3,4} ([ABC](?:\.[0-9]+)+)\.", lines[index])
        if heading:
            section = heading[1]
        selected = section in fixed or section.startswith(prefixes)
        if (selected and lines[index].startswith("|") and index + 1 < len(lines)
                and re.fullmatch(r"\|[ -:|]+\|", lines[index + 1])):
            index += 2
            while index < len(lines) and lines[index].startswith("|"):
                columns = re.split(r"(?<!\\)\|", lines[index])[1:-1]
                if columns:
                    field = columns[0].strip().replace("`", "")
                    keys.append(f"catalogue.{section}.{field}")
                index += 1
            continue
        index += 1
    if len(keys) != len(set(keys)):
        raise AssertionError("Catalogue oracle found duplicate scoped field identities")
    return set(keys)


class ExpandedFieldTests(SourceFixture):
    """Exercise authored fields through the real CLI and independent readers."""

    def set_field(self, key: str, value: str, status: str = "populated") -> None:
        """Replace one complete inline TOML field record without parsing helpers."""
        text = self.record.read_text(encoding="utf-8")
        entry = {"value": value, "status": status,
                 "source": "Owned independent metadata fixture", "reason": "Explicit fixture disposition"}
        encoded = ", ".join(f"{name} = {json.dumps(data, ensure_ascii=False)}" for name, data in entry.items())
        line = f"{json.dumps(key)} = {{ {encoded} }}"
        changed, count = re.subn(r"(?m)^" + re.escape(json.dumps(key)) + r"\s*=.*$", lambda _: line, text)
        self.assertEqual(count, 1, f"Expected one field record for {key}")
        self.write_record(changed)

    def field_rows(self) -> dict[str, dict]:
        """Observe the published register; no production lookup is an oracle."""
        self.generate()
        register = json_data(self.output / "field-register.json")
        self.assertEqual(register["scope"], "internal-authoring")
        rows = register["fields"]
        indexed = {row["key"]: row for row in rows}
        self.assertEqual(len(indexed), len(rows))
        return indexed

    def test_every_declared_catalogue_row_has_a_disposition(self) -> None:
        expected = catalogue_keys()
        self.assertEqual(Counter(key.split(".")[1] for key in expected), {"A": 88, "B": 267, "C": 196})
        rows = self.field_rows()
        actual = {key for key in rows if key.startswith("catalogue.")}
        self.assertEqual(actual, expected)
        for key in expected:
            with self.subTest(key=key):
                self.assertIn(rows[key]["status"], {"populated", "derived", "pending", "not-applicable"})
                self.assertTrue(rows[key]["owner"])
                self.assertTrue(rows[key]["source_ref"])
                self.assertTrue(rows[key]["reason"])

    def test_each_component_has_all_twelve_conventional_pe_strings(self) -> None:
        rows = self.field_rows()
        names = {"Comments", "CompanyName", "FileDescription", "FileVersion", "InternalName",
                 "LegalCopyright", "LegalTrademarks", "OriginalFilename", "PrivateBuild",
                 "ProductName", "ProductVersion", "SpecialBuild"}
        for component in ("ui", "observe", "engine", "broker", "helper"):
            with self.subTest(component=component):
                prefix = f"component.{component}.pe."
                self.assertEqual({key.removeprefix(prefix) for key in rows if key.startswith(prefix)}, names)

    def test_unknown_values_stay_empty_with_explicit_owners(self) -> None:
        rows = self.field_rows()
        for key in ("project.urls.support", "project.urls.privacy", "project.release.version",
                    "component.engine.scm.name", "component.broker.scm.name"):
            with self.subTest(key=key):
                self.assertEqual(rows[key]["value"], "")
                self.assertEqual(rows[key]["status"], "pending")
                self.assertTrue(rows[key]["owner"])
                self.assertTrue(rows[key]["reason"])
        self.assertEqual(rows["project.build.source_revision"]["value"], "")
        self.assertEqual(rows["project.build.source_revision"]["status"], "derived")
        public = json_data(self.output / "metadata.json")["public_metadata"]
        self.assertNotIn("project.urls.support", public)
        self.assertNotIn("SUPPORT_URL", (self.output / "rust/product.rs").read_text())

    def test_existing_fact_derivations_and_author_ids_reach_public_constants(self) -> None:
        rows = self.field_rows()
        expected = {"project.author.isni": "0000 0005 1802 960X", "project.author.orcid": "0009-0005-2713-9242",
                    "project.author.viaf": "139173726847611590332", "project.author.wikidata": "Q130604188"}
        for key, value in expected.items():
            with self.subTest(key=key):
                self.assertEqual(rows[key]["value"], value)
        self.assertEqual(rows["project.rights.core_license"]["value"], "GPL-2.0-only")
        self.assertEqual(rows["project.rights.gui_license"]["value"], "GPL-3.0-only")
        source = (self.output / "rust/product.rs").read_text()
        self.assertEqual(rust_string(source, "AUTHOR_ORCID"), expected["project.author.orcid"])
        self.assertNotIn("AUTHOR_EMAIL", source)
        self.assertNotIn("project.author.email", json_data(self.output / "metadata.json")["public_metadata"])

    def test_populated_copyright_reaches_only_appropriate_resource(self) -> None:
        value = 'Fixture rights "notice" & Я 漢字'
        self.set_field("component.ui.pe.LegalCopyright", value)
        self.generate()
        self.assertEqual(rc_string((self.output / "rc/ui.rc").read_text(), "LegalCopyright"), value)
        self.assertNotIn(value, (self.output / "rc/observe.rc").read_text())
        self.assertEqual(rust_string((self.output / "rust/ui.rs").read_text(), "COPYRIGHT"), value)

    def test_metadata_text_does_not_select_private_build_flags(self) -> None:
        self.set_field("component.ui.pe.PrivateBuild", "Unselected private classification")
        self.require_rejection(self.cli("generate", output=self.output))
        self.assertFalse(self.output.exists())

    def test_missing_individual_field_is_rejected(self) -> None:
        text, count = re.subn(r'(?m)^"project.urls.support"\s*=.*\n', "", self.original_record)
        self.assertEqual(count, 1)
        self.write_record(text)
        self.require_rejection(self.cli("generate", output=self.output))
        self.assertFalse(self.output.exists())

    def test_unknown_individual_field_is_rejected(self) -> None:
        self.write_record(self.original_record + '\n"unowned.field" = { value = "", status = "pending", source = "fixture", reason = "fixture" }\n')
        self.require_rejection(self.cli("validate"))

    def test_pending_field_cannot_contain_a_value(self) -> None:
        self.set_field("project.urls.support", "https://github.com/DavidOsipov/NevroTick", "pending")
        self.require_rejection(self.cli("validate"))

    def test_field_status_is_closed(self) -> None:
        self.set_field("project.urls.support", "", "approved-by-input")
        self.require_rejection(self.cli("validate"))

    def test_valid_populated_support_url_is_seeded(self) -> None:
        value = "https://github.com/DavidOsipov/NevroTick/issues"
        self.set_field("project.urls.support", value)
        self.generate()
        source = (self.output / "rust/product.rs").read_text()
        self.assertEqual(rust_string(source, "SUPPORT_URL"), value)
        self.assertEqual(json_data(self.output / "metadata.json")["public_metadata"]["project.urls.support"], value)

    def test_current_ui_locale_facts_cannot_claim_unimplemented_languages(self) -> None:
        for index, (key, value) in enumerate((
            ("project.localization.adopted_locales", "en;ru"),
            ("project.localization.fallback_locale", "ru"),
        )):
            with self.subTest(key=key):
                self.write_record(self.original_record)
                self.set_field(key, value)
                output = self.base / f"unadopted-locale-{index}"
                self.require_rejection(self.cli("generate", output=output))
                self.assertFalse(output.exists())

    def test_current_english_locale_facts_reach_public_projections(self) -> None:
        self.set_field("project.localization.adopted_locales", "en")
        self.set_field("project.localization.fallback_locale", "en")
        self.generate()
        public = json_data(self.output / "metadata.json")["public_metadata"]
        self.assertEqual(public["project.localization.adopted_locales"], "en")
        self.assertEqual(public["project.localization.fallback_locale"], "en")
        source = (self.output / "rust/product.rs").read_text()
        self.assertEqual(rust_string(source, "ADOPTED_LOCALES"), "en")
        self.assertEqual(rust_string(source, "FALLBACK_LOCALE"), "en")

    def test_coverage_counts_actual_dispositions_not_release_qualification(self) -> None:
        rows = self.field_rows()
        coverage = json_data(self.output / "field-coverage.json")
        self.assertTrue(coverage["complete_declared_field_set"])
        self.assertEqual(coverage["field_count"], len(rows))
        self.assertEqual(coverage["status_counts"], dict(Counter(row["status"] for row in rows.values())))
        self.assertEqual(coverage["resolved_value_count"], sum(bool(row["value"]) for row in rows.values()))
        self.assertFalse(coverage["release_qualified"])
        self.assertEqual(coverage["native_readback"], "not-run")

    def test_store_additional_license_terms_keep_their_own_limit_through_aliases(self) -> None:
        for size, accepted in ((8192, True), (8193, True), (10000, True), (10001, False)):
            with self.subTest(size=size):
                self.write_record(self.original_record)
                self.set_field("project.store.additional_license_terms", "x" * size)
                result = self.cli("validate")
                if accepted:
                    self.require_success(result)
                else:
                    self.require_rejection(result)

    def test_store_hardware_limits_apply_to_each_entry_and_the_whole_list(self) -> None:
        for key in ("project.store.hardware_minimum", "project.store.hardware_recommended"):
            for count, width, accepted in ((11, 200, True), (12, 200, False), (1, 201, False)):
                with self.subTest(key=key, count=count, width=width):
                    self.write_record(self.original_record)
                    self.set_field(key, "\n".join(["x" * width] * count))
                    result = self.cli("validate")
                    if accepted:
                        self.require_success(result)
                    else:
                        self.require_rejection(result)

    def test_valid_store_limits_reach_the_review_csv_through_catalogue_aliases(self) -> None:
        terms = "x" * 10000
        hardware = "\n".join(["x" * 200] * 11)
        self.set_field("project.store.additional_license_terms", terms)
        self.set_field("project.store.hardware_minimum", hardware)
        self.generate()
        with (self.output / "authoring/store.csv").open(encoding="utf-8", newline="") as stream:
            rows = list(csv.DictReader(stream))
        values = {row["field"]: row["value"] for row in rows}
        self.assertEqual(
            values["catalogue.B.9.1.StoreAdditional.Additional license terms"], terms
        )
        self.assertEqual(
            values["catalogue.B.9.1.StoreListing[locale].Additional system requirements.Minimum hardware"], hardware
        )

    def test_emoji_raw_comments_and_escaped_known_field_reject(self) -> None:
        for text in (self.original_record + "\n# emoji \U0001f600\n",
                     self.original_record.replace('description = "A Windows time synchronization client under development."',
                                                  'description = "bad \\U0001F600"', 1)):
            with self.subTest(kind="comment" if "# emoji" in text else "escaped"):
                self.write_record(text)
                self.require_rejection(self.cli("generate", output=self.output))
                self.assertFalse(self.output.exists())

    def test_emoji_escaped_field_record_cannot_bypass_decoding(self) -> None:
        self.set_field("component.ui.pe.LegalCopyright", "bad \\U0001F600")
        # Replace the TOML literal backslash with its valid TOML Unicode escape.
        self.write_record(self.record.read_text().replace("\\\\U0001F600", "\\U0001F600"))
        self.require_rejection(self.cli("validate"))

    def test_normal_international_text_and_nonbmp_letter_survive(self) -> None:
        value = "Letters Я 漢字 café \U00010400"
        self.set_field("component.ui.pe.LegalCopyright", value)
        self.write_record(self.record.read_text() + "\n# Ordinary Unicode: Русский 漢字 café\n")
        self.generate()
        self.assertEqual(rc_string((self.output / "rc/ui.rc").read_text(), "LegalCopyright"), value)

    def test_utf8_bom_and_invalid_utf8_are_rejected(self) -> None:
        for raw in (b"\xef\xbb\xbf" + self.original_record.encode(),
                    self.original_record.encode() + b"\n#\xff\n"):
            with self.subTest(prefix=raw[:3]):
                self.record.write_bytes(raw)
                self.require_rejection(self.cli("validate"))


class ExpandedModelTests(SourceFixture):
    """Direct-model callers must preserve the expanded field contract too."""

    @classmethod
    def setUpClass(cls) -> None:
        sys.path.insert(0, str(REPOSITORY))
        try:
            cls.contract = importlib.import_module("metadata.pipeline.contract")
            cls.generator = importlib.import_module("metadata.pipeline.generate")
            cls.registry = importlib.import_module("metadata.pipeline.fields")
        finally:
            sys.path.remove(str(REPOSITORY))

    def test_direct_model_cannot_omit_or_forge_a_field(self) -> None:
        model = self.contract.load_metadata(self.root)
        with self.assertRaises(self.contract.MetadataError):
            self.generator.build_outputs(dataclasses.replace(model, fields=model.fields[:-1]))
        target = next(field for field in model.fields if field.key == "project.urls.support")
        for patch in ({"status": "unowned"}, {"value": "not a url", "status": "populated"},
                      {"owner": "self-approved"}, {"value": "\U0001f600", "status": "populated"}):
            with self.subTest(patch=patch), self.assertRaises(self.contract.MetadataError):
                forged = dataclasses.replace(target, **patch)
                fields = tuple(forged if field.key == target.key else field for field in model.fields)
                self.generator.build_outputs(dataclasses.replace(model, fields=fields))

    def frozen_records(self) -> tuple[object, ...]:
        model = self.contract.load_metadata(self.root)
        return (model, model.product, model.components[0], model.pending[0], model.inputs[0],
                model.fields[0], self.registry.FIELD_SPECS[0])

    def test_all_seven_record_types_reject_ordinary_assignment_and_deletion(self) -> None:
        """Admitted records retain their values across normal Python caller use."""
        records = self.frozen_records()
        self.assertEqual({type(record).__name__ for record in records},
                         {"Metadata", "Product", "Component", "PendingField", "InputDigest", "FieldValue", "FieldSpec"})
        for record in records:
            with self.subTest(record=type(record).__name__):
                field = dataclasses.fields(record)[0].name
                original = getattr(record, field)
                legal_copy = dataclasses.replace(record)
                self.assertEqual(legal_copy, record)
                self.assertIsNot(legal_copy, record)
                with self.assertRaises(dataclasses.FrozenInstanceError):
                    setattr(record, field, original)
                with self.assertRaises(dataclasses.FrozenInstanceError):
                    delattr(record, field)
                self.assertEqual(getattr(record, field), original)

    def test_all_seven_record_types_have_fixed_attribute_layout(self) -> None:
        """Slots bound the record layout; they are not a hostile-code sandbox."""
        for record in self.frozen_records():
            with self.subTest(record=type(record).__name__):
                self.assertFalse(hasattr(record, "__dict__"))
                with self.assertRaises((AttributeError, TypeError)):
                    setattr(record, "unowned_extra_metadata_field", "unowned")
                self.assertFalse(hasattr(record, "unowned_extra_metadata_field"))

    def test_complete_field_register_has_reviewed_shape_values_and_catalogue_binding(self) -> None:
        """Bind all reviewed register data; a digest is not semantic approval."""
        expected_columns = ("key", "surface", "kind", "value_type", "owner", "applicability", "derivation", "source_ref")
        self.assertEqual(tuple(field.name for field in dataclasses.fields(self.registry.FieldSpec)), expected_columns)
        rows = [[getattr(spec, column) for column in expected_columns] for spec in self.registry.FIELD_SPECS]
        self.assertEqual(len(rows), 731)
        self.assertTrue(all(len(row) == 8 and all(type(value) is str for value in row) for row in rows))
        canonical = json.dumps(rows, ensure_ascii=False, separators=(",", ":")).encode("utf-8")
        self.assertEqual(hashlib.sha256(canonical).hexdigest(), "50ac89bb2a7eb3fe92aaf9319754cdb840f2aa44c3be89c5951a14ed9bf68724")
        indexed = self.registry.FIELD_BY_KEY
        self.assertEqual(set(indexed), {row[0] for row in rows})
        catalogue = {spec.key for spec in self.registry.FIELD_SPECS if spec.key.startswith("catalogue.")}
        self.assertEqual(catalogue, catalogue_keys())
        for spec in self.registry.FIELD_SPECS:
            if spec.key in catalogue:
                self.assertTrue(spec.source_ref.startswith("catalogue:"))
                self.assertTrue(spec.key.startswith("catalogue." + spec.source_ref.removeprefix("catalogue:") + "."))
        with self.assertRaises(TypeError):
            indexed[rows[0][0]] = indexed[rows[0][0]]
        with self.assertRaises(TypeError):
            del indexed[rows[0][0]]

    def test_public_field_counts_reject_forged_models(self) -> None:
        model = self.contract.load_metadata(self.root)
        actual = self.generator.field_counts(model)
        self.assertEqual(actual["field_count"], len(model.fields))
        expected_unresolved = sum(field.status == "pending" or
                                  (field.status == "derived" and not field.value)
                                  for field in model.fields)
        self.assertEqual(actual["unresolved_field_count"], expected_unresolved)
        with self.assertRaises(self.contract.MetadataError):
            self.generator.field_counts(dataclasses.replace(model, fields=()))

    def test_direct_model_cannot_claim_unimplemented_current_locales(self) -> None:
        model = self.contract.load_metadata(self.root)
        for key, value in (("project.localization.adopted_locales", "en;ru"),
                           ("project.localization.fallback_locale", "ru")):
            with self.subTest(key=key), self.assertRaises(self.contract.MetadataError):
                fields = tuple(dataclasses.replace(field, value=value) if field.key == key else field
                               for field in model.fields)
                self.generator.build_outputs(dataclasses.replace(model, fields=fields))


if __name__ == "__main__":
    unittest.main()
