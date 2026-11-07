"""Actual source-admission bounds and semantic neighbors, without native effects."""

from __future__ import annotations

import dataclasses
import importlib
import json
import re
import sys
import tomllib
import unittest

from test_pipeline import INPUT_FILES, REPOSITORY, SourceFixture, json_data


def shape(value: object, depth: int = 0) -> tuple[int, int]:
    """Count decoded values/containers independently; keys are not value nodes."""
    children = value.values() if isinstance(value, dict) else value if isinstance(value, list) else ()
    measured = [shape(child, depth + 1) for child in children]
    return 1 + sum(count for count, _ in measured), max([depth, *(level for _, level in measured)])


class AdmissionFixture(SourceFixture):
    def set_field(self, key: str, value: str, status: str = "populated") -> None:
        """Change one real record; aliases must still resolve through source admission."""
        entry = {"value": value, "status": status,
                 "source": "Independent admission fixture", "reason": "Bounded source control"}
        line = json.dumps(key) + " = { " + ", ".join(
            name + " = " + json.dumps(text, ensure_ascii=False) for name, text in entry.items()) + " }"
        text, count = re.subn(r"(?m)^" + re.escape(json.dumps(key)) + r"\s*=.*$",
                              lambda _: line, self.record.read_text(encoding="utf-8"))
        self.assertEqual(count, 1)
        self.write_record(text)

    def expect_source(self, accepted: bool) -> None:
        result = self.cli("validate")
        if accepted:
            self.require_success(result)
        else:
            self.require_rejection(result)


class EmailAdmissionTests(AdmissionFixture):
    """Selected ASCII dot-atom/DNS grammar, not deliverability or ownership."""

    def test_invalid_email_dot_atoms_are_refused_before_publication(self) -> None:
        for value in (".person@example.test", "person.@example.test", "per..son@example.test"):
            with self.subTest(value=value):
                self.write_record(self.original_record)
                self.set_field("project.author.email", value)
                self.expect_source(False)
                self.require_rejection(self.cli("generate", output=self.output))
                self.assertFalse(self.output.exists())

    def test_valid_email_punctuation_and_case_are_preserved(self) -> None:
        for value in ("personal@david-osipov.vision", "First.Last+tag@Sub.Example.test",
                      "!#$%&'*+-/=?^_`{|}~@example.test"):
            with self.subTest(value=value):
                self.write_record(self.original_record)
                self.set_field("project.author.email", value)
                self.expect_source(True)
        self.generate()
        rows = {row["key"]: row for row in json_data(self.output / "field-register.json")["fields"]}
        self.assertEqual(rows["project.author.email"]["value"], value)
        self.assertNotIn("project.author.email", json_data(self.output / "metadata.json")["public_metadata"])

    def test_email_local_part_length_has_exact_neighbor(self) -> None:
        # RFC 5321 §4.5.3.1.1: this ASCII subset has one octet per character.
        for length, accepted in ((64, True), (65, False)):
            with self.subTest(length=length):
                self.write_record(self.original_record)
                self.set_field("project.author.email", "a" * length + "@example.test")
                self.expect_source(accepted)

    def test_email_domain_label_length_has_exact_neighbor(self) -> None:
        # RFC 1035 §2.3.4: DNS labels contain at most 63 octets.
        for length, accepted in ((63, True), (64, False)):
            with self.subTest(length=length):
                self.write_record(self.original_record)
                self.set_field("project.author.email", "person@" + "a" * length + ".test")
                self.expect_source(accepted)

    def test_email_total_length_keeps_the_existing_254_octet_bound(self) -> None:
        for last_length, accepted in ((61, True), (62, False)):
            value = "a" * 64 + "@" + "b" * 63 + "." + "c" * 63 + "." + "d" * last_length
            self.assertEqual(len(value), 193 + last_length)
            with self.subTest(length=len(value)):
                self.write_record(self.original_record)
                self.set_field("project.author.email", value)
                self.expect_source(accepted)

    def test_email_domain_empty_and_hyphen_edge_labels_are_refused(self) -> None:
        for value in ("person@example..test", "person@-example.test", "person@example-.test",
                      "person@exa_mple.test", "person@@example.test"):
            with self.subTest(value=value):
                self.write_record(self.original_record)
                self.set_field("project.author.email", value)
                self.expect_source(False)

    def test_pending_email_remains_empty_in_internal_records(self) -> None:
        self.set_field("project.author.email", "", "pending")
        self.generate()
        rows = {row["key"]: row for row in json_data(self.output / "field-register.json")["fields"]}
        self.assertEqual(rows["project.author.email"]["value"], "")
        self.assertEqual(rows["project.author.email"]["status"], "pending")


