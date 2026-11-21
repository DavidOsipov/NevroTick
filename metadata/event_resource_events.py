"""Read the commissioned SDK's two-event WEVT_TEMPLATE and message resource.

The WEVT offsets below describe the CRIM 5.1 shape observed in the admitted
Message Compiler output. The PE resource traversal and input custody live
outside this module.
"""

from __future__ import annotations

from struct import pack
from uuid import UUID

from .pipeline.contract import EventProvider, MetadataError, _validate_event_provider

_TEMPLATE_LEAF = ("WEVT_TEMPLATE", 1, 1033)
_MESSAGE_LEAF = (11, 1, 1033)
_MAX_LEAF_BYTES = 64 * 1024
_EVENT_KEYWORD = 0x8000000000000001
_NO_MESSAGE = 0xFFFFFFFF

# These are reader expectations for the selected W11-W08 event contract and
# the pinned MC profile. No generated header or intermediate sets expectations.
_EVENTS = {
    2: (4, 0xB0010002, "The Service Control Manager acknowledged Running."),
    4: (2, 0xB0010004, "The service recorded its first failure (cause %5)."),
}
_LEVEL_MESSAGES = {0x50000002: "Error", 0x50000004: "Information"}
_FIELDS = (
    ("Component", 4, "UInt8", 1),
    ("Sequence", 10, "UInt64", 8),
    ("ObservationKind", 4, "UInt8", 1),
    ("ObservedElapsedNs", 10, "UInt64", 8),
    ("Cause", 4, "UInt8", 1),
)
_PROVIDER_TABLES = {b"CHAN", b"TTBL", b"PRVA", b"OPCO", b"LEVL", b"TASK", b"KEYW", b"EVNT"}


def _require(condition: bool, detail: str) -> None:
    if not condition:
        raise MetadataError("compiled event resources: " + detail)


def _span(data: bytes, start: int, length: int) -> bytes:
    _require(0 <= start <= start + length <= len(data), "truncated or out-of-range record")
    return data[start:start + length]


def _uint(data: bytes, start: int, length: int = 4) -> int:
    return int.from_bytes(_span(data, start, length), "little")


def _terminated_utf16(encoded: bytes) -> str:
    try:
        value, terminator, padding = encoded.decode("utf-16-le").partition("\0")
    except UnicodeDecodeError as error:
        raise MetadataError("compiled event resources: invalid UTF-16") from error
    _require(bool(terminator) and not padding.strip("\0"), "invalid UTF-16 termination or padding")
    return value


def _sized_string(data: bytes, start: int, minimum: int) -> tuple[str, int]:
    _require(start >= minimum, "string overlaps its record")
    size = _uint(data, start)
    _require(6 <= size <= 512, "unsupported string size")
    return _terminated_utf16(_span(data, start + 4, size - 4)), start + size


def _provider_tables(data: bytes, provider: EventProvider) -> dict[bytes, tuple[int, int, int]]:
    size = _uint(data, 4)
    _require(_span(data, 0, 4) == b"CRIM"
             and (_uint(data, 8, 2), _uint(data, 10, 2), _uint(data, 12)) == (5, 1, 1)
             and 36 <= size <= len(data) <= size + 3 and not any(data[size:]),
             "expected a single CRIM 5.1 provider")
    _require(str(UUID(bytes_le=_span(data, 16, 16))) == provider.guid, "wrong provider GUID")
    root = _uint(data, 32)
    tables_count = _uint(data, root + 12)
    _require(root >= 36 and _span(data, root, 4) == b"WEVT"
             and root + _uint(data, root + 4) == size and _uint(data, root + 8) == _NO_MESSAGE
             and tables_count == len(_PROVIDER_TABLES), "invalid WEVT provider directory")

    tables: dict[bytes, tuple[int, int, int]] = {}
    directory_start = root + 20
    # In this profile the final directory offset immediately precedes the first table.
    directory_end = directory_start + 8 * (tables_count - 1) + 4
    for index in range(tables_count):
        start = _uint(data, directory_start + 8 * index)
        name = _span(data, start, 4)
        length = _uint(data, start + 4)
        count = _uint(data, start + 8)
        end = start + max(length, 12)
        _require(directory_end <= start < end <= size and name not in tables
                 and (length >= 12 or (length, count) == (0, 0)), "invalid WEVT table extent")
        tables[name] = (start, end, count)

    _require(tables.keys() == _PROVIDER_TABLES, "unexpected WEVT table inventory")
    spans = sorted(tables.values())
    _require(all(left[1] <= right[0] for left, right in zip(spans, spans[1:])),
             "overlapping WEVT tables")
    return tables


