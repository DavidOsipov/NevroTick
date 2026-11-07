"""Independent source-delivery obligations and bounded projection readers.

This checker interprets only this pipeline's declared source subset. It imports
no production encoders. Its observations are neither Rust/RC compilation nor PE,
installed-service, MSIX, signature or Store readback. Caller custody still owns
filesystem reads; this module accepts bounded in-memory immutable byte strings.
"""

from __future__ import annotations

from collections import Counter
import csv
from dataclasses import asdict
import io
import json
import re
import xml.etree.ElementTree as ET

from .contract import Metadata, MetadataError, validate_model
from .fields import FIELD_BY_KEY

_MAX_FILE = 1024 * 1024
_MAX_TOTAL = 4 * _MAX_FILE
_NAMESPACE = "urn:nevrotick:metadata:authoring:1"
# Deliberately independent of generate.PUBLIC_STRINGS: a wrong producer mapping
# must not redefine the expected delivery contract at the same time.
_PUBLIC_FACTS = (
    ("project.author.name", "AUTHOR_NAME"), ("project.author.role", "AUTHOR_ROLE"),
    ("project.author.website", "AUTHOR_WEBSITE"), ("project.author.isni", "AUTHOR_ISNI"),
    ("project.author.orcid", "AUTHOR_ORCID"), ("project.author.viaf", "AUTHOR_VIAF"),
    ("project.author.wikidata", "AUTHOR_WIKIDATA"), ("project.urls.homepage", "HOMEPAGE"),
    ("project.urls.documentation", "DOCUMENTATION_URL"), ("project.urls.support", "SUPPORT_URL"),
    ("project.urls.privacy", "PRIVACY_URL"), ("project.urls.license", "LICENSE_URL"),
    ("project.urls.release_notes", "RELEASE_NOTES_URL"), ("project.rights.copyright", "COPYRIGHT"),
    ("project.rights.trademarks", "TRADEMARKS"),
    ("project.localization.adopted_locales", "ADOPTED_LOCALES"),
    ("project.localization.fallback_locale", "FALLBACK_LOCALE"),
    ("project.release.channel", "RELEASE_CHANNEL"),
)
_INTERNAL = (
    "field-register.json", "field-coverage.json", "authoring/display.xml",
    "authoring/msix-inputs.json", "authoring/service-inputs.json",
    "authoring/event-inputs.json", "authoring/store.csv", "authoring/project-inputs.json",
)


def _values(model: Metadata) -> dict[str, str]:
    """Select resolved facts; pending/absent values acquire no delivery value."""
    return {row.key: row.value for row in model.fields
            if row.status in {"populated", "derived"} and row.value}


