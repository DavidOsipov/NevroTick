#!/usr/bin/env python3
"""Offline, unapproved publisher evidence. Contract and limits: adjacent README."""

from __future__ import annotations

import argparse
import base64
import binascii
import hashlib
import ipaddress
import json
import os
import re
import stat
import sys
import tempfile
import unicodedata
from urllib.parse import urlsplit


MANIFEST_BYTES = 65_536
SOURCE_BYTES = 524_288
TOTAL_SOURCE_BYTES = 2_097_152
MAX_SOURCES = 8
MAX_RECORDS = 1_024
TOTAL_RECORDS = 2_048
MAX_ADDRESSES = 8
MAX_RELATED = 8
MAX_STRING = 4_096
MAX_LINE = 8_192
MAX_DEPTH = 12
MAX_MEMBERS = 32
MAX_ITEMS = 4_096
MAX_TOKENS = 500_000
MAX_INTEGER = 12
OUTPUT_BYTES = 16_777_216

NTS = "jauderho-nts-markdown-v1"
NTP = "jauderho-public-ntp-markdown-v1"
ROUGH = "tannerryan-ecosystem-json-v1"
ADAPTERS = {NTS, NTP, ROUGH}
AUTHORITY = "unapproved-publisher-evidence"
VM_MARKER = (
    "The following servers are known to be virtualized and may be less accurate. YMMV."
)
HEADERS = {
    NTS: ["Hostname", "Stratum", "Location", "Owner", "Notes"],
    NTP: ["Hostname", "AS", "Stratum", "Location", "Owner", "Notes"],
}
SOURCE_ERRORS = {
    "input-file", "input-read", "input-size", "input-length", "input-digest",
    "input-blob", "text-encoding", "text-control", "text-line", "json-shape",
    "json-depth", "json-members", "json-items", "json-tokens", "json-string",
    "json-integer", "json-float", "json-duplicate", "json-syntax", "record-limit",
    "source-empty", "markdown-shape", "endpoint", "port", "key", "profile",
    "source-schema", "string", "uri", "unsupported-platform",
}


class Refusal(Exception):
    """Only a static code may reach a diagnostic; source text is never echoed."""

    def __init__(self, code: str) -> None:
        self.code = code
        super().__init__(code)


class AggregateRefusal(Refusal):
    """An aggregate work failure must not be downgraded to one failed source."""


def require(condition: bool, code: str) -> None:
    if not condition:
        raise Refusal(code)


def closed(value: object, keys: set[str], code: str = "json-shape") -> dict:
    require(type(value) is dict and set(value) == keys, code)
    return value


def integer(value: object, low: int, high: int, code: str) -> int:
    require(type(value) is int and low <= value <= high, code)
    return value


def string(value: object, *, empty: bool = True, code: str = "string") -> str:
    require(type(value) is str and len(value) <= MAX_STRING, code)
    require(empty or bool(value), code)
    require(not any(unicodedata.category(c).startswith("C") for c in value), code)
    return value


def hex_string(value: object, length: int) -> str:
    require(type(value) is str and re.fullmatch(r"[0-9a-f]{" + str(length) + r"}", value)
            is not None, "json-shape")
    return value


def uri(value: object) -> str:
    result = string(value, empty=False, code="uri")
    require(result.isascii() and not any(c.isspace() or c in '<>"\\' for c in result),
            "uri")
    try:
        parts = urlsplit(result)
        valid = (parts.scheme in {"http", "https"} and bool(parts.hostname)
                 and parts.username is None and parts.password is None)
        port = parts.port
    except ValueError:
        raise Refusal("uri") from None
    require(valid and (port is None or 1 <= port <= 65535), "uri")
    return result


def origin(value: object) -> dict:
    result = closed(value, {"uri", "revision", "git_blob", "license_uri", "related"})
    uri(result["uri"])
    for field in ("revision", "git_blob"):
        if result[field] is not None:
            hex_string(result[field], 40)
    if result["license_uri"] is not None:
        uri(result["license_uri"])
    related = result["related"]
    require(type(related) is list and len(related) <= MAX_RELATED, "json-shape")
    for entry in related:
        closed(entry, {"kind", "uri", "git_blob"})
        require(type(entry["kind"]) is str
                and entry["kind"] in {"canonical-input", "generator", "use-document"},
                "json-shape")
        uri(entry["uri"])
        if entry["git_blob"] is not None:
            hex_string(entry["git_blob"], 40)
    return result


