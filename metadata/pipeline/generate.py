"""Deterministic, bounded source metadata projections and read-only comparison.

The source loader owns admission. This module rechecks immutable models at its
direct entry, renders only code-owned destinations and publishes a fresh tree
under an owned parent. It does not compile resources, sign, register services or
authenticate its own input. Enforced isolation of dependency execution belongs
to the caller's build environment, not these filesystem checks.
"""

from __future__ import annotations

import csv
from collections import Counter
from dataclasses import asdict
import hashlib
import io
import json
import os
from pathlib import Path
import secrets
import shutil
import stat
import xml.etree.ElementTree as ET

from .contract import Component, Metadata, MetadataError, load_metadata, validate_model
from .delivery import delivery_contract, verify_delivery
from .fields import FIELD_BY_KEY

MAX_OUTPUT_FILE = 1024 * 1024
MAX_OUTPUT_TOTAL = 4 * MAX_OUTPUT_FILE
MAX_OUTPUT_FILES = 64
XML_NAMESPACE = "urn:nevrotick:metadata:authoring:1"
CODE_FILES = ("__init__.py", "pipeline/__init__.py", "pipeline/__main__.py",
              "pipeline/contract.py", "pipeline/fields.py", "pipeline/text_policy.py",
              "pipeline/delivery.py", "pipeline/generate.py")

# Only these reviewed descriptive facts enter shared public application data.
# Complete fields, private contacts and operational configuration remain in the
# internal authoring register; a catalogue entry never grants effect authority.
PUBLIC_STRINGS = (
    ("project.author.name", "AUTHOR_NAME"),
    ("project.author.role", "AUTHOR_ROLE"),
    ("project.author.website", "AUTHOR_WEBSITE"),
    ("project.author.isni", "AUTHOR_ISNI"),
    ("project.author.orcid", "AUTHOR_ORCID"),
    ("project.author.viaf", "AUTHOR_VIAF"),
    ("project.author.wikidata", "AUTHOR_WIKIDATA"),
    ("project.urls.homepage", "HOMEPAGE"),
    ("project.urls.documentation", "DOCUMENTATION_URL"),
    ("project.urls.support", "SUPPORT_URL"),
    ("project.urls.privacy", "PRIVACY_URL"),
    ("project.urls.license", "LICENSE_URL"),
    ("project.urls.release_notes", "RELEASE_NOTES_URL"),
    ("project.rights.copyright", "COPYRIGHT"),
    ("project.rights.trademarks", "TRADEMARKS"),
    ("project.localization.adopted_locales", "ADOPTED_LOCALES"),
    ("project.localization.fallback_locale", "FALLBACK_LOCALE"),
    ("project.release.channel", "RELEASE_CHANNEL"),
)


class PublicationUncertainError(MetadataError):
    """Rename completed, but the final containing-directory flush was not acknowledged.

    The committed tree is retained. This is neither a precommit failure nor
    permission to overwrite/retry; a read-only check can reconcile visible bytes
    but cannot retroactively confirm the failed durability acknowledgement.
    """

    def __init__(self) -> None:
        super().__init__("metadata-publication-durability-unconfirmed")


def _field_value(model: Metadata, key: str) -> str:
    """Return only a populated/derived value; unresolved empty text stays absent."""
    for field in model.fields:
        if field.key == key:
            return field.value if field.status in {"populated", "derived"} else ""
    return ""


def _field_rows(model: Metadata, prefixes: tuple[str, ...] = ()) -> list[dict[str, str]]:
    """Project internal register rows with their code-owned interpretation."""
    rows = []
    for field in model.fields:
        if prefixes and not field.key.startswith(prefixes):
            continue
        spec = FIELD_BY_KEY[field.key]
        rows.append({**asdict(field), "surface": spec.surface, "kind": spec.kind,
                     "value_type": spec.value_type, "applicability": spec.applicability,
                     "derivation": spec.derivation, "source_ref": spec.source_ref})
    return rows