class StructureAdmissionTests(AdmissionFixture):
    """Exercise decoded limits through ignored-but-bounded Cargo metadata."""

    def setUp(self) -> None:
        super().setUp()
        self.cargo = self.root / "Cargo.toml"
        self.original_cargo = self.cargo.read_text(encoding="utf-8")
        self.prefix = self.original_cargo + "\n[workspace.metadata.nevrotick_admission_fixture]\n"

    def test_decoded_node_budget_has_exact_cli_neighbors(self) -> None:
        empty = self.prefix + "values = []\n"
        baseline, _ = shape(tomllib.loads(empty))
        for target, accepted in ((8192, True), (8193, False)):
            data = self.prefix + "values = [" + ",".join(["0"] * (target - baseline)) + "]\n"
            self.assertEqual(shape(tomllib.loads(data))[0], target)
            self.cargo.write_text(data, encoding="utf-8")
            with self.subTest(nodes=target):
                self.expect_source(accepted)

    def test_decoded_depth_budget_has_exact_cli_neighbors(self) -> None:
        for target, accepted in ((16, True), (17, False)):
            # workspace / metadata / fixture / value place its outer node at 4.
            levels = target - 4
            data = self.prefix + "value = " + "[" * levels + "0" + "]" * levels + "\n"
            self.assertEqual(shape(tomllib.loads(data))[1], target)
            self.cargo.write_text(data, encoding="utf-8")
            with self.subTest(depth=target):
                self.expect_source(accepted)

    def test_decoded_string_budget_has_exact_cli_neighbors(self) -> None:
        for size, accepted in ((16384, True), (16385, False)):
            data = self.prefix + "value = " + json.dumps("x" * size) + "\n"
            self.cargo.write_text(data, encoding="utf-8")
            with self.subTest(size=size):
                self.expect_source(accepted)

    def test_decoded_key_budget_has_exact_cli_neighbors(self) -> None:
        for size, accepted in ((16384, True), (16385, False)):
            data = self.prefix + json.dumps("k" * size) + " = 0\n"
            self.cargo.write_text(data, encoding="utf-8")
            with self.subTest(size=size):
                self.expect_source(accepted)

    def test_cargo_file_byte_budget_has_exact_cli_neighbors(self) -> None:
        for size, accepted in ((262144, True), (262145, False)):
            raw = self.original_cargo.encode("utf-8") + b"\n#"
            raw += b"x" * (size - len(raw))
            self.assertEqual(len(raw), size)
            self.cargo.write_bytes(raw)
            with self.subTest(bytes=size):
                self.expect_source(accepted)

    def test_four_maximum_files_fit_the_aggregate_budget(self) -> None:
        # Four unique inputs each bounded at 256 KiB exhaust the 1 MiB aggregate.
        # No reachable one-over aggregate case can keep all four per-file caps.
        total = 0
        for relative in INPUT_FILES:
            path = self.root / relative
            raw = path.read_bytes() + b"\n#"
            raw += b"x" * (262144 - len(raw))
            self.assertEqual(len(raw), 262144)
            path.write_bytes(raw)
            total += len(raw)
        self.assertEqual(total, 1048576)
        self.expect_source(True)