def read_regular(path: str, limit: int) -> bytes:
    """Read at most limit+1 bytes; no FIFO/device or final symlink is admitted."""
    require(hasattr(os, "O_NOFOLLOW"), "unsupported-platform")
    descriptor = -1
    try:
        descriptor = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
        info = os.fstat(descriptor)
        require(stat.S_ISREG(info.st_mode), "input-file")
        require(info.st_size <= limit, "input-size")
        with os.fdopen(descriptor, "rb") as stream:
            descriptor = -1
            data = stream.read(limit + 1)
        require(len(data) <= limit, "input-size")
        return data
    except OSError:
        raise Refusal("input-read") from None
    finally:
        if descriptor >= 0:
            os.close(descriptor)


def utf8(data: bytes) -> str:
    try:
        result = data.decode("utf-8", errors="strict")
    except UnicodeError:
        raise Refusal("text-encoding") from None
    require(not result.startswith("\ufeff"), "text-encoding")
    return result


def json_preflight(text: str) -> None:
    """Bound nesting, containers and encoded tokens before JSON materialization."""
    stack: list[list] = []
    index = 0
    tokens = 0
    while index < len(text):
        char = text[index]
        if char in " \t\r\n":
            index += 1
            continue
        if char in "},]":
            if char == ",":
                require(bool(stack), "json-syntax")
                stack[-1][1] += 1
                cap = MAX_MEMBERS if stack[-1][0] == "{" else MAX_ITEMS
                require(stack[-1][1] <= cap, "json-members" if stack[-1][0] == "{"
                        else "json-items")
            else:
                require(bool(stack) and stack[-1][0] == ("{" if char == "}" else "["),
                        "json-syntax")
                stack.pop()
            index += 1
            continue
        if char == ":":
            index += 1
            continue
        if stack and stack[-1][1] == 0:
            stack[-1][1] = 1
        tokens += 1
        require(tokens <= MAX_TOKENS, "json-tokens")
        if char in "{[":
            stack.append([char, 0])
            require(len(stack) <= MAX_DEPTH, "json-depth")
            index += 1
        elif char == '"':
            start = index
            index += 1
            while index < len(text):
                # One non-BMP code point may occupy two six-character JSON
                # surrogate escapes. The decoded cap is still enforced below.
                require(index - start <= 12 * MAX_STRING + 1, "json-string")
                if text[index] == '"':
                    index += 1
                    break
                index += 2 if text[index] == "\\" else 1
            else:
                raise Refusal("json-syntax")
        else:
            start = index
            while index < len(text) and text[index] not in ' \t\r\n,:[]{}"':
                index += 1
                require(index - start <= MAX_INTEGER, "json-integer")
            require(index != start, "json-syntax")
    require(not stack, "json-syntax")


def strict_json(data: bytes) -> object:
    text = utf8(data)
    json_preflight(text)

    def pairs(values: list[tuple[str, object]]) -> dict:
        require(len(values) <= MAX_MEMBERS, "json-members")
        result = {}
        for key, value in values:
            require(key not in result, "json-duplicate")
            result[key] = value
        return result

    def refuse_number(_: str) -> object:
        raise Refusal("json-float")

    try:
        result = json.loads(text, object_pairs_hook=pairs, parse_float=refuse_number,
                            parse_constant=refuse_number)
    except (ValueError, RecursionError):
        raise Refusal("json-syntax") from None

    def check(value: object) -> None:
        if type(value) is str:
            string(value, code="json-string")
        elif type(value) is dict:
            for key, member in value.items():
                string(key, code="json-string")
                check(member)
        elif type(value) is list:
            require(len(value) <= MAX_ITEMS, "json-items")
            for member in value:
                check(member)

    check(result)
    return result


