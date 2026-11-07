"""Admit bounded reviewed metadata into immutable build-time values.

The POSIX build-host reader uses directory descriptors and no-follow opens.
These checks protect input custody, not the trustworthiness of executing Python,
dependencies or the host. Native Windows reading/installation is not qualified.
"""

from __future__ import annotations

import hashlib
import ipaddress
import os
import re
import stat
import tomllib
import unicodedata
from dataclasses import dataclass
from pathlib import Path
from urllib.parse import urlsplit

from .fields import FIELD_BY_KEY, FIELD_SPECS
from .text_policy import TextPolicyError, require_text, validate_source

MAX_FILE_BYTES = 256 * 1024
MAX_INPUT_BYTES = 1024 * 1024
MAX_DEPTH = 16
MAX_NODES = 8192  # Closed 716-field register plus bounded authoring headroom.
MAX_STRING = 16384  # A Store description may contain 10,000 characters.
TARGET = "x86_64-pc-windows-msvc"
CPU_FLOOR = "x86-64-v3"
REPOSITORY = "https://github.com/DavidOsipov/NevroTick"

# Code-owned admission is independent of candidate-controlled applicability rows.
_COMPONENTS = {
    "ui": ("ui", "fixture", "crates/nevrotick-ui/Cargo.toml", "nevrotick-ui", "GPL-3.0-only"),
    "observe": ("observer", "experiment", "crates/nevrotick-engine/Cargo.toml", "nevrotick-observe", "GPL-2.0-only"),
    "engine": ("engine", "planned", None, None, None),
    "broker": ("broker", "planned", None, None, None),
    "helper": ("helper", "planned", None, None, None),
}
_PENDING = {
    "release.version": "W11-W10/P6",
    "package.identity": "W11-W10/P6",
    "services.identities": "W11-W05",
    "diagnostics.provider": "W11-W08",
    "public.destinations": "W11-W10/P6",
    "release.qualification": "W11-W10/P6",
}


class MetadataError(ValueError):
    """Bounded source admission failed; no metadata should be published."""


@dataclass(frozen=True, slots=True)
class Product:
    """Reviewed public intent, distinct from signature or installed identity."""

    name: str
    publisher: str
    description: str
    repository: str
    homepage: str | None


@dataclass(frozen=True, slots=True)
class Component:
    """One role; planned roles intentionally have no emitted binary facts."""

    id: str
    role: str
    state: str
    binary_name: str | None
    description: str
    version: str | None
    file_version: tuple[int, int, int, int] | None
    license: str | None
    cargo_manifest: str | None
    display_name: str

    @property
    def emittable(self) -> bool:
        """Whether the admitted current source role has a prototype projection."""
        return self.state in {"fixture", "experiment"}


@dataclass(frozen=True, slots=True)
class PendingField:
    """Unresolved authoring/qualification obligation, never an invented value."""

    path: str
    owner: str
    reason: str
    required_stage: str
    next_action: str


@dataclass(frozen=True, slots=True)
class InputDigest:
    """Digest of the exact bounded bytes parsed under a root-relative name."""

    path: str
    sha256: str


@dataclass(frozen=True, slots=True)
class FieldValue:
    """One field disposition; blank unresolved values never imply release readiness."""

    key: str
    value: str
    status: str
    owner: str
    source: str
    reason: str


@dataclass(frozen=True, slots=True)
class Metadata:
    """Immutable admitted intent and observations; not a release approval."""

    schema_version: int
    stage: str
    product: Product
    target: str
    cpu_floor: str
    components: tuple[Component, ...]
    pending: tuple[PendingField, ...]
    inputs: tuple[InputDigest, ...]
    file_version_revision: int = 0
    fields: tuple[FieldValue, ...] = ()


def _table(value: object, field: str) -> dict[str, object]:
    """Require a literal string-keyed TOML table, without implicit conversion."""
    if type(value) is not dict or any(type(key) is not str for key in value):
        raise MetadataError(f"{field}: expected a table")
    return value


def _keys(table: dict[str, object], required: set[str], optional: set[str], field: str) -> None:
    """Reject unknown or missing owned fields before using any candidate value."""
    if set(table) - required - optional or required - set(table):
        raise MetadataError(f"{field}: missing or unknown fields")


