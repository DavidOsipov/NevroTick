#!/usr/bin/env python3
"""Qualify pure NTP wire and pending NTS-KE negotiation analysis scopes.

No downloads, installations, host Cargo fallback or product dependency changes.
The full candidate is sealed; cargo-mutants mutates only its owned temporary copy.
The separate test-only libFuzzer graph requires exact source/native-grant admission.
Deterministic generated tests remain a separate evidence layer.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import shutil
import sys
import time
import tomllib

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[1]
PACKAGE = "ntp-wire"
HEADER = "crates/ntp-wire/src/header.rs"
DATAGRAM = "crates/ntp-wire/src/datagram.rs"
NTS_KE = "crates/ntp-wire/src/nts_ke.rs"
NTS_NEGOTIATION = "crates/ntp-wire/src/nts_negotiation.rs"
NTS_PACKET = "crates/ntp-wire/src/nts_packet.rs"
NTS_PLAINTEXT = "crates/ntp-wire/src/nts_plaintext.rs"
NTS_REQUEST = "crates/ntp-wire/src/nts_request.rs"
SUBJECTS = {"header": HEADER, "datagram": DATAGRAM, "nts_ke": NTS_KE,
            "nts_negotiation": NTS_NEGOTIATION, "nts_packet": NTS_PACKET, "nts_plaintext": NTS_PLAINTEXT,
            "nts_request": NTS_REQUEST}
# The extended source-admitted target compares all six pure owners with
# independent framing and request/response negotiation expectations.
FUZZ_SUBJECTS = {"header": HEADER, "datagram": DATAGRAM, "nts_ke": NTS_KE,
                 "nts_negotiation": NTS_NEGOTIATION, "nts_packet": NTS_PACKET, "nts_plaintext": NTS_PLAINTEXT}
FUZZ_CASES = ("healthy", "defect", "ke-defect", "negotiation-defect",
              "context-defect", "membership-defect", "packet-defect", "plaintext-defect")
POLICY = "policy/ntp-wire-analysis.json"
FUZZ_CARGO_CONFIG = ('[source.crates-io]\nreplace-with="fuzz-reviewed"\n'
                     '[source.fuzz-reviewed]\ndirectory="/work/fuzz-vendor"\n')


def load_module(name: str, path: Path):
    """Load reviewed repository tooling without running its CLI entry point."""
    specification = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


COMMON = load_module("wire_portable_shared", ROOT / "scripts/portable-analysis.py")
ANALYSIS, PRODUCT, PREPARE = COMMON.ANALYSIS, COMMON.PRODUCT, COMMON.PREPARE
RUNTIME, Incomplete = COMMON.RUNTIME, COMMON.Incomplete
SCOPE = COMMON.Scope(PACKAGE, tuple(SUBJECTS.items()), POLICY)


BOUND_INPUTS = (POLICY, "scripts/portable-analysis.py", "scripts/verify-wire-analysis.py",
                "scripts/prepare-product.py", "scripts/analysis-runtime.py", "scripts/check-analysis.py",
                "scripts/analysis-controls.py", "policy/analysis-tools.json", "policy/verification-manifest.toml")


def read_policy(root: Path) -> dict:
    """Reject changed scope, thresholds or resource caps before running tools."""
    policy = json.loads((root / POLICY).read_text())
    required = {"schema_version": 2, "package": PACKAGE, "critical_files": list(SUBJECTS.values()),
                "coverage_minimum_line_percent": 90, "mutation_minimum_viable_killed_percent": 95,
                "maximum_mutants": 512, "coverage_wall_seconds": 300,
                "mutation_wall_seconds": 1200, "mutation_build_seconds": 90,
                "mutation_test_seconds": 60, "jobs": RUNTIME.MAX_JOBS, "test_threads": RUNTIME.MAX_JOBS}
    if "critical_file" in policy or any(type(policy.get(key)) is not type(value) or policy.get(key) != value
           for key, value in required.items()):
        raise Incomplete("Unreviewed NTP analysis scope, threshold or budget.")
    fuzz = policy.get("coverage_guided_fuzzing", {})
    if fuzz.get("status") != "admitted-test-only-source" or fuzz.get("fuzzer_flags") != list(RUNTIME.WIRE_KE_SANITIZER_FLAGS):
        raise Incomplete("The fixed test-only libFuzzer source/profile admission is absent.")
    manifest = tomllib.loads((root / "policy/verification-manifest.toml").read_text())
    for filename in SUBJECTS.values():
        if manifest.get("critical_files", []).count(filename) != 1:
            raise Incomplete("A parser is absent or duplicated in the reviewed critical-file inventory.")
        if sum(module.get("files", []).count(filename) for module in manifest.get("security_modules", [])
               if module.get("package") == PACKAGE) != 1:
            raise Incomplete("A wire owner is absent or duplicated in the security-module inventory.")
    return policy


def select_subjects(subject: str) -> dict[str, str]:
    """Return only the fixed reviewed file mapping; all includes all seven owners."""
    if subject == "all":
        return dict(SUBJECTS)
    if subject not in SUBJECTS:
        raise Incomplete("Unknown NTP analysis subject.")
    return {subject: SUBJECTS[subject]}


def admit_fuzz_subject(lane: str, subject: str) -> None:
    """Admit only fixed source-reviewed fuzz owners and known lane selections.

    The admitted target covers six owners. The seventh formatter owner therefore
    closes fuzz/all until separately reviewed target admission; contracts,
    coverage and mutation remain selectable without extending fuzz authority.
    """
    if lane not in {"contracts", "coverage", "mutation", "fuzz", "all"} or subject not in {*SUBJECTS, "all"}:
        raise Incomplete("Unknown wire analysis lane or subject.")
    if lane in {"fuzz", "all"} and SUBJECTS != FUZZ_SUBJECTS:
        raise Incomplete("A selected wire owner lacks admitted fuzz target coverage.")


def selected_file(filename: str) -> str:
    """Delegate unchanged wire evidence to the explicit immutable scope."""
    return SCOPE.selected_file(filename)


def require_unchanged(root: Path, identity: dict[str, str]) -> None:
    """Reject changed, removed, linked or replaced admitted input bytes."""
    if any(PRODUCT.sha256(root / name) != expected for name, expected in identity.items()):
        raise Incomplete("Source, policy or analysis input changed during execution.")


def require_current_candidate(root: Path, identity: dict[str, str]) -> None:
    """Re-admit the complete candidate set so additions cannot evade old hashes."""
    if PRODUCT.admit_product(root)[1] != identity:
        raise Incomplete("The complete candidate input set changed during analysis.")


def coverage_counts(report: dict, expected_file: str) -> dict:
    """Delegate unchanged wire evidence to the explicit immutable scope."""
    return COMMON.coverage_counts(SCOPE, report, expected_file)


def mutation_inventory(items: list, maximum: int, expected_file: str = HEADER) -> dict[str, dict]:
    """Delegate unchanged wire evidence to the explicit immutable scope."""
    return COMMON.mutation_inventory(SCOPE, items, maximum, expected_file)


def mutation_counts(report: dict, expected: dict[str, dict], expected_file: str = HEADER) -> dict:
    """Delegate unchanged wire evidence to the explicit immutable scope."""
    return COMMON.mutation_counts(SCOPE, report, expected, expected_file)


def combined_mutant_count(inventories: dict[str, dict], maximum: int) -> int:
    """Delegate unchanged wire evidence to the explicit immutable scope."""
    return COMMON.combined_mutant_count(inventories, maximum)


def remaining_timeout(deadline: float, maximum: int) -> int:
    """Delegate unchanged wire evidence to the explicit immutable scope."""
    return COMMON.remaining_timeout(deadline, maximum)


def test_count(output: str) -> int:
    """Delegate unchanged wire evidence to the explicit immutable scope."""
    return COMMON.test_count(output)


def tool_path(root: Path, name: str) -> Path:
    """Select only the installed hash-pinned extension; never acquire a tool."""
    found = shutil.which(name)
    if not found:
        raise Incomplete(f"Missing installed analysis extension: {name}.")
    path = Path(found).resolve(strict=True)
    entry = next(item for item in json.loads((root / "policy/tool-manifest.json").read_text())["tools"]
                 if item["name"] == name)
    PRODUCT.ENV.require_admitted_binary(path, entry["binary_sha256"], name)
    return path


def privacy_error(result: dict, expected: str) -> None:
    """Require the intended compiler privacy diagnostic, not any compile failure."""
    codes = set(re.findall(r"error\[(E\d+)\]", result.get("output", "")))
    if (result.get("failure") or result.get("exit_code") == 0 or codes != {expected}):
        raise Incomplete("External consumer did not fail for its intended privacy restriction.")


def contract_examples() -> dict[str, tuple[str, str | None]]:
    """Keep independent legal, privacy and borrow specimens for all seven wire owners."""
    setup = ('let Ok(limits) = ntp_wire::FrameLimits::new(76, 1) else { return; }; '
             'let mut input = vec![0u8; 76]; input[51] = 28; ')
    frame = ('let Ok(frame) = ntp_wire::frame_no_legacy_mac_datagram(&input, limits) '
             'else { return; }; ')
    field = 'let Some(mut field) = frame.extensions().next() else { return; }; '
    examples = {
        "legal": (
            'fn main() { let Ok(header) = ntp_wire::decode_fixed_header(&[0; 48]) else { return; }; '
            'let _ = (header.version(), header.receive_timestamp().seconds()); '
            + setup + frame +
            'let _ = (frame.header().version(), frame.extension_count()); '
            'for field in frame.extensions() { let _ = (field.field_type(), field.bytes(), field.body()); } }', None),
        "raw_header_construction": (
            'fn main() { let _ = ntp_wire::RawHeader { bytes: [0; 48] }; }', "E0451"),
        "raw_header_mutation": (
            'fn main() { let Ok(mut header) = ntp_wire::decode_fixed_header(&[0; 48]) else { return; }; '
            'header.bytes[0] = 1; }', "E0616"),
        "timestamp_construction": (
            'fn main() { let _ = ntp_wire::NtpTimestamp { bytes: [0; 8] }; }', "E0451"),
        "frame_limits_construction": (
            'fn main() { let _ = ntp_wire::FrameLimits { max_datagram_bytes: 48, max_extension_fields: 0 }; }',
            "E0451"),
        "raw_datagram_construction": (
            'fn main() { let Ok(header) = ntp_wire::decode_fixed_header(&[0; 48]) else { return; }; '
            'let _ = ntp_wire::RawDatagram { header, tail: &[], count: 0 }; }', "E0451"),
        "raw_extension_construction": (
            'fn main() { let _ = ntp_wire::RawExtension { bytes: &[0; 28] }; }', "E0451"),
        "extension_fields_construction": (
            'fn main() { let _ = ntp_wire::ExtensionFields { remaining: &[] }; }', "E0451"),
        "frame_limits_mutation": (
            'fn main() { let Ok(mut limits) = ntp_wire::FrameLimits::new(48, 0) else { return; }; '
            'limits.max_extension_fields = 99; }', "E0616"),
        "raw_datagram_mutation": (
            'fn main() { ' + setup + frame.replace('Ok(frame)', 'Ok(mut frame)') + 'frame.count = 99; }',
            "E0616"),
        "raw_extension_mutation": (
            'fn main() { ' + setup + frame + field + 'field.bytes = &[]; }', "E0616"),
        "live_input_borrow_mutation": (
            'fn main() { ' + setup + frame + field + 'input[0] = 1; let _ = field.bytes(); }', "E0502"),
        "datagram_borrow_escape": (
            'fn escape() -> Result<ntp_wire::RawDatagram<\'static>, ntp_wire::FrameError> { '
            'let input = [0u8; 48]; let limits = ntp_wire::FrameLimits::new(48, 0)?; '
            'ntp_wire::frame_no_legacy_mac_datagram(&input, limits) } fn main() {}', "E0515"),
        "frame_limits_default": (
            'fn main() { let _: ntp_wire::FrameLimits = Default::default(); }', "E0277"),
        "raw_datagram_debug": (
            'fn main() { ' + setup + frame + 'let _ = format!("{frame:?}"); }', "E0277"),
        "raw_extension_debug": (
            'fn main() { ' + setup + frame + field + 'let _ = format!("{field:?}"); }', "E0277"),
        "extension_fields_debug": (
            'fn main() { ' + setup + frame
            + 'let fields = frame.extensions(); let _ = format!("{fields:?}"); }', "E0277"),
    }
    examples.update(nts_ke_contract_examples())
    examples.update(nts_negotiation_contract_examples())
    examples.update(nts_ke_accumulator_contract_examples())
    examples.update(nts_exporter_contract_examples())
    examples.update(nts_packet_contract_examples())
    examples.update(nts_plaintext_contract_examples())
    examples.update({name: (specimen["source"], specimen["code"])
                     for name, specimen in nts_request_contract_specs().items()})
    # Direct rustc consumers get no rustdoc main wrapper. This is structural
    # preflight only; the actual compiler still owns syntax/type diagnostics.
    if any(re.search(r"\bfn\s+main\s*\(", text) is None for text, _ in examples.values()):
        raise Incomplete("Direct external Rust consumer lacks an explicit main.")
    return examples

def nts_ke_contract_examples() -> dict[str, tuple[str, str | None]]:
    """Literal external type/privacy/borrow specimens; behavioral bytes have public tests."""
    return {
        'nts_ke_limits_construction': ('fn main() {\n    use ntp_wire::NtsKeFrameLimits;\n    let _ = NtsKeFrameLimits { max_message_bytes: 0, max_records: 0 };\n}', 'E0451'),
        'nts_ke_limits_default': ('fn main() {\n    use ntp_wire::NtsKeFrameLimits;\n    let _: NtsKeFrameLimits = Default::default();\n}', 'E0277'),
        'nts_ke_legal': ('use ntp_wire::{NtsKeFrameError, NtsKeFrameLimits, frame_nts_ke_message};\nfn main() -> Result<(), NtsKeFrameError> {\nlet bytes = [0x80, 0x00, 0x00, 0x00];\nlet mut records = {\n    let message = frame_nts_ke_message(&bytes, NtsKeFrameLimits::new(4, 1)?)?;\n    message.records()\n};\nlet record = records.next().ok_or(NtsKeFrameError::MissingEndOfMessage)?;\nassert_eq!(record.record_type(), 0);\nassert!(record.critical());\nassert!(record.body().is_empty());\nassert!(records.next().is_none());\nOk(())\n}', None),
        'nts_ke_message_construction': ('fn main() {\n    use ntp_wire::RawNtsKeMessage;\n    let _ = RawNtsKeMessage { bytes: &[], count: 0 };\n}', 'E0451'),
        'nts_ke_message_mutation': ('use ntp_wire::{NtsKeFrameError, NtsKeFrameLimits, frame_nts_ke_message};\nfn main() -> Result<(), NtsKeFrameError> {\nlet bytes = [0x80, 0x00, 0x00, 0x00];\nlet mut message = frame_nts_ke_message(&bytes, NtsKeFrameLimits::new(4, 1)?)?;\nmessage.count = usize::MAX;\nOk(())\n}', 'E0616'),
        'nts_ke_message_default': ("fn main() {\n    use ntp_wire::RawNtsKeMessage;\n    let _: RawNtsKeMessage<'static> = Default::default();\n}", 'E0277'),
        'nts_ke_message_borrow_escape': ('use ntp_wire::{NtsKeFrameError, NtsKeFrameLimits, frame_nts_ke_message};\nfn main() -> Result<(), NtsKeFrameError> {\nlet message;\n{\n    let bytes = [0x80, 0x00, 0x00, 0x00];\n    message = frame_nts_ke_message(&bytes, NtsKeFrameLimits::new(4, 1)?)?;\n}\nassert_eq!(message.record_count(), 1);\nOk(())\n}', 'E0597'),
        'nts_ke_live_input_mutation': ('use ntp_wire::{NtsKeFrameError, NtsKeFrameLimits, frame_nts_ke_message};\nfn main() -> Result<(), NtsKeFrameError> {\nlet mut bytes = [0x80, 0x00, 0x00, 0x00];\nlet message = frame_nts_ke_message(&bytes, NtsKeFrameLimits::new(4, 1)?)?;\nbytes[0] = 0x00;\nassert_eq!(message.bytes()[0], 0x80);\nOk(())\n}', 'E0506'),
        'nts_ke_message_debug': ("fn main() {\n    use ntp_wire::RawNtsKeMessage;\n    fn requires_debug<T: core::fmt::Debug>() {}\n    requires_debug::<RawNtsKeMessage<'static>>();\n}", 'E0277'),
        'nts_ke_record_construction': ('fn main() {\n    use ntp_wire::RawNtsKeRecord;\n    let _ = RawNtsKeRecord { bytes: &[] };\n}', 'E0451'),
        'nts_ke_record_borrow_escape': ('use ntp_wire::{NtsKeFrameError, NtsKeFrameLimits, frame_nts_ke_message};\nfn main() -> Result<(), NtsKeFrameError> {\nlet record;\n{\n    let bytes = [0x80, 0x00, 0x00, 0x00];\n    let message = frame_nts_ke_message(&bytes, NtsKeFrameLimits::new(4, 1)?)?;\n    record = message.records().next().ok_or(NtsKeFrameError::MissingEndOfMessage)?;\n}\nassert_eq!(record.record_type(), 0);\nOk(())\n}', 'E0597'),
        'nts_ke_record_debug': ("fn main() {\n    use ntp_wire::RawNtsKeRecord;\n    fn requires_debug<T: core::fmt::Debug>() {}\n    requires_debug::<RawNtsKeRecord<'static>>();\n}", 'E0277'),
        'nts_ke_iterator_construction': ('fn main() {\n    use ntp_wire::NtsKeRecords;\n    let _ = NtsKeRecords { remaining: &[] };\n}', 'E0451'),
        'nts_ke_iterator_borrow_escape': ('use ntp_wire::{NtsKeFrameError, NtsKeFrameLimits, frame_nts_ke_message};\nfn main() -> Result<(), NtsKeFrameError> {\nlet mut records;\n{\n    let bytes = [0x80, 0x00, 0x00, 0x00];\n    let message = frame_nts_ke_message(&bytes, NtsKeFrameLimits::new(4, 1)?)?;\n    records = message.records();\n}\nassert!(records.next().is_some());\nOk(())\n}', 'E0597'),
        'nts_ke_iterator_debug': ("fn main() {\n    use ntp_wire::NtsKeRecords;\n    fn requires_debug<T: core::fmt::Debug>() {}\n    requires_debug::<NtsKeRecords<'static>>();\n}", 'E0277'),
    }


def nts_negotiation_contract_examples() -> dict[str, tuple[str, str | None]]:
    """Compile literal legal and bypass callers at the pending negotiation API.

    These external specimens establish private construction, unchanged borrows
    and absence of payload defaults/debug traits. Actual protocol results remain
    the responsibility of independent public integration tests.
    """
    imports = (
        "use ntp_wire::{NtsKeFrameLimits, NtsKeNtpv4Offer, NtsKeNtpv4Negotiation, "
        "NtsKeCookies, frame_nts_ke_message, read_nts_ke_ntpv4_offer, negotiate_nts_ke_ntpv4};\n"
    )
    limits = "let limits = NtsKeFrameLimits::new(131_072, 32_768).unwrap();\n"
    request = (
        "let request = [0x80, 1, 0, 2, 0, 0, 0x80, 4, 0, 2, 0, 15, 0x80, 0, 0, 0];\n"
    )
    offered = (
        "let request_frame = frame_nts_ke_message(&request, limits).unwrap();\n"
        "let offer = read_nts_ke_ntpv4_offer(&request_frame).unwrap();\n"
    )
    response = (
        "let response = [0x80, 1, 0, 2, 0, 0, 0, 4, 0, 2, 0, 15, "
        "0, 5, 0, 1, 0xa5, 0x80, 0, 0, 0];\n"
    )
    negotiated = (
        "let response_frame = frame_nts_ke_message(&response, limits).unwrap();\n"
        "let negotiation = negotiate_nts_ke_ntpv4(&offer, response_frame).unwrap();\n"
    )

    def main(body: str) -> str:
        """Wrap a fixed external body; direct rustc supplies no rustdoc main."""
        return imports + "fn main() {\n" + body + "}\n"

    examples = {
        "nts_negotiation_legal": (main(
            limits + request + offered + response + negotiated
            + "assert_eq!(offer.request_bytes(), &request);\n"
            + "assert_eq!(offer.offered_aead_bytes(), &[0, 15]);\n"
            + "assert_eq!(negotiation.bytes(), &response);\n"
            + "assert_eq!(negotiation.aead_id(), 15);\n"
            + "assert_eq!(negotiation.cookie_count(), 1);\n"
            + "assert_eq!(negotiation.server_hint(), None);\n"
            + "assert_eq!(negotiation.port_hint(), None);\n"
            + "let mut cookies = negotiation.cookies();\n"
            + "assert_eq!(cookies.next(), Some(&[0xa5][..]));\n"
            + "assert!(cookies.next().is_none());\n"
            + "assert!(cookies.next().is_none());\n"), None),
        "nts_negotiation_offer_construction": (main(
            "let _ = NtsKeNtpv4Offer { request: &[], aeads: &[] };\n"), "E0451"),
        "nts_negotiation_offer_mutation": (main(
            limits + request + offered
            + "let mut offer = offer; offer.aeads = &[];\n"), "E0616"),
        "nts_negotiation_pending_construction": (main(
            limits + "let bytes = [0x80, 0, 0, 0];\n"
            + "let message = frame_nts_ke_message(&bytes, limits).unwrap();\n"
            + "let _ = NtsKeNtpv4Negotiation { message, aead_id: 15, cookie_count: 0, "
              "server_hint: None, port_hint: None };\n"), "E0451"),
        "nts_negotiation_pending_mutation": (main(
            limits + request + offered + response + negotiated
            + "let mut negotiation = negotiation; negotiation.cookie_count = 0;\n"), "E0616"),
        "nts_negotiation_cookies_construction": (main(
            limits + "let bytes = [0x80, 0, 0, 0];\n"
            + "let message = frame_nts_ke_message(&bytes, limits).unwrap();\n"
            + "let _ = NtsKeCookies { records: message.records() };\n"), "E0451"),
        "nts_negotiation_cookies_mutation": (main(
            limits + request + offered + response + negotiated
            + "let mut cookies = negotiation.cookies();\n"
            + "cookies.records = frame_nts_ke_message(&request, limits).unwrap().records();\n"), "E0616"),
        "nts_negotiation_offer_borrow_escape": (main(
            limits + "let offer;\n{\n" + request
            + "let frame = frame_nts_ke_message(&request, limits).unwrap();\n"
            + "offer = read_nts_ke_ntpv4_offer(&frame).unwrap();\n}\n"
            + "assert_eq!(offer.offered_aead_bytes(), &[0, 15]);\n"), "E0597"),
        "nts_negotiation_pending_borrow_escape": (main(
            limits + request + offered + "let negotiation;\n{\n" + response
            + "let frame = frame_nts_ke_message(&response, limits).unwrap();\n"
            + "negotiation = negotiate_nts_ke_ntpv4(&offer, frame).unwrap();\n}\n"
            + "assert_eq!(negotiation.cookie_count(), 1);\n"), "E0597"),
        "nts_negotiation_cookies_borrow_escape": (main(
            limits + request + offered + "let mut cookies;\n{\n" + response + negotiated
            + "cookies = negotiation.cookies();\n}\n"
            + "assert!(cookies.next().is_some());\n"), "E0597"),
        "nts_negotiation_request_live_mutation": (main(
            limits + request.replace("let request =", "let mut request =") + offered
            + "request[0] = 0;\nlet _ = offer.request_bytes();\n"), "E0506"),
        "nts_negotiation_response_live_mutation": (main(
            limits + request + offered
            + response.replace("let response =", "let mut response =") + negotiated
            + "response[0] = 0;\nlet _ = negotiation.cookies().next();\n"), "E0506"),
    }
    for owner, typename in (("offer", "NtsKeNtpv4Offer"),
                            ("pending", "NtsKeNtpv4Negotiation"), ("cookies", "NtsKeCookies")):
        examples["nts_negotiation_" + owner + "_default"] = (
            main("let _: " + typename + "<'static> = Default::default();\n"), "E0277")
        examples["nts_negotiation_" + owner + "_debug"] = (
            main("fn requires_debug<T: core::fmt::Debug>() {}\n"
                 + "requires_debug::<" + typename + "<'static>>();\n"), "E0277")
    return examples


def nts_ke_accumulator_contract_examples() -> dict[str, tuple[str, str | None]]:
    """Exercise the actual owner API without naming its private cursor type."""
    imports = (
        "use ntp_wire::{NtsKeAccumulator, NtsKeAccumulatorError, NtsKeFrameLimits, "
        "NtsKeProgress, RawNtsKeMessage};\n"
    )
    limits = "let limits = NtsKeFrameLimits::new(4, 1).unwrap();\n"
    storage = "let mut storage = [0_u8; 4];\n"
    owner = "let mut owner = NtsKeAccumulator::new(&mut storage, limits).unwrap();\n"
    complete = "owner.push(&[0x80, 0, 0, 0]).unwrap();\n"

    def main(body: str) -> str:
        """Supply an explicit direct-rustc entry point, not a rustdoc wrapper."""
        return imports + "fn main() {\n" + body + "}\n"

    return {
        "nts_accumulator_legal": (main(
            "let mut storage = [0xa5; 8];\n"
            + "let limits = NtsKeFrameLimits::new(8, 2).unwrap();\n"
            + owner
            + "assert_eq!(owner.push(&[]).unwrap(), NtsKeProgress::NeedMore);\n"
            + "assert_eq!(owner.push(&[0x80, 0]).unwrap(), NtsKeProgress::NeedMore);\n"
            + "assert_eq!(owner.push(&[0, 0]).unwrap(), NtsKeProgress::EndOfMessage);\n"
            + "let mut records = { let message = owner.finish().unwrap();\n"
            + "assert_eq!(message.bytes(), &[0x80, 0, 0, 0]);\n"
            + "assert_eq!(message.record_count(), 1); message.records() };\n"
            + "let record = records.next().unwrap();\n"
            + "assert_eq!(record.record_type(), 0); assert!(record.critical());\n"
            + "assert!(record.body().is_empty());\n"
            + "assert!(records.next().is_none()); assert!(records.next().is_none());\n"
        ), None),
        "nts_accumulator_construction": (main(
            limits + storage + owner
            + "let _ = NtsKeAccumulator { used: usize::MAX, ..owner };\n"
        ), "E0451"),
        "nts_accumulator_mutation": (main(
            limits + storage + owner + "owner.used = usize::MAX;\n"
        ), "E0616"),
        "nts_accumulator_default": (main(
            "let _: NtsKeAccumulator<'static> = Default::default();\n"
        ), "E0277"),
        "nts_accumulator_debug": (main(
            "fn requires_debug<T: core::fmt::Debug>() {}\n"
            + "requires_debug::<NtsKeAccumulator<'static>>();\n"
        ), "E0277"),
        "nts_accumulator_live_storage_mutation": (main(
            limits + storage + owner
            + "storage[0] = 0xff;\nlet _ = owner.push(&[]);\n"
        ), "E0506"),
        "nts_accumulator_borrow_escape": (main(
            limits + "let mut owner;\n{\n" + storage
            + "owner = NtsKeAccumulator::new(&mut storage, limits).unwrap();\n}\n"
            + "let _ = owner.push(&[]);\n"
        ), "E0597"),
        "nts_accumulator_output_escape": (
            imports
            + "fn escape() -> Result<RawNtsKeMessage<'static>, NtsKeAccumulatorError> {\n"
            + storage
            + "let limits = NtsKeFrameLimits::new(4, 1).map_err(NtsKeAccumulatorError::Frame)?;\n"
            + "let mut owner = NtsKeAccumulator::new(&mut storage, limits)?;\n"
            + "owner.push(&[0x80, 0, 0, 0])?;\nowner.finish()\n}\nfn main() {}\n",
            "E0515",
        ),
        "nts_accumulator_reuse_after_finish": (main(
            limits + storage + owner + complete
            + "let _message = owner.finish().unwrap();\nlet _ = owner.push(&[]);\n"
        ), "E0382"),
    }


def nts_exporter_contract_examples() -> dict[str, tuple[str, str | None]]:
    """Bind owned metadata and its borrowed getter to real negotiated literals."""
    setup = (
        "use ntp_wire::{NtsKeExporterContext, NtsKeExporterDirection, NtsKeFrameLimits, "
        "frame_nts_ke_message, read_nts_ke_ntpv4_offer, negotiate_nts_ke_ntpv4};\n"
        "fn make_context(direction: NtsKeExporterDirection) -> NtsKeExporterContext {\n"
        "let limits = NtsKeFrameLimits::new(64, 8).unwrap();\n"
        "let request = [0x80, 1, 0, 2, 0, 0, 0x80, 4, 0, 2, 0, 15, 0x80, 0, 0, 0];\n"
        "let response = [0x80, 1, 0, 2, 0, 0, 0, 4, 0, 2, 0, 15, 0, 5, 0, 0, 0x80, 0, 0, 0];\n"
        "let raw = frame_nts_ke_message(&request, limits).unwrap();\n"
        "let offer = read_nts_ke_ntpv4_offer(&raw).unwrap();\n"
        "let response = frame_nts_ke_message(&response, limits).unwrap();\n"
        "let pending = negotiate_nts_ke_ntpv4(&offer, response).unwrap();\n"
        "pending.exporter_context(direction)\n}\n"
    )

    def main(body: str) -> str:
        """Use an explicit direct consumer after the literal buffers have expired."""
        return setup + "fn main() {\n" + body + "}\n"

    return {
        "nts_exporter_legal": (main(
            "let c2s = make_context(NtsKeExporterDirection::ClientToServer);\n"
            + "let s2c = make_context(NtsKeExporterDirection::ServerToClient);\n"
            + "assert_eq!(c2s.label(), \"EXPORTER-network-time-security\");\n"
            + "assert_eq!(s2c.label(), \"EXPORTER-network-time-security\");\n"
            + "assert_eq!(c2s.context_bytes(), &[0, 0, 0, 15, 0]);\n"
            + "assert_eq!(s2c.context_bytes(), &[0, 0, 0, 15, 1]);\n"
        ), None),
        "nts_exporter_construction": (main(
            "let _ = NtsKeExporterContext { bytes: [0; 5] };\n"
        ), "E0451"),
        "nts_exporter_mutation": (main(
            "let mut context = make_context(NtsKeExporterDirection::ClientToServer);\n"
            + "context.bytes[4] = 1;\n"
        ), "E0616"),
        "nts_exporter_readonly_getter": (main(
            "let context = make_context(NtsKeExporterDirection::ClientToServer);\n"
            + "context.context_bytes()[4] = 1;\n"
        ), "E0594"),
        "nts_exporter_default": (main(
            "let _: NtsKeExporterContext = Default::default();\n"
        ), "E0277"),
        "nts_exporter_debug": (main(
            "fn requires_debug<T: core::fmt::Debug>() {}\n"
            + "requires_debug::<NtsKeExporterContext>();\n"
        ), "E0277"),
        "nts_exporter_getter_escape": (
            setup + "fn escape() -> &'static [u8; 5] {\n"
            + "let context = make_context(NtsKeExporterDirection::ClientToServer);\n"
            + "context.context_bytes()\n}\nfn main() {}\n",
            "E0515",
        ),
    }


def nts_packet_contract_examples() -> dict[str, tuple[str, str | None]]:
    """Exercise the public reader and private input-tied packet projection."""
    setup = (
        "use ntp_wire::{FrameLimits, NtsAuthenticatorParts, read_nts_authenticator_parts};\n"
        "fn input() -> [u8; 76] {\n"
        "let mut bytes = [0u8; 76];\n"
        "bytes[48..56].copy_from_slice(&[4, 4, 0, 28, 0, 1, 0, 3]);\n"
        "bytes[56] = 0x11; bytes[60..63].copy_from_slice(&[0x21, 0x22, 0x23]); bytes\n}\n"
    )
    def main(body: str) -> str:
        return setup + "fn main() {\n" + body + "}\n"
    admitted = ("let bytes = input(); let limits = FrameLimits::new(76, 1).unwrap();\n"
                "let view = read_nts_authenticator_parts(&bytes, limits).unwrap();\n")
    return {
        "nts_packet_legal": (main(admitted
            + "assert_eq!(view.associated_data(), &bytes[..48]);\n"
            + "assert_eq!(view.nonce(), &[0x11]); assert_eq!(view.ciphertext(), &[0x21, 0x22, 0x23]);\n"), None),
        "nts_packet_construction": (main(
            "let _ = NtsAuthenticatorParts { associated_data: &[], nonce: &[], ciphertext: &[] };\n"), "E0451"),
        "nts_packet_mutation": (main(admitted.replace("let view", "let mut view")
            + "view.nonce = &[];\n"), "E0616"),
        "nts_packet_readonly_getter": (main(admitted + "view.nonce()[0] = 0;\n"), "E0594"),
        "nts_packet_default": (main("let _: NtsAuthenticatorParts<'static> = Default::default();\n"), "E0277"),
        "nts_packet_debug": (main("fn debug<T: core::fmt::Debug>() {} debug::<NtsAuthenticatorParts<'static>>();\n"), "E0277"),
        "nts_packet_live_input_mutation": (main(
            "let mut bytes = input(); let limits = FrameLimits::new(76, 1).unwrap();\n"
            + "let view = read_nts_authenticator_parts(&bytes, limits).unwrap();\n"
            + "bytes[56] = 0; let _ = view.nonce();\n"), "E0506"),
        "nts_packet_borrow_escape": (setup
            + "fn escape() -> NtsAuthenticatorParts<'static> {\n"
            + "let bytes = input(); let limits = FrameLimits::new(76, 1).unwrap();\n"
            + "read_nts_authenticator_parts(&bytes, limits).unwrap()\n}\nfn main() {}\n", "E0515"),
    }


def nts_plaintext_contract_examples() -> dict[str, tuple[str, str | None]]:
    """External legal, privacy, immutable-input and borrow witnesses."""
    setup = ("use ntp_wire::{NtsPlaintextLimits, RawNtsPlaintext, RawNtsPlaintextField, "
             "NtsPlaintextFields, frame_nts_plaintext};\n")
    def main(body: str) -> str:
        return setup + "fn main() {\n" + body + "}\n"
    admitted = ("let bytes = [0x12, 0x34, 0, 4, 2, 4, 0, 8, 0xde, 0xad, 0xbe, 0xef];\n"
                "let frame = frame_nts_plaintext(&bytes, NtsPlaintextLimits::new(12, 2).unwrap()).unwrap();\n")
    return {
        "nts_plaintext_legal": (main(admitted + "assert_eq!(frame.field_count(), 2);\n"
            "let mut fields = frame.fields(); let first = fields.next().unwrap();\n"
            "assert_eq!(first.field_type(), 0x1234); assert!(first.body().is_empty());\n"
            "let second = fields.next().unwrap(); assert_eq!(second.bytes(), &bytes[4..]);\n"
            "assert_eq!(second.body(), &[0xde, 0xad, 0xbe, 0xef]); assert!(fields.next().is_none());\n"), None),
        "nts_plaintext_limits_construction": (main("let _ = NtsPlaintextLimits { max_bytes: 1, max_fields: 1 };\n"), "E0451"),
        "nts_plaintext_limits_mutation": (main("let mut limits = NtsPlaintextLimits::new(4, 1).unwrap(); limits.max_fields = 3;\n"), "E0616"),
        "nts_plaintext_limits_default": (main("let _: NtsPlaintextLimits = Default::default();\n"), "E0277"),
        "nts_plaintext_frame_construction": (main("let _ = RawNtsPlaintext { bytes: &[], count: 0 };\n"), "E0451"),
        "nts_plaintext_frame_mutation": (main(admitted.replace("let frame", "let mut frame") + "frame.count = 4;\n"), "E0616"),
        "nts_plaintext_frame_default": (main("let _: RawNtsPlaintext<'static> = Default::default();\n"), "E0277"),
        "nts_plaintext_frame_debug": (main("fn debug<T: core::fmt::Debug>() {} debug::<RawNtsPlaintext<'static>>();\n"), "E0277"),
        "nts_plaintext_field_construction": (main("let _ = RawNtsPlaintextField { bytes: &[0; 4] };\n"), "E0451"),
        "nts_plaintext_field_debug": (main("fn debug<T: core::fmt::Debug>() {} debug::<RawNtsPlaintextField<'static>>();\n"), "E0277"),
        "nts_plaintext_iterator_construction": (main("let _ = NtsPlaintextFields { remaining: &[] };\n"), "E0451"),
        "nts_plaintext_iterator_debug": (main("fn debug<T: core::fmt::Debug>() {} debug::<NtsPlaintextFields<'static>>();\n"), "E0277"),
        "nts_plaintext_readonly_body": (main(admitted + "let field = frame.fields().next().unwrap(); field.body()[0] = 1;\n"), "E0594"),
        "nts_plaintext_live_input_mutation": (main("let mut bytes = [0x12, 0x34, 0, 4];\n"
            "let frame = frame_nts_plaintext(&bytes, NtsPlaintextLimits::new(4, 1).unwrap()).unwrap();\n"
            "bytes[0] = 0; let _ = frame.fields().next();\n"), "E0506"),
        "nts_plaintext_borrow_escape": (setup + "fn escape() -> RawNtsPlaintext<'static> {\n"
            "let bytes = [0x12, 0x34, 0, 4];\n"
            "frame_nts_plaintext(&bytes, NtsPlaintextLimits::new(4, 1).unwrap()).unwrap()\n}\nfn main() {}\n", "E0515"),
    }



def nts_request_contract_specs() -> dict[str, dict]:
    """Closed literal formatter consumer and source-bound external bypass causes."""
    imports = ("use ntp_wire::{NtsRequestShape, PreparedNtsRequest, NtsClientRequest, "
               "prepare_nts_client_request, decode_fixed_header, FrameLimits};\n")
    def main(body: str) -> str:
        return imports + "fn main() {\n" + body + "\n}\n"
    setup = ("let mut header_bytes = [0u8; 48]; header_bytes[0] = 0x23;\n"
             "let header = decode_fixed_header(&header_bytes).expect(\"fixed header\");\n"
             "let uid = [0x11u8; 32]; let cookie = [0x22u8; 12];\n"
             "let shape = NtsRequestShape::new(3, 5, 16).expect(\"declared shape\");\n"
             "let limits = FrameLimits::new(148, 4).expect(\"literal limits\");\n"
             "let mut output = [0xa5u8; 160];\n")
    call = "prepare_nts_client_request(&header, &uid, &cookie, 1, shape, &mut output, limits).expect(\"valid request\")"
    prepared = setup + "let plan = " + call + ";\n"
    finish = ".finish(&[0xa1, 0xa2, 0xa3], &[0xc1, 0xc2, 0xc3, 0xc4, 0xc5]).expect(\"declared tail\")"
    # Independent fixed wire bytes: 48-byte header, UID36, Cookie16,
    # Placeholder16, Authenticator32. AAD ends at literal offset116.
    literal = ([0x23] + [0] * 47 + [1, 4, 0, 36] + [0x11] * 32
               + [2, 4, 0, 16] + [0x22] * 12 + [3, 4, 0, 16] + [0] * 12
               + [4, 4, 0, 32, 0, 3, 0, 5, 0xa1, 0xa2, 0xa3, 0,
                  0xc1, 0xc2, 0xc3, 0xc4, 0xc5] + [0] * 15)
    assert len(literal) == 148
    expected = ",".join(str(byte) for byte in literal)
    legal = ("let expected: [u8; 148] = [" + expected + "];\n" + setup
             + "let original = output.as_ptr();\n{\nlet plan = " + call + ";\n"
             + "assert_eq!(plan.associated_data(), &expected[..116]);\n"
             + "let packet = plan" + finish + ";\n"
             + "assert_eq!(packet.packet_bytes(), &expected);\n"
             + "assert_eq!(packet.packet_bytes().as_ptr(), original);\n}\n"
             + "assert_eq!(&output[148..], &[0xa5; 12]);\n"
             + 'println!("wire-nts-request:literal-prefix-finish");')
    specs = {"nts_request_legal": {"source": main(legal), "code": None,
                                     "success_marker": "wire-nts-request:literal-prefix-finish"}}
    def add(name: str, before: str, statement: str, code: str, tokens: list[str]) -> None:
        source = main(before + statement)
        specs["nts_request_" + name] = {"source": source, "code": code,
            "line": source.splitlines().index(statement) + 1, "tokens": tokens}
    add("shape_construction", "", "let _ = NtsRequestShape { nonce_length: 0, ciphertext_length: 0, nonce_width: 0, authenticator_length: 0 };", "E0451", ["NtsRequestShape", "private"])
    for field in ("nonce_length", "ciphertext_length", "nonce_width", "authenticator_length"):
        add("shape_" + field + "_mutation", "", "fn change(mut shape: NtsRequestShape) { shape." + field + " = 0; }", "E0616", [field, "private"])
    add("shape_default", "", "let _: NtsRequestShape = Default::default();", "E0277", ["NtsRequestShape", "Default"])
    add("plan_construction", "", "fn forge(output: &mut [u8], shape: NtsRequestShape) { let _ = PreparedNtsRequest { packet: output, aad_length: 0, shape }; }", "E0451", ["PreparedNtsRequest", "private"])
    for field, value, argument in (("packet", "replacement", "replacement: &'a mut [u8]"),
                                   ("aad_length", "0", "_unused: ()"),
                                   ("shape", "replacement", "replacement: NtsRequestShape")):
        add("plan_" + field + "_mutation", "", "fn change<'a>(mut plan: PreparedNtsRequest<'a>, " + argument + ") { plan." + field + " = " + value + "; }", "E0616", [field, "private"])
    add("plan_default", "", "let _: PreparedNtsRequest<'static> = Default::default();", "E0277", ["PreparedNtsRequest", "Default"])
    add("plan_clone", "", "fn needs_clone<T: Clone>() {} needs_clone::<PreparedNtsRequest<'static>>();", "E0277", ["PreparedNtsRequest", "Clone"])
    add("plan_debug", "", "fn needs_debug<T: core::fmt::Debug>() {} needs_debug::<PreparedNtsRequest<'static>>();", "E0277", ["PreparedNtsRequest", "Debug"])
    add("packet_construction", "", "let _ = NtsClientRequest { packet: &[] };", "E0451", ["NtsClientRequest", "private"])
    add("packet_mutation", "", "fn change<'a>(mut packet: NtsClientRequest<'a>, replacement: &'a [u8]) { packet.packet = replacement; }", "E0616", ["packet", "private"])
    add("packet_default", "", "let _: NtsClientRequest<'static> = Default::default();", "E0277", ["NtsClientRequest", "Default"])
    add("packet_debug", "", "fn needs_debug<T: core::fmt::Debug>() {} needs_debug::<NtsClientRequest<'static>>();", "E0277", ["NtsClientRequest", "Debug"])
    add("aad_readonly", "", "fn change(plan: PreparedNtsRequest<'_>) { plan.associated_data()[0] = 1; }", "E0594", ["cannot assign", "reference"])
    add("packet_readonly", "", "fn change(packet: NtsClientRequest<'_>) { packet.packet_bytes()[0] = 1; }", "E0594", ["cannot assign", "reference"])
    add("prepared_output_borrow", prepared, "output[0] = 1; let _ = plan.associated_data(); core::hint::black_box(output[0]);", "E0506", ["output", "borrow"])
    add("finished_output_borrow", prepared + "let packet = plan" + finish + ";\n", "output[0] = 1; let _ = packet.packet_bytes(); core::hint::black_box(output[0]);", "E0506", ["output", "borrow"])
    add("aad_blocks_finish", "", "fn overlap(plan: PreparedNtsRequest<'_>) { let aad = plan.associated_data(); let _ = plan" + finish + "; let _ = aad.len(); }", "E0505", ["plan", "borrow"])
    add("plan_consumed", "", "fn twice(plan: PreparedNtsRequest<'_>) { let _ = plan" + finish + "; let _ = plan" + finish + "; }", "E0382", ["plan", "moved"])
    local_setup = setup.replace("\n", " ")
    add("plan_escape", "", "fn escape() -> PreparedNtsRequest<'static> { " + local_setup + call + " }", "E0515", ["output"])
    add("packet_escape", "", "fn escape() -> NtsClientRequest<'static> { " + local_setup + call + finish + " }", "E0515", ["output"])
    add("aad_escape", "", "fn escape<'a>(plan: PreparedNtsRequest<'a>) -> &'a [u8] { plan.associated_data() }", "E0515", ["plan"])
    return specs


def request_privacy_error(result: dict, name: str, expected: str) -> None:
    """Bind only a closed new formatter specimen to its exact JSON compiler cause."""
    specimen = nts_request_contract_specs().get(name)
    if specimen is None or specimen["code"] is None or specimen["code"] != expected:
        raise Incomplete("Unknown or misbound NTS request compiler specimen.")
    COMMON.compiler_cause(result, specimen, "/work/contract-inputs/" + name + ".rs")

def fuzz_seeds() -> dict[str, bytes]:
    """Declare bounded literal word-framing neighbors independently of the parser."""
    def extension(field_type: int, length: int) -> bytes:
        """Encode one finite fixture field; lengths are this helper's trusted literals."""
        return field_type.to_bytes(2, "big") + length.to_bytes(2, "big") + bytes(length - 4)

    header = bytes(48)
    seeds = {"length-" + str(length): bytes(length) for length in (0, 1, 47, 48, 49, 4096)}
    seeds.update({"value-" + str(value): bytes([value]) * 48 for value in (0, 1, 127, 128, 255)})
    seeds["single-28"] = header + extension(0xabcd, 28)
    seeds["chain-16-28"] = header + extension(0x8001, 16) + extension(0x0404, 28)
    for count in (63, 64, 65):
        seeds["fields-" + str(count)] = header + extension(0x8001, 16) * (count - 1) + extension(0x0404, 28)
    seeds["malformed-final-16"] = header + extension(0xabcd, 16)
    seeds["malformed-good-prefix-tail"] = header + extension(0xabcd, 28) + b"\x00"
    for length in (4092, 4096):
        seeds["byte-edge-" + str(length)] = header + extension(0xabcd, length - 48)
    seeds["plaintext-literal"] = bytes.fromhex("12 34 00 04 02 04 00 08 de ad be ef")
    seeds["plaintext-late-short"] = bytes.fromhex("12 34 00 04 01 02 03")
    seeds["plaintext-unaligned"] = bytes.fromhex("12 34 00 05 00")
    seeds["plaintext-overrun"] = bytes.fromhex("12 34 00 08 aa bb")
    seeds["plaintext-high-length"] = bytes.fromhex("12 34 01 00") + bytes([0x5a]) * 252
    seeds["plaintext-maximum-fields"] = bytes.fromhex("12 34 00 04") * 1024
    # Independent literal packet envelopes and malformed neighbors.
    prefix = bytes([0xa5]) * 48 + bytes.fromhex("01040010808182838485868788898a8b")
    auth = bytes.fromhex("0404001c0001000311aabbcc212223dd9192939495969798999a9b9c")
    seeds["packet-literal-nonzero"] = prefix + auth
    seeds["packet-missing"] = bytes(48)
    seeds["packet-exact-fit"] = bytes(48) + bytes.fromhex("0404001c00040010") + bytes(range(20))
    seeds["packet-one-over"] = bytes(48) + bytes.fromhex("0404001c00040011") + bytes(range(20))
    seeds["packet-u16-extreme"] = bytes(48) + bytes.fromhex("0404001cffff0001") + bytes(20)
    seeds["packet-second-auth"] = prefix + auth + auth
    seeds["packet-malformed-suffix"] = prefix + auth + b"\x00"
    seeds.update(ke_fuzz_seeds())
    seeds.update(negotiation_fuzz_seeds())
    seeds.update(negotiation_pair_fuzz_seeds())
    # A separate capacity octet permits successful low-valued AEAD identifiers.
    # Keep exact-fit and one-byte-short witnesses in the actual fuzz corpus.
    seeds.update({"ke-encode-aead15": bytes.fromhex("10 00 0f"),
                  "ke-encode-aead16": bytes.fromhex("10 00 10"),
                  "ke-encode-aead15-16": bytes.fromhex("12 00 0f 00 10"),
                  "ke-encode-short": bytes.fromhex("0f 00 0f"),
                  "ke-encode-empty": bytes.fromhex("30")})
    # Eight records fit well below 64 bytes, separating the two stream budgets.
    seeds["ke-stream-record-limit"] = bytes.fromhex("00 2a 00 00") * 7 + bytes.fromhex("80 00 00 00")
    seeds["ke-stream-record-excess"] = bytes.fromhex("00 2a 00 00") * 8 + bytes.fromhex("80 00 00 00")
    return seeds