def encoded(value: object, limit: int = OUTPUT_BYTES) -> bytes:
    """Canonical R bytes, bounded during encoding; never HTML/Markdown output."""
    result = bytearray()
    encoder = json.JSONEncoder(ensure_ascii=True, sort_keys=True, separators=(",", ":"),
                               allow_nan=False)
    for part in encoder.iterencode(value):
        part = part.replace("<", "\\u003c").replace(">", "\\u003e").replace("&", "\\u0026")
        chunk = part.encode("ascii")
        require(len(result) + len(chunk) + 1 <= limit, "output-size")
        result.extend(chunk)
    result.extend(b"\n")
    return bytes(result)


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def endpoint(token: object, family: str, transport: str) -> dict:
    value = string(token, empty=False, code="endpoint")
    require(value.isascii() and not any(c.isspace() or c in "/\\@?#%" for c in value),
            "endpoint")
    explicit = None
    if value.startswith("["):
        match = re.fullmatch(r"\[([^\[\]]+)\](?::([0-9]+))?", value)
        require(match is not None, "endpoint")
        host_text, port_text = match.groups()
        try:
            host = {"kind": "ipv6", "value": str(ipaddress.IPv6Address(host_text))}
        except ValueError:
            raise Refusal("endpoint") from None
    else:
        require("[" not in value and "]" not in value and value.count(":") <= 1,
                "endpoint")
        host_text, separator, port_text = value.partition(":")
        if not separator:
            port_text = None
        try:
            address = ipaddress.IPv4Address(host_text)
        except ValueError:
            bare = host_text[:-1] if host_text.endswith(".") else host_text
            labels = bare.split(".")
            require(len(labels) >= 2 and len(bare) <= 253, "endpoint")
            require(not all(re.fullmatch(r"(?:[0-9]+|0[xX][0-9a-fA-F]+)", label)
                            for label in labels), "endpoint")
            require(all(re.fullmatch(r"[A-Za-z0-9](?:[A-Za-z0-9-]{0,61}[A-Za-z0-9])?",
                                     label) is not None for label in labels), "endpoint")
            host = {"kind": "dns", "value": bare.lower() + "."}
        else:
            host = {"kind": "ipv4", "value": str(address)}
    if port_text is not None:
        require(re.fullmatch(r"0|[1-9][0-9]{0,4}", port_text) is not None, "port")
        explicit = integer(int(port_text), 0, 65535, "port")
    require(family in {"nts", "ntp", "roughtime"}, "source-schema")
    require(transport in {"udp", "tcp"}, "source-schema")
    if explicit is None:
        require(family != "roughtime", "port")
        port = 4460 if family == "nts" else 123
        basis = "adapter-default:rfc8915-section4" if family == "nts" else "adapter-default:ntp-udp123"
    else:
        port, basis = explicit, "source-explicit"
    is_pool = (host["kind"] == "dns" and (host["value"] == "pool.ntp.org."
               or host["value"].endswith(".pool.ntp.org.")))
    return {"host": host, "transport": transport,
            "role": "nts-ke" if family == "nts" else family,
            "explicit_port": explicit, "effective_port": port, "port_basis": basis,
            "alias_kind": "dynamic-pool-alias" if is_pool else "not-classified",
            "port_state": "zero-unusable" if port == 0 else "nonzero-unqualified"}


def key_material(algorithm: object, spelling: object) -> dict:
    require(algorithm in {"ed25519", "ml-dsa-44"}, "profile")
    text = string(spelling, empty=False, code="key")
    require(text.isascii() and len(text) <= 2732, "key")
    try:
        material = base64.b64decode(text, validate=True)
    except (ValueError, binascii.Error):
        raise Refusal("key") from None
    require(base64.b64encode(material).decode("ascii") == text, "key")
    require(len(material) == 32 if algorithm == "ed25519" else 1 <= len(material) <= 2048,
            "key")
    return {"algorithm": algorithm, "hex": material.hex(), "bytes": len(material),
            "encoding_status": "canonical",
            "algorithm_status": "length-checked-only" if algorithm == "ed25519" else "unsupported"}