def _bin_xml_name(name: str) -> bytes:
    name_hash = 0
    for character in name:
        name_hash = (name_hash * 65599 + ord(character)) & 0xFFFF
    return pack("<HH", name_hash, len(name)) + (name + "\0").encode("utf-16-le")


def _event_data_fragment() -> bytes:
    """Construct the fixed EventData and five ordered scalar substitution slots."""
    # CRIM BinXML omits the name offsets used in an EVTX record. The inline
    # attribute value is a counted UTF-16 string.
    name_attribute = b"\x06" + _bin_xml_name("Name") + b"\x05\x01"
    children = bytearray()
    for index, (name, input_type, _, _) in enumerate(_FIELDS):
        attribute = name_attribute + pack("<H", len(name)) + name.encode("utf-16-le")
        body = (_bin_xml_name("Data") + pack("<I", len(attribute)) + attribute + b"\x02"
                + pack("<BHB", 0x0D, index, input_type) + b"\x04")
        children += pack("<BHI", 0x41, 0xFFFF, len(body)) + body
    event_data = _bin_xml_name("EventData") + b"\x02" + children + b"\x04"
    return b"\x0f\x01\x01\x00" + pack("<BHI", 1, 0xFFFF, len(event_data)) + event_data + b"\0"


def _verify_template(data: bytes, table: tuple[int, int, int]) -> int:
    start, end, count = table
    template = start + 12
    region = data[:end]
    _require(count == 1 and _span(region, template, 4) == b"TEMP"
             and template + _uint(region, template + 4) == end, "wrong event template")
    _require(_uint(region, template + 8) == _uint(region, template + 12) == len(_FIELDS)
             and _uint(region, template + 20) == 1, "wrong EventData template kind or field count")
    records = _uint(region, template + 16)
    bin_xml = _event_data_fragment()
    _require(records == template + 40 + len(bin_xml)
             and _span(region, template + 40, len(bin_xml)) == bin_xml,
             "wrong EventData field names, substitution order or types")

    names_start = records + 20 * len(_FIELDS)
    previous_name_end = names_start
    for index, (name, input_type, _, _) in enumerate(_FIELDS):
        record = _span(region, records + 20 * index, 20)
        _require(record[:16] == pack("<IBBHIHH", 0, input_type, input_type, 0, 0, 0, 0),
                 "wrong scalar event field type, flags, count or length")
        name_offset = _uint(record, 16)
        observed_name, previous_name_end = _sized_string(region, name_offset, previous_name_end)
        _require(observed_name == name, "wrong ordered event field name")
    _require(previous_name_end == end, "unexpected trailing EventData template bytes")
    return template


def _read_manifest(data: bytes, provider: EventProvider, messages: dict[int, str]
                   ) -> tuple[str, list[dict[str, object]]]:
    tables = _provider_tables(data, provider)
    start, end, count = tables[b"CHAN"]
    channel_record = start + 12
    region = data[:end]
    channel, channel_end = _sized_string(region, _uint(region, start + 16), start + 28)
    _require(count == 1 and _uint(region, start + 12) == 0
             and _uint(region, start + 20) == 16 and _uint(region, start + 24) == _NO_MESSAGE
             and channel == provider.name + "/Operational" and channel_end == end,
             "wrong compiled Operational channel")

    start, end, count = tables[b"PRVA"]
    region = data[:end]
    name_offset = _uint(region, start + 16)
    _require(count == 1 and _uint(region, start + 12) == 0x10000001 and name_offset >= start + 20
             and _terminated_utf16(_span(region, name_offset, end - name_offset)) == provider.name,
             "wrong compiled provider name")

    start, end, count = tables[b"KEYW"]
    keyword_record = start + 12
    region = data[:end]
    keyword, keyword_end = _sized_string(region, _uint(region, start + 24), start + 28)
    _require(count == 1 and _uint(region, start + 12, 8) == 1
             and _uint(region, start + 20) == _NO_MESSAGE and keyword == "ServiceLifecycle" and keyword_end == end,
             "wrong compiled ServiceLifecycle keyword")

    for table in (b"OPCO", b"TASK"):
        start, end, count = tables[table]
        _require((end - start, count) == (12, 0) and _uint(data, start + 4) == 0,
                 "unexpected compiled opcode or task records")

    start, end, count = tables[b"LEVL"]
    region = data[:end]
    _require(count == 2, "wrong compiled level count")
    levels: dict[int, int] = {}
    next_name = start + 36
    for index, (level, message_id, name) in enumerate(
            ((2, 0x50000002, "win:Error"), (4, 0x50000004, "win:Informational"))):
        record = start + 12 + 12 * index
        name_offset = _uint(region, record + 8)
        parsed_name, next_name = _sized_string(region, name_offset, next_name)
        _require(_uint(region, record) == level and _uint(region, record + 4) == message_id
                 and parsed_name == name, "wrong compiled level binding")
        levels[level] = record
    _require(next_name == end, "unexpected compiled level bytes")

    template = _verify_template(data, tables[b"TTBL"])
    start, end, count = tables[b"EVNT"]
    region = data[:end]
    records_end = start + 16 + 48 * len(_EVENTS)
    _require(count == len(_EVENTS) and _uint(region, start + 12) == 0
             and end == records_end + 4 * len(_EVENTS), "wrong compiled event table shape")
    events: dict[int, dict[str, object]] = {}
    for index in range(count):
        event = _span(region, start + 16 + 48 * index, 48)
        ident = _uint(event, 0, 2)
        _require(ident in _EVENTS and ident not in events, "unexpected or duplicate event ID")
        expected_level, expected_message_id, _ = _EVENTS[ident]
        version, event_channel, level, opcode = event[2:6]
        task = _uint(event, 6, 2)
        keyword_mask = _uint(event, 8, 8)
        message_id = _uint(event, 16)
        _require((version, event_channel, level, opcode, task) == (1, 16, expected_level, 0, 0),
                 "wrong event descriptor")
        _require(keyword_mask == _EVENT_KEYWORD, "wrong full event keyword mask")
        _require(message_id == expected_message_id, "wrong full event message ID")
        _require(_uint(event, 20) == template, "wrong event-to-template binding")
        keyword_pointer = records_end + 4 * index
        _require(_span(event, 24, 24) == pack("<6I", 0, levels[level], 0, 1,
                                              keyword_pointer, channel_record)
                 and _uint(region, keyword_pointer) == keyword_record,
                 "wrong compiled event record references")
        events[ident] = {"id": ident, "version": version, "channel": event_channel, "level": level,
                         "opcode": opcode, "task": task, "keyword": keyword_mask,
                         "message_id": message_id, "message": messages[message_id]}
    return channel, [events[ident] for ident in sorted(events)]