def field_counts(model: Metadata) -> dict[str, int]:
    """Count fields only after enforcing the public model contract.

    Unresolved means pending or derived with no observation/value yet. It is
    separate from native/release qualification and excludes justified absence.
    """
    validate_model(model)
    return {"field_count": len(model.fields),
            "unresolved_field_count": sum(field.status == "pending" or
                                           (field.status == "derived" and not field.value)
                                           for field in model.fields)}


def _field_coverage(model: Metadata) -> dict[str, object]:
    """Report complete declared-field disposition separately from resolved values."""
    rows = _field_rows(model)
    pending = [{key: row[key] for key in ("key", "owner", "source", "reason")}
               for row in rows if row["status"] == "pending"]
    return {"schema_version": 1, "scope": "declared-source-field-register",
            "complete_declared_field_set": True,
            **field_counts(model),
            "status_counts": dict(sorted(Counter(row["status"] for row in rows).items())),
            "surface_counts": dict(sorted(Counter(row["surface"] for row in rows).items())),
            "resolved_value_count": sum(bool(row["value"]) for row in rows),
            "pending": pending,
            "unobserved_derived": [{key: row[key] for key in ("key", "owner", "source", "reason")}
                                   for row in rows if row["status"] == "derived" and not row["value"]],
            "release_qualified": False,
            "native_readback": "not-run",
            "delivery": delivery_contract(model)}


def _json(value: object) -> bytes:
    """Encode finite typed values in stable UTF-8 JSON without implicit coercion."""
    return (json.dumps(value, ensure_ascii=False, allow_nan=False, sort_keys=True,
                       indent=2) + "\n").encode("utf-8")


def _rust_string(value: str) -> str:
    """Quote text as an ASCII Rust string; JSON Unicode escapes are invalid Rust."""
    result = ['"']
    for character in value:
        code = ord(character)
        if character in ('"', "\\"):
            result.append("\\" + character)
        elif 0x20 <= code <= 0x7E:
            result.append(character)
        else:
            result.append("\\u{" + format(code, "x") + "}")
    return "".join(result) + '"'


def _rust_constants(values: tuple[tuple[str, str], ...],
                    file_version: tuple[int, int, int, int] | None = None) -> bytes:
    """Render only fixed identifiers and quoted data, never metadata as Rust code."""
    lines = ["// Generated by the NevroTick metadata pipeline; do not edit.",
             "// Source projection; native/package qualification remains separate."]
    for name, value in values:
        lines.extend((f"/// Reviewed metadata projection for `{name}`.",
                      f"pub const {name}: &str = {_rust_string(value)};"))
    if file_version is not None:
        parts = ", ".join(str(part) for part in file_version)
        lines.extend(("/// Checked component version mapped to four Windows words.",
                      f"pub const FILE_VERSION: [u16; 4] = [{parts}];"))
    return ("\n".join(lines) + "\n").encode("ascii")


def _rc_string(value: str) -> str:
    """Quote a terminated VERSIONINFO string with resource-compiler quote rules.

    Microsoft documents doubled embedded quotes for resource strings; its
    VERSIONINFO examples explicitly terminate string values with ``\\0``.
    Source text cannot supply NUL through the validated contract.
    """
    return '"' + value.replace("\\", "\\\\").replace('"', '""').replace(
        "\r", "\\r").replace("\n", "\\n").replace("\t", "\\t") + '\\0"'


def _filename(binary_name: str) -> str:
    """Project the admitted Windows binary name without accepting a path."""
    return binary_name if binary_name.endswith(".exe") else binary_name + ".exe"