def meaning(adapter: str, raw: object, locator: object) -> dict:
    """The single raw-to-meaning constructor, including imported snapshot records."""
    require(adapter in ADAPTERS, "source-schema")
    metadata = {key: None for key in ("name", "owner", "location", "as", "stratum",
                                     "notes", "virtualization")}
    result = {"family": "roughtime", "endpoints": [], "information_uri": None,
              "claims": metadata, "geography": "unknown", "time_behavior": "unqualified",
              "use_permission": "unknown", "independence": "unknown", "profile": None,
              "key": None, "disposition": "candidate-unreviewed"}
    if adapter in HEADERS:
        closed(locator, {"line", "table"}, "source-schema")
        integer(locator["line"], 1, SOURCE_BYTES + 1, "source-schema")
        require(type(locator["table"]) is str
                and locator["table"] in {"unmarked", "source-claimed-virtualized"}, "source-schema")
        fields = {header.lower() for header in HEADERS[adapter]}
        closed(raw, fields, "source-schema")
        for value in raw.values():
            string(value)
        hostname = raw["hostname"]
        if hostname.startswith("[") and "](" in hostname:
            match = re.fullmatch(r"\[([^\[\]]+)\]\(([^()]+)\)", hostname)
            require(match is not None, "endpoint")
            hostname, information = match.groups()
            result["information_uri"] = uri(information)
        family = "nts" if adapter == NTS else "ntp"
        result["family"] = family
        result["endpoints"] = [endpoint(hostname, family, "tcp" if family == "nts" else "udp")]
        for field in ("owner", "location", "as", "stratum", "notes"):
            metadata[field] = raw.get(field)
        metadata["virtualization"] = locator["table"]
    else:
        closed(locator, {"pointer"}, "source-schema")
        require(type(locator["pointer"]) is str
                and re.fullmatch(r"/servers/(0|[1-9][0-9]{0,3})", locator["pointer"])
                is not None, "source-schema")
        integer(int(locator["pointer"].rsplit("/", 1)[1]), 0, MAX_RECORDS - 1, "source-schema")
        closed(raw, {"name", "version", "publicKeyType", "publicKey", "addresses"}, "source-schema")
        metadata["name"] = string(raw["name"], empty=False)
        algorithm = raw["publicKeyType"]
        require(type(algorithm) is str, "profile")
        key = key_material(algorithm, raw["publicKey"])
        version = raw["version"]
        interpretation = "maximum-version-claim"
        if algorithm == "ml-dsa-44":
            require(type(version) is str and version == "ml-dsa-44", "profile")
            result["disposition"] = "unsupported-algorithm"
            interpretation = "unsupported-experiment"
        elif type(version) is int:
            integer(version, 1, 65535, "profile")
            if version != 1:
                result["disposition"] = "unsupported-profile"
        else:
            require(type(version) is str and version == "IETF-Roughtime", "profile")
            interpretation = "ambiguous-label"
            result["disposition"] = "unresolved-profile"
        result["profile"] = {"raw_type": "integer" if type(version) is int else "string",
                             "declaration": version, "interpretation": interpretation,
                             "supported_profile": None}
        result["key"] = key
        addresses = raw["addresses"]
        require(type(addresses) is list and 1 <= len(addresses) <= MAX_ADDRESSES,
                "source-schema")
        for entry in addresses:
            closed(entry, {"protocol", "address"}, "source-schema")
            require(type(entry["protocol"]) is str and entry["protocol"] in {"udp", "tcp"},
                    "source-schema")
            result["endpoints"].append(endpoint(entry["address"], "roughtime", entry["protocol"]))
    return result


class RecordBudget:
    def __init__(self) -> None:
        self.used = 0

    def take(self) -> None:
        if self.used >= TOTAL_RECORDS:
            raise AggregateRefusal("aggregate-records")
        self.used += 1