def ke_fuzz_seeds() -> dict[str, bytes]:
    """Literal body-only envelopes, independent of product serialization."""
    end = bytes.fromhex("80 00 00 00")
    seeds = {"ke-eom": end, "ke-noncritical-eom": bytes(4),
             "ke-missing-body": bytes.fromhex("12 34 ff ff"),
             "ke-eom-tail": end + b"x", "ke-short-header": b"\x80\x00\x00",
             "ke-odd-critical": bytes.fromhex("92 34 00 03 aa bb cc") + end,
             "ke-three-records": bytes.fromhex("12 34 00 00 12 35 00 00") + end}
    for total, low in ((65_535, 0xf7), (65_536, 0xf8), (65_537, 0xf9)):
        seeds["ke-byte-" + str(total)] = bytes([0x12, 0x34, 0xff, low]) + b"Z" * (total - 8) + end
    seeds["ke-max-body"] = bytes.fromhex("12 34 ff ff") + b"Z" * 65_535 + end
    seeds["ke-max-body-truncated"] = seeds["ke-max-body"][:65_538]
    seeds["ke-complete-body-no-eom"] = seeds["ke-max-body"][:65_539]
    seeds["ke-max-records"] = bytes.fromhex("12 34 00 00") * 32_767 + end
    seeds["ke-nonempty-eom"] = bytes.fromhex("80 00 00 01 5a")
    seeds["ke-duplicate-eom"] = end + end
    for total, low in ((131_071, 0xf4), (131_072, 0xf5), (131_073, 0xf6)):
        seeds["ke-cap-" + str(total)] = (bytes.fromhex("12 34 ff ff") + b"Z" * 65_535
            + bytes([0x12, 0x35, 0xff, low]) + b"Z" * (total - 65_547) + end)
    return seeds