def delivery_contract(model: Metadata) -> dict[str, object]:
    """Describe source-derived destinations without exposing their private values.

    Every listed obligation is for generated source only. The report is another
    projection of code-owned rules, never an editable authority or native pass.
    """
    validate_model(model)
    obligations: list[dict[str, str]] = []

    def add(source: str, artifact: str, locator: str, component: str = "product",
            representation: str = "identity") -> None:
        obligations.append({"source": source, "component": component,
                            "artifact": artifact, "locator": locator,
                            "representation": representation})

    for field in ("name", "publisher", "description", "repository"):
        add("product." + field, "metadata.json", "/product/" + field)
        add("product." + field, "authoring/display.xml", "/metadata/product/" + field)
        add("product." + field, "authoring/store.csv", "product." + field)
    for field, constant in (("name", "PRODUCT_NAME"), ("publisher", "PUBLISHER"),
                            ("description", "PRODUCT_DESCRIPTION"), ("repository", "REPOSITORY")):
        add("product." + field, "rust/product.rs", constant)
    for source, constant, locator in (("platform.target", "TARGET", "target"),
                                       ("platform.cpu_floor", "CPU_FLOOR", "cpu_floor")):
        add(source, "rust/product.rs", constant)
        add(source, "metadata.json", "/" + locator)
        add(source, "authoring/msix-inputs.json", "/" + locator)
    values = _values(model)
    if model.product.homepage is not None:
        add("product.homepage", "metadata.json", "/product/homepage")
        add("product.homepage", "authoring/store.csv", "product.homepage")
    for key, constant in _PUBLIC_FACTS:
        if key in values:
            add(key, "metadata.json", "/public_metadata/" + key)
            add(key, "rust/product.rs", constant)
    for component in model.components:
        for field in ("id", "role", "state", "display_name", "description", "binary_name",
                      "version", "file_version", "license", "cargo_manifest"):
            add("component." + component.id + "." + field, "metadata.json",
                "/components/" + component.id + "/" + field, component.id)
        for field in ("display_name", "description"):
            add("component." + component.id + "." + field, "authoring/display.xml",
                "/metadata/components/" + component.id + "/" +
                ("name" if field == "display_name" else field), component.id)
        if component.id in {"engine", "broker"}:
            for field in ("role", "state", "display_name", "description"):
                add("component." + component.id + "." + field, "authoring/service-inputs.json",
                    "/services/" + component.role + "/" + field, component.id)
            add("component." + component.id + ".scm.name", "authoring/service-inputs.json",
                "/services/" + component.role + "/technical_name", component.id,
                "resolved-value-or-null")
        if component.emittable:
            for field, constant in (("id", "COMPONENT_ID"), ("role", "ROLE"),
                                    ("display_name", "DISPLAY_NAME"), ("description", "DESCRIPTION"),
                                    ("version", "VERSION"), ("license", "LICENSE"),
                                    ("binary_name", "ORIGINAL_FILENAME"), ("file_version", "FILE_VERSION")):
                add("component." + component.id + "." + field,
                    "rust/" + component.id + ".rs", constant, component.id,
                    "append-.exe-if-absent" if constant == "ORIGINAL_FILENAME" else "identity")
            for source, target in (("product.publisher", "CompanyName"), ("product.name", "ProductName"),
                                   ("component." + component.id + ".description", "FileDescription"),
                                   ("component." + component.id + ".version", "FileVersion"),
                                   ("component." + component.id + ".version", "ProductVersion"),
                                   ("component." + component.id + ".binary_name", "InternalName"),
                                   ("component." + component.id + ".binary_name", "OriginalFilename"),
                                   ("component." + component.id + ".file_version", "FILEVERSION"),
                                   ("component." + component.id + ".file_version", "PRODUCTVERSION")):
                add(source, "rc/" + component.id + ".rc", target, component.id,
                    "append-.exe-if-absent" if target == "OriginalFilename" else "identity")
            for source, constant in (("LegalCopyright", "COPYRIGHT"),
                                     ("LegalTrademarks", "TRADEMARKS"), ("Comments", "COMMENTS")):
                key = "component." + component.id + ".pe." + source
                if key in values:
                    add(key, "rust/" + component.id + ".rs", constant, component.id)
                    add(key, "rc/" + component.id + ".rc", source, component.id)
    for source, target in (("name", "display_name"), ("publisher", "publisher_display_name"),
                           ("description", "description")):
        add("product." + source, "authoring/msix-inputs.json", "/" + target)
    add("fields", "field-register.json", "/fields", representation="complete-internal-register")
    add("fields:project.*", "authoring/project-inputs.json", "/fields",
        representation="exact-owned-prefix-selection")
    add("fields:catalogue.B.6.*", "authoring/msix-inputs.json", "/fields",
        representation="exact-owned-prefix-selection")
    add("fields:catalogue.A.*,component.engine.scm.*,component.broker.scm.*",
        "authoring/service-inputs.json", "/fields", representation="exact-owned-prefix-selection")
    add("fields:catalogue.C.*", "authoring/event-inputs.json", "/fields",
        representation="exact-owned-prefix-selection")
    add("pending:diagnostics.provider", "authoring/event-inputs.json", "/pending",
        representation="pending-owner-without-invented-provider")
    add("fields", "field-coverage.json", "/", representation="dispositions-and-source-contract")
    public = ["metadata.json", "rust/product.rs"]
    for component in model.components:
        if component.emittable:
            public.extend(("rust/" + component.id + ".rs", "rc/" + component.id + ".rc"))
    modes = {"json": "exact-typed-whole-record", "rs": "exact-generated-constant-subset",
             "rc": "exact-prototype-versioninfo-subset", "xml": "exact-authoring-tree",
             "csv": "exact-review-rows"}
    artifacts = [{"artifact": artifact, "observation": modes[artifact.rsplit(".", 1)[1]]}
                 for artifact in sorted(public + list(_INTERNAL))]
    return {"schema_version": 1, "scope": "generated-source", "status": "required-source-contract",
            "binding_scope": "selected-facts-and-complete-artifact-records",
            "locator_convention": "semantic-members; collections keyed by admitted component id or service role",
            "obligation_count": len(obligations), "obligations": obligations,
            "artifact_contracts": artifacts,
            "public_artifacts": public, "internal_only_artifacts": list(_INTERNAL),
            "compiled_artifact_readback": "not-run", "native_installed_readback": "not-run",
            "package_privacy_readback": "not-run", "release_qualified": False}