def _text(value: object, field: str, limit: int = 1024, *, multiline: bool = False) -> str:
    """Enforce nonempty bounded text; preserve valid text without sanitization."""
    try:
        require_text(value)
    except TextPolicyError as error:
        raise MetadataError(f"{field}: text policy refused") from error
    if type(value) is not str or not value.strip() or len(value) > MAX_STRING:
        raise MetadataError(f"{field}: expected bounded nonempty text")
    for character in value:
        if (unicodedata.category(character) in {"Cc", "Cs"}
                and not (multiline and character in "\n\t")) or character in "\ufffe\uffff\u202a\u202b\u202c\u202d\u202e\u2066\u2067\u2068\u2069":
            raise MetadataError(f"{field}: unsupported control or XML character")
    if len(value.encode("utf-16-le")) // 2 > limit:
        raise MetadataError(f"{field}: exceeds its UTF-16 length limit")
    if not multiline and value != value.strip():
        raise MetadataError(f"{field}: leading or trailing whitespace")
    return value


def _relative(value: object, field: str) -> str:
    """Admit a portable root-relative path with no normalization or traversal."""
    path = _text(value, field, 512)
    if ("\\" in path or ":" in path or path.startswith("/")
            or any(part in {"", ".", ".."} for part in path.split("/"))
            or any(not re.fullmatch(r"[A-Za-z0-9_.-]+", part) for part in path.split("/"))):
        raise MetadataError(f"{field}: unsafe relative path")
    return path


def _url(value: object, field: str) -> str:
    """Require a bounded HTTPS DNS locator; prove no reachability or ownership."""
    text = _text(value, field, 2048)
    try:
        url = urlsplit(text)
        host = url.hostname
        port = url.port
    except ValueError as error:
        raise MetadataError(f"{field}: malformed URL") from error
    literal_address = False
    if host:
        try:
            ipaddress.ip_address(host)
            literal_address = True
        except ValueError:
            pass
    if (url.scheme != "https" or not host or url.username is not None
            or url.password is not None or port is not None or url.query or url.fragment
            or "\\" in text or "%" in url.netloc or "." not in host or literal_address
            or any(character.isspace() for character in text) or len(host) > 253
            or not re.search(r"[A-Za-z]", host.rsplit(".", 1)[-1])
            or host.lower().endswith((".localhost", ".local", ".internal"))
            or host.lower() in {"localhost", "example.com", "example.org", "example.net"}
            or not all(len(label) <= 63 and re.fullmatch(r"[A-Za-z0-9](?:[A-Za-z0-9-]*[A-Za-z0-9])?", label)
                       for label in host.split("."))):
        raise MetadataError(f"{field}: unsupported public HTTPS URL")
    return text


def _version(value: object, field: str) -> tuple[str, tuple[int, int, int, int]]:
    """Preserve Cargo SemVer and derive the explicit prototype numeric mapping."""
    text = _text(value, field, 128)
    match = re.fullmatch(r"(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)(?:-([0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*))?(?:\+[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*)?", text)
    if match is None:
        raise MetadataError(f"{field}: invalid Cargo semantic version")
    if match[4] and any(item.isdigit() and len(item) > 1 and item.startswith("0") for item in match[4].split(".")):
        raise MetadataError(f"{field}: invalid numeric prerelease identifier")
    numbers = tuple(int(match[index]) for index in (1, 2, 3))
    if any(number > 65535 for number in numbers):
        raise MetadataError(f"{field}: cannot represent version as Windows u16 values")
    return text, (numbers[0], numbers[1], numbers[2], 0)


def _preflight(raw: bytes, field: str) -> str:
    """Bound raw input and lexical nesting before invoking the TOML parser."""
    if len(raw) > MAX_FILE_BYTES:
        raise MetadataError(f"{field}: exceeds the input byte budget")
    try:
        text = raw.decode("utf-8")
    except UnicodeDecodeError as error:
        raise MetadataError(f"{field}: expected UTF-8") from error
    quote = ""
    escaped = comment = False
    depth = dots = index = 0
    key = True
    while index < len(text):
        character = text[index]
        if comment:
            if character == "\n":
                comment = False
                key, dots = True, 0
        elif quote:
            if escaped:
                escaped = False
            elif quote[0] == '"' and character == "\\":
                escaped = True
            elif text.startswith(quote, index):
                index += len(quote) - 1
                quote = ""
        elif character == "#":
            comment = True
        elif character in "\"'":
            quote = character * 3 if text.startswith(character * 3, index) else character
            index += len(quote) - 1
        elif character in "[{":
            depth += 1
            if depth > MAX_DEPTH:
                raise MetadataError(f"{field}: exceeds the lexical nesting budget")
        elif character in "]}":
            depth -= 1
        elif character == "=":
            key = False
        elif character == "." and key:
            dots += 1
            if dots >= MAX_DEPTH:
                raise MetadataError(f"{field}: exceeds the dotted-key depth budget")
        elif character == "\n":
            key, dots = True, 0
        index += 1
    return text