def source_rows(adapter: str, data: bytes, budget: RecordBudget) -> list[tuple[dict, dict]]:
    """Recognize a complete supported source; never salvage a partial table/list."""
    if adapter == ROUGH:
        value = closed(strict_json(data), {"servers"}, "source-schema")
        rows = value["servers"]
        require(type(rows) is list, "source-schema")
        require(bool(rows), "source-empty")
        require(len(rows) <= MAX_RECORDS, "record-limit")
        result = []
        for index, raw in enumerate(rows):
            budget.take()
            result.append(({"pointer": f"/servers/{index}"}, raw))
        return result
    text = utf8(data).replace("\r\n", "\n")
    require(not any(unicodedata.category(c).startswith("C") and c != "\n" for c in text),
            "text-control")
    lines = text.split("\n")
    require(all(len(line) <= MAX_LINE for line in lines), "text-line")
    starts = [index for index, line in enumerate(lines) if line == "## The List"]
    ends = [index for index, line in enumerate(lines) if line == "## Star History"]
    require(len(starts) == len(ends) == 1 and starts[0] < ends[0], "markdown-shape")
    header = HEADERS[adapter]
    state = "header"
    table = "unmarked"
    table_rows = 0
    after_separator = False
    result = []
    for index in range(starts[0] + 1, ends[0]):
        line = lines[index].strip(" ")
        if not line:
            continue
        if line == VM_MARKER:
            require(state == "rows" and table == "unmarked" and table_rows > 0
                    and not after_separator, "markdown-shape")
            table, state, table_rows = "source-claimed-virtualized", "header", 0
            continue
        if line == "||":
            require(state == "rows" and table_rows > 0 and not after_separator,
                    "markdown-shape")
            after_separator = True
            continue
        require(line.startswith("|") and line.endswith("|"), "markdown-shape")
        cells = [cell.strip(" ") for cell in line[1:-1].split("|")]
        require(len(cells) == len(header), "markdown-shape")
        for cell in cells:
            string(cell)
        if state == "header":
            require(cells == header, "markdown-shape")
            state = "separator"
        elif state == "separator":
            require(all(re.fullmatch(r":?-{3,}:?", cell) is not None for cell in cells),
                    "markdown-shape")
            state = "rows"
        else:
            require(cells != header, "markdown-shape")
            require(len(result) < MAX_RECORDS, "record-limit")
            budget.take()
            raw = dict(zip((name.lower() for name in header), cells, strict=True))
            result.append(({"line": index + 1, "table": table}, raw))
            table_rows += 1
            after_separator = False
    require(state == "rows" and table_rows > 0 and not after_separator, "markdown-shape")
    require(bool(result), "source-empty")
    return result


def record(source: dict, locator: dict, raw: dict) -> dict:
    normalized = meaning(source["adapter"], raw, locator)
    identity = {"source_id": source["id"], "artifact_sha256": source["artifact"]["sha256"],
                "locator": locator, "raw": raw}
    return {"claim_id": digest(encoded(identity)), "locator": locator, "raw": raw,
            "meaning": normalized}


def endpoint_identity(value: dict) -> dict:
    return {key: value[key] for key in ("host", "transport", "role", "effective_port")}


def review_groups(sources: list[dict]) -> list[dict]:
    groups: dict[bytes, dict] = {}
    for source in sources:
        for item in source["records"]:
            # One original row contributes once to a given endpoint group, even
            # when that row repeats an identical address. Its raw list survives.
            seen = set()
            for address in item["meaning"]["endpoints"]:
                target = endpoint_identity(address)
                group_key = encoded(target)
                if group_key in seen:
                    continue
                seen.add(group_key)
                group = groups.setdefault(group_key, {"endpoint": target, "claims": []})
                group["claims"].append(item)
    result = []
    for group_key in sorted(groups):
        group = groups[group_key]
        if len(group["claims"]) < 2:
            continue
        equal = len({encoded(item["meaning"]) for item in group["claims"]}) == 1
        require(len(result) < MAX_ITEMS, "output-size")
        result.append({"endpoint": group["endpoint"],
                       "claim_ids": sorted(item["claim_id"] for item in group["claims"]),
                       "kind": "equivalent-claims" if equal else "differing-claims"})
    return result


def snapshot(sources: list[dict], producer: str, manifest: str) -> dict:
    ordered = sorted(sources, key=lambda source: source["id"])
    count = sum(len(source["records"]) for source in ordered)
    require(count <= TOTAL_RECORDS, "aggregate-records")
    return {"schema": "nevrotick-r-candidate-v1", "authority": AUTHORITY,
            "producer_sha256": producer, "manifest_sha256": manifest,
            "status": "complete-candidate" if all(source["status"] == "complete"
                                                  for source in ordered) else "incomplete-candidate",
            "sources": ordered, "record_count": count, "review_groups": review_groups(ordered)}