def _rows(model: Metadata, prefixes: tuple[str, ...] = ()) -> list[dict[str, str]]:
    """Expected complete internal records from the admitted immutable fields."""
    result = []
    for field in model.fields:
        if prefixes and not field.key.startswith(prefixes):
            continue
        spec = FIELD_BY_KEY[field.key]
        result.append({"key": field.key, "value": field.value, "status": field.status,
                       "owner": field.owner, "source": field.source, "reason": field.reason,
                       "surface": spec.surface, "kind": spec.kind, "value_type": spec.value_type,
                       "applicability": spec.applicability, "derivation": spec.derivation,
                       "source_ref": spec.source_ref})
    return result


def _plain(value: object) -> object:
    """Map immutable dataclass tuples to their JSON observation representation."""
    if type(value) is dict:
        return {key: _plain(item) for key, item in value.items()}
    if type(value) in (tuple, list):
        return [_plain(item) for item in value]
    return value


def _expected_json(model: Metadata) -> dict[str, object]:
    """Independent semantic expectations, not serialized production output."""
    values = _values(model)
    rows = _rows(model)
    pending = [asdict(item) for item in model.pending]
    coverage = {"schema_version": 1, "scope": "declared-source-field-register",
                "complete_declared_field_set": True, "field_count": len(model.fields),
                "unresolved_field_count": sum(row.status == "pending" or
                    (row.status == "derived" and not row.value) for row in model.fields),
                "status_counts": dict(Counter(row.status for row in model.fields)),
                "surface_counts": dict(Counter(FIELD_BY_KEY[row.key].surface for row in model.fields)),
                "resolved_value_count": sum(bool(row.value) for row in model.fields),
                "pending": [{key: row[key] for key in ("key", "owner", "source", "reason")}
                            for row in rows if row["status"] == "pending"],
                "unobserved_derived": [{key: row[key] for key in ("key", "owner", "source", "reason")}
                                       for row in rows if row["status"] == "derived" and not row["value"]],
                "release_qualified": False, "native_readback": "not-run",
                "delivery": delivery_contract(model)}
    return {
        "metadata.json": {"schema_version": 1, "stage": model.stage, "target": model.target,
            "cpu_floor": model.cpu_floor, "product": asdict(model.product),
            "components": [_plain(asdict(item)) for item in model.components],
            "pending": pending, "inputs": [asdict(item) for item in model.inputs],
            "public_metadata": {key: values[key] for key, _ in _PUBLIC_FACTS if key in values},
            "qualification": {"scope": "source-authoring-projections", "native": "not-run",
                              "release": "not-qualified"}},
        "field-register.json": {"schema_version": 1, "scope": "internal-authoring", "fields": rows},
        "field-coverage.json": coverage,
        "authoring/project-inputs.json": {"schema_version": 1, "kind": "internal-project-authoring-inputs",
                                           "fields": _rows(model, ("project.",))},
        "authoring/msix-inputs.json": {"schema_version": 1, "kind": "partial-msix-authoring-inputs",
            "complete_manifest": False, "display_name": model.product.name,
            "publisher_display_name": model.product.publisher, "description": model.product.description,
            "identity": None, "target": model.target, "cpu_floor": model.cpu_floor,
            "pending": pending, "fields": _rows(model, ("catalogue.B.6.",))},
        "authoring/service-inputs.json": {"schema_version": 1, "kind": "partial-service-authoring-inputs",
            "services": [{"role": item.role, "state": item.state, "display_name": item.display_name,
                          "description": item.description,
                          "technical_name": values.get("component." + item.id + ".scm.name") or None}
                         for item in model.components if item.id in {"engine", "broker"}],
            "fields": _rows(model, ("catalogue.A.", "component.engine.scm.", "component.broker.scm."))},
        "authoring/event-inputs.json": {"schema_version": 1, "kind": "partial-event-authoring-inputs",
            "provider_name": None, "provider_guid": None, "event_schema_version": None,
            "message_resource": None,
            "pending": [asdict(item) for item in model.pending
                        if item.path.startswith(("diagnostics.", "event.", "provider."))],
            "fields": _rows(model, ("catalogue.C.",))},
    }