def negotiation_fuzz_seeds() -> dict[str, bytes]:
    """Independent literal negotiation facts, with no production serialization."""
    end = bytes.fromhex("80 00 00 00")
    protocol = bytes.fromhex("80 01 00 02 00 00")
    aead15 = bytes.fromhex("00 04 00 02 00 0f")
    aead16 = bytes.fromhex("80 04 00 02 00 10")
    cookie = bytes.fromhex("00 05 00 01 a5")
    request = protocol + bytes.fromhex("00 04 00 04 00 0f 00 10")
    response = protocol + aead15 + cookie
    port = bytes.fromhex("00 07 00 02 00 7b")
    server = bytes.fromhex("00 06 00 0c") + b"time.example"
    seeds = {
        "ke-neg-request": request + end,
        "ke-neg-request-critical-aead": protocol + bytes.fromhex("80 04 00 04 00 0f 00 10") + end,
        "ke-neg-response15": response + end,
        "ke-neg-response16-critical": protocol + aead16 + bytes.fromhex("80 05 00 01 a5") + end,
        "ke-neg-empty-cookie": protocol + aead15 + bytes.fromhex("00 05 00 00") + end,
        "ke-neg-two-cookies": response + bytes.fromhex("80 05 00 00") + end,
        "ke-neg-server-port": response + server + port + end,
        "ke-neg-zero-port": response + bytes.fromhex("80 07 00 02 00 00") + end,
        "ke-neg-v6-hint": response + bytes.fromhex("80 06 00 0b") + b"2001:db8::1" + end,
        "ke-neg-unknown-noncritical": response + bytes.fromhex("12 34 00 03 00 81 ff") + end,
        "ke-neg-unknown-critical": response + bytes.fromhex("92 34 00 03 00 81 ff") + end,
        "ke-neg-error": bytes.fromhex("80 02 00 02 00 01") + end,
        "ke-neg-error-with-negotiation": response + bytes.fromhex("80 02 00 02 00 02") + end,
        "ke-neg-warning": response + bytes.fromhex("80 03 00 02 ff ff") + end,
        "ke-neg-refusal": bytes.fromhex("80 01 00 00") + end,
        "ke-neg-empty-aead": protocol + bytes.fromhex("00 04 00 00") + end,
        "ke-neg-unoffered-aead": protocol + bytes.fromhex("00 04 00 02 00 11") + cookie + end,
        "ke-neg-duplicate-protocol": protocol + response + end,
        "ke-neg-duplicate-aead": response + aead15 + end,
        "ke-neg-duplicate-server": response + server + server + end,
        "ke-neg-duplicate-port": response + port + port + end,
        "ke-neg-noncritical-protocol": bytes.fromhex("00 01 00 02 00 00") + aead15 + cookie + end,
        "ke-neg-odd-protocol": bytes.fromhex("80 01 00 01 00") + aead15 + cookie + end,
        "ke-neg-wide-response-aead": request + cookie + end,
        "ke-neg-odd-aead": protocol + bytes.fromhex("00 04 00 01 0f") + cookie + end,
        "ke-neg-bad-error": response + bytes.fromhex("00 02 00 02 00 01") + end,
        "ke-neg-short-warning": response + bytes.fromhex("80 03 00 01 00") + end,
        "ke-neg-short-port": response + bytes.fromhex("00 07 00 01 7b") + end,
        "ke-neg-empty-server": response + bytes.fromhex("00 06 00 00") + end,
        "ke-neg-bad-server": response + bytes.fromhex("00 06 00 01 25") + end,
        "ke-neg-missing-cookie": protocol + aead15 + end,
        "ke-neg-request-cookie": request + cookie + end,
        "ke-neg-max-cookie": protocol + aead15 + bytes.fromhex("00 05 ff ff") + b"Z" * 65_535 + end,
        "ke-neg-max-offer": protocol + bytes.fromhex("00 04 ff fe 00 0f") + bytes.fromhex("00 10") * 32_766 + end,
        "ke-neg-semantic-max-records": protocol + aead15 + bytes.fromhex("00 05 00 00")
            + bytes.fromhex("12 34 00 00") * 32_763 + end,
    }
    # Each positive boundary remains semantically complete: padding is an ignored
    # noncritical record. The one-above input is rejected by the existing byte cap.
    for total in (65_535, 65_536, 65_537, 131_071, 131_072, 131_073):
        remaining = total - len(response) - len(end)
        padding = bytearray()
        while remaining:
            length = min(remaining - 4, 65_535)
            padding.extend(bytes.fromhex("12 34") + length.to_bytes(2, "big") + b"Z" * length)
            remaining -= length + 4
        seeds["ke-neg-byte-" + str(total)] = response + bytes(padding) + end
    return seeds


