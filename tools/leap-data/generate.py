#!/usr/bin/env python3
"""Bounded offline leap-data generation from reviewed local input bytes.

Parsing checks consistency and calendar structure. It grants no authentication,
current date authority, runtime reload capability or Windows clock effect.
"""
from __future__ import annotations

import argparse
from dataclasses import dataclass
from datetime import date, datetime, timezone
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import re
import stat
import sys
import tempfile
from typing import BinaryIO

RAW_LIMIT = 65536
MANIFEST_LIMIT = 16384
LINE_LIMIT = 512
LINES_LIMIT = 1024
RECORD_LIMIT = 128
I64_MAX = 9223372036854775807
I32_MIN, I32_MAX = -2147483648, 2147483647
BASELINE = (2272060800, 10)
HERE = Path(__file__).resolve().parent
LIMITS = {
    "generated_bytes": 65536, "interpretation_record_inspections": 128,
    "line_bytes_without_ending": 512, "metadata_occurrences_per_kind": 1,
    "offset_maximum": I32_MAX, "offset_minimum": I32_MIN,
    "physical_lines": 1024, "raw_bytes": 65536, "records_including_baseline": 128,
    "signed_offset_characters": 11, "timestamp_decimal_digits": 19,
    "timestamp_maximum": I64_MAX,
}
RECORD = re.compile(rb"([0-9]{1,19})[ \t]+([+-]?[0-9]+)[ \t]*(?:#.*)?")
TIMESTAMP = re.compile(rb"#[\$@][ \t]+([0-9]{1,19})[ \t]*(?:#.*)?")
CHECKSUM = re.compile(rb"#h[ \t]+([0-9a-fA-F]{1,8})[ \t]+([0-9a-fA-F]{1,8})"
                      rb"[ \t]+([0-9a-fA-F]{1,8})[ \t]+([0-9a-fA-F]{1,8})"
                      rb"[ \t]+([0-9a-fA-F]{1,8})[ \t]*(?:#.*)?")


class GeneratorError(ValueError):
    """A bounded, non-authenticating input or generation failure."""


@dataclass(frozen=True)
class Ledger:
    records: tuple[tuple[int, int], ...]
    updated: int | None
    expires: int | None
    checksum: str | None


@dataclass(frozen=True)
class Reviewed:
    ledger: Ledger
    history_complete_before: int
    continuous_from: int
    raw_sha256: str
    provenance_sha256: str
    licence_sha256: str
    protected_paths: tuple[Path, ...]


def require(ok: bool, message: str) -> None:
    if not ok:
        raise GeneratorError(message)


def read_bounded(stream: BinaryIO, limit: int) -> bytes:
    """Read at most limit+1 bytes; reject rather than truncate."""
    value = stream.read(limit + 1)
    require(len(value) <= limit, "input byte limit exceeded")
    return value


def read_local(path: Path, limit: int) -> bytes:
    require(path.is_absolute() and path.resolve(strict=True) == path, "noncanonical local input path")
    # A FIFO must be rejected before a blocking buffered open can wait for a
    # writer. The descriptor is nonblocking and checked before any read.
    flags = os.O_RDONLY | getattr(os, "O_NONBLOCK", 0) | getattr(os, "O_NOFOLLOW", 0)
    descriptor = os.open(path, flags)
    try:
        require(stat.S_ISREG(os.fstat(descriptor).st_mode), "regular local input required")
        with os.fdopen(descriptor, "rb", closefd=False) as stream:
            return read_bounded(stream, limit)
    finally:
        os.close(descriptor)


def physical_lines(raw: bytes) -> tuple[bytes, ...]:
    require(type(raw) is bytes and len(raw) <= RAW_LIMIT, "input byte limit exceeded")
    require(all(32 <= c <= 126 or c in (9, 10, 13) for c in raw), "non-ASCII or control byte")
    lines = raw.split(b"\n")
    ended = raw.endswith(b"\n")
    if ended:
        lines.pop()
    if not raw:
        lines = []
    require(len(lines) <= LINES_LIMIT, "physical line limit exceeded")
    clean = []
    for index, line in enumerate(lines):
        if line.endswith(b"\r") and (ended or index < len(lines) - 1):
            line = line[:-1]
        require(b"\r" not in line, "bare carriage return")
        require(len(line) <= LINE_LIMIT, "line byte limit exceeded")
        clean.append(line)
    return tuple(clean)


