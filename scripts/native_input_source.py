# SPDX-License-Identifier: GPL-3.0-only
"""Bounded lexical custody checks for the maintained Windows input filter.

Exact small function bodies exclude dead-branch/unused-closure call witnesses.
This selected-syntax source check is not Rust resolution, compiled hook delivery,
upstream allocation containment, field authorization or native qualification.
Independent Rust tests own the pure input decisions.
"""
import hashlib
import re
from pathlib import Path

import check_ui_text_coverage as source

BASE = "crates/nevrotick-ui/src/"
MAIN = BASE + "main.rs"
HOST = BASE + "slint_host.rs"
INPUT = BASE + "slint_host/native_input.rs"
RAW = BASE + "ui_drafts.rs"
MAX_FILES = 256
MAX_FILE_BYTES = 1_048_576
MAX_TOTAL_BYTES = 8_388_608
# Pin only non-test production Rust tokens, excluding comments and whitespace.
# This retains the exact reviewed stateless function set, imports, cfgs and cap;
# it intentionally fails closed on alternate syntax until independently reviewed.
INPUT_PRODUCTION_TOKENS_SHA256 = "e3afe117f4b5ef1ee13cfeb9be376aa5c3da67c88e6d31ed50523c394a87cba8"


def read_sources(root: Path) -> dict[str, str]:
    """Read bounded regular UTF-8 Rust sources from the maintained UI crate only."""
    result = {}
    total = 0
    for path in sorted((root / BASE).rglob("*.rs")):
        if len(result) >= MAX_FILES or path.is_symlink() or not path.is_file():
            raise ValueError("native input source inventory")
        with path.open("rb") as stream:
            raw = stream.read(MAX_FILE_BYTES + 1)
        total += len(raw)
        if len(raw) > MAX_FILE_BYTES or total > MAX_TOTAL_BYTES:
            raise ValueError("native input source byte bound")
        result[path.relative_to(root).as_posix()] = raw.decode("utf-8", "strict")
    return result


def tokens(text: str) -> list[str]:
    """Tokenize the selected ordinary-string syntax after comments are masked."""
    return re.findall(r'"(?:[^"\\]|\\.)*"|[A-Za-z_][A-Za-z0-9_]*|[0-9]+|[^\s]',
                      source.inert_source(text, keep_strings=True))


def check(files: dict[str, str]) -> list[str]:
    """Return fixed redacted faults; no raw source string becomes diagnostics."""
    errors = []
    if not {MAIN, HOST, INPUT, RAW}.issubset(files):
        return ["native input required source missing"]
    if (len(files) > MAX_FILES or any(not isinstance(text, str) for text in files.values())
            or any(len(text.encode()) > MAX_FILE_BYTES for text in files.values())
            or sum(len(text.encode()) for text in files.values()) > MAX_TOTAL_BYTES):
        return ["native input source inventory bound"]
    try:
        # The inherited lexer supports ordinary strings, not raw strings or a
        # double-quote character literal. Reject those forms in every possible
        # hook owner BEFORE using it, including files later masked as test-only.
        hook_candidates = {path for path, text in files.items()
                           if "on_winit_window_event" in text}
        unsupported = re.compile(r"\b(?:br|cr|r)\#*\"|'(?:\\)?\"'")
        if any(unsupported.search(files[path]) for path in hook_candidates):
            return ["native input unsupported hook-owner literal syntax"]
        # Admission above still bounds every file; hook-free unrelated files
        # cannot contain this literal setter identifier and need no Rust scan.
        selected = {path: source.without_test_items(text) for path, text in files.items()
                    if path in {MAIN, HOST, INPUT, RAW} or path in hook_candidates}
        clean = {path: source.inert_source(text) for path, text in selected.items()}

        def attributes_before(text, start):
            prefix = text[:start].rstrip()
            visibility = re.search(r'\bpub(?:\s*\([^)]*\))?\s*$', prefix)
            if visibility:
                prefix = prefix[:visibility.start()].rstrip()
            attributes = []
            while prefix.endswith("]"):
                at, depth = len(prefix) - 2, 1
                while at >= 0 and depth:
                    depth += int(prefix[at] == "]") - int(prefix[at] == "[")
                    at -= 1
                if not depth and at > 0 and prefix[at - 1:at + 1] == "#!":
                    break  # A crate inner attribute is not an item outer attribute.
                if depth or at < 0 or prefix[at] != "#":
                    return ["unsupported attribute stack"]
                attributes.append("".join(tokens(prefix[at:])))
                prefix = prefix[:at].rstrip()
            return list(reversed(attributes))

        def function(path, name, *, windows=False):
            text = selected[path]
            spans = [(start, end) for found, start, end in source.functions(text) if found == name]
            if windows:
                spans = [(start, end) for start, end in spans
                         if attributes_before(clean[path], start) == ["#[cfg(windows)]"]]
            if len(spans) != 1:
                errors.append("native input unique current function")
                return ""
            start, end = spans[0]
            if path == MAIN and windows and tokens(text[:start]) != tokens(
                    '#![cfg_attr(windows, windows_subsystem = "windows")] #[cfg(windows)]'):
                errors.append("native input current main attribute prefix")
            body = text[start:end]
            masked = source.inert_source(body)
            return body[masked.index("{") + 1:masked.rindex("}")]

        def exact(path, name, expected, *, windows=False):
            if tokens(function(path, name, windows=windows)) != tokens(expected):
                errors.append("native input closed " + name + " body")

        exact(MAIN, "main", "nevrotick_ui::slint_host::run_native()", windows=True)
        exact(HOST, "run_native", "create_native()?.run().map_err(HostError::Platform)", windows=True)
        exact(HOST, "create_native", '''
            slint::BackendSelector::new()
                .backend_name("winit".into())
                .renderer_name("software".into())
                .select()
                .map_err(HostError::Platform)?;
            let host = create()?;
            native_input::install(host.window.window());
            native_workflow::install(&host.window, &host.state).map_err(HostError::Native)?;
            Ok(host)
        ''', windows=True)
        modules = list(re.finditer(r'\bmod\s+native_input\s*;', clean[HOST]))
        if (len(modules) != 1 or attributes_before(clean[HOST], modules[0].start())
                != ["#[cfg(any(windows,test))]"]):
            errors.append("native input private platform module")
        fixture = source.inert_source(function(HOST, "create"))
        if re.search(r'\b(?:native_input|native_workflow|BackendSelector|on_winit_window_event)\b|\.\s*(?:show|run)\s*\(', fixture):
            errors.append("native input fixture initialization")
        registrations = [(path, hit.start()) for path, text in clean.items()
                         for hit in re.finditer(r'\bon_winit_window_event\b', text)]
        if len(registrations) != 1 or registrations[0][0] != INPUT:
            errors.append("native input sole production hook")
        input_text = selected[INPUT]
        if len(files[INPUT].encode()) > 32768:
            errors.append("native input standalone module byte bound")
        if not re.search(r'\bpub\s+const\s+MAX_RAW_TEXT_BYTES\s*:\s*usize\s*=\s*1024\s*;', clean[RAW]):
            errors.append("native input maintained raw byte ceiling")
        digest = hashlib.sha256("\n".join(tokens(input_text)).encode()).hexdigest()
        if digest != INPUT_PRODUCTION_TOKENS_SHA256:
            errors.append("native input reviewed stateless filter contract")
    except (ValueError, UnicodeError):
        errors.append("native input unsupported selected source")
    return errors