def negotiation_pair_fuzz_seeds() -> dict[str, bytes]:
    """Literal two-message facts; encoding uses no product parser/serializer.

    Four big-endian request-length bytes precede the original request/response.
    Raw seeds above retain the full per-message boundaries; these pairs retain
    the unchanged aggregate 131,073-byte ceiling, including the adapter prefix.
    """
    end = bytes.fromhex("80 00 00 00")
    protocol = bytes.fromhex("80 01 00 02 00 00")
    cookie = bytes.fromhex("00 05 00 00")

    def record(kind: int, body: bytes) -> bytes:
        """Encode only trusted bounded fixture literals, never product output."""
        return kind.to_bytes(2, "big") + len(body).to_bytes(2, "big") + body

    def request(*identifiers: int) -> bytes:
        """Keep preference order, complete words, duplicates and zero unchanged."""
        return protocol + record(4, b"".join(word.to_bytes(2, "big") for word in identifiers)) + end

    def response(identifier: int) -> bytes:
        """A fixed ordinary selection with one opaque empty cookie."""
        return protocol + record(4, identifier.to_bytes(2, "big")) + cookie + end

    def pair(offered: bytes, selected: bytes) -> bytes:
        """Bind both original slices and their declared split within the aggregate cap."""
        data = len(offered).to_bytes(4, "big") + offered + selected
        if len(data) > 131_073:
            raise Incomplete("A relational fuzz seed exceeds the fixed aggregate input cap.")
        return data

    offered = request(0xabcd, 0x1234)
    selected = response(0x1234)
    # This whole literal is independently asserted in the target's adapter and
    # Python controls. The non-first full-width ID distinguishes both new faults.
    literal = bytes.fromhex(
        "00 00 00 12 80 01 00 02 00 00 00 04 00 04 ab cd 12 34 80 00 00 00 "
        "80 01 00 02 00 00 00 04 00 02 12 34 00 05 00 00 80 00 00 00")
    seeds = {
        "ke-pair-literal-nonfirst": literal,
        "ke-pair-same-response-disjoint": pair(request(0xabcd), selected),
        "ke-pair-same-response-overlap": pair(request(0, 0xabcd, 0x1234), selected),
        "ke-pair-reordered": pair(request(0x1234, 0xabcd), selected),
        "ke-pair-duplicate-identifiers": pair(request(0xabcd, 0x1234, 0x1234), selected),
        "ke-pair-critical-aead": pair(protocol + record(0x8004, bytes.fromhex("ab cd 12 34")) + end,
                                      protocol + record(0x8004, bytes.fromhex("12 34")) + cookie + end),
        "ke-pair-repeated-protocol-zero": pair(record(0x8001, bytes(4)) + offered[6:],
                                               record(0x8001, bytes(4)) + selected[6:]),
        "ke-pair-zero": pair(request(1, 0), response(0)),
        "ke-pair-unoffered-zero": pair(request(1), response(0)),
        "ke-pair-full-width": pair(request(0xffff, 0x0100), response(0x0100)),
        "ke-pair-low-byte-collision": pair(request(0x1234), response(0x0034)),
        "ke-pair-byte-overlap": pair(request(0x0012, 0x3400), selected),
        "ke-pair-reversed-word": pair(offered, response(0x3412)),
        "ke-pair-zero-request": bytes(4) + selected,
        "ke-pair-zero-response": pair(offered, b""),
        "ke-pair-short-prefix": bytes(3),
        "ke-pair-overwide-split": bytes.fromhex("ff ff ff ff") + offered + selected,
        "ke-pair-empty-offer": pair(request(), selected),
        "ke-pair-refusal": pair(offered, bytes.fromhex("80 01 00 00") + end),
        "ke-pair-no15-empty-aead": pair(offered, protocol + bytes.fromhex("00 04 00 00") + end),
        "ke-pair-offered15-empty-aead": pair(request(15, 0x1234), protocol + bytes.fromhex("00 04 00 00") + end),
        "ke-pair-missing-cookie": pair(offered, protocol + record(4, bytes.fromhex("12 34")) + end),
        "ke-pair-two-cookies": pair(offered, selected[:-4] + bytes.fromhex("80 05 00 00") + end),
        "ke-pair-request-duplicate-aead": pair(offered[:-4] + record(4, bytes.fromhex("12 34")) + end, selected),
        "ke-pair-response-duplicate-aead": pair(offered, selected[:-4] + record(4, bytes.fromhex("12 34")) + end),
        "ke-pair-request-odd-aead": pair(protocol + record(4, b"\x12") + end, selected),
        "ke-pair-response-odd-aead": pair(offered, protocol + record(4, b"\x12") + cookie + end),
        "ke-pair-duplicate-protocol": pair(offered, protocol + selected),
        "ke-pair-request-cookie": pair(offered[:-4] + cookie + end, selected),
        "ke-pair-request-unknown-critical": pair(offered[:-4] + record(0x9234, b"x") + end, selected),
        "ke-pair-response-unknown-critical": pair(offered, selected[:-4] + record(0x9234, b"x") + end),
        "ke-pair-unknown-noncritical": pair(offered[:-4] + record(0x1234, b"x") + end, selected),
        "ke-pair-error": pair(offered, bytes.fromhex("80 02 00 02 00 01") + end),
        "ke-pair-warning": pair(offered, selected[:-4] + bytes.fromhex("80 03 00 02 ff ff") + end),
        "ke-pair-malformed-after-error": pair(offered, bytes.fromhex("80 02 00 02 00 01 00 07 00 01 00") + end),
        "ke-pair-max-offer": pair(protocol + record(4, bytes.fromhex("00 0f")
            + bytes.fromhex("00 10") * 32_765 + bytes.fromhex("ab cd")) + end, response(0xabcd)),
    }
    for total in (131_071, 131_072, 131_073):
        remaining = total - len(literal)
        padding = bytearray()
        while remaining:
            length = min(remaining - 4, 65_535)
            padding.extend(record(0x1234, b"Z" * length))
            remaining -= length + 4
        seeds["ke-pair-cap-" + str(total)] = pair(offered, selected[:-4] + bytes(padding) + end)
    return seeds