def _structure(value: object, field: str) -> None:
    """Bound decoded nodes, depth and strings without recursive traversal."""
    stack: list[tuple[object, int]] = [(value, 0)]
    count = 0
    while stack:
        item, depth = stack.pop()
        count += 1
        if count > MAX_NODES or depth > MAX_DEPTH:
            raise MetadataError(f"{field}: exceeds the decoded structure budget")
        if type(item) is dict:
            if len(item) > MAX_NODES:
                raise MetadataError(f"{field}: exceeds the table budget")
            for key, child in item.items():
                if type(key) is not str or len(key) > MAX_STRING:
                    raise MetadataError(f"{field}: oversized key")
                stack.append((child, depth + 1))
        elif type(item) is list:
            if len(item) > MAX_NODES:
                raise MetadataError(f"{field}: exceeds the array budget")
            stack.extend((child, depth + 1) for child in item)
        elif type(item) is str and len(item) > MAX_STRING:
            raise MetadataError(f"{field}: oversized string")


def _identity(info: os.stat_result) -> tuple[int, ...]:
    """Bind one open file identity and mutation-sensitive metadata snapshot."""
    return (info.st_dev, info.st_ino, info.st_mode, info.st_nlink, info.st_size,
            info.st_mtime_ns, info.st_ctime_ns)


def _open_root(root: Path) -> int:
    """Open every absolute root component without following directory links."""
    if os.name != "posix" or not hasattr(os, "O_NOFOLLOW") or os.open not in os.supports_dir_fd:
        raise MetadataError("input custody requires a qualified POSIX directory-descriptor host")
    absolute = root.absolute()
    if any(part in {".", ".."} for part in absolute.parts):
        raise MetadataError("root: noncanonical path")
    flags = os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | getattr(os, "O_CLOEXEC", 0)
    descriptor = os.open("/", flags)
    try:
        for part in absolute.parts[1:]:
            child = os.open(part, flags, dir_fd=descriptor)
            os.close(descriptor)
            descriptor = child
        return descriptor
    except BaseException:
        os.close(descriptor)
        raise


def _read(root_fd: int, relative: str) -> bytes:
    """Read one single-linked regular file within a fixed descriptor root."""
    _relative(relative, "input path")
    directory = os.dup(root_fd)
    stream: int | None = None
    try:
        parts = relative.split("/")
        for part in parts[:-1]:
            child = os.open(part, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW
                            | getattr(os, "O_CLOEXEC", 0), dir_fd=directory)
            os.close(directory)
            directory = child
        stream = os.open(parts[-1], os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK
                         | getattr(os, "O_CLOEXEC", 0), dir_fd=directory)
        before = os.fstat(stream)
        if not stat.S_ISREG(before.st_mode) or before.st_nlink != 1 or before.st_size > MAX_FILE_BYTES:
            raise MetadataError(f"{relative}: requires a bounded single-linked regular file")
        chunks = bytearray()
        while len(chunks) <= MAX_FILE_BYTES:
            chunk = os.read(stream, min(65536, MAX_FILE_BYTES + 1 - len(chunks)))
            if not chunk:
                break
            chunks.extend(chunk)
        after = os.fstat(stream)
        if len(chunks) != before.st_size or _identity(before) != _identity(after):
            raise MetadataError(f"{relative}: file changed during its bounded read")
        return bytes(chunks)
    except OSError as error:
        raise MetadataError(f"{relative}: input could not be safely read") from error
    finally:
        if stream is not None:
            os.close(stream)
        os.close(directory)


def _decode(raw: bytes, field: str) -> dict[str, object]:
    """Decode bounded TOML; parser failure never becomes a partial document."""
    text = _preflight(raw, field)
    try:
        result = tomllib.loads(text)
    except (ValueError, RecursionError) as error:
        raise MetadataError(f"{field}: malformed TOML") from error
    _structure(result, field)
    return result


def _source_text_policy(value: object) -> None:
    """Check all decoded authored keys/strings, including TOML escape results."""
    stack = [value]
    try:
        while stack:
            item = stack.pop()
            if type(item) is dict:
                for key, child in item.items():
                    require_text(key)
                    stack.append(child)
            elif type(item) is list:
                stack.extend(item)
            elif type(item) is str:
                require_text(item)
    except TextPolicyError as error:
        raise MetadataError("metadata: decoded text policy refused") from error