def _version_resource(model: Metadata, component: Component) -> bytes:
    """Produce a prerelease user-mode EXE resource candidate, not an emitted PE."""
    version = ",".join(str(part) for part in component.file_version)
    strings = [("CompanyName", model.product.publisher),
               ("FileDescription", component.description),
               ("FileVersion", component.version),
               ("InternalName", component.binary_name),
               ("OriginalFilename", _filename(component.binary_name)),
               ("ProductName", model.product.name),
               ("ProductVersion", component.version)]
    for name in ("LegalCopyright", "LegalTrademarks", "Comments"):
        value = _field_value(model, f"component.{component.id}.pe.{name}")
        if value:
            strings.append((name, value))
    for name in ("PrivateBuild", "SpecialBuild"):
        if _field_value(model, f"component.{component.id}.pe.{name}"):
            # Text cannot select a build classification or its native flags.
            raise MetadataError("unselected-resource-build-classification")
    lines = ["// Generated; source prototype resource candidate, not a native pass.",
             "// ProductVersion uses the component prototype mapping; release mapping is held.",
             "#pragma code_page(65001)", "LANGUAGE 0x9, 0x1", "1 VERSIONINFO", f"FILEVERSION {version}",
             f"PRODUCTVERSION {version}", "FILEFLAGSMASK 0x3fL", "FILEFLAGS 0x2L",
             "FILEOS 0x40004L", "FILETYPE 0x1L", "FILESUBTYPE 0x0L", "BEGIN",
             '  BLOCK "StringFileInfo"', "  BEGIN", '    BLOCK "040904B0"', "    BEGIN"]
    for key, value in strings:
        lines.append(f'      VALUE "{key}", {_rc_string(value)}')
    lines.extend(("    END", "  END", '  BLOCK "VarFileInfo"', "  BEGIN",
                  '    VALUE "Translation", 0x0409, 1200', "  END", "END"))
    return ("\n".join(lines) + "\n").encode("utf-8")


def _display_xml(model: Metadata) -> bytes:
    """Encode a custom authoring projection; this is not an MSIX/event manifest."""
    ET.register_namespace("", XML_NAMESPACE)
    root = ET.Element("{" + XML_NAMESPACE + "}metadata",
                      {"schema-version": "1", "stage": model.stage})
    product = ET.SubElement(root, "product")
    for key in ("name", "publisher", "description", "repository"):
        ET.SubElement(product, key).text = getattr(model.product, key)
    components = ET.SubElement(root, "components")
    for component in model.components:
        element = ET.SubElement(components, "component", {
            "id": component.id, "role": component.role, "state": component.state})
        ET.SubElement(element, "name").text = component.display_name
        ET.SubElement(element, "description").text = component.description
    ET.indent(root, space="  ")
    return ET.tostring(root, encoding="utf-8", xml_declaration=True) + b"\n"


def _store_csv(model: Metadata) -> bytes:
    """Export review fields; reject formula-leading cells without changing values."""
    stream = io.StringIO(newline="")
    writer = csv.writer(stream, lineterminator="\n", quoting=csv.QUOTE_ALL)
    writer.writerow(("field", "value", "status"))
    for key in ("name", "publisher", "description", "repository", "homepage"):
        value = getattr(model.product, key)
        if value is not None:
            if value.lstrip().startswith(("=", "+", "-", "@")):
                raise MetadataError("store-export-formula-prefix")
            writer.writerow(("product." + key, value, "derived"))
    for field in model.pending:
        writer.writerow((field.path, "", "pending"))
    for key in ("project.release.notes", "project.urls.support", "project.urls.privacy",
                "project.urls.license", "project.urls.release_notes"):
        value = _field_value(model, key)
        if value:
            if value.lstrip().startswith(("=", "+", "-", "@")):
                raise MetadataError("store-export-formula-prefix")
            writer.writerow((key, value, "derived"))
    # The catalogue register contains the actual per-listing field dispositions;
    # this remains a custom review CSV, not the Partner Center upload schema.
    for field in model.fields:
        if field.key.startswith(("catalogue.B.9.1.", "catalogue.B.9.2.")):
            if field.value and field.status in {"populated", "derived"}:
                if field.value.lstrip().startswith(("=", "+", "-", "@")):
                    raise MetadataError("store-export-formula-prefix")
                writer.writerow((field.key, field.value, field.status))
    return stream.getvalue().encode("utf-8")