def fuzz_case_options(case: str) -> tuple[list[str], str | None]:
    """Return one fixed test-only feature and its intended reached fault marker."""
    cases: dict[str, tuple[list[str], str | None]] = {
        "healthy": ([], None),
        "defect": (["--features", "seeded-defect"], "independent datagram field-count oracle"),
        "ke-defect": (["--features", "seeded-ke-defect"], "independent NTS-KE record-count oracle"),
        "negotiation-defect": (["--features", "seeded-negotiation-defect"],
                               "independent NTS-KE negotiation cookie-count oracle"),
        "context-defect": (["--features", "seeded-context-defect"],
                           "independent NTS-KE request-context forwarding oracle"),
        "membership-defect": (["--features", "seeded-membership-defect"],
                              "independent NTS-KE full-list membership observation oracle"),
        "packet-defect": (["--features", "seeded-packet-defect"],
                          "independent NTS packet span observation oracle"),
        "plaintext-defect": (["--features", "seeded-plaintext-defect"],
                             "independent NTS plaintext field-count observation oracle"),
    }
    if type(case) is not str or case not in cases:
        raise Incomplete("Unknown bounded wire fuzz control.")
    return cases[case]


def fuzz_execution_argv(binary: str, case: str) -> list[str]:
    """Route relational and packet harness faults through their exact literal witnesses.

    Healthy coverage-guided execution retains the full corpus and 256-run profile.
    All seeded faults are harness checks, not product mutants or ASan mechanism faults.
    """
    fuzz_case_options(case)
    stimulus = ("/work/fuzz-corpus/ke-pair-literal-nonfirst"
                if case in {"context-defect", "membership-defect"} else
                "/work/fuzz-corpus/packet-literal-nonzero" if case == "packet-defect" else
                "/work/fuzz-corpus/plaintext-literal" if case == "plaintext-defect" else "/work/fuzz-corpus")
    return [binary, stimulus, "-artifact_prefix=/work/fuzz-artifacts/", *RUNTIME.WIRE_KE_SANITIZER_FLAGS]