def _derived_value(metadata: Metadata, derivation: str) -> str:
    """Resolve only code-owned explicit facts; never evaluate source expressions."""
    product_values = {"product.name": metadata.product.name,
                      "product.publisher": metadata.product.publisher,
                      "product.description": metadata.product.description,
                      "product.repository": metadata.product.repository,
                      "product.homepage": metadata.product.homepage or "",
                      "platform.target": metadata.target, "platform.cpu_floor": metadata.cpu_floor}
    if derivation in product_values:
        return product_values[derivation]
    if derivation.startswith("component."):
        parts = derivation.split(".")
        if len(parts) == 3:
            component = next((item for item in metadata.components if item.id == parts[1]), None)
            if component is not None:
                values = {"description": component.description, "display_name": component.display_name,
                          "license": component.license or "", "version": component.version or "",
                          "binary_name": component.binary_name or "",
                          "original_filename": component.binary_name + ".exe" if component.binary_name else ""}
                if parts[2] in values:
                    return values[parts[2]]
    if derivation.startswith("field:"):
        target = derivation[6:]
        field = next((item for item in metadata.fields if item.key == target), None)
        if field is not None:
            return field.value
        raise MetadataError("fields: missing semantic alias target")
    if derivation.startswith(("per-component:", "per-service:", "per-listing:")):
        return ""  # A schema alias has no single value across its actual role instances.
    raise MetadataError("fields: unsupported code-owned derivation")


def _has_iso7064_mod11_2_check_digit(compact: str) -> bool:
    """Check exactly fifteen ASCII digits plus their decimal/X check character.

    ISO 7064 MOD 11-2 detects identifier transcription errors; it establishes
    neither registry assignment nor ownership. Each caller retains its exact
    ORCID/ISNI separator grammar and removes only those admitted separators.
    """
    if type(compact) is not str or re.fullmatch(r"[0-9]{15}[0-9X]", compact) is None:
        return False
    remainder = 0
    for character in compact[:-1]:
        remainder = ((remainder + ord(character) - ord("0")) * 2) % 11
    check = (12 - remainder) % 11
    return compact[-1] == ("X" if check == 10 else str(check))


def _validate_field_value(value: str, key: str, value_type: str) -> None:
    """Validate filled register values against their declared semantic type."""
    # A schema alias shares the single semantic target's constraints. Giving it
    # an independent generic cap can reject valid source values or admit values
    # outside the actual destination's narrower contract.
    alias = FIELD_BY_KEY[key].derivation
    if alias.startswith("field:"):
        target = FIELD_BY_KEY[alias[6:]]
        _validate_field_value(value, target.key, target.value_type)
        return
    limits = {"project.store.description_long": 10000, "project.store.description_short": 1000,
              "project.store.short_title": 50, "project.store.sort_title": 255,
              "project.store.voice_title": 255, "project.store.whats_new": 1500,
              "project.store.copyright_trademark": 200, "project.store.published_by": 255,
              "project.store.additional_license_terms": 10000}
    limit = limits.get(key, 10000 if value_type == "long-text" else 8192)
    _text(value, "field.value", limit, multiline=value_type in {"text", "long-text"})
    if value_type == "url":
        _url(value, "field.url")
    elif value_type == "relative-path":
        _relative(value, "field.path")
    elif value_type == "service-name":
        if len(value.encode("utf-16-le")) // 2 > 256 or "/" in value or "\\" in value:
            raise MetadataError("field: invalid technical service name")
    elif value_type == "mui-ref":
        match = re.fullmatch(r"@[A-Za-z0-9_.-]+\.(?:exe|dll),-([1-9][0-9]{0,4})", value, re.I)
        if match is None or int(match[1]) > 65535:
            raise MetadataError("field: expected owned relative PE MUI reference")
    elif value_type == "version":
        _version(value, "field.version")
    elif value_type in {"target", "cpu"}:
        if value != (TARGET if value_type == "target" else CPU_FLOOR):
            raise MetadataError("field: target policy mismatch")
    elif value_type == "guid":
        if re.fullmatch(r"(?:[0-9A-Fa-f]{8}(?:-[0-9A-Fa-f]{4}){3}-[0-9A-Fa-f]{12}|\{[0-9A-Fa-f]{8}(?:-[0-9A-Fa-f]{4}){3}-[0-9A-Fa-f]{12}\})", value) is None:
            raise MetadataError("field: invalid GUID")
    elif value_type == "email":
        if re.fullmatch(r"[A-Za-z0-9.!#$%&'*+/=?^_`{|}~-]+@[A-Za-z0-9](?:[A-Za-z0-9.-]*[A-Za-z0-9])?\.[A-Za-z]{2,63}", value) is None or len(value) > 254:
            raise MetadataError("field: invalid public email syntax")
    elif value_type == "isni":
        if re.fullmatch(r"[0-9]{4} [0-9]{4} [0-9]{4} [0-9]{3}[0-9X]", value) is None:
            raise MetadataError("field: invalid ISNI syntax")
        if not _has_iso7064_mod11_2_check_digit(value.replace(" ", "")):
            raise MetadataError("field: invalid ISNI check digit")
    elif value_type == "orcid":
        if re.fullmatch(r"[0-9]{4}-[0-9]{4}-[0-9]{4}-[0-9]{3}[0-9X]", value) is None:
            raise MetadataError("field: invalid ORCID syntax")
        if not _has_iso7064_mod11_2_check_digit(value.replace("-", "")):
            raise MetadataError("field: invalid ORCID check digit")
    elif value_type == "wikidata":
        if re.fullmatch(r"Q[1-9][0-9]{0,15}", value) is None:
            raise MetadataError("field: invalid Wikidata syntax")
    elif value_type == "digits":
        if re.fullmatch(r"[0-9]{1,32}", value) is None:
            raise MetadataError("field: expected bounded digits")
    elif value_type == "spdx":
        if value not in {"GPL-2.0-only", "GPL-3.0-only"}:
            raise MetadataError("field: licence differs from the current admitted authority")
    elif value_type in {"locale", "locale-list"}:
        if key in {"project.localization.adopted_locales", "project.localization.fallback_locale"} and value != "en":
            raise MetadataError("field: current UI locale policy admits English only")
        entries = value.split(";") if value_type == "locale-list" else [value]
        if len(entries) > 32 or len(set(entries)) != len(entries) or any(re.fullmatch(r"[A-Za-z]{2,3}(?:-[A-Za-z0-9]{2,8})*", entry) is None for entry in entries):
            raise MetadataError("field: invalid locale syntax")
    if key == "project.store.features":
        entries = value.split("\n")
        if len(entries) > 20 or any(not entry.strip() or len(entry) > 200 for entry in entries):
            raise MetadataError("field: Store features exceed their bounded contract")
    if key in {"project.store.hardware_minimum", "project.store.hardware_recommended"}:
        entries = value.split("\n")
        if len(entries) > 11 or any(not entry.strip() or len(entry) > 200 for entry in entries):
            raise MetadataError("field: Store hardware requirements exceed their bounded contract")
    if key == "project.store.keywords":
        entries = value.split(";")
        if len(entries) > 7 or sum(len(entry.split()) for entry in entries) > 21 or any(not entry.strip() or len(entry) > 40 for entry in entries):
            raise MetadataError("field: Store keywords exceed their bounded contract")