def source_descriptor(value: object) -> dict:
    item = closed(value, {"id", "adapter", "file", "sha256", "bytes", "origin"})
    require(type(item["id"]) is str and re.fullmatch(r"[a-z][a-z0-9-]{0,63}", item["id"])
            is not None, "manifest")
    require(type(item["adapter"]) is str and item["adapter"] in ADAPTERS, "manifest")
    require(type(item["file"]) is str and re.fullmatch(r"[A-Za-z0-9][A-Za-z0-9._-]{0,127}",
                                                      item["file"]) is not None, "manifest")
    hex_string(item["sha256"], 64)
    integer(item["bytes"], 0, SOURCE_BYTES, "manifest")
    origin(item["origin"])
    return item


def collect(manifest_path: str) -> dict:
    data = read_regular(manifest_path, MANIFEST_BYTES)
    manifest = closed(strict_json(data), {"schema", "sources"})
    require(manifest["schema"] == "nevrotick-r-input-v1", "manifest")
    entries = manifest["sources"]
    require(type(entries) is list and 1 <= len(entries) <= MAX_SOURCES, "manifest")
    for entry in entries:
        source_descriptor(entry)
    require(len({entry["id"] for entry in entries}) == len(entries), "manifest")
    require(sum(entry["bytes"] for entry in entries) <= TOTAL_SOURCE_BYTES, "aggregate-bytes")
    budget = RecordBudget()
    results = []
    directory = os.path.dirname(os.path.abspath(manifest_path))
    for entry in sorted(entries, key=lambda item: item["id"]):
        source = {"id": entry["id"], "adapter": entry["adapter"],
                  "artifact": {key: entry[key] for key in ("sha256", "bytes", "origin")},
                  "observed": None, "status": "failed", "error": None, "records": []}
        try:
            raw_bytes = read_regular(os.path.join(directory, entry["file"]), entry["bytes"])
            source["observed"] = {"sha256": digest(raw_bytes), "bytes": len(raw_bytes)}
            require(len(raw_bytes) == entry["bytes"], "input-length")
            require(digest(raw_bytes) == entry["sha256"], "input-digest")
            blob = entry["origin"]["git_blob"]
            if blob is not None:
                actual = hashlib.sha1(b"blob " + str(len(raw_bytes)).encode("ascii") + b"\0"
                                      + raw_bytes).hexdigest()
                require(actual == blob, "input-blob")
            rows = source_rows(entry["adapter"], raw_bytes, budget)
            records = [record(source, locator, raw) for locator, raw in rows]
            source["records"], source["status"] = records, "complete"
        except AggregateRefusal:
            raise
        except Refusal as error:
            require(error.code in SOURCE_ERRORS, "internal-contract")
            source["error"] = error.code
        results.append(source)
    result = snapshot(results, digest(read_regular(__file__, SOURCE_BYTES)), digest(data))
    # Reuse the same imported-snapshot gate before publishing our own output.
    return validate_snapshot(result)