def fuzz_control_witness(corpus: Path, case: str, identity: dict[str, str]) -> Path | None:
    """Recheck the named stimulus after earlier corpus-writing runs."""
    fuzz_case_options(case)
    if case not in {"context-defect", "membership-defect", "packet-defect", "plaintext-defect"}:
        return None
    name = ("plaintext-literal" if case == "plaintext-defect" else
            "packet-literal-nonzero" if case == "packet-defect" else "ke-pair-literal-nonfirst")
    witness = corpus / name
    if PRODUCT.sha256(witness) != identity.get(name):
        raise Incomplete("The literal fuzz witness changed before its control.")
    return witness


def fuzz_metadata(metadata: dict, policy: dict) -> dict:
    """Check exact GNU-reachable feature/build closure before dependency code runs."""
    packages = {item["id"]: item for item in metadata["packages"]}
    nodes = {item["id"]: item for item in metadata["resolve"]["nodes"]}
    reachable, pending = set(), [metadata["resolve"]["root"]]
    while pending:
        key = pending.pop()
        if key in reachable:
            continue
        if key not in nodes or key not in packages:
            raise Incomplete("Fuzz metadata has an unresolved graph node.")
        reachable.add(key)
        pending.extend(item["pkg"] for item in nodes[key]["deps"])
    selected = policy["coverage_guided_fuzzing"]
    expected = {**selected["gnu_reachable_features"], "ntp-wire": [], "ntp-wire-fuzz": []}
    actual = {packages[key]["name"]: sorted(nodes[key]["features"]) for key in reachable}
    if len(actual) != len(reachable) or actual != expected:
        raise Incomplete("The resolved GNU fuzz feature closure differs from its exact admission.")
    versions = {item["name"]: item["version"] for item in selected["locked_registry_packages"]}
    if any(packages[key]["version"] != versions.get(packages[key]["name"], "0.0.0")
           for key in reachable):
        raise Incomplete("The resolved fuzz graph has an unreviewed package version.")
    return {"target": "x86_64-unknown-linux-gnu", "reachable_features": actual}