def _validate_fields(metadata: Metadata) -> None:
    """Require full code-owned coverage and reject forged values or dispositions."""
    if type(metadata.fields) is not tuple or len(metadata.fields) != len(FIELD_SPECS):
        raise MetadataError("fields: complete code-owned field register is required")
    # Establish the complete nested scalar shape before resolving cross-field
    # aliases, so malformed direct-construction inputs cannot escape as an
    # AttributeError or trigger user-defined equality conversions.
    for field in metadata.fields:
        if type(field) is not FieldValue or any(type(item) is not str for item in
                (field.key, field.value, field.status, field.owner, field.source, field.reason)):
            raise MetadataError("fields: exact typed records and strings are required")
    seen: set[str] = set()
    nonauthorable = {"runtime", "format", "behavior", "reserved", "container", "extension", "conditional-msi", "schema", "native", "observation"}
    for field in metadata.fields:
        if type(field) is not FieldValue or type(field.key) is not str or field.key not in FIELD_BY_KEY or field.key in seen:
            raise MetadataError("fields: unknown or duplicate typed field")
        seen.add(field.key)
        spec = FIELD_BY_KEY[field.key]
        if any(type(item) is not str for item in (field.value, field.status, field.owner, field.source, field.reason)):
            raise MetadataError("fields: exact string fields are required")
        try:
            for item in (field.key, field.value, field.status, field.owner, field.source, field.reason):
                require_text(item)
        except TextPolicyError as error:
            raise MetadataError("fields: text policy refused") from error
        if field.owner != spec.owner or field.status not in {"populated", "derived", "pending", "not-applicable"}:
            raise MetadataError("fields: unapproved owner or disposition")
        _text(field.reason, "field.reason", 1024)
        if field.source:
            _text(field.source, "field.source", 1024)
        if spec.derivation:
            if field.status != "derived" or field.value != _derived_value(metadata, spec.derivation):
                raise MetadataError("fields: fact must use its code-owned derivation")
        elif field.status in {"pending", "not-applicable"}:
            if field.value:
                raise MetadataError("fields: unresolved or inapplicable value must be empty")
        elif spec.kind in nonauthorable or spec.applicability == "catalogue-reference" or spec.applicability == "conditional" and spec.surface == "diagnostics":
            if field.value or field.status != "derived":
                raise MetadataError("fields: current mode does not authorize a static value")
        elif field.status == "derived":
            if field.value:
                raise MetadataError("fields: no admitted source derivation for this value")
        elif not field.value or not field.source:
            raise MetadataError("fields: populated values require a value and provenance")
        if field.value:
            _validate_field_value(field.value, field.key, spec.value_type)