def build_outputs(model: Metadata) -> dict[str, bytes]:
    """Revalidate a model and return a bounded fixed-name projection set in memory."""
    validate_model(model)
    public_values = {key: _field_value(model, key) for key, _ in PUBLIC_STRINGS
                     if _field_value(model, key)}
    descriptor = {
        "schema_version": 1, "stage": model.stage, "target": model.target,
        "cpu_floor": model.cpu_floor, "product": asdict(model.product),
        "components": [asdict(component) for component in model.components],
        "pending": [asdict(field) for field in model.pending],
        "inputs": [asdict(item) for item in model.inputs],
        "public_metadata": public_values,
        "qualification": {"scope": "source-authoring-projections", "native": "not-run",
                          "release": "not-qualified"},
    }
    shared_constants = (
        ("PRODUCT_NAME", model.product.name), ("PUBLISHER", model.product.publisher),
        ("PRODUCT_DESCRIPTION", model.product.description),
        ("REPOSITORY", model.product.repository), ("TARGET", model.target),
        ("CPU_FLOOR", model.cpu_floor),
        *((name, public_values[key]) for key, name in PUBLIC_STRINGS if key in public_values),
    )
    outputs = {"metadata.json": _json(descriptor),
                "rust/product.rs": _rust_constants(shared_constants),
                "field-register.json": _json({"schema_version": 1, "scope": "internal-authoring",
                                               "fields": _field_rows(model)}),
                "field-coverage.json": _json(_field_coverage(model)),
               "authoring/display.xml": _display_xml(model),
                "authoring/store.csv": _store_csv(model),
                "authoring/project-inputs.json": _json({
                    "schema_version": 1, "kind": "internal-project-authoring-inputs",
                    "fields": _field_rows(model, ("project.",))})}
    for component in model.components:
        if component.emittable:
            outputs[f"rust/{component.id}.rs"] = _rust_constants((
                ("COMPONENT_ID", component.id), ("ROLE", component.role),
                ("DISPLAY_NAME", component.display_name), ("DESCRIPTION", component.description),
                ("VERSION", component.version), ("LICENSE", component.license),
                ("ORIGINAL_FILENAME", _filename(component.binary_name)),
                *((name, value) for name, value in (
                    ("COPYRIGHT", _field_value(model, f"component.{component.id}.pe.LegalCopyright")),
                    ("TRADEMARKS", _field_value(model, f"component.{component.id}.pe.LegalTrademarks")),
                    ("COMMENTS", _field_value(model, f"component.{component.id}.pe.Comments")),
                ) if value)), component.file_version)
            outputs[f"rc/{component.id}.rc"] = _version_resource(model, component)
    outputs["authoring/msix-inputs.json"] = _json({
        "schema_version": 1, "kind": "partial-msix-authoring-inputs", "complete_manifest": False,
        "display_name": model.product.name, "publisher_display_name": model.product.publisher,
        "description": model.product.description, "identity": None,
        "target": model.target, "cpu_floor": model.cpu_floor,
        "pending": [asdict(item) for item in model.pending],
        "fields": _field_rows(model, ("catalogue.B.6.",))})
    outputs["authoring/service-inputs.json"] = _json({
        "schema_version": 1, "kind": "partial-service-authoring-inputs",
        "services": [{"role": item.role, "state": item.state,
                      "display_name": item.display_name, "description": item.description,
                      "technical_name": _field_value(model, f"component.{item.id}.scm.name") or None}
                     for item in model.components if item.id in ("engine", "broker")],
        "fields": _field_rows(model, ("catalogue.A.", "component.engine.scm.", "component.broker.scm."))})
    outputs["authoring/event-inputs.json"] = _json({
        "schema_version": 1, "kind": "partial-event-authoring-inputs", "provider_name": None,
        "provider_guid": None, "event_schema_version": None, "message_resource": None,
        "pending": [asdict(item) for item in model.pending
                    if item.path.startswith(("diagnostics.", "event.", "provider."))],
        "fields": _field_rows(model, ("catalogue.C.",))})
    if len(outputs) >= MAX_OUTPUT_FILES or sum(map(len, outputs.values())) > MAX_OUTPUT_TOTAL:
        raise MetadataError("output-budget")
    if any(len(value) > MAX_OUTPUT_FILE for value in outputs.values()):
        raise MetadataError("output-budget")
    verify_delivery(model, outputs)
    return outputs