def admit_fuzz_sources(base: Path, policy: dict) -> None:
    """Check archive/source maps, every native file and its complete selected notice."""
    selected = policy["coverage_guided_fuzzing"]
    if PRODUCT.sha256(ROOT / selected["llvm_notice"]) != selected["llvm_notice_sha256"]:
        raise Incomplete("The selected complete LLVM-exception notice changed.")
    if PRODUCT.sha256(ROOT / selected["target_source"]) != selected["target_source_sha256"]:
        raise Incomplete("The independent fuzz target changed without source admission.")
    control = base / "common/controls-source/fuzz/Cargo.lock"
    if PRODUCT.sha256(control) != selected["control_lock_sha256"]:
        raise Incomplete("The admitted libFuzzer control graph changed.")
    locked = [item for item in tomllib.loads(control.read_text())["package"] if "source" in item]
    recorded = selected["locked_registry_packages"]
    if [{key: item[key] for key in ("name", "version", "source", "checksum")} for item in locked] != [
            {key: item[key] for key in ("name", "version", "source", "checksum")} for item in recorded]:
        raise Incomplete("The full locked fuzz registry graph differs from admission.")
    for item in recorded:
        name = item["name"] + "-" + item["version"]
        sources = list((base / "common/cargo/registry/src").glob("*/" + name))
        archives = list((base / "common/cargo/registry/cache").glob("*/" + name + ".crate"))
        if len(sources) != 1 or len(archives) != 1 or PRODUCT.sha256(archives[0]) != item["checksum"]:
            raise Incomplete("Fuzz package source/archive identity is absent or ambiguous.")
        files = PRODUCT.inventory(sources[0])
        digest = hashlib.sha256(json.dumps(files, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
        manifest = tomllib.loads((sources[0] / "Cargo.toml").read_text())["package"]
        if (digest != item["source_files_sha256"] or manifest.get("license") != item["declared_license"]
                or manifest.get("build", False) != item["build_script"]):
            raise Incomplete("Fuzz package files, declared grant or build owner changed.")
        if item["name"] == "libfuzzer-sys":
            native = PRODUCT.inventory(sources[0] / "libfuzzer")
            if native != {name: entry["sha256"] for name, entry in selected["native_files"].items()}:
                raise Incomplete("The complete pinned native libFuzzer source differs.")


def fuzz_lock(control: str) -> str:
    """Retain exact registry entries; change only the two owned local harness nodes."""
    blocks = control.split("[[package]]")
    packages = {}
    for block in blocks[1:]:
        item = tomllib.loads("[[package]]" + block)["package"][0]
        if "source" in item:
            packages[item["name"]] = "[[package]]\n" + block.strip() + "\n"
    packages["ntp-wire"] = '[[package]]\nname = "ntp-wire"\nversion = "0.0.0"\n'
    packages["ntp-wire-fuzz"] = ('[[package]]\nname = "ntp-wire-fuzz"\nversion = "0.0.0"\n'
                                     'dependencies = ["libfuzzer-sys", "ntp-wire"]\n')
    return "version = 4\n\n" + "\n".join(packages[name] for name in sorted(packages))


def fuzz_input_snapshot(work: Path, inputs: list[Path]) -> dict:
    """Admit only the exact owned fuzz Cargo config, retaining ancestry guards.

    The bootstrap snapshot helper deliberately forbids every Cargo-home config;
    this product caller needs one fixed, sealed vendor mapping. Inspect that
    exception explicitly, then reuse the unchanged helper on each input island.
    All other work/home configuration remains forbidden and aggregate bounds stay
    equal to the bootstrap helper's limits.
    """
    if len(inputs) != 2 or set(inputs) != {work / "fuzz", work / "fuzz-vendor"}:
        raise Incomplete("The complete pair of fuzz input islands is required.")
    for unexpected in (work / ".cargo", work / "controls/.cargo", work / "home/.cargo",
                       work / "cargo-home/config"):
        if unexpected.exists() or unexpected.is_symlink():
            raise Incomplete("Unexpected Cargo ancestry/home configuration in fuzz work.")
    config = work / "cargo-home/config.toml"
    if ANALYSIS.read_once(config, maximum=4096) != FUZZ_CARGO_CONFIG.encode():
        raise Incomplete("The owned fuzz Cargo-home configuration differs from its fixed admission.")
    result = {"cargo-home/config.toml": {"kind": "file", "bytes": config.stat().st_size,
                                         "sha256": ANALYSIS.digest(config)}}
    for island in inputs:
        if island not in (work / "fuzz", work / "fuzz-vendor") or island.parent != work:
            raise Incomplete("Unexpected fuzz input island.")
        for relative, entry in ANALYSIS.input_snapshot(island, [island]).items():
            key = (Path(island.name) / relative).as_posix()
            if key in result:
                raise Incomplete("Duplicate fuzz input island entry.")
            result[key] = entry
    if len(result) > 100000 or sum(entry.get("bytes", 0) for entry in result.values()) > 2 * 1024 ** 3:
        raise Incomplete("Fuzz input snapshot exceeds aggregate entry/byte bounds.")
    return result


def prepare_fuzz_vendor(original: Path, archive: Path, package: dict, destination: Path) -> None:
    """Prepare one new owned package; reject source/archive drift before publication."""
    cached = PRODUCT.inventory(original)
    # These are Cargo-generated cache records, not published package source.
    source_files = {name: digest for name, digest in cached.items()
                    if name not in {".cargo-ok", ".cargo-checksum.json"}}
    specification = {**package, "files_sha256": PREPARE.files_digest(source_files)}
    data = ANALYSIS.read_once(archive, maximum=PREPARE.MAX_ARCHIVE)
    files = PREPARE.archive_files(data, specification)
    destination.mkdir(mode=0o700)
    for name, contents in files.items():
        target = destination / name
        target.parent.mkdir(parents=True, exist_ok=True, mode=0o700)
        target.write_bytes(contents)
    checksums = {"files": {name: PREPARE.sha(contents) for name, contents in files.items()},
                 "package": package["checksum"]}
    (destination / ".cargo-checksum.json").write_text(json.dumps(checksums, sort_keys=True) + "\n")


def execute(args, report: dict) -> None:
    """Freeze graph/tools, execute the selected isolated lane, then reject drift."""
    policy = read_policy(ROOT)
    subjects = select_subjects(args.subject)
    admit_fuzz_subject(args.lane, args.subject)
    report["subjects"] = subjects
    dependency_policy, identity = PRODUCT.admit_product(ROOT)
    product_identity = dict(identity)
    extra = BOUND_INPUTS
    extra += (policy["coverage_guided_fuzzing"]["target_source"],
              policy["coverage_guided_fuzzing"]["llvm_notice"])
    identity.update({name: PRODUCT.sha256(ROOT / name) for name in extra})
    admitted = ANALYSIS.admit(ROOT)
    base, stable = admitted["base"], admitted["stable"]
    vendor = args.vendor_dir.resolve(strict=True)
    vendor_identity = PRODUCT.admit_vendor(vendor, dependency_policy)
    output = args.output_dir
    if (not output.is_absolute() or output.parent != output.parent.resolve(strict=True)
            or output.is_relative_to(ROOT) or output.is_relative_to(Path("/mnt"))):
        raise Incomplete("Use a fresh external canonical private Linux output directory.")
    output.mkdir(mode=0o700)
    report["owned_output"] = str(output)
    work = output / "work"
    work.mkdir(mode=0o700)
    source = work / "source"
    source.mkdir()
    for name in identity:
        destination = source / name
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(ROOT / name, destination)
    require_unchanged(source, identity)
    mutations = tomllib.loads((source / ".cargo/mutants.toml").read_text())
    if mutations.get("additional_cargo_args") != ["--frozen"] or mutations.get("cap_lints") is not False:
        raise Incomplete("Mutation commands require the unchanged frozen Cargo and lint policy.")
    tools = output / "tools"
    tools.mkdir(mode=0o700)
    selected = (("cargo-llvm-cov", "cargo-mutants") if args.lane == "all" else
                ("cargo-llvm-cov",) if args.lane == "coverage" else
                ("cargo-mutants",) if args.lane == "mutation" else ())
    tool_identity = {}
    for name in selected:
        origin = tool_path(ROOT, name)
        shutil.copyfile(origin, tools / name)
        (tools / name).chmod(0o500)
        tool_identity[name] = PRODUCT.sha256(origin)
    if any(PRODUCT.sha256(tools / name) != digest for name, digest in tool_identity.items()):
        raise Incomplete("An analysis extension changed while its sealed copy was prepared.")
    protected_tools = {tools: PRODUCT.inventory(tools)}
    (work / "cargo-home").mkdir()
    (work / "cargo-home/config.toml").write_text(
        '[source.crates-io]\nreplace-with="reviewed"\n[source.reviewed]\ndirectory=' + json.dumps(str(vendor)) + '\n')
    report.update(source_sha256=identity, tools_sha256=tool_identity, lanes=[],
                  analysis_policy_sha256=admitted["policy_sha256"], work=str(work),
                  coverage_guided_fuzzing=policy["coverage_guided_fuzzing"])

    def run(label: str, command: list[str], timeout: int, *, expected_error=None, sealed=(),
            compiler=None, extra_tools=None, mode="ordinary", negative=None, cwd=None, mutation_subject=None,
            sanitizer_profile="legacy"):
        """Run immutable selected source with cleared environment and finite custody."""
        if ((mode == "sanitizer" and sanitizer_profile != RUNTIME.WIRE_KE_SANITIZER_PROFILE)
                or (mode != "sanitizer" and sanitizer_profile != "legacy")):
            raise Incomplete("Wire fuzz execution requires its exact sanitizer profile and mode.")
        if mutation_subject is not None:
            if (label != "mutation-campaign:" + mutation_subject or expected_error or negative
                    or compiler is not None or extra_tools is not None or mode != "ordinary" or cwd is not None):
                raise Incomplete("Mutation campaign cannot borrow another execution mode or failure handler.")
            COMMON.mutation_campaign_command(
                SCOPE, tools, mutation_subject, command,
                expected_jobs=COMMON.mutation_jobs(policy),
                expected_message_format=COMMON.mutation_message_format(policy))
        require_unchanged(ROOT, identity)
        require_current_candidate(ROOT, product_identity)
        require_unchanged(source, identity)
        if any(PRODUCT.inventory(folder) != expected for folder, expected in protected_tools.items()):
            raise Incomplete("An admitted analysis extension changed before execution.")
        compiler = compiler or stable
        # Mutation workers use the admitted policy count with serial Cargo and
        # libtest inside each private copy. Other lanes keep twelve build slots.
        inner_jobs = 1 if label.startswith("mutation-") else RUNTIME.MAX_JOBS
        environment = {"RUSTC": str(compiler / "bin/rustc"),
                       "RUSTDOC": str(compiler / "bin/rustdoc"), "RUST_TEST_THREADS": str(inner_jobs)}
        if compiler != stable:
            environment.update(RUSTUP_HOME=str(base / "common/rustup"), RUSTUP_TOOLCHAIN=ANALYSIS.NIGHTLY)
        readonly = [base, stable, vendor, tools, *([extra_tools] if extra_tools else [])]
        paths = [*([extra_tools] if extra_tools else []), compiler / "bin", tools]
        profile_options = ({"cargo_target_mode": "worker",
                            "process_limit": COMMON.mutation_process_limit(policy)}
                           if mutation_subject is not None else {})
        result = RUNTIME.run(ROOT, work, command, readonly=readonly,
                             sealed=[source, work / "cargo-home/config.toml", *sealed], path_dirs=paths, cwd=cwd or source,
                             environment=environment, mode=mode, sanitizer_profile=sanitizer_profile,
                             timeout=timeout, cpu_seconds=min(timeout, 600), build_jobs=inner_jobs,
                             **profile_options)
        report["lanes"].append({"lane": label, "command": command, **result})
        (output / "progress.json").write_text(json.dumps(report, indent=2) + "\n")
        require_unchanged(source, identity)
        require_unchanged(ROOT, identity)
        require_current_candidate(ROOT, product_identity)
        if any(PRODUCT.inventory(folder) != expected for folder, expected in protected_tools.items()):
            raise Incomplete("An admitted analysis extension changed during execution.")
        if mutation_subject is not None:
            return COMMON.mutation_campaign_result(
                SCOPE, tools, mutation_subject, command, result,
                expected_jobs=COMMON.mutation_jobs(policy),
                expected_message_format=COMMON.mutation_message_format(policy))
        if expected_error:
            name = label.removeprefix("external-consumer-")
            if name.startswith("nts_request_"):
                request_privacy_error(result, name, expected_error)
            else:
                privacy_error(result, expected_error)
            return result["output"]
        if negative:
            if result["failure"] or result["exit_code"] == 0 or negative not in result["output"]:
                raise Incomplete("The seeded fuzz control did not reach its intended failure.")
            return result["output"]
        PRODUCT.ENV.require_ok(result)
        return result["output"]

    if args.lane in {"contracts", "all"}:
        consumers = work / "contract-inputs"
        consumers.mkdir()
        examples = contract_examples()
        for name, (text, _) in examples.items():
            (consumers / (name + ".rs")).write_text(text + "\n")
        (work / "contracts").mkdir()
        rustc = str(stable / "bin/rustc")
        run("contract-library", [rustc, "--edition", "2024", "--crate-name", "ntp_wire",
                                  "--crate-type", "rlib", "/work/source/crates/ntp-wire/src/lib.rs",
                                  "--out-dir", "/work/contracts"], 120, sealed=[consumers])
        executed_legal = {}
        request_specs = nts_request_contract_specs()
        for name, (_, diagnostic) in examples.items():
            run("external-consumer-" + name,
                [rustc, "--edition", "2024", "--crate-name", name,
                 "/work/contract-inputs/" + name + ".rs",
                 "--extern", "ntp_wire=/work/contracts/libntp_wire.rlib", "--out-dir", "/work/contracts",
                 *(["--error-format=json"] if name in request_specs and diagnostic else [])],
                120, sealed=[consumers],
                expected_error=diagnostic)
            if name == "nts_request_legal":
                binary = work / "contracts" / name
                before = PRODUCT.sha256(binary)
                text = run("external-legal-execution-" + name, ["/work/contracts/" + name],
                           30, sealed=[consumers, binary])
                if (text.splitlines() != [request_specs[name]["success_marker"]]
                        or PRODUCT.sha256(binary) != before):
                    raise Incomplete("NTS request legal consumer did not preserve its literal execution/binary.")
                executed_legal[name] = {"binary_sha256": before, "success_marker": text.strip()}
        report["contracts"] = {"legal_consumers_compiled": sum(diagnostic is None for _, diagnostic in examples.values()),
                               "privacy_bypasses_rejected": sum(
                                   diagnostic in {"E0451", "E0616"} for _, diagnostic in examples.values()),
                               "subjects": dict(SUBJECTS),
                               "construction_or_mutation_bypasses_rejected": sum(
                                   diagnostic in {"E0451", "E0616"} for _, diagnostic in examples.values()),
                               "borrow_bypasses_rejected": sum(
                                   diagnostic in {"E0502", "E0505", "E0515", "E0506", "E0594", "E0597"} for _, diagnostic in examples.values()),
                               "consumed_owner_bypasses_rejected": sum(
                                   diagnostic == "E0382" for _, diagnostic in examples.values()),
                               "default_or_payload_debug_bypasses_rejected": sum(
                                   diagnostic == "E0277" for _, diagnostic in examples.values()),
                               "required_diagnostics": {name: diagnostic for name, (_, diagnostic) in examples.items()
                                                        if diagnostic is not None}}
        report["contracts"]["legal_consumers_executed"] = executed_legal
        print("PASSED: ntp-wire external-consumer contracts", flush=True)
    COMMON.run_analysis_lanes(SCOPE, subjects, policy, work, tools, run, report, args.lane)
    if args.lane in {"fuzz", "all"}:
        admit_fuzz_sources(base, policy)
        fuzz = work / "fuzz"
        fuzz.mkdir()
        (fuzz / "Cargo.toml").write_text(
            '[package]\nname="ntp-wire-fuzz"\nversion="0.0.0"\nedition="2024"\npublish=false\n'
            '[workspace]\n[package.metadata]\ncargo-fuzz=true\n[features]\nseeded-defect=[]\nseeded-ke-defect=[]\nseeded-negotiation-defect=[]\nseeded-context-defect=[]\nseeded-membership-defect=[]\nseeded-packet-defect=[]\nseeded-plaintext-defect=[]\n'
            '[dependencies]\nlibfuzzer-sys="=0.4.13"\nntp-wire={path="../source/crates/ntp-wire"}\n'
            '[[bin]]\nname="wire"\npath="target.rs"\ntest=false\ndoc=false\nbench=false\n')
        (fuzz / "Cargo.lock").write_text(fuzz_lock((base / "common/controls-source/fuzz/Cargo.lock").read_text()))
        shutil.copyfile(ROOT / policy["coverage_guided_fuzzing"]["target_source"], fuzz / "target.rs")
        fuzz_vendor = work / "fuzz-vendor"
        fuzz_vendor.mkdir()
        for item in policy["coverage_guided_fuzzing"]["locked_registry_packages"]:
            name = item["name"] + "-" + item["version"]
            original = next((base / "common/cargo/registry/src").glob("*/" + name))
            archive = next((base / "common/cargo/registry/cache").glob("*/" + name + ".crate"))
            prepare_fuzz_vendor(original, archive, item, fuzz_vendor / name)
        (work / "cargo-home/config.toml").write_text(FUZZ_CARGO_CONFIG)
        sealed = [fuzz, fuzz_vendor]
        sealed_before = fuzz_input_snapshot(work, sealed)
        night = base / admitted["policy"]["common_nightly"]
        fuzz_tools = output / "fuzz-tools"
        fuzz_tools.mkdir(mode=0o700)
        # cargo-fuzz 0.13.2 has no frozen CLI flag. This fixed launcher adds the
        # flag to every permitted underlying Cargo command; no general shell runs.
        launcher = fuzz_tools / "cargo"
        launcher.write_text('#!/usr/bin/python3\nimport os,sys\n'
                            'if len(sys.argv)<2 or sys.argv[1] not in ("build","check","rustc","metadata"):\n'
                            ' raise SystemExit("unreviewed fuzz Cargo subcommand")\n'
                            f'os.execv({str(night / "bin/cargo")!r}, [{str(night / "bin/cargo")!r},sys.argv[1],"--frozen",*sys.argv[2:]])\n')
        launcher.chmod(0o500)
        shutil.copyfile(base / "common/bin/cargo-fuzz", fuzz_tools / "cargo-fuzz")
        (fuzz_tools / "cargo-fuzz").chmod(0o500)
        fuzz_tool_hashes = PRODUCT.inventory(fuzz_tools)
        protected_tools[fuzz_tools] = fuzz_tool_hashes
        report["fuzz_launcher_sha256"] = fuzz_tool_hashes
        metadata = run("fuzz-graph", [str(night / "bin/cargo"), "metadata", "--frozen", "--format-version", "1",
                                     "--filter-platform", ANALYSIS.PLATFORM, "--manifest-path", "/work/fuzz/Cargo.toml"],
                       120, compiler=night, extra_tools=fuzz_tools, sealed=sealed, cwd=fuzz)
        report["fuzz_graph"] = fuzz_metadata(json.loads(metadata), policy)
        corpus = work / "fuzz-corpus"
        corpus.mkdir()
        seeds = fuzz_seeds()
        if any(len(data) > 131_073 for data in seeds.values()):
            raise Incomplete("A fuzz seed exceeds the fixed aggregate input cap.")
        for name, data in seeds.items():
            (corpus / name).write_bytes(data)
        seed_identity = PRODUCT.inventory(corpus)
        (work / "fuzz-artifacts").mkdir()
        (work / "fuzz-binaries").mkdir()
        retained_binaries = {}
        for case in FUZZ_CASES:
            command = [str(fuzz_tools / "cargo-fuzz"), "fuzz", "build", "wire", "--fuzz-dir", "/work/fuzz",
                       "--sanitizer", "address", "--target", ANALYSIS.PLATFORM,
                       "--target-dir", "/work/target-fuzz"]
            features, negative = fuzz_case_options(case)
            command.extend(features)
            # Only identical dependency compilation is reused. Preserve each
            # feature-specific executable as an independent immutable byte copy.
            run("fuzz-build-" + case, command, 600, compiler=night, extra_tools=fuzz_tools,
                sealed=[*sealed, *retained_binaries], cwd=fuzz)
            built_binary = work / "target-fuzz" / ANALYSIS.PLATFORM / "release/wire"
            binary_sha256 = PRODUCT.sha256(built_binary)
            if binary_sha256 in retained_binaries.values():
                raise Incomplete("Healthy and seeded-control builds emitted the same executable.")
            binary = "/work/fuzz-binaries/" + case
            actual_binary = work / binary.removeprefix("/work/")
            shutil.copyfile(built_binary, actual_binary)
            actual_binary.chmod(0o500)
            if PRODUCT.sha256(actual_binary) != binary_sha256:
                raise Incomplete("The retained fuzz executable differs from its actual build.")
            retained_binaries[actual_binary] = binary_sha256
            report.setdefault("fuzz_binaries_sha256", {})[case] = binary_sha256
            report.setdefault("fuzz_binary_paths", {})[case] = str(actual_binary)
            witness = fuzz_control_witness(corpus, case, seed_identity)
            witness_inputs = [witness] if witness is not None else []
            if witness is not None:
                report.setdefault("fuzz_control_witnesses", {})[case] = {
                    "path": str(witness), "sha256": seed_identity[witness.name],
                    "scope": "harness forwarding/observation sensitivity; not product mutation"}
            text = run("fuzz-" + case, fuzz_execution_argv(binary, case),
                       90, compiler=night, extra_tools=fuzz_tools,
                       sealed=[*sealed, *retained_binaries, *witness_inputs], cwd=fuzz, mode="sanitizer",
                       sanitizer_profile=RUNTIME.WIRE_KE_SANITIZER_PROFILE,
                       negative=negative)
            if case == "healthy":
                count = re.search(r"Done (\d+) runs", text)
                if not count or int(count[1]) != 256:
                    raise Incomplete("The actual healthy fuzz target did not complete its finite 256 runs.")
                report["fuzz"] = {"observed_runs": 256, "subjects": dict(FUZZ_SUBJECTS),
                                  "unqualified_subjects": {},
                                  "sanitizer_profile": RUNTIME.WIRE_KE_SANITIZER_PROFILE,
                                  "maximum_input_bytes": 131_073,
                                  "seed_lengths": sorted({len(data) for data in seeds.values()}),
                                  "seed_files_sha256": seed_identity,
                                  "request_response_adapter": "four-byte big-endian request length, clamped to payload",
                                  "relational_seed_files_sha256": {name: digest for name, digest in seed_identity.items()
                                                                   if name.startswith("ke-pair-")},
                                  "harness_control_scope": "seeded forwarding/observation faults; no product mutation or ASan mechanism claim",
                                  "seeded_fault": "pending", "sanitizer": "address", "target": ANALYSIS.PLATFORM}
            elif case == "defect":
                report["fuzz"]["seeded_fault"] = "observed intended datagram field-count failure"
            elif case == "ke-defect":
                report["fuzz"]["nts_ke_seeded_fault"] = "observed intended NTS-KE record-count failure"
            elif case == "negotiation-defect":
                report["fuzz"]["nts_negotiation_seeded_fault"] = "observed intended NTS-KE negotiation cookie-count failure"
            elif case == "context-defect":
                report["fuzz"]["nts_request_context_harness_fault"] = "observed intended wrong-request forwarding failure on the literal pair"
            elif case == "membership-defect":
                report["fuzz"]["nts_membership_harness_fault"] = "observed intended first-only membership observation failure after genuine non-first acceptance"
            elif case == "packet-defect":
                report["fuzz"]["nts_packet_harness_fault"] = "observed intended packet span observation failure on the fixed literal"
            elif case == "plaintext-defect":
                report["fuzz"]["nts_plaintext_harness_fault"] = "observed intended plaintext field-count observation failure on the fixed literal"
            else:
                raise Incomplete("Unknown fuzz result case.")
            if any(PRODUCT.sha256(path) != digest for path, digest in retained_binaries.items()):
                raise Incomplete("A retained ASan executable changed during a build or execution.")
            if witness is not None and PRODUCT.sha256(witness) != seed_identity[witness.name]:
                raise Incomplete("The literal fuzz witness changed during its control.")
            if (fuzz_input_snapshot(work, sealed) != sealed_before
                    or PRODUCT.inventory(fuzz_tools) != fuzz_tool_hashes):
                raise Incomplete("Fuzz source, dependency or fixed Cargo launcher bytes changed.")
        admit_fuzz_sources(base, policy)
        report["fuzz"]["fault_artifacts_sha256"] = PRODUCT.inventory(work / "fuzz-artifacts")
        print("PASSED: ntp-wire bounded ASan/libFuzzer target and seeded control", flush=True)
    require_unchanged(ROOT, identity)
    require_current_candidate(ROOT, product_identity)
    require_unchanged(source, identity)
    if PRODUCT.admit_vendor(vendor, dependency_policy) != vendor_identity:
        raise Incomplete("Vendor bytes changed during parser qualification.")
    if any(PRODUCT.sha256(tools / name) != digest for name, digest in tool_identity.items()):
        raise Incomplete("An admitted analysis extension changed during execution.")
    after = ANALYSIS.admit(ROOT)
    if any(after[key] != admitted[key] for key in ("base", "stable", "policy_sha256", "inventories")):
        raise Incomplete("Analysis installation or payload identities changed during execution.")


def main(argv=None) -> int:
    """Run only explicit fixed lanes, retaining partial results when a gate fails."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--lane", choices=("contracts", "coverage", "mutation", "fuzz", "all"), default="all")
    parser.add_argument("--subject", choices=(*SUBJECTS, "all"), default="all")
    parser.add_argument("--vendor-dir", type=Path)
    parser.add_argument("--output-dir", type=Path, required=True)
    args = parser.parse_args(argv)
    report = {"status": "incomplete", "scope": "ntp-wire seven-owner contracts/coverage/mutation including inert two-stage client request formatting; fuzz/all fail closed until the separate six-owner fuzz admission is extended; no TLS, keys or endpoint authority",
              "native_qualification": "not performed"}
    try:
        if args.vendor_dir is None:
            args.vendor_dir = Path(json.loads(PRODUCT.POINTER.read_text())["vendor_dir"])
        execute(args, report)
        report["status"] = "passed"
    except (Incomplete, PRODUCT.ENV.Incomplete, OSError, ValueError, KeyError, TypeError, StopIteration) as error:
        report["error"] = str(error)
    if report.get("owned_output") == str(args.output_dir):
        (args.output_dir / "verification.json").write_text(json.dumps(report, indent=2) + "\n")
    print(report["status"].upper() + ": " + report["scope"], flush=True)
    if report.get("error"):
        print(report["error"], file=sys.stderr)
    return 0 if report["status"] == "passed" else 1


if __name__ == "__main__":
    raise SystemExit(main())