def _equal(actual: object, expected: object, depth: int = 0) -> bool:
    """Compare exact JSON types, identities and multiplicity, including bool/int."""
    if depth > 16 or type(actual) is not type(expected):
        return False
    if type(expected) is dict:
        return actual.keys() == expected.keys() and all(
            _equal(actual[key], value, depth + 1) for key, value in expected.items())
    if type(expected) is list:
        return len(actual) == len(expected) and all(
            _equal(a, b, depth + 1) for a, b in zip(actual, expected))
    return actual == expected


def _json(raw: bytes) -> object:
    """Reject ambiguous duplicate keys and nonfinite JSON constants."""
    def pairs(items: list[tuple[str, object]]) -> dict[str, object]:
        result = {}
        for key, value in items:
            if key in result:
                raise MetadataError("delivery-duplicate-json-key")
            result[key] = value
        return result

    def constant(_: str) -> None:
        raise MetadataError("delivery-nonfinite-json")

    return json.loads(raw.decode("utf-8"), object_pairs_hook=pairs, parse_constant=constant)


def _literal(text: str, *, resource: bool) -> str:
    """Read only the explicitly generated literal subset, never execute source."""
    result = []
    index = 0
    while index < len(text):
        if resource and text[index:index + 2] == '""':
            result.append('"'); index += 2
        elif text[index] == "\\":
            match = None if resource else re.match(r"\\u\{([0-9a-fA-F]{1,6})\}", text[index:])
            if match:
                value = int(match[1], 16)
                if value > 0x10FFFF or 0xD800 <= value <= 0xDFFF:
                    raise MetadataError("delivery-invalid-rust-scalar")
                result.append(chr(value)); index += len(match[0])
            else:
                index += 1
                escapes = {"\\": "\\", "n": "\n", "r": "\r", "t": "\t", "0": "\0"}
                if not resource:
                    escapes['"'] = '"'
                if index >= len(text) or text[index] not in escapes:
                    raise MetadataError("delivery-unknown-literal-escape")
                result.append(escapes[text[index]]); index += 1
        else:
            if text[index] == '"' or ord(text[index]) < 0x20:
                raise MetadataError("delivery-unparsed-literal-character")
            result.append(text[index]); index += 1
    value = "".join(result)
    if resource:
        if not value.endswith("\0") or "\0" in value[:-1]:
            raise MetadataError("delivery-resource-termination")
        value = value[:-1]
    return value