def _absolute(path: Path) -> Path:
    """Make a lexical absolute path without resolving away symlink evidence."""
    if ".." in path.parts:
        raise MetadataError("path-traversal")
    return path.absolute()


def _open_directory(path: Path, *, private: bool = False) -> int:
    """Walk POSIX directory descriptors without following any ancestor symlink."""
    if not hasattr(os, "O_NOFOLLOW") or os.open not in os.supports_dir_fd:
        raise MetadataError("filesystem-custody-unavailable")
    path = _absolute(path)
    flags = os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_CLOEXEC
    descriptor = os.open(path.anchor, flags)
    try:
        for part in path.parts[1:]:
            child = os.open(part, flags, dir_fd=descriptor)
            os.close(descriptor)
            descriptor = child
        observed = os.fstat(descriptor)
        if private and (observed.st_uid != os.geteuid() or observed.st_mode & 0o022):
            raise MetadataError("output-parent-not-owned")
        return descriptor
    except BaseException:
        os.close(descriptor)
        raise


def _read_regular(directory: int, name: str, maximum: int) -> bytes:
    """Read one bounded regular single-link file and reject replacement/growth."""
    flags = os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK | os.O_CLOEXEC
    descriptor = os.open(name, flags, dir_fd=directory)
    try:
        before = os.fstat(descriptor)
        if not stat.S_ISREG(before.st_mode) or before.st_nlink != 1 or before.st_size > maximum:
            raise MetadataError("output-file-custody")
        value = bytearray()
        while len(value) <= maximum:
            block = os.read(descriptor, min(65536, maximum + 1 - len(value)))
            if not block:
                break
            value.extend(block)
        after = os.fstat(descriptor)
        identity = lambda item: (item.st_dev, item.st_ino, item.st_size, item.st_mtime_ns,
                                 item.st_ctime_ns, item.st_nlink)
        current = os.stat(name, dir_fd=directory, follow_symlinks=False)
        if len(value) > maximum or len(value) != before.st_size or identity(before) != identity(after):
            raise MetadataError("output-file-changed")
        if identity(after) != identity(current):
            raise MetadataError("output-file-changed")
        return bytes(value)
    finally:
        os.close(descriptor)


def _code_identity() -> list[dict[str, str]]:
    """Observe fixed pipeline source bytes; hashes are custody data, not signatures."""
    base = Path(__file__).absolute().parent.parent
    result = []
    for relative in CODE_FILES:
        path = Path(relative)
        directory = _open_directory(base / path.parent)
        try:
            result.append({"path": "metadata/" + relative,
                           "sha256": hashlib.sha256(_read_regular(
                               directory, path.name, MAX_OUTPUT_FILE)).hexdigest()})
        finally:
            os.close(directory)
    return result


def _expected(model: Metadata) -> dict[str, bytes]:
    """Bind the generated payload to source/schema identities without a self-hash."""
    outputs = build_outputs(model)
    receipt = {"schema_version": 1, "stage": model.stage,
               "inputs": [asdict(item) for item in model.inputs], "tools": _code_identity(),
               "files": [{"path": name, "bytes": len(data),
                          "sha256": hashlib.sha256(data).hexdigest()}
                         for name, data in sorted(outputs.items())]}
    outputs["receipt.json"] = _json(receipt)
    return outputs


def _outside_source(root: Path, output: Path) -> Path:
    """Forbid generated output from entering or replacing the source checkout."""
    root, output = _absolute(root), _absolute(output)
    if output.is_relative_to(root) or root.is_relative_to(output):
        raise MetadataError("output-overlaps-source")
    if output.name in ("", ".", ".."):
        raise MetadataError("output-location")
    return output