class StoreAdmissionTests(AdmissionFixture):
    def test_feature_count_and_width_limits_have_valid_neighbors(self) -> None:
        for count, width, accepted in ((20, 200, True), (21, 200, False),
                                        (1, 200, True), (1, 201, False)):
            with self.subTest(entries=count, width=width):
                self.write_record(self.original_record)
                self.set_field("project.store.features", "\n".join(["x" * width] * count))
                self.expect_source(accepted)

    def test_keyword_entry_word_and_width_limits_have_valid_neighbors(self) -> None:
        cases = ((["one two three"] * 7, True),
                 (["one two three"] * 6 + ["one two three four"], False),
                 (["one"] * 7, True), (["one"] * 8, False),
                 (["x" * 40], True), (["x" * 41], False))
        for entries, accepted in cases:
            with self.subTest(entries=entries, accepted=accepted):
                self.write_record(self.original_record)
                self.set_field("project.store.keywords", ";".join(entries))
                self.expect_source(accepted)

    def test_feature_and_keyword_empty_entries_are_refused(self) -> None:
        for key, value in (("project.store.features", "first\n\nthird"),
                           ("project.store.keywords", "first;;third")):
            with self.subTest(key=key):
                self.write_record(self.original_record)
                self.set_field(key, value)
                self.expect_source(False)


class ServiceMetadataAdmissionTests(AdmissionFixture):
    """Source grammars confer no native service/resource authority."""

    def test_service_name_uses_utf16_units_at_its_exact_limit(self) -> None:
        for role in ("engine", "broker"):
            for value, accepted in (("a" * 256, True), ("a" * 257, False),
                                    ("\U00010400" * 128, True), ("\U00010400" * 129, False)):
                with self.subTest(role=role, units=len(value.encode("utf-16-le")) // 2):
                    self.write_record(self.original_record)
                    self.set_field("component." + role + ".scm.name", value)
                    self.expect_source(accepted)

    def test_service_name_forbidden_separators_are_refused(self) -> None:
        for value in ("Nevro/Tick", "Nevro\\Tick"):
            with self.subTest(value=value):
                self.write_record(self.original_record)
                self.set_field("component.engine.scm.name", value)
                self.expect_source(False)

    def test_mui_reference_resource_id_has_exact_neighbors(self) -> None:
        for identifier, accepted in ((1, True), (65535, True), (0, False), (65536, False)):
            with self.subTest(identifier=identifier):
                self.write_record(self.original_record)
                self.set_field("component.engine.scm.display_resource", "@NevroTick.exe,-" + str(identifier))
                self.expect_source(accepted)

    def test_mui_reference_requires_the_owned_relative_literal_shape(self) -> None:
        for value in ("@../NevroTick.dll,-1", "@C:\\NevroTick.dll,-1", "@NevroTick.txt,-1",
                      "@NevroTick.dll,1", "@NevroTick.dll,-01", "@NevroTick.dll,-1 trailing"):
            with self.subTest(value=value):
                self.write_record(self.original_record)
                self.set_field("component.broker.scm.description_resource", value)
                self.expect_source(False)


class EmailModelAdmissionTests(AdmissionFixture):
    @classmethod
    def setUpClass(cls) -> None:
        sys.path.insert(0, str(REPOSITORY))
        try:
            cls.contract = importlib.import_module("metadata.pipeline.contract")
        finally:
            sys.path.remove(str(REPOSITORY))

    def test_direct_model_cannot_bypass_email_dot_and_component_bounds(self) -> None:
        model = self.contract.load_metadata(self.root)
        self.contract.validate_model(model)
        for key in ("project.author.email", "project.contacts.support_email", "project.contacts.regulatory_email"):
            for value in (".person@example.test", "person.@example.test", "per..son@example.test",
                          "a" * 65 + "@example.test", "person@" + "a" * 64 + ".test"):
                with self.subTest(key=key, value=value):
                    fields = tuple(dataclasses.replace(row, value=value, status="populated",
                        source="Independent forged model", reason="Email rule must remain mandatory")
                        if row.key == key else row for row in model.fields)
                    with self.assertRaises(self.contract.MetadataError):
                        self.contract.validate_model(dataclasses.replace(model, fields=fields))


if __name__ == "__main__":
    unittest.main()