def unsigned(token: bytes) -> int:
    require(1 <= len(token) <= 19 and token.isdigit(), "invalid timestamp decimal")
    value = int(token)
    require(value <= I64_MAX, "timestamp range exceeded")
    return value


def month_start(seconds: int) -> bool:
    # Gregorian dates repeat every 400 years. Reduction keeps the full i64
    # timestamp range check while avoiding datetime's year-9999 limitation.
    if seconds % 86400:
        return False
    ordinal = (date(1900, 1, 1).toordinal() - 1 + seconds // 86400) % 146097 + 1
    return date.fromordinal(ordinal).day == 1


def parse_leap_list(raw: bytes) -> Ledger:
    """Consume the entire bounded grammar, preserving optional metadata."""
    lines = physical_lines(raw)
    records: list[tuple[int, int]] = []
    metadata: dict[bytes, int | str] = {}
    digits = bytearray()
    for line in lines:
        if not line.strip(b" \t"):
            continue
        require(line[0] not in (9, 32), "leading whitespace on nonblank line")
        if line.startswith((b"#$", b"#@", b"#h")):
            marker = line[:2]
            require(marker not in metadata, "duplicate semantic metadata")
            if marker == b"#h":
                match = CHECKSUM.fullmatch(line)
                require(match is not None, "malformed compatibility checksum")
                assert match is not None
                metadata[marker] = b"".join(int(word, 16).to_bytes(4, "big")
                                             for word in match.groups()).hex()
                continue
            match = TIMESTAMP.fullmatch(line)
            require(match is not None, "malformed timestamp metadata")
            assert match is not None
            metadata[marker] = unsigned(match.group(1))
            contribution = line[2:]
        elif line.startswith(b"#"):
            continue
        else:
            match = RECORD.fullmatch(line)
            require(match is not None, "malformed ledger record")
            assert match is not None
            timestamp, offset = match.groups()
            require(len(offset) <= 11, "offset character limit exceeded")
            value = int(offset)
            require(I32_MIN <= value <= I32_MAX, "offset range exceeded")
            require(len(records) < RECORD_LIMIT, "record limit exceeded")
            records.append((unsigned(timestamp), value))
            contribution = line
        digits.extend(c for c in contribution.split(b"#", 1)[0] if 48 <= c <= 57)
    require(bool(records), "no ledger records")
    expected = metadata.get(b"#h")
    if expected is not None:
        actual = hashlib.sha1(digits, usedforsecurity=False).hexdigest()
        require(expected == actual, "compatibility checksum mismatch")
    require(records[0] == BASELINE, "invalid 1972 baseline")
    for previous, current in zip(records, records[1:]):
        require(current[0] > previous[0], "records not strictly increasing")
        require(month_start(current[0]), "transition is not a monthly UTC effective instant")
        require(abs(current[1] - previous[1]) == 1, "invalid offset step")
    updated, expires = metadata.get(b"#$"), metadata.get(b"#@")
    assert updated is None or type(updated) is int
    assert expires is None or type(expires) is int
    assert expected is None or type(expected) is str
    return Ledger(tuple(records), updated, expires, expected)


def object_fields(value: object, fields: set[str], name: str) -> dict[str, object]:
    if type(value) is not dict:
        raise GeneratorError("invalid " + name + " fields")
    result: dict[str, object] = {}
    for key, item in value.items():
        if type(key) is not str:
            raise GeneratorError("invalid " + name + " field name")
        result[key] = item
    require(set(result) == fields, "invalid " + name + " fields")
    return result


def text(value: object, name: str) -> str:
    if (type(value) is not str or not 0 < len(value) <= 1024
            or not value.isascii() or any(not 32 <= ord(c) <= 126 for c in value)):
        raise GeneratorError("invalid " + name + " text")
    return value


def number(value: object, name: str, maximum: int = I64_MAX) -> int:
    if type(value) is not int or not 0 <= value <= maximum:
        raise GeneratorError("invalid " + name + " integer")
    return value


def digest(value: object, name: str, width: int = 64) -> str:
    if type(value) is not str or re.fullmatch("[0-9a-f]{" + str(width) + "}", value) is None:
        raise GeneratorError("invalid " + name + " digest")
    return value


def local_member(parent: Path, value: object) -> Path:
    name = text(value, "local input path")
    parts = PurePosixPath(name)
    require(not parts.is_absolute() and str(parts) == name and ".." not in parts.parts
            and "\\" not in name and bool(parts.parts), "invalid relative input path")
    return parent / name


def utc_identity(value: object, seconds: int) -> None:
    spelling = text(value, "reviewed UTC")
    require(re.fullmatch(r"[0-9]{4}-[0-9]{2}-[0-9]{2}T00:00:00Z", spelling) is not None,
            "invalid reviewed UTC spelling")
    try:
        instant = datetime.fromisoformat(spelling.replace("Z", "+00:00"))
    except ValueError as error:
        raise GeneratorError("invalid reviewed UTC date") from error
    difference = instant - datetime(1900, 1, 1, tzinfo=timezone.utc)
    require(difference.days * 86400 + difference.seconds == seconds, "reviewed UTC identity mismatch")


def strict_manifest(raw: bytes) -> dict[str, object]:
    def pairs(items: list[tuple[str, object]]) -> dict[str, object]:
        result: dict[str, object] = {}
        for key, value in items:
            require(key not in result, "duplicate provenance field")
            result[key] = value
        return result
    def invalid(_: str) -> None:
        raise GeneratorError("nonfinite provenance value")
    require(len(raw) <= MANIFEST_LIMIT, "provenance byte limit exceeded")
    # Bound nesting before the recursive JSON decoder allocates containers.
    depth = 0
    quoted = escaped = False
    for char in raw:
        if quoted:
            if escaped:
                escaped = False
            elif char == 92:
                escaped = True
            elif char == 34:
                quoted = False
        elif char == 34:
            quoted = True
        elif char in (91, 123):
            depth += 1
            require(depth <= 8, "provenance depth limit exceeded")
        elif char in (93, 125):
            depth -= 1
            require(depth >= 0, "malformed provenance JSON")
    try:
        value: object = json.loads(raw, object_pairs_hook=pairs, parse_constant=invalid)
    except (ValueError, UnicodeError, RecursionError) as error:
        if isinstance(error, GeneratorError):
            raise
        raise GeneratorError("malformed provenance JSON") from error
    return object_fields(value, {"checksum_reference", "history_review", "licence", "limits",
        "production_date_scope", "raw_input", "schema_version", "selected_continuous_rule", "status"}, "provenance")


def load_reviewed(path: Path) -> Reviewed:
    """Validate all local provenance fields/pins before constructing output."""
    path = path.absolute()
    provenance_raw = read_local(path, MANIFEST_LIMIT)
    provenance = strict_manifest(provenance_raw)
    require(type(provenance["schema_version"]) is int and provenance["schema_version"] == 1,
            "unsupported provenance schema")
    text(provenance["status"], "source status")
    require(provenance["status"] == "pinned-source-inputs-only; no product or release qualification",
            "unsupported source data role")
    limits = object_fields(provenance["limits"], set(LIMITS), "limits")
    require(all(type(limits[k]) is int and limits[k] == v for k, v in LIMITS.items()),
            "unsupported input or output limits")
    source = object_fields(provenance["raw_input"], {"bytes", "checksum_role",
        "compatibility_checksum_sha1", "expires_before_ntp_seconds", "maximum_line_bytes",
        "path", "physical_lines", "records", "sha256", "updated_ntp_seconds", "url"}, "raw input")
    licence = object_fields(provenance["licence"], {"bytes", "path", "scope", "sha256", "url"}, "licence")
    history = object_fields(provenance["history_review"], {"baseline_ntp_seconds",
        "baseline_tai_minus_utc_seconds", "complete_before_ntp_seconds", "complete_before_utc",
        "semantics", "source_title", "source_url"}, "history review")
    rule = object_fields(provenance["selected_continuous_rule"], {"effective_ntp_seconds",
        "effective_utc", "semantics", "source_doi", "source_url"}, "continuous rule")
    reference = object_fields(provenance["checksum_reference"], {"role", "url"}, "checksum reference")
    scope = object_fields(provenance["production_date_scope"], {"default", "semantics"}, "date scope")
    require(scope["default"] is None, "production date scope must remain explicit")
    for section, names in ((source, ("checksum_role", "url")), (licence, ("scope", "url")),
                           (history, ("semantics", "source_title", "source_url")),
                           (rule, ("semantics", "source_doi", "source_url")),
                           (reference, ("role", "url")), (scope, ("semantics",))):
        for name in names:
            value = text(section[name], name)
            if name.endswith("url"):
                require(value.startswith("https://"), "HTTPS source identity required")
    require(number(history["baseline_ntp_seconds"], "baseline") == BASELINE[0]
            and number(history["baseline_tai_minus_utc_seconds"], "baseline offset") == BASELINE[1],
            "reviewed baseline mismatch")
    complete = number(history["complete_before_ntp_seconds"], "history completeness")
    continuous = number(rule["effective_ntp_seconds"], "continuous rule")
    utc_identity(history["complete_before_utc"], complete)
    utc_identity(rule["effective_utc"], continuous)
    require(month_start(BASELINE[0]) and complete > BASELINE[0] and continuous > BASELINE[0],
            "invalid reviewed coverage or rule")
    raw_path = local_member(path.parent, source["path"])
    licence_path = local_member(path.parent, licence["path"])
    require(len({path, raw_path, licence_path}) == 3, "input roles must be distinct")
    raw = read_local(raw_path, RAW_LIMIT)
    licence_raw = read_local(licence_path, RAW_LIMIT)
    raw_sha = digest(source["sha256"], "raw SHA256")
    licence_sha = digest(licence["sha256"], "licence SHA256")
    require(hashlib.sha256(raw).hexdigest() == raw_sha
            and len(raw) == number(source["bytes"], "raw bytes", RAW_LIMIT), "raw input pin mismatch")
    require(hashlib.sha256(licence_raw).hexdigest() == licence_sha
            and len(licence_raw) == number(licence["bytes"], "licence bytes", RAW_LIMIT),
            "licence input pin mismatch")
    ledger = parse_leap_list(raw)
    lines = physical_lines(raw)
    require(len(lines) == number(source["physical_lines"], "physical lines", LINES_LIMIT)
            and max(map(len, lines), default=0) == number(source["maximum_line_bytes"], "maximum line", LINE_LIMIT)
            and len(ledger.records) == number(source["records"], "record count", RECORD_LIMIT),
            "raw input inventory mismatch")
    for key, actual in (("updated_ntp_seconds", ledger.updated),
                        ("expires_before_ntp_seconds", ledger.expires)):
        expected = source[key]
        require(expected is None or type(expected) is int, "invalid optional source timestamp")
        require(expected is None or 0 <= expected <= I64_MAX, "invalid optional source timestamp")
        require(expected == actual, "source timestamp metadata mismatch")
    expected_checksum = source["compatibility_checksum_sha1"]
    require(expected_checksum is None or digest(expected_checksum, "compatibility SHA1", 40) == expected_checksum,
            "invalid source checksum metadata")
    require(expected_checksum == ledger.checksum, "source checksum metadata mismatch")
    require(ledger.records[-1][0] < complete, "records extend beyond reviewed history")
    require(all(when < continuous for when, _ in ledger.records),
            "record conflicts with continuous rule")
    return Reviewed(ledger, complete, continuous, raw_sha,
                    hashlib.sha256(provenance_raw).hexdigest(), licence_sha,
                    (path, raw_path, licence_path))


def bounded_output(value: bytes) -> bytes:
    require(len(value) <= RAW_LIMIT, "generated output byte limit exceeded")
    return value


def render_json(value: Reviewed) -> bytes:
    first, *later = value.ledger.records
    offsets = value.ledger.records
    document = {
        "schema_version": 1,
        "baseline": {"effective_ntp_seconds": first[0], "tai_minus_utc_seconds": first[1]},
        "transitions": [{"effective_ntp_seconds": when, "tai_minus_utc_seconds": offset,
                         "delta_seconds": offset - offsets[i][1]}
                        for i, (when, offset) in enumerate(later)],
        "history_complete_before_ntp_seconds": value.history_complete_before,
        "finite_future_before_ntp_seconds": value.ledger.expires,
        "continuous_from_ntp_seconds": value.continuous_from,
        "source_metadata": {"updated_ntp_seconds": value.ledger.updated,
                            "compatibility_checksum_sha1": value.ledger.checksum},
        "identity": {"raw_sha256": value.raw_sha256, "provenance_sha256": value.provenance_sha256,
                     "licence_sha256": value.licence_sha256},
        "authority": "Local reviewed data identity only; no authentication, date authority or clock effect.",
    }
    return bounded_output((json.dumps(document, sort_keys=True, indent=2, allow_nan=False)+"\n").encode())


def render_rust(value: Reviewed) -> bytes:
    lines = ["//! Immutable reviewed leap-data inputs for checked knowledge construction.",
             "//!",
             "//! Generated by tools/leap-data/generate.py from complete local inputs.",
             "//! Record seconds use the NTP epoch and remain within signed-i64 bounds.",
             "//! Hashes establish identity only; no authentication or clock authority.",
             "/// Format revision of this generated data bundle.",
             "pub(super) const DATA_REVISION: u64 = 1;",
             "/// Effective NTP seconds and TAI minus UTC seconds, with the distinct baseline first.",
             "/// Subsequent records represent checked signed changes of exactly one second.",
             "pub(super) const LEAP_RECORDS: &[(u64, i32)] = &["]
    lines.extend(f"    ({when:_}, {offset:_})," for when, offset in value.ledger.records)
    lines.extend(["];", "/// Exclusive NTP seconds bound of explicitly reviewed complete history.",
                  "/// This review fact is independent of file expiry and the last transition.",
                  f"pub(super) const HISTORY_COMPLETE_BEFORE: u64 = {value.history_complete_before:_};"])
    future = "None" if value.ledger.expires is None else f"Some({value.ledger.expires:_})"
    lines.extend(["/// Optional exclusive NTP seconds endpoint of finite future coverage.",
                  "/// Absence means unknown coverage; expiry does not erase known history.",
                  f"pub(super) const FINITE_FUTURE_BEFORE: Option<u64> = {future};",
                  "/// Optional NTP seconds start of the reviewed rule permitting no subsequent leap events.",
                  "/// It creates no offset impulse and cannot fill an earlier unknown offset interval.",
                  f"pub(super) const CONTINUOUS_FROM: Option<u64> = Some({value.continuous_from:_});"])
    for name, hexadecimal, description in (
            ("RAW_SHA256", value.raw_sha256,
             "SHA256 identity of the complete raw source bytes; not authentication."),
            ("PROVENANCE_SHA256", value.provenance_sha256,
             "SHA256 identity of the manifest carrying review facts and limits; not authentication.")):
        lines.append("/// " + description)
        lines.append(f"pub(super) const {name}: [u8; 32] = [")
        data = bytes.fromhex(hexadecimal)
        for index in range(0, 32, 16):
            lines.append("    "+", ".join(f"0x{byte:02x}" for byte in data[index:index+16])+",")
        lines.append("];")
    return bounded_output(("\n".join(lines)+"\n").encode())


def atomic_write(path: Path, value: bytes) -> None:
    """Publish a complete same-directory candidate; failure removes its temp."""
    require(not path.is_symlink() and (not path.exists() or path.is_file()), "regular output required")
    temporary: Path | None = None
    try:
        with tempfile.NamedTemporaryFile(prefix="."+path.name+".", dir=path.parent, delete=False) as stream:
            temporary = Path(stream.name)
            stream.write(value)
            stream.flush()
            os.fsync(stream.fileno())
        os.replace(temporary, path)
        temporary = None
    finally:
        if temporary is not None:
            temporary.unlink(missing_ok=True)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--provenance", type=Path, default=HERE/"provenance.json")
    parser.add_argument("--format", choices=("json", "rust"), default="rust")
    parser.add_argument("--output", type=Path)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--check", action="store_true")
    mode.add_argument("--write", action="store_true")
    args = parser.parse_args(argv)
    try:
        value = load_reviewed(args.provenance)
        expected = render_rust(value) if args.format == "rust" else render_json(value)
        output = (args.output or HERE/("generated-data.rs" if args.format == "rust" else "generated-data.json")).absolute()
        require(output.parent.resolve(strict=True) == output.parent and not output.is_symlink(),
                "noncanonical output path")
        require(output not in value.protected_paths, "cannot replace a pinned input")
        if args.check:
            require(read_local(output, RAW_LIMIT) == expected, "generated output differs")
        else:
            atomic_write(output, expected)
        return 0
    except (GeneratorError, OSError, ValueError) as error:
        print("leap-data: "+str(error)[:240], file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