def _directory_identity(descriptor: int) -> tuple[int, int]:
    """Identify a held directory without invalidation from our own child writes."""
    observed = os.fstat(descriptor)
    return observed.st_dev, observed.st_ino


def _parent_unchanged(path: Path, descriptor: int) -> None:
    """Bind a held directory to its current path before a path-facing result.

    The caller's owned isolation remains necessary after this observation; this
    check cannot prevent a competing same-identity mutation after it returns.
    """
    current = _open_directory(path)
    try:
        if _directory_identity(current) != _directory_identity(descriptor):
            raise MetadataError("output-parent-changed")
    finally:
        os.close(current)


def _new_staging_directory(parent: int) -> tuple[str, int]:
    """Create a fresh private sibling through the already admitted parent FD."""
    for _ in range(8):
        name = ".nevrotick-metadata-" + secrets.token_hex(16)
        try:
            os.mkdir(name, 0o700, dir_fd=parent)
        except FileExistsError:
            continue
        try:
            descriptor = os.open(name, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_CLOEXEC,
                                 dir_fd=parent)
        except BaseException:
            # Creation has transferred custody even if acquiring the child FD
            # fails. No caller has received the name yet, so release it here.
            os.rmdir(name, dir_fd=parent)
            raise
        return name, descriptor
    raise MetadataError("output-staging-collision-budget")


def _write_payload(staging: int, relative: str, data: bytes) -> None:
    """Create a fixed output beneath held directory FDs, never reopened paths."""
    directory = os.dup(staging)
    try:
        path = Path(relative)
        if path.is_absolute() or ".." in path.parts or len(path.parts) > 2:
            raise MetadataError("output-name-contract")
        for part in path.parts[:-1]:
            try:
                os.mkdir(part, 0o700, dir_fd=directory)
            except FileExistsError:
                pass
            child = os.open(part, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_CLOEXEC,
                            dir_fd=directory)
            os.close(directory)
            directory = child
        destination = os.open(path.name, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW
                              | os.O_CLOEXEC, 0o600, dir_fd=directory)
        try:
            offset = 0
            while offset < len(data):
                written = os.write(destination, data[offset:])
                if written <= 0:
                    raise MetadataError("output-write-incomplete")
                offset += written
            os.fsync(destination)
        finally:
            os.close(destination)
        # File flushes alone do not acknowledge the entries in reached child
        # directories. Flush this held container before its parent is published.
        os.fsync(directory)
    finally:
        os.close(directory)


def _remove_staging(parent: int, name: str, staging: int) -> None:
    """Remove only our still-named staging directory using safe descriptor custody."""
    observed = os.stat(name, dir_fd=parent, follow_symlinks=False)
    if (not stat.S_ISDIR(observed.st_mode)
            or (observed.st_dev, observed.st_ino) != _directory_identity(staging)):
        raise MetadataError("output-cleanup-custody-lost")
    if not shutil.rmtree.avoids_symlink_attacks:
        raise MetadataError("output-cleanup-custody-unavailable")
    shutil.rmtree(name, dir_fd=parent)