def _rust(raw: bytes) -> dict[str, object]:
    """Read generated constant declarations and refuse every other source form."""
    result: dict[str, object] = {}
    lines = raw.decode("ascii").splitlines()
    if (lines[:2] != ["// Generated by the NevroTick metadata pipeline; do not edit.",
                      "// Source projection; native/package qualification remains separate."]
            or len(lines) % 2):
        raise MetadataError("delivery-unparsed-rust-preamble")
    for index in range(2, len(lines), 2):
        comment, line = lines[index:index + 2]
        string = re.fullmatch(r'pub const ([A-Z_]+): &str = "((?:[^"\\]|\\.)*)";', line)
        numbers = re.fullmatch(r"pub const ([A-Z_]+): \[u16; 4\] = \[([0-9]+), ([0-9]+), ([0-9]+), ([0-9]+)\];", line)
        if string:
            if comment != f"/// Reviewed metadata projection for `{string[1]}`.":
                raise MetadataError("delivery-unparsed-rust-comment")
            key, value = string[1], _literal(string[2], resource=False)
        elif numbers:
            if comment != "/// Checked component version mapped to four Windows words.":
                raise MetadataError("delivery-unparsed-rust-comment")
            key, value = numbers[1], [int(item) for item in numbers.groups()[1:]]
            if any(item > 65535 for item in value):
                raise MetadataError("delivery-version-word-overflow")
        else:
            raise MetadataError("delivery-unparsed-rust-source")
        if key in result:
            raise MetadataError("delivery-duplicate-rust-constant")
        result[key] = value
    return result


def _rc(raw: bytes) -> dict[str, object]:
    """Read this prototype's VERSIONINFO subset; this is not an RC compiler."""
    strings: dict[str, str] = {}
    result: dict[str, object] = {"strings": strings}
    structure = []
    lines = raw.decode("utf-8").splitlines()
    if lines[:2] != ["// Generated; source prototype resource candidate, not a native pass.",
                     "// ProductVersion uses the component prototype mapping; release mapping is held."]:
        raise MetadataError("delivery-unparsed-resource-preamble")
    for original in lines[2:]:
        line = original.strip()
        if not line:
            continue
        string = re.fullmatch(r'VALUE "([A-Za-z]+)", "((?:[^"\\]|\\.|"")*)"', line)
        version = re.fullmatch(r"(FILEVERSION|PRODUCTVERSION) ([0-9]+),([0-9]+),([0-9]+),([0-9]+)", line)
        if string:
            if string[1] in strings:
                raise MetadataError("delivery-duplicate-resource-string")
            strings[string[1]] = _literal(string[2], resource=True)
            structure.append("STRING")
        elif version:
            if version[1] in result:
                raise MetadataError("delivery-duplicate-resource-version")
            result[version[1]] = [int(item) for item in version.groups()[1:]]
            structure.append(version[1])
        else:
            structure.append(line)
    required_structure = ["#pragma code_page(65001)", "LANGUAGE 0x9, 0x1", "1 VERSIONINFO",
        "FILEVERSION", "PRODUCTVERSION", "FILEFLAGSMASK 0x3fL", "FILEFLAGS 0x2L",
        "FILEOS 0x40004L", "FILETYPE 0x1L", "FILESUBTYPE 0x0L", "BEGIN",
        'BLOCK "StringFileInfo"', "BEGIN", 'BLOCK "040904B0"', "BEGIN",
        *(["STRING"] * len(strings)), "END", "END", 'BLOCK "VarFileInfo"', "BEGIN",
        'VALUE "Translation", 0x0409, 1200', "END", "END"]
    if structure != required_structure:
        raise MetadataError("delivery-unparsed-resource-source")
    return result