def validate_model(metadata: Metadata) -> None:
    """Recheck all model invariants at public consumers, including forged values.

    Frozen dataclasses prevent mutation, not malicious construction. This guard
    is mandatory for transformed or hand-constructed values. It authenticates
    neither a model's source nor the native product and cannot approve a release.
    """
    if type(metadata) is not Metadata or type(metadata.schema_version) is not int or metadata.schema_version != 1:
        raise MetadataError("metadata: unsupported typed model/schema")
    if type(metadata.stage) is not str or metadata.stage not in {"source", "release"}:
        raise MetadataError("metadata: unsupported stage")
    if (type(metadata.target) is not str or type(metadata.cpu_floor) is not str
            or metadata.target != TARGET or metadata.cpu_floor != CPU_FLOOR):
        raise MetadataError("platform: target or CPU floor differs from the owned policy")
    if type(metadata.file_version_revision) is not int or not 0 <= metadata.file_version_revision <= 65535:
        raise MetadataError("version_policy: prototype revision must be a Windows u16")
    product = metadata.product
    if type(product) is not Product or product.name != "NevroTick" or product.publisher != "David Osipov":
        raise MetadataError("product: unapproved product or publisher identity")
    _text(product.name, "product.name", 256)
    _text(product.publisher, "product.publisher", 256)
    _text(product.description, "product.description", 4096, multiline=True)
    if _url(product.repository, "product.repository") != REPOSITORY:
        raise MetadataError("product.repository: differs from the project authority")
    if product.homepage is not None:
        _url(product.homepage, "product.homepage")
    if type(metadata.components) is not tuple or len(metadata.components) != len(_COMPONENTS):
        raise MetadataError("components: all owned roles are required")
    ids: set[str] = set()
    for component in metadata.components:
        if type(component) is not Component or type(component.id) is not str or component.id not in _COMPONENTS or component.id in ids:
            raise MetadataError("components: unknown or duplicate typed role")
        ids.add(component.id)
        role, state, manifest, binary, license_name = _COMPONENTS[component.id]
        observed_scalars = (component.role, component.state, component.cargo_manifest,
                            component.binary_name, component.license)
        expected_scalars = (role, state, manifest, binary, license_name)
        if any((observed is not None if expected is None
                else type(observed) is not str or observed != expected)
               for observed, expected in zip(observed_scalars, expected_scalars)):
            raise MetadataError("components: current role/state/manifest/binary/licence differs from admission")
        _text(component.display_name, "component.display_name", 256)
        _text(component.description, "component.description", 1024)
        if component.emittable:
            version, numbers = _version(component.version, "component.version")
            expected_numbers = (*numbers[:3], metadata.file_version_revision)
            if (type(component.file_version) is not tuple or len(component.file_version) != 4
                    or any(type(number) is not int for number in component.file_version)
                    or component.file_version != expected_numbers or version != component.version):
                raise MetadataError("component.file_version: invalid numeric mapping")
        elif component.version is not None or component.file_version is not None:
            raise MetadataError("planned component: must not acquire emitted version facts")
    if type(metadata.pending) is not tuple or len(metadata.pending) != len(_PENDING):
        raise MetadataError("pending: code-owned release obligations are required")
    paths: set[str] = set()
    for pending in metadata.pending:
        if type(pending) is not PendingField or type(pending.path) is not str or pending.path not in _PENDING or pending.path in paths:
            raise MetadataError("pending: unknown or duplicate obligation")
        paths.add(pending.path)
        if (type(pending.owner) is not str or type(pending.required_stage) is not str
                or pending.owner != _PENDING[pending.path] or pending.required_stage != "release"):
            raise MetadataError("pending: owner or required stage differs from the owned obligation")
        _text(pending.reason, "pending.reason", 1024)
        _text(pending.next_action, "pending.next_action", 1024)
    if type(metadata.inputs) is not tuple or len(metadata.inputs) != 4:
        raise MetadataError("inputs: exact product and Cargo snapshots are required")
    names: set[str] = set()
    for entry in metadata.inputs:
        if type(entry) is not InputDigest:
            raise MetadataError("inputs: invalid digest record")
        _relative(entry.path, "input.path")
        if entry.path in names or type(entry.sha256) is not str or not re.fullmatch(r"[a-f0-9]{64}", entry.sha256):
            raise MetadataError("inputs: duplicate or invalid digest")
        names.add(entry.path)
    required = {"Cargo.toml", "crates/nevrotick-ui/Cargo.toml", "crates/nevrotick-engine/Cargo.toml"}
    if not required <= names or len(names - required) != 1:
        raise MetadataError("inputs: missing Cargo authority or ambiguous source")
    _validate_fields(metadata)
    if metadata.stage == "release":
        raise MetadataError("release: metadata obligations and prototype exclusions remain open; no release output is admitted")


