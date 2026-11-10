"""Parse a bounded English-profile HOST systeminfo observation, without effects.

The caller owns Windows-host provenance, decoding and any resource-admission
floor. This module makes no Windows call and cannot establish current host or
WSL memory facts. Malformed, ambiguous or out-of-profile data supplies no count.
"""
from __future__ import annotations

import re

MAX_OUTPUT_BYTES = 65_536
MAX_LINE_BYTES = 1_024
MAX_MEMORY_MB = (1 << 64) - 1
_LABEL = re.compile(r"[ \t]*Available Physical Memory[ \t]*:")
_FIELD = re.compile(r"Available Physical Memory:[ \t]+([0-9](?:[0-9, \u00a0\u202f]*[0-9])?) MB")
_UNGROUPED = re.compile(r"0|[1-9][0-9]{0,19}")
_FIRST_GROUP = re.compile(r"[1-9][0-9]{0,2}")
_GROUP = re.compile(r"[0-9]{3}")
_SEPARATORS = frozenset((",", " ", "\u00a0", "\u202f"))


def _number_mb(number: str) -> int:
    """Admit canonical ASCII digits or one consistent three-digit separator."""
    digits = number
    if _UNGROUPED.fullmatch(number) is None:
        separators = set(number) & _SEPARATORS
        if len(separators) != 1:
            raise ValueError("Invalid HOST memory number")
        separator = next(iter(separators))
        groups = number.split(separator)
        if (_FIRST_GROUP.fullmatch(groups[0]) is None
                or any(_GROUP.fullmatch(group) is None for group in groups[1:])):
            raise ValueError("Invalid HOST memory grouping")
        digits = "".join(groups)
    if len(digits) > 20:
        raise ValueError("HOST memory number exceeds its integer range")
    value = int(digits)
    if value > MAX_MEMORY_MB:
        raise ValueError("HOST memory number exceeds its integer range")
    return value


def parse_available_physical_mb(output: str) -> int:
    """Return one nonnegative uint64 MB count from already-decoded HOST stdout.

    Accept LF/CRLF, the exact English field and integer MB, with either no
    thousands separator or consistent comma/space/NBSP/narrow-NBSP grouping.
    Refuse duplicate fields, unsupported syntax and UTF-8 extents above 64 KiB
    or 1,024 bytes per line. Errors contain no supplied output. No memory floor
    is applied: an observed 928 remains 928 for the caller's separate decision.
    """
    if type(output) is not str:
        raise TypeError("Expected decoded HOST systeminfo text")
    if len(output) > MAX_OUTPUT_BYTES:
        raise ValueError("HOST systeminfo output exceeds its byte budget")
    try:
        byte_count = len(output.encode("utf-8"))
    except UnicodeError:
        raise ValueError("Invalid decoded HOST systeminfo text") from None
    if byte_count > MAX_OUTPUT_BYTES:
        raise ValueError("HOST systeminfo output exceeds its byte budget")
    if any((ord(character) < 32 and character not in "\t\r\n")
           or character in "\x7f\x85\u2028\u2029" for character in output):
        raise ValueError("Unsupported HOST systeminfo line syntax")
    candidates: list[str] = []
    raw_lines = output.split("\n")
    for index, raw_line in enumerate(raw_lines):
        line = raw_line[:-1] if raw_line.endswith("\r") else raw_line
        if "\r" in line or (raw_line.endswith("\r") and index == len(raw_lines) - 1):
            raise ValueError("Unsupported HOST systeminfo line syntax")
        if len(line.encode("utf-8")) > MAX_LINE_BYTES:
            raise ValueError("HOST systeminfo line exceeds its byte budget")
        if _LABEL.match(line) is not None:
            candidates.append(line)
    if len(candidates) != 1:
        raise ValueError("Expected exactly one HOST available-physical-memory field")
    match = _FIELD.fullmatch(candidates[0])
    if match is None:
        raise ValueError("Invalid English HOST memory field")
    return _number_mb(match.group(1))