def validate_snapshot(value: object) -> dict:
    root = closed(value, {"schema", "authority", "producer_sha256", "manifest_sha256",
                          "status", "sources", "record_count", "review_groups"}, "snapshot")
    require(root["schema"] == "nevrotick-r-candidate-v1" and root["authority"] == AUTHORITY,
            "snapshot")
    hex_string(root["producer_sha256"], 64)
    hex_string(root["manifest_sha256"], 64)
    require(type(root["sources"]) is list and 1 <= len(root["sources"]) <= MAX_SOURCES,
            "snapshot")
    sources = []
    identities = set()
    total = 0
    byte_total = 0
    for source in root["sources"]:
        closed(source, {"id", "adapter", "artifact", "observed", "status", "error", "records"},
               "snapshot")
        artifact = closed(source["artifact"], {"sha256", "bytes", "origin"}, "snapshot")
        source_descriptor({"id": source["id"], "adapter": source["adapter"], "file": "input",
                           **artifact})
        require(source["id"] not in identities, "snapshot")
        identities.add(source["id"])
        byte_total += artifact["bytes"]
        require(byte_total <= TOTAL_SOURCE_BYTES, "aggregate-bytes")
        observed = source["observed"]
        if observed is not None:
            closed(observed, {"sha256", "bytes"}, "snapshot")
            hex_string(observed["sha256"], 64)
            integer(observed["bytes"], 0, SOURCE_BYTES, "snapshot")
        rows = source["records"]
        require(type(rows) is list and len(rows) <= MAX_RECORDS, "snapshot")
        total += len(rows)
        require(total <= TOTAL_RECORDS, "aggregate-records")
        rebuilt = {**source, "records": []}
        if source["status"] == "failed":
            require(type(source["error"]) is str and source["error"] in SOURCE_ERRORS and not rows,
                    "snapshot")
        else:
            require(source["status"] == "complete" and source["error"] is None and bool(rows),
                    "snapshot")
            require(observed is not None and observed == {"sha256": artifact["sha256"],
                                                           "bytes": artifact["bytes"]}, "snapshot")
            previous_line = 0
            virtualized = False
            for index, item in enumerate(rows):
                closed(item, {"claim_id", "locator", "raw", "meaning"}, "snapshot")
                current = record(source, item["locator"], item["raw"])
                if source["adapter"] == ROUGH:
                    require(item["locator"]["pointer"] == f"/servers/{index}", "snapshot")
                else:
                    require(item["locator"]["line"] > previous_line, "snapshot")
                    previous_line = item["locator"]["line"]
                    is_vm = item["locator"]["table"] == "source-claimed-virtualized"
                    require(index != 0 or not is_vm, "snapshot")
                    require(not virtualized or is_vm, "snapshot")
                    virtualized = is_vm
                require(encoded(item) == encoded(current), "snapshot-derived")
                rebuilt["records"].append(current)
        sources.append(rebuilt)
    result = snapshot(sources, root["producer_sha256"], root["manifest_sha256"])
    require(encoded(root) == encoded(result), "snapshot-derived")
    return result


def semantic_groups(source: dict) -> dict[bytes, dict]:
    groups: dict[bytes, dict] = {}
    for item in source["records"]:
        value = item["meaning"]
        anchor = (value["claims"]["name"] if value["family"] == "roughtime"
                  else value["endpoints"][0]["host"]["value"])
        join = {"family": value["family"], "anchor": anchor}
        group = groups.setdefault(encoded(join), {"join": join, "meanings": []})
        group["meanings"].append(value)
    for group in groups.values():
        group["meanings"].sort(key=encoded)
    return groups


def compare(before: dict, after: dict, before_bytes: bytes, after_bytes: bytes) -> dict:
    """Revalidate even direct callers; imported origins confer no authority."""
    before, after = validate_snapshot(before), validate_snapshot(after)
    require(encoded(before) == before_bytes and encoded(after) == after_bytes,
            "snapshot-noncanonical")
    old = {source["id"]: source for source in before["sources"]}
    new = {source["id"]: source for source in after["sources"]}
    comparisons = []
    for source_id in sorted(old.keys() | new.keys()):
        left, right = old.get(source_id), new.get(source_id)
        row = {"id": source_id, "comparison": "compared", "reason": None,
               "provenance_changed": encoded(left) != encoded(right),
               "added": [], "absent": [], "changed": [], "unchanged_groups": 0}
        if left is None or right is None:
            row["reason"] = "missing-before" if left is None else "missing-after"
        elif left["status"] != "complete" or right["status"] != "complete":
            row["reason"] = "source-failed"
        elif left["adapter"] != right["adapter"]:
            row["reason"] = "adapter-changed"
        if row["reason"] is not None:
            row["comparison"] = "blocked"
            comparisons.append(row)
            continue
        left_groups, right_groups = semantic_groups(left), semantic_groups(right)
        for join in sorted(left_groups.keys() | right_groups.keys()):
            left_group, right_group = left_groups.get(join), right_groups.get(join)
            if left_group is None:
                row["added"].append(right_group)
            elif right_group is None:
                row["absent"].append(left_group)
            elif encoded(left_group) == encoded(right_group):
                row["unchanged_groups"] += 1
            else:
                left_values, right_values = left_group["meanings"], right_group["meanings"]
                fields = [field for field in sorted(left_values[0])
                          if sorted(encoded(value[field]) for value in left_values)
                          != sorted(encoded(value[field]) for value in right_values)]
                if len(left_values) != len(right_values):
                    fields.append("claim-multiplicity")
                if not fields:
                    fields.append("binding-scope")
                row["changed"].append({"join": left_group["join"], "fields": fields,
                                       "before": left_values, "after": right_values})
        comparisons.append(row)
    return {"schema": "nevrotick-r-diff-v1", "authority": AUTHORITY,
            "before": {"sha256": digest(before_bytes), "bytes": len(before_bytes)},
            "after": {"sha256": digest(after_bytes), "bytes": len(after_bytes)},
            "producer_changed": before["producer_sha256"] != after["producer_sha256"],
            "status": "comparison-blocked" if any(item["comparison"] == "blocked"
                                                    for item in comparisons) else "compared",
            "sources": comparisons}