def generate(root: Path, output: Path, source: Path | None = None, *,
             stage: str = "source") -> dict[str, object]:
    """Publish a fresh complete tree; reject existing output and preserve it on failure.

    The parent must be owned and unwritable by other users. Protecting it from a
    malicious process with the same identity requires the build sandbox. Rename
    publication is atomic within that owned-parent prerequisite; no updater or
    overwrite operation is supplied here. Precommit failures release staging;
    a failed final parent flush after rename raises PublicationUncertainError
    and preserves the committed complete tree for explicit reconciliation.
    """
    model = load_metadata(root, source, stage=stage)
    outputs = _expected(model)
    output = _outside_source(root, output)
    parent = _open_directory(output.parent, private=True)
    temporary: str | None = None
    staging: int | None = None
    try:
        try:
            os.stat(output.name, dir_fd=parent, follow_symlinks=False)
        except FileNotFoundError:
            pass
        else:
            raise MetadataError("output-already-exists")
        temporary, staging = _new_staging_directory(parent)
        for name, data in sorted(outputs.items()):
            _write_payload(staging, name, data)
        if load_metadata(root, source, stage=stage) != model:
            raise MetadataError("source-changed-during-generation")
        if _code_identity() != json.loads(outputs["receipt.json"])["tools"]:
            raise MetadataError("generator-changed-during-generation")
        # An existing tree is never an update target. The private parent plus
        # caller-enforced isolation is required across this final publication.
        try:
            os.stat(output.name, dir_fd=parent, follow_symlinks=False)
        except FileNotFoundError:
            pass
        else:
            raise MetadataError("output-already-exists")
        _parent_unchanged(output.parent, parent)
        os.fsync(staging)
        os.rename(temporary, output.name, src_dir_fd=parent, dst_dir_fd=parent)
        temporary = None
        try:
            os.fsync(parent)
        except OSError as error:
            raise PublicationUncertainError() from error
        return {"status": "generated-" + stage, "file_count": len(outputs),
                "pending_count": len(model.pending), **field_counts(model)}
    finally:
        try:
            if temporary is not None and staging is not None:
                _remove_staging(parent, temporary, staging)
        finally:
            if staging is not None:
                os.close(staging)
            os.close(parent)


def check(root: Path, output: Path, source: Path | None = None, *,
          stage: str = "source") -> dict[str, object]:
    """Read a path-bound frozen candidate and recheck source/code/root identity.

    This is observational: it neither repairs bytes nor establishes durability,
    native interpretation or stability after the caller releases its isolation.
    """
    model = load_metadata(root, source, stage=stage)
    expected = _expected(model)
    output = _outside_source(root, output)
    root_fd = _open_directory(output)
    seen: set[str] = set()
    observed_outputs: dict[str, bytes] = {}
    total = 0

    def visit(directory: int, prefix: str = "", depth: int = 0) -> None:
        """Traverse the bounded fixed output tree through no-follow descriptors."""
        nonlocal total
        if depth > 2:
            raise MetadataError("output-tree-budget")
        names = []
        with os.scandir(directory) as entries:
            for entry in entries:
                if len(names) >= MAX_OUTPUT_FILES:
                    raise MetadataError("output-tree-budget")
                names.append(entry.name)
        for name in names:
            relative = prefix + name
            observed = os.stat(name, dir_fd=directory, follow_symlinks=False)
            if stat.S_ISDIR(observed.st_mode):
                if not any(path.startswith(relative + "/") for path in expected):
                    raise MetadataError("unexpected-output-directory")
                child = os.open(name, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_CLOEXEC,
                                dir_fd=directory)
                try:
                    visit(child, relative + "/", depth + 1)
                finally:
                    os.close(child)
            else:
                if relative not in expected or len(seen) >= MAX_OUTPUT_FILES:
                    raise MetadataError("unexpected-output-file")
                data = _read_regular(directory, name, MAX_OUTPUT_FILE)
                total += len(data)
                if total > MAX_OUTPUT_TOTAL or data != expected[relative]:
                    raise MetadataError("output-content-mismatch")
                seen.add(relative)
                if relative != "receipt.json":
                    observed_outputs[relative] = data

    try:
        _parent_unchanged(output, root_fd)
        visit(root_fd)
        if seen != set(expected):
            raise MetadataError("missing-output-file")
        verify_delivery(model, observed_outputs)
        if load_metadata(root, source, stage=stage) != model:
            raise MetadataError("source-changed-during-check")
        if _code_identity() != json.loads(expected["receipt.json"])["tools"]:
            raise MetadataError("generator-changed-during-check")
        _parent_unchanged(output, root_fd)
        return {"status": "checked-" + stage, "file_count": len(seen),
                "pending_count": len(model.pending), **field_counts(model)}
    finally:
        os.close(root_fd)