def _xml(raw: bytes) -> dict[str, object]:
    """Read the declared UTF-8 byte form, rejecting unclaimed XML constructs."""
    # This reader owns the generated XML 1.0/UTF-8 subset. Decoding to a str
    # before parsing would make ElementTree ignore a conflicting declaration.
    declaration, newline, body = raw.partition(b"\n")
    if (not newline or re.fullmatch(
            rb"<\?xml version=(['\"])1[.]0\1 encoding=(['\"])(?:utf-8|UTF-8)\2\?>",
            declaration.rstrip(b"\r")) is None):
        raise MetadataError("delivery-xml-encoding-declaration")
    # Enforce strict UTF-8 without replacing malformed bytes.
    raw.decode("utf-8")
    if any(token in body for token in (b"<!DOCTYPE", b"<!ENTITY", b"<!--", b"<?")):
        raise MetadataError("delivery-xml-declaration")
    # The parser must still see the bytes and their encoding declaration.
    root = ET.fromstring(raw)
    namespace = "{" + _NAMESPACE + "}"
    if root.tag != namespace + "metadata" or set(root.attrib) != {"schema-version", "stage"}:
        raise MetadataError("delivery-xml-root")
    if [child.tag for child in root] != [namespace + "product", namespace + "components"]:
        raise MetadataError("delivery-xml-children")
    product, components = root
    if any(item.text and item.text.strip() for item in (root, product, components)):
        raise MetadataError("delivery-xml-unparsed-text")
    if any(item.tail and item.tail.strip() for item in root.iter()):
        raise MetadataError("delivery-xml-unparsed-tail")
    if product.attrib or components.attrib:
        raise MetadataError("delivery-xml-attributes")
    if [item.tag for item in product] != [namespace + key for key in ("name", "publisher", "description", "repository")]:
        raise MetadataError("delivery-xml-product-fields")
    product_values = {}
    for child in product:
        if child.attrib or len(child):
            raise MetadataError("delivery-xml-leaf")
        product_values[child.tag[len(namespace):]] = child.text or ""
    component_values = []
    for child in components:
        if (child.tag != namespace + "component" or set(child.attrib) != {"id", "role", "state"}
                or [item.tag for item in child] != [namespace + "name", namespace + "description"]
                or child.text and child.text.strip()
                or any(item.attrib or len(item) for item in child)):
            raise MetadataError("delivery-xml-component")
        component_values.append({**child.attrib, "name": child[0].text or "",
                                 "description": child[1].text or ""})
    return {"schema-version": root.attrib["schema-version"], "stage": root.attrib["stage"],
            "product": product_values, "components": component_values}


def _csv(raw: bytes) -> list[list[str]]:
    """Preserve row identity/cardinality, including multiline quoted cells."""
    rows = list(csv.reader(io.StringIO(raw.decode("utf-8"), newline=""), strict=True))
    if (not rows or rows[0] != ["field", "value", "status"]
            or any(len(row) != 3 for row in rows)
            or len({row[0] for row in rows[1:]}) != len(rows) - 1):
        raise MetadataError("delivery-csv-shape-or-identity")
    return rows