def load_snapshot(path: str, expected_digest: str) -> tuple[dict, bytes]:
    hex_string(expected_digest, 64)
    data = read_regular(path, OUTPUT_BYTES)
    require(digest(data) == expected_digest, "snapshot-digest")
    value = validate_snapshot(strict_json(data))
    require(encoded(value) == data, "snapshot-noncanonical")
    return value, data


def publish(path: str, data: bytes) -> bool:
    """No clobber. Return cleanup status AFTER the successful link commit point."""
    require(len(data) <= OUTPUT_BYTES, "output-size")
    directory = os.path.dirname(os.path.abspath(path))
    temporary = None
    published = False
    clean = True
    code = "output-publication"
    try:
        descriptor, temporary = tempfile.mkstemp(prefix=".catalogue-", suffix=".partial", dir=directory)
        with os.fdopen(descriptor, "wb") as stream:
            stream.write(data)
            stream.flush()
            os.fsync(stream.fileno())
        os.link(temporary, path, follow_symlinks=False)
        published = True
    except FileExistsError:
        code = "output-exists"
    except OSError:
        pass
    finally:
        if temporary is not None:
            try:
                os.unlink(temporary)
            except OSError:
                clean = False
    require(published, code)
    return clean


class Parser(argparse.ArgumentParser):
    def error(self, message: str) -> None:
        raise Refusal("arguments")


def diagnostic(message: str, *, error: bool = False) -> None:
    """Best-effort acknowledgement must not reclassify a committed artifact."""
    try:
        print(message, file=sys.stderr if error else sys.stdout, flush=True)
    except (OSError, ValueError):
        # Shutdown/OS termination may still obscure process acknowledgement;
        # no stream mutation or filesystem rollback is attempted here.
        pass


def main(arguments: list[str] | None = None) -> int:
    try:
        arguments = sys.argv[1:] if arguments is None else arguments
        require(len(arguments) <= 16 and all(type(arg) is str and len(arg) <= MAX_STRING
                                           for arg in arguments), "arguments")
        parser = Parser(description="Offline unapproved server-catalogue evidence")
        commands = parser.add_subparsers(dest="command", required=True)
        collection = commands.add_parser("collect")
        collection.add_argument("--manifest", required=True)
        collection.add_argument("--output", required=True)
        comparison = commands.add_parser("diff")
        for name in ("before", "before-sha256", "after", "after-sha256", "output"):
            comparison.add_argument("--" + name, required=True)
        args = parser.parse_args(arguments)
        if args.command == "collect":
            result = collect(args.manifest)
            status = 0 if result["status"] == "complete-candidate" else 3
        else:
            before, before_bytes = load_snapshot(args.before, args.before_sha256)
            after, after_bytes = load_snapshot(args.after, args.after_sha256)
            result = compare(before, after, before_bytes, after_bytes)
            status = 0 if result["status"] == "compared" else 3
        payload = encoded(result)
        # Counts and field validation alone do not bound serialized fan-out.
        # Generated candidate and diff bytes enter the same strict JSON gate
        # as imported evidence before the output directory is touched.
        strict_json(payload)
        clean = publish(args.output, payload)
        if not clean:
            diagnostic("warning: temporary-cleanup-after-publication", error=True)
        diagnostic("published: " + result["status"])
        return status
    except Refusal as error:
        diagnostic("refused: " + error.code, error=True)
        return 2
    except OSError:
        diagnostic("refused: local-io", error=True)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