def load_metadata(root: Path, source: Path | None = None, *, stage: str = "source") -> Metadata:
    """Read reviewed source and actual Cargo facts without executing build code.

    Inputs remain read-only. All files are bounded regular snapshots under the
    same no-follow directory root. Unknown source keys and unresolved source
    invariants fail before generation. Release mode is deliberately closed while
    the code-owned product/metadata obligations and prototypes remain unresolved.
    """
    if not isinstance(root, Path) or (source is not None and not isinstance(source, Path)):
        raise MetadataError("root/source: expected pathlib paths")
    if type(stage) is not str or stage not in {"source", "release"}:
        raise MetadataError("stage: expected source or release")
    selected = source or Path("metadata/product.toml")
    if selected.is_absolute():
        try:
            selected = selected.relative_to(root.absolute())
        except ValueError as error:
            raise MetadataError("source: must be inside the selected root") from error
    relative = _relative(str(selected), "source")
    records: dict[str, tuple[dict[str, object], InputDigest]] = {}
    used = 0
    try:
        root_fd = _open_root(root)
    except OSError as error:
        raise MetadataError("root: cannot establish no-follow input custody") from error
    try:
        def read(path: str) -> dict[str, object]:
            """Reuse each actual immutable byte snapshot once within this load."""
            nonlocal used
            if path not in records:
                raw = _read(root_fd, path)
                used += len(raw)
                if used > MAX_INPUT_BYTES:
                    raise MetadataError("inputs: aggregate byte budget exceeded")
                if path == relative:
                    try:
                        validate_source(raw)
                    except TextPolicyError as error:
                        raise MetadataError("metadata: raw UTF-8/text policy refused") from error
                decoded = _decode(raw, path)
                if path == relative:
                    _source_text_policy(decoded)
                records[path] = (decoded, InputDigest(path, hashlib.sha256(raw).hexdigest()))
            return records[path][0]

        source_table = read(relative)
        _keys(source_table, {"schema_version", "product", "platform", "version_policy", "components", "pending", "fields"}, set(), "metadata")
        if type(source_table["schema_version"]) is not int or source_table["schema_version"] != 1:
            raise MetadataError("schema_version: expected integer 1")
        product = _table(source_table["product"], "product")
        _keys(product, {"name", "publisher", "description"}, {"homepage"}, "product")
        platform = _table(source_table["platform"], "platform")
        _keys(platform, {"target", "cpu_floor"}, set(), "platform")
        policy = _table(source_table["version_policy"], "version_policy")
        _keys(policy, {"file_version_revision"}, set(), "version_policy")
        revision = policy["file_version_revision"]
        if type(revision) is not int or not 0 <= revision <= 65535:
            raise MetadataError("version_policy: prototype revision must be a Windows u16")
        workspace = _table(read("Cargo.toml").get("workspace"), "Cargo.workspace")
        package = _table(workspace.get("package"), "Cargo.workspace.package")
        if package.get("license") != "GPL-2.0-only" or package.get("repository") != REPOSITORY:
            raise MetadataError("Cargo.workspace.package: source authority differs from admission")
        members = workspace.get("members")
        if (type(members) is not list or any(type(item) is not str for item in members)
                or not {"crates/nevrotick-ui", "crates/nevrotick-engine"} <= set(members)):
            raise MetadataError("Cargo.workspace.members: required actual components are absent")
        components = _table(source_table["components"], "components")
        if set(components) != set(_COMPONENTS):
            raise MetadataError("components: require all current and planned owned roles")
        admitted: list[Component] = []
        for identifier in sorted(components):
            entry = _table(components[identifier], "component")
            role, state, manifest, binary, license_name = _COMPONENTS[identifier]
            expected = {"role", "state", "display_name", "description"}
            if manifest is not None:
                expected |= {"cargo_manifest", "binary_name"}
            _keys(entry, expected, set(), "component")
            if entry["role"] != role or entry["state"] != state:
                raise MetadataError("component: candidate cannot change owned role or lifecycle stage")
            version = numbers = None
            if manifest is not None:
                if entry["cargo_manifest"] != manifest or entry["binary_name"] != binary:
                    raise MetadataError("component: unapproved Cargo manifest or binary")
                cargo = read(manifest)
                component_package = _table(cargo.get("package"), "Cargo.package")
                expected_name = "nevrotick-ui" if identifier == "ui" else "nevrotick-engine"
                if component_package.get("name") != expected_name:
                    raise MetadataError("Cargo.package.name: wrong component source")
                observed_license = component_package.get("license")
                if type(observed_license) is dict:
                    if (set(observed_license) != {"workspace"}
                            or type(observed_license["workspace"]) is not bool
                            or observed_license["workspace"] is not True):
                        raise MetadataError("Cargo.package.license: unsupported inheritance")
                    observed_license = package["license"]
                if observed_license != license_name:
                    raise MetadataError("Cargo.package.license: component licence differs from admission")
                version, numbers = _version(component_package.get("version"), "Cargo.package.version")
                numbers = (*numbers[:3], revision)
                if identifier == "observe":
                    bins = cargo.get("bin")
                    if (type(bins) is not list or sum(type(item) is dict and item.get("name") == binary
                                                     and item.get("path") == "src/bin/nevrotick-observe.rs"
                                                     for item in bins) != 1):
                        raise MetadataError("Cargo.bin: actual observe entry is absent")
                elif component_package.get("autobins", True) is not True or "bin" in cargo:
                    raise MetadataError("Cargo.bin: UI implicit binary contract changed")
            admitted.append(Component(identifier, role, state, binary, entry["description"], version,
                                      numbers, license_name, manifest, entry["display_name"]))
        pending = source_table["pending"]
        if type(pending) is not list or len(pending) != len(_PENDING):
            raise MetadataError("pending: all code-owned release obligations are required")
        pending_fields: list[PendingField] = []
        for raw in pending:
            entry = _table(raw, "pending")
            _keys(entry, {"path", "owner", "reason", "required_stage", "next_action"}, set(), "pending")
            pending_fields.append(PendingField(entry["path"], entry["owner"], entry["reason"],
                                               entry["required_stage"], entry["next_action"]))
        raw_fields = _table(source_table["fields"], "fields")
        if set(raw_fields) != set(FIELD_BY_KEY):
            raise MetadataError("fields: all code-owned fields must have a disposition")
        authored_fields: list[FieldValue] = []
        for key, raw in raw_fields.items():
            entry = _table(raw, "field")
            _keys(entry, {"value", "status", "source", "reason"}, set(), "field")
            if FIELD_BY_KEY[key].derivation and entry["value"] != "":
                raise MetadataError("fields: derived authority cannot be duplicated in authored input")
            authored_fields.append(FieldValue(key, entry["value"], entry["status"], FIELD_BY_KEY[key].owner,
                                             entry["source"], entry["reason"]))
        homepage = product.get("homepage")
        if type(homepage) is str and homepage == "":
            homepage = None
        value = Metadata(1, stage, Product(product["name"], product["publisher"], product["description"],
                                          package["repository"], homepage),
                         platform["target"], platform["cpu_floor"], tuple(admitted),
                         tuple(sorted(pending_fields, key=lambda item: str(item.path))),
                         tuple(records[path][1] for path in sorted(records)), revision)
        # Resolve immutable aliases after base source facts are available. The
        # source file remains the only authoring location; derived facts are not
        # independently editable copies.
        raw_by_key = {field.key: field for field in authored_fields}
        resolved_values: dict[str, str] = {}

        def resolve(key: str, active: frozenset[str] = frozenset()) -> str:
            """Resolve a finite trusted alias graph without evaluating data."""
            if key in resolved_values:
                return resolved_values[key]
            if key in active or len(active) > 8 or key not in raw_by_key:
                raise MetadataError("fields: invalid code-owned alias graph")
            derivation = FIELD_BY_KEY[key].derivation
            if derivation.startswith("field:"):
                result = resolve(derivation[6:], active | {key})
            elif derivation:
                result = _derived_value(value, derivation)
            else:
                result = raw_by_key[key].value
            resolved_values[key] = result
            return result

        resolved = tuple(FieldValue(field.key, resolve(field.key), field.status,
                                    field.owner, field.source, field.reason)
                         for field in sorted(authored_fields, key=lambda item: item.key))
        value = Metadata(value.schema_version, value.stage, value.product, value.target,
                         value.cpu_floor, value.components, value.pending, value.inputs,
                         value.file_version_revision, resolved)
        validate_model(value)
        return value
    finally:
        os.close(root_fd)