def verify_delivery(model: Metadata, outputs: dict[str, bytes]) -> None:
    """Enforce exact interpreted source facts and explicit exposure boundaries.

    The input set contains payloads, excluding the external receipt. Bounds and
    exact plain types are checked before any parser. Failed observations raise a
    redacted MetadataError; no output is repaired or published here.
    """
    validate_model(model)
    if (type(outputs) is not dict or len(outputs) > 32
            or any(type(name) is not str or type(raw) is not bytes or len(raw) > _MAX_FILE
                   for name, raw in outputs.items())
            or sum(len(raw) for raw in outputs.values()) > _MAX_TOTAL):
        raise MetadataError("delivery-output-budget-or-type")
    expected_json = _expected_json(model)
    expected_paths = set(expected_json) | {"rust/product.rs", "authoring/display.xml", "authoring/store.csv"}
    for component in model.components:
        if component.emittable:
            expected_paths.update(("rust/" + component.id + ".rs", "rc/" + component.id + ".rc"))
    if set(outputs) != expected_paths:
        raise MetadataError("delivery-output-set")
    values = _values(model)
    try:
        for name, expected in expected_json.items():
            if not _equal(_json(outputs[name]), expected):
                raise MetadataError("delivery-json-facts")
        expected_rust = {"PRODUCT_NAME": model.product.name, "PUBLISHER": model.product.publisher,
            "PRODUCT_DESCRIPTION": model.product.description, "REPOSITORY": model.product.repository,
            "TARGET": model.target, "CPU_FLOOR": model.cpu_floor,
            **{constant: values[key] for key, constant in _PUBLIC_FACTS if key in values}}
        if not _equal(_rust(outputs["rust/product.rs"]), expected_rust):
            raise MetadataError("delivery-shared-constants")
        for component in model.components:
            if not component.emittable:
                continue
            filename = component.binary_name if component.binary_name.endswith(".exe") else component.binary_name + ".exe"
            expected_rust = {"COMPONENT_ID": component.id, "ROLE": component.role,
                "DISPLAY_NAME": component.display_name, "DESCRIPTION": component.description,
                "VERSION": component.version, "LICENSE": component.license,
                "ORIGINAL_FILENAME": filename, "FILE_VERSION": list(component.file_version)}
            expected_strings = {"CompanyName": model.product.publisher, "FileDescription": component.description,
                "FileVersion": component.version, "InternalName": component.binary_name,
                "OriginalFilename": filename, "ProductName": model.product.name, "ProductVersion": component.version}
            for field, constant in (("LegalCopyright", "COPYRIGHT"),
                                     ("LegalTrademarks", "TRADEMARKS"), ("Comments", "COMMENTS")):
                value = values.get("component." + component.id + ".pe." + field)
                if value:
                    expected_rust[constant] = value
                    expected_strings[field] = value
            if not _equal(_rust(outputs["rust/" + component.id + ".rs"]), expected_rust):
                raise MetadataError("delivery-component-constants")
            if not _equal(_rc(outputs["rc/" + component.id + ".rc"]),
                    {"strings": expected_strings, "FILEVERSION": list(component.file_version),
                     "PRODUCTVERSION": list(component.file_version)}):
                raise MetadataError("delivery-component-resource")
        expected_xml = {"schema-version": "1", "stage": model.stage,
            "product": {field: getattr(model.product, field) for field in ("name", "publisher", "description", "repository")},
            "components": [{"id": item.id, "role": item.role, "state": item.state,
                            "name": item.display_name, "description": item.description}
                           for item in model.components]}
        if not _equal(_xml(outputs["authoring/display.xml"]), expected_xml):
            raise MetadataError("delivery-display-xml")
        expected_csv = [["field", "value", "status"]]
        for key in ("name", "publisher", "description", "repository", "homepage"):
            value = getattr(model.product, key)
            if value is not None:
                expected_csv.append(["product." + key, value, "derived"])
        expected_csv.extend([item.path, "", "pending"] for item in model.pending)
        for key in ("project.release.notes", "project.urls.support", "project.urls.privacy",
                    "project.urls.license", "project.urls.release_notes"):
            if key in values:
                expected_csv.append([key, values[key], "derived"])
        expected_csv.extend([row.key, row.value, row.status] for row in model.fields
            if row.key.startswith(("catalogue.B.9.1.", "catalogue.B.9.2.")) and row.value
            and row.status in {"populated", "derived"})
        if not _equal(_csv(outputs["authoring/store.csv"]), expected_csv):
            raise MetadataError("delivery-store-csv")
    except (UnicodeError, ValueError, TypeError, RecursionError, ET.ParseError, csv.Error) as error:
        raise MetadataError("delivery-invalid-source-projection") from error