def _read_messages(data: bytes) -> dict[int, str]:
    """Read bounded Win32 MESSAGE_RESOURCE_DATA blocks and Unicode entries."""
    expected = _LEVEL_MESSAGES | {message_id: text for _, message_id, text in _EVENTS.values()}
    count = _uint(data, 0)
    _require(0 < count <= len(expected), "wrong message table block count")
    header_end = 4 + 12 * count
    values: dict[int, str] = {}
    ranges: list[tuple[int, int]] = []
    for index in range(count):
        row = 4 + 12 * index
        lower = _uint(data, row)
        upper = _uint(data, row + 4)
        start = _uint(data, row + 8)
        _require(0 <= upper - lower < len(expected) and header_end <= start < len(data),
                 "invalid message block range")
        cursor = start
        for message_id in range(lower, upper + 1):
            length = _uint(data, cursor, 2)
            _require(length >= 8 and length % 4 == 0 and _uint(data, cursor + 2, 2) == 1
                     and message_id in expected and message_id not in values,
                     "unexpected, duplicate or non-Unicode message entry")
            raw = _terminated_utf16(_span(data, cursor + 4, length - 4))
            _require(raw == expected[message_id] + "\r\n", "wrong compiled message text")
            values[message_id] = raw.removesuffix("\r\n")
            cursor += length
        ranges.append((start, cursor))
    end = header_end
    for start, next_end in sorted(ranges):
        _require(start == end, "overlapping or unclaimed message table bytes")
        end = next_end
    _require(end == len(data) and values.keys() == expected.keys(), "incomplete message table")
    return values


def verify_event_resources(
    provider: EventProvider, leaves: dict[tuple[int | str, int, int], bytes]
) -> dict[str, object]:
    """Verify linked resources against admitted provider identity and the selected event contract."""
    _validate_event_provider(provider)
    _require(leaves.keys() == {_TEMPLATE_LEAF, _MESSAGE_LEAF}, "wrong event resource leaves or language")
    template, messages = leaves[_TEMPLATE_LEAF], leaves[_MESSAGE_LEAF]
    _require(type(template) is bytes and type(messages) is bytes
             and 0 < len(template) <= _MAX_LEAF_BYTES and 0 < len(messages) <= _MAX_LEAF_BYTES,
             "invalid event resource byte lengths")
    message_table = _read_messages(messages)
    channel, events = _read_manifest(template, provider, message_table)
    return {
        "provider_guid": provider.guid,
        "channel": channel,
        "template_fields": [{"name": name, "type": type_name, "width": width}
                            for name, _, type_name, width in _FIELDS],
        "events": events,
        "message_ids": sorted(message_table),
    }
